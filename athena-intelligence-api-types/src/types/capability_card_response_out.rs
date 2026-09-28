pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// L1: the card an asset broadcasts, its text view and the layered results.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityCardResponseOut {
    /// The card as an agent reads it.
    #[serde(default)]
    pub agent_view: String,
    /// Why anchor values are missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_note: Option<String>,
    /// The layered can-result of every offered action.
    #[serde(default)]
    pub can: Vec<HashMap<String, serde_json::Value>>,
    /// The L1 card.
    #[serde(default)]
    pub card: HashMap<String, serde_json::Value>,
    /// The surface, sender and agent cards.
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
    /// Export option ids.
    #[serde(default)]
    pub export_formats: Vec<String>,
}

impl CapabilityCardResponseOut {
    pub fn builder() -> CapabilityCardResponseOutBuilder {
        <CapabilityCardResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityCardResponseOutBuilder {
    agent_view: Option<String>,
    anchor_note: Option<String>,
    can: Option<Vec<HashMap<String, serde_json::Value>>>,
    card: Option<HashMap<String, serde_json::Value>>,
    context: Option<HashMap<String, serde_json::Value>>,
    export_formats: Option<Vec<String>>,
}

impl CapabilityCardResponseOutBuilder {
    pub fn agent_view(mut self, value: impl Into<String>) -> Self {
        self.agent_view = Some(value.into());
        self
    }

    pub fn anchor_note(mut self, value: impl Into<String>) -> Self {
        self.anchor_note = Some(value.into());
        self
    }

    pub fn can(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.can = Some(value);
        self
    }

    pub fn card(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.card = Some(value);
        self
    }

    pub fn context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.context = Some(value);
        self
    }

    pub fn export_formats(mut self, value: Vec<String>) -> Self {
        self.export_formats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityCardResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_view`](CapabilityCardResponseOutBuilder::agent_view)
    /// - [`can`](CapabilityCardResponseOutBuilder::can)
    /// - [`card`](CapabilityCardResponseOutBuilder::card)
    /// - [`context`](CapabilityCardResponseOutBuilder::context)
    /// - [`export_formats`](CapabilityCardResponseOutBuilder::export_formats)
    pub fn build(self) -> Result<CapabilityCardResponseOut, BuildError> {
        Ok(CapabilityCardResponseOut {
            agent_view: self.agent_view.ok_or_else(|| BuildError::missing_field("agent_view"))?,
            anchor_note: self.anchor_note,
            can: self.can.ok_or_else(|| BuildError::missing_field("can"))?,
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            context: self.context.ok_or_else(|| BuildError::missing_field("context"))?,
            export_formats: self.export_formats.ok_or_else(|| BuildError::missing_field("export_formats"))?,
        })
    }
}
