"""Compare des sauvegardes Civ6 décompressées, pour vérifier qu'un détour ne change pas l'état de jeu.

Une .Civ6Save est un en-tête non compressé, puis un flux zlib découpé en blocs de 64 Kio précédés
chacun de leur longueur sur 4 octets, puis une fin non compressée. On compare en-tête + flux
décompressé.

Deux sauvegardes d'une même partie rechargée diffèrent déjà par le hasard des IA (≈ 38 600 octets
au tour 80 de AutoSave_0078). D'où le test par groupes : un octet « suit le mode » s'il est identique
dans toutes les sauvegardes d'un groupe, identique dans toutes celles de l'autre, et différent entre
les deux groupes.

Usage :
    python -I compare_saves.py A.Civ6Save B.Civ6Save               # écarts, regroupés par zone
    python -I compare_saves.py --groupes A1 A2 -- B1 B2             # octets qui suivent le mode
"""
import sys
import zlib

MARK = b'\x00\x00\x01\x00\x78\x9c'  # longueur du premier bloc de 64 Kio, puis l'en-tête zlib


def decode(path):
    raw = open(path, 'rb').read()
    start = raw.find(MARK)
    if start < 0:
        raise ValueError(f"{path} : flux compressé introuvable")
    chunks, p = [], start
    while p + 4 <= len(raw):
        n = int.from_bytes(raw[p:p + 4], 'little')
        if n == 0 or p + 4 + n > len(raw):
            break
        chunks.append(raw[p + 4:p + 4 + n])
        p += 4 + n
        if n != 65536:
            break
    return raw[:start] + zlib.decompressobj().decompress(b''.join(chunks))


def zones(a, b, gap=4096):
    out = []
    for k in range(min(len(a), len(b))):
        if a[k] != b[k]:
            if out and k - out[-1][1] < gap:
                out[-1][1] = k
                out[-1][2] += 1
            else:
                out.append([k, k, 1])
    return out


def pair(pa, pb):
    a, b = decode(pa), decode(pb)
    z = zones(a, b)
    print(f"{len(a)} / {len(b)} octets décodés, {sum(n for _, _, n in z)} différents en {len(z)} zones")
    for s, e, n in sorted(z, key=lambda c: -c[2])[:25]:
        print(f"  {s:#010x}-{e:#010x} ({e - s + 1:>7} o) {n:>6} diff  {a[s:s + 16].hex()} / {b[s:s + 16].hex()}")


def groups(ga, gb):
    a = [decode(p) for p in ga]
    b = [decode(p) for p in gb]
    n = min(map(len, a + b))
    hits = [i for i in range(n)
            if len({d[i] for d in a}) == 1 and len({d[i] for d in b}) == 1 and a[0][i] != b[0][i]]
    print(f"{len(hits)} octets suivent le mode sur {n}" + (" : " + " ".join(hex(h) for h in hits[:40]) if hits else ""))


if __name__ == "__main__":
    args = sys.argv[1:]
    if args and args[0] == "--groupes":
        sep = args.index("--")
        groups(args[1:sep], args[sep + 1:])
    else:
        pair(*args[:2])
