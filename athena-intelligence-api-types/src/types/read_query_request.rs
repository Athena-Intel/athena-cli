pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for read
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadQueryRequest {
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Simulated policies to overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// List blocked action names instead of a count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocked: Option<bool>,
    /// Read real sheet and slide ids into the cite block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_anchors: Option<bool>,
}

impl ReadQueryRequest {
    pub fn builder() -> ReadQueryRequestBuilder {
        <ReadQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadQueryRequestBuilder {
    agent: Option<String>,
    surface: Option<String>,
    policies: Option<Vec<String>>,
    include_blocked: Option<bool>,
    resolve_anchors: Option<bool>,
}

impl ReadQueryRequestBuilder {
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

    pub fn include_blocked(mut self, value: bool) -> Self {
        self.include_blocked = Some(value);
        self
    }

    pub fn resolve_anchors(mut self, value: bool) -> Self {
        self.resolve_anchors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadQueryRequest`].
    pub fn build(self) -> Result<ReadQueryRequest, BuildError> {
        Ok(ReadQueryRequest {
            agent: self.agent,
            surface: self.surface,
            policies: self.policies,
            include_blocked: self.include_blocked,
            resolve_anchors: self.resolve_anchors,
        })
    }
}

