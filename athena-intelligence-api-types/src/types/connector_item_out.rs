pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One item of a listing. Names and ids only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConnectorItemOut {
    /// The catalog to name on the next call for this item. To open a source, list its catalog_asset_id with no folder_id. To open a library or a folder, list its catalog_asset_id with folder_id set to its id. To import a file, post its id to its catalog_asset_id.
    #[serde(default)]
    pub catalog_asset_id: String,
    /// The item's id in its source. For a source, its catalog asset id.
    #[serde(default)]
    pub id: String,
    /// Whether POST /connectors/{catalog_asset_id}/items/import brings this item in as an asset.
    #[serde(default)]
    pub importable: bool,
    /// A file's media type, when the source names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// When the item last changed, as the source says (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// The item's name.
    #[serde(default)]
    pub name: String,
    /// A file's size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// For a source, its kind (onedrive, sharepoint_site); else null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<String>,
    /// source: a source of a connected account (a OneDrive, a SharePoint site). library: a document library. folder: a folder. file: a file.
    pub r#type: ConnectorItemOutType,
    /// The item's own address in the outside system, which asks for the person's sign-in there. Never a signed download link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

impl ConnectorItemOut {
    pub fn builder() -> ConnectorItemOutBuilder {
        <ConnectorItemOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorItemOutBuilder {
    catalog_asset_id: Option<String>,
    id: Option<String>,
    importable: Option<bool>,
    mime_type: Option<String>,
    modified_at: Option<String>,
    name: Option<String>,
    size: Option<i64>,
    source_kind: Option<String>,
    r#type: Option<ConnectorItemOutType>,
    web_url: Option<String>,
}

impl ConnectorItemOutBuilder {
    pub fn catalog_asset_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_asset_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn importable(mut self, value: bool) -> Self {
        self.importable = Some(value);
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn source_kind(mut self, value: impl Into<String>) -> Self {
        self.source_kind = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ConnectorItemOutType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn web_url(mut self, value: impl Into<String>) -> Self {
        self.web_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorItemOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_asset_id`](ConnectorItemOutBuilder::catalog_asset_id)
    /// - [`id`](ConnectorItemOutBuilder::id)
    /// - [`importable`](ConnectorItemOutBuilder::importable)
    /// - [`name`](ConnectorItemOutBuilder::name)
    /// - [`r#type`](ConnectorItemOutBuilder::r#type)
    pub fn build(self) -> Result<ConnectorItemOut, BuildError> {
        Ok(ConnectorItemOut {
            catalog_asset_id: self.catalog_asset_id.ok_or_else(|| BuildError::missing_field("catalog_asset_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            importable: self.importable.ok_or_else(|| BuildError::missing_field("importable"))?,
            mime_type: self.mime_type,
            modified_at: self.modified_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            size: self.size,
            source_kind: self.source_kind,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            web_url: self.web_url,
        })
    }
}
