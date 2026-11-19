//! A Word pie legend key is an independent DrawingML solid, not the pie's
//! circular lathe or a height-map approximation of its painted bitmap.

use super::ImageItem;
use crate::common::drawingml_3d::{self, Static3dStyleParts};
use crate::common::drawingml_shape_raster::{
  PageToRasterMapping, rasterize_vector_scene_at_mapping,
};
use crate::common::{DisplayItem, PathItem};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::units::CoordinateValue;

pub(super) fn lower(
  path: &PathItem<'static>,
  scene: &a::Scene3DType,
  shape: &a::Shape3DType,
  colors: &Static3dStyleParts,
  output_dpi: f32,
) -> Option<ImageItem> {
  let mut physical = path.clone();
  if let Some(stroke) = &mut physical.stroke {
    // The fill solid is widened by the native one-point geometry floor.
    // The unlit outline tube independently retains the actual line width.
    stroke.width.0 = stroke.width.0.max(1.0);
  }
  let painted_geometry = crate::common::drawingml_stroke::closed_shape_solid_geometry(&physical);
  if physical.stroke.is_some() && painted_geometry.is_none() {
    return None;
  }
  let (commands, model_bounds) = painted_geometry.as_ref().map_or(
    (path.commands.as_slice(), path.bounds),
    |(commands, bounds)| (commands.as_slice(), *bounds),
  );
  // Native indexed legend meshes use z=0 at the outline/extrusion, then
  // bevelT above it. The common WPS adapter instead accepts the cap's z.
  let mut solid = shape.clone();
  if let Some(bevel) = &solid.bevel_top {
    solid.z = Some(CoordinateValue::Emu(
      solid.z.map_or(0, |z| z.to_emu()) + bevel.height.map_or(76_200, |h| h.to_emu()),
    ));
  }
  let mut contour_color = colors.contour_color;
  if let Some(stroke) = &path.stroke {
    if solid.contour_width.is_some_and(|w| w.to_emu() > 0) {
      return None;
    }
    solid.contour_width = Some(CoordinateValue::Emu(
      (stroke.width.0 * 12_700.0).round() as i64
    ));
    contour_color = Some(drawingml_3d::Static3dColor {
      color: crate::model::RgbColor {
        r: stroke.color.r,
        g: stroke.color.g,
        b: stroke.color.b,
      },
      alpha: stroke.color.a,
    });
  }
  let projection = drawingml_3d::camera_projection(scene, 0.0);
  let output = drawingml_3d::projected_output_bounds(
    projection,
    &solid,
    model_bounds.size.width.0,
    model_bounds.size.height.0,
  );
  let pixels_per_point = crate::units::OFFICE_FIXED_OUTPUT_DPI / crate::units::POINTS_PER_INCH;
  let dot_pt = pixels_per_point.recip();
  // Actual native key draws use the 600-DPI printer target (63x63 for
  // the half-point outline), including two guard dots on every edge.
  let camera_near = [
    model_bounds.origin.x.0 + output.left_pt,
    model_bounds.origin.y.0 + output.top_pt,
  ]
  .map(|v| v - 2.0 * dot_pt);
  let camera_far = [
    model_bounds.origin.x.0 + output.right_pt,
    model_bounds.origin.y.0 + output.bottom_pt,
  ]
  .map(|v| v + 2.0 * dot_pt);
  let near = camera_near.map(|v| (f64::from(v) * f64::from(pixels_per_point)).floor());
  let far = camera_far.map(|v| (f64::from(v) * f64::from(pixels_per_point)).ceil());
  let width = (far[0] - near[0]) as u32;
  let height = (far[1] - near[1]) as u32;
  if !crate::common::drawingml_shape_raster::fixed_output_effect_raster_size_fits(width, height) {
    return None;
  }
  let raster_bounds = super::common_rect(
    (near[0] / f64::from(pixels_per_point)) as f32,
    (near[1] / f64::from(pixels_per_point)) as f32,
    width as f32 * dot_pt,
    height as f32 * dot_pt,
  );
  // VF11 is an independent unlit tube, but the bevel's texture retains
  // fill-plus-line paint as well. Native green-quarter-point controls tint
  // visible bevel samples beyond the tube's spatial/depth support; their
  // neutral COLOR0/1 streams cannot supply that tint. The physical paint
  // shares the observed one-point minimum, independently of tube width.
  let material_path = physical;
  let display = DisplayItem::Path(material_path.clone());
  let mut image = rasterize_vector_scene_at_mapping(
    std::slice::from_ref(&display),
    PageToRasterMapping {
      width_px: width,
      height_px: height,
      scale_x: pixels_per_point,
      scale_y: pixels_per_point,
      translate_x: -near[0] as f32,
      translate_y: -near[1] as f32,
      text_hinting: None,
    },
  )?;
  let geometry = crate::common::drawingml_shape_raster::static_3d_shape_geometry(
    commands,
    raster_bounds,
    pixels_per_point,
  )?;
  let options = drawingml_3d::Static3dRenderOptions {
    extrusion_color: colors.extrusion_color.or_else(|| {
      drawingml_3d::automatic_extrusion_color_from_items(std::slice::from_ref(&display))
    }),
    contour_color,
    pixels_per_point,
    model_surface: Some(drawingml_3d::Static3dSurface {
      left_px: (model_bounds.origin.x.0 - raster_bounds.origin.x.0) * pixels_per_point,
      top_px: (model_bounds.origin.y.0 - raster_bounds.origin.y.0) * pixels_per_point,
      width_px: model_bounds.size.width.0 * pixels_per_point,
      height_px: model_bounds.size.height.0 * pixels_per_point,
    }),
  };
  // The captured native matrix fits the continuous guarded window, then
  // stretches it onto the independently enclosed printer target. For the
  // half-point control its X scale is19.271128/inch, not144/7.56.
  let sx = width as f32 / (camera_far[0] - camera_near[0]);
  let sy = height as f32 / (camera_far[1] - camera_near[1]);
  let center = [
    model_bounds.origin.x.0 + model_bounds.size.width.0 * 0.5,
    model_bounds.origin.y.0 + model_bounds.size.height.0 * 0.5,
  ];
  let projection = projection
    .with_viewport_scale(sx / pixels_per_point, sy / pixels_per_point)
    .with_viewport_translation_px(
      (center[0] - camera_near[0]) * sx - (center[0] - raster_bounds.origin.x.0) * pixels_per_point,
      (center[1] - camera_near[1]) * sy - (center[1] - raster_bounds.origin.y.0) * pixels_per_point,
    );
  let realization = drawingml_3d::TextSurfaceRealizationPlan::for_shape(
    &geometry,
    projection,
    &solid,
    options,
    pixels_per_point,
  );
  let texture = realization.and_then(|realization| {
    let plan =
      drawingml_3d::TextMaterialTexturePlan::from_shape_realization(&geometry, realization)?;
    crate::common::drawingml_shape_raster::rasterize_shape_surface_material_texture(
      &material_path,
      plan,
    )
  });
  let realized = realization.map(|realization| {
    geometry
      .clone()
      .with_shape_profile_realization(realization, pixels_per_point)
  });
  drawingml_3d::apply_static_3d_chart_legend_geometry(
    &mut image,
    drawingml_3d::Static3dShapeSurface {
      wordprocessing_shape: true,
      material_texture: texture.as_ref(),
      geometry: realized.as_ref().unwrap_or(&geometry),
      material_geometry: &geometry,
      rasterization: drawingml_3d::Static3dShapeRasterization::Multisample8,
      uniform_paint_opacity: None,
      two_sided_extrusion: false,
    },
    scene,
    projection,
    &solid,
    options,
  );
  let display_bounds = super::common_rect(
    raster_bounds.origin.x.0,
    raster_bounds.origin.y.0,
    raster_bounds.size.width.0 - dot_pt * 0.5,
    raster_bounds.size.height.0 - dot_pt * 0.5,
  );
  // Existing Office fixed-output bitmap policy: integer twips, inclusive
  // device endpoints. Keep the 600-DPI mesh independent of this resample.
  let extent = |pt: f32| {
    (((f64::from((pt * 20.0).floor()) * f64::from(output_dpi) / 1440.0).ceil() - 1.0).max(1.0))
      as u32
  };
  let mut output = super::chart_raster_image_with_target(
    &image,
    display_bounds,
    Some((
      extent(display_bounds.size.width.0),
      extent(display_bounds.size.height.0),
    )),
    None,
  )?;
  output.content_type = Some("application/vnd.ooxmlsdk.wordprocessing-chart-3d+png".to_owned());
  Some(output)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::{Color, Fill, PathCommand, Stroke};

  #[test]
  fn legend_key_keeps_native_printer_targets_separate_from_pdf_images() {
    let bounds = super::super::common_rect(416.941_62, 188.027_4, 5.492_25, 5.492_25);
    let corners = [
      (bounds.origin.x.0, bounds.origin.y.0),
      (bounds.origin.x.0 + bounds.size.width.0, bounds.origin.y.0),
      (
        bounds.origin.x.0 + bounds.size.width.0,
        bounds.origin.y.0 + bounds.size.height.0,
      ),
      (bounds.origin.x.0, bounds.origin.y.0 + bounds.size.height.0),
    ]
    .map(|(x, y)| super::super::common_point(x, y));
    let mut scene = a::Scene3DType::default();
    scene.camera.preset = a::PresetCameraValues::OrthographicFront;
    scene.light_rig.rig = a::LightRigValues::ThreePoints;
    scene.light_rig.direction = a::LightRigDirectionValues::Top;
    let shape = a::Shape3DType {
      preset_material: Some(a::PresetMaterialTypeValues::Clear),
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(260_350)),
        height: Some(CoordinateValue::Emu(50_800)),
        preset: Some(a::BevelPresetValues::SoftRound),
      }),
      bevel_bottom: Some(a::BevelBottom {
        preset: Some(a::BevelPresetValues::SoftRound),
        ..Default::default()
      }),
      ..Default::default()
    };
    // Actual native line controls: NoFill/.25/.5/2/4pt. A half-point
    // key's GFX full/target rectangles are3466,1558..3529,1621 /0..63.
    // The independently exported PDF places20samples in7.5physical points.
    for (width, printer, left, top, extent) in [
      (None, 51, 416.64, 187.68, 6.06),
      (Some(0.25), 61, 416.04, 187.08, 7.26),
      (Some(0.5), 63, 415.92, 186.96, 7.5),
      (Some(2.0), 84, 414.6, 185.76, 10.02),
      (Some(4.0), 117, 412.68, 183.72, 13.98),
    ] {
      let path = PathItem {
        bounds,
        points: corners.to_vec(),
        commands: std::iter::once(PathCommand::MoveTo(corners[0]))
          .chain(corners[1..].iter().copied().map(PathCommand::LineTo))
          .chain(std::iter::once(PathCommand::Close))
          .collect(),
        closed: true,
        fill: Fill::Solid(Color {
          r: 191,
          g: 191,
          b: 191,
          a: 255,
        }),
        stroke: width.map(|w| Stroke {
          width: crate::common::Pt(w),
          color: Color {
            r: 174,
            g: 90,
            b: 33,
            a: 255,
          },
          ..Default::default()
        }),
      };
      let key = lower(&path, &scene, &shape, &Static3dStyleParts::default(), 200.0).unwrap();
      let raw = image::load_from_memory(&key.data).unwrap();
      assert_eq!((raw.width(), raw.height()), (printer, printer));
      for (actual, native) in [
        (key.x_pt, left),
        (key.y_pt, top),
        (key.width_pt, extent),
        (key.height_pt, extent),
      ] {
        assert!(
          (actual - native).abs() < 0.000_1,
          "line{width:?}: {actual} != {native}"
        );
      }
      assert!(raw.to_rgba8().pixels().any(|p| p[3] > 0 && p[0] < 240));
      if width == Some(0.5) {
        // Native GPU mesh rays and the independently exported20x20 mask
        // distinguish front-face source-over from nearest-only resolve:
        // right/bottom inner edges are236/235, not184; the center is27.
        let rgba = raw.to_rgba8();
        let alpha_at_output = |x: f64, y: f64| {
          let x = x * f64::from(printer) / 20.0;
          let y = y * f64::from(printer) / 20.0;
          let left = x.floor() as u32;
          let top = y.floor() as u32;
          let fx = x - x.floor();
          let fy = y - y.floor();
          f64::from(rgba.get_pixel(left, top)[3]) * (1.0 - fx) * (1.0 - fy)
            + f64::from(rgba.get_pixel(left + 1, top)[3]) * fx * (1.0 - fy)
            + f64::from(rgba.get_pixel(left, top + 1)[3]) * (1.0 - fx) * fy
            + f64::from(rgba.get_pixel(left + 1, top + 1)[3]) * fx * fy
        };
        for (x, y, native) in [(18.0, 10.0, 236.0), (10.0, 18.0, 235.0), (10.0, 10.0, 27.0)] {
          let alpha = alpha_at_output(x, y);
          assert!(
            (alpha - native).abs() <= 3.0,
            "({x},{y}): alpha{alpha} != native{native}"
          );
        }
      }
      if width.is_none() {
        assert!(raw.to_rgba8().pixels().any(|p| p[3] > 0 && p[3] < 128));
      }
    }
  }
}
