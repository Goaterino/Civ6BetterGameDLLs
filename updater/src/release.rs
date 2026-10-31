//! Lecture d'une release : manifeste signé, puis fichiers contrôlés par leur SHA-256 (voir
//! release/README.md). Les téléchargements passent par le curl.exe de Windows, sans pile TLS à embarquer.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Deserialize;
use sha2::{Digest, Sha256};

const RELEASES: &str = "https://github.com/Goaterino/Civ6BetterGameDLLs/releases";
const PUBLIC_KEY: &str = include_str!("../../release/public_key.txt");

#[derive(Deserialize)]
pub struct Manifest {
    pub version: String,
    pub files: Vec<File>,
}

#[derive(Deserialize)]
pub struct File {
    pub name: String,
    pub dest: String,
    pub sha256: String,
}

/// Dossier de travail, où le manifeste et les fichiers sont téléchargés avant l'installation.
pub fn work_dir() -> PathBuf {
    std::env::temp_dir().join("BetterGameDLLs")
}

/// Télécharge le manifeste de la dernière release publiée dans `work`, et vérifie sa signature.
pub fn fetch_manifest(work: &Path) -> Result<Manifest, String> {
    fs::create_dir_all(work).map_err(|e| format!("{}: {e}", work.display()))?;
    for name in ["manifest.json", "manifest.json.sig"] {
        download(&format!("{RELEASES}/latest/download/{name}"), &work.join(name))?;
    }
    verify_manifest(work)
}

/// Télécharge dans `work` les fichiers du manifeste qui n'y sont pas déjà, et les contrôle.
pub fn fetch_files(m: &Manifest, work: &Path) -> Result<(), String> {
    for f in &m.files {
        if check_file(work, f).is_err() {
            download(&format!("{RELEASES}/download/v{}/{}", m.version, f.name), &work.join(&f.name))?;
            check_file(work, f)?;
        }
    }
    Ok(())
}

/// Relit une release déjà téléchargée dans `work` (côté administrateur, avant d'installer) :
/// signature du manifeste et SHA-256 de chaque fichier.
pub fn load(work: &Path) -> Result<Manifest, String> {
    let m = verify_manifest(work)?;
    for f in &m.files {
        check_file(work, f)?;
    }
    Ok(m)
}

fn verify_manifest(work: &Path) -> Result<Manifest, String> {
    let b64 = base64::engine::general_purpose::STANDARD;
    let key: [u8; 32] = b64.decode(PUBLIC_KEY.trim()).unwrap().try_into().unwrap();
    let key = VerifyingKey::from_bytes(&key).unwrap();
    let data = fs::read(work.join("manifest.json")).map_err(|e| format!("manifest.json : {e}"))?;
    let sig = fs::read_to_string(work.join("manifest.json.sig")).map_err(|e| format!("manifest.json.sig : {e}"))?;
    let sig = b64.decode(sig.trim()).ok().and_then(|s| <[u8; 64]>::try_from(s).ok());
    let sig = sig.ok_or("signature du manifeste illisible")?;
    key.verify_strict(&data, &Signature::from_bytes(&sig)).map_err(|_| "signature du manifeste invalide")?;
    serde_json::from_slice(&data).map_err(|e| format!("manifeste illisible : {e}"))
}

fn check_file(work: &Path, f: &File) -> Result<(), String> {
    if sha256(&work.join(&f.name))? == f.sha256 {
        Ok(())
    } else {
        Err(format!("{} : SHA-256 différent de celui du manifeste", f.name))
    }
}

pub fn sha256(path: &Path) -> Result<String, String> {
    let data = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(Sha256::digest(data).iter().map(|b| format!("{b:02x}")).collect())
}

/// `BGD_DIST`, s'il est défini, remplace GitHub par un dossier local (tests).
fn download(url: &str, dest: &Path) -> Result<(), String> {
    if let Some(dir) = std::env::var_os("BGD_DIST") {
        let name = url.rsplit('/').next().unwrap();
        return fs::copy(Path::new(&dir).join(name), dest).map(drop).map_err(|e| format!("{name} : {e}"));
    }
    let curl = Path::new(&std::env::var_os("SystemRoot").unwrap_or("C:\\Windows".into())).join("System32\\curl.exe");
    let out = Command::new(curl)
        .args(["-fsSL", "--retry", "2", "-o"])
        .arg(dest)
        .arg(url)
        .output()
        .map_err(|e| format!("curl.exe : {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!("téléchargement de {url} : {}", String::from_utf8_lossy(&out.stderr).trim()))
    }
}
