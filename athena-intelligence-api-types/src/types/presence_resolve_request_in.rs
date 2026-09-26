pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceResolveRequestIn {
    /// Asset guids from presence contexts of type asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<String>>,
    /// Also list the workspace's members (name, email, avatar) so the app can name people by their attested user id rather than the self-reported slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_members: Option<bool>,
    /// Also list every project the caller may open in this workspace, each with the linked documents the caller may open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_projects: Option<bool>,
    /// Project guids from coarse presence contexts or from a visible context's projects annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    /// Thread ids from presence contexts of type session (what an open chat publishes). Each resolves to the workspace's session asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_ids: Option<Vec<String>>,
}

impl PresenceResolveRequestIn {
    pub fn builder() -> PresenceResolveRequestInBuilder {
        <PresenceResolveRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceResolveRequestInBuilder {
    asset_ids: Option<Vec<String>>,
    include_members: Option<bool>,
    include_projects: Option<bool>,
    project_ids: Option<Vec<String>>,
    session_ids: Option<Vec<String>>,
}

impl PresenceResolveRequestInBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn include_members(mut self, value: bool) -> Self {
        self.include_members = Some(value);
        self
    }

    pub fn include_projects(mut self, value: bool) -> Self {
        self.include_projects = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    pub fn session_ids(mut self, value: Vec<String>) -> Self {
        self.session_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PresenceResolveRequestIn`].
    pub fn build(self) -> Result<PresenceResolveRequestIn, BuildError> {
        Ok(PresenceResolveRequestIn {
            asset_ids: self.asset_ids,
            include_members: self.include_members,
            include_projects: self.include_projects,
            project_ids: self.project_ids,
            session_ids: self.session_ids,
        })
    }
}

