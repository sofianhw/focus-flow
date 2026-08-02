use thiserror::Error;

pub const MAX_PRIORITIES: usize = 3;
pub type PriorityId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Priority {
    id: PriorityId,
    title: String,
    completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PriorityError {
    #[error("priority title cannot be empty")]
    EmptyTitle,
    #[error("a workspace can contain at most {MAX_PRIORITIES} priorities")]
    LimitReached,
    #[error("priority {0} was not found")]
    NotFound(PriorityId),
}

impl Priority {
    pub fn new(id: PriorityId, title: impl Into<String>) -> Result<Self, PriorityError> {
        let title = title.into().trim().to_owned();
        if title.is_empty() {
            return Err(PriorityError::EmptyTitle);
        }
        Ok(Self { id, title, completed: false })
    }

    pub const fn id(&self) -> PriorityId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn rename(&mut self, title: impl Into<String>) -> Result<(), PriorityError> {
        let title = title.into().trim().to_owned();
        if title.is_empty() {
            return Err(PriorityError::EmptyTitle);
        }
        self.title = title;
        Ok(())
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_idempotently_completes() {
        assert_eq!(Priority::new(1, "  "), Err(PriorityError::EmptyTitle));
        let mut priority = Priority::new(1, "Outline project").expect("valid priority");
        priority.complete();
        priority.complete();
        assert!(priority.is_completed());
    }
}
