pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One message's headers and plain-text body.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailMessageResponseOut {
    /// The message body as plain text, HTML converted. At most 50,000 characters; see `truncated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    /// The connected account the message was read from.
    #[serde(default)]
    pub catalog_id: String,
    /// Cc recipients, comma-separated as the provider reports them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    /// Microsoft Graph conversation id. Outlook accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Sent or received timestamp, in the provider's format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Provider draft id, when the provider exposes one for a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    /// Sender, as the provider reports it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Provider message id, as `GET /tools/email/search` returns it and `reply_to_message_id` accepts.
    #[serde(default)]
    pub id: String,
    /// True when the message is an unsent draft sitting in the mailbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Account provider as Athena reports it: `gmail` or `outlook` for accounts connected through the Integrations page; `google` or `microsoft365` for directly-connected accounts.
    #[serde(default)]
    pub provider: String,
    /// Subject line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Gmail thread id. Gmail accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// Recipients, comma-separated as the provider reports them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// True when `body_text` was cut to its maximum length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

impl EmailMessageResponseOut {
    pub fn builder() -> EmailMessageResponseOutBuilder {
        <EmailMessageResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailMessageResponseOutBuilder {
    body_text: Option<String>,
    catalog_id: Option<String>,
    cc: Option<String>,
    conversation_id: Option<String>,
    date: Option<String>,
    draft_id: Option<String>,
    from: Option<String>,
    id: Option<String>,
    is_draft: Option<bool>,
    provider: Option<String>,
    subject: Option<String>,
    thread_id: Option<String>,
    to: Option<String>,
    truncated: Option<bool>,
}

impl EmailMessageResponseOutBuilder {
    pub fn body_text(mut self, value: impl Into<String>) -> Self {
        self.body_text = Some(value.into());
        self
    }

    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn cc(mut self, value: impl Into<String>) -> Self {
        self.cc = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn draft_id(mut self, value: impl Into<String>) -> Self {
        self.draft_id = Some(value.into());
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_draft(mut self, value: bool) -> Self {
        self.is_draft = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailMessageResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_id`](EmailMessageResponseOutBuilder::catalog_id)
    /// - [`id`](EmailMessageResponseOutBuilder::id)
    /// - [`provider`](EmailMessageResponseOutBuilder::provider)
    pub fn build(self) -> Result<EmailMessageResponseOut, BuildError> {
        Ok(EmailMessageResponseOut {
            body_text: self.body_text,
            catalog_id: self.catalog_id.ok_or_else(|| BuildError::missing_field("catalog_id"))?,
            cc: self.cc,
            conversation_id: self.conversation_id,
            date: self.date,
            draft_id: self.draft_id,
            from: self.from,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_draft: self.is_draft,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            subject: self.subject,
            thread_id: self.thread_id,
            to: self.to,
            truncated: self.truncated,
        })
    }
}
