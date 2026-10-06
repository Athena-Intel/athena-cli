pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for can
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CanQueryRequest {
    /// Action name.
    #[serde(default)]
    pub action: String,
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Surface key: cli (default; these routes' client), api, spaces, slack, sms, email, voice, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Simulated policies to overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// The manifest_version of the card the caller acted from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_version: Option<String>,
}

impl CanQueryRequest {
    pub fn builder() -> CanQueryRequestBuilder {
        <CanQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CanQueryRequestBuilder {
    action: Option<String>,
    agent: Option<String>,
    surface: Option<String>,
    policies: Option<Vec<String>>,
    card_version: Option<String>,
}

impl CanQueryRequestBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
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

    pub fn card_version(mut self, value: impl Into<String>) -> Self {
        self.card_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CanQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CanQueryRequestBuilder::action)
    pub fn build(self) -> Result<CanQueryRequest, BuildError> {
        Ok(CanQueryRequest {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            agent: self.agent,
            surface: self.surface,
            policies: self.policies,
            card_version: self.card_version,
        })
    }
}

