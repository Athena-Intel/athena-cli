pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What ends a one-off wait early: news on a pull request this conversation owns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaitEarlyOnInput {
    /// Names of the checks you are waiting for, as GitHub shows them, e.g. ["Check"] (at most 10; name the required ones). One of them finishing, whatever its conclusion, ends the wait. Without checks, only a review, a review comment, a person's comment, a merge or a close does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<String>>,
    /// The pull request you are waiting on, as owner/repo#123 or its link. It must be one this conversation opened or took on.
    #[serde(default)]
    pub pull_request: String,
}

impl WaitEarlyOnInput {
    pub fn builder() -> WaitEarlyOnInputBuilder {
        <WaitEarlyOnInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitEarlyOnInputBuilder {
    checks: Option<Vec<String>>,
    pull_request: Option<String>,
}

impl WaitEarlyOnInputBuilder {
    pub fn checks(mut self, value: Vec<String>) -> Self {
        self.checks = Some(value);
        self
    }

    pub fn pull_request(mut self, value: impl Into<String>) -> Self {
        self.pull_request = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WaitEarlyOnInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pull_request`](WaitEarlyOnInputBuilder::pull_request)
    pub fn build(self) -> Result<WaitEarlyOnInput, BuildError> {
        Ok(WaitEarlyOnInput {
            checks: self.checks,
            pull_request: self.pull_request.ok_or_else(|| BuildError::missing_field("pull_request"))?,
        })
    }
}
