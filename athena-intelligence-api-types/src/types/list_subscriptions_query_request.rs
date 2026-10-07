pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSubscriptionsQueryRequest {
    /// Only subscriptions on this asset (VIEW required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Also list cancelled and expired subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_closed: Option<bool>,
    /// The next_cursor of the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListSubscriptionsQueryRequest {
    pub fn builder() -> ListSubscriptionsQueryRequestBuilder {
        <ListSubscriptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSubscriptionsQueryRequestBuilder {
    asset_id: Option<String>,
    include_closed: Option<bool>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListSubscriptionsQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn include_closed(mut self, value: bool) -> Self {
        self.include_closed = Some(value);
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

    /// Consumes the builder and constructs a [`ListSubscriptionsQueryRequest`].
    pub fn build(self) -> Result<ListSubscriptionsQueryRequest, BuildError> {
        Ok(ListSubscriptionsQueryRequest {
            asset_id: self.asset_id,
            include_closed: self.include_closed,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

