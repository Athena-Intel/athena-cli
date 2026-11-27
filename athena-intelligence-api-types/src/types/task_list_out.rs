pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A workspace's tasks the caller can open, soonest due first (undated last).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskListOut {
    /// At most `limit` tasks; each carries no links and a null can_edit.
    #[serde(default)]
    pub items: Vec<TaskOut>,
}

impl TaskListOut {
    pub fn builder() -> TaskListOutBuilder {
        <TaskListOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskListOutBuilder {
    items: Option<Vec<TaskOut>>,
}

impl TaskListOutBuilder {
    pub fn items(mut self, value: Vec<TaskOut>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskListOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](TaskListOutBuilder::items)
    pub fn build(self) -> Result<TaskListOut, BuildError> {
        Ok(TaskListOut {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
