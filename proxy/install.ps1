# Installe (ou retire avec -Uninstall) le proxy de GameCore_XP2_FinalRelease.dll.
#   powershell -ExecutionPolicy Bypass -File perf\rollover\proxy\install.ps1 -Mode count
#   powershell -ExecutionPolicy Bypass -File perf\rollover\proxy\install.ps1 -Ameliorations verify
#   powershell -ExecutionPolicy Bypass -File perf\rollover\proxy\install.ps1 -Uninstall
# L'original est renommé GameCore_XP2_FinalRelease_orig.dll, jamais supprimé ; le proxy prend son
# nom. -Uninstall remet l'original en place. Une vérification des fichiers par Steam ou une mise à
# jour du jeu remet aussi l'original : il faut alors réinstaller.
param(
    [ValidateSet("coalesce", "count", "off")]
    [string]$Mode = "coalesce",
    # Portée du regroupement des instantanés : cascade de guerres seulement, ou chaque passe de message.
    [ValidateSet("cascade", "message")]
    [string]$Portee = "cascade",
    # Détour de 180260720 (amélioration constructible) : voir src/improvements.rs.
    [ValidateSet("off", "count", "verify", "fast", "quiet")]
    [string]$Ameliorations = "off",
    # Mémoire de 18037ec60 (rendement d'une case) pendant les attentes de l'IA : voir src/rendements.rs.
    [ValidateSet("off", "count", "verify", "fast", "quiet")]
    [string]$Rendements = "off",
    [switch]$Uninstall,
    [string]$GameDir = "C:\Program Files (x86)\Steam\steamapps\common\Sid Meier's Civilization VI"
)
$ErrorActionPreference = "Stop"

$bin = Join-Path $GameDir "DLC\Expansion2\Binaries\Win64"
$dll = Join-Path $bin "GameCore_XP2_FinalRelease.dll"
$orig = Join-Path $bin "GameCore_XP2_FinalRelease_orig.dll"
$cfg = Join-Path $bin "rollover_proxy.cfg"
if (-not (Test-Path $dll)) { throw "GameCore_XP2_FinalRelease.dll introuvable dans $bin" }
if (Get-Process | Where-Object { $_.Name -like "CivilizationVI*" }) { throw "Ferme Civilization VI d'abord." }

function Test-Ours($path) {
    if (-not (Test-Path $path)) { return $false }
    [Text.Encoding]::ASCII.GetString([IO.File]::ReadAllBytes($path)).Contains("RolloverProxyStatus")
}

# Prototypes précédents, posés à côté de l'exe : on les retire s'ils sont encore là.
$exeDir = Join-Path $GameDir "Base\Binaries\Win64Steam"
foreach ($old in "version.dll", "dxgi.dll") {
    $p = Join-Path $exeDir $old
    if (Test-Ours $p) { Remove-Item $p; Write-Host "Ancien proxy $old retiré." }
}
Remove-Item (Join-Path $exeDir "rollover_proxy.cfg") -ErrorAction SilentlyContinue

if ($Uninstall) {
    if (Test-Ours $dll) {
        if (-not (Test-Path $orig)) { throw "Le proxy est en place mais $orig manque : vérifie les fichiers du jeu dans Steam." }
        Remove-Item $dll
        Rename-Item $orig (Split-Path $dll -Leaf)
    }
    Remove-Item $cfg -ErrorAction SilentlyContinue
    Write-Host "Proxy retiré : GameCore_XP2_FinalRelease.dll est de nouveau l'original."
    return
}

$built = Join-Path $PSScriptRoot "target\release\GameCore_XP2_FinalRelease.dll"
if (-not (Test-Path $built)) { throw "Compile d'abord : cargo build --release dans $PSScriptRoot" }
if (-not (Test-Ours $dll)) {
    # L'original est en place. Un _orig déjà présent viendrait d'une installation précédente
    # écrasée par Steam : il peut être d'une autre version, on ne choisit pas à la place de l'utilisateur.
    if (Test-Path $orig) { throw "$orig existe déjà alors que l'original est en place (mise à jour Steam ?). Supprime ou renomme l'un des deux." }
    Rename-Item $dll (Split-Path $orig -Leaf)
}
Copy-Item $built $dll -Force
Set-Content -Encoding ascii $cfg "mode=$Mode", "portee=$Portee", "ameliorations=$Ameliorations", "rendements=$Rendements"
Write-Host "Proxy installé dans $bin (mode $Mode, portée $Portee, ameliorations $Ameliorations, rendements $Rendements) ; original conservé sous GameCore_XP2_FinalRelease_orig.dll."
Write-Host "Journal : $env:LOCALAPPDATA\Civ6RolloverProxy\RolloverProxy.log"
