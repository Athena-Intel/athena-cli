pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SqlUnauthorizedError {
    #[serde(default)]
    pub message: String,
}

impl SqlUnauthorizedError {
    pub fn builder() -> SqlUnauthorizedErrorBuilder {
        <SqlUnauthorizedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SqlUnauthorizedErrorBuilder {
    message: Option<String>,
}

impl SqlUnauthorizedErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SqlUnauthorizedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SqlUnauthorizedErrorBuilder::message)
    pub fn build(self) -> Result<SqlUnauthorizedError, BuildError> {
        Ok(SqlUnauthorizedError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
