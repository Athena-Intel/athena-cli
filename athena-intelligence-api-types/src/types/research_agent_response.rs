pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResearchAgentResponse {
    /// Research findings and compiled information
    #[serde(default)]
    pub findings: HashMap<String, serde_json::Value>,
}

impl ResearchAgentResponse {
    pub fn builder() -> ResearchAgentResponseBuilder {
        <ResearchAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResearchAgentResponseBuilder {
    findings: Option<HashMap<String, serde_json::Value>>,
}

impl ResearchAgentResponseBuilder {
    pub fn findings(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.findings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResearchAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`findings`](ResearchAgentResponseBuilder::findings)
    pub fn build(self) -> Result<ResearchAgentResponse, BuildError> {
        Ok(ResearchAgentResponse {
            findings: self.findings.ok_or_else(|| BuildError::missing_field("findings"))?,
        })
    }
}
