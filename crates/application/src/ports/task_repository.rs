use crate::{StartTaskMutationError, TaskRepositoryError};
use domain::{Task, TaskId, TaskRevision};
use std::future::Future;

pub trait TaskRepository: Clone + Send + Sync + 'static {
    fn insert(&self, task: &Task) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send;

    fn find(
        &self,
        task_id: &TaskId,
    ) -> impl Future<Output = Result<Option<Task>, TaskRepositoryError>> + Send;

    /// Atomically reconstructs the current Task, checks the caller-observed revision,
    /// invokes `Task::start` exactly once, and persists the resulting snapshot.
    ///
    /// Adapters must distinguish missing rows, stale revisions, rejected Domain
    /// transitions, corrupt records, and unavailable persistence.
    fn start(
        &self,
        task_id: &TaskId,
        expected_revision: TaskRevision,
    ) -> impl Future<Output = Result<Task, StartTaskMutationError>> + Send;
}
