pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One option an approval offers, with the effect a decision on it has.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalOptionOut {
    /// approve, reject or custom
    #[serde(default)]
    pub effect: String,
    /// The option id a decision names
    #[serde(default)]
    pub id: String,
}

impl ApprovalOptionOut {
    pub fn builder() -> ApprovalOptionOutBuilder {
        <ApprovalOptionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalOptionOutBuilder {
    effect: Option<String>,
    id: Option<String>,
}

impl ApprovalOptionOutBuilder {
    pub fn effect(mut self, value: impl Into<String>) -> Self {
        self.effect = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalOptionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`effect`](ApprovalOptionOutBuilder::effect)
    /// - [`id`](ApprovalOptionOutBuilder::id)
    pub fn build(self) -> Result<ApprovalOptionOut, BuildError> {
        Ok(ApprovalOptionOut {
            effect: self.effect.ok_or_else(|| BuildError::missing_field("effect"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
