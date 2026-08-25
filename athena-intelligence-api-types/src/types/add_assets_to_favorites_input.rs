pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddAssetsToFavoritesInput {
    /// List of asset IDs to add to favorites
    #[serde(default)]
    pub asset_ids: Vec<String>,
}

impl AddAssetsToFavoritesInput {
    pub fn builder() -> AddAssetsToFavoritesInputBuilder {
        <AddAssetsToFavoritesInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddAssetsToFavoritesInputBuilder {
    asset_ids: Option<Vec<String>>,
}

impl AddAssetsToFavoritesInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddAssetsToFavoritesInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](AddAssetsToFavoritesInputBuilder::asset_ids)
    pub fn build(self) -> Result<AddAssetsToFavoritesInput, BuildError> {
        Ok(AddAssetsToFavoritesInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
        })
    }
}

