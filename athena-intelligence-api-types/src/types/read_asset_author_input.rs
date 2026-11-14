pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadAssetAuthorInput {
    /// Up to 10 of the parameterized asset ids read_asset takes: an asset id, optionally with read_asset's anchor for one place (asset_…?anchor=sheet_range&sheet_id=1&range=B2:F40, ?anchor=sheet_cell&sheet_id=1&cell=B6, ?anchor=pm_node&node_id=<data-id>, ?anchor=pm_node_range&start_node_id=…&end_node_id=…, ?anchor=slide&slide_index=3, ?anchor=shape&shape_ids=<id>). A long answer pages as read_asset pages: re-read with offset=<next_offset> appended; the answer's `next` is that request.
    #[serde(default)]
    pub asset_ids: Vec<String>,
    /// Most places (paragraphs, cells, shapes) per asset in one answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ReadAssetAuthorInput {
    pub fn builder() -> ReadAssetAuthorInputBuilder {
        <ReadAssetAuthorInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadAssetAuthorInputBuilder {
    asset_ids: Option<Vec<String>>,
    limit: Option<i64>,
}

impl ReadAssetAuthorInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadAssetAuthorInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](ReadAssetAuthorInputBuilder::asset_ids)
    pub fn build(self) -> Result<ReadAssetAuthorInput, BuildError> {
        Ok(ReadAssetAuthorInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
            limit: self.limit,
        })
    }
}

