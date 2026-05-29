$ErrorActionPreference = "Stop"

$repoRoot = $PWD
$manifestPath = Join-Path $repoRoot "vendor\cache\windows\latest.env"
$vendorDir = Join-Path $repoRoot "vendor\windows"
$extractDir = Join-Path $vendorDir "Sentinel-LDK_SDK"

if (-not (Test-Path $manifestPath)) {
  throw "Missing $manifestPath; run ./scripts/download-sentinel-sdk.sh windows first"
}

$archivePath = $null
Get-Content $manifestPath | ForEach-Object {
  if ($_ -match "^ARCHIVE_PATH=(.+)$") {
    $archivePath = Join-Path $repoRoot $Matches[1]
  }
}

if (-not $archivePath -or -not (Test-Path $archivePath)) {
  throw "Archive not found; run ./scripts/download-sentinel-sdk.sh windows first"
}

New-Item `
  -ItemType Directory `
  -Force `
  -Path $vendorDir | Out-Null

$needsExpand = -not (Test-Path $extractDir)
if (-not $needsExpand) {
  $archiveTime = (Get-Item $archivePath).LastWriteTimeUtc
  $extractTime = (Get-Item $extractDir).LastWriteTimeUtc
  $needsExpand = $archiveTime -gt $extractTime
}

if ($needsExpand) {
  Write-Host "Extracting SDK from $archivePath..."

  if (Test-Path $extractDir) {
    Remove-Item `
      -Recurse `
      -Force `
      $extractDir
  }

  Expand-Archive `
    -Path $archivePath `
    -DestinationPath $vendorDir `
    -Force
}
else {
  Write-Host "Extracted SDK already up to date"
}

$msiPaths = @(
  "Windows\subsetup\Sentinel Vendor Suite.msi"
  "Windows\subsetup\Sentinel Runtime.msi"
)
foreach ($msiPath in $msiPaths) {
  Write-Host "Installing $($msiPath)"

  $setupPath = Join-Path $extractDir $msiPath
  if (-not (Test-Path $setupPath)) {
    throw "Installer not found: $setupPath"
  }

  $process = Start-Process `
    -FilePath "msiexec.exe" `
    -ArgumentList "/i `"$setupPath`" /qn" `
    -Wait `
    -PassThru

  if ($process.ExitCode -ne 0) {
    throw "Installer failed with exit code $($process.ExitCode)"
  }
}

Write-Host "Installing license file"
# Wait for LDK to run
sleep 10

$licensePath = Join-Path $repoRoot "licenses\Unlocked_20260529_182601.v2c"

Start-Process `
  -FilePath "curl.exe" `
  -ArgumentList "-F `"check_in_file=@$licensePath`" `"http://localhost:1947/_int_/checkin_file.html`"" `
  -Wait `
  -PassThru

$sdkDir = Join-Path ${env:ProgramFiles(x86)} "Thales\Sentinel LDK"
$dllDir = Join-Path $sdkDir "API\Licensing\C\x64"

"PATH=$dllDir;$env:PATH" >> $env:GITHUB_ENV
"SENTINEL_LDK_SDK_DIR=$sdkDir" >> $env:GITHUB_ENV
