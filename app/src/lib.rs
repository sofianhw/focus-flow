#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
// #![recursion_limit = "256"]

pub mod adapters;
pub mod common;
pub mod components;
pub mod domain;

pub mod app;
pub mod shell;

/// Mount the desktop/web CSR application into an empty document.
///
/// The desktop bundle uses this entrypoint instead of the SSR hydration
/// entrypoint. `wasm-bindgen(start)` invokes it after the local WASM module is
/// initialized, so the embedded Tauri page does not need server-rendered
/// markup or a running Axum process.
#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn mount() {
    use app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
