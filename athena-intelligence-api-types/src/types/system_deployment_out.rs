pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The spec version deployed to the environment, and who moved it there.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemDeploymentOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub promoted_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub promoted_by_ref: String,
    #[serde(default)]
    pub version_id: String,
}

impl SystemDeploymentOut {
    pub fn builder() -> SystemDeploymentOutBuilder {
        <SystemDeploymentOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemDeploymentOutBuilder {
    approval_id: Option<String>,
    promoted_at: Option<DateTime<FixedOffset>>,
    promoted_by_ref: Option<String>,
    version_id: Option<String>,
}

impl SystemDeploymentOutBuilder {
    pub fn approval_id(mut self, value: impl Into<String>) -> Self {
        self.approval_id = Some(value.into());
        self
    }

    pub fn promoted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.promoted_at = Some(value);
        self
    }

    pub fn promoted_by_ref(mut self, value: impl Into<String>) -> Self {
        self.promoted_by_ref = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemDeploymentOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`promoted_at`](SystemDeploymentOutBuilder::promoted_at)
    /// - [`promoted_by_ref`](SystemDeploymentOutBuilder::promoted_by_ref)
    /// - [`version_id`](SystemDeploymentOutBuilder::version_id)
    pub fn build(self) -> Result<SystemDeploymentOut, BuildError> {
        Ok(SystemDeploymentOut {
            approval_id: self.approval_id,
            promoted_at: self.promoted_at.ok_or_else(|| BuildError::missing_field("promoted_at"))?,
            promoted_by_ref: self.promoted_by_ref.ok_or_else(|| BuildError::missing_field("promoted_by_ref"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
