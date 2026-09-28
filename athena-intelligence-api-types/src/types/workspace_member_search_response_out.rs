pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for the workspace people search.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMemberSearchResponseOut {
    /// People matching the prefix, ordered by email
    #[serde(default)]
    pub members: Vec<WorkspaceMemberOut>,
    /// The normalized search prefix that was applied
    #[serde(default)]
    pub query: String,
    /// The workspace that was searched
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceMemberSearchResponseOut {
    pub fn builder() -> WorkspaceMemberSearchResponseOutBuilder {
        <WorkspaceMemberSearchResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMemberSearchResponseOutBuilder {
    members: Option<Vec<WorkspaceMemberOut>>,
    query: Option<String>,
    workspace_id: Option<String>,
}

impl WorkspaceMemberSearchResponseOutBuilder {
    pub fn members(mut self, value: Vec<WorkspaceMemberOut>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMemberSearchResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`members`](WorkspaceMemberSearchResponseOutBuilder::members)
    /// - [`query`](WorkspaceMemberSearchResponseOutBuilder::query)
    /// - [`workspace_id`](WorkspaceMemberSearchResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceMemberSearchResponseOut, BuildError> {
        Ok(WorkspaceMemberSearchResponseOut {
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
