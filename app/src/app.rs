use focus_flow_core::WorkspaceSnapshot;
use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::Router;

use crate::adapters::controller::WorkspaceController;
use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::components::layout::header::Header;
use crate::components::layout::sidebar::Sidebar;
use crate::domain::home::HomePage;
use crate::domain::settings::SettingsPage;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Home,
    Settings,
}

#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();
    let current_view = RwSignal::new(AppView::Home);
    let sidebar_collapsed = RwSignal::new(false);
    let controller = WorkspaceController::new(WorkspaceSnapshot::default());
    let workspace = RwSignal::new(controller.snapshot());

    provide_meta_context();

    view! {
        <Title text="Focus Flow" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppWrapper>
                <Header />

                <div class="flex min-h-0 flex-1">
                    <Sidebar current_view=current_view collapsed=sidebar_collapsed />
                    <main class="min-w-0 flex-1 overflow-y-auto overflow-x-clip">
                        {move || match current_view.get() {
                            AppView::Home => view! {
                                <HomePage controller=controller.clone() workspace=workspace />
                            }.into_any(),
                            AppView::Settings => view! {
                                <SettingsPage controller=controller.clone() workspace=workspace />
                            }.into_any(),
                        }}
                    </main>
                </div>
            </AppWrapper>
        </Router>
    }
}
