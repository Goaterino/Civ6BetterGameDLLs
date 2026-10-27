//! Proxy de GameCore_XP2_FinalRelease.dll pour Civilization VI (prototype).
//!
//! L'originale est renommée GameCore_XP2_FinalRelease_orig.dll ; ce proxy prend son nom, relaie ses
//! 157 exports (`forward`) et, au premier appel, la charge puis pose les détours de `coalesce`.
//!
//! Réglage : fichier `rollover_proxy.cfg` à côté de la DLL, ligne `mode=coalesce|count|off`
//! (coalesce par défaut) et ligne `ameliorations=off|count|verify|fast|quiet` (off par défaut). Journal : `%LOCALAPPDATA%\Civ6RolloverProxy\RolloverProxy.log`, hors du
//! dossier Logs du jeu.

#![allow(non_snake_case)] // le nom de la DLL produite, GameCore_XP2_FinalRelease, est imposé.

mod coalesce;
mod forward;
mod improvements;
mod patch;
mod win;

use std::ffi::c_void;
use std::io::Write;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub(crate) static SELF_MODULE: AtomicUsize = AtomicUsize::new(0);

/// Étapes franchies, pour les tests : voir `RolloverProxyStatus`.
pub(crate) static STATUS: AtomicU32 = AtomicU32::new(0);
pub(crate) const ST_DLLMAIN: u32 = 1;
pub(crate) const ST_ORIGINAL: u32 = 2;
pub(crate) const ST_HOOKED: u32 = 8;

/// Export de diagnostic : DllMain appelée (1), originale chargée (2), détours posés (8).
#[no_mangle]
pub extern "system" fn RolloverProxyStatus() -> u32 {
    STATUS.load(Ordering::Relaxed)
}

/// Export de test : charge l'originale et pose les détours sans appeler aucune fonction du jeu.
#[no_mangle]
pub extern "system" fn RolloverProxyLoadOriginal() -> usize {
    forward::real_module()
}

fn log_path() -> Option<std::path::PathBuf> {
    let dir = std::path::Path::new(&std::env::var_os("LOCALAPPDATA")?).join("Civ6RolloverProxy");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("RolloverProxy.log"))
}

pub(crate) fn log(msg: &str) {
    let Some(path) = log_path() else { return };
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{secs}] {msg}");
    }
}

pub(crate) fn self_dir() -> Option<std::path::PathBuf> {
    let mut buf = [0u16; 1024];
    let n = unsafe {
        win::GetModuleFileNameW(SELF_MODULE.load(Ordering::Relaxed) as win::HMODULE, buf.as_mut_ptr(), buf.len() as u32)
    } as usize;
    let path = std::path::PathBuf::from(String::from_utf16_lossy(&buf[..n]));
    path.parent().map(|p| p.to_path_buf())
}

pub(crate) fn read_config() {
    let Some(text) = self_dir().and_then(|d| std::fs::read_to_string(d.join("rollover_proxy.cfg")).ok()) else {
        return;
    };
    for line in text.lines() {
        if let Some(v) = line.trim().strip_prefix("mode=") {
            coalesce::set_mode(match v.trim() {
                "off" => coalesce::Mode::Off,
                "count" => coalesce::Mode::Count,
                _ => coalesce::Mode::Coalesce,
            });
        }
        if let Some(v) = line.trim().strip_prefix("ameliorations=") {
            improvements::set_mode(match v.trim() {
                "count" => improvements::Mode::Count,
                "verify" => improvements::Mode::Verify,
                "fast" => improvements::Mode::Fast,
                "quiet" => improvements::Mode::Quiet,
                _ => improvements::Mode::Off,
            });
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(module: win::HMODULE, reason: u32, reserved: *mut c_void) -> i32 {
    match reason {
        win::DLL_PROCESS_ATTACH => {
            SELF_MODULE.store(module as usize, Ordering::Relaxed);
            STATUS.fetch_or(ST_DLLMAIN, Ordering::Relaxed);
            win::DisableThreadLibraryCalls(module);
        }
        // Déchargement par FreeLibrary (reserved nul), pas fin du processus : l'originale reste
        // chargée, on y remet les octets d'origine pour qu'aucun saut ne vise ce proxy déchargé.
        win::DLL_PROCESS_DETACH if reserved.is_null() => {
            coalesce::on_proxy_unload();
            improvements::on_proxy_unload();
        }
        _ => {}
    }
    1
}
