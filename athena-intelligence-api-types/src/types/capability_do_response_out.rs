pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What ``do`` did: ran the action, or why it ran nothing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilityDoResponseOut {
    #[serde(default)]
    pub action: String,
    /// The card changed and the action or inputs no longer fit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_changed: Option<HashMap<String, serde_json::Value>>,
    /// What happened, for whoever reads it.
    #[serde(default)]
    pub message: String,
    /// The layered can-result the decision was made on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HashMap<String, serde_json::Value>>,
    /// done or failed (the tool ran), blocked, needs_approval, capability_changed or not_runnable (nothing ran).
    pub status: CapabilityDoResponseOutStatus,
    /// The tool call: tool_id, success, result, error and duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<HashMap<String, serde_json::Value>>,
    /// The way in the can-result chose: a tool id, route:<key> or ui.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

impl CapabilityDoResponseOut {
    pub fn builder() -> CapabilityDoResponseOutBuilder {
        <CapabilityDoResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityDoResponseOutBuilder {
    action: Option<String>,
    capability_changed: Option<HashMap<String, serde_json::Value>>,
    message: Option<String>,
    result: Option<HashMap<String, serde_json::Value>>,
    status: Option<CapabilityDoResponseOutStatus>,
    tool: Option<HashMap<String, serde_json::Value>>,
    via: Option<String>,
}

impl CapabilityDoResponseOutBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn capability_changed(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.capability_changed = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    pub fn status(mut self, value: CapabilityDoResponseOutStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn via(mut self, value: impl Into<String>) -> Self {
        self.via = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilityDoResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CapabilityDoResponseOutBuilder::action)
    /// - [`message`](CapabilityDoResponseOutBuilder::message)
    /// - [`status`](CapabilityDoResponseOutBuilder::status)
    pub fn build(self) -> Result<CapabilityDoResponseOut, BuildError> {
        Ok(CapabilityDoResponseOut {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            capability_changed: self.capability_changed,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            result: self.result,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool: self.tool,
            via: self.via,
        })
    }
}
