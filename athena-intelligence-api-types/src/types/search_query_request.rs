pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueryRequest {
    /// Search query. Gmail operators (`from:`, `to:`, `subject:`, `has:attachment`, `newer_than:7d`, `-term`, …) are accepted for both providers; operators with no Outlook equivalent are dropped and reported in `ignored_operators`. Use `in:drafts` to search only unsent drafts.
    #[serde(default)]
    pub query: String,
    /// Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// Maximum number of results (1-50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl SearchQueryRequest {
    pub fn builder() -> SearchQueryRequestBuilder {
        <SearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueryRequestBuilder {
    query: Option<String>,
    catalog_id: Option<String>,
    limit: Option<i64>,
}

impl SearchQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchQueryRequestBuilder::query)
    pub fn build(self) -> Result<SearchQueryRequest, BuildError> {
        Ok(SearchQueryRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            catalog_id: self.catalog_id,
            limit: self.limit,
        })
    }
}

