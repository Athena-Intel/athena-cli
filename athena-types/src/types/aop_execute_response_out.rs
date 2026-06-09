pub use crate::prelude::*;
use super::*;

/// Response model for AOP execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopExecuteResponseOut {
    /// ID of the AOP asset that was executed
    #[serde(default)]
    pub aop_asset_id: String,
    /// Full configuration of the AOP asset
    #[serde(default)]
    pub aop_config: HashMap<String, serde_json::Value>,
    /// Title of the AOP asset
    #[serde(default)]
    pub aop_title: String,
    /// Base prompt of the AOP before user inputs were added
    #[serde(default)]
    pub base_prompt: String,
    /// The conversation result from the AOP execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationResult>,
    /// Error message if execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Final prompt used for execution including user inputs
    #[serde(default)]
    pub final_prompt: String,
    /// Status of the execution (e.g., 'submitted')
    #[serde(default)]
    pub status: String,
    /// Sync server URL used for execution
    #[serde(default)]
    pub sync_server: String,
    /// Unique thread ID for tracking the execution
    #[serde(default)]
    pub thread_id: String,
    /// Type of trigger that initiated the execution
    #[serde(default)]
    pub trigger_type: String,
}

impl AopExecuteResponseOut {
    pub fn builder() -> AopExecuteResponseOutBuilder {
        <AopExecuteResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopExecuteResponseOutBuilder {
    aop_asset_id: Option<String>,
    aop_config: Option<HashMap<String, serde_json::Value>>,
    aop_title: Option<String>,
    base_prompt: Option<String>,
    conversation: Option<ConversationResult>,
    error: Option<String>,
    final_prompt: Option<String>,
    status: Option<String>,
    sync_server: Option<String>,
    thread_id: Option<String>,
    trigger_type: Option<String>,
}

impl AopExecuteResponseOutBuilder {
    pub fn aop_asset_id(mut self, value: impl Into<String>) -> Self {
        self.aop_asset_id = Some(value.into());
        self
    }

    pub fn aop_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.aop_config = Some(value);
        self
    }

    pub fn aop_title(mut self, value: impl Into<String>) -> Self {
        self.aop_title = Some(value.into());
        self
    }

    pub fn base_prompt(mut self, value: impl Into<String>) -> Self {
        self.base_prompt = Some(value.into());
        self
    }

    pub fn conversation(mut self, value: ConversationResult) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn final_prompt(mut self, value: impl Into<String>) -> Self {
        self.final_prompt = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn sync_server(mut self, value: impl Into<String>) -> Self {
        self.sync_server = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn trigger_type(mut self, value: impl Into<String>) -> Self {
        self.trigger_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AopExecuteResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aop_asset_id`](AopExecuteResponseOutBuilder::aop_asset_id)
    /// - [`aop_config`](AopExecuteResponseOutBuilder::aop_config)
    /// - [`aop_title`](AopExecuteResponseOutBuilder::aop_title)
    /// - [`base_prompt`](AopExecuteResponseOutBuilder::base_prompt)
    /// - [`final_prompt`](AopExecuteResponseOutBuilder::final_prompt)
    /// - [`status`](AopExecuteResponseOutBuilder::status)
    /// - [`sync_server`](AopExecuteResponseOutBuilder::sync_server)
    /// - [`thread_id`](AopExecuteResponseOutBuilder::thread_id)
    /// - [`trigger_type`](AopExecuteResponseOutBuilder::trigger_type)
    pub fn build(self) -> Result<AopExecuteResponseOut, BuildError> {
        Ok(AopExecuteResponseOut {
            aop_asset_id: self.aop_asset_id.ok_or_else(|| BuildError::missing_field("aop_asset_id"))?,
            aop_config: self.aop_config.ok_or_else(|| BuildError::missing_field("aop_config"))?,
            aop_title: self.aop_title.ok_or_else(|| BuildError::missing_field("aop_title"))?,
            base_prompt: self.base_prompt.ok_or_else(|| BuildError::missing_field("base_prompt"))?,
            conversation: self.conversation,
            error: self.error,
            final_prompt: self.final_prompt.ok_or_else(|| BuildError::missing_field("final_prompt"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            sync_server: self.sync_server.ok_or_else(|| BuildError::missing_field("sync_server"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            trigger_type: self.trigger_type.ok_or_else(|| BuildError::missing_field("trigger_type"))?,
        })
    }
}
