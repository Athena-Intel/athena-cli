pub use crate::prelude::*;
use super::*;

/// Query parameters for select
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SelectQueryRequest {
    /// Columns to return (comma-separated, e.g., 'id,name,email')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    /// Order by clause (e.g., 'created_at.desc', 'name.asc')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Maximum number of rows to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of rows to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl SelectQueryRequest {
    pub fn builder() -> SelectQueryRequestBuilder {
        <SelectQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SelectQueryRequestBuilder {
    select: Option<String>,
    order: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl SelectQueryRequestBuilder {
    pub fn select(mut self, value: impl Into<String>) -> Self {
        self.select = Some(value.into());
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SelectQueryRequest`].
    pub fn build(self) -> Result<SelectQueryRequest, BuildError> {
        Ok(SelectQueryRequest {
            select: self.select,
            order: self.order,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

