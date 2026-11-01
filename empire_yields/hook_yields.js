// Hook Frida : quels rapports d'événement et quels gestionnaires de cache suivent un changement de
// citoyen, une action de gouverneur ou l'activation d'un personnage illustre. Chargé par
// hook_yields.py. Adresses : projet Ghidra ~/tools/ghidra-projects/civ6xp2 (base 0x180000000),
// chaque site vérifié octet par octet avant d'être accroché.

function emit(kind, data) {
    send({ t: Date.now(), tid: Process.getCurrentThreadId(), kind, ...data });
}

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    return Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('') === hex;
}

const SITES = {
    // Répartition d'un événement vers les gestionnaires de cache abonnés (événement, valeurs).
    dispatch: { rva: 0x07c670, bytes: '488bc4488970184c89702055' },
    // Player_Governors : assignation (gouverneurs, gouverneur, ville), retrait (gouverneurs,
    // gouverneur, …), promotion (gouverneurs, type de gouverneur, promotion), appelée par
    // l'opération de joueur (18030d330) après CanPromote (1802ddbb0).
    govAssign: { rva: 0x2dd340, bytes: '4885d20f849f030000555741' },
    govUnassign: { rva: 0x2df800, bytes: '48895c240848897424104889' },
    govPromote: { rva: 0x2df340, bytes: '48895c240848896c24104889' },
    // Governor_Instance : établi / plus établi (gouverneur, drapeau).
    govEstablish: { rva: 0x259b10, bytes: '4c8bdc4989732055498dabb8' },
    // Unit_Commands_ActivateGreatPerson.
    gpActivate: { rva: 0x655560, bytes: '4c8944241848894c24085553' },
    // Instantané du trésor d'un joueur (joueur, 1) : ce que lit GetGoldYield dans l'interface.
    treasury: { rva: 0x0c66e0, bytes: '48895c240848896c24104889' },
    // Portée du cache de rendements des villes (jeu+0x2a0) : ouverture, fermeture.
    scopeOpen: { rva: 0x63d1a0, bytes: '40534883ec20ff81a0020000' },
    scopeClose: { rva: 0x63cee0, bytes: '40534883ec208b81a0020000' },
};

// Table des abonnés de 18007c670 : drapeau, seaux, nombre de seaux. Nœud : clé en +0, abonnés de
// +0x8 à +0x10 (0x20 octets chacun), suivant en +0x28. Un abonné est appelé par son relais en +8
// (18000b470, le même pour tous), qui reçoit en premier argument le gestionnaire en +0
// (enregistré par 18007c210, par exemple 1800c6e20 pour le trésor).
const SUBS_READY = 0xb61e00, SUBS_BUCKETS = 0xb61e10, SUBS_COUNT = 0xb61e18;

let base = null;
const actions = {};   // tid -> pile d'actions en cours

function hook(mod) {
    for (const k of Object.keys(SITES)) {
        if (!checkBytes(mod.base, SITES[k].rva, SITES[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, rien d'accroché` });
            return;
        }
    }
    base = mod.base;
    const rva = (p) => `0x${p.sub(base).toString(16)}`;
    const hex = (n) => `0x${(n >>> 0).toString(16).padStart(8, '0')}`;
    const stack = () => actions[Process.getCurrentThreadId()] || [];
    const inAction = () => stack().map((a) => a.kind).join('>') || null;

    function subscribers(hash) {
        if (base.add(SUBS_READY).readU8() === 0) return null;
        const n = base.add(SUBS_COUNT).readU32();
        const buckets = base.add(SUBS_BUCKETS).readPointer();
        for (let node = buckets.add((hash % n) * 8).readPointer(); !node.isNull();
            node = node.add(0x28).readPointer()) {
            if (node.readU32() !== hash) continue;
            const out = [];
            for (let h = node.add(0x8).readPointer(), end = node.add(0x10).readPointer();
                h.compare(end) < 0; h = h.add(0x20)) {
                const fn = h.readPointer();
                if (!fn.isNull()) out.push(rva(fn));
            }
            return out;
        }
        return [];
    }

    function action(kind, describe) {
        Interceptor.attach(base.add(SITES[kind].rva), {
            onEnter(a) {
                const tid = Process.getCurrentThreadId();
                (actions[tid] = actions[tid] || []).push({ kind });
                emit(kind, { phase: 'debut', ...describe(a), appelant: rva(this.returnAddress) });
            },
            onLeave() {
                stack().pop();
                emit(kind, { phase: 'fin' });
            },
        });
    }

    const readPlayer = (p) => (p.isNull() ? -1 : p.add(0xd8).readS32());
    // Gouverneur : type en +4, joueur propriétaire en +0x18 (lus par 180259b10).
    const gov = (g) => (g.isNull() ? {} : { gouverneur: g.add(4).readS32(), joueur: g.add(0x18).readS32() });
    // Ville : identifiant en +0xa8, joueur en +0xd8.
    const city = (c) => (c.isNull() ? {} : { ville: c.add(0xa8).readS32(), joueur_ville: c.add(0xd8).readS32() });

    action('govAssign', (a) => ({ ...gov(a[1]), ...city(a[2]) }));
    action('govUnassign', (a) => ({ ...gov(a[1]), silencieux: a[2].toInt32() & 0xff }));
    // Gouverneurs du joueur : joueur en +0x70 → +0xd8.
    action('govPromote', (a) => ({ joueur: readPlayer(a[0].add(0x70).readPointer()),
        gouverneur: a[1].toInt32(), promotion: a[2].toInt32() }));
    action('govEstablish', (a) => ({ ...gov(a[0]), etabli: a[1].toInt32() & 0xff }));
    action('gpActivate', () => ({}));

    Interceptor.attach(base.add(SITES.dispatch.rva), {
        onEnter(a) {
            const hash = a[0].toUInt32();
            const subs = subscribers(hash);
            if (subs === null || subs.length === 0) return;   // aucun cache abonné : sans intérêt
            emit('rapport', { evenement: hex(hash), action: inAction(), caches: subs,
                appelant: rva(this.returnAddress) });
        },
    });

    Interceptor.attach(base.add(SITES.treasury.rva), {
        onEnter(a) { this.j = readPlayer(a[0]); },
        onLeave() { emit('tresor', { joueur: this.j, action: inAction() }); },
    });

    Interceptor.attach(base.add(SITES.scopeOpen.rva), {
        onLeave(r) { if (r.toInt32() === 1) emit('portee', { etat: 'ouverte', appelant: rva(this.returnAddress) }); },
    });
    Interceptor.attach(base.add(SITES.scopeClose.rva), {
        onLeave(r) { if (r.toInt32() === 0) emit('portee', { etat: 'fermee' }); },
    });

    emit('info', { msg: `${mod.name} accroché en ${base}` });
}

function tryModule(mod) {
    // Avec le proxy, les deux noms sont chargés : seul l'original a les bons octets.
    if (base === null && /^GameCore_XP2_FinalRelease/i.test(mod.name)
        && checkBytes(mod.base, SITES.dispatch.rva, SITES.dispatch.bytes)) hook(mod);
}

Process.enumerateModules().forEach(tryModule);
Process.attachModuleObserver({ onAdded: tryModule });
