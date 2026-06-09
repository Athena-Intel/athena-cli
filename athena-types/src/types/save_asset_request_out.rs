pub use crate::prelude::*;
use super::*;

/// Response model asset information representation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveAssetRequestOut {
    #[serde(default)]
    pub asset_id: String,
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
}

impl SaveAssetRequestOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SaveAssetRequestOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](SaveAssetRequestOutBuilder::asset_id)
    pub fn build(self) -> Result<SaveAssetRequestOut, BuildError> {
        Ok(SaveAssetRequestOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}
