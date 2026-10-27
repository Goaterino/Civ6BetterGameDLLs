"""Ventilation du temps d'un passage de tour de Civ6.

Deux sources :
- les journaux du jeu (net_message_debug.log, Profile.csv), à la seconde près ;
- une trace WPR (trace.etl, voir capture.ps1), échantillonnée à la milliseconde avec les piles,
  lue directement par etl.py puis gardée en cache (trace.pkl).

Usage :
    python -I perf/rollover/analyze.py <dossier de capture ou dossier Logs du jeu> [--tout]

Écrit rapport.md dans le dossier analysé (ou dans --out).
"""
import argparse
import bisect
import collections
import csv
import datetime as dt
import pathlib
import pickle
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import etl as etl_reader  # noqa: E402

REPO = pathlib.Path(__file__).resolve().parents[2]
FUNCTIONS_TSV = REPO / "docs" / "gamecore_xp2" / "functions.tsv"
GAMECORE = "gamecore_xp2_finalrelease.dll"
GAMECORE_BASE = 0x180000000  # base Ghidra des adresses de functions.tsv
LOCAL_PLAYER = 0


# --- journaux du jeu ------------------------------------------------------------------------

LINE = re.compile(r"\[(\d{4}-\d\d-\d\d \d\d:\d\d:\d\d)\]\t,?\s*(.*)")


def read_log(path):
    for line in path.open(encoding="utf-8", errors="replace"):
        m = LINE.match(line.rstrip("\n"))
        if m:
            yield dt.datetime.strptime(m[1], "%Y-%m-%d %H:%M:%S"), m[2]


def turn_transitions(net_log):
    """Un passage par tour : fin du tour précédent, début du tour, activation de chaque joueur
    jusqu'au joueur local. Les horodatages du jeu sont à la seconde."""
    out, cur = [], None
    end = None
    for t, msg in read_log(net_log):
        if msg.startswith("Game Turn Ended:"):
            end = t
        elif msg.startswith("Game Turn: "):
            cur = {"tour": int(msg.split()[-1]), "fin": end, "debut": t, "joueurs": []}
        elif cur:
            m = re.match(r"Player (\d+) set TurnActive 1", msg)
            if not m:
                continue
            player = int(m[1])
            cur["joueurs"].append((player, t))
            if player == LOCAL_PLAYER:
                cur["main"] = t
                out.append(cur)
                cur, end = None, None
    return out


def log_report(logs, lines):
    net = logs / "net_message_debug.log"
    if not net.exists():
        return []
    trans = turn_transitions(net)
    lines.append("## Passages de tour (net_message_debug.log, à la seconde)\n")
    lines.append("| tour | heure | fin → début | début → main | total | IA activées |")
    lines.append("|---:|---|---:|---:|---:|---:|")
    for tr in trans:
        fin = (tr["debut"] - tr["fin"]).total_seconds() if tr["fin"] else None
        total = (tr["main"] - tr["fin"]).total_seconds() if tr["fin"] else None
        lines.append(
            f"| {tr['tour']} | {tr['main']:%H:%M:%S} | {fmt(fin)} | "
            f"{(tr['main'] - tr['debut']).total_seconds():.0f} | {fmt(total)} | {len(tr['joueurs']) - 1} |")

    # Temps moyen entre deux activations : l'activation du joueur suivant marque la fin du tour
    # de l'IA précédente. Moyenné sur beaucoup de tours, l'arrondi à la seconde s'efface.
    per_player = collections.defaultdict(list)
    for tr in trans:
        prev = tr["debut"]
        for player, t in tr["joueurs"]:
            per_player[player].append((t - prev).total_seconds())
            prev = t
    if per_player:
        lines.append("\n## Délai moyen avant l'activation de chaque joueur\n")
        lines.append("Le délai avant le joueur N couvre le tour de l'IA précédente.\n")
        lines.append("| joueur | tours | moyenne (s) |")
        lines.append("|---:|---:|---:|")
        for player, v in sorted(per_player.items(), key=lambda kv: (kv[0] == LOCAL_PLAYER, kv[0])):
            lines.append(f"| {player} | {len(v)} | {sum(v) / len(v):.2f} |")

    prof = logs / "Profile.csv"
    if prof.exists():
        saves = [float(m[1]) for _, msg in read_log(prof)
                 if (m := re.match(r"DoGameStateSaveRequest, ([\d.]+) ms", msg))]
        stalls = [float(m[1]) for _, msg in read_log(prof)
                  if (m := re.search(r"stalled on GC lock for ([\d.]+) ms", msg))]
        lines.append("\n## Thread principal (Profile.csv)\n")
        if saves:
            lines.append(f"- Sauvegarde auto (`DoGameStateSaveRequest`, bloque le thread principal) : "
                         f"{len(saves)} fois, moyenne {sum(saves) / len(saves):.0f} ms, max {max(saves):.0f} ms.")
        lines.append(f"- Blocages sur le verrou GameCore : {len(stalls)}"
                     + (f", total {sum(stalls):.0f} ms." if stalls else "."))
    lines.append("")
    return trans


def fmt(x):
    return "" if x is None else f"{x:.0f}"


# --- trace WPR ------------------------------------------------------------------------------

def load_trace(etl):
    """Lit la trace une fois, puis la garde en cache (trace.pkl) : la relire prend plusieurs minutes."""
    cache = etl.with_suffix(".pkl")
    if cache.exists() and cache.stat().st_mtime >= etl.stat().st_mtime:
        with cache.open("rb") as f:
            return pickle.load(f)
    tr = etl_reader.read(etl)
    with cache.open("wb") as f:
        pickle.dump(tr, f, protocol=pickle.HIGHEST_PROTOCOL)
    return tr


class Functions:
    def __init__(self, path):
        self.starts, self.rows = [], []
        with path.open(encoding="utf-8") as f:
            for row in csv.DictReader(f, delimiter="\t"):
                self.starts.append(int(row["adresse"], 16))
                self.rows.append(row)
        self.by_addr = {r["adresse"]: r for r in self.rows}

    def find(self, addr):
        i = bisect.bisect_right(self.starts, addr) - 1
        if i >= 0 and addr < self.starts[i] + int(self.rows[i]["octets"]):
            return self.rows[i]
        return None

    def label(self, a):
        r = self.by_addr[a]
        if r["fichier"]:
            where = r["fichier"].rsplit("\\", 1)[-1]
        else:
            where = "? " + r["bornes"].replace("\\", "/").rsplit("/", 1)[-1][:40]
        hint = r["indices_release"].split(" | ")[0][:60] if r["indices_release"] else ""
        lua = r["noms_lua"].split(",")[0] if r["noms_lua"] else ""
        return " ".join(x for x in (f"`{a}`", where, hint and f"« {hint} »", lua and f"lua:{lua}") if x)


def trace_report(etl, lines, window):
    tr = load_trace(etl)
    ms = (tr.interval_100ns or 10000) / 10000
    start_utc = dt.datetime(1601, 1, 1, tzinfo=dt.timezone.utc) + dt.timedelta(microseconds=tr.start_filetime / 10)

    def wall(qpc):
        t = start_utc + dt.timedelta(seconds=(qpc - tr.first_qpc) / tr.perf_freq)
        return t.astimezone().replace(tzinfo=None)

    civ = [i for i in tr.images if i[3].startswith("civilizationvi")]
    if not civ:
        lines.append("Aucun processus CivilizationVI dans la trace.\n")
        return
    pid = civ[0][0]
    mods = sorted({(b, s, n) for p, b, s, n in tr.images if p == pid})
    mod_starts = [m[0] for m in mods]

    def module(ip):
        if ip >= 0xFFFF800000000000:
            return "noyau"
        i = bisect.bisect_right(mod_starts, ip) - 1
        if i >= 0 and ip < mods[i][0] + mods[i][1]:
            return mods[i][2]
        return "?"

    funcs = Functions(FUNCTIONS_TSV)
    gc = next((m for m in mods if m[2] == GAMECORE), None)

    def gc_func(ip):
        if gc and gc[0] <= ip < gc[0] + gc[1]:
            r = funcs.find(ip - gc[0] + GAMECORE_BASE)
            return r["adresse"] if r else None
        return None

    ours = []
    for qpc, tid, ip in tr.samples:
        if tr.thread_pid.get(tid) != pid:
            continue
        t = wall(qpc)
        if window and not (window[0] <= t < window[1]):
            continue
        ours.append((qpc, tid, ip, t))
    if not ours:
        lines.append("Aucun échantillon du jeu dans la fenêtre.\n")
        return
    if window:
        span = (window[1] - window[0]).total_seconds()
    else:
        span = (ours[-1][0] - ours[0][0]) / tr.perf_freq

    lines.append("## Trace WPR\n")
    if window:
        lines.append(f"- Fenêtre : {window[0]:%H:%M:%S} → {window[1]:%H:%M:%S} ({span:.0f} s).")
    else:
        lines.append(f"- Toute la capture ({span:.0f} s).")
    n_stack = sum(1 for s in ours if (s[0], s[1]) in tr.stacks)
    lines.append(f"- {civ[0][3]} (pid {pid}) : {len(ours)} échantillons à {ms:g} ms, "
                 f"soit {len(ours) * ms / 1000:.1f} s de CPU ; {n_stack} avec pile.\n")

    by_thread = collections.Counter(s[1] for s in ours)
    gc_thread = collections.Counter(s[1] for s in ours if module(s[2]) == GAMECORE)
    main_gc = gc_thread.most_common(1)[0][0] if gc_thread else None
    lines.append("### Threads\n")
    lines.append("| thread | nom | CPU (ms) | occupation | dont GameCore (ms) |")
    lines.append("|---:|---|---:|---:|---:|")
    for tid, n in by_thread.most_common(10):
        lines.append(f"| {tid}{' (GameCore)' if tid == main_gc else ''} | {tr.thread_name.get(tid, '')} | "
                     f"{n * ms:.0f} | {100 * n * ms / 1000 / span:.0f} % | {gc_thread[tid] * ms:.0f} |")

    def module_table(title, samples):
        c = collections.Counter(module(s[2]) for s in samples)
        lines.append(f"\n### {title}\n")
        lines.append("| module | CPU (ms) | % |")
        lines.append("|---|---:|---:|")
        for name, n in c.most_common(12):
            lines.append(f"| {name} | {n * ms:.0f} | {100 * n / len(samples):.1f} |")

    module_table("Modules, tout le processus (temps propre)", ours)
    gct = [s for s in ours if s[1] == main_gc]
    if not gct:
        lines.append("")
        return
    module_table(f"Modules, thread GameCore {main_gc} (temps propre)", gct)

    # Fonctions GameCore sur le thread GameCore : propre (IP) et inclusif (présente dans la pile).
    self_f, incl_f, self_cpp, incl_cpp = (collections.Counter() for _ in range(4))
    leaf_mod = collections.defaultdict(collections.Counter)  # dernière fonction GameCore -> module de l'IP
    tree = {}
    with_stack = 0
    for qpc, tid, ip, _ in gct:
        a = gc_func(ip)
        if a:
            self_f[a] += 1
            self_cpp[funcs.by_addr[a]["fichier"] or "?"] += 1
        frames = tr.stacks.get((qpc, tid))
        if not frames:
            continue
        with_stack += 1
        chain = []
        for f in reversed(frames):  # de la racine vers la feuille
            fa = gc_func(f)
            if fa and (not chain or chain[-1] != fa):
                chain.append(fa)
        for x in set(chain):
            incl_f[x] += 1
        for c in {funcs.by_addr[x]["fichier"] or "?" for x in chain}:
            incl_cpp[c] += 1
        if chain:
            leaf_mod[chain[-1]][module(ip)] += 1
        node = tree
        for x in chain[:16]:
            node = node.setdefault(x, [0, {}])
            node[0] += 1
            node = node[1]

    lines.append(f"\n### Arbre d'appels GameCore (thread {main_gc}, {with_stack} piles)\n")
    lines.append("Seules les fonctions GameCore sont gardées dans les piles ; nœuds ≥ 1 % du thread.\n")
    lines.append("```")
    floor = max(1, with_stack // 100)

    def walk(node, depth):
        for a, (n, kids) in sorted(node.items(), key=lambda kv: -kv[1][0]):
            if n >= floor:
                lines.append(f"{'  ' * depth}{n * ms:7.0f} ms  {funcs.label(a)}")
                walk(kids, depth + 1)

    walk(tree, 0)
    lines.append("```")

    lines.append("\n### Fonctions GameCore, temps inclusif\n")
    lines.append("| fonction | inclusif (ms) | propre (ms) |")
    lines.append("|---|---:|---:|")
    for a, n in incl_f.most_common(40):
        lines.append(f"| {funcs.label(a)} | {n * ms:.0f} | {self_f[a] * ms:.0f} |")

    outside = collections.Counter({a: sum(n for m, n in c.items() if m != GAMECORE) for a, c in leaf_mod.items()})
    lines.append("\n### Fonctions GameCore, temps propre et appels hors GameCore en feuille\n")
    lines.append("| fonction | propre (ms) | feuille hors GameCore (ms) | modules |")
    lines.append("|---|---:|---:|---|")
    for a, _ in (self_f + outside).most_common(40):
        other = ", ".join(f"{m} {v * ms:.0f}" for m, v in leaf_mod[a].most_common(4) if m != GAMECORE)
        lines.append(f"| {funcs.label(a)} | {self_f[a] * ms:.0f} | {outside[a] * ms:.0f} | {other} |")

    lines.append("\n### Fichiers source GameCore\n")
    lines.append("| fichier | inclusif (ms) | propre (ms) |")
    lines.append("|---|---:|---:|")
    for c, n in incl_cpp.most_common(30):
        lines.append(f"| {c} | {n * ms:.0f} | {self_cpp[c] * ms:.0f} |")
    lines.append("")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dossier", type=pathlib.Path)
    ap.add_argument("--out", type=pathlib.Path)
    ap.add_argument("--tout", action="store_true", help="toute la capture, pas seulement le dernier passage de tour")
    args = ap.parse_args()

    lines = [f"# Passage de tour : {args.dossier.name}\n"]
    trans = log_report(args.dossier, lines)
    etl = args.dossier / "trace.etl"
    if etl.exists():
        window = None
        if trans and trans[-1]["fin"] and not args.tout:
            # Les journaux sont à la seconde : on borne large d'une seconde de chaque côté.
            window = (trans[-1]["fin"], trans[-1]["main"] + dt.timedelta(seconds=1))
        trace_report(etl, lines, window)
    out = args.out or args.dossier / "rapport.md"
    out.write_text("\n".join(lines), encoding="utf-8")
    print(out)


if __name__ == "__main__":
    sys.exit(main())
