pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_context
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContextQueryRequest {
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Surface key: spaces (default), slack, sms, email, voice, api, cli, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Simulated policies to overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

impl GetContextQueryRequest {
    pub fn builder() -> GetContextQueryRequestBuilder {
        <GetContextQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContextQueryRequestBuilder {
    agent: Option<String>,
    surface: Option<String>,
    policies: Option<Vec<String>>,
}

impl GetContextQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`GetContextQueryRequest`].
    pub fn build(self) -> Result<GetContextQueryRequest, BuildError> {
        Ok(GetContextQueryRequest {
            agent: self.agent,
            surface: self.surface,
            policies: self.policies,
        })
    }
}

