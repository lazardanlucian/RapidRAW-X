#!/usr/bin/env bash
# build-android.sh — Build RapidRAW-X for Android.
#
# Usage:
#   bash build-android.sh                   # release build, all ABIs
#   bash build-android.sh --debug           # debug build
#   bash build-android.sh --target aarch64  # single ABI (aarch64 | armv7 | x86_64 | x86)
#   bash build-android.sh --debug --target aarch64
#
# Output is copied to build/android/.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ── Source Android environment ──────────────────────────────────────────────
source "${REPO_ROOT}/agent-env/android-env.sh"

# ── Parse arguments ──────────────────────────────────────────────────────────
BUILD_TYPE="release"
TARGET=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug)   BUILD_TYPE="debug"; shift ;;
    --target)  TARGET="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

# ── Validate required tools ──────────────────────────────────────────────────
for tool in java sdkmanager rustup npm node; do
  if ! command -v "$tool" &>/dev/null; then
    echo "ERROR: '$tool' not found. Run 'bash agent-env/setup.sh' first." >&2
    exit 1
  fi
done

echo "==> Java:  $(java -version 2>&1 | head -1)"
echo "==> Node:  $(node --version)"
echo "==> Cargo: $(cargo --version)"

# ── npm install (idempotent) ─────────────────────────────────────────────────
cd "${REPO_ROOT}"
if [[ ! -d node_modules ]]; then
  echo "==> Installing npm dependencies..."
  npm install
fi

# ── Tauri Android init (first-time only) ────────────────────────────────────
if [[ ! -d "${REPO_ROOT}/gen/android" ]]; then
  echo "==> Initialising Tauri Android project (first run)..."
  npm run tauri android init
fi

# ── Build ────────────────────────────────────────────────────────────────────
TAURI_ARGS=("android" "build")

if [[ "${BUILD_TYPE}" == "debug" ]]; then
  TAURI_ARGS+=("--debug")
fi

if [[ -n "${TARGET}" ]]; then
  TAURI_ARGS+=("--target" "${TARGET}")
fi

echo "==> Building Android (${BUILD_TYPE}${TARGET:+, target=${TARGET}})..."
npm run tauri -- "${TAURI_ARGS[@]}"

# ── Collect outputs ──────────────────────────────────────────────────────────
OUT_DIR="${REPO_ROOT}/build/android"
mkdir -p "${OUT_DIR}"

echo "==> Copying build artefacts to ${OUT_DIR}..."

# APKs (debug or release split / universal)
find "${REPO_ROOT}/gen/android/app/build/outputs" \
  \( -name "*.apk" -o -name "*.aab" \) \
  -exec cp -v {} "${OUT_DIR}/" \;

echo ""
echo "==> Done. Artefacts:"
ls -lh "${OUT_DIR}"/ 2>/dev/null || echo "  (none found in build/android/ — the Gradle output path may differ; check gen/android/app/build/outputs/ directly)"
