pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAssetRequestIn {
    /// Optional destination folder for the duplicated asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// ID of the asset to duplicate
    #[serde(default)]
    pub source_asset_id: String,
    /// Workspace to create the duplicate in. If omitted, the source asset's workspace is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl DuplicateAssetRequestIn {
    pub fn builder() -> DuplicateAssetRequestInBuilder {
        <DuplicateAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAssetRequestInBuilder {
    parent_folder_id: Option<String>,
    source_asset_id: Option<String>,
    workspace_id: Option<String>,
}

impl DuplicateAssetRequestInBuilder {
    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn source_asset_id(mut self, value: impl Into<String>) -> Self {
        self.source_asset_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAssetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_asset_id`](DuplicateAssetRequestInBuilder::source_asset_id)
    pub fn build(self) -> Result<DuplicateAssetRequestIn, BuildError> {
        Ok(DuplicateAssetRequestIn {
            parent_folder_id: self.parent_folder_id,
            source_asset_id: self.source_asset_id.ok_or_else(|| BuildError::missing_field("source_asset_id"))?,
            workspace_id: self.workspace_id,
        })
    }
}

