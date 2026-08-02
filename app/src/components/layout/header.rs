use leptos::prelude::*;

use crate::components::layout::theme_toggle::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            data-tauri-drag-region
            class="sticky top-0 z-100 flex h-14 select-none items-center justify-between border-b bg-background px-5 py-3 sm:px-8"
        >
            <div data-tauri-drag-region class="flex items-center gap-3">
                <div class="grid size-8 place-items-center rounded-lg bg-violet-600 font-bold text-white">"F"</div>
                <span class="font-semibold tracking-tight">"Focus Flow"</span>
            </div>
            <ThemeToggle />
        </header>
    }
}
