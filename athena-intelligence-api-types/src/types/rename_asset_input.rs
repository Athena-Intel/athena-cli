pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RenameAssetInput {
    /// List of rename operations, each containing an asset_id and new_title
    #[serde(default)]
    pub renames: Vec<HashMap<String, serde_json::Value>>,
}

impl RenameAssetInput {
    pub fn builder() -> RenameAssetInputBuilder {
        <RenameAssetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenameAssetInputBuilder {
    renames: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl RenameAssetInputBuilder {
    pub fn renames(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.renames = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RenameAssetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`renames`](RenameAssetInputBuilder::renames)
    pub fn build(self) -> Result<RenameAssetInput, BuildError> {
        Ok(RenameAssetInput {
            renames: self.renames.ok_or_else(|| BuildError::missing_field("renames"))?,
        })
    }
}

