"""Attache hook_epingles.js au jeu (DX12) et rassemble les mesures des épingles (voir README.md).

    .venv/Scripts/python.exe -I map_pins/hook_epingles.py [dossier]

Les mesures Lua viennent du proxy installé avec -UiMesureEpingles on ; ce script ajoute le côté
natif et les recopie. Le lancer jeu ouvert, à n'importe quel moment. Ctrl+C pour arrêter. Le dossier,
par défaut map_pins/runs/<date-heure>/, reçoit :
- frida.log : événements natifs (BroadcastPlayerInfo, NetPlayerInfo lu et traité), écarts de 50 ms
  et plus entre deux PeekMessage d'un même thread (images longues), bilan des images toutes les 5 s ;
- lua.tsv : les lignes MesureEpingles de Lua.log écrites pendant la session ;
- un résumé par fonction racine, affiché aussi à l'écran.
"""

import collections
import datetime
import json
import os
import pathlib
import sys
import time

import frida

HERE = pathlib.Path(__file__).resolve().parent
PROCESS_NAMES = ("CivilizationVI_DX12.exe", "CivilizationVI.exe")
LUA_LOG = pathlib.Path(os.environ["LOCALAPPDATA"]) / "Firaxis Games" / "Sid Meier's Civilization VI" / "Logs" / "Lua.log"


def find_game(device):
    for p in device.enumerate_processes():
        if p.name in PROCESS_NAMES:
            return p
    return None


def lua_lines(start):
    """Lignes MesureEpingles de Lua.log après l'octet start (Lua.log est recréé à chaque lancement)."""
    if not LUA_LOG.exists():
        return []
    with open(LUA_LOG, "rb") as f:
        if LUA_LOG.stat().st_size >= start:
            f.seek(start)
        text = f.read().decode("utf-8", errors="replace")
    return [l for l in text.splitlines() if "MesureEpingles\t" in l]


def summary(lines, events):
    roots = collections.defaultdict(list)
    for l in lines:
        cols = l.split("\t")
        ms = next((c[3:] for c in cols if c.startswith("ms=")), None)
        if ms is None:
            continue
        # Lua.log préfixe chaque ligne du nom du contexte : "MapPinManager: MesureEpingles".
        i = next(k for k, c in enumerate(cols) if c.endswith("MesureEpingles"))
        roots[(cols[i + 3], cols[i + 4])].append(float(ms))
    out = ["racine (contexte, fonction)\tappels\tmoyenne ms\tmax ms\ttotal ms"]
    for (ctx, label), v in sorted(roots.items(), key=lambda kv: -sum(kv[1])):
        out.append(f"{ctx} {label}\t{len(v)}\t{sum(v) / len(v):.1f}\t{max(v):.1f}\t{sum(v):.0f}")
    lus = [e["octets"] for e in events if e["kind"] == "net_player_info_lu"]
    out.append(f"BroadcastPlayerInfo locaux : {sum(e['kind'] == 'broadcast_player_info' for e in events)}")
    if lus:
        out.append(f"NetPlayerInfo reçus : {len(lus)}, octets moyen {sum(lus) / len(lus):.0f}, max {max(lus)}")
    out.extend(frames_summary(events))
    return "\n".join(out)


def frames_summary(events, window_ms=1000):
    """Gels du thread principal après chaque NetPlayerInfo reçu, comparés au reste de la session.

    Le thread principal est celui qui traite NetPlayerInfo (il fait aussi tourner l'interface Lua) ;
    à défaut, celui qui appelle le plus PeekMessage."""
    recv = [e for e in events if e["kind"] == "net_player_info_traite"]
    frames = [e for e in events if e["kind"] == "image_longue"]
    if not frames:
        return ["Images : aucun écart de 50 ms ou plus"]
    if recv:
        main_tid = collections.Counter(e["tid"] for e in recv).most_common(1)[0][0]
    else:
        calls = collections.Counter()
        for e in events:
            if e["kind"] == "images":
                calls[e["thread"]] += e["appels"]
        main_tid = calls.most_common(1)[0][0]
    frames = [e for e in frames if e["tid"] == main_tid]
    out = [f"Images longues (>= 50 ms) du thread principal {main_tid} : {len(frames)}, "
           f"max {max((e['ms'] for e in frames), default=0)} ms"]
    if not recv:
        return out
    # PeekMessage note l'écart quand la boucle reprend : un gel causé par une réception à t finit après t.
    after, near = [], set()
    for r in recv:
        hit = [f for f in frames if 0 <= f["t"] - r["t"] <= window_ms]
        near.update(id(f) for f in hit)
        after.append(max((f["ms"] for f in hit), default=0))
    others = [f["ms"] for f in frames if id(f) not in near]
    after.sort()
    out.append(f"Gel max dans la seconde qui suit un NetPlayerInfo reçu ({len(after)}) : "
               f"médiane {after[len(after) // 2]} ms, max {after[-1]} ms, "
               f">= 100 ms : {sum(a >= 100 for a in after)}, >= 250 ms : {sum(a >= 250 for a in after)}")
    out.append(f"Images longues ailleurs : {len(others)}, max {max(others, default=0)} ms")
    return out


def main():
    out_dir = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else (
        HERE / "runs" / datetime.datetime.now().strftime("%Y%m%d-%H%M%S"))
    out_dir.mkdir(parents=True, exist_ok=True)
    device = frida.get_local_device()

    proc = find_game(device)
    while proc is None:
        print("En attente de Civilization VI…", flush=True)
        time.sleep(2)
        proc = find_game(device)

    lua_start = LUA_LOG.stat().st_size if LUA_LOG.exists() else 0
    log = open(out_dir / "frida.log", "a", encoding="utf-8")
    events = []
    done = []

    def on_message(message, data):
        if message["type"] == "send":
            p = dict(message["payload"])
            events.append(p)
            q = dict(p)
            t = datetime.datetime.fromtimestamp(q.pop("t") / 1000)
            line = f"{t:%Y-%m-%d %H:%M:%S.%f}"[:-3] + f"\t{q.pop('tid')}\t{q.pop('kind')}\t" + json.dumps(
                q, ensure_ascii=False)
        else:
            line = f"{datetime.datetime.now():%Y-%m-%d %H:%M:%S}\t-\terreur_script\t{message.get('description')}"
        log.write(line + "\n")
        log.flush()
        print(line, flush=True)

    js = (HERE / "hook_epingles.js").read_text(encoding="utf-8")
    session = device.attach(proc.pid)
    session.on("detached", lambda reason, crash: done.append(reason))
    script = session.create_script(js)
    script.on("message", on_message)
    script.load()
    print(f"Attaché à {proc.name} (pid {proc.pid}). Dossier : {out_dir}", flush=True)
    try:
        while not done:
            time.sleep(0.5)
        print(f"Détaché : {done[0]}", flush=True)
    except KeyboardInterrupt:
        session.detach()
    log.close()

    lines = lua_lines(lua_start)
    (out_dir / "lua.tsv").write_text("\n".join(lines) + "\n", encoding="utf-8")
    text = summary(lines, events)
    (out_dir / "resume.txt").write_text(text + "\n", encoding="utf-8")
    print(text, flush=True)


if __name__ == "__main__":
    main()
