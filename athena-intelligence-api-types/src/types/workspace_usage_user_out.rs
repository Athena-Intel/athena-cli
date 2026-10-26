pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Usage of one workspace member over the window.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageUserOut {
    /// The user's email
    #[serde(default)]
    pub email: String,
    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// When the user's most recent session was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_session_at: Option<DateTime<FixedOffset>>,
    /// Number of sessions
    #[serde(default)]
    pub session_count: i64,
    /// Token counters across the user's sessions
    #[serde(default)]
    pub tokens: WorkspaceUsageTokensOut,
    /// Total LLM cost in USD
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_cost_usd: f64,
    /// The user ID
    #[serde(default)]
    pub user_id: String,
}

impl WorkspaceUsageUserOut {
    pub fn builder() -> WorkspaceUsageUserOutBuilder {
        <WorkspaceUsageUserOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageUserOutBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    last_session_at: Option<DateTime<FixedOffset>>,
    session_count: Option<i64>,
    tokens: Option<WorkspaceUsageTokensOut>,
    total_cost_usd: Option<f64>,
    user_id: Option<String>,
}

impl WorkspaceUsageUserOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn last_session_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_session_at = Some(value);
        self
    }

    pub fn session_count(mut self, value: i64) -> Self {
        self.session_count = Some(value);
        self
    }

    pub fn tokens(mut self, value: WorkspaceUsageTokensOut) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn total_cost_usd(mut self, value: f64) -> Self {
        self.total_cost_usd = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageUserOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](WorkspaceUsageUserOutBuilder::email)
    /// - [`session_count`](WorkspaceUsageUserOutBuilder::session_count)
    /// - [`tokens`](WorkspaceUsageUserOutBuilder::tokens)
    /// - [`total_cost_usd`](WorkspaceUsageUserOutBuilder::total_cost_usd)
    /// - [`user_id`](WorkspaceUsageUserOutBuilder::user_id)
    pub fn build(self) -> Result<WorkspaceUsageUserOut, BuildError> {
        Ok(WorkspaceUsageUserOut {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            last_session_at: self.last_session_at,
            session_count: self.session_count.ok_or_else(|| BuildError::missing_field("session_count"))?,
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
            total_cost_usd: self.total_cost_usd.ok_or_else(|| BuildError::missing_field("total_cost_usd"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
