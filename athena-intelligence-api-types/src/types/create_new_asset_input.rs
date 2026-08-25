pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateNewAssetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Title of the asset to be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateNewAssetInput {
    pub fn builder() -> CreateNewAssetInputBuilder {
        <CreateNewAssetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNewAssetInputBuilder {
    parent_folder_id: Option<String>,
    title: Option<String>,
}

impl CreateNewAssetInputBuilder {
    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateNewAssetInput`].
    pub fn build(self) -> Result<CreateNewAssetInput, BuildError> {
        Ok(CreateNewAssetInput {
            parent_folder_id: self.parent_folder_id,
            title: self.title,
        })
    }
}

