use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskTitleError {
    Empty,
    TooLong,
    ControlCharacter,
}

impl fmt::Display for TaskTitleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("task title is invalid")
    }
}

impl Error for TaskTitleError {}
