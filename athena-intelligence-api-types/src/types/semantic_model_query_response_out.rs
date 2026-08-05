pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response from a semantic model query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelQueryResponseOut {
    /// Query result rows
    #[serde(default)]
    pub data: Vec<HashMap<String, serde_json::Value>>,
    /// Number of rows returned
    #[serde(default)]
    pub row_count: i64,
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
    row_count: Option<i64>,
}

impl SemanticModelQueryResponseOutBuilder {
    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelQueryResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SemanticModelQueryResponseOutBuilder::data)
    /// - [`row_count`](SemanticModelQueryResponseOutBuilder::row_count)
    pub fn build(self) -> Result<SemanticModelQueryResponseOut, BuildError> {
        Ok(SemanticModelQueryResponseOut {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            row_count: self.row_count.ok_or_else(|| BuildError::missing_field("row_count"))?,
        })
    }
}
