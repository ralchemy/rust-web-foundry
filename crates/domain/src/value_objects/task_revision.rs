use crate::TaskRevisionError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaskRevision(u64);

impl TaskRevision {
    pub fn initial() -> Self {
        Self(1)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

impl TryFrom<u64> for TaskRevision {
    type Error = TaskRevisionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        (value > 0).then_some(Self(value)).ok_or(TaskRevisionError)
    }
}
