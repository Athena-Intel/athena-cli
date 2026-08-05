pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Sheet {
    /// Column count (Defaults to 100)
    #[serde(rename = "columnCount")]
    #[serde(default)]
    pub column_count: i64,
    /// Array of column properties, one per column (1-indexed). REQUIRED when changing column widths. Each entry defines width, visibility for that column. Example: To set column C (index 3) width to 150px: columnMetadata[3] = DimensionProperties(size=150). Example: To hide column B: columnMetadata[2] = DimensionProperties(hiddenByUser=True). Common widths: 100px (default), 150px (wide text), 200px (very wide), 50px (narrow IDs)
    #[serde(rename = "columnMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<Option<DimensionProperties>>>,
    #[serde(rename = "frozenColumnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_column_count: Option<i64>,
    #[serde(rename = "frozenRowCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_row_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The order of the new sheet
    #[serde(default)]
    pub index: i64,
    /// List of merged cell ranges in the sheet. Each merge combines multiple cells into a single cell. The top-left cell (startRowIndex, startColumnIndex) becomes the anchor cell that displays the content. Example: To merge cells A1:C3, use GridRange(startRowIndex=1, endRowIndex=3, startColumnIndex=1, endColumnIndex=3). Defaults to empty list (no merged cells).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges: Option<Vec<GridRange>>,
    /// Row count (Defaults to 1000)
    #[serde(rename = "rowCount")]
    #[serde(default)]
    pub row_count: i64,
    /// Array of row properties, one per row (1-indexed). REQUIRED when changing row heights. Each entry defines height, visibility for that row. Example: To set row 5 height to 50px: rowMetadata[5] = DimensionProperties(size=50). Example: To hide row 3: rowMetadata[3] = DimensionProperties(hiddenByUser=True). Common heights: 21px (default), 30px (comfortable), 50px (tall headers)
    #[serde(rename = "rowMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_metadata: Option<Vec<Option<DimensionProperties>>>,
    /// Sheet ID (required)
    #[serde(rename = "sheetId")]
    #[serde(default)]
    pub sheet_id: i64,
    #[serde(rename = "showGridLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_grid_lines: Option<bool>,
    /// Tab color as hex string (e.g., '#FF0000'). Use either tabColor_hex or tabColor_theme, not both.
    #[serde(rename = "tabColor_hex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_color_hex: Option<String>,
    /// Tab color as theme reference. Use either tabColor_hex or tabColor_theme, not both.
    #[serde(rename = "tabColor_theme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_color_theme: Option<ThemeColor>,
    #[serde(default)]
    pub title: String,
}

impl Sheet {
    pub fn builder() -> SheetBuilder {
        <SheetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SheetBuilder {
    column_count: Option<i64>,
    column_metadata: Option<Vec<Option<DimensionProperties>>>,
    frozen_column_count: Option<i64>,
    frozen_row_count: Option<i64>,
    hidden: Option<bool>,
    index: Option<i64>,
    merges: Option<Vec<GridRange>>,
    row_count: Option<i64>,
    row_metadata: Option<Vec<Option<DimensionProperties>>>,
    sheet_id: Option<i64>,
    show_grid_lines: Option<bool>,
    tab_color_hex: Option<String>,
    tab_color_theme: Option<ThemeColor>,
    title: Option<String>,
}

impl SheetBuilder {
    pub fn column_count(mut self, value: i64) -> Self {
        self.column_count = Some(value);
        self
    }

    pub fn column_metadata(mut self, value: Vec<Option<DimensionProperties>>) -> Self {
        self.column_metadata = Some(value);
        self
    }

    pub fn frozen_column_count(mut self, value: i64) -> Self {
        self.frozen_column_count = Some(value);
        self
    }

    pub fn frozen_row_count(mut self, value: i64) -> Self {
        self.frozen_row_count = Some(value);
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn merges(mut self, value: Vec<GridRange>) -> Self {
        self.merges = Some(value);
        self
    }

    pub fn row_count(mut self, value: i64) -> Self {
        self.row_count = Some(value);
        self
    }

    pub fn row_metadata(mut self, value: Vec<Option<DimensionProperties>>) -> Self {
        self.row_metadata = Some(value);
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn show_grid_lines(mut self, value: bool) -> Self {
        self.show_grid_lines = Some(value);
        self
    }

    pub fn tab_color_hex(mut self, value: impl Into<String>) -> Self {
        self.tab_color_hex = Some(value.into());
        self
    }

    pub fn tab_color_theme(mut self, value: ThemeColor) -> Self {
        self.tab_color_theme = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Sheet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column_count`](SheetBuilder::column_count)
    /// - [`index`](SheetBuilder::index)
    /// - [`row_count`](SheetBuilder::row_count)
    /// - [`sheet_id`](SheetBuilder::sheet_id)
    /// - [`title`](SheetBuilder::title)
    pub fn build(self) -> Result<Sheet, BuildError> {
        Ok(Sheet {
            column_count: self.column_count.ok_or_else(|| BuildError::missing_field("column_count"))?,
            column_metadata: self.column_metadata,
            frozen_column_count: self.frozen_column_count,
            frozen_row_count: self.frozen_row_count,
            hidden: self.hidden,
            index: self.index.ok_or_else(|| BuildError::missing_field("index"))?,
            merges: self.merges,
            row_count: self.row_count.ok_or_else(|| BuildError::missing_field("row_count"))?,
            row_metadata: self.row_metadata,
            sheet_id: self.sheet_id.ok_or_else(|| BuildError::missing_field("sheet_id"))?,
            show_grid_lines: self.show_grid_lines,
            tab_color_hex: self.tab_color_hex,
            tab_color_theme: self.tab_color_theme,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
