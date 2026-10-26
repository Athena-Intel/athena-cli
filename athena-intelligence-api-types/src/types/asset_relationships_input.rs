pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetRelationshipsInput {
    /// The asset to read the relationships of.
    #[serde(default)]
    pub asset_id: String,
    /// Only related assets of these types (e.g. ['spreadsheet', 'session']); the counts by type still cover every type. Omit for all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_types: Option<Vec<String>>,
    /// Read every source: links, folders, drives and collections, plus subscriptions, agent configurations, automations, lineage recorded in documents and shares. False reads only what Explore shows members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_sources: Option<bool>,
}

impl AssetRelationshipsInput {
    pub fn builder() -> AssetRelationshipsInputBuilder {
        <AssetRelationshipsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetRelationshipsInputBuilder {
    asset_id: Option<String>,
    asset_types: Option<Vec<String>>,
    include_all_sources: Option<bool>,
}

impl AssetRelationshipsInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_types(mut self, value: Vec<String>) -> Self {
        self.asset_types = Some(value);
        self
    }

    pub fn include_all_sources(mut self, value: bool) -> Self {
        self.include_all_sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetRelationshipsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetRelationshipsInputBuilder::asset_id)
    pub fn build(self) -> Result<AssetRelationshipsInput, BuildError> {
        Ok(AssetRelationshipsInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_types: self.asset_types,
            include_all_sources: self.include_all_sources,
        })
    }
}

