pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_usage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUsageQueryRequest {
    /// Reporting window in days, ending now (1-365)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Count a Computer's LLM gateway calls that its app attributed to a session under that session's member in `per_user`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_attribution: Option<bool>,
}

impl GetUsageQueryRequest {
    pub fn builder() -> GetUsageQueryRequestBuilder {
        <GetUsageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUsageQueryRequestBuilder {
    days: Option<i64>,
    gateway_attribution: Option<bool>,
}

impl GetUsageQueryRequestBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn gateway_attribution(mut self, value: bool) -> Self {
        self.gateway_attribution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUsageQueryRequest`].
    pub fn build(self) -> Result<GetUsageQueryRequest, BuildError> {
        Ok(GetUsageQueryRequest {
            days: self.days,
            gateway_attribution: self.gateway_attribution,
        })
    }
}

