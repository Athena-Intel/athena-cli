pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_runs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationsListRunsQueryRequest {
    /// Only runs in these statuses (scheduled, queued, running, needs_input, completed, failed, canceled). Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<Vec<String>>,
    /// Maximum number of runs per page (1-200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of runs to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl AutomationsListRunsQueryRequest {
    pub fn builder() -> AutomationsListRunsQueryRequestBuilder {
        <AutomationsListRunsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationsListRunsQueryRequestBuilder {
    run_status: Option<Vec<String>>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl AutomationsListRunsQueryRequestBuilder {
    pub fn run_status(mut self, value: Vec<String>) -> Self {
        self.run_status = Some(value);
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

    /// Consumes the builder and constructs a [`AutomationsListRunsQueryRequest`].
    pub fn build(self) -> Result<AutomationsListRunsQueryRequest, BuildError> {
        Ok(AutomationsListRunsQueryRequest {
            run_status: self.run_status,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

