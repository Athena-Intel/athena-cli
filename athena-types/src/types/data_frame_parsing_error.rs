pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataFrameParsingError {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub message: String,
}

impl DataFrameParsingError {
    pub fn builder() -> DataFrameParsingErrorBuilder {
        <DataFrameParsingErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataFrameParsingErrorBuilder {
    asset_id: Option<String>,
    message: Option<String>,
}

impl DataFrameParsingErrorBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DataFrameParsingError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DataFrameParsingErrorBuilder::asset_id)
    /// - [`message`](DataFrameParsingErrorBuilder::message)
    pub fn build(self) -> Result<DataFrameParsingError, BuildError> {
        Ok(DataFrameParsingError {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
