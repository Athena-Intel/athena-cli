pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where the grant comes from: 'owner' (the user created the asset), 'direct' (the asset is shared with the user), 'group' (it is shared with a workspace group the user belongs to), 'workspace' (it is shared with the whole workspace), 'drive' (a role on the drive the asset is in) or 'other' (access the share list does not hold, for example an environment admin, also on an asset in a drive, or the view of a folder that holds an asset the user can open). A folder's sharing reaches each asset inside it as that asset's own share, so a 'workspace' source on a sub-folder shows that a share of its parent folder reached it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetAccessSourceOutKind {
    Owner,
    Direct,
    Group,
    Workspace,
    Drive,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AssetAccessSourceOutKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("owner"),
            Self::Direct => serializer.serialize_str("direct"),
            Self::Group => serializer.serialize_str("group"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::Drive => serializer.serialize_str("drive"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AssetAccessSourceOutKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "owner" => Ok(Self::Owner),
            "direct" => Ok(Self::Direct),
            "group" => Ok(Self::Group),
            "workspace" => Ok(Self::Workspace),
            "drive" => Ok(Self::Drive),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AssetAccessSourceOutKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "owner"),
            Self::Direct => write!(f, "direct"),
            Self::Group => write!(f, "group"),
            Self::Workspace => write!(f, "workspace"),
            Self::Drive => write!(f, "drive"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
