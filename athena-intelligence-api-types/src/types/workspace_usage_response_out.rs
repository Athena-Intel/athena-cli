pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Aggregated LLM token and cost usage for one workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceUsageResponseOut {
    /// Users with at least one session
    #[serde(default)]
    pub active_users: i64,
    /// Portion of total_cost_usd produced by Computers
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub computer_cost_usd: f64,
    /// Number of Computer usage sessions
    #[serde(default)]
    pub computer_sessions: i64,
    /// Portion of tokens produced by Computers' LLM usage (already included in tokens, never additive)
    #[serde(default)]
    pub computer_tokens: WorkspaceUsageTokensOut,
    /// Per-day breakdown, oldest first
    #[serde(default)]
    pub daily: Vec<WorkspaceUsageDayOut>,
    /// Length of the reporting window in days
    #[serde(default)]
    pub days: i64,
    /// Cost of dry-run (test) sessions
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub dry_run_cost_usd: f64,
    /// Number of dry-run (test) sessions
    #[serde(default)]
    pub dry_run_sessions: i64,
    /// Token counters from dry-run (test) sessions, excluded from tokens
    #[serde(default)]
    pub dry_run_tokens: WorkspaceUsageTokensOut,
    /// Per-Computer breakdown, highest cost first
    #[serde(default)]
    pub per_computer: Vec<WorkspaceUsageComputerOut>,
    /// Per-member breakdown, highest cost first
    #[serde(default)]
    pub per_user: Vec<WorkspaceUsageUserOut>,
    /// Token counters across the workspace's sessions in the window, excluding dry-run (test) sessions
    #[serde(default)]
    pub tokens: WorkspaceUsageTokensOut,
    /// Total LLM cost in USD
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_cost_usd: f64,
    /// Number of sessions in the window
    #[serde(default)]
    pub total_sessions: i64,
    /// The workspace ID
    #[serde(default)]
    pub workspace_id: String,
}

impl WorkspaceUsageResponseOut {
    pub fn builder() -> WorkspaceUsageResponseOutBuilder {
        <WorkspaceUsageResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageResponseOutBuilder {
    active_users: Option<i64>,
    computer_cost_usd: Option<f64>,
    computer_sessions: Option<i64>,
    computer_tokens: Option<WorkspaceUsageTokensOut>,
    daily: Option<Vec<WorkspaceUsageDayOut>>,
    days: Option<i64>,
    dry_run_cost_usd: Option<f64>,
    dry_run_sessions: Option<i64>,
    dry_run_tokens: Option<WorkspaceUsageTokensOut>,
    per_computer: Option<Vec<WorkspaceUsageComputerOut>>,
    per_user: Option<Vec<WorkspaceUsageUserOut>>,
    tokens: Option<WorkspaceUsageTokensOut>,
    total_cost_usd: Option<f64>,
    total_sessions: Option<i64>,
    workspace_id: Option<String>,
}

impl WorkspaceUsageResponseOutBuilder {
    pub fn active_users(mut self, value: i64) -> Self {
        self.active_users = Some(value);
        self
    }

    pub fn computer_cost_usd(mut self, value: f64) -> Self {
        self.computer_cost_usd = Some(value);
        self
    }

    pub fn computer_sessions(mut self, value: i64) -> Self {
        self.computer_sessions = Some(value);
        self
    }

    pub fn computer_tokens(mut self, value: WorkspaceUsageTokensOut) -> Self {
        self.computer_tokens = Some(value);
        self
    }

    pub fn daily(mut self, value: Vec<WorkspaceUsageDayOut>) -> Self {
        self.daily = Some(value);
        self
    }

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn dry_run_cost_usd(mut self, value: f64) -> Self {
        self.dry_run_cost_usd = Some(value);
        self
    }

    pub fn dry_run_sessions(mut self, value: i64) -> Self {
        self.dry_run_sessions = Some(value);
        self
    }

    pub fn dry_run_tokens(mut self, value: WorkspaceUsageTokensOut) -> Self {
        self.dry_run_tokens = Some(value);
        self
    }

    pub fn per_computer(mut self, value: Vec<WorkspaceUsageComputerOut>) -> Self {
        self.per_computer = Some(value);
        self
    }

    pub fn per_user(mut self, value: Vec<WorkspaceUsageUserOut>) -> Self {
        self.per_user = Some(value);
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

    pub fn total_sessions(mut self, value: i64) -> Self {
        self.total_sessions = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_users`](WorkspaceUsageResponseOutBuilder::active_users)
    /// - [`computer_cost_usd`](WorkspaceUsageResponseOutBuilder::computer_cost_usd)
    /// - [`computer_sessions`](WorkspaceUsageResponseOutBuilder::computer_sessions)
    /// - [`computer_tokens`](WorkspaceUsageResponseOutBuilder::computer_tokens)
    /// - [`daily`](WorkspaceUsageResponseOutBuilder::daily)
    /// - [`days`](WorkspaceUsageResponseOutBuilder::days)
    /// - [`dry_run_cost_usd`](WorkspaceUsageResponseOutBuilder::dry_run_cost_usd)
    /// - [`dry_run_sessions`](WorkspaceUsageResponseOutBuilder::dry_run_sessions)
    /// - [`dry_run_tokens`](WorkspaceUsageResponseOutBuilder::dry_run_tokens)
    /// - [`per_computer`](WorkspaceUsageResponseOutBuilder::per_computer)
    /// - [`per_user`](WorkspaceUsageResponseOutBuilder::per_user)
    /// - [`tokens`](WorkspaceUsageResponseOutBuilder::tokens)
    /// - [`total_cost_usd`](WorkspaceUsageResponseOutBuilder::total_cost_usd)
    /// - [`total_sessions`](WorkspaceUsageResponseOutBuilder::total_sessions)
    /// - [`workspace_id`](WorkspaceUsageResponseOutBuilder::workspace_id)
    pub fn build(self) -> Result<WorkspaceUsageResponseOut, BuildError> {
        Ok(WorkspaceUsageResponseOut {
            active_users: self.active_users.ok_or_else(|| BuildError::missing_field("active_users"))?,
            computer_cost_usd: self.computer_cost_usd.ok_or_else(|| BuildError::missing_field("computer_cost_usd"))?,
            computer_sessions: self.computer_sessions.ok_or_else(|| BuildError::missing_field("computer_sessions"))?,
            computer_tokens: self.computer_tokens.ok_or_else(|| BuildError::missing_field("computer_tokens"))?,
            daily: self.daily.ok_or_else(|| BuildError::missing_field("daily"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            dry_run_cost_usd: self.dry_run_cost_usd.ok_or_else(|| BuildError::missing_field("dry_run_cost_usd"))?,
            dry_run_sessions: self.dry_run_sessions.ok_or_else(|| BuildError::missing_field("dry_run_sessions"))?,
            dry_run_tokens: self.dry_run_tokens.ok_or_else(|| BuildError::missing_field("dry_run_tokens"))?,
            per_computer: self.per_computer.ok_or_else(|| BuildError::missing_field("per_computer"))?,
            per_user: self.per_user.ok_or_else(|| BuildError::missing_field("per_user"))?,
            tokens: self.tokens.ok_or_else(|| BuildError::missing_field("tokens"))?,
            total_cost_usd: self.total_cost_usd.ok_or_else(|| BuildError::missing_field("total_cost_usd"))?,
            total_sessions: self.total_sessions.ok_or_else(|| BuildError::missing_field("total_sessions"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
