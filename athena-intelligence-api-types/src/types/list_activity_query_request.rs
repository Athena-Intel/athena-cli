pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_activity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListActivityQueryRequest {
    /// Maximum items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Return only items at or before this clock. Pass the previous response's next_page_to_clock to page backwards through history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_clock: Option<i64>,
}

impl ListActivityQueryRequest {
    pub fn builder() -> ListActivityQueryRequestBuilder {
        <ListActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListActivityQueryRequestBuilder {
    limit: Option<i64>,
    to_clock: Option<i64>,
}

impl ListActivityQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn to_clock(mut self, value: i64) -> Self {
        self.to_clock = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListActivityQueryRequest`].
    pub fn build(self) -> Result<ListActivityQueryRequest, BuildError> {
        Ok(ListActivityQueryRequest {
            limit: self.limit,
            to_clock: self.to_clock,
        })
    }
}

