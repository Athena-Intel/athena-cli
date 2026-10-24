pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExplainAssetAccessInput {
    /// The asset.
    #[serde(default)]
    pub asset_id: String,
    /// The person: a member of the asset's workspace, by user id or email.
    #[serde(default)]
    pub person: String,
}

impl ExplainAssetAccessInput {
    pub fn builder() -> ExplainAssetAccessInputBuilder {
        <ExplainAssetAccessInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExplainAssetAccessInputBuilder {
    asset_id: Option<String>,
    person: Option<String>,
}

impl ExplainAssetAccessInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn person(mut self, value: impl Into<String>) -> Self {
        self.person = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExplainAssetAccessInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ExplainAssetAccessInputBuilder::asset_id)
    /// - [`person`](ExplainAssetAccessInputBuilder::person)
    pub fn build(self) -> Result<ExplainAssetAccessInput, BuildError> {
        Ok(ExplainAssetAccessInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            person: self.person.ok_or_else(|| BuildError::missing_field("person"))?,
        })
    }
}

