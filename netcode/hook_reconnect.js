// Hook Frida de la machine hôte : déconnexion, reconnexion et instantané.
// Chargé par hook_reconnect.py ; chaque événement part par send() avec son heure et son thread.
// Adresses : projets Ghidra ~/Tools/ghidra-projects/civ6exe (base 0x140000000) et civ6xp2
// (base 0x180000000). Chaque site est vérifié octet par octet avant d'être accroché.

const SLOT = ['OPEN', 'COMPUTER', 'CLOSED', 'TAKEN', 'OBSERVER'];
const slotName = (v) => (v >= 0 && v < SLOT.length ? SLOT[v] : String(v));

function emit(kind, data) {
    send({ t: Date.now(), tid: Process.getCurrentThreadId(), kind, ...data });
}

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    const s = Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('');
    return s === hex;
}

// --- GameCore_XP2 -----------------------------------------------------------------------------

const GC = {
    handler: { rva: 0x383e0, bytes: '4c8bdc5557498dab28ffffff' },   // PlayerConnectionChange
    getSlot: { rva: 0x16a2b0, bytes: '4883ec28488b49108b15aa33' },  // statut lu dans la config
    setSlot: { rva: 0x16b840, bytes: '48895c2408574883ec20488b' },  // statut écrit dans la config
    players: 0xb8aa60,  // pointeur global ; +0xe10 : table des joueurs, 0x20 octets par joueur
    flags: 0xb8ab20,    // 3 octets testés par 18004ab50 avant la relance de la mise à jour
};
let gcListeners = [];
let gcBase = null;

function gcRecord(base, player) {
    const table = base.add(GC.players).readPointer().add(0xe10).readPointer();
    return table.add(player * 0x20);
}

function gcPlayerOf(base, rec) {
    try {
        const table = base.add(GC.players).readPointer().add(0xe10).readPointer();
        const off = rec.sub(table).toInt32();
        return off >= 0 && off % 0x20 === 0 && off / 0x20 < 64 ? off / 0x20 : -1;
    } catch (e) {
        return -1;
    }
}

function hookGameCore(mod) {
    const base = mod.base;
    for (const k of ['handler', 'getSlot', 'setSlot']) {
        if (!checkBytes(base, GC[k].rva, GC[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, GameCore non accroché` });
            return;
        }
    }
    const getSlot = new NativeFunction(base.add(GC.getSlot.rva), 'int', ['pointer']);
    let inHandler = 0;
    gcListeners.push(Interceptor.attach(base.add(GC.handler.rva), {
        onEnter(args) {
            const msg = args[0];
            this.player = msg.add(0x18).readU32();
            inHandler = Process.getCurrentThreadId();
            const ev = { player: this.player, connecting: msg.add(0x1c).readU8(), kicked: msg.add(0x1d).readU8() };
            if (this.player < 64) {
                this.rec = gcRecord(base, this.player);
                ev.statut_avant = slotName(getSlot(this.rec));
            }
            const f = base.add(GC.flags);
            ev.drapeaux_relance = [f.readU8(), f.add(1).readU8(), f.add(2).readU8()];
            emit('gc_connexion_debut', ev);
        },
        onLeave() {
            inHandler = 0;
            const ev = { player: this.player };
            if (this.rec) ev.statut_apres = slotName(getSlot(this.rec));
            emit('gc_connexion_fin', ev);
        },
    }));
    gcListeners.push(Interceptor.attach(base.add(GC.setSlot.rva), {
        onEnter(args) {
            emit('gc_statut_ecrit', {
                player: gcPlayerOf(base, args[0]),
                statut: slotName(args[1].toInt32()),
                dans_message: inHandler === Process.getCurrentThreadId(),
                appelant: this.returnAddress.sub(base).toString(),
            });
        },
    }));
    gcBase = base;
    emit('info', { msg: `${mod.name} accroché en ${base}` });
}

// --- Exe (DX12 seulement) ---------------------------------------------------------------------

const EXE = {
    playerLeft: { rva: 0x273c40, bytes: '48895c241855565741564157' },
    playerConnected: { rva: 0x272910, bytes: '48895c240855565741544155' },
    setSlot: { rva: 0x1176a0, bytes: '48895c2408574883ec20488b' },
    snapRequest: { rva: 0x274f90, bytes: '40554156488dac2448e0ffff' },
    snapSaved: { rva: 0x2723a0, bytes: '4055b8a0200000e87413c300' },
    snapSend: { rva: 0x2752a0, bytes: '48895c241848896c24205657' },
    syncCompare: { rva: 0x7f6120, bytes: '488954241048894c24085556' },
};

function hookExe(mod) {
    const base = mod.base;
    for (const k of Object.keys(EXE)) {
        if (!checkBytes(base, EXE[k].rva, EXE[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, exe non accroché (lancer la version DX12)` });
            return;
        }
    }
    const at = (k) => base.add(EXE[k].rva);
    Interceptor.attach(at('playerLeft'), {
        onEnter(args) { emit('exe_player_left', { player: args[1].toInt32(), net_player_removed: args[2].toInt32() & 0xff }); },
        onLeave() { emit('exe_player_left_fin', {}); },
    });
    Interceptor.attach(at('playerConnected'), {
        onEnter(args) { emit('exe_player_connected', { player: args[1].toInt32() }); },
    });
    Interceptor.attach(at('setSlot'), {
        onEnter(args) {
            emit('exe_statut_ecrit', { statut: slotName(args[1].toInt32()), appelant: this.returnAddress.sub(base).toString() });
        },
    });
    Interceptor.attach(at('snapRequest'), {
        onEnter(args) { emit('exe_instantane_demande', { etat_cache: args[0].add(0x30).readS32() }); },
    });
    Interceptor.attach(at('snapSaved'), {
        onEnter() { emit('exe_instantane_sauve', {}); },
    });
    Interceptor.attach(at('snapSend'), {
        onEnter() { emit('exe_instantane_envoye', {}); },
    });
    Interceptor.attach(at('syncCompare'), {
        onEnter(args) { this.player = args[1].add(0x24).readS32(); },
        // Un contrôle reste en file (retour 0) tant que l'hôte n'est pas prêt : on ne note que le traitement.
        onLeave(ret) { if ((ret.toInt32() & 0xff) !== 0) emit('exe_synchro_comparee', { player: this.player }); },
    });
    emit('info', { msg: `${mod.name} accroché en ${base}` });
}

// --- Chargement des modules -------------------------------------------------------------------

const GC_NAMES = ['gamecore_xp2_finalrelease.dll', 'gamecore_xp2_finalrelease_orig.dll'];
const seen = new Set();

function onModule(mod) {
    const key = mod.base.toString();
    if (seen.has(key)) return;
    seen.add(key);
    const name = mod.name.toLowerCase();
    if (name === 'civilizationvi_dx12.exe' || name === 'civilizationvi.exe') {
        hookExe(mod);
    } else if (GC_NAMES.includes(name)) {
        // Avec le proxy de GameCore (proxy/), les deux noms sont chargés : seul l'original a les bons octets.
        if (checkBytes(mod.base, GC.handler.rva, GC.handler.bytes)) hookGameCore(mod);
    }
}

Process.attachModuleObserver({
    onAdded: onModule,
    onRemoved(mod) {
        seen.delete(mod.base.toString());
        if (gcBase !== null && mod.base.equals(gcBase)) {
            for (const l of gcListeners) {
                try { l.detach(); } catch (e) { /* module déjà déchargé */ }
            }
            gcListeners = [];
            gcBase = null;
            emit('info', { msg: `${mod.name} déchargé` });
        }
    },
});
for (const mod of Process.enumerateModules()) onModule(mod);
emit('info', { msg: 'hook prêt' });
