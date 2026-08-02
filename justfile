# Cross-platform justfile for Focus Flow
# Install just: cargo install just

# Default recipe - show available commands
default:
    @just --list

# ============================================================================
# SETUP
# ============================================================================

setup:
    @echo "No repository setup changes are required. Install the pinned tools from PREREQUISITES.md."

# Canonical local/CI quality gate. Keep this list single-sourced.
check:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo check -p app --no-default-features --features csr
    cargo check -p server --no-default-features
    cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
    pnpm build:desktop
    just check-desktop-bundle
    just check-port-resolution

# Network-dependent advisory scans are kept separate from the fast PR gate.
audit:
    pnpm audit --audit-level high
    just check-cargo-audit

[unix]
check-cargo-audit:
    #!/usr/bin/env sh
    set -eu
    if ! command -v cargo-audit >/dev/null 2>&1; then
        echo "cargo-audit is not installed; run: cargo install cargo-audit --locked" >&2
        exit 1
    fi
    cargo audit

[windows]
check-cargo-audit:
    #!powershell
    if (-not (Get-Command cargo-audit -ErrorAction SilentlyContinue)) {
        Write-Error 'cargo-audit is not installed; run: cargo install cargo-audit --locked'
        exit 1
    }
    cargo audit

# Regression test for the desktop-port contract. The port is supplied to both
# Leptos and Tauri through environment/config overrides; tracked JSON is never edited.
check-port-resolution:
    ./scripts/check-port-resolution.sh

# Structural smoke gate for the packaged offline desktop runtime.
check-desktop-bundle:
    ./scripts/check-desktop-bundle.sh

[windows]
check-port-resolution:
    #!powershell
    $content = Get-Content justfile -Raw
    if ($content -notmatch 'LEPTOS_SITE_ADDR="127\.0\.0\.1:\{\{port\}\}"') { throw 'Leptos port contract missing' }
    if ($content -notmatch 'devUrl.*127\.0\.0\.1:\{\{port\}\}') { throw 'Tauri devUrl port contract missing' }
    if ($content -match 'sed -i.*tauri\.conf\.json') { throw 'Tracked Tauri config rewrite detected' }
    Write-Host 'desktop port contract OK'

# ============================================================================
# DESKTOP
# ============================================================================

# Run Tauri desktop dev with custom port (reload_port = port + 1)
[macos]
run_desktop port="4317":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    echo "Running on port {{port}} (reload: $reload_port)"
    PORT="{{port}}" LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev --config "{\"build\":{\"devUrl\":\"http://127.0.0.1:{{port}}\"}}"

[linux]
run_desktop port="4317":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    echo "Running on port {{port}} (reload: $reload_port)"
    PORT="{{port}}" LEPTOS_SITE_ADDR="127.0.0.1:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri dev --config "{\"build\":{\"devUrl\":\"http://127.0.0.1:{{port}}\"}}"

[windows]
run_desktop port="4317":
    #!powershell
    $reload_port = [int]{{port}} + 1
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "127.0.0.1:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    $env:PORT = "{{port}}"
    cargo tauri dev --config ('{"build":{"devUrl":"http://127.0.0.1:' + {{port}} + '"}}')

# ============================================================================
# MOBILE - iOS (macOS only)
# ============================================================================

# Run iOS simulator (macOS only)
[macos]
run_ios port="3000" device="iPhone 16 Pro":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.ios.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"

    APP_NAME=$(grep '^name' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    cp __HideKeyboardAccessory.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cp __DisableContentInsetAdjustment.m "src-tauri/gen/apple/Sources/${APP_NAME}/"
    cd src-tauri/gen/apple && xcodegen generate && cd ../../..

    xcrun simctl boot "{{device}}" || true
    open -a Simulator
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri ios dev "{{device}}"

[linux]
[windows]
run_ios port="" device="":
    @echo "Error: iOS development requires macOS"
    @exit 1

# ============================================================================
# MOBILE - Android (all platforms)
# ============================================================================

# Run Android emulator/device
[macos]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(ipconfig getifaddr en0)
    sed -i '' "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[linux]
run_android port="3000":
    #!/usr/bin/env bash
    set -e
    reload_port=$(({{port}} + 1))
    SERVER_IP=$(hostname -I | awk '{print $1}')
    sed -i "s|http://[0-9.]*:[0-9]*|http://${SERVER_IP}:{{port}}|g" src-tauri/tauri.android.conf.json
    echo "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    echo "Running on port {{port}} (reload: $reload_port)"
    LEPTOS_SITE_ADDR="0.0.0.0:{{port}}" LEPTOS_RELOAD_PORT="$reload_port" cargo tauri android dev

[windows]
run_android port="3000":
    #!powershell
    $reload_port = [int]{{port}} + 1
    $SERVER_IP = (Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.InterfaceAlias -notmatch 'Loopback' -and $_.PrefixOrigin -eq 'Dhcp' } | Select-Object -First 1).IPAddress
    (Get-Content src-tauri/tauri.android.conf.json) -replace 'http://[0-9.]+:[0-9]+', "http://${SERVER_IP}:{{port}}" | Set-Content src-tauri/tauri.android.conf.json
    Write-Host "Updated SERVER_URL to http://${SERVER_IP}:{{port}}"
    Write-Host "Running on port {{port}} (reload: $reload_port)"
    $env:LEPTOS_SITE_ADDR = "0.0.0.0:{{port}}"
    $env:LEPTOS_RELOAD_PORT = "$reload_port"
    cargo tauri android dev
