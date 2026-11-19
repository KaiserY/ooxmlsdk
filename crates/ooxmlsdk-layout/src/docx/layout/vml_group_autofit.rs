//! VML keeps the group's local viewport separate from its page-sized host.
//!
//! Word's native AutoSize route (a38c50 -> MSO 148c7f8/7c3bc) fits physical
//! text, inversely maps the changed rectangle, then unions the authored child
//! rectangles (5efb30). The host frame is cached until the layout transaction
//! finishes when relative sizing is active. Absolute-only groups expose each
//! committed host immediately. A subsequent transaction resolves dimensions
//! again, after both measurement and positioned layout have visited the children.
//! The local Writer counterparts are SdrObjGroup::NbcSetSnapRect/NbcResize and
//! SdrTextObj::NbcAdjustTextFrameWidthAndHeight; they likewise keep these two
//! coordinate spaces distinct. Native measurements establish source ordering
//! and Word's strict eight-twip comparison, which are not specified by VML.

use super::*;
use crate::docx::{FloatingAlignmentExtent, ImagePlacement, InlineVmlGroupViewport, Paragraph};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
  left: f64,
  top: f64,
  right: f64,
  bottom: f64,
}

impl Rect {
  fn from_coordinates([left, top, right, bottom]: [f64; 4]) -> Self {
    Self {
      left,
      top,
      right,
      bottom,
    }
  }

  fn width(self) -> f64 {
    self.right - self.left
  }
  fn height(self) -> f64 {
    self.bottom - self.top
  }

  fn union(self, other: Self) -> Self {
    Self {
      left: self.left.min(other.left),
      top: self.top.min(other.top),
      right: self.right.max(other.right),
      bottom: self.bottom.max(other.bottom),
    }
  }

  fn map(self, viewport: Self, host: [f64; 2]) -> Self {
    Self {
      left: ((self.left - viewport.left) * host[0] / viewport.width()).round(),
      top: ((self.top - viewport.top) * host[1] / viewport.height()).round(),
      right: ((self.right - viewport.left) * host[0] / viewport.width()).round(),
      bottom: ((self.bottom - viewport.top) * host[1] / viewport.height()).round(),
    }
  }

  fn unmap(self, viewport: Self, host: [f64; 2]) -> Self {
    Self {
      left: viewport.left + (self.left * viewport.width() / host[0]).round(),
      top: viewport.top + (self.top * viewport.height() / host[1]).round(),
      right: viewport.left + (self.right * viewport.width() / host[0]).round(),
      bottom: viewport.top + (self.bottom * viewport.height() / host[1]).round(),
    }
  }

  fn native_equal(self, other: Self) -> bool {
    // Word 10a2068: every edge difference must be strictly below eight twips.
    // This is the shape layout contract, independent of PDF acceptance gates.
    (self.left - other.left).abs() < 8.0
      && (self.top - other.top).abs() < 8.0
      && (self.right - other.right).abs() < 8.0
      && (self.bottom - other.bottom).abs() < 8.0
  }
}

struct Resolved {
  viewport: Rect,
  children: Vec<Rect>,
  host: [f64; 2],
  offset: [f64; 2],
}

#[derive(Clone, Copy)]
enum LayoutPhase {
  Measure,
  Position,
}

fn resolve(
  model: &InlineVmlGroupViewport,
  relative: [Option<f64>; 2],
  mut fit: impl FnMut(usize, [f64; 2]) -> Option<[f64; 2]>,
) -> Option<Resolved> {
  let mut viewport = Rect::from_coordinates(model.coordinates);
  let mut children: Vec<_> = model
    .children
    .iter()
    .map(|child| Rect::from_coordinates(child.coordinates))
    .collect();
  let mut authored = [
    f64::from(model.width_pt) * 20.0,
    f64::from(model.height_pt) * 20.0,
  ]
  .map(f64::round);
  let mut offset = [0.0; 2];
  let mut budget = 256usize;
  let cache_host = relative.iter().any(Option::is_some);
  loop {
    let mut host = [
      relative[0].unwrap_or(authored[0]),
      relative[1].unwrap_or(authored[1]),
    ];
    if host.iter().any(|value| !value.is_finite() || *value <= 0.0) {
      return None;
    }
    let mut changed = false;
    // Actual Word callers are measurement (133705c/d4d978) and positioned
    // layout (bd0d40/bd0b70). Visit once per phase, then rebuild an invalidated
    // host. Converging the cached host before positioning loses source-order
    // behavior: native reversed-order/H ends at 12733, not 12934 twips.
    for _phase in [LayoutPhase::Measure, LayoutPhase::Position] {
      for index in 0..children.len() {
        let page = children[index].map(viewport, host);
        let Some([width, height]) = fit(index, [page.width(), page.height()]) else {
          continue;
        };
        if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
          return None;
        }
        let desired = Rect {
          right: page.left + width.round(),
          bottom: page.top + height.round(),
          ..page
        };
        if page.native_equal(desired) {
          continue;
        }
        // Bound malformed/nonconvergent drawings, not the ordinary number of
        // layout passes. A valid transaction terminates on actual edge equality.
        budget = budget.checked_sub(1)?;
        let updated = desired.unmap(viewport, host);
        if updated == children[index] {
          continue;
        }
        children[index] = updated;
        changed = true;
        let union = children.iter().copied().reduce(Rect::union)?;
        if union != viewport {
          let committed = union.map(viewport, host);
          authored = [committed.width(), committed.height()];
          offset[0] += committed.left;
          offset[1] += committed.top;
          viewport = union;
          if !cache_host {
            host = authored;
          }
        }
      }
    }
    let next_host = [
      relative[0].unwrap_or(authored[0]),
      relative[1].unwrap_or(authored[1]),
    ];
    if !changed && host == next_host {
      return Some(Resolved {
        viewport,
        children,
        host,
        offset,
      });
    }
    budget = budget.checked_sub(1)?;
  }
}

pub(super) fn prepare<'a>(
  paragraph: &'a Paragraph,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> Cow<'a, Paragraph> {
  let mut prepared = Cow::Borrowed(paragraph);
  for (start, item) in paragraph.inlines.iter().enumerate() {
    let InlineItem::DrawingGroupStart(group) = item else {
      continue;
    };
    let Some(model) = group.vml_viewport.as_deref() else {
      continue;
    };
    let ImagePlacement::Floating(placement) = group.placement else {
      continue;
    };
    let Some(end) = paragraph.inlines[start + 1..]
      .iter()
      .position(|item| matches!(item, InlineItem::DrawingGroupEnd))
      .map(|index| start + 1 + index)
    else {
      continue;
    };
    let children = &paragraph.inlines[start + 1..end];
    let fitting: Vec<_> = model
      .children
      .iter()
      .map(|child| {
        children.iter().find_map(|item| match item {
          InlineItem::Shape(shape)
            if shape.vml_shape_id.as_deref() == Some(child.id.as_ref())
              && !shape.text_box_blocks.is_empty() =>
          {
            Some(shape)
          }
          _ => None,
        })
      })
      .collect();
    let relative = [
      placement
        .relative_width_to
        .zip(placement.relative_width_pct)
        .map(|(reference, pct)| {
          f64::from(horizontal_reference_width(reference, flow) * pct) * 20.0
        }),
      placement
        .relative_height_to
        .zip(placement.relative_height_pct)
        .map(|(reference, pct)| f64::from(vertical_reference_height(reference, flow) * pct) * 20.0),
    ]
    .map(|value| value.map(f64::round));
    let Some(resolved) = resolve(model, relative, |index, [width, height]| {
      let shape = fitting[index]?;
      shape_text_box_auto_fit_size(
        flow,
        shape,
        (width / 20.0) as f32,
        (height / 20.0) as f32,
        text_metrics,
      )
      .map(|size| {
        [
          f64::from(size.width_pt) * 20.0,
          f64::from(size.height_pt) * 20.0,
        ]
      })
    }) else {
      continue;
    };
    let extent = FloatingAlignmentExtent {
      vml_child_resize: Default::default(),
      width_pt: (resolved.host[0] / 20.0) as f32,
      height_pt: (resolved.host[1] / 20.0) as f32,
      vml_auto_fit_resolved: true,
      wordprocessing_effect_extent_pt: None,
      vml_path_effect_extent_pt: None,
      relative_width_to: None,
      relative_width_pct: None,
      relative_height_to: None,
      relative_height_pct: None,
    };
    let mut root = placement;
    root.alignment_extent = Some(extent);
    root.relative_width_to = None;
    root.relative_width_pct = None;
    root.relative_height_to = None;
    root.relative_height_pct = None;
    if root.horizontal_alignment.is_none() {
      root.horizontal_offset_pt += (resolved.offset[0] / 20.0) as f32;
    }
    if root.vertical_alignment.is_none() {
      root.vertical_offset_pt += (resolved.offset[1] / 20.0) as f32;
    }
    let target = prepared.to_mut();
    if let InlineItem::DrawingGroupStart(group) = &mut target.inlines[start] {
      group.placement = ImagePlacement::Floating(root);
    }
    for item in &mut target.inlines[start + 1..end] {
      let InlineItem::Shape(shape) = item else {
        continue;
      };
      let mut child_placement = root;
      if let ImagePlacement::Floating(previous) = shape.placement {
        child_placement.paint_order = previous.paint_order;
      }
      if let Some(index) = shape.vml_shape_id.as_deref().and_then(|id| {
        model
          .children
          .iter()
          .position(|child| child.id.as_ref() == id)
      }) {
        let rect = resolved.children[index].map(resolved.viewport, resolved.host);
        shape.width_pt = (rect.width() / 20.0) as f32;
        shape.height_pt = (rect.height() / 20.0) as f32;
        child_placement.group_child_offset_x_pt = (rect.left / 20.0) as f32;
        child_placement.group_child_offset_y_pt = (rect.top / 20.0) as f32;
      } else {
        // Import established that the only item without a child identity is
        // the trailing unpainted root flow frame.
        shape.width_pt = extent.width_pt;
        shape.height_pt = extent.height_pt;
      }
      shape.placement = ImagePlacement::Floating(child_placement);
    }
  }
  prepared
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::InlineVmlGroupChild;

  fn native_model() -> InlineVmlGroupViewport {
    InlineVmlGroupViewport {
      coordinates: [0.0, 1440.0, 12239.0, 14400.0],
      width_pt: 611.95,
      height_pt: 648.0,
      children: [
        [1800.0, 1440.0, 10438.0, 2356.0],
        [6494.0, 11160.0, 11492.0, 12458.0],
        [1800.0, 2294.0, 10438.0, 9562.0],
      ]
      .into_iter()
      .enumerate()
      .map(|(index, coordinates)| InlineVmlGroupChild {
        id: Arc::from(index.to_string()),
        coordinates,
      })
      .collect(),
    }
  }

  #[test]
  fn native_vml_autofit_first_only_rebuilds_absolute_host() {
    let resolved = resolve(
      &native_model(),
      [Some(12240.0), None],
      |index, [width, _]| (index == 0).then_some([width, 872.0]),
    )
    .unwrap();
    assert_eq!(resolved.host, [12240.0, 11018.0]);
    assert_eq!(resolved.children[0].height(), 872.0);
    assert_eq!(
      resolved.children[2]
        .map(resolved.viewport, resolved.host)
        .height(),
      7268.0
    );
  }

  #[test]
  fn native_vml_autofit_second_already_fits_without_tightening_group() {
    let model = native_model();
    let resolved = resolve(
      &model,
      [Some(12240.0), Some(12461.0)],
      |index, [width, _]| (index == 1).then_some([width, 1248.0]),
    )
    .unwrap();
    assert_eq!(resolved.viewport, Rect::from_coordinates(model.coordinates));
    assert_eq!(resolved.host, [12240.0, 12461.0]);
  }

  #[test]
  fn native_vml_autofit_keeps_shared_source_coordinates() {
    let model = native_model();
    let resolved = resolve(
      &model,
      [Some(12240.0), Some(12461.0)],
      |index, [width, _]| [Some(872.0), Some(1248.0), None][index].map(|height| [width, height]),
    )
    .unwrap();
    assert_eq!(
      resolved.children[2],
      Rect::from_coordinates(model.children[2].coordinates)
    );
    assert!((resolved.viewport.bottom - 12244.0).abs() <= 1.0);
    assert!((resolved.children[0].bottom - 2197.0).abs() <= 1.0);
  }

  #[test]
  fn native_vml_autofit_eight_twips_is_a_resize() {
    let rect = Rect::from_coordinates([0.0, 0.0, 100.0, 100.0]);
    assert!(rect.native_equal(Rect {
      bottom: 107.0,
      ..rect
    }));
    assert!(!rect.native_equal(Rect {
      bottom: 108.0,
      ..rect
    }));
  }

  #[test]
  fn native_vml_autofit_absolute_group_commits_before_next_child() {
    let resolved = resolve(&native_model(), [None, None], |index, [width, _]| {
      [Some(872.0), Some(1248.0), None][index].map(|height| [width, height])
    })
    .unwrap();
    assert_eq!(resolved.host, [9692.0, 10968.0]);
    assert_eq!(resolved.children[0].height(), 872.0);
    assert_eq!(resolved.children[1].height(), 1248.0);
  }

  #[test]
  fn native_vml_autofit_source_order_crosses_layout_phase_boundary() {
    for reverse in [false, true] {
      let mut model = native_model();
      let mut heights = [Some(872.0), Some(1248.0), None];
      if reverse {
        model.children.swap(0, 1);
        heights.swap(0, 1);
      }
      let resolved = resolve(&model, [Some(12240.0), None], |index, [width, _]| {
        heights[index].map(|height| [width, height])
      })
      .unwrap();
      assert_eq!(
        resolved.host,
        [12240.0, if reverse { 12733.0 } else { 12934.0 }]
      );
    }
  }
}
