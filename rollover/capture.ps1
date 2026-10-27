# Capture WPR (profil CPU échantillonné, avec piles) d'un passage de tour de Civ6.
# À lancer dans un PowerShell administrateur, jeu ouvert, juste avant de finir le tour :
#   powershell -ExecutionPolicy Bypass -File perf\rollover\capture.ps1 -Name tour79
# Puis analyser (sans droits admin) :
#   .venv\Scripts\python.exe -I perf\rollover\analyze.py perf\rollover\captures\<dossier>
param(
    [string]$Name = "passage",
    # Ajoute les E/S disque et fichier, pour voir le coût de la sauvegarde auto.
    [switch]$WithIO
)
$ErrorActionPreference = "Stop"

$admin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $admin) { throw "WPR a besoin d'un PowerShell lancé en administrateur." }
if (-not (Get-Process | Where-Object { $_.Name -like "CivilizationVI*" })) { throw "Civilization VI ne tourne pas." }

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$dir = Join-Path $PSScriptRoot "captures\$stamp-$Name"
New-Item -ItemType Directory -Force $dir | Out-Null
$etl = Join-Path $dir "trace.etl"

# Une capture déjà en cours (session interrompue) empêcherait le démarrage. Sans capture en cours,
# wpr -cancel écrit sur stderr, ce que PowerShell 5.1 transformerait en erreur fatale : on passe par cmd.
cmd /c "wpr -cancel >nul 2>&1"
$profiles = @("-start", "CPU")
if ($WithIO) { $profiles += @("-start", "DiskIO", "-start", "FileIO") }
wpr @profiles -filemode
if ($LASTEXITCODE -ne 0) { throw "wpr n'a pas démarré (code $LASTEXITCODE)." }
$t0 = Get-Date
Write-Host ""
Write-Host "Capture en cours. Termine le tour dans le jeu, attends d'avoir de nouveau la main,"
Write-Host "puis appuie sur Entrée ici." -ForegroundColor Yellow
[void](Read-Host)
$t1 = Get-Date
wpr -stop $etl "Civ6 passage de tour $Name"
if ($LASTEXITCODE -ne 0) { throw "wpr n'a pas écrit la trace (code $LASTEXITCODE)." }

# Les journaux du jeu sont réécrits à chaque lancement : on garde la copie de cette session.
$logs = Join-Path $env:LOCALAPPDATA "Firaxis Games\Sid Meier's Civilization VI\Logs"
foreach ($f in "Profile.csv", "net_message_debug.log", "Lua.log", "Modding.log") {
    Copy-Item (Join-Path $logs $f) $dir -ErrorAction SilentlyContinue
}
@{ name = $Name; start = $t0.ToString("o"); stop = $t1.ToString("o"); with_io = [bool]$WithIO } |
    ConvertTo-Json | Set-Content -Encoding utf8 (Join-Path $dir "capture.json")
Write-Host "Capture écrite dans $dir"
