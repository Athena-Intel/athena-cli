pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_runs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScriptsListRunsQueryRequest {
    /// Maximum number of runs per page (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of runs to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ScriptsListRunsQueryRequest {
    pub fn builder() -> ScriptsListRunsQueryRequestBuilder {
        <ScriptsListRunsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptsListRunsQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ScriptsListRunsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScriptsListRunsQueryRequest`].
    pub fn build(self) -> Result<ScriptsListRunsQueryRequest, BuildError> {
        Ok(ScriptsListRunsQueryRequest {
            limit: self.limit,
            offset: self.offset,
        })
    }
}

