pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One run of a script: a sandboxed execution of one version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScriptRunOut {
    /// The automation run whose step started it, when one did
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_run_id: Option<String>,
    /// When the run was claimed
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Whether the caller may read this run's output, logs and error message: the person who started it, or a viewer of the automation whose step did
    #[serde(default)]
    pub details_visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    /// Why it failed: script_failed, output_invalid, script_timeout, script_interrupted, sandbox_lost, sandbox_unavailable, script_canceled, …
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The failure in words; null unless details_visible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// Whether the run's stdout/stderr were stored
    #[serde(default)]
    pub has_logs: bool,
    /// The JSON the script wrote to $ATHENA_OUTPUT_PATH; null unless details_visible, and null for a run that wrote none. A preview (`{_truncated, bytes, preview}`) when output_truncated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    /// Whether output is a preview of an output stored by reference; read the whole one from GET /scripts/runs/{run_id}/output
    #[serde(default)]
    pub output_truncated: bool,
    /// Who ran it: `user_…`, or `automation_…` for an automation step
    #[serde(default)]
    pub principal_ref: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub script_asset_id: String,
    /// When the launch was requested
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// queued | running | completed | failed | canceled
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub timeout_seconds: i64,
    /// The saved version that ran; null when the live source ran unpinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default)]
    pub workspace_id: String,
}

impl ScriptRunOut {
    pub fn builder() -> ScriptRunOutBuilder {
        <ScriptRunOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScriptRunOutBuilder {
    automation_run_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    details_visible: Option<bool>,
    duration_ms: Option<i64>,
    ended_at: Option<DateTime<FixedOffset>>,
    error_code: Option<String>,
    error_message: Option<String>,
    exit_code: Option<i64>,
    has_logs: Option<bool>,
    output: Option<serde_json::Value>,
    output_truncated: Option<bool>,
    principal_ref: Option<String>,
    run_id: Option<String>,
    script_asset_id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<String>,
    timeout_seconds: Option<i64>,
    version_id: Option<String>,
    workspace_id: Option<String>,
}

impl ScriptRunOutBuilder {
    pub fn automation_run_id(mut self, value: impl Into<String>) -> Self {
        self.automation_run_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn details_visible(mut self, value: bool) -> Self {
        self.details_visible = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn error_code(mut self, value: impl Into<String>) -> Self {
        self.error_code = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn exit_code(mut self, value: i64) -> Self {
        self.exit_code = Some(value);
        self
    }

    pub fn has_logs(mut self, value: bool) -> Self {
        self.has_logs = Some(value);
        self
    }

    pub fn output(mut self, value: serde_json::Value) -> Self {
        self.output = Some(value);
        self
    }

    pub fn output_truncated(mut self, value: bool) -> Self {
        self.output_truncated = Some(value);
        self
    }

    pub fn principal_ref(mut self, value: impl Into<String>) -> Self {
        self.principal_ref = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn script_asset_id(mut self, value: impl Into<String>) -> Self {
        self.script_asset_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn timeout_seconds(mut self, value: i64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScriptRunOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ScriptRunOutBuilder::created_at)
    /// - [`details_visible`](ScriptRunOutBuilder::details_visible)
    /// - [`has_logs`](ScriptRunOutBuilder::has_logs)
    /// - [`output_truncated`](ScriptRunOutBuilder::output_truncated)
    /// - [`principal_ref`](ScriptRunOutBuilder::principal_ref)
    /// - [`run_id`](ScriptRunOutBuilder::run_id)
    /// - [`script_asset_id`](ScriptRunOutBuilder::script_asset_id)
    /// - [`status`](ScriptRunOutBuilder::status)
    /// - [`timeout_seconds`](ScriptRunOutBuilder::timeout_seconds)
    /// - [`workspace_id`](ScriptRunOutBuilder::workspace_id)
    pub fn build(self) -> Result<ScriptRunOut, BuildError> {
        Ok(ScriptRunOut {
            automation_run_id: self.automation_run_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            details_visible: self.details_visible.ok_or_else(|| BuildError::missing_field("details_visible"))?,
            duration_ms: self.duration_ms,
            ended_at: self.ended_at,
            error_code: self.error_code,
            error_message: self.error_message,
            exit_code: self.exit_code,
            has_logs: self.has_logs.ok_or_else(|| BuildError::missing_field("has_logs"))?,
            output: self.output,
            output_truncated: self.output_truncated.ok_or_else(|| BuildError::missing_field("output_truncated"))?,
            principal_ref: self.principal_ref.ok_or_else(|| BuildError::missing_field("principal_ref"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            script_asset_id: self.script_asset_id.ok_or_else(|| BuildError::missing_field("script_asset_id"))?,
            started_at: self.started_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            timeout_seconds: self.timeout_seconds.ok_or_else(|| BuildError::missing_field("timeout_seconds"))?,
            version_id: self.version_id,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
