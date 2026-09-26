pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search_members
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchMembersQueryRequest {
    /// Search prefix, matched case-insensitively against email, first name and last name
    #[serde(default)]
    pub q: String,
    /// Maximum number of people to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl SearchMembersQueryRequest {
    pub fn builder() -> SearchMembersQueryRequestBuilder {
        <SearchMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMembersQueryRequestBuilder {
    q: Option<String>,
    limit: Option<i64>,
}

impl SearchMembersQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchMembersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`q`](SearchMembersQueryRequestBuilder::q)
    pub fn build(self) -> Result<SearchMembersQueryRequest, BuildError> {
        Ok(SearchMembersQueryRequest {
            q: self.q.ok_or_else(|| BuildError::missing_field("q"))?,
            limit: self.limit,
        })
    }
}

