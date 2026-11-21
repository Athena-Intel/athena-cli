pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksListQueryRequest {
    /// The workspace to list; the caller must be a member.
    #[serde(default)]
    pub workspace_id: String,
    /// Only tasks assigned to this user_… or agent_asset_…, or me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ref: Option<String>,
    /// open (the default), done or all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTasksRequestStatus>,
    /// YYYY-MM-DD: only tasks due on or before this day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on_or_before: Option<String>,
    /// YYYY-MM-DD: only tasks due on or after this day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on_or_after: Option<String>,
    /// Only tasks with a label of exactly this name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Only tasks that link this asset or ontology object id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_id: Option<String>,
    /// At most this many tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TasksListQueryRequest {
    pub fn builder() -> TasksListQueryRequestBuilder {
        <TasksListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksListQueryRequestBuilder {
    workspace_id: Option<String>,
    assignee_ref: Option<String>,
    status: Option<ListTasksRequestStatus>,
    due_on_or_before: Option<String>,
    due_on_or_after: Option<String>,
    label: Option<String>,
    linked_id: Option<String>,
    limit: Option<i64>,
}

impl TasksListQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn assignee_ref(mut self, value: impl Into<String>) -> Self {
        self.assignee_ref = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListTasksRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn due_on_or_before(mut self, value: impl Into<String>) -> Self {
        self.due_on_or_before = Some(value.into());
        self
    }

    pub fn due_on_or_after(mut self, value: impl Into<String>) -> Self {
        self.due_on_or_after = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn linked_id(mut self, value: impl Into<String>) -> Self {
        self.linked_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TasksListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](TasksListQueryRequestBuilder::workspace_id)
    pub fn build(self) -> Result<TasksListQueryRequest, BuildError> {
        Ok(TasksListQueryRequest {
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            assignee_ref: self.assignee_ref,
            status: self.status,
            due_on_or_before: self.due_on_or_before,
            due_on_or_after: self.due_on_or_after,
            label: self.label,
            linked_id: self.linked_id,
            limit: self.limit,
        })
    }
}

