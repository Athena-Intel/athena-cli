pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The rows one read-only statement over a lakehouse namespace returned.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LakehouseNamespaceSqlResponseOut {
    /// The lakehouse namespace asset the statement read
    #[serde(default)]
    pub asset_id: String,
    /// The result's columns, in order
    #[serde(default)]
    pub columns: Vec<LakehouseNamespaceSqlColumnOut>,
    /// How many rows `rows` holds
    #[serde(default)]
    pub row_count: i64,
    /// The result's rows, each a list of values in `columns` order
    #[serde(default)]
    pub rows: Vec<Vec<serde_json::Value>>,
    /// True when more rows matched than were returned: the request's `max_rows`, or the engine's cap of 2,000 rows per statement
    #[serde(default)]
    pub truncated: bool,
}

impl LakehouseNamespaceSqlResponseOut {
    pub fn builder() -> LakehouseNamespaceSqlResponseOutBuilder {
        <LakehouseNamespaceSqlResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LakehouseNamespaceSqlResponseOutBuilder {
    asset_id: Option<String>,
    columns: Option<Vec<LakehouseNamespaceSqlColumnOut>>,
    row_count: Option<i64>,
    rows: Option<Vec<Vec<serde_json::Value>>>,
    truncated: Option<bool>,
}

impl LakehouseNamespaceSqlResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn columns(mut self, value: Vec<LakehouseNamespaceSqlColumnOut>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LakehouseNamespaceSqlResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](LakehouseNamespaceSqlResponseOutBuilder::asset_id)
    /// - [`columns`](LakehouseNamespaceSqlResponseOutBuilder::columns)
    /// - [`row_count`](LakehouseNamespaceSqlResponseOutBuilder::row_count)
    /// - [`rows`](LakehouseNamespaceSqlResponseOutBuilder::rows)
    /// - [`truncated`](LakehouseNamespaceSqlResponseOutBuilder::truncated)
    pub fn build(self) -> Result<LakehouseNamespaceSqlResponseOut, BuildError> {
        Ok(LakehouseNamespaceSqlResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            columns: self.columns.ok_or_else(|| BuildError::missing_field("columns"))?,
            row_count: self.row_count.ok_or_else(|| BuildError::missing_field("row_count"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
