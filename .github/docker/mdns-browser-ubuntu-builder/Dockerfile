# Copyright 2025 hrzlgnm
# SPDX-License-Identifier: MIT-0

FROM ubuntu:latest@sha256:3131b4cc82a783df6c9df078f86e01819a13594b865c2cad47bd1bca2b7063bb
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    file \
    git \
    jq \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    rustup \
    unzip \
    wget \
    && rm -rf /var/lib/apt/lists/*

LABEL org.opencontainers.image.source="https://github.com/hrzlgnm/mdns-browser"
LABEL org.opencontainers.image.title="mDNS-Browser Ubuntu Builder"
LABEL org.opencontainers.image.base.name="ubuntu:latest"
