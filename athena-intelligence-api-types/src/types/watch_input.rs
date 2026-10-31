pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WatchInput {
    /// kind event only: a CEL condition each event must meet. It reads event (type, time, data: the event's ids and status words), actor ({kind, user_id} or null), and for a spreadsheet change: cells (the changed cells' current values, each {a1, row, column, header, value}), changed (their headers) and row (when they sit in one row, their values by header). Headers are the first-row text, trimmed and lowercased. Example: 'row.status == "Passed"' or 'event.data.outcome == "failure"'. Needs a subscription that wakes this conversation (not when_idle hold or while_running prioritize).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// kind event only: 'tell me if nothing happens by then'. When the deadline passes with nothing delivered since it was set, one subscription.deadline.passed event wakes the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<SubscriptionDeadlineInput>,
    /// Usually leave unset. Seconds a wake waits after the first matching event, gathering the events that follow into one turn, 0 to 3600; each event type has its own default (none for spreadsheet edits). Content changes already arrive one per editing pause (about half a second after a spreadsheet edit, two seconds after typing stops in a document), and every second set here delays every wake by that much, so set it only when the person asked to hear about changes in batches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_seconds: Option<i64>,
    /// Event types to hear; each must be one the subject's capability card offers (read the card first), e.g. asset.content.changed, or github.pr.review.submitted and github.check_run.completed on a pull request. Prefer one watch with several events over several watches.
    #[serde(default)]
    pub events: Vec<String>,
    /// How long it lives: 5m to 7d (24h by default), or up to 90d (30d by default) with until.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Which events count. A filter judges only the events that take it, so one watch can hold a check conclusion beside reviews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SubscriptionFiltersInput>,
    /// Most turns it may start in an hour, 1 to 60 (20 by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fires_per_hour: Option<i64>,
    /// Most turns it may start, 1 to 100 (50 by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runs: Option<i64>,
    /// What you are watching for and what to do then, in your own words (at most 500 characters); it is read back to you when you wake.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub subject: WatchSubjectInput,
    /// With conditions: every_match (default) delivers every event that meets them; on_become_true only when they turn true, e.g. 'tell me when Status becomes Passed'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<WatchInputTransition>,
    /// Events that end a watch on one pull request or issue (its subject item or link): github.pr.merged, github.pr.closed (closed without merging), github.issue.closed. Each must also be one of its events; the event itself is still delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<Vec<String>>,
    /// While this conversation is idle: wake starts a turn (the default, and the only choice for a schedule); for an event trigger, digest batches and hold waits for the next turn anyone starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_idle: Option<WatchInputWhenIdle>,
    /// While you are mid-run: queue delivers it as the next turn, notify also tells you it is waiting, prioritize ends the run at its next step. Never interrupts a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_running: Option<WatchInputWhileRunning>,
}

impl WatchInput {
    pub fn builder() -> WatchInputBuilder {
        <WatchInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WatchInputBuilder {
    conditions: Option<String>,
    deadline: Option<SubscriptionDeadlineInput>,
    debounce_seconds: Option<i64>,
    events: Option<Vec<String>>,
    expires: Option<String>,
    filters: Option<SubscriptionFiltersInput>,
    max_fires_per_hour: Option<i64>,
    max_runs: Option<i64>,
    note: Option<String>,
    subject: Option<WatchSubjectInput>,
    transition: Option<WatchInputTransition>,
    until: Option<Vec<String>>,
    when_idle: Option<WatchInputWhenIdle>,
    while_running: Option<WatchInputWhileRunning>,
}

impl WatchInputBuilder {
    pub fn conditions(mut self, value: impl Into<String>) -> Self {
        self.conditions = Some(value.into());
        self
    }

    pub fn deadline(mut self, value: SubscriptionDeadlineInput) -> Self {
        self.deadline = Some(value);
        self
    }

    pub fn debounce_seconds(mut self, value: i64) -> Self {
        self.debounce_seconds = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(value.into());
        self
    }

    pub fn filters(mut self, value: SubscriptionFiltersInput) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn max_fires_per_hour(mut self, value: i64) -> Self {
        self.max_fires_per_hour = Some(value);
        self
    }

    pub fn max_runs(mut self, value: i64) -> Self {
        self.max_runs = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn subject(mut self, value: WatchSubjectInput) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn transition(mut self, value: WatchInputTransition) -> Self {
        self.transition = Some(value);
        self
    }

    pub fn until(mut self, value: Vec<String>) -> Self {
        self.until = Some(value);
        self
    }

    pub fn when_idle(mut self, value: WatchInputWhenIdle) -> Self {
        self.when_idle = Some(value);
        self
    }

    pub fn while_running(mut self, value: WatchInputWhileRunning) -> Self {
        self.while_running = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WatchInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](WatchInputBuilder::events)
    /// - [`subject`](WatchInputBuilder::subject)
    pub fn build(self) -> Result<WatchInput, BuildError> {
        Ok(WatchInput {
            conditions: self.conditions,
            deadline: self.deadline,
            debounce_seconds: self.debounce_seconds,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            expires: self.expires,
            filters: self.filters,
            max_fires_per_hour: self.max_fires_per_hour,
            max_runs: self.max_runs,
            note: self.note,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
            transition: self.transition,
            until: self.until,
            when_idle: self.when_idle,
            while_running: self.while_running,
        })
    }
}

