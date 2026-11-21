pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportConnectorItemRequestIn {
    /// The folder asset the new asset is filed in. It must be a folder of the source's workspace that the caller may edit.
    #[serde(default)]
    pub folder_asset_id: String,
    /// The file's id, as GET /connectors/{catalog_asset_id}/items listed it.
    #[serde(default)]
    pub item_id: String,
}

impl ImportConnectorItemRequestIn {
    pub fn builder() -> ImportConnectorItemRequestInBuilder {
        <ImportConnectorItemRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportConnectorItemRequestInBuilder {
    folder_asset_id: Option<String>,
    item_id: Option<String>,
}

impl ImportConnectorItemRequestInBuilder {
    pub fn folder_asset_id(mut self, value: impl Into<String>) -> Self {
        self.folder_asset_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportConnectorItemRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`folder_asset_id`](ImportConnectorItemRequestInBuilder::folder_asset_id)
    /// - [`item_id`](ImportConnectorItemRequestInBuilder::item_id)
    pub fn build(self) -> Result<ImportConnectorItemRequestIn, BuildError> {
        Ok(ImportConnectorItemRequestIn {
            folder_asset_id: self.folder_asset_id.ok_or_else(|| BuildError::missing_field("folder_asset_id"))?,
            item_id: self.item_id.ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}

