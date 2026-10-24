pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How often a subscription may start a turn.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionRateOut {
    #[serde(default)]
    pub max_fires_per_hour: i64,
}

impl CapabilitySubscriptionRateOut {
    pub fn builder() -> CapabilitySubscriptionRateOutBuilder {
        <CapabilitySubscriptionRateOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionRateOutBuilder {
    max_fires_per_hour: Option<i64>,
}

impl CapabilitySubscriptionRateOutBuilder {
    pub fn max_fires_per_hour(mut self, value: i64) -> Self {
        self.max_fires_per_hour = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionRateOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_fires_per_hour`](CapabilitySubscriptionRateOutBuilder::max_fires_per_hour)
    pub fn build(self) -> Result<CapabilitySubscriptionRateOut, BuildError> {
        Ok(CapabilitySubscriptionRateOut {
            max_fires_per_hour: self.max_fires_per_hour.ok_or_else(|| BuildError::missing_field("max_fires_per_hour"))?,
        })
    }
}
