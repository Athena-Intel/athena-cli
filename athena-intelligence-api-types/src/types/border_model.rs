pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderModel {
    /// Border color as hex string (e.g., '#FF0000'). Use either color_hex or color_theme, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_hex: Option<String>,
    /// Border color as theme reference. Use either color_hex or color_theme, not both. Defaults to theme color 1 if neither is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<ThemeColor>,
    pub style: BorderStyle,
    #[serde(default)]
    pub width: i64,
}

impl BorderModel {
    pub fn builder() -> BorderModelBuilder {
        <BorderModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BorderModelBuilder {
    color_hex: Option<String>,
    color_theme: Option<ThemeColor>,
    style: Option<BorderStyle>,
    width: Option<i64>,
}

impl BorderModelBuilder {
    pub fn color_hex(mut self, value: impl Into<String>) -> Self {
        self.color_hex = Some(value.into());
        self
    }

    pub fn color_theme(mut self, value: ThemeColor) -> Self {
        self.color_theme = Some(value);
        self
    }

    pub fn style(mut self, value: BorderStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BorderModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`style`](BorderModelBuilder::style)
    /// - [`width`](BorderModelBuilder::width)
    pub fn build(self) -> Result<BorderModel, BuildError> {
        Ok(BorderModel {
            color_hex: self.color_hex,
            color_theme: self.color_theme,
            style: self.style.ok_or_else(|| BuildError::missing_field("style"))?,
            width: self.width.ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
