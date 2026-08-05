pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model representing the conversation result from task/AOP execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationResult {
    /// ID of the conversation asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// ISO timestamp when conversation was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The last message from the assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_assistant_message: Option<ConversationMessage>,
    /// Complete list of messages in the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ConversationMessage>>,
    /// Source of the messages (e.g., 'checkpoints')
    #[serde(default)]
    pub messages_source: String,
    /// Additional conversation metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Total number of messages in the conversation
    #[serde(default)]
    pub num_messages: i64,
    /// Title of the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ISO timestamp when conversation was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ConversationResult {
    pub fn builder() -> ConversationResultBuilder {
        <ConversationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationResultBuilder {
    conversation_id: Option<String>,
    created_at: Option<String>,
    last_assistant_message: Option<ConversationMessage>,
    messages: Option<Vec<ConversationMessage>>,
    messages_source: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    num_messages: Option<i64>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl ConversationResultBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn last_assistant_message(mut self, value: ConversationMessage) -> Self {
        self.last_assistant_message = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ConversationMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn messages_source(mut self, value: impl Into<String>) -> Self {
        self.messages_source = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn num_messages(mut self, value: i64) -> Self {
        self.num_messages = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages_source`](ConversationResultBuilder::messages_source)
    /// - [`num_messages`](ConversationResultBuilder::num_messages)
    pub fn build(self) -> Result<ConversationResult, BuildError> {
        Ok(ConversationResult {
            conversation_id: self.conversation_id,
            created_at: self.created_at,
            last_assistant_message: self.last_assistant_message,
            messages: self.messages,
            messages_source: self.messages_source.ok_or_else(|| BuildError::missing_field("messages_source"))?,
            metadata: self.metadata,
            num_messages: self.num_messages.ok_or_else(|| BuildError::missing_field("num_messages"))?,
            title: self.title,
            updated_at: self.updated_at,
        })
    }
}
