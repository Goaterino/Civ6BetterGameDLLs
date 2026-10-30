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
  passer `luac -p`. Il n'a pas encore tourné dans le jeu.

## Reste à vérifier en jeu

Installer avec `-UiJournal on` pour la première partie, puis lire le journal du proxy et `Lua.log`.

- Le nom de chunk passé à `luaL_loadbuffer` finit-il par `CityBannerManager.lua` ? Sinon, le
  journal liste les noms réels.
- Les boutons restent-ils à leur place, en 3D comme en vue stratégique, au zoom proche comme
  lointain ? Sont-ils au-dessus des drapeaux ?
- `UI.SelectCity(pCity, false)` ne recentre-t-il plus ? Et `UI.SelectDistrict` avec le même
  argument ?
- Que se passe-t-il pour le silo d'un autre joueur (bouton masqué), à la destruction d'une
  bannière, à la révélation d'une merveille, et en quittant la partie puis en en chargeant une
  autre ?

Scripts Ghidra : `~/tools/ghidra-scripts`, dont `FindSymbols.java` (symboles par expression
régulière, avec leurs appelants ; `@fichier` pour une expression qui contient des `|`). Sorties :
`~/tools/ghidra-projects/uiload_*`.
