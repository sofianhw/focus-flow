#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dist_dir="$repo_root/dist"
site_dir="$repo_root/target/site"

command -v cargo >/dev/null 2>&1 || {
  echo "cargo is required; install the pinned Rust toolchain from PREREQUISITES.md" >&2
  exit 1
}
command -v cargo-leptos >/dev/null 2>&1 || {
  echo "cargo-leptos is required; install the pinned version from PREREQUISITES.md" >&2
  exit 1
}

rm -rf "$dist_dir"

# cargo-leptos owns the Rust/WASM and Tailwind pipeline. The CSR feature is
# selected explicitly so this build never compiles or starts the Axum server.
(cd "$repo_root" && cargo leptos build --release --frontend-only --lib-features csr --project focus_flow)

test -f "$site_dir/pkg/focus_flow.js"
test -f "$site_dir/pkg/focus_flow.wasm"
test -f "$site_dir/pkg/focus_flow.css"

mkdir -p "$dist_dir"
cp -R "$site_dir/." "$dist_dir/"
cp "$repo_root/desktop/index.html" "$dist_dir/index.html"

test -f "$dist_dir/index.html"
test -f "$dist_dir/desktop.js"
test -f "$dist_dir/pkg/focus_flow.js"
test -f "$dist_dir/pkg/focus_flow.wasm"
test -f "$dist_dir/pkg/focus_flow.css"
test -f "$dist_dir/icons/logo.png"

echo "Desktop assets written to $dist_dir"
