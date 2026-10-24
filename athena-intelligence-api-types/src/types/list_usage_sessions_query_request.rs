pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_usage_sessions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListUsageSessionsQueryRequest {
    /// First UTC day of the window (YYYY-MM-DD); requires `to`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Last UTC day of the window, inclusive (YYYY-MM-DD); requires `from`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Rolling window in days ending now; ignored when `from`/`to` are given
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// `next_cursor` from the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum sessions per page (each may yield several rows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListUsageSessionsQueryRequest {
    pub fn builder() -> ListUsageSessionsQueryRequestBuilder {
        <ListUsageSessionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUsageSessionsQueryRequestBuilder {
    from: Option<String>,
    to: Option<String>,
    days: Option<i64>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListUsageSessionsQueryRequestBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUsageSessionsQueryRequest`].
    pub fn build(self) -> Result<ListUsageSessionsQueryRequest, BuildError> {
        Ok(ListUsageSessionsQueryRequest {
            from: self.from,
            to: self.to,
            days: self.days,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

