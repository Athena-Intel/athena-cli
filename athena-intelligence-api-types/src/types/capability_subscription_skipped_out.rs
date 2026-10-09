pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Events a subscription heard and skipped, by reason, over the last 14 days.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionSkippedOut {
    #[serde(default)]
    pub conditions_false: i64,
    #[serde(default)]
    pub no_change: i64,
    #[serde(default)]
    pub subsumed: i64,
}

impl CapabilitySubscriptionSkippedOut {
    pub fn builder() -> CapabilitySubscriptionSkippedOutBuilder {
        <CapabilitySubscriptionSkippedOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionSkippedOutBuilder {
    conditions_false: Option<i64>,
    no_change: Option<i64>,
    subsumed: Option<i64>,
}

impl CapabilitySubscriptionSkippedOutBuilder {
    pub fn conditions_false(mut self, value: i64) -> Self {
        self.conditions_false = Some(value);
        self
    }

    pub fn no_change(mut self, value: i64) -> Self {
        self.no_change = Some(value);
        self
    }

    pub fn subsumed(mut self, value: i64) -> Self {
        self.subsumed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionSkippedOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions_false`](CapabilitySubscriptionSkippedOutBuilder::conditions_false)
    /// - [`no_change`](CapabilitySubscriptionSkippedOutBuilder::no_change)
    /// - [`subsumed`](CapabilitySubscriptionSkippedOutBuilder::subsumed)
    pub fn build(self) -> Result<CapabilitySubscriptionSkippedOut, BuildError> {
        Ok(CapabilitySubscriptionSkippedOut {
            conditions_false: self.conditions_false.ok_or_else(|| BuildError::missing_field("conditions_false"))?,
            no_change: self.no_change.ok_or_else(|| BuildError::missing_field("no_change"))?,
            subsumed: self.subsumed.ok_or_else(|| BuildError::missing_field("subsumed"))?,
        })
    }
}
