use crate::{GetTaskError, TaskRepository, TaskView};
use domain::TaskId;

#[derive(Clone)]
pub struct GetTask<R> {
    repository: R,
}

impl<R> GetTask<R>
where
    R: TaskRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, task_id: TaskId) -> Result<Option<TaskView>, GetTaskError> {
        self.repository
            .find(&task_id)
            .await
            .map(|task| task.map(TaskView::from))
            .map_err(|_| GetTaskError)
    }
}
