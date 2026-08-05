pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A partial update to one workspace tool override.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceToolRegistryToolIn {
    /// Approval policy for the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<ApprovalMode>,
    /// Whether to enable the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The canonical tool identifier
    #[serde(default)]
    pub tool_id: String,
}

impl UpdateWorkspaceToolRegistryToolIn {
    pub fn builder() -> UpdateWorkspaceToolRegistryToolInBuilder {
        <UpdateWorkspaceToolRegistryToolInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceToolRegistryToolInBuilder {
    approval_mode: Option<ApprovalMode>,
    enabled: Option<bool>,
    tool_id: Option<String>,
}

impl UpdateWorkspaceToolRegistryToolInBuilder {
    pub fn approval_mode(mut self, value: ApprovalMode) -> Self {
        self.approval_mode = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceToolRegistryToolIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_id`](UpdateWorkspaceToolRegistryToolInBuilder::tool_id)
    pub fn build(self) -> Result<UpdateWorkspaceToolRegistryToolIn, BuildError> {
        Ok(UpdateWorkspaceToolRegistryToolIn {
            approval_mode: self.approval_mode,
            enabled: self.enabled,
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
        })
    }
}
