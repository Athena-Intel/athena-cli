pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComputerInitializationResponseReason {
    DeadlineExceeded,
    AuthorizationChanged,
    InputsChanged,
    RuntimeChanged,
    DispatchUnconfirmed,
    ExecutionUnconfirmed,
    ExecutionFailed,
    WorkerUnavailable,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ComputerInitializationResponseReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DeadlineExceeded => serializer.serialize_str("deadline_exceeded"),
            Self::AuthorizationChanged => serializer.serialize_str("authorization_changed"),
            Self::InputsChanged => serializer.serialize_str("inputs_changed"),
            Self::RuntimeChanged => serializer.serialize_str("runtime_changed"),
            Self::DispatchUnconfirmed => serializer.serialize_str("dispatch_unconfirmed"),
            Self::ExecutionUnconfirmed => serializer.serialize_str("execution_unconfirmed"),
            Self::ExecutionFailed => serializer.serialize_str("execution_failed"),
            Self::WorkerUnavailable => serializer.serialize_str("worker_unavailable"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ComputerInitializationResponseReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "deadline_exceeded" => Ok(Self::DeadlineExceeded),
            "authorization_changed" => Ok(Self::AuthorizationChanged),
            "inputs_changed" => Ok(Self::InputsChanged),
            "runtime_changed" => Ok(Self::RuntimeChanged),
            "dispatch_unconfirmed" => Ok(Self::DispatchUnconfirmed),
            "execution_unconfirmed" => Ok(Self::ExecutionUnconfirmed),
            "execution_failed" => Ok(Self::ExecutionFailed),
            "worker_unavailable" => Ok(Self::WorkerUnavailable),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ComputerInitializationResponseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeadlineExceeded => write!(f, "deadline_exceeded"),
            Self::AuthorizationChanged => write!(f, "authorization_changed"),
            Self::InputsChanged => write!(f, "inputs_changed"),
            Self::RuntimeChanged => write!(f, "runtime_changed"),
            Self::DispatchUnconfirmed => write!(f, "dispatch_unconfirmed"),
            Self::ExecutionUnconfirmed => write!(f, "execution_unconfirmed"),
            Self::ExecutionFailed => write!(f, "execution_failed"),
            Self::WorkerUnavailable => write!(f, "worker_unavailable"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
