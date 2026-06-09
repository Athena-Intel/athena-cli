pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MediaFormatError {
    #[serde(default)]
    pub media_type: String,
    #[serde(default)]
    pub message: String,
}

impl MediaFormatError {
    pub fn builder() -> MediaFormatErrorBuilder {
        <MediaFormatErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaFormatErrorBuilder {
    media_type: Option<String>,
    message: Option<String>,
}

impl MediaFormatErrorBuilder {
    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaFormatError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_type`](MediaFormatErrorBuilder::media_type)
    /// - [`message`](MediaFormatErrorBuilder::message)
    pub fn build(self) -> Result<MediaFormatError, BuildError> {
        Ok(MediaFormatError {
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
