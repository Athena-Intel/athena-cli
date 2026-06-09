pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomAgentResponse {
    /// The agent's response. Format depends on the specific agent implementation.
    #[serde(default)]
    pub result: HashMap<String, serde_json::Value>,
}

impl CustomAgentResponse {
    pub fn builder() -> CustomAgentResponseBuilder {
        <CustomAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomAgentResponseBuilder {
    result: Option<HashMap<String, serde_json::Value>>,
}

impl CustomAgentResponseBuilder {
    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](CustomAgentResponseBuilder::result)
    pub fn build(self) -> Result<CustomAgentResponse, BuildError> {
        Ok(CustomAgentResponse {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
