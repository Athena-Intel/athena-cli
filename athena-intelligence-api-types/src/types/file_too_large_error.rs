pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileTooLargeError {
    #[serde(default)]
    pub message: String,
}

impl FileTooLargeError {
    pub fn builder() -> FileTooLargeErrorBuilder {
        <FileTooLargeErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileTooLargeErrorBuilder {
    message: Option<String>,
}

impl FileTooLargeErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileTooLargeError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](FileTooLargeErrorBuilder::message)
    pub fn build(self) -> Result<FileTooLargeError, BuildError> {
        Ok(FileTooLargeError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
