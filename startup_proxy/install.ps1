# Installe (ou retire avec -Uninstall) le proxy de démarrage, DatabaseDLL_FinalRelease.dll.
#   powershell -ExecutionPolicy Bypass -File startup_proxy\install.ps1
#   powershell -ExecutionPolicy Bypass -File startup_proxy\install.ps1 -Uninstall
# L'original est renommé DatabaseDLL_FinalRelease_orig.dll, jamais supprimé ; le proxy prend son nom.
# -Uninstall remet l'original en place. Une vérification des fichiers par Steam ou une mise à jour du
# jeu remet aussi l'original : il faut alors réinstaller.
param(
    [switch]$Uninstall,
    [string]$GameDir = "C:\Program Files (x86)\Steam\steamapps\common\Sid Meier's Civilization VI"
)
$ErrorActionPreference = "Stop"

$bin = Join-Path $GameDir "Base\Binaries\Win64Steam"
$dll = Join-Path $bin "DatabaseDLL_FinalRelease.dll"
$orig = Join-Path $bin "DatabaseDLL_FinalRelease_orig.dll"
if (-not (Test-Path $dll)) { throw "DatabaseDLL_FinalRelease.dll introuvable dans $bin" }
if (Get-Process | Where-Object { $_.Name -like "CivilizationVI*" }) { throw "Ferme Civilization VI d'abord." }

function Test-Ours($path) {
    if (-not (Test-Path $path)) { return $false }
    [Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($path)).Contains("StartupProxyStatus")
}

if ($Uninstall) {
    if (Test-Ours $dll) {
        if (-not (Test-Path $orig)) { throw "Le proxy est en place mais $orig manque : vérifie les fichiers du jeu dans Steam." }
        Remove-Item $dll
        Rename-Item $orig (Split-Path $dll -Leaf)
    }
    Write-Host "Proxy de démarrage retiré : DatabaseDLL_FinalRelease.dll est de nouveau l'original."
    return
}

$built = Join-Path $PSScriptRoot "target\release\DatabaseDLL_FinalRelease.dll"
if (-not (Test-Path $built)) { throw "Compile d'abord : cargo build --release dans $PSScriptRoot" }
if (-not (Test-Ours $dll)) {
    # L'original est en place. Un _orig déjà présent viendrait d'une installation précédente
    # écrasée par Steam : il peut être d'une autre version, on ne choisit pas à la place de l'utilisateur.
    if (Test-Path $orig) { throw "$orig existe déjà alors que l'original est en place (mise à jour Steam ?). Supprime ou renomme l'un des deux." }
    Rename-Item $dll (Split-Path $orig -Leaf)
}
Copy-Item $built $dll -Force
Write-Host "Proxy de démarrage installé dans $bin ; original conservé sous DatabaseDLL_FinalRelease_orig.dll."
Write-Host "Journal : $env:LOCALAPPDATA\Civ6RolloverProxy\RolloverProxy.log"
