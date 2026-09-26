pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationApprovalDecideRequestIn {
    /// The edited subject, when the approval allows edits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_payload: Option<HashMap<String, serde_json::Value>>,
    /// A note for the audit row
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// One of the options the approval offered (e.g. approve, reject)
    #[serde(default)]
    pub option: String,
}

impl AutomationApprovalDecideRequestIn {
    pub fn builder() -> AutomationApprovalDecideRequestInBuilder {
        <AutomationApprovalDecideRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationApprovalDecideRequestInBuilder {
    edited_payload: Option<HashMap<String, serde_json::Value>>,
    note: Option<String>,
    option: Option<String>,
}

impl AutomationApprovalDecideRequestInBuilder {
    pub fn edited_payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.edited_payload = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn option(mut self, value: impl Into<String>) -> Self {
        self.option = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationApprovalDecideRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`option`](AutomationApprovalDecideRequestInBuilder::option)
    pub fn build(self) -> Result<AutomationApprovalDecideRequestIn, BuildError> {
        Ok(AutomationApprovalDecideRequestIn {
            edited_payload: self.edited_payload,
            note: self.note,
            option: self.option.ok_or_else(|| BuildError::missing_field("option"))?,
        })
    }
}

