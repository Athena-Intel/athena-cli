pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApprovalDecideRequestIn {
    /// The edited subject, when the approval allows edits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_payload: Option<HashMap<String, serde_json::Value>>,
    /// A note recorded with the decision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// One of the options the approval offered (e.g. approve, reject)
    #[serde(default)]
    pub option: String,
    /// A standing-grant request; refused with detail.reason standing_not_allowed unless the approval's gate allows standing grants, and only with an approving option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing: Option<StandingGrantIn>,
}

impl ApprovalDecideRequestIn {
    pub fn builder() -> ApprovalDecideRequestInBuilder {
        <ApprovalDecideRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalDecideRequestInBuilder {
    edited_payload: Option<HashMap<String, serde_json::Value>>,
    note: Option<String>,
    option: Option<String>,
    standing: Option<StandingGrantIn>,
}

impl ApprovalDecideRequestInBuilder {
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

    pub fn standing(mut self, value: StandingGrantIn) -> Self {
        self.standing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApprovalDecideRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`option`](ApprovalDecideRequestInBuilder::option)
    pub fn build(self) -> Result<ApprovalDecideRequestIn, BuildError> {
        Ok(ApprovalDecideRequestIn {
            edited_payload: self.edited_payload,
            note: self.note,
            option: self.option.ok_or_else(|| BuildError::missing_field("option"))?,
            standing: self.standing,
        })
    }
}

