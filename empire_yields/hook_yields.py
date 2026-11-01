"""Attache hook_yields.js au jeu et écrit ses événements dans un journal horodaté.

    .venv/Scripts/python.exe -I empire_yields/hook_yields.py [dossier]

Lancer une fois la partie chargée. Le journal va dans <dossier>/frida.log, par défaut
empire_yields/runs/<date-heure>/. Ctrl+C pour arrêter.

Repères, pour séparer les actions faites en jeu :
- la touche Inser, pressée en jeu (sans quitter la fenêtre), écrit « repere N » dans le journal ;
- une ligne tapée dans la console (puis Entrée) y est écrite telle quelle.
"""

import argparse
import ctypes
import datetime
import json
import pathlib
import threading
import time

import frida

HERE = pathlib.Path(__file__).resolve().parent
PROCESS_NAMES = ("CivilizationVI_DX12.exe", "CivilizationVI.exe")


def find_game(device):
    for p in device.enumerate_processes():
        if p.name in PROCESS_NAMES:
            return p
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dossier", nargs="?", type=pathlib.Path)
    a = ap.parse_args()
    out_dir = a.dossier or HERE / "runs" / datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    out_dir.mkdir(parents=True, exist_ok=True)
    log_path = out_dir / "frida.log"
    device = frida.get_local_device()

    proc = find_game(device)
    while proc is None:
        print("En attente de Civilization VI…", flush=True)
        time.sleep(2)
        proc = find_game(device)

    log = open(log_path, "a", encoding="utf-8")
    done = []
    lock = threading.Lock()

    def write(line):
        with lock:
            log.write(line + "\n")
            log.flush()
            print(line, flush=True)

    def read_markers():
        while True:
            try:
                text = input()
            except EOFError:
                return
            write(f"{datetime.datetime.now():%H:%M:%S.%f}"[:-3] + f"\t-\trepere\t{text}")

    def watch_insert_key():
        # GetAsyncKeyState lit l'état global du clavier : marche quelle que soit la fenêtre active.
        VK_INSERT = 0x2D
        down, n = False, 0
        while not done:
            # Bit 0 : pressée depuis la lecture précédente, pour ne pas manquer un appui bref.
            now = bool(ctypes.windll.user32.GetAsyncKeyState(VK_INSERT) & 0x8001)
            if now and not down:
                n += 1
                write(f"{datetime.datetime.now():%H:%M:%S.%f}"[:-3] + f"\t-\trepere\t{n}")
            down = now
            time.sleep(0.03)

    def on_message(message, data):
        if message["type"] == "send":
            p = dict(message["payload"])
            t = datetime.datetime.fromtimestamp(p.pop("t") / 1000)
            line = f"{t:%H:%M:%S.%f}"[:-3] + f"\t{p.pop('tid')}\t{p.pop('kind')}\t" + json.dumps(p)
        else:
            line = f"{datetime.datetime.now():%H:%M:%S}\t-\terreur_script\t{message.get('description')}"
        write(line)

    session = device.attach(proc.pid)
    session.on("detached", lambda reason, crash: done.append(reason))
    script = session.create_script((HERE / "hook_yields.js").read_text(encoding="utf-8"))
    script.on("message", on_message)
    script.load()
    print(f"Attaché à {proc.name} (pid {proc.pid}). Journal : {log_path}", flush=True)
    threading.Thread(target=read_markers, daemon=True).start()
    threading.Thread(target=watch_insert_key, daemon=True).start()
    print("Repères : touche Inser en jeu, ou une ligne tapée ici.", flush=True)
    try:
        while not done:
            time.sleep(0.5)
        print(f"Détaché : {done[0]}", flush=True)
    except KeyboardInterrupt:
        session.detach()
    log.close()


if __name__ == "__main__":
    main()
