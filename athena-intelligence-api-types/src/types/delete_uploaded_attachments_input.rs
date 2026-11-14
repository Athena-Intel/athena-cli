pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteUploadedAttachmentsInput {
    /// Asset IDs of files or images the current user uploaded (for example a chat screenshot) to archive. Only the uploader's own assets are accepted.
    #[serde(default)]
    pub asset_ids: Vec<String>,
}

impl DeleteUploadedAttachmentsInput {
    pub fn builder() -> DeleteUploadedAttachmentsInputBuilder {
        <DeleteUploadedAttachmentsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteUploadedAttachmentsInputBuilder {
    asset_ids: Option<Vec<String>>,
}

impl DeleteUploadedAttachmentsInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteUploadedAttachmentsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](DeleteUploadedAttachmentsInputBuilder::asset_ids)
    pub fn build(self) -> Result<DeleteUploadedAttachmentsInput, BuildError> {
        Ok(DeleteUploadedAttachmentsInput {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
        })
    }
}

