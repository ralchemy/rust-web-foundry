use crate::TaskDescriptionError;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskDescription(String);

impl TaskDescription {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for TaskDescription {
    type Err = TaskDescriptionError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
        {
            return Err(TaskDescriptionError::ControlCharacter);
        }

        let description = raw.trim();
        match description.chars().count() {
            0 => Err(TaskDescriptionError::Empty),
            1..=2_000 => Ok(Self(description.to_owned())),
            _ => Err(TaskDescriptionError::TooLong),
        }
    }
}

impl AsRef<str> for TaskDescription {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TaskDescription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_text_and_keeps_readable_line_breaks() {
        let description = "  first line\nsecond line  "
            .parse::<TaskDescription>()
            .unwrap();

        assert_eq!(description.as_str(), "first line\nsecond line");
    }

    #[test]
    fn rejects_empty_overlong_and_non_text_control_values() {
        assert_eq!(
            "   ".parse::<TaskDescription>(),
            Err(TaskDescriptionError::Empty)
        );
        assert_eq!(
            "x".repeat(2_001).parse::<TaskDescription>(),
            Err(TaskDescriptionError::TooLong)
        );
        assert_eq!(
            "bad\0value".parse::<TaskDescription>(),
            Err(TaskDescriptionError::ControlCharacter)
        );
    }
}
