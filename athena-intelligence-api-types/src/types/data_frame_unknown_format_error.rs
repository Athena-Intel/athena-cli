pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataFrameUnknownFormatError {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub media_type: String,
    #[serde(default)]
    pub message: String,
}

impl DataFrameUnknownFormatError {
    pub fn builder() -> DataFrameUnknownFormatErrorBuilder {
        <DataFrameUnknownFormatErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataFrameUnknownFormatErrorBuilder {
    asset_id: Option<String>,
    media_type: Option<String>,
    message: Option<String>,
}

impl DataFrameUnknownFormatErrorBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DataFrameUnknownFormatError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DataFrameUnknownFormatErrorBuilder::asset_id)
    /// - [`media_type`](DataFrameUnknownFormatErrorBuilder::media_type)
    /// - [`message`](DataFrameUnknownFormatErrorBuilder::message)
    pub fn build(self) -> Result<DataFrameUnknownFormatError, BuildError> {
        Ok(DataFrameUnknownFormatError {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
