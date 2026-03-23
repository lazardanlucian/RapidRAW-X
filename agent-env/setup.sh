#!/usr/bin/env bash
# setup.sh — One-shot dev-environment bootstrap for Android porting.
# Run once after cloning (or after the devcontainer postCreateCommand).
# Usage:  bash agent-env/setup.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/android-env.sh"

echo "==> Checking Java..."
java -version

echo "==> Checking Android SDK manager..."
sdkmanager --version

echo "==> Adding Rust Android cross-compilation targets..."
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android

echo "==> Installing npm dependencies..."
cd "${SCRIPT_DIR}/.."
npm install

echo ""
echo "==> Setup complete. You can now run:"
echo "      bash build-android.sh          # build a release APK/AAB"
echo "      npm run tauri android dev      # run on a connected device or emulator"
