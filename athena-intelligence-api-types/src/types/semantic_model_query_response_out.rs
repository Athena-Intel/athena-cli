pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response from a semantic model query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelQueryResponseOut {
    /// 'snapshot' when 'data_as_of' comes from the source tables' Iceberg snapshots (lakehouse-backed models); 'query_time' when the data is as current as the moment the warehouse answered ('queried_at').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of_kind: Option<String>,
    /// Query result rows
    #[serde(default)]
    pub data: Vec<HashMap<String, serde_json::Value>>,
    /// ISO-8601 UTC time the data is current as of. For a lakehouse-backed model: the oldest last-commit time across the model's source tables. Null for models over other engines (Snowflake, Databricks, BigQuery, Postgres), which expose no snapshot, and for rows the engine served from a cache computed before a source table's latest commit — use 'queried_at' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_as_of: Option<String>,
    /// Server-side execution time in milliseconds (semantic layer + engine, excluding network) — surface it to show how fast the engine answered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<i64>,
    /// ISO-8601 UTC time the engine computed the result (an earlier time when it was served from the engine's cache).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queried_at: Option<String>,
    /// Number of rows returned
    #[serde(default)]
    pub row_count: i64,
    /// Hash of the deployed semantic model definition that computed the rows (the same value the model's citations record); null for a model that was never deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_hash: Option<String>,
    /// Deployment number of the semantic model definition that computed the rows; null for a model that was never deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    /// For a lakehouse-backed model, the Iceberg snapshot of each source table the returned rows reflect; empty for models over other engines and when the rows were computed before a source table's latest commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshots: Option<Vec<SemanticModelSourceSnapshotOut>>,
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
    as_of_kind: Option<String>,
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    data_as_of: Option<String>,
    elapsed_ms: Option<i64>,
    queried_at: Option<String>,
    row_count: Option<i64>,
    schema_hash: Option<String>,
    schema_version: Option<i64>,
    source_snapshots: Option<Vec<SemanticModelSourceSnapshotOut>>,
    truncated: Option<bool>,
}

impl SemanticModelQueryResponseOutBuilder {
    pub fn as_of_kind(mut self, value: impl Into<String>) -> Self {
        self.as_of_kind = Some(value.into());
        self
    }

    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn data_as_of(mut self, value: impl Into<String>) -> Self {
        self.data_as_of = Some(value.into());
        self
    }

    pub fn elapsed_ms(mut self, value: i64) -> Self {
        self.elapsed_ms = Some(value);
        self
    }

    pub fn queried_at(mut self, value: impl Into<String>) -> Self {
        self.queried_at = Some(value.into());
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn schema_hash(mut self, value: impl Into<String>) -> Self {
        self.schema_hash = Some(value.into());
        self
    }

    pub fn schema_version(mut self, value: i64) -> Self {
        self.schema_version = Some(value);
        self
    }

    pub fn source_snapshots(mut self, value: Vec<SemanticModelSourceSnapshotOut>) -> Self {
        self.source_snapshots = Some(value);
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
            as_of_kind: self.as_of_kind,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            data_as_of: self.data_as_of,
            elapsed_ms: self.elapsed_ms,
            queried_at: self.queried_at,
            row_count: self.row_count.ok_or_else(|| BuildError::missing_field("row_count"))?,
            schema_hash: self.schema_hash,
            schema_version: self.schema_version,
            source_snapshots: self.source_snapshots,
            truncated: self.truncated,
        })
    }
}
