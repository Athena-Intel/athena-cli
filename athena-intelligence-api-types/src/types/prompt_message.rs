pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A message to use for the structured data extractor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PromptMessage {
    #[serde(default)]
    pub content: String,
    pub r#type: PromptMessageType,
}

impl PromptMessage {
    pub fn builder() -> PromptMessageBuilder {
        <PromptMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptMessageBuilder {
    content: Option<String>,
    r#type: Option<PromptMessageType>,
}

impl PromptMessageBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PromptMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](PromptMessageBuilder::content)
    /// - [`r#type`](PromptMessageBuilder::r#type)
    pub fn build(self) -> Result<PromptMessage, BuildError> {
        Ok(PromptMessage {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
