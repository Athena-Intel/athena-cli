pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A computer's live runtime state, read without starting or waking it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComputerStatusResponseOut {
    /// The computer asset id.
    #[serde(default)]
    pub asset_id: String,
    /// Set when someone stopped the computer with Stop. While it is `stopping`, `stopped` or `uncertain`, implicit wakes are refused: minting SSH access answers 409 and an SSH connection does not start it. Call `start_computer` to clear it. `null` or `cleared` means no explicit stop is in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_stop: Option<ComputerStatusResponseOutExplicitStop>,
    /// Why `ready` is false; `null` when it is true. `not_started`: the computer is not running — call `start_computer`. `starting`: it is running and its guest is still coming up — keep polling. `unobservable`: readiness could not be read this time — keep polling. `launcher_outdated`: the computer runs an outdated runtime component that polling will not change — stop it and start it again to update it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ready_reason: Option<ComputerStatusResponseOutNotReadyReason>,
    /// The computer's runtime provider (e.g. `talos_v2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Whether the computer serves work now: `state` is `started` and the guest answers ready (its workspace attached and its environment applied). Only then do commands, SSH sessions and deployed URLs reach the computer without first waiting for a start. `started` alone can come a few seconds before `ready`.
    #[serde(default)]
    pub ready: bool,
    /// The runtime's current state: `started` (running; check `ready`), `stopped` (suspended — by idle auto-stop or by Stop; files are kept), `restoring` (a start is in progress), `error` (the runtime failed; start or restart it), or `unknown` (the state could not be read this time; retry). `started`, `stopped` and `error` hold until something changes them; `restoring` and `unknown` are transient.
    #[serde(default)]
    pub state: String,
}

impl ComputerStatusResponseOut {
    pub fn builder() -> ComputerStatusResponseOutBuilder {
        <ComputerStatusResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputerStatusResponseOutBuilder {
    asset_id: Option<String>,
    explicit_stop: Option<ComputerStatusResponseOutExplicitStop>,
    not_ready_reason: Option<ComputerStatusResponseOutNotReadyReason>,
    provider: Option<String>,
    ready: Option<bool>,
    state: Option<String>,
}

impl ComputerStatusResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn explicit_stop(mut self, value: ComputerStatusResponseOutExplicitStop) -> Self {
        self.explicit_stop = Some(value);
        self
    }

    pub fn not_ready_reason(mut self, value: ComputerStatusResponseOutNotReadyReason) -> Self {
        self.not_ready_reason = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn ready(mut self, value: bool) -> Self {
        self.ready = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ComputerStatusResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ComputerStatusResponseOutBuilder::asset_id)
    /// - [`ready`](ComputerStatusResponseOutBuilder::ready)
    /// - [`state`](ComputerStatusResponseOutBuilder::state)
    pub fn build(self) -> Result<ComputerStatusResponseOut, BuildError> {
        Ok(ComputerStatusResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            explicit_stop: self.explicit_stop,
            not_ready_reason: self.not_ready_reason,
            provider: self.provider,
            ready: self.ready.ok_or_else(|| BuildError::missing_field("ready"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
