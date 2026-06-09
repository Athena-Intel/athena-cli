pub use crate::prelude::*;
use super::*;

/// Query parameters for _execute_snippet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecuteSnippetQueryRequest {
    #[serde(default)]
    pub snippet_asset_id: String,
}

impl ExecuteSnippetQueryRequest {
    pub fn builder() -> ExecuteSnippetQueryRequestBuilder {
        <ExecuteSnippetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecuteSnippetQueryRequestBuilder {
    snippet_asset_id: Option<String>,
}

impl ExecuteSnippetQueryRequestBuilder {
    pub fn snippet_asset_id(mut self, value: impl Into<String>) -> Self {
        self.snippet_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecuteSnippetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`snippet_asset_id`](ExecuteSnippetQueryRequestBuilder::snippet_asset_id)
    pub fn build(self) -> Result<ExecuteSnippetQueryRequest, BuildError> {
        Ok(ExecuteSnippetQueryRequest {
            snippet_asset_id: self.snippet_asset_id.ok_or_else(|| BuildError::missing_field("snippet_asset_id"))?,
        })
    }
}

