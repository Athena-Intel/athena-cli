pub use crate::prelude::*;
use super::*;

/// Error response from semantic model operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SemanticModelErrorResponse {
    #[serde(default)]
    pub message: String,
}

impl SemanticModelErrorResponse {
    pub fn builder() -> SemanticModelErrorResponseBuilder {
        <SemanticModelErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelErrorResponseBuilder {
    message: Option<String>,
}

impl SemanticModelErrorResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SemanticModelErrorResponseBuilder::message)
    pub fn build(self) -> Result<SemanticModelErrorResponse, BuildError> {
        Ok(SemanticModelErrorResponse {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
