pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One offset page of the automations the caller may view, newest change first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedAutomationsOut {
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<AutomationListItemOut>,
    #[serde(default)]
    pub limit: i64,
    /// Offset for the next page, null when there is none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub offset: i64,
    /// Automations matching the filter, among the 1000 most recently updated ones the caller may view
    #[serde(default)]
    pub total: i64,
}

impl PaginatedAutomationsOut {
    pub fn builder() -> PaginatedAutomationsOutBuilder {
        <PaginatedAutomationsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedAutomationsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<AutomationListItemOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedAutomationsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<AutomationListItemOut>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedAutomationsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedAutomationsOutBuilder::has_more)
    /// - [`items`](PaginatedAutomationsOutBuilder::items)
    /// - [`limit`](PaginatedAutomationsOutBuilder::limit)
    /// - [`offset`](PaginatedAutomationsOutBuilder::offset)
    /// - [`total`](PaginatedAutomationsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedAutomationsOut, BuildError> {
        Ok(PaginatedAutomationsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
