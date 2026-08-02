use icons::{House, Settings};
use leptos::prelude::*;

use crate::app::AppView;

#[component]
pub fn Sidebar(current_view: RwSignal<AppView>, collapsed: RwSignal<bool>) -> impl IntoView {
    view! {
        <aside
            class=move || {
                let width = if collapsed.get() { "w-20" } else { "w-64" };
                format!("hidden shrink-0 flex-col border-r bg-card/60 p-3 transition-[width] duration-200 md:flex {width}")
            }
        >
            <nav class="flex flex-1 flex-col gap-1" aria-label="Primary navigation">
                <div class=move || if collapsed.get() { "mb-3 flex justify-center" } else { "mb-3 flex items-center justify-between px-3" }>
                    <p class=move || if collapsed.get() { "sr-only" } else { "text-xs font-medium uppercase tracking-wider text-muted-foreground" }>
                        "Workspace"
                    </p>
                    <button
                        type="button"
                        aria-label=move || if collapsed.get() { "Expand sidebar" } else { "Collapse sidebar" }
                        title=move || if collapsed.get() { "Expand sidebar" } else { "Collapse sidebar" }
                        class="grid size-8 place-items-center rounded-md text-lg text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                        on:click=move |_| collapsed.update(|value| *value = !*value)
                    >
                        {move || if collapsed.get() { "»" } else { "«" }}
                    </button>
                </div>

                <button
                    type="button"
                    aria-label="Home"
                    title="Home"
                    class=move || {
                        let active = current_view.get() == AppView::Home;
                        let alignment = if collapsed.get() { "justify-center px-0" } else { "justify-start px-3" };
                        let base = format!("flex h-10 items-center gap-3 rounded-lg text-left text-sm font-medium transition-colors {alignment}");
                        if active { format!("{base} bg-accent text-accent-foreground") } else { format!("{base} text-muted-foreground hover:bg-accent/70 hover:text-foreground") }
                    }
                    on:click=move |_| current_view.set(AppView::Home)
                >
                    <span class="grid size-5 shrink-0 place-items-center" aria-hidden="true">
                        <House class="size-5" />
                    </span>
                    <span class=move || if collapsed.get() { "hidden" } else { "block" }>
                        "Home"
                    </span>
                </button>

                <button
                    type="button"
                    aria-label="Settings"
                    title="Settings"
                    class=move || {
                        let active = current_view.get() == AppView::Settings;
                        let alignment = if collapsed.get() { "justify-center px-0" } else { "justify-start px-3" };
                        let base = format!("flex h-10 items-center gap-3 rounded-lg text-left text-sm font-medium transition-colors {alignment}");
                        if active { format!("{base} bg-accent text-accent-foreground") } else { format!("{base} text-muted-foreground hover:bg-accent/70 hover:text-foreground") }
                    }
                    on:click=move |_| current_view.set(AppView::Settings)
                >
                    <span class="grid size-5 shrink-0 place-items-center" aria-hidden="true">
                        <Settings class="size-5" />
                    </span>
                    <span class=move || if collapsed.get() { "hidden" } else { "block" }>
                        "Settings"
                    </span>
                </button>
            </nav>

            <div class=move || if collapsed.get() { "sr-only" } else { "mt-auto border-t px-3 pt-3 text-xs text-muted-foreground" }>
                "Local workspace"
            </div>
        </aside>
    }
}
