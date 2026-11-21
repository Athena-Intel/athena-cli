pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One member of a deployment the caller may open.
/// 
/// ``registry_id`` is in the answer only when there is one to give: both
/// routes write a member without it with no such key, never ``null``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BundleDescriptorMemberOut {
    /// This place's asset for the member.
    #[serde(default)]
    pub asset_id: String,
    /// An object type's registry id (src_...), the id the records APIs take, only where the caller can already read it: a caller the ontology platform admits (platform staff, an enrolled workspace), for a type whose registration is active. Absent otherwise, and for every other member type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The member asset's type, e.g. collab_agent.
    #[serde(default)]
    pub r#type: String,
}

impl BundleDescriptorMemberOut {
    pub fn builder() -> BundleDescriptorMemberOutBuilder {
        <BundleDescriptorMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleDescriptorMemberOutBuilder {
    asset_id: Option<String>,
    registry_id: Option<String>,
    r#type: Option<String>,
}

impl BundleDescriptorMemberOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn registry_id(mut self, value: impl Into<String>) -> Self {
        self.registry_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleDescriptorMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](BundleDescriptorMemberOutBuilder::asset_id)
    /// - [`r#type`](BundleDescriptorMemberOutBuilder::r#type)
    pub fn build(self) -> Result<BundleDescriptorMemberOut, BuildError> {
        Ok(BundleDescriptorMemberOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            registry_id: self.registry_id,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
