pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Only what the caller may open comes back; unknown or refused ids are absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceResolveResponseOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<PresenceAssetOut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<PresenceMemberOut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<PresenceProjectOut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<PresenceSessionOut>>,
}

impl PresenceResolveResponseOut {
    pub fn builder() -> PresenceResolveResponseOutBuilder {
        <PresenceResolveResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceResolveResponseOutBuilder {
    assets: Option<Vec<PresenceAssetOut>>,
    members: Option<Vec<PresenceMemberOut>>,
    projects: Option<Vec<PresenceProjectOut>>,
    sessions: Option<Vec<PresenceSessionOut>>,
}

impl PresenceResolveResponseOutBuilder {
    pub fn assets(mut self, value: Vec<PresenceAssetOut>) -> Self {
        self.assets = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<PresenceMemberOut>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn projects(mut self, value: Vec<PresenceProjectOut>) -> Self {
        self.projects = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<PresenceSessionOut>) -> Self {
        self.sessions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PresenceResolveResponseOut`].
    pub fn build(self) -> Result<PresenceResolveResponseOut, BuildError> {
        Ok(PresenceResolveResponseOut {
            assets: self.assets,
            members: self.members,
            projects: self.projects,
            sessions: self.sessions,
        })
    }
}
