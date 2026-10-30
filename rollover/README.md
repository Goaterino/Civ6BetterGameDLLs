# Mesure du passage de tour

Outils pour mesurer où part le temps entre la fin d'un tour et la reprise de la main au tour suivant.

- `capture.ps1` : capture WPR (CPU échantillonné à 1 ms avec piles, E/S en option), plus une copie
  des journaux du jeu. À lancer en administrateur, jeu ouvert.
- `etl.py` : lecture directe du `.etl` par l'API ETW de Windows. Elle ne décode que les images, les
  threads, les échantillons et les piles, et lit 4 Go en 1 min 40 s, là où tracerpt produirait des
  dizaines de Go de XML.
- `analyze.py` : écrit `rapport.md` dans le dossier de capture.
  - Partie journaux : durées des passages de tour et délai par joueur.
  - Partie trace : threads, modules, arbre d'appels GameCore, fonctions et fichiers `.cpp`. Les noms
    viennent de [la carte des sources](../docs/gamecore_xp2/README.md).
  - Fenêtre : par défaut, le dernier passage de tour des journaux ; `--tout` pour la capture entière.

```bash
.venv/Scripts/python.exe -I rollover/analyze.py rollover/captures/<dossier>
```

Les captures (`captures/`) ne sont pas versionnées.

## Premier constat : tour 78 → 79 (capture du 2026-10-27)

Partie multijoueur locale avec un humain, 19 cités-États, les cités libres et les barbares, mod BBG.
La sauvegarde est `AutoSave_0078`. Le passage dure 9 s, comme dans la partie d'origine.

**Déroulé.** Les IA jouent l'une après l'autre avant que l'humain reprenne la main. La cité-État 6
prend à elle seule 4 s (12:51:48 → 12:51:52).

**Les threads.**
- **Thread GameCore :** occupé 69 % de la fenêtre, soit 6,9 s de CPU.
  - Dont 73 % dans la DLL de règles, 11 % dans le noyau et 3 % dans `Localization`, appelée depuis
    `Game_Eras`.
  - La base de données ne pèse presque rien (12 ms).
- **Autres threads :** une dizaine de threads de travail tournent à environ 30 % chacun, presque
  entièrement dans l'exécutable. C'est à identifier : attente active d'un pool de tâches, ou travail
  réel ?

**Points chauds du thread GameCore**, en temps inclusif. Les noms de `.cpp` précédés de « ? » sont
déduits des fonctions voisines.

| chaîne | temps | remarque |
|---|---:|---|
| IA diplomatique → `Diplomacy_Manager` → `Player_Diplomacy` `1802a7100` | 2,7 s | une seule action, pendant le tour de la cité-État 6 |
| ↳ cascade `Player_Diplomacy` ⇄ `Player_Influence` ⇄ `Player_EventManager` | | réentrante, sur au moins 4 niveaux |
| ↳ chaque niveau → `Cache_District_EventStream` → `Cache_City_Gold` → `Cache_City_*` → `City_Instance` → `District_Instance` → `Improvement_Yields` | | recalcul complet des rendements des villes |
| `GlobalAi` → `CityBuild` → `Value_Manager` → `Improvement_Yields` | 1,0 s | évaluation des constructions par l'IA |
| `Player_Units` (combat, files de production, gouverneurs, routes commerciales) | 0,8 s | |
| `Game::Processor::BeginTurn` | 0,5 s | |

**Fonction la plus coûteuse.** `180260720` (`Improvement_Yields.cpp`) consomme **1,9 s de temps
propre**, soit 28 % du thread. Elle est atteinte surtout par la cascade diplomatique.

**Piste principale.** Chaque événement diplomatique déclenche, à chaque niveau de la cascade, une
remise à jour immédiate des caches `Cache_*` des villes. Ces caches portent l'état que lit
l'interface, et leur remise à jour recalcule les rendements de toutes les villes concernées. Si ces
remises à jour étaient différées et regroupées, une seule par ville à la fin, la majeure partie des
2,7 s disparaîtrait. Il faudra le confirmer par décompilation : que fait `1802a7100`, et est-ce que
les caches `Cache_*` comptent dans l'état synchronisé ou servent seulement à l'affichage ?

## Après le regroupement : tour 78 → 79 en mode coalesce (capture du 2026-10-27, 16:21)

Même sauvegarde, proxy en mode `coalesce`. Le passage dure 6 s au lieu de 9 s. `analyze.py`
reconnaît maintenant l'original chargé sous le nom `_orig`.

**Le thread GameCore** passe de 6,2 s à 4,0 s dans l'arbre du passage de tour :

| chaîne | avant | après |
|---|---:|---:|
| cascade diplomatique (`Diplomatic` → … → `1802a1f90`) | 2,97 s | 0,77 s |
| `GlobalAi` → `CityBuild` → `Value_Manager` → `Improvement_Yields` | 1,0 s | 0,9 s |
| `Player_Units` (combat, files de production, gouverneurs, routes commerciales) | 0,8 s | 0,9 s |
| `Game::Processor::BeginTurn` | 0,5 s | 0,7 s |

**Prochaine cible : `180260720` (`Improvement_Yields.cpp`).** Elle consomme encore **1,1 s de temps
propre**, soit plus d'un quart du thread. On l'atteint par deux chemins : l'évaluation des
constructions par l'IA (`CityBuild`, 0,37 s) et le calcul des rendements de quartier
(`District_Instance` → `18025f570`, 0,83 s).

Décompilée, elle répond à la question « ce joueur peut-il construire cette amélioration ? » :
1. elle vérifie les prérequis de technologie et de dogme de l'amélioration ;
2. elle parcourt une liste de la base de règles (`base + 0xe40`, des entrées
   (nom d'amélioration en `+0x20`, trait en `+0x28`)) et compare **par `strcmp`** le nom de
   l'amélioration à celui de chaque entrée. Les 62 noms commencent tous par `IMPROVEMENT_`, donc
   chaque comparaison parcourt au moins 12 caractères ;
3. pour chaque entrée trouvée, elle cherche le trait dans les traits de la civilisation et du
   dirigeant du joueur, par deux recherches linéaires. `1802209a0`, qui retrouve la civilisation et le
   dirigeant, pèse encore 0,2 s de temps propre.

L'étape 2 ne dépend que de l'amélioration, et la base de règles ne change pas pendant la partie. On
peut donc la précalculer une fois : une table de l'amélioration vers ses traits remplace les
`strcmp`. La fonction reste une lecture pure. Remplacée par un détour qui rend le même booléen,
elle ne change pas l'état de jeu.

**Ce qui reste hors GameCore.** Un thread de l'exe tourne à 72 % (5 s), et une dizaine de threads de
travail à environ 35 % chacun. La sauvegarde automatique bloque toujours le thread principal environ
500 ms par tour.

## Les attentes du thread GameCore (même capture, changements de contexte)

`etl.py` décode maintenant les changements de contexte (CSwitch) et les réveils (ReadyThread) des
threads du jeu, avec leurs piles. Une pile arrive en deux événements : partie noyau, puis partie
utilisateur. `waits.py` les exploite.

**Ce qu'on voit.** Pendant le passage de tour (16:21:34,4 → 16:21:41,25), le thread GameCore
travaille 4,8 s et passe **2,06 s hors CPU** :
- **1,99 s en attente volontaire** (`Waiting / UserRequest`), en 61 fois, dont 26 de plus de 20 ms et
  une de 375 ms ;
- **il attend dans sa propre boucle** (`exe+0x999dbe`, appelé directement par le point d'entrée du
  thread) : il n'a plus de travail ;
- **c'est presque toujours le thread principal 8724 qui le réveille** (1,96 s sur 2,06), depuis sa
  boucle principale (`exe+0x6ff769` → `0x76c3a` → `0x77d65` → `0x999ce6`), pour lui confier le
  message suivant (`Messages_GameUpdate`).

Le passage de tour fonctionne donc **à l'alternance** : GameCore traite un message, puis attend que
le thread principal ait fini son tour de boucle (traitement des événements de jeu, interface Lua,
rendu) avant de recevoir le suivant.

**Ce que fait le thread principal pendant ces attentes** (1,94 s, occupé à 76 %), par branche de sa
boucle principale :

| branche (sous `exe+0x6ff769`) | temps | contenu |
|---|---:|---|
| `0x76c3a` → `0x7795f` (`0x80880`, `0x8073e`, `0x8066e`…) | ~620 ms | code de l'exe, plus GameCore (93 ms) et Localization : probablement le traitement des événements de jeu pour la vue |
| `0x76bf0` → `0x6e917` | ~300 ms | Lua (`havokscript`) et GameCore : scripts d'interface |
| `0x76c3a` → `0x77c5b` → `0x94e62f` | ~150 ms | rendu (pilote graphique) |
| `0x77cbc`, `0x77be7` | ~150 ms | appels système, attentes |
| Steam (`steam_api64`) | ~70 ms (+ 139 ms en feuille) | rappels Steam |

**Conséquences pour la suite :**
- Sur ce passage de tour, environ 30 % du temps réel ne se joue pas dans GameCore, mais dans le thread
  principal de l'exe. Accélérer GameCore ne réduit pas ces attentes.
- Pour les réduire, il faudrait alléger le travail du thread principal entre deux messages :
  - **réglages du jeu** : mouvements et combats rapides, moins d'animations ;
  - **mods d'interface** et leurs écouteurs Lua ;
  - ou **détourner des fonctions de l'exe**, ce que le proxy peut faire aussi : il est dans le même
    processus. Mais l'exe n'a pas de symboles, et il faudrait d'abord nommer `0x7795f` et ses
    branches.

## Les trois fronts remplacés : tour 78 → 79 (capture du 2026-10-27, 22:33)

Même sauvegarde, proxy en `mode=coalesce`, `portee=message`, `ameliorations=quiet`,
`rendements=fast`. La sauvegarde du tour 80 ne s'écarte de la référence `runs/mesure` que du bruit
habituel (38 587 octets en 22 zones, contre 38 641 entre deux parties non modifiées).

| capture | passage (s) | fenêtre (s) | CPU du processus (s) | thread GameCore, CPU (ms) | arbre `18004afc0` (ms) |
|---|---:|---:|---:|---:|---:|
| `tour79` (jeu non modifié) | 9 | 10 | 54,0 | 6 904 | 6 102 |
| `tour79-coalesce` | 6 | 7 | 39,6 | 4 478 | 4 027 |
| `tour79-quiet` (+ améliorations) | 5 | 6 | 33,9 | 3 217 | 3 121 |
| `tour79-remplacement` (+ diplo, portée message, rendements) | 4 | 5 | 27,8 | 2 424 | 2 174 |

Par chaîne, entre `tour79-quiet` et `tour79-remplacement` :

| chaîne | quiet | remplacement |
|---|---:|---:|
| cascade diplomatique (`18042dc40`) | 680 ms | 247 ms |
| attente des tâches de l'IA (`1809922c0`, sous le détour de `1804a16e0`) | 595 ms | 478 ms |
| dont `CityBuild` → `Value_Manager` | 278 ms | 156 ms |
| `Player_Units` (`180344a30`) | 553 ms | 321 ms |

**Le goulot est maintenant le thread principal de l'exe.** Sur la fenêtre de 5 s, le thread GameCore
passe 2,29 s hors CPU, dont 2,23 s dans sa boucle d'attente (`exe+0x999dbe`), réveillé par le thread
principal (`exe+0x6ff769` → `0x76c3a` → `0x77d65` → `0x999ce6`). La plus longue attente dure 473 ms.
GameCore ne calcule plus que la moitié du passage ; le reste attend le traitement des événements,
l'interface Lua et le rendu côté exe.

## Banc de comparaison : tours 78 → 80, proxy retiré / proxy complet (2026-10-27, 22:55 → 23:06)

Quatre captures WPR (`captures/*-bench-*`), en alternance : sans proxy (`-Uninstall`), puis proxy en
`mode=coalesce`, `portee=message`, `ameliorations=quiet`, `rendements=quiet`, deux fois chacun (sauvegardes
du tour 80 dans `runs/bench-*`). Chaque
capture couvre les deux passages, de `AutoSave_0078` jusqu'à la main au tour 80. `analyze.py` mesure
maintenant chaque passage à la milliseconde, d'après l'activité du thread GameCore (section « Passages
mesurés dans la trace »).

| passage | sans proxy (1 / 2) | avec proxy (1 / 2) | moyenne sans → avec |
|---|---:|---:|---:|
| 78 → 79 | 9 483 / 9 327 ms | 4 245 / 4 586 ms | 9 405 → 4 416 ms (−53 %) |
| 79 → 80 | 5 923 / 6 124 ms | 4 977 / 4 154 ms | 6 024 → 4 566 ms (−24 %) |
| **total** | 15 406 / 15 450 ms | 9 222 / 8 740 ms | **15 428 → 8 981 ms (−42 %)** |
| CPU du thread GameCore | 10 547 / 10 606 ms | 4 275 / 4 193 ms | −60 % |
| CPU du processus | 81,4 / 82,1 s | 48,5 / 44,9 s | −43 % |

Dans la passe 1 avec proxy, le tour 79 a été fini dans la seconde où la main est revenue : la
répartition entre les deux passages y est approximative (à une demi-seconde près), pas le total. La
sauvegarde automatique reste à environ 492 ms par tour dans les deux configurations.

**État de jeu.** Deux passes de même configuration diffèrent de 2 763 octets (sans proxy) et
2 747 (avec). Entre configurations, l'écart est d'environ 38 700 octets, dont 35 971 suivent
l'alternance. Ce n'est pas le proxy : une zone de 56 Kio (avec l'ordre de deux valeurs répété six
fois) bascule d'une passe à l'autre, dans l'ordre chronologique, quelle que soit la configuration. Sur
les passes de l'après-midi, le test par groupes le montre : 35 973 octets suivent la parité des passes
(`count`, `coalesce2` / `coalesce`, `count2`) et 0 le mode (`count`, `count2` / `coalesce`,
`coalesce2`). Cause probable, non vérifiée : `AutoSave_0078` est une sauvegarde de fin du tour 77 que
le jeu réécrit à chaque chargement, si bien que chaque passe part de celle réécrite par la précédente.
Un redémarrage du jeu une fois sur deux ne l'explique pas ici : les quatre passes du banc ont toutes
redémarré le jeu. Copie fixe de départ pour les prochains bancs : `runs/depart/AutoSave_0078.Civ6Save`
(celle écrite par la dernière passe avec proxy), à remettre dans `Saves\Multi\auto` avant chaque passe.
