#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
config="$repo_root/src-tauri/tauri.conf.json"
dist="$repo_root/dist"

python3 - "$config" "$dist" <<'PY'
import json
import pathlib
import sys

config_path = pathlib.Path(sys.argv[1])
dist = pathlib.Path(sys.argv[2])
config = json.loads(config_path.read_text())
build = config["build"]
app = config["app"]
security = app["security"]

frontend_dist = build.get("frontendDist")
assert frontend_dist == "../dist", f"expected ../dist frontendDist, got {frontend_dist!r}"
assert build.get("devUrl", "").startswith("http://127.0.0.1:"), "devUrl must stay loopback-only"
assert all("url" not in window or not str(window["url"]).startswith(("http://", "https://"))
           for window in app.get("windows", [])), "release window URL must be local/embedded"
assert app.get("withGlobalTauri") is False, "global Tauri API must remain disabled"
csp = security.get("csp")
assert csp and csp is not None, "production CSP must not be null"
assert "wasm-unsafe-eval" in csp, "CSP must allow local WASM evaluation"
assert "https://" not in csp and "http://" not in csp, "CSP must not permit remote origins"

for relative in ("index.html", "desktop.js", "pkg/focus_flow.js", "pkg/focus_flow.wasm", "pkg/focus_flow.css"):
    assert (dist / relative).is_file(), f"missing desktop asset: {relative}"

index = (dist / "index.html").read_text()
assert "https://" not in index and "http://" not in index, "desktop HTML must be self-contained"
assert "4317" not in index, "desktop HTML must not reference the development port"
desktop_js = (dist / "desktop.js").read_text()
assert 'init("/pkg/focus_flow.wasm")' in desktop_js, "desktop entrypoint must load the emitted WASM filename"
print("Desktop bundle structure is valid")
PY

if rg -n 'features\s*=.*devtools|"devtools"' "$repo_root/src-tauri/Cargo.toml"; then
  echo "release Tauri dependency must not enable devtools" >&2
  exit 1
fi

if rg -n 'https?://|4317' "$dist/index.html" "$dist/desktop.js"; then
  echo "desktop output contains a remote origin or development port" >&2
  exit 1
fi
