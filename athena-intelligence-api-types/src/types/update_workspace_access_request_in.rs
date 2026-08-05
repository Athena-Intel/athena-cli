pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceAccessRequestIn {
    /// Permission level to grant to the entire workspace. Set to 'view' or 'edit'.
    pub workspace_access: WorkspaceShareAccess,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_access`](UpdateWorkspaceAccessRequestInBuilder::workspace_access)
    pub fn build(self) -> Result<UpdateWorkspaceAccessRequestIn, BuildError> {
        Ok(UpdateWorkspaceAccessRequestIn {
            workspace_access: self.workspace_access.ok_or_else(|| BuildError::missing_field("workspace_access"))?,
        })
    }
}

