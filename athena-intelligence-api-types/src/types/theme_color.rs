pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ThemeColor {
    #[serde(default)]
    pub theme: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<f64>,
}

impl ThemeColor {
    pub fn builder() -> ThemeColorBuilder {
        <ThemeColorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThemeColorBuilder {
    theme: Option<i64>,
    tint: Option<f64>,
}

impl ThemeColorBuilder {
    pub fn theme(mut self, value: i64) -> Self {
        self.theme = Some(value);
        self
    }

    pub fn tint(mut self, value: f64) -> Self {
        self.tint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThemeColor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`theme`](ThemeColorBuilder::theme)
    pub fn build(self) -> Result<ThemeColor, BuildError> {
        Ok(ThemeColor {
            theme: self.theme.ok_or_else(|| BuildError::missing_field("theme"))?,
            tint: self.tint,
        })
    }
}
