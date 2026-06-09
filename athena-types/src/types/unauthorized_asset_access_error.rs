pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnauthorizedAssetAccessError {
    #[serde(default)]
    pub message: String,
}

impl UnauthorizedAssetAccessError {
    pub fn builder() -> UnauthorizedAssetAccessErrorBuilder {
        <UnauthorizedAssetAccessErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnauthorizedAssetAccessErrorBuilder {
    message: Option<String>,
}

impl UnauthorizedAssetAccessErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnauthorizedAssetAccessError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](UnauthorizedAssetAccessErrorBuilder::message)
    pub fn build(self) -> Result<UnauthorizedAssetAccessError, BuildError> {
        Ok(UnauthorizedAssetAccessError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
