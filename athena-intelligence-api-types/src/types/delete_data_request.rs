pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request body for delete operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDataRequest {
    /// If true, return the deleted rows in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_representation: Option<bool>,
}

impl DeleteDataRequest {
    pub fn builder() -> DeleteDataRequestBuilder {
        <DeleteDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDataRequestBuilder {
    return_representation: Option<bool>,
}

impl DeleteDataRequestBuilder {
    pub fn return_representation(mut self, value: bool) -> Self {
        self.return_representation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteDataRequest`].
    pub fn build(self) -> Result<DeleteDataRequest, BuildError> {
        Ok(DeleteDataRequest {
            return_representation: self.return_representation,
        })
    }
}
