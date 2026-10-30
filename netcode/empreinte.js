// Empreinte de synchro : à chaque StartGameSyncCheck (fin de tour en multijoueur), relève le
// hachage de chaque AutoArchive enregistrée dans FGameSyncManager, et celui de chacune de ses
// variables. Chargé par empreinte.py. Adresses : projet Ghidra ~/Tools/ghidra-projects/civ6exe
// (CivilizationVI_DX12.exe, base 0x140000000), voir docs/netcode.

const SITES = {
    startSync: { rva: 0x7f8780, bytes: '405541564881ece80200008b' },  // StartGameSyncCheck(this)
    archiveHash: { rva: 0xdca6b0, bytes: '488bc4488958104889701848' },  // FNV-1a des hachages de variables
};

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    return Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('') === hex;
}

const hex8 = (v) => (v >>> 0).toString(16).padStart(8, '0');

// Variables dont on relève aussi la disposition en mémoire (nom d'archive sans identifiant, indice).
// Player::Units.m_aiAddOnDistrictFounding : table de hachage EASTL int -> (table int -> int). Son
// hachage de synchro suit l'ordre des seaux, voir docs/netcode.
const DETAILS = { 'Player::Units': { 13: addOnDistrictFounding } };

// Table EASTL en `map` : +8 seaux, +0x10 nombre de seaux, +0x14 taille. Le suivant d'un nœud est en
// `next` ; `entry(node)` donne le texte d'un élément.
function hashMap(map, next, entry) {
    const buckets = map.add(8).readPointer();
    const n = map.add(0x10).readU32();
    const parts = [];
    if (n > 100000) return '?';
    for (let i = 0; i < n; i++) {
        for (let node = buckets.add(i * 8).readPointer(); !node.isNull(); node = node.add(next).readPointer())
            parts.push(entry(node));
    }
    return `n=${n} t=${map.add(0x14).readU32()} [${parts.join(' ')}]`;
}

function addOnDistrictFounding(v) {
    return hashMap(v.add(0x10), 0x30, (outer) => hex8(outer.readU32()) + ':{' +
        hashMap(outer.add(8), 8, (inner) => `${hex8(inner.readU32())}=${inner.add(4).readS32()}`) + '}');
}

// Nom affiché par le jeu (140dca310) : type, espace, puis les trois champs de l'identifiant en +0x58.
function archiveName(archive) {
    const id = archive.add(0x58);
    const begin = archive.add(0x60).readPointer();
    const end = archive.add(0x68).readPointer();
    const len = end.sub(begin).toInt32();
    const type = len > 0 && len < 256 ? begin.readUtf8String(len) : '?';
    return `${type} ${id.readS16()}-${id.add(2).readS16()}-${id.add(4).readS32()}`;
}

function hook(mod) {
    const base = mod.base;
    for (const k of Object.keys(SITES)) {
        if (!checkBytes(base, SITES[k].rva, SITES[k].bytes)) {
            send({ kind: 'erreur', msg: `${mod.name} : octets inattendus en ${k} (lancer la version DX12)` });
            return;
        }
    }
    const archiveHash = new NativeFunction(base.add(SITES.archiveHash.rva), 'uint32', ['pointer']);
    const varHash = new Map();  // méthode virtuelle +0x38 de chaque type de variable

    // Hachages des variables, et détail des variables listées dans DETAILS.
    function variableHashes(archive, detail) {
        const out = [];
        const details = [];
        let p = archive.add(0x08).readPointer();
        const end = archive.add(0x10).readPointer();
        for (let i = 0; p.compare(end) < 0; p = p.add(8), i++) {
            const v = p.readPointer();
            if (detail && detail[i]) details.push(`${i}:${detail[i](v)}`);
            const fn = v.readPointer().add(0x38).readPointer();
            const key = fn.toString();
            let f = varHash.get(key);
            if (f === undefined) {
                f = new NativeFunction(fn, 'uint32', ['pointer']);
                varHash.set(key, f);
            }
            out.push(hex8(f(v)));
        }
        return [out.join(','), details.join(' ; ')];
    }

    Interceptor.attach(base.add(SITES.startSync.rva), {
        onEnter(args) {
            const t0 = Date.now();
            const mgr = args[0];
            const buckets = mgr.add(0x18).readPointer();
            const count = mgr.add(0x20).readU32();
            const sentinel = buckets.add(count * 8).readPointer();
            const rows = [];
            // Table chaînée avec sentinelle : nœud = { +0 id 64 bits, +8 archive, +0x10 suivant }.
            let i = 0;
            let node = buckets.readPointer();
            while (node.isNull() && i < count) node = buckets.add(++i * 8).readPointer();
            while (!node.equals(sentinel) && !node.isNull()) {
                const archive = node.add(8).readPointer();
                const name = archiveName(archive);
                const [variables, detail] = variableHashes(archive, DETAILS[name.slice(0, name.lastIndexOf(' '))]);
                rows.push([node.readU64().toString(16), hex8(archiveHash(archive)), name, variables, detail]);
                node = node.add(0x10).readPointer();
                while (node.isNull() && i < count) node = buckets.add(++i * 8).readPointer();
            }
            send({ kind: 'empreinte', t: t0, ms: Date.now() - t0, rows });
        },
    });
    send({ kind: 'info', msg: `${mod.name} accroché en ${base}` });
}

for (const mod of Process.enumerateModules()) {
    const name = mod.name.toLowerCase();
    if (name === 'civilizationvi_dx12.exe' || name === 'civilizationvi.exe') hook(mod);
}
