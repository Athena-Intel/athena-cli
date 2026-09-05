pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextFormatModel {
    /// Superscript/subscript text positioning (Excel's vertAlign): 'superscript' or 'subscript' to set, 'baseline' to explicitly remove an existing offset (return text to the normal baseline). Omit to leave the current offset unchanged.
    #[serde(rename = "baselineOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_offset: Option<TextFormatModelBaselineOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Text color as hex string (e.g., '#FF0000'). Use either color_hex or color_theme, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_hex: Option<String>,
    /// Text color as theme reference. Use either color_hex or color_theme, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<ThemeColor>,
    #[serde(rename = "fontFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
}

impl TextFormatModel {
    pub fn builder() -> TextFormatModelBuilder {
        <TextFormatModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextFormatModelBuilder {
    baseline_offset: Option<TextFormatModelBaselineOffset>,
    bold: Option<bool>,
    color_hex: Option<String>,
    color_theme: Option<ThemeColor>,
    font_family: Option<String>,
    font_size: Option<i64>,
    italic: Option<bool>,
    strikethrough: Option<bool>,
    underline: Option<bool>,
}

impl TextFormatModelBuilder {
    pub fn baseline_offset(mut self, value: TextFormatModelBaselineOffset) -> Self {
        self.baseline_offset = Some(value);
        self
    }

    pub fn bold(mut self, value: bool) -> Self {
        self.bold = Some(value);
        self
    }

    pub fn color_hex(mut self, value: impl Into<String>) -> Self {
        self.color_hex = Some(value.into());
        self
    }

    pub fn color_theme(mut self, value: ThemeColor) -> Self {
        self.color_theme = Some(value);
        self
    }

    pub fn font_family(mut self, value: impl Into<String>) -> Self {
        self.font_family = Some(value.into());
        self
    }

    pub fn font_size(mut self, value: i64) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn italic(mut self, value: bool) -> Self {
        self.italic = Some(value);
        self
    }

    pub fn strikethrough(mut self, value: bool) -> Self {
        self.strikethrough = Some(value);
        self
    }

    pub fn underline(mut self, value: bool) -> Self {
        self.underline = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextFormatModel`].
    pub fn build(self) -> Result<TextFormatModel, BuildError> {
        Ok(TextFormatModel {
            baseline_offset: self.baseline_offset,
            bold: self.bold,
            color_hex: self.color_hex,
            color_theme: self.color_theme,
            font_family: self.font_family,
            font_size: self.font_size,
            italic: self.italic,
            strikethrough: self.strikethrough,
            underline: self.underline,
        })
    }
}
