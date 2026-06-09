pub use crate::prelude::*;
use super::*;

/// Model representing a single conversation message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationMessage {
    /// Additional message metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    /// Message content as text or structured content blocks
    pub content: ConversationMessageContent,
    /// Unique identifier for the message
    #[serde(default)]
    pub id: String,
    /// Tool name for tool messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Role of the message sender (user, assistant, system)
    #[serde(default)]
    pub role: String,
}

impl ConversationMessage {
    pub fn builder() -> ConversationMessageBuilder {
        <ConversationMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationMessageBuilder {
    additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    content: Option<ConversationMessageContent>,
    id: Option<String>,
    name: Option<String>,
    role: Option<String>,
}

impl ConversationMessageBuilder {
    pub fn additional_kwargs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_kwargs = Some(value);
        self
    }

    pub fn content(mut self, value: ConversationMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ConversationMessageBuilder::content)
    /// - [`id`](ConversationMessageBuilder::id)
    /// - [`role`](ConversationMessageBuilder::role)
    pub fn build(self) -> Result<ConversationMessage, BuildError> {
        Ok(ConversationMessage {
            additional_kwargs: self.additional_kwargs,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
