# Bouton d'attaque des bannières : patcher l'interface depuis le proxy

Deux gênes, à corriger sans mod, depuis le proxy GameCore :
- le bouton rouge d'attaque (ville, campement) et le bouton nuke du silo sont dessinés sous les drapeaux
  d'unités, qui prennent le clic ;
- un clic sur l'un de ces boutons recentre la caméra sur la ville.

## Côté Lua et XML (2026-10-30)

Fichiers du jeu, Gathering Storm : `DLC/Expansion2/UI/CityBanners/`.

- **Boutons** (`CityBannerInstances.xml`) :
  - `CityStrikeButton` dans les instances `CityBanner` et `EncampmentBanner`, avec `ConsumeMouse="1"` ;
  - `NukeBombButton` et `ThermoNukeBombButton` dans `WMDBanner`, **sans** `ConsumeMouse`.
- **Clics** (`CityBannerManager.lua`) : `OnCityStrikeButtonClick`, `OnDistrictRangeStrikeButtonClick`,
  `OnICBMStrikeButtonClick`. Chacun sélectionne avant de changer de mode (`UI.SelectCity`,
  `UI.SelectDistrict`), et c'est cette sélection qui recentre. Le jeu contient lui-même, en
  commentaire, `UI.SelectCity( pCity, false ); -- Don't auto center`. À vérifier en jeu.
  - Les modes `CITY_RANGE_ATTACK`, `DISTRICT_RANGE_ATTACK` et `ICBM_STRIKE` de `WorldInput.lua` ne
    recentrent pas.
- **Ordre d'affichage** (`InGame.xml`) : dans `BannerAndFlags`, `UnitFlagManager` suit
  `CityBannerManager`. Les drapeaux sont donc dessinés au-dessus et prennent le clic en premier.

**Dans les parties réelles, ce ne sont pas les fichiers du jeu qui sont chargés** (`UserInterface.log`) :
- `CityBannerManager.lua` et `CityBannerManager.xml` viennent de Better Spectator Mod (Workshop
  1916397407, `UI/Replacements/`). Les trois gestionnaires y sont les mêmes (lignes 2890 à 2932).
  Son XML inclut `CityBannerInstances`, qui reste celui du jeu ;
- `InGame.xml` et `InGame.lua` viennent de MPH (Workshop 3307282026).

Le patch doit donc remplacer des motifs dans le texte effectivement chargé, et ne pas remplacer un
fichier par un autre.

## Côté exe : où les fichiers sont lus

Projet Ghidra `civ6exe` (`CivilizationVI_DX12.exe`, base 0x140000000). HavokScript est une DLL à
part (`HavokScript_FinalRelease.dll`) ; l'exe importe ses fonctions.

| adresse | rôle |
|---|---|
| `1408ec9f0` | `LuaContext::Initialize` (« START  LuaContext::Initialize() id: '%s' ») : `LoadXML`, puis construit `<FileName>.lua`, applique `ForgeUI_UIManager::GetUIReplacementScript` (`14088d670`) et range le chemin en `contexte+0xc70` (`1400389b0` n'est qu'une copie de chaîne) |
| `1409801b0` | chargeur de script `(this, L, chemin UTF-16, include)` : résout le chemin (virtuelle `+0x30`), le convertit en UTF-8, lit le fichier entier (`140995bb0` par son thunk `1409965f0`), puis appelle `140dc30e0`. Messages : « Error opening/reading where=%s, file=%s », « Attempt to load empty file » |
| `140dc30e0` | `(L, tampon, taille, nom)` : `luaL_loadbuffer` sous une section critique (`14fca2920`), puis exécution du chunk (`140dc1f60`). Seul appelant : `1409801b0` |
| `140dcc248` | thunk de l'import `luaL_loadbuffer`, entrée d'IAT `140fa54e8`. Seul appelant : `140dc30e0` |
| `14087f4d0` | `ContextBase::LoadXML_File(chemin, bool)`, exporté : ajoute `.xml`, résout le chemin (`UIManager` virtuelle `+600`), puis `140a306d0` → `140a306e0` |
| `140a2d280` | `(chemin UTF-16, tampon)` : lit le fichier XML entier (`140996500` ouvre, `140996620` lit), ajoute un NUL final. `140a307f0` analyse ensuite le tampon |
| `140995bb0`, `140995d10` | lecture générique d'un fichier entier (`CreateFileW` + `ReadFile`) dans un tampon extensible `{ptr, taille, …}`, agrandi par `1409971c0` |

**Points d'accroche pour le proxy :**
- **Lua.** Remplacer l'entrée d'IAT de `luaL_loadbuffer` (`140fa54e8`). Ce n'est qu'un pointeur à
  changer, sans réécrire d'instruction, et l'entrée se retrouve par le nom de l'import. Si le nom du
  chunk finit par `CityBannerManager.lua`, on passe un tampon modifié qui appartient au proxy :
  `luaL_loadbuffer` compile tout de suite et ne garde pas le tampon. Sinon, on relaie l'appel.
- **XML.** Détourner `140a2d280` : appeler l'original, puis, si le chemin finit par
  `CityBannerInstances.xml`, réécrire le tampon. L'agrandir avec `1409971c0`, l'allocateur du jeu,
  puisque l'analyseur et la libération lisent ce tampon.

**Moment du chargement.** Dans `Lua.log`, les scripts de gameplay (`MP_helper`, `Spectator`, contexte
GameCore) tournent avant `CityBannerManager: BSM CityBanner`. `DllCreateGameContext`, qui charge le
proxy, passe donc avant l'interface de partie. Le journal du proxy le confirme (11:35:47, pendant
`CONFIGURE_CONTENT`, 11:35:46 → 11:35:52). Les contextes du menu principal sont chargés plus tôt,
mais ils ne sont pas concernés.

## Prototype dans le proxy (2026-10-30)

Le but est que les boutons soient **dessinés au-dessus** des drapeaux d'unités : il ne suffit pas
qu'ils prennent le clic. Mise en œuvre et réglages : section « Cinquième accroche » de
[proxy/README.md](../proxy/README.md).

- **Pourquoi déplacer le bouton, et pas la bannière.** `InGame.lua` sait déjà passer toutes les
  bannières au-dessus des drapeaux : avec les lentilles religion, loyauté et tourisme, il remet le
  contexte `CityBannerManager` en dernier dans `BannerAndFlags`. Mais les bannières couvriraient
  alors les drapeaux partout. On ne sort donc que les boutons.
- **Le calque.** Il reproduit le cadre de la bannière, et `ui_boutons.xml` copie ces cadres. Par
  exemple, `CityStrike` est ancré `C,B` / `I,O` sous `Container` (`C,C`, décalage `0,-20`,
  `200x41`). Déplacé dans un cadre identique, le bouton reste à la même place à l'écran.
- **Le conteneur.** `SelectedUnitContainer` est déclaré par `UnitFlagManager.lua` sans jamais servir
  (vanilla et BSM). Il est dessiné après `BannerAndFlags` et `SelectedMapPinContainer`, et avant
  `WorldViewPlotMessages`.
- **La position sur la carte.** `place` refait le calcul de `CityBanner:UpdatePosition`
  (`GridToWorld`, plus `YOFFSET_2DVIEW` ou `ZOFFSET_3DVIEW`). Si un mod change ce calcul, le calque
  se décale.
- **Le Lua compilé dans le jeu est HavokScript.** Le bloc n'utilise que du Lua 5.1 et ne fait que
  passer `luac -p` hors du jeu. Il tourne en jeu depuis le correctif de l'import (voir plus bas).

## Essais en jeu (2026-10-30)

- **Premier essai : aucun effet.** L'accroche Lua n'était pas posée : l'exe importe
  `?luaL_loadbuffer@@YAHPEAUlua_State@@PEBD_K1@Z`, nom C++ décoré, et le proxy cherchait le nom
  simple. Une fois ce point corrigé, les boutons passent au-dessus des drapeaux, ils prennent le clic
  devant un camion de ravitaillement ou un tank, et la caméra ne recentre plus.
- **Second clic nécessaire sur le bouton d'un centre-ville non sélectionné.** Le mode de visée
  s'ouvre puis se referme aussitôt. La cause est une faute de `CityPanel.lua`
  (`OnCitySelectionChanged`, ligne 629), sans rapport avec le proxy :
  1. le clic sélectionne la ville, puis passe en `CITY_RANGE_ATTACK` ;
  2. la sélection a changé : `CitySelectionChanged` arrive, déjà dans ce mode ;
  3. le gestionnaire veut garder le mode si la ville peut tirer, mais teste
     `CityManager.GetCity(owningPlayerID, cityID)`. `owningPlayerID` n'existe pas (le paramètre est
     `ownerPlayerID`) : la ville vaut nil, le test échoue, et l'interface repasse en `SELECTION` ;
  4. une ville déjà sélectionnée ne produit pas d'événement, d'où le second clic qui fonctionne.

  Le campement (`UI.SelectDistrict`) et le silo (testé en `ICBM_STRIKE` sans cette faute) ne sont
  pas touchés. Le proxy corrige le nom au chargement de `CityPanel.lua`. Vérifié en jeu : un seul
  clic suffit.
- **Un clic sur le drapeau de la cible ne tirait pas** en visée d'une ville ou d'un quartier : il
  fallait cliquer sur la case. Une unité, elle, tire aussi en cliquant sur le drapeau de sa cible.
  - Le bouton du drapeau prend le clic avant `WorldInput`, qui ne le voit donc pas.
  - `OnUnitFlagClick` (`unitflagmanager.lua` de BSM, identique au jeu hors mode spectateur) ne
    connaît pour un drapeau étranger que l'attaque par l'unité sélectionnée
    (`UI.GetHeadSelectedUnit`). En visée d'une ville, il n'y en a pas.
  - Le proxy branche le clic gauche des drapeaux sur un gestionnaire qui tire sur la case de l'unité,
    exactement comme `CityRangeAttack` / `DistrictRangeAttack` : `CanStartCommand`, puis
    `RequestCommand`, puis retour en `SELECTION`. Sinon, il appelle `OnUnitFlagClick`.
  - Le clic droit et le WMD ne changent pas. À vérifier en jeu.

## Reste à vérifier en jeu

- Un clic sur le drapeau d'une unité ennemie à portée, en visée d'une ville puis d'un campement,
  tire-t-il ? Un clic sur un drapeau hors de portée, ou sur le drapeau d'une de ses propres unités,
  se comporte-t-il comme avant ?
- `UI.SelectDistrict` avec `false` ne recentre-t-il plus non plus (bouton du campement) ?
- Que se passe-t-il pour le silo d'un autre joueur (bouton masqué), à la destruction d'une
  bannière, à la révélation d'une merveille, et en quittant la partie puis en en chargeant une
  autre ?

Scripts Ghidra : `~/tools/ghidra-scripts`, dont `FindSymbols.java` (symboles par expression
régulière, avec leurs appelants ; `@fichier` pour une expression qui contient des `|`). Sorties :
`~/tools/ghidra-projects/uiload_*`.
