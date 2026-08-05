pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Persisted Tool Registry policy for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceToolRegistryResponseOut {
    /// Default visibility for tools without an explicit override
    pub default_visibility: DefaultVisibility,
    /// Persisted per-tool overrides, sorted by tool ID
    #[serde(default)]
    pub tools: Vec<WorkspaceToolRegistryToolOut>,
    /// The workspace ID
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceToolRegistryResponseOut {
    pub fn builder() -> WorkspaceToolRegistryResponseOutBuilder {
        <WorkspaceToolRegistryResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceToolRegistryResponseOutBuilder {
    default_visibility: Option<DefaultVisibility>,
    tools: Option<Vec<WorkspaceToolRegistryToolOut>>,
    workspace_id: Option<String>,
}

impl WorkspaceToolRegistryResponseOutBuilder {
    pub fn default_visibility(mut self, value: DefaultVisibility) -> Self {
        self.default_visibility = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<WorkspaceToolRegistryToolOut>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceToolRegistryResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_visibility`](WorkspaceToolRegistryResponseOutBuilder::default_visibility)
    /// - [`tools`](WorkspaceToolRegistryResponseOutBuilder::tools)
    /// - [`workspace_id`](WorkspaceToolRegistryResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceToolRegistryResponseOut, BuildError> {
        Ok(WorkspaceToolRegistryResponseOut {
            default_visibility: self.default_visibility.ok_or_else(|| BuildError::missing_field("default_visibility"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
