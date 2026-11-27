pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How far along a task's sub-tasks are, among those the caller can open.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskSubtaskProgressOut {
    /// How many of them are done.
    #[serde(default)]
    pub done: i64,
    /// How many there are.
    #[serde(default)]
    pub total: i64,
}

impl TaskSubtaskProgressOut {
    pub fn builder() -> TaskSubtaskProgressOutBuilder {
        <TaskSubtaskProgressOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskSubtaskProgressOutBuilder {
    done: Option<i64>,
    total: Option<i64>,
}

impl TaskSubtaskProgressOutBuilder {
    pub fn done(mut self, value: i64) -> Self {
        self.done = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskSubtaskProgressOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`done`](TaskSubtaskProgressOutBuilder::done)
    /// - [`total`](TaskSubtaskProgressOutBuilder::total)
    pub fn build(self) -> Result<TaskSubtaskProgressOut, BuildError> {
        Ok(TaskSubtaskProgressOut {
            done: self.done.ok_or_else(|| BuildError::missing_field("done"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
