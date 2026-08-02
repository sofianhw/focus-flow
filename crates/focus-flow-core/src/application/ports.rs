use crate::WorkspaceSnapshot;
use thiserror::Error;

pub trait Clock {
    fn now_ms(&self) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RepositoryError {
    #[error("workspace repository failed: {0}")]
    Failed(String),
}

pub trait WorkspaceRepository {
    fn load(&self) -> Result<WorkspaceSnapshot, RepositoryError>;
    fn save(&mut self, snapshot: &WorkspaceSnapshot) -> Result<(), RepositoryError>;
}
