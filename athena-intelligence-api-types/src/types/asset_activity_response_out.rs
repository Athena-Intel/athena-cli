pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A page of an asset's activity log, newest edit first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetActivityResponseOut {
    #[serde(default)]
    pub asset_id: String,
    /// The asset's Athena type.
    #[serde(default)]
    pub asset_type: String,
    #[serde(default)]
    pub items: Vec<ActivityItemOut>,
    /// Pass as to_clock to fetch the next (older) page. Null when the log has been fully read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_to_clock: Option<i64>,
    /// Number of items in this page.
    #[serde(default)]
    pub returned: i64,
}

impl AssetActivityResponseOut {
    pub fn builder() -> AssetActivityResponseOutBuilder {
        <AssetActivityResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetActivityResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    items: Option<Vec<ActivityItemOut>>,
    next_page_to_clock: Option<i64>,
    returned: Option<i64>,
}

impl AssetActivityResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<ActivityItemOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_page_to_clock(mut self, value: i64) -> Self {
        self.next_page_to_clock = Some(value);
        self
    }

    pub fn returned(mut self, value: i64) -> Self {
        self.returned = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetActivityResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetActivityResponseOutBuilder::asset_id)
    /// - [`asset_type`](AssetActivityResponseOutBuilder::asset_type)
    /// - [`items`](AssetActivityResponseOutBuilder::items)
    /// - [`returned`](AssetActivityResponseOutBuilder::returned)
    pub fn build(self) -> Result<AssetActivityResponseOut, BuildError> {
        Ok(AssetActivityResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_page_to_clock: self.next_page_to_clock,
            returned: self.returned.ok_or_else(|| BuildError::missing_field("returned"))?,
        })
    }
}
