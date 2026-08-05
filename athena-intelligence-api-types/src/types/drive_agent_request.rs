pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DriveAgentRequest {
    /// Configuration for the drive agent including folder paths and search parameters
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
    /// The messages to send to the drive agent
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
}

impl DriveAgentRequest {
    pub fn builder() -> DriveAgentRequestBuilder {
        <DriveAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DriveAgentRequestBuilder {
    config: Option<HashMap<String, serde_json::Value>>,
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl DriveAgentRequestBuilder {
    pub fn config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DriveAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](DriveAgentRequestBuilder::config)
    /// - [`messages`](DriveAgentRequestBuilder::messages)
    pub fn build(self) -> Result<DriveAgentRequest, BuildError> {
        Ok(DriveAgentRequest {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
        })
    }
}

