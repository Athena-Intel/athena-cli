pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncidentResolveRequestIn {
    /// Why the incident is resolved; recorded on its timeline. A blank reason is refused with 400 (detail.reason reason_required)
    #[serde(default)]
    pub reason: String,
}

impl IncidentResolveRequestIn {
    pub fn builder() -> IncidentResolveRequestInBuilder {
        <IncidentResolveRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncidentResolveRequestInBuilder {
    reason: Option<String>,
}

impl IncidentResolveRequestInBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncidentResolveRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](IncidentResolveRequestInBuilder::reason)
    pub fn build(self) -> Result<IncidentResolveRequestIn, BuildError> {
        Ok(IncidentResolveRequestIn {
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}

