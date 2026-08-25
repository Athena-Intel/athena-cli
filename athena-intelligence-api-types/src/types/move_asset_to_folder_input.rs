pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MoveAssetToFolderInput {
    /// The IDs of the assets to move
    #[serde(default)]
    pub asset_ids: Vec<String>,
    /// The ID of the folder to move the assets to
    #[serde(default)]
    pub folder_asset_id: String,
}

impl MoveAssetToFolderInput {
    pub fn builder() -> MoveAssetToFolderInputBuilder {
        <MoveAssetToFolderInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MoveAssetToFolderInputBuilder {
    asset_ids: Option<Vec<String>>,
    folder_asset_id: Option<String>,
}

impl MoveAssetToFolderInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn folder_asset_id(mut self, value: impl Into<String>) -> Self {
        self.folder_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MoveAssetToFolderInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](MoveAssetToFolderInputBuilder::asset_ids)
    /// - [`folder_asset_id`](MoveAssetToFolderInputBuilder::folder_asset_id)
    pub fn build(self) -> Result<MoveAssetToFolderInput, BuildError> {
        Ok(MoveAssetToFolderInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
            folder_asset_id: self.folder_asset_id.ok_or_else(|| BuildError::missing_field("folder_asset_id"))?,
        })
    }
}

