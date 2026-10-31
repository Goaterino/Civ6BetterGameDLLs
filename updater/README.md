# Installeur : BetterGameDLLs.exe

Installe, met à jour, répare ou retire les deux proxys à partir de la dernière release publiée sur
GitHub (voir [release/README.md](../release/README.md)). Il est publié avec chaque release.

## Utilisation

- **Double-clic** : une fenêtre (TaskDialog de Windows) affiche le dossier du jeu, la dernière
  version et l'état de l'installation (pas installé, à jour, mise à jour disponible, à réparer), le
  détail par DLL sous « Détails », et propose installer, mettre à jour ou réparer, et désinstaller.
  Après une installation, elle donne l'option de lancement Steam, avec un bouton pour la copier.
- **`--game-dir "<dossier>"`** : même fenêtre, pour un jeu hors des bibliothèques Steam.
- **Options de lancement Steam** (Propriétés > Général) :
  ```
  "C:\chemin\vers\BetterGameDLLs.exe" %command%
  ```
  À chaque lancement, l'installeur vérifie la dernière release (environ 1 s, sans fenêtre si tout
  est à jour), installe ou répare si besoin derrière une fenêtre de progression (après une
  vérification des fichiers ou une mise à jour du jeu par Steam), puis lance le jeu et attend sa
  fin, pour que Steam le voie en cours. Un échec (GitHub injoignable, droits refusés) est affiché,
  et le jeu se lance quand même.

## Fonctionnement

- **Jeu** : le dossier qui contient `DLC/Expansion2/Binaries/Win64/GameCore_XP2_FinalRelease.dll`,
  cherché parmi les parents de l'exe lancé par Steam, sinon dans les bibliothèques de
  `steamapps/libraryfolders.vdf` (dossier de Steam lu dans `HKCU\Software\Valve\Steam`).
- **Téléchargement** : par le `curl.exe` de Windows, dans `%TEMP%\BetterGameDLLs`. Le manifeste doit
  porter une signature valide pour la clé publique embarquée, et chaque DLL le SHA-256 du manifeste.
- **Pose**, comme les `install.ps1` : l'original devient `<nom>_orig.dll`, jamais supprimé, et le
  proxy prend son nom. Un proxy se reconnaît à une chaîne qu'il contient (`RolloverProxyStatus`,
  `StartupProxyStatus`). Si Steam a remis l'original alors qu'un `_orig` existe, l'original remis
  par Steam fait foi et remplace le `_orig`. `rollover_proxy.cfg` n'est jamais touché.
- **Droits** : si le dossier du jeu n'est pas modifiable (Program Files), seule la pose est relancée
  en administrateur (`--apply`, invite UAC, sans fenêtre), qui revérifie signature et SHA-256 avant
  de copier.
- **Fenêtres** : le TaskDialog vient de comctl32 v6, qu'on active d'habitude par un manifeste
  embarqué dans l'exe. La chaîne GNU n'ayant pas `windres`, `gui.rs` l'active au démarrage par un
  contexte d'activation, à partir d'un manifeste écrit dans `%TEMP%\BetterGameDLLs`.
- **À jour** : le SHA-256 de la DLL installée est celui du manifeste ; les DLL ne connaissent pas
  leur numéro de release.

## Tester sans GitHub

`BGD_DIST` remplace GitHub par un dossier local qui contient une release signée (`manifest.json`,
`manifest.json.sig` et les DLL, produits par `release/manifest.py`) :
```
$env:BGD_DIST = "C:\...\dist"; .\target\release\BetterGameDLLs.exe --game-dir "C:\...\faux_jeu"
```
