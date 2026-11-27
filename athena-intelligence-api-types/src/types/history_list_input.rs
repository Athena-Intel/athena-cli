pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryListInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// A conversation's thread id, or `current` for the one this runs in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    /// Also describe what each group changed (slower: diffs each group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<bool>,
    /// Most groups of edits to return (1 to 100; any other number is brought into that range).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// When: today, yesterday, this week, last 7 days, since 2026-10-28, 2026-10-28 (that day) or 2026-10-28..2026-10-30. Times are UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    /// Who: everyone (default), me, everyone-but-me, athena (any agent), people, or a person's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who: Option<String>,
}

impl HistoryListInput {
    pub fn builder() -> HistoryListInputBuilder {
        <HistoryListInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryListInputBuilder {
    asset_id: Option<String>,
    conversation: Option<String>,
    details: Option<bool>,
    limit: Option<i64>,
    when: Option<String>,
    who: Option<String>,
}

impl HistoryListInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn conversation(mut self, value: impl Into<String>) -> Self {
        self.conversation = Some(value.into());
        self
    }

    pub fn details(mut self, value: bool) -> Self {
        self.details = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

    /// Consumes the builder and constructs a [`HistoryListInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryListInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryListInput, BuildError> {
        Ok(HistoryListInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            conversation: self.conversation,
            details: self.details,
            limit: self.limit,
            when: self.when,
            who: self.who,
        })
    }
}

