pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for describe
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeQueryRequest {
    /// Action name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Event name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Simulated policies to overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

impl DescribeQueryRequest {
    pub fn builder() -> DescribeQueryRequestBuilder {
        <DescribeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeQueryRequestBuilder {
    action: Option<String>,
    event: Option<String>,
    agent: Option<String>,
    surface: Option<String>,
    policies: Option<Vec<String>>,
}

impl DescribeQueryRequestBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`DescribeQueryRequest`].
    pub fn build(self) -> Result<DescribeQueryRequest, BuildError> {
        Ok(DescribeQueryRequest {
            action: self.action,
            event: self.event,
            agent: self.agent,
            surface: self.surface,
            policies: self.policies,
        })
    }
}

