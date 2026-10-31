# Releases

Les releases sont construites par GitHub Actions ([release.yml](../.github/workflows/release.yml)),
jamais depuis un PC : n'importe qui peut relier un binaire publié au commit qui l'a produit.

## Contenu d'une release

| fichier | rôle |
|---|---|
| `GameCore_XP2_FinalRelease.dll` | proxy de GameCore (`proxy/`), à poser dans `DLC/Expansion2/Binaries/Win64` |
| `DatabaseDLL_FinalRelease.dll` | proxy de démarrage (`startup_proxy/`), à poser dans `Base/Binaries/Win64Steam` |
| `manifest.json` | version, version du salon (`proxy/src/version.rs`), commit, et pour chaque fichier : dossier de destination, taille, SHA-256 |
| `manifest.json.sig` | signature ed25519 du manifeste, en base64 |

Les scripts Lua et XML sont compilés dans les DLL (`include_str!`) : aucun autre fichier à installer.

L'updater lira `releases/latest/download/manifest.json` (une redirection, sans l'API GitHub ni sa
limite de requêtes), vérifiera la signature avec [public_key.txt](public_key.txt), puis téléchargera
chaque fichier depuis `releases/download/v<version>/` et contrôlera son SHA-256. Un proxy installé est
à jour si son SHA-256 est celui du manifeste : les DLL n'ont pas besoin de connaître leur numéro de
release.

`version` (le tag) et `lobby_version` sont distincts : une release qui ne touche ni la simulation ni
l'échange du salon garde la même `lobby_version` et reste compatible en multijoueur avec la précédente.

## Publier une release

1. Vérifier que tout est commité et poussé : la release ne compile que le contenu du tag.
2. Si la release change la simulation ou l'échange du salon, augmenter `VERSION` dans
   `proxy/src/version.rs`.
3. Taguer et pousser :
   ```
   git tag v0.2.0
   git push origin v0.2.0
   ```
4. Le workflow teste, compile, signe et crée une release **en brouillon**. La relire sur GitHub, puis
   la publier : l'updater ne voit que les releases publiées.

Lancé à la main (onglet Actions, *Run workflow*), le workflow compile et signe sans créer de release ;
les fichiers sont dans l'artefact du run.

## Clé de signature

- Clé publique : [public_key.txt](public_key.txt), dans le dépôt ; l'updater l'embarquera.
- Clé privée : hors du dépôt (`%USERPROFILE%\.bgd\signing_key.txt`), et dans le secret GitHub
  `BGD_SIGNING_KEY` :
  ```
  Get-Content "$env:USERPROFILE\.bgd\signing_key.txt" | gh secret set BGD_SIGNING_KEY
  ```
  À sauvegarder ailleurs (gestionnaire de mots de passe) : perdue, il faudrait publier un updater
  avec une nouvelle clé publique.

Vérifier une release téléchargée (dossier contenant les 4 fichiers) :
```
python release/manifest.py verify <dossier>/manifest.json
```
