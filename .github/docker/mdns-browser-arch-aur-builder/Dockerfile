# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

FROM archlinux:base-devel@sha256:0cce2e42862208217cb44f67feb93630e45300619d293601202d34bb6cf77d98
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm --needed \
    actionlint \
    appmenu-gtk-module \
    cargo \
    cargo-auditable \
    coreutils \
    desktop-file-utils \
    gdk-pixbuf2 \
    git \
    glib2 \
    gtk3 \
    jq \
    libappindicator-gtk3 \
    librsvg \
    namcap \
    openssh \
    rust \
    rust-wasm \
    shellcheck \
    webkit2gtk-4.1 \
    sudo \
    trunk \
    wget \
    libsoup \
    && useradd -m -G wheel -s /bin/bash runner \
    && echo '%wheel ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers \
    && pacman -Scc --noconfirm

LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Arch AUR Builder"
LABEL org.opencontainers.image.base.name="archlinux:base-devel"
