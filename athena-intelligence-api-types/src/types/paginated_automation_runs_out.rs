pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One offset page of an automation's runs, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedAutomationRunsOut {
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<AutomationRunOut>,
    #[serde(default)]
    pub limit: i64,
    /// Offset for the next page, null when there is none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub offset: i64,
    /// Runs matching the filter
    #[serde(default)]
    pub total: i64,
}

impl PaginatedAutomationRunsOut {
    pub fn builder() -> PaginatedAutomationRunsOutBuilder {
        <PaginatedAutomationRunsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedAutomationRunsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<AutomationRunOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedAutomationRunsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<AutomationRunOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_offset(mut self, value: i64) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedAutomationRunsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedAutomationRunsOutBuilder::has_more)
    /// - [`items`](PaginatedAutomationRunsOutBuilder::items)
    /// - [`limit`](PaginatedAutomationRunsOutBuilder::limit)
    /// - [`offset`](PaginatedAutomationRunsOutBuilder::offset)
    /// - [`total`](PaginatedAutomationRunsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedAutomationRunsOut, BuildError> {
        Ok(PaginatedAutomationRunsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
