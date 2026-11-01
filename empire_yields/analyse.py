"""Résume un journal de hook_yields.py, repère par repère.

    .venv/Scripts/python.exe -I empire_yields/analyse.py empire_yields/runs/<date-heure>/frida.log

Pour chaque segment entre deux repères : les actions (gouverneurs, personnage illustre), puis
chaque événement rapporté aux caches de l'interface, avec ses gestionnaires nommés par leur
fichier source (docs/gamecore_xp2/functions.tsv), et les instantanés du trésor par joueur.
"""

import bisect
import collections
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
BASE = 0x180000000


def load_functions():
    starts, names = [], []
    with open(ROOT / "docs" / "gamecore_xp2" / "functions.tsv", encoding="utf-8") as f:
        next(f)
        for line in f:
            col = line.rstrip("\n").split("\t")
            starts.append(int(col[0], 16))
            fichier = col[2] or (f"? {col[4]}" if len(col) > 4 and col[4] else "?")
            names.append(fichier.replace("GameCore\\", "").replace("\\", "/"))
    return starts, names


def namer():
    starts, names = load_functions()

    def name(rva):
        a = BASE + int(rva, 16)
        i = bisect.bisect_right(starts, a) - 1
        short = names[i].rsplit("/", 1)[-1].replace(".cpp", "")
        return f"{a:x} {short}"

    return name


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    name = namer()
    segments = [("début", [])]
    for line in open(sys.argv[1], encoding="utf-8"):
        t, tid, kind, payload = line.rstrip("\n").split("\t", 3)
        if kind == "repere":
            segments.append((f"repère {payload} ({t})", []))
            continue
        try:
            p = json.loads(payload)
        except json.JSONDecodeError:
            p = {"brut": payload}
        segments[-1][1].append((t, kind, p))

    for title, rows in segments:
        print(f"===== {title}")
        reports = collections.Counter()
        tresor = collections.Counter()
        for t, kind, p in rows:
            if kind == "rapport":
                caches = ", ".join(name(c) for c in p["caches"])
                reports[(p["evenement"], p["action"] or "-", caches)] += 1
            elif kind == "tresor":
                tresor[(p["joueur"], p["action"] or "-")] += 1
            elif kind in ("portee", "info"):
                continue
            elif p.get("phase") == "debut":
                detail = {k: v for k, v in p.items() if k != "phase"}
                print(f"  {t} {kind} {json.dumps(detail, ensure_ascii=False)}")
        for (ev, action, caches), n in reports.most_common():
            print(f"  {n:4d} x {ev} [{action}] -> {caches}")
        for (j, action), n in sorted(tresor.items()):
            print(f"  {n:4d} x trésor du joueur {j} [{action}]")


if __name__ == "__main__":
    main()
