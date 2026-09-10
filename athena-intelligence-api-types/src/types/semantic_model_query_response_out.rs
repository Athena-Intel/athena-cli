pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response from a semantic model query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelQueryResponseOut {
    /// Query result rows
    #[serde(default)]
    pub data: Vec<HashMap<String, serde_json::Value>>,
    /// Server-side execution time in milliseconds (semantic layer + engine, excluding network) — surface it to show how fast the engine answered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<i64>,
    /// Number of rows returned
    #[serde(default)]
    pub row_count: i64,
    /// True when the query produced more rows than 'limit' and the result was cut; raise 'limit' or add filters to see the rest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

impl SemanticModelQueryResponseOut {
    pub fn builder() -> SemanticModelQueryResponseOutBuilder {
        <SemanticModelQueryResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelQueryResponseOutBuilder {
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    elapsed_ms: Option<i64>,
    row_count: Option<i64>,
    truncated: Option<bool>,
}

impl SemanticModelQueryResponseOutBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn elapsed_ms(mut self, value: i64) -> Self {
        self.elapsed_ms = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelQueryResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SemanticModelQueryResponseOutBuilder::data)
    /// - [`row_count`](SemanticModelQueryResponseOutBuilder::row_count)
    pub fn build(self) -> Result<SemanticModelQueryResponseOut, BuildError> {
        Ok(SemanticModelQueryResponseOut {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            elapsed_ms: self.elapsed_ms,
            row_count: self.row_count.ok_or_else(|| BuildError::missing_field("row_count"))?,
            truncated: self.truncated,
        })
    }
}
