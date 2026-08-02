use thiserror::Error;

pub const MIN_SESSION_MINUTES: u32 = 5;
pub const MAX_SESSION_MINUTES: u32 = 90;
pub const MIN_DAILY_GOAL_MINUTES: u32 = 15;
pub const MAX_DAILY_GOAL_MINUTES: u32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusSettings {
    session_minutes: u32,
    daily_goal_minutes: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SettingsError {
    #[error(
        "session length must be between {MIN_SESSION_MINUTES} and {MAX_SESSION_MINUTES} minutes"
    )]
    SessionMinutesOutOfRange,
    #[error(
        "daily focus goal must be between {MIN_DAILY_GOAL_MINUTES} and {MAX_DAILY_GOAL_MINUTES} minutes"
    )]
    DailyGoalOutOfRange,
}

impl FocusSettings {
    pub fn new(session_minutes: u32, daily_goal_minutes: u32) -> Result<Self, SettingsError> {
        if !(MIN_SESSION_MINUTES..=MAX_SESSION_MINUTES).contains(&session_minutes) {
            return Err(SettingsError::SessionMinutesOutOfRange);
        }
        if !(MIN_DAILY_GOAL_MINUTES..=MAX_DAILY_GOAL_MINUTES).contains(&daily_goal_minutes) {
            return Err(SettingsError::DailyGoalOutOfRange);
        }
        Ok(Self { session_minutes, daily_goal_minutes })
    }

    pub const fn defaults() -> Self {
        Self { session_minutes: 25, daily_goal_minutes: 180 }
    }

    pub const fn session_minutes(self) -> u32 {
        self.session_minutes
    }

    pub const fn daily_goal_minutes(self) -> u32 {
        self.daily_goal_minutes
    }

    pub fn with_session_minutes(self, session_minutes: u32) -> Result<Self, SettingsError> {
        Self::new(session_minutes, self.daily_goal_minutes)
    }

    pub fn with_daily_goal_minutes(self, daily_goal_minutes: u32) -> Result<Self, SettingsError> {
        Self::new(self.session_minutes, daily_goal_minutes)
    }
}

impl Default for FocusSettings {
    fn default() -> Self {
        Self::defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_boundaries_and_rejects_outside_values() {
        assert!(FocusSettings::new(MIN_SESSION_MINUTES, MIN_DAILY_GOAL_MINUTES).is_ok());
        assert!(FocusSettings::new(MAX_SESSION_MINUTES, MAX_DAILY_GOAL_MINUTES).is_ok());
        assert_eq!(
            FocusSettings::new(MIN_SESSION_MINUTES - 1, 30),
            Err(SettingsError::SessionMinutesOutOfRange)
        );
        assert_eq!(
            FocusSettings::new(25, MAX_DAILY_GOAL_MINUTES + 1),
            Err(SettingsError::DailyGoalOutOfRange)
        );
    }
}
