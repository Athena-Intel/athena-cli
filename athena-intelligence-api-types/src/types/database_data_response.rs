pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for read (SELECT) operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatabaseDataResponse {
    /// Array of row objects
    #[serde(default)]
    pub data: Vec<HashMap<String, serde_json::Value>>,
}

impl DatabaseDataResponse {
    pub fn builder() -> DatabaseDataResponseBuilder {
        <DatabaseDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseDataResponseBuilder {
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl DatabaseDataResponseBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DatabaseDataResponseBuilder::data)
    pub fn build(self) -> Result<DatabaseDataResponse, BuildError> {
        Ok(DatabaseDataResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
