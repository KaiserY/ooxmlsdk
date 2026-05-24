use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use std::sync::Arc;

use crate::docx::{BorderStyle, RgbColor, TextStyle};
use crate::error::Result;

#[derive(Clone, Debug, Default)]
pub(crate) struct StylesCatalog {
  pub(crate) custom_number_formats: Vec<NumberFormatRecord>,
  pub(crate) style_xfs: Vec<CellFormatRecord>,
  pub(crate) cell_xfs: Vec<CellFormatRecord>,
  pub(crate) font_records: Vec<FontRecord>,
  pub(crate) fill_records: Vec<FillRecord>,
  pub(crate) border_records: Vec<BorderRecord>,
  pub(crate) fonts: usize,
  pub(crate) fills: usize,
  pub(crate) borders: usize,
  pub(crate) cell_style_formats: usize,
  pub(crate) cell_formats: usize,
  pub(crate) cell_styles: usize,
  pub(crate) differential_formats: usize,
  pub(crate) table_styles: usize,
  pub(crate) default_table_style: Option<String>,
  pub(crate) default_pivot_style: Option<String>,
  pub(crate) has_colors: bool,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NumberFormatRecord {
  pub(crate) id: u32,
  pub(crate) code: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CellFormatRecord {
  pub(crate) number_format_id: Option<u32>,
  pub(crate) font_id: Option<u32>,
  pub(crate) fill_id: Option<u32>,
  pub(crate) border_id: Option<u32>,
  pub(crate) style_xf_id: Option<u32>,
  pub(crate) quote_prefix: bool,
  pub(crate) pivot_button: bool,
  pub(crate) apply_number_format: bool,
  pub(crate) apply_font: bool,
  pub(crate) apply_fill: bool,
  pub(crate) apply_border: bool,
  pub(crate) apply_alignment: bool,
  pub(crate) apply_protection: bool,
  pub(crate) has_alignment: bool,
  pub(crate) has_protection: bool,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FontRecord {
  pub(crate) name: Option<Arc<str>>,
  pub(crate) size_pt: Option<OrderedF64>,
  pub(crate) color: Option<RgbColor>,
  pub(crate) bold: bool,
  pub(crate) italic: bool,
  pub(crate) underline: bool,
  pub(crate) strikethrough: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FillRecord {
  pub(crate) color: Option<RgbColor>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BorderRecord {
  pub(crate) left: Option<BorderStyle>,
  pub(crate) right: Option<BorderStyle>,
  pub(crate) top: Option<BorderStyle>,
  pub(crate) bottom: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct OrderedF64(u64);

impl OrderedF64 {
  fn new(value: f64) -> Self {
    Self(value.to_bits())
  }

  pub(crate) fn get(self) -> f64 {
    f64::from_bits(self.0)
  }
}

impl StylesCatalog {
  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
  ) -> Result<Self> {
    let Some(styles_part) = workbook_part.workbook_styles_part(package) else {
      return Ok(Self::default());
    };

    let stylesheet = styles_part.root_element(package)?;
    Ok(Self::from_stylesheet(stylesheet))
  }

  fn from_stylesheet(stylesheet: &x::Stylesheet) -> Self {
    let table_styles = stylesheet.table_styles.as_ref();
    Self {
      custom_number_formats: stylesheet
        .numbering_formats
        .as_ref()
        .map(|formats| {
          formats
            .numbering_format
            .iter()
            .map(|format| NumberFormatRecord {
              id: format.number_format_id,
              code: format.format_code.clone(),
            })
            .collect()
        })
        .unwrap_or_default(),
      style_xfs: stylesheet
        .cell_style_formats
        .as_ref()
        .map(|formats| {
          formats
            .cell_format
            .iter()
            .map(|format| CellFormatRecord::from_cell_format(format, true))
            .collect()
        })
        .unwrap_or_default(),
      cell_xfs: stylesheet
        .cell_formats
        .as_ref()
        .map(|formats| {
          formats
            .cell_format
            .iter()
            .map(|format| CellFormatRecord::from_cell_format(format, false))
            .collect()
        })
        .unwrap_or_default(),
      font_records: stylesheet
        .fonts
        .as_ref()
        .map(|fonts| fonts.font.iter().map(FontRecord::from_font).collect())
        .unwrap_or_default(),
      fill_records: stylesheet
        .fills
        .as_ref()
        .map(|fills| fills.fill.iter().map(FillRecord::from_fill).collect())
        .unwrap_or_default(),
      border_records: stylesheet
        .borders
        .as_ref()
        .map(|borders| {
          borders
            .border
            .iter()
            .map(BorderRecord::from_border)
            .collect()
        })
        .unwrap_or_default(),
      fonts: stylesheet
        .fonts
        .as_ref()
        .map_or(0, |fonts| fonts.font.len()),
      fills: stylesheet
        .fills
        .as_ref()
        .map_or(0, |fills| fills.fill.len()),
      borders: stylesheet
        .borders
        .as_ref()
        .map_or(0, |borders| borders.border.len()),
      cell_style_formats: stylesheet
        .cell_style_formats
        .as_ref()
        .map_or(0, |formats| formats.cell_format.len()),
      cell_formats: stylesheet
        .cell_formats
        .as_ref()
        .map_or(0, |formats| formats.cell_format.len()),
      cell_styles: stylesheet
        .cell_styles
        .as_ref()
        .map_or(0, |styles| styles.cell_style.len()),
      differential_formats: stylesheet
        .differential_formats
        .as_ref()
        .map_or(0, |formats| formats.differential_format.len()),
      table_styles: table_styles.map_or(0, |styles| styles.table_style.len()),
      default_table_style: table_styles.and_then(|styles| styles.default_table_style.clone()),
      default_pivot_style: table_styles.and_then(|styles| styles.default_pivot_style.clone()),
      has_colors: stylesheet.colors.is_some(),
      has_extensions: stylesheet.stylesheet_extension_list.is_some(),
    }
  }

  pub(crate) fn number_format_code(&self, id: u32) -> Option<&str> {
    match id {
      // Source: LibreOffice sc/source/filter/oox/numberformatsbuffer.cxx
      // maps OOXML numFmtId 0 to NUMBER_STANDARD (General).
      0 => Some("General"),
      1 => Some("0"),
      2 => Some("0.00"),
      3 => Some("#,##0"),
      4 => Some("#,##0.00"),
      9 => Some("0%"),
      10 => Some("0.00%"),
      11 => Some("0.00E+00"),
      12 => Some("# ?/?"),
      13 => Some("# ??/??"),
      14 => Some("mm-dd-yy"),
      15 => Some("d-mmm-yy"),
      16 => Some("d-mmm"),
      17 => Some("mmm-yy"),
      18 => Some("h:mm AM/PM"),
      19 => Some("h:mm:ss AM/PM"),
      20 => Some("h:mm"),
      21 => Some("h:mm:ss"),
      22 => Some("m/d/yy h:mm"),
      37 => Some("#,##0 ;(#,##0)"),
      38 => Some("#,##0 ;[Red](#,##0)"),
      39 => Some("#,##0.00;(#,##0.00)"),
      40 => Some("#,##0.00;[Red](#,##0.00)"),
      45 => Some("mm:ss"),
      46 => Some("[h]:mm:ss"),
      47 => Some("mmss.0"),
      48 => Some("##0.0E+0"),
      49 => Some("@"),
      _ => self
        .custom_number_formats
        .iter()
        .find(|format| format.id == id)
        .map(|format| format.code.as_str()),
    }
  }

  pub(crate) fn text_style_for_cell(&self, style_index: Option<u32>) -> TextStyle {
    let mut style = TextStyle::default();
    let Some(format) = self.effective_cell_format(style_index) else {
      return style;
    };
    if !format.apply_font {
      return style;
    }
    let Some(font) = format
      .font_id
      .and_then(|id| self.font_records.get(id as usize))
    else {
      return style;
    };
    if let Some(name) = &font.name {
      style.font_family = Some(Arc::clone(name));
    }
    if let Some(size_pt) = font.size_pt {
      style.font_size_pt = size_pt.get() as f32;
    }
    if let Some(color) = font.color {
      style.color = color;
    }
    style.bold = font.bold;
    style.italic = font.italic;
    style.underline = font.underline;
    style.strikethrough = font.strikethrough;
    style
  }

  pub(crate) fn fill_color_for_cell(&self, style_index: Option<u32>) -> Option<RgbColor> {
    let format = self.effective_cell_format(style_index)?;
    if !format.apply_fill {
      return None;
    }
    format
      .fill_id
      .and_then(|id| self.fill_records.get(id as usize))
      .and_then(|fill| fill.color)
  }

  pub(crate) fn borders_for_cell(&self, style_index: Option<u32>) -> BorderRecord {
    let Some(format) = self.effective_cell_format(style_index) else {
      return BorderRecord::default();
    };
    if !format.apply_border {
      return BorderRecord::default();
    }
    format
      .border_id
      .and_then(|id| self.border_records.get(id as usize).copied())
      .unwrap_or_default()
  }

  fn effective_cell_format(&self, style_index: Option<u32>) -> Option<CellFormatRecord> {
    let mut format = self.cell_xfs.get(style_index? as usize)?.clone();
    let Some(style_xf) = format
      .style_xf_id
      .and_then(|id| self.style_xfs.get(id as usize))
    else {
      return Some(format);
    };
    // Source: LibreOffice sc/source/filter/oox/stylesbuffer.cxx
    // Xf::createPattern enables a cell XF property when it differs from the
    // parent style XF, even if the cell XF apply flag was initially false.
    if !format.apply_font {
      format.apply_font = !style_xf.apply_font || format.font_id != style_xf.font_id;
    }
    if !format.apply_number_format {
      format.apply_number_format =
        !style_xf.apply_number_format || format.number_format_id != style_xf.number_format_id;
    }
    if !format.apply_border {
      format.apply_border = !style_xf.apply_border || format.border_id != style_xf.border_id;
    }
    if !format.apply_fill {
      format.apply_fill = !style_xf.apply_fill || format.fill_id != style_xf.fill_id;
    }
    Some(format)
  }
}

impl CellFormatRecord {
  fn from_cell_format(format: &x::CellFormat, style_xf: bool) -> Self {
    // Source: LibreOffice sc/source/filter/oox/stylesbuffer.cxx
    // Xf::importXf. Office effectively lets explicit xf properties apply by
    // default; cellXf records with xfId are the branch where apply defaults to
    // false unless an id makes the property used.
    let apply_default = style_xf || format.format_id.is_none();
    let number_format_id = format.number_format_id;
    let font_id = format.font_id;
    let fill_id = format.fill_id;
    let border_id = format.border_id;
    Self {
      number_format_id,
      font_id,
      fill_id,
      border_id,
      style_xf_id: format.format_id,
      quote_prefix: format.quote_prefix.is_some_and(|value| value.as_bool()),
      pivot_button: format.pivot_button.is_some_and(|value| value.as_bool()),
      apply_number_format: format.apply_number_format.map_or(
        apply_default || number_format_id.is_some_and(|id| id > 0),
        |value| value.as_bool(),
      ),
      apply_font: format
        .apply_font
        .map_or(apply_default || font_id.is_some_and(|id| id > 0), |value| {
          value.as_bool()
        }),
      apply_fill: format
        .apply_fill
        .map_or(apply_default || fill_id.is_some_and(|id| id > 0), |value| {
          value.as_bool()
        }),
      apply_border: format.apply_border.map_or(
        apply_default || border_id.is_some_and(|id| id > 0),
        |value| value.as_bool(),
      ),
      apply_alignment: format
        .apply_alignment
        .map_or(apply_default, |value| value.as_bool()),
      apply_protection: format
        .apply_protection
        .map_or(apply_default, |value| value.as_bool()),
      has_alignment: format.alignment.is_some(),
      has_protection: format.protection.is_some(),
      has_extensions: format.extension_list.is_some(),
    }
  }

  pub(crate) fn used_flag_count(&self) -> usize {
    usize::from(self.apply_number_format)
      + usize::from(self.apply_font)
      + usize::from(self.apply_fill)
      + usize::from(self.apply_border)
      + usize::from(self.apply_alignment)
      + usize::from(self.apply_protection)
      + usize::from(self.quote_prefix)
      + usize::from(self.pivot_button)
      + usize::from(self.has_alignment)
      + usize::from(self.has_protection)
      + usize::from(self.has_extensions)
  }
}

impl FontRecord {
  fn from_font(font: &x::Font) -> Self {
    let mut record = Self::default();
    for choice in &font.font_choice {
      match choice {
        x::FontChoice::Bold(value) => {
          record.bold = value.val.map_or(true, |value| value.as_bool());
        }
        x::FontChoice::Italic(value) => {
          record.italic = value.val.map_or(true, |value| value.as_bool());
        }
        x::FontChoice::Strike(value) => {
          record.strikethrough = value.val.map_or(true, |value| value.as_bool());
        }
        x::FontChoice::Underline(value) => {
          record.underline = !matches!(value.val, Some(x::UnderlineValues::None));
        }
        x::FontChoice::FontSize(value) => {
          record.size_pt = Some(OrderedF64::new(value.val));
        }
        x::FontChoice::Color(value) => {
          record.color = color_from_ooxml(value.rgb.as_deref());
        }
        x::FontChoice::FontName(value) => {
          record.name = Some(Arc::from(value.val.as_str()));
        }
        _ => {}
      }
    }
    record
  }
}

impl FillRecord {
  fn from_fill(fill: &x::Fill) -> Self {
    let color = match &fill.fill_choice {
      Some(x::FillChoice::PatternFill(pattern)) => color_from_pattern_fill(pattern),
      Some(x::FillChoice::GradientFill(gradient)) => color_from_gradient_fill(gradient),
      None => None,
    };
    Self { color }
  }
}

impl BorderRecord {
  fn from_border(border: &x::Border) -> Self {
    Self {
      left: border
        .left_border
        .as_deref()
        .and_then(|border| border_style(border.style, border.color.as_ref())),
      right: border
        .right_border
        .as_deref()
        .and_then(|border| border_style(border.style, border.color.as_ref())),
      top: border
        .top_border
        .as_deref()
        .and_then(|border| border_style(border.style, border.color.as_ref())),
      bottom: border
        .bottom_border
        .as_deref()
        .and_then(|border| border_style(border.style, border.color.as_ref())),
    }
  }
}

fn color_from_pattern_fill(pattern: &x::PatternFill) -> Option<RgbColor> {
  let pattern_type = pattern.pattern_type.unwrap_or_default();
  if matches!(pattern_type, x::PatternValues::None) {
    return None;
  }
  let pattern_color = pattern
    .foreground_color
    .as_ref()
    .and_then(|color| color_from_ooxml(color.rgb.as_deref()));
  let fill_color = pattern
    .background_color
    .as_ref()
    .and_then(|color| color_from_ooxml(color.rgb.as_deref()));
  match pattern_type {
    x::PatternValues::Solid => pattern_color.or(fill_color),
    _ => Some(mix_colors(
      pattern_color.unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
      fill_color.unwrap_or(RgbColor {
        r: 255,
        g: 255,
        b: 255,
      }),
      pattern_alpha(pattern_type),
    )),
  }
}

fn color_from_gradient_fill(gradient: &x::GradientFill) -> Option<RgbColor> {
  let mut colors = gradient
    .gradient_stop
    .iter()
    .map(|stop| &stop.color)
    .filter_map(|color| color_from_ooxml(color.rgb.as_deref()));
  let first = colors.next()?;
  let Some(second) = colors.next() else {
    return Some(first);
  };
  Some(mix_colors(first, second, 0x40))
}

fn pattern_alpha(pattern_type: x::PatternValues) -> i32 {
  match pattern_type {
    x::PatternValues::DarkDown
    | x::PatternValues::DarkGrid
    | x::PatternValues::DarkHorizontal
    | x::PatternValues::DarkUp
    | x::PatternValues::DarkVertical
    | x::PatternValues::MediumGray => 0x40,
    x::PatternValues::DarkGray | x::PatternValues::DarkTrellis => 0x60,
    x::PatternValues::Gray0625 => 0x08,
    x::PatternValues::Gray125 => 0x10,
    x::PatternValues::LightDown
    | x::PatternValues::LightGray
    | x::PatternValues::LightHorizontal
    | x::PatternValues::LightUp
    | x::PatternValues::LightVertical => 0x20,
    x::PatternValues::LightGrid => 0x38,
    x::PatternValues::LightTrellis => 0x30,
    x::PatternValues::Solid | x::PatternValues::None => 0x80,
  }
}

fn mix_colors(pattern: RgbColor, fill: RgbColor, alpha: i32) -> RgbColor {
  RgbColor {
    r: mix_color_component(pattern.r, fill.r, alpha),
    g: mix_color_component(pattern.g, fill.g, alpha),
    b: mix_color_component(pattern.b, fill.b, alpha),
  }
}

fn mix_color_component(pattern: u8, fill: u8, alpha: i32) -> u8 {
  (((i32::from(pattern) - i32::from(fill)) * alpha) / 0x80 + i32::from(fill)).clamp(0, 255) as u8
}

fn border_style(
  style: Option<x::BorderStyleValues>,
  color: Option<&x::Color>,
) -> Option<BorderStyle> {
  let style = style?;
  if matches!(style, x::BorderStyleValues::None) {
    return None;
  }
  Some(BorderStyle {
    width_pt: border_width_pt(style),
    color: color
      .and_then(|color| color_from_ooxml(color.rgb.as_deref()))
      .unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
    compound: matches!(style, x::BorderStyleValues::Double),
    ..BorderStyle::default()
  })
}

fn border_width_pt(style: x::BorderStyleValues) -> f32 {
  // Source: LibreOffice maps OOXML border tokens through Border::convertBorderLine
  // into editeng SvxBorderLine widths. Keep the same thin/medium/thick groups.
  match style {
    x::BorderStyleValues::Hair => 0.25,
    x::BorderStyleValues::Thin
    | x::BorderStyleValues::Dashed
    | x::BorderStyleValues::Dotted
    | x::BorderStyleValues::DashDot
    | x::BorderStyleValues::DashDotDot
    | x::BorderStyleValues::SlantDashDot => 0.5,
    x::BorderStyleValues::Medium
    | x::BorderStyleValues::MediumDashed
    | x::BorderStyleValues::MediumDashDot
    | x::BorderStyleValues::MediumDashDotDot
    | x::BorderStyleValues::Double => 1.0,
    x::BorderStyleValues::Thick => 1.5,
    x::BorderStyleValues::None => 0.0,
  }
}

fn color_from_ooxml(rgb: Option<&str>) -> Option<RgbColor> {
  let rgb = rgb?;
  let color = rgb.strip_prefix('#').unwrap_or(rgb);
  let color = match color.len() {
    8 => &color[2..],
    6 => color,
    _ => return None,
  };
  let value = u32::from_str_radix(color, 16).ok()?;
  Some(RgbColor {
    r: ((value >> 16) & 0xff) as u8,
    g: ((value >> 8) & 0xff) as u8,
    b: (value & 0xff) as u8,
  })
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DefinedNamesCatalog {
  pub(crate) records: Vec<DefinedNameRecord>,
  pub(crate) print_areas: usize,
  pub(crate) print_titles: usize,
  pub(crate) filter_databases: usize,
  pub(crate) local_names: usize,
  pub(crate) hidden_names: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DefinedNameRecord {
  pub(crate) name: String,
  pub(crate) builtin: Option<DefinedNameBuiltin>,
  pub(crate) local_sheet_id: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) formula: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum DefinedNameBuiltin {
  PrintArea,
  PrintTitles,
  FilterDatabase,
}

impl DefinedNamesCatalog {
  pub(crate) fn from_workbook(workbook: &x::Workbook) -> Self {
    let Some(defined_names) = &workbook.defined_names else {
      return Self::default();
    };

    let records = defined_names
      .defined_name
      .iter()
      .map(|name| DefinedNameRecord {
        name: name.name.clone(),
        builtin: defined_name_builtin(name.name.as_str()),
        local_sheet_id: name.local_sheet_id,
        hidden: name.hidden.map(|hidden| hidden.as_bool()).unwrap_or(false),
        formula: name.xml_content.clone().unwrap_or_default(),
      })
      .collect::<Vec<_>>();

    let mut catalog = Self {
      records,
      ..Self::default()
    };
    for record in &catalog.records {
      match record.builtin {
        Some(DefinedNameBuiltin::PrintArea) => catalog.print_areas += 1,
        Some(DefinedNameBuiltin::PrintTitles) => catalog.print_titles += 1,
        Some(DefinedNameBuiltin::FilterDatabase) => catalog.filter_databases += 1,
        _ => {}
      }
      if record.local_sheet_id.is_some() {
        catalog.local_names += 1;
      }
      if record.hidden {
        catalog.hidden_names += 1;
      }
    }
    catalog
  }

  pub(crate) fn records_for_sheet(
    &self,
    sheet_index: usize,
    builtin: DefinedNameBuiltin,
  ) -> Vec<&DefinedNameRecord> {
    self
      .records
      .iter()
      .filter(|record| {
        record.builtin == Some(builtin) && record.local_sheet_id == Some(sheet_index as u32)
      })
      .collect()
  }
}

fn defined_name_builtin(name: &str) -> Option<DefinedNameBuiltin> {
  let base = name
    .strip_prefix("_xlnm.")
    .or_else(|| name.strip_prefix("_XLNM."))
    .unwrap_or(name);
  if base.eq_ignore_ascii_case("Print_Area") {
    Some(DefinedNameBuiltin::PrintArea)
  } else if base.eq_ignore_ascii_case("Print_Titles") {
    Some(DefinedNameBuiltin::PrintTitles)
  } else if base.eq_ignore_ascii_case("_FilterDatabase") {
    Some(DefinedNameBuiltin::FilterDatabase)
  } else {
    None
  }
}
