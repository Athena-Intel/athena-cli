pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for renaming an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenameAssetResponseOut {
    /// Athena asset type
    #[serde(default)]
    pub asset_type: String,
    /// Unique identifier of the renamed asset
    #[serde(default)]
    pub id: String,
    /// Updated asset title
    #[serde(default)]
    pub title: String,
    /// Timestamp when the asset was updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl RenameAssetResponseOut {
    pub fn builder() -> RenameAssetResponseOutBuilder {
        <RenameAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenameAssetResponseOutBuilder {
    asset_type: Option<String>,
    id: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RenameAssetResponseOutBuilder {
    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RenameAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](RenameAssetResponseOutBuilder::asset_type)
    /// - [`id`](RenameAssetResponseOutBuilder::id)
    /// - [`title`](RenameAssetResponseOutBuilder::title)
    /// - [`updated_at`](RenameAssetResponseOutBuilder::updated_at)
    pub fn build(self) -> Result<RenameAssetResponseOut, BuildError> {
        Ok(RenameAssetResponseOut {
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
