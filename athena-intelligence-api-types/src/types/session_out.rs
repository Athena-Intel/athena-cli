pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An agent session (conversation) asset with flattened metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionOut {
    /// Agent identity the session ran with, when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Source AOP asset identifier for an AOP/workflow run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_asset_id: Option<String>,
    /// Execution error recorded for a failed AOP run, when present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_execution_error: Option<String>,
    /// Whether the AOP execution completed successfully, when known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_execution_succeeded: Option<bool>,
    /// Application identifier the session belongs to, when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Asset ID of the collab agent the session was created with, when one was bound; null for stock-agent sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collab_agent_id: Option<String>,
    /// Timestamp when the session was created (ISO 8601)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Unique identifier of the user who created this session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Canonical failure reason for failed runs; null for non-failed runs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason_v2: Option<String>,
    /// Unique identifier of the session asset (e.g., 'asset_abc123')
    #[serde(default)]
    pub id: String,
    /// Whether this is a branched sub-session of another session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sub_session: Option<bool>,
    /// Whether the session is unread for the calling user: it changed since they last opened it, or they never opened it and it did not originate from the web app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unread: Option<bool>,
    /// Plain-text preview of the most recent message, when available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_preview: Option<String>,
    /// Model the session ran with, when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Number of messages in the session, when tracked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_messages: Option<i64>,
    /// Asset ID of the parent session for sub-sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_session_id: Option<String>,
    /// Canonical latest-run status: scheduled, queued, running, needs_input, completed, failed, or canceled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status_v2: Option<String>,
    /// Canonical user-facing session status: idle, active, needs_input, or error
    #[serde(default)]
    pub session_status_v2: String,
    /// Kind of session: 'session' (chat), 'video_session', 'desktop_session', or 'mobile_session'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<String>,
    /// Channel the session originated from (e.g., 'web', 'api', 'agent_email', 'agent_slack', 'agent_sms')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<String>,
    /// Deprecated legacy execution state. Use session_status_v2, run_status_v2, and failure_reason_v2 for status UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// LangChain thread ID backing this session's message history
    #[serde(default)]
    pub thread_id: String,
    /// Display title of the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Total LLM cost of the session in USD, when tracked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_usd: Option<f64>,
    /// Trigger that started an AOP/workflow run, such as schedule or api
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// Timestamp when the session was last updated (ISO 8601)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Unique identifier of the workspace this session belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl SessionOut {
    pub fn builder() -> SessionOutBuilder {
        <SessionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionOutBuilder {
    agent: Option<String>,
    aop_asset_id: Option<String>,
    aop_execution_error: Option<String>,
    aop_execution_succeeded: Option<bool>,
    app_id: Option<String>,
    collab_agent_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_id: Option<String>,
    failure_reason_v2: Option<String>,
    id: Option<String>,
    is_sub_session: Option<bool>,
    is_unread: Option<bool>,
    last_message_preview: Option<String>,
    model: Option<String>,
    num_messages: Option<i64>,
    parent_session_id: Option<String>,
    run_status_v2: Option<String>,
    session_status_v2: Option<String>,
    session_type: Option<String>,
    source_channel: Option<String>,
    state: Option<String>,
    thread_id: Option<String>,
    title: Option<String>,
    total_cost_usd: Option<f64>,
    trigger_type: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl SessionOutBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
        self
    }

    pub fn aop_execution_error(mut self, value: impl Into<String>) -> Self {
        self.aop_execution_error = Some(value.into());
        self
    }

    pub fn aop_execution_succeeded(mut self, value: bool) -> Self {
        self.aop_execution_succeeded = Some(value);
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn collab_agent_id(mut self, value: impl Into<String>) -> Self {
        self.collab_agent_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_id = Some(value.into());
        self
    }

    pub fn failure_reason_v2(mut self, value: impl Into<String>) -> Self {
        self.failure_reason_v2 = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_sub_session(mut self, value: bool) -> Self {
        self.is_sub_session = Some(value);
        self
    }

    pub fn is_unread(mut self, value: bool) -> Self {
        self.is_unread = Some(value);
        self
    }

    pub fn last_message_preview(mut self, value: impl Into<String>) -> Self {
        self.last_message_preview = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn num_messages(mut self, value: i64) -> Self {
        self.num_messages = Some(value);
        self
    }

    pub fn parent_session_id(mut self, value: impl Into<String>) -> Self {
        self.parent_session_id = Some(value.into());
        self
    }

    pub fn run_status_v2(mut self, value: impl Into<String>) -> Self {
        self.run_status_v2 = Some(value.into());
        self
    }

    pub fn session_status_v2(mut self, value: impl Into<String>) -> Self {
        self.session_status_v2 = Some(value.into());
        self
    }

    pub fn session_type(mut self, value: impl Into<String>) -> Self {
        self.session_type = Some(value.into());
        self
    }

    pub fn source_channel(mut self, value: impl Into<String>) -> Self {
        self.source_channel = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn total_cost_usd(mut self, value: f64) -> Self {
        self.total_cost_usd = Some(value);
        self
    }

    pub fn trigger_type(mut self, value: impl Into<String>) -> Self {
        self.trigger_type = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SessionOutBuilder::created_at)
    /// - [`id`](SessionOutBuilder::id)
    /// - [`session_status_v2`](SessionOutBuilder::session_status_v2)
    /// - [`thread_id`](SessionOutBuilder::thread_id)
    /// - [`updated_at`](SessionOutBuilder::updated_at)
    pub fn build(self) -> Result<SessionOut, BuildError> {
        Ok(SessionOut {
            agent: self.agent,
            aop_asset_id: self.aop_asset_id,
            aop_execution_error: self.aop_execution_error,
            aop_execution_succeeded: self.aop_execution_succeeded,
            app_id: self.app_id,
            collab_agent_id: self.collab_agent_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by_id: self.created_by_id,
            failure_reason_v2: self.failure_reason_v2,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_sub_session: self.is_sub_session,
            is_unread: self.is_unread,
            last_message_preview: self.last_message_preview,
            model: self.model,
            num_messages: self.num_messages,
            parent_session_id: self.parent_session_id,
            run_status_v2: self.run_status_v2,
            session_status_v2: self.session_status_v2.ok_or_else(|| BuildError::missing_field("session_status_v2"))?,
            session_type: self.session_type,
            source_channel: self.source_channel,
            state: self.state,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            title: self.title,
            total_cost_usd: self.total_cost_usd,
            trigger_type: self.trigger_type,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self.workspace_id,
        })
    }
}
