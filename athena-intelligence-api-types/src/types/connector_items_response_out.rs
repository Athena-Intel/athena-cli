pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A page of what a connected account or a source holds.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorItemsResponseOut {
    /// The catalog that was listed.
    #[serde(default)]
    pub catalog_asset_id: String,
    #[serde(default)]
    pub items: Vec<ConnectorItemOut>,
    /// What was listed: microsoft_365 (a connected account: its items are sources), sharepoint_site or onedrive.
    #[serde(default)]
    pub kind: String,
    /// Pass as page_token, with the same catalog_asset_id and folder_id, for the next page. Opaque. Null on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ConnectorItemsResponseOut {
    pub fn builder() -> ConnectorItemsResponseOutBuilder {
        <ConnectorItemsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorItemsResponseOutBuilder {
    catalog_asset_id: Option<String>,
    items: Option<Vec<ConnectorItemOut>>,
    kind: Option<String>,
    next_page_token: Option<String>,
}

impl ConnectorItemsResponseOutBuilder {
    pub fn catalog_asset_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_asset_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<ConnectorItemOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorItemsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_asset_id`](ConnectorItemsResponseOutBuilder::catalog_asset_id)
    /// - [`items`](ConnectorItemsResponseOutBuilder::items)
    /// - [`kind`](ConnectorItemsResponseOutBuilder::kind)
    pub fn build(self) -> Result<ConnectorItemsResponseOut, BuildError> {
        Ok(ConnectorItemsResponseOut {
            catalog_asset_id: self.catalog_asset_id.ok_or_else(|| BuildError::missing_field("catalog_asset_id"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            next_page_token: self.next_page_token,
        })
    }
}
