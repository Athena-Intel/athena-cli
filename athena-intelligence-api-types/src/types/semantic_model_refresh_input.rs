pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SemanticModelRefreshInput {
    /// Semantic model asset id (asset_…). The caller needs EDIT on it; the working copy is re-deployed through the model's own deploy path.
    #[serde(default)]
    pub asset_id: String,
}

impl SemanticModelRefreshInput {
    pub fn builder() -> SemanticModelRefreshInputBuilder {
        <SemanticModelRefreshInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelRefreshInputBuilder {
    asset_id: Option<String>,
}

impl SemanticModelRefreshInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelRefreshInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](SemanticModelRefreshInputBuilder::asset_id)
    pub fn build(self) -> Result<SemanticModelRefreshInput, BuildError> {
        Ok(SemanticModelRefreshInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

