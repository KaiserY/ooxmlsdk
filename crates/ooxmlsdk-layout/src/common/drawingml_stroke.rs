use kurbo::{
  BezPath, Cap as KurboCap, Join as KurboJoin, Shape as KurboShape, Stroke as KurboStroke,
  StrokeOpts, stroke as widen_path,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{
  DisplayItem, PathCommand, PathItem, Point, Pt, Rect, Size, Stroke, StrokeAlignment, StrokeCap,
  StrokeCompound, StrokeDashPreset, StrokeEnd, StrokeEndKind, StrokeEndSize, StrokeJoin,
};

const MIN_MARKER_BASE_PT: f32 = 70.0 * 72.0 / 2_540.0;
// Match the path-flattening accuracy used by the DrawingML effect masks.  This
// is intentionally expressed in page points: the widened bounds are a logical
// image boundary and must not change when Direct2D/Word pre-scales a blur.
const STROKE_BOUNDS_FLATTENING_TOLERANCE_PT: f64 = 0.01;

/// Resolves DrawingML line properties at their authored field granularity.
///
/// LibreOffice's `LineProperties::assignUsed()` applies reference, theme, and
/// direct line properties in that order, replacing only fields which are
/// present in the later source.  In particular, an `a:ln` which changes only
/// its fill must retain the width, dash, joins, and line ends supplied by its
/// `a:lnRef` theme style.  Keeping the merged schema node also delays custom
/// dash expansion until the final inherited width is known.
pub(crate) fn merge_outlines(
  base: Option<&a::Outline>,
  direct: Option<&a::Outline>,
) -> Option<a::Outline> {
  match (base, direct) {
    (Some(base), Some(direct)) => {
      let mut merged = base.clone();
      merge_outline_into(&mut merged, direct);
      Some(merged)
    }
    (Some(base), None) => Some(base.clone()),
    (None, Some(direct)) => Some(direct.clone()),
    (None, None) => None,
  }
}

fn merge_outline_into(target: &mut a::Outline, source: &a::Outline) {
  if source.width.is_some() {
    target.width = source.width;
  }
  if source.cap_type.is_some() {
    target.cap_type = source.cap_type;
  }
  if source.compound_line_type.is_some() {
    target.compound_line_type = source.compound_line_type;
  }
  if source.alignment.is_some() {
    target.alignment = source.alignment;
  }
  if let Some(source_fill) = source.outline_choice1.as_ref() {
    merge_outline_fill(&mut target.outline_choice1, source_fill);
  }
  if let Some(source_dash) = source.outline_choice2.as_ref() {
    let empty_custom_dash =
      matches!(source_dash, a::OutlineChoice2::CustomDash(dash) if dash.dash_stop.is_empty());
    if !empty_custom_dash {
      target.outline_choice2 = Some(source_dash.clone());
    }
  }
  if source.outline_choice3.is_some() {
    target.outline_choice3 = source.outline_choice3.clone();
  }
  if let Some(source_end) = source.head_end.as_ref() {
    merge_head_end(&mut target.head_end, source_end);
  }
  if let Some(source_end) = source.tail_end.as_ref() {
    merge_tail_end(&mut target.tail_end, source_end);
  }
  if source.line_properties_extension_list.is_some() {
    target.line_properties_extension_list = source.line_properties_extension_list.clone();
  }
}

fn merge_outline_fill(target: &mut Option<a::OutlineChoice>, source: &a::OutlineChoice) {
  match (target.as_mut(), source) {
    (Some(a::OutlineChoice::SolidFill(target)), a::OutlineChoice::SolidFill(source)) => {
      if source.solid_fill_choice.is_some() {
        target.solid_fill_choice = source.solid_fill_choice.clone();
      }
    }
    (Some(a::OutlineChoice::GradientFill(target)), a::OutlineChoice::GradientFill(source)) => {
      merge_gradient_fill(target, source);
    }
    (Some(a::OutlineChoice::PatternFill(target)), a::OutlineChoice::PatternFill(source)) => {
      merge_pattern_fill(target, source);
    }
    _ => *target = Some(source.clone()),
  }
}

fn merge_gradient_fill(target: &mut a::GradientFill, source: &a::GradientFill) {
  if source.flip.is_some() {
    target.flip = source.flip;
  }
  if source.rotate_with_shape.is_some() {
    target.rotate_with_shape = source.rotate_with_shape;
  }
  if source
    .gradient_stop_list
    .as_ref()
    .is_some_and(|list| !list.gradient_stop.is_empty())
  {
    target.gradient_stop_list = source.gradient_stop_list.clone();
  }
  if let Some(source_kind) = source.gradient_fill_choice.as_ref() {
    match (target.gradient_fill_choice.as_mut(), source_kind) {
      (
        Some(a::GradientFillChoice::LinearGradientFill(target)),
        a::GradientFillChoice::LinearGradientFill(source),
      ) => {
        if source.angle.is_some() {
          target.angle = source.angle;
        }
        if source.scaled.is_some() {
          target.scaled = source.scaled;
        }
      }
      (
        Some(a::GradientFillChoice::PathGradientFill(target)),
        a::GradientFillChoice::PathGradientFill(source),
      ) => {
        if source.path.is_some() {
          target.path = source.path;
        }
        if source.fill_to_rectangle.is_some() {
          target.fill_to_rectangle = source.fill_to_rectangle.clone();
        }
      }
      _ => target.gradient_fill_choice = Some(source_kind.clone()),
    }
  }
  if source.tile_rectangle.is_some() {
    target.tile_rectangle = source.tile_rectangle.clone();
  }
}

fn merge_pattern_fill(target: &mut a::PatternFill, source: &a::PatternFill) {
  if source.preset.is_some() {
    target.preset = source.preset;
  }
  if source
    .foreground_color
    .as_ref()
    .is_some_and(|color| color.foreground_color_choice.is_some())
  {
    target.foreground_color = source.foreground_color.clone();
  }
  if source
    .background_color
    .as_ref()
    .is_some_and(|color| color.background_color_choice.is_some())
  {
    target.background_color = source.background_color.clone();
  }
}

fn merge_head_end(target: &mut Option<a::HeadEnd>, source: &a::HeadEnd) {
  let target = target.get_or_insert_with(a::HeadEnd::default);
  if source.r#type.is_some() {
    target.r#type = source.r#type;
  }
  if source.width.is_some() {
    target.width = source.width;
  }
  if source.length.is_some() {
    target.length = source.length;
  }
}

fn merge_tail_end(target: &mut Option<a::TailEnd>, source: &a::TailEnd) {
  let target = target.get_or_insert_with(a::TailEnd::default);
  if source.r#type.is_some() {
    target.r#type = source.r#type;
  }
  if source.width.is_some() {
    target.width = source.width;
  }
  if source.length.is_some() {
    target.length = source.length;
  }
}

pub(crate) fn apply_outline_style(stroke: &mut Stroke<'_>, outline: &a::Outline) {
  stroke.cap = outline.cap_type.map(line_cap);
  stroke.compound = outline.compound_line_type.map(compound);
  stroke.alignment = outline.alignment.map(alignment);
  match outline.outline_choice2.as_ref() {
    Some(a::OutlineChoice2::PresetDash(dash)) => {
      stroke.preset_dash = dash.val.map(preset_dash);
      stroke.dash = None;
    }
    Some(a::OutlineChoice2::CustomDash(dash)) => {
      stroke.preset_dash = None;
      stroke.dash = Some(
        dash
          .dash_stop
          .iter()
          .flat_map(|stop| {
            [
              Pt(stroke.width.0 * stop.dash_length.as_ratio() as f32),
              Pt(stroke.width.0 * stop.space_length.as_ratio() as f32),
            ]
          })
          .collect(),
      );
    }
    None => {}
  }
  stroke.join = match outline.outline_choice3.as_ref() {
    Some(a::OutlineChoice3::Round) => Some(StrokeJoin::Round),
    Some(a::OutlineChoice3::LineJoinBevel) => Some(StrokeJoin::Bevel),
    Some(a::OutlineChoice3::Miter(miter)) => Some(StrokeJoin::Miter {
      limit: miter.limit.map(|limit| limit.as_ratio() as f32),
    }),
    None => None,
  };
  stroke.head_end = outline.head_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
  stroke.tail_end = outline.tail_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
}

/// Applies only line-style fields explicitly present on `outline` over an
/// already resolved inherited stroke.
///
/// `apply_outline_style` is the terminal conversion for a complete outline
/// and therefore clears omitted cap/join/end fields. Chart markers also allow
/// a partial direct `a:ln` over a themed line; at that cascade boundary an
/// omitted field must retain its inherited value instead.
pub(crate) fn apply_outline_style_over_inherited(stroke: &mut Stroke<'_>, outline: &a::Outline) {
  let mut direct = stroke.clone();
  apply_outline_style(&mut direct, outline);
  if outline.cap_type.is_some() {
    stroke.cap = direct.cap;
  }
  if outline.compound_line_type.is_some() {
    stroke.compound = direct.compound;
  }
  if outline.alignment.is_some() {
    stroke.alignment = direct.alignment;
  }
  if outline.outline_choice2.as_ref().is_some_and(
    |choice| !matches!(choice, a::OutlineChoice2::CustomDash(dash) if dash.dash_stop.is_empty()),
  ) {
    stroke.preset_dash = direct.preset_dash;
    stroke.dash = direct.dash;
  }
  if outline.outline_choice3.is_some() {
    stroke.join = direct.join;
  }
  if outline.head_end.is_some() {
    stroke.head_end = direct.head_end;
  }
  if outline.tail_end.is_some() {
    stroke.tail_end = direct.tail_end;
  }
}

fn line_cap(value: a::LineCapValues) -> StrokeCap {
  match value {
    a::LineCapValues::Round => StrokeCap::Round,
    a::LineCapValues::Square => StrokeCap::Square,
    a::LineCapValues::Flat => StrokeCap::Flat,
  }
}

fn compound(value: a::CompoundLineValues) -> StrokeCompound {
  match value {
    a::CompoundLineValues::Single => StrokeCompound::Single,
    a::CompoundLineValues::Double => StrokeCompound::Double,
    a::CompoundLineValues::ThickThin => StrokeCompound::ThickThin,
    a::CompoundLineValues::ThinThick => StrokeCompound::ThinThick,
    a::CompoundLineValues::Triple => StrokeCompound::Triple,
  }
}

fn alignment(value: a::PenAlignmentValues) -> StrokeAlignment {
  match value {
    a::PenAlignmentValues::Center => StrokeAlignment::Center,
    a::PenAlignmentValues::Insert => StrokeAlignment::Inside,
  }
}

fn preset_dash(value: a::PresetLineDashValues) -> StrokeDashPreset {
  match value {
    a::PresetLineDashValues::Solid => StrokeDashPreset::Solid,
    a::PresetLineDashValues::Dot => StrokeDashPreset::Dot,
    a::PresetLineDashValues::Dash => StrokeDashPreset::Dash,
    a::PresetLineDashValues::LargeDash => StrokeDashPreset::LargeDash,
    a::PresetLineDashValues::DashDot => StrokeDashPreset::DashDot,
    a::PresetLineDashValues::LargeDashDot => StrokeDashPreset::LargeDashDot,
    a::PresetLineDashValues::LargeDashDotDot => StrokeDashPreset::LargeDashDotDot,
    a::PresetLineDashValues::SystemDash => StrokeDashPreset::SystemDash,
    a::PresetLineDashValues::SystemDot => StrokeDashPreset::SystemDot,
    a::PresetLineDashValues::SystemDashDot => StrokeDashPreset::SystemDashDot,
    a::PresetLineDashValues::SystemDashDotDot => StrokeDashPreset::SystemDashDotDot,
  }
}

fn line_end(value: a::LineEndValues) -> StrokeEndKind {
  match value {
    a::LineEndValues::None => StrokeEndKind::None,
    a::LineEndValues::Triangle => StrokeEndKind::Triangle,
    a::LineEndValues::Stealth => StrokeEndKind::Stealth,
    a::LineEndValues::Diamond => StrokeEndKind::Diamond,
    a::LineEndValues::Oval => StrokeEndKind::Oval,
    a::LineEndValues::Arrow => StrokeEndKind::Arrow,
  }
}

fn end_width(value: a::LineEndWidthValues) -> StrokeEndSize {
  match value {
    a::LineEndWidthValues::Small => StrokeEndSize::Small,
    a::LineEndWidthValues::Medium => StrokeEndSize::Medium,
    a::LineEndWidthValues::Large => StrokeEndSize::Large,
  }
}

fn end_length(value: a::LineEndLengthValues) -> StrokeEndSize {
  match value {
    a::LineEndLengthValues::Small => StrokeEndSize::Small,
    a::LineEndLengthValues::Medium => StrokeEndSize::Medium,
    a::LineEndLengthValues::Large => StrokeEndSize::Large,
  }
}

pub(crate) fn stroke_end_marker_polygons(
  path: &PathItem<'_>,
  stroke: &Stroke<'_>,
) -> Vec<Vec<Point>> {
  let Some(endpoints) = path_endpoints(path) else {
    return Vec::new();
  };
  [
    stroke
      .head_end
      .and_then(|marker| marker_polygon(marker, endpoints.start, endpoints.start_outward, stroke)),
    stroke
      .tail_end
      .and_then(|marker| marker_polygon(marker, endpoints.end, endpoints.end_outward, stroke)),
  ]
  .into_iter()
  .flatten()
  .collect()
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct StrokedOpenArrowMarker {
  pub(crate) points: [Point; 3],
  pub(crate) width: Pt,
  pub(crate) bounds: Rect,
}

pub(crate) fn stroked_open_arrow_markers(
  path: &PathItem<'_>,
  stroke: &Stroke<'_>,
) -> Vec<StrokedOpenArrowMarker> {
  let Some(endpoints) = path_endpoints(path) else {
    return Vec::new();
  };
  [
    stroke.head_end.and_then(|marker| {
      stroked_open_arrow(marker, endpoints.start, endpoints.start_outward, stroke)
    }),
    stroke
      .tail_end
      .and_then(|marker| stroked_open_arrow(marker, endpoints.end, endpoints.end_outward, stroke)),
  ]
  .into_iter()
  .flatten()
  .collect()
}

pub(crate) fn stroke_end_shaft_insets(stroke: &Stroke<'_>) -> (f32, f32) {
  (
    stroke
      .head_end
      .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
      .map(|_| stroke.width.0)
      .unwrap_or_default(),
    stroke
      .tail_end
      .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
      .map(|_| stroke.width.0)
      .unwrap_or_default(),
  )
}

pub(crate) fn stroke_end_marker_bounds(path: &PathItem<'_>, stroke: &Stroke<'_>) -> Option<Rect> {
  let mut bounds = None;
  for polygon in stroke_end_marker_polygons(path, stroke) {
    bounds = union_optional_rect(bounds, points_bounds(&polygon));
  }
  for marker in stroked_open_arrow_markers(path, stroke) {
    bounds = union_optional_rect(bounds, Some(marker.bounds));
  }
  bounds
}

/// Returns the visible bounds of a DrawingML path stroke, including line-end
/// markers.
///
/// Direct2D's `ID2D1Geometry::GetWidenedBounds` is the reference contract for
/// an effect source made from vector primitives.  Microsoft's
/// `GeometryRealizationSample` uses that widened result (rather than the fill
/// geometry bounds) when allocating a stroked opacity mask.  Keep the same
/// distinction here so a shadow/glow receives the complete alpha of the line,
/// including cap, join, dash, and marker geometry.
pub(crate) fn path_stroke_bounds(path: &PathItem<'_>, stroke: &Stroke<'_>) -> Option<Rect> {
  if stroke.width.0 <= 0.0
    || stroke.color.a == 0 && stroke.pattern.is_none() && stroke.gradient.is_none()
  {
    return None;
  }

  let elements = path_kurbo_elements(path);
  let mut bounds = if elements.is_empty() {
    None
  } else {
    let centerline = BezPath::from_vec(elements.clone());
    let outline = widen_path(
      elements,
      &kurbo_stroke_style(stroke, 1.0),
      &StrokeOpts::default(),
      STROKE_BOUNDS_FLATTENING_TOLERANCE_PT,
    );
    let mut outline_bounds = outline.bounding_box();

    // DrawingML `algn="in"` is an inset pen.  For a closed path its visible
    // ink cannot escape the fill geometry.  Intersecting the centered widened
    // outline with the geometry bounds supplies the tight axis-aligned range
    // while retaining cap/join/dash handling for the default centered pen.
    if path.closed && stroke.alignment == Some(StrokeAlignment::Inside) {
      let geometry_bounds = centerline.bounding_box();
      outline_bounds = kurbo::Rect::new(
        outline_bounds.x0.max(geometry_bounds.x0),
        outline_bounds.y0.max(geometry_bounds.y0),
        outline_bounds.x1.min(geometry_bounds.x1),
        outline_bounds.y1.min(geometry_bounds.y1),
      );
    }
    kurbo_rect(outline_bounds)
  };

  bounds = union_optional_rect(bounds, stroke_end_marker_bounds(path, stroke));
  bounds
}

/// Collects widened stroke bounds for every vector item in an effect source.
/// This deliberately excludes text outline measurement: Word text effects
/// have a separate glyph-ink boundary, while DrawingML shape/group effects
/// feed Path/Rect/Line primitives through this vector source.
pub(crate) fn display_items_stroke_bounds(items: &[DisplayItem<'_>]) -> Option<Rect> {
  let mut bounds = None;
  for item in items {
    let item_bounds = match item {
      DisplayItem::Path(path) => path
        .stroke
        .as_ref()
        .and_then(|stroke| path_stroke_bounds(path, stroke)),
      DisplayItem::Rect(rect) => rect
        .stroke
        .as_ref()
        .and_then(|stroke| path_stroke_bounds(&rectangle_path(rect.bounds), stroke)),
      DisplayItem::Line(line) => path_stroke_bounds(&line_path(line.start, line.end), &line.stroke),
      DisplayItem::Group(group) => display_items_stroke_bounds(&group.items),
      DisplayItem::Text(_)
      | DisplayItem::Glyphs(_)
      | DisplayItem::Image(_)
      | DisplayItem::LinkArea(_)
      | DisplayItem::AnnotationHint(_)
      | DisplayItem::Clip(_)
      | DisplayItem::Transform(_) => None,
    };
    bounds = union_optional_rect(bounds, item_bounds);
  }
  bounds
}

fn rectangle_path(bounds: Rect) -> PathItem<'static> {
  let left = bounds.origin.x.0;
  let top = bounds.origin.y.0;
  let right = left + bounds.size.width.0;
  let bottom = top + bounds.size.height.0;
  PathItem {
    bounds,
    commands: vec![
      PathCommand::MoveTo(Point {
        x: Pt(left),
        y: Pt(top),
      }),
      PathCommand::LineTo(Point {
        x: Pt(right),
        y: Pt(top),
      }),
      PathCommand::LineTo(Point {
        x: Pt(right),
        y: Pt(bottom),
      }),
      PathCommand::LineTo(Point {
        x: Pt(left),
        y: Pt(bottom),
      }),
      PathCommand::Close,
    ],
    closed: true,
    ..PathItem::default()
  }
}

fn line_path(start: Point, end: Point) -> PathItem<'static> {
  PathItem {
    bounds: Rect {
      origin: Point {
        x: Pt(start.x.0.min(end.x.0)),
        y: Pt(start.y.0.min(end.y.0)),
      },
      size: Size {
        width: Pt((end.x.0 - start.x.0).abs()),
        height: Pt((end.y.0 - start.y.0).abs()),
      },
    },
    commands: vec![PathCommand::MoveTo(start), PathCommand::LineTo(end)],
    closed: false,
    ..PathItem::default()
  }
}

pub(crate) fn kurbo_stroke_style(stroke: &Stroke<'_>, coordinate_scale: f32) -> KurboStroke {
  let mut style = KurboStroke::new(f64::from(stroke.width.0 * coordinate_scale));
  style.join = match stroke.join {
    Some(StrokeJoin::Round) => KurboJoin::Round,
    Some(StrokeJoin::Bevel) => KurboJoin::Bevel,
    Some(StrokeJoin::Miter { .. }) | None => KurboJoin::Miter,
  };
  style.miter_limit = match stroke.join {
    Some(StrokeJoin::Miter { limit: Some(limit) }) => f64::from(limit),
    _ => KurboStroke::default().miter_limit,
  };
  let cap = match stroke.cap {
    Some(StrokeCap::Round) => KurboCap::Round,
    Some(StrokeCap::Square) => KurboCap::Square,
    Some(StrokeCap::Flat) | None => KurboCap::Butt,
  };
  style.start_cap = cap;
  style.end_cap = cap;
  if let Some(dash) = stroke.resolved_dash() {
    style = style.with_dashes(
      f64::from(stroke.dash_offset.0 * coordinate_scale),
      dash
        .iter()
        .map(|length| f64::from(length.0 * coordinate_scale)),
    );
  }
  style
}

fn path_kurbo_elements(path: &PathItem<'_>) -> Vec<kurbo::PathEl> {
  if !path.commands.is_empty() {
    return super::drawingml_geometry::mapped_path_elements(
      &path.commands,
      super::drawingml_geometry::kurbo_point,
    );
  }
  let Some(first) = path.points.first() else {
    return Vec::new();
  };
  let mut output = BezPath::new();
  output.move_to((f64::from(first.x.0), f64::from(first.y.0)));
  for point in &path.points[1..] {
    output.line_to((f64::from(point.x.0), f64::from(point.y.0)));
  }
  if path.closed {
    output.close_path();
  }
  output.into_elements()
}

fn kurbo_rect(bounds: kurbo::Rect) -> Option<Rect> {
  if !bounds.x0.is_finite()
    || !bounds.y0.is_finite()
    || !bounds.x1.is_finite()
    || !bounds.y1.is_finite()
    || bounds.x0 > bounds.x1
    || bounds.y0 > bounds.y1
  {
    return None;
  }
  Some(Rect {
    origin: Point {
      x: Pt(bounds.x0 as f32),
      y: Pt(bounds.y0 as f32),
    },
    size: Size {
      width: Pt(bounds.width() as f32),
      height: Pt(bounds.height() as f32),
    },
  })
}

fn points_bounds(points: &[Point]) -> Option<Rect> {
  let first = points.first()?;
  let (mut left, mut top, mut right, mut bottom) = (first.x.0, first.y.0, first.x.0, first.y.0);
  for point in &points[1..] {
    left = left.min(point.x.0);
    top = top.min(point.y.0);
    right = right.max(point.x.0);
    bottom = bottom.max(point.y.0);
  }
  Some(Rect {
    origin: Point {
      x: Pt(left),
      y: Pt(top),
    },
    size: Size {
      width: Pt(right - left),
      height: Pt(bottom - top),
    },
  })
}

fn union_optional_rect(first: Option<Rect>, second: Option<Rect>) -> Option<Rect> {
  match (first, second) {
    (None, other) | (other, None) => other,
    (Some(first), Some(second)) => {
      let left = first.origin.x.0.min(second.origin.x.0);
      let top = first.origin.y.0.min(second.origin.y.0);
      let right =
        (first.origin.x.0 + first.size.width.0).max(second.origin.x.0 + second.size.width.0);
      let bottom =
        (first.origin.y.0 + first.size.height.0).max(second.origin.y.0 + second.size.height.0);
      Some(Rect {
        origin: Point {
          x: Pt(left),
          y: Pt(top),
        },
        size: Size {
          width: Pt(right - left),
          height: Pt(bottom - top),
        },
      })
    }
  }
}

struct PathEndpoints {
  start: (f32, f32),
  start_outward: (f32, f32),
  end: (f32, f32),
  end_outward: (f32, f32),
}

fn path_endpoints(path: &PathItem<'_>) -> Option<PathEndpoints> {
  if path.closed {
    return None;
  }
  if path.commands.is_empty() {
    let [first, second, ..] = path.points.as_slice() else {
      return None;
    };
    let penultimate = path.points[path.points.len() - 2];
    let last = path.points[path.points.len() - 1];
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
  for command in &path.commands {
    match *command {
      PathCommand::MoveTo(point) => {
        current = Some((point.x.0, point.y.0));
        first.get_or_insert((point.x.0, point.y.0));
      }
      PathCommand::LineTo(point) => {
        let start = current?;
        let end = (point.x.0, point.y.0);
        first_tangent.get_or_insert((start, end));
        last_tangent = Some((start, end));
        current = Some(end);
      }
      PathCommand::CubicTo {
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
      PathCommand::Close => return None,
    }
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

fn marker_dimensions(marker: StrokeEnd, line_width: f32) -> (f32, f32) {
  let is_open_arrow = marker.kind == StrokeEndKind::Arrow;
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  if marker.kind == StrokeEndKind::Arrow
    && marker.width == StrokeEndSize::Medium
    && marker.length == StrokeEndSize::Medium
    && !uses_stroked_open_arrow(marker, line_width)
  {
    return (
      3.5 * baseline + (3.0_f32.sqrt() / 2.0) * line_width,
      3.0 * baseline + 0.75 * line_width,
    );
  }
  (
    marker_size_factor(marker.width, is_open_arrow) * baseline,
    marker_size_factor(marker.length, is_open_arrow) * baseline,
  )
}

fn marker_size_factor(size: StrokeEndSize, is_open_arrow: bool) -> f32 {
  match (size, is_open_arrow) {
    (StrokeEndSize::Small, false) => 2.0,
    (StrokeEndSize::Medium, false) => 3.0,
    (StrokeEndSize::Large, false) => 5.0,
    (StrokeEndSize::Small, true) => 2.5,
    (StrokeEndSize::Medium, true) => 3.5,
    (StrokeEndSize::Large, true) => 5.5,
  }
}

fn uses_stroked_open_arrow(marker: StrokeEnd, line_width: f32) -> bool {
  // Once the authored line width drives the marker scale, Word fixed output
  // paints an open arrow as a round-capped V and ends the shaft one line width
  // before its tip. Keep the fixed-minimum LibreOffice polygon below this
  // boundary; thin and hairline Office goldens exercise that distinct shape.
  marker.kind == StrokeEndKind::Arrow && line_width >= MIN_MARKER_BASE_PT
}

fn stroked_open_arrow(
  marker: StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  stroke: &Stroke<'_>,
) -> Option<StrokedOpenArrowMarker> {
  if !uses_stroked_open_arrow(marker, stroke.width.0) {
    return None;
  }
  let baseline = stroke.width.0.max(MIN_MARKER_BASE_PT);
  let marker_width = marker_size_factor(marker.width, true) * baseline;
  let marker_length = marker_size_factor(marker.length, true) * baseline;
  let radius = stroke.width.0 / 2.0;
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
  let point = |back: f32, across: f32| Point {
    x: Pt(endpoint.0 - outward.0 * back + perpendicular.0 * across),
    y: Pt(endpoint.1 - outward.1 * back + perpendicular.1 * across),
  };
  let left = point(base_offset, -half_width);
  let apex = point(miter_inset, 0.0);
  let right = point(base_offset, half_width);
  let cap_bounds = |point: Point| Rect {
    origin: Point {
      x: Pt(point.x.0 - radius),
      y: Pt(point.y.0 - radius),
    },
    size: Size {
      width: Pt(stroke.width.0),
      height: Pt(stroke.width.0),
    },
  };
  let tip_bounds = Rect {
    origin: Point {
      x: Pt(endpoint.0),
      y: Pt(endpoint.1),
    },
    size: Size::default(),
  };
  let bounds = union_optional_rect(
    union_optional_rect(Some(cap_bounds(left)), Some(cap_bounds(right))),
    Some(tip_bounds),
  )?;
  Some(StrokedOpenArrowMarker {
    points: [left, apex, right],
    width: stroke.width,
    bounds,
  })
}

fn marker_polygon(
  marker: StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  stroke: &Stroke<'_>,
) -> Option<Vec<Point>> {
  if marker.kind == StrokeEndKind::None || uses_stroked_open_arrow(marker, stroke.width.0) {
    return None;
  }
  let (width, length) = marker_dimensions(marker, stroke.width.0);
  let centered = matches!(marker.kind, StrokeEndKind::Diamond | StrokeEndKind::Oval);
  let line_half_width = (50.0 * stroke.width.0 / width).max(1.0);
  let points: &[(f32, f32)] = match marker.kind {
    StrokeEndKind::Triangle => &[(50.0, 0.0), (100.0, 100.0), (0.0, 100.0)],
    StrokeEndKind::Stealth => &[(50.0, 0.0), (100.0, 100.0), (50.0, 60.0), (0.0, 100.0)],
    StrokeEndKind::Diamond => &[(50.0, 0.0), (100.0, 50.0), (50.0, 100.0), (0.0, 50.0)],
    StrokeEndKind::Oval => &[
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
    StrokeEndKind::Arrow => &[
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
    StrokeEndKind::None => return None,
  };
  let perpendicular = (-outward.1, outward.0);
  Some(
    points
      .iter()
      .map(|&(x, y)| {
        let across = (x / 100.0 - 0.5) * width;
        let back = (y / 100.0 - if centered { 0.5 } else { 0.0 }) * length;
        Point {
          x: Pt(endpoint.0 - outward.0 * back + perpendicular.0 * across),
          y: Pt(endpoint.1 - outward.1 * back + perpendicular.1 * across),
        }
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::sdk::SdkType;

  #[test]
  fn actual_outline_merge_preserves_unmodified_theme_fields() {
    let themed = a::Outline::from_bytes(
      br##"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
          w="25400" cap="flat" cmpd="dbl" algn="in">
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:prstDash val="dash"/><a:miter lim="400000"/>
        <a:headEnd type="triangle" w="lg" len="lg"/>
        <a:tailEnd type="diamond" w="med" len="sm"/>
      </a:ln>"##,
    )
    .expect("theme outline");
    let direct = a::Outline::from_bytes(
      br##"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" cap="rnd">
        <a:solidFill><a:srgbClr val="112233"/></a:solidFill>
        <a:headEnd w="sm"/>
      </a:ln>"##,
    )
    .expect("direct outline");

    let merged = merge_outlines(Some(&themed), Some(&direct)).expect("actual outline");

    assert_eq!(merged.width.map(i64::from), Some(25_400));
    assert_eq!(merged.cap_type, Some(a::LineCapValues::Round));
    assert_eq!(
      merged.compound_line_type,
      Some(a::CompoundLineValues::Double)
    );
    assert_eq!(merged.alignment, Some(a::PenAlignmentValues::Insert));
    assert!(matches!(
      merged.outline_choice1,
      Some(a::OutlineChoice::SolidFill(ref fill))
        if matches!(fill.solid_fill_choice, Some(a::SolidFillChoice::RgbColorModelHex(_)))
    ));
    assert!(matches!(
      merged.outline_choice2,
      Some(a::OutlineChoice2::PresetDash(ref dash))
        if dash.val == Some(a::PresetLineDashValues::Dash)
    ));
    assert!(matches!(
      merged.outline_choice3,
      Some(a::OutlineChoice3::Miter(_))
    ));
    let head = merged.head_end.expect("merged head end");
    assert_eq!(head.r#type, Some(a::LineEndValues::Triangle));
    assert_eq!(head.width, Some(a::LineEndWidthValues::Small));
    assert_eq!(head.length, Some(a::LineEndLengthValues::Large));
    let tail = merged.tail_end.expect("inherited tail end");
    assert_eq!(tail.r#type, Some(a::LineEndValues::Diamond));
    assert_eq!(tail.width, Some(a::LineEndWidthValues::Medium));
    assert_eq!(tail.length, Some(a::LineEndLengthValues::Small));
  }

  #[test]
  fn actual_outline_merge_honors_no_fill_and_nested_fill_inheritance() {
    let themed = a::Outline::from_bytes(
      br##"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" w="25400">
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
      </a:ln>"##,
    )
    .expect("theme outline");
    let colorless_direct = a::Outline::from_bytes(
      br#"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:solidFill/></a:ln>"#,
    )
    .expect("colorless direct outline");
    let inherited_color =
      merge_outlines(Some(&themed), Some(&colorless_direct)).expect("actual outline");
    assert!(matches!(
      inherited_color.outline_choice1,
      Some(a::OutlineChoice::SolidFill(ref fill))
        if matches!(fill.solid_fill_choice, Some(a::SolidFillChoice::SchemeColor(_)))
    ));

    let no_fill = a::Outline::from_bytes(
      br#"<a:ln xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:noFill/></a:ln>"#,
    )
    .expect("no-fill direct outline");
    let suppressed = merge_outlines(Some(&themed), Some(&no_fill)).expect("actual outline");
    assert!(matches!(
      suppressed.outline_choice1,
      Some(a::OutlineChoice::NoFill(_))
    ));
    assert_eq!(suppressed.width.map(i64::from), Some(25_400));
  }
  use crate::common::{Color, Fill};

  fn rectangle_path() -> PathItem<'static> {
    PathItem {
      bounds: Rect {
        origin: Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        size: Size {
          width: Pt(20.0),
          height: Pt(20.0),
        },
      },
      commands: vec![
        PathCommand::MoveTo(Point {
          x: Pt(10.0),
          y: Pt(20.0),
        }),
        PathCommand::LineTo(Point {
          x: Pt(30.0),
          y: Pt(20.0),
        }),
        PathCommand::LineTo(Point {
          x: Pt(30.0),
          y: Pt(40.0),
        }),
        PathCommand::LineTo(Point {
          x: Pt(10.0),
          y: Pt(40.0),
        }),
        PathCommand::Close,
      ],
      closed: true,
      fill: Fill::None,
      ..PathItem::default()
    }
  }

  fn visible_stroke(width: f32) -> Stroke<'static> {
    Stroke {
      width: Pt(width),
      color: Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      cap: Some(StrokeCap::Flat),
      join: Some(StrokeJoin::Miter { limit: Some(10.0) }),
      ..Stroke::default()
    }
  }

  fn assert_rect_close(actual: Rect, expected: Rect) {
    assert!((actual.origin.x.0 - expected.origin.x.0).abs() < 0.001);
    assert!((actual.origin.y.0 - expected.origin.y.0).abs() < 0.001);
    assert!((actual.size.width.0 - expected.size.width.0).abs() < 0.001);
    assert!((actual.size.height.0 - expected.size.height.0).abs() < 0.001);
  }

  #[test]
  fn centered_path_stroke_bounds_use_the_widened_geometry() {
    let bounds =
      path_stroke_bounds(&rectangle_path(), &visible_stroke(2.0)).expect("centered stroke bounds");

    assert_rect_close(
      bounds,
      Rect {
        origin: Point {
          x: Pt(9.0),
          y: Pt(19.0),
        },
        size: Size {
          width: Pt(22.0),
          height: Pt(22.0),
        },
      },
    );
  }

  #[test]
  fn square_cap_is_part_of_open_path_stroke_bounds() {
    let path = PathItem {
      bounds: Rect {
        origin: Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        size: Size {
          width: Pt(20.0),
          height: Pt(0.0),
        },
      },
      commands: vec![
        PathCommand::MoveTo(Point {
          x: Pt(10.0),
          y: Pt(20.0),
        }),
        PathCommand::LineTo(Point {
          x: Pt(30.0),
          y: Pt(20.0),
        }),
      ],
      closed: false,
      fill: Fill::None,
      ..PathItem::default()
    };
    let mut stroke = visible_stroke(4.0);
    stroke.cap = Some(StrokeCap::Square);

    let bounds = path_stroke_bounds(&path, &stroke).expect("square-cap stroke bounds");

    assert_rect_close(
      bounds,
      Rect {
        origin: Point {
          x: Pt(8.0),
          y: Pt(18.0),
        },
        size: Size {
          width: Pt(24.0),
          height: Pt(4.0),
        },
      },
    );
  }

  #[test]
  fn inset_and_transparent_strokes_do_not_expand_effect_source_bounds() {
    let path = rectangle_path();
    let mut inset = visible_stroke(4.0);
    inset.alignment = Some(StrokeAlignment::Inside);
    assert_rect_close(
      path_stroke_bounds(&path, &inset).expect("inset stroke bounds"),
      path.bounds,
    );

    let mut transparent = visible_stroke(4.0);
    transparent.color.a = 0;
    assert!(path_stroke_bounds(&path, &transparent).is_none());
  }

  #[test]
  fn medium_open_arrow_bounds_include_the_stroked_arm_envelope() {
    let path = PathItem {
      bounds: Rect {
        origin: Point::default(),
        size: Size {
          width: Pt(0.0),
          height: Pt(100.0),
        },
      },
      points: vec![
        Point::default(),
        Point {
          x: Pt(0.0),
          y: Pt(100.0),
        },
      ],
      commands: Vec::new(),
      closed: false,
      fill: Fill::None,
      stroke: None,
    };
    let stroke = Stroke {
      width: Pt(2.0),
      color: Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
      },
      tail_end: Some(StrokeEnd {
        kind: StrokeEndKind::Arrow,
        width: StrokeEndSize::Medium,
        length: StrokeEndSize::Medium,
      }),
      ..Stroke::default()
    };

    let bounds = stroke_end_marker_bounds(&path, &stroke).expect("tail marker bounds");

    assert_eq!(bounds.origin.x.0, -4.5);
    assert_eq!(bounds.size.width.0, 9.0);
    assert_eq!(bounds.origin.y.0, 91.0);
    assert_eq!(bounds.size.height.0, 9.0);

    let markers = stroked_open_arrow_markers(&path, &stroke);
    assert_eq!(markers.len(), 1);
    assert!((markers[0].points[1].y.0 - 98.012_35).abs() < 0.000_1);
  }
}
