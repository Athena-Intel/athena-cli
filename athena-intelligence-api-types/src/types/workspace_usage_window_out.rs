pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The reporting window a usage page covers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceUsageWindowOut {
    /// Number of days the window spans
    #[serde(default)]
    pub days: i64,
    /// Last UTC day (inclusive) of an explicit window; null for a rolling window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// First UTC day of an explicit `from`/`to` window; null for a rolling window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Window end (exclusive); null means 'now' for a rolling window
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub usage_period_end_exclusive: Option<DateTime<FixedOffset>>,
    /// Window start (inclusive)
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub usage_period_start: DateTime<FixedOffset>,
}

impl WorkspaceUsageWindowOut {
    pub fn builder() -> WorkspaceUsageWindowOutBuilder {
        <WorkspaceUsageWindowOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageWindowOutBuilder {
    days: Option<i64>,
    end_date: Option<NaiveDate>,
    start_date: Option<NaiveDate>,
    usage_period_end_exclusive: Option<DateTime<FixedOffset>>,
    usage_period_start: Option<DateTime<FixedOffset>>,
}

impl WorkspaceUsageWindowOutBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn usage_period_end_exclusive(mut self, value: DateTime<FixedOffset>) -> Self {
        self.usage_period_end_exclusive = Some(value);
        self
    }

    pub fn usage_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.usage_period_start = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageWindowOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days`](WorkspaceUsageWindowOutBuilder::days)
    /// - [`usage_period_start`](WorkspaceUsageWindowOutBuilder::usage_period_start)
    pub fn build(self) -> Result<WorkspaceUsageWindowOut, BuildError> {
        Ok(WorkspaceUsageWindowOut {
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            end_date: self.end_date,
            start_date: self.start_date,
            usage_period_end_exclusive: self.usage_period_end_exclusive,
            usage_period_start: self.usage_period_start.ok_or_else(|| BuildError::missing_field("usage_period_start"))?,
        })
    }
}
