use super::{focus_session::FocusSessionState, focus_settings::FocusSettings, priority::Priority};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceSnapshot {
    pub settings: FocusSettings,
    pub session: FocusSessionState,
    pub priorities: Vec<Priority>,
    pub completed_sessions: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum WorkspaceError {
    #[error("completed session count overflowed")]
    CompletedSessionOverflow,
}

impl WorkspaceSnapshot {
    pub fn defaults() -> Self {
        let settings = FocusSettings::default();
        let priorities = [
            Priority::new(1, "Finish project outline"),
            Priority::new(2, "Reply to design notes"),
            Priority::new(3, "Plan Monday's top three"),
        ]
        .into_iter()
        .filter_map(Result::ok)
        .collect();
        Self {
            settings,
            session: FocusSessionState::ready(settings.session_minutes() as u64 * 60),
            priorities,
            completed_sessions: 2,
        }
    }

    pub fn record_completed_session(&mut self) -> Result<(), WorkspaceError> {
        self.completed_sessions = self
            .completed_sessions
            .checked_add(1)
            .ok_or(WorkspaceError::CompletedSessionOverflow)?;
        Ok(())
    }

    pub fn completed_priorities(&self) -> usize {
        self.priorities.iter().filter(|priority| priority.is_completed()).count()
    }
}

impl Default for WorkspaceSnapshot {
    fn default() -> Self {
        Self::defaults()
    }
}
