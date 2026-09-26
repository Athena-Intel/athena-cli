pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncidentCloseResolvedInput {
    /// Close incidents resolved longer ago than this duration (30s, 2m, 4h, 2d); defaults to the spec's escalation.close_after_resolved (24h).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Spec environment: development or production.
    #[serde(default)]
    pub environment: String,
    /// Project asset id (asset_…) whose system spec the node belongs to. The caller needs EDIT on it.
    #[serde(default)]
    pub project_asset_id: String,
}

impl IncidentCloseResolvedInput {
    pub fn builder() -> IncidentCloseResolvedInputBuilder {
        <IncidentCloseResolvedInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentCloseResolvedInputBuilder {
    after: Option<String>,
    environment: Option<String>,
    project_asset_id: Option<String>,
}

impl IncidentCloseResolvedInputBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncidentCloseResolvedInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](IncidentCloseResolvedInputBuilder::environment)
    /// - [`project_asset_id`](IncidentCloseResolvedInputBuilder::project_asset_id)
    pub fn build(self) -> Result<IncidentCloseResolvedInput, BuildError> {
        Ok(IncidentCloseResolvedInput {
            after: self.after,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
        })
    }
}

