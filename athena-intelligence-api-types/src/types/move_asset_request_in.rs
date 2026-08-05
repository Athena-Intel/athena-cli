pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MoveAssetRequestIn {
    /// ID of the destination folder. Pass null or omit the field to move the asset to the workspace root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Optional drive asset ID. When provided without parent_folder_id, the asset moves to the drive root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_drive_id: Option<String>,
}

impl MoveAssetRequestIn {
    pub fn builder() -> MoveAssetRequestInBuilder {
        <MoveAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MoveAssetRequestInBuilder {
    parent_folder_id: Option<String>,
    target_drive_id: Option<String>,
}

impl MoveAssetRequestInBuilder {
    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn target_drive_id(mut self, value: impl Into<String>) -> Self {
        self.target_drive_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MoveAssetRequestIn`].
    pub fn build(self) -> Result<MoveAssetRequestIn, BuildError> {
        Ok(MoveAssetRequestIn {
            parent_folder_id: self.parent_folder_id,
            target_drive_id: self.target_drive_id,
        })
    }
}

