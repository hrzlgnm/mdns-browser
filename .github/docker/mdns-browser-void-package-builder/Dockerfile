FROM ghcr.io/void-linux/void-glibc@sha256:7816cc9f516017282a0ebc93fe23ded9fc5208cc243f9188f0f75ab6d3e3d62f
RUN xbps-install -Syu || xbps-install -yu xbps \
    && xbps-install -yu \
    && xbps-install -y bash git rustup base-devel curl jq util-linux coreutils binutils bsdtar findutils \
    && xbps-remove -Ooy
RUN rustup-init -y -q \
    && . $HOME/.cargo/env \
    && rustup target add wasm32-unknown-unknown \
    && cargo --locked install trunk@0.21.14 --no-default-features --features rustls \
    && cargo --locked install tauri-cli@2.5.0 \
    && cargo --locked install cargo-auditable@0.6.7 \
    && rm -rf $HOME/.cargo/registry $HOME/.cargo/git
LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Void Package Builder"
LABEL org.opencontainers.image.base.name="ghcr.io/void-linux/void-glibc"
