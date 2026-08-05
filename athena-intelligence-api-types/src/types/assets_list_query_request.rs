pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsListQueryRequest {
    /// Maximum number of assets to return per page (1-500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of assets to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status, workspace_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    /// JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Workspace to list assets from. Caller must be a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AssetsListQueryRequest {
    pub fn builder() -> AssetsListQueryRequestBuilder {
        <AssetsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    filters: Option<String>,
    sort: Option<String>,
    workspace_id: Option<String>,
}

impl AssetsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn filters(mut self, value: impl Into<String>) -> Self {
        self.filters = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsListQueryRequest`].
    pub fn build(self) -> Result<AssetsListQueryRequest, BuildError> {
        Ok(AssetsListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            filters: self.filters,
            sort: self.sort,
            workspace_id: self.workspace_id,
        })
    }
}

