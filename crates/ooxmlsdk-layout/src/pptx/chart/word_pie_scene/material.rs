//! Word's pie material is premultiplied at each source vertex. GFX's VF15
//! carries seven-bit diffuse RGB and separate eight-bit alpha/specular.
//! Independent native gray controls at elevations15/30/90 pin the projective
//! eye, both cap normals and the order of the alpha/color quantization.

use super::{PieCamera, Point3, Projector, VertexLight, vertex_rgb};
use crate::common::drawingml_3d::{
  apply_material_diffuse_fresnel, material_specular_power, material_specularity,
  material_surface_alpha_factor, quantize_gouraud_channel,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

pub(super) fn diffuse(projector: &Projector, point: Point3, light: VertexLight) -> ([f64; 3], f64) {
  if projector.material != Some(a::PresetMaterialTypeValues::Clear) {
    return (light.diffuse, 255.0);
  }
  let Some(camera) = projector.camera else {
    return (light.diffuse, 255.0);
  };
  let mut view = camera.view(point, projector.sine, projector.cosine);
  // The chart producer retains both sides of a translucent material. Its
  // Fresnel input is |N.V|; directional illumination keeps the signed normal.
  if light
    .normal
    .iter()
    .zip(view)
    .map(|(n, v)| f64::from(*n) * v)
    .sum::<f64>()
    < 0.0
  {
    view = view.map(|v| -v);
  }
  let view = view.map(|v| v as f32);
  let alpha = (255.0 * material_surface_alpha_factor(light.normal, view, projector.material))
    .floor()
    .clamp(0.0, 255.0);
  let shade = 0.4 + 0.75 / 3.0_f32.sqrt() * (-light.normal.iter().sum::<f32>()).max(0.0);
  let shade = apply_material_diffuse_fresnel([shade; 3], light.normal, view, projector.material)[0];
  let channel = |c| quantize_gouraud_channel(f32::from(c) / 255.0 * shade * (alpha / 255.0));
  let color = super::RgbColor {
    r: channel(light.source_color.r),
    g: channel(light.source_color.g),
    b: channel(light.source_color.b),
  };
  (vertex_rgb(color), f64::from(alpha))
}

pub(super) fn specular(
  camera: PieCamera,
  point: Point3,
  normal: [f32; 3],
  sine: f64,
  cosine: f64,
  preset: Option<a::PresetMaterialTypeValues>,
) -> f64 {
  let light = -0.75 / 3.0_f64.sqrt();
  let normal = normal.map(f64::from);
  let normal_light = normal.iter().sum::<f64>() * light;
  let reflected = normal.map(|n| 2.0 * normal_light * n - light);
  let view = camera.view(point, sine, cosine);
  let reflected_view = reflected.iter().zip(view).map(|(r, v)| r * v).sum::<f64>() / 0.75;
  (255.0
    * f64::from(material_specularity(preset))
    * 0.75
    * reflected_view
      .max(0.0)
      .powf(f64::from(material_specular_power(preset))))
  .floor()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pptx::chart::{RadialExtrusionProjection, RadialPerspectiveProjection};

  #[test]
  fn hidden_clear_back_faces_do_not_tint_the_visible_cap() {
    use super::super::{Scene, lighting};
    use crate::common::drawingml_shape_raster::PageToRasterMapping;
    let (s, c) = 30.0_f64.to_radians().sin_cos();
    let projector = Projector {
      plane: RadialPerspectiveProjection {
        conic_center: (2.0, 2.0),
        radii: (1.0, 1.0),
        strength: 0.0,
        plane_offset: (0.0, 0.0),
        extrusion: Some(RadialExtrusionProjection {
          denominator: 0.0,
          numerator: 0.24,
        }),
      },
      thickness: 0.24,
      sine: s,
      cosine: c,
      parallel: false,
      camera: Some(PieCamera {
        top_y: 0.0,
        top_distance: 3.0,
      }),
      material: Some(a::PresetMaterialTypeValues::Clear),
    };
    let mapping = PageToRasterMapping {
      width_px: 4,
      height_px: 4,
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x: 0.0,
      translate_y: 0.0,
      text_hinting: None,
    };
    let mut images = Vec::new();
    for include_hidden in [false, true] {
      let mut scene = Scene::default();
      if include_hidden {
        let light = lighting(
          super::super::RgbColor { r: 0, g: 0, b: 255 },
          [0.0, c as f32, s as f32],
        );
        scene.face(
          &projector,
          [[-1.0, -1.0, -0.24], [-1.0, 1.0, -0.24], [1.0, -1.0, -0.24]],
          [light; 3],
          None,
          [-1.0, -1.0, 1.0, 1.0],
        );
      }
      let light = lighting(
        super::super::RgbColor {
          r: 128,
          g: 128,
          b: 128,
        },
        [0.0, -c as f32, -s as f32],
      );
      scene.face(
        &projector,
        [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [-1.0, 1.0, 0.0]],
        [light; 3],
        None,
        [-1.0, -1.0, 1.0, 1.0],
      );
      let image = scene.rasterize(mapping).unwrap();
      assert!(image.pixels().any(|p| p[3] > 0));
      images.push(image);
    }
    assert_eq!(images[0], images[1]);
  }

  #[test]
  fn clear_pie_quantizes_alpha_before_premultiplied_diffuse() {
    // Independent source-gray128 controls and actual GFX projective matrices
    // identify these eye positions at elevations30 and15. The lower15-degree
    // cap distinguishes eight-bit alpha first from multiplying raw alpha.
    for (elevation, eye, expected) in [
      (
        30.0_f64,
        [0.0, 1.91887631, 0.49879418],
        [(172.0, 16.0), (197.0, 44.0)],
      ),
      (
        15.0_f64,
        [-0.18757275, 1.91808225, 0.10313498],
        [(214.0, 16.0), (242.0, 20.0)],
      ),
    ] {
      let (s, c) = elevation.to_radians().sin_cos();
      let camera = PieCamera {
        top_y: c * eye[2] - s * eye[1],
        top_distance: s * eye[2] + c * eye[1],
      };
      let projector = Projector {
        plane: RadialPerspectiveProjection {
          conic_center: (0.0, 0.0),
          radii: (1.0, 1.0),
          strength: 0.0,
          plane_offset: (0.0, 0.0),
          extrusion: Some(RadialExtrusionProjection {
            denominator: 0.0,
            numerator: 0.24,
          }),
        },
        thickness: 0.24,
        sine: s,
        cosine: c,
        parallel: false,
        camera: Some(camera),
        material: Some(a::PresetMaterialTypeValues::Clear),
      };
      for (i, (z, normal)) in [
        (-0.24, [0.0, c as f32, s as f32]),
        (0.0, [0.0, -c as f32, -s as f32]),
      ]
      .into_iter()
      .enumerate()
      {
        let source = super::super::RgbColor {
          r: 128,
          g: 128,
          b: 128,
        };
        let light = super::super::lighting(source, normal);
        let (color, alpha) = diffuse(&projector, [-eye[0], 0.0, z], light);
        assert_eq!(alpha, expected[i].0);
        for value in color {
          assert!(
            (value * 128.0 / 255.0 - expected[i].1).abs() < 0.001,
            "{elevation},cap={i}: {value}"
          );
        }
      }
    }
  }
}
