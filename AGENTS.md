# AGENTS.md

This document contains guidelines and commands for agentic coding agents working on the mdns-browser repository.

## Project Overview

This is a Tauri desktop application for browsing mDNS services with:
- Rust backend (src-tauri/) using Tauri framework
- Frontend (src/) built with Leptos web framework
- Shared models and constants in crates/
- Targets: Windows, macOS, Linux, Android, iOS

## Essential Commands

### Build Commands
```bash
# Build the entire application (frontend + Tauri app)
cargo --locked tauri build --no-bundle --no-sign

# Development build with hot reload
cargo tauri dev
```

### Test Commands
```bash
# Run all tests using nextest (preferred)
cargo nextest run --profile ci

# Run specific package tests
cargo nextest run -p mdns-browser --profile ci
cargo nextest run -p models --profile ci

# Run a single test
cargo nextest run --profile ci test_name

# Traditional cargo test (fallback)
cargo test -p models  -p mdns-browser

# Run tests with specific features
cargo nextest run --profile ci --features desktop
```

### Lint Commands
```bash
# Format Rust code
cargo fmt

# Format Leptos components
leptosfmt --check src

# Check formatting without modifying
cargo fmt -- --check

# Run clippy lints
cargo clippy --workspace --tests -- -D warnings

# Full lint check (as run in CI)
cargo fmt -- --check && \
leptosfmt --check src && \
cargo clippy --workspace --tests -- -D warnings
```

### Development Workflow
```bash
# Start development server
cargo tauri dev

# With custom arguments
cargo tauri dev -- --log-level debug --enable-devtools
```

## Code Style Guidelines

### File Headers
All source files must include:
```rust
// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0
```

### Rust Code Style
- Use `cargo fmt` for formatting
- Clippy must pass with `--tests -- -D warnings`
- Prefer explicit error handling over `unwrap()`
- Use `Result<T, String>` for Tauri command return types
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use workspace dependencies defined in root Cargo.toml

### Leptos Frontend Style
- Use `leptosfmt` for component formatting
- Components use PascalCase naming
- Files in `src/app/` are organized by feature
- Use leptos `prelude::*` imports consistently
- Prefer `<Show>` over conditional rendering in view! macros
- Use `view! { }` macro for all UI components

### Imports Organization
- Group imports: std, external crates, workspace crates, local modules
- Use `{}` for single-item imports when possible
- Keep imports at file level, not inside functions

### Error Handling
- Tauri commands should return `Result<T, String>`
- Use `map_err()` for error conversion with context
- Log errors with `log::error!()` before propagating
- Use `?` operator for error propagation

### Async Patterns
- Use `tauri::async_runtime::spawn()` for background tasks
- Prefer `recv_async()` for channel operations
- Handle task lifecycle properly (don't forget event listeners)

### State Management
- Use `Arc<Mutex<T>>` for shared state between threads
- Use `AtomicBool` for simple flags with `Ordering::SeqCst`
- Prefer `State<T>` injection for Tauri commands when possible

### Serialization
- All structs crossing the frontend-backend boundary need `#[derive(Serialize, Deserialize)]`
- Use `serde(rename_all = "camelCase")` for frontend compatibility
- Dates use microsecond timestamps with `serde_with::DisplayFromStr`

### Testing Guidelines
- Write unit tests in `#[cfg(test)]` modules
- Use descriptive test names following `test_functionality_scenario` pattern
- Test error cases as well as success cases
- Mock external dependencies when needed

### Platform-Specific Code
- Use `#[cfg(target_os = "...")]` for OS-specific code
- Use `#[cfg(desktop)]` vs `#[cfg(mobile)]` for platform targeting
- Separate platform-specific implementations into submodules

### Documentation
- Document public APIs with rustdoc comments
- Include example usage in complex functions
- Document Tauri command purposes and parameters
- Use `#[deprecated]` for old APIs that must remain

## Project Structure

```text
├── src/                    # Leptos frontend
│   ├── app/               # Feature modules
│   └── main.rs            # Frontend entry point
├── src-tauri/             # Tauri backend
│   ├── src/               # Rust backend code
│   ├── tauri.conf.json    # Tauri configuration
│   └── Cargo.toml         # Backend dependencies
├── crates/                # Shared libraries
│   ├── models/            # Data structures and validation
│   └── shared_constants/  # Constants shared across crates
├── Trunk.toml             # Frontend build configuration
├── Cargo.toml             # Workspace configuration
└── .config/nextest.toml   # Test configuration
```

## Important Notes

- This is a workspace with multiple crates - always run commands from root
- The application uses custom Tauri plugins for system integration
- mDNS functionality uses the `mdns-sd` crate
- Frontend and backend communicate via Tauri events and commands
- The app supports both desktop and mobile platforms
- CI runs on Ubuntu, macOS, and Windows - ensure cross-platform compatibility
- The project uses auditible binaries with `cargo-auditable`

## Before Committing

Always run the full CI check:
```bash
cargo fmt -- --check && \
cd src-tauri && cargo fmt -- --check && \
cd .. && \
leptosfmt --check src && \
cargo clippy --tests -- -D warnings && \
cd src-tauri && cargo clippy --tests -- -D warnings && \
cd .. && \
cargo nextest run --profile ci
```

This ensures your changes follow all project conventions and pass all checks.
