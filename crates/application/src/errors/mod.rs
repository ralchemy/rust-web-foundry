use domain::TaskTransitionError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPolicyError {
    Unavailable,
    BadResponse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskRepositoryError {
    Unavailable,
    CorruptRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartTaskMutationError {
    NotFound,
    Conflict,
    Rejected(TaskTransitionError),
    Unavailable,
    CorruptRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadinessError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreateTaskError {
    PolicyRejected,
    PolicyUnavailable,
    PolicyBadResponse,
    Persistence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GetTaskError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartTaskError {
    NotFound,
    Conflict,
    Rejected(TaskTransitionError),
    Persistence,
}
