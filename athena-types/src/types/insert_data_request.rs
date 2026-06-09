pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsertDataRequest {
    /// Single row object or array of row objects to insert
    pub data: InsertDataRequestData,
    /// If true, return the inserted rows in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_representation: Option<bool>,
}

impl InsertDataRequest {
    pub fn builder() -> InsertDataRequestBuilder {
        <InsertDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsertDataRequestBuilder {
    data: Option<InsertDataRequestData>,
    return_representation: Option<bool>,
}

impl InsertDataRequestBuilder {
    pub fn data(mut self, value: InsertDataRequestData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn return_representation(mut self, value: bool) -> Self {
        self.return_representation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsertDataRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](InsertDataRequestBuilder::data)
    pub fn build(self) -> Result<InsertDataRequest, BuildError> {
        Ok(InsertDataRequest {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            return_representation: self.return_representation,
        })
    }
}

