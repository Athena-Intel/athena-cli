pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of `GET /tools/calendar/events`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalendarEventsResponseOut {
    /// The connected account that was read.
    #[serde(default)]
    pub catalog_id: String,
    /// Number of events returned.
    #[serde(default)]
    pub count: i64,
    /// Account provider as Athena reports it: `gmail` or `outlook` for accounts connected through the Integrations page; `google` or `microsoft365` for directly-connected accounts (read-only for drafts).
    #[serde(default)]
    pub provider: String,
    /// Matching events in ascending start order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CalendarEventOut>>,
}

impl CalendarEventsResponseOut {
    pub fn builder() -> CalendarEventsResponseOutBuilder {
        <CalendarEventsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalendarEventsResponseOutBuilder {
    catalog_id: Option<String>,
    count: Option<i64>,
    provider: Option<String>,
    results: Option<Vec<CalendarEventOut>>,
}

impl CalendarEventsResponseOutBuilder {
    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<CalendarEventOut>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalendarEventsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_id`](CalendarEventsResponseOutBuilder::catalog_id)
    /// - [`count`](CalendarEventsResponseOutBuilder::count)
    /// - [`provider`](CalendarEventsResponseOutBuilder::provider)
    pub fn build(self) -> Result<CalendarEventsResponseOut, BuildError> {
        Ok(CalendarEventsResponseOut {
            catalog_id: self.catalog_id.ok_or_else(|| BuildError::missing_field("catalog_id"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            results: self.results,
        })
    }
}
