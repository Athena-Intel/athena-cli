pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How much attributed Computer LLM gateway usage `per_user` carries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceUsageGatewayAttributionOut {
    /// For `partial`, the first UTC day whose Computer usage sessions were all read; null otherwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_since: Option<NaiveDate>,
    /// `applied`: every Computer usage session in the window was read. `partial`: the window holds more of them than one request reads; every one from `covered_since` on was read, and usage attributed before that day may still be under the Computer's owner. `unavailable`: the read failed and `per_user` is the breakdown without attribution.
    pub status: WorkspaceUsageGatewayAttributionOutStatus,
}

impl WorkspaceUsageGatewayAttributionOut {
    pub fn builder() -> WorkspaceUsageGatewayAttributionOutBuilder {
        <WorkspaceUsageGatewayAttributionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageGatewayAttributionOutBuilder {
    covered_since: Option<NaiveDate>,
    status: Option<WorkspaceUsageGatewayAttributionOutStatus>,
}

impl WorkspaceUsageGatewayAttributionOutBuilder {
    pub fn covered_since(mut self, value: NaiveDate) -> Self {
        self.covered_since = Some(value);
        self
    }

    pub fn status(mut self, value: WorkspaceUsageGatewayAttributionOutStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageGatewayAttributionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](WorkspaceUsageGatewayAttributionOutBuilder::status)
    pub fn build(self) -> Result<WorkspaceUsageGatewayAttributionOut, BuildError> {
        Ok(WorkspaceUsageGatewayAttributionOut {
            covered_since: self.covered_since,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
