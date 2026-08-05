pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileChunkRequestIn {
    /// Identifiers of the assets
    #[serde(default)]
    pub asset_ids: Vec<String>,
}

impl FileChunkRequestIn {
    pub fn builder() -> FileChunkRequestInBuilder {
        <FileChunkRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileChunkRequestInBuilder {
    asset_ids: Option<Vec<String>>,
}

impl FileChunkRequestInBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileChunkRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](FileChunkRequestInBuilder::asset_ids)
    pub fn build(self) -> Result<FileChunkRequestIn, BuildError> {
        Ok(FileChunkRequestIn {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
        })
    }
}

