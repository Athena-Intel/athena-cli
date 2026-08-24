pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AthenaResourcesSearchInput {
    /// Number of search results to return (default: 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Search query to find relevant Athena documentation about agents, integrations, use cases, and AOPs.
    #[serde(default)]
    pub query: String,
}

impl AthenaResourcesSearchInput {
    pub fn builder() -> AthenaResourcesSearchInputBuilder {
        <AthenaResourcesSearchInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AthenaResourcesSearchInputBuilder {
    limit: Option<i64>,
    query: Option<String>,
}

impl AthenaResourcesSearchInputBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AthenaResourcesSearchInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](AthenaResourcesSearchInputBuilder::query)
    pub fn build(self) -> Result<AthenaResourcesSearchInput, BuildError> {
        Ok(AthenaResourcesSearchInput {
            limit: self.limit,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
        })
    }
}

