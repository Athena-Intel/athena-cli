pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for duplicating an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAssetResponseOut {
    /// Type of the duplicated asset
    #[serde(default)]
    pub asset_type: String,
    /// ID of the newly duplicated asset
    #[serde(default)]
    pub new_asset_id: String,
    /// ID of the duplicated asset's parent folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// ID of the source asset
    #[serde(default)]
    pub source_asset_id: String,
    /// Title of the duplicated asset
    #[serde(default)]
    pub title: String,
    /// ID of the workspace that owns the duplicated asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl DuplicateAssetResponseOut {
    pub fn builder() -> DuplicateAssetResponseOutBuilder {
        <DuplicateAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAssetResponseOutBuilder {
    asset_type: Option<String>,
    new_asset_id: Option<String>,
    parent_folder_id: Option<String>,
    source_asset_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl DuplicateAssetResponseOutBuilder {
    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn new_asset_id(mut self, value: impl Into<String>) -> Self {
        self.new_asset_id = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn source_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_asset_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](DuplicateAssetResponseOutBuilder::asset_type)
    /// - [`new_asset_id`](DuplicateAssetResponseOutBuilder::new_asset_id)
    /// - [`source_asset_id`](DuplicateAssetResponseOutBuilder::source_asset_id)
    /// - [`title`](DuplicateAssetResponseOutBuilder::title)
    pub fn build(self) -> Result<DuplicateAssetResponseOut, BuildError> {
        Ok(DuplicateAssetResponseOut {
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            new_asset_id: self.new_asset_id.ok_or_else(|| BuildError::missing_field("new_asset_id"))?,
            parent_folder_id: self.parent_folder_id,
            source_asset_id: self.source_asset_id.ok_or_else(|| BuildError::missing_field("source_asset_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
