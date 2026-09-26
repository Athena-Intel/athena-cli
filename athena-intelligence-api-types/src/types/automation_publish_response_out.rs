pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of a publish.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationPublishResponseOut {
    #[serde(default)]
    pub asset_id: String,
    /// False when the latest version already had this fingerprint
    #[serde(default)]
    pub created_version: bool,
    #[serde(default)]
    pub fingerprint: String,
    /// Compile warnings and post-commit provisioning errors
    #[serde(default)]
    pub issues: Vec<PublishIssueOut>,
    #[serde(default)]
    pub trigger_rows: Vec<AutomationPublishTriggerRowOut>,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub version_id: String,
}

impl AutomationPublishResponseOut {
    pub fn builder() -> AutomationPublishResponseOutBuilder {
        <AutomationPublishResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationPublishResponseOutBuilder {
    asset_id: Option<String>,
    created_version: Option<bool>,
    fingerprint: Option<String>,
    issues: Option<Vec<PublishIssueOut>>,
    trigger_rows: Option<Vec<AutomationPublishTriggerRowOut>>,
    version: Option<i64>,
    version_id: Option<String>,
}

impl AutomationPublishResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn created_version(mut self, value: bool) -> Self {
        self.created_version = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<PublishIssueOut>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn trigger_rows(mut self, value: Vec<AutomationPublishTriggerRowOut>) -> Self {
        self.trigger_rows = Some(value);
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

    /// Consumes the builder and constructs a [`AutomationPublishResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationPublishResponseOutBuilder::asset_id)
    /// - [`created_version`](AutomationPublishResponseOutBuilder::created_version)
    /// - [`fingerprint`](AutomationPublishResponseOutBuilder::fingerprint)
    /// - [`issues`](AutomationPublishResponseOutBuilder::issues)
    /// - [`trigger_rows`](AutomationPublishResponseOutBuilder::trigger_rows)
    /// - [`version`](AutomationPublishResponseOutBuilder::version)
    /// - [`version_id`](AutomationPublishResponseOutBuilder::version_id)
    pub fn build(self) -> Result<AutomationPublishResponseOut, BuildError> {
        Ok(AutomationPublishResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            created_version: self.created_version.ok_or_else(|| BuildError::missing_field("created_version"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            issues: self.issues.ok_or_else(|| BuildError::missing_field("issues"))?,
            trigger_rows: self.trigger_rows.ok_or_else(|| BuildError::missing_field("trigger_rows"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
