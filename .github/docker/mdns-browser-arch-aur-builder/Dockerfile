# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT

FROM archlinux:base-devel@sha256:51dd3d24f7fba779e7c471caeee7804c50e8c134ad948e19685a1c83a42facc3
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm --needed \
    appmenu-gtk-module \
    cargo \
    cargo-auditable \
    coreutils \
    desktop-file-utils \
    gdk-pixbuf2 \
    git \
    glib2 \
    github-cli \
    gtk3 \
    jq \
    libappindicator-gtk3 \
    librsvg \
    namcap \
    nodejs \
    openssh \
    pnpm \
    rust \
    webkit2gtk-4.1 \
    sudo \
    wget \
    libsoup3 \
    && useradd -m -G wheel -s /bin/bash runner \
    && echo '%wheel ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers \
    && pacman -Scc --noconfirm

LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Arch AUR Builder"
LABEL org.opencontainers.image.base.name="archlinux:base-devel"
