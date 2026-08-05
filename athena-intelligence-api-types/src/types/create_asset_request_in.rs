pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAssetRequestIn {
    /// Type of asset to create. Supported types: 'spreadsheet' (or 'sheet'), 'document' (or 'doc'), 'folder', 'database' (or 'db'), 'computer'
    pub asset_type: CreatableAssetType,
    /// ID of the parent folder to create the asset in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title for the new asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ID of the workspace to create the asset in. If not provided, the asset is created in the user's current workspace. The user must be a member of the specified workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl CreateAssetRequestIn {
    pub fn builder() -> CreateAssetRequestInBuilder {
        <CreateAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAssetRequestInBuilder {
    asset_type: Option<CreatableAssetType>,
    parent_folder_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl CreateAssetRequestInBuilder {
    pub fn asset_type(mut self, value: CreatableAssetType) -> Self {
        self.asset_type = Some(value);
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

    /// Consumes the builder and constructs a [`CreateAssetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](CreateAssetRequestInBuilder::asset_type)
    pub fn build(self) -> Result<CreateAssetRequestIn, BuildError> {
        Ok(CreateAssetRequestIn {
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title,
            workspace_id: self.workspace_id,
        })
    }
}

