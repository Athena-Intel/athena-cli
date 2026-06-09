pub use crate::prelude::*;
use super::*;

/// Column metadata for table schema response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseColumnInfo {
    /// PostgreSQL data type
    #[serde(default)]
    pub data_type: String,
    /// Default value expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Whether the column allows NULL values
    #[serde(default)]
    pub is_nullable: bool,
    /// Maximum length for character types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// Column name
    #[serde(default)]
    pub name: String,
}

impl DatabaseColumnInfo {
    pub fn builder() -> DatabaseColumnInfoBuilder {
        <DatabaseColumnInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseColumnInfoBuilder {
    data_type: Option<String>,
    default_value: Option<String>,
    is_nullable: Option<bool>,
    max_length: Option<i64>,
    name: Option<String>,
}

impl DatabaseColumnInfoBuilder {
    pub fn data_type(mut self, value: impl Into<String>) -> Self {
        self.data_type = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn is_nullable(mut self, value: bool) -> Self {
        self.is_nullable = Some(value);
        self
    }

    pub fn max_length(mut self, value: i64) -> Self {
        self.max_length = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabaseColumnInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data_type`](DatabaseColumnInfoBuilder::data_type)
    /// - [`is_nullable`](DatabaseColumnInfoBuilder::is_nullable)
    /// - [`name`](DatabaseColumnInfoBuilder::name)
    pub fn build(self) -> Result<DatabaseColumnInfo, BuildError> {
        Ok(DatabaseColumnInfo {
            data_type: self.data_type.ok_or_else(|| BuildError::missing_field("data_type"))?,
            default_value: self.default_value,
            is_nullable: self.is_nullable.ok_or_else(|| BuildError::missing_field("is_nullable"))?,
            max_length: self.max_length,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
