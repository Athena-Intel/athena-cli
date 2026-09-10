pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InitializationPhase {
    Queued,
    Preparing,
    Allocating,
    WaitingForRuntime,
    ImportingSnapshot,
    VerifyingSnapshot,
    BootingVm,
    InitializingEnvironment,
    AttachingStorage,
    ConfiguringResources,
    RunningSetup,
    StartingAgent,
    CheckingReadiness,
    Complete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InitializationPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::Preparing => serializer.serialize_str("preparing"),
            Self::Allocating => serializer.serialize_str("allocating"),
            Self::WaitingForRuntime => serializer.serialize_str("waiting_for_runtime"),
            Self::ImportingSnapshot => serializer.serialize_str("importing_snapshot"),
            Self::VerifyingSnapshot => serializer.serialize_str("verifying_snapshot"),
            Self::BootingVm => serializer.serialize_str("booting_vm"),
            Self::InitializingEnvironment => serializer.serialize_str("initializing_environment"),
            Self::AttachingStorage => serializer.serialize_str("attaching_storage"),
            Self::ConfiguringResources => serializer.serialize_str("configuring_resources"),
            Self::RunningSetup => serializer.serialize_str("running_setup"),
            Self::StartingAgent => serializer.serialize_str("starting_agent"),
            Self::CheckingReadiness => serializer.serialize_str("checking_readiness"),
            Self::Complete => serializer.serialize_str("complete"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InitializationPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "preparing" => Ok(Self::Preparing),
            "allocating" => Ok(Self::Allocating),
            "waiting_for_runtime" => Ok(Self::WaitingForRuntime),
            "importing_snapshot" => Ok(Self::ImportingSnapshot),
            "verifying_snapshot" => Ok(Self::VerifyingSnapshot),
            "booting_vm" => Ok(Self::BootingVm),
            "initializing_environment" => Ok(Self::InitializingEnvironment),
            "attaching_storage" => Ok(Self::AttachingStorage),
            "configuring_resources" => Ok(Self::ConfiguringResources),
            "running_setup" => Ok(Self::RunningSetup),
            "starting_agent" => Ok(Self::StartingAgent),
            "checking_readiness" => Ok(Self::CheckingReadiness),
            "complete" => Ok(Self::Complete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InitializationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Preparing => write!(f, "preparing"),
            Self::Allocating => write!(f, "allocating"),
            Self::WaitingForRuntime => write!(f, "waiting_for_runtime"),
            Self::ImportingSnapshot => write!(f, "importing_snapshot"),
            Self::VerifyingSnapshot => write!(f, "verifying_snapshot"),
            Self::BootingVm => write!(f, "booting_vm"),
            Self::InitializingEnvironment => write!(f, "initializing_environment"),
            Self::AttachingStorage => write!(f, "attaching_storage"),
            Self::ConfiguringResources => write!(f, "configuring_resources"),
            Self::RunningSetup => write!(f, "running_setup"),
            Self::StartingAgent => write!(f, "starting_agent"),
            Self::CheckingReadiness => write!(f, "checking_readiness"),
            Self::Complete => write!(f, "complete"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
