# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

FROM ghcr.io/void-linux/void-glibc@sha256:f5d88cafc86032345532fdeacaf0c84e6226c032bbfbe14130bbe0d9b11ec76c
RUN xbps-install -Syu xbps \
    && xbps-install -Syu \
    && xbps-install -Sy \
    base-devel \
    bash \
    binutils \
    bsdtar \
    coreutils \
    curl \
    findutils \
    git \
    jq \
    rustup \
    util-linux \
    xtools \
    && xbps-remove -Ooy
RUN rustup-init -y -q \
    && . $HOME/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && cargo --locked install trunk@0.21.14 --no-default-features --features rustls \
    && cargo --locked install tauri-cli@2.9.5 \
    && cargo --locked install cargo-auditable@0.7.2 \
    && rm -rf $HOME/.cargo/registry $HOME/.cargo/git
LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
