pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated response containing a list of sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedSessionsOut {
    /// Whether there are more sessions available beyond this page
    #[serde(default)]
    pub has_more: bool,
    /// Array of session objects for the current page
    #[serde(default)]
    pub items: Vec<SessionOut>,
    /// Maximum number of sessions returned in this response (1-500)
    #[serde(default)]
    pub limit: i64,
    /// Offset value to use for the next page request, null if no more pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    /// Number of sessions skipped from the beginning of the result set
    #[serde(default)]
    pub offset: i64,
    /// Total number of sessions matching the query filters
    #[serde(default)]
    pub total: i64,
}

impl PaginatedSessionsOut {
    pub fn builder() -> PaginatedSessionsOutBuilder {
        <PaginatedSessionsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedSessionsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<SessionOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedSessionsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<SessionOut>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedSessionsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedSessionsOutBuilder::has_more)
    /// - [`items`](PaginatedSessionsOutBuilder::items)
    /// - [`limit`](PaginatedSessionsOutBuilder::limit)
    /// - [`offset`](PaginatedSessionsOutBuilder::offset)
    /// - [`total`](PaginatedSessionsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedSessionsOut, BuildError> {
        Ok(PaginatedSessionsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
