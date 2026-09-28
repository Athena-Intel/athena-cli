pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One compile or provisioning finding of a publish.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublishIssueOut {
    /// Stable code of the finding
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
    /// Where in the definition
    #[serde(default)]
    pub path: String,
    pub severity: PublishIssueOutSeverity,
}

impl PublishIssueOut {
    pub fn builder() -> PublishIssueOutBuilder {
        <PublishIssueOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishIssueOutBuilder {
    code: Option<String>,
    message: Option<String>,
    path: Option<String>,
    severity: Option<PublishIssueOutSeverity>,
}

impl PublishIssueOutBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn severity(mut self, value: PublishIssueOutSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublishIssueOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PublishIssueOutBuilder::code)
    /// - [`message`](PublishIssueOutBuilder::message)
    /// - [`path`](PublishIssueOutBuilder::path)
    /// - [`severity`](PublishIssueOutBuilder::severity)
    pub fn build(self) -> Result<PublishIssueOut, BuildError> {
        Ok(PublishIssueOut {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            severity: self.severity.ok_or_else(|| BuildError::missing_field("severity"))?,
        })
    }
}
