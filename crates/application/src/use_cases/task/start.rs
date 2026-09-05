use crate::{StartTaskError, StartTaskMutationError, TaskRepository, TaskView};
use domain::{TaskId, TaskRevision};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StartTaskCommand {
    pub task_id: TaskId,
    pub expected_revision: TaskRevision,
}

#[derive(Clone)]
pub struct StartTask<R> {
    repository: R,
}

impl<R> StartTask<R>
where
    R: TaskRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: StartTaskCommand) -> Result<TaskView, StartTaskError> {
        self.repository
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
