use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::text_body::TextBody;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableProperties {
  pub(crate) style_id: Option<String>,
  pub(crate) first_row: bool,
  pub(crate) first_column: bool,
  pub(crate) last_row: bool,
  pub(crate) last_column: bool,
  pub(crate) band_row: bool,
  pub(crate) band_column: bool,
  pub(crate) grid: Vec<i64>,
  pub(crate) rows: Vec<TableRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableRow {
  pub(crate) height: i64,
  pub(crate) cells: Vec<TableCell>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableCell {
  pub(crate) row_span: Option<i32>,
  pub(crate) grid_span: Option<i32>,
  pub(crate) horizontal_merge: bool,
  pub(crate) vertical_merge: bool,
  pub(crate) margins: TableCellMargins,
  pub(crate) text_body: Option<TextBody>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TableCellMargins {
  pub(crate) left: i32,
  pub(crate) right: i32,
  pub(crate) top: i32,
  pub(crate) bottom: i32,
}

impl Default for TableProperties {
  fn default() -> Self {
    Self {
      style_id: None,
      first_row: false,
      first_column: false,
      last_row: false,
      last_column: false,
      band_row: false,
      band_column: false,
      grid: Vec::new(),
      rows: Vec::new(),
    }
  }
}

impl Default for TableCellMargins {
  fn default() -> Self {
    Self {
      left: 91_440,
      right: 91_440,
      top: 45_720,
      bottom: 45_720,
    }
  }
}

impl TableProperties {
  pub(crate) fn from_dml_table(source: &a::Table) -> Self {
    // Source: LibreOffice oox/source/drawingml/table/tablecontext.cxx
    // TableContext owns tblPr, tblGrid/gridCol, and tr/tc import after
    // GraphicalObjectFrameContext dispatches a table graphicData URI.
    let mut table = Self {
      grid: source
        .table_grid
        .grid_column
        .iter()
        .map(|column| column.width.to_emu())
        .collect(),
      rows: source.table_row.iter().map(TableRow::from_dml).collect(),
      ..Self::default()
    };
    if let Some(properties) = &source.table_properties {
      table.apply_dml_properties(properties);
    }
    table
  }

  fn apply_dml_properties(&mut self, properties: &a::TableProperties) {
    self.first_row = properties.first_row.is_some_and(|value| value.as_bool());
    self.first_column = properties.first_column.is_some_and(|value| value.as_bool());
    self.last_row = properties.last_row.is_some_and(|value| value.as_bool());
    self.last_column = properties.last_column.is_some_and(|value| value.as_bool());
    self.band_row = properties.band_row.is_some_and(|value| value.as_bool());
    self.band_column = properties.band_column.is_some_and(|value| value.as_bool());
    self.style_id = match &properties.table_properties_choice3 {
      Some(a::TablePropertiesChoice3::TableStyle(style)) => Some(style.style_id.clone()),
      Some(a::TablePropertiesChoice3::TableStyleId(style_id)) => Some(style_id.clone()),
      None => None,
    };
  }
}

impl TableRow {
  fn from_dml(source: &a::TableRow) -> Self {
    // Source: LibreOffice oox/source/drawingml/table/tablerowcontext.cxx
    // TableRowContext owns ordered table-cell import.
    Self {
      height: source.height.to_emu(),
      cells: source.table_cell.iter().map(TableCell::from_dml).collect(),
    }
  }
}

impl TableCell {
  fn from_dml(source: &a::TableCell) -> Self {
    // Source: LibreOffice oox/source/drawingml/table/tablecellcontext.cxx
    // TableCellContext owns merge flags, margins, and typed TextBody import.
    let mut cell = Self {
      row_span: source.row_span,
      grid_span: source.grid_span,
      horizontal_merge: source.horizontal_merge.is_some_and(|value| value.as_bool()),
      vertical_merge: source.vertical_merge.is_some_and(|value| value.as_bool()),
      margins: TableCellMargins::default(),
      text_body: source.text_body.as_deref().map(TextBody::from_dml),
    };
    if let Some(properties) = &source.table_cell_properties {
      cell.apply_dml_properties(properties);
    }
    cell
  }

  fn apply_dml_properties(&mut self, properties: &a::TableCellProperties) {
    if let Some(value) = properties.left_margin {
      self.margins.left = coordinate32_to_i32_emu(value);
    }
    if let Some(value) = properties.right_margin {
      self.margins.right = coordinate32_to_i32_emu(value);
    }
    if let Some(value) = properties.top_margin {
      self.margins.top = coordinate32_to_i32_emu(value);
    }
    if let Some(value) = properties.bottom_margin {
      self.margins.bottom = coordinate32_to_i32_emu(value);
    }
  }
}

fn coordinate32_to_i32_emu(value: ooxmlsdk::simple_type::Coordinate32Value) -> i32 {
  i32::try_from(value.to_emu()).unwrap_or_else(|_| {
    if value.to_emu().is_negative() {
      i32::MIN
    } else {
      i32::MAX
    }
  })
}
