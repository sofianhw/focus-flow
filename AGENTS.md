# Focus Flow — Agent Guide

## Project purpose

Focus Flow is a local-first desktop focus planner. It is a small Rust/UI
prototype for macOS and Windows, not an account product: there is no login,
remote product service, database, or network API.

The current product surface is:

- Home dashboard with a 25-minute focus timer, task editing, completion counter, daily goal progress, and a weekly focus-minutes bar chart.
- Collapsible desktop sidebar with Home and Settings navigation.
- Settings for session length and daily focus goal. Validated settings live in the shared workspace snapshot for the current app session only.
- Light/dark theme toggle.

## Stack and architecture

Read [`ARCHITECTURE.md`](ARCHITECTURE.md) and the accepted decisions in
[`docs/adr/0001-embedded-desktop-runtime.md`](docs/adr/0001-embedded-desktop-runtime.md)
and [`docs/adr/0002-hexagonal-boundaries.md`](docs/adr/0002-hexagonal-boundaries.md)
before changing runtime boundaries or adding a persistence adapter.

The implemented desktop topology is an embedded Leptos CSR frontend inside
Tauri. Desktop development builds the same CSR assets into `dist/` and serves
them from a loopback-only static server; `server/` remains an optional web/SSR
adapter, not a required desktop service. Node.js and pnpm provide build-time
tooling only. A packaged app embeds `dist/` and does not start Axum, Node.js,
or a local server.

- `app/`: Leptos UI crate. Rust/UI components are copied into `app/src/components/ui/` and styled with Tailwind CSS.
- `app/src/app.rs`: application shell, `AppView`, shared presentation signals, sidebar/header composition, and view switching.
- `crates/focus-flow-core/`: framework-independent settings, session, priority, workspace, commands, events, and ports.
- `app/src/adapters/`: browser/system clock, long-lived workspace controller,
  in-memory repository test adapter, and cancelable browser timer scheduler.
  The repository port exists, but persistence is not wired into the controller
  or desktop composition root yet.
- `app/src/domain/home/page_home.rs`: dashboard presentation and command dispatch for timer and priorities.
- `app/src/domain/settings/page.rs`: settings inputs, validation feedback, and command dispatch.
- `app/src/components/layout/sidebar.rs`: collapsible navigation and centered collapsed icons.
- `app/src/components/layout/header.rs`: custom header below the native macOS title bar; keep the Tauri drag region here when changing the chrome.
- `server/`: Axum/Leptos SSR adapter used by `cargo-leptos` for development and
  optional web builds; it is not the target desktop runtime.
- `src-tauri/`: Tauri 2 native wrapper and installer configuration for macOS/Windows.
- `style/tailwind.css`: Rust/UI-inspired design tokens and Tailwind theme.

The important state flow is:

```text
Presentation signals (AppView, sidebar_collapsed, workspace snapshot)
  ├─ Sidebar: navigation + collapse state
  ├─ HomePage: renders snapshot and dispatches session/priority commands
  └─ SettingsPage: validates input and dispatches settings commands
       │
       └─ WorkspaceController → WorkspaceService → focus-flow-core

`App()` creates one controller and one workspace signal above the Home/Settings
switch. That is why navigating between views does not reset a running session.
The initial snapshot is `WorkspaceSnapshot::default()` on every app mount.
```

The refreshed graph's code-relevant hubs include `WorkspaceSnapshot`,
`WorkspaceController`, `ThemeMode`, `Priority`, `FocusSessionState`, and
`AppClock`. Check the workspace/controller, timer, and shared layout paths
before changing cross-cutting behavior. Document and plan nodes also appear in
the repository-wide graph; they are navigation metadata, not runtime modules.

## Local development

From this directory:

```sh
pnpm install
cargo tauri dev
```

Install the pinned tools listed in [`PREREQUISITES.md`](PREREQUISITES.md),
including `just`, `cargo-leptos`, and the Tauri CLI, before running the gate.

Desktop development uses `127.0.0.1:4317`; the Leptos reload port is `4318`.
`just run_desktop` is also available and defaults to the same port. These
loopback ports are development details only; the accepted production desktop
runtime will not require an open port.

Useful verification commands:

```sh
# Canonical local/CI gate (format, lint, tests, CSR/server/Tauri checks, build)
just check

# Network-dependent advisory scans
just audit
```

`just check` is the contributor contract and is intentionally single-sourced
in `justfile`; pull-request CI calls this command instead of duplicating its
steps. `cargo test --workspace` exercises the core domain/application transition
tests and adapter tests; keep adding deterministic coverage when behavior changes.

The project pins stable Rust and the `wasm32-unknown-unknown` target in `rust-toolchain.toml`. If a machine reports `can't find crate for core` for `wasm32-unknown-unknown`, run:

```sh
rustup target add wasm32-unknown-unknown
```

`pnpm-workspace.yaml` allows the optional `@parcel/watcher` build script required by pnpm 11. Do not remove that allowlist without testing a fresh `pnpm install`.

## Desktop packaging

```sh
cargo tauri build
```

The Tauri window uses a normal visible native title bar on macOS so traffic-light
controls have their own row. The web header uses `data-tauri-drag-region` for
custom dragging. Keep interactive controls outside the drag-region element when
adding new header actions. `cargo tauri build` embeds the generated `dist/`
assets; no Axum, Node.js, or loopback server is required by the installed app.

Release automation lives in `.github/workflows/release.yml` and builds an Apple Silicon macOS target plus an x86_64 Windows target from tags (`v*`). Signing credentials still need to be configured before public distribution.

## UI conventions

- Read [`DESIGN.md`](DESIGN.md) before changing the visual language or adding a new screen.
- Prefer existing Rust/UI primitives (`Button`, `Card`, `Input`) over one-off markup.
- Keep the calm, restrained visual language: system typography, violet/indigo primary accent, muted borders, and modest transitions.
- The sidebar collapse control belongs beside “Workspace”; when collapsed, the control and navigation icons must remain centered.
- Add `aria-label`/`title` to icon-only controls.
- Keep timer feedback visible with `aria-live`; the browser timer scheduler only runs in `csr`/`hydrate` builds, is derived from an absolute core deadline, and is cleared on owner cleanup so SSR never accesses browser APIs.
- The app is desktop-first. The sidebar is hidden below the `md` breakpoint; mobile navigation is still a future improvement.

## Known limitations and safe extension points

- Focus settings and priorities are in-memory only. Add persistence deliberately
  (browser storage or a native store), wire it through `WorkspaceRepository`,
  and update this guide and the architecture ADR if that changes. The theme
  toggle is the exception: `ThemeMode` stores its boolean in browser
  `localStorage`.
- The weekly chart is currently a static prototype dataset. Replace it with a typed data model before adding history or analytics.
- Starter-era test pages, route metadata, and the reload component were removed
  once their call sites were confirmed absent. Add new screens through `AppView`
  and the feature folders rather than reviving those placeholders.
- Android/iOS configs are deferred and default to `http://localhost:3000`; mobile is unsupported for now. Device/LAN users must override those URLs locally without committing LAN addresses. Desktop work should use `src-tauri/tauri.conf.json` and port `4317`.
- There is intentionally no authentication flow. Do not add username/password assumptions to the current product without a new product decision.

## Graphify map

The current code graph is in `graphify-out/`:

- `graph.json`: 462-node, 609-edge raw graph across 48 communities.
- `graph.html`: interactive graph visualization.
- `GRAPH_REPORT.md`: summarized communities, god nodes, and suggested questions.

This graph was refreshed from the current repository corpus (77 files) and
excludes semantic extraction of documentation/image assets because no Graphify
semantic backend was configured. To refresh it from the project root:

```sh
graphify extract app --cargo --out .
graphify cluster-only . --no-label
```

Keep this file current when the app shell, build commands, ports, or persistence model change.
