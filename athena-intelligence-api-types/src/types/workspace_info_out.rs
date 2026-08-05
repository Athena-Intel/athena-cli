pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Workspace summary returned in user info.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceInfoOut {
    /// Display name of the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's role in this workspace (owner, manager, user)
    #[serde(default)]
    pub role: String,
    /// Unique workspace identifier
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceInfoOut {
    pub fn builder() -> WorkspaceInfoOutBuilder {
        <WorkspaceInfoOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceInfoOutBuilder {
    name: Option<String>,
    role: Option<String>,
    workspace_id: Option<String>,
}

impl WorkspaceInfoOutBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceInfoOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](WorkspaceInfoOutBuilder::role)
    /// - [`workspace_id`](WorkspaceInfoOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceInfoOut, BuildError> {
        Ok(WorkspaceInfoOut {
            name: self.name,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
