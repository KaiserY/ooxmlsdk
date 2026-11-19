//! A Word 3-D pie shadow is an unlit texture at the extrusion's bottom.
//!
//! Native source uploads and their drawn D3D resources identify a 512-square
//! texture on a radius-100 circle's 212-square support. The source alpha uses
//! the existing disk-outset/finite-Gaussian glow kernel. Distance/direction
//! controls leave this texture and plane unchanged; a 2-D screen-space shadow
//! would move and blur the wrong surface.

use super::{PageToRasterMapping, Projector, RadialChartStyle, RadialSlice, Vertex, edge};
use crate::common::drawingml_image_effects::{
  GlowBlurKernel, GlowSpreadKernel, GlowSpreadRadiusRounding, ImageEffect, ImageEffectContainer,
  ImageEffectContainerKind, ImageEffectSourceImages, ResolvedEffectColor,
  apply_container_to_padded_image_with_sources,
};
use crate::common::{Fill, ShapeStyleValue};
use crate::pptx::chart::word_pie_shadow::{MODEL_RADIUS, SHADOW_SUPPORT_OUTSET};
use image::{Rgba, RgbaImage};

const TEXTURE_SIZE: u32 = 512;
const MODEL_EXTENT: f64 = 2.0 * (MODEL_RADIUS + SHADOW_SUPPORT_OUTSET);

pub(super) struct ImagePlane {
  vertices: [Vertex; 4],
  image: RgbaImage,
}

#[derive(Clone, Copy, PartialEq)]
struct Shadow {
  radius_px: f32,
  color: ResolvedEffectColor,
}

fn shadow(container: &ImageEffectContainer) -> Result<Option<Shadow>, ()> {
  let mut result = None;
  for effect in &container.effects {
    let next = match effect {
      // Native Word's original/empty-effect controls are pixel-identical:
      // the 3-D pie producer has no inner-shadow surface pass. Retain the
      // outer-shadow plane while ignoring this unsupported native operation.
      ImageEffect::Identity | ImageEffect::InnerShadow { .. } => None,
      ImageEffect::Container(nested) => shadow(nested)?,
      ImageEffect::OuterShadow {
        blur_radius_px,
        raster_length_scale,
        transform,
        color,
        ..
      } if transform.is_identity()
        && *raster_length_scale == 1.0
        && blur_radius_px.is_finite()
        && *blur_radius_px >= 0.0
        // Native colored textures have different RGB and expanded-alpha
        // support. Keep that independent color realization on its old path.
        && [color.color.r, color.color.g, color.color.b] == [0; 3] =>
      {
        Some(Shadow {
          radius_px: *blur_radius_px,
          color: *color,
        })
      }
      _ => return Err(()),
    };
    if let Some(next) = next
      && result.replace(next).is_some()
    {
      return Err(());
    }
  }
  if result.is_some() && container.kind != ImageEffectContainerKind::Sibling {
    return Err(());
  }
  Ok(result)
}

pub(super) fn lower(
  slices: &[RadialSlice],
  style: &RadialChartStyle,
  projector: &Projector,
) -> Result<Option<ImagePlane>, ()> {
  let mut selected = None;
  let mut missing = false;
  for slice in slices {
    let effect = match style
      .point_image_effects
      .get(slice.index)
      .and_then(Option::as_ref)
    {
      Some(container) => shadow(container)?,
      None => None,
    };
    match effect {
      Some(effect) => {
        if selected.is_some_and(|previous| previous != effect) {
          return Err(());
        }
        selected = Some(effect);
      }
      None => missing = true,
    }
  }
  let Some(shadow) = selected else {
    return Ok(None);
  };
  // This shared circle owns a complete, opaque, un-exploded perspective pie.
  // Mixed point effects, transparent casters and pens keep their prior path.
  if missing
    || projector.parallel
    || projector.sine <= f64::EPSILON
    || (slices.iter().map(|slice| slice.sweep).sum::<f32>() - std::f32::consts::TAU).abs() > 0.0001
    || slices.iter().any(|slice| {
      slice.perspective_plane_offset != (0.0, 0.0)
        || style.point_styles.get(slice.index).is_some_and(|paint| {
          matches!(
            paint.fill,
            ShapeStyleValue::NoPaint | ShapeStyleValue::Paint(Fill::None)
          ) || matches!(paint.stroke, ShapeStyleValue::Paint(_))
        })
    })
  {
    return Err(());
  }
  let image = texture(shadow).ok_or(())?;
  let radius = (MODEL_RADIUS + SHADOW_SUPPORT_OUTSET) / MODEL_RADIUS;
  let vertices = [
    ([-radius, -radius], [0.0, 0.0]),
    ([radius, -radius], [1.0, 0.0]),
    ([radius, radius], [1.0, 1.0]),
    ([-radius, radius], [0.0, 1.0]),
  ]
  .map(|(point, uv)| {
    let mut vertex = projector.vertex([point[0], point[1], -projector.thickness], [255.0; 3]);
    let reciprocal_w = vertex.texture[2];
    vertex.texture = [uv[0] * reciprocal_w, uv[1] * reciprocal_w, reciprocal_w];
    vertex
  });
  Ok(Some(ImagePlane { vertices, image }))
}

fn texture(shadow: Shadow) -> Option<RgbaImage> {
  let mut source = tiny_skia::Pixmap::new(TEXTURE_SIZE, TEXTURE_SIZE)?;
  let scale = f64::from(TEXTURE_SIZE) / MODEL_EXTENT;
  let center = TEXTURE_SIZE as f32 * 0.5;
  let path = tiny_skia::PathBuilder::from_circle(center, center, (MODEL_RADIUS * scale) as f32)?;
  let mut paint = tiny_skia::Paint::default();
  paint.set_color_rgba8(255, 255, 255, 255);
  source.fill_path(
    &path,
    &paint,
    tiny_skia::FillRule::Winding,
    tiny_skia::Transform::identity(),
    None,
  );
  let mut image = RgbaImage::from_raw(TEXTURE_SIZE, TEXTURE_SIZE, source.take())?;
  blur_texture(&mut image, shadow);
  Some(image)
}

fn blur_texture(image: &mut RgbaImage, shadow: Shadow) {
  // The parsed radius is in 96-DPI pixels. Native source geometry is in
  // model units, independently of the chart's output DPI and fitted camera.
  let radius_px = shadow.radius_px * 72.0 / 96.0 * (f64::from(TEXTURE_SIZE) / MODEL_EXTENT) as f32;
  let effect = ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: vec![ImageEffect::Glow {
      radius_px,
      raster_length_scale: 1.0,
      bounds_radius_scale: 1.0,
      bounds_radius_offset_px: 0.0,
      spread_ratio: 0.5,
      spread_kernel: GlowSpreadKernel::Disk,
      spread_radius_rounding: GlowSpreadRadiusRounding::Inward,
      blur_kernel: GlowBlurKernel::WordShapeGaussian,
      color: shadow.color,
    }],
  };
  apply_container_to_padded_image_with_sources(
    image,
    &effect,
    0.0,
    0.0,
    TEXTURE_SIZE as f32,
    TEXTURE_SIZE as f32,
    ImageEffectSourceImages::default(),
  );
}

impl ImagePlane {
  pub(super) fn rasterize(&self, mapping: PageToRasterMapping) -> Option<RgbaImage> {
    let mut vertices = self.vertices.map(|mut vertex| {
      vertex.point = [
        vertex.point[0] * f64::from(mapping.scale_x) + f64::from(mapping.translate_x),
        vertex.point[1] * f64::from(mapping.scale_y) + f64::from(mapping.translate_y),
      ];
      vertex
    });
    if edge(vertices[0].point, vertices[1].point, vertices[2].point) < 0.0 {
      vertices.reverse();
    }
    let area = edge(vertices[0].point, vertices[1].point, vertices[2].point);
    if !area.is_finite() || area <= f64::EPSILON {
      return None;
    }
    let edges: [_; 4] = std::array::from_fn(|i| (vertices[i].point, vertices[(i + 1) % 4].point));
    let inclusive = edges.map(|(a, b)| b[1] < a[1] || (b[1] == a[1] && b[0] > a[0]));
    let barycentric = [
      (vertices[1].point, vertices[2].point),
      (vertices[2].point, vertices[0].point),
      (vertices[0].point, vertices[1].point),
    ];
    let low = |axis: usize| {
      vertices
        .iter()
        .map(|v| v.point[axis])
        .fold(f64::INFINITY, f64::min)
        .floor()
        .max(0.0) as u32
    };
    let high = |axis: usize, limit: u32| {
      vertices
        .iter()
        .map(|v| v.point[axis])
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil()
        .clamp(0.0, f64::from(limit)) as u32
    };
    let mut image = RgbaImage::new(mapping.width_px, mapping.height_px);
    for y in low(1)..high(1, mapping.height_px) {
      for x in low(0)..high(0, mapping.width_px) {
        let coverage = crate::common::drawingml_3d::DIRECT3D_STANDARD_8_SAMPLES
          .iter()
          .filter(|&&(sx, sy)| {
            let point = [f64::from(x) + f64::from(sx), f64::from(y) + f64::from(sy)];
            edges.iter().enumerate().all(|(i, &(a, b))| {
              let e = edge(a, b, point);
              e > 0.0 || (e == 0.0 && inclusive[i])
            })
          })
          .count();
        if coverage == 0 {
          continue;
        }
        let center = [f64::from(x) + 0.5, f64::from(y) + 0.5];
        let weights = barycentric.map(|(a, b)| edge(a, b, center) / area);
        let uvw: [f64; 3] =
          std::array::from_fn(|axis| (0..3).map(|i| weights[i] * vertices[i].texture[axis]).sum());
        let color = sample(&self.image, [uvw[0] / uvw[2], uvw[1] / uvw[2]]);
        if color[3] > 0.0 {
          image.put_pixel(
            x,
            y,
            Rgba([
              (color[0] * 255.0 / color[3]).round().clamp(0.0, 255.0) as u8,
              (color[1] * 255.0 / color[3]).round().clamp(0.0, 255.0) as u8,
              (color[2] * 255.0 / color[3]).round().clamp(0.0, 255.0) as u8,
              (color[3] * coverage as f64 / 8.0).round().clamp(0.0, 255.0) as u8,
            ]),
          );
        }
      }
    }
    Some(image)
  }
}

fn sample(image: &RgbaImage, uv: [f64; 2]) -> [f64; 4] {
  let point = [
    uv[0].clamp(0.0, 1.0) * f64::from(image.width()) - 0.5,
    uv[1].clamp(0.0, 1.0) * f64::from(image.height()) - 0.5,
  ];
  let base = point.map(f64::floor);
  let fraction = [point[0] - base[0], point[1] - base[1]];
  let mut color = [0.0; 4];
  for (dy, wy) in [(0.0, 1.0 - fraction[1]), (1.0, fraction[1])] {
    for (dx, wx) in [(0.0, 1.0 - fraction[0]), (1.0, fraction[0])] {
      let x = (base[0] + dx).clamp(0.0, f64::from(image.width() - 1)) as u32;
      let y = (base[1] + dy).clamp(0.0, f64::from(image.height() - 1)) as u32;
      let pixel = image.get_pixel(x, y);
      let alpha = f64::from(pixel[3]);
      for i in 0..3 {
        color[i] += f64::from(pixel[i]) * alpha / 255.0 * wx * wy;
      }
      color[3] += alpha * wx * wy;
    }
  }
  color
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::RgbColor;

  #[test]
  fn shadow_kernel_matches_native_straight_edge_profiles() {
    // Literal A8 values from independent native CPU texture uploads. The
    // native zero-radius caster's straight edge has 55/89 pixel coverage.
    for (radius_pt, expected) in [
      (1.0, [85, 4, 0, 0, 0, 0]),
      (2.0, [87, 67, 22, 2, 0, 0]),
      (40000.0 / 12700.0, [87, 79, 59, 30, 10, 2]),
    ] {
      let mut image = RgbaImage::from_fn(TEXTURE_SIZE, 16, |x, _| {
        Rgba([255, 255, 255, if x <= 497 { 255 } else { 0 }])
      });
      let shadow = Shadow {
        radius_px: radius_pt * 96.0 / 72.0,
        color: ResolvedEffectColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 89,
        },
      };
      blur_texture(&mut image, shadow);
      let actual: [u8; 6] = std::array::from_fn(|i| image.get_pixel(498 + i as u32, 8)[3]);
      assert_eq!(actual, expected);
    }
  }

  #[test]
  fn transparent_texels_do_not_tint_the_shadow_plane() {
    let image = RgbaImage::from_fn(2, 1, |x, _| {
      if x == 0 {
        Rgba([255, 0, 0, 128])
      } else {
        Rgba([0, 0, 255, 0])
      }
    });
    let sampled = sample(&image, [0.5, 0.5]);
    assert_eq!(sampled, [64.0, 0.0, 0.0, 64.0]);
  }
}
