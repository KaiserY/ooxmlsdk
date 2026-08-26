use kurbo::{
  Arc, BezPath, Cap as KurboCap, CubicBez, Join as KurboJoin, PathEl, Point, Shape,
  Stroke as KurboStroke, StrokeOpts, Vec2, flatten, offset::offset_cubic, stroke as expand_stroke,
};
use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::{LocationRef, Size},
  outline::{DrawSettings, OutlinePen},
  raw::TableProvider,
};

use super::paint::{PaintGlyph, PaintGlyphFontRun};
use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;

const SYNTHETIC_ITALIC_SHEAR: f32 = 1.0 / 3.0;
const TEXT_WARP_FLATTEN_TOLERANCE_PT: f64 = 0.2;
const GLYPH_STROKE_EXPANSION_TOLERANCE_PT: f64 = 0.02;

#[derive(Clone, Copy, Debug)]
pub(super) struct GlyphOutlinePlacement {
  pub(super) anchor_x_pt: f32,
  pub(super) run_x_offset_pt: f32,
  pub(super) baseline_y_pt: f32,
  pub(super) horizontal_scale: f32,
  pub(super) vertical_scale: f32,
}

#[derive(Clone, Debug, Default)]
pub(super) struct DirectOutlinePath {
  commands: Vec<common::PathCommand>,
}

impl DirectOutlinePath {
  pub(super) fn from_commands(commands: &[common::PathCommand]) -> Self {
    Self {
      commands: commands.to_vec(),
    }
  }

  pub(super) fn commands(&self) -> &[common::PathCommand] {
    &self.commands
  }

  pub(super) fn is_empty(&self) -> bool {
    self.commands.is_empty()
  }

  pub(super) fn paint_bounds(&self) -> Option<common::Rect> {
    if self.commands.is_empty() {
      return None;
    }
    let bounds = common_commands_to_bez_path(&self.commands).bounding_box();
    let values = [bounds.x0, bounds.y0, bounds.x1, bounds.y1];
    let width = bounds.width();
    let height = bounds.height();
    if !values.into_iter().all(f64::is_finite)
      || !width.is_finite()
      || !height.is_finite()
      || width <= f64::EPSILON
      || height <= f64::EPSILON
    {
      return None;
    }
    Some(common::Rect {
      origin: common::Point {
        x: common::Pt(bounds.x0 as f32),
        y: common::Pt(bounds.y0 as f32),
      },
      size: common::Size {
        width: common::Pt(width as f32),
        height: common::Pt(height as f32),
      },
    })
  }

  /// Expands a glyph stroke into the closed fill geometry used by Office when
  /// the stroke itself carries a fill paint.
  ///
  /// DrawingML compound presets divide the total pen width into alternating
  /// line and gap bands. The fractions below are the normative MS-ODRAW
  /// pictures expressed as MS-EMFPLUS compound-line boundary arrays. Office
  /// fixed output emits those boundaries as alternating-winding contours and
  /// fills them with the nonzero rule. Keeping that geometry here also avoids
  /// painting a fake opaque gap over arbitrary page content.
  pub(super) fn expanded_stroke(&self, stroke: &common::Stroke<'static>) -> Result<Self> {
    let source = common_commands_to_bez_path(&self.commands);
    let compound = stroke.compound.unwrap_or(common::StrokeCompound::Single);
    let expanded = if compound == common::StrokeCompound::Single {
      expand_centered_stroke(&source, stroke, f64::from(stroke.width.0))?
    } else if stroke.resolved_dash().is_some() {
      expand_dashed_symmetric_compound_stroke(&source, stroke, compound)?
    } else {
      expand_solid_compound_stroke(&source, stroke, compound)?
    };
    let mut commands = Vec::new();
    append_mapped_outline(&expanded, |point| point, &mut commands)?;
    Ok(Self { commands })
  }
}

fn expand_centered_stroke(
  source: &BezPath,
  stroke: &common::Stroke<'static>,
  width: f64,
) -> Result<BezPath> {
  let style = kurbo_stroke(stroke, width)?;
  Ok(expand_stroke(
    source.iter(),
    &style,
    &StrokeOpts::default(),
    GLYPH_STROKE_EXPANSION_TOLERANCE_PT,
  ))
}

fn expand_dashed_symmetric_compound_stroke(
  source: &BezPath,
  stroke: &common::Stroke<'static>,
  compound: common::StrokeCompound,
) -> Result<BezPath> {
  let width = f64::from(stroke.width.0);
  let widths: &[f64] = match compound {
    common::StrokeCompound::Double => &[1.0, 1.0 / 3.0],
    common::StrokeCompound::Triple => &[1.0, 2.0 / 3.0, 1.0 / 3.0],
    common::StrokeCompound::ThickThin | common::StrokeCompound::ThinThick => {
      return Err(PdfError::DirectWriterUnsupported {
        feature: "dashed asymmetric compound outlined glyph strokes",
      });
    }
    common::StrokeCompound::Single => &[1.0],
  };
  let mut result = BezPath::new();
  for (index, multiplier) in widths.iter().enumerate() {
    let boundary = expand_centered_stroke(source, stroke, width * multiplier)?;
    if index % 2 == 0 {
      result.extend(boundary);
    } else {
      result.extend(boundary.reverse_subpaths());
    }
  }
  Ok(result)
}

fn expand_solid_compound_stroke(
  source: &BezPath,
  stroke: &common::Stroke<'static>,
  compound: common::StrokeCompound,
) -> Result<BezPath> {
  let width = f64::from(stroke.width.0);
  validate_stroke_width(width)?;
  let boundaries: &[f64] = match compound {
    common::StrokeCompound::Single => &[0.5, -0.5],
    common::StrokeCompound::Double => &[0.5, 1.0 / 6.0, -1.0 / 6.0, -0.5],
    common::StrokeCompound::ThickThin => &[0.5, -0.1, -0.3, -0.5],
    common::StrokeCompound::ThinThick => &[0.5, 0.3, 0.1, -0.5],
    common::StrokeCompound::Triple => &[0.5, 1.0 / 3.0, 1.0 / 6.0, -1.0 / 6.0, -1.0 / 3.0, -0.5],
  };
  let mut result = BezPath::new();
  for (index, multiplier) in boundaries.iter().enumerate() {
    let boundary = centered_stroke_boundary(source, stroke, width * multiplier)?;
    if index % 2 == 0 {
      result.extend(boundary);
    } else {
      result.extend(boundary.reverse_subpaths());
    }
  }
  Ok(result)
}

fn centered_stroke_boundary(
  source: &BezPath,
  stroke: &common::Stroke<'static>,
  distance: f64,
) -> Result<BezPath> {
  if !distance.is_finite() || distance.abs() <= f64::EPSILON {
    return Err(PdfError::Writer(
      "compound outlined glyph boundary must have a finite nonzero offset".to_string(),
    ));
  }
  let (join, miter_limit) = kurbo_join(stroke)?;
  let mut boundary = BezPath::new();
  for elements in source.subpaths() {
    let subpath = elements.iter().copied().collect::<BezPath>();
    let area = subpath.area();
    if !area.is_finite() || area.abs() <= f64::EPSILON {
      return Err(PdfError::Writer(format!(
        "compound outlined glyph contour must have a finite nonzero winding area: {area}"
      )));
    }
    // Kurbo's offset convention is positive on the left of a directed path.
    // Positive `distance` here means geometrically outside the contour, so its
    // sign is opposite the winding. Evaluate this per subpath: glyph counters
    // deliberately wind opposite their exterior contours.
    let signed_offset = -area.signum() * distance;
    boundary.extend(offset_closed_subpath(
      &subpath,
      signed_offset,
      join,
      miter_limit,
    )?);
  }
  if boundary.is_empty() {
    return Err(PdfError::Writer(
      "compound outlined glyph path has no closed contour".to_string(),
    ));
  }
  Ok(boundary)
}

#[derive(Clone, Debug)]
struct OffsetSegment {
  source_end: Point,
  start: Point,
  end: Point,
  start_tangent: Vec2,
  end_tangent: Vec2,
  elements: Vec<PathEl>,
}

fn offset_closed_subpath(
  source: &BezPath,
  distance: f64,
  join: KurboJoin,
  miter_limit: f64,
) -> Result<BezPath> {
  let segments = source_segments(source)?;
  if segments.is_empty() {
    return Err(PdfError::Writer(
      "compound outlined glyph contour has no drawable segments".to_string(),
    ));
  }
  let offset_segments = segments
    .iter()
    .map(|segment| offset_segment(segment, distance))
    .collect::<Result<Vec<_>>>()?;
  let mut result = BezPath::new();
  result.move_to(offset_segments[0].start);
  append_offset_segment(&mut result, &offset_segments[0]);
  for index in 1..offset_segments.len() {
    append_offset_join(
      &mut result,
      &offset_segments[index - 1],
      &offset_segments[index],
      distance,
      join,
      miter_limit,
      false,
    );
    append_offset_segment(&mut result, &offset_segments[index]);
  }
  append_offset_join(
    &mut result,
    offset_segments.last().expect("offset contour is non-empty"),
    &offset_segments[0],
    distance,
    join,
    miter_limit,
    true,
  );
  result.close_path();
  Ok(result)
}

#[derive(Clone, Copy, Debug)]
enum SourceSegment {
  Line { start: Point, end: Point },
  Cubic(CubicBez),
}

fn source_segments(source: &BezPath) -> Result<Vec<SourceSegment>> {
  let mut segments = Vec::new();
  let mut first = None;
  let mut current = None;
  let mut closed = false;
  for element in source.iter() {
    match element {
      PathEl::MoveTo(point) => {
        if current.is_some() {
          return Err(PdfError::Writer(
            "compound outlined glyph offset received more than one contour".to_string(),
          ));
        }
        first = Some(point);
        current = Some(point);
      }
      PathEl::LineTo(end) => {
        let start = current.ok_or_else(|| {
          PdfError::Writer("compound outlined glyph contour has no initial move".to_string())
        })?;
        push_line_segment(&mut segments, start, end);
        current = Some(end);
      }
      PathEl::QuadTo(control, end) => {
        let start = current.ok_or_else(|| {
          PdfError::Writer("compound outlined glyph contour has no initial move".to_string())
        })?;
        let control1 = start + (control - start) * (2.0 / 3.0);
        let control2 = end + (control - end) * (2.0 / 3.0);
        push_cubic_segment(&mut segments, CubicBez::new(start, control1, control2, end));
        current = Some(end);
      }
      PathEl::CurveTo(control1, control2, end) => {
        let start = current.ok_or_else(|| {
          PdfError::Writer("compound outlined glyph contour has no initial move".to_string())
        })?;
        push_cubic_segment(&mut segments, CubicBez::new(start, control1, control2, end));
        current = Some(end);
      }
      PathEl::ClosePath => {
        let start = current.ok_or_else(|| {
          PdfError::Writer("compound outlined glyph contour has no initial move".to_string())
        })?;
        let end = first.ok_or_else(|| {
          PdfError::Writer("compound outlined glyph contour has no initial move".to_string())
        })?;
        push_line_segment(&mut segments, start, end);
        current = Some(end);
        closed = true;
      }
    }
  }
  if !closed {
    return Err(PdfError::Writer(
      "compound outlined glyph contour must be closed".to_string(),
    ));
  }
  Ok(segments)
}

fn push_line_segment(segments: &mut Vec<SourceSegment>, start: Point, end: Point) {
  if start.distance_squared(end) > f64::EPSILON {
    segments.push(SourceSegment::Line { start, end });
  }
}

fn push_cubic_segment(segments: &mut Vec<SourceSegment>, cubic: CubicBez) {
  if cubic.p0 != cubic.p1 || cubic.p0 != cubic.p2 || cubic.p0 != cubic.p3 {
    segments.push(SourceSegment::Cubic(cubic));
  }
}

fn offset_segment(segment: &SourceSegment, distance: f64) -> Result<OffsetSegment> {
  match *segment {
    SourceSegment::Line { start, end } => {
      let tangent = end - start;
      let normal = distance * tangent.normalize().turn_90();
      let offset_start = start + normal;
      let offset_end = end + normal;
      validate_offset_point(offset_start)?;
      validate_offset_point(offset_end)?;
      Ok(OffsetSegment {
        source_end: end,
        start: offset_start,
        end: offset_end,
        start_tangent: tangent,
        end_tangent: tangent,
        elements: vec![PathEl::LineTo(offset_end)],
      })
    }
    SourceSegment::Cubic(cubic) => offset_cubic_segment(cubic, distance),
  }
}

fn offset_cubic_segment(cubic: CubicBez, distance: f64) -> Result<OffsetSegment> {
  let (start_tangent, end_tangent) = cubic_tangents(cubic);
  if start_tangent.hypot2() <= f64::EPSILON || end_tangent.hypot2() <= f64::EPSILON {
    return Err(PdfError::Writer(
      "compound outlined glyph cubic has no usable endpoint tangent".to_string(),
    ));
  }
  if cubic_is_collinear(cubic, start_tangent) {
    return offset_segment(
      &SourceSegment::Line {
        start: cubic.p0,
        end: cubic.p3,
      },
      distance,
    );
  }
  let mut path = BezPath::new();
  offset_cubic(
    cubic,
    distance,
    GLYPH_STROKE_EXPANSION_TOLERANCE_PT,
    &mut path,
  );
  let mut elements = path.iter();
  let Some(PathEl::MoveTo(start)) = elements.next() else {
    return Err(PdfError::Writer(
      "Kurbo cubic offset did not produce an initial point".to_string(),
    ));
  };
  let elements = elements.collect::<Vec<_>>();
  let end = elements
    .last()
    .and_then(|element| element.end_point())
    .ok_or_else(|| PdfError::Writer("Kurbo cubic offset produced no curve".to_string()))?;
  validate_offset_point(start)?;
  validate_offset_point(end)?;
  for element in &elements {
    validate_path_element(*element)?;
  }
  Ok(OffsetSegment {
    source_end: cubic.p3,
    start,
    end,
    start_tangent,
    end_tangent,
    elements,
  })
}

fn cubic_tangents(cubic: CubicBez) -> (Vec2, Vec2) {
  const TANGENT_EPSILON_SQUARED: f64 = 1e-12;
  let start = [
    cubic.p1 - cubic.p0,
    cubic.p2 - cubic.p0,
    cubic.p3 - cubic.p0,
  ]
  .into_iter()
  .find(|tangent| tangent.hypot2() > TANGENT_EPSILON_SQUARED)
  .unwrap_or(Vec2::ZERO);
  let end = [
    cubic.p3 - cubic.p2,
    cubic.p3 - cubic.p1,
    cubic.p3 - cubic.p0,
  ]
  .into_iter()
  .find(|tangent| tangent.hypot2() > TANGENT_EPSILON_SQUARED)
  .unwrap_or(Vec2::ZERO);
  (start, end)
}

fn cubic_is_collinear(cubic: CubicBez, tangent: Vec2) -> bool {
  if tangent.hypot2() <= f64::EPSILON || cubic.p0 == cubic.p3 {
    return false;
  }
  let tolerance = GLYPH_STROKE_EXPANSION_TOLERANCE_PT * tangent.hypot();
  (cubic.p1 - cubic.p0).cross(tangent).abs() <= tolerance
    && (cubic.p2 - cubic.p0).cross(tangent).abs() <= tolerance
    && (cubic.p3 - cubic.p0).cross(tangent).abs() <= tolerance
}

fn append_offset_segment(path: &mut BezPath, segment: &OffsetSegment) {
  path.extend(segment.elements.iter().copied());
}

fn append_offset_join(
  path: &mut BezPath,
  previous: &OffsetSegment,
  next: &OffsetSegment,
  distance: f64,
  join: KurboJoin,
  miter_limit: f64,
  closing: bool,
) {
  let cross = previous.end_tangent.cross(next.start_tangent);
  let tangent_scale = previous.end_tangent.hypot() * next.start_tangent.hypot();
  if tangent_scale <= f64::EPSILON
    || (cross.abs() <= tangent_scale * 1e-12 && previous.end_tangent.dot(next.start_tangent) > 0.0)
  {
    line_to_if_distinct(path, next.start);
    return;
  }

  let intersection = offset_line_intersection(previous, next);
  let outside_corner = cross * distance < 0.0;
  if !outside_corner {
    if let Some(intersection) = intersection {
      append_intersection(path, previous, intersection, closing);
    } else {
      line_to_if_distinct(path, next.start);
    }
    return;
  }

  match join {
    KurboJoin::Bevel => line_to_if_distinct(path, next.start),
    KurboJoin::Miter => {
      if let Some(intersection) = intersection
        && miter_limit > 0.0
        && intersection.distance(previous.source_end) <= distance.abs() * miter_limit
      {
        append_intersection(path, previous, intersection, closing);
      } else {
        line_to_if_distinct(path, next.start);
      }
    }
    KurboJoin::Round => append_round_offset_join(path, previous, next, distance),
  }
}

fn append_intersection(
  path: &mut BezPath,
  previous: &OffsetSegment,
  intersection: Point,
  closing: bool,
) {
  let trimmed_line = match path.elements_mut().last_mut() {
    Some(PathEl::LineTo(end)) if end.distance_squared(previous.end) <= 1e-18 => {
      *end = intersection;
      true
    }
    _ => false,
  };
  if !trimmed_line {
    line_to_if_distinct(path, intersection);
  }
  if closing && let Some(PathEl::MoveTo(start)) = path.elements_mut().first_mut() {
    *start = intersection;
  }
}

fn offset_line_intersection(previous: &OffsetSegment, next: &OffsetSegment) -> Option<Point> {
  let denominator = previous.end_tangent.cross(next.start_tangent);
  let scale = previous.end_tangent.hypot() * next.start_tangent.hypot();
  if !denominator.is_finite() || denominator.abs() <= scale * 1e-12 {
    return None;
  }
  let parameter = (next.start - previous.end).cross(next.start_tangent) / denominator;
  let intersection = previous.end + parameter * previous.end_tangent;
  (intersection.x.is_finite() && intersection.y.is_finite()).then_some(intersection)
}

fn append_round_offset_join(
  path: &mut BezPath,
  previous: &OffsetSegment,
  next: &OffsetSegment,
  distance: f64,
) {
  let radius = distance.abs();
  let start_vector = previous.end - previous.source_end;
  let sweep = previous
    .end_tangent
    .cross(next.start_tangent)
    .atan2(previous.end_tangent.dot(next.start_tangent));
  if radius <= f64::EPSILON || sweep.abs() <= f64::EPSILON {
    line_to_if_distinct(path, next.start);
    return;
  }
  let start_angle = start_vector.y.atan2(start_vector.x);
  Arc::new(
    previous.source_end,
    (radius, radius),
    start_angle,
    sweep,
    0.0,
  )
  .to_cubic_beziers(
    GLYPH_STROKE_EXPANSION_TOLERANCE_PT,
    |control1, control2, end| {
      path.curve_to(control1, control2, end);
    },
  );
  // Numerical offset approximation and the analytic circular join can differ
  // by a few ulps at the seam. Preserve the exact next-segment endpoint.
  line_to_if_distinct(path, next.start);
}

fn line_to_if_distinct(path: &mut BezPath, point: Point) {
  let current = path
    .elements()
    .last()
    .and_then(|element| element.end_point());
  if current.is_none_or(|current| current.distance_squared(point) > 1e-18) {
    path.line_to(point);
  }
}

fn validate_path_element(element: PathEl) -> Result<()> {
  match element {
    PathEl::MoveTo(point) | PathEl::LineTo(point) => validate_offset_point(point),
    PathEl::QuadTo(control, end) => {
      validate_offset_point(control)?;
      validate_offset_point(end)
    }
    PathEl::CurveTo(control1, control2, end) => {
      validate_offset_point(control1)?;
      validate_offset_point(control2)?;
      validate_offset_point(end)
    }
    PathEl::ClosePath => Ok(()),
  }
}

fn validate_offset_point(point: Point) -> Result<()> {
  if point.x.is_finite() && point.y.is_finite() {
    Ok(())
  } else {
    Err(PdfError::Writer(
      "compound outlined glyph offset produced a non-finite point".to_string(),
    ))
  }
}

fn kurbo_stroke(stroke: &common::Stroke<'static>, width: f64) -> Result<KurboStroke> {
  validate_stroke_width(width)?;
  let (join, miter_limit) = kurbo_join(stroke)?;
  let cap = match stroke.cap {
    Some(common::StrokeCap::Round) => KurboCap::Round,
    Some(common::StrokeCap::Square) => KurboCap::Square,
    Some(common::StrokeCap::Flat) | None => KurboCap::Butt,
  };
  let dash_offset = f64::from(stroke.dash_offset.0);
  if !dash_offset.is_finite() {
    return Err(PdfError::Writer(
      "outlined glyph dash offset must be finite".to_string(),
    ));
  }
  let mut style = KurboStroke::new(width)
    .with_caps(cap)
    .with_join(join)
    .with_miter_limit(miter_limit);
  if let Some(dash) = stroke.resolved_dash() {
    let dash = dash
      .into_iter()
      .map(|value| f64::from(value.0))
      .collect::<Vec<_>>();
    if dash
      .iter()
      .any(|value| !value.is_finite() || *value <= f64::EPSILON)
    {
      return Err(PdfError::Writer(
        "outlined glyph dash lengths must be finite and positive".to_string(),
      ));
    }
    style = style.with_dashes(dash_offset, dash);
  }
  Ok(style)
}

fn validate_stroke_width(width: f64) -> Result<()> {
  if !width.is_finite() || width <= f64::EPSILON {
    return Err(PdfError::Writer(
      "outlined glyph stroke width must be finite and positive".to_string(),
    ));
  }
  Ok(())
}

fn kurbo_join(stroke: &common::Stroke<'static>) -> Result<(KurboJoin, f64)> {
  let (join, miter_limit) = match stroke.join {
    Some(common::StrokeJoin::Round) => (KurboJoin::Round, 10.0),
    Some(common::StrokeJoin::Bevel) => (KurboJoin::Bevel, 10.0),
    Some(common::StrokeJoin::Miter { limit }) => {
      let limit = f64::from(limit.unwrap_or(10.0));
      if !limit.is_finite() || limit < 0.0 {
        return Err(PdfError::Writer(
          "outlined glyph miter limit must be finite and nonnegative".to_string(),
        ));
      }
      (KurboJoin::Miter, limit)
    }
    None => (KurboJoin::Miter, 10.0),
  };
  Ok((join, miter_limit))
}

fn common_commands_to_bez_path(commands: &[common::PathCommand]) -> BezPath {
  let mut path = BezPath::new();
  for command in commands {
    match *command {
      common::PathCommand::MoveTo(point) => path.move_to(common_point(point)),
      common::PathCommand::LineTo(point) => path.line_to(common_point(point)),
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => path.curve_to(
        common_point(control1),
        common_point(control2),
        common_point(end),
      ),
      common::PathCommand::Close => path.close_path(),
    }
  }
  path
}

/// Converts the exact shaped glyph IDs into page-space PDF path geometry.
///
/// The font design coordinate system is Y-up, whereas the shared layout and
/// direct-content coordinate system are Y-down. Synthetic italic is applied
/// in design space before scaling, matching LibreOffice's PDF writer and the
/// former Krilla lowering. A WordArt warp replaces the affine-only transform;
/// the layout model deliberately makes those alternatives mutually exclusive.
pub(super) fn build_glyph_outline_path(
  run: &PaintGlyphFontRun,
  placement: GlyphOutlinePlacement,
  transform: Option<common::Transform>,
  warp: Option<&common::TextWarp>,
) -> Result<DirectOutlinePath> {
  validate_placement(placement)?;
  let face =
    FontRef::from_index(run.font_face.data.as_slice(), run.font_face.index).map_err(|error| {
      PdfError::Writer(format!(
        "outlined glyph font could not be parsed: font_id={} face_index={} error={error}",
        run.font_face.id(),
        run.font_face.index
      ))
    })?;
  let head = face.head().map_err(|error| {
    PdfError::Writer(format!(
      "outlined glyph font has no readable head table: font_id={} error={error}",
      run.font_face.id()
    ))
  })?;
  let units_per_em = f32::from(head.units_per_em());
  if !units_per_em.is_finite() || units_per_em <= f32::EPSILON {
    return Err(PdfError::Writer(format!(
      "outlined glyph font {} has invalid units-per-em {units_per_em}",
      run.font_face.id()
    )));
  }
  let design_scale = run.font_size_pt / units_per_em;
  let warp_boundaries = warp
    .map(|warp| {
      warp
        .boundaries
        .iter()
        .map(|commands| flatten_text_warp_boundary(commands))
        .collect::<Result<Vec<_>>>()
    })
    .transpose()?;
  if warp_boundaries.as_ref().is_some_and(Vec::is_empty) {
    return Err(PdfError::Writer(
      "outlined glyph text warp has no usable boundary".to_string(),
    ));
  }

  let mut commands = Vec::new();
  let mut cursor_x_pt =
    placement.anchor_x_pt + placement.run_x_offset_pt * placement.horizontal_scale;
  let mut cursor_y_pt = placement.baseline_y_pt;
  for glyph in &run.glyphs {
    let Some(outline) = face.outline_glyphs().get(GlyphId::new(glyph.glyph_id)) else {
      if glyph_has_visible_bounds(glyph) {
        return Err(PdfError::DirectWriterUnsupported {
          feature: "non-outline color or bitmap glyph painting",
        });
      }
      advance_cursor(&mut cursor_x_pt, &mut cursor_y_pt, glyph, run, placement);
      continue;
    };
    let mut raw = SkrifaGlyphOutline::default();
    outline
      .draw(
        DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
        &mut raw,
      )
      .map_err(|error| {
        PdfError::Writer(format!(
          "font {} glyph {} outline could not be drawn: {error}",
          run.font_face.id(),
          glyph.glyph_id
        ))
      })?;

    let origin_x_pt = cursor_x_pt + glyph.x_offset * run.font_size_pt * placement.horizontal_scale;
    let origin_y_pt = cursor_y_pt - glyph.y_offset * run.font_size_pt * placement.vertical_scale;
    let map = |point: kurbo::Point| {
      let design_x = if run.font_face.synthetic_italic {
        point.x + point.y * f64::from(SYNTHETIC_ITALIC_SHEAR)
      } else {
        point.x
      };
      let point = kurbo::Point::new(
        f64::from(origin_x_pt) + design_x * f64::from(design_scale * placement.horizontal_scale),
        f64::from(origin_y_pt) - point.y * f64::from(design_scale * placement.vertical_scale),
      );
      if let (Some(warp), Some(boundaries)) = (warp, warp_boundaries.as_deref()) {
        text_warp_point(warp, boundaries, point)
      } else if let Some(transform) = transform {
        transform_point(transform, point)
      } else {
        point
      }
    };
    append_mapped_outline(&raw.path, map, &mut commands)?;
    advance_cursor(&mut cursor_x_pt, &mut cursor_y_pt, glyph, run, placement);
  }

  Ok(DirectOutlinePath { commands })
}

fn validate_placement(placement: GlyphOutlinePlacement) -> Result<()> {
  if ![
    placement.anchor_x_pt,
    placement.run_x_offset_pt,
    placement.baseline_y_pt,
    placement.horizontal_scale,
    placement.vertical_scale,
  ]
  .into_iter()
  .all(f32::is_finite)
    || placement.horizontal_scale <= 0.0
    || placement.vertical_scale <= 0.0
  {
    return Err(PdfError::Writer(
      "outlined glyph placement contains an invalid coordinate or scale".to_string(),
    ));
  }
  Ok(())
}

fn glyph_has_visible_bounds(glyph: &PaintGlyph) -> bool {
  glyph
    .bounds_em
    .is_some_and(|bounds| bounds.x_min_em < bounds.x_max_em && bounds.y_min_em < bounds.y_max_em)
}

fn advance_cursor(
  cursor_x_pt: &mut f32,
  cursor_y_pt: &mut f32,
  glyph: &PaintGlyph,
  run: &PaintGlyphFontRun,
  placement: GlyphOutlinePlacement,
) {
  *cursor_x_pt += glyph.x_advance * run.font_size_pt * placement.horizontal_scale;
  *cursor_y_pt -= glyph.y_advance * run.font_size_pt * placement.vertical_scale;
}

#[derive(Default)]
struct SkrifaGlyphOutline {
  path: BezPath,
}

impl OutlinePen for SkrifaGlyphOutline {
  fn move_to(&mut self, x: f32, y: f32) {
    self.path.move_to((f64::from(x), f64::from(y)));
  }

  fn line_to(&mut self, x: f32, y: f32) {
    self.path.line_to((f64::from(x), f64::from(y)));
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    self
      .path
      .quad_to((f64::from(x1), f64::from(y1)), (f64::from(x), f64::from(y)));
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    self.path.curve_to(
      (f64::from(x1), f64::from(y1)),
      (f64::from(x2), f64::from(y2)),
      (f64::from(x), f64::from(y)),
    );
  }

  fn close(&mut self) {
    self.path.close_path();
  }
}

fn append_mapped_outline(
  path: &BezPath,
  map: impl Fn(kurbo::Point) -> kurbo::Point,
  commands: &mut Vec<common::PathCommand>,
) -> Result<()> {
  let mut current = None;
  for element in path.elements() {
    match *element {
      PathEl::MoveTo(point) => {
        let point = checked_point(map(point))?;
        commands.push(common::PathCommand::MoveTo(point));
        current = Some(point);
      }
      PathEl::LineTo(point) => {
        let point = checked_point(map(point))?;
        commands.push(common::PathCommand::LineTo(point));
        current = Some(point);
      }
      PathEl::QuadTo(control, end) => {
        let start = current.ok_or_else(|| {
          PdfError::Writer("outlined glyph quadratic segment has no current point".to_string())
        })?;
        let control = checked_point(map(control))?;
        let end = checked_point(map(end))?;
        let control1 = common::Point {
          x: common::Pt(start.x.0 + (control.x.0 - start.x.0) * (2.0 / 3.0)),
          y: common::Pt(start.y.0 + (control.y.0 - start.y.0) * (2.0 / 3.0)),
        };
        let control2 = common::Point {
          x: common::Pt(end.x.0 + (control.x.0 - end.x.0) * (2.0 / 3.0)),
          y: common::Pt(end.y.0 + (control.y.0 - end.y.0) * (2.0 / 3.0)),
        };
        commands.push(common::PathCommand::CubicTo {
          control1,
          control2,
          end,
        });
        current = Some(end);
      }
      PathEl::CurveTo(control1, control2, end) => {
        let control1 = checked_point(map(control1))?;
        let control2 = checked_point(map(control2))?;
        let end = checked_point(map(end))?;
        commands.push(common::PathCommand::CubicTo {
          control1,
          control2,
          end,
        });
        current = Some(end);
      }
      PathEl::ClosePath => {
        commands.push(common::PathCommand::Close);
        current = None;
      }
    }
  }
  Ok(())
}

fn checked_point(point: kurbo::Point) -> Result<common::Point> {
  let x = point.x as f32;
  let y = point.y as f32;
  if !x.is_finite() || !y.is_finite() {
    return Err(PdfError::Writer(
      "outlined glyph path contains a non-finite coordinate".to_string(),
    ));
  }
  Ok(common::Point {
    x: common::Pt(x),
    y: common::Pt(y),
  })
}

fn transform_point(transform: common::Transform, point: kurbo::Point) -> kurbo::Point {
  kurbo::Point::new(
    f64::from(transform.m11) * point.x
      + f64::from(transform.m21) * point.y
      + f64::from(transform.dx.0),
    f64::from(transform.m12) * point.x
      + f64::from(transform.m22) * point.y
      + f64::from(transform.dy.0),
  )
}

fn flatten_text_warp_boundary(commands: &[common::PathCommand]) -> Result<Vec<kurbo::Point>> {
  let elements = commands.iter().map(|command| match *command {
    common::PathCommand::MoveTo(point) => PathEl::MoveTo(common_point(point)),
    common::PathCommand::LineTo(point) => PathEl::LineTo(common_point(point)),
    common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    } => PathEl::CurveTo(
      common_point(control1),
      common_point(control2),
      common_point(end),
    ),
    common::PathCommand::Close => PathEl::ClosePath,
  });
  let mut points = Vec::new();
  flatten(
    elements,
    TEXT_WARP_FLATTEN_TOLERANCE_PT,
    |element| match element {
      PathEl::MoveTo(point) | PathEl::LineTo(point) => points.push(point),
      PathEl::ClosePath => {}
      PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
        unreachable!("kurbo::flatten only emits line path elements")
      }
    },
  );
  if points.len() < 2
    || points
      .iter()
      .any(|point| !point.x.is_finite() || !point.y.is_finite())
  {
    return Err(PdfError::Writer(
      "outlined glyph text warp has an invalid boundary".to_string(),
    ));
  }
  Ok(points)
}

fn common_point(point: common::Point) -> kurbo::Point {
  kurbo::Point::new(f64::from(point.x.0), f64::from(point.y.0))
}

fn text_warp_point(
  warp: &common::TextWarp,
  boundaries: &[Vec<kurbo::Point>],
  point: kurbo::Point,
) -> kurbo::Point {
  let source = warp.source_bounds;
  let width = f64::from(source.size.width.0).max(f64::EPSILON);
  let height = f64::from(source.size.height.0).max(f64::EPSILON);
  let u = ((point.x - f64::from(source.origin.x.0)) / width).clamp(0.0, 1.0);
  let v = ((point.y - f64::from(source.origin.y.0)) / height).clamp(0.0, 1.0);
  if boundaries.len() >= 2 {
    let grid_position = v * (boundaries.len() - 1) as f64;
    let upper_index = (grid_position.floor() as usize).min(boundaries.len() - 2);
    let local_v = (grid_position - upper_index as f64).clamp(0.0, 1.0);
    let upper = sample_text_warp_boundary(&boundaries[upper_index], u);
    let lower = sample_text_warp_boundary(&boundaries[upper_index + 1], u);
    return upper + (lower - upper) * local_v;
  }

  let center = sample_text_warp_boundary(&boundaries[0], u);
  let before = sample_text_warp_boundary(&boundaries[0], (u - 0.001).max(0.0));
  let after = sample_text_warp_boundary(&boundaries[0], (u + 0.001).min(1.0));
  let tangent = after - before;
  let length = tangent.hypot();
  if length <= f64::EPSILON {
    return center;
  }
  let normal = kurbo::Vec2::new(-tangent.y / length, tangent.x / length);
  center + normal * ((v - 0.5) * height)
}

fn sample_text_warp_boundary(points: &[kurbo::Point], position: f64) -> kurbo::Point {
  let total = points
    .windows(2)
    .map(|segment| segment[0].distance(segment[1]))
    .sum::<f64>();
  if total <= f64::EPSILON {
    return points[0];
  }
  let target = position.clamp(0.0, 1.0) * total;
  let mut traversed = 0.0;
  for segment in points.windows(2) {
    let length = segment[0].distance(segment[1]);
    if traversed + length >= target && length > f64::EPSILON {
      let local = (target - traversed) / length;
      return segment[0] + (segment[1] - segment[0]) * local;
    }
    traversed += length;
  }
  *points.last().expect("text warp boundary is non-empty")
}

#[cfg(test)]
mod tests {
  use kurbo::{Circle, Rect, Shape};

  use super::*;

  fn rectangle_path() -> DirectOutlinePath {
    DirectOutlinePath {
      commands: vec![
        common::PathCommand::MoveTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(0.0),
        }),
        common::PathCommand::LineTo(common::Point {
          x: common::Pt(100.0),
          y: common::Pt(0.0),
        }),
        common::PathCommand::LineTo(common::Point {
          x: common::Pt(100.0),
          y: common::Pt(50.0),
        }),
        common::PathCommand::LineTo(common::Point {
          x: common::Pt(0.0),
          y: common::Pt(50.0),
        }),
        common::PathCommand::Close,
      ],
    }
  }

  fn subpaths(path: &DirectOutlinePath) -> Vec<BezPath> {
    let path = common_commands_to_bez_path(path.commands());
    let mut subpaths = Vec::new();
    let mut current = BezPath::new();
    for element in path {
      if matches!(element, PathEl::MoveTo(_)) && !current.is_empty() {
        subpaths.push(current);
        current = BezPath::new();
      }
      current.push(element);
      if element == PathEl::ClosePath {
        subpaths.push(current);
        current = BezPath::new();
      }
    }
    if !current.is_empty() {
      subpaths.push(current);
    }
    subpaths
  }

  fn assert_boundaries(compound: common::StrokeCompound, expected_left_edges: &[f64]) {
    let stroke = common::Stroke {
      width: common::Pt(6.0),
      compound: Some(compound),
      join: Some(common::StrokeJoin::Miter { limit: Some(10.0) }),
      ..Default::default()
    };
    let expanded = rectangle_path().expanded_stroke(&stroke).unwrap();
    let subpaths = subpaths(&expanded);
    assert_eq!(subpaths.len(), expected_left_edges.len());
    for (index, (subpath, expected)) in subpaths.iter().zip(expected_left_edges).enumerate() {
      let bounds = subpath.bounding_box();
      assert!(
        (bounds.x0 - expected).abs() <= 0.001,
        "boundary {index}: expected left edge {expected}, got {}",
        bounds.x0
      );
      if index > 0 {
        assert!(
          subpaths[index - 1].area().signum() != subpath.area().signum(),
          "boundary winding must alternate at index {index}"
        );
      }
    }
  }

  #[test]
  fn compound_stroke_boundaries_follow_official_total_width_fractions() {
    assert_boundaries(common::StrokeCompound::Double, &[-3.0, -1.0, 1.0, 3.0]);
    assert_boundaries(common::StrokeCompound::ThickThin, &[-3.0, 0.6, 1.8, 3.0]);
    assert_boundaries(common::StrokeCompound::ThinThick, &[-3.0, -1.8, -0.6, 3.0]);
    assert_boundaries(
      common::StrokeCompound::Triple,
      &[-3.0, -2.0, -1.0, 1.0, 2.0, 3.0],
    );
  }

  fn assert_rect_close(actual: Rect, expected: Rect, tolerance: f64) {
    for (name, actual, expected) in [
      ("x0", actual.x0, expected.x0),
      ("y0", actual.y0, expected.y0),
      ("x1", actual.x1, expected.x1),
      ("y1", actual.y1, expected.y1),
    ] {
      assert!(
        (actual - expected).abs() <= tolerance,
        "expected {name}={expected}, got {actual}"
      );
    }
  }

  #[test]
  fn curved_compound_boundaries_are_winding_independent() {
    let source = Circle::new((50.0, 25.0), 20.0).to_path(0.001);
    let reversed = source.reverse_subpaths();
    let stroke = common::Stroke {
      join: Some(common::StrokeJoin::Round),
      ..Default::default()
    };
    for contour in [&source, &reversed] {
      let outward = centered_stroke_boundary(contour, &stroke, 3.0).unwrap();
      let inward = centered_stroke_boundary(contour, &stroke, -2.0).unwrap();
      assert_rect_close(
        outward.bounding_box(),
        Rect::new(27.0, 2.0, 73.0, 48.0),
        0.03,
      );
      assert_rect_close(
        inward.bounding_box(),
        Rect::new(32.0, 7.0, 68.0, 43.0),
        0.03,
      );
      assert_eq!(outward.area().signum(), contour.area().signum());
      assert_eq!(inward.area().signum(), contour.area().signum());
    }
  }

  #[test]
  fn zero_miter_limit_bevels_only_the_outside_boundary() {
    let source = common_commands_to_bez_path(rectangle_path().commands());
    let stroke = common::Stroke {
      join: Some(common::StrokeJoin::Miter { limit: Some(0.0) }),
      ..Default::default()
    };
    let outward = centered_stroke_boundary(&source, &stroke, 3.0).unwrap();
    let inward = centered_stroke_boundary(&source, &stroke, -3.0).unwrap();
    assert_rect_close(
      outward.bounding_box(),
      Rect::new(-3.0, -3.0, 103.0, 53.0),
      0.001,
    );
    assert!(
      !outward
        .iter()
        .filter_map(|element| element.end_point())
        .any(|point| point.distance_squared(Point::new(-3.0, -3.0)) <= 1e-18),
      "zero miter limit must remove the outside miter tip"
    );
    assert_rect_close(
      inward.bounding_box(),
      Rect::new(3.0, 3.0, 97.0, 47.0),
      0.001,
    );
    assert!(
      inward
        .iter()
        .filter_map(|element| element.end_point())
        .any(|point| point.distance_squared(Point::new(3.0, 3.0)) <= 1e-18),
      "inside offset must retain its geometric intersection"
    );
  }

  #[test]
  fn dashed_compound_preserves_symmetric_presets_and_rejects_asymmetric_ambiguity() {
    for compound in [
      common::StrokeCompound::Double,
      common::StrokeCompound::Triple,
    ] {
      let stroke = common::Stroke {
        width: common::Pt(6.0),
        preset_dash: Some(common::StrokeDashPreset::Dash),
        compound: Some(compound),
        join: Some(common::StrokeJoin::Miter { limit: Some(0.0) }),
        ..Default::default()
      };
      assert!(
        !rectangle_path()
          .expanded_stroke(&stroke)
          .unwrap()
          .is_empty()
      );
    }

    for compound in [
      common::StrokeCompound::ThickThin,
      common::StrokeCompound::ThinThick,
    ] {
      let stroke = common::Stroke {
        width: common::Pt(6.0),
        preset_dash: Some(common::StrokeDashPreset::Dash),
        compound: Some(compound),
        ..Default::default()
      };
      assert!(matches!(
        rectangle_path().expanded_stroke(&stroke),
        Err(PdfError::DirectWriterUnsupported {
          feature: "dashed asymmetric compound outlined glyph strokes"
        })
      ));
    }
  }
}
