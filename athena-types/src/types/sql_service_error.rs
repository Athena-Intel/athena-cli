pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SqlServiceError {
    pub message: String,
}

impl SqlServiceError {
    pub fn builder() -> SqlServiceErrorBuilder {
        <SqlServiceErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SqlServiceErrorBuilder {
    message: Option<String>,
}

impl SqlServiceErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SqlServiceError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SqlServiceErrorBuilder::message)
    pub fn build(self) -> Result<SqlServiceError, BuildError> {
        Ok(SqlServiceError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
