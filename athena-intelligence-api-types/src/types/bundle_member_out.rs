pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A member name, resolved in the caller's place.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BundleMemberOut {
    /// This place's asset for the member.
    #[serde(default)]
    pub asset_id: String,
    /// The bundle's permanent key.
    #[serde(default)]
    pub bundle_key: String,
    /// How the member joins: ships, pin, pin+notify or follow.
    #[serde(default)]
    pub link: String,
    /// The member's permanent key.
    #[serde(default)]
    pub member_key: String,
    /// With a track, the revision it serves of the member (null for a followed member); null without a track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<BundleMemberRevisionOut>,
    /// The track the answer read; null when none was asked for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    /// The member asset's type, e.g. collab_agent.
    #[serde(default)]
    pub r#type: String,
}

impl BundleMemberOut {
    pub fn builder() -> BundleMemberOutBuilder {
        <BundleMemberOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleMemberOutBuilder {
    asset_id: Option<String>,
    bundle_key: Option<String>,
    link: Option<String>,
    member_key: Option<String>,
    revision: Option<BundleMemberRevisionOut>,
    track: Option<String>,
    r#type: Option<String>,
}

impl BundleMemberOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn bundle_key(mut self, value: impl Into<String>) -> Self {
        self.bundle_key = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn member_key(mut self, value: impl Into<String>) -> Self {
        self.member_key = Some(value.into());
        self
    }

    pub fn revision(mut self, value: BundleMemberRevisionOut) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn track(mut self, value: impl Into<String>) -> Self {
        self.track = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleMemberOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](BundleMemberOutBuilder::asset_id)
    /// - [`bundle_key`](BundleMemberOutBuilder::bundle_key)
    /// - [`link`](BundleMemberOutBuilder::link)
    /// - [`member_key`](BundleMemberOutBuilder::member_key)
    /// - [`r#type`](BundleMemberOutBuilder::r#type)
    pub fn build(self) -> Result<BundleMemberOut, BuildError> {
        Ok(BundleMemberOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            bundle_key: self.bundle_key.ok_or_else(|| BuildError::missing_field("bundle_key"))?,
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
            member_key: self.member_key.ok_or_else(|| BuildError::missing_field("member_key"))?,
            revision: self.revision,
            track: self.track,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
