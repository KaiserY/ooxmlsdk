use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;
use super::fill::FillProperties;
use super::line::{LineFill, LineProperties};
use super::shape::{FontStyleReference, ShapeStyleReference};
use super::text_body::TextBody;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableProperties {
  pub(crate) style_id: Option<String>,
  pub(crate) inline_style: Option<TableStyle>,
  pub(crate) first_row: bool,
  pub(crate) first_column: bool,
  pub(crate) last_row: bool,
  pub(crate) last_column: bool,
  pub(crate) band_row: bool,
  pub(crate) band_column: bool,
  pub(crate) grid: Vec<i64>,
  pub(crate) rows: Vec<TableRow>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyleList {
  pub(crate) path: Option<String>,
  pub(crate) default_style_id: Option<String>,
  pub(crate) styles: Vec<TableStyle>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyle {
  pub(crate) style_id: Option<String>,
  pub(crate) style_name: Option<String>,
  pub(crate) table_background: TableStylePart,
  pub(crate) whole_table: TableStylePart,
  pub(crate) band1_horizontal: TableStylePart,
  pub(crate) band2_horizontal: TableStylePart,
  pub(crate) band1_vertical: TableStylePart,
  pub(crate) band2_vertical: TableStylePart,
  pub(crate) last_column: TableStylePart,
  pub(crate) first_column: TableStylePart,
  pub(crate) last_row: TableStylePart,
  pub(crate) southeast_cell: TableStylePart,
  pub(crate) southwest_cell: TableStylePart,
  pub(crate) first_row: TableStylePart,
  pub(crate) northeast_cell: TableStylePart,
  pub(crate) northwest_cell: TableStylePart,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStylePart {
  pub(crate) fill_properties: Option<FillProperties>,
  pub(crate) fill_reference: Option<ShapeStyleReference>,
  pub(crate) borders: TableStyleBorders,
  pub(crate) text: TableStyleTextProperties,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyleBorders {
  pub(crate) left: Option<LineProperties>,
  pub(crate) left_reference: Option<ShapeStyleReference>,
  pub(crate) right: Option<LineProperties>,
  pub(crate) right_reference: Option<ShapeStyleReference>,
  pub(crate) top: Option<LineProperties>,
  pub(crate) top_reference: Option<ShapeStyleReference>,
  pub(crate) bottom: Option<LineProperties>,
  pub(crate) bottom_reference: Option<ShapeStyleReference>,
  pub(crate) inside_horizontal: Option<LineProperties>,
  pub(crate) inside_horizontal_reference: Option<ShapeStyleReference>,
  pub(crate) inside_vertical: Option<LineProperties>,
  pub(crate) inside_vertical_reference: Option<ShapeStyleReference>,
  pub(crate) top_left_to_bottom_right: Option<LineProperties>,
  pub(crate) top_left_to_bottom_right_reference: Option<ShapeStyleReference>,
  pub(crate) bottom_left_to_top_right: Option<LineProperties>,
  pub(crate) bottom_left_to_top_right_reference: Option<ShapeStyleReference>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyleTextProperties {
  pub(crate) bold: Option<a::BooleanStyleValues>,
  pub(crate) italic: Option<a::BooleanStyleValues>,
  pub(crate) fonts: TableStyleTextFonts,
  pub(crate) font_reference: Option<FontStyleReference>,
  pub(crate) color: Option<Color>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyleTextFonts {
  pub(crate) latin: Option<String>,
  pub(crate) east_asian: Option<String>,
  pub(crate) complex_script: Option<String>,
  pub(crate) symbol: Option<String>,
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
      inline_style: None,
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
      Some(a::TablePropertiesChoice3::TableStyle(style)) => {
        self.inline_style = Some(TableStyle::from_dml_table_style(style));
        Some(style.style_id.clone())
      }
      Some(a::TablePropertiesChoice3::TableStyleId(style_id)) => Some(style_id.clone()),
      None => None,
    };
  }
}

impl TableStyleList {
  pub(crate) fn from_dml(path: Option<String>, source: &a::TableStyleList) -> Self {
    Self {
      path,
      default_style_id: (!source.default.is_empty()).then(|| source.default.clone()),
      styles: source
        .table_style_entry
        .iter()
        .map(TableStyle::from_dml_table_style_entry)
        .collect(),
    }
  }

  pub(crate) fn style(&self, style_id: Option<&str>) -> Option<&TableStyle> {
    let style_id = style_id
      .filter(|style_id| !style_id.is_empty())
      .or(self.default_style_id.as_deref())?;
    self
      .styles
      .iter()
      .find(|style| style.style_id.as_deref() == Some(style_id))
  }
}

trait TableStyleSource {
  fn style_id(&self) -> &str;
  fn style_name(&self) -> &str;
  fn table_background(&self) -> Option<&a::TableBackground>;
  fn whole_table(&self) -> Option<&a::WholeTable>;
  fn band1_horizontal(&self) -> Option<&a::Band1Horizontal>;
  fn band2_horizontal(&self) -> Option<&a::Band2Horizontal>;
  fn band1_vertical(&self) -> Option<&a::Band1Vertical>;
  fn band2_vertical(&self) -> Option<&a::Band2Vertical>;
  fn last_column(&self) -> Option<&a::LastColumn>;
  fn first_column(&self) -> Option<&a::FirstColumn>;
  fn last_row(&self) -> Option<&a::LastRow>;
  fn southeast_cell(&self) -> Option<&a::SoutheastCell>;
  fn southwest_cell(&self) -> Option<&a::SouthwestCell>;
  fn first_row(&self) -> Option<&a::FirstRow>;
  fn northeast_cell(&self) -> Option<&a::NortheastCell>;
  fn northwest_cell(&self) -> Option<&a::NorthwestCell>;
}

impl TableStyleSource for a::TableStyle {
  fn style_id(&self) -> &str {
    &self.style_id
  }

  fn style_name(&self) -> &str {
    &self.style_name
  }

  fn table_background(&self) -> Option<&a::TableBackground> {
    self.table_background.as_deref()
  }

  fn whole_table(&self) -> Option<&a::WholeTable> {
    self.whole_table.as_deref()
  }

  fn band1_horizontal(&self) -> Option<&a::Band1Horizontal> {
    self.band1_horizontal.as_deref()
  }

  fn band2_horizontal(&self) -> Option<&a::Band2Horizontal> {
    self.band2_horizontal.as_deref()
  }

  fn band1_vertical(&self) -> Option<&a::Band1Vertical> {
    self.band1_vertical.as_deref()
  }

  fn band2_vertical(&self) -> Option<&a::Band2Vertical> {
    self.band2_vertical.as_deref()
  }

  fn last_column(&self) -> Option<&a::LastColumn> {
    self.last_column.as_deref()
  }

  fn first_column(&self) -> Option<&a::FirstColumn> {
    self.first_column.as_deref()
  }

  fn last_row(&self) -> Option<&a::LastRow> {
    self.last_row.as_deref()
  }

  fn southeast_cell(&self) -> Option<&a::SoutheastCell> {
    self.southeast_cell.as_deref()
  }

  fn southwest_cell(&self) -> Option<&a::SouthwestCell> {
    self.southwest_cell.as_deref()
  }

  fn first_row(&self) -> Option<&a::FirstRow> {
    self.first_row.as_deref()
  }

  fn northeast_cell(&self) -> Option<&a::NortheastCell> {
    self.northeast_cell.as_deref()
  }

  fn northwest_cell(&self) -> Option<&a::NorthwestCell> {
    self.northwest_cell.as_deref()
  }
}

impl TableStyleSource for a::TableStyleEntry {
  fn style_id(&self) -> &str {
    &self.style_id
  }

  fn style_name(&self) -> &str {
    &self.style_name
  }

  fn table_background(&self) -> Option<&a::TableBackground> {
    self.table_background.as_deref()
  }

  fn whole_table(&self) -> Option<&a::WholeTable> {
    self.whole_table.as_deref()
  }

  fn band1_horizontal(&self) -> Option<&a::Band1Horizontal> {
    self.band1_horizontal.as_deref()
  }

  fn band2_horizontal(&self) -> Option<&a::Band2Horizontal> {
    self.band2_horizontal.as_deref()
  }

  fn band1_vertical(&self) -> Option<&a::Band1Vertical> {
    self.band1_vertical.as_deref()
  }

  fn band2_vertical(&self) -> Option<&a::Band2Vertical> {
    self.band2_vertical.as_deref()
  }

  fn last_column(&self) -> Option<&a::LastColumn> {
    self.last_column.as_deref()
  }

  fn first_column(&self) -> Option<&a::FirstColumn> {
    self.first_column.as_deref()
  }

  fn last_row(&self) -> Option<&a::LastRow> {
    self.last_row.as_deref()
  }

  fn southeast_cell(&self) -> Option<&a::SoutheastCell> {
    self.southeast_cell.as_deref()
  }

  fn southwest_cell(&self) -> Option<&a::SouthwestCell> {
    self.southwest_cell.as_deref()
  }

  fn first_row(&self) -> Option<&a::FirstRow> {
    self.first_row.as_deref()
  }

  fn northeast_cell(&self) -> Option<&a::NortheastCell> {
    self.northeast_cell.as_deref()
  }

  fn northwest_cell(&self) -> Option<&a::NorthwestCell> {
    self.northwest_cell.as_deref()
  }
}

trait TablePartStyleSource {
  fn table_cell_text_style(&self) -> Option<&a::TableCellTextStyle>;
  fn table_cell_style(&self) -> Option<&a::TableCellStyle>;
}

macro_rules! impl_table_part_style_source {
  ($($ty:ty),+ $(,)?) => {
    $(
      impl TablePartStyleSource for $ty {
        fn table_cell_text_style(&self) -> Option<&a::TableCellTextStyle> {
          self.table_cell_text_style.as_deref()
        }

        fn table_cell_style(&self) -> Option<&a::TableCellStyle> {
          self.table_cell_style.as_deref()
        }
      }
    )+
  };
}

impl_table_part_style_source!(
  a::WholeTable,
  a::Band1Horizontal,
  a::Band2Horizontal,
  a::Band1Vertical,
  a::Band2Vertical,
  a::LastColumn,
  a::FirstColumn,
  a::LastRow,
  a::SoutheastCell,
  a::SouthwestCell,
  a::FirstRow,
  a::NortheastCell,
  a::NorthwestCell,
);

impl TableStyle {
  fn from_dml_table_style(source: &a::TableStyle) -> Self {
    Self::from_style_source(source)
  }

  fn from_dml_table_style_entry(source: &a::TableStyleEntry) -> Self {
    Self::from_style_source(source)
  }

  fn from_style_source(source: &impl TableStyleSource) -> Self {
    Self {
      style_id: (!source.style_id().is_empty()).then(|| source.style_id().to_string()),
      style_name: (!source.style_name().is_empty()).then(|| source.style_name().to_string()),
      table_background: source
        .table_background()
        .map(TableStylePart::from_table_background)
        .unwrap_or_default(),
      whole_table: source
        .whole_table()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      band1_horizontal: source
        .band1_horizontal()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      band2_horizontal: source
        .band2_horizontal()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      band1_vertical: source
        .band1_vertical()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      band2_vertical: source
        .band2_vertical()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      last_column: source
        .last_column()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      first_column: source
        .first_column()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      last_row: source
        .last_row()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      southeast_cell: source
        .southeast_cell()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      southwest_cell: source
        .southwest_cell()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      first_row: source
        .first_row()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      northeast_cell: source
        .northeast_cell()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
      northwest_cell: source
        .northwest_cell()
        .map(TableStylePart::from_table_part)
        .unwrap_or_default(),
    }
  }
}

impl TableStylePart {
  fn from_table_background(source: &a::TableBackground) -> Self {
    let (fill_properties, fill_reference) = match source.table_background_choice1.as_ref() {
      Some(a::TableBackgroundChoice::FillProperties(fill)) => {
        (FillProperties::from_dml_fill_properties(fill), None)
      }
      Some(a::TableBackgroundChoice::FillReference(reference)) => {
        (None, Some(fill_style_reference(reference)))
      }
      None => (None, None),
    };
    Self {
      fill_properties,
      fill_reference,
      ..Self::default()
    }
  }

  fn from_table_part(source: &impl TablePartStyleSource) -> Self {
    let text = source
      .table_cell_text_style()
      .map(TableStyleTextProperties::from_dml)
      .unwrap_or_default();
    let (fill_properties, fill_reference, borders) = source
      .table_cell_style()
      .map(table_cell_style_properties)
      .unwrap_or_default();
    Self {
      fill_properties,
      fill_reference,
      borders,
      text,
    }
  }
}

impl TableStyleTextProperties {
  fn from_dml(source: &a::TableCellTextStyle) -> Self {
    let (fonts, font_reference) =
      table_cell_text_font(source.table_cell_text_style_choice1.as_ref());
    Self {
      bold: source.bold,
      italic: source.italic,
      fonts,
      font_reference,
      color: source
        .table_cell_text_style_choice2
        .as_ref()
        .and_then(table_cell_text_color),
    }
  }

  pub(crate) fn merge_from(&mut self, source: &Self) {
    if source.bold.is_some() {
      self.bold = source.bold;
    }
    if source.italic.is_some() {
      self.italic = source.italic;
    }
    self.fonts.merge_from(&source.fonts);
    if source.font_reference.is_some() {
      self.font_reference = source.font_reference.clone();
    }
    if source.color.is_some() {
      self.color = source.color.clone();
    }
  }
}

impl TableStyleTextFonts {
  fn from_fonts(source: &a::Fonts) -> Self {
    Self {
      latin: text_font_typeface(&source.latin_font.typeface),
      east_asian: text_font_typeface(&source.east_asian_font.typeface),
      complex_script: text_font_typeface(&source.complex_script_font.typeface),
      symbol: None,
    }
  }

  fn merge_from(&mut self, source: &Self) {
    if source.latin.is_some() {
      self.latin = source.latin.clone();
    }
    if source.east_asian.is_some() {
      self.east_asian = source.east_asian.clone();
    }
    if source.complex_script.is_some() {
      self.complex_script = source.complex_script.clone();
    }
    if source.symbol.is_some() {
      self.symbol = source.symbol.clone();
    }
  }
}

fn table_cell_text_font(
  choice: Option<&a::TableCellTextStyleChoice>,
) -> (TableStyleTextFonts, Option<FontStyleReference>) {
  match choice {
    Some(a::TableCellTextStyleChoice::Fonts(fonts)) => {
      (TableStyleTextFonts::from_fonts(fonts), None)
    }
    Some(a::TableCellTextStyleChoice::FontReference(reference)) => (
      TableStyleTextFonts::default(),
      Some(FontStyleReference {
        index: reference.index,
        placeholder_color: reference
          .font_reference_choice
          .as_ref()
          .and_then(Color::from_font_reference_choice),
      }),
    ),
    None => (TableStyleTextFonts::default(), None),
  }
}

fn text_font_typeface(typeface: &Option<String>) -> Option<String> {
  typeface.as_ref().filter(|value| !value.is_empty()).cloned()
}

fn table_cell_style_properties(
  source: &a::TableCellStyle,
) -> (
  Option<FillProperties>,
  Option<ShapeStyleReference>,
  TableStyleBorders,
) {
  let (fill_properties, fill_reference) = match source.table_cell_style_choice.as_ref() {
    Some(a::TableCellStyleChoice::FillProperties(fill)) => {
      (FillProperties::from_dml_fill_properties(fill), None)
    }
    Some(a::TableCellStyleChoice::FillReference(reference)) => {
      (None, Some(fill_style_reference(reference)))
    }
    None => (None, None),
  };
  let borders = source
    .table_cell_borders
    .as_deref()
    .map(table_style_borders)
    .unwrap_or_default();
  (fill_properties, fill_reference, borders)
}

fn table_style_borders(source: &a::TableCellBorders) -> TableStyleBorders {
  TableStyleBorders {
    left: source.left_border.as_deref().and_then(|border| {
      border
        .left_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::LeftBorderChoice::Outline(outline) => LineProperties::from_dml_outline(outline),
          a::LeftBorderChoice::LineReference(_) => None,
        })
    }),
    left_reference: source.left_border.as_deref().and_then(|border| {
      border
        .left_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::LeftBorderChoice::LineReference(reference) => Some(line_style_reference(reference)),
          a::LeftBorderChoice::Outline(_) => None,
        })
    }),
    right: source.right_border.as_deref().and_then(|border| {
      border
        .right_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::RightBorderChoice::Outline(outline) => LineProperties::from_dml_outline(outline),
          a::RightBorderChoice::LineReference(_) => None,
        })
    }),
    right_reference: source.right_border.as_deref().and_then(|border| {
      border
        .right_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::RightBorderChoice::LineReference(reference) => Some(line_style_reference(reference)),
          a::RightBorderChoice::Outline(_) => None,
        })
    }),
    top: source.top_border.as_deref().and_then(|border| {
      border
        .top_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::TopBorderChoice::Outline(outline) => LineProperties::from_dml_outline(outline),
          a::TopBorderChoice::LineReference(_) => None,
        })
    }),
    top_reference: source.top_border.as_deref().and_then(|border| {
      border
        .top_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::TopBorderChoice::LineReference(reference) => Some(line_style_reference(reference)),
          a::TopBorderChoice::Outline(_) => None,
        })
    }),
    bottom: source.bottom_border.as_deref().and_then(|border| {
      border
        .bottom_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::BottomBorderChoice::Outline(outline) => LineProperties::from_dml_outline(outline),
          a::BottomBorderChoice::LineReference(_) => None,
        })
    }),
    bottom_reference: source.bottom_border.as_deref().and_then(|border| {
      border
        .bottom_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::BottomBorderChoice::LineReference(reference) => Some(line_style_reference(reference)),
          a::BottomBorderChoice::Outline(_) => None,
        })
    }),
    inside_horizontal: source
      .inside_horizontal_border
      .as_deref()
      .and_then(|border| {
        border
          .inside_horizontal_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::InsideHorizontalBorderChoice::Outline(outline) => {
              LineProperties::from_dml_outline(outline)
            }
            a::InsideHorizontalBorderChoice::LineReference(_) => None,
          })
      }),
    inside_horizontal_reference: source
      .inside_horizontal_border
      .as_deref()
      .and_then(|border| {
        border
          .inside_horizontal_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::InsideHorizontalBorderChoice::LineReference(reference) => {
              Some(line_style_reference(reference))
            }
            a::InsideHorizontalBorderChoice::Outline(_) => None,
          })
      }),
    inside_vertical: source.inside_vertical_border.as_deref().and_then(|border| {
      border
        .inside_vertical_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::InsideVerticalBorderChoice::Outline(outline) => {
            LineProperties::from_dml_outline(outline)
          }
          a::InsideVerticalBorderChoice::LineReference(_) => None,
        })
    }),
    inside_vertical_reference: source.inside_vertical_border.as_deref().and_then(|border| {
      border
        .inside_vertical_border_choice
        .as_ref()
        .and_then(|choice| match choice {
          a::InsideVerticalBorderChoice::LineReference(reference) => {
            Some(line_style_reference(reference))
          }
          a::InsideVerticalBorderChoice::Outline(_) => None,
        })
    }),
    top_left_to_bottom_right: source.top_left_to_bottom_right_border.as_deref().and_then(
      |border| {
        border
          .top_left_to_bottom_right_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::TopLeftToBottomRightBorderChoice::Outline(outline) => {
              LineProperties::from_dml_outline(outline)
            }
            a::TopLeftToBottomRightBorderChoice::LineReference(_) => None,
          })
      },
    ),
    top_left_to_bottom_right_reference: source.top_left_to_bottom_right_border.as_deref().and_then(
      |border| {
        border
          .top_left_to_bottom_right_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::TopLeftToBottomRightBorderChoice::LineReference(reference) => {
              Some(line_style_reference(reference))
            }
            a::TopLeftToBottomRightBorderChoice::Outline(_) => None,
          })
      },
    ),
    bottom_left_to_top_right: source.top_right_to_bottom_left_border.as_deref().and_then(
      |border| {
        border
          .top_right_to_bottom_left_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::TopRightToBottomLeftBorderChoice::Outline(outline) => {
              LineProperties::from_dml_outline(outline)
            }
            a::TopRightToBottomLeftBorderChoice::LineReference(_) => None,
          })
      },
    ),
    bottom_left_to_top_right_reference: source.top_right_to_bottom_left_border.as_deref().and_then(
      |border| {
        border
          .top_right_to_bottom_left_border_choice
          .as_ref()
          .and_then(|choice| match choice {
            a::TopRightToBottomLeftBorderChoice::LineReference(reference) => {
              Some(line_style_reference(reference))
            }
            a::TopRightToBottomLeftBorderChoice::Outline(_) => None,
          })
      },
    ),
  }
}

fn fill_style_reference(reference: &a::FillReference) -> ShapeStyleReference {
  ShapeStyleReference {
    index: reference.index,
    placeholder_color: reference
      .fill_reference_choice
      .as_ref()
      .and_then(Color::from_fill_reference_choice),
  }
}

fn line_style_reference(reference: &a::LineReference) -> ShapeStyleReference {
  ShapeStyleReference {
    index: reference.index,
    placeholder_color: reference
      .line_reference_choice
      .as_ref()
      .and_then(Color::from_line_reference_choice),
  }
}

fn table_cell_text_color(choice: &a::TableCellTextStyleChoice2) -> Option<Color> {
  Color::from_table_cell_text_style_choice(choice)
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
