pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for moving an asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MoveAssetResponseOut {
    /// ID of the moved asset
    #[serde(default)]
    pub asset_id: String,
    /// Type of the moved asset
    #[serde(default)]
    pub asset_type: String,
    /// ID of the new parent folder, or null for workspace root
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title of the moved asset
    #[serde(default)]
    pub title: String,
    /// ID of the workspace that owns the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl MoveAssetResponseOut {
    pub fn builder() -> MoveAssetResponseOutBuilder {
        <MoveAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MoveAssetResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl MoveAssetResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`MoveAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](MoveAssetResponseOutBuilder::asset_id)
    /// - [`asset_type`](MoveAssetResponseOutBuilder::asset_type)
    /// - [`title`](MoveAssetResponseOutBuilder::title)
    pub fn build(self) -> Result<MoveAssetResponseOut, BuildError> {
        Ok(MoveAssetResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
