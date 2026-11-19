//! Device enclosures of fixed-size floating VML groups.

use super::*;

pub(super) fn resolve(group: &v::Group, inlines: &[InlineItem]) -> Option<FloatingAlignmentExtent> {
  let style = vml_image_style(group.style.as_deref());
  let (width_pt, height_pt) = style.size_pt?;
  // A wholly absolute group locates its nominal content. Positioning either
  // axis by alignment or percentage makes the outer ink bounds own both axes.
  let uses_enclosure = style.horizontal_alignment.is_some()
    || style.vertical_alignment.is_some()
    || style.left_percent.is_some()
    || style.top_percent.is_some();
  if style.rotation_deg != 0.0
    || style.flip_horizontal
    || style.flip_vertical
    || style.relative_width_pct.is_some()
    || style.relative_height_pct.is_some()
    || !uses_enclosure
  {
    return None;
  }
  plain_path_group_extent(group, inlines, style, width_pt, height_pt)
    .or_else(|| positioned_group_extent(style, inlines, width_pt, height_pt))
}

fn plain_path_group_extent(
  group: &v::Group,
  inlines: &[InlineItem],
  style: VmlImageStyle,
  width_pt: f32,
  height_pt: f32,
) -> Option<FloatingAlignmentExtent> {
  if !matches!(
    style.horizontal_alignment,
    None | Some(HorizontalImageAlignment::Center)
  ) || !matches!(
    style.vertical_alignment,
    None | Some(VerticalImageAlignment::Center)
  ) || (style.horizontal_alignment.is_none() && style.vertical_alignment.is_none())
  {
    return None;
  }
  let (frame, children) = inlines.split_last()?;
  let InlineItem::Shape(frame) = frame else {
    return None;
  };
  let ImagePlacement::Floating(root) = frame.placement else {
    return None;
  };
  if root.alignment_extent.is_some() || children.is_empty() {
    return None;
  }
  let types = vml_group_shape_types(group);
  let mut realized = children.iter();
  let mut bounds: Option<kurbo::Rect> = None;
  for choice in &group.group_choice {
    let source = match choice {
      v::GroupChoice::Shape(shape) => shape,
      v::GroupChoice::Shapetype(_) | v::GroupChoice::TextWrap(_) | v::GroupChoice::Lock(_) => {
        continue;
      }
      _ => return None,
    };
    let InlineItem::Shape(shape) = realized.next()? else {
      return None;
    };
    let ImagePlacement::Floating(placement) = shape.placement else {
      return None;
    };
    let InlineShapeGeometry::Path {
      paths,
      preset: None,
      ..
    } = &shape.geometry
    else {
      return None;
    };
    if shape.rotation_deg != 0.0
      || shape.flip_horizontal
      || shape.flip_vertical
      || shape.effects.is_some()
      || shape.static3d.is_some()
      || shape.text_warp.is_some()
      || shape.word_text_frame
      || !shape.text_box_blocks.is_empty()
      || shape.fill_image.is_some()
      || placement.alignment_extent.is_some()
      || placement.horizontal_alignment != root.horizontal_alignment
      || placement.vertical_alignment != root.vertical_alignment
      || shape.stroke_override.as_ref().is_some_and(|stroke| {
        !matches!(stroke.compound, None | Some(common::StrokeCompound::Single))
          || stroke.head_end.is_some()
          || stroke.tail_end.is_some()
      })
    {
      return None;
    }
    let pen = (shape.stroke?.width_pt / units::POINTS_PER_CSS_PIXEL).max(1.0);
    let inherited = vml_shape_type_for_reference(source, &types).and_then(vml_shapetype_stroke);
    let limit = vml_shape_stroke(source)
      .and_then(|stroke| stroke.miterlimit.as_deref())
      .or_else(|| inherited.and_then(|stroke| stroke.miterlimit.as_deref()))
      .and_then(|value| value.parse::<f32>().ok())
      .unwrap_or(8.0);
    if !limit.is_finite() || limit < 0.5 {
      return None;
    }
    let (x, y) = child_offset(shape, root, width_pt, height_pt);
    let transform = Affine::scale(1.0 / f64::from(units::POINTS_PER_CSS_PIXEL))
      * Affine::translate((
        f64::from(style.horizontal_offset_pt + x),
        f64::from(style.vertical_offset_pt + y),
      ));
    for path in paths {
      if !path.stroke || path.commands.is_empty() {
        return None;
      }
      if path
        .commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::CubicTo { .. }))
      {
        return None;
      }
      let path = BezPath::from_vec(common::drawingml_geometry::mapped_path_elements(
        &path.commands,
        common::drawingml_geometry::kurbo_point,
      ));
      let measured = common::vml_path_bounds::stroked_bounds(&(transform * path), pen, limit)?;
      bounds = Some(bounds.map_or(measured, |bounds| bounds.union(measured)));
    }
  }
  if realized.next().is_some() {
    return None;
  }
  let bounds = bounds?;
  let grid = f64::from(units::POINTS_PER_CSS_PIXEL);
  let left = f64::from(style.horizontal_offset_pt);
  let top = f64::from(style.vertical_offset_pt);
  // The Word host compares the integral device bounds with the authored
  // rectangle, retaining only outward growth. Paint joins are independent.
  let outsets = [
    left - bounds.x0.floor() * grid,
    top - bounds.y0.floor() * grid,
    bounds.x1.ceil() * grid - left - f64::from(width_pt),
    bounds.y1.ceil() * grid - top - f64::from(height_pt),
  ]
  .map(|value| value.max(0.0) as f32);
  Some(FloatingAlignmentExtent {
    vml_child_resize: Default::default(),
    width_pt,
    height_pt,
    vml_auto_fit_resolved: false,
    wordprocessing_effect_extent_pt: None,
    vml_path_effect_extent_pt: Some(outsets),
    relative_width_to: None,
    relative_width_pct: None,
    relative_height_to: None,
    relative_height_pct: None,
  })
}

// Absolute offsets, percentages and alignment address the outer OfficeArt
// enclosure. The authored offsets supply its 96-dpi measurement phase.
// A nested child's coordinates here are already in the root viewport.
fn positioned_group_extent(
  style: VmlImageStyle,
  inlines: &[InlineItem],
  width_pt: f32,
  height_pt: f32,
) -> Option<FloatingAlignmentExtent> {
  let (InlineItem::Shape(frame), children) = inlines.split_last()? else {
    return None;
  };
  let ImagePlacement::Floating(root) = frame.placement else {
    return None;
  };
  let mut outsets = [0.0_f32; 4];
  for item in children {
    let InlineItem::Shape(shape) = item else {
      return None;
    };
    let ImagePlacement::Floating(_) = shape.placement else {
      return None;
    };
    if shape.rotation_deg != 0.0
      || shape.effects.is_some()
      || shape.static3d.is_some()
      || shape.flip_horizontal
      || shape.flip_vertical
      || shape.text_box_resizes_to_fit
      || shape.text_warp.is_some()
      || shape.fill_image.is_some()
      || shape
        .stroke_override
        .as_ref()
        .is_some_and(|stroke| stroke.head_end.is_some() || stroke.tail_end.is_some())
    {
      return None;
    }
    let pen = shape.stroke.map_or(0.0, |stroke| {
      stroke.width_pt.max(units::POINTS_PER_CSS_PIXEL)
    });
    let (x, y) = child_offset(shape, root, width_pt, height_pt);
    // Curves and mitered joins can extend beyond the authored rectangle.
    if let InlineShapeGeometry::Path { paths, .. } = &shape.geometry {
      let grid = f64::from(units::POINTS_PER_CSS_PIXEL);
      let transform = Affine::scale(1.0 / grid)
        * Affine::translate((
          f64::from(style.horizontal_offset_pt + x),
          f64::from(style.vertical_offset_pt + y),
        ));
      for path in paths {
        let path_geometry = BezPath::from_vec(common::drawingml_geometry::mapped_path_elements(
          &path.commands,
          common::drawingml_geometry::kurbo_point,
        ));
        let path_geometry = transform * path_geometry;
        let bounds = if path.stroke && pen > 0.0 {
          let limit = match shape
            .stroke_override
            .as_ref()
            .and_then(|stroke| stroke.join)
          {
            Some(common::StrokeJoin::Miter { limit: Some(limit) }) => limit,
            _ => 8.0,
          };
          common::vml_path_bounds::stroked_bounds(
            &path_geometry,
            pen / units::POINTS_PER_CSS_PIXEL,
            limit,
          )?
        } else {
          path_geometry.control_box()
        };
        let snap = |v: f64, leading: bool| {
          (if path.stroke && pen > 0.0 {
            if leading { v.floor() } else { v.ceil() }
          } else {
            v.round()
          }) * grid
        };
        let left = f64::from(style.horizontal_offset_pt);
        let top = f64::from(style.vertical_offset_pt);
        for (extent, candidate) in outsets.iter_mut().zip([
          left - snap(bounds.x0, true),
          top - snap(bounds.y0, true),
          snap(bounds.x1, false) - left - f64::from(width_pt),
          snap(bounds.y1, false) - top - f64::from(height_pt),
        ]) {
          *extent = extent.max(candidate as f32);
        }
      }
      continue;
    }
    let (left, right) =
      vml_device_axis_outsets(style.horizontal_offset_pt + x, shape.width_pt, pen);
    let (top, bottom) = vml_device_axis_outsets(style.vertical_offset_pt + y, shape.height_pt, pen);
    for (extent, candidate) in outsets.iter_mut().zip([
      left - x,
      top - y,
      x + shape.width_pt + right - width_pt,
      y + shape.height_pt + bottom - height_pt,
    ]) {
      *extent = extent.max(candidate);
    }
  }
  Some(FloatingAlignmentExtent {
    vml_child_resize: Default::default(),
    width_pt,
    height_pt,
    vml_auto_fit_resolved: false,
    wordprocessing_effect_extent_pt: None,
    vml_path_effect_extent_pt: Some(outsets),
    relative_width_to: None,
    relative_width_pct: None,
    relative_height_to: None,
    relative_height_pct: None,
  })
}

fn child_offset(
  shape: &InlineShape,
  root: FloatingImagePlacement,
  width: f32,
  height: f32,
) -> (f32, f32) {
  let ImagePlacement::Floating(placement) = shape.placement else {
    unreachable!("validated floating VML group child");
  };
  if placement.alignment_extent.is_some() {
    return (
      placement.group_child_offset_x_pt,
      placement.group_child_offset_y_pt,
    );
  }
  (
    placement.horizontal_offset_pt
      - root.horizontal_offset_pt
      - vml_group_aligned_child_offset(root.horizontal_alignment, width, 0.0, shape.width_pt),
    placement.vertical_offset_pt
      - root.vertical_offset_pt
      - vml_group_aligned_vertical_child_offset(
        root.vertical_alignment,
        height,
        0.0,
        shape.height_pt,
      ),
  )
}

pub(super) fn apply(
  inlines: &mut [InlineItem],
  root: FloatingImagePlacement,
  extent: FloatingAlignmentExtent,
) {
  for item in inlines {
    let InlineItem::Shape(shape) = item else {
      unreachable!("validated plain VML group");
    };
    let (x, y) = child_offset(shape, root, extent.width_pt, extent.height_pt);
    if let ImagePlacement::Floating(placement) = &mut shape.placement {
      placement.alignment_extent = Some(extent);
      placement.horizontal_offset_pt = root.horizontal_offset_pt;
      placement.vertical_offset_pt = root.vertical_offset_pt;
      placement.group_child_offset_x_pt = x;
      placement.group_child_offset_y_pt = y;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_path_groups_keep_conservative_bounds_separate_from_painted_joins() {
    for (path, pen, expected) in [
      ("m0,0l5760,0,5760,4320,0,4320xe", 0.75, [0.75; 4]),
      ("m0,0l5760,0,5760,4320,0,4320xe", 2.0, [1.5; 4]),
      ("m0,0l5760,0,2880,4320xe", 0.75, [1.5, 0.75, 1.5, 0.75]),
      ("m0,0l5760,0,2880,4320xe", 2.0, [3.0, 1.5, 3.0, 1.5]),
      ("m0,0l5760,216,0,4320xe", 0.75, [0.75, 1.5, 1.5, 1.5]),
      ("m0,0l5760,216,0,4320xe", 2.0, [1.5, 2.25, 4.5, 3.0]),
    ] {
      for join in ["round", "miter", "bevel"] {
        let xml = format!(
          r#"<v:group xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office" coordsize="5760,4320"
          style="position:absolute;width:720pt;height:540pt;mso-position-horizontal:center;mso-position-vertical:center">
          <o:lock v:ext="edit" aspectratio="t"/>
          <v:shape coordsize="5760,4320" style="position:absolute;width:5760;height:4320"
          path="{path}" strokeweight="{pen}pt"><v:stroke joinstyle="{join}" miterlimit="10"/></v:shape></v:group>"#
        );
        let group = v::Group::from_bytes(xml.as_bytes()).unwrap();
        let v::GroupChoice::Shape(source) = &group.group_choice[1] else {
          unreachable!()
        };
        let transform = VmlGroupTransform::from_group(&group).unwrap();
        let child_style = transform
          .child_anchor_style(group.style.as_deref(), source.style.as_deref())
          .unwrap();
        let child =
          vml_shape_shape_with_style(source, Some(&child_style), &ImageCatalog::default(), &[])
            .unwrap();
        let frame = vml_inline_group_frame(&group).unwrap();
        let mut items = vec![InlineItem::Shape(child), InlineItem::Shape(frame)];
        let extent = resolve(&group, &items).unwrap();
        assert_eq!(
          extent.vml_path_effect_extent_pt,
          Some(expected),
          "{path} pen{pen} {join}"
        );
        let InlineItem::Shape(frame) = &items[1] else {
          unreachable!()
        };
        let ImagePlacement::Floating(root) = frame.placement else {
          unreachable!()
        };
        apply(&mut items, root, extent);
        for item in items {
          let InlineItem::Shape(shape) = item else {
            unreachable!()
          };
          let ImagePlacement::Floating(placement) = shape.placement else {
            unreachable!()
          };
          assert_eq!(placement.alignment_extent, Some(extent));
          assert_eq!(
            (
              placement.group_child_offset_x_pt,
              placement.group_child_offset_y_pt
            ),
            (0.0, 0.0)
          );
        }
      }
    }
  }
}
