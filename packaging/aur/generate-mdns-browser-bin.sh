#!/usr/bin/env bash
# Copyright 2024-2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

version=$1
sha256sum=$2
sha256sum_exe=$3

if [[ -z "$version" || -z "$sha256sum" || -z "$sha256sum_exe" ]]; then
    echo "Usage: $0 <version> <sha256sum> <sha256sum_exe>" >&2
    exit 1
fi

cat <<EOF
# Maintainer: Valentin Batz <valentin.batz+archlinux@posteo.de>
pkgname=mdns-browser-bin
pkgver=$version
pkgrel=1
pkgdesc="A cross platform mDNS browsing app written in Rust using tauri and leptos "
arch=('x86_64')
url="https://github.com/hrzlgnm/mdns-browser"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'libsoup' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!emptydirs')
conflicts=('mdns-browser')
source_x86_64=("https://github.com/hrzlgnm/mdns-browser/releases/download/mdns-browser-v\$pkgver/mdns-browser_\${pkgver}_amd64.deb" "https://github.com/hrzlgnm/mdns-browser/releases/download/mdns-browser-v\$pkgver/mdns-browser_linux_x64")
sha256sums_x86_64=('$sha256sum' '$sha256sum_exe')
package() {
    tar -xz -f data.tar.gz -C "\${pkgdir}"
    install -Dm755 mdns-browser_linux_x64 "\${pkgdir}/usr/bin/mdns-browser"
}
EOF
