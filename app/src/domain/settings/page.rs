use focus_flow_core::{Command, WorkspaceSnapshot};
use leptos::prelude::*;

use crate::adapters::controller::WorkspaceController;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::{Input, InputType};

#[component]
pub fn SettingsPage(
    controller: WorkspaceController,
    workspace: RwSignal<WorkspaceSnapshot>,
) -> impl IntoView {
    let focus_value =
        RwSignal::new(workspace.get_untracked().settings.session_minutes().to_string());
    let goal_value =
        RwSignal::new(workspace.get_untracked().settings.daily_goal_minutes().to_string());
    let validation_error = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        let session_text = focus_value.get();
        let goal_text = goal_value.get();
        let parsed = session_text.parse::<u32>().and_then(|session_minutes| {
            goal_text.parse::<u32>().map(|daily_goal_minutes| (session_minutes, daily_goal_minutes))
        });

        match parsed {
            Ok((session_minutes, daily_goal_minutes)) => match controller
                .dispatch(Command::UpdateSettings { session_minutes, daily_goal_minutes })
            {
                Ok(result) => {
                    workspace.set(result.snapshot);
                    validation_error.set(None);
                }
                Err(error) => validation_error.set(Some(error.to_string())),
            },
            Err(_) => {
                validation_error.set(Some("Enter whole numbers for both settings.".to_owned()))
            }
        }
    });

    view! {
        <div class="mx-auto flex w-full max-w-3xl flex-col gap-6 px-5 py-8 sm:px-8">
            <div><p class="mb-2 text-sm font-medium text-muted-foreground">"WORKSPACE SETTINGS"</p><h1 class="text-3xl font-bold tracking-tight">"Make Focus Flow yours."</h1><p class="mt-2 text-muted-foreground">"Tune the rhythm of your sessions and daily target."</p></div>
            <Card><CardHeader><CardTitle>"Focus rhythm"</CardTitle><CardDescription>"These values apply to your next session immediately."</CardDescription></CardHeader><CardContent class="flex flex-col gap-5">
                <label class="flex flex-col gap-2 text-sm font-medium">"Session length"<div class="flex items-center gap-3"><Input r#type=InputType::Number min="5" max="90" step="5" class="max-w-32" bind_value=focus_value /><span class="text-sm text-muted-foreground">"minutes"</span></div></label>
                <label class="flex flex-col gap-2 text-sm font-medium">"Daily focus goal"<div class="flex items-center gap-3"><Input r#type=InputType::Number min="15" max="600" step="15" class="max-w-32" bind_value=goal_value /><span class="text-sm text-muted-foreground">"minutes"</span></div></label>
                <p class=move || if validation_error.get().is_some() { "text-sm text-destructive" } else { "hidden" } role="alert">
                    {move || validation_error.get().unwrap_or_default()}
                </p>
            </CardContent></Card>
            <Card><CardHeader><CardTitle>"About this workspace"</CardTitle><CardDescription>"Focus Flow keeps this prototype local to your device."</CardDescription></CardHeader><CardContent class="text-sm leading-6 text-muted-foreground">"No account or server is required. Your preferences stay active for this session while the app is open."</CardContent></Card>
        </div>
    }
}
