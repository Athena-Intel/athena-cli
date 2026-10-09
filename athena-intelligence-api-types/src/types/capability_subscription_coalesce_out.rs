pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Events inside this window arrive as one delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionCoalesceOut {
    #[serde(default)]
    pub debounce_seconds: i64,
}

impl CapabilitySubscriptionCoalesceOut {
    pub fn builder() -> CapabilitySubscriptionCoalesceOutBuilder {
        <CapabilitySubscriptionCoalesceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionCoalesceOutBuilder {
    debounce_seconds: Option<i64>,
}

impl CapabilitySubscriptionCoalesceOutBuilder {
    pub fn debounce_seconds(mut self, value: i64) -> Self {
        self.debounce_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionCoalesceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`debounce_seconds`](CapabilitySubscriptionCoalesceOutBuilder::debounce_seconds)
    pub fn build(self) -> Result<CapabilitySubscriptionCoalesceOut, BuildError> {
        Ok(CapabilitySubscriptionCoalesceOut {
            debounce_seconds: self.debounce_seconds.ok_or_else(|| BuildError::missing_field("debounce_seconds"))?,
        })
    }
}
