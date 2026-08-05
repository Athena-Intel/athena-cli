pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceConfigurationRequestIn {
    /// Workspace disclaimer settings to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_disclaimer: Option<UpdateWorkspaceDisclaimerIn>,
}

impl UpdateWorkspaceConfigurationRequestIn {
    pub fn builder() -> UpdateWorkspaceConfigurationRequestInBuilder {
        <UpdateWorkspaceConfigurationRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceConfigurationRequestInBuilder {
    workspace_disclaimer: Option<UpdateWorkspaceDisclaimerIn>,
}

impl UpdateWorkspaceConfigurationRequestInBuilder {
    pub fn workspace_disclaimer(mut self, value: UpdateWorkspaceDisclaimerIn) -> Self {
        self.workspace_disclaimer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceConfigurationRequestIn`].
    pub fn build(self) -> Result<UpdateWorkspaceConfigurationRequestIn, BuildError> {
        Ok(UpdateWorkspaceConfigurationRequestIn {
            workspace_disclaimer: self.workspace_disclaimer,
        })
    }
}

