pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Usage attributed to one Computer asset over the window.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageComputerOut {
    /// The Computer asset ID
    #[serde(default)]
    pub computer_asset_id: String,
    /// Whether the Computer is archived
    #[serde(default)]
    pub is_archived: bool,
    /// When the most recent usage session was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_session_at: Option<DateTime<FixedOffset>>,
    /// Number of successful model calls
    #[serde(default)]
    pub request_count: i64,
    /// Number of usage sessions
    #[serde(default)]
    pub session_count: i64,
    /// The Computer's title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Token counters behind this Computer's usage
    #[serde(default)]
    pub tokens: WorkspaceUsageTokensOut,
    /// Total LLM cost in USD
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_cost_usd: f64,
}

impl WorkspaceUsageComputerOut {
    pub fn builder() -> WorkspaceUsageComputerOutBuilder {
        <WorkspaceUsageComputerOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageComputerOutBuilder {
    computer_asset_id: Option<String>,
    is_archived: Option<bool>,
    last_session_at: Option<DateTime<FixedOffset>>,
    request_count: Option<i64>,
    session_count: Option<i64>,
    title: Option<String>,
    tokens: Option<WorkspaceUsageTokensOut>,
    total_cost_usd: Option<f64>,
}

impl WorkspaceUsageComputerOutBuilder {
    pub fn computer_asset_id(mut self, value: impl Into<String>) -> Self {
        self.computer_asset_id = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn last_session_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_session_at = Some(value);
        self
    }

    pub fn request_count(mut self, value: i64) -> Self {
        self.request_count = Some(value);
        self
    }

    pub fn session_count(mut self, value: i64) -> Self {
        self.session_count = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
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

    /// Consumes the builder and constructs a [`WorkspaceUsageComputerOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`computer_asset_id`](WorkspaceUsageComputerOutBuilder::computer_asset_id)
    /// - [`is_archived`](WorkspaceUsageComputerOutBuilder::is_archived)
    /// - [`request_count`](WorkspaceUsageComputerOutBuilder::request_count)
    /// - [`session_count`](WorkspaceUsageComputerOutBuilder::session_count)
    /// - [`tokens`](WorkspaceUsageComputerOutBuilder::tokens)
    /// - [`total_cost_usd`](WorkspaceUsageComputerOutBuilder::total_cost_usd)
    pub fn build(self) -> Result<WorkspaceUsageComputerOut, BuildError> {
        Ok(WorkspaceUsageComputerOut {
            computer_asset_id: self.computer_asset_id.ok_or_else(|| BuildError::missing_field("computer_asset_id"))?,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            last_session_at: self.last_session_at,
            request_count: self.request_count.ok_or_else(|| BuildError::missing_field("request_count"))?,
            session_count: self.session_count.ok_or_else(|| BuildError::missing_field("session_count"))?,
            title: self.title,
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
            total_cost_usd: self.total_cost_usd.ok_or_else(|| BuildError::missing_field("total_cost_usd"))?,
        })
    }
}
