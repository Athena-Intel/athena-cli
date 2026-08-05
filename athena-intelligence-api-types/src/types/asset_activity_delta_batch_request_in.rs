pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetActivityDeltaBatchRequestIn {
    /// Activity clock ranges to diff, at most 25 per call. Results are returned in request order.
    #[serde(default)]
    pub ranges: Vec<ActivityClockRangeIn>,
}

impl AssetActivityDeltaBatchRequestIn {
    pub fn builder() -> AssetActivityDeltaBatchRequestInBuilder {
        <AssetActivityDeltaBatchRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetActivityDeltaBatchRequestInBuilder {
    ranges: Option<Vec<ActivityClockRangeIn>>,
}

impl AssetActivityDeltaBatchRequestInBuilder {
    pub fn ranges(mut self, value: Vec<ActivityClockRangeIn>) -> Self {
        self.ranges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetActivityDeltaBatchRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ranges`](AssetActivityDeltaBatchRequestInBuilder::ranges)
    pub fn build(self) -> Result<AssetActivityDeltaBatchRequestIn, BuildError> {
        Ok(AssetActivityDeltaBatchRequestIn {
            ranges: self.ranges.ok_or_else(|| BuildError::missing_field("ranges"))?,
        })
    }
}

