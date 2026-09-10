pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InitializationRecipeStage {
    BaseTemplate,
    EnvironmentSeed,
    EnvironmentInitialize,
    EnvironmentMaintenance,
    ServiceConfiguration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InitializationRecipeStage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BaseTemplate => serializer.serialize_str("base_template"),
            Self::EnvironmentSeed => serializer.serialize_str("environment_seed"),
            Self::EnvironmentInitialize => serializer.serialize_str("environment_initialize"),
            Self::EnvironmentMaintenance => serializer.serialize_str("environment_maintenance"),
            Self::ServiceConfiguration => serializer.serialize_str("service_configuration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InitializationRecipeStage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "base_template" => Ok(Self::BaseTemplate),
            "environment_seed" => Ok(Self::EnvironmentSeed),
            "environment_initialize" => Ok(Self::EnvironmentInitialize),
            "environment_maintenance" => Ok(Self::EnvironmentMaintenance),
            "service_configuration" => Ok(Self::ServiceConfiguration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InitializationRecipeStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BaseTemplate => write!(f, "base_template"),
            Self::EnvironmentSeed => write!(f, "environment_seed"),
            Self::EnvironmentInitialize => write!(f, "environment_initialize"),
            Self::EnvironmentMaintenance => write!(f, "environment_maintenance"),
            Self::ServiceConfiguration => write!(f, "service_configuration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
