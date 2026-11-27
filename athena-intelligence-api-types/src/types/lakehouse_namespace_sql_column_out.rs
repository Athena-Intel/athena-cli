pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One column of a lakehouse namespace statement's result.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LakehouseNamespaceSqlColumnOut {
    /// The column's name as the statement gave it
    #[serde(default)]
    pub name: String,
    /// The engine's type for the column (e.g. `bigint`)
    #[serde(default)]
    pub r#type: String,
}

impl LakehouseNamespaceSqlColumnOut {
    pub fn builder() -> LakehouseNamespaceSqlColumnOutBuilder {
        <LakehouseNamespaceSqlColumnOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LakehouseNamespaceSqlColumnOutBuilder {
    name: Option<String>,
    r#type: Option<String>,
}

impl LakehouseNamespaceSqlColumnOutBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LakehouseNamespaceSqlColumnOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](LakehouseNamespaceSqlColumnOutBuilder::name)
    /// - [`r#type`](LakehouseNamespaceSqlColumnOutBuilder::r#type)
    pub fn build(self) -> Result<LakehouseNamespaceSqlColumnOut, BuildError> {
        Ok(LakehouseNamespaceSqlColumnOut {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
