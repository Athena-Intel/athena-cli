pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a bundle's track serves, as an app reads it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BundleDescriptorOut {
    /// The bundle's permanent key.
    #[serde(default)]
    pub bundle_key: String,
    /// The shape's version: 1.
    #[serde(default)]
    pub descriptor: i64,
    /// By member key: each member the caller may open. A member the caller can't open is absent.
    #[serde(default)]
    pub members: HashMap<String, BundleDescriptorMemberOut>,
    /// By slot key: every slot the served version declares.
    #[serde(default)]
    pub slots: HashMap<String, BundleDescriptorSlotOut>,
    /// The track the answer read, e.g. production.
    #[serde(default)]
    pub track: String,
    /// The version the track serves, major.minor.
    #[serde(default)]
    pub version: String,
    /// The bundle's workspace, one the caller is a member of.
    #[serde(default)]
    pub workspace_id: String,
}

impl BundleDescriptorOut {
    pub fn builder() -> BundleDescriptorOutBuilder {
        <BundleDescriptorOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleDescriptorOutBuilder {
    bundle_key: Option<String>,
    descriptor: Option<i64>,
    members: Option<HashMap<String, BundleDescriptorMemberOut>>,
    slots: Option<HashMap<String, BundleDescriptorSlotOut>>,
    track: Option<String>,
    version: Option<String>,
    workspace_id: Option<String>,
}

impl BundleDescriptorOutBuilder {
    pub fn bundle_key(mut self, value: impl Into<String>) -> Self {
        self.bundle_key = Some(value.into());
        self
    }

    pub fn descriptor(mut self, value: i64) -> Self {
        self.descriptor = Some(value);
        self
    }

    pub fn members(mut self, value: HashMap<String, BundleDescriptorMemberOut>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn slots(mut self, value: HashMap<String, BundleDescriptorSlotOut>) -> Self {
        self.slots = Some(value);
        self
    }

    pub fn track(mut self, value: impl Into<String>) -> Self {
        self.track = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleDescriptorOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bundle_key`](BundleDescriptorOutBuilder::bundle_key)
    /// - [`descriptor`](BundleDescriptorOutBuilder::descriptor)
    /// - [`members`](BundleDescriptorOutBuilder::members)
    /// - [`slots`](BundleDescriptorOutBuilder::slots)
    /// - [`track`](BundleDescriptorOutBuilder::track)
    /// - [`version`](BundleDescriptorOutBuilder::version)
    /// - [`workspace_id`](BundleDescriptorOutBuilder::workspace_id)
    pub fn build(self) -> Result<BundleDescriptorOut, BuildError> {
        Ok(BundleDescriptorOut {
            bundle_key: self.bundle_key.ok_or_else(|| BuildError::missing_field("bundle_key"))?,
            descriptor: self.descriptor.ok_or_else(|| BuildError::missing_field("descriptor"))?,
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
            slots: self.slots.ok_or_else(|| BuildError::missing_field("slots"))?,
            track: self.track.ok_or_else(|| BuildError::missing_field("track"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
