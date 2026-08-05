pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_activity_delta
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetActivityDeltaQueryRequest {
    /// Start clock, from an activity item's from_clock.
    #[serde(default)]
    pub from: i64,
    /// End clock, from the same activity item's to_clock.
    #[serde(default)]
    pub to: i64,
}

impl GetActivityDeltaQueryRequest {
    pub fn builder() -> GetActivityDeltaQueryRequestBuilder {
        <GetActivityDeltaQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetActivityDeltaQueryRequestBuilder {
    from: Option<i64>,
    to: Option<i64>,
}

impl GetActivityDeltaQueryRequestBuilder {
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetActivityDeltaQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](GetActivityDeltaQueryRequestBuilder::from)
    /// - [`to`](GetActivityDeltaQueryRequestBuilder::to)
    pub fn build(self) -> Result<GetActivityDeltaQueryRequest, BuildError> {
        Ok(GetActivityDeltaQueryRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}

