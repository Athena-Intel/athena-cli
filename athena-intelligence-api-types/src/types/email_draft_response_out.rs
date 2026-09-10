pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of `POST /tools/email/draft`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailDraftResponseOut {
    /// The connected account the draft is in.
    #[serde(default)]
    pub catalog_id: String,
    /// Provider draft id. Open, edit, or send it in the mail client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    /// The address the draft will be sent from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Human-readable outcome, when the provider reports one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Provider message id backing the draft. Gmail accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Account provider as Athena reports it: `gmail` or `outlook` for accounts connected through the Integrations page; `google` or `microsoft365` for directly-connected accounts (read-only for drafts).
    #[serde(default)]
    pub provider: String,
    /// The message this draft replies to, when it is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
    /// Whether the draft was saved.
    #[serde(default)]
    pub success: bool,
    /// Athena email-tracking id, when tracking is enabled on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

impl EmailDraftResponseOut {
    pub fn builder() -> EmailDraftResponseOutBuilder {
        <EmailDraftResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailDraftResponseOutBuilder {
    catalog_id: Option<String>,
    draft_id: Option<String>,
    email_address: Option<String>,
    message: Option<String>,
    message_id: Option<String>,
    provider: Option<String>,
    reply_to_message_id: Option<String>,
    success: Option<bool>,
    tracking_id: Option<String>,
}

impl EmailDraftResponseOutBuilder {
    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn draft_id(mut self, value: impl Into<String>) -> Self {
        self.draft_id = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn reply_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.reply_to_message_id = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn tracking_id(mut self, value: impl Into<String>) -> Self {
        self.tracking_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmailDraftResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_id`](EmailDraftResponseOutBuilder::catalog_id)
    /// - [`provider`](EmailDraftResponseOutBuilder::provider)
    /// - [`success`](EmailDraftResponseOutBuilder::success)
    pub fn build(self) -> Result<EmailDraftResponseOut, BuildError> {
        Ok(EmailDraftResponseOut {
            catalog_id: self.catalog_id.ok_or_else(|| BuildError::missing_field("catalog_id"))?,
            draft_id: self.draft_id,
            email_address: self.email_address,
            message: self.message,
            message_id: self.message_id,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            reply_to_message_id: self.reply_to_message_id,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            tracking_id: self.tracking_id,
        })
    }
}
