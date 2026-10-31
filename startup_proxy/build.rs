//! Les 5 exports de DatabaseDLL_FinalRelease.dll sont des renvois (forwarders) PE vers l'originale,
//! renommée DatabaseDLL_FinalRelease_orig.dll : le chargeur de Windows résout lui-même chaque import
//! de l'exe dans l'originale, sans code de relais dans le proxy.
//!
//! La chaîne d'outils est GNU : les renvois passent par un fichier .def donné à ld, en plus de celui
//! de rustc.

const ORIGINAL: &str = "DatabaseDLL_FinalRelease_orig";

/// Exports de l'originale (`proxy/gen_exports.py` les liste) : fonctions libres de l'espace de noms
/// `Database`, sous leur nom C++ décoré.
const EXPORTS: [&str; 5] = [
    "?CreateConnection@Database@@YAPEAVIDatabaseConnection1@1@PEBDHPEAVIDatabaseLogger@1@@Z",
    "?Initialize@Database@@YA_NPEAVIDatabaseMemoryAllocator@1@P6AXPEAX@Z@Z",
    "?Initialize@Database@@YA_NXZ",
    "?SetDefaultLogger@Database@@YAXPEAVIDatabaseLogger@1@@Z",
    "?Shutdown@Database@@YA_NXZ",
];

fn main() {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("renvois.def");
    let mut def = String::from("EXPORTS\n");
    for name in EXPORTS {
        def += &format!("    \"{name}\" = \"{ORIGINAL}.{name}\"\n");
    }
    std::fs::write(&out, def).unwrap();
    println!("cargo:rustc-cdylib-link-arg={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
}
