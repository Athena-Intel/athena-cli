pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A queued redrive.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunRedriveResponseOut {
    #[serde(default)]
    pub automation_asset_id: String,
    /// How many finished step attempts before the resume point were copied from the source run instead of run again
    #[serde(default)]
    pub copied_steps: i64,
    #[serde(default)]
    pub fingerprint: String,
    /// The run this one redrives
    #[serde(default)]
    pub redrive_of_run_id: String,
    /// The top-level step the redrive resumes at
    #[serde(default)]
    pub resume_from_step_id: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The source run's pinned version, which the redrive runs
    #[serde(default)]
    pub version_id: String,
}

impl AutomationRunRedriveResponseOut {
    pub fn builder() -> AutomationRunRedriveResponseOutBuilder {
        <AutomationRunRedriveResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunRedriveResponseOutBuilder {
    automation_asset_id: Option<String>,
    copied_steps: Option<i64>,
    fingerprint: Option<String>,
    redrive_of_run_id: Option<String>,
    resume_from_step_id: Option<String>,
    run_id: Option<String>,
    run_status: Option<String>,
    version_id: Option<String>,
}

impl AutomationRunRedriveResponseOutBuilder {
    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn copied_steps(mut self, value: i64) -> Self {
        self.copied_steps = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn redrive_of_run_id(mut self, value: impl Into<String>) -> Self {
        self.redrive_of_run_id = Some(value.into());
        self
    }

    pub fn resume_from_step_id(mut self, value: impl Into<String>) -> Self {
        self.resume_from_step_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`AutomationRunRedriveResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automation_asset_id`](AutomationRunRedriveResponseOutBuilder::automation_asset_id)
    /// - [`copied_steps`](AutomationRunRedriveResponseOutBuilder::copied_steps)
    /// - [`fingerprint`](AutomationRunRedriveResponseOutBuilder::fingerprint)
    /// - [`redrive_of_run_id`](AutomationRunRedriveResponseOutBuilder::redrive_of_run_id)
    /// - [`resume_from_step_id`](AutomationRunRedriveResponseOutBuilder::resume_from_step_id)
    /// - [`run_id`](AutomationRunRedriveResponseOutBuilder::run_id)
    /// - [`version_id`](AutomationRunRedriveResponseOutBuilder::version_id)
    pub fn build(self) -> Result<AutomationRunRedriveResponseOut, BuildError> {
        Ok(AutomationRunRedriveResponseOut {
            automation_asset_id: self.automation_asset_id.ok_or_else(|| BuildError::missing_field("automation_asset_id"))?,
            copied_steps: self.copied_steps.ok_or_else(|| BuildError::missing_field("copied_steps"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            redrive_of_run_id: self.redrive_of_run_id.ok_or_else(|| BuildError::missing_field("redrive_of_run_id"))?,
            resume_from_step_id: self.resume_from_step_id.ok_or_else(|| BuildError::missing_field("resume_from_step_id"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            run_status: self.run_status,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
