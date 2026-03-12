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

#!/bin/bash
set -e


SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
cd "$PROJECT_ROOT"

if [ -f .env ]; then
    set -a; source .env; set +a
    echo "[MOSAIC BUILD] Secrets loaded."
else
    echo "[MOSAIC BUILD] WARNING: .env not found."
fi

PY_VER="3.11"
DEFAULT_BIN="/usr/bin/python$PY_VER"

echo "Please provide the path to your Python $PY_VER executable."
echo "It is likely: $DEFAULT_BIN"

read -r UserPath

if [[ -z "$UserPath" ]]; then
    echo "Using default path: $DEFAULT_BIN"
    PY_BIN="$DEFAULT_BIN"
else
    PY_BIN="$UserPath"
fi

if [ ! -x "$PY_BIN" ]; then
    echo "ERROR: Python executable not found at $PY_BIN"
    exit 1
fi

echo "Python executable set to: $PY_BIN"

PY_LIBDIR=$("$PY_BIN" -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
PY_LDLIBRARY=$("$PY_BIN" -c "import sysconfig; print(sysconfig.get_config_var('LDLIBRARY'))")
PY_STDLIB=$("$PY_BIN" -c "import sysconfig; print(sysconfig.get_path('stdlib'))")

STAGING_DIR="MOSAIC_STAGING"
RESOURCES="$STAGING_DIR/Resources"
SITE_PACKAGES="$RESOURCES/python_lib/site-packages"

BINARY_NAME="mosaic_engine"

echo "[MOSAIC] Removing old builds"
rm -rf "$STAGING_DIR" "MOSAIC_v0.3.3.tar.gz"
mkdir -p "$STAGING_DIR"
mkdir -p "$RESOURCES/python_lib/stdlib"
mkdir -p "$SITE_PACKAGES"
mkdir -p "$RESOURCES/python_lib/modules"

echo "[MOSAIC] Compiling MOSAIC"
export PYO3_PYTHON="$PY_BIN"
cd .app-shell
RUSTFLAGS="-A warnings" cargo tauri build
cd ..

cp ".app-shell/src-tauri/target/release/$BINARY_NAME" "$STAGING_DIR/$BINARY_NAME"

echo "Copying Python shared library ($PY_LDLIBRARY)"
cp "$PY_LIBDIR/$PY_LDLIBRARY" "$STAGING_DIR/$PY_LDLIBRARY"

echo "[MOSAIC] Patching binary to look for Python in its local directory..."
patchelf --set-rpath '$ORIGIN' "$STAGING_DIR/$BINARY_NAME"

echo "[MOSAIC] Copying Python Library..."
rsync -a "$PY_STDLIB/" "$RESOURCES/python_lib/stdlib/" \
    --exclude 'test' \
    --exclude 'tests' \
    --exclude 'site-packages' \
    --exclude '__pycache__' \
    --exclude 'idlelib' \
    --exclude '*.a'

echo "[MOSAIC] Installing Python Dependencies"
"$PY_BIN" -m pip install \
    praat-parselmouth numpy \
    --target "$SITE_PACKAGES" \
    --no-user --upgrade --no-warn-script-location > /dev/null

echo "[MOSAIC] Copying MOSAIC python scripts"
cp src/praatAnalysis/*.py "$RESOURCES/python_lib/modules/"

echo "[MOSAIC] Skipped Apple code signing and notarization for Linux."

echo "[MOSAIC] Preparing to make Distribution Archive"
echo "[MOSAIC] Creating Tarball (tar.gz)"

tar -czvf "MOSAIC_v0.3.3.tar.gz" -C "$STAGING_DIR" .

rm -rf "$STAGING_DIR"

echo "Build Complete: MOSAIC_v0.3.3.tar.gz is ready for distribution."