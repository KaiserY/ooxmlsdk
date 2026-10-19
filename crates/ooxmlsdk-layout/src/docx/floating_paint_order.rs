//! Resolve the common paint domain of legacy and modern floating anchors.
//!
//! VML's signed CSS z-index is not a DrawingML relativeHeight. Word first
//! resolves the deferred VML indices for the whole story, then repairs missing
//! or colliding native indices before sorting both producers together. Doing
//! this after pagination loses objects anchored on other pages.

use super::model::{
  Block, FloatingPaintOrder, FloatingPaintOrderSource, ImagePlacement, ImageWrapMode,
  ImportedSection, InlineItem,
};

const NATIVE_MIDDLE: u32 = 0x0f00_0000;
const NATIVE_LIMIT: u32 = 0x1e00_0000;
const NATIVE_STEP: u32 = 1024;

pub(super) fn normalize_sections(sections: &mut [ImportedSection]) {
  normalize_story(sections.iter_mut().flat_map(|section| &mut section.blocks));
  for section in sections {
    for story in [
      &mut section.header_blocks,
      &mut section.footer_blocks,
      &mut section.first_header_blocks,
      &mut section.first_footer_blocks,
      &mut section.even_header_blocks,
      &mut section.even_footer_blocks,
    ] {
      normalize_story(story);
    }
  }
}

pub(super) fn normalize_story<'a>(blocks: impl IntoIterator<Item = &'a mut Block>) {
  let mut orders = Vec::new();
  collect_story_orders(blocks, &mut orders);
  normalize_orders(&mut orders);
}

fn collect_story_orders<'a>(
  blocks: impl IntoIterator<Item = &'a mut Block>,
  orders: &mut Vec<&'a mut FloatingPaintOrder>,
) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => {
        collect_inline_orders(&mut paragraph.inlines, orders);
      }
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            collect_story_orders(&mut cell.blocks, orders);
          }
        }
      }
      // framePr relocates paragraphs of this story, rather than creating a
      // separate textbox story.
      Block::Frame(frame) => {
        collect_story_orders(&mut frame.blocks, orders);
      }
    }
  }
}

fn collect_inline_orders<'a>(
  inlines: &'a mut [InlineItem],
  orders: &mut Vec<&'a mut FloatingPaintOrder>,
) {
  let mut group_depth = 0usize;
  for inline in inlines {
    let mut group_root = false;
    let placement = match inline {
      InlineItem::Image(image) => &mut image.placement,
      InlineItem::Shape(shape) => {
        normalize_story(&mut shape.text_box_blocks);
        if let Some(children) = &mut shape.canvas_children {
          // Canvas children belong to one atomic anchor; they do not add
          // independently floating objects to the surrounding story.
          let mut child_orders = Vec::new();
          collect_inline_orders(children, &mut child_orders);
          normalize_orders(&mut child_orders);
        }
        &mut shape.placement
      }
      InlineItem::DrawingGroupStart(group) => {
        let outer = group_depth == 0;
        group_depth += 1;
        if !outer {
          continue;
        }
        group_root = true;
        &mut group.placement
      }
      InlineItem::DrawingGroupEnd => {
        group_depth = group_depth.saturating_sub(1);
        continue;
      }
      _ => continue,
    };
    if let ImagePlacement::Floating(placement) = placement
      && placement.wrap != ImageWrapMode::Inline
      && (group_depth == 0 || group_root)
    {
      orders.push(&mut placement.paint_order);
    }
  }
}

fn normalize_orders(orders: &mut [&mut FloatingPaintOrder]) {
  let mut has_vml = false;
  let mut has_drawing_ml = false;
  let mut authored_vml_count = 0usize;
  for order in orders.iter() {
    match order.source {
      FloatingPaintOrderSource::VmlZIndex(value) => {
        has_vml = true;
        authored_vml_count += usize::from(value.is_some());
      }
      FloatingPaintOrderSource::DrawingMlRelativeHeight(_) => has_drawing_ml = true,
      FloatingPaintOrderSource::Unspecified => {}
    }
  }
  // Homogeneous stories retain their existing ordering, including unauthored
  // VML indices. No key/index allocations are needed on that path.
  if !has_vml || !has_drawing_ml {
    return;
  }

  // Word's deferred VML property pass skips a list with at most one authored
  // entry. Missing z-index is absent from that list, whereas explicit 0/1
  // participates. Positive native indices 0/1 themselves remain unset.
  // Native Word observations: MSO 26ec0/27197/276e6; native property938.
  // An empty import allocates its first default at the midpoint. Once VML is
  // present its native drawing collection reserves that midpoint, even while
  // negative source indices are still deferred. Authored positive native keys
  // and earlier modern keys can raise this high-water mark independently.
  let mut imported_highest = NATIVE_MIDDLE - NATIVE_STEP;
  let mut keys: Vec<u32> = orders
    .iter()
    .map(|order| match order.source {
      FloatingPaintOrderSource::VmlZIndex(value) => {
        imported_highest = imported_highest
          .max(NATIVE_MIDDLE)
          .max(vml_native_key(value, false));
        vml_native_key(value, authored_vml_count > 1)
      }
      FloatingPaintOrderSource::DrawingMlRelativeHeight(value) => {
        // Default allocation sees already imported native keys, before the
        // later signed-VML resolution. Native53..55 distinguish a first
        // modern0 (midpoint), a prior VML collection (+1024), and a larger
        // positive VML key. Merely counting objects gives the wrong answer.
        let key = if (2..NATIVE_LIMIT).contains(&value) {
          value
        } else {
          imported_highest
            .saturating_add(NATIVE_STEP)
            .min(NATIVE_LIMIT - 1)
        };
        imported_highest = imported_highest.max(key);
        key
      }
      _ => 0,
    })
    .collect();
  let mut indices: Vec<usize> = (0..orders.len())
    .filter(|&index| orders[index].source != FloatingPaintOrderSource::Unspecified)
    .collect();
  indices.sort_by_key(|&index| keys[index]);
  let duplicates = indices
    .windows(2)
    .any(|pair| keys[pair[0]] == keys[pair[1]]);
  let needs_repair = duplicates
    || indices.iter().any(|&index| match orders[index].source {
      FloatingPaintOrderSource::VmlZIndex(_) => {
        keys[index] < 2 || keys[index] >= NATIVE_LIMIT - NATIVE_STEP
      }
      FloatingPaintOrderSource::DrawingMlRelativeHeight(_) => {
        !(2..NATIVE_LIMIT).contains(&keys[index])
      }
      FloatingPaintOrderSource::Unspecified => false,
    });

  if needs_repair {
    // Word repairs the native VML collection in its current z-order, above
    // the highest occupied key. Near the domain limit there is no append
    // space: retaining the old keys here is essential. It then merges both
    // collections and renumbers in1024steps around the midpoint. That final
    // monotone renumbering is unnecessary for paint-only comparison.
    // Native Word observations: f3b7f8 validation,59bb40 highest occupied,
    // 17a3b00 allocation and557a60 common ordering.
    let mut next = indices
      .iter()
      .map(|&index| keys[index].saturating_add(NATIVE_STEP))
      .fold(NATIVE_MIDDLE, u32::max)
      .min(NATIVE_LIMIT - 1);
    for &index in &indices {
      if next >= NATIVE_LIMIT - NATIVE_STEP {
        break;
      }
      if orders[index].is_vml() {
        keys[index] = next;
        next += NATIVE_STEP;
      }
    }
  }

  for (order, key) in orders.iter_mut().zip(keys) {
    if order.source != FloatingPaintOrderSource::Unspecified {
      order.normalized_key = Some(key);
    }
  }
}

fn vml_native_key(value: Option<i32>, resolve_negative: bool) -> u32 {
  let value = match value {
    Some(value) if value < 0 && resolve_negative => NATIVE_LIMIT.wrapping_add_signed(value),
    Some(value) if value > 1 => value as u32,
    _ => 0,
  };
  // Native import leaves out-of-domain positive938 assignments unset. The
  // largest signed VML values remain unset; treating them as huge native keys
  // also corrupts the defaults subsequently allocated to modern anchors.
  if (2..NATIVE_LIMIT).contains(&value) {
    value
  } else {
    0
  }
}

pub(super) fn drawing_ml_key(value: u32, compatibility_mode: u16) -> u32 {
  if compatibility_mode < 15 && !(2..NATIVE_LIMIT).contains(&value) {
    NATIVE_LIMIT - 1
  } else {
    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn paint_order(vml: &[Option<i32>], drawing: u32) -> Vec<usize> {
    let mut orders: Vec<_> = vml
      .iter()
      .copied()
      .map(FloatingPaintOrder::vml_z_index)
      .chain([FloatingPaintOrder::drawing_ml_relative_height(drawing)])
      .collect();
    let sources: Vec<_> = orders.iter().map(|order| order.source).collect();
    normalize_orders(&mut orders.iter_mut().collect::<Vec<_>>());
    assert!(orders.iter().all(|order| order.normalized_key.is_some()));
    assert_eq!(
      sources,
      orders.iter().map(|order| order.source).collect::<Vec<_>>()
    );
    let mut indices: Vec<_> = (0..orders.len()).collect();
    indices.sort_by_key(|&i| (orders[i].normalized_key, orders[i].is_drawing_ml()));
    indices
  }

  #[test]
  fn mixed_order_matches_native_opposites_instead_of_producer_buckets() {
    let t = NATIVE_MIDDLE as i32;
    assert_eq!(paint_order(&[Some(-t)], t as u32), [1, 0]);
    assert_eq!(paint_order(&[Some(-t)], NATIVE_LIMIT - 2), [0, 1]);
    assert_eq!(paint_order(&[Some(-t), Some(-998)], t as u32), [0, 2, 1]);
    assert_eq!(
      paint_order(&[Some(-t), Some(-998)], (t - 1024) as u32),
      [2, 0, 1]
    );
    assert_eq!(
      paint_order(&[Some(-t + 1024), Some(-998)], t as u32),
      [2, 0, 1]
    );
    assert_eq!(
      paint_order(&[Some(-t), Some(-t - 1024)], t as u32),
      [2, 1, 0]
    );
    assert_eq!(paint_order(&[Some(-t), Some(-t)], t as u32), [2, 0, 1]);
    assert_eq!(paint_order(&[Some(-t), Some(998)], t as u32), [2, 1, 0]);
    assert_eq!(
      paint_order(&[Some(t + 1024), Some(998)], t as u32),
      [1, 2, 0]
    );
  }

  #[test]
  fn deferred_vml_count_distinguishes_absent_zero_and_one() {
    let t = NATIVE_MIDDLE as i32;
    assert_eq!(paint_order(&[Some(-t), None], t as u32), [2, 0, 1]);
    for extra in [0, 1, 2] {
      assert_eq!(paint_order(&[Some(-t), Some(extra)], t as u32), [2, 1, 0]);
    }
  }

  #[test]
  fn invalid_modern_keys_follow_the_prior_modern_import_allocation() {
    // Actual pre-sort native values from independent Native48/50 controls.
    // The high legacy key prevents the later append pass from obscuring the
    // modern import defaults. Both pictures and WPS shapes share this rule.
    let t = NATIVE_MIDDLE;
    for (raw, expected) in [
      ([0, 0], [t + 1024, t + 2048]),
      ([0, t + 512], [t + 1024, t + 512]),
      ([t + 512, 0], [t + 512, t + 1536]),
      ([1, u32::MAX], [t + 1024, t + 2048]),
      ([2, 0], [2, t + 1024]),
    ] {
      let mut orders = [
        FloatingPaintOrder::vml_z_index(Some(-(t as i32))),
        FloatingPaintOrder::drawing_ml_relative_height(raw[0]),
        FloatingPaintOrder::drawing_ml_relative_height(raw[1]),
        FloatingPaintOrder::vml_z_index(Some(-998)),
      ];
      normalize_orders(&mut orders.iter_mut().collect::<Vec<_>>());
      assert_eq!(orders[0].normalized_key, Some(t));
      assert_eq!(orders[1].normalized_key, Some(expected[0]), "{raw:?}");
      assert_eq!(orders[2].normalized_key, Some(expected[1]), "{raw:?}");
      assert_eq!(orders[3].normalized_key, Some(NATIVE_LIMIT - 998));
    }
  }

  #[test]
  fn first_modern_default_and_prior_legacy_collection_have_different_owners() {
    let t = NATIVE_MIDDLE;
    let mut modern_first = [
      FloatingPaintOrder::drawing_ml_relative_height(0),
      FloatingPaintOrder::drawing_ml_relative_height(t),
      FloatingPaintOrder::vml_z_index(Some(0)),
    ];
    normalize_orders(&mut modern_first.iter_mut().collect::<Vec<_>>());
    assert_eq!(
      modern_first.map(|order| order.normalized_key),
      [Some(t), Some(t), Some(t + 1024)]
    );

    // Native54/55 show that valid positive VML keys participate before modern
    // defaults, while deferred negatives and invalid signed maxima do not.
    for (raw, expected_legacy, expected_modern) in [
      (-(t as i32), t, t + 1024),
      (0, 0, t + 1024),
      (998, 998, t + 1024),
      (t as i32 + 512, t + 512, t + 1536),
      (i32::MAX, 0, t + 1024),
      (NATIVE_LIMIT as i32 - 1, NATIVE_LIMIT - 1, NATIVE_LIMIT - 1),
    ] {
      let mut orders = [
        FloatingPaintOrder::vml_z_index(Some(raw)),
        FloatingPaintOrder::drawing_ml_relative_height(0),
        FloatingPaintOrder::vml_z_index(Some(-998)),
      ];
      normalize_orders(&mut orders.iter_mut().collect::<Vec<_>>());
      assert_eq!(orders[0].normalized_key, Some(expected_legacy), "{raw}");
      assert_eq!(orders[1].normalized_key, Some(expected_modern), "{raw}");
    }
  }

  #[test]
  fn a_prior_low_modern_key_does_not_reserve_the_legacy_midpoint() {
    let t = NATIVE_MIDDLE;
    for first in [2, t - 2048] {
      let mut orders = [
        FloatingPaintOrder::drawing_ml_relative_height(first),
        FloatingPaintOrder::drawing_ml_relative_height(0),
        FloatingPaintOrder::vml_z_index(Some(-(t as i32))),
        FloatingPaintOrder::vml_z_index(Some(-998)),
      ];
      normalize_orders(&mut orders.iter_mut().collect::<Vec<_>>());
      assert_eq!(orders[0].normalized_key, Some(first));
      assert_eq!(orders[1].normalized_key, Some(t));
    }
  }

  #[test]
  fn homogeneous_stories_keep_authored_order_metadata() {
    for mut orders in [
      vec![
        FloatingPaintOrder::vml_z_index(None),
        FloatingPaintOrder::vml_z_index(Some(-1)),
      ],
      vec![
        FloatingPaintOrder::drawing_ml_relative_height(0),
        FloatingPaintOrder::drawing_ml_relative_height(4),
      ],
    ] {
      let before = orders.clone();
      normalize_orders(&mut orders.iter_mut().collect::<Vec<_>>());
      assert_eq!(orders, before);
    }
  }
}
