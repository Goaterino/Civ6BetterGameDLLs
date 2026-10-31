//! Langues de l'installeur. Chaque texte est un tableau `Texts`, une entrée par langue de
//! `LANGUAGES`, dans le même ordre : ajouter une langue fait échouer la compilation de chaque texte
//! qui ne l'a pas encore. Le choix du joueur (boutons radio des fenêtres) est gardé dans
//! `%LOCALAPPDATA%\BetterGameDLLs\langue.txt` ; sans choix, la langue de Windows, sinon l'anglais.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Language {
    pub code: &'static str,
    /// Libellé du bouton radio, avec son drapeau.
    pub label: &'static str,
    /// Langue principale Windows (`PRIMARYLANGID`).
    primary: u16,
}

pub const LANGUAGES: [Language; 2] = [
    Language { code: "fr", label: "🇫🇷 Français", primary: 0x0C },
    Language { code: "en", label: "🇬🇧 English", primary: 0x09 },
];

const DEFAULT: usize = 1;

pub type Texts = [&'static str; LANGUAGES.len()];

static CURRENT: AtomicUsize = AtomicUsize::new(DEFAULT);

#[link(name = "kernel32")]
extern "system" {
    fn GetUserDefaultUILanguage() -> u16;
}

fn choice_file() -> Option<PathBuf> {
    Some(PathBuf::from(std::env::var_os("LOCALAPPDATA")?).join("BetterGameDLLs\\langue.txt"))
}

pub fn init() {
    let saved = choice_file().and_then(|p| std::fs::read_to_string(p).ok());
    let index = match saved {
        Some(code) => LANGUAGES.iter().position(|l| l.code == code.trim()),
        None => {
            let primary = unsafe { GetUserDefaultUILanguage() } & 0x3FF;
            LANGUAGES.iter().position(|l| l.primary == primary)
        }
    };
    CURRENT.store(index.unwrap_or(DEFAULT), Ordering::Relaxed);
}

pub fn current() -> usize {
    CURRENT.load(Ordering::Relaxed)
}

pub fn set(index: usize) {
    CURRENT.store(index, Ordering::Relaxed);
    if let Some(path) = choice_file() {
        let _ = std::fs::create_dir_all(path.parent().unwrap());
        let _ = std::fs::write(path, LANGUAGES[index].code);
    }
}

/// Le texte dans la langue choisie.
pub fn t(texts: Texts) -> &'static str {
    texts[current()]
}

/// Le texte dans la langue choisie, chaque `{}` remplacé par l'argument suivant.
pub fn f(texts: Texts, args: &[&dyn std::fmt::Display]) -> String {
    let mut out = String::new();
    let mut args = args.iter();
    let mut parts = t(texts).split("{}");
    out.push_str(parts.next().unwrap_or(""));
    for part in parts {
        if let Some(a) = args.next() {
            out.push_str(&a.to_string());
        }
        out.push_str(part);
    }
    out
}
