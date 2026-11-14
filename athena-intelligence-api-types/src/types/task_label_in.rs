pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A label to put on a task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskLabelIn {
    /// #RRGGBB. Left out, a label the task already has keeps its color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The label's name (at most 64 characters, matched case-insensitively).
    #[serde(default)]
    pub name: String,
}

impl TaskLabelIn {
    pub fn builder() -> TaskLabelInBuilder {
        <TaskLabelInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskLabelInBuilder {
    color: Option<String>,
    name: Option<String>,
}

impl TaskLabelInBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskLabelIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TaskLabelInBuilder::name)
    pub fn build(self) -> Result<TaskLabelIn, BuildError> {
        Ok(TaskLabelIn {
            color: self.color,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
