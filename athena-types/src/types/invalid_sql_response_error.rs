pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvalidSqlResponseError {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub response_keys: Vec<String>,
}

impl InvalidSqlResponseError {
    pub fn builder() -> InvalidSqlResponseErrorBuilder {
        <InvalidSqlResponseErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvalidSqlResponseErrorBuilder {
    message: Option<String>,
    response_keys: Option<Vec<String>>,
}

impl InvalidSqlResponseErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn response_keys(mut self, value: Vec<String>) -> Self {
        self.response_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvalidSqlResponseError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](InvalidSqlResponseErrorBuilder::message)
    /// - [`response_keys`](InvalidSqlResponseErrorBuilder::response_keys)
    pub fn build(self) -> Result<InvalidSqlResponseError, BuildError> {
        Ok(InvalidSqlResponseError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            response_keys: self.response_keys.ok_or_else(|| BuildError::missing_field("response_keys"))?,
        })
    }
}
