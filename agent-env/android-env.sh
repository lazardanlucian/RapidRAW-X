#!/usr/bin/env bash
# android-env.sh — Export Android SDK / NDK / Java environment variables.
# Source this file before running any Android or Tauri-Android commands:
#   source agent-env/android-env.sh

export JAVA_HOME="${JAVA_HOME:-/usr/lib/jvm/java-17-openjdk-amd64}"
export ANDROID_HOME="${ANDROID_HOME:-/opt/android-sdk}"
export NDK_HOME="${NDK_HOME:-${ANDROID_HOME}/ndk/27.2.12479018}"

export PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${JAVA_HOME}/bin:${PATH}"
