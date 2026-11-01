pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryUndoChangesInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// Undo Athena's edits for you in this conversation. A conversation's thread id, or `current` for the one this runs in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    /// List the edits that would be undone without changing anything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    /// When: today, yesterday, this week, last 7 days, since 2026-10-28, 2026-10-28 (that day) or 2026-10-28..2026-10-30. Times are UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    /// athena (Athena's edits for you; the default with --conversation) or me (edits you made yourself).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who: Option<String>,
}

impl HistoryUndoChangesInput {
    pub fn builder() -> HistoryUndoChangesInputBuilder {
        <HistoryUndoChangesInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryUndoChangesInputBuilder {
    asset_id: Option<String>,
    conversation: Option<String>,
    preview: Option<bool>,
    when: Option<String>,
    who: Option<String>,
}

impl HistoryUndoChangesInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn conversation(mut self, value: impl Into<String>) -> Self {
        self.conversation = Some(value.into());
        self
    }

    pub fn preview(mut self, value: bool) -> Self {
        self.preview = Some(value);
        self
    }

    pub fn when(mut self, value: impl Into<String>) -> Self {
        self.when = Some(value.into());
        self
    }

    pub fn who(mut self, value: impl Into<String>) -> Self {
        self.who = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryUndoChangesInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryUndoChangesInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryUndoChangesInput, BuildError> {
        Ok(HistoryUndoChangesInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            conversation: self.conversation,
            preview: self.preview,
            when: self.when,
            who: self.who,
        })
    }
}

