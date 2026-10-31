//! Salon multijoueur : chaque joueur annonce sa version de BetterGameDLLs, et l'hôte établit si tous
//! les humains ont la même (voir `salon.lua`).
//!
//! L'accroche `luaL_loadbuffer` d'`options` ajoute `salon.lua` à la fin de `StagingRoom.lua`, ou de
//! son remplaçant par un mod (MPH en a un, du même nom), après avoir posé dans la table globale du
//! contexte trois fonctions natives : `Civ6Proxy_Version()`, `Civ6Proxy_Jeton()` et
//! `Civ6Proxy_Journal(texte)`.

use crate::options::{hks, publish_natives, push_str, Natives};
use crate::version::VERSION;
use std::sync::atomic::{AtomicU64, Ordering};

pub const LUA_BLOCK: &str = include_str!("salon.lua");
pub const SCRIPT: &str = "stagingroom.lua";

/// StagingRoom.lua suivi du bloc du salon. Octets laissés tels quels, comme pour Options.lua.
pub fn patch_staging_lua(src: &[u8]) -> Vec<u8> {
    let mut out = src.to_vec();
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(LUA_BLOCK.as_bytes());
    out
}

/// Jeton d'une ouverture du salon : l'heure en nanosecondes, l'identifiant du processus et un
/// compteur, en hexadécimal. Il suffit qu'il diffère d'une ouverture à l'autre et d'une machine à
/// l'autre ; ce n'est pas un secret.
pub fn new_token() -> String {
    static COUNT: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0);
    let n = COUNT.fetch_add(1, Ordering::Relaxed);
    format!("{nanos:x}{:x}{n:x}", std::process::id())
}

unsafe extern "C-unwind" fn lua_version(l: *mut std::ffi::c_void) -> i32 {
    let Some(h) = hks() else { return 0 };
    push_str(h, l, VERSION);
    1
}

unsafe extern "C-unwind" fn lua_token(l: *mut std::ffi::c_void) -> i32 {
    let Some(h) = hks() else { return 0 };
    push_str(h, l, &new_token());
    1
}

pub(crate) unsafe extern "C-unwind" fn lua_log(l: *mut std::ffi::c_void) -> i32 {
    let Some(h) = hks() else { return 0 };
    let mut len = 0usize;
    let p = (h.to_lstring)(l, 1, &mut len);
    if !p.is_null() {
        crate::log(&String::from_utf8_lossy(std::slice::from_raw_parts(p as *const u8, len)));
    }
    0
}

pub const NATIVES: Natives = &[
    (lua_version, b"Civ6Proxy_Version\0"),
    (lua_token, b"Civ6Proxy_Jeton\0"),
    (lua_log, b"Civ6Proxy_Journal\0"),
];

/// Appelée par l'accroche sur StagingRoom.lua : pose les natives, puis rend le texte à compiler.
pub unsafe fn prepare(l: *mut std::ffi::c_void, src: &[u8]) -> Vec<u8> {
    publish_natives(l, NATIVES, "salon");
    patch_staging_lua(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jetons_distincts() {
        let (a, b) = (new_token(), new_token());
        assert_ne!(a, b);
        assert!(a.bytes().all(|c| c.is_ascii_hexdigit()), "{a}");
    }

    #[test]
    fn bloc_ajoute() {
        let out = patch_staging_lua(b"Initialize();");
        assert!(out.starts_with(b"Initialize();\r\n-- "));
        assert!(LUA_BLOCK.contains("BGD - "));
        for &(_, name) in NATIVES {
            let name = std::str::from_utf8(&name[..name.len() - 1]).unwrap();
            assert!(LUA_BLOCK.contains(name), "{name} posée mais pas utilisée par le bloc");
        }
    }

    #[test]
    fn version_lisible() {
        // Annonce « <version>@<jeton>#<jeton> », verdict « <version>@<jeton>:<1|0>:<jetons> » : la
        // version ne doit contenir aucun de ces séparateurs.
        assert!(!VERSION.is_empty() && !VERSION.contains(['@', '#', ':', ',']), "{VERSION}");
    }
}
