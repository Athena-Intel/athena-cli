pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One instant: at or after.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionDeadlineInput {
    /// The deadline as a delay from now: 4h, 3d.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The deadline (RFC 3339), before the subscription expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// IANA time zone for an at without an offset (UTC by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl SubscriptionDeadlineInput {
    pub fn builder() -> SubscriptionDeadlineInputBuilder {
        <SubscriptionDeadlineInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionDeadlineInputBuilder {
    after: Option<String>,
    at: Option<String>,
    timezone: Option<String>,
}

impl SubscriptionDeadlineInputBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionDeadlineInput`].
    pub fn build(self) -> Result<SubscriptionDeadlineInput, BuildError> {
        Ok(SubscriptionDeadlineInput {
            after: self.after,
            at: self.at,
            timezone: self.timezone,
        })
    }
}
