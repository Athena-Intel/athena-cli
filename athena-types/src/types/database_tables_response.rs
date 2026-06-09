pub use crate::prelude::*;
use super::*;

/// Response for list tables operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseTablesResponse {
    /// List of tables in the database
    #[serde(default)]
    pub tables: Vec<DatabaseTableInfo>,
}

impl DatabaseTablesResponse {
    pub fn builder() -> DatabaseTablesResponseBuilder {
        <DatabaseTablesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseTablesResponseBuilder {
    tables: Option<Vec<DatabaseTableInfo>>,
}

impl DatabaseTablesResponseBuilder {
    pub fn tables(mut self, value: Vec<DatabaseTableInfo>) -> Self {
        self.tables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseTablesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tables`](DatabaseTablesResponseBuilder::tables)
    pub fn build(self) -> Result<DatabaseTablesResponse, BuildError> {
        Ok(DatabaseTablesResponse {
            tables: self.tables.ok_or_else(|| BuildError::missing_field("tables"))?,
        })
    }
}
