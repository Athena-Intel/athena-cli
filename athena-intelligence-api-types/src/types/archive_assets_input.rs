pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchiveAssetsInput {
    /// Asset IDs to archive (soft delete). Each needs edit access in the current workspace; a folder takes what it holds with it. Archived assets can be restored with unarchive_assets.
    #[serde(default)]
    pub asset_ids: Vec<String>,
    /// True (the default): every asset that passes its checks is processed and the rest are reported back with the reason. False: all or nothing; if any asset is refused or fails, nothing is changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
}

impl ArchiveAssetsInput {
    pub fn builder() -> ArchiveAssetsInputBuilder {
        <ArchiveAssetsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchiveAssetsInputBuilder {
    asset_ids: Option<Vec<String>>,
    partial: Option<bool>,
}

impl ArchiveAssetsInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn partial(mut self, value: bool) -> Self {
        self.partial = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArchiveAssetsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](ArchiveAssetsInputBuilder::asset_ids)
    pub fn build(self) -> Result<ArchiveAssetsInput, BuildError> {
        Ok(ArchiveAssetsInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
            partial: self.partial,
        })
    }
}

