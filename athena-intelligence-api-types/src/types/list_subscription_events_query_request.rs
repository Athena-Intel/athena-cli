pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_subscription_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSubscriptionEventsQueryRequest {
    /// Only events queued and not yet delivered (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// The next_cursor of the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListSubscriptionEventsQueryRequest {
    pub fn builder() -> ListSubscriptionEventsQueryRequestBuilder {
        <ListSubscriptionEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSubscriptionEventsQueryRequestBuilder {
    pending: Option<bool>,
    cursor: Option<String>,
}

impl ListSubscriptionEventsQueryRequestBuilder {
    pub fn pending(mut self, value: bool) -> Self {
        self.pending = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSubscriptionEventsQueryRequest`].
    pub fn build(self) -> Result<ListSubscriptionEventsQueryRequest, BuildError> {
        Ok(ListSubscriptionEventsQueryRequest {
            pending: self.pending,
            cursor: self.cursor,
        })
    }
}

