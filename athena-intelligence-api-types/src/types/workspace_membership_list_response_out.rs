pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of a workspace's membership roster.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMembershipListResponseOut {
    /// Memberships for this page, ordered by user id
    #[serde(default)]
    pub memberships: Vec<WorkspaceMembershipOut>,
    /// Pass as `cursor` to fetch the next page; null on the last page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Workspace the roster belongs to
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceMembershipListResponseOut {
    pub fn builder() -> WorkspaceMembershipListResponseOutBuilder {
        <WorkspaceMembershipListResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMembershipListResponseOutBuilder {
    memberships: Option<Vec<WorkspaceMembershipOut>>,
    next_cursor: Option<String>,
    workspace_id: Option<String>,
}

impl WorkspaceMembershipListResponseOutBuilder {
    pub fn memberships(mut self, value: Vec<WorkspaceMembershipOut>) -> Self {
        self.memberships = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMembershipListResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`memberships`](WorkspaceMembershipListResponseOutBuilder::memberships)
    /// - [`workspace_id`](WorkspaceMembershipListResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceMembershipListResponseOut, BuildError> {
        Ok(WorkspaceMembershipListResponseOut {
            memberships: self.memberships.ok_or_else(|| BuildError::missing_field("memberships"))?,
            next_cursor: self.next_cursor,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
