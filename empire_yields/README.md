# Rendements d'empire figés après un gouverneur ou un personnage illustre

Gêne observée en partie (2026-11-01). Après un titre de gouverneur, un déplacement de gouverneur ou
l'activation d'un personnage illustre, le bandeau du haut (or, science, culture, foi) garde ses
anciennes valeurs. Il fallait sélectionner une ville et retirer puis replacer un citoyen pour le
voir changer. Projet Ghidra `civ6xp2` (FinalRelease d'origine, base 0x180000000), traces Frida en
jeu, correctif dans le proxy ([proxy/src/rendements_empire.rs](../proxy/src/rendements_empire.rs),
[proxy/src/ui_rendements.lua](../proxy/src/ui_rendements.lua)).

## GameCore calcule les bons rendements

| adresse | rôle |
|---|---|
| `1801332f0` | rendement d'une ville par type, mémorisé dans `ville+0x1950` (valeur, valide) |
| `18063d1a0`, `18063cee0` | ouverture et fermeture de la portée du cache (`jeu+0x2a0`, compteur) |
| `180265d40` → `1801346f0` | à l'ouverture, signal `jeu+0x280+0x18f0` : caches de toutes les villes vidés |
| `180341600` | rendement en or du trésor, somme des villes par `1801332f0` |

Le cache des villes ne sert que dans la portée, ouverte par le traitement du tour et de l'IA. Hors
portée, chaque lecture refait le calcul. Les rendements cumulés à la fin du tour sont donc justes :
seul l'affichage est en retard.

## L'interface lit des instantanés

| adresse | rôle |
|---|---|
| `1805fc5a0`, `1805fb9d0`, `1805fb000`, `1805fb110` | rapport d'un événement : gestionnaires de cache (`18007c670`), puis transmission au moteur (Lua, interface) |
| `18007c670` | répartition vers les gestionnaires abonnés au hachage de l'événement (`18007c210` les enregistre) |
| `1800c6e90` | gestionnaire du trésor : instantané `1800c66e0(joueur, 1)`, posté par le moteur (`+0xb0`) avec `0xcdf04e2f` |
| `1800c4b40`, `1800b42e0`, `1800c0da0` | sciences (`1800c45e0`), culture (`1800b4490`), foi (`1800c0f60`) |
| `180072770` | une ville (`180072120`) |

Un gestionnaire reçoit `(événement, enregistrement)` et lit le joueur en `+0x18`, le type en `+0x1a`
et l'identifiant en `+0x1c`.

`TopPanel.lua` ne redessine ses rendements que sur une liste d'événements : `CityWorkerChanged`,
`UnitGreatPersonActivated`, `TreasuryChanged`… Aucun événement de gouverneur n'y figure.

## Traces

[hook_yields.js](hook_yields.js), lancé par :

```bash
.venv/Scripts/python.exe -I empire_yields/hook_yields.py
```

La touche Inser, en jeu, écrit un repère numéroté dans le journal. Résumé par repère :

```bash
.venv/Scripts/python.exe -I empire_yields/analyse.py empire_yields/runs/<date-heure>/frida.log
```

Partie du 2026-11-01 (`runs/20261101-213827`), joueur local 6 :

| action | événements | caches refaits | bandeau |
|---|---|---|---|
| retirer et replacer un citoyen | `0xc2f385c0` | villes, culture, foi, sciences, trésor | à jour |
| titre de gouverneur (Reyna) | `0x8c6b2399` | gouverneurs, une partie de la ville | inchangé |
| déplacement de Reyna | `0xef1e2348` pendant le retrait, puis `0xcf9c73f1` | villes, culture, foi, sciences, trésor, **pendant** le retrait | inchangé |
| Einstein (+4 science sur le laboratoire) | `0x48062f97`, `0xd8adfd71`… pendant l'activation | trésor, sciences (`1800c4af0`, pas le gestionnaire du citoyen) | inchangé |

Le titre ne refait pas les bons instantanés. Le déplacement et le personnage illustre les refont
avant que l'action ne soit appliquée, et le bandeau n'écoute pas les gouverneurs.

Le premier journal (`runs/20261101-210505`) a servi à mettre la trace au point : la promotion y
était mal accrochée (`1802ddbb0` est le test « peut-il être promu ? ») et les caches y apparaissent
tous sous leur relais commun, `18000b470`.

## Le correctif

Réglage `ui_rendements=on|off`, on par défaut.

- **GameCore.** Promotion (`1802df340`), retrait (`1802df800`) et établissement (`180259b10`) d'un
  gouverneur, activation d'un personnage illustre (`180655560`, joueur en `unité+0x128`) notent le
  joueur, s'il est le joueur local (`180044e60`). Une fois l'action appliquée, le proxy appelle
  pour lui les gestionnaires de `0xc2f385c0` : `180072f80` pour ses villes, puis culture, foi,
  sciences et trésor avec un enregistrement fabriqué. Il le fait à la sortie de l'action la plus
  externe, ou, si elle s'exécute dans une passe de `Game_Processor` (`18004afc0`), à la fin de la
  passe, avant l'envoi des caches à l'interface. Une première version n'attendait que la fin de
  passe : les ordres du joueur pendant son tour (titre, personnage illustre) n'en sont pas suivis,
  et rien n'était rafraîchi.
- **Interface.** `TopPanel.lua` appelle `OnRefreshYields` sur `GovernorAssigned`,
  `GovernorChanged`, `GovernorPromoted` et `UnitGreatPersonActivated`, puis encore à
  `GameCoreEventPublishComplete`. Le bloc est ajouté au fichier inclus : BSM
  (`TopPanel_Spectator.lua`) en profite.

**Pas de désynchro.** Le multijoueur ne fait circuler que des commandes, et le proxy n'en envoie
aucune. Il n'émet pas non plus d'événement Lua, qu'un script de gameplay pourrait suivre. Les
gestionnaires appelés sont ceux que le jeu exécute à chaque changement de citoyen. Ils n'écrivent
dans aucun champ suivi : vérifié pour l'instantané des villes
([cascade_guerre.md](../rollover/cascade_guerre.md)), et au premier niveau pour le trésor
(`180341600`, `180342620`), les sciences, la culture et la foi. Le passage de tour du trésor
(`180343780`), qui écrit, n'est pas sur ce chemin. Un joueur sans proxy n'est pas concerné.

## Vérifié en jeu (2026-11-01)

Titre de gouverneur, déplacement de Reyna et Einstein : le bandeau se met à jour aussitôt. Le
journal du proxy note chaque action (`rendements d'empire : promotion de gouverneur du joueur 6,
rafraîchi en fin de l'action`), puis `instantanés refaits pour le joueur 6`.
