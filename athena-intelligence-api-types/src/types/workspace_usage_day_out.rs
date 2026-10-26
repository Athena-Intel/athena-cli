pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Usage of the workspace on one UTC day.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageDayOut {
    /// Date in YYYY-MM-DD format (UTC)
    #[serde(default)]
    pub date: String,
    /// Number of sessions created on the day
    #[serde(default)]
    pub session_count: i64,
    /// Token counters for the day
    #[serde(default)]
    pub tokens: WorkspaceUsageTokensOut,
    /// Total LLM cost in USD for the day
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_cost_usd: f64,
}

impl WorkspaceUsageDayOut {
    pub fn builder() -> WorkspaceUsageDayOutBuilder {
        <WorkspaceUsageDayOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageDayOutBuilder {
    date: Option<String>,
    session_count: Option<i64>,
    tokens: Option<WorkspaceUsageTokensOut>,
    total_cost_usd: Option<f64>,
}

impl WorkspaceUsageDayOutBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    /// Consumes the builder and constructs a [`WorkspaceUsageDayOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](WorkspaceUsageDayOutBuilder::date)
    /// - [`session_count`](WorkspaceUsageDayOutBuilder::session_count)
    /// - [`tokens`](WorkspaceUsageDayOutBuilder::tokens)
    /// - [`total_cost_usd`](WorkspaceUsageDayOutBuilder::total_cost_usd)
    pub fn build(self) -> Result<WorkspaceUsageDayOut, BuildError> {
        Ok(WorkspaceUsageDayOut {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            session_count: self.session_count.ok_or_else(|| BuildError::missing_field("session_count"))?,
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
            total_cost_usd: self.total_cost_usd.ok_or_else(|| BuildError::missing_field("total_cost_usd"))?,
        })
    }
}
