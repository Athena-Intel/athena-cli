pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Approve once for a while.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StandingGrantIn {
    /// How long the grant runs, e.g. 7d or 36h
    #[serde(default)]
    pub duration: String,
}

impl StandingGrantIn {
    pub fn builder() -> StandingGrantInBuilder {
        <StandingGrantInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StandingGrantInBuilder {
    duration: Option<String>,
}

impl StandingGrantInBuilder {
    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StandingGrantIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration`](StandingGrantInBuilder::duration)
    pub fn build(self) -> Result<StandingGrantIn, BuildError> {
        Ok(StandingGrantIn {
            duration: self.duration.ok_or_else(|| BuildError::missing_field("duration"))?,
        })
    }
}
