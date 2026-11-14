pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryVersionsInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// Include automatic saves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_autosaves: Option<bool>,
    /// Most versions to return (1 to 200; any other number is brought into that range).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl HistoryVersionsInput {
    pub fn builder() -> HistoryVersionsInputBuilder {
        <HistoryVersionsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryVersionsInputBuilder {
    asset_id: Option<String>,
    include_autosaves: Option<bool>,
    limit: Option<i64>,
}

impl HistoryVersionsInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn include_autosaves(mut self, value: bool) -> Self {
        self.include_autosaves = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoryVersionsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryVersionsInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryVersionsInput, BuildError> {
        Ok(HistoryVersionsInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            include_autosaves: self.include_autosaves,
            limit: self.limit,
        })
    }
}

