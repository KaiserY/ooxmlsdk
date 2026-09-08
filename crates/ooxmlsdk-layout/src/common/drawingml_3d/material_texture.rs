//! Material paint has its own realization, independent of the mesh/effect grid.

use super::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextMaterialTexturePlan {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) page_to_texture: Transform,
  combined_to_texture: Transform,
}

/// Common projected-solid footprint. Paint allocation and physical curve
/// subdivision consume it independently; neither is derived from texture size.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TextSurfaceRealizationPlan {
  bounds: kurbo::Rect,
  width_request: f32,
  height_request: f32,
  pub(crate) curve_tolerance_px: f64,
}

/// Associated RGBA. Do not demultiply into bytes before bilinear filtering:
/// doing so introduces another rounding step at translucent paint boundaries.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextMaterialTexture {
  associated: RgbaImage,
  combined_to_texture: Transform,
}

impl TextSurfaceRealizationPlan {
  pub(super) fn bevel_profile_tolerance_px(self, pixels_per_point: f32) -> f32 {
    // Word's profile producer uses four times its independently realized
    // glyph tolerance, rounded to float in physical inches before flattening.
    // Actual GFX profile inputs/outputs across 21 width/height/boundary controls
    // establish this separately from generic WPF's quarter-device-pixel default.
    let pixels_per_inch = f64::from(pixels_per_point) * 72.0;
    let tolerance_inches = (4.0 * self.curve_tolerance_px / pixels_per_inch) as f32;
    (f64::from(tolerance_inches) * pixels_per_inch) as f32
  }

  pub(crate) fn new(
    geometry: &Static3dTextGeometryPaths,
    projection: Static3dProjection,
    shape: &a::Shape3DType,
    options: Static3dRenderOptions,
    target_pixels_per_point: f32,
  ) -> Option<Self> {
    // The perspective textured-solid route is distinct from the orthographic
    // independent front-coverage route. Do not change the latter's paint grid.
    if projection.parallel || !target_pixels_per_point.is_finite() || target_pixels_per_point <= 0.0
    {
      return None;
    }
    let model = options.model_surface?;
    let scale = options.pixels_per_point;
    if !scale.is_finite() || scale <= 0.0 {
      return None;
    }
    let bounds = geometry.page.source_coverage_path.bounding_box();
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
      return None;
    }
    let contour = shape
      .contour_width
      .map_or(0.0, |value| value.to_emu() as f32 / EMUS_PER_POINT)
      * scale
      * 0.5;
    let z = shape
      .z
      .map_or(0.0, |value| value.to_emu() as f32 / EMUS_PER_POINT)
      * scale;
    let depth = shape
      .extrusion_height
      .map_or(0.0, |value| value.to_emu() as f32 / EMUS_PER_POINT)
      * scale;
    let top = shape
      .bevel_top
      .as_ref()
      .map(|v| {
        resolve_bevel(
          v.width.map(|v| v.to_emu()),
          v.height.map(|v| v.to_emu()),
          v.preset,
          scale,
        )
      })
      .filter(|v| v.has_authored_surface());
    let bottom = shape
      .bevel_bottom
      .as_ref()
      .map(|v| {
        resolve_bevel(
          v.width.map(|v| v.to_emu()),
          v.height.map(|v| v.to_emu()),
          v.preset,
          scale,
        )
      })
      .filter(|v| v.has_authored_surface());
    let front = z - top.map_or(0.0, |v| v.height_px);
    let back = front - depth;
    let cap_front = front
      + top.map_or(0.0, |v| {
        v.height_px * word_text_bevel_terminal_profile(v.preset).height
      });
    let cap_back = back
      - bottom.map_or(0.0, |v| {
        v.height_px * word_text_bevel_terminal_profile(v.preset).height
      });
    let zs = [cap_back.min(back - contour), cap_front.max(front + contour)];
    let xs = [bounds.x0 as f32 - contour, bounds.x1 as f32 + contour];
    let ys = [bounds.y0 as f32 - contour, bounds.y1 as f32 + contour];
    let center = (
      model.left_px + model.width_px * 0.5,
      model.top_px + model.height_px * 0.5,
    );
    let corners: [(f32, f32); 8] = std::array::from_fn(|i| {
      let point = project_local_pixels(
        projection,
        xs[i & 1] - center.0,
        ys[(i >> 1) & 1] - center.1,
        zs[i >> 2],
        model.width_px,
        model.height_px,
        scale,
      );
      (
        point.0 / scale * target_pixels_per_point,
        point.1 / scale * target_pixels_per_point,
      )
    });
    let edge =
      |a: usize, b: usize| (corners[b].0 - corners[a].0).hypot(corners[b].1 - corners[a].1);
    let width_request = [(0, 1), (2, 3), (4, 5), (6, 7)]
      .map(|(a, b)| edge(a, b))
      .into_iter()
      .fold(0.0_f32, f32::max);
    let height_request = [(0, 2), (1, 3), (4, 6), (5, 7)]
      .map(|(a, b)| edge(a, b))
      .into_iter()
      .fold(0.0_f32, f32::max);
    let curve_tolerance_px = 0.125
      * (bounds.width() / f64::from(width_request))
        .min(bounds.height() / f64::from(height_request));
    if !curve_tolerance_px.is_finite() || curve_tolerance_px <= 0.0 {
      return None;
    }
    Some(Self {
      bounds,
      width_request,
      height_request,
      curve_tolerance_px,
    })
  }
}

impl TextMaterialTexturePlan {
  pub(crate) fn from_realization(
    geometry: &Static3dTextGeometryPaths,
    realization: TextSurfaceRealizationPlan,
  ) -> Option<Self> {
    let TextSurfaceRealizationPlan {
      bounds,
      width_request,
      height_request,
      ..
    } = realization;
    let width = material_texture_extent(width_request)?;
    let height = material_texture_extent(height_request)?;
    let sx = width as f32 / bounds.width() as f32;
    let sy = height as f32 / bounds.height() as f32;
    let combined_to_texture = Transform::from_row(
      sx,
      0.0,
      0.0,
      sy,
      -bounds.x0 as f32 * sx,
      -bounds.y0 as f32 * sy,
    );
    let page = &geometry.page;
    Some(Self {
      width,
      height,
      combined_to_texture,
      page_to_texture: Transform::from_row(
        page.page_plane_scale_x * sx,
        0.0,
        0.0,
        page.page_plane_scale_y * sy,
        (page.page_plane_translate_x - bounds.x0 as f32) * sx,
        (page.page_plane_translate_y - bounds.y0 as f32) * sy,
      ),
    })
  }

  pub(crate) fn finish(self, pixmap: Pixmap) -> Option<TextMaterialTexture> {
    if pixmap.width() != self.width || pixmap.height() != self.height {
      return None;
    }
    Some(TextMaterialTexture {
      associated: RgbaImage::from_raw(self.width, self.height, pixmap.take())?,
      combined_to_texture: self.combined_to_texture,
    })
  }
}

/// Office's observed textured-solid allocator rounds the requested projected
/// edge length, then doubles until it fits n + floor(n/4). Its 4096 edge cap
/// bounds the paint allocation independently of the high-density mesh canvas.
/// Twelve exact-config content/font-size controls and the actual allocation
/// branch distinguish this from ordinary next_power_of_two or DPI scaling.
pub(super) fn material_texture_extent(request: f32) -> Option<u32> {
  if !request.is_finite() || request < 0.0 {
    return None;
  }
  let request = request.round();
  let mut extent = 1_u32;
  if request > 1.0 {
    loop {
      extent *= 2;
      if request <= (extent + (extent >> 2)) as f32 || extent == 4096 {
        break;
      }
    }
  }
  Some(extent)
}

impl TextMaterialTexture {
  pub(super) fn sample_associated(&self, point: (f32, f32)) -> [f32; 4] {
    let t = self.combined_to_texture;
    let x = (point.0 * t.sx + t.tx - 0.5).clamp(0.0, (self.associated.width() - 1) as f32);
    let y = (point.1 * t.sy + t.ty - 0.5).clamp(0.0, (self.associated.height() - 1) as f32);
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1).min(self.associated.width() - 1);
    let y1 = (y0 + 1).min(self.associated.height() - 1);
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;
    let mut value = [0.0_f32; 4];
    for (px, py, weight) in [
      (x0, y0, (1.0 - fx) * (1.0 - fy)),
      (x1, y0, fx * (1.0 - fy)),
      (x0, y1, (1.0 - fx) * fy),
      (x1, y1, fx * fy),
    ] {
      for (sum, channel) in value.iter_mut().zip(self.associated.get_pixel(px, py).0) {
        *sum += weight * f32::from(channel);
      }
    }
    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn profile_precision_uses_realized_density_and_rounds_in_physical_units() {
    let glyph_tolerance_inches = 0.001_227_676_602_013_887_1;
    let expected_profile_inches = 0.004_910_706_542_432_308;
    for dpi in [72.0_f32, 96.0, 200.0, 600.0, 1200.0] {
      let plan = TextSurfaceRealizationPlan {
        bounds: kurbo::Rect::new(0.0, 0.0, 100.0, 20.0),
        width_request: 100.0,
        height_request: 20.0,
        curve_tolerance_px: glyph_tolerance_inches * f64::from(dpi),
      };
      let actual = f64::from(plan.bevel_profile_tolerance_px(dpi / 72.0)) / f64::from(dpi);
      assert!((actual - expected_profile_inches).abs() < 1.0e-9);
      let finer = TextSurfaceRealizationPlan {
        curve_tolerance_px: plan.curve_tolerance_px * 0.5,
        ..plan
      };
      assert_eq!(
        finer.bevel_profile_tolerance_px(dpi / 72.0),
        plan.bevel_profile_tolerance_px(dpi / 72.0) * 0.5,
      );
    }
  }

  fn material_vertex() -> TextSurfaceMaterialVertex {
    TextSurfaceMaterialVertex {
      source: TextSurfaceMaterialSource::Combined,
      material_point: (0.5, 0.5),
      diffuse: [1.0; 3],
      specular_incident: [0.0; 3],
      material_opacity: 1.0,
      alpha_texture_fraction: 1.0,
    }
  }

  #[test]
  fn material_texture_filters_rgba_together_and_clamps_edges() {
    let texture = TextMaterialTexture {
      associated: RgbaImage::from_fn(2, 1, |x, _| {
        if x == 0 {
          Rgba([120, 60, 30, 128])
        } else {
          Rgba([0; 4])
        }
      }),
      combined_to_texture: Transform::identity(),
    };
    assert_eq!(
      texture.sample_associated((1.0, 0.5)),
      [60.0, 30.0, 15.0, 64.0]
    );
    assert_eq!(
      texture.sample_associated((-10.0, 2.0)),
      [120.0, 60.0, 30.0, 128.0]
    );
    assert_eq!(texture.sample_associated((10.0, -2.0)), [0.0; 4]);
  }

  #[test]
  fn material_texture_alpha_is_not_replaced_or_multiplied_by_old_coverage() {
    let texture = TextMaterialTexture {
      associated: RgbaImage::from_pixel(1, 1, Rgba([32, 16, 8, 64])),
      combined_to_texture: Transform::identity(),
    };
    let combined = RgbaImage::from_pixel(1, 1, Rgba([255; 4]));
    let mask = TextGeometryAreaMask {
      left: 0,
      top: 0,
      width: 1,
      height: 1,
      alpha: vec![0],
    };
    let images = TextSurfaceMaterialImages {
      combined: &combined,
      surface_material_texture: Some(&texture),
      independent_to_combined: Transform::identity(),
      independent_alpha: Some(&mask),
    };
    let vertex = TextSurfaceMaterialVertex {
      diffuse: [0.5; 3],
      material_opacity: 0.5,
      ..material_vertex()
    };
    for source in [
      TextSurfaceMaterialSource::Combined,
      TextSurfaceMaterialSource::Independent,
    ] {
      for old_alpha in [0.0, 17.0, 128.0, 255.0] {
        assert_eq!(
          resolve_text_surface_material_color(
            images,
            Some(&mask),
            None,
            TextSurfaceMaterialVertex { source, ..vertex },
            old_alpha,
          ),
          Some([127.5, 63.75, 31.875, 32.0])
        );
      }
    }
    let mixed = interpolate_text_surface_material(vertex, material_vertex(), 0.25);
    assert_eq!(mixed.material_opacity, 0.625);
  }

  #[test]
  fn material_texture_specular_is_added_after_premultiplied_diffuse() {
    let vertex = TextSurfaceMaterialVertex {
      diffuse: [0.5; 3],
      specular_incident: [1.0, 0.0, 0.0],
      ..material_vertex()
    };
    for alpha in [0.0, 0.5, 0.999, 1.0, 32.0, 128.0, 255.0] {
      let output = shade_associated_text_material(
        [alpha * 0.5, alpha * 0.5, alpha * 0.5, alpha],
        vertex,
        None,
      );
      if alpha < 1.0 {
        assert_eq!(output, [0.0; 4]);
      } else {
        let expected_red = (alpha * 0.25 + 76.5_f32).min(alpha) * 255.0 / alpha;
        assert!((output[0] - expected_red).abs() < 0.0001);
        assert!((output[1] - 63.75).abs() < 0.0001);
        assert!((output[2] - 63.75).abs() < 0.0001);
        assert_eq!(output[3], alpha);
      }
    }
  }

  #[test]
  fn material_texture_preserves_diffuse_headroom_before_sampling_and_interpolation() {
    let high = text_surface_diffuse_attribute([140.0 / 128.0, 1.5, 3.0]);
    assert_eq!(high, [140.0 / 128.0, 1.5, 255.0 / 128.0]);
    assert_eq!(
      text_surface_diffuse_attribute([-1.0, 0.5, 1.0]),
      [0.0, 0.5, 1.0]
    );
    let low = TextSurfaceMaterialVertex {
      diffuse: [0.5; 3],
      ..material_vertex()
    };
    let high = TextSurfaceMaterialVertex {
      diffuse: high,
      ..material_vertex()
    };
    let middle = interpolate_text_surface_material(low, high, 0.5);
    assert_eq!(middle.diffuse[1], 1.0);
    for alpha in [64.0, 128.0, 255.0] {
      // Dark paint retains the additional illumination. White paint saturates
      // only in the associated pixel shader, without changing coverage alpha.
      for paint in [0.25, 1.0] {
        let texture = [alpha * paint, alpha * paint, alpha * paint, alpha];
        let output = shade_associated_text_material(texture, high, None);
        for (channel, &actual) in output.iter().take(3).enumerate() {
          let expected = (paint * high.diffuse[channel]).min(1.0) * 255.0;
          assert!((actual - expected).abs() < 0.0001);
        }
        assert_eq!(output[3], alpha);
      }
    }
  }

  #[test]
  fn material_texture_allocation_matches_office_controls_and_boundaries() {
    for (request, expected) in [
      (79.069855, 64),
      (105.22825, 128),
      (157.10573, 128),
      (209.18689, 256),
      (22.27358, 32),
      (43.625786, 64),
      (9.759672, 8),
      (12.593183, 16),
      (18.261137, 16),
      (64.49477, 64),
      (85.16392, 128),
      (34.891895, 32),
      (46.31577, 64),
      (0.0, 1),
      (1.49, 1),
      (1.5, 2),
      (2.49, 2),
      (2.5, 4),
      (5.49, 4),
      (5.5, 8),
      (160.49, 128),
      (160.5, 256),
      (100_000.0, 4096),
    ] {
      assert_eq!(
        material_texture_extent(request),
        Some(expected),
        "{request}"
      );
    }
    assert_eq!(material_texture_extent(f32::NAN), None);
    assert_eq!(material_texture_extent(f32::INFINITY), None);
    assert_eq!(material_texture_extent(-1.0), None);
  }
}
