# Contributing to Focus Flow

Focus Flow is a local-first Rust/Leptos/Tauri desktop prototype for macOS and
Windows. Start with the project context before changing code:

1. [README](README.md) — current status and setup.
2. [ARCHITECTURE](ARCHITECTURE.md) — runtime topology, boundaries, and ports.
3. [DESIGN](DESIGN.md) — UI language, interaction, and accessibility rules.
4. The relevant [ADR](docs/adr/) — accepted decisions for boundary changes.
5. [AGENTS](AGENTS.md) — file map, workflow, and known limitations.

## Before you start

Install the tools and platform prerequisites in
[PREREQUISITES.md](PREREQUISITES.md), then follow the quick start in the
[README](README.md). Keep changes focused and issue-first:

- For behavior, architecture, runtime, or persistence changes, describe the
  user outcome and trade-offs in an issue or ADR before implementation.
- For small fixes, explain the problem and intended scope in the pull request.
- Use a descriptive branch such as `feature/timer-sound`,
  `fix/sidebar-focus`, or `docs/contributing`; use short imperative commit
  subjects (for example, `Fix timer pause state`).

## Where changes belong

| Change | Primary location |
| --- | --- |
| Invariants, commands, events, ports | `crates/focus-flow-core/` |
| Use-case coordination and adapters | `app/src/adapters/` and the core application layer |
| Leptos screens and presentation state | `app/src/domain/` and `app/src/app.rs` |
| Shared controls and layout | `app/src/components/` |
| Native commands, capabilities, packaging | `src-tauri/` |
| Optional web/SSR adapter | `server/` |
| Tokens and global presentation styles | `style/` |

Keep framework, browser, storage, and Tauri types at adapter boundaries. Read
[ARCHITECTURE](ARCHITECTURE.md) before adding a port, changing runtime
topology, or introducing persistence.

## Tests and quality gate

Add deterministic core or adapter tests when behavior changes. UI changes must
preserve keyboard access, focus visibility, semantic labels, and the states
described in [DESIGN](DESIGN.md). Tauri or runtime changes should include the
appropriate desktop smoke or configuration coverage. Before requesting review,
run the repository gate:

```sh
just check
```

Installer and platform-specific changes should also be exercised on the target
OS and summarized in the pull request. The installed desktop app must remain
offline-capable; Node.js, Axum, and loopback serving are development/build
concerns only.

## Keep documentation in sync

| Change | Update |
| --- | --- |
| Runtime topology or desktop packaging | [README](README.md), [ARCHITECTURE](ARCHITECTURE.md), relevant ADR, [AGENTS](AGENTS.md) |
| Ports, domain rules, or persistence | [ARCHITECTURE](ARCHITECTURE.md), relevant ADR, [AGENTS](AGENTS.md) |
| UI tokens, interaction, or accessibility | [DESIGN](DESIGN.md) and affected source-map notes |
| Commands, prerequisites, or supported platforms | [README](README.md), [PREREQUISITES](PREREQUISITES.md), [AGENTS](AGENTS.md) |

## Pull requests

Keep one coherent change per pull request. The template checks for a clear
problem, a bounded approach, evidence from `just check`, and documentation or
ADR updates when needed. UI changes need before/after screenshots and an
accessibility note. Reviewers will look for correctness, boundary direction,
regressions, test coverage, and scope discipline; explain intentional
trade-offs rather than hiding them in unrelated refactors.

## Maintainer TODOs for public release

Focus Flow is open source and welcomes contributions while it is in early
alpha. The following repository-maintenance items are intentionally tracked as
follow-up work:

- [ ] Confirm the copyright holder and canonical GitHub repository.
- [ ] Configure private security reporting and conduct-enforcement contacts.
- [ ] Publish the supported release lines and support expectations.
- [ ] Run `cargo audit` in a maintainer environment and record the result.
- [ ] Configure macOS and Windows signing before promoting installers beyond
  experimental status.
