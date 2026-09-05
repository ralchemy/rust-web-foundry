use crate::{
    AssigneeId, TaskDescription, TaskEstimateMinutes, TaskId, TaskPriority, TaskRevision,
    TaskStatus, TaskTitle, TaskTransitionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    description: Option<TaskDescription>,
    priority: TaskPriority,
    status: TaskStatus,
    assignee_id: Option<AssigneeId>,
    estimate_minutes: Option<TaskEstimateMinutes>,
    revision: TaskRevision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewTask {
    pub title: TaskTitle,
    pub description: Option<TaskDescription>,
    pub priority: TaskPriority,
    pub assignee_id: Option<AssigneeId>,
    pub estimate_minutes: Option<TaskEstimateMinutes>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskSnapshot {
    pub id: TaskId,
    pub title: TaskTitle,
    pub description: Option<TaskDescription>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub assignee_id: Option<AssigneeId>,
    pub estimate_minutes: Option<TaskEstimateMinutes>,
    pub revision: TaskRevision,
}

impl Task {
    pub fn create(input: NewTask) -> Self {
        Self {
            id: TaskId::generate(),
            title: input.title,
            description: input.description,
            priority: input.priority,
            status: TaskStatus::Pending,
            assignee_id: input.assignee_id,
            estimate_minutes: input.estimate_minutes,
            revision: TaskRevision::initial(),
        }
    }

    pub fn reconstitute(snapshot: TaskSnapshot) -> Self {
        Self {
            id: snapshot.id,
            title: snapshot.title,
            description: snapshot.description,
            priority: snapshot.priority,
            status: snapshot.status,
            assignee_id: snapshot.assignee_id,
            estimate_minutes: snapshot.estimate_minutes,
            revision: snapshot.revision,
        }
    }

    pub fn start(&mut self) -> Result<(), TaskTransitionError> {
        if self.status != TaskStatus::Pending {
            return Err(TaskTransitionError::StartRequiresPending);
        }
        let revision = self
            .revision
            .next()
            .ok_or(TaskTransitionError::RevisionExhausted)?;
        self.status = TaskStatus::InProgress;
        self.revision = revision;
        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), TaskTransitionError> {
        if self.status != TaskStatus::InProgress {
            return Err(TaskTransitionError::CompleteRequiresInProgress);
        }
        let revision = self
            .revision
            .next()
            .ok_or(TaskTransitionError::RevisionExhausted)?;
        self.status = TaskStatus::Completed;
        self.revision = revision;
        Ok(())
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }

    pub fn description(&self) -> Option<&TaskDescription> {
        self.description.as_ref()
    }

    pub fn priority(&self) -> TaskPriority {
        self.priority
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }

    pub fn assignee_id(&self) -> Option<AssigneeId> {
        self.assignee_id
    }

    pub fn estimate_minutes(&self) -> Option<TaskEstimateMinutes> {
        self.estimate_minutes
    }

    pub fn revision(&self) -> TaskRevision {
        self.revision
    }

    pub fn into_snapshot(self) -> TaskSnapshot {
        TaskSnapshot {
            id: self.id,
            title: self.title,
            description: self.description,
            priority: self.priority,
            status: self.status,
            assignee_id: self.assignee_id,
            estimate_minutes: self.estimate_minutes,
            revision: self.revision,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task() -> Task {
        Task::create(NewTask {
            title: "Ship template".parse().unwrap(),
            description: Some("Document every conversion".parse().unwrap()),
            priority: TaskPriority::High,
            assignee_id: None,
            estimate_minutes: Some(90.try_into().unwrap()),
        })
    }

    #[test]
    fn creation_establishes_system_owned_state() {
        let task = task();

        assert_eq!(task.status(), TaskStatus::Pending);
        assert_eq!(task.revision(), TaskRevision::initial());
        assert_eq!(task.estimate_minutes().unwrap().get(), 90);
    }

    #[test]
    fn reconstitution_preserves_validated_persisted_state() {
        let created = task();
        let snapshot = created.clone().into_snapshot();

        assert_eq!(Task::reconstitute(snapshot), created);
    }

    #[test]
    fn state_transitions_are_named_domain_operations() {
        let mut task = task();

        task.start().unwrap();
        assert_eq!(task.status(), TaskStatus::InProgress);
        assert_eq!(task.revision().get(), 2);

        task.complete().unwrap();
        assert_eq!(task.status(), TaskStatus::Completed);
        assert_eq!(task.revision().get(), 3);
    }

    #[test]
    fn rejected_transition_does_not_partially_mutate_state() {
        let mut task = task();
        let before = task.clone();

        assert_eq!(
            task.complete(),
            Err(TaskTransitionError::CompleteRequiresInProgress)
        );
        assert_eq!(task, before);
    }
}
