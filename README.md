# Civ6BetterGameDLLs

Modding de Civilization VI par DLL : un proxy de `GameCore_XP2_FinalRelease.dll` qui pose des
détours dans GameCore et dans l'exe, et les analyses qui l'ont motivé. Séparé de Civ6SimSolver
(le solveur hors partie) le 2026-10-30, avec l'historique des fichiers concernés.

- [proxy](proxy/README.md) : la DLL proxy (Rust), son installation et ses réglages
  (`rollover_proxy.cfg`) ;
- [rollover](rollover/README.md) : mesure du passage de tour (capture WPR, lecteur ETL, analyse) et
  [cascade d'une déclaration de guerre](rollover/cascade_guerre.md) ;
- [nuke](nuke/README.md) : animation des WMD (trace Frida, mod artdef, accélération dans le proxy) ;
- [ui_strike](ui_strike/README.md) : boutons d'attaque des bannières, patchés depuis le proxy ;
- [production_turns](production_turns/README.md) : compteur de tours de production qui ignore les
  bonus plats de gouvernement (fascisme) ;
- [netcode](netcode/README.md) : reconnexions et boucles de désynchro (hooks Frida, empreintes), analyse dans
  [docs/netcode](docs/netcode/README.md) ;
- [docs/gamecore_xp2](docs/gamecore_xp2/README.md) : carte des fichiers source de GameCore_XP2.

## Installation

Prérequis : Rust (`stable-x86_64-pc-windows-gnu` suffit), Python 3.10+.

```bash
python -m venv .venv
.venv/Scripts/python.exe -m pip install frida
```

Les commandes des README se lancent depuis la racine du dépôt.
