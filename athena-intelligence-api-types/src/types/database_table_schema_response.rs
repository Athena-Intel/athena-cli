pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for table schema (columns and types).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseTableSchemaResponse {
    /// List of columns with their types and constraints
    #[serde(default)]
    pub columns: Vec<DatabaseColumnInfo>,
    /// Schema containing the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Name of the table
    #[serde(default)]
    pub table_name: String,
}

impl DatabaseTableSchemaResponse {
    pub fn builder() -> DatabaseTableSchemaResponseBuilder {
        <DatabaseTableSchemaResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseTableSchemaResponseBuilder {
    columns: Option<Vec<DatabaseColumnInfo>>,
    schema: Option<String>,
    table_name: Option<String>,
}

impl DatabaseTableSchemaResponseBuilder {
    pub fn columns(mut self, value: Vec<DatabaseColumnInfo>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    pub fn table_name(mut self, value: impl Into<String>) -> Self {
        self.table_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabaseTableSchemaResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`columns`](DatabaseTableSchemaResponseBuilder::columns)
    /// - [`table_name`](DatabaseTableSchemaResponseBuilder::table_name)
    pub fn build(self) -> Result<DatabaseTableSchemaResponse, BuildError> {
        Ok(DatabaseTableSchemaResponse {
            columns: self.columns.ok_or_else(|| BuildError::missing_field("columns"))?,
            schema: self.schema,
            table_name: self.table_name.ok_or_else(|| BuildError::missing_field("table_name"))?,
        })
    }
}
