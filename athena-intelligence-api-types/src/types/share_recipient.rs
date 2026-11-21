pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single recipient for asset sharing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShareRecipient {
    /// Email address of the user to share with
    #[serde(default)]
    pub email: String,
    /// Permission level to grant: 'view' or 'edit'. On an asset type that publishes (documents, presentations, Word documents, spreadsheets, interfaces, agents, skills, object types and AOPs), also 'view_drafts' (reads the published asset's draft, nothing more) or 'publish' (reads the draft and publishes it, never edits it); any other type refuses those two with 422.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<SharePermissionType>,
}

impl ShareRecipient {
    pub fn builder() -> ShareRecipientBuilder {
        <ShareRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareRecipientBuilder {
    email: Option<String>,
    permission: Option<SharePermissionType>,
}

impl ShareRecipientBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn permission(mut self, value: SharePermissionType) -> Self {
        self.permission = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShareRecipient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ShareRecipientBuilder::email)
    pub fn build(self) -> Result<ShareRecipient, BuildError> {
        Ok(ShareRecipient {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            permission: self.permission,
        })
    }
}
