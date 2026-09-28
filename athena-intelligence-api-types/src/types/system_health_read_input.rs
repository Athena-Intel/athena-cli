pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemHealthReadInput {
    /// Spec environment to read: `development` or `production`.
    #[serde(default)]
    pub environment: String,
    /// Node keys to return; every node with a reading when omitted. With exactly one node returned, `state` and `reason` are filled at the top level for a gate step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<String>>,
    /// Project asset id (asset_…) whose current health map is read. The caller needs VIEW on it; nodes whose bound asset the caller cannot view are counted as hidden and not returned.
    #[serde(default)]
    pub project_asset_id: String,
}

impl SystemHealthReadInput {
    pub fn builder() -> SystemHealthReadInputBuilder {
        <SystemHealthReadInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemHealthReadInputBuilder {
    environment: Option<String>,
    only: Option<Vec<String>>,
    project_asset_id: Option<String>,
}

impl SystemHealthReadInputBuilder {
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

    /// Consumes the builder and constructs a [`SystemHealthReadInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](SystemHealthReadInputBuilder::environment)
    /// - [`project_asset_id`](SystemHealthReadInputBuilder::project_asset_id)
    pub fn build(self) -> Result<SystemHealthReadInput, BuildError> {
        Ok(SystemHealthReadInput {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            only: self.only,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
        })
    }
}

