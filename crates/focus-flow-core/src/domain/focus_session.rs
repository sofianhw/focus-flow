use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusSessionState {
    Ready { duration_seconds: u64 },
    Running { deadline_ms: u64 },
    Paused { remaining_seconds: u64 },
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SessionError {
    #[error("a session can only be started when ready, paused, or completed")]
    CannotStart,
    #[error("a session can only be paused while it is running")]
    CannotPause,
    #[error("session duration overflowed")]
    DurationOverflow,
}

impl FocusSessionState {
    pub const fn ready(duration_seconds: u64) -> Self {
        Self::Ready { duration_seconds }
    }

    pub const fn remaining_seconds(self, now_ms: u64) -> u64 {
        match self {
            Self::Ready { duration_seconds }
            | Self::Paused { remaining_seconds: duration_seconds } => duration_seconds,
            Self::Running { deadline_ms } => deadline_ms.saturating_sub(now_ms).div_ceil(1000),
            Self::Completed => 0,
        }
    }

    pub fn start(self, now_ms: u64) -> Result<Self, SessionError> {
        let seconds = match self {
            Self::Ready { duration_seconds }
            | Self::Paused { remaining_seconds: duration_seconds } => duration_seconds,
            Self::Completed => 0,
            Self::Running { .. } => return Err(SessionError::CannotStart),
        };
        let duration_ms = seconds.checked_mul(1000).ok_or(SessionError::DurationOverflow)?;
        Ok(Self::Running { deadline_ms: now_ms.saturating_add(duration_ms) })
    }

    pub fn pause(self, now_ms: u64) -> Result<Self, SessionError> {
        match self {
            Self::Running { deadline_ms } => {
                let remaining = deadline_ms.saturating_sub(now_ms).div_ceil(1000);
                if remaining == 0 {
                    Ok(Self::Completed)
                } else {
                    Ok(Self::Paused { remaining_seconds: remaining })
                }
            }
            _ => Err(SessionError::CannotPause),
        }
    }

    pub fn refresh(self, now_ms: u64) -> Self {
        match self {
            Self::Running { deadline_ms } if now_ms >= deadline_ms => Self::Completed,
            other => other,
        }
    }

    pub const fn reset(duration_seconds: u64) -> Self {
        Self::Ready { duration_seconds }
    }

    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running { .. })
    }

    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deadline_survives_large_clock_jump() {
        let running = FocusSessionState::ready(90).start(1_000).expect("valid start");
        assert_eq!(running.remaining_seconds(31_000), 60);
        assert_eq!(running.refresh(91_000), FocusSessionState::Completed);
    }

    #[test]
    fn pause_resume_and_reset_are_deterministic() {
        let running = FocusSessionState::ready(60).start(10_000).expect("valid start");
        let paused = running.pause(25_000).expect("running can pause");
        assert_eq!(paused, FocusSessionState::Paused { remaining_seconds: 45 });
        let resumed = paused.start(40_000).expect("paused can resume");
        assert_eq!(resumed.remaining_seconds(40_000), 45);
        assert_eq!(FocusSessionState::reset(60).remaining_seconds(0), 60);
    }

    #[test]
    fn expired_pause_completes_and_repeated_completion_is_safe() {
        let running = FocusSessionState::ready(5).start(0).expect("valid start");
        assert_eq!(running.pause(5_000), Ok(FocusSessionState::Completed));
        assert_eq!(FocusSessionState::Completed.refresh(10_000), FocusSessionState::Completed);
    }
}
