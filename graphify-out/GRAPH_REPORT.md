# Graph Report - focus-flow  (2026-08-02)

## Corpus Check
- 77 files · ~43,987 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 462 nodes · 609 edges · 48 communities (43 shown, 5 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `13268dbc`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]

## God Nodes (most connected - your core abstractions)
1. `WorkspaceSnapshot` - 24 edges
2. `WorkspaceController` - 13 edges
3. `Plan 001: Decide and document the runtime and hexagonal boundaries` - 12 edges
4. `Plan 002: Establish one contributor and CI quality gate` - 12 edges
5. `Plan 003: Ship an offline, hardened Tauri desktop bundle` - 12 edges
6. `Plan 004: Extract a tested, framework-independent focus core` - 12 edges
7. `Plan 005: Publish the contributor contract and community files` - 12 edges
8. `ThemeMode` - 11 edges
9. `Priority` - 11 edges
10. `FocusSessionState` - 10 edges

## Surprising Connections (you probably didn't know these)
- `WorkspaceController` --references--> `WorkspaceService`  [EXTRACTED]
  app/src/adapters/controller.rs → crates/focus-flow-core/src/application/service.rs
- `InMemoryWorkspaceRepository` --implements--> `WorkspaceRepository`  [EXTRACTED]
  app/src/adapters/repository.rs → crates/focus-flow-core/src/application/ports.rs
- `leptos_routes_handler()` --calls--> `shell()`  [INFERRED]
  server/src/app_router/build_app_router.rs → app/src/shell.rs
- `AppClock` --implements--> `Clock`  [EXTRACTED]
  app/src/adapters/clock.rs → crates/focus-flow-core/src/application/ports.rs
- `InMemoryWorkspaceRepository` --references--> `WorkspaceSnapshot`  [EXTRACTED]
  app/src/adapters/repository.rs → crates/focus-flow-core/src/domain/workspace.rs

## Import Cycles
- None detected.

## Communities (48 total, 5 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.07
Nodes (24): AppClock, WorkspaceController, InMemoryWorkspaceRepository, round_trips_snapshot(), Clock, RepositoryError, WorkspaceRepository, Arc (+16 more)

### Community 1 - "Community 1"
Cohesion: 0.22
Nodes (8): TimerHandle, TimerScheduler, Closure, Drop, Fn, FnMut, Rc, RefCell

### Community 2 - "Community 2"
Cohesion: 0.06
Nodes (35): ADR 0001: Embed the desktop frontend in Tauri, Consequences, Context, Decision, Implementation status, ADR 0002: Keep the focus core framework-independent, Consequences, Context (+27 more)

### Community 3 - "Community 3"
Cohesion: 0.23
Nodes (5): ThemeMode, use_theme_mode(), ThemeToggle(), Option, Storage

### Community 4 - "Community 4"
Cohesion: 0.21
Nodes (7): Command, Priority, PriorityError, validates_and_idempotently_completes(), Into, PriorityId, String

### Community 5 - "Community 5"
Cohesion: 0.11
Nodes (25): build_app_router(), leptos_routes_handler(), server_fn_handler(), AxumBody, AxumResponse, Body, AppState, use_app_state() (+17 more)

### Community 6 - "Community 6"
Cohesion: 0.14
Nodes (10): Default, deadline_survives_large_clock_jump(), expired_pause_completes_and_repeated_completion_is_safe(), FocusSessionState, pause_resume_and_reset_are_deterministic(), SessionError, FocusSettings, SettingsError (+2 more)

### Community 7 - "Community 7"
Cohesion: 0.20
Nodes (14): Event, SessionCompletionReason, adding_a_fourth_priority_is_rejected_without_mutating_the_snapshot(), changing_settings_does_not_restart_a_running_session(), CommandError, CommandResult, FakeClock, navigation_like_snapshot_access_preserves_running_session() (+6 more)

### Community 8 - "Community 8"
Cohesion: 0.10
Nodes (20): Adding a domain feature, Adding a feature, Adding a Tauri capability, Adding a UI screen, Adding an optional web endpoint, Adding persistence, ADR index and change policy, Current architecture (+12 more)

### Community 9 - "Community 9"
Cohesion: 0.10
Nodes (19): app, security, windows, withGlobalTauri, build, beforeBuildCommand, beforeDevCommand, devUrl (+11 more)

### Community 10 - "Community 10"
Cohesion: 0.11
Nodes (18): Commands you will need, Current state, Done criteria, Git workflow, Maintenance notes, Plan 004: Extract a tested, framework-independent focus core, Scope, Status (+10 more)

### Community 16 - "Community 16"
Cohesion: 0.11
Nodes (17): Commands you will need, Current state, Done criteria, Git workflow, Maintenance notes, Plan 005: Publish the contributor contract and community files, Scope, Status (+9 more)

### Community 21 - "Community 21"
Cohesion: 0.12
Nodes (16): dependencies, tailwindcss, @tailwindcss/cli, tw-animate-css, engines, node, pnpm, name (+8 more)

### Community 22 - "Community 22"
Cohesion: 0.20
Nodes (10): Community and project documents, Contributing, Current limitations, Focus Flow, Implemented features, Privacy and offline runtime, Run locally, Runtime and architecture (+2 more)

### Community 26 - "Community 26"
Cohesion: 0.12
Nodes (16): Commands you will need, Current state, Done criteria, Git workflow, Maintenance notes, Plan 001: Decide and document the runtime and hexagonal boundaries, Scope, Status (+8 more)

### Community 27 - "Community 27"
Cohesion: 0.12
Nodes (16): Commands you will need, Current state, Done criteria, Git workflow, Maintenance notes, Plan 002: Establish one contributor and CI quality gate, Scope, Status (+8 more)

### Community 28 - "Community 28"
Cohesion: 0.12
Nodes (16): Commands you will need, Current state, Done criteria, Git workflow, Maintenance notes, Plan 003: Ship an offline, hardened Tauri desktop bundle, Scope, Status (+8 more)

### Community 29 - "Community 29"
Cohesion: 0.14
Nodes (14): Accessibility and platform behavior, Application frame, Buttons and inputs, Cards, Charts and progress, Component rules, Contributor checklist, Focus Flow UI design guide (+6 more)

### Community 30 - "Community 30"
Cohesion: 0.15
Nodes (12): background_color, description, dir, display, icons, lang, name, orientation (+4 more)

### Community 31 - "Community 31"
Cohesion: 0.25
Nodes (8): Desktop packaging, Focus Flow — Agent Guide, Graphify map, Known limitations and safe extension points, Local development, Project purpose, Stack and architecture, UI conventions

### Community 32 - "Community 32"
Cohesion: 0.25
Nodes (7): Audit baseline, Dependency notes, Direction after these plans, Execution order and status, Findings considered and rejected, Implementation plans, Vetted findings covered

### Community 34 - "Community 34"
Cohesion: 0.33
Nodes (5): Approach, Documentation and architecture, Notes for reviewers, Problem, Verification

### Community 35 - "Community 35"
Cohesion: 0.40
Nodes (4): mimeTypes, port, root, server

## Knowledge Gaps
- **204 isolated node(s):** `SessionCompletionReason`, `name`, `private`, `type`, `packageManager` (+199 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `WorkspaceSnapshot` connect `Community 0` to `Community 4`, `Community 6`, `Community 7`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `Focus Flow architecture` connect `Community 8` to `Community 2`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **Why does `get_static_file()` connect `Community 5` to `Community 4`, `Community 6`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **What connects `SessionCompletionReason`, `name`, `private` to the rest of the system?**
  _204 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.07149758454106281 - nodes in this community are weakly interconnected._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.05603864734299517 - nodes in this community are weakly interconnected._
- **Should `Community 5` be split into smaller, more focused modules?**
  _Cohesion score 0.10887096774193548 - nodes in this community are weakly interconnected._