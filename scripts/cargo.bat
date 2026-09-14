@echo off
rem Copyright 2026 hrzlgnm
rem SPDX-License-Identifier: MIT
rem Route `cargo` invocations through cargo-auditable so release binaries built
rem via `pnpm tauri` (which spawns `cargo` itself) stay auditable.
rem Uses where.exe to find the first cargo binary outside this wrapper's own
rem directory: without that skip, `cargo` would resolve back to this script
rem and recurse until the command line overflows. Aborts if none is found.
setlocal enabledelayedexpansion
set "HERE=%~dp0"
set "HERE=%HERE:~0,-1%"
set "REAL="
for /f "delims=" %%C in ('where cargo 2^>nul') do (
  set "CAND=%%C"
  echo "!CAND!" | find /i "%HERE%" >nul
  if errorlevel 1 (
    if not defined REAL set "REAL=%%C"
  )
)
if not defined REAL (
  echo scripts/cargo.bat: no real cargo binary found on PATH 1>&2
  exit /b 1
)
"%REAL%" auditable %*
