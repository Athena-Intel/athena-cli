pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The asset an import made.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportConnectorItemResponseOut {
    /// The new asset.
    #[serde(default)]
    pub asset_id: String,
    /// Its type, to open it with.
    #[serde(default)]
    pub asset_type: String,
    /// The source it was imported from.
    #[serde(default)]
    pub catalog_asset_id: String,
    /// The folder it was filed in.
    #[serde(default)]
    pub folder_asset_id: String,
    /// Its title: the file's name.
    #[serde(default)]
    pub title: String,
}

impl ImportConnectorItemResponseOut {
    pub fn builder() -> ImportConnectorItemResponseOutBuilder {
        <ImportConnectorItemResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportConnectorItemResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    catalog_asset_id: Option<String>,
    folder_asset_id: Option<String>,
    title: Option<String>,
}

impl ImportConnectorItemResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn catalog_asset_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_asset_id = Some(value.into());
        self
    }

    pub fn folder_asset_id(mut self, value: impl Into<String>) -> Self {
        self.folder_asset_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportConnectorItemResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ImportConnectorItemResponseOutBuilder::asset_id)
    /// - [`asset_type`](ImportConnectorItemResponseOutBuilder::asset_type)
    /// - [`catalog_asset_id`](ImportConnectorItemResponseOutBuilder::catalog_asset_id)
    /// - [`folder_asset_id`](ImportConnectorItemResponseOutBuilder::folder_asset_id)
    /// - [`title`](ImportConnectorItemResponseOutBuilder::title)
    pub fn build(self) -> Result<ImportConnectorItemResponseOut, BuildError> {
        Ok(ImportConnectorItemResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            catalog_asset_id: self.catalog_asset_id.ok_or_else(|| BuildError::missing_field("catalog_asset_id"))?,
            folder_asset_id: self.folder_asset_id.ok_or_else(|| BuildError::missing_field("folder_asset_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
