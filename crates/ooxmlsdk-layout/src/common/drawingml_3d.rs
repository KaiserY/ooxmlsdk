use image::{Rgba, RgbaImage};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use super::DisplayItem;
use crate::model::RgbColor;

const EMUS_PER_POINT: f32 = 12_700.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Static3dColor {
  pub(crate) color: RgbColor,
  pub(crate) alpha: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Static3dStyle {
  pub(crate) scene: Box<a::Scene3DType>,
  pub(crate) shape: Box<a::Shape3DType>,
  pub(crate) extrusion_color: Option<Static3dColor>,
  pub(crate) contour_color: Option<Static3dColor>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Static3dPadding {
  pub(crate) left_pt: f32,
  pub(crate) top_pt: f32,
  pub(crate) right_pt: f32,
  pub(crate) bottom_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Static3dSurface {
  pub(crate) left_px: f32,
  pub(crate) top_px: f32,
  pub(crate) width_px: f32,
  pub(crate) height_px: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Static3dProjection {
  /// Page-plane movement of one point of positive extrusion depth.
  pub(crate) offset_x_per_depth: f32,
  pub(crate) offset_y_per_depth: f32,
  /// Static rotation contributed by the camera to the front face.
  pub(crate) face_rotation_degrees: f32,
  pub(crate) parallel: bool,
  pub(crate) field_of_view_degrees: Option<f32>,
  /// Perspective camera distance in points when the preset defines one.
  pub(crate) perspective_distance_pt: Option<f32>,
  rotation: [[f32; 3]; 3],
  skew_x_per_depth: f32,
  skew_y_per_depth: f32,
  origin_x: f32,
  origin_y: f32,
  viewpoint_x_pt: f32,
  viewpoint_y_pt: f32,
}

#[derive(Clone, Copy)]
struct CameraPreset {
  parallel: bool,
  latitude: i32,
  longitude: i32,
  revolution: i32,
  origin_x: f32,
  origin_y: f32,
  skew_amount: f32,
  skew_angle_degrees: f32,
  viewpoint_x: f32,
  viewpoint_y: f32,
  viewpoint_z: f32,
}

impl CameraPreset {
  const fn angles(parallel: bool, latitude: i32, longitude: i32, revolution: i32) -> Self {
    Self {
      parallel,
      latitude,
      longitude,
      revolution,
      origin_x: 0.0,
      origin_y: 0.0,
      skew_amount: 0.0,
      skew_angle_degrees: 0.0,
      viewpoint_x: 0.0,
      viewpoint_y: 0.0,
      viewpoint_z: if parallel { 0.0 } else { 38_451.0 },
    }
  }

  const fn oblique(origin_x: f32, origin_y: f32, amount: f32, angle: f32) -> Self {
    Self {
      parallel: true,
      latitude: 0,
      longitude: 0,
      revolution: 0,
      origin_x,
      origin_y,
      skew_amount: amount,
      skew_angle_degrees: angle,
      viewpoint_x: 0.0,
      viewpoint_y: 0.0,
      viewpoint_z: 0.0,
    }
  }

  const fn legacy_perspective(viewpoint_x: f32, viewpoint_y: f32) -> Self {
    Self {
      parallel: false,
      latitude: 0,
      longitude: 0,
      revolution: 0,
      // The legacy camera table uses the same compass displacement for the
      // relative origin and the 3,472 Hmm viewpoint offset.
      origin_x: viewpoint_x / 6_944.0,
      origin_y: viewpoint_y / 6_944.0,
      skew_amount: 0.0,
      skew_angle_degrees: 0.0,
      viewpoint_x,
      viewpoint_y,
      viewpoint_z: 25_000.0,
    }
  }
}

/// Resolves the fixed DrawingML camera table into a deterministic page-plane
/// projection. The preset values are the 62-entry Office table translated in
/// LibreOffice `oox/source/drawingml/scene3dhelper.cxx`; an explicit `a:rot`
/// replaces the preset latitude/longitude/revolution as required by ECMA-376.
pub(crate) fn camera_projection(
  scene: &a::Scene3DType,
  shape_rotation_degrees: f32,
) -> Static3dProjection {
  let preset = camera_preset(scene.camera.preset);
  let (latitude, longitude, mut revolution) = scene.camera.rotation.as_ref().map_or(
    (preset.latitude, preset.longitude, preset.revolution),
    |rotation| (rotation.latitude, rotation.longitude, rotation.revolution),
  );
  if is_legacy_perspective(scene.camera.preset) {
    // Office ignores authored z revolution for all nine legacy-perspective
    // cameras. LibreOffice carries this compatibility rule before converting
    // OOXML's Y-X-Z rotation order.
    revolution = 0;
  } else if scene.camera.rotation.is_some()
    && is_modern_perspective(scene.camera.preset)
    && shape_rotation_degrees.abs() > f32::EPSILON
    && latitude > 5_400_000
    && latitude <= 16_200_000
  {
    // Office adds this half turn only for user-entered angles on modern
    // perspective cameras when the x rotation crosses the rear hemisphere.
    revolution += 10_800_000;
  }
  // `a:xfrm/@rot` is part of the 3-D transform, not a preprocessing
  // rotation of the painted 2-D bitmap.  LibreOffice's
  // Scene3DHelper::getAPIAnglesFrom3DProperties() folds the shape rotation
  // into the camera revolution (the signs are opposite in DrawingML's
  // y-down coordinate system) before it derives the final matrix.  Keeping
  // the rotation here is essential for pictures: pre-rotating their source
  // rectangle and then applying this camera matrix produces a detached,
  // twice-rotated front face.
  revolution -= (shape_rotation_degrees * 60_000.0).round() as i32;
  let (matrix, face_rotation) = oox_rotation_matrix(latitude, longitude, revolution);
  let rotation = matrix.map(|row| row.map(|value| value as f32));
  let (skew_x_per_depth, skew_y_per_depth) = if preset.skew_amount != 0.0 {
    let skew_angle = if is_modern_oblique(scene.camera.preset) {
      // Office applies a modern oblique camera's z rotation after creating
      // the projection. LibreOffice carries the same preset-specific
      // correction in addProjectionGeometryToMap().
      preset.skew_angle_degrees - shape_rotation_degrees
    } else {
      preset.skew_angle_degrees
    };
    let angle = skew_angle.to_radians();
    // LibreOffice applies this as a z shear in its y-up scene. DrawingML
    // raster coordinates point down, hence the sign change on the y term.
    (
      angle.cos() * preset.skew_amount / 100.0,
      -angle.sin() * preset.skew_amount / 100.0,
    )
  } else {
    (0.0, 0.0)
  };
  let field_of_view_degrees = scene
    .camera
    .field_of_view
    .map(|value| (value as f32 / 60_000.0).clamp(0.5, 179.5));
  let perspective_distance_pt = (!preset.parallel).then(|| {
    let distance_hmm = if let Some(fov) = field_of_view_degrees {
      15_976.0 / (fov * 0.5).to_radians().tan()
    } else {
      preset.viewpoint_z
    };
    // 1/100 mm to points.
    distance_hmm * 72.0 / 2_540.0
  });
  Static3dProjection {
    offset_x_per_depth: rotation[0][2] + skew_x_per_depth,
    offset_y_per_depth: rotation[1][2] + skew_y_per_depth,
    face_rotation_degrees: face_rotation.to_degrees() as f32,
    parallel: preset.parallel,
    field_of_view_degrees,
    perspective_distance_pt,
    rotation,
    skew_x_per_depth,
    skew_y_per_depth,
    origin_x: preset.origin_x,
    origin_y: preset.origin_y,
    viewpoint_x_pt: preset.viewpoint_x * 72.0 / 2_540.0,
    viewpoint_y_pt: preset.viewpoint_y * 72.0 / 2_540.0,
  }
}

fn is_legacy_perspective(preset: a::PresetCameraValues) -> bool {
  use a::PresetCameraValues as P;
  matches!(
    preset,
    P::LegacyPerspectiveBottom
      | P::LegacyPerspectiveBottomLeft
      | P::LegacyPerspectiveBottomRight
      | P::LegacyPerspectiveFront
      | P::LegacyPerspectiveLeft
      | P::LegacyPerspectiveRight
      | P::LegacyPerspectiveTop
      | P::LegacyPerspectiveTopLeft
      | P::LegacyPerspectiveTopRight
  )
}

fn is_modern_perspective(preset: a::PresetCameraValues) -> bool {
  use a::PresetCameraValues as P;
  matches!(
    preset,
    P::PerspectiveAbove
      | P::PerspectiveAboveLeftFacing
      | P::PerspectiveAboveRightFacing
      | P::PerspectiveBelow
      | P::PerspectiveContrastingLeftFacing
      | P::PerspectiveContrastingRightFacing
      | P::PerspectiveFront
      | P::PerspectiveHeroicExtremeLeftFacing
      | P::PerspectiveHeroicExtremeRightFacing
      | P::PerspectiveHeroicLeftFacing
      | P::PerspectiveHeroicRightFacing
      | P::PerspectiveLeft
      | P::PerspectiveRelaxed
      | P::PerspectiveRelaxedModerately
      | P::PerspectiveRight
  )
}

fn is_modern_oblique(preset: a::PresetCameraValues) -> bool {
  use a::PresetCameraValues as P;
  matches!(
    preset,
    P::ObliqueBottom
      | P::ObliqueBottomLeft
      | P::ObliqueBottomRight
      | P::ObliqueLeft
      | P::ObliqueRight
      | P::ObliqueTop
      | P::ObliqueTopLeft
      | P::ObliqueTopRight
  )
}

fn is_legacy_camera(preset: a::PresetCameraValues) -> bool {
  use a::PresetCameraValues as P;
  matches!(
    preset,
    P::LegacyObliqueBottom
      | P::LegacyObliqueBottomLeft
      | P::LegacyObliqueBottomRight
      | P::LegacyObliqueFront
      | P::LegacyObliqueLeft
      | P::LegacyObliqueRight
      | P::LegacyObliqueTop
      | P::LegacyObliqueTopLeft
      | P::LegacyObliqueTopRight
      | P::LegacyPerspectiveBottom
      | P::LegacyPerspectiveBottomLeft
      | P::LegacyPerspectiveBottomRight
      | P::LegacyPerspectiveFront
      | P::LegacyPerspectiveLeft
      | P::LegacyPerspectiveRight
      | P::LegacyPerspectiveTop
      | P::LegacyPerspectiveTopLeft
      | P::LegacyPerspectiveTopRight
  )
}

pub(crate) fn output_padding(
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  width_pt: f32,
  height_pt: f32,
) -> Static3dPadding {
  let depth_pt = shape
    .extrusion_height
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  let z_pt = shape
    .z
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  let contour_pt = shape
    .contour_width
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  // DrawingML's front plane is located at `z`; extrusion extends behind it
  // to `z - extrusionH`. LibreOffice expresses the same interval as
  // forwardDepth=z and backwardDepth=extrusionH-z before translating the
  // extruded solid. Project all eight volume corners so x/y camera rotation
  // and perspective contribute to bounds as well as z travel.
  let mut min_x = f32::INFINITY;
  let mut min_y = f32::INFINITY;
  let mut max_x = f32::NEG_INFINITY;
  let mut max_y = f32::NEG_INFINITY;
  for depth in [z_pt, z_pt - depth_pt] {
    for x in [-width_pt * 0.5, width_pt * 0.5] {
      for y in [-height_pt * 0.5, height_pt * 0.5] {
        let (projected_x, projected_y) =
          project_local(projection, x, y, depth, width_pt, height_pt);
        min_x = min_x.min(projected_x);
        min_y = min_y.min(projected_y);
        max_x = max_x.max(projected_x);
        max_y = max_y.max(projected_y);
      }
    }
  }
  // ECMA-376 §20.1.5.3-4 defines bevel width as an inset into the face and
  // MS-OI29500 §20.1.10.9 defines bevel height along the z axis. A bevel
  // therefore does not enlarge the 2-D silhouette for an orthographic-front
  // camera. Camera-projected z travel is already represented by the
  // front/back depth bounds above; only a contour grows every edge in screen
  // space.
  let edge = contour_pt;
  Static3dPadding {
    left_pt: (-width_pt * 0.5 - min_x).max(0.0) + edge,
    top_pt: (-height_pt * 0.5 - min_y).max(0.0) + edge,
    right_pt: (max_x - width_pt * 0.5).max(0.0) + edge,
    bottom_pt: (max_y - height_pt * 0.5).max(0.0) + edge,
  }
}

/// Lowers DrawingML static 3-D to a bounded RGBA layer. This follows the
/// DrawingML painter order: back/extruded faces, contour/bevel, then the
/// original front face. The caller supplies a padded image and resolved theme
/// colors, so this stage is shared by DOCX/PPTX/XLSX without resolving package
/// theme state a second time.
pub(crate) fn apply_static_3d(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  extrusion_color: Option<Static3dColor>,
  contour_color: Option<Static3dColor>,
  pixels_per_point: f32,
  model_surface: Option<Static3dSurface>,
) {
  let depth_pt = shape
    .extrusion_height
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  let z_pt = shape
    .z
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  let front_z_px = z_pt * pixels_per_point;
  let back_z_px = (z_pt - depth_pt) * pixels_per_point;
  let contour_px = shape
    .contour_width
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
    .unwrap_or(0.0)
    .round()
    .clamp(0.0, 32.0) as i32;
  let top_bevel_px = shape
    .bevel_top
    .as_ref()
    .map_or(0.0, |bevel| {
      bevel.width.map_or(0.0, |value| {
        value.to_emu() as f32 / EMUS_PER_POINT
          * pixels_per_point
          * bevel_terminal_inset(bevel.preset)
      })
    })
    .round()
    .clamp(0.0, 24.0) as i32;
  let bottom_bevel_px = shape
    .bevel_bottom
    .as_ref()
    .map_or(0.0, |bevel| {
      bevel.width.map_or(0.0, |value| {
        value.to_emu() as f32 / EMUS_PER_POINT
          * pixels_per_point
          * bevel_terminal_inset(bevel.preset)
      })
    })
    .round()
    .clamp(0.0, 24.0) as i32;
  let wireframe = shape.preset_material == Some(a::PresetMaterialTypeValues::LegacyWireframe);
  let bounds = alpha_bounds(image);
  let Some(bounds) = bounds else {
    return;
  };
  let model_surface = model_surface.unwrap_or(Static3dSurface {
    left_px: bounds.0 as f32,
    top_px: bounds.1 as f32,
    width_px: (bounds.2 - bounds.0 + 1).max(1) as f32,
    height_px: (bounds.3 - bounds.1 + 1).max(1) as f32,
  });
  let bounds_width = model_surface.width_px.max(1.0);
  let bounds_height = model_surface.height_px.max(1.0);
  if projection_plane_is_identity(
    projection,
    front_z_px,
    bounds_width,
    bounds_height,
    pixels_per_point,
  ) && depth_pt <= f32::EPSILON
    && contour_px == 0
    && top_bevel_px == 0
    && !wireframe
  {
    return;
  }

  let front = image.clone();
  image.fill(0);
  if depth_pt > f32::EPSILON {
    let steps = projected_depth_steps(
      projection,
      front_z_px,
      back_z_px,
      bounds_width,
      bounds_height,
      pixels_per_point,
    );
    let extrusion = extrusion_color.unwrap_or_else(|| average_extrusion_color(&front));
    let diffusion = material_diffusion(shape.preset_material);
    let back_normal = lighting_surface_normal(scene, projection, [0.0, 0.0, -1.0]);
    let back_lighting = light_rig_surface_shade(scene, back_normal);
    if !wireframe {
      let mut back_face = RgbaImage::new(image.width(), image.height());
      composite_projected_image(
        &mut back_face,
        &front,
        projection,
        back_z_px,
        bounds,
        model_surface,
        pixels_per_point,
        Some((extrusion, scale_shade(back_lighting, diffusion))),
      );
      if bottom_bevel_px > 0 {
        let bevel_height_px = shape
          .bevel_bottom
          .as_ref()
          .and_then(|bevel| bevel.height)
          .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
          .unwrap_or(bottom_bevel_px as f32);
        let mask = back_face.clone();
        composite_bevel(
          &mut back_face,
          &mask,
          bottom_bevel_px,
          bevel_height_px,
          scene,
          shape.preset_material,
          true,
        );
      }
      composite_image(image, &back_face);
    }
    composite_extrusion_edges(
      image,
      &front,
      bounds,
      model_surface,
      projection,
      front_z_px,
      back_z_px,
      pixels_per_point,
      steps,
      extrusion,
      scene,
      shape.preset_material,
      wireframe,
    );
  }
  if contour_px > 0 {
    let contour = contour_color.unwrap_or(Static3dColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 255,
    });
    // Office contours the complete projected solid, including extrusion
    // edges, rather than only the untransformed front-face mask.
    let mut silhouette = image.clone();
    composite_projected_image(
      &mut silhouette,
      &front,
      projection,
      front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      None,
    );
    composite_outline(image, &silhouette, contour_px, contour);
  }
  let mut front_face = front.clone();
  if top_bevel_px > 0 {
    let bevel_height_px = shape
      .bevel_top
      .as_ref()
      .and_then(|bevel| bevel.height)
      .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
      .unwrap_or(top_bevel_px as f32);
    composite_bevel(
      &mut front_face,
      &front,
      top_bevel_px,
      bevel_height_px,
      scene,
      shape.preset_material,
      false,
    );
  }
  if wireframe {
    let mut outline = RgbaImage::new(image.width(), image.height());
    composite_outline(
      &mut outline,
      &front,
      1,
      contour_color.or(extrusion_color).unwrap_or(Static3dColor {
        color: RgbColor { r: 0, g: 0, b: 0 },
        alpha: 255,
      }),
    );
    composite_projected_image(
      image,
      &outline,
      projection,
      front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      None,
    );
  } else {
    composite_projected_image(
      image,
      &front_face,
      projection,
      front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      None,
    );
  }
}

fn bevel_terminal_inset(preset: Option<a::BevelPresetValues>) -> f32 {
  use a::BevelPresetValues as B;
  // Terminal y coordinates from the Office bevel-space curves published in
  // MS-OI29500 §20.1.10.9. Office stretches x to the authored bevel height
  // but scales y directly by bevel width.
  match preset.unwrap_or(B::Circle) {
    B::RelaxedInset => 0.64,
    B::SoftRound => 0.333_33,
    B::HardEdge => 0.822_129,
    B::Slope => 0.01,
    B::Angle
    | B::ArtDeco
    | B::Circle
    | B::Convex
    | B::CoolSlant
    | B::Cross
    | B::Divot
    | B::Riblet => 1.0,
  }
}

#[derive(Clone, Copy)]
struct RigLight {
  color: [f32; 3],
  direction: [f32; 3],
  scale: f32,
  offset: f32,
  specular: bool,
  diffuse: bool,
}

const NO_LIGHT: RigLight = RigLight {
  color: [0.0; 3],
  direction: [0.0; 3],
  scale: 0.0,
  offset: 0.0,
  specular: false,
  diffuse: false,
};

#[derive(Clone, Copy)]
struct LightRigPreset {
  ambient: [f32; 3],
  lights: [RigLight; 4],
  count: usize,
}

const fn rig_light(color: [f32; 3], direction: [f32; 3]) -> RigLight {
  RigLight {
    color,
    direction,
    scale: 1.0,
    offset: 0.0,
    specular: true,
    diffuse: true,
  }
}

const fn adjusted_rig_light(
  color: [f32; 3],
  direction: [f32; 3],
  scale: f32,
  offset: f32,
) -> RigLight {
  RigLight {
    color,
    direction,
    scale,
    offset,
    specular: true,
    diffuse: true,
  }
}

const fn nondiffuse_rig_light(color: [f32; 3], direction: [f32; 3]) -> RigLight {
  RigLight {
    color,
    direction,
    scale: 1.0,
    offset: 0.0,
    specular: true,
    diffuse: false,
  }
}

const fn light_rig_preset(
  ambient: [f32; 3],
  lights: [RigLight; 4],
  count: usize,
) -> LightRigPreset {
  LightRigPreset {
    ambient,
    lights,
    count,
  }
}

fn light_rig_surface_shade(scene: &a::Scene3DType, normal: [f32; 3]) -> [f32; 3] {
  let preset = light_rig(scene.light_rig.rig);
  let rotation_degrees = scene.light_rig.rotation.as_ref().map_or_else(
    || light_rig_direction_degrees(scene.light_rig.direction),
    |rotation| rotation.revolution as f32 / 60_000.0,
  );
  let mut shade = preset.ambient;
  for light in &preset.lights[..preset.count] {
    if !light.diffuse {
      continue;
    }
    // MS-OI29500 publishes light directions in light-rig coordinates. Office
    // first applies the fixed 90-degree basis conversion and then the rig
    // direction or explicit rotation.
    let direction = resolved_light_direction(scene, *light, rotation_degrees);
    // The preset table describes the direction in which light travels.
    // Surface illumination needs the vector from the surface toward the
    // light. LibreOffice documents the same conversion when it stores the
    // negated direction as First/SecondLightDirection.
    let level = light.scale * dot3([-direction[0], -direction[1], -direction[2]], normal).max(0.0)
      + light.offset;
    for (channel, color) in shade.iter_mut().zip(light.color) {
      *channel += color * level;
    }
  }
  shade.map(|channel| channel.clamp(0.12, 2.5))
}

fn resolved_light_direction(
  scene: &a::Scene3DType,
  light: RigLight,
  rotation_degrees: f32,
) -> [f32; 3] {
  let mut direction = [-light.direction[1], -light.direction[0], light.direction[2]];
  normalize3(&mut direction);
  rotate_z(&mut direction, rotation_degrees.to_radians());
  if let Some(rotation) = scene.light_rig.rotation.as_ref() {
    rotate_x(
      &mut direction,
      (rotation.latitude as f32 / 60_000.0).to_radians(),
    );
    rotate_y(
      &mut direction,
      (rotation.longitude as f32 / 60_000.0).to_radians(),
    );
  }
  direction
}

fn light_rig_surface_specular(
  scene: &a::Scene3DType,
  normal: [f32; 3],
  view_direction: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 3] {
  let amount = material_specularity(material);
  let power = material_specular_power(material);
  let blinn_highlight = material_blinn_highlight(material);
  if amount <= f32::EPSILON || power <= f32::EPSILON {
    return [0.0; 3];
  }
  let preset = light_rig(scene.light_rig.rig);
  let rotation_degrees = scene.light_rig.rotation.as_ref().map_or_else(
    || light_rig_direction_degrees(scene.light_rig.direction),
    |rotation| rotation.revolution as f32 / 60_000.0,
  );
  let mut specular = [0.0; 3];
  for light in &preset.lights[..preset.count] {
    if !light.specular {
      continue;
    }
    let direction = resolved_light_direction(scene, *light, rotation_degrees);
    let mut toward_light = [-direction[0], -direction[1], -direction[2]];
    normalize3(&mut toward_light);
    let normal_light = dot3(normal, toward_light);
    if normal_light <= 0.0 {
      continue;
    }
    let highlight = if blinn_highlight {
      let mut halfway = [
        toward_light[0] + view_direction[0],
        toward_light[1] + view_direction[1],
        toward_light[2] + view_direction[2],
      ];
      normalize3(&mut halfway);
      dot3(normal, halfway).max(0.0)
    } else {
      // MS-OI29500's material table selects the classic reflection-vector
      // highlight unless `Blinn Highlight` is Yes.
      let reflected = [
        2.0 * normal_light * normal[0] - toward_light[0],
        2.0 * normal_light * normal[1] - toward_light[1],
        2.0 * normal_light * normal[2] - toward_light[2],
      ];
      dot3(reflected, view_direction).max(0.0)
    };
    let level = highlight.powf(power) * amount;
    for (channel, light_color) in specular.iter_mut().zip(light.color) {
      *channel += light_color * level;
    }
  }
  specular.map(|channel| channel.clamp(0.0, 1.0))
}

fn light_rig(rig: a::LightRigValues) -> LightRigPreset {
  use a::LightRigValues as R;
  const D1: [f32; 3] = [0.6574, -0.7316, -0.1806];
  const D2: [f32; 3] = [-0.2781, -0.4509, -0.8482];
  const D3: [f32; 3] = [0.6720, -0.6185, -0.4073];
  const D4: [f32; 3] = [-0.1825, 0.9680, 0.1722];
  match rig {
    R::ThreePoints => light_rig_preset(
      [0.0; 3],
      [
        rig_light([1.141; 3], [-0.6515, -0.2693, -0.7093]),
        rig_light([0.5; 3], [0.8482, 0.2469, -0.4686]),
        rig_light([1.0; 3], [0.5634, -0.2812, 0.7769]),
        NO_LIGHT,
      ],
      3,
    ),
    R::Balanced => light_rig_preset(
      [0.13; 3],
      [
        rig_light([1.05; 3], [0.5263, -0.4092, -0.7453]),
        rig_light([1.0; 3], [-0.9386, 0.3426, -0.0410]),
        rig_light([0.5; 3], [0.0934, 0.7630, 0.6396]),
        NO_LIGHT,
      ],
      3,
    ),
    R::Soft => light_rig_preset(
      [0.3; 3],
      [
        adjusted_rig_light([0.8; 3], [-0.6897, 0.2484, -0.6802], 0.5, 0.5),
        NO_LIGHT,
        NO_LIGHT,
        NO_LIGHT,
      ],
      1,
    ),
    R::Harsh => light_rig_preset(
      [0.28; 3],
      [
        rig_light([0.88; 3], [0.6689, -0.6755, -0.3104]),
        rig_light([0.88; 3], [-0.5920, -0.7371, -0.3260]),
        NO_LIGHT,
        NO_LIGHT,
      ],
      2,
    ),
    R::Flood => light_rig_preset(
      [0.13; 3],
      [
        rig_light([1.1; 3], [0.5685, -0.7651, -0.3022]),
        rig_light([1.1; 3], [-0.2366, -0.9595, -0.1531]),
        rig_light([0.55; 3], [-0.8982, 0.1386, -0.4171]),
        NO_LIGHT,
      ],
      3,
    ),
    R::Contrasting => light_rig_preset(
      [1.0; 3],
      [
        nondiffuse_rig_light([1.0; 3], [0.0, -1.0, 0.0]),
        nondiffuse_rig_light([1.0; 3], [0.0, 1.0, 0.0]),
        NO_LIGHT,
        NO_LIGHT,
      ],
      2,
    ),
    R::Morning => light_rig_preset(
      [0.0; 3],
      [
        rig_light([0.669, 0.648, 0.596], D1),
        rig_light([0.459, 0.454, 0.385], D2),
        rig_light([0.9, 0.86, 0.83], D3),
        rig_light([0.911, 0.846, 0.728], D4),
      ],
      4,
    ),
    R::Sunrise => light_rig_preset(
      [0.0; 3],
      [
        rig_light([0.667, 0.63, 0.527], D1),
        rig_light([0.459, 0.459, 0.371], D2),
        rig_light([0.826, 0.712, 0.638], D3),
        rig_light([1.511, 1.319, 0.994], D4),
      ],
      4,
    ),
    R::Sunset => light_rig_preset(
      [0.0; 3],
      [
        rig_light([0.672, 0.169, 0.169], D1),
        rig_light([0.459, 0.448, 0.327], [0.0922, -0.3551, -0.9303]),
        rig_light([0.775, 0.612, 0.502], D3),
        rig_light([0.761, 0.69, 0.397], [-0.4240, 0.8891, 0.1722]),
      ],
      4,
    ),
    R::Chilly => light_rig_preset(
      [0.11; 3],
      [
        rig_light([0.31, 0.32, 0.32], D1),
        rig_light([0.45; 3], [-0.3539, -0.1505, -0.9231]),
        rig_light([1.03, 1.02, 1.15], D3),
        rig_light([0.41, 0.45, 0.48], [-0.5781, 0.7976, 0.1722]),
      ],
      4,
    ),
    R::Freezing => light_rig_preset(
      [0.0; 3],
      [
        rig_light([0.53, 0.567, 0.661], D1),
        rig_light([0.37, 0.461, 0.461], D2),
        rig_light([0.649, 0.638, 0.904], D3),
        rig_light([0.971, 1.19, 1.363], D4),
      ],
      4,
    ),
    R::Flat => light_rig_preset(
      [1.0; 3],
      [
        nondiffuse_rig_light([0.821; 3], [-0.9546, -0.1619, -0.2502]),
        nondiffuse_rig_light([2.072, 2.54, 2.91], [0.0009, 0.8605, 0.5095]),
        nondiffuse_rig_light([3.843; 3], D1),
        NO_LIGHT,
      ],
      3,
    ),
    R::TwoPoints => light_rig_preset(
      [0.25; 3],
      [
        rig_light([0.84; 3], [0.5266, -0.4089, -0.7454]),
        rig_light([0.3; 3], [-0.8983, 0.2365, -0.3704]),
        NO_LIGHT,
        NO_LIGHT,
      ],
      2,
    ),
    R::Glow => light_rig_preset(
      [1.0; 3],
      [
        rig_light([1.0; 3], [0.0, -1.0, 0.0]),
        rig_light([0.7; 3], [0.0, 1.0, 0.0]),
        NO_LIGHT,
        NO_LIGHT,
      ],
      2,
    ),
    R::BrightRoom => light_rig_preset(
      [1.5; 3],
      [
        rig_light([1.0; 3], [0.0, -1.0, 0.0]),
        nondiffuse_rig_light([1.0; 3], [0.8227, -0.1882, -0.5364]),
        rig_light([-0.5; 3], [0.0, 0.0, -1.0]),
        rig_light([0.5; 3], [0.0, 1.0, 0.0]),
      ],
      4,
    ),
    R::LegacyFlat1 => legacy_light_rig(0.305, 0.58, 0.58, 0.5, 0.5, 1),
    R::LegacyFlat2 => legacy_light_rig(0.305, 0.58, 0.58, 0.5, 0.5, 2),
    R::LegacyFlat3 => legacy_light_rig(0.305, 0.58, 0.58, 0.5, 0.5, 3),
    R::LegacyFlat4 => legacy_light_rig(0.305, 0.58, 0.58, 0.5, 0.5, 4),
    R::LegacyNormal1 => legacy_light_rig(0.153, 0.671, 0.366, 0.5, 0.5, 1),
    R::LegacyNormal2 => legacy_light_rig(0.153, 0.671, 0.366, 0.5, 0.5, 2),
    R::LegacyNormal3 => legacy_light_rig(0.153, 0.671, 0.366, 0.5, 0.5, 3),
    R::LegacyNormal4 => legacy_light_rig(0.153, 0.671, 0.366, 0.5, 0.5, 4),
    R::LegacyHarsh1 => legacy_light_rig(0.061, 0.793, 0.214, 1.0, 0.0, 1),
    R::LegacyHarsh2 => legacy_light_rig(0.061, 0.793, 0.214, 1.0, 0.0, 2),
    R::LegacyHarsh3 => legacy_light_rig(0.061, 0.793, 0.214, 1.0, 0.0, 3),
    R::LegacyHarsh4 => legacy_light_rig(0.061, 0.793, 0.214, 1.0, 0.0, 4),
  }
}

const fn legacy_light_rig(
  ambient: f32,
  key_color: f32,
  fill_color: f32,
  fill_scale: f32,
  fill_offset: f32,
  direction: u8,
) -> LightRigPreset {
  let key_direction = match direction {
    1 => [0.0, 0.0, -0.2],
    2 => [-1.0, -1.0, -0.2],
    3 => [0.0, -1.0, -0.2],
    _ => [1.0, -1.0, -0.2],
  };
  let fill_direction = if direction == 1 {
    [0.0, 0.0, -0.2]
  } else {
    [0.0, 1.0, -0.2]
  };
  light_rig_preset(
    [ambient; 3],
    [
      rig_light([key_color; 3], key_direction),
      adjusted_rig_light([fill_color; 3], fill_direction, fill_scale, fill_offset),
      NO_LIGHT,
      NO_LIGHT,
    ],
    2,
  )
}

fn scale_shade(shade: [f32; 3], scale: f32) -> [f32; 3] {
  shade.map(|channel| channel * scale)
}

fn clamp_shade_min(shade: [f32; 3], minimum: f32) -> [f32; 3] {
  shade.map(|channel| channel.max(minimum))
}

fn light_rig_direction_degrees(direction: a::LightRigDirectionValues) -> f32 {
  use a::LightRigDirectionValues as D;
  match direction {
    D::Top => 0.0,
    D::TopRight => 45.0,
    D::Right => 90.0,
    D::BottomRight => 135.0,
    D::Bottom => 180.0,
    D::BottomLeft => -135.0,
    D::Left => -90.0,
    D::TopLeft => -45.0,
  }
}

fn normalize3(vector: &mut [f32; 3]) {
  let length = dot3(*vector, *vector).sqrt();
  if length > f32::EPSILON {
    for value in vector {
      *value /= length;
    }
  }
}

fn dot3(left: [f32; 3], right: [f32; 3]) -> f32 {
  left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn rotate_x(vector: &mut [f32; 3], angle: f32) {
  let (sin, cos) = angle.sin_cos();
  (vector[1], vector[2]) = (
    vector[1] * cos - vector[2] * sin,
    vector[1] * sin + vector[2] * cos,
  );
}

fn rotate_y(vector: &mut [f32; 3], angle: f32) {
  let (sin, cos) = angle.sin_cos();
  (vector[0], vector[2]) = (
    vector[0] * cos + vector[2] * sin,
    -vector[0] * sin + vector[2] * cos,
  );
}

fn rotate_z(vector: &mut [f32; 3], angle: f32) {
  let (sin, cos) = angle.sin_cos();
  (vector[0], vector[1]) = (
    vector[0] * cos - vector[1] * sin,
    vector[0] * sin + vector[1] * cos,
  );
}

/// Resolves Office's automatic extrusion paint from the authored outline.
///
/// MS-OI29500 §20.1.5.7 makes the front fill the conceptual default, but
/// Office's fixed-output path uses the resolved line color when a line exists.
/// LibreOffice mirrors that producer behavior in
/// `oox/source/drawingml/shape.cxx` before falling back to the fill.
pub(crate) fn automatic_extrusion_color_from_items(
  items: &[DisplayItem<'_>],
) -> Option<Static3dColor> {
  items.iter().find_map(automatic_extrusion_color_from_item)
}

fn automatic_extrusion_color_from_item(item: &DisplayItem<'_>) -> Option<Static3dColor> {
  let color = match item {
    DisplayItem::Path(path) => path.stroke.as_ref().map(|stroke| stroke.color),
    DisplayItem::Rect(rect) => rect.stroke.as_ref().map(|stroke| stroke.color),
    DisplayItem::Line(line) => Some(line.stroke.color),
    DisplayItem::Group(group) => {
      return automatic_extrusion_color_from_items(&group.items);
    }
    DisplayItem::Text(_)
    | DisplayItem::Glyphs(_)
    | DisplayItem::Image(_)
    | DisplayItem::LinkArea(_)
    | DisplayItem::AnnotationHint(_)
    | DisplayItem::Clip(_)
    | DisplayItem::Transform(_) => None,
  }?;
  (color.a != 0).then_some(Static3dColor {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    alpha: color.a,
  })
}

fn average_extrusion_color(image: &RgbaImage) -> Static3dColor {
  let mut red = 0_u64;
  let mut green = 0_u64;
  let mut blue = 0_u64;
  let mut alpha = 0_u64;
  for pixel in image.pixels() {
    let weight = u64::from(pixel[3]);
    red += u64::from(pixel[0]) * weight;
    green += u64::from(pixel[1]) * weight;
    blue += u64::from(pixel[2]) * weight;
    alpha += weight;
  }
  let color = if alpha == 0 {
    RgbColor {
      r: 128,
      g: 128,
      b: 128,
    }
  } else {
    RgbColor {
      r: (red / alpha) as u8,
      g: (green / alpha) as u8,
      b: (blue / alpha) as u8,
    }
  };
  Static3dColor { color, alpha: 255 }
}

fn material_diffusion(material: Option<a::PresetMaterialTypeValues>) -> f32 {
  use a::PresetMaterialTypeValues as M;
  match material.unwrap_or(M::WarmMatte) {
    M::LegacyMetal => 0.666_992_2,
    M::DarkEdge => 0.7,
    M::Clear
    | M::Flat
    | M::LegacyMatte
    | M::LegacyPlastic
    | M::Matte
    | M::Metal
    | M::Plastic
    | M::Powder
    | M::SoftEdge
    | M::SoftMetal
    | M::TranslucentPowder
    | M::WarmMatte => 1.0,
    // LibreOffice renders this preset in wireframe shade mode. The raster
    // lowering retains a faint side face so the original front outline stays
    // visible rather than manufacturing a solid material.
    M::LegacyWireframe => 0.0,
  }
}

fn material_specularity(material: Option<a::PresetMaterialTypeValues>) -> f32 {
  use a::PresetMaterialTypeValues as M;
  // Direct scalar forms of MS-OI29500's Specular Color table. Shape-derived
  // metal colors are lowered separately when that material family is needed.
  match material.unwrap_or(M::WarmMatte) {
    M::LegacyMatte | M::Matte | M::LegacyWireframe => 0.0,
    M::Powder | M::TranslucentPowder | M::WarmMatte => 0.3,
    M::Clear | M::Plastic => 0.6,
    M::Flat => 0.8,
    M::DarkEdge | M::Metal | M::SoftEdge | M::SoftMetal => 1.0,
    M::LegacyMetal | M::LegacyPlastic => 1.0,
  }
}

fn material_blinn_highlight(material: Option<a::PresetMaterialTypeValues>) -> bool {
  use a::PresetMaterialTypeValues as M;
  matches!(
    material.unwrap_or(M::WarmMatte),
    M::LegacyMatte
      | M::LegacyMetal
      | M::LegacyPlastic
      | M::LegacyWireframe
      | M::Powder
      | M::TranslucentPowder
  )
}

fn material_specular_power(material: Option<a::PresetMaterialTypeValues>) -> f32 {
  use a::PresetMaterialTypeValues as M;
  match material.unwrap_or(M::WarmMatte) {
    M::LegacyMatte | M::Matte | M::LegacyWireframe => 0.0,
    M::SoftMetal | M::WarmMatte => 8.0,
    M::Powder | M::TranslucentPowder => 10.0,
    M::Metal | M::Plastic => 12.0,
    M::Clear => 20.0,
    M::LegacyMetal | M::LegacyPlastic => 32.0,
    M::DarkEdge | M::SoftEdge => 35.0,
    M::Flat => 50.0,
  }
}

fn project_local(
  projection: Static3dProjection,
  x: f32,
  y: f32,
  z: f32,
  width: f32,
  height: f32,
) -> (f32, f32) {
  project_local_pixels(projection, x, y, z, width, height, 1.0)
}

fn project_local_pixels(
  projection: Static3dProjection,
  x: f32,
  y: f32,
  z: f32,
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> (f32, f32) {
  let homography = plane_homography(projection, z, width, height, pixels_per_point);
  map_homogeneous(homography, x, y)
}

fn plane_homography(
  projection: Static3dProjection,
  z: f32,
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> [[f32; 3]; 3] {
  let rotation = projection.rotation;
  if projection.parallel {
    return [
      [
        rotation[0][0],
        rotation[0][1],
        (rotation[0][2] + projection.skew_x_per_depth) * z,
      ],
      [
        rotation[1][0],
        rotation[1][1],
        (rotation[1][2] + projection.skew_y_per_depth) * z,
      ],
      [0.0, 0.0, 1.0],
    ];
  }

  let viewpoint_z = projection
    .perspective_distance_pt
    .unwrap_or(25_000.0 * 72.0 / 2_540.0)
    * pixels_per_point;
  let viewpoint_x = projection.origin_x * width + projection.viewpoint_x_pt * pixels_per_point;
  let viewpoint_y = projection.origin_y * height + projection.viewpoint_y_pt * pixels_per_point;
  [
    [
      viewpoint_z * rotation[0][0] - viewpoint_x * rotation[2][0],
      viewpoint_z * rotation[0][1] - viewpoint_x * rotation[2][1],
      z * (viewpoint_z * rotation[0][2] - viewpoint_x * rotation[2][2]),
    ],
    [
      viewpoint_z * rotation[1][0] - viewpoint_y * rotation[2][0],
      viewpoint_z * rotation[1][1] - viewpoint_y * rotation[2][1],
      z * (viewpoint_z * rotation[1][2] - viewpoint_y * rotation[2][2]),
    ],
    [
      -rotation[2][0],
      -rotation[2][1],
      viewpoint_z - z * rotation[2][2],
    ],
  ]
}

fn map_homogeneous(matrix: [[f32; 3]; 3], x: f32, y: f32) -> (f32, f32) {
  let denominator = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2];
  if denominator.abs() <= 1.0e-6 {
    return (x, y);
  }
  (
    (matrix[0][0] * x + matrix[0][1] * y + matrix[0][2]) / denominator,
    (matrix[1][0] * x + matrix[1][1] * y + matrix[1][2]) / denominator,
  )
}

fn projection_plane_is_identity(
  projection: Static3dProjection,
  z: f32,
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> bool {
  let matrix = plane_homography(projection, z, width, height, pixels_per_point);
  let normalized = if matrix[2][2].abs() > 1.0e-6 {
    let scale = matrix[2][2];
    matrix.map(|row| row.map(|value| value / scale))
  } else {
    matrix
  };
  let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
  normalized
    .iter()
    .flatten()
    .zip(identity.iter().flatten())
    .all(|(actual, expected)| (actual - expected).abs() < 0.001)
}

fn projected_depth_steps(
  projection: Static3dProjection,
  front_z: f32,
  back_z: f32,
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> u32 {
  let mut travel: f32 = 0.0;
  for x in [-width * 0.5, width * 0.5] {
    for y in [-height * 0.5, height * 0.5] {
      let front = project_local_pixels(projection, x, y, front_z, width, height, pixels_per_point);
      let back = project_local_pixels(projection, x, y, back_z, width, height, pixels_per_point);
      travel = travel.max((back.0 - front.0).hypot(back.1 - front.1));
    }
  }
  travel.ceil().clamp(1.0, 256.0) as u32
}

fn composite_projected_image(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  projection: Static3dProjection,
  z: f32,
  bounds: (i32, i32, i32, i32),
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  tint: Option<(Static3dColor, [f32; 3])>,
) {
  let (left, top, right, bottom) = bounds;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);
  let matrix = plane_homography(projection, z, width, height, pixels_per_point);
  let Some(inverse) = inverse_3x3(matrix) else {
    return;
  };

  let projected_corners = [
    map_homogeneous(matrix, -width * 0.5, -height * 0.5),
    map_homogeneous(matrix, width * 0.5, -height * 0.5),
    map_homogeneous(matrix, width * 0.5, height * 0.5),
    map_homogeneous(matrix, -width * 0.5, height * 0.5),
  ];
  let min_x = projected_corners
    .iter()
    .map(|point| point.0 + center_x)
    .fold(f32::INFINITY, f32::min)
    .floor() as i32
    - 1;
  let min_y = projected_corners
    .iter()
    .map(|point| point.1 + center_y)
    .fold(f32::INFINITY, f32::min)
    .floor() as i32
    - 1;
  let max_x = projected_corners
    .iter()
    .map(|point| point.0 + center_x)
    .fold(f32::NEG_INFINITY, f32::max)
    .ceil() as i32
    + 1;
  let max_y = projected_corners
    .iter()
    .map(|point| point.1 + center_y)
    .fold(f32::NEG_INFINITY, f32::max)
    .ceil() as i32
    + 1;
  let min_x = min_x.max(0);
  let min_y = min_y.max(0);
  let max_x = max_x.min(destination.width() as i32 - 1);
  let max_y = max_y.min(destination.height() as i32 - 1);

  for target_y in min_y..=max_y {
    for target_x in min_x..=max_x {
      let (source_local_x, source_local_y) = map_homogeneous(
        inverse,
        target_x as f32 + 0.5 - center_x,
        target_y as f32 + 0.5 - center_y,
      );
      // The projection is expressed in geometric raster coordinates, where
      // integer values are pixel edges. `image` sampling uses integer pixel
      // indices, whose geometric centers are at index + 0.5.
      let source_x = center_x + source_local_x - 0.5;
      let source_y = center_y + source_local_y - 0.5;
      if source_x < left as f32 - 0.5
        || source_y < top as f32 - 0.5
        || source_x > right as f32 + 0.5
        || source_y > bottom as f32 + 0.5
      {
        continue;
      }
      let Some(mut pixel) = sample_bilinear(source, source_x, source_y) else {
        continue;
      };
      if let Some((color, shade)) = tint {
        pixel = shaded_pixel(
          color,
          shade,
          ((u16::from(pixel[3]) * u16::from(color.alpha) + 127) / 255) as u8,
        );
      }
      if pixel[3] != 0 {
        blend_over(
          destination.get_pixel_mut(target_x as u32, target_y as u32),
          pixel,
        );
      }
    }
  }
}

fn inverse_3x3(matrix: [[f32; 3]; 3]) -> Option<[[f32; 3]; 3]> {
  let determinant = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
    - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
    + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);
  if determinant.abs() <= 1.0e-8 {
    return None;
  }
  let inverse_determinant = determinant.recip();
  Some([
    [
      (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1]) * inverse_determinant,
      (matrix[0][2] * matrix[2][1] - matrix[0][1] * matrix[2][2]) * inverse_determinant,
      (matrix[0][1] * matrix[1][2] - matrix[0][2] * matrix[1][1]) * inverse_determinant,
    ],
    [
      (matrix[1][2] * matrix[2][0] - matrix[1][0] * matrix[2][2]) * inverse_determinant,
      (matrix[0][0] * matrix[2][2] - matrix[0][2] * matrix[2][0]) * inverse_determinant,
      (matrix[0][2] * matrix[1][0] - matrix[0][0] * matrix[1][2]) * inverse_determinant,
    ],
    [
      (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]) * inverse_determinant,
      (matrix[0][1] * matrix[2][0] - matrix[0][0] * matrix[2][1]) * inverse_determinant,
      (matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) * inverse_determinant,
    ],
  ])
}

fn sample_bilinear(image: &RgbaImage, x: f32, y: f32) -> Option<Rgba<u8>> {
  if x < -0.5 || y < -0.5 || x > image.width() as f32 - 0.5 || y > image.height() as f32 - 0.5 {
    return None;
  }
  let x0 = x.floor() as i32;
  let y0 = y.floor() as i32;
  let fx = x - x0 as f32;
  let fy = y - y0 as f32;
  let samples = [
    (x0, y0, (1.0 - fx) * (1.0 - fy)),
    (x0 + 1, y0, fx * (1.0 - fy)),
    (x0, y0 + 1, (1.0 - fx) * fy),
    (x0 + 1, y0 + 1, fx * fy),
  ];
  let mut alpha = 0.0;
  let mut premultiplied = [0.0; 3];
  for (sample_x, sample_y, weight) in samples {
    if sample_x < 0
      || sample_y < 0
      || sample_x >= image.width() as i32
      || sample_y >= image.height() as i32
    {
      continue;
    }
    let pixel = image.get_pixel(sample_x as u32, sample_y as u32);
    let sample_alpha = f32::from(pixel[3]) / 255.0;
    alpha += sample_alpha * weight;
    for channel in 0..3 {
      premultiplied[channel] += f32::from(pixel[channel]) * sample_alpha * weight;
    }
  }
  if alpha <= f32::EPSILON {
    return None;
  }
  Some(Rgba([
    (premultiplied[0] / alpha).round().clamp(0.0, 255.0) as u8,
    (premultiplied[1] / alpha).round().clamp(0.0, 255.0) as u8,
    (premultiplied[2] / alpha).round().clamp(0.0, 255.0) as u8,
    (alpha * 255.0).round().clamp(0.0, 255.0) as u8,
  ]))
}

fn composite_extrusion_edges(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  bounds: (i32, i32, i32, i32),
  model_surface: Static3dSurface,
  projection: Static3dProjection,
  front_z: f32,
  back_z: f32,
  pixels_per_point: f32,
  steps: u32,
  tint: Static3dColor,
  scene: &a::Scene3DType,
  material: Option<a::PresetMaterialTypeValues>,
  wireframe: bool,
) {
  let _ = bounds;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);

  if !wireframe {
    // Sweep each exposed source-mask edge as one projected quadrilateral.
    // tiny-skia coverage-rasterizes the full side surface; the aliased
    // interior prevents adjacent mesh cells from creating coverage seams.
    let Some(mut side_layer) = Pixmap::new(destination.width(), destination.height()) else {
      return;
    };
    let source_alpha = |sample_x: i32, sample_y: i32| {
      if sample_x < 0
        || sample_y < 0
        || sample_x >= source.width() as i32
        || sample_y >= source.height() as i32
      {
        0
      } else {
        source.get_pixel(sample_x as u32, sample_y as u32)[3]
      }
    };
    for (x, y, source_pixel) in source.enumerate_pixels() {
      if source_pixel[3] == 0 {
        continue;
      }
      let x = x as i32;
      let y = y as i32;
      let model_normal = alpha_boundary_normal(source, x, y);
      let smooth_facing = (model_normal[0].hypot(model_normal[1]) > f32::EPSILON).then(|| {
        surface_faces_camera(
          projection,
          [model_normal[0], model_normal[1], 0.0],
          [
            x as f32 + 0.5 - center_x,
            y as f32 + 0.5 - center_y,
            (front_z + back_z) * 0.5,
          ],
          width,
          height,
          pixels_per_point,
        )
      });
      let surface_normal =
        lighting_surface_normal(scene, projection, [model_normal[0], model_normal[1], 0.0]);
      let shade = scale_shade(
        light_rig_surface_shade(scene, surface_normal),
        material_diffusion(material),
      );
      let view_direction = surface_view_direction(
        scene,
        projection,
        [
          x as f32 + 0.5 - center_x,
          y as f32 + 0.5 - center_y,
          (front_z + back_z) * 0.5,
        ],
        width,
        height,
        pixels_per_point,
      );
      let specular = light_rig_surface_specular(scene, surface_normal, view_direction, material);
      let color = shaded_pixel_with_specular(tint, shade, specular, tint.alpha);
      let exposed_edges = [
        (
          source_alpha(x - 1, y) == 0,
          (x as f32, y as f32),
          (x as f32, y as f32 + 1.0),
          [-1.0, 0.0, 0.0],
        ),
        (
          source_alpha(x + 1, y) == 0,
          (x as f32 + 1.0, y as f32 + 1.0),
          (x as f32 + 1.0, y as f32),
          [1.0, 0.0, 0.0],
        ),
        (
          source_alpha(x, y - 1) == 0,
          (x as f32 + 1.0, y as f32),
          (x as f32, y as f32),
          [0.0, -1.0, 0.0],
        ),
        (
          source_alpha(x, y + 1) == 0,
          (x as f32, y as f32 + 1.0),
          (x as f32 + 1.0, y as f32 + 1.0),
          [0.0, 1.0, 0.0],
        ),
      ];
      for (exposed, first, second, face_normal) in exposed_edges {
        if !exposed {
          continue;
        }
        let facing = smooth_facing.unwrap_or_else(|| {
          surface_faces_camera(
            projection,
            face_normal,
            [
              (first.0 + second.0) * 0.5 - center_x,
              (first.1 + second.1) * 0.5 - center_y,
              (front_z + back_z) * 0.5,
            ],
            width,
            height,
            pixels_per_point,
          )
        });
        if !facing {
          continue;
        }
        let project = |point: (f32, f32), z| {
          let projected = project_local_pixels(
            projection,
            point.0 - center_x,
            point.1 - center_y,
            z,
            width,
            height,
            pixels_per_point,
          );
          (center_x + projected.0, center_y + projected.1)
        };
        let front_first = project(first, front_z);
        let front_second = project(second, front_z);
        let back_second = project(second, back_z);
        let back_first = project(first, back_z);
        let mut path = PathBuilder::new();
        path.move_to(front_first.0, front_first.1);
        path.line_to(front_second.0, front_second.1);
        path.line_to(back_second.0, back_second.1);
        path.line_to(back_first.0, back_first.1);
        path.close();
        let Some(path) = path.finish() else {
          continue;
        };
        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
        side_layer.fill_path(
          &path,
          &paint,
          FillRule::Winding,
          Transform::identity(),
          None,
        );
        paint.anti_alias = false;
        side_layer.fill_path(
          &path,
          &paint,
          FillRule::Winding,
          Transform::identity(),
          None,
        );
      }
    }
    for (target, source) in destination.pixels_mut().zip(side_layer.pixels()) {
      let source = source.demultiply();
      if source.alpha() != 0 {
        blend_over(
          target,
          Rgba([source.red(), source.green(), source.blue(), source.alpha()]),
        );
      }
    }
    return;
  }

  for (x, y, source_pixel) in source.enumerate_pixels() {
    if source_pixel[3] == 0 || !is_alpha_boundary(source, x as i32, y as i32) {
      continue;
    }
    let model_normal = alpha_boundary_normal(source, x as i32, y as i32);
    let surface_normal =
      lighting_surface_normal(scene, projection, [model_normal[0], model_normal[1], 0.0]);
    let mut shade = scale_shade(
      light_rig_surface_shade(scene, surface_normal),
      material_diffusion(material),
    );
    if wireframe {
      shade = clamp_shade_min(shade, 0.35);
    }
    for step in (1..steps).rev() {
      let fraction = step as f32 / steps as f32;
      let z = front_z + (back_z - front_z) * fraction;
      let (projected_x, projected_y) = project_local_pixels(
        projection,
        x as f32 + 0.5 - center_x,
        y as f32 + 0.5 - center_y,
        z,
        width,
        height,
        pixels_per_point,
      );
      let target_x = (center_x + projected_x).round() as i32;
      let target_y = (center_y + projected_y).round() as i32;
      if target_x < 0
        || target_y < 0
        || target_x >= destination.width() as i32
        || target_y >= destination.height() as i32
      {
        continue;
      }
      let alpha = ((u16::from(source_pixel[3]) * u16::from(tint.alpha) + 127) / 255) as u8;
      blend_over(
        destination.get_pixel_mut(target_x as u32, target_y as u32),
        shaded_pixel(tint, shade, alpha),
      );
    }
  }
}

fn alpha_boundary_normal(image: &RgbaImage, x: i32, y: i32) -> [f32; 2] {
  let alpha = |sample_x: i32, sample_y: i32| {
    if sample_x < 0
      || sample_y < 0
      || sample_x >= image.width() as i32
      || sample_y >= image.height() as i32
    {
      0.0
    } else {
      f32::from(image.get_pixel(sample_x as u32, sample_y as u32)[3])
    }
  };
  // A one-pixel central difference follows the stair steps of a rasterized
  // curve and creates alternating lighting bands on the swept side mesh.
  // Use a compact separable derivative over the antialiased alpha coverage,
  // equivalent to a radius-two Sobel normal, so adjacent contour cells share
  // the smooth vector of the authored curve.
  let mut normal = [0.0, 0.0];
  for offset in -2_i32..=2 {
    let weight = (3 - offset.abs()) as f32;
    normal[0] += weight * (alpha(x - 2, y + offset) - alpha(x + 2, y + offset));
    normal[1] += weight * (alpha(x + offset, y - 2) - alpha(x + offset, y + 2));
  }
  let length = normal[0].hypot(normal[1]);
  if length > f32::EPSILON {
    normal[0] /= length;
    normal[1] /= length;
  }
  normal
}

fn transform_normal(matrix: [[f32; 3]; 3], normal: [f32; 3]) -> [f32; 3] {
  let mut transformed = [
    matrix[0][0] * normal[0] + matrix[0][1] * normal[1] + matrix[0][2] * normal[2],
    matrix[1][0] * normal[0] + matrix[1][1] * normal[1] + matrix[1][2] * normal[2],
    matrix[2][0] * normal[0] + matrix[2][1] * normal[1] + matrix[2][2] * normal[2],
  ];
  normalize3(&mut transformed);
  transformed
}

fn lighting_surface_normal(
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  model_normal: [f32; 3],
) -> [f32; 3] {
  if is_legacy_camera(scene.camera.preset) {
    // Legacy presets rotate the shape relative to a stationary camera and
    // light rig. Modern presets move the camera while the rig stays fixed to
    // the shape, so their illumination is evaluated in model coordinates.
    transform_normal(projection.rotation, model_normal)
  } else {
    model_normal
  }
}

fn surface_faces_camera(
  projection: Static3dProjection,
  model_normal: [f32; 3],
  model_point: [f32; 3],
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> bool {
  let camera_normal = transform_normal(projection.rotation, model_normal);
  let view_direction =
    camera_view_direction(projection, model_point, width, height, pixels_per_point);
  dot3(camera_normal, view_direction) > 0.0
}

fn camera_view_direction(
  projection: Static3dProjection,
  model_point: [f32; 3],
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> [f32; 3] {
  if projection.parallel {
    return [0.0, 0.0, 1.0];
  }
  let rotated_point = [
    dot3(projection.rotation[0], model_point),
    dot3(projection.rotation[1], model_point),
    dot3(projection.rotation[2], model_point),
  ];
  let viewpoint = [
    projection.origin_x * width + projection.viewpoint_x_pt * pixels_per_point,
    projection.origin_y * height + projection.viewpoint_y_pt * pixels_per_point,
    projection
      .perspective_distance_pt
      .unwrap_or(25_000.0 * 72.0 / 2_540.0)
      * pixels_per_point,
  ];
  let mut direction = [
    viewpoint[0] - rotated_point[0],
    viewpoint[1] - rotated_point[1],
    viewpoint[2] - rotated_point[2],
  ];
  normalize3(&mut direction);
  direction
}

fn surface_view_direction(
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  model_point: [f32; 3],
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> [f32; 3] {
  let camera_direction =
    camera_view_direction(projection, model_point, width, height, pixels_per_point);
  if is_legacy_camera(scene.camera.preset) {
    camera_direction
  } else {
    let mut model_direction = [
      projection.rotation[0][0] * camera_direction[0]
        + projection.rotation[1][0] * camera_direction[1]
        + projection.rotation[2][0] * camera_direction[2],
      projection.rotation[0][1] * camera_direction[0]
        + projection.rotation[1][1] * camera_direction[1]
        + projection.rotation[2][1] * camera_direction[2],
      projection.rotation[0][2] * camera_direction[0]
        + projection.rotation[1][2] * camera_direction[1]
        + projection.rotation[2][2] * camera_direction[2],
    ];
    normalize3(&mut model_direction);
    model_direction
  }
}

fn shaded_pixel(color: Static3dColor, shade: [f32; 3], alpha: u8) -> Rgba<u8> {
  Rgba([
    shade_gouraud_channel(color.color.r, shade[0]),
    shade_gouraud_channel(color.color.g, shade[1]),
    shade_gouraud_channel(color.color.b, shade[2]),
    alpha,
  ])
}

fn shaded_pixel_with_specular(
  color: Static3dColor,
  shade: [f32; 3],
  specular: [f32; 3],
  alpha: u8,
) -> Rgba<u8> {
  Rgba([
    shade_gouraud_channel_with_specular(color.color.r, shade[0], specular[0]),
    shade_gouraud_channel_with_specular(color.color.g, shade[1], specular[1]),
    shade_gouraud_channel_with_specular(color.color.b, shade[2], specular[2]),
    alpha,
  ])
}

fn shade_gouraud_channel(channel: u8, shade: f32) -> u8 {
  // Office's graphics team identifies the preset calculation as its D3D9
  // Gouraud shading equation. The fixed-function diffuse stage multiplies
  // normalized material and light color components directly; it does not
  // insert an sRGB decode/encode around that product.
  (f32::from(channel) * shade).round().clamp(0.0, 255.0) as u8
}

fn shade_gouraud_channel_with_specular(channel: u8, shade: f32, specular: f32) -> u8 {
  (f32::from(channel) * shade + 255.0 * specular)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn alpha_bounds(image: &RgbaImage) -> Option<(i32, i32, i32, i32)> {
  let mut left = image.width() as i32;
  let mut top = image.height() as i32;
  let mut right = -1;
  let mut bottom = -1;
  for (x, y, pixel) in image.enumerate_pixels() {
    if pixel[3] == 0 {
      continue;
    }
    left = left.min(x as i32);
    top = top.min(y as i32);
    right = right.max(x as i32);
    bottom = bottom.max(y as i32);
  }
  (right >= left && bottom >= top).then_some((left, top, right, bottom))
}

fn composite_outline(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  radius: i32,
  color: Static3dColor,
) {
  let radius_squared = radius * radius;
  for (x, y, pixel) in source.enumerate_pixels() {
    if pixel[3] == 0 || !is_alpha_boundary(source, x as i32, y as i32) {
      continue;
    }
    for offset_y in -radius..=radius {
      for offset_x in -radius..=radius {
        if offset_x * offset_x + offset_y * offset_y > radius_squared {
          continue;
        }
        let target_x = x as i32 + offset_x;
        let target_y = y as i32 + offset_y;
        if target_x < 0
          || target_y < 0
          || target_x >= destination.width() as i32
          || target_y >= destination.height() as i32
        {
          continue;
        }
        let alpha = ((u16::from(pixel[3]) * u16::from(color.alpha) + 127) / 255) as u8;
        blend_over(
          destination.get_pixel_mut(target_x as u32, target_y as u32),
          Rgba([color.color.r, color.color.g, color.color.b, alpha]),
        );
      }
    }
  }
}

fn is_alpha_boundary(image: &RgbaImage, x: i32, y: i32) -> bool {
  const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
  NEIGHBORS.iter().any(|&(offset_x, offset_y)| {
    let sample_x = x + offset_x;
    let sample_y = y + offset_y;
    sample_x < 0
      || sample_y < 0
      || sample_x >= image.width() as i32
      || sample_y >= image.height() as i32
      || image.get_pixel(sample_x as u32, sample_y as u32)[3] == 0
  })
}

fn composite_bevel(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  width: i32,
  height: f32,
  scene: &a::Scene3DType,
  material: Option<a::PresetMaterialTypeValues>,
  back_face: bool,
) {
  let is_inside = |x: i32, y: i32| -> bool {
    x >= 0
      && y >= 0
      && x < source.width() as i32
      && y < source.height() as i32
      && source.get_pixel(x as u32, y as u32)[3] != 0
  };
  let distance_to_edge = |x: i32, y: i32, dx: i32, dy: i32| -> i32 {
    for distance in 1..=width {
      if !is_inside(x + dx * distance, y + dy * distance) {
        return distance - 1;
      }
    }
    width
  };
  let diffusion = material_diffusion(material);
  let specularity = material_specularity(material);
  // OOXML shape coordinates and LibreOffice's Scene3DHelper use +z toward the
  // observer. The front bevel therefore has a positive-z normal; the back
  // bevel uses the opposite orientation. Rig vectors describe the direction
  // in which light travels and are negated separately by the lighting code.
  let z = if back_face {
    -width as f32
  } else {
    width as f32
  };
  for y in 0..source.height() as i32 {
    for x in 0..source.width() as i32 {
      let pixel = source.get_pixel(x as u32, y as u32);
      if pixel[3] == 0 {
        continue;
      }
      let distances = [
        distance_to_edge(x, y, -1, 0),
        distance_to_edge(x, y, 1, 0),
        distance_to_edge(x, y, 0, -1),
        distance_to_edge(x, y, 0, 1),
      ];
      let distance = *distances.iter().min().unwrap_or(&width);
      if distance >= width {
        continue;
      }
      let mut normal = [0.0_f32; 3];
      if distances[0] == distance {
        normal[0] -= height;
      }
      if distances[1] == distance {
        normal[0] += height;
      }
      if distances[2] == distance {
        normal[1] -= height;
      }
      if distances[3] == distance {
        normal[1] += height;
      }
      normal[2] = z;
      normalize3(&mut normal);
      let shade = light_rig_surface_shade(scene, normal).map(|value| {
        let diffuse = value * diffusion;
        if diffuse > 1.0 {
          1.0 + (diffuse - 1.0) * (1.0 + specularity)
        } else {
          diffuse
        }
      });
      let t = distance as f32 / width.max(1) as f32;
      let weight = 1.0 - t * t * (3.0 - 2.0 * t);
      let target = destination.get_pixel_mut(x as u32, y as u32);
      for channel in 0..3 {
        let lit = f32::from(pixel[channel]) * shade[channel];
        target[channel] = (f32::from(pixel[channel]) + (lit - f32::from(pixel[channel])) * weight)
          .round()
          .clamp(0.0, 255.0) as u8;
      }
      target[3] = pixel[3];
    }
  }
}

fn composite_image(destination: &mut RgbaImage, source: &RgbaImage) {
  for (target, source) in destination.pixels_mut().zip(source.pixels()) {
    blend_over(target, *source);
  }
}

fn blend_over(destination: &mut Rgba<u8>, source: Rgba<u8>) {
  let source_alpha = f32::from(source[3]) / 255.0;
  let destination_alpha = f32::from(destination[3]) / 255.0;
  let output_alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
  if output_alpha <= f32::EPSILON {
    *destination = Rgba([0, 0, 0, 0]);
    return;
  }
  for channel in 0..3 {
    destination[channel] = ((f32::from(source[channel]) * source_alpha
      + f32::from(destination[channel]) * destination_alpha * (1.0 - source_alpha))
      / output_alpha)
      .round()
      .clamp(0.0, 255.0) as u8;
  }
  destination[3] = (output_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
}

fn oox_rotation_matrix(latitude: i32, longitude: i32, revolution: i32) -> ([[f64; 3]; 3], f64) {
  let latitude = (f64::from(latitude) / 60_000.0).to_radians();
  let longitude = (f64::from(longitude) / 60_000.0).to_radians();
  let revolution = (f64::from(revolution) / 60_000.0).to_radians();
  let (sin_x, cos_x) = latitude.sin_cos();
  let (sin_y, cos_y) = longitude.sin_cos();
  let (sin_z, cos_z) = revolution.sin_cos();
  let x = [[1.0, 0.0, 0.0], [0.0, cos_x, sin_x], [0.0, -sin_x, cos_x]];
  let y = [[cos_y, 0.0, -sin_y], [0.0, 1.0, 0.0], [sin_y, 0.0, cos_y]];
  let z = [[cos_z, sin_z, 0.0], [-sin_z, cos_z, 0.0], [0.0, 0.0, 1.0]];
  let matrix = multiply(z, multiply(x, y));
  let y_angle = (-matrix[0][2]).asin();
  let face_rotation = if y_angle.cos().abs() <= f64::EPSILON {
    matrix[2][1].atan2(matrix[1][1])
  } else {
    (matrix[0][1] / y_angle.cos()).atan2(matrix[0][0] / y_angle.cos())
  };
  (matrix, face_rotation)
}

fn multiply(left: [[f64; 3]; 3], right: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
  let mut result = [[0.0; 3]; 3];
  for row in 0..3 {
    for column in 0..3 {
      result[row][column] = (0..3)
        .map(|index| left[row][index] * right[index][column])
        .sum();
    }
  }
  result
}

fn camera_preset(preset: a::PresetCameraValues) -> CameraPreset {
  use a::PresetCameraValues as P;
  match preset {
    P::IsometricBottomDown => CameraPreset::angles(true, 2_124_000, 18_882_000, 17_988_000),
    P::IsometricBottomUp => CameraPreset::angles(true, 2_124_000, 2_718_000, 3_612_000),
    P::IsometricLeftDown => CameraPreset::angles(true, 2_100_000, 2_700_000, 0),
    P::IsometricLeftUp => CameraPreset::angles(true, 19_500_000, 2_700_000, 0),
    P::IsometricOffAxis1Left => CameraPreset::angles(true, 1_080_000, 3_840_000, 0),
    P::IsometricOffAxis1Right => CameraPreset::angles(true, 1_080_000, 20_040_000, 0),
    P::IsometricOffAxis1Top => CameraPreset::angles(true, 18_078_000, 18_390_000, 3_456_000),
    P::IsometricOffAxis2Left => CameraPreset::angles(true, 1_080_000, 1_560_000, 0),
    P::IsometricOffAxis2Right => CameraPreset::angles(true, 1_080_000, 17_760_000, 0),
    P::IsometricOffAxis2Top => CameraPreset::angles(true, 18_078_000, 3_210_000, 18_144_000),
    P::IsometricOffAxis3Bottom => CameraPreset::angles(true, 3_522_000, 18_390_000, 18_144_000),
    P::IsometricOffAxis3Left => CameraPreset::angles(true, 20_520_000, 3_840_000, 0),
    P::IsometricOffAxis3Right => CameraPreset::angles(true, 20_520_000, 20_040_000, 0),
    P::IsometricOffAxis4Bottom => CameraPreset::angles(true, 3_522_000, 3_210_000, 3_456_000),
    P::IsometricOffAxis4Left => CameraPreset::angles(true, 20_520_000, 1_560_000, 0),
    P::IsometricOffAxis4Right => CameraPreset::angles(true, 20_520_000, 17_760_000, 0),
    P::IsometricRightDown => CameraPreset::angles(true, 19_500_000, 18_900_000, 0),
    P::IsometricRightUp => CameraPreset::angles(true, 2_100_000, 18_900_000, 0),
    P::IsometricTopDown => CameraPreset::angles(true, 19_476_000, 2_718_000, 17_988_000),
    P::IsometricTopUp => CameraPreset::angles(true, 19_476_000, 18_882_000, 3_612_000),
    P::LegacyObliqueBottom => CameraPreset::oblique(0.0, 0.5, 50.0, 90.0),
    P::LegacyObliqueBottomLeft => CameraPreset::oblique(-0.5, 0.5, 50.0, 45.0),
    P::LegacyObliqueBottomRight => CameraPreset::oblique(0.5, 0.5, 50.0, 135.0),
    P::LegacyObliqueFront => CameraPreset::oblique(0.0, 0.0, 0.0, 0.0),
    P::LegacyObliqueLeft => CameraPreset::oblique(-0.5, 0.0, 50.0, -360.0),
    P::LegacyObliqueRight => CameraPreset::oblique(0.5, 0.0, 50.0, 180.0),
    P::LegacyObliqueTop => CameraPreset::oblique(0.0, -0.5, 50.0, -90.0),
    P::LegacyObliqueTopLeft => CameraPreset::oblique(-0.5, -0.5, 50.0, -45.0),
    P::LegacyObliqueTopRight => CameraPreset::oblique(0.5, -0.5, 50.0, -135.0),
    P::LegacyPerspectiveBottom => CameraPreset::legacy_perspective(0.0, 3_472.0),
    P::LegacyPerspectiveBottomLeft => CameraPreset::legacy_perspective(-3_472.0, 3_472.0),
    P::LegacyPerspectiveBottomRight => CameraPreset::legacy_perspective(3_472.0, 3_472.0),
    P::LegacyPerspectiveFront => CameraPreset::legacy_perspective(0.0, 0.0),
    P::LegacyPerspectiveLeft => CameraPreset::legacy_perspective(-3_472.0, 0.0),
    P::LegacyPerspectiveRight => CameraPreset::legacy_perspective(3_472.0, 0.0),
    P::LegacyPerspectiveTop => CameraPreset::legacy_perspective(0.0, -3_472.0),
    P::LegacyPerspectiveTopLeft => CameraPreset::legacy_perspective(-3_472.0, -3_472.0),
    P::LegacyPerspectiveTopRight => CameraPreset::legacy_perspective(3_472.0, -3_472.0),
    P::ObliqueBottom => CameraPreset::oblique(0.0, 0.5, 30.0, 90.0),
    P::ObliqueBottomLeft => CameraPreset::oblique(-0.5, 0.5, 30.0, 45.0),
    P::ObliqueBottomRight => CameraPreset::oblique(0.5, 0.5, 30.0, 135.0),
    P::ObliqueLeft => CameraPreset::oblique(-0.5, 0.0, 30.0, -360.0),
    P::ObliqueRight => CameraPreset::oblique(0.5, 0.0, 30.0, 180.0),
    P::ObliqueTop => CameraPreset::oblique(0.0, -0.5, 30.0, -90.0),
    P::ObliqueTopLeft => CameraPreset::oblique(-0.5, -0.5, 30.0, -45.0),
    P::ObliqueTopRight => CameraPreset::oblique(0.5, -0.5, 30.0, -135.0),
    P::OrthographicFront => CameraPreset::angles(true, 0, 0, 0),
    P::PerspectiveAbove => CameraPreset::angles(false, 20_400_000, 0, 0),
    P::PerspectiveAboveLeftFacing => CameraPreset::angles(false, 2_358_000, 858_000, 20_466_000),
    P::PerspectiveAboveRightFacing => CameraPreset::angles(false, 2_358_000, 20_742_000, 1_134_000),
    P::PerspectiveBelow => CameraPreset::angles(false, 1_200_000, 0, 0),
    P::PerspectiveContrastingLeftFacing => {
      CameraPreset::angles(false, 624_000, 2_634_000, 21_384_000)
    }
    P::PerspectiveContrastingRightFacing => {
      CameraPreset::angles(false, 624_000, 18_966_000, 216_000)
    }
    P::PerspectiveFront => CameraPreset::angles(false, 0, 0, 0),
    P::PerspectiveHeroicExtremeLeftFacing => {
      let mut value = CameraPreset::angles(false, 486_000, 2_070_000, 21_426_000);
      value.viewpoint_z = 18_981.0;
      value
    }
    P::PerspectiveHeroicExtremeRightFacing => {
      let mut value = CameraPreset::angles(false, 486_000, 19_530_000, 174_000);
      value.viewpoint_z = 18_981.0;
      value
    }
    P::PerspectiveHeroicLeftFacing => CameraPreset::angles(false, 20_940_000, 858_000, 156_000),
    P::PerspectiveHeroicRightFacing => {
      CameraPreset::angles(false, 20_940_000, 20_742_000, 21_444_000)
    }
    P::PerspectiveLeft => CameraPreset::angles(false, 0, 1_200_000, 0),
    P::PerspectiveRelaxed => CameraPreset::angles(false, 18_576_000, 0, 0),
    P::PerspectiveRelaxedModerately => CameraPreset::angles(false, 19_488_000, 0, 0),
    P::PerspectiveRight => CameraPreset::angles(false, 0, 20_400_000, 0),
  }
}

#[cfg(test)]
mod tests {
  use image::{Rgba, RgbaImage};
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
  use ooxmlsdk::units::CoordinateValue;

  use super::{
    Static3dColor, apply_static_3d, bevel_terminal_inset, camera_projection, light_rig,
    output_padding,
  };
  use crate::model::RgbColor;

  fn scene(preset: a::PresetCameraValues) -> a::Scene3DType {
    a::Scene3DType {
      camera: Box::new(a::Camera {
        preset,
        ..a::Camera::default()
      }),
      light_rig: Box::new(a::LightRig::default()),
      ..a::Scene3DType::default()
    }
  }

  #[test]
  fn orthographic_front_has_no_depth_translation() {
    let projection = camera_projection(&scene(a::PresetCameraValues::OrthographicFront), 0.0);
    assert_eq!(projection.offset_x_per_depth, 0.0);
    assert_eq!(projection.offset_y_per_depth, 0.0);
    assert!(projection.parallel);
  }

  #[test]
  fn orthographic_front_bevel_does_not_expand_the_face_bounds() {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let shape = a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(190_500)),
        height: Some(CoordinateValue::Emu(38_100)),
        ..a::BevelTop::default()
      }),
      ..a::Shape3DType::default()
    };
    assert_eq!(
      output_padding(camera_projection(&scene, 0.0), &shape, 64.0, 452.0),
      super::Static3dPadding::default()
    );
  }

  #[test]
  fn unspecified_bevel_preset_uses_the_ecma_circle_default() {
    assert_eq!(bevel_terminal_inset(None), 1.0);
  }

  #[test]
  fn explicit_camera_rotation_replaces_preset_angles() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 5_400_000,
      revolution: 0,
    });
    let projection = camera_projection(&scene, 0.0);
    assert!((projection.offset_x_per_depth + 1.0).abs() < 0.001);
    assert!(projection.offset_y_per_depth.abs() < 0.001);
  }

  #[test]
  fn extrusion_padding_follows_camera_depth_vector() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 5_400_000,
      revolution: 0,
    });
    let shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(127_000)),
      ..a::Shape3DType::default()
    };
    let padding = output_padding(camera_projection(&scene, 0.0), &shape, 10.0, 10.0);
    // Positive extrusion height extends behind the front plane. With a
    // +90-degree longitude that is the right-hand side of the projected
    // volume; the original 10-point box already contains its first 5 points.
    assert_eq!(padding.left_pt, 0.0);
    assert!((padding.right_pt - 5.0).abs() < 0.01);
  }

  #[test]
  fn extrusion_paints_resolved_color_behind_front_face() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 5_400_000,
      revolution: 0,
    });
    let shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(25_400)),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::new(8, 4);
    image.put_pixel(4, 1, Rgba([0, 255, 0, 255]));
    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      Some(Static3dColor {
        color: RgbColor { r: 255, g: 0, b: 0 },
        alpha: 255,
      }),
      None,
      1.0,
      None,
    );
    // The front face is exactly edge-on at +90 degrees, while the extrusion
    // remains a visible side plane.
    assert!(!image.pixels().any(|pixel| pixel[1] > 0));
    assert!(image.pixels().any(|pixel| pixel[0] > 0 && pixel[1] == 0));
  }

  #[test]
  fn shape_z_contributes_to_camera_padding_without_extrusion() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 5_400_000,
      revolution: 0,
    });
    let shape = a::Shape3DType {
      z: Some(CoordinateValue::Emu(63_500)),
      ..a::Shape3DType::default()
    };
    let padding = output_padding(camera_projection(&scene, 0.0), &shape, 10.0, 10.0);
    assert_eq!(padding, super::Static3dPadding::default());
  }

  #[test]
  fn shape_z_translates_front_face_without_fabricating_extrusion() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 5_400_000,
      revolution: 0,
    });
    let shape = a::Shape3DType {
      z: Some(CoordinateValue::Emu(25_400)),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::new(8, 3);
    image.put_pixel(4, 1, Rgba([0, 255, 0, 255]));
    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      None,
      None,
      1.0,
      None,
    );
    assert!(!image.pixels().any(|pixel| pixel[3] != 0));
  }

  #[test]
  fn legacy_wireframe_does_not_retain_a_solid_front_face() {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let shape = a::Shape3DType {
      preset_material: Some(a::PresetMaterialTypeValues::LegacyWireframe),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::from_pixel(5, 5, Rgba([255, 0, 0, 255]));
    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      None,
      None,
      1.0,
      None,
    );
    assert_eq!(image.get_pixel(2, 2)[3], 0);
    assert_eq!(image.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
  }

  #[test]
  fn top_bevel_survives_front_face_compositing() {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let shape = a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(25_400)),
        height: Some(CoordinateValue::Emu(25_400)),
        preset: Some(a::BevelPresetValues::Circle),
      }),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::from_pixel(7, 7, Rgba([200, 40, 40, 255]));

    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      None,
      None,
      1.0,
      None,
    );

    assert_ne!(image.get_pixel(0, 0), &Rgba([200, 40, 40, 255]));
    assert_eq!(image.get_pixel(3, 3), &Rgba([200, 40, 40, 255]));
  }

  #[test]
  fn bright_room_retains_all_office_diffuse_lights_and_negative_color() {
    let rig = light_rig(a::LightRigValues::BrightRoom);
    assert_eq!(rig.ambient, [1.5; 3]);
    assert_eq!(rig.count, 4);
    assert!(rig.lights[0].diffuse);
    assert!(!rig.lights[1].diffuse);
    assert_eq!(rig.lights[2].color, [-0.5; 3]);
    assert!(rig.lights[3].diffuse);
  }

  #[test]
  fn legacy_normal_uses_key_and_softened_fill_lights() {
    let rig = light_rig(a::LightRigValues::LegacyNormal2);
    assert_eq!(rig.ambient, [0.153; 3]);
    assert_eq!(rig.count, 2);
    assert_eq!(rig.lights[0].color, [0.671; 3]);
    assert_eq!(rig.lights[1].color, [0.366; 3]);
    assert_eq!(rig.lights[1].scale, 0.5);
    assert_eq!(rig.lights[1].offset, 0.5);
  }
}
