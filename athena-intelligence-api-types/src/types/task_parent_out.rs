pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The task a sub-task belongs to, read through its own gate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskParentOut {
    /// Its asset id; null when the caller can't open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Its number (T-<n>); null when the caller can't open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// Whether the caller can open it.
    #[serde(default)]
    pub readable: bool,
    /// open or done; null when the caller can't open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskParentOutStatus>,
    /// Its title; null when the caller can't open it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl TaskParentOut {
    pub fn builder() -> TaskParentOutBuilder {
        <TaskParentOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskParentOutBuilder {
    id: Option<String>,
    number: Option<i64>,
    readable: Option<bool>,
    status: Option<TaskParentOutStatus>,
    title: Option<String>,
}

impl TaskParentOutBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn readable(mut self, value: bool) -> Self {
        self.readable = Some(value);
        self
    }

    pub fn status(mut self, value: TaskParentOutStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskParentOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`readable`](TaskParentOutBuilder::readable)
    pub fn build(self) -> Result<TaskParentOut, BuildError> {
        Ok(TaskParentOut {
            id: self.id,
            number: self.number,
            readable: self.readable.ok_or_else(|| BuildError::missing_field("readable"))?,
            status: self.status,
            title: self.title,
        })
    }
}
