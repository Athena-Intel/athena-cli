pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A persisted per-tool override in a workspace Tool Registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceToolRegistryToolOut {
    /// The approval policy applied to the tool
    pub approval_mode: ApprovalMode,
    /// Whether the tool is enabled
    #[serde(default)]
    pub enabled: bool,
    /// The canonical tool identifier
    #[serde(default)]
    pub tool_id: String,
}

impl WorkspaceToolRegistryToolOut {
    pub fn builder() -> WorkspaceToolRegistryToolOutBuilder {
        <WorkspaceToolRegistryToolOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceToolRegistryToolOutBuilder {
    approval_mode: Option<ApprovalMode>,
    enabled: Option<bool>,
    tool_id: Option<String>,
}

impl WorkspaceToolRegistryToolOutBuilder {
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

    /// Consumes the builder and constructs a [`WorkspaceToolRegistryToolOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_mode`](WorkspaceToolRegistryToolOutBuilder::approval_mode)
    /// - [`enabled`](WorkspaceToolRegistryToolOutBuilder::enabled)
    /// - [`tool_id`](WorkspaceToolRegistryToolOutBuilder::tool_id)
    pub fn build(self) -> Result<WorkspaceToolRegistryToolOut, BuildError> {
        Ok(WorkspaceToolRegistryToolOut {
            approval_mode: self.approval_mode.ok_or_else(|| BuildError::missing_field("approval_mode"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
        })
    }
}
