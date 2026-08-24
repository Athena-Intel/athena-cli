pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InDepthAnalysisInput {
    /// The Asset IDs to include in the analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<String>>,
    /// The query to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl InDepthAnalysisInput {
    pub fn builder() -> InDepthAnalysisInputBuilder {
        <InDepthAnalysisInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InDepthAnalysisInputBuilder {
    asset_ids: Option<Vec<String>>,
    query: Option<String>,
}

impl InDepthAnalysisInputBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InDepthAnalysisInput`].
    pub fn build(self) -> Result<InDepthAnalysisInput, BuildError> {
        Ok(InDepthAnalysisInput {
            asset_ids: self.asset_ids,
            query: self.query,
        })
    }
}

