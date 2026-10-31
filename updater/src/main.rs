//! BetterGameDLLs.exe : installe, met à jour, répare ou retire les proxys de BetterGameDLLs dans
//! Civilization VI, à partir des releases GitHub (voir updater/README.md).
//!
//!   BetterGameDLLs.exe                        menu
//!   BetterGameDLLs.exe --game-dir <dossier>   menu, pour un jeu hors des bibliothèques Steam
//!   BetterGameDLLs.exe %command%              (options de lancement Steam) met à jour, puis lance le jeu

mod game;
mod install;
mod release;
mod win;

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use install::{Action, State};

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
    println!("BetterGameDLLs, installeur {}\n", env!("CARGO_PKG_VERSION"));
    let code = match menu(game) {
        Ok(()) => 0,
        Err(e) => {
            println!("\nErreur : {e}");
            1
        }
    };
    print!("\nAppuie sur Entrée pour quitter.");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().lock().read_line(&mut String::new());
    code
}

fn menu(game: Option<PathBuf>) -> Result<(), String> {
    let game = match game {
        Some(dir) if game::is_game_dir(&dir) => dir,
        Some(dir) => return Err(format!("{} n'est pas le dossier de Civilization VI (Gathering Storm)", dir.display())),
        None => game::find().ok_or("Civilization VI introuvable. Relance avec --game-dir \"<dossier du jeu>\".")?,
    };
    println!("Jeu : {}", game.display());
    let work = release::work_dir();
    let manifest = match release::fetch_manifest(&work) {
        Ok(m) => {
            println!("Dernière version : {}\n", m.version);
            Some(m)
        }
        Err(e) => {
            println!("Dernière version inconnue ({e})\n");
            None
        }
    };
    let states = install::status(&game, manifest.as_ref());
    for (name, state) in &states {
        let text = match state {
            State::Missing => "introuvable",
            State::Original => "original du jeu",
            State::Installed => "proxy installé, autre version",
            State::UpToDate => "proxy à jour",
        };
        println!("  {name} : {text}");
    }
    let installed = states.iter().any(|(_, s)| matches!(s, State::Installed | State::UpToDate));
    let up_to_date = states.iter().all(|(_, s)| *s == State::UpToDate);

    println!();
    let can_install = manifest.is_some() && !up_to_date;
    if can_install {
        println!("  [1] {}", if installed { "Mettre à jour / réparer" } else { "Installer" });
    }
    if installed {
        println!("  [2] Désinstaller");
    }
    println!("  [Q] Quitter");
    print!("\nChoix : ");
    let _ = std::io::stdout().flush();
    let mut choice = String::new();
    let _ = std::io::stdin().lock().read_line(&mut choice);
    let action = match choice.trim() {
        "1" if can_install => Action::Install,
        "2" if installed => Action::Uninstall,
        _ => return Ok(()),
    };
    if game::running() {
        return Err("ferme Civilization VI d'abord".into());
    }
    match action {
        Action::Install => {
            let m = manifest.unwrap();
            release::fetch_files(&m, &work)?;
            install::run(action, &game, &work)?;
            println!("\nBetterGameDLLs {} installé.", m.version);
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            println!(
                "Pour les mises à jour et réparations automatiques, mets dans les options de lancement\n\
                 de Civilization VI (Steam > Propriétés > Général) :\n\n  \"{}\" %command%",
                exe.display()
            );
        }
        Action::Uninstall => {
            install::run(action, &game, &work)?;
            println!("\nBetterGameDLLs désinstallé : le jeu a retrouvé ses fichiers d'origine.");
        }
    }
    Ok(())
}

/// Options de lancement Steam : met à jour ou répare avant de lancer le jeu. Un échec n'empêche pas
/// de jouer.
fn launch(command: &[String]) -> i32 {
    if let Err(e) = update(Path::new(&command[0])) {
        println!("BetterGameDLLs : {e}\nLe jeu se lance sans mise à jour.");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    // Plus de fenêtre de console pendant la partie ; l'attente garde le jeu « en cours » pour Steam.
    unsafe { win::FreeConsole() };
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
    println!("BetterGameDLLs : installation de la version {}…", m.version);
    release::fetch_files(&m, &work)?;
    install::run(Action::Install, &game, &work)
}
