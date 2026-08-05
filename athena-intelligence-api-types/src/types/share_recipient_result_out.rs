pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result for a single share recipient.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShareRecipientResultOut {
    /// Email address of the recipient
    #[serde(default)]
    pub email: String,
    /// Error message if sharing failed for this recipient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Permission level granted ('view' or 'edit')
    #[serde(default)]
    pub permission: String,
    /// Whether sharing was successful for this recipient
    #[serde(default)]
    pub success: bool,
}

impl ShareRecipientResultOut {
    pub fn builder() -> ShareRecipientResultOutBuilder {
        <ShareRecipientResultOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareRecipientResultOutBuilder {
    email: Option<String>,
    error: Option<String>,
    permission: Option<String>,
    success: Option<bool>,
}

impl ShareRecipientResultOutBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn permission(mut self, value: impl Into<String>) -> Self {
        self.permission = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShareRecipientResultOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ShareRecipientResultOutBuilder::email)
    /// - [`permission`](ShareRecipientResultOutBuilder::permission)
    /// - [`success`](ShareRecipientResultOutBuilder::success)
    pub fn build(self) -> Result<ShareRecipientResultOut, BuildError> {
        Ok(ShareRecipientResultOut {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            error: self.error,
            permission: self.permission.ok_or_else(|| BuildError::missing_field("permission"))?,
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
