# Blocage pendant l'animation d'un WMD

Pendant l'animation d'un lancer de bombe nucléaire, les joueurs en vue 3D ne peuvent plus agir
jusqu'à la fin de l'animation, qui est longue. Les joueurs en vue stratégique ne sont pas bloqués.
Il s'agit de trouver le code qui fait attendre.

## Ce que montre l'analyse statique (2026-10-29)

Adresses des projets Ghidra `civ6exe` (`CivilizationVI_DX12.exe`, base 0x140000000) et `civ6xp2`
(`GameCore_XP2_FinalRelease.dll`, base 0x180000000).

- **Lua.** Il ne fait qu'envoyer l'ordre : `UnitPanel.lua` (`OnUnitActionClicked_WMDStrike`) puis
  `WorldInput.lua` (mode `WMD_STRIKE`).
- **GameCore.** Une frappe est traitée comme un combat qui porte un type de WMD :
  - `Unit_Operations_WMDStrike.cpp` : `18067e260` vérifie la portée et la cible ;
  - `Combat_Manager.cpp` : `180158af0` (ICBM) et `180156ae0` calculent les résultats. Les deux
    citent `Trying to generate ICBM combat results without a valid WMD`.
- **Exe.** L'animation passe par le visualiseur de combat ordinaire.
  - La cible est une fausse unité, `UNIT_WMD_TARGET_PLOT`, dont `1404e1130` charge l'artdef.
  - La durée vient des artdefs :
    - `Units.artdef` : `Bomber_ABOMB_Hit`, `Bomber_HBOMB_Hit`, `MissileSilo_*BOMB_Attack`,
      `NuclearSubmarine_*BOMB_Hit` ;
    - `VFX.artdef` : `AtomBomb_Hit` et `HydrogenBomb_Hit`, soit l'effet
      `FX_Hydrogen_Bomb_Explosion`.
  - En vue stratégique, l'exe ne joue que les sons `Nuke_Attack_2D` (`14009f250`).
- **Événements.**
  - `SerialEvent::CombatVisBegin` est créé par `1400ed220`, et `CombatVisEnd` par `1400ed2e0`.
  - Leurs fonctions de diffusion sont `1400f0400` et `1400f0460`.
  - Le C++ les appelle par un pointeur indexé, et Ghidra ne voit pas d'où. La trace qui suit sert
    à le trouver.

## Première capture : 4 nukes sur un tour (2026-10-29)

Journal `runs/20261029-224450/frida.log`, trace `perf/rollover/captures/20261029-224517-nuke`.
L'analyse `attentes.py` est dans `runs/20261029-224450/attentes.txt`.

- **Les tirs.** Les 4 nukes partent de villes (silo) : la pile GameCore passe par `180682fea`,
  dans `City_Commands_WMDStrike.cpp`. Chaque visualisation dure 11,0 à 11,7 s.
- **La visualisation.** `CombatVisBegin` et `CombatVisEnd` sont diffusés par le thread principal,
  depuis la même vidange de file (`exe+0x60308` < `0x6e453` < `0x6e9a8` < `0x76bf0`), sous la
  boucle principale `exe+0x6ff769`. Le membre est de type 3 : la ville qui tire.
- **Le thread GameCore ne travaille pas, il attend.**
  - Il a 0 à 173 ms de CPU sur chaque fenêtre de 11 s.
  - Pour le 1er nuke, il dort d'un seul tenant pendant 15,6 s (22:45:31.39 → 46.98), jusqu'à 3,9 s
    après `CombatVisEnd`.
  - Les 40 attentes sont toutes au même endroit : `exe+0x999dbe`, dans `140999d10`, la boucle du
    thread « Game Core ». Elle exécute `1400f7640`, qui appelle `IGameCore+0x238`, puis attend
    son événement.
- **Qui le réveille.** C'est toujours le thread principal, dans `Civ6App_UpdateFrame`
  (`140077230`), bloc `Civ6App_GameCoreUpdate` : `1400f7400(gc, 1)` puis
  `GameCoreController::Update` (`1400f4b40`) → `140999c90` → `SetEvent`. Le bloc ne s'exécute que
  si `IGameCore+0x240` est vrai et `DAT_14230f390` est nul.
- **Côté DLL.**
  - `IGameCore+0x240` → `180036f30` : vrai si la file des messages `180b8a9e8` n'est pas vide.
  - `IGameCore+0x238` → `1800372f0` : vide cette file.
  - `IGameCore+0x230` → `1800373e0` : y ajoute un message.
  - La seconde file (`180b8aa00`) et les octets `180b8aa19`/`aa1a` servent à la commande de
    console `gamecore message record/playback`. Ce n'est pas une porte.

**Conclusion provisoire.** Pendant l'animation, GameCore n'est pas en retard : sa file est vide.
Ce qui retient les joueurs se trouve en amont, dans l'exe. Soit il ne poste pas les ordres pendant
la visualisation, soit l'interface les refuse avant de les envoyer : les scripts consultent
`UI.IsGameCoreBusy()` et `UI.IsProcessingMessages()`.

## Prochaine capture

`hook_nuke.js` note maintenant aussi `gc_post`, chaque message posté (type et pile), et
`gc_drain`, chaque vidange. Refaire un nuke en vue 3D et, pendant l'animation :
- donner un ordre (déplacer une unité) ;
- tenter de finir le tour ;
- noter l'heure de chaque action.
Si aucun `gc_post` ne suit l'ordre avant `exe_visEnd`, l'exe retient les messages. La pile du
premier `gc_post` après `exe_visEnd` montrera alors où il les retenait.

## Deuxième capture : 2 nukes (2026-10-29, 23:17)

Journal `runs/20261029-231612/frida.log`, recoupé avec `net_message_debug.log` du jeu.

- **Les messages postés à GameCore** (`gc_post`) ont tous le type `0x6`. Ils arrivent tous par la
  couche réseau, sous `Civ6App_UpdateNetwork` (`exe+0x294aeb` < `0x77cbc`). Chacun est vidé par le
  thread GameCore en moins de 200 ms.
- **Les ordres envoyés pendant cette fenêtre** :
  - 23:17:23 et 23:17:46 : `CityCommand eCommand=5`, les deux lancers ;
  - 23:17:48, pendant la 2e animation : `PlayerOperation ACCEPT_EMERGENCY`, posté puis traité en
    6 ms.
- **Ce qui manque** : aucun `UnitOperation` ni aucune fin de tour n'a été envoyé, ni pendant les
  animations ni après.
- **Conclusion.** Pendant l'animation, la chaîne réseau → GameCore accepte et traite les ordres.
  Un ordre de déplacement ou de fin de tour donné à ce moment n'a jamais été envoyé : l'interface
  l'a refusé avant.
- **Les deux tests Lua.**
  - `UI.IsGameCoreBusy` (`1401aa4d0`) lit l'état du thread GameCore (`1400f5790`), qui est
    inactif ici.
  - `UI.IsProcessingMessages` (`1401aa760`) renvoie `HasFocus` d'un objet global (`14230f760`),
    que la boucle de frame met à jour. C'est le candidat pour « la lecture des événements est en
    cours ». C'est ce test qui désactive la fin de tour dans `ActionPanel.lua`.

## Le vrai symptôme : un retard de l'affichage

D'après Tim, la partie elle-même n'est pas bloquée. Pendant les quelque 10 s de l'animation, le
joueur n'a simplement aucun retour visuel de ses actions. Ses animations se jouent toutes juste
après la fin de celle du nuke. C'est cohérent avec les captures : GameCore traite les ordres tout
de suite, et c'est le rendu qui attend.

Où le rendu peut attendre, dans `Civ6App_PublishEvents` (`14006e7e0`, appelé à chaque frame) :
- `Civ6App_SequencerEvents` : séquenceur d'animations, trois variantes (`14006e470`, `14006d9a0`,
  `140070410`) selon un drapeau en `141b0b140+0x78` ;
- `Civ6App_PublishGameCoreEvents` (`14006e5d0`) : publié seulement si le thread GameCore est libre
  et si l'octet `GameCoreController+0xc` est nul (`14003e4c0`, contrôleur en `14230d6f0`).

Deux hypothèses :
1. **Cet octet bloque la publication des événements GameCore** pendant une visualisation de combat.
   Les déplacements ne sont alors pas publiés du tout avant `CombatVisEnd`.
2. **Les événements sont publiés, mais le séquenceur les met en file** derrière la visualisation du
   nuke.

`hook_nuke.js` note maintenant :
- `exe_publish_reprise` : la publication reprend après une pause de plus de 250 ms ;
- `exe_etat` : chaque changement de `GameCoreController+0xc` et de l'occupation du thread
  GameCore, relevés toutes les 50 ms.

Sous l'hypothèse 1, on doit voir une pause de la publication d'environ 10 s, et l'octet à 1 pendant
l'animation.

### Troisième capture : 2 nukes avec déplacements (2026-10-29, 23:29)

Journal `runs/20261029-232828/frida.log`, recoupé avec `net_message_debug.log`.

- **L'hypothèse 1 est rejetée.** Pendant les deux animations (23:29:27.9 → 39.3 et
  23:29:49.5 → 30:01.5), `GameCoreController+0xc` reste à 0 et la publication ne s'arrête jamais
  plus de 250 ms. La seule longue pause, 14,35 s avec l'octet à 1, est le chargement de la
  sauvegarde.
- **Les déplacements passent.** Il y en a 9 pendant la 1re animation et 5 pendant la 2e. Chacun
  est envoyé, posté (`gc_post`) et traité : le thread GameCore est occupé environ 50 ms à chaque
  fois.
- **Hypothèse 2.** Les événements sont donc publiés tout de suite, et c'est en aval que leur rendu
  attend la fin de l'animation du nuke.
- **Le séquenceur.** Les trois fonctions de `Civ6App_SequencerEvents` sont des files à double
  tampon génériques, avec leurs variantes d'enregistrement et de relecture. Elles n'ordonnent rien.
  La mise en série se fait chez les consommateurs des événements (simulation visuelle des unités),
  qui reste à localiser.

## Mod « Nuke Quick Vis » : raccourcir l'animation

`build_mod.py` construit le mod dans `Documents/My Games/Sid Meier's Civilization VI/Mods/NukeQuickVis` :
- un `Units.artdef` partiel qui redéfinit les six attaques WMD de la collection `CombatAttack` :
  `MissileSilo_*BOMB_Attack`, `Bomber_*BOMB_Hit` et `NuclearSubmarine_*BOMB_Hit` ;
- un `.dep` minimal (consommateur `Units` seulement) et un `.modinfo` avec `AffectsSavedGames = 0`.

Leviers :
- `--vitesse` multiplie `ProjectileSpeed` (40 pour le silo et le sous-marin ; le bombardier
  largue sans projectile) ;
- `--impact` remplace l'effet `AtomBomb_Hit` / `HydrogenBomb_Hit` par un autre élément de
  `VFX.artdef`.

`artdef_params.py` affiche les paramètres d'un élément, pour vérifier le résultat.

Points à vérifier :
- qu'un élément du mod portant le même nom remplace bien celui du jeu, avec
  `m_ReplaceMergedCollectionElements = false` ;
- si chaque joueur doit installer le mod en multijoueur.

**Mesure.** Durée `exe_visBegin` → `exe_visEnd` dans le journal de `hook_nuke.py`. Sans le mod,
elle est de 11,0 à 11,8 s pour un tir de silo.

| essai | réglage | durée de la visualisation |
|---|---|---|
| référence | aucun mod | 11,0 – 11,8 s |
| 1 | vitesse ×10 | 6,1 – 6,9 s (3 tirs de silo) |
| 2 | vitesse ×10, gravité ×100 | 5,8 – 7,2 s (4 tirs) ; arc trop haut |
| 3 | vitesse ×10, gravité ×50, silo tirant avec `Special/TargetPlot` | 0,39 s : plus aucune animation |
| 4 | mod désactivé, `hook_nuke.py --accel 3` | 3,4 – 4,5 s (4 tirs) ; minuteur intact |
| 5 | mod désactivé, `hook_nuke.py --accel 10` | 1,17 – 1,22 s (3 tirs) ; animation encore lisible |
| 6 | mod désactivé, `hook_nuke.py --accel 15` | 0,81 – 0,84 s (5 tirs, hors tirs qui déclarent la guerre) |

**Comparaison avec une attaque de ville.** Visualisations à deux membres, dont une ville (type 3),
relevées dans toutes les runs :
- attaques de ville en mêlée ou à distance : 0,20 à 0,66 s, le plus souvent 0,45 à 0,51 s ;
- l'attaque de ville de la run `20261030-002134` (unité 7208972 sur la case 24,26) : 0,66 s ;
- la durée vanilla d'un nuke est d'environ 12,3 s (0,82 × 15). Il faudrait ×19 pour atteindre
  0,66 s, et ×24 à ×27 pour 0,45 – 0,51 s ;
- limite : la visualisation de combat ne couvre que l'attaque. Lors d'une prise de ville, l'entrée
  de l'unité dans la ville se joue ensuite, hors de cette fenêtre, et n'est pas mesurée.

**Comparaison avec des combats d'unités** (runs `20261030-002439` et `20261030-002536`, ×15) :
- 5 combats entre unités (type 1 contre type 1) : 0,52 à 0,67 s, médiane 0,64 s ;
- 3 nukes : 0,81 à 0,82 s. Un nuke dure donc 1,2 à 1,6 fois plus qu'un combat d'unités ;
- le premier nuke de chaque run dure 1,62 à 1,64 s (runs `20261030-002134` et `002439`). D'après
  Tim, ce tir déclare la guerre, et la déclaration fige brièvement le jeu. Ce surcoût n'existe pas
  en situation réelle, où les joueurs sont déjà en guerre quand ils s'envoient des nukes. Ce tir
  est donc exclu des comparaisons ;
- pour égaler la médiane de 0,64 s, il faudrait environ ×19 (vanilla ≈ 12,2 s) ; ×18 à ×23 pour
  couvrir tout l'intervalle.

**Bilan des comparaisons** (hors tirs qui déclarent la guerre) :

| animation | durée | facteur pour un nuke de même durée |
|---|---|---|
| nuke vanilla | 11,0 – 11,9 s | — |
| nuke ×10 | 1,17 – 1,22 s | — |
| nuke ×15 | 0,81 – 0,84 s | — |
| combat d'unités (5) | 0,52 – 0,67 s, médiane 0,64 s | ×18 – ×23, ×19 pour la médiane |
| attaque de ville (11) | 0,20 – 0,66 s, le plus souvent 0,45 – 0,51 s | ×19 – ×27 |

En ×15, Tim trouve que l'animation reste très lisible et que le rendu est bon. Pour une durée
alignée sur un combat d'unités, ×20 est un bon défaut pour le proxy.

Essai 4 (`runs/20261030-001047`) :
- l'échelle revient à 1 à chaque `CombatVisEnd` ;
- Tim confirme que le minuteur n'est pas touché ;
- défaut : deux visualisations qui n'étaient pas des nukes ont aussi été accélérées, un combat
  ordinaire (joueur 1 contre un barbare) et une visualisation vide. `180156ae0` sert aussi aux
  combats ordinaires calculés par GameCore ;
- correction : l'accélération exige maintenant une visualisation à un seul membre, le tireur.

Essai 1 (`runs/20261029-234430`) :
- le mod est bien chargé, et l'élément du mod remplace celui du jeu ;
- le gain est d'environ 45 %, mais l'arc du missile est presque aplati ;
- le décollage reste long. C'est l'animation du modèle du silo (`UnitMemberTypes/MissileSilo`,
  modèles par culture), que les paramètres de l'artdef ne règlent pas.

L'essai 2 multiplie la gravité par le carré du facteur de vitesse, pour retrouver la même
trajectoire sur un vol dix fois plus court. La durée ne change pas (`runs/20261029-234929`), mais
l'arc devient trop haut : la gravité du moteur n'est donc pas une simple accélération. L'option
`--gravite` règle maintenant le facteur directement, avec vitesse² / 2 par défaut.

**Le décollage.**
- C'est l'animation d'attaque du modèle `MissileSilo` (`units/units.blp`), atteint par
  `UnitMemberTypes/MissileSilo` → `Bins` → `Special/MissileSilo`.
- Sa chronologie est compilée dans le BLP, et l'artdef ne la règle pas.
- L'essai 3 fait donc tirer le silo avec le modèle de la fausse unité cible, `Special/TargetPlot`
  (option `--modele-silo`).
- Risque : si ce modèle n'a pas d'animation d'attaque, le tir peut ne jamais partir, ou partir
  seulement au bout d'un délai de garde.

**Essai 3** (`runs/20261029-235706`).
- La visualisation ne dure plus que 0,39 s, et plus rien n'est animé : ni décollage, ni vol, ni
  explosion.
- Le missile est une pièce attachée au modèle du silo. Sans ce modèle, il n'y a plus de missile à
  faire voler.
- **Option gardée de côté** (« tout supprimer ») : elle pourrait être la solution finale.
- La catégorie `Special` ne contient que `MissileSilo` et `TargetPlot` : aucun autre modèle ne
  porte le missile.
- L'artdef n'expose aucune vitesse d'animation. `AnimSpeed` ne sert qu'aux flèches de portée
  (`140239f10`), et `TimeScale` est un champ Lua de caméra.

**Essai 4 : accélérer le temps pendant la visualisation.**
- La commande de console `timescale` écrit un flottant en `App+0x559c`. L'application est en
  `exe+0x12fa2c8` ; la valeur vaut 1 en temps normal, vérifiée en jeu.
- `140076a40` multiplie le pas de chaque frame par ce facteur avant toute la mise à jour de
  l'affichage : animations, effets, caméra. GameCore ne la voit pas.
- `hook_nuke.py --accel N` :
  - repère une frappe WMD calculée pendant une vidange de la file GameCore ;
  - multiplie l'échelle par N au `CombatVisBegin` qui suit (dans les 2 s) ;
  - la remet à sa valeur au `CombatVisEnd`, et en se détachant.
- On garde ainsi l'animation vanilla complète (décollage, arc, explosion), jouée N fois plus vite.
- Pour une solution durable, le même réglage pourrait être fait par le proxy GameCore, sans Frida.

**Le minuteur de tour n'est pas touché.**
- Il est géré côté client par `1400f8170`, celui qui écrit « Turn Timer Sending TurnComplete ».
- Son temps écoulé (`1400f4df0`) vaut « maintenant − départ » sur `QueryPerformanceCounter`
  (`140998430`, `140998220`), auquel s'ajoute un cumul. C'est l'horloge réelle, pas le pas de
  frame que multiplie l'échelle de temps.
- GameCore, et donc l'état synchronisé, n'utilise pas non plus ce pas.
- L'indicateur et le compte à rebours sonore ne dérivent pas non plus.
  - `ActionPanel.lua` (`OnTurnTimerUpdated`) les calcule à partir du `elapsedTime` passé par
    l'événement `TurnTimerUpdated` : la barre, le texte, et les bips
    `Play_MP_Game_Launch_Timer_Beep` des 7 dernières secondes (arrondi de `timeRemaining`).
  - Côté exe, ce temps vient de `1400f5860`, qui sert aussi `UI.IsTurnTimerElapsed`. Il lit le
    même `1400f4df0` (`QueryPerformanceCounter`) que la fin de tour effective.
- Ce qui accélère pendant la fenêtre ne relève que de l'affichage local : animations, effets,
  caméra, et sans doute les scripts d'interface rafraîchis avec le pas de frame.

## Dans le proxy

L'accélération est maintenant dans le proxy de GameCore ([perf/rollover/proxy](../rollover/proxy),
module `src/wmd.rs`), avec le réglage `wmd_accel=20` de `rollover_proxy.cfg`. Elle reprend la logique
de `hook_nuke.js --accel`, avec deux différences :
- **Plusieurs frappes à la suite.** Les frappes calculées sont comptées, chacune attend sa
  visualisation pendant 3 s après la frappe précédente ou la fin de la visualisation accélérée
  précédente.
- **Le combat rapide.** L'accélération n'a lieu que si le joueur a activé le combat rapide.
  Sinon, l'animation garde sa vitesse d'origine.

**Essai en jeu (2026-10-30, ×20, combat rapide activé).** Tim trouve le résultat nickel. Le
journal du proxy compte 7 frappes accélérées sur deux parties, chaque fois rendues à 1 à
`CombatVisEnd` :
- le premier tir de chaque partie, qui déclare la guerre, dure 1,19 s et 1,47 s ;
- les suivants durent 0,62 et 0,66 s ;
- le mod « Nuke Quick Vis » était encore actif et faussait l'animation, ce qui explique les trois
  tirs à 0,39 – 0,42 s. Il est maintenant supprimé du dossier Mods, `build_mod.py` reste là pour
  le régénérer.

**Sans combat rapide.** Les deux frappes suivantes gardent leur vitesse d'origine (`combat rapide
désactivé, visualisation à vitesse normale`). Avec le combat rapide réactivé, les deux d'après
sont de nouveau accélérées, à 0,64 et 0,65 s. Tim valide les deux cas.

**Bombardier et sous-marin nucléaire.** Leurs frappes passent par `180156ae0`. Elles sont
accélérées de la même façon que celles d'un silo (essai de Tim, 2026-10-30).

**Exe DX11.** Le proxy reconnaît aussi `CivilizationVI.exe` (agencement `DX11` de `src/wmd.rs`).
Essai de Tim : `wmd : détours posés (DX11)`, puis deux frappes accélérées. La première, qui
déclare la guerre, dure 1,49 s, la suivante 0,68 s. Rien d'autre à signaler.

## Trace d'un lancer

`hook_nuke.js` date et note la pile (24 cadres, `module!rva`) de :

| événement | site | sens |
|---|---|---|
| `gc_icbmResults`, `gc_wmdResults` (+ `_fin`) | `180158af0`, `180156ae0` | GameCore calcule la frappe ; le thread est celui de GameCore |
| `exe_visBegin` | `1400f0400` | début de la visualisation du combat, avec membres et case |
| `exe_visEnd` | `1400f0460` | fin de la visualisation |
| `gc_post` | `1800373e0` | un message entre dans la file de GameCore, avec son type |
| `gc_drain` (+ `_fin`) | `1800372f0` | le thread GameCore vide la file |

Les autres combats déclenchent aussi `exe_visBegin` et `exe_visEnd`. Le nuke se reconnaît parce
qu'il suit de près `gc_*Results`.

### Protocole

1. Préparer une partie de test (multijoueur local, un humain, comme pour `perf/rollover`) :
   - un bombardier ou un silo en état de tirer, et une cible à portée ;
   - pour se donner une bombe, mettre `EnableTuner 1` dans `AppOptions.txt`, puis dans FireTuner
     (contexte GameCore) :
     `Players[0]:GetWMDs():ChangeWeaponCount(GameInfo.WMDs["WMD_NUCLEAR_DEVICE"].Index, 1)`.
2. Jeu ouvert (version DX12), lancer le hook :
   ```bash
   .venv/Scripts/python.exe -I perf/nuke/hook_nuke.py
   ```
3. Dans un PowerShell administrateur, démarrer la capture WPR :
   ```bash
   powershell -ExecutionPolicy Bypass -File perf\rollover\capture.ps1 -Name nuke
   ```
4. En vue 3D, lancer la bombe. Attendre d'avoir de nouveau la main, puis appuyer sur Entrée dans
   la fenêtre de capture et faire Ctrl+C dans celle du hook.
5. Analyser les attentes. `attentes.py` trouve seul les visualisations de plus de 2 s et les
   threads GameCore et principal. Il compte aussi les attentes qui débordent de la fenêtre, ce que
   `perf/rollover/waits.py` ne fait pas :
   ```bash
   .venv/Scripts/python.exe -I perf/nuke/attentes.py perf/nuke/runs/<dossier>/frida.log perf/rollover/captures/<dossier>
   ```
