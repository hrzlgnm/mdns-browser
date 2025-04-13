#!/usr/bin/env bash
version=$1
sha256sum=$2

if [[ -z "$version" || -z "$sha256sum" ]]; then
    echo "Usage: $0 <version> <sha256sum>" >&2
    exit 1
fi

cat <<EOF
# Maintainer: Valentin Batz <valentin.batz+archlinux@posteo.de>
pkgname=mdns-browser-bin
pkgver=$version
pkgrel=1
pkgdesc="A cross platform mDNS-Browser app written in Rust using tauri and leptos "
arch=('x86_64')
url="https://github.com/hrzlgnm/mdns-browser"
license=('mit')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!emptydirs')
source_x86_64=("https://github.com/hrzlgnm/mdns-browser/releases/download/mdns-browser-v\$pkgver/mdns-browser_\${pkgver}_amd64.deb")
sha256sums_x86_64=('$sha256sum')
package() {
    tar -xz -f data.tar.gz -C "\${pkgdir}"
}
EOF
