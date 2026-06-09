pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetNotFoundError {
    #[serde(default)]
    pub message: String,
}

impl AssetNotFoundError {
    pub fn builder() -> AssetNotFoundErrorBuilder {
        <AssetNotFoundErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetNotFoundErrorBuilder {
    message: Option<String>,
}

impl AssetNotFoundErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetNotFoundError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AssetNotFoundErrorBuilder::message)
    pub fn build(self) -> Result<AssetNotFoundError, BuildError> {
        Ok(AssetNotFoundError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
