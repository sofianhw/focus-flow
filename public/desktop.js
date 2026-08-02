import init from "/pkg/focus_flow.js";

// The WASM module's `#[wasm_bindgen(start)]` function mounts the CSR app once
// initialization completes. Keeping this as a local module avoids inline
// JavaScript and keeps the desktop CSP useful.
await init();
