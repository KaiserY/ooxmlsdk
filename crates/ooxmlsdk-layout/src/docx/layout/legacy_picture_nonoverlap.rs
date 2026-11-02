//! Legacy Word body pictures: source-order collision placement and anchor replay.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
  left: i64,
  top: i64,
  width: i64,
  height: i64,
}

fn twips(value: f32) -> Option<i64> {
  let value = (f64::from(value) * 20.0).round();
  // Leave arithmetic headroom for two origins and extents. Nonrepresentable
  // package geometry keeps its existing positioning path.
  (value.is_finite() && value.abs() < (i64::MAX / 4) as f64).then_some(value as i64)
}

impl Rect {
  fn right(self) -> i64 {
    self.left + self.width
  }

  fn bottom(self) -> i64 {
    self.top + self.height
  }

  fn intersects(self, other: Self, minimum: i64) -> bool {
    self.right().min(other.right()) - self.left.max(other.left) >= minimum
      && self.bottom().min(other.bottom()) - self.top.max(other.top) >= minimum
  }
}

#[derive(Clone, Copy, Debug)]
struct Picture {
  inline: usize,
  placement: FloatingImagePlacement,
  original: Rect,
  resolved: Rect,
  offset: (f32, f32),
  right_limit: i64,
  bottom_limit: i64,
}

#[derive(Clone, Debug)]
pub(super) struct Plan {
  pictures: Vec<Picture>,
  page_right: i64,
  page_bottom: i64,
  replayed: bool,
}

fn eligible(image: &crate::docx::InlineImage) -> Option<FloatingImagePlacement> {
  let crate::docx::ImagePlacement::Floating(placement) = image.placement else {
    return None;
  };
  (placement.paint_order.is_drawing_ml()
    && placement.wrap == ImageWrapMode::Square
    && !placement.behind_text
    && image.rotation_deg == 0.0
    && matches!(
      placement.horizontal_relative_to,
      HorizontalImageReference::Page
        | HorizontalImageReference::Margin
        | HorizontalImageReference::Column
    )
    && matches!(
      placement.vertical_relative_to,
      VerticalImageReference::Page
        | VerticalImageReference::Margin
        | VerticalImageReference::Paragraph
    ))
  .then_some(placement)
}

pub(super) fn prepare(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  frame: TextFrame,
  position: ParagraphLayoutPosition,
) -> Option<Plan> {
  if flow.compatibility_mode >= 15
    || flow.text_segmentation != TextSegmentation::Body
    || flow.inside_paragraph_frame
  {
    return None;
  }
  let mut depth = 0usize;
  let mut count = 0;
  let mut forbids_overlap = false;
  for inline in &paragraph.inlines {
    match inline {
      InlineItem::DrawingGroupStart(_) => depth += 1,
      InlineItem::DrawingGroupEnd => depth = depth.saturating_sub(1),
      InlineItem::Image(image) if depth == 0 => {
        if let Some(placement) = eligible(image) {
          count += 1;
          forbids_overlap |= !placement.allow_overlap;
        }
      }
      // A later page/column's anchors have a different physical owner.
      InlineItem::PageBreak | InlineItem::ColumnBreak => break,
      _ => {}
    }
  }
  if count < 2 || !forbids_overlap {
    return None;
  }
  let page_right = twips(flow.setup.width_pt)?;
  let page_bottom = twips(flow.setup.height_pt)?;
  let mut pictures = Vec::with_capacity(count);
  depth = 0;
  for (inline, item) in paragraph.inlines.iter().enumerate() {
    let image = match item {
      InlineItem::DrawingGroupStart(_) => {
        depth += 1;
        continue;
      }
      InlineItem::DrawingGroupEnd => {
        depth = depth.saturating_sub(1);
        continue;
      }
      InlineItem::PageBreak | InlineItem::ColumnBreak => break,
      InlineItem::Image(image) if depth == 0 => image,
      _ => continue,
    };
    let Some(placement) = eligible(image) else {
      continue;
    };
    let metrics = floating_image_metrics(image, placement, flow);
    let anchor_y = floating_anchor_reference_y(placement, position.anchor_top, position.y);
    let (left, top) = floating_picture_position(
      placement,
      flow,
      frame.paragraph_indents(),
      (frame.first_line_left, anchor_y),
      (metrics.frame_width_pt, metrics.frame_height_pt),
      false,
    );
    let original = Rect {
      left: twips(left)?,
      top: twips(top)?,
      width: twips(metrics.frame_width_pt)?,
      height: twips(metrics.frame_height_pt)?,
    };
    if original.width <= 0 || original.height <= 0 {
      continue;
    }
    let right_limit = match placement.horizontal_relative_to {
      HorizontalImageReference::Page => page_right,
      HorizontalImageReference::Margin => twips(flow.setup.width_pt - flow.setup.margin_right_pt)?,
      _ => {
        let (left, width) =
          floating_column_reference_bounds(placement, flow, frame.paragraph_indents());
        twips(left + width)?
      }
    };
    let bottom_limit = if placement.vertical_relative_to == VerticalImageReference::Page {
      page_bottom
    } else {
      twips(flow.body_content_bottom_pt)?
    };
    pictures.push(Picture {
      inline,
      placement,
      original,
      resolved: original,
      offset: (metrics.content_offset_x_pt, metrics.content_offset_y_pt),
      right_limit,
      bottom_limit,
    });
  }
  let mut plan = Plan {
    pictures,
    page_right,
    page_bottom,
    replayed: false,
  };
  // ECMA-376 wp:anchor/allowOverlap applies when either participant forbids
  // overlap. Native source-order controls distinguish this from paint order.
  for index in 0..plan.pictures.len() {
    let (earlier, later) = plan.pictures.split_at_mut(index);
    settle(&mut later[0], earlier, page_bottom, 10);
  }
  Some(plan)
}

// Native 20/40/80/120-height controls expose repeated (own-height - one-twip)
// steps, capped by the obstacle bottom. The CURRENT frame must fit before the
// next step; page327.60/327.65 is an exact opposite. Jump over identical steps
// arithmetically: a1pt object requires105steps in the equivalent recurrence.
fn move_down(current: &mut Picture, obstacle: Rect, page_bottom: i64, minimum: i64) {
  let rect = current.resolved;
  let step = rect.height - 1;
  if step <= 0 || rect.bottom() > current.bottom_limit {
    return;
  }
  let distance = obstacle.bottom() - minimum + 1 - rect.top;
  if distance <= 0 {
    return;
  }
  let clear_steps = (distance - 1) / step + 1;
  let admitted_steps = (current.bottom_limit - rect.bottom()) / step + 1;
  let steps = clear_steps.min(admitted_steps);
  let top = (rect.top + steps * step)
    .min(obstacle.bottom() - 1)
    .min(page_bottom - rect.height);
  if top > rect.top {
    current.resolved.top = top;
  }
}

fn settle(current: &mut Picture, earlier: &[Picture], page_bottom: i64, minimum: i64) {
  loop {
    let before = current.resolved;
    for previous in earlier {
      if (current.placement.allow_overlap && previous.placement.allow_overlap)
        || !current.resolved.intersects(previous.resolved, minimum)
      {
        continue;
      }
      let right = previous.resolved.right() - 1;
      if right + current.resolved.width <= current.right_limit {
        current.resolved.left = right;
      } else {
        move_down(current, previous.resolved, page_bottom, minimum);
      }
    }
    if current.resolved == before {
      break;
    }
  }
}

impl Plan {
  pub(super) fn position(&self, inline: usize) -> Option<(f32, f32)> {
    let picture = self
      .pictures
      .iter()
      .find(|picture| picture.inline == inline)?;
    Some((
      picture.resolved.left as f32 / 20.0 + picture.offset.0,
      picture.resolved.top as f32 / 20.0 + picture.offset.1,
    ))
  }

  pub(super) fn after_paragraph(&self, fragments: &[FrameFragment]) -> Option<Self> {
    if self.replayed {
      return None;
    }
    let bottom = fragments
      .iter()
      .filter(|fragment| fragment.kind == FrameFragmentKind::ParagraphLine)
      .filter_map(|fragment| {
        // The painted bounds include anchored pictures. Native short-anchor
        // controls and GDB distinguish their183.65pt ink box from the actual
        // 13.8pt text advance. Empty marks retain their explicit line bounds.
        let top = fragment
          .flow_top_pt
          .or_else(|| fragment.bounds.map(|bounds| bounds.y_pt))?;
        let height = fragment
          .content_advance_pt
          .or_else(|| fragment.bounds.map(|bounds| bounds.height_pt))?;
        twips(top + height)
      })
      .max()?;
    self.after_contact(bottom)
  }

  fn after_contact(&self, bottom: i64) -> Option<Self> {
    let mut replay = self.clone();
    replay.replayed = true;
    for index in 0..replay.pictures.len() {
      let (earlier, later) = replay.pictures.split_at_mut(index);
      let current = &mut later[0];
      if !earlier.iter().any(|previous| {
        bottom > previous.original.top
          && (!current.placement.allow_overlap || !previous.placement.allow_overlap)
          && current.original.intersects(previous.original, 2)
      }) {
        continue;
      }
      // Native exact14.20/14.25 and offset30/line28/32 controls separate
      // actual anchor-frame contact from text length or page overflow. A
      // paragraph-relative displacement persists; Page/Margin restart from
      // their authored base. The paragraph's later sibling does not replay it.
      current.right_limit = replay.page_right;
      if current.placement.vertical_relative_to != VerticalImageReference::Paragraph {
        current.resolved.top = current.original.top;
      }
      // The completed paragraph resolves residual intersections beyond the
      // shared one-twip edge. Equal20pt frames with .15pt effect-bottom move
      // right here, while the fully cleared downward frame retains its X.
      settle(current, earlier, replay.page_bottom, 2);
    }
    self
      .pictures
      .iter()
      .zip(&replay.pictures)
      .any(|(before, after)| before.resolved != after.resolved)
      .then_some(replay)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn picture(left: f32, top: f32, width: f32, height: f32) -> Picture {
    let rect = Rect {
      left: twips(left).unwrap(),
      top: twips(top).unwrap(),
      width: twips(width).unwrap(),
      height: twips(height).unwrap(),
    };
    Picture {
      inline: 0,
      placement: FloatingImagePlacement {
        allow_overlap: false,
        vertical_relative_to: VerticalImageReference::Paragraph,
        horizontal_relative_to: HorizontalImageReference::Column,
        horizontal_alignment: None,
        vertical_alignment: None,
        alignment_extent: None,
        group_child_offset_x_pt: 0.0,
        group_child_offset_y_pt: 0.0,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        horizontal_offset_pct: None,
        vertical_offset_pct: None,
        wrap: ImageWrapMode::Square,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: false,
        layout_in_cell: true,
        layout_in_cell_forced: false,
        paint_order: FloatingPaintOrder::drawing_ml_relative_height(1),
        relative_width_to: None,
        relative_width_pct: None,
        relative_height_to: None,
        relative_height_pct: None,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      },
      original: rect,
      resolved: rect,
      offset: (0.0, 0.0),
      right_limit: 5512,
      bottom_limit: 4514,
    }
  }

  #[test]
  fn native_repeated_steps_and_current_frame_admission() {
    for (small_height, large_height, page_height, expected) in [
      (20.15, 20.0, 297.7, 106.15),
      (40.15, 20.0, 297.7, 126.10),
      (80.15, 20.0, 297.7, 166.00),
      (120.15, 20.0, 297.7, 205.90),
      (120.15, 100.0, 297.7, 186.15),
      (120.15, 100.0, 497.7, 206.30),
      (94.5, 84.75, 327.60, 170.90),
      (94.5, 84.75, 327.65, 180.65),
      (100.15, 1.0, 297.7, 185.95),
    ] {
      let previous = picture(178.2, 86.2, 63.2, small_height);
      let mut current = picture(153.4, 86.2, 112.8, large_height);
      current.bottom_limit = twips(page_height - 72.0).unwrap();
      settle(&mut current, &[previous], twips(page_height).unwrap(), 10);
      assert_eq!(current.resolved.top, twips(expected).unwrap());
    }
  }

  #[test]
  fn native_half_point_intersection_includes_the_boundary() {
    for (small_height, expected) in [(20.4, 106.15), (20.45, 106.60), (20.5, 106.65)] {
      let previous = picture(178.2, 86.2, 63.2, small_height);
      let mut current = picture(153.4, 86.2, 112.8, 20.0);
      settle(&mut current, &[previous], 5954, 10);
      assert_eq!(current.resolved.top, twips(expected).unwrap());
    }
    for height in [0.1, 0.25] {
      let previous = picture(178.2, 86.2, 63.2, 1.15);
      let mut current = picture(153.4, 86.2, 112.8, height);
      settle(&mut current, &[previous], 5954, 10);
      assert_eq!(current.resolved, current.original);
    }
  }

  #[test]
  fn native_anchor_contact_preserves_only_paragraph_relative_displacement() {
    for reference in [
      VerticalImageReference::Paragraph,
      VerticalImageReference::Margin,
      VerticalImageReference::Page,
    ] {
      let previous = picture(178.2, 86.2, 63.2, 94.5);
      let mut current = picture(153.4, 86.2, 112.8, 84.75);
      current.inline = 1;
      current.placement.vertical_relative_to = reference;
      if reference == VerticalImageReference::Page {
        current.bottom_limit = 5954;
      }
      settle(&mut current, &[previous], 5954, 10);
      let plan = Plan {
        pictures: vec![previous, current],
        page_right: 8392,
        page_bottom: 5954,
        replayed: false,
      };
      assert!(plan.after_contact(1724).is_none());
      let replay = plan
        .after_contact(1725)
        .expect("exact14.25 crosses the original14.20 band");
      assert_eq!(
        replay.position(1),
        Some((
          241.35,
          if reference == VerticalImageReference::Paragraph {
            170.9
          } else {
            86.2
          }
        ))
      );
    }
    let previous = picture(178.2, 44.2, 63.2, 94.5);
    let mut current = picture(153.4, 44.2, 112.8, 84.75);
    settle(&mut current, &[previous], 5954, 10);
    assert_eq!(current.resolved.top, twips(138.65).unwrap());
    let plan = Plan {
      pictures: vec![previous, current],
      page_right: 8392,
      page_bottom: 5954,
      replayed: false,
    };
    assert!(
      plan.after_contact(4514).is_none(),
      "fully cleared paragraph-relative frame retains its X"
    );
  }

  #[test]
  fn anchor_contact_uses_text_advance_instead_of_the_floating_picture_ink_box() {
    let previous = picture(178.2, 86.2, 63.2, 94.5);
    let mut current = picture(153.4, 86.2, 112.8, 84.75);
    current.inline = 1;
    settle(&mut current, &[previous], 5954, 10);
    let plan = Plan {
      pictures: vec![previous, current],
      page_right: 8392,
      page_bottom: 5954,
      replayed: false,
    };
    let mut fragment = FrameFragment {
      kind: FrameFragmentKind::ParagraphLine,
      split: FragmentSplitKind::Complete,
      index: 0,
      row_index: 0,
      cell_index: None,
      item_start: 0,
      item_end: 3,
      bounds: Some(FrameBounds {
        x_pt: 144.0,
        y_pt: 72.0,
        width_pt: 122.2,
        height_pt: 183.65,
      }),
      table_cell_non_content_vertical_advance_pt: 0.0,
      table_cell_wrap_vertical_advance_pt: 0.0,
      content_advance_pt: Some(14.2),
      flow_top_pt: Some(72.0),
      paragraph_spacing_after_pt: 0.0,
    };
    assert!(
      plan
        .after_paragraph(std::slice::from_ref(&fragment))
        .is_none()
    );
    fragment.content_advance_pt = Some(14.25);
    assert_eq!(
      plan.after_paragraph(&[fragment]).unwrap().position(1),
      Some((241.35, 170.9))
    );
  }

  #[test]
  fn permission_and_horizontal_capture_are_independent() {
    let mut previous = picture(178.2, 86.2, 63.2, 94.5);
    let mut current = picture(153.4, 86.2, 112.8, 84.75);
    current.right_limit = 8392;
    settle(&mut current, &[previous], 5954, 10);
    assert_eq!(current.resolved.left, 4827);
    assert_eq!(current.resolved.top, 1724);
    previous.placement.allow_overlap = true;
    current.placement.allow_overlap = true;
    current.resolved = current.original;
    settle(&mut current, &[previous], 5954, 10);
    assert_eq!(current.resolved, current.original);
  }
}
