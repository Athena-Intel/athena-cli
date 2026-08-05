pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for sharing an asset with specific users.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShareAssetResponseOut {
    /// ID of the shared asset
    #[serde(default)]
    pub asset_id: String,
    /// Results for each recipient that was shared with
    #[serde(default)]
    pub recipients_result: Vec<ShareRecipientResultOut>,
}

impl ShareAssetResponseOut {
    pub fn builder() -> ShareAssetResponseOutBuilder {
        <ShareAssetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareAssetResponseOutBuilder {
    asset_id: Option<String>,
    recipients_result: Option<Vec<ShareRecipientResultOut>>,
}

impl ShareAssetResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn recipients_result(mut self, value: Vec<ShareRecipientResultOut>) -> Self {
        self.recipients_result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShareAssetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ShareAssetResponseOutBuilder::asset_id)
    /// - [`recipients_result`](ShareAssetResponseOutBuilder::recipients_result)
    pub fn build(self) -> Result<ShareAssetResponseOut, BuildError> {
        Ok(ShareAssetResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            recipients_result: self.recipients_result.ok_or_else(|| BuildError::missing_field("recipients_result"))?,
        })
    }
}
