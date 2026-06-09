pub use crate::prelude::*;
use super::*;

/// Metadata response listing cubes, measures, and dimensions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelMetaResponseOut {
    /// List of cubes with their measures and dimensions
    #[serde(default)]
    pub cubes: Vec<HashMap<String, serde_json::Value>>,
}

impl SemanticModelMetaResponseOut {
    pub fn builder() -> SemanticModelMetaResponseOutBuilder {
        <SemanticModelMetaResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SemanticModelMetaResponseOutBuilder {
    cubes: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SemanticModelMetaResponseOutBuilder {
    pub fn cubes(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.cubes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelMetaResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cubes`](SemanticModelMetaResponseOutBuilder::cubes)
    pub fn build(self) -> Result<SemanticModelMetaResponseOut, BuildError> {
        Ok(SemanticModelMetaResponseOut {
            cubes: self.cubes.ok_or_else(|| BuildError::missing_field("cubes"))?,
        })
    }
}
