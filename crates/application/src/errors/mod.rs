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
