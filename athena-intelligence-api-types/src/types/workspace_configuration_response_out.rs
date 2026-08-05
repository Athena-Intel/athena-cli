pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for workspace configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceConfigurationResponseOut {
    /// Workspace disclaimer settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_disclaimer: Option<WorkspaceDisclaimerOut>,
    /// The workspace ID
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceConfigurationResponseOut {
    pub fn builder() -> WorkspaceConfigurationResponseOutBuilder {
        <WorkspaceConfigurationResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceConfigurationResponseOutBuilder {
    workspace_disclaimer: Option<WorkspaceDisclaimerOut>,
    workspace_id: Option<String>,
}

impl WorkspaceConfigurationResponseOutBuilder {
    pub fn workspace_disclaimer(mut self, value: WorkspaceDisclaimerOut) -> Self {
        self.workspace_disclaimer = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceConfigurationResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](WorkspaceConfigurationResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceConfigurationResponseOut, BuildError> {
        Ok(WorkspaceConfigurationResponseOut {
            workspace_disclaimer: self.workspace_disclaimer,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
