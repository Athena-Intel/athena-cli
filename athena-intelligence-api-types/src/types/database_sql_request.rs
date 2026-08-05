pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseSqlRequest {
    /// SQL statement to execute
    #[serde(default)]
    pub sql: String,
}

impl DatabaseSqlRequest {
    pub fn builder() -> DatabaseSqlRequestBuilder {
        <DatabaseSqlRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseSqlRequestBuilder {
    sql: Option<String>,
}

impl DatabaseSqlRequestBuilder {
    pub fn sql(mut self, value: impl Into<String>) -> Self {
        self.sql = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabaseSqlRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sql`](DatabaseSqlRequestBuilder::sql)
    pub fn build(self) -> Result<DatabaseSqlRequest, BuildError> {
        Ok(DatabaseSqlRequest {
            sql: self.sql.ok_or_else(|| BuildError::missing_field("sql"))?,
        })
    }
}

