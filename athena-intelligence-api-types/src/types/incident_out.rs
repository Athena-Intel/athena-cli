pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A node that would not converge, with an owner (an incidents row).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncidentOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub acknowledged_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_by_ref: Option<String>,
    /// The asset bound to the node when the incident opened
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub closed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deadline_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub environment: String,
    /// The system_health_checks ids that failed
    #[serde(default)]
    pub failing_checks: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub node_key: String,
    #[serde(default)]
    pub on_call_refs: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub opened_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub opened_by_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postmortem_asset_id: Option<String>,
    #[serde(default)]
    pub project_asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub resolved_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by_ref: Option<String>,
    #[serde(default)]
    pub runbook_run_ids: Vec<String>,
    /// warning or critical
    #[serde(default)]
    pub severity: String,
    /// open, acknowledged, mitigating, resolved or closed
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl IncidentOut {
    pub fn builder() -> IncidentOutBuilder {
        <IncidentOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentOutBuilder {
    acknowledged_at: Option<DateTime<FixedOffset>>,
    acknowledged_by_ref: Option<String>,
    asset_id: Option<String>,
    closed_at: Option<DateTime<FixedOffset>>,
    deadline_at: Option<DateTime<FixedOffset>>,
    environment: Option<String>,
    failing_checks: Option<Vec<String>>,
    id: Option<String>,
    node_key: Option<String>,
    on_call_refs: Option<Vec<String>>,
    opened_at: Option<DateTime<FixedOffset>>,
    opened_by_ref: Option<String>,
    postmortem_asset_id: Option<String>,
    project_asset_id: Option<String>,
    resolved_at: Option<DateTime<FixedOffset>>,
    resolved_by_ref: Option<String>,
    runbook_run_ids: Option<Vec<String>>,
    severity: Option<String>,
    state: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl IncidentOutBuilder {
    pub fn acknowledged_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.acknowledged_at = Some(value);
        self
    }

    pub fn acknowledged_by_ref(mut self, value: impl Into<String>) -> Self {
        self.acknowledged_by_ref = Some(value.into());
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn closed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.closed_at = Some(value);
        self
    }

    pub fn deadline_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deadline_at = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn failing_checks(mut self, value: Vec<String>) -> Self {
        self.failing_checks = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn node_key(mut self, value: impl Into<String>) -> Self {
        self.node_key = Some(value.into());
        self
    }

    pub fn on_call_refs(mut self, value: Vec<String>) -> Self {
        self.on_call_refs = Some(value);
        self
    }

    pub fn opened_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.opened_at = Some(value);
        self
    }

    pub fn opened_by_ref(mut self, value: impl Into<String>) -> Self {
        self.opened_by_ref = Some(value.into());
        self
    }

    pub fn postmortem_asset_id(mut self, value: impl Into<String>) -> Self {
        self.postmortem_asset_id = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    pub fn resolved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    pub fn resolved_by_ref(mut self, value: impl Into<String>) -> Self {
        self.resolved_by_ref = Some(value.into());
        self
    }

    pub fn runbook_run_ids(mut self, value: Vec<String>) -> Self {
        self.runbook_run_ids = Some(value);
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IncidentOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](IncidentOutBuilder::environment)
    /// - [`failing_checks`](IncidentOutBuilder::failing_checks)
    /// - [`id`](IncidentOutBuilder::id)
    /// - [`node_key`](IncidentOutBuilder::node_key)
    /// - [`on_call_refs`](IncidentOutBuilder::on_call_refs)
    /// - [`opened_at`](IncidentOutBuilder::opened_at)
    /// - [`opened_by_ref`](IncidentOutBuilder::opened_by_ref)
    /// - [`project_asset_id`](IncidentOutBuilder::project_asset_id)
    /// - [`runbook_run_ids`](IncidentOutBuilder::runbook_run_ids)
    /// - [`severity`](IncidentOutBuilder::severity)
    /// - [`state`](IncidentOutBuilder::state)
    /// - [`title`](IncidentOutBuilder::title)
    /// - [`updated_at`](IncidentOutBuilder::updated_at)
    pub fn build(self) -> Result<IncidentOut, BuildError> {
        Ok(IncidentOut {
            acknowledged_at: self.acknowledged_at,
            acknowledged_by_ref: self.acknowledged_by_ref,
            asset_id: self.asset_id,
            closed_at: self.closed_at,
            deadline_at: self.deadline_at,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            failing_checks: self.failing_checks.ok_or_else(|| BuildError::missing_field("failing_checks"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            node_key: self.node_key.ok_or_else(|| BuildError::missing_field("node_key"))?,
            on_call_refs: self.on_call_refs.ok_or_else(|| BuildError::missing_field("on_call_refs"))?,
            opened_at: self.opened_at.ok_or_else(|| BuildError::missing_field("opened_at"))?,
            opened_by_ref: self.opened_by_ref.ok_or_else(|| BuildError::missing_field("opened_by_ref"))?,
            postmortem_asset_id: self.postmortem_asset_id,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            resolved_at: self.resolved_at,
            resolved_by_ref: self.resolved_by_ref,
            runbook_run_ids: self.runbook_run_ids.ok_or_else(|| BuildError::missing_field("runbook_run_ids"))?,
            severity: self.severity.ok_or_else(|| BuildError::missing_field("severity"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
