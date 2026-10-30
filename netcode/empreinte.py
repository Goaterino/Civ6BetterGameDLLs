"""Attache empreinte.js au jeu et écrit une empreinte de synchro à chaque fin de tour.

    .venv/Scripts/python.exe -I netcode/empreinte.py [dossier]

À lancer jeu ouvert (DX12), avant de charger la partie, dans une partie multijoueur : le relevé se
déclenche sur StartGameSyncCheck. Chaque fin de tour donne <dossier>/empreinte_<n>_<heure>.tsv,
une ligne par AutoArchive : id, hachage, nom, hachages de ses variables. Dossier par défaut :
netcode/runs/empreinte-<date-heure>/. Ctrl+C pour arrêter.
"""

import datetime
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
        HERE / "runs" / ("empreinte-" + datetime.datetime.now().strftime("%Y%m%d-%H%M%S")))
    out_dir.mkdir(parents=True, exist_ok=True)
    device = frida.get_local_device()

    proc = find_game(device)
    while proc is None:
        print("En attente de Civilization VI…", flush=True)
        time.sleep(2)
        proc = find_game(device)

    done = []
    count = [len(list(out_dir.glob("empreinte_*.tsv")))]

    def on_message(message, data):
        if message["type"] != "send":
            print("erreur du script :", message.get("description"), flush=True)
            return
        p = message["payload"]
        if p["kind"] != "empreinte":
            print(f"{p['kind']} : {p['msg']}", flush=True)
            return
        count[0] += 1
        t = datetime.datetime.fromtimestamp(p["t"] / 1000)
        path = out_dir / f"empreinte_{count[0]:02d}_{t:%H%M%S}.tsv"
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            f.write("id\thachage\tnom\tvariables\tdétail\n")
            for row in sorted(p["rows"], key=lambda r: r[2]):
                f.write("\t".join(row) + "\n")
        print(f"{t:%H:%M:%S} empreinte {count[0]} : {len(p['rows'])} archives en {p['ms']} ms -> {path.name}",
              flush=True)

    session = device.attach(proc.pid)
    session.on("detached", lambda reason, crash: done.append(reason))
    script = session.create_script((HERE / "empreinte.js").read_text(encoding="utf-8"))
    script.on("message", on_message)
    script.load()
    print(f"Attaché à {proc.name} (pid {proc.pid}). Empreintes dans {out_dir}", flush=True)
    try:
        while not done:
            time.sleep(0.5)
        print(f"Détaché : {done[0]}", flush=True)
    except KeyboardInterrupt:
        session.detach()


if __name__ == "__main__":
    main()
