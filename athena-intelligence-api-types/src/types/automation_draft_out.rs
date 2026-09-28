pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The draft definition and where it was read from.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationDraftOut {
    /// The draft definition document, or null when there is none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<HashMap<String, serde_json::Value>>,
    /// Why the draft could not be read, when it could not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// 'snapshot:latest' when read from the @latest snapshot, 'live' when read from the live Keryx document, null when unreadable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AutomationDraftOutSource>,
}

impl AutomationDraftOut {
    pub fn builder() -> AutomationDraftOutBuilder {
        <AutomationDraftOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationDraftOutBuilder {
    definition: Option<HashMap<String, serde_json::Value>>,
    note: Option<String>,
    source: Option<AutomationDraftOutSource>,
}

impl AutomationDraftOutBuilder {
    pub fn definition(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn source(mut self, value: AutomationDraftOutSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationDraftOut`].
    pub fn build(self) -> Result<AutomationDraftOut, BuildError> {
        Ok(AutomationDraftOut {
            definition: self.definition,
            note: self.note,
            source: self.source,
        })
    }
}
