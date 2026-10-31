# Proxy de démarrage : DatabaseDLL_FinalRelease.dll

Le proxy de GameCore ([proxy](../proxy/README.md)) n'est chargé qu'au lancement d'une partie
Gathering Storm. Ce second proxy est chargé au démarrage de l'exe, avant le menu principal. Il
ajoute à l'écran d'options du jeu un onglet **BetterGameDLLs**, qui édite les réglages du proxy de
GameCore (`rollover_proxy.cfg`).

## Pourquoi DatabaseDLL

Les deux exe (DX11 et DX12) importent statiquement quatre DLL du jeu, posées à côté d'eux dans
`Base\Binaries\Win64Steam` :

| DLL | exports |
|---|---:|
| `DatabaseDLL_FinalRelease.dll` | 5 |
| `Localization_FinalRelease.dll` | 118 |
| `HavokScript_FinalRelease.dll` | 728 |
| `steam_api64.dll` | (Steam, écartée) |

DatabaseDLL a le moins d'exports, et ce n'est pas une DLL de Windows : pas de conflit avec le moteur
de compatibilité ni avec le graphisme, contrairement aux essais avec `version.dll` et `dxgi.dll`
(voir le README du proxy de GameCore).

## Principe

- L'original est renommé `DatabaseDLL_FinalRelease_orig.dll`, et le proxy prend son nom.
- Les 5 exports (`Database::Initialize`, `Shutdown`, `CreateConnection`…) sont des renvois PE
  (*forwarders*) vers l'original : le chargeur de Windows les résout lui-même, sans code de relais.
  La chaîne d'outils est GNU, donc les renvois passent par un fichier `.def` écrit par
  [build.rs](build.rs).
- Au chargement, le proxy écrit `démarrage : proxy DatabaseDLL chargé dans <exe>` dans le journal
  commun, `%LOCALAPPDATA%\Civ6RolloverProxy\RolloverProxy.log`.

## L'onglet BetterGameDLLs

[src/options.rs](src/options.rs). L'écran d'options est le même contexte `Options.xml` /
`Options.lua` au menu principal et en jeu.

- **Les accroches.** Dès `DllMain`, le proxy remplace l'entrée `luaL_loadbuffer` de la table
  d'imports de l'exe et détourne sa lecture XML. Ce sont les accroches du proxy de GameCore, dans
  le module commun [proxy/src/exe.rs](../proxy/src/exe.rs), inclus par `#[path]`. L'entrée doit déjà
  viser HavokScript : sinon rien n'est posé, et le journal le dit.
- **Le XML.** Après `Options.xml`, on ajoute le bouton d'onglet
  ([src/options_proxy_tab.xml](src/options_proxy_tab.xml)) sous `AppTab`, et le panneau
  ([src/options_proxy_panel.xml](src/options_proxy_panel.xml)) avant `KeyBindings`.
- **Le Lua.** [src/options_proxy.lua](src/options_proxy.lua) est ajouté à la fin d'`Options.lua`,
  pour lire `m_tabs`. Il construit une liste déroulante par réglage, avec les instances du fichier
  (`PullDownParameterInstance`, `KeyBindingCategory` pour les titres).
- **Les fonctions natives**, posées dans la table globale du contexte avant sa compilation :
  `Civ6Proxy_LireReglages()` rend le texte du fichier, 1 si le proxy de GameCore est installé, et 1
  s'il a déjà lu le fichier ; `Civ6Proxy_EcrireReglages(lignes)` fusionne des lignes `clé=valeur`
  dans le fichier, et rend aussi ce dernier indicateur. Seules les
  lignes changées sont réécrites, et les doublons d'une clé changée sont retirés : `read_config`
  garde la dernière occurrence. HavokScript n'exporte ni `lua_pushlstring` ni `lua_pushboolean` :
  les chaînes passent par `lua_pushfstring("%s")`, les booléens par `lua_pushinteger`.
- **Redémarrage seulement s'il le faut.** Le proxy de GameCore ne lit le fichier qu'à son
  chargement, au lancement de la première partie. `gamecore_config_read` le demande à son export
  `RolloverProxyStatus` (bit 2, originale chargée), si le module est chargé.
  - Pas encore lu : **Confirmer** écrit le fichier sans autre message, et l'avertissement dit que
    les réglages prendront effet à la prochaine partie.
  - Déjà lu : **Confirmer** écrit le fichier puis lève `_PromptRestartApp`. Le jeu affiche son
    propre message « redémarrage nécessaire » (`LOC_OPTIONS_CHANGES_REQUIRE_APP_RESTART`), et
    l'avertissement le dit aussi.
  - **Annuler** n'écrit rien. Chaque ouverture relit le fichier et met l'avertissement à jour.
- **Langue.** Textes en français si la langue du jeu (`Locale.GetCurrentLanguage().Type`)
  commence par `fr`, en anglais pour toutes les autres. Chaque texte de `options_proxy.lua` est une
  paire `{ français, anglais }`.
- **Un mod qui remplace `Options.xml`** : sans le bouton et le panneau, le bloc Lua ne fait rien.
- **Coexistence avec le proxy de GameCore.** L'accroche Lua s'enchaîne d'elle-même : il lit notre
  accroche comme fonction d'origine. Pour la lecture XML, il trouve notre `jmp [rip+0]` au lieu du
  prologue et le recopie dans son trampoline. À son déchargement, il remet notre saut.
- **Le journal.** `options : accroches posées`, puis à l'ouverture des options
  `options : …Options.xml : onglet des proxys ajouté` et `…Options.lua : …, compilation réussie`,
  et `options : rollover_proxy.cfg enregistré (clé=valeur, …)` à chaque confirmation.

**Toute nouvelle fonctionnalité des proxys doit avoir son réglage dans cet onglet.** Le test
`cles_communes_avec_le_proxy` échoue si une clé lue par `read_config` (proxy/src/lib.rs) manque à
`PROXY_REGLAGES` dans `options_proxy.lua`, ou l'inverse. Le libellé, l'aide et les choix se
donnent en français et en anglais.

## Compiler, tester, installer

```bash
cd startup_proxy && cargo build --release
```

**Tests hors jeu.**
- `cargo test --release` : le patch du vrai `Options.xml`, la fusion du fichier de réglages, la
  concordance des clés avec le proxy de GameCore.
- Le bloc Lua, chargé à la suite du vrai `Options.lua` : onglet, listes, valeurs du fichier,
  enregistrement à la confirmation, message de redémarrage, réouverture.

  ```bash
  lua startup_proxy/tests/test_options_proxy.lua
  ```
- La DLL : elle se charge, chaque export aboutit à la même adresse que dans l'original, la ligne de
  journal est écrite. Une fois le proxy installé, l'original est `DatabaseDLL_FinalRelease_orig.dll`.

  ```bash
  .venv/Scripts/python.exe -I startup_proxy/tests/check_forwarders.py startup_proxy/target/release/DatabaseDLL_FinalRelease.dll "<jeu>\Base\Binaries\Win64Steam\DatabaseDLL_FinalRelease.dll" <dossier temporaire>
  ```

**Installation**, jeu fermé. Comme pour le proxy de GameCore, une vérification des fichiers par
Steam ou une mise à jour du jeu remet l'original :

```bash
powershell -ExecutionPolicy Bypass -File startup_proxy/install.ps1
```

```bash
powershell -ExecutionPolicy Bypass -File startup_proxy/install.ps1 -Uninstall
```

## Résultat (31 octobre 2026)

Lancements en DX12 puis en DX11 jusqu'au menu principal : le journal contient
`démarrage : proxy DatabaseDLL chargé dans …\Win64Steam\CivilizationVI_DX12.exe`, puis la même
ligne avec `CivilizationVI.exe`. Le jeu démarre normalement dans les deux cas.

Onglet, en DX12 : il apparaît au menu principal et en jeu, avec les valeurs du fichier. Trois
réglages changés et confirmés sont écrits (`portee`, `ameliorations`, `rendements`), et le jeu
affiche son message de redémarrage. En partie, les boutons d'attaque restent au-dessus des
drapeaux : les deux détours de la lecture XML s'enchaînent.
