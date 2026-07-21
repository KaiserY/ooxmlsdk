use std::borrow::Cow;
use std::sync::Arc;

use crate::common;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyle {
  pub width_pt: f32,
  pub spacing_pt: f32,
  pub color: RgbColor,
  pub compound: bool,
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      spacing_pt: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      compound: false,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
  pub font_family: Option<Arc<str>>,
  pub fallback_font_family: Option<Arc<str>>,
  pub east_asia_font_family: Option<Arc<str>>,
  pub complex_font_family: Option<Arc<str>>,
  pub symbol_font_family: Option<Arc<str>>,
  pub font_size_pt: f32,
  pub complex_font_size_pt: Option<f32>,
  /// Minimum WordprocessingML font size at which OpenType kerning is active.
  /// `None` leaves the shaping engine's native default unchanged.
  pub kerning_minimum_size_pt: Option<f32>,
  /// OpenType ligature categories selected by the source document. `None`
  /// leaves the shaping engine's native defaults unchanged.
  pub ligatures: Option<common::OpenTypeLigatures>,
  /// Horizontal WordprocessingML character scale. `None` means 100% and
  /// preserves explicit 100% overrides in style inheritance.
  pub horizontal_scale: Option<f32>,
  pub character_spacing_pt: f32,
  pub baseline_shift_pt: f32,
  pub use_windows_font_metrics: bool,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub small_caps: bool,
  pub hidden: bool,
  pub rotation_deg: f32,
  pub color: RgbColor,
  pub opacity: f32,
  pub outline_color: Option<RgbColor>,
  pub outline_opacity: f32,
  pub outline_width_pt: f32,
  pub highlight: Option<RgbColor>,
  pub underline_color: Option<RgbColor>,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_family: None,
      fallback_font_family: None,
      east_asia_font_family: None,
      complex_font_family: None,
      symbol_font_family: None,
      font_size_pt: 11.0,
      complex_font_size_pt: None,
      kerning_minimum_size_pt: None,
      ligatures: None,
      horizontal_scale: None,
      character_spacing_pt: 0.0,
      baseline_shift_pt: 0.0,
      use_windows_font_metrics: false,
      bold: false,
      italic: false,
      underline: false,
      strikethrough: false,
      uppercase: false,
      small_caps: false,
      hidden: false,
      rotation_deg: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      opacity: 1.0,
      outline_color: None,
      outline_opacity: 1.0,
      outline_width_pt: 0.0,
      highlight: None,
      underline_color: None,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RgbColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImageCrop {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSetup {
  pub width_pt: f32,
  pub height_pt: f32,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
  pub mirror_margins: bool,
  pub top_margin_was_negative: bool,
  pub bottom_margin_was_negative: bool,
  pub header_distance_pt: f32,
  pub footer_distance_pt: f32,
  pub background: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub borders_offset_from_text: bool,
  pub line_numbering: Option<LineNumbering>,
  pub doc_grid_line_pitch_pt: Option<f32>,
  pub adjust_table_line_heights_to_grid: bool,
  pub page_number_start: Option<i32>,
}

impl Default for PageSetup {
  fn default() -> Self {
    Self {
      // The fixed-output reference environment uses A4 as its default paper.
      // Explicit w:pgSz and SpreadsheetML paper sizes replace these values.
      width_pt: 595.2756,
      height_pt: 841.8898,
      margin_top_pt: 72.0,
      margin_right_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
      mirror_margins: false,
      top_margin_was_negative: false,
      bottom_margin_was_negative: false,
      header_distance_pt: 36.0,
      footer_distance_pt: 36.0,
      background: None,
      borders: CellBordersModel::default(),
      borders_offset_from_text: false,
      line_numbering: None,
      doc_grid_line_pitch_pt: None,
      adjust_table_line_heights_to_grid: false,
      page_number_start: None,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineNumbering {
  pub count_by: i16,
  pub start: i16,
  pub distance_pt: f32,
  pub restart_each_page: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DynamicFieldKind {
  Page,
  NumPages,
  PageRef {
    bookmark_name: Arc<str>,
  },
  StyleRef {
    style_name: Arc<str>,
    from_bottom: bool,
  },
}

#[derive(Clone, Debug)]
pub struct FormWidget {
  pub id: u32,
  pub kind: FormWidgetKind,
  pub entries: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormWidgetKind {
  Text,
  DropDownList,
  ComboBox,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  LinkArea(LinkAreaItem),
  Path(common::PathItem<'static>),
  Rect(RectItem),
  Line(LineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub line_height_pt: f32,
  pub text: String,
  pub style: TextStyle,
  pub rotation_center_pt: Option<(f32, f32)>,
  pub hyperlink_url: Option<String>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub preserve_text_portion: bool,
  pub pdf_text_segmentation: PdfTextSegmentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PdfTextSegmentation {
  Line,
  Portion,
}

#[derive(Clone, Debug)]
pub(crate) struct ImageItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub crop: ImageCrop,
  pub clip_path: Vec<common::PathCommand>,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub data: Arc<[u8]>,
  pub content_type: Option<String>,
  pub alt_text: Option<String>,
  pub hyperlink_url: Option<String>,
  pub floating: bool,
  pub behind_text: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct LinkAreaItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub hyperlink_url: String,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RectItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub fill_color: Option<RgbColor>,
  pub fill_opacity: f32,
  pub stroke: Option<BorderStyle>,
  pub stroke_opacity: f32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
  pub color: RgbColor,
  pub kind: LineItemKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineItemKind {
  Stroke,
}

pub(crate) fn common_page_setup(setup: PageSetup) -> common::PageSetup {
  common::PageSetup {
    size: common::Size {
      width: common::Pt(setup.width_pt),
      height: common::Pt(setup.height_pt),
    },
    margins: common::Insets {
      top: common::Pt(setup.margin_top_pt),
      right: common::Pt(setup.margin_right_pt),
      bottom: common::Pt(setup.margin_bottom_pt),
      left: common::Pt(setup.margin_left_pt),
    },
    mirror_margins: setup.mirror_margins,
    top_margin_was_negative: setup.top_margin_was_negative,
    bottom_margin_was_negative: setup.bottom_margin_was_negative,
    header_distance: common::Pt(setup.header_distance_pt),
    footer_distance: common::Pt(setup.footer_distance_pt),
    background: setup.background.map(|color| common_rgb(color, 1.0)),
    borders: common::CellBorders {
      top: setup.borders.top.map(common_border_style),
      right: setup.borders.right.map(common_border_style),
      bottom: setup.borders.bottom.map(common_border_style),
      left: setup.borders.left.map(common_border_style),
    },
    borders_offset_from_text: setup.borders_offset_from_text,
    line_numbering: setup.line_numbering.map(|line| common::LineNumbering {
      count_by: line.count_by,
      start: line.start,
      distance: common::Pt(line.distance_pt),
      restart_each_page: line.restart_each_page,
    }),
    doc_grid_line_pitch: setup.doc_grid_line_pitch_pt.map(common::Pt),
    page_number_start: setup.page_number_start,
  }
}

pub(crate) fn common_text_style(style: TextStyle) -> common::TextStyle<'static> {
  common::TextStyle {
    font_family: style.font_family.map(|value| Cow::Owned(value.to_string())),
    fallback_font_family: style
      .fallback_font_family
      .map(|value| Cow::Owned(value.to_string())),
    east_asia_font_family: style
      .east_asia_font_family
      .map(|value| Cow::Owned(value.to_string())),
    complex_font_family: style
      .complex_font_family
      .map(|value| Cow::Owned(value.to_string())),
    symbol_font_family: style
      .symbol_font_family
      .map(|value| Cow::Owned(value.to_string())),
    font_size: common::Pt(style.font_size_pt),
    complex_font_size: style.complex_font_size_pt.map(common::Pt),
    kerning_minimum_size: style.kerning_minimum_size_pt.map(common::Pt),
    ligatures: style.ligatures,
    horizontal_scale: style.horizontal_scale,
    character_spacing: common::Pt(style.character_spacing_pt),
    baseline_shift: common::Pt(style.baseline_shift_pt),
    use_windows_font_metrics: style.use_windows_font_metrics,
    bold: style.bold,
    italic: style.italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
    rotation_degrees: style.rotation_deg,
    color: common_rgb(style.color, style.opacity),
    outline_color: style
      .outline_color
      .map(|color| common_rgb(color, style.outline_opacity)),
    outline_width: common::Pt(style.outline_width_pt),
    highlight: style.highlight.map(|color| common_rgb(color, 1.0)),
    underline_color: style.underline_color.map(|color| common_rgb(color, 1.0)),
  }
}

pub(crate) fn common_border_style(style: BorderStyle) -> common::BorderStyle {
  common::BorderStyle {
    width: common::Pt(style.width_pt),
    spacing: common::Pt(style.spacing_pt),
    color: common_rgb(style.color, 1.0),
    compound: style.compound,
  }
}

pub(crate) fn common_stroke_from_border(
  style: BorderStyle,
  opacity: f32,
) -> common::Stroke<'static> {
  common::Stroke {
    width: common::Pt(style.width_pt),
    color: common_rgb(style.color, opacity),
    dash: None,
    source_style_id: None,
  }
}

pub(crate) fn common_rect(x: f32, y: f32, width: f32, height: f32) -> common::Rect {
  common::Rect {
    origin: common_point(x, y),
    size: common::Size {
      width: common::Pt(width),
      height: common::Pt(height),
    },
  }
}

pub(crate) fn common_point(x: f32, y: f32) -> common::Point {
  common::Point {
    x: common::Pt(x),
    y: common::Pt(y),
  }
}

pub(crate) fn common_rgb(color: RgbColor, opacity: f32) -> common::Color {
  common::Color {
    r: color.r,
    g: color.g,
    b: color.b,
    a: (opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
  }
}
