pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Token counters summed from the sessions' LLM usage ledgers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceUsageTokensOut {
    /// Input tokens written to the provider's prompt cache
    #[serde(default)]
    pub cache_creation_input_tokens: i64,
    /// Input tokens served from the provider's prompt cache
    #[serde(default)]
    pub cache_read_input_tokens: i64,
    /// Input (prompt) tokens, including cached reads
    #[serde(default)]
    pub input_tokens: i64,
    /// Output (completion) tokens, including reasoning tokens
    #[serde(default)]
    pub output_tokens: i64,
    /// Output tokens spent on reasoning
    #[serde(default)]
    pub reasoning_output_tokens: i64,
    /// Total tokens (input + output)
    #[serde(default)]
    pub total_tokens: i64,
}

impl WorkspaceUsageTokensOut {
    pub fn builder() -> WorkspaceUsageTokensOutBuilder {
        <WorkspaceUsageTokensOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUsageTokensOutBuilder {
    cache_creation_input_tokens: Option<i64>,
    cache_read_input_tokens: Option<i64>,
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
    reasoning_output_tokens: Option<i64>,
    total_tokens: Option<i64>,
}

impl WorkspaceUsageTokensOutBuilder {
    pub fn cache_creation_input_tokens(mut self, value: i64) -> Self {
        self.cache_creation_input_tokens = Some(value);
        self
    }

    pub fn cache_read_input_tokens(mut self, value: i64) -> Self {
        self.cache_read_input_tokens = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn reasoning_output_tokens(mut self, value: i64) -> Self {
        self.reasoning_output_tokens = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUsageTokensOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cache_creation_input_tokens`](WorkspaceUsageTokensOutBuilder::cache_creation_input_tokens)
    /// - [`cache_read_input_tokens`](WorkspaceUsageTokensOutBuilder::cache_read_input_tokens)
    /// - [`input_tokens`](WorkspaceUsageTokensOutBuilder::input_tokens)
    /// - [`output_tokens`](WorkspaceUsageTokensOutBuilder::output_tokens)
    /// - [`reasoning_output_tokens`](WorkspaceUsageTokensOutBuilder::reasoning_output_tokens)
    /// - [`total_tokens`](WorkspaceUsageTokensOutBuilder::total_tokens)
    pub fn build(self) -> Result<WorkspaceUsageTokensOut, BuildError> {
        Ok(WorkspaceUsageTokensOut {
            cache_creation_input_tokens: self.cache_creation_input_tokens.ok_or_else(|| BuildError::missing_field("cache_creation_input_tokens"))?,
            cache_read_input_tokens: self.cache_read_input_tokens.ok_or_else(|| BuildError::missing_field("cache_read_input_tokens"))?,
            input_tokens: self.input_tokens.ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            output_tokens: self.output_tokens.ok_or_else(|| BuildError::missing_field("output_tokens"))?,
            reasoning_output_tokens: self.reasoning_output_tokens.ok_or_else(|| BuildError::missing_field("reasoning_output_tokens"))?,
            total_tokens: self.total_tokens.ok_or_else(|| BuildError::missing_field("total_tokens"))?,
        })
    }
}
