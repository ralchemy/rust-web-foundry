use domain::{
    AssigneeId, Task, TaskDescription, TaskEstimateMinutes, TaskId, TaskPriority, TaskRevision,
    TaskStatus, TaskTitle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskView {
    pub id: TaskId,
    pub title: TaskTitle,
    pub description: Option<TaskDescription>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub assignee_id: Option<AssigneeId>,
    pub estimate_minutes: Option<TaskEstimateMinutes>,
    pub revision: TaskRevision,
}

impl TaskView {
    pub fn id(&self) -> TaskId {
        self.id
    }
}

impl From<Task> for TaskView {
    fn from(task: Task) -> Self {
        let snapshot = task.into_snapshot();
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
}
