pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_batch_status
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBatchStatusQueryRequest {
    /// Which runs to list: `terminal` (completed/failed/canceled), `active` (everything else) or `all`. Counts always cover the whole batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// `next_cursor` from the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum runs to return in this page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetBatchStatusQueryRequest {
    pub fn builder() -> GetBatchStatusQueryRequestBuilder {
        <GetBatchStatusQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBatchStatusQueryRequestBuilder {
    status: Option<String>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl GetBatchStatusQueryRequestBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetBatchStatusQueryRequest`].
    pub fn build(self) -> Result<GetBatchStatusQueryRequest, BuildError> {
        Ok(GetBatchStatusQueryRequest {
            status: self.status,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

