# ADR 0002: Keep the focus core framework-independent

- **Status:** Accepted
- **Date:** 2026-08-02
- **Decision owners:** Focus Flow maintainers

## Context

The current Home and Settings components contain presentation, timer
validation, browser scheduling, and task state in the same Leptos functions.
That makes the behavior difficult to test without a browser and makes a
future native store or web adapter unnecessarily expensive to add.

## Decision

The target architecture has one pure `focus-flow-core` crate. It contains two
inward-facing modules:

- `domain`: timer/session concepts, priorities, invariants, and domain events;
- `application`: use cases that coordinate domain behavior through ports.

`focus-flow-core` must not depend on Leptos, Tauri, Axum, Tailwind, `web_sys`,
or filesystem/database libraries. Framework and platform crates may depend on
the core, never the reverse.

The initial outbound ports are:

- `Clock`: supplies time to timer use cases without binding them to browser or
  system-clock APIs;
- `WorkspaceRepository`: loads and saves workspace settings, priorities, and
  focus history without selecting a storage technology.

Reserve `NotificationPort` for a later decision. It should be introduced when
session completion notifications are implemented, rather than as an unused
abstraction today.

Inbound adapters own delivery and presentation concerns:

- Leptos components translate user intent into application use-case calls and
  render state.
- Tauri commands expose native capabilities and translate command payloads.
- Optional Axum handlers expose web/SSR delivery and HTTP concerns.

Outbound adapters implement ports:

- an in-memory adapter supports deterministic tests;
- a native file/store adapter will provide durable desktop persistence;
- a future browser or web adapter may provide an appropriate web store.

Navigation, theme state, responsive layout, interval scheduling, Tauri
commands, Axum handlers, and storage implementations remain outside the core.
The UI may choose how to display an application result, but it must not define
the domain invariant that produced it.

## Dependency direction

```text
Leptos UI ───────────────┐
Tauri commands ──────────┼──> application use cases ──> domain
Optional Axum API/SSR ───┘              │
                                        ├── Clock
                                        └── WorkspaceRepository
                                                ▲
                     Native store / in-memory adapters
```

The arrows point toward policy. Adapters are replaceable details; domain and
application policy are stable inward dependencies.

## Consequences

- Timer and workspace rules can be tested with plain Rust and a fake `Clock`.
- Desktop persistence can change without rewriting Home or Settings views.
- A web endpoint can reuse application behavior without importing Tauri.
- The repository now contains the `focus-flow-core` crate and an explicit
  controller composition root, implemented through Plan 004.
- Existing components may still need incremental refinement, but new behavior
  must preserve the established core boundary rather than reintroducing
  framework policy into views.

Architectural changes that add a new port, reverse a dependency, or change the
desktop runtime require a new ADR. Do not rewrite an accepted ADR silently.
