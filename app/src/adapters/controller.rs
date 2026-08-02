use std::sync::{Arc, Mutex};

use focus_flow_core::{Command, CommandError, CommandResult, WorkspaceService, WorkspaceSnapshot};

use super::clock::AppClock;

#[derive(Clone)]
pub struct WorkspaceController {
    service: Arc<Mutex<WorkspaceService<AppClock>>>,
}

impl WorkspaceController {
    pub fn new(snapshot: WorkspaceSnapshot) -> Self {
        Self { service: Arc::new(Mutex::new(WorkspaceService::new(snapshot, AppClock))) }
    }

    pub fn snapshot(&self) -> WorkspaceSnapshot {
        match self.service.lock() {
            Ok(service) => service.snapshot().clone(),
            Err(poisoned) => poisoned.into_inner().snapshot().clone(),
        }
    }

    pub fn dispatch(&self, command: Command) -> Result<CommandResult, CommandError> {
        match self.service.lock() {
            Ok(mut service) => service.execute(command),
            Err(poisoned) => poisoned.into_inner().execute(command),
        }
    }
}
