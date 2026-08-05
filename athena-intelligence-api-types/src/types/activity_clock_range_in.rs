pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One activity item's clock range, from the activity endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivityClockRangeIn {
    /// Start clock, from an activity item's from_clock.
    #[serde(default)]
    pub from: i64,
    /// End clock, from the same activity item's to_clock.
    #[serde(default)]
    pub to: i64,
}

impl ActivityClockRangeIn {
    pub fn builder() -> ActivityClockRangeInBuilder {
        <ActivityClockRangeInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityClockRangeInBuilder {
    from: Option<i64>,
    to: Option<i64>,
}

impl ActivityClockRangeInBuilder {
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActivityClockRangeIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](ActivityClockRangeInBuilder::from)
    /// - [`to`](ActivityClockRangeInBuilder::to)
    pub fn build(self) -> Result<ActivityClockRangeIn, BuildError> {
        Ok(ActivityClockRangeIn {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
