pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LakehouseSyncRunInput {
    /// Lakehouse sync asset id (asset_…). The caller needs EDIT on it; the run executes as the sync definition's own principal.
    #[serde(default)]
    pub asset_id: String,
}

impl LakehouseSyncRunInput {
    pub fn builder() -> LakehouseSyncRunInputBuilder {
        <LakehouseSyncRunInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LakehouseSyncRunInputBuilder {
    asset_id: Option<String>,
}

impl LakehouseSyncRunInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LakehouseSyncRunInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](LakehouseSyncRunInputBuilder::asset_id)
    pub fn build(self) -> Result<LakehouseSyncRunInput, BuildError> {
        Ok(LakehouseSyncRunInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

