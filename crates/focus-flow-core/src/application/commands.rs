use crate::PriorityId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    StartSession,
    PauseSession,
    ResetSession,
    RefreshSession,
    UpdateSettings { session_minutes: u32, daily_goal_minutes: u32 },
    ReplacePriority { id: PriorityId, title: String },
    AddPriority { title: String },
    CompletePriority { id: PriorityId },
}
