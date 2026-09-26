pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemHealthRepairAttemptInput {
    /// Spec environment: development or production.
    #[serde(default)]
    pub environment: String,
    /// The spec node key.
    #[serde(default)]
    pub node_key: String,
    /// Project asset id (asset_…) whose system spec the node belongs to. The caller needs EDIT on it.
    #[serde(default)]
    pub project_asset_id: String,
}

impl SystemHealthRepairAttemptInput {
    pub fn builder() -> SystemHealthRepairAttemptInputBuilder {
        <SystemHealthRepairAttemptInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemHealthRepairAttemptInputBuilder {
    environment: Option<String>,
    node_key: Option<String>,
    project_asset_id: Option<String>,
}

impl SystemHealthRepairAttemptInputBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn node_key(mut self, value: impl Into<String>) -> Self {
        self.node_key = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemHealthRepairAttemptInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](SystemHealthRepairAttemptInputBuilder::environment)
    /// - [`node_key`](SystemHealthRepairAttemptInputBuilder::node_key)
    /// - [`project_asset_id`](SystemHealthRepairAttemptInputBuilder::project_asset_id)
    pub fn build(self) -> Result<SystemHealthRepairAttemptInput, BuildError> {
        Ok(SystemHealthRepairAttemptInput {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            node_key: self.node_key.ok_or_else(|| BuildError::missing_field("node_key"))?,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
        })
    }
}

