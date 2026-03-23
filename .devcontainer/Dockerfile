FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install Node.js 22 (matches CI)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Tauri Linux system dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Java 17 (required by the Android SDK tools)
RUN apt-get update && apt-get install -y openjdk-17-jdk \
    && rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64

# Install Android SDK command-line tools
ENV ANDROID_HOME=/opt/android-sdk
ENV NDK_HOME=${ANDROID_HOME}/ndk/27.2.12479018
ENV PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${PATH}"

RUN mkdir -p ${ANDROID_HOME}/cmdline-tools \
    && curl -o /tmp/cmdline-tools.zip \
        https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip \
    && unzip /tmp/cmdline-tools.zip -d ${ANDROID_HOME}/cmdline-tools \
    && mv ${ANDROID_HOME}/cmdline-tools/cmdline-tools ${ANDROID_HOME}/cmdline-tools/latest \
    && rm /tmp/cmdline-tools.zip \
    && yes | sdkmanager --licenses \
    && sdkmanager \
        "platform-tools" \
        "platforms;android-34" \
        "build-tools;34.0.0" \
        "ndk;27.2.12479018" \
    && chown -R vscode:vscode ${ANDROID_HOME}
