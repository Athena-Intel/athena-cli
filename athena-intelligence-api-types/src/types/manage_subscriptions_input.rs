pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ManageSubscriptionsInput {
    /// snooze: the new fire time as a delay from now (15m, 8h, 2d).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// list: only subscriptions on this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// snooze: the new fire time (RFC 3339).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// list, events: the next_cursor of the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// renew: how long it lives from now (5m to 7d; to 90d with until or for a cron).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// list: also cancelled, completed and expired ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_closed: Option<bool>,
    /// list: your subscriptions (this conversation's and your own elsewhere). show: one of them. events: what one heard, without ending your turn (by default what is queued, oldest first). cancel, pause, resume: as named; cancel one once the task it waited for is done. renew: give one a new lifetime (expires). snooze: move a schedule's next fire (at or after).
    pub op: ManageSubscriptionsInputOp,
    /// events: only those queued and not yet delivered (the default); false lists everything it heard, delivered or skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// The asub_... id; every op but list names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl ManageSubscriptionsInput {
    pub fn builder() -> ManageSubscriptionsInputBuilder {
        <ManageSubscriptionsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManageSubscriptionsInputBuilder {
    after: Option<String>,
    asset_id: Option<String>,
    at: Option<String>,
    cursor: Option<String>,
    expires: Option<String>,
    include_closed: Option<bool>,
    op: Option<ManageSubscriptionsInputOp>,
    pending: Option<bool>,
    subscription_id: Option<String>,
}

impl ManageSubscriptionsInputBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn at(mut self, value: impl Into<String>) -> Self {
        self.at = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(value.into());
        self
    }

    pub fn include_closed(mut self, value: bool) -> Self {
        self.include_closed = Some(value);
        self
    }

    pub fn op(mut self, value: ManageSubscriptionsInputOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn pending(mut self, value: bool) -> Self {
        self.pending = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ManageSubscriptionsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`op`](ManageSubscriptionsInputBuilder::op)
    pub fn build(self) -> Result<ManageSubscriptionsInput, BuildError> {
        Ok(ManageSubscriptionsInput {
            after: self.after,
            asset_id: self.asset_id,
            at: self.at,
            cursor: self.cursor,
            expires: self.expires,
            include_closed: self.include_closed,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            pending: self.pending,
            subscription_id: self.subscription_id,
        })
    }
}

