FROM ghcr.io/void-linux/void-glibc@sha256:433691c192e373ab9f328385798f9b2f837c7b4ed69ee2784889bfd6ea077409
RUN xbps-install -Syu || xbps-install -yu xbps \
    && xbps-install -yu \
    && xbps-install -y bash git rustup base-devel curl jq util-linux coreutils binutils bsdtar findutils \
    && xbps-remove -Ooy
LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
LABEL org.opencontainers.image.base.digest="sha256:433691c192e373ab9f328385798f9b2f837c7b4ed69ee2784889bfd6ea077409"
