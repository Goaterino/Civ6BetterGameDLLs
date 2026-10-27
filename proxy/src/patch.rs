//! Détours par saut absolu, sans désassembleur : chaque site est décrit à la main.
//!
//! Le site est écrasé par `jmp [rip+0]` suivi de l'adresse du détour (14 octets). Le trampoline
//! recopie les instructions déplaçables du prologue puis saute à l'instruction qui suit.

use crate::win;
use std::ffi::c_void;

pub struct Site {
    pub name: &'static str,
    /// Adresse dans GameCore_XP2_FinalRelease.dll, base 0x180000000 (adresses Ghidra).
    pub address: u64,
    /// Octets attendus au site : le détour n'est posé que s'ils correspondent.
    pub expected: &'static [u8],
    /// Instructions recopiées dans le trampoline : [début, fin) en décalage depuis le site.
    pub copy: (usize, usize),
}

pub const IMAGE_BASE: u64 = 0x1_8000_0000;
const JMP_LEN: usize = 14;

fn jmp_abs(target: u64) -> [u8; JMP_LEN] {
    let mut j = [0u8; JMP_LEN];
    j[..6].copy_from_slice(&[0xFF, 0x25, 0, 0, 0, 0]);
    j[6..].copy_from_slice(&target.to_le_bytes());
    j
}

/// Vérifie tous les sites avant d'en modifier un seul : une autre version du jeu ne doit rien patcher.
pub unsafe fn check(base: usize, sites: &[Site]) -> Result<(), String> {
    for s in sites {
        let at = base + (s.address - IMAGE_BASE) as usize;
        let have = std::slice::from_raw_parts(at as *const u8, s.expected.len());
        if have != s.expected {
            return Err(format!("{} : octets inattendus en {:#x} ({:02x?})", s.name, s.address, have));
        }
        assert!(s.expected.len() >= JMP_LEN && s.copy.1 <= s.expected.len());
    }
    Ok(())
}

/// Remet les octets d'origine au site (le trampoline reste alloué, il n'est plus visé).
pub unsafe fn restore(base: usize, site: &Site) {
    let at = base + (site.address - IMAGE_BASE) as usize;
    let mut old = 0u32;
    if win::VirtualProtect(at as *mut c_void, JMP_LEN, win::PAGE_EXECUTE_READWRITE, &mut old) != 0 {
        std::ptr::copy_nonoverlapping(site.expected.as_ptr(), at as *mut u8, JMP_LEN);
        win::VirtualProtect(at as *mut c_void, JMP_LEN, old, &mut old);
        win::FlushInstructionCache(win::GetCurrentProcess(), at as *const c_void, JMP_LEN);
    }
}

/// Pose le détour et rend l'adresse du trampoline, qui exécute la fonction d'origine.
pub unsafe fn install(base: usize, site: &Site, hook: usize) -> Result<usize, String> {
    let at = base + (site.address - IMAGE_BASE) as usize;
    let copied = &site.expected[site.copy.0..site.copy.1];
    let tramp = win::VirtualAlloc(
        std::ptr::null_mut(),
        copied.len() + JMP_LEN,
        win::MEM_COMMIT | win::MEM_RESERVE,
        win::PAGE_EXECUTE_READWRITE,
    ) as *mut u8;
    if tramp.is_null() {
        return Err(format!("{} : VirtualAlloc a échoué", site.name));
    }
    std::ptr::copy_nonoverlapping(copied.as_ptr(), tramp, copied.len());
    let resume = (at + site.copy.1) as u64;
    std::ptr::copy_nonoverlapping(jmp_abs(resume).as_ptr(), tramp.add(copied.len()), JMP_LEN);

    let mut old = 0u32;
    if win::VirtualProtect(at as *mut c_void, JMP_LEN, win::PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return Err(format!("{} : VirtualProtect a échoué", site.name));
    }
    std::ptr::copy_nonoverlapping(jmp_abs(hook as u64).as_ptr(), at as *mut u8, JMP_LEN);
    win::VirtualProtect(at as *mut c_void, JMP_LEN, old, &mut old);
    win::FlushInstructionCache(win::GetCurrentProcess(), at as *const c_void, JMP_LEN);
    Ok(tramp as usize)
}
