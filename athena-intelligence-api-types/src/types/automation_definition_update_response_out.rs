pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of a draft replacement.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationDefinitionUpdateResponseOut {
    #[serde(default)]
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether an existing @latest snapshot was re-pointed at the new draft
    #[serde(default)]
    pub snapshot_refreshed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The updated_at stamp written into the Keryx document
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl AutomationDefinitionUpdateResponseOut {
    pub fn builder() -> AutomationDefinitionUpdateResponseOutBuilder {
        <AutomationDefinitionUpdateResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationDefinitionUpdateResponseOutBuilder {
    asset_id: Option<String>,
    message: Option<String>,
    snapshot_refreshed: Option<bool>,
    snapshot_version_id: Option<String>,
    status: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl AutomationDefinitionUpdateResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn snapshot_refreshed(mut self, value: bool) -> Self {
        self.snapshot_refreshed = Some(value);
        self
    }

    pub fn snapshot_version_id(mut self, value: impl Into<String>) -> Self {
        self.snapshot_version_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationDefinitionUpdateResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationDefinitionUpdateResponseOutBuilder::asset_id)
    /// - [`snapshot_refreshed`](AutomationDefinitionUpdateResponseOutBuilder::snapshot_refreshed)
    /// - [`updated_at`](AutomationDefinitionUpdateResponseOutBuilder::updated_at)
    pub fn build(self) -> Result<AutomationDefinitionUpdateResponseOut, BuildError> {
        Ok(AutomationDefinitionUpdateResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            message: self.message,
            snapshot_refreshed: self.snapshot_refreshed.ok_or_else(|| BuildError::missing_field("snapshot_refreshed"))?,
            snapshot_version_id: self.snapshot_version_id,
            status: self.status,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
