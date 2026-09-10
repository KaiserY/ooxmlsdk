//! A complete effect drawable has a texture domain separate from the solid.

use super::super::drawingml_shape_raster::PageToRasterMapping;
use super::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct BackdropTexturePlan {
  pub(crate) mapping: PageToRasterMapping,
  target: PageToRasterMapping,
  inverse: [[f32; 3]; 3],
  centre: (f32, f32),
  source_origin_in_target: (f32, f32),
  projected_corners: [(f32, f32); 4],
}

impl BackdropTexturePlan {
  pub(crate) fn new(
    source_bounds: Rect,
    target: PageToRasterMapping,
    model: Static3dSurface,
    projection: Static3dProjection,
    plane_z_pt: f32,
    allocation_pixels_per_point: f32,
  ) -> Option<Self> {
    let scale = target.scale_x;
    if !scale.is_finite()
      || scale <= 0.0
      || target.scale_y != scale
      || !allocation_pixels_per_point.is_finite()
      || allocation_pixels_per_point <= 0.0
      || !source_bounds.size.width.0.is_finite()
      || !source_bounds.size.height.0.is_finite()
      || source_bounds.size.width.0 <= 0.0
      || source_bounds.size.height.0 <= 0.0
      || target.width_px == 0
      || target.height_px == 0
    {
      return None;
    }
    let centre = (
      model.left_px + model.width_px * 0.5,
      model.top_px + model.height_px * 0.5,
    );
    let matrix = plane_homography(
      projection,
      plane_z_pt * scale,
      model.width_px,
      model.height_px,
      scale,
    );
    let inverse = inverse_3x3(matrix)?;
    let source_origin_in_target = (
      source_bounds.origin.x.0 * scale + target.translate_x,
      source_bounds.origin.y.0 * scale + target.translate_y,
    );
    let left = source_origin_in_target.0 - centre.0;
    let top = source_origin_in_target.1 - centre.1;
    let right = left + source_bounds.size.width.0 * scale;
    let bottom = top + source_bounds.size.height.0 * scale;
    let projected_corners =
      [(left, top), (right, top), (right, bottom), (left, bottom)].map(|(x, y)| {
        let p = map_homogeneous(matrix, x, y);
        (p.0 + centre.0, p.1 + centre.1)
      });
    if projected_corners
      .iter()
      .any(|p| !p.0.is_finite() || !p.1.is_finite())
    {
      return None;
    }
    let edge = |a: usize, b: usize| {
      (projected_corners[b].0 - projected_corners[a].0)
        .hypot(projected_corners[b].1 - projected_corners[a].1)
        / scale
        * allocation_pixels_per_point
    };
    let width_px = material_texture::material_texture_extent(edge(0, 1).max(edge(3, 2)))?;
    let height_px = material_texture::material_texture_extent(edge(0, 3).max(edge(1, 2)))?;
    let scale_x = width_px as f32 / source_bounds.size.width.0;
    let scale_y = height_px as f32 / source_bounds.size.height.0;
    let mapping = PageToRasterMapping {
      width_px,
      height_px,
      scale_x,
      scale_y,
      translate_x: -source_bounds.origin.x.0 * scale_x,
      translate_y: -source_bounds.origin.y.0 * scale_y,
      text_hinting: None,
    };
    Some(Self {
      mapping,
      target,
      inverse,
      centre,
      source_origin_in_target,
      projected_corners,
    })
  }

  /// Keep prerequisite source/intermediate pixels even when they lie outside
  /// the final effect drawable. Expansion is by whole texture pixels: the
  /// final crop is a subrectangle, never another resize or a shifted grid.
  pub(crate) fn working_mapping(self, bounds: Rect) -> Option<(PageToRasterMapping, u32, u32)> {
    let m = self.mapping;
    let left = (bounds.origin.x.0 * m.scale_x + m.translate_x)
      .floor()
      .min(0.0);
    let top = (bounds.origin.y.0 * m.scale_y + m.translate_y)
      .floor()
      .min(0.0);
    let right = ((bounds.origin.x.0 + bounds.size.width.0) * m.scale_x + m.translate_x)
      .ceil()
      .max(m.width_px as f32);
    let bottom = ((bounds.origin.y.0 + bounds.size.height.0) * m.scale_y + m.translate_y)
      .ceil()
      .max(m.height_px as f32);
    if [left, top, right, bottom].iter().any(|v| !v.is_finite())
      || right - left >= u32::MAX as f32
      || bottom - top >= u32::MAX as f32
    {
      return None;
    }
    Some((
      PageToRasterMapping {
        width_px: (right - left) as u32,
        height_px: (bottom - top) as u32,
        translate_x: m.translate_x - left,
        translate_y: m.translate_y - top,
        ..m
      },
      -left as u32,
      -top as u32,
    ))
  }

  pub(crate) fn project(self, texture: &RgbaImage) -> Option<RgbaImage> {
    if texture.dimensions() != (self.mapping.width_px, self.mapping.height_px) {
      return None;
    }
    let mut output = RgbaImage::new(self.target.width_px, self.target.height_px);
    let x0 = self
      .projected_corners
      .iter()
      .map(|p| p.0)
      .fold(f32::INFINITY, f32::min)
      .floor()
      .max(0.0) as u32;
    let y0 = self
      .projected_corners
      .iter()
      .map(|p| p.1)
      .fold(f32::INFINITY, f32::min)
      .floor()
      .max(0.0) as u32;
    let x1 = self
      .projected_corners
      .iter()
      .map(|p| p.0)
      .fold(f32::NEG_INFINITY, f32::max)
      .ceil()
      .max(0.0) as u32;
    let y1 = self
      .projected_corners
      .iter()
      .map(|p| p.1)
      .fold(f32::NEG_INFINITY, f32::max)
      .ceil()
      .max(0.0) as u32;
    for y in y0..y1.min(output.height()) {
      for x in x0..x1.min(output.width()) {
        let local = map_homogeneous(
          self.inverse,
          x as f32 + 0.5 - self.centre.0,
          y as f32 + 0.5 - self.centre.1,
        );
        let tx = (local.0 + self.centre.0 - self.source_origin_in_target.0)
          * (self.mapping.scale_x / self.target.scale_x)
          - 0.5;
        let ty = (local.1 + self.centre.1 - self.source_origin_in_target.1)
          * (self.mapping.scale_y / self.target.scale_y)
          - 0.5;
        if let Some(pixel) = sample_bilinear(texture, tx, ty) {
          output.put_pixel(x, y, pixel);
        }
      }
    }
    Some(output)
  }

  /// Sample the source drawable on the scene's final target, without first
  /// projecting it into a working bitmap and resampling that bitmap. The
  /// window is in the same model coordinates as the physical scene; texture
  /// allocation and the PDF placement rectangle do not redefine this window.
  pub(crate) fn project_on_grid(
    self,
    texture: &RgbaImage,
    grid: Static3dTextFinalGrid,
  ) -> Option<RgbaImage> {
    if texture.dimensions() != (self.mapping.width_px, self.mapping.height_px) {
      return None;
    }
    let dimensions = (grid.width_px, grid.height_px);
    let raster = TextSurfaceRasterization::Final(grid);
    let pattern = raster.sample_pattern();
    let corners = self
      .projected_corners
      .map(|point| raster.surface_to_raster(point, dimensions));
    if corners.iter().any(|p| !p.0.is_finite() || !p.1.is_finite()) {
      return None;
    }
    let mut output = RgbaImage::new(dimensions.0, dimensions.1);
    let left = corners
      .iter()
      .map(|p| p.0)
      .fold(f32::INFINITY, f32::min)
      .floor()
      .max(0.0) as u32;
    let top = corners
      .iter()
      .map(|p| p.1)
      .fold(f32::INFINITY, f32::min)
      .floor()
      .max(0.0) as u32;
    let right = corners
      .iter()
      .map(|p| p.0)
      .fold(f32::NEG_INFINITY, f32::max)
      .ceil()
      .max(0.0) as u32;
    let bottom = corners
      .iter()
      .map(|p| p.1)
      .fold(f32::NEG_INFINITY, f32::max)
      .ceil()
      .max(0.0) as u32;
    let triangles = [
      [corners[0], corners[1], corners[2]],
      [corners[0], corners[2], corners[3]],
    ];
    for y in top..bottom.min(dimensions.1) {
      for x in left..right.min(dimensions.0) {
        let mut associated = [0.0_f32; 4];
        for [a, b, c] in triangles {
          let area = text_surface_edge(a, b, c);
          if area.abs() <= 1.0e-6 {
            continue;
          }
          let mut coverage = 0;
          for i in 0..pattern.count() {
            let s = pattern.position(i);
            if text_surface_triangle_covers_sample(a, b, c, (x as f32 + s.0, y as f32 + s.1), area)
            {
              coverage |= 1 << i;
            }
          }
          let Some(centroid) = pattern.centroid(coverage) else {
            continue;
          };
          let point =
            raster.raster_to_surface((x as f32 + centroid.0, y as f32 + centroid.1), dimensions);
          let local = map_homogeneous(
            self.inverse,
            point.0 - self.centre.0,
            point.1 - self.centre.1,
          );
          let tx = (local.0 + self.centre.0 - self.source_origin_in_target.0)
            * (self.mapping.scale_x / self.target.scale_x)
            - 0.5;
          let ty = (local.1 + self.centre.1 - self.source_origin_in_target.1)
            * (self.mapping.scale_y / self.target.scale_y)
            - 0.5;
          // Retain the source's associated bilinear filter. MSAA evaluates
          // this once per primitive/pixel, not once per coverage sample.
          let Some(pixel) = sample_bilinear(texture, tx, ty) else {
            continue;
          };
          let weight = coverage.count_ones() as f32 / pattern.count() as f32;
          let alpha = f32::from(pixel[3]);
          for channel in 0..3 {
            associated[channel] += f32::from(pixel[channel]) * alpha / 255.0 * weight;
          }
          associated[3] += alpha * weight;
        }
        let mut pixel = [0; 4];
        pixel[3] = associated[3].round().clamp(0.0, 255.0) as u8;
        if associated[3] > f32::EPSILON {
          for channel in 0..3 {
            pixel[channel] = (associated[channel] * 255.0 / associated[3])
              .round()
              .clamp(0.0, 255.0) as u8;
          }
        }
        output.put_pixel(x, y, Rgba(pixel));
      }
    }
    Some(output)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn plan(origin: (f32, f32), allocation: f32) -> BackdropTexturePlan {
    let scene = a::Scene3DType {
      camera: Box::new(a::Camera {
        preset: a::PresetCameraValues::OrthographicFront,
        ..Default::default()
      }),
      ..Default::default()
    };
    BackdropTexturePlan::new(
      crate::model::common_rect(origin.0, origin.1, 32.0, 16.0),
      PageToRasterMapping {
        width_px: 100,
        height_px: 100,
        scale_x: 2.0,
        scale_y: 2.0,
        translate_x: -origin.0 * 2.0,
        translate_y: -origin.1 * 2.0,
        text_hinting: None,
      },
      Static3dSurface {
        left_px: 0.0,
        top_px: 0.0,
        width_px: 64.0,
        height_px: 32.0,
      },
      camera_projection(&scene, 0.0),
      0.0,
      allocation,
    )
    .unwrap()
  }

  #[test]
  fn backdrop_texture_mapping_depends_on_source_not_page_translation() {
    let a = plan((0.0, 0.0), 2.0);
    let b = plan((75.0, 120.0), 2.0);
    assert_eq!((a.mapping.width_px, a.mapping.height_px), (64, 32));
    assert_eq!(a.projected_corners, b.projected_corners);
    let texture = RgbaImage::from_fn(64, 32, |x, y| image::Rgba([x as u8, y as u8, 90, 255]));
    let output = a.project(&texture).unwrap();
    assert_eq!(output, b.project(&texture).unwrap());
    assert_eq!(
      image::imageops::crop_imm(&output, 0, 0, 64, 32).to_image(),
      texture
    );
    assert_eq!(output.get_pixel(64, 32).0, [0; 4]);
    assert!(a.project(&RgbaImage::new(63, 32)).is_none());
  }

  #[test]
  fn backdrop_working_padding_keeps_exact_final_lattice() {
    let plan = plan((75.0, 120.0), 2.0);
    let (working, x, y) = plan
      .working_mapping(crate::model::common_rect(71.25, 110.25, 40.0, 30.0))
      .unwrap();
    assert_eq!((x, y), (8, 20));
    assert_eq!(working.scale_x, plan.mapping.scale_x);
    assert_eq!(working.scale_y, plan.mapping.scale_y);
    for (px, py) in [(75.0, 120.0), (79.375, 125.625), (107.0, 136.0)] {
      assert_eq!(
        px * working.scale_x + working.translate_x - x as f32,
        px * plan.mapping.scale_x + plan.mapping.translate_x
      );
      assert_eq!(
        py * working.scale_y + working.translate_y - y as f32,
        py * plan.mapping.scale_y + plan.mapping.translate_y
      );
    }
    assert!(x + plan.mapping.width_px <= working.width_px);
    assert!(y + plan.mapping.height_px <= working.height_px);
  }

  #[test]
  fn backdrop_allocation_changes_texture_not_camera_output() {
    let low = plan((75.0, 120.0), 1.0);
    let high = plan((75.0, 120.0), 4.0);
    assert_eq!((low.mapping.width_px, low.mapping.height_px), (32, 16));
    assert_eq!((high.mapping.width_px, high.mapping.height_px), (128, 64));
    assert_eq!(low.projected_corners, high.projected_corners);
    assert_eq!(low.inverse, high.inverse);
    assert_eq!(low.target.width_px, high.target.width_px);
    assert_eq!(low.target.height_px, high.target.height_px);
  }

  #[test]
  fn backdrop_final_grid_samples_source_once_and_ignores_working_extent() {
    let plan = plan((75.0, 120.0), 2.0);
    let texture = RgbaImage::from_fn(64, 32, |x, y| {
      Rgba([if x % 2 == 0 { 0 } else { 240 }, y as u8, 90, 255])
    });
    for (width, height, left, top) in [
      (32, 16, 0.25, 0.5),
      (21, 13, 1.125, 2.25),
      (40, 20, -3.0, -2.0),
    ] {
      let grid = Static3dTextFinalGrid::new(width, height, left, top, 60.0, 28.0).unwrap();
      let actual = plan.project_on_grid(&texture, grid).unwrap();
      let resized_target = BackdropTexturePlan {
        target: PageToRasterMapping {
          width_px: 700,
          height_px: 430,
          ..plan.target
        },
        ..plan
      };
      assert_eq!(
        actual,
        resized_target.project_on_grid(&texture, grid).unwrap()
      );
      // Away from all primitive boundaries, a pixel is shaded at its centre
      // once. A high-frequency texture distinguishes this from averaging
      // eight separate UV fetches or filtering a projected intermediate.
      let x = width / 4;
      let y = height / 2;
      let sx = left + (x as f32 + 0.5) * 60.0 / width as f32 - 0.5;
      let sy = top + (y as f32 + 0.5) * 28.0 / height as f32 - 0.5;
      assert_eq!(
        *actual.get_pixel(x, y),
        sample_bilinear(&texture, sx, sy).unwrap()
      );
    }
    assert!(
      plan
        .project_on_grid(
          &RgbaImage::new(63, 32),
          Static3dTextFinalGrid::new(32, 16, 0.0, 0.0, 64.0, 32.0).unwrap()
        )
        .is_none()
    );
  }

  #[test]
  fn backdrop_final_grid_shared_diagonal_has_one_owner_per_sample() {
    let plan = plan((0.0, 0.0), 2.0);
    let texture = RgbaImage::from_pixel(64, 32, Rgba([90, 120, 180, 128]));
    // Keep the output inside the texture support so this tests the shared
    // triangle boundary independently of the finite texture's outer edge.
    let grid = Static3dTextFinalGrid::new(31, 15, 1.0, 1.0, 62.0, 30.0).unwrap();
    let output = plan.project_on_grid(&texture, grid).unwrap();
    assert!(output.pixels().all(|p| p.0 == [90, 120, 180, 128]));
  }
}
