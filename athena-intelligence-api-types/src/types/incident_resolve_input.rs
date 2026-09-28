pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncidentResolveInput {
    /// With node_key: the spec environment whose node's active incident is resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Incident id (inc_…); it must belong to the project. Omit to select by environment + node_key instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    /// With environment: the node whose one active incident is resolved; incident_not_found when the node has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_key: Option<String>,
    /// Project asset id (asset_…) whose system spec the node belongs to. The caller needs EDIT on it.
    #[serde(default)]
    pub project_asset_id: String,
    /// Why it is resolved: the node recovered, or a person fixed it. Required and non-blank — it is the timeline's audit explanation.
    #[serde(default)]
    pub reason: String,
}

impl IncidentResolveInput {
    pub fn builder() -> IncidentResolveInputBuilder {
        <IncidentResolveInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentResolveInputBuilder {
    environment: Option<String>,
    incident_id: Option<String>,
    node_key: Option<String>,
    project_asset_id: Option<String>,
    reason: Option<String>,
}

impl IncidentResolveInputBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn incident_id(mut self, value: impl Into<String>) -> Self {
        self.incident_id = Some(value.into());
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

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncidentResolveInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_asset_id`](IncidentResolveInputBuilder::project_asset_id)
    /// - [`reason`](IncidentResolveInputBuilder::reason)
    pub fn build(self) -> Result<IncidentResolveInput, BuildError> {
        Ok(IncidentResolveInput {
            environment: self.environment,
            incident_id: self.incident_id,
            node_key: self.node_key,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}

