pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SqlAssetError {
    #[serde(default)]
    pub message: String,
}

impl SqlAssetError {
    pub fn builder() -> SqlAssetErrorBuilder {
        <SqlAssetErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SqlAssetErrorBuilder {
    message: Option<String>,
}

impl SqlAssetErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SqlAssetError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SqlAssetErrorBuilder::message)
    pub fn build(self) -> Result<SqlAssetError, BuildError> {
        Ok(SqlAssetError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
