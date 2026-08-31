# AGENTS.md

Guidelines and commands for agentic coding agents working on the mdns-browser repository.

## Project Overview

This is a Tauri desktop application for browsing mDNS services with:
- Rust backend (src-tauri/) using Tauri framework
- Frontend (src/) built with Leptos web framework
- Shared models and constants in crates/
- Targets: Windows, macOS, Linux, Android, iOS

mDNS functionality uses the `mdns-sd` crate. Release binaries are auditable
(`cargo-auditable`). CI runs on Ubuntu, macOS, and Windows - keep changes
cross-platform.

## Architecture

The frontend code in src/ which also includes the crate models in crates/models/src is platform agnostic.
It is forbidden to add #[cfg(windows)] or any other platform flags to that code and tests in that code.

## Essential Commands

This is a workspace with multiple crates - always run commands from the root.

### Build and Run

```bash
# Build the entire application (frontend + Tauri app)
cargo --locked tauri build --no-bundle --no-sign

# Development build with hot reload (optional args after --)
cargo tauri dev
cargo tauri dev -- --log-level debug --enable-devtools
```

### Test

```bash
# Run all tests using nextest (preferred)
cargo nextest run --profile ci --workspace

# Run specific package tests
cargo nextest run -p mdns-browser --profile ci
cargo nextest run -p models --profile ci

# Run a single test
cargo nextest run --profile ci test_name
```

### Format and Lint (individual)

```bash
cargo fmt                                       # format Rust code
leptosfmt src                                   # format Leptos components
cargo clippy --workspace --tests -- -D warnings # lint

# Validate renovate configuration (when .github/renovate.json5 changed)
docker run --rm --volume=$(pwd)/.github/renovate.json5:/github/renovate.json5:ro kokuwaio/renovate-config-validator:latest
```

### Full check (run before every commit)

```bash
cargo fmt -- --check && \
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check && \
leptosfmt --check src && \
cargo clippy --workspace --tests -- -D warnings && \
cargo clippy --release --workspace --tests -- -D warnings && \
cargo nextest run --profile ci --workspace && \
actionlint .github/workflows/*.yml
```

### Task-specific guides

- Android: building the Android app, adding/removing Tauri plugins, or touching `reqwest`/TLS features - read [docs/agents/android.md](docs/agents/android.md) first.
- AUR packaging: testing the source or `-bin` AUR packages - see [docs/agents/aur.md](docs/agents/aur.md).
- GitHub Actions: before modifying `.github/workflows/*.yml` or `.github/actions/*/action.yml`, read [docs/agents/github-actions.md](docs/agents/github-actions.md).

## Development Workflow

1. Create a branch for your changes (see [Git Conventions](#git-conventions))
2. Make your changes
3. Run the [full check](#full-check-run-before-every-commit); also run
   `cargo --locked tauri build --no-bundle --no-sign` to verify the release build
4. Conditional checks: renovate config validator if `.github/renovate.json5`
   changed; if README.md changed, update the manpage (`docs/mdns-browser.1`)
5. Commit only when all checks pass, then push and open a PR (see
   [After Completion](#after-completion))

## Code Style Guidelines

### File Headers

All source files must include:
```rust
// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT
```

### Rust Code Style

- `cargo fmt` applies and checks formatting; no need to review formatting or import style, the formatter covers it
- Prefer explicit error handling over `unwrap()`
- Use workspace dependencies defined in root Cargo.toml
- Keep imports at file level, not inside functions

### Leptos Frontend Style

- Use `leptosfmt` for component formatting
- Files in `src/app/` are organized by feature
- Use leptos `prelude::*` imports consistently
- Prefer `<Show>` over conditional rendering in view! macros
- Use `view! { }` macro for all UI components

### Error Handling

- Tauri commands return `Result<T, String>`
- Use `map_err()` for error conversion with context
- Log errors with `log::error!()` before propagating

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

### Platform-Specific Code

- Use `#[cfg(target_os = "...")]` for OS-specific code
- Use `#[cfg(desktop)]` vs `#[cfg(mobile)]` for platform targeting
- Separate platform-specific implementations into submodules

### Documentation

- Document public APIs (including Tauri commands and their parameters) with rustdoc comments
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
├── crates/                       # Shared libraries and custom Tauri plugins
│   ├── models/                   # Data structures and validation
│   ├── shared_constants/         # Constants shared across crates
│   └── webkit2gtk-nvidia-quirk/  # WebKit2Gtk NVIDIA quirk
├── docs/agents/                  # Task-specific agent guides
├── Trunk.toml                    # Frontend build configuration
├── Cargo.toml                    # Workspace configuration
└── .config/nextest.toml          # Test configuration
```

## Git Conventions

- Conventional commits: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, etc.
- All changes land via pull requests on a branch (direct pushes to `main` are blocked). Create a `feat/...`, `fix/...`, etc. branch and open a PR.

### Tags

- Releases are tagged with `vMAJOR.MINOR.PATCH` (e.g. `v1.18.0`). This is the
  only tag format used since the project switched to it; older
  `mdns-browser-vX.Y.Z` style tags are no longer produced and should be ignored.
- When adding a "added with release" note to docs (e.g. the README option
  sections), link to the matching `vX.Y.Z` tag, not the old prefix format.

### When to commit

- Do not leave completed work uncommitted. Once a logical unit of work is done and the tree is green, commit it — don't wait to be asked. This is a standing authorization: treat every task as implicitly including "and commit your work" unless the user says otherwise.
- Commit as you go, not all at once at the end. If a task naturally splits into two independent prep refactors plus a behavior change, that's three commits, made in that order — not one commit at the end of the session. (Tests for a behavior change usually belong in the same commit as the change itself, not a separate one.)

### How to structure commits

- Prefer a fine-grained commit history. Commits should be as small as possible while still being meaningful and self-contained.
- Every commit must compile and pass all tests. No "WIP" commits, no commits that leave the tree broken and rely on a follow-up to fix it.
- Every commit must pass the [full check](#full-check-run-before-every-commit) — don't introduce a warning in one commit that a later commit (or the user) cleans up.
- Commit messages explain why, not what. The diff already shows what changed; the message should capture the motivation, the constraint, or the bug being fixed. If the reason is obvious from a one-line subject, no body is needed — but never paraphrase the diff.
- Separate preparatory refactorings from behavior changes. If a fix or feature is easier to review after a refactor, land the refactor in its own commit first. Pure refactors must be behavior-preserving.
- Wrap the message body to 72 characters. The subject may go up to 80 characters, or a little more if needed to convey a good single-line summary; the body wraps at 72 exactly.

### Attributing AI usage

- Every commit gets both trailers in a trailer block at the end, after a blank line. Use `--trailer` on the command line so no wrapping or manual formatting is needed:
  - `Co-authored-by: opencode <noreply@opencode.ai>`
  - `Assisted-by: opencode (<model-name>)`
- Trailers are exempt from the 72-character body wrap.
- Never use `--author` or `--committer` for this attribution. The release-notes tooling derives the credited `@username` from the commit author, so doing so would replace the user with the bot throughout the release notes.
- `amend!` commits must repeat both trailers in the replacement message body. The replacement overwrites the target's message wholesale, so omitting them strips attribution from the target when the user folds the amend in with `--autosquash`. Plain `fixup!` commits need no special care: their message is discarded on autosquash and the target keeps its own trailers.

### Iterate with fixup! commits

- When refining work that's already committed — adjusting an approach, incorporating an idea from elsewhere, fixing something that belongs to the same logical unit — create a fixup against the target commit (`git commit --fixup=<sha>`) so it sits alongside its target, ready for the user to fold in later with `git rebase --autosquash`. Don't pile follow-up commits on top with the intent of squashing them later.
- This holds even when the target is HEAD: use `git commit --fixup`, not `git commit --amend`. An `--amend` rewrites the commit on the spot and skips the reviewable checkpoint a fixup provides.
- If the changes don't map cleanly onto existing commits — they cut across several of them, or restructure something at a different layer than any existing commit naturally owns — stop and ask the user how to proceed.
- After writing a fixup, re-read the target commit's message. If anything in that message has become inaccurate because of the fixup, use an `amend!` commit instead (`git commit --fixup=amend:<sha>`).
- Never squash the fixups yourself. Leave them in the history as separate commits; collapsing them into their targets is the user's action, taken once they've reviewed the iterations. If you think the history is ready to collapse, say so and leave it to them.

## After Completion

After all checks pass and changes are committed:

0. **Review the change** on the two axes described in [Code Review](#code-review) before pushing.
1. **Push changes** to the repository
2. **Create a pull request** immediately after pushing - do not wait for a prompt. Open PRs proactively for any pushed commit that does not already have one.
3. **Include in PR description**:
   - Summary of changes made
   - Any relevant issue numbers (e.g., "Closes #123")
   - Testing performed
4. **Request review** if applicable

## Code Review

Review every change on two axes before it is treated as complete:

- **Standards**: does the change conform to the coding standards documented
  in this file (and the Fowler smell baseline where the repo is silent)?
- **Spec**: does the change faithfully implement the originating issue or
  spec — no missing requirements, no scope creep, no incorrectly
  implemented requirements?

Run both axes as parallel sub-agents and report them separately; aggregate
the findings, fix any defects, then proceed to push and open a PR. Use the
`code-review` skill to drive this review.

When additional changes land on a branch after its PR was already opened,
update the PR description so it still reflects the cumulative set of changes
(summary, issue references, testing performed) rather than only the original
scope.

## Code comments

Comments in source code explain *why* this code is shaped the way it is. They are not the place to narrate the path taken during development — what was tried first, what didn't work, what's "more reliable" or "cleaner" than some alternative. That framing is noise to later readers: the rejected alternative is nowhere in the file, so the comparison is meaningless.

- Avoid phrasings like "we used to … but …", "after trying X, we found Y", or "X rather than Y" where Y is what the code did before the change.
- The iteration story sometimes belongs in the commit message — the durable record of *why* a change was made — not in the code comment.
- The check to apply: would you have written this comment if you were writing the file from scratch, with no diff in mind? If not, the sentence belongs in the commit message.
- If the codebase calls a helper in many places without explanation, your new call site doesn't need one either. A comment there says "something here is unusual"; when nothing is, it's noise.

## Engineering judgment

### Surface decisions
When a decision surfaces while implementing — a design choice, a tradeoff, a scope cut, an "this turned out harder than expected, so maybe X" — don't quietly make the call and keep going, even if you have a clear recommendation. Stop, lay out the options and your recommendation, and let the user weigh in. Obvious mechanical choices with one sensible answer don't need a checkpoint, but genuine forks — where a reasonable person might pick differently, or where you'd trade away something the plan assumed — do. This applies to unforeseen discoveries (a latent bug, a race, a wrong assumption) too: stop and raise them before designing or writing a fix.

### Don't present "live with the bug" as an option
When investigating a defect and laying out fix options, "accept the race / leave it as-is / document it and move on" is not one of them. A known race, data corruption, or correctness violation is a bug that needs a real fix. If a real fix is genuinely out of reach, say so plainly; don't dress "no fix" up as a viable option alongside real ones.

### Prefer the cleaner design over the smaller diff
When a task could be done by tacking onto existing code or by first restructuring it slightly, choose the restructuring. "Minimal change" is not a goal in itself; a readable final state is. The prep-refactor-then-behavior-change pattern exists for exactly this. This is not license for speculative abstraction, but if the current change would be clearer after extracting a method, splitting a function, or adjusting names, that refactor is part of the task.

## Hard Rules

- No `unsafe` code anywhere - CI rejects it
- No `#[allow(warnings)]` attributes - fix the underlying issue instead
- Leave crate versions in `Cargo.toml` and `CHANGELOG.md` entries untouched: the crate publishing workflow bumps versions and generates changelogs automatically (git-cliff, based on conventional commits). Only change source, tests, and docs.
