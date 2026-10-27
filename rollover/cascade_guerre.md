# Cascade d'une déclaration de guerre

Décompilation (Ghidra via REA) de la chaîne qui a pris 2,7 s pendant le tour de la cité-État 6, au
passage 78 → 79 (voir [README.md](README.md)). Les adresses sont celles de
`GameCore_XP2_FinalRelease.dll` (base 0x180000000). Les noms sont déduits du code : la DLL n'a pas
de symboles.

## Les fonctions

| adresse | rôle déduit | `.cpp` |
|---|---|---|
| `1802a1f90` | `DeclareWar(déclarant, cible, raison, forcer…)` : si les deux joueurs sont dans une équipe, la guerre est déclarée pour chaque paire (membre de l'équipe du déclarant, membre de l'équipe adverse) ; sinon un seul appel à `1802a7100` | Player_Diplomacy |
| `1802a7100` | `DoDeclareWar(déclarant, cible)` : met à jour l'état des deux joueurs, puis émet le signal « guerre déclarée », envoie les notifications et rapporte l'événement à l'interface | Player_Diplomacy |
| `1802a2390` | émission d'un signal : appelle tour à tour chaque abonné (`std::function`) de la liste globale `jeu+0x280+0xcc0` | Player_Diplomacy |
| `1802d8040` | abonné installé une fois par joueur : appelle `1802acac0` puis `1802ec560` pour ce joueur | Player_EventManager |
| `1802acac0` | réaction diplomatique : compteurs, griefs (`LOC_DIPLO_GRIEVANCE_LOG_*WAR_DECLARED*`) ; **si le joueur a une alliance avec la cible, il déclare à son tour la guerre au déclarant** (raison `0xfb189687`) | Player_Diplomacy |
| `1802ec560` | réaction d'une cité-État : **si le déclarant ou la cible est son suzerain, elle déclare la guerre à l'autre camp** (raison `0x5064ce64`) | Player_Influence |
| `1805fc5a0` | rapport d'événement : construit une table de valeurs, appelle les gestionnaires de cache (`18007c670`), puis transmet l'événement au moteur (file pour Lua et l'interface) | ? Reporting |
| `18007c670` | répartition vers les gestionnaires de cache abonnés à ce type d'événement, de façon synchrone | ? Cache_District_EventStream |
| `180072f80` | gestionnaire « villes d'un joueur » : **pour chaque ville du joueur, refait un instantané complet** et le poste à l'interface (`0xe431107f`) | ? Cache_City_Gold |
| `180072120` | instantané complet d'une ville : données de base, rendements, croissance, culture, bâtiments, quartiers, IA (`Cache_City_*`) | Cache_City_EventStream |
| `180075360` → `180132b60`, `180132460` → `District_Instance` → `Improvement_Yields` `180260720` | calcul des rendements pendant l'instantané : rien n'est mis en cache, tout est recalculé aménagement par aménagement | City_Instance, Improvement_Yields |

## Comment ça s'emballe

```
DeclareWar(A, B)
└─ pour chaque paire (membre de l'équipe de A, membre de l'équipe de B) : DoDeclareWar(x, y)
   ├─ mise à jour de l'état diplomatique de x et y
   ├─ signal « guerre déclarée » → pour CHAQUE joueur j (≈ 22) :
   │   ├─ 1802acac0 : si j a une alliance avec y → DeclareWar(j, x)    ← récursion
   │   └─ 1802ec560 : si j est une cité-État dont x ou y est le suzerain
   │                  → DeclareWar(j, l'autre)                          ← récursion
   └─ rapport « DiplomacyDeclareWar » → caches de l'interface
       └─ pour chaque joueur concerné, pour chaque ville : instantané complet
          └─ recalcul de tous les rendements (Improvement_Yields)
```

Chaque guerre en déclenche d'autres : les alliés, puis les cités-États dont un belligérant est
suzerain. Chacune émet à nouveau le signal et un rapport. Les villes des belligérants sont donc
remises en instantané une fois par déclaration de la cascade. Or chaque nouvelle guerre change aussi
les rendements de ces villes : les bonus d'émissaires ne s'appliquent qu'en paix avec la cité-État.
Le recalcul est alors à la fois répété et réellement nécessaire, mais seul le dernier instantané de
chaque ville sert.

Dans la trace, l'appel externe de `1802a7100` passe 2,74 s dans la cascade. Le rapport à l'interface
(`1805fc5a0`) en coûte environ 0,9 s à son propre niveau, et à peu près autant à chaque niveau
inférieur. Les instantanés de villes (`180072f80` → `180072120`) en sont l'essentiel.

## Pistes d'optimisation

1. **Regrouper les instantanés de villes.** Pendant une cascade, `180072f80` noterait seulement les
   villes à rafraîchir. Un seul instantané par ville serait fait à la fin, au retour de l'appel
   `DeclareWar` le plus externe, ou à la fin de la passe de `Game::Processor::Update`.
   - **Gain attendu :** l'essentiel des 2,7 s. Il reste un instantané par ville concernée et le
     travail diplomatique lui-même.
   - **Risque pour la synchronisation :** les instantanés semblent n'alimenter que l'interface (les
     caches lus par `GameViewState_UpdateCache` sur le thread principal). Il faut vérifier que
     `180072120` et les calculs de rendements qu'il appelle n'écrivent rien dans l'état de jeu (un
     cache paresseux, par exemple). Dans ce cas, différer ne change pas la simulation et ne peut pas
     désynchroniser les joueurs.
   - **Ordre des messages vers l'interface :** les instantanés partent dans une file moteur
     (`DAT_180b8aa80` vtable +0xb0) ; les poster plus tard, dans le même ordre relatif, devrait
     suffire.
2. **Ne pas répartir de rapport vers les caches d'un joueur sans villes** (cités libres,
   barbares) : gain marginal.
3. **Accélérer `180260720`** (`Improvement_Yields`, 1,9 s de temps propre sur tout le passage). Ce
   chantier est plus délicat : c'est du calcul de règles, donc toute réécriture doit donner un
   résultat identique au bit près. Il ne faut l'envisager qu'après la piste 1, selon ce qu'il reste.

## Vérification : l'instantané ne modifie pas l'état de jeu

La question : est-ce que `180072120`, et ce qu'il appelle, écrit ailleurs que dans l'instantané qu'il
prépare ? Pour chaque fonction décompilée, on a relevé toutes les écritures (affectations à travers
un pointeur, globals).

| fonction | rôle | écrit dans |
|---|---|---|
| `180072120` | assemble l'instantané | l'enregistrement de cache qu'il vient d'allouer |
| `180072360` | alloue un enregistrement dans la réserve de cache | la réserve `DAT_180b8b3f0`, propre au cache |
| `180075360` | données de base de la ville | l'instantané |
| `180132b60`, `180132460`, `180132850` | rendements par type, avec leur détail | des vecteurs de l'instantané |
| `18012ff20`, `1801313a0` | détail d'un rendement, infobulles `LOC_CITY_YIELD_FROM_*` comprises | la structure de résultat, des variables locales |
| `1801346b0` | indicateur de la ville | rien |
| `180191c00`, `1801905f0` | rendement d'un quartier | rien (valeur de retour) |
| `180260510`, `18025f570`, `18025f890` | bonus de proximité | des variables locales ; la liste de sortie de `18025f890` est nulle sur ce chemin |
| `180260720` | condition d'un bonus de proximité | le global `DAT_180b8aa60`, mais avec la valeur qu'elle vient d'y lire : un artefact du compilateur, sans effet |
| `180074010`, `180071130`, `1800791c0`, `1800712d0`, `18007a5e0`, `1800708f0`, `1800787e0` | croissance, culture, religion, bâtiments, quartiers, loyauté | l'instantané |
| `180068f90`, `1803e4d90`, `1803e5a40` | section IA : meilleures constructions et cases à acheter | les tableaux de sortie de l'instantané |

**Qui lit la réserve de cache ?** Ses deux globals, `DAT_180b8b438` (compteur d'identifiants) et
`DAT_180b8b410` (table des enregistrements), ne sont référencés que dans `Cache_City_EventStream`
(`180072xxx`) et par une fonction isolée en `1809b1b26`, probablement l'initialisation ou la
destruction statique. Aucun code de règles n'y lit.

**Conclusion.** Sur tout le chemin chaud de la trace, l'instantané ne fait que lire l'état de jeu.
Regrouper les instantanés ne change donc pas la simulation. Deux conséquences de bord, sans effet sur
la simulation :
- l'interface reçoit moins de messages « ville modifiée » (`0xe431107f`) ;
- les identifiants de la réserve de cache sont attribués dans un autre ordre.

**Limites de la vérification.**
- Les getters les plus profonds (les dizaines de fonctions appelées par `18012ff20`, par exemple
  `180130f70` ou `180133780`) n'ont pas tous été décompilés. Seuls ceux du chemin chaud l'ont été.
- Deux appels indirects n'ont pas été suivis : celui via `_guard_check_icall` sur `ville+0x8b8`
  dans `180072120`, et la méthode +0x178 de l'instantané lui-même.
- Le test décisif reste expérimental : rejouer le même passage de tour avec et sans regroupement, puis
  comparer les sauvegardes produites, qui doivent être identiques à l'octet près.

## Chiffrage : appels réels pendant la cascade

**Sources.**
- **Le journal du jeu `DiplomacySummary.csv`** (dossier `Logs`, session de la capture). La ligne
  « At War » est écrite par `1801d31b0`, que `1802a7100` appelle une fois par déclaration, juste après
  sa garde. C'est l'unique référence à la chaîne `At War` dans la DLL. `1801d31b0` a trois autres
  appelants (`180227a30`, `1802a7710`, `1802abba0`), mais toutes les lignes du tour 79 donnent une
  raison de déclaration de guerre (« Surprise », « Defensive Pact », « Suzerain War »).
- Les lignes « Individual Declaring War on Team START/END » viennent de `1802a1f90`.

**La partie.** Ce n'est pas une partie avec seulement des cités-États : il y a 8 civilisations
majeures, 12 cités-États et les cités libres. L'équipe 0 compte six joueurs (0 à 5).

**Une seule déclaration au départ.** Le joueur 6 déclare une guerre surprise à l'équipe 0. Toute la
cascade tient entre son START et son END.

| mesure | valeur |
|---|---:|
| appels de `DeclareWar` (`1802a1f90`, lignes START) | 358 |
| appels de `DoDeclareWar` (`1802a7100`, lignes « At War ») | **586** |
| dont « Suzerain War » / « Surprise » / « Defensive Pact » | 562 / 12 / 12 |
| paires orientées distinctes | 90 |
| paires de joueurs distinctes (non orientées) | 60 |
| joueurs impliqués | 17 (8 majeures, 9 cités-États) |
| villes de ces joueurs | 61 |

Chaque paire orientée est donc déclarée **6,5 fois en moyenne**, jusqu'à 12 fois pour certaines
(`9→19`, `19→1`…). La boucle d'équipe de `1802a1f90` appelle `1802a7100` en forçant la déclaration
(`param_4 = 1`), sans vérifier si les deux joueurs sont déjà en guerre.

**Temps sur le thread GameCore**, à partir de la trace : part des échantillons de la cascade qui
contiennent chaque fonction dans leur pile.

| fonction | temps | part |
|---|---:|---:|
| `1802a7100` (cascade entière) | 2 742 ms | 100 % |
| `18007c670` (répartition vers les caches) | 2 549 ms | 93 % |
| `180072f80` / `180072120` (instantanés de villes) | 2 169 ms | 79 % |
| `1801d31b0` (écriture du journal diplomatique) | 41 ms | 1 % |

**Nombre d'instantanés.** Si chaque déclaration refait les instantanés des deux joueurs, la cascade
en produit environ **2 900** (somme des villes des deux joueurs sur les 586 déclarations). Un seul
par ville en exige **61**, soit environ 48 fois moins. Le coût moyen est d'environ 0,75 ms par
instantané.

**Gain attendu de la piste 1** (regrouper les instantanés) :
- 2 169 ms × (1 − 61/2 900) ≈ **2,1 s** sur les 2,74 s de la cascade, soit environ un quart des 9 s
  du passage de tour.
- L'estimation tient même si un seul joueur est rafraîchi par déclaration (≈ 1 450 instantanés) :
  le temps mesuré reste 2 169 ms, et le gain vaut alors 2,08 s.
- Les ≈ 380 ms des autres gestionnaires de cache (`18007c670` hors instantanés de villes) sont
  probablement redondants de la même façon. Ils ne sont pas comptés ici.

**Autre piste : les déclarations redondantes** (586 appels pour 90 paires orientées).
- Ne pas redéclarer une guerre déjà en cours couperait aussi le travail diplomatique et les
  notifications.
- Mais ce n'est plus une optimisation neutre : certains de ces appels modifient l'état de jeu.
  Les lignes de journal, elles, ne comptent pas : elles ne font pas partie de l'état. Le détail est
  dans la section suivante.
- **Aucun paquet réseau n'accompagne la cascade.** Le journal réseau de la capture ne contient aucun
  message entre l'activation du joueur 6 (12:51:48) et celle du joueur 7 (12:51:52), alors que le
  tour du joueur 6 contient toute la cascade et sa déclaration de guerre. Chaque client calcule donc
  lui-même la décision de l'IA et toute la cascade, et seuls les ordres des joueurs et les passes
  `GameUpdate` circulent.
  - Réserve : la partie capturée n'avait qu'un client connecté.
- **Conséquence.** Un client qui sauterait les appels redondants arriverait à un état différent des
  autres, sans qu'aucun message ne le corrige : c'est une désynchronisation. Cette piste exige donc
  que tous les joueurs aient la même DLL, et elle donne des parties différentes du jeu de base.
- **La piste 1, elle, n'a pas cette contrainte.** Les instantanés ne modifient pas l'état de jeu.
  Chaque joueur peut l'activer ou non de son côté.

## Ce que modifie la réaction d'un joueur à une déclaration (`1802acac0`)

**Hachages.** Les constantes de la DLL sont des hachages de noms de la base de règles : CRC32 avec le
XOR final inversé, soit `~crc32(nom)`. On les retrouve dans la table `Types` de
`Cache/DebugGameplay.sqlite`.

| constante | nom |
|---|---|
| `0x253718b0` | `CIVILIZATION_LEVEL_FULL_CIV` : `1802f9860` teste « civilisation majeure » |
| `0xc2577075` | `CIVILIZATION_LEVEL_CITY_STATE` : comparée au niveau du joueur, lu par `1802f7f20` |
| `0x4396d725` | `SURPRISE_WAR` |
| `0x114b1c94` | `PROTECTORATE_WAR` |
| `0xfff75c0e` | `LIBERATION_WAR` |
| `0x4599ae3f` | `TERRITORIAL_WAR` |
| `0x5064ce64` | `SUZERAIN_WAR` : raison des déclarations de `1802ec560` |
| `0xfb189687` | `DEFENSIVE_PACT` : raison des déclarations de `1802acac0` |
| `0x01f4de45` | `EMERGENCY_WAR` |
| `0x2e66c50c`, `0xb3eff359`, `0x83c34d2a` | `NOTIFICATION_DECLARE_WAR`, `…_CITY_STATE`, `NOTIFICATION_GRIEVANCES_AGAINST_YOU` |

**Champs suivis.** Les écritures passent par `18064a6d0` (affectation) et `18072a920` (accès en
écriture). Les deux inscrivent le champ dans l'ensemble des champs modifiés de son propriétaire :
ce sont des champs suivis, qui font partie de l'état du joueur.

Pour un joueur `j` qui réagit à « `d` déclare la guerre à `c` pour la raison `r` » :

| bloc | condition | effet |
|---|---|---|
| A | `j == d`, `d` et `c` majeurs | `+0x1fe8` ← tour courant ; compteur `+0x20b8` += 1 ; si `SURPRISE_WAR` : compteur `+0x20d0` += 1 et `+0x2030` ← tour ; si `LIBERATION_WAR`, `PROTECTORATE_WAR` ou `TERRITORIAL_WAR` : `+0x2048`, `+0x2060` ou `+0x2078` ← tour |
| B | `j == c`, `d` et `c` majeurs | `+0x2000` ← tour courant |
| — | `j == d`, ou `r` vaut `DEFENSIVE_PACT` ou `EMERGENCY_WAR` | arrêt |
| C | `d` majeur, `j` allié de `c`, `j` pas en guerre contre `d` | `j` déclare la guerre à `d` (`DEFENSIVE_PACT`) |
| D | `j` a rencontré `d`, n'est pas en guerre contre lui ; `r = PROTECTORATE_WAR` | `PlayerStats.m_aCityStateWarCounts[d]` += 1 |
| D' | même condition ; autre raison, et `c` est une cité-État | `PlayerStats.m_aCityStateWarCounts[d]` −= 1 |
| E | `j` a rencontré `d`, `d` majeur | griefs de `j` envers `d` (`18029e9d0`), notification |
| F | `j == c` | `1802aa300` / `18029fb80` avec `0x9c55a0d8` (non identifié) |

Le composant du joueur placé à `+0x710` est `PlayerStats`. Son constructeur, `180326250`, enregistre
ses champs par nom, et `+0xa28` est `m_aCityStateWarCounts`. Aucun nom Lua ne l'expose, et ses
lecteurs n'ont pas été recherchés.

**Appliqué aux 586 appels du tour 79.** Les déclarations redondantes sont toutes des
`SUZERAIN_WAR`, déclarées par une cité-État.
- **A, B, C et E ne s'exécutent pas pour elles**, puisque ces blocs exigent un déclarant majeur. Les
  compteurs de guerres déclarées et les griefs ne changent donc qu'une fois par vraie déclaration
  d'une majeure.
- **Seul D' est répété** quand la cible est aussi une cité-État (par exemple `19→9`, `12→9`, `9→19`).
  Chaque joueur qui a rencontré la cité-État déclarante sans être en guerre contre elle décrémente
  `m_aCityStateWarCounts[déclarante]`, à chaque appel redondant.
- **Non examiné ici :** le corps de `1802a7100` pour un appel redondant. On y voit des remises à zéro
  de champs suivis (`+0x3c8`, `+0x418`), `1802ad6d0`, le retrait de gouverneurs et
  `1802e8ec0` (influence). Ces opérations semblent idempotentes, mais ce n'est pas vérifié.

## Qui lit `PlayerStats.m_aCityStateWarCounts`

**Où est la donnée.** Le tableau est au champ suivi `PlayerStats+0xa28`, et ses données sont à
`+0xa28 + 0x10 + 0x110 = +0xb48`. Le joueur `j` y compte, pour chaque autre joueur `d`, les guerres
de `d` qui protègent une cité-État (+1) ou qui en attaquent une (−1).

**Dans le composant `PlayerStats` lui-même**, on ne trouve que :
- la remise à zéro (`18032dd10`, appelée par `1802f8450` dans `Player_Instance.cpp`) ;
- la sérialisation (`18032a030`, qui écrit les 64 entrées avec les autres tableaux du composant).

Le compteur fait donc partie des sauvegardes.

**Seul lecteur extérieur trouvé : `1808ea5f0`.** C'est une condition de `GameEffects`. Elle prend un
joueur propriétaire et un joueur sujet, lit `m_aCityStateWarCounts[sujet]` chez le propriétaire, puis
la compare à quatre arguments : borne basse (`+0x90`), borne haute (`+0x94`), sans borne basse
(`+0x98`) et sans borne haute (`+0x99`). Elle n'a pas d'appelant direct, seulement une entrée de
table virtuelle en `0x180aa13f8`.
- **Le type de condition.** Ces quatre arguments sont exactement ceux de
  `REQUIREMENT_PLAYER_CITY_STATE_PROTECTOR` dans la base de règles (`LowerBound`, `UpperBound`,
  `NoLowerBound`, `NoUpperBound`). Le lien entre la table virtuelle et la chaîne (référencée en
  `1807e4d30` et `1807e75bf`) n'a pas été suivi jusqu'au bout.
- **Qui s'en sert.** Dans la base, avec les mods de la partie, cette condition ne sert qu'à quatre
  ensembles de conditions. Ceux-ci n'alimentent que les programmes d'IA
  `AGENDA_[SAGA_]{HIGH,MED}_{PROTECT,ATTACK}_CITY_STATES`, tous de type
  `MODIFIER_PLAYER_DIPLOMACY_SIMPLE_MODIFIER`, c'est-à-dire l'opinion d'une IA majeure envers un
  joueur. Le mod `LIME_TEO_OWL` réutilise deux de ces conditions dans un ensemble qui ne porte aucun
  modificateur.

**Conclusion.** Le compteur ne sert qu'à l'opinion des IA majeures qui ont l'un de ces programmes.
Dans une partie entre humains, avec seulement des cités-États comme IA, aucun code de règles ne le lit.
Les décréments répétés de la cascade n'ont alors aucun effet sur la partie. Ils restent visibles
dans la sauvegarde, et donc dans un test « sauvegardes identiques à l'octet près ».

**Limites.**
- La recherche des lecteurs est heuristique : elle cherche les fonctions qui contiennent à la fois
  un accès `[reg+0x710]` et un accès `[reg+0xb48]`. Un lecteur qui recevrait le composant
  `PlayerStats` en paramètre, sans passer par `+0x710`, lui échapperait.
- La correspondance entre `1808ea5f0` et `REQUIREMENT_PLAYER_CITY_STATE_PROTECTOR` repose sur la
  forme des arguments, pas sur la table virtuelle.

## À vérifier ensuite

- Confirmer le nombre d'appels à `180072f80` par déclaration (un ou deux joueurs), avec un compteur
  dans une DLL proxy.
- Lister tous les gestionnaires inscrits dans la table de `18007c670`, pour les autres types
  d'événement, afin de repérer d'autres instantanés répétés de la même façon.
- Piste 3 : `180260720` parcourt toute une table de règles (`FUN_1807d0a80`) et compare des chaînes
  à chaque ligne, pour chaque aménagement et chaque règle de proximité. Une table d'index préparée
  une fois ferait le même travail sans parcours. C'est un calcul booléen, sans virgule flottante :
  le résultat reste identique.
