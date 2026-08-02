#![deny(clippy::unwrap_used)]

pub mod application;
pub mod domain;

pub use application::commands::Command;
pub use application::events::{Event, SessionCompletionReason};
pub use application::ports::{Clock, RepositoryError, WorkspaceRepository};
pub use application::service::{CommandError, CommandResult, WorkspaceService};
pub use domain::focus_session::{FocusSessionState, SessionError};
pub use domain::focus_settings::{FocusSettings, SettingsError};
pub use domain::priority::{Priority, PriorityError, PriorityId};
pub use domain::workspace::{WorkspaceError, WorkspaceSnapshot};
