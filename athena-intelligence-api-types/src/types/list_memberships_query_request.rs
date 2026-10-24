pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_memberships
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembershipsQueryRequest {
    /// Also return memberships that were removed from the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// `next_cursor` from the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum memberships per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListMembershipsQueryRequest {
    pub fn builder() -> ListMembershipsQueryRequestBuilder {
        <ListMembershipsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembershipsQueryRequestBuilder {
    include_archived: Option<bool>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListMembershipsQueryRequestBuilder {
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembershipsQueryRequest`].
    pub fn build(self) -> Result<ListMembershipsQueryRequest, BuildError> {
        Ok(ListMembershipsQueryRequest {
            include_archived: self.include_archived,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

