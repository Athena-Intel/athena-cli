pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryOpenInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// Which state to read (default current): current, published, a saved version id or name, a time (2026-10-28T14:00Z), or another asset id (asset_…, optionally @<version>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

impl HistoryOpenInput {
    pub fn builder() -> HistoryOpenInputBuilder {
        <HistoryOpenInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryOpenInputBuilder {
    asset_id: Option<String>,
    at: Option<String>,
}

impl HistoryOpenInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryOpenInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryOpenInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryOpenInput, BuildError> {
        Ok(HistoryOpenInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            at: self.at,
        })
    }
}

