pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which page of the roster to read. Every field is optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceRosterRequestIn {
    /// People per page. A value above 100 is served as 100, one below 1 as 1; the answer says what was served.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// People to skip. A negative value is served as 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Keep only people whose name contains this, whatever its case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl PresenceRosterRequestIn {
    pub fn builder() -> PresenceRosterRequestInBuilder {
        <PresenceRosterRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceRosterRequestInBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    search: Option<String>,
}

impl PresenceRosterRequestInBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceRosterRequestIn`].
    pub fn build(self) -> Result<PresenceRosterRequestIn, BuildError> {
        Ok(PresenceRosterRequestIn {
            limit: self.limit,
            offset: self.offset,
            search: self.search,
        })
    }
}
