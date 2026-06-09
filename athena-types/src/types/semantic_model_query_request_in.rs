pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelQueryRequestIn {
    /// Optional dimension identifiers for grouping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    /// Optional filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Maximum rows to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of measure identifiers, e.g. ["orders.total_revenue"]
    #[serde(default)]
    pub measures: Vec<String>,
    /// Optional time dimension configs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_dimensions: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SemanticModelQueryRequestIn {
    pub fn builder() -> SemanticModelQueryRequestInBuilder {
        <SemanticModelQueryRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelQueryRequestInBuilder {
    dimensions: Option<Vec<String>>,
    filters: Option<Vec<HashMap<String, serde_json::Value>>>,
    limit: Option<i64>,
    measures: Option<Vec<String>>,
    time_dimensions: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SemanticModelQueryRequestInBuilder {
    pub fn dimensions(mut self, value: Vec<String>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn measures(mut self, value: Vec<String>) -> Self {
        self.measures = Some(value);
        self
    }

    pub fn time_dimensions(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.time_dimensions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelQueryRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`measures`](SemanticModelQueryRequestInBuilder::measures)
    pub fn build(self) -> Result<SemanticModelQueryRequestIn, BuildError> {
        Ok(SemanticModelQueryRequestIn {
            dimensions: self.dimensions,
            filters: self.filters,
            limit: self.limit,
            measures: self.measures.ok_or_else(|| BuildError::missing_field("measures"))?,
            time_dimensions: self.time_dimensions,
        })
    }
}

