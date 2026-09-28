pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A can-result, or ``capability_changed`` with a fresh card (R17).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityCanResponseOut {
    /// The result as an agent reads it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_view: Option<String>,
    /// The card changed and the action or inputs no longer fit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_changed: Option<HashMap<String, serde_json::Value>>,
    /// The layered can-result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HashMap<String, serde_json::Value>>,
}

impl CapabilityCanResponseOut {
    pub fn builder() -> CapabilityCanResponseOutBuilder {
        <CapabilityCanResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityCanResponseOutBuilder {
    agent_view: Option<String>,
    capability_changed: Option<HashMap<String, serde_json::Value>>,
    result: Option<HashMap<String, serde_json::Value>>,
}

impl CapabilityCanResponseOutBuilder {
    pub fn agent_view(mut self, value: impl Into<String>) -> Self {
        self.agent_view = Some(value.into());
        self
    }

    pub fn capability_changed(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.capability_changed = Some(value);
        self
    }

    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityCanResponseOut`].
    pub fn build(self) -> Result<CapabilityCanResponseOut, BuildError> {
        Ok(CapabilityCanResponseOut {
            agent_view: self.agent_view,
            capability_changed: self.capability_changed,
            result: self.result,
        })
    }
}
