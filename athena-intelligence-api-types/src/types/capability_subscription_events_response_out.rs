pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the events one of the caller's subscriptions heard.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscriptionEventsResponseOut {
    #[serde(default)]
    pub events: Vec<CapabilitySubscriptionEventOut>,
    /// Pass as cursor for the next page; null on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl CapabilitySubscriptionEventsResponseOut {
    pub fn builder() -> CapabilitySubscriptionEventsResponseOutBuilder {
        <CapabilitySubscriptionEventsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionEventsResponseOutBuilder {
    events: Option<Vec<CapabilitySubscriptionEventOut>>,
    next_cursor: Option<String>,
}

impl CapabilitySubscriptionEventsResponseOutBuilder {
    pub fn events(mut self, value: Vec<CapabilitySubscriptionEventOut>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionEventsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](CapabilitySubscriptionEventsResponseOutBuilder::events)
    pub fn build(self) -> Result<CapabilitySubscriptionEventsResponseOut, BuildError> {
        Ok(CapabilitySubscriptionEventsResponseOut {
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            next_cursor: self.next_cursor,
        })
    }
}
