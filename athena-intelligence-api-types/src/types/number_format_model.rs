pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NumberFormatModel {
    /// Excel-compatible format string. If omitted, the server will pick a sensible default based on the chosen type (e.g. NUMBER → '#,##0', CURRENCY → '$#,##0.00').
    #[serde(default)]
    pub pattern: String,
    /// Target number format category (NUMBER, CURRENCY, DATE, etc.)
    pub r#type: NumberFormatType,
}

impl NumberFormatModel {
    pub fn builder() -> NumberFormatModelBuilder {
        <NumberFormatModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NumberFormatModelBuilder {
    pattern: Option<String>,
    r#type: Option<NumberFormatType>,
}

impl NumberFormatModelBuilder {
    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: NumberFormatType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NumberFormatModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pattern`](NumberFormatModelBuilder::pattern)
    /// - [`r#type`](NumberFormatModelBuilder::r#type)
    pub fn build(self) -> Result<NumberFormatModel, BuildError> {
        Ok(NumberFormatModel {
            pattern: self.pattern.ok_or_else(|| BuildError::missing_field("pattern"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
