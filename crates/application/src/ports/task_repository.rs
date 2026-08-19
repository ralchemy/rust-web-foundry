use crate::TaskRepositoryError;
use domain::{Task, TaskId};
use std::future::Future;

pub trait TaskRepository: Clone + Send + Sync + 'static {
    fn insert(&self, task: &Task) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send;

    fn find(
        &self,
        task_id: &TaskId,
    ) -> impl Future<Output = Result<Option<Task>, TaskRepositoryError>> + Send;
}
