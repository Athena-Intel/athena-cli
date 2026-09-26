pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncidentOpenInput {
    /// The failing system_health_checks id (shc_…) that triggered the open; appended to the incident's failing checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    /// Spec environment: development or production.
    #[serde(default)]
    pub environment: String,
    /// The spec node key.
    #[serde(default)]
    pub node_key: String,
    /// Project asset id (asset_…) whose system spec the node belongs to. The caller needs EDIT on it.
    #[serde(default)]
    pub project_asset_id: String,
    /// Optional title; defaults to the node's current health reading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl IncidentOpenInput {
    pub fn builder() -> IncidentOpenInputBuilder {
        <IncidentOpenInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentOpenInputBuilder {
    check_id: Option<String>,
    environment: Option<String>,
    node_key: Option<String>,
    project_asset_id: Option<String>,
    title: Option<String>,
}

impl IncidentOpenInputBuilder {
    pub fn check_id(mut self, value: impl Into<String>) -> Self {
        self.check_id = Some(value.into());
        self
    }

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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncidentOpenInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](IncidentOpenInputBuilder::environment)
    /// - [`node_key`](IncidentOpenInputBuilder::node_key)
    /// - [`project_asset_id`](IncidentOpenInputBuilder::project_asset_id)
    pub fn build(self) -> Result<IncidentOpenInput, BuildError> {
        Ok(IncidentOpenInput {
            check_id: self.check_id,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            node_key: self.node_key.ok_or_else(|| BuildError::missing_field("node_key"))?,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            title: self.title,
        })
    }
}

