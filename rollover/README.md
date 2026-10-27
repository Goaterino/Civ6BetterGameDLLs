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
    viennent de [la carte des sources](../../docs/gamecore_xp2/README.md).
  - Fenêtre : par défaut, le dernier passage de tour des journaux ; `--tout` pour la capture entière.

```bash
.venv/Scripts/python.exe -I perf/rollover/analyze.py perf/rollover/captures/<dossier>
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
