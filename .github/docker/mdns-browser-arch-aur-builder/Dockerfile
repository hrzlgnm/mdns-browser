# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

FROM archlinux:base-devel@sha256:233f52197321141f63c1d389284c868101dc94ef8f46ac31b908e3e4b6ea7cb5
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
