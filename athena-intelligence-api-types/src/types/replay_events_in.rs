pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayEventsIn {
    /// Continue the window after a previous report's `next_cursor` (the same window, types and target)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The event types to replay, at most 16
    #[serde(default)]
    pub event_types: Vec<String>,
    /// How many events to read, 1 to 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// `shadow` (the default) evaluates and writes nothing; `live` re-sends each would-fire event as a copy that fires only this automation's rules, starting its runs (needs `source: published` and EDIT on the automation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ReplayEventsInMode>,
    /// Start of the window on the audit row's received time, inclusive
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub since: DateTime<FixedOffset>,
    /// Whose trigger shapes to evaluate: the current draft compiled in memory (the default) or the published trigger rows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ReplayEventsInSource>,
    /// The automation whose triggers the events are evaluated against
    #[serde(default)]
    pub target_automation_asset_id: String,
    /// End of the window, exclusive; at most 31 days after `since`
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub until: DateTime<FixedOffset>,
}

impl ReplayEventsIn {
    pub fn builder() -> ReplayEventsInBuilder {
        <ReplayEventsInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayEventsInBuilder {
    after: Option<String>,
    event_types: Option<Vec<String>>,
    limit: Option<i64>,
    mode: Option<ReplayEventsInMode>,
    since: Option<DateTime<FixedOffset>>,
    source: Option<ReplayEventsInSource>,
    target_automation_asset_id: Option<String>,
    until: Option<DateTime<FixedOffset>>,
}

impl ReplayEventsInBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn event_types(mut self, value: Vec<String>) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn mode(mut self, value: ReplayEventsInMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since = Some(value);
        self
    }

    pub fn source(mut self, value: ReplayEventsInSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target_automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.target_automation_asset_id = Some(value.into());
        self
    }

    pub fn until(mut self, value: DateTime<FixedOffset>) -> Self {
        self.until = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayEventsIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_types`](ReplayEventsInBuilder::event_types)
    /// - [`since`](ReplayEventsInBuilder::since)
    /// - [`target_automation_asset_id`](ReplayEventsInBuilder::target_automation_asset_id)
    /// - [`until`](ReplayEventsInBuilder::until)
    pub fn build(self) -> Result<ReplayEventsIn, BuildError> {
        Ok(ReplayEventsIn {
            after: self.after,
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            limit: self.limit,
            mode: self.mode,
            since: self.since.ok_or_else(|| BuildError::missing_field("since"))?,
            source: self.source,
            target_automation_asset_id: self.target_automation_asset_id.ok_or_else(|| BuildError::missing_field("target_automation_asset_id"))?,
            until: self.until.ok_or_else(|| BuildError::missing_field("until"))?,
        })
    }
}

