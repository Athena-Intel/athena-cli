pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for a wake request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatabaseWakeResponseOut {
    /// The database asset ID
    #[serde(default)]
    pub asset_id: String,
    /// Optional detail from the control plane
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// `running`: the compute was already running; its idle clock was reset. `starting`: the compute is starting; poll `GET /databases/{asset_id}/compute-status` until it reports `running`.
    pub status: DatabaseWakeResponseOutStatus,
}

impl DatabaseWakeResponseOut {
    pub fn builder() -> DatabaseWakeResponseOutBuilder {
        <DatabaseWakeResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseWakeResponseOutBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    status: Option<DatabaseWakeResponseOutStatus>,
}

impl DatabaseWakeResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn status(mut self, value: DatabaseWakeResponseOutStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseWakeResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DatabaseWakeResponseOutBuilder::asset_id)
    /// - [`status`](DatabaseWakeResponseOutBuilder::status)
    pub fn build(self) -> Result<DatabaseWakeResponseOut, BuildError> {
        Ok(DatabaseWakeResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
