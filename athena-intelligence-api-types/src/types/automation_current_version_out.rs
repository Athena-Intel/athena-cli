pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The current version, with the definition it was compiled from.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationCurrentVersionOut {
    /// The definition document this version was compiled from
    #[serde(default)]
    pub definition: HashMap<String, serde_json::Value>,
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

impl AutomationCurrentVersionOut {
    pub fn builder() -> AutomationCurrentVersionOutBuilder {
        <AutomationCurrentVersionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationCurrentVersionOutBuilder {
    definition: Option<HashMap<String, serde_json::Value>>,
    fingerprint: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    published_by_ref: Option<String>,
    superseded_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
    version_id: Option<String>,
}

impl AutomationCurrentVersionOutBuilder {
    pub fn definition(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.definition = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`AutomationCurrentVersionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](AutomationCurrentVersionOutBuilder::definition)
    /// - [`fingerprint`](AutomationCurrentVersionOutBuilder::fingerprint)
    /// - [`published_at`](AutomationCurrentVersionOutBuilder::published_at)
    /// - [`published_by_ref`](AutomationCurrentVersionOutBuilder::published_by_ref)
    /// - [`version`](AutomationCurrentVersionOutBuilder::version)
    /// - [`version_id`](AutomationCurrentVersionOutBuilder::version_id)
    pub fn build(self) -> Result<AutomationCurrentVersionOut, BuildError> {
        Ok(AutomationCurrentVersionOut {
            definition: self.definition.ok_or_else(|| BuildError::missing_field("definition"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            published_at: self.published_at.ok_or_else(|| BuildError::missing_field("published_at"))?,
            published_by_ref: self.published_by_ref.ok_or_else(|| BuildError::missing_field("published_by_ref"))?,
            superseded_at: self.superseded_at,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
