//! Boutons d'attaque des bannières (voir ui_strike/README.md) : patch de l'interface au
//! moment où l'exe lit ses fichiers, sans mod. Rien ne part sur le réseau, chaque joueur qui a le
//! proxy change son propre affichage.
//!
//! - **Lua.** L'exe charge chaque script par `140dc30e0`, seul appelant de `luaL_loadbuffer`
//!   (HavokScript_FinalRelease.dll). On remplace l'entrée de cet import dans la table d'imports
//!   de l'exe : pour `CityBannerManager.lua`, quel que soit le mod qui le fournit, on compile un
//!   texte modifié qui appartient au proxy (luaL_loadbuffer ne garde pas le tampon).
//! - **XML.** `140a2d280` (DX12 ; `140a1f3f0` en DX11, retrouvée par la même signature) lit un
//!   fichier XML entier dans un tampon du jeu `{ptr, len, cap, allocateur}`, NUL final compris.
//!   Pour `CityBannerInstances.xml`, on réécrit ce tampon après la lecture, en l'agrandissant par
//!   l'allocateur du jeu, qui le libérera.
//!
//! Réglages dans rollover_proxy.cfg :
//! - `ui_boutons=on|off` (on) : boutons au-dessus des drapeaux d'unités (`ui_boutons.lua` et
//!   `ui_boutons.xml`), et `ConsumeMouse` sur les boutons nucléaires ;
//! - `ui_recentrage=on|off` (off) : off retire le recentrage de la caméra au clic sur ces boutons,
//!   en passant `false` en second argument de `UI.SelectCity` / `UI.SelectDistrict` ;
//! - `ui_journal=on|off` (off) : note le nom de chaque script et de chaque XML chargés ;
//! - `ui_mesure_epingles=on|off` (off) : ajoute `ui_epingles.lua` à la fin des scripts que
//!   recharge chaque `PlayerInfoChanged` (épingles, liste des épingles, bandeau des dirigeants, DMT).
//!   Les durées vont dans Lua.log (voir map_pins/README.md) ;
//! - `ui_epingles=on|off` (on) : ajoute `ui_epingles_opti.lua` à MapPinManager.lua et
//!   MapPinListPanel.lua. Plus de reconstruction quand aucune épingle n'a changé, une seule lecture
//!   de GetMapPins par joueur et par reconstruction, liste fermée reconstruite à son ouverture ;
//! - `ui_merveilles=on|off` (on) : ajoute `ui_merveilles.lua` à ProductionHelper.lua. Une merveille
//!   retirée de la file par un échange (wonder_queue/README.md) est remise en tête par des commandes
//!   du jeu, rejouées à l'identique par tous les joueurs, proxy ou non.
//!
//! Dès que l'accroche Lua est posée, trois défauts du jeu sont aussi corrigés (`FIXES`) : le bouton
//! d'attaque d'une ville non sélectionnée demandait deux clics (`patch_city_panel`), un clic sur le
//! drapeau d'une unité ne tirait pas en visée d'une ville ou d'un quartier (`patch_unit_flags`), et
//! la production en cours du panneau de production ne se montrait pas en passant d'une ville sans
//! production à une ville qui en a une (`patch_production_panel`).

use crate::{log, patch, win};
use std::ffi::{c_char, c_void, CStr};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const LUA_BLOCK: &str = include_str!("ui_boutons.lua");
const XML_BLOCK: &str = include_str!("ui_boutons.xml");
const MESURE_BLOCK: &str = include_str!("ui_epingles.lua");
const OPTI_BLOCK: &str = include_str!("ui_epingles_opti.lua");
const MERVEILLES_BLOCK: &str = include_str!("ui_merveilles.lua");

static BOUTONS: AtomicBool = AtomicBool::new(true);
static RECENTRAGE: AtomicBool = AtomicBool::new(false);
static JOURNAL: AtomicBool = AtomicBool::new(false);
static MESURE: AtomicBool = AtomicBool::new(false);
static OPTI: AtomicBool = AtomicBool::new(true);
static MERVEILLES: AtomicBool = AtomicBool::new(true);
/// Les calques sont définis dans le dernier CityBannerInstances.xml lu : le bloc Lua, qui s'en
/// sert, n'est ajouté qu'à cette condition.
static XML_PATCHED: AtomicBool = AtomicBool::new(false);

static IAT_SLOT: AtomicUsize = AtomicUsize::new(0);
static REAL_LOADBUFFER: AtomicUsize = AtomicUsize::new(0);
static XML_SITE: AtomicUsize = AtomicUsize::new(0);
static XML_TRAMP: AtomicUsize = AtomicUsize::new(0);

pub fn set_boutons(on: bool) {
    BOUTONS.store(on, Ordering::Relaxed);
}

pub fn set_recentrage(on: bool) {
    RECENTRAGE.store(on, Ordering::Relaxed);
}

pub fn set_journal(on: bool) {
    JOURNAL.store(on, Ordering::Relaxed);
}

pub fn set_mesure_epingles(on: bool) {
    MESURE.store(on, Ordering::Relaxed);
}

pub fn set_epingles(on: bool) {
    OPTI.store(on, Ordering::Relaxed);
}

pub fn set_merveilles(on: bool) {
    MERVEILLES.store(on, Ordering::Relaxed);
}

fn active() -> bool {
    BOUTONS.load(Ordering::Relaxed)
        || !RECENTRAGE.load(Ordering::Relaxed)
        || JOURNAL.load(Ordering::Relaxed)
        || MESURE.load(Ordering::Relaxed)
        || OPTI.load(Ordering::Relaxed)
        || MERVEILLES.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------------------------
// Remplacements de texte, sans état : testés sur les vrais fichiers (voir les tests en bas).

/// Le chemin finit-il par `/nom` ou `\nom` (ou est-il `nom`), sans tenir compte de la casse ?
fn is_file(path: &str, name: &str) -> bool {
    let (p, n) = (path.to_ascii_lowercase(), name.to_ascii_lowercase());
    p == n || p.ends_with(&format!("/{n}")) || p.ends_with(&format!("\\{n}"))
}

/// Les fonctions de clic et l'appel qui sélectionne, donc recentre.
const CLICKS: [(&str, &str); 3] = [
    ("OnCityStrikeButtonClick", "UI.SelectCity("),
    ("OnDistrictRangeStrikeButtonClick", "UI.SelectDistrict("),
    ("OnICBMStrikeButtonClick", "UI.SelectCity("),
];

/// Ajoute `false` en second argument du premier `call` du corps de la fonction `func`.
fn no_recenter(text: &mut String, func: &str, call: &str) -> Result<(), String> {
    let header = format!("function {func}");
    let start = text
        .match_indices(&header)
        .map(|(i, _)| i)
        .find(|&i| text[i + header.len()..].trim_start().starts_with('('))
        .ok_or_else(|| format!("{func} introuvable"))?;
    let body_end = text[start + header.len()..].find("\nfunction ").map_or(text.len(), |i| start + header.len() + i);
    let at = text[start..body_end].find(call).ok_or_else(|| format!("{func} : {call}…) introuvable"))? + start + call.len();
    let close = text[at..body_end].find(')').ok_or_else(|| format!("{func} : parenthèse fermante introuvable"))? + at;
    let args = text[at..close].trim();
    if args.is_empty() || args.contains(',') {
        return Err(format!("{func} : arguments inattendus ({args})"));
    }
    let new = format!(" {args}, false ");
    text.replace_range(at..close, &new);
    Ok(())
}

/// Texte modifié de CityBannerManager.lua, et ce qui a été fait ou pas, pour le journal.
pub fn patch_lua(src: &[u8], boutons: bool, recentrage: bool) -> (Option<Vec<u8>>, Vec<String>) {
    let mut notes = Vec::new();
    let Ok(text) = std::str::from_utf8(src) else {
        return (None, vec!["texte non UTF-8, laissé tel quel".into()]);
    };
    let mut text = text.to_owned();
    let mut changed = false;
    if !recentrage {
        for (func, call) in CLICKS {
            match no_recenter(&mut text, func, call) {
                Ok(()) => changed = true,
                Err(e) => notes.push(format!("recentrage gardé : {e}")),
            }
        }
    }
    if boutons {
        text.push_str("\r\n");
        text.push_str(LUA_BLOCK);
        changed = true;
    }
    (changed.then(|| text.into_bytes()), notes)
}

/// Corrige la faute de `CityPanel.lua` (OnCitySelectionChanged) qui coûte un second clic au bouton
/// d'attaque d'une ville non sélectionnée. Le clic sélectionne la ville puis passe en
/// CITY_RANGE_ATTACK ; l'événement CitySelectionChanged arrive ensuite et doit garder ce mode si la
/// ville peut tirer, mais teste `CityManager.GetCity(owningPlayerID, cityID)`, alors que le
/// paramètre s'appelle `ownerPlayerID`. Avec un joueur nil, le test échoue toujours et l'interface
/// repasse en SELECTION.
pub fn patch_city_panel(src: &[u8]) -> Result<Vec<u8>, String> {
    const HEADER: &str = "function OnCitySelectionChanged( ownerPlayerID";
    const WRONG: &str = "CityManager.GetCity(owningPlayerID, cityID)";
    let text = std::str::from_utf8(src).map_err(|_| "texte non UTF-8".to_string())?;
    let start = text.find(HEADER).ok_or("OnCitySelectionChanged( ownerPlayerID… introuvable")?;
    let end = text[start + HEADER.len()..].find("\nfunction ").map_or(text.len(), |i| start + HEADER.len() + i);
    let body = &text[start..end];
    if body.contains("local owningPlayerID") {
        return Err("owningPlayerID est défini dans la fonction".into());
    }
    let at = body.find(WRONG).ok_or("appel fautif introuvable (déjà corrigé ?)")? + start;
    let mut out = text.to_owned();
    out.replace_range(at..at + WRONG.len(), "CityManager.GetCity(ownerPlayerID, cityID)");
    Ok(out.into_bytes())
}

const FLAG_BLOCK: &str = include_str!("ui_drapeaux.lua");

/// En visée d'une ville ou d'un quartier, un clic gauche sur le drapeau d'une unité étrangère ne
/// tirait pas : `OnUnitFlagClick` ne sait faire attaquer que l'unité sélectionnée, et le drapeau
/// prend le clic avant WorldInput. Les deux clics gauches des drapeaux passent par
/// `Civ6RolloverProxy_OnUnitFlagLClick` (ui_drapeaux.lua), inséré avant `OnUnitFlagClick` : il
/// tire sur la case de l'unité, sinon il appelle `OnUnitFlagClick`. Le clic droit ne change pas.
///
/// Travaille sur les octets : le fichier du jeu n'est pas en UTF-8 (celui de BSM l'est).
pub fn patch_unit_flags(src: &[u8]) -> Result<Vec<u8>, String> {
    const CLICK: &[u8] = b"RegisterCallback( Mouse.eLClick, OnUnitFlagClick )";
    const NEW_CLICK: &[u8] = b"RegisterCallback( Mouse.eLClick, Civ6RolloverProxy_OnUnitFlagLClick )";
    const HEADER: &[u8] = b"\nfunction OnUnitFlagClick(";
    let find_from = |from: usize, pat: &[u8]| src[from..].windows(pat.len()).position(|w| w == pat).map(|i| from + i);
    let mut clicks = Vec::new();
    while let Some(i) = find_from(clicks.last().map_or(0, |&i: &usize| i + CLICK.len()), CLICK) {
        clicks.push(i);
    }
    if clicks.len() != 2 {
        return Err(format!("{} clics gauches branchés sur OnUnitFlagClick au lieu de 2", clicks.len()));
    }
    let header = find_from(0, HEADER).ok_or("OnUnitFlagClick introuvable")? + 1;
    // (position, texte inséré, octets remplacés), dans l'ordre du fichier.
    let mut edits = vec![(header, FLAG_BLOCK.as_bytes(), 0)];
    edits.extend(clicks.iter().map(|&c| (c, NEW_CLICK, CLICK.len())));
    edits.sort_by_key(|e| e.0);
    let mut out = Vec::with_capacity(src.len() + FLAG_BLOCK.len() + 64);
    let mut pos = 0;
    for (at, insert, skip) in edits {
        out.extend_from_slice(&src[pos..at]);
        out.extend_from_slice(insert);
        pos = at + skip;
    }
    out.extend_from_slice(&src[pos..]);
    Ok(out)
}

const PRODUCTION_BLOCK: &str = include_str!("ui_production.lua");

/// Les variables locales de ProductionPanel.lua que lit `ui_production.lua`, et la fonction qu'il
/// enveloppe.
const PRODUCTION_NAMES: [&str; 5] =
    ["local LISTMODE", "local m_isQueueOpen", "local m_hasProductionToShow", "local m_CurrentListMode", "\nfunction View("];

/// En changeant de ville, la production en cours du panneau gardait la visibilité de la ville
/// précédente : cachée après une ville sans production. `ui_production.lua`, ajouté à la fin du
/// fichier pour lire ses variables locales, enveloppe `View` et applique la règle des onglets.
/// Octets laissés tels quels, comme pour les drapeaux.
pub fn patch_production_panel(src: &[u8]) -> Result<Vec<u8>, String> {
    let has = |pat: &str| src.windows(pat.len()).any(|w| w == pat.as_bytes());
    if let Some(missing) = PRODUCTION_NAMES.iter().find(|n| !has(n)) {
        return Err(format!("{} introuvable", missing.trim_start()));
    }
    if has("Civ6RolloverProxy (src/ui_production.lua)") {
        return Err("déjà corrigé".into());
    }
    let mut out = src.to_vec();
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(PRODUCTION_BLOCK.as_bytes());
    Ok(out)
}

/// Nom du fichier d'un script, en minuscules.
fn script_name(chunk: &str) -> String {
    chunk.rsplit(['/', '\\']).next().unwrap_or("").trim_start_matches('@').to_ascii_lowercase()
}

/// Scripts que `ui_epingles_opti.lua` allège : les drapeaux des épingles et leur liste.
const OPTIMIZED: [&str; 4] = ["mappinmanager.lua", "mappinmanager_dmt.lua", "mappinmanager_cqui.lua", "mappinlistpanel.lua"];

/// Nom du script, en minuscules, s'il fait partie de ceux que mesure `ui_epingles.lua` : ceux qui
/// reconstruisent quelque chose à chaque `PlayerInfoChanged`, et le calcul des rendements de DMT.
fn measured(chunk: &str) -> Option<String> {
    let name = script_name(chunk);
    // DiplomacyRibbonSupport.lua est une bibliothèque incluse ailleurs (LeaderIcon.lua), pas le bandeau.
    let ribbon = name == "diplomacyribbon.lua" || (name.starts_with("diplomacyribbon_") && name.ends_with(".lua"));
    (OPTIMIZED.contains(&name.as_str()) || name == "dmt_yieldcalculator.lua" || ribbon).then_some(name)
}

/// HavokScript : `os.clock` n'y avance que par secondes entières. Le bloc de mesure lit donc
/// `Civ6RolloverProxy_Clock()` (secondes, résolution QueryPerformanceCounter), posée dans la table
/// globale (`LUA_GLOBALSINDEX`, -10002, celle où l'exe pose `PlayerConfiguration`) avant chaque
/// script mesuré. La pose passe par `lua_cpcall` : si la table refuse l'écriture, l'erreur reste
/// dans Lua (d'où `C-unwind`) et le bloc se rabat sur `os.clock`.
const CLOCK_NAME: &[u8] = b"Civ6RolloverProxy_Clock\0";
const LUA_GLOBALSINDEX: i32 = -10002;

type LuaCFn = unsafe extern "C-unwind" fn(*mut c_void) -> i32;

struct Hks {
    push_number: unsafe extern "C-unwind" fn(*mut c_void, f64),
    push_closure: unsafe extern "C-unwind" fn(*mut c_void, LuaCFn, i32, *const c_char, i32),
    set_field: unsafe extern "C-unwind" fn(*mut c_void, i32, *const c_char),
    cpcall: unsafe extern "C-unwind" fn(*mut c_void, LuaCFn, *mut c_void) -> i32,
    set_top: unsafe extern "C-unwind" fn(*mut c_void, i32),
}

static HKS: std::sync::OnceLock<Option<Hks>> = std::sync::OnceLock::new();
static CLOCK_LOGGED: AtomicBool = AtomicBool::new(false);

unsafe fn hks() -> Option<&'static Hks> {
    HKS.get_or_init(|| {
        let m = win::GetModuleHandleW(win::wide("HavokScript_FinalRelease.dll").as_ptr());
        if m.is_null() {
            return None;
        }
        let get = |name: &[u8]| {
            let p = win::GetProcAddress(m, name.as_ptr());
            (!p.is_null()).then_some(p)
        };
        Some(Hks {
            push_number: std::mem::transmute(get(b"?lua_pushnumber@@YAXPEAUlua_State@@N@Z\0")?),
            push_closure: std::mem::transmute(get(b"?hks_pushnamedcclosure@@YAXPEAUlua_State@@P6AH0@ZHPEBDH@Z\0")?),
            set_field: std::mem::transmute(get(b"?lua_setfield@@YAXPEAUlua_State@@HPEBD@Z\0")?),
            cpcall: std::mem::transmute(get(b"?lua_cpcall@@YAHPEAUlua_State@@P6AH0@ZPEAX@Z\0")?),
            set_top: std::mem::transmute(get(b"?lua_settop@@YAXPEAUlua_State@@H@Z\0")?),
        })
    })
    .as_ref()
}

unsafe extern "C-unwind" fn lua_clock(l: *mut c_void) -> i32 {
    if let Some(h) = hks() {
        (h.push_number)(l, win::qpc_ms(win::qpc()) / 1000.0);
        return 1;
    }
    0
}

unsafe extern "C-unwind" fn install_clock(l: *mut c_void) -> i32 {
    if let Some(h) = hks() {
        (h.push_closure)(l, lua_clock, 0, CLOCK_NAME.as_ptr() as *const c_char, 0);
        (h.set_field)(l, LUA_GLOBALSINDEX, CLOCK_NAME.as_ptr() as *const c_char);
    }
    0
}

unsafe fn publish_clock(l: *mut c_void) {
    let Some(h) = hks() else {
        if !CLOCK_LOGGED.swap(true, Ordering::Relaxed) {
            log("ui : fonctions HavokScript introuvables, mesure des épingles avec os.clock (à la seconde)");
        }
        return;
    };
    let r = (h.cpcall)(l, install_clock, std::ptr::null_mut());
    if r != 0 {
        // lua_cpcall laisse l'erreur sur la pile.
        (h.set_top)(l, -2);
    }
    if !CLOCK_LOGGED.swap(true, Ordering::Relaxed) {
        log(&if r == 0 { "ui : horloge Civ6RolloverProxy_Clock posée".to_string() } else { format!("ui : horloge non posée ({r}), mesure avec os.clock") });
    }
}

/// Le script suivi du bloc d'optimisation (`opti`), puis du bloc de mesure (`mesure`), chacun
/// précédé de son contexte (`EpinglesOpti_Ctx`, `MesureEpingles_Ctx`). La mesure vient en dernier :
/// elle enveloppe les fonctions déjà optimisées. Octets laissés tels quels : les fichiers du jeu ne
/// sont pas tous en UTF-8.
pub fn patch_epingles(src: &[u8], ctx: &str, opti: bool, mesure: bool) -> Vec<u8> {
    let mut out = src.to_vec();
    if opti {
        out.extend_from_slice(format!("\r\nlocal EpinglesOpti_Ctx = \"{ctx}\";\r\n").as_bytes());
        out.extend_from_slice(OPTI_BLOCK.as_bytes());
    }
    if mesure {
        out.extend_from_slice(format!("\r\nlocal MesureEpingles_Ctx = \"{ctx}\";\r\n").as_bytes());
        out.extend_from_slice(MESURE_BLOCK.as_bytes());
    }
    out
}

/// ProductionHelper.lua suivi de `ui_merveilles.lua`.
pub fn patch_merveilles(src: &[u8]) -> Vec<u8> {
    let mut out = src.to_vec();
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(MERVEILLES_BLOCK.as_bytes());
    out
}

type Fix = fn(&[u8]) -> Result<Vec<u8>, String>;

/// Corrections appliquées dès que l'accroche Lua est posée : fichier, correction, journal.
const FIXES: [(&str, Fix, &str); 3] = [
    ("CityPanel.lua", patch_city_panel, "owningPlayerID corrigé"),
    ("UnitFlagManager.lua", patch_unit_flags, "clic sur un drapeau en visée d'une ville ou d'un quartier"),
    ("ProductionPanel.lua", patch_production_panel, "production en cours affichée au changement de ville"),
];

/// Les cadres que les calques de ui_boutons.xml reproduisent : sans eux, les boutons déplacés ne
/// seraient plus à leur place, et on ne patche rien.
const FRAMES: [&str; 6] = [
    r#"<ZoomAnchor ID="Anchor" ZoomOffsetNear="0,0,0" ZoomOffsetFar="0,0,-20">"#,
    r#"<Container ID="Container" Anchor="C,C" Offset="0,-20" Size="200,41">"#,
    r#"<Container ID="EncampmentBannerContainer" Anchor="C,C" Size="106,34">"#,
    r#"<Container ID="WMDBannerContainer" Anchor="C,C" Size="88,34">"#,
    r#"<Container Anchor="L,C" Offset="25,0">"#,
    r#"<Container Anchor="R,C" Offset="25,0">"#,
];

/// Texte modifié de CityBannerInstances.xml : calques ajoutés, `ConsumeMouse` sur les boutons
/// nucléaires (le bouton d'attaque l'a déjà).
pub fn patch_xml(src: &[u8]) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(src).map_err(|_| "texte non UTF-8".to_string())?;
    if let Some(missing) = FRAMES.iter().find(|f| !text.contains(*f)) {
        return Err(format!("cadre introuvable : {missing}"));
    }
    let end = text.rfind("</Include>").ok_or("balise </Include> introuvable")?;
    let mut out = String::with_capacity(text.len() + XML_BLOCK.len() + 64);
    out.push_str(&text[..end]);
    out.push_str(XML_BLOCK);
    out.push_str(&text[end..]);
    for id in ["NukeBombButton", "ThermoNukeBombButton"] {
        let open = format!(r#"<Button ID="{id}""#);
        let at = out.find(&open).ok_or_else(|| format!("{id} introuvable"))?;
        let tag_end = out[at..].find('>').ok_or_else(|| format!("{id} : balise non fermée"))? + at;
        if !out[at..tag_end].contains("ConsumeMouse") {
            out.insert_str(at + open.len(), r#" ConsumeMouse="1""#);
        }
    }
    Ok(out.into_bytes())
}

// ---------------------------------------------------------------------------------------------
// Accroches.

type LoadBuffer = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const c_char) -> i32;

unsafe extern "C" fn hook_loadbuffer(l: *mut c_void, buf: *const u8, size: usize, name: *const c_char) -> i32 {
    let real: LoadBuffer = std::mem::transmute(REAL_LOADBUFFER.load(Ordering::Relaxed));
    if name.is_null() || buf.is_null() {
        return real(l, buf, size, name);
    }
    let chunk = CStr::from_ptr(name).to_string_lossy();
    if JOURNAL.load(Ordering::Relaxed) {
        log(&format!("ui : script {chunk} ({size} octets)"));
    }
    let ctx = script_name(&chunk);
    let opti = OPTI.load(Ordering::Relaxed) && OPTIMIZED.contains(&ctx.as_str());
    let mesure = MESURE.load(Ordering::Relaxed) && measured(&chunk).is_some();
    if opti || mesure {
        if mesure {
            publish_clock(l);
        }
        let text = patch_epingles(std::slice::from_raw_parts(buf, size), &ctx, opti, mesure);
        let r = real(l, text.as_ptr(), text.len(), name);
        let what = match (opti, mesure) {
            (true, true) => "épingles allégées et mesurées",
            (true, false) => "épingles allégées",
            _ => "mesure des épingles ajoutée",
        };
        log(&format!("ui : {chunk} : {what}, compilation {}", if r == 0 { "réussie".to_string() } else { format!("en échec ({r})") }));
        return r;
    }
    if MERVEILLES.load(Ordering::Relaxed) && ctx == "productionhelper.lua" {
        let text = patch_merveilles(std::slice::from_raw_parts(buf, size));
        let r = real(l, text.as_ptr(), text.len(), name);
        log(&format!(
            "ui : {chunk} : rattrapage des merveilles ajouté, compilation {}",
            if r == 0 { "réussie".to_string() } else { format!("en échec ({r})") }
        ));
        return r;
    }
    for (file, patch, what) in FIXES {
        if !is_file(&chunk, file) {
            continue;
        }
        return match patch(std::slice::from_raw_parts(buf, size)) {
            Ok(text) => {
                let r = real(l, text.as_ptr(), text.len(), name);
                log(&format!(
                    "ui : {chunk} : {what}, compilation {}",
                    if r == 0 { "réussie".to_string() } else { format!("en échec ({r})") }
                ));
                r
            }
            Err(e) => {
                log(&format!("ui : {chunk} laissé tel quel : {e}"));
                real(l, buf, size, name)
            }
        };
    }
    if !is_file(&chunk, "CityBannerManager.lua") {
        return real(l, buf, size, name);
    }
    let boutons = BOUTONS.load(Ordering::Relaxed) && XML_PATCHED.load(Ordering::Relaxed);
    if BOUTONS.load(Ordering::Relaxed) && !boutons {
        log("ui : calques absents de CityBannerInstances.xml, boutons laissés dans les bannières");
    }
    let (new, notes) = patch_lua(std::slice::from_raw_parts(buf, size), boutons, RECENTRAGE.load(Ordering::Relaxed));
    for n in &notes {
        log(&format!("ui : {chunk} : {n}"));
    }
    match new {
        Some(text) => {
            let r = real(l, text.as_ptr(), text.len(), name);
            log(&format!(
                "ui : {chunk} modifié (boutons {}, recentrage {}), compilation {}",
                if boutons { "au-dessus" } else { "inchangés" },
                if RECENTRAGE.load(Ordering::Relaxed) { "gardé" } else { "retiré" },
                if r == 0 { "réussie".to_string() } else { format!("en échec ({r})") }
            ));
            r
        }
        None => real(l, buf, size, name),
    }
}

/// Tampon du jeu rempli par la lecture d'un fichier (voir `1409971c0`, qui le réserve).
#[repr(C)]
struct GameBuf {
    ptr: *mut u8,
    len: usize,
    cap: usize,
    /// Objet allocateur : table virtuelle `alloc(this, taille, alignement)`, `free(this, ptr)`.
    alloc: *const *const usize,
}

/// Remplace le contenu du tampon, NUL final compris.
unsafe fn replace_buffer(b: &mut GameBuf, data: &[u8]) -> bool {
    let need = data.len() + 1;
    if need > b.cap {
        if b.alloc.is_null() {
            return false;
        }
        let vt = *b.alloc;
        let alloc: unsafe extern "C" fn(*const c_void, usize, usize) -> *mut u8 = std::mem::transmute(*vt);
        let free: unsafe extern "C" fn(*const c_void, *mut u8) = std::mem::transmute(*vt.add(1));
        let p = alloc(b.alloc as *const c_void, need, 0x10);
        if p.is_null() {
            return false;
        }
        if !b.ptr.is_null() {
            free(b.alloc as *const c_void, b.ptr);
        }
        b.ptr = p;
        b.cap = need;
    }
    std::ptr::copy_nonoverlapping(data.as_ptr(), b.ptr, data.len());
    *b.ptr.add(data.len()) = 0;
    b.len = need;
    true
}

type ReadXml = unsafe extern "C" fn(*const u16, *mut GameBuf) -> i32;

unsafe extern "C" fn hook_read_xml(path: *const u16, buf: *mut GameBuf) -> i32 {
    let real: ReadXml = std::mem::transmute(XML_TRAMP.load(Ordering::Relaxed));
    let r = real(path, buf);
    if r != 0 || path.is_null() || buf.is_null() {
        return r;
    }
    let n = (0..).take_while(|&i| *path.add(i) != 0).count();
    let name = String::from_utf16_lossy(std::slice::from_raw_parts(path, n));
    if JOURNAL.load(Ordering::Relaxed) {
        log(&format!("ui : XML {name}"));
    }
    if !BOUTONS.load(Ordering::Relaxed) || !is_file(&name, "CityBannerInstances.xml") {
        return r;
    }
    let b = &mut *buf;
    XML_PATCHED.store(false, Ordering::Relaxed);
    if b.ptr.is_null() || b.len == 0 {
        return r;
    }
    // Le NUL final ajouté par la lecture n'est pas du texte.
    let len = if *b.ptr.add(b.len - 1) == 0 { b.len - 1 } else { b.len };
    match patch_xml(std::slice::from_raw_parts(b.ptr, len)) {
        Ok(text) if replace_buffer(b, &text) => {
            XML_PATCHED.store(true, Ordering::Relaxed);
            log(&format!("ui : {name} : calques ajoutés"));
        }
        Ok(_) => log(&format!("ui : {name} : allocation impossible, laissé tel quel")),
        Err(e) => log(&format!("ui : {name} laissé tel quel : {e}")),
    }
    r
}

/// Prologue de la lecture XML, puis son appel à l'ouverture (rel32 libre) et le test qui suit.
const XML_SIG: [u8; 0x47] = [
    0x48, 0x8b, 0xc4, 0x48, 0x89, 0x58, 0x08, 0x57, 0x48, 0x83, 0xec, 0x50, 0x48, 0x8b, 0xfa, 0x48, 0xc7, 0x42,
    0x08, 0x00, 0x00, 0x00, 0x00, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x40, 0xc8, 0xff, 0xff, 0xff, 0xff,
    0x48, 0x8d, 0x50, 0xc8, 0x89, 0x58, 0xd4, 0x89, 0x58, 0xd0, 0x48, 0xc7, 0x40, 0xd8, 0x00, 0x00, 0x00, 0x00,
    0x48, 0xc7, 0x40, 0xe0, 0x00, 0x00, 0x00, 0x00, 0xe8, 0x3d, 0x92, 0xf6, 0xff, 0x3b, 0xc3, 0x74, 0x50,
];
/// Octets du rel32 de l'appel, différents d'un exe à l'autre.
const XML_SIG_FREE: std::ops::Range<usize> = 0x3e..0x42;
/// mov rax,rsp / mov [rax+8],rbx / push rdi / sub rsp,50h / mov rdi,rdx / mov qword [rdx+8],0 :
/// aucune instruction relative, recopiées telles quelles.
const XML_PROLOGUE: usize = 23;

fn find_sig(hay: &[u8]) -> Vec<usize> {
    hay.windows(XML_SIG.len())
        .enumerate()
        .filter(|(_, w)| w.iter().zip(XML_SIG.iter()).enumerate().all(|(i, (a, b))| XML_SIG_FREE.contains(&i) || a == b))
        .map(|(i, _)| i)
        .collect()
}

unsafe fn rd<T: Copy>(at: usize) -> T {
    std::ptr::read_unaligned(at as *const T)
}

/// Section `.text` du module : (début, taille).
unsafe fn text_section(module: usize) -> Option<(usize, usize)> {
    let nt = module + rd::<u32>(module + 0x3c) as usize;
    let count = rd::<u16>(nt + 6) as usize;
    let first = nt + 24 + rd::<u16>(nt + 20) as usize;
    (0..count).map(|i| first + 40 * i).find(|&s| &rd::<[u8; 8]>(s)[..5] == b".text").map(|s| {
        let size = rd::<u32>(s + 8) as usize;
        (module + rd::<u32>(s + 12) as usize, size)
    })
}

/// Le nom importé est-il `func`, tel quel ou décoré en C++ (`?func@@…`) ? L'exe importe les
/// fonctions de HavokScript sous leur nom décoré : `?luaL_loadbuffer@@YAHPEAUlua_State@@PEBD_K1@Z`.
fn is_import(name: &[u8], func: &str) -> bool {
    name == func.as_bytes() || name.strip_prefix(b"?").and_then(|n| n.strip_prefix(func.as_bytes())).is_some_and(|n| n.starts_with(b"@@"))
}

/// Entrée de la table d'imports du module pour `dll!func`.
unsafe fn iat_slot(module: usize, dll: &str, func: &str) -> Option<usize> {
    let nt = module + rd::<u32>(module + 0x3c) as usize;
    // PE32+ : répertoire des imports, deuxième entrée des répertoires de données.
    let imports = rd::<u32>(nt + 24 + 0x78) as usize;
    if imports == 0 {
        return None;
    }
    let mut d = module + imports;
    loop {
        let (oft, name, ft) = (rd::<u32>(d) as usize, rd::<u32>(d + 12) as usize, rd::<u32>(d + 16) as usize);
        if name == 0 {
            return None;
        }
        let dll_name = CStr::from_ptr((module + name) as *const c_char).to_string_lossy();
        if dll_name.eq_ignore_ascii_case(dll) {
            let names = if oft != 0 { oft } else { ft };
            for i in 0.. {
                let thunk = rd::<u64>(module + names + i * 8);
                if thunk == 0 {
                    break;
                }
                if thunk & (1 << 63) != 0 {
                    continue;
                }
                let f = CStr::from_ptr((module + thunk as usize + 2) as *const c_char);
                if is_import(f.to_bytes(), func) {
                    return Some(module + ft + i * 8);
                }
            }
            return None;
        }
        d += 20;
    }
}

unsafe fn write_slot(slot: usize, value: usize) -> bool {
    let mut old = 0u32;
    if win::VirtualProtect(slot as *mut c_void, 8, win::PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return false;
    }
    *(slot as *mut usize) = value;
    win::VirtualProtect(slot as *mut c_void, 8, old, &mut old);
    true
}

/// Appelé quand l'originale vient d'être chargée : l'interface de la partie n'est pas encore lue.
pub unsafe fn on_gamecore_loaded() {
    if !active() || IAT_SLOT.load(Ordering::Relaxed) != 0 {
        return;
    }
    let exe = win::GetModuleHandleW(std::ptr::null()) as usize;

    // XML d'abord : le bloc Lua dépend des calques.
    if BOUTONS.load(Ordering::Relaxed) || JOURNAL.load(Ordering::Relaxed) {
        let hits = text_section(exe)
            .map(|(start, size)| find_sig(std::slice::from_raw_parts(start as *const u8, size)).into_iter().map(|o| start + o).collect())
            .unwrap_or_else(Vec::new);
        match hits[..] {
            [at] => match patch::install_at("lecture XML", at, &XML_SIG[..XML_PROLOGUE], at + XML_PROLOGUE, hook_read_xml as *const () as usize) {
                Ok(tramp) => {
                    XML_TRAMP.store(tramp, Ordering::Relaxed);
                    XML_SITE.store(at, Ordering::Relaxed);
                }
                Err(e) => log(&format!("ui : {e}")),
            },
            _ => log(&format!("ui : lecture XML trouvée {} fois au lieu d'une, pas de calques", hits.len())),
        }
    }

    let Some(slot) = iat_slot(exe, "HavokScript_FinalRelease.dll", "luaL_loadbuffer") else {
        log("ui : import luaL_loadbuffer introuvable dans l'exe, interface inchangée");
        return;
    };
    REAL_LOADBUFFER.store(*(slot as *const usize), Ordering::Relaxed);
    if !write_slot(slot, hook_loadbuffer as *const () as usize) {
        log("ui : VirtualProtect a échoué sur la table d'imports");
        return;
    }
    IAT_SLOT.store(slot, Ordering::Relaxed);
    log(&format!(
        "ui : accroches posées (lecture XML {}), boutons {}, recentrage {}, journal {}, mesure des épingles {}, épingles allégées {}, merveilles {}",
        if XML_TRAMP.load(Ordering::Relaxed) != 0 { "oui" } else { "non" },
        if BOUTONS.load(Ordering::Relaxed) { "on" } else { "off" },
        if RECENTRAGE.load(Ordering::Relaxed) { "on" } else { "off" },
        if JOURNAL.load(Ordering::Relaxed) { "on" } else { "off" },
        if MESURE.load(Ordering::Relaxed) { "on" } else { "off" },
        if OPTI.load(Ordering::Relaxed) { "on" } else { "off" },
        if MERVEILLES.load(Ordering::Relaxed) { "on" } else { "off" },
    ));
}

/// Le proxy est déchargé alors que l'exe reste en mémoire : on rend l'import et la lecture XML.
pub unsafe fn on_proxy_unload() {
    let slot = IAT_SLOT.swap(0, Ordering::Relaxed);
    if slot != 0 && *(slot as *const usize) == hook_loadbuffer as *const () as usize {
        write_slot(slot, REAL_LOADBUFFER.load(Ordering::Relaxed));
    }
    let at = XML_SITE.swap(0, Ordering::Relaxed);
    if at != 0 {
        patch::restore_at(at, &XML_SIG[..XML_PROLOGUE]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Sid Meier's Civilization VI";
    const BSM: &str = r"C:\Program Files (x86)\Steam\steamapps\workshop\content\289070\1916397407";

    fn read(path: String) -> Option<Vec<u8>> {
        let r = std::fs::read(&path).ok();
        if r.is_none() {
            eprintln!("absent, test sauté : {path}");
        }
        r
    }

    #[test]
    fn noms_de_fichiers() {
        assert!(is_file("C:/x/UI/Replacements/CityBannerManager.lua", "CityBannerManager.lua"));
        assert!(is_file(r"..\..\DLC\citybannermanager.lua", "CityBannerManager.lua"));
        assert!(!is_file("C:/x/CityBannerManager_Expansion2.lua", "CityBannerManager.lua"));
        assert!(!is_file("C:/x/MyCityBannerManager.lua", "CityBannerManager.lua"));
    }

    #[test]
    fn merveilles_ajoutees_a_production_helper() {
        assert_eq!(script_name(r"C:\x\Base\Assets\UI\ProductionHelper.lua"), "productionhelper.lua");
        let out = String::from_utf8(patch_merveilles(b"x = 1")).unwrap();
        assert!(out.starts_with("x = 1\r\n-- ====="));
        assert!(out.ends_with(MERVEILLES_BLOCK));
        let Some(src) = read(format!(r"{GAME}\Base\Assets\UI\ProductionHelper.lua")) else { return };
        assert!(String::from_utf8_lossy(&src).contains("function SwapQueueItem(sourceIndex, destIndex)"));
    }

    #[test]
    fn scripts_mesures() {
        assert_eq!(measured(r"C:\x\Base\Assets\UI\MapPinManager.lua").as_deref(), Some("mappinmanager.lua"));
        assert_eq!(measured("ui/mappinmanager_dmt.lua").as_deref(), Some("mappinmanager_dmt.lua"));
        assert_eq!(measured(r"..\DLC\Expansion2\UI\Replacements\DiplomacyRibbon_Expansion2.lua").as_deref(), Some("diplomacyribbon_expansion2.lua"));
        assert_eq!(measured("@MapPinListPanel.lua").as_deref(), Some("mappinlistpanel.lua"));
        assert!(measured("C:/x/MapPinPopup.lua").is_none());
        assert!(measured("C:/x/DiplomacyRibbon.xml").is_none());
        assert!(measured(r"C:\x\Base\Assets\UI\DiplomacyRibbonSupport.lua").is_none());
        assert_eq!(measured(r"C:\x\diplomacyribbon_spec.lua").as_deref(), Some("diplomacyribbon_spec.lua"));
        let out = patch_epingles(b"x = 1", "mappinmanager.lua", false, true);
        assert!(out.starts_with(b"x = 1\r\nlocal MesureEpingles_Ctx = \"mappinmanager.lua\";\r\n"));
        assert!(out.ends_with(MESURE_BLOCK.as_bytes()));
        // Optimisation d'abord, mesure ensuite : la mesure enveloppe les fonctions optimisées.
        let both = String::from_utf8(patch_epingles(b"x = 1", "mappinlistpanel.lua", true, true)).unwrap();
        let (o, m) = (both.find("local EpinglesOpti_Ctx = \"mappinlistpanel.lua\";").unwrap(), both.find("local MesureEpingles_Ctx").unwrap());
        assert!(o < m && both.ends_with(MESURE_BLOCK));
        assert_eq!(patch_epingles(b"x = 1", "a.lua", false, false), b"x = 1");
        assert!(!OPTIMIZED.contains(&script_name(r"C:\x\DiplomacyRibbon.lua").as_str()));
    }

    #[test]
    fn recentrage() {
        let src = "function OnICBMStrikeButtonClick( iPlotID, eWMD )\n\tUI.SelectCity(pCity);\nend\nfunction X()\n\tUI.SelectCity(a);\nend\n";
        let mut t = src.to_string();
        no_recenter(&mut t, "OnICBMStrikeButtonClick", "UI.SelectCity(").unwrap();
        assert!(t.contains("UI.SelectCity( pCity, false );"));
        assert!(t.contains("UI.SelectCity(a);"), "seule la fonction visée change");
        let mut t2 = t.clone();
        assert!(no_recenter(&mut t2, "OnICBMStrikeButtonClick", "UI.SelectCity(").is_err(), "pas deux fois");
        assert!(no_recenter(&mut t2, "Absente", "UI.SelectCity(").is_err());
    }

    fn check_lua(path: String) {
        let Some(src) = read(path) else { return };
        let (new, notes) = patch_lua(&src, true, false);
        assert!(notes.is_empty(), "{notes:?}");
        let new = String::from_utf8(new.unwrap()).unwrap();
        assert!(new.contains("UI.SelectCity( pCity, false )"));
        assert!(new.contains("UI.SelectDistrict( pDistrict, false )"));
        // Le fichier en contient déjà une, en commentaire : « Don't auto center ».
        let before = String::from_utf8_lossy(&src).matches(", false )").count();
        assert_eq!(new.matches(", false )").count(), before + 3);
        assert!(new.ends_with(LUA_BLOCK));
        // Les noms dont le bloc a besoin existent dans ce fichier.
        for name in ["local m_CityBannerIM", "local m_EncampmentBannerIM", "local m_WMDBannerIM", "local YOFFSET_2DVIEW",
            "local ZOFFSET_3DVIEW", "function CityBanner:UpdatePosition", "function CityBanner:UpdateVisibility",
            "function CityBanner:UpdateRangeStrike", "function CityBanner:UpdateWMDBanner", "function CityBanner:Uninitialize",
            "function OnShutdown"] {
            assert!(new.contains(name), "{name} absent");
        }
    }

    #[test]
    fn lua_du_jeu() {
        check_lua(format!(r"{GAME}\DLC\Expansion2\UI\CityBanners\CityBannerManager.lua"));
    }

    #[test]
    fn lua_de_bsm() {
        check_lua(format!(r"{BSM}\UI\Replacements\CityBannerManager.lua"));
    }

    #[test]
    fn xml_du_jeu() {
        let Some(src) = read(format!(r"{GAME}\DLC\Expansion2\UI\CityBanners\CityBannerInstances.xml")) else { return };
        let new = String::from_utf8(patch_xml(&src).unwrap()).unwrap();
        assert!(new.contains(r#"<Button ID="NukeBombButton" ConsumeMouse="1" Anchor="C,C""#));
        assert!(new.contains(r#"<Button ID="ThermoNukeBombButton" ConsumeMouse="1" Anchor="C,C""#));
        assert!(new.contains(r#"<Instance Name="ProxyStrikeWMD">"#));
        assert!(new.trim_end().ends_with("</Include>"));
        // Trois instances ajoutées, chacune fermée.
        let old = String::from_utf8_lossy(&src);
        for (open, close) in [("<Instance ", "</Instance>"), ("<ZoomAnchor", "</ZoomAnchor>")] {
            assert_eq!(new.matches(open).count(), old.matches(open).count() + 3, "{open}");
            assert_eq!(new.matches(close).count(), old.matches(close).count() + 3, "{close}");
        }
        assert!(patch_xml(b"<Include></Include>").is_err());
    }

    /// La table d'imports du binaire de test : son entrée pour GetModuleHandleW pointe sur la
    /// fonction que rend GetProcAddress.
    #[test]
    fn table_d_imports() {
        unsafe {
            let exe = win::GetModuleHandleW(std::ptr::null()) as usize;
            let slot = iat_slot(exe, "kernel32.dll", "GetModuleHandleW").expect("import introuvable");
            let k32 = win::GetModuleHandleW(win::wide("kernel32.dll").as_ptr());
            assert_eq!(*(slot as *const usize), win::GetProcAddress(k32, b"GetModuleHandleW\0".as_ptr()) as usize);
            assert!(iat_slot(exe, "kernel32.dll", "PasUneFonction").is_none());
            assert!(iat_slot(exe, "HavokScript_FinalRelease.dll", "luaL_loadbuffer").is_none());
        }
    }

    #[test]
    fn city_panel_du_jeu() {
        let Some(src) = read(format!(r"{GAME}\Base\Assets\UI\Panels\CityPanel.lua")) else { return };
        let new = String::from_utf8(patch_city_panel(&src).unwrap()).unwrap();
        assert!(!new.contains("owningPlayerID"));
        assert_eq!(new.len(), src.len() - 1, "owning → owner, rien d'autre");
        assert!(patch_city_panel(new.as_bytes()).is_err(), "pas deux fois");
    }

    fn check_flags(path: String) {
        let Some(src) = read(path) else { return };
        let bytes = patch_unit_flags(&src).unwrap();
        // Rien d'autre ne change : le bloc, plus deux noms de fonction plus longs.
        let longer = "Civ6RolloverProxy_OnUnitFlagLClick".len() - "OnUnitFlagClick".len();
        assert_eq!(bytes.len(), src.len() + FLAG_BLOCK.len() + 2 * longer);
        let new = String::from_utf8_lossy(&bytes);
        assert_eq!(new.matches("RegisterCallback( Mouse.eLClick, Civ6RolloverProxy_OnUnitFlagLClick )").count(), 2);
        assert_eq!(new.matches("RegisterCallback( Mouse.eRClick, OnUnitFlagClick )").count(), 2, "clic droit inchangé");
        // Le gestionnaire est défini avant OnUnitFlagClick, donc avant tout branchement.
        assert!(new.find("function Civ6RolloverProxy_OnUnitFlagLClick").unwrap() < new.find("\nfunction OnUnitFlagClick(").unwrap());
        assert!(patch_unit_flags(&bytes).is_err(), "pas deux fois");
    }

    #[test]
    fn drapeaux_du_jeu() {
        check_flags(format!(r"{GAME}\Base\Assets\UI\UnitFlagManager.lua"));
    }

    #[test]
    fn production_panel_du_jeu() {
        let Some(src) = read(format!(r"{GAME}\Base\Assets\UI\Panels\ProductionPanel.lua")) else { return };
        let new = patch_production_panel(&src).unwrap();
        assert!(new.starts_with(&src) && new.ends_with(PRODUCTION_BLOCK.as_bytes()));
        assert!(patch_production_panel(&new).is_err(), "pas deux fois");
        assert!(patch_production_panel(b"function View(data) end").is_err());
        assert!(is_file(r"C:\x\Base\Assets\UI\Panels\ProductionPanel.lua", "ProductionPanel.lua"));
        assert!(!is_file("ui/replacements/productionpanel_bbg.lua", "ProductionPanel.lua"));
    }

    #[test]
    fn drapeaux_de_bsm() {
        check_flags(format!(r"{BSM}\UI\Replacements\unitflagmanager.lua"));
    }

    #[test]
    fn noms_decores() {
        let mangled = b"?luaL_loadbuffer@@YAHPEAUlua_State@@PEBD_K1@Z";
        assert!(is_import(mangled, "luaL_loadbuffer"));
        assert!(is_import(b"luaL_loadbuffer", "luaL_loadbuffer"));
        assert!(!is_import(b"?luaL_loadbufferx@@YAHPEAUlua_State@@PEBD_K1@Z", "luaL_loadbuffer"));
        assert!(!is_import(b"?luaL_loadstring@@YAHPEAUlua_State@@PEBD@Z", "luaL_loadbuffer"));
    }

    /// L'entrée luaL_loadbuffer des vrais exe, projetés comme images sans être exécutés : celle que
    /// Ghidra montre en DX12 (`140fa54e8`).
    #[test]
    fn import_havokscript_des_exe() {
        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryExW(name: *const u16, file: *mut c_void, flags: u32) -> win::HMODULE;
            fn FreeLibrary(module: win::HMODULE) -> i32;
        }
        const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 0x20;
        for exe in ["CivilizationVI_DX12.exe", "CivilizationVI.exe"] {
            let path = format!(r"{GAME}\Base\Binaries\Win64Steam\{exe}");
            if !std::path::Path::new(&path).exists() {
                eprintln!("absent, test sauté : {path}");
                continue;
            }
            unsafe {
                let h = LoadLibraryExW(win::wide(&path).as_ptr(), std::ptr::null_mut(), LOAD_LIBRARY_AS_IMAGE_RESOURCE);
                assert!(!h.is_null(), "{exe}");
                // Les bits bas du handle signalent une image projetée en ressource.
                let base = h as usize & !3;
                let slot = iat_slot(base, "HavokScript_FinalRelease.dll", "luaL_loadbuffer").unwrap_or_else(|| panic!("{exe}"));
                if exe == "CivilizationVI_DX12.exe" {
                    assert_eq!(slot - base, 0xfa54e8);
                }
                FreeLibrary(h);
            }
        }
    }

    #[test]
    fn signature_xml() {
        for (exe, expected) in [("CivilizationVI_DX12.exe", 0xa2d280usize), ("CivilizationVI.exe", 0xa1f3f0)] {
            let Some(bytes) = read(format!(r"{GAME}\Base\Binaries\Win64Steam\{exe}")) else { continue };
            // Adresse virtuelle relative de chaque occurrence, par les en-têtes de sections.
            let base = bytes.as_ptr() as usize;
            let (nt, hits) = unsafe { (base + rd::<u32>(base + 0x3c) as usize, find_sig(&bytes)) };
            assert_eq!(hits.len(), 1, "{exe}");
            let rva = unsafe {
                let count = rd::<u16>(nt + 6) as usize;
                let first = nt + 24 + rd::<u16>(nt + 20) as usize;
                (0..count)
                    .map(|i| first + 40 * i)
                    .find_map(|s| {
                        let (va, raw, rawsize) = (rd::<u32>(s + 12) as usize, rd::<u32>(s + 20) as usize, rd::<u32>(s + 16) as usize);
                        (raw..raw + rawsize).contains(&hits[0]).then(|| hits[0] - raw + va)
                    })
                    .unwrap()
            };
            assert_eq!(rva, expected, "{exe}");
        }
    }
}
