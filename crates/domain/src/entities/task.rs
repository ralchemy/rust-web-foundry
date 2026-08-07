use crate::{TaskId, TaskTitle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
}

impl Task {
    pub fn new(title: TaskTitle) -> Self {
        Self {
            id: TaskId::new(),
            title,
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_has_a_unique_canonical_id() {
        let title = TaskTitle::parse("Ship template").unwrap();

        let first = Task::new(title.clone());
        let second = Task::new(title);

        assert_ne!(first.id(), second.id());
        assert_eq!(first.id().to_string().len(), 26);
        assert!(
            first
                .id()
                .to_string()
                .chars()
                .all(|character| !character.is_ascii_lowercase())
        );
        assert_eq!(first.title().as_str(), "Ship template");
    }
}
