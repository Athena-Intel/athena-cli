pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One action a paused run is waiting for a person to approve.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollabAgentPendingActionOut {
    /// Bounded summary of the pending action, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Name of the tool waiting for approval, when known.
    #[serde(rename = "toolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

impl CollabAgentPendingActionOut {
    pub fn builder() -> CollabAgentPendingActionOutBuilder {
        <CollabAgentPendingActionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollabAgentPendingActionOutBuilder {
    summary: Option<String>,
    tool_name: Option<String>,
}

impl CollabAgentPendingActionOutBuilder {
    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CollabAgentPendingActionOut`].
    pub fn build(self) -> Result<CollabAgentPendingActionOut, BuildError> {
        Ok(CollabAgentPendingActionOut {
            summary: self.summary,
            tool_name: self.tool_name,
        })
    }
}
