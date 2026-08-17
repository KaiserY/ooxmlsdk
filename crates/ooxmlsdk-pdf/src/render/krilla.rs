use std::borrow::Cow;
use std::io::Cursor;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use krilla::action::{Action, LinkAction};
use krilla::annotation::{Annotation, LinkAnnotation, Target};
use krilla::blend::BlendMode;
use krilla::color::rgb;
use krilla::destination::{Destination, NamedDestination, XyzDestination};
use krilla::embed::{AssociationKind, EmbeddedFile, MimeType};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::Image;
use krilla::mask::{Mask, MaskType};
use krilla::metadata::{DateTime, Metadata};
use krilla::num::NormalizedF32;
use krilla::outline::{Outline, OutlineNode};
use krilla::page::{NumberingStyle, PageLabel, PageSettings};
use krilla::paint::{
  Fill, FillRule, LineCap, LineJoin, LinearGradient, Pattern, RadialGradient, SpreadMethod, Stop,
  Stroke, StrokeDash,
};
use krilla::surface::Surface;
use krilla::tagging::{
  Artifact, ArtifactType, BBox, ContentTag, Identifier, Node, SpanTag, TableHeaderScope, Tag,
  TagGroup, TagTree,
};
use krilla::text::{Font, Glyph, GlyphId, KrillaGlyph};
use krilla::{Data, Document};
use krilla_svg::{SurfaceExt, SvgSettings};
use kurbo::{BezPath, PathEl, flatten};
use rustc_hash::FxHashMap as HashMap;
use skrifa::{
  FontRef as SkrifaFontRef, GlyphId as SkrifaGlyphId, MetadataProvider,
  instance::{LocationRef as SkrifaLocationRef, Size as SkrifaSize},
  outline::{DrawSettings as SkrifaDrawSettings, OutlinePen as SkrifaOutlinePen},
  raw::TableProvider as SkrifaTableProvider,
  string::StringId as SkrifaStringId,
};
use smallvec::SmallVec;

use super::fonts::FontSet;
use super::form_widgets::{collect_form_widget_annotations, inject_form_widget_annotations};
use super::image::ImageSet;
use super::settings::serialize_settings;
use crate::error::{PdfError, Result};
use crate::options::{PdfAttachmentAssociation, PdfDateTime, PdfOptions};
use crate::{
  PdfConversionDiagnostics, PdfConversionOutput, PdfFontAudit, PdfFontAuditIssue,
  PdfFontAuditIssueKind, PdfFontAuditOutput, PdfFontFaceDiagnostics, PdfGlyphBoundsDiagnostics,
  PdfGlyphDiagnostics, PdfGlyphRunDiagnostics, PdfPageDiagnostics, PdfTextPortionDiagnostics,
  PdfTextPortionKind, PdfTextRunDiagnostics,
};
use ooxmlsdk_layout::fonts::{FontFaceData, FontStyleRef};
use ooxmlsdk_layout::text_metrics::TextMetrics;
use ooxmlsdk_layout::{common, units};

const INTERNAL_LINK_DESTINATION_SHIFT_PT: f32 = 10.0;
// LibreOffice's LogicalFontInstance.hxx and PDFWriterImpl use a 1/3 shear
// when the requested italic style has no physical italic face. Keep the
// synthesis angle shared by ordinary PDF text and the outline paths below.
const SYNTHETIC_ITALIC_SHEAR: f32 = 1.0 / 3.0;
// Historical fixed-output calibration retained from the DOCX numbering/font
// parity path. No matching LibreOffice source constant has been identified;
// keep the trigger and scale isolated so font-metric work can remove it
// without adding another renderer-wide conditional.
const LEGACY_ARIAL_BOLD_FONT_SIZE_PT: f32 = 11.0;
const LEGACY_ARIAL_BOLD_FONT_SIZE_TOLERANCE_PT: f32 = 0.01;
const LEGACY_ARIAL_BOLD_VERTICAL_SCALE: f32 = 1.07;
// Path gradients that cannot be represented by one PDF radial shading are
// rasterized within a bounded shape-sized surface. Match the established
// Drawing effect budget so authored shapes cannot allocate page-sized masks
// at device resolution.
const MAX_PATH_GRADIENT_RASTER_PIXELS: f32 = 250_000.0;
const MAX_PATH_GRADIENT_PIXELS_PER_POINT: f32 = 2.0;
const PATH_GRADIENT_BINARY_STEPS: usize = 10;
// Office emits DrawingML preset hatches as 16×16 image cells under a 0.375
// pattern matrix, i.e. a 6pt tile. The canonical GDI+ mask is 8×8, so each
// logical mask cell occupies 0.75pt.

type PaintTextPortionRanges = SmallVec<[(PaintTextPortionKind, Range<usize>); 2]>;
type PaintGlyphFontRuns = SmallVec<[PaintGlyphFontRun; 2]>;

pub(crate) fn render(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<Vec<u8>> {
  render_inner(document, options, RenderObservation::None).map(|output| output.pdf)
}

pub(crate) fn render_with_diagnostics(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfConversionOutput> {
  let output = render_inner(document, options, RenderObservation::Diagnostics)?;
  Ok(PdfConversionOutput {
    pdf: output.pdf,
    diagnostics: output.diagnostics,
  })
}

pub(crate) fn render_with_font_audit(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfFontAuditOutput> {
  let output = render_inner(document, options, RenderObservation::FontAudit)?;
  Ok(PdfFontAuditOutput {
    pdf: output.pdf,
    audit: output.font_audit,
  })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RenderObservation {
  None,
  FontAudit,
  Diagnostics,
}

struct RenderOutput {
  pdf: Vec<u8>,
  diagnostics: PdfConversionDiagnostics,
  font_audit: PdfFontAudit,
}

fn render_inner(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
  observation: RenderObservation,
) -> Result<RenderOutput> {
  debug_assert!(
    document
      .follows
      .iter()
      .all(|follow| follow.to_page_index < document.pages.len())
  );
  debug_assert!(document.frames.iter().all(|frame| {
    let _kind = &frame.kind;
    let _block_index = frame.block_index;
    let _split_start = frame.split_start;
    let _split_end = frame.split_end;
    let _invalidation = frame.invalidation;
    frame.page_index < document.pages.len()
      && frame.section_index == document.pages[frame.page_index].section_index
      && frame.section_page_index == document.pages[frame.page_index].section_page_index
      && frame.item_range.start <= frame.item_range.end
      && frame.column_index < 64
      && frame
        .bounds
        .is_none_or(|bounds| bounds.size.width.0 >= 0.0 && bounds.size.height.0 >= 0.0)
      && frame.lines.iter().all(|line| {
        line.item_range.start >= frame.item_range.start
          && line.item_range.end <= frame.item_range.end
          && line.item_range.start < line.item_range.end
          && line.bounds.size.width.0 >= 0.0
          && line.bounds.size.height.0 >= 0.0
          && line.bounds.origin.x.0.is_finite()
          && line.bounds.origin.y.0.is_finite()
      })
      && frame.fragments.iter().all(|fragment| {
        let _fragment_kind = fragment.kind;
        fragment.item_range.start >= frame.item_range.start
          && fragment.item_range.end <= frame.item_range.end
          && fragment.item_range.start < fragment.item_range.end
          && fragment
            .bounds
            .is_none_or(|bounds| bounds.size.width.0 >= 0.0 && bounds.size.height.0 >= 0.0)
      })
      && frame.influences.iter().all(|influence| {
        let _influence_kind = influence.kind;
        influence.count > 0
          && influence.block_index == frame.block_index
          && influence
            .bounds
            .is_none_or(|bounds| bounds.size.width.0 >= 0.0 && bounds.size.height.0 >= 0.0)
      })
  }));
  debug_assert!(document.reflow.reflow_requests.iter().all(|request| {
    document
      .frames
      .get(request.frame_index)
      .is_some_and(|frame| {
        let _reason = request.reason;
        let _scope = request.scope;
        frame_kind_name_from_common(&frame.kind) == frame_kind_from_common(request.kind)
          && frame.page_index == request.page_index
          && frame.section_page_index == request.section_page_index
          && frame.column_index == request.column_index
          && frame.split_start == request.restart
          && request.influence_count == frame.influences.len()
      })
  }));
  debug_assert!(
    document
      .reflow
      .page_invalidations
      .iter()
      .all(|invalidation| {
        document
          .frames
          .get(invalidation.first_frame_index)
          .is_some_and(|frame| {
            let _reason = invalidation.reason;
            let _scope = invalidation.scope;
            frame.page_index == invalidation.page_index
              && frame.section_page_index == invalidation.section_page_index
          })
      })
  );
  debug_assert!(document.reflow.page_replays.iter().all(|replay| {
    let _scope = replay.scope;
    replay.page_index < document.pages.len()
      && replay.item_range.start <= replay.item_range.end
      && replay.column_index < 64
      && replay.section_page_index == document.pages[replay.page_index].section_page_index
      && !replay.replacement_items.is_empty()
  }));
  debug_assert!(
    document
      .reflow
      .page_replay_applications
      .iter()
      .all(|application| {
        let _scope = application.scope;
        application.page_index < document.pages.len()
          && application.item_range.start <= application.item_range.end
          && application.column_index < 64
          && application.section_page_index
            == document.pages[application.page_index].section_page_index
          && application.replacement_count > 0
          && application.applied
      })
  );
  debug_assert!(document.reflow.backward_moves.iter().all(|move_back| {
    let _scope = move_back.scope;
    let _reason = move_back.reason;
    move_back.frame_index < document.frames.len()
      && move_back.replay_start_frame_index < document.frames.len()
      && move_back.from_page_index < document.pages.len()
      && move_back.to_page_index < document.pages.len()
      && move_back.to_page_index <= move_back.from_page_index
      && (move_back.suppressed || move_back.replayed_frames > 0)
  }));
  debug_assert!(document.reflow.layout_reruns.iter().all(|rerun| {
    let _scope = rerun.scope;
    let _reason = rerun.reason;
    rerun.page_index < document.pages.len()
      && rerun.frame_index < document.frames.len()
      && rerun.produced_pages > 0
      && rerun.produced_frames > 0
      && rerun.constraints.iter().all(|constraint| {
        let _kind = constraint.kind;
        let _scope = constraint.scope;
        constraint.content_width.0 >= 0.0
          && constraint.content_bottom.0.is_finite()
          && constraint
            .bounds
            .is_none_or(|bounds| bounds.size.width.0 >= 0.0 && bounds.size.height.0 >= 0.0)
      })
  }));
  debug_assert!(document.reflow.reflow_executions.iter().all(|execution| {
    let _action = execution.action;
    let _scope = execution.scope;
    execution.request_count > 0
      && execution.first_page_index < document.pages.len()
      && execution.backward_moves <= document.reflow.backward_moves.len()
  }));
  debug_assert!(document.reflow.restart_plan.as_ref().is_none_or(|plan| {
    document.frames.get(plan.frame_index).is_some_and(|frame| {
      let _reason = plan.reason;
      let _scope = plan.scope;
      frame.page_index == plan.page_index
        && frame.block_index == plan.block_index
        && frame.split_start == plan.cursor
    })
  }));
  let mut text_metrics = TextMetrics::new();
  let paint =
    PaintDocument::from_layout(document, &mut text_metrics, options.ui_language.as_deref());
  let diagnostics = if observation == RenderObservation::Diagnostics {
    conversion_diagnostics(&paint)
  } else {
    PdfConversionDiagnostics::default()
  };
  let font_audit = if observation == RenderObservation::FontAudit {
    conversion_font_audit(&paint)
  } else {
    PdfFontAudit::default()
  };
  let form_widget_annotations = collect_form_widget_annotations(document, &mut text_metrics);
  if options.general.pdf_ua_compliance && !form_widget_annotations.is_empty() {
    return Err(PdfError::Options(
      "PDF/UA form widgets require a tagged form API and cannot use the lopdf post-processor"
        .to_string(),
    ));
  }
  let internal_links = InternalLinkTargets::from_layout(&paint, document);
  debug_assert_eq!(paint.pages.len(), document.pages.len());
  debug_assert!(paint.pages.iter().all(|page| {
    page.width_pt >= 3.0
      && page.height_pt >= 3.0
      && page.items.iter().all(|item| match item {
        PaintItem::Text(text) => {
          text
            .source_frame_index
            .is_none_or(|index| index < document.frames.len())
            && text.source_line_index.is_none_or(|index| index < 4096)
            && text.baseline_y.is_finite()
            // Signed horizontal advance can be negative when DrawingML uses
            // its permitted negative character spacing.
            && text.width_pt.is_finite()
            && !text.portions.is_empty()
            && text.portions.iter().all(|portion| {
              match portion.kind {
                PaintTextPortionKind::Field => {}
                PaintTextPortionKind::Text
                | PaintTextPortionKind::Tab
                | PaintTextPortionKind::Link => {}
              }
              portion.baseline_y.is_finite()
                && portion.width_pt.is_finite()
                && portion.text_range.start <= portion.text_range.end
                && portion.text_range.end <= text.item.text.len()
                && portion
                  .clip
                  .as_ref()
                  .is_none_or(|clip| clip.width_pt >= 0.0 && clip.height_pt >= 0.0)
                && portion.glyphs.as_ref().is_none_or(|glyphs| {
                  glyphs
                    .iter()
                    .flat_map(|run| run.glyphs.iter())
                    // ECMA-376 Part 1, 20.1.10.74-75 permits negative
                    // DrawingML character spacing down to -4000pt. Such
                    // tracking can legitimately make a glyph advance
                    // negative; the paint invariant is finiteness.
                    .all(|glyph| glyph.x_advance.is_finite())
                })
                && portion
                  .highlight
                  .as_ref()
                  .is_none_or(|rect| rect.width_pt >= 0.0 && rect.height_pt >= 0.0)
                && portion
                  .link
                  .as_ref()
                  .is_none_or(|link| link.width_pt >= 0.0 && link.height_pt >= 0.0)
            })
        }
        PaintItem::Image(_)
        | PaintItem::Group { .. }
        | PaintItem::LinkArea(_)
        | PaintItem::Rect(_)
        | PaintItem::Line(_)
        | PaintItem::Polyline(_) => true,
      })
  }));
  let mut pdf = Document::new_with(serialize_settings(options)?);
  pdf.set_metadata(pdf_metadata(options));
  embed_attachments(&mut pdf, options)?;
  register_named_destinations(&mut pdf, document, options)?;
  let mut fonts = FontSet::new();
  let mut images = ImageSet::default();
  let tagging_enabled = options.general.tagged_pdf || options.general.pdf_ua_compliance;
  let mut tag_tree = TagTree::new().with_lang(options.canonical_document_language());

  for (page_index, page) in paint.pages.iter().enumerate() {
    let mut settings = PageSettings::from_wh(page.width_pt, page.height_pt)
      .ok_or_else(|| PdfError::Krilla("invalid page size".to_string()))?;
    if let Some(label) = page_label(document, page_index) {
      settings = settings.with_page_label(label);
    }

    let mut pdf_page = pdf.start_page_with(settings);
    let mut surface = pdf_page.surface();
    let mut link_annotations = Vec::new();
    let mut tagged_items = Vec::new();
    for (item_index, item) in page
      .items
      .iter()
      .enumerate()
      .filter(|(_, item)| paint_item_intersects_page(item, page.width_pt, page.height_pt))
    {
      let content_tag = tagging_enabled.then(|| tagged_content_tag(item)).flatten();
      let identifier = content_tag.map(|tag| surface.start_tagged(tag));
      let annotation_start = link_annotations.len();
      let draw_result = draw_paint_item(
        &mut surface,
        item,
        &mut fonts,
        &mut images,
        &internal_links,
        &mut link_annotations,
        options,
      );
      if content_tag.is_some() {
        surface.end_tagged();
      }
      draw_result?;
      if tagging_enabled && (identifier.is_some() || link_annotations.len() > annotation_start) {
        tagged_items.push(TaggedPaintRecord {
          item_index,
          identifier,
          annotation_range: annotation_start..link_annotations.len(),
        });
      }
    }
    surface.finish();
    let mut annotation_ids = Vec::with_capacity(link_annotations.len());
    for annotation in link_annotations {
      if tagging_enabled {
        annotation_ids.push(pdf_page.add_tagged_annotation(annotation));
      } else {
        pdf_page.add_annotation(annotation);
      }
    }
    if tagging_enabled {
      tag_tree.push(build_page_tag_group(
        document,
        page_index,
        page,
        tagged_items,
        &annotation_ids,
      ));
    }
  }

  if let Some(outline) = pdf_outline_for_entries(&document.outline_entries) {
    pdf.set_outline(outline);
  }
  if tagging_enabled {
    pdf.set_tag_tree(tag_tree);
  }

  let pdf = pdf
    .finish()
    .map_err(|err| PdfError::Krilla(format!("{err:?}")))?;
  let pdf = fonts.restore_office_font_metadata(pdf)?;
  let pdf = inject_form_widget_annotations(pdf, form_widget_annotations)?;
  Ok(RenderOutput {
    pdf,
    diagnostics,
    font_audit,
  })
}

#[derive(Clone, Debug)]
struct PaintDocument<'doc> {
  pages: Vec<PaintPage<'doc>>,
}

#[derive(Clone, Debug)]
struct PaintPage<'doc> {
  width_pt: f32,
  height_pt: f32,
  items: Vec<PaintItem<'doc>>,
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
  Rect(RectItem),
  Line(LineItem),
  Polyline(PolylineItem<'doc>),
}

#[derive(Clone, Debug)]
struct TextItem<'doc> {
  x_pt: f32,
  y_pt: f32,
  line_height_pt: f32,
  line_metrics_participant: bool,
  paint_clip: Option<PaintClipRect>,
  text: Cow<'doc, str>,
  style: TextStyle<'doc>,
  rotation_center_pt: Option<(f32, f32)>,
  hyperlink_url: Option<Cow<'doc, str>>,
  // Dynamic fields are uncommon, while this item is stored in a mixed page
  // enum for every text run. Keep the cold payload indirect so ordinary text
  // does not inflate every non-text enum variant or require boxing the hot
  // TextItem itself.
  dynamic_field: Option<Box<common::DynamicField<'doc>>>,
  form_widget_id: Option<u32>,
  paragraph_bidi: bool,
  word_spacing_pt: f32,
  preserve_text_portion: bool,
  decoration_span_start_x_pt: Option<f32>,
  pdf_text_segmentation: common::PdfTextSegmentation,
  source_path: Option<&'doc [usize]>,
  semantic_target_width_pt: Option<f32>,
}

#[derive(Clone, Debug)]
struct ImageItem<'doc> {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  crop: ImageCrop,
  clip_path: &'doc [common::PathCommand],
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  data: Cow<'doc, [u8]>,
  content_type: Option<Cow<'doc, str>>,
  metafile_monochrome_dib_palette_override: Option<[[u8; 3]; 2]>,
  metafile_background_color: Option<[u8; 3]>,
  metafile_external_header: Option<ooxmlsdk_layout::render::emf_wmf::WmfExternalHeader>,
  alt_text: Option<Cow<'doc, str>>,
  hyperlink_url: Option<Cow<'doc, str>>,
  semantic_metafile_text: bool,
  metafile_semantic_text_includes_raster_backdrop: bool,
  signature_line: Option<common::SignatureLineProperties<'doc>>,
  metafile_native_size: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ImageCrop {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

#[derive(Clone, Debug)]
struct LinkAreaItem<'doc> {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  hyperlink_url: Cow<'doc, str>,
}

#[derive(Clone, Copy, Debug)]
enum RectFill {
  Solid { color: RgbColor, opacity: f32 },
  Pattern(common::PatternFill),
}

#[derive(Clone, Copy, Debug)]
struct RectItem {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  fill: Option<RectFill>,
  stroke: Option<BorderStyle>,
  stroke_opacity: f32,
}

#[derive(Clone, Debug)]
struct LineItem {
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  width_pt: f32,
  color: RgbColor,
  dash: Option<Vec<f32>>,
  dash_offset: f32,
  line_cap: LineCap,
  kind: LineItemKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LineItemKind {
  Stroke,
  FilledRect,
}

#[derive(Clone, Debug)]
struct PolylineItem<'doc> {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  points: &'doc [common::Point],
  commands: &'doc [common::PathCommand],
  closed: bool,
  fill: &'doc common::Fill<'static>,
  stroke: Option<&'doc common::Stroke<'static>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BorderStyle {
  width_pt: f32,
  color: RgbColor,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RgbColor {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct TextStyle<'doc> {
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
  explicit_symbol_character: bool,
  font_size_pt: f32,
  complex_font_size_pt: Option<f32>,
  complex_script: Option<bool>,
  right_to_left: Option<bool>,
  resolved_bidi_level: Option<u8>,
  kerning_minimum_size_pt: Option<f32>,
  ligatures: Option<common::OpenTypeLigatures>,
  horizontal_scale: Option<f32>,
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
  pdf_glyph_outlines: bool,
  pdf_glyph_outline_options: Option<common::PdfGlyphOutlineOptions>,
  bold: bool,
  italic: bool,
  complex_bold: Option<bool>,
  complex_italic: Option<bool>,
  underline: bool,
  strikethrough: bool,
  uppercase: bool,
  small_caps: bool,
  hidden: bool,
  semantic_only: bool,
  /// The text origin is already the metafile playback baseline rather than a
  /// document-layout line-box origin.
  metafile_reference_baseline: bool,
  rotation_deg: f32,
  color: RgbColor,
  opacity: f32,
  outline_color: Option<RgbColor>,
  outline_opacity: f32,
  outline_width_pt: f32,
  highlight: Option<RgbColor>,
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

  fn horizontal_scale(&self) -> f32 {
    self.horizontal_scale.unwrap_or(1.0)
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
enum PaintItem<'doc> {
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
  Rect(RectItem),
  Line(LineItem),
  Polyline(PolylineItem<'doc>),
}

#[derive(Clone, Debug)]
struct PaintText<'doc> {
  item: TextItem<'doc>,
  source_frame_index: Option<usize>,
  source_line_index: Option<usize>,
  baseline_y: f32,
  width_pt: f32,
  portions: Vec<PaintTextPortion>,
}

#[derive(Clone, Debug)]
struct PaintTextPortion {
  kind: PaintTextPortionKind,
  text_range: std::ops::Range<usize>,
  x_pt: f32,
  baseline_y: f32,
  width_pt: f32,
  clip: Option<PaintClipRect>,
  glyphs: Option<PaintGlyphFontRuns>,
  highlight: Option<PaintRect>,
  underline: Option<PaintStrokeLine>,
  strikethrough: Option<PaintStrokeLine>,
  link: Option<PaintLink>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PaintTextPortionKind {
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
struct PaintGlyphFontRun {
  font_face: FontFaceData,
  font_size_pt: f32,
  x_offset_pt: f32,
  glyphs: Vec<PaintGlyph>,
}

#[derive(Clone, Debug)]
struct PaintGlyph {
  // Krilla's public Glyph trait is designed for caller-owned glyph records.
  // Keep the shaping bounds beside the exact normalized values consumed by
  // draw_glyphs instead of reconstructing them from the serialized PDF.
  glyph_id: GlyphId,
  text_range: Range<usize>,
  x_advance: f32,
  x_offset: f32,
  y_offset: f32,
  y_advance: f32,
  bounds_em: Option<PdfGlyphBoundsDiagnostics>,
}

fn conversion_diagnostics(paint: &PaintDocument<'_>) -> PdfConversionDiagnostics {
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

fn conversion_font_audit(paint: &PaintDocument<'_>) -> PdfFontAudit {
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
            if !krilla_font_loads(&run.font_face) {
              push_font_audit_issue(
                &mut audit,
                PdfFontAuditIssue {
                  page_index,
                  text_run_index,
                  portion_index: Some(portion_index),
                  glyph_run_index: Some(glyph_run_index),
                  glyph_index: None,
                  kind: PdfFontAuditIssueKind::KrillaFontLoad,
                  detail: format!("font_id={:?}", font.font_id),
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
            if font_parsed && glyph.glyph_id.to_u32() >= u32::from(font_glyph_count) {
              let mut issue = location();
              issue.kind = PdfFontAuditIssueKind::GlyphIdOutOfRange;
              issue.detail = format!(
                "font_index={font_index}, glyph_id={}, glyph_count={}",
                glyph.glyph_id.to_u32(),
                font_glyph_count
              );
              push_font_audit_issue(&mut audit, issue);
            }
            if visible
              && glyph.glyph_id.to_u32() == 0
              && source_range_requires_visible_glyph(&text.item.text, &glyph.text_range)
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

fn krilla_font_loads(face: &FontFaceData) -> bool {
  let data: Arc<dyn AsRef<[u8]> + Send + Sync> = face.data.clone();
  krilla::text::Font::new(data.into(), face.index).is_some()
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
                glyph_id: glyph.glyph_id.to_u32(),
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
  // Mirrors Krilla's FontInfo identity: face index plus OpenType `head`
  // checksum adjustment and data length distinguish faces without hashing a
  // multi-megabyte font once per glyph run.
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

impl Glyph for PaintGlyph {
  fn glyph_id(&self) -> GlyphId {
    self.glyph_id
  }

  fn text_range(&self) -> Range<usize> {
    self.text_range.clone()
  }

  fn x_advance(&self, size: f32) -> f32 {
    self.x_advance * size
  }

  fn x_offset(&self, size: f32) -> f32 {
    self.x_offset * size
  }

  fn y_offset(&self, size: f32) -> f32 {
    self.y_offset * size
  }

  fn y_advance(&self, size: f32) -> f32 {
    self.y_advance * size
  }

  fn location(&self) -> Option<krilla::surface::Location> {
    None
  }
}

#[derive(Clone, Copy, Debug)]
struct PaintRect {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
struct PaintClipRect {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
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
struct PaintStrokeLine {
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  width_pt: f32,
  color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
struct PaintLink {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct InternalLinkPosition {
  page_index: usize,
  x_pt: f32,
  y_pt: f32,
}

#[derive(Clone, Debug, Default)]
struct InternalLinkTargets {
  positions: HashMap<String, InternalLinkPosition>,
}

impl InternalLinkTargets {
  fn from_layout(paint: &PaintDocument<'_>, document: &common::LayoutDocument<'static>) -> Self {
    let mut positions = HashMap::default();
    for anchor in &document.anchor_pages {
      if anchor.name.is_empty() || anchor.page_index >= paint.pages.len() {
        continue;
      }
      positions
        .entry(format!("ooxmlsdk-pdf:bookmark:{}", anchor.name))
        .or_insert(InternalLinkPosition {
          page_index: anchor.page_index,
          x_pt: 0.0,
          y_pt: 0.0,
        });
    }
    for (page_index, page) in paint.pages.iter().enumerate() {
      for item in &page.items {
        match item {
          PaintItem::Text(text) => {
            if let Some(url) = &text.item.hyperlink_url
              && let Some(source_url) = reciprocal_internal_link_url(url)
            {
              positions.entry(source_url).or_insert(InternalLinkPosition {
                page_index,
                x_pt: text.item.x_pt,
                // position links upward by 10pt so baseline targets remain visible.
                y_pt: (text.baseline_y - INTERNAL_LINK_DESTINATION_SHIFT_PT).max(0.0),
              });
            }
          }
          PaintItem::Group { items, .. } => {
            collect_internal_link_targets(items, page_index, &mut positions);
          }
          PaintItem::Image(_)
          | PaintItem::LinkArea(_)
          | PaintItem::Rect(_)
          | PaintItem::Line(_)
          | PaintItem::Polyline(_) => {}
        }
      }
    }
    Self { positions }
  }

  fn target_for_url(&self, url: &str) -> Option<Target> {
    let position = self.positions.get(url)?;
    Some(Target::Destination(Destination::Xyz(XyzDestination::new(
      position.page_index,
      Point::from_xy(position.x_pt, position.y_pt),
    ))))
  }
}

fn collect_internal_link_targets(
  items: &[PaintItem<'_>],
  page_index: usize,
  positions: &mut HashMap<String, InternalLinkPosition>,
) {
  for item in items {
    match item {
      PaintItem::Text(text) => {
        if let Some(url) = &text.item.hyperlink_url
          && let Some(source_url) = reciprocal_internal_link_url(url)
        {
          positions.entry(source_url).or_insert(InternalLinkPosition {
            page_index,
            x_pt: text.item.x_pt,
            y_pt: (text.baseline_y - INTERNAL_LINK_DESTINATION_SHIFT_PT).max(0.0),
          });
        }
      }
      PaintItem::Group { items, .. } => {
        collect_internal_link_targets(items, page_index, positions);
      }
      PaintItem::Image(_)
      | PaintItem::LinkArea(_)
      | PaintItem::Rect(_)
      | PaintItem::Line(_)
      | PaintItem::Polyline(_) => {}
    }
  }
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

fn is_internal_link_url(url: &str) -> bool {
  url.starts_with("ooxmlsdk-pdf:")
}

fn reciprocal_internal_link_url(url: &str) -> Option<String> {
  let (kind, id) = internal_link_url_parts(url)?;
  let (note_kind, target_suffix) = if let Some(note_kind) = kind.strip_suffix("-reference") {
    (note_kind, "-backlink")
  } else {
    (kind.strip_suffix("-backlink")?, "-reference")
  };
  let mut target_url = String::with_capacity(
    "ooxmlsdk-pdf:".len() + note_kind.len() + target_suffix.len() + id.len() + 1,
  );
  target_url.push_str("ooxmlsdk-pdf:");
  target_url.push_str(note_kind);
  target_url.push_str(target_suffix);
  target_url.push(':');
  target_url.push_str(id);
  Some(target_url)
}

fn internal_link_url_parts(url: &str) -> Option<(&str, &str)> {
  let rest = url.strip_prefix("ooxmlsdk-pdf:")?;
  rest.rsplit_once(':')
}

impl<'doc> PaintDocument<'doc> {
  fn from_layout(
    document: &'doc common::LayoutDocument<'static>,
    text_metrics: &mut TextMetrics,
    ui_language: Option<&str>,
  ) -> Self {
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
            page_item_from_common(item, ui_language, text_metrics)
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

fn pdf_page_dimension(engine_kind: common::LayoutEngineKind, dimension_pt: f32) -> f32 {
  match engine_kind {
    common::LayoutEngineKind::Pptx => {
      // PowerPoint's fixed-format writer quantizes presentation MediaBox
      // dimensions to its 600 dpi print-device grid, with positive half-grid
      // dimensions rounded upward. Keep the OOXML/layout coordinate space
      // exact and apply this only at PDF page creation.
      ooxmlsdk_layout::units::quantize_points_to_office_print_grid(dimension_pt)
    }
    common::LayoutEngineKind::Docx => {
      let print_grid_position = dimension_pt * ooxmlsdk_layout::units::OFFICE_FIXED_OUTPUT_DPI
        / ooxmlsdk_layout::units::POINTS_PER_INCH;
      // Word MediaBoxes use the same 600 dpi grid, but real corpus half-grid
      // dimensions round in both directions depending on printer/page state
      // that is not represented by w:pgSz. Preserve an exact half-grid source
      // dimension instead of choosing a contradicted tie rule. Non-ties have
      // one nearest device coordinate and can be normalized safely.
      if (print_grid_position.fract() - 0.5).abs() <= 0.001 {
        dimension_pt
      } else {
        ooxmlsdk_layout::units::quantize_points_to_office_print_grid(dimension_pt)
      }
    }
    _ => dimension_pt,
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
      let paint_native_text = image.metafile_semantic_text_includes_raster_backdrop;
      let localize_signature_ui_text = image
        .signature_line
        .as_ref()
        .is_some_and(|properties| properties.state == common::SignatureLineState::Unsigned);
      let solid_rects = if paint_native_text {
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
      let bitmap_layers = if paint_native_text {
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
          metafile_monochrome_dib_palette_override: None,
          metafile_background_color: None,
          metafile_external_header: None,
          alt_text: None,
          hyperlink_url: None,
          semantic_metafile_text: false,
          metafile_semantic_text_includes_raster_backdrop: false,
          signature_line: None,
          metafile_native_size: false,
        })
      })
      .collect::<Vec<_>>();
      let semantic_runs =
        ooxmlsdk_layout::render::emf_wmf::extract_metafile_text_runs_with_options(
          &image.data,
          image.content_type.as_deref(),
          image.metafile_semantic_text_includes_raster_backdrop,
          extraction_options,
        )
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
            text: Cow::Owned(run.text),
            style: TextStyle {
              font_family: run.font_family.map(Cow::Owned),
              font_size_pt,
              bold: run.bold,
              italic: run.italic,
              semantic_only: !paint_native_text,
              metafile_reference_baseline: true,
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
        .filter_map(|item| page_item_from_common(item, ui_language, text_metrics))
        .collect(),
    }),
    common::DisplayItem::Path(path) => Some(PageItem::Polyline(polyline_from_common(path))),
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
    metafile_monochrome_dib_palette_override: image.metafile_monochrome_dib_palette_override,
    metafile_background_color: image.metafile_background_color,
    metafile_external_header: image.metafile_external_header,
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

fn polyline_from_common<'doc>(path: &'doc common::PathItem<'static>) -> PolylineItem<'doc> {
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
  }
}

fn rect_item_from_common(rect: &common::RectItem<'static>) -> RectItem {
  let fill = match &rect.fill {
    common::Fill::Solid(color) => Some(RectFill::Solid {
      color: rgb(*color),
      opacity: opacity(*color),
    }),
    common::Fill::Pattern(pattern) => Some(RectFill::Pattern(*pattern)),
    common::Fill::None
    | common::Fill::Theme(_)
    | common::Fill::Gradient(_)
    | common::Fill::Image { .. } => None,
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
    Some(common::StrokeCap::Round) => LineCap::Round,
    Some(common::StrokeCap::Square) => LineCap::Square,
    Some(common::StrokeCap::Flat) | None => LineCap::Butt,
  };
  LineItem {
    x1_pt: line.start.x.0,
    y1_pt: line.start.y.0,
    x2_pt: line.end.x.0,
    y2_pt: line.end.y.0,
    width_pt: line.stroke.width.0,
    color: rgb(line.stroke.color),
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
    complex_script: style.complex_script,
    right_to_left: style.right_to_left,
    resolved_bidi_level: style.resolved_bidi_level(),
    kerning_minimum_size_pt: style.kerning_minimum_size.map(|size| size.0),
    ligatures: style.ligatures,
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

fn frame_kind_from_common(kind: common::FrameKind) -> FollowFrameKind {
  match kind {
    common::FrameKind::Paragraph => FollowFrameKind::Paragraph,
    common::FrameKind::Table => FollowFrameKind::Table,
    common::FrameKind::Notes => FollowFrameKind::Notes,
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
    let glyphs = shaped_pdf_glyphs(
      &text_ref.text,
      &text_ref.style,
      text_ref.word_spacing_pt,
      text_metrics,
    );
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
    let highlight = text_ref.style.highlight.map(|color| PaintRect {
      x_pt: text_ref.x_pt,
      y_pt: text_box_y_pt,
      width_pt,
      height_pt: text_box_height_pt,
      color,
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
    highlight,
    underline,
    strikethrough,
    link,
  } = source;
  let ranges = visually_ordered_text_portion_ranges(text);
  let can_move_glyphs =
    glyphs.is_some() && ranges.len() == 1 && ranges[0].1 == (0..text.text.len());
  let mut glyphs = glyphs;
  let mut portions = Vec::with_capacity(ranges.len().max(1));
  let mut x_pt = text.x_pt;
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
      .map(|glyphs| glyph_runs_width_pt(glyphs))
      .unwrap_or_else(|| {
        text_metrics.measure_text(&text.text[range.start..range.end], &text.style)
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
        .map(|rect| paint_rect_for_portion(rect, x_pt, portion_width)),
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

fn glyph_runs_width_pt(glyphs: &[PaintGlyphFontRun]) -> f32 {
  glyphs
    .iter()
    .map(|run| {
      run
        .glyphs
        .iter()
        .map(|glyph| glyph.x_advance * run.font_size_pt)
        .sum::<f32>()
    })
    .sum()
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
            let item_origin = match &items[item_index] {
              common::DisplayItem::Text(text) => Some((text.origin.x.0, text.origin.y.0)),
              common::DisplayItem::Glyphs(glyphs) => Some((glyphs.origin.x.0, glyphs.origin.y.0)),
              _ => None,
            };
            frame
              .fragments
              .iter()
              .filter(|fragment| fragment.kind == common::FrameFragmentKind::TableCell)
              .filter(|fragment| {
                if let Some((x_pt, y_pt)) = item_origin {
                  // Nested table fragments are folded into their outer table
                  // frame, so a same-baseline item can outlive an imprecise
                  // flattened item range. Its page-space origin remains an
                  // unambiguous owner: prefer the smallest cell containing
                  // that origin. Cell rectangles are left/top-closed and
                  // right/bottom-open; otherwise text whose origin is exactly
                  // on an adjacent cell boundary is assigned to the earlier
                  // cell and gets clipped out of the visible PDF paint.
                  fragment.bounds.is_some_and(|bounds| {
                    x_pt + f32::EPSILON >= bounds.origin.x.0
                      && x_pt < bounds.origin.x.0 + bounds.size.width.0
                      && y_pt + f32::EPSILON >= bounds.origin.y.0
                      && y_pt < bounds.origin.y.0 + bounds.size.height.0
                  })
                } else {
                  fragment.item_range.start <= item_index && item_index < fragment.item_range.end
                }
              })
              .min_by(|left, right| {
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
              })
              .and_then(|fragment| fragment.bounds)
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

#[derive(Debug)]
struct TaggedPaintRecord {
  item_index: usize,
  identifier: Option<Identifier>,
  annotation_range: Range<usize>,
}

fn tagged_content_tag(item: &PaintItem<'_>) -> Option<ContentTag<'static>> {
  match item {
    PaintItem::Text(text) if !text.item.text.is_empty() => Some(ContentTag::Span(SpanTag::empty())),
    PaintItem::Image(image)
      if image
        .alt_text
        .as_deref()
        .is_some_and(|alt| !alt.trim().is_empty()) =>
    {
      Some(ContentTag::Other)
    }
    PaintItem::Group { .. } if paint_item_alt_text(item).is_some() => Some(ContentTag::Other),
    PaintItem::Image(_) | PaintItem::Rect(_) | PaintItem::Line(_) | PaintItem::Polyline(_) => Some(
      ContentTag::Artifact(Artifact::with_kind(ArtifactType::Layout)),
    ),
    PaintItem::Group { .. } => Some(ContentTag::Artifact(Artifact::with_kind(
      ArtifactType::Layout,
    ))),
    PaintItem::Text(_) | PaintItem::LinkArea(_) => None,
  }
}

fn paint_item_alt_text<'a>(item: &'a PaintItem<'_>) -> Option<&'a str> {
  match item {
    PaintItem::Image(image) => image
      .alt_text
      .as_deref()
      .filter(|alt| !alt.trim().is_empty()),
    PaintItem::Group { items, .. } => items.iter().find_map(paint_item_alt_text),
    PaintItem::Text(_)
    | PaintItem::LinkArea(_)
    | PaintItem::Rect(_)
    | PaintItem::Line(_)
    | PaintItem::Polyline(_) => None,
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParagraphTagKey {
  Frame(usize),
  Source(Vec<usize>),
  Loose(usize),
}

#[derive(Debug)]
struct ParagraphTagBuilder {
  key: ParagraphTagKey,
  is_note: bool,
  text: String,
  children: Vec<Node>,
}

#[derive(Debug)]
struct TableCellTagBuilder {
  cell_index: usize,
  header: bool,
  children: Vec<Node>,
}

#[derive(Debug)]
struct TableRowTagBuilder {
  row_index: usize,
  header: bool,
  cells: Vec<TableCellTagBuilder>,
}

#[derive(Debug)]
struct TableTagBuilder {
  frame_index: usize,
  rows: Vec<TableRowTagBuilder>,
}

#[derive(Debug)]
enum PageTagBlock {
  Paragraph(ParagraphTagBuilder),
  Table(TableTagBuilder),
  Node(Node),
}

fn build_page_tag_group(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
  page: &PaintPage<'_>,
  records: Vec<TaggedPaintRecord>,
  annotation_ids: &[Identifier],
) -> TagGroup {
  let mut blocks = Vec::<PageTagBlock>::new();
  for record in records {
    let Some(item) = page.items.get(record.item_index) else {
      continue;
    };
    let annotations = record
      .annotation_range
      .clone()
      .filter_map(|index| annotation_ids.get(index).cloned())
      .collect::<Vec<_>>();
    match item {
      PaintItem::Text(text) if !text.item.text.is_empty() => {
        let Some(mut node) = tagged_leaf_node(record.identifier, annotations) else {
          continue;
        };
        if text.item.style.italic {
          node = TagGroup::with_children(Tag::Em, vec![node]).into();
        }
        if text.item.style.bold {
          node = TagGroup::with_children(Tag::Strong, vec![node]).into();
        }
        if let Some(frame_index) = text.source_frame_index
          && document
            .frames
            .get(frame_index)
            .is_some_and(|frame| frame.kind == "table")
        {
          let (row_index, cell_index, header) = table_cell_position(document, text).unwrap_or((
            text.source_line_index.unwrap_or(0),
            0,
            false,
          ));
          push_table_text_node(
            &mut blocks,
            frame_index,
            row_index,
            cell_index,
            header,
            node,
          );
          continue;
        }

        let key = if let Some(frame_index) = text.source_frame_index {
          ParagraphTagKey::Frame(frame_index)
        } else if let Some(path) = text.item.source_path {
          ParagraphTagKey::Source(path.to_vec())
        } else {
          ParagraphTagKey::Loose(record.item_index)
        };
        let is_note = text
          .source_frame_index
          .and_then(|index| document.frames.get(index))
          .is_some_and(|frame| frame.kind == "notes");
        push_paragraph_text_node(&mut blocks, key, is_note, text.item.text.as_ref(), node);
      }
      PaintItem::Image(image)
        if image
          .alt_text
          .as_deref()
          .is_some_and(|alt| !alt.trim().is_empty()) =>
      {
        let Some(content) = tagged_leaf_node(record.identifier, Vec::new()) else {
          continue;
        };
        let bbox = Rect::from_xywh(image.x_pt, image.y_pt, image.width_pt, image.height_pt)
          .map(|rect| BBox::new(page_index, rect));
        let figure = Tag::Figure(image.alt_text.as_deref().map(str::to_string)).with_bbox(bbox);
        let figure: Node = TagGroup::with_children(figure, vec![content]).into();
        let node = if annotations.is_empty() {
          figure
        } else {
          let mut link_children = annotations.into_iter().map(Node::Leaf).collect::<Vec<_>>();
          link_children.push(figure);
          TagGroup::with_children(Tag::Link, link_children).into()
        };
        blocks.push(PageTagBlock::Node(node));
      }
      PaintItem::Group { .. }
        if let Some(alt_text) = paint_item_alt_text(item)
          && let Some(content) = tagged_leaf_node(record.identifier, Vec::new()) =>
      {
        let bbox = paint_item_bounds(item)
          .and_then(|(left, top, right, bottom)| {
            Rect::from_xywh(left, top, right - left, bottom - top)
          })
          .map(|rect| BBox::new(page_index, rect));
        let figure = Tag::Figure(Some(alt_text.to_string())).with_bbox(bbox);
        let figure: Node = TagGroup::with_children(figure, vec![content]).into();
        let node = if annotations.is_empty() {
          figure
        } else {
          let mut link_children = annotations.into_iter().map(Node::Leaf).collect::<Vec<_>>();
          link_children.push(figure);
          TagGroup::with_children(Tag::Link, link_children).into()
        };
        blocks.push(PageTagBlock::Node(node));
      }
      PaintItem::LinkArea(_) if !annotations.is_empty() => {
        blocks.push(PageTagBlock::Node(
          TagGroup::with_children(Tag::Link, annotations.into_iter().map(Node::Leaf).collect())
            .into(),
        ));
      }
      PaintItem::Text(_)
      | PaintItem::Image(_)
      | PaintItem::Group { .. }
      | PaintItem::Rect(_)
      | PaintItem::Line(_)
      | PaintItem::Polyline(_)
      | PaintItem::LinkArea(_) => {}
    }
  }

  let mut part = TagGroup::new(Tag::Part);
  for block in blocks {
    match block {
      PageTagBlock::Paragraph(paragraph) => {
        part.push(paragraph_tag_group(document, page_index, paragraph));
      }
      PageTagBlock::Table(table) => part.push(table_tag_group(table)),
      PageTagBlock::Node(node) => part.children.push(node),
    }
  }
  part
}

fn tagged_leaf_node(identifier: Option<Identifier>, annotations: Vec<Identifier>) -> Option<Node> {
  let identifier = identifier?;
  if annotations.is_empty() {
    Some(identifier.into())
  } else {
    let mut children = annotations.into_iter().map(Node::Leaf).collect::<Vec<_>>();
    children.push(identifier.into());
    Some(TagGroup::with_children(Tag::Link, children).into())
  }
}

fn push_paragraph_text_node(
  blocks: &mut Vec<PageTagBlock>,
  key: ParagraphTagKey,
  is_note: bool,
  text: &str,
  node: Node,
) {
  if let Some(PageTagBlock::Paragraph(paragraph)) = blocks
    .iter_mut()
    .find(|block| matches!(block, PageTagBlock::Paragraph(paragraph) if paragraph.key == key))
  {
    paragraph.text.push_str(text);
    paragraph.children.push(node);
    return;
  }
  blocks.push(PageTagBlock::Paragraph(ParagraphTagBuilder {
    key,
    is_note,
    text: text.to_string(),
    children: vec![node],
  }));
}

fn push_table_text_node(
  blocks: &mut Vec<PageTagBlock>,
  frame_index: usize,
  row_index: usize,
  cell_index: usize,
  header: bool,
  node: Node,
) {
  let table_index = blocks
    .iter()
    .position(
      |block| matches!(block, PageTagBlock::Table(table) if table.frame_index == frame_index),
    )
    .unwrap_or_else(|| {
      blocks.push(PageTagBlock::Table(TableTagBuilder {
        frame_index,
        rows: Vec::new(),
      }));
      blocks.len() - 1
    });
  let PageTagBlock::Table(table) = &mut blocks[table_index] else {
    unreachable!();
  };
  let row_index_in_table = table
    .rows
    .iter()
    .position(|row| row.row_index == row_index)
    .unwrap_or_else(|| {
      table.rows.push(TableRowTagBuilder {
        row_index,
        header,
        cells: Vec::new(),
      });
      table.rows.len() - 1
    });
  let row = &mut table.rows[row_index_in_table];
  row.header |= header;
  if let Some(cell) = row
    .cells
    .iter_mut()
    .find(|cell| cell.cell_index == cell_index)
  {
    cell.header |= header;
    cell.children.push(node);
  } else {
    row.cells.push(TableCellTagBuilder {
      cell_index,
      header,
      children: vec![node],
    });
  }
}

fn table_cell_position(
  document: &common::LayoutDocument<'static>,
  text: &PaintText<'_>,
) -> Option<(usize, usize, bool)> {
  let frame = document.frames.get(text.source_frame_index?)?;
  let line = frame.lines.get(text.source_line_index?)?;
  frame
    .fragments
    .iter()
    .filter(|fragment| fragment.kind == common::FrameFragmentKind::TableCell)
    .filter(|fragment| {
      fragment.item_range.start < line.item_range.end
        && line.item_range.start < fragment.item_range.end
    })
    .min_by_key(|fragment| fragment.item_range.end - fragment.item_range.start)
    .map(|fragment| {
      (
        fragment.row_index,
        fragment.cell_index.unwrap_or(0),
        fragment.split == common::FragmentSplitKind::RepeatedHeader,
      )
    })
}

fn paragraph_tag_group(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
  paragraph: ParagraphTagBuilder,
) -> TagGroup {
  if paragraph.is_note {
    return TagGroup::with_children(Tag::Note, paragraph.children);
  }
  let normalized = normalize_tag_text(&paragraph.text);
  if let Some(outline) = document.outline_entries.iter().find(|entry| {
    entry.page_index == page_index && normalize_tag_text(entry.text.as_ref()) == normalized
  }) {
    let level =
      NonZeroU16::new(u16::from(outline.level).saturating_add(1)).unwrap_or(NonZeroU16::MIN);
    TagGroup::with_children(
      Tag::Hn(level, Some(outline.text.to_string())),
      paragraph.children,
    )
  } else {
    TagGroup::with_children(Tag::P, paragraph.children)
  }
}

fn normalize_tag_text(text: &str) -> String {
  text.split_whitespace().collect::<String>()
}

fn table_tag_group(table: TableTagBuilder) -> TagGroup {
  let mut head = TagGroup::new(Tag::THead);
  let mut body = TagGroup::new(Tag::TBody);
  for row in table.rows {
    let mut row_group = TagGroup::new(Tag::TR);
    for cell in row.cells {
      let paragraph = TagGroup::with_children(Tag::P, cell.children);
      let cell_group = if cell.header {
        TagGroup::with_children(Tag::TH(TableHeaderScope::Column), vec![paragraph.into()])
      } else {
        TagGroup::with_children(Tag::TD, vec![paragraph.into()])
      };
      row_group.push(cell_group);
    }
    if row.header {
      head.push(row_group);
    } else {
      body.push(row_group);
    }
  }
  let mut table_group = TagGroup::new(Tag::Table);
  if !head.children.is_empty() {
    table_group.push(head);
  }
  if !body.children.is_empty() {
    table_group.push(body);
  }
  table_group
}

fn draw_paint_item(
  surface: &mut Surface<'_>,
  item: &PaintItem<'_>,
  fonts: &mut FontSet,
  images: &mut ImageSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
  options: &PdfOptions,
) -> Result<()> {
  match item {
    PaintItem::Text(text) if !text.item.text.is_empty() => {
      draw_text_item(surface, text, fonts, internal_links, link_annotations)?;
    }
    PaintItem::Text(_) => {}
    PaintItem::LinkArea(link_area) => {
      if let Some(annotation) = link_annotation_for_rect(
        link_area.x_pt,
        link_area.y_pt,
        link_area.width_pt,
        link_area.height_pt,
        &link_area.hyperlink_url,
        internal_links,
      ) {
        link_annotations.push(annotation);
      }
    }
    PaintItem::Rect(rect) => draw_rect_item(surface, rect),
    PaintItem::Image(image) => {
      let _alt_text = image.alt_text.as_deref();
      if let Some(color) = image.metafile_background_color {
        draw_metafile_host_background(surface, image, color);
      }
      if image.data.is_empty() {
        draw_missing_linked_image(surface, image);
      } else if is_svg_image(image) {
        if images
          .svg(&image.data)
          .map(|tree| draw_svg_item(surface, image, &tree))
          .is_err()
        {
          draw_missing_image(surface, image);
        }
      } else {
        let metafile_options = metafile_render_options_for_image(image, options);
        match ooxmlsdk_layout::render::emf_wmf::extract_metafile_vector_scene_with_options(
          &image.data,
          image.content_type.as_deref(),
          metafile_options,
        ) {
          Ok(Some(scene)) => draw_metafile_vector_item(surface, image, &scene),
          Ok(None) | Err(_) => match images.raster(
            &image.data,
            image.content_type.as_deref(),
            options,
            Some(metafile_options),
            image.width_pt,
            image.height_pt,
          ) {
            Ok(pdf_image) => draw_image_item(surface, image, pdf_image),
            Err(_) => draw_missing_image(surface, image),
          },
        }
      }
      if let Some(url) = image.hyperlink_url.as_deref()
        && let Some(annotation) = link_annotation_for_rect(
          image.x_pt,
          image.y_pt,
          image.width_pt,
          image.height_pt,
          url,
          internal_links,
        )
      {
        link_annotations.push(annotation);
      }
    }
    PaintItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity,
      items,
    } => {
      draw_compositing_group(
        surface,
        CompositingGroup {
          mask: mask.as_ref(),
          clip: clip.as_ref(),
          transform: transform.as_ref(),
          blend_mode: *blend_mode,
          opacity: *opacity,
          flatten_identity: *flatten_identity,
          items,
        },
        fonts,
        images,
        internal_links,
        link_annotations,
        options,
      )?;
    }
    PaintItem::Line(line) => draw_line_item(surface, line),
    PaintItem::Polyline(polyline) => draw_polyline_item(surface, polyline),
  }
  Ok(())
}

struct CompositingGroup<'borrow, 'paint> {
  mask: Option<&'borrow ImageItem<'paint>>,
  clip: Option<&'borrow PaintClipRect>,
  transform: Option<&'borrow common::Transform>,
  blend_mode: common::BlendMode,
  opacity: f32,
  flatten_identity: bool,
  items: &'borrow [PaintItem<'paint>],
}

fn draw_compositing_group(
  surface: &mut Surface<'_>,
  group: CompositingGroup<'_, '_>,
  fonts: &mut FontSet,
  images: &mut ImageSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
  options: &PdfOptions,
) -> Result<()> {
  if group.flatten_identity
    && group.mask.is_none()
    && group.transform.is_none()
    && group.blend_mode == common::BlendMode::Normal
    && (group.opacity - 1.0).abs() <= f32::EPSILON
    && (group.clip.is_some()
      || group
        .items
        .iter()
        .all(|item| !matches!(item, PaintItem::Group { .. })))
  {
    // Source-over leaf paint is associative, so an identity wrapper does not
    // need an isolated Form XObject. Keeping it flat also preserves Word's
    // fixed-output sequence for w14 character effects: raster backdrop first,
    // then ordinary PDF text in the page content stream.
    let pushed_clip = push_paint_clip(surface, group.clip);
    let mut result = Ok(());
    for item in group.items {
      if let Err(error) = draw_paint_item(
        surface,
        item,
        fonts,
        images,
        internal_links,
        link_annotations,
        options,
      ) {
        result = Err(error);
        break;
      }
    }
    if pushed_clip {
      surface.pop();
    }
    return result;
  }

  let decoded_mask = group.mask.and_then(|mask| {
    let pdf_mask_image = images
      .raster(
        &mask.data,
        mask.content_type.as_deref(),
        options,
        None,
        mask.width_pt,
        mask.height_pt,
      )
      .ok()?;
    let mut builder = surface.stream_builder();
    let mut mask_surface = builder.surface();
    draw_image_item(&mut mask_surface, mask, pdf_mask_image);
    mask_surface.finish();
    Some(Mask::new(builder.finish(), MaskType::Alpha))
  });

  let mut pushed_states = 0;
  // The clip is authored in the parent coordinate space. Establish it before
  // applying the group's child-to-parent transform so transformed drawing ink
  // cannot escape the host's printable/page-frame boundary.
  if push_paint_clip(surface, group.clip) {
    pushed_states += 1;
  }
  if let Some(transform) = group.transform {
    surface.push_transform(&Transform::from_row(
      transform.m11,
      transform.m12,
      transform.m21,
      transform.m22,
      transform.dx.0,
      transform.dy.0,
    ));
    pushed_states += 1;
  }
  if group.blend_mode != common::BlendMode::Normal {
    surface.push_blend_mode(krilla_blend_mode(group.blend_mode));
    pushed_states += 1;
  }
  if group.opacity < 1.0 {
    let opacity = NormalizedF32::new(group.opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ONE);
    surface.push_opacity(opacity);
    pushed_states += 1;
  }
  if let Some(mask) = decoded_mask {
    surface.push_mask(mask);
    pushed_states += 1;
  }
  // Keep the authored content as one transparency group. Blend mode and
  // opacity are pushed on the parent first so they apply when this group
  // XObject is composited against its backdrop, not independently to each
  // child inside the group.
  surface.push_isolated();
  pushed_states += 1;
  for item in group.items {
    draw_paint_item(
      surface,
      item,
      fonts,
      images,
      internal_links,
      link_annotations,
      options,
    )?;
  }
  for _ in 0..pushed_states {
    surface.pop();
  }
  Ok(())
}

fn krilla_blend_mode(mode: common::BlendMode) -> BlendMode {
  match mode {
    common::BlendMode::Normal => BlendMode::Normal,
    common::BlendMode::Multiply => BlendMode::Multiply,
    common::BlendMode::Screen => BlendMode::Screen,
    common::BlendMode::Darken => BlendMode::Darken,
    common::BlendMode::Lighten => BlendMode::Lighten,
    common::BlendMode::Overlay => BlendMode::Overlay,
  }
}

fn office_fixed_output_raster_pixels(points: f32, visible_fraction: f32) -> u32 {
  let print_dots =
    (points.max(0.0) / visible_fraction) * units::OFFICE_FIXED_OUTPUT_DPI / units::POINTS_PER_INCH;
  let nearest_print_dot = print_dots.round();
  let print_grid_slack = f32::EPSILON * print_dots.abs().max(1.0) * 8.0;
  // Office's raster target is allocated from an integer printer-device
  // rectangle. Recover an authored 600dpi grid point before converting to
  // the 200dpi image grid so, for example, 123 printer dots remain exactly
  // 41 pixels instead of becoming 40 through f32 representation error.
  let print_dots = if (print_dots - nearest_print_dot).abs() <= print_grid_slack {
    nearest_print_dot
  } else {
    print_dots
  };
  (print_dots * units::OFFICE_FIXED_OUTPUT_RASTER_DPI / units::OFFICE_FIXED_OUTPUT_DPI)
    .floor()
    .clamp(1.0, u32::MAX as f32) as u32
}

fn metafile_render_options_for_image(
  image: &ImageItem<'_>,
  options: &PdfOptions,
) -> ooxmlsdk_layout::render::emf_wmf::RenderOptions {
  let dpi = options
    .images
    .max_resolution_dpi
    .unwrap_or(300)
    .clamp(72, 600);
  let visible_width = (1.0 - image.crop.left - image.crop.right).max(f32::EPSILON);
  let visible_height = (1.0 - image.crop.top - image.crop.bottom).max(f32::EPSILON);
  let pixels_for_axis = |size_pt: f32, visible_fraction: f32| {
    ((size_pt.max(0.0) / visible_fraction) * dpi as f32 / 72.0)
      .ceil()
      .clamp(1.0, u32::MAX as f32) as u32
  };
  let target_size = if image.metafile_semantic_text_includes_raster_backdrop {
    // PowerPoint ActiveX previews lift WMF vectors and text into the PDF form
    // while keeping an embedded DIB at its authored sample dimensions. Let
    // the WMF viewport resolve naturally here so a 57x57 source bitmap is not
    // first enlarged into a 200-DPI control-sized raster and then resampled a
    // second time by the PDF image matrix.
    None
  } else if options.images.reduce_resolution {
    Some((
      pixels_for_axis(image.width_pt, visible_width),
      pixels_for_axis(image.height_pt, visible_height),
    ))
  } else {
    // Office fixed output rasterizes vector metafile previews at 200 DPI,
    // independently of the producer's screen-sized import bitmap. For
    // example, tdf136841.docx imports as a 76x76 bitmap in LibreOffice but
    // Word's PDF contains a 157x157 image for the 56.8pt frame. VML-hosted
    // tdf135653.docx follows the same rule and emits 214x137 pixels. Both use
    // the floor of the uncropped viewport dimensions.
    Some((
      office_fixed_output_raster_pixels(image.width_pt, visible_width),
      office_fixed_output_raster_pixels(image.height_pt, visible_height),
    ))
  };
  ooxmlsdk_layout::render::emf_wmf::RenderOptions {
    target_width_px: target_size.map(|size| size.0),
    target_height_px: target_size.map(|size| size.1),
    max_pixels: Some(dpi.saturating_mul(dpi).saturating_mul(64)),
    transparent_background: image.metafile_background_color.is_some()
      || image.metafile_semantic_text_includes_raster_backdrop
      || (image.semantic_metafile_text
        && ooxmlsdk_layout::render::emf_wmf::metafile_text_requires_raster_backdrop(
          &image.data,
          image.content_type.as_deref(),
        )),
    background_color: None,
    monochrome_dib_palette_override: image.metafile_monochrome_dib_palette_override,
    filter_high_frequency_pattern_brushes: true,
    suppress_text: image.metafile_semantic_text_includes_raster_backdrop,
    suppress_solid_pattern_rects: image.metafile_semantic_text_includes_raster_backdrop,
    suppress_bitmap_layers: image.metafile_semantic_text_includes_raster_backdrop,
    wmf_external_header: image.metafile_external_header,
  }
}

fn paint_item_intersects_page(
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

fn pdf_outline_for_entries(entries: &[common::OutlineEntry<'static>]) -> Option<Outline> {
  if entries.is_empty() {
    return None;
  }
  let mut outline = Outline::new();
  let mut index = 0;
  while index < entries.len() {
    let level = entries[index].level;
    outline.push_child(pdf_outline_node(entries, &mut index, level));
  }
  Some(outline)
}

fn pdf_outline_node(
  entries: &[common::OutlineEntry<'static>],
  index: &mut usize,
  level: u8,
) -> OutlineNode {
  let entry = &entries[*index];
  *index += 1;
  let mut node = OutlineNode::new(
    entry.text.to_string(),
    XyzDestination::new(
      entry.page_index,
      Point::from_xy(entry.target.x.0, entry.target.y.0),
    ),
  );
  while *index < entries.len() && entries[*index].level > level {
    let child_level = entries[*index].level;
    node.push_child(pdf_outline_node(entries, index, child_level));
  }
  node
}

fn draw_text_item(
  surface: &mut Surface<'_>,
  text: &PaintText<'_>,
  fonts: &mut FontSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
) -> Result<()> {
  let item = &text.item;
  let small_caps_semantic_text = word_small_caps_semantic_text(&item.text, item.style.small_caps);
  let glyph_semantic_text = symbol_font_semantic_text(
    small_caps_semantic_text.as_ref(),
    item.style.font_family.as_deref(),
  );
  for portion in &text.portions {
    let semantic_clipped = if item.style.semantic_only {
      push_paint_clip(
        surface,
        Some(&PaintClipRect {
          x_pt: -10_000.0,
          y_pt: -10_000.0,
          width_pt: 0.001,
          height_pt: 0.001,
        }),
      )
    } else {
      false
    };
    let rotated = item.style.rotation_deg.abs() > f32::EPSILON;
    if rotated {
      let (rotation_x, rotation_y) = item
        .rotation_center_pt
        .unwrap_or((portion.x_pt, portion.baseline_y));
      surface.push_transform(&Transform::from_rotate_at(
        item.style.rotation_deg,
        rotation_x,
        rotation_y,
      ));
    }
    let clipped = push_paint_clip(surface, portion.clip.as_ref());
    if let Some(highlight) = &portion.highlight {
      draw_paint_rect(surface, highlight);
    }
    let vertical_scale = if item.text.contains('\t') {
      1.0
    } else {
      text_vertical_scale(&item.style)
    };
    let text_warp = item
      .style
      .pdf_glyph_outline_options
      .as_ref()
      .and_then(|options| options.text_warp.as_deref());
    let glyph_fill = item
      .style
      .pdf_glyph_outline_options
      .as_ref()
      .and_then(|options| options.fill.as_ref())
      .and_then(|fill| {
        text_outline_fill(
          surface,
          fill,
          portion.x_pt,
          item.y_pt,
          portion.width_pt.max(item.style.font_size_pt),
          item.line_height_pt.max(item.style.font_size_pt),
        )
      });
    let glyph_outline_fill = item
      .style
      .pdf_glyph_outline_options
      .as_ref()
      .and_then(|options| options.outline_fill.as_ref())
      .and_then(|fill| {
        text_outline_fill(
          surface,
          fill,
          portion.x_pt,
          item.y_pt,
          portion.width_pt.max(item.style.font_size_pt),
          item.line_height_pt.max(item.style.font_size_pt),
        )
      });
    let glyph_outline_stroke = item
      .style
      .pdf_glyph_outline_options
      .as_ref()
      .and_then(|options| options.outline_stroke.as_ref())
      .map(|stroke| {
        text_stroke_from_common(
          surface,
          stroke,
          portion.x_pt,
          item.y_pt,
          portion.width_pt.max(item.style.font_size_pt),
          item.line_height_pt.max(item.style.font_size_pt),
        )
      });
    surface.set_fill(glyph_fill.clone().or_else(|| Some(fill(&item.style))));
    if text_warp.is_none() && (vertical_scale - 1.0).abs() > f32::EPSILON {
      surface.push_transform(&Transform::from_row(
        1.0,
        0.0,
        0.0,
        vertical_scale,
        0.0,
        portion.baseline_y * (1.0 - vertical_scale),
      ));
    }
    let horizontal_scale = item.style.horizontal_scale.unwrap_or(1.0);
    if text_warp.is_none() && (horizontal_scale - 1.0).abs() > f32::EPSILON {
      surface.push_transform(&Transform::from_row(
        horizontal_scale,
        0.0,
        0.0,
        1.0,
        portion.x_pt * (1.0 - horizontal_scale),
        0.0,
      ));
    }
    // Tabs are layout controls: their measured advance positions subsequent
    // portions, but emitting the font's glyph 0 leaks a NUL into PDF text.
    if !matches!(portion.kind, PaintTextPortionKind::Tab)
      && text_has_visible_glyph_paint(&item.style)
      && let Some(glyphs) = &portion.glyphs
    {
      for run in glyphs {
        if item.style.explicit_symbol_character {
          for glyph in &run.glyphs {
            if glyph.glyph_id.to_u32() == 0
              && let Some(semantic) = glyph_semantic_text.get(glyph.text_range.clone())
            {
              fonts.record_explicit_notdef_semantic(&run.font_face, semantic);
            }
          }
        }
        let selected = fonts.select_face(&run.font_face)?;
        let glyph_outlines = text_requires_glyph_outlines(&item.style);
        surface.set_stroke(text_stroke_with_fill(
          &item.style,
          selected.synthetic_bold,
          run.font_size_pt,
          glyph_outline_stroke.clone(),
          glyph_outline_fill.clone(),
        ));
        let warped = glyph_outlines
          && text_warp.is_some_and(|warp| {
            draw_warped_glyphs(
              surface,
              warp,
              item
                .style
                .pdf_glyph_outline_options
                .as_ref()
                .and_then(|options| options.fill.as_ref()),
              &run.font_face,
              &run.glyphs,
              WarpedGlyphPlacement {
                start_x: portion.x_pt + run.x_offset_pt * horizontal_scale,
                baseline_y: portion.baseline_y,
                font_size_pt: run.font_size_pt,
                horizontal_scale,
              },
            )
          });
        let path_gradient_rasterized = glyph_outlines
          && !warped
          && item
            .style
            .pdf_glyph_outline_options
            .as_ref()
            .and_then(|options| options.fill.as_ref())
            .is_some_and(|fill| {
              draw_unwarped_path_gradient_glyphs(
                surface,
                fill,
                &run.font_face,
                &run.glyphs,
                UnwarpedGlyphPlacement {
                  start_x: portion.x_pt + run.x_offset_pt,
                  baseline_y: portion.baseline_y,
                  font_size_pt: run.font_size_pt,
                },
                TextPaintFrame {
                  x_pt: portion.x_pt,
                  y_pt: item.y_pt,
                  width_pt: portion.width_pt.max(item.style.font_size_pt),
                  height_pt: item.line_height_pt.max(item.style.font_size_pt),
                },
              )
            });
        if glyph_outlines
          && !warped
          && let Some(transform) = item
            .style
            .pdf_glyph_outline_options
            .as_ref()
            .and_then(|options| options.transform)
        {
          surface.push_transform(&Transform::from_row(
            transform.m11,
            transform.m12,
            transform.m21,
            transform.m22,
            transform.dx.0,
            transform.dy.0,
          ));
        }
        if !warped && !path_gradient_rasterized {
          draw_glyphs_with_synthetic_italic(
            surface,
            Point::from_xy(portion.x_pt + run.x_offset_pt, portion.baseline_y),
            &run.glyphs,
            selected.font.clone(),
            &glyph_semantic_text,
            run.font_size_pt,
            glyph_outlines,
            run.font_face.synthetic_italic,
          );
        }
        if path_gradient_rasterized
          && text_stroke_with_fill(
            &item.style,
            selected.synthetic_bold,
            run.font_size_pt,
            glyph_outline_stroke.clone(),
            glyph_outline_fill.clone(),
          )
          .is_some()
        {
          surface.set_fill(None);
          draw_glyphs_with_synthetic_italic(
            surface,
            Point::from_xy(portion.x_pt + run.x_offset_pt, portion.baseline_y),
            &run.glyphs,
            selected.font.clone(),
            &glyph_semantic_text,
            run.font_size_pt,
            true,
            run.font_face.synthetic_italic,
          );
          surface.set_fill(glyph_fill.clone().or_else(|| Some(fill(&item.style))));
        }
        if glyph_outlines
          && !warped
          && item
            .style
            .pdf_glyph_outline_options
            .as_ref()
            .is_some_and(|options| options.transform.is_some())
        {
          surface.pop();
        }
        if glyph_outlines
          && item
            .style
            .pdf_glyph_outline_options
            .as_ref()
            .is_some_and(|options| options.semantic_text_overlay)
        {
          // Some explicitly outlined DrawingML text retains a separate
          // invisible search/accessibility layer. Warped WordArt disables
          // this option because Office's fixed output contains outlines only.
          surface.set_fill(Some(Fill {
            paint: rgb::Color::new(0, 0, 0).into(),
            opacity: NormalizedF32::ZERO,
            rule: Default::default(),
          }));
          surface.set_stroke(None);
          draw_glyphs_with_synthetic_italic(
            surface,
            Point::from_xy(portion.x_pt + run.x_offset_pt, portion.baseline_y),
            &run.glyphs,
            selected.font,
            &glyph_semantic_text,
            run.font_size_pt,
            false,
            run.font_face.synthetic_italic,
          );
          surface.set_fill(Some(fill(&item.style)));
        }
      }
    }
    if text_warp.is_none() && (horizontal_scale - 1.0).abs() > f32::EPSILON {
      surface.pop();
    }
    if text_warp.is_none() && (vertical_scale - 1.0).abs() > f32::EPSILON {
      surface.pop();
    }
    if let Some(underline) = &portion.underline {
      draw_paint_stroke_line(surface, underline);
    }
    if let Some(strikethrough) = &portion.strikethrough {
      draw_paint_stroke_line(surface, strikethrough);
    }
    if let (Some(link), Some(url)) = (&portion.link, item.hyperlink_url.as_ref())
      && let Some(annotation) = link_annotation_for_rect(
        link.x_pt,
        link.y_pt,
        link.width_pt,
        link.height_pt,
        url,
        internal_links,
      )
    {
      link_annotations.push(annotation);
    }
    if clipped {
      surface.pop();
    }
    if rotated {
      surface.pop();
    }
    if semantic_clipped {
      surface.pop();
    }
  }
  Ok(())
}

fn synthetic_italic_text_transform(synthetic_italic: bool, baseline_y: f32) -> Option<Transform> {
  synthetic_italic.then(|| {
    // Krilla's layout coordinates point down the page. A negative x/y shear
    // therefore becomes the positive 1/3 glyph-space shear emitted by
    // PDFWriterImpl after Krilla applies its page-axis inversion. Translate by
    // the baseline so the run origin and all horizontal advances stay fixed.
    Transform::from_row(
      1.0,
      0.0,
      -SYNTHETIC_ITALIC_SHEAR,
      1.0,
      baseline_y * SYNTHETIC_ITALIC_SHEAR,
      0.0,
    )
  })
}

#[allow(clippy::too_many_arguments)]
fn draw_glyphs_with_synthetic_italic(
  surface: &mut Surface<'_>,
  start: Point,
  glyphs: &[PaintGlyph],
  font: Font,
  semantic_text: &str,
  font_size_pt: f32,
  outlined: bool,
  synthetic_italic: bool,
) {
  let transform = synthetic_italic_text_transform(synthetic_italic, start.y);
  if let Some(transform) = transform {
    surface.push_transform(&transform);
  }
  surface.draw_glyphs(start, glyphs, font, semantic_text, font_size_pt, outlined);
  if transform.is_some() {
    surface.pop();
  }
}

struct WarpedGlyphPlacement {
  start_x: f32,
  baseline_y: f32,
  font_size_pt: f32,
  horizontal_scale: f32,
}

struct UnwarpedGlyphPlacement {
  start_x: f32,
  baseline_y: f32,
  font_size_pt: f32,
}

#[derive(Clone, Copy)]
struct TextPaintFrame {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

fn draw_unwarped_path_gradient_glyphs(
  surface: &mut Surface<'_>,
  fill: &common::Fill<'static>,
  face_data: &FontFaceData,
  glyphs: &[PaintGlyph],
  placement: UnwarpedGlyphPlacement,
  frame: TextPaintFrame,
) -> bool {
  let resolved_fill = resolved_text_outline_common_fill(
    fill,
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
  );
  let common::Fill::Gradient(gradient) = &resolved_fill else {
    return false;
  };
  let Some(path_gradient) = gradient.path else {
    return false;
  };
  if path_gradient_paint(gradient, path_gradient).is_some() {
    return false;
  }
  let Ok(face) = SkrifaFontRef::from_index(face_data.data.as_ref(), face_data.index) else {
    return false;
  };
  let Ok(head) = face.head() else {
    return false;
  };
  let units_per_em = f32::from(head.units_per_em());
  if units_per_em <= f32::EPSILON {
    return false;
  }
  let scale = placement.font_size_pt / units_per_em;
  let mut path = PathBuilder::new();
  let mut commands = Vec::new();
  let mut cursor_x = placement.start_x;
  for glyph in glyphs {
    let origin_x = cursor_x + glyph.x_offset * placement.font_size_pt;
    let origin_y = placement.baseline_y - glyph.y_offset * placement.font_size_pt;
    let mut outline = KrillaGlyphOutline {
      path: &mut path,
      commands: &mut commands,
      origin_x,
      origin_y,
      scale,
      synthetic_italic: face_data.synthetic_italic,
      current: None,
    };
    if let Some(glyph_outline) = face
      .outline_glyphs()
      .get(SkrifaGlyphId::new(glyph.glyph_id.to_u32()))
    {
      let _ = glyph_outline.draw(
        SkrifaDrawSettings::unhinted(SkrifaSize::unscaled(), SkrifaLocationRef::default()),
        &mut outline,
      );
    }
    cursor_x += glyph.x_advance * placement.font_size_pt;
  }
  let Some(path) = path.finish() else {
    return false;
  };
  let gradient_frame = PolylineItem {
    x_pt: frame.x_pt,
    y_pt: frame.y_pt,
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    points: &[],
    commands: &commands,
    closed: true,
    fill: &resolved_fill,
    stroke: None,
  };
  draw_path_gradient_raster(surface, &path, &gradient_frame)
}

struct KrillaGlyphOutline<'a> {
  path: &'a mut PathBuilder,
  commands: &'a mut Vec<common::PathCommand>,
  origin_x: f32,
  origin_y: f32,
  scale: f32,
  synthetic_italic: bool,
  current: Option<common::Point>,
}

impl KrillaGlyphOutline<'_> {
  fn point(&self, x: f32, y: f32) -> common::Point {
    let x = if self.synthetic_italic {
      x + y * SYNTHETIC_ITALIC_SHEAR
    } else {
      x
    };
    common::Point {
      x: common::Pt(self.origin_x + x * self.scale),
      y: common::Pt(self.origin_y - y * self.scale),
    }
  }
}

impl SkrifaOutlinePen for KrillaGlyphOutline<'_> {
  fn move_to(&mut self, x: f32, y: f32) {
    let point = self.point(x, y);
    self.path.move_to(point.x.0, point.y.0);
    self.commands.push(common::PathCommand::MoveTo(point));
    self.current = Some(point);
  }

  fn line_to(&mut self, x: f32, y: f32) {
    let point = self.point(x, y);
    self.path.line_to(point.x.0, point.y.0);
    self.commands.push(common::PathCommand::LineTo(point));
    self.current = Some(point);
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    let control = self.point(x1, y1);
    let end = self.point(x, y);
    self
      .path
      .quad_to(control.x.0, control.y.0, end.x.0, end.y.0);
    if let Some(start) = self.current {
      let control1 = common::Point {
        x: common::Pt(start.x.0 + (control.x.0 - start.x.0) * (2.0 / 3.0)),
        y: common::Pt(start.y.0 + (control.y.0 - start.y.0) * (2.0 / 3.0)),
      };
      let control2 = common::Point {
        x: common::Pt(end.x.0 + (control.x.0 - end.x.0) * (2.0 / 3.0)),
        y: common::Pt(end.y.0 + (control.y.0 - end.y.0) * (2.0 / 3.0)),
      };
      self.commands.push(common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      });
    }
    self.current = Some(end);
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    let control1 = self.point(x1, y1);
    let control2 = self.point(x2, y2);
    let end = self.point(x, y);
    self.path.cubic_to(
      control1.x.0,
      control1.y.0,
      control2.x.0,
      control2.y.0,
      end.x.0,
      end.y.0,
    );
    self.commands.push(common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    });
    self.current = Some(end);
  }

  fn close(&mut self) {
    self.path.close();
    self.commands.push(common::PathCommand::Close);
    self.current = None;
  }
}

fn draw_warped_glyphs(
  surface: &mut Surface<'_>,
  warp: &common::TextWarp,
  outline_fill: Option<&common::Fill<'static>>,
  face_data: &FontFaceData,
  glyphs: &[PaintGlyph],
  placement: WarpedGlyphPlacement,
) -> bool {
  let Ok(face) = SkrifaFontRef::from_index(face_data.data.as_ref(), face_data.index) else {
    return false;
  };
  let Ok(head) = face.head() else {
    return false;
  };
  let units_per_em = f32::from(head.units_per_em());
  if units_per_em <= f32::EPSILON {
    return false;
  }
  let boundaries = warp
    .boundaries
    .iter()
    .filter_map(|commands| flatten_text_warp_boundary(commands))
    .collect::<Vec<_>>();
  if boundaries.is_empty() {
    return false;
  }
  let paint_bounds = warp.paint_bounds;
  let target_fill = outline_fill.and_then(|fill| {
    (paint_bounds.size.width.0 > f32::EPSILON && paint_bounds.size.height.0 > f32::EPSILON).then(
      || {
        text_outline_fill(
          surface,
          fill,
          paint_bounds.origin.x.0,
          paint_bounds.origin.y.0,
          paint_bounds.size.width.0,
          paint_bounds.size.height.0,
        )
      },
    )
  });
  if let Some(fill) = target_fill.flatten() {
    // WordArt brushes are defined over the authored warp envelope, not each
    // shaped line portion. Office reuses the same radial/path brush matrix for
    // every paragraph clipped into that envelope.
    surface.set_fill(Some(fill));
  }

  let mut cursor_x = placement.start_x;
  let glyph_scale = placement.font_size_pt / units_per_em;
  let raster_path_gradient_fill = outline_fill
    .filter(|fill| {
      matches!(
        fill,
        common::Fill::Gradient(common::GradientFill {
          path: Some(common::GradientPath {
            kind: common::GradientPathKind::Rectangle | common::GradientPathKind::Shape,
            ..
          }),
          ..
        })
      )
    })
    .and_then(|fill| {
      (paint_bounds.size.width.0 > f32::EPSILON && paint_bounds.size.height.0 > f32::EPSILON).then(
        || {
          resolved_text_outline_common_fill(
            fill,
            paint_bounds.origin.x.0,
            paint_bounds.origin.y.0,
            paint_bounds.size.width.0,
            paint_bounds.size.height.0,
          )
        },
      )
    });
  let raster_path_gradient = raster_path_gradient_fill.is_some();
  let mut warped_paths = Vec::new();
  let mut combined_path = raster_path_gradient.then(PathBuilder::new);
  let mut combined_commands = Vec::new();
  for glyph in glyphs {
    let mut outline = SkrifaGlyphOutline::default();
    let Some(glyph_outline) = face
      .outline_glyphs()
      .get(SkrifaGlyphId::new(glyph.glyph_id.to_u32()))
    else {
      cursor_x += glyph.x_advance * placement.font_size_pt * placement.horizontal_scale;
      continue;
    };
    if glyph_outline
      .draw(
        SkrifaDrawSettings::unhinted(SkrifaSize::unscaled(), SkrifaLocationRef::default()),
        &mut outline,
      )
      .is_err()
    {
      cursor_x += glyph.x_advance * placement.font_size_pt * placement.horizontal_scale;
      continue;
    }
    let origin_x = cursor_x + glyph.x_offset * placement.font_size_pt * placement.horizontal_scale;
    let origin_y = placement.baseline_y - glyph.y_offset * placement.font_size_pt;
    let elements = outline.path.into_elements().into_iter().map(|element| {
      let point = |point: kurbo::Point| {
        // LibreOffice's PDF writer uses a 1/3 horizontal shear for an
        // artificial italic face. Apply it in glyph space before the
        // non-linear WordArt mapping so the authored envelope also bends the
        // synthesized slant.
        let synthetic_italic_x = if face_data.synthetic_italic {
          point.x + point.y * f64::from(SYNTHETIC_ITALIC_SHEAR)
        } else {
          point.x
        };
        kurbo::Point::new(
          f64::from(origin_x)
            + synthetic_italic_x * f64::from(glyph_scale * placement.horizontal_scale),
          f64::from(origin_y) - point.y * f64::from(glyph_scale),
        )
      };
      match element {
        PathEl::MoveTo(value) => PathEl::MoveTo(point(value)),
        PathEl::LineTo(value) => PathEl::LineTo(point(value)),
        PathEl::QuadTo(control, end) => PathEl::QuadTo(point(control), point(end)),
        PathEl::CurveTo(control1, control2, end) => {
          PathEl::CurveTo(point(control1), point(control2), point(end))
        }
        PathEl::ClosePath => PathEl::ClosePath,
      }
    });
    let mut path = PathBuilder::new();
    let mut current = None;
    for element in elements {
      match element {
        PathEl::MoveTo(point) => {
          let point = text_warp_point(warp, &boundaries, point);
          path.move_to(point.x as f32, point.y as f32);
          current = Some(point);
          if let Some(combined) = &mut combined_path {
            combined.move_to(point.x as f32, point.y as f32);
            combined_commands.push(common::PathCommand::MoveTo(common::Point {
              x: common::Pt(point.x as f32),
              y: common::Pt(point.y as f32),
            }));
          }
        }
        PathEl::LineTo(point) => {
          let point = text_warp_point(warp, &boundaries, point);
          path.line_to(point.x as f32, point.y as f32);
          current = Some(point);
          if let Some(combined) = &mut combined_path {
            combined.line_to(point.x as f32, point.y as f32);
            combined_commands.push(common::PathCommand::LineTo(common::Point {
              x: common::Pt(point.x as f32),
              y: common::Pt(point.y as f32),
            }));
          }
        }
        PathEl::QuadTo(control, end) => {
          let control = text_warp_point(warp, &boundaries, control);
          let end = text_warp_point(warp, &boundaries, end);
          path.quad_to(
            control.x as f32,
            control.y as f32,
            end.x as f32,
            end.y as f32,
          );
          if let Some(combined) = &mut combined_path {
            combined.quad_to(
              control.x as f32,
              control.y as f32,
              end.x as f32,
              end.y as f32,
            );
            if let Some(start) = current {
              let control1 = start + (control - start) * (2.0 / 3.0);
              let control2 = end + (control - end) * (2.0 / 3.0);
              combined_commands.push(common::PathCommand::CubicTo {
                control1: common::Point {
                  x: common::Pt(control1.x as f32),
                  y: common::Pt(control1.y as f32),
                },
                control2: common::Point {
                  x: common::Pt(control2.x as f32),
                  y: common::Pt(control2.y as f32),
                },
                end: common::Point {
                  x: common::Pt(end.x as f32),
                  y: common::Pt(end.y as f32),
                },
              });
            }
          }
          current = Some(end);
        }
        PathEl::CurveTo(control1, control2, end) => {
          let control1 = text_warp_point(warp, &boundaries, control1);
          let control2 = text_warp_point(warp, &boundaries, control2);
          let end = text_warp_point(warp, &boundaries, end);
          path.cubic_to(
            control1.x as f32,
            control1.y as f32,
            control2.x as f32,
            control2.y as f32,
            end.x as f32,
            end.y as f32,
          );
          if let Some(combined) = &mut combined_path {
            combined.cubic_to(
              control1.x as f32,
              control1.y as f32,
              control2.x as f32,
              control2.y as f32,
              end.x as f32,
              end.y as f32,
            );
            combined_commands.push(common::PathCommand::CubicTo {
              control1: common::Point {
                x: common::Pt(control1.x as f32),
                y: common::Pt(control1.y as f32),
              },
              control2: common::Point {
                x: common::Pt(control2.x as f32),
                y: common::Pt(control2.y as f32),
              },
              end: common::Point {
                x: common::Pt(end.x as f32),
                y: common::Pt(end.y as f32),
              },
            });
          }
          current = Some(end);
        }
        PathEl::ClosePath => {
          path.close();
          current = None;
          if let Some(combined) = &mut combined_path {
            combined.close();
            combined_commands.push(common::PathCommand::Close);
          }
        }
      }
    }
    if let Some(path) = path.finish() {
      warped_paths.push(path);
    }
    cursor_x += glyph.x_advance * placement.font_size_pt * placement.horizontal_scale;
  }
  if let (Some(fill), Some(path), Some(bounds)) = (
    raster_path_gradient_fill.as_ref(),
    combined_path.and_then(PathBuilder::finish),
    raster_path_gradient_fill
      .as_ref()
      .and_then(|fill| match fill {
        common::Fill::Gradient(gradient) => gradient.definition_bounds,
        _ => None,
      }),
  ) {
    let gradient_frame = PolylineItem {
      x_pt: bounds.origin.x.0,
      y_pt: bounds.origin.y.0,
      width_pt: bounds.size.width.0,
      height_pt: bounds.size.height.0,
      points: &[],
      commands: &combined_commands,
      closed: true,
      fill,
      stroke: None,
    };
    if draw_path_gradient_raster(surface, &path, &gradient_frame) {
      return true;
    }
  }
  for path in &warped_paths {
    surface.draw_path(path);
  }
  !warped_paths.is_empty()
}

#[derive(Default)]
struct SkrifaGlyphOutline {
  path: BezPath,
}

impl SkrifaOutlinePen for SkrifaGlyphOutline {
  fn move_to(&mut self, x: f32, y: f32) {
    self.path.move_to((f64::from(x), f64::from(y)));
  }

  fn line_to(&mut self, x: f32, y: f32) {
    self.path.line_to((f64::from(x), f64::from(y)));
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    self
      .path
      .quad_to((f64::from(x1), f64::from(y1)), (f64::from(x), f64::from(y)));
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    self.path.curve_to(
      (f64::from(x1), f64::from(y1)),
      (f64::from(x2), f64::from(y2)),
      (f64::from(x), f64::from(y)),
    );
  }

  fn close(&mut self) {
    self.path.close_path();
  }
}

fn flatten_text_warp_boundary(commands: &[common::PathCommand]) -> Option<Vec<kurbo::Point>> {
  let elements = commands.iter().map(|command| match *command {
    common::PathCommand::MoveTo(point) => {
      PathEl::MoveTo(kurbo::Point::new(f64::from(point.x), f64::from(point.y)))
    }
    common::PathCommand::LineTo(point) => {
      PathEl::LineTo(kurbo::Point::new(f64::from(point.x), f64::from(point.y)))
    }
    common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    } => PathEl::CurveTo(
      kurbo::Point::new(f64::from(control1.x), f64::from(control1.y)),
      kurbo::Point::new(f64::from(control2.x), f64::from(control2.y)),
      kurbo::Point::new(f64::from(end.x), f64::from(end.y)),
    ),
    common::PathCommand::Close => PathEl::ClosePath,
  });
  let mut points = Vec::new();
  flatten(elements, 0.2, |element| match element {
    PathEl::MoveTo(point) | PathEl::LineTo(point) => points.push(point),
    PathEl::ClosePath => {}
    PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
      unreachable!("kurbo::flatten only emits line path elements")
    }
  });
  (points.len() >= 2).then_some(points)
}

fn text_warp_point(
  warp: &common::TextWarp,
  boundaries: &[Vec<kurbo::Point>],
  point: kurbo::Point,
) -> kurbo::Point {
  let source = warp.source_bounds;
  let width = f64::from(source.size.width).max(f64::EPSILON);
  let height = f64::from(source.size.height).max(f64::EPSILON);
  let u = ((point.x - f64::from(source.origin.x)) / width).clamp(0.0, 1.0);
  let v = ((point.y - f64::from(source.origin.y)) / height).clamp(0.0, 1.0);
  if boundaries.len() >= 2 {
    // Presets such as textButton (3 paths), textDeflateInflate (4), and
    // textButtonPour (6) define intermediate deformation grid lines. Map the
    // source height piecewise between each adjacent authored pair; using only
    // the outer paths would erase the defining inner bulges and pinches.
    let grid_position = v * (boundaries.len() - 1) as f64;
    let upper_index = (grid_position.floor() as usize).min(boundaries.len() - 2);
    let local_v = (grid_position - upper_index as f64).clamp(0.0, 1.0);
    let upper = sample_text_warp_boundary(&boundaries[upper_index], u);
    let lower = sample_text_warp_boundary(&boundaries[upper_index + 1], u);
    return upper + (lower - upper) * local_v;
  }

  // Curve-only presets (arch, circle, and curve) define a centerline
  // rather than two envelope edges. Offset shaped outlines along its local
  // normal so glyph proportions are retained while their baseline follows the
  // authored path.
  let upper = sample_text_warp_boundary(&boundaries[0], u);
  let before = sample_text_warp_boundary(&boundaries[0], (u - 0.001).max(0.0));
  let after = sample_text_warp_boundary(&boundaries[0], (u + 0.001).min(1.0));
  let tangent = after - before;
  let length = tangent.hypot();
  if length <= f64::EPSILON {
    return upper;
  }
  let normal = kurbo::Vec2::new(-tangent.y / length, tangent.x / length);
  upper + normal * ((v - 0.5) * height)
}

fn sample_text_warp_boundary(points: &[kurbo::Point], position: f64) -> kurbo::Point {
  let total = points
    .windows(2)
    .map(|segment| segment[0].distance(segment[1]))
    .sum::<f64>();
  if total <= f64::EPSILON {
    return points[0];
  }
  let target = position.clamp(0.0, 1.0) * total;
  let mut traversed = 0.0;
  for segment in points.windows(2) {
    let length = segment[0].distance(segment[1]);
    if traversed + length >= target && length > f64::EPSILON {
      let local = (target - traversed) / length;
      return segment[0] + (segment[1] - segment[0]) * local;
    }
    traversed += length;
  }
  *points.last().expect("text warp boundary is non-empty")
}

fn text_has_visible_glyph_paint(style: &TextStyle<'_>) -> bool {
  style.semantic_only
    || style.opacity > f32::EPSILON
    || (style.outline_color.is_some()
      && style.outline_width_pt > f32::EPSILON
      && style.outline_opacity > f32::EPSILON)
}

fn text_requires_glyph_outlines(style: &TextStyle<'_>) -> bool {
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

fn word_small_caps_semantic_text(text: &str, small_caps: bool) -> Cow<'_, str> {
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

  // ECMA-376 Part 1 §17.3.2.33 keeps the OOXML Unicode unchanged while
  // displaying lowercase letters as smaller capitals. Word's fixed-format
  // writer exposes those displayed capitals through PDF ToUnicode. Preserve
  // the source text in layout and shaping, and change only the PDF semantic
  // mapping after glyph selection.
  Cow::Owned(uppercase)
}

fn symbol_font_semantic_text<'a>(text: &'a str, font_family: Option<&str>) -> Cow<'a, str> {
  let symbol = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Symbol") || family.eq_ignore_ascii_case("SymbolMT")
  });
  let wingdings = font_family.is_some_and(|family| family.eq_ignore_ascii_case("Wingdings"));
  if !(symbol
    && (text.contains('\u{f02d}') || text.contains('\u{f05e}') || text.contains('\u{f0b7}'))
    || wingdings
      && (text.contains('\u{f04a}')
        || text.contains('\u{f06c}')
        || text.contains('\u{f06e}')
        || text.contains('\u{f071}')
        || text.contains('\u{f075}')
        || text.contains('\u{f076}')
        || text.contains('\u{f0a7}')
        || text.contains('\u{f0d8}')
        || text.contains('\u{f0e0}')
        || text.contains('\u{f020}')
        || text.contains('\u{f0fc}')))
  {
    return Cow::Borrowed(text);
  }

  // Keep the legacy symbol-font glyph selected by the shaped run, but expose
  // its standardized character through the PDF ToUnicode map. Unicode WG2
  // N4363 maps Wingdings character 108 to U+26AB; LibreOffice's Microsoft
  // symbol conversion tables map character 0xD8 to U+27A2 and Symbol 0xB7
  // to U+2022. PowerPoint's PDF export maps Wingdings 0x6E to U+25FC,
  // 0x76 to U+2756, 0xA7 to U+25AA, and 0xE0 to U+2192.
  // Word's w:sym fixed output keeps the declared Wingdings glyph but maps
  // 0x20 to a space-equivalent en space and 0xFC to U+2713 in ToUnicode.
  // Keeping a three-byte scalar preserves shaped cluster byte offsets.
  Cow::Owned(
    text
      .chars()
      .map(|character| match character {
        '\u{f02d}' if symbol => '\u{2212}',
        '\u{f05e}' if symbol => '\u{22a5}',
        '\u{f0b7}' if symbol => '\u{2022}',
        '\u{f04a}' if wingdings => '\u{263a}',
        '\u{f06c}' if wingdings => '\u{26ab}',
        '\u{f06e}' if wingdings => '\u{25fc}',
        '\u{f071}' if wingdings => '\u{2751}',
        '\u{f075}' if wingdings => '\u{25c6}',
        '\u{f076}' if wingdings => '\u{2756}',
        '\u{f0a7}' if wingdings => '\u{25aa}',
        '\u{f0d8}' if wingdings => '\u{27a2}',
        '\u{f0e0}' if wingdings => '\u{2192}',
        '\u{f020}' if wingdings => '\u{2002}',
        '\u{f0fc}' if wingdings => '\u{2713}',
        _ => character,
      })
      .collect(),
  )
}

fn link_annotation_for_rect(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  url: &str,
  internal_links: &InternalLinkTargets,
) -> Option<Annotation> {
  let rect = Rect::from_ltrb(x_pt, y_pt, x_pt + width_pt, y_pt + height_pt)?;
  let target = if is_internal_link_url(url) {
    internal_links.target_for_url(url)?
  } else {
    Target::Action(Action::Link(LinkAction::new(normalize_external_url(url))))
  };
  Some(Annotation::new_link(
    LinkAnnotation::new(rect, target),
    None,
  ))
}

fn normalize_external_url(url: &str) -> String {
  // OOXML relationship targets may contain Windows backslashes even when the
  // value is already a file URI; LO normalizes them before creating links.
  let normalized = url.replace('\\', "/");
  let url = normalized.as_str();
  if let Some(prefix) = url.strip_suffix("://").map(|scheme| format!("{scheme}://")) {
    return prefix;
  }
  if let Some((scheme, rest)) = url.split_once("://")
    && !rest.is_empty()
    && !rest.contains('/')
    && !rest.contains('?')
    && !rest.contains('#')
  {
    return format!("{scheme}://{rest}/");
  }
  normalized
}

fn push_paint_clip(surface: &mut Surface<'_>, clip: Option<&PaintClipRect>) -> bool {
  let Some(clip) = clip else {
    return false;
  };
  if clip.width_pt <= 0.0 || clip.height_pt <= 0.0 {
    return false;
  }
  if let Some(path) = rect_path(clip.x_pt, clip.y_pt, clip.width_pt, clip.height_pt) {
    surface.push_clip_path(&path, &krilla::paint::FillRule::NonZero);
    return true;
  }
  false
}

fn draw_paint_rect(surface: &mut Surface<'_>, rect: &PaintRect) {
  surface.set_stroke(None);
  surface.set_fill(Some(Fill {
    paint: rgb::Color::new(rect.color.r, rect.color.g, rect.color.b).into(),
    opacity: NormalizedF32::ONE,
    rule: Default::default(),
  }));
  let mut path = PathBuilder::new();
  path.move_to(rect.x_pt, rect.y_pt);
  path.line_to(rect.x_pt + rect.width_pt, rect.y_pt);
  path.line_to(rect.x_pt + rect.width_pt, rect.y_pt + rect.height_pt);
  path.line_to(rect.x_pt, rect.y_pt + rect.height_pt);
  path.close();
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_paint_stroke_line(surface: &mut Surface<'_>, line: &PaintStrokeLine) {
  surface.set_fill(None);
  surface.set_stroke(Some(Stroke {
    width: line.width_pt,
    paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
    ..Default::default()
  }));
  let mut path = PathBuilder::new();
  path.move_to(line.x1_pt, line.y1_pt);
  path.line_to(line.x2_pt, line.y2_pt);
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_missing_image(surface: &mut Surface<'_>, image: &ImageItem<'_>) {
  surface.set_fill(None);
  surface.set_stroke(Some(Stroke {
    width: 0.5,
    paint: rgb::Color::new(128, 128, 128).into(),
    ..Default::default()
  }));
  let mut path = PathBuilder::new();
  path.move_to(image.x_pt, image.y_pt);
  path.line_to(image.x_pt + image.width_pt, image.y_pt);
  path.line_to(image.x_pt + image.width_pt, image.y_pt + image.height_pt);
  path.line_to(image.x_pt, image.y_pt + image.height_pt);
  path.close();
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_missing_linked_image(surface: &mut Surface<'_>, image: &ImageItem<'_>) {
  const FRAME_WIDTH_PT: f32 = 0.14;
  const FRAME_INSET_PT: f32 = FRAME_WIDTH_PT / 2.0;
  surface.set_fill(None);
  surface.set_stroke(Some(Stroke {
    width: FRAME_WIDTH_PT,
    paint: rgb::Color::new(0, 0, 0).into(),
    ..Default::default()
  }));
  let mut frame = PathBuilder::new();
  frame.move_to(image.x_pt + FRAME_INSET_PT, image.y_pt + FRAME_INSET_PT);
  frame.line_to(
    image.x_pt + image.width_pt - FRAME_INSET_PT,
    image.y_pt + FRAME_INSET_PT,
  );
  frame.line_to(
    image.x_pt + image.width_pt - FRAME_INSET_PT,
    image.y_pt + image.height_pt - FRAME_INSET_PT,
  );
  frame.line_to(
    image.x_pt + FRAME_INSET_PT,
    image.y_pt + image.height_pt - FRAME_INSET_PT,
  );
  frame.close();
  if let Some(frame) = frame.finish() {
    surface.draw_path(&frame);
  }

  if let Some(icon) = missing_linked_image_icon() {
    surface.push_transform(&Transform::from_translate(
      image.x_pt + 0.84,
      image.y_pt + 0.84,
    ));
    if let Some(size) = Size::from_wh(1.68, 1.92) {
      surface.draw_image(icon, size);
    }
    surface.pop();
  }
}

fn missing_linked_image_icon() -> Option<Image> {
  static ICON: OnceLock<Option<Image>> = OnceLock::new();
  ICON
    .get_or_init(|| {
      const PIXELS: [u8; 60] = [
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 128, 128, 128, 255, 255, 255, 255, 0, 0, 255, 255, 255, 128,
        128, 128, 255, 204, 204, 255, 255, 255, 255, 255, 255, 128, 128, 128, 255, 255, 255, 255,
        255, 255, 255, 255, 255,
      ];
      let mut encoded = Cursor::new(Vec::new());
      PngEncoder::new(&mut encoded)
        .write_image(&PIXELS, 4, 5, ColorType::Rgb8.into())
        .ok()?;
      Image::from_png(encoded.into_inner().into(), false).ok()
    })
    .clone()
}

fn draw_line_item(surface: &mut Surface<'_>, line: &LineItem) {
  let mut path = PathBuilder::new();
  match line.kind {
    LineItemKind::Stroke => {
      surface.set_fill(None);
      surface.set_stroke(Some(Stroke {
        width: line.width_pt,
        paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
        line_cap: line.line_cap,
        dash: line.dash.as_ref().map(|array| StrokeDash {
          array: array.clone(),
          offset: line.dash_offset,
        }),
        ..Default::default()
      }));
      path.move_to(line.x1_pt, line.y1_pt);
      path.line_to(line.x2_pt, line.y2_pt);
    }
    LineItemKind::FilledRect => {
      surface.set_fill(Some(Fill {
        paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
        opacity: NormalizedF32::ONE,
        rule: FillRule::EvenOdd,
      }));
      surface.set_stroke(Some(Stroke {
        width: line.width_pt,
        paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
        line_cap: line.line_cap,
        dash: line.dash.as_ref().map(|array| StrokeDash {
          array: array.clone(),
          offset: line.dash_offset,
        }),
        ..Default::default()
      }));
      path.move_to(line.x1_pt, line.y2_pt);
      path.line_to(line.x1_pt, line.y1_pt);
      path.line_to(line.x2_pt, line.y1_pt);
      path.line_to(line.x2_pt, line.y2_pt);
      path.close();
    }
  }
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_polyline_item(surface: &mut Surface<'_>, polyline: &PolylineItem<'_>) {
  if polyline.points.len() < 2 && polyline.commands.is_empty() {
    return;
  }

  let mut path = PathBuilder::new();
  if let Some((start, end)) = shortened_straight_polyline_points(polyline) {
    path.move_to(start.0, start.1);
    path.line_to(end.0, end.1);
  } else if polyline.commands.is_empty() {
    let first = polyline.points[0];
    path.move_to(first.x.0, first.y.0);
    for point in &polyline.points[1..] {
      path.line_to(point.x.0, point.y.0);
    }
    if polyline.closed {
      path.close();
    }
  } else {
    for command in polyline.commands {
      match *command {
        common::PathCommand::MoveTo(point) => path.move_to(point.x.0, point.y.0),
        common::PathCommand::LineTo(point) => path.line_to(point.x.0, point.y.0),
        common::PathCommand::CubicTo {
          control1,
          control2,
          end,
        } => path.cubic_to(
          control1.x.0,
          control1.y.0,
          control2.x.0,
          control2.y.0,
          end.x.0,
          end.y.0,
        ),
        common::PathCommand::Close => path.close(),
      }
    }
  }
  if let Some(path) = path.finish() {
    let mut fill = path_fill_from_common(surface, polyline.fill, polyline);
    if fill.is_none()
      && matches!(polyline.fill, common::Fill::Gradient(_))
      && draw_path_gradient_raster(surface, &path, polyline)
    {
      // The bounded raster already painted the fill under the same path clip;
      // retain only the independently resolved stroke below.
      fill = None;
    }
    if let Some(stroke) = polyline.stroke
      && stroke.alignment == Some(common::StrokeAlignment::Inside)
      && polyline.closed
    {
      surface.set_fill(fill);
      surface.set_stroke(None);
      surface.draw_path(&path);
      surface.push_clip_path(&path, &FillRule::NonZero);
      let mut inside_stroke = path_stroke_from_common(surface, stroke, polyline);
      inside_stroke.width *= 2.0;
      surface.set_fill(None);
      surface.set_stroke(Some(inside_stroke));
      surface.draw_path(&path);
      surface.pop();
    } else {
      let stroke = polyline
        .stroke
        .map(|stroke| path_stroke_from_common(surface, stroke, polyline));
      if fill.is_some() || stroke.is_some() {
        surface.set_fill(fill);
        surface.set_stroke(stroke);
        surface.draw_path(&path);
      }
    }
  }
  if let Some(stroke) = polyline.stroke {
    draw_stroke_end_markers(surface, polyline, stroke);
  }
}

fn shortened_straight_polyline_points(
  polyline: &PolylineItem<'_>,
) -> Option<((f32, f32), (f32, f32))> {
  if polyline.closed {
    return None;
  }
  let (start, end) = if polyline.commands.is_empty() {
    let [start, end] = polyline.points else {
      return None;
    };
    ((start.x.0, start.y.0), (end.x.0, end.y.0))
  } else {
    let [
      common::PathCommand::MoveTo(start),
      common::PathCommand::LineTo(end),
    ] = polyline.commands
    else {
      return None;
    };
    ((start.x.0, start.y.0), (end.x.0, end.y.0))
  };
  let stroke = polyline.stroke?;
  let head_inset = stroke
    .head_end
    .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
    .map(|_| stroke.width.0)
    .unwrap_or_default();
  let tail_inset = stroke
    .tail_end
    .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
    .map(|_| stroke.width.0)
    .unwrap_or_default();
  if head_inset <= 0.0 && tail_inset <= 0.0 {
    return None;
  }
  let dx = end.0 - start.0;
  let dy = end.1 - start.1;
  let length = dx.hypot(dy);
  if length <= head_inset + tail_inset || length <= f32::EPSILON {
    return None;
  }
  let direction = (dx / length, dy / length);
  Some((
    (
      start.0 + direction.0 * head_inset,
      start.1 + direction.1 * head_inset,
    ),
    (
      end.0 - direction.0 * tail_inset,
      end.1 - direction.1 * tail_inset,
    ),
  ))
}

fn draw_stroke_end_markers(
  surface: &mut Surface<'_>,
  polyline: &PolylineItem<'_>,
  stroke: &common::Stroke<'static>,
) {
  let Some(endpoints) = path_endpoints(polyline) else {
    return;
  };
  let markers = [
    stroke
      .head_end
      .map(|marker| (marker, endpoints.start, endpoints.start_outward)),
    stroke
      .tail_end
      .map(|marker| (marker, endpoints.end, endpoints.end_outward)),
  ];
  let marker_opacity = NormalizedF32::new(opacity(stroke.color)).unwrap_or(NormalizedF32::ZERO);
  for marker in markers.into_iter().flatten() {
    if uses_stroked_open_arrow(marker.0, stroke.width.0) {
      let Some(path) = stroked_open_arrow_path(marker.0, marker.1, marker.2, stroke.width.0) else {
        continue;
      };
      surface.set_fill(None);
      surface.set_stroke(Some(Stroke {
        width: stroke.width.0,
        paint: rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into(),
        opacity: marker_opacity,
        line_cap: LineCap::Round,
        line_join: LineJoin::Miter,
        ..Stroke::default()
      }));
      surface.draw_path(&path);
    } else if let Some(path) = stroke_end_path(marker.0, marker.1, marker.2, stroke.width.0) {
      surface.set_stroke(None);
      surface.set_fill(Some(Fill {
        paint: rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into(),
        opacity: marker_opacity,
        rule: FillRule::NonZero,
      }));
      surface.draw_path(&path);
    }
  }
}

struct PathEndpoints {
  start: (f32, f32),
  start_outward: (f32, f32),
  end: (f32, f32),
  end_outward: (f32, f32),
}

fn path_endpoints(polyline: &PolylineItem<'_>) -> Option<PathEndpoints> {
  if polyline.commands.is_empty() {
    let [first, second, ..] = polyline.points else {
      return None;
    };
    let penultimate = polyline.points[polyline.points.len() - 2];
    let last = polyline.points[polyline.points.len() - 1];
    return Some(PathEndpoints {
      start: (first.x.0, first.y.0),
      start_outward: normalized_direction(second.x.0, second.y.0, first.x.0, first.y.0)?,
      end: (last.x.0, last.y.0),
      end_outward: normalized_direction(penultimate.x.0, penultimate.y.0, last.x.0, last.y.0)?,
    });
  }
  let mut first = None;
  let mut first_tangent = None;
  let mut current = None;
  let mut last_tangent = None;
  let mut closed = false;
  for command in polyline.commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        current = Some((point.x.0, point.y.0));
        first.get_or_insert((point.x.0, point.y.0));
      }
      common::PathCommand::LineTo(point) => {
        let start = current?;
        let end = (point.x.0, point.y.0);
        first_tangent.get_or_insert((start, end));
        last_tangent = Some((start, end));
        current = Some(end);
      }
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        let start = current?;
        let control1 = (control1.x.0, control1.y.0);
        let control2 = (control2.x.0, control2.y.0);
        let end = (end.x.0, end.y.0);
        first_tangent.get_or_insert((start, if control1 != start { control1 } else { end }));
        last_tangent = Some((if control2 != end { control2 } else { start }, end));
        current = Some(end);
      }
      common::PathCommand::Close => closed = true,
    }
  }
  if closed {
    return None;
  }
  let first = first?;
  let (first_from, first_to) = first_tangent?;
  let (last_from, last) = last_tangent?;
  Some(PathEndpoints {
    start: first,
    start_outward: normalized_direction(first_to.0, first_to.1, first_from.0, first_from.1)?,
    end: last,
    end_outward: normalized_direction(last_from.0, last_from.1, last.0, last.1)?,
  })
}

fn normalized_direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Option<(f32, f32)> {
  let dx = to_x - from_x;
  let dy = to_y - from_y;
  let length = dx.hypot(dy);
  (length > f32::EPSILON).then_some((dx / length, dy / length))
}

const MIN_MARKER_BASE_PT: f32 = 70.0 * 72.0 / 2_540.0;

fn stroke_end_size_factor(size: common::StrokeEndSize, is_open_arrow: bool) -> f32 {
  use common::StrokeEndSize as Size;
  match (size, is_open_arrow) {
    (Size::Small, false) => 2.0,
    (Size::Medium, false) => 3.0,
    (Size::Large, false) => 5.0,
    (Size::Small, true) => 2.5,
    (Size::Medium, true) => 3.5,
    (Size::Large, true) => 5.5,
  }
}

fn uses_stroked_open_arrow(marker: common::StrokeEnd, line_width: f32) -> bool {
  // Word fixed output uses a round-capped V once the authored line width,
  // rather than the fixed marker minimum, drives the open-arrow scale.
  marker.kind == common::StrokeEndKind::Arrow && line_width >= MIN_MARKER_BASE_PT
}

fn stroke_end_dimensions(marker: common::StrokeEnd, line_width: f32) -> (f32, f32) {
  use common::{StrokeEndKind as Kind, StrokeEndSize as Size};

  // LibreOffice's DrawingML importer carries line widths in hundredths of a
  // millimetre here. `lineproperties.cxx::lclPushMarkerProperties` clamps the
  // marker baseline to 70 of those units before applying these multipliers.
  let is_open_arrow = marker.kind == Kind::Arrow;
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  if marker.kind == Kind::Arrow
    && marker.width == Size::Medium
    && marker.length == Size::Medium
    && !uses_stroked_open_arrow(marker, line_width)
  {
    // The LibreOffice marker table supplies the open arrow's minimum
    // centerline dimensions. Office fixed output additionally includes the
    // stroke envelope of the two approximately 30-degree arms. Independent
    // 0.75pt and 2pt Word goldens retain the same projection.
    return (
      3.5 * baseline + (3.0_f32.sqrt() / 2.0) * line_width,
      3.0 * baseline + 0.75 * line_width,
    );
  }
  (
    stroke_end_size_factor(marker.width, is_open_arrow) * baseline,
    stroke_end_size_factor(marker.length, is_open_arrow) * baseline,
  )
}

fn stroked_open_arrow_path(
  marker: common::StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  line_width: f32,
) -> Option<krilla::geom::Path> {
  if !uses_stroked_open_arrow(marker, line_width) {
    return None;
  }
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  let marker_width = stroke_end_size_factor(marker.width, true) * baseline;
  let marker_length = stroke_end_size_factor(marker.length, true) * baseline;
  let radius = line_width / 2.0;
  let half_width = marker_width / 2.0;
  let base_offset = marker_length + radius;
  let coefficient = half_width * half_width - radius * radius;
  let linear = 2.0 * radius * radius * base_offset;
  let constant = -radius * radius * (half_width * half_width + base_offset * base_offset);
  let discriminant = linear * linear - 4.0 * coefficient * constant;
  let miter_inset = if coefficient > f32::EPSILON && discriminant >= 0.0 {
    (-linear + discriminant.sqrt()) / (2.0 * coefficient)
  } else {
    radius
  };
  let perpendicular = (-outward.1, outward.0);
  let point = |back: f32, across: f32| {
    (
      endpoint.0 - outward.0 * back + perpendicular.0 * across,
      endpoint.1 - outward.1 * back + perpendicular.1 * across,
    )
  };
  let left = point(base_offset, -half_width);
  let apex = point(miter_inset, 0.0);
  let right = point(base_offset, half_width);
  let mut path = PathBuilder::new();
  path.move_to(left.0, left.1);
  path.line_to(apex.0, apex.1);
  path.line_to(right.0, right.1);
  path.finish()
}

fn stroke_end_path(
  marker: common::StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  line_width: f32,
) -> Option<krilla::geom::Path> {
  use common::StrokeEndKind as Kind;
  if marker.kind == Kind::None {
    return None;
  }
  let (width, length) = stroke_end_dimensions(marker, line_width);
  let centered = matches!(marker.kind, Kind::Diamond | Kind::Oval);
  let line_half_width = (50.0 * line_width / width).max(1.0);
  let points: &[(f32, f32)] = match marker.kind {
    Kind::Triangle => &[(50.0, 0.0), (100.0, 100.0), (0.0, 100.0)],
    Kind::Stealth => &[(50.0, 0.0), (100.0, 100.0), (50.0, 60.0), (0.0, 100.0)],
    Kind::Diamond => &[(50.0, 0.0), (100.0, 50.0), (50.0, 100.0), (0.0, 50.0)],
    Kind::Oval => &[
      (50.0, 0.0),
      (75.0, 7.0),
      (93.0, 25.0),
      (100.0, 50.0),
      (93.0, 75.0),
      (75.0, 93.0),
      (50.0, 100.0),
      (25.0, 93.0),
      (7.0, 75.0),
      (0.0, 50.0),
      (7.0, 25.0),
      (25.0, 7.0),
    ],
    Kind::Arrow => &[
      (50.0, 0.0),
      (100.0, 100.0 - line_half_width * 1.5),
      (100.0 - line_half_width * 1.5, 100.0),
      (50.0 + line_half_width, 5.5 * line_half_width),
      (50.0 + line_half_width, 100.0),
      (50.0 - line_half_width, 100.0),
      (50.0 - line_half_width, 5.5 * line_half_width),
      (line_half_width * 1.5, 100.0),
      (0.0, 100.0 - line_half_width * 1.5),
    ],
    Kind::None => return None,
  };
  let perpendicular = (-outward.1, outward.0);
  let point = |x: f32, y: f32| {
    let across = (x / 100.0 - 0.5) * width;
    let back = (y / 100.0 - if centered { 0.5 } else { 0.0 }) * length;
    (
      endpoint.0 - outward.0 * back + perpendicular.0 * across,
      endpoint.1 - outward.1 * back + perpendicular.1 * across,
    )
  };
  let mut path = PathBuilder::new();
  let first = point(points[0].0, points[0].1);
  path.move_to(first.0, first.1);
  for &(x, y) in &points[1..] {
    let point = point(x, y);
    path.line_to(point.0, point.1);
  }
  path.close();
  path.finish()
}

fn path_stroke_from_common(
  surface: &mut Surface<'_>,
  stroke: &common::Stroke<'static>,
  path: &PolylineItem<'_>,
) -> Stroke {
  let line_join = match stroke.join {
    Some(common::StrokeJoin::Round) => LineJoin::Round,
    Some(common::StrokeJoin::Bevel) => LineJoin::Bevel,
    Some(common::StrokeJoin::Miter { .. }) | None => LineJoin::Miter,
  };
  let miter_limit = match stroke.join {
    Some(common::StrokeJoin::Miter { limit: Some(limit) }) => limit,
    _ => Stroke::default().miter_limit,
  };
  let line_cap = match stroke.cap {
    Some(common::StrokeCap::Round) => LineCap::Round,
    Some(common::StrokeCap::Square) => LineCap::Square,
    Some(common::StrokeCap::Flat) | None => LineCap::Butt,
  };
  let dash = stroke.resolved_dash().as_ref().map(|values| StrokeDash {
    array: values.iter().map(|value| value.0).collect(),
    offset: stroke.dash_offset.0,
  });
  Stroke {
    width: stroke.width.0,
    paint: if let Some(gradient) = stroke.gradient.as_ref() {
      if let Some(path_gradient) = gradient.path {
        path_gradient_paint(gradient, path_gradient)
          .unwrap_or_else(|| rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into())
      } else {
        let (start, end) = gradient.line.unwrap_or_else(|| {
          linear_gradient_line(
            gradient.definition_bounds.unwrap_or(common::Rect {
              origin: common::Point {
                x: common::Pt(path.x_pt),
                y: common::Pt(path.y_pt),
              },
              size: common::Size {
                width: common::Pt(path.width_pt),
                height: common::Pt(path.height_pt),
              },
            }),
            gradient.angle_degrees,
            gradient.scaled,
          )
        });
        let stops = gradient_stops_for_pdf(gradient);
        LinearGradient {
          x1: start.x.0,
          y1: start.y.0,
          x2: end.x.0,
          y2: end.y.0,
          transform: Transform::default(),
          spread_method: SpreadMethod::Pad,
          stops: pdf_gradient_stops(&stops, false),
          anti_alias: true,
        }
        .into()
      }
    } else if let Some(pattern) = stroke.pattern {
      drawingml_pattern_paint(surface, pattern, path.x_pt, path.y_pt)
    } else {
      rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into()
    },
    opacity: if stroke.pattern.is_some() || stroke.gradient.is_some() {
      NormalizedF32::ONE
    } else {
      NormalizedF32::new(opacity(stroke.color)).unwrap_or(NormalizedF32::ZERO)
    },
    line_cap,
    line_join,
    miter_limit,
    dash,
  }
}

fn path_from_commands(commands: &[common::PathCommand]) -> Option<krilla::geom::Path> {
  if commands.is_empty() {
    return None;
  }
  let mut path = PathBuilder::new();
  for command in commands {
    match *command {
      common::PathCommand::MoveTo(point) => path.move_to(point.x.0, point.y.0),
      common::PathCommand::LineTo(point) => path.line_to(point.x.0, point.y.0),
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => path.cubic_to(
        control1.x.0,
        control1.y.0,
        control2.x.0,
        control2.y.0,
        end.x.0,
        end.y.0,
      ),
      common::PathCommand::Close => path.close(),
    }
  }
  path.finish()
}

fn path_fill_from_common(
  surface: &mut Surface<'_>,
  fill: &common::Fill<'static>,
  path: &PolylineItem<'_>,
) -> Option<Fill> {
  let fill_opacity = match fill {
    common::Fill::Solid(color) => {
      NormalizedF32::new(opacity(*color)).unwrap_or(NormalizedF32::ZERO)
    }
    common::Fill::None
    | common::Fill::Theme(_)
    | common::Fill::Gradient(_)
    | common::Fill::Image { .. }
    | common::Fill::Pattern(_) => NormalizedF32::ONE,
  };
  let paint = match fill {
    common::Fill::Solid(color) => rgb::Color::new(color.r, color.g, color.b).into(),
    common::Fill::Gradient(gradient) => {
      if let Some(path) = gradient.path {
        path_gradient_paint(gradient, path)?
      } else {
        let (start, end) = gradient.line.unwrap_or_else(|| {
          let bounds = gradient.definition_bounds.unwrap_or(common::Rect {
            origin: common::Point {
              x: common::Pt(path.x_pt),
              y: common::Pt(path.y_pt),
            },
            size: common::Size {
              width: common::Pt(path.width_pt),
              height: common::Pt(path.height_pt),
            },
          });
          linear_gradient_line(bounds, gradient.angle_degrees, gradient.scaled)
        });
        let stops = gradient_stops_for_pdf(gradient);
        LinearGradient {
          x1: start.x.0,
          y1: start.y.0,
          x2: end.x.0,
          y2: end.y.0,
          transform: Transform::default(),
          spread_method: SpreadMethod::Pad,
          stops: pdf_gradient_stops(&stops, false),
          anti_alias: true,
        }
        .into()
      }
    }
    common::Fill::Pattern(pattern) => {
      drawingml_pattern_paint(surface, *pattern, path.x_pt, path.y_pt)
    }
    common::Fill::None | common::Fill::Theme(_) | common::Fill::Image { .. } => return None,
  };
  Some(Fill {
    paint,
    opacity: fill_opacity,
    rule: FillRule::EvenOdd,
  })
}

fn drawingml_pattern_paint(
  surface: &mut Surface<'_>,
  pattern: common::PatternFill,
  origin_x: f32,
  origin_y: f32,
) -> krilla::paint::Paint {
  let tile_image = pattern_tile_image(pattern);
  let tile_size_pt = if tile_image.is_some() {
    pattern.bitmap_tile_size_points()
  } else {
    pattern.tile_size_points()
  };
  let pattern_units = if tile_image.is_some() {
    f32::from(pattern.bitmap_sampling.image_size_px())
  } else {
    tile_size_pt
  };
  let pattern_scale = tile_size_pt / pattern_units;
  let cell_size = pattern_units / 8.0;
  let mut stream_builder = surface.stream_builder();
  let mut pattern_surface = stream_builder.surface();
  // Krilla serializes its y-down page space through a y-reflected PDF root
  // transform. Office instead keeps tiling-pattern matrices in PDF's y-up
  // user space. Reflect both the pattern stream and its outer transform: the
  // two reflections preserve the logical brush, while matching Office's
  // sampling direction at device-pixel boundaries.
  pattern_surface.push_transform(&Transform::from_row(
    1.0,
    0.0,
    0.0,
    -1.0,
    0.0,
    pattern_units,
  ));
  if let (Some(image), Some(size)) = (tile_image, Size::from_wh(pattern_units, pattern_units)) {
    // Office fixed output embeds the authored 8×8 mask as a non-interpolated
    // bitmap inside the tiling pattern. Vectorizing its 0.12pt cells lets PDF
    // antialiasing blend away the foreground color and changes the apparent
    // hatch density, especially for Word's 0.96pt shading brush.
    pattern_surface.draw_image(image, size);
  } else {
    pattern_surface.set_stroke(None);
    pattern_surface.set_fill(Some(pattern_color_fill(pattern.background)));
    if let Some(background) = rect_path(0.0, 0.0, pattern_units, pattern_units) {
      pattern_surface.draw_path(&background);
    }

    let mut foreground = PathBuilder::new();
    for (row, mask) in pattern.pattern_rows().iter().copied().enumerate() {
      for column in 0..emfsdk::emfplus::EmfPlusHatchStyle::TILE_SIZE as usize {
        if mask & (0x80_u8 >> column) == 0 {
          continue;
        }
        let x = column as f32 * cell_size;
        let y = row as f32 * cell_size;
        foreground.move_to(x, y);
        foreground.line_to(x + cell_size, y);
        foreground.line_to(x + cell_size, y + cell_size);
        foreground.line_to(x, y + cell_size);
        foreground.close();
      }
    }
    pattern_surface.set_fill(Some(pattern_color_fill(pattern.foreground)));
    if let Some(foreground) = foreground.finish() {
      pattern_surface.draw_path(&foreground);
    }
  }
  pattern_surface.pop();
  pattern_surface.finish();

  Pattern {
    stream: stream_builder.finish(),
    // Office keeps the preset hatch brush in page/world coordinates. Snap to
    // an equivalent global tile boundary near the path so separate shapes do
    // not restart the 8x8 mask at their own top-left corners.
    transform: Transform::from_row(
      pattern_scale,
      0.0,
      0.0,
      -pattern_scale,
      pattern_origin(origin_x, tile_size_pt),
      pattern_origin(origin_y, tile_size_pt),
    ),
    width: pattern_units,
    height: pattern_units,
  }
  .into()
}

fn pattern_tile_image(pattern: common::PatternFill) -> Option<Image> {
  if matches!(pattern.mask, common::PatternMask::EmfPlusHatch(_))
    && pattern.bitmap_sampling == common::PatternBitmapSampling::NATIVE_8X8
  {
    return None;
  }
  let image_size = u32::from(pattern.bitmap_sampling.image_size_px());
  let has_alpha = pattern.foreground.a != u8::MAX || pattern.background.a != u8::MAX;
  let component_count = if has_alpha { 4 } else { 3 };
  let mut pixels = Vec::with_capacity(image_size as usize * image_size as usize * component_count);
  for y in 0..image_size {
    for x in 0..image_size {
      let color = if pattern.bitmap_sample_is_foreground(x, y) {
        pattern.foreground
      } else {
        pattern.background
      };
      pixels.extend_from_slice(&[color.r, color.g, color.b]);
      if has_alpha {
        pixels.push(color.a);
      }
    }
  }
  let color_type = if has_alpha {
    ColorType::Rgba8
  } else {
    ColorType::Rgb8
  };
  let mut encoded = Cursor::new(Vec::new());
  PngEncoder::new(&mut encoded)
    .write_image(&pixels, image_size, image_size, color_type.into())
    .ok()?;
  Image::from_png(encoded.into_inner().into(), false).ok()
}

fn pattern_origin(value: f32, tile_size_pt: f32) -> f32 {
  (value / tile_size_pt).floor() * tile_size_pt
}

fn pattern_color_fill(color: common::Color) -> Fill {
  Fill {
    paint: rgb::Color::new(color.r, color.g, color.b).into(),
    opacity: NormalizedF32::new(opacity(color)).unwrap_or(NormalizedF32::ZERO),
    rule: FillRule::NonZero,
  }
}

fn path_gradient_paint(
  gradient: &common::GradientFill<'static>,
  mut path: common::GradientPath,
) -> Option<krilla::paint::Paint> {
  if path.kind != common::GradientPathKind::Circle || path.mirror_tile {
    return None;
  }
  path.fill_to = normalized_path_gradient_focus(path.fill_to);
  let focus_width = 1.0 - path.fill_to.left - path.fill_to.right;
  let focus_height = 1.0 - path.fill_to.top - path.fill_to.bottom;
  // A PDF type-3 radial shading can represent two circles under one affine.
  // This exactly covers DrawingML circle gradients whose focus path is a
  // similarly scaled ellipse. Independent x/y focus scales require the
  // bounded path-gradient raster path used for rect/shape gradients.
  if focus_width < 0.0 || focus_height < 0.0 || (focus_width - focus_height).abs() > 1.0e-5 {
    return None;
  }
  let focus_x = (path.fill_to.left + 1.0 - path.fill_to.right) / 2.0;
  let focus_y = (path.fill_to.top + 1.0 - path.fill_to.bottom) / 2.0;
  let transform = path.transform;
  let stops = gradient_stops_for_pdf(gradient);
  Some(
    RadialGradient {
      fx: focus_x,
      fy: focus_y,
      fr: focus_width / 2.0,
      cx: 0.5,
      cy: 0.5,
      cr: 0.5,
      transform: Transform::from_row(
        transform.m11,
        transform.m12,
        transform.m21,
        transform.m22,
        transform.dx.0,
        transform.dy.0,
      ),
      spread_method: SpreadMethod::Pad,
      // DrawingML path gradients start at the focus path and grow toward the
      // outer boundary. A PDF radial shading uses the same direction.
      stops: pdf_gradient_stops(&stops, false),
      anti_alias: true,
    }
    .into(),
  )
}

fn draw_path_gradient_raster(
  surface: &mut Surface<'_>,
  clip_path: &krilla::geom::Path,
  polyline: &PolylineItem<'_>,
) -> bool {
  let common::Fill::Gradient(gradient) = polyline.fill else {
    return false;
  };
  let Some(path) = gradient.path else {
    return false;
  };
  if polyline.width_pt <= f32::EPSILON
    || polyline.height_pt <= f32::EPSILON
    || gradient.stops.is_empty()
  {
    return false;
  }
  let mut pixels_per_point =
    (MAX_PATH_GRADIENT_RASTER_PIXELS / (polyline.width_pt * polyline.height_pt)).sqrt();
  pixels_per_point = pixels_per_point.clamp(0.25, MAX_PATH_GRADIENT_PIXELS_PER_POINT);
  let width_px = (polyline.width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (polyline.height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let base_shape = if path.kind == common::GradientPathKind::Shape {
    let Some(polygons) = path_polygons_in_gradient_space(polyline.commands, path.transform) else {
      return false;
    };
    Some(polygons)
  } else {
    None
  };

  let mut rgba = Vec::with_capacity(width_px as usize * height_px as usize * 4);
  let pixel_width_pt = f64::from(polyline.width_pt) / f64::from(width_px);
  let pixel_height_pt = f64::from(polyline.height_pt) / f64::from(height_px);
  for y in 0..height_px {
    let page_y = f64::from(polyline.y_pt) + (f64::from(y) + 0.5) * pixel_height_pt;
    for x in 0..width_px {
      let page_x = f64::from(polyline.x_pt) + (f64::from(x) + 0.5) * pixel_width_pt;
      let Some(point) = inverse_gradient_point(path.transform, page_x, page_y) else {
        return false;
      };
      let position = path_gradient_position(path, point, base_shape.as_deref()).unwrap_or_default();
      let color = sample_gradient(&gradient.stops, position);
      rgba.extend_from_slice(&[color.r, color.g, color.b, color.a]);
    }
  }

  let mut encoded = Cursor::new(Vec::new());
  if PngEncoder::new(&mut encoded)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .is_err()
  {
    return false;
  }
  let Ok(image) = Image::from_png(encoded.into_inner().into(), true) else {
    return false;
  };
  let Some(size) = Size::from_wh(polyline.width_pt, polyline.height_pt) else {
    return false;
  };
  surface.push_clip_path(clip_path, &FillRule::EvenOdd);
  surface.push_transform(&Transform::from_translate(polyline.x_pt, polyline.y_pt));
  surface.draw_image(image, size);
  surface.pop();
  surface.pop();
  true
}

fn inverse_gradient_point(
  transform: common::Transform,
  page_x: f64,
  page_y: f64,
) -> Option<kurbo::Point> {
  let m11 = f64::from(transform.m11);
  let m12 = f64::from(transform.m12);
  let m21 = f64::from(transform.m21);
  let m22 = f64::from(transform.m22);
  let determinant = m11 * m22 - m12 * m21;
  if !determinant.is_finite() || determinant.abs() <= f64::from(f32::EPSILON) {
    return None;
  }
  let x = page_x - f64::from(transform.dx);
  let y = page_y - f64::from(transform.dy);
  Some(kurbo::Point::new(
    (m22 * x - m21 * y) / determinant,
    (-m12 * x + m11 * y) / determinant,
  ))
}

fn path_gradient_position(
  path: common::GradientPath,
  mut point: kurbo::Point,
  shape: Option<&[Vec<kurbo::Point>]>,
) -> Option<f32> {
  if path.mirror_tile {
    point.x = mirrored_tile_coordinate(point.x);
    point.y = mirrored_tile_coordinate(point.y);
  }
  if !path_gradient_contains(path, point, 1.0, shape)? {
    return Some(1.0);
  }
  if path_gradient_contains(path, point, 0.0, shape)? {
    return Some(0.0);
  }
  // Search the monotonic family of affine copies between the focus and outer
  // paths. DrawingML stop position 0 is the focus and position 1 is the outer
  // boundary.
  let mut outside = 0.0;
  let mut inside = 1.0;
  for _ in 0..PATH_GRADIENT_BINARY_STEPS {
    let middle = (outside + inside) / 2.0;
    if path_gradient_contains(path, point, middle, shape)? {
      inside = middle;
    } else {
      outside = middle;
    }
  }
  Some(normalized_f64_to_f32(inside))
}

fn mirrored_tile_coordinate(value: f64) -> f64 {
  let tile = value.floor();
  let fraction = value - tile;
  if tile.rem_euclid(2.0) < 1.0 {
    fraction
  } else {
    1.0 - fraction
  }
}

fn path_gradient_contains(
  path: common::GradientPath,
  point: kurbo::Point,
  outer_ratio: f64,
  shape: Option<&[Vec<kurbo::Point>]>,
) -> Option<bool> {
  let focus = normalized_path_gradient_focus(path.fill_to);
  let focus_width = 1.0 - f64::from(focus.left) - f64::from(focus.right);
  let focus_height = 1.0 - f64::from(focus.top) - f64::from(focus.bottom);
  let scale_x = focus_width + (1.0 - focus_width) * outer_ratio;
  let scale_y = focus_height + (1.0 - focus_height) * outer_ratio;
  let offset_x = f64::from(focus.left) * (1.0 - outer_ratio);
  let offset_y = f64::from(focus.top) * (1.0 - outer_ratio);
  if scale_x.abs() <= f64::EPSILON || scale_y.abs() <= f64::EPSILON {
    return Some(
      (point.x - offset_x).abs() <= f64::EPSILON && (point.y - offset_y).abs() <= f64::EPSILON,
    );
  }
  let base = kurbo::Point::new(
    (point.x - offset_x) / scale_x,
    (point.y - offset_y) / scale_y,
  );
  Some(match path.kind {
    common::GradientPathKind::Circle => {
      let x = (base.x - 0.5) * 2.0;
      let y = (base.y - 0.5) * 2.0;
      x.mul_add(x, y * y) <= 1.0
    }
    common::GradientPathKind::Rectangle => {
      (0.0..=1.0).contains(&base.x) && (0.0..=1.0).contains(&base.y)
    }
    common::GradientPathKind::Shape => point_in_polygons(base, shape?),
  })
}

fn normalized_path_gradient_focus(rect: common::RelativeRect) -> common::RelativeRect {
  let authored_right = 1.0 - rect.right;
  let authored_bottom = 1.0 - rect.bottom;
  let left = rect.left.min(authored_right);
  let top = rect.top.min(authored_bottom);
  let right = rect.left.max(authored_right);
  let bottom = rect.top.max(authored_bottom);
  common::RelativeRect {
    left,
    top,
    right: 1.0 - right,
    bottom: 1.0 - bottom,
  }
}

fn path_polygons_in_gradient_space(
  commands: &[common::PathCommand],
  transform: common::Transform,
) -> Option<Vec<Vec<kurbo::Point>>> {
  let mut elements = Vec::with_capacity(commands.len());
  for command in commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        let point = inverse_gradient_point(transform, f64::from(point.x), f64::from(point.y))?;
        elements.push(PathEl::MoveTo(point));
      }
      common::PathCommand::LineTo(point) => {
        let point = inverse_gradient_point(transform, f64::from(point.x), f64::from(point.y))?;
        elements.push(PathEl::LineTo(point));
      }
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        let control1 =
          inverse_gradient_point(transform, f64::from(control1.x), f64::from(control1.y))?;
        let control2 =
          inverse_gradient_point(transform, f64::from(control2.x), f64::from(control2.y))?;
        let end = inverse_gradient_point(transform, f64::from(end.x), f64::from(end.y))?;
        elements.push(PathEl::CurveTo(control1, control2, end));
      }
      common::PathCommand::Close => elements.push(PathEl::ClosePath),
    }
  }
  let mut polygons = Vec::new();
  let mut polygon = Vec::new();
  // The unit gradient space is scaled to at most 500 pixels in either axis,
  // so this is a quarter-pixel curve tolerance at the raster budget ceiling.
  flatten(elements, 0.0005, |element| match element {
    PathEl::MoveTo(point) => {
      finish_gradient_polygon(&mut polygons, &mut polygon);
      polygon.push(point);
    }
    PathEl::LineTo(point) => polygon.push(point),
    PathEl::ClosePath => finish_gradient_polygon(&mut polygons, &mut polygon),
    PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
      unreachable!("kurbo::flatten only emits line path elements")
    }
  });
  finish_gradient_polygon(&mut polygons, &mut polygon);
  (!polygons.is_empty()).then_some(polygons)
}

fn finish_gradient_polygon(polygons: &mut Vec<Vec<kurbo::Point>>, polygon: &mut Vec<kurbo::Point>) {
  if polygon.len() >= 3 {
    if polygon.first() != polygon.last() {
      polygon.push(polygon[0]);
    }
    polygons.push(std::mem::take(polygon));
  } else {
    polygon.clear();
  }
}

fn point_in_polygons(point: kurbo::Point, polygons: &[Vec<kurbo::Point>]) -> bool {
  let mut inside = false;
  for polygon in polygons {
    for edge in polygon.windows(2) {
      let (x1, y1) = (edge[0].x, edge[0].y);
      let (x2, y2) = (edge[1].x, edge[1].y);
      if (y1 > point.y) != (y2 > point.y) && point.x < (x2 - x1) * (point.y - y1) / (y2 - y1) + x1 {
        inside = !inside;
      }
    }
  }
  inside
}

fn normalized_f64_to_f32(value: f64) -> f32 {
  debug_assert!(value.is_finite());
  value.clamp(0.0, 1.0) as f32
}

fn sample_gradient(stops: &[common::GradientStop<'static>], position: f32) -> common::Color {
  let Some(first) = stops.first() else {
    return common::Color::default();
  };
  if position <= first.position {
    return first.color;
  }
  for pair in stops.windows(2) {
    let start = &pair[0];
    let end = &pair[1];
    if position <= end.position {
      let span = end.position - start.position;
      let ratio = if span.abs() <= f32::EPSILON {
        1.0
      } else {
        ((position - start.position) / span).clamp(0.0, 1.0)
      };
      let channel = |start: u8, end: u8| {
        (f32::from(start) + (f32::from(end) - f32::from(start)) * ratio)
          .round()
          .clamp(0.0, 255.0) as u8
      };
      return common::Color {
        r: channel(start.color.r, end.color.r),
        g: channel(start.color.g, end.color.g),
        b: channel(start.color.b, end.color.b),
        a: channel(start.color.a, end.color.a),
      };
    }
  }
  stops.last().map_or(first.color, |stop| stop.color)
}

fn pdf_gradient_stops(stops: &[common::GradientStop<'static>], reverse: bool) -> Vec<Stop> {
  let iter: Box<dyn Iterator<Item = &common::GradientStop<'static>> + '_> = if reverse {
    Box::new(stops.iter().rev())
  } else {
    Box::new(stops.iter())
  };
  iter
    .filter_map(|stop| {
      let position = if reverse {
        1.0 - stop.position
      } else {
        stop.position
      };
      Some(Stop {
        offset: NormalizedF32::new(position.clamp(0.0, 1.0))?,
        color: rgb::Color::new(stop.color.r, stop.color.g, stop.color.b).into(),
        opacity: NormalizedF32::new(f32::from(stop.color.a) / 255.0)?,
      })
    })
    .collect()
}

fn draw_rect_item(surface: &mut Surface<'_>, rect: &RectItem) {
  if let Some(fill) = rect.fill {
    let fill = match fill {
      RectFill::Solid { color, opacity } => Fill {
        paint: rgb::Color::new(color.r, color.g, color.b).into(),
        opacity: NormalizedF32::new(opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
        rule: FillRule::EvenOdd,
      },
      RectFill::Pattern(pattern) => Fill {
        paint: drawingml_pattern_paint(surface, pattern, rect.x_pt, rect.y_pt),
        opacity: NormalizedF32::ONE,
        rule: FillRule::EvenOdd,
      },
    };
    surface.set_fill(Some(fill));
    surface.set_stroke(None);
    draw_rect_path(surface, rect);
  }

  if let Some(stroke) = rect.stroke
    && (rect.stroke_opacity > f32::EPSILON || (rect.fill.is_none() && rect.height_pt < 50.0))
  {
    surface.set_fill(None);
    surface.set_stroke(Some(Stroke {
      width: stroke.width_pt,
      paint: rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into(),
      opacity: NormalizedF32::new(rect.stroke_opacity.clamp(0.0, 1.0))
        .unwrap_or(NormalizedF32::ZERO),
      ..Default::default()
    }));
    draw_rect_path(surface, rect);
  }
}

fn gradient_stops_for_pdf(
  gradient: &common::GradientFill<'static>,
) -> Vec<common::GradientStop<'static>> {
  if gradient.interpolation != common::GradientInterpolation::PowerPointGammaSigma
    || gradient.stops.len() < 2
  {
    return gradient.stops.clone();
  }

  // Samples of the position-independent blend factor produced by the Windows
  // GDI+ LinearGradientBrush SetSigmaBellShape(1, 1) path. PowerPoint's
  // fixed-format PDF writer combines this falloff with gamma-correct color
  // interpolation for transformed DrawingML gradients.
  const SIGMA_BLEND_U8: [u8; 33] = [
    0, 2, 5, 8, 12, 17, 22, 29, 36, 45, 54, 65, 76, 88, 101, 114, 128, 141, 154, 167, 179, 190,
    201, 210, 219, 226, 233, 238, 243, 247, 250, 253, 255,
  ];
  let mut stops = Vec::with_capacity((gradient.stops.len() - 1) * 32 + 1);
  for pair in gradient.stops.windows(2) {
    let start = &pair[0];
    let end = &pair[1];
    for (step, blend) in SIGMA_BLEND_U8[..32].iter().enumerate() {
      let position_ratio = step as f32 / 32.0;
      let blend = f32::from(*blend) / 255.0;
      stops.push(common::GradientStop {
        position: start.position + (end.position - start.position) * position_ratio,
        color: gamma_correct_gradient_color(start.color, end.color, blend),
        scheme: None,
      });
    }
  }
  stops.push(
    gradient
      .stops
      .last()
      .expect("gradient has two stops")
      .clone(),
  );
  stops
}

fn gamma_correct_gradient_color(
  start: common::Color,
  end: common::Color,
  blend: f32,
) -> common::Color {
  let channel = |start: u8, end: u8| {
    let start = gdiplus_gamma_decode(f32::from(start) / 255.0);
    let end = gdiplus_gamma_decode(f32::from(end) / 255.0);
    (gdiplus_gamma_encode(start + (end - start) * blend) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  common::Color {
    r: channel(start.r, end.r),
    g: channel(start.g, end.g),
    b: channel(start.b, end.b),
    a: (f32::from(start.a) + (f32::from(end.a) - f32::from(start.a)) * blend)
      .round()
      .clamp(0.0, 255.0) as u8,
  }
}

fn gdiplus_gamma_decode(value: f32) -> f32 {
  value.powf(2.2)
}

fn gdiplus_gamma_encode(value: f32) -> f32 {
  value.powf(1.0 / 2.2)
}

fn linear_gradient_line(
  bounds: common::Rect,
  angle_degrees: Option<f32>,
  scaled: bool,
) -> (common::Point, common::Point) {
  let angle = angle_degrees.unwrap_or(0.0).to_radians();
  let mut direction_x = angle.cos();
  let mut direction_y = angle.sin();
  if scaled {
    direction_x *= bounds.size.width.0;
    direction_y *= bounds.size.height.0;
  }
  let length = direction_x.hypot(direction_y).max(f32::EPSILON);
  direction_x /= length;
  direction_y /= length;
  let half_span =
    (direction_x.abs() * bounds.size.width.0 + direction_y.abs() * bounds.size.height.0) / 2.0;
  let center_x = bounds.origin.x.0 + bounds.size.width.0 / 2.0;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 / 2.0;
  (
    common::Point {
      x: common::Pt(center_x - direction_x * half_span),
      y: common::Pt(center_y - direction_y * half_span),
    },
    common::Point {
      x: common::Pt(center_x + direction_x * half_span),
      y: common::Pt(center_y + direction_y * half_span),
    },
  )
}

fn draw_rect_path(surface: &mut Surface<'_>, rect: &RectItem) {
  let mut path = PathBuilder::new();
  path.move_to(rect.x_pt, rect.y_pt + rect.height_pt);
  path.line_to(rect.x_pt, rect.y_pt);
  path.line_to(rect.x_pt + rect.width_pt, rect.y_pt);
  path.line_to(rect.x_pt + rect.width_pt, rect.y_pt + rect.height_pt);
  path.close();
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_image_item(surface: &mut Surface<'_>, image: &ImageItem<'_>, pdf_image: Image) {
  let adjusted;
  let image = if let Some((width_pt, height_pt)) = metafile_native_paint_size(image) {
    adjusted = {
      let mut adjusted = image.clone();
      adjusted.width_pt = width_pt;
      adjusted.height_pt = height_pt;
      adjusted
    };
    &adjusted
  } else {
    image
  };
  draw_transformed_image_content(surface, image, |surface, size| {
    surface.draw_image(pdf_image, size);
  });
}

fn draw_metafile_vector_item(
  surface: &mut Surface<'_>,
  image: &ImageItem<'_>,
  scene: &ooxmlsdk_layout::render::emf_wmf::MetafileVectorScene,
) {
  let adjusted;
  let image = if let Some((width_pt, height_pt)) = metafile_native_paint_size(image) {
    adjusted = {
      let mut adjusted = image.clone();
      adjusted.width_pt = width_pt;
      adjusted.height_pt = height_pt;
      adjusted
    };
    &adjusted
  } else {
    image
  };
  draw_transformed_image_content(surface, image, |surface, size| {
    surface.set_stroke(None);
    for fill in &scene.fills {
      let mut path = PathBuilder::new();
      for subpath in &fill.subpaths {
        let Some(first) = subpath.first() else {
          continue;
        };
        path.move_to(first.x * size.width(), first.y * size.height());
        for point in &subpath[1..] {
          path.line_to(point.x * size.width(), point.y * size.height());
        }
        path.close();
      }
      let Some(path) = path.finish() else {
        continue;
      };
      surface.set_fill(Some(Fill {
        paint: rgb::Color::new(fill.color[0], fill.color[1], fill.color[2]).into(),
        opacity: NormalizedF32::ONE,
        rule: match fill.fill_rule {
          ooxmlsdk_layout::render::emf_wmf::MetafileVectorFillRule::Alternate => FillRule::EvenOdd,
          ooxmlsdk_layout::render::emf_wmf::MetafileVectorFillRule::Winding => FillRule::NonZero,
        },
      }));
      surface.draw_path(&path);
    }
  });
}

fn metafile_native_paint_size(image: &ImageItem<'_>) -> Option<(f32, f32)> {
  if !image.metafile_native_size
    || image.metafile_background_color.is_some()
    || image.crop != ImageCrop::default()
  {
    return None;
  }
  let physical = ooxmlsdk_layout::render::emf_wmf::metafile_physical_size(
    &image.data,
    image.content_type.as_deref(),
  )?;
  let width_pixel_pt = physical.width_pt / physical.natural_width_px.max(1) as f32;
  let height_pixel_pt = physical.height_pt / physical.natural_height_px.max(1) as f32;
  // Producers commonly round an EMF's 0.01mm Frame when writing DrawingML
  // extents. Word keeps the authored frame as the clip but paints a
  // native-size preview when the difference is below one source device
  // pixel. Larger differences are intentional OOXML resizing and must keep
  // the authored dimensions.
  ((image.width_pt - physical.width_pt).abs() <= width_pixel_pt
    && (image.height_pt - physical.height_pt).abs() <= height_pixel_pt)
    .then_some((physical.width_pt, physical.height_pt))
}

fn draw_metafile_host_background(surface: &mut Surface<'_>, image: &ImageItem<'_>, color: [u8; 3]) {
  let mut background = image.clone();
  background.crop = ImageCrop::default();
  draw_transformed_image_content(surface, &background, |surface, size| {
    draw_paint_rect(
      surface,
      &PaintRect {
        x_pt: 0.0,
        y_pt: 0.0,
        width_pt: size.width(),
        height_pt: size.height(),
        color: RgbColor {
          r: color[0],
          g: color[1],
          b: color[2],
        },
      },
    );
  });
}

fn is_svg_image(image: &ImageItem<'_>) -> bool {
  image
    .content_type
    .as_deref()
    .is_some_and(|content_type| content_type.eq_ignore_ascii_case("image/svg+xml"))
    || std::str::from_utf8(&image.data)
      .ok()
      .is_some_and(|text| text.trim_start().starts_with("<svg"))
}

fn draw_svg_item(surface: &mut Surface<'_>, image: &ImageItem<'_>, tree: &usvg::Tree) {
  draw_transformed_image_content(surface, image, |surface, size| {
    let office_math = image.content_type.as_deref().is_some_and(|content_type| {
      content_type.eq_ignore_ascii_case("application/vnd.ooxmlsdk.office-math+xml")
    });
    surface.draw_svg(
      tree,
      size,
      SvgSettings {
        // An OOXML picture is one semantic image. Keeping SVG text as paths
        // avoids leaking decorative image text into PDF text extraction.
        // OfficeMath carries its visible and semantic glyphs in marked text
        // nodes and lowers them in source order immediately below.
        embed_text: false,
        ..SvgSettings::default()
      },
    );
    if office_math {
      draw_office_math_text(surface, tree, size);
    }
  });
}

const OFFICE_MATH_VISIBLE_GLYPH_PREFIX: &str = "ooxmlsdk-math-visible-";
const OFFICE_MATH_SEMANTIC_GLYPH_PREFIX: &str = "ooxmlsdk-math-semantic-";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OfficeMathSvgTextMarker {
  Visible,
  Semantic { exact_glyph_id: Option<u32> },
}

fn office_math_svg_text_marker(id: &str) -> Option<OfficeMathSvgTextMarker> {
  if let Some(item_index) = id.strip_prefix(OFFICE_MATH_VISIBLE_GLYPH_PREFIX) {
    item_index.parse::<usize>().ok()?;
    return Some(OfficeMathSvgTextMarker::Visible);
  }

  let marker = id.strip_prefix(OFFICE_MATH_SEMANTIC_GLYPH_PREFIX)?;
  if let Ok(_item_index) = marker.parse::<usize>() {
    return Some(OfficeMathSvgTextMarker::Semantic {
      exact_glyph_id: None,
    });
  }
  let (item_index, glyph_id) = marker.rsplit_once("-gid-")?;
  item_index.parse::<usize>().ok()?;
  Some(OfficeMathSvgTextMarker::Semantic {
    exact_glyph_id: Some(glyph_id.parse::<u32>().ok()?),
  })
}

#[cfg(test)]
fn office_math_semantic_glyph_id(id: &str) -> Option<u32> {
  match office_math_svg_text_marker(id)? {
    OfficeMathSvgTextMarker::Semantic {
      exact_glyph_id: Some(glyph_id),
    } => Some(glyph_id),
    OfficeMathSvgTextMarker::Visible
    | OfficeMathSvgTextMarker::Semantic {
      exact_glyph_id: None,
    } => None,
  }
}

type OfficeMathSvgFontVariations = SmallVec<[usvg::FontVariation; 2]>;
type OfficeMathSvgFontInstance = (usvg::fontdb::ID, OfficeMathSvgFontVariations);

struct OfficeMathSvgFonts {
  database: Arc<usvg::fontdb::Database>,
  font_data: HashMap<usvg::fontdb::ID, Option<(Data, u32)>>,
  fonts: HashMap<OfficeMathSvgFontInstance, Option<Font>>,
  supported_axes: HashMap<usvg::fontdb::ID, SmallVec<[[u8; 4]; 2]>>,
}

impl OfficeMathSvgFonts {
  fn new(database: Arc<usvg::fontdb::Database>) -> Self {
    Self {
      database,
      font_data: HashMap::default(),
      fonts: HashMap::default(),
      supported_axes: HashMap::default(),
    }
  }

  fn retrieve(&mut self, span: &usvg::layout::Span, id: usvg::fontdb::ID) -> Option<Font> {
    let variations = self.resolve_variations(span, id);
    let key = (id, variations);
    if let Some(font) = self.fonts.get(&key) {
      return font.clone();
    }

    let font = self.data_for_face(id).and_then(|(font_data, index)| {
      let coordinates = key
        .1
        .iter()
        .map(|variation| (krilla::text::Tag::new(&variation.tag), variation.value))
        .collect::<SmallVec<[_; 2]>>();
      Font::new_variable(font_data, index, &coordinates)
    });
    self.fonts.insert(key, font.clone());
    font
  }

  fn data_for_face(&mut self, id: usvg::fontdb::ID) -> Option<(Data, u32)> {
    if let Some(data) = self.font_data.get(&id) {
      return data.clone();
    }

    let data = match self.database.face_source(id) {
      // In-memory font data already has stable shared ownership and can stay
      // zero-copy. File-backed data is read through fontdb's safe scoped API
      // and copied once: Krilla then yokes its parsed FontRef to these owned
      // bytes. This avoids both a persistent unsafe mmap and cloning the whole
      // usvg database through Arc::make_mut.
      Some((usvg::fontdb::Source::Binary(data), index)) => Some((data.into(), index)),
      Some(_) => self
        .database
        .with_face_data(id, |data, index| (data.to_vec().into(), index)),
      None => None,
    };
    self.font_data.insert(id, data.clone());
    data
  }

  fn resolve_variations(
    &mut self,
    span: &usvg::layout::Span,
    id: usvg::fontdb::ID,
  ) -> OfficeMathSvgFontVariations {
    let supported_axes = self.supported_axes.entry(id).or_insert_with(|| {
      self
        .database
        .with_face_data(id, |data, index| {
          SkrifaFontRef::from_index(data, index)
            .into_iter()
            .flat_map(|font| font.axes().iter())
            .map(|axis| axis.tag().to_be_bytes())
            .collect()
        })
        .unwrap_or_default()
    });
    let supports = |tag| supported_axes.contains(tag);
    let mut variations = span
      .variations
      .iter()
      .filter(|variation| supports(&variation.tag))
      .copied()
      .collect::<OfficeMathSvgFontVariations>();

    const OPTICAL_SIZE_TAG: &[u8; 4] = b"opsz";
    if span.font_optical_sizing == usvg::FontOpticalSizing::Auto
      && supports(OPTICAL_SIZE_TAG)
      && !variations
        .iter()
        .any(|variation| variation.tag == *OPTICAL_SIZE_TAG)
    {
      variations.push(usvg::FontVariation {
        tag: *OPTICAL_SIZE_TAG,
        value: span.font_size.get(),
      });
    }
    variations
  }
}

fn draw_office_math_text(surface: &mut Surface<'_>, tree: &usvg::Tree, size: Size) {
  let old_fill = surface.get_fill().cloned();
  let old_stroke = surface.get_stroke().cloned();
  surface.push_transform(&Transform::from_scale(
    size.width() / tree.size().width(),
    size.height() / tree.size().height(),
  ));
  let mut pop_count = 1;
  if let Some(viewport) = rect_path(0.0, 0.0, tree.size().width(), tree.size().height()) {
    surface.push_clip_path(&viewport, &FillRule::NonZero);
    pop_count += 1;
  }

  let mut fonts = OfficeMathSvgFonts::new(tree.fontdb().clone());
  draw_office_math_text_in_group(surface, tree.root(), &mut fonts, false);

  for _ in 0..pop_count {
    surface.pop();
  }
  surface.set_fill(old_fill);
  surface.set_stroke(old_stroke);
}

fn draw_office_math_text_in_group(
  surface: &mut Surface<'_>,
  group: &usvg::Group,
  fonts: &mut OfficeMathSvgFonts,
  parent_clipped: bool,
) {
  let clip_count = group.clip_path().map_or(0, |clip_path| {
    push_office_math_text_clip_path(surface, group, clip_path)
  });
  let clipped = parent_clipped || clip_count > 0;
  for child in group.children() {
    match child {
      usvg::Node::Group(group) => {
        draw_office_math_text_in_group(surface, group, fonts, clipped);
      }
      usvg::Node::Text(text) => {
        let Some(marker) = office_math_svg_text_marker(text.id()) else {
          continue;
        };
        draw_office_math_text_node(surface, text, marker, fonts, clipped);
      }
      usvg::Node::Path(_) | usvg::Node::Image(_) => {}
    }
  }
  for _ in 0..clip_count {
    surface.pop();
  }
}

fn push_office_math_text_clip_path(
  surface: &mut Surface<'_>,
  group: &usvg::Group,
  clip_path: &usvg::ClipPath,
) -> usize {
  let mut clip_paths = Vec::new();
  collect_office_math_text_clip_paths(group.abs_transform(), clip_path, &mut clip_paths);
  let count = clip_paths.len();
  for (path, rule) in clip_paths {
    surface.push_clip_path(&path, &rule);
  }
  count
}

fn collect_office_math_text_clip_paths(
  group_transform: usvg::Transform,
  clip_path: &usvg::ClipPath,
  paths: &mut Vec<(krilla::geom::Path, FillRule)>,
) {
  if let Some(parent) = clip_path.clip_path() {
    collect_office_math_text_clip_paths(group_transform, parent, paths);
  }

  let Some(rule) = office_math_text_clip_fill_rule(clip_path.root()) else {
    return;
  };
  let mut builder = PathBuilder::new();
  // A clip containing no drawable segment still clips all paint. Keeping a
  // degenerate subpath mirrors krilla-svg's PDF clip conversion instead of
  // silently dropping the OfficeMath semantic clip.
  builder.move_to(0.0, 0.0);
  let transform = group_transform.pre_concat(clip_path.transform());
  extend_office_math_text_clip_segments(clip_path.root(), transform, &mut builder);
  let path = builder.finish().or_else(|| {
    let mut fallback = PathBuilder::new();
    fallback.move_to(0.0, 0.0);
    fallback.line_to(0.0, 0.0);
    fallback.finish()
  });
  if let Some(path) = path {
    paths.push((path, rule));
  }
}

fn office_math_text_clip_fill_rule(group: &usvg::Group) -> Option<FillRule> {
  let mut rules = Vec::new();
  collect_office_math_text_clip_fill_rules(group, &mut rules);
  let rule = rules.first().copied().unwrap_or(usvg::FillRule::NonZero);
  if rules.iter().any(|candidate| *candidate != rule) {
    return None;
  }
  Some(match rule {
    usvg::FillRule::NonZero => FillRule::NonZero,
    usvg::FillRule::EvenOdd => FillRule::EvenOdd,
  })
}

fn collect_office_math_text_clip_fill_rules(group: &usvg::Group, rules: &mut Vec<usvg::FillRule>) {
  for child in group.children() {
    match child {
      usvg::Node::Path(path) => {
        if let Some(fill) = path.fill() {
          rules.push(fill.rule());
        }
      }
      usvg::Node::Group(group) => collect_office_math_text_clip_fill_rules(group, rules),
      usvg::Node::Text(text) => {
        collect_office_math_text_clip_fill_rules(text.flattened(), rules);
      }
      usvg::Node::Image(_) => {}
    }
  }
}

fn extend_office_math_text_clip_segments(
  group: &usvg::Group,
  transform: usvg::Transform,
  builder: &mut PathBuilder,
) {
  use usvg::tiny_skia_path::PathSegment;

  for child in group.children() {
    match child {
      usvg::Node::Path(path) if path.is_visible() => {
        for segment in path.data().segments() {
          match segment {
            PathSegment::MoveTo(mut point) => {
              transform.map_point(&mut point);
              builder.move_to(point.x, point.y);
            }
            PathSegment::LineTo(mut point) => {
              transform.map_point(&mut point);
              builder.line_to(point.x, point.y);
            }
            PathSegment::QuadTo(first, last) => {
              let mut points = [first, last];
              transform.map_points(&mut points);
              builder.quad_to(points[0].x, points[0].y, points[1].x, points[1].y);
            }
            PathSegment::CubicTo(first, second, last) => {
              let mut points = [first, second, last];
              transform.map_points(&mut points);
              builder.cubic_to(
                points[0].x,
                points[0].y,
                points[1].x,
                points[1].y,
                points[2].x,
                points[2].y,
              );
            }
            PathSegment::Close => builder.close(),
          }
        }
      }
      usvg::Node::Group(group) => extend_office_math_text_clip_segments(
        group,
        transform.pre_concat(group.transform()),
        builder,
      ),
      usvg::Node::Text(text) => {
        extend_office_math_text_clip_segments(text.flattened(), transform, builder);
      }
      usvg::Node::Path(_) | usvg::Node::Image(_) => {}
    }
  }
}

fn draw_office_math_text_node(
  surface: &mut Surface<'_>,
  text: &usvg::Text,
  marker: OfficeMathSvgTextMarker,
  fonts: &mut OfficeMathSvgFonts,
  clipped: bool,
) {
  // A semantic carrier is deliberately non-painting, but it must keep the
  // authored text paint. The internal SVG supplies an empty clip for that
  // purpose; never substitute a transparent style or let malformed unclipped
  // semantic text duplicate the visible MATH outline.
  if matches!(marker, OfficeMathSvgTextMarker::Semantic { .. }) && !clipped {
    return;
  }
  for span in text.layouted() {
    for positioned in &span.positioned_glyphs {
      let Some(font) = fonts.retrieve(span, positioned.font) else {
        continue;
      };
      let glyph_id = match marker {
        OfficeMathSvgTextMarker::Visible
        | OfficeMathSvgTextMarker::Semantic {
          exact_glyph_id: None,
        } => GlyphId::new(u32::from(positioned.id.0)),
        OfficeMathSvgTextMarker::Semantic {
          exact_glyph_id: Some(glyph_id),
        } => GlyphId::new(glyph_id),
      };
      let units_per_em = font.units_per_em();
      let transform =
        positioned
          .transform()
          .pre_concat(usvg::tiny_skia_path::Transform::from_scale(
            units_per_em / span.font_size.get(),
            units_per_em / span.font_size.get(),
          ));
      let text_transform = text.abs_transform();
      surface.push_transform(&Transform::from_row(
        text_transform.sx,
        text_transform.ky,
        text_transform.kx,
        text_transform.sy,
        text_transform.tx,
        text_transform.ty,
      ));
      surface.push_transform(&Transform::from_row(
        transform.sx,
        transform.ky,
        transform.kx,
        transform.sy,
        transform.tx,
        transform.ty,
      ));
      let Some(fill) = span.fill.as_ref() else {
        surface.pop();
        surface.pop();
        continue;
      };
      // The generated OfficeMath SVG has a closed solid-RGB paint domain.
      // Both visible and clipped semantic text retain the authored color and
      // opacity; the clip, not a paint mutation, controls semantic ink.
      let usvg::Paint::Color(color) = fill.paint() else {
        surface.pop();
        surface.pop();
        continue;
      };
      let fill = Fill {
        paint: rgb::Color::new(color.red, color.green, color.blue).into(),
        opacity: NormalizedF32::new(fill.opacity().get()).unwrap_or(NormalizedF32::ZERO),
        rule: match fill.rule() {
          usvg::FillRule::NonZero => FillRule::NonZero,
          usvg::FillRule::EvenOdd => FillRule::EvenOdd,
        },
      };
      surface.set_fill(Some(fill));
      surface.set_stroke(None);
      surface.draw_glyphs(
        Point::from_xy(0.0, 0.0),
        &[KrillaGlyph::new(
          glyph_id,
          0.0,
          0.0,
          0.0,
          0.0,
          0..positioned.text.len(),
          None,
        )],
        font,
        &positioned.text,
        span.font_size.get(),
        false,
      );
      surface.pop();
      surface.pop();
      if matches!(
        marker,
        OfficeMathSvgTextMarker::Semantic {
          exact_glyph_id: Some(_)
        }
      ) {
        return;
      }
    }
  }
}

fn draw_transformed_image_content(
  surface: &mut Surface<'_>,
  image: &ImageItem<'_>,
  draw: impl FnOnce(&mut Surface<'_>, Size),
) {
  if image.width_pt <= f32::EPSILON || image.height_pt <= f32::EPSILON {
    return;
  }
  let width = image.width_pt;
  let height = image.height_pt;
  let visible_width = 1.0 - image.crop.left - image.crop.right;
  let visible_height = 1.0 - image.crop.top - image.crop.bottom;
  if visible_width <= f32::EPSILON || visible_height <= f32::EPSILON {
    return;
  }
  let mut pop_count = 0;
  if let Some(clip) = path_from_commands(image.clip_path) {
    surface.push_clip_path(&clip, &krilla::paint::FillRule::NonZero);
    pop_count += 1;
  }

  surface.push_transform(&Transform::from_translate(image.x_pt, image.y_pt));
  pop_count += 1;

  if image.rotation_deg.abs() > f32::EPSILON {
    surface.push_transform(&Transform::from_rotate_at(
      image.rotation_deg,
      width / 2.0,
      height / 2.0,
    ));
    pop_count += 1;
  }

  if image.crop != ImageCrop::default()
    && let Some(clip) = rect_path(0.0, 0.0, width, height)
  {
    // An uncropped bitmap already paints exactly inside its unit-square
    // XObject bounds. Avoid wrapping every ordinary image in a redundant
    // rectangular clip: PowerPoint's fixed output likewise emits the direct
    // image transform, while SVG keeps its own view-box clip in draw_svg().
    // A non-default source rectangle can extend the scaled source outside the
    // authored picture frame and therefore still requires this clip.
    surface.push_clip_path(&clip, &krilla::paint::FillRule::NonZero);
    pop_count += 1;
  }

  if image.flip_horizontal {
    surface.push_transform(&Transform::from_translate(width, 0.0));
    surface.push_transform(&Transform::from_scale(-1.0, 1.0));
    pop_count += 2;
  }
  if image.flip_vertical {
    surface.push_transform(&Transform::from_translate(0.0, height));
    surface.push_transform(&Transform::from_scale(1.0, -1.0));
    pop_count += 2;
  }

  let draw_width = width / visible_width;
  let draw_height = height / visible_height;
  if let Some(size) = Size::from_wh(draw_width, draw_height) {
    surface.push_transform(&Transform::from_translate(
      -image.crop.left * draw_width,
      -image.crop.top * draw_height,
    ));
    draw(surface, size);
    surface.pop();
  }

  for _ in 0..pop_count {
    surface.pop();
  }
}

fn embed_attachments(pdf: &mut Document, options: &PdfOptions) -> Result<()> {
  for attachment in &options.attachments {
    if attachment.path.is_empty() {
      return Err(PdfError::Options(
        "attachment path must not be empty".to_string(),
      ));
    }
    if attachment.description.is_empty() {
      return Err(PdfError::Options(format!(
        "attachment '{}' must have a description",
        attachment.path
      )));
    }
    let mime_type = MimeType::new(&attachment.mime_type).ok_or_else(|| {
      PdfError::Options(format!(
        "attachment '{}' has invalid MIME type '{}'",
        attachment.path, attachment.mime_type
      ))
    })?;
    let file = EmbeddedFile {
      path: attachment.path.clone(),
      mime_type: Some(mime_type),
      description: Some(attachment.description.clone()),
      association_kind: match attachment.association {
        PdfAttachmentAssociation::Source => AssociationKind::Source,
        PdfAttachmentAssociation::Data => AssociationKind::Data,
        PdfAttachmentAssociation::Alternative => AssociationKind::Alternative,
        PdfAttachmentAssociation::Supplement => AssociationKind::Supplement,
        PdfAttachmentAssociation::Unspecified => AssociationKind::Unspecified,
      },
      data: attachment.data.as_ref().to_vec().into(),
      modification_date: attachment.modification_date.map(pdf_date_time),
      compress: attachment.compress,
      location: None,
    };
    pdf.embed_file(file).ok_or_else(|| {
      PdfError::Options(format!(
        "attachment path '{}' is present more than once",
        attachment.path
      ))
    })?;
  }
  Ok(())
}

fn pdf_date_time(value: PdfDateTime) -> DateTime {
  let mut date = DateTime::new(value.year);
  if let Some(month) = value.month {
    date = date.month(month);
  }
  if let Some(day) = value.day {
    date = date.day(day);
  }
  if let Some(hour) = value.hour {
    date = date.hour(hour);
  }
  if let Some(minute) = value.minute {
    date = date.minute(minute);
  }
  if let Some(second) = value.second {
    date = date.second(second);
  }
  if let Some(offset_hour) = value.utc_offset_hour {
    date = date.utc_offset_hour(offset_hour);
  }
  if let Some(offset_minute) = value.utc_offset_minute {
    date = date.utc_offset_minute(offset_minute);
  }
  date
}

fn register_named_destinations(
  pdf: &mut Document,
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<()> {
  if !options.links.export_bookmarks_to_pdf_destinations {
    return Ok(());
  }
  for anchor in &document.anchor_pages {
    if anchor.name.is_empty() || anchor.page_index >= document.pages.len() {
      continue;
    }
    let destination = NamedDestination::new(
      anchor.name.to_string(),
      XyzDestination::new(anchor.page_index, Point::from_xy(0.0, 0.0)),
    );
    pdf.register_named_destination(destination).ok_or_else(|| {
      PdfError::Options(format!(
        "bookmark '{}' resolves to more than one PDF destination",
        anchor.name
      ))
    })?;
  }
  Ok(())
}

fn page_label(document: &common::LayoutDocument<'static>, page_index: usize) -> Option<PageLabel> {
  let page = document.pages.get(page_index)?;
  let physical_page_number = page_index.saturating_add(1);
  let virtual_page_number = page
    .setup
    .page_number_start
    .and_then(|start| {
      i64::from(start)
        .checked_add(i64::try_from(page.section_page_index).ok()?)
        .and_then(|number| u32::try_from(number).ok())
    })
    .or_else(|| {
      document
        .anchor_pages
        .iter()
        .find(|anchor| anchor.page_index == page_index)
        .and_then(|anchor| u32::try_from(anchor.virtual_page_number).ok())
        .filter(|number| usize::try_from(*number).ok() != Some(physical_page_number))
    })?;
  Some(PageLabel::new(
    Some(NumberingStyle::Arabic),
    None,
    NonZeroU32::new(virtual_page_number),
  ))
}

fn pdf_metadata(options: &PdfOptions) -> Metadata {
  let mut metadata = Metadata::new();
  if let Some(title) = &options.metadata.title {
    metadata = metadata.title(title.clone());
  }
  if let Some(author) = &options.metadata.author {
    metadata = metadata.authors(vec![author.clone()]);
  }
  if let Some(subject) = &options.metadata.subject {
    metadata = metadata.description(subject.clone());
  }
  if let Some(keywords) = &options.metadata.keywords {
    let keywords = keywords
      .split([',', ';'])
      .map(str::trim)
      .filter(|keyword| !keyword.is_empty())
      .map(str::to_string)
      .collect::<Vec<_>>();
    metadata = metadata.keywords(keywords);
  }
  if let Some(creator) = &options.metadata.creator {
    metadata = metadata.creator(creator.clone());
  }
  if let Some(producer) = &options.metadata.producer {
    metadata = metadata.producer(producer.clone());
  }
  if let Some(language) = options.canonical_document_language() {
    metadata = metadata.language(language);
  }
  metadata
}

fn shaped_pdf_glyphs(
  text: &str,
  style: &TextStyle<'_>,
  word_spacing_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Option<PaintGlyphRun> {
  let shaped = text_metrics.shape_text(text, style)?;
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
    let is_word_space = text
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
        glyph_id: GlyphId::new(glyph.glyph_id),
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

fn text_vertical_scale(style: &TextStyle<'_>) -> f32 {
  if style.bold
    && (style.font_size_pt - LEGACY_ARIAL_BOLD_FONT_SIZE_PT).abs()
      < LEGACY_ARIAL_BOLD_FONT_SIZE_TOLERANCE_PT
    && style
      .font_family
      .as_deref()
      .is_some_and(|family| family.eq_ignore_ascii_case("Arial"))
  {
    LEGACY_ARIAL_BOLD_VERTICAL_SCALE
  } else {
    1.0
  }
}

fn rect_path(x: f32, y: f32, width: f32, height: f32) -> Option<krilla::geom::Path> {
  let mut path = PathBuilder::new();
  path.move_to(x, y);
  path.line_to(x + width, y);
  path.line_to(x + width, y + height);
  path.line_to(x, y + height);
  path.close();
  path.finish()
}

fn fill(style: &TextStyle<'_>) -> Fill {
  Fill {
    paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
    opacity: NormalizedF32::new(style.opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
    rule: Default::default(),
  }
}

fn text_outline_fill(
  surface: &mut Surface<'_>,
  fill: &common::Fill<'static>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) -> Option<Fill> {
  let resolved_fill = resolved_text_outline_common_fill(fill, x_pt, y_pt, width_pt, height_pt);
  let path = PolylineItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    points: &[],
    commands: &[],
    closed: true,
    fill: &resolved_fill,
    stroke: None,
  };
  path_fill_from_common(surface, &resolved_fill, &path)
}

fn text_stroke_from_common(
  surface: &mut Surface<'_>,
  stroke: &common::Stroke<'static>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) -> Stroke {
  let path = PolylineItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    points: &[],
    commands: &[],
    closed: true,
    fill: &common::Fill::None,
    stroke: Some(stroke),
  };
  path_stroke_from_common(surface, stroke, &path)
}

fn resolved_text_outline_common_fill(
  fill: &common::Fill<'static>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) -> common::Fill<'static> {
  let mut resolved_fill = fill.clone();
  if let common::Fill::Gradient(gradient) = &mut resolved_fill {
    let unresolved = gradient.definition_bounds.is_none();
    let bounds = common::Rect {
      origin: common::Point {
        x: common::Pt(x_pt),
        y: common::Pt(y_pt),
      },
      size: common::Size {
        width: common::Pt(width_pt),
        height: common::Pt(height_pt),
      },
    };
    gradient.definition_bounds.get_or_insert(bounds);
    if let Some(path) = &mut gradient.path
      && unresolved
    {
      let normalized = path.transform;
      path.transform = common::Transform {
        m11: width_pt * normalized.m11,
        m12: height_pt * normalized.m12,
        m21: width_pt * normalized.m21,
        m22: height_pt * normalized.m22,
        dx: common::Pt(x_pt + width_pt * normalized.dx.0),
        dy: common::Pt(y_pt + height_pt * normalized.dy.0),
      };
      if path.kind == common::GradientPathKind::Circle {
        path.transform = common::office_circle_gradient_transform(path.transform);
      }
    }
  }
  resolved_fill
}

fn stroke(style: &TextStyle<'_>) -> Option<Stroke> {
  let color = style.outline_color?;
  if style.outline_width_pt <= f32::EPSILON {
    return None;
  }
  Some(Stroke {
    width: style.outline_width_pt,
    paint: rgb::Color::new(color.r, color.g, color.b).into(),
    opacity: NormalizedF32::new(style.outline_opacity.clamp(0.0, 1.0))
      .unwrap_or(NormalizedF32::ZERO),
    ..Default::default()
  })
}

fn text_stroke_with_fill(
  style: &TextStyle<'_>,
  synthetic_bold: bool,
  rendered_font_size_pt: f32,
  outline_stroke: Option<Stroke>,
  outline_fill: Option<Fill>,
) -> Option<Stroke> {
  // LibreOffice's PDF writer uses fill-then-stroke for an artificial bold
  // face, with a stroke width of one thirtieth of the font height. An
  // explicit text outline wins over artificial bold there as well.
  outline_stroke
    .or_else(|| {
      outline_fill
        .filter(|_| style.outline_width_pt > f32::EPSILON)
        .map(|fill| Stroke {
          width: style.outline_width_pt,
          paint: fill.paint,
          opacity: fill.opacity,
          ..Default::default()
        })
    })
    .or_else(|| stroke(style))
    .or_else(|| {
      synthetic_bold.then(|| Stroke {
        width: rendered_font_size_pt / 30.0,
        paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
        opacity: NormalizedF32::new(style.opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
        ..Default::default()
      })
    })
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;
  use std::sync::Arc;

  use super::{
    FollowFrameKind, GlyphId, ImageCrop, ImageItem, OfficeMathSvgTextMarker, PageItem,
    PaintDocument, PaintItem, PaintLineOwner, PaintTextPortionKind, TextItem, TextMetrics,
    TextStyle as PaintTextStyle, common_writer_line_baselines, conversion_font_audit,
    draw_office_math_text, gamma_correct_gradient_color, localized_metafile_ui_font_family,
    metafile_render_options_for_image, office_math_semantic_glyph_id, office_math_svg_text_marker,
    pdf_metadata, pdf_page_dimension, render, semantic_advance_for_text_range, shaped_pdf_glyphs,
    source_range_requires_visible_glyph, stroke_end_dimensions, symbol_font_semantic_text,
    synthetic_italic_text_transform, text_portion_ranges, text_requires_glyph_outlines,
    text_stroke_with_fill, text_style_from_common, visually_ordered_text_portion_ranges,
    word_small_caps_semantic_text, word_unsigned_signature_line_items, writer_item_line_metrics,
  };
  use crate::options::{PdfAttachment, PdfAttachmentAssociation, PdfOptions};
  use krilla::Document;
  use krilla::geom::Size;
  use krilla::page::PageSettings;
  use ooxmlsdk_layout::common::{
    self, Color, DisplayItem, DisplayPage, LayoutDocument, LayoutEngineKind, Pt, TextRun, TextStyle,
  };
  use ooxmlsdk_layout::fonts::FontStyleRef;

  #[test]
  fn office_math_semantic_marker_carries_an_exact_glyph_id() {
    assert_eq!(
      office_math_svg_text_marker("ooxmlsdk-math-visible-16"),
      Some(OfficeMathSvgTextMarker::Visible)
    );
    assert_eq!(
      office_math_svg_text_marker("ooxmlsdk-math-semantic-17"),
      Some(OfficeMathSvgTextMarker::Semantic {
        exact_glyph_id: None
      })
    );
    assert_eq!(
      office_math_semantic_glyph_id("ooxmlsdk-math-semantic-17-gid-3542"),
      Some(3542)
    );
    assert_eq!(
      office_math_semantic_glyph_id("ooxmlsdk-math-semantic-x-gid-3542"),
      None
    );
    assert_eq!(
      office_math_semantic_glyph_id("ooxmlsdk-math-semantic-17-gid-x"),
      None
    );
    assert_eq!(office_math_semantic_glyph_id("unrelated-gid-3542"), None);
  }

  #[test]
  fn office_math_semantic_text_keeps_authored_paint_beneath_its_clip() {
    let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20">
      <defs><clipPath id="math-semantic-clip" clipPathUnits="userSpaceOnUse"><path d="M0 0L0 0"/></clipPath></defs>
      <g clip-path="url(#math-semantic-clip)"><text id="ooxmlsdk-math-semantic-1" visibility="hidden" x="2" y="14" font-family="DejaVu Sans" font-size="12" fill="#00000a" fill-opacity="1">x</text></g>
    </svg>"##;
    let mut svg_options = usvg::Options::default();
    svg_options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_data(svg, &svg_options).unwrap();

    let page_size = Size::from_wh(20.0, 20.0).unwrap();
    let mut document = Document::new();
    let mut page = document.start_page_with(PageSettings::new(page_size));
    let mut surface = page.surface();
    draw_office_math_text(&mut surface, &tree, page_size);
    surface.finish();
    page.finish();

    let bytes = document.finish().unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let page_id = pdf.get_pages()[&1];
    let page = pdf.get_dictionary(page_id).unwrap();
    let resources = resolved_dictionary(&pdf, page.get(b"Resources").unwrap());
    assert!(resources.get(b"ExtGState").is_err());

    let content = pdf.get_page_content(page_id);
    let operations = lopdf::content::Content::decode(&content)
      .unwrap()
      .operations;
    assert!(operations.iter().any(|operation| operation.operator == "W"));
    assert!(operations.iter().any(|operation| operation.operator == "n"));
    assert!(
      operations
        .iter()
        .any(|operation| operation.operator == "BT")
    );
    assert!(
      operations
        .iter()
        .any(|operation| operation.operator == "Tj")
    );
    assert!(
      !operations
        .iter()
        .any(|operation| operation.operator == "gs")
    );
    assert!(operations.iter().any(|operation| {
      operation.operator == "rg"
        && operation.operands.len() == 3
        && operation.operands[0]
          .as_float()
          .is_ok_and(|value| value.abs() < 0.000_001)
        && operation.operands[1]
          .as_float()
          .is_ok_and(|value| value.abs() < 0.000_001)
        && operation.operands[2]
          .as_float()
          .is_ok_and(|value| (value - 10.0 / 255.0).abs() < 0.000_001)
    }));
  }

  #[test]
  fn semantic_character_advances_follow_utf8_glyph_clusters() {
    let text = "Aωfi";
    let advances = [1.0, 2.0, 3.0, 4.0];

    assert_eq!(
      semantic_advance_for_text_range(text, &advances, &(1..3)),
      Some(2.0)
    );
    assert_eq!(
      semantic_advance_for_text_range(text, &advances, &(3..5)),
      Some(7.0)
    );
    assert_eq!(
      semantic_advance_for_text_range(text, &advances, &(5..6)),
      None
    );
  }

  #[test]
  fn localized_native_metafile_ui_text_uses_the_simplified_chinese_ui_face() {
    assert_eq!(
      localized_metafile_ui_font_family(Some("Segoe UI".to_string()), Some("zh-CN"), true),
      Some("Microsoft YaHei UI".to_string())
    );
    assert_eq!(
      localized_metafile_ui_font_family(Some("Segoe UI".to_string()), Some("en-US"), true),
      Some("Segoe UI".to_string())
    );
    assert_eq!(
      localized_metafile_ui_font_family(Some("Segoe UI".to_string()), Some("zh-CN"), false),
      Some("Segoe UI".to_string())
    );
    assert_eq!(
      localized_metafile_ui_font_family(Some("Arial".to_string()), Some("zh-CN"), true),
      Some("Arial".to_string())
    );
  }

  #[test]
  fn word_unsigned_signature_line_uses_host_geometry_and_ooxml_metadata() {
    let properties = common::SignatureLineProperties {
      show_sign_date: true,
      suggested_signer: Some(Cow::Borrowed("John Doe")),
      suggested_signer_title: Some(Cow::Borrowed("Farmer")),
      ..Default::default()
    };

    let items =
      word_unsigned_signature_line_items(10.0, 20.0, 192.0, 96.0, &properties, &[], Some("zh-CN"));

    assert_eq!(items.len(), 5);
    let PageItem::Text(x) = &items[2] else {
      panic!("signature X text");
    };
    assert_eq!(x.text, "X");
    assert!((x.x_pt - 17.10).abs() < 0.001);
    assert!((x.y_pt - 65.70).abs() < 0.001);
    assert!((x.style.font_size_pt - 12.0).abs() < 0.001);
    assert_eq!(x.style.font_family.as_deref(), Some("Arial"));

    let PageItem::Text(signer) = &items[3] else {
      panic!("signature signer text");
    };
    assert_eq!(signer.text, "John Doe");
    assert!((signer.x_pt - 23.85).abs() < 0.001);
    assert!((signer.y_pt - 81.42).abs() < 0.001);
    assert_eq!(
      signer.style.font_family.as_deref(),
      Some("Microsoft YaHei UI")
    );

    let empty = word_unsigned_signature_line_items(
      0.0,
      0.0,
      192.0,
      96.0,
      &common::SignatureLineProperties::default(),
      &[],
      Some("en-US"),
    );
    assert_eq!(empty.len(), 3, "empty metadata must not emit empty runs");
  }

  #[test]
  fn drawingml_marker_dimensions_use_libreoffice_mm100_minimum_baseline() {
    let marker = common::StrokeEnd {
      kind: common::StrokeEndKind::Triangle,
      width: common::StrokeEndSize::Medium,
      length: common::StrokeEndSize::Medium,
    };

    let (thin_width, thin_length) = stroke_end_dimensions(marker, 0.75);
    let libreoffice_medium_minimum = 3.0 * 70.0 * 72.0 / 2_540.0;
    assert!((thin_width - libreoffice_medium_minimum).abs() < 0.000_1);
    assert!((thin_length - libreoffice_medium_minimum).abs() < 0.000_1);

    let (thick_width, thick_length) = stroke_end_dimensions(marker, 3.0);
    assert_eq!((thick_width, thick_length), (9.0, 9.0));
  }

  #[test]
  fn medium_open_arrow_uses_centerline_dimensions_for_the_stroked_envelope() {
    let marker = common::StrokeEnd {
      kind: common::StrokeEndKind::Arrow,
      width: common::StrokeEndSize::Medium,
      length: common::StrokeEndSize::Medium,
    };

    let (thin_width, thin_length) = stroke_end_dimensions(marker, 0.75);
    assert!((thin_width - 7.594_401).abs() < 0.000_1);
    assert!((thin_length - 6.515_256).abs() < 0.000_1);

    let (two_point_width, two_point_length) = stroke_end_dimensions(marker, 2.0);
    assert_eq!((two_point_width, two_point_length), (7.0, 7.0));
  }

  #[test]
  fn metafile_raster_size_uses_office_fixed_output_density() {
    let image = ImageItem {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: 72.0,
      height_pt: 36.0,
      crop: ImageCrop {
        left: 0.25,
        right: 0.25,
        ..ImageCrop::default()
      },
      clip_path: &[],
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      data: Cow::Borrowed(&[]),
      content_type: Some(Cow::Borrowed("image/emf")),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
      metafile_native_size: true,
    };
    let fixed_output = metafile_render_options_for_image(&image, &PdfOptions::default());

    assert_eq!(fixed_output.target_width_px, Some(400));
    assert_eq!(fixed_output.target_height_px, Some(100));
    assert!(!fixed_output.transparent_background);

    let mut pdf_options = PdfOptions::default();
    pdf_options.images.reduce_resolution = true;
    let reduced = metafile_render_options_for_image(&image, &pdf_options);
    assert_eq!(reduced.target_width_px, Some(600));
    assert_eq!(reduced.target_height_px, Some(150));

    let mut vml_preview = image;
    vml_preview.width_pt = 77.25;
    vml_preview.height_pt = 49.5;
    vml_preview.crop = ImageCrop::default();
    vml_preview.metafile_background_color = Some([255, 0, 0]);
    let vml = metafile_render_options_for_image(&vml_preview, &PdfOptions::default());
    assert_eq!(vml.target_width_px, Some(214));
    assert_eq!(vml.target_height_px, Some(137));
    assert!(vml.transparent_background);
    assert_eq!(vml.background_color, None);

    vml_preview.width_pt = 14.76;
    vml_preview.height_pt = 26.76;
    let printer_grid_vml = metafile_render_options_for_image(&vml_preview, &PdfOptions::default());
    assert_eq!(printer_grid_vml.target_width_px, Some(41));
    assert_eq!(printer_grid_vml.target_height_px, Some(74));

    vml_preview.width_pt = 14.759;
    let below_printer_grid =
      metafile_render_options_for_image(&vml_preview, &PdfOptions::default());
    assert_eq!(below_printer_grid.target_width_px, Some(40));
  }

  #[test]
  fn word_line_segmentation_isolates_breakable_hyphens() {
    let item = TextItem {
      x_pt: 0.0,
      y_pt: 0.0,
      line_height_pt: 12.0,
      line_metrics_participant: true,
      paint_clip: None,
      text: "non-business".into(),
      style: PaintTextStyle::default(),
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: common::PdfTextSegmentation::WordLine,
      source_path: None,
      semantic_target_width_pt: None,
    };

    let ranges = text_portion_ranges(&item)
      .into_iter()
      .map(|(_, range)| range)
      .collect::<Vec<_>>();
    assert_eq!(ranges, vec![0..3, 3..4, 4..12]);
  }

  #[test]
  fn word_shared_baseline_ignores_painted_nonparticipating_blank_run() {
    let text_item =
      |text: &'static str, font_size_pt: f32, line_metrics_participant: bool| TextItem {
        x_pt: 0.0,
        y_pt: 0.0,
        line_height_pt: font_size_pt * 1.15,
        line_metrics_participant,
        paint_clip: None,
        text: text.into(),
        style: PaintTextStyle {
          font_size_pt,
          ..PaintTextStyle::default()
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
      };
    let visible = text_item("U+0020", 12.0, true);
    let ignored_blank = text_item(" ", 48.0, false);
    let mut text_metrics = TextMetrics::new();
    let expected = writer_item_line_metrics(
      &PageItem::Text(Box::new(visible.clone())),
      &mut text_metrics,
    )
    .expect("visible label participates in the shared baseline")
    .baseline_offset_pt();
    assert!(
      writer_item_line_metrics(
        &PageItem::Text(Box::new(ignored_blank.clone())),
        &mut text_metrics,
      )
      .is_none(),
      "the blank remains paintable without contributing its 48pt font box",
    );

    let owner = Some(PaintLineOwner {
      frame_index: 0,
      line_index: 0,
      frame_kind: FollowFrameKind::Paragraph,
      clip: None,
    });
    let baselines = common_writer_line_baselines(
      &[
        PageItem::Text(Box::new(visible)),
        PageItem::Text(Box::new(ignored_blank)),
      ],
      &[owner, owner],
      &mut text_metrics,
    );
    assert_eq!(baselines.len(), 2);
    for baseline in baselines {
      assert!((baseline.expect("shared paragraph baseline") - expected).abs() < 0.001);
    }
  }

  #[test]
  fn odd_bidi_word_line_portions_follow_visual_order() {
    // Comment066.docx contains this exact directionally uniform w:rtl
    // fragment. The source ranges stay logical while their paint order is
    // reversed as one level-1 sequence.
    let text = " قبرص /افب-تصز";
    let rtl_item = TextItem {
      x_pt: 0.0,
      y_pt: 0.0,
      line_height_pt: 12.0,
      line_metrics_participant: true,
      paint_clip: None,
      text: text.into(),
      style: PaintTextStyle {
        resolved_bidi_level: Some(1),
        ..PaintTextStyle::default()
      },
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: true,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: common::PdfTextSegmentation::WordLine,
      source_path: None,
      semantic_target_width_pt: None,
    };

    let rtl_ranges = visually_ordered_text_portion_ranges(&rtl_item)
      .into_iter()
      .map(|(_, range)| range)
      .collect::<Vec<_>>();
    assert_eq!(rtl_ranges, vec![18..text.len(), 17..18, 0..17]);

    let ltr_item = TextItem {
      style: PaintTextStyle {
        resolved_bidi_level: Some(2),
        ..rtl_item.style.clone()
      },
      ..rtl_item
    };
    let ltr_ranges = visually_ordered_text_portion_ranges(&ltr_item)
      .into_iter()
      .map(|(_, range)| range)
      .collect::<Vec<_>>();
    assert_eq!(ltr_ranges, vec![0..17, 17..18, 18..text.len()]);
  }

  #[test]
  fn office_tab_leaders_split_into_thirty_two_character_pdf_portions() {
    let item = TextItem {
      x_pt: 0.0,
      y_pt: 0.0,
      line_height_pt: 12.0,
      line_metrics_participant: true,
      paint_clip: None,
      text: ".".repeat(70).into(),
      style: PaintTextStyle::default(),
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: true,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: common::PdfTextSegmentation::Portion,
      source_path: None,
      semantic_target_width_pt: None,
    };

    let ranges = text_portion_ranges(&item)
      .into_iter()
      .map(|(_, range)| range)
      .collect::<Vec<_>>();
    assert_eq!(ranges, vec![0..32, 32..64, 64..70]);
  }

  #[test]
  fn decorated_tab_remains_a_non_painting_tab_portion() {
    let item = TextItem {
      x_pt: 0.0,
      y_pt: 0.0,
      line_height_pt: 12.0,
      line_metrics_participant: true,
      paint_clip: None,
      text: "\t".into(),
      style: PaintTextStyle {
        underline: true,
        ..PaintTextStyle::default()
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
    };

    assert_eq!(
      text_portion_ranges(&item).as_slice(),
      [(PaintTextPortionKind::Tab, 0..1)]
    );
  }

  #[test]
  fn translucent_office_text_is_painted_as_glyph_outlines() {
    let mut style = PaintTextStyle::default();
    assert!(!text_requires_glyph_outlines(&style));

    style.opacity = 0.74;
    assert!(text_requires_glyph_outlines(&style));

    style.opacity = 0.0;
    assert!(!text_requires_glyph_outlines(&style));

    style.pdf_glyph_outlines = true;
    assert!(text_requires_glyph_outlines(&style));

    style.opacity = 0.74;
    style.semantic_only = true;
    assert!(!text_requires_glyph_outlines(&style));
  }

  fn collect_structure_roles(
    pdf: &lopdf::Document,
    object: &lopdf::Object,
    roles: &mut Vec<Vec<u8>>,
  ) {
    let object = match object {
      lopdf::Object::Reference(id) => pdf.get_object(*id).unwrap(),
      object => object,
    };
    match object {
      lopdf::Object::Array(children) => {
        for child in children {
          collect_structure_roles(pdf, child, roles);
        }
      }
      lopdf::Object::Dictionary(dictionary) => {
        if let Ok(role) = dictionary.get(b"S").and_then(lopdf::Object::as_name) {
          roles.push(role.to_vec());
        }
        if let Ok(children) = dictionary.get(b"K") {
          collect_structure_roles(pdf, children, roles);
        }
      }
      _ => {}
    }
  }

  fn resolved_dictionary<'a>(
    pdf: &'a lopdf::Document,
    object: &'a lopdf::Object,
  ) -> &'a lopdf::Dictionary {
    match object {
      lopdf::Object::Reference(id) => pdf.get_dictionary(*id).unwrap(),
      lopdf::Object::Dictionary(dictionary) => dictionary,
      object => panic!("expected PDF dictionary, got {object:?}"),
    }
  }

  fn tagged_test_document() -> LayoutDocument<'static> {
    let text = TextRun {
      text: "Tagged paragraph".into(),
      origin: common::Point {
        x: Pt(12.0),
        y: Pt(24.0),
      },
      line_height: Pt(14.0),
      line_metrics_participant: true,
      paint_clip: None,
      style: TextStyle {
        font_family: Some("Arial".into()),
        font_size: Pt(11.0),
        ..TextStyle::default()
      },
      font_id: None,
      color: Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      rotation_center: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      pdf_text_segmentation: Default::default(),
      source: None,
    };
    let page_size = common::Size {
      width: Pt(200.0),
      height: Pt(100.0),
    };
    LayoutDocument {
      pages: vec![DisplayPage {
        setup: common::PageSetup {
          size: page_size,
          ..Default::default()
        },
        bounds: common::Rect {
          origin: Default::default(),
          size: page_size,
        },
        items: vec![DisplayItem::Text(text)],
        ..Default::default()
      }],
      outline_entries: vec![common::OutlineEntry {
        level: 0,
        text: "Document".into(),
        page_index: 0,
        target: common::Point::default(),
        merged_hidden_separator: false,
      }],
      ..Default::default()
    }
  }

  #[test]
  fn font_audit_distinguishes_missing_text_from_explicit_symbol_notdef() {
    let mut document = tagged_test_document();
    let DisplayItem::Text(text) = &mut document.pages[0].items[0] else {
      unreachable!();
    };
    text.style.color = Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    };

    let mut text_metrics = TextMetrics::new();
    let mut paint = PaintDocument::from_layout(&document, &mut text_metrics, None);
    let PaintItem::Text(text) = &mut paint.pages[0].items[0] else {
      unreachable!();
    };
    text.portions[0].glyphs.as_mut().expect("shaped test text")[0].glyphs[0].glyph_id =
      GlyphId::new(0);

    let audit = conversion_font_audit(&paint);
    let issue = audit
      .issues
      .iter()
      .find(|issue| issue.kind == crate::PdfFontAuditIssueKind::MissingGlyph)
      .unwrap_or_else(|| {
        panic!(
          "glyph zero must be reported at the layout-to-PDF boundary: {:#?}",
          audit
        )
      });

    assert!(issue.detail.contains("requested_family=Some(\"Arial\")"));
    assert!(issue.detail.contains("text=\"T\""));

    let mut symbol_document = tagged_test_document();
    let DisplayItem::Text(symbol_text) = &mut symbol_document.pages[0].items[0] else {
      unreachable!();
    };
    symbol_text.text = "\u{f081}".into();
    symbol_text.style.font_family = Some("UniversalMath1 BT".into());
    symbol_text.style.symbol_font_family = Some("UniversalMath1 BT".into());
    symbol_text.style.explicit_symbol_character = true;
    symbol_text.style.wordprocessingml_font_slots = false;
    symbol_text.style.color = Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    };

    let mut text_metrics = TextMetrics::new();
    let mut symbol_paint = PaintDocument::from_layout(&symbol_document, &mut text_metrics, None);
    let PaintItem::Text(symbol_text) = &mut symbol_paint.pages[0].items[0] else {
      unreachable!();
    };
    symbol_text.portions[0]
      .glyphs
      .as_mut()
      .expect("shaped explicit symbol text")[0]
      .glyphs[0]
      .glyph_id = GlyphId::new(0);

    let symbol_audit = conversion_font_audit(&symbol_paint);
    assert_eq!(symbol_audit.explicit_symbol_notdef_glyph_count, 1);
    assert!(
      symbol_audit
        .issues
        .iter()
        .all(|issue| issue.kind != crate::PdfFontAuditIssueKind::MissingGlyph),
      "explicit symbol .notdef must remain visible without becoming a font-integrity failure: {symbol_audit:#?}"
    );
  }

  #[test]
  fn font_audit_ignores_notdef_for_non_rendering_control_clusters() {
    assert!(!source_range_requires_visible_glyph("\n", &(0..1)));
    assert!(!source_range_requires_visible_glyph("\t\r", &(0..2)));
    assert!(source_range_requires_visible_glyph("\nT", &(0..2)));
    assert!(source_range_requires_visible_glyph("\u{e000}", &(0..3)));
  }

  fn pdf_info_text(object: &lopdf::Object) -> String {
    let bytes = object.as_str().unwrap();
    if let Some(bytes) = bytes.strip_prefix(&[0xfe, 0xff]) {
      let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
      String::from_utf16(&units).unwrap()
    } else {
      String::from_utf8(bytes.to_vec()).unwrap()
    }
  }

  #[test]
  fn pdf_metadata_options_reach_document_info() {
    let mut options = PdfOptions {
      ui_language: Some("zh-CN".to_string()),
      ..PdfOptions::default()
    };
    options.metadata.title = Some("标题".to_string());
    options.metadata.author = Some("作者".to_string());
    options.metadata.subject = Some("主题".to_string());
    options.metadata.keywords = Some("alpha, beta; gamma".to_string());
    options.metadata.creator = Some("creator".to_string());
    options.metadata.producer = Some("producer".to_string());
    let mut document = Document::new();
    document.set_metadata(pdf_metadata(&options));
    let settings = PageSettings::new(Size::from_wh(10.0, 10.0).unwrap());
    document.start_page_with(settings).finish();
    let bytes = document.finish().unwrap();
    let parsed = lopdf::Document::load_mem(&bytes).unwrap();
    let info_id = parsed.trailer.get(b"Info").unwrap().as_reference().unwrap();
    let info = parsed.get_dictionary(info_id).unwrap();

    assert_eq!(pdf_info_text(info.get(b"Title").unwrap()), "标题");
    assert_eq!(pdf_info_text(info.get(b"Author").unwrap()), "作者");
    assert_eq!(pdf_info_text(info.get(b"Subject").unwrap()), "主题");
    assert_eq!(info.get(b"Creator").unwrap().as_str().unwrap(), b"creator");
    assert_eq!(
      info.get(b"Producer").unwrap().as_str().unwrap(),
      b"producer"
    );
  }

  #[test]
  fn tagged_pdf_emits_language_and_paragraph_structure() {
    let document = tagged_test_document();
    let mut options = PdfOptions::default();
    options.general.tagged_pdf = true;
    options.ui_language = Some("en-US".to_string());

    let bytes = render(&document, &options).unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let catalog_id = pdf.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let catalog = pdf.get_dictionary(catalog_id).unwrap();
    assert_eq!(catalog.get(b"Lang").unwrap().as_str().unwrap(), b"en-US");
    let mark_info = catalog.get(b"MarkInfo").unwrap().as_dict().unwrap();
    assert!(mark_info.get(b"Marked").unwrap().as_bool().unwrap());

    let structure_root = catalog.get(b"StructTreeRoot").unwrap();
    let mut roles = Vec::new();
    collect_structure_roles(&pdf, structure_root, &mut roles);
    assert!(roles.iter().any(|role| role == b"Document"));
    assert!(roles.iter().any(|role| role == b"Part"));
    assert!(roles.iter().any(|role| role == b"P"));
  }

  #[test]
  fn identity_leaf_group_stays_in_the_page_content_stream() {
    let mut document = tagged_test_document();
    let text = document.pages[0].items.remove(0);
    document.pages[0]
      .items
      .push(DisplayItem::Group(common::CompositingGroup {
        mask: None,
        clip: None,
        transform: None,
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        flatten_identity: true,
        inherit_text_line_owner: true,
        items: vec![text],
      }));

    let bytes = render(&document, &PdfOptions::default()).unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let page_id = pdf.get_pages()[&1];
    let page = pdf.get_dictionary(page_id).unwrap();
    let resources = resolved_dictionary(&pdf, page.get(b"Resources").unwrap());

    assert!(resources.get(b"XObject").is_err());
  }

  #[test]
  fn identity_group_remains_isolated_without_an_explicit_flatten_request() {
    let mut document = tagged_test_document();
    let text = document.pages[0].items.remove(0);
    document.pages[0]
      .items
      .push(DisplayItem::Group(common::CompositingGroup {
        mask: None,
        clip: None,
        transform: None,
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        flatten_identity: false,
        inherit_text_line_owner: true,
        items: vec![text],
      }));

    let bytes = render(&document, &PdfOptions::default()).unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let page_id = pdf.get_pages()[&1];
    let page = pdf.get_dictionary(page_id).unwrap();
    let resources = resolved_dictionary(&pdf, page.get(b"Resources").unwrap());

    assert!(resources.get(b"XObject").is_ok());
  }

  #[test]
  fn pdf_ua_validator_accepts_a_structured_text_document() {
    let document = tagged_test_document();
    let mut options = PdfOptions::default();
    options.general.pdf_ua_compliance = true;
    options.ui_language = Some("en-US".to_string());
    options.metadata.title = Some("Tagged test document".to_string());

    let bytes = render(&document, &options).unwrap();

    assert!(bytes.starts_with(b"%PDF-"));
  }

  #[test]
  fn pdf_ua_rejects_untagged_lopdf_form_widget_post_processing() {
    let mut document = tagged_test_document();
    let DisplayItem::Text(text) = &mut document.pages[0].items[0] else {
      unreachable!();
    };
    text.form_widget_id = Some(1);
    document.form_widgets.push(common::FormWidget {
      id: 1,
      kind: common::FormWidgetKind::Text,
      entries: Vec::new(),
    });
    let mut options = PdfOptions::default();
    options.general.pdf_ua_compliance = true;

    assert!(matches!(
      render(&document, &options),
      Err(crate::PdfError::Options(message)) if message.contains("tagged form API")
    ));
  }

  #[test]
  fn attachment_is_written_to_the_embedded_files_name_tree() {
    let document = tagged_test_document();
    let mut options = PdfOptions::default();
    options.attachments.push(PdfAttachment {
      path: "source.txt".to_string(),
      mime_type: "text/plain".to_string(),
      description: "Source data".to_string(),
      association: PdfAttachmentAssociation::Source,
      data: Arc::from(&b"attachment contents"[..]),
      modification_date: None,
      compress: Some(false),
    });

    let bytes = render(&document, &options).unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let catalog_id = pdf.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let catalog = pdf.get_dictionary(catalog_id).unwrap();
    let names = resolved_dictionary(&pdf, catalog.get(b"Names").unwrap());
    let embedded_files = resolved_dictionary(&pdf, names.get(b"EmbeddedFiles").unwrap());
    let entries = embedded_files.get(b"Names").unwrap().as_array().unwrap();
    assert_eq!(entries[0].as_str().unwrap(), b"source.txt");
    let file_spec = resolved_dictionary(&pdf, &entries[1]);
    assert_eq!(
      file_spec.get(b"Desc").unwrap().as_str().unwrap(),
      b"Source data"
    );
    let embedded_streams = resolved_dictionary(&pdf, file_spec.get(b"EF").unwrap());
    let stream_id = embedded_streams.get(b"F").unwrap().as_reference().unwrap();
    let stream = pdf.get_object(stream_id).unwrap().as_stream().unwrap();
    assert_eq!(stream.content, b"attachment contents");
  }

  #[test]
  fn virtual_page_numbers_and_bookmarks_reach_pdf_name_trees() {
    let mut document = tagged_test_document();
    document.pages[0].setup.page_number_start = Some(7);
    document.anchor_pages.push(common::AnchorPage {
      name: "section-one".into(),
      page_index: 0,
      section_index: 0,
      section_page_index: 0,
      physical_page_number: 1,
      virtual_page_number: 7,
    });
    let mut options = PdfOptions::default();
    options.links.export_bookmarks_to_pdf_destinations = true;

    let bytes = render(&document, &options).unwrap();
    let pdf = lopdf::Document::load_mem(&bytes).unwrap();
    let catalog_id = pdf.trailer.get(b"Root").unwrap().as_reference().unwrap();
    let catalog = pdf.get_dictionary(catalog_id).unwrap();
    let labels = resolved_dictionary(&pdf, catalog.get(b"PageLabels").unwrap());
    let label_entries = labels.get(b"Nums").unwrap().as_array().unwrap();
    assert_eq!(label_entries[0].as_i64().unwrap(), 0);
    let first_label = resolved_dictionary(&pdf, &label_entries[1]);
    assert_eq!(first_label.get(b"S").unwrap().as_name().unwrap(), b"D");
    assert_eq!(first_label.get(b"St").unwrap().as_i64().unwrap(), 7);

    let names = resolved_dictionary(&pdf, catalog.get(b"Names").unwrap());
    let destinations = resolved_dictionary(&pdf, names.get(b"Dests").unwrap());
    let destination_entries = destinations.get(b"Names").unwrap().as_array().unwrap();
    assert_eq!(destination_entries[0].as_str().unwrap(), b"section-one");
  }

  #[test]
  fn powerpoint_pdf_page_dimensions_use_the_600_dpi_print_grid() {
    assert!((pdf_page_dimension(LayoutEngineKind::Pptx, 793.75) - 793.8).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Pptx, 595.25) - 595.2).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Pptx, 446.5) - 446.52).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Pptx, 793.5) - 793.56).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Pptx, 595.5) - 595.56).abs() < 0.001);
  }

  #[test]
  fn word_pdf_page_dimensions_quantize_only_unambiguous_print_grid_positions() {
    assert!((pdf_page_dimension(LayoutEngineKind::Docx, 595.35) - 595.32).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Docx, 842.0) - 842.04).abs() < 0.001);
    assert_eq!(pdf_page_dimension(LayoutEngineKind::Docx, 612.0), 612.0);
    assert!((pdf_page_dimension(LayoutEngineKind::Docx, 287.7) - 287.7).abs() < 0.001);
    assert!((pdf_page_dimension(LayoutEngineKind::Docx, 283.5) - 283.5).abs() < 0.001);
  }

  #[test]
  fn odd_bidi_mirrored_glyph_keeps_its_authored_pdf_semantic_range() {
    let authored_text = "(";
    let rtl_style = PaintTextStyle {
      font_family: Some(Cow::Borrowed("Arial")),
      complex_font_family: Some(Cow::Borrowed("Arial")),
      font_size_pt: 11.0,
      complex_font_size_pt: Some(11.0),
      right_to_left: Some(true),
      resolved_bidi_level: Some(1),
      ..PaintTextStyle::default()
    };
    let mut text_metrics = TextMetrics::new();
    let rtl = shaped_pdf_glyphs(authored_text, &rtl_style, 0.0, &mut text_metrics)
      .expect("odd-level parenthesis must shape");
    assert_eq!(rtl.font_runs.len(), 1);
    assert_eq!(rtl.font_runs[0].glyphs.len(), 1);
    let rtl_glyph = &rtl.font_runs[0].glyphs[0];

    // UAX #9 L4 changes the visible glyph, but Krilla's semantic source range
    // remains the authored character used for ToUnicode/ActualText.
    assert_eq!(rtl_glyph.text_range, 0..authored_text.len());
    assert_eq!(&authored_text[rtl_glyph.text_range.clone()], "(");

    let ltr_style = PaintTextStyle {
      right_to_left: Some(false),
      resolved_bidi_level: Some(0),
      ..rtl_style.clone()
    };
    let visible_counterpart = shaped_pdf_glyphs(")", &ltr_style, 0.0, &mut text_metrics)
      .expect("visible mirrored counterpart must shape");
    let unmirrored_counterexample = shaped_pdf_glyphs("(", &ltr_style, 0.0, &mut text_metrics)
      .expect("even-level parenthesis must shape");
    let counterpart_run = &visible_counterpart.font_runs[0];
    let unmirrored_run = &unmirrored_counterexample.font_runs[0];
    assert_eq!(rtl.font_runs[0].font_face, counterpart_run.font_face);
    assert_eq!(rtl_glyph.glyph_id, counterpart_run.glyphs[0].glyph_id);
    assert_eq!(rtl.font_runs[0].font_face, unmirrored_run.font_face);
    assert_ne!(rtl_glyph.glyph_id, unmirrored_run.glyphs[0].glyph_id);
  }

  #[test]
  fn word_small_caps_expose_displayed_capitals_in_pdf_semantic_text() {
    assert_eq!(
      word_small_caps_semantic_text("Xxxx Xxxx", true),
      "XXXX XXXX"
    );
    assert_eq!(
      word_small_caps_semantic_text("Xxxx Xxxx", false),
      "Xxxx Xxxx"
    );
    assert_eq!(word_small_caps_semantic_text("ı", true), "ı");
  }

  #[test]
  fn wingdings_black_circle_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f06c}", Some("Wingdings")),
      "\u{26ab}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f06c}", Some("Calibri")),
      "\u{f06c}"
    );
  }

  #[test]
  fn wingdings_smiley_and_black_diamond_use_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f04a}\u{f075}", Some("Wingdings")),
      "\u{263a}\u{25c6}"
    );
  }

  #[test]
  fn wingdings_white_square_bullet_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f071}", Some("Wingdings")),
      "\u{2751}"
    );
  }

  #[test]
  fn wingdings_small_black_square_uses_powerpoint_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f06e}", Some("Wingdings")),
      "\u{25fc}"
    );
  }

  #[test]
  fn wingdings_arrow_uses_libreoffice_conversion_table_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f0d8}", Some("Wingdings")),
      "\u{27a2}"
    );
  }

  #[test]
  fn wingdings_small_square_uses_powerpoint_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f0a7}", Some("Wingdings")),
      "\u{25aa}"
    );
  }

  #[test]
  fn wingdings_black_diamond_minus_white_x_uses_powerpoint_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f076}", Some("Wingdings")),
      "\u{2756}"
    );
  }

  #[test]
  fn wingdings_right_arrow_uses_powerpoint_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f0e0}", Some("Wingdings")),
      "\u{2192}"
    );
  }

  #[test]
  fn wordprocessingml_wingdings_space_and_checkmark_use_office_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f020}\u{f0fc}", Some("Wingdings")),
      "\u{2002}✓"
    );
  }

  #[test]
  fn symbol_font_bullet_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f0b7}", Some("Symbol")),
      "\u{2022}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f0b7}", Some("SymbolMT")),
      "\u{2022}"
    );
    assert_eq!(
      symbol_font_semantic_text("\u{f0b7}", Some("Calibri")),
      "\u{f0b7}"
    );
  }

  #[test]
  fn symbol_font_minus_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f02d}", Some("Symbol")),
      "\u{2212}"
    );
  }

  #[test]
  fn symbol_font_perpendicular_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f05e}", Some("Symbol")),
      "\u{22a5}"
    );
  }

  #[test]
  fn powerpoint_transformed_gradient_uses_gdiplus_gamma_samples() {
    let black = Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    };
    let red = Color {
      r: 255,
      g: 0,
      b: 0,
      a: 255,
    };

    assert_eq!(
      gamma_correct_gradient_color(black, red, 36.0 / 255.0).r,
      105
    );
    assert_eq!(
      gamma_correct_gradient_color(black, red, 128.0 / 255.0).r,
      186
    );
    assert_eq!(gamma_correct_gradient_color(black, red, 1.0).r, 255);
  }

  #[test]
  fn synthetic_bold_uses_libreoffice_pdf_stroke_width() {
    let common_style = TextStyle {
      font_size: Pt(15.0),
      ..TextStyle::default()
    };
    let style = text_style_from_common(&common_style);

    let stroke =
      text_stroke_with_fill(&style, true, 18.0, None, None).expect("synthetic bold stroke");

    assert!((stroke.width - 0.6).abs() < f32::EPSILON);
  }

  #[test]
  fn synthetic_italic_shears_about_the_run_baseline_only_when_needed() {
    let baseline_y = 96.0;
    let transform = synthetic_italic_text_transform(true, baseline_y)
      .expect("upright fallback for italic request needs synthesis");

    assert_eq!(transform.sx(), 1.0);
    assert_eq!(transform.sy(), 1.0);
    assert_eq!(transform.ky(), 0.0);
    assert!((transform.kx() + 1.0 / 3.0).abs() < f32::EPSILON);
    assert!((transform.tx() - 32.0).abs() < f32::EPSILON);
    assert_eq!(transform.ty(), 0.0);
    assert!(synthetic_italic_text_transform(false, baseline_y).is_none());
  }

  #[test]
  fn wordart_uses_every_intermediate_warp_boundary() {
    let warp = common::TextWarp {
      source_bounds: common::Rect {
        origin: common::Point {
          x: Pt(0.0),
          y: Pt(0.0),
        },
        size: common::Size {
          width: Pt(100.0),
          height: Pt(100.0),
        },
      },
      paint_bounds: common::Rect {
        origin: common::Point {
          x: Pt(0.0),
          y: Pt(0.0),
        },
        size: common::Size {
          width: Pt(100.0),
          height: Pt(100.0),
        },
      },
      boundaries: Vec::new(),
    };
    let line = |y| vec![kurbo::Point::new(0.0, y), kurbo::Point::new(100.0, y)];
    let boundaries = vec![line(0.0), line(80.0), line(100.0)];

    let upper_half = super::text_warp_point(&warp, &boundaries, kurbo::Point::new(50.0, 25.0));
    let lower_half = super::text_warp_point(&warp, &boundaries, kurbo::Point::new(50.0, 75.0));

    assert!((upper_half.y - 40.0).abs() < f64::EPSILON);
    assert!((lower_half.y - 90.0).abs() < f64::EPSILON);
  }

  #[test]
  fn pdf_text_style_preserves_complex_script_formatting() {
    let common_style = TextStyle {
      complex_font_size: Some(Pt(18.0)),
      complex_script: Some(true),
      right_to_left: Some(true),
      complex_bold: Some(true),
      complex_italic: Some(false),
      font_charset: Some(ooxmlsdk_fonts::FontCharset::ShiftJis),
      high_ansi_font_charset: Some(ooxmlsdk_fonts::FontCharset::Ansi),
      wordprocessingml_east_asia_font_charset: Some(ooxmlsdk_fonts::FontCharset::Gb2312),
      complex_font_charset: Some(ooxmlsdk_fonts::FontCharset::Arabic),
      font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      high_ansi_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      east_asia_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      complex_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      wordprocessingml_cjk_line_metrics: true,
      ..TextStyle::default()
    };

    let style = text_style_from_common(&common_style);

    assert_eq!(style.complex_font_size_pt, Some(18.0));
    assert_eq!(style.complex_script, Some(true));
    assert_eq!(style.right_to_left, Some(true));
    assert_eq!(style.complex_bold, Some(true));
    assert_eq!(style.complex_italic, Some(false));
    assert_eq!(style.font_charset(), common_style.font_charset);
    assert_eq!(
      style.high_ansi_font_charset(),
      common_style.high_ansi_font_charset
    );
    assert_eq!(
      style.wordprocessingml_east_asia_font_charset(),
      common_style.wordprocessingml_east_asia_font_charset
    );
    assert_eq!(
      style.complex_font_charset(),
      common_style.complex_font_charset
    );
    assert_eq!(style.font_pitch(), common_style.font_pitch);
    assert_eq!(
      style.high_ansi_font_pitch(),
      common_style.high_ansi_font_pitch
    );
    assert_eq!(
      style.east_asia_font_pitch(),
      common_style.east_asia_font_pitch
    );
    assert_eq!(style.complex_font_pitch(), common_style.complex_font_pitch);
    assert!(style.wordprocessingml_cjk_line_metrics());
  }
}
