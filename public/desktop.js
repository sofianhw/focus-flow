import init from "/pkg/focus_flow.js";

// The WASM module's `#[wasm_bindgen(start)]` function mounts the CSR app once
// initialization completes. Keeping this as a local module avoids inline
// JavaScript and keeps the desktop CSP useful.
// cargo-leptos emits the desktop WASM artifact as `focus_flow.wasm`.
// Pass it explicitly because wasm-bindgen's default sibling lookup expects
// the alternate `focus_flow_bg.wasm` filename.
await init("/pkg/focus_flow.wasm");
