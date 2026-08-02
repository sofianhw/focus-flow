# Plan 002: Establish one contributor and CI quality gate

> **Executor instructions**: Follow every step and verification gate. If a STOP condition occurs, stop and report. Update this plan's row in `plans/README.md` when complete.
>
> **Drift check (run first)**: `git diff --stat 13268db..HEAD -- Cargo.toml src-tauri/Cargo.toml rust-toolchain.toml rustfmt.toml package.json pnpm-lock.yaml justfile PREREQUISITES.md .github`
> Material differences from the current state below are a STOP condition until reconciled.

## Status

- **Priority**: P0
- **Effort**: M
- **Risk**: LOW
- **Depends on**: `plans/001-runtime-and-hexagonal-decisions.md`
- **Category**: tests, dx, migration, security
- **Planned at**: commit `13268db`, 2026-08-02

## Why this matters

The repository has zero tests and no pull-request workflow. The root Cargo workspace excludes `src-tauri`, so a successful root check does not validate the desktop wrapper. Tool versions float, rustfmt emits stable/nightly mismatch warnings, and `pnpm audit` currently fails on a high-severity build dependency advisory.

## Current state

- `.github/workflows/release.yml:3-36` runs only on tags/manual dispatch and goes straight from install to packaging.
- `AGENTS.md:48-56` lists five commands but no canonical one-command gate.
- `Cargo.toml:1-3` contains `app` and `server`; `src-tauri/Cargo.toml:7` declares a separate workspace.
- `rust-toolchain.toml:2` floats on `stable`; the audited working environment uses Rust 1.90.0.
- `package.json:1-8` has no name, scripts, engines, or `packageManager`; CI uses Node 22 and pnpm 11 while the audited environment uses pnpm 11.9.0.
- `PREREQUISITES.md:17-27` installs latest `cargo-leptos` and Tauri CLI; the audited compatible versions are cargo-leptos 0.3.7 and tauri-cli 2.9.5.
- `rustfmt.toml:3-5` uses nightly-only settings with a stable toolchain.
- `justfile:13-15` hides tracked mobile configs with `git update-index --assume-unchanged`.
- `justfile:27,36,44` rewrites `localhost`, but `src-tauri/tauri.conf.json` uses `127.0.0.1`, so custom ports do not work.
- `pnpm audit --audit-level high` reports one high and one moderate advisory; the high advisory is `picomatch <2.3.2` through Tailwind CLI → Parcel watcher → micromatch.

## Commands you will need

| Purpose | Command | Expected on success |
| --- | --- | --- |
| Format | `cargo fmt --all -- --check` | Exit 0 with no nightly-option warning |
| Root lint | `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| Root tests | `cargo test --workspace` | Exit 0; non-zero tests after Plan 004 |
| Tauri lint | `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` | Exit 0 |
| App CSR check | `cargo check -p app --no-default-features --features csr` | Exit 0 |
| Server check | `cargo check -p server --no-default-features` | Exit 0 |
| JS audit | `pnpm audit --audit-level high` | Exit 0 |
| Canonical gate | `just check` | Runs all required checks and exits 0 |

## Scope

**In scope**:

- `justfile`
- `.github/workflows/ci.yml` (create)
- `.github/workflows/release.yml`
- `.github/dependabot.yml` (create)
- `rust-toolchain.toml`
- `rustfmt.toml`
- `package.json`
- `pnpm-lock.yaml`
- `PREREQUISITES.md`
- `AGENTS.md` command section
- `plans/README.md`

**Out of scope**:

- Application behavior and UI changes
- Tauri runtime topology changes (Plan 003)
- Combining the two Cargo workspaces
- Adding token-dependent or flaky network tests to every PR

## Git workflow

- Branch: `codex/002-verification-ci`
- Suggested commits: `build: pin contributor toolchain`, then `ci: add pull request quality gate`.
- Do not push or open a PR unless instructed.

## Steps

### Step 1: Pin the supported toolchain

- Pin `rust-toolchain.toml` to the verified Rust version `1.90.0` unless dependencies introduced after this audit require a newer compiler; record the MSRV in README/PREREQUISITES.
- Add `packageManager: "pnpm@11.9.0"` and Node engine `>=22 <23` to `package.json` so local and CI match Node 22.
- Add scripts that delegate to repository commands rather than duplicating logic.
- Pin cargo-leptos 0.3.7 and tauri-cli 2.9.5 in documentation and release CI.
- Remove or replace rustfmt settings that are ignored on stable. Keep `edition`, `max_width`, and stable settings only.

**Verify**: `cargo fmt --all -- --check` → exit 0 without nightly-feature warnings; `pnpm --version` → 11.9.0 in a Corepack-managed fresh environment.

### Step 2: Repair dependency health

Update the Tailwind/Parcel dependency chain or add a narrowly documented pnpm override so `picomatch` resolves to 2.3.2 or later. Prefer a normal upstream upgrade over a permanent override. Run a frozen reinstall and both Tailwind dev/build flows before accepting the lockfile.

Add Dependabot groups for Cargo, GitHub Actions, and npm/pnpm with a weekly schedule. Do not auto-merge major framework updates.

**Verify**: `pnpm install --frozen-lockfile && pnpm audit --audit-level high` → both exit 0.

### Step 3: Add a portable canonical gate

Add `just check` that runs, in order:

1. `cargo fmt --all -- --check`
2. root Clippy with warnings denied
3. root tests
4. explicit CSR and server feature checks
5. Tauri Clippy with warnings denied
6. the static frontend build command chosen by Plan 003 once it exists; until then, `cargo leptos build --release`

Add a separate `just audit` for network-dependent `pnpm audit` and Rust advisory scanning. If `cargo-audit` is adopted, pin its version in setup/CI rather than silently installing latest.

Replace `just setup` so it never changes Git index flags. Replace the tracked-file port rewrite with environment/config overrides. A custom desktop port must update both the dev server and Tauri `devUrl` without modifying tracked JSON.

**Verify**: `just check` → exit 0; `git ls-files -v src-tauri/tauri.*.conf.json` → no assume-unchanged flags introduced.

### Step 4: Add pull-request CI

Create `.github/workflows/ci.yml` triggered by pull requests and pushes to the default branch. Use least-privilege `contents: read`, concurrency cancellation, pinned Node/pnpm/Rust/tool versions, and Cargo/pnpm caches keyed by lockfiles. Run `just check` on Linux for fast feedback and add a small macOS/Windows matrix for platform-sensitive Tauri compilation. Keep installer creation in release CI.

Update release CI to run the same canonical gate before packaging, pin CLI versions, and cache Cargo registry/git/targets so both platform jobs do not rebuild every tool from scratch.

**Verify**: validate YAML syntax locally; open a draft PR or use `act` only if already installed. On GitHub, all matrix jobs must pass before merge.

## Test plan

- Add a lightweight shell/config test for the custom-port resolution rather than relying on manual observation.
- Ensure CI demonstrates that root and Tauri workspaces are both checked.
- Plan 004 supplies behavioral tests; until then, record that `cargo test --workspace` executes zero tests rather than presenting it as coverage.

## Done criteria

- [ ] One `just check` command matches CI and covers both Cargo workspaces.
- [ ] Pull requests run formatting, lint, tests, and build checks.
- [ ] Tool versions are machine-readable and match CI/docs.
- [ ] `pnpm audit --audit-level high` exits 0.
- [ ] Stable rustfmt produces no ignored-option warning.
- [ ] Setup does not hide tracked files; custom desktop ports work without editing tracked config.
- [ ] Release CI uses pinned cached tools and runs the quality gate before packaging.
- [ ] `plans/README.md` marks Plan 002 DONE.

## STOP conditions

- Pinning Rust 1.90.0 fails after dependencies are refreshed. Stop, identify the minimum passing compiler, and update ADR/docs consistently.
- The dependency advisory cannot be removed without a Tailwind major migration. Stop and create a focused dependency-migration plan with reachability evidence.
- A GitHub-hosted runner cannot compile Tauri without new system packages. Stop and record the exact missing package rather than weakening the gate.

## Maintenance notes

Keep the canonical command single-sourced in `justfile`; CI should call it rather than reproduce its steps. Review tool pin updates like code changes and preserve a scheduled dependency-maintenance path.

