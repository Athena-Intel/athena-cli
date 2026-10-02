pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The caller's approved-recipient policy for agent-sent email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeEmailPreferencesResponseOut {
    /// Domains the agent may email anyone at, as bare lowercase domains (`acme.com`). Match a recipient's domain against this list.
    #[serde(default)]
    pub approved_domains: Vec<String>,
    /// Specific addresses the agent may email, lowercase (`user@acme.com`). Match the whole recipient address against this list.
    #[serde(default)]
    pub approved_emails: Vec<String>,
    /// Free-text guidance the user wants followed whenever an agent drafts email on their behalf; null when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instruction: Option<String>,
    /// Where the user edits this policy (Settings → Email & Meetings). Send the user here when a recipient is not approved; the policy cannot be changed through this API.
    #[serde(default)]
    pub manage_url: String,
}

impl MeEmailPreferencesResponseOut {
    pub fn builder() -> MeEmailPreferencesResponseOutBuilder {
        <MeEmailPreferencesResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeEmailPreferencesResponseOutBuilder {
    approved_domains: Option<Vec<String>>,
    approved_emails: Option<Vec<String>>,
    custom_instruction: Option<String>,
    manage_url: Option<String>,
}

impl MeEmailPreferencesResponseOutBuilder {
    pub fn approved_domains(mut self, value: Vec<String>) -> Self {
        self.approved_domains = Some(value);
        self
    }

    pub fn approved_emails(mut self, value: Vec<String>) -> Self {
        self.approved_emails = Some(value);
        self
    }

    pub fn custom_instruction(mut self, value: impl Into<String>) -> Self {
        self.custom_instruction = Some(value.into());
        self
    }

    pub fn manage_url(mut self, value: impl Into<String>) -> Self {
        self.manage_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MeEmailPreferencesResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approved_domains`](MeEmailPreferencesResponseOutBuilder::approved_domains)
    /// - [`approved_emails`](MeEmailPreferencesResponseOutBuilder::approved_emails)
    /// - [`manage_url`](MeEmailPreferencesResponseOutBuilder::manage_url)
    pub fn build(self) -> Result<MeEmailPreferencesResponseOut, BuildError> {
        Ok(MeEmailPreferencesResponseOut {
            approved_domains: self.approved_domains.ok_or_else(|| BuildError::missing_field("approved_domains"))?,
            approved_emails: self.approved_emails.ok_or_else(|| BuildError::missing_field("approved_emails"))?,
            custom_instruction: self.custom_instruction,
            manage_url: self.manage_url.ok_or_else(|| BuildError::missing_field("manage_url"))?,
        })
    }
}
