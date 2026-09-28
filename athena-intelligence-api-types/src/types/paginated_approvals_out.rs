pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One keyset page of the caller's inbox, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedApprovalsOut {
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub items: Vec<ApprovalOut>,
    /// Pass as `before` for the next page; null when there is none
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_before: Option<DateTime<FixedOffset>>,
    /// Pass as `before_id` together with `before`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_before_id: Option<String>,
}

impl PaginatedApprovalsOut {
    pub fn builder() -> PaginatedApprovalsOutBuilder {
        <PaginatedApprovalsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedApprovalsOutBuilder {
    has_more: Option<bool>,
    items: Option<Vec<ApprovalOut>>,
    next_before: Option<DateTime<FixedOffset>>,
    next_before_id: Option<String>,
}

impl PaginatedApprovalsOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ApprovalOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_before = Some(value);
        self
    }

    pub fn next_before_id(mut self, value: impl Into<String>) -> Self {
        self.next_before_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaginatedApprovalsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](PaginatedApprovalsOutBuilder::has_more)
    /// - [`items`](PaginatedApprovalsOutBuilder::items)
    pub fn build(self) -> Result<PaginatedApprovalsOut, BuildError> {
        Ok(PaginatedApprovalsOut {
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_before: self.next_before,
            next_before_id: self.next_before_id,
        })
    }
}
