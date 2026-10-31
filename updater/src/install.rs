//! Pose et retrait des proxys, comme proxy/install.ps1 et startup_proxy/install.ps1 : l'original est
//! renommé `<nom>_orig.dll`, jamais supprimé, et le proxy prend son nom. `rollover_proxy.cfg` n'est
//! pas touché : sans lui, le proxy prend ses réglages par défaut.

use std::fs;
use std::path::{Path, PathBuf};

use crate::release::{self, Manifest};
use crate::win;

pub struct Target {
    pub name: &'static str,
    dest: &'static str,
    /// Chaîne présente dans le proxy et absente de l'original.
    marker: &'static [u8],
}

pub const TARGETS: [Target; 2] = [
    Target {
        name: "GameCore_XP2_FinalRelease.dll",
        dest: "DLC/Expansion2/Binaries/Win64",
        marker: b"RolloverProxyStatus",
    },
    Target {
        name: "DatabaseDLL_FinalRelease.dll",
        dest: "Base/Binaries/Win64Steam",
        marker: b"StartupProxyStatus",
    },
];

/// Fichier où le processus administrateur laisse son erreur.
const ERROR_FILE: &str = "erreur.txt";

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Missing,
    Original,
    /// Proxy posé, d'une autre version que la release (ou release inconnue).
    Installed,
    UpToDate,
}

#[derive(Clone, Copy)]
pub enum Action {
    Install,
    Uninstall,
}

impl Target {
    fn dll(&self, game: &Path) -> PathBuf {
        game.join(self.dest).join(self.name)
    }

    fn orig(&self, game: &Path) -> PathBuf {
        game.join(self.dest).join(self.name.replace(".dll", "_orig.dll"))
    }

    fn is_ours(&self, game: &Path) -> Result<bool, String> {
        let path = self.dll(game);
        let data = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(data.windows(self.marker.len()).any(|w| w == self.marker))
    }
}

pub fn status(game: &Path, m: Option<&Manifest>) -> Vec<(&'static str, State)> {
    TARGETS
        .iter()
        .map(|t| {
            let state = match t.is_ours(game) {
                Err(_) => State::Missing,
                Ok(false) => State::Original,
                Ok(true) => {
                    let expected = m.and_then(|m| m.files.iter().find(|f| f.name == t.name));
                    let current = release::sha256(&t.dll(game)).ok();
                    if expected.is_some_and(|f| Some(&f.sha256) == current.as_ref()) {
                        State::UpToDate
                    } else {
                        State::Installed
                    }
                }
            };
            (t.name, state)
        })
        .collect()
}

/// Installe la release téléchargée dans `work`, ou retire les proxys ; en administrateur si le
/// dossier du jeu n'est pas modifiable (Program Files).
pub fn run(action: Action, game: &Path, work: &Path) -> Result<(), String> {
    if writable(game) {
        return apply(action, game, work);
    }
    let _ = fs::remove_file(work.join(ERROR_FILE));
    let arg = match action {
        Action::Install => "install",
        Action::Uninstall => "uninstall",
    };
    let params = format!("--apply {arg} \"{}\" \"{}\"", work.display(), game.display());
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    match win::run_elevated(&exe, &params)? {
        0 => Ok(()),
        code => Err(fs::read_to_string(work.join(ERROR_FILE)).unwrap_or(format!("échec (code {code})"))),
    }
}

/// Point d'entrée du processus administrateur : son erreur va dans `work`.
pub fn apply_elevated(action: &str, work: &Path, game: &Path) -> i32 {
    let action = if action == "uninstall" { Action::Uninstall } else { Action::Install };
    match apply(action, game, work) {
        Ok(()) => 0,
        Err(e) => {
            let _ = fs::write(work.join(ERROR_FILE), e);
            1
        }
    }
}

fn apply(action: Action, game: &Path, work: &Path) -> Result<(), String> {
    match action {
        // Relue ici : en administrateur, on ne se fie pas à ce que le processus utilisateur a vérifié.
        Action::Install => install(game, work, &release::load(work)?),
        Action::Uninstall => uninstall(game),
    }
}

fn writable(game: &Path) -> bool {
    TARGETS.iter().all(|t| {
        let probe = game.join(t.dest).join(".bgd_test");
        let ok = fs::write(&probe, b"").is_ok();
        let _ = fs::remove_file(&probe);
        ok
    })
}

fn install(game: &Path, work: &Path, m: &Manifest) -> Result<(), String> {
    // La release doit poser exactement nos cibles, aux mêmes endroits.
    let known = |f: &release::File| TARGETS.iter().any(|t| t.name == f.name && t.dest == f.dest);
    if m.files.len() != TARGETS.len() || !m.files.iter().all(known) {
        return Err(format!("la release {} demande un installeur plus récent", m.version));
    }
    let mut ours = Vec::new();
    for t in &TARGETS {
        ours.push(t.is_ours(game)?);
    }
    for (t, ours) in TARGETS.iter().zip(ours) {
        let (dll, orig) = (t.dll(game), t.orig(game));
        if !ours {
            // L'original est en place. Un _orig déjà présent vient d'une installation écrasée par
            // Steam (vérification des fichiers, mise à jour) : l'original remis par Steam fait foi.
            if orig.exists() {
                fs::remove_file(&orig).map_err(|e| format!("{}: {e}", orig.display()))?;
            }
            fs::rename(&dll, &orig).map_err(|e| format!("{}: {e}", dll.display()))?;
        }
        let new = dll.with_extension("dll.new");
        fs::copy(work.join(t.name), &new).map_err(|e| format!("{}: {e}", new.display()))?;
        fs::rename(&new, &dll).map_err(|e| format!("{}: {e}", dll.display()))?;
    }
    Ok(())
}

fn uninstall(game: &Path) -> Result<(), String> {
    let mut ours = Vec::new();
    for t in &TARGETS {
        let is = t.is_ours(game)?;
        if is && !t.orig(game).exists() {
            return Err(format!("{} manque : vérifie les fichiers du jeu dans Steam", t.orig(game).display()));
        }
        ours.push(is);
    }
    for (t, _) in TARGETS.iter().zip(ours).filter(|(_, ours)| *ours) {
        let (dll, orig) = (t.dll(game), t.orig(game));
        fs::remove_file(&dll).map_err(|e| format!("{}: {e}", dll.display()))?;
        fs::rename(&orig, &dll).map_err(|e| format!("{}: {e}", orig.display()))?;
    }
    Ok(())
}
