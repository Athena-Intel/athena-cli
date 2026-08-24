pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchAssetsInput {
    /// Filter assets created after this ISO date string (e.g., YYYY-MM-DDTHH:MM:SS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_after: Option<String>,
    /// Filter assets created before this ISO date string (e.g., YYYY-MM-DDTHH:MM:SS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_before: Option<String>,
    /// Filter by creator email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Maximum number of results to return (defaults to 25).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of asset types to explicitly exclude from the results. Provide as an array like ['meeting', 'document'], not as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_types: Option<Vec<String>>,
    /// Offset for pagination (defaults to 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort direction, either 'asc' or 'desc'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// Field to sort by. Accepts: 'title', 'createdAt'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// List of tags to filter assets by. Returns assets that have any of the specified tags. Provide as an array like ['GTM Opportunity', 'Priority'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Keyword filter for asset title. The search will find assets where the title contains the provided query string. Use empty string to return all assets or None for no text query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_contains_keyword: Option<String>,
    /// List of asset types to include in the results. eg: meeting, document, spreadsheet. Accepts lists without escape characters. Do not use backslashes like \.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl SearchAssetsInput {
    pub fn builder() -> SearchAssetsInputBuilder {
        <SearchAssetsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchAssetsInputBuilder {
    created_at_after: Option<String>,
    created_at_before: Option<String>,
    created_by: Option<String>,
    limit: Option<i64>,
    not_types: Option<Vec<String>>,
    offset: Option<i64>,
    sort_direction: Option<String>,
    sort_field: Option<String>,
    tags: Option<Vec<String>>,
    title_contains_keyword: Option<String>,
    types: Option<Vec<String>>,
}

impl SearchAssetsInputBuilder {
    pub fn created_at_after(mut self, value: impl Into<String>) -> Self {
        self.created_at_after = Some(value.into());
        self
    }

    pub fn created_at_before(mut self, value: impl Into<String>) -> Self {
        self.created_at_before = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn not_types(mut self, value: Vec<String>) -> Self {
        self.not_types = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: impl Into<String>) -> Self {
        self.sort_direction = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: impl Into<String>) -> Self {
        self.sort_field = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title_contains_keyword(mut self, value: impl Into<String>) -> Self {
        self.title_contains_keyword = Some(value.into());
        self
    }

    pub fn types(mut self, value: Vec<String>) -> Self {
        self.types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchAssetsInput`].
    pub fn build(self) -> Result<SearchAssetsInput, BuildError> {
        Ok(SearchAssetsInput {
            created_at_after: self.created_at_after,
            created_at_before: self.created_at_before,
            created_by: self.created_by,
            limit: self.limit,
            not_types: self.not_types,
            offset: self.offset,
            sort_direction: self.sort_direction,
            sort_field: self.sort_field,
            tags: self.tags,
            title_contains_keyword: self.title_contains_keyword,
            types: self.types,
        })
    }
}

