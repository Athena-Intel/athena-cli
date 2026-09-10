pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One message matched by an email search.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailSearchResultOut {
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
    /// Provider message id (Gmail message id, or Microsoft Graph message id for Outlook). This — never an Athena asset id — is what `reply_to_message_id` accepts.
    #[serde(default)]
    pub id: String,
    /// True when the message is an unsent draft sitting in the mailbox. Drafts are never sent or received mail and cannot be replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Short plain-text preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// Subject line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Gmail thread id. Gmail accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// Recipients, comma-separated as the provider reports them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl EmailSearchResultOut {
    pub fn builder() -> EmailSearchResultOutBuilder {
        <EmailSearchResultOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailSearchResultOutBuilder {
    conversation_id: Option<String>,
    date: Option<String>,
    draft_id: Option<String>,
    from: Option<String>,
    id: Option<String>,
    is_draft: Option<bool>,
    snippet: Option<String>,
    subject: Option<String>,
    thread_id: Option<String>,
    to: Option<String>,
}

impl EmailSearchResultOutBuilder {
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

    pub fn snippet(mut self, value: impl Into<String>) -> Self {
        self.snippet = Some(value.into());
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

    /// Consumes the builder and constructs a [`EmailSearchResultOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmailSearchResultOutBuilder::id)
    pub fn build(self) -> Result<EmailSearchResultOut, BuildError> {
        Ok(EmailSearchResultOut {
            conversation_id: self.conversation_id,
            date: self.date,
            draft_id: self.draft_id,
            from: self.from,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_draft: self.is_draft,
            snippet: self.snippet,
            subject: self.subject,
            thread_id: self.thread_id,
            to: self.to,
        })
    }
}
