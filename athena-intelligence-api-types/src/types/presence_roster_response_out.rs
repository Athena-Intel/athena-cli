pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of who is online in the workspace, as the caller may see them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PresenceRosterResponseOut {
    /// False only while Athena has no fresh presence data for the workspace yet (it starts observing a workspace when someone asks). people is then empty; ask again in a few seconds.
    #[serde(default)]
    pub available: bool,
    /// People per page, as served.
    #[serde(default)]
    pub limit: i64,
    /// People skipped, as served.
    #[serde(default)]
    pub offset: i64,
    /// Active first, then by name. The agent runs the caller may see come with the last page, outside limit and total_online.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<PresenceRosterPersonOut>>,
    /// People online that match search, across every page. Agent runs are not counted.
    #[serde(default)]
    pub total_online: i64,
}

impl PresenceRosterResponseOut {
    pub fn builder() -> PresenceRosterResponseOutBuilder {
        <PresenceRosterResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceRosterResponseOutBuilder {
    available: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    people: Option<Vec<PresenceRosterPersonOut>>,
    total_online: Option<i64>,
}

impl PresenceRosterResponseOutBuilder {
    pub fn available(mut self, value: bool) -> Self {
        self.available = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn people(mut self, value: Vec<PresenceRosterPersonOut>) -> Self {
        self.people = Some(value);
        self
    }

    pub fn total_online(mut self, value: i64) -> Self {
        self.total_online = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PresenceRosterResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available`](PresenceRosterResponseOutBuilder::available)
    /// - [`limit`](PresenceRosterResponseOutBuilder::limit)
    /// - [`offset`](PresenceRosterResponseOutBuilder::offset)
    /// - [`total_online`](PresenceRosterResponseOutBuilder::total_online)
    pub fn build(self) -> Result<PresenceRosterResponseOut, BuildError> {
        Ok(PresenceRosterResponseOut {
            available: self.available.ok_or_else(|| BuildError::missing_field("available"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            offset: self.offset.ok_or_else(|| BuildError::missing_field("offset"))?,
            people: self.people,
            total_online: self.total_online.ok_or_else(|| BuildError::missing_field("total_online"))?,
        })
    }
}
