pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the caller's own subscriptions, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscriptionListResponseOut {
    /// Pass as cursor for the next page; null on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub subscriptions: Vec<CapabilitySubscriptionOut>,
}

impl CapabilitySubscriptionListResponseOut {
    pub fn builder() -> CapabilitySubscriptionListResponseOutBuilder {
        <CapabilitySubscriptionListResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionListResponseOutBuilder {
    next_cursor: Option<String>,
    subscriptions: Option<Vec<CapabilitySubscriptionOut>>,
}

impl CapabilitySubscriptionListResponseOutBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn subscriptions(mut self, value: Vec<CapabilitySubscriptionOut>) -> Self {
        self.subscriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionListResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`subscriptions`](CapabilitySubscriptionListResponseOutBuilder::subscriptions)
    pub fn build(self) -> Result<CapabilitySubscriptionListResponseOut, BuildError> {
        Ok(CapabilitySubscriptionListResponseOut {
            next_cursor: self.next_cursor,
            subscriptions: self.subscriptions.ok_or_else(|| BuildError::missing_field("subscriptions"))?,
        })
    }
}
