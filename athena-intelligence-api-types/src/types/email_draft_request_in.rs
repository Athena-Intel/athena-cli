pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailDraftRequestIn {
    /// BCC recipients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// Body content. HTML is supported; plain-text newlines become line breaks. Do not write a signature into it — the account's signature is appended automatically (see `include_signature`).
    #[serde(default)]
    pub body: String,
    /// Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// CC recipients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Optional: Whether to append the sending account's own signature at the end of the body. Leave unset to follow the user's saved preference for this account (which distinguishes new messages from replies) — that is almost always the right choice. Set false only when the user asks for no signature. Never write a signature into the body yourself: the real one is read from the account and appended automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_signature: Option<bool>,
    /// Provider message id (from `GET /tools/email/search`) to reply to, so the draft is threaded under that message. Outlook rejects an id it cannot find (404). Gmail saves the draft *unthreaded* when the referenced message cannot be read, and `reply_to_message_id` in the response tells you which happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
    /// Subject line.
    #[serde(default)]
    pub subject: String,
    /// Recipient email addresses.
    #[serde(default)]
    pub to: Vec<String>,
}

impl EmailDraftRequestIn {
    pub fn builder() -> EmailDraftRequestInBuilder {
        <EmailDraftRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailDraftRequestInBuilder {
    bcc: Option<Vec<String>>,
    body: Option<String>,
    catalog_id: Option<String>,
    cc: Option<Vec<String>>,
    include_signature: Option<bool>,
    reply_to_message_id: Option<String>,
    subject: Option<String>,
    to: Option<Vec<String>>,
}

impl EmailDraftRequestInBuilder {
    pub fn bcc(mut self, value: Vec<String>) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn cc(mut self, value: Vec<String>) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn include_signature(mut self, value: bool) -> Self {
        self.include_signature = Some(value);
        self
    }

    pub fn reply_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.reply_to_message_id = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn to(mut self, value: Vec<String>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailDraftRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](EmailDraftRequestInBuilder::body)
    /// - [`subject`](EmailDraftRequestInBuilder::subject)
    /// - [`to`](EmailDraftRequestInBuilder::to)
    pub fn build(self) -> Result<EmailDraftRequestIn, BuildError> {
        Ok(EmailDraftRequestIn {
            bcc: self.bcc,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            catalog_id: self.catalog_id,
            cc: self.cc,
            include_signature: self.include_signature,
            reply_to_message_id: self.reply_to_message_id,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

