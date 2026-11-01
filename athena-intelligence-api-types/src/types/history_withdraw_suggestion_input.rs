pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryWithdrawSuggestionInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// One suggestion's id from `history suggestions`; omit to withdraw all of yours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_id: Option<String>,
}

impl HistoryWithdrawSuggestionInput {
    pub fn builder() -> HistoryWithdrawSuggestionInputBuilder {
        <HistoryWithdrawSuggestionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryWithdrawSuggestionInputBuilder {
    asset_id: Option<String>,
    suggestion_id: Option<String>,
}

impl HistoryWithdrawSuggestionInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn suggestion_id(mut self, value: impl Into<String>) -> Self {
        self.suggestion_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryWithdrawSuggestionInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryWithdrawSuggestionInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryWithdrawSuggestionInput, BuildError> {
        Ok(HistoryWithdrawSuggestionInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            suggestion_id: self.suggestion_id,
        })
    }
}

