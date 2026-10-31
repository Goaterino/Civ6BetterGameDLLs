// Hook Frida : échange dans la file de production et remise en tête d'une merveille. Chargé par
// hook_swap.py. Adresses : projet Ghidra ~/tools/ghidra-projects/civ6xp2 (base 0x180000000), chaque
// site vérifié octet par octet avant d'être accroché.

function emit(kind, data) {
    send({ t: Date.now(), tid: Process.getCurrentThreadId(), kind, ...data });
}

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    return Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('') === hex;
}

// Élément i d'un tableau GameCore (pointeur en +p, taille en +n), -1 hors bornes.
function at(q, p, n, i, size) {
    if (i < 0 || i >= q.add(n).readU32()) return -1;
    const a = q.add(p).readPointer().add(i * size);
    return size === 2 ? a.readS16() : a.readS32();
}

// Contenu de la file (éléments de 0x38 octets en +0x1d0, nombre en +0x1e0) : u unité, b bâtiment,
// d district, p projet, puis case visée et, pour un bâtiment, case posée et progression.
function queueState(q) {
    const n = q.add(0x1e0).readU32();
    const items = [];
    for (let i = 0; i < n && i < 16; i++) {
        const e = q.add(0x1d0).readPointer().add(i * 0x38);
        const it = {};
        for (const [k, off] of [['u', 0x10], ['b', 0x14], ['d', 0x18], ['p', 0x1c]]) {
            const v = e.add(off).readS32();
            if (v !== -1) it[k] = v;
        }
        const x = e.add(0x28).readS32();
        if (x > -9999) { it.x = x; it.y = e.add(0x2c).readS32(); }
        if (it.b !== undefined) {
            const posee = at(q, 0x290, 0x2a0, it.b, 2);
            if (posee !== -1) it.case_posee = posee;
            it.progression = at(q, 0x218, 0x228, it.b, 4);
        }
        items.push(it);
    }
    const city = q.add(0x1e8).readPointer();
    return { joueur: city.add(0xd8).readS32(), ville: city.add(0xa8).readS32(), file: items };
}

const SITES = {
    // City_BuildQueue : opération sur la file (op 9 / 10 : échange ou déplacement, p4 / p5 : index).
    op: { rva: 0x0f4d70, bytes: '48895c241048897424185557' },
    // City_BuildQueue : lancement du bâtiment en tête de file (1801007a0).
    start: { rva: 0x1007a0, bytes: '4055565741554157488d6c24' },
    // CanConstruct (1800f61c0), appelé par start.
    canConstruct: { rva: 0x0f61c0, bytes: '4c8944241889542410535556' },
    // CanHaveWonderAt(plot) (18025bad0), appelé par start puis par 1801020b0.
    wonderAt: { rva: 0x25bad0, bytes: '44894c242048895424105356' },
    // Deux refus de wonderAt sur l'index de case (180_1edc00, 180_7392a0).
    plotBusyA: { rva: 0x1edc00, bytes: '4c8bc185d27838488b81c000' },
    plotBusyB: { rva: 0x7392a0, bytes: '488b41384c8b4140493bc074' },
    // Retrait de l'élément en tête (1800ffbf0 et 180101400).
    dropFront: { rva: 0x0ffbf0, bytes: '40534883ec20488bd9e87217' },
    popFront: { rva: 0x101400, bytes: '8b81e00100004c8bc183f801' },
    // Pose de la merveille sur la case (1801020b0).
    place: { rva: 0x1020b0, bytes: '48895c240855565741544155' },
};

const depth = {};   // tid -> profondeur dans start
let base = null;

function hook(mod) {
    for (const k of Object.keys(SITES)) {
        if (!checkBytes(mod.base, SITES[k].rva, SITES[k].bytes)) {
            emit('erreur', { msg: `${mod.name} : octets inattendus en ${k}, rien d'accroché` });
            return;
        }
    }
    base = mod.base;
    const rva = (p) => `0x${p.sub(base).toString(16)}`;
    const inStart = () => (depth[Process.getCurrentThreadId()] || 0) > 0;

    Interceptor.attach(base.add(SITES.op.rva), {
        onEnter(a) {
            this.q = a[0];
            emit('op', { op: a[2].toInt32(), p4: a[3].toInt32(), p5: a[4].toInt32(),
                taille: a[0].add(0x1e0).readU32(), ...queueState(a[0]) });
        },
        onLeave(r) { emit('op_fin', { ret: r.toInt32() & 0xff, ...queueState(this.q) }); },
    });
    Interceptor.attach(base.add(SITES.start.rva), {
        onEnter(a) {
            const tid = Process.getCurrentThreadId();
            depth[tid] = (depth[tid] || 0) + 1;
            const q = a[0];
            this.q = q;
            const item = q.add(0x1d0).readPointer();
            const b = item.add(0x14).readS32();
            emit('start', { batiment: b, x: item.add(0x28).readS32(), y: item.add(0x2c).readS32(),
                case_posee: at(q, 0x290, 0x2a0, b, 2), progression: at(q, 0x218, 0x228, b, 4),
                appelant: rva(this.returnAddress) });
        },
        onLeave(r) {
            depth[Process.getCurrentThreadId()]--;
            emit('start_fin', { ret: r.toInt32() & 0xff, ...queueState(this.q) });
        },
    });
    for (const k of ['canConstruct', 'plotBusyA', 'plotBusyB', 'place']) {
        Interceptor.attach(base.add(SITES[k].rva), {
            onEnter(a) { this.on = inStart(); this.arg = a[1].toInt32(); },
            onLeave(r) { if (this.on) emit(k, { arg: this.arg, ret: r.toInt32() & 0xff }); },
        });
    }
    // (case) -> propriété de la case (18004_2d00) : +0xc ville, +0x1e merveille posée.
    const plotInfo = new NativeFunction(base.add(0x42d00), 'pointer', ['pointer']);
    Interceptor.attach(base.add(SITES.wonderAt.rva), {
        onEnter(a) {
            this.on = inStart();
            if (this.on) {
                const p = a[1];
                const info = plotInfo(p);
                const city = a[9]; // 10ᵉ argument, sur la pile

                this.ev = { batiment: a[2].toInt32(), x: p.add(0x28).readS16(), y: p.add(0x2a).readS16(),
                    district: p.add(0x3c).readS16(), merveille_case: info.add(0x1e).readS16(),
                    ville_case: info.add(0xc).readS32(), ville: city.isNull() ? -1 : city.add(0xa8).readS32(),
                    appelant: rva(this.returnAddress) };
            }
        },
        onLeave(r) { if (this.on) emit('wonderAt', { ...this.ev, ret: r.toInt32() & 0xff }); },
    });
    for (const k of ['dropFront', 'popFront']) {
        Interceptor.attach(base.add(SITES[k].rva), {
            onEnter() { emit(k, { appelant: rva(this.returnAddress) }); },
        });
    }
    emit('info', { msg: `${mod.name} accroché en ${base}` });
}

function tryModule(mod) {
    // Avec le proxy, les deux noms sont chargés : seul l'original a les bons octets.
    if (base === null && /^GameCore_XP2_FinalRelease/i.test(mod.name)
        && checkBytes(mod.base, SITES.start.rva, SITES.start.bytes)) hook(mod);
}

Process.enumerateModules().forEach(tryModule);
Process.attachModuleObserver({ onAdded: tryModule });
