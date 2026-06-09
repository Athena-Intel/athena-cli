pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileFetchError {
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl FileFetchError {
    pub fn builder() -> FileFetchErrorBuilder {
        <FileFetchErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileFetchErrorBuilder {
    message: Option<String>,
    status_code: Option<i64>,
}

impl FileFetchErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileFetchError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](FileFetchErrorBuilder::message)
    pub fn build(self) -> Result<FileFetchError, BuildError> {
        Ok(FileFetchError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            status_code: self.status_code,
        })
    }
}
