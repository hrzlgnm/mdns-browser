# AGENTS.md

This document contains guidelines and commands for agentic coding agents working on the mdns-browser repository.

## Project Overview

This is a Tauri desktop application for browsing mDNS services with:
- Rust backend (src-tauri/) using Tauri framework
- Frontend (src/) built with Leptos web framework
- Shared models and constants in crates/
- Targets: Windows, macOS, Linux, Android, iOS

## Architecture

The frontend code in src/ which also includes the crate models in crates/models/src is platform agnostic.
It is forbidden to add #[cfg(windows)] or any other platform flags to that code and tests in that code.

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
cargo nextest run --profile ci --workspace

# Run specific package tests
cargo nextest run -p mdns-browser --profile ci
cargo nextest run -p models --profile ci

# Run a single test
cargo nextest run --profile ci test_name

# Traditional cargo test (fallback)
cargo test -p models -p mdns-browser

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

# Validate renovate configuration
docker run --rm -v "$(pwd):/repo" -w /repo ghcr.io/renovatebot/renovate renovate-config-validator .github/renovate.json5
```

### Lint GitHub Actions workflows and actions
```bash
actionlint .github/workflows/*.yml
```

### Full lint check (as run in CI)
```bash
cargo fmt -- --check && \
cd src-tauri && cargo fmt -- --check
```

```bash
leptosfmt --check src && \
cargo clippy --workspace --tests -- -D warnings && \
cargo nextest run --profile ci --workspace && \
actionlint .github/workflows/*.yml
```

### Development Workflow
```bash
# Start development server
cargo tauri dev

# With custom arguments
cargo tauri dev -- --log-level debug --enable-devtools

```
1. REQUIRED: Create a branch for your changes with an appropriate prefix (e.g., `feat/`, `fix/`, `chore/`, `refactor/`, `docs/`)
2. Make changes to source code
3. Run `cargo fmt` to format code
4. Run `cargo clippy --workspace --tests -- -D warnings` to check for issues
5. Run `cargo nextest run --profile ci -p models -p mdns-browser` to run tests
6. Run `cargo --locked tauri build --no-bundle --no-sign` to build release version
7. Run `cargo clippy --release -workspace --tests -- -D warnings` to ensure no warnings in release
8. Run `actionlint` to check GitHub Actions workflows if modified
9. Run renovate config validator if `.github/renovate.json5` was modified
10. If README.md was updated, update the manpage (`docs/mdns-browser.1`)
11. Commit only when all checks pass
12. After committing, push to the repository and create a pull request if applicable
13. Use conventional commit format (e.g., feat:, fix:, chore:, refactor:, docs:) for commit messages

# Code Style Guidelines

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
├── src/                          # Leptos frontend
│   ├── app/                      # Feature modules
│   └── main.rs                   # Frontend entry point
├── src-tauri/                    # Tauri backend
│   ├── src/                      # Rust backend code
│   ├── tauri.conf.json           # Tauri configuration
│   └── Cargo.toml                # Backend dependencies
├── crates/                       # Shared libraries
│   ├── models/                   # Data structures and validation
│   └── shared_constants/         # Constants shared across crates
│   └── webkit2gtk-nvidia-quirk/  # WebKit2Gtk NVIDIA quirk
├── Trunk.toml                    # Frontend build configuration
├── Cargo.toml                    # Workspace configuration
└── .config/nextest.toml          # Test configuration
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

1. **Create a branch** for your changes (never commit directly to main)
2. **Run the full CI check**:
```bash
cargo fmt -- --check && \
cd src-tauri && cargo fmt -- --check && \
cd .. && \
leptosfmt --check src && \
cargo clippy --workspace --tests -- -D warnings && \
cargo nextest run --profile ci --workspace && \
actionlint .github/workflows/*.yml
```
3. **Use conventional commits**: Follow the conventional commits format (e.g., `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`)

## Before Modifying Workflows or Actions

When changing any GitHub Actions workflows (.github/workflows/*.yml) or actions (.github/actions/*/action.yml):

1. **Run actionlint validation**:
   ```bash
   actionlint .github/workflows/*.yml
   ```

2. **Fix any actionlint issues** before committing:
   - Remove invalid syntax or context usage
   - Ensure proper secret definitions  
   - Follow GitHub Actions best practices
   - Validate workflow structure

3. **Re-run actionlint** to ensure all issues are resolved

This ensures your workflow changes follow GitHub Actions best practices and will execute correctly.

## Common Pitfalls to Avoid

- **Never** use `unsafe` code - this will cause CI to fail
- **Never** add `#[allow(warnings)]` attributes to suppress warnings - fix the underlying issues instead
- **Never** amend commits - commits will be squashed in GitHub, just create a new commit instead
- **Always** format code before committing
- **Always** run clippy and fix warnings (both debug and release)
- **Don't** add dependencies without updating Cargo.toml properly
- **Don't** break the async patterns used throughout the codebase
- **Don't** ignore test failures - all tests must pass
- **Don't** have warnings in release builds - run `cargo clippy --release --workspace --tests -- -D warnings` before committing
