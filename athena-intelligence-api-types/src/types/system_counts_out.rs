pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Visible nodes by state; ``unchecked`` have no reading yet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemCountsOut {
    #[serde(default)]
    pub failing: i64,
    #[serde(default)]
    pub fresh: i64,
    #[serde(default)]
    pub stale: i64,
    #[serde(default)]
    pub unavailable: i64,
    #[serde(default)]
    pub unchecked: i64,
    #[serde(default)]
    pub unknown: i64,
}

impl SystemCountsOut {
    pub fn builder() -> SystemCountsOutBuilder {
        <SystemCountsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemCountsOutBuilder {
    failing: Option<i64>,
    fresh: Option<i64>,
    stale: Option<i64>,
    unavailable: Option<i64>,
    unchecked: Option<i64>,
    unknown: Option<i64>,
}

impl SystemCountsOutBuilder {
    pub fn failing(mut self, value: i64) -> Self {
        self.failing = Some(value);
        self
    }

    pub fn fresh(mut self, value: i64) -> Self {
        self.fresh = Some(value);
        self
    }

    pub fn stale(mut self, value: i64) -> Self {
        self.stale = Some(value);
        self
    }

    pub fn unavailable(mut self, value: i64) -> Self {
        self.unavailable = Some(value);
        self
    }

    pub fn unchecked(mut self, value: i64) -> Self {
        self.unchecked = Some(value);
        self
    }

    pub fn unknown(mut self, value: i64) -> Self {
        self.unknown = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemCountsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failing`](SystemCountsOutBuilder::failing)
    /// - [`fresh`](SystemCountsOutBuilder::fresh)
    /// - [`stale`](SystemCountsOutBuilder::stale)
    /// - [`unavailable`](SystemCountsOutBuilder::unavailable)
    /// - [`unchecked`](SystemCountsOutBuilder::unchecked)
    /// - [`unknown`](SystemCountsOutBuilder::unknown)
    pub fn build(self) -> Result<SystemCountsOut, BuildError> {
        Ok(SystemCountsOut {
            failing: self.failing.ok_or_else(|| BuildError::missing_field("failing"))?,
            fresh: self.fresh.ok_or_else(|| BuildError::missing_field("fresh"))?,
            stale: self.stale.ok_or_else(|| BuildError::missing_field("stale"))?,
            unavailable: self.unavailable.ok_or_else(|| BuildError::missing_field("unavailable"))?,
            unchecked: self.unchecked.ok_or_else(|| BuildError::missing_field("unchecked"))?,
            unknown: self.unknown.ok_or_else(|| BuildError::missing_field("unknown"))?,
        })
    }
}
