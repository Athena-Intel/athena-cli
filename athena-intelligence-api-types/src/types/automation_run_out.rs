pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One run of an automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationRunOut {
    #[serde(default)]
    pub automation_asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub event_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inngest_run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// live, or dry_run for a rehearsal whose writing effects were captured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<AutomationRunOutMode>,
    /// success | skipped | failure | canceled — how the run ended, refining its status (`skipped` is a completed run whose `end` step said so); null until it ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<AutomationRunOutOutcome>,
    #[serde(default)]
    pub principal_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_asset_id: Option<String>,
    #[serde(default)]
    pub run_id: String,
    /// scheduled | queued | running | needs_input | completed | failed | canceled
    #[serde(default)]
    pub run_status: String,
    /// When a `scheduled` run was created for (quiet hours defer a fire to the window's end); null for a run queued at once
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// The canonical status_v2 projection of the run
    #[serde(default)]
    pub status_v2: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// manual | schedule | table_change | run_completed | event | call_automation | redrive
    #[serde(default)]
    pub trigger_kind: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub version_id: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl AutomationRunOut {
    pub fn builder() -> AutomationRunOutBuilder {
        <AutomationRunOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunOutBuilder {
    automation_asset_id: Option<String>,
    cost_usd: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    event_ids: Option<Vec<String>>,
    failure_reason: Option<String>,
    fingerprint: Option<String>,
    inngest_run_id: Option<String>,
    inputs: Option<HashMap<String, serde_json::Value>>,
    mode: Option<AutomationRunOutMode>,
    outcome: Option<AutomationRunOutOutcome>,
    principal_ref: Option<String>,
    project_asset_id: Option<String>,
    run_id: Option<String>,
    run_status: Option<String>,
    scheduled_for: Option<DateTime<FixedOffset>>,
    started_at: Option<DateTime<FixedOffset>>,
    status_v2: Option<HashMap<String, serde_json::Value>>,
    tokens: Option<i64>,
    trigger_execution_id: Option<String>,
    trigger_id: Option<String>,
    trigger_kind: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    version_id: Option<String>,
    workspace_id: Option<String>,
}

impl AutomationRunOutBuilder {
    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn cost_usd(mut self, value: f64) -> Self {
        self.cost_usd = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn event_ids(mut self, value: Vec<String>) -> Self {
        self.event_ids = Some(value);
        self
    }

    pub fn failure_reason(mut self, value: impl Into<String>) -> Self {
        self.failure_reason = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn inngest_run_id(mut self, value: impl Into<String>) -> Self {
        self.inngest_run_id = Some(value.into());
        self
    }

    pub fn inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn mode(mut self, value: AutomationRunOutMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn outcome(mut self, value: AutomationRunOutOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn principal_ref(mut self, value: impl Into<String>) -> Self {
        self.principal_ref = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn run_status(mut self, value: impl Into<String>) -> Self {
        self.run_status = Some(value.into());
        self
    }

    pub fn scheduled_for(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_for = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status_v2(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.status_v2 = Some(value);
        self
    }

    pub fn tokens(mut self, value: i64) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn trigger_execution_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_execution_id = Some(value.into());
        self
    }

    pub fn trigger_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_id = Some(value.into());
        self
    }

    pub fn trigger_kind(mut self, value: impl Into<String>) -> Self {
        self.trigger_kind = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automation_asset_id`](AutomationRunOutBuilder::automation_asset_id)
    /// - [`created_at`](AutomationRunOutBuilder::created_at)
    /// - [`event_ids`](AutomationRunOutBuilder::event_ids)
    /// - [`fingerprint`](AutomationRunOutBuilder::fingerprint)
    /// - [`principal_ref`](AutomationRunOutBuilder::principal_ref)
    /// - [`run_id`](AutomationRunOutBuilder::run_id)
    /// - [`run_status`](AutomationRunOutBuilder::run_status)
    /// - [`status_v2`](AutomationRunOutBuilder::status_v2)
    /// - [`trigger_kind`](AutomationRunOutBuilder::trigger_kind)
    /// - [`updated_at`](AutomationRunOutBuilder::updated_at)
    /// - [`version_id`](AutomationRunOutBuilder::version_id)
    /// - [`workspace_id`](AutomationRunOutBuilder::workspace_id)
    pub fn build(self) -> Result<AutomationRunOut, BuildError> {
        Ok(AutomationRunOut {
            automation_asset_id: self.automation_asset_id.ok_or_else(|| BuildError::missing_field("automation_asset_id"))?,
            cost_usd: self.cost_usd,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            ended_at: self.ended_at,
            event_ids: self.event_ids.ok_or_else(|| BuildError::missing_field("event_ids"))?,
            failure_reason: self.failure_reason,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            inngest_run_id: self.inngest_run_id,
            inputs: self.inputs,
            mode: self.mode,
            outcome: self.outcome,
            principal_ref: self.principal_ref.ok_or_else(|| BuildError::missing_field("principal_ref"))?,
            project_asset_id: self.project_asset_id,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            run_status: self.run_status.ok_or_else(|| BuildError::missing_field("run_status"))?,
            scheduled_for: self.scheduled_for,
            started_at: self.started_at,
            status_v2: self.status_v2.ok_or_else(|| BuildError::missing_field("status_v2"))?,
            tokens: self.tokens,
            trigger_execution_id: self.trigger_execution_id,
            trigger_id: self.trigger_id,
            trigger_kind: self.trigger_kind.ok_or_else(|| BuildError::missing_field("trigger_kind"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
