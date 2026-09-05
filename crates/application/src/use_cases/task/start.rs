use crate::{StartTaskError, StartTaskMutationError, TaskStarter, TaskView};
use domain::{TaskId, TaskRevision};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StartTaskCommand {
    pub task_id: TaskId,
    pub expected_revision: TaskRevision,
}

#[derive(Clone)]
pub struct StartTask<S> {
    starter: S,
}

impl<S> StartTask<S>
where
    S: TaskStarter,
{
    pub fn new(starter: S) -> Self {
        Self { starter }
    }

    pub async fn execute(&self, command: StartTaskCommand) -> Result<TaskView, StartTaskError> {
        self.starter
            .start(&command.task_id, command.expected_revision)
            .await
            .map(TaskView::from)
            .map_err(|error| match error {
                StartTaskMutationError::NotFound => StartTaskError::NotFound,
                StartTaskMutationError::Conflict => StartTaskError::Conflict,
                StartTaskMutationError::Rejected(error) => StartTaskError::Rejected(error),
                StartTaskMutationError::Unavailable | StartTaskMutationError::CorruptRecord => {
                    StartTaskError::Persistence
                }
            })
    }
}
