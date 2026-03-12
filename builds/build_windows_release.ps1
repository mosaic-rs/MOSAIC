#This file is part of MOSAIC.
#
#MOSAIC is free software: you can redistribute it and/or modify it under 
#the terms of the GNU General Public License as published by the Free 
#Software Foundation, either version 3 of the License, or any later version.
#
#MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
#without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
#PURPOSE. See the GNU General Public License for more details.
#
#You should have received a copy of the GNU General Public License along with 
#MOSAIC. If not, see <https://www.gnu.org/licenses/>.

$ErrorActionPreference = 'Stop'

$SCRIPT_DIR = $PSScriptRoot
if ([string]::IsNullOrEmpty($SCRIPT_DIR)) { $SCRIPT_DIR = (Get-Location).Path }

$PROJECT_ROOT = (Get-Item "$SCRIPT_DIR\..").FullName
Set-Location $PROJECT_ROOT

if (Test-Path ".env") {
    Get-Content ".env" | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+)\s*=\s*(.*)$') {
            [Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim())
        }
    }
    Write-Host "[MOSAIC BUILD] Secrets loaded."
} else {
    Write-Host "[MOSAIC BUILD] WARNING: .env not found."
}

# Asking for python path - if empty then uses default
$PY_VER = "3.11"
$DEFAULT_PATH = "$env:LOCALAPPDATA\Programs\Python\Python311"

Write-Host "Please provide the path where Python 3.11.9 is installed."
Write-Host "It is likely: $DEFAULT_PATH"

$UserPath = Read-Host "Path (leave empty for default)"

if ([string]::IsNullOrWhiteSpace($UserPath)) {
    Write-Host "Using default path: $DEFAULT_PATH"
    $PY_PATH = $DEFAULT_PATH
} else {
    $PY_PATH = $UserPath
}

Write-Host "Python path set to: $PY_PATH"

$STAGING_DIR = "MOSAIC_STAGING"
$RESOURCES = "$STAGING_DIR\Resources"
$SITE_PACKAGES = "$RESOURCES\python_lib\site-packages"

$BINARY_NAME = "mosaic_engine.exe"

Write-Host "[MOSAIC] Removing old builds"
if (Test-Path $STAGING_DIR) { Remove-Item -Recurse -Force $STAGING_DIR }
if (Test-Path "MOSAIC_v0.3.3.zip") { Remove-Item -Force "MOSAIC_v0.3.3.zip" }

New-Item -ItemType Directory -Force -Path $STAGING_DIR | Out-Null
New-Item -ItemType Directory -Force -Path "$RESOURCES\python_lib\stdlib" | Out-Null
New-Item -ItemType Directory -Force -Path $SITE_PACKAGES | Out-Null
New-Item -ItemType Directory -Force -Path "$RESOURCES\python_lib\modules" | Out-Null

Write-Host "[MOSAIC] Compiling MOSAIC"
$env:PYO3_PYTHON = "$PY_PATH\python.exe"
Set-Location ".app-shell"
$env:RUSTFLAGS = "-A warnings"

cargo tauri build
Set-Location ..

Copy-Item ".app-shell\src-tauri\target\release\$BINARY_NAME" -Destination "$STAGING_DIR\"

Write-Host "Copying Python $PY_VER DLL"
$PY_VER_STR = $PY_VER -replace '\.', ''
Copy-Item "$PY_PATH\python$PY_VER_STR.dll" -Destination "$STAGING_DIR\"

Write-Host "[MOSAIC] Copying Python Library..."
$excludes = @("test", "tests", "site-packages", "__pycache__", "idlelib")
Get-ChildItem -Path "$PY_PATH\Lib" -Exclude $excludes | 
    Copy-Item -Destination "$RESOURCES\python_lib\stdlib\" -Recurse -Force

Write-Host "[MOSAIC] Installing Python Dependencies"
& "$PY_PATH\python.exe" -m pip install praat-parselmouth numpy `
    --target "$SITE_PACKAGES" `
    --no-user --upgrade --no-warn-script-location | Out-Null

Write-Host "[MOSAIC] Copying MOSAIC python scripts"
Copy-Item "src\praatAnalysis\*.py" -Destination "$RESOURCES\python_lib\modules\"

Write-Host "[MOSAIC] Preparing to make Distribution Archive"
Write-Host "[MOSAIC] Skipped code signing and notarization for Windows."

Write-Host "[MOSAIC] Creating Zip Archive"
Compress-Archive -Path "$STAGING_DIR\*" -DestinationPath "MOSAIC_v0.3.3.zip"

Remove-Item -Recurse -Force $STAGING_DIR

Write-Host "Build Complete: MOSAIC_v0.3.3.zip is ready for distribution."