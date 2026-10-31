# File de production : merveille supprimée après deux échanges

Gêne observée en partie (2026-10-31). La file contient A (merveille, ajoutée pour la première fois)
puis B. Un échange donne B puis A. L'échange inverse ne laisse que B : A a disparu.

## Côté Lua

`SwapQueueItem` (`Base/Assets/UI/ProductionHelper.lua` du jeu, ligne 205) envoie seulement `CityOperationTypes.BUILD` avec `VALUE_SWAP` et les deux index. Le
retrait se fait dans GameCore.

## Côté GameCore

Projet Ghidra `civ6xp2` (FinalRelease d'origine, SHA-256 `324c51e9…c98c`, base 0x180000000).

| adresse | rôle |
|---|---|
| `180683900` | lecture des paramètres de l'opération BUILD : mode d'insertion en `+0x20` (9 et 10 pour les deux modes à source et destination), index en `+0x24` / `+0x28` |
| `1800f4d70` | application à la file. Op 9 : `180104810` échange les deux éléments (0x38 octets). Si un index vaut 0, la tête change : `+0x3ac` = 1 puis `1800fc5d0` |
| `1800fc5d0` → `180100e80` | relance de l'élément en tête selon son type ; bâtiment : `1801007a0` |
| `1801007a0` | lancement d'un bâtiment. Pour une merveille (définition `+0x159 & 0x10`) : `18025bad0` sur la case de l'élément (`+0x28`, `+0x2c`), puis `1801020b0`. **Si `18025bad0` refuse, `1800ffbf0` retire l'élément de tête** |
| `18025bad0` | CanHaveWonderAt(case, bâtiment, joueur, …, ville) |
| `1801020b0` | pose la merveille : case enregistrée dans `m_aiConstructionPlot` (file `+0x290`, `short`, taille en `+0x2a0`), bâtiment placé sur la carte, caractéristique ou ressource retirée. Ne fait rien si la case est déjà enregistrée ou si la progression est positive |

La file est incluse dans la ville en `+0x10d0` (`lea rcx,[ville+0x10d0]` en `18006c8cc`), la ville en
file `+0x1e8`.

## Le déroulé

1. A passe en tête pour la première fois : la case est libre, `18025bad0` accepte et `1801020b0`
   pose A.
2. Premier échange : B passe en tête, A reste posée sur sa case.
3. Second échange : A revient en tête. `1801007a0` rappelle `18025bad0`, mais la case porte
   maintenant le chantier de A. Le test refuse et A est retirée.

`1801020b0` sait qu'une merveille déjà posée ne se revérifie pas ; seul le test de `1801007a0` manque
cette exception.

## Trace en jeu

[hook_swap.js](hook_swap.js), lancé par :

```bash
.venv/Scripts/python.exe -I wonder_queue/hook_swap.py
```

Partie du 2026-10-31, échanges répétés de deux merveilles (index 51, puis 48) avec un bâtiment
(index 3) :

```
op       {"op": 9, "p4": 1, "p5": 0, "taille": 2}
start    {"batiment": 51, "x": 48, "y": 29, "case_posee": 2484, "progression": 0}
wonderAt {"batiment": 51, "x": 48, "y": 29, "appelant": "0x100a9f", "ret": 0}
dropFront {"appelant": "0x100aab"}
```

`case_posee` vaut 29 × 84 + 48 : c'est la case visée. Au premier passage en tête, le même appel
renvoyait 1. Même chose pour la merveille 48. `CanConstruct` (`1800f61c0`) accepte à chaque fois.

## Passage sur le réseau

Le multijoueur est un lockstep de commandes (voir [docs/netcode](../docs/netcode/README.md)) : chaque
machine fait tourner la simulation, seules les commandes circulent. L'interface ne modifie jamais
GameCore directement, elle lit un cache. Choisir une production et échanger deux éléments passent
par le même `CityManager.RequestOperation(ville, BUILD, paramètres)` : seul le mode d'insertion
change (`VALUE_SWAP` pour l'échange). Côté GameCore, `1800f4d70` n'est appelée que par
`18074fd60` (`XP2_City_Operations_Build.cpp`, exécution de l'opération, référencée seulement par
des vtables) et par `180105850`. L'échange est donc rejoué chez chaque joueur, avec le même retrait.

Si un seul joueur corrige ce retrait, les files divergent. Le passage de tour, calculé localement
par chaque machine, produit alors une merveille d'un côté et autre chose de l'autre. Le contrôle de
synchro de fin de tour doit le détecter, et l'hôte recopie ses archives chez le client : c'est la
version de l'hôte qui reste. Je ne l'ai pas observé pour ce cas précis.

## Prise de ville

Le jeu retire tout chantier de merveille de la case à la prise de la ville. Côté GameCore, la
destruction de la file (`1800f38e0`, appelée depuis `City_Instance.cpp` et `City_Manager.cpp`)
appelle `1801034d0`, qui appelle `1800ffc20` pour chaque merveille. `1800ffc20` :
- retire le chantier de la carte (virtuelle `+0xa8` du gestionnaire de bâtiments, puis
  `18025d460` si la case porte encore cette merveille, `info+0x1e`) ;
- remet `m_aiConstructionPlot` à -1 et la progression à 0 ;
- retire la merveille de la file.

Le correctif ne s'applique qu'à une case qui porte encore le chantier : ce comportement ne change
pas.

## Correctif dans GameCore (mis de côté)

[merveilles_gamecore.rs](merveilles_gamecore.rs), ancien module du proxy, réglage
`merveilles_file=on|off`. Il n'est plus compilé : le rattrapage par commandes (`ui_merveilles`, plus
bas) le remplace sans risque de désynchro. Le `call` de `180100a9a` mène à un relais. Le test réussit si les trois conditions
suivantes sont vraies ; sinon, appel d'origine :
- la case visée est celle enregistrée dans la file pour cette merveille (`m_aiConstructionPlot`) ;
- la case porte toujours cette merveille (`18004_2d00(case)+0x1e`, lu par `1800ffc20`) ;
- la case appartient toujours à cette ville (`+0xc` comparé à `ville+0xa8`, comme dans `18025bad0`).

`1801020b0` prend alors son chemin « déjà posée », comme pour une merveille entamée. Une prise de
ville, un chantier retiré par un autre chemin ou une case passée à une autre ville font échouer une
des conditions : le jeu décide alors comme avant.

**Ceci change la partie.** Chaque joueur applique l'opération dans son GameCore : en multijoueur, un
joueur sans le réglage retire la merveille, les autres non, et la partie se désynchronise. À
activer seulement si tous les joueurs ont le proxy avec `merveilles_file=on`.

Les deux champs `+0x1e` et `+0xc` viennent de la lecture du code. La trace les relève
(`merveille_case`, `ville_case`, `ville` dans `wonderAt`) pour les vérifier en jeu.

## Piste sans désynchro : rattrapage par commandes

Le correctif de GameCore change une règle chez un seul joueur. Autre piste : le joueur qui a le
proxy garde les règles du jeu, subit le retrait comme les autres, puis son interface renvoie des
commandes ordinaires (rajout de la merveille, échange) qui la remettent en tête. Chaque machine
rejoue ces commandes avec les mêmes règles : la file est la même partout, proxy ou non.

Le retrait passe par `1800ffbf0` (`180101370` puis `180101400`), qui décale la file sans toucher au
chantier : la case et `m_aiConstructionPlot` restent. La piste suppose que la merveille, rajoutée
puis remise en tête, n'est pas retirée une seconde fois. À vérifier en jeu.

### Test

`merveilles_file=off` dans `rollover_proxy.cfg` (sinon le bug ne se déclenche pas ; aujourd'hui,
`ui_merveilles=off` pour voir le bug sans rattrapage), partie chargée,
puis `.venv/Scripts/python.exe -I wonder_queue/hook_swap.py`. La trace donne le contenu de la file
(`file`) à chaque `op`, `op_fin` et `start_fin`.

Dans une ville, file ouverte :
1. ajouter une merveille A jamais posée, sur sa case, puis un bâtiment B : file A, B ;
2. échanger : B, A ; échanger encore : A disparaît (`dropFront`), file B ;
3. rajouter A (elle est proposée sur sa case) : B, A ;
4. échanger : A doit rester en tête (`wonderAt` à 1, pas de `dropFront`), file A, B ;
5. finir le tour, puis vérifier que la production est allée à A.

Si A est retirée à l'étape 4, la piste ne tient pas : noter le `op`, le `wonderAt` et la
progression de A.

### Résultat du 2026-10-31 (`runs/20261031-121554`)

Merveille 95 sur la case 1578, unité 1 derrière :

| heure | action | file après |
|---|---|---|
| 12:16:04 | ajout de 95, avec sa case (66, 18) : `wonderAt` = 1, pose | 95 (case 66, 18) |
| 12:16:12 | échange | u1, 95 |
| 12:16:13 | échange : `wonderAt` = 0, `dropFront` | u1 |
| 12:16:27 | rajout de 95 : l'élément n'a **pas de case** (x, y = -9999) | u1, 95 |
| 12:16:29 | échange : pas d'appel à `wonderAt`, pose (déjà faite) | 95, u1 |

**Pourquoi le rajout tient.** Le panneau rajoute une merveille déjà posée sans case. `1801007a0`
prend alors l'autre branche : `18064d630` cherche la case qui porte déjà cette merveille
(`+0x1e`), sans appeler `18025bad0`. Seul un élément qui porte encore ses coordonnées est
revérifié. « Seulement la première fois » vient de là : l'élément d'origine a sa case, le rajout
non.

**Conséquence pour le rattrapage.** Après le retrait, l'interface du joueur avec proxy envoie le
rajout comme le panneau (sans case), puis l'échange qui le remet en tête. Toutes les machines
rejouent ces deux commandes avec les règles du jeu et arrivent à la même file.

Au passage de tour, la production est bien allée à la merveille (constaté en jeu).

### Mise en œuvre : `ui_merveilles`

[proxy/src/ui_merveilles.lua](../proxy/src/ui_merveilles.lua), ajouté par le proxy à la fin de
`ProductionHelper.lua` (réglage `ui_merveilles=on|off`, on par défaut). `SwapQueueItem` est
enveloppée : quand l'échange amène une merveille en tête, on note la file attendue. Au
`CityProductionQueueChanged` suivant de cette ville :
- file inchangée : l'échange n'est pas encore appliqué, on attend ;
- file attendue sans sa tête : la merveille a été retirée. On envoie son rajout en fin de file
  (`VALUE_APPEND`, sans case, comme le panneau), puis des échanges voisins qui la ramènent en tête
  sans changer l'ordre des autres éléments ;
- autre file : rien.

Seul le contexte où l'échange a été demandé réagit (le panneau et le gestionnaire de production
chargent chacun `ProductionHelper.lua`). Test hors jeu, avec une file qui imite GameCore et une
machine sans le bloc qui rejoue le même flux de commandes : `lua proxy/tests/test_ui_merveilles.lua`.

Validé en jeu le 2026-10-31 : échange puis échange inverse d'une merveille posée, elle revient en
tête d'elle-même. L'identifiant
de ville 393221, revu à 12:16:40 avec une file vide, est donc celui d'une ville d'un autre joueur :
la trace note désormais le joueur (`joueur`) avec la ville.
