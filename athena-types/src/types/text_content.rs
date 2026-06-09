pub use crate::prelude::*;
use super::*;

/// A text content item in a multimodal message content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextContent {
    #[serde(default)]
    pub text: String,
}

impl TextContent {
    pub fn builder() -> TextContentBuilder {
        <TextContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextContentBuilder {
    text: Option<String>,
}

impl TextContentBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextContentBuilder::text)
    pub fn build(self) -> Result<TextContent, BuildError> {
        Ok(TextContent {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
