pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model asset information representation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveAssetRequestOut {
    #[serde(default)]
    pub asset_id: String,
    /// Size in bytes of the file received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Folder the asset was filed into
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Lowercase hex SHA-256 of the file received, to check the upload without downloading it again; null when it was not computed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// Workspace the asset was saved into
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl SaveAssetRequestOut {
    pub fn builder() -> SaveAssetRequestOutBuilder {
        <SaveAssetRequestOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveAssetRequestOutBuilder {
    asset_id: Option<String>,
    file_size: Option<i64>,
    parent_folder_id: Option<String>,
    sha256: Option<String>,
    workspace_id: Option<String>,
}

impl SaveAssetRequestOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn file_size(mut self, value: i64) -> Self {
        self.file_size = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SaveAssetRequestOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](SaveAssetRequestOutBuilder::asset_id)
    pub fn build(self) -> Result<SaveAssetRequestOut, BuildError> {
        Ok(SaveAssetRequestOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            file_size: self.file_size,
            parent_folder_id: self.parent_folder_id,
            sha256: self.sha256,
            workspace_id: self.workspace_id,
        })
    }
}
