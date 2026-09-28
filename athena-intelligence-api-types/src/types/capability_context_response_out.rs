pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// R2: the surface, sender and agent cards that open every context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityContextResponseOut {
    /// The agent card.
    #[serde(default)]
    pub agent: HashMap<String, serde_json::Value>,
    /// The three cards as an agent reads them.
    #[serde(default)]
    pub agent_view: String,
    /// Approximate token cost.
    #[serde(default)]
    pub estimated_tokens: i64,
    /// The sender card.
    #[serde(default)]
    pub sender: HashMap<String, serde_json::Value>,
    /// The surface card.
    #[serde(default)]
    pub surface: HashMap<String, serde_json::Value>,
}

impl CapabilityContextResponseOut {
    pub fn builder() -> CapabilityContextResponseOutBuilder {
        <CapabilityContextResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityContextResponseOutBuilder {
    agent: Option<HashMap<String, serde_json::Value>>,
    agent_view: Option<String>,
    estimated_tokens: Option<i64>,
    sender: Option<HashMap<String, serde_json::Value>>,
    surface: Option<HashMap<String, serde_json::Value>>,
}

impl CapabilityContextResponseOutBuilder {
    pub fn agent(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn agent_view(mut self, value: impl Into<String>) -> Self {
        self.agent_view = Some(value.into());
        self
    }

    pub fn estimated_tokens(mut self, value: i64) -> Self {
        self.estimated_tokens = Some(value);
        self
    }

    pub fn sender(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn surface(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityContextResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](CapabilityContextResponseOutBuilder::agent)
    /// - [`agent_view`](CapabilityContextResponseOutBuilder::agent_view)
    /// - [`estimated_tokens`](CapabilityContextResponseOutBuilder::estimated_tokens)
    /// - [`sender`](CapabilityContextResponseOutBuilder::sender)
    /// - [`surface`](CapabilityContextResponseOutBuilder::surface)
    pub fn build(self) -> Result<CapabilityContextResponseOut, BuildError> {
        Ok(CapabilityContextResponseOut {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
            agent_view: self.agent_view.ok_or_else(|| BuildError::missing_field("agent_view"))?,
            estimated_tokens: self.estimated_tokens.ok_or_else(|| BuildError::missing_field("estimated_tokens"))?,
            sender: self.sender.ok_or_else(|| BuildError::missing_field("sender"))?,
            surface: self.surface.ok_or_else(|| BuildError::missing_field("surface"))?,
        })
    }
}
