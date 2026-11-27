pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One slot of a deployment: what it asks for and where the track stands.
/// 
/// ``value`` is in the answer only when there is one to give: both routes
/// write an entry without it with no such key, never ``null``
/// (``response_model_exclude_none`` on the public route, the same dump on
/// Marathon's).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BundleDescriptorSlotOut {
    /// What the slot asks for: phone_number, mailbox, database, connection, computer_image, place, lakehouse_namespace, outside_source, text, secret or slack_install.
    #[serde(default)]
    pub kind: String,
    /// filled (the track's fill is in force), not_used (the track doesn't use it) or waiting (nothing in force).
    #[serde(default)]
    pub state: String,
    /// What the track filled the slot with, only for a slot the bundle declares for the app, while its fill is in force, to a caller who can open an application the bundle lists: a text value's words, or the asset id a computer_image, place or lakehouse_namespace fill names, where the caller can open that asset. Absent otherwise, and always for a connection, an outside_source, a secret, a slack_install, a phone_number, a mailbox and a database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl BundleDescriptorSlotOut {
    pub fn builder() -> BundleDescriptorSlotOutBuilder {
        <BundleDescriptorSlotOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleDescriptorSlotOutBuilder {
    kind: Option<String>,
    state: Option<String>,
    value: Option<String>,
}

impl BundleDescriptorSlotOutBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleDescriptorSlotOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](BundleDescriptorSlotOutBuilder::kind)
    /// - [`state`](BundleDescriptorSlotOutBuilder::state)
    pub fn build(self) -> Result<BundleDescriptorSlotOut, BuildError> {
        Ok(BundleDescriptorSlotOut {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            value: self.value,
        })
    }
}
