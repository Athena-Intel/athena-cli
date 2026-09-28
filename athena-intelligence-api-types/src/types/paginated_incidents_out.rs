pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One offset page of the incidents a caller may see, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginatedIncidentsOut {
    #[serde(default)]
    pub has_more: bool,
    /// Incidents matching the filter that the caller may not see: counted, never returned
    #[serde(default)]
    pub hidden_count: i64,
    #[serde(default)]
    pub items: Vec<IncidentOut>,
    #[serde(default)]
    pub limit: i64,
    /// Offset for the next page, null when there is none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub offset: i64,
    /// Incidents matching the filter that the caller may see
    #[serde(default)]
    pub total: i64,
}

impl PaginatedIncidentsOut {
    pub fn builder() -> PaginatedIncidentsOutBuilder {
        <PaginatedIncidentsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedIncidentsOutBuilder {
    has_more: Option<bool>,
    hidden_count: Option<i64>,
    items: Option<Vec<IncidentOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedIncidentsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn hidden_count(mut self, value: i64) -> Self {
        self.hidden_count = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<IncidentOut>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedIncidentsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedIncidentsOutBuilder::has_more)
    /// - [`hidden_count`](PaginatedIncidentsOutBuilder::hidden_count)
    /// - [`items`](PaginatedIncidentsOutBuilder::items)
    /// - [`limit`](PaginatedIncidentsOutBuilder::limit)
    /// - [`offset`](PaginatedIncidentsOutBuilder::offset)
    /// - [`total`](PaginatedIncidentsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedIncidentsOut, BuildError> {
        Ok(PaginatedIncidentsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            hidden_count: self.hidden_count.ok_or_else(|| BuildError::missing_field("hidden_count"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
