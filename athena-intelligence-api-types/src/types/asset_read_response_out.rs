pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for a parameterized asset read.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetReadResponseOut {
    /// One result per requested asset, in request order
    #[serde(default)]
    pub results: Vec<AssetReadResult>,
}

impl AssetReadResponseOut {
    pub fn builder() -> AssetReadResponseOutBuilder {
        <AssetReadResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetReadResponseOutBuilder {
    results: Option<Vec<AssetReadResult>>,
}

impl AssetReadResponseOutBuilder {
    pub fn results(mut self, value: Vec<AssetReadResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetReadResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](AssetReadResponseOutBuilder::results)
    pub fn build(self) -> Result<AssetReadResponseOut, BuildError> {
        Ok(AssetReadResponseOut {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
