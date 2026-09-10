pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The caller's Microsoft 365 source readiness, for app boot screens.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MeSourcesResponseOut {
    /// Per-group source counts (mail / files / sites / chats).
    #[serde(rename = "groupCounts")]
    #[serde(default)]
    pub group_counts: HashMap<String, i64>,
    /// Present while (or shortly after) a SharePoint fan-out runs for the caller; null when there is no live provisioning signal.
    #[serde(rename = "sharepointProvisioning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharepoint_provisioning: Option<MeSharePointProvisioningOut>,
    /// All of the caller's connected Microsoft 365 sources.
    #[serde(rename = "totalSources")]
    #[serde(default)]
    pub total_sources: i64,
}

impl MeSourcesResponseOut {
    pub fn builder() -> MeSourcesResponseOutBuilder {
        <MeSourcesResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeSourcesResponseOutBuilder {
    group_counts: Option<HashMap<String, i64>>,
    sharepoint_provisioning: Option<MeSharePointProvisioningOut>,
    total_sources: Option<i64>,
}

impl MeSourcesResponseOutBuilder {
    pub fn group_counts(mut self, value: HashMap<String, i64>) -> Self {
        self.group_counts = Some(value);
        self
    }

    pub fn sharepoint_provisioning(mut self, value: MeSharePointProvisioningOut) -> Self {
        self.sharepoint_provisioning = Some(value);
        self
    }

    pub fn total_sources(mut self, value: i64) -> Self {
        self.total_sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeSourcesResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_counts`](MeSourcesResponseOutBuilder::group_counts)
    /// - [`total_sources`](MeSourcesResponseOutBuilder::total_sources)
    pub fn build(self) -> Result<MeSourcesResponseOut, BuildError> {
        Ok(MeSourcesResponseOut {
            group_counts: self.group_counts.ok_or_else(|| BuildError::missing_field("group_counts"))?,
            sharepoint_provisioning: self.sharepoint_provisioning,
            total_sources: self.total_sources.ok_or_else(|| BuildError::missing_field("total_sources"))?,
        })
    }
}
