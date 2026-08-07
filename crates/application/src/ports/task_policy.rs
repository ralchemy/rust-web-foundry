use crate::TaskPolicyError;
use domain::TaskTitle;
use std::future::Future;

pub trait TaskPolicy: Clone + Send + Sync + 'static {
    fn is_allowed(
        &self,
        title: &TaskTitle,
    ) -> impl Future<Output = Result<bool, TaskPolicyError>> + Send;
}
