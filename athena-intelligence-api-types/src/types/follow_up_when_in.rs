pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When a follow-up fires: exactly one of an instant, a delay or an event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FollowUpWhenIn {
    /// A delay from now, e.g. 14d, 4h or 30m
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The instant to follow up at (RFC 3339; UTC without an offset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<DateTime<FixedOffset>>,
    /// An event type to follow up on its first occurrence (`github.pull_request.merged`, a `custom.*` type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// With `event_type`: a CEL condition the event must meet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

impl FollowUpWhenIn {
    pub fn builder() -> FollowUpWhenInBuilder {
        <FollowUpWhenInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FollowUpWhenInBuilder {
    after: Option<String>,
    at: Option<DateTime<FixedOffset>>,
    event_type: Option<String>,
    r#match: Option<String>,
}

impl FollowUpWhenInBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.at = Some(value);
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn r#match(mut self, value: impl Into<String>) -> Self {
        self.r#match = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FollowUpWhenIn`].
    pub fn build(self) -> Result<FollowUpWhenIn, BuildError> {
        Ok(FollowUpWhenIn {
            after: self.after,
            at: self.at,
            event_type: self.event_type,
            r#match: self.r#match,
        })
    }
}
