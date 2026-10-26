pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseComputeSettingsUpdateRequestIn {
    /// Seconds without activity after which the database suspends (scales to zero). 0 means it never suspends: the compute keeps running, and keeps using capacity, until you change this. Otherwise 60 to 86400 (24 hours). A new database starts with 1800 (30 minutes). The same setting as auto-suspend in the database's Settings in Athena.
    #[serde(default)]
    pub suspend_timeout_seconds: i64,
}

impl DatabaseComputeSettingsUpdateRequestIn {
    pub fn builder() -> DatabaseComputeSettingsUpdateRequestInBuilder {
        <DatabaseComputeSettingsUpdateRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseComputeSettingsUpdateRequestInBuilder {
    suspend_timeout_seconds: Option<i64>,
}

impl DatabaseComputeSettingsUpdateRequestInBuilder {
    pub fn suspend_timeout_seconds(mut self, value: i64) -> Self {
        self.suspend_timeout_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseComputeSettingsUpdateRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`suspend_timeout_seconds`](DatabaseComputeSettingsUpdateRequestInBuilder::suspend_timeout_seconds)
    pub fn build(self) -> Result<DatabaseComputeSettingsUpdateRequestIn, BuildError> {
        Ok(DatabaseComputeSettingsUpdateRequestIn {
            suspend_timeout_seconds: self.suspend_timeout_seconds.ok_or_else(|| BuildError::missing_field("suspend_timeout_seconds"))?,
        })
    }
}

