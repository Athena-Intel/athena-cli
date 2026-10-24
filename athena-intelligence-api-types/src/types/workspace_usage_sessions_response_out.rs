pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of per-session token usage for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageSessionsResponseOut {
    /// Pass as `cursor` to fetch the next page; null when the window is exhausted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Rows for this page, newest session first
    #[serde(default)]
    pub sessions: Vec<WorkspaceUsageSessionRowOut>,
    #[serde(default)]
    pub window: WorkspaceUsageWindowOut,
    /// Workspace the rows belong to
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceUsageSessionsResponseOut {
    pub fn builder() -> WorkspaceUsageSessionsResponseOutBuilder {
        <WorkspaceUsageSessionsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageSessionsResponseOutBuilder {
    next_cursor: Option<String>,
    sessions: Option<Vec<WorkspaceUsageSessionRowOut>>,
    window: Option<WorkspaceUsageWindowOut>,
    workspace_id: Option<String>,
}

impl WorkspaceUsageSessionsResponseOutBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn sessions(mut self, value: Vec<WorkspaceUsageSessionRowOut>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn window(mut self, value: WorkspaceUsageWindowOut) -> Self {
        self.window = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageSessionsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sessions`](WorkspaceUsageSessionsResponseOutBuilder::sessions)
    /// - [`window`](WorkspaceUsageSessionsResponseOutBuilder::window)
    /// - [`workspace_id`](WorkspaceUsageSessionsResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceUsageSessionsResponseOut, BuildError> {
        Ok(WorkspaceUsageSessionsResponseOut {
            next_cursor: self.next_cursor,
            sessions: self.sessions.ok_or_else(|| BuildError::missing_field("sessions"))?,
            window: self.window.ok_or_else(|| BuildError::missing_field("window"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
