pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One offset page of a script's runs, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedScriptRunsOut {
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<ScriptRunOut>,
    #[serde(default)]
    pub limit: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    #[serde(default)]
    pub offset: i64,
    #[serde(default)]
    pub total: i64,
}

impl PaginatedScriptRunsOut {
    pub fn builder() -> PaginatedScriptRunsOutBuilder {
        <PaginatedScriptRunsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedScriptRunsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<ScriptRunOut>>,
    limit: Option<i64>,
    next_offset: Option<i64>,
    offset: Option<i64>,
    total: Option<i64>,
}

impl PaginatedScriptRunsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ScriptRunOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn next_offset(mut self, value: i64) -> Self {
        self.next_offset = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedScriptRunsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedScriptRunsOutBuilder::has_more)
    /// - [`items`](PaginatedScriptRunsOutBuilder::items)
    /// - [`limit`](PaginatedScriptRunsOutBuilder::limit)
    /// - [`offset`](PaginatedScriptRunsOutBuilder::offset)
    /// - [`total`](PaginatedScriptRunsOutBuilder::total)
    pub fn build(self) -> Result<PaginatedScriptRunsOut, BuildError> {
        Ok(PaginatedScriptRunsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            next_offset: self.next_offset,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
