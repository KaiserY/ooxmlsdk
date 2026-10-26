//! Word's paired-envelope shadow allocation, separate from visible stroke paint.

use kurbo::{BezPath, PathEl, Point, Rect, Shape};

use crate::common::drawingml_image_effects::WordprocessingTextShadow;
use crate::common::{self, PathItem, StrokeJoin};

const PRINTER_POINT: f64 = 72.0 / 600.0;

pub(super) struct Grid {
  pub raster_bounds: common::Rect,
  pub display_bounds: common::Rect,
  pub width: u32,
  pub height: u32,
  pub pixels_per_point: f32,
}

impl Grid {
  pub fn new(
    source: &PathItem<'_>,
    shadow: WordprocessingTextShadow,
    page: common::Rect,
    pixels_per_point: f32,
  ) -> Option<Self> {
    let stroke = source.stroke.as_ref()?;
    if !matches!(stroke.join, Some(StrokeJoin::Miter { limit: Some(0.0) })) {
      return None;
    }
    let path = common::text_warp_projection::common_commands_to_bez_path(&source.commands);
    let path = kurbo::Affine::scale(1.0 / PRINTER_POINT) * path;
    let bounds = zero_limit_bounds(&path, f64::from(stroke.width.0) / PRINTER_POINT)?;
    let direction = f64::from(shadow.direction_degrees).to_radians();
    let distance = f64::from(shadow.distance_px) * 0.75 / PRINTER_POINT;
    let shift = kurbo::Vec2::new(direction.cos(), direction.sin()) * distance;
    let blur = f64::from(shadow.blur_radius_px) * 0.75 / PRINTER_POINT;
    let page = Rect::new(
      f64::from(page.origin.x.0) / PRINTER_POINT,
      f64::from(page.origin.y.0) / PRINTER_POINT,
      f64::from(page.origin.x.0 + page.size.width.0) / PRINTER_POINT,
      f64::from(page.origin.y.0 + page.size.height.0) / PRINTER_POINT,
    );
    Self::from_bounds(bounds + shift, blur, page, pixels_per_point)
  }

  fn from_bounds(bounds: Rect, blur: f64, page: Rect, requested_density: f32) -> Option<Self> {
    if !bounds.is_finite() || !blur.is_finite() || blur < 0.0 {
      return None;
    }
    // The vector owner includes half a printer sample before outward integer
    // rounding. The shadow expands that integer rectangle, then intersects
    // the requested page clip expanded by the same filter support.
    let support = blur.ceil();
    let bounds = Rect::new(
      (bounds.x0 - 0.5).floor() - support,
      (bounds.y0 - 0.5).floor() - support,
      (bounds.x1 + 0.5).ceil() + support,
      (bounds.y1 + 0.5).ceil() + support,
    )
    .intersect(page.inflate(support, support));
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
      return None;
    }
    let raster_bounds =
      layout_rect(kurbo::Affine::scale(PRINTER_POINT).transform_rect_bbox(bounds));
    let pixels_per_point = common::drawingml_shape_raster::fixed_output_effect_pixels_per_point(
      raster_bounds.size.width.0,
      raster_bounds.size.height.0,
      requested_density,
    );
    // Native 3pt-blur and 36pt-font controls distinguish nearest rounding
    // from ceil. Each axis subsequently uses its actual extent ratio.
    let width = (bounds.width() * PRINTER_POINT * f64::from(pixels_per_point)).round() as u32;
    let height = (bounds.height() * PRINTER_POINT * f64::from(pixels_per_point)).round() as u32;
    if width == 0 || height == 0 {
      return None;
    }
    let half_sample = 0.5 / f64::from(pixels_per_point);
    let extent = |dots: f64| ((dots * PRINTER_POINT - half_sample) * 100.0).round() / 100.0;
    let display_bounds = common::Rect {
      origin: raster_bounds.origin,
      size: common::Size {
        width: common::Pt(extent(bounds.width()) as f32),
        height: common::Pt(extent(bounds.height()) as f32),
      },
    };
    Some(Self {
      raster_bounds,
      display_bounds,
      width,
      height,
      pixels_per_point,
    })
  }

  pub fn scales(&self) -> (f32, f32) {
    (
      self.width as f32 / self.raster_bounds.size.width.0,
      self.height as f32 / self.raster_bounds.size.height.0,
    )
  }
}

fn layout_rect(rect: Rect) -> common::Rect {
  common::Rect {
    origin: common::Point {
      x: common::Pt(rect.x0 as f32),
      y: common::Pt(rect.y0 as f32),
    },
    size: common::Size {
      width: common::Pt(rect.width() as f32),
      height: common::Pt(rect.height() as f32),
    },
  }
}

fn zero_limit_bounds(path: &BezPath, width: f64) -> Option<Rect> {
  if path.is_empty() || !width.is_finite() || width <= 0.0 {
    return None;
  }
  let half = width / 2.0;
  let mut bounds = path.bounding_box().inflate(half, half);
  let mut start = Point::ZERO;
  let mut last = start;
  let mut incoming = None;
  let mut outgoing = None;
  for &element in path.elements() {
    let (next, previous, end) = match element {
      PathEl::MoveTo(p) => {
        start = p;
        last = p;
        incoming = None;
        outgoing = None;
        continue;
      }
      PathEl::LineTo(p) => (p, last, p),
      PathEl::QuadTo(c, end) => (c, c, end),
      PathEl::CurveTo(a, b, end) => (
        if a != last {
          a
        } else if b != last {
          b
        } else {
          end
        },
        if b != end {
          b
        } else if a != end {
          a
        } else {
          last
        },
        end,
      ),
      PathEl::ClosePath => {
        if last != start {
          if let Some(previous) = incoming {
            include_join(&mut bounds, half, previous, last, start);
          }
          incoming = Some(last);
        }
        if let (Some(previous), Some(next)) = (incoming, outgoing) {
          include_join(&mut bounds, half, previous, start, next);
        }
        last = start;
        incoming = None;
        outgoing = None;
        continue;
      }
    };
    if next != last {
      outgoing.get_or_insert(next);
      if let Some(previous) = incoming {
        include_join(&mut bounds, half, previous, last, next);
      }
    }
    if previous != end {
      incoming = Some(previous);
    }
    last = end;
  }
  bounds.is_finite().then_some(bounds)
}

fn include_join(bounds: &mut Rect, half: f64, previous: Point, point: Point, next: Point) {
  let mut a = point - previous;
  let mut b = point - next;
  let cross = a.cross(b);
  if cross.abs() <= 0.0005 || a.hypot2() <= f64::EPSILON || b.hypot2() <= f64::EPSILON {
    return;
  }
  if cross < 0.0 {
    std::mem::swap(&mut a, &mut b);
  }
  a /= a.hypot();
  b /= b.hypot();
  let bisector = a + b;
  if bisector.hypot2() <= f64::EPSILON {
    return;
  }
  let bisector = bisector / bisector.hypot();
  // A zero miter limit clips on the plane through the vertex. These two
  // intersections belong only to conservative measurement; visible paint
  // continues to use the original stroke's clipped/bevelled join.
  for (direction, offset) in [
    (a, kurbo::Vec2::new(-a.y, a.x)),
    (b, kurbo::Vec2::new(b.y, -b.x)),
  ] {
    let denominator = direction.dot(bisector);
    if denominator.abs() <= 0.000_596_046_447_753_906_2 {
      continue;
    }
    let edge = point + offset * half;
    let intersection = edge + direction * ((point - edge).dot(bisector) / denominator);
    *bounds = bounds.union_pt(intersection);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zero_limit_join_has_independent_conservative_support() {
    let mut bounds = Rect::new(
      364.96992579961534,
      491.48260498046875,
      2929.822818994522,
      1049.1129801273346,
    );
    include_join(
      &mut bounds,
      5.9083333015441895 / 2.0,
      Point::new(2069.83837890625, 520.6832885742188),
      Point::new(2070.305419921875, 525.343994140625),
      Point::new(2084.002685546875, 652.1309814453125),
    );
    assert!((bounds.y0 + 233.7569874632951).abs() < 1e-8);
    assert!((bounds.y1 - 1284.4449757456446).abs() < 1e-8);
  }

  #[test]
  fn native_shadow_grid_expands_before_page_clip_and_rounds_each_axis() {
    let source = Rect::new(
      60.04587531089783,
      -4278.16247149,
      2989.95416665,
      5290.19234314,
    );
    let page = Rect::new(0.0, 0.0, 5100.0, 6600.0);
    let distance = 22999.0 / 12700.0 / PRINTER_POINT;
    for (blur, degrees, density, dots, dimensions) in [
      (
        0.0,
        117.0_f64,
        96.0,
        [1071.0, 0.0, 4003.0, 5905.0],
        [469, 945],
      ),
      (
        16.733333333333334,
        0.0,
        96.0,
        [1076.0, -17.0, 4042.0, 5908.0],
        [475, 948],
      ),
      (
        25.0,
        117.0,
        48.0,
        [1046.0, -25.0, 4028.0, 5930.0],
        [239, 476],
      ),
    ] {
      let angle = degrees.to_radians();
      let shift = kurbo::Vec2::new(
        1019.0 + distance * angle.cos(),
        600.0 + distance * angle.sin(),
      );
      let grid = Grid::from_bounds(source + shift, blur, page, density / 72.0).unwrap();
      assert_eq!([grid.width, grid.height], dimensions);
      let actual = grid.raster_bounds;
      let expected = layout_rect(
        kurbo::Affine::scale(PRINTER_POINT)
          .transform_rect_bbox(Rect::new(dots[0], dots[1], dots[2], dots[3])),
      );
      assert_eq!(actual, expected);
    }
  }

  #[test]
  fn closed_cubic_bounds_are_translation_invariant() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.curve_to((10.0, 0.0), (10.0, 10.0), (0.0, 10.0));
    path.close_path();
    let shift = kurbo::Vec2::new(1200.0, -800.0);
    let a = zero_limit_bounds(&path, 4.0).unwrap() + shift;
    let b = zero_limit_bounds(&(kurbo::Affine::translate(shift) * path), 4.0).unwrap();
    assert_eq!(a, b);
  }
}
