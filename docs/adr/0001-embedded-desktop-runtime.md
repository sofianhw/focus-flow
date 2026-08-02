# ADR 0001: Embed the desktop frontend in Tauri

- **Status:** Accepted
- **Date:** 2026-08-02
- **Decision owners:** Focus Flow maintainers

## Context

Focus Flow is a local-first desktop application. The current development setup
runs `cargo leptos watch`, serves the Leptos application from a local Axum
process, and points the Tauri window at `http://127.0.0.1:4317`. That is useful
for hot reload, but it is not a reliable installed-app runtime: Tauri currently
does not supervise a server process, and an installed application must not
depend on another process or an available TCP port.

The repository also contains a `server/` crate because Leptos SSR and the web
development workflow need an Axum entrypoint. That server is an adapter for
web/SSR use, not a product backend or a required desktop service.

## Decision

Production macOS and Windows builds will embed a static Leptos CSR frontend in
the Tauri application. A packaged desktop application must run offline and
must not require:

- a Node.js process;
- an Axum/Rust server process;
- an open local TCP port; or
- a remote backend or account service.

The `server/` crate remains an optional web/SSR adapter. It is used by local
development and any future web deployment, but it is not included in the
desktop bundle and is not started by `src-tauri`.

The production Tauri configuration and release workflow must therefore use a
static frontend distribution directory for the desktop build. Development may
continue to use the local Leptos server and hot reload. The separate entry
points are intentional:

```text
Desktop: Tauri → embedded Leptos CSR assets
Web/SSR:  Axum → Leptos SSR application
```

## Consequences

Positive consequences:

- Installed macOS and Windows builds work without Node, Rust, or a local
  server being installed.
- The desktop app has no port collision or localhost availability failure.
- Offline behavior is a first-class release requirement.
- Shared behavior can move into a framework-independent core instead of being
  hidden in the SSR or browser entrypoint.

Costs and constraints:

- Desktop and web builds have separate entrypoints and must be tested
  separately.
- Build automation must produce and package the CSR assets before `cargo tauri
  build` runs.
- Features that require synchronization or accounts need a new product and
  architecture decision; they cannot silently turn the desktop runtime into a
  network client.
- Packaged smoke tests must launch the application with network access
  disabled and verify the first screen renders.

## Implementation status

Plan 003 implemented this decision. `src-tauri/tauri.conf.json` now uses
`frontendDist: "../dist"` for packaged builds and keeps the loopback
`devUrl` only for development. The release window has no HTTP URL, the CSR
assets are built before packaging, and `scripts/check-desktop-bundle.sh`
asserts the embedded assets, CSP, and offline-safe configuration. The optional
`server/` crate remains a web/SSR adapter.
