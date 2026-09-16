use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

use rustc_hash::FxHashMap as HashMap;
use skrifa::{
  FontRef as SkrifaFontRef, MetadataProvider,
  instance::{LocationRef as SkrifaLocationRef, Size as SkrifaSize},
  raw::TableProvider as SkrifaTableProvider,
  string::StringId as SkrifaStringId,
};
use smallvec::SmallVec;

use super::page::pdf_page_dimension;
use crate::options::PdfOptions;
use crate::{
  PdfConversionDiagnostics, PdfFontAudit, PdfFontAuditIssue, PdfFontAuditIssueKind,
  PdfFontFaceDiagnostics, PdfGlyphBoundsDiagnostics, PdfGlyphDiagnostics, PdfGlyphRunDiagnostics,
  PdfPageDiagnostics, PdfTextPortionDiagnostics, PdfTextPortionKind, PdfTextRunDiagnostics,
};
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceData, FontStyleRef};
use ooxmlsdk_layout::text_metrics::{TextMetrics, TextVerticalMetrics};

type PaintTextPortionRanges = SmallVec<[(PaintTextPortionKind, Range<usize>); 2]>;
pub(super) type PaintGlyphFontRuns = SmallVec<[PaintGlyphFontRun; 2]>;

pub(super) fn prepare_for_direct<'doc>(
  document: &'doc common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> PaintDocument<'doc> {
  let mut text_metrics = TextMetrics::new();
  PaintDocument::from_layout(document, &mut text_metrics, options.ui_language.as_deref())
}

#[derive(Clone, Debug)]
pub(super) struct PaintDocument<'doc> {
  pub(super) pages: Vec<PaintPage<'doc>>,
}

#[derive(Clone, Debug)]
pub(super) struct PaintPage<'doc> {
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) items: Vec<PaintItem<'doc>>,
}

#[derive(Clone, Debug)]
enum PageItem<'doc> {
  Text(Box<TextItem<'doc>>),
  Image(ImageItem<'doc>),
  Group {
    mask: Option<ImageItem<'doc>>,
    clip: Option<PaintClipRect>,
    transform: Option<common::Transform>,
    blend_mode: common::BlendMode,
    opacity: f32,
    flatten_identity: bool,
    inherit_text_line_owner: bool,
    items: Vec<PageItem<'doc>>,
  },
  LinkArea(LinkAreaItem<'doc>),
  Rect(RectItem<'doc>),
  Line(LineItem),
  Polyline(PolylineItem<'doc>),
}

#[derive(Clone, Debug)]
pub(super) struct TextItem<'doc> {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) line_height_pt: f32,
  line_metrics_participant: bool,
  pub(super) paint_clip: Option<PaintClipRect>,
  page_culling_bounds: Option<PaintClipRect>,
  pub(super) text: Cow<'doc, str>,
  pub(super) style: TextStyle<'doc>,
  pub(super) rotation_center_pt: Option<(f32, f32)>,
  pub(super) hyperlink_url: Option<Cow<'doc, str>>,
  // Dynamic fields are uncommon, while this item is stored in a mixed page
  // enum for every text run. Keep the cold payload indirect so ordinary text
  // does not inflate every non-text enum variant or require boxing the hot
  // TextItem itself.
  pub(super) dynamic_field: Option<Box<common::DynamicField<'doc>>>,
  pub(super) form_widget_id: Option<u32>,
  paragraph_bidi: bool,
  word_spacing_pt: f32,
  preserve_text_portion: bool,
  decoration_span_start_x_pt: Option<f32>,
  pdf_text_segmentation: common::PdfTextSegmentation,
  source_path: Option<&'doc [usize]>,
  semantic_target_width_pt: Option<f32>,
}

#[derive(Clone, Debug)]
pub(super) struct ImageItem<'doc> {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) crop: ImageCrop,
  pub(super) clip_path: &'doc [common::PathCommand],
  pub(super) rotation_deg: f32,
  pub(super) flip_horizontal: bool,
  pub(super) flip_vertical: bool,
  pub(super) data: Cow<'doc, [u8]>,
  pub(super) content_type: Option<Cow<'doc, str>>,
  pub(super) blip_compression_state: common::BlipCompressionState,
  pub(super) metafile_monochrome_dib_palette_override: Option<[[u8; 3]; 2]>,
  pub(super) metafile_background_color: Option<[u8; 3]>,
  pub(super) metafile_external_header: Option<ooxmlsdk_layout::render::emf_wmf::WmfExternalHeader>,
  pub(super) metafile_fixed_output_profile: common::MetafileFixedOutputProfile,
  pub(super) alt_text: Option<Cow<'doc, str>>,
  pub(super) hyperlink_url: Option<Cow<'doc, str>>,
  pub(super) semantic_metafile_text: bool,
  pub(super) metafile_semantic_text_includes_raster_backdrop: bool,
  pub(super) signature_line: Option<common::SignatureLineProperties<'doc>>,
  pub(super) metafile_native_size: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct ImageCrop {
  pub(super) left: f32,
  pub(super) top: f32,
  pub(super) right: f32,
  pub(super) bottom: f32,
}

#[derive(Clone, Debug)]
pub(super) struct LinkAreaItem<'doc> {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) hyperlink_url: Cow<'doc, str>,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RectFill<'doc> {
  Solid { color: RgbColor, opacity: f32 },
  Gradient(&'doc common::GradientFill<'static>),
  Pattern(&'doc common::PatternFill),
}

#[derive(Clone, Copy, Debug)]
pub(super) struct RectItem<'doc> {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) fill: Option<RectFill<'doc>>,
  pub(super) stroke: Option<BorderStyle>,
  pub(super) stroke_opacity: f32,
}

#[derive(Clone, Debug)]
pub(super) struct LineItem {
  pub(super) x1_pt: f32,
  pub(super) y1_pt: f32,
  pub(super) x2_pt: f32,
  pub(super) y2_pt: f32,
  pub(super) width_pt: f32,
  pub(super) color: RgbColor,
  pub(super) opacity: f32,
  pub(super) dash: Option<Vec<f32>>,
  pub(super) dash_offset: f32,
  pub(super) line_cap: PaintLineCap,
  pub(super) kind: LineItemKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PaintLineCap {
  Butt,
  Round,
  Square,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LineItemKind {
  Stroke,
  FilledRect,
}

#[derive(Clone, Debug)]
pub(super) struct PolylineItem<'doc> {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) points: &'doc [common::Point],
  pub(super) commands: &'doc [common::PathCommand],
  pub(super) closed: bool,
  pub(super) fill: &'doc common::Fill<'static>,
  pub(super) stroke: Option<&'doc common::Stroke<'static>>,
  pub(super) separate_fill_and_stroke: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct BorderStyle {
  pub(super) width_pt: f32,
  pub(super) color: RgbColor,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct RgbColor {
  pub(super) r: u8,
  pub(super) g: u8,
  pub(super) b: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct TextStyle<'doc> {
  font_family: Option<Cow<'doc, str>>,
  high_ansi_font_family: Option<Cow<'doc, str>>,
  fallback_font_family: Option<Cow<'doc, str>>,
  high_ansi_fallback_font_family: Option<Cow<'doc, str>>,
  east_asia_fallback_font_family: Option<Cow<'doc, str>>,
  complex_fallback_font_family: Option<Cow<'doc, str>>,
  font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  high_ansi_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  east_asia_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  complex_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  east_asia_font_family: Option<Cow<'doc, str>>,
  complex_font_family: Option<Cow<'doc, str>>,
  symbol_font_family: Option<Cow<'doc, str>>,
  pub(super) explicit_symbol_character: bool,
  font_size_pt: f32,
  complex_font_size_pt: Option<f32>,
  layout_font_sizes: Option<common::LayoutFontSizes>,
  complex_script: Option<bool>,
  right_to_left: Option<bool>,
  resolved_bidi_level: Option<u8>,
  kerning_minimum_size_pt: Option<f32>,
  ligatures: Option<common::OpenTypeLigatures>,
  open_type_features: common::OpenTypeFeatureSettings,
  pub(super) horizontal_scale: Option<f32>,
  semantic_character_advances_pt: Option<Arc<[f32]>>,
  character_spacing_pt: f32,
  baseline_shift_pt: f32,
  automatic_escapement_font_size_pt: Option<f32>,
  automatic_escapement_complex_font_size_pt: Option<f32>,
  line_vertical_alignment: common::LineVerticalAlignment,
  use_windows_font_metrics: bool,
  wordprocessingml_font_slots: bool,
  wordprocessingml_cjk_line_metrics: bool,
  wordprocessingml_font_hint: Option<ooxmlsdk_fonts::WordprocessingFontTypeHint>,
  wordprocessingml_east_asia_language_is_chinese: bool,
  font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  high_ansi_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  wordprocessingml_east_asia_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  complex_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  high_ansi_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  east_asia_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  complex_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  cjk_punctuation_compression_ratio: f32,
  wordprocessingml_balance_single_byte_double_byte_width: bool,
  pub(super) pdf_glyph_outlines: bool,
  pub(super) pdf_glyph_outline_options: Option<common::PdfGlyphOutlineOptions>,
  pub(super) bold: bool,
  pub(super) italic: bool,
  complex_bold: Option<bool>,
  complex_italic: Option<bool>,
  pub(super) underline: bool,
  pub(super) strikethrough: bool,
  pub(super) uppercase: bool,
  pub(super) small_caps: bool,
  pub(super) hidden: bool,
  pub(super) semantic_only: bool,
  /// The text origin is already the metafile playback baseline rather than a
  /// document-layout line-box origin.
  metafile_reference_baseline: bool,
  pub(super) rotation_deg: f32,
  pub(super) color: RgbColor,
  pub(super) opacity: f32,
  pub(super) outline_color: Option<RgbColor>,
  pub(super) outline_opacity: f32,
  pub(super) outline_width_pt: f32,
  pub(super) highlight: Option<RgbColor>,
  underline_color: Option<RgbColor>,
}

impl FontStyleRef for TextStyle<'_> {
  fn font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }

  fn symbol_font_family(&self) -> Option<&str> {
    self.symbol_font_family.as_deref()
  }

  fn high_ansi_font_family(&self) -> Option<&str> {
    self
      .high_ansi_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family.as_deref()
  }

  fn high_ansi_fallback_font_family(&self) -> Option<&str> {
    self.high_ansi_fallback_font_family.as_deref()
  }

  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    self.east_asia_fallback_font_family.as_deref()
  }

  fn complex_fallback_font_family(&self) -> Option<&str> {
    self.complex_fallback_font_family.as_deref()
  }

  fn font_family_class(&self) -> Option<ooxmlsdk_fonts::FontFamilyClass> {
    self.font_family_class
  }

  fn high_ansi_font_family_class(&self) -> Option<ooxmlsdk_fonts::FontFamilyClass> {
    self.high_ansi_font_family_class
  }

  fn east_asia_font_family_class(&self) -> Option<ooxmlsdk_fonts::FontFamilyClass> {
    self.east_asia_font_family_class
  }

  fn complex_font_family_class(&self) -> Option<ooxmlsdk_fonts::FontFamilyClass> {
    self.complex_font_family_class
  }

  fn east_asia_font_family(&self) -> Option<&str> {
    self
      .east_asia_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn complex_font_family(&self) -> Option<&str> {
    self
      .complex_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn font_size_pt(&self) -> f32 {
    self.font_size_pt
  }

  fn complex_font_size_pt(&self) -> Option<f32> {
    self.complex_font_size_pt
  }

  fn complex_script_override(&self) -> Option<bool> {
    if self.complex_script == Some(true) || self.right_to_left == Some(true) {
      Some(true)
    } else {
      None
    }
  }

  fn right_to_left(&self) -> bool {
    self.right_to_left == Some(true)
  }

  fn resolved_bidi_level(&self) -> Option<u8> {
    self.resolved_bidi_level
  }

  fn complex_bold(&self) -> Option<bool> {
    self.complex_bold
  }

  fn complex_italic(&self) -> Option<bool> {
    self.complex_italic
  }

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing_pt
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift_pt
  }

  fn automatic_escapement_font_sizes_pt(&self) -> Option<(f32, Option<f32>)> {
    self
      .automatic_escapement_font_size_pt
      .map(|size| (size, self.automatic_escapement_complex_font_size_pt))
  }

  fn bold(&self) -> bool {
    self.bold
  }

  fn italic(&self) -> bool {
    self.italic
  }

  fn small_caps(&self) -> bool {
    self.small_caps
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    self.wordprocessingml_font_slots
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.wordprocessingml_cjk_line_metrics
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    self.wordprocessingml_font_hint
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    self.wordprocessingml_east_asia_language_is_chinese
  }

  fn font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.font_charset
  }

  fn high_ansi_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.high_ansi_font_charset.or(self.font_charset)
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.wordprocessingml_east_asia_font_charset
  }

  fn complex_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.complex_font_charset
  }

  fn font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.font_pitch
  }

  fn high_ansi_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.high_ansi_font_pitch.or(self.font_pitch)
  }

  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.east_asia_font_pitch
  }

  fn complex_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.complex_font_pitch
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self.wordprocessingml_balance_single_byte_double_byte_width
  }

  fn kerning_enabled(&self) -> bool {
    let font_size_pt = if self.complex_script_override() == Some(true) {
      self.complex_font_size_pt.unwrap_or(self.font_size_pt)
    } else {
      self.font_size_pt
    };
    self
      .kerning_minimum_size_pt
      .is_none_or(|minimum| font_size_pt + f32::EPSILON >= minimum)
  }

  fn ligatures(&self) -> Option<common::OpenTypeLigatures> {
    self.ligatures
  }

  fn open_type_features(&self) -> common::OpenTypeFeatureSettings {
    self.open_type_features
  }

  fn horizontal_scale(&self) -> f32 {
    self.horizontal_scale.unwrap_or(1.0)
  }
}

impl TextStyle<'_> {
  pub(super) fn pdf_font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }
}

impl PaintText<'_> {
  pub(super) fn tag_source_frame_index(&self) -> Option<usize> {
    self.source_frame_index
  }

  pub(super) fn tag_source_line_index(&self) -> Option<usize> {
    self.source_line_index
  }

  pub(super) fn tag_source_path(&self) -> Option<&[usize]> {
    self.item.source_path
  }

  /// Whether serializing this item emits page-stream paint or semantic text.
  ///
  /// A zero-alpha/no-fill run still participates in layout and can own a link,
  /// but Office's fixed-format writers omit its glyphs and font resources. A
  /// highlight or decoration remains independent paint and semantic-only text
  /// deliberately emits a clipped searchable carrier.
  pub(super) fn emits_page_content(&self) -> bool {
    self.portions.iter().any(|portion| {
      portion.highlight.is_some()
        || portion.underline.is_some()
        || portion.strikethrough.is_some()
        || (!matches!(portion.kind, PaintTextPortionKind::Tab)
          && text_has_visible_glyph_paint(&self.item.style))
    })
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FollowFrameKind {
  Paragraph,
  Table,
  Notes,
}

#[derive(Clone, Copy, Debug, Default)]
struct DecorationRenderMetadata {
  suppress: bool,
  span_start_x_pt: Option<f32>,
}

#[derive(Clone, Debug)]
pub(super) enum PaintItem<'doc> {
  Text(Box<PaintText<'doc>>),
  Image(ImageItem<'doc>),
  Group {
    mask: Option<ImageItem<'doc>>,
    clip: Option<PaintClipRect>,
    transform: Option<common::Transform>,
    blend_mode: common::BlendMode,
    opacity: f32,
    flatten_identity: bool,
    items: Vec<PaintItem<'doc>>,
  },
  LinkArea(LinkAreaItem<'doc>),
  Rect(RectItem<'doc>),
  Line(LineItem),
  Polyline(PolylineItem<'doc>),
}

#[derive(Clone, Debug)]
pub(super) struct PaintText<'doc> {
  pub(super) item: TextItem<'doc>,
  source_frame_index: Option<usize>,
  source_line_index: Option<usize>,
  pub(super) baseline_y: f32,
  width_pt: f32,
  pub(super) portions: Vec<PaintTextPortion>,
}

#[derive(Clone, Debug)]
pub(super) struct PaintTextPortion {
  pub(super) kind: PaintTextPortionKind,
  pub(super) text_range: std::ops::Range<usize>,
  pub(super) x_pt: f32,
  pub(super) baseline_y: f32,
  pub(super) width_pt: f32,
  pub(super) clip: Option<PaintClipRect>,
  pub(super) glyphs: Option<PaintGlyphFontRuns>,
  pub(super) highlight: Option<PaintRect>,
  pub(super) underline: Option<PaintStrokeLine>,
  pub(super) strikethrough: Option<PaintStrokeLine>,
  pub(super) link: Option<PaintLink>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PaintTextPortionKind {
  Text,
  Tab,
  Field,
  Link,
}

#[derive(Clone, Debug)]
struct PaintGlyphRun {
  width_pt: f32,
  font_runs: PaintGlyphFontRuns,
}

#[derive(Clone, Debug)]
pub(super) struct PaintGlyphFontRun {
  pub(super) font_face: FontFaceData,
  pub(super) font_size_pt: f32,
  pub(super) x_offset_pt: f32,
  pub(super) glyphs: Vec<PaintGlyph>,
}

#[derive(Clone, Debug)]
pub(super) struct PaintGlyph {
  // Keep the shaping bounds beside the exact normalized values consumed by
  // the serializer instead of reconstructing them from the serialized PDF.
  pub(super) glyph_id: u32,
  pub(super) text_range: Range<usize>,
  pub(super) x_advance: f32,
  pub(super) x_offset: f32,
  pub(super) y_offset: f32,
  pub(super) y_advance: f32,
  pub(super) bounds_em: Option<PdfGlyphBoundsDiagnostics>,
}

pub(super) fn word_small_caps_semantic_text(text: &str, small_caps: bool) -> Cow<'_, str> {
  if !small_caps || !text.chars().any(char::is_lowercase) {
    return Cow::Borrowed(text);
  }

  let mut uppercase = String::with_capacity(text.len());
  for character in text.chars() {
    let mapped = character.to_uppercase().collect::<String>();
    if mapped.len() != character.len_utf8() {
      // Glyph clusters still address the original UTF-8 ranges. Keep the
      // source text when a case mapping would move those boundaries; a
      // future explicit semantic-range map can cover that uncommon case.
      return Cow::Borrowed(text);
    }
    uppercase.push_str(&mapped);
  }

  // ECMA-376 Part 1 section 17.3.2.33 keeps the OOXML Unicode unchanged while
  // displaying lowercase letters as smaller capitals. Word fixed output
  // exposes those displayed capitals through PDF ToUnicode.
  Cow::Owned(uppercase)
}

pub(super) fn word_no_break_hyphen_semantic_text(text: &str) -> Cow<'_, str> {
  if !text.contains('\u{2011}') {
    return Cow::Borrowed(text);
  }

  // Word preserves no-break layout behavior but exposes the displayed hyphen
  // as U+002D in fixed-output ToUnicode. Change semantics after shaping so the
  // line-breaking input remains U+2011.
  Cow::Owned(
    text
      .chars()
      .map(|character| {
        if character == '\u{2011}' {
          '-'
        } else {
          character
        }
      })
      .collect(),
  )
}

pub(super) fn remap_glyph_text_ranges<'a>(
  glyphs: &'a [PaintGlyph],
  source_text: &str,
  semantic_text: &str,
) -> Option<Cow<'a, [PaintGlyph]>> {
  let mut source_boundaries = source_text
    .char_indices()
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  source_boundaries.push(source_text.len());
  let mut semantic_boundaries = semantic_text
    .char_indices()
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  semantic_boundaries.push(semantic_text.len());

  // Every supported semantic substitution owns exactly one source scalar.
  // Refuse to infer cluster ownership if a future mapping violates that
  // contract.
  if source_boundaries.len() != semantic_boundaries.len() {
    return None;
  }
  if source_boundaries == semantic_boundaries {
    return Some(Cow::Borrowed(glyphs));
  }

  let mut remapped = Vec::with_capacity(glyphs.len());
  for glyph in glyphs {
    let start = source_boundaries
      .binary_search(&glyph.text_range.start)
      .ok()?;
    let end = source_boundaries
      .binary_search(&glyph.text_range.end)
      .ok()?;
    let mut glyph = glyph.clone();
    glyph.text_range = semantic_boundaries[start]..semantic_boundaries[end];
    remapped.push(glyph);
  }
  Some(Cow::Owned(remapped))
}

pub(super) fn merge_variation_selector_font_runs<'a>(
  glyph_runs: &'a PaintGlyphFontRuns,
  source_text: &str,
) -> Cow<'a, PaintGlyphFontRuns> {
  let mut merged = glyph_runs.clone();
  let mut changed = false;
  let mut run_index = 0;
  while run_index < merged.len() {
    let mut glyph_index = 0;
    while glyph_index < merged[run_index].glyphs.len() {
      let glyph = &merged[run_index].glyphs[glyph_index];
      let selector = source_text
        .get(glyph.text_range.clone())
        .is_some_and(|text| {
          !text.is_empty() && text.chars().all(is_unicode_text_presentation_selector)
        });
      let zero_width_no_ink = glyph.x_advance.abs() <= f32::EPSILON
        && (glyph.glyph_id == 0
          || glyph.bounds_em.is_some_and(|bounds| {
            bounds.x_min_em.abs() <= f32::EPSILON
              && bounds.y_min_em.abs() <= f32::EPSILON
              && bounds.x_max_em.abs() <= f32::EPSILON
              && bounds.y_max_em.abs() <= f32::EPSILON
          }));
      if !(selector && zero_width_no_ink) {
        glyph_index += 1;
        continue;
      }
      let previous = if glyph_index > 0 {
        Some((run_index, glyph_index - 1))
      } else {
        (0..run_index).rev().find_map(|previous_run| {
          merged[previous_run]
            .glyphs
            .len()
            .checked_sub(1)
            .map(|previous_glyph| (previous_run, previous_glyph))
        })
      };
      let Some((previous_run, previous_glyph)) = previous else {
        glyph_index += 1;
        continue;
      };
      let selector_end = glyph.text_range.end;
      merged[previous_run].glyphs[previous_glyph].text_range.end = selector_end;
      merged[run_index].glyphs.remove(glyph_index);
      changed = true;
    }
    if merged[run_index].glyphs.is_empty() {
      merged.remove(run_index);
    } else {
      run_index += 1;
    }
  }
  if changed {
    Cow::Owned(merged)
  } else {
    Cow::Borrowed(glyph_runs)
  }
}

pub(super) fn is_unicode_text_presentation_selector(character: char) -> bool {
  character == '\u{fe0e}'
}

pub(super) fn symbol_font_semantic_text<'a>(
  text: &'a str,
  font_family: Option<&str>,
) -> Cow<'a, str> {
  let symbol = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Symbol") || family.eq_ignore_ascii_case("SymbolMT")
  });
  let wingdings = font_family.is_some_and(|family| family.eq_ignore_ascii_case("Wingdings"));
  let wingdings_2 = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Wingdings 2") || family.eq_ignore_ascii_case("Wingdings2")
  });
  let wingdings_3 = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Wingdings 3") || family.eq_ignore_ascii_case("Wingdings3")
  });
  let webdings = font_family.is_some_and(|family| family.eq_ignore_ascii_case("Webdings"));
  let mt_extra = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("MT Extra") || family.eq_ignore_ascii_case("MTExtra")
  });
  let bookshelf_symbol_7 =
    font_family.is_some_and(|family| family.eq_ignore_ascii_case("Bookshelf Symbol 7"));
  if !(symbol
    || wingdings
    || wingdings_2
    || wingdings_3
    || webdings
    || mt_extra
    || bookshelf_symbol_7)
  {
    return Cow::Borrowed(text);
  }

  // ECMA-376 Part 1 section 17.3.3.30 defines F000-offset storage as a
  // legacy glyph selector, not portable Unicode semantics. LibreOffice
  // fontcvt.cxx supplies Adobe Symbol mappings; Unicode WG2 N4363 supplies
  // standardized Webdings/Wingdings mappings. Keep the selected glyph and
  // alter only ToUnicode semantics.
  let mut changed = false;
  let mapped = text
    .chars()
    .map(|character| {
      let mapped = match character {
        '\u{f020}' if symbol => '\u{0020}',
        '\u{f02a}' if symbol => '\u{2217}',
        '\u{f02d}' if symbol => '\u{2212}',
        '\u{f028}' if symbol => '\u{0028}',
        '\u{f031}' if symbol => '\u{0031}',
        '\u{f05e}' if symbol => '\u{22a5}',
        '\u{f061}' if symbol => '\u{03b1}',
        '\u{f062}' if symbol => '\u{03b2}',
        '\u{f0a2}' if symbol => '\u{2032}',
        '\u{f0a3}' if symbol => '\u{2264}',
        '\u{f0b3}' if symbol => '\u{2265}',
        '\u{f0b4}' if symbol => '\u{00d7}',
        '\u{f0b7}' if symbol => '\u{2022}',
        '\u{f0b9}' if symbol => '\u{2260}',
        '\u{f0c9}' if symbol => '\u{2283}',
        '\u{f0ca}' if symbol => '\u{2287}',
        '\u{f0cb}' if symbol => '\u{2284}',
        '\u{f0cc}' if symbol => '\u{2282}',
        '\u{f0cd}' if symbol => '\u{2286}',
        '\u{f0ce}' if symbol => '\u{2208}',
        '\u{f0cf}' if symbol => '\u{2209}',
        '\u{f0d5}' if symbol => '\u{220f}',
        '\u{f0d6}' if symbol => '\u{221a}',
        '\u{f0d7}' if symbol => '\u{22c5}',
        '\u{f0de}' if symbol => '\u{21d2}',
        '\u{f0e5}' if symbol => '\u{2211}',
        '\u{f0e6}' if symbol => '\u{239b}',
        '\u{f0e7}' if symbol => '\u{239c}',
        '\u{f0e8}' if symbol => '\u{239d}',
        '\u{f0f6}' if symbol => '\u{239e}',
        '\u{f0f7}' if symbol => '\u{239f}',
        '\u{f0f8}' if symbol => '\u{23a0}',
        '\u{006f}' if bookshelf_symbol_7 => '\u{f06f}',
        '\u{f04a}' if wingdings => '\u{263a}',
        '\u{f04c}' if wingdings => '\u{2639}',
        '\u{f04d}' if wingdings => '\u{1f4a3}',
        '\u{f04f}' if wingdings => '\u{1f3f3}',
        '\u{f06c}' if wingdings => '\u{26ab}',
        '\u{f06e}' if wingdings => '\u{25fc}',
        '\u{f06f}' if wingdings => '\u{1f78f}',
        '\u{f071}' if wingdings => '\u{2751}',
        '\u{f075}' if wingdings => '\u{25c6}',
        '\u{f076}' if wingdings => '\u{2756}',
        '\u{f097}' if wingdings => '\u{1f660}',
        '\u{f0a3}' if wingdings => '\u{1f788}',
        '\u{f0a7}' if wingdings => '\u{25aa}',
        '\u{f0c9}' if wingdings => '\u{2bb6}',
        '\u{f0ca}' if wingdings => '\u{2bb7}',
        '\u{f0cb}' if wingdings => '\u{1f66a}',
        '\u{f0cc}' if wingdings => '\u{1f66b}',
        '\u{f0cd}' if wingdings => '\u{1f655}',
        '\u{f0ce}' if wingdings => '\u{1f654}',
        '\u{f0cf}' if wingdings => '\u{1f657}',
        '\u{f0d5}' if wingdings => '\u{232b}',
        '\u{f0d8}' if wingdings => '\u{27a2}',
        '\u{f0e0}' if wingdings => '\u{2192}',
        '\u{f0e7}' if wingdings => '\u{1f878}',
        '\u{f0e8}' if wingdings => '\u{1f87a}',
        '\u{f0fb}' if wingdings => '\u{1f5f6}',
        '\u{f0fc}' if wingdings => '\u{2713}',
        '\u{f0fd}' if wingdings => '\u{1f5f7}',
        '\u{f0fe}' if wingdings => '\u{1f5f9}',
        '\u{f020}' if wingdings => '\u{2002}',
        '\u{f097}' if wingdings_2 => '\u{2981}',
        '\u{f0a3}' if wingdings_2 => '\u{25a1}',
        // Unicode WG2 N4363, Webdings IDs 0069, 0073 and 0074
        // (decimal legacy selectors), pp. 22-23: desert, beach, island.
        '\u{f045}' if webdings => '\u{1f3dc}',
        '\u{f049}' if webdings => '\u{26f1}',
        '\u{f04a}' if webdings => '\u{1f3dd}',
        '\u{f067}' if webdings => '\u{2b1b}',
        '\u{f06e}' if webdings => '\u{2b24}',
        '\u{f07d}' if wingdings_3 => '\u{1f782}',
        '\u{f04c}' if mt_extra => '\u{22ef}',
        '\u{f04d}' if mt_extra => '\u{22ee}',
        '\u{f04f}' if mt_extra => '\u{22f1}',
        _ => character,
      };
      changed |= mapped != character;
      mapped
    })
    .collect();
  if changed {
    Cow::Owned(mapped)
  } else {
    Cow::Borrowed(text)
  }
}

pub(super) fn conversion_diagnostics(paint: &PaintDocument<'_>) -> PdfConversionDiagnostics {
  let mut fonts = Vec::new();
  let mut font_indices = HashMap::default();
  let pages = paint
    .pages
    .iter()
    .enumerate()
    .map(|(page_index, page)| {
      let text_runs = page
        .items
        .iter()
        .filter_map(|item| match item {
          PaintItem::Text(text) => Some(text_run_diagnostics(text, &mut fonts, &mut font_indices)),
          _ => None,
        })
        .collect();
      PdfPageDiagnostics {
        page_index,
        width_pt: page.width_pt,
        height_pt: page.height_pt,
        text_runs,
      }
    })
    .collect();
  PdfConversionDiagnostics { fonts, pages }
}

const MAX_FONT_AUDIT_ISSUES: usize = 64;

/// Audits the exact shared paint list consumed by the direct serializer.
///
/// Font parsing, outline support, subsetting, and embedding are validated by
/// the direct writer itself, so the audit reports exactly that backend's input
/// instead of imposing a second renderer's font-constructor boundary.
pub(super) fn conversion_font_audit_for_direct(paint: &PaintDocument<'_>) -> PdfFontAudit {
  let mut audit = PdfFontAudit::default();
  let mut font_indices = HashMap::default();
  for (page_index, page) in paint.pages.iter().enumerate() {
    let mut text_run_index = 0;
    for item in &page.items {
      let PaintItem::Text(text) = item else {
        continue;
      };
      for (portion_index, portion) in text.portions.iter().enumerate() {
        audit.text_portion_count += 1;
        let visible = !matches!(portion.kind, PaintTextPortionKind::Tab)
          && text_has_visible_glyph_paint(&text.item.style);
        let source_requires_visible_glyph =
          source_range_requires_visible_glyph(&text.item.text, &portion.text_range);
        let painted_as_text = visible && !text_requires_glyph_outlines(&text.item.style);
        if painted_as_text {
          audit.painted_text_portion_count += 1;
        }
        if !valid_text_range(&text.item.text, &portion.text_range) {
          push_font_audit_issue(
            &mut audit,
            PdfFontAuditIssue {
              page_index,
              text_run_index,
              portion_index: Some(portion_index),
              glyph_run_index: None,
              glyph_index: None,
              kind: PdfFontAuditIssueKind::PortionTextRange,
              detail: format!(
                "range={:?}, text_len={}",
                portion.text_range,
                text.item.text.len()
              ),
            },
          );
        }
        let Some(glyph_runs) = &portion.glyphs else {
          if visible && source_requires_visible_glyph {
            push_font_audit_issue(
              &mut audit,
              PdfFontAuditIssue {
                page_index,
                text_run_index,
                portion_index: Some(portion_index),
                glyph_run_index: None,
                glyph_index: None,
                kind: PdfFontAuditIssueKind::MissingShapedGlyphs,
                detail: format!("range={:?}", portion.text_range),
              },
            );
          }
          continue;
        };
        audit.explicit_glyph_portion_count += 1;
        for (glyph_run_index, run) in glyph_runs.iter().enumerate() {
          audit.glyph_run_count += 1;
          if painted_as_text {
            let mut in_multi_glyph_cluster = false;
            for glyphs in run.glyphs.windows(2) {
              if glyphs[0].text_range == glyphs[1].text_range {
                if !in_multi_glyph_cluster {
                  audit.actual_text_cluster_count += 1;
                }
                in_multi_glyph_cluster = true;
              } else {
                in_multi_glyph_cluster = false;
              }
            }
          }
          let key = run.font_face.cache_key();
          let font_index = if let Some(index) = font_indices.get(&key) {
            *index
          } else {
            let index = audit.fonts.len();
            let font = font_face_diagnostics(&run.font_face);
            if let Some(error) = &font.parse_error {
              push_font_audit_issue(
                &mut audit,
                PdfFontAuditIssue {
                  page_index,
                  text_run_index,
                  portion_index: Some(portion_index),
                  glyph_run_index: Some(glyph_run_index),
                  glyph_index: None,
                  kind: PdfFontAuditIssueKind::FontParse,
                  detail: format!("font_id={:?}, error={error}", font.font_id),
                },
              );
            }
            audit.fonts.push(font);
            font_indices.insert(key, index);
            index
          };
          if !run.x_offset_pt.is_finite() {
            push_font_audit_issue(
              &mut audit,
              PdfFontAuditIssue {
                page_index,
                text_run_index,
                portion_index: Some(portion_index),
                glyph_run_index: Some(glyph_run_index),
                glyph_index: None,
                kind: PdfFontAuditIssueKind::NonFiniteGlyphMetric,
                detail: format!("font_index={font_index}, x_offset_pt={}", run.x_offset_pt),
              },
            );
          }
          if !run.font_size_pt.is_finite() || run.font_size_pt <= 0.0 {
            push_font_audit_issue(
              &mut audit,
              PdfFontAuditIssue {
                page_index,
                text_run_index,
                portion_index: Some(portion_index),
                glyph_run_index: Some(glyph_run_index),
                glyph_index: None,
                kind: PdfFontAuditIssueKind::NonFiniteGlyphMetric,
                detail: format!("font_index={font_index}, font_size_pt={}", run.font_size_pt),
              },
            );
          }
          for (glyph_index, glyph) in run.glyphs.iter().enumerate() {
            audit.glyph_count += 1;
            let location = || PdfFontAuditIssue {
              page_index,
              text_run_index,
              portion_index: Some(portion_index),
              glyph_run_index: Some(glyph_run_index),
              glyph_index: Some(glyph_index),
              kind: PdfFontAuditIssueKind::GlyphTextRange,
              detail: String::new(),
            };
            if !valid_text_range(&text.item.text, &glyph.text_range) {
              let mut issue = location();
              issue.detail = format!(
                "font_index={font_index}, range={:?}, text_len={}",
                glyph.text_range,
                text.item.text.len()
              );
              push_font_audit_issue(&mut audit, issue);
            }
            let (font_parsed, font_glyph_count, resolved_family) = {
              let font = &audit.fonts[font_index];
              (
                font.parse_error.is_none(),
                font.glyph_count,
                font.family_names.first().cloned(),
              )
            };
            if font_parsed && glyph.glyph_id >= u32::from(font_glyph_count) {
              let mut issue = location();
              issue.kind = PdfFontAuditIssueKind::GlyphIdOutOfRange;
              issue.detail = format!(
                "font_index={font_index}, glyph_id={}, glyph_count={}",
                glyph.glyph_id, font_glyph_count
              );
              push_font_audit_issue(&mut audit, issue);
            }
            if visible
              && glyph.glyph_id == 0
              && glyph_requires_font_coverage(
                &text.item.text,
                glyph,
                text.item.style.explicit_symbol_character,
              )
            {
              if text.item.style.explicit_symbol_character {
                audit.explicit_symbol_notdef_glyph_count += 1;
              } else {
                let mut issue = location();
                issue.kind = PdfFontAuditIssueKind::MissingGlyph;
                let source_text = text
                  .item
                  .text
                  .get(glyph.text_range.clone())
                  .unwrap_or("<invalid-range>");
                issue.detail = format!(
                  "font_index={font_index}, requested_family={:?}, resolved_family={:?}, text={source_text:?}, range={:?}",
                  text.item.style.font_family, resolved_family, glyph.text_range
                );
                push_font_audit_issue(&mut audit, issue);
              }
            }
            if ![
              glyph.x_advance,
              glyph.x_offset,
              glyph.y_offset,
              glyph.y_advance,
            ]
            .into_iter()
            .all(f32::is_finite)
            {
              let mut issue = location();
              issue.kind = PdfFontAuditIssueKind::NonFiniteGlyphMetric;
              issue.detail = format!(
                "font_index={font_index}, advance=({}, {}), offset=({}, {})",
                glyph.x_advance, glyph.y_advance, glyph.x_offset, glyph.y_offset
              );
              push_font_audit_issue(&mut audit, issue);
            }
            if let Some(bounds) = glyph.bounds_em
              && (![
                bounds.x_min_em,
                bounds.y_min_em,
                bounds.x_max_em,
                bounds.y_max_em,
              ]
              .into_iter()
              .all(f32::is_finite)
                || bounds.x_min_em > bounds.x_max_em
                || bounds.y_min_em > bounds.y_max_em)
            {
              let mut issue = location();
              issue.kind = PdfFontAuditIssueKind::InvalidGlyphBounds;
              issue.detail = format!("font_index={font_index}, bounds={bounds:?}");
              push_font_audit_issue(&mut audit, issue);
            }
          }
        }
      }
      text_run_index += 1;
    }
  }
  audit
}

fn valid_text_range(text: &str, range: &Range<usize>) -> bool {
  range.start <= range.end
    && range.end <= text.len()
    && text.is_char_boundary(range.start)
    && text.is_char_boundary(range.end)
}

fn source_range_requires_visible_glyph(text: &str, range: &Range<usize>) -> bool {
  text
    .get(range.clone())
    .is_some_and(|source| source.chars().any(|ch| !ch.is_control()))
}

fn glyph_requires_font_coverage(
  text: &str,
  glyph: &PaintGlyph,
  explicit_symbol_character: bool,
) -> bool {
  let Some(source) = text.get(glyph.text_range.clone()) else {
    return true;
  };
  if !source.chars().any(|ch| !ch.is_control()) {
    return false;
  }

  // Ordinary PUA text has font-specific semantics rather than portable
  // character coverage. Excluding it from a missing-character audit does not
  // suppress painting: the selected face's .notdef can have a visible outline
  // (as in Word's Times New Roman PUA output). Explicit w:sym selectors are
  // accounted for separately by the caller, even for an inkless glyph.
  let private_use_only =
    !explicit_symbol_character && !source.is_empty() && source.chars().all(is_unicode_private_use);
  if !private_use_only {
    return true;
  }

  glyph.glyph_id != 0
    && glyph
      .bounds_em
      .is_some_and(|bounds| bounds.x_min_em < bounds.x_max_em && bounds.y_min_em < bounds.y_max_em)
}

pub(super) fn is_unicode_private_use(character: char) -> bool {
  matches!(
    character as u32,
    0xe000..=0xf8ff | 0xf0000..=0xffffd | 0x100000..=0x10fffd
  )
}

fn push_font_audit_issue(audit: &mut PdfFontAudit, issue: PdfFontAuditIssue) {
  if audit.issues.len() < MAX_FONT_AUDIT_ISSUES {
    audit.issues.push(issue);
  }
}

fn text_run_diagnostics(
  text: &PaintText<'_>,
  fonts: &mut Vec<PdfFontFaceDiagnostics>,
  font_indices: &mut HashMap<ooxmlsdk_layout::fonts::FontFaceCacheKey, usize>,
) -> PdfTextRunDiagnostics {
  let portions = text
    .portions
    .iter()
    .map(|portion| {
      let glyph_runs = portion
        .glyphs
        .iter()
        .flatten()
        .map(|run| {
          let key = run.font_face.cache_key();
          let font_index = *font_indices.entry(key).or_insert_with(|| {
            let index = fonts.len();
            fonts.push(font_face_diagnostics(&run.font_face));
            index
          });
          PdfGlyphRunDiagnostics {
            font_index,
            font_size_pt: run.font_size_pt,
            x_offset_pt: run.x_offset_pt,
            synthetic_bold: run.font_face.synthetic_bold,
            synthetic_italic: run.font_face.synthetic_italic,
            glyphs: run
              .glyphs
              .iter()
              .map(|glyph| PdfGlyphDiagnostics {
                glyph_id: glyph.glyph_id,
                text_range_start: glyph.text_range.start,
                text_range_end: glyph.text_range.end,
                x_advance_em: glyph.x_advance,
                x_offset_em: glyph.x_offset,
                y_offset_em: glyph.y_offset,
                y_advance_em: glyph.y_advance,
                bounds_em: glyph.bounds_em,
              })
              .collect(),
          }
        })
        .collect();
      PdfTextPortionDiagnostics {
        kind: match portion.kind {
          PaintTextPortionKind::Text => PdfTextPortionKind::Text,
          PaintTextPortionKind::Tab => PdfTextPortionKind::Tab,
          PaintTextPortionKind::Field => PdfTextPortionKind::Field,
          PaintTextPortionKind::Link => PdfTextPortionKind::Link,
        },
        text_range_start: portion.text_range.start,
        text_range_end: portion.text_range.end,
        x_pt: portion.x_pt,
        baseline_y_pt: portion.baseline_y,
        width_pt: portion.width_pt,
        has_explicit_glyphs: portion.glyphs.is_some(),
        glyph_runs,
      }
    })
    .collect();
  PdfTextRunDiagnostics {
    text: text.item.text.to_string(),
    source_frame_index: text.source_frame_index,
    source_line_index: text.source_line_index,
    source_path: text
      .item
      .source_path
      .map_or_else(Vec::new, |path| path.to_vec()),
    x_pt: text.item.x_pt,
    y_pt: text.item.y_pt,
    baseline_y_pt: text.baseline_y,
    line_height_pt: text.item.line_height_pt,
    width_pt: text.width_pt,
    font_size_pt: text.item.style.font_size_pt,
    character_spacing_pt: text.item.style.character_spacing_pt,
    baseline_shift_pt: text.item.style.baseline_shift_pt,
    requested_font_family: text.item.style.font_family.as_deref().map(str::to_string),
    requested_east_asia_font_family: text
      .item
      .style
      .east_asia_font_family
      .as_deref()
      .map(str::to_string),
    requested_complex_font_family: text
      .item
      .style
      .complex_font_family
      .as_deref()
      .map(str::to_string),
    bold: text.item.style.bold,
    italic: text.item.style.italic,
    small_caps: text.item.style.small_caps,
    portions,
  }
}

fn font_face_diagnostics(face_data: &FontFaceData) -> PdfFontFaceDiagnostics {
  let data = face_data.data.as_slice();
  // Face index plus the OpenType `head` checksum adjustment and data length
  // distinguish faces without hashing a multi-megabyte font per glyph run.
  let parsed_face = SkrifaFontRef::from_index(data, face_data.index);
  let checksum_adjustment = parsed_face
    .as_ref()
    .ok()
    .and_then(|face| face.head().ok())
    .map(|head| head.checksum_adjustment());
  let face = match parsed_face {
    Ok(face) => face,
    Err(error) => {
      return PdfFontFaceDiagnostics {
        font_id: face_data.id().to_string(),
        face_index: face_data.index,
        data_len: data.len(),
        parse_error: Some(error.to_string()),
        checksum_adjustment,
        postscript_name: None,
        family_names: Vec::new(),
        style_name: None,
        units_per_em: 0,
        glyph_count: 0,
        ascender_em: 0.0,
        descender_em: 0.0,
        cap_height_em: None,
        global_bounds_em: PdfGlyphBoundsDiagnostics::default(),
        monospaced: false,
      };
    }
  };
  let metrics = face.metrics(SkrifaSize::new(1.0), SkrifaLocationRef::default());
  let units_per_em = metrics.units_per_em;
  let mut family_names = Vec::new();
  for name_id in [
    SkrifaStringId::FAMILY_NAME,
    SkrifaStringId::TYPOGRAPHIC_FAMILY_NAME,
  ] {
    for name in face.localized_strings(name_id) {
      let value = name.to_string();
      if !family_names.contains(&value) {
        family_names.push(value);
      }
    }
  }
  let font_name = |name_id| {
    face
      .localized_strings(name_id)
      .english_or_first()
      .map(|name| name.to_string())
  };
  let postscript_name = font_name(SkrifaStringId::POSTSCRIPT_NAME);
  let style_name = font_name(SkrifaStringId::SUBFAMILY_NAME);
  let bounds = metrics.bounds.unwrap_or_default();
  PdfFontFaceDiagnostics {
    font_id: face_data.id().to_string(),
    face_index: face_data.index,
    data_len: data.len(),
    parse_error: None,
    checksum_adjustment,
    postscript_name,
    family_names,
    style_name,
    units_per_em,
    glyph_count: metrics.glyph_count,
    ascender_em: metrics.ascent,
    descender_em: metrics.descent,
    cap_height_em: metrics.cap_height,
    global_bounds_em: PdfGlyphBoundsDiagnostics {
      x_min_em: bounds.x_min,
      y_min_em: bounds.y_min,
      x_max_em: bounds.x_max,
      y_max_em: bounds.y_max,
    },
    monospaced: metrics.is_monospace,
  }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PaintRect {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
  pub(super) color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PaintClipRect {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
}

fn paint_clip_from_common(rect: common::Rect) -> PaintClipRect {
  PaintClipRect {
    x_pt: rect.origin.x.0,
    y_pt: rect.origin.y.0,
    width_pt: rect.size.width.0,
    height_pt: rect.size.height.0,
  }
}

fn intersect_paint_clips(
  left: Option<PaintClipRect>,
  right: Option<PaintClipRect>,
) -> Option<PaintClipRect> {
  match (left, right) {
    (Some(left), Some(right)) => {
      let x_pt = left.x_pt.max(right.x_pt);
      let y_pt = left.y_pt.max(right.y_pt);
      let right_pt = (left.x_pt + left.width_pt).min(right.x_pt + right.width_pt);
      let bottom_pt = (left.y_pt + left.height_pt).min(right.y_pt + right.height_pt);
      Some(PaintClipRect {
        x_pt,
        y_pt,
        width_pt: (right_pt - x_pt).max(0.0),
        height_pt: (bottom_pt - y_pt).max(0.0),
      })
    }
    (Some(clip), None) | (None, Some(clip)) => Some(clip),
    (None, None) => None,
  }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PaintStrokeLine {
  pub(super) x1_pt: f32,
  pub(super) y1_pt: f32,
  pub(super) x2_pt: f32,
  pub(super) y2_pt: f32,
  pub(super) width_pt: f32,
  pub(super) color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PaintLink {
  pub(super) x_pt: f32,
  pub(super) y_pt: f32,
  pub(super) width_pt: f32,
  pub(super) height_pt: f32,
}

fn decoration_render_metadata(items: &[PageItem<'_>]) -> Vec<DecorationRenderMetadata> {
  let mut metadata = vec![DecorationRenderMetadata::default(); items.len()];
  let mut index = 0usize;

  while index < items.len() {
    let Some(PageItem::Text(text)) = items.get(index) else {
      index += 1;
      continue;
    };

    if !text.style.underline && !text.style.strikethrough {
      index += 1;
      continue;
    }

    let start_index = index;
    let start_x_pt = text.x_pt;
    let mut end_index = index;

    while end_index + 1 < items.len() {
      let Some(PageItem::Text(next)) = items.get(end_index + 1) else {
        break;
      };
      if !decoration_compatible(text, next) {
        break;
      }
      end_index += 1;
    }

    if end_index > start_index {
      for entry in metadata.iter_mut().take(end_index).skip(start_index) {
        entry.suppress = true;
      }
      metadata[end_index].span_start_x_pt = Some(start_x_pt);
    }

    index = end_index + 1;
  }

  metadata
}

fn decoration_compatible(current: &TextItem<'_>, next: &TextItem<'_>) -> bool {
  current.style == next.style
    && current.hyperlink_url == next.hyperlink_url
    && current.dynamic_field == next.dynamic_field
    && (current.y_pt - next.y_pt).abs() < 0.01
    && (current.line_height_pt - next.line_height_pt).abs() < 0.01
}

impl<'doc> PaintDocument<'doc> {
  fn from_layout(
    document: &'doc common::LayoutDocument<'static>,
    text_metrics: &mut TextMetrics,
    ui_language: Option<&str>,
  ) -> Self {
    let separate_path_fill_and_stroke = document.engine_kind == common::LayoutEngineKind::Pptx;
    let pages = document
      .pages
      .iter()
      .enumerate()
      .map(|(page_index, page)| {
        let source_line_owners = paint_line_owners(document, page_index, &page.items);
        let page_items = page
          .items
          .iter()
          .enumerate()
          .filter_map(|(item_index, item)| {
            page_item_from_common(
              item,
              ui_language,
              text_metrics,
              separate_path_fill_and_stroke,
            )
            .map(|item| (item, source_line_owners.get(item_index).copied().flatten()))
          })
          .collect::<Vec<_>>();
        let (layout_items, line_owners) = coalesced_writer_text_items(page_items, text_metrics);
        let (layout_items, line_owners) =
          expand_metafile_semantic_text_items(layout_items, line_owners, ui_language);
        let common_line_baselines =
          common_writer_line_baselines(&layout_items, &line_owners, text_metrics);
        let decoration_metadata = decoration_render_metadata(&layout_items);
        let items = layout_items
          .into_iter()
          .enumerate()
          .map(|(item_index, item)| {
            let owner = line_owners.get(item_index).copied().flatten();
            let common_line_baseline = common_line_baselines.get(item_index).copied().flatten();
            match item {
              PageItem::Text(mut text) => {
                let metadata = decoration_metadata[item_index];
                if metadata.suppress {
                  text.style.underline = false;
                  text.style.strikethrough = false;
                }
                text.decoration_span_start_x_pt = metadata.span_start_x_pt;
                PaintItem::Text(Box::new(PaintText::from_layout_text(
                  *text,
                  owner,
                  common_line_baseline,
                  page.setup.size.width.0,
                  text_metrics,
                )))
              }
              PageItem::Image(image) => PaintItem::Image(image),
              PageItem::Group {
                mask,
                clip,
                transform,
                blend_mode,
                opacity,
                flatten_identity,
                inherit_text_line_owner,
                items,
              } => {
                let mut items = {
                  let child_owner = inherit_text_line_owner.then_some(owner).flatten();
                  let child_line_baseline = inherit_text_line_owner
                    .then_some(common_line_baseline)
                    .flatten();
                  items
                    .into_iter()
                    .map(|item| {
                      paint_group_item(
                        item,
                        child_owner,
                        child_line_baseline,
                        page.setup.size.width.0,
                        text_metrics,
                      )
                    })
                    .collect::<Vec<_>>()
                };
                if clip.is_some() && flatten_identity {
                  // A clip-only worksheet wrapper replaces several former
                  // top-level drawing items. Preserve their original
                  // physical-page culling before emitting text operators;
                  // clipping alone still leaves invisible text extractable
                  // from a PDF content stream.
                  items.retain(|item| {
                    paint_item_intersects_page(
                      item,
                      page.setup.size.width.0,
                      page.setup.size.height.0,
                    )
                  });
                }
                PaintItem::Group {
                  mask,
                  clip,
                  transform,
                  blend_mode,
                  opacity,
                  flatten_identity,
                  items,
                }
              }
              PageItem::LinkArea(link_area) => PaintItem::LinkArea(link_area),
              PageItem::Rect(rect) => PaintItem::Rect(rect),
              PageItem::Line(line) => PaintItem::Line(line),
              PageItem::Polyline(polyline) => PaintItem::Polyline(polyline),
            }
          })
          .collect();
        PaintPage {
          width_pt: pdf_page_dimension(document.engine_kind, page.setup.size.width.0),
          height_pt: pdf_page_dimension(document.engine_kind, page.setup.size.height.0),
          items,
        }
      })
      .collect();
    Self { pages }
  }
}

fn expand_metafile_semantic_text_items<'doc>(
  items: Vec<PageItem<'doc>>,
  owners: Vec<Option<PaintLineOwner>>,
  ui_language: Option<&str>,
) -> (Vec<PageItem<'doc>>, Vec<Option<PaintLineOwner>>) {
  let mut expanded_items = Vec::with_capacity(items.len());
  let mut expanded_owners = Vec::with_capacity(owners.len());
  for (item, owner) in items.into_iter().zip(owners) {
    expanded_items.push(expand_metafile_semantic_text_item(item, owner, ui_language));
    expanded_owners.push(owner);
  }
  (expanded_items, expanded_owners)
}

fn expand_metafile_semantic_text_item<'doc>(
  item: PageItem<'doc>,
  owner: Option<PaintLineOwner>,
  ui_language: Option<&str>,
) -> PageItem<'doc> {
  match item {
    PageItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity,
      inherit_text_line_owner,
      items,
    } => {
      let child_owner = inherit_text_line_owner.then_some(owner).flatten();
      let child_count = items.len();
      let (items, _) =
        expand_metafile_semantic_text_items(items, vec![child_owner; child_count], ui_language);
      PageItem::Group {
        mask,
        clip,
        transform,
        blend_mode,
        opacity,
        flatten_identity,
        inherit_text_line_owner,
        items,
      }
    }
    PageItem::Image(image)
      if (image.semantic_metafile_text || image.signature_line.is_some())
        && image.rotation_deg.abs() <= f32::EPSILON
        && !image.flip_horizontal
        && !image.flip_vertical
        && image.crop == ImageCrop::default() =>
    {
      let extraction_options = ooxmlsdk_layout::render::emf_wmf::RenderOptions {
        wmf_external_header: image.metafile_external_header,
        ..ooxmlsdk_layout::render::emf_wmf::RenderOptions::default()
      };
      if let Some(signature_line) = image
        .signature_line
        .as_ref()
        .filter(|properties| properties.state == common::SignatureLineState::Unsigned)
      {
        let preview_runs =
          ooxmlsdk_layout::render::emf_wmf::extract_metafile_text_runs_with_options(
            &image.data,
            image.content_type.as_deref(),
            true,
            extraction_options,
          );
        let items = word_unsigned_signature_line_items(
          image.x_pt,
          image.y_pt,
          image.width_pt,
          image.height_pt,
          signature_line,
          &preview_runs,
          ui_language,
        );
        return PageItem::Group {
          mask: None,
          clip: None,
          transform: None,
          blend_mode: common::BlendMode::Normal,
          opacity: 1.0,
          flatten_identity: true,
          inherit_text_line_owner: true,
          items,
        };
      }
      let include_raster_backdrop_text = image.metafile_semantic_text_includes_raster_backdrop;
      let localize_signature_ui_text = image
        .signature_line
        .as_ref()
        .is_some_and(|properties| properties.state == common::SignatureLineState::Unsigned);
      let solid_rects = if include_raster_backdrop_text {
        ooxmlsdk_layout::render::emf_wmf::extract_metafile_solid_rects_with_options(
          &image.data,
          image.content_type.as_deref(),
          extraction_options,
        )
      } else {
        Vec::new()
      }
      .into_iter()
      .map(|rect| {
        PageItem::Rect(RectItem {
          x_pt: image.x_pt + rect.x * image.width_pt,
          y_pt: image.y_pt + rect.y * image.height_pt,
          width_pt: rect.width * image.width_pt,
          height_pt: rect.height * image.height_pt,
          fill: Some(RectFill::Solid {
            color: RgbColor {
              r: rect.color[0],
              g: rect.color[1],
              b: rect.color[2],
            },
            opacity: 1.0,
          }),
          stroke: None,
          stroke_opacity: 1.0,
        })
      })
      .collect::<Vec<_>>();
      let bitmap_layers = if include_raster_backdrop_text {
        ooxmlsdk_layout::render::emf_wmf::extract_metafile_bitmap_layers_with_options(
          &image.data,
          image.content_type.as_deref(),
          extraction_options,
        )
      } else {
        Vec::new()
      }
      .into_iter()
      .map(|layer| {
        // The PowerPoint 365 golden stores every ActiveX preview DIB color
        // plane as a quality-75 JPEG, including layers whose binary WMF
        // mask becomes a PDF SMask. Preserve that mask while matching the
        // fixed-output color samples; ordinary presentation blips do not
        // enter this ActiveX-only expansion path.
        let data =
          super::image::powerpoint_activex_bitmap_png(&layer.data, 75).unwrap_or(layer.data);
        PageItem::Image(ImageItem {
          x_pt: image.x_pt + layer.x * image.width_pt,
          y_pt: image.y_pt + layer.y * image.height_pt,
          width_pt: layer.width * image.width_pt,
          height_pt: layer.height * image.height_pt,
          crop: ImageCrop::default(),
          clip_path: &[],
          rotation_deg: 0.0,
          flip_horizontal: layer.flip_horizontal,
          flip_vertical: layer.flip_vertical,
          data: Cow::Owned(data),
          content_type: Some(Cow::Borrowed(layer.content_type)),
          blip_compression_state: common::BlipCompressionState::Unspecified,
          metafile_monochrome_dib_palette_override: None,
          metafile_background_color: None,
          metafile_external_header: None,
          metafile_fixed_output_profile: common::MetafileFixedOutputProfile::Default,
          alt_text: None,
          hyperlink_url: None,
          semantic_metafile_text: false,
          metafile_semantic_text_includes_raster_backdrop: false,
          signature_line: None,
          metafile_native_size: false,
        })
      })
      .collect::<Vec<_>>();
      let semantic_runs = ooxmlsdk_layout::render::emf_wmf::extract_metafile_text_runs_with_options(
        &image.data,
        image.content_type.as_deref(),
        include_raster_backdrop_text,
        extraction_options,
      );
      let paint_native_text = include_raster_backdrop_text;
      let semantic_runs = semantic_runs
        .into_iter()
        .map(|mut run| {
          run.font_family = localized_metafile_ui_font_family(
            run.font_family,
            ui_language,
            localize_signature_ui_text,
          );
          let font_size_pt = run
            .font_size
            .map(|size| size * image.height_pt)
            .unwrap_or(11.0)
            .max(1.0);
          PageItem::Text(Box::new(TextItem {
            x_pt: image.x_pt + run.x * image.width_pt,
            y_pt: image.y_pt + run.y * image.height_pt,
            line_height_pt: (font_size_pt * 1.15).max(1.0),
            line_metrics_participant: true,
            paint_clip: None,
            page_culling_bounds: None,
            text: Cow::Owned(run.text),
            style: TextStyle {
              font_family: run.font_family.map(Cow::Owned),
              font_size_pt,
              bold: run.bold,
              italic: run.italic,
              semantic_only: !paint_native_text,
              metafile_reference_baseline: true,
              rotation_deg: run.rotation_degrees,
              opacity: 1.0,
              semantic_character_advances_pt: run.advances.map(|advances| {
                advances
                  .into_iter()
                  .map(|advance| advance * image.width_pt)
                  .collect()
              }),
              ..TextStyle::default()
            },
            rotation_center_pt: None,
            hyperlink_url: None,
            dynamic_field: None,
            form_widget_id: None,
            paragraph_bidi: false,
            word_spacing_pt: 0.0,
            preserve_text_portion: false,
            decoration_span_start_x_pt: None,
            pdf_text_segmentation: common::PdfTextSegmentation::Line,
            source_path: None,
            semantic_target_width_pt: run.width.map(|width| width * image.width_pt),
          }))
        })
        .collect::<Vec<_>>();
      if semantic_runs.is_empty() && solid_rects.is_empty() && bitmap_layers.is_empty() {
        return PageItem::Image(image);
      }

      let flatten_native_emf_text = paint_native_text
        && image.signature_line.is_some()
        && image.content_type.as_deref().is_some_and(|content_type| {
          matches!(
            content_type.to_ascii_lowercase().as_str(),
            "image/emf" | "image/x-emf" | "application/emf" | "application/x-emf"
          )
        });
      let mut items =
        Vec::with_capacity(solid_rects.len() + bitmap_layers.len() + semantic_runs.len() + 1);
      items.extend(solid_rects);
      items.extend(bitmap_layers);
      items.push(PageItem::Image(image));
      items.extend(semantic_runs);
      // Ordinary semantic-only previews and PowerPoint's native WMF controls
      // remain Form XObjects. Word's native unsigned signature-line EMF is a
      // counterexample: fixed output writes its Arial/localized UI labels in
      // the page stream, which also makes their real text styles observable.
      PageItem::Group {
        mask: None,
        clip: None,
        transform: None,
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        flatten_identity: flatten_native_emf_text,
        inherit_text_line_owner: true,
        items,
      }
    }
    item => item,
  }
}

fn localized_metafile_ui_font_family(
  font_family: Option<String>,
  ui_language: Option<&str>,
  localized_ui_text: bool,
) -> Option<String> {
  let mut font_family = font_family?;
  let simplified_chinese_ui = localized_ui_text
    && ooxmlsdk_layout::localization::canonical_office_locale_tag(ui_language)
      .is_some_and(|locale| locale.eq_ignore_ascii_case("zh-CN") || locale.starts_with("zh-Hans"));
  // Microsoft's localized Windows UI guidance replaces Segoe UI with the
  // locale's UI face; the fixed-output zh-CN signature-line preview uses
  // Microsoft YaHei UI even for the Latin signer/title stored in the EMF.
  if simplified_chinese_ui && font_family.eq_ignore_ascii_case("Segoe UI") {
    font_family = "Microsoft YaHei UI".to_string();
  }
  Some(font_family)
}

fn word_unsigned_signature_line_items<'doc>(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  properties: &common::SignatureLineProperties<'_>,
  preview_runs: &[ooxmlsdk_layout::render::emf_wmf::MetafileTextRun],
  ui_language: Option<&str>,
) -> Vec<PageItem<'doc>> {
  // Word does not replay the stored unsigned preview verbatim when producing
  // fixed output. Office asks the host for a rendered-page EMF, and the
  // signature provider API separately exposes generated signature-line
  // images. The normalized positions below are the host-rendered 192 x 96 pt
  // unsigned Office signature line: a white field, a 0.75 pt rule, a 12 pt X,
  // and two 9 pt metadata rows. The values scale with the authored VML shape,
  // rather than with the fallback EMF's pixel LOGFONT.
  const REFERENCE_WIDTH_PT: f32 = 192.0;
  const REFERENCE_HEIGHT_PT: f32 = 96.0;
  const RULE_TOP_PT: f32 = 48.32;
  const RULE_HEIGHT_PT: f32 = 0.75;
  const X_OFFSET_PT: f32 = 7.10;
  const X_BASELINE_PT: f32 = 45.70;
  const X_FONT_SIZE_PT: f32 = 12.0;
  const LABEL_OFFSET_PT: f32 = 13.85;
  const SIGNER_BASELINE_PT: f32 = 61.42;
  const TITLE_BASELINE_PT: f32 = 76.04;
  const LABEL_FONT_SIZE_PT: f32 = 9.0;

  let scale_x = width_pt / REFERENCE_WIDTH_PT;
  let scale_y = height_pt / REFERENCE_HEIGHT_PT;
  let mut items = Vec::with_capacity(5);
  items.push(PageItem::Rect(RectItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    fill: Some(RectFill::Solid {
      color: RgbColor {
        r: 255,
        g: 255,
        b: 255,
      },
      opacity: 1.0,
    }),
    stroke: None,
    stroke_opacity: 1.0,
  }));
  items.push(PageItem::Rect(RectItem {
    x_pt,
    y_pt: y_pt + RULE_TOP_PT * scale_y,
    width_pt,
    height_pt: RULE_HEIGHT_PT * scale_y,
    fill: Some(RectFill::Solid {
      color: RgbColor { r: 0, g: 0, b: 0 },
      opacity: 1.0,
    }),
    stroke: None,
    stroke_opacity: 1.0,
  }));

  let x_preview = preview_runs
    .iter()
    .find(|run| run.text.trim().eq_ignore_ascii_case("x"))
    .or_else(|| preview_runs.first());
  let label_preview = preview_runs
    .iter()
    .find(|run| !run.text.trim().eq_ignore_ascii_case("x"));
  let x_font_family = x_preview
    .and_then(|run| run.font_family.clone())
    .or_else(|| Some("Arial".to_string()));
  let label_font_family = localized_metafile_ui_font_family(
    label_preview
      .and_then(|run| run.font_family.clone())
      .or_else(|| Some("Segoe UI".to_string())),
    ui_language,
    true,
  );
  items.push(word_signature_line_text_item(
    x_pt + X_OFFSET_PT * scale_x,
    y_pt + X_BASELINE_PT * scale_y,
    X_FONT_SIZE_PT * scale_y,
    "X".to_string(),
    x_font_family,
    x_preview.is_some_and(|run| run.bold),
    x_preview.is_some_and(|run| run.italic),
  ));
  if let Some(signer) = properties
    .suggested_signer
    .as_deref()
    .filter(|value| !value.is_empty())
  {
    items.push(word_signature_line_text_item(
      x_pt + LABEL_OFFSET_PT * scale_x,
      y_pt + SIGNER_BASELINE_PT * scale_y,
      LABEL_FONT_SIZE_PT * scale_y,
      signer.to_string(),
      label_font_family.clone(),
      label_preview.is_some_and(|run| run.bold),
      label_preview.is_some_and(|run| run.italic),
    ));
  }
  if let Some(title) = properties
    .suggested_signer_title
    .as_deref()
    .filter(|value| !value.is_empty())
  {
    items.push(word_signature_line_text_item(
      x_pt + LABEL_OFFSET_PT * scale_x,
      y_pt + TITLE_BASELINE_PT * scale_y,
      LABEL_FONT_SIZE_PT * scale_y,
      title.to_string(),
      label_font_family,
      label_preview.is_some_and(|run| run.bold),
      label_preview.is_some_and(|run| run.italic),
    ));
  }
  items
}

fn word_signature_line_text_item<'doc>(
  x_pt: f32,
  baseline_y_pt: f32,
  font_size_pt: f32,
  text: String,
  font_family: Option<String>,
  bold: bool,
  italic: bool,
) -> PageItem<'doc> {
  PageItem::Text(Box::new(TextItem {
    x_pt,
    y_pt: baseline_y_pt,
    line_height_pt: font_size_pt * 1.15,
    line_metrics_participant: true,
    paint_clip: None,
    page_culling_bounds: None,
    text: Cow::Owned(text),
    style: TextStyle {
      font_family: font_family.map(Cow::Owned),
      font_size_pt,
      bold,
      italic,
      metafile_reference_baseline: true,
      opacity: 1.0,
      ..TextStyle::default()
    },
    rotation_center_pt: None,
    hyperlink_url: None,
    dynamic_field: None,
    form_widget_id: None,
    paragraph_bidi: false,
    word_spacing_pt: 0.0,
    preserve_text_portion: false,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: common::PdfTextSegmentation::Line,
    source_path: None,
    semantic_target_width_pt: None,
  }))
}

fn page_item_from_common<'doc>(
  item: &'doc common::DisplayItem<'static>,
  ui_language: Option<&str>,
  text_metrics: &mut TextMetrics,
  separate_path_fill_and_stroke: bool,
) -> Option<PageItem<'doc>> {
  match item {
    common::DisplayItem::Text(text) => Some(PageItem::Text(Box::new(text_item_from_common(text)))),
    common::DisplayItem::Image(image) => Some(image_page_item_from_common(
      image,
      ui_language,
      text_metrics,
    )),
    common::DisplayItem::Group(group) => Some(PageItem::Group {
      mask: group.mask.as_ref().map(image_item_from_common),
      clip: group.clip.map(paint_clip_from_common),
      transform: group.transform,
      blend_mode: group.blend_mode,
      opacity: group.opacity,
      flatten_identity: group.flatten_identity,
      inherit_text_line_owner: group.inherit_text_line_owner,
      items: group
        .items
        .iter()
        .filter_map(|item| {
          page_item_from_common(
            item,
            ui_language,
            text_metrics,
            separate_path_fill_and_stroke,
          )
        })
        .collect(),
    }),
    common::DisplayItem::Path(path) => Some(PageItem::Polyline(polyline_from_common(
      path,
      separate_path_fill_and_stroke,
    ))),
    common::DisplayItem::Rect(rect) => Some(PageItem::Rect(rect_item_from_common(rect))),
    common::DisplayItem::Line(line) => Some(PageItem::Line(line_item_from_common(line))),
    common::DisplayItem::LinkArea(link) => Some(PageItem::LinkArea(link_area_from_common(link))),
    common::DisplayItem::Glyphs(_)
    | common::DisplayItem::AnnotationHint(_)
    | common::DisplayItem::Clip(_)
    | common::DisplayItem::Transform(_) => None,
  }
}

fn image_page_item_from_common<'doc>(
  image: &'doc common::ImageItem<'static>,
  ui_language: Option<&str>,
  text_metrics: &mut TextMetrics,
) -> PageItem<'doc> {
  let image = image_item_from_common(image);
  if !image.data.is_empty() {
    return PageItem::Image(image);
  }

  let style = missing_linked_image_text_style(ui_language);
  let text = missing_linked_image_text(ui_language);
  let text_x_pt = image.x_pt + 3.5;
  let text_y_pt = image.y_pt + 0.84;
  let max_width_pt = (image.width_pt - 5.0).max(0.0);
  let clip = Some(PaintClipRect {
    x_pt: image.x_pt,
    y_pt: image.y_pt,
    width_pt: image.width_pt,
    height_pt: image.height_pt,
  });
  let mut items = Vec::with_capacity(3);
  items.push(PageItem::Image(image));
  items.extend(
    wrap_missing_linked_image_text(text, &style, max_width_pt, text_metrics)
      .into_iter()
      .enumerate()
      .map(|(line_index, text)| {
        PageItem::Text(Box::new(TextItem {
          x_pt: text_x_pt,
          y_pt: text_y_pt + line_index as f32 * 1.2,
          line_height_pt: 1.2,
          line_metrics_participant: true,
          paint_clip: clip,
          page_culling_bounds: None,
          text: Cow::Owned(text),
          style: style.clone(),
          rotation_center_pt: None,
          hyperlink_url: None,
          dynamic_field: None,
          form_widget_id: None,
          paragraph_bidi: false,
          word_spacing_pt: 0.0,
          preserve_text_portion: false,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: common::PdfTextSegmentation::default(),
          source_path: None,
          semantic_target_width_pt: None,
        }))
      }),
  );
  PageItem::Group {
    mask: None,
    clip: None,
    transform: None,
    blend_mode: common::BlendMode::Normal,
    opacity: 1.0,
    flatten_identity: false,
    inherit_text_line_owner: true,
    items,
  }
}

fn missing_linked_image_text(ui_language: Option<&str>) -> &'static str {
  ooxmlsdk_layout::localization::office_missing_linked_image_resource(ui_language).text
}

fn missing_linked_image_text_style(ui_language: Option<&str>) -> TextStyle<'static> {
  let font_family =
    ooxmlsdk_layout::localization::office_missing_linked_image_resource(ui_language).font_family;
  TextStyle {
    font_family: Some(Cow::Borrowed(font_family)),
    east_asia_font_family: Some(Cow::Borrowed(font_family)),
    font_size_pt: 1.32,
    line_vertical_alignment: common::LineVerticalAlignment::Top,
    use_windows_font_metrics: true,
    wordprocessingml_font_slots: true,
    pdf_glyph_outlines: true,
    color: RgbColor { r: 0, g: 0, b: 0 },
    opacity: 1.0,
    outline_opacity: 1.0,
    ..TextStyle::default()
  }
}

fn wrap_missing_linked_image_text(
  text: &str,
  style: &TextStyle<'_>,
  max_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Vec<String> {
  if max_width_pt <= f32::EPSILON {
    return Vec::new();
  }
  let mut lines = Vec::new();
  let mut line = String::new();
  for character in text.chars() {
    line.push(character);
    if line.chars().count() > 1 && text_metrics.measure_text(&line, style) > max_width_pt {
      line.pop();
      lines.push(std::mem::take(&mut line));
      line.push(character);
    }
  }
  if !line.is_empty() {
    lines.push(line);
  }
  lines
}

fn paint_group_item<'doc>(
  item: PageItem<'doc>,
  owner: Option<PaintLineOwner>,
  common_line_baseline: Option<f32>,
  page_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> PaintItem<'doc> {
  match item {
    PageItem::Text(text) => PaintItem::Text(Box::new(PaintText::from_layout_text(
      *text,
      owner,
      common_line_baseline,
      page_width_pt,
      text_metrics,
    ))),
    PageItem::Image(image) => PaintItem::Image(image),
    PageItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity,
      inherit_text_line_owner,
      items,
    } => PaintItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity,
      items: {
        let child_owner = inherit_text_line_owner.then_some(owner).flatten();
        let child_line_baseline = inherit_text_line_owner
          .then_some(common_line_baseline)
          .flatten();
        items
          .into_iter()
          .map(|item| {
            paint_group_item(
              item,
              child_owner,
              child_line_baseline,
              page_width_pt,
              text_metrics,
            )
          })
          .collect()
      },
    },
    PageItem::LinkArea(link_area) => PaintItem::LinkArea(link_area),
    PageItem::Rect(rect) => PaintItem::Rect(rect),
    PageItem::Line(line) => PaintItem::Line(line),
    PageItem::Polyline(polyline) => PaintItem::Polyline(polyline),
  }
}

fn text_item_from_common<'doc>(text: &'doc common::TextRun<'static>) -> TextItem<'doc> {
  TextItem {
    x_pt: text.origin.x.0,
    y_pt: text.origin.y.0,
    line_height_pt: text.line_height.0,
    line_metrics_participant: text.line_metrics_participant,
    paint_clip: text.paint_clip.map(paint_clip_from_common),
    page_culling_bounds: text.page_culling_bounds.map(paint_clip_from_common),
    text: Cow::Borrowed(text.text.as_ref()),
    style: text_style_from_common(&text.style),
    rotation_center_pt: text.rotation_center.map(|point| (point.x.0, point.y.0)),
    hyperlink_url: text
      .hyperlink_url
      .as_ref()
      .map(|url| Cow::Borrowed(url.as_ref())),
    dynamic_field: text
      .dynamic_field
      .as_ref()
      .map(dynamic_field_borrowed)
      .map(Box::new),
    form_widget_id: text.form_widget_id,
    paragraph_bidi: text.paragraph_bidi,
    word_spacing_pt: text.word_spacing_pt,
    preserve_text_portion: text.preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: text.pdf_text_segmentation,
    source_path: text.source.as_ref().map(|source| source.path.as_slice()),
    semantic_target_width_pt: None,
  }
}

fn image_item_from_common<'doc>(image: &'doc common::ImageItem<'static>) -> ImageItem<'doc> {
  ImageItem {
    x_pt: image.bounds.origin.x.0,
    y_pt: image.bounds.origin.y.0,
    width_pt: image.bounds.size.width.0,
    height_pt: image.bounds.size.height.0,
    crop: image.crop.unwrap_or_default().into(),
    clip_path: &image.clip_path,
    rotation_deg: image.rotation_degrees,
    flip_horizontal: image.flip_horizontal,
    flip_vertical: image.flip_vertical,
    data: Cow::Borrowed(image.bytes.as_ref()),
    content_type: Some(Cow::Borrowed(image.content_type.as_ref())),
    blip_compression_state: image.blip_compression_state,
    metafile_monochrome_dib_palette_override: image.metafile_monochrome_dib_palette_override,
    metafile_background_color: image.metafile_background_color,
    metafile_external_header: image.metafile_external_header,
    metafile_fixed_output_profile: image.metafile_fixed_output_profile,
    alt_text: image
      .alt_text
      .as_ref()
      .map(|text| Cow::Borrowed(text.as_ref())),
    hyperlink_url: image
      .hyperlink_url
      .as_ref()
      .map(|url| Cow::Borrowed(url.as_ref())),
    semantic_metafile_text: image.semantic_metafile_text,
    metafile_semantic_text_includes_raster_backdrop: image
      .metafile_semantic_text_includes_raster_backdrop,
    signature_line: image
      .signature_line
      .as_ref()
      .map(signature_line_properties_from_common),
    metafile_native_size: image.metafile_native_size,
  }
}

fn signature_line_properties_from_common<'doc>(
  properties: &'doc common::SignatureLineProperties<'static>,
) -> common::SignatureLineProperties<'doc> {
  common::SignatureLineProperties {
    state: properties.state,
    id: properties
      .id
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    provider_id: properties
      .provider_id
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    signing_instructions_set: properties.signing_instructions_set,
    allow_comments: properties.allow_comments,
    show_sign_date: properties.show_sign_date,
    suggested_signer: properties
      .suggested_signer
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    suggested_signer_title: properties
      .suggested_signer_title
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    suggested_signer_email: properties
      .suggested_signer_email
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    signing_instructions: properties
      .signing_instructions
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    additional_xml: properties
      .additional_xml
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    signature_provider_url: properties
      .signature_provider_url
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
  }
}

fn polyline_from_common<'doc>(
  path: &'doc common::PathItem<'static>,
  separate_fill_and_stroke: bool,
) -> PolylineItem<'doc> {
  let x_pt = path.bounds.origin.x.0;
  let y_pt = path.bounds.origin.y.0;
  PolylineItem {
    x_pt,
    y_pt,
    width_pt: path.bounds.size.width.0,
    height_pt: path.bounds.size.height.0,
    points: &path.points,
    commands: &path.commands,
    closed: path.closed,
    fill: &path.fill,
    stroke: path.stroke.as_ref(),
    separate_fill_and_stroke,
  }
}

fn rect_item_from_common<'doc>(rect: &'doc common::RectItem<'static>) -> RectItem<'doc> {
  let fill = match &rect.fill {
    common::Fill::Solid(color) => Some(RectFill::Solid {
      color: rgb(*color),
      opacity: opacity(*color),
    }),
    common::Fill::Gradient(gradient) => Some(RectFill::Gradient(gradient)),
    common::Fill::Pattern(pattern) => Some(RectFill::Pattern(pattern)),
    common::Fill::None | common::Fill::Theme(_) | common::Fill::Image { .. } => None,
  };
  RectItem {
    x_pt: rect.bounds.origin.x.0,
    y_pt: rect.bounds.origin.y.0,
    width_pt: rect.bounds.size.width.0,
    height_pt: rect.bounds.size.height.0,
    fill,
    stroke: rect.stroke.as_ref().map(stroke_from_common),
    stroke_opacity: rect
      .stroke
      .as_ref()
      .map_or(1.0, |stroke| opacity(stroke.color)),
  }
}

fn line_item_from_common(line: &common::LineItem<'static>) -> LineItem {
  let line_cap = match line.stroke.cap {
    Some(common::StrokeCap::Round) => PaintLineCap::Round,
    Some(common::StrokeCap::Square) => PaintLineCap::Square,
    Some(common::StrokeCap::Flat) | None => PaintLineCap::Butt,
  };
  LineItem {
    x1_pt: line.start.x.0,
    y1_pt: line.start.y.0,
    x2_pt: line.end.x.0,
    y2_pt: line.end.y.0,
    width_pt: line.stroke.width.0,
    color: rgb(line.stroke.color),
    opacity: opacity(line.stroke.color),
    dash: line
      .stroke
      .resolved_dash()
      .map(|values| values.into_iter().map(|value| value.0).collect()),
    dash_offset: line.stroke.dash_offset.0,
    line_cap,
    kind: match line.kind {
      common::LineKind::Stroke => LineItemKind::Stroke,
      common::LineKind::FilledRect => LineItemKind::FilledRect,
    },
  }
}

fn link_area_from_common<'doc>(link: &'doc common::LinkArea<'static>) -> LinkAreaItem<'doc> {
  LinkAreaItem {
    x_pt: link.bounds.origin.x.0,
    y_pt: link.bounds.origin.y.0,
    width_pt: link.bounds.size.width.0,
    height_pt: link.bounds.size.height.0,
    hyperlink_url: Cow::Borrowed(link.target.as_ref()),
  }
}

fn text_style_from_common<'doc>(style: &'doc common::TextStyle<'static>) -> TextStyle<'doc> {
  TextStyle {
    font_family: style
      .font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    high_ansi_font_family: style
      .high_ansi_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    fallback_font_family: style
      .fallback_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    high_ansi_fallback_font_family: style
      .high_ansi_fallback_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    east_asia_fallback_font_family: style
      .east_asia_fallback_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    complex_fallback_font_family: style
      .complex_fallback_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    font_family_class: style.font_family_class,
    high_ansi_font_family_class: style.high_ansi_font_family_class,
    east_asia_font_family_class: style.east_asia_font_family_class,
    complex_font_family_class: style.complex_font_family_class,
    east_asia_font_family: style
      .east_asia_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    complex_font_family: style
      .complex_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    symbol_font_family: style
      .symbol_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    explicit_symbol_character: style.explicit_symbol_character,
    font_size_pt: style.font_size.0,
    complex_font_size_pt: style.complex_font_size.map(|size| size.0),
    layout_font_sizes: style.layout_font_sizes,
    complex_script: style.complex_script,
    right_to_left: style.right_to_left,
    resolved_bidi_level: style.resolved_bidi_level(),
    kerning_minimum_size_pt: style.kerning_minimum_size.map(|size| size.0),
    ligatures: style.ligatures,
    open_type_features: style.open_type_features,
    horizontal_scale: style.horizontal_scale,
    semantic_character_advances_pt: style.semantic_character_advances_pt.clone(),
    character_spacing_pt: style.character_spacing.0,
    baseline_shift_pt: style.baseline_shift.0,
    automatic_escapement_font_size_pt: style.automatic_escapement_font_size.map(|size| size.0),
    automatic_escapement_complex_font_size_pt: style
      .automatic_escapement_complex_font_size
      .map(|size| size.0),
    line_vertical_alignment: style.line_vertical_alignment,
    use_windows_font_metrics: style.use_windows_font_metrics,
    wordprocessingml_font_slots: style.wordprocessingml_font_slots,
    wordprocessingml_cjk_line_metrics: style.wordprocessingml_cjk_line_metrics,
    wordprocessingml_font_hint: style.wordprocessingml_font_hint,
    wordprocessingml_east_asia_language_is_chinese: style
      .wordprocessingml_east_asia_language_is_chinese,
    font_charset: style.font_charset,
    high_ansi_font_charset: style.high_ansi_font_charset,
    wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset,
    complex_font_charset: style.complex_font_charset,
    font_pitch: style.font_pitch,
    high_ansi_font_pitch: style.high_ansi_font_pitch,
    east_asia_font_pitch: style.east_asia_font_pitch,
    complex_font_pitch: style.complex_font_pitch,
    cjk_punctuation_compression_ratio: style.cjk_punctuation_compression_ratio,
    wordprocessingml_balance_single_byte_double_byte_width: style
      .wordprocessingml_balance_single_byte_double_byte_width,
    pdf_glyph_outlines: style.pdf_glyph_outlines,
    pdf_glyph_outline_options: style.pdf_glyph_outline_options.as_deref().cloned(),
    bold: style.bold,
    italic: style.italic,
    complex_bold: style.complex_bold,
    complex_italic: style.complex_italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
    semantic_only: style.semantic_only,
    metafile_reference_baseline: false,
    rotation_deg: style.rotation_degrees,
    color: rgb(style.color),
    opacity: opacity(style.color),
    outline_color: style.outline_color.map(rgb),
    outline_opacity: style.outline_color.map_or(1.0, opacity),
    outline_width_pt: style.outline_width.0,
    highlight: style.highlight.map(rgb),
    underline_color: style.underline_color.map(rgb),
  }
}

fn dynamic_field_borrowed<'doc>(
  field: &'doc common::DynamicField<'static>,
) -> common::DynamicField<'doc> {
  match field {
    common::DynamicField::Page { number_format } => common::DynamicField::Page {
      number_format: *number_format,
    },
    common::DynamicField::NumPages { number_format } => common::DynamicField::NumPages {
      number_format: *number_format,
    },
    common::DynamicField::Sequence {
      identifier,
      number_format,
    } => common::DynamicField::Sequence {
      identifier: Cow::Borrowed(identifier.as_ref()),
      number_format: *number_format,
    },
    common::DynamicField::PageRef { bookmark_name } => common::DynamicField::PageRef {
      bookmark_name: Cow::Borrowed(bookmark_name.as_ref()),
    },
    common::DynamicField::StyleRef {
      style_name,
      from_bottom,
    } => common::DynamicField::StyleRef {
      style_name: Cow::Borrowed(style_name.as_ref()),
      from_bottom: *from_bottom,
    },
  }
}

impl From<common::ImageCrop> for ImageCrop {
  fn from(crop: common::ImageCrop) -> Self {
    Self {
      left: crop.left,
      top: crop.top,
      right: crop.right,
      bottom: crop.bottom,
    }
  }
}

fn frame_kind_name_from_common(kind: &str) -> FollowFrameKind {
  match kind {
    "table" => FollowFrameKind::Table,
    "notes" => FollowFrameKind::Notes,
    _ => FollowFrameKind::Paragraph,
  }
}

fn stroke_from_common(stroke: &common::Stroke<'static>) -> BorderStyle {
  BorderStyle {
    width_pt: stroke.width.0,
    color: rgb(stroke.color),
  }
}

fn rgb(color: common::Color) -> RgbColor {
  RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  }
}

fn opacity(color: common::Color) -> f32 {
  f32::from(color.a) / 255.0
}

fn coalesced_writer_text_items<'doc>(
  items: impl IntoIterator<Item = (PageItem<'doc>, Option<PaintLineOwner>)>,
  text_metrics: &mut TextMetrics,
) -> (Vec<PageItem<'doc>>, Vec<Option<PaintLineOwner>>) {
  let items = items.into_iter();
  let mut output: Vec<PageItem<'doc>> = Vec::with_capacity(items.size_hint().0);
  let mut owners = Vec::with_capacity(items.size_hint().0);
  for (item, owner) in items {
    match item {
      PageItem::Text(text) => {
        if let Some(PageItem::Text(previous)) = output.last_mut()
          && same_paint_line_owner(owners.last().copied().flatten(), owner)
          && writer_text_items_coalesce(previous, &text, text_metrics)
        {
          previous.text.to_mut().push_str(&text.text);
          previous.line_height_pt = previous.line_height_pt.max(text.line_height_pt);
          previous.line_metrics_participant |= text.line_metrics_participant;
          continue;
        }
        output.push(PageItem::Text(text));
      }
      PageItem::Image(image) => output.push(PageItem::Image(image)),
      PageItem::Group {
        mask,
        clip,
        transform,
        blend_mode,
        opacity,
        flatten_identity,
        inherit_text_line_owner,
        items,
      } => {
        output.push(PageItem::Group {
          mask,
          clip,
          transform,
          blend_mode,
          opacity,
          flatten_identity,
          inherit_text_line_owner,
          items,
        });
      }
      PageItem::LinkArea(link_area) => output.push(PageItem::LinkArea(link_area)),
      PageItem::Rect(rect) => output.push(PageItem::Rect(rect)),
      PageItem::Line(line) => output.push(PageItem::Line(line)),
      PageItem::Polyline(polyline) => output.push(PageItem::Polyline(polyline)),
    }
    owners.push(owner);
  }
  (output, owners)
}

fn writer_text_items_coalesce(
  current: &TextItem<'_>,
  next: &TextItem<'_>,
  text_metrics: &mut TextMetrics,
) -> bool {
  if current.pdf_text_segmentation != next.pdf_text_segmentation
    || current.form_widget_id.is_some()
    || next.form_widget_id.is_some()
    || current.preserve_text_portion
    || next.preserve_text_portion
    || current.style.small_caps
    || next.style.small_caps
  {
    return false;
  }
  if current.pdf_text_segmentation == common::PdfTextSegmentation::Portion
    && (current.text.contains('\t') || next.text.contains('\t'))
  {
    return false;
  }
  if current.style != next.style
    || current.hyperlink_url != next.hyperlink_url
    || current.dynamic_field != next.dynamic_field
    || current.paragraph_bidi != next.paragraph_bidi
    || (current.word_spacing_pt - next.word_spacing_pt).abs() >= 0.001
    || current.rotation_center_pt != next.rotation_center_pt
    || current.source_path != next.source_path
    || current.decoration_span_start_x_pt != next.decoration_span_start_x_pt
    || (current.y_pt - next.y_pt).abs() >= 0.01
    || (current.line_height_pt - next.line_height_pt).abs() >= 0.01
  {
    return false;
  }
  let current_right = current.x_pt
    + text_metrics.measure_text(&current.text, &current.style)
    + current.text.matches(' ').count() as f32 * current.word_spacing_pt;
  (current_right - next.x_pt).abs() < 0.25
}

fn common_writer_line_baselines(
  items: &[PageItem<'_>],
  owners: &[Option<PaintLineOwner>],
  text_metrics: &mut TextMetrics,
) -> Vec<Option<f32>> {
  let mut line_metrics = HashMap::<(usize, usize), WriterLineMetrics>::default();
  for (item, owner) in items.iter().zip(owners) {
    let Some(owner) = owner else {
      continue;
    };
    if !matches!(
      owner.frame_kind,
      FollowFrameKind::Paragraph | FollowFrameKind::Notes
    ) {
      continue;
    }
    let Some(metrics) = writer_item_line_metrics(item, text_metrics) else {
      continue;
    };
    line_metrics
      .entry((owner.frame_index, owner.line_index))
      .and_modify(|line| line.include(metrics))
      .or_insert(metrics);
  }
  owners
    .iter()
    .map(|owner| {
      let owner = owner.as_ref()?;
      line_metrics
        .get(&(owner.frame_index, owner.line_index))
        .map(|metrics| metrics.baseline_offset_pt())
    })
    .collect()
}

#[derive(Clone, Copy, Debug, Default)]
struct WriterLineMetrics {
  ascent_pt: f32,
  descent_pt: f32,
  resolved_height_pt: f32,
  top_aligned: bool,
}

impl WriterLineMetrics {
  fn include(&mut self, other: Self) {
    self.ascent_pt = self.ascent_pt.max(other.ascent_pt);
    self.descent_pt = self.descent_pt.max(other.descent_pt);
    self.resolved_height_pt = self.resolved_height_pt.max(other.resolved_height_pt);
    self.top_aligned &= other.top_aligned;
  }

  fn baseline_offset_pt(self) -> f32 {
    let content_height_pt = self.ascent_pt + self.descent_pt;
    let extra_leading_pt = (self.resolved_height_pt - content_height_pt).max(0.0);
    self.ascent_pt
      + if self.top_aligned {
        0.0
      } else {
        extra_leading_pt / 2.0
      }
  }
}

fn writer_item_line_metrics(
  item: &PageItem<'_>,
  text_metrics: &mut TextMetrics,
) -> Option<WriterLineMetrics> {
  match item {
    PageItem::Text(text)
      if text.line_metrics_participant
        && !text.style.semantic_only
        && matches!(
          text.style.line_vertical_alignment,
          common::LineVerticalAlignment::Auto | common::LineVerticalAlignment::Baseline
        ) =>
    {
      let normalized_automatic_escapement = text.style.automatic_escapement_font_size_pt.is_some()
        && text.style.baseline_shift_pt.abs() <= f32::EPSILON;
      let metrics = if normalized_automatic_escapement {
        text_metrics.vertical_metrics_for_text(&text.text, &text.style)
      } else {
        text_metrics.line_vertical_metrics_for_text(&text.text, &text.style)
      };
      let default_baseline_pt = if text.style.use_windows_font_metrics {
        metrics.directwrite_baseline_offset_pt
      } else {
        metrics.leading_above_pt() + metrics.ascent_pt
      };
      let line_shift_pt = if text.style.automatic_escapement_font_size_pt.is_some() {
        0.0
      } else {
        text.style.baseline_shift_pt
      };
      Some(WriterLineMetrics {
        ascent_pt: (default_baseline_pt + line_shift_pt).max(0.0),
        descent_pt: (metrics.line_height_pt() - default_baseline_pt - line_shift_pt).max(0.0),
        resolved_height_pt: text.line_height_pt,
        top_aligned: normalized_automatic_escapement,
      })
    }
    PageItem::Group {
      inherit_text_line_owner: true,
      items,
      ..
    } => items.iter().fold(None, |line, item| {
      let Some(metrics) = writer_item_line_metrics(item, text_metrics) else {
        return line;
      };
      Some(line.map_or(metrics, |mut line: WriterLineMetrics| {
        line.include(metrics);
        line
      }))
    }),
    PageItem::Group { .. } => None,
    PageItem::Text(_)
    | PageItem::Image(_)
    | PageItem::LinkArea(_)
    | PageItem::Rect(_)
    | PageItem::Line(_)
    | PageItem::Polyline(_) => None,
  }
}

impl<'doc> PaintText<'doc> {
  fn from_layout_text(
    mut text: TextItem<'doc>,
    owner: Option<PaintLineOwner>,
    common_line_baseline: Option<f32>,
    page_width_pt: f32,
    text_metrics: &mut TextMetrics,
  ) -> Self {
    let paint_clip = intersect_paint_clips(owner.and_then(|owner| owner.clip), text.paint_clip);
    if let Some(target_width_pt) = text.semantic_target_width_pt {
      let measured_width_pt = text_metrics.measure_text(&text.text, &text.style);
      if measured_width_pt > f32::EPSILON {
        text.style.horizontal_scale = Some((target_width_pt / measured_width_pt).max(0.01));
      }
    }
    let text_ref = &text;
    let mut glyphs = shaped_pdf_glyphs(
      &text_ref.text,
      &text_ref.style,
      text_ref.word_spacing_pt,
      text_metrics,
    );
    let has_text_warp = text_ref
      .style
      .pdf_glyph_outline_options
      .as_ref()
      .is_some_and(|options| options.text_warp.is_some());
    let layout_glyphs = (has_text_warp || text_ref.style.highlight.is_some())
      .then(|| {
        text_layout_glyphs(
          &text_ref.text,
          &text_ref.style,
          text_ref.word_spacing_pt,
          text_metrics,
        )
      })
      .flatten();
    if has_text_warp && let (Some(paint), Some(layout)) = (glyphs.as_mut(), layout_glyphs.as_ref())
    {
      retain_layout_glyph_positions(paint, layout);
    }
    let width_pt = glyphs
      .as_ref()
      .map(|run| run.width_pt)
      .unwrap_or_else(|| text_metrics.measure_text(&text_ref.text, &text_ref.style));
    let vertical_metrics = text_metrics.vertical_metrics_for_text(&text_ref.text, &text_ref.style);
    let baseline_y = if text_ref.style.semantic_only || text_ref.style.metafile_reference_baseline {
      // EMF text extraction reports the reference point consumed as the
      // baseline by emfsdk's raster replay. Preserve that exact coordinate
      // instead of applying the surrounding document line-box metrics.
      text_ref.y_pt
    } else {
      match owner.map(|owner| owner.frame_kind) {
        Some(FollowFrameKind::Table) => text_ref.y_pt - text_ref.style.baseline_shift_pt,
        Some(FollowFrameKind::Paragraph | FollowFrameKind::Notes) | None => {
          let mut centered_offset = || {
            if text_ref.style.use_windows_font_metrics {
              text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
                &text_ref.text,
                &text_ref.style,
                text_ref.line_height_pt,
              )
            } else {
              text_metrics.baseline_offset_in_line_for_text(
                &text_ref.text,
                &text_ref.style,
                text_ref.line_height_pt,
              )
            }
          };
          let natural_baseline = if text_ref.style.use_windows_font_metrics
            && vertical_metrics.baseline_offset_pt > 0.0
          {
            vertical_metrics.baseline_offset_pt
          } else {
            vertical_metrics.leading_above_pt() + vertical_metrics.ascent_pt
          };
          let natural_height =
            vertical_metrics.line_height_pt() + text_ref.style.baseline_shift_pt.abs();
          let offset = match text_ref.style.line_vertical_alignment {
            common::LineVerticalAlignment::Auto | common::LineVerticalAlignment::Baseline => {
              common_line_baseline
                .map(|baseline| baseline - text_ref.style.baseline_shift_pt)
                .unwrap_or_else(centered_offset)
            }
            common::LineVerticalAlignment::Top => {
              natural_baseline - text_ref.style.baseline_shift_pt
            }
            common::LineVerticalAlignment::Center => centered_offset(),
            common::LineVerticalAlignment::Bottom => {
              (text_ref.line_height_pt - natural_height).max(0.0) + natural_baseline
                - text_ref.style.baseline_shift_pt
            }
          };
          text_ref.y_pt + offset
        }
      }
    };
    let text_box_y_pt =
      baseline_y - vertical_metrics.ascent_pt - vertical_metrics.leading_above_pt();
    let text_box_height_pt = vertical_metrics.line_height_pt();
    let highlight_glyphs = text_ref.style.highlight.and(layout_glyphs);
    let highlight = text_ref.style.highlight.map(|color| {
      let (top_pt, height_pt) = text_highlight_vertical_bounds(
        baseline_y,
        vertical_metrics,
        text_ref.style.use_windows_font_metrics,
      );
      PaintRect {
        x_pt: text_ref.x_pt,
        y_pt: top_pt,
        width_pt: highlight_glyphs
          .as_ref()
          .map_or(width_pt, |run| run.width_pt),
        height_pt,
        color,
      }
    });
    let decoration_metrics = text_metrics.text_decoration_metrics(&text_ref.style);
    let decoration_start_x_pt = text_ref.decoration_span_start_x_pt.unwrap_or(text_ref.x_pt);
    let underline_y_pt = baseline_y + decoration_metrics.underline_offset_pt;
    let underline = text_ref.style.underline.then_some(PaintStrokeLine {
      x1_pt: decoration_start_x_pt,
      y1_pt: underline_y_pt,
      x2_pt: text_ref.x_pt + width_pt,
      y2_pt: underline_y_pt,
      width_pt: decoration_metrics.underline_width_pt,
      color: text_ref
        .style
        .underline_color
        .unwrap_or(text_ref.style.color),
    });
    let strikethrough_y_pt = baseline_y - decoration_metrics.strikethrough_offset_pt;
    let strikethrough = text_ref.style.strikethrough.then_some(PaintStrokeLine {
      x1_pt: decoration_start_x_pt,
      y1_pt: strikethrough_y_pt,
      x2_pt: text_ref.x_pt + width_pt,
      y2_pt: strikethrough_y_pt,
      width_pt: decoration_metrics.strikethrough_width_pt,
      color: text_ref.style.color,
    });
    let link = text_ref.hyperlink_url.as_ref().map(|_| PaintLink {
      x_pt: text_ref.x_pt,
      y_pt: text_box_y_pt,
      width_pt,
      height_pt: text_box_height_pt,
    });

    let portions = text_paint_portions(
      PaintTextPortionSource {
        text: text_ref,
        baseline_y,
        width_pt,
        page_width_pt,
        clip: paint_clip,
        glyphs: glyphs.map(|run| run.font_runs),
        highlight_glyphs: highlight_glyphs.map(|run| run.font_runs),
        highlight,
        underline,
        strikethrough,
        link,
      },
      text_metrics,
    );

    Self {
      item: text,
      source_frame_index: owner.map(|owner| owner.frame_index),
      source_line_index: owner.map(|owner| owner.line_index),
      baseline_y,
      width_pt,
      portions,
    }
  }
}

struct PaintTextPortionSource<'a, 'doc> {
  text: &'a TextItem<'doc>,
  baseline_y: f32,
  width_pt: f32,
  page_width_pt: f32,
  clip: Option<PaintClipRect>,
  glyphs: Option<PaintGlyphFontRuns>,
  highlight_glyphs: Option<PaintGlyphFontRuns>,
  highlight: Option<PaintRect>,
  underline: Option<PaintStrokeLine>,
  strikethrough: Option<PaintStrokeLine>,
  link: Option<PaintLink>,
}

fn text_paint_portions<'doc>(
  source: PaintTextPortionSource<'_, 'doc>,
  text_metrics: &mut TextMetrics,
) -> Vec<PaintTextPortion> {
  let PaintTextPortionSource {
    text,
    baseline_y,
    width_pt,
    page_width_pt,
    clip,
    glyphs,
    highlight_glyphs,
    highlight,
    underline,
    strikethrough,
    link,
  } = source;
  let ranges = visually_ordered_text_portion_ranges(text);
  let horizontal_scale = text.style.horizontal_scale.unwrap_or(1.0).max(f32::EPSILON);
  let can_move_glyphs =
    glyphs.is_some() && ranges.len() == 1 && ranges[0].1 == (0..text.text.len());
  let mut glyphs = glyphs;
  let mut portions = Vec::with_capacity(ranges.len().max(1));
  let mut x_pt = text.x_pt;
  let mut highlight_x_pt = text.x_pt;
  for (kind, range) in ranges {
    let portion_clip = paint_clip_for_portion(clip, &kind, page_width_pt);
    let portion_glyphs = if can_move_glyphs {
      glyphs.take()
    } else {
      glyphs
        .as_ref()
        .map(|glyphs| glyphs_for_text_range(glyphs, &range))
    };
    let portion_width = portion_glyphs
      .as_ref()
      .map(|glyphs| glyph_runs_visible_width_pt(glyphs, horizontal_scale))
      .unwrap_or_else(|| {
        text_metrics.measure_text(&text.text[range.start..range.end], &text.style)
      });
    let highlight_width = highlight_glyphs.as_ref().map_or(portion_width, |glyphs| {
      if range == (0..text.text.len()) {
        glyph_runs_visible_width_pt(glyphs, horizontal_scale)
      } else {
        glyph_runs_visible_width_pt(&glyphs_for_text_range(glyphs, &range), horizontal_scale)
      }
    });
    portions.push(PaintTextPortion {
      kind,
      text_range: range,
      x_pt,
      baseline_y,
      width_pt: portion_width,
      clip: portion_clip,
      glyphs: portion_glyphs.filter(|glyphs| !glyphs.is_empty()),
      highlight: highlight
        .as_ref()
        .map(|rect| paint_rect_for_portion(rect, highlight_x_pt, highlight_width)),
      underline: underline
        .as_ref()
        .map(|line| paint_line_for_portion(line, x_pt, portion_width)),
      strikethrough: strikethrough
        .as_ref()
        .map(|line| paint_line_for_portion(line, x_pt, portion_width)),
      link: link
        .as_ref()
        .map(|link| paint_link_for_portion(link, x_pt, portion_width)),
    });
    x_pt += portion_width;
    highlight_x_pt += highlight_width;
  }
  if portions.is_empty() {
    let portion_clip = paint_clip_for_portion(clip, &PaintTextPortionKind::Text, page_width_pt);
    portions.push(PaintTextPortion {
      kind: PaintTextPortionKind::Text,
      text_range: 0..text.text.len(),
      x_pt: text.x_pt,
      baseline_y,
      width_pt,
      clip: portion_clip,
      glyphs,
      highlight,
      underline,
      strikethrough,
      link,
    });
  }
  portions
}

fn visually_ordered_text_portion_ranges(text: &TextItem<'_>) -> PaintTextPortionRanges {
  let mut ranges = text_portion_ranges(text);
  if text
    .style
    .resolved_bidi_level
    .is_some_and(|level| level % 2 == 1)
  {
    // The source ranges remain logical for tagging and ActualText, but their
    // paint origins must follow the visual order resolved by UAX #9 rule L2.
    // This matters when Office's WordLine segmentation isolates a hyphen
    // inside one otherwise directionally uniform RTL text item.
    ranges.reverse();
  }
  ranges
}

fn text_portion_ranges(text: &TextItem<'_>) -> PaintTextPortionRanges {
  if text.text.is_empty() {
    return PaintTextPortionRanges::new();
  }
  if let Some(ranges) = office_tab_leader_portion_ranges(text) {
    return ranges;
  }
  if text.dynamic_field.is_some() {
    let mut ranges = PaintTextPortionRanges::new();
    ranges.push((PaintTextPortionKind::Field, 0..text.text.len()));
    return ranges;
  }
  let decorated_edge_space = (text.style.underline || text.style.strikethrough)
    && (text.text.starts_with(char::is_whitespace) || text.text.ends_with(char::is_whitespace));
  let split_decorated_portions =
    text.preserve_text_portion && (text.style.underline || text.style.strikethrough);
  if decorated_edge_space
    && !text.text.contains('\t')
    && text.pdf_text_segmentation != common::PdfTextSegmentation::Portion
    && !split_decorated_portions
  {
    return edge_whitespace_text_portion_ranges(text);
  }
  let split_portions =
    text.pdf_text_segmentation == common::PdfTextSegmentation::Portion || split_decorated_portions;
  if text.pdf_text_segmentation == common::PdfTextSegmentation::Line
    && !split_decorated_portions
    && text.hyperlink_url.is_some()
    && !text.text.contains('\t')
  {
    let mut ranges = PaintTextPortionRanges::new();
    ranges.push((PaintTextPortionKind::Link, 0..text.text.len()));
    return ranges;
  }

  let mut ranges = PaintTextPortionRanges::new();
  let mut start = 0usize;
  for (index, ch) in text.text.char_indices() {
    if text.pdf_text_segmentation == common::PdfTextSegmentation::WordLine && ch == '-' {
      if start < index {
        let kind = if text.hyperlink_url.is_some() {
          PaintTextPortionKind::Link
        } else {
          PaintTextPortionKind::Text
        };
        ranges.push((kind, start..index));
      }
      let kind = if text.hyperlink_url.is_some() {
        PaintTextPortionKind::Link
      } else {
        PaintTextPortionKind::Text
      };
      ranges.push((kind, index..index + ch.len_utf8()));
      start = index + ch.len_utf8();
      continue;
    }
    if ch != '\t' && !(split_portions && ch.is_whitespace()) {
      continue;
    }
    if start < index {
      let kind = if text.hyperlink_url.is_some() {
        PaintTextPortionKind::Link
      } else {
        PaintTextPortionKind::Text
      };
      ranges.push((kind, start..index));
    }
    if ch == '\t' {
      ranges.push((PaintTextPortionKind::Tab, index..index + ch.len_utf8()));
      start = index + ch.len_utf8();
    } else if split_portions && start < index {
      start = index;
    }
  }
  if start < text.text.len() {
    let kind = if text.hyperlink_url.is_some() {
      PaintTextPortionKind::Link
    } else {
      PaintTextPortionKind::Text
    };
    ranges.push((kind, start..text.text.len()));
  }
  ranges
}

const OFFICE_TAB_LEADER_PORTION_CHARACTERS: usize = 32;

fn office_tab_leader_portion_ranges(text: &TextItem<'_>) -> Option<PaintTextPortionRanges> {
  if !text.preserve_text_portion {
    return None;
  }
  let mut characters = text.text.chars();
  let fill = characters.next()?;
  if !matches!(fill, '.' | '-' | '_' | '·')
    || characters.clone().count() < OFFICE_TAB_LEADER_PORTION_CHARACTERS
    || !characters.all(|character| character == fill)
  {
    return None;
  }
  // Word fixed output caps a repeated tab-leader text operation at 32
  // characters. Preserve those portion boundaries: besides matching the
  // content stream, PDFium exposes them through segment extraction.
  let kind = if text.hyperlink_url.is_some() {
    PaintTextPortionKind::Link
  } else {
    PaintTextPortionKind::Text
  };
  let mut ranges = PaintTextPortionRanges::new();
  let mut start = 0;
  for (character_index, (byte_index, _)) in text.text.char_indices().enumerate() {
    if character_index > 0 && character_index.is_multiple_of(OFFICE_TAB_LEADER_PORTION_CHARACTERS) {
      ranges.push((kind, start..byte_index));
      start = byte_index;
    }
  }
  ranges.push((kind, start..text.text.len()));
  Some(ranges)
}

fn edge_whitespace_text_portion_ranges(text: &TextItem<'_>) -> PaintTextPortionRanges {
  let kind = if text.hyperlink_url.is_some() {
    PaintTextPortionKind::Link
  } else {
    PaintTextPortionKind::Text
  };
  let leading_end = text
    .text
    .char_indices()
    .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index))
    .unwrap_or(text.text.len());
  let trailing_start = text
    .text
    .char_indices()
    .rev()
    .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index + ch.len_utf8()))
    .unwrap_or(0);
  let mut ranges = PaintTextPortionRanges::new();
  if leading_end > 0 {
    ranges.push((kind, 0..leading_end));
  }
  if leading_end < trailing_start {
    ranges.push((kind, leading_end..trailing_start));
  }
  if trailing_start < text.text.len() {
    ranges.push((kind, trailing_start..text.text.len()));
  }
  ranges
}

fn glyphs_for_text_range(glyphs: &[PaintGlyphFontRun], range: &Range<usize>) -> PaintGlyphFontRuns {
  let mut output = PaintGlyphFontRuns::new();
  let mut range_origin_x_pt = None::<f32>;
  for run in glyphs {
    let mut x_pt = run.x_offset_pt;
    let mut active = None::<PaintGlyphFontRun>;
    for glyph in &run.glyphs {
      let intersects = glyph.text_range.start < range.end && glyph.text_range.end > range.start;
      if intersects {
        let origin_x_pt = *range_origin_x_pt.get_or_insert(x_pt);
        active
          .get_or_insert_with(|| PaintGlyphFontRun {
            font_face: run.font_face.clone(),
            font_size_pt: run.font_size_pt,
            x_offset_pt: x_pt - origin_x_pt,
            glyphs: Vec::with_capacity(run.glyphs.len().min(range.len())),
          })
          .glyphs
          .push(glyph.clone());
      } else if let Some(active) = active.take() {
        output.push(active);
      }
      x_pt += glyph.x_advance * run.font_size_pt;
    }
    if let Some(active) = active {
      output.push(active);
    }
  }
  output
}

fn glyph_runs_visible_width_pt(glyphs: &[PaintGlyphFontRun], horizontal_scale: f32) -> f32 {
  // Paint glyph coordinates are normalized back to the pre-transform text
  // surface so the direct writer can scale their outlines, offsets, and
  // advances exactly once. Portion geometry, however, is page-visible state:
  // highlights, links, decorations, following portions, and unresolved paint
  // definitions all consume the transformed advance.
  glyphs
    .iter()
    .map(|run| {
      run
        .glyphs
        .iter()
        .map(|glyph| glyph.x_advance * run.font_size_pt)
        .sum::<f32>()
    })
    .sum::<f32>()
    * horizontal_scale
}

fn text_highlight_vertical_bounds(
  baseline_y_pt: f32,
  metrics: TextVerticalMetrics,
  use_windows_font_metrics: bool,
) -> (f32, f32) {
  if use_windows_font_metrics && metrics.baseline_offset_pt > 0.0 {
    // The opaque character cell follows the same metrics as its baseline:
    // TEXTMETRIC tmAscent above it and tmHeight (ascent + descent) in total.
    // Typographic ascent plus half-leading is a different alignment box.
    // Mixing the latter with a Windows baseline displaced WML highlights
    // even when the glyphs themselves were correctly placed. Office PDF
    // body/WPS/group controls corroborate both Calibri and Consolas cells.
    (
      baseline_y_pt - metrics.baseline_offset_pt,
      metrics.windows_line_height_pt(),
    )
  } else {
    (
      baseline_y_pt - metrics.ascent_pt - metrics.leading_above_pt(),
      metrics.line_height_pt(),
    )
  }
}

fn paint_rect_for_portion(rect: &PaintRect, x_pt: f32, width_pt: f32) -> PaintRect {
  PaintRect {
    x_pt,
    width_pt,
    ..*rect
  }
}

fn paint_line_for_portion(line: &PaintStrokeLine, x_pt: f32, width_pt: f32) -> PaintStrokeLine {
  PaintStrokeLine {
    x1_pt: x_pt,
    x2_pt: x_pt + width_pt,
    ..*line
  }
}

fn paint_link_for_portion(link: &PaintLink, x_pt: f32, width_pt: f32) -> PaintLink {
  PaintLink {
    x_pt,
    width_pt,
    ..*link
  }
}

fn paint_clip_for_portion(
  clip: Option<PaintClipRect>,
  kind: &PaintTextPortionKind,
  page_width_pt: f32,
) -> Option<PaintClipRect> {
  let mut clip = clip?;
  if matches!(kind, PaintTextPortionKind::Tab) {
    let paint_right_pt = page_width_pt.max(clip.x_pt + clip.width_pt);
    clip.width_pt = (paint_right_pt - clip.x_pt).max(clip.width_pt);
  }
  Some(clip)
}

#[derive(Clone, Copy, Debug)]
struct PaintLineOwner {
  frame_index: usize,
  line_index: usize,
  frame_kind: FollowFrameKind,
  clip: Option<PaintClipRect>,
}

fn same_paint_line_owner(left: Option<PaintLineOwner>, right: Option<PaintLineOwner>) -> bool {
  match (left, right) {
    (None, None) => true,
    (Some(left), Some(right)) => {
      left.frame_index == right.frame_index
        && left.line_index == right.line_index
        && left.frame_kind == right.frame_kind
    }
    _ => false,
  }
}

fn display_item_paint_owner_origin(item: &common::DisplayItem<'_>) -> Option<(f32, f32)> {
  match item {
    common::DisplayItem::Text(text) => {
      let x_pt = text.origin.x.0;
      let y_pt = text.origin.y.0;
      let degrees = text.style.rotation_degrees;
      if !degrees.is_finite() || degrees.abs() <= f32::EPSILON {
        return Some((x_pt, y_pt));
      }
      let center = text.rotation_center.unwrap_or(text.origin);
      if ![x_pt, y_pt, center.x.0, center.y.0]
        .into_iter()
        .all(f32::is_finite)
      {
        return Some((x_pt, y_pt));
      }

      let normalized = f64::from(degrees).rem_euclid(360.0);
      let signed_degrees = if normalized > 180.0 {
        normalized - 360.0
      } else {
        normalized
      };
      let (sin, cos) = signed_degrees.to_radians().sin_cos();
      let dx = f64::from(x_pt - center.x.0);
      let dy = f64::from(y_pt - center.y.0);
      let transformed = (
        (f64::from(center.x.0) + dx * cos - dy * sin) as f32,
        (f64::from(center.y.0) + dx * sin + dy * cos) as f32,
      );
      if transformed.0.is_finite() && transformed.1.is_finite() {
        Some(transformed)
      } else {
        Some((x_pt, y_pt))
      }
    }
    common::DisplayItem::Glyphs(glyphs) => Some((glyphs.origin.x.0, glyphs.origin.y.0)),
    _ => None,
  }
}

fn table_cell_fragment_order(
  left: &common::FrameFragment,
  right: &common::FrameFragment,
) -> std::cmp::Ordering {
  let left_range = left.item_range.end - left.item_range.start;
  let right_range = right.item_range.end - right.item_range.start;
  left_range.cmp(&right_range).then_with(|| {
    let area = |fragment: &common::FrameFragment| {
      fragment.bounds.map_or(f32::INFINITY, |bounds| {
        bounds.size.width.0 * bounds.size.height.0
      })
    };
    area(left).total_cmp(&area(right))
  })
}

fn table_cell_clip_bounds(
  fragments: &[common::FrameFragment],
  item_index: usize,
  item: &common::DisplayItem<'_>,
) -> Option<common::Rect> {
  let cell_fragments = || {
    fragments
      .iter()
      .filter(|fragment| fragment.kind == common::FrameFragmentKind::TableCell)
  };
  let origin_match = display_item_paint_owner_origin(item).and_then(|(x_pt, y_pt)| {
    cell_fragments()
      .filter(|fragment| {
        fragment.bounds.is_some_and(|bounds| {
          x_pt + f32::EPSILON >= bounds.origin.x.0
            && x_pt < bounds.origin.x.0 + bounds.size.width.0
            && y_pt + f32::EPSILON >= bounds.origin.y.0
            && y_pt < bounds.origin.y.0 + bounds.size.height.0
        })
      })
      .min_by(|left, right| table_cell_fragment_order(left, right))
  });

  // A rotated baseline can lie exactly on the cell's right/bottom-open edge,
  // as Word's btLr layout does. The page display range remains authoritative
  // in that boundary case. Keep it as a fallback so ordinary and nested-cell
  // ownership continues to prefer the unambiguous physical origin.
  origin_match
    .or_else(|| {
      cell_fragments()
        .filter(|fragment| {
          fragment.item_range.start <= item_index && item_index < fragment.item_range.end
        })
        .min_by(|left, right| table_cell_fragment_order(left, right))
    })
    .and_then(|fragment| fragment.bounds)
}

fn paint_line_owners(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
  items: &[common::DisplayItem<'static>],
) -> Vec<Option<PaintLineOwner>> {
  let item_count = items.len();
  let mut owners = vec![None; item_count];
  for (frame_index, frame) in document
    .frames
    .iter()
    .enumerate()
    .filter(|(_, frame)| frame.page_index == page_index)
  {
    let frame_kind = frame_kind_name_from_common(&frame.kind);
    for (line_index, line) in frame.lines.iter().enumerate() {
      let start = line.item_range.start.min(item_count);
      let end = line.item_range.end.min(item_count);
      // Writer's normal PDF paint path does not clip paragraph text to the
      // line rectangle. Glyph ink and justified terminal blanks may extend
      // into the paragraph margin; SwTextPainter only installs a line clip
      // for an undersized/clipping frame. Table cells are the exception: the
      // cell fragment owns a real print rectangle and clips its inline text.
      for (item_index, owner) in owners.iter_mut().enumerate().take(end).skip(start) {
        if owner.is_none() {
          let clip_bounds = (frame_kind == FollowFrameKind::Table).then(|| {
            // One physical baseline can contain text from several adjacent
            // cells, and a nested table can add another cell hierarchy at
            // that same baseline. Select the narrowest fragment which owns
            // this item, rather than one fragment for the complete line;
            // otherwise the chosen cell clips every sibling's text while
            // leaving it only in the semantic PDF layer.
            // Nested table fragments are folded into their outer table frame,
            // so a same-baseline item can outlive an imprecise flattened item
            // range. Prefer the smallest cell containing its physical paint
            // origin. Rotated text must first move that logical origin into
            // page space; its item range is used only at an open-edge tie.
            table_cell_clip_bounds(&frame.fragments, item_index, &items[item_index])
              .unwrap_or(line.bounds)
          });
          *owner = Some(PaintLineOwner {
            frame_index,
            line_index,
            frame_kind,
            clip: clip_bounds.map(|bounds| PaintClipRect {
              x_pt: bounds.origin.x.0,
              y_pt: bounds.origin.y.0,
              width_pt: bounds.size.width.0,
              height_pt: bounds.size.height.0,
            }),
          });
        }
      }
    }
  }
  owners
}

pub(super) fn paint_item_intersects_page(
  item: &PaintItem<'_>,
  page_width_pt: f32,
  page_height_pt: f32,
) -> bool {
  // the page rectangle before SwRootFrame::PaintSwFrame(); drawing layers also
  // receive the page frame in sw/source/core/view/vdraw.cxx.
  let Some((left, top, right, bottom)) = paint_item_bounds(item) else {
    return true;
  };
  right > 0.0 && bottom > 0.0 && left < page_width_pt && top < page_height_pt
}

fn paint_item_bounds(item: &PaintItem<'_>) -> Option<(f32, f32, f32, f32)> {
  match item {
    PaintItem::Text(text) => {
      let item = &text.item;
      if let Some(bounds) = item.page_culling_bounds {
        return Some((
          bounds.x_pt,
          bounds.y_pt,
          bounds.x_pt + bounds.width_pt,
          bounds.y_pt + bounds.height_pt,
        ));
      }
      let bounds = (
        item.x_pt,
        item.y_pt,
        item.x_pt + text.width_pt,
        item.y_pt + item.line_height_pt,
      );
      if item.style.rotation_deg.abs() <= f32::EPSILON {
        return Some(bounds);
      }
      let (rotation_x, rotation_y) = item.rotation_center_pt.unwrap_or((item.x_pt, item.y_pt));
      Some(rotated_rect_bounds(
        bounds,
        rotation_x,
        rotation_y,
        item.style.rotation_deg,
      ))
    }
    PaintItem::Image(image) => Some((
      image.x_pt,
      image.y_pt,
      image.x_pt + image.width_pt,
      image.y_pt + image.height_pt,
    )),
    PaintItem::Group {
      transform, items, ..
    } => {
      let bounds = items
        .iter()
        .filter_map(paint_item_bounds)
        .reduce(union_paint_bounds)?;
      transform.as_ref().map_or(Some(bounds), |transform| {
        Some(transform_paint_bounds(bounds, transform))
      })
    }
    PaintItem::LinkArea(link_area) => Some((
      link_area.x_pt,
      link_area.y_pt,
      link_area.x_pt + link_area.width_pt,
      link_area.y_pt + link_area.height_pt,
    )),
    PaintItem::Rect(rect) => Some((
      rect.x_pt,
      rect.y_pt,
      rect.x_pt + rect.width_pt,
      rect.y_pt + rect.height_pt,
    )),
    PaintItem::Line(line) => {
      let half_width = line.width_pt / 2.0;
      Some((
        line.x1_pt.min(line.x2_pt) - half_width,
        line.y1_pt.min(line.y2_pt) - half_width,
        line.x1_pt.max(line.x2_pt) + half_width,
        line.y1_pt.max(line.y2_pt) + half_width,
      ))
    }
    PaintItem::Polyline(polyline) => Some((
      polyline.x_pt,
      polyline.y_pt,
      polyline.x_pt + polyline.width_pt,
      polyline.y_pt + polyline.height_pt,
    )),
  }
}

fn union_paint_bounds(
  left: (f32, f32, f32, f32),
  right: (f32, f32, f32, f32),
) -> (f32, f32, f32, f32) {
  (
    left.0.min(right.0),
    left.1.min(right.1),
    left.2.max(right.2),
    left.3.max(right.3),
  )
}

fn transform_paint_bounds(
  (left, top, right, bottom): (f32, f32, f32, f32),
  transform: &common::Transform,
) -> (f32, f32, f32, f32) {
  let transform_point = |x: f32, y: f32| {
    (
      transform.m11 * x + transform.m21 * y + transform.dx.0,
      transform.m12 * x + transform.m22 * y + transform.dy.0,
    )
  };
  let corners = [
    transform_point(left, top),
    transform_point(right, top),
    transform_point(right, bottom),
    transform_point(left, bottom),
  ];
  corners.into_iter().fold(
    (
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ),
    |bounds, (x, y)| {
      (
        bounds.0.min(x),
        bounds.1.min(y),
        bounds.2.max(x),
        bounds.3.max(y),
      )
    },
  )
}

fn rotated_rect_bounds(
  (left, top, right, bottom): (f32, f32, f32, f32),
  rotation_x: f32,
  rotation_y: f32,
  rotation_deg: f32,
) -> (f32, f32, f32, f32) {
  let angle = rotation_deg.to_radians();
  let corners = [
    rotate_point(left, top, rotation_x, rotation_y, angle),
    rotate_point(right, top, rotation_x, rotation_y, angle),
    rotate_point(right, bottom, rotation_x, rotation_y, angle),
    rotate_point(left, bottom, rotation_x, rotation_y, angle),
  ];
  let mut min_x = f32::INFINITY;
  let mut min_y = f32::INFINITY;
  let mut max_x = f32::NEG_INFINITY;
  let mut max_y = f32::NEG_INFINITY;
  for (x, y) in corners {
    min_x = min_x.min(x);
    min_y = min_y.min(y);
    max_x = max_x.max(x);
    max_y = max_y.max(y);
  }
  (min_x, min_y, max_x, max_y)
}

fn rotate_point(x: f32, y: f32, rotation_x: f32, rotation_y: f32, angle: f32) -> (f32, f32) {
  let dx = x - rotation_x;
  let dy = y - rotation_y;
  (
    rotation_x + dx * angle.cos() - dy * angle.sin(),
    rotation_y + dx * angle.sin() + dy * angle.cos(),
  )
}

pub(super) fn text_has_visible_glyph_paint(style: &TextStyle<'_>) -> bool {
  style.semantic_only
    || style.opacity > f32::EPSILON
    || (style.outline_color.is_some()
      && style.outline_width_pt > f32::EPSILON
      && style.outline_opacity > f32::EPSILON)
}

pub(super) fn text_requires_glyph_outlines(style: &TextStyle<'_>) -> bool {
  // Office's fixed-format writers convert translucent glyphs to paths. This
  // preserves the alpha compositing result without exposing those glyphs as
  // PDF text; both Word's w14:textFill alpha and PowerPoint's DrawingML alpha
  // use that path. Explicit glyph-outline rendering must remain active when
  // the authored fill is `noFill`: the independently visible text outline is
  // still painted as vector glyph geometry. Opaque ordinary text remains real
  // PDF text for search/accessibility.
  !style.semantic_only
    && (style.pdf_glyph_outlines
      || (style.opacity > f32::EPSILON && style.opacity < 1.0 - f32::EPSILON))
}

/// Preserve laid-out glyph positions independently of the realized paint em.
/// WordArt's deformation source retains the authored layout positions while
/// enclosing the realized outlines. Replacing its advances with the rounded
/// PDF font's advances accumulates a new error
/// inside every XML run. Office size/run-segmentation controls distinguish the
/// two owners; the glyph outlines themselves keep their realized font sizes.
fn retain_layout_glyph_positions(paint: &mut PaintGlyphRun, layout: &PaintGlyphRun) {
  let positions = layout
    .font_runs
    .iter()
    .flat_map(|run| {
      let mut x = run.x_offset_pt;
      run.glyphs.iter().map(move |glyph| {
        let position = (run, glyph, x);
        x += glyph.x_advance * run.font_size_pt;
        position
      })
    })
    .collect::<Vec<_>>();
  let compatible = paint
    .font_runs
    .iter()
    .map(|run| run.glyphs.len())
    .sum::<usize>()
    == positions.len()
    && paint
      .font_runs
      .iter()
      .flat_map(|run| run.glyphs.iter().map(move |glyph| (run, glyph)))
      .zip(&positions)
      .all(
        |((paint_run, paint_glyph), (layout_run, layout_glyph, _))| {
          paint_run.font_face.id() == layout_run.font_face.id()
            && paint_glyph.glyph_id == layout_glyph.glyph_id
            && paint_glyph.text_range == layout_glyph.text_range
        },
      );
  // A different shaping topology is not a one-to-one position transfer. Never
  // partially rewrite a run or assign another cluster's placement to a glyph.
  if !compatible {
    return;
  }
  let mut positions = positions.into_iter();
  for run in &mut paint.font_runs {
    for (index, glyph) in run.glyphs.iter_mut().enumerate() {
      let (layout_run, layout_glyph, x) = positions.next().expect("glyph counts were checked");
      if index == 0 {
        run.x_offset_pt = x;
      }
      let scale = layout_run.font_size_pt / run.font_size_pt;
      glyph.x_advance = layout_glyph.x_advance * scale;
      glyph.y_advance = layout_glyph.y_advance * scale;
      glyph.x_offset = layout_glyph.x_offset * scale;
      glyph.y_offset = layout_glyph.y_offset * scale;
    }
  }
  paint.width_pt = layout.width_pt;
}

fn text_layout_glyphs(
  text: &str,
  style: &TextStyle<'_>,
  word_spacing_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Option<PaintGlyphRun> {
  let sizes = style.layout_font_sizes?;
  if sizes.primary.0 == style.font_size_pt
    && sizes.complex.map(|size| size.0) == style.complex_font_size_pt
  {
    return None;
  }
  // Logical character cells are not the advances remeasured at the rounded
  // output em. Shape at the logical sizes instead
  // of scaling the final width: character spacing, justification and explicit
  // GDI advances are independent of the em, and scripts can use different ems.
  let mut logical_style = style.clone();
  logical_style.font_size_pt = sizes.primary.0;
  logical_style.complex_font_size_pt = sizes.complex.map(|size| size.0);
  shaped_pdf_glyphs(text, &logical_style, word_spacing_pt, text_metrics)
}

fn shaped_pdf_glyphs(
  text: &str,
  style: &TextStyle<'_>,
  word_spacing_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Option<PaintGlyphRun> {
  // U+FE0E selects the text presentation of the preceding emoji, but it is
  // not itself a drawable character. Keep it in the semantic source while
  // shaping the base emoji alone so the selector cannot redirect the whole
  // cluster to a symbol fallback face (Pandoc test/command/11113.docx).
  let shaping_text_storage = text
    .strip_suffix('\u{fe0e}')
    .map(|prefix| format!("{prefix}\u{fe0f}"));
  let shaping_text = shaping_text_storage.as_deref().unwrap_or(text);
  let shaped = text_metrics.shape_text(shaping_text, style)?;
  let semantic_advances = style
    .semantic_character_advances_pt
    .as_deref()
    .filter(|advances| advances.len() == text.chars().count());
  let horizontal_scale = style.horizontal_scale.unwrap_or(1.0).max(f32::EPSILON);
  let mut font_runs = PaintGlyphFontRuns::new();
  let mut x_offset_pt = 0.0;
  let mut last_run = None::<(usize, u32)>;
  for glyph in shaped.glyphs {
    let run_key = (glyph.font_index, glyph.font_size_pt.to_bits());
    if last_run != Some(run_key) {
      let font_face = shaped.font_faces.get(glyph.font_index)?.clone();
      font_runs.push(PaintGlyphFontRun {
        font_face,
        font_size_pt: glyph.font_size_pt,
        // The surface transform scales the glyph outlines and their paint
        // coordinates. Undo the logical layout scale here so the transform
        // produces exactly the already-scaled advance once, not twice.
        x_offset_pt: x_offset_pt / horizontal_scale,
        glyphs: Vec::new(),
      });
      last_run = Some(run_key);
    }
    let is_word_space = shaping_text
      .get(glyph.text_range.clone())
      .is_some_and(|cluster| cluster.contains(' '));
    let word_spacing_em = if is_word_space {
      word_spacing_pt / glyph.font_size_pt
    } else {
      0.0
    };
    let natural_advance_pt =
      glyph.x_advance_em * glyph.font_size_pt + word_spacing_em * glyph.font_size_pt;
    let advance_pt = semantic_advances
      .and_then(|advances| semantic_advance_for_text_range(text, advances, &glyph.text_range))
      .unwrap_or(natural_advance_pt);
    font_runs
      .last_mut()
      .expect("font run was just pushed")
      .glyphs
      .push(PaintGlyph {
        glyph_id: glyph.glyph_id,
        text_range: glyph.text_range,
        x_advance: advance_pt / glyph.font_size_pt / horizontal_scale,
        x_offset: glyph.x_offset_em / horizontal_scale,
        y_offset: glyph.y_offset_em,
        y_advance: glyph.y_advance_em,
        bounds_em: glyph.bounds_em.map(|bounds| PdfGlyphBoundsDiagnostics {
          x_min_em: bounds.x_min_em,
          y_min_em: bounds.y_min_em,
          x_max_em: bounds.x_max_em,
          y_max_em: bounds.y_max_em,
        }),
      });
    x_offset_pt += advance_pt;
  }
  if shaping_text.len() < text.len()
    && let Some(glyph) = font_runs
      .iter_mut()
      .rev()
      .find_map(|run| run.glyphs.last_mut())
    && glyph.text_range.end == shaping_text.len()
  {
    glyph.text_range.end = text.len();
  }
  Some(PaintGlyphRun {
    width_pt: x_offset_pt,
    font_runs,
  })
}

fn semantic_advance_for_text_range(
  text: &str,
  advances: &[f32],
  range: &Range<usize>,
) -> Option<f32> {
  let mut total = 0.0;
  let mut matched = false;
  for ((byte_index, _), advance) in text.char_indices().zip(advances) {
    if byte_index >= range.start && byte_index < range.end {
      total += advance;
      matched = true;
    }
  }
  (matched && total.is_finite()).then_some(total)
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use super::{
    PaintGlyph, display_item_paint_owner_origin, remap_glyph_text_ranges,
    symbol_font_semantic_text, table_cell_clip_bounds, text_style_from_common,
    word_no_break_hyphen_semantic_text, word_small_caps_semantic_text,
  };
  use ooxmlsdk_layout::common;

  #[test]
  fn wordart_positions_keep_layout_advances_and_realized_glyph_sizes() {
    use super::{TextMetrics, TextStyle, retain_layout_glyph_positions, shaped_pdf_glyphs};

    let mut metrics = TextMetrics::new();
    for text in ["abc Transform", "This is a longer second line.", "אב i Wm"] {
      for (size, realized) in [(10.0, 9.96), (11.0, 11.04), (12.0, 12.0), (14.0, 14.04)] {
        for spacing in [0.0, 1.5] {
          for horizontal_scale in [0.8, 1.0, 1.2] {
            let style = TextStyle {
              font_family: Some(Cow::Borrowed("Courier New")),
              font_size_pt: size,
              character_spacing_pt: spacing,
              horizontal_scale: Some(horizontal_scale),
              ..Default::default()
            };
            let layout = shaped_pdf_glyphs(text, &style, spacing, &mut metrics).unwrap();
            let mut paint = shaped_pdf_glyphs(
              text,
              &TextStyle {
                font_size_pt: realized,
                ..style
              },
              spacing,
              &mut metrics,
            )
            .unwrap();
            let paint_sizes = paint
              .font_runs
              .iter()
              .map(|run| run.font_size_pt)
              .collect::<Vec<_>>();
            retain_layout_glyph_positions(&mut paint, &layout);
            assert_eq!(paint.width_pt, layout.width_pt);
            assert_eq!(
              paint
                .font_runs
                .iter()
                .map(|run| run.font_size_pt)
                .collect::<Vec<_>>(),
              paint_sizes,
            );
            let advances = |run: &super::PaintGlyphRun| {
              run
                .font_runs
                .iter()
                .flat_map(|font| {
                  font
                    .glyphs
                    .iter()
                    .map(move |glyph| glyph.x_advance * font.font_size_pt)
                })
                .collect::<Vec<_>>()
            };
            for (actual, expected) in advances(&paint).into_iter().zip(advances(&layout)) {
              assert!((actual - expected).abs() < 0.00001);
            }
          }
        }
      }
    }
  }

  #[test]
  fn wordart_position_transfer_rejects_different_clusters_without_partial_changes() {
    let mut metrics = super::TextMetrics::new();
    let style = super::TextStyle {
      font_family: Some(Cow::Borrowed("Arial")),
      font_size_pt: 11.0,
      ..Default::default()
    };
    let layout = super::shaped_pdf_glyphs("abc", &style, 0.0, &mut metrics).unwrap();
    let mut paint = super::shaped_pdf_glyphs(
      "xyz",
      &super::TextStyle {
        font_size_pt: 11.04,
        ..style
      },
      0.0,
      &mut metrics,
    )
    .unwrap();
    let before = format!("{paint:?}");
    super::retain_layout_glyph_positions(&mut paint, &layout);
    assert_eq!(format!("{paint:?}"), before);
  }

  #[test]
  fn highlight_width_uses_layout_sizes_without_scaling_absolute_spacing() {
    use super::{TextMetrics, TextStyle, shaped_pdf_glyphs, text_layout_glyphs};

    let mut metrics = TextMetrics::new();
    for text in ["darkYellow", "i Wm", "אב cd"] {
      for spacing in [0.0, 1.5] {
        for scale in [0.8, 1.0, 1.2] {
          let logical = TextStyle {
            font_family: Some(Cow::Borrowed("Arial")),
            font_size_pt: 11.0,
            complex_font_size_pt: Some(14.0),
            character_spacing_pt: spacing,
            horizontal_scale: Some(scale),
            ..Default::default()
          };
          let expected = shaped_pdf_glyphs(text, &logical, spacing, &mut metrics)
            .expect("test font must shape")
            .width_pt;
          let painted = TextStyle {
            font_size_pt: 11.04,
            complex_font_size_pt: Some(14.04),
            layout_font_sizes: Some(common::LayoutFontSizes {
              primary: common::Pt(11.0),
              complex: Some(common::Pt(14.0)),
            }),
            ..logical
          };
          assert_eq!(
            text_layout_glyphs(text, &painted, spacing, &mut metrics)
              .expect("logical highlight glyphs")
              .width_pt,
            expected
          );
          assert_eq!(painted.font_size_pt, 11.04);
          assert_eq!(painted.complex_font_size_pt, Some(14.04));
        }
      }
    }
    let plain = TextStyle::default();
    assert!(text_layout_glyphs("x", &plain, 0.0, &mut metrics).is_none());
  }

  #[test]
  fn highlight_layout_width_survives_paint_portion_segmentation() {
    let common::DisplayItem::Text(mut run) = text_item(point(10.0, 20.0), None, 0.0) else {
      unreachable!()
    };
    run.text = Cow::Borrowed("one-two");
    run.pdf_text_segmentation = common::PdfTextSegmentation::WordLine;
    run.style.font_family = Some(Cow::Borrowed("Arial"));
    run.style.font_size = common::Pt(11.04);
    run.style.highlight = Some(common::Color {
      r: 255,
      g: 255,
      b: 0,
      a: 255,
    });
    run.style.layout_font_sizes = Some(common::LayoutFontSizes {
      primary: common::Pt(11.0),
      complex: None,
    });
    let mut metrics = super::TextMetrics::new();
    let text = super::text_item_from_common(&run);
    let expected = super::text_layout_glyphs(&text.text, &text.style, 0.0, &mut metrics)
      .expect("logical glyphs")
      .width_pt;
    let painted = super::PaintText::from_layout_text(text, None, None, 600.0, &mut metrics);
    assert_eq!(painted.portions.len(), 3);
    let mut end = 10.0;
    for portion in &painted.portions {
      let rect = portion.highlight.expect("every portion highlighted");
      assert!((rect.x_pt - end).abs() < 0.0001);
      end += rect.width_pt;
    }
    assert!((end - 10.0 - expected).abs() < 0.0001);
    let mut original_run = run.clone();
    original_run.style.layout_font_sizes = None;
    let unchanged = super::PaintText::from_layout_text(
      super::text_item_from_common(&original_run),
      None,
      None,
      600.0,
      &mut metrics,
    );
    assert_eq!(painted.width_pt, unchanged.width_pt);
    assert_eq!(painted.baseline_y, unchanged.baseline_y);
    for (actual, original) in painted.portions.iter().zip(&unchanged.portions) {
      assert_eq!(actual.x_pt, original.x_pt);
      assert_eq!(actual.width_pt, original.width_pt);
    }
  }

  #[test]
  fn highlight_bounds_keep_windows_and_typographic_metrics_independent() {
    use ooxmlsdk_layout::text_metrics::TextVerticalMetrics;

    // Deliberately unequal alignment boxes, so half-leading cannot
    // accidentally produce the Windows result. Scale without a fitted offset.
    for scale in [0.5, 1.0, 2.0, 4.0] {
      let metrics = TextVerticalMetrics {
        ascent_pt: 8.0 * scale,
        descent_pt: 2.0 * scale,
        windows_line_height_pt: 13.0 * scale,
        line_gap_pt: 2.0 * scale,
        baseline_offset_pt: 10.5 * scale,
        directwrite_baseline_offset_pt: 11.0 * scale,
        wordprocessingml_cjk_line_metrics: false,
      };
      for baseline in [0.0, 20.0, 100.0] {
        assert_eq!(
          super::text_highlight_vertical_bounds(baseline, metrics, true),
          (baseline - 10.5 * scale, 13.0 * scale)
        );
        assert_eq!(
          super::text_highlight_vertical_bounds(baseline, metrics, false),
          (baseline - 9.0 * scale, 12.0 * scale)
        );
        assert_eq!(
          super::text_highlight_vertical_bounds(
            baseline,
            TextVerticalMetrics {
              baseline_offset_pt: 0.0,
              ..metrics
            },
            true
          ),
          (baseline - 9.0 * scale, 12.0 * scale)
        );
      }
    }
  }

  fn point(x: f32, y: f32) -> common::Point {
    common::Point {
      x: common::Pt(x),
      y: common::Pt(y),
    }
  }

  fn rect(x: f32, y: f32, width: f32, height: f32) -> common::Rect {
    common::Rect {
      origin: point(x, y),
      size: common::Size {
        width: common::Pt(width),
        height: common::Pt(height),
      },
    }
  }

  fn text_item(
    origin: common::Point,
    rotation_center: Option<common::Point>,
    rotation_degrees: f32,
  ) -> common::DisplayItem<'static> {
    let style = common::TextStyle {
      rotation_degrees,
      ..common::TextStyle::default()
    };
    common::DisplayItem::Text(common::TextRun {
      text: Cow::Borrowed("x"),
      origin,
      line_height: common::Pt(12.0),
      line_metrics_participant: true,
      paint_clip: None,
      page_culling_bounds: None,
      style,
      font_id: None,
      color: common::Color {
        r: 0,
        g: 0,
        b: 0,
        a: u8::MAX,
      },
      rotation_center,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      pdf_text_segmentation: common::PdfTextSegmentation::default(),
      source: None,
    })
  }

  fn table_cell_fragment(
    bounds: common::Rect,
    item_range: std::ops::Range<usize>,
  ) -> common::FrameFragment {
    common::FrameFragment {
      kind: common::FrameFragmentKind::TableCell,
      split: common::FragmentSplitKind::Complete,
      index: 0,
      row_index: 0,
      cell_index: Some(0),
      item_range: common::ItemRange {
        start: item_range.start,
        end: item_range.end,
      },
      bounds: Some(bounds),
    }
  }

  fn glyph(text_range: std::ops::Range<usize>) -> PaintGlyph {
    PaintGlyph {
      glyph_id: 1,
      text_range,
      x_advance: 1.0,
      x_offset: 0.0,
      y_offset: 0.0,
      y_advance: 0.0,
      bounds_em: None,
    }
  }

  #[test]
  fn semantic_text_transforms_keep_layout_text_and_change_only_source_backed_pdf_mappings() {
    assert_eq!(word_small_caps_semantic_text("Xxxx", true), "XXXX");
    assert_eq!(word_small_caps_semantic_text("Xxxx", false), "Xxxx");
    assert_eq!(word_small_caps_semantic_text("ı", true), "ı");
    assert_eq!(word_no_break_hyphen_semantic_text("A\u{2011}B"), "A-B");
    assert_eq!(word_no_break_hyphen_semantic_text("A-B"), "A-B");

    assert_eq!(
      symbol_font_semantic_text("\u{f0b7}", Some("Symbol")),
      "\u{2022}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f020}\u{f0fc}", Some("Wingdings")),
      "\u{2002}\u{2713}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f097}\u{f0a3}", Some("Wingdings 2")),
      "\u{2981}\u{25a1}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f07d}", Some("Wingdings 3")),
      "\u{1f782}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f067}\u{f06e}", Some("Webdings")),
      "\u{2b1b}\u{2b24}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f04c}\u{f04d}\u{f04f}", Some("MT Extra")),
      "\u{22ef}\u{22ee}\u{22f1}"
    );
    assert_eq!(
      symbol_font_semantic_text("o", Some("Bookshelf Symbol 7")),
      "\u{f06f}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f0b7}", Some("Calibri")),
      "\u{f0b7}"
    );
  }

  #[test]
  fn webdings_map_symbols_remap_semantics_without_changing_glyphs() {
    let source = "\u{f045}\u{f04a}\u{f049}";
    let semantic = symbol_font_semantic_text(source, Some("wEbDiNgS"));
    assert_eq!(semantic, "\u{1f3dc}\u{1f3dd}\u{26f1}");
    let glyphs = [glyph(0..3), glyph(3..6), glyph(6..9)];
    let remapped = remap_glyph_text_ranges(&glyphs, source, &semantic).unwrap();
    for (index, expected_range) in [0..4, 4..8, 8..11].into_iter().enumerate() {
      assert_eq!(remapped[index].text_range, expected_range);
      assert_eq!(remapped[index].glyph_id, glyphs[index].glyph_id);
      assert_eq!(remapped[index].x_advance, glyphs[index].x_advance);
    }
    assert_eq!(symbol_font_semantic_text(source, Some("Calibri")), source);
    assert_eq!(
      symbol_font_semantic_text("\u{f04a}", Some("Wingdings")),
      "\u{263a}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{e225}", Some("ZBFH")),
      "\u{e225}"
    );
  }

  #[test]
  fn legacy_symbol_missing_selectors_keep_font_scope_and_unknown_private_use() {
    for family in ["Symbol", "SymbolMT", "sYmBoLmT"] {
      assert_eq!(
        symbol_font_semantic_text("\u{f0de}", Some(family)),
        "\u{21d2}"
      );
    }
    for family in ["Wingdings", "wInGdInGs"] {
      assert_eq!(
        symbol_font_semantic_text("\u{f0a8}\u{f0e8}", Some(family)),
        "\u{f0a8}\u{1f87a}"
      );
    }
    let selectors = "\u{f0a8}\u{f0e8}\u{f0de}";
    for family in [
      None,
      Some("Calibri"),
      Some("Wingdings 2"),
      Some("Wingdings2"),
      Some("Wingdings 3"),
      Some("Webdings"),
      Some("Symbol Bold"),
    ] {
      let semantic = symbol_font_semantic_text(selectors, family);
      assert_eq!(semantic, selectors, "unrelated font {family:?}");
      assert!(matches!(semantic, Cow::Borrowed(_)));
    }
    // The same selector has different meaning in the two legacy fonts.
    assert_eq!(
      symbol_font_semantic_text("\u{f0e8}", Some("Symbol")),
      "\u{239d}"
    );
    for family in ["Symbol", "Wingdings"] {
      let unknown = "\u{f0ff}\u{e225}\u{e004}";
      assert_eq!(symbol_font_semantic_text(unknown, Some(family)), unknown);
      // Ordinary Unicode scalars are not F000-offset legacy selectors.
      let ordinary = "\u{00a8}\u{00e8}\u{00de}";
      assert_eq!(symbol_font_semantic_text(ordinary, Some(family)), ordinary);
    }
  }

  #[test]
  fn legacy_symbol_office_square_selector_preserves_its_private_use_encoding() {
    // Office retains F0A8 in tdf138899's PDF 1.4 output. A standardized
    // pictograph exists, but substituting it changes the extracted text.
    for family in ["Wingdings", "wInGdInGs"] {
      let semantic = symbol_font_semantic_text("\u{f0a8}", Some(family));
      assert_eq!(semantic, "\u{f0a8}");
      assert!(matches!(semantic, Cow::Borrowed(_)));
    }
  }

  #[test]
  fn legacy_symbol_scalar_ranges_preserve_all_glyph_metrics_and_visual_order() {
    let source = "A\u{f0e8}\u{f0e8}B\u{e225}";
    let semantic = symbol_font_semantic_text(source, Some("Wingdings"));
    assert_eq!(semantic, "A\u{1f87a}\u{1f87a}B\u{e225}");
    assert_eq!((source.len(), semantic.len()), (11, 13));
    let source_ranges = [0..1, 1..4, 4..7, 4..7, 7..8, 8..11];
    let semantic_ranges = [0..1, 1..5, 5..9, 5..9, 9..10, 10..13];
    let ids = [17, 42, 91, 92, 3, 99];
    let advances = [0.75, 1.25, 0.0, 1.5, 0.25, 0.5];
    let y_advances = [0.0, 0.125, 0.0, -0.125, 0.25, 0.0];
    let glyphs = source_ranges
      .iter()
      .enumerate()
      .map(|(index, range)| PaintGlyph {
        glyph_id: ids[index],
        text_range: range.clone(),
        x_advance: advances[index],
        x_offset: index as f32 * 0.125 - 0.25,
        y_offset: index as f32 * -0.0625,
        y_advance: y_advances[index],
        bounds_em: Some(super::PdfGlyphBoundsDiagnostics {
          x_min_em: -0.125,
          y_min_em: -0.25,
          x_max_em: 0.75 + index as f32 * 0.125,
          y_max_em: 1.0,
        }),
      })
      .collect::<Vec<_>>();
    for reverse in [false, true] {
      let mut visual_glyphs = glyphs.clone();
      let mut expected_ranges = semantic_ranges.clone();
      if reverse {
        visual_glyphs.reverse();
        expected_ranges.reverse();
      }
      let remapped = remap_glyph_text_ranges(&visual_glyphs, source, &semantic)
        .expect("one-to-one scalars must preserve shared clusters and visual order");
      assert_eq!(remapped.len(), visual_glyphs.len());
      for ((before, after), expected) in visual_glyphs
        .iter()
        .zip(remapped.iter())
        .zip(expected_ranges)
      {
        assert_eq!(after.text_range, expected);
        assert_eq!(after.glyph_id, before.glyph_id);
        assert_eq!(after.x_advance.to_bits(), before.x_advance.to_bits());
        assert_eq!(after.y_advance.to_bits(), before.y_advance.to_bits());
        assert_eq!(after.x_offset.to_bits(), before.x_offset.to_bits());
        assert_eq!(after.y_offset.to_bits(), before.y_offset.to_bits());
        assert_eq!(after.bounds_em, before.bounds_em);
        assert!(semantic.get(after.text_range.clone()).is_some());
      }
    }
    // Remapping cannot mutate the original glyph run used for visible outlines.
    for (glyph, original_range) in glyphs.iter().zip(source_ranges) {
      assert_eq!(glyph.text_range, original_range);
    }
    assert!(remap_glyph_text_ranges(&[glyph(2..4)], source, &semantic).is_none());
    assert!(remap_glyph_text_ranges(&glyphs, source, "expanded into extra scalars").is_none());
  }

  #[test]
  fn semantic_range_remapping_is_scalar_indexed_and_rejects_non_bijective_substitutions() {
    let source = "\u{f04d}x";
    let semantic = "\u{1f4a3}x";
    let glyphs = [glyph(0..3), glyph(3..4)];
    let remapped = remap_glyph_text_ranges(&glyphs, source, semantic)
      .expect("one-to-one scalar substitution must remap");
    assert_eq!(remapped[0].text_range, 0..4);
    assert_eq!(remapped[1].text_range, 4..5);
    assert!(remap_glyph_text_ranges(&[glyph(0..1)], "A", "AA").is_none());
  }

  #[test]
  fn pdf_paint_style_preserves_word_opentype_features() {
    let mut stylistic_sets = common::OpenTypeStylisticSets::default();
    stylistic_sets.enable(7);
    let mut open_type_features = common::OpenTypeFeatureSettings::default();
    open_type_features.number_form = Some(common::OpenTypeNumberForm::OldStyle);
    open_type_features.number_spacing = Some(common::OpenTypeNumberSpacing::Proportional);
    open_type_features.contextual_alternates = Some(true);
    open_type_features.stylistic_sets = Some(stylistic_sets);
    let common_style = common::TextStyle {
      open_type_features,
      ..Default::default()
    };

    let paint_style = text_style_from_common(&common_style);

    assert_eq!(
      paint_style.open_type_features,
      common_style.open_type_features
    );
    assert_ne!(
      paint_style,
      text_style_from_common(&common::TextStyle::default())
    );
  }

  #[test]
  fn rotated_text_uses_its_page_space_origin_to_select_a_table_cell() {
    let cell = rect(110.0, 90.0, 20.0, 20.0);
    let item = text_item(point(100.0, 120.0), Some(point(100.0, 100.0)), -90.0);
    let fragment = table_cell_fragment(cell, 10..11);

    assert_eq!(display_item_paint_owner_origin(&item), Some((120.0, 100.0)));
    assert_eq!(table_cell_clip_bounds(&[fragment], 2, &item), Some(cell));
  }

  #[test]
  fn rotated_text_uses_item_range_only_at_an_open_cell_edge() {
    let item = text_item(point(70.6, 138.008), Some(point(70.6, 128.05)), -90.0);
    let (paint_x, paint_y) =
      display_item_paint_owner_origin(&item).expect("text has a paint origin");
    let cell = rect(65.45, 70.85, 460.6, paint_y - 70.85);
    assert!(paint_x >= cell.origin.x.0);
    assert_eq!(paint_y, cell.origin.y.0 + cell.size.height.0);

    let outside_range = table_cell_fragment(cell, 7..8);
    assert_eq!(table_cell_clip_bounds(&[outside_range], 2, &item), None);

    let owning_range = table_cell_fragment(cell, 0..5);
    assert_eq!(
      table_cell_clip_bounds(&[owning_range], 2, &item),
      Some(cell)
    );
  }

  #[test]
  fn unrotated_physical_origin_precedes_an_imprecise_fragment_range() {
    let physical_cell = rect(0.0, 0.0, 50.0, 50.0);
    let range_cell = rect(100.0, 0.0, 50.0, 50.0);
    let fragments = [
      table_cell_fragment(physical_cell, 50..60),
      table_cell_fragment(range_cell, 0..10),
    ];
    let item = text_item(point(10.0, 10.0), None, 0.0);

    assert_eq!(
      table_cell_clip_bounds(&fragments, 3, &item),
      Some(physical_cell)
    );
  }
}
