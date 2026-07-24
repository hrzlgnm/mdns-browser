# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [1.9.17] - 2026-07-14

### Changed

- feat: add daily retry workflow for failed CI on PRs #2264
- fix: pass branch/number to jq via environment variables #2265
- fix: only rerun failed jobs #2267
- chore: apply clippy fixes #2268
- feat: use retry-failed-ci reusable workflow #2271
- fix: migrate release-drafter to new category syntax #2275
- chore: update workflow name to reflect the actual purpose #2277
- docs: add shell: bash instruction to workflow guidelines in AGENTS.md #2257

## [1.9.15] - 2026-07-05

### Fixed

- fix: preserve release notes from release-drafter when tauri-action uploads artifacts #2253
- fix: use curl instead of gh for fetching release body #2255
- fix: use bash to get current release body step for windows #2256

## [1.9.9] - 2026-06-04

### Changed

- chore: Update AUR builder Docker image to use libsoup3 #2197

## [1.9.8] - 2026-05-18

### Changed

- chore: update msrv to 1.90 #2163

## [1.9.7] - 2026-05-08

### Fixed

- fix: specify repo for gh release download in homebrew workflow #2157

## [1.9.6] - 2026-05-06

### Added

- feat: Add Homebrew tap support #2154

### Changed

- chore(homebrew): Use fine grained token for homebrew updates #2156

### Fixed

- fix: Address review comments missed from #2154 #2155

## [1.9.2] - 2026-04-24

### Security

- GHSA-82j2-j2ch-gfr8 with #2134

## [1.9.0] - 2026-04-17

### Added

- feat: add --no-nvidia-workaround CLI option to disable all NVIDIA workarounds #2126

### Changed

- refactor: simplify ResolvedServiceItem by deriving memos from single try_get #2122

### Fixed

- fix: resolve variable shadowing and replace Memo with Signal::derive #2125

## [1.8.4] - 2026-04-15

### Fixed

- fix: handle disposal gracefully in ResolvedServiceItem component #2113

## [1.8.3] - 2026-04-14

### Changed

- chore(renovate): maintain lock files weekly #2088
- chore(arch-aur): remove unused tools #2095
- chore: switch to actionlint from hrzlgnm/actions #2097
- performance(ci): blazingly fast ⚡ actionlint #2094
- doc: add instructions how to validate renovate config #2099

### Fixed

- fix: ensure url memo tracks reactive field changes #2107

## [1.8.2] - 2026-04-08

### Changed

- refactor: use sysfs instead of udev for GPU detection #2083

## [1.8.1] - 2026-04-07

### Added

- feat: webkit2gtk-nvidia-quirk: check primary gpu #2070

### Changed

- chore: add libudev-dev dependencies to docker containers #2073
- chore: add eudev libudev devel to void package builder #2075

### Fixed

- fix: use ubuntu builder image to ensure build dependencies #2079
- fix: use ubuntu builder image the correct step #2080

## [1.8.0] - 2026-04-06

### Added

- feat: create webkit2gtk-nvidia-quirk crate #2064
- feat: improve webkit2gtk nvidia workaround #2069

### Changed

- docs: add README to webkit2gtk-nvidia-quirk crate #2067
- doc: clarify the issues webkit2gtk has with nvidia drivers #2068

## [1.7.1] - 2026-04-05

### Changed

- chore: cache platform tools #2059
- chore(deps): switch mdns-sd to crates.io version 0.19 #2062

## [1.7.0] - 2026-04-04

### Added

- feat: display network interfaces for IP addresses #2054

### Changed

- chore: enable coderabbit auto-review #2051
- chore: update agent instructions #2052
- chore: update agents #2057

## [1.6.0] - 2026-04-03

### Added

- feat: improve splashscreen to not be timer based anymore #2049
- feat: set mdns ip check interval to 1s #2050

## [1.5.13] - 2026-03-29

### Fixed

- fix: resolve 3 security vulnerabilities in dependencies #2041

## [1.5.12] - 2026-03-11

### Added

- ci: add actionlint validation step to CI workflow #1990

### Changed

- chore: temporary disable sscache to debug #1964 #1965
- feat(ci): extract cargo-edit installation into reusable workflow #1969
- chore: cached install komac using cargo-install #1973
- chore: pass winget token via env to komac #1976
- chore: improve cache-tools job ordering #1978
- chore: Enable SCCACHE action again #1979
- chore: add actionlint and shellcheck to arch aur builder #1991
- chore: don't allow skipping actionlint job #1994
- docs: update agent instructions #1982
- docs: address a leftover review comment #1984

### Fixed

- ci: add permission comments and remove redundant comments from workflows #1985
- fix: use RULESET_ID secret instead of hardcoded ruleset ID #1986
- fix: address issues reported by actionlint #1988

## [1.5.11] - 2026-03-04

### Changed

- chore: consolidate winget jobs to install komac only once #1957

## [1.5.10] - 2026-03-04

### Changed

- chore: consolidate winget jobs to install komac only once #1957

## [1.5.9] - 2026-03-03

### Changed

- chore: increase collapse limit for dependency updates #1949
- chore: self sign macOS bundle #1955

## [1.5.7] - 2026-02-28

### Changed

- chore: remove unused file #1925
- chore(deps): consolidate actions/attest-build-provenance and actions/attest-sbom into actions/attest #1940
- docs: move man page to docs #1924

## [1.5.6] - 2026-02-21

### Added

- feat: add debug symbols attestation and upload for all desktop platforms #1919
- feat: rename debug symbol artifacts #1922

### Changed

- chore: optimize interface filtering #1918
- chore: Remove debug logs for received events #1921
- chore: remove debug logging statements #1923

## [1.5.5] - 2026-02-21

### Added

- feat: add debug symbols attestation and upload for all desktop platforms #1919

### Changed

- chore: optimize interface filtering #1918
- chore: Remove debug logs for received events #1921

## [1.5.4] - 2026-02-21

### Added

- feat: add debug symbols attestation and upload for all desktop platforms #1919

### Changed

- chore: optimize interface filtering #1918

## [1.5.3] - 2026-02-20

### Fixed

- fix: aur deployment key setup #1917

## [1.5.2] - 2026-02-19

### Added

- feat(aur): use plain executable for mdns-browser-bin #1915

### Changed

- chore: update copyright years to 2026 #1906
- chore: make all dependencies workspace dependencies #1910

### Fixed

- fix: upload sbom to release in workflow_call context #1913
- fix: use env vars for secrets in workflows to prevent log exposure #1914

## [1.5.1] - 2026-02-18

### Added

- feat: enable auto updates for bundle types `deb` and `rpm` #1900

### Changed

- chore: update aur templates to remove stripping #1902
- chore: use binary without bundler type in void package #1904
- chore: Update year in LICENSE #1905

## [1.4.0] - 2026-02-11

### Changed

- chore: rename release workflow to be more shorter and concise #1881
- refactor: consolidate release workflows into single unified workflow #1878
- docs: add link to terminal based app #1872
- docs: Fix wording and indentation #1874

### Fixed

- fix: various issues in the new release workflow #1880

## [1.3.2] - 2026-02-08

### Changed

- docs: Add manpage #1866

## [1.3.1] - 2026-02-07

### Changed

- chore: tweak fetch-depth of checkout steps #1857
- chore: Update MSRV to 1.88 to allow for security updates #1861

## [1.3.0] - 2026-02-04

### Added

- feat: Enable full binary stripping #1837

### Changed

- chore: Add AGENTS.md #1833
- chore: Simplify clippy command and also check tests #1834
- chore: Configure version resolver in release drafter #1835
- chore: Remove version extraction step from release drafter #1836
- chore: Remove Release Drafter trigger from bump version workflow #1838
- chore: Add 'enhancement' label to minor version changes #1852

## [1.2.2] - 2026-02-03

### Changed

- chore: Fix workflow typo and improve SSH key handling #1829
- chore: Quote environment vars when writing to github environment #1830
- chore: Ensure new line in AUR deploy key setup #1831
- chore: Increase release optimization to level 'z' and retain debug info #1825

## [1.2.1] - 2026-01-28

### Fixed

- fix: Ignore service type removal events #1828

## [1.2.0] - 2026-01-27

### Fixed

- fix: skip service subtypes in service type enumeration #1824

## [1.1.2] - 2026-01-22

### Changed

- chore: vendor thaw-ui to be able to update dependencies #1806

## [1.1.0] - 2026-01-16

### Added

- feat: reduce splashscreen duration #1798

## [1.0.7] - 2026-01-04

### Changed

- chore(zed): extend zed tasks #1764

## [1.0.6] - 2025-12-19

### Changed

- docs: make package version links #1748
- docs: make latest releases links, too #1749
- docs: fix spacing in acknowledgment section of README #1753

## [1.0.5] - 2025-12-16

### Changed

- chore(ci): trigger release drafter run after bumping the version #1735
- chore(ci): pass gh token to gh workflow run #1736
- chore(ci): add missing actions write permissions #1737
- chore(ci): draft releases with latest tag reflected in tauri config #1738
- chore(void): refactor void packaging to template generation #1743
- fix(void): use correct version outputs after refactoring #1745

## [1.0.1] - 2025-12-13

### Added

- feat: show additional IP addresses hint #1734

### Changed

- chore(ci): migrate to maitained actions/labeler #1726

## [1.0.0] - 2025-12-06

### Changed

- chore(ci): update schedule to rerun CI to the middle of the month #1712
- chore(renovate): maintain lock files monthly #1721
- docs: add winget version badge #1705
- docs: add AUR version badge #1714

## [0.28.1] - 2025-12-01

### Changed

- chore(void): Use tauri --no-sign option instead of swallowing failures #1690
- chore(copyright): Add license headers #1691
- chore(ci): don't create and scan SBOM when not publishing #1699
- chore: tweak coderabbit settings #1702
- chore(ci): Fix missing sbom for android on publish #1703
- refactor: move local crates into directory `crates` #1697
- refactor: move packaging related directories to `packaging` #1700
- docs: move screenshots to docs/assets #1701

### Fixed

- Revert "fix(aur): Add workaround for `--no-sign` not working as expected" #1689

## [0.28.0] - 2025-11-30

### Changed

- chore(void): Use tauri --no-sign option instead of swallowing failures #1690
- chore(copyright): Add license headers #1691
- chore(ci): don't create and scan SBOM when not publishing #1699
- chore: tweak coderabbit settings #1702
- refactor: move local crates into directory `crates` #1697
- refactor: move packaging related directories to `packaging` #1700
- docs: move screenshots to docs/assets #1701

### Fixed

- Revert "fix(aur): Add workaround for `--no-sign` not working as expected" #1689

## [0.27.8] - 2025-11-29

### Fixed

- fix(aur): Add workaround for `--no-sign` not working as expected #1687

## [0.27.7] - 2025-11-29

### Changed

- chore(ci): Update release drafter template and pr-labeler settings #1662
- chore(ci): Demote full changelog heading in release-drafter template #1663
- chore(ci): Align release drafter template with tagging schema #1664
- chore(ci): Rename reusable workflows for better clarity #1661
- chore: Add feature request issue template #1665
- chore(ci): Update rust-cache action configuration #1666
- chore(ci): Sync our winget-pkgs fork before updating #1667
- chore(ci): Refactor lint workflow into rustfmt and leptosfmt jobs #1669
- chore(aur): Disable signing when building the bundle artifacts #1670
- chore(doc): Update license year #1671
- chore: Add license headers #1674
- chore(ci): Handle removed or renamed Dockerfiles properly #1675
- chore(ci): Externalize docker workflow #1677
- chore: Tweak typos settings #1679
- chore(ci): Replace local typos workflow with external action #1680
- chore: Hack to not ignore `.github/` directory #1683

## [0.27.6] - 2025-11-24

### Changed

- #1641 chore(ci): Enable caching of db when using anchore/scan-action
- #1642 chore(ci): Ignore GHSA-wrw7-89jp-8q8g in grype scanning
- #1644 chore(ci): Consolidate common sbom steps into a composite action
- #1645 chore(ci): Rename desktop sbom step to align with android
- #1646 chore(ci): Cleanup docker build workflow
- #1649 chore(ci): Fix tagging of docker builds and add job names
- #1653 chore(ci): Slimify workflows
- #1655 chore(ci): Refactor to use re-actors/all-green
- #1658 docs: Add badges for license and build status

## [0.27.5] - 2025-11-20

### Changed

- #1638 chore(aur): Use another directory in lint step

## [0.27.4] - 2025-11-20

### Changed

- #1614 chore(winget): Always use latest version of `komac`
- #1617 chore(ci): Run tests with nextest runner
- #1621 chore(aur): Install namcap for linting PKGBUILD
- #1622 chore(aur): Only perform minimal checks in pull requests
- #1624 chore(void): Install xtools for linting void packages
- #1626 chore(void): Only perform minimal checks in pull requests
- #1634 chore(ci): Run CI workflow once per month
- #1635 chore(ci): Optimize change detection to not trigger unrelated jobs

## [0.27.3] - 2025-11-16

### Changed

- #1590 chore(publish): Shorten step names of checksum workflows
- #1591 chore(ci): Fix dependencies after name changes
- #1592 chore(ci): Rename workflow file for better maintainability
- #1593 chore(ci): Consolidate release logic with composite action
- #1595 chore(void): Optimize installation by always updating xbps first
- #1597 chore(void): Attest build provenance when publishing only
- #1599 chrore(ci): Only build platform specific if platform workflow changes
- #1598 chore(ci): Create sbom for android
- #1600 chore(bundler): Enable msi bundling
- #1603 chore(desktop): Prefer NSIS updater in latest.json
- #1605 chore(sbom): Pass name and version to syft via env
- #1608 chore(sbom): Scan SBOM for security vulnerabilities
- #1610 chore(ci): Filter changes more granulary
- #1611 chore(ci): Create SBOM before building

## [0.26.8] - 2025-11-15

### Changed

- #1581 chore(aur): Fix package options
- #1582 chore(ci): Build AUR package on AUR template changes
- #1584 chore(ci): Refactor publishing
- #1585 chore(void): Build Void packages in PR if templates change
- #1586 chore(ci): Fix conflict of asset checksums with source checksums
- #1587 chore(ci): Resolve an artifact name collision

## [0.26.4] - 2025-11-15

### Changed

- #1580 chore(aur): Build mdns-browser without bundling

## [0.26.3] - 2025-11-15

### Changed

- #1571 chore: Update updater endpoints
- #1572 chore(ci): Move android specific steps after rust steps
- #1574 chore(aur): Enable default binary stripping in mdns-browser-bin package
- #1577 chore(ci): Only build changed dockerfiles in pull requests
- #1575 chore(void): Remove already installed packages from dependencies

## [0.25.8] - 2025-11-12

### Changed

- #1540 chore(signing): Rotate signing key
- #1543 chore(ci): Update bump version workflow
- #1550 chore(ci): Refactor CI into a single workflow
- #1555 chore(ci): Always run typos regardless of changes
- #1556 chore(ci): Filter changes more explicitly and fix caching
- #1557 chore(ci): Enable sccache when running clippy
- #1558 chore(ci): Rename publish workflows for better clarity
- #1559 chore(ci): Handle enabling `sccache` correctly
- #1560 chore(ci): Fix passing caching flags
- #1562 chore(ci): Fix conditional `sccache`

## [0.25.3] - 2025-11-07

### Changed

- #1517 chore(ci): Run tests in separate job
- #1518 chore(ci): add comment to schedule
- #1519 chore(ci): Run tests on push
- #1521 chore(ci): Add building and publishing of unbundled executables
- #1525 chore(ci): Use composite actions to deduplicate workflows
- #1527 chore(renovate): Resolve config issues
- #1530 chore(ci): Run in bash shell
- #1536 chore(ci): Clarify naming of step extract android signing key

## [0.25.1] - 2025-10-30

### Changed

- #1473 chore(ci): Remove linting in publish workflow
- #1474 chore(ci): winget: Fix url generation after artifact name change
- #1475 chore(aur): Fixup artifact names in AUR update workflow
- #1480 chore(ci): Update workflows from ubuntu-24.04 to use ubuntu-latest
- #1481 chore(coderabbit): Disable docstring check
- #1484 chore(ci): Add ubuntu builder docker image
- #1487 chore(ubuntu-builder): Add missing dependencies
- #1486 chore(ci): Use ubuntu builder docker image for lint steps
- #1488 chore(ci): Also build docker images in PR on updates
- #1489 chore(ci): Also use ubuntu builder in desktop build workflow
- #1492 chore(renovate): Fix matching and replacing `container_image` in workflows
- #1498 chore(ci): Run clippy with multiple targets
- #1501 chore(ci): consolidate rust caches
- #1502 chore(ocd): reorder matrix definitions and update job names
- #1507 chore(ci): Add unzip to ubuntu builder dependencies

## [0.24.10] - 2025-10-26

### Changed

- #1470 chore(ci): Lint in own separate job
- #1472 chore(ci): Tweak bump version workflow

## [0.24.8] - 2025-10-17

### Changed

- #1450 chore(aur): Disable fail-fast option when publishing to `AUR`
- #1452 chore: Simplify `nvidia` or `nouveau` detection
- #1453 chore(ci): Compress windows debug symbols
- #1454 chore(ci): Don't publish unused sha512 checksums

### Fixed

- #1449 fix(aur): Reenable CFLAGS workaound and extend tests
- #1451 fix: Enable the workaround for nvidia also when running in wayland

## [0.24.4] - 2025-10-09

### Fixed

- #1434 fix(void-packaging): update webkit2gtk dependencies

## [0.24.2] - 2025-09-23

### Fixed

- #1410 fix: Use Memo for dead state with try_get default `true`

## [0.24.0] - 2025-09-10

### Added

- #1392 feat: increase mDNS-Browser window width to 1615

## [0.23.0] - 2025-09-02

### Fixed

- #1377 fix: Ensure details dialog close button is visible with long titles

## [0.22.2] - 2025-08-18

### Added

- #1337 feat: Restructure details dialog: split header, add close button

## [0.22.1] - 2025-08-15

### Added

- #1327 feat: Add BackTop  component, wire into app, add CSS
- #1332 feat: Refactor BackTop scroll handling to be smooth and passive

## [0.21.10] - 2025-08-12

### Added

- #1323 feat: enumerate_mdns_incapable_interfaces: skip loopback on Linux/Windows
- #1326 feat: Add responsive viewport meta tag to index.html

## [0.21.9] - 2025-08-11

### Fixed

- #1315 fix: Tweak layout on mobile so top controls are usable again

## [0.21.8] - 2025-08-10

### Changed

- #1308 chore: Update mdns_sd API usage

## [0.21.7] - 2025-07-29

### Changed

- #1278 chore(renovate): Update Renovate config to set lock file maintenance to weekly
- #1279 chore(ci): Add paths-ignore filters to Android and Desktop GitHub Actions workflows

## [0.21.4] - 2025-07-21

### Changed

- #1270 chore: Set resolve version to 2 in Cargo.toml workspace section
- #1271 chore: Fix typo in Cargo.toml by changing resolve to resolver in workspace section

## [0.21.0] - 2025-07-19

### Added

- #1264 feat: Add sorting by port, and IP to service browsing UI and logic

## [0.20.4] - 2025-07-15

### Added

- #1258 feat: disable mDNS incapable interfaces on `ServiceDaemon`

## [0.20.3] - 2025-07-14

### Fixed

- #1254 fix: Add deduplication of sorted addresses in from_resolved_service_detailed

## [0.20.2] - 2025-07-14

### Added

- #1251 feat: Use new ServiceDetailed API from mdns_sd
- #1252 feat: Add ScopedAddr type and update ResolvedService usage

## [0.20.0] - 2025-07-13

### Added

- #1249 chore: Update mdns-sd dependency to use Git repository

### Changed

- #1187 chore: Update Renovate config to use automerge preset extends and remove explicit settings
- #1189 chore: Reorder extends array entries in .github/renovate.json5 configuration file
- #1191 chore: Remove header comment line about global drop event handling in index.js
- #1194 chore: Update Renovate config for presets, automerge, concurrency, and scheduling
- #1225 chore: Update string formatting to use Rust inline variable interpolation syntax

## [0.19.7] - 2025-06-09

### Fixed

- #1181 fix: Fix drag-and-drop event handling to selectively block non-editable targets to avoid unintended navigation

## [0.19.5] - 2025-06-08

### Added

- #1179 feat: Indicate interactivity on `ThemeSwitcher` icon

### Changed

- #1171 chore(ci): Enable sccache
- #1173 chore: Move caching before building
- #1174 chore: Update workflow to exclude sccache on Windows
- #1176 chore: Update GitHub Actions workflow to restrict specific steps to Ubuntu runners
- #1177 chore: Optimize CI workflows and restrict tests to models package

## [0.19.4] - 2025-06-05

### Changed

- #1168 chore: Remove unused GetUntracked import from Browse component
- #1169 chore: Fix typo in .coderabbit.yaml tone_instructions field
- #1164 refactor: Remove redundant event structs and unify event type definitions
- #1167 refactor: Simplify subscription in event listener functions
- #1166 docs: Move sections about audiable builds and attestation to readme

## [0.19.3] - 2025-06-03

### Changed

- #1163 feat: ignore redundant updates of resolved services

## [0.19.2] - 2025-06-03

### Changed

- #1159 and #1160 chore: only specify up to minor version in dependencies
- #1157 refactor: Refactor event system to use async subscriber closures

### Fixed

- #1162 fix: spelling of browse_types command indroduced in #1157

## [0.19.1] - 2025-06-02

### Changed

- #1149 refactor: event listening system and helpers
- #1152 refactor: Change glob import to specific component import

### Fixed

- #1153 fix: Report an Issue preloads the correct issue template

## [0.19.0] - 2025-05-27

### Added

- #1136 feat: Add `--enable-devtools` CLI argument to enable devtools at startup
- #1138 feat: Disable `Verify`-button of resolved services when not browsing anymore

### Changed

- #1135 refactor: Centralize protocol flag management in a ProtocolFlags component
- #1139 refactor: Refactor protocol flags handling to set entire object and optimize updates

## [0.18.5] - 2025-05-24

### Added

- #1129 feat: Integrate icon rendering into CopyToClipBoardButton
- #1130 feat: Enhance filtering services by their `dead` or `alive` status.

## [0.18.3] - 2025-05-24

### Changed

- #1123 chore: Consolidate shared crates to workspace dependencies
- #1124 chore: remove uuid crate dependency workaround
- #1127 refactor: Simplify layout in ResolvedServiceItem components

### Fixed

- #1125 fix: Make display of table row button texts reactive
- #1126 fix:  Refine UI layout for service status

## [0.18.0] - 2025-05-23

### Added

- #1112 feat: Ensure we are not browsing after a frontend reload
- #1105 feat: Use reactive stores to track resolved service updates
- #1120 feat: Reduce timestamp precision to microseconds
- #1121 feat: Replace `disabled` style by colored status circle icon for removed services

### Changed

- #1111 chore(renovate): remove custom package rules
- #1115 chore: Tweak coderabbit to talk like jblow

## [0.17.1] - 2025-05-19

### Fixed

- #1106 fix: Ensure resolved records are updated

## [0.17.0] - 2025-05-08

### Added

- #1098 feat: allow for switching off usage of IPv4 or IPv6 for browsing

## [0.16.1] - 2025-05-06

### Added

- #1085 feat: start with system theme
- #1095 feat: Use dark theme on mobile platform by default

### Changed

- #1078 refactor: factor out a ThemeSwitcher component
- #1093 refactor: use custom format for log messages
- #1077 refactor: pass value as RwSignal to AutoComplete
- #1081 refactor: move imports and remove redundant cloning
- #1082 refactor: use crate version as single source of truth
- #1083 refactor:  reorganize tauri commands
- #1089 refactor: reorganize commands more
- #1088 refactor: reduce indentation levels in metrics update task
- #1087 refactor: cleanup naming
- #1094 chore: remove unused events from backend

## [0.15.3] - 2025-05-04

### Changed

- #1059 chore: disable trunk version check
- #1060 chore: add bug report issue template
- #1070 chore: remove unused dependency leptos_meta

## [0.15.2] - 2025-04-27

### Changed

- #1052 chore: format css and js code using prettier with tab width of 4

### Fixed

- #1051 bugfix: properly determine the instance name

## [0.15.1] - 2025-04-27

### Changed

- #1046 chore: fix usage of !important in css
- #1044 refactor: deduplicate listening, browsing and table row rendering
- #1049 refactor: state tracking and improve command error handling
- #1047 doc: add doc strings to app/listen.rs module

## [0.14.5] - 2025-04-21

### Changed

- #1025 chore: cleanup workflows
- #1029 refactor: show metrics as grid instead of table

## [0.14.4] - 2025-04-18

### Fixed

- #1024 fix: move counter badge next to stop button

## [0.14.0] - 2025-04-18

### Added

- #1023 feat: add number of displayed / total services badge

### Changed

- #1019 chore(deps): update ghcr.io/hrzlgnm/mdns-browser-void-package-builder:v1 docker digest to fbea29a
- #1020 chore: add sonarqube scanning

## [0.13.2] - 2025-04-16

### Added

- #1009 Improve nvidia detection further

### Changed

- #1008 chore: Bump version workflow creates a signed commit
- #1012 chrore(renovate): tweak renovate settings
- #1013 chore(renovate) Fix renovate settings

### Fixed

- #1016 fix: downgrade crate mdns-sd to v0.13.2 to resolve an issue

## [0.13.1] - 2025-04-14

### Added

- #998 Improve handling of network status changes

### Changed

- #991 chore(ci): Don't ask for confirmation in CI
- #992 chore(ci): Use correct tag_name property from the release event
- #1001 chore(ci): rename sbom artifacts and use platform names
- #1003 chore(ci): Only checksum binaries and source tar balls
- #999 docs: Document new command line options

## [0.13.0] - 2025-04-13

### Added

- #986, #987 Improve `mDNS` capable network detection and logging

### Changed

- #965 chore(attestation): Use correct path in SBOM attestation
- #967 chore(release): publish release assets checksums
- #975 chore(release): publish release checksums for each file separately
- #976 chore(release): publish checksums on released event
- #979 chore(release): add checksums for tarball from tag archive
- #981 chore(ci): add docker image for testing and publishing to AUR
- #980 chore(ci): add workflow for publishing releases to AUR
- #983 chore(actions): use specific version of michidk/run-komac
- #988 chore(version): bump version to 0.13.0

## [0.12.0] - 2025-04-11

### Added

- #963 feat: add `-V, --version` option

### Changed

- #953 chore: fix macOS SBOM attestation
- #958 chore: Add manully triggered bump version workflow
- #954 refactor: subscribe `metrics` once, restart `type` browsing on reload

### Fixed

- #956 fix: improve nvidia detection, add `-d, --disable-dmabuf-renderer` option

## [0.11.31] - 2025-04-07

### Added

- #938 feat: build macos bundle using universal-apple-darwin target

### Changed

- #935 Revert "chore(bundler): enable all desktop targets"

## [0.11.30] - 2025-04-04

### Added

- #932 feat: use async runtime instead of threads

### Changed

- #926 Revert "chore(ci): Enable sccache"
- #928 chore: Use Swatinem/rust-cache for caching

### Fixed

- #930 fix: filter out ipv6 link local addresses for opening `_http._tcp` like services

## [0.11.29] - 2025-04-02

### Added

- #923 feat: Start browsing after selecting a service type and pressing enter

### Changed

- #913 docs: add section about installing on void linux
- #918 docs: add command line options documentation
- #920 docs: update screenshots to contain the new style and layout

### Fixed

- #921 fix(ux): make filtering service types more intuitive

## [0.11.28] - 2025-03-30

### Changed

- chore(void-package): Speed up building by  building build dependencies in docker image
- #905 chore(bundler): enable all desktop targets

## [0.11.20] - 2025-03-29

### Changed

- #881 chore(ci): Enable sccache
- #871 refactor: Make logging a noop in macro log_fn in release build
- #872 refactor: Move inline style to CSS

## [0.11.9] - 2025-03-26

### Added

- #866 feat: Pressing enter in quick filter starts browsing

### Changed

- #865 chore(version): bump version to 0.11.9

## [0.11.8] - 2025-03-26

### Added

- #858 feat: Clear results when starting browsing instead of stopping
- #864 feat: Add support for opening resolved services

### Changed

- #852 chore(version): bump version to 0.11.8

### Fixed

- #861 fix: Make details view scrollable

## [0.11.7] - 2025-03-25

### Added

- #827 feat: add command line option to enable logging to file

### Changed

- #831 chore(debugging): enabled building debug symbols
- #823 refactor: De-duplicate selecting CSS class for desktop/mobile
- #824, #825 refactor: Split components into modules

### Fixed

- #840 fix: Improve error handling to avoid crashing due to panics
- #846, #659  fix: Ignore invalid service types when browsing service types
- #848 fix: Make metrics visible again by not using log_fn! macro, a regression introduced in #824
- #850 Revert "fix(deps): update rust crate mdns-sd to v0.13.4 (#845)"

## [0.11.2] - 2025-03-13

### Added

- #815 feat: update style of splashscreen to match the theme from thaw
- #816 feat: update icon colors to match the theme of thaw

### Changed

- #811 chore(version): bump version to 0.11.2
- #817 refactor: deduplicate size of CopyToClipboardButton with a default

## [0.11.1] - 2025-03-12

### Added

- Update ui frameworks leptos and thaw to a new minor release
- #808 feat: disable any auto-correction attribute along with auto-capitalization
- #809 feat: use a table to visualize the resolved service card

### Changed

- #805 chore(version): bump version to 0.11.1
- #800 refactor: use Signal::get_untracked() in non reactive contexts

## [0.10.12] - 2025-02-19

### Changed

- #722 chore(version): bump version to 0.10.12
- #724 chore: Fully migrate to baptiste0928/cargo-install for installing crates
- #743 chore: discard old drafts

## [0.10.11] - 2025-01-28

### Added

- #721 feat: Build linux bundles with ubuntu-22.04

### Changed

- #720 chore(version): bump version to 0.10.11

## [0.10.10] - 2025-01-28

### Changed

- #706 chore(version): bump version to 0.10.10

### Fixed

- #719 fix: Shorten text in version is already latest

## [0.10.9] - 2025-01-20

### Added

- #704 feat: Browse for added service types while browsing all

### Changed

- #693 chore(version): bump version to 0.10.9

## [0.10.8] - 2025-01-16

### Added

- #692 feat: Add icon for switching between dark and light mode

### Changed

- #642 chore(version): bump version to 0.10.8
- #643 chore: migrate to winget-updater action
- #645 chore: fix dependency in winget publish workflow
- #646 chore: fix versioning in winget publish workflow
- #656 chore: set ws_protocol to "ws" in trunk config
- #678 chore: Fix clippy warning about redundant map_or usage
- #679 chore: Drop Retry on Failure Workflow as it doesn't work as intended
- #676 docs: add badges to readme

## [0.10.7] - 2024-12-18

### Added

- #641 feat: unify handling of trailing dot when copying to clipboard

### Changed

- #634 chore(version): bump version to 0.10.7
- #635 Revert "chore: pin winget-releaser to v2 (#542)"
- #639 chore: migrate to baptiste0928/cargo-install action for installing tools

## [0.10.6] - 2024-12-17

### Added

- #619 feat: also use opener plugin on android
- #625 feat: add toast when text is copied to clipboard and use default cursor
- #627 feat: only show toast about copied text on desktop plaforms

### Changed

- #610 chore(version): bump version to 0.10.5
- #626 chore: install trunk without `evil` features
- #629 chore: ci runs tests only on macos runners
- #630 chore(version): bump version to 0.10.6
- #631 Revert "chore(deps): update tauri-apps/tauri-action action to v0.5.17…

## [0.10.4] - 2024-12-10

### Added

- #608 feat: Disable auto-capitalize attribute in quick-filter input

### Changed

- #603 chore(version): bump version to 0.10.3
- #607 chore(version): bump version to 0.10.4

### Fixed

- #605 fix: Only use opener plugin on desktop platforms

## [0.10.2] - 2024-12-09

### Added

- #599 feat: Allow quick filtering resolved services

### Changed

- #590 chore(version): bump version to 0.10.2
- #602 refactor: migrate to opener plugin for opening url

## [0.10.1] - 2024-12-08

### Added

- #575 feat: Allow browsing for all found service types
- #586 feat: Browse all if no service is selected
- #589 feat: Allow sorting resolved services by fields

### Changed

- #561 chore(version): bump version to 0.9.11
- #562 Revert "chore: Move treat warnings as error flag to config (#484)"
- #579 chore(version): bump version to 0.10.1

## [0.9.10] - 2024-11-26

### Added

- #549 feat: Use material design icons
- #550 feat: Allow for verifying a service instance
- #560 feat: show loading animation while verify timeout is not over

### Changed

- #540 chore(version): bump version to 0.9.9
- #541 chore: Use published as trigger for bumping version
- #542 chore: pin winget-releaser to v2
- #555 chore(version): bump version to 0.9.10

## [0.9.8] - 2024-11-15

### Changed

- #534 chore: drop rust caches as those cause building issuse
- #538 chore: Sign windows bundle with a self signed cert
- #539 chore(version): bump version to 0.9.8

## [0.9.7] - 2024-11-11

### Changed

- #485 chore(version): bump version to 0.9.6
- #487 chore: Sign Commits of Bump Version PRs
- #491 chore: Allow for Running Release Drafter Manually
- #504 chore(version): bump version to 0.9.7
- #505 chore: Retry Workflows on Failure in Publish or Builds on Branch main
- #506 chore: Tweak Release Drafter Settings
- #514 chore: Align the Android Artifact Naming to Other Artifacts
- #486 refactor: Refactor to use expect instead of unwrap
- #516 docs: add screenshots

### Fixed

- #533 x11 workaround: disable dma buf renderer in favor of disabling compositing mode

## [0.9.5] - 2024-11-02

### Added

- #483 feat: Improve workflow when stopping a browse

## [0.9.4] - 2024-10-30

### Changed

- #474 refactor: share constants between backend and frontend
- #475 refactor: Share models between backend and frontend

### Fixed

- #466 fix: attempt to fix loading ends up with a blank/white screen

## [0.9.2] - 2024-10-26

### Added

- #460 feat: Tweak metrics send interval to 3 seconds

### Changed

- #454 docs: Add missing build dependency
- #457 docs: add privacy statement
- #458 docs: Add link to privacy statement

### Fixed

- #459 fix: Workaound  window sometimes only showing a white background

## [0.9.1] - 2024-10-23

### Changed

- #432 docs: overhaul readme and building instructions

### Fixed

- #437 fix: Move copy to clipboard button to front of text
- #444 fix: keep metrics sorted by name
- #448 fix: Add feedback to the user in case no update is available

## [0.9.0] - 2024-10-20

### Added

- #407 feat: package license file
- #409 feat: drop appimage bundle and v1 compatible updater
- #412 feat: add bundling of rpm
- #414 feat: don show check for update if auto update is not supported

### Changed

- #409 feat: drop appimage bundle and v1 compatible updater

### Fixed

- #392 fix(ui): disable copy button for when a resolved record dies

[Unreleased]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.17...HEAD
[1.9.17]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.15...mdns-browser-v1.9.17
[1.9.15]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.9...mdns-browser-v1.9.15
[1.9.9]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.8...mdns-browser-v1.9.9
[1.9.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.7...mdns-browser-v1.9.8
[1.9.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.6...mdns-browser-v1.9.7
[1.9.6]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.2...mdns-browser-v1.9.6
[1.9.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.9.0...mdns-browser-v1.9.2
[1.9.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.4...mdns-browser-v1.9.0
[1.8.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.3...mdns-browser-v1.8.4
[1.8.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.2...mdns-browser-v1.8.3
[1.8.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.1...mdns-browser-v1.8.2
[1.8.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.8.0...mdns-browser-v1.8.1
[1.8.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.7.1...mdns-browser-v1.8.0
[1.7.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.7.0...mdns-browser-v1.7.1
[1.7.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.6.0...mdns-browser-v1.7.0
[1.6.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.13...mdns-browser-v1.6.0
[1.5.13]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.12...mdns-browser-v1.5.13
[1.5.12]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.11...mdns-browser-v1.5.12
[1.5.11]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.10...mdns-browser-v1.5.11
[1.5.10]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.9...mdns-browser-v1.5.10
[1.5.9]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.7...mdns-browser-v1.5.9
[1.5.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.6...mdns-browser-v1.5.7
[1.5.6]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.5...mdns-browser-v1.5.6
[1.5.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.4...mdns-browser-v1.5.5
[1.5.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.3...mdns-browser-v1.5.4
[1.5.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.2...mdns-browser-v1.5.3
[1.5.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.5.1...mdns-browser-v1.5.2
[1.5.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.4.0...mdns-browser-v1.5.1
[1.4.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.2...mdns-browser-v1.4.0
[1.3.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.1...mdns-browser-v1.3.2
[1.3.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.3.0...mdns-browser-v1.3.1
[1.3.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.2...mdns-browser-v1.3.0
[1.2.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.1...mdns-browser-v1.2.2
[1.2.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.2.0...mdns-browser-v1.2.1
[1.2.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.1.2...mdns-browser-v1.2.0
[1.1.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.1.0...mdns-browser-v1.1.2
[1.1.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.7...mdns-browser-v1.1.0
[1.0.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.6...mdns-browser-v1.0.7
[1.0.6]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.5...mdns-browser-v1.0.6
[1.0.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.1...mdns-browser-v1.0.5
[1.0.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v1.0.0...mdns-browser-v1.0.1
[1.0.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.28.1...mdns-browser-v1.0.0
[0.28.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.28.0...mdns-browser-v0.28.1
[0.28.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.8...mdns-browser-v0.28.0
[0.27.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.7...mdns-browser-v0.27.8
[0.27.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.6...mdns-browser-v0.27.7
[0.27.6]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.5...mdns-browser-v0.27.6
[0.27.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.4...mdns-browser-v0.27.5
[0.27.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.27.3...mdns-browser-v0.27.4
[0.27.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.8...mdns-browser-v0.27.3
[0.26.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.4...mdns-browser-v0.26.8
[0.26.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.26.3...mdns-browser-v0.26.4
[0.26.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.8...mdns-browser-v0.26.3
[0.25.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.3...mdns-browser-v0.25.8
[0.25.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.25.1...mdns-browser-v0.25.3
[0.25.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.10...mdns-browser-v0.25.1
[0.24.10]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.8...mdns-browser-v0.24.10
[0.24.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.4...mdns-browser-v0.24.8
[0.24.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.2...mdns-browser-v0.24.4
[0.24.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.24.0...mdns-browser-v0.24.2
[0.24.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.23.0...mdns-browser-v0.24.0
[0.23.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.2...mdns-browser-v0.23.0
[0.22.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.22.1...mdns-browser-v0.22.2
[0.22.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.10...mdns-browser-v0.22.1
[0.21.10]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.9...mdns-browser-v0.21.10
[0.21.9]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.8...mdns-browser-v0.21.9
[0.21.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.7...mdns-browser-v0.21.8
[0.21.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.4...mdns-browser-v0.21.7
[0.21.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.21.0...mdns-browser-v0.21.4
[0.21.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.4...mdns-browser-v0.21.0
[0.20.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.3...mdns-browser-v0.20.4
[0.20.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.2...mdns-browser-v0.20.3
[0.20.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.20.0...mdns-browser-v0.20.2
[0.20.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.7...mdns-browser-v0.20.0
[0.19.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.5...mdns-browser-v0.19.7
[0.19.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.4...mdns-browser-v0.19.5
[0.19.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.3...mdns-browser-v0.19.4
[0.19.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.2...mdns-browser-v0.19.3
[0.19.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.1...mdns-browser-v0.19.2
[0.19.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.19.0...mdns-browser-v0.19.1
[0.19.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.5...mdns-browser-v0.19.0
[0.18.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.3...mdns-browser-v0.18.5
[0.18.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.18.0...mdns-browser-v0.18.3
[0.18.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.17.1...mdns-browser-v0.18.0
[0.17.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.17.0...mdns-browser-v0.17.1
[0.17.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.16.1...mdns-browser-v0.17.0
[0.16.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.3...mdns-browser-v0.16.1
[0.15.3]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.2...mdns-browser-v0.15.3
[0.15.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.15.1...mdns-browser-v0.15.2
[0.15.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.5...mdns-browser-v0.15.1
[0.14.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.4...mdns-browser-v0.14.5
[0.14.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.14.0...mdns-browser-v0.14.4
[0.14.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.2...mdns-browser-v0.14.0
[0.13.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.1...mdns-browser-v0.13.2
[0.13.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.13.0...mdns-browser-v0.13.1
[0.13.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.12.0...mdns-browser-v0.13.0
[0.12.0]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.31...mdns-browser-v0.12.0
[0.11.31]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.30...mdns-browser-v0.11.31
[0.11.30]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.29...mdns-browser-v0.11.30
[0.11.29]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.28...mdns-browser-v0.11.29
[0.11.28]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.20...mdns-browser-v0.11.28
[0.11.20]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.9...mdns-browser-v0.11.20
[0.11.9]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.8...mdns-browser-v0.11.9
[0.11.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.7...mdns-browser-v0.11.8
[0.11.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.2...mdns-browser-v0.11.7
[0.11.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.11.1...mdns-browser-v0.11.2
[0.11.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.12...mdns-browser-v0.11.1
[0.10.12]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.11...mdns-browser-v0.10.12
[0.10.11]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.10...mdns-browser-v0.10.11
[0.10.10]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.9...mdns-browser-v0.10.10
[0.10.9]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.8...mdns-browser-v0.10.9
[0.10.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.7...mdns-browser-v0.10.8
[0.10.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.6...mdns-browser-v0.10.7
[0.10.6]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.4...mdns-browser-v0.10.6
[0.10.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.2...mdns-browser-v0.10.4
[0.10.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.10.1...mdns-browser-v0.10.2
[0.10.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.10...mdns-browser-v0.10.1
[0.9.10]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.8...mdns-browser-v0.9.10
[0.9.8]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.7...mdns-browser-v0.9.8
[0.9.7]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.5...mdns-browser-v0.9.7
[0.9.5]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.4...mdns-browser-v0.9.5
[0.9.4]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.2...mdns-browser-v0.9.4
[0.9.2]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.1...mdns-browser-v0.9.2
[0.9.1]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v0.9.0...mdns-browser-v0.9.1
[0.9.0]: https://github.com/hrzlgnm/mdns-browser/releases/tag/mdns-browser-v0.9.0