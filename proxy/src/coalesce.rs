//! Regroupement des instantanés de villes pendant une cascade de déclarations de guerre.
//!
//! Voir perf/rollover/cascade_guerre.md :
//! - `1802a1f90` (DeclareWar) est récursive à travers le signal « guerre déclarée » ; on compte sa
//!   profondeur ;
//! - `180072f80(événement, joueur)` refait l'instantané de toutes les villes d'un joueur pour
//!   l'interface. Il ne fait que lire l'état de jeu : pendant la cascade, on note les appels au lieu
//!   de les exécuter, puis on rejoue chaque couple (événement, joueur) une seule fois, dans l'ordre
//!   de première apparition, quand l'appel le plus externe de DeclareWar se termine.
//!
//! L'état de cascade est un statique protégé par un verrou, rattaché au thread qui l'a ouverte :
//! pas de `thread_local!`, dont les destructeurs survivraient au déchargement du proxy.

use crate::patch::{self, Site};
use crate::{log, win};
use std::sync::atomic::{AtomicU32, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;

const SNAPSHOT: Site = Site {
    name: "instantané des villes d'un joueur",
    address: 0x1_8007_2f80,
    // mov [rsp+10h],rbx / push rdi / sub rsp,30h / movsxd rbx,edx / mov edi,ecx
    expected: &[0x48, 0x89, 0x5c, 0x24, 0x10, 0x57, 0x48, 0x83, 0xec, 0x30, 0x48, 0x63, 0xda, 0x8b, 0xf9],
    copy: (0, 15),
    resume: 15,
};

const DECLARE_WAR: Site = Site {
    name: "DeclareWar",
    address: 0x1_802a_1f90,
    // cmp edx,-1 / jz <ret> : refait par le détour, car le saut relatif ne se recopie pas.
    // Puis mov rax,rsp / mov [rax+8],rcx / push rbx / push rbp / push rdi / push r12 / sub rsp,68h
    expected: &[
        0x83, 0xfa, 0xff, 0x0f, 0x84, 0x0a, 0x02, 0x00, 0x00, 0x48, 0x8b, 0xc4, 0x48, 0x89, 0x48, 0x08,
        0x53, 0x55, 0x57, 0x41, 0x54, 0x48, 0x83, 0xec, 0x68,
    ],
    copy: (9, 25),
    resume: 25,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Mode {
    /// Aucun détour.
    Off = 0,
    /// Détours posés, comportement inchangé : on mesure et on compte seulement.
    Count = 1,
    /// Les instantanés sont regroupés.
    Coalesce = 2,
}

static MODE: AtomicU8 = AtomicU8::new(Mode::Coalesce as u8);
static TRAMP_SNAPSHOT: AtomicUsize = AtomicUsize::new(0);
static TRAMP_DECLARE: AtomicUsize = AtomicUsize::new(0);
/// Base de l'originale une fois les détours posés, pour les retirer au déchargement.
static HOOKED_BASE: AtomicUsize = AtomicUsize::new(0);

fn mode() -> Mode {
    match MODE.load(Ordering::Relaxed) {
        0 => Mode::Off,
        1 => Mode::Count,
        _ => Mode::Coalesce,
    }
}

pub fn set_mode(m: Mode) {
    MODE.store(m as u8, Ordering::Relaxed);
}

#[derive(Default)]
struct Cascade {
    depth: u32,
    max_depth: u32,
    start: i64,
    declare_calls: u32,
    snapshot_calls: u32,
    /// Couples (événement, joueur) dans l'ordre de première apparition.
    pending: Vec<(usize, usize)>,
}

/// Thread qui a ouvert la cascade en cours (0 : aucune). Les autres threads passent sans s'arrêter.
static OWNER: AtomicU32 = AtomicU32::new(0);
static CASCADE: Mutex<Cascade> = Mutex::new(Cascade {
    depth: 0,
    max_depth: 0,
    start: 0,
    declare_calls: 0,
    snapshot_calls: 0,
    pending: Vec::new(),
});

fn cascade() -> std::sync::MutexGuard<'static, Cascade> {
    CASCADE.lock().unwrap_or_else(|e| e.into_inner())
}

type SnapshotFn = unsafe extern "system" fn(usize, usize);
type DeclareWarFn = unsafe extern "system" fn(usize, usize, usize, usize, usize, usize);

unsafe fn call_snapshot(event: usize, player: usize) {
    let f: SnapshotFn = std::mem::transmute(TRAMP_SNAPSHOT.load(Ordering::Relaxed));
    f(event, player)
}

unsafe extern "system" fn hook_snapshot(event: usize, player: usize) {
    let mut defer = false;
    if OWNER.load(Ordering::Relaxed) == win::GetCurrentThreadId() {
        let mut c = cascade();
        c.snapshot_calls += 1;
        // Seuls les 32 bits bas comptent : la fonction lit ECX et EDX.
        let key = (event & 0xffff_ffff, player & 0xffff_ffff);
        if !c.pending.contains(&key) {
            c.pending.push(key);
        }
        defer = mode() == Mode::Coalesce;
    }
    if !defer {
        call_snapshot(event, player);
    }
}

unsafe extern "system" fn hook_declare_war(this: usize, target: usize, reason: usize, force: usize, a5: usize, a6: usize) {
    if target as u32 == u32::MAX {
        return;
    }
    let f: DeclareWarFn = std::mem::transmute(TRAMP_DECLARE.load(Ordering::Relaxed));
    let tid = win::GetCurrentThreadId();
    let owner = OWNER.load(Ordering::Relaxed);
    if owner != 0 && owner != tid {
        // Cascade ouverte sur un autre thread : on ne s'en mêle pas.
        return f(this, target, reason, force, a5, a6);
    }
    {
        let mut c = cascade();
        if c.depth == 0 {
            *c = Cascade { start: win::qpc(), ..Cascade::default() };
            OWNER.store(tid, Ordering::Relaxed);
        }
        c.depth += 1;
        c.max_depth = c.max_depth.max(c.depth);
        c.declare_calls += 1;
    }
    f(this, target, reason, force, a5, a6);
    let done = {
        let mut c = cascade();
        c.depth -= 1;
        (c.depth == 0).then(|| std::mem::take(&mut *c))
    };
    if let Some(c) = done {
        OWNER.store(0, Ordering::Relaxed);
        finish(c);
    }
}

unsafe fn finish(c: Cascade) {
    let cascade_end = win::qpc();
    if mode() == Mode::Coalesce {
        for &(event, player) in &c.pending {
            call_snapshot(event, player);
        }
    }
    let flush_end = win::qpc();
    // Une déclaration isolée (sans instantané) n'est pas journalisée : seules les cascades comptent.
    if c.snapshot_calls > 0 || c.declare_calls > 1 {
        let players: Vec<String> = c.pending.iter().map(|&(e, p)| format!("{}:{:#x}", p as i32, e)).collect();
        log(&format!(
            "cascade mode={:?} declarewar={} profondeur_max={} instantanes_demandes={} uniques={} \
             cascade_ms={:.1} rejeu_ms={:.1} joueurs=[{}]",
            mode(),
            c.declare_calls,
            c.max_depth,
            c.snapshot_calls,
            c.pending.len(),
            win::qpc_ms(cascade_end - c.start),
            win::qpc_ms(flush_end - cascade_end),
            players.join(" "),
        ));
    }
}

/// Appelé quand l'originale vient d'être chargée, avant le premier appel relayé.
pub unsafe fn on_gamecore_loaded(base: usize) {
    if mode() == Mode::Off {
        log("originale chargée, mode Off : aucun détour");
        return;
    }
    let sites = [SNAPSHOT, DECLARE_WAR];
    if let Err(e) = patch::check(base, &sites) {
        log(&format!("originale chargée en {base:#x}, version inattendue, aucun détour : {e}"));
        return;
    }
    match patch::install(base, &SNAPSHOT, hook_snapshot as *const () as usize).and_then(|t| {
        TRAMP_SNAPSHOT.store(t, Ordering::Relaxed);
        patch::install(base, &DECLARE_WAR, hook_declare_war as *const () as usize)
    }) {
        Ok(t) => {
            TRAMP_DECLARE.store(t, Ordering::Relaxed);
            HOOKED_BASE.store(base, Ordering::Relaxed);
            crate::STATUS.fetch_or(crate::ST_HOOKED, Ordering::Relaxed);
            log(&format!("originale chargée en {base:#x}, détours posés, mode {:?}", mode()));
        }
        Err(e) => log(&format!("échec de la pose des détours : {e}")),
    }
}

/// Le proxy est déchargé alors que l'originale reste en mémoire : on remet ses octets d'origine.
/// Aucune écriture de fichier ici (on est sous le verrou du chargeur).
pub unsafe fn on_proxy_unload() {
    let base = HOOKED_BASE.swap(0, Ordering::Relaxed);
    if base != 0 {
        patch::restore(base, &DECLARE_WAR);
        patch::restore(base, &SNAPSHOT);
    }
}
