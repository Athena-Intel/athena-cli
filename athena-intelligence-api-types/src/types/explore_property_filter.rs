pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One per-type property filter (``AssetSetSpec.properties``).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExplorePropertyFilter {
    /// A date property only: on or after this date or time (ISO 8601; UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A date property only: before this date or time (ISO 8601; UTC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Leave out the assets holding any of the values instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    /// The property id, e.g. meeting.joined (listed in the tool's description).
    #[serde(default)]
    pub id: String,
    /// Values, any of which matches: an option value (zoom, failed, true, 15m_30m), a month (YYYY-MM) for a date property, an agent or model id or name, or for a person property an email, a user id, a name, or 'me'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl ExplorePropertyFilter {
    pub fn builder() -> ExplorePropertyFilterBuilder {
        <ExplorePropertyFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExplorePropertyFilterBuilder {
    after: Option<String>,
    before: Option<String>,
    exclude: Option<bool>,
    id: Option<String>,
    values: Option<Vec<String>>,
}

impl ExplorePropertyFilterBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExplorePropertyFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExplorePropertyFilterBuilder::id)
    pub fn build(self) -> Result<ExplorePropertyFilter, BuildError> {
        Ok(ExplorePropertyFilter {
            after: self.after,
            before: self.before,
            exclude: self.exclude,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            values: self.values,
        })
    }
}
