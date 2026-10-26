pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaitInput {
    /// One fire after this delay from now: 15m, 8h, 7d (at most 90d).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// One fire at this instant (RFC 3339, e.g. 2026-10-16T09:00:00-04:00), at least a minute and at most 90 days ahead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// cron only: read it against a business calendar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<SubscriptionCalendarInput>,
    /// Recurring: a 5-field cron in timezone, e.g. '0 9 * * 1' for Mondays at 9:00. Avoid :00/:30 when the time is approximate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    /// cron only: how long it lives, up to 90d (30d by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// cron only: most turns it may start, 1 to 100 (50 by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runs: Option<i64>,
    /// What future-you needs to resume (at most 500 characters); it is read back to you when you wake.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Cancel this conversation's pending one-off wakes instead of setting one (other arguments are ignored). A recurring wait is cancelled through manage_subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    /// IANA time zone for cron and for an at without an offset (UTC by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl WaitInput {
    pub fn builder() -> WaitInputBuilder {
        <WaitInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitInputBuilder {
    after: Option<String>,
    at: Option<String>,
    calendar: Option<SubscriptionCalendarInput>,
    cron: Option<String>,
    expires: Option<String>,
    max_runs: Option<i64>,
    note: Option<String>,
    stop: Option<bool>,
    timezone: Option<String>,
}

impl WaitInputBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn calendar(mut self, value: SubscriptionCalendarInput) -> Self {
        self.calendar = Some(value);
        self
    }

    pub fn cron(mut self, value: impl Into<String>) -> Self {
        self.cron = Some(value.into());
        self
    }

    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(value.into());
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

    pub fn stop(mut self, value: bool) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaitInput`].
    pub fn build(self) -> Result<WaitInput, BuildError> {
        Ok(WaitInput {
            after: self.after,
            at: self.at,
            calendar: self.calendar,
            cron: self.cron,
            expires: self.expires,
            max_runs: self.max_runs,
            note: self.note,
            stop: self.stop,
            timezone: self.timezone,
        })
    }
}

