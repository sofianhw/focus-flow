use crate::{FocusSessionState, PriorityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionCompletionReason {
    DeadlineReached,
    PausedAfterExpiry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    SessionStarted,
    SessionPaused,
    SessionReset,
    SessionRefreshed,
    SessionCompleted { reason: SessionCompletionReason },
    SettingsUpdated,
    PriorityAdded { id: PriorityId },
    PriorityReplaced { id: PriorityId },
    PriorityCompleted { id: PriorityId },
    StateChanged { session: FocusSessionState },
}
