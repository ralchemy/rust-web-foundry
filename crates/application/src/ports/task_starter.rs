use crate::StartTaskMutationError;
use domain::{Task, TaskId, TaskRevision};
use std::future::Future;

pub trait TaskStarter: Clone + Send + Sync + 'static {
    /// Atomically reconstructs the current Task, checks the caller-observed revision,
    /// invokes `Task::start` exactly once, and persists the resulting snapshot.
    fn start(
        &self,
        task_id: &TaskId,
        expected_revision: TaskRevision,
    ) -> impl Future<Output = Result<Task, StartTaskMutationError>> + Send;
}
