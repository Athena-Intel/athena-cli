pub use crate::prelude::*;
use super::*;

/// Conversation asset information associated with a thread.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationAssetInfo {
    /// Agent configuration used in conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Complete athena metadata for the conversation asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_metadata: Option<HashMap<String, serde_json::Value>>,
    /// ID of the conversation asset
    #[serde(default)]
    pub conversation_asset_id: String,
    /// ISO timestamp when conversation was created
    #[serde(default)]
    pub created_at: String,
    /// User ID who created the conversation
    #[serde(default)]
    pub created_by: String,
    /// Error details if the conversation execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, serde_json::Value>>,
    /// Last active channel for the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_channel: Option<String>,
    /// Last message in the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<ConversationMessage>,
    /// List of linked AOP assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_aops: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// List of linked project assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_projects: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Complete list of messages in the conversation from checkpoints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ConversationMessage>>,
    /// Model used in conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Number of messages in the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_messages: Option<i64>,
    /// Channel where conversation was started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_channel: Option<String>,
    /// Current state of the conversation (e.g., 'running', 'completed')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Title of the conversation
    #[serde(default)]
    pub title: String,
    /// ISO timestamp when conversation was last updated
    #[serde(default)]
    pub updated_at: String,
    /// Workspace ID where conversation exists
    #[serde(default)]
    pub workspace_id: String,
}

impl ConversationAssetInfo {
    pub fn builder() -> ConversationAssetInfoBuilder {
        <ConversationAssetInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAssetInfoBuilder {
    agent: Option<String>,
    athena_metadata: Option<HashMap<String, serde_json::Value>>,
    conversation_asset_id: Option<String>,
    created_at: Option<String>,
    created_by: Option<String>,
    error: Option<HashMap<String, serde_json::Value>>,
    last_channel: Option<String>,
    last_message: Option<ConversationMessage>,
    linked_aops: Option<Vec<HashMap<String, serde_json::Value>>>,
    linked_projects: Option<Vec<HashMap<String, serde_json::Value>>>,
    messages: Option<Vec<ConversationMessage>>,
    model: Option<String>,
    num_messages: Option<i64>,
    start_channel: Option<String>,
    state: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
    workspace_id: Option<String>,
}

impl ConversationAssetInfoBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn athena_metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.athena_metadata = Some(value);
        self
    }

    pub fn conversation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_asset_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn error(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn last_channel(mut self, value: impl Into<String>) -> Self {
        self.last_channel = Some(value.into());
        self
    }

    pub fn last_message(mut self, value: ConversationMessage) -> Self {
        self.last_message = Some(value);
        self
    }

    pub fn linked_aops(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.linked_aops = Some(value);
        self
    }

    pub fn linked_projects(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.linked_projects = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ConversationMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn num_messages(mut self, value: i64) -> Self {
        self.num_messages = Some(value);
        self
    }

    pub fn start_channel(mut self, value: impl Into<String>) -> Self {
        self.start_channel = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
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

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationAssetInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_asset_id`](ConversationAssetInfoBuilder::conversation_asset_id)
    /// - [`created_at`](ConversationAssetInfoBuilder::created_at)
    /// - [`created_by`](ConversationAssetInfoBuilder::created_by)
    /// - [`title`](ConversationAssetInfoBuilder::title)
    /// - [`updated_at`](ConversationAssetInfoBuilder::updated_at)
    /// - [`workspace_id`](ConversationAssetInfoBuilder::workspace_id)
    pub fn build(self) -> Result<ConversationAssetInfo, BuildError> {
        Ok(ConversationAssetInfo {
            agent: self.agent,
            athena_metadata: self.athena_metadata,
            conversation_asset_id: self.conversation_asset_id.ok_or_else(|| BuildError::missing_field("conversation_asset_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            error: self.error,
            last_channel: self.last_channel,
            last_message: self.last_message,
            linked_aops: self.linked_aops,
            linked_projects: self.linked_projects,
            messages: self.messages,
            model: self.model,
            num_messages: self.num_messages,
            start_channel: self.start_channel,
            state: self.state,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
