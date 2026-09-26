pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a run printed, as stored when it settled.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScriptRunLogsOut {
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub stdout: String,
    /// Whether either stream was cut (the tails are kept)
    #[serde(default)]
    pub truncated: bool,
}

impl ScriptRunLogsOut {
    pub fn builder() -> ScriptRunLogsOutBuilder {
        <ScriptRunLogsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptRunLogsOutBuilder {
    stderr: Option<String>,
    stdout: Option<String>,
    truncated: Option<bool>,
}

impl ScriptRunLogsOutBuilder {
    pub fn stderr(mut self, value: impl Into<String>) -> Self {
        self.stderr = Some(value.into());
        self
    }

    pub fn stdout(mut self, value: impl Into<String>) -> Self {
        self.stdout = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScriptRunLogsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stderr`](ScriptRunLogsOutBuilder::stderr)
    /// - [`stdout`](ScriptRunLogsOutBuilder::stdout)
    /// - [`truncated`](ScriptRunLogsOutBuilder::truncated)
    pub fn build(self) -> Result<ScriptRunLogsOut, BuildError> {
        Ok(ScriptRunLogsOut {
            stderr: self.stderr.ok_or_else(|| BuildError::missing_field("stderr"))?,
            stdout: self.stdout.ok_or_else(|| BuildError::missing_field("stdout"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
