pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A message to be sent to the agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    pub content: InputMessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl InputMessage {
    pub fn builder() -> InputMessageBuilder {
        <InputMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InputMessageBuilder {
    additional_kwargs: Option<HashMap<String, serde_json::Value>>,
    content: Option<InputMessageContent>,
    id: Option<String>,
    name: Option<String>,
    role: Option<String>,
    r#type: Option<String>,
}

impl InputMessageBuilder {
    pub fn additional_kwargs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_kwargs = Some(value);
        self
    }

    pub fn content(mut self, value: InputMessageContent) -> Self {
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

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InputMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](InputMessageBuilder::content)
    pub fn build(self) -> Result<InputMessage, BuildError> {
        Ok(InputMessage {
            additional_kwargs: self.additional_kwargs,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            id: self.id,
            name: self.name,
            role: self.role,
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
