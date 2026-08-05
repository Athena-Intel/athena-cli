pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SqlAgentRequest {
    /// Configuration for the SQL agent including database connection details and query parameters
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
    /// The messages to send to the SQL agent
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
}

impl SqlAgentRequest {
    pub fn builder() -> SqlAgentRequestBuilder {
        <SqlAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SqlAgentRequestBuilder {
    config: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SqlAgentRequestBuilder {
    pub fn config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SqlAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](SqlAgentRequestBuilder::config)
    /// - [`messages`](SqlAgentRequestBuilder::messages)
    pub fn build(self) -> Result<SqlAgentRequest, BuildError> {
        Ok(SqlAgentRequest {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}

