use crate::TaskPolicyError;
use domain::{AssigneeId, TaskDescription, TaskEstimateMinutes, TaskPriority, TaskTitle};
use std::future::Future;

#[derive(Clone, Copy)]
pub struct TaskPolicyInput<'a> {
    pub title: &'a TaskTitle,
    pub description: Option<&'a TaskDescription>,
    pub priority: TaskPriority,
    pub assignee_id: Option<AssigneeId>,
    pub estimate_minutes: Option<TaskEstimateMinutes>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPolicyDecision {
    Allowed,
    Rejected,
}

pub trait TaskPolicy: Clone + Send + Sync + 'static {
    fn evaluate(
        &self,
        input: TaskPolicyInput<'_>,
    ) -> impl Future<Output = Result<TaskPolicyDecision, TaskPolicyError>> + Send;
}
