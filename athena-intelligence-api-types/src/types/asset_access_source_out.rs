pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One grant that contributes to a user's access on an asset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetAccessSourceOut {
    /// The workspace group, when kind is 'group'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The workspace group's name, when kind is 'group'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Where the grant comes from: 'owner' (the user created the asset), 'direct' (the asset is shared with the user), 'group' (it is shared with a workspace group the user belongs to), 'workspace' (it is shared with the whole workspace), 'drive' (a role on the drive the asset is in) or 'other' (access the share list does not hold, for example an environment admin, also on an asset in a drive, or the view of a folder that holds an asset the user can open). A folder's sharing reaches each asset inside it as that asset's own share, so a 'workspace' source on a sub-folder shows that a share of its parent folder reached it.
    pub kind: AssetAccessSourceOutKind,
    /// The level this grant gives
    pub level: AssetAccessSourceOutLevel,
}

impl AssetAccessSourceOut {
    pub fn builder() -> AssetAccessSourceOutBuilder {
        <AssetAccessSourceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetAccessSourceOutBuilder {
    group_id: Option<String>,
    group_name: Option<String>,
    kind: Option<AssetAccessSourceOutKind>,
    level: Option<AssetAccessSourceOutLevel>,
}

impl AssetAccessSourceOutBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn kind(mut self, value: AssetAccessSourceOutKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn level(mut self, value: AssetAccessSourceOutLevel) -> Self {
        self.level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetAccessSourceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](AssetAccessSourceOutBuilder::kind)
    /// - [`level`](AssetAccessSourceOutBuilder::level)
    pub fn build(self) -> Result<AssetAccessSourceOut, BuildError> {
        Ok(AssetAccessSourceOut {
            group_id: self.group_id,
            group_name: self.group_name,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            level: self.level.ok_or_else(|| BuildError::missing_field("level"))?,
        })
    }
}
