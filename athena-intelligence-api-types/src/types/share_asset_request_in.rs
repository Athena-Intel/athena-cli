pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShareAssetRequestIn {
    /// Controls child-sharing when sharing a container asset (folder, collection, or project). true: also share the container's children owned by the current user. false: container only, skip children. null (default): folders share all accessible children; collections and projects are container-only (their cascade is opt-in via true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folder_contents: Option<bool>,
    /// Optional personal message to include in the notification email (max 2000 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether to send email notifications to recipients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    /// List of users to share the asset with. Each entry specifies an email address and the permission level to grant.
    #[serde(default)]
    pub recipients: Vec<ShareRecipient>,
}

impl ShareAssetRequestIn {
    pub fn builder() -> ShareAssetRequestInBuilder {
        <ShareAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareAssetRequestInBuilder {
    include_folder_contents: Option<bool>,
    message: Option<String>,
    notify: Option<bool>,
    recipients: Option<Vec<ShareRecipient>>,
}

impl ShareAssetRequestInBuilder {
    pub fn include_folder_contents(mut self, value: bool) -> Self {
        self.include_folder_contents = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn notify(mut self, value: bool) -> Self {
        self.notify = Some(value);
        self
    }

    pub fn recipients(mut self, value: Vec<ShareRecipient>) -> Self {
        self.recipients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShareAssetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recipients`](ShareAssetRequestInBuilder::recipients)
    pub fn build(self) -> Result<ShareAssetRequestIn, BuildError> {
        Ok(ShareAssetRequestIn {
            include_folder_contents: self.include_folder_contents,
            message: self.message,
            notify: self.notify,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
        })
    }
}

