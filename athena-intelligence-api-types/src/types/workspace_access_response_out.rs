pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for updating workspace access on an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceAccessResponseOut {
    /// ID of the asset
    #[serde(default)]
    pub asset_id: String,
    /// Workspace access level that was set ('view' or 'edit')
    #[serde(default)]
    pub workspace_access: String,
}

impl WorkspaceAccessResponseOut {
    pub fn builder() -> WorkspaceAccessResponseOutBuilder {
        <WorkspaceAccessResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAccessResponseOutBuilder {
    asset_id: Option<String>,
    workspace_access: Option<String>,
}

impl WorkspaceAccessResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn workspace_access(mut self, value: impl Into<String>) -> Self {
        self.workspace_access = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAccessResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](WorkspaceAccessResponseOutBuilder::asset_id)
    /// - [`workspace_access`](WorkspaceAccessResponseOutBuilder::workspace_access)
    pub fn build(self) -> Result<WorkspaceAccessResponseOut, BuildError> {
        Ok(WorkspaceAccessResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            workspace_access: self.workspace_access.ok_or_else(|| BuildError::missing_field("workspace_access"))?,
        })
    }
}
