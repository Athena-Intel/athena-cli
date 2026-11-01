pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistorySaveVersionInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// Save the asset as it was at this time instead of now (2026-10-28T14:00Z).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// A name a person will recognize ("Sent to counterparty").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl HistorySaveVersionInput {
    pub fn builder() -> HistorySaveVersionInputBuilder {
        <HistorySaveVersionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistorySaveVersionInputBuilder {
    asset_id: Option<String>,
    at: Option<String>,
    name: Option<String>,
}

impl HistorySaveVersionInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistorySaveVersionInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistorySaveVersionInputBuilder::asset_id)
    pub fn build(self) -> Result<HistorySaveVersionInput, BuildError> {
        Ok(HistorySaveVersionInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            at: self.at,
            name: self.name,
        })
    }
}

