pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_definitions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDefinitionsQueryRequest {
    /// Only return tools in this toolkit (identifier or alias).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit: Option<String>,
    /// Only return tools the caller can currently invoke over HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocable_only: Option<bool>,
}

impl ListDefinitionsQueryRequest {
    pub fn builder() -> ListDefinitionsQueryRequestBuilder {
        <ListDefinitionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDefinitionsQueryRequestBuilder {
    toolkit: Option<String>,
    invocable_only: Option<bool>,
}

impl ListDefinitionsQueryRequestBuilder {
    pub fn toolkit(mut self, value: impl Into<String>) -> Self {
        self.toolkit = Some(value.into());
        self
    }

    pub fn invocable_only(mut self, value: bool) -> Self {
        self.invocable_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDefinitionsQueryRequest`].
    pub fn build(self) -> Result<ListDefinitionsQueryRequest, BuildError> {
        Ok(ListDefinitionsQueryRequest {
            toolkit: self.toolkit,
            invocable_only: self.invocable_only,
        })
    }
}

