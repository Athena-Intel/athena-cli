pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceAccessRequestIn {
    /// Permission level to grant to the entire workspace: 'view' or 'edit'. Set 'none' (or null) to remove the workspace-wide share; people the asset is shared with directly keep their access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<WorkspaceShareAccess>,
}

impl UpdateWorkspaceAccessRequestIn {
    pub fn builder() -> UpdateWorkspaceAccessRequestInBuilder {
        <UpdateWorkspaceAccessRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceAccessRequestInBuilder {
    workspace_access: Option<WorkspaceShareAccess>,
}

impl UpdateWorkspaceAccessRequestInBuilder {
    pub fn workspace_access(mut self, value: WorkspaceShareAccess) -> Self {
        self.workspace_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceAccessRequestIn`].
    pub fn build(self) -> Result<UpdateWorkspaceAccessRequestIn, BuildError> {
        Ok(UpdateWorkspaceAccessRequestIn {
            workspace_access: self.workspace_access,
        })
    }
}

