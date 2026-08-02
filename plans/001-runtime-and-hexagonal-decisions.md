# Plan 001: Decide and document the runtime and hexagonal boundaries

> **Executor instructions**: Follow this plan step by step. Run every verification command and confirm the expected result before moving on. If a STOP condition occurs, stop and report; do not improvise. When done, update this plan's row in `plans/README.md`.
>
> **Drift check (run first)**: `git diff --stat 13268db..HEAD -- README.md AGENTS.md DESIGN.md ARCHITECTURE.md docs/adr`
> If an in-scope file changed since this plan was written, compare the current state below with live code. A material mismatch is a STOP condition.

## Status

- **Priority**: P0
- **Effort**: M
- **Risk**: LOW
- **Depends on**: none
- **Category**: tech-debt, docs
- **Planned at**: commit `13268db`, 2026-08-02

## Why this matters

The repository does not currently say whether the installed desktop app embeds its UI or requires a Rust server. The code builds an Axum server, while Tauri opens a loopback URL without starting that server. Hexagonal architecture will not help contributors unless the repository also defines dependency direction, production topology, and concrete extension recipes.

## Current state

- `src-tauri/tauri.conf.json:6-20` runs a Leptos release build but has no `frontendDist`; the application window points to `http://127.0.0.1:4317` in every build.
- `src-tauri/src/lib.rs:10-14` starts Tauri and the opener plugin only; it does not supervise the Axum binary.
- `server/src/main.rs:15-21` independently binds and serves Axum.
- `app/src/domain/home/page_home.rs:10-56` combines UI, timer rules, browser scheduling, task rules, and reactive state.
- `app/src/domain/settings/page.rs:7-20` performs parsing and domain limits inside Leptos effects.
- `AGENTS.md:14-35` maps current files but defines no allowed dependency directions, ports, adapters, or runtime decision.
- `README.md:14-27` describes installers and says data is not sent to a server without explaining the local Axum process.
- `ARCHITECTURE.md` and `docs/adr/` do not exist.

The target architecture for this plan is:

```text
Leptos UI (inbound adapter) ───────┐
Tauri commands (inbound adapter) ─┼──> application use cases ──> domain
Optional Axum API/SSR adapter ─────┘              │
                                                  ├── Clock port
                                                  ├── WorkspaceRepository port
                                                  └── Notification port (later)

Native file/store adapter ───────── implements outbound ports
In-memory adapter ───────────────── implements outbound ports for tests
```

Dependency rule: adapters may depend inward; `focus-flow-core` must not depend on Leptos, Axum, Tauri, Tailwind, `web_sys`, or filesystem/database libraries.

## Commands you will need

| Purpose | Command | Expected on success |
| --- | --- | --- |
| Find forbidden terms | `rg -n "sidecar|frontendDist|hexagonal|FocusRepository" README.md AGENTS.md DESIGN.md ARCHITECTURE.md docs/adr` | Matches agree with the decisions below |
| Check links/style | `git diff --check -- README.md AGENTS.md DESIGN.md ARCHITECTURE.md docs/adr` | Exit 0 |
| Confirm no code changes | `git diff --name-only` | Only documentation files listed in Scope plus `plans/README.md` |

## Scope

**In scope**:

- `ARCHITECTURE.md` (create)
- `docs/adr/0001-embedded-desktop-runtime.md` (create)
- `docs/adr/0002-hexagonal-boundaries.md` (create)
- `README.md`
- `AGENTS.md`
- `DESIGN.md` only if it needs an architecture link or boundary clarification
- `plans/README.md` status row

**Out of scope**:

- Rust, JSON, workflow, lockfile, and package configuration changes
- Persistence implementation
- Generic plugin APIs
- Mobile architecture promises

## Git workflow

- Branch: `codex/001-runtime-architecture`
- Use one documentation commit such as `docs: define runtime and architecture boundaries`.
- Do not push or open a PR unless the operator asks.

## Steps

### Step 1: Record the desktop runtime decision

Create ADR 0001 with status `Accepted`. Decide that production macOS and Windows builds embed a static Leptos CSR frontend inside Tauri and require no Node process, Axum server, open TCP port, or remote backend at runtime. State that `server/` is an optional web/SSR adapter and is not part of the desktop bundle. Include consequences: desktop and web builds have separate entrypoints; shared behavior belongs in the core; packaged smoke tests must work offline.

**Verify**: `rg -n "Accepted|embedded|CSR|Axum|offline" docs/adr/0001-embedded-desktop-runtime.md` → all five concepts are present.

### Step 2: Record the dependency rules

Create ADR 0002 with status `Accepted`. Define one pure `focus-flow-core` crate with `domain` and `application` modules. Define initial outbound ports as `Clock` and `WorkspaceRepository`; reserve `NotificationPort` until completion notification is implemented. Explicitly keep navigation, theme, responsive layout, interval scheduling, Tauri commands, Axum handlers, and storage implementations outside the core.

**Verify**: `rg -n "focus-flow-core|Clock|WorkspaceRepository|Leptos|Tauri|Axum" docs/adr/0002-hexagonal-boundaries.md` → the inward dependency and adapter ownership are explicit.

### Step 3: Create the authoritative architecture guide

Create `ARCHITECTURE.md` with:

1. System context and supported targets.
2. The dependency diagram above.
3. Current state versus target state, clearly labeled.
4. Ownership table for `focus-flow-core`, `app`, `src-tauri`, `server`, and shared UI primitives.
5. Port definitions and current/planned adapters.
6. Timer command/event flow and persistence flow.
7. Rules enforced by tests/CI.
8. Recipes for adding a domain feature, UI screen, persistence adapter, Tauri capability, and optional web endpoint.
9. ADR index and a rule that architectural changes require a new ADR rather than silently rewriting accepted history.

Avoid claiming that the target folders already exist. Use “current” and “target” headings so the guide remains honest during migration.

**Verify**: `rg -n "Current architecture|Target architecture|Dependency rule|Adding a feature|ADR" ARCHITECTURE.md` → all required sections exist.

### Step 4: Reconcile the existing guides

- README: add a short architecture summary and link to `ARCHITECTURE.md`; label installers as not ready until Plan 003 completes.
- AGENTS: replace “no backend” with “no remote/product backend”; explain that Axum is currently a local SSR/dev adapter and is not the target desktop runtime. Remove the machine-specific OrbStack/Langfuse explanation. Link the architecture and ADRs before the file map.
- DESIGN: link to `ARCHITECTURE.md` and say design state belongs to the presentation adapter; do not add domain rules to visual components.

**Verify**: `rg -n "no backend|OrbStack|Langfuse" README.md AGENTS.md DESIGN.md` → no misleading or machine-specific matches.

## Test plan

This is a decision/documentation plan; no runtime tests are added. Validate relative links manually or with the repository's Markdown link checker once Plan 002 adds it. Confirm every claimed command/path exists with `rg --files`.

## Done criteria

- [ ] Two accepted ADRs state the runtime and dependency decisions.
- [ ] `ARCHITECTURE.md` distinguishes current and target states and includes extension recipes.
- [ ] README does not claim installers are usable before Plan 003.
- [ ] AGENTS no longer contradicts the presence of the local Axum adapter.
- [ ] `git diff --check` exits 0 for all in-scope docs.
- [ ] No source/config files changed.
- [ ] `plans/README.md` marks Plan 001 DONE.

## STOP conditions

- The maintainer wants Axum to be a required production sidecar. Stop and replace ADR 0001 with a decision proposal; do not accept either topology without explicit approval.
- The project must support mobile in the same release architecture immediately. Stop and scope a mobile-specific ADR before promising parity.
- Existing docs added after `13268db` already record a conflicting accepted decision.

## Maintenance notes

Reviewers should reject code that imports framework/platform crates into `focus-flow-core`. Update the architecture guide only when the code or an accepted ADR changes; do not use it as a speculative roadmap.

