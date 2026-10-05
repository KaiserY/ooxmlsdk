//! Exact envelope sampling shared by text layout and direct PDF outlines.

use kurbo::{BezPath, CubicBez, ParamCurve, ParamCurveArclen, PathEl, PathSeg, Point};

use super::{PathCommand, Point as LayoutPoint, Pt, TextWarp};

const TEXT_WARP_ARCLEN_ACCURACY_PT: f64 = 0.0001;

pub fn common_commands_to_bez_path(commands: &[PathCommand]) -> BezPath {
  let mut path = BezPath::new();
  for command in commands {
    match *command {
      PathCommand::MoveTo(point) => {
        path.move_to(Point::new(f64::from(point.x.0), f64::from(point.y.0)))
      }
      PathCommand::LineTo(point) => {
        path.line_to(Point::new(f64::from(point.x.0), f64::from(point.y.0)))
      }
      PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => path.curve_to(
        Point::new(f64::from(control1.x.0), f64::from(control1.y.0)),
        Point::new(f64::from(control2.x.0), f64::from(control2.y.0)),
        Point::new(f64::from(end.x.0), f64::from(end.y.0)),
      ),
      PathCommand::Close => path.close_path(),
    }
  }
  path
}

pub fn text_warp_source_seams(warp: &TextWarp, boundaries: &[TextWarpBoundary]) -> Vec<f64> {
  let mut seams = Vec::new();
  for boundary in boundaries {
    if boundary.total <= f64::EPSILON {
      continue;
    }
    for &end in &boundary.ends[..boundary.ends.len() - 1] {
      let fraction = end / boundary.total;
      if fraction > 0.0 && fraction < 1.0 {
        seams.push(
          f64::from(warp.source_bounds.origin.x.0)
            + fraction * f64::from(warp.source_bounds.size.width.0),
        );
      }
    }
  }
  seams.sort_by(f64::total_cmp);
  seams.dedup_by(|a, b| (*a - *b).abs() < TEXT_WARP_ARCLEN_ACCURACY_PT);
  seams
}

/// Office splits the original glyph geometry at the joins of an envelope's
/// boundary segments. Mapping one unsplit segment across that join replaces
/// the piecewise deformation with a different curve. Keep quadratics quadratic
/// until after the warp, and retain untouched outlines byte-for-byte.
pub fn split_outline_at_warp_seams(
  path: &BezPath,
  glyph_to_page: impl Fn(Point) -> Point,
  seams: &[f64],
) -> Option<BezPath> {
  if seams.is_empty() {
    return None;
  }
  let mut result = None;
  let mut current = Point::ZERO;
  let mut start = Point::ZERO;
  for (index, &element) in path.elements().iter().enumerate() {
    let segment = match element {
      PathEl::MoveTo(point) => {
        current = point;
        start = point;
        None
      }
      PathEl::LineTo(end) => Some(PathSeg::Line(kurbo::Line::new(current, end))),
      PathEl::QuadTo(control, end) => {
        Some(PathSeg::Quad(kurbo::QuadBez::new(current, control, end)))
      }
      PathEl::CurveTo(a, b, end) => Some(PathSeg::Cubic(CubicBez::new(current, a, b, end))),
      PathEl::ClosePath => Some(PathSeg::Line(kurbo::Line::new(current, start))),
    };
    let mut cuts = Vec::new();
    if let Some(segment) = segment {
      let x = |point| glyph_to_page(point).x;
      let (coefficients, min, max) = match segment {
        PathSeg::Line(line) => {
          let (a, b) = (x(line.p0), x(line.p1));
          ([a, b - a, 0.0, 0.0], a.min(b), a.max(b))
        }
        PathSeg::Quad(quad) => {
          let (a, b, c) = (x(quad.p0), x(quad.p1), x(quad.p2));
          (
            [a, 2.0 * (b - a), a - 2.0 * b + c, 0.0],
            a.min(b).min(c),
            a.max(b).max(c),
          )
        }
        PathSeg::Cubic(cubic) => {
          let (a, b, c, d) = (x(cubic.p0), x(cubic.p1), x(cubic.p2), x(cubic.p3));
          (
            [
              a,
              3.0 * (b - a),
              3.0 * (a - 2.0 * b + c),
              d - a + 3.0 * (b - c),
            ],
            a.min(b).min(c).min(d),
            a.max(b).max(c).max(d),
          )
        }
      };
      for &seam in seams {
        if seam <= min || seam >= max {
          continue;
        }
        cuts.extend(
          kurbo::common::solve_cubic(
            coefficients[0] - seam,
            coefficients[1],
            coefficients[2],
            coefficients[3],
          )
          .into_iter()
          .filter(|&t| t > 1e-9 && t < 1.0 - 1e-9),
        );
      }
      current = segment.end();
      cuts.sort_by(f64::total_cmp);
      cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
      if !cuts.is_empty() {
        let result =
          result.get_or_insert_with(|| BezPath::from_vec(path.elements()[..index].to_vec()));
        let mut from = 0.0;
        for to in cuts.into_iter().chain(std::iter::once(1.0)) {
          result.push(segment.subsegment(from..to).as_path_el());
          from = to;
        }
        if element == PathEl::ClosePath {
          result.close_path();
        }
        continue;
      }
    }
    if let Some(result) = &mut result {
      result.push(element);
    }
  }
  result
}

/// A DrawingML envelope is sampled by distance along its original curves, not
/// along coarse chords. Keep segment lengths once per run; mapping each glyph
/// control point then needs only a segment lookup and inverse arc length.
pub struct TextWarpBoundary {
  segments: Vec<PathSeg>,
  ends: Vec<f64>,
  pub total: f64,
}

impl TextWarpBoundary {
  pub fn new(commands: &[PathCommand]) -> Result<Self, &'static str> {
    let path = common_commands_to_bez_path(commands);
    if !path.is_finite() {
      return Err("outlined glyph text warp has an invalid boundary");
    }
    let segments: Vec<_> = path.segments().collect();
    if segments.is_empty() {
      return Err("outlined glyph text warp has no boundary segments");
    }
    let accuracy = TEXT_WARP_ARCLEN_ACCURACY_PT / segments.len() as f64;
    let mut total = 0.0;
    let ends = segments
      .iter()
      .map(|segment| {
        total += segment.arclen(accuracy);
        total
      })
      .collect();
    Ok(Self {
      segments,
      ends,
      total,
    })
  }

  pub fn sample(&self, position: f64) -> Point {
    if self.total <= f64::EPSILON || position <= 0.0 {
      return self.segments[0].start();
    }
    if position >= 1.0 {
      return self.segments.last().expect("validated boundary").end();
    }
    let target = position.clamp(0.0, 1.0) * self.total;
    let index = self
      .ends
      .partition_point(|&end| end < target)
      .min(self.segments.len() - 1);
    let start = if index == 0 {
      0.0
    } else {
      self.ends[index - 1]
    };
    let segment = &self.segments[index];
    segment.eval(segment.inv_arclen(target - start, TEXT_WARP_ARCLEN_ACCURACY_PT))
  }
}

pub fn text_warp_point(
  warp: &TextWarp,
  boundaries: &[TextWarpBoundary],
  point: kurbo::Point,
) -> kurbo::Point {
  let source = warp.source_bounds;
  let width = f64::from(source.size.width.0).max(f64::EPSILON);
  let height = f64::from(source.size.height.0).max(f64::EPSILON);
  let u = ((point.x - f64::from(source.origin.x.0)) / width).clamp(0.0, 1.0);
  let v = ((point.y - f64::from(source.origin.y.0)) / height).clamp(0.0, 1.0);
  if boundaries.len() >= 2 {
    let grid_position = if let Some(band) = warp.vml_trim_band {
      let line_height = f64::from(band.source_bottom.0 - band.source_top.0).max(f64::EPSILON);
      band.upper_boundary as f64
        + ((point.y - f64::from(band.source_top.0)) / line_height).clamp(0.0, 1.0)
    } else {
      v * (boundaries.len() - 1) as f64
    };
    let upper_index = (grid_position.floor() as usize).min(boundaries.len() - 2);
    let local_v = (grid_position - upper_index as f64).clamp(0.0, 1.0);
    let upper = boundaries[upper_index].sample(u);
    let lower = boundaries[upper_index + 1].sample(u);
    return upper + (lower - upper) * local_v;
  }

  let center = boundaries[0].sample(u);
  let before = boundaries[0].sample((u - 0.001).max(0.0));
  let after = boundaries[0].sample((u + 0.001).min(1.0));
  let tangent = after - before;
  let length = tangent.hypot();
  if length <= f64::EPSILON {
    return center;
  }
  let normal = kurbo::Vec2::new(-tangent.y / length, tangent.x / length);
  center + normal * ((v - 0.5) * height)
}

#[derive(Default)]
pub struct GlyphOutlinePath {
  pub path: BezPath,
}

impl skrifa::outline::OutlinePen for GlyphOutlinePath {
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

pub fn append_mapped_outline(
  path: &BezPath,
  map: impl Fn(kurbo::Point) -> kurbo::Point,
  commands: &mut Vec<PathCommand>,
) -> Result<(), &'static str> {
  let mut current = None;
  for element in path.elements() {
    match *element {
      PathEl::MoveTo(point) => {
        let point = checked_point(map(point))?;
        commands.push(PathCommand::MoveTo(point));
        current = Some(point);
      }
      PathEl::LineTo(point) => {
        let point = checked_point(map(point))?;
        commands.push(PathCommand::LineTo(point));
        current = Some(point);
      }
      PathEl::QuadTo(control, end) => {
        let start = current.ok_or("outlined glyph quadratic segment has no current point")?;
        let control = checked_point(map(control))?;
        let end = checked_point(map(end))?;
        let control1 = LayoutPoint {
          x: Pt(start.x.0 + (control.x.0 - start.x.0) * (2.0 / 3.0)),
          y: Pt(start.y.0 + (control.y.0 - start.y.0) * (2.0 / 3.0)),
        };
        let control2 = LayoutPoint {
          x: Pt(end.x.0 + (control.x.0 - end.x.0) * (2.0 / 3.0)),
          y: Pt(end.y.0 + (control.y.0 - end.y.0) * (2.0 / 3.0)),
        };
        commands.push(PathCommand::CubicTo {
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
        commands.push(PathCommand::CubicTo {
          control1,
          control2,
          end,
        });
        current = Some(end);
      }
      PathEl::ClosePath => {
        commands.push(PathCommand::Close);
        current = None;
      }
    }
  }
  Ok(())
}

fn checked_point(point: kurbo::Point) -> Result<LayoutPoint, &'static str> {
  let x = point.x as f32;
  let y = point.y as f32;
  if !x.is_finite() || !y.is_finite() {
    return Err("outlined glyph path contains a non-finite coordinate");
  }
  Ok(LayoutPoint { x: Pt(x), y: Pt(y) })
}
