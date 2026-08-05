pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CellFormat {
    /// Background color as hex string (e.g., '#FF0000'). Use either backgroundColor_hex or backgroundColor_theme, not both.
    #[serde(rename = "backgroundColor_hex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color_hex: Option<String>,
    /// Background color as theme reference. Use either backgroundColor_hex or backgroundColor_theme, not both.
    #[serde(rename = "backgroundColor_theme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color_theme: Option<ThemeColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<BordersModel>,
    #[serde(rename = "horizontalAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<CellFormatHorizontalAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<i64>,
    #[serde(rename = "numberFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_format: Option<NumberFormatModel>,
    #[serde(rename = "textFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<TextFormatModel>,
    #[serde(rename = "textRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_rotation: Option<CellFormatTextRotation>,
    #[serde(rename = "verticalAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<CellFormatVerticalAlignment>,
    #[serde(rename = "wrapStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_strategy: Option<WrapStrategy>,
}

impl CellFormat {
    pub fn builder() -> CellFormatBuilder {
        <CellFormatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CellFormatBuilder {
    background_color_hex: Option<String>,
    background_color_theme: Option<ThemeColor>,
    borders: Option<BordersModel>,
    horizontal_alignment: Option<CellFormatHorizontalAlignment>,
    indent: Option<i64>,
    number_format: Option<NumberFormatModel>,
    text_format: Option<TextFormatModel>,
    text_rotation: Option<CellFormatTextRotation>,
    vertical_alignment: Option<CellFormatVerticalAlignment>,
    wrap_strategy: Option<WrapStrategy>,
}

impl CellFormatBuilder {
    pub fn background_color_hex(mut self, value: impl Into<String>) -> Self {
        self.background_color_hex = Some(value.into());
        self
    }

    pub fn background_color_theme(mut self, value: ThemeColor) -> Self {
        self.background_color_theme = Some(value);
        self
    }

    pub fn borders(mut self, value: BordersModel) -> Self {
        self.borders = Some(value);
        self
    }

    pub fn horizontal_alignment(mut self, value: CellFormatHorizontalAlignment) -> Self {
        self.horizontal_alignment = Some(value);
        self
    }

    pub fn indent(mut self, value: i64) -> Self {
        self.indent = Some(value);
        self
    }

    pub fn number_format(mut self, value: NumberFormatModel) -> Self {
        self.number_format = Some(value);
        self
    }

    pub fn text_format(mut self, value: TextFormatModel) -> Self {
        self.text_format = Some(value);
        self
    }

    pub fn text_rotation(mut self, value: CellFormatTextRotation) -> Self {
        self.text_rotation = Some(value);
        self
    }

    pub fn vertical_alignment(mut self, value: CellFormatVerticalAlignment) -> Self {
        self.vertical_alignment = Some(value);
        self
    }

    pub fn wrap_strategy(mut self, value: WrapStrategy) -> Self {
        self.wrap_strategy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CellFormat`].
    pub fn build(self) -> Result<CellFormat, BuildError> {
        Ok(CellFormat {
            background_color_hex: self.background_color_hex,
            background_color_theme: self.background_color_theme,
            borders: self.borders,
            horizontal_alignment: self.horizontal_alignment,
            indent: self.indent,
            number_format: self.number_format,
            text_format: self.text_format,
            text_rotation: self.text_rotation,
            vertical_alignment: self.vertical_alignment,
            wrap_strategy: self.wrap_strategy,
        })
    }
}
