pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One entry in an asset's activity log.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActivityItemOut {
    /// Who made the edit: 'user', 'agent', 'canvas-ai', 'rollback', or 'version-restore'. Matches the in-app Activity pane.
    #[serde(default)]
    pub actor_type: String,
    /// Attribution metadata carried by the edit. Common keys: user_id, agent_id, session_id, user_message_tracking_id, ai_message_tracking_id, source. Join session_id against your trace store to link an edit to the run that produced it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<HashMap<String, String>>,
    /// Raw Keryx author id. 'backend-service' for agent writes; a user id for direct edits. Prefer actor_type for classification.
    #[serde(default)]
    pub by: String,
    /// Keryx clock at the start of the edit, in epoch milliseconds. Use with to_clock to request this item's delta.
    #[serde(default)]
    pub from_clock: i64,
    /// from_clock as an ISO-8601 UTC timestamp; empty if not a timestamp.
    #[serde(default)]
    pub from_iso: String,
    /// Keryx clock at the end of the edit, in epoch milliseconds.
    #[serde(default)]
    pub to_clock: i64,
    /// to_clock as an ISO-8601 UTC timestamp; empty if not a timestamp.
    #[serde(default)]
    pub to_iso: String,
}

impl ActivityItemOut {
    pub fn builder() -> ActivityItemOutBuilder {
        <ActivityItemOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityItemOutBuilder {
    actor_type: Option<String>,
    attributions: Option<HashMap<String, String>>,
    by: Option<String>,
    from_clock: Option<i64>,
    from_iso: Option<String>,
    to_clock: Option<i64>,
    to_iso: Option<String>,
}

impl ActivityItemOutBuilder {
    pub fn actor_type(mut self, value: impl Into<String>) -> Self {
        self.actor_type = Some(value.into());
        self
    }

    pub fn attributions(mut self, value: HashMap<String, String>) -> Self {
        self.attributions = Some(value);
        self
    }

    pub fn by(mut self, value: impl Into<String>) -> Self {
        self.by = Some(value.into());
        self
    }

    pub fn from_clock(mut self, value: i64) -> Self {
        self.from_clock = Some(value);
        self
    }

    pub fn from_iso(mut self, value: impl Into<String>) -> Self {
        self.from_iso = Some(value.into());
        self
    }

    pub fn to_clock(mut self, value: i64) -> Self {
        self.to_clock = Some(value);
        self
    }

    pub fn to_iso(mut self, value: impl Into<String>) -> Self {
        self.to_iso = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivityItemOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor_type`](ActivityItemOutBuilder::actor_type)
    /// - [`by`](ActivityItemOutBuilder::by)
    /// - [`from_clock`](ActivityItemOutBuilder::from_clock)
    /// - [`from_iso`](ActivityItemOutBuilder::from_iso)
    /// - [`to_clock`](ActivityItemOutBuilder::to_clock)
    /// - [`to_iso`](ActivityItemOutBuilder::to_iso)
    pub fn build(self) -> Result<ActivityItemOut, BuildError> {
        Ok(ActivityItemOut {
            actor_type: self.actor_type.ok_or_else(|| BuildError::missing_field("actor_type"))?,
            attributions: self.attributions,
            by: self.by.ok_or_else(|| BuildError::missing_field("by"))?,
            from_clock: self.from_clock.ok_or_else(|| BuildError::missing_field("from_clock"))?,
            from_iso: self.from_iso.ok_or_else(|| BuildError::missing_field("from_iso"))?,
            to_clock: self.to_clock.ok_or_else(|| BuildError::missing_field("to_clock"))?,
            to_iso: self.to_iso.ok_or_else(|| BuildError::missing_field("to_iso"))?,
        })
    }
}
