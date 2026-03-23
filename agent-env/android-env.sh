#!/usr/bin/env bash
# android-env.sh — Export Android SDK / NDK / Java environment variables.
# Source this file before running any Android or Tauri-Android commands:
#   source agent-env/android-env.sh

# GitHub-hosted runners (ubuntu-latest) ship the SDK at this path.
# Devcontainer uses /opt/android-sdk — the fallback handles both.
export JAVA_HOME="${JAVA_HOME:-/usr/lib/jvm/temurin-17-jdk-amd64}"
export ANDROID_HOME="${ANDROID_HOME:-/usr/local/lib/android/sdk}"
# NDK 27.3 is the latest 27.x available on GitHub runners; devcontainer has 27.2.
export NDK_HOME="${NDK_HOME:-${ANDROID_HOME}/ndk/29.0.14206865}"

export PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${JAVA_HOME}/bin:${PATH}"
