//! Fixed SmartArt text fitting uses Word's text formatter, before canvas paint.

use super::*;

fn fit_metadata(shape: &crate::docx::InlineShape) -> Option<&crate::docx::DiagramTextAutoFit> {
  shape.diagram_text_frame.as_ref()?.auto_fit.as_ref()
}

pub(super) fn fit_children(
  children: &mut [InlineItem],
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) {
  let mut sizes = Vec::new();
  let mut groups: HashMap<String, f32> = HashMap::default();
  for (index, item) in children.iter().enumerate() {
    let InlineItem::Shape(shape) = item else {
      continue;
    };
    let Some(metadata) = fit_metadata(shape) else {
      continue;
    };
    let size = fitted_size(shape, flow, text_metrics);
    if let Some(group) = &metadata.sync_group {
      groups
        .entry(group.clone())
        .and_modify(|current| *current = (*current).min(size))
        .or_insert(size);
    }
    sizes.push((index, size));
  }
  for (index, size) in sizes {
    let InlineItem::Shape(shape) = &mut children[index] else {
      unreachable!("fitted shape index");
    };
    let metadata = fit_metadata(shape).unwrap();
    let size = metadata
      .sync_group
      .as_ref()
      .and_then(|group| groups.get(group))
      .copied()
      .unwrap_or(size);
    apply_size(shape, size);
  }
}

fn fitted_size(
  source: &crate::docx::InlineShape,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let metadata = fit_metadata(source).unwrap();
  let maximum = metadata.maximum_font_size_pt;
  let minimum = metadata.minimum_font_size_pt.max(1.0).min(maximum);
  if !maximum.is_finite() || maximum <= 0.0 {
    return maximum;
  }
  // Reuse one scratch shape; no document reload, font collection or shared
  // cache is created for the candidate sizes. The canvas already owns the
  // real children, and only its generated, automatically fitted text is tagged.
  let mut probe = source.clone();
  let mut fits = |size| {
    probe.clone_from(source);
    apply_size(&mut probe, size);
    let frame = probe.diagram_text_frame.as_ref().unwrap();
    let content = shape_text_box_content_rect(
      &probe,
      ShapeTextBoxRect {
        x: 0.0,
        y: 0.0,
        width: frame.width_pt,
        height: frame.height_pt,
      },
      textbox_content_border_inset(&probe),
    );
    if content.width <= 0.0 || content.height <= 0.0 {
      return false;
    }
    let height = shape_text_box_measured_content_height(flow, &probe, content.width, text_metrics);
    height <= content.height + LAYOUT_EPSILON_PT
      && (probe.text_box_word_wrap
        || shape_text_box_unwrapped_content_width(flow, &probe, content.width, text_metrics)
          <= content.width + LAYOUT_EPSILON_PT)
  };
  if fits(maximum) {
    return maximum;
  }
  // Native tx fitting chooses whole-point sizes below the requested upper.
  // Text height/line capacity is monotone over this bounded size range. Keep
  // the authored minimum even if it cannot fit, rather than dropping text or
  // making the physical shape grow. Each trial uses the actual Word formatter.
  let mut lower = minimum.ceil() as i32;
  let mut upper = maximum.floor() as i32;
  let mut best = minimum;
  while lower <= upper {
    let middle = lower + (upper - lower) / 2;
    if fits(middle as f32) {
      best = middle as f32;
      lower = middle + 1;
    } else {
      upper = middle - 1;
    }
  }
  best
}

fn round_font_size(style: &mut TextStyle) {
  style.font_size_pt = style.font_size_pt.round().max(1.0);
  if let Some(size) = &mut style.complex_font_size_pt {
    *size = size.round().max(1.0);
  }
}

fn apply_size(shape: &mut crate::docx::InlineShape, size: f32) {
  let metadata = shape
    .diagram_text_frame
    .as_ref()
    .and_then(|frame| frame.auto_fit.as_ref())
    .unwrap();
  let scale = size / metadata.maximum_font_size_pt;
  if !scale.is_finite() || scale <= 0.0 {
    return;
  }
  super::super::scale_drawingml_textbox_blocks(&mut shape.text_box_blocks, scale, scale, false);
  let count = shape.text_box_blocks.len();
  for (index, (block, source)) in shape
    .text_box_blocks
    .iter_mut()
    .zip(&metadata.paragraphs)
    .enumerate()
  {
    let Block::Paragraph(paragraph) = block else {
      continue;
    };
    round_font_size(&mut paragraph.base_style);
    round_font_size(&mut paragraph.list_label_style);
    for inline in &mut paragraph.inlines {
      if let InlineItem::Text(run) = inline {
        round_font_size(&mut run.style);
      }
    }
    let font_size = paragraph.base_style.font_size_pt;
    if let Some(properties) = source.properties.as_deref() {
      super::super::diagram::apply_paragraph_spacing(
        &mut paragraph.format,
        properties,
        font_size,
        index == 0,
        index + 1 == count,
      );
      // Synthesized SmartArt bullets use .9em per nesting step; an authored
      // marL/indent is a literal DrawingML distance and remains unchanged.
      if source.synthesized_bullet_left_margin {
        let levels = properties.left_margin.unwrap_or_default() as f32 / 285_750.0;
        paragraph.format.indent_left_pt = font_size * 0.9 * levels;
      }
      if source.synthesized_bullet_indent {
        paragraph.format.first_line_indent_pt = -font_size * 0.9;
      }
      paragraph.format.indent_left_pt = paragraph
        .format
        .indent_left_pt
        .max(-paragraph.format.first_line_indent_pt);
    }
  }
  for (index, inset) in [
    &mut shape.text_inset_left_pt,
    &mut shape.text_inset_top_pt,
    &mut shape.text_inset_right_pt,
    &mut shape.text_inset_bottom_pt,
  ]
  .into_iter()
  .enumerate()
  {
    if metadata.font_scaled_insets[index] {
      *inset *= scale;
    }
  }
}
