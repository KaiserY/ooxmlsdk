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
      vertical_banding: false,
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
    row_band_size: usize,
    explicit_first_row: bool,
  ) -> Self {
    let mut mask = Self::default();
    if look.first_row && (explicit_first_row || row_index == 0) {
      mask.first_row = true;
    } else if look.last_row && row_index + 1 == row_count {
      mask.last_row = true;
    }
    if !mask.has_row_style() && look.horizontal_banding && row_band_size > 0 {
      // The emphasized first row is outside the alternating band sequence.
      // ECMA-376 Part 1 §17.7.6.7 groups the remaining rows into authored-size
      // bands; [MS-OI29500] makes size zero mean no row band formatting.
      let band_position = row_index.saturating_sub(usize::from(look.first_row));
      if (band_position / row_band_size).is_multiple_of(2) {
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
    column_band_size: usize,
  ) -> Self {
    let mut mask = Self::default();
    if look.first_column && cell_index == 0 {
      mask.first_column = true;
    } else if look.last_column && cell_index + 1 == cell_count {
      mask.last_column = true;
    }
    if !mask.has_column_style() && look.vertical_banding && column_band_size > 0 {
      // As with row bands, the emphasized first column does not consume the
      // first authored band. A zero width disables column band formatting in
      // Word instead of falling back to one-column alternation.
      let band_position = cell_index.saturating_sub(usize::from(look.first_column));
      if (band_position / column_band_size).is_multiple_of(2) {
        mask.odd_vertical_band = true;
      } else {
        mask.even_vertical_band = true;
      }
    }
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
  row_band_size: usize,
  explicit_first_row: bool,
) -> bool {
  TableConditionalStyleMask::from_row_position(
    look,
    row_index,
    row_count,
    row_band_size,
    explicit_first_row,
  )
  .row_condition_applies(condition)
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CellStyleConditionContext {
  pub look: TableLookModel,
  pub row_index: usize,
  pub row_count: usize,
  pub cell_index: usize,
  pub cell_count: usize,
  pub row_band_size: usize,
  pub column_band_size: usize,
  pub explicit_first_row: bool,
}

pub(super) fn cell_style_condition_applies(
  condition: w::TableStyleOverrideValues,
  context: CellStyleConditionContext,
) -> bool {
  TableConditionalStyleMask::from_row_position(
    context.look,
    context.row_index,
    context.row_count,
    context.row_band_size,
    context.explicit_first_row,
  )
  .with_cell_mask(TableConditionalStyleMask::from_cell_position(
    context.look,
    context.cell_index,
    context.cell_count,
    context.column_band_size,
  ))
  .cell_condition_applies(condition)
}

pub(super) fn conditional_style_priority(condition: w::TableStyleOverrideValues) -> u8 {
  // [MS-OI29500] Part 1 §17.7.6(c) documents the order used by Office,
  // where later regions override earlier ones. This intentionally differs
  // from both XML document order and the ISO ordering of row/column regions.
  match condition {
    w::TableStyleOverrideValues::WholeTable => 0,
    w::TableStyleOverrideValues::Band1Horizontal => 1,
    w::TableStyleOverrideValues::Band2Horizontal => 2,
    w::TableStyleOverrideValues::Band1Vertical => 3,
    w::TableStyleOverrideValues::Band2Vertical => 4,
    w::TableStyleOverrideValues::FirstColumn => 5,
    w::TableStyleOverrideValues::LastColumn => 6,
    w::TableStyleOverrideValues::FirstRow => 7,
    w::TableStyleOverrideValues::LastRow => 8,
    w::TableStyleOverrideValues::NorthWestCell => 9,
    w::TableStyleOverrideValues::NorthEastCell => 10,
    w::TableStyleOverrideValues::SouthWestCell => 11,
    w::TableStyleOverrideValues::SouthEastCell => 12,
  }
}
