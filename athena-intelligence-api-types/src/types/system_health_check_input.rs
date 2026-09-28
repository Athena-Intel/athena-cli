pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemHealthCheckInput {
    /// Spec environment to check: `development` or `production`.
    #[serde(default)]
    pub environment: String,
    /// Node keys to re-check; every node when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<String>>,
    /// Project asset id (asset_…) whose published system spec is checked. The caller needs VIEW on it; a node is read only when the caller can also view the node's bound asset (otherwise it reads unknown/grant_denied).
    #[serde(default)]
    pub project_asset_id: String,
    /// The detector run recording this check (bind `$run.id`); stamped on every check row for the run inspector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// The spec version (pssv_…) the caller expects to be the environment's effective one — its deployment, else the latest publish. A pin, not a choice: the check writes the environment's standing health map, so a version that is not the effective one is refused (version_not_effective) rather than evaluated. Omit to check whatever is effective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl SystemHealthCheckInput {
    pub fn builder() -> SystemHealthCheckInputBuilder {
        <SystemHealthCheckInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemHealthCheckInputBuilder {
    environment: Option<String>,
    only: Option<Vec<String>>,
    project_asset_id: Option<String>,
    run_id: Option<String>,
    version_id: Option<String>,
}

impl SystemHealthCheckInputBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn only(mut self, value: Vec<String>) -> Self {
        self.only = Some(value);
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

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemHealthCheckInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](SystemHealthCheckInputBuilder::environment)
    /// - [`project_asset_id`](SystemHealthCheckInputBuilder::project_asset_id)
    pub fn build(self) -> Result<SystemHealthCheckInput, BuildError> {
        Ok(SystemHealthCheckInput {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            only: self.only,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            run_id: self.run_id,
            version_id: self.version_id,
        })
    }
}

