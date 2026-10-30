"""Attache hook_reconnect.js au jeu de l'hôte et écrit ses événements dans un journal horodaté.

    .venv/Scripts/python.exe -I netcode/hook_reconnect.py [dossier]

Lancer après avoir démarré le jeu (version DX12), avant de charger la partie. Le journal va dans
<dossier>/frida.log, par défaut netcode/runs/<date-heure>/. Ctrl+C pour arrêter.
"""

import datetime
import json
import pathlib
import sys
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
    out_dir = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else (
        HERE / "runs" / datetime.datetime.now().strftime("%Y%m%d-%H%M%S"))
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

    def on_message(message, data):
        if message["type"] == "send":
            p = dict(message["payload"])
            t = datetime.datetime.fromtimestamp(p.pop("t") / 1000)
            line = f"{t:%Y-%m-%d %H:%M:%S.%f}"[:-3] + f"\t{p.pop('tid')}\t{p.pop('kind')}\t" + json.dumps(
                p, ensure_ascii=False)
        else:
            line = f"{datetime.datetime.now():%Y-%m-%d %H:%M:%S}\t-\terreur_script\t{message.get('description')}"
        log.write(line + "\n")
        log.flush()
        print(line, flush=True)

    session = device.attach(proc.pid)
    session.on("detached", lambda reason, crash: done.append(reason))
    script = session.create_script((HERE / "hook_reconnect.js").read_text(encoding="utf-8"))
    script.on("message", on_message)
    script.load()
    print(f"Attaché à {proc.name} (pid {proc.pid}). Journal : {log_path}", flush=True)
    try:
        while not done:
            time.sleep(0.5)
        print(f"Détaché : {done[0]}", flush=True)
    except KeyboardInterrupt:
        session.detach()
    log.close()


if __name__ == "__main__":
    main()
