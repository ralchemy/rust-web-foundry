#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskIdError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssigneeIdError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskTitleError {
    Empty,
    TooLong,
    ControlCharacter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskDescriptionError {
    Empty,
    TooLong,
    ControlCharacter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskPriorityError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskStatusError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskEstimateMinutesError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskRevisionError;
