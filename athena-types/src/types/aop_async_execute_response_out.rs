pub use crate::prelude::*;
use super::*;

/// Response model for async AOP execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AopAsyncExecuteResponseOut {
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
    /// Final prompt used for execution including user inputs
    #[serde(default)]
    pub final_prompt: String,
    /// Status message about the async execution
    #[serde(default)]
    pub message: String,
    /// Status of the execution (always 'started' for async)
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

impl AopAsyncExecuteResponseOut {
    pub fn builder() -> AopAsyncExecuteResponseOutBuilder {
        <AopAsyncExecuteResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AopAsyncExecuteResponseOutBuilder {
    aop_asset_id: Option<String>,
    aop_config: Option<HashMap<String, serde_json::Value>>,
    aop_title: Option<String>,
    base_prompt: Option<String>,
    final_prompt: Option<String>,
    message: Option<String>,
    status: Option<String>,
    sync_server: Option<String>,
    thread_id: Option<String>,
    trigger_type: Option<String>,
}

impl AopAsyncExecuteResponseOutBuilder {
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

    pub fn final_prompt(mut self, value: impl Into<String>) -> Self {
        self.final_prompt = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
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

    /// Consumes the builder and constructs a [`AopAsyncExecuteResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aop_asset_id`](AopAsyncExecuteResponseOutBuilder::aop_asset_id)
    /// - [`aop_config`](AopAsyncExecuteResponseOutBuilder::aop_config)
    /// - [`aop_title`](AopAsyncExecuteResponseOutBuilder::aop_title)
    /// - [`base_prompt`](AopAsyncExecuteResponseOutBuilder::base_prompt)
    /// - [`final_prompt`](AopAsyncExecuteResponseOutBuilder::final_prompt)
    /// - [`message`](AopAsyncExecuteResponseOutBuilder::message)
    /// - [`status`](AopAsyncExecuteResponseOutBuilder::status)
    /// - [`sync_server`](AopAsyncExecuteResponseOutBuilder::sync_server)
    /// - [`thread_id`](AopAsyncExecuteResponseOutBuilder::thread_id)
    /// - [`trigger_type`](AopAsyncExecuteResponseOutBuilder::trigger_type)
    pub fn build(self) -> Result<AopAsyncExecuteResponseOut, BuildError> {
        Ok(AopAsyncExecuteResponseOut {
            aop_asset_id: self.aop_asset_id.ok_or_else(|| BuildError::missing_field("aop_asset_id"))?,
            aop_config: self.aop_config.ok_or_else(|| BuildError::missing_field("aop_config"))?,
            aop_title: self.aop_title.ok_or_else(|| BuildError::missing_field("aop_title"))?,
            base_prompt: self.base_prompt.ok_or_else(|| BuildError::missing_field("base_prompt"))?,
            final_prompt: self.final_prompt.ok_or_else(|| BuildError::missing_field("final_prompt"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            sync_server: self.sync_server.ok_or_else(|| BuildError::missing_field("sync_server"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            trigger_type: self.trigger_type.ok_or_else(|| BuildError::missing_field("trigger_type"))?,
        })
    }
}
