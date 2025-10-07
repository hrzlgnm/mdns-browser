FROM archlinux:base-devel@sha256:b3809917ab5a7840d42237f5f92d92660cd036bd75ae343e7825e6a24401f166
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm \
    appmenu-gtk-module \
    cargo \
    coreutils \
    desktop-file-utils \
    gdk-pixbuf2 \
    git \
    glib2 \
    gtk3 \
    jq \
    libappindicator-gtk3 \
    librsvg \
    openssh \
    rustup \
    webkit2gtk-4.1 \
    sudo \
    wget \
    libsoup \
    && useradd -m -G wheel -s /bin/bash runner \
    && echo '%wheel ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers \
    && pacman -Scc --noconfirm

LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Arch AUR Builder"
LABEL org.opencontainers.image.base.name="archlinux:base-devel"
