pub use crate::prelude::*;
use super::*;

/// Paginated response containing a list of assets with pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedAssetsOut {
    /// Whether there are more assets available beyond this page
    #[serde(default)]
    pub has_more: bool,
    /// Array of asset objects for the current page
    #[serde(default)]
    pub items: Vec<PublicAssetOut>,
    /// Maximum number of assets returned in this response (1-500)
    #[serde(default)]
    pub limit: i64,
    /// Offset value to use for the next page request, null if no more pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    /// Number of assets skipped from the beginning of the result set
    #[serde(default)]
    pub offset: i64,
    /// Total number of assets matching the query filters
    #[serde(default)]
    pub total: i64,
}

impl PaginatedAssetsOut {
    pub fn builder() -> PaginatedAssetsOutBuilder {
        <PaginatedAssetsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedAssetsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<PublicAssetOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedAssetsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<PublicAssetOut>) -> Self {
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

    /// Consumes the builder and constructs a [`PaginatedAssetsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedAssetsOutBuilder::has_more)
    /// - [`items`](PaginatedAssetsOutBuilder::items)
    /// - [`limit`](PaginatedAssetsOutBuilder::limit)
    /// - [`offset`](PaginatedAssetsOutBuilder::offset)
    /// - [`total`](PaginatedAssetsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedAssetsOut, BuildError> {
        Ok(PaginatedAssetsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
