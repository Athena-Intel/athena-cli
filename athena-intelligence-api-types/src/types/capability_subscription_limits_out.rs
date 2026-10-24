pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A subscription's bounds.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionLimitsOut {
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub max_runs: i64,
}

impl CapabilitySubscriptionLimitsOut {
    pub fn builder() -> CapabilitySubscriptionLimitsOutBuilder {
        <CapabilitySubscriptionLimitsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionLimitsOutBuilder {
    expires_at: Option<String>,
    max_runs: Option<i64>,
}

impl CapabilitySubscriptionLimitsOutBuilder {
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn max_runs(mut self, value: i64) -> Self {
        self.max_runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionLimitsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](CapabilitySubscriptionLimitsOutBuilder::expires_at)
    /// - [`max_runs`](CapabilitySubscriptionLimitsOutBuilder::max_runs)
    pub fn build(self) -> Result<CapabilitySubscriptionLimitsOut, BuildError> {
        Ok(CapabilitySubscriptionLimitsOut {
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            max_runs: self.max_runs.ok_or_else(|| BuildError::missing_field("max_runs"))?,
        })
    }
}
