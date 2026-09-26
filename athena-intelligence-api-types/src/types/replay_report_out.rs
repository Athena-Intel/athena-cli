pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What one replay evaluated, compared and (live) re-sent; never a payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplayReportOut {
    /// Events both sides fired on
    #[serde(default)]
    pub agree: i64,
    /// Live: copies an earlier call already sent for this target and published version, which were not sent again
    #[serde(default)]
    pub already_replayed_event_ids: Vec<String>,
    /// What the verdicts cannot know and what was left out
    #[serde(default)]
    pub caveats: Vec<String>,
    #[serde(default)]
    pub events_considered: i64,
    /// Events with at least one legacy execution
    #[serde(default)]
    pub legacy_fired: i64,
    pub mode: ReplayReportOutMode,
    /// When truncated: pass it as `after` to read the rest of the window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Events the target fires on and the legacy side did not
    #[serde(default)]
    pub only_automation: i64,
    /// Events the legacy side fired on and the target does not
    #[serde(default)]
    pub only_legacy: i64,
    /// Live: the copies this call enqueued (empty for shadow)
    #[serde(default)]
    pub replayed_event_ids: Vec<String>,
    pub source: ReplayReportOutSource,
    #[serde(default)]
    pub target_automation_asset_id: String,
    /// Events the target's own rules recorded an execution for
    #[serde(default)]
    pub target_fired: i64,
    /// True when the window holds more events than `limit`
    #[serde(default)]
    pub truncated: bool,
    /// One verdict per event, oldest first
    #[serde(default)]
    pub verdicts: Vec<ReplayVerdictOut>,
    /// Events the target fires on
    #[serde(default)]
    pub would_fire: i64,
}

impl ReplayReportOut {
    pub fn builder() -> ReplayReportOutBuilder {
        <ReplayReportOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayReportOutBuilder {
    agree: Option<i64>,
    already_replayed_event_ids: Option<Vec<String>>,
    caveats: Option<Vec<String>>,
    events_considered: Option<i64>,
    legacy_fired: Option<i64>,
    mode: Option<ReplayReportOutMode>,
    next_cursor: Option<String>,
    only_automation: Option<i64>,
    only_legacy: Option<i64>,
    replayed_event_ids: Option<Vec<String>>,
    source: Option<ReplayReportOutSource>,
    target_automation_asset_id: Option<String>,
    target_fired: Option<i64>,
    truncated: Option<bool>,
    verdicts: Option<Vec<ReplayVerdictOut>>,
    would_fire: Option<i64>,
}

impl ReplayReportOutBuilder {
    pub fn agree(mut self, value: i64) -> Self {
        self.agree = Some(value);
        self
    }

    pub fn already_replayed_event_ids(mut self, value: Vec<String>) -> Self {
        self.already_replayed_event_ids = Some(value);
        self
    }

    pub fn caveats(mut self, value: Vec<String>) -> Self {
        self.caveats = Some(value);
        self
    }

    pub fn events_considered(mut self, value: i64) -> Self {
        self.events_considered = Some(value);
        self
    }

    pub fn legacy_fired(mut self, value: i64) -> Self {
        self.legacy_fired = Some(value);
        self
    }

    pub fn mode(mut self, value: ReplayReportOutMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn only_automation(mut self, value: i64) -> Self {
        self.only_automation = Some(value);
        self
    }

    pub fn only_legacy(mut self, value: i64) -> Self {
        self.only_legacy = Some(value);
        self
    }

    pub fn replayed_event_ids(mut self, value: Vec<String>) -> Self {
        self.replayed_event_ids = Some(value);
        self
    }

    pub fn source(mut self, value: ReplayReportOutSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target_automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.target_automation_asset_id = Some(value.into());
        self
    }

    pub fn target_fired(mut self, value: i64) -> Self {
        self.target_fired = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    pub fn verdicts(mut self, value: Vec<ReplayVerdictOut>) -> Self {
        self.verdicts = Some(value);
        self
    }

    pub fn would_fire(mut self, value: i64) -> Self {
        self.would_fire = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayReportOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agree`](ReplayReportOutBuilder::agree)
    /// - [`already_replayed_event_ids`](ReplayReportOutBuilder::already_replayed_event_ids)
    /// - [`caveats`](ReplayReportOutBuilder::caveats)
    /// - [`events_considered`](ReplayReportOutBuilder::events_considered)
    /// - [`legacy_fired`](ReplayReportOutBuilder::legacy_fired)
    /// - [`mode`](ReplayReportOutBuilder::mode)
    /// - [`only_automation`](ReplayReportOutBuilder::only_automation)
    /// - [`only_legacy`](ReplayReportOutBuilder::only_legacy)
    /// - [`replayed_event_ids`](ReplayReportOutBuilder::replayed_event_ids)
    /// - [`source`](ReplayReportOutBuilder::source)
    /// - [`target_automation_asset_id`](ReplayReportOutBuilder::target_automation_asset_id)
    /// - [`target_fired`](ReplayReportOutBuilder::target_fired)
    /// - [`truncated`](ReplayReportOutBuilder::truncated)
    /// - [`verdicts`](ReplayReportOutBuilder::verdicts)
    /// - [`would_fire`](ReplayReportOutBuilder::would_fire)
    pub fn build(self) -> Result<ReplayReportOut, BuildError> {
        Ok(ReplayReportOut {
            agree: self.agree.ok_or_else(|| BuildError::missing_field("agree"))?,
            already_replayed_event_ids: self.already_replayed_event_ids.ok_or_else(|| BuildError::missing_field("already_replayed_event_ids"))?,
            caveats: self.caveats.ok_or_else(|| BuildError::missing_field("caveats"))?,
            events_considered: self.events_considered.ok_or_else(|| BuildError::missing_field("events_considered"))?,
            legacy_fired: self.legacy_fired.ok_or_else(|| BuildError::missing_field("legacy_fired"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            next_cursor: self.next_cursor,
            only_automation: self.only_automation.ok_or_else(|| BuildError::missing_field("only_automation"))?,
            only_legacy: self.only_legacy.ok_or_else(|| BuildError::missing_field("only_legacy"))?,
            replayed_event_ids: self.replayed_event_ids.ok_or_else(|| BuildError::missing_field("replayed_event_ids"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            target_automation_asset_id: self.target_automation_asset_id.ok_or_else(|| BuildError::missing_field("target_automation_asset_id"))?,
            target_fired: self.target_fired.ok_or_else(|| BuildError::missing_field("target_fired"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
            verdicts: self.verdicts.ok_or_else(|| BuildError::missing_field("verdicts"))?,
            would_fire: self.would_fire.ok_or_else(|| BuildError::missing_field("would_fire"))?,
        })
    }
}
