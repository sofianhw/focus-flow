# Plan 004: Extract a tested, framework-independent focus core

> **Executor instructions**: Migrate one use case at a time and keep the app runnable between steps. Run every gate. Stop and report on a STOP condition. Update `plans/README.md` when complete.
>
> **Drift check (run first)**: `git diff --stat 13268db..HEAD -- Cargo.toml app server src-tauri ARCHITECTURE.md docs/adr`
> Expected changes from Plans 001–003 must match their documented target. Any new behavior model is a STOP condition until reconciled.

## Status

- **Priority**: P1
- **Effort**: L
- **Risk**: MED
- **Depends on**: Plans 001 and 002; run after Plan 003 unless maintained in a separate non-conflicting branch
- **Category**: tech-debt, tests, bug
- **Planned at**: commit `13268db`, 2026-08-02

## Why this matters

Timer, task, and settings behavior currently lives in Leptos closures and effects. Navigation destroys Home state, browser interval callbacks are never cancelled, and elapsed time is measured by counting callbacks. A small pure Rust core gives contributors stable extension points and deterministic tests without turning every folder into a framework.

## Current state

- `app/src/app.rs:22-25` stores view, sidebar, duration, and goal as raw Leptos signals.
- `app/src/app.rs:41-47` replaces Home with Settings, disposing Home-owned state.
- `app/src/domain/home/page_home.rs:11-16` owns session, completion, and task state inside the component.
- `app/src/domain/home/page_home.rs:18-36` implements start/pause/reset/complete/replace rules in event closures.
- `app/src/domain/home/page_home.rs:41-56` decrements once per real interval callback and never retains a cancelable handle.
- `app/src/domain/settings/page.rs:11-20` parses/clamps settings inside reactive effects.
- `app/src/components/hooks/use_theme_mode.rs:58-79` directly uses browser storage; product state uses unrelated in-memory signals.
- `app/src/domain/test/`, route metadata modules, mounted no-op bottom navigation, and reload exports remain from the starter but are not active product architecture.

Follow the existing Rust conventions: edition 2024, rustfmt, `thiserror` for typed errors, and `#![deny(clippy::unwrap_used)]` in library code. The design guide keeps navigation/theme/layout in the presentation layer.

## Commands you will need

| Purpose | Command | Expected on success |
| --- | --- | --- |
| Core tests | `cargo test -p focus-flow-core` | All deterministic domain/application tests pass |
| Core boundary | `cargo tree -p focus-flow-core` | No Leptos, Axum, Tauri, `web-sys`, Tailwind, or filesystem/database crate |
| Workspace tests | `cargo test --workspace` | Exit 0 with non-zero tests |
| Root lint | `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| CSR check | `cargo check -p app --no-default-features --features csr` | Exit 0 |
| Server check | `cargo check -p server --no-default-features` | Exit 0 |
| Canonical gate | `just check` | Exit 0 |

## Scope

**In scope**:

- Root `Cargo.toml`
- `crates/focus-flow-core/Cargo.toml` and `src/**` (create)
- Application state/presenter modules under `app/src/`
- `app/src/app.rs`
- `app/src/domain/home/page_home.rs`
- `app/src/domain/settings/page.rs`
- Timer scheduling adapter under `app/src/adapters/`
- Minimal server/Tauri composition changes required by the shared core
- Removal of confirmed unused starter modules/exports
- `ARCHITECTURE.md`, AGENTS, and `plans/README.md`

**Out of scope**:

- Durable storage format/database
- Native notifications
- Generic plugin registry
- Redesigning the UI
- Adding projects, tags, recurrence, or cloud sync

## Git workflow

- Branch: `codex/004-hexagonal-core`
- Commit per vertical slice: core model/tests, settings migration, session/timer migration, priority migration, cleanup.
- Do not push or open a PR unless instructed.

## Steps

### Step 1: Add the pure core with characterization tests

Create workspace crate `focus-flow-core`, initially with:

```text
src/
  domain/
    focus_settings.rs
    focus_session.rs
    priority.rs
    workspace.rs
  application/
    commands.rs
    events.rs
    ports.rs
    service.rs
  lib.rs
```

Model validated values rather than raw UI strings:

- `FocusSettings` with session length 5–90 minutes and daily goal 15–600 minutes.
- `FocusSessionState`: Ready, Running, Paused, Completed. Running state stores an absolute deadline supplied by a clock port; it does not count callbacks.
- `Priority` with stable ID, title, and pending/completed state. Initially preserve the current maximum of three priorities as an explicit rule or remove the limit only after a product decision.
- `WorkspaceSnapshot` aggregating settings, current session, priorities, and completed-session history placeholder.
- Commands for start, pause, reset, refresh/tick, update settings, replace/add priority, and complete priority.
- Events including `SessionCompleted` and state-change events; do not invoke Tauri from the event type.
- Ports: `Clock` and `WorkspaceRepository`, synchronous unless ADR 0002 explicitly chooses async. Provide typed errors.

Write tests before UI migration for all current transitions, invalid boundaries, zero/expired time, pause/resume, reset, repeated completion, and clock jumps/throttling. Use a fake clock and in-memory repository.

**Verify**: `cargo test -p focus-flow-core` → all tests pass; `cargo tree -p focus-flow-core` → no forbidden framework/platform dependencies.

### Step 2: Create the long-lived application composition

Create an application service/controller that owns the workspace state above page navigation. Add an in-memory repository adapter outside the core for the current prototype. Expose view-friendly state to Leptos without putting `RwSignal` in the core. `AppView`, sidebar collapse, and theme remain presentation state.

Document the direction explicitly:

```text
Leptos event -> application command -> core transition -> event/snapshot
snapshot -> presenter/view model -> Leptos render
```

**Verify**: a test switches presentation views while a running session and edited priority remain in the same application snapshot.

### Step 3: Migrate settings and priorities

Replace settings parsing/clamping effects with a UI parser that calls the core update command and renders validation feedback. Replace the single string and hard-coded completion counter with the core priority model, but preserve visible behavior unless the tests/documentation intentionally change it.

**Verify**: settings boundary tests and priority transition tests pass; Home → Settings → Home preserves values.

### Step 4: Migrate timer scheduling and lifecycle

The UI scheduling adapter may request refreshes at one-second intervals, but remaining time must be derived from the clock/deadline. Use a cancelable interval handle and clear it with the Leptos owner cleanup hook. Starting, pausing, navigating, minimizing, and resuming must not create multiple schedulers or drift by counting missed callbacks.

**Verify**:

- fake-clock tests advance 90 seconds in one jump and remaining time is correct;
- mount/unmount or adapter lifecycle test proves one interval is active and then cleared;
- Home → Settings → Home retains the same running/paused session.

### Step 5: Remove false extension points

After confirming no call sites, remove the placeholder Test domain, unused route metadata, mounted no-op bottom navigation, and unused reload export/component. Keep any genuinely required mobile helper behind a named feature with a test and documentation; do not retain “compatibility” code without a consumer.

**Verify**: `rg -n "TestPage|TestRoutes|HomeRoutes|SettingsRoutes|AppBottomNav|ReloadButton" app/src` → no matches, except an explicitly feature-gated and documented mobile helper if proven necessary.

### Step 6: Update architecture documentation

Mark the target core boundary as implemented, list the actual ports/adapters, and add a concrete “add a use case” recipe using one migrated command as the exemplar. AGENTS must tell contributors to put rules in core and browser/native mechanics in adapters.

**Verify**: documentation paths and names match `rg --files crates app/src/adapters` exactly.

## Test plan

- Settings: minimum, maximum, below/above range, invalid text handling.
- Session: ready/start/pause/resume/reset/complete; immediate expiry; large clock jump; repeated commands; duration change applies only according to documented semantics.
- Priorities: title validation, maximum/ordering decision, complete idempotence, stable IDs.
- Repository: load default, save/load round trip with in-memory adapter, error propagation.
- Presentation: navigation preserves application state; only one scheduler exists; UI labels reflect session state.
- Boundary: automated dependency check prevents forbidden framework/platform crates in core.

## Done criteria

- [ ] `focus-flow-core` contains tested domain/application behavior and no framework/platform dependencies.
- [ ] `cargo test --workspace` runs meaningful tests and passes.
- [ ] Navigation no longer resets timer, task, or completion state.
- [ ] Timer uses elapsed time/deadline and cancels its scheduler on cleanup.
- [ ] Leptos pages dispatch commands and render view state rather than own business rules.
- [ ] Starter-era unused modules are removed or explicitly feature-gated with evidence.
- [ ] ARCHITECTURE and AGENTS match the implemented dependency direction.
- [ ] `just check` passes.
- [ ] `plans/README.md` marks Plan 004 DONE.

## STOP conditions

- A core type requires Leptos, Tauri, Axum, `web_sys`, or a storage implementation. Stop and move that concern to an adapter.
- Current behavior is ambiguous enough that a test would encode an accidental prototype rule. Stop and request the product decision, especially for priority count and duration changes during a running session.
- The migration requires a durable schema. Stop and create a separate persistence ADR/plan rather than inventing compatibility guarantees.
- UI state cannot be preserved without changing visible navigation behavior. Stop and document the concrete Leptos ownership constraint.

## Maintenance notes

Ports should be added only for a real adapter/use case. Reviewers should be skeptical of generic repositories, plugin registries, or abstractions with one speculative caller. A future native store should implement `WorkspaceRepository`; it should not move filesystem types into core.

