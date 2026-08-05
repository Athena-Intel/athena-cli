pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata response listing cubes, views, measures, and dimensions.
/// 
/// The ``cubes`` field preserves Cube's native ``/v1/meta`` shape and
/// contains both cubes and views interleaved (each entry has a ``type``
/// field of ``cube`` or ``view``). The ``views`` field is a convenience
/// filter that lists only the view-typed entries — recommended for
/// AI agents that should query the curated surface.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SemanticModelMetaResponseOut {
    /// All meta entries (cubes + views) in author order. Each entry has a 'type' field — 'cube' or 'view'.
    #[serde(default)]
    pub cubes: Vec<HashMap<String, serde_json::Value>>,
    /// View-only subset of 'cubes' for convenience. Empty when no views are defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<HashMap<String, serde_json::Value>>>,
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
    views: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SemanticModelMetaResponseOutBuilder {
    pub fn cubes(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.cubes = Some(value);
        self
    }

    pub fn views(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.views = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SemanticModelMetaResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cubes`](SemanticModelMetaResponseOutBuilder::cubes)
    pub fn build(self) -> Result<SemanticModelMetaResponseOut, BuildError> {
        Ok(SemanticModelMetaResponseOut {
            cubes: self.cubes.ok_or_else(|| BuildError::missing_field("cubes"))?,
            views: self.views,
        })
    }
}
