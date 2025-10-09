FROM archlinux:base-devel@sha256:06ab929f935145dd65994a89dd06651669ea28d43c812f3e24de990978511821
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
