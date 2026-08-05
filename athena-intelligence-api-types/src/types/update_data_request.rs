pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDataRequest {
    /// Column values to update
    #[serde(default)]
    pub data: HashMap<String, serde_json::Value>,
    /// If true, return the updated rows in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_representation: Option<bool>,
    /// Set to true to update all rows (required when no filters provided)
    #[serde(skip)]
    pub force: Option<bool>,
}

impl UpdateDataRequest {
    pub fn builder() -> UpdateDataRequestBuilder {
        <UpdateDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDataRequestBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
    return_representation: Option<bool>,
    force: Option<bool>,
}

impl UpdateDataRequestBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn return_representation(mut self, value: bool) -> Self {
        self.return_representation = Some(value);
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDataRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](UpdateDataRequestBuilder::data)
    pub fn build(self) -> Result<UpdateDataRequest, BuildError> {
        Ok(UpdateDataRequest {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            return_representation: self.return_representation,
            force: self.force,
        })
    }
}

