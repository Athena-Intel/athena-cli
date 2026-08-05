pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Table metadata for list tables response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseTableInfo {
    /// Name of the table
    #[serde(default)]
    pub name: String,
    /// Approximate row count (may be estimated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i64>,
    /// Schema containing the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

impl DatabaseTableInfo {
    pub fn builder() -> DatabaseTableInfoBuilder {
        <DatabaseTableInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseTableInfoBuilder {
    name: Option<String>,
    row_count: Option<i64>,
    schema: Option<String>,
}

impl DatabaseTableInfoBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabaseTableInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DatabaseTableInfoBuilder::name)
    pub fn build(self) -> Result<DatabaseTableInfo, BuildError> {
        Ok(DatabaseTableInfo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            row_count: self.row_count,
            schema: self.schema,
        })
    }
}
