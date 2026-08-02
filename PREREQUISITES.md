# Prerequisites

## Rust 1.90.0 (MSRV)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The project pins Rust 1.90.0 in rust-toolchain.toml
rustup toolchain install 1.90.0
rustup target add wasm32-unknown-unknown --toolchain 1.90.0
```

## Leptos

```bash
# Install the pinned cargo-leptos release used by CI
cargo install cargo-leptos --version 0.3.7 --locked
```

## Repository quality gate

Install the pinned command runner used by local development and CI:

```bash
cargo install just --version 1.40.0 --locked
```

## Tauri

```bash
# Install the pinned Tauri CLI release used by CI
cargo install tauri-cli --version 2.9.5 --locked

# macOS dependencies (if needed)
xcode-select --install
```

For other platforms, see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/).

## Node.js and pnpm

```bash
# Use Node.js 22.x (the supported range is >=22 <23)
# Enable the pinned pnpm release through Corepack
corepack enable
corepack prepare pnpm@11.9.0 --activate

pnpm --version # 11.9.0
```

The lockfile pins `picomatch` to 2.3.2 through the `pnpm.overrides` entry in
`package.json`. This closes the transitive advisory pulled in by the Tailwind
CLI until its upstream watcher dependency raises the minimum version.

## Verification

Run the same fast quality gate used by pull requests:

```bash
just check
```

Network-dependent advisory scans are separate:

```bash
just audit
```

`just audit` always runs the JavaScript advisory scan. The Rust advisory scan
is optional and requires the separately maintained `cargo-audit` tool; it is
checked explicitly and will print this install command when absent:

```bash
cargo install cargo-audit --locked
```
