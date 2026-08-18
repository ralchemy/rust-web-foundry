use crate::{
    AssigneeId, TaskDescription, TaskEstimateMinutes, TaskId, TaskPriority, TaskRevision,
    TaskStatus, TaskTitle,
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
            id: TaskId::new(),
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

    #[test]
    fn creation_establishes_system_owned_state() {
        let task = Task::create(NewTask {
            title: "Ship template".parse().unwrap(),
            description: Some("Document every conversion".parse().unwrap()),
            priority: TaskPriority::High,
            assignee_id: None,
            estimate_minutes: Some(90.try_into().unwrap()),
        });

        assert_eq!(task.status(), TaskStatus::Pending);
        assert_eq!(task.revision(), TaskRevision::initial());
        assert_eq!(task.estimate_minutes().unwrap().get(), 90);
    }

    #[test]
    fn reconstitution_preserves_validated_persisted_state() {
        let created = Task::create(NewTask {
            title: "Ship template".parse().unwrap(),
            description: None,
            priority: TaskPriority::Normal,
            assignee_id: None,
            estimate_minutes: None,
        });
        let snapshot = created.clone().into_snapshot();

        assert_eq!(Task::reconstitute(snapshot), created);
    }
}
