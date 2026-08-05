pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The change one activity item made, or why it is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActivityDeltaOut {
    /// The change itself. Always carries kind, type, summary, totals, and coverage; typed detail (cell_changes, slide_changes, inserted_text, ...) varies by kind. Null when error is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<HashMap<String, serde_json::Value>>,
    /// Why this range could not be diffed. Null on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub from_clock: i64,
    #[serde(default)]
    pub to_clock: i64,
}

impl ActivityDeltaOut {
    pub fn builder() -> ActivityDeltaOutBuilder {
        <ActivityDeltaOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityDeltaOutBuilder {
    delta: Option<HashMap<String, serde_json::Value>>,
    error: Option<String>,
    from_clock: Option<i64>,
    to_clock: Option<i64>,
}

impl ActivityDeltaOutBuilder {
    pub fn delta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.delta = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn from_clock(mut self, value: i64) -> Self {
        self.from_clock = Some(value);
        self
    }

    pub fn to_clock(mut self, value: i64) -> Self {
        self.to_clock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActivityDeltaOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_clock`](ActivityDeltaOutBuilder::from_clock)
    /// - [`to_clock`](ActivityDeltaOutBuilder::to_clock)
    pub fn build(self) -> Result<ActivityDeltaOut, BuildError> {
        Ok(ActivityDeltaOut {
            delta: self.delta,
            error: self.error,
            from_clock: self.from_clock.ok_or_else(|| BuildError::missing_field("from_clock"))?,
            to_clock: self.to_clock.ok_or_else(|| BuildError::missing_field("to_clock"))?,
        })
    }
}
