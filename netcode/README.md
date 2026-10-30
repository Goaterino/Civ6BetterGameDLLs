# Outils de diagnostic du netcode

Analyse et hypothèses : [docs/netcode](../docs/netcode/README.md).

- `hook_reconnect.js` / `hook_reconnect.py` : hook Frida pour la **machine hôte**. Il journalise avec
  l'heure à la milliseconde et le thread :
  - côté GameCore_XP2 :
    - chaque `PlayerConnectionChange` traité, avec le joueur, connexion ou départ, et exclusion ;
    - le statut d'emplacement lu **avant** et **après** le traitement ;
    - les drapeaux qui autorisent la relance de la mise à jour ;
    - toute écriture du statut d'emplacement, en précisant si elle a lieu pendant le message ;
  - côté exe (DX12) :
    - `PlayerLeft` et `PlayerConnected`, et les écritures du statut d'emplacement ;
    - la demande, la sauvegarde et l'envoi de l'instantané ;
    - chaque contrôle de synchro comparé.

  Le hook vérifie les octets de chaque fonction avant de l'accrocher. Il fonctionne avec ou sans le
  proxy de GameCore ([proxy](../proxy/README.md)).
- `collect_logs.ps1` : archive sur le Bureau les journaux du jeu, les derniers autosaves
  multijoueur, `AppOptions.txt`, `Mods.sqlite` et, avec `-Extra`, un dossier en plus. Il est
  autonome, on peut l'envoyer à un autre joueur.

```bash
.venv/Scripts/python.exe -I netcode/hook_reconnect.py
```

```bash
powershell -ExecutionPolicy Bypass -File netcode/collect_logs.ps1 -Extra netcode/runs/<dossier>
```

Les journaux du hook vont dans `netcode/runs/<date-heure>/frida.log` (non versionné).

- `empreinte.js` / `empreinte.py` : à chaque `StartGameSyncCheck` (fin de tour en multijoueur),
  relève le hachage de **chaque AutoArchive**, le même que celui du contrôle de synchro, ainsi que le
  hachage de chacune de ses variables. Résultat : `netcode/runs/empreinte-<date-heure>/empreinte_<n>_<heure>.tsv`.
  Le relevé bloque le jeu environ une seconde par fin de tour. Pour certaines variables (liste
  `DETAILS` du script, aujourd'hui `Player::Units.m_aiAddOnDistrictFounding`), une 5ᵉ colonne
  donne aussi leur disposition en mémoire : nombre de seaux, taille, éléments dans l'ordre.
- `compare_empreintes.py` : compare deux empreintes. Il donne les archives différentes par type,
  puis les indices des variables qui diffèrent.

```bash
.venv/Scripts/python.exe -I netcode/empreinte.py
```

```bash
.venv/Scripts/python.exe -I netcode/compare_empreintes.py A.tsv B.tsv
```

## Protocole solo : l'état en mémoire contre l'état rechargé

Question : un tour joué avec l'état gardé en mémoire donne-t-il le même résultat qu'un tour joué juste
après avoir chargé la sauvegarde ? Si non, les archives qui diffèrent sont celles qui font
désynchroniser un revenant (voir le test du 2026-10-29 dans [docs/netcode](../docs/netcode/README.md)).

**Préparation**
- Proxy retiré (`install.ps1 -Uninstall`), Civ6 en DX12.
- Partie **multijoueur hébergée seul** (Internet ou LAN), puisque le relevé se déclenche sur le
  contrôle de synchro du multijoueur.
- Sauvegarde : un autosave multijoueur, par exemple `AutoSave_0077` de la partie du 29.
- Si le salon refuse de lancer avec l'emplacement de l'autre joueur vide, passer cet emplacement en
  IA. Faire de même dans **tous** les parcours.
- **Ne rien jouer** : juste finir les tours, sans toucher aux unités ni aux villes, pour que seuls
  le moteur et les IA agissent.

**Étape 0, vérifier que l'outil marche.** Lancer `empreinte.py`, charger la partie, finir un tour. Le
terminal doit écrire `empreinte 1 : ~9300 archives`. Si rien ne vient en fin de tour, s'arrêter et me
le dire : le contrôle de synchro ne tourne peut-être pas sans client.

**Parcours A, en mémoire.** Charger `AutoSave_0077`, finir le tour 77 (empreinte A77), puis le
tour 78 (empreinte A78). Copier tout de suite le nouvel `AutoSave_0078` dans `Saves\Multi` sous le
nom `TestB_0078.Civ6Save`, avant qu'il ne soit écrasé : le jeu le proposera au chargement.

**Parcours B, rechargé.** Charger `TestB_0078` et finir le tour 78 (empreinte B78).

**Parcours C, témoin.** Recharger `AutoSave_0077` et finir le tour 77 (empreinte C77).

**Lecture :**
- `A77` contre `C77` doivent être identiques : deux parcours qui partent du même chargement. Sinon,
  la fin de tour n'est pas reproductible, même sans rechargement, et la méthode ne vaut rien en
  l'état ;
- `A78` contre `B78` : chaque archive différente est un état que la sauvegarde ne restitue pas, ou que
  le chargement recalcule autrement.

Garder une seule session `empreinte.py` pour les trois parcours, ou noter quel fichier correspond à
quel parcours. À la fin, `collect_logs.ps1 -Extra netcode\runs\empreinte-<dossier>` : le
`net_message_debug.log` dira si les opérations envoyées par l'interface (BBG, MPH) ont différé d'un
parcours à l'autre.

## Protocole : reconnexion en cours de partie

Objectif : reproduire la boucle de désynchro après déconnexion puis retour, et départager les
hypothèses de [docs/netcode](../docs/netcode/README.md#reconnexion-en-cours-de-partie-et-boucles-de-désynchro).

**Avant, chez l'hôte :**
1. Jeu fermé, retirer le proxy de GameCore, pour avoir le même GameCore que l'autre joueur :
   `powershell -ExecutionPolicy Bypass -File proxy\install.ps1 -Uninstall`.
2. Lancer Civ6 en **DX12**, puis le hook. Charger la partie : le hook doit écrire
   `GameCore_XP2_FinalRelease.dll accroché` et `CivilizationVI_DX12.exe accroché`.

**Avant, chez l'autre joueur :** mêmes mods, `collect_logs.ps1` reçu, horloge Windows synchronisée.

**Pendant :**
3. Jouer 1 ou 2 tours normalement (référence sans désynchro).
4. En plein tour, après que l'autre joueur a bougé quelques unités, le couper :
   - essai A : il ferme le jeu de force (Gestionnaire des tâches) ;
   - essai B : il coupe son réseau une vingtaine de secondes.

   Pas la commande MPH `.manual_disconnect…`, qui laisse MPH agir avant la coupure.
5. S'il a fermé le jeu, il lance `collect_logs.ps1` **avant** de relancer Civ6, qui réécrit les
   journaux au démarrage. Puis il revient.
6. L'hôte attend son retour, en pause, sans cliquer sur la reprise de MPH.
7. Noter :
   - l'hôte a-t-il eu la notification « X Disconnected » ?
   - au retour : PM des unités, gouverneurs et production du revenant, comparés à avant la coupure
     (captures d'écran) ;
   - les tours où « X Desynced » apparaît.
8. Jouer au moins 6 tours après le retour.

**Après :** arrêter le hook (Ctrl+C), quitter le jeu des deux côtés **sans le relancer**, puis lancer
`collect_logs.ps1`. L'hôte ajoute `-Extra netcode\runs\<dossier>`. Rassembler les archives.

**Le revenant collecte deux fois** : à l'étape 5, puis à la fin de la partie, avant toute relance.
C'est la seconde archive qui contient sa réaction aux réparations de l'hôte (`delta data mismatch`,
`No desynced AutoVariables found`, `Received empty Delta data`). Au test du 2026-10-29, elle manquait.

Si la boucle ne se produit pas, recommencer en collectant à chaque fois avant de relancer le jeu.
Variante utile : un essai où l'hôte reprend **sans** attendre le revenant, qui doit désynchroniser à
coup sûr d'après le code de MPH.
