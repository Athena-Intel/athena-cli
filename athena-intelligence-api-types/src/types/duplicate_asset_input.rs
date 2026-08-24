pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateAssetInput {
    /// ID of the asset to duplicate
    #[serde(default)]
    pub asset_id: String,
    /// Optional custom title for the duplicated asset. If not provided, defaults to 'COPY - {original_title}'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
}

impl DuplicateAssetInput {
    pub fn builder() -> DuplicateAssetInputBuilder {
        <DuplicateAssetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAssetInputBuilder {
    asset_id: Option<String>,
    new_title: Option<String>,
}

impl DuplicateAssetInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn new_title(mut self, value: impl Into<String>) -> Self {
        self.new_title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAssetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DuplicateAssetInputBuilder::asset_id)
    pub fn build(self) -> Result<DuplicateAssetInput, BuildError> {
        Ok(DuplicateAssetInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            new_title: self.new_title,
        })
    }
}

