#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPolicyError {
    Unavailable,
    BadResponse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskRepositoryError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadinessError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreateTaskError {
    InvalidTitle,
    PolicyRejected,
    PolicyUnavailable,
    PolicyBadResponse,
    Persistence,
}
