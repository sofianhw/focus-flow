#!/usr/bin/env bash
set -euo pipefail

# Keep this small contract test independent of the desktop runtime. It verifies
# that the port recipe passes the same loopback port to Leptos and Tauri and
# never rewrites a tracked configuration file.
port="${1:-4317}"
[[ "$port" =~ ^[0-9]+$ ]]
(( port > 0 && port < 65536 ))

justfile="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/justfile"
grep -F 'LEPTOS_SITE_ADDR="127.0.0.1:{{port}}"' "$justfile" >/dev/null
grep -F 'devUrl' "$justfile" >/dev/null
grep -F '127.0.0.1:{{port}}' "$justfile" >/dev/null
! grep -E 'sed -i.*tauri\.conf\.json' "$justfile" >/dev/null

echo "desktop port contract OK: 127.0.0.1:${port}"
