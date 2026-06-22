use std::collections::HashMap;

use lopdf::{Document as LopdfDocument, Object as LopdfObject, dictionary};

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common::{self, FormWidget, FormWidgetKind};
use ooxmlsdk_layout::text_metrics::TextMetrics;
// SwContentControlPortion::DescribePDFControl() expands content-control widget
// bounds by 20 twips before handing them to PDFWriter.
const LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT: f32 = 20.0 / ooxmlsdk_layout::units::TWIPS_PER_POINT;
const LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT: f32 =
  LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT + LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT;
const LO_CONTENT_CONTROL_WIDGET_VERTICAL_OFFSET_PT: f32 =
  19.0 / ooxmlsdk_layout::units::TWIPS_PER_POINT;
// PDFWriterImpl::emitWidgetAnnotations() applies iRectMargin = 1 for
// non-signature widget annotation rectangles.
const LO_PDF_WIDGET_ANNOTATION_MARGIN_PT: f32 = 1.0 / 1000.0;
const LO_PDF_COMBO_BOX_FLAGS: i64 = 0x00060000;
const LO_PDF_DROPDOWN_LIST_FLAGS: i64 = 0x00020000;

pub(super) struct WidgetAnnotationSpec {
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

pub(super) fn inject_form_widget_annotations(
  pdf: Vec<u8>,
  annotations: Vec<WidgetAnnotationSpec>,
) -> Result<Vec<u8>> {
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

pub(super) fn collect_form_widget_annotations(
  document: &common::LayoutDocument<'static>,
  text_metrics: &mut TextMetrics,
) -> Vec<WidgetAnnotationSpec> {
  let mut annotations = Vec::new();
  for (page_index, page) in document.pages.iter().enumerate() {
    let mut widgets = HashMap::<u32, WidgetBounds>::new();
    for item in &page.items {
      let common::DisplayItem::Text(text) = item else {
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
      let width = text_metrics.measure_text(text.text.as_ref(), &text.style);
      let bounds = WidgetBounds {
        left: text.origin.x.0,
        top: text.origin.y.0,
        right: text.origin.x.0 + width,
        bottom: text.origin.y.0 + text.line_height.0,
        paragraph_bidi: text.paragraph_bidi,
        text: text.text.to_string(),
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
    let page_height = page.setup.size.height.0;
    let content_left = page.setup.margins.left.0;
    let content_right = page.setup.size.width.0 - page.setup.margins.right.0;
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
        let top = lo_swrect_leading_edge(bounds.top) - LO_CONTENT_CONTROL_WIDGET_VERTICAL_OFFSET_PT;
        let right = lo_swrect_trailing_x_edge(right) + LO_CONTENT_CONTROL_WIDGET_EXPANSION_PT;
        let bottom = lo_swrect_leading_edge(bounds.bottom)
          + LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT
          - LO_CONTENT_CONTROL_WIDGET_VERTICAL_OFFSET_PT;
        let (field_type, field_value, field_flags) = widget_annotation_field(widget, &bounds.text);
        Some(WidgetAnnotationSpec {
          page_index,
          field_type,
          field_value,
          field_flags,
          rect: [
            round_annotation_coordinate(left - LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
            round_annotation_coordinate(page_height - bottom + LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
            round_annotation_coordinate(right + LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
            round_annotation_coordinate(page_height - top - LO_PDF_WIDGET_ANNOTATION_MARGIN_PT),
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
  (value * ooxmlsdk_layout::units::TWIPS_PER_POINT).floor()
    / ooxmlsdk_layout::units::TWIPS_PER_POINT
}

fn lo_swrect_trailing_x_edge(value: f32) -> f32 {
  (value * ooxmlsdk_layout::units::TWIPS_PER_POINT).ceil() / ooxmlsdk_layout::units::TWIPS_PER_POINT
}

fn widget_annotation_field(
  widget: &FormWidget,
  text: &str,
) -> (&'static [u8], String, Option<i64>) {
  match widget.kind {
    FormWidgetKind::Text => (b"Tx", text.to_string(), None),
    FormWidgetKind::ComboBox => (b"Ch", text.to_string(), Some(LO_PDF_COMBO_BOX_FLAGS)),
    FormWidgetKind::DropDownList => {
      let value = if widget.entries.iter().any(|entry| entry.as_ref() == text) {
        text.to_string()
      } else {
        widget
          .entries
          .first()
          .map(|entry| entry.to_string())
          .unwrap_or_else(|| text.to_string())
      };
      (b"Ch", value, Some(LO_PDF_DROPDOWN_LIST_FLAGS))
    }
  }
}

fn round_annotation_coordinate(value: f32) -> f32 {
  format!("{value:.3}").parse().unwrap_or(value)
}
