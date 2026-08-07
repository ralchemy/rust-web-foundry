use crate::TaskRepositoryError;
use domain::Task;
use std::future::Future;

pub trait TaskRepository: Clone + Send + Sync + 'static {
    fn insert(&self, task: &Task) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send;
}
