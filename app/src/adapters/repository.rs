use std::sync::{Arc, Mutex};

use focus_flow_core::{RepositoryError, WorkspaceRepository, WorkspaceSnapshot};

#[derive(Clone, Default)]
pub struct InMemoryWorkspaceRepository {
    snapshot: Arc<Mutex<WorkspaceSnapshot>>,
}

impl InMemoryWorkspaceRepository {
    pub fn new(snapshot: WorkspaceSnapshot) -> Self {
        Self { snapshot: Arc::new(Mutex::new(snapshot)) }
    }
}

impl WorkspaceRepository for InMemoryWorkspaceRepository {
    fn load(&self) -> Result<WorkspaceSnapshot, RepositoryError> {
        self.snapshot
            .lock()
            .map(|snapshot| snapshot.clone())
            .map_err(|_| RepositoryError::Failed("in-memory repository lock poisoned".to_owned()))
    }

    fn save(&mut self, snapshot: &WorkspaceSnapshot) -> Result<(), RepositoryError> {
        self.snapshot
            .lock()
            .map(|mut current| *current = snapshot.clone())
            .map_err(|_| RepositoryError::Failed("in-memory repository lock poisoned".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_snapshot() {
        let mut repository = InMemoryWorkspaceRepository::new(WorkspaceSnapshot::default());
        let changed = WorkspaceSnapshot { completed_sessions: 9, ..WorkspaceSnapshot::default() };
        repository.save(&changed).expect("save succeeds");
        assert_eq!(repository.load().expect("load succeeds").completed_sessions, 9);
    }
}
