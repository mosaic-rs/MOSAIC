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
PY_PATH="/Library/Frameworks/Python.framework/Versions/$PY_VER"

APP_BUNDLE="MOSAIC.app"
CONTENTS="$APP_BUNDLE/Contents"
MACOS="$CONTENTS/MacOS"
FRAMEWORKS="$CONTENTS/Frameworks"
RESOURCES="$CONTENTS/Resources"
SITE_PACKAGES="$RESOURCES/python_lib/site-packages"

BINARY_NAME="mosaic_engine"

echo "[MOSAIC] Removing old builds"
rm -rf "$APP_BUNDLE" "MOSAIC_DMG_STAGING"
mkdir -p "$MACOS"
mkdir -p "$FRAMEWORKS"
mkdir -p "$RESOURCES/python_lib/stdlib"
mkdir -p "$SITE_PACKAGES"
mkdir -p "$RESOURCES/python_lib/modules"

echo "[MOSAIC] Compiling MOSAIC"
export PYO3_PYTHON="$PY_PATH/bin/python$PY_VER"
RUSTFLAGS="-A warnings" cargo build --release
cp "target/release/MOSAIC" "$MACOS/$BINARY_NAME"

echo "[MOSAIC] App metadata"
cp "Info.plist" "$CONTENTS/Info.plist"

echo "Copying python 3.11.9 to /libpython"
cp "$PY_PATH/Python" "$FRAMEWORKS/libpython$PY_VER.dylib"

install_name_tool -id "@executable_path/../Frameworks/libpython$PY_VER.dylib" "$FRAMEWORKS/libpython$PY_VER.dylib"

echo "[MOSAIC] Copying Python Library..."
rsync -a "$PY_PATH/lib/python$PY_VER/" "$RESOURCES/python_lib/stdlib/" \
    --exclude 'test' --exclude 'tests' --exclude 'site-packages' --exclude '__pycache__'

echo "[MOSAIC] Installing Python Dependencies"
"$PY_PATH/bin/python$PY_VER" -m pip install \
    praat-parselmouth numpy \
    --target "$SITE_PACKAGES" \
    --no-user --upgrade --no-warn-script-location > /dev/null

echo "[MOSAIC] Copying MOSAIC python scripts"
cp src/praatAnalysis/*.py "$RESOURCES/python_lib/modules/"

install_name_tool -change \
    "$PY_PATH/Python" \
    "@executable_path/../Frameworks/libpython$PY_VER.dylib" \
    "$MACOS/$BINARY_NAME"

echo "[MOSAIC] Signing App Bundle"
if [ -z "$APPLE_SIGNING_IDENTITY" ]; then
    echo "Skipping signing (Identity not found)"
else
    codesign --force --timestamp --options runtime --sign "$APPLE_SIGNING_IDENTITY" "$FRAMEWORKS/libpython$PY_VER.dylib"
    
    find "$SITE_PACKAGES" -name "*.so" -exec \
        codesign --force --timestamp --options runtime --sign "$APPLE_SIGNING_IDENTITY" {} \;

    codesign --force --timestamp --options runtime \
        --entitlements "entitlements.plist" \
        --sign "$APPLE_SIGNING_IDENTITY" \
        "$MACOS/$BINARY_NAME"
        
    codesign --force --timestamp --options runtime --sign "$APPLE_SIGNING_IDENTITY" "$APP_BUNDLE"
fi

echo "[MOSAIC] Preparing to make DMG"
mkdir -p "MOSAIC_DMG_STAGING"
cp -R "$APP_BUNDLE" "MOSAIC_DMG_STAGING/"
ln -s /Applications "MOSAIC_DMG_STAGING/Applications"

echo "[MOSAIC] Creating Disk Image"
rm -f "MOSAIC_v0.3.3.dmg"
hdiutil create -volname "MOSAIC-Installer" \
               -srcfolder "MOSAIC_DMG_STAGING" \
               -ov -format UDZO \
               "MOSAIC_v0.3.3.dmg"

rm -rf "MOSAIC_DMG_STAGING"

echo "Build Complete: MOSAIC_v0.3.3.dmg"