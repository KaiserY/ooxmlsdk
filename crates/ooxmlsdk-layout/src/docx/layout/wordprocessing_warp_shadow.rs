//! Word's glyph-path shadow allocation, separate from visible stroke paint.

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
  pub fn flat_shape(
    source: common::Rect,
    shadow: WordprocessingTextShadow,
    host: [f32; 2],
    page: common::Rect,
    output_dpi: f32,
  ) -> Option<Self> {
    if shadow.distance_px != 0.0
      || shadow.scale_x != 1.0
      || shadow.scale_y != 1.0
      || shadow.skew_x_degrees != 0.0
      || shadow.skew_y_degrees != 0.0
      || !host.into_iter().all(f32::is_finite)
      || !output_dpi.is_finite()
      || output_dpi <= 0.0
    {
      return None;
    }
    let local = |rect: common::Rect| {
      let left = f64::from(rect.origin.x.0) - f64::from(host[0]);
      let top = f64::from(rect.origin.y.0) - f64::from(host[1]);
      let coordinate = |point: f64| {
        let dots = point / PRINTER_POINT;
        let half = (dots * 2.0).round() / 2.0;
        // Match the containing formatter's existing f32 half-cell policy.
        // Native48pt descenders land exactly on a half source-printer dot.
        let tolerance = f64::from(f32::EPSILON) * dots.abs().max(1.0) * 8.0;
        if (dots - half).abs() <= tolerance {
          half
        } else {
          dots
        }
      };
      Rect::new(
        coordinate(left),
        coordinate(top),
        coordinate(left + f64::from(rect.size.width.0)),
        coordinate(top + f64::from(rect.size.height.0)),
      )
    };
    let blur = f64::from(shadow.blur_radius_px) * 0.75 * f64::from(shadow.geometry_length_scale)
      / PRINTER_POINT;
    let density = super::wordprocessing_drawing_backdrop_pixels_per_point(
      shadow.blur_radius_px * shadow.raster_length_scale,
      output_dpi / crate::units::POINTS_PER_INCH,
    );
    // Native GDI path/world pairs retain the same half-sample enclosure,
    // integer bitmap dimensions and independent axis fits as envelope paths.
    // The source keeps its continuous host; PDF uses the realized host.
    let mut grid = Self::from_bounds(local(source), blur, local(page), density)?;
    if grid.pixels_per_point == density {
      let divisor = common::drawingml_image_effects::direct2d_balanced_blur_prescale_divisor(
        shadow.blur_radius_px * shadow.raster_length_scale,
      );
      let extent = |points: f32| {
        let printer_dots = (f64::from(points) / PRINTER_POINT).round();
        (printer_dots * f64::from(output_dpi) / (600.0 * f64::from(divisor))).round() as u32
      };
      // Allocate from integral600dpi extents and the selected output rate.
      // Native Screen/Print submit the same glyph path directly to96/200dpi
      // effect surfaces; this is not a later PDF image resample.
      // A rounded f32 density turns the native242.5px case into242.49999px.
      grid.width = extent(grid.raster_bounds.size.width.0);
      grid.height = extent(grid.raster_bounds.size.height.0);
      if grid.width == 0 || grid.height == 0 {
        return None;
      }
    }
    grid.raster_bounds.origin.x.0 += host[0];
    grid.raster_bounds.origin.y.0 += host[1];
    let printer = host.map(|value| {
      (common::wordprocessing_device::source_coordinate(value) as f64 * PRINTER_POINT) as f32
    });
    grid.display_bounds.origin.x.0 += printer[0];
    grid.display_bounds.origin.y.0 += printer[1];
    Some(grid)
  }

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

  fn flat_shadow(font: f32, blur_pt: f32) -> WordprocessingTextShadow {
    let scale = super::super::wordprocessing_text_geometry_render_scale(font);
    let scale =
      common::drawingml_image_effects::word_static_3d_shadow_length_scale(blur_pt / 0.75, scale);
    WordprocessingTextShadow {
      blur_radius_px: blur_pt / 0.75,
      distance_px: 0.0,
      raster_length_scale: scale,
      geometry_length_scale: scale,
      direction_degrees: 0.0,
      scale_x: 1.0,
      scale_y: 1.0,
      skew_x_degrees: 0.0,
      skew_y_degrees: 0.0,
      alignment: (0.0, 0.0),
      color: common::drawingml_image_effects::ResolvedEffectColor {
        color: crate::model::RgbColor { r: 0, g: 0, b: 0 },
        alpha: 110,
      },
    }
  }

  #[test]
  fn flat_shape_grid_preserves_native_local_world_and_printer_host() {
    // Actual GDI point buffers, world transforms and image dimensions.
    // Original four lines plus independently authored short 18/36pt inputs;
    // the short inputs were also observed at both additional host phases.
    for (font, blur, phased, ink, window, dimensions) in [
      (
        36.0,
        2.0,
        false,
        [8.887_506, 13.7578125, 330.69303, 46.875],
        [7.44, 12.36, 332.16, 48.36],
        [902, 100],
      ),
      (
        36.0,
        2.0,
        false,
        [7.446098, 65.188125, 318.09515, 98.14711],
        [6.0, 63.72, 319.56, 99.6],
        [871, 100],
      ),
      (
        36.0,
        2.0,
        false,
        [7.5339894, 116.47781, 329.93976, 149.595],
        [6.12, 115.08, 331.32, 151.08],
        [903, 100],
      ),
      (
        36.0,
        2.0,
        false,
        [7.533988, 167.90813, 289.25647, 200.76164],
        [6.12, 166.44, 290.64, 202.2],
        [790, 99],
      ),
      (
        18.0,
        2.0,
        true,
        [8.043752, 8.894062, 97.38528, 25.4175],
        [7.08, 7.92, 98.4, 26.4],
        [254, 51],
      ),
      (
        36.0,
        2.0,
        true,
        [8.887504, 13.948125, 175.81055, 46.995],
        [7.44, 12.48, 177.24, 48.48],
        [472, 100],
      ),
      (
        18.0,
        0.0,
        false,
        [8.043752, 8.174063, 97.38528, 24.6975],
        [7.92, 8.04, 97.56, 24.84],
        [249, 47],
      ),
      (
        18.0,
        6.0,
        false,
        [8.043752, 10.454062, 97.38528, 26.9775],
        [5.52, 7.92, 99.96, 29.52],
        [262, 60],
      ),
      (
        36.0,
        0.0,
        false,
        [8.887504, 12.628125, 175.81055, 45.675],
        [8.76, 12.48, 175.92, 45.84],
        [464, 93],
      ),
      (
        36.0,
        6.0,
        false,
        [8.887504, 16.468124, 175.81055, 49.515],
        [5.04, 12.6, 179.64, 53.4],
        [243, 57],
      ),
      (
        48.0,
        6.0,
        false,
        [9.450003, 20.1975, 227.77405, 64.26],
        [4.8, 15.48, 232.44, 68.88],
        [316, 74],
      ),
    ] {
      let phases: &[f32] = if phased { &[0.0, 0.06, 0.12] } else { &[0.0] };
      for &phase in phases {
        let host = [39.75, 43.5 + phase];
        let source = layout_rect(Rect::new(
          f64::from(host[0] + ink[0]),
          f64::from(host[1] + ink[1]),
          f64::from(host[0] + ink[2]),
          f64::from(host[1] + ink[3]),
        ));
        let grid = Grid::flat_shape(
          source,
          flat_shadow(font, blur),
          host,
          layout_rect(Rect::new(0.0, 0.0, 595.3, 841.9)),
          200.0,
        )
        .unwrap();
        assert_eq!([grid.width, grid.height], dimensions);
        let actual = grid.raster_bounds;
        for (actual, expected) in [
          (actual.origin.x.0, host[0] + window[0]),
          (actual.origin.y.0, host[1] + window[1]),
          (actual.size.width.0, window[2] - window[0]),
          (actual.size.height.0, window[3] - window[1]),
        ] {
          assert!((actual - expected).abs() < 0.0001);
        }
        assert!((grid.display_bounds.origin.x.0 - 39.72 - window[0]).abs() < 0.0001);
        let printer_y = if phase == 0.12 { 43.68 } else { 43.56 };
        assert!((grid.display_bounds.origin.y.0 - printer_y - window[1]).abs() < 0.0001);
        let (scale_x, scale_y) = grid.scales();
        assert!((scale_x - dimensions[0] as f32 / (window[2] - window[0])).abs() < 0.00001);
        assert!((scale_y - dimensions[1] as f32 / (window[3] - window[1])).abs() < 0.00001);
      }
    }
  }

  #[test]
  fn flat_shape_grid_keeps_native_font_scaled_density_and_stopping_cases() {
    let source = layout_rect(Rect::new(48.0, 56.0, 140.0, 73.0));
    let page = layout_rect(Rect::new(0.0, 0.0, 595.3, 841.9));
    // Native large-radius controls retain 200dpi at18pt and100dpi at36/48pt.
    for (font, density) in [(18.0, 200.0), (36.0, 100.0), (48.0, 100.0)] {
      let grid =
        Grid::flat_shape(source, flat_shadow(font, 6.0), [39.75, 43.5], page, 200.0).unwrap();
      assert!((grid.pixels_per_point * 72.0 - density).abs() < 0.0001);
    }
    let mut shadow = flat_shadow(36.0, 2.0);
    shadow.distance_px = 1.0;
    assert!(Grid::flat_shape(source, shadow, [39.75, 43.5], page, 200.0).is_none());
    shadow.distance_px = 0.0;
    shadow.scale_y = -1.0;
    assert!(Grid::flat_shape(source, shadow, [39.75, 43.5], page, 200.0).is_none());
  }

  #[test]
  fn flat_shape_grid_uses_observed_screen_and_print_source_surfaces() {
    // Same actual native path and two GDI+ worlds, independently captured
    // before the bitmap export. Both profiles retain this printer enclosure.
    let source = layout_rect(Rect::new(8.887_512, 13.757_812_5, 536.137_48, 46.875));
    let page = layout_rect(Rect::new(0.0, 0.0, 595.3, 841.9));
    for (dpi, dimensions, native_world) in [
      (
        96.0,
        [707, 48],
        [0.000_105_004_7, 0.000_104_986_87, -9.921_684, -16.48],
      ),
      (
        200.0,
        [1473, 100],
        [0.000_218_772_17, 0.000_218_722_66, -20.671_345, -34.333_332],
      ),
    ] {
      let grid = Grid::flat_shape(source, flat_shadow(36.0, 2.0), [0.0, 0.0], page, dpi)
        .expect("native fixed-output source");
      assert_eq!([grid.width, grid.height], dimensions);
      let bounds = grid.raster_bounds;
      for (actual, expected) in [
        (bounds.origin.x.0, 7.44),
        (bounds.origin.y.0, 12.36),
        (bounds.size.width.0, 530.16),
        (bounds.size.height.0, 36.0),
      ] {
        assert!((actual - expected).abs() < 0.0001);
      }
      let (x, y) = grid.scales();
      for (actual, expected) in [
        (x / 12700.0, native_world[0]),
        (y / 12700.0, native_world[1]),
        (-bounds.origin.x.0 * x, native_world[2]),
        (-bounds.origin.y.0 * y, native_world[3]),
      ] {
        assert!((actual - expected).abs() < 0.00006);
      }
    }
    for dpi in [f32::NAN, f32::INFINITY, 0.0, -96.0] {
      assert!(Grid::flat_shape(source, flat_shadow(36.0, 2.0), [0.0, 0.0], page, dpi).is_none());
    }
  }

  #[test]
  fn flat_shape_grid_clips_filter_support_to_the_resolved_story() {
    // Last of17 native paths in two independently observed overflow exports.
    // Its filter support intersects the text-frame clip, not the page bottom.
    let source = layout_rect(Rect::new(8.799_62, 835.637_8, 575.319_13, 868.754_96));
    let clip = layout_rect(Rect::new(0.0, 3.6, 595.2, 838.2));
    for (dpi, dimensions) in [(96.0, [759, 7]), (200.0, [1582, 15])] {
      let grid = Grid::flat_shape(source, flat_shadow(36.0, 2.0), [0.0, 0.0], clip, dpi)
        .expect("the admitted last line retains its clipped shadow");
      assert_eq!([grid.width, grid.height], dimensions);
      assert!((grid.raster_bounds.origin.y.0 - 834.24).abs() < 0.0001);
      assert!((grid.raster_bounds.size.height.0 - 5.28).abs() < 0.0001);
    }
  }

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
