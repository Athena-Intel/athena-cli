pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryMeetingsInput {
    /// End date for filtering (ISO format YYYY-MM-DDTHH:MM:SS). Omit to search across all time (no upper bound).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range_end: Option<String>,
    /// Start date for filtering (ISO format YYYY-MM-DDTHH:MM:SS). Omit to search across all time (no lower bound).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range_start: Option<String>,
    /// Structured AND/OR domain filter for attendee email domains. Use operator 'AND' with domains to require attendees from all domains, 'OR' to match any. Example: {"operator": "OR", "domains": ["acme.com", "example.com"]}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_filter: Option<HashMap<String, serde_json::Value>>,
    /// Maximum number of results to return (default 30).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset for pagination (default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Structured AND/OR participant email filter. Use operator 'AND' with emails to require all, 'OR' to match any. Supports nested groups for compound expressions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_filter: Option<HashMap<String, serde_json::Value>>,
    /// Keyword to search for in meeting title, ai_summary, and transcript content (case insensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,
    /// Field to sort results by. Allowed values: created_at, updated_at, title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl QueryMeetingsInput {
    pub fn builder() -> QueryMeetingsInputBuilder {
        <QueryMeetingsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryMeetingsInputBuilder {
    date_range_end: Option<String>,
    date_range_start: Option<String>,
    domain_filter: Option<HashMap<String, serde_json::Value>>,
    limit: Option<i64>,
    offset: Option<i64>,
    participant_filter: Option<HashMap<String, serde_json::Value>>,
    search_keyword: Option<String>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
}

impl QueryMeetingsInputBuilder {
    pub fn date_range_end(mut self, value: impl Into<String>) -> Self {
        self.date_range_end = Some(value.into());
        self
    }

    pub fn date_range_start(mut self, value: impl Into<String>) -> Self {
        self.date_range_start = Some(value.into());
        self
    }

    pub fn domain_filter(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.domain_filter = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn participant_filter(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.participant_filter = Some(value);
        self
    }

    pub fn search_keyword(mut self, value: impl Into<String>) -> Self {
        self.search_keyword = Some(value.into());
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: impl Into<String>) -> Self {
        self.sort_direction = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryMeetingsInput`].
    pub fn build(self) -> Result<QueryMeetingsInput, BuildError> {
        Ok(QueryMeetingsInput {
            date_range_end: self.date_range_end,
            date_range_start: self.date_range_start,
            domain_filter: self.domain_filter,
            limit: self.limit,
            offset: self.offset,
            participant_filter: self.participant_filter,
            search_keyword: self.search_keyword,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
        })
    }
}

