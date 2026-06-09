pub use crate::prelude::*;
use super::*;

/// Response model for asset creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAssetResponseOut {
    /// ID of the newly created asset
    #[serde(default)]
    pub asset_id: String,
    /// Type of the created asset
    #[serde(default)]
    pub asset_type: String,
    /// Timestamp when the asset was created
    #[serde(default)]
    pub created_at: String,
    /// ID of the parent folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title of the created asset
    #[serde(default)]
    pub title: String,
}

impl CreateAssetResponseOut {
    pub fn builder() -> CreateAssetResponseOutBuilder {
        <CreateAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAssetResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    created_at: Option<String>,
    parent_folder_id: Option<String>,
    title: Option<String>,
}

impl CreateAssetResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](CreateAssetResponseOutBuilder::asset_id)
    /// - [`asset_type`](CreateAssetResponseOutBuilder::asset_type)
    /// - [`created_at`](CreateAssetResponseOutBuilder::created_at)
    /// - [`title`](CreateAssetResponseOutBuilder::title)
    pub fn build(self) -> Result<CreateAssetResponseOut, BuildError> {
        Ok(CreateAssetResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            parent_folder_id: self.parent_folder_id,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
