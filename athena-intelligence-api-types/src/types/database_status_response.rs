pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for database status check.
/// 
/// Used to determine if a serverless database is running or suspended
/// (scale-to-zero). Poll this endpoint to know when a database is ready.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatabaseStatusResponse {
    /// The database asset ID
    #[serde(default)]
    pub asset_id: String,
    /// Optional status message or error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Database provider ('neon_cloud' or 'kronos')
    #[serde(default)]
    pub provider: String,
    /// Standardized database status
    pub status: DatabaseStatusResponseStatus,
}

impl DatabaseStatusResponse {
    pub fn builder() -> DatabaseStatusResponseBuilder {
        <DatabaseStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseStatusResponseBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    provider: Option<String>,
    status: Option<DatabaseStatusResponseStatus>,
}

impl DatabaseStatusResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn status(mut self, value: DatabaseStatusResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatabaseStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DatabaseStatusResponseBuilder::asset_id)
    /// - [`provider`](DatabaseStatusResponseBuilder::provider)
    /// - [`status`](DatabaseStatusResponseBuilder::status)
    pub fn build(self) -> Result<DatabaseStatusResponse, BuildError> {
        Ok(DatabaseStatusResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
