pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryAuditTrailInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// A conversation's thread id, or `current` for the one this runs in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    /// Rows as JSON, or as CSV text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<HistoryAuditTrailInputFormat>,
    /// When: today, yesterday, this week, last 7 days, since 2026-10-28, 2026-10-28 (that day) or 2026-10-28..2026-10-30. Times are UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    /// Where: a cell or range (Sheet1!B6, 'Q3 Plan'!B2:F40), a sheet (sheet:Sheet1), a slide (slide 7), or text to look for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<String>,
    /// Who: everyone (default), me, everyone-but-me, athena (any agent), people, or a person's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who: Option<String>,
}

impl HistoryAuditTrailInput {
    pub fn builder() -> HistoryAuditTrailInputBuilder {
        <HistoryAuditTrailInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryAuditTrailInputBuilder {
    asset_id: Option<String>,
    conversation: Option<String>,
    format: Option<HistoryAuditTrailInputFormat>,
    when: Option<String>,
    r#where: Option<String>,
    who: Option<String>,
}

impl HistoryAuditTrailInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn conversation(mut self, value: impl Into<String>) -> Self {
        self.conversation = Some(value.into());
        self
    }

    pub fn format(mut self, value: HistoryAuditTrailInputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn when(mut self, value: impl Into<String>) -> Self {
        self.when = Some(value.into());
        self
    }

    pub fn r#where(mut self, value: impl Into<String>) -> Self {
        self.r#where = Some(value.into());
        self
    }

    pub fn who(mut self, value: impl Into<String>) -> Self {
        self.who = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryAuditTrailInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryAuditTrailInputBuilder::asset_id)
    pub fn build(self) -> Result<HistoryAuditTrailInput, BuildError> {
        Ok(HistoryAuditTrailInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            conversation: self.conversation,
            format: self.format,
            when: self.when,
            r#where: self.r#where,
            who: self.who,
        })
    }
}

