# Android Porting — agent-env

This directory contains the environment configuration and setup scripts needed
to build RapidRAW-X for Android.

## Directory layout

| File | Purpose |
|------|---------|
| `android-env.sh` | Exports `ANDROID_HOME`, `NDK_HOME`, `JAVA_HOME`, and `PATH`. Source this before any Android build command. |
| `setup.sh` | One-shot bootstrap: validates the SDK, adds Rust Android targets, runs `npm install`. |

## Prerequisites

The repo's **devcontainer** (`.devcontainer/`) already provisions everything
you need (Android SDK 34, NDK 27.2, Java 17, Rust 1.94). Open the repo in a
GitHub Codespace or a local Dev Container and the environment is ready.

If you are working outside a devcontainer, install:

```
Java 17            → /usr/lib/jvm/java-17-openjdk-amd64
Android SDK        → /opt/android-sdk  (cmdline-tools, platform-tools, platforms;android-34, build-tools;34.0.0, ndk;27.2.12479018)
Rust 1.94          → https://rustup.rs
Node 22 + npm      → https://nodejs.org
```

Then run:

```bash
bash agent-env/setup.sh
```

## Building

```bash
# Full release build (APK + AAB, all ABIs)
bash build-android.sh

# Debug / fast iteration
bash build-android.sh --debug

# Specific ABI only (aarch64 | armv7 | x86_64 | x86)
bash build-android.sh --target aarch64

# Live-reload on a connected device or emulator
npm run tauri android dev
```

Output APKs and AABs are written to `build/android/`.

## Known porting status

| Area | Status | Notes |
|------|--------|-------|
| Tauri 2 Android init | ✅ scripted | `build-android.sh` runs `tauri android init` on first use |
| Rust compilation | ✅ targets added | `aarch64`, `armv7`, `x86_64`, `x86` |
| GPU (wgpu) | ⚠️ needs validation | wgpu supports Vulkan/GLES; test on device |
| ONNX Runtime | ⚠️ needs Android `.so` | See `build.rs` note; add Android ONNX artefacts when available |
| `tauri-plugin-single-instance` | ❌ desktop-only | Disabled via `tauri.android.conf.json` (future) |
| `trash` crate | ⚠️ may need stub | Android has no system trash; evaluate at runtime |
