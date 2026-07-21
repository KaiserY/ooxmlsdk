use std::borrow::Cow;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use krilla::Document;
use krilla::action::{Action, LinkAction};
use krilla::annotation::{Annotation, LinkAnnotation, Target};
use krilla::color::rgb;
use krilla::destination::{Destination, NamedDestination, XyzDestination};
use krilla::embed::{AssociationKind, EmbeddedFile, MimeType};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::Image;
use krilla::metadata::{DateTime, Metadata};
use krilla::num::NormalizedF32;
use krilla::outline::{Outline, OutlineNode};
use krilla::page::{NumberingStyle, PageLabel, PageSettings};
use krilla::paint::{Fill, FillRule, LineJoin, LinearGradient, SpreadMethod, Stop, Stroke};
use krilla::surface::Surface;
use krilla::tagging::{
  Artifact, ArtifactType, BBox, ContentTag, Identifier, Node, SpanTag, TableHeaderScope, Tag,
  TagGroup, TagTree,
};
use krilla::text::{Glyph, GlyphId};
use krilla_svg::{SurfaceExt, SvgSettings};
use rustc_hash::FxHashMap as HashMap;
use smallvec::SmallVec;

use super::fonts::FontSet;
use super::form_widgets::{collect_form_widget_annotations, inject_form_widget_annotations};
use super::image::decode_image;
use super::settings::serialize_settings;
use crate::error::{PdfError, Result};
use crate::options::{PdfAttachmentAssociation, PdfDateTime, PdfOptions};
use crate::{
  PdfConversionDiagnostics, PdfConversionOutput, PdfFontAudit, PdfFontAuditIssue,
  PdfFontAuditIssueKind, PdfFontAuditOutput, PdfFontFaceDiagnostics, PdfGlyphBoundsDiagnostics,
  PdfGlyphDiagnostics, PdfGlyphRunDiagnostics, PdfPageDiagnostics, PdfTextPortionDiagnostics,
  PdfTextPortionKind, PdfTextRunDiagnostics,
};
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceData, FontStyleRef};
use ooxmlsdk_layout::text_metrics::TextMetrics;

const INTERNAL_LINK_DESTINATION_SHIFT_PT: f32 = 10.0;
const LO_ARIAL_BOLD_11PT_VERTICAL_SCALE: f32 = 1.07;

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
  let paint = PaintDocument::from_layout(document, &mut text_metrics);
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
  let internal_links = InternalLinkTargets::from_paint(&paint);
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
  let tagging_enabled = options.general.tagged_pdf || options.general.pdf_ua_compliance;
  let mut tag_tree = TagTree::new().with_lang(options.ui_language.clone());

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
  Text(TextItem<'doc>),
  Image(ImageItem<'doc>),
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
  text: Cow<'doc, str>,
  style: TextStyle<'doc>,
  rotation_center_pt: Option<(f32, f32)>,
  hyperlink_url: Option<Cow<'doc, str>>,
  dynamic_field: Option<common::DynamicField<'doc>>,
  form_widget_id: Option<u32>,
  paragraph_bidi: bool,
  preserve_text_portion: bool,
  decoration_span_start_x_pt: Option<f32>,
  pdf_text_segmentation: common::PdfTextSegmentation,
  source_path: Option<&'doc [usize]>,
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
  alt_text: Option<Cow<'doc, str>>,
  hyperlink_url: Option<Cow<'doc, str>>,
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
struct RectItem {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  fill_color: Option<RgbColor>,
  fill_opacity: f32,
  stroke: Option<BorderStyle>,
  stroke_opacity: f32,
}

#[derive(Clone, Copy, Debug)]
struct LineItem {
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  width_pt: f32,
  color: RgbColor,
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
  stroke: Option<BorderStyle>,
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

#[derive(Clone, Debug, PartialEq)]
struct TextStyle<'doc> {
  font_family: Option<Cow<'doc, str>>,
  east_asia_font_family: Option<Cow<'doc, str>>,
  complex_font_family: Option<Cow<'doc, str>>,
  symbol_font_family: Option<Cow<'doc, str>>,
  font_size_pt: f32,
  complex_font_size_pt: Option<f32>,
  character_spacing_pt: f32,
  baseline_shift_pt: f32,
  use_windows_font_metrics: bool,
  bold: bool,
  italic: bool,
  underline: bool,
  strikethrough: bool,
  uppercase: bool,
  small_caps: bool,
  hidden: bool,
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

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing_pt
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift_pt
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

#[derive(Clone, Copy, Debug)]
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
        let painted = !matches!(portion.kind, PaintTextPortionKind::Tab)
          && text_has_visible_glyph_paint(&text.item.style);
        if painted {
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
          if painted && portion.text_range.start < portion.text_range.end {
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
          if painted {
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
            let font = &audit.fonts[font_index];
            if font.parse_error.is_none() && glyph.glyph_id.to_u32() >= u32::from(font.glyph_count)
            {
              let mut issue = location();
              issue.kind = PdfFontAuditIssueKind::GlyphIdOutOfRange;
              issue.detail = format!(
                "font_index={font_index}, glyph_id={}, glyph_count={}",
                glyph.glyph_id.to_u32(),
                font.glyph_count
              );
              push_font_audit_issue(&mut audit, issue);
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
  let checksum_adjustment = ttf_parser::RawFace::parse(data, face_data.index)
    .ok()
    .and_then(|raw| raw.table(ttf_parser::Tag::from_bytes(b"head")))
    .and_then(|head| head.get(8..12))
    .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
  let face = match ttf_parser::Face::parse(data, face_data.index) {
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
  let units_per_em = face.units_per_em();
  let em_divisor = f32::from(units_per_em);
  let mut family_names = Vec::new();
  let mut postscript_name = None;
  let mut style_name = None;
  for name in face.names() {
    let Some(value) = name.to_string() else {
      continue;
    };
    match name.name_id {
      ttf_parser::name_id::FAMILY | ttf_parser::name_id::TYPOGRAPHIC_FAMILY
        if !family_names.contains(&value) =>
      {
        family_names.push(value);
      }
      ttf_parser::name_id::POST_SCRIPT_NAME => postscript_name = Some(value),
      ttf_parser::name_id::SUBFAMILY => style_name = Some(value),
      _ => {}
    }
  }
  let bounds = face.global_bounding_box();
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
    glyph_count: face.number_of_glyphs(),
    ascender_em: f32::from(face.ascender()) / em_divisor,
    descender_em: f32::from(face.descender()) / em_divisor,
    cap_height_em: face
      .capital_height()
      .map(|value| f32::from(value) / em_divisor),
    global_bounds_em: PdfGlyphBoundsDiagnostics {
      x_min_em: f32::from(bounds.x_min) / em_divisor,
      y_min_em: f32::from(bounds.y_min) / em_divisor,
      x_max_em: f32::from(bounds.x_max) / em_divisor,
      y_max_em: f32::from(bounds.y_max) / em_divisor,
    },
    monospaced: face.is_monospaced(),
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
  fn from_paint(paint: &PaintDocument<'_>) -> Self {
    let mut positions = HashMap::default();
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
  } else if let Some(note_kind) = kind.strip_suffix("-backlink") {
    (note_kind, "-reference")
  } else {
    return None;
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
  ) -> Self {
    let pages = document
      .pages
      .iter()
      .enumerate()
      .map(|(page_index, page)| {
        let layout_items = coalesced_writer_text_items(
          page.items.iter().filter_map(page_item_from_common),
          text_metrics,
        );
        let line_owners = paint_line_owners(document, page_index, layout_items.len());
        let decoration_metadata = decoration_render_metadata(&layout_items);
        let items = layout_items
          .into_iter()
          .enumerate()
          .map(|(item_index, item)| match item {
            PageItem::Text(mut text) => {
              let owner = line_owners.get(item_index).copied().flatten();
              let metadata = decoration_metadata[item_index];
              if metadata.suppress {
                text.style.underline = false;
                text.style.strikethrough = false;
              }
              text.decoration_span_start_x_pt = metadata.span_start_x_pt;
              PaintItem::Text(Box::new(PaintText::from_layout_text(
                text,
                owner,
                page.setup.size.width.0,
                text_metrics,
              )))
            }
            PageItem::Image(image) => PaintItem::Image(image),
            PageItem::LinkArea(link_area) => PaintItem::LinkArea(link_area),
            PageItem::Rect(rect) => PaintItem::Rect(rect),
            PageItem::Line(line) => PaintItem::Line(line),
            PageItem::Polyline(polyline) => PaintItem::Polyline(polyline),
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
  if engine_kind == common::LayoutEngineKind::Pptx {
    // PowerPoint's fixed-format writer quantizes presentation MediaBox
    // dimensions to its 600 dpi print-device grid. Keep the OOXML/layout
    // coordinate space exact and apply this only at PDF page creation.
    const PRINT_DPI: f32 = 600.0;
    (dimension_pt * PRINT_DPI / 72.0).round_ties_even() * 72.0 / PRINT_DPI
  } else {
    dimension_pt
  }
}

fn page_item_from_common<'doc>(item: &'doc common::DisplayItem<'static>) -> Option<PageItem<'doc>> {
  match item {
    common::DisplayItem::Text(text) => Some(PageItem::Text(text_item_from_common(text))),
    common::DisplayItem::Image(image) => Some(PageItem::Image(image_item_from_common(image))),
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

fn text_item_from_common<'doc>(text: &'doc common::TextRun<'static>) -> TextItem<'doc> {
  TextItem {
    x_pt: text.origin.x.0,
    y_pt: text.origin.y.0,
    line_height_pt: text.line_height.0,
    text: Cow::Borrowed(text.text.as_ref()),
    style: text_style_from_common(&text.style),
    rotation_center_pt: text.rotation_center.map(|point| (point.x.0, point.y.0)),
    hyperlink_url: text
      .hyperlink_url
      .as_ref()
      .map(|url| Cow::Borrowed(url.as_ref())),
    dynamic_field: text.dynamic_field.as_ref().map(dynamic_field_borrowed),
    form_widget_id: text.form_widget_id,
    paragraph_bidi: text.paragraph_bidi,
    preserve_text_portion: text.preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: text.pdf_text_segmentation,
    source_path: text.source.as_ref().map(|source| source.path.as_slice()),
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
    alt_text: image
      .alt_text
      .as_ref()
      .map(|text| Cow::Borrowed(text.as_ref())),
    hyperlink_url: image
      .hyperlink_url
      .as_ref()
      .map(|url| Cow::Borrowed(url.as_ref())),
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
    stroke: path.stroke.as_ref().map(stroke_from_common),
  }
}

fn rect_item_from_common(rect: &common::RectItem<'static>) -> RectItem {
  let (fill_color, fill_opacity) = solid_color(&rect.fill)
    .map(|color| (Some(rgb(color)), opacity(color)))
    .unwrap_or((None, 1.0));
  RectItem {
    x_pt: rect.bounds.origin.x.0,
    y_pt: rect.bounds.origin.y.0,
    width_pt: rect.bounds.size.width.0,
    height_pt: rect.bounds.size.height.0,
    fill_color,
    fill_opacity,
    stroke: rect.stroke.as_ref().map(stroke_from_common),
    stroke_opacity: rect
      .stroke
      .as_ref()
      .map_or(1.0, |stroke| opacity(stroke.color)),
  }
}

fn line_item_from_common(line: &common::LineItem<'static>) -> LineItem {
  LineItem {
    x1_pt: line.start.x.0,
    y1_pt: line.start.y.0,
    x2_pt: line.end.x.0,
    y2_pt: line.end.y.0,
    width_pt: line.stroke.width.0,
    color: rgb(line.stroke.color),
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
    font_size_pt: style.font_size.0,
    complex_font_size_pt: style.complex_font_size.map(|size| size.0),
    character_spacing_pt: style.character_spacing.0,
    baseline_shift_pt: style.baseline_shift.0,
    use_windows_font_metrics: style.use_windows_font_metrics,
    bold: style.bold,
    italic: style.italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
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
    common::DynamicField::Page => common::DynamicField::Page,
    common::DynamicField::NumPages => common::DynamicField::NumPages,
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

fn solid_color(fill: &common::Fill<'static>) -> Option<common::Color> {
  match fill {
    common::Fill::Solid(color) => Some(*color),
    common::Fill::None
    | common::Fill::Theme(_)
    | common::Fill::Gradient(_)
    | common::Fill::Image { .. }
    | common::Fill::Pattern { .. } => None,
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
  items: impl IntoIterator<Item = PageItem<'doc>>,
  text_metrics: &mut TextMetrics,
) -> Vec<PageItem<'doc>> {
  let items = items.into_iter();
  let mut output: Vec<PageItem<'doc>> = Vec::with_capacity(items.size_hint().0);
  for item in items {
    match item {
      PageItem::Text(text) => {
        if let Some(PageItem::Text(previous)) = output.last_mut()
          && writer_text_items_coalesce(previous, &text, text_metrics)
        {
          previous.text.to_mut().push_str(&text.text);
          previous.line_height_pt = previous.line_height_pt.max(text.line_height_pt);
          continue;
        }
        output.push(PageItem::Text(text));
      }
      PageItem::Image(image) => output.push(PageItem::Image(image)),
      PageItem::LinkArea(link_area) => output.push(PageItem::LinkArea(link_area)),
      PageItem::Rect(rect) => output.push(PageItem::Rect(rect)),
      PageItem::Line(line) => output.push(PageItem::Line(line)),
      PageItem::Polyline(polyline) => output.push(PageItem::Polyline(polyline)),
    }
  }
  output
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
    || current.rotation_center_pt != next.rotation_center_pt
    || current.source_path != next.source_path
    || current.decoration_span_start_x_pt != next.decoration_span_start_x_pt
    || (current.y_pt - next.y_pt).abs() >= 0.01
    || (current.line_height_pt - next.line_height_pt).abs() >= 0.01
  {
    return false;
  }
  let current_right = current.x_pt + text_metrics.measure_text(&current.text, &current.style);
  (current_right - next.x_pt).abs() < 0.25
}

impl<'doc> PaintText<'doc> {
  fn from_layout_text(
    text: TextItem<'doc>,
    owner: Option<PaintLineOwner>,
    page_width_pt: f32,
    text_metrics: &mut TextMetrics,
  ) -> Self {
    let text_ref = &text;
    let glyphs = shaped_pdf_glyphs(&text_ref.text, &text_ref.style, text_metrics);
    let width_pt = glyphs
      .as_ref()
      .map(|run| run.width_pt)
      .unwrap_or_else(|| text_metrics.measure_text(&text_ref.text, &text_ref.style));
    let baseline_y = match owner.map(|owner| owner.frame_kind) {
      Some(FollowFrameKind::Table) => text_ref.y_pt - text_ref.style.baseline_shift_pt,
      Some(FollowFrameKind::Paragraph | FollowFrameKind::Notes) | None => {
        let offset = if text_ref.style.use_windows_font_metrics {
          text_metrics
            .baseline_offset_in_line_with_windows_metrics(&text_ref.style, text_ref.line_height_pt)
        } else {
          text_metrics.baseline_offset_in_line(&text_ref.style, text_ref.line_height_pt)
        };
        text_ref.y_pt + offset
      }
    };
    let vertical_metrics = text_metrics.vertical_metrics(&text_ref.style);
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
        clip: owner.map(|owner| owner.clip),
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
  let ranges = text_portion_ranges(text);
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

fn text_portion_ranges(text: &TextItem<'_>) -> PaintTextPortionRanges {
  if text.text.is_empty() {
    return PaintTextPortionRanges::new();
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
    && text.pdf_text_segmentation != common::PdfTextSegmentation::Portion
    && !split_decorated_portions
  {
    return edge_whitespace_text_portion_ranges(text);
  }
  let split_portions =
    text.pdf_text_segmentation == common::PdfTextSegmentation::Portion || split_decorated_portions;
  if !split_portions && text.hyperlink_url.is_some() && !text.text.contains('\t') {
    let mut ranges = PaintTextPortionRanges::new();
    ranges.push((PaintTextPortionKind::Link, 0..text.text.len()));
    return ranges;
  }

  let mut ranges = PaintTextPortionRanges::new();
  let mut start = 0usize;
  for (index, ch) in text.text.char_indices() {
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
  clip: PaintClipRect,
}

fn paint_line_owners(
  document: &common::LayoutDocument<'static>,
  page_index: usize,
  item_count: usize,
) -> Vec<Option<PaintLineOwner>> {
  let mut owners = vec![None; item_count];
  for (frame_index, frame) in document
    .frames
    .iter()
    .enumerate()
    .filter(|(_, frame)| frame.page_index == page_index)
  {
    for (line_index, line) in frame.lines.iter().enumerate() {
      let start = line.item_range.start.min(item_count);
      let end = line.item_range.end.min(item_count);
      for owner in owners.iter_mut().take(end).skip(start) {
        if owner.is_none() {
          *owner = Some(PaintLineOwner {
            frame_index,
            line_index,
            frame_kind: frame_kind_name_from_common(&frame.kind),
            clip: PaintClipRect {
              x_pt: line.bounds.origin.x.0,
              y_pt: line.bounds.origin.y.0,
              width_pt: line.bounds.size.width.0,
              height_pt: line.bounds.size.height.0,
            },
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
    PaintItem::Image(_) | PaintItem::Rect(_) | PaintItem::Line(_) | PaintItem::Polyline(_) => Some(
      ContentTag::Artifact(Artifact::with_kind(ArtifactType::Layout)),
    ),
    PaintItem::Text(_) | PaintItem::LinkArea(_) => None,
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
      PaintItem::LinkArea(_) if !annotations.is_empty() => {
        blocks.push(PageTagBlock::Node(
          TagGroup::with_children(Tag::Link, annotations.into_iter().map(Node::Leaf).collect())
            .into(),
        ));
      }
      PaintItem::Text(_)
      | PaintItem::Image(_)
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
      if is_svg_image(image) {
        if draw_svg_item(surface, image).is_err() {
          draw_missing_image(surface, image);
        }
      } else {
        match decode_image(
          &image.data,
          image.content_type.as_deref(),
          options,
          Some(metafile_render_options_for_image(image, options)),
        ) {
          Ok(pdf_image) => draw_image_item(surface, image, pdf_image),
          Err(_) => draw_missing_image(surface, image),
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
    PaintItem::Line(line) => draw_line_item(surface, line),
    PaintItem::Polyline(polyline) => draw_polyline_item(surface, polyline),
  }
  Ok(())
}

fn metafile_render_options_for_image(
  _image: &ImageItem<'_>,
  options: &PdfOptions,
) -> ooxmlsdk_layout::render::emf_wmf::RenderOptions {
  let dpi = options
    .images
    .max_resolution_dpi
    .unwrap_or(300)
    .clamp(72, 600);
  ooxmlsdk_layout::render::emf_wmf::RenderOptions {
    target_width_px: None,
    target_height_px: None,
    max_pixels: Some(dpi.saturating_mul(dpi).saturating_mul(64)),
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
  let glyph_semantic_text =
    symbol_font_semantic_text(&item.text, item.style.font_family.as_deref());
  for portion in &text.portions {
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
    surface.set_fill(Some(fill(&item.style)));
    let vertical_scale = if item.text.contains('\t') {
      1.0
    } else {
      text_vertical_scale(&item.style)
    };
    if (vertical_scale - 1.0).abs() > f32::EPSILON {
      surface.push_transform(&Transform::from_row(
        1.0,
        0.0,
        0.0,
        vertical_scale,
        0.0,
        portion.baseline_y * (1.0 - vertical_scale),
      ));
    }
    // Tabs are layout controls: their measured advance positions subsequent
    // portions, but emitting the font's glyph 0 leaks a NUL into PDF text.
    if !matches!(portion.kind, PaintTextPortionKind::Tab)
      && text_has_visible_glyph_paint(&item.style)
      && let Some(glyphs) = &portion.glyphs
    {
      for run in glyphs {
        let selected = fonts.select_face(&run.font_face)?;
        surface.set_stroke(text_stroke(&item.style, selected.synthetic_bold));
        surface.draw_glyphs(
          Point::from_xy(portion.x_pt + run.x_offset_pt, portion.baseline_y),
          &run.glyphs,
          selected.font,
          &glyph_semantic_text,
          run.font_size_pt,
          false,
        );
      }
    }
    if (vertical_scale - 1.0).abs() > f32::EPSILON {
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
  }
  Ok(())
}

fn text_has_visible_glyph_paint(style: &TextStyle<'_>) -> bool {
  style.opacity > f32::EPSILON
    || (style.outline_color.is_some()
      && style.outline_width_pt > f32::EPSILON
      && style.outline_opacity > f32::EPSILON)
}

fn symbol_font_semantic_text<'a>(text: &'a str, font_family: Option<&str>) -> Cow<'a, str> {
  let symbol = font_family.is_some_and(|family| {
    family.eq_ignore_ascii_case("Symbol") || family.eq_ignore_ascii_case("SymbolMT")
  });
  let wingdings = font_family.is_some_and(|family| family.eq_ignore_ascii_case("Wingdings"));
  if !(symbol && (text.contains('\u{f02d}') || text.contains('\u{f0b7}'))
    || wingdings && (text.contains('\u{f06c}') || text.contains('\u{f071}')))
  {
    return Cow::Borrowed(text);
  }

  // Keep the legacy symbol-font glyph selected by the shaped run, but expose
  // its standardized character through the PDF ToUnicode map. Unicode WG2
  // N4363 maps Wingdings character 108 to U+26AB; LibreOffice's Microsoft
  // Symbol conversion table maps character 0xB7 to U+2022.
  Cow::Owned(
    text
      .chars()
      .map(|character| match character {
        '\u{f02d}' if symbol => '\u{2212}',
        '\u{f0b7}' if symbol => '\u{2022}',
        '\u{f06c}' if wingdings => '\u{26ab}',
        '\u{f071}' if wingdings => '\u{2751}',
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

fn draw_line_item(surface: &mut Surface<'_>, line: &LineItem) {
  let mut path = PathBuilder::new();
  match line.kind {
    LineItemKind::Stroke => {
      surface.set_fill(None);
      surface.set_stroke(Some(Stroke {
        width: line.width_pt,
        paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
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

  surface.set_fill(path_fill_from_common(polyline.fill, polyline));
  if let Some(stroke) = polyline.stroke {
    surface.set_stroke(Some(Stroke {
      width: stroke.width_pt,
      paint: rgb::Color::new(stroke.color.r, stroke.color.g, stroke.color.b).into(),
      line_join: LineJoin::Bevel,
      ..Default::default()
    }));
  } else {
    surface.set_stroke(None);
  }

  let mut path = PathBuilder::new();
  if polyline.commands.is_empty() {
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
    surface.draw_path(&path);
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

fn path_fill_from_common(fill: &common::Fill<'static>, path: &PolylineItem<'_>) -> Option<Fill> {
  let paint = match fill {
    common::Fill::Solid(color) => rgb::Color::new(color.r, color.g, color.b).into(),
    common::Fill::Gradient(gradient) => {
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
        stops: stops
          .iter()
          .filter_map(|stop| {
            Some(Stop {
              offset: NormalizedF32::new(stop.position.clamp(0.0, 1.0))?,
              color: rgb::Color::new(stop.color.r, stop.color.g, stop.color.b).into(),
              opacity: NormalizedF32::new(f32::from(stop.color.a) / 255.0)?,
            })
          })
          .collect(),
        anti_alias: true,
      }
      .into()
    }
    common::Fill::None
    | common::Fill::Theme(_)
    | common::Fill::Image { .. }
    | common::Fill::Pattern { .. } => return None,
  };
  Some(Fill {
    paint,
    opacity: NormalizedF32::ONE,
    rule: FillRule::EvenOdd,
  })
}

fn draw_rect_item(surface: &mut Surface<'_>, rect: &RectItem) {
  if let Some(fill_color) = rect.fill_color {
    surface.set_fill(Some(Fill {
      paint: rgb::Color::new(fill_color.r, fill_color.g, fill_color.b).into(),
      opacity: NormalizedF32::new(rect.fill_opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
      rule: FillRule::EvenOdd,
    }));
    surface.set_stroke(None);
    draw_rect_path(surface, rect);
  }

  if let Some(stroke) = rect.stroke
    && (rect.stroke_opacity > f32::EPSILON || (rect.fill_color.is_none() && rect.height_pt < 50.0))
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
  draw_transformed_image_content(surface, image, |surface, size| {
    surface.draw_image(pdf_image, size);
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

fn draw_svg_item(surface: &mut Surface<'_>, image: &ImageItem<'_>) -> Result<()> {
  static FONT_DATABASE: OnceLock<Arc<fontdb::Database>> = OnceLock::new();
  let fontdb = FONT_DATABASE
    .get_or_init(|| {
      let mut database = fontdb::Database::new();
      database.load_system_fonts();
      Arc::new(database)
    })
    .clone();
  let tree = usvg::Tree::from_data(
    &image.data,
    &usvg::Options {
      fontdb,
      ..usvg::Options::default()
    },
  )
  .map_err(|err| PdfError::Krilla(format!("failed to decode SVG image: {err}")))?;
  draw_transformed_image_content(surface, image, |surface, size| {
    surface.draw_svg(
      &tree,
      size,
      SvgSettings {
        // An OOXML picture is one semantic image. Keeping SVG text as paths
        // avoids leaking decorative image text into PDF text extraction.
        embed_text: false,
        ..SvgSettings::default()
      },
    );
  });
  Ok(())
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

  if let Some(clip) = rect_path(0.0, 0.0, width, height) {
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
  if let Some(language) = &options.ui_language {
    metadata = metadata.language(language.clone());
  }
  metadata
}

fn shaped_pdf_glyphs(
  text: &str,
  style: &TextStyle<'_>,
  text_metrics: &mut TextMetrics,
) -> Option<PaintGlyphRun> {
  let shaped = text_metrics.shape_text(text, style)?;
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
        x_offset_pt,
        glyphs: Vec::new(),
      });
      last_run = Some(run_key);
    }
    font_runs
      .last_mut()
      .expect("font run was just pushed")
      .glyphs
      .push(PaintGlyph {
        glyph_id: GlyphId::new(glyph.glyph_id),
        text_range: glyph.text_range,
        x_advance: glyph.x_advance_em,
        x_offset: glyph.x_offset_em,
        y_offset: glyph.y_offset_em,
        y_advance: glyph.y_advance_em,
        bounds_em: glyph.bounds_em.map(|bounds| PdfGlyphBoundsDiagnostics {
          x_min_em: bounds.x_min_em,
          y_min_em: bounds.y_min_em,
          x_max_em: bounds.x_max_em,
          y_max_em: bounds.y_max_em,
        }),
      });
    x_offset_pt += glyph.x_advance_em * glyph.font_size_pt;
  }
  Some(PaintGlyphRun {
    width_pt: shaped.width_pt,
    font_runs,
  })
}

fn text_vertical_scale(style: &TextStyle<'_>) -> f32 {
  if style.bold
    && (style.font_size_pt - 11.0).abs() < 0.01
    && style
      .font_family
      .as_deref()
      .is_some_and(|family| family.eq_ignore_ascii_case("Arial"))
  {
    LO_ARIAL_BOLD_11PT_VERTICAL_SCALE
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

fn text_stroke(style: &TextStyle<'_>, synthetic_bold: bool) -> Option<Stroke> {
  // LibreOffice's PDF writer uses fill-then-stroke for an artificial bold
  // face, with a stroke width of one thirtieth of the font height. An
  // explicit text outline wins over artificial bold there as well.
  stroke(style).or_else(|| {
    synthetic_bold.then(|| Stroke {
      width: style.font_size_pt / 30.0,
      paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
      opacity: NormalizedF32::new(style.opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
      ..Default::default()
    })
  })
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use super::{
    gamma_correct_gradient_color, pdf_metadata, pdf_page_dimension, render,
    symbol_font_semantic_text, text_stroke, text_style_from_common,
  };
  use crate::options::{PdfAttachment, PdfAttachmentAssociation, PdfOptions};
  use krilla::Document;
  use krilla::geom::Size;
  use krilla::page::PageSettings;
  use ooxmlsdk_layout::common::{
    self, Color, DisplayItem, DisplayPage, LayoutDocument, LayoutEngineKind, Pt, TextRun, TextStyle,
  };

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
    assert_eq!(pdf_page_dimension(LayoutEngineKind::Docx, 793.75), 793.75);
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
  fn wingdings_white_square_bullet_uses_standardized_pdf_unicode() {
    assert_eq!(
      symbol_font_semantic_text("\u{f071}", Some("Wingdings")),
      "\u{2751}"
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

    let stroke = text_stroke(&style, true).expect("synthetic bold stroke");

    assert!((stroke.width - 0.5).abs() < f32::EPSILON);
  }
}
