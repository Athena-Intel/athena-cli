pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One audited event's verdict and the executions recorded for it; ids only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayVerdictOut {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub event_type: String,
    /// Trigger executions the engine recorded for the event on rules no automation manages
    #[serde(default)]
    pub legacy_execution_ids: Vec<String>,
    /// The target's triggers that match, by definition id
    #[serde(default)]
    pub matched_trigger_ids: Vec<String>,
    /// matched, scope mismatch, conditions false, own event, cel false, cel not evaluated, cel error, no rule or unreadable envelope — the furthest stage any trigger reached when none matched
    #[serde(default)]
    pub reason: String,
    /// When the trigger engine received the event
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub received_at: DateTime<FixedOffset>,
    /// Trigger executions the target's own rules recorded for the event
    #[serde(default)]
    pub target_execution_ids: Vec<String>,
    /// True when at least one of the target's triggers matches it
    #[serde(default)]
    pub would_fire: bool,
}

impl ReplayVerdictOut {
    pub fn builder() -> ReplayVerdictOutBuilder {
        <ReplayVerdictOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayVerdictOutBuilder {
    event_id: Option<String>,
    event_type: Option<String>,
    legacy_execution_ids: Option<Vec<String>>,
    matched_trigger_ids: Option<Vec<String>>,
    reason: Option<String>,
    received_at: Option<DateTime<FixedOffset>>,
    target_execution_ids: Option<Vec<String>>,
    would_fire: Option<bool>,
}

impl ReplayVerdictOutBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn legacy_execution_ids(mut self, value: Vec<String>) -> Self {
        self.legacy_execution_ids = Some(value);
        self
    }

    pub fn matched_trigger_ids(mut self, value: Vec<String>) -> Self {
        self.matched_trigger_ids = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn received_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.received_at = Some(value);
        self
    }

    pub fn target_execution_ids(mut self, value: Vec<String>) -> Self {
        self.target_execution_ids = Some(value);
        self
    }

    pub fn would_fire(mut self, value: bool) -> Self {
        self.would_fire = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayVerdictOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ReplayVerdictOutBuilder::event_id)
    /// - [`event_type`](ReplayVerdictOutBuilder::event_type)
    /// - [`legacy_execution_ids`](ReplayVerdictOutBuilder::legacy_execution_ids)
    /// - [`matched_trigger_ids`](ReplayVerdictOutBuilder::matched_trigger_ids)
    /// - [`reason`](ReplayVerdictOutBuilder::reason)
    /// - [`received_at`](ReplayVerdictOutBuilder::received_at)
    /// - [`target_execution_ids`](ReplayVerdictOutBuilder::target_execution_ids)
    /// - [`would_fire`](ReplayVerdictOutBuilder::would_fire)
    pub fn build(self) -> Result<ReplayVerdictOut, BuildError> {
        Ok(ReplayVerdictOut {
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            legacy_execution_ids: self.legacy_execution_ids.ok_or_else(|| BuildError::missing_field("legacy_execution_ids"))?,
            matched_trigger_ids: self.matched_trigger_ids.ok_or_else(|| BuildError::missing_field("matched_trigger_ids"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            received_at: self.received_at.ok_or_else(|| BuildError::missing_field("received_at"))?,
            target_execution_ids: self.target_execution_ids.ok_or_else(|| BuildError::missing_field("target_execution_ids"))?,
            would_fire: self.would_fire.ok_or_else(|| BuildError::missing_field("would_fire"))?,
        })
    }
}
