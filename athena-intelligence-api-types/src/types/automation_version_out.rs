pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One immutable published version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationVersionOut {
    /// sha256 JCS fingerprint of the compiled plan
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub published_at: DateTime<FixedOffset>,
    /// Principal ref of the publisher
    #[serde(default)]
    pub published_by_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_at: Option<DateTime<FixedOffset>>,
    /// Monotonic version number
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub version_id: String,
}

impl AutomationVersionOut {
    pub fn builder() -> AutomationVersionOutBuilder {
        <AutomationVersionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationVersionOutBuilder {
    fingerprint: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    published_by_ref: Option<String>,
    superseded_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
    version_id: Option<String>,
}

impl AutomationVersionOutBuilder {
    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn published_by_ref(mut self, value: impl Into<String>) -> Self {
        self.published_by_ref = Some(value.into());
        self
    }

    pub fn superseded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.superseded_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationVersionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fingerprint`](AutomationVersionOutBuilder::fingerprint)
    /// - [`published_at`](AutomationVersionOutBuilder::published_at)
    /// - [`published_by_ref`](AutomationVersionOutBuilder::published_by_ref)
    /// - [`version`](AutomationVersionOutBuilder::version)
    /// - [`version_id`](AutomationVersionOutBuilder::version_id)
    pub fn build(self) -> Result<AutomationVersionOut, BuildError> {
        Ok(AutomationVersionOut {
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            published_at: self.published_at.ok_or_else(|| BuildError::missing_field("published_at"))?,
            published_by_ref: self.published_by_ref.ok_or_else(|| BuildError::missing_field("published_by_ref"))?,
            superseded_at: self.superseded_at,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
