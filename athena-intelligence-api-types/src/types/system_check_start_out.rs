pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A queued manual run of the environment's generated detector.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemCheckStartOut {
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub project_asset_id: String,
    /// Poll GET /automations/runs/{run_id}, then re-read the map
    #[serde(default)]
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The detector's published version
    #[serde(default)]
    pub version_id: String,
}

impl SystemCheckStartOut {
    pub fn builder() -> SystemCheckStartOutBuilder {
        <SystemCheckStartOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemCheckStartOutBuilder {
    environment: Option<String>,
    fingerprint: Option<String>,
    project_asset_id: Option<String>,
    run_id: Option<String>,
    run_status: Option<String>,
    version_id: Option<String>,
}

impl SystemCheckStartOutBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`SystemCheckStartOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](SystemCheckStartOutBuilder::environment)
    /// - [`fingerprint`](SystemCheckStartOutBuilder::fingerprint)
    /// - [`project_asset_id`](SystemCheckStartOutBuilder::project_asset_id)
    /// - [`run_id`](SystemCheckStartOutBuilder::run_id)
    /// - [`version_id`](SystemCheckStartOutBuilder::version_id)
    pub fn build(self) -> Result<SystemCheckStartOut, BuildError> {
        Ok(SystemCheckStartOut {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            run_status: self.run_status,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
