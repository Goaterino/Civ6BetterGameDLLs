# Proxy de démarrage : DatabaseDLL_FinalRelease.dll

Le proxy de GameCore ([proxy](../proxy/README.md)) n'est chargé qu'au lancement d'une partie
Gathering Storm. Ce second proxy est chargé au démarrage de l'exe, avant le menu principal. Il doit
servir de base à un onglet de réglages des proxys dans l'écran d'options du jeu.

## Pourquoi DatabaseDLL

Les deux exe (DX11 et DX12) importent statiquement quatre DLL du jeu, posées à côté d'eux dans
`Base\Binaries\Win64Steam` :

| DLL | exports |
|---|---:|
| `DatabaseDLL_FinalRelease.dll` | 5 |
| `Localization_FinalRelease.dll` | 118 |
| `HavokScript_FinalRelease.dll` | 728 |
| `steam_api64.dll` | (Steam, écartée) |

DatabaseDLL a le moins d'exports, et ce n'est pas une DLL de Windows : pas de conflit avec le moteur
de compatibilité ni avec le graphisme, contrairement aux essais avec `version.dll` et `dxgi.dll`
(voir le README du proxy de GameCore).

## Principe

- L'original est renommé `DatabaseDLL_FinalRelease_orig.dll`, et le proxy prend son nom.
- Les 5 exports (`Database::Initialize`, `Shutdown`, `CreateConnection`…) sont des renvois PE
  (*forwarders*) vers l'original : le chargeur de Windows les résout lui-même, sans code de relais.
  La chaîne d'outils est GNU, donc les renvois passent par un fichier `.def` écrit par
  [build.rs](build.rs).
- Étape 0 : le proxy ne fait qu'écrire `démarrage : proxy DatabaseDLL chargé dans <exe>` dans le
  journal commun, `%LOCALAPPDATA%\Civ6RolloverProxy\RolloverProxy.log`.

## Compiler, tester, installer

```bash
cd startup_proxy && cargo build --release
```

**Test hors jeu.** Il vérifie que le proxy se charge, que chaque export aboutit à la même adresse que
dans l'original et que la ligne de journal est écrite :

```bash
.venv/Scripts/python.exe -I startup_proxy/tests/check_forwarders.py startup_proxy/target/release/DatabaseDLL_FinalRelease.dll "<jeu>\Base\Binaries\Win64Steam\DatabaseDLL_FinalRelease.dll" <dossier temporaire>
```

**Installation**, jeu fermé. Comme pour le proxy de GameCore, une vérification des fichiers par
Steam ou une mise à jour du jeu remet l'original :

```bash
powershell -ExecutionPolicy Bypass -File startup_proxy/install.ps1
```

```bash
powershell -ExecutionPolicy Bypass -File startup_proxy/install.ps1 -Uninstall
```

## Résultat (31 octobre 2026)

Lancements en DX12 puis en DX11 jusqu'au menu principal : le journal contient
`démarrage : proxy DatabaseDLL chargé dans …\Win64Steam\CivilizationVI_DX12.exe`, puis la même
ligne avec `CivilizationVI.exe`. Le jeu démarre normalement dans les deux cas.
