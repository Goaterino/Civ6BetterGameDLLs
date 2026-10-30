"""Compare deux empreintes de synchro (fichiers écrits par empreinte.py).

    .venv/Scripts/python.exe -I netcode/compare_empreintes.py A.tsv B.tsv [--detail N]

Affiche le nombre d'archives différentes par type, puis, pour les N premières (20 par défaut), les
indices des variables qui diffèrent. Les archives sont appariées par nom (« Player::Units 2-40-0 »).
"""

import argparse
import collections
import pathlib


def load(path):
    rows = {}
    with open(path, encoding="utf-8") as f:
        next(f)
        for line in f:
            # La colonne détail n'existe que dans les empreintes récentes.
            _id, h, name, variables, *detail = line.rstrip("\n").split("\t")
            rows[name] = (h, variables.split(",") if variables else [], detail[0] if detail else "")
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("a", type=pathlib.Path)
    ap.add_argument("b", type=pathlib.Path)
    ap.add_argument("--detail", type=int, default=20)
    args = ap.parse_args()
    a, b = load(args.a), load(args.b)

    only_a = sorted(set(a) - set(b))
    only_b = sorted(set(b) - set(a))
    diff = sorted(n for n in set(a) & set(b) if a[n][0] != b[n][0])
    print(f"{args.a.name} : {len(a)} archives ; {args.b.name} : {len(b)} archives")
    print(f"différentes : {len(diff)} ; seulement dans A : {len(only_a)} ; seulement dans B : {len(only_b)}")

    by_type = collections.Counter(n.rsplit(" ", 1)[0] for n in diff)
    if by_type:
        print("\nArchives différentes par type :")
        for t, c in by_type.most_common():
            print(f"  {c:5d}  {t}")
    for label, names in (("seulement dans A", only_a), ("seulement dans B", only_b)):
        if names:
            print(f"\n{label} : " + ", ".join(names[:30]) + (" …" if len(names) > 30 else ""))

    if diff and args.detail:
        print(f"\nVariables différentes (indice dans l'archive), {min(args.detail, len(diff))} premières archives :")
        for n in diff[: args.detail]:
            va, vb = a[n][1], b[n][1]
            idx = [i for i in range(max(len(va), len(vb)))
                   if i >= len(va) or i >= len(vb) or va[i] != vb[i]]
            size = f"{len(va)} variables" if len(va) == len(vb) else f"{len(va)} contre {len(vb)} variables"
            print(f"  {n} ({size}) : {idx[:40]}{' …' if len(idx) > 40 else ''}")
            if a[n][2] != b[n][2]:
                print(f"    A : {a[n][2]}\n    B : {b[n][2]}")


if __name__ == "__main__":
    main()
