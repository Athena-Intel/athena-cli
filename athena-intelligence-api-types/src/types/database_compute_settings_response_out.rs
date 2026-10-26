pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The database's compute settings that the public API can read and set.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseComputeSettingsResponseOut {
    /// The database asset ID
    #[serde(default)]
    pub asset_id: String,
    /// Seconds without activity after which the database suspends (scales to zero). 0 means it never suspends: the compute keeps running, and keeps using capacity, until you change this. Otherwise 60 to 86400 (24 hours). A new database starts with 1800 (30 minutes). The same setting as auto-suspend in the database's Settings in Athena.
    #[serde(default)]
    pub suspend_timeout_seconds: i64,
}

impl DatabaseComputeSettingsResponseOut {
    pub fn builder() -> DatabaseComputeSettingsResponseOutBuilder {
        <DatabaseComputeSettingsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseComputeSettingsResponseOutBuilder {
    asset_id: Option<String>,
    suspend_timeout_seconds: Option<i64>,
}

impl DatabaseComputeSettingsResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn suspend_timeout_seconds(mut self, value: i64) -> Self {
        self.suspend_timeout_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseComputeSettingsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DatabaseComputeSettingsResponseOutBuilder::asset_id)
    /// - [`suspend_timeout_seconds`](DatabaseComputeSettingsResponseOutBuilder::suspend_timeout_seconds)
    pub fn build(self) -> Result<DatabaseComputeSettingsResponseOut, BuildError> {
        Ok(DatabaseComputeSettingsResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            suspend_timeout_seconds: self.suspend_timeout_seconds.ok_or_else(|| BuildError::missing_field("suspend_timeout_seconds"))?,
        })
    }
}
