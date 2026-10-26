pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for reading a user's effective access on an asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetAccessOut {
    /// ID of the asset
    #[serde(default)]
    pub asset_id: String,
    /// Whether the user has an active membership in the asset's workspace. A user who is not a member reads as 'none'; the one exception is the workspace's own external viewer, who can still open what is shared with them. A suspended member reads as 'none' with no sources, and this field still reports their membership.
    #[serde(default)]
    pub is_member: bool,
    /// The access the user has, as Athena's permission checks decide it: 'view' can open the asset, 'edit' can also change it and manage its sharing, 'owner' created it and can edit it, 'none' cannot open it. When a check refuses what a listed source grants, the level is lower than that source, and the level is what holds.
    pub level: AssetAccessOutLevel,
    /// Every grant that contributes to the level
    #[serde(default)]
    pub sources: Vec<AssetAccessSourceOut>,
    /// The user whose access was read
    #[serde(default)]
    pub user_id: String,
    /// The workspace the asset belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AssetAccessOut {
    pub fn builder() -> AssetAccessOutBuilder {
        <AssetAccessOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetAccessOutBuilder {
    asset_id: Option<String>,
    is_member: Option<bool>,
    level: Option<AssetAccessOutLevel>,
    sources: Option<Vec<AssetAccessSourceOut>>,
    user_id: Option<String>,
    workspace_id: Option<String>,
}

impl AssetAccessOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn is_member(mut self, value: bool) -> Self {
        self.is_member = Some(value);
        self
    }

    pub fn level(mut self, value: AssetAccessOutLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<AssetAccessSourceOut>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetAccessOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetAccessOutBuilder::asset_id)
    /// - [`is_member`](AssetAccessOutBuilder::is_member)
    /// - [`level`](AssetAccessOutBuilder::level)
    /// - [`sources`](AssetAccessOutBuilder::sources)
    /// - [`user_id`](AssetAccessOutBuilder::user_id)
    pub fn build(self) -> Result<AssetAccessOut, BuildError> {
        Ok(AssetAccessOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            is_member: self.is_member.ok_or_else(|| BuildError::missing_field("is_member"))?,
            level: self.level.ok_or_else(|| BuildError::missing_field("level"))?,
            sources: self.sources.ok_or_else(|| BuildError::missing_field("sources"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            workspace_id: self.workspace_id,
        })
    }
}
