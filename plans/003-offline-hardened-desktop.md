# Plan 003: Ship an offline, hardened Tauri desktop bundle

> **Executor instructions**: Follow the accepted runtime ADR from Plan 001. Run each verification gate. Stop on any STOP condition and report instead of switching to a sidecar or remote URL. Update `plans/README.md` when complete.
>
> **Drift check (run first)**: `git diff --stat 13268db..HEAD -- Cargo.toml app src-tauri package.json pnpm-lock.yaml justfile .github/workflows`
> Reconcile expected Plan 002 changes; unexpected runtime/build changes are a STOP condition.

## Status

- **Priority**: P0
- **Effort**: L
- **Risk**: MED
- **Depends on**: Plans 001 and 002
- **Category**: bug, security, dx
- **Planned at**: commit `13268db`, 2026-08-02

## Why this matters

The current installer can contain a valid Tauri binary yet show nothing—or an unrelated local service—because its production window loads port 4317 and no bundled process serves that port. The same webview disables CSP, injects the global Tauri object, and compiles release devtools. This must be fixed before any public installer is advertised.

## Current state

- `src-tauri/tauri.conf.json:6-9` has `devUrl` and a build command but no `frontendDist`.
- `src-tauri/tauri.conf.json:12-19` enables global Tauri, sets CSP to null, and gives the window a loopback HTTP URL.
- `src-tauri/tauri.conf.json:39` includes no server or frontend resources.
- `src-tauri/Cargo.toml:21` enables Tauri's `devtools` feature in all profiles.
- `src-tauri/src/lib.rs:10-14` never starts the Axum server.
- `target/site/` contains generated JS/WASM/CSS/assets but no `index.html`; it is SSR output and cannot simply be assigned to `frontendDist`.
- `app/src/shell.rs:19-43` contains inline theme/loading scripts that require deliberate CSP handling.

Tauri's documented invariant is that a path-valued `frontendDist` is recursively embedded and served from its `index.html`; a URL-valued distribution embeds no assets. The target here is a path-valued static CSR distribution.

## Commands you will need

| Purpose | Command | Expected on success |
| --- | --- | --- |
| CSR compile | `cargo check -p app --no-default-features --features csr` | Exit 0 |
| Static build | `pnpm build:desktop` | Exit 0; produces `dist/index.html`, JS, WASM, CSS, and icons |
| Tauri check | `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` | Exit 0 |
| Package | `cargo tauri build` | Exit 0 on macOS and Windows |
| Config assertion | `rg -n "127\.0\.0\.1|localhost" src-tauri/tauri.conf.json` | Only `build.devUrl` may match |
| Asset assertion | `test -f dist/index.html` | Exit 0 |

## Scope

**In scope**:

- Static desktop entry HTML/config required by the chosen pinned Rust/WASM bundler
- `app/src/lib.rs` and a dedicated CSR desktop entrypoint
- `Cargo.toml` Leptos/build metadata only as required
- `package.json`, `pnpm-lock.yaml`, and build-tool config
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- Tauri capability files if native commands remain
- `justfile`, release workflow, and runtime smoke tests
- Runtime/build sections of README, AGENTS, and ARCHITECTURE
- `plans/README.md`

**Out of scope**:

- Domain refactor and persistence (Plan 004 and future work)
- Axum sidecar packaging
- Remote content
- New Tauri plugins unrelated to rendering the current app
- Mobile packaging

## Git workflow

- Branch: `codex/003-offline-desktop`
- Suggested commits: static CSR entry/build, Tauri hardening, packaged smoke/CI.
- Do not push or publish installers unless explicitly asked.

## Steps

### Step 1: Produce a standalone CSR distribution

Add a dedicated wasm32 desktop entry that mounts `App` with Leptos CSR; do not call the hydration entrypoint against an empty document. Add a pinned static Rust/WASM bundler configuration and an `index.html` that references only local assets. Add `pnpm build:desktop` as the canonical command and make it emit all files under `dist/`.

Keep `server/` buildable as the optional SSR/web adapter, but do not invoke it from the desktop build. Do not duplicate business behavior between the CSR and SSR entrypoints.

**Verify**: delete only the ignored `dist/` directory, run `pnpm build:desktop`, then confirm `dist/index.html`, at least one `.wasm`, one `.js`, the compiled Tailwind CSS, and icons exist. Open the static output through a basic local file server and verify Home and Settings render.

### Step 2: Embed assets and remove the production loopback URL

Set `build.frontendDist` to the static output path relative to `src-tauri/tauri.conf.json`. Keep `build.devUrl` for development only. Remove the explicit HTTP `app.windows[0].url` or change it to the local app path (`index.html`) so release builds cannot navigate to port 4317.

Set `beforeBuildCommand` to `pnpm build:desktop`. Align `beforeDevCommand` with the same CSR topology so development and production do not exercise different rendering modes.

**Verify**: a config assertion proves only `devUrl` contains loopback HTTP; `cargo tauri build --no-bundle` exits 0 with no server process running.

### Step 3: Minimize the webview privilege surface

- Set `withGlobalTauri` false unless a documented current call site requires it.
- Remove the Tauri `devtools` Cargo feature from release builds; enable development inspection through debug-only configuration if needed.
- Remove `tauri-plugin-opener` if no current UI path calls it. If retained, define an explicit least-privilege capability and test only the allowed target/scope.
- Add a restrictive production CSP appropriate for local WASM. Include only the minimum needed sources; Tauri documents that Rust/WASM needs `wasm-unsafe-eval`. Account for current inline style attributes. Move the theme/loading JavaScript to local files or use Tauri's hash/nonce processing rather than leaving CSP null.
- Default the Leptos/Axum desktop development address to `127.0.0.1`; keep `0.0.0.0` only for explicit mobile/LAN recipes with a warning.

**Verify**: `rg -n '"csp": null|withGlobalTauri.*true|features.*devtools' src-tauri` → no production matches; app starts, theme toggles, sidebar works, and timer starts under the CSP.

### Step 4: Add a packaged-runtime smoke gate

Add an automated config/artifact test that fails when:

- `frontendDist` is absent or lacks `index.html`;
- the production window URL uses HTTP(S);
- CSP is null;
- the release dependency enables devtools;
- the output references an external CDN or port 4317.

In macOS and Windows release CI, launch the built application with no server on port 4317 and verify the dashboard becomes visible. If full WebDriver automation is not stable on a runner, keep the structural assertions automated and require a signed release-candidate manual smoke checklist on both platforms.

**Verify**: intentionally replace `frontendDist` with a loopback URL in a temporary working-tree change and confirm the smoke/config test fails, then restore and confirm it passes.

## Test plan

- CSR entry renders the app without SSR markup.
- Home → Settings → Home navigation works from embedded assets.
- Theme initialization and loading screen comply with CSP.
- Packaged app starts while port 4317 is unused.
- Packaged app never starts a listener or requires Node/Axum.
- macOS title bar/drag region and Windows window controls remain usable.

## Done criteria

- [ ] `dist/index.html` and all local UI assets are generated deterministically.
- [ ] Tauri embeds a path-valued `frontendDist` and production has no loopback window URL.
- [ ] Installed macOS and Windows apps render offline with no sidecar.
- [ ] CSP is enabled, global Tauri is off unless justified, and release devtools are disabled.
- [ ] The unused opener plugin is removed or least-privilege scoped.
- [ ] Desktop dev binds loopback by default.
- [ ] CI has structural and platform smoke evidence.
- [ ] Runtime docs match the shipped result.
- [ ] `plans/README.md` marks Plan 003 DONE.

## STOP conditions

- The accepted ADR chooses a production sidecar instead of embedded CSR.
- The selected static build cannot produce a working `index.html` without copying SSR-only logic. Stop and document the bundler/entrypoint failure before choosing another tool.
- CSP requires broad remote origins or `unsafe-eval` beyond Tauri's documented WASM requirement. Stop and identify the exact asset/code path.
- A native capability is required but its allowed scope cannot be bounded.

## Maintenance notes

Any future Tauri plugin must include a capability review, CSP impact, and packaged smoke case. Keep development and release asset entrypoints as similar as possible; a build-only success is not evidence that the installed application renders.

