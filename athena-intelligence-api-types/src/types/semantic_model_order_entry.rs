pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Object form of an order entry; normalized to a [member, direction] pair.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SemanticModelOrderEntry {
    /// "asc" or "desc"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Measure or dimension to sort by
    #[serde(default)]
    pub member: String,
}

impl SemanticModelOrderEntry {
    pub fn builder() -> SemanticModelOrderEntryBuilder {
        <SemanticModelOrderEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelOrderEntryBuilder {
    direction: Option<String>,
    member: Option<String>,
}

impl SemanticModelOrderEntryBuilder {
    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.direction = Some(value.into());
        self
    }

    pub fn member(mut self, value: impl Into<String>) -> Self {
        self.member = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelOrderEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member`](SemanticModelOrderEntryBuilder::member)
    pub fn build(self) -> Result<SemanticModelOrderEntry, BuildError> {
        Ok(SemanticModelOrderEntry {
            direction: self.direction,
            member: self.member.ok_or_else(|| BuildError::missing_field("member"))?,
        })
    }
}
