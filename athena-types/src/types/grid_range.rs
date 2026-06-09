pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GridRange {
    /// End column index (1-based, INCLUSIVE). Last column of the range. For range A1:C5, this would be 3. For single columns, equals startColumnIndex.
    #[serde(rename = "endColumnIndex")]
    #[serde(default)]
    pub end_column_index: i64,
    /// End row index (1-based, INCLUSIVE). Last row of the range. For a range A1:A5, this would be 5. For single cells, equals startRowIndex.
    #[serde(rename = "endRowIndex")]
    #[serde(default)]
    pub end_row_index: i64,
    /// Start column index (1-based, INCLUSIVE). First column of the range. A=1, B=2, C=3, etc. For range A1:C5, this would be 1.
    #[serde(rename = "startColumnIndex")]
    #[serde(default)]
    pub start_column_index: i64,
    /// Start row index (1-based, INCLUSIVE). First row of the range. For A1 this would be 1, for A5 this would be 5.
    #[serde(rename = "startRowIndex")]
    #[serde(default)]
    pub start_row_index: i64,
}

impl GridRange {
    pub fn builder() -> GridRangeBuilder {
        <GridRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GridRangeBuilder {
    end_column_index: Option<i64>,
    end_row_index: Option<i64>,
    start_column_index: Option<i64>,
    start_row_index: Option<i64>,
}

impl GridRangeBuilder {
    pub fn end_column_index(mut self, value: i64) -> Self {
        self.end_column_index = Some(value);
        self
    }

    pub fn end_row_index(mut self, value: i64) -> Self {
        self.end_row_index = Some(value);
        self
    }

    pub fn start_column_index(mut self, value: i64) -> Self {
        self.start_column_index = Some(value);
        self
    }

    pub fn start_row_index(mut self, value: i64) -> Self {
        self.start_row_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GridRange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_column_index`](GridRangeBuilder::end_column_index)
    /// - [`end_row_index`](GridRangeBuilder::end_row_index)
    /// - [`start_column_index`](GridRangeBuilder::start_column_index)
    /// - [`start_row_index`](GridRangeBuilder::start_row_index)
    pub fn build(self) -> Result<GridRange, BuildError> {
        Ok(GridRange {
            end_column_index: self.end_column_index.ok_or_else(|| BuildError::missing_field("end_column_index"))?,
            end_row_index: self.end_row_index.ok_or_else(|| BuildError::missing_field("end_row_index"))?,
            start_column_index: self.start_column_index.ok_or_else(|| BuildError::missing_field("start_column_index"))?,
            start_row_index: self.start_row_index.ok_or_else(|| BuildError::missing_field("start_row_index"))?,
        })
    }
}
