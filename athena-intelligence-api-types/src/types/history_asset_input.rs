pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An asset whose history to read.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryAssetInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
}

impl HistoryAssetInput {
    pub fn builder() -> HistoryAssetInputBuilder {
        <HistoryAssetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryAssetInputBuilder {
    asset_id: Option<String>,
}

impl HistoryAssetInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryAssetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryAssetInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryAssetInput, BuildError> {
        Ok(HistoryAssetInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}
