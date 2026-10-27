# Carte des fichiers source de GameCore_XP2

Carte fichier source → fonctions de `GameCore_XP2_FinalRelease.dll`, la DLL de règles qui tourne en
Gathering Storm (SHA-256 `324c51e9…c98c`, build du 2024-06-26). Les adresses sont celles du projet
Ghidra `~/tools/ghidra-projects/civ6.gpr` (base 0x180000000), donc les `FUN_xxxx` du code et des notes.

Établie le 2026-10-27 avec Ghidra 12.1.4, sur deux builds : FinalRelease (34 920 fonctions, sans
symboles) et `GameCore_XP2_Release.dll`, du même source avec ses asserts (41 253 fonctions).

## Fichiers

- `functions.tsv` : une ligne par fonction FinalRelease.
  - `adresse`, `octets`.
  - `fichier` : chemin relatif à `Civ6\Src\` (ou `CivTech\` pour les libs), vide si inconnu.
  - `methode` : voir plus bas.
  - `bornes` : pour une fonction inconnue, les fichiers rattachés qui l'encadrent (`A .. B`). Elle est
    dans A, dans B ou dans un `.cpp` que Release place entre les deux.
  - `lignes` : numéros `__LINE__` cités, en général ceux d'une allocation.
  - `noms_lua` : noms sous lesquels la fonction est enregistrée en Lua, par exemple `GetTurnsUntilGrowth`.
  - `jumelle_release` : adresse de la même fonction dans la DLL Release.
  - `indices_release` : chaînes que seule la jumelle Release cite, surtout des textes d'assert
    (`!"eLuxuryRecipient != player->GetID()"`) et des noms de portée du profileur
    (`Game::Processor::BeginTurn`). C'est le meilleur moyen de nommer une `FUN_`.
- `files.md` : un tableau par dossier, puis un par fichier, dans l'ordre de `.text` (début, fin,
  nombre de fonctions, octets, méthodes, ligne max, noms Lua).
- `files.json` : la même chose en JSON.

## Couverture

40,9 % des octets de code ont un fichier : 11 706 fonctions sur 34 920, pour 795 des 900 `.cpp` connus
de Release. Les 23 067 fonctions inconnues sont petites (130 octets en médiane, 690 seulement dépassent
1 Ko) : des utilitaires, des templates, du code inline. Chacune a des bornes.

| méthode | fonctions | % octets | principe |
|---|---:|---:|---|
| `fichier` | 891 | 2,5 | la fonction cite le `__FILE__` du `.cpp` (`alloc(taille, align, __FILE__, __LINE__)`) |
| `bloc` | 6 531 | 17,7 | elle se trouve entre deux fonctions du même `.cpp` |
| `chaine` | 883 | 8,8 | elle cite une chaîne que seul ce `.cpp` cite dans Release |
| `release` | 3 348 | 11,7 | sa jumelle Release est rattachée à ce `.cpp` |
| `lua` | 53 | 0,2 | elle porte un nom Lua que seul ce `.cpp` enregistre dans Release |
| `inconnu` | 23 067 | 59,0 | bornée par `bornes` |

## Pourquoi ça marche

- **Les objets ne s'entrelacent pas.** En FinalRelease, chacun des 502 `.cpp` cités forme un seul
  bloc contigu dans `.text`. En Release, c'est le cas de 879 fichiers sur 900. Les exceptions sont des
  `GameEffects_*`, dont le code template est instancié ailleurs. Le linker a gardé l'ordre des objets.
- **L'ordre est celui du projet.** C'est presque un tri ASCII sensible à la casse (`BuildQueue` avant
  `Buildings`), mais selon les dossiers les fichiers passent avant ou après les sous-dossiers. Les deux
  builds suivent le même ordre. Au niveau global : `Core`, `XP2`, `Shared`, `GameEffects`, libs `CivTech`.
- **Appariement des builds** (`sourcemap_match.py`) :
  - graines : 5 936 paires sûres (chaîne unique des deux côtés, nom Lua unique, couple `__FILE__` +
    ligne), dont on garde la plus longue sous-suite croissante, soit 5 533 paires ;
  - alignement par programmation dynamique entre graines : taille, nombre d'appelés, appelés déjà
    appariés ;
  - résultat : 29 225 fonctions appariées.
  - Validation : quand on cache une graine sur cinq, l'alignement en retrouve 97,7 % et en rate 0,4 %.
    Les 772 ancres `__FILE__` qui ont une jumelle concordent toutes avec le fichier de leur jumelle.
- Tout rattachement transféré (`chaine`, `lua`, `release`) doit respecter l'ordre des fichiers, sinon
  il est rejeté (204, 27 et 165 rejets).

## Pistes essayées et écartées

- **Fichier unique des appelants.** Sur les ancres, il ne donne le bon fichier que 8 fois sur 30 : les
  utilitaires partagés sont appelés de partout.
- **Fichier de la fonction qui enregistre un binding Lua.** 39 justes / 13 faux en Release, 0 / 5 en
  FinalRelease. Chaque nom Lua est enregistré deux fois, côté `Cache_Lua_*` (UI) et côté `Lua_*`
  (gameplay).
- **Coupure d'un trou A|B par affinité d'appels.** 65 à 67 % de bonnes places sur les rares frontières
  testables : pas assez pour l'utiliser.
- **RTTI de Release.** Elle ne couvre que des types de pointeurs de fonction (12 vtables).

## Limites

- Le code template ou inline est placé dans le premier objet qui l'instancie : `bloc` peut l'attribuer à
  un `.cpp` qui ne fait que l'utiliser.
- Les `.cpp` qui ne citent jamais `__FILE__`, même en Release, sont invisibles. Leurs fonctions restent
  `inconnu` avec des bornes.
- Seule FinalRelease est vérifiée. Une mise à jour du jeu change toutes les adresses.

## Exemple

Les fonctions de la répartition des luxes (voir la mémoire `luxury-allocation-dll`) tombent toutes
dans `XP2\Common\Player_Resources.cpp` (18031cd20–180323220, 82 fonctions), sauf deux :
- les accords, dans `DiplomacyDeals\Deal_Item_Resources.cpp` (FUN_180587d50) ;
- les imports des cités-États, dans `Player_Influence.cpp` (FUN_1802ebe60). Indice Release de cette
  dernière : `!"eLuxuryRecipient != player->GetID()"`.

## Refaire

Scripts dans `~/tools/ghidra-scripts`. Python : `.venv/Scripts/python.exe -I` du dépôt.

```bash
# 1. export Ghidra (headless, projet en lecture seule), pour chaque DLL
analyzeHeadless ~/tools/ghidra-projects civ6 -process GameCore_XP2_FinalRelease.dll -noanalysis -readOnly -scriptPath ~/tools/ghidra-scripts -postScript SourceMap.java ~/tools/ghidra-projects/sourcemap
analyzeHeadless ~/tools/ghidra-projects civ6 -process GameCore_XP2_Release.dll -noanalysis -readOnly -scriptPath ~/tools/ghidra-scripts -postScript SourceMap.java ~/tools/ghidra-projects/sourcemap_release
# 2. consolidation
python -I ~/tools/ghidra-scripts/sourcemap_build.py ~/tools/ghidra-projects/sourcemap ~/tools/ghidra-projects/sourcemap_release docs/gamecore_xp2
# contrôles : sourcemap_check.py, sourcemap_match_check.py, sourcemap_split_check.py
```
