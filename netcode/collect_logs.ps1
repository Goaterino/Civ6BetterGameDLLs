# Rassemble ce qu'il faut pour analyser une désynchro, dans une archive sur le Bureau.
# Autonome : on peut l'envoyer tel quel à un autre joueur. À lancer jeu FERMÉ ou juste après la
# partie, et surtout AVANT de relancer Civ6, qui réécrit ses journaux à chaque démarrage :
#   powershell -ExecutionPolicy Bypass -File collect_logs.ps1
#   powershell -ExecutionPolicy Bypass -File collect_logs.ps1 -Extra netcode\runs\<dossier>
param(
    # Nombre d'autosaves multijoueur les plus récents à joindre.
    [int]$Saves = 6,
    # Dossier supplémentaire à joindre (par exemple le journal Frida de l'hôte).
    [string]$Extra = ""
)
$ErrorActionPreference = "Stop"

$local = Join-Path $env:LOCALAPPDATA "Firaxis Games\Sid Meier's Civilization VI"
$docs = Join-Path ([Environment]::GetFolderPath("MyDocuments")) "My Games\Sid Meier's Civilization VI"
$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$name = "civ6-desync-$env:COMPUTERNAME-$stamp"
$tmp = Join-Path $env:TEMP $name
New-Item -ItemType Directory -Force $tmp | Out-Null

Copy-Item (Join-Path $local "Logs") (Join-Path $tmp "Logs") -Recurse
foreach ($f in "AppOptions.txt", "Mods.sqlite") {
    Copy-Item (Join-Path $local $f) $tmp -ErrorAction SilentlyContinue
}
$auto = Join-Path $docs "Saves\Multi\auto"
if (Test-Path $auto) {
    $dest = New-Item -ItemType Directory -Force (Join-Path $tmp "autosaves")
    Get-ChildItem $auto -Filter *.Civ6Save | Sort-Object LastWriteTime -Descending |
        Select-Object -First $Saves | Copy-Item -Destination $dest
}
if ($Extra -and (Test-Path $Extra)) {
    Copy-Item $Extra (Join-Path $tmp "extra") -Recurse
}
$proc = Get-Process | Where-Object { $_.Name -like "CivilizationVI*" }
@{
    machine = $env:COMPUTERNAME
    collecte = (Get-Date).ToString("o")
    jeu_ouvert = [bool]$proc
    fuseau = [TimeZoneInfo]::Local.Id
} | ConvertTo-Json | Set-Content -Encoding utf8 (Join-Path $tmp "collecte.json")

$zip = Join-Path ([Environment]::GetFolderPath("Desktop")) "$name.zip"
Compress-Archive -Path (Join-Path $tmp "*") -DestinationPath $zip
Remove-Item $tmp -Recurse -Force
Write-Host "Archive écrite : $zip"
