use super::{
    commands::Command,
    events::{Event, SessionCompletionReason},
    ports::Clock,
};
use crate::{
    FocusSessionState, FocusSettings, Priority, PriorityError, PriorityId, SessionError,
    SettingsError, WorkspaceError, WorkspaceSnapshot,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    pub snapshot: WorkspaceSnapshot,
    pub events: Vec<Event>,
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    Session(#[from] SessionError),
    #[error(transparent)]
    Priority(#[from] PriorityError),
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
}

pub struct WorkspaceService<C> {
    snapshot: WorkspaceSnapshot,
    clock: C,
    next_priority_id: PriorityId,
}

impl<C: Clock> WorkspaceService<C> {
    pub fn new(snapshot: WorkspaceSnapshot, clock: C) -> Self {
        let next_priority_id =
            snapshot.priorities.iter().map(Priority::id).max().unwrap_or(0).saturating_add(1);
        Self { snapshot, clock, next_priority_id }
    }

    pub fn snapshot(&self) -> &WorkspaceSnapshot {
        &self.snapshot
    }

    pub fn execute(&mut self, command: Command) -> Result<CommandResult, CommandError> {
        let now = self.clock.now_ms();
        let mut events = Vec::new();
        match command {
            Command::StartSession => {
                let current = self.snapshot.session.refresh(now);
                if current.is_completed() {
                    self.snapshot.session = FocusSessionState::reset(
                        self.snapshot.settings.session_minutes() as u64 * 60,
                    );
                } else {
                    self.snapshot.session = current;
                }
                self.snapshot.session = self.snapshot.session.start(now)?;
                events.push(Event::SessionStarted);
            }
            Command::PauseSession => {
                self.snapshot.session = self.snapshot.session.pause(now)?;
                if self.snapshot.session.is_completed() {
                    self.snapshot.record_completed_session()?;
                    events.push(Event::SessionCompleted {
                        reason: SessionCompletionReason::PausedAfterExpiry,
                    });
                } else {
                    events.push(Event::SessionPaused);
                }
            }
            Command::ResetSession => {
                self.snapshot.session =
                    FocusSessionState::reset(self.snapshot.settings.session_minutes() as u64 * 60);
                events.push(Event::SessionReset);
            }
            Command::RefreshSession => {
                let before = self.snapshot.session;
                self.snapshot.session = before.refresh(now);
                events.push(Event::SessionRefreshed);
                if !before.is_completed() && self.snapshot.session.is_completed() {
                    self.snapshot.record_completed_session()?;
                    events.push(Event::SessionCompleted {
                        reason: SessionCompletionReason::DeadlineReached,
                    });
                }
            }
            Command::UpdateSettings { session_minutes, daily_goal_minutes } => {
                self.snapshot.settings = FocusSettings::new(session_minutes, daily_goal_minutes)?;
                if matches!(self.snapshot.session, FocusSessionState::Ready { .. }) {
                    self.snapshot.session = FocusSessionState::reset(session_minutes as u64 * 60);
                }
                events.push(Event::SettingsUpdated);
            }
            Command::ReplacePriority { id, title } => {
                let priority = self
                    .snapshot
                    .priorities
                    .iter_mut()
                    .find(|priority| priority.id() == id)
                    .ok_or(PriorityError::NotFound(id))?;
                priority.rename(title)?;
                events.push(Event::PriorityReplaced { id });
            }
            Command::AddPriority { title } => {
                if self.snapshot.priorities.len() >= crate::domain::priority::MAX_PRIORITIES {
                    return Err(PriorityError::LimitReached.into());
                }
                let id = self.next_priority_id;
                self.next_priority_id = self.next_priority_id.saturating_add(1);
                self.snapshot.priorities.push(Priority::new(id, title)?);
                events.push(Event::PriorityAdded { id });
            }
            Command::CompletePriority { id } => {
                let priority = self
                    .snapshot
                    .priorities
                    .iter_mut()
                    .find(|priority| priority.id() == id)
                    .ok_or(PriorityError::NotFound(id))?;
                priority.complete();
                events.push(Event::PriorityCompleted { id });
            }
        }
        events.push(Event::StateChanged { session: self.snapshot.session });
        Ok(CommandResult { snapshot: self.snapshot.clone(), events })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::Clock;
    use std::{cell::Cell, rc::Rc};

    #[derive(Clone)]
    struct FakeClock(Rc<Cell<u64>>);
    impl Clock for FakeClock {
        fn now_ms(&self) -> u64 {
            self.0.get()
        }
    }

    #[test]
    fn navigation_like_snapshot_access_preserves_running_session() {
        let clock = FakeClock(Rc::new(Cell::new(0)));
        let mut service = WorkspaceService::new(WorkspaceSnapshot::default(), clock.clone());
        service.execute(Command::StartSession).expect("start");
        clock.0.set(90_000);
        let result = service.execute(Command::RefreshSession).expect("refresh");
        assert_eq!(result.snapshot.session.remaining_seconds(90_000), 25 * 60 - 90);
        assert!(result.events.iter().any(|event| matches!(event, Event::StateChanged { .. })));
    }

    #[test]
    fn priorities_and_settings_are_validated() {
        let clock = FakeClock(Rc::new(Cell::new(0)));
        let mut service = WorkspaceService::new(WorkspaceSnapshot::default(), clock);
        assert!(
            service
                .execute(Command::UpdateSettings { session_minutes: 4, daily_goal_minutes: 180 })
                .is_err()
        );
        assert!(
            service.execute(Command::ReplacePriority { id: 1, title: "New title".into() }).is_ok()
        );
        assert!(service.execute(Command::CompletePriority { id: 1 }).is_ok());
        assert!(service.snapshot().priorities[0].is_completed());
    }

    #[test]
    fn refresh_records_completion_once_after_a_clock_jump() {
        let clock = FakeClock(Rc::new(Cell::new(0)));
        let mut service = WorkspaceService::new(WorkspaceSnapshot::default(), clock.clone());
        let initial_count = service.snapshot().completed_sessions;
        service.execute(Command::StartSession).expect("start");

        clock.0.set(25 * 60 * 1_000);
        let result = service.execute(Command::RefreshSession).expect("refresh");
        assert!(result.events.iter().any(|event| matches!(
            event,
            Event::SessionCompleted { reason: SessionCompletionReason::DeadlineReached }
        )));
        assert_eq!(service.snapshot().completed_sessions, initial_count + 1);

        service.execute(Command::RefreshSession).expect("repeated refresh");
        assert_eq!(service.snapshot().completed_sessions, initial_count + 1);
    }

    #[test]
    fn adding_a_fourth_priority_is_rejected_without_mutating_the_snapshot() {
        let clock = FakeClock(Rc::new(Cell::new(0)));
        let mut service = WorkspaceService::new(WorkspaceSnapshot::default(), clock);
        let before = service.snapshot().priorities.clone();

        let error = service
            .execute(Command::AddPriority { title: "A fourth priority".to_owned() })
            .expect_err("the prototype keeps the three-priority limit");

        assert!(matches!(error, CommandError::Priority(PriorityError::LimitReached)));
        assert_eq!(service.snapshot().priorities, before);
    }

    #[test]
    fn changing_settings_does_not_restart_a_running_session() {
        let clock = FakeClock(Rc::new(Cell::new(0)));
        let mut service = WorkspaceService::new(WorkspaceSnapshot::default(), clock.clone());
        service.execute(Command::StartSession).expect("start");
        clock.0.set(10_000);
        service
            .execute(Command::UpdateSettings { session_minutes: 50, daily_goal_minutes: 240 })
            .expect("valid settings");

        assert_eq!(service.snapshot().settings.session_minutes(), 50);
        assert_eq!(service.snapshot().session.remaining_seconds(10_000), 25 * 60 - 10);
    }
}
