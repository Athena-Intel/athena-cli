pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LakehouseNamespaceSqlRequestIn {
    /// Rows to return. Defaults to 1,000; at most 10,000 may be asked for, and the lakehouse engine returns at most 2,000 for one statement. `truncated` in the answer says when more rows matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<i64>,
    /// One SELECT in StarRocks SQL: a plain SELECT, WITH … SELECT, or SELECTs joined by UNION, INTERSECT or EXCEPT. It may read only this namespace's tables, named `<catalog>.<namespace>.<table>`; a table of any other namespace is refused (403). Any other statement is refused (403): a write or schema change, SHOW, DESCRIBE, EXPLAIN, or more than one statement. A blank statement is refused (422).
    #[serde(default)]
    pub sql: String,
}

impl LakehouseNamespaceSqlRequestIn {
    pub fn builder() -> LakehouseNamespaceSqlRequestInBuilder {
        <LakehouseNamespaceSqlRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LakehouseNamespaceSqlRequestInBuilder {
    max_rows: Option<i64>,
    sql: Option<String>,
}

impl LakehouseNamespaceSqlRequestInBuilder {
    pub fn max_rows(mut self, value: i64) -> Self {
        self.max_rows = Some(value);
        self
    }

    pub fn sql(mut self, value: impl Into<String>) -> Self {
        self.sql = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LakehouseNamespaceSqlRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sql`](LakehouseNamespaceSqlRequestInBuilder::sql)
    pub fn build(self) -> Result<LakehouseNamespaceSqlRequestIn, BuildError> {
        Ok(LakehouseNamespaceSqlRequestIn {
            max_rows: self.max_rows,
            sql: self.sql.ok_or_else(|| BuildError::missing_field("sql"))?,
        })
    }
}

