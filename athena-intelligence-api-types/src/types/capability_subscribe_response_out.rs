pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The subscription, whether this call created it, and what to do next.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilitySubscribeResponseOut {
    /// False when an identical open subscription already existed.
    #[serde(default)]
    pub created: bool,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub subscription: CapabilitySubscriptionOut,
}

impl CapabilitySubscribeResponseOut {
    pub fn builder() -> CapabilitySubscribeResponseOutBuilder {
        <CapabilitySubscribeResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscribeResponseOutBuilder {
    created: Option<bool>,
    next: Option<String>,
    subscription: Option<CapabilitySubscriptionOut>,
}

impl CapabilitySubscribeResponseOutBuilder {
    pub fn created(mut self, value: bool) -> Self {
        self.created = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn subscription(mut self, value: CapabilitySubscriptionOut) -> Self {
        self.subscription = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscribeResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](CapabilitySubscribeResponseOutBuilder::created)
    /// - [`next`](CapabilitySubscribeResponseOutBuilder::next)
    /// - [`subscription`](CapabilitySubscribeResponseOutBuilder::subscription)
    pub fn build(self) -> Result<CapabilitySubscribeResponseOut, BuildError> {
        Ok(CapabilitySubscribeResponseOut {
            created: self.created.ok_or_else(|| BuildError::missing_field("created"))?,
            next: self.next.ok_or_else(|| BuildError::missing_field("next"))?,
            subscription: self.subscription.ok_or_else(|| BuildError::missing_field("subscription"))?,
        })
    }
}
