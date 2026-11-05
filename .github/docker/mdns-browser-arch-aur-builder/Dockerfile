# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT

FROM archlinux:base-devel@sha256:767de6eaa40dcfeda5263a48737ea03a782caabdf3a403b6e6a203c7fd333681
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
