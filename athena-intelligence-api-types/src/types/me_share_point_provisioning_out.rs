pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Live SharePoint fan-out progress for the caller, when one is running.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeSharePointProvisioningOut {
    /// Sites made searchable so far.
    #[serde(rename = "provisionedSites")]
    #[serde(default)]
    pub provisioned_sites: i64,
    /// 'in_progress' while the site sweep runs, 'complete' after.
    #[serde(default)]
    pub status: String,
    /// Total reachable sites; 0 while enumeration is still running.
    #[serde(rename = "totalSites")]
    #[serde(default)]
    pub total_sites: i64,
}

impl MeSharePointProvisioningOut {
    pub fn builder() -> MeSharePointProvisioningOutBuilder {
        <MeSharePointProvisioningOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeSharePointProvisioningOutBuilder {
    provisioned_sites: Option<i64>,
    status: Option<String>,
    total_sites: Option<i64>,
}

impl MeSharePointProvisioningOutBuilder {
    pub fn provisioned_sites(mut self, value: i64) -> Self {
        self.provisioned_sites = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn total_sites(mut self, value: i64) -> Self {
        self.total_sites = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeSharePointProvisioningOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provisioned_sites`](MeSharePointProvisioningOutBuilder::provisioned_sites)
    /// - [`status`](MeSharePointProvisioningOutBuilder::status)
    /// - [`total_sites`](MeSharePointProvisioningOutBuilder::total_sites)
    pub fn build(self) -> Result<MeSharePointProvisioningOut, BuildError> {
        Ok(MeSharePointProvisioningOut {
            provisioned_sites: self.provisioned_sites.ok_or_else(|| BuildError::missing_field("provisioned_sites"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            total_sites: self.total_sites.ok_or_else(|| BuildError::missing_field("total_sites"))?,
        })
    }
}
