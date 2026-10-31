//! Merveille retirée de la file de production quand elle revient en tête après un échange.
//!
//! **Mis de côté, plus compilé.** Ancien `proxy/src/merveilles.rs`, remplacé par `ui_merveilles`
//! (proxy/src/ui_merveilles.lua), qui ne change aucune règle. Pour le rebrancher : le remettre dans
//! proxy/src sous le nom merveilles.rs, puis rajouter `mod merveilles;` et la lecture de
//! `merveilles_file=` dans lib.rs, `merveilles::on_proxy_unload()` dans DllMain,
//! `crate::merveilles::on_gamecore_loaded(h)` dans forward.rs, et le paramètre `-MerveillesFile` dans
//! install.ps1.
//!
//! Quand l'élément en tête change, `1801007a0` lance le bâtiment en tête. Pour une merveille, il
//! vérifie d'abord la case avec `18025bad0` (CanHaveWonderAt) et, en cas de refus, retire l'élément
//! (`1800ffbf0`). Au premier passage en tête, `1801020b0` pose la merveille sur sa case
//! (`m_aiConstructionPlot`, file `+0x290`). Après un échange qui la sort de la tête puis un autre qui
//! l'y remet, la case porte le chantier de cette merveille même : `18025bad0` refuse et la merveille
//! disparaît. `1801020b0` ne revérifie rien quand la case est déjà enregistrée ; seul le test de
//! `1801007a0` manque cette exception. Analyse et trace : wonder_queue/README.md.
//!
//! Le `call` de `180100a9a` est redirigé vers un relais qui mène à `wonder_at` : si la case visée
//! porte toujours le chantier que cette ville y a posé, le test réussit ; sinon, appel d'origine.
//! À la prise d'une ville, la destruction de sa file (`1800f38e0` → `1801034d0` → `1800ffc20`)
//! retire ses chantiers de la carte : ce cas reste celui du jeu.
//! `18025bad0` sert ailleurs (IA, interface, `1801020b0`) : ces appels ne changent pas.
//!
//! Contrairement aux autres réglages du proxy, celui-ci change l'état de la partie : en
//! multijoueur, tous les joueurs doivent l'avoir, sinon la file diverge et la partie se désynchronise.
//! Réglage `merveilles_file=on|off` dans rollover_proxy.cfg (off par défaut).

use crate::log;
use crate::patch::{alloc_near, rel32, write_rel32, IMAGE_BASE};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);
static BASE: AtomicUsize = AtomicUsize::new(0);
static PATCHED_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

const SITE: u64 = 0x1_8010_0a9a; // call CanHaveWonderAt dans 1801007a0
const WONDER_AT: u64 = 0x1_8025_bad0; // (?, case, bâtiment, joueur, 5 octets, ville) -> bool
const PLOT_INDEX: u64 = 0x1_8005_f2d0; // (case) -> y × largeur + x
const PLOT_INFO: u64 = 0x1_8004_2d00; // (case) -> propriété : +0xc ville, +0x1e merveille posée

const QUEUE_IN_CITY: usize = 0x10d0; // lea rcx, [ville + 0x10d0] (18006c8cc)

fn gc(addr: u64) -> usize {
    BASE.load(Ordering::Relaxed) + (addr - IMAGE_BASE) as usize
}

unsafe fn rd<T: Copy>(addr: usize) -> T {
    std::ptr::read(addr as *const T)
}

/// Case déjà enregistrée pour `building` dans la file de `city`, -1 sinon.
unsafe fn placed(city: usize, building: u32) -> i32 {
    let queue = city + QUEUE_IN_CITY;
    if rd::<usize>(queue + 0x1e8) != city || building >= rd::<u32>(queue + 0x2a0) {
        return -1;
    }
    rd::<i16>(rd::<usize>(queue + 0x290) + building as usize * 2) as i32
}

/// Le chantier de `building` posé par cette ville est toujours sur `plot` : case enregistrée dans
/// la file, merveille encore inscrite sur la case (ce que lit `1800ffc20` avant de l'en retirer) et
/// case encore à cette ville (ce que compare `18025bad0`). Une prise de ville ou tout autre retrait
/// du chantier fait échouer l'une des trois : on laisse alors le test d'origine décider.
unsafe fn still_ours(plot: usize, building: u32, city: usize) -> bool {
    let at = placed(city, building);
    if at < 0 {
        return false;
    }
    let index: unsafe extern "system" fn(usize) -> i32 = std::mem::transmute(gc(PLOT_INDEX));
    let info: unsafe extern "system" fn(usize) -> usize = std::mem::transmute(gc(PLOT_INFO));
    let p = info(plot);
    index(plot) == at && rd::<i16>(p + 0x1e) as i32 == building as i32 && rd::<i32>(p + 0xc) == rd::<i32>(city + 0xa8)
}

#[allow(clippy::too_many_arguments)]
unsafe extern "system" fn wonder_at(
    a: usize, plot: usize, building: usize, player: usize,
    e: usize, f: usize, g: usize, h: usize, i: usize, city: usize,
) -> usize {
    if city != 0 && plot != 0 && still_ours(plot, building as u32, city) {
        return 1;
    }
    type F = unsafe extern "system" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let original: F = std::mem::transmute(gc(WONDER_AT));
    original(a, plot, building, player, e, f, g, h, i, city)
}

/// Appelé quand l'originale vient d'être chargée.
pub unsafe fn on_gamecore_loaded(base: usize) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    BASE.store(base, Ordering::Relaxed);
    let at = gc(SITE);
    if rd::<u8>(at) != 0xe8 || rel32(at, gc(WONDER_AT)) != Some(rd::<i32>(at + 1)) {
        log(&format!("merveilles : version inattendue en {SITE:#x}, rien de redirigé"));
        return;
    }
    let Some(stub) = alloc_near(base) else {
        log("merveilles : pas de mémoire à moins de 2 Go de GameCore, rien de redirigé");
        return;
    };
    // Relais : jmp [rip+0] puis l'adresse de wonder_at.
    std::ptr::copy_nonoverlapping([0xffu8, 0x25, 0, 0, 0, 0].as_ptr(), stub as *mut u8, 6);
    std::ptr::write_unaligned((stub + 6) as *mut u64, wonder_at as *const () as u64);
    let Some(rel) = rel32(at, stub) else {
        log("merveilles : relais hors de portée, rien de redirigé");
        return;
    };
    write_rel32(at, rel);
    PATCHED_BASE.store(base, Ordering::Relaxed);
    log("merveilles : une merveille déjà posée n'est plus retirée en revenant en tête de file");
}

/// Le proxy est déchargé alors que l'originale reste en mémoire : on remet l'appel d'origine.
pub unsafe fn on_proxy_unload() {
    let base = PATCHED_BASE.swap(0, Ordering::Relaxed);
    if base == 0 {
        return;
    }
    let at = base + (SITE - IMAGE_BASE) as usize;
    if let Some(rel) = rel32(at, base + (WONDER_AT - IMAGE_BASE) as usize) {
        write_rel32(at, rel);
    }
}
