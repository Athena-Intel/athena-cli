pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsListQueryRequest {
    /// Keyword to search session titles (case-insensitive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Execution state(s) to filter by (e.g. 'running', 'completed'). Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
    /// Originating channel(s) to filter by (e.g. 'web', 'api', 'agent_email'). Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<Vec<String>>,
    /// Session kind(s) to include: 'session', 'video_session', 'desktop_session', 'mobile_session'. Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<Vec<String>>,
    /// Only include sessions belonging to this application identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Include branched sub-sessions (excluded by default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_sub_sessions: Option<bool>,
    /// Include AOP/workflow task runs (excluded by default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_task_sessions: Option<bool>,
    /// Only include task sessions originating from this AOP asset identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_asset_id: Option<String>,
    /// Workspace to list sessions from. Defaults to the caller's current workspace; any other workspace the caller is a member of can be requested explicitly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// Trigger type(s) to filter by (e.g. 'schedule', 'api', 'email'). Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<Vec<String>>,
    /// Only include sessions created at or after this ISO 8601 timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only include sessions created at or before this ISO 8601 timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Field to sort by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ListSessionsRequestSortBy>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<ListSessionsRequestSortDirection>,
    /// Maximum number of sessions to return per page (1-500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of sessions to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl SessionsListQueryRequest {
    pub fn builder() -> SessionsListQueryRequestBuilder {
        <SessionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListQueryRequestBuilder {
    query: Option<String>,
    state: Option<Vec<String>>,
    source_channel: Option<Vec<String>>,
    session_type: Option<Vec<String>>,
    app_id: Option<String>,
    include_sub_sessions: Option<bool>,
    include_task_sessions: Option<bool>,
    aop_asset_id: Option<String>,
    workspace_id: Option<String>,
    trigger_type: Option<Vec<String>>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    sort_by: Option<ListSessionsRequestSortBy>,
    sort_direction: Option<ListSessionsRequestSortDirection>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl SessionsListQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn state(mut self, value: Vec<String>) -> Self {
        self.state = Some(value);
        self
    }

    pub fn source_channel(mut self, value: Vec<String>) -> Self {
        self.source_channel = Some(value);
        self
    }

    pub fn session_type(mut self, value: Vec<String>) -> Self {
        self.session_type = Some(value);
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn include_sub_sessions(mut self, value: bool) -> Self {
        self.include_sub_sessions = Some(value);
        self
    }

    pub fn include_task_sessions(mut self, value: bool) -> Self {
        self.include_task_sessions = Some(value);
        self
    }

    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn trigger_type(mut self, value: Vec<String>) -> Self {
        self.trigger_type = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn sort_by(mut self, value: ListSessionsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: ListSessionsRequestSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsListQueryRequest`].
    pub fn build(self) -> Result<SessionsListQueryRequest, BuildError> {
        Ok(SessionsListQueryRequest {
            query: self.query,
            state: self.state,
            source_channel: self.source_channel,
            session_type: self.session_type,
            app_id: self.app_id,
            include_sub_sessions: self.include_sub_sessions,
            include_task_sessions: self.include_task_sessions,
            aop_asset_id: self.aop_asset_id,
            workspace_id: self.workspace_id,
            trigger_type: self.trigger_type,
            created_after: self.created_after,
            created_before: self.created_before,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

