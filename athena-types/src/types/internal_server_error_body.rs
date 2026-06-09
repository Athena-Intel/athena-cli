pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum InternalServerErrorBody {
        SqlServiceError(SqlServiceError),

        InvalidSqlResponseError(InvalidSqlResponseError),
}

impl InternalServerErrorBody {
    pub fn is_sql_service_error(&self) -> bool {
        matches!(self, Self::SqlServiceError(_))
    }

    pub fn is_invalid_sql_response_error(&self) -> bool {
        matches!(self, Self::InvalidSqlResponseError(_))
    }


    pub fn as_sql_service_error(&self) -> Option<&SqlServiceError> {
        match self {
                    Self::SqlServiceError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sql_service_error(self) -> Option<SqlServiceError> {
        match self {
                    Self::SqlServiceError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_invalid_sql_response_error(&self) -> Option<&InvalidSqlResponseError> {
        match self {
                    Self::InvalidSqlResponseError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_invalid_sql_response_error(self) -> Option<InvalidSqlResponseError> {
        match self {
                    Self::InvalidSqlResponseError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for InternalServerErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SqlServiceError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InvalidSqlResponseError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
