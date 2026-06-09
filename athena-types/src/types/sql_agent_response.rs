pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SqlAgentResponse {
    /// Additional metadata about the generated query
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The asset ID of the generated SQL query object
    #[serde(default)]
    pub query_asset_id: String,
}

impl SqlAgentResponse {
    pub fn builder() -> SqlAgentResponseBuilder {
        <SqlAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SqlAgentResponseBuilder {
    metadata: Option<HashMap<String, serde_json::Value>>,
    query_asset_id: Option<String>,
}

impl SqlAgentResponseBuilder {
    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn query_asset_id(mut self, value: impl Into<String>) -> Self {
        self.query_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SqlAgentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metadata`](SqlAgentResponseBuilder::metadata)
    /// - [`query_asset_id`](SqlAgentResponseBuilder::query_asset_id)
    pub fn build(self) -> Result<SqlAgentResponse, BuildError> {
        Ok(SqlAgentResponse {
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            query_asset_id: self.query_asset_id.ok_or_else(|| BuildError::missing_field("query_asset_id"))?,
        })
    }
}
