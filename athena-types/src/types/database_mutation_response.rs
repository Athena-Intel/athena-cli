pub use crate::prelude::*;
use super::*;

/// Response for insert/update/delete operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatabaseMutationResponse {
    /// Affected rows (returned when return_representation=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl DatabaseMutationResponse {
    pub fn builder() -> DatabaseMutationResponseBuilder {
        <DatabaseMutationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseMutationResponseBuilder {
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl DatabaseMutationResponseBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseMutationResponse`].
    pub fn build(self) -> Result<DatabaseMutationResponse, BuildError> {
        Ok(DatabaseMutationResponse {
            data: self.data,
        })
    }
}
