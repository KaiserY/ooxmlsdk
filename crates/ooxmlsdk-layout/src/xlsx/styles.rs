use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use std::sync::Arc;

use crate::error::Result;
use crate::localization::{OfficeLocaleContext, OfficeResourceLocale};
use crate::model::{BorderDashPattern, BorderStyle, RgbColor, TextStyle};
use crate::pptx::drawingml::color::{ResolvedColor, apply_excel_tint};
use crate::pptx::drawingml::fill::FillProperties;
use crate::pptx::drawingml::line::LineProperties;
use crate::pptx::drawingml::shape_properties::EffectProperties;
use crate::pptx::drawingml::theme::{ThemeColorScheme, ThemeFontScheme, ThemeFormatScheme};
use crate::units;

#[derive(Clone, Debug, Default)]
pub(crate) struct StylesCatalog {
  pub(crate) custom_number_formats: Vec<NumberFormatRecord>,
  pub(crate) style_xfs: Vec<CellFormatRecord>,
  pub(crate) cell_xfs: Vec<CellFormatRecord>,
  pub(crate) font_records: Vec<FontRecord>,
  pub(crate) fill_records: Vec<FillRecord>,
  pub(crate) border_records: Vec<BorderRecord>,
  pub(crate) differential_format_records: Vec<DifferentialFormatRecord>,
  pub(crate) table_style_records: Vec<TableStyleRecord>,
  theme_fonts: Option<ThemeFontScheme>,
  theme_format: Option<ThemeFormatScheme>,
  theme_colors: ThemeColorPalette,
  indexed_colors: Vec<RgbColor>,
  theme_major_east_asian: Option<Arc<str>>,
  theme_minor_east_asian: Option<Arc<str>>,
  missing_theme_minor_from_document_language: bool,
  builtin_number_format_locale: BuiltinNumberFormatLocale,
  locales: OfficeLocaleContext,
}

#[derive(Clone, Copy, Debug, Default)]
struct ThemeColorPalette {
  colors: [Option<RgbColor>; 12],
}

impl ThemeColorPalette {
  fn from_dml(scheme: &a::ColorScheme) -> Self {
    let scheme = ThemeColorScheme::from_dml(scheme);
    let tokens = [
      a::ColorSchemeIndexValues::Light1,
      a::ColorSchemeIndexValues::Dark1,
      a::ColorSchemeIndexValues::Light2,
      a::ColorSchemeIndexValues::Dark2,
      a::ColorSchemeIndexValues::Accent1,
      a::ColorSchemeIndexValues::Accent2,
      a::ColorSchemeIndexValues::Accent3,
      a::ColorSchemeIndexValues::Accent4,
      a::ColorSchemeIndexValues::Accent5,
      a::ColorSchemeIndexValues::Accent6,
      a::ColorSchemeIndexValues::Hyperlink,
      a::ColorSchemeIndexValues::FollowedHyperlink,
    ];
    let mut colors = [None; 12];
    for (index, token) in tokens.into_iter().enumerate() {
      colors[index] = scheme
        .get_color(token)
        .and_then(|color| color.resolve_rgb(&mut |_| None, None))
        .map(rgb_from_resolved);
    }
    Self { colors }
  }

  fn get(&self, index: u32) -> Option<RgbColor> {
    self.colors.get(index as usize).copied().flatten()
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum BuiltinNumberFormatLocale {
  #[default]
  EnglishUs,
  ChineseSimplified,
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
  pub(crate) alignment: Option<AlignmentRecord>,
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
  pub(crate) scheme: x::FontSchemeValues,
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

#[derive(Clone, Debug, Default)]
pub(crate) struct DifferentialFormatRecord {
  pub(crate) font: Option<FontRecord>,
  pub(crate) fill: Option<FillRecord>,
  pub(crate) border: Option<BorderRecord>,
  pub(crate) alignment: Option<AlignmentRecord>,
  pub(crate) number_format: Option<NumberFormatRecord>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TableStyleRecord {
  pub(crate) name: String,
  pub(crate) elements: Vec<TableStyleElementRecord>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TableStyleElementRecord {
  pub(crate) r#type: x::TableStyleValues,
  pub(crate) size: u32,
  pub(crate) format_id: Option<u32>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct AlignmentRecord {
  pub(crate) horizontal: Option<x::HorizontalAlignmentValues>,
  pub(crate) vertical: Option<x::VerticalAlignmentValues>,
  pub(crate) text_rotation: Option<u32>,
  pub(crate) wrap_text: bool,
  pub(crate) indent: Option<u32>,
  pub(crate) relative_indent: Option<i32>,
  pub(crate) justify_last_line: bool,
  pub(crate) shrink_to_fit: bool,
  pub(crate) reading_order: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct IconSetPrintMetrics {
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
  pub(crate) leading_inset_pt: f32,
  pub(crate) bottom_inset_pt: f32,
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
  pub(crate) fn has_explicit_ui_language(&self) -> bool {
    self.locales.ui_language().is_some()
  }

  pub(crate) fn output_ui_language(&self) -> &str {
    self.locales.ui_language_or_default()
  }

  pub(crate) fn output_format_locale(&self) -> Option<&str> {
    self.locales.format_locale()
  }

  pub(crate) fn theme_color(&self, index: u32, tint: f64) -> Option<RgbColor> {
    let base = self.theme_colors.get(index)?;
    if tint == 0.0 {
      return Some(base);
    }
    Some(rgb_from_resolved(apply_excel_tint(
      ResolvedColor::new(base.r, base.g, base.b),
      tint,
    )))
  }

  pub(crate) fn spreadsheet_color(&self, color: &x::Color) -> Option<RgbColor> {
    color_from_color(color, &self.indexed_colors, &self.theme_colors)
  }

  pub(crate) fn theme_fill_style(&self, index: u32) -> Option<&FillProperties> {
    self.theme_format.as_ref()?.get_fill_style(index)
  }

  pub(crate) fn theme_line_style(&self, index: u32) -> Option<&LineProperties> {
    self.theme_format.as_ref()?.get_line_style(index)
  }

  pub(crate) fn theme_effect_style(&self, index: u32) -> Option<&EffectProperties> {
    self.theme_format.as_ref()?.get_effect_style(index)
  }

  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
    locales: &OfficeLocaleContext,
  ) -> Result<Self> {
    let (theme_fonts, theme_format, theme_colors) =
      if let Some(theme_part) = workbook_part.theme_part(package) {
        let theme = theme_part.root_element(package)?;
        (
          Some(ThemeFontScheme::from_dml(&theme.theme_elements.font_scheme)),
          Some(ThemeFormatScheme::from_dml(
            &theme.theme_elements.format_scheme,
          )),
          ThemeColorPalette::from_dml(&theme.theme_elements.color_scheme),
        )
      } else {
        (None, None, ThemeColorPalette::default())
      };
    let missing_theme_minor = missing_theme_minor_font(
      theme_fonts.as_ref(),
      locales.default_document_resource_locale(),
    );
    let Some(styles_part) = workbook_part.workbook_styles_part(package) else {
      return Ok(Self {
        theme_fonts,
        theme_format,
        theme_colors,
        missing_theme_minor_from_document_language: missing_theme_minor.is_some(),
        theme_minor_east_asian: missing_theme_minor,
        builtin_number_format_locale: builtin_number_format_locale(locales),
        locales: locales.clone(),
        ..Self::default()
      });
    };

    let stylesheet = styles_part.root_element(package)?;
    let mut catalog =
      Self::from_stylesheet(stylesheet, theme_fonts.as_ref(), theme_colors, locales);
    catalog.theme_format = theme_format;
    catalog.missing_theme_minor_from_document_language = missing_theme_minor.is_some();
    catalog.theme_minor_east_asian = catalog.theme_minor_east_asian.or(missing_theme_minor);
    Ok(catalog)
  }

  fn from_stylesheet(
    stylesheet: &x::Stylesheet,
    theme_fonts: Option<&ThemeFontScheme>,
    theme_colors: ThemeColorPalette,
    locales: &OfficeLocaleContext,
  ) -> Self {
    let document_language = locales.default_document_language();
    let indexed_colors = stylesheet
      .colors
      .as_ref()
      .and_then(|colors| colors.indexed_colors.as_ref())
      .map(|colors| {
        colors
          .rgb_color
          .iter()
          .filter_map(|color| color_from_ooxml(color.rgb.as_deref()))
          .collect::<Vec<_>>()
      })
      .unwrap_or_default();
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
            .xml_children
            .iter()
            .filter_map(|child| match child {
              x::CellFormatsChoice::CellFormat(format) => Some(format.as_ref()),
              x::CellFormatsChoice::AlternateContent(_) => None,
            })
            .map(|format| CellFormatRecord::from_cell_format(format, false))
            .collect()
        })
        .unwrap_or_default(),
      font_records: stylesheet
        .fonts
        .as_ref()
        .map(|fonts| {
          fonts
            .xml_children
            .iter()
            .filter_map(|child| match child {
              x::FontsChoice::Font(font) => Some(font),
              x::FontsChoice::AlternateContent(_) => None,
            })
            .map(|font| FontRecord::from_font_with_colors(font, &indexed_colors, &theme_colors))
            .collect()
        })
        .unwrap_or_default(),
      fill_records: stylesheet
        .fills
        .as_ref()
        .map(|fills| {
          fills
            .xml_children
            .iter()
            .filter_map(|child| match child {
              x::FillsChoice::Fill(fill) => Some(fill.as_ref()),
              x::FillsChoice::AlternateContent(_) => None,
            })
            .map(|fill| FillRecord::from_fill_with_colors(fill, &indexed_colors, &theme_colors))
            .collect()
        })
        .unwrap_or_default(),
      border_records: stylesheet
        .borders
        .as_ref()
        .map(|borders| {
          borders
            .border
            .iter()
            .map(|border| {
              BorderRecord::from_border_with_colors(border, &indexed_colors, &theme_colors)
            })
            .collect()
        })
        .unwrap_or_default(),
      differential_format_records: stylesheet
        .differential_formats
        .as_ref()
        .map(|formats| {
          formats
            .xml_children
            .iter()
            .filter_map(|child| match child {
              x::DifferentialFormatsChoice::DifferentialFormat(format) => Some(format.as_ref()),
              x::DifferentialFormatsChoice::AlternateContent(_) => None,
            })
            .map(|format| {
              DifferentialFormatRecord::from_differential_format_with_colors(
                format,
                &indexed_colors,
                &theme_colors,
              )
            })
            .collect()
        })
        .unwrap_or_default(),
      table_style_records: stylesheet
        .table_styles
        .as_ref()
        .map(|styles| {
          styles
            .table_style
            .iter()
            .map(|style| TableStyleRecord {
              name: style.name.clone(),
              elements: style
                .table_style_element
                .iter()
                .map(|element| TableStyleElementRecord {
                  r#type: element.r#type,
                  size: element.size.unwrap_or(1).max(1),
                  format_id: element.format_id,
                })
                .collect(),
            })
            .collect()
        })
        .unwrap_or_default(),
      theme_fonts: theme_fonts.cloned(),
      theme_format: None,
      theme_major_east_asian: theme_fonts
        .and_then(|fonts| fonts.resolve_font_for_language("+mj-ea", document_language))
        .map(Arc::from),
      theme_minor_east_asian: theme_fonts
        .and_then(|fonts| fonts.resolve_font_for_language("+mn-ea", document_language))
        .map(Arc::from),
      missing_theme_minor_from_document_language: false,
      theme_colors,
      indexed_colors,
      builtin_number_format_locale: builtin_number_format_locale(locales),
      locales: locales.clone(),
    }
  }

  pub(crate) fn number_format_code(&self, id: u32) -> Option<&str> {
    if let Some(format) = self
      .custom_number_formats
      .iter()
      .find(|format| format.id == id)
    {
      return Some(format.code.as_str());
    }
    match id {
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
      // Excel built-in date formats are locale-dependent. [MS-OSHARED]
      // specifies yyyy/M/d for the Chinese short-date format, and Calc keeps
      // per-locale built-in format tables instead of a single English table.
      14 if self.builtin_number_format_locale == BuiltinNumberFormatLocale::ChineseSimplified => {
        Some("YYYY/M/D")
      }
      14 => Some("M/D/YYYY"),
      15 => Some("d-mmm-yy"),
      16 => Some("d-mmm"),
      17 => Some("mmm-yy"),
      18 => Some("h:mm AM/PM"),
      19 => Some("h:mm:ss AM/PM"),
      20 => Some("h:mm"),
      21 => Some("h:mm:ss"),
      22 if self.builtin_number_format_locale == BuiltinNumberFormatLocale::ChineseSimplified => {
        Some("YYYY/M/D h:mm")
      }
      22 => Some("M/D/YYYY h:mm"),
      37 => Some("#,##0 ;(#,##0)"),
      38 => Some("#,##0 ;[Red](#,##0)"),
      39 => Some("#,##0.00;(#,##0.00)"),
      40 => Some("#,##0.00;[Red](#,##0.00)"),
      // maps OOXML builtin ids 41..44 to accounting formats without a
      // currency symbol in the default OOXML table.
      41 => Some("_-* #,##0_-;-* #,##0_-;_-* \"-\"_-;_-@_-"),
      42 => Some("_-* #,##0_-;-* #,##0_-;_-* \"-\"_-;_-@_-"),
      43 => Some("_-* #,##0.00_-;-* #,##0.00_-;_-* \"-\"??_-;_-@_-"),
      44 => Some("_-* #,##0.00_-;-* #,##0.00_-;_-* \"-\"??_-;_-@_-"),
      45 => Some("mm:ss"),
      46 => Some("[h]:mm:ss"),
      47 => Some("mmss.0"),
      48 => Some("##0.0E+0"),
      49 => Some("@"),
      _ => None,
    }
  }

  pub(crate) fn number_format_code_for_cell(&self, style_index: Option<u32>) -> Option<&str> {
    self
      .effective_cell_format(style_index)?
      .number_format_id
      .and_then(|id| self.number_format_code(id))
  }

  pub(crate) fn resolve_drawingml_theme_font<'a>(&'a self, typeface: &'a str) -> &'a str {
    self
      .theme_fonts
      .as_ref()
      .and_then(|fonts| fonts.resolve_font(typeface))
      .unwrap_or(typeface)
  }

  pub(crate) fn text_style_for_cell(&self, style_index: Option<u32>) -> TextStyle {
    let mut style = self.default_font_text_style();
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
    self.apply_font_family(font, &mut style);
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

  pub(crate) fn direct_nondefault_font_color_for_cell(
    &self,
    style_index: Option<u32>,
  ) -> Option<RgbColor> {
    let format = self.cell_xfs.get(style_index? as usize)?;
    if !format.apply_font {
      return None;
    }
    let color = format
      .font_id
      .and_then(|id| self.font_records.get(id as usize))
      .and_then(|font| font.color)?;
    // LibreOffice ScPatternAttr::lcl_populateresult gives a non-default
    // direct cell font value precedence over table formatting. Black is the
    // font-color default for table styles, so it deliberately falls through
    // and lets the table style supply its region color.
    (color != RgbColor { r: 0, g: 0, b: 0 }).then_some(color)
  }

  pub(crate) fn default_font_text_style(&self) -> TextStyle {
    let mut style = TextStyle::default();
    let Some(font) = self.font_records.first() else {
      if self.missing_theme_minor_from_document_language
        && let Some(font_family) = &self.theme_minor_east_asian
      {
        style.font_family = Some(Arc::clone(font_family));
        style.east_asia_font_family = Some(Arc::clone(font_family));
      }
      return style;
    };
    self.apply_font_family(font, &mut style);
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

  pub(crate) fn default_chart_text_style(&self) -> TextStyle {
    TextStyle {
      font_family: Some(Arc::from(
        self
          .theme_fonts
          .as_ref()
          .and_then(|fonts| fonts.minor_latin.as_deref())
          // A chart without theme1.xml still uses the current Office
          // application theme. Microsoft 365 fixed output resolves that chart
          // default independently from the worksheet Normal style.
          .unwrap_or("Aptos Narrow"),
      )),
      east_asia_font_family: self.theme_minor_east_asian.clone(),
      // DrawingML's latin/eastAsian/complex-script faces are font slots just
      // like WordprocessingML rFonts. In particular, ASCII digits following
      // Han text remain on the latin face.
      wordprocessingml_font_slots: true,
      // Legacy c:chart without a related ChartStyle part inherits plain tx1.
      // A related Office 2013+ ChartStyle supplies its own transformed font
      // reference and is layered by the drawing chart lowerer.
      color: RgbColor::default(),
      ..TextStyle::default()
    }
  }

  pub(crate) fn default_drawing_text_style(&self) -> TextStyle {
    // ECMA-376 Part 1 §20.1.4.1.26: an omitted DrawingML latin face inherits
    // the theme minor font. Spreadsheet shapes use the workbook theme rather
    // than the worksheet cell-XF font; Office's pre-Aptos built-in theme is
    // Calibri when the package has no theme part.
    TextStyle {
      font_family: Some(Arc::from(
        self
          .theme_fonts
          .as_ref()
          .and_then(|fonts| fonts.minor_latin.as_deref())
          .unwrap_or("Calibri"),
      )),
      east_asia_font_family: self.theme_minor_east_asian.clone(),
      wordprocessingml_font_slots: true,
      ..TextStyle::default()
    }
  }

  pub(crate) fn document_font_text_style_for_column_width(&self) -> Option<TextStyle> {
    let mut style = TextStyle::default();
    let font = self.font_records.first()?;
    if self.missing_theme_minor_from_document_language {
      if let Some(name) = &font.name {
        style.font_family = Some(Arc::clone(name));
      }
    } else {
      self.apply_font_family(font, &mut style);
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
    Some(style)
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

  pub(crate) fn alignment_for_cell(&self, style_index: Option<u32>) -> Option<AlignmentRecord> {
    let format = self.effective_cell_format(style_index)?;
    if !format.apply_alignment {
      return None;
    }
    format.alignment
  }

  pub(crate) fn differential_fill_color(&self, format_id: u32) -> Option<RgbColor> {
    self
      .differential_format_records
      .get(format_id as usize)
      .and_then(|format| format.fill.as_ref())
      .and_then(|fill| fill.color)
  }

  pub(crate) fn table_style(&self, name: &str) -> Option<&TableStyleRecord> {
    self
      .table_style_records
      .iter()
      .find(|style| style.name == name)
  }

  pub(crate) fn differential_number_format_code(&self, format_id: u32) -> Option<&str> {
    self
      .differential_format_records
      .get(format_id as usize)
      .and_then(|format| format.number_format.as_ref())
      .map(|format| format.code.as_str())
  }

  pub(crate) fn apply_differential_text_style(&self, format_id: u32, style: &mut TextStyle) {
    let Some(font) = self
      .differential_format_records
      .get(format_id as usize)
      .and_then(|format| format.font.as_ref())
    else {
      return;
    };
    self.apply_font_family(font, style);
    if let Some(size_pt) = font.size_pt {
      style.font_size_pt = size_pt.get() as f32;
    }
    if let Some(color) = font.color {
      style.color = color;
    }
    if font.bold {
      style.bold = true;
    }
    if font.italic {
      style.italic = true;
    }
    if font.underline {
      style.underline = true;
    }
    if font.strikethrough {
      style.strikethrough = true;
    }
  }

  pub(crate) fn differential_has_font(&self, format_id: u32) -> bool {
    self
      .differential_format_records
      .get(format_id as usize)
      .is_some_and(|format| format.font.is_some())
  }

  pub(crate) fn differential_borders(&self, format_id: u32) -> Option<BorderRecord> {
    self
      .differential_format_records
      .get(format_id as usize)
      .and_then(|format| format.border)
  }

  pub(crate) fn differential_alignment(&self, format_id: u32) -> Option<AlignmentRecord> {
    self
      .differential_format_records
      .get(format_id as usize)
      .and_then(|format| format.alignment)
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

  pub(crate) fn fill_for_cell(&self, style_index: Option<u32>) -> FillRecord {
    let Some(format) = self.effective_cell_format(style_index) else {
      return FillRecord::default();
    };
    if !format.apply_fill {
      return FillRecord::default();
    }
    format
      .fill_id
      .and_then(|id| self.fill_records.get(id as usize).cloned())
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
    // Xf::createPattern enables a cell XF property when it differs from the
    // parent style XF, even if the cell XF apply flag was initially false.
    if !format.apply_font {
      format.apply_font = if format.font_id == style_xf.font_id {
        style_xf.apply_font
      } else {
        !style_xf.apply_font || format.font_id != style_xf.font_id
      }
    }
    if !format.apply_number_format {
      format.apply_number_format = if format.number_format_id == style_xf.number_format_id {
        style_xf.apply_number_format
      } else {
        !style_xf.apply_number_format || format.number_format_id != style_xf.number_format_id
      }
    }
    if !format.apply_border {
      format.apply_border = !style_xf.apply_border || format.border_id != style_xf.border_id;
    }
    if !format.apply_fill {
      format.apply_fill = !style_xf.apply_fill || format.fill_id != style_xf.fill_id;
    }
    Some(format)
  }

  fn theme_east_asian_font(&self, scheme: x::FontSchemeValues) -> Option<Arc<str>> {
    match scheme {
      x::FontSchemeValues::Major => self.theme_major_east_asian.clone(),
      x::FontSchemeValues::Minor => self.theme_minor_east_asian.clone(),
      x::FontSchemeValues::None => None,
    }
  }

  fn apply_font_family(&self, font: &FontRecord, style: &mut TextStyle) {
    if let Some(theme_font) = self.theme_east_asian_font(font.scheme) {
      // SpreadsheetML stores Calibri/Aptos alongside the major/minor scheme.
      // Office's CJK output resolves the complete cell run through the
      // theme's matching supplemental script font.
      style.font_family = Some(Arc::clone(&theme_font));
      style.east_asia_font_family = Some(theme_font);
    } else if let Some(name) = &font.name {
      style.font_family = Some(Arc::clone(name));
    }
  }

  pub(crate) fn uses_application_default_minor_theme(&self) -> bool {
    self.missing_theme_minor_from_document_language
  }

  pub(crate) fn default_font_uses_theme(&self) -> bool {
    self
      .font_records
      .first()
      .is_some_and(|font| font.scheme != x::FontSchemeValues::default())
  }

  pub(crate) fn normal_style_uses_calibri_11_minor_theme(&self) -> bool {
    let Some(normal_font) = self.font_records.first() else {
      return false;
    };
    normal_font
      .name
      .as_deref()
      .is_some_and(|font| font.eq_ignore_ascii_case("Calibri"))
      && normal_font
        .size_pt
        .is_some_and(|size| (size.get() - 11.0).abs() <= f64::EPSILON)
      && normal_font.scheme == x::FontSchemeValues::Minor
      && self
        .theme_fonts
        .as_ref()
        .and_then(|fonts| fonts.minor_latin.as_deref())
        .is_some_and(|font| font.eq_ignore_ascii_case("Calibri"))
  }

  pub(crate) fn normal_style_uses_explicit_calibri_11(&self) -> bool {
    let Some(normal_font) = self.font_records.first() else {
      return false;
    };
    normal_font
      .name
      .as_deref()
      .is_some_and(|font| font.eq_ignore_ascii_case("Calibri"))
      && normal_font
        .size_pt
        .is_some_and(|size| (size.get() - 11.0).abs() <= f64::EPSILON)
      && normal_font.scheme == x::FontSchemeValues::None
  }

  pub(crate) fn normal_style_uses_explicit_arial_10(&self) -> bool {
    let Some(normal_font) = self.font_records.first() else {
      return false;
    };
    normal_font
      .name
      .as_deref()
      .is_some_and(|font| font.eq_ignore_ascii_case("Arial"))
      && normal_font
        .size_pt
        .is_some_and(|size| (size.get() - 10.0).abs() <= f64::EPSILON)
      && normal_font.scheme == x::FontSchemeValues::None
  }

  pub(crate) fn normal_style_uses_explicit_verdana_10(&self) -> bool {
    let Some(normal_font) = self.font_records.first() else {
      return false;
    };
    normal_font
      .name
      .as_deref()
      .is_some_and(|font| font.eq_ignore_ascii_case("Verdana"))
      && normal_font
        .size_pt
        .is_some_and(|size| (size.get() - 10.0).abs() <= f64::EPSILON)
      && normal_font.scheme == x::FontSchemeValues::None
  }

  pub(crate) fn icon_set_print_metrics(&self, font_size_pt: f32) -> IconSetPrintMetrics {
    let modern_aptos = self
      .theme_fonts
      .as_ref()
      .and_then(|fonts| fonts.minor_latin.as_deref())
      .is_some_and(|font| font.eq_ignore_ascii_case("Aptos Narrow"));
    if modern_aptos {
      // Microsoft 365's Aptos Narrow theme uses the enlarged 24px icon slot
      // observed in tdf162948: 12.48x12.36pt for a 12pt cell font.
      let width_pt = units::quantize_points_to_office_print_grid(font_size_pt * 1.04);
      IconSetPrintMetrics {
        width_pt,
        height_pt: (width_pt - units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI).max(0.0),
        leading_inset_pt: 1.68,
        bottom_inset_pt: 1.0,
      }
    } else {
      // Legacy Calibri-theme Office output and Calc's ScIconSetInfo both use
      // the effective cell-font height. Office quantizes 11pt to 11.04pt on
      // its 600dpi print grid; complex_icon_set and POI's conditional-format
      // samples also retain the two legacy device insets shown here.
      let side_pt = units::quantize_points_to_office_print_grid(font_size_pt);
      IconSetPrintMetrics {
        width_pt: side_pt,
        height_pt: side_pt,
        leading_inset_pt: 1.44,
        bottom_inset_pt: 1.6,
      }
    }
  }
}

fn builtin_number_format_locale(locales: &OfficeLocaleContext) -> BuiltinNumberFormatLocale {
  if locales.format_locale_is_simplified_chinese() {
    BuiltinNumberFormatLocale::ChineseSimplified
  } else {
    BuiltinNumberFormatLocale::EnglishUs
  }
}

fn missing_theme_minor_font(
  theme_fonts: Option<&ThemeFontScheme>,
  document_resource_locale: OfficeResourceLocale,
) -> Option<Arc<str>> {
  if theme_fonts.is_some() {
    return None;
  }
  // Office's installed editing language supplies the default theme when an
  // OOXML package omits theme1.xml. Microsoft documents DengXian as the
  // Simplified Chinese Office default; retain stored Latin names for every
  // explicit non-theme font and for all other locales.
  document_resource_locale
    .is_simplified_chinese()
    .then(|| Arc::from("DengXian"))
}

impl CellFormatRecord {
  fn from_cell_format(format: &x::CellFormat, style_xf: bool) -> Self {
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
        .map_or(apply_default, |value| value.as_bool())
        || (!style_xf && format.alignment.is_some()),
      apply_protection: format
        .apply_protection
        .map_or(apply_default, |value| value.as_bool()),
      has_alignment: format.alignment.is_some(),
      alignment: format
        .alignment
        .as_ref()
        .map(AlignmentRecord::from_alignment),
    }
  }
}

impl FontRecord {
  fn from_font_with_colors(
    font: &x::Font,
    indexed_colors: &[RgbColor],
    theme_colors: &ThemeColorPalette,
  ) -> Self {
    let mut record = Self::default();
    for choice in &font.font_choice {
      match choice {
        x::FontChoice::Bold(value) => {
          record.bold = value.val.is_none_or(|value| value.as_bool());
        }
        x::FontChoice::Italic(value) => {
          record.italic = value.val.is_none_or(|value| value.as_bool());
        }
        x::FontChoice::Strike(value) => {
          record.strikethrough = value.val.is_none_or(|value| value.as_bool());
        }
        x::FontChoice::Underline(value) => {
          record.underline = !matches!(value.val, Some(x::UnderlineValues::None));
        }
        x::FontChoice::FontSize(value) => {
          record.size_pt = Some(OrderedF64::new(value.val));
        }
        x::FontChoice::Color(value) => {
          record.color = color_from_color(value, indexed_colors, theme_colors);
        }
        x::FontChoice::FontName(value) => {
          record.name = Some(Arc::from(value.val.as_str()));
        }
        x::FontChoice::FontScheme(value) => {
          record.scheme = value.val;
        }
        _ => {}
      }
    }
    record
  }
}

impl FillRecord {
  fn from_fill_with_colors(
    fill: &x::Fill,
    indexed_colors: &[RgbColor],
    theme_colors: &ThemeColorPalette,
  ) -> Self {
    let color = match &fill.fill_choice {
      Some(x::FillChoice::PatternFill(pattern)) => {
        color_from_pattern_fill(pattern, indexed_colors, theme_colors)
      }
      Some(x::FillChoice::GradientFill(gradient)) => {
        color_from_gradient_fill(gradient, indexed_colors, theme_colors)
      }
      None => None,
    };
    Self { color }
  }
}

impl BorderRecord {
  fn from_border_with_colors(
    border: &x::Border,
    indexed_colors: &[RgbColor],
    theme_colors: &ThemeColorPalette,
  ) -> Self {
    Self {
      left: border.left_border.as_deref().and_then(|border| {
        border_style(
          border.style,
          border.color.as_ref(),
          indexed_colors,
          theme_colors,
        )
      }),
      right: border.right_border.as_deref().and_then(|border| {
        border_style(
          border.style,
          border.color.as_ref(),
          indexed_colors,
          theme_colors,
        )
      }),
      top: border.top_border.as_deref().and_then(|border| {
        border_style(
          border.style,
          border.color.as_ref(),
          indexed_colors,
          theme_colors,
        )
      }),
      bottom: border.bottom_border.as_deref().and_then(|border| {
        border_style(
          border.style,
          border.color.as_ref(),
          indexed_colors,
          theme_colors,
        )
      }),
    }
  }
}

impl DifferentialFormatRecord {
  fn from_differential_format_with_colors(
    format: &x::DifferentialFormat,
    indexed_colors: &[RgbColor],
    theme_colors: &ThemeColorPalette,
  ) -> Self {
    Self {
      font: format
        .font
        .as_ref()
        .map(|font| FontRecord::from_font_with_colors(font, indexed_colors, theme_colors)),
      fill: format
        .fill
        .as_deref()
        .map(|fill| FillRecord::from_fill_with_colors(fill, indexed_colors, theme_colors)),
      border: format
        .border
        .as_deref()
        .map(|border| BorderRecord::from_border_with_colors(border, indexed_colors, theme_colors)),
      alignment: format
        .alignment
        .as_ref()
        .map(AlignmentRecord::from_alignment),
      number_format: format
        .numbering_format
        .as_ref()
        .map(|format| NumberFormatRecord {
          id: format.number_format_id,
          code: format.format_code.clone(),
        }),
    }
  }
}

impl AlignmentRecord {
  fn from_alignment(alignment: &x::Alignment) -> Self {
    let horizontal = alignment.horizontal.or_else(|| {
      // Alignment::importAlignment. Rotated OOXML cells default to left for
      // rotations below 90 degrees or exactly 180, and to right otherwise.
      alignment.text_rotation.and_then(|rotation| {
        if rotation != 0 {
          Some(if rotation < 90 || rotation == 180 {
            x::HorizontalAlignmentValues::Left
          } else {
            x::HorizontalAlignmentValues::Right
          })
        } else {
          None
        }
      })
    });
    Self {
      horizontal,
      vertical: alignment.vertical,
      text_rotation: alignment.text_rotation,
      wrap_text: alignment.wrap_text.is_some_and(|value| value.as_bool()),
      indent: alignment.indent,
      relative_indent: alignment.relative_indent,
      justify_last_line: alignment
        .justify_last_line
        .is_some_and(|value| value.as_bool()),
      shrink_to_fit: alignment.shrink_to_fit.is_some_and(|value| value.as_bool()),
      reading_order: alignment.reading_order,
    }
  }
}

fn color_from_pattern_fill(
  pattern: &x::PatternFill,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  let pattern_type = pattern.pattern_type.unwrap_or_default();
  if matches!(pattern_type, x::PatternValues::None) {
    return None;
  }
  let pattern_color = pattern
    .foreground_color
    .as_ref()
    .and_then(|color| color_from_foreground_color(color, indexed_colors, theme_colors));
  let fill_color = pattern
    .background_color
    .as_ref()
    .and_then(|color| color_from_background_color(color, indexed_colors, theme_colors));
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

fn color_from_gradient_fill(
  gradient: &x::GradientFill,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  let mut colors = gradient
    .gradient_stop
    .iter()
    .map(|stop| &stop.color)
    .filter_map(|color| color_from_color(color, indexed_colors, theme_colors));
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
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<BorderStyle> {
  let style = style?;
  if matches!(style, x::BorderStyleValues::None) {
    return None;
  }
  Some(BorderStyle {
    width_pt: border_width_pt(style),
    color: color
      .and_then(|color| color_from_color(color, indexed_colors, theme_colors))
      .unwrap_or(RgbColor { r: 0, g: 0, b: 0 }),
    compound: matches!(style, x::BorderStyleValues::Double),
    dash_pattern: border_dash_pattern(style),
    ..BorderStyle::default()
  })
}

fn border_dash_pattern(style: x::BorderStyleValues) -> BorderDashPattern {
  match style {
    x::BorderStyleValues::Dotted => BorderDashPattern::Dotted,
    x::BorderStyleValues::Dashed | x::BorderStyleValues::MediumDashed => BorderDashPattern::Dashed,
    x::BorderStyleValues::DashDot
    | x::BorderStyleValues::MediumDashDot
    | x::BorderStyleValues::SlantDashDot => BorderDashPattern::DashDot,
    x::BorderStyleValues::DashDotDot | x::BorderStyleValues::MediumDashDotDot => {
      BorderDashPattern::DashDotDot
    }
    _ => BorderDashPattern::Solid,
  }
}

fn border_width_pt(style: x::BorderStyleValues) -> f32 {
  // Excel fixed output emits the named border weights on a one-point grid:
  // thin is 1pt, medium is 2pt, and thick is 3pt. Keep dashed variants in
  // the corresponding weight group; hair remains half of thin.
  match style {
    x::BorderStyleValues::Hair => 0.5,
    x::BorderStyleValues::Thin
    | x::BorderStyleValues::Dashed
    | x::BorderStyleValues::Dotted
    | x::BorderStyleValues::DashDot
    | x::BorderStyleValues::DashDotDot
    | x::BorderStyleValues::SlantDashDot => 1.0,
    x::BorderStyleValues::Medium
    | x::BorderStyleValues::MediumDashed
    | x::BorderStyleValues::MediumDashDot
    | x::BorderStyleValues::MediumDashDotDot
    | x::BorderStyleValues::Double => 2.0,
    x::BorderStyleValues::Thick => 3.0,
    x::BorderStyleValues::None => 0.0,
  }
}

fn color_from_color(
  color: &x::Color,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  color_from_components(
    color.theme,
    color.rgb.as_deref(),
    color.indexed,
    color.tint,
    indexed_colors,
    theme_colors,
  )
}

fn color_from_foreground_color(
  color: &x::ForegroundColor,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  color_from_components(
    color.theme,
    color.rgb.as_deref(),
    color.indexed,
    color.tint,
    indexed_colors,
    theme_colors,
  )
}

fn color_from_background_color(
  color: &x::BackgroundColor,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  color_from_components(
    color.theme,
    color.rgb.as_deref(),
    color.indexed,
    color.tint,
    indexed_colors,
    theme_colors,
  )
}

fn color_from_components(
  theme: Option<u32>,
  rgb: Option<&str>,
  indexed: Option<u32>,
  tint: Option<f64>,
  indexed_colors: &[RgbColor],
  theme_colors: &ThemeColorPalette,
) -> Option<RgbColor> {
  // sc/source/filter/oox/stylesbuffer.cxx XlsColor::importColor(). Excel gives
  // theme precedence over rgb and indexed when malformed input supplies more
  // than one representation.
  let base = theme
    .and_then(|index| theme_colors.get(index))
    .or_else(|| color_from_ooxml(rgb))
    .or_else(|| indexed.and_then(|index| indexed_colors.get(index as usize).copied()))?;
  Some(match tint {
    Some(tint) if tint != 0.0 => rgb_from_resolved(apply_excel_tint(
      ResolvedColor::new(base.r, base.g, base.b),
      tint,
    )),
    _ => base,
  })
}

fn rgb_from_resolved(color: ResolvedColor) -> RgbColor {
  RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn office_border_weights_follow_fixed_output_point_grid() {
    assert_eq!(border_width_pt(x::BorderStyleValues::Hair), 0.5);
    assert_eq!(border_width_pt(x::BorderStyleValues::Thin), 1.0);
    assert_eq!(border_width_pt(x::BorderStyleValues::Medium), 2.0);
    assert_eq!(border_width_pt(x::BorderStyleValues::Thick), 3.0);
  }

  #[test]
  fn excel_border_dash_styles_are_preserved() {
    assert_eq!(
      border_dash_pattern(x::BorderStyleValues::Dotted),
      BorderDashPattern::Dotted
    );
    assert_eq!(
      border_dash_pattern(x::BorderStyleValues::MediumDashed),
      BorderDashPattern::Dashed
    );
    assert_eq!(
      border_dash_pattern(x::BorderStyleValues::SlantDashDot),
      BorderDashPattern::DashDot
    );
    assert_eq!(
      border_dash_pattern(x::BorderStyleValues::MediumDashDotDot),
      BorderDashPattern::DashDotDot
    );
  }

  #[test]
  fn rotated_alignment_defaults_match_libreoffice_import() {
    // Alignment::importAlignment and
    // sc/qa/unit/subsequent_export_test2.cxx:testTdf120168.
    let left = AlignmentRecord::from_alignment(&x::Alignment {
      text_rotation: Some(45),
      ..x::Alignment::default()
    });
    let right = AlignmentRecord::from_alignment(&x::Alignment {
      text_rotation: Some(135),
      ..x::Alignment::default()
    });
    let upside_down = AlignmentRecord::from_alignment(&x::Alignment {
      text_rotation: Some(180),
      ..x::Alignment::default()
    });
    let horizontal = AlignmentRecord::from_alignment(&x::Alignment {
      text_rotation: Some(0),
      ..x::Alignment::default()
    });

    assert_eq!(left.horizontal, Some(x::HorizontalAlignmentValues::Left));
    assert_eq!(right.horizontal, Some(x::HorizontalAlignmentValues::Right));
    assert_eq!(
      upside_down.horizontal,
      Some(x::HorizontalAlignmentValues::Left)
    );
    assert_eq!(horizontal.horizontal, None);
  }

  #[test]
  fn cjk_theme_scheme_replaces_the_stored_latin_snapshot_for_the_cell_run() {
    let catalog = StylesCatalog {
      theme_minor_east_asian: Some(Arc::from("SimSun")),
      ..StylesCatalog::default()
    };
    let font = FontRecord {
      name: Some(Arc::from("Calibri")),
      scheme: x::FontSchemeValues::Minor,
      ..FontRecord::default()
    };
    let mut style = TextStyle::default();

    catalog.apply_font_family(&font, &mut style);

    assert_eq!(style.font_family.as_deref(), Some("SimSun"));
    assert_eq!(style.east_asia_font_family.as_deref(), Some("SimSun"));
  }

  #[test]
  fn builtin_short_date_format_follows_the_format_locale() {
    let simplified_chinese = StylesCatalog {
      builtin_number_format_locale: builtin_number_format_locale(&OfficeLocaleContext::new(
        None,
        Some("zh-CN"),
        None,
      )),
      ..StylesCatalog::default()
    };
    let english = StylesCatalog::default();

    assert_eq!(simplified_chinese.number_format_code(14), Some("YYYY/M/D"));
    assert_eq!(
      simplified_chinese.number_format_code(22),
      Some("YYYY/M/D h:mm")
    );
    assert_eq!(english.number_format_code(14), Some("M/D/YYYY"));
  }

  #[test]
  fn drawing_text_inherits_the_workbook_minor_latin_theme_font() {
    let catalog = StylesCatalog::default();

    let style = catalog.default_drawing_text_style();

    assert_eq!(style.font_family.as_deref(), Some("Calibri"));
  }

  #[test]
  fn nondefault_direct_font_color_has_precedence_over_table_formatting() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let dark_gray = RgbColor {
      r: 0x26,
      g: 0x26,
      b: 0x26,
    };
    let cell_format = |font_id| CellFormatRecord {
      number_format_id: None,
      font_id: Some(font_id),
      fill_id: None,
      border_id: None,
      style_xf_id: None,
      quote_prefix: false,
      pivot_button: false,
      apply_number_format: false,
      apply_font: true,
      apply_fill: false,
      apply_border: false,
      apply_alignment: false,
      apply_protection: false,
      has_alignment: false,
      alignment: None,
    };
    let catalog = StylesCatalog {
      cell_xfs: vec![cell_format(0), cell_format(1)],
      font_records: vec![
        FontRecord {
          color: Some(black),
          ..FontRecord::default()
        },
        FontRecord {
          color: Some(dark_gray),
          ..FontRecord::default()
        },
      ],
      ..StylesCatalog::default()
    };

    assert_eq!(catalog.direct_nondefault_font_color_for_cell(Some(0)), None);
    assert_eq!(
      catalog.direct_nondefault_font_color_for_cell(Some(1)),
      Some(dark_gray)
    );
  }

  #[test]
  fn missing_theme_uses_the_simplified_chinese_office_minor_font() {
    let catalog = StylesCatalog {
      theme_minor_east_asian: missing_theme_minor_font(
        None,
        OfficeResourceLocale::SimplifiedChinese,
      ),
      missing_theme_minor_from_document_language: true,
      ..StylesCatalog::default()
    };
    let themed_font = FontRecord {
      name: Some(Arc::from("Calibri")),
      scheme: x::FontSchemeValues::Minor,
      ..FontRecord::default()
    };
    let explicit_font = FontRecord {
      name: Some(Arc::from("Calibri")),
      ..FontRecord::default()
    };

    let mut themed_style = TextStyle::default();
    catalog.apply_font_family(&themed_font, &mut themed_style);
    let mut explicit_style = TextStyle::default();
    catalog.apply_font_family(&explicit_font, &mut explicit_style);

    assert_eq!(themed_style.font_family.as_deref(), Some("DengXian"));
    assert_eq!(explicit_style.font_family.as_deref(), Some("Calibri"));
    assert!(catalog.uses_application_default_minor_theme());
    assert_eq!(
      missing_theme_minor_font(None, OfficeResourceLocale::English),
      None
    );
  }

  #[test]
  fn workbook_without_a_styles_part_uses_the_application_default_font() {
    let catalog = StylesCatalog {
      theme_minor_east_asian: Some(Arc::from("DengXian")),
      missing_theme_minor_from_document_language: true,
      ..StylesCatalog::default()
    };

    let style = catalog.default_font_text_style();

    assert_eq!(style.font_family.as_deref(), Some("DengXian"));
    assert_eq!(style.east_asia_font_family.as_deref(), Some("DengXian"));
  }

  #[test]
  fn explicit_column_width_keeps_the_stored_normal_font_snapshot() {
    let catalog = StylesCatalog {
      font_records: vec![FontRecord {
        name: Some(Arc::from("Calibri")),
        scheme: x::FontSchemeValues::Minor,
        ..FontRecord::default()
      }],
      theme_minor_east_asian: Some(Arc::from("DengXian")),
      missing_theme_minor_from_document_language: true,
      ..StylesCatalog::default()
    };

    assert_eq!(
      catalog.default_font_text_style().font_family.as_deref(),
      Some("DengXian")
    );
    assert_eq!(
      catalog
        .document_font_text_style_for_column_width()
        .unwrap()
        .font_family
        .as_deref(),
      Some("Calibri")
    );
  }

  #[test]
  fn missing_default_font_keeps_unit_converter_digit_fallback() {
    let catalog = StylesCatalog::default();

    assert!(
      catalog
        .document_font_text_style_for_column_width()
        .is_none()
    );
  }

  #[test]
  fn calibri_application_column_profile_requires_the_normal_style_font() {
    let theme_fonts = ThemeFontScheme {
      minor_latin: Some("Calibri".to_owned()),
      ..ThemeFontScheme::default()
    };
    let calibri = StylesCatalog {
      font_records: vec![FontRecord {
        name: Some(Arc::from("Calibri")),
        size_pt: Some(OrderedF64::new(11.0)),
        scheme: x::FontSchemeValues::Minor,
        ..FontRecord::default()
      }],
      theme_fonts: Some(theme_fonts.clone()),
      ..StylesCatalog::default()
    };
    let arial = StylesCatalog {
      font_records: vec![FontRecord {
        name: Some(Arc::from("Arial")),
        size_pt: Some(OrderedF64::new(11.0)),
        ..FontRecord::default()
      }],
      theme_fonts: Some(theme_fonts),
      ..StylesCatalog::default()
    };

    assert!(calibri.normal_style_uses_calibri_11_minor_theme());
    assert!(!arial.normal_style_uses_calibri_11_minor_theme());
  }
}
