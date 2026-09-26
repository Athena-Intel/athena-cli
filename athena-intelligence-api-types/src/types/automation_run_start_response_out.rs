pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A queued manual run.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunStartResponseOut {
    #[serde(default)]
    pub automation_asset_id: String,
    /// True when this response replays the run an earlier request with the same Idempotency-Key already queued; no second run was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<bool>,
    #[serde(default)]
    pub fingerprint: String,
    /// live, or dry_run for a rehearsal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<AutomationRunStartResponseOutMode>,
    #[serde(default)]
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    #[serde(default)]
    pub version_id: String,
}

impl AutomationRunStartResponseOut {
    pub fn builder() -> AutomationRunStartResponseOutBuilder {
        <AutomationRunStartResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunStartResponseOutBuilder {
    automation_asset_id: Option<String>,
    deduplicated: Option<bool>,
    fingerprint: Option<String>,
    mode: Option<AutomationRunStartResponseOutMode>,
    run_id: Option<String>,
    run_status: Option<String>,
    version_id: Option<String>,
}

impl AutomationRunStartResponseOutBuilder {
    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn deduplicated(mut self, value: bool) -> Self {
        self.deduplicated = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn mode(mut self, value: AutomationRunStartResponseOutMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn run_status(mut self, value: impl Into<String>) -> Self {
        self.run_status = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunStartResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automation_asset_id`](AutomationRunStartResponseOutBuilder::automation_asset_id)
    /// - [`fingerprint`](AutomationRunStartResponseOutBuilder::fingerprint)
    /// - [`run_id`](AutomationRunStartResponseOutBuilder::run_id)
    /// - [`version_id`](AutomationRunStartResponseOutBuilder::version_id)
    pub fn build(self) -> Result<AutomationRunStartResponseOut, BuildError> {
        Ok(AutomationRunStartResponseOut {
            automation_asset_id: self.automation_asset_id.ok_or_else(|| BuildError::missing_field("automation_asset_id"))?,
            deduplicated: self.deduplicated,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            mode: self.mode,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            run_status: self.run_status,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
