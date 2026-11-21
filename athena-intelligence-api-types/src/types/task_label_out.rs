pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A label on a task: its name, color and who applied it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskLabelOut {
    /// When it was applied (ISO 8601).
    #[serde(default)]
    pub applied_at: String,
    /// Principal ref of who applied it.
    #[serde(default)]
    pub applied_by_ref: String,
    /// #RRGGBB, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub name: String,
}

impl TaskLabelOut {
    pub fn builder() -> TaskLabelOutBuilder {
        <TaskLabelOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskLabelOutBuilder {
    applied_at: Option<String>,
    applied_by_ref: Option<String>,
    color: Option<String>,
    name: Option<String>,
}

impl TaskLabelOutBuilder {
    pub fn applied_at(mut self, value: impl Into<String>) -> Self {
        self.applied_at = Some(value.into());
        self
    }

    pub fn applied_by_ref(mut self, value: impl Into<String>) -> Self {
        self.applied_by_ref = Some(value.into());
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskLabelOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`applied_at`](TaskLabelOutBuilder::applied_at)
    /// - [`applied_by_ref`](TaskLabelOutBuilder::applied_by_ref)
    /// - [`name`](TaskLabelOutBuilder::name)
    pub fn build(self) -> Result<TaskLabelOut, BuildError> {
        Ok(TaskLabelOut {
            applied_at: self.applied_at.ok_or_else(|| BuildError::missing_field("applied_at"))?,
            applied_by_ref: self.applied_by_ref.ok_or_else(|| BuildError::missing_field("applied_by_ref"))?,
            color: self.color,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
