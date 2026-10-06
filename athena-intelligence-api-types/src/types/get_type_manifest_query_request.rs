pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_type_manifest
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTypeManifestQueryRequest {
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Simulated policies to overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// Simulated sender access: view, edit, owner or staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_access: Option<String>,
}

impl GetTypeManifestQueryRequest {
    pub fn builder() -> GetTypeManifestQueryRequestBuilder {
        <GetTypeManifestQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTypeManifestQueryRequestBuilder {
    agent: Option<String>,
    surface: Option<String>,
    policies: Option<Vec<String>>,
    assumed_access: Option<String>,
}

impl GetTypeManifestQueryRequestBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn surface(mut self, value: impl Into<String>) -> Self {
        self.surface = Some(value.into());
        self
    }

    pub fn policies(mut self, value: Vec<String>) -> Self {
        self.policies = Some(value);
        self
    }

    pub fn assumed_access(mut self, value: impl Into<String>) -> Self {
        self.assumed_access = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTypeManifestQueryRequest`].
    pub fn build(self) -> Result<GetTypeManifestQueryRequest, BuildError> {
        Ok(GetTypeManifestQueryRequest {
            agent: self.agent,
            surface: self.surface,
            policies: self.policies,
            assumed_access: self.assumed_access,
        })
    }
}

