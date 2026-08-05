pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceToolRegistryRequestIn {
    /// Default visibility for tools without an explicit override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_visibility: Option<DefaultVisibility>,
    /// An optional update to one tool override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<UpdateWorkspaceToolRegistryToolIn>,
}

impl UpdateWorkspaceToolRegistryRequestIn {
    pub fn builder() -> UpdateWorkspaceToolRegistryRequestInBuilder {
        <UpdateWorkspaceToolRegistryRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceToolRegistryRequestInBuilder {
    default_visibility: Option<DefaultVisibility>,
    tool: Option<UpdateWorkspaceToolRegistryToolIn>,
}

impl UpdateWorkspaceToolRegistryRequestInBuilder {
    pub fn default_visibility(mut self, value: DefaultVisibility) -> Self {
        self.default_visibility = Some(value);
        self
    }

    pub fn tool(mut self, value: UpdateWorkspaceToolRegistryToolIn) -> Self {
        self.tool = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceToolRegistryRequestIn`].
    pub fn build(self) -> Result<UpdateWorkspaceToolRegistryRequestIn, BuildError> {
        Ok(UpdateWorkspaceToolRegistryRequestIn {
            default_visibility: self.default_visibility,
            tool: self.tool,
        })
    }
}

