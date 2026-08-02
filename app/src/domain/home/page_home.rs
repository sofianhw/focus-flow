use focus_flow_core::{Clock, Command, FocusSessionState, WorkspaceSnapshot};
use leptos::prelude::*;

use crate::adapters::controller::WorkspaceController;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use crate::adapters::timer::TimerScheduler;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::Input;

#[component]
pub fn HomePage(
    controller: WorkspaceController,
    workspace: RwSignal<WorkspaceSnapshot>,
) -> impl IntoView {
    let new_task = RwSignal::new(String::new());

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let scheduler = TimerScheduler::new();
        scheduler.install_cleanup();
        Effect::new({
            let controller = controller.clone();
            move |_| {
                let running = matches!(workspace.get().session, FocusSessionState::Running { .. });
                let scheduler = scheduler.clone();
                let controller = controller.clone();
                scheduler.sync(running, move || {
                    if let Ok(result) = controller.dispatch(Command::RefreshSession) {
                        workspace.set(result.snapshot);
                    }
                });
            }
        });
    }

    let toggle_controller = controller.clone();
    let toggle_timer = move |_| {
        let command = if matches!(workspace.get().session, FocusSessionState::Running { .. }) {
            Command::PauseSession
        } else {
            Command::StartSession
        };
        if let Ok(result) = toggle_controller.dispatch(command) {
            workspace.set(result.snapshot);
        }
    };
    let reset_controller = controller.clone();
    let reset_timer = move |_| {
        if let Ok(result) = reset_controller.dispatch(Command::ResetSession) {
            workspace.set(result.snapshot);
        }
    };
    let finish_controller = controller.clone();
    let finish_task = move |_| {
        if let Some(priority) = workspace.get().priorities.first()
            && let Ok(result) =
                finish_controller.dispatch(Command::CompletePriority { id: priority.id() })
        {
            workspace.set(result.snapshot);
        }
    };
    let add_controller = controller.clone();
    let add_task = move |_| {
        let title = new_task.get();
        if !title.trim().is_empty()
            && let Some(priority) = workspace.get().priorities.first()
            && let Ok(result) =
                add_controller.dispatch(Command::ReplacePriority { id: priority.id(), title })
        {
            workspace.set(result.snapshot);
            new_task.set(String::new());
        }
    };

    view! {
        <div class="mx-auto flex w-full max-w-5xl flex-col gap-6 px-5 py-8 sm:px-8">
            <section class="flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
                <div>
                    <p class="mb-2 text-sm font-medium text-muted-foreground">"FRIDAY, JULY 31"</p>
                    <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Make today count."</h1>
                    <p class="mt-2 text-muted-foreground">"A calm little space for your next focused session."</p>
                </div>
                <div class="rounded-full bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground">
                    {move || format!("{} of 3 priorities complete", workspace.get().completed_priorities())}
                </div>
            </section>

            <section class="grid gap-6 lg:grid-cols-[1.25fr_0.75fr]">
                <Card class="overflow-hidden border-0 bg-gradient-to-br from-violet-600 to-indigo-700 text-white shadow-lg">
                    <CardHeader>
                        <CardDescription class="text-violet-100">"FOCUS SESSION"</CardDescription>
                        <CardTitle class="tabular-nums text-5xl font-semibold tracking-tight text-white sm:text-6xl">
                            {move || {
                                let remaining = workspace.get().session.remaining_seconds(crate::adapters::clock::AppClock.now_ms());
                                format!("{:02}:{:02}", remaining / 60, remaining % 60)
                            }}
                        </CardTitle>
                    </CardHeader>
                    <CardContent class="flex flex-col gap-5">
                        <p class="text-violet-100">"One small commitment, uninterrupted."</p>
                        <div class="flex flex-wrap gap-3">
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Lg on:click=toggle_timer>
                                {move || if matches!(workspace.get().session, FocusSessionState::Running { .. }) { "Pause session" } else { "Start focus" }}
                            </Button>
                            <button class="rounded-md border border-white/30 px-5 py-2 text-sm font-medium transition hover:bg-white/10" on:click=reset_timer>"Reset"</button>
                        </div>
                        <p class="text-sm text-violet-100" aria-live="polite">
                            {move || if workspace.get().session.is_completed() { "Session complete. Nice work." } else if matches!(workspace.get().session, FocusSessionState::Running { .. }) { "Focus session is running." } else { "Ready when you are." }}
                        </p>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <CardDescription>"TODAY'S RHYTHM"</CardDescription>
                        <CardTitle>"2h 15m focused"</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="mb-3 flex justify-between text-sm"><span>"Daily goal"</span><span class="font-medium">"75%"</span></div>
                        <div class="h-3 overflow-hidden rounded-full bg-secondary"><div class="h-full w-3/4 rounded-full bg-violet-600"></div></div>
                        <p class="mt-5 text-sm text-muted-foreground">{move || format!("Your daily target is {} focused minutes.", workspace.get().settings.daily_goal_minutes())}</p>
                    </CardContent>
                </Card>
            </section>

            <section>
                <Card>
                    <CardHeader class="flex-row items-end justify-between"><div><CardDescription>"THIS WEEK"</CardDescription><CardTitle>"Focus minutes"</CardTitle></div><span class="text-sm font-medium text-emerald-600">"+18% vs last week"</span></CardHeader>
                    <CardContent><div class="grid h-44 grid-cols-7 items-end gap-2 sm:gap-4">
                        {[ ("Mon", 42_u32), ("Tue", 68_u32), ("Wed", 55_u32), ("Thu", 84_u32), ("Fri", 72_u32), ("Sat", 28_u32), ("Sun", 12_u32) ].into_iter().map(|(day, height)| view! { <div class="flex h-full flex-col items-center justify-end gap-2"><div class="flex w-full flex-1 items-end rounded-md bg-secondary/60"><div class="w-full rounded-md bg-violet-500 transition-all duration-300" style=format!("height: {}%", height) title=format!("{}: {} focus minutes", day, height)></div></div><span class="text-xs text-muted-foreground">{day}</span></div> }).collect_view()}
                    </div></CardContent>
                </Card>
            </section>

            <section class="grid gap-6 lg:grid-cols-[1.25fr_0.75fr]">
                <Card>
                    <CardHeader><CardDescription>"NEXT PRIORITY"</CardDescription><CardTitle>{move || workspace.get().priorities.first().map(|priority| priority.title().to_owned()).unwrap_or_else(|| "No priority yet".to_owned())}</CardTitle></CardHeader>
                    <CardContent class="flex flex-col gap-4"><div class="flex gap-3"><Input bind_value=new_task placeholder="Replace with a new priority" /><Button variant=ButtonVariant::Outline on:click=add_task>"Save"</Button></div><Button variant=ButtonVariant::Default on:click=finish_task>"Mark complete"</Button></CardContent>
                </Card>
                <Card><CardHeader><CardDescription>"UP NEXT"</CardDescription><CardTitle>"A lighter afternoon"</CardTitle></CardHeader><CardContent><ul class="space-y-3 text-sm"><li class="flex items-center gap-3"><span class="size-2 rounded-full bg-emerald-500"></span>"Reply to design notes"</li><li class="flex items-center gap-3"><span class="size-2 rounded-full bg-amber-500"></span>"Plan Monday's top three"</li><li class="flex items-center gap-3"><span class="size-2 rounded-full bg-sky-500"></span>"Take a proper break"</li></ul></CardContent></Card>
            </section>
        </div>
    }
}
