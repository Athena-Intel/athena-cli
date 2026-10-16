pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What an event subscription listens to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionSubjectOut {
    /// Where in the asset; empty means the whole asset.
    #[serde(default)]
    pub anchors: Vec<CapabilitySubscriptionAnchorOut>,
    #[serde(default)]
    pub asset_id: String,
}

impl CapabilitySubscriptionSubjectOut {
    pub fn builder() -> CapabilitySubscriptionSubjectOutBuilder {
        <CapabilitySubscriptionSubjectOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionSubjectOutBuilder {
    anchors: Option<Vec<CapabilitySubscriptionAnchorOut>>,
    asset_id: Option<String>,
}

impl CapabilitySubscriptionSubjectOutBuilder {
    pub fn anchors(mut self, value: Vec<CapabilitySubscriptionAnchorOut>) -> Self {
        self.anchors = Some(value);
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionSubjectOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`anchors`](CapabilitySubscriptionSubjectOutBuilder::anchors)
    /// - [`asset_id`](CapabilitySubscriptionSubjectOutBuilder::asset_id)
    pub fn build(self) -> Result<CapabilitySubscriptionSubjectOut, BuildError> {
        Ok(CapabilitySubscriptionSubjectOut {
            anchors: self.anchors.ok_or_else(|| BuildError::missing_field("anchors"))?,
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}
