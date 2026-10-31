//! Proxy de démarrage : DatabaseDLL_FinalRelease.dll pour Civilization VI (étape 0).
//!
//! L'exe importe statiquement cette DLL du jeu, posée à côté de lui dans `Base\Binaries\Win64Steam`.
//! Elle est donc chargée au lancement, avant le menu principal, alors que le proxy de GameCore ne
//! l'est qu'au lancement d'une partie. L'originale est renommée DatabaseDLL_FinalRelease_orig.dll ;
//! ses 5 exports sont renvoyés vers elle par le chargeur (voir build.rs).
//!
//! Cette étape ne fait que vérifier que le jeu charge le proxy : une ligne dans le journal commun,
//! `%LOCALAPPDATA%\Civ6RolloverProxy\RolloverProxy.log`.

#![allow(non_snake_case)] // le nom de la DLL produite, DatabaseDLL_FinalRelease, est imposé.

use std::ffi::c_void;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};

const DLL_PROCESS_ATTACH: u32 = 1;

#[link(name = "kernel32")]
extern "system" {
    fn DisableThreadLibraryCalls(module: *mut c_void) -> i32;
}

static STATUS: AtomicU32 = AtomicU32::new(0);

/// Export de diagnostic, et marque que `install.ps1` cherche dans le fichier : DllMain appelée (1).
#[no_mangle]
pub extern "system" fn StartupProxyStatus() -> u32 {
    STATUS.load(Ordering::Relaxed)
}

fn log(msg: &str) {
    let Some(dir) = std::env::var_os("LOCALAPPDATA").map(|d| std::path::Path::new(&d).join("Civ6RolloverProxy")) else {
        return;
    };
    let _ = std::fs::create_dir_all(&dir);
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("RolloverProxy.log")) {
        let _ = writeln!(f, "[{secs}] {msg}");
    }
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(module: *mut c_void, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        STATUS.fetch_or(1, Ordering::Relaxed);
        DisableThreadLibraryCalls(module);
        // Sous le verrou du chargeur : seulement des appels à kernel32, déjà initialisée.
        let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
        log(&format!("démarrage : proxy DatabaseDLL chargé dans {exe}"));
    }
    1
}
