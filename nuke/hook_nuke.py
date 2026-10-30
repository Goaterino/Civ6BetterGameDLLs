"""Attache hook_nuke.js au jeu et écrit ses événements dans un journal horodaté.

    .venv/Scripts/python.exe -I nuke/hook_nuke.py [--accel 3] [dossier]

Lancer après avoir démarré le jeu (version DX12). Le journal va dans <dossier>/frida.log, par défaut
nuke/runs/<date-heure>/. Ctrl+C pour arrêter.

--accel : pendant la visualisation d'une frappe WMD, multiplie l'échelle de temps de l'affichage
(celle de la commande de console « timescale ») ; elle est remise à sa valeur à CombatVisEnd et au
détachement.
"""

import argparse
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
    ap = argparse.ArgumentParser()
    ap.add_argument("--accel", type=float, default=1.0)
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
        # Les piles vont au journal seulement : la console garde une ligne par événement.
        print(line.split('"pile"')[0][:200], flush=True)

    session = device.attach(proc.pid)
    session.on("detached", lambda reason, crash: done.append(reason))
    source = (HERE / "hook_nuke.js").read_text(encoding="utf-8")
    if "const ACCEL = 1;" not in source:
        sys.exit("hook_nuke.js : constante ACCEL introuvable")
    script = session.create_script(source.replace("const ACCEL = 1;", f"const ACCEL = {a.accel!r};"))
    script.on("message", on_message)
    script.load()
    print(f"Attaché à {proc.name} (pid {proc.pid}), accélération ×{a.accel:g}. Journal : {log_path}", flush=True)
    try:
        while not done:
            time.sleep(0.5)
        print(f"Détaché : {done[0]}", flush=True)
    except KeyboardInterrupt:
        try:
            script.exports_sync.restaurer()
        except frida.InvalidOperationError:
            pass
        session.detach()
    log.close()


if __name__ == "__main__":
    main()
