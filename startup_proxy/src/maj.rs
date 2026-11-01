//! Notification de mise à jour au menu principal (voir `maj.lua`).
//!
//! Au premier script Lua chargé par l'exe, bien avant le menu principal, un thread lit le manifeste
//! de la dernière release (`curl.exe` de Windows, sans fenêtre) et compare sa version à celle de ces
//! DLL (`version::RELEASE`). Rien n'est cherché pour une compilation locale, ni avec `verif_maj=off`
//! dans `rollover_proxy.cfg`. Le manifeste n'est pas vérifié ici : il ne sert qu'à prévenir, et
//! l'installeur vérifie sa signature avant d'installer quoi que ce soit.
//!
//! L'accroche `luaL_loadbuffer` d'`options` ajoute `maj.lua` à la fin de `MainMenu.lua`, après avoir
//! posé `Civ6Proxy_MiseAJour([action])` et `Civ6Proxy_Journal(texte)`.

use crate::log;
use crate::options::{hks, publish_natives, push_str, Natives};
use crate::version::RELEASE;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Mutex, Once};

pub const LUA_BLOCK: &str = include_str!("maj.lua");
pub const SCRIPT: &str = "mainmenu.lua";

const MANIFEST_URL: &str = "https://github.com/Goaterino/Civ6BetterGameDLLs/releases/latest/download/manifest.json";
const RELEASE_PAGE: &str = "https://github.com/Goaterino/Civ6BetterGameDLLs/releases/latest";
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

enum Check {
    Pending,
    /// Version plus récente que celle installée, pas encore proposée ou remise à plus tard.
    Newer(String),
    /// Rien à proposer : à jour, vérification désactivée ou en échec, ou joueur déjà sollicité.
    Done,
}

static CHECK: Mutex<Check> = Mutex::new(Check::Pending);
static START: Once = Once::new();

fn set(state: Check) {
    *CHECK.lock().unwrap() = state;
}

/// Lance la vérification, une fois. Appelée par l'accroche Lua : hors du verrou du chargeur.
pub fn start() {
    START.call_once(|| {
        let Some(current) = RELEASE else {
            log("maj : compilation locale, pas de vérification des mises à jour");
            return set(Check::Done);
        };
        if !enabled() {
            log("maj : vérification des mises à jour désactivée (verif_maj=off)");
            return set(Check::Done);
        }
        std::thread::spawn(move || match fetch_latest() {
            Ok(latest) if newer(&latest, current) => {
                log(&format!("maj : version {latest} disponible (installée : {current})"));
                set(Check::Newer(latest));
            }
            Ok(latest) => {
                log(&format!("maj : à jour ({current}, dernière : {latest})"));
                set(Check::Done);
            }
            Err(e) => {
                log(&format!("maj : vérification impossible : {e}"));
                set(Check::Done);
            }
        });
    });
}

/// `verif_maj=off` dans le fichier de réglages coupe la vérification ; la dernière ligne l'emporte.
fn enabled() -> bool {
    let text = crate::options::read_config().map(|(text, _)| text).unwrap_or_default();
    config_enabled(&text)
}

fn config_enabled(text: &str) -> bool {
    text.lines().filter_map(|l| l.trim().strip_prefix("verif_maj=")).last().is_none_or(|v| v.trim() != "off")
}

fn fetch_latest() -> Result<String, String> {
    let root = std::env::var_os("SystemRoot").unwrap_or("C:\\Windows".into());
    let out = Command::new(PathBuf::from(root).join("System32\\curl.exe"))
        .args(["-fsSL", "--max-time", "20", MANIFEST_URL])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("curl.exe : {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    parse_version(&String::from_utf8_lossy(&out.stdout)).ok_or_else(|| "manifeste sans version".into())
}

/// Le champ `"version"` du manifeste (release/manifest.py), sans analyseur JSON.
fn parse_version(json: &str) -> Option<String> {
    let rest = &json[json.find("\"version\"")? + 9..];
    let rest = rest.trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    Some(rest[..rest.find('"')?].to_string())
}

fn numbers(v: &str) -> Option<(u32, u32, u32)> {
    let mut it = v.split('.').map(|n| n.parse().ok());
    let r = (it.next()??, it.next()??, it.next()??);
    it.next().is_none().then_some(r)
}

fn newer(latest: &str, current: &str) -> bool {
    matches!((numbers(latest), numbers(current)), (Some(a), Some(b)) if a > b)
}

/// L'installeur se copie là à chaque installation (updater/src/main.rs).
fn installer() -> Option<PathBuf> {
    let path = PathBuf::from(std::env::var_os("LOCALAPPDATA")?).join("BetterGameDLLs\\BetterGameDLLs.exe");
    path.is_file().then_some(path)
}

/// Lance l'installeur, qui attendra la fermeture du jeu ; sinon ouvre la page de la release.
fn launch_installer() -> bool {
    if let Some(exe) = installer() {
        // Le dossier de l'exe : l'installeur y retrouve le jeu, même hors des bibliothèques Steam.
        let mut cmd = Command::new(&exe);
        cmd.arg("--after-game");
        if let Some(dir) = crate::self_dir() {
            cmd.arg(dir);
        }
        match cmd.spawn() {
            Ok(_) => {
                log(&format!("maj : installeur lancé ({})", exe.display()));
                return true;
            }
            Err(e) => log(&format!("maj : lancement de {} impossible : {e}", exe.display())),
        }
    }
    log("maj : installeur introuvable, page de la release ouverte");
    let _ = Command::new("explorer.exe").arg(RELEASE_PAGE).spawn();
    false
}

/// `Civ6Proxy_MiseAJour()` → `""` tant que la vérification tourne, la nouvelle version et la version
/// installée s'il y en a une, rien sinon. `Civ6Proxy_MiseAJour("plus_tard")` la retire jusqu'au
/// prochain lancement du jeu. `Civ6Proxy_MiseAJour("installer")` → 1 si l'installeur est lancé (le
/// bloc ferme alors le jeu), 0 si la page de la release a été ouverte à sa place.
unsafe extern "C-unwind" fn lua_update(l: *mut std::ffi::c_void) -> i32 {
    let Some(h) = hks() else { return 0 };
    let mut len = 0usize;
    let p = (h.to_lstring)(l, 1, &mut len);
    let action = if p.is_null() { "" } else { std::str::from_utf8(std::slice::from_raw_parts(p as *const u8, len)).unwrap_or("") };
    match action {
        "plus_tard" => {
            log("maj : remise à plus tard");
            set(Check::Done);
            0
        }
        "installer" => {
            set(Check::Done);
            (h.push_integer)(l, launch_installer() as i32);
            1
        }
        _ => match &*CHECK.lock().unwrap() {
            Check::Pending => {
                push_str(h, l, "");
                1
            }
            Check::Newer(latest) => {
                push_str(h, l, latest);
                push_str(h, l, RELEASE.unwrap_or(""));
                2
            }
            Check::Done => 0,
        },
    }
}

pub const NATIVES: Natives = &[(lua_update, b"Civ6Proxy_MiseAJour\0"), (crate::salon::lua_log, b"Civ6Proxy_Journal\0")];

pub const XML: &str = "MainMenu.xml";

/// MainMenu.xml avec l'instance du popup du menu, s'il ne l'a pas : celui de MPH inclut le modèle
/// `PopupDialog` sans l'instancier, et son Lua ne crée pas `g_PopupDialog`. On ajoute, comme le XML du
/// jeu, `<MakeInstance Name="PopupDialog" />` à la fin du contexte (et le modèle s'il manque) : le bloc
/// Lua crée alors son popup sur ces contrôles. `None` si le menu a déjà le sien.
pub fn patch_main_menu_xml(src: &[u8]) -> Result<Option<Vec<u8>>, String> {
    let text = std::str::from_utf8(src).map_err(|_| "texte non UTF-8".to_string())?;
    if text.contains(r#"<MakeInstance Name="PopupDialog""#) {
        return Ok(None);
    }
    let end = text.rfind("</ContextDefaults>").ok_or("fin du contexte introuvable")?;
    let mut out = String::with_capacity(text.len() + 100);
    out.push_str(&text[..end]);
    if !text.contains(r#"<Include File="PopupDialog""#) {
        out.push_str("  <Include File=\"PopupDialog\" />\r\n");
    }
    out.push_str("  <MakeInstance Name=\"PopupDialog\" />\r\n");
    out.push_str(&text[end..]);
    Ok(Some(out.into_bytes()))
}

/// MainMenu.lua suivi du bloc de la notification.
pub fn patch_main_menu_lua(src: &[u8]) -> Vec<u8> {
    let mut out = src.to_vec();
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(LUA_BLOCK.as_bytes());
    out
}

/// Appelée par l'accroche sur MainMenu.lua : pose les natives, puis rend le texte à compiler.
pub unsafe fn prepare(l: *mut std::ffi::c_void, src: &[u8]) -> Vec<u8> {
    publish_natives(l, NATIVES, "maj");
    patch_main_menu_lua(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_du_manifeste() {
        let json = "{\n  \"version\": \"0.1.2\",\n  \"lobby_version\": \"0.1.0\",\n  \"files\": []\n}\n";
        assert_eq!(parse_version(json).as_deref(), Some("0.1.2"));
        assert_eq!(parse_version("{}"), None);
    }

    #[test]
    fn comparaison() {
        assert!(newer("0.1.2", "0.1.1"));
        assert!(newer("0.10.0", "0.9.9"));
        assert!(newer("1.0.0", "0.99.99"));
        assert!(!newer("0.1.1", "0.1.1"));
        assert!(!newer("0.1.0", "0.1.1"));
        assert!(!newer("0.1", "0.0.1"));
        assert!(!newer("0.1.2", "0.0.0-dev"));
    }

    #[test]
    fn reglage() {
        assert!(config_enabled(""));
        assert!(config_enabled("actif=on\r\nverif_maj=on\r\n"));
        assert!(!config_enabled("verif_maj=off\r\n"));
        assert!(config_enabled("verif_maj=off\r\nverif_maj=on\r\n"));
    }

    #[test]
    fn popup_ajoute_au_menu_sans_popup() {
        let base = "<ContextDefaults>\r\n  <Include File=\"PopupDialog\" />\r\n  <MakeInstance Name=\"PopupDialog\" />\r\n</ContextDefaults>";
        assert_eq!(patch_main_menu_xml(base.as_bytes()), Ok(None));
        let mph = "<ContextDefaults>\r\n  <Include File=\"PopupDialog\" />\r\n  <Stack ID=\"A\" />\r\n</ContextDefaults>";
        let out = String::from_utf8(patch_main_menu_xml(mph.as_bytes()).unwrap().unwrap()).unwrap();
        assert!(out.ends_with("<Stack ID=\"A\" />\r\n  <MakeInstance Name=\"PopupDialog\" />\r\n</ContextDefaults>"), "{out}");
        assert_eq!(out.matches("<Include File=\"PopupDialog\"").count(), 1);
        let nu = "<ContextDefaults>\r\n</ContextDefaults>";
        let out = String::from_utf8(patch_main_menu_xml(nu.as_bytes()).unwrap().unwrap()).unwrap();
        assert!(out.contains("<Include File=\"PopupDialog\" />\r\n  <MakeInstance"), "{out}");
        assert!(patch_main_menu_xml(b"<Context>").is_err());
    }

    /// Les vrais fichiers du jeu et de MPH, s'ils sont là.
    #[test]
    fn vrais_menus() {
        let steam = r"C:\Program Files (x86)\Steam\steamapps";
        let base = format!(r"{steam}\common\Sid Meier's Civilization VI\Base\Assets\UI\FrontEnd\MainMenu.xml");
        if let Ok(src) = std::fs::read(&base) {
            assert_eq!(patch_main_menu_xml(&src), Ok(None), "le menu du jeu a déjà son popup");
        }
        let mph = format!(r"{steam}\workshop\content\289070\3307282026\ui\mainmenu.xml");
        if let Ok(src) = std::fs::read(&mph) {
            let out = String::from_utf8(patch_main_menu_xml(&src).unwrap().unwrap()).unwrap();
            assert!(out.trim_end().ends_with("<MakeInstance Name=\"PopupDialog\" />\r\n</ContextDefaults>"));
        }
    }

    #[test]
    fn bloc_ajoute() {
        let out = patch_main_menu_lua(b"Initialize();");
        assert!(out.starts_with(b"Initialize();\r\n-- "));
        for &(_, name) in NATIVES {
            let name = std::str::from_utf8(&name[..name.len() - 1]).unwrap();
            assert!(LUA_BLOCK.contains(name), "{name} posée mais pas utilisée par le bloc");
        }
    }
}
