// Hook Frida : côté natif de la mesure du lag des épingles (voir README.md), chargé par
// hook_epingles.py. Le côté Lua est dans le proxy (`ui_mesure_epingles=on`). Exe DX12 seulement
// (projet Ghidra civ6exe, base 0x140000000) : envoi et réception de NetPlayerInfo, taille du message
// reçu. Chaque site est vérifié octet par octet avant d'être accroché.

function emit(kind, data) {
    send({ t: Date.now(), tid: Process.getCurrentThreadId(), kind, ...data });
}

function checkBytes(base, rva, hex) {
    const got = base.add(rva).readByteArray(hex.length / 2);
    const s = Array.from(new Uint8Array(got), (b) => b.toString(16).padStart(2, '0')).join('');
    return s === hex;
}

// --- Exe DX12 -----------------------------------------------------------------------------------

const EXE = {
    broadcast: { rva: 0x26ad60, bytes: '48895c2410564881' },  // lBroadcastPlayerInfo
    read: { rva: 0x267100, bytes: '48895c2408488974' },       // NetPlayerInfo, lecture du flux
    process: { rva: 0x266800, bytes: '405356b898200000' },    // NetPlayerInfo, traitement à la réception
};

// Flux de lecture (voir 14028c640) : méthode +0x10 taille, +0x18 position.
function streamCall(stream, off) {
    const f = new NativeFunction(stream.readPointer().add(off).readPointer(), 'uint64', ['pointer']);
    return f(stream).toNumber();
}

function hookExe() {
    // L'ordre d'enumerateModules() n'est pas garanti : on cherche l'exe par son nom.
    const exe = Process.findModuleByName('CivilizationVI_DX12.exe');
    if (exe === null) {
        emit('info', { msg: 'CivilizationVI_DX12.exe absent (DX11 ?) : sites connus en DX12 seulement, rien d\'accroché' });
        return;
    }
    for (const k of Object.keys(EXE)) {
        if (!checkBytes(exe.base, EXE[k].rva, EXE[k].bytes)) {
            emit('erreur', { msg: `octets inattendus en ${k}, exe non accroché` });
            return;
        }
    }
    // Durée de l'appel : l'envoi (140277410 → méthode +0xb8 de la session, fiable) devrait seulement
    // mettre le message en file. Une durée qui grimpe avec le ping voudrait dire qu'il bloque.
    Interceptor.attach(exe.base.add(EXE.broadcast.rva), {
        onEnter() {
            this.t0 = Date.now();
        },
        onLeave() {
            emit('broadcast_player_info', { ms: Date.now() - this.t0 });
        },
    });
    Interceptor.attach(exe.base.add(EXE.read.rva), {
        onEnter(args) {
            this.stream = args[1];
            try {
                this.pos = streamCall(this.stream, 0x18);
                this.size = streamCall(this.stream, 0x10);
            } catch (e) {
                this.pos = null;
            }
        },
        onLeave() {
            if (this.pos === null) return;
            try {
                emit('net_player_info_lu', { octets: streamCall(this.stream, 0x18) - this.pos, flux: this.size });
            } catch (e) {
                emit('erreur', { msg: `taille du flux illisible : ${e}` });
            }
        },
    });
    Interceptor.attach(exe.base.add(EXE.process.rva), {
        onEnter(args) {
            this.player = args[0].add(0x38).readS32();
            this.cloud = args[0].add(0x98).readU8();
            this.t0 = Date.now();
        },
        onLeave() {
            emit('net_player_info_traite', { player: this.player, cloud: this.cloud, ms: Date.now() - this.t0 });
        },
    });
    emit('info', { msg: 'exe DX12 accroché : BroadcastPlayerInfo, lecture et traitement de NetPlayerInfo' });
}

// --- Images ------------------------------------------------------------------------------------
// La boucle principale vide la file de messages Windows à chaque tour : l'écart entre deux appels à
// PeekMessage sur un même thread mesure une image, ou un gel, quel que soit l'exe (DX11 ou DX12).
// Chaque écart d'au moins IMAGE_LONGUE ms est écrit. Toutes les 5 s, un bilan par thread actif
// donne le nombre d'appels, l'écart maximal et le nombre d'écarts de plus de 33 ms.

const IMAGE_LONGUE = 50;
const threads = new Map();

function hookFrames() {
    const user32 = Process.getModuleByName('user32.dll');
    const onEnter = function () {
        const now = Date.now();
        const tid = Process.getCurrentThreadId();
        let s = threads.get(tid);
        if (s === undefined) {
            s = { last: now, n: 0, max: 0, over33: 0 };
            threads.set(tid, s);
        }
        const gap = now - s.last;
        s.last = now;
        s.n++;
        if (gap > s.max) s.max = gap;
        if (gap > 33) s.over33++;
        if (gap >= IMAGE_LONGUE) send({ t: now, tid, kind: 'image_longue', ms: gap });
    };
    for (const name of ['PeekMessageW', 'PeekMessageA']) {
        Interceptor.attach(user32.getExportByName(name), { onEnter });
    }
    setInterval(() => {
        for (const [tid, s] of threads) {
            // Les threads qui n'appellent PeekMessage qu'en passant ne comptent pas.
            if (s.n >= 50) emit('images', { thread: tid, appels: s.n, max_ms: s.max, plus_de_33ms: s.over33 });
            s.n = 0;
            s.max = 0;
            s.over33 = 0;
        }
    }, 5000);
    emit('info', { msg: `PeekMessage accroché : écarts de ${IMAGE_LONGUE} ms et plus écrits` });
}

hookExe();
hookFrames();
