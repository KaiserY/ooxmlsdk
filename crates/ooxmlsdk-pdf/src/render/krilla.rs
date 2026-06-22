use std::borrow::Cow;
use std::ops::Range;

use krilla::Document;
use krilla::action::{Action, LinkAction};
use krilla::annotation::{Annotation, LinkAnnotation, Target};
use krilla::color::rgb;
use krilla::destination::{Destination, XyzDestination};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::Image;
use krilla::num::NormalizedF32;
use krilla::outline::{Outline, OutlineNode};
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, LineJoin, Stroke};
use krilla::surface::Surface;
use krilla::text::{GlyphId, KrillaGlyph, TextDirection};
use rustc_hash::FxHashMap as HashMap;
use smallvec::SmallVec;

use super::fonts::FontSet;
use super::form_widgets::inject_form_widget_annotations;
use super::image::decode_image;
use super::settings::serialize_settings;
use crate::error::{PdfError, Result};
use crate::options::PdfOptions;
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceData, FontStyleRef};
use ooxmlsdk_layout::text_metrics::TextMetrics;

const INTERNAL_LINK_DESTINATION_SHIFT_PT: f32 = 10.0;
const LO_ARIAL_BOLD_11PT_VERTICAL_SCALE: f32 = 1.07;

type PaintTextPortionRanges<'doc> = SmallVec<[(PaintTextPortionKind<'doc>, Range<usize>); 2]>;
type PaintGlyphFontRuns = SmallVec<[PaintGlyphFontRun; 2]>;

pub(crate) fn render(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<Vec<u8>> {
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
            && text.width_pt >= 0.0
            && !text.portions.is_empty()
            && text.portions.iter().all(|portion| {
              match &portion.kind {
                PaintTextPortionKind::Field(kind) => {
                  let _field_kind = kind;
                }
                PaintTextPortionKind::Text
                | PaintTextPortionKind::Tab
                | PaintTextPortionKind::Link => {}
              }
              portion.baseline_y.is_finite()
                && portion.width_pt >= 0.0
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
                    .all(|glyph| glyph.x_advance >= 0.0)
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
  let mut fonts = FontSet::load(text_metrics.into_font_resolver())?;

  for page in &paint.pages {
    let settings = PageSettings::from_wh(page.width_pt, page.height_pt)
      .ok_or_else(|| PdfError::Krilla("invalid page size".to_string()))?;

    let mut pdf_page = pdf.start_page_with(settings);
    let mut surface = pdf_page.surface();
    let mut link_annotations = Vec::new();
    for item in page
      .items
      .iter()
      .filter(|item| paint_item_intersects_page(item, page.width_pt, page.height_pt))
    {
      draw_paint_item(
        &mut surface,
        item,
        &mut fonts,
        &internal_links,
        &mut link_annotations,
        options,
      );
    }
    surface.finish();
    for annotation in link_annotations {
      pdf_page.add_annotation(annotation);
    }
  }

  if let Some(outline) = pdf_outline_for_entries(&document.outline_entries) {
    pdf.set_outline(outline);
  }

  let pdf = pdf
    .finish()
    .map_err(|err| PdfError::Krilla(format!("{err:?}")))?;
  let mut annotation_text_metrics = TextMetrics::new();
  inject_form_widget_annotations(document, pdf, &mut annotation_text_metrics)
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
  Polyline(PolylineItem),
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
}

#[derive(Clone, Debug)]
struct ImageItem<'doc> {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  crop: ImageCrop,
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
struct PolylineItem {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  points: Vec<(f32, f32)>,
  closed: bool,
  fill_color: Option<RgbColor>,
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
  symbol_font_family: Option<Cow<'doc, str>>,
  font_size_pt: f32,
  complex_font_size_pt: Option<f32>,
  character_spacing_pt: f32,
  baseline_shift_pt: f32,
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
  Text(PaintText<'doc>),
  Image(ImageItem<'doc>),
  LinkArea(LinkAreaItem<'doc>),
  Rect(RectItem),
  Line(LineItem),
  Polyline(PolylineItem),
}

#[derive(Clone, Debug)]
struct PaintText<'doc> {
  item: TextItem<'doc>,
  source_frame_index: Option<usize>,
  source_line_index: Option<usize>,
  baseline_y: f32,
  width_pt: f32,
  portions: Vec<PaintTextPortion<'doc>>,
}

#[derive(Clone, Debug)]
struct PaintTextPortion<'doc> {
  kind: PaintTextPortionKind<'doc>,
  text_range: std::ops::Range<usize>,
  x_pt: f32,
  baseline_y: f32,
  width_pt: f32,
  clip: Option<PaintClipRect>,
  glyphs: Option<PaintGlyphFontRuns>,
  highlight: Option<PaintRect>,
  underline: Option<PaintStrokeLine>,
  strikethrough: Option<PaintStrokeLine>,
  link: Option<PaintLink<'doc>>,
}

#[derive(Clone, Debug)]
enum PaintTextPortionKind<'doc> {
  Text,
  Tab,
  Field(common::DynamicField<'doc>),
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
  x_offset_pt: f32,
  glyphs: Vec<KrillaGlyph>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct PaintStrokeLine {
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  width_pt: f32,
  color: RgbColor,
}

#[derive(Clone, Debug)]
struct PaintLink<'doc> {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  url: Cow<'doc, str>,
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
          page
            .items
            .iter()
            .filter_map(page_item_from_common)
            .collect::<Vec<_>>(),
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
              PaintItem::Text(PaintText::from_layout_text(
                text,
                owner,
                page.setup.size.width.0,
                text_metrics,
              ))
            }
            PageItem::Image(image) => PaintItem::Image(image),
            PageItem::LinkArea(link_area) => PaintItem::LinkArea(link_area),
            PageItem::Rect(rect) => PaintItem::Rect(rect),
            PageItem::Line(line) => PaintItem::Line(line),
            PageItem::Polyline(polyline) => PaintItem::Polyline(polyline),
          })
          .collect();
        PaintPage {
          width_pt: page.setup.size.width.0,
          height_pt: page.setup.size.height.0,
          items,
        }
      })
      .collect();
    Self { pages }
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
  }
}

fn image_item_from_common<'doc>(image: &'doc common::ImageItem<'static>) -> ImageItem<'doc> {
  ImageItem {
    x_pt: image.bounds.origin.x.0,
    y_pt: image.bounds.origin.y.0,
    width_pt: image.bounds.size.width.0,
    height_pt: image.bounds.size.height.0,
    crop: image.crop.unwrap_or_default().into(),
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

fn polyline_from_common(path: &common::PathItem<'static>) -> PolylineItem {
  let x_pt = path.bounds.origin.x.0;
  let y_pt = path.bounds.origin.y.0;
  PolylineItem {
    x_pt,
    y_pt,
    width_pt: path.bounds.size.width.0,
    height_pt: path.bounds.size.height.0,
    points: path
      .points
      .iter()
      .map(|point| (point.x.0 - x_pt, point.y.0 - y_pt))
      .collect(),
    closed: path.closed,
    fill_color: solid_rgb(&path.fill),
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
    symbol_font_family: style
      .symbol_font_family
      .as_ref()
      .map(|value| Cow::Borrowed(value.as_ref())),
    font_size_pt: style.font_size.0,
    complex_font_size_pt: style.complex_font_size.map(|size| size.0),
    character_spacing_pt: style.character_spacing.0,
    baseline_shift_pt: style.baseline_shift.0,
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

fn solid_rgb(fill: &common::Fill<'static>) -> Option<RgbColor> {
  solid_color(fill).map(rgb)
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
  items: Vec<PageItem<'doc>>,
  text_metrics: &mut TextMetrics,
) -> Vec<PageItem<'doc>> {
  let mut output: Vec<PageItem<'doc>> = Vec::with_capacity(items.len());
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
    let glyphs = if should_shape_pdf_glyphs(text_ref) {
      shaped_pdf_glyphs(&text_ref.text, &text_ref.style, text_metrics)
    } else {
      None
    };
    let width_pt = glyphs
      .as_ref()
      .map(|run| run.width_pt)
      .unwrap_or_else(|| text_metrics.measure_text(&text_ref.text, &text_ref.style));
    let baseline_y = match owner.map(|owner| owner.frame_kind) {
      Some(FollowFrameKind::Table) => text_ref.y_pt - text_ref.style.baseline_shift_pt,
      Some(FollowFrameKind::Paragraph | FollowFrameKind::Notes) | None => {
        text_ref.y_pt
          + text_metrics.baseline_offset_in_line(&text_ref.style, text_ref.line_height_pt)
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
    let link = text_ref.hyperlink_url.as_ref().map(|url| PaintLink {
      x_pt: text_ref.x_pt,
      y_pt: text_box_y_pt,
      width_pt,
      height_pt: text_box_height_pt,
      url: url.clone(),
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
  link: Option<PaintLink<'doc>>,
}

fn text_paint_portions<'doc>(
  source: PaintTextPortionSource<'_, 'doc>,
  text_metrics: &mut TextMetrics,
) -> Vec<PaintTextPortion<'doc>> {
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
  let mut portions = Vec::with_capacity(ranges.len().max(1));
  let mut x_pt = text.x_pt;
  for (kind, range) in ranges {
    let portion_clip = paint_clip_for_portion(clip, &kind, page_width_pt);
    let portion_glyphs = glyphs
      .as_ref()
      .map(|glyphs| glyphs_for_text_range(glyphs, range.clone(), text.style.font_size_pt));
    let portion_width = portion_glyphs
      .as_ref()
      .map(|glyphs| glyph_runs_width_pt(glyphs, text.style.font_size_pt))
      .unwrap_or_else(|| text_metrics.measure_text(&text.text[range.clone()], &text.style));
    portions.push(PaintTextPortion {
      kind,
      text_range: range.clone(),
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

fn text_portion_ranges<'doc>(text: &TextItem<'doc>) -> PaintTextPortionRanges<'doc> {
  if text.text.is_empty() {
    return PaintTextPortionRanges::new();
  }
  if let Some(kind) = &text.dynamic_field {
    let mut ranges = PaintTextPortionRanges::new();
    ranges.push((
      PaintTextPortionKind::Field(kind.clone()),
      0..text.text.len(),
    ));
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

fn edge_whitespace_text_portion_ranges<'doc>(
  text: &TextItem<'doc>,
) -> PaintTextPortionRanges<'doc> {
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
    ranges.push((kind.clone(), 0..leading_end));
  }
  if leading_end < trailing_start {
    ranges.push((kind.clone(), leading_end..trailing_start));
  }
  if trailing_start < text.text.len() {
    ranges.push((kind, trailing_start..text.text.len()));
  }
  ranges
}

fn glyphs_for_text_range(
  glyphs: &[PaintGlyphFontRun],
  range: Range<usize>,
  font_size_pt: f32,
) -> PaintGlyphFontRuns {
  let mut output = PaintGlyphFontRuns::new();
  for run in glyphs {
    let mut x_pt = run.x_offset_pt;
    let mut active = None::<PaintGlyphFontRun>;
    for glyph in &run.glyphs {
      let intersects = glyph.text_range.start < range.end && glyph.text_range.end > range.start;
      if intersects {
        active
          .get_or_insert_with(|| PaintGlyphFontRun {
            font_face: run.font_face.clone(),
            x_offset_pt: x_pt,
            glyphs: Vec::with_capacity(run.glyphs.len().min(range.len())),
          })
          .glyphs
          .push(glyph.clone());
      } else if let Some(active) = active.take() {
        output.push(active);
      }
      x_pt += glyph.x_advance * font_size_pt;
    }
    if let Some(active) = active {
      output.push(active);
    }
  }
  output
}

fn glyph_runs_width_pt(glyphs: &[PaintGlyphFontRun], font_size_pt: f32) -> f32 {
  glyphs
    .iter()
    .flat_map(|run| run.glyphs.iter())
    .map(|glyph| glyph.x_advance * font_size_pt)
    .sum()
}

fn paint_rect_for_portion(rect: &PaintRect, x_pt: f32, width_pt: f32) -> PaintRect {
  PaintRect {
    x_pt,
    width_pt,
    ..rect.clone()
  }
}

fn paint_line_for_portion(line: &PaintStrokeLine, x_pt: f32, width_pt: f32) -> PaintStrokeLine {
  PaintStrokeLine {
    x1_pt: x_pt,
    x2_pt: x_pt + width_pt,
    ..line.clone()
  }
}

fn paint_link_for_portion<'doc>(
  link: &PaintLink<'doc>,
  x_pt: f32,
  width_pt: f32,
) -> PaintLink<'doc> {
  PaintLink {
    x_pt,
    width_pt,
    ..link.clone()
  }
}

fn paint_clip_for_portion(
  clip: Option<PaintClipRect>,
  kind: &PaintTextPortionKind<'_>,
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

fn draw_paint_item(
  surface: &mut Surface<'_>,
  item: &PaintItem<'_>,
  fonts: &mut FontSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
  options: &PdfOptions,
) {
  match item {
    PaintItem::Text(text) if !text.item.text.is_empty() => {
      draw_text_item(surface, text, fonts, internal_links, link_annotations);
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
      match decode_image(
        &image.data,
        image.content_type.as_deref(),
        options,
        Some(metafile_render_options_for_image(image, options)),
      ) {
        Ok(pdf_image) => draw_image_item(surface, image, pdf_image),
        Err(_) => draw_missing_image(surface, image),
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
) {
  let item = &text.item;
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
    surface.set_stroke(stroke(&item.style));
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
    if item.style.small_caps {
      let font = fonts.select(&item.style);
      surface.draw_text(
        Point::from_xy(portion.x_pt, portion.baseline_y),
        font,
        item.style.font_size_pt,
        &item.text[portion.text_range.clone()],
        false,
        TextDirection::Auto,
      );
    } else if let Some(glyphs) = &portion.glyphs {
      for run in glyphs {
        let font = fonts.select_face(&run.font_face);
        surface.draw_glyphs(
          Point::from_xy(portion.x_pt + run.x_offset_pt, portion.baseline_y),
          &run.glyphs,
          font,
          &item.text,
          item.style.font_size_pt,
          false,
        );
      }
    } else {
      let font = fonts.select(&item.style);
      surface.draw_text(
        Point::from_xy(portion.x_pt, portion.baseline_y),
        font,
        item.style.font_size_pt,
        &item.text[portion.text_range.clone()],
        false,
        TextDirection::Auto,
      );
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
    if let Some(link) = &portion.link
      && let Some(annotation) = link_annotation_for_rect(
        link.x_pt,
        link.y_pt,
        link.width_pt,
        link.height_pt,
        &link.url,
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

fn draw_polyline_item(surface: &mut Surface<'_>, polyline: &PolylineItem) {
  if polyline.points.len() < 2 {
    return;
  }

  if let Some(fill_color) = polyline.fill_color {
    surface.set_fill(Some(Fill {
      paint: rgb::Color::new(fill_color.r, fill_color.g, fill_color.b).into(),
      opacity: NormalizedF32::ONE,
      rule: FillRule::EvenOdd,
    }));
  } else {
    surface.set_fill(None);
  }
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
  let (first_x, first_y) = polyline.points[0];
  path.move_to(polyline.x_pt + first_x, polyline.y_pt + first_y);
  for &(x, y) in &polyline.points[1..] {
    path.line_to(polyline.x_pt + x, polyline.y_pt + y);
  }
  if polyline.closed {
    path.close();
  }
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
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

  if let Some(stroke) = rect.stroke {
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
  surface.push_transform(&Transform::from_translate(image.x_pt, image.y_pt));
  let mut pop_count = 1;

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
    surface.draw_image(pdf_image, size);
    surface.pop();
  }

  for _ in 0..pop_count {
    surface.pop();
  }
}

fn shaped_pdf_glyphs(
  text: &str,
  style: &TextStyle<'_>,
  text_metrics: &mut TextMetrics,
) -> Option<PaintGlyphRun> {
  let shaped = text_metrics.shape_text(text, style)?;
  let mut font_runs = PaintGlyphFontRuns::new();
  let mut x_offset_pt = 0.0;
  for glyph in shaped.glyphs {
    let font_face = shaped.font_faces.get(glyph.font_index)?.clone();
    if !font_runs
      .last()
      .is_some_and(|run| run.font_face == font_face)
    {
      font_runs.push(PaintGlyphFontRun {
        font_face,
        x_offset_pt,
        glyphs: Vec::new(),
      });
    }
    font_runs
      .last_mut()
      .expect("font run was just pushed")
      .glyphs
      .push(KrillaGlyph::new(
        GlyphId::new(glyph.glyph_id),
        glyph.x_advance_em,
        glyph.x_offset_em,
        glyph.y_offset_em,
        glyph.y_advance_em,
        glyph.text_range,
        None,
      ));
    x_offset_pt += glyph.x_advance_em * style.font_size_pt;
  }
  Some(PaintGlyphRun {
    width_pt: shaped.width_pt,
    font_runs,
  })
}

fn should_shape_pdf_glyphs(text: &TextItem<'_>) -> bool {
  // glyphs. This renderer only has text items at PDF time, so keep ordinary
  // Latin runs on krilla's text path and reserve explicit glyph painting for
  // cases where we need fallback faces or complex ActualText ranges.
  !text.style.small_caps
    && (text.paragraph_bidi
      || text_has_pdf_significant_spacing(&text.text)
      || text.preserve_text_portion && text.text.chars().any(requires_shaped_pdf_glyph)
      || text.text.chars().any(requires_shaped_pdf_glyph))
}

fn text_has_pdf_significant_spacing(text: &str) -> bool {
  text.starts_with(char::is_whitespace)
    || text.ends_with(char::is_whitespace)
    || text.as_bytes().windows(2).any(|window| window == b"  ")
}

fn requires_shaped_pdf_glyph(ch: char) -> bool {
  !ch.is_ascii()
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
