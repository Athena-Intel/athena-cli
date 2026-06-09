pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ParentFolderError {
    #[serde(default)]
    pub message: String,
}

impl ParentFolderError {
    pub fn builder() -> ParentFolderErrorBuilder {
        <ParentFolderErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ParentFolderErrorBuilder {
    message: Option<String>,
}

impl ParentFolderErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ParentFolderError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ParentFolderErrorBuilder::message)
    pub fn build(self) -> Result<ParentFolderError, BuildError> {
        Ok(ParentFolderError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
