# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.19.0...HEAD)

### Added

- Gzip the SBOM workflow artifacts on publish ([#2527](https://github.com/hrzlgnm/mdns-browser/pull/2527))

- Trigger snap release after a release is published ([#2529](https://github.com/hrzlgnm/mdns-browser/pull/2529))

### Changed

- Add Snap Store badge and installation instructions ([#2528](https://github.com/hrzlgnm/mdns-browser/pull/2528))

- Improve AGENTS.md efficiency (dedup, prune, progressive disclosure) ([#2532](https://github.com/hrzlgnm/mdns-browser/pull/2532))

- Cleanup some unused files and ignore settings ([#2533](https://github.com/hrzlgnm/mdns-browser/pull/2533))

### Fixed

- Disable grype DB caching in sbom composite ([#2531](https://github.com/hrzlgnm/mdns-browser/pull/2531))

## [1.19.0] - 2026-08-25 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.18.1...v1.19.0)

### Added

- Add env-var debug trace to webkit2gtk-nvidia-quirk ([#2524](https://github.com/hrzlgnm/mdns-browser/pull/2524))

### Changed

- Require two-axis code review before completion ([#2525](https://github.com/hrzlgnm/mdns-browser/pull/2525))

- *(webkit2gtk-nvidia-quirk)* Bump webkit2gtk-nvidia-quirk to 2.1.0

- Point README to webkit2gtk-nvidia-quirk tracing docs ([#2526](https://github.com/hrzlgnm/mdns-browser/pull/2526))

### Dependencies

- *(deps)* Update rust crate log to v0.4.34 ([#2515](https://github.com/hrzlgnm/mdns-browser/pull/2515))

- *(deps)* Lock file maintenance ([#2517](https://github.com/hrzlgnm/mdns-browser/pull/2517))

- *(deps)* Update archlinux:base-devel docker digest to 68bfc3b ([#2519](https://github.com/hrzlgnm/mdns-browser/pull/2519))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 3bb1a10 ([#2522](https://github.com/hrzlgnm/mdns-browser/pull/2522))

- *(deps)* Update actions/setup-java action to v6 ([#2520](https://github.com/hrzlgnm/mdns-browser/pull/2520))

### Fixed

- Disable coderabbit request changes workflow ([#2518](https://github.com/hrzlgnm/mdns-browser/pull/2518))

### Maintenance

- *(ci)* Drop unused tagName output from crate publish workflows ([#2512](https://github.com/hrzlgnm/mdns-browser/pull/2512))

- *(ci)* Harden crate-publish reusable workflow from review ([#2514](https://github.com/hrzlgnm/mdns-browser/pull/2514))

- Drop unneeded actions: write from release draft job ([#2523](https://github.com/hrzlgnm/mdns-browser/pull/2523))

## [1.18.1] - 2026-08-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.18.0...v1.18.1)

### Added

- Add tauri-gh-android-update plugin crate ([#2489](https://github.com/hrzlgnm/mdns-browser/pull/2489))

### Changed

- *(webkit2gtk-nvidia-quirk)* Bump webkit2gtk-nvidia-quirk to 2.0.0

- Migrate mobile updater to tauri-gh-android-update plugin ([#2498](https://github.com/hrzlgnm/mdns-browser/pull/2498))

- Register updater commands as namespaced plugin commands ([#2500](https://github.com/hrzlgnm/mdns-browser/pull/2500))

- *(tauri-plugin-android-update)* Bump tauri-plugin-android-update to 0.1.1

- *(tauri-plugin-android-update)* Bump tauri-plugin-android-update to 0.1.2

### Dependencies

- *(deps)* Update ubuntu:latest docker digest to 4b92853 ([#2493](https://github.com/hrzlgnm/mdns-browser/pull/2493))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to ccf3ff2 ([#2495](https://github.com/hrzlgnm/mdns-browser/pull/2495))

- *(deps)* Update ubuntu:latest docker digest to 6df9e8d ([#2496](https://github.com/hrzlgnm/mdns-browser/pull/2496))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to b7ad4e1 ([#2497](https://github.com/hrzlgnm/mdns-browser/pull/2497))

- *(deps)* Update ubuntu:latest docker digest to 2260313 ([#2501](https://github.com/hrzlgnm/mdns-browser/pull/2501))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to ac1df04 ([#2503](https://github.com/hrzlgnm/mdns-browser/pull/2503))

### Fixed

- Route workflow run inputs and secrets via env ([#2505](https://github.com/hrzlgnm/mdns-browser/pull/2505))

- Regenerate stale permissions schema for tauri-plugin-android-update ([#2509](https://github.com/hrzlgnm/mdns-browser/pull/2509))

- *(tauri-plugin-android-update)* Render MIT-0 license badge correctly ([#2510](https://github.com/hrzlgnm/mdns-browser/pull/2510))

### Maintenance

- Add publish workflow for tauri-plugin-android-update crate ([#2504](https://github.com/hrzlgnm/mdns-browser/pull/2504))

## [1.18.0] - 2026-08-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.17.0...v1.18.0)

### Added

- Make workaround diagnostic output opt-in (verbose arg) ([#2481](https://github.com/hrzlgnm/mdns-browser/pull/2481))

- Add opt-in verbosity flag for NVIDIA workarounds ([#2484](https://github.com/hrzlgnm/mdns-browser/pull/2484))

### Changed

- *(release)* Surface webkit2gtk-nvidia-quirk bump in changelog ([#2482](https://github.com/hrzlgnm/mdns-browser/pull/2482))

- Adopt Git/AI-attribution and engineering-judgment conventions from zux ([#2485](https://github.com/hrzlgnm/mdns-browser/pull/2485))

- Document vX.Y.Z release tag format in AGENTS.md ([#2486](https://github.com/hrzlgnm/mdns-browser/pull/2486))

### Fixed

- *(webkit2gtk-nvidia-quirk)* Disable WebKit DMABUF renderer on Hyprland ([#2480](https://github.com/hrzlgnm/mdns-browser/pull/2480))

### Maintenance

- *(ci)* Set up Rust toolchain before cargo-install git-cliff ([#2487](https://github.com/hrzlgnm/mdns-browser/pull/2487))

- *(ci)* Drop Void Linux release pipeline and packaging ([#2488](https://github.com/hrzlgnm/mdns-browser/pull/2488))

## [1.17.0] - 2026-08-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.16.1...v1.17.0)

### Dependencies

- *(deps)* Lock file maintenance ([#2474](https://github.com/hrzlgnm/mdns-browser/pull/2474))

- *(deps)* Update archlinux:base-devel docker digest to 714acd1 ([#2477](https://github.com/hrzlgnm/mdns-browser/pull/2477))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 8eda1e6 ([#2478](https://github.com/hrzlgnm/mdns-browser/pull/2478))

### Fixed

- Publish unbundled macOS executable on release ([#2476](https://github.com/hrzlgnm/mdns-browser/pull/2476))

- Borderless on tiling Wayland, working buttons on non-tiling ([#2475](https://github.com/hrzlgnm/mdns-browser/pull/2475))

## [1.16.1] - 2026-08-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.16.0...v1.16.1)

### Changed

- Clarify cargo fmt covers formatting ([#2472](https://github.com/hrzlgnm/mdns-browser/pull/2472))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 28b4f11 ([#2468](https://github.com/hrzlgnm/mdns-browser/pull/2468))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 2247606 ([#2469](https://github.com/hrzlgnm/mdns-browser/pull/2469))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to e03c7a7 ([#2470](https://github.com/hrzlgnm/mdns-browser/pull/2470))

### Fixed

- *(webkit2gtk-nvidia-quirk)* Scope explicit sync workaround to hyprland ([#2471](https://github.com/hrzlgnm/mdns-browser/pull/2471))

## [1.16.0] - 2026-08-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.15.3...v1.16.0)

### Changed

- Document immutable releases ([#2455](https://github.com/hrzlgnm/mdns-browser/pull/2455))

- Adjust immutable releases wording ([#2456](https://github.com/hrzlgnm/mdns-browser/pull/2456))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to ee205c2 ([#2458](https://github.com/hrzlgnm/mdns-browser/pull/2458))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 7060a98 ([#2460](https://github.com/hrzlgnm/mdns-browser/pull/2460))

- *(deps)* Update rust crate futures to v0.3.34 ([#2461](https://github.com/hrzlgnm/mdns-browser/pull/2461))

- *(deps)* Update rust crate mdns-sd to 0.21 ([#2462](https://github.com/hrzlgnm/mdns-browser/pull/2462))

- *(deps)* Update hrzlgnm/actions action to v2.6.0 ([#2465](https://github.com/hrzlgnm/mdns-browser/pull/2465))

### Maintenance

- Trigger bot PR workflow approval on pull_request to main ([#2459](https://github.com/hrzlgnm/mdns-browser/pull/2459))

- Approve bot PR workflow runs via reusable workflow ([#2464](https://github.com/hrzlgnm/mdns-browser/pull/2464))

## [1.15.3] - 2026-08-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.15.2...v1.15.3)

### Changed

- Cross-link zux graph visualizer ([#2450](https://github.com/hrzlgnm/mdns-browser/pull/2450))

- Remove can_browse and network banner ([#2454](https://github.com/hrzlgnm/mdns-browser/pull/2454))

### Dependencies

- *(deps)* Update rust crate serde_with to v3.22.0 ([#2451](https://github.com/hrzlgnm/mdns-browser/pull/2451))

- *(deps)* Lock file maintenance ([#2452](https://github.com/hrzlgnm/mdns-browser/pull/2452))

## [1.15.2] - 2026-08-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.15.1...v1.15.2)

### Changed

- Add FlatPak install method via FlatPark and version badge ([#2449](https://github.com/hrzlgnm/mdns-browser/pull/2449))

### Fixed

- *(webkit2gtk-nvidia-quirk)* Detect NVIDIA driver without /sys/module or XDG_SESSION_TYPE ([#2448](https://github.com/hrzlgnm/mdns-browser/pull/2448))

## [1.15.1] - 2026-08-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/v1.15.0...v1.15.1)

### Added

- *(aur)* Add local helper to test AUR packaging against latest release ([#2440](https://github.com/hrzlgnm/mdns-browser/pull/2440))

### Changed

- *(aur)* Remove local test script moving to dotfiles repo ([#2442](https://github.com/hrzlgnm/mdns-browser/pull/2442))

- Document AUR packaging tests ([#2443](https://github.com/hrzlgnm/mdns-browser/pull/2443))

- Replace tauri-plugin-http with a direct reqwest dependency ([#2444](https://github.com/hrzlgnm/mdns-browser/pull/2444))

### Dependencies

- *(deps)* Lock file maintenance ([#2435](https://github.com/hrzlgnm/mdns-browser/pull/2435))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to a767676 ([#2437](https://github.com/hrzlgnm/mdns-browser/pull/2437))

### Fixed

- *(release)* Order release notes sections consistently ([#2433](https://github.com/hrzlgnm/mdns-browser/pull/2433))

- *(aur)* Sync release version into Cargo and tauri config ([#2434](https://github.com/hrzlgnm/mdns-browser/pull/2434))

- Retry browse when mdns-sd command queue is full ([#2438](https://github.com/hrzlgnm/mdns-browser/pull/2438))

- *(aur)* Use correct build directory in aur source build ([#2439](https://github.com/hrzlgnm/mdns-browser/pull/2439))

### Maintenance

- *(ci)* Always sbom scan and cache db when run on main ([#2436](https://github.com/hrzlgnm/mdns-browser/pull/2436))

## [1.15.0] - 2026-08-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.14.1...v1.15.0)

### Added

- *(cliff)* Move CI-related commits into dedicated Maintenance group ([#2424](https://github.com/hrzlgnm/mdns-browser/pull/2424))

### Changed

- Don't duplicate license file in bundling ([#2420](https://github.com/hrzlgnm/mdns-browser/pull/2420))

- Update spelling error replacement ([#2427](https://github.com/hrzlgnm/mdns-browser/pull/2427))

### Dependencies

- *(deps)* Update rust crate clap to v4.6.6 ([#2413](https://github.com/hrzlgnm/mdns-browser/pull/2413))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 97562a8 ([#2423](https://github.com/hrzlgnm/mdns-browser/pull/2423))

- *(deps)* Update rust crate thiserror to v2.0.20 ([#2428](https://github.com/hrzlgnm/mdns-browser/pull/2428))

### Fixed

- Strip sha256: prefix from checksum ([#2415](https://github.com/hrzlgnm/mdns-browser/pull/2415))

- Strip duplicate (#NNN) PR refs in generated changelog ([#2430](https://github.com/hrzlgnm/mdns-browser/pull/2430))

- *(release)* Remove incorrect --tag flag from git-cliff changelog generation ([#2431](https://github.com/hrzlgnm/mdns-browser/pull/2431))

- *(release)* Escape [bot] brackets and link bot authors in release notes ([#2432](https://github.com/hrzlgnm/mdns-browser/pull/2432))

### Maintenance

- *(ci)* Auto-approve workflow runs of github-actions[bot] PRs ([#2417](https://github.com/hrzlgnm/mdns-browser/pull/2417))

- *(ci)* Remove stray lines causing github-script syntax error ([#2418](https://github.com/hrzlgnm/mdns-browser/pull/2418))

- *(ci)* Publish releases on tag push and switch to vX.Y.Z tags ([#2421](https://github.com/hrzlgnm/mdns-browser/pull/2421))

- *(ci)* Generate release notes using GitHub username (@hrzlgnm) ([#2422](https://github.com/hrzlgnm/mdns-browser/pull/2422))

- *(ci)* Resolve GitHub username from noreply email in release notes ([#2425](https://github.com/hrzlgnm/mdns-browser/pull/2425))

## [1.14.1] - 2026-08-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.14.0...mdns-browser-v1.14.1)

### Changed

- Add github-cli to AUR builder Dockerfile ([#2406](https://github.com/hrzlgnm/mdns-browser/pull/2406))

- Use GitHub API checksums for release assets ([#2405](https://github.com/hrzlgnm/mdns-browser/pull/2405))

- Update release workflow input description ([#2408](https://github.com/hrzlgnm/mdns-browser/pull/2408))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to de0c834 ([#2407](https://github.com/hrzlgnm/mdns-browser/pull/2407))

- *(deps)* Update swatinem/rust-cache digest to 6323deb ([#2412](https://github.com/hrzlgnm/mdns-browser/pull/2412))

### Fixed

- Prevent mobile update prompts for older releases ([#2404](https://github.com/hrzlgnm/mdns-browser/pull/2404))

- Restore desktop download-and-install update label ([#2411](https://github.com/hrzlgnm/mdns-browser/pull/2411))

## [1.14.0] - 2026-08-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.13.4...mdns-browser-v1.14.0)

### Added

- Add selectable network interfaces ([#2403](https://github.com/hrzlgnm/mdns-browser/pull/2403))

### Dependencies

- *(deps)* Lock file maintenance ([#2387](https://github.com/hrzlgnm/mdns-browser/pull/2387))

- *(deps)* Update hrzlgnm/actions action to v2.5.5 ([#2389](https://github.com/hrzlgnm/mdns-browser/pull/2389))

- *(deps)* Update archlinux:base-devel docker digest to c1829f3 ([#2390](https://github.com/hrzlgnm/mdns-browser/pull/2390))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 2e59a58 ([#2391](https://github.com/hrzlgnm/mdns-browser/pull/2391))

- *(deps)* Update ubuntu:latest docker digest to 678c655 ([#2393](https://github.com/hrzlgnm/mdns-browser/pull/2393))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to d73d5ba ([#2394](https://github.com/hrzlgnm/mdns-browser/pull/2394))

- *(deps)* Update dependency cargo-nextest to v0.9.143 ([#2397](https://github.com/hrzlgnm/mdns-browser/pull/2397))

- *(deps)* Update actions/attest digest to 1e69f48 ([#2396](https://github.com/hrzlgnm/mdns-browser/pull/2396))

- *(deps)* Update dtolnay/rust-toolchain digest to 4360b52 ([#2399](https://github.com/hrzlgnm/mdns-browser/pull/2399))

- *(deps)* Update dorny/paths-filter action to v4.0.3 ([#2402](https://github.com/hrzlgnm/mdns-browser/pull/2402))

### Fixed

- Use github-actions[bot] identity for homebrew-tap commits ([#2385](https://github.com/hrzlgnm/mdns-browser/pull/2385))

## [1.13.4] - 2026-08-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.13.3...mdns-browser-v1.13.4)

### Fixed

- *(release)* Replace release-downloader with gh release download in asset checksums ([#2384](https://github.com/hrzlgnm/mdns-browser/pull/2384))

## [1.13.3] - 2026-08-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.13.2...mdns-browser-v1.13.3)

### Changed

- Configure git safe directory ([#2383](https://github.com/hrzlgnm/mdns-browser/pull/2383))

## [1.13.2] - 2026-08-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.13.1...mdns-browser-v1.13.2)

### Added

- Add gh to ubuntu-package-builder image ([#2381](https://github.com/hrzlgnm/mdns-browser/pull/2381))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to a2788ba ([#2382](https://github.com/hrzlgnm/mdns-browser/pull/2382))

### Fixed

- *(release)* Use gh release download for source checksum in void workflow ([#2380](https://github.com/hrzlgnm/mdns-browser/pull/2380))

## [1.13.1] - 2026-08-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.13.0...mdns-browser-v1.13.1)

### Fixed

- *(release)* Replace draft input with publish in release-drafter config ([#2374](https://github.com/hrzlgnm/mdns-browser/pull/2374))

- *(release)* Replace draft input with publish and rename job ([#2377](https://github.com/hrzlgnm/mdns-browser/pull/2377))

- *(release)* Use gh release download for source tarball checksum in void workflow ([#2376](https://github.com/hrzlgnm/mdns-browser/pull/2376))

- *(release)* Set releaseDraft: true and add releaseName in desktop workflow ([#2375](https://github.com/hrzlgnm/mdns-browser/pull/2375))

## [1.13.0] - 2026-08-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.12.0...mdns-browser-v1.13.0)

### Added

- *(release)* Use immutable releases with draft workflow ([#2354](https://github.com/hrzlgnm/mdns-browser/pull/2354)) ([#2373](https://github.com/hrzlgnm/mdns-browser/pull/2373))

### Changed

- Update agent instructions ([#2362](https://github.com/hrzlgnm/mdns-browser/pull/2362))

- Tweak tauri config ([#2361](https://github.com/hrzlgnm/mdns-browser/pull/2361))

- Use github-actions bot for automated commits ([#2365](https://github.com/hrzlgnm/mdns-browser/pull/2365))

- Remove redundant android tauri config ([#2364](https://github.com/hrzlgnm/mdns-browser/pull/2364))

- Create changelog PR as verified github-actions bot ([#2368](https://github.com/hrzlgnm/mdns-browser/pull/2368))

- Include dependencies in changelog ([#2370](https://github.com/hrzlgnm/mdns-browser/pull/2370))

- Make the typos digest ignore pattern more broader ([#2372](https://github.com/hrzlgnm/mdns-browser/pull/2372))

## [1.12.0] - 2026-08-01 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.10.0...mdns-browser-v1.12.0)

### Added

- *(mobile)* Implement Android auto-update via tauri plugin ([#2355](https://github.com/hrzlgnm/mdns-browser/pull/2355))

### Changed

- Simplify mobile update to open release page in browser ([#2359](https://github.com/hrzlgnm/mdns-browser/pull/2359))

### Fixed

- *(aur)* Copy license from the new deb location ([#2356](https://github.com/hrzlgnm/mdns-browser/pull/2356))

### Maintenance

- *(ci)* Update NDK version to 29 ([#2358](https://github.com/hrzlgnm/mdns-browser/pull/2358))

## [1.10.0] - 2026-08-01 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.21...mdns-browser-v1.10.0)

### Added

- *(webkit2gtk-nvidia-quirk)* Skip wayland workaround when egl-wayland2 is active ([#2350](https://github.com/hrzlgnm/mdns-browser/pull/2350))

### Changed

- *(aur)* Clarify why we install a separate unbundled binary ([#2345](https://github.com/hrzlgnm/mdns-browser/pull/2345))

- *(rpm)* Add installation of the license to the rpm bundle ([#2343](https://github.com/hrzlgnm/mdns-browser/pull/2343))

- *(deb)* Install the copyright to the correct location ([#2344](https://github.com/hrzlgnm/mdns-browser/pull/2344))

- Create pull requests proactively after pushing commits ([#2351](https://github.com/hrzlgnm/mdns-browser/pull/2351))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 40d14ac ([#2327](https://github.com/hrzlgnm/mdns-browser/pull/2327))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 0f536db ([#2328](https://github.com/hrzlgnm/mdns-browser/pull/2328))

- *(deps)* Update hrzlgnm/actions action to v2.5.4 ([#2329](https://github.com/hrzlgnm/mdns-browser/pull/2329))

- *(deps)* Update release-drafter/release-drafter action to v7.7.0 ([#2332](https://github.com/hrzlgnm/mdns-browser/pull/2332))

- *(deps)* Update actions/setup-java digest to b6effb0 ([#2337](https://github.com/hrzlgnm/mdns-browser/pull/2337))

- *(deps)* Update actions/attest digest to 508db95 ([#2334](https://github.com/hrzlgnm/mdns-browser/pull/2334))

- *(deps)* Update mozilla-actions/sccache-action action to v0.0.11 ([#2331](https://github.com/hrzlgnm/mdns-browser/pull/2331))

- *(deps)* Update rust crate clap to v4.6.5 ([#2335](https://github.com/hrzlgnm/mdns-browser/pull/2335))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 59d65c3 ([#2346](https://github.com/hrzlgnm/mdns-browser/pull/2346))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 7463eb4 ([#2352](https://github.com/hrzlgnm/mdns-browser/pull/2352))

- *(deps)* Lock file maintenance ([#2353](https://github.com/hrzlgnm/mdns-browser/pull/2353))

### Maintenance

- *(ci)* Disable sccache for windows ([#2336](https://github.com/hrzlgnm/mdns-browser/pull/2336))

- *(ci)* Run changelog update on a nightly schedule ([#2349](https://github.com/hrzlgnm/mdns-browser/pull/2349))

## [1.9.21] - 2026-07-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.20...mdns-browser-v1.9.21)

### Changed

- Move changelog comparison links inline with version sections ([#2320](https://github.com/hrzlgnm/mdns-browser/pull/2320))

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v2.4.0 ([#2322](https://github.com/hrzlgnm/mdns-browser/pull/2322))

- *(deps)* Update rust crate mdns-sd to v0.20.3 ([#2324](https://github.com/hrzlgnm/mdns-browser/pull/2324))

- *(deps)* Lock file maintenance ([#2326](https://github.com/hrzlgnm/mdns-browser/pull/2326))

## [1.9.20] - 2026-07-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.19...mdns-browser-v1.9.20)

### Added

- Replace generate_changelog.py with git-cliff ([#2311](https://github.com/hrzlgnm/mdns-browser/pull/2311))

- Add workflow to keep Unreleased changelog sections up to date ([#2312](https://github.com/hrzlgnm/mdns-browser/pull/2312))

### Changed

- Add CHANGELOG.md with automated maintenance ([#2308](https://github.com/hrzlgnm/mdns-browser/pull/2308))

- Add GH_TOKEN to changelog generation steps in workflow ([#2310](https://github.com/hrzlgnm/mdns-browser/pull/2310))

- Include changelog in packaging configurations ([#2314](https://github.com/hrzlgnm/mdns-browser/pull/2314))

- Ignore update changelog PRs in release-drafter ([#2316](https://github.com/hrzlgnm/mdns-browser/pull/2316))

### Dependencies

- *(deps)* Update rust crate futures to v0.3.33 ([#2296](https://github.com/hrzlgnm/mdns-browser/pull/2296))

- *(deps)* Update rust crate thiserror to v2.0.19 ([#2298](https://github.com/hrzlgnm/mdns-browser/pull/2298))

- *(deps)* Update rust crate serde to v1.0.229 ([#2297](https://github.com/hrzlgnm/mdns-browser/pull/2297))

- *(deps)* Update release-drafter/release-drafter action to v7.6.0 ([#2299](https://github.com/hrzlgnm/mdns-browser/pull/2299))

- *(deps)* Update rust crate serde_json to v1.0.151 ([#2300](https://github.com/hrzlgnm/mdns-browser/pull/2300))

- *(deps)* Update archlinux:base-devel docker digest to 412efeb ([#2302](https://github.com/hrzlgnm/mdns-browser/pull/2302))

- *(deps)* Update actions/checkout digest to 3d3c42e ([#2301](https://github.com/hrzlgnm/mdns-browser/pull/2301))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to e5ae39e ([#2303](https://github.com/hrzlgnm/mdns-browser/pull/2303))

- *(deps)* Update rust crate clap to v4.6.3 ([#2304](https://github.com/hrzlgnm/mdns-browser/pull/2304))

- *(deps)* Update rust crate tokio to v1.53.1 ([#2305](https://github.com/hrzlgnm/mdns-browser/pull/2305))

- *(deps)* Update actions/labeler action to v7 ([#2306](https://github.com/hrzlgnm/mdns-browser/pull/2306))

- *(deps)* Update hrzlgnm/actions action to v2.3.1 ([#2307](https://github.com/hrzlgnm/mdns-browser/pull/2307))

- *(deps)* Update actions/setup-python action to v7 ([#2309](https://github.com/hrzlgnm/mdns-browser/pull/2309))

## [1.9.19] - 2026-07-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.18...mdns-browser-v1.9.19)

### Dependencies

- *(deps)* Lock file maintenance ([#2295](https://github.com/hrzlgnm/mdns-browser/pull/2295))

## [1.9.18] - 2026-07-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.17...mdns-browser-v1.9.18)

### Dependencies

- *(deps)* Update dependency cargo-edit to v0.13.12 ([#2278](https://github.com/hrzlgnm/mdns-browser/pull/2278))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 66b197b ([#2279](https://github.com/hrzlgnm/mdns-browser/pull/2279))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to de9a928 ([#2280](https://github.com/hrzlgnm/mdns-browser/pull/2280))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to ee2735a ([#2281](https://github.com/hrzlgnm/mdns-browser/pull/2281))

- *(deps)* Update rust crate console_log to v1.1.0 ([#2282](https://github.com/hrzlgnm/mdns-browser/pull/2282))

- *(deps)* Update dependency cargo-edit to v0.13.13 ([#2283](https://github.com/hrzlgnm/mdns-browser/pull/2283))

- *(deps)* Update rust crate clap to v4.6.2 ([#2284](https://github.com/hrzlgnm/mdns-browser/pull/2284))

- *(deps)* Update ubuntu:latest docker digest to 651ba3f ([#2285](https://github.com/hrzlgnm/mdns-browser/pull/2285))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to ee6b730 ([#2286](https://github.com/hrzlgnm/mdns-browser/pull/2286))

- *(deps)* Update rust crate tokio to v1.52.4 ([#2287](https://github.com/hrzlgnm/mdns-browser/pull/2287))

- *(deps)* Update actions/setup-java digest to 03ad4de ([#2289](https://github.com/hrzlgnm/mdns-browser/pull/2289))

- *(deps)* Update actions/attest digest to f7c74d2 ([#2288](https://github.com/hrzlgnm/mdns-browser/pull/2288))

- *(deps)* Update ubuntu:latest docker digest to 3131b4c ([#2291](https://github.com/hrzlgnm/mdns-browser/pull/2291))

- *(deps)* Update dtolnay/rust-toolchain digest to 4cda84d ([#2290](https://github.com/hrzlgnm/mdns-browser/pull/2290))

- *(deps)* Update rust crate mdns-sd to v0.20.2 ([#2292](https://github.com/hrzlgnm/mdns-browser/pull/2292))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to d4a3e88 ([#2293](https://github.com/hrzlgnm/mdns-browser/pull/2293))

- *(deps)* Update rust crate tokio to v1.53.0 ([#2294](https://github.com/hrzlgnm/mdns-browser/pull/2294))

## [1.9.16] - 2026-07-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.15...mdns-browser-v1.9.16)

### Added

- Add daily retry workflow for failed CI on PRs ([#2264](https://github.com/hrzlgnm/mdns-browser/pull/2264))

- Use retry-failed-ci reusable workflow ([#2271](https://github.com/hrzlgnm/mdns-browser/pull/2271))

### Changed

- Add shell: bash instruction to workflow guidelines in AGENTS.md ([#2257](https://github.com/hrzlgnm/mdns-browser/pull/2257))

- Apply clippy fixes ([#2268](https://github.com/hrzlgnm/mdns-browser/pull/2268))

- Update workflow name to reflect the actual purpose ([#2277](https://github.com/hrzlgnm/mdns-browser/pull/2277))

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.140 ([#2258](https://github.com/hrzlgnm/mdns-browser/pull/2258))

- *(deps)* Lock file maintenance ([#2259](https://github.com/hrzlgnm/mdns-browser/pull/2259))

- *(deps)* Update archlinux:base-devel docker digest to b21289e ([#2260](https://github.com/hrzlgnm/mdns-browser/pull/2260))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 6258dc8 ([#2261](https://github.com/hrzlgnm/mdns-browser/pull/2261))

- *(deps)* Update actions/setup-java digest to 0f481fc ([#2262](https://github.com/hrzlgnm/mdns-browser/pull/2262))

- *(deps)* Update actions/labeler action to v6.2.0 ([#2263](https://github.com/hrzlgnm/mdns-browser/pull/2263))

- *(deps)* Lock file maintenance ([#2266](https://github.com/hrzlgnm/mdns-browser/pull/2266))

- *(deps)* Update archlinux:base-devel docker digest to 212b1e5 ([#2270](https://github.com/hrzlgnm/mdns-browser/pull/2270))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 00160eb ([#2272](https://github.com/hrzlgnm/mdns-browser/pull/2272))

- *(deps)* Update softprops/action-gh-release digest to 3d0d988 ([#2269](https://github.com/hrzlgnm/mdns-browser/pull/2269))

- *(deps)* Update hrzlgnm/actions action to v2.2.0 ([#2273](https://github.com/hrzlgnm/mdns-browser/pull/2273))

- *(deps)* Update rust crate tauri-plugin-log to v2.9.0 ([#2274](https://github.com/hrzlgnm/mdns-browser/pull/2274))

- *(deps)* Update hrzlgnm/actions action to v2.3.0 ([#2276](https://github.com/hrzlgnm/mdns-browser/pull/2276))

### Fixed

- Pass branch/number to jq via environment variables ([#2265](https://github.com/hrzlgnm/mdns-browser/pull/2265))

- Only rerun failed jobs ([#2267](https://github.com/hrzlgnm/mdns-browser/pull/2267))

- Migrate release-drafter to new category syntax ([#2275](https://github.com/hrzlgnm/mdns-browser/pull/2275))

## [1.9.15] - 2026-07-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.14...mdns-browser-v1.9.15)

### Fixed

- Add shell: bash to get current release body step for Windows compatibility ([#2256](https://github.com/hrzlgnm/mdns-browser/pull/2256))

## [1.9.14] - 2026-07-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.13...mdns-browser-v1.9.14)

### Fixed

- Use curl instead of gh for fetching release body

## [1.9.13] - 2026-07-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.12...mdns-browser-v1.9.13)

### Fixed

- Preserve release notes from release-drafter when tauri-action uploads artifacts ([#2253](https://github.com/hrzlgnm/mdns-browser/pull/2253))

## [1.9.12] - 2026-07-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.11...mdns-browser-v1.9.12)

### Dependencies

- *(deps)* Update rust crate mdns-sd to v0.20.1 ([#2235](https://github.com/hrzlgnm/mdns-browser/pull/2235))

- *(deps)* Lock file maintenance ([#2236](https://github.com/hrzlgnm/mdns-browser/pull/2236))

- *(deps)* Update dependency tauri-cli to v2.11.4 ([#2237](https://github.com/hrzlgnm/mdns-browser/pull/2237))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 4913e93 ([#2238](https://github.com/hrzlgnm/mdns-browser/pull/2238))

- *(deps)* Update dependency cargo-auditable to v0.7.5 ([#2239](https://github.com/hrzlgnm/mdns-browser/pull/2239))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 9899634 ([#2240](https://github.com/hrzlgnm/mdns-browser/pull/2240))

- *(deps)* Lock file maintenance ([#2241](https://github.com/hrzlgnm/mdns-browser/pull/2241))

- *(deps)* Update tauri-apps/tauri-action action to v1 ([#2242](https://github.com/hrzlgnm/mdns-browser/pull/2242))

- *(deps)* Update archlinux:base-devel docker digest to 5fb487a ([#2243](https://github.com/hrzlgnm/mdns-browser/pull/2243))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 6b22aa4 ([#2244](https://github.com/hrzlgnm/mdns-browser/pull/2244))

- *(deps)* Update dtolnay/rust-toolchain digest to 4be7066 ([#2245](https://github.com/hrzlgnm/mdns-browser/pull/2245))

- *(deps)* Update rust crate tauri to v2.11.4 ([#2246](https://github.com/hrzlgnm/mdns-browser/pull/2246))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 4fe0986 ([#2247](https://github.com/hrzlgnm/mdns-browser/pull/2247))

- *(deps)* Update rust crate tauri to v2.11.5 ([#2248](https://github.com/hrzlgnm/mdns-browser/pull/2248))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 21ce90d ([#2249](https://github.com/hrzlgnm/mdns-browser/pull/2249))

- *(deps)* Update ubuntu:latest docker digest to b7f4819 ([#2250](https://github.com/hrzlgnm/mdns-browser/pull/2250))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 75d0b43 ([#2251](https://github.com/hrzlgnm/mdns-browser/pull/2251))

- *(deps)* Update dorny/paths-filter action to v4.0.2 ([#2252](https://github.com/hrzlgnm/mdns-browser/pull/2252))

## [1.9.11] - 2026-06-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.10...mdns-browser-v1.9.11)

### Dependencies

- *(deps)* Update actions/checkout action to v7 ([#2217](https://github.com/hrzlgnm/mdns-browser/pull/2217))

- *(deps)* Update ubuntu:latest docker digest to e153663 ([#2218](https://github.com/hrzlgnm/mdns-browser/pull/2218))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to d4b31d2 ([#2219](https://github.com/hrzlgnm/mdns-browser/pull/2219))

- *(deps)* Update softprops/action-gh-release digest to 718ea10 ([#2220](https://github.com/hrzlgnm/mdns-browser/pull/2220))

- *(deps)* Update ubuntu:latest docker digest to 53958ec ([#2221](https://github.com/hrzlgnm/mdns-browser/pull/2221))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 5365f02 ([#2222](https://github.com/hrzlgnm/mdns-browser/pull/2222))

- *(deps)* Update mikepenz/action-junit-report digest to d9f48fc ([#2223](https://github.com/hrzlgnm/mdns-browser/pull/2223))

- *(deps)* Update rust crate log to v0.4.33 ([#2224](https://github.com/hrzlgnm/mdns-browser/pull/2224))

- *(deps)* Update dependency cargo-nextest to v0.9.138 ([#2225](https://github.com/hrzlgnm/mdns-browser/pull/2225))

- *(deps)* Lock file maintenance ([#2226](https://github.com/hrzlgnm/mdns-browser/pull/2226))

- *(deps)* Update archlinux:base-devel docker digest to cf028ae ([#2227](https://github.com/hrzlgnm/mdns-browser/pull/2227))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to cce7644 ([#2228](https://github.com/hrzlgnm/mdns-browser/pull/2228))

- *(deps)* Update release-drafter/release-drafter action to v7.5.0 ([#2229](https://github.com/hrzlgnm/mdns-browser/pull/2229))

- *(deps)* Update rust crate leptos to v0.8.20 ([#2230](https://github.com/hrzlgnm/mdns-browser/pull/2230))

- *(deps)* Update release-drafter/release-drafter action to v7.5.1 ([#2232](https://github.com/hrzlgnm/mdns-browser/pull/2232))

- *(deps)* Update actions/setup-java digest to 1bcf9fb ([#2231](https://github.com/hrzlgnm/mdns-browser/pull/2231))

- *(deps)* Update actions/attest digest to a1948c3 ([#2233](https://github.com/hrzlgnm/mdns-browser/pull/2233))

- *(deps)* Lock file maintenance ([#2234](https://github.com/hrzlgnm/mdns-browser/pull/2234))

## [1.9.10] - 2026-06-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.9...mdns-browser-v1.9.10)

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v2.1.6 ([#2201](https://github.com/hrzlgnm/mdns-browser/pull/2201))

- *(deps)* Lock file maintenance ([#2202](https://github.com/hrzlgnm/mdns-browser/pull/2202))

- *(deps)* Update archlinux:base-devel docker digest to dd60dfc ([#2203](https://github.com/hrzlgnm/mdns-browser/pull/2203))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d73dfab ([#2204](https://github.com/hrzlgnm/mdns-browser/pull/2204))

- *(deps)* Lock file maintenance ([#2205](https://github.com/hrzlgnm/mdns-browser/pull/2205))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 40985c0 ([#2206](https://github.com/hrzlgnm/mdns-browser/pull/2206))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to e8cdbc0 ([#2207](https://github.com/hrzlgnm/mdns-browser/pull/2207))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 2d973b5 ([#2208](https://github.com/hrzlgnm/mdns-browser/pull/2208))

- *(deps)* Update archlinux:base-devel docker digest to 0cf5eb7 ([#2209](https://github.com/hrzlgnm/mdns-browser/pull/2209))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 5011ed4 ([#2210](https://github.com/hrzlgnm/mdns-browser/pull/2210))

- *(deps)* Update release-drafter/release-drafter action to v7.4.0 ([#2211](https://github.com/hrzlgnm/mdns-browser/pull/2211))

- *(deps)* Update actions/setup-java digest to ad2b381 ([#2212](https://github.com/hrzlgnm/mdns-browser/pull/2212))

- *(deps)* Update rust crate tauri to v2.11.3 ([#2214](https://github.com/hrzlgnm/mdns-browser/pull/2214))

- *(deps)* Update dependency tauri-cli to v2.11.3 ([#2213](https://github.com/hrzlgnm/mdns-browser/pull/2213))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 18bd5b1 ([#2215](https://github.com/hrzlgnm/mdns-browser/pull/2215))

- *(deps)* Lock file maintenance ([#2216](https://github.com/hrzlgnm/mdns-browser/pull/2216))

## [1.9.9] - 2026-06-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.8...mdns-browser-v1.9.9)

### Changed

- Update AUR builder Docker image to use libsoup3 ([#2197](https://github.com/hrzlgnm/mdns-browser/pull/2197))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 6db772f ([#2178](https://github.com/hrzlgnm/mdns-browser/pull/2178))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to b50db3b ([#2179](https://github.com/hrzlgnm/mdns-browser/pull/2179))

- *(deps)* Update rust crate serde_json to v1.0.150 ([#2180](https://github.com/hrzlgnm/mdns-browser/pull/2180))

- *(deps)* Update hrzlgnm/actions action to v2.1.4 ([#2181](https://github.com/hrzlgnm/mdns-browser/pull/2181))

- *(deps)* Update rust crate log to v0.4.30 ([#2182](https://github.com/hrzlgnm/mdns-browser/pull/2182))

- *(deps)* Update rust crate mdns-sd to 0.20 ([#2183](https://github.com/hrzlgnm/mdns-browser/pull/2183))

- *(deps)* Update release-drafter/release-drafter action to v7.3.1 ([#2184](https://github.com/hrzlgnm/mdns-browser/pull/2184))

- *(deps)* Update archlinux:base-devel docker digest to 2507482 ([#2185](https://github.com/hrzlgnm/mdns-browser/pull/2185))

- *(deps)* Update dependency cargo-nextest to v0.9.137 ([#2186](https://github.com/hrzlgnm/mdns-browser/pull/2186))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to c326515 ([#2187](https://github.com/hrzlgnm/mdns-browser/pull/2187))

- *(deps)* Update dependency cargo-edit to v0.13.11 ([#2188](https://github.com/hrzlgnm/mdns-browser/pull/2188))

- *(deps)* Update hrzlgnm/actions action to v2.1.5 ([#2189](https://github.com/hrzlgnm/mdns-browser/pull/2189))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 1a429bd ([#2190](https://github.com/hrzlgnm/mdns-browser/pull/2190))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to fa3fba2 ([#2191](https://github.com/hrzlgnm/mdns-browser/pull/2191))

- *(deps)* Update rust crate log to v0.4.31 ([#2193](https://github.com/hrzlgnm/mdns-browser/pull/2193))

- *(deps)* Update actions/checkout digest to df4cb1c ([#2194](https://github.com/hrzlgnm/mdns-browser/pull/2194))

- *(deps)* Update rust crate log to v0.4.32 ([#2195](https://github.com/hrzlgnm/mdns-browser/pull/2195))

- *(deps)* Update rust crate chrono to v0.4.45 ([#2196](https://github.com/hrzlgnm/mdns-browser/pull/2196))

- *(deps)* Update archlinux:base-devel docker digest to c84ad63 ([#2192](https://github.com/hrzlgnm/mdns-browser/pull/2192))

- *(deps)* Lock file maintenance ([#2198](https://github.com/hrzlgnm/mdns-browser/pull/2198))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4dea06f ([#2199](https://github.com/hrzlgnm/mdns-browser/pull/2199))

- *(deps)* Update rust crate serde_with to v3.21.0 ([#2200](https://github.com/hrzlgnm/mdns-browser/pull/2200))

## [1.9.8] - 2026-05-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.7...mdns-browser-v1.9.8)

### Changed

- Update msrv to 1.90 ([#2163](https://github.com/hrzlgnm/mdns-browser/pull/2163))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 88792fc ([#2160](https://github.com/hrzlgnm/mdns-browser/pull/2160))

- *(deps)* Update release-drafter/release-drafter action to v7.3.0 ([#2161](https://github.com/hrzlgnm/mdns-browser/pull/2161))

- *(deps)* Update rust crate tokio to v1.52.3 ([#2162](https://github.com/hrzlgnm/mdns-browser/pull/2162))

- *(deps)* Update rust crate serde_with to v3.20.0 ([#2164](https://github.com/hrzlgnm/mdns-browser/pull/2164))

- *(deps)* Lock file maintenance ([#2165](https://github.com/hrzlgnm/mdns-browser/pull/2165))

- *(deps)* Update archlinux:base-devel docker digest to 6ec1f50 ([#2166](https://github.com/hrzlgnm/mdns-browser/pull/2166))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4becbbb ([#2167](https://github.com/hrzlgnm/mdns-browser/pull/2167))

- *(deps)* Update dependency cargo-nextest to v0.9.135 ([#2168](https://github.com/hrzlgnm/mdns-browser/pull/2168))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4e372d5 ([#2169](https://github.com/hrzlgnm/mdns-browser/pull/2169))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to b367006 ([#2170](https://github.com/hrzlgnm/mdns-browser/pull/2170))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f47cd7e ([#2171](https://github.com/hrzlgnm/mdns-browser/pull/2171))

- *(deps)* Update tauri monorepo ([#2172](https://github.com/hrzlgnm/mdns-browser/pull/2172))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to a8ca1b3 ([#2173](https://github.com/hrzlgnm/mdns-browser/pull/2173))

- *(deps)* Update dependency cargo-nextest to v0.9.136 ([#2174](https://github.com/hrzlgnm/mdns-browser/pull/2174))

- *(deps)* Update mikepenz/action-junit-report digest to 3a81627 ([#2175](https://github.com/hrzlgnm/mdns-browser/pull/2175))

- *(deps)* Update rust crate mdns-sd to v0.19.2 ([#2176](https://github.com/hrzlgnm/mdns-browser/pull/2176))

- *(deps)* Lock file maintenance ([#2177](https://github.com/hrzlgnm/mdns-browser/pull/2177))

## [1.9.7] - 2026-05-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.6...mdns-browser-v1.9.7)

### Dependencies

- *(deps)* Lock file maintenance ([#2158](https://github.com/hrzlgnm/mdns-browser/pull/2158))

- *(deps)* Update archlinux:base-devel docker digest to fdff15f ([#2159](https://github.com/hrzlgnm/mdns-browser/pull/2159))

### Fixed

- Specify repo for gh release download in homebrew workflow ([#2157](https://github.com/hrzlgnm/mdns-browser/pull/2157))

## [1.9.6] - 2026-05-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.5...mdns-browser-v1.9.6)

### Added

- Add Homebrew tap support ([#2154](https://github.com/hrzlgnm/mdns-browser/pull/2154))

### Dependencies

- *(deps)* Update ubuntu:latest docker digest to f3d2860 ([#2149](https://github.com/hrzlgnm/mdns-browser/pull/2149))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 6dba25f ([#2150](https://github.com/hrzlgnm/mdns-browser/pull/2150))

- *(deps)* Update actions/labeler action to v6.1.0 ([#2151](https://github.com/hrzlgnm/mdns-browser/pull/2151))

- *(deps)* Update tauri monorepo ([#2152](https://github.com/hrzlgnm/mdns-browser/pull/2152))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 1f662ea ([#2153](https://github.com/hrzlgnm/mdns-browser/pull/2153))

### Fixed

- Address review comments missed from #2154 ([#2154](https://github.com/hrzlgnm/mdns-browser/pull/2154)) ([#2155](https://github.com/hrzlgnm/mdns-browser/pull/2155))

## [1.9.5] - 2026-05-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.4...mdns-browser-v1.9.5)

### Dependencies

- *(deps)* Lock file maintenance ([#2146](https://github.com/hrzlgnm/mdns-browser/pull/2146))

- *(deps)* Update rust crate tokio to v1.52.2 ([#2147](https://github.com/hrzlgnm/mdns-browser/pull/2147))

- *(deps)* Lock file maintenance ([#2148](https://github.com/hrzlgnm/mdns-browser/pull/2148))

## [1.9.4] - 2026-05-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.3...mdns-browser-v1.9.4)

### Dependencies

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 7df0a47 ([#2142](https://github.com/hrzlgnm/mdns-browser/pull/2142))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f89af55 ([#2143](https://github.com/hrzlgnm/mdns-browser/pull/2143))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.4 ([#2144](https://github.com/hrzlgnm/mdns-browser/pull/2144))

- *(deps)* Update rust crate serde_with to v3.19.0 ([#2145](https://github.com/hrzlgnm/mdns-browser/pull/2145))

## [1.9.3] - 2026-04-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.2...mdns-browser-v1.9.3)

### Dependencies

- *(deps)* Lock file maintenance ([#2135](https://github.com/hrzlgnm/mdns-browser/pull/2135))

- *(deps)* Update hrzlgnm/actions action to v2.1.2 ([#2136](https://github.com/hrzlgnm/mdns-browser/pull/2136))

- *(deps)* Update release-drafter/release-drafter action to v7.2.1 ([#2137](https://github.com/hrzlgnm/mdns-browser/pull/2137))

- *(deps)* Update hrzlgnm/actions action to v2.1.3 ([#2138](https://github.com/hrzlgnm/mdns-browser/pull/2138))

- *(deps)* Update tauri monorepo ([#2139](https://github.com/hrzlgnm/mdns-browser/pull/2139))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 24a5bf9 ([#2140](https://github.com/hrzlgnm/mdns-browser/pull/2140))

- *(deps)* Lock file maintenance ([#2141](https://github.com/hrzlgnm/mdns-browser/pull/2141))

## [1.9.2] - 2026-04-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.1...mdns-browser-v1.9.2)

### Dependencies

- *(deps)* Update robinraju/release-downloader action to v1.13 ([#2130](https://github.com/hrzlgnm/mdns-browser/pull/2130))

- *(deps)* Update archlinux:base-devel docker digest to f15064b ([#2131](https://github.com/hrzlgnm/mdns-browser/pull/2131))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to aa2b8b5 ([#2132](https://github.com/hrzlgnm/mdns-browser/pull/2132))

- *(deps)* Update mozilla-actions/sccache-action action to v0.0.10 ([#2133](https://github.com/hrzlgnm/mdns-browser/pull/2133))

- *(deps)* Lock file maintenance ([#2134](https://github.com/hrzlgnm/mdns-browser/pull/2134))

## [1.9.1] - 2026-04-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.0...mdns-browser-v1.9.1)

### Dependencies

- *(deps)* Update dependency cargo-edit to v0.13.10 ([#2127](https://github.com/hrzlgnm/mdns-browser/pull/2127))

- *(deps)* Lock file maintenance ([#2128](https://github.com/hrzlgnm/mdns-browser/pull/2128))

- *(deps)* Update rust crate mdns-sd to v0.19.1 ([#2129](https://github.com/hrzlgnm/mdns-browser/pull/2129))

## [1.9.0] - 2026-04-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.4...mdns-browser-v1.9.0)

### Added

- Add --no-nvidia-workaround CLI option to disable all NVIDIA workarounds ([#2126](https://github.com/hrzlgnm/mdns-browser/pull/2126))

### Changed

- Simplify ResolvedServiceItem by deriving memos from single try_get ([#2122](https://github.com/hrzlgnm/mdns-browser/pull/2122))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to c528201 ([#2118](https://github.com/hrzlgnm/mdns-browser/pull/2118))

- *(deps)* Update ubuntu:latest docker digest to c4a8d55 ([#2119](https://github.com/hrzlgnm/mdns-browser/pull/2119))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 9380162 ([#2120](https://github.com/hrzlgnm/mdns-browser/pull/2120))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to ec6faf0 ([#2121](https://github.com/hrzlgnm/mdns-browser/pull/2121))

- *(deps)* Update rust crate leptos to v0.8.19 ([#2123](https://github.com/hrzlgnm/mdns-browser/pull/2123))

- *(deps)* Update rust crate tokio to v1.52.1 ([#2124](https://github.com/hrzlgnm/mdns-browser/pull/2124))

### Fixed

- Resolve variable shadowing and replace Memo with Signal::derive ([#2125](https://github.com/hrzlgnm/mdns-browser/pull/2125))

## [1.8.4] - 2026-04-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.3...mdns-browser-v1.8.4)

### Dependencies

- *(deps)* Update rust crate tokio to v1.52.0 ([#2108](https://github.com/hrzlgnm/mdns-browser/pull/2108))

- *(deps)* Update dependency cargo-nextest to v0.9.133 ([#2109](https://github.com/hrzlgnm/mdns-browser/pull/2109))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 75337d6 ([#2110](https://github.com/hrzlgnm/mdns-browser/pull/2110))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 7d70f2d ([#2111](https://github.com/hrzlgnm/mdns-browser/pull/2111))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to d2ccc41 ([#2112](https://github.com/hrzlgnm/mdns-browser/pull/2112))

- *(deps)* Update rust crate clap to v4.6.1 ([#2115](https://github.com/hrzlgnm/mdns-browser/pull/2115))

- *(deps)* Lock file maintenance ([#2116](https://github.com/hrzlgnm/mdns-browser/pull/2116))

### Fixed

- Handle disposal gracefully in ResolvedServiceItem component ([#2113](https://github.com/hrzlgnm/mdns-browser/pull/2113))

## [1.8.3] - 2026-04-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.2...mdns-browser-v1.8.3)

### Changed

- Switch to actionlint from hrzlgnm/actions ([#2097](https://github.com/hrzlgnm/mdns-browser/pull/2097))

- Add instructions how to validate renovate config ([#2099](https://github.com/hrzlgnm/mdns-browser/pull/2099))

### Dependencies

- *(deps)* Update rust crate tokio to v1.51.1 ([#2086](https://github.com/hrzlgnm/mdns-browser/pull/2086))

- *(deps)* Lock file maintenance ([#2087](https://github.com/hrzlgnm/mdns-browser/pull/2087))

- *(deps)* Update release-drafter/release-drafter action to v7.2.0 ([#2089](https://github.com/hrzlgnm/mdns-browser/pull/2089))

- *(deps)* Update actions/github-script action to v9 ([#2090](https://github.com/hrzlgnm/mdns-browser/pull/2090))

- *(deps)* Update hrzlgnm/actions action to v2.0.7 ([#2091](https://github.com/hrzlgnm/mdns-browser/pull/2091))

- *(deps)* Update actions/upload-artifact digest to 043fb46 ([#2092](https://github.com/hrzlgnm/mdns-browser/pull/2092))

- *(deps)* Update actions/upload-artifact action to v7.0.1 ([#2093](https://github.com/hrzlgnm/mdns-browser/pull/2093))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to be72b19 ([#2096](https://github.com/hrzlgnm/mdns-browser/pull/2096))

- *(deps)* Update hrzlgnm/actions action to v2.1.0 ([#2098](https://github.com/hrzlgnm/mdns-browser/pull/2098))

- *(deps)* Update softprops/action-gh-release digest to 3bb1273 ([#2100](https://github.com/hrzlgnm/mdns-browser/pull/2100))

- *(deps)* Update softprops/action-gh-release action to v3 ([#2101](https://github.com/hrzlgnm/mdns-browser/pull/2101))

- *(deps)* Lock file maintenance ([#2102](https://github.com/hrzlgnm/mdns-browser/pull/2102))

- *(deps)* Update hrzlgnm/actions action to v2.1.1 ([#2103](https://github.com/hrzlgnm/mdns-browser/pull/2103))

- *(deps)* Update archlinux:base-devel docker digest to 01bd0ee ([#2104](https://github.com/hrzlgnm/mdns-browser/pull/2104))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d9bd9c7 ([#2105](https://github.com/hrzlgnm/mdns-browser/pull/2105))

- *(deps)* Lock file maintenance ([#2106](https://github.com/hrzlgnm/mdns-browser/pull/2106))

### Fixed

- Ensure url memo tracks reactive field changes ([#2107](https://github.com/hrzlgnm/mdns-browser/pull/2107))

### Maintenance

- *(ci)* Blazingly fast actionlint ([#2094](https://github.com/hrzlgnm/mdns-browser/pull/2094))

## [1.8.2] - 2026-04-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.1...mdns-browser-v1.8.2)

### Changed

- Use default options quick start example ([#2082](https://github.com/hrzlgnm/mdns-browser/pull/2082))

- Use sysfs instead of udev for GPU detection ([#2083](https://github.com/hrzlgnm/mdns-browser/pull/2083))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 001c948 ([#2085](https://github.com/hrzlgnm/mdns-browser/pull/2085))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to dc56c02 ([#2084](https://github.com/hrzlgnm/mdns-browser/pull/2084))

## [1.8.1] - 2026-04-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.0...mdns-browser-v1.8.1)

### Added

- Webkit2gtk-nvidia-quirk: check primary gpu ([#2070](https://github.com/hrzlgnm/mdns-browser/pull/2070))

### Changed

- Add libudev-dev dependencies to docker containers ([#2073](https://github.com/hrzlgnm/mdns-browser/pull/2073))

- Add eudev libudev devel to void package builder ([#2075](https://github.com/hrzlgnm/mdns-browser/pull/2075))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 4a48384 ([#2071](https://github.com/hrzlgnm/mdns-browser/pull/2071))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to efcda21 ([#2072](https://github.com/hrzlgnm/mdns-browser/pull/2072))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 36341fe ([#2074](https://github.com/hrzlgnm/mdns-browser/pull/2074))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 61e37a9 ([#2076](https://github.com/hrzlgnm/mdns-browser/pull/2076))

- *(deps)* Update ubuntu:latest docker digest to 84e77de ([#2077](https://github.com/hrzlgnm/mdns-browser/pull/2077))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 5fa741c ([#2078](https://github.com/hrzlgnm/mdns-browser/pull/2078))

### Fixed

- Use ubuntu builder image to ensure build dependencies ([#2079](https://github.com/hrzlgnm/mdns-browser/pull/2079))

- Use ubuntu builder image the correct step ([#2080](https://github.com/hrzlgnm/mdns-browser/pull/2080))

## [1.8.0] - 2026-04-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.7.1...mdns-browser-v1.8.0)

### Added

- Create webkit2gtk-nvidia-quirk crate ([#2064](https://github.com/hrzlgnm/mdns-browser/pull/2064))

- Improve webkit2gtk nvidia workaround ([#2069](https://github.com/hrzlgnm/mdns-browser/pull/2069))

### Changed

- Add README to webkit2gtk-nvidia-quirk crate ([#2067](https://github.com/hrzlgnm/mdns-browser/pull/2067))

- Clarify the issues webkit2gtk has with nvidia drivers ([#2068](https://github.com/hrzlgnm/mdns-browser/pull/2068))

### Dependencies

- *(deps)* Lock file maintenance ([#2063](https://github.com/hrzlgnm/mdns-browser/pull/2063))

## [1.7.1] - 2026-04-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.7.0...mdns-browser-v1.7.1)

### Changed

- Cache platform tools ([#2059](https://github.com/hrzlgnm/mdns-browser/pull/2059))

### Dependencies

- *(deps)* Update mdns-sd digest to b6ddc18 ([#2058](https://github.com/hrzlgnm/mdns-browser/pull/2058))

- *(deps)* Update android-actions/setup-android digest to 40fd30f ([#2060](https://github.com/hrzlgnm/mdns-browser/pull/2060))

- *(deps)* Update mdns-sd digest to d5f9060 ([#2061](https://github.com/hrzlgnm/mdns-browser/pull/2061))

- *(deps)* Switch mdns-sd to crates.io version 0.19 ([#2062](https://github.com/hrzlgnm/mdns-browser/pull/2062))

## [1.7.0] - 2026-04-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.6.0...mdns-browser-v1.7.0)

### Added

- Display network interfaces for IP addresses ([#2054](https://github.com/hrzlgnm/mdns-browser/pull/2054))

### Changed

- Enable coderabbit auto-review ([#2051](https://github.com/hrzlgnm/mdns-browser/pull/2051))

- Update agent instructions ([#2052](https://github.com/hrzlgnm/mdns-browser/pull/2052))

- Update agents ([#2057](https://github.com/hrzlgnm/mdns-browser/pull/2057))

### Dependencies

- *(deps)* Update rust crate tauri-plugin-updater to v2.10.1 ([#2055](https://github.com/hrzlgnm/mdns-browser/pull/2055))

- *(deps)* Lock file maintenance ([#2056](https://github.com/hrzlgnm/mdns-browser/pull/2056))

## [1.6.0] - 2026-04-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.13...mdns-browser-v1.6.0)

### Added

- Improve splashscreen to not be timer based anymore ([#2049](https://github.com/hrzlgnm/mdns-browser/pull/2049))

- Set mdns ip check interval to 1s ([#2050](https://github.com/hrzlgnm/mdns-browser/pull/2050))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 0cce2e4 ([#2042](https://github.com/hrzlgnm/mdns-browser/pull/2042))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to b11278a ([#2043](https://github.com/hrzlgnm/mdns-browser/pull/2043))

- *(deps)* Update hrzlgnm/actions action to v2.0.5 ([#2045](https://github.com/hrzlgnm/mdns-browser/pull/2045))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 56c1ad4 ([#2044](https://github.com/hrzlgnm/mdns-browser/pull/2044))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 3c91822 ([#2046](https://github.com/hrzlgnm/mdns-browser/pull/2046))

- *(deps)* Update hrzlgnm/actions action to v2.0.6 ([#2047](https://github.com/hrzlgnm/mdns-browser/pull/2047))

- *(deps)* Update rust crate tokio to v1.51.0 ([#2048](https://github.com/hrzlgnm/mdns-browser/pull/2048))

## [1.5.13] - 2026-03-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.12...mdns-browser-v1.5.13)

### Dependencies

- *(deps)* Update actions/download-artifact digest to 3e5f45b ([#2002](https://github.com/hrzlgnm/mdns-browser/pull/2002))

- *(deps)* Update swatinem/rust-cache digest to c676846 ([#2003](https://github.com/hrzlgnm/mdns-browser/pull/2003))

- *(deps)* Update swatinem/rust-cache digest to e18b497 ([#2004](https://github.com/hrzlgnm/mdns-browser/pull/2004))

- *(deps)* Update rust crate clap to v4.6.0 ([#2005](https://github.com/hrzlgnm/mdns-browser/pull/2005))

- *(deps)* Update dorny/paths-filter action to v3.0.3 ([#2006](https://github.com/hrzlgnm/mdns-browser/pull/2006))

- *(deps)* Update dorny/paths-filter action to v4 ([#2007](https://github.com/hrzlgnm/mdns-browser/pull/2007))

- *(deps)* Update release-drafter/release-drafter action to v7 ([#2008](https://github.com/hrzlgnm/mdns-browser/pull/2008))

- *(deps)* Update rust crate serde_with to v3.18.0 ([#2009](https://github.com/hrzlgnm/mdns-browser/pull/2009))

- *(deps)* Update dorny/paths-filter action to v4.0.1 ([#2010](https://github.com/hrzlgnm/mdns-browser/pull/2010))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 405a8aa ([#2011](https://github.com/hrzlgnm/mdns-browser/pull/2011))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to b8ab6fb ([#2012](https://github.com/hrzlgnm/mdns-browser/pull/2012))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 3a2fdc6 ([#2013](https://github.com/hrzlgnm/mdns-browser/pull/2013))

- *(deps)* Update softprops/action-gh-release digest to b25b93d ([#2014](https://github.com/hrzlgnm/mdns-browser/pull/2014))

- *(deps)* Update tauri-apps/tauri-action action to v0.6.2 ([#2015](https://github.com/hrzlgnm/mdns-browser/pull/2015))

- *(deps)* Update softprops/action-gh-release digest to 153bb8e ([#2016](https://github.com/hrzlgnm/mdns-browser/pull/2016))

- *(deps)* Update archlinux:base-devel docker digest to 87c122f ([#2018](https://github.com/hrzlgnm/mdns-browser/pull/2018))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 0d86f89 ([#2019](https://github.com/hrzlgnm/mdns-browser/pull/2019))

- *(deps)* Update ubuntu:latest docker digest to 9d6e6f7 ([#2020](https://github.com/hrzlgnm/mdns-browser/pull/2020))

- *(deps)* Update ubuntu:latest docker digest to 0d39fcc ([#2022](https://github.com/hrzlgnm/mdns-browser/pull/2022))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to cb26790 ([#2021](https://github.com/hrzlgnm/mdns-browser/pull/2021))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to a51e2e1 ([#2023](https://github.com/hrzlgnm/mdns-browser/pull/2023))

- *(deps)* Update release-drafter/release-drafter action to v7.1.0 ([#2024](https://github.com/hrzlgnm/mdns-browser/pull/2024))

- *(deps)* Update release-drafter/release-drafter action to v7.1.1 ([#2026](https://github.com/hrzlgnm/mdns-browser/pull/2026))

- *(deps)* Update dependency cargo-nextest to v0.9.131 ([#2025](https://github.com/hrzlgnm/mdns-browser/pull/2025))

- *(deps)* Update ubuntu:latest docker digest to 186072b ([#2027](https://github.com/hrzlgnm/mdns-browser/pull/2027))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 3d1f37d ([#2028](https://github.com/hrzlgnm/mdns-browser/pull/2028))

- *(deps)* Update anchore/scan-action action to v7.4.0 ([#2030](https://github.com/hrzlgnm/mdns-browser/pull/2030))

- *(deps)* Update anchore/sbom-action digest to e22c389 ([#2029](https://github.com/hrzlgnm/mdns-browser/pull/2029))

- *(deps)* Update dependency cargo-nextest to v0.9.132 ([#2031](https://github.com/hrzlgnm/mdns-browser/pull/2031))

- *(deps)* Update archlinux:base-devel docker digest to 233f521 ([#2032](https://github.com/hrzlgnm/mdns-browser/pull/2032))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 0483b22 ([#2033](https://github.com/hrzlgnm/mdns-browser/pull/2033))

- *(deps)* Update rust crate ipconfig to v0.3.3 ([#2034](https://github.com/hrzlgnm/mdns-browser/pull/2034))

- *(deps)* Update rust crate ipconfig to v0.3.4 ([#2035](https://github.com/hrzlgnm/mdns-browser/pull/2035))

- *(deps)* Update android-actions/setup-android action to v4 ([#2036](https://github.com/hrzlgnm/mdns-browser/pull/2036))

- *(deps)* Pin dtolnay/rust-toolchain action to 631a55b ([#2037](https://github.com/hrzlgnm/mdns-browser/pull/2037))

- *(deps)* Update dtolnay/rust-toolchain digest to 29eef33 ([#2038](https://github.com/hrzlgnm/mdns-browser/pull/2038))

- *(deps)* Update mikepenz/action-junit-report digest to bccf2e3 ([#2039](https://github.com/hrzlgnm/mdns-browser/pull/2039))

- *(deps)* Update dependency komac to v2.16.0 ([#2040](https://github.com/hrzlgnm/mdns-browser/pull/2040))

### Fixed

- Resolve 3 security vulnerabilities in dependencies ([#2041](https://github.com/hrzlgnm/mdns-browser/pull/2041))

## [1.5.12] - 2026-03-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.11...mdns-browser-v1.5.12)

### Changed

- Temporary disable sscache to debug #1964 ([#1964](https://github.com/hrzlgnm/mdns-browser/pull/1964)) ([#1965](https://github.com/hrzlgnm/mdns-browser/pull/1965))

- Cached install komac using cargo-install ([#1973](https://github.com/hrzlgnm/mdns-browser/pull/1973))

- Pass winget token via env to komac ([#1976](https://github.com/hrzlgnm/mdns-browser/pull/1976))

- Improve cache-tools job ordering ([#1978](https://github.com/hrzlgnm/mdns-browser/pull/1978))

- Enable SCCACHE action again ([#1979](https://github.com/hrzlgnm/mdns-browser/pull/1979))

- Update agent instructions ([#1982](https://github.com/hrzlgnm/mdns-browser/pull/1982))

- Address a leftover review comment ([#1984](https://github.com/hrzlgnm/mdns-browser/pull/1984))

- Add actionlint and shellcheck to arch aur builder ([#1991](https://github.com/hrzlgnm/mdns-browser/pull/1991))

- Don't allow skipping actionlint job ([#1994](https://github.com/hrzlgnm/mdns-browser/pull/1994))

### Dependencies

- *(deps)* Update dependency cargo-auditable to v0.7.4 ([#1966](https://github.com/hrzlgnm/mdns-browser/pull/1966))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f83d313 ([#1967](https://github.com/hrzlgnm/mdns-browser/pull/1967))

- *(deps)* Update hrzlgnm/actions action to v2.0.4 ([#1977](https://github.com/hrzlgnm/mdns-browser/pull/1977))

- *(deps)* Update release-drafter/release-drafter action to v6.3.0 ([#1980](https://github.com/hrzlgnm/mdns-browser/pull/1980))

- *(deps)* Update release-drafter/release-drafter action to v6.4.0 ([#1981](https://github.com/hrzlgnm/mdns-browser/pull/1981))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 6bb7b3e ([#1992](https://github.com/hrzlgnm/mdns-browser/pull/1992))

- *(deps)* Pin ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder docker tag to 6bb7b3e ([#1993](https://github.com/hrzlgnm/mdns-browser/pull/1993))

- *(deps)* Update archlinux:base-devel docker digest to b3c1ff7 ([#1997](https://github.com/hrzlgnm/mdns-browser/pull/1997))

- *(deps)* Update anchore/sbom-action digest to 57aae52 ([#1996](https://github.com/hrzlgnm/mdns-browser/pull/1996))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 0d1a4ba ([#1998](https://github.com/hrzlgnm/mdns-browser/pull/1998))

- *(deps)* Update dependency cargo-nextest to v0.9.130 ([#1999](https://github.com/hrzlgnm/mdns-browser/pull/1999))

- *(deps)* Update rust crate mdns-sd to v0.18.2 ([#2000](https://github.com/hrzlgnm/mdns-browser/pull/2000))

- *(deps)* Lock file maintenance ([#2001](https://github.com/hrzlgnm/mdns-browser/pull/2001))

### Fixed

- Use RULESET_ID secret instead of hardcoded ruleset ID ([#1986](https://github.com/hrzlgnm/mdns-browser/pull/1986))

- Address issues reported by actionlint ([#1988](https://github.com/hrzlgnm/mdns-browser/pull/1988))

### Maintenance

- *(ci)* Extract cargo-edit installation into reusable workflow ([#1969](https://github.com/hrzlgnm/mdns-browser/pull/1969))

- Add permission comments and remove redundant comments from workflows ([#1985](https://github.com/hrzlgnm/mdns-browser/pull/1985))

- Add actionlint validation step to CI workflow ([#1990](https://github.com/hrzlgnm/mdns-browser/pull/1990))

## [1.5.11] - 2026-03-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.10...mdns-browser-v1.5.11)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to e110288 ([#1962](https://github.com/hrzlgnm/mdns-browser/pull/1962))

- *(deps)* Lock file maintenance ([#1963](https://github.com/hrzlgnm/mdns-browser/pull/1963))

## [1.5.10] - 2026-03-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.9...mdns-browser-v1.5.10)

### Changed

- Consolidate winget jobs to install komac only once ([#1957](https://github.com/hrzlgnm/mdns-browser/pull/1957))

### Dependencies

- *(deps)* Update dependency cargo-auditable to v0.7.3 ([#1958](https://github.com/hrzlgnm/mdns-browser/pull/1958))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to bfdb24f ([#1959](https://github.com/hrzlgnm/mdns-browser/pull/1959))

- *(deps)* Update hrzlgnm/actions action to v2.0.3 ([#1960](https://github.com/hrzlgnm/mdns-browser/pull/1960))

- *(deps)* Update tauri monorepo ([#1961](https://github.com/hrzlgnm/mdns-browser/pull/1961))

## [1.5.9] - 2026-03-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.8...mdns-browser-v1.5.9)

### Changed

- Increase collapse limit for dependency updates ([#1949](https://github.com/hrzlgnm/mdns-browser/pull/1949))

- Self sign macOS bundle ([#1955](https://github.com/hrzlgnm/mdns-browser/pull/1955))

### Dependencies

- *(deps)* Update mikepenz/action-junit-report digest to 49b2ca0 ([#1950](https://github.com/hrzlgnm/mdns-browser/pull/1950))

- *(deps)* Update dependency cargo-edit to v0.13.9 ([#1953](https://github.com/hrzlgnm/mdns-browser/pull/1953))

- *(deps)* Update archlinux:base-devel docker digest to a4e49d3 ([#1952](https://github.com/hrzlgnm/mdns-browser/pull/1952))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d06a3c6 ([#1954](https://github.com/hrzlgnm/mdns-browser/pull/1954))

- *(deps)* Update rust crate tokio to v1.50.0 ([#1956](https://github.com/hrzlgnm/mdns-browser/pull/1956))

## [1.5.8] - 2026-03-01 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.7...mdns-browser-v1.5.8)

### Dependencies

- *(deps)* Update rust crate mdns-sd to v0.18.1 ([#1942](https://github.com/hrzlgnm/mdns-browser/pull/1942))

- *(deps)* Lock file maintenance ([#1943](https://github.com/hrzlgnm/mdns-browser/pull/1943))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 1770b94 ([#1944](https://github.com/hrzlgnm/mdns-browser/pull/1944))

- *(deps)* Update rust crate leptos to v0.8.17 ([#1945](https://github.com/hrzlgnm/mdns-browser/pull/1945))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f1c91f3 ([#1946](https://github.com/hrzlgnm/mdns-browser/pull/1946))

- *(deps)* Lock file maintenance ([#1948](https://github.com/hrzlgnm/mdns-browser/pull/1948))

## [1.5.7] - 2026-02-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.6...mdns-browser-v1.5.7)

### Changed

- Move man page to docs ([#1924](https://github.com/hrzlgnm/mdns-browser/pull/1924))

- Remove unused file ([#1925](https://github.com/hrzlgnm/mdns-browser/pull/1925))

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.129 ([#1927](https://github.com/hrzlgnm/mdns-browser/pull/1927))

- *(deps)* Update baptiste0928/cargo-install digest to f204293 ([#1928](https://github.com/hrzlgnm/mdns-browser/pull/1928))

- *(deps)* Update rust crate chrono to v0.4.44 ([#1929](https://github.com/hrzlgnm/mdns-browser/pull/1929))

- *(deps)* Update archlinux:base-devel docker digest to f7227ef ([#1930](https://github.com/hrzlgnm/mdns-browser/pull/1930))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 738111f ([#1931](https://github.com/hrzlgnm/mdns-browser/pull/1931))

- *(deps)* Update mikepenz/action-junit-report digest to 5e05ac0 ([#1933](https://github.com/hrzlgnm/mdns-browser/pull/1933))

- *(deps)* Update anchore/sbom-action digest to 17ae174 ([#1932](https://github.com/hrzlgnm/mdns-browser/pull/1932))

- *(deps)* Update hrzlgnm/actions action to v2.0.2 ([#1934](https://github.com/hrzlgnm/mdns-browser/pull/1934))

- *(deps)* Update rust crate serde_with to v3.17.0 ([#1935](https://github.com/hrzlgnm/mdns-browser/pull/1935))

- *(deps)* Update actions/attest-sbom action to v4 ([#1937](https://github.com/hrzlgnm/mdns-browser/pull/1937))

- *(deps)* Update github artifact actions (major) ([#1938](https://github.com/hrzlgnm/mdns-browser/pull/1938))

- *(deps)* Update actions/attest-build-provenance action to v4 ([#1936](https://github.com/hrzlgnm/mdns-browser/pull/1936))

- *(deps)* Consolidate actions/attest-build-provenance and actions/attest-sbom into actions/attest ([#1940](https://github.com/hrzlgnm/mdns-browser/pull/1940))

- *(deps)* Pin actions/attest action to 59d8942 ([#1941](https://github.com/hrzlgnm/mdns-browser/pull/1941))

## [1.5.6] - 2026-02-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.5...mdns-browser-v1.5.6)

### Changed

- Remove debug logging statements ([#1923](https://github.com/hrzlgnm/mdns-browser/pull/1923))

- Rename debug symbol artifacts ([#1922](https://github.com/hrzlgnm/mdns-browser/pull/1922))

## [1.5.5] - 2026-02-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.4...mdns-browser-v1.5.5)

### Changed

- Remove debug logs for received events ([#1921](https://github.com/hrzlgnm/mdns-browser/pull/1921))

## [1.5.4] - 2026-02-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.3...mdns-browser-v1.5.4)

### Added

- Add debug symbols attestation and upload for all desktop platforms ([#1919](https://github.com/hrzlgnm/mdns-browser/pull/1919))

### Changed

- Optimize interface filtering ([#1918](https://github.com/hrzlgnm/mdns-browser/pull/1918))

## [1.5.3] - 2026-02-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.2...mdns-browser-v1.5.3)

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.128 ([#1916](https://github.com/hrzlgnm/mdns-browser/pull/1916))

### Fixed

- Aur deployment key setup ([#1917](https://github.com/hrzlgnm/mdns-browser/pull/1917))

## [1.5.2] - 2026-02-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.1...mdns-browser-v1.5.2)

### Added

- *(aur)* Use plain executable for mdns-browser-bin ([#1915](https://github.com/hrzlgnm/mdns-browser/pull/1915))

### Changed

- Update copyright years to 2026 ([#1906](https://github.com/hrzlgnm/mdns-browser/pull/1906))

- Make all dependencies workspace dependencies ([#1910](https://github.com/hrzlgnm/mdns-browser/pull/1910))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.60 ([#1912](https://github.com/hrzlgnm/mdns-browser/pull/1912))

### Fixed

- Upload sbom to release in workflow_call context ([#1913](https://github.com/hrzlgnm/mdns-browser/pull/1913))

- Use env vars for secrets in workflows to prevent log exposure ([#1914](https://github.com/hrzlgnm/mdns-browser/pull/1914))

## [1.5.1] - 2026-02-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.0...mdns-browser-v1.5.1)

### Changed

- Use binary without bundler type in void package ([#1904](https://github.com/hrzlgnm/mdns-browser/pull/1904))

- Update year in LICENSE ([#1905](https://github.com/hrzlgnm/mdns-browser/pull/1905))

## [1.5.0] - 2026-02-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.4.2...mdns-browser-v1.5.0)

### Added

- Enable auto updates for bundle types deb and rpm ([#1900](https://github.com/hrzlgnm/mdns-browser/pull/1900))

### Changed

- Update aur templates to remove stripping ([#1902](https://github.com/hrzlgnm/mdns-browser/pull/1902))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 8534b6a ([#1899](https://github.com/hrzlgnm/mdns-browser/pull/1899))

- *(deps)* Update dependency tauri-cli to v2.10.0 ([#1903](https://github.com/hrzlgnm/mdns-browser/pull/1903))

## [1.4.2] - 2026-02-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.4.1...mdns-browser-v1.4.2)

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v2.0.1 ([#1888](https://github.com/hrzlgnm/mdns-browser/pull/1888))

- *(deps)* Update rust crate clap to v4.5.59 ([#1889](https://github.com/hrzlgnm/mdns-browser/pull/1889))

- *(deps)* Update rust crate leptos to v0.8.16 ([#1890](https://github.com/hrzlgnm/mdns-browser/pull/1890))

- *(deps)* Lock file maintenance ([#1892](https://github.com/hrzlgnm/mdns-browser/pull/1892))

- *(deps)* Update archlinux:base-devel docker digest to 839e930 ([#1893](https://github.com/hrzlgnm/mdns-browser/pull/1893))

- *(deps)* Update ubuntu:latest docker digest to 25f3726 ([#1894](https://github.com/hrzlgnm/mdns-browser/pull/1894))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d5b1842 ([#1895](https://github.com/hrzlgnm/mdns-browser/pull/1895))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 9c8532d ([#1896](https://github.com/hrzlgnm/mdns-browser/pull/1896))

- *(deps)* Update ubuntu:latest docker digest to d1e2e92 ([#1897](https://github.com/hrzlgnm/mdns-browser/pull/1897))

- *(deps)* Update rust crate leptos to v0.8.16 ([#1898](https://github.com/hrzlgnm/mdns-browser/pull/1898))

## [1.4.1] - 2026-02-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.4.0...mdns-browser-v1.4.1)

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.127 ([#1882](https://github.com/hrzlgnm/mdns-browser/pull/1882))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to ee97f53 ([#1883](https://github.com/hrzlgnm/mdns-browser/pull/1883))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 3a00381 ([#1884](https://github.com/hrzlgnm/mdns-browser/pull/1884))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 7a1cffc ([#1885](https://github.com/hrzlgnm/mdns-browser/pull/1885))

- *(deps)* Update rust crate futures to v0.3.32 ([#1886](https://github.com/hrzlgnm/mdns-browser/pull/1886))

- *(deps)* Update rust crate mdns-sd to 0.18 ([#1887](https://github.com/hrzlgnm/mdns-browser/pull/1887))

## [1.4.0] - 2026-02-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.2...mdns-browser-v1.4.0)

### Changed

- Add link to terminal based app ([#1872](https://github.com/hrzlgnm/mdns-browser/pull/1872))

- Fix wording and indentation ([#1874](https://github.com/hrzlgnm/mdns-browser/pull/1874))

- Consolidate release workflows into single unified workflow ([#1878](https://github.com/hrzlgnm/mdns-browser/pull/1878))

- Rename release workflow to be more shorter and concise ([#1881](https://github.com/hrzlgnm/mdns-browser/pull/1881))

### Dependencies

- *(deps)* Update re-actors/alls-green digest to a638d64 ([#1867](https://github.com/hrzlgnm/mdns-browser/pull/1867))

- *(deps)* Update hrzlgnm/actions action to v1.6.7 ([#1869](https://github.com/hrzlgnm/mdns-browser/pull/1869))

- *(deps)* Update archlinux:base-devel docker digest to 7c81df5 ([#1868](https://github.com/hrzlgnm/mdns-browser/pull/1868))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 1912107 ([#1870](https://github.com/hrzlgnm/mdns-browser/pull/1870))

- *(deps)* Update hrzlgnm/actions action to v2 ([#1871](https://github.com/hrzlgnm/mdns-browser/pull/1871))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 2a907be ([#1873](https://github.com/hrzlgnm/mdns-browser/pull/1873))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 3176c6b ([#1876](https://github.com/hrzlgnm/mdns-browser/pull/1876))

- *(deps)* Update rust crate clap to v4.5.58 ([#1877](https://github.com/hrzlgnm/mdns-browser/pull/1877))

- *(deps)* Update dependency cargo-edit to v0.13.8 ([#1879](https://github.com/hrzlgnm/mdns-browser/pull/1879))

### Fixed

- Various issues in the new release workflow ([#1880](https://github.com/hrzlgnm/mdns-browser/pull/1880))

## [1.3.2] - 2026-02-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.1...mdns-browser-v1.3.2)

### Changed

- Add man page ([#1866](https://github.com/hrzlgnm/mdns-browser/pull/1866))

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v1.6.6 ([#1864](https://github.com/hrzlgnm/mdns-browser/pull/1864))

- *(deps)* Update re-actors/alls-green digest to b4ca9c2 ([#1865](https://github.com/hrzlgnm/mdns-browser/pull/1865))

## [1.3.1] - 2026-02-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.0...mdns-browser-v1.3.1)

### Changed

- Tweak fetch-depth of checkout steps ([#1857](https://github.com/hrzlgnm/mdns-browser/pull/1857))

- Update MSRV to 1.88 to allow for security updates ([#1861](https://github.com/hrzlgnm/mdns-browser/pull/1861))

### Dependencies

- *(deps)* Update anchore/sbom-action digest to 28d7154 ([#1853](https://github.com/hrzlgnm/mdns-browser/pull/1853))

- *(deps)* Update anchore/scan-action action to v7.3.2 ([#1854](https://github.com/hrzlgnm/mdns-browser/pull/1854))

- *(deps)* Update hrzlgnm/actions action to v1.6.4 ([#1855](https://github.com/hrzlgnm/mdns-browser/pull/1855))

- *(deps)* Update tauri monorepo ([#1856](https://github.com/hrzlgnm/mdns-browser/pull/1856))

- *(deps)* Update dependency cargo-nextest to v0.9.126 ([#1858](https://github.com/hrzlgnm/mdns-browser/pull/1858))

- *(deps)* Update hrzlgnm/actions action to v1.6.5 ([#1859](https://github.com/hrzlgnm/mdns-browser/pull/1859))

- *(deps)* Lock file maintenance ([#1862](https://github.com/hrzlgnm/mdns-browser/pull/1862))

## [1.3.0] - 2026-02-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.3...mdns-browser-v1.3.0)

### Added

- Enable stripping ([#1837](https://github.com/hrzlgnm/mdns-browser/pull/1837))

### Changed

- Add AGENTS.md ([#1833](https://github.com/hrzlgnm/mdns-browser/pull/1833))

- Simplify clippy command and also check tests ([#1834](https://github.com/hrzlgnm/mdns-browser/pull/1834))

- Configure version resolver in release drafter ([#1835](https://github.com/hrzlgnm/mdns-browser/pull/1835))

- Remove version extraction step from release drafter ([#1836](https://github.com/hrzlgnm/mdns-browser/pull/1836))

- Remove Release Drafter trigger from bump version workflow ([#1838](https://github.com/hrzlgnm/mdns-browser/pull/1838))

- Add 'enhancement' label to minor version changes ([#1852](https://github.com/hrzlgnm/mdns-browser/pull/1852))

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v1.6.3 ([#1846](https://github.com/hrzlgnm/mdns-browser/pull/1846))

- *(deps)* Update mikepenz/action-junit-report digest to 74626db ([#1843](https://github.com/hrzlgnm/mdns-browser/pull/1843))

- *(deps)* Update archlinux:base-devel docker digest to 9387492 ([#1841](https://github.com/hrzlgnm/mdns-browser/pull/1841))

- *(deps)* Update dependency cargo-nextest to v0.9.125 ([#1845](https://github.com/hrzlgnm/mdns-browser/pull/1845))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 28fdbec ([#1842](https://github.com/hrzlgnm/mdns-browser/pull/1842))

- *(deps)* Update anchore/sbom-action digest to deef08a ([#1840](https://github.com/hrzlgnm/mdns-browser/pull/1840))

- *(deps)* Update actions/checkout digest to de0fac2 ([#1839](https://github.com/hrzlgnm/mdns-browser/pull/1839))

- *(deps)* Update actions/attest-build-provenance action to v3.2.0 ([#1847](https://github.com/hrzlgnm/mdns-browser/pull/1847))

- *(deps)* Update anchore/scan-action action to v7.3.1 ([#1844](https://github.com/hrzlgnm/mdns-browser/pull/1844))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 6f471f9 ([#1850](https://github.com/hrzlgnm/mdns-browser/pull/1850))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to ae41dd8 ([#1849](https://github.com/hrzlgnm/mdns-browser/pull/1849))

- *(deps)* Update dependency tauri-cli to v2.10.0 ([#1848](https://github.com/hrzlgnm/mdns-browser/pull/1848))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 5c4ee0a ([#1851](https://github.com/hrzlgnm/mdns-browser/pull/1851))

## [1.2.3] - 2026-02-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.2...mdns-browser-v1.2.3)

### Dependencies

- *(deps)* Lock file maintenance ([#1832](https://github.com/hrzlgnm/mdns-browser/pull/1832))

## [1.2.2] - 2026-02-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.1...mdns-browser-v1.2.2)

### Changed

- Fix workflow typo and improve SSH key handling ([#1829](https://github.com/hrzlgnm/mdns-browser/pull/1829))

- Quote environment vars when writing to github environment ([#1830](https://github.com/hrzlgnm/mdns-browser/pull/1830))

- Ensure new line in AUR deploy key setup ([#1831](https://github.com/hrzlgnm/mdns-browser/pull/1831))

- Increase release optimization to level 'z' and retain debug info ([#1825](https://github.com/hrzlgnm/mdns-browser/pull/1825))

## [1.2.1] - 2026-01-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.0...mdns-browser-v1.2.1)

### Fixed

- Ignore service type removal events ([#1828](https://github.com/hrzlgnm/mdns-browser/pull/1828))

## [1.2.0] - 2026-01-27 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.1.2...mdns-browser-v1.2.0)

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.123 ([#1822](https://github.com/hrzlgnm/mdns-browser/pull/1822))

- *(deps)* Update dependency cargo-nextest to v0.9.124 ([#1823](https://github.com/hrzlgnm/mdns-browser/pull/1823))

### Fixed

- Skip service subtypes in service type enumeration ([#1824](https://github.com/hrzlgnm/mdns-browser/pull/1824))

## [1.1.2] - 2026-01-22 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.1.1...mdns-browser-v1.1.2)

### Changed

- Vendor thaw-ui to be able to update dependencies ([#1806](https://github.com/hrzlgnm/mdns-browser/pull/1806))

### Dependencies

- *(deps)* Update rust crate thiserror to v2.0.18 ([#1808](https://github.com/hrzlgnm/mdns-browser/pull/1808))

- *(deps)* Update ubuntu:latest docker digest to cd1dba6 ([#1809](https://github.com/hrzlgnm/mdns-browser/pull/1809))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 651fd6a ([#1810](https://github.com/hrzlgnm/mdns-browser/pull/1810))

- *(deps)* Update hrzlgnm/actions action to v1.6.2 ([#1811](https://github.com/hrzlgnm/mdns-browser/pull/1811))

- *(deps)* Update release-drafter/release-drafter action to v6.1.1 ([#1812](https://github.com/hrzlgnm/mdns-browser/pull/1812))

- *(deps)* Update archlinux:base-devel docker digest to d2bd09b ([#1813](https://github.com/hrzlgnm/mdns-browser/pull/1813))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 2a774f7 ([#1814](https://github.com/hrzlgnm/mdns-browser/pull/1814))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 4241b9d ([#1815](https://github.com/hrzlgnm/mdns-browser/pull/1815))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 650f432 ([#1816](https://github.com/hrzlgnm/mdns-browser/pull/1816))

- *(deps)* Update anchore/sbom-action digest to 62ad528 ([#1817](https://github.com/hrzlgnm/mdns-browser/pull/1817))

- *(deps)* Update anchore/scan-action action to v7.3.0 ([#1818](https://github.com/hrzlgnm/mdns-browser/pull/1818))

- *(deps)* Update actions/setup-java digest to be666c2 ([#1819](https://github.com/hrzlgnm/mdns-browser/pull/1819))

- *(deps)* Update release-drafter/release-drafter action to v6.2.0 ([#1820](https://github.com/hrzlgnm/mdns-browser/pull/1820))

- *(deps)* Lock file maintenance ([#1821](https://github.com/hrzlgnm/mdns-browser/pull/1821))

## [1.1.1] - 2026-01-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.1.0...mdns-browser-v1.1.1)

### Dependencies

- *(deps)* Update rust crate mdns-sd to v0.17.2 ([#1802](https://github.com/hrzlgnm/mdns-browser/pull/1802))

- *(deps)* Lock file maintenance ([#1803](https://github.com/hrzlgnm/mdns-browser/pull/1803))

## [1.1.0] - 2026-01-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.8...mdns-browser-v1.1.0)

### Added

- Reduce splashscreen duration ([#1798](https://github.com/hrzlgnm/mdns-browser/pull/1798))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to e115fa3 ([#1794](https://github.com/hrzlgnm/mdns-browser/pull/1794))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to c54bcb7 ([#1795](https://github.com/hrzlgnm/mdns-browser/pull/1795))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to a974cb8 ([#1796](https://github.com/hrzlgnm/mdns-browser/pull/1796))

- *(deps)* Update rust crate chrono to v0.4.43 ([#1797](https://github.com/hrzlgnm/mdns-browser/pull/1797))

- *(deps)* Update ubuntu:latest docker digest to 7a39814 ([#1799](https://github.com/hrzlgnm/mdns-browser/pull/1799))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 355deb4 ([#1800](https://github.com/hrzlgnm/mdns-browser/pull/1800))

- *(deps)* Lock file maintenance ([#1801](https://github.com/hrzlgnm/mdns-browser/pull/1801))

## [1.0.8] - 2026-01-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.7...mdns-browser-v1.0.8)

### Dependencies

- *(deps)* Update dependency cargo-nextest to v0.9.118 ([#1779](https://github.com/hrzlgnm/mdns-browser/pull/1779))

- *(deps)* Update archlinux:base-devel docker digest to ebcaeca ([#1780](https://github.com/hrzlgnm/mdns-browser/pull/1780))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 675f6b3 ([#1781](https://github.com/hrzlgnm/mdns-browser/pull/1781))

- *(deps)* Update rust crate serde_json to v1.0.149 ([#1782](https://github.com/hrzlgnm/mdns-browser/pull/1782))

- *(deps)* Update dependency cargo-nextest to v0.9.120 ([#1783](https://github.com/hrzlgnm/mdns-browser/pull/1783))

- *(deps)* Update anchore/sbom-action digest to 0b82b0b ([#1784](https://github.com/hrzlgnm/mdns-browser/pull/1784))

- *(deps)* Update anchore/scan-action action to v7.2.3 ([#1785](https://github.com/hrzlgnm/mdns-browser/pull/1785))

- *(deps)* Update hrzlgnm/actions action to v1.6.1 ([#1786](https://github.com/hrzlgnm/mdns-browser/pull/1786))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.3 ([#1787](https://github.com/hrzlgnm/mdns-browser/pull/1787))

- *(deps)* Update rust crate tauri-plugin-log to v2.8.0 ([#1788](https://github.com/hrzlgnm/mdns-browser/pull/1788))

- *(deps)* Update dependency cargo-nextest to v0.9.121 ([#1789](https://github.com/hrzlgnm/mdns-browser/pull/1789))

- *(deps)* Update archlinux:base-devel docker digest to 84c36fa ([#1790](https://github.com/hrzlgnm/mdns-browser/pull/1790))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 3596d1a ([#1791](https://github.com/hrzlgnm/mdns-browser/pull/1791))

- *(deps)* Update dependency cargo-nextest to v0.9.122 ([#1792](https://github.com/hrzlgnm/mdns-browser/pull/1792))

- *(deps)* Lock file maintenance ([#1793](https://github.com/hrzlgnm/mdns-browser/pull/1793))

## [1.0.7] - 2026-01-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.6...mdns-browser-v1.0.7)

### Changed

- *(zed)* Extend zed tasks ([#1764](https://github.com/hrzlgnm/mdns-browser/pull/1764))

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v1.5.7 ([#1756](https://github.com/hrzlgnm/mdns-browser/pull/1756))

- *(deps)* Update rust crate serde_json to v1.0.146 ([#1758](https://github.com/hrzlgnm/mdns-browser/pull/1758))

- *(deps)* Update anchore/sbom-action digest to a930d0a ([#1759](https://github.com/hrzlgnm/mdns-browser/pull/1759))

- *(deps)* Update archlinux:base-devel docker digest to 0a03ad5 ([#1760](https://github.com/hrzlgnm/mdns-browser/pull/1760))

- *(deps)* Update rust crate serde_json to v1.0.147 ([#1761](https://github.com/hrzlgnm/mdns-browser/pull/1761))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 9d818bf ([#1762](https://github.com/hrzlgnm/mdns-browser/pull/1762))

- *(deps)* Update dependency cargo-nextest to v0.9.116 ([#1765](https://github.com/hrzlgnm/mdns-browser/pull/1765))

- *(deps)* Update rust crate serde_json to v1.0.148 ([#1766](https://github.com/hrzlgnm/mdns-browser/pull/1766))

- *(deps)* Update archlinux:base-devel docker digest to f6b259c ([#1767](https://github.com/hrzlgnm/mdns-browser/pull/1767))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 142b950 ([#1768](https://github.com/hrzlgnm/mdns-browser/pull/1768))

- *(deps)* Update mikepenz/action-junit-report digest to a294a61 ([#1769](https://github.com/hrzlgnm/mdns-browser/pull/1769))

- *(deps)* Update hrzlgnm/actions action to v1.5.8 ([#1770](https://github.com/hrzlgnm/mdns-browser/pull/1770))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to a1c486c ([#1771](https://github.com/hrzlgnm/mdns-browser/pull/1771))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 6137e47 ([#1772](https://github.com/hrzlgnm/mdns-browser/pull/1772))

- *(deps)* Update hrzlgnm/actions action to v1.6.0 ([#1774](https://github.com/hrzlgnm/mdns-browser/pull/1774))

- *(deps)* Update dependency cargo-nextest to v0.9.117 ([#1773](https://github.com/hrzlgnm/mdns-browser/pull/1773))

- *(deps)* Update rust crate clap to v4.5.54 ([#1775](https://github.com/hrzlgnm/mdns-browser/pull/1775))

- *(deps)* Update rust crate tokio to v1.49.0 ([#1777](https://github.com/hrzlgnm/mdns-browser/pull/1777))

- *(deps)* Update tauri-apps/tauri-action action to v0.6.1 ([#1776](https://github.com/hrzlgnm/mdns-browser/pull/1776))

- *(deps)* Update rust crate icondata to 0.7 ([#1778](https://github.com/hrzlgnm/mdns-browser/pull/1778))

## [1.0.6] - 2025-12-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.5...mdns-browser-v1.0.6)

### Changed

- Make package version links ([#1748](https://github.com/hrzlgnm/mdns-browser/pull/1748))

- Make latest releases links, too ([#1749](https://github.com/hrzlgnm/mdns-browser/pull/1749))

- Fix spacing in acknowledgment section of README ([#1753](https://github.com/hrzlgnm/mdns-browser/pull/1753))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 9414f5b ([#1750](https://github.com/hrzlgnm/mdns-browser/pull/1750))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 068aaec ([#1751](https://github.com/hrzlgnm/mdns-browser/pull/1751))

- *(deps)* Update actions/attest-build-provenance action to v3.1.0 ([#1752](https://github.com/hrzlgnm/mdns-browser/pull/1752))

- *(deps)* Update rust crate leptos to v0.8.15 ([#1754](https://github.com/hrzlgnm/mdns-browser/pull/1754))

- *(deps)* Lock file maintenance ([#1755](https://github.com/hrzlgnm/mdns-browser/pull/1755))

## [1.0.5] - 2025-12-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.4...mdns-browser-v1.0.5)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 2498279 ([#1739](https://github.com/hrzlgnm/mdns-browser/pull/1739))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 8b37b98 ([#1740](https://github.com/hrzlgnm/mdns-browser/pull/1740))

- *(deps)* Update dependency cargo-nextest to v0.9.115 ([#1742](https://github.com/hrzlgnm/mdns-browser/pull/1742))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f0d8065 ([#1741](https://github.com/hrzlgnm/mdns-browser/pull/1741))

- *(deps)* Update archlinux:base-devel docker digest to 1635f38 ([#1744](https://github.com/hrzlgnm/mdns-browser/pull/1744))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 8babd70 ([#1746](https://github.com/hrzlgnm/mdns-browser/pull/1746))

- *(deps)* Lock file maintenance ([#1747](https://github.com/hrzlgnm/mdns-browser/pull/1747))

### Fixed

- *(void)* Use correct version outputs after refactoring ([#1745](https://github.com/hrzlgnm/mdns-browser/pull/1745))

### Maintenance

- *(ci)* Draft releases with latest tag reflected in tauri config ([#1738](https://github.com/hrzlgnm/mdns-browser/pull/1738))

## [1.0.4] - 2025-12-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.3...mdns-browser-v1.0.4)

### Maintenance

- *(ci)* Add missing actions write permissions ([#1737](https://github.com/hrzlgnm/mdns-browser/pull/1737))

## [1.0.3] - 2025-12-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.2...mdns-browser-v1.0.3)

### Maintenance

- *(ci)* Pass gh token to gh workflow run ([#1736](https://github.com/hrzlgnm/mdns-browser/pull/1736))

## [1.0.2] - 2025-12-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.1...mdns-browser-v1.0.2)

### Maintenance

- *(ci)* Trigger release drafter run after bumping the version ([#1735](https://github.com/hrzlgnm/mdns-browser/pull/1735))

## [1.0.1] - 2025-12-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.0...mdns-browser-v1.0.1)

### Added

- Show additional IP addresses hint ([#1734](https://github.com/hrzlgnm/mdns-browser/pull/1734))

### Dependencies

- *(deps)* Update hrzlgnm/actions action to v1.5.4 ([#1723](https://github.com/hrzlgnm/mdns-browser/pull/1723))

- *(deps)* Update hrzlgnm/actions action to v1.5.5 ([#1724](https://github.com/hrzlgnm/mdns-browser/pull/1724))

- *(deps)* Pin actions/labeler action to 634933e ([#1727](https://github.com/hrzlgnm/mdns-browser/pull/1727))

- *(deps)* Update tauri monorepo ([#1728](https://github.com/hrzlgnm/mdns-browser/pull/1728))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to ed6cc8a ([#1729](https://github.com/hrzlgnm/mdns-browser/pull/1729))

- *(deps)* Update anchore/scan-action action to v7.2.2 ([#1731](https://github.com/hrzlgnm/mdns-browser/pull/1731))

- *(deps)* Update anchore/sbom-action digest to 43a17d6 ([#1730](https://github.com/hrzlgnm/mdns-browser/pull/1730))

- *(deps)* Update github artifact actions (major) ([#1732](https://github.com/hrzlgnm/mdns-browser/pull/1732))

- *(deps)* Update hrzlgnm/actions action to v1.5.6 ([#1733](https://github.com/hrzlgnm/mdns-browser/pull/1733))

### Maintenance

- *(ci)* Migrate to maintained actions/labeler ([#1726](https://github.com/hrzlgnm/mdns-browser/pull/1726))

## [1.0.0] - 2025-12-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.28.1...mdns-browser-v1.0.0)

### Changed

- Add winget version badge ([#1705](https://github.com/hrzlgnm/mdns-browser/pull/1705))

- Add AUR version badge ([#1714](https://github.com/hrzlgnm/mdns-browser/pull/1714))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 6450668 ([#1706](https://github.com/hrzlgnm/mdns-browser/pull/1706))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 17b99ea ([#1707](https://github.com/hrzlgnm/mdns-browser/pull/1707))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to ce0efdf ([#1708](https://github.com/hrzlgnm/mdns-browser/pull/1708))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to f5d88ca ([#1709](https://github.com/hrzlgnm/mdns-browser/pull/1709))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 415895b ([#1710](https://github.com/hrzlgnm/mdns-browser/pull/1710))

- *(deps)* Update softprops/action-gh-release digest to a06a81a ([#1711](https://github.com/hrzlgnm/mdns-browser/pull/1711))

- *(deps)* Update hrzlgnm/actions action to v1.5.1 ([#1713](https://github.com/hrzlgnm/mdns-browser/pull/1713))

- *(deps)* Update actions/checkout digest to 8e8c483 ([#1715](https://github.com/hrzlgnm/mdns-browser/pull/1715))

- *(deps)* Update hrzlgnm/actions action to v1.5.2 ([#1716](https://github.com/hrzlgnm/mdns-browser/pull/1716))

- *(deps)* Update rust crate log to v0.4.29 ([#1717](https://github.com/hrzlgnm/mdns-browser/pull/1717))

- *(deps)* Update hrzlgnm/actions action to v1.5.3 ([#1718](https://github.com/hrzlgnm/mdns-browser/pull/1718))

- *(deps)* Update actions/setup-java digest to f2beeb2 ([#1719](https://github.com/hrzlgnm/mdns-browser/pull/1719))

- *(deps)* Lock file maintenance ([#1720](https://github.com/hrzlgnm/mdns-browser/pull/1720))

- *(deps)* Update rust crate mdns-sd to v0.17.1 ([#1722](https://github.com/hrzlgnm/mdns-browser/pull/1722))

### Maintenance

- *(ci)* Update schedule to rerun CI to the middle of the month ([#1712](https://github.com/hrzlgnm/mdns-browser/pull/1712))

## [0.28.1] - 2025-12-01 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.28.0...mdns-browser-v0.28.1)

### Dependencies

- *(deps)* Lock file maintenance ([#1704](https://github.com/hrzlgnm/mdns-browser/pull/1704))

### Maintenance

- *(ci)* Fix missing sbom for android on publish ([#1703](https://github.com/hrzlgnm/mdns-browser/pull/1703))

## [0.28.0] - 2025-11-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.8...mdns-browser-v0.28.0)

### Changed

- *(copyright)* Add license headers ([#1691](https://github.com/hrzlgnm/mdns-browser/pull/1691))

- Move local crates into directory `crates` ([#1697](https://github.com/hrzlgnm/mdns-browser/pull/1697))

- Move packaging related directories to `packaging` ([#1700](https://github.com/hrzlgnm/mdns-browser/pull/1700))

- Move screenshots to docs/assets ([#1701](https://github.com/hrzlgnm/mdns-browser/pull/1701))

- Tweak coderabbit settings ([#1702](https://github.com/hrzlgnm/mdns-browser/pull/1702))

### Dependencies

- *(deps)* Update tauri monorepo ([#1688](https://github.com/hrzlgnm/mdns-browser/pull/1688))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 1dbe9e1 ([#1692](https://github.com/hrzlgnm/mdns-browser/pull/1692))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 561cb55 ([#1693](https://github.com/hrzlgnm/mdns-browser/pull/1693))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 0a6535c ([#1695](https://github.com/hrzlgnm/mdns-browser/pull/1695))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 8bc5efb ([#1694](https://github.com/hrzlgnm/mdns-browser/pull/1694))

- *(deps)* Update hrzlgnm/actions action to v1.4.0 ([#1696](https://github.com/hrzlgnm/mdns-browser/pull/1696))

- *(deps)* Update hrzlgnm/actions action to v1.5.0 ([#1698](https://github.com/hrzlgnm/mdns-browser/pull/1698))

### Maintenance

- *(ci)* Don't create and scan SBOM when not publishing ([#1699](https://github.com/hrzlgnm/mdns-browser/pull/1699))

## [0.27.8] - 2025-11-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.7...mdns-browser-v0.27.8)

### Fixed

- Add workaround --no-sign not working as expected ([#1687](https://github.com/hrzlgnm/mdns-browser/pull/1687))

## [0.27.7] - 2025-11-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.6...mdns-browser-v0.27.7)

### Changed

- Add feature request issue template ([#1665](https://github.com/hrzlgnm/mdns-browser/pull/1665))

- *(aur)* Disable signing when building the bundle artifacts ([#1670](https://github.com/hrzlgnm/mdns-browser/pull/1670))

- *(doc)* Update license year ([#1671](https://github.com/hrzlgnm/mdns-browser/pull/1671))

- Add license headers ([#1674](https://github.com/hrzlgnm/mdns-browser/pull/1674))

- Tweak typos settings ([#1679](https://github.com/hrzlgnm/mdns-browser/pull/1679))

- Hack to not ignore `.github/` directory ([#1683](https://github.com/hrzlgnm/mdns-browser/pull/1683))

### Dependencies

- *(deps)* Update anchore/scan-action action to v7.2.1 ([#1660](https://github.com/hrzlgnm/mdns-browser/pull/1660))

- *(deps)* Update rust crate leptos to v0.8.14 ([#1668](https://github.com/hrzlgnm/mdns-browser/pull/1668))

- *(deps)* Update swatinem/rust-cache digest to 779680d ([#1673](https://github.com/hrzlgnm/mdns-browser/pull/1673))

- *(deps)* Update crate-ci/typos action to v1.40.0 ([#1676](https://github.com/hrzlgnm/mdns-browser/pull/1676))

- *(deps)* Update hrzlgnm/actions action to v1.1.0 ([#1681](https://github.com/hrzlgnm/mdns-browser/pull/1681))

- *(deps)* Update rust crate serde_with to v3.16.1 ([#1682](https://github.com/hrzlgnm/mdns-browser/pull/1682))

- *(deps)* Update hrzlgnm/actions action to v1.2.2 ([#1684](https://github.com/hrzlgnm/mdns-browser/pull/1684))

- *(deps)* Update hrzlgnm/actions action to v1.3.0 ([#1685](https://github.com/hrzlgnm/mdns-browser/pull/1685))

- *(deps)* Lock file maintenance ([#1686](https://github.com/hrzlgnm/mdns-browser/pull/1686))

### Maintenance

- *(ci)* Update release drafter template and pr-labeler settings ([#1662](https://github.com/hrzlgnm/mdns-browser/pull/1662))

- *(ci)* Demote full changelog heading in release-drafter template ([#1663](https://github.com/hrzlgnm/mdns-browser/pull/1663))

- *(ci)* Align release drafter template with tagging schema ([#1664](https://github.com/hrzlgnm/mdns-browser/pull/1664))

- *(ci)* Rename reusable workflows for better clarity ([#1661](https://github.com/hrzlgnm/mdns-browser/pull/1661))

- *(ci)* Update rust-cache action configuration ([#1666](https://github.com/hrzlgnm/mdns-browser/pull/1666))

- *(ci)* Sync our winget-pkgs fork before updating ([#1667](https://github.com/hrzlgnm/mdns-browser/pull/1667))

- *(ci)* Refactor lint workflow into rustfmt and leptosfmt jobs ([#1669](https://github.com/hrzlgnm/mdns-browser/pull/1669))

- *(ci)* Handle removed or renamed Dockerfiles properly ([#1675](https://github.com/hrzlgnm/mdns-browser/pull/1675))

- *(ci)* Externalize docker workflow ([#1677](https://github.com/hrzlgnm/mdns-browser/pull/1677))

- *(ci)* Replace local typos workflow with external action ([#1680](https://github.com/hrzlgnm/mdns-browser/pull/1680))

## [0.27.6] - 2025-11-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.5...mdns-browser-v0.27.6)

### Changed

- Add badges for license and build status ([#1658](https://github.com/hrzlgnm/mdns-browser/pull/1658))

### Dependencies

- *(deps)* Update actions/checkout action to v6 ([#1639](https://github.com/hrzlgnm/mdns-browser/pull/1639))

- *(deps)* Update rust crate leptos to v0.8.13 ([#1647](https://github.com/hrzlgnm/mdns-browser/pull/1647))

- *(deps)* Pin dependencies ([#1648](https://github.com/hrzlgnm/mdns-browser/pull/1648))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 91adfd6 ([#1650](https://github.com/hrzlgnm/mdns-browser/pull/1650))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 15ee614 ([#1652](https://github.com/hrzlgnm/mdns-browser/pull/1652))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to f863756 ([#1651](https://github.com/hrzlgnm/mdns-browser/pull/1651))

- *(deps)* Update re-actors/alls-green digest to 2765efe ([#1656](https://github.com/hrzlgnm/mdns-browser/pull/1656))

- *(deps)* Lock file maintenance ([#1659](https://github.com/hrzlgnm/mdns-browser/pull/1659))

### Maintenance

- *(ci)* Enable caching of db when using anchore/scan-action ([#1641](https://github.com/hrzlgnm/mdns-browser/pull/1641))

- *(ci)* Ignore GHSA-wrw7-89jp-8q8g in grype scanning ([#1642](https://github.com/hrzlgnm/mdns-browser/pull/1642))

- *(ci)* Consolidate common sbom steps into a composite action ([#1644](https://github.com/hrzlgnm/mdns-browser/pull/1644))

- *(ci)* Rename desktop sbom step to align with android ([#1645](https://github.com/hrzlgnm/mdns-browser/pull/1645))

- *(ci)* Cleanup docker build workflow ([#1646](https://github.com/hrzlgnm/mdns-browser/pull/1646))

- *(ci)* Fix tagging of docker builds and add job names ([#1649](https://github.com/hrzlgnm/mdns-browser/pull/1649))

- *(ci)* Slimify workflows ([#1653](https://github.com/hrzlgnm/mdns-browser/pull/1653))

- *(ci)* Refactor to use re-actors/all-green ([#1655](https://github.com/hrzlgnm/mdns-browser/pull/1655))

## [0.27.5] - 2025-11-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.4...mdns-browser-v0.27.5)

### Changed

- *(aur)* Use another directory in lint step ([#1638](https://github.com/hrzlgnm/mdns-browser/pull/1638))

## [0.27.4] - 2025-11-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.3...mdns-browser-v0.27.4)

### Changed

- *(aur)* Install namcap for linting PKGBUILD ([#1621](https://github.com/hrzlgnm/mdns-browser/pull/1621))

- *(aur)* Only perform minimal checks in pull requests ([#1622](https://github.com/hrzlgnm/mdns-browser/pull/1622))

### Dependencies

- *(deps)* Lock file maintenance ([#1615](https://github.com/hrzlgnm/mdns-browser/pull/1615))

- *(deps)* Update rust crate clap to v4.5.52 ([#1619](https://github.com/hrzlgnm/mdns-browser/pull/1619))

- *(deps)* Update actions/checkout digest to 93cb6ef ([#1618](https://github.com/hrzlgnm/mdns-browser/pull/1618))

- *(deps)* Update anchore/sbom-action digest to fbfd9c6 ([#1625](https://github.com/hrzlgnm/mdns-browser/pull/1625))

- *(deps)* Update anchore/scan-action action to v7.2.0 ([#1627](https://github.com/hrzlgnm/mdns-browser/pull/1627))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 7a17003 ([#1629](https://github.com/hrzlgnm/mdns-browser/pull/1629))

- *(deps)* Update dependency cargo-nextest to v0.9.114 ([#1631](https://github.com/hrzlgnm/mdns-browser/pull/1631))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 55d821f ([#1628](https://github.com/hrzlgnm/mdns-browser/pull/1628))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 86a96fe ([#1630](https://github.com/hrzlgnm/mdns-browser/pull/1630))

- *(deps)* Update rust crate clap to v4.5.53 ([#1636](https://github.com/hrzlgnm/mdns-browser/pull/1636))

- *(deps)* Lock file maintenance ([#1637](https://github.com/hrzlgnm/mdns-browser/pull/1637))

### Maintenance

- *(ci)* Run tests with nextest runner ([#1617](https://github.com/hrzlgnm/mdns-browser/pull/1617))

- *(ci)* Run CI workflow once per month ([#1634](https://github.com/hrzlgnm/mdns-browser/pull/1634))

- *(ci)* Optimize change detection to not trigger unrelated jobs ([#1635](https://github.com/hrzlgnm/mdns-browser/pull/1635))

## [0.27.3] - 2025-11-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.2...mdns-browser-v0.27.3)

### Maintenance

- *(ci)* Create SBOM before building ([#1611](https://github.com/hrzlgnm/mdns-browser/pull/1611))

## [0.27.2] - 2025-11-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.1...mdns-browser-v0.27.2)

### Dependencies

- *(deps)* Pin anchore/scan-action action to 568b89d ([#1609](https://github.com/hrzlgnm/mdns-browser/pull/1609))

### Maintenance

- *(ci)* Filter changes more granulary ([#1610](https://github.com/hrzlgnm/mdns-browser/pull/1610))

## [0.27.1] - 2025-11-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.0...mdns-browser-v0.27.1)

### Changed

- *(desktop)* Prefer NSIS updater in latest.json ([#1603](https://github.com/hrzlgnm/mdns-browser/pull/1603))

## [0.27.0] - 2025-11-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.8...mdns-browser-v0.27.0)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to bff59e7 ([#1596](https://github.com/hrzlgnm/mdns-browser/pull/1596))

### Maintenance

- *(ci)* Fix dependencies after name changes ([#1591](https://github.com/hrzlgnm/mdns-browser/pull/1591))

- *(ci)* Rename workflow file for better maintainability ([#1592](https://github.com/hrzlgnm/mdns-browser/pull/1592))

- *(ci)* Consolidate release logic with composite action ([#1593](https://github.com/hrzlgnm/mdns-browser/pull/1593))

- *(ci)* Only build platform specific if platform workflow changes ([#1599](https://github.com/hrzlgnm/mdns-browser/pull/1599))

- *(ci)* Create sbom for android ([#1598](https://github.com/hrzlgnm/mdns-browser/pull/1598))

## [0.26.8] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.7...mdns-browser-v0.26.8)

### Maintenance

- *(ci)* Resolve an artifact name collision ([#1587](https://github.com/hrzlgnm/mdns-browser/pull/1587))

## [0.26.7] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.6...mdns-browser-v0.26.7)

### Maintenance

- *(ci)* Fix conflict of asset checksums with source checksums ([#1586](https://github.com/hrzlgnm/mdns-browser/pull/1586))

## [0.26.5] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.4...mdns-browser-v0.26.5)

### Changed

- *(aur)* Fix package options ([#1581](https://github.com/hrzlgnm/mdns-browser/pull/1581))

### Maintenance

- *(ci)* Build AUR package on AUR template changes ([#1582](https://github.com/hrzlgnm/mdns-browser/pull/1582))

- *(ci)* Refactor publishing ([#1584](https://github.com/hrzlgnm/mdns-browser/pull/1584))

## [0.26.4] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.3...mdns-browser-v0.26.4)

### Changed

- *(aur)* Build mdns-browser without bundling ([#1580](https://github.com/hrzlgnm/mdns-browser/pull/1580))

## [0.26.3] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.2...mdns-browser-v0.26.3)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 5f85072 ([#1576](https://github.com/hrzlgnm/mdns-browser/pull/1576))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 6752f84 ([#1578](https://github.com/hrzlgnm/mdns-browser/pull/1578))

### Maintenance

- *(ci)* Only build changed dockerfiles in pull requests ([#1577](https://github.com/hrzlgnm/mdns-browser/pull/1577))

## [0.26.2] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.1...mdns-browser-v0.26.2)

### Changed

- *(aur)* Enable default binary stripping in mdns-browser-bin package ([#1574](https://github.com/hrzlgnm/mdns-browser/pull/1574))

### Dependencies

- *(deps)* Update ubuntu:latest docker digest to c35e29c ([#1573](https://github.com/hrzlgnm/mdns-browser/pull/1573))

## [0.26.1] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.0...mdns-browser-v0.26.1)

### Maintenance

- *(ci)* Move android specific steps after rust steps ([#1572](https://github.com/hrzlgnm/mdns-browser/pull/1572))

## [0.26.0] - 2025-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.8...mdns-browser-v0.26.0)

### Changed

- Update updater endpoints ([#1571](https://github.com/hrzlgnm/mdns-browser/pull/1571))

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.39.1 ([#1565](https://github.com/hrzlgnm/mdns-browser/pull/1565))

- *(deps)* Update crate-ci/typos action to v1.39.2 ([#1566](https://github.com/hrzlgnm/mdns-browser/pull/1566))

- *(deps)* Update tauri monorepo ([#1567](https://github.com/hrzlgnm/mdns-browser/pull/1567))

- *(deps)* Update ubuntu:latest docker digest to e96e81f ([#1568](https://github.com/hrzlgnm/mdns-browser/pull/1568))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 1cc911e ([#1569](https://github.com/hrzlgnm/mdns-browser/pull/1569))

- *(deps)* Update rust crate serde_with to v3.16.0 ([#1570](https://github.com/hrzlgnm/mdns-browser/pull/1570))

## [0.25.8] - 2025-11-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.7...mdns-browser-v0.25.8)

### Maintenance

- *(ci)* Fix conditional `sccache` ([#1562](https://github.com/hrzlgnm/mdns-browser/pull/1562))

## [0.25.7] - 2025-11-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.6...mdns-browser-v0.25.7)

### Dependencies

- *(deps)* Update tauri-apps/tauri-action action to v0.6.0 ([#1561](https://github.com/hrzlgnm/mdns-browser/pull/1561))

### Maintenance

- *(ci)* Fix passing caching flags ([#1560](https://github.com/hrzlgnm/mdns-browser/pull/1560))

## [0.25.6] - 2025-11-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.5...mdns-browser-v0.25.6)

### Maintenance

- *(ci)* Handle enabling `sccache` correctly ([#1559](https://github.com/hrzlgnm/mdns-browser/pull/1559))

## [0.25.5] - 2025-11-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.4...mdns-browser-v0.25.5)

### Dependencies

- *(deps)* Update softprops/action-gh-release digest to 5be0e66 ([#1544](https://github.com/hrzlgnm/mdns-browser/pull/1544))

- *(deps)* Update dependency cargo-auditable to v0.7.2 ([#1545](https://github.com/hrzlgnm/mdns-browser/pull/1545))

- *(deps)* Update dependency tauri-cli to v2.9.4 ([#1546](https://github.com/hrzlgnm/mdns-browser/pull/1546))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 2291d7e ([#1547](https://github.com/hrzlgnm/mdns-browser/pull/1547))

- *(deps)* Lock file maintenance ([#1548](https://github.com/hrzlgnm/mdns-browser/pull/1548))

- *(deps)* Pin dependencies ([#1553](https://github.com/hrzlgnm/mdns-browser/pull/1553))

- *(deps)* Update actions/checkout action to v5 ([#1554](https://github.com/hrzlgnm/mdns-browser/pull/1554))

### Maintenance

- *(ci)* Refactor CI into a single workflow ([#1550](https://github.com/hrzlgnm/mdns-browser/pull/1550))

- *(ci)* Always run typos regardless of changes ([#1555](https://github.com/hrzlgnm/mdns-browser/pull/1555))

- *(ci)* Filter changes more explicitly and fix caching ([#1556](https://github.com/hrzlgnm/mdns-browser/pull/1556))

- *(ci)* Enable sccache when running clippy ([#1557](https://github.com/hrzlgnm/mdns-browser/pull/1557))

- *(ci)* Rename publish workflows for better clarity ([#1558](https://github.com/hrzlgnm/mdns-browser/pull/1558))

## [0.25.4] - 2025-11-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.3...mdns-browser-v0.25.4)

### Maintenance

- *(ci)* Update bump version workflow ([#1541](https://github.com/hrzlgnm/mdns-browser/pull/1541))

- *(ci)* Update bump version workflow ([#1543](https://github.com/hrzlgnm/mdns-browser/pull/1543))

## [0.25.3] - 2025-11-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.2...mdns-browser-v0.25.3)

### Dependencies

- *(deps)* Update dependency tauri-cli to v2.9.3 ([#1522](https://github.com/hrzlgnm/mdns-browser/pull/1522))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 6574041 ([#1523](https://github.com/hrzlgnm/mdns-browser/pull/1523))

- *(deps)* Lock file maintenance ([#1528](https://github.com/hrzlgnm/mdns-browser/pull/1528))

- *(deps)* Lock file maintenance ([#1535](https://github.com/hrzlgnm/mdns-browser/pull/1535))

- *(deps)* Update rust crate mdns-sd to 0.17 ([#1537](https://github.com/hrzlgnm/mdns-browser/pull/1537))

### Maintenance

- *(ci)* Use composite actions to deduplicate workflows ([#1525](https://github.com/hrzlgnm/mdns-browser/pull/1525))

- *(ci)* Run in bash shell ([#1530](https://github.com/hrzlgnm/mdns-browser/pull/1530))

- *(ci)* Temporarily disable weekly lockfile maintenance ([#1531](https://github.com/hrzlgnm/mdns-browser/pull/1531))

- *(ci)* Remove publish only from step extract android signing key ([#1536](https://github.com/hrzlgnm/mdns-browser/pull/1536))

## [0.25.2] - 2025-11-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.1...mdns-browser-v0.25.2)

### Changed

- Create tasks for zed ([#1515](https://github.com/hrzlgnm/mdns-browser/pull/1515))

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.39.0 ([#1510](https://github.com/hrzlgnm/mdns-browser/pull/1510))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to c188270 ([#1511](https://github.com/hrzlgnm/mdns-browser/pull/1511))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 41b291e ([#1512](https://github.com/hrzlgnm/mdns-browser/pull/1512))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 59ee625 ([#1514](https://github.com/hrzlgnm/mdns-browser/pull/1514))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to ec0b505 ([#1513](https://github.com/hrzlgnm/mdns-browser/pull/1513))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to bf51a3b ([#1516](https://github.com/hrzlgnm/mdns-browser/pull/1516))

### Maintenance

- *(ci)* Run tests in separate job ([#1517](https://github.com/hrzlgnm/mdns-browser/pull/1517))

- *(ci)* Add comment to schedule ([#1518](https://github.com/hrzlgnm/mdns-browser/pull/1518))

- *(ci)* Run tests on push ([#1519](https://github.com/hrzlgnm/mdns-browser/pull/1519))

- *(ci)* Add building and publishing of un-bundled executables ([#1521](https://github.com/hrzlgnm/mdns-browser/pull/1521))

## [0.25.1] - 2025-10-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.0...mdns-browser-v0.25.1)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-ubuntu-builder:v1 docker digest to 06bb8f7 ([#1508](https://github.com/hrzlgnm/mdns-browser/pull/1508))

### Maintenance

- *(ci)* Add unzip to ubuntu builder dependencies ([#1507](https://github.com/hrzlgnm/mdns-browser/pull/1507))

## [0.25.0] - 2025-10-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.10...mdns-browser-v0.25.0)

### Changed

- *(aur)* Fixup artifact names in AUR update workflow ([#1475](https://github.com/hrzlgnm/mdns-browser/pull/1475))

- *(coderabbit)* Disable docstring check ([#1481](https://github.com/hrzlgnm/mdns-browser/pull/1481))

- *(ubuntu-builder)* Add missing dependencies ([#1487](https://github.com/hrzlgnm/mdns-browser/pull/1487))

- *(ocd)* Reorder matrix definitions and update job names ([#1502](https://github.com/hrzlgnm/mdns-browser/pull/1502))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 893e7ef ([#1477](https://github.com/hrzlgnm/mdns-browser/pull/1477))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to ba36980 ([#1478](https://github.com/hrzlgnm/mdns-browser/pull/1478))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to a6886bd ([#1479](https://github.com/hrzlgnm/mdns-browser/pull/1479))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.3.1 ([#1482](https://github.com/hrzlgnm/mdns-browser/pull/1482))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.1 ([#1483](https://github.com/hrzlgnm/mdns-browser/pull/1483))

- *(deps)* Pin ubuntu docker tag to 66460d5 ([#1485](https://github.com/hrzlgnm/mdns-browser/pull/1485))

- *(deps)* Lock file maintenance ([#1493](https://github.com/hrzlgnm/mdns-browser/pull/1493))

- *(deps)* Update rust crate tauri-plugin-log to v2.7.1 ([#1495](https://github.com/hrzlgnm/mdns-browser/pull/1495))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.3.2 ([#1494](https://github.com/hrzlgnm/mdns-browser/pull/1494))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.2 ([#1496](https://github.com/hrzlgnm/mdns-browser/pull/1496))

- *(deps)* Update rust crate leptos to v0.8.12 ([#1499](https://github.com/hrzlgnm/mdns-browser/pull/1499))

- *(deps)* Update tauri monorepo to v2.9.2 ([#1504](https://github.com/hrzlgnm/mdns-browser/pull/1504))

- *(deps)* Update rust crate clap to v4.5.51 ([#1503](https://github.com/hrzlgnm/mdns-browser/pull/1503))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 2dd8706 ([#1505](https://github.com/hrzlgnm/mdns-browser/pull/1505))

- *(deps)* Update rust crate mdns-sd to 0.16 ([#1506](https://github.com/hrzlgnm/mdns-browser/pull/1506))

### Maintenance

- *(ci)* Remove linting in publish workflow ([#1473](https://github.com/hrzlgnm/mdns-browser/pull/1473))

- *(ci)* Winget: Fix url generation after artifact name change ([#1474](https://github.com/hrzlgnm/mdns-browser/pull/1474))

- *(ci)* Update workflows from ubuntu-24.04 to use ubuntu-latest ([#1480](https://github.com/hrzlgnm/mdns-browser/pull/1480))

- *(ci)* Add ubuntu builder docker image ([#1484](https://github.com/hrzlgnm/mdns-browser/pull/1484))

- *(ci)* Use ubuntu builder docker image for lint steps ([#1486](https://github.com/hrzlgnm/mdns-browser/pull/1486))

- *(ci)* Also build docker images in PR on updates ([#1488](https://github.com/hrzlgnm/mdns-browser/pull/1488))

- *(ci)* Also use ubuntu builder in desktop build workflow ([#1489](https://github.com/hrzlgnm/mdns-browser/pull/1489))

- *(ci)* Run clippy with multiple targets ([#1490](https://github.com/hrzlgnm/mdns-browser/pull/1490))

- *(ci)* Run clippy with multiple targets ([#1498](https://github.com/hrzlgnm/mdns-browser/pull/1498))

- *(ci)* Consolidate rust caches ([#1501](https://github.com/hrzlgnm/mdns-browser/pull/1501))

## [0.24.10] - 2025-10-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.9...mdns-browser-v0.24.10)

### Maintenance

- *(ci)* Tweak bump version workflow ([#1472](https://github.com/hrzlgnm/mdns-browser/pull/1472))

## [0.24.9] - 2025-10-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.8...mdns-browser-v0.24.9)

### Dependencies

- *(deps)* Update dependency cargo-auditable to v0.7.1 ([#1455](https://github.com/hrzlgnm/mdns-browser/pull/1455))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 5c4d66b ([#1456](https://github.com/hrzlgnm/mdns-browser/pull/1456))

- *(deps)* Lock file maintenance ([#1457](https://github.com/hrzlgnm/mdns-browser/pull/1457))

- *(deps)* Update rust crate clap to v4.5.50 ([#1458](https://github.com/hrzlgnm/mdns-browser/pull/1458))

- *(deps)* Update archlinux:base-devel docker digest to 943bdad ([#1460](https://github.com/hrzlgnm/mdns-browser/pull/1460))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d96ab51 ([#1461](https://github.com/hrzlgnm/mdns-browser/pull/1461))

- *(deps)* Update rust crate serde_with to v3.15.1 ([#1462](https://github.com/hrzlgnm/mdns-browser/pull/1462))

- *(deps)* Update tauri monorepo ([#1459](https://github.com/hrzlgnm/mdns-browser/pull/1459))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 06475a2 ([#1463](https://github.com/hrzlgnm/mdns-browser/pull/1463))

- *(deps)* Update anchore/sbom-action digest to 8e94d75 ([#1464](https://github.com/hrzlgnm/mdns-browser/pull/1464))

- *(deps)* Update rust crate leptos to v0.8.11 ([#1465](https://github.com/hrzlgnm/mdns-browser/pull/1465))

- *(deps)* Update github artifact actions (major) ([#1467](https://github.com/hrzlgnm/mdns-browser/pull/1467))

- *(deps)* Lock file maintenance ([#1468](https://github.com/hrzlgnm/mdns-browser/pull/1468))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.24 ([#1469](https://github.com/hrzlgnm/mdns-browser/pull/1469))

### Maintenance

- *(ci)* Lint in own separate job ([#1470](https://github.com/hrzlgnm/mdns-browser/pull/1470))

## [0.24.8] - 2025-10-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.7...mdns-browser-v0.24.8)

### Maintenance

- *(ci)* Compress windows debug symbols ([#1453](https://github.com/hrzlgnm/mdns-browser/pull/1453))

- *(ci)* Don't publish unused sha512 checksums ([#1454](https://github.com/hrzlgnm/mdns-browser/pull/1454))

## [0.24.7] - 2025-10-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.6...mdns-browser-v0.24.7)

### Changed

- Simplify `nvidia` or `nouveau` detection ([#1452](https://github.com/hrzlgnm/mdns-browser/pull/1452))

## [0.24.6] - 2025-10-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.5...mdns-browser-v0.24.6)

### Changed

- *(aur)* Add cargo tests and uncomment CFLAGS export ([#1449](https://github.com/hrzlgnm/mdns-browser/pull/1449))

- *(aur)* Disable fail-fast option ([#1450](https://github.com/hrzlgnm/mdns-browser/pull/1450))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 950538e ([#1447](https://github.com/hrzlgnm/mdns-browser/pull/1447))

### Fixed

- Enable the workaround for nvidia also when running in wayland ([#1451](https://github.com/hrzlgnm/mdns-browser/pull/1451))

## [0.24.5] - 2025-10-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.4...mdns-browser-v0.24.5)

### Added

- *(aur)* Use packages instead of cargo-install for most dependencies except tauri-cli ([#1445](https://github.com/hrzlgnm/mdns-browser/pull/1445))

### Dependencies

- *(deps)* Update rust crate regex to v1.12.1 ([#1436](https://github.com/hrzlgnm/mdns-browser/pull/1436))

- *(deps)* Update softprops/action-gh-release digest to 6da8fa9 ([#1437](https://github.com/hrzlgnm/mdns-browser/pull/1437))

- *(deps)* Lock file maintenance ([#1438](https://github.com/hrzlgnm/mdns-browser/pull/1438))

- *(deps)* Update rust crate regex to v1.12.2 ([#1440](https://github.com/hrzlgnm/mdns-browser/pull/1440))

- *(deps)* Update rust crate clap to v4.5.49 ([#1439](https://github.com/hrzlgnm/mdns-browser/pull/1439))

- *(deps)* Update archlinux:base-devel docker digest to 87a967f ([#1441](https://github.com/hrzlgnm/mdns-browser/pull/1441))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to dae2f6b ([#1442](https://github.com/hrzlgnm/mdns-browser/pull/1442))

- *(deps)* Update rust crate tokio to v1.48.0 ([#1443](https://github.com/hrzlgnm/mdns-browser/pull/1443))

- *(deps)* Update anchore/sbom-action digest to d8a2c01 ([#1444](https://github.com/hrzlgnm/mdns-browser/pull/1444))

- *(deps)* Update anchore/sbom-action digest to aa0e114 ([#1446](https://github.com/hrzlgnm/mdns-browser/pull/1446))

## [0.24.4] - 2025-10-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.3...mdns-browser-v0.24.4)

### Fixed

- *(void-packaging)* Update webkit2gtk dependencies ([#1434](https://github.com/hrzlgnm/mdns-browser/pull/1434))

## [0.24.3] - 2025-10-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.2...mdns-browser-v0.24.3)

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.36.3 ([#1411](https://github.com/hrzlgnm/mdns-browser/pull/1411))

- *(deps)* Update archlinux:base-devel docker digest to 5d95edc ([#1412](https://github.com/hrzlgnm/mdns-browser/pull/1412))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4aa2b44 ([#1413](https://github.com/hrzlgnm/mdns-browser/pull/1413))

- *(deps)* Update crate-ci/typos action to v1.37.0 ([#1414](https://github.com/hrzlgnm/mdns-browser/pull/1414))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d39c60f ([#1415](https://github.com/hrzlgnm/mdns-browser/pull/1415))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 0c79c04 ([#1416](https://github.com/hrzlgnm/mdns-browser/pull/1416))

- *(deps)* Update crate-ci/typos action to v1.37.1 ([#1418](https://github.com/hrzlgnm/mdns-browser/pull/1418))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to e85211c ([#1417](https://github.com/hrzlgnm/mdns-browser/pull/1417))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 5199629 ([#1419](https://github.com/hrzlgnm/mdns-browser/pull/1419))

- *(deps)* Update crate-ci/typos action to v1.37.2 ([#1420](https://github.com/hrzlgnm/mdns-browser/pull/1420))

- *(deps)* Update archlinux:base-devel docker digest to b380991 ([#1421](https://github.com/hrzlgnm/mdns-browser/pull/1421))

- *(deps)* Update crate-ci/typos action to v1.38.0 ([#1422](https://github.com/hrzlgnm/mdns-browser/pull/1422))

- *(deps)* Update softprops/action-gh-release digest to aec2ec5 ([#1424](https://github.com/hrzlgnm/mdns-browser/pull/1424))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 6078a61 ([#1423](https://github.com/hrzlgnm/mdns-browser/pull/1423))

- *(deps)* Update crate-ci/typos action to v1.38.1 ([#1426](https://github.com/hrzlgnm/mdns-browser/pull/1426))

- *(deps)* Update archlinux:base-devel docker digest to 06ab929 ([#1427](https://github.com/hrzlgnm/mdns-browser/pull/1427))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 449af96 ([#1428](https://github.com/hrzlgnm/mdns-browser/pull/1428))

- *(deps)* Update rust crate regex to v1.11.3 ([#1430](https://github.com/hrzlgnm/mdns-browser/pull/1430))

- *(deps)* Update rust crate leptos to v0.8.10 ([#1429](https://github.com/hrzlgnm/mdns-browser/pull/1429))

- *(deps)* Update rust crate serde_with to v3.15.0 ([#1433](https://github.com/hrzlgnm/mdns-browser/pull/1433))

- *(deps)* Update rust crate thiserror to v2.0.17 ([#1432](https://github.com/hrzlgnm/mdns-browser/pull/1432))

- *(deps)* Update rust crate serde to v1.0.228 ([#1431](https://github.com/hrzlgnm/mdns-browser/pull/1431))

## [0.24.2] - 2025-09-23 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.1...mdns-browser-v0.24.2)

### Fixed

- Use Memo for dead state with try_get default `true` ([#1410](https://github.com/hrzlgnm/mdns-browser/pull/1410))

## [0.24.1] - 2025-09-23 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.0...mdns-browser-v0.24.1)

### Dependencies

- *(deps)* Update rust crate serde_json to v1.0.144 ([#1394](https://github.com/hrzlgnm/mdns-browser/pull/1394))

- *(deps)* Update rust crate serde_json to v1.0.145 ([#1395](https://github.com/hrzlgnm/mdns-browser/pull/1395))

- *(deps)* Update rust crate serde to v1.0.223 ([#1393](https://github.com/hrzlgnm/mdns-browser/pull/1393))

- *(deps)* Lock file maintenance ([#1396](https://github.com/hrzlgnm/mdns-browser/pull/1396))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.0 ([#1397](https://github.com/hrzlgnm/mdns-browser/pull/1397))

- *(deps)* Update anchore/sbom-action digest to f8bdd1d ([#1398](https://github.com/hrzlgnm/mdns-browser/pull/1398))

- *(deps)* Update archlinux:base-devel docker digest to 9019fd8 ([#1399](https://github.com/hrzlgnm/mdns-browser/pull/1399))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d5c6d20 ([#1400](https://github.com/hrzlgnm/mdns-browser/pull/1400))

- *(deps)* Update rust crate serde to v1.0.225 ([#1401](https://github.com/hrzlgnm/mdns-browser/pull/1401))

- *(deps)* Update rust crate leptos to v0.8.9 ([#1402](https://github.com/hrzlgnm/mdns-browser/pull/1402))

- *(deps)* Update rust crate clap to v4.5.48 ([#1403](https://github.com/hrzlgnm/mdns-browser/pull/1403))

- *(deps)* Update rust crate serde_with to v3.14.1 ([#1404](https://github.com/hrzlgnm/mdns-browser/pull/1404))

- *(deps)* Update rust crate serde to v1.0.226 ([#1405](https://github.com/hrzlgnm/mdns-browser/pull/1405))

- *(deps)* Lock file maintenance ([#1406](https://github.com/hrzlgnm/mdns-browser/pull/1406))

- *(deps)* Update archlinux:base-devel docker digest to 0589aa8 ([#1407](https://github.com/hrzlgnm/mdns-browser/pull/1407))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 5d1f45d ([#1408](https://github.com/hrzlgnm/mdns-browser/pull/1408))

## [0.24.0] - 2025-09-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.23.1...mdns-browser-v0.24.0)

### Added

- Increase mDNS-Browser window width to 1615 ([#1392](https://github.com/hrzlgnm/mdns-browser/pull/1392))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 52caef7 ([#1390](https://github.com/hrzlgnm/mdns-browser/pull/1390))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 393a4f4 ([#1391](https://github.com/hrzlgnm/mdns-browser/pull/1391))

## [0.23.1] - 2025-09-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.23.0...mdns-browser-v0.23.1)

### Dependencies

- *(deps)* Update rust crate tauri-plugin-log to v2.7.0 ([#1379](https://github.com/hrzlgnm/mdns-browser/pull/1379))

- *(deps)* Update crate-ci/typos action to v1.36.0 ([#1378](https://github.com/hrzlgnm/mdns-browser/pull/1378))

- *(deps)* Update rust crate clap to v4.5.47 ([#1380](https://github.com/hrzlgnm/mdns-browser/pull/1380))

- *(deps)* Update crate-ci/typos action to v1.36.1 ([#1381](https://github.com/hrzlgnm/mdns-browser/pull/1381))

- *(deps)* Update rust crate log to v0.4.28 ([#1382](https://github.com/hrzlgnm/mdns-browser/pull/1382))

- *(deps)* Update actions/github-script action to v8 ([#1384](https://github.com/hrzlgnm/mdns-browser/pull/1384))

- *(deps)* Update crate-ci/typos action to v1.36.2 ([#1383](https://github.com/hrzlgnm/mdns-browser/pull/1383))

- *(deps)* Update rust crate mdns-sd to v0.15.1 ([#1385](https://github.com/hrzlgnm/mdns-browser/pull/1385))

- *(deps)* Update softprops/action-gh-release digest to 6cbd405 ([#1386](https://github.com/hrzlgnm/mdns-browser/pull/1386))

- *(deps)* Lock file maintenance ([#1387](https://github.com/hrzlgnm/mdns-browser/pull/1387))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.0 ([#1388](https://github.com/hrzlgnm/mdns-browser/pull/1388))

- *(deps)* Update rust crate chrono to v0.4.42 ([#1389](https://github.com/hrzlgnm/mdns-browser/pull/1389))

## [0.23.0] - 2025-09-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.3...mdns-browser-v0.23.0)

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.35.8 ([#1376](https://github.com/hrzlgnm/mdns-browser/pull/1376))

### Fixed

- Ensure details dialog close button is visible with long titles ([#1377](https://github.com/hrzlgnm/mdns-browser/pull/1377))

## [0.22.3] - 2025-09-01 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.2...mdns-browser-v0.22.3)

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 925a18f ([#1339](https://github.com/hrzlgnm/mdns-browser/pull/1339))

- *(deps)* Update tauri monorepo ([#1340](https://github.com/hrzlgnm/mdns-browser/pull/1340))

- *(deps)* Update crate-ci/typos action to v1.35.5 ([#1341](https://github.com/hrzlgnm/mdns-browser/pull/1341))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 05267a7 ([#1342](https://github.com/hrzlgnm/mdns-browser/pull/1342))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 3648ac5 ([#1343](https://github.com/hrzlgnm/mdns-browser/pull/1343))

- *(deps)* Update rust crate serde_json to v1.0.143 ([#1344](https://github.com/hrzlgnm/mdns-browser/pull/1344))

- *(deps)* Update rust crate tauri to v2.8.2 ([#1345](https://github.com/hrzlgnm/mdns-browser/pull/1345))

- *(deps)* Update rust crate thiserror to v2.0.16 ([#1346](https://github.com/hrzlgnm/mdns-browser/pull/1346))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.0 ([#1347](https://github.com/hrzlgnm/mdns-browser/pull/1347))

- *(deps)* Update rust crate mdns-sd to v0.14.1 ([#1348](https://github.com/hrzlgnm/mdns-browser/pull/1348))

- *(deps)* Update actions/setup-java action to v5 ([#1349](https://github.com/hrzlgnm/mdns-browser/pull/1349))

- *(deps)* Update baptiste0928/cargo-install digest to b687c65 ([#1350](https://github.com/hrzlgnm/mdns-browser/pull/1350))

- *(deps)* Update rust crate regex to v1.11.2 ([#1351](https://github.com/hrzlgnm/mdns-browser/pull/1351))

- *(deps)* Update tauri monorepo ([#1352](https://github.com/hrzlgnm/mdns-browser/pull/1352))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 6829a82 ([#1353](https://github.com/hrzlgnm/mdns-browser/pull/1353))

- *(deps)* Lock file maintenance ([#1354](https://github.com/hrzlgnm/mdns-browser/pull/1354))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.0 ([#1355](https://github.com/hrzlgnm/mdns-browser/pull/1355))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.23 ([#1356](https://github.com/hrzlgnm/mdns-browser/pull/1356))

- *(deps)* Update tauri monorepo ([#1357](https://github.com/hrzlgnm/mdns-browser/pull/1357))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to c9eb74f ([#1358](https://github.com/hrzlgnm/mdns-browser/pull/1358))

- *(deps)* Update rust crate leptos to v0.8.7 ([#1360](https://github.com/hrzlgnm/mdns-browser/pull/1360))

- *(deps)* Update archlinux:base-devel docker digest to 8ccc930 ([#1359](https://github.com/hrzlgnm/mdns-browser/pull/1359))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to b626009 ([#1361](https://github.com/hrzlgnm/mdns-browser/pull/1361))

- *(deps)* Update rust crate clap to v4.5.46 ([#1362](https://github.com/hrzlgnm/mdns-browser/pull/1362))

- *(deps)* Update rust crate leptos to v0.8.8 ([#1363](https://github.com/hrzlgnm/mdns-browser/pull/1363))

- *(deps)* Update crate-ci/typos action to v1.35.6 ([#1364](https://github.com/hrzlgnm/mdns-browser/pull/1364))

- *(deps)* Update actions/attest-build-provenance action to v3 ([#1365](https://github.com/hrzlgnm/mdns-browser/pull/1365))

- *(deps)* Update actions/attest-sbom action to v3 ([#1366](https://github.com/hrzlgnm/mdns-browser/pull/1366))

- *(deps)* Update crate-ci/typos action to v1.35.7 ([#1367](https://github.com/hrzlgnm/mdns-browser/pull/1367))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 01a1ace ([#1369](https://github.com/hrzlgnm/mdns-browser/pull/1369))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to dac640d ([#1368](https://github.com/hrzlgnm/mdns-browser/pull/1368))

- *(deps)* Update rust crate mdns-sd to 0.15 ([#1370](https://github.com/hrzlgnm/mdns-browser/pull/1370))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 801b119 ([#1371](https://github.com/hrzlgnm/mdns-browser/pull/1371))

- *(deps)* Update tauri monorepo ([#1372](https://github.com/hrzlgnm/mdns-browser/pull/1372))

- *(deps)* Lock file maintenance ([#1373](https://github.com/hrzlgnm/mdns-browser/pull/1373))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to a3625dd ([#1374](https://github.com/hrzlgnm/mdns-browser/pull/1374))

- *(deps)* Update rust crate tauri-plugin-opener to v2.5.0 ([#1375](https://github.com/hrzlgnm/mdns-browser/pull/1375))

## [0.22.2] - 2025-08-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.1...mdns-browser-v0.22.2)

### Added

- Restructure details dialog: split header, add close button ([#1337](https://github.com/hrzlgnm/mdns-browser/pull/1337))

### Dependencies

- *(deps)* Update rust crate thiserror to v2.0.15 ([#1334](https://github.com/hrzlgnm/mdns-browser/pull/1334))

- *(deps)* Lock file maintenance ([#1335](https://github.com/hrzlgnm/mdns-browser/pull/1335))

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1336](https://github.com/hrzlgnm/mdns-browser/pull/1336))

## [0.22.1] - 2025-08-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.0...mdns-browser-v0.22.1)

### Added

- Refactor BackTop scroll handling to be smooth and passive ([#1332](https://github.com/hrzlgnm/mdns-browser/pull/1332))

### Dependencies

- *(deps)* Update anchore/sbom-action digest to da167ea ([#1331](https://github.com/hrzlgnm/mdns-browser/pull/1331))

## [0.22.0] - 2025-08-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.10...mdns-browser-v0.22.0)

### Added

- Add BackTop  component, wire into app, add CSS ([#1327](https://github.com/hrzlgnm/mdns-browser/pull/1327))

## [0.21.10] - 2025-08-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.9...mdns-browser-v0.21.10)

### Added

- Enumerate_mdns_incapable_interfaces: skip loopback on Linux/Windows ([#1323](https://github.com/hrzlgnm/mdns-browser/pull/1323))

### Changed

- Add responsive viewport meta tag to index.html ([#1326](https://github.com/hrzlgnm/mdns-browser/pull/1326))

### Dependencies

- *(deps)* Update rust crate thiserror to v2.0.14 ([#1322](https://github.com/hrzlgnm/mdns-browser/pull/1322))

- *(deps)* Update crate-ci/typos action to v1.35.4 ([#1324](https://github.com/hrzlgnm/mdns-browser/pull/1324))

- *(deps)* Update rust crate clap to v4.5.45 ([#1325](https://github.com/hrzlgnm/mdns-browser/pull/1325))

## [0.21.9] - 2025-08-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.8...mdns-browser-v0.21.9)

### Changed

- Switch mdns-sd to crates.io 0.14 in src-tauri/Cargo.toml ([#1310](https://github.com/hrzlgnm/mdns-browser/pull/1310))

### Dependencies

- *(deps)* Lock file maintenance ([#1309](https://github.com/hrzlgnm/mdns-browser/pull/1309))

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1311](https://github.com/hrzlgnm/mdns-browser/pull/1311))

- *(deps)* Update actions/checkout digest to 08eba0b ([#1312](https://github.com/hrzlgnm/mdns-browser/pull/1312))

- *(deps)* Update actions/checkout action to v5 ([#1314](https://github.com/hrzlgnm/mdns-browser/pull/1314))

- *(deps)* Update archlinux:base-devel docker digest to 92a0740 ([#1317](https://github.com/hrzlgnm/mdns-browser/pull/1317))

- *(deps)* Update rust crate clap to v4.5.44 ([#1318](https://github.com/hrzlgnm/mdns-browser/pull/1318))

- *(deps)* Update rust crate thiserror to v2.0.13 ([#1320](https://github.com/hrzlgnm/mdns-browser/pull/1320))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 3e11dc3 ([#1321](https://github.com/hrzlgnm/mdns-browser/pull/1321))

### Fixed

- Tweak layout on mobile so top controls are usable again ([#1315](https://github.com/hrzlgnm/mdns-browser/pull/1315))

## [0.21.8] - 2025-08-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.7...mdns-browser-v0.21.8)

### Changed

- Update mdns_sd API usage ([#1308](https://github.com/hrzlgnm/mdns-browser/pull/1308))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.42 ([#1291](https://github.com/hrzlgnm/mdns-browser/pull/1291))

- *(deps)* Update rust crate serde_json to v1.0.142 ([#1293](https://github.com/hrzlgnm/mdns-browser/pull/1293))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 2b8e6a2 ([#1295](https://github.com/hrzlgnm/mdns-browser/pull/1295))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4995b89 ([#1294](https://github.com/hrzlgnm/mdns-browser/pull/1294))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 4553217 ([#1296](https://github.com/hrzlgnm/mdns-browser/pull/1296))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to b2b2c47 ([#1297](https://github.com/hrzlgnm/mdns-browser/pull/1297))

- *(deps)* Update rust crate tokio to v1.47.1 ([#1298](https://github.com/hrzlgnm/mdns-browser/pull/1298))

- *(deps)* Lock file maintenance ([#1299](https://github.com/hrzlgnm/mdns-browser/pull/1299))

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1300](https://github.com/hrzlgnm/mdns-browser/pull/1300))

- *(deps)* Update crate-ci/typos action to v1.35.1 ([#1301](https://github.com/hrzlgnm/mdns-browser/pull/1301))

- *(deps)* Update archlinux:base-devel docker digest to 15d3106 ([#1302](https://github.com/hrzlgnm/mdns-browser/pull/1302))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to d10a4e7 ([#1303](https://github.com/hrzlgnm/mdns-browser/pull/1303))

- *(deps)* Update actions/download-artifact action to v5 ([#1304](https://github.com/hrzlgnm/mdns-browser/pull/1304))

- *(deps)* Update rust crate clap to v4.5.43 ([#1305](https://github.com/hrzlgnm/mdns-browser/pull/1305))

- *(deps)* Update crate-ci/typos action to v1.35.2 ([#1306](https://github.com/hrzlgnm/mdns-browser/pull/1306))

- *(deps)* Update crate-ci/typos action to v1.35.3 ([#1307](https://github.com/hrzlgnm/mdns-browser/pull/1307))

## [0.21.7] - 2025-07-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.6...mdns-browser-v0.21.7)

### Dependencies

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1289](https://github.com/hrzlgnm/mdns-browser/pull/1289))

## [0.21.6] - 2025-07-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.5...mdns-browser-v0.21.6)

### Dependencies

- *(deps)* Update rust crate tokio to v1.47.0 ([#1280](https://github.com/hrzlgnm/mdns-browser/pull/1280))

- *(deps)* Update rust crate leptos to v0.8.6 ([#1281](https://github.com/hrzlgnm/mdns-browser/pull/1281))

- *(deps)* Update hugo19941994/delete-draft-releases action to v2 ([#1282](https://github.com/hrzlgnm/mdns-browser/pull/1282))

- *(deps)* Lock file maintenance ([#1283](https://github.com/hrzlgnm/mdns-browser/pull/1283))

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1284](https://github.com/hrzlgnm/mdns-browser/pull/1284))

- *(deps)* Update archlinux:base-devel docker digest to af82cf8 ([#1285](https://github.com/hrzlgnm/mdns-browser/pull/1285))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 80b189b ([#1287](https://github.com/hrzlgnm/mdns-browser/pull/1287))

- *(deps)* Lock file maintenance ([#1288](https://github.com/hrzlgnm/mdns-browser/pull/1288))

### Maintenance

- *(ci)* Add paths-ignore filters to Android and Desktop GitHub Actions workflows ([#1279](https://github.com/hrzlgnm/mdns-browser/pull/1279))

## [0.21.5] - 2025-07-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.4...mdns-browser-v0.21.5)

### Dependencies

- *(deps)* Update anchore/sbom-action digest to 7b36ad6 ([#1272](https://github.com/hrzlgnm/mdns-browser/pull/1272))

- *(deps)* Update archlinux:base-devel docker digest to 210f84b ([#1273](https://github.com/hrzlgnm/mdns-browser/pull/1273))

- *(deps)* Update dependency tauri-cli to v2.7.1 ([#1274](https://github.com/hrzlgnm/mdns-browser/pull/1274))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 3efd7c7 ([#1275](https://github.com/hrzlgnm/mdns-browser/pull/1275))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 4664bbd ([#1276](https://github.com/hrzlgnm/mdns-browser/pull/1276))

- *(deps)* Lock file maintenance ([#1277](https://github.com/hrzlgnm/mdns-browser/pull/1277))

## [0.21.4] - 2025-07-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.3...mdns-browser-v0.21.4)

### Changed

- Fix typo in Cargo.toml by changing resolve to resolver in workspace section ([#1271](https://github.com/hrzlgnm/mdns-browser/pull/1271))

## [0.21.3] - 2025-07-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.2...mdns-browser-v0.21.3)

### Changed

- Set resolve version to 2 in Cargo.toml workspace section ([#1270](https://github.com/hrzlgnm/mdns-browser/pull/1270))

## [0.21.2] - 2025-07-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.1...mdns-browser-v0.21.2)

### Dependencies

- *(deps)* Update tauri monorepo ([#1266](https://github.com/hrzlgnm/mdns-browser/pull/1266))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to e76eee3 ([#1267](https://github.com/hrzlgnm/mdns-browser/pull/1267))

- *(deps)* Update rust crate reactive_stores to v0.2.5 ([#1268](https://github.com/hrzlgnm/mdns-browser/pull/1268))

- *(deps)* Update rust crate leptos to v0.8.5 ([#1269](https://github.com/hrzlgnm/mdns-browser/pull/1269))

## [0.21.1] - 2025-07-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.0...mdns-browser-v0.21.1)

### Dependencies

- *(deps)* Update rust crate reactive_stores to v0.2.4 ([#1265](https://github.com/hrzlgnm/mdns-browser/pull/1265))

- *(deps)* Update rust crate leptos to v0.8.4 ([#1247](https://github.com/hrzlgnm/mdns-browser/pull/1247))

## [0.21.0] - 2025-07-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.4...mdns-browser-v0.21.0)

### Added

- Add sorting by port, and IP to service browsing UI and logic ([#1264](https://github.com/hrzlgnm/mdns-browser/pull/1264))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to ae06553 ([#1259](https://github.com/hrzlgnm/mdns-browser/pull/1259))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 7570720 ([#1260](https://github.com/hrzlgnm/mdns-browser/pull/1260))

- *(deps)* Update rust crate serde_json to v1.0.141 ([#1263](https://github.com/hrzlgnm/mdns-browser/pull/1263))

## [0.20.4] - 2025-07-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.3...mdns-browser-v0.20.4)

### Added

- Disable mDNS incapable interfaces on `ServiceDaemon` ([#1258](https://github.com/hrzlgnm/mdns-browser/pull/1258))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 10b7661 ([#1256](https://github.com/hrzlgnm/mdns-browser/pull/1256))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to bb45a80 ([#1257](https://github.com/hrzlgnm/mdns-browser/pull/1257))

## [0.20.3] - 2025-07-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.2...mdns-browser-v0.20.3)

### Fixed

- Add deduplication of sorted addresses in from_resolved_service_detailed ([#1254](https://github.com/hrzlgnm/mdns-browser/pull/1254))

## [0.20.2] - 2025-07-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.1...mdns-browser-v0.20.2)

### Added

- Use new ServiceDetailed API from mdns_sd ([#1251](https://github.com/hrzlgnm/mdns-browser/pull/1251))

- Add ScopedAddr type and update ResolvedService usage ([#1252](https://github.com/hrzlgnm/mdns-browser/pull/1252))

## [0.20.1] - 2025-07-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.0...mdns-browser-v0.20.1)

### Changed

- Add explicit conflict declarations between mdns-browser and mdns-browser-bin ([#1250](https://github.com/hrzlgnm/mdns-browser/pull/1250))

## [0.20.0] - 2025-07-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.7...mdns-browser-v0.20.0)

### Changed

- Update Renovate config to use automerge preset extends and remove explicit settings ([#1187](https://github.com/hrzlgnm/mdns-browser/pull/1187))

- Reorder extends array entries in .github/renovate.json5 configuration file ([#1189](https://github.com/hrzlgnm/mdns-browser/pull/1189))

- Remove header comment line about global drop event handling in index.js ([#1191](https://github.com/hrzlgnm/mdns-browser/pull/1191))

- Update Renovate config for presets, automerge, concurrency, and scheduling ([#1194](https://github.com/hrzlgnm/mdns-browser/pull/1194))

- Update string formatting to use Rust inline variable interpolation syntax ([#1225](https://github.com/hrzlgnm/mdns-browser/pull/1225))

- Update mdns-sd dependency to use Git repository ([#1249](https://github.com/hrzlgnm/mdns-browser/pull/1249))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.40 ([#1182](https://github.com/hrzlgnm/mdns-browser/pull/1182))

- *(deps)* Update archlinux:base-devel docker digest to 5b87f50 ([#1183](https://github.com/hrzlgnm/mdns-browser/pull/1183))

- *(deps)* Update softprops/action-gh-release digest to d5382d3 ([#1184](https://github.com/hrzlgnm/mdns-browser/pull/1184))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 8410f1c ([#1185](https://github.com/hrzlgnm/mdns-browser/pull/1185))

- *(deps)* Update softprops/action-gh-release digest to 72f2c25 ([#1190](https://github.com/hrzlgnm/mdns-browser/pull/1190))

- *(deps)* Update actions/attest-sbom digest to bd218ad ([#1193](https://github.com/hrzlgnm/mdns-browser/pull/1193))

- *(deps)* Update actions/attest-build-provenance action to v2.4.0 ([#1192](https://github.com/hrzlgnm/mdns-browser/pull/1192))

- *(deps)* Update anchore/sbom-action digest to 9246b90 ([#1195](https://github.com/hrzlgnm/mdns-browser/pull/1195))

- *(deps)* Update rust crate serde_with to v3.13.0 ([#1196](https://github.com/hrzlgnm/mdns-browser/pull/1196))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to b922884 ([#1197](https://github.com/hrzlgnm/mdns-browser/pull/1197))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to aa150d5 ([#1198](https://github.com/hrzlgnm/mdns-browser/pull/1198))

- *(deps)* Update archlinux:base-devel docker digest to 3f808d4 ([#1199](https://github.com/hrzlgnm/mdns-browser/pull/1199))

- *(deps)* Update rust crate tauri-plugin-opener to v2.3.0 ([#1202](https://github.com/hrzlgnm/mdns-browser/pull/1202))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.2.3 ([#1200](https://github.com/hrzlgnm/mdns-browser/pull/1200))

- *(deps)* Update rust crate tauri-plugin-log to v2.5.0 ([#1201](https://github.com/hrzlgnm/mdns-browser/pull/1201))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 3d775cc ([#1203](https://github.com/hrzlgnm/mdns-browser/pull/1203))

- *(deps)* Update rust crate tauri-plugin-updater to v2.8.0 ([#1204](https://github.com/hrzlgnm/mdns-browser/pull/1204))

- *(deps)* Update baptiste0928/cargo-install digest to e38323e ([#1205](https://github.com/hrzlgnm/mdns-browser/pull/1205))

- *(deps)* Update rust crate tauri-plugin-updater to v2.8.1 ([#1206](https://github.com/hrzlgnm/mdns-browser/pull/1206))

- *(deps)* Update rust crate mdns-sd to v0.13.10 ([#1207](https://github.com/hrzlgnm/mdns-browser/pull/1207))

- *(deps)* Update archlinux:base-devel docker digest to bb4464b ([#1209](https://github.com/hrzlgnm/mdns-browser/pull/1209))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to dd5a368 ([#1210](https://github.com/hrzlgnm/mdns-browser/pull/1210))

- *(deps)* Update rust crate tauri-plugin-log to v2.5.1 ([#1212](https://github.com/hrzlgnm/mdns-browser/pull/1212))

- *(deps)* Update rust crate tauri-plugin-opener to v2.3.1 ([#1213](https://github.com/hrzlgnm/mdns-browser/pull/1213))

- *(deps)* Lock file maintenance ([#1214](https://github.com/hrzlgnm/mdns-browser/pull/1214))

- *(deps)* Update dependency tauri-cli to v2.6.0 ([#1211](https://github.com/hrzlgnm/mdns-browser/pull/1211))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 8163ffc ([#1216](https://github.com/hrzlgnm/mdns-browser/pull/1216))

- *(deps)* Update swatinem/rust-cache digest to 98c8021 ([#1217](https://github.com/hrzlgnm/mdns-browser/pull/1217))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.3.0 ([#1218](https://github.com/hrzlgnm/mdns-browser/pull/1218))

- *(deps)* Update rust crate tauri-plugin-opener to v2.4.0 ([#1220](https://github.com/hrzlgnm/mdns-browser/pull/1220))

- *(deps)* Update rust crate tauri-plugin-log to v2.6.0 ([#1219](https://github.com/hrzlgnm/mdns-browser/pull/1219))

- *(deps)* Update rust crate tauri-plugin-updater to v2.9.0 ([#1221](https://github.com/hrzlgnm/mdns-browser/pull/1221))

- *(deps)* Update tauri monorepo to v2.6.1 ([#1222](https://github.com/hrzlgnm/mdns-browser/pull/1222))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 1c62e89 ([#1223](https://github.com/hrzlgnm/mdns-browser/pull/1223))

- *(deps)* Update rust crate tauri to v2.6.2 ([#1224](https://github.com/hrzlgnm/mdns-browser/pull/1224))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.21 10 ([#1226](https://github.com/hrzlgnm/mdns-browser/pull/1226))

- *(deps)* Update dependency tauri-cli to v2.6.2 ([#1227](https://github.com/hrzlgnm/mdns-browser/pull/1227))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.22 ([#1229](https://github.com/hrzlgnm/mdns-browser/pull/1229))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to f3c97d4 ([#1228](https://github.com/hrzlgnm/mdns-browser/pull/1228))

- *(deps)* Update crate-ci/typos action to v1.34.0 ([#1230](https://github.com/hrzlgnm/mdns-browser/pull/1230))

- *(deps)* Update rust crate serde_with to v3.14.0 ([#1231](https://github.com/hrzlgnm/mdns-browser/pull/1231))

- *(deps)* Update archlinux:base-devel docker digest to 16c85e5 ([#1232](https://github.com/hrzlgnm/mdns-browser/pull/1232))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to d26bbe1 ([#1234](https://github.com/hrzlgnm/mdns-browser/pull/1234))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to ad47064 ([#1233](https://github.com/hrzlgnm/mdns-browser/pull/1233))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 3f1babb ([#1235](https://github.com/hrzlgnm/mdns-browser/pull/1235))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 5c8cbc3 ([#1236](https://github.com/hrzlgnm/mdns-browser/pull/1236))

- *(deps)* Update rust crate tokio to v1.46.0 ([#1237](https://github.com/hrzlgnm/mdns-browser/pull/1237))

- *(deps)* Update anchore/sbom-action digest to cee1b8e ([#1238](https://github.com/hrzlgnm/mdns-browser/pull/1238))

- *(deps)* Update dependency cargo-auditable to v0.7.0 ([#1239](https://github.com/hrzlgnm/mdns-browser/pull/1239))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to e08d9b9 ([#1240](https://github.com/hrzlgnm/mdns-browser/pull/1240))

- *(deps)* Update rust crate tokio to v1.46.1 ([#1241](https://github.com/hrzlgnm/mdns-browser/pull/1241))

- *(deps)* Update archlinux:base-devel docker digest to 7beca11 ([#1242](https://github.com/hrzlgnm/mdns-browser/pull/1242))

- *(deps)* Update rust crate mdns-sd to v0.13.11 ([#1243](https://github.com/hrzlgnm/mdns-browser/pull/1243))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4584074 ([#1244](https://github.com/hrzlgnm/mdns-browser/pull/1244))

- *(deps)* Update rust crate clap to v4.5.41 ([#1245](https://github.com/hrzlgnm/mdns-browser/pull/1245))

- *(deps)* Update rust crate reactive_stores to v0.2.3 ([#1246](https://github.com/hrzlgnm/mdns-browser/pull/1246))

- *(deps)* Lock file maintenance ([#1248](https://github.com/hrzlgnm/mdns-browser/pull/1248))

## [0.19.7] - 2025-06-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.6...mdns-browser-v0.19.7)

### Fixed

- Refactor drag-and-drop event handling to selectively block non-editable targets ([#1181](https://github.com/hrzlgnm/mdns-browser/pull/1181))

## [0.19.6] - 2025-06-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.5...mdns-browser-v0.19.6)

### Added

- Disable drag-and-drop interactions to prevent uninended navigation ([#1180](https://github.com/hrzlgnm/mdns-browser/pull/1180))

## [0.19.5] - 2025-06-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.4...mdns-browser-v0.19.5)

### Added

- Indicate interactivity on `ThemeSwitcher` icon ([#1179](https://github.com/hrzlgnm/mdns-browser/pull/1179))

### Changed

- Use SCCACHE_PATH env var to set RUSTC_WRAPPER ([#1173](https://github.com/hrzlgnm/mdns-browser/pull/1173))

- Update workflow to exclude sccache on Windows ([#1174](https://github.com/hrzlgnm/mdns-browser/pull/1174))

- Switch thaw dependencies to Git and update checkbox disabling logic in UI ([#1175](https://github.com/hrzlgnm/mdns-browser/pull/1175))

- Update GitHub Actions workflow to restrict specific steps to Ubuntu runners ([#1176](https://github.com/hrzlgnm/mdns-browser/pull/1176))

- Optimize CI workflows and restrict tests to models package ([#1177](https://github.com/hrzlgnm/mdns-browser/pull/1177))

### Dependencies

- *(deps)* Pin mozilla-actions/sccache-action action to 7d986dd ([#1172](https://github.com/hrzlgnm/mdns-browser/pull/1172))

- *(deps)* Update rust crate icondata to 0.6 ([#1178](https://github.com/hrzlgnm/mdns-browser/pull/1178))

### Maintenance

- *(ci)* Enable sccache ([#1171](https://github.com/hrzlgnm/mdns-browser/pull/1171))

## [0.19.4] - 2025-06-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.3...mdns-browser-v0.19.4)

### Changed

- Remove redundant event structs and unify event type definitions ([#1164](https://github.com/hrzlgnm/mdns-browser/pull/1164))

- Move sections about audiable builds and attestation to readme ([#1166](https://github.com/hrzlgnm/mdns-browser/pull/1166))

- Simplify subscription in event listener functions ([#1167](https://github.com/hrzlgnm/mdns-browser/pull/1167))

- Remove unused GetUntracked import from Browse component ([#1168](https://github.com/hrzlgnm/mdns-browser/pull/1168))

- Fix typo in .coderabbit.yaml tone_instructions field ([#1169](https://github.com/hrzlgnm/mdns-browser/pull/1169))

## [0.19.3] - 2025-06-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.2...mdns-browser-v0.19.3)

### Added

- Ignore redundant updates of resolved services ([#1163](https://github.com/hrzlgnm/mdns-browser/pull/1163))

## [0.19.2] - 2025-06-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.1...mdns-browser-v0.19.2)

### Changed

- Refactor event system to use async subscriber closures ([#1157](https://github.com/hrzlgnm/mdns-browser/pull/1157))

- Only specify up to minor version in dependencies ([#1159](https://github.com/hrzlgnm/mdns-browser/pull/1159))

- Only specify up to minor version in more dependencies ([#1160](https://github.com/hrzlgnm/mdns-browser/pull/1160))

### Dependencies

- *(deps)* Lock file maintenance ([#1161](https://github.com/hrzlgnm/mdns-browser/pull/1161))

### Fixed

- Spelling of browse_types command ([#1162](https://github.com/hrzlgnm/mdns-browser/pull/1162))

## [0.19.1] - 2025-06-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.0...mdns-browser-v0.19.1)

### Changed

- Document `--enable-devtools` cli option ([#1141](https://github.com/hrzlgnm/mdns-browser/pull/1141))

- Event listening system and helpers ([#1149](https://github.com/hrzlgnm/mdns-browser/pull/1149))

- Change glob import to specific component import ([#1152](https://github.com/hrzlgnm/mdns-browser/pull/1152))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to cc583ad ([#1142](https://github.com/hrzlgnm/mdns-browser/pull/1142))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 5a9a49a ([#1143](https://github.com/hrzlgnm/mdns-browser/pull/1143))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to f5bda9f ([#1144](https://github.com/hrzlgnm/mdns-browser/pull/1144))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to ebdf265 ([#1145](https://github.com/hrzlgnm/mdns-browser/pull/1145))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 2a4a2c1 ([#1146](https://github.com/hrzlgnm/mdns-browser/pull/1146))

- *(deps)* Update crate-ci/typos action to v1.33.1 ([#1147](https://github.com/hrzlgnm/mdns-browser/pull/1147))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to c6e7319 ([#1148](https://github.com/hrzlgnm/mdns-browser/pull/1148))

- *(deps)* Update archlinux:base-devel docker digest to 3f7f6e4 ([#1151](https://github.com/hrzlgnm/mdns-browser/pull/1151))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 4b0ca29 ([#1154](https://github.com/hrzlgnm/mdns-browser/pull/1154))

### Fixed

- Report an Issue to preload the correct issue template ([#1153](https://github.com/hrzlgnm/mdns-browser/pull/1153))

## [0.19.0] - 2025-05-27 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.5...mdns-browser-v0.19.0)

### Added

- Add enable_devtools CLI argument to enable devtools at startup ([#1136](https://github.com/hrzlgnm/mdns-browser/pull/1136))

- Disable Verify button of resolved services when not browsing anymore ([#1138](https://github.com/hrzlgnm/mdns-browser/pull/1138))

### Changed

- Centralize protocol flag management in a ProtocolFlags component ([#1135](https://github.com/hrzlgnm/mdns-browser/pull/1135))

- Refactor protocol flags handling to set entire object and optimize updates ([#1139](https://github.com/hrzlgnm/mdns-browser/pull/1139))

- Don't update a dead service if removed again ([#1140](https://github.com/hrzlgnm/mdns-browser/pull/1140))

### Dependencies

- *(deps)* Update rust crate tokio to v1.45.1 ([#1131](https://github.com/hrzlgnm/mdns-browser/pull/1131))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 6ddd6c7 ([#1132](https://github.com/hrzlgnm/mdns-browser/pull/1132))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 8897189 ([#1134](https://github.com/hrzlgnm/mdns-browser/pull/1134))

- *(deps)* Update rust crate clap to v4.5.39 ([#1137](https://github.com/hrzlgnm/mdns-browser/pull/1137))

## [0.18.5] - 2025-05-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.4...mdns-browser-v0.18.5)

### Added

- Enhance filtering services by their `dead` or `alive` status. ([#1130](https://github.com/hrzlgnm/mdns-browser/pull/1130))

## [0.18.4] - 2025-05-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.3...mdns-browser-v0.18.4)

### Added

- Integrate icon rendering into CopyToClipBoardButton ([#1129](https://github.com/hrzlgnm/mdns-browser/pull/1129))

## [0.18.3] - 2025-05-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.2...mdns-browser-v0.18.3)

### Changed

- Simplify layout in ResolvedServiceItem components ([#1127](https://github.com/hrzlgnm/mdns-browser/pull/1127))

## [0.18.2] - 2025-05-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.1...mdns-browser-v0.18.2)

### Fixed

- Refine UI layout for service status ([#1126](https://github.com/hrzlgnm/mdns-browser/pull/1126))

## [0.18.1] - 2025-05-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.0...mdns-browser-v0.18.1)

### Changed

- Consolidate shared crates to workspace dependencies ([#1123](https://github.com/hrzlgnm/mdns-browser/pull/1123))

- Remove uuid crate dependency workaround ([#1124](https://github.com/hrzlgnm/mdns-browser/pull/1124))

### Fixed

- Make display of table row button texts reactive ([#1125](https://github.com/hrzlgnm/mdns-browser/pull/1125))

## [0.18.0] - 2025-05-23 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.17.1...mdns-browser-v0.18.0)

### Added

- Replace  `disabled` style by  colored status circle icon for removed services ([#1121](https://github.com/hrzlgnm/mdns-browser/pull/1121))

### Changed

- Disable lock file maintenance in renovate config

- Add coderabbit configuration file

- Ensure we are not browsing after a frontend reload ([#1112](https://github.com/hrzlgnm/mdns-browser/pull/1112))

- Migrate to thaw 0.5.0-beta ([#1074](https://github.com/hrzlgnm/mdns-browser/pull/1074))

- Tweak coderabbit to talk like jblow ([#1115](https://github.com/hrzlgnm/mdns-browser/pull/1115))

- Use a reactive stores to track resolved service updates ([#1105](https://github.com/hrzlgnm/mdns-browser/pull/1105))

- Reduce timestamp precision to microseconds ([#1120](https://github.com/hrzlgnm/mdns-browser/pull/1120))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to f44a86a ([#1107](https://github.com/hrzlgnm/mdns-browser/pull/1107))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 22df7d1 ([#1108](https://github.com/hrzlgnm/mdns-browser/pull/1108))

- *(deps)* Lock file maintenance ([#1109](https://github.com/hrzlgnm/mdns-browser/pull/1109))

- *(deps)* Update rust crate leptos to v0.8.2 ([#1113](https://github.com/hrzlgnm/mdns-browser/pull/1113))

- *(deps)* Update rust crate uuid to v1.17.0 ([#1118](https://github.com/hrzlgnm/mdns-browser/pull/1118))

## [0.17.1] - 2025-05-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.17.0...mdns-browser-v0.17.1)

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to 08b8824 ([#1099](https://github.com/hrzlgnm/mdns-browser/pull/1099))

- *(deps)* Update rust crate clap to v4.5.38 ([#1100](https://github.com/hrzlgnm/mdns-browser/pull/1100))

- *(deps)* Update archlinux:base-devel docker digest to 880766d ([#1101](https://github.com/hrzlgnm/mdns-browser/pull/1101))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 1f275a2 ([#1102](https://github.com/hrzlgnm/mdns-browser/pull/1102))

- *(deps)* Update anchore/sbom-action digest to e11c554 ([#1104](https://github.com/hrzlgnm/mdns-browser/pull/1104))

### Fixed

- Ensure resolved records are updated ([#1106](https://github.com/hrzlgnm/mdns-browser/pull/1106))

## [0.17.0] - 2025-05-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.16.1...mdns-browser-v0.17.0)

### Added

- Allow for switching off usage of IPv4 or IPv6 for browsing ([#1098](https://github.com/hrzlgnm/mdns-browser/pull/1098))

### Changed

- *(config)* Migrate renovate config ([#1096](https://github.com/hrzlgnm/mdns-browser/pull/1096))

### Dependencies

- *(deps)* Update dependency trunk to v0.21.14 ([#1097](https://github.com/hrzlgnm/mdns-browser/pull/1097))

## [0.16.1] - 2025-05-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.16.0...mdns-browser-v0.16.1)

### Added

- Use dark theme on mobile platform by default ([#1095](https://github.com/hrzlgnm/mdns-browser/pull/1095))

### Changed

- Reorganize commands more ([#1089](https://github.com/hrzlgnm/mdns-browser/pull/1089))

- Reduce indentation levels in metrics update task ([#1088](https://github.com/hrzlgnm/mdns-browser/pull/1088))

- Cleanup naming ([#1087](https://github.com/hrzlgnm/mdns-browser/pull/1087))

- Use custom format for log messages ([#1093](https://github.com/hrzlgnm/mdns-browser/pull/1093))

- Remove unused events from backend ([#1094](https://github.com/hrzlgnm/mdns-browser/pull/1094))

### Dependencies

- *(deps)* Update rust crate tokio to v1.45.0 ([#1091](https://github.com/hrzlgnm/mdns-browser/pull/1091))

- *(deps)* Update sonarsource/sonarqube-scan-action digest to 2500896 ([#1092](https://github.com/hrzlgnm/mdns-browser/pull/1092))

## [0.16.0] - 2025-05-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.3...mdns-browser-v0.16.0)

### Added

- Start with system theme ([#1085](https://github.com/hrzlgnm/mdns-browser/pull/1085))

### Changed

- Pass value as RwSignal to AutoComplete ([#1077](https://github.com/hrzlgnm/mdns-browser/pull/1077))

- Factor out a ThemeSwitcher component ([#1078](https://github.com/hrzlgnm/mdns-browser/pull/1078))

- Move imports and remove redundant cloning ([#1081](https://github.com/hrzlgnm/mdns-browser/pull/1081))

- Use single source of truth for version ([#1082](https://github.com/hrzlgnm/mdns-browser/pull/1082))

- Reorganize tauri commands ([#1083](https://github.com/hrzlgnm/mdns-browser/pull/1083))

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to 39c081e ([#1084](https://github.com/hrzlgnm/mdns-browser/pull/1084))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 8924114 ([#1086](https://github.com/hrzlgnm/mdns-browser/pull/1086))

## [0.15.3] - 2025-05-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.2...mdns-browser-v0.15.3)

### Changed

- Disable trunk version check ([#1059](https://github.com/hrzlgnm/mdns-browser/pull/1059))

- Add bug report issue template ([#1060](https://github.com/hrzlgnm/mdns-browser/pull/1060))

- Fix bug report template

- Add screenshot sectio to bug report issue template

- Exclude sonar coverage

- Fix typo in comment

- Remove unused dependency leptos_meta ([#1070](https://github.com/hrzlgnm/mdns-browser/pull/1070))

### Dependencies

- *(deps)* Update actions/attest-build-provenance action to v2.3.0 ([#1054](https://github.com/hrzlgnm/mdns-browser/pull/1054))

- *(deps)* Update crate-ci/typos action to v1.31.2 ([#1053](https://github.com/hrzlgnm/mdns-browser/pull/1053))

- *(deps)* Update archlinux:base-devel docker digest to d53a6f8 ([#1055](https://github.com/hrzlgnm/mdns-browser/pull/1055))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 21d02a6 ([#1056](https://github.com/hrzlgnm/mdns-browser/pull/1056))

- *(deps)* Update rust crate chrono to v0.4.41 ([#1057](https://github.com/hrzlgnm/mdns-browser/pull/1057))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to e8290ad ([#1063](https://github.com/hrzlgnm/mdns-browser/pull/1063))

- *(deps)* Lock file maintenance ([#1065](https://github.com/hrzlgnm/mdns-browser/pull/1065))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 7816cc9 ([#1064](https://github.com/hrzlgnm/mdns-browser/pull/1064))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to a673de5 ([#1062](https://github.com/hrzlgnm/mdns-browser/pull/1062))

- *(deps)* Update rust crate leptos_meta to 0.8 ([#1067](https://github.com/hrzlgnm/mdns-browser/pull/1067))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to df0b9dc ([#1068](https://github.com/hrzlgnm/mdns-browser/pull/1068))

- *(deps)* Update crate-ci/typos action to v1.32.0 ([#1069](https://github.com/hrzlgnm/mdns-browser/pull/1069))

- *(deps)* Update rust crate thaw_utils to v0.1.2 ([#1072](https://github.com/hrzlgnm/mdns-browser/pull/1072))

- *(deps)* Update rust crate thaw to v0.4.7 ([#1071](https://github.com/hrzlgnm/mdns-browser/pull/1071))

- *(deps)* Update dependency cargo-auditable to v0.6.7 ([#1073](https://github.com/hrzlgnm/mdns-browser/pull/1073))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to b35edf6 ([#1075](https://github.com/hrzlgnm/mdns-browser/pull/1075))

- *(deps)* Update rust crate mdns-sd to v0.13.9 ([#1076](https://github.com/hrzlgnm/mdns-browser/pull/1076))

## [0.15.2] - 2025-04-27 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.1...mdns-browser-v0.15.2)

### Changed

- Format css and js code using prettier with tab width of 4 ([#1052](https://github.com/hrzlgnm/mdns-browser/pull/1052))

## [0.15.1] - 2025-04-27 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.0...mdns-browser-v0.15.1)

### Changed

- Deduplicate listening, browsing and table row rendering ([#1044](https://github.com/hrzlgnm/mdns-browser/pull/1044))

- Fix usage of !important in css ([#1046](https://github.com/hrzlgnm/mdns-browser/pull/1046))

- Add doc strings to app/listen.rs module ([#1047](https://github.com/hrzlgnm/mdns-browser/pull/1047))

- State tracking and improve command error handling ([#1049](https://github.com/hrzlgnm/mdns-browser/pull/1049))

### Dependencies

- *(deps)* Update rust crate tauri-plugin-updater to v2.7.1 ([#1033](https://github.com/hrzlgnm/mdns-browser/pull/1033))

- *(deps)* Update rust crate tauri-plugin-log to v2.4.0 ([#1034](https://github.com/hrzlgnm/mdns-browser/pull/1034))

- *(deps)* Update rust crate tauri to v2.5.1 ([#1035](https://github.com/hrzlgnm/mdns-browser/pull/1035))

- *(deps)* Update actions/download-artifact digest to d3f86a1 ([#1036](https://github.com/hrzlgnm/mdns-browser/pull/1036))

- *(deps)* Update anchore/sbom-action digest to 9f73021 ([#1037](https://github.com/hrzlgnm/mdns-browser/pull/1037))

## [0.15.0] - 2025-04-23 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.5...mdns-browser-v0.15.0)

### Dependencies

- *(deps)* Update archlinux:base-devel docker digest to ef9c9e8 ([#1030](https://github.com/hrzlgnm/mdns-browser/pull/1030))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to f19d580 ([#1031](https://github.com/hrzlgnm/mdns-browser/pull/1031))

- *(deps)* Update rust crate mdns-sd to v0.13.8 ([#1032](https://github.com/hrzlgnm/mdns-browser/pull/1032))

## [0.14.5] - 2025-04-21 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.4...mdns-browser-v0.14.5)

### Changed

- Cleanup workflows ([#1025](https://github.com/hrzlgnm/mdns-browser/pull/1025))

- Show metrics as grid instead of table ([#1029](https://github.com/hrzlgnm/mdns-browser/pull/1029))

### Dependencies

- *(deps)* Update softprops/action-gh-release digest to da05d55 ([#1026](https://github.com/hrzlgnm/mdns-browser/pull/1026))

- *(deps)* Update rust crate clap to v4.5.37 ([#1027](https://github.com/hrzlgnm/mdns-browser/pull/1027))

- *(deps)* Update rust crate thaw to v0.4.6 ([#1028](https://github.com/hrzlgnm/mdns-browser/pull/1028))

## [0.14.4] - 2025-04-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.3...mdns-browser-v0.14.4)

### Changed

- Subscribe void linux publish workflows to published event

## [0.14.3] - 2025-04-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.2...mdns-browser-v0.14.3)

### Changed

- Refactor publish workflows to start on prereleased releases

- Subscribe publish workflows to published event

## [0.14.2] - 2025-04-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.0...mdns-browser-v0.14.2)

### Changed

- Also create a tag when bumping the version

- Fix tagging

- Bump version

### Fixed

- Move counter badge next to stop button ([#1024](https://github.com/hrzlgnm/mdns-browser/pull/1024))

## [0.14.0] - 2025-04-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.2...mdns-browser-v0.14.0)

### Added

- Add number of displayed services badge ([#1023](https://github.com/hrzlgnm/mdns-browser/pull/1023))

### Changed

- Install komac in winget publish release action ([#1018](https://github.com/hrzlgnm/mdns-browser/pull/1018))

- Add sonarqube scanning ([#1020](https://github.com/hrzlgnm/mdns-browser/pull/1020))

### Dependencies

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to fbea29a ([#1019](https://github.com/hrzlgnm/mdns-browser/pull/1019))

- *(deps)* Pin dependencies ([#1021](https://github.com/hrzlgnm/mdns-browser/pull/1021))

## [0.13.2] - 2025-04-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.1...mdns-browser-v0.13.2)

### Changed

- Bump version workflow creates a signed commit ([#1008](https://github.com/hrzlgnm/mdns-browser/pull/1008))

- *(config)* Migrate renovate config ([#1014](https://github.com/hrzlgnm/mdns-browser/pull/1014))

### Dependencies

- *(deps)* Update tauri monorepo ([#1010](https://github.com/hrzlgnm/mdns-browser/pull/1010))

- *(deps)* Update rust crate mdns-sd to v0.13.7 ([#1015](https://github.com/hrzlgnm/mdns-browser/pull/1015))

### Fixed

- Downgrade crate mdns-sd to v0.13.2 to resolve an issue ([#1016](https://github.com/hrzlgnm/mdns-browser/pull/1016))

## [0.13.1] - 2025-04-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.0...mdns-browser-v0.13.1)

### Changed

- Document new command line options ([#999](https://github.com/hrzlgnm/mdns-browser/pull/999))

### Dependencies

- *(deps)* Lock file maintenance ([#994](https://github.com/hrzlgnm/mdns-browser/pull/994))

- *(deps)* Lock file maintenance ([#996](https://github.com/hrzlgnm/mdns-browser/pull/996))

- *(deps)* Update archlinux:base-devel docker digest to 6c2b425 ([#1000](https://github.com/hrzlgnm/mdns-browser/pull/1000))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to fcd57e0 ([#1004](https://github.com/hrzlgnm/mdns-browser/pull/1004))

### Maintenance

- *(ci)* Don't ask for confirmation in CI ([#991](https://github.com/hrzlgnm/mdns-browser/pull/991))

- *(ci)* Use correct tag_name property from the release event ([#992](https://github.com/hrzlgnm/mdns-browser/pull/992))

- *(ci)* Rename sbom artifacts and use platform names ([#1001](https://github.com/hrzlgnm/mdns-browser/pull/1001))

- *(ci)* Only checksum binaries and source tar balls ([#1003](https://github.com/hrzlgnm/mdns-browser/pull/1003))

## [0.13.0] - 2025-04-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.12.0...mdns-browser-v0.13.0)

### Added

- Block browsing if no network interface is up ([#986](https://github.com/hrzlgnm/mdns-browser/pull/986))

### Changed

- *(attestation)* Use correct path in SBOM attestation ([#965](https://github.com/hrzlgnm/mdns-browser/pull/965))

- Add checksums for tarball from tag archive ([#979](https://github.com/hrzlgnm/mdns-browser/pull/979))

- Add docker image for testing and publishing to AUR ([#981](https://github.com/hrzlgnm/mdns-browser/pull/981))

- Fix publishing new versions to AUR ([#980](https://github.com/hrzlgnm/mdns-browser/pull/980))

- *(actions)* Use specific version of michidk/run-komac ([#983](https://github.com/hrzlgnm/mdns-browser/pull/983))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.36 ([#964](https://github.com/hrzlgnm/mdns-browser/pull/964))

- *(deps)* Pin dependencies ([#984](https://github.com/hrzlgnm/mdns-browser/pull/984))

- *(deps)* Update ghcr.io/hrzlgnm/mdns-browser-arch-aur-builder:v1 docker digest to 91bbc1e ([#985](https://github.com/hrzlgnm/mdns-browser/pull/985))

### Maintenance

- *(ci)* Publish release assets checksums ([#967](https://github.com/hrzlgnm/mdns-browser/pull/967))

- *(ci)* Fix publishing release asset checksums ([#969](https://github.com/hrzlgnm/mdns-browser/pull/969))

- *(ci)* Fix publish release assets download repository name ([#970](https://github.com/hrzlgnm/mdns-browser/pull/970))

- *(ci)* Fix publish release assets permissions ([#971](https://github.com/hrzlgnm/mdns-browser/pull/971))

- *(ci)* Fix publish release assets action versions ([#972](https://github.com/hrzlgnm/mdns-browser/pull/972))

- *(ci)* Fix publish release checksums ([#973](https://github.com/hrzlgnm/mdns-browser/pull/973))

- *(ci)* Publish release checksums for each file separately ([#975](https://github.com/hrzlgnm/mdns-browser/pull/975))

- *(ci)* Publish checksums on released event ([#976](https://github.com/hrzlgnm/mdns-browser/pull/976))

- *(ci)* Add workflow for publishing to AUR ([#977](https://github.com/hrzlgnm/mdns-browser/pull/977))

## [0.12.0] - 2025-04-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.34...mdns-browser-v0.12.0)

### Added

- Add `-V, --version` option ([#963](https://github.com/hrzlgnm/mdns-browser/pull/963))

### Changed

- Add manually triggered bump version workflow ([#958](https://github.com/hrzlgnm/mdns-browser/pull/958))

- Fix Cargo.toml path ([#961](https://github.com/hrzlgnm/mdns-browser/pull/961))

- Fix setting up git config

### Dependencies

- *(deps)* Update actions/checkout action to v4 ([#959](https://github.com/hrzlgnm/mdns-browser/pull/959))

### Fixed

- Improve nvidia detection ([#956](https://github.com/hrzlgnm/mdns-browser/pull/956))

### Maintenance

- *(ci)* Set dry-run to true and create-release to false

- *(ci)* Migrate to manually triggered bumping workflow

- *(ci)* Version bump workflow: allow creating pull requests

- *(ci)* Fix updating shared constants in Cargo.lock

## [0.11.34] - 2025-04-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.33...mdns-browser-v0.11.34)

### Changed

- Fix macOS SBOM attestation ([#953](https://github.com/hrzlgnm/mdns-browser/pull/953))

- Subscribe `metrics` once, stop `type` browsing on reload ([#954](https://github.com/hrzlgnm/mdns-browser/pull/954))

## [0.11.33] - 2025-04-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.32...mdns-browser-v0.11.33)

### Dependencies

- *(deps)* Bump crossbeam-channel from 0.5.14 to 0.5.15 ([#948](https://github.com/hrzlgnm/mdns-browser/pull/948))

## [0.11.32] - 2025-04-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.31...mdns-browser-v0.11.32)

### Dependencies

- *(deps)* Update dependency trunk to v0.21.13 ([#941](https://github.com/hrzlgnm/mdns-browser/pull/941))

- *(deps)* Lock file maintenance ([#939](https://github.com/hrzlgnm/mdns-browser/pull/939))

- *(deps)* Update rust crate mdns-sd to v0.13.6 ([#942](https://github.com/hrzlgnm/mdns-browser/pull/942))

- *(deps)* Update actions/setup-java digest to c5195ef ([#943](https://github.com/hrzlgnm/mdns-browser/pull/943))

### Fixed

- Only disable webkit dmabuf rendering if nvidia driver is detected ([#949](https://github.com/hrzlgnm/mdns-browser/pull/949))

## [0.11.31] - 2025-04-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.30...mdns-browser-v0.11.31)

### Added

- Build macos bundle using universal-apple-darwin target ([#938](https://github.com/hrzlgnm/mdns-browser/pull/938))

### Changed

- Set auto bump label to ignore ([#934](https://github.com/hrzlgnm/mdns-browser/pull/934))

### Dependencies

- *(deps)* Update rust crate tokio to v1.44.2 ([#936](https://github.com/hrzlgnm/mdns-browser/pull/936))

## [0.11.30] - 2025-04-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.29...mdns-browser-v0.11.30)

### Added

- Use async runtime instead of threads ([#932](https://github.com/hrzlgnm/mdns-browser/pull/932))

### Changed

- Ignore version bump in release drafter ([#927](https://github.com/hrzlgnm/mdns-browser/pull/927))

- Use Swatinem/rust-cache for caching ([#928](https://github.com/hrzlgnm/mdns-browser/pull/928))

### Fixed

- Filter out ipv6 link local addresses for opening `_http._tcp` like services ([#930](https://github.com/hrzlgnm/mdns-browser/pull/930))

### Maintenance

- *(ci)* Use correct git ref for save-if condition when caching

## [0.11.29] - 2025-04-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.28...mdns-browser-v0.11.29)

### Added

- Start browsing after selecting a service type and pressing enter ([#923](https://github.com/hrzlgnm/mdns-browser/pull/923))

### Changed

- Add section about installing on void linux ([#913](https://github.com/hrzlgnm/mdns-browser/pull/913))

- Add command line options documentation ([#918](https://github.com/hrzlgnm/mdns-browser/pull/918))

- Update screenshots ([#920](https://github.com/hrzlgnm/mdns-browser/pull/920))

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.31.1 ([#908](https://github.com/hrzlgnm/mdns-browser/pull/908))

- *(deps)* Lock file maintenance ([#907](https://github.com/hrzlgnm/mdns-browser/pull/907))

- *(deps)* Update ghcr.io/void-linux/void-glibc docker digest to 32363ee ([#909](https://github.com/hrzlgnm/mdns-browser/pull/909))

- *(deps)* Update rust crate clap to v4.5.35 ([#910](https://github.com/hrzlgnm/mdns-browser/pull/910))

- *(deps)* Update tauri monorepo ([#911](https://github.com/hrzlgnm/mdns-browser/pull/911))

- *(deps)* Update rust crate tauri-plugin-updater to v2.7.0 ([#912](https://github.com/hrzlgnm/mdns-browser/pull/912))

### Fixed

- *(ux)* Make filtering service types more intuitive ([#921](https://github.com/hrzlgnm/mdns-browser/pull/921))

## [0.11.28] - 2025-03-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.27...mdns-browser-v0.11.28)

### Dependencies

- *(deps)* Update rust crate thaw to v0.4.5 ([#904](https://github.com/hrzlgnm/mdns-browser/pull/904))

## [0.11.15] - 2025-03-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.14...mdns-browser-v0.11.15)

### Changed

- Fix input check in desktop-tauri workflow

- Check tag after checking out

## [0.11.14] - 2025-03-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.13...mdns-browser-v0.11.14)

### Changed

- *(config)* Migrate renovate config ([#879](https://github.com/hrzlgnm/mdns-browser/pull/879))

- Verify tag matches version in tauri conf on publish ([#335](https://github.com/hrzlgnm/mdns-browser/pull/335))

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.31.0 ([#878](https://github.com/hrzlgnm/mdns-browser/pull/878))

- *(deps)* Update dependency trunk to v0.21.12 ([#880](https://github.com/hrzlgnm/mdns-browser/pull/880))

### Maintenance

- *(ci)* Enable sscache ([#881](https://github.com/hrzlgnm/mdns-browser/pull/881))

## [0.11.10] - 2025-03-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.9...mdns-browser-v0.11.10)

### Changed

- Make debug logging a noop in log_fn macro in release builds ([#871](https://github.com/hrzlgnm/mdns-browser/pull/871))

- Move inline style to CSS ([#872](https://github.com/hrzlgnm/mdns-browser/pull/872))

- Build void-linux xbps-package ([#873](https://github.com/hrzlgnm/mdns-browser/pull/873))

- Correct docker image for void linux

- Correct install dependencies

- Use bash for cloning void packages

- Add more dependencies to install

### Dependencies

- *(deps)* Update rust crate clap to v4.5.34 ([#868](https://github.com/hrzlgnm/mdns-browser/pull/868))

- *(deps)* Update dependency trunk to v0.21.11 ([#869](https://github.com/hrzlgnm/mdns-browser/pull/869))

- *(deps)* Update dependency trunk to v0.21.12 ([#870](https://github.com/hrzlgnm/mdns-browser/pull/870))

## [0.11.9] - 2025-03-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.8...mdns-browser-v0.11.9)

### Added

- Pressing enter in quick filter starts browsing ([#866](https://github.com/hrzlgnm/mdns-browser/pull/866))

## [0.11.8] - 2025-03-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.7...mdns-browser-v0.11.8)

### Added

- Clear resolved services when starting browsing instead of stopping ([#858](https://github.com/hrzlgnm/mdns-browser/pull/858))

- Add support for opening resolved services ([#864](https://github.com/hrzlgnm/mdns-browser/pull/864))

### Dependencies

- *(deps)* Update rust crate mdns-sd to v0.13.5 ([#853](https://github.com/hrzlgnm/mdns-browser/pull/853))

- *(deps)* Update rust crate clap to v4.5.33 ([#863](https://github.com/hrzlgnm/mdns-browser/pull/863))

### Fixed

- Make details view scrollable ([#861](https://github.com/hrzlgnm/mdns-browser/pull/861))

## [0.11.6] - 2025-03-25 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.5...mdns-browser-v0.11.6)

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.30.3 ([#844](https://github.com/hrzlgnm/mdns-browser/pull/844))

- *(deps)* Lock file maintenance ([#838](https://github.com/hrzlgnm/mdns-browser/pull/838))

- *(deps)* Update rust crate mdns-sd to v0.13.4 ([#845](https://github.com/hrzlgnm/mdns-browser/pull/845))

### Fixed

- Ignore invalid service types when browsing service types ([#846](https://github.com/hrzlgnm/mdns-browser/pull/846))

- Make metrics visible again by using log_fn! macro ([#848](https://github.com/hrzlgnm/mdns-browser/pull/848))

## [0.11.5] - 2025-03-24 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.4...mdns-browser-v0.11.5)

### Dependencies

- *(deps)* Update actions/upload-artifact action to v4.6.2 ([#832](https://github.com/hrzlgnm/mdns-browser/pull/832))

- *(deps)* Update rust crate leptos to v0.7.8 ([#834](https://github.com/hrzlgnm/mdns-browser/pull/834))

- *(deps)* Update rust crate leptos_meta to v0.7.8 ([#835](https://github.com/hrzlgnm/mdns-browser/pull/835))

- *(deps)* Lock file maintenance ([#837](https://github.com/hrzlgnm/mdns-browser/pull/837))

- *(deps)* Update dependency tauri-cli to v2.4.0 ([#836](https://github.com/hrzlgnm/mdns-browser/pull/836))

- *(deps)* Update rust crate log to v0.4.27 ([#839](https://github.com/hrzlgnm/mdns-browser/pull/839))

### Fixed

- Improve error handling to avoid crashing due to panics ([#840](https://github.com/hrzlgnm/mdns-browser/pull/840))

## [0.11.4] - 2025-03-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.3...mdns-browser-v0.11.4)

### Added

- Enabled building debug symbols ([#831](https://github.com/hrzlgnm/mdns-browser/pull/831))

### Dependencies

- *(deps)* Update actions/upload-artifact action to v4.6.2 ([#830](https://github.com/hrzlgnm/mdns-browser/pull/830))

## [0.11.3] - 2025-03-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.2...mdns-browser-v0.11.3)

### Added

- Add command line option to enable logging to file ([#827](https://github.com/hrzlgnm/mdns-browser/pull/827))

### Changed

- Deduplicate getting class for desktop/mobile ([#823](https://github.com/hrzlgnm/mdns-browser/pull/823))

- Split components into modules ([#824](https://github.com/hrzlgnm/mdns-browser/pull/824))

- More modules ([#825](https://github.com/hrzlgnm/mdns-browser/pull/825))

### Dependencies

- *(deps)* Update rust crate uuid to v1.16.0 ([#819](https://github.com/hrzlgnm/mdns-browser/pull/819))

- *(deps)* Update dependency trunk to v0.21.9 ([#820](https://github.com/hrzlgnm/mdns-browser/pull/820))

- *(deps)* Update rust crate tauri-plugin-updater to v2.6.1 ([#821](https://github.com/hrzlgnm/mdns-browser/pull/821))

- *(deps)* Lock file maintenance ([#822](https://github.com/hrzlgnm/mdns-browser/pull/822))

## [0.11.2] - 2025-03-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.1...mdns-browser-v0.11.2)

### Added

- Update style of splashscreen to match the theme from thaw ([#815](https://github.com/hrzlgnm/mdns-browser/pull/815))

- Update icon colors to match the theme of thaw ([#816](https://github.com/hrzlgnm/mdns-browser/pull/816))

### Changed

- Deduplicate size of CopyToClipboardButton with a default ([#817](https://github.com/hrzlgnm/mdns-browser/pull/817))

### Dependencies

- *(deps)* Update rust crate tauri-plugin-log to v2.3.1 ([#813](https://github.com/hrzlgnm/mdns-browser/pull/813))

- *(deps)* Update rust crate tokio to v1.44.1 ([#814](https://github.com/hrzlgnm/mdns-browser/pull/814))

## [0.11.1] - 2025-03-12 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.0...mdns-browser-v0.11.1)

### Added

- Disable any autocorrection attribute along with autocapitalization ([#808](https://github.com/hrzlgnm/mdns-browser/pull/808))

- Use a table to visualize the resolved service card ([#809](https://github.com/hrzlgnm/mdns-browser/pull/809))

## [0.11.0] - 2025-03-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.12...mdns-browser-v0.11.0)

### Added

- Milestone 0.11 kickoff ([#781](https://github.com/hrzlgnm/mdns-browser/pull/781))

### Changed

- Migrate to leptos 0.7 and thaw 0.4 ([#775](https://github.com/hrzlgnm/mdns-browser/pull/775))

### Dependencies

- *(deps)* Update rust crate serde to v1.0.218 ([#747](https://github.com/hrzlgnm/mdns-browser/pull/747))

- *(deps)* Update rust crate serde_json to v1.0.139 ([#746](https://github.com/hrzlgnm/mdns-browser/pull/746))

- *(deps)* Update crate-ci/typos action to v1.29.9 ([#748](https://github.com/hrzlgnm/mdns-browser/pull/748))

- *(deps)* Update rust crate uuid to v1.14.0 ([#749](https://github.com/hrzlgnm/mdns-browser/pull/749))

- *(deps)* Update rust crate log to v0.4.26 ([#750](https://github.com/hrzlgnm/mdns-browser/pull/750))

- *(deps)* Update actions/upload-artifact action to v4.6.1 ([#751](https://github.com/hrzlgnm/mdns-browser/pull/751))

- *(deps)* Update peter-evans/create-pull-request digest to dd2324f ([#753](https://github.com/hrzlgnm/mdns-browser/pull/753))

- *(deps)* Update rust crate tauri-plugin-log to v2.2.2 ([#754](https://github.com/hrzlgnm/mdns-browser/pull/754))

- *(deps)* Update rust crate tauri-plugin-updater to v2.5.1 ([#755](https://github.com/hrzlgnm/mdns-browser/pull/755))

- *(deps)* Update rust crate clap to v4.5.31 ([#738](https://github.com/hrzlgnm/mdns-browser/pull/738))

- *(deps)* Lock file maintenance ([#752](https://github.com/hrzlgnm/mdns-browser/pull/752))

- *(deps)* Update crate-ci/typos action to v1.29.10 ([#756](https://github.com/hrzlgnm/mdns-browser/pull/756))

- *(deps)* Update rust crate uuid to v1.15.0 ([#757](https://github.com/hrzlgnm/mdns-browser/pull/757))

- *(deps)* Update rust crate chrono to v0.4.40 ([#758](https://github.com/hrzlgnm/mdns-browser/pull/758))

- *(deps)* Update tauri monorepo ([#759](https://github.com/hrzlgnm/mdns-browser/pull/759))

- *(deps)* Update actions/attest-build-provenance action to v2.2.1 ([#760](https://github.com/hrzlgnm/mdns-browser/pull/760))

- *(deps)* Update rust crate uuid to v1.15.1 ([#761](https://github.com/hrzlgnm/mdns-browser/pull/761))

- *(deps)* Update actions/attest-build-provenance action to v2.2.2 ([#762](https://github.com/hrzlgnm/mdns-browser/pull/762))

- *(deps)* Update rust crate tauri-plugin-log to v2.2.3 ([#763](https://github.com/hrzlgnm/mdns-browser/pull/763))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.6 ([#764](https://github.com/hrzlgnm/mdns-browser/pull/764))

- *(deps)* Update tauri monorepo to v2.3.1 ([#765](https://github.com/hrzlgnm/mdns-browser/pull/765))

- *(deps)* Update crate-ci/typos action to v1.30.0 ([#766](https://github.com/hrzlgnm/mdns-browser/pull/766))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.20 ([#767](https://github.com/hrzlgnm/mdns-browser/pull/767))

- *(deps)* Update rust crate mdns-sd to v0.13.3 ([#768](https://github.com/hrzlgnm/mdns-browser/pull/768))

- *(deps)* Update rust crate thiserror to v2.0.12 ([#770](https://github.com/hrzlgnm/mdns-browser/pull/770))

- *(deps)* Update rust crate serde_json to v1.0.140 ([#771](https://github.com/hrzlgnm/mdns-browser/pull/771))

- *(deps)* Lock file maintenance ([#769](https://github.com/hrzlgnm/mdns-browser/pull/769))

- *(deps)* Update crate-ci/typos action to v1.30.1 ([#772](https://github.com/hrzlgnm/mdns-browser/pull/772))

- *(deps)* Update dependency trunk to v0.21.8 ([#774](https://github.com/hrzlgnm/mdns-browser/pull/774))

- *(deps)* Update peter-evans/create-pull-request digest to 271a8d0 ([#773](https://github.com/hrzlgnm/mdns-browser/pull/773))

- *(deps)* Update actions/attest-build-provenance action to v2.2.3 ([#782](https://github.com/hrzlgnm/mdns-browser/pull/782))

- *(deps)* Update rust crate tokio to v1.44.0 ([#786](https://github.com/hrzlgnm/mdns-browser/pull/786))

- *(deps)* Lock file maintenance ([#787](https://github.com/hrzlgnm/mdns-browser/pull/787))

- *(deps)* Update rust crate serde to v1.0.219 ([#791](https://github.com/hrzlgnm/mdns-browser/pull/791))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.2.2 ([#794](https://github.com/hrzlgnm/mdns-browser/pull/794))

- *(deps)* Update rust crate tauri-plugin-log to v2.3.0 ([#795](https://github.com/hrzlgnm/mdns-browser/pull/795))

- *(deps)* Update crate-ci/typos action to v1.30.2 ([#796](https://github.com/hrzlgnm/mdns-browser/pull/796))

- *(deps)* Update rust crate tauri-plugin-updater to v2.6.0 ([#797](https://github.com/hrzlgnm/mdns-browser/pull/797))

- *(deps)* Lock file maintenance ([#793](https://github.com/hrzlgnm/mdns-browser/pull/793))

- *(deps)* Update rust crate clap to v4.5.32 ([#801](https://github.com/hrzlgnm/mdns-browser/pull/801))

### Fixed

- Table layout and unify copy to clipboard toasting experience ([#789](https://github.com/hrzlgnm/mdns-browser/pull/789))

- Resolved service grid layout ([#790](https://github.com/hrzlgnm/mdns-browser/pull/790))

- Browsing added service types while browsing all ([#798](https://github.com/hrzlgnm/mdns-browser/pull/798))

- Bring back service type invalid on input feedback ([#799](https://github.com/hrzlgnm/mdns-browser/pull/799))

- Use Signal::get_untracked() in non reactive contexts ([#800](https://github.com/hrzlgnm/mdns-browser/pull/800))

- Workaround not being able to set autocapitalize attribute to inputs ([#803](https://github.com/hrzlgnm/mdns-browser/pull/803))

- Layout issues on android ([#804](https://github.com/hrzlgnm/mdns-browser/pull/804))

## [0.10.12] - 2025-02-19 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.11...mdns-browser-v0.10.12)

### Changed

- Fully migrate to baptiste0928/cargo-install for installing crates ([#724](https://github.com/hrzlgnm/mdns-browser/pull/724))

- Discard old drafts ([#742](https://github.com/hrzlgnm/mdns-browser/pull/742))

- Discard old drafts ([#743](https://github.com/hrzlgnm/mdns-browser/pull/743))

### Dependencies

- *(deps)* Update actions/setup-java digest to 3a4f6e1 ([#723](https://github.com/hrzlgnm/mdns-browser/pull/723))

- *(deps)* Update dependency leptosfmt to v0.1.33 ([#725](https://github.com/hrzlgnm/mdns-browser/pull/725))

- *(deps)* Update crate-ci/typos action to v1.29.5 ([#726](https://github.com/hrzlgnm/mdns-browser/pull/726))

- *(deps)* Update rust crate mdns-sd to v0.13.2 ([#728](https://github.com/hrzlgnm/mdns-browser/pull/728))

- *(deps)* Lock file maintenance ([#727](https://github.com/hrzlgnm/mdns-browser/pull/727))

- *(deps)* Update rust crate clap to v4.5.28 ([#729](https://github.com/hrzlgnm/mdns-browser/pull/729))

- *(deps)* Update rust crate tauri-plugin-updater to v2.5.0 ([#730](https://github.com/hrzlgnm/mdns-browser/pull/730))

- *(deps)* Update rust crate clap to v4.5.29 ([#733](https://github.com/hrzlgnm/mdns-browser/pull/733))

- *(deps)* Update crate-ci/typos action to v1.29.7 ([#736](https://github.com/hrzlgnm/mdns-browser/pull/736))

- *(deps)* Update dependency on uuid with explicit features set ([#735](https://github.com/hrzlgnm/mdns-browser/pull/735))

- *(deps)* Lock file maintenance ([#734](https://github.com/hrzlgnm/mdns-browser/pull/734))

- *(deps)* Lock file maintenance ([#737](https://github.com/hrzlgnm/mdns-browser/pull/737))

- *(deps)* Lock file maintenance ([#739](https://github.com/hrzlgnm/mdns-browser/pull/739))

- *(deps)* Update rust crate uuid to v1.13.2 ([#740](https://github.com/hrzlgnm/mdns-browser/pull/740))

- *(deps)* Update crate-ci/typos action to v1.29.8 ([#741](https://github.com/hrzlgnm/mdns-browser/pull/741))

- *(deps)* Update hugo19941994/delete-draft-releases action to v1.0.1 ([#744](https://github.com/hrzlgnm/mdns-browser/pull/744))

## [0.10.11] - 2025-01-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.10...mdns-browser-v0.10.11)

### Added

- Build linux bundles with ubuntu-22.04 ([#721](https://github.com/hrzlgnm/mdns-browser/pull/721))

## [0.10.10] - 2025-01-28 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.9...mdns-browser-v0.10.10)

### Dependencies

- *(deps)* Update dependency trunk to v0.21.7 ([#707](https://github.com/hrzlgnm/mdns-browser/pull/707))

- *(deps)* Update actions/attest-build-provenance action to v2.2.0 ([#708](https://github.com/hrzlgnm/mdns-browser/pull/708))

- *(deps)* Update actions/attest-sbom digest to 115c3be ([#709](https://github.com/hrzlgnm/mdns-browser/pull/709))

- *(deps)* Update anchore/sbom-action digest to f325610 ([#710](https://github.com/hrzlgnm/mdns-browser/pull/710))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.19 ([#711](https://github.com/hrzlgnm/mdns-browser/pull/711))

- *(deps)* Update tauri monorepo ([#712](https://github.com/hrzlgnm/mdns-browser/pull/712))

- *(deps)* Update tauri monorepo ([#713](https://github.com/hrzlgnm/mdns-browser/pull/713))

- *(deps)* Lock file maintenance ([#714](https://github.com/hrzlgnm/mdns-browser/pull/714))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.2.1 ([#715](https://github.com/hrzlgnm/mdns-browser/pull/715))

- *(deps)* Update rust crate tauri-plugin-updater to v2.4.0 ([#717](https://github.com/hrzlgnm/mdns-browser/pull/717))

- *(deps)* Update rust crate tauri-plugin-log to v2.2.1 ([#716](https://github.com/hrzlgnm/mdns-browser/pull/716))

- *(deps)* Update rust crate serde_json to v1.0.138 ([#718](https://github.com/hrzlgnm/mdns-browser/pull/718))

### Fixed

- Shorten text in version is already latest ([#719](https://github.com/hrzlgnm/mdns-browser/pull/719))

## [0.10.9] - 2025-01-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.8...mdns-browser-v0.10.9)

### Added

- Browse for added service types while browsing all ([#704](https://github.com/hrzlgnm/mdns-browser/pull/704))

### Dependencies

- *(deps)* Update rust crate serde_json to v1.0.136 ([#694](https://github.com/hrzlgnm/mdns-browser/pull/694))

- *(deps)* Update tauri monorepo ([#695](https://github.com/hrzlgnm/mdns-browser/pull/695))

- *(deps)* Update release-drafter/release-drafter action to v6.1.0 ([#696](https://github.com/hrzlgnm/mdns-browser/pull/696))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.5 ([#697](https://github.com/hrzlgnm/mdns-browser/pull/697))

- *(deps)* Update rust crate serde_json to v1.0.137 ([#698](https://github.com/hrzlgnm/mdns-browser/pull/698))

- *(deps)* Update dependency trunk to v0.21.6 ([#700](https://github.com/hrzlgnm/mdns-browser/pull/700))

- *(deps)* Lock file maintenance ([#699](https://github.com/hrzlgnm/mdns-browser/pull/699))

- *(deps)* Update taiki-e/cache-cargo-install-action action to v2.1.0 ([#701](https://github.com/hrzlgnm/mdns-browser/pull/701))

- *(deps)* Update rust crate clap to v4.5.27 ([#705](https://github.com/hrzlgnm/mdns-browser/pull/705))

## [0.10.8] - 2025-01-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.7...mdns-browser-v0.10.8)

### Added

- Add icon for switching between dark and light mode ([#692](https://github.com/hrzlgnm/mdns-browser/pull/692))

### Changed

- Migrate to winget-updater action ([#643](https://github.com/hrzlgnm/mdns-browser/pull/643))

- Fix dependency in winget publish workflow ([#645](https://github.com/hrzlgnm/mdns-browser/pull/645))

- Fix versioning in winget publish workflow ([#646](https://github.com/hrzlgnm/mdns-browser/pull/646))

- Fix versioning in winget publish workflow

- Manually extract short version

- Fix passing token to komac

- Remove unintendedly added input args

- Update komac version to 2.8.0

- Set ws_protocol to "ws" in trunk config ([#656](https://github.com/hrzlgnm/mdns-browser/pull/656))

- Add badges to readme ([#676](https://github.com/hrzlgnm/mdns-browser/pull/676))

- Fix clippy warning about redundant map_or usage ([#678](https://github.com/hrzlgnm/mdns-browser/pull/678))

- Drop Retry on Failure Workflow as it doesn't work as intended ([#679](https://github.com/hrzlgnm/mdns-browser/pull/679))

### Dependencies

- *(deps)* Update dependency ubuntu to v24 ([#644](https://github.com/hrzlgnm/mdns-browser/pull/644))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.18 ([#647](https://github.com/hrzlgnm/mdns-browser/pull/647))

- *(deps)* Update rust crate serde_json to v1.0.134 ([#648](https://github.com/hrzlgnm/mdns-browser/pull/648))

- *(deps)* Update rust crate thiserror to v2.0.9 ([#649](https://github.com/hrzlgnm/mdns-browser/pull/649))

- *(deps)* Lock file maintenance ([#650](https://github.com/hrzlgnm/mdns-browser/pull/650))

- *(deps)* Update peter-evans/create-pull-request digest to 67ccf78 ([#654](https://github.com/hrzlgnm/mdns-browser/pull/654))

- *(deps)* Update baptiste0928/cargo-install digest to bd2e567 ([#653](https://github.com/hrzlgnm/mdns-browser/pull/653))

- *(deps)* Update rust crate serde to v1.0.217 ([#655](https://github.com/hrzlgnm/mdns-browser/pull/655))

- *(deps)* Lock file maintenance ([#657](https://github.com/hrzlgnm/mdns-browser/pull/657))

- *(deps)* Update crate-ci/typos action to v1.29.0 ([#658](https://github.com/hrzlgnm/mdns-browser/pull/658))

- *(deps)* Update crate-ci/typos action to v1.29.1 ([#660](https://github.com/hrzlgnm/mdns-browser/pull/660))

- *(deps)* Update tauri monorepo ([#661](https://github.com/hrzlgnm/mdns-browser/pull/661))

- *(deps)* Update crate-ci/typos action to v1.29.3 ([#662](https://github.com/hrzlgnm/mdns-browser/pull/662))

- *(deps)* Update crate-ci/typos action to v1.29.4 ([#663](https://github.com/hrzlgnm/mdns-browser/pull/663))

- *(deps)* Update dependency tauri-cli to v2.2.1 ([#664](https://github.com/hrzlgnm/mdns-browser/pull/664))

- *(deps)* Update dependency tauri-cli to v2.2.2 ([#665](https://github.com/hrzlgnm/mdns-browser/pull/665))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.3 ([#668](https://github.com/hrzlgnm/mdns-browser/pull/668))

- *(deps)* Update rust crate tauri-plugin-updater to v2.3.1 ([#669](https://github.com/hrzlgnm/mdns-browser/pull/669))

- *(deps)* Lock file maintenance ([#667](https://github.com/hrzlgnm/mdns-browser/pull/667))

- *(deps)* Update baptiste0928/cargo-install digest to 91c5da1 ([#666](https://github.com/hrzlgnm/mdns-browser/pull/666))

- *(deps)* Update rust crate clap to v4.5.24 ([#671](https://github.com/hrzlgnm/mdns-browser/pull/671))

- *(deps)* Update rust crate serde_json to v1.0.135 ([#673](https://github.com/hrzlgnm/mdns-browser/pull/673))

- *(deps)* Update rust crate thiserror to v2.0.10 ([#675](https://github.com/hrzlgnm/mdns-browser/pull/675))

- *(deps)* Update rust crate tokio to v1.43.0 ([#674](https://github.com/hrzlgnm/mdns-browser/pull/674))

- *(deps)* Update softprops/action-gh-release digest to c95fe14 ([#670](https://github.com/hrzlgnm/mdns-browser/pull/670))

- *(deps)* Update rust crate clap to v4.5.25 ([#677](https://github.com/hrzlgnm/mdns-browser/pull/677))

- *(deps)* Update rust crate clap to v4.5.26 ([#680](https://github.com/hrzlgnm/mdns-browser/pull/680))

- *(deps)* Update actions/upload-artifact action to v4.6.0 ([#685](https://github.com/hrzlgnm/mdns-browser/pull/685))

- *(deps)* Update tauri monorepo ([#684](https://github.com/hrzlgnm/mdns-browser/pull/684))

- *(deps)* Update rust crate thiserror to v2.0.11 ([#686](https://github.com/hrzlgnm/mdns-browser/pull/686))

- *(deps)* Update rust crate log to v0.4.24 ([#682](https://github.com/hrzlgnm/mdns-browser/pull/682))

- *(deps)* Update dependency tauri-cli to v2.2.4 ([#687](https://github.com/hrzlgnm/mdns-browser/pull/687))

- *(deps)* Lock file maintenance ([#688](https://github.com/hrzlgnm/mdns-browser/pull/688))

- *(deps)* Update rust crate tauri to v2.2.2 ([#689](https://github.com/hrzlgnm/mdns-browser/pull/689))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.4 ([#690](https://github.com/hrzlgnm/mdns-browser/pull/690))

- *(deps)* Update rust crate log to v0.4.25 ([#691](https://github.com/hrzlgnm/mdns-browser/pull/691))

## [0.10.7] - 2024-12-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.6...mdns-browser-v0.10.7)

### Added

- Unify handling of trailing dot when copying to clipboard ([#641](https://github.com/hrzlgnm/mdns-browser/pull/641))

### Changed

- Migrate to baptiste0928/cargo-install action for installing tools ([#639](https://github.com/hrzlgnm/mdns-browser/pull/639))

- Fix renovate config ([#640](https://github.com/hrzlgnm/mdns-browser/pull/640))

### Dependencies

- *(deps)* Update actions/upload-artifact action to v4.5.0 ([#632](https://github.com/hrzlgnm/mdns-browser/pull/632))

- *(deps)* Update rust crate thiserror to v2.0.8 ([#637](https://github.com/hrzlgnm/mdns-browser/pull/637))

- *(deps)* Update actions/setup-java digest to 7a6d8a8 ([#638](https://github.com/hrzlgnm/mdns-browser/pull/638))

## [0.10.5] - 2024-12-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.4...mdns-browser-v0.10.5)

### Added

- Also use opener plugin on android ([#619](https://github.com/hrzlgnm/mdns-browser/pull/619))

- Add toast when text is copied to clipboard and use default cursor ([#625](https://github.com/hrzlgnm/mdns-browser/pull/625))

- Only show toast about copied text on desktop platforms ([#627](https://github.com/hrzlgnm/mdns-browser/pull/627))

### Changed

- Install trunk without `evil` features ([#626](https://github.com/hrzlgnm/mdns-browser/pull/626))

- Ci runs tests only on macos runners ([#629](https://github.com/hrzlgnm/mdns-browser/pull/629))

### Dependencies

- *(deps)* Update rust crate tauri-plugin-updater to v2.3.0 ([#612](https://github.com/hrzlgnm/mdns-browser/pull/612))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.1 ([#611](https://github.com/hrzlgnm/mdns-browser/pull/611))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.17 ([#613](https://github.com/hrzlgnm/mdns-browser/pull/613))

- *(deps)* Update rust crate serde to v1.0.216 ([#614](https://github.com/hrzlgnm/mdns-browser/pull/614))

- *(deps)* Update crate-ci/typos action to v1.28.3 ([#615](https://github.com/hrzlgnm/mdns-browser/pull/615))

- *(deps)* Update rust crate thiserror to v2.0.7 ([#617](https://github.com/hrzlgnm/mdns-browser/pull/617))

- *(deps)* Update anchore/sbom-action digest to df80a98 ([#616](https://github.com/hrzlgnm/mdns-browser/pull/616))

- *(deps)* Update rust crate tauri-plugin-opener to v2.2.2 ([#618](https://github.com/hrzlgnm/mdns-browser/pull/618))

- *(deps)* Update rust crate mdns-sd to 0.13.0 ([#621](https://github.com/hrzlgnm/mdns-browser/pull/621))

- *(deps)* Lock file maintenance ([#620](https://github.com/hrzlgnm/mdns-browser/pull/620))

- *(deps)* Update dependency trunk to v0.21.5 ([#622](https://github.com/hrzlgnm/mdns-browser/pull/622))

- *(deps)* Update crate-ci/typos action to v1.28.4 ([#623](https://github.com/hrzlgnm/mdns-browser/pull/623))

- *(deps)* Update rust crate mdns-sd to v0.13.1 ([#624](https://github.com/hrzlgnm/mdns-browser/pull/624))

- *(deps)* Pin baptiste0928/cargo-install action to 904927d ([#628](https://github.com/hrzlgnm/mdns-browser/pull/628))

## [0.10.4] - 2024-12-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.3...mdns-browser-v0.10.4)

### Added

- Disable autocapitalize attribute in quick-filter input ([#608](https://github.com/hrzlgnm/mdns-browser/pull/608))

### Dependencies

- *(deps)* Update actions/attest-build-provenance action to v2.1.0 ([#606](https://github.com/hrzlgnm/mdns-browser/pull/606))

- *(deps)* Update actions/attest-sbom digest to cbfd002 ([#609](https://github.com/hrzlgnm/mdns-browser/pull/609))

## [0.10.3] - 2024-12-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.2...mdns-browser-v0.10.3)

### Fixed

- Only use opener plugin on desktop platforms ([#605](https://github.com/hrzlgnm/mdns-browser/pull/605))

## [0.10.2] - 2024-12-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.1...mdns-browser-v0.10.2)

### Added

- Allow quick filtering resolved services ([#599](https://github.com/hrzlgnm/mdns-browser/pull/599))

### Changed

- Migrate to opener plugin for opening url ([#602](https://github.com/hrzlgnm/mdns-browser/pull/602))

### Dependencies

- *(deps)* Update rust crate thiserror to v2.0.6 ([#591](https://github.com/hrzlgnm/mdns-browser/pull/591))

- *(deps)* Lock file maintenance ([#592](https://github.com/hrzlgnm/mdns-browser/pull/592))

- *(deps)* Update rust crate tauri-plugin-log to v2.0.4 ([#594](https://github.com/hrzlgnm/mdns-browser/pull/594))

- *(deps)* Update rust crate chrono to v0.4.39 ([#593](https://github.com/hrzlgnm/mdns-browser/pull/593))

- *(deps)* Update rust crate tauri-plugin-log to v2.2.0 ([#598](https://github.com/hrzlgnm/mdns-browser/pull/598))

- *(deps)* Update rust crate tauri-plugin-updater to v2.2.0 ([#601](https://github.com/hrzlgnm/mdns-browser/pull/601))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.2.0 ([#597](https://github.com/hrzlgnm/mdns-browser/pull/597))

## [0.10.1] - 2024-12-08 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.0...mdns-browser-v0.10.1)

### Added

- Browse all if no service is selected ([#586](https://github.com/hrzlgnm/mdns-browser/pull/586))

- Sort resolved services by updated timestamp reverse ([#587](https://github.com/hrzlgnm/mdns-browser/pull/587))

- Allow sorting resolved services by fields ([#589](https://github.com/hrzlgnm/mdns-browser/pull/589))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.23 ([#583](https://github.com/hrzlgnm/mdns-browser/pull/583))

- *(deps)* Update actions/attest-build-provenance action to v2.0.1 ([#585](https://github.com/hrzlgnm/mdns-browser/pull/585))

- *(deps)* Update actions/attest-sbom digest to 34581d8 ([#584](https://github.com/hrzlgnm/mdns-browser/pull/584))

- *(deps)* Update rust crate thiserror to v2.0.5 ([#588](https://github.com/hrzlgnm/mdns-browser/pull/588))

## [0.10.0] - 2024-12-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.10...mdns-browser-v0.10.0)

### Added

- Allow browsing for all found service types ([#575](https://github.com/hrzlgnm/mdns-browser/pull/575))

### Dependencies

- *(deps)* Update rust crate js-sys to v0.3.73 ([#563](https://github.com/hrzlgnm/mdns-browser/pull/563))

- *(deps)* Update rust crate js-sys to v0.3.74 ([#564](https://github.com/hrzlgnm/mdns-browser/pull/564))

- *(deps)* Lock file maintenance ([#567](https://github.com/hrzlgnm/mdns-browser/pull/567))

- *(deps)* Update rust crate tauri-plugin-updater to v2.1.0 ([#568](https://github.com/hrzlgnm/mdns-browser/pull/568))

- *(deps)* Update crate-ci/typos action to v1.28.2 ([#569](https://github.com/hrzlgnm/mdns-browser/pull/569))

- *(deps)* Update rust crate tauri-plugin-log to v2.0.3 ([#570](https://github.com/hrzlgnm/mdns-browser/pull/570))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.16 ([#572](https://github.com/hrzlgnm/mdns-browser/pull/572))

- *(deps)* Update rust crate tokio to v1.42.0 ([#574](https://github.com/hrzlgnm/mdns-browser/pull/574))

- *(deps)* Update rust crate thiserror to v2.0.4 ([#573](https://github.com/hrzlgnm/mdns-browser/pull/573))

- *(deps)* Update rust crate clap to v4.5.22 ([#576](https://github.com/hrzlgnm/mdns-browser/pull/576))

- *(deps)* Update actions/attest-sbom action to v2 ([#578](https://github.com/hrzlgnm/mdns-browser/pull/578))

- *(deps)* Update actions/attest-build-provenance action to v2 ([#577](https://github.com/hrzlgnm/mdns-browser/pull/577))

## [0.9.10] - 2024-11-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.9...mdns-browser-v0.9.10)

### Added

- Show loading animation while verify timeout is not over ([#560](https://github.com/hrzlgnm/mdns-browser/pull/560))

### Dependencies

- *(deps)* Update crate-ci/typos action to v1.28.0 ([#556](https://github.com/hrzlgnm/mdns-browser/pull/556))

- *(deps)* Update crate-ci/typos action to v1.28.1 ([#559](https://github.com/hrzlgnm/mdns-browser/pull/559))

### Fixed

- Pass full instance name correctly to verify ([#557](https://github.com/hrzlgnm/mdns-browser/pull/557))

## [0.9.9] - 2024-11-25 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.8...mdns-browser-v0.9.9)

### Added

- Use material design icons ([#549](https://github.com/hrzlgnm/mdns-browser/pull/549))

- Allow for verifying a service instance ([#550](https://github.com/hrzlgnm/mdns-browser/pull/550))

### Changed

- Use published as trigger for bumping version ([#541](https://github.com/hrzlgnm/mdns-browser/pull/541))

- Pin winget-releaser to v2 ([#542](https://github.com/hrzlgnm/mdns-browser/pull/542))

### Dependencies

- *(deps)* Pin vedantmgoyal9/winget-releaser action to 93fd8b6 ([#543](https://github.com/hrzlgnm/mdns-browser/pull/543))

- *(deps)* Update rust crate serde_json to v1.0.133 ([#544](https://github.com/hrzlgnm/mdns-browser/pull/544))

- *(deps)* Lock file maintenance ([#545](https://github.com/hrzlgnm/mdns-browser/pull/545))

- *(deps)* Update rust crate icondata to 0.5.0 ([#546](https://github.com/hrzlgnm/mdns-browser/pull/546))

- *(deps)* Update anchore/sbom-action digest to 55dc4ee ([#551](https://github.com/hrzlgnm/mdns-browser/pull/551))

- *(deps)* Update dependency cargo-auditable to v0.6.6 ([#552](https://github.com/hrzlgnm/mdns-browser/pull/552))

- *(deps)* Update rust crate mdns-sd to 0.12.0 ([#553](https://github.com/hrzlgnm/mdns-browser/pull/553))

- *(deps)* Lock file maintenance ([#554](https://github.com/hrzlgnm/mdns-browser/pull/554))

## [0.9.8] - 2024-11-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.7...mdns-browser-v0.9.8)

### Changed

- Drop rust caches as those cause building issues ([#534](https://github.com/hrzlgnm/mdns-browser/pull/534))

- Sign windows bundle with a self signed cert ([#538](https://github.com/hrzlgnm/mdns-browser/pull/538))

### Dependencies

- *(deps)* Update rust crate serde to v1.0.215 ([#535](https://github.com/hrzlgnm/mdns-browser/pull/535))

- *(deps)* Update rust crate clap to v4.5.21 ([#537](https://github.com/hrzlgnm/mdns-browser/pull/537))

## [0.9.7] - 2024-11-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.5...mdns-browser-v0.9.7)

### Changed

- Refactor to use expect instead of unwrap ([#486](https://github.com/hrzlgnm/mdns-browser/pull/486))

- Sign Commits of Bump Version PRs ([#487](https://github.com/hrzlgnm/mdns-browser/pull/487))

- Properly Sign Commits in Bump Version PRs ([#489](https://github.com/hrzlgnm/mdns-browser/pull/489))

- Allow for Running Release Drafter Manually ([#491](https://github.com/hrzlgnm/mdns-browser/pull/491))

- Auto Merge Version Bump PRs ([#493](https://github.com/hrzlgnm/mdns-browser/pull/493))

- Set Labels in Bump Version PRs

- Bump Version ([#496](https://github.com/hrzlgnm/mdns-browser/pull/496))

- Use PAT in Auto Bump PRs ([#498](https://github.com/hrzlgnm/mdns-browser/pull/498))

- Back to 0.9.6 ([#500](https://github.com/hrzlgnm/mdns-browser/pull/500))

- Exclude Labels in Releases ([#501](https://github.com/hrzlgnm/mdns-browser/pull/501))

- No Auto Merge for Auto Bump PRs ([#503](https://github.com/hrzlgnm/mdns-browser/pull/503))

- Retry Workflows on Failure in Publish or Builds on Branch main ([#505](https://github.com/hrzlgnm/mdns-browser/pull/505))

- Tweak Release Drafter Settings ([#506](https://github.com/hrzlgnm/mdns-browser/pull/506))

- Align the Android Artifact Naming to Other Artifacts ([#514](https://github.com/hrzlgnm/mdns-browser/pull/514))

- Add screenshots ([#516](https://github.com/hrzlgnm/mdns-browser/pull/516))

- Align wasm opt level with cargo

- Bump crate thiserror to v2.0.0 ([#523](https://github.com/hrzlgnm/mdns-browser/pull/523))

### Dependencies

- *(deps)* Pin peter-evans/enable-pull-request-automerge action to a660677 ([#494](https://github.com/hrzlgnm/mdns-browser/pull/494))

- *(deps)* Update rust crate thiserror to v1.0.67 ([#507](https://github.com/hrzlgnm/mdns-browser/pull/507))

- *(deps)* Lock file maintenance ([#508](https://github.com/hrzlgnm/mdns-browser/pull/508))

- *(deps)* Update rust crate thiserror to v1.0.68 ([#510](https://github.com/hrzlgnm/mdns-browser/pull/510))

- *(deps)* Update anchore/sbom-action digest to fc46e51 ([#511](https://github.com/hrzlgnm/mdns-browser/pull/511))

- *(deps)* Update actions/attest-build-provenance action to v1.4.4 ([#512](https://github.com/hrzlgnm/mdns-browser/pull/512))

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.0.2 ([#513](https://github.com/hrzlgnm/mdns-browser/pull/513))

- *(deps)* Update rust crate tauri-plugin-log to v2.0.2 ([#515](https://github.com/hrzlgnm/mdns-browser/pull/515))

- *(deps)* Update crate-ci/typos action to v1.27.1 ([#517](https://github.com/hrzlgnm/mdns-browser/pull/517))

- *(deps)* Update dependency trunk to v0.21.3 ([#518](https://github.com/hrzlgnm/mdns-browser/pull/518))

- *(deps)* Update crate-ci/typos action to v1.27.2 ([#519](https://github.com/hrzlgnm/mdns-browser/pull/519))

- *(deps)* Update dependency trunk to v0.21.4 ([#521](https://github.com/hrzlgnm/mdns-browser/pull/521))

- *(deps)* Update rust crate tokio to v1.41.1 ([#522](https://github.com/hrzlgnm/mdns-browser/pull/522))

- *(deps)* Update crate-ci/typos action to v1.27.3 ([#524](https://github.com/hrzlgnm/mdns-browser/pull/524))

- *(deps)* Update rust crate thiserror to v2.0.1 ([#525](https://github.com/hrzlgnm/mdns-browser/pull/525))

- *(deps)* Update tauri monorepo ([#526](https://github.com/hrzlgnm/mdns-browser/pull/526))

- *(deps)* Update rust crate thiserror to v2.0.2 ([#528](https://github.com/hrzlgnm/mdns-browser/pull/528))

- *(deps)* Update android-actions/setup-android digest to 9fc6c4e ([#527](https://github.com/hrzlgnm/mdns-browser/pull/527))

- *(deps)* Update rust crate thiserror to v2.0.3 ([#529](https://github.com/hrzlgnm/mdns-browser/pull/529))

- *(deps)* Update dependency cargo-auditable to v0.6.5 ([#531](https://github.com/hrzlgnm/mdns-browser/pull/531))

- *(deps)* Update rust crate tauri to v2.1.1 ([#532](https://github.com/hrzlgnm/mdns-browser/pull/532))

- *(deps)* Lock file maintenance ([#530](https://github.com/hrzlgnm/mdns-browser/pull/530))

## [0.9.5] - 2024-11-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.4...mdns-browser-v0.9.5)

### Added

- Improve workflow when stopping a browse ([#483](https://github.com/hrzlgnm/mdns-browser/pull/483))

### Changed

- Treat warnings as errors, remove unused crate, fix warning about unused import ([#480](https://github.com/hrzlgnm/mdns-browser/pull/480))

- Use typos-action for checking spelling mistakes ([#481](https://github.com/hrzlgnm/mdns-browser/pull/481))

- Move treat warnings as error flag to config ([#484](https://github.com/hrzlgnm/mdns-browser/pull/484))

### Dependencies

- *(deps)* Update softprops/action-gh-release digest to e7a8f85 ([#478](https://github.com/hrzlgnm/mdns-browser/pull/478))

- *(deps)* Update rust crate thiserror to v1.0.66 ([#479](https://github.com/hrzlgnm/mdns-browser/pull/479))

- *(deps)* Pin crate-ci/typos action to d01f29c ([#482](https://github.com/hrzlgnm/mdns-browser/pull/482))

## [0.9.4] - 2024-10-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.3...mdns-browser-v0.9.4)

### Changed

- Fix typo in release drafter template ([#471](https://github.com/hrzlgnm/mdns-browser/pull/471))

- Auto focus delay 5s to actually show it after 3s ([#473](https://github.com/hrzlgnm/mdns-browser/pull/473))

- Share constants between frontend and backend ([#474](https://github.com/hrzlgnm/mdns-browser/pull/474))

- Share models between backend and frontend ([#475](https://github.com/hrzlgnm/mdns-browser/pull/475))

- Tweak release settings and drop unneeded comments ([#476](https://github.com/hrzlgnm/mdns-browser/pull/476))

### Dependencies

- *(deps)* Update dependency trunk to v0.21.2 ([#472](https://github.com/hrzlgnm/mdns-browser/pull/472))

## [0.9.3] - 2024-10-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.2...mdns-browser-v0.9.3)

### Changed

- Enable debug log in before dev command ([#464](https://github.com/hrzlgnm/mdns-browser/pull/464))

- Increase splash screen time to 2s and message delay by roughly… ([#465](https://github.com/hrzlgnm/mdns-browser/pull/465))

### Dependencies

- *(deps)* Lock file maintenance ([#462](https://github.com/hrzlgnm/mdns-browser/pull/462))

- *(deps)* Lock file maintenance ([#467](https://github.com/hrzlgnm/mdns-browser/pull/467))

- *(deps)* Update rust crate serde to v1.0.214 ([#468](https://github.com/hrzlgnm/mdns-browser/pull/468))

- *(deps)* Update anchore/sbom-action digest to 251a468 ([#469](https://github.com/hrzlgnm/mdns-browser/pull/469))

### Fixed

- Attempt to fix loading ends up with a blank/white screen ([#466](https://github.com/hrzlgnm/mdns-browser/pull/466))

## [0.9.2] - 2024-10-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.1...mdns-browser-v0.9.2)

### Added

- Tweak metrics send interval to 3 seconds ([#460](https://github.com/hrzlgnm/mdns-browser/pull/460))

### Changed

- Enable signoff of version bump commits ([#450](https://github.com/hrzlgnm/mdns-browser/pull/450))

- Remap path prefix for reproducible builds ([#451](https://github.com/hrzlgnm/mdns-browser/pull/451))

- Add missing build dependency ([#454](https://github.com/hrzlgnm/mdns-browser/pull/454))

- Add privacy statement ([#457](https://github.com/hrzlgnm/mdns-browser/pull/457))

- Add link to privacy statement ([#458](https://github.com/hrzlgnm/mdns-browser/pull/458))

### Dependencies

- *(deps)* Update actions/setup-java digest to 8df1039 ([#452](https://github.com/hrzlgnm/mdns-browser/pull/452))

### Fixed

- Workaround  window sometimes only showing a white background ([#459](https://github.com/hrzlgnm/mdns-browser/pull/459))

## [0.9.1] - 2024-10-23 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.0...mdns-browser-v0.9.1)

### Added

- Migrate snapcraft spec to own repo ([#429](https://github.com/hrzlgnm/mdns-browser/pull/429))

### Changed

- *(snap)* Build from deb

- *(snap)* Specify stage packages

- *(snap)* Remove layout specficiation

- *(snap)* Fix version and icon

- *(snap)* Add icon, and fix other properties

- *(snap)* Correct license and set website

- *(docs)* Add snap

- Drop snap ([#431](https://github.com/hrzlgnm/mdns-browser/pull/431))

- Overhaul readme and building instructions ([#432](https://github.com/hrzlgnm/mdns-browser/pull/432))

- Label docs/ branches as documentation and add those to own sec… ([#439](https://github.com/hrzlgnm/mdns-browser/pull/439))

- Tune pr labeler config ([#445](https://github.com/hrzlgnm/mdns-browser/pull/445))

### Dependencies

- *(deps)* Lock file maintenance ([#426](https://github.com/hrzlgnm/mdns-browser/pull/426))

- *(deps)* Update tauri monorepo

- *(deps)* Update anchore/sbom-action digest to 1ca97d9 ([#430](https://github.com/hrzlgnm/mdns-browser/pull/430))

- *(deps)* Update rust crate serde to v1.0.211

- *(deps)* Update rust crate tokio to v1.41.0

- *(deps)* Update rust crate serde to v1.0.212

- *(deps)* Update rust crate thiserror to v1.0.65 ([#440](https://github.com/hrzlgnm/mdns-browser/pull/440))

- *(deps)* Update rust crate serde to v1.0.213 ([#441](https://github.com/hrzlgnm/mdns-browser/pull/441))

- *(deps)* Update actions/checkout action to v4.2.2 ([#446](https://github.com/hrzlgnm/mdns-browser/pull/446))

- *(deps)* Update actions/checkout digest to 11bd719 ([#447](https://github.com/hrzlgnm/mdns-browser/pull/447))

### Fixed

- Move copy to clipboard button to front of text ([#437](https://github.com/hrzlgnm/mdns-browser/pull/437))

- Keep metrics sorted by name ([#444](https://github.com/hrzlgnm/mdns-browser/pull/444))

- Add feedback to the user in case no update is available ([#448](https://github.com/hrzlgnm/mdns-browser/pull/448))

### Maintenance

- *(ci)* Allow dispatching auto bump pr workflow

- *(ci)* Auto bump version on prerelease as releases are now drafte… ([#424](https://github.com/hrzlgnm/mdns-browser/pull/424))

## [0.9.0] - 2024-10-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.13...mdns-browser-v0.9.0)

### Added

- Package license file ([#407](https://github.com/hrzlgnm/mdns-browser/pull/407))

- Drop appimage bundle and v1 compatible updater ([#409](https://github.com/hrzlgnm/mdns-browser/pull/409))

- Add bundling of rpm ([#412](https://github.com/hrzlgnm/mdns-browser/pull/412))

- Don show check for update if auto update is not supported ([#414](https://github.com/hrzlgnm/mdns-browser/pull/414))

### Changed

- *(snap)* Add snapcraft.yaml

- *(snap)* Ensure proper rust toolchain is installed

- *(snap)* Set architectures to amd64 only

- *(snap)* More explicit architecture specification

- *(snap)* Use core24 and set platforms to amd64 only

- *(snap)* Fix architecture spelling

- *(snap)* Fix typo when building the bundle

- *(snap)* Build the bundle for real this time

- *(snap)* Pick up the deb from the correct directory

- Tweak pr labeler and release drafter settings

### Dependencies

- *(deps)* Update rust crate serde_json to v1.0.130

- *(deps)* Pin dependencies ([#399](https://github.com/hrzlgnm/mdns-browser/pull/399))

- *(deps)* Update rust crate serde_json to v1.0.131

- *(deps)* Update dependency cargo-make to v0.37.23

- *(deps)* Update rust crate serde_json to v1.0.132

- *(deps)* Update rust crate tauri-plugin-shell to v2.0.2

- *(deps)* Update rust crate tauri to v2.0.5

### Maintenance

- *(ci)* Add release drafter workflow ([#398](https://github.com/hrzlgnm/mdns-browser/pull/398))

- *(ci)* Draft prereleases ([#400](https://github.com/hrzlgnm/mdns-browser/pull/400))

- *(ci)* Release drafter things ([#401](https://github.com/hrzlgnm/mdns-browser/pull/401))

- *(ci)* Fix release drafter permissions ([#402](https://github.com/hrzlgnm/mdns-browser/pull/402))

- *(ci)* Remove include labels spec ([#403](https://github.com/hrzlgnm/mdns-browser/pull/403))

- *(ci)* Pr-labeler

- *(ci)* Fix pr labeling and cleanup release drafting ([#416](https://github.com/hrzlgnm/mdns-browser/pull/416))

- *(ci)* Use label "bug" for fixed bugs when drafting releases ([#417](https://github.com/hrzlgnm/mdns-browser/pull/417))

- *(ci)* Tweak release drafter settings ([#418](https://github.com/hrzlgnm/mdns-browser/pull/418))

- *(ci)* Add fixed sections to release draft ([#419](https://github.com/hrzlgnm/mdns-browser/pull/419))

- *(ci)* Add escaping for fixed sections in release drafter ([#420](https://github.com/hrzlgnm/mdns-browser/pull/420))

- *(ci)* Tweak release drafter ([#421](https://github.com/hrzlgnm/mdns-browser/pull/421))

- *(ci)* Add missing new line to footer ([#422](https://github.com/hrzlgnm/mdns-browser/pull/422))

## [0.8.13] - 2024-10-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.12...mdns-browser-v0.8.13)

### Changed

- *(templates)* Steal release drafter template ([#358](https://github.com/hrzlgnm/mdns-browser/pull/358))

- *(renovate)* Downgrade tauri-cli for testing whether renovate custom manager works

- *(trunk)* Address deprecation of serve.address in Trunk.toml ([#393](https://github.com/hrzlgnm/mdns-browser/pull/393))

### Dependencies

- *(deps)* Update actions/upload-artifact action to v4.4.1

- *(deps)* Update actions/checkout digest to eef6144 ([#350](https://github.com/hrzlgnm/mdns-browser/pull/350))

- *(deps)* Update rust crate tauri to v2.0.2

- *(deps)* Update rust crate clap to v4.5.20

- *(deps)* Update actions/upload-artifact action to v4.4.2

- *(deps)* Update actions/upload-artifact action to v4.4.3

- *(deps)* Update rust crate js-sys to v0.3.71

- *(deps)* Update rust crate js-sys to v0.3.72

- *(deps)* Update swatinem/rust-cache action to v2.7.5

- *(deps)* Update anchore/sbom-action digest to f5e124a ([#361](https://github.com/hrzlgnm/mdns-browser/pull/361))

- *(deps)* Update rust crate tauri to v2.0.3

- *(deps)* Bump tauri cli to 2.0.3 ([#367](https://github.com/hrzlgnm/mdns-browser/pull/367)) ([#364](https://github.com/hrzlgnm/mdns-browser/pull/364))

- *(deps)* Lock file maintenance ([#371](https://github.com/hrzlgnm/mdns-browser/pull/371))

- *(deps)* Update rust crate tauri to v2.0.4

- *(deps)* Update anchore/sbom-action digest to 8d0a650 ([#373](https://github.com/hrzlgnm/mdns-browser/pull/373))

- *(deps)* Update dependency cargo-make to v0.37.22

- *(deps)* Update dependency tauri-cli to v2.0.3 ([#377](https://github.com/hrzlgnm/mdns-browser/pull/377))

- *(deps)* Update dependency trunk to v0.21.1

- *(deps)* Pin actions/github-script action to 60a0d83 ([#387](https://github.com/hrzlgnm/mdns-browser/pull/387))

- *(deps)* Update rust crate serde_json to v1.0.129

### Fixed

- *(renovate)* Fix current value template ([#369](https://github.com/hrzlgnm/mdns-browser/pull/369))

- *(renovate)* Use matchStringStrategy any ([#383](https://github.com/hrzlgnm/mdns-browser/pull/383))

- *(ui)* Disable copy button for when a resolved record dies ([#392](https://github.com/hrzlgnm/mdns-browser/pull/392))

### Maintenance

- *(ci)* Set TAURI_ANDROID_PACKAGE_UNESCAPED ([#364](https://github.com/hrzlgnm/mdns-browser/pull/364))

- *(ci)* Build android on pushes to main and upload apk artifact ([#365](https://github.com/hrzlgnm/mdns-browser/pull/365))

- *(ci)* Always sign apk and upload as artifact if not publishing ([#366](https://github.com/hrzlgnm/mdns-browser/pull/366))

- *(ci)* Binstall trunk on windows instead of building ([#381](https://github.com/hrzlgnm/mdns-browser/pull/381))

- *(ci)* Install openssl via vpkg on windows ([#384](https://github.com/hrzlgnm/mdns-browser/pull/384))

- *(ci)* Tweak job names to be more concise and add leptosfmt check ([#385](https://github.com/hrzlgnm/mdns-browser/pull/385))

- *(ci)* Emojify step names ([#388](https://github.com/hrzlgnm/mdns-browser/pull/388))

## [0.8.12] - 2024-10-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.11...mdns-browser-v0.8.12)

### Added

- *(updater)* Install updates on demand only ([#343](https://github.com/hrzlgnm/mdns-browser/pull/343))

### Changed

- *(build-deps)* Update build dependencies ([#348](https://github.com/hrzlgnm/mdns-browser/pull/348))

### Dependencies

- *(deps)* Lock file maintenance ([#344](https://github.com/hrzlgnm/mdns-browser/pull/344))

## [0.8.11] - 2024-10-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.10...mdns-browser-v0.8.11)

### Dependencies

- *(deps)* Update rust crate futures to v0.3.31

### Maintenance

- *(ci)* Fix android app artifact naming ([#339](https://github.com/hrzlgnm/mdns-browser/pull/339))

- *(ci)* Establish rust cache in android builds ([#340](https://github.com/hrzlgnm/mdns-browser/pull/340))

## [0.8.10] - 2024-10-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.9...mdns-browser-v0.8.10)

### Maintenance

- *(ci)* Fix naming the android published artifact ([#336](https://github.com/hrzlgnm/mdns-browser/pull/336))

## [0.8.9] - 2024-10-05 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.8...mdns-browser-v0.8.9)

### Added

- *(open)* Use shell plugin with strict permissions for open ([#333](https://github.com/hrzlgnm/mdns-browser/pull/333))

### Maintenance

- *(ci)* Attest build provenance of android package ([#331](https://github.com/hrzlgnm/mdns-browser/pull/331))

## [0.8.8] - 2024-10-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.7...mdns-browser-v0.8.8)

### Changed

- *(icon)* Update icons using cargo tauri icon ([#321](https://github.com/hrzlgnm/mdns-browser/pull/321))

### Dependencies

- *(deps)* Pin swatinem/rust-cache action to 23bce25 ([#319](https://github.com/hrzlgnm/mdns-browser/pull/319))

- *(deps)* Update softprops/action-gh-release action to v2 ([#324](https://github.com/hrzlgnm/mdns-browser/pull/324))

- *(deps)* Pin dependencies ([#323](https://github.com/hrzlgnm/mdns-browser/pull/323))

### Fixed

- *(ui)* Fix layout issues on android ([#326](https://github.com/hrzlgnm/mdns-browser/pull/326))

### Maintenance

- *(ci)* Fix SBOM attestation ([#316](https://github.com/hrzlgnm/mdns-browser/pull/316))

- *(ci)* Cleanup naming in workflows ([#317](https://github.com/hrzlgnm/mdns-browser/pull/317))

- *(ci)* Establish rust cache ([#318](https://github.com/hrzlgnm/mdns-browser/pull/318))

- *(ci)* Drop dependabot ([#320](https://github.com/hrzlgnm/mdns-browser/pull/320))

- *(ci)* Add android build ([#322](https://github.com/hrzlgnm/mdns-browser/pull/322))

- *(ci)* Only save cache if running on main branch ([#325](https://github.com/hrzlgnm/mdns-browser/pull/325))

- *(ci)* Add apk signing and android publish workflow ([#328](https://github.com/hrzlgnm/mdns-browser/pull/328))

## [0.8.7] - 2024-10-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.6...mdns-browser-v0.8.7)

### Dependencies

- *(deps)* Pin dependencies ([#312](https://github.com/hrzlgnm/mdns-browser/pull/312))

### Maintenance

- *(ci)* Pass releaseName in publish workflow ([#314](https://github.com/hrzlgnm/mdns-browser/pull/314))

- *(ci)* Fix typo in reusable workflow

## [0.8.6] - 2024-10-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.5...mdns-browser-v0.8.6)

### Changed

- *(attest)* Introduce signed sbom ([#311](https://github.com/hrzlgnm/mdns-browser/pull/311))

### Maintenance

- *(ci)* Introduce reusable workflows ([#309](https://github.com/hrzlgnm/mdns-browser/pull/309))

- *(ci)* Fix uploading pr artifacts ([#310](https://github.com/hrzlgnm/mdns-browser/pull/310))

## [0.8.5] - 2024-10-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.4...mdns-browser-v0.8.5)

### Dependencies

- *(deps)* Update rust crate tauri-plugin-clipboard-manager to v2.0.1

- *(deps)* Lock file maintenance ([#307](https://github.com/hrzlgnm/mdns-browser/pull/307))

## [0.8.4] - 2024-10-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.3...mdns-browser-v0.8.4)

### Changed

- *(identifier)* Use identifier matching the github repository ([#303](https://github.com/hrzlgnm/mdns-browser/pull/303))

## [0.8.3] - 2024-10-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.2...mdns-browser-v0.8.3)

### Fixed

- *(conf)* Fix typo in product name ([#301](https://github.com/hrzlgnm/mdns-browser/pull/301))

## [0.8.2] - 2024-10-03 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.1...mdns-browser-v0.8.2)

### Added

- *(ui)* Add copy to clipboard button to texts ([#298](https://github.com/hrzlgnm/mdns-browser/pull/298))

### Dependencies

- *(deps)* Update tauri monorepo to v2.0.1

## [0.8.1] - 2024-10-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.8.0...mdns-browser-v0.8.1)

### Fixed

- *(compat)* Restore identifier < 0.8.0 to avoid issues with updates ([#295](https://github.com/hrzlgnm/mdns-browser/pull/295))

## [0.8.0] - 2024-10-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.17...mdns-browser-v0.8.0)

### Added

- Add updater plugin 2.0.0 support ([#289](https://github.com/hrzlgnm/mdns-browser/pull/289))

### Changed

- *(updater)* Add another fallback url for updates ([#288](https://github.com/hrzlgnm/mdns-browser/pull/288))

### Dependencies

- *(deps)* Update rust crate clap to v4.5.19

- *(deps)* Update tauri-apps/tauri-action action to v0.5.15

- *(deps)* Lock file maintenance ([#287](https://github.com/hrzlgnm/mdns-browser/pull/287))

## [0.7.17] - 2024-09-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.16...mdns-browser-v0.7.17)

### Changed

- *(bump)* Version 0.7.17 ([#280](https://github.com/hrzlgnm/mdns-browser/pull/280))

## [0.7.16] - 2024-09-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.15...mdns-browser-v0.7.16)

### Fixed

- *(attest)* Fix permissions ([#278](https://github.com/hrzlgnm/mdns-browser/pull/278))

## [0.7.15] - 2024-09-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.14...mdns-browser-v0.7.15)

### Added

- Add creation of sbom ([#255](https://github.com/hrzlgnm/mdns-browser/pull/255))

- *(renovate)* Enable custom manager for updating cached install tools ([#271](https://github.com/hrzlgnm/mdns-browser/pull/271))

- *(attest)* Add attestation provenance to releases ([#274](https://github.com/hrzlgnm/mdns-browser/pull/274))

### Changed

- Maintain lock files ([#264](https://github.com/hrzlgnm/mdns-browser/pull/264))

### Dependencies

- *(deps)* Update actions/upload-artifact action to v3.2.1

- *(deps)* Update actions/upload-artifact action to v4 ([#259](https://github.com/hrzlgnm/mdns-browser/pull/259))

- *(deps)* Pin dependencies ([#260](https://github.com/hrzlgnm/mdns-browser/pull/260))

- *(deps)* Update rust crate mdns-sd to v0.11.5

- *(deps)* Lock file maintenance ([#273](https://github.com/hrzlgnm/mdns-browser/pull/273))

- *(deps)* Pin actions/attest-build-provenance action to 1c608d1 ([#276](https://github.com/hrzlgnm/mdns-browser/pull/276))

### Maintenance

- *(ci)* Locked builds

- *(ci)* Auditable builds ([#265](https://github.com/hrzlgnm/mdns-browser/pull/265))

- *(ci)* Eliminate direct usage of rustup ([#266](https://github.com/hrzlgnm/mdns-browser/pull/266))

- *(ci)* Install cached cargo-auditable v0.6.4 ([#270](https://github.com/hrzlgnm/mdns-browser/pull/270))

- *(ci)* Add package versions to tools ([#272](https://github.com/hrzlgnm/mdns-browser/pull/272))

## [0.7.14] - 2024-09-20 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.13...mdns-browser-v0.7.14)

### Added

- *(ui)* Make input field min 320px wide ([#248](https://github.com/hrzlgnm/mdns-browser/pull/248))

- *(about)* Add about with links to github ([#249](https://github.com/hrzlgnm/mdns-browser/pull/249))

### Changed

- *(updater)* Add alternative url due to issues with duckdns ([#239](https://github.com/hrzlgnm/mdns-browser/pull/239))

- Tune renovate settings ([#243](https://github.com/hrzlgnm/mdns-browser/pull/243))

### Dependencies

- *(deps)* Update rust crate thaw_utils to 0.0.6

- *(deps)* Update tauri-apps/tauri-action action to v0.5.14

- *(deps)* Bump dependencies: mdns-sd, leptos, leptos_meta and thaw ([#244](https://github.com/hrzlgnm/mdns-browser/pull/244))

- *(deps)* Sort dependencies to avoid merge conflicts ([#245](https://github.com/hrzlgnm/mdns-browser/pull/245))

- *(deps)* Bump deps of tauri-build and tauri ([#246](https://github.com/hrzlgnm/mdns-browser/pull/246))

## [0.7.13] - 2024-09-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.12...mdns-browser-v0.7.13)

### Added

- Remove service types when those are removed ([#230](https://github.com/hrzlgnm/mdns-browser/pull/230))

- Allow for setting log level filter via cmdline `ez clap` ([#231](https://github.com/hrzlgnm/mdns-browser/pull/231))

### Dependencies

- *(deps)* Use crate mdns-sd 0.11.3 ([#232](https://github.com/hrzlgnm/mdns-browser/pull/232))

- *(deps)* Update tauri-apps/tauri-action action to v0.5.13

- *(deps)* Update peter-evans/create-pull-request action to v7 ([#236](https://github.com/hrzlgnm/mdns-browser/pull/236))

- *(deps)* Bump tauri-build, clap and tauri dependencies ([#237](https://github.com/hrzlgnm/mdns-browser/pull/237))

## [0.7.12] - 2024-08-18 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.11...mdns-browser-v0.7.12)

### Added

- Integrate latest changes of dependency mdns-sd ([#226](https://github.com/hrzlgnm/mdns-browser/pull/226))

### Dependencies

- *(deps)* Bump patch version of dependencies ([#225](https://github.com/hrzlgnm/mdns-browser/pull/225))

## [0.7.11] - 2024-08-13 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.10...mdns-browser-v0.7.11)

### Added

- Set attributes autofocus and autocapitalize to service type input ([#223](https://github.com/hrzlgnm/mdns-browser/pull/223))

- Actually use attr as it supposed to be used :see_no_evil:

## [0.7.10] - 2024-08-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.9...mdns-browser-v0.7.10)

### Added

- Validate service type while user is typing ([#218](https://github.com/hrzlgnm/mdns-browser/pull/218))

## [0.7.9] - 2024-08-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.8...mdns-browser-v0.7.9)

### Added

- Remove annoying popovers ([#216](https://github.com/hrzlgnm/mdns-browser/pull/216))

## [0.7.8] - 2024-08-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.7...mdns-browser-v0.7.8)

### Added

- Move location of popover and trigger it on click ([#214](https://github.com/hrzlgnm/mdns-browser/pull/214))

## [0.7.7] - 2024-08-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.6...mdns-browser-v0.7.7)

### Added

- Tweak layout ([#212](https://github.com/hrzlgnm/mdns-browser/pull/212))

### Dependencies

- *(deps)* Use mdns-sd crate ([#211](https://github.com/hrzlgnm/mdns-browser/pull/211))

## [0.7.6] - 2024-08-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.5...mdns-browser-v0.7.6)

### Added

- Add actual completion of services as one types ([#202](https://github.com/hrzlgnm/mdns-browser/pull/202))

### Changed

- *(docs)* Add acknowledgements

- *(docs)* Fix typo in readme ([#196](https://github.com/hrzlgnm/mdns-browser/pull/196))

### Dependencies

- *(deps)* Use mdns-sd by git rev ([#203](https://github.com/hrzlgnm/mdns-browser/pull/203))

### Maintenance

- *(ci)* Update workflows ([#199](https://github.com/hrzlgnm/mdns-browser/pull/199))

## [0.7.4] - 2024-08-06 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.3...mdns-browser-v0.7.4)

### Fixed

- Use fixed fork of mdns-sd temporarily ([#191](https://github.com/hrzlgnm/mdns-browser/pull/191))

## [0.7.3] - 2024-08-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.2...mdns-browser-v0.7.3)

### Changed

- *(build)* Reduce release binary size ([#189](https://github.com/hrzlgnm/mdns-browser/pull/189))

### Dependencies

- *(deps)* Update tauri-apps/tauri-action action to v0.5.12

## [0.7.2] - 2024-07-30 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.1...mdns-browser-v0.7.2)

### Added

- More pleasant splash screen experience ([#186](https://github.com/hrzlgnm/mdns-browser/pull/186))

## [0.7.1] - 2024-07-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.7.0...mdns-browser-v0.7.1)

### Changed

- *(style)* Use a more prettier font in the splash screen

## [0.7.0] - 2024-07-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.6.3...mdns-browser-v0.7.0)

### Added

- Add splashscreen ([#181](https://github.com/hrzlgnm/mdns-browser/pull/181))

### Changed

- *(docs)* Add winget instructions ([#177](https://github.com/hrzlgnm/mdns-browser/pull/177))

- *(winget)* Use short form

### Dependencies

- *(deps)* Update tauri-apps/tauri-action action to v0.5.7

- *(deps)* Update tauri-apps/tauri-action action to v0.5.8

- *(deps)* Update tauri-apps/tauri-action action to v0.5.9

- *(deps)* Update rust crate thaw_utils to 0.0.5

- *(deps)* Update tauri-apps/tauri-action action to v0.5.10

- *(deps)* Update tauri-apps/tauri-action action to v0.5.11

### Maintenance

- *(ci)* Add workflow to publish to WinGet ([#178](https://github.com/hrzlgnm/mdns-browser/pull/178))

## [0.6.3] - 2024-06-26 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.6.2...mdns-browser-v0.6.3)

### Changed

- *(perf)* Increase send interval for metrics to 10s ([#157](https://github.com/hrzlgnm/mdns-browser/pull/157))

### Dependencies

- *(deps)* Update ad-m/github-push-action action to v0.8.0

- *(deps)* Update actions/checkout action to v4 ([#152](https://github.com/hrzlgnm/mdns-browser/pull/152))

- *(deps)* Update deps ([#155](https://github.com/hrzlgnm/mdns-browser/pull/155))

- *(deps)* Update rust crate thaw_utils to 0.0.4

- *(deps)* Update tauri-apps/tauri-action action to v0.5.4

- *(deps)* Update tauri-apps/tauri-action action to v0.5.5

- *(deps)* Update taiki-e/cache-cargo-install-action action to v2.0.1

- *(deps)* Update tauri-apps/tauri-action action to v0.5.6

- *(deps)* Humps and bumps ([#163](https://github.com/hrzlgnm/mdns-browser/pull/163))

### Fixed

- *(renovate)* Config ([#165](https://github.com/hrzlgnm/mdns-browser/pull/165))

- Handle txt values properly ([#170](https://github.com/hrzlgnm/mdns-browser/pull/170)) ([#169](https://github.com/hrzlgnm/mdns-browser/pull/169))

### Maintenance

- *(ci)* Add auto bump action ([#150](https://github.com/hrzlgnm/mdns-browser/pull/150))

- *(ci)* Auto bump via pr ([#153](https://github.com/hrzlgnm/mdns-browser/pull/153))

- *(ci)* Set permission flags for auto bump pr workflow

- *(ci)* Fix syntax of auto bump pr action

- *(ci)* Update script action and pass output via env

- *(ci)* Auto bump pr set base branch

- *(ci)* Auto bump as pr attempt to fix

- *(ci)* Skip local branch create and push

- *(ci)* Auto bump pr - skip committing changes

- *(ci)* Cleanup auto bump pr workflow

- *(ci)* Codespell ([#166](https://github.com/hrzlgnm/mdns-browser/pull/166))

- *(ci)* Drop cleanup caches workflow ([#167](https://github.com/hrzlgnm/mdns-browser/pull/167))

## [0.6.2] - 2024-05-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.6.1...mdns-browser-v0.6.2)

### Added

- Show removed services visually as dead instead of removing them ([#149](https://github.com/hrzlgnm/mdns-browser/pull/149))

## [0.6.1] - 2024-05-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.6.0...mdns-browser-v0.6.1)

### Fixed

- *(ui)* Remove comment as it appears as a random doc element ([#145](https://github.com/hrzlgnm/mdns-browser/pull/145))

## [0.6.0] - 2024-05-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.5.3...mdns-browser-v0.6.0)

### Added

- Responsive grid layout automatically adjusting columns depending on width ([#144](https://github.com/hrzlgnm/mdns-browser/pull/144))

## [0.5.3] - 2024-05-14 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.5.2...mdns-browser-v0.5.3)

### Added

- Show timestamp in local time ([#139](https://github.com/hrzlgnm/mdns-browser/pull/139))

### Dependencies

- *(deps)* Update crate mdns-sd to 0.11.1 ([#141](https://github.com/hrzlgnm/mdns-browser/pull/141))

## [0.5.2] - 2024-05-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.5.1...mdns-browser-v0.5.2)

### Dependencies

- *(deps)* Update mdns-sd to latest git rev ([#138](https://github.com/hrzlgnm/mdns-browser/pull/138))

## [0.5.1] - 2024-05-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.5.0...mdns-browser-v0.5.1)

### Fixed

- Remove existing service instances before adding new ones ([#137](https://github.com/hrzlgnm/mdns-browser/pull/137))

### Maintenance

- *(ci)* Run rustfmt and clippy only once on macos ([#136](https://github.com/hrzlgnm/mdns-browser/pull/136))

## [0.5.0] - 2024-05-11 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.4.1...mdns-browser-v0.5.0)

### Added

- Open end browse rendering as cards ([#134](https://github.com/hrzlgnm/mdns-browser/pull/134))

- Remove resolve workflow ([#135](https://github.com/hrzlgnm/mdns-browser/pull/135))

## [0.4.1] - 2024-05-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.4.0...mdns-browser-v0.4.1)

### Fixed

- Deduplicate browsed service types

## [0.4.0] - 2024-05-10 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.3.2...mdns-browser-v0.4.0)

### Added

- Add updated at and ttls to resolved service ([#131](https://github.com/hrzlgnm/mdns-browser/pull/131))

- Browse for service types while running ([#132](https://github.com/hrzlgnm/mdns-browser/pull/132))

## [0.3.2] - 2024-05-09 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.3.1...mdns-browser-v0.3.2)

### Added

- Faster metrics updates ([#126](https://github.com/hrzlgnm/mdns-browser/pull/126))

- Render txt in open end browse mode

### Changed

- *(docs)* Where exe?

- Version bump

### Dependencies

- *(deps)* Update deps

## [0.3.1] - 2024-05-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.3.0...mdns-browser-v0.3.1)

### Added

- Add loading suspense and add port to endless browsing output ([#125](https://github.com/hrzlgnm/mdns-browser/pull/125))

## [0.3.0] - 2024-05-07 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.10...mdns-browser-v0.3.0)

### Added

- Mdns sd daemon metrics for nerds ([#117](https://github.com/hrzlgnm/mdns-browser/pull/117))

- Endless browsing with basic resolve and remove support ([#121](https://github.com/hrzlgnm/mdns-browser/pull/121))

### Changed

- Bump version to 0.2.11

- *(clippy)* Fix clippy.empty_docs warnings for components ([#116](https://github.com/hrzlgnm/mdns-browser/pull/116))

- Bump version to 0.3.0 ([#123](https://github.com/hrzlgnm/mdns-browser/pull/123))

### Fixed

- Make resolved records unique by full name and host name ([#120](https://github.com/hrzlgnm/mdns-browser/pull/120))

## [0.2.10] - 2024-05-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.8...mdns-browser-v0.2.10)

### Added

- Overhaul filtering ([#113](https://github.com/hrzlgnm/mdns-browser/pull/113))

### Changed

- *(docs)* Fix and improve documentation regarding building ([#91](https://github.com/hrzlgnm/mdns-browser/pull/91))

- *(cleanup)* Remove some debug log leftovers ([#103](https://github.com/hrzlgnm/mdns-browser/pull/103))

- Bump version to 0.2.10

### Dependencies

- *(deps)* Update rust crate leptos_meta to 0.6.11

- *(deps)* Update rust crate thaw to 0.3.0

- *(deps)* Update rust crate mdns-sd to 0.11.0

- *(deps)* Update taiki-e/cache-cargo-install-action action to v2 ([#96](https://github.com/hrzlgnm/mdns-browser/pull/96))

- *(deps)* Update tauri-app action to v0.5.2 ([#97](https://github.com/hrzlgnm/mdns-browser/pull/97))

- *(deps)* Update rust crate thaw to 0.3.1

- *(deps)* Update tauri-apps/tauri-action action to v0.5.3

- *(deps)* Update rust crate leptos to 0.6.11

- *(deps)* Update rust crate serde-wasm-bindgen to 0.6.5

- *(deps)* Update rust crate serde to 1.0.200

- *(deps)* Update rust-wasm-bindgen monorepo

- *(deps)* Update rust crate serde_json to 1.0.116

- *(deps)* Update tauri monorepo

- *(deps)* Update rust crate thaw to 0.3.1 ([#114](https://github.com/hrzlgnm/mdns-browser/pull/114))

### Maintenance

- *(ci)* Add action to refresh caches

- *(ci)* Add cleanup of caches only used in pull requests

## [0.2.8] - 2024-04-04 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.7...mdns-browser-v0.2.8)

### Added

- Sort resolved entries by instance ([#86](https://github.com/hrzlgnm/mdns-browser/pull/86))

## [0.2.7] - 2024-04-02 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.6...mdns-browser-v0.2.7)

### Added

- *(deps)* Adopt to changes of if_addrs

### Dependencies

- *(deps)* Update rust crate thaw to 0.2.6

- *(deps)* Update rust crate leptos_meta to 0.6.10

- *(deps)* Update rust crate if-addrs to 0.12.0

### Maintenance

- *(ci)* Run clippy and fmt  checks also in src-tauri ([#81](https://github.com/hrzlgnm/mdns-browser/pull/81))

## [0.2.6] - 2024-03-29 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.5...mdns-browser-v0.2.6)

### Added

- Do not filter addresses when no interface is selected ([#80](https://github.com/hrzlgnm/mdns-browser/pull/80))

## [0.2.5] - 2024-03-27 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.4...mdns-browser-v0.2.5)

### Changed

- Setup automerging for minor and patch updates

### Dependencies

- *(deps)* Update rust crate network-interface to 1.1.2 ([#70](https://github.com/hrzlgnm/mdns-browser/pull/70))

### Fixed

- *(renovate)* Fix typo in preset extends

- *(crash)* Avoid endless browsing with empty service type ([#77](https://github.com/hrzlgnm/mdns-browser/pull/77))

## [0.2.4] - 2024-03-25 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.3...mdns-browser-v0.2.4)

### Fixed

- Speedup resolving ([#69](https://github.com/hrzlgnm/mdns-browser/pull/69))

## [0.2.3] - 2024-03-25 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.2...mdns-browser-v0.2.3)

### Changed

- Add clippy and fmt checks to pr builds ([#64](https://github.com/hrzlgnm/mdns-browser/pull/64))

### Dependencies

- *(deps)* Update rust crate thaw to 0.2.5 ([#66](https://github.com/hrzlgnm/mdns-browser/pull/66))

- *(deps)* Update rust crate mdns-sd to 0.10.5 ([#67](https://github.com/hrzlgnm/mdns-browser/pull/67))

### Fixed

- Resolving for services taking longer ([#68](https://github.com/hrzlgnm/mdns-browser/pull/68))

## [0.2.2] - 2024-03-17 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.1...mdns-browser-v0.2.2)

### Added

- Allow filtering of resolved records by network interfaces :sparkles: ([#62](https://github.com/hrzlgnm/mdns-browser/pull/62))

## [0.2.1] - 2024-03-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.2.0...mdns-browser-v0.2.1)

### Fixed

- :sparkles: show friendly name of the network interface on windows ([#61](https://github.com/hrzlgnm/mdns-browser/pull/61))

## [0.2.0] - 2024-03-16 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.1.12...mdns-browser-v0.2.0)

### Added

- :sparkles: allow selection of network interfaces to browse on ([#60](https://github.com/hrzlgnm/mdns-browser/pull/60))

### Changed

- Leptosfmt the source

## [0.1.12] - 2024-03-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.1.11...mdns-browser-v0.1.12)

### Added

- New custom app icon ([#59](https://github.com/hrzlgnm/mdns-browser/pull/59))

## [0.1.11] - 2024-03-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.1.10...mdns-browser-v0.1.11)

### Added

- Nsis is actually better! ([#58](https://github.com/hrzlgnm/mdns-browser/pull/58))

## [0.1.10] - 2024-03-15 [compare](https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.1.9...mdns-browser-v0.1.10)

### Added

- Show version in window title :sparkles: ([#57](https://github.com/hrzlgnm/mdns-browser/pull/57))

### Changed

- Disable nsis bundle ([#56](https://github.com/hrzlgnm/mdns-browser/pull/56))

## [0.1.9] - 2024-03-15

### Added

- Sort resolved addresses in backend ([#33](https://github.com/hrzlgnm/mdns-browser/pull/33))

- Automatically add .local. when resolving when missing ([#34](https://github.com/hrzlgnm/mdns-browser/pull/34))

- Show subtype if present ([#35](https://github.com/hrzlgnm/mdns-browser/pull/35))

- Show txt records ([#36](https://github.com/hrzlgnm/mdns-browser/pull/36))

### Changed

- Update description in package

- Updater kung-foo ([#54](https://github.com/hrzlgnm/mdns-browser/pull/54))


