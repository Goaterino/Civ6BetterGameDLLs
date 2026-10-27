# Proxy de GameCore : regroupement des instantanés de villes

Prototype de la piste 1 de [cascade_guerre.md](../cascade_guerre.md). Pendant une cascade de
déclarations de guerre, le jeu refait l'instantané de toutes les villes des belligérants à chaque
déclaration : environ 2 900 instantanés pour 61 villes au tour 79, soit 2,2 s. Le proxy n'en fait plus
qu'un par couple (événement, joueur), à la fin de la cascade.

## Principe

- **Substitution.** L'original `DLC\Expansion2\Binaries\Win64\GameCore_XP2_FinalRelease.dll` est
  renommé `GameCore_XP2_FinalRelease_orig.dll`, et le proxy prend son nom. Le nom de la DLL vient de
  `Expansion2.modinfo`, l'exe ne le contient pas.
- **Relais.** Le proxy relaie les 157 exports de l'original sous les mêmes noms, noms C++ décorés
  compris. La table `src/exports.rs` est générée par `gen_exports.py`. L'exe n'utilise que
  `DllCreateGameContext`, `DllDestroyGameContext` et `EXP_GetTelemetrySessionHash`, qu'il récupère
  par nom ; les 154 autres (API Lua de Havok) ne sont importés par aucun binaire du jeu. Le relais
  générique à 8 entiers ne transmet pas les arguments flottants, ce qui est sans effet pour ces 3
  fonctions.
- **Chargement de l'original.** Au premier appel relayé (`DllCreateGameContext`, hors du verrou du
  chargeur), le proxy charge l'original et vérifie octet par octet les deux sites. S'ils
  correspondent, il pose ses détours ; sinon, il ne patche rien et le note dans le journal.
- **Les deux détours :**
  - `1802a1f90` (DeclareWar) : un compteur de profondeur repère la cascade, rattachée au thread qui
    l'a ouverte. Le test `cmp edx,-1 / jz` du prologue est refait dans le détour, puisqu'un saut
    relatif ne se recopie pas.
  - `180072f80(événement, joueur)` : pendant la cascade, l'appel est noté au lieu d'être exécuté.
    À la sortie de l'appel le plus externe, chaque couple est rejoué une fois, dans l'ordre de
    première apparition, donc sur l'état final.
- **Déchargement.** Si le jeu décharge le proxy alors que l'original reste en mémoire, le proxy y
  remet les octets d'origine. Pour la même raison, l'état de cascade est un statique et non un
  `thread_local!`, dont les destructeurs survivraient au déchargement.
- **Pourquoi c'est sans risque pour la synchronisation.** L'instantané ne fait que lire l'état de jeu
  (vérifié dans la note), et sa réserve n'est lue par aucun code de règles. Différer ces instantanés
  ne change que le moment où l'interface reçoit ses mises à jour de villes.

### Pistes écartées

Les deux premiers prototypes se plaçaient à côté de l'exe, sous le nom d'une DLL système qu'il
importe :
- **`version.dll`** n'a jamais été chargé. L'exe démarre avec le moteur de compatibilité de Windows
  (`apphelp.dll`, `AcGenral.DLL`), qui importe `VERSION.dll` depuis System32 avant que l'exe ne
  résolve ses imports. Le module déjà chargé sous ce nom est réutilisé.
- **`dxgi.dll`** a bien été chargé, mais n'a pas réussi à charger la vraie `dxgi.dll` de System32.
  Le jeu a alors affiché « aucune carte graphique compatible » (`Renderer.log` :
  `Failed to create DXGI factory. Error: 0x80004001`).

Se substituer à GameCore évite toute interaction avec Windows et le graphisme. Et un échec reste
limité au lancement d'une partie Gathering Storm : le menu principal utilise `GameCore_Base`.

## Modes

Le mode se règle dans `rollover_proxy.cfg`, à côté de la DLL, par une ligne `mode=…` :

| mode | effet |
|---|---|
| `coalesce` (défaut) | regroupe les instantanés |
| `count` | pose les détours, ne change rien, compte et chronomètre les cascades |
| `off` | aucun détour |

**Journal** : `%LOCALAPPDATA%\Civ6RolloverProxy\RolloverProxy.log`, hors du dossier `Logs` du jeu.
On y trouve :
- `proxy chargé dans <exe>`, au premier appel relayé ;
- une ligne au chargement de l'original, qui dit si les détours sont posés ;
- une ligne par cascade :

```
cascade mode=Count declarewar=358 profondeur_max=… instantanes_demandes=… uniques=… cascade_ms=… rejeu_ms=… joueurs=[…]
```

## Compiler, tester, installer

```bash
cd perf/rollover/proxy && cargo build --release
```

**Régénérer la table des exports**, après une mise à jour du jeu :

```bash
.venv/Scripts/python.exe -I perf/rollover/proxy/gen_exports.py "<jeu>\DLC\Expansion2\Binaries\Win64\GameCore_XP2_FinalRelease.dll" perf/rollover/proxy/src/exports.rs
```

**Test hors jeu.** Il vérifie :
- les exports, identiques à ceux de l'original, plus les trois du proxy ;
- le chargement de l'original et la lecture de la configuration ;
- les sauts écrits aux deux sites ;
- la remise des octets d'origine au déchargement.

L'original refuse de s'initialiser hors du jeu : le test le projette d'abord sans initialisation,
et le proxy récupère ce module tel quel.

```bash
.venv/Scripts/python.exe -I perf/rollover/proxy/tests/check_hooks.py <proxy compilé> <GameCore_XP2_FinalRelease.dll d'origine> <dossier temporaire>
```

**Installation**, jeu fermé. Le script :
- renomme l'original en `_orig`, sans jamais le supprimer ;
- copie le proxy et `rollover_proxy.cfg` ;
- retire les anciens prototypes posés à côté de l'exe.

`-Uninstall` remet l'original à sa place. Une vérification des fichiers par Steam ou une mise à jour
du jeu remet aussi l'original : il faut alors réinstaller. Si l'original est en place alors qu'un
`_orig` existe déjà, le script s'arrête plutôt que de choisir lequel garder.

```bash
powershell -ExecutionPolicy Bypass -File perf/rollover/proxy/install.ps1 -Mode count
```

```bash
powershell -ExecutionPolicy Bypass -File perf/rollover/proxy/install.ps1 -Uninstall
```

## Protocole de mesure en jeu

On part de `AutoSave_0078`, dont le passage au tour 79 déclenche la cascade, et on répète ce qui
suit pour chaque mode, en relançant le jeu entre deux modes :

1. Installer le proxy dans le mode voulu.
2. Charger `AutoSave_0078` et finir le tour sans rien jouer (en option, avec `capture.ps1` pour une
   trace WPR).
3. Au tour 79, finir encore le tour sans rien jouer. Copier de côté `AutoSave_0080.Civ6Save`, qui
   contient l'état après tout le tour 79.
4. Relever la ligne `cascade` du journal du proxy, et la durée du passage dans
   `net_message_debug.log` (`analyze.py`).

Les comparaisons à faire :

| comparaison | ce qu'elle montre |
|---|---|
| `count` contre `coalesce` | le gain : `cascade_ms`, et le nombre d'instantanés demandés pour 586 déclarations, qui dit si une déclaration rafraîchit un joueur ou deux |
| deux passages en `off` | si le jeu est reproductible : les deux `AutoSave_0080` sont-elles identiques à l'octet près ? |
| `off` contre `coalesce` | si les sauvegardes sont identiques. Ce test n'a de sens que si la comparaison précédente l'est déjà, sans quoi il faut comparer le contenu décodé, pas les octets |

## Résultats (27 octobre 2026, `AutoSave_0078` → tour 80)

**Gain.** Au passage au tour 79, la cascade compte 358 appels à DeclareWar et 1 172 instantanés
demandés, pour 17 couples (événement, joueur) différents, tous sur le même événement
`0x7dddaff1`. Une déclaration rafraîchit donc environ 3,3 joueurs, pas seulement les deux
belligérants.

| partie | mode | cascade | dont rejeu | passage de tour (à la seconde) |
|---|---|---|---|---|
| solo | count | 2 946 ms | – | 8 s |
| solo | count | 2 830 ms | – | 8 s |
| en ligne | count | 2 878 ms | – | – |
| solo | coalesce | 528 ms | 48 ms | 6 s |
| solo | coalesce | 513 ms | 48 ms | 6 s |
| en ligne | coalesce | 560 ms | 49 ms | 7 s, contrôle de synchronisation compris |

La cascade perd environ 2,4 s, soit 82 %. Un instantané coûte environ 2,1 ms pendant la cascade,
et 2,8 ms au rejeu, sur l'état final. Les petites cascades du tour 80 (1 à 4 déclarations) pèsent
quelques ms.

**État de jeu inchangé, en solo.** On a comparé les `AutoSave_0080` décompressées (29,6 Mo) de
deux parties count et de deux parties coalesce :
- **Elles se regroupent en deux familles sans rapport avec le mode**, distantes d'environ
  38 600 octets : {count 1, coalesce 2} et {count 2, coalesce 1}. Ce qui les sépare, c'est le
  hasard des IA au rechargement : un bloc de 55 Ko, et l'ordre `UNIT_SETTLER` / `UNIT_BUILDER`
  dans 6 listes.
- **Environ 2 600 octets d'écart restent dans une même famille.** Ce sont surtout des identifiants
  qui augmentent de 9 à chaque partie, quel que soit le mode.
- **Le test décisif :** aucun octet n'est identique dans les deux parties d'un même mode et
  différent entre les deux modes.

**Synchronisation en ligne.** On a joué à deux, l'hôte avec le proxy, l'invité sans. À chaque fin
de tour, l'invité envoie les sommes de contrôle de ≈ 9 350 archives d'état (`NetGameSyncCheck`).
L'hôte répond `NetSyncSnapshot : numDeltaArchives=0`, y compris à la fin du tour 79, qui couvre
l'état d'après la cascade. Aucune désynchronisation, et le salon a accepté la DLL modifiée.
**Le proxy n'a pas besoin d'être installé chez tout le monde.** Mais chaque client calcule le
passage de tour de son côté, donc seuls ceux qui ont le proxy gagnent du temps.

## Deuxième détour : `180260720`, amélioration constructible

[src/improvements.rs](src/improvements.rs). La fonction répond à « ce joueur peut-il construire
cette amélioration ? ». Pour retrouver les traits qui interdisent une amélioration, elle parcourt une
liste de la base de règles et compare chaque nom par `strcmp`. Le détour refait le même calcul en
appelant les mêmes fonctions de l'original, dans le même ordre, mais remplace ce parcours par une
table « amélioration → traits » remplie au premier appel. La table est refaite si la liste change
d'adresse ou de taille.

Réglage : une ligne `ameliorations=off|count|verify|fast|quiet` dans `rollover_proxy.cfg`
(`install.ps1 -Ameliorations …`). Le mode `verify` calcule les deux versions, rend celle de
l'original et compte les écarts. `quiet` fait comme `fast`, mais sans compteur, sans chronomètre ni
journal : c'est le mode pour jouer. Journal : une ligne `ameliorations …` par rafale d'appels, écrite au
premier appel qui suit une seconde de pause.

Mesures sur `AutoSave_0078` → tour 80, avec `mode=coalesce`. Les durées additionnent tous les
threads :

| mode | appels | durée cumulée | par appel | écarts | passages 79 / 80 |
|---|---:|---:|---:|---:|---|
| `verify` | 4,45 M + 4,44 M + 4,27 M | 13,2 s | 0,99 µs | **0** | 6 s / 6 s |
| `count` | 13 166 324 | 11 009 ms | 0,84 µs | – | 6 s / 6 s |
| `fast`, première version | 13 166 078 | 4 380 ms | 0,33 µs | – | 5 s / 4 s |
| `verify`, deuxième version | 13 252 977 | 11,6 s | 0,89 µs | **0** | 6 s / 6 s |
| `fast`, deuxième version | 13 166 078 | **1 609 ms** | 0,12 µs | – | **5 s / 4 s** |

- **Le gain :** 9,4 s de CPU économisées en trois passages de tour, soit environ 3,1 s par passage,
  tous threads confondus. Les passages de tour, mesurés à la seconde près, gagnent 1 à 2 s.
- **La deuxième version.** Une capture WPR en mode `quiet` montrait 417 ms dans le proxy lui-même :
  le `RwLock` de la table était en contention, et un compteur de références (`Arc`) changeait à chaque
  appel, depuis 13 threads. S'y ajoutaient 253 ms de `1802209a0`, la recherche dichotomique de la
  civilisation et du dirigeant, que l'original refait à chaque trait trouvé. La table est maintenant
  lue sans verrou : entrées jamais libérées, publiées par pointeur atomique. La civilisation et le
  dirigeant sont mémorisés par joueur, revérifiés à chaque appel par l'adresse de la configuration du
  joueur. Les mémoires sont abandonnées à chaque `DllCreateGameContext`.
- **Ce qui reste dans les 0,12 µs :** les prérequis de technologie et de dogme, les deux lectures du
  compteur de performance du mode `fast`, et l'entrée dans le détour.
- **Le volume d'appels :** environ 4,4 millions par passage de tour. Les appelants (rendements de
  quartier, évaluation de l'IA) posent sans cesse la même question. Ce serait la cible suivante,
  mais une mémorisation par (joueur, amélioration) devrait être invalidée chaque fois qu'une
  technologie ou un dogme change.

## Limites du prototype

- **Un seul build du jeu.** Les adresses sont celles de GameCore_XP2 FinalRelease
  (TimeDateStamp `0x667c6f5b`). Une mise à jour désactive le proxy, qui le note dans le journal ;
  il faudrait alors retrouver les sites par motif d'octets.
- **Multijoueur.** Testé avec deux joueurs seulement, le proxy étant chez l'hôte. Le cas où seul
  l'invité a le proxy n'a pas été essayé.
- **Un seul thread suivi.** Si un instantané était demandé depuis un autre thread pendant la
  cascade, il serait exécuté normalement.
- **Interface momentanément en retard.** Pendant la cascade, les infobulles de villes ne sont plus
  rafraîchies au fil de l'eau, mais une fois à la fin.
- **Couverture partielle.** Les autres gestionnaires de cache de la cascade (≈ 380 ms) ne sont pas
  encore regroupés.
