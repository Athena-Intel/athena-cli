pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Token usage of one session for one provider/model/status combination.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageSessionRowOut {
    /// AOP asset id when started by an AOP run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_id: Option<String>,
    /// Title of that AOP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aop_title: Option<String>,
    /// Billing attribution of the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_to: Option<String>,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<i64>,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<i64>,
    /// Earliest model call in the row
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_event_at: Option<DateTime<FixedOffset>>,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    /// Latest model call in the row
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_event_at: Option<DateTime<FixedOffset>>,
    /// How the session was started (user_chat, aop_execution, public_api, ...)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_kind: Option<String>,
    /// Human label for origin_kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_kind_label: Option<String>,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
    /// Project asset id, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Title of that project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_title: Option<String>,
    /// LLM provider
    #[serde(default)]
    pub provider: String,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_output_tokens: Option<i64>,
    /// Model the session asked for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_model: Option<String>,
    /// Model that actually served the calls
    #[serde(default)]
    pub resolved_model: String,
    /// Billed cost of this row in USD; safe to sum across rows
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub row_total_cost_usd: f64,
    /// Model calls aggregated into this row
    #[serde(default)]
    pub run_count: i64,
    /// When the session was created
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub session_created_at: DateTime<FixedOffset>,
    /// Session (conversation) asset id
    #[serde(default)]
    pub session_id: String,
    /// Billed cost of the whole session in USD, carried by the session's first row only and null on its other rows, so summing the column never double-counts (same convention as the CSV export)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub session_total_cost_usd: Option<f64>,
    /// Outcome of the aggregated calls; `untracked` marks a session metered before token tracking existed, whose token counters are null
    #[serde(default)]
    pub status: String,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
    /// AOP trigger type, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// Null for an `untracked` row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncached_input_tokens: Option<i64>,
    /// Window end (exclusive) the row was bounded to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub usage_period_end_exclusive: Option<DateTime<FixedOffset>>,
    /// Window start the row was bounded to
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub usage_period_start: DateTime<FixedOffset>,
    /// Email of the member who ran the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    /// Display name of that member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl WorkspaceUsageSessionRowOut {
    pub fn builder() -> WorkspaceUsageSessionRowOutBuilder {
        <WorkspaceUsageSessionRowOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageSessionRowOutBuilder {
    aop_id: Option<String>,
    aop_title: Option<String>,
    billed_to: Option<String>,
    cache_creation_input_tokens: Option<i64>,
    cache_read_input_tokens: Option<i64>,
    first_event_at: Option<DateTime<FixedOffset>>,
    input_tokens: Option<i64>,
    last_event_at: Option<DateTime<FixedOffset>>,
    origin_kind: Option<String>,
    origin_kind_label: Option<String>,
    output_tokens: Option<i64>,
    project_id: Option<String>,
    project_title: Option<String>,
    provider: Option<String>,
    reasoning_output_tokens: Option<i64>,
    requested_model: Option<String>,
    resolved_model: Option<String>,
    row_total_cost_usd: Option<f64>,
    run_count: Option<i64>,
    session_created_at: Option<DateTime<FixedOffset>>,
    session_id: Option<String>,
    session_total_cost_usd: Option<f64>,
    status: Option<String>,
    total_tokens: Option<i64>,
    trigger_type: Option<String>,
    uncached_input_tokens: Option<i64>,
    usage_period_end_exclusive: Option<DateTime<FixedOffset>>,
    usage_period_start: Option<DateTime<FixedOffset>>,
    user_email: Option<String>,
    user_name: Option<String>,
}

impl WorkspaceUsageSessionRowOutBuilder {
    pub fn aop_id(mut self, value: impl Into<String>) -> Self {
        self.aop_id = Some(value.into());
        self
    }

    pub fn aop_title(mut self, value: impl Into<String>) -> Self {
        self.aop_title = Some(value.into());
        self
    }

    pub fn billed_to(mut self, value: impl Into<String>) -> Self {
        self.billed_to = Some(value.into());
        self
    }

    pub fn cache_creation_input_tokens(mut self, value: i64) -> Self {
        self.cache_creation_input_tokens = Some(value);
        self
    }

    pub fn cache_read_input_tokens(mut self, value: i64) -> Self {
        self.cache_read_input_tokens = Some(value);
        self
    }

    pub fn first_event_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_event_at = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn last_event_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_event_at = Some(value);
        self
    }

    pub fn origin_kind(mut self, value: impl Into<String>) -> Self {
        self.origin_kind = Some(value.into());
        self
    }

    pub fn origin_kind_label(mut self, value: impl Into<String>) -> Self {
        self.origin_kind_label = Some(value.into());
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn project_title(mut self, value: impl Into<String>) -> Self {
        self.project_title = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn reasoning_output_tokens(mut self, value: i64) -> Self {
        self.reasoning_output_tokens = Some(value);
        self
    }

    pub fn requested_model(mut self, value: impl Into<String>) -> Self {
        self.requested_model = Some(value.into());
        self
    }

    pub fn resolved_model(mut self, value: impl Into<String>) -> Self {
        self.resolved_model = Some(value.into());
        self
    }

    pub fn row_total_cost_usd(mut self, value: f64) -> Self {
        self.row_total_cost_usd = Some(value);
        self
    }

    pub fn run_count(mut self, value: i64) -> Self {
        self.run_count = Some(value);
        self
    }

    pub fn session_created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.session_created_at = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_total_cost_usd(mut self, value: f64) -> Self {
        self.session_total_cost_usd = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    pub fn trigger_type(mut self, value: impl Into<String>) -> Self {
        self.trigger_type = Some(value.into());
        self
    }

    pub fn uncached_input_tokens(mut self, value: i64) -> Self {
        self.uncached_input_tokens = Some(value);
        self
    }

    pub fn usage_period_end_exclusive(mut self, value: DateTime<FixedOffset>) -> Self {
        self.usage_period_end_exclusive = Some(value);
        self
    }

    pub fn usage_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.usage_period_start = Some(value);
        self
    }

    pub fn user_email(mut self, value: impl Into<String>) -> Self {
        self.user_email = Some(value.into());
        self
    }

    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageSessionRowOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](WorkspaceUsageSessionRowOutBuilder::provider)
    /// - [`resolved_model`](WorkspaceUsageSessionRowOutBuilder::resolved_model)
    /// - [`row_total_cost_usd`](WorkspaceUsageSessionRowOutBuilder::row_total_cost_usd)
    /// - [`run_count`](WorkspaceUsageSessionRowOutBuilder::run_count)
    /// - [`session_created_at`](WorkspaceUsageSessionRowOutBuilder::session_created_at)
    /// - [`session_id`](WorkspaceUsageSessionRowOutBuilder::session_id)
    /// - [`status`](WorkspaceUsageSessionRowOutBuilder::status)
    /// - [`usage_period_start`](WorkspaceUsageSessionRowOutBuilder::usage_period_start)
    pub fn build(self) -> Result<WorkspaceUsageSessionRowOut, BuildError> {
        Ok(WorkspaceUsageSessionRowOut {
            aop_id: self.aop_id,
            aop_title: self.aop_title,
            billed_to: self.billed_to,
            cache_creation_input_tokens: self.cache_creation_input_tokens,
            cache_read_input_tokens: self.cache_read_input_tokens,
            first_event_at: self.first_event_at,
            input_tokens: self.input_tokens,
            last_event_at: self.last_event_at,
            origin_kind: self.origin_kind,
            origin_kind_label: self.origin_kind_label,
            output_tokens: self.output_tokens,
            project_id: self.project_id,
            project_title: self.project_title,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            reasoning_output_tokens: self.reasoning_output_tokens,
            requested_model: self.requested_model,
            resolved_model: self.resolved_model.ok_or_else(|| BuildError::missing_field("resolved_model"))?,
            row_total_cost_usd: self.row_total_cost_usd.ok_or_else(|| BuildError::missing_field("row_total_cost_usd"))?,
            run_count: self.run_count.ok_or_else(|| BuildError::missing_field("run_count"))?,
            session_created_at: self.session_created_at.ok_or_else(|| BuildError::missing_field("session_created_at"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            session_total_cost_usd: self.session_total_cost_usd,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            total_tokens: self.total_tokens,
            trigger_type: self.trigger_type,
            uncached_input_tokens: self.uncached_input_tokens,
            usage_period_end_exclusive: self.usage_period_end_exclusive,
            usage_period_start: self.usage_period_start.ok_or_else(|| BuildError::missing_field("usage_period_start"))?,
            user_email: self.user_email,
            user_name: self.user_name,
        })
    }
}
