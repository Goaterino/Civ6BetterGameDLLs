"""Génère src/exports.rs : un relais par export de la DLL d'origine, sous le même nom.

    python -I proxy/gen_exports.py <GameCore_XP2_FinalRelease.dll> proxy/src/exports.rs

Les 157 exports de GameCore_XP2 sont tous du code (vérifié : leurs adresses tombent dans .text).
"""
import struct
import sys


def exports(path):
    b = open(path, "rb").read()
    pe = struct.unpack_from("<I", b, 0x3C)[0]
    nsec = struct.unpack_from("<H", b, pe + 6)[0]
    opt = pe + 24
    optsz = struct.unpack_from("<H", b, pe + 20)[0]
    secs = [struct.unpack_from("<8sIIII", b, opt + optsz + 40 * i) for i in range(nsec)]

    def off(r):
        return next(r - va + ro for _, vs, va, rs, ro in secs if va <= r < va + max(vs, rs))

    rva, _ = struct.unpack_from("<II", b, opt + 112)
    e = off(rva)
    _, _, nn, _, anp, _ = struct.unpack_from("<IIIIII", b, e + 16)
    names = []
    for i in range(nn):
        o = off(struct.unpack_from("<I", b, off(anp) + 4 * i)[0])
        names.append(b[o:b.index(b"\0", o)].decode())
    return names


def main():
    dll, out = sys.argv[1:3]
    names = exports(dll)
    lines = [
        "// Généré par gen_exports.py depuis GameCore_XP2_FinalRelease.dll : ne pas modifier à la main.",
        f"// {len(names)} exports, relayés sous le même nom vers la DLL d'origine renommée.",
        "",
        "forward! {",
    ]
    for i, n in enumerate(names):
        lines.append(f'    e{i:03} => "{n}",')
    lines.append("}")
    open(out, "w", encoding="utf-8", newline="\n").write("\n".join(lines) + "\n")
    print(f"{len(names)} exports écrits dans {out}")


if __name__ == "__main__":
    main()
