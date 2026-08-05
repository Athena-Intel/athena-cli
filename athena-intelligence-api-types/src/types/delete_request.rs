pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request for delete (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteRequest {
    /// Set to true to delete all rows (required when no filters provided)
    #[serde(skip)]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DeleteDataRequest>,
}

impl DeleteRequest {
    pub fn builder() -> DeleteRequestBuilder {
        <DeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteRequestBuilder {
    force: Option<bool>,
    body: Option<DeleteDataRequest>,
}

impl DeleteRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    pub fn body(mut self, value: DeleteDataRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteRequest`].
    pub fn build(self) -> Result<DeleteRequest, BuildError> {
        Ok(DeleteRequest {
            force: self.force,
            body: self.body,
        })
    }
}

