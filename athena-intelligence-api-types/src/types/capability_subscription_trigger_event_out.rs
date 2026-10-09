pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The events an event subscription hears, and on what.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionTriggerEventOut {
    #[serde(default)]
    pub event_types: Vec<String>,
    #[serde(default)]
    pub subject: CapabilitySubscriptionSubjectOut,
}

impl CapabilitySubscriptionTriggerEventOut {
    pub fn builder() -> CapabilitySubscriptionTriggerEventOutBuilder {
        <CapabilitySubscriptionTriggerEventOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionTriggerEventOutBuilder {
    event_types: Option<Vec<String>>,
    subject: Option<CapabilitySubscriptionSubjectOut>,
}

impl CapabilitySubscriptionTriggerEventOutBuilder {
    pub fn event_types(mut self, value: Vec<String>) -> Self {
        self.event_types = Some(value);
        self
    }

    pub fn subject(mut self, value: CapabilitySubscriptionSubjectOut) -> Self {
        self.subject = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionTriggerEventOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_types`](CapabilitySubscriptionTriggerEventOutBuilder::event_types)
    /// - [`subject`](CapabilitySubscriptionTriggerEventOutBuilder::subject)
    pub fn build(self) -> Result<CapabilitySubscriptionTriggerEventOut, BuildError> {
        Ok(CapabilitySubscriptionTriggerEventOut {
            event_types: self.event_types.ok_or_else(|| BuildError::missing_field("event_types"))?,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
        })
    }
}
