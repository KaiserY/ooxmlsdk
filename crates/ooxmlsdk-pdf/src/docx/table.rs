use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

#[derive(Clone, Copy, Debug)]
pub(super) struct TableLookModel {
  pub first_row: bool,
  pub last_row: bool,
  pub first_column: bool,
  pub last_column: bool,
  pub horizontal_banding: bool,
  pub vertical_banding: bool,
}

impl Default for TableLookModel {
  fn default() -> Self {
    Self {
      first_row: true,
      last_row: false,
      first_column: true,
      last_column: false,
      horizontal_banding: true,
      vertical_banding: true,
    }
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct TableConditionalStyleMask {
  first_row: bool,
  last_row: bool,
  first_column: bool,
  last_column: bool,
  odd_vertical_band: bool,
  even_vertical_band: bool,
  odd_horizontal_band: bool,
  even_horizontal_band: bool,
  first_row_first_column: bool,
  first_row_last_column: bool,
  last_row_first_column: bool,
  last_row_last_column: bool,
}

impl TableConditionalStyleMask {
  pub(super) fn from_row_position(
    look: TableLookModel,
    row_index: usize,
    row_count: usize,
  ) -> Self {
    let mut mask = Self::default();
    if look.first_row && row_index == 0 {
      mask.first_row = true;
    } else if look.last_row && row_index + 1 == row_count {
      mask.last_row = true;
    }
    if !mask.has_row_style() && look.horizontal_banding {
      let mut band_index = row_index + 1;
      if look.first_row {
        band_index += 1;
      }
      if band_index % 2 == 1 {
        mask.odd_horizontal_band = true;
      } else {
        mask.even_horizontal_band = true;
      }
    }
    mask
  }

  pub(super) fn from_cell_position(
    look: TableLookModel,
    cell_index: usize,
    cell_count: usize,
  ) -> Self {
    let mut mask = Self::default();
    if look.first_column && cell_index == 0 {
      mask.first_column = true;
    } else if look.last_column && cell_index + 1 == cell_count {
      mask.last_column = true;
    }
    if !mask.has_column_style() && look.vertical_banding {
      let mut band_index = cell_index + 1;
      if look.first_column {
        band_index += 1;
      }
      if band_index % 2 == 1 {
        mask.odd_vertical_band = true;
      } else {
        mask.even_vertical_band = true;
      }
    }
    mask
  }

  pub(super) fn from_cnf_style(style: &w::ConditionalFormatStyle) -> Self {
    let mut mask = Self::from_cnf_value(style.val.as_str());
    mask.first_row |= style.first_row.unwrap_or(false);
    mask.last_row |= style.last_row.unwrap_or(false);
    mask.first_column |= style.first_column.unwrap_or(false);
    mask.last_column |= style.last_column.unwrap_or(false);
    mask.odd_vertical_band |= style.odd_vertical_band.unwrap_or(false);
    mask.even_vertical_band |= style.even_vertical_band.unwrap_or(false);
    mask.odd_horizontal_band |= style.odd_horizontal_band.unwrap_or(false);
    mask.even_horizontal_band |= style.even_horizontal_band.unwrap_or(false);
    mask.first_row_first_column |= style.first_row_first_column.unwrap_or(false);
    mask.first_row_last_column |= style.first_row_last_column.unwrap_or(false);
    mask.last_row_first_column |= style.last_row_first_column.unwrap_or(false);
    mask.last_row_last_column |= style.last_row_last_column.unwrap_or(false);
    mask
  }

  pub(super) fn with_cell_mask(mut self, cell_mask: Self) -> Self {
    self.first_column |= cell_mask.first_column;
    self.last_column |= cell_mask.last_column;
    self.odd_vertical_band |= cell_mask.odd_vertical_band;
    self.even_vertical_band |= cell_mask.even_vertical_band;
    self.first_row_first_column |= cell_mask.first_row_first_column;
    self.first_row_last_column |= cell_mask.first_row_last_column;
    self.last_row_first_column |= cell_mask.last_row_first_column;
    self.last_row_last_column |= cell_mask.last_row_last_column;
    self.add_corner_conditions();
    self
  }

  pub(super) fn row_condition_applies(self, condition: w::TableStyleOverrideValues) -> bool {
    match condition {
      w::TableStyleOverrideValues::WholeTable => true,
      w::TableStyleOverrideValues::FirstRow => self.first_row,
      w::TableStyleOverrideValues::LastRow => self.last_row,
      w::TableStyleOverrideValues::Band1Horizontal => self.odd_horizontal_band,
      w::TableStyleOverrideValues::Band2Horizontal => self.even_horizontal_band,
      _ => false,
    }
  }

  pub(super) fn cell_condition_applies(self, condition: w::TableStyleOverrideValues) -> bool {
    match condition {
      w::TableStyleOverrideValues::WholeTable => true,
      w::TableStyleOverrideValues::FirstRow => self.first_row,
      w::TableStyleOverrideValues::LastRow => self.last_row,
      w::TableStyleOverrideValues::FirstColumn => self.first_column,
      w::TableStyleOverrideValues::LastColumn => self.last_column,
      w::TableStyleOverrideValues::Band1Horizontal => self.odd_horizontal_band,
      w::TableStyleOverrideValues::Band2Horizontal => self.even_horizontal_band,
      w::TableStyleOverrideValues::Band1Vertical => self.odd_vertical_band,
      w::TableStyleOverrideValues::Band2Vertical => self.even_vertical_band,
      w::TableStyleOverrideValues::NorthWestCell => self.first_row_first_column,
      w::TableStyleOverrideValues::NorthEastCell => self.first_row_last_column,
      w::TableStyleOverrideValues::SouthWestCell => self.last_row_first_column,
      w::TableStyleOverrideValues::SouthEastCell => self.last_row_last_column,
    }
  }

  fn from_cnf_value(value: &str) -> Self {
    let mut mask = Self::default();
    let mut bits = value.chars();
    mask.first_row = matches!(bits.next(), Some('1'));
    mask.last_row = matches!(bits.next(), Some('1'));
    mask.first_column = matches!(bits.next(), Some('1'));
    mask.last_column = matches!(bits.next(), Some('1'));
    mask.odd_vertical_band = matches!(bits.next(), Some('1'));
    mask.even_vertical_band = matches!(bits.next(), Some('1'));
    mask.odd_horizontal_band = matches!(bits.next(), Some('1'));
    mask.even_horizontal_band = matches!(bits.next(), Some('1'));
    mask.first_row_first_column = matches!(bits.next(), Some('1'));
    mask.first_row_last_column = matches!(bits.next(), Some('1'));
    mask.last_row_first_column = matches!(bits.next(), Some('1'));
    mask.last_row_last_column = matches!(bits.next(), Some('1'));
    mask
  }

  fn has_row_style(self) -> bool {
    self.first_row || self.last_row
  }

  fn has_column_style(self) -> bool {
    self.first_column || self.last_column
  }

  fn add_corner_conditions(&mut self) {
    if self.first_row && self.first_column {
      self.first_row_first_column = true;
    } else if self.last_row && self.first_column {
      self.last_row_first_column = true;
    } else if self.first_row && self.last_column {
      self.first_row_last_column = true;
    } else if self.last_row && self.last_column {
      self.last_row_last_column = true;
    }
  }
}

pub(super) fn row_style_condition_applies(
  condition: w::TableStyleOverrideValues,
  look: TableLookModel,
  row_index: usize,
  row_count: usize,
) -> bool {
  TableConditionalStyleMask::from_row_position(look, row_index, row_count)
    .row_condition_applies(condition)
}

pub(super) fn cell_style_condition_applies(
  condition: w::TableStyleOverrideValues,
  look: TableLookModel,
  row_index: usize,
  row_count: usize,
  cell_index: usize,
  cell_count: usize,
) -> bool {
  TableConditionalStyleMask::from_row_position(look, row_index, row_count)
    .with_cell_mask(TableConditionalStyleMask::from_cell_position(
      look, cell_index, cell_count,
    ))
    .cell_condition_applies(condition)
}
