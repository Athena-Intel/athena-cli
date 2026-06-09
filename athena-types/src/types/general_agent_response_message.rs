pub use crate::prelude::*;
use super::*;

/// A response message from the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneralAgentResponseMessage {
    /// Additional keyword arguments for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    /// The content of the message, can be string or list of content parts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<GeneralAgentResponseMessageContent>,
    /// Unique identifier for the message or LangChain class identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<GeneralAgentResponseMessageId>,
    /// Structured kwargs field containing known message data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwargs: Option<GeneralAgentResponseMessageKwargs>,
    /// LangChain class identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langchain_id: Option<Vec<String>>,
    /// LangChain version marker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc: Option<i64>,
    /// The actual message identifier (when id contains LangChain class path).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Name associated with the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Metadata about the response generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Role of the message sender (e.g., 'ai', 'human', 'system', 'tool'). Automatically populated from kwargs.type for convenience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// ID of the tool call this message responds to (for tool messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls made by the message (for AI messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// The type of the message (e.g., 'human', 'ai', 'system', 'tool').
    #[serde(default)]
    pub r#type: String,
    /// Usage metadata including token counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl GeneralAgentResponseMessage {
    pub fn builder() -> GeneralAgentResponseMessageBuilder {
        <GeneralAgentResponseMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralAgentResponseMessageBuilder {
    additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    content: Option<GeneralAgentResponseMessageContent>,
    id: Option<GeneralAgentResponseMessageId>,
    kwargs: Option<GeneralAgentResponseMessageKwargs>,
    langchain_id: Option<Vec<String>>,
    lc: Option<i64>,
    message_id: Option<String>,
    name: Option<String>,
    response_metadata: Option<HashMap<String, serde_json::Value>>,
    role: Option<String>,
    tool_call_id: Option<String>,
    tool_calls: Option<Vec<HashMap<String, serde_json::Value>>>,
    r#type: Option<String>,
    usage_metadata: Option<HashMap<String, serde_json::Value>>,
}

impl GeneralAgentResponseMessageBuilder {
    pub fn additional_kwargs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_kwargs = Some(value);
        self
    }

    pub fn content(mut self, value: GeneralAgentResponseMessageContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn id(mut self, value: GeneralAgentResponseMessageId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn kwargs(mut self, value: GeneralAgentResponseMessageKwargs) -> Self {
        self.kwargs = Some(value);
        self
    }

    pub fn langchain_id(mut self, value: Vec<String>) -> Self {
        self.langchain_id = Some(value);
        self
    }

    pub fn lc(mut self, value: i64) -> Self {
        self.lc = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn response_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_metadata = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn usage_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.usage_metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralAgentResponseMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](GeneralAgentResponseMessageBuilder::r#type)
    pub fn build(self) -> Result<GeneralAgentResponseMessage, BuildError> {
        Ok(GeneralAgentResponseMessage {
            additional_kwargs: self.additional_kwargs,
            content: self.content,
            id: self.id,
            kwargs: self.kwargs,
            langchain_id: self.langchain_id,
            lc: self.lc,
            message_id: self.message_id,
            name: self.name,
            response_metadata: self.response_metadata,
            role: self.role,
            tool_call_id: self.tool_call_id,
            tool_calls: self.tool_calls,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            usage_metadata: self.usage_metadata,
            extra: Default::default(),
        })
    }
}
