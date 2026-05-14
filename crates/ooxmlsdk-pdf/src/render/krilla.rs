use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use image::{GenericImageView, ImageFormat as RasterImageFormat, Rgba};
use krilla::action::{Action, LinkAction};
use krilla::annotation::{Annotation, LinkAnnotation, Target};
use krilla::color::rgb;
use krilla::configure::{Configuration, PdfVersion};
use krilla::destination::{Destination, XyzDestination};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::{BitsPerComponent, CustomImage, Image, ImageColorspace};
use krilla::num::NormalizedF32;
use krilla::outline::{Outline, OutlineNode};
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, Stroke};
use krilla::surface::Surface;
use krilla::text::{Font, GlyphId, KrillaGlyph, TextDirection};
use krilla::{Document, SerializeSettings};
use lopdf::{Document as LopdfDocument, Object as LopdfObject, dictionary};

use super::emf_wmf;
use crate::docx::{DynamicFieldKind, FormWidget, FormWidgetKind, RgbColor, TextStyle};
use crate::error::{PdfError, Result};
use crate::fonts::load_text_face;
use crate::layout::{
  FillItem, FollowFrameKind, ImageItem, LayoutDocument, LineItem, LineItemKind, OutlineEntry,
  PageItem, PdfTextSegmentation, RectItem, TextItem,
};
use crate::options::PdfOptions;
use crate::text_metrics::{
  baseline_offset_in_line, measure_text, shape_text, text_decoration_metrics, vertical_metrics,
};

const INTERNAL_LINK_DESTINATION_SHIFT_PT: f32 = 10.0;
// Source: LibreOffice sw/source/core/text/itrform2.cxx
// SwContentControlPortion::DescribePDFControl() expands content-control widget
// bounds by 20 twips before handing them to PDFWriter.
const LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT: f32 = 20.0 / crate::units::TWIPS_PER_POINT;
const LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT: f32 =
  LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT + LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT;
// Source: LibreOffice vcl/source/pdf/pdfwriter_impl.cxx
// PDFWriterImpl::emitWidgetAnnotations() applies iRectMargin = 1 for
// non-signature widget annotation rectangles.
const LO_PDF_WIDGET_ANNOTATION_MARGIN_PT: f32 = 1.0 / 1000.0;
const LO_PDF_COMBO_BOX_FLAGS: i64 = 0x00060000;
const LO_PDF_DROPDOWN_LIST_FLAGS: i64 = 0x00020000;

pub(crate) fn render(document: &LayoutDocument, options: &PdfOptions) -> Result<Vec<u8>> {
  debug_assert!(
    document
      .follows
      .iter()
      .all(|follow| follow.to_page_index < document.pages.len())
  );
  debug_assert!(document.frames.iter().all(|frame| {
    let _kind = frame.kind;
    let _block_index = frame.block_index;
    let _split_start = frame.split_start;
    let _split_end = frame.split_end;
    let _invalidation = frame.invalidation;
    frame.page_index < document.pages.len()
      && frame.section_index == document.pages[frame.page_index].section_index
      && frame.section_page_index == document.pages[frame.page_index].section_page_index
      && frame.item_start < frame.item_end
      && frame.items.len() == frame.item_end - frame.item_start
      && frame.column_index < 64
      && frame
        .bounds
        .is_none_or(|bounds| bounds.width_pt >= 0.0 && bounds.height_pt >= 0.0)
      && frame.lines.iter().all(|line| {
        line.item_start >= frame.item_start
          && line.item_end <= frame.item_end
          && line.item_start < line.item_end
          && line.width_pt >= 0.0
          && line.height_pt >= 0.0
          && line.x_pt.is_finite()
          && line.y_pt.is_finite()
      })
      && frame.fragments.iter().all(|fragment| {
        let _fragment_kind = fragment.kind;
        fragment.item_start >= frame.item_start
          && fragment.item_end <= frame.item_end
          && fragment.item_start < fragment.item_end
          && fragment
            .bounds
            .is_none_or(|bounds| bounds.width_pt >= 0.0 && bounds.height_pt >= 0.0)
      })
      && frame.influences.iter().all(|influence| {
        let _influence_kind = influence.kind;
        influence.count > 0
          && influence.block_index == frame.block_index
          && influence
            .bounds
            .is_none_or(|bounds| bounds.width_pt >= 0.0 && bounds.height_pt >= 0.0)
      })
  }));
  debug_assert!(document.reflow_requests.iter().all(|request| {
    document
      .frames
      .get(request.frame_index)
      .is_some_and(|frame| {
        let _reason = request.reason;
        let _scope = request.scope;
        frame.kind == request.kind
          && frame.page_index == request.page_index
          && frame.section_page_index == request.section_page_index
          && frame.column_index == request.column_index
          && frame.split_start == request.restart
          && request.influence_count == frame.influences.len()
      })
  }));
  debug_assert!(document.page_invalidations.iter().all(|invalidation| {
    document
      .frames
      .get(invalidation.first_frame_index)
      .is_some_and(|frame| {
        let _reason = invalidation.reason;
        let _scope = invalidation.scope;
        frame.page_index == invalidation.page_index
          && frame.section_page_index == invalidation.section_page_index
      })
  }));
  debug_assert!(document.page_replays.iter().all(|replay| {
    let _scope = replay.scope;
    replay.page_index < document.pages.len()
      && replay.item_start <= replay.item_end
      && replay.column_index < 64
      && replay.section_page_index == document.pages[replay.page_index].section_page_index
      && !replay.replacement_items.is_empty()
  }));
  debug_assert!(document.page_replay_applications.iter().all(|application| {
    let _scope = application.scope;
    application.page_index < document.pages.len()
      && application.item_start <= application.item_end
      && application.column_index < 64
      && application.section_page_index == document.pages[application.page_index].section_page_index
      && application.replacement_count > 0
      && application.applied
  }));
  debug_assert!(document.backward_moves.iter().all(|move_back| {
    let _scope = move_back.scope;
    let _reason = move_back.reason;
    move_back.frame_index < document.frames.len()
      && move_back.replay_start_frame_index < document.frames.len()
      && move_back.from_page_index < document.pages.len()
      && move_back.to_page_index < document.pages.len()
      && move_back.to_page_index <= move_back.from_page_index
      && (move_back.suppressed || move_back.replayed_frames > 0)
  }));
  debug_assert!(document.layout_reruns.iter().all(|rerun| {
    let _scope = rerun.scope;
    let _reason = rerun.reason;
    rerun.page_index < document.pages.len()
      && rerun.frame_index < document.frames.len()
      && rerun.produced_pages > 0
      && rerun.produced_frames > 0
      && rerun.constraints.iter().all(|constraint| {
        let _kind = constraint.kind;
        let _scope = constraint.scope;
        constraint.content_width >= 0.0
          && constraint.content_bottom.is_finite()
          && constraint
            .bounds
            .is_none_or(|bounds| bounds.width_pt >= 0.0 && bounds.height_pt >= 0.0)
      })
  }));
  debug_assert!(document.reflow_executions.iter().all(|execution| {
    let _action = execution.action;
    let _scope = execution.scope;
    execution.request_count > 0
      && execution.first_page_index < document.pages.len()
      && execution.backward_moves <= document.backward_moves.len()
  }));
  debug_assert!(document.restart_plan.as_ref().is_none_or(|plan| {
    document.frames.get(plan.frame_index).is_some_and(|frame| {
      let _reason = plan.reason;
      let _scope = plan.scope;
      frame.page_index == plan.page_index
        && frame.block_index == plan.block_index
        && frame.split_start == plan.cursor
    })
  }));
  let paint = PaintDocument::from_layout(document);
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
              match portion.kind {
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
                && portion
                  .glyphs
                  .as_ref()
                  .is_none_or(|glyphs| glyphs.iter().all(|glyph| glyph.x_advance >= 0.0))
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
        PaintItem::Image(_) | PaintItem::Rect(_) | PaintItem::Fill(_) | PaintItem::Line(_) => true,
      })
  }));
  let mut pdf = Document::new_with(serialize_settings(options));
  let fonts = FontSet::load()?;

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
        &fonts,
        &internal_links,
        &mut link_annotations,
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
  inject_form_widget_annotations(document, pdf)
}

#[derive(Clone, Debug)]
struct WidgetAnnotationSpec {
  page_index: usize,
  rect: [f32; 4],
  field_type: &'static [u8],
  field_value: String,
  field_flags: Option<i64>,
}

#[derive(Clone, Debug)]
struct WidgetBounds {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
  paragraph_bidi: bool,
  text: String,
}

fn inject_form_widget_annotations(document: &LayoutDocument, pdf: Vec<u8>) -> Result<Vec<u8>> {
  let annotations = collect_form_widget_annotations(document);
  if annotations.is_empty() {
    return Ok(pdf);
  }

  let mut patched =
    LopdfDocument::load_mem(&pdf).map_err(|error| PdfError::Lopdf(format!("{error}")))?;
  let pages = patched.get_pages();
  for annotation in annotations {
    let Some((_, page_id)) = pages.iter().nth(annotation.page_index) else {
      continue;
    };
    let annotation_id = patched.add_object(dictionary! {
      "Type" => LopdfObject::Name(b"Annot".to_vec()),
      "Subtype" => LopdfObject::Name(b"Widget".to_vec()),
      "FT" => LopdfObject::Name(annotation.field_type.to_vec()),
      "V" => LopdfObject::String(
        annotation.field_value.clone().into_bytes(),
        lopdf::StringFormat::Literal,
      ),
      "DV" => LopdfObject::String(
        annotation.field_value.into_bytes(),
        lopdf::StringFormat::Literal,
      ),
      "Rect" => LopdfObject::Array(
        annotation
          .rect
          .into_iter()
          .map(LopdfObject::Real)
          .collect(),
      ),
    });
    if let Some(field_flags) = annotation.field_flags {
      patched
        .get_object_mut(annotation_id)
        .and_then(LopdfObject::as_dict_mut)
        .map_err(|error| PdfError::Lopdf(format!("{error}")))?
        .set("Ff", LopdfObject::Integer(field_flags));
    }
    let page = patched
      .get_object_mut(*page_id)
      .and_then(LopdfObject::as_dict_mut)
      .map_err(|error| PdfError::Lopdf(format!("{error}")))?;
    match page.get_mut(b"Annots") {
      Ok(annots) => annots
        .as_array_mut()
        .map_err(|error| PdfError::Lopdf(format!("{error}")))?
        .push(LopdfObject::Reference(annotation_id)),
      Err(lopdf::Error::DictKey(_)) => {
        page.set(
          "Annots",
          LopdfObject::Array(vec![LopdfObject::Reference(annotation_id)]),
        );
      }
      Err(error) => return Err(PdfError::Lopdf(format!("{error}"))),
    }
  }

  let mut output = Vec::new();
  patched
    .save_to(&mut output)
    .map_err(|error| PdfError::Lopdf(format!("{error}")))?;
  Ok(output)
}

fn collect_form_widget_annotations(document: &LayoutDocument) -> Vec<WidgetAnnotationSpec> {
  let mut annotations = Vec::new();
  for (page_index, page) in document.pages.iter().enumerate() {
    let mut widgets = HashMap::<u32, WidgetBounds>::new();
    for item in &page.items {
      let PageItem::Text(text) = item else {
        continue;
      };
      let Some(widget_id) = text.form_widget_id else {
        continue;
      };
      if !document
        .form_widgets
        .iter()
        .any(|widget| widget.id == widget_id)
      {
        continue;
      }
      let width = measure_text(&text.text, &text.style);
      let bounds = WidgetBounds {
        left: text.x_pt,
        top: text.y_pt,
        right: text.x_pt + width,
        bottom: text.y_pt + text.line_height_pt,
        paragraph_bidi: text.paragraph_bidi,
        text: text.text.clone(),
      };
      widgets
        .entry(widget_id)
        .and_modify(|current| {
          current.left = current.left.min(bounds.left);
          current.top = current.top.min(bounds.top);
          current.right = current.right.max(bounds.right);
          current.bottom = current.bottom.max(bounds.bottom);
          current.paragraph_bidi |= bounds.paragraph_bidi;
          current.text.push_str(&bounds.text);
        })
        .or_insert(bounds);
    }
    let page_height = page.setup.height_pt;
    let content_left = page.setup.margin_left_pt;
    let content_right = page.setup.width_pt - page.setup.margin_right_pt;
    let mut page_annotations = widgets
      .into_iter()
      .filter_map(|(widget_id, bounds)| {
        let widget = document
          .form_widgets
          .iter()
          .find(|widget| widget.id == widget_id)?;
        let (left, right) = if bounds.paragraph_bidi {
          let mirrored_left = content_left + content_right - bounds.right;
          let mirrored_right = content_left + content_right - bounds.left;
          (mirrored_left, mirrored_right)
        } else {
          (bounds.left, bounds.right)
        };
        let left = lo_swrect_leading_edge(left) - LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT;
        let top = lo_swrect_leading_edge(bounds.top);
        let right = lo_swrect_trailing_x_edge(right) + LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT;
        let bottom =
          lo_swrect_leading_edge(bounds.bottom) + LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT;
        // Source: LibreOffice sw/source/core/text/itrform2.cxx obtains the
        // widget location from SwTextCursor::GetCharRect() after the bidi text
        // frame has been laid out. In this path the exported bidi control
        // rectangle is one Writer expansion unit lower than the text ink box.
        let bidi_block_offset = if bounds.paragraph_bidi {
          LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT
        } else {
          0.0
        };
        let (field_type, field_value, field_flags) = widget_annotation_field(widget, &bounds.text);
        Some(WidgetAnnotationSpec {
          page_index,
          field_type,
          field_value,
          field_flags,
          rect: [
            round_annotation_coordinate(left - LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
            round_annotation_coordinate(
              page_height - bottom - bidi_block_offset + LO_PDF_WIDGET_ANNOTATION_MARGIN_PT,
            ),
            round_annotation_coordinate(right + LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
            round_annotation_coordinate(
              page_height - top - bidi_block_offset - LO_PDF_WIDGET_ANNOTATION_MARGIN_PT,
            ),
          ],
        })
      })
      .collect::<Vec<_>>();
    page_annotations.sort_by(|left, right| {
      left.rect[1]
        .total_cmp(&right.rect[1])
        .reverse()
        .then_with(|| left.rect[0].total_cmp(&right.rect[0]))
    });
    annotations.extend(page_annotations);
  }
  annotations
}

fn lo_swrect_leading_edge(value: f32) -> f32 {
  (value * crate::units::TWIPS_PER_POINT).floor() / crate::units::TWIPS_PER_POINT
}

fn lo_swrect_trailing_x_edge(value: f32) -> f32 {
  (value * crate::units::TWIPS_PER_POINT).ceil() / crate::units::TWIPS_PER_POINT
}

fn widget_annotation_field(
  widget: &FormWidget,
  text: &str,
) -> (&'static [u8], String, Option<i64>) {
  match widget.kind {
    FormWidgetKind::Text => (b"Tx", text.to_string(), None),
    FormWidgetKind::ComboBox => (b"Ch", text.to_string(), Some(LO_PDF_COMBO_BOX_FLAGS)),
    FormWidgetKind::DropDownList => {
      let value = if widget.entries.iter().any(|entry| entry == text) {
        text.to_string()
      } else {
        widget
          .entries
          .first()
          .cloned()
          .unwrap_or_else(|| text.to_string())
      };
      (b"Ch", value, Some(LO_PDF_DROPDOWN_LIST_FLAGS))
    }
  }
}

fn round_annotation_coordinate(value: f32) -> f32 {
  format!("{value:.3}").parse().unwrap_or(value)
}

#[derive(Clone, Debug)]
struct PaintDocument {
  pages: Vec<PaintPage>,
}

#[derive(Clone, Debug)]
struct PaintPage {
  width_pt: f32,
  height_pt: f32,
  items: Vec<PaintItem>,
}

#[derive(Clone, Copy, Debug, Default)]
struct DecorationRenderMetadata {
  suppress: bool,
  span_start_x_pt: Option<f32>,
}

#[derive(Clone, Debug)]
enum PaintItem {
  Text(PaintText),
  Image(ImageItem),
  Rect(RectItem),
  Fill(FillItem),
  Line(LineItem),
}

#[derive(Clone, Debug)]
struct PaintText {
  item: TextItem,
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
  glyphs: Option<Vec<KrillaGlyph>>,
  highlight: Option<PaintRect>,
  underline: Option<PaintStrokeLine>,
  strikethrough: Option<PaintStrokeLine>,
  link: Option<PaintLink>,
}

#[derive(Clone, Debug)]
enum PaintTextPortionKind {
  Text,
  Tab,
  Field(DynamicFieldKind),
  Link,
}

#[derive(Clone, Debug)]
struct PaintGlyphRun {
  width_pt: f32,
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
struct PaintLink {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  url: String,
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
  fn from_paint(paint: &PaintDocument) -> Self {
    let mut positions = HashMap::new();
    for (page_index, page) in paint.pages.iter().enumerate() {
      for item in &page.items {
        match item {
          PaintItem::Text(text) => {
            if let Some(url) = &text.item.hyperlink_url
              && is_internal_link_url(url)
            {
              positions
                .entry(url.clone())
                .or_insert(InternalLinkPosition {
                  page_index,
                  x_pt: text.item.x_pt,
                  // Source: Typst typst-pdf/src/link.rs pos_to_xyz shifts
                  // position links upward by 10pt so baseline targets remain visible.
                  y_pt: (text.baseline_y - INTERNAL_LINK_DESTINATION_SHIFT_PT).max(0.0),
                });
            }
          }
          PaintItem::Image(_) | PaintItem::Rect(_) | PaintItem::Fill(_) | PaintItem::Line(_) => {}
        }
      }
    }
    Self { positions }
  }

  fn target_for_url(&self, url: &str) -> Option<Target> {
    let target_url = reciprocal_internal_link_url(url)?;
    let position = self.positions.get(&target_url)?;
    Some(Target::Destination(Destination::Xyz(XyzDestination::new(
      position.page_index,
      Point::from_xy(position.x_pt, position.y_pt),
    ))))
  }
}

fn decoration_render_metadata(items: &[PageItem]) -> Vec<DecorationRenderMetadata> {
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

fn decoration_compatible(current: &TextItem, next: &TextItem) -> bool {
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
  let rest = url.strip_prefix("ooxmlsdk-pdf:")?;
  let (kind, id) = rest.rsplit_once(':')?;
  let target_kind = if let Some(note_kind) = kind.strip_suffix("-reference") {
    format!("{note_kind}-backlink")
  } else if let Some(note_kind) = kind.strip_suffix("-backlink") {
    format!("{note_kind}-reference")
  } else {
    return None;
  };
  Some(format!("ooxmlsdk-pdf:{target_kind}:{id}"))
}

impl PaintDocument {
  fn from_layout(document: &LayoutDocument) -> Self {
    let pages = document
      .pages
      .iter()
      .enumerate()
      .map(|(page_index, page)| {
        let layout_items = coalesced_writer_text_items(&page.items);
        let line_owners = paint_line_owners(document, page_index, layout_items.len());
        let decoration_metadata = decoration_render_metadata(&layout_items);
        let items = layout_items
          .iter()
          .enumerate()
          .map(|(item_index, item)| match item {
            PageItem::Text(text) => {
              let owner = line_owners.get(item_index).copied().flatten();
              let mut text = text.clone();
              let metadata = decoration_metadata[item_index];
              if metadata.suppress {
                text.style.underline = false;
                text.style.strikethrough = false;
              }
              text.decoration_span_start_x_pt = metadata.span_start_x_pt;
              PaintItem::Text(PaintText::from_layout_text(
                &text,
                owner,
                page.setup.width_pt,
              ))
            }
            PageItem::Image(image) => PaintItem::Image(image.clone()),
            PageItem::Rect(rect) => PaintItem::Rect(*rect),
            PageItem::Fill(fill) => PaintItem::Fill(*fill),
            PageItem::Line(line) => PaintItem::Line(*line),
          })
          .collect();
        PaintPage {
          width_pt: page.setup.width_pt,
          height_pt: page.setup.height_pt,
          items,
        }
      })
      .collect();
    Self { pages }
  }
}

fn coalesced_writer_text_items(items: &[PageItem]) -> Vec<PageItem> {
  let mut output: Vec<PageItem> = Vec::with_capacity(items.len());
  for item in items {
    let PageItem::Text(text) = item else {
      output.push(item.clone());
      continue;
    };
    if let Some(PageItem::Text(previous)) = output.last_mut()
      && writer_text_items_coalesce(previous, text)
    {
      previous.text.push_str(&text.text);
      previous.line_height_pt = previous.line_height_pt.max(text.line_height_pt);
      continue;
    }
    output.push(PageItem::Text(text.clone()));
  }
  output
}

fn writer_text_items_coalesce(current: &TextItem, next: &TextItem) -> bool {
  if current.pdf_text_segmentation != next.pdf_text_segmentation
    || current.form_widget_id.is_some()
    || next.form_widget_id.is_some()
  {
    return false;
  }
  if current.pdf_text_segmentation == PdfTextSegmentation::Portion
    && (current.text.contains('\t') || next.text.contains('\t'))
  {
    return false;
  }
  if current.style != next.style
    || current.hyperlink_url != next.hyperlink_url
    || current.dynamic_field != next.dynamic_field
    || current.paragraph_bidi != next.paragraph_bidi
    || current.decoration_span_start_x_pt != next.decoration_span_start_x_pt
    || (current.y_pt - next.y_pt).abs() >= 0.01
    || (current.line_height_pt - next.line_height_pt).abs() >= 0.01
  {
    return false;
  }
  let current_right = current.x_pt + measure_text(&current.text, &current.style);
  (current_right - next.x_pt).abs() < 0.25
}

impl PaintText {
  fn from_layout_text(text: &TextItem, owner: Option<PaintLineOwner>, page_width_pt: f32) -> Self {
    let glyphs = shaped_pdf_glyphs(&text.text, &text.style);
    let width_pt = glyphs
      .as_ref()
      .map(|run| run.width_pt)
      .unwrap_or_else(|| measure_text(&text.text, &text.style));
    let baseline_y = match owner.map(|owner| owner.frame_kind) {
      Some(FollowFrameKind::Table) => text.y_pt - text.style.baseline_shift_pt,
      Some(FollowFrameKind::Paragraph | FollowFrameKind::Notes) | None => {
        text.y_pt + baseline_offset_in_line(&text.style, text.line_height_pt)
      }
    };
    let vertical_metrics = vertical_metrics(&text.style);
    let text_box_y_pt =
      baseline_y - vertical_metrics.ascent_pt - vertical_metrics.leading_above_pt();
    let text_box_height_pt = vertical_metrics.line_height_pt();
    let highlight = text.style.highlight.map(|color| PaintRect {
      x_pt: text.x_pt,
      y_pt: text_box_y_pt,
      width_pt,
      height_pt: text_box_height_pt,
      color,
    });
    let decoration_metrics = text_decoration_metrics(&text.style);
    let decoration_start_x_pt = text.decoration_span_start_x_pt.unwrap_or(text.x_pt);
    let underline_y_pt = baseline_y + decoration_metrics.underline_offset_pt;
    let underline = text.style.underline.then_some(PaintStrokeLine {
      x1_pt: decoration_start_x_pt,
      y1_pt: underline_y_pt,
      x2_pt: text.x_pt + width_pt,
      y2_pt: underline_y_pt,
      width_pt: decoration_metrics.underline_width_pt,
      color: text.style.color,
    });
    let strikethrough_y_pt = baseline_y - decoration_metrics.strikethrough_offset_pt;
    let strikethrough = text.style.strikethrough.then_some(PaintStrokeLine {
      x1_pt: decoration_start_x_pt,
      y1_pt: strikethrough_y_pt,
      x2_pt: text.x_pt + width_pt,
      y2_pt: strikethrough_y_pt,
      width_pt: decoration_metrics.strikethrough_width_pt,
      color: text.style.color,
    });
    let link = text.hyperlink_url.as_ref().map(|url| PaintLink {
      x_pt: text.x_pt,
      y_pt: text_box_y_pt,
      width_pt,
      height_pt: text_box_height_pt,
      url: url.clone(),
    });

    Self {
      item: text.clone(),
      source_frame_index: owner.map(|owner| owner.frame_index),
      source_line_index: owner.map(|owner| owner.line_index),
      baseline_y,
      width_pt,
      portions: text_paint_portions(PaintTextPortionSource {
        text,
        baseline_y,
        width_pt,
        page_width_pt,
        clip: owner.map(|owner| owner.clip),
        glyphs: glyphs.map(|run| run.glyphs),
        highlight,
        underline,
        strikethrough,
        link,
      }),
    }
  }
}

struct PaintTextPortionSource<'a> {
  text: &'a TextItem,
  baseline_y: f32,
  width_pt: f32,
  page_width_pt: f32,
  clip: Option<PaintClipRect>,
  glyphs: Option<Vec<KrillaGlyph>>,
  highlight: Option<PaintRect>,
  underline: Option<PaintStrokeLine>,
  strikethrough: Option<PaintStrokeLine>,
  link: Option<PaintLink>,
}

fn text_paint_portions(source: PaintTextPortionSource<'_>) -> Vec<PaintTextPortion> {
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
      .map(|glyphs| glyphs_for_text_range(glyphs, range.clone()));
    let portion_width = portion_glyphs
      .as_ref()
      .map(|glyphs| glyph_width_pt(glyphs, text.style.font_size_pt))
      .unwrap_or_else(|| measure_text(&text.text[range.clone()], &text.style));
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

fn text_portion_ranges(text: &TextItem) -> Vec<(PaintTextPortionKind, std::ops::Range<usize>)> {
  if text.text.is_empty() {
    return Vec::new();
  }
  if let Some(kind) = text.dynamic_field {
    return vec![(PaintTextPortionKind::Field(kind), 0..text.text.len())];
  }
  let split_portions = text.pdf_text_segmentation == PdfTextSegmentation::Portion
    || text.style.underline
    || text.style.strikethrough;
  if !split_portions && text.hyperlink_url.is_some() && !text.text.contains('\t') {
    return vec![(PaintTextPortionKind::Link, 0..text.text.len())];
  }

  let mut ranges = Vec::new();
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
    } else if split_portions {
      if start < index {
        start = index;
      }
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

fn glyphs_for_text_range(
  glyphs: &[KrillaGlyph],
  range: std::ops::Range<usize>,
) -> Vec<KrillaGlyph> {
  glyphs
    .iter()
    .filter(|glyph| glyph.text_range.start < range.end && glyph.text_range.end > range.start)
    .cloned()
    .collect()
}

fn glyph_width_pt(glyphs: &[KrillaGlyph], font_size_pt: f32) -> f32 {
  glyphs
    .iter()
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

fn paint_link_for_portion(link: &PaintLink, x_pt: f32, width_pt: f32) -> PaintLink {
  PaintLink {
    x_pt,
    width_pt,
    ..link.clone()
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
  document: &LayoutDocument,
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
      let start = line.item_start.min(item_count);
      let end = line.item_end.min(item_count);
      for owner in owners.iter_mut().take(end).skip(start) {
        if owner.is_none() {
          *owner = Some(PaintLineOwner {
            frame_index,
            line_index,
            frame_kind: frame.kind,
            clip: PaintClipRect {
              x_pt: line.x_pt,
              y_pt: line.y_pt,
              width_pt: line.width_pt,
              height_pt: line.height_pt,
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
  item: &PaintItem,
  fonts: &FontSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
) {
  match item {
    PaintItem::Text(text) if !text.item.text.is_empty() => {
      draw_text_item(surface, text, fonts, internal_links, link_annotations);
    }
    PaintItem::Text(_) => {}
    PaintItem::Rect(rect) => draw_rect_item(surface, rect),
    PaintItem::Fill(fill_item) => draw_fill_item(surface, fill_item),
    PaintItem::Image(image) => {
      let _alt_text = image.alt_text.as_deref();
      match decode_image(&image.data, image.content_type.as_deref()) {
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
  }
}

fn paint_item_intersects_page(item: &PaintItem, page_width_pt: f32, page_height_pt: f32) -> bool {
  // Source: LibreOffice sw/source/core/view/vprint.cxx intersects output with
  // the page rectangle before SwRootFrame::PaintSwFrame(); drawing layers also
  // receive the page frame in sw/source/core/view/vdraw.cxx.
  let Some((left, top, right, bottom)) = paint_item_bounds(item) else {
    return true;
  };
  right > 0.0 && bottom > 0.0 && left < page_width_pt && top < page_height_pt
}

fn paint_item_bounds(item: &PaintItem) -> Option<(f32, f32, f32, f32)> {
  match item {
    PaintItem::Text(text) => {
      let item = &text.item;
      Some((
        item.x_pt,
        item.y_pt,
        item.x_pt + text.width_pt,
        item.y_pt + item.line_height_pt,
      ))
    }
    PaintItem::Image(image) => Some((
      image.x_pt,
      image.y_pt,
      image.x_pt + image.width_pt,
      image.y_pt + image.height_pt,
    )),
    PaintItem::Rect(rect) => Some((
      rect.x_pt,
      rect.y_pt,
      rect.x_pt + rect.width_pt,
      rect.y_pt + rect.height_pt,
    )),
    PaintItem::Fill(fill) => Some((
      fill.x_pt,
      fill.y_pt,
      fill.x_pt + fill.width_pt,
      fill.y_pt + fill.height_pt,
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
  }
}

fn pdf_outline_for_entries(entries: &[OutlineEntry]) -> Option<Outline> {
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

fn pdf_outline_node(entries: &[OutlineEntry], index: &mut usize, level: u8) -> OutlineNode {
  let entry = &entries[*index];
  *index += 1;
  let mut node = OutlineNode::new(
    entry.text.clone(),
    XyzDestination::new(entry.page_index, Point::from_xy(entry.x_pt, entry.y_pt)),
  );
  while *index < entries.len() && entries[*index].level > level {
    let child_level = entries[*index].level;
    node.push_child(pdf_outline_node(entries, index, child_level));
  }
  node
}

fn draw_text_item(
  surface: &mut Surface<'_>,
  text: &PaintText,
  fonts: &FontSet,
  internal_links: &InternalLinkTargets,
  link_annotations: &mut Vec<Annotation>,
) {
  let item = &text.item;
  for portion in &text.portions {
    let clipped = push_paint_clip(surface, portion.clip.as_ref());
    if let Some(highlight) = &portion.highlight {
      draw_paint_rect(surface, highlight);
    }
    surface.set_stroke(stroke(&item.style));
    surface.set_fill(Some(fill(&item.style)));
    if let Some(glyphs) = &portion.glyphs {
      let font = fonts.select(&item.style);
      surface.draw_glyphs(
        Point::from_xy(portion.x_pt, portion.baseline_y),
        glyphs,
        font,
        &item.text,
        item.style.font_size_pt,
        false,
      );
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
  url.to_string()
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

fn draw_fill_item(surface: &mut Surface<'_>, fill_item: &FillItem) {
  surface.set_stroke(None);
  surface.set_fill(Some(Fill {
    paint: rgb::Color::new(fill_item.color.r, fill_item.color.g, fill_item.color.b).into(),
    opacity: NormalizedF32::ONE,
    rule: Default::default(),
  }));
  let mut path = PathBuilder::new();
  path.move_to(fill_item.x_pt, fill_item.y_pt);
  path.line_to(fill_item.x_pt + fill_item.width_pt, fill_item.y_pt);
  path.line_to(
    fill_item.x_pt + fill_item.width_pt,
    fill_item.y_pt + fill_item.height_pt,
  );
  path.line_to(fill_item.x_pt, fill_item.y_pt + fill_item.height_pt);
  path.close();
  if let Some(path) = path.finish() {
    surface.draw_path(&path);
  }
}

fn draw_missing_image(surface: &mut Surface<'_>, image: &ImageItem) {
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

fn draw_rect_item(surface: &mut Surface<'_>, rect: &RectItem) {
  if let Some(fill_color) = rect.fill_color {
    surface.set_fill(Some(Fill {
      paint: rgb::Color::new(fill_color.r, fill_color.g, fill_color.b).into(),
      opacity: NormalizedF32::ONE,
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

fn draw_image_item(surface: &mut Surface<'_>, image: &ImageItem, pdf_image: Image) {
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

fn shaped_pdf_glyphs(text: &str, style: &TextStyle) -> Option<PaintGlyphRun> {
  let shaped = shape_text(text, style)?;
  let glyphs = shaped
    .glyphs
    .into_iter()
    .map(|glyph| {
      KrillaGlyph::new(
        GlyphId::new(glyph.glyph_id),
        glyph.x_advance_em,
        glyph.x_offset_em,
        glyph.y_offset_em,
        glyph.y_advance_em,
        glyph.text_range,
        None,
      )
    })
    .collect();
  Some(PaintGlyphRun {
    width_pt: shaped.width_pt,
    glyphs,
  })
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

fn decode_image(data: &[u8], content_type: Option<&str>) -> Result<Image> {
  if let Some(jpeg) = emf_wmf::decode_metafile_as_jpeg(data, content_type)
    .map_err(|err| PdfError::Krilla(format!("failed to decode EMF/WMF image: {err}")))?
  {
    return Image::from_jpeg(jpeg.into(), true).map_err(PdfError::Krilla);
  }

  let format = content_type
    .and_then(image_format_from_content_type)
    .or_else(|| image::guess_format(data).ok());

  if matches!(format, Some(RasterImageFormat::Jpeg))
    && let Ok(image) = Image::from_jpeg(data.to_vec().into(), true)
  {
    return Ok(image);
  }
  if matches!(format, Some(RasterImageFormat::Png))
    && let Ok(image) = decode_png_relaxed(data)
  {
    return Image::from_custom(image, true).map_err(PdfError::Krilla);
  }

  let raster = match format {
    Some(format) => image::load_from_memory_with_format(data, format),
    None => image::load_from_memory(data),
  };

  let raster =
    raster.map_err(|err| PdfError::Krilla(format!("failed to decode raster image: {err}")))?;
  Image::from_custom(PdfRasterImage::from_dynamic(raster), true).map_err(PdfError::Krilla)
}

fn decode_png_relaxed(data: &[u8]) -> std::result::Result<PdfRasterImage, String> {
  let mut decoder = png::Decoder::new(Cursor::new(data));
  decoder.ignore_checksums(true);
  decoder.set_transformations(png::Transformations::normalize_to_color8());
  let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
  let buffer_size = reader
    .output_buffer_size()
    .ok_or_else(|| "PNG output buffer size is unavailable".to_string())?;
  let mut buffer = vec![0; buffer_size];
  let info = reader
    .next_frame(&mut buffer)
    .map_err(|err| err.to_string())?;
  buffer.truncate(info.buffer_size());
  Ok(PdfRasterImage::from_png_frame(
    info.width,
    info.height,
    info.color_type,
    &buffer,
  ))
}

fn image_format_from_content_type(content_type: &str) -> Option<RasterImageFormat> {
  match content_type {
    "image/png" => Some(RasterImageFormat::Png),
    "image/jpeg" | "image/jpg" => Some(RasterImageFormat::Jpeg),
    "image/gif" => Some(RasterImageFormat::Gif),
    "image/webp" => Some(RasterImageFormat::WebP),
    _ => None,
  }
}

#[derive(Clone, Debug)]
struct PdfRasterImage {
  pixels: Arc<PdfRasterPixels>,
}

#[derive(Debug)]
struct PdfRasterPixels {
  width: u32,
  height: u32,
  rgb: Vec<u8>,
  alpha: Option<Vec<u8>>,
}

impl PdfRasterImage {
  fn from_dynamic(image: image::DynamicImage) -> Self {
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8();
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    let mut alpha = Vec::with_capacity(width as usize * height as usize);
    let mut opaque = true;

    for Rgba([r, g, b, a]) in rgba.pixels() {
      rgb.extend_from_slice(&[*r, *g, *b]);
      alpha.push(*a);
      opaque &= *a == u8::MAX;
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque).then_some(alpha),
      }),
    }
  }

  fn from_png_frame(width: u32, height: u32, color_type: png::ColorType, data: &[u8]) -> Self {
    let pixel_count = width as usize * height as usize;
    let mut rgb = Vec::with_capacity(pixel_count * 3);
    let mut alpha = Vec::with_capacity(pixel_count);
    let mut opaque = true;

    match color_type {
      png::ColorType::Grayscale => {
        for value in data {
          rgb.extend_from_slice(&[*value, *value, *value]);
        }
      }
      png::ColorType::GrayscaleAlpha => {
        for pixel in data.chunks_exact(2) {
          rgb.extend_from_slice(&[pixel[0], pixel[0], pixel[0]]);
          alpha.push(pixel[1]);
          opaque &= pixel[1] == u8::MAX;
        }
      }
      png::ColorType::Rgb => {
        rgb.extend_from_slice(data);
      }
      png::ColorType::Rgba => {
        for pixel in data.chunks_exact(4) {
          rgb.extend_from_slice(&pixel[..3]);
          alpha.push(pixel[3]);
          opaque &= pixel[3] == u8::MAX;
        }
      }
      png::ColorType::Indexed => {}
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque && !alpha.is_empty()).then_some(alpha),
      }),
    }
  }
}

impl Hash for PdfRasterImage {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pixels.width.hash(state);
    self.pixels.height.hash(state);
    self.pixels.rgb.hash(state);
    self.pixels.alpha.hash(state);
  }
}

impl CustomImage for PdfRasterImage {
  fn color_channel(&self) -> &[u8] {
    &self.pixels.rgb
  }

  fn alpha_channel(&self) -> Option<&[u8]> {
    self.pixels.alpha.as_deref()
  }

  fn bits_per_component(&self) -> BitsPerComponent {
    BitsPerComponent::Eight
  }

  fn size(&self) -> (u32, u32) {
    (self.pixels.width, self.pixels.height)
  }

  fn icc_profile(&self) -> Option<&[u8]> {
    None
  }

  fn color_space(&self) -> ImageColorspace {
    ImageColorspace::Rgb
  }
}

fn serialize_settings(options: &PdfOptions) -> SerializeSettings {
  SerializeSettings {
    compress_content_streams: options.compress_content_streams,
    no_device_cs: true,
    ascii_compatible: false,
    xmp_metadata: true,
    cmyk_profile: None,
    configuration: Configuration::new_with_version(PdfVersion::Pdf17),
    enable_tagging: false,
    render_svg_glyph_fn: krilla_svg::render_svg_glyph,
  }
}

struct FontSet {
  fallback: Font,
  fonts: Mutex<HashMap<FontKey, Font>>,
}

impl FontSet {
  fn load() -> Result<Self> {
    let fallback_style = TextStyle::default();
    Ok(Self {
      fallback: load_font(&fallback_style)?,
      fonts: Mutex::new(HashMap::new()),
    })
  }

  fn select(&self, style: &TextStyle) -> Font {
    let key = FontKey {
      family: style.font_family.as_deref().map(str::to_string),
      bold: style.bold,
      italic: style.italic,
    };
    let Ok(mut fonts) = self.fonts.lock() else {
      return self.fallback.clone();
    };
    if let Some(font) = fonts.get(&key) {
      return font.clone();
    }
    let font = load_font(style).unwrap_or_else(|_| self.fallback.clone());
    fonts.insert(key, font.clone());
    font
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

fn load_font(style: &TextStyle) -> Result<Font> {
  if let Some(face) = load_text_face(style)
    && let Some(font) = Font::new(Arc::new(face.data).into(), face.index)
  {
    return Ok(font);
  }

  Err(PdfError::font_unavailable(style))
}

fn fill(style: &TextStyle) -> Fill {
  Fill {
    paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
    opacity: NormalizedF32::new(style.opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ZERO),
    rule: Default::default(),
  }
}

fn stroke(style: &TextStyle) -> Option<Stroke> {
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
