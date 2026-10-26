pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What to watch: an asset, a catalog's item, another conversation, or a link.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WatchSubjectInput {
    /// Spreadsheets only: hear only content changes that touch these ranges, 1 to 20. Omit to hear every change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<SubscriptionAnchorInput>>,
    /// The asset to watch (its id, not a version). Name this or session_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// A connection catalog (your Slack connection, a GitHub repository's catalog), in place of asset_id, to narrow to one item in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// With catalog_id or asset_id: one item in the catalog. Slack: a conversation id (C0123ABCD), a thread by its root message's ts (C0123ABCD/1712345678.000100), or a message's permalink. GitHub: a pull request (owner/repo#12, or its URL). ServiceNow: a table (incident) or one record (incident/<sys_id>, or its form URL). SharePoint or OneDrive: a library of the site by its drive id (b!…: every file created in it), a folder in it (b!…/<folder item id>: the files created directly in it), or a folder of the site's default library or the OneDrive by its item id. Omit to hear the whole catalog (a SharePoint site: its default library).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    /// Another conversation to watch, by its thread id, for session.run.completed (never this conversation). Name this or asset_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// A link to what to watch, in place of the ids: a GitHub pull request or issue (https://github.com/owner/repo/pull/123, heard on its repository's catalog as that item) or an Athena asset link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WatchSubjectInput {
    pub fn builder() -> WatchSubjectInputBuilder {
        <WatchSubjectInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WatchSubjectInputBuilder {
    anchors: Option<Vec<SubscriptionAnchorInput>>,
    asset_id: Option<String>,
    catalog_id: Option<String>,
    item: Option<String>,
    session_id: Option<String>,
    url: Option<String>,
}

impl WatchSubjectInputBuilder {
    pub fn anchors(mut self, value: Vec<SubscriptionAnchorInput>) -> Self {
        self.anchors = Some(value);
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn item(mut self, value: impl Into<String>) -> Self {
        self.item = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WatchSubjectInput`].
    pub fn build(self) -> Result<WatchSubjectInput, BuildError> {
        Ok(WatchSubjectInput {
            anchors: self.anchors,
            asset_id: self.asset_id,
            catalog_id: self.catalog_id,
            item: self.item,
            session_id: self.session_id,
            url: self.url,
        })
    }
}
