pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWorkspaceMembersInput {
    /// Include each member's workspace role in the listing (default true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_role: Option<bool>,
}

impl ListWorkspaceMembersInput {
    pub fn builder() -> ListWorkspaceMembersInputBuilder {
        <ListWorkspaceMembersInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkspaceMembersInputBuilder {
    include_role: Option<bool>,
}

impl ListWorkspaceMembersInputBuilder {
    pub fn include_role(mut self, value: bool) -> Self {
        self.include_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWorkspaceMembersInput`].
    pub fn build(self) -> Result<ListWorkspaceMembersInput, BuildError> {
        Ok(ListWorkspaceMembersInput {
            include_role: self.include_role,
        })
    }
}

