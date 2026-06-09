pub use crate::prelude::*;
use super::*;

/// Response model with JSON dataframe representation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DataFrameRequestOut {
    #[serde(default)]
    pub columns: Vec<Option<DataFrameRequestOutColumnsItem>>,
    #[serde(default)]
    pub data: Vec<Vec<Option<DataFrameRequestOutDataItemItem>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Vec<Option<DataFrameRequestOutIndexItem>>>,
}

impl DataFrameRequestOut {
    pub fn builder() -> DataFrameRequestOutBuilder {
        <DataFrameRequestOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataFrameRequestOutBuilder {
    columns: Option<Vec<Option<DataFrameRequestOutColumnsItem>>>,
    data: Option<Vec<Vec<Option<DataFrameRequestOutDataItemItem>>>>,
    index: Option<Vec<Option<DataFrameRequestOutIndexItem>>>,
}

impl DataFrameRequestOutBuilder {
    pub fn columns(mut self, value: Vec<Option<DataFrameRequestOutColumnsItem>>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<Vec<Option<DataFrameRequestOutDataItemItem>>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn index(mut self, value: Vec<Option<DataFrameRequestOutIndexItem>>) -> Self {
        self.index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataFrameRequestOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`columns`](DataFrameRequestOutBuilder::columns)
    /// - [`data`](DataFrameRequestOutBuilder::data)
    pub fn build(self) -> Result<DataFrameRequestOut, BuildError> {
        Ok(DataFrameRequestOut {
            columns: self.columns.ok_or_else(|| BuildError::missing_field("columns"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            index: self.index,
        })
    }
}
