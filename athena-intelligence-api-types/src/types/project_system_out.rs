pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A project's system map in one environment, as the caller may see it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectSystemOut {
    #[serde(default)]
    pub counts: SystemCountsOut,
    /// Null when no version is deployed to the environment yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<SystemDeploymentOut>,
    #[serde(default)]
    pub edges: Vec<SystemEdgeOut>,
    #[serde(default)]
    pub environment: String,
    /// The environment's generated detector and healer
    #[serde(default)]
    pub generated_automations: Vec<GeneratedAutomationOut>,
    /// Nodes returned hidden: their bound asset is not shared with you
    #[serde(default)]
    pub hidden_count: i64,
    /// Active incidents you may not see: counted, never returned
    #[serde(default)]
    pub hidden_incident_count: i64,
    /// The environment's active incidents you may see, oldest first
    #[serde(default)]
    pub incidents: Vec<IncidentOut>,
    #[serde(default)]
    pub nodes: Vec<SystemNodeOut>,
    #[serde(default)]
    pub project_asset_id: String,
    /// False before the project's first publish; the map is then empty
    #[serde(default)]
    pub published: bool,
    /// The effective spec version: the environment's deployment, else the latest publish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<SystemVersionOut>,
}

impl ProjectSystemOut {
    pub fn builder() -> ProjectSystemOutBuilder {
        <ProjectSystemOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectSystemOutBuilder {
    counts: Option<SystemCountsOut>,
    deployment: Option<SystemDeploymentOut>,
    edges: Option<Vec<SystemEdgeOut>>,
    environment: Option<String>,
    generated_automations: Option<Vec<GeneratedAutomationOut>>,
    hidden_count: Option<i64>,
    hidden_incident_count: Option<i64>,
    incidents: Option<Vec<IncidentOut>>,
    nodes: Option<Vec<SystemNodeOut>>,
    project_asset_id: Option<String>,
    published: Option<bool>,
    version: Option<SystemVersionOut>,
}

impl ProjectSystemOutBuilder {
    pub fn counts(mut self, value: SystemCountsOut) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn deployment(mut self, value: SystemDeploymentOut) -> Self {
        self.deployment = Some(value);
        self
    }

    pub fn edges(mut self, value: Vec<SystemEdgeOut>) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn generated_automations(mut self, value: Vec<GeneratedAutomationOut>) -> Self {
        self.generated_automations = Some(value);
        self
    }

    pub fn hidden_count(mut self, value: i64) -> Self {
        self.hidden_count = Some(value);
        self
    }

    pub fn hidden_incident_count(mut self, value: i64) -> Self {
        self.hidden_incident_count = Some(value);
        self
    }

    pub fn incidents(mut self, value: Vec<IncidentOut>) -> Self {
        self.incidents = Some(value);
        self
    }

    pub fn nodes(mut self, value: Vec<SystemNodeOut>) -> Self {
        self.nodes = Some(value);
        self
    }

    pub fn project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.project_asset_id = Some(value.into());
        self
    }

    pub fn published(mut self, value: bool) -> Self {
        self.published = Some(value);
        self
    }

    pub fn version(mut self, value: SystemVersionOut) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectSystemOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counts`](ProjectSystemOutBuilder::counts)
    /// - [`edges`](ProjectSystemOutBuilder::edges)
    /// - [`environment`](ProjectSystemOutBuilder::environment)
    /// - [`generated_automations`](ProjectSystemOutBuilder::generated_automations)
    /// - [`hidden_count`](ProjectSystemOutBuilder::hidden_count)
    /// - [`hidden_incident_count`](ProjectSystemOutBuilder::hidden_incident_count)
    /// - [`incidents`](ProjectSystemOutBuilder::incidents)
    /// - [`nodes`](ProjectSystemOutBuilder::nodes)
    /// - [`project_asset_id`](ProjectSystemOutBuilder::project_asset_id)
    /// - [`published`](ProjectSystemOutBuilder::published)
    pub fn build(self) -> Result<ProjectSystemOut, BuildError> {
        Ok(ProjectSystemOut {
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            deployment: self.deployment,
            edges: self.edges.ok_or_else(|| BuildError::missing_field("edges"))?,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            generated_automations: self.generated_automations.ok_or_else(|| BuildError::missing_field("generated_automations"))?,
            hidden_count: self.hidden_count.ok_or_else(|| BuildError::missing_field("hidden_count"))?,
            hidden_incident_count: self.hidden_incident_count.ok_or_else(|| BuildError::missing_field("hidden_incident_count"))?,
            incidents: self.incidents.ok_or_else(|| BuildError::missing_field("incidents"))?,
            nodes: self.nodes.ok_or_else(|| BuildError::missing_field("nodes"))?,
            project_asset_id: self.project_asset_id.ok_or_else(|| BuildError::missing_field("project_asset_id"))?,
            published: self.published.ok_or_else(|| BuildError::missing_field("published"))?,
            version: self.version,
        })
    }
}
