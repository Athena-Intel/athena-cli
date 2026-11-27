pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnarchiveAssetsInput {
    /// Archived asset IDs to restore. Each needs edit access in the current workspace; a folder comes back with what its archive took down.
    #[serde(default)]
    pub asset_ids: Vec<String>,
    /// True (the default): every asset that passes its checks is processed and the rest are reported back with the reason. False: all or nothing; if any asset is refused or fails, nothing is changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
}

impl UnarchiveAssetsInput {
    pub fn builder() -> UnarchiveAssetsInputBuilder {
        <UnarchiveAssetsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnarchiveAssetsInputBuilder {
    asset_ids: Option<Vec<String>>,
    partial: Option<bool>,
}

impl UnarchiveAssetsInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn partial(mut self, value: bool) -> Self {
        self.partial = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnarchiveAssetsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](UnarchiveAssetsInputBuilder::asset_ids)
    pub fn build(self) -> Result<UnarchiveAssetsInput, BuildError> {
        Ok(UnarchiveAssetsInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
            partial: self.partial,
        })
    }
}

