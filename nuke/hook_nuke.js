// Hook Frida : déroulé d'un lancer de WMD, du calcul des résultats dans GameCore jusqu'à la fin de
// la visualisation du combat dans l'exe. Chargé par hook_nuke.py ; chaque événement part par send()
// avec son heure, son thread et sa pile d'appels (module!rva).
// Adresses : projets Ghidra ~/Tools/ghidra-projects/civ6exe (base 0x140000000) et civ6xp2
// (base 0x180000000). Chaque site est vérifié octet par octet avant d'être accroché.

const DEPTH = 24;

function emit(kind, data) {
    send({ t: Date.now(), tid: Process.getCurrentThreadId(), kind, ...data });
}

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    const s = Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('');
    return s === hex;
}

function stack(ctx) {
    return Thread.backtrace(ctx, Backtracer.ACCURATE).slice(0, DEPTH).map((a) => {
        const m = Process.findModuleByAddress(a);
        return m ? `${m.name}!${a.sub(m.base)}` : a.toString();
    });
}

// --- GameCore_XP2 -----------------------------------------------------------------------------

const GC = {
    // Combat_Manager.cpp : résultats d'une frappe ICBM (assert « without a valid WMD »).
    icbmResults: { rva: 0x158af0, bytes: '448844241888542410535556', pile: true },
    // Combat_Manager.cpp : même assert, résultats côté attaque d'une unité (bombardier, sous-marin).
    wmdResults: { rva: 0x156ae0, bytes: '405356574883ec60410fb6d8', pile: true },
    // Messages_Manager.cpp : ajout d'un message à la file (IGameCore+0x230). args[0] : type.
    post: { rva: 0x373e0, bytes: '405556574156488d6c24c14881ecb800', pile: true,
        data: (args) => ({ type: `0x${(args[0].toUInt32() >>> 0).toString(16)}` }) },
    // Messages_Manager.cpp : vidange de la file par le thread GameCore (IGameCore+0x238).
    drain: { rva: 0x372f0, bytes: '48895c24084889742410574883ec6033' },
};
let gcListeners = [];
let gcBase = null;
// Une frappe WMD calculée pendant une vidange de la file : la prochaine visualisation est la sienne.
let drainTid = 0;
let wmdAt = 0;

function hookGameCore(mod) {
    const base = mod.base;
    for (const k of Object.keys(GC)) {
        if (!checkBytes(base, GC[k].rva, GC[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, GameCore non accroché` });
            return;
        }
    }
    for (const k of Object.keys(GC)) {
        const site = GC[k];
        gcListeners.push(Interceptor.attach(base.add(site.rva), {
            onEnter(args) {
                const tid = Process.getCurrentThreadId();
                if (k === 'drain') drainTid = tid;
                else if ((k === 'icbmResults' || k === 'wmdResults') && tid === drainTid) wmdAt = Date.now();
                const ev = site.data ? site.data(args) : {};
                if (site.pile) ev.pile = stack(this.context);
                emit(`gc_${k}`, ev);
            },
            onLeave() {
                if (k === 'drain') drainTid = 0;
                emit(`gc_${k}_fin`, {});
            },
        }));
    }
    gcBase = base;
    emit('info', { msg: `${mod.name} accroché en ${base}` });
}

// --- Exe (DX12 seulement) ---------------------------------------------------------------------

// Fonctions de diffusion rangées dans les emplacements de SerialEvent::CombatVisBegin et
// CombatVisEnd (14230c150 et 14230c180, écrits par 1400ed220 et 1400ed2e0). Tout envoi de
// l'événement, depuis le C++ ou depuis Lua, passe par elles. args[0] : données de l'événement.
const EXE = {
    visBegin: { rva: 0xf0400, bytes: '48895c2408574883ec40488b0577cd21' },
    visEnd: { rva: 0xf0460, bytes: '48895c2408574883ec40488b0527cd21' },
    // Civ6App_PublishGameCoreEvents : appelé par 14006e7e0 à chaque frame où la publication est
    // permise (thread GameCore libre et GameCoreController+0xc nul).
    publish: { rva: 0x6e5d0, bytes: '4c8bdc574881ec90000000488bf933c0' },
};
const PUBLISH_GAP_MS = 250;
const CONTROLLER = 0x230d6f0;   // GameCoreController* (1400f4ce0) ; +0xc : porte de la publication
const WORKER = 0x230d728;       // thread « Game Core » (1400f49a0) ; +0x40 : occupé
const SAMPLE_MS = 50;
// Échelle de temps de l'affichage : App+0x559c, écrite par la commande de console « timescale »
// (14006a190) et appliquée au pas de chaque frame par 140076a40. GameCore ne la voit pas.
// ACCEL est remplacé par hook_nuke.py --accel ; 1 : on ne touche à rien.
const ACCEL = 1;
const APP = 0x12fa2c8;
const APP_TIMESCALE = 0x559c;
const WMD_TO_VIS_MS = 2000;
let restoreTimescale = null;

// Données construites par 1403c72d0 : 4 membres { i16 joueur, u16 type, i32 id } en 0..31, puis
// x, y de la case en 48 et 52 (-9999 par défaut).
function visData(p) {
    const members = [];
    for (let i = 0; i < 4; i++) {
        const m = p.add(i * 8);
        const player = m.readS16();
        if (player !== -1) members.push({ joueur: player, type: m.add(2).readU16(), id: m.add(4).readS32() });
    }
    return { membres: members, x: p.add(48).readS32(), y: p.add(52).readS32() };
}

function hookExe(mod) {
    const base = mod.base;
    for (const k of Object.keys(EXE)) {
        if (!checkBytes(base, EXE[k].rva, EXE[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, exe non accroché (lancer la version DX12)` });
            return;
        }
    }
    let lastPublish = 0;
    Interceptor.attach(base.add(EXE.publish.rva), {
        onEnter() {
            const now = Date.now();
            if (lastPublish && now - lastPublish > PUBLISH_GAP_MS) emit('exe_publish_reprise', { pause_ms: now - lastPublish });
            lastPublish = now;
        },
    });
    // Relevé périodique, émis seulement quand une valeur change.
    let prev = '';
    setInterval(() => {
        const read = (rva, off, kind) => {
            const p = base.add(rva).readPointer();
            return p.isNull() ? null : (kind === 'u8' ? p.add(off).readU8() : p.add(off).readS32());
        };
        try {
            const s = { porte_publication: read(CONTROLLER, 0xc, 'u8'), gamecore_occupe: read(WORKER, 0x40, 's32') };
            const key = JSON.stringify(s);
            if (key !== prev) {
                prev = key;
                emit('exe_etat', s);
            }
        } catch (e) {
            emit('erreur', { msg: `relevé : ${e.message}` });
        }
    }, SAMPLE_MS);
    const timescale = () => base.add(APP).readPointer().add(APP_TIMESCALE);
    restoreTimescale = (why) => {
        if (accelFrom === null) return;
        timescale().writeFloat(accelFrom);
        emit('exe_acceleration_fin', { timescale: accelFrom, raison: why });
        accelFrom = null;
    };
    let accelFrom = null;
    for (const k of ['visBegin', 'visEnd']) {
        Interceptor.attach(base.add(EXE[k].rva), {
            onEnter(args) {
                const ev = { pile: stack(this.context) };
                try {
                    Object.assign(ev, visData(args[0]));
                } catch (e) {
                    ev.donnees = `illisibles : ${e.message}`;
                }
                emit(`exe_${k}`, ev);
                if (k === 'visEnd') {
                    restoreTimescale('fin de visualisation');
                } else if (ACCEL !== 1 && accelFrom === null && Date.now() - wmdAt < WMD_TO_VIS_MS
                           // Une frappe WMD n'a qu'un membre, le tireur : un combat ordinaire en a deux.
                           && ev.membres && ev.membres.length === 1) {
                    wmdAt = 0;
                    const cur = timescale().readFloat();
                    // Garde-fou : une valeur inattendue veut dire que l'adresse est fausse.
                    if (!(cur > 0.05 && cur < 20)) {
                        emit('erreur', { msg: `échelle de temps inattendue (${cur}), pas d'accélération` });
                        return;
                    }
                    accelFrom = cur;
                    timescale().writeFloat(cur * ACCEL);
                    emit('exe_acceleration', { timescale: cur * ACCEL });
                }
            },
        });
    }
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
        // Avec le proxy de perf/rollover, les deux noms sont chargés : seul l'original a les bons octets.
        if (checkBytes(mod.base, GC.icbmResults.rva, GC.icbmResults.bytes)) hookGameCore(mod);
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
// Appelé par hook_nuke.py avant de se détacher : ne jamais laisser le jeu accéléré.
rpc.exports = {
    restaurer() {
        if (restoreTimescale) restoreTimescale('détachement');
    },
};
emit('info', { msg: 'hook prêt' });
