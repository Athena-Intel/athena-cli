pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated response containing a list of meetings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginatedMeetingsOut {
    /// Whether there are more meetings available beyond this page
    #[serde(default)]
    pub has_more: bool,
    /// Array of meeting objects for the current page
    #[serde(default)]
    pub items: Vec<MeetingOut>,
    /// Maximum number of meetings returned in this response (1-500)
    #[serde(default)]
    pub limit: i64,
    /// Offset value to use for the next page request, null if no more pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    /// Number of meetings skipped from the beginning of the result set
    #[serde(default)]
    pub offset: i64,
    /// Total number of meetings matching the query filters
    #[serde(default)]
    pub total: i64,
}

impl PaginatedMeetingsOut {
    pub fn builder() -> PaginatedMeetingsOutBuilder {
        <PaginatedMeetingsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedMeetingsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<MeetingOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedMeetingsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<MeetingOut>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedMeetingsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedMeetingsOutBuilder::has_more)
    /// - [`items`](PaginatedMeetingsOutBuilder::items)
    /// - [`limit`](PaginatedMeetingsOutBuilder::limit)
    /// - [`offset`](PaginatedMeetingsOutBuilder::offset)
    /// - [`total`](PaginatedMeetingsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedMeetingsOut, BuildError> {
        Ok(PaginatedMeetingsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
