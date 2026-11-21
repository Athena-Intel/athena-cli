pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Who a task is assigned to: a person or an agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskAssigneeOut {
    /// Their name, when the caller may see it (an agent's only when they can open it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// person or agent.
    pub kind: TaskAssigneeOutKind,
    /// user_… for a person, agent_asset_… for an agent.
    #[serde(default)]
    pub r#ref: String,
}

impl TaskAssigneeOut {
    pub fn builder() -> TaskAssigneeOutBuilder {
        <TaskAssigneeOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskAssigneeOutBuilder {
    display_name: Option<String>,
    kind: Option<TaskAssigneeOutKind>,
    r#ref: Option<String>,
}

impl TaskAssigneeOutBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TaskAssigneeOutKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskAssigneeOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TaskAssigneeOutBuilder::kind)
    /// - [`r#ref`](TaskAssigneeOutBuilder::r#ref)
    pub fn build(self) -> Result<TaskAssigneeOut, BuildError> {
        Ok(TaskAssigneeOut {
            display_name: self.display_name,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            r#ref: self.r#ref.ok_or_else(|| BuildError::missing_field("r#ref"))?,
        })
    }
}
