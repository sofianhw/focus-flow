# Focus Flow architecture

This document describes the architecture that is in the repository today. It
is intentionally concrete: if a future design is not implemented, it is listed
under **Not implemented yet** rather than presented as a current boundary.

## Product and runtime

Focus Flow is a local-first focus timer and three-priority workspace for macOS
and Windows. The desktop app has no login, remote API, database, or required
background service. A fresh application mount starts with
`WorkspaceSnapshot::default()`.

The desktop runtime is an embedded Leptos CSR bundle inside Tauri:

```text
Development
  cargo tauri dev
    ├─ pnpm build:desktop
    │    └─ cargo leptos ... --frontend-only --lib-features csr
    └─ static server → http://127.0.0.1:4317 → dist/

Packaged desktop app
  Tauri native window → embedded ../dist/
                         ├─ desktop/index.html
                         ├─ public/desktop.js
                         └─ Leptos WASM, CSS, icons
```

The installed app does not start Node.js, Axum, or a loopback server. Node.js
and pnpm are build-time tools only. Development uses port `4317`; the Leptos
reload port is `4318`.

## Package layout

There are two Cargo workspaces:

1. The root workspace contains `app`, `server`, and
   `crates/focus-flow-core`.
2. `src-tauri` is a separate Tauri workspace used to build the native wrapper.

| Path | What it owns now |
| --- | --- |
| `crates/focus-flow-core/` | Framework-independent domain values, invariants, commands, events, ports, and `WorkspaceService`. |
| `app/src/app.rs` | Leptos composition root: creates the controller, presentation signals, header, sidebar, and Home/Settings switch. |
| `app/src/domain/home/page_home.rs` | Dashboard rendering and commands for starting/pausing/resetting the timer and editing/completing the first priority. |
| `app/src/domain/settings/page.rs` | Settings inputs, parsing, validation feedback, and settings command dispatch. |
| `app/src/adapters/controller.rs` | Long-lived `WorkspaceController` around `WorkspaceService<AppClock>`. |
| `app/src/adapters/clock.rs` | Browser `Date.now()` in CSR/hydrate builds and system time outside the browser. |
| `app/src/adapters/timer.rs` | One cancelable browser interval that requests `Command::RefreshSession`. |
| `app/src/adapters/repository.rs` | In-memory `WorkspaceRepository` implementation used by adapter tests; not wired into the app yet. |
| `app/src/components/` | Leptos presentation components, layout, theme hook, and shared Button/Card/Input primitives. |
| `app/src/lib.rs` | CSR `mount()` entrypoint and SSR `hydrate()` entrypoint. |
| `app/src/shell.rs` | SSR HTML shell, metadata, stylesheet links, and optional reload/hydration scripts. |
| `server/` | Optional Axum/Leptos SSR adapter. It is not a desktop sidecar. |
| `src-tauri/` | Native window, bundle, icon, CSP, and platform configuration. It currently exposes no application commands. |
| `desktop/`, `public/`, `scripts/` | Offline desktop HTML/JS entrypoint and the asset/build/smoke scripts. |

## Hexagonal boundary as implemented

The core is the policy boundary. Framework and platform code points toward it:

```text
Leptos Home / Settings handlers
              │ focus_flow_core::Command
              ▼
WorkspaceController
  Arc<Mutex<WorkspaceService<AppClock>>>
              │
              ▼
WorkspaceService
  validates commands, updates WorkspaceSnapshot, emits Events
              │
              ▼
focus-flow-core domain
  FocusSettings · FocusSessionState · Priority · WorkspaceSnapshot
```

`focus-flow-core` has no Leptos, Tauri, Axum, browser, filesystem, or database
dependency. Its ports are deliberately small:

- `Clock::now_ms()` makes timer behavior deterministic in tests.
- `WorkspaceRepository` describes load/save behavior, but is not used by the
  current controller.

The application service accepts `Command` values and returns a
`CommandResult { snapshot, events }` or a typed `CommandError`. Domain rules
include:

- session length: 5–90 minutes;
- daily goal: 15–600 minutes;
- at most three priorities;
- non-empty trimmed priority titles;
- deadline-based timers rather than callback-counted timers.

The UI currently renders the returned snapshot. Events are emitted by the core
for explicit transition semantics and future integrations, but no notification
adapter consumes them yet.

## State and timer flow

`App()` creates one `WorkspaceController` and one `RwSignal<WorkspaceSnapshot>`
above the Home/Settings view switch. Both pages receive the same controller and
signal, so changing views does not reset a running session.

```text
click Start/Pause/Reset
  → page event handler
  → WorkspaceController::dispatch(Command)
  → WorkspaceService + AppClock
  → new snapshot and events
  → workspace signal update
  → Leptos re-render
```

When the core reports a running session, `TimerScheduler` installs one
one-second browser interval in CSR/hydrate builds. Each tick dispatches
`RefreshSession`; the core compares the current clock to the absolute deadline.
Dropping the scheduler or leaving the owner cancels the interval. SSR builds do
not install browser scheduling.

Changing settings updates the settings in the snapshot immediately, but does
not restart or alter an already running/paused session. The Home weekly chart,
daily rhythm card, and several labels are static prototype data; they are not
yet backed by focus history.

## Tauri and optional web paths

`src-tauri/src/main.rs` only calls `focus_flow::run()`. The Tauri library builds
a native window from `tauri.conf.json`; it does not own workspace state and does
not call an Axum process. The visible native title bar is kept separate from
the custom Leptos header, while `header.rs` supplies the drag region.

The optional `server` binary reads the Leptos configuration, builds an Axum
router, renders `app::shell::shell`, serves static assets, and exposes the
generated Leptos server-function route. This path is useful for SSR/web
development only; desktop release does not depend on it.

## Not implemented yet

These are explicit seams, not current behavior:

- durable workspace persistence (native file/store or browser storage);
- a Tauri command/API layer for native features;
- notifications when a session completes;
- typed focus history feeding the weekly chart;
- mobile navigation parity;
- authentication, accounts, teams, or a remote sync service.

Do not add an abstraction for one of these areas until there is a user-facing
behavior to support. When adding one, keep the core independent, implement the
in-memory/test path first, and record a boundary change in a new ADR.

## Contribution rules

### Domain or behavior change

1. Put invariants and typed errors in `crates/focus-flow-core`.
2. Add or update `WorkspaceService` commands and deterministic tests.
3. Use ports for time or storage; do not import framework types into the core.
4. Wire the behavior into the Leptos page or native adapter at the edge.

### New UI surface

1. Read [`DESIGN.md`](DESIGN.md).
2. Keep presentation state in `app/src/app.rs` or the relevant component/page.
3. Reuse `Button`, `Card`, and `Input` from `app/src/components/ui/`.
4. Keep business rules in a core command rather than duplicating them in a
   view event handler.
5. Exercise light/dark, keyboard, focus-visible, and collapsed-sidebar states.

### Persistence or native capability

1. Define behavior first in a core port or command.
2. Extend the in-memory adapter and tests.
3. Implement the native adapter or Tauri command at the boundary.
4. Document privacy, failure, migration, and platform behavior.
5. Add/update an ADR if the runtime topology or dependency direction changes.

## Verification contract

The canonical gate is:

```sh
just check
```

It runs formatting, workspace Clippy, workspace tests, CSR and server checks,
Tauri Clippy, the desktop asset build, the offline bundle smoke test, and the
desktop port contract. `just audit` is the separate network-dependent advisory
scan. Keep both commands green before opening a pull request.

Architectural decisions are recorded in:

- [ADR 0001 — embedded desktop runtime](docs/adr/0001-embedded-desktop-runtime.md)
- [ADR 0002 — framework-independent focus core](docs/adr/0002-hexagonal-boundaries.md)

