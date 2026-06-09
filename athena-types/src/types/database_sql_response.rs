pub use crate::prelude::*;
use super::*;

/// Response for direct SQL execution.
/// 
/// For SELECT queries, ``columns`` and ``rows`` are populated.
/// For non-SELECT statements (DDL/DML), ``statuses`` is populated.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatabaseSqlResponse {
    /// Column names (populated for SELECT queries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Row data as objects (populated for SELECT queries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Execution statuses (populated for non-SELECT statements)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
}

impl DatabaseSqlResponse {
    pub fn builder() -> DatabaseSqlResponseBuilder {
        <DatabaseSqlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseSqlResponseBuilder {
    columns: Option<Vec<String>>,
    rows: Option<Vec<HashMap<String, serde_json::Value>>>,
    statuses: Option<Vec<String>>,
}

impl DatabaseSqlResponseBuilder {
    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<String>) -> Self {
        self.statuses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseSqlResponse`].
    pub fn build(self) -> Result<DatabaseSqlResponse, BuildError> {
        Ok(DatabaseSqlResponse {
            columns: self.columns,
            rows: self.rows,
            statuses: self.statuses,
        })
    }
}
