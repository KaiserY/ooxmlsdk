use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;
use super::fill::FillProperties;
use super::line::{LineFill, LineProperties};
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
  pub(crate) fill_properties: Option<FillProperties>,
  pub(crate) borders: TableCellBorders,
  pub(crate) vertical: Option<a::TextVerticalValues>,
  pub(crate) anchor: a::TextAnchoringTypeValues,
  pub(crate) anchor_center: bool,
  pub(crate) horizontal_overflow: a::TextHorizontalOverflowValues,
  pub(crate) text_body: Option<TextBody>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableCellBorders {
  pub(crate) left: Option<LineProperties>,
  pub(crate) right: Option<LineProperties>,
  pub(crate) top: Option<LineProperties>,
  pub(crate) bottom: Option<LineProperties>,
  pub(crate) top_left_to_bottom_right: Option<LineProperties>,
  pub(crate) bottom_left_to_top_right: Option<LineProperties>,
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
      fill_properties: None,
      borders: TableCellBorders::default(),
      vertical: None,
      anchor: a::TextAnchoringTypeValues::Top,
      anchor_center: false,
      horizontal_overflow: a::TextHorizontalOverflowValues::Clip,
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
    self.vertical = properties.vertical;
    self.anchor = properties.anchor.unwrap_or(a::TextAnchoringTypeValues::Top);
    self.anchor_center = properties
      .anchor_center
      .is_some_and(|value| value.as_bool());
    self.horizontal_overflow = properties
      .horizontal_overflow
      .unwrap_or(a::TextHorizontalOverflowValues::Clip);
    self.fill_properties = properties
      .table_cell_properties_choice
      .as_ref()
      .map(FillProperties::from_table_cell_properties_choice);
    self.borders = TableCellBorders {
      left: properties
        .left_border_line_properties
        .as_deref()
        .and_then(left_border_line_properties),
      right: properties
        .right_border_line_properties
        .as_deref()
        .and_then(right_border_line_properties),
      top: properties
        .top_border_line_properties
        .as_deref()
        .and_then(top_border_line_properties),
      bottom: properties
        .bottom_border_line_properties
        .as_deref()
        .and_then(bottom_border_line_properties),
      top_left_to_bottom_right: properties
        .top_left_to_bottom_right_border_line_properties
        .as_deref()
        .and_then(top_left_to_bottom_right_border_line_properties),
      bottom_left_to_top_right: properties
        .bottom_left_to_top_right_border_line_properties
        .as_deref()
        .and_then(bottom_left_to_top_right_border_line_properties),
    };
  }
}

fn left_border_line_properties(properties: &a::LeftBorderLineProperties) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .left_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::LeftBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::LeftBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::LeftBorderLinePropertiesChoice::GradientFill(fill) => LineFill::Gradient(fill.clone()),
        a::LeftBorderLinePropertiesChoice::PatternFill(fill) => LineFill::Pattern(fill.clone()),
      }),
  )
}

fn right_border_line_properties(
  properties: &a::RightBorderLineProperties,
) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .right_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::RightBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::RightBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::RightBorderLinePropertiesChoice::GradientFill(fill) => LineFill::Gradient(fill.clone()),
        a::RightBorderLinePropertiesChoice::PatternFill(fill) => LineFill::Pattern(fill.clone()),
      }),
  )
}

fn top_border_line_properties(properties: &a::TopBorderLineProperties) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .top_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::TopBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::TopBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::TopBorderLinePropertiesChoice::GradientFill(fill) => LineFill::Gradient(fill.clone()),
        a::TopBorderLinePropertiesChoice::PatternFill(fill) => LineFill::Pattern(fill.clone()),
      }),
  )
}

fn bottom_border_line_properties(
  properties: &a::BottomBorderLineProperties,
) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .bottom_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::BottomBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::BottomBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::BottomBorderLinePropertiesChoice::GradientFill(fill) => LineFill::Gradient(fill.clone()),
        a::BottomBorderLinePropertiesChoice::PatternFill(fill) => LineFill::Pattern(fill.clone()),
      }),
  )
}

fn top_left_to_bottom_right_border_line_properties(
  properties: &a::TopLeftToBottomRightBorderLineProperties,
) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .top_left_to_bottom_right_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::TopLeftToBottomRightBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::TopLeftToBottomRightBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::TopLeftToBottomRightBorderLinePropertiesChoice::GradientFill(fill) => {
          LineFill::Gradient(fill.clone())
        }
        a::TopLeftToBottomRightBorderLinePropertiesChoice::PatternFill(fill) => {
          LineFill::Pattern(fill.clone())
        }
      }),
  )
}

fn bottom_left_to_top_right_border_line_properties(
  properties: &a::BottomLeftToTopRightBorderLineProperties,
) -> Option<LineProperties> {
  table_border_line_properties(
    properties.width.map(i64::from),
    properties
      .bottom_left_to_top_right_border_line_properties_choice1
      .as_ref()
      .map(|choice| match choice {
        a::BottomLeftToTopRightBorderLinePropertiesChoice::NoFill(_) => LineFill::None,
        a::BottomLeftToTopRightBorderLinePropertiesChoice::SolidFill(fill) => solid_line_fill(fill),
        a::BottomLeftToTopRightBorderLinePropertiesChoice::GradientFill(fill) => {
          LineFill::Gradient(fill.clone())
        }
        a::BottomLeftToTopRightBorderLinePropertiesChoice::PatternFill(fill) => {
          LineFill::Pattern(fill.clone())
        }
      }),
  )
}

fn table_border_line_properties(
  width_emu: Option<i64>,
  fill: Option<LineFill>,
) -> Option<LineProperties> {
  let fill = fill.unwrap_or(LineFill::Unspecified);
  if fill == LineFill::Unspecified && width_emu.is_none() {
    None
  } else {
    Some(LineProperties {
      fill,
      width_emu,
      placeholder_color: None,
    })
  }
}

fn solid_line_fill(fill: &a::SolidFill) -> LineFill {
  LineFill::Solid(
    fill
      .solid_fill_choice
      .as_ref()
      .and_then(Color::from_solid_fill_choice),
  )
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
