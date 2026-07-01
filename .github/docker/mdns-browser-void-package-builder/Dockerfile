# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

FROM ghcr.io/void-linux/void-glibc@sha256:4fe098653419a41d7b733394061807a01b46b0e4b808a83dee76349b42aecc38
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
    && cargo --locked install tauri-cli@2.11.4 \
    && cargo --locked install cargo-auditable@0.7.5 \
    && rm -rf $HOME/.cargo/registry $HOME/.cargo/git
LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
