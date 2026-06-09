pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DriveAgentResponse {
    /// Results of the drive operation
    #[serde(default)]
    pub result: HashMap<String, serde_json::Value>,
}

impl DriveAgentResponse {
    pub fn builder() -> DriveAgentResponseBuilder {
        <DriveAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DriveAgentResponseBuilder {
    result: Option<HashMap<String, serde_json::Value>>,
}

impl DriveAgentResponseBuilder {
    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DriveAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](DriveAgentResponseBuilder::result)
    pub fn build(self) -> Result<DriveAgentResponse, BuildError> {
        Ok(DriveAgentResponse {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
