//! BetterGameDLLs.exe : installe, met à jour, répare ou retire les proxys de BetterGameDLLs dans
//! Civilization VI, à partir des releases GitHub (voir updater/README.md).
//!
//!   BetterGameDLLs.exe                        fenêtre d'installation
//!   BetterGameDLLs.exe --game-dir <dossier>   idem, pour un jeu hors des bibliothèques Steam
//!   BetterGameDLLs.exe %command%              (options de lancement Steam) met à jour, puis lance le jeu
#![windows_subsystem = "windows"]

mod game;
mod gui;
mod install;
mod release;
mod win;

use std::path::{Path, PathBuf};
use std::process::Command;

use gui::{Icon, Page};
use install::{Action, State};

const INSTALL: i32 = 100;
const UNINSTALL: i32 = 101;
const COPY: i32 = 102;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        // Relance en administrateur par install::run.
        Some("--apply") if args.len() == 4 => install::apply_elevated(&args[1], Path::new(&args[2]), Path::new(&args[3])),
        Some("--game-dir") if args.len() == 2 => interactive(Some(PathBuf::from(&args[1]))),
        Some(arg) if !arg.starts_with("--") => launch(&args),
        _ => interactive(None),
    };
    std::process::exit(code);
}

fn interactive(game: Option<PathBuf>) -> i32 {
    match menu(game) {
        Ok(()) => 0,
        Err(e) => {
            gui::message(Icon::Error, "L'opération a échoué", &e);
            1
        }
    }
}

fn menu(game: Option<PathBuf>) -> Result<(), String> {
    let game = match game {
        Some(dir) if game::is_game_dir(&dir) => dir,
        Some(dir) => return Err(format!("{} n'est pas le dossier de Civilization VI (Gathering Storm).", dir.display())),
        None => game::find().ok_or("Civilization VI est introuvable dans les bibliothèques Steam.\n\nRelance l'installeur avec --game-dir \"<dossier du jeu>\".")?,
    };
    let work = release::work_dir();
    let manifest = release::fetch_manifest(&work);
    let states = install::status(&game, manifest.as_ref().ok());
    if let Some((name, _)) = states.iter().find(|(_, s)| *s == State::Missing) {
        return Err(format!("{name} est introuvable dans {} : vérifie les fichiers du jeu dans Steam.", game.display()));
    }
    let ours = states.iter().filter(|(_, s)| matches!(s, State::Installed | State::UpToDate)).count();
    let up_to_date = states.iter().all(|(_, s)| *s == State::UpToDate);

    let mut content = format!("Jeu : {}", game.display());
    let (icon, instruction, install_text) = match &manifest {
        Err(e) => {
            content += &format!("\n\nImpossible de lire la dernière version : {e}");
            (Icon::Warning, "Dernière version inconnue".to_string(), String::new())
        }
        Ok(m) if up_to_date => (Icon::Info, format!("BetterGameDLLs {} est installé et à jour", m.version), String::new()),
        Ok(m) if ours == 0 => (
            Icon::Info,
            "BetterGameDLLs n'est pas installé".to_string(),
            format!("Installer\nTélécharge et installe la version {}.", m.version),
        ),
        // Une partie seulement : Steam a remis un original (vérification des fichiers, mise à jour).
        Ok(m) if ours < states.len() => (
            Icon::Warning,
            "BetterGameDLLs doit être réparé".to_string(),
            format!("Réparer\nRéinstalle la version {}.", m.version),
        ),
        Ok(m) => (
            Icon::Info,
            format!("La version {} de BetterGameDLLs est disponible", m.version),
            format!("Mettre à jour\nInstalle la version {}.", m.version),
        ),
    };
    let details: Vec<String> = states
        .iter()
        .map(|(name, state)| {
            let text = match state {
                State::Missing => "introuvable",
                State::Original => "original du jeu",
                State::Installed => "proxy installé, autre version",
                State::UpToDate => "proxy à jour",
            };
            format!("{name} : {text}")
        })
        .collect();
    let mut buttons = Vec::new();
    if !install_text.is_empty() {
        buttons.push((INSTALL, install_text.as_str()));
    }
    if ours > 0 {
        buttons.push((UNINSTALL, "Désinstaller\nRemet les fichiers d'origine du jeu."));
    }
    let details = details.join("\n");
    let page = Page { icon, instruction: &instruction, content: &content, details: Some(&details), buttons: &buttons };
    let action = match gui::ask(&page, &|_| false) {
        INSTALL => Action::Install,
        UNINSTALL => Action::Uninstall,
        _ => return Ok(()),
    };
    if game::running() {
        return Err("Ferme Civilization VI, puis relance l'installeur.".into());
    }
    match action {
        Action::Install => {
            let m = manifest.unwrap();
            gui::progress(&format!("Installation de BetterGameDLLs {}…", m.version), || {
                release::fetch_files(&m, &work)?;
                install::run(action, &game, &work)
            })?;
            installed(&m.version)
        }
        Action::Uninstall => {
            gui::progress("Désinstallation de BetterGameDLLs…", || install::run(action, &game, &work))?;
            gui::message(Icon::Info, "BetterGameDLLs est désinstallé", "Le jeu a retrouvé ses fichiers d'origine.");
            Ok(())
        }
    }
}

/// Fin d'installation : propose l'option de lancement Steam qui garde le proxy à jour.
fn installed(version: &str) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let option = format!("\"{}\" %command%", exe.display());
    let content = format!(
        "Pour qu'il se mette à jour et se répare tout seul, mets BetterGameDLLs.exe dans un dossier où \
         il restera, puis ajoute cette option de lancement à Civilization VI dans Steam (Propriétés > \
         Général > Options de lancement) :\n\n{option}"
    );
    let instruction = format!("BetterGameDLLs {version} est installé");
    let buttons = [(COPY, "Copier l'option de lancement\nÀ coller dans les options de lancement de Steam.")];
    let page = Page { icon: Icon::Info, instruction: &instruction, content: &content, details: None, buttons: &buttons };
    gui::ask(&page, &|id| id == COPY && win::copy_to_clipboard(&option));
    Ok(())
}

/// Options de lancement Steam : met à jour ou répare avant de lancer le jeu, sans fenêtre si tout est
/// à jour. Un échec n'empêche pas de jouer.
fn launch(command: &[String]) -> i32 {
    if let Err(e) = update(Path::new(&command[0])) {
        gui::message(
            Icon::Warning,
            "BetterGameDLLs n'a pas pu se mettre à jour",
            &format!("{e}\n\nLe jeu va se lancer sans mise à jour."),
        );
    }
    // L'attente garde le jeu « en cours » pour Steam.
    match Command::new(&command[0]).args(&command[1..]).status() {
        Ok(status) => status.code().unwrap_or(0),
        Err(_) => 1,
    }
}

fn update(exe: &Path) -> Result<(), String> {
    let game = game::from_command(exe).or_else(game::find).ok_or("Civilization VI introuvable")?;
    let work = release::work_dir();
    let m = release::fetch_manifest(&work)?;
    if install::status(&game, Some(&m)).iter().all(|(_, s)| *s == State::UpToDate) {
        return Ok(());
    }
    gui::progress(&format!("Mise à jour de BetterGameDLLs vers la version {}…", m.version), || {
        release::fetch_files(&m, &work)?;
        install::run(Action::Install, &game, &work)
    })
}
