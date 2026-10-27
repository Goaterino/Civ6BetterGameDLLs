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

## À vérifier ensuite

- Compter les appels réels à `1802a7100` et `180072f80` pendant la cascade, avec un hook Frida ou
  un compteur dans une DLL proxy, pour chiffrer le gain de la piste 1.
- Lister tous les gestionnaires inscrits dans la table de `18007c670`, pour les autres types
  d'événement, afin de repérer d'autres instantanés répétés de la même façon.
- Piste 3 : `180260720` parcourt toute une table de règles (`FUN_1807d0a80`) et compare des chaînes
  à chaque ligne, pour chaque aménagement et chaque règle de proximité. Une table d'index préparée
  une fois ferait le même travail sans parcours. C'est un calcul booléen, sans virgule flottante :
  le résultat reste identique.
