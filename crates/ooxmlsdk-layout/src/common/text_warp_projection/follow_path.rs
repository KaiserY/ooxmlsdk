//! Printer-cell placement on a single DrawingML path.
//!
//! Native GFX IPathingLine controls distinguish this from envelope warping:
//! advance-cell centers follow a distance lookup, and complete glyphs rotate
//! along its derivative. The lookup uses midpoint integration, not an exact
//! inverse arc length. Build it once for the path, independently of glyphs.

use std::sync::Arc;

use kurbo::{Affine, BezPath, CubicBez, PathSeg, Point, Rect, Shape, Vec2};
use skrifa::{FontRef, raw::TableProvider};

use super::super::{PathCommand, TextFollowPath, TextWarp};

const PRINTER_PIXEL_PT: f64 = 72.0 / 600.0;
const LOOKUP_CAPACITY: usize = 50;

#[derive(Clone, Debug, PartialEq)]
struct Polynomial([Vec2; 4]);

impl Polynomial {
  fn cubic(curve: CubicBez) -> Self {
    let a = curve.p1 - curve.p0;
    let b = curve.p2 - curve.p1;
    let c = curve.p3 - curve.p2;
    Self([curve.p0.to_vec2(), a * 3.0, (b - a) * 3.0, c - b * 2.0 + a])
  }

  fn derivative(&self, t: f64) -> Vec2 {
    (self.0[3] * 3.0 * t + self.0[2] * 2.0) * t + self.0[1]
  }

  fn speed(&self, t: f64) -> f64 {
    // Device control points are FLOATs, so their squared derivatives fit in
    // f64. Native CRT controls retain this arithmetic; a compensated hypot
    // can change the final lookup-count branch by one sample.
    self.derivative(t).hypot2().sqrt()
  }

  fn length(&self) -> f64 {
    let mut t = 0.05;
    let mut length = 0.0;
    for _ in 0..10 {
      length += self.speed(t);
      t += 0.1;
    }
    length / 10.0
  }

  fn point(&self, t: f64) -> Point {
    (self.0[0] + (self.0[1] + (self.0[2] + self.0[3] * t) * t) * t).to_point()
  }

  fn is_line(&self) -> bool {
    self.0[2] == Vec2::ZERO && self.0[3] == Vec2::ZERO
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFollowPathSampler {
  curves: Vec<Polynomial>,
  lookup: Vec<(usize, f64)>,
  length: f64,
}

impl TextFollowPathSampler {
  fn new(curves: Vec<Polynomial>) -> Option<Self> {
    let mut lengths = Vec::with_capacity(curves.len());
    for curve in &curves {
      let length = curve.length();
      if !length.is_finite() || length <= f64::EPSILON {
        return None;
      }
      lengths.push(length);
    }
    let length: f64 = lengths.iter().sum();
    if length <= f64::EPSILON || !length.is_finite() {
      return None;
    }
    let step = length / (LOOKUP_CAPACITY - 1) as f64;
    let mut lookup = Vec::with_capacity(LOOKUP_CAPACITY);
    let mut before = 0.0;
    let mut target = 0.0;
    'segments: for (index, (curve, &length)) in curves.iter().zip(&lengths).enumerate() {
      let remaining = target - before;
      if remaining > length {
        before += length;
        continue;
      }
      if curve.is_line() {
        while target - before <= length {
          lookup.push((index, (target - before) / length));
          if lookup.len() == LOOKUP_CAPACITY {
            break 'segments;
          }
          target = lookup.len() as f64 * step;
        }
      } else {
        let speed = curve.speed(remaining * 0.5 / length);
        let mut t = if speed == 0.0 { 0.0 } else { remaining / speed };
        let half_step = step * 0.5 / length;
        loop {
          t = t.min(1.0);
          lookup.push((index, t));
          if lookup.len() == LOOKUP_CAPACITY {
            break 'segments;
          }
          target = lookup.len() as f64 * step;
          if target - before > length {
            break;
          }
          let mut previous = length;
          let mut speed = curve.speed((t + half_step).min(1.0));
          for _ in 1..32 {
            if speed.abs() >= (previous - speed).abs() * 64.0 {
              break;
            }
            let middle = if speed == 0.0 {
              1.0
            } else {
              (t + step / (speed + previous)).min(1.0)
            };
            previous = speed;
            speed = curve.speed(middle);
          }
          t = if speed == 0.0 { 1.0 } else { t + step / speed };
        }
      }
      before += length;
    }
    (lookup.len() >= 2).then_some(Self {
      curves,
      lookup,
      length,
    })
  }

  fn sample(&self, position: f64) -> Option<(Point, Vec2)> {
    let position = position.clamp(0.0, 1.0) * (self.lookup.len() - 1) as f64;
    let index = (position.floor() as usize).min(self.lookup.len() - 2);
    let fraction = position - index as f64;
    let (start_segment, start) = self.lookup[index];
    let (end_segment, end) = self.lookup[index + 1];
    let (segment, t) = if start_segment == end_segment {
      (start_segment, start + fraction * (end - start))
    } else {
      let t = start + fraction * (end + 1.0 - start);
      if t > 1.0 {
        (end_segment, t - 1.0)
      } else {
        (start_segment, t)
      }
    };
    let curve = &self.curves[segment];
    let tangent = curve.derivative(t);
    let length = tangent.hypot();
    (length > f64::EPSILON).then(|| (curve.point(t), tangent / length))
  }
}

impl TextFollowPath {
  /// Construct from already resolved page-space paths. Native producers can
  /// instead retain their device-space coordinates until polynomial creation.
  pub fn from_path(scale: f64, commands: &[PathCommand]) -> Option<Self> {
    let path = Affine::scale(1.0 / PRINTER_PIXEL_PT) * super::common_commands_to_bez_path(commands);
    Self::from_device_path(scale, &path)
  }

  pub(crate) fn from_device_path(scale: f64, path: &BezPath) -> Option<Self> {
    if !path.is_finite() || !scale.is_finite() || scale <= 0.0 {
      return None;
    }
    let curves = path
      .segments()
      .map(|segment| match segment {
        PathSeg::Line(line) => {
          Polynomial([line.p0.to_vec2(), line.p1 - line.p0, Vec2::ZERO, Vec2::ZERO])
        }
        PathSeg::Quad(curve) => Polynomial::cubic(curve.raise()),
        PathSeg::Cubic(curve) => Polynomial::cubic(curve),
      })
      .collect();
    Some(Self {
      scale,
      sampler: Arc::new(TextFollowPathSampler::new(curves)?),
      translation: super::super::Point::default(),
    })
  }

  pub(crate) fn fit_device_path(
    commands: &[PathCommand],
    frame: Rect,
    source_height: f64,
  ) -> Option<(Self, Vec<PathCommand>)> {
    let path = super::common_commands_to_bez_path(commands);
    let bounds = frame
      .union(path.bounding_box())
      .inflate(source_height, source_height);
    let center = frame.center();
    let available = frame.width().max(frame.height()) * 0.5;
    let required = (center.x - bounds.x0)
      .max(bounds.x1 - center.x)
      .max(center.y - bounds.y0)
      .max(bounds.y1 - center.y);
    if !required.is_finite() || required <= 0.0 || available <= 0.0 {
      return None;
    }
    let scale = (available / required).min(1.0);
    // Native fitting updates the IPath before rebuilding its distance table.
    // Transform FLOAT control points, then form double polynomials; merely
    // multiplying an existing lookup preserves a different endpoint branch.
    let transform = Affine::new([
      scale,
      0.0,
      0.0,
      scale,
      center.x - center.x * scale,
      center.y - center.y * scale,
    ]);
    let fitted =
      super::super::drawingml_geometry::transform_commands(commands.iter().copied(), transform);
    let follow = Self::from_device_path(scale, &super::common_commands_to_bez_path(&fitted))?;
    Some((follow, fitted))
  }

  pub(crate) fn translate(&mut self, dx: f32, dy: f32) {
    self.translation.x.0 += dx;
    self.translation.y.0 += dy;
  }
}

#[derive(Clone, Copy)]
pub struct TextFollowPathFontCell {
  reference_offset_pt: f64,
}

impl TextFollowPathFontCell {
  pub fn from_font(font: &FontRef<'_>, font_size_pt: f32) -> Option<Self> {
    let units = f64::from(font.head().ok()?.units_per_em());
    let os2 = font.os2().ok()?;
    if units == 0.0 || !font_size_pt.is_finite() || font_size_pt <= 0.0 {
      return None;
    }
    let ppem = (f64::from(font_size_pt) / PRINTER_PIXEL_PT).round();
    let ascent = (f64::from(os2.us_win_ascent()) * ppem / units).round();
    let descent = (f64::from(os2.us_win_descent()) * ppem / units).round();
    Some(Self {
      reference_offset_pt: ((ascent + descent) * 0.65 - ascent) * PRINTER_PIXEL_PT,
    })
  }
}

pub fn text_follow_path_glyph_transform(
  warp: &TextWarp,
  origin: Point,
  advance_pt: f64,
  cell: TextFollowPathFontCell,
) -> Option<Affine> {
  let fit = warp.follow_path.as_ref()?;
  let scale = fit.scale;
  let center = Point::new(
    origin.x + advance_pt * 0.5,
    origin.y + cell.reference_offset_pt,
  );
  let u = (center.x - f64::from(warp.source_bounds.origin.x.0)) * scale
    / (fit.sampler.length * PRINTER_PIXEL_PT);
  let (point, tangent) = fit.sampler.sample(u)?;
  let point = Point::new(
    point.x * PRINTER_PIXEL_PT + f64::from(fit.translation.x.0),
    point.y * PRINTER_PIXEL_PT + f64::from(fit.translation.y.0),
  );
  let x = tangent * scale;
  let y = Vec2::new(-x.y, x.x);
  let translation = point - center.x * x - center.y * y;
  Some(Affine::new([
    x.x,
    x.y,
    y.x,
    y.y,
    translation.x,
    translation.y,
  ]))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn polynomial(c: [f64; 8]) -> Polynomial {
    Polynomial(std::array::from_fn(|i| Vec2::new(c[2 * i], c[2 * i + 1])))
  }

  #[test]
  fn pathing_midpoint_lookup_matches_native_device_vectors() {
    // Independent grouped and standalone Native factory observations. In
    // particular the first path ends with49 samples, not an invented endpoint.
    for (curves, length, count, first, last) in [
      (
        [
          [
            141.0,
            214.5,
            0.0,
            -80.35743713378906,
            699.7789306640625,
            15.214874267578125,
            -178.7789306640625,
            16.642562866210938,
          ],
          [
            662.0,
            166.0,
            863.2210693359375,
            0.0,
            -163.442138671875,
            65.14256286621094,
            -178.7789306640625,
            -16.642562866210938,
          ],
        ],
        1056.62962944015,
        49,
        0.15818704975373107,
        0.8414837683878018,
      ),
      (
        [
          [
            305.0,
            116.5,
            0.0,
            -60.475181579589844,
            385.4827880859375,
            11.450363159179688,
            -98.4827880859375,
            12.524818420410156,
          ],
          [
            592.0,
            80.0,
            475.5172119140625,
            0.0,
            -90.034423828125,
            49.024818420410156,
            -98.4827880859375,
            -12.524818420410156,
          ],
        ],
        587.6502938794537,
        50,
        0.14762964587592448,
        1.0,
      ),
      (
        [
          [
            -75.0,
            66.5,
            0.0,
            -110.1808090209961,
            386.1544075012207,
            20.861618041992166,
            -98.6544075012207,
            22.81919097900392,
          ],
          [
            212.5,
            -7.382983113757291e-15,
            476.3455810546875,
            -4.4297898682543746e-14,
            -90.191162109375,
            89.31919097900402,
            -98.6544189453125,
            -22.819190979003963,
          ],
        ],
        611.6040232688156,
        50,
        0.11219595184376815,
        0.9987076315572494,
      ),
    ] {
      let sampler =
        TextFollowPathSampler::new(curves.into_iter().map(polynomial).collect()).unwrap();
      assert!((sampler.length - length).abs() < 1e-10);
      assert_eq!(sampler.lookup.len(), count);
      assert!((sampler.lookup[1].1 - first).abs() < 1e-14);
      assert!((sampler.lookup.last().unwrap().1 - last).abs() < 1e-14);
      let (point, tangent) = sampler.sample(0.5).unwrap();
      assert!(point.is_finite());
      assert!((tangent.hypot() - 1.0).abs() < 1e-14);
    }
  }

  #[test]
  fn pathing_linear_segments_keep_uniform_distance() {
    let sampler = TextFollowPathSampler::new(vec![Polynomial([
      Vec2::ZERO,
      Vec2::new(100.0, 0.0),
      Vec2::ZERO,
      Vec2::ZERO,
    ])])
    .unwrap();
    for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
      let (point, tangent) = sampler.sample(u).unwrap();
      assert!((point.x - u * 100.0).abs() < 1e-12);
      assert_eq!(tangent, Vec2::new(1.0, 0.0));
    }
  }

  #[test]
  fn pathing_fit_and_font_cell_match_native_grouped_glyph() {
    use super::super::super::{Point as LayoutPoint, Pt, Size};

    // Native IPathingLine factory and committed paint observations, rather
    // than fitted PDF origins. Retain the FLOAT path before the uniform fit.
    let curves = [
      [
        305.0,
        116.5,
        0.0,
        -60.475181579589844,
        386.1544189453125,
        11.450363159179688,
        -98.6544189453125,
        12.524818420410156,
      ],
      [
        592.5,
        80.0,
        476.3455810546875,
        0.0,
        -90.191162109375,
        49.024818420410156,
        -98.6544189453125,
        -12.524818420410156,
      ],
    ];
    let mut path = BezPath::new();
    for (index, coefficients) in curves.into_iter().enumerate() {
      let c = polynomial(coefficients).0;
      if index == 0 {
        path.move_to(c[0].to_point());
      }
      path.curve_to(
        (c[0] + c[1] / 3.0).to_point(),
        (c[0] + (c[1] * 2.0 + c[2]) / 3.0).to_point(),
        (c[0] + c[1] + c[2] + c[3]).to_point(),
      );
    }
    let commands = super::super::super::drawingml_geometry::bez_path_commands(path);
    let (fit, _) = TextFollowPath::fit_device_path(
      &commands,
      Rect::new(305.0, 80.0, 880.0, 153.0),
      64.23583948612213,
    )
    .unwrap();
    assert!((fit.scale - 0.8173747674391976).abs() < 1e-14);
    assert_eq!(fit.sampler.lookup.len(), 49);
    assert!((fit.sampler.length - 481.13470432337647).abs() < 1e-10);
    let mut metrics = crate::text_metrics::TextMetrics::new();
    let shaped = metrics
      .shape_text(
        "W",
        &crate::model::TextStyle {
          font_family: Some(Arc::from("DejaVu Sans")),
          font_size_pt: 10.0,
          ..Default::default()
        },
      )
      .unwrap();
    let face = &shaped.font_faces[0];
    let font = FontRef::from_index(face.data.as_ref(), face.index).unwrap();
    let cell = TextFollowPathFontCell::from_font(&font, 9.96).unwrap();
    assert!((cell.reference_offset_pt - (-1.674)).abs() < 1e-12);
    let origin = Point::new(305.0 * PRINTER_PIXEL_PT, 157.0 * PRINTER_PIXEL_PT);
    let warp = TextWarp {
      source_bounds: super::super::super::Rect {
        origin: LayoutPoint {
          x: Pt((307.7558608055115 * PRINTER_PIXEL_PT) as f32),
          y: Pt(0.0),
        },
        size: Size {
          width: Pt(68.0),
          height: Pt(8.0),
        },
      },
      paint_bounds: super::super::super::Rect::default(),
      boundaries: Vec::new(),
      word_envelope: false,
      follow_path: Some(fit),
      vml_fit_path: None,
      vml_trim_band: None,
    };
    let matrix =
      text_follow_path_glyph_transform(&warp, origin, 82.0 * PRINTER_PIXEL_PT, cell).unwrap();
    let coefficients = matrix.as_coeffs();
    assert!((coefficients[0] - 0.7940849480864122).abs() < 1e-7);
    assert!((coefficients[1] - (-0.19372817469042147)).abs() < 1e-7);
    let baseline = matrix * origin;
    assert!((baseline.x - 42.486372124230876).abs() < 1e-5);
    assert!((baseline.y - 14.612086733326791).abs() < 1e-5);
  }
}
