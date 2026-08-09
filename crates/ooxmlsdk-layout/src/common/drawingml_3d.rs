use image::{Rgba, RgbaImage};
use kurbo::{PathEl, flatten};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use super::{DisplayItem, PathCommand, Rect, drawingml_geometry};
use crate::model::RgbColor;

const EMUS_PER_POINT: f32 = 12_700.0;
const TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX: f64 = 0.1;

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

/// Independently inheritable scene and shape properties attached to a text
/// run. Word 2010 stores `w14:scene3d` and `w14:props3d` as separate `rPr`
/// children, so either half can be supplied by a style while the text body's
/// DrawingML 3-D properties supply the other half.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Static3dStyleParts {
  pub(crate) scene: Option<Box<a::Scene3DType>>,
  pub(crate) shape: Option<Box<a::Shape3DType>>,
  pub(crate) extrusion_color: Option<Static3dColor>,
  pub(crate) contour_color: Option<Static3dColor>,
}

impl Static3dStyleParts {
  pub(crate) fn merge_from(&mut self, source: &Self) {
    if source.scene.is_some() {
      self.scene.clone_from(&source.scene);
    }
    if source.shape.is_some() {
      self.shape.clone_from(&source.shape);
      self.extrusion_color = source.extrusion_color;
      self.contour_color = source.contour_color;
    }
  }
}

pub(crate) fn resolve_static_3d_style(
  body: Option<&Static3dStyle>,
  run: Option<&Static3dStyleParts>,
) -> Option<Static3dStyle> {
  let run_supplies_shape = run.is_some_and(|run| run.shape.is_some());
  let shape = run
    .and_then(|run| run.shape.clone())
    .or_else(|| body.map(|body| body.shape.clone()))?;
  let scene = run
    .and_then(|run| run.scene.clone())
    .or_else(|| body.map(|body| body.scene.clone()))
    .unwrap_or_else(default_text_3d_scene);
  let (extrusion_color, contour_color) = if run_supplies_shape {
    let run = run.expect("run shape source");
    (run.extrusion_color, run.contour_color)
  } else {
    body.map_or((None, None), |body| {
      (body.extrusion_color, body.contour_color)
    })
  };
  Some(Static3dStyle {
    scene,
    shape,
    extrusion_color,
    contour_color,
  })
}

fn default_text_3d_scene() -> Box<a::Scene3DType> {
  // `w14:props3d` is independently inheritable from `w14:scene3d` and still
  // produces visible 3-D text when no scene property is present. Word's
  // neutral text scene is the same scene it serializes for an unrotated 3-D
  // text effect: an orthographic-front camera with the three-point rig aimed
  // from the top. Requiring an authored scene silently flattened props-only
  // runs, including bevel, contour, extrusion, and material.
  Box::new(a::Scene3DType {
    camera: Box::new(a::Camera {
      preset: a::PresetCameraValues::OrthographicFront,
      ..a::Camera::default()
    }),
    light_rig: Box::new(a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    }),
    ..a::Scene3DType::default()
  })
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Static3dPadding {
  pub(crate) left_pt: f32,
  pub(crate) top_pt: f32,
  pub(crate) right_pt: f32,
  pub(crate) bottom_pt: f32,
}

/// Projected page-plane range of a 3-D surface, expressed relative to the
/// unprojected model surface's top-left corner.
///
/// Unlike [`Static3dPadding`], this preserves translations and contractions.
/// A perspective camera can move every projected point to one side of the
/// original rectangle, which cannot be represented by four non-negative
/// padding values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Static3dOutputBounds {
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

/// Device-space glyph geometry consumed by the dedicated static-3-D text
/// renderer.
///
/// Microsoft DirectWrite's `GetGlyphRunOutline` supplies one winding path for
/// the shaped run. Its Direct2D samples flatten that path before tessellating
/// the front/back faces and walking the same contours for extrusion. Keeping
/// these contours beside the painted text bitmap prevents the 3-D stage from
/// reconstructing letter edges and counters from antialiased alpha pixels.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Static3dTextGeometry {
  contours: Vec<Static3dTextContour>,
  solid_on_right: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Static3dTextContour {
  points: Vec<(f32, f32)>,
}

impl Static3dTextGeometry {
  pub(crate) fn from_page_path(
    commands: &[PathCommand],
    raster_bounds: Rect,
    pixels_per_point: f32,
  ) -> Option<Self> {
    if commands.is_empty() || !pixels_per_point.is_finite() || pixels_per_point <= f32::EPSILON {
      return None;
    }
    let elements = drawingml_geometry::mapped_path_elements(commands, |point| {
      kurbo::Point::new(
        f64::from((point.x.0 - raster_bounds.origin.x.0) * pixels_per_point),
        f64::from((point.y.0 - raster_bounds.origin.y.0) * pixels_per_point),
      )
    });
    let mut contours = Vec::new();
    let mut points = Vec::new();
    flatten(
      elements,
      TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX,
      |element| match element {
        PathEl::MoveTo(point) => {
          finish_text_3d_contour(&mut contours, &mut points);
          points.push((point.x as f32, point.y as f32));
        }
        PathEl::LineTo(point) => {
          let point = (point.x as f32, point.y as f32);
          if points
            .last()
            .is_none_or(|previous| (previous.0 - point.0).hypot(previous.1 - point.1) > 1.0e-4)
          {
            points.push(point);
          }
        }
        PathEl::ClosePath => finish_text_3d_contour(&mut contours, &mut points),
        PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
          unreachable!("kurbo::flatten emits only line path elements")
        }
      },
    );
    finish_text_3d_contour(&mut contours, &mut points);
    let outer_area = contours
      .iter()
      .map(|contour| signed_contour_area(&contour.points))
      .max_by(|left, right| left.abs().total_cmp(&right.abs()))?;
    if outer_area.abs() <= f32::EPSILON {
      return None;
    }
    let solid_on_right = outer_area > 0.0;
    (!contours.is_empty()).then_some(Self {
      contours,
      solid_on_right,
    })
  }

  fn inset(&self, distance_px: f32) -> Option<Self> {
    if distance_px <= f32::EPSILON {
      return Some(self.clone());
    }
    let mut contours = self
      .contours
      .iter()
      .map(|contour| Static3dTextContour {
        points: offset_text_3d_contour(&contour.points, distance_px, self.solid_on_right),
      })
      .collect::<Vec<_>>();
    contours.retain(|contour| {
      contour.points.len() >= 3 && signed_contour_area(&contour.points).abs() > 1.0e-3
    });
    (!contours.is_empty()).then_some(Self {
      contours,
      solid_on_right: self.solid_on_right,
    })
  }
}

fn finish_text_3d_contour(contours: &mut Vec<Static3dTextContour>, points: &mut Vec<(f32, f32)>) {
  if points.len() >= 2
    && (points[0].0 - points[points.len() - 1].0).hypot(points[0].1 - points[points.len() - 1].1)
      <= 1.0e-4
  {
    points.pop();
  }
  if points.len() >= 3 && signed_contour_area(points).abs() > 1.0e-3 {
    contours.push(Static3dTextContour {
      points: std::mem::take(points),
    });
  } else {
    points.clear();
  }
}

fn signed_contour_area(points: &[(f32, f32)]) -> f32 {
  points
    .iter()
    .zip(points.iter().cycle().skip(1))
    .map(|(&(x0, y0), &(x1, y1))| x0 * y1 - x1 * y0)
    .sum::<f32>()
    * 0.5
}

fn offset_text_3d_contour(
  points: &[(f32, f32)],
  inward_distance: f32,
  solid_on_right: bool,
) -> Vec<(f32, f32)> {
  if points.len() < 3 || inward_distance.abs() <= f32::EPSILON {
    return points.to_vec();
  }
  let unit_edge = |from: (f32, f32), to: (f32, f32)| {
    let edge = (to.0 - from.0, to.1 - from.1);
    let length = edge.0.hypot(edge.1);
    if length <= f32::EPSILON {
      (0.0, 0.0)
    } else {
      (edge.0 / length, edge.1 / length)
    }
  };
  let inward_normal = |edge: (f32, f32)| {
    if solid_on_right {
      (-edge.1, edge.0)
    } else {
      (edge.1, -edge.0)
    }
  };
  let mut output = Vec::with_capacity(points.len());
  for index in 0..points.len() {
    let previous = points[(index + points.len() - 1) % points.len()];
    let current = points[index];
    let next = points[(index + 1) % points.len()];
    let previous_edge = unit_edge(previous, current);
    let next_edge = unit_edge(current, next);
    if previous_edge == (0.0, 0.0) || next_edge == (0.0, 0.0) {
      output.push(current);
      continue;
    }
    let previous_normal = inward_normal(previous_edge);
    let next_normal = inward_normal(next_edge);
    let bisector = (
      previous_normal.0 + next_normal.0,
      previous_normal.1 + next_normal.1,
    );
    let bisector_length = bisector.0.hypot(bisector.1);
    let offset = if bisector_length <= 1.0e-4 {
      (
        current.0 + next_normal.0 * inward_distance,
        current.1 + next_normal.1 * inward_distance,
      )
    } else {
      let bisector = (bisector.0 / bisector_length, bisector.1 / bisector_length);
      // A raw offset-line intersection is unbounded at sharp concave glyph
      // corners and requires a boolean-outline pass to remove the resulting
      // self-intersection. Direct2D performs that pass before tessellation.
      // This bounded bisector is its local equivalent: ordinary right-angle
      // miters retain their sqrt(2) length, while cusps transition to a bevel
      // join instead of producing spikes across neighboring letters.
      let projection = (bisector.0 * next_normal.0 + bisector.1 * next_normal.1).abs();
      let miter_scale = projection.max(0.5).recip();
      (
        current.0 + bisector.0 * inward_distance * miter_scale,
        current.1 + bisector.1 * inward_distance * miter_scale,
      )
    };
    output.push(offset);
  }
  output
}

fn text_3d_contour_edge_normals(
  points: &[(f32, f32)],
  solid_on_right: bool,
) -> Vec<([f32; 2], [f32; 2])> {
  let edge_normals = points
    .iter()
    .zip(points.iter().cycle().skip(1))
    .map(|(&(from_x, from_y), &(to_x, to_y))| {
      let edge_x = to_x - from_x;
      let edge_y = to_y - from_y;
      let length = edge_x.hypot(edge_y).max(f32::EPSILON);
      if solid_on_right {
        [edge_y / length, -edge_x / length]
      } else {
        [-edge_y / length, edge_x / length]
      }
    })
    .collect::<Vec<_>>();
  let mut starts = edge_normals.clone();
  let mut ends = edge_normals.clone();
  for index in 0..edge_normals.len() {
    let previous = (index + edge_normals.len() - 1) % edge_normals.len();
    let incoming = edge_normals[previous];
    let outgoing = edge_normals[index];
    if incoming[0] * outgoing[0] + incoming[1] * outgoing[1] > 0.5 {
      let mut average = [incoming[0] + outgoing[0], incoming[1] + outgoing[1]];
      let length = average[0].hypot(average[1]).max(f32::EPSILON);
      average[0] /= length;
      average[1] /= length;
      ends[previous] = average;
      starts[index] = average;
    }
  }
  starts.into_iter().zip(ends).collect()
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Static3dRenderOptions {
  pub(crate) extrusion_color: Option<Static3dColor>,
  pub(crate) contour_color: Option<Static3dColor>,
  pub(crate) pixels_per_point: f32,
  pub(crate) model_surface: Option<Static3dSurface>,
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
  let bounds = projected_output_bounds(projection, shape, width_pt, height_pt);
  Static3dPadding {
    left_pt: (-bounds.left_pt).max(0.0),
    top_pt: (-bounds.top_pt).max(0.0),
    right_pt: (bounds.right_pt - width_pt).max(0.0),
    bottom_pt: (bounds.bottom_pt - height_pt).max(0.0),
  }
}

/// Returns the projected range of the complete model surface.
pub(crate) fn projected_output_bounds(
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  width_pt: f32,
  height_pt: f32,
) -> Static3dOutputBounds {
  projected_region_output_bounds(
    projection,
    shape,
    width_pt,
    height_pt,
    Static3dOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: width_pt,
      bottom_pt: height_pt,
    },
  )
}

/// Returns the projected range of one painted region inside a larger model
/// surface. Text is the important case: glyph ink occupies only a small part
/// of its owning text frame, while perspective and lighting are evaluated in
/// the coordinate system of the complete text frame.
pub(crate) fn projected_region_output_bounds(
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  model_width_pt: f32,
  model_height_pt: f32,
  region: Static3dOutputBounds,
) -> Static3dOutputBounds {
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
  let top_bevel_height_pt = shape.bevel_top.as_ref().map_or(0.0, |bevel| {
    bevel
      .height
      .or(bevel.width)
      .map_or(0.0, |value| value.to_emu() as f32 / EMUS_PER_POINT)
  });
  let bottom_bevel_height_pt = shape.bevel_bottom.as_ref().map_or(0.0, |bevel| {
    bevel
      .height
      .or(bevel.width)
      .map_or(0.0, |value| value.to_emu() as f32 / EMUS_PER_POINT)
  });
  // DrawingML's front plane is located at `z`; extrusion extends behind it
  // to `z - extrusionH`. LibreOffice expresses the same interval as
  // forwardDepth=z and backwardDepth=extrusionH-z before translating the
  // extruded solid. Project all eight volume corners so x/y camera rotation
  // and perspective contribute to bounds as well as z travel.
  let mut min_x = f32::INFINITY;
  let mut min_y = f32::INFINITY;
  let mut max_x = f32::NEG_INFINITY;
  let mut max_y = f32::NEG_INFINITY;
  for depth in [
    z_pt + top_bevel_height_pt,
    z_pt,
    z_pt - depth_pt,
    z_pt - depth_pt - bottom_bevel_height_pt,
  ] {
    for x in [region.left_pt, region.right_pt] {
      for y in [region.top_pt, region.bottom_pt] {
        let (projected_x, projected_y) = project_local(
          projection,
          x - model_width_pt * 0.5,
          y - model_height_pt * 0.5,
          depth,
          model_width_pt,
          model_height_pt,
        );
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
  // camera. For a rotated or perspective camera its authored height does
  // move the edge in screen space, so the depth bounds above include both
  // bevel terminal planes. Only a contour grows every edge in screen space.
  // `contourW` is the width of the complete contour line, not the radius by
  // which each side of the silhouette grows. The line is centered on the
  // boundary, so only half of its width contributes to an outer bound.
  let edge = contour_pt * 0.5;
  Static3dOutputBounds {
    left_pt: min_x + model_width_pt * 0.5 - edge,
    top_pt: min_y + model_height_pt * 0.5 - edge,
    right_pt: max_x + model_width_pt * 0.5 + edge,
    bottom_pt: max_y + model_height_pt * 0.5 + edge,
  }
}

/// Projects one logical region on the authored front plane without adding
/// extrusion, bevel, or contour extents.
///
/// Word's W14 effect graph keeps separate painted and logical rectangles. A
/// static-3-D front image therefore also needs the projected character-cell
/// rectangle: shadow/reflection alignment cannot keep using its unprojected
/// left/top anchor after the pixels have passed through the camera.
pub(crate) fn projected_front_region_output_bounds(
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  model_width_pt: f32,
  model_height_pt: f32,
  region: Static3dOutputBounds,
) -> Static3dOutputBounds {
  let z_pt = shape
    .z
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  let mut min_x = f32::INFINITY;
  let mut min_y = f32::INFINITY;
  let mut max_x = f32::NEG_INFINITY;
  let mut max_y = f32::NEG_INFINITY;
  for x in [region.left_pt, region.right_pt] {
    for y in [region.top_pt, region.bottom_pt] {
      let (projected_x, projected_y) = project_local(
        projection,
        x - model_width_pt * 0.5,
        y - model_height_pt * 0.5,
        z_pt,
        model_width_pt,
        model_height_pt,
      );
      min_x = min_x.min(projected_x);
      min_y = min_y.min(projected_y);
      max_x = max_x.max(projected_x);
      max_y = max_y.max(projected_y);
    }
  }
  Static3dOutputBounds {
    left_pt: min_x + model_width_pt * 0.5,
    top_pt: min_y + model_height_pt * 0.5,
    right_pt: max_x + model_width_pt * 0.5,
    bottom_pt: max_y + model_height_pt * 0.5,
  }
}

/// Projects the authored planar face into the 3-D scene without adding the
/// extrusion, contour, bevel, material lighting, or back face.
///
/// Word text effects consume this front-plane image: their glyph counters
/// follow the camera, but a shadow, glow, or reflection must not inherit the
/// solid's side faces. Microsoft's Direct2D 3-D transform has the same bitmap
/// input boundary; solid construction remains a separate stage below.
pub(crate) fn project_static_3d_front_face(
  source: &RgbaImage,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  pixels_per_point: f32,
  model_surface: Option<Static3dSurface>,
) -> RgbaImage {
  let Some(bounds) = alpha_bounds(source) else {
    return RgbaImage::new(source.width(), source.height());
  };
  let model_surface = model_surface.unwrap_or(Static3dSurface {
    left_px: bounds.0 as f32,
    top_px: bounds.1 as f32,
    width_px: (bounds.2 - bounds.0 + 1).max(1) as f32,
    height_px: (bounds.3 - bounds.1 + 1).max(1) as f32,
  });
  let front_z_px = shape
    .z
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
    .unwrap_or(0.0);
  let mut output = RgbaImage::new(source.width(), source.height());
  composite_projected_image(
    &mut output,
    source,
    ProjectedImageOptions {
      projection,
      z: front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      tint: None,
    },
  );
  output
}

/// Restricts the painted text surface to the glyph fill that owns the 3-D
/// solid, while retaining line paint already composited into that surface.
///
/// A W14 `textOutline` is a paint property of the text face. Treating it as a
/// second flat image above the completed solid hides the bevel lighting;
/// absorbing its centered stroke into the tessellated glyph instead expands
/// counters and distorts tight joins. The fill alpha therefore remains the
/// geometry/opacity mask while the combined fill-and-line RGB supplies the
/// material color sampled by the bevel and planar face.
pub(crate) fn mask_static_3d_text_surface_paint(
  surface_paint: &mut RgbaImage,
  fill_geometry: &RgbaImage,
) {
  debug_assert_eq!(surface_paint.dimensions(), fill_geometry.dimensions());
  for (surface, fill) in surface_paint.pixels_mut().zip(fill_geometry.pixels()) {
    surface[3] = fill[3];
    if fill[3] == 0 {
      surface[0] = 0;
      surface[1] = 0;
      surface[2] = 0;
    }
  }
}

fn static_3d_front_cap_z_emu(shape: &a::Shape3DType) -> i64 {
  let z_emu = shape.z.map_or(0, |value| value.to_emu());
  let bevel_width_emu = shape
    .bevel_top
    .as_ref()
    .and_then(|bevel| bevel.width)
    .map_or(0, |value| value.to_emu());
  let bevel_height_emu = if bevel_width_emu > 0 {
    shape
      .bevel_top
      .as_ref()
      .and_then(|bevel| bevel.height)
      .map_or(bevel_width_emu, |value| value.to_emu())
  } else {
    0
  };
  z_emu.saturating_add(bevel_height_emu)
}

fn static_3d_front_cap_z_px(shape: &a::Shape3DType, pixels_per_point: f32) -> f32 {
  static_3d_front_cap_z_emu(shape) as f32 / EMUS_PER_POINT * pixels_per_point
}

fn static_3d_top_bevel_terminal_inset_px(shape: &a::Shape3DType, pixels_per_point: f32) -> f32 {
  shape.bevel_top.as_ref().map_or(0.0, |bevel| {
    bevel.width.map_or(0.0, |value| {
      value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point * bevel_terminal_inset(bevel.preset)
    })
  })
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
  options: Static3dRenderOptions,
) {
  apply_static_3d_impl(image, scene, projection, shape, options, None);
}

pub(crate) fn apply_static_3d_text(
  image: &mut RgbaImage,
  geometry: &Static3dTextGeometry,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
) {
  apply_static_3d_impl(image, scene, projection, shape, options, Some(geometry));
}

fn apply_static_3d_impl(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
  text_geometry: Option<&Static3dTextGeometry>,
) {
  let Static3dRenderOptions {
    extrusion_color,
    contour_color,
    pixels_per_point,
    model_surface,
  } = options;
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
  let contour_radius_px = shape
    .contour_width
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point * 0.5)
    .unwrap_or(0.0)
    .round()
    .clamp(0.0, 32.0) as i32;
  let top_bevel_authored_width_px = shape
    .bevel_top
    .as_ref()
    .map_or(0.0, |bevel| {
      bevel.width.map_or(0.0, |value| {
        value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point
      })
    })
    .clamp(0.0, 24.0);
  let top_bevel_terminal_inset_px = static_3d_top_bevel_terminal_inset_px(shape, pixels_per_point);
  let top_bevel_height_px = if top_bevel_authored_width_px > f32::EPSILON {
    shape
      .bevel_top
      .as_ref()
      .and_then(|bevel| bevel.height)
      .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
      .unwrap_or(top_bevel_authored_width_px)
  } else {
    0.0
  };
  // MS-OI29500 §20.1.10.9 places the bevel's outer edge at (z, inset) =
  // (0, 0), then moves away from the authored face and inward along its
  // published profile. The remaining flat cap continues from that profile's
  // terminal height, so it lies at `z + bevelH` and is inset by the terminal
  // profile width. This is one continuous solid, not a second complete glyph.
  let planar_front_z_px = static_3d_front_cap_z_px(shape, pixels_per_point);
  let top_bevel_px = top_bevel_terminal_inset_px.round().clamp(0.0, 24.0) as i32;
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
  let front = image.clone();
  image.fill(0);
  let mut text_surface_triangles = Vec::new();
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
    let back_normal = lighting_surface_normal(scene, projection, [0.0, 0.0, -1.0]);
    let back_shade = if text_geometry.is_some() {
      material_diffuse_shade(scene, back_normal, shape.preset_material)
    } else {
      legacy_material_diffuse_shade(scene, back_normal, shape.preset_material)
    };
    if !wireframe {
      let mut back_face = RgbaImage::new(image.width(), image.height());
      let options = ProjectedImageOptions {
        projection,
        z: back_z_px,
        bounds,
        model_surface,
        pixels_per_point,
        tint: Some((extrusion, back_shade)),
      };
      if let Some(geometry) = text_geometry {
        composite_projected_text_geometry(&mut back_face, &front, geometry, options);
      } else {
        composite_projected_image(&mut back_face, &front, options);
      }
      if bottom_bevel_px > 0 {
        let bevel_height_px = shape
          .bevel_bottom
          .as_ref()
          .and_then(|bevel| bevel.height)
          .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
          .unwrap_or(bottom_bevel_px as f32);
        let mask = back_face.clone();
        let _ = composite_bevel(
          &mut back_face,
          &mask,
          BevelOptions {
            width: bottom_bevel_px,
            height: bevel_height_px,
            preset: shape.bevel_bottom.as_ref().and_then(|bevel| bevel.preset),
            scene,
            projection,
            model_surface,
            pixels_per_point,
            surface_z: back_z_px,
            material: shape.preset_material,
            back_face: true,
          },
        );
      }
      composite_image(image, &back_face);
    }
    let options = ExtrusionEdgeOptions {
      bounds,
      model_surface,
      projection,
      front_z: front_z_px,
      back_z: back_z_px,
      pixels_per_point,
      steps,
      tint: extrusion,
      scene,
      material: shape.preset_material,
      wireframe,
    };
    if let Some(geometry) = text_geometry.filter(|_| !wireframe) {
      text_surface_triangles.extend(text_extrusion_edge_triangles(geometry, options));
    } else {
      composite_extrusion_edges(image, &front, options);
    }
  }
  // A text contour is derived from the final solid silhouette below. Keep it
  // behind the front material surfaces, as Office's contour is an unlit line
  // around the solid rather than a fourth material face participating in the
  // depth buffer.
  if contour_radius_px > 0 && (text_geometry.is_none() || wireframe) {
    let contour = contour_color.unwrap_or(Static3dColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 255,
    });
    // Office contours the complete projected solid, including extrusion
    // edges, rather than only the untransformed front-face mask.
    let mut silhouette = image.clone();
    let options = ProjectedImageOptions {
      projection,
      z: front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      tint: None,
    };
    if let Some(geometry) = text_geometry {
      composite_projected_text_geometry(&mut silhouette, &front, geometry, options);
    } else {
      composite_projected_image(&mut silhouette, &front, options);
    }
    composite_outline(image, &silhouette, contour_radius_px, contour);
  }
  let mut front_face = front.clone();
  let mut top_bevel = None;
  let mut text_planar_geometry = None;
  let mut text_bevel = None;
  // MS-OI29500 defines every Office bevel, including `circle`, as a 2-D
  // profile swept inward from the vector face boundary. Keep text on that
  // parametric surface path; the raster fallback below is only for callers
  // that do not provide glyph geometry.
  let dedicated_text_bevel = text_geometry.filter(|_| top_bevel_authored_width_px > f32::EPSILON);
  if let Some(geometry) = dedicated_text_bevel {
    text_planar_geometry = geometry.inset(top_bevel_terminal_inset_px);
    text_bevel = Some(TextBevelOptions {
      width: top_bevel_authored_width_px,
      height: top_bevel_height_px,
      preset: shape.bevel_top.as_ref().and_then(|bevel| bevel.preset),
      scene,
      projection,
      model_surface,
      pixels_per_point,
      surface_z: front_z_px,
      material: shape.preset_material,
    });
  } else if top_bevel_px > 0 {
    let options = BevelOptions {
      width: top_bevel_px,
      height: top_bevel_height_px,
      preset: shape.bevel_top.as_ref().and_then(|bevel| bevel.preset),
      scene,
      projection,
      model_surface,
      pixels_per_point,
      surface_z: front_z_px,
      material: shape.preset_material,
      back_face: false,
    };
    let mut bevel_layer = RgbaImage::new(image.width(), image.height());
    let height_offsets = composite_bevel(&mut bevel_layer, &front, options);
    // The bevel profile connects the authored outer edge to the raised,
    // inset cap. Remove that boundary band from the cap so the two surfaces
    // do not occupy the same source-space geometry.
    for (flat, bevel) in front_face.pixels_mut().zip(bevel_layer.pixels()) {
      if bevel[3] != 0 {
        *flat = Rgba([0, 0, 0, 0]);
      }
    }
    let variable_z = (options.preset.unwrap_or(a::BevelPresetValues::Circle)
      == a::BevelPresetValues::Circle)
      .then_some(VariableZSurface {
        pixel_offsets: height_offsets,
        vertex_offsets: None,
      });
    top_bevel = Some((bevel_layer, variable_z));
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
      ProjectedImageOptions {
        projection,
        z: front_z_px,
        bounds,
        model_surface,
        pixels_per_point,
        tint: None,
      },
    );
  } else {
    // Preserve the authored profile as finite surface strips. Besides being
    // the path needed by rotated scenes, this retains the separate lit faces
    // of folded presets such as `relaxedInset`; collapsing an orthographic
    // text bevel to one nearest-boundary sample flattened those material
    // bands even though its glyph coverage was equivalent.
    if let (Some(geometry), Some(options)) = (text_geometry, text_bevel) {
      text_surface_triangles.extend(text_bevel_triangles(&front, geometry, options));
    } else if let Some((bevel_layer, height_surface)) = top_bevel {
      if let Some(height_surface) = height_surface {
        composite_projected_variable_z_image(
          image,
          &bevel_layer,
          &height_surface,
          VariableZProjectedImageOptions {
            projection,
            base_z: front_z_px,
            model_surface,
            pixels_per_point,
          },
        );
      } else {
        composite_projected_image(
          image,
          &bevel_layer,
          ProjectedImageOptions {
            projection,
            z: front_z_px,
            bounds,
            model_surface,
            pixels_per_point,
            tint: None,
          },
        );
      }
    }
    let options = ProjectedImageOptions {
      projection,
      z: planar_front_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      tint: None,
    };
    shade_planar_surface(
      &mut front_face,
      scene,
      &options,
      [0.0, 0.0, 1.0],
      shape.preset_material,
      text_geometry.is_some(),
    );
    if let Some(geometry) = text_geometry {
      let planar_geometry = text_planar_geometry.as_ref().unwrap_or(geometry);
      let mut solid = RgbaImage::new(image.width(), image.height());
      composite_text_solid_surfaces(
        &mut solid,
        &front_face,
        geometry,
        planar_geometry,
        &text_surface_triangles,
        options,
      );
      if contour_radius_px > 0 {
        let contour = contour_color.unwrap_or(Static3dColor {
          color: RgbColor { r: 0, g: 0, b: 0 },
          alpha: 255,
        });
        let mut silhouette = image.clone();
        composite_image(&mut silhouette, &solid);
        composite_outline(image, &silhouette, contour_radius_px, contour);
      }
      composite_image(image, &solid);
    } else {
      composite_projected_image(image, &front_face, options);
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum BevelProfileSegment {
  Line {
    to: [f32; 2],
  },
  Quadratic {
    control: [f32; 2],
    to: [f32; 2],
  },
  Cubic {
    control_1: [f32; 2],
    control_2: [f32; 2],
    to: [f32; 2],
  },
}

#[derive(Clone, Copy, Debug)]
struct BevelProfileSample {
  /// Distance away from the original face, normalized to the full authored
  /// bevel height. MS-OI29500 requires this normalization because presets
  /// such as `cross` terminate at x=0.6 but still consume the full height.
  height: f32,
  /// Distance into the face, in authored bevel-width units.
  inset: f32,
  height_tangent: f32,
  inset_tangent: f32,
}

const ANGLE_BEVEL: &[BevelProfileSegment] = &[BevelProfileSegment::Line { to: [1.0, 1.0] }];
const ART_DECO_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.184_095],
    control_2: [0.149_238, 0.333_333],
    to: [0.333_333, 0.333_333],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.333_333, 0.701_523],
    control_2: [0.631_810, 1.0],
    to: [1.0, 1.0],
  },
];
const CIRCLE_BEVEL: &[BevelProfileSegment] = &[BevelProfileSegment::Cubic {
  control_1: [0.0, 0.556_27],
  control_2: [0.443_73, 1.0],
  to: [1.0, 1.0],
}];
const CONVEX_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.070_820],
    control_2: [0.029_745_8, 0.1],
    to: [0.101_416, 0.1],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.501_416, 0.1],
    control_2: [0.9, 0.7],
    to: [0.901_416, 0.899_999],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.9, 0.971_670],
    control_2: [0.933_430, 1.0],
    to: [1.0, 1.0],
  },
];
const COOL_SLANT_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.138_122],
    control_2: [0.0, 0.2],
    to: [0.271_356, 0.775_535],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.377_654, 1.0],
    control_2: [0.519_455, 1.0],
    to: [0.583_589, 1.0],
  },
];
const CROSS_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.055_63],
    control_2: [0.044_37, 0.1],
    to: [0.1, 0.1],
  },
  BevelProfileSegment::Line { to: [0.4, 0.1] },
  BevelProfileSegment::Cubic {
    control_1: [0.455_63, 0.1],
    control_2: [0.5, 0.144_37],
    to: [0.5, 0.2],
  },
  BevelProfileSegment::Line { to: [0.5, 0.9] },
  BevelProfileSegment::Cubic {
    control_1: [0.5, 0.955_63],
    control_2: [0.544_37, 1.0],
    to: [0.6, 1.0],
  },
];
const DIVOT_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.236_604],
    control_2: [0.119_276, 0.607_024],
    to: [0.263_046, 0.760_235],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.361_098, 0.864_726],
    control_2: [0.457_934, 0.909_567],
    to: [0.537_806, 0.925_082],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.559_245, 0.929_246],
    control_2: [0.567_625, 0.897_930],
    to: [0.542_066, 0.845_567],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.484_073, 0.726_757],
    control_2: [0.477_103, 0.393_693],
    to: [0.551_651, 0.393_693],
  },
  BevelProfileSegment::Line {
    to: [0.899_894, 0.393_693],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.991_480, 0.393_693],
    control_2: [0.958_466, 0.746_659],
    to: [0.907_348, 0.779_629],
  },
  BevelProfileSegment::Line {
    to: [0.879_394, 0.797_658],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.845_545, 0.819_489],
    control_2: [0.848_775, 0.954_172],
    to: [0.874_334, 0.971_627],
  },
  BevelProfileSegment::Quadratic {
    control: [0.915_883, 1.0],
    to: [1.0, 1.0],
  },
];
const HARD_EDGE_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Quadratic {
    control: [0.0, 0.092_437],
    to: [0.042_353, 0.305_322],
  },
  BevelProfileSegment::Line {
    to: [0.170_124, 0.947_558],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.177_203, 0.983_142],
    control_2: [0.2, 1.0],
    to: [0.268_235, 0.998_599],
  },
  BevelProfileSegment::Line {
    to: [0.614_118, 0.998_599],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.647_059, 0.998_599],
    control_2: [0.656_471, 0.987_395],
    to: [0.663_529, 0.969_188],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.687_006, 0.908_633],
    control_2: [0.802_353, 0.822_129],
    to: [1.0, 0.822_129],
  },
];
const RELAXED_INSET_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.367],
    control_2: [0.124_605, 0.820],
    to: [0.507_899, 1.0],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.724_115, 0.737],
    control_2: [0.790_455, 0.640],
    to: [1.0, 0.640],
  },
];
const RIBLET_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.238_519],
    control_2: [0.132_047, 0.500_741],
    to: [0.357_567, 0.731_852],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.513_167, 0.891_311],
    control_2: [0.563_798, 0.912_593],
    to: [0.735_905, 0.912_593],
  },
  BevelProfileSegment::Line {
    to: [0.873_887, 0.912_593],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.956_973, 0.912_593],
    control_2: [0.878_338, 1.0],
    to: [1.0, 1.0],
  },
];
const SLOPE_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Quadratic {
    control: [0.0, 0.125],
    to: [0.025, 0.25],
  },
  BevelProfileSegment::Line { to: [0.125, 0.75] },
  BevelProfileSegment::Cubic {
    control_1: [0.175, 1.0],
    control_2: [0.25, 1.0],
    to: [0.375, 1.0],
  },
  BevelProfileSegment::Line { to: [0.5, 1.0] },
  BevelProfileSegment::Cubic {
    control_1: [0.625, 1.0],
    control_2: [0.7, 1.0],
    to: [0.75, 0.75],
  },
  BevelProfileSegment::Line { to: [0.875, 0.125] },
  BevelProfileSegment::Cubic {
    control_1: [0.9, 0.01],
    control_2: [0.98, 0.01],
    to: [1.0, 0.01],
  },
];
const SOFT_ROUND_BEVEL: &[BevelProfileSegment] = &[
  BevelProfileSegment::Cubic {
    control_1: [0.0, 0.477_50],
    control_2: [0.096_873_6, 1.0],
    to: [0.156_301, 1.0],
  },
  BevelProfileSegment::Cubic {
    control_1: [0.264_179, 1.0],
    control_2: [0.376_919, 0.333_33],
    to: [1.0, 0.333_33],
  },
];

fn bevel_profile(preset: Option<a::BevelPresetValues>) -> &'static [BevelProfileSegment] {
  use a::BevelPresetValues as B;
  match preset.unwrap_or(B::Circle) {
    B::Angle => ANGLE_BEVEL,
    B::ArtDeco => ART_DECO_BEVEL,
    B::Circle => CIRCLE_BEVEL,
    B::Convex => CONVEX_BEVEL,
    B::CoolSlant => COOL_SLANT_BEVEL,
    B::Cross => CROSS_BEVEL,
    B::Divot => DIVOT_BEVEL,
    B::HardEdge => HARD_EDGE_BEVEL,
    B::RelaxedInset => RELAXED_INSET_BEVEL,
    B::Riblet => RIBLET_BEVEL,
    B::Slope => SLOPE_BEVEL,
    B::SoftRound => SOFT_ROUND_BEVEL,
  }
}

fn bevel_segment_endpoint(segment: BevelProfileSegment) -> [f32; 2] {
  match segment {
    BevelProfileSegment::Line { to }
    | BevelProfileSegment::Quadratic { to, .. }
    | BevelProfileSegment::Cubic { to, .. } => to,
  }
}

fn sample_bevel_profile_segment(
  segment: BevelProfileSegment,
  from: [f32; 2],
  terminal_height: f32,
  t: f32,
) -> BevelProfileSample {
  let t = t.clamp(0.0, 1.0);
  let one_minus_t = 1.0 - t;
  let (point, tangent) = match segment {
    BevelProfileSegment::Line { to } => (
      [
        from[0] + (to[0] - from[0]) * t,
        from[1] + (to[1] - from[1]) * t,
      ],
      [to[0] - from[0], to[1] - from[1]],
    ),
    BevelProfileSegment::Quadratic { control, to } => (
      [
        one_minus_t.powi(2) * from[0] + 2.0 * one_minus_t * t * control[0] + t.powi(2) * to[0],
        one_minus_t.powi(2) * from[1] + 2.0 * one_minus_t * t * control[1] + t.powi(2) * to[1],
      ],
      [
        2.0 * one_minus_t * (control[0] - from[0]) + 2.0 * t * (to[0] - control[0]),
        2.0 * one_minus_t * (control[1] - from[1]) + 2.0 * t * (to[1] - control[1]),
      ],
    ),
    BevelProfileSegment::Cubic {
      control_1,
      control_2,
      to,
    } => (
      [
        one_minus_t.powi(3) * from[0]
          + 3.0 * one_minus_t.powi(2) * t * control_1[0]
          + 3.0 * one_minus_t * t.powi(2) * control_2[0]
          + t.powi(3) * to[0],
        one_minus_t.powi(3) * from[1]
          + 3.0 * one_minus_t.powi(2) * t * control_1[1]
          + 3.0 * one_minus_t * t.powi(2) * control_2[1]
          + t.powi(3) * to[1],
      ],
      [
        3.0 * one_minus_t.powi(2) * (control_1[0] - from[0])
          + 6.0 * one_minus_t * t * (control_2[0] - control_1[0])
          + 3.0 * t.powi(2) * (to[0] - control_2[0]),
        3.0 * one_minus_t.powi(2) * (control_1[1] - from[1])
          + 6.0 * one_minus_t * t * (control_2[1] - control_1[1])
          + 3.0 * t.powi(2) * (to[1] - control_2[1]),
      ],
    ),
  };
  let height_scale = terminal_height.max(f32::EPSILON).recip();
  BevelProfileSample {
    height: point[0] * height_scale,
    inset: point[1],
    height_tangent: tangent[0] * height_scale,
    inset_tangent: tangent[1],
  }
}

fn bevel_profile_sample(
  preset: Option<a::BevelPresetValues>,
  segment_index: usize,
  t: f32,
) -> BevelProfileSample {
  let profile = bevel_profile(preset);
  let terminal_height = bevel_segment_endpoint(*profile.last().expect("bevel profile"))[0];
  let from = if segment_index == 0 {
    [0.0, 0.0]
  } else {
    bevel_segment_endpoint(profile[segment_index - 1])
  };
  sample_bevel_profile_segment(profile[segment_index], from, terminal_height, t)
}

fn bevel_terminal_inset(preset: Option<a::BevelPresetValues>) -> f32 {
  let profile = bevel_profile(preset);
  bevel_segment_endpoint(*profile.last().expect("bevel profile"))[1]
}

fn circle_bevel_profile(inward_fraction: f32) -> (f32, f32, f32) {
  // Circle is monotone in bevel-space y. Invert that coordinate for the
  // bounded distance-field renderer; vector text uses the parametric profile
  // directly so folded presets retain every authored surface branch.
  let authored_y = inward_fraction.clamp(0.0, 1.0);
  let mut low = 0.0;
  let mut high = 1.0;
  for _ in 0..16 {
    let middle = (low + high) * 0.5;
    if bevel_profile_sample(Some(a::BevelPresetValues::Circle), 0, middle).inset < authored_y {
      low = middle;
    } else {
      high = middle;
    }
  }
  let sample = bevel_profile_sample(Some(a::BevelPresetValues::Circle), 0, (low + high) * 0.5);
  (sample.height, sample.height_tangent, sample.inset_tangent)
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
    // direction or explicit rotation. These preset vectors are already in the
    // toward-light convention consumed by Office's material dot product.
    let direction = resolved_light_direction(scene, *light, rotation_degrees);
    let level = light.scale * dot3(direction, normal).max(0.0) + light.offset;
    for (channel, color) in shade.iter_mut().zip(light.color) {
      *channel += color * level;
    }
  }
  // D3D9's fixed-function lighting equation adds ambient and positive
  // diffuse terms; it does not inject a minimum illumination. Preserve zero
  // for a surface facing away from every light so Office materials can form
  // their authored deep edge shadows. Final color conversion still clamps
  // overbright channels to the device range.
  shade.map(|channel| channel.clamp(0.0, 2.5))
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
    let mut toward_light = direction;
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

/// PowerPoint's established fixed-output path interprets the published rig
/// vectors as the direction in which light travels. W14 text uses the newer
/// toward-light convention above; keep the two host contracts explicit.
fn legacy_light_rig_surface_shade(scene: &a::Scene3DType, normal: [f32; 3]) -> [f32; 3] {
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
    let direction = resolved_light_direction(scene, *light, rotation_degrees);
    let level = light.scale * dot3([-direction[0], -direction[1], -direction[2]], normal).max(0.0)
      + light.offset;
    for (channel, color) in shade.iter_mut().zip(light.color) {
      *channel += color * level;
    }
  }
  shade.map(|channel| channel.clamp(0.12, 2.5))
}

fn legacy_light_rig_surface_specular(
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

/// Resolves only the diffuse term of the fixed-function material equation.
///
/// MS-OI29500 defines diffuse and specular color as independent material
/// properties, and Direct3D 9 adds their lighting terms. In particular, a
/// material's specular color must not amplify diffuse values above 1.0.
fn material_diffuse_shade(
  scene: &a::Scene3DType,
  normal: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 3] {
  scale_shade(
    light_rig_surface_shade(scene, normal),
    material_diffusion(material),
  )
}

fn legacy_material_diffuse_shade(
  scene: &a::Scene3DType,
  normal: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 3] {
  scale_shade(
    legacy_light_rig_surface_shade(scene, normal),
    material_diffusion(material),
  )
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
  let color = match (
    red.checked_div(alpha),
    green.checked_div(alpha),
    blue.checked_div(alpha),
  ) {
    (Some(red), Some(green), Some(blue)) => RgbColor {
      r: red as u8,
      g: green as u8,
      b: blue as u8,
    },
    _ => RgbColor {
      r: 128,
      g: 128,
      b: 128,
    },
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

struct ProjectedImageOptions {
  projection: Static3dProjection,
  z: f32,
  bounds: (i32, i32, i32, i32),
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  tint: Option<(Static3dColor, [f32; 3])>,
}

/// Applies the selected DrawingML material and light rig to one planar
/// surface while preserving the source fill as the material's Shape color.
/// LibreOffice's 3-D processor solves the same color model for every face;
/// the front face is not an unlit overlay on top of the extruded solid.
fn shade_planar_surface(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  options: &ProjectedImageOptions,
  model_normal: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
  word_text_lighting: bool,
) {
  let normal = lighting_surface_normal(scene, options.projection, model_normal);
  let shade = if word_text_lighting {
    material_diffuse_shade(scene, normal, material)
  } else {
    legacy_material_diffuse_shade(scene, normal, material)
  };
  let center_x = options.model_surface.left_px + options.model_surface.width_px * 0.5;
  let center_y = options.model_surface.top_px + options.model_surface.height_px * 0.5;
  let width = options.model_surface.width_px.max(1.0);
  let height = options.model_surface.height_px.max(1.0);
  for (x, y, pixel) in image.enumerate_pixels_mut() {
    if pixel[3] == 0 {
      continue;
    }
    let view_direction = surface_view_direction(
      scene,
      options.projection,
      [
        x as f32 + 0.5 - center_x,
        y as f32 + 0.5 - center_y,
        options.z,
      ],
      width,
      height,
      options.pixels_per_point,
    );
    let specular = if word_text_lighting {
      light_rig_surface_specular(scene, normal, view_direction, material)
    } else {
      legacy_light_rig_surface_specular(scene, normal, view_direction, material)
    };
    let alpha = pixel[3];
    for channel in 0..3 {
      pixel[channel] =
        shade_gouraud_channel_with_specular(pixel[channel], shade[channel], specular[channel]);
    }
    pixel[3] = alpha;
  }
}

fn composite_projected_image(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  options: ProjectedImageOptions,
) {
  let ProjectedImageOptions {
    projection,
    z,
    bounds,
    model_surface,
    pixels_per_point,
    tint,
  } = options;
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

fn composite_projected_text_geometry(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  geometry: &Static3dTextGeometry,
  options: ProjectedImageOptions,
) {
  let ProjectedImageOptions {
    projection,
    z,
    bounds: _,
    model_surface,
    pixels_per_point,
    tint,
  } = options;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);
  let matrix = plane_homography(projection, z, width, height, pixels_per_point);
  let Some(inverse) = inverse_3x3(matrix) else {
    return;
  };
  let Some(source_path) = text_geometry_path(geometry, |point| point) else {
    return;
  };
  let Some(projected_path) = text_geometry_path(geometry, |point| {
    let projected = map_homogeneous(matrix, point.0 - center_x, point.1 - center_y);
    (center_x + projected.0, center_y + projected.1)
  }) else {
    return;
  };
  let Some(source_mask) = text_geometry_mask(source.width(), source.height(), &source_path) else {
    return;
  };
  let Some(projected_mask) =
    text_geometry_mask(destination.width(), destination.height(), &projected_path)
  else {
    return;
  };

  for target_y in 0..destination.height() {
    for target_x in 0..destination.width() {
      let target_coverage = f32::from(
        projected_mask
          .pixel(target_x, target_y)
          .map_or(0, |pixel| pixel.alpha()),
      ) / 255.0;
      if target_coverage <= f32::EPSILON {
        continue;
      }
      let source_local = map_homogeneous(
        inverse,
        target_x as f32 + 0.5 - center_x,
        target_y as f32 + 0.5 - center_y,
      );
      let source_x = center_x + source_local.0 - 0.5;
      let source_y = center_y + source_local.1 - 0.5;
      let Some(mut pixel) = sample_bilinear(source, source_x, source_y) else {
        continue;
      };
      let Some(source_coverage) = sample_pixmap_alpha(&source_mask, source_x, source_y) else {
        continue;
      };
      if source_coverage <= f32::EPSILON {
        continue;
      }
      // The flat text bitmap already contains source-space edge coverage.
      // Divide that coverage out before applying the projected vector mask so
      // antialiasing is evaluated once, in the destination plane. Paint
      // opacity (including a translucent text outline) remains independent.
      let paint_opacity = (f32::from(pixel[3]) / 255.0 / source_coverage).clamp(0.0, 1.0);
      let mut alpha = (target_coverage * paint_opacity * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8;
      if let Some((color, shade)) = tint {
        alpha = ((u16::from(alpha) * u16::from(color.alpha) + 127) / 255) as u8;
        pixel = shaded_pixel(color, shade, alpha);
      } else {
        pixel[3] = alpha;
      }
      if pixel[3] != 0 {
        blend_over(destination.get_pixel_mut(target_x, target_y), pixel);
      }
    }
  }
}

fn text_geometry_path(
  geometry: &Static3dTextGeometry,
  mut map: impl FnMut((f32, f32)) -> (f32, f32),
) -> Option<tiny_skia::Path> {
  let mut builder = PathBuilder::new();
  for contour in &geometry.contours {
    let Some((&first, remaining)) = contour.points.split_first() else {
      continue;
    };
    let first = map(first);
    builder.move_to(first.0, first.1);
    for &point in remaining {
      let point = map(point);
      builder.line_to(point.0, point.1);
    }
    builder.close();
  }
  builder.finish()
}

fn text_geometry_mask(width: u32, height: u32, path: &tiny_skia::Path) -> Option<Pixmap> {
  let mut mask = Pixmap::new(width, height)?;
  let mut paint = Paint {
    anti_alias: true,
    ..Paint::default()
  };
  paint.set_color_rgba8(255, 255, 255, 255);
  mask.fill_path(path, &paint, FillRule::Winding, Transform::identity(), None);
  Some(mask)
}

fn sample_pixmap_alpha(pixmap: &Pixmap, x: f32, y: f32) -> Option<f32> {
  if x < -0.5 || y < -0.5 || x > pixmap.width() as f32 - 0.5 || y > pixmap.height() as f32 - 0.5 {
    return None;
  }
  let x0 = x.floor() as i32;
  let y0 = y.floor() as i32;
  let fraction_x = x - x0 as f32;
  let fraction_y = y - y0 as f32;
  let mut alpha = 0.0;
  for (sample_y, weight_y) in [(y0, 1.0 - fraction_y), (y0 + 1, fraction_y)] {
    if sample_y < 0 || sample_y >= pixmap.height() as i32 {
      continue;
    }
    for (sample_x, weight_x) in [(x0, 1.0 - fraction_x), (x0 + 1, fraction_x)] {
      if sample_x < 0 || sample_x >= pixmap.width() as i32 {
        continue;
      }
      alpha += f32::from(
        pixmap
          .pixel(sample_x as u32, sample_y as u32)
          .map_or(0, |pixel| pixel.alpha()),
      ) / 255.0
        * weight_x
        * weight_y;
    }
  }
  Some(alpha)
}

struct VariableZProjectedImageOptions {
  projection: Static3dProjection,
  base_z: f32,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
}

struct VariableZSurface {
  pixel_offsets: Vec<f32>,
  vertex_offsets: Option<Vec<f32>>,
}

fn composite_projected_variable_z_image(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  surface: &VariableZSurface,
  options: VariableZProjectedImageOptions,
) {
  let z_offsets = &surface.pixel_offsets;
  if source.dimensions() != destination.dimensions()
    || z_offsets.len() != source.width() as usize * source.height() as usize
    || surface.vertex_offsets.as_ref().is_some_and(|offsets| {
      offsets.len() != (source.width() as usize + 1) * (source.height() as usize + 1)
    })
  {
    return;
  }
  let VariableZProjectedImageOptions {
    projection,
    base_z,
    model_surface,
    pixels_per_point,
  } = options;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);
  let source_width = source.width() as usize;
  let source_height = source.height() as usize;
  let vertex_width = source_width + 1;
  let mut vertex_z_sum = vec![0.0_f32; vertex_width * (source_height + 1)];
  let mut vertex_sample_count = vec![0_u8; vertex_z_sum.len()];

  // A bevel is a continuous height field. Accumulate the four vertices of
  // every covered source cell so neighbouring cells share exactly the same
  // projected edge. Projecting only pixel centres and bilinearly splatting
  // them leaves holes wherever perspective stretches the slope (most visibly
  // at the right ends of horizontal text strokes), allowing the raised cap to
  // hide too much of the lower bevel.
  for (x, y, pixel) in source.enumerate_pixels() {
    if pixel[3] == 0 {
      continue;
    }
    let z = z_offsets[y as usize * source_width + x as usize];
    for (vertex_x, vertex_y) in [
      (x as usize, y as usize),
      (x as usize + 1, y as usize),
      (x as usize + 1, y as usize + 1),
      (x as usize, y as usize + 1),
    ] {
      let index = vertex_y * vertex_width + vertex_x;
      vertex_z_sum[index] += z;
      vertex_sample_count[index] = vertex_sample_count[index].saturating_add(1);
    }
  }

  #[derive(Clone, Copy)]
  struct ProjectedHeightCell {
    points: [(f32, f32); 4],
    average_z: f32,
    color: Rgba<u8>,
  }

  let project = |x: f32, y: f32, z: f32| {
    let projected = project_local_pixels(
      projection,
      x - center_x,
      y - center_y,
      base_z + z,
      width,
      height,
      pixels_per_point,
    );
    (center_x + projected.0, center_y + projected.1)
  };
  let mut cells = Vec::new();
  for (x, y, pixel) in source.enumerate_pixels() {
    if pixel[3] == 0 {
      continue;
    }
    let vertex_z = |vertex_x: usize, vertex_y: usize| {
      let index = vertex_y * vertex_width + vertex_x;
      if let Some(offsets) = &surface.vertex_offsets
        && offsets[index].is_finite()
      {
        return offsets[index];
      }
      let count = vertex_sample_count[index];
      if count == 0 {
        z_offsets[y as usize * source_width + x as usize]
      } else {
        vertex_z_sum[index] / f32::from(count)
      }
    };
    let left = x as usize;
    let top = y as usize;
    let z = [
      vertex_z(left, top),
      vertex_z(left + 1, top),
      vertex_z(left + 1, top + 1),
      vertex_z(left, top + 1),
    ];
    cells.push(ProjectedHeightCell {
      points: [
        project(x as f32, y as f32, z[0]),
        project(x as f32 + 1.0, y as f32, z[1]),
        project(x as f32 + 1.0, y as f32 + 1.0, z[2]),
        project(x as f32, y as f32 + 1.0, z[3]),
      ],
      average_z: z.iter().sum::<f32>() * 0.25,
      color: *pixel,
    });
  }
  // Positive z is nearer the camera for the DrawingML text surface. Paint
  // farther cells first, matching the strip renderer used by folded presets.
  cells.sort_by(|left, right| left.average_z.total_cmp(&right.average_z));

  let Some(mut projected_layer) = Pixmap::new(destination.width(), destination.height()) else {
    return;
  };
  for cell in cells {
    let mut builder = PathBuilder::new();
    builder.move_to(cell.points[0].0, cell.points[0].1);
    for point in &cell.points[1..] {
      builder.line_to(point.0, point.1);
    }
    builder.close();
    let Some(path) = builder.finish() else {
      continue;
    };
    let mut paint = Paint {
      anti_alias: true,
      ..Paint::default()
    };
    paint.set_color_rgba8(cell.color[0], cell.color[1], cell.color[2], cell.color[3]);
    projected_layer.fill_path(
      &path,
      &paint,
      FillRule::Winding,
      Transform::identity(),
      None,
    );
  }
  for (target, source) in destination.pixels_mut().zip(projected_layer.pixels()) {
    let source = source.demultiply();
    if source.alpha() != 0 {
      blend_over(
        target,
        Rgba([source.red(), source.green(), source.blue(), source.alpha()]),
      );
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

struct ExtrusionEdgeOptions<'a> {
  bounds: (i32, i32, i32, i32),
  model_surface: Static3dSurface,
  projection: Static3dProjection,
  front_z: f32,
  back_z: f32,
  pixels_per_point: f32,
  steps: u32,
  tint: Static3dColor,
  scene: &'a a::Scene3DType,
  material: Option<a::PresetMaterialTypeValues>,
  wireframe: bool,
}

fn text_extrusion_edge_triangles(
  geometry: &Static3dTextGeometry,
  options: ExtrusionEdgeOptions<'_>,
) -> Vec<TextSurfaceTriangle> {
  let ExtrusionEdgeOptions {
    bounds: _,
    model_surface,
    projection,
    front_z,
    back_z,
    pixels_per_point,
    steps: _,
    tint,
    scene,
    material,
    wireframe: _,
  } = options;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);
  let project = |point: (f32, f32), z, color: Rgba<u8>| {
    let model_point = [point.0 - center_x, point.1 - center_y, z];
    let projected = project_local_pixels(
      projection,
      model_point[0],
      model_point[1],
      z,
      width,
      height,
      pixels_per_point,
    );
    TextSurfaceVertex {
      point: (center_x + projected.0, center_y + projected.1),
      visibility_depth: text_surface_visibility_depth(projection, model_point, pixels_per_point),
      color: color.0.map(f32::from),
    }
  };
  let mut triangles = Vec::new();

  for contour in &geometry.contours {
    for (&first, &second) in contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
    {
      let edge = (second.0 - first.0, second.1 - first.1);
      let edge_length = edge.0.hypot(edge.1);
      if edge_length <= 1.0e-4 {
        continue;
      }
      let edge = (edge.0 / edge_length, edge.1 / edge_length);
      let inward = if geometry.solid_on_right {
        (-edge.1, edge.0)
      } else {
        (edge.1, -edge.0)
      };
      let outward = [-inward.0, -inward.1, 0.0];
      let model_point = [
        (first.0 + second.0) * 0.5 - center_x,
        (first.1 + second.1) * 0.5 - center_y,
        (front_z + back_z) * 0.5,
      ];
      if !surface_faces_camera(
        projection,
        outward,
        model_point,
        width,
        height,
        pixels_per_point,
      ) {
        continue;
      }
      let surface_normal = lighting_surface_normal(scene, projection, outward);
      let shade = material_diffuse_shade(scene, surface_normal, material);
      let view_direction = surface_view_direction(
        scene,
        projection,
        model_point,
        width,
        height,
        pixels_per_point,
      );
      let specular = light_rig_surface_specular(scene, surface_normal, view_direction, material);
      let color = shaded_pixel_with_specular(tint, shade, specular, tint.alpha);
      let front_first = project(first, front_z, color);
      let front_second = project(second, front_z, color);
      let back_second = project(second, back_z, color);
      let back_first = project(first, back_z, color);
      triangles.push(TextSurfaceTriangle {
        vertices: [front_first, front_second, back_second],
      });
      triangles.push(TextSurfaceTriangle {
        vertices: [back_second, back_first, front_first],
      });
    }
  }
  triangles
}

fn composite_extrusion_edges(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  options: ExtrusionEdgeOptions<'_>,
) {
  let ExtrusionEdgeOptions {
    bounds,
    model_surface,
    projection,
    front_z,
    back_z,
    pixels_per_point,
    steps,
    tint,
    scene,
    material,
    wireframe,
  } = options;
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
      let shade = legacy_material_diffuse_shade(scene, surface_normal, material);
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
      let specular =
        legacy_light_rig_surface_specular(scene, surface_normal, view_direction, material);
      // This mesh is reconstructed from the rasterized glyph boundary. Keep
      // the source pixel's antialias coverage on its swept side face;
      // promoting every fringe pixel to an opaque quad turns adjacent glyph
      // edges into dark rectangular bridges.
      let side_alpha = ((u16::from(source_pixel[3]) * u16::from(tint.alpha) + 127) / 255) as u8;
      let color = shaded_pixel_with_specular(tint, shade, specular, side_alpha);
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
        let mut paint = Paint {
          anti_alias: true,
          ..Paint::default()
        };
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
    let mut shade = legacy_material_diffuse_shade(scene, surface_normal, material);
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

pub(crate) fn alpha_bounds(image: &RgbaImage) -> Option<(i32, i32, i32, i32)> {
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

#[derive(Clone, Copy)]
struct BevelOptions<'a> {
  width: i32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  scene: &'a a::Scene3DType,
  projection: Static3dProjection,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  surface_z: f32,
  material: Option<a::PresetMaterialTypeValues>,
  back_face: bool,
}

#[derive(Clone, Copy)]
struct TextBevelOptions<'a> {
  width: f32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  scene: &'a a::Scene3DType,
  projection: Static3dProjection,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  surface_z: f32,
  material: Option<a::PresetMaterialTypeValues>,
}

#[derive(Clone, Copy, Debug)]
struct TextSurfaceVertex {
  point: (f32, f32),
  visibility_depth: f32,
  color: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
struct TextSurfaceTriangle {
  vertices: [TextSurfaceVertex; 3],
}

#[derive(Clone, Copy)]
struct TextSurfaceRasterSample {
  visibility_depth: f32,
  color: [f32; 4],
  covered: bool,
}

impl Default for TextSurfaceRasterSample {
  fn default() -> Self {
    Self {
      visibility_depth: f32::NEG_INFINITY,
      color: [0.0; 4],
      covered: false,
    }
  }
}

fn text_surface_edge(a: (f32, f32), b: (f32, f32), point: (f32, f32)) -> f32 {
  (b.0 - a.0) * (point.1 - a.1) - (b.1 - a.1) * (point.0 - a.0)
}

fn text_surface_visibility_depth(
  projection: Static3dProjection,
  model_point: [f32; 3],
  pixels_per_point: f32,
) -> f32 {
  let camera_depth = dot3(projection.rotation[2], model_point);
  if projection.parallel {
    return camera_depth;
  }
  // The perspective homography's denominator is the camera-space distance
  // to the viewpoint. Its reciprocal is linear in screen space, just like a
  // hardware depth-buffer value, and remains monotonic toward the camera.
  let viewpoint_z = projection
    .perspective_distance_pt
    .unwrap_or(25_000.0 * 72.0 / 2_540.0)
    * pixels_per_point;
  (viewpoint_z - camera_depth).max(1.0e-6).recip()
}

fn text_geometry_contains(geometry: &Static3dTextGeometry, point: (f32, f32)) -> bool {
  let mut winding = 0_i32;
  for contour in &geometry.contours {
    for (&first, &second) in contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
    {
      if first.1 <= point.1 {
        if second.1 > point.1 && text_surface_edge(first, second, point) > 0.0 {
          winding += 1;
        }
      } else if second.1 <= point.1 && text_surface_edge(first, second, point) < 0.0 {
        winding -= 1;
      }
    }
  }
  winding != 0
}

fn composite_text_solid_surfaces(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  source_geometry: &Static3dTextGeometry,
  planar_geometry: &Static3dTextGeometry,
  triangles: &[TextSurfaceTriangle],
  options: ProjectedImageOptions,
) {
  const SAMPLE_GRID: usize = 4;
  const SAMPLE_COUNT: usize = SAMPLE_GRID * SAMPLE_GRID;

  let ProjectedImageOptions {
    projection,
    z: planar_z,
    bounds: _,
    model_surface,
    pixels_per_point,
    tint: _,
  } = options;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let model_width = model_surface.width_px.max(1.0);
  let model_height = model_surface.height_px.max(1.0);
  let planar_matrix = plane_homography(
    projection,
    planar_z,
    model_width,
    model_height,
    pixels_per_point,
  );
  let planar_inverse = inverse_3x3(planar_matrix);

  let triangle_bounds = triangles
    .iter()
    .flat_map(|triangle| triangle.vertices.iter().map(|vertex| vertex.point))
    .fold(None, |bounds, point| {
      Some(bounds.map_or(
        (point.0, point.1, point.0, point.1),
        |(left, top, right, bottom): (f32, f32, f32, f32)| {
          (
            left.min(point.0),
            top.min(point.1),
            right.max(point.0),
            bottom.max(point.1),
          )
        },
      ))
    });
  let planar_bounds = planar_geometry
    .contours
    .iter()
    .flat_map(|contour| contour.points.iter().copied())
    .map(|point| {
      let projected = map_homogeneous(planar_matrix, point.0 - center_x, point.1 - center_y);
      (center_x + projected.0, center_y + projected.1)
    })
    .fold(None, |bounds, point| {
      Some(bounds.map_or(
        (point.0, point.1, point.0, point.1),
        |(left, top, right, bottom): (f32, f32, f32, f32)| {
          (
            left.min(point.0),
            top.min(point.1),
            right.max(point.0),
            bottom.max(point.1),
          )
        },
      ))
    });
  let bounds = match (triangle_bounds, planar_bounds) {
    (Some(first), Some(second)) => Some((
      first.0.min(second.0),
      first.1.min(second.1),
      first.2.max(second.2),
      first.3.max(second.3),
    )),
    (Some(bounds), None) | (None, Some(bounds)) => Some(bounds),
    (None, None) => None,
  };
  let Some((left, top, right, bottom)) = bounds else {
    return;
  };
  let left = (left.floor() as i32).clamp(0, destination.width() as i32);
  let top = (top.floor() as i32).clamp(0, destination.height() as i32);
  let right = (right.ceil() as i32).clamp(0, destination.width() as i32);
  let bottom = (bottom.ceil() as i32).clamp(0, destination.height() as i32);
  if right <= left || bottom <= top {
    return;
  }
  let raster_width = (right - left) as usize;
  let raster_height = (bottom - top) as usize;
  let mut samples =
    vec![TextSurfaceRasterSample::default(); raster_width * raster_height * SAMPLE_COUNT];

  // Microsoft's Direct2D 3-D text samples emit one shared triangle mesh and
  // explicitly require that adjacent faces neither overlap nor leave
  // T-junctions. Rasterizing every quad through source-over violates that
  // contract at each antialiased shared edge. Give each subpixel sample one
  // surface owner instead, and use camera-space depth for folded Office
  // profiles whose branches overlap after projection.
  for triangle in triangles {
    let [first, second, third] = triangle.vertices;
    let signed_area = text_surface_edge(first.point, second.point, third.point);
    if signed_area.abs() <= 1.0e-6 {
      continue;
    }
    let triangle_left = first.point.0.min(second.point.0).min(third.point.0).floor() as i32;
    let triangle_top = first.point.1.min(second.point.1).min(third.point.1).floor() as i32;
    let triangle_right = first.point.0.max(second.point.0).max(third.point.0).ceil() as i32;
    let triangle_bottom = first.point.1.max(second.point.1).max(third.point.1).ceil() as i32;
    for pixel_y in triangle_top.max(top)..triangle_bottom.min(bottom) {
      for pixel_x in triangle_left.max(left)..triangle_right.min(right) {
        let local_pixel = (pixel_y - top) as usize * raster_width + (pixel_x - left) as usize;
        for sample_y in 0..SAMPLE_GRID {
          for sample_x in 0..SAMPLE_GRID {
            let point = (
              pixel_x as f32 + (sample_x as f32 + 0.5) / SAMPLE_GRID as f32,
              pixel_y as f32 + (sample_y as f32 + 0.5) / SAMPLE_GRID as f32,
            );
            let first_weight = text_surface_edge(second.point, third.point, point) / signed_area;
            let second_weight = text_surface_edge(third.point, first.point, point) / signed_area;
            let third_weight = text_surface_edge(first.point, second.point, point) / signed_area;
            if first_weight < -1.0e-5 || second_weight < -1.0e-5 || third_weight < -1.0e-5 {
              continue;
            }
            let visibility_depth = first.visibility_depth * first_weight
              + second.visibility_depth * second_weight
              + third.visibility_depth * third_weight;
            let sample_index = local_pixel * SAMPLE_COUNT + sample_y * SAMPLE_GRID + sample_x;
            let sample = &mut samples[sample_index];
            if sample.covered && visibility_depth <= sample.visibility_depth {
              continue;
            }
            sample.covered = true;
            sample.visibility_depth = visibility_depth;
            for channel in 0..4 {
              sample.color[channel] = first.color[channel] * first_weight
                + second.color[channel] * second_weight
                + third.color[channel] * third_weight;
            }
          }
        }
      }
    }
  }

  // The cap is a vector plane rather than another painter layer. Test each
  // destination subpixel against the inset winding path, map it back through
  // the same homography used for projection, and submit it to the same depth
  // samples as the bevel and extrusion walls. This is the software analogue
  // of Microsoft's one-mesh/one-depth-stencil text samples.
  if let (Some(planar_inverse), Some(source_path)) = (
    planar_inverse,
    text_geometry_path(source_geometry, |point| point),
  ) && let Some(source_mask) = text_geometry_mask(source.width(), source.height(), &source_path)
  {
    let (planar_left, planar_top, planar_right, planar_bottom) =
      planar_bounds.unwrap_or((left as f32, top as f32, right as f32, bottom as f32));
    let planar_left = (planar_left.floor() as i32).max(left);
    let planar_top = (planar_top.floor() as i32).max(top);
    let planar_right = (planar_right.ceil() as i32).min(right);
    let planar_bottom = (planar_bottom.ceil() as i32).min(bottom);
    for pixel_y in planar_top..planar_bottom {
      for pixel_x in planar_left..planar_right {
        let local_pixel = (pixel_y - top) as usize * raster_width + (pixel_x - left) as usize;
        for sample_y in 0..SAMPLE_GRID {
          for sample_x in 0..SAMPLE_GRID {
            let target = (
              pixel_x as f32 + (sample_x as f32 + 0.5) / SAMPLE_GRID as f32,
              pixel_y as f32 + (sample_y as f32 + 0.5) / SAMPLE_GRID as f32,
            );
            let source_local =
              map_homogeneous(planar_inverse, target.0 - center_x, target.1 - center_y);
            let source_point = (center_x + source_local.0, center_y + source_local.1);
            if !text_geometry_contains(planar_geometry, source_point) {
              continue;
            }
            let Some(mut color) =
              sample_bilinear(source, source_point.0 - 0.5, source_point.1 - 0.5)
            else {
              continue;
            };
            let Some(source_coverage) =
              sample_pixmap_alpha(&source_mask, source_point.0 - 0.5, source_point.1 - 0.5)
            else {
              continue;
            };
            if source_coverage <= f32::EPSILON {
              continue;
            }
            let paint_opacity = (f32::from(color[3]) / 255.0 / source_coverage).clamp(0.0, 1.0);
            if paint_opacity <= f32::EPSILON {
              continue;
            }
            color[3] = (paint_opacity * 255.0).round().clamp(0.0, 255.0) as u8;
            let model_point = [source_local.0, source_local.1, planar_z];
            let visibility_depth =
              text_surface_visibility_depth(projection, model_point, pixels_per_point);
            let sample_index = local_pixel * SAMPLE_COUNT + sample_y * SAMPLE_GRID + sample_x;
            let sample = &mut samples[sample_index];
            if sample.covered && visibility_depth <= sample.visibility_depth {
              continue;
            }
            sample.covered = true;
            sample.visibility_depth = visibility_depth;
            sample.color = color.0.map(f32::from);
          }
        }
      }
    }
  }

  for local_y in 0..raster_height {
    for local_x in 0..raster_width {
      let pixel_index = local_y * raster_width + local_x;
      let mut premultiplied = [0.0_f32; 3];
      let mut alpha_sum = 0.0_f32;
      for sample in &samples[pixel_index * SAMPLE_COUNT..(pixel_index + 1) * SAMPLE_COUNT] {
        if !sample.covered {
          continue;
        }
        let alpha = (sample.color[3] / 255.0).clamp(0.0, 1.0);
        alpha_sum += alpha;
        for (accumulator, channel) in premultiplied.iter_mut().zip(&sample.color) {
          *accumulator += channel.clamp(0.0, 255.0) * alpha;
        }
      }
      if alpha_sum <= f32::EPSILON {
        continue;
      }
      let alpha = alpha_sum / SAMPLE_COUNT as f32;
      let color = Rgba([
        (premultiplied[0] / alpha_sum).round().clamp(0.0, 255.0) as u8,
        (premultiplied[1] / alpha_sum).round().clamp(0.0, 255.0) as u8,
        (premultiplied[2] / alpha_sum).round().clamp(0.0, 255.0) as u8,
        (alpha * 255.0).round().clamp(0.0, 255.0) as u8,
      ]);
      blend_over(
        destination.get_pixel_mut(
          (left as usize + local_x) as u32,
          (top as usize + local_y) as u32,
        ),
        color,
      );
    }
  }
}

fn text_bevel_triangles(
  source: &RgbaImage,
  geometry: &Static3dTextGeometry,
  options: TextBevelOptions<'_>,
) -> Vec<TextSurfaceTriangle> {
  let TextBevelOptions {
    width,
    height,
    preset,
    scene,
    projection,
    model_surface,
    pixels_per_point,
    surface_z,
    material,
  } = options;
  if width <= f32::EPSILON || height <= f32::EPSILON {
    return Vec::new();
  }
  let Some(source_path) = text_geometry_path(geometry, |point| point) else {
    return Vec::new();
  };
  let Some(source_mask) = text_geometry_mask(source.width(), source.height(), &source_path) else {
    return Vec::new();
  };
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let model_width = model_surface.width_px.max(1.0);
  let model_height = model_surface.height_px.max(1.0);
  let project = |point: (f32, f32), z, color: [u8; 4]| {
    let model_point = [point.0 - center_x, point.1 - center_y, z];
    let projected = project_local_pixels(
      projection,
      model_point[0],
      model_point[1],
      z,
      model_width,
      model_height,
      pixels_per_point,
    );
    TextSurfaceVertex {
      point: (center_x + projected.0, center_y + projected.1),
      visibility_depth: text_surface_visibility_depth(projection, model_point, pixels_per_point),
      color: color.map(f32::from),
    }
  };
  // MS-OI29500 publishes every Office preset as a sequence of bevel-space
  // curves. Keep those curves parametric: `relaxedInset`, `softRound`, and
  // several other presets fold back in y, so reducing them to one height per
  // alpha-mask distance discards a complete visible surface. The shared mesh
  // is depth-tested below, so folded branches retain their authored order in
  // profile space while camera-space visibility decides the output sample.
  let profile = bevel_profile(preset);
  let subdivisions = (width * 1.5 / profile.len() as f32).ceil().clamp(4.0, 16.0) as usize;
  let mut profile_strips = Vec::with_capacity(profile.len() * subdivisions);
  for segment_index in 0..profile.len() {
    for subdivision in 0..subdivisions {
      let outer_t = subdivision as f32 / subdivisions as f32;
      let inner_t = (subdivision + 1) as f32 / subdivisions as f32;
      let middle_t = (outer_t + inner_t) * 0.5;
      profile_strips.push((
        bevel_profile_sample(preset, segment_index, outer_t),
        bevel_profile_sample(preset, segment_index, inner_t),
        bevel_profile_sample(preset, segment_index, middle_t),
      ));
    }
  }
  let mut triangles = Vec::new();
  for contour in &geometry.contours {
    let edge_normals = text_3d_contour_edge_normals(&contour.points, geometry.solid_on_right);
    for &(outer_profile, inner_profile, middle_profile) in &profile_strips {
      let outer_ring = offset_text_3d_contour(
        &contour.points,
        width * outer_profile.inset,
        geometry.solid_on_right,
      );
      let inner_ring = offset_text_3d_contour(
        &contour.points,
        width * inner_profile.inset,
        geometry.solid_on_right,
      );
      for index in 0..contour.points.len() {
        let next = (index + 1) % contour.points.len();
        let outer_first = outer_ring[index];
        let outer_second = outer_ring[next];
        let inner_second = inner_ring[next];
        let inner_first = inner_ring[index];
        let source_point = (
          (outer_first.0 + outer_second.0 + inner_second.0 + inner_first.0) * 0.25,
          (outer_first.1 + outer_second.1 + inner_second.1 + inner_first.1) * 0.25,
        );
        let Some(source_coverage) =
          sample_pixmap_alpha(&source_mask, source_point.0 - 0.5, source_point.1 - 0.5)
        else {
          continue;
        };
        if source_coverage <= f32::EPSILON {
          continue;
        }
        let normal_xy = height * middle_profile.height_tangent;
        let normal_z = width * middle_profile.inset_tangent;
        let light_color = |outward: [f32; 2], point: (f32, f32)| {
          // Adjacent contour quads share this endpoint. Sample the material
          // paint at that shared point as well as sharing its interpolated
          // normal; sampling once at each quad's center gives the two copies
          // of the same vertex different colors and exposes the tessellation
          // edge at tight glyph joins.
          let source_pixel = sample_bilinear(source, point.0 - 0.5, point.1 - 0.5)?;
          let source_coverage = sample_pixmap_alpha(&source_mask, point.0 - 0.5, point.1 - 0.5)?;
          if source_coverage <= f32::EPSILON {
            return None;
          }
          let paint_opacity =
            (f32::from(source_pixel[3]) / 255.0 / source_coverage).clamp(0.0, 1.0);
          if paint_opacity <= f32::EPSILON {
            return None;
          }
          let mut normal = [outward[0] * normal_xy, outward[1] * normal_xy, normal_z];
          normalize3(&mut normal);
          let normal = lighting_surface_normal(scene, projection, normal);
          let model_point = [
            point.0 - center_x,
            point.1 - center_y,
            surface_z + middle_profile.height * height,
          ];
          let view_direction = surface_view_direction(
            scene,
            projection,
            model_point,
            model_width,
            model_height,
            pixels_per_point,
          );
          let specular = light_rig_surface_specular(scene, normal, view_direction, material);
          let shade = material_diffuse_shade(scene, normal, material);
          let mut color = [0_u8; 4];
          for channel in 0..3 {
            let original = f32::from(source_pixel[channel]);
            let lit = original * shade[channel] + 255.0 * specular[channel];
            color[channel] = lit.round().clamp(0.0, 255.0) as u8;
          }
          color[3] = (paint_opacity * 255.0).round().clamp(0.0, 255.0) as u8;
          Some(color)
        };
        let source_first = (
          (outer_first.0 + inner_first.0) * 0.5,
          (outer_first.1 + inner_first.1) * 0.5,
        );
        let source_second = (
          (outer_second.0 + inner_second.0) * 0.5,
          (outer_second.1 + inner_second.1) * 0.5,
        );
        let (start_normal, end_normal) = edge_normals[index];
        let (start_color, end_color) = match (
          light_color(start_normal, source_first),
          light_color(end_normal, source_second),
        ) {
          (Some(start), Some(end)) => (start, end),
          (Some(color), None) | (None, Some(color)) => (color, color),
          (None, None) => continue,
        };
        let outer_z = surface_z + outer_profile.height * height;
        let inner_z = surface_z + inner_profile.height * height;
        let outer_first = project(outer_first, outer_z, start_color);
        let outer_second = project(outer_second, outer_z, end_color);
        let inner_second = project(inner_second, inner_z, end_color);
        let inner_first = project(inner_first, inner_z, start_color);
        triangles.push(TextSurfaceTriangle {
          vertices: [outer_first, outer_second, inner_second],
        });
        triangles.push(TextSurfaceTriangle {
          vertices: [inner_second, inner_first, outer_first],
        });
      }
    }
  }
  triangles
}

fn bevel_distance_field(source: &RgbaImage, width: i32) -> Vec<f32> {
  let image_width = source.width() as usize;
  let image_height = source.height() as usize;
  let limit = width.max(1) as f32 + 1.0;
  // A one-pixel transparent border makes the distance to the image edge
  // explicit. The former two-pass 8-neighbour chamfer overestimated slopes
  // such as sqrt(5) as 1 + sqrt(2), quantizing circle-bevel normals and
  // producing broad flat lighting bands on diagonal glyph strokes.
  let padded_width = image_width + 2;
  let padded_height = image_height + 2;
  let maximum_squared_distance =
    (padded_width * padded_width + padded_height * padded_height) as f32 + 1.0;
  let mut horizontal = vec![0.0; padded_width * padded_height];
  let mut input = vec![0.0; padded_width.max(padded_height)];
  let mut output = vec![0.0; padded_width.max(padded_height)];

  for y in 0..padded_height {
    for (x, value) in input[..padded_width].iter_mut().enumerate() {
      *value = if x == 0
        || y == 0
        || x + 1 == padded_width
        || y + 1 == padded_height
        || source.get_pixel((x - 1) as u32, (y - 1) as u32)[3] == 0
      {
        0.0
      } else {
        maximum_squared_distance
      };
    }
    squared_distance_transform_1d(&input[..padded_width], &mut output[..padded_width]);
    horizontal[y * padded_width..(y + 1) * padded_width].copy_from_slice(&output[..padded_width]);
  }

  let mut distances = vec![limit; image_width * image_height];
  for x in 0..padded_width {
    for y in 0..padded_height {
      input[y] = horizontal[y * padded_width + x];
    }
    squared_distance_transform_1d(&input[..padded_height], &mut output[..padded_height]);
    if x == 0 || x + 1 == padded_width {
      continue;
    }
    for y in 1..=image_height {
      distances[(y - 1) * image_width + (x - 1)] = output[y].sqrt().min(limit);
    }
  }
  distances
}

fn squared_distance_transform_1d(input: &[f32], output: &mut [f32]) {
  debug_assert_eq!(input.len(), output.len());
  let count = input.len();
  if count == 0 {
    return;
  }
  let mut sites = vec![0_usize; count];
  let mut boundaries = vec![0.0_f32; count + 1];
  let mut last = 0_usize;
  boundaries[0] = f32::NEG_INFINITY;
  boundaries[1] = f32::INFINITY;

  for candidate in 1..count {
    let mut intersection;
    loop {
      let site = sites[last];
      intersection = ((input[candidate] + (candidate * candidate) as f32)
        - (input[site] + (site * site) as f32))
        / (2.0 * (candidate - site) as f32);
      if intersection > boundaries[last] || last == 0 {
        break;
      }
      last -= 1;
    }
    last += 1;
    sites[last] = candidate;
    boundaries[last] = intersection;
    boundaries[last + 1] = f32::INFINITY;
  }

  last = 0;
  for (position, result) in output.iter_mut().enumerate() {
    while boundaries[last + 1] < position as f32 {
      last += 1;
    }
    let delta = position as f32 - sites[last] as f32;
    *result = delta * delta + input[sites[last]];
  }
}

fn composite_bevel(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  options: BevelOptions<'_>,
) -> Vec<f32> {
  let BevelOptions {
    width,
    height,
    preset,
    scene,
    projection,
    model_surface,
    pixels_per_point,
    surface_z,
    material,
    back_face,
  } = options;
  let distance_field = bevel_distance_field(source, width);
  let distance_at = |x: i32, y: i32| -> f32 {
    if x < 0 || y < 0 || x >= source.width() as i32 || y >= source.height() as i32 {
      0.0
    } else {
      distance_field[y as usize * source.width() as usize + x as usize]
    }
  };
  let mut height_offsets = vec![0.0; source.width() as usize * source.height() as usize];
  // OOXML shape coordinates and LibreOffice's Scene3DHelper use +z toward the
  // observer. The front bevel therefore has a positive-z normal; the back
  // bevel uses the opposite orientation. Rig vectors describe the direction
  // in which light travels and are negated separately by the lighting code.
  for y in 0..source.height() as i32 {
    for x in 0..source.width() as i32 {
      let pixel = source.get_pixel(x as u32, y as u32);
      if pixel[3] == 0 {
        continue;
      }
      // Pixel centers on the first covered row are one pixel from the first
      // transparent center. Subtract that unit so the authored profile starts
      // at zero on the rasterized outline, matching the former cardinal-edge
      // convention while retaining diagonal curvature.
      let distance = (distance_at(x, y) - 1.0).max(0.0);
      if distance >= width as f32 {
        continue;
      }
      let inward_fraction = distance / width.max(1) as f32;
      let (profile_height, profile_dx, profile_dy) =
        if preset.unwrap_or(a::BevelPresetValues::Circle) == a::BevelPresetValues::Circle {
          circle_bevel_profile(inward_fraction)
        } else {
          // The current bounded raster lowering cannot represent the
          // self-overlapping profiles used by presets such as `divot`. Keep
          // their established linear tangent until those profiles are
          // lowered as explicit surfaces rather than guessing one branch.
          (f32::NAN, 1.0, 1.0)
        };
      let normal_xy = height * profile_dx;
      let normal_z = width as f32 * profile_dy * if back_face { -1.0 } else { 1.0 };
      if !profile_height.is_nan() {
        let index = y as usize * source.width() as usize + x as usize;
        height_offsets[index] = profile_height * height * if back_face { -1.0 } else { 1.0 };
      }
      let mut outward = [
        distance_at(x - 1, y) - distance_at(x + 1, y),
        distance_at(x, y - 1) - distance_at(x, y + 1),
      ];
      let outward_length = outward[0].hypot(outward[1]);
      if outward_length > f32::EPSILON {
        outward[0] /= outward_length;
        outward[1] /= outward_length;
      } else {
        outward = alpha_boundary_normal(source, x, y);
      }
      let mut normal = [outward[0] * normal_xy, outward[1] * normal_xy, 0.0];
      normal[2] = normal_z;
      normalize3(&mut normal);
      let normal = lighting_surface_normal(scene, projection, normal);
      let view_direction = surface_view_direction(
        scene,
        projection,
        [
          x as f32 + 0.5 - (model_surface.left_px + model_surface.width_px * 0.5),
          y as f32 + 0.5 - (model_surface.top_px + model_surface.height_px * 0.5),
          surface_z,
        ],
        model_surface.width_px.max(1.0),
        model_surface.height_px.max(1.0),
        pixels_per_point,
      );
      let specular = legacy_light_rig_surface_specular(scene, normal, view_direction, material);
      let shade = legacy_material_diffuse_shade(scene, normal, material);
      let weight = if profile_height.is_nan() {
        1.0 - inward_fraction * inward_fraction * (3.0 - 2.0 * inward_fraction)
      } else {
        // MS-OI29500 defines bevel-space x as geometric distance away from
        // the face. It controls this pixel's projected z position, not how
        // much of the surface receives material lighting. Every covered
        // circle-bevel sample is a complete physical surface; blending by x
        // suppresses all lighting at the outer edge and half of it through
        // the middle of the curve, flattening the authored highlight bands.
        1.0
      };
      let target = destination.get_pixel_mut(x as u32, y as u32);
      for channel in 0..3 {
        // MS-OI29500's material table feeds the D3D9 fixed-function
        // specular term additively. Applying diffuse alone makes a bevel on
        // black picture pixels permanently black, while PowerPoint fixed
        // output retains the lit rim (tdf170095).
        let lit = f32::from(pixel[channel]) * shade[channel] + 255.0 * specular[channel];
        target[channel] = (f32::from(pixel[channel]) + (lit - f32::from(pixel[channel])) * weight)
          .round()
          .clamp(0.0, 255.0) as u8;
      }
      target[3] = pixel[3];
    }
  }
  height_offsets
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
    BevelOptions, ProjectedImageOptions, Static3dColor, Static3dRenderOptions, Static3dStyleParts,
    Static3dSurface, Static3dTextGeometry, TextSurfaceTriangle, TextSurfaceVertex, apply_static_3d,
    bevel_distance_field, bevel_profile_sample, bevel_terminal_inset, camera_projection,
    circle_bevel_profile, composite_bevel, composite_text_solid_surfaces, light_rig,
    light_rig_surface_shade, mask_static_3d_text_surface_paint, material_diffuse_shade,
    output_padding, project_static_3d_front_face, projected_front_region_output_bounds,
    projected_output_bounds, projected_region_output_bounds, resolve_static_3d_style,
    text_3d_contour_edge_normals, text_geometry_mask, text_geometry_path,
  };
  use crate::common::{PathCommand, Point, Pt, Rect, Size};
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

  fn text_surface_quad(visibility_depth: f32, color: [f32; 4]) -> [TextSurfaceTriangle; 2] {
    let vertex = |point| TextSurfaceVertex {
      point,
      visibility_depth,
      color,
    };
    [
      TextSurfaceTriangle {
        vertices: [vertex((0.0, 0.0)), vertex((1.0, 0.0)), vertex((1.0, 1.0))],
      },
      TextSurfaceTriangle {
        vertices: [vertex((1.0, 1.0)), vertex((0.0, 1.0)), vertex((0.0, 0.0))],
      },
    ]
  }

  fn rasterize_test_text_surfaces(triangles: &[TextSurfaceTriangle]) -> RgbaImage {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let projection = camera_projection(&scene, 0.0);
    let geometry = Static3dTextGeometry {
      contours: Vec::new(),
      solid_on_right: true,
    };
    let source = RgbaImage::new(1, 1);
    let mut destination = RgbaImage::new(1, 1);
    composite_text_solid_surfaces(
      &mut destination,
      &source,
      &geometry,
      &geometry,
      triangles,
      ProjectedImageOptions {
        projection,
        z: 0.0,
        bounds: (0, 0, 0, 0),
        model_surface: Static3dSurface {
          left_px: 0.0,
          top_px: 0.0,
          width_px: 1.0,
          height_px: 1.0,
        },
        pixels_per_point: 1.0,
        tint: None,
      },
    );
    destination
  }

  #[test]
  fn shared_text_surface_depth_is_independent_of_submission_order() {
    let far = text_surface_quad(1.0, [255.0, 0.0, 0.0, 255.0]);
    let near = text_surface_quad(2.0, [0.0, 255.0, 0.0, 255.0]);
    let far_then_near = far.into_iter().chain(near).collect::<Vec<_>>();
    let near_then_far = near.into_iter().chain(far).collect::<Vec<_>>();

    let first = rasterize_test_text_surfaces(&far_then_near);
    let second = rasterize_test_text_surfaces(&near_then_far);

    assert_eq!(first, second);
    assert_eq!(first.get_pixel(0, 0), &Rgba([0, 255, 0, 255]));
  }

  #[test]
  fn shared_text_surface_edge_has_one_subpixel_owner() {
    let triangles = text_surface_quad(1.0, [255.0, 0.0, 0.0, 128.0]);

    let raster = rasterize_test_text_surfaces(&triangles);

    assert_eq!(raster.get_pixel(0, 0), &Rgba([255, 0, 0, 128]));
  }

  #[test]
  fn props_only_text_3d_uses_the_neutral_word_scene() {
    let parts = Static3dStyleParts {
      shape: Some(Box::new(a::Shape3DType::default())),
      ..Static3dStyleParts::default()
    };

    let style = resolve_static_3d_style(None, Some(&parts)).expect("props3d must remain visible");

    assert_eq!(
      style.scene.camera.preset,
      a::PresetCameraValues::OrthographicFront
    );
    assert_eq!(style.scene.light_rig.rig, a::LightRigValues::ThreePoints);
    assert_eq!(
      style.scene.light_rig.direction,
      a::LightRigDirectionValues::Top
    );
  }

  #[test]
  fn body_scene_and_run_shape_resolve_to_one_solid_without_synthetic_z() {
    let body = super::Static3dStyle {
      scene: Box::new(scene(a::PresetCameraValues::PerspectiveLeft)),
      shape: Box::new(a::Shape3DType::default()),
      extrusion_color: None,
      contour_color: None,
    };
    let run_shape = Box::new(a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(38_100)),
        height: Some(CoordinateValue::Emu(38_100)),
        ..a::BevelTop::default()
      }),
      ..a::Shape3DType::default()
    });
    let run = Static3dStyleParts {
      shape: Some(run_shape.clone()),
      ..Static3dStyleParts::default()
    };

    let combined = resolve_static_3d_style(Some(&body), Some(&run)).unwrap();

    assert_eq!(combined.shape, run_shape);
    assert_eq!(combined.shape.z, None);
    assert_eq!(
      combined.scene.camera.preset,
      a::PresetCameraValues::PerspectiveLeft
    );
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
  fn projected_bitmap_guard_expands_the_complete_perspective_input() {
    let scene = scene(a::PresetCameraValues::PerspectiveLeft);
    let projection = camera_projection(&scene, 0.0);
    let shape = a::Shape3DType::default();
    let unguarded = projected_output_bounds(projection, &shape, 210.0, 116.35);
    let guarded = projected_region_output_bounds(
      projection,
      &shape,
      210.0,
      116.35,
      super::Static3dOutputBounds {
        left_pt: -7.5,
        top_pt: -7.5,
        right_pt: 217.5,
        bottom_pt: 123.85,
      },
    );
    assert!(guarded.left_pt < unguarded.left_pt);
    assert!(guarded.top_pt < unguarded.top_pt);
    assert!(guarded.right_pt > unguarded.right_pt);
    assert!(guarded.bottom_pt > unguarded.bottom_pt);
  }

  #[test]
  fn perspective_left_projects_the_logical_left_anchor_before_effect_alignment() {
    let scene = scene(a::PresetCameraValues::PerspectiveLeft);
    let projection = camera_projection(&scene, 0.0);
    let projected = projected_front_region_output_bounds(
      projection,
      &a::Shape3DType::default(),
      210.0,
      116.35,
      super::Static3dOutputBounds {
        left_pt: 15.0,
        top_pt: 20.0,
        right_pt: 195.0,
        bottom_pt: 80.0,
      },
    );

    // The left edge recedes under perspectiveLeft and moves toward the model
    // center. A 70%-wide, left-aligned shadow must scale around this projected
    // x coordinate, not the authored 15pt coordinate.
    assert!(projected.left_pt > 15.0);
    assert!(projected.right_pt > projected.left_pt);
    assert!(projected.bottom_pt > projected.top_pt);
  }

  #[test]
  fn contour_width_is_centered_on_the_solid_boundary() {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let shape = a::Shape3DType {
      contour_width: Some(CoordinateValue::Emu(12_700)),
      ..a::Shape3DType::default()
    };
    let bounds = projected_output_bounds(camera_projection(&scene, 0.0), &shape, 10.0, 6.0);

    assert!((bounds.left_pt + 0.5).abs() < f32::EPSILON);
    assert!((bounds.top_pt + 0.5).abs() < f32::EPSILON);
    assert!((bounds.right_pt - 10.5).abs() < f32::EPSILON);
    assert!((bounds.bottom_pt - 6.5).abs() < f32::EPSILON);
  }

  #[test]
  fn text_geometry_preserves_counter_winding_without_absorbing_the_text_outline() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let commands = vec![
      PathCommand::MoveTo(point(1.0, 1.0)),
      PathCommand::LineTo(point(9.0, 1.0)),
      PathCommand::LineTo(point(9.0, 9.0)),
      PathCommand::LineTo(point(1.0, 9.0)),
      PathCommand::Close,
      PathCommand::MoveTo(point(3.0, 3.0)),
      PathCommand::LineTo(point(3.0, 7.0)),
      PathCommand::LineTo(point(7.0, 7.0)),
      PathCommand::LineTo(point(7.0, 3.0)),
      PathCommand::Close,
    ];
    let geometry = Static3dTextGeometry::from_page_path(
      &commands,
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(12.0),
          height: Pt(12.0),
        },
      },
      1.0,
    )
    .expect("text geometry");

    assert!(geometry.solid_on_right);
    assert_eq!(geometry.contours.len(), 2);
    let path = text_geometry_path(&geometry, |point| point).expect("geometry path");
    let mask = text_geometry_mask(12, 12, &path).expect("geometry mask");
    assert_eq!(mask.pixel(1, 1).map(|pixel| pixel.alpha()), Some(255));
    assert_eq!(mask.pixel(5, 5).map(|pixel| pixel.alpha()), Some(0));
    let outer_left = geometry.contours[0]
      .points
      .iter()
      .map(|point| point.0)
      .fold(f32::INFINITY, f32::min);
    let hole_left = geometry.contours[1]
      .points
      .iter()
      .map(|point| point.0)
      .fold(f32::INFINITY, f32::min);
    assert!((outer_left - 1.0).abs() < 0.001);
    assert!((hole_left - 3.0).abs() < 0.001);
  }

  #[test]
  fn projected_effect_front_excludes_extruded_side_faces() {
    let scene = scene(a::PresetCameraValues::PerspectiveLeft);
    let projection = camera_projection(&scene, 0.0);
    let shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(127_000)),
      ..a::Shape3DType::default()
    };
    let mut source = RgbaImage::new(120, 80);
    for y in 24..56 {
      for x in 30..90 {
        source.put_pixel(x, y, Rgba([160, 180, 200, 255]));
      }
    }
    let surface = super::Static3dSurface {
      left_px: 0.0,
      top_px: 0.0,
      width_px: 120.0,
      height_px: 80.0,
    };
    let front = project_static_3d_front_face(&source, projection, &shape, 1.0, Some(surface));
    let mut solid = source;
    apply_static_3d(
      &mut solid,
      &scene,
      projection,
      &shape,
      Static3dRenderOptions {
        extrusion_color: Some(Static3dColor {
          color: RgbColor {
            r: 80,
            g: 90,
            b: 100,
          },
          alpha: 255,
        }),
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: Some(surface),
      },
    );

    let front_pixels = front.pixels().filter(|pixel| pixel[3] != 0).count();
    let solid_pixels = solid.pixels().filter(|pixel| pixel[3] != 0).count();
    assert!(solid_pixels > front_pixels);
  }

  #[test]
  fn camera_padding_includes_top_bevel_height() {
    let scene = scene(a::PresetCameraValues::ObliqueLeft);
    let projection = camera_projection(&scene, 0.0);
    let flat = output_padding(projection, &a::Shape3DType::default(), 64.0, 32.0);
    let shape = a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(38_100)),
        height: Some(CoordinateValue::Emu(38_100)),
        preset: Some(a::BevelPresetValues::Circle),
      }),
      ..a::Shape3DType::default()
    };
    let beveled = output_padding(projection, &shape, 64.0, 32.0);
    assert!(
      beveled.left_pt + beveled.right_pt > flat.left_pt + flat.right_pt,
      "camera projection must reserve the raised terminal plane"
    );
  }

  #[test]
  fn unspecified_bevel_preset_uses_the_ecma_circle_default() {
    assert_eq!(bevel_terminal_inset(None), 1.0);
  }

  #[test]
  fn circle_bevel_profile_follows_office_outer_to_inner_coordinates() {
    let (outer_height, outer_dx, outer_dy) = circle_bevel_profile(0.0);
    let (middle_height, _, _) = circle_bevel_profile(0.5);
    let (inner_height, inner_dx, inner_dy) = circle_bevel_profile(1.0);
    assert!(outer_height.abs() < 0.001);
    assert!(outer_dx.abs() < 0.001);
    assert!((outer_dy - 1.668_81).abs() < 0.001);
    assert!(middle_height > 0.1 && middle_height < 0.2);
    assert!((inner_height - 1.0).abs() < 0.001);
    assert!((inner_dx - 1.668_81).abs() < 0.001);
    assert!(inner_dy.abs() < 0.001);
  }

  #[test]
  fn circle_bevel_outer_edge_receives_full_material_lighting() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::Harsh,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let source = RgbaImage::from_pixel(9, 9, Rgba([200, 200, 200, 255]));
    let mut bevel = RgbaImage::new(9, 9);
    let projection = camera_projection(&scene, 0.0);

    composite_bevel(
      &mut bevel,
      &source,
      BevelOptions {
        width: 4,
        height: 4.0,
        preset: Some(a::BevelPresetValues::Circle),
        scene: &scene,
        projection,
        model_surface: Static3dSurface {
          left_px: 0.0,
          top_px: 0.0,
          width_px: 9.0,
          height_px: 9.0,
        },
        pixels_per_point: 1.0,
        surface_z: 0.0,
        material: Some(a::PresetMaterialTypeValues::Matte),
        back_face: false,
      },
    );

    // The profile begins at bevel-space x=0, but that is still a complete
    // +z-facing surface. Harsh/top lights it below the original gray; treating
    // x as opacity would incorrectly leave this boundary pixel at 200.
    let outer_edge = bevel.get_pixel(0, 4);
    assert_eq!(outer_edge[3], 255);
    assert!(outer_edge[0] < 190, "outer edge was {outer_edge:?}");
  }

  #[test]
  fn text_outline_colors_the_3d_surface_without_expanding_its_geometry() {
    let mut surface = RgbaImage::from_pixel(3, 1, Rgba([180, 60, 20, 153]));
    surface.put_pixel(1, 0, Rgba([160, 90, 40, 255]));
    let mut fill = RgbaImage::new(3, 1);
    fill.put_pixel(1, 0, Rgba([220, 220, 120, 255]));

    mask_static_3d_text_surface_paint(&mut surface, &fill);

    assert_eq!(surface.get_pixel(0, 0), &Rgba([0, 0, 0, 0]));
    assert_eq!(surface.get_pixel(1, 0), &Rgba([160, 90, 40, 255]));
    assert_eq!(surface.get_pixel(2, 0), &Rgba([0, 0, 0, 0]));
  }

  #[test]
  fn relaxed_inset_bevel_retains_the_folded_office_surface() {
    let preset = Some(a::BevelPresetValues::RelaxedInset);
    let crest = bevel_profile_sample(preset, 0, 1.0);
    let fold = bevel_profile_sample(preset, 1, 0.0);
    let terminal = bevel_profile_sample(preset, 1, 1.0);

    assert!((crest.height - 0.507_899).abs() < 0.000_001);
    assert!((crest.inset - 1.0).abs() < 0.000_001);
    assert_eq!(crest.height, fold.height);
    assert_eq!(crest.inset, fold.inset);
    assert!(fold.height_tangent > 0.0);
    assert!(fold.inset_tangent < 0.0);
    assert!((terminal.height - 1.0).abs() < 0.000_001);
    assert!((terminal.inset - 0.64).abs() < 0.000_001);
    assert!((bevel_terminal_inset(preset) - 0.64).abs() < f32::EPSILON);
  }

  #[test]
  fn text_curve_normals_smooth_shallow_segments_but_keep_hard_corners() {
    let normals =
      text_3d_contour_edge_normals(&[(0.0, 0.0), (10.0, 0.0), (20.0, 1.0), (20.0, 10.0)], true);

    assert_eq!(normals[0].1, normals[1].0);
    assert_ne!(normals[1].1, normals[2].0);
  }

  #[test]
  fn bevel_distance_field_follows_diagonal_glyph_edges() {
    let mut source = RgbaImage::new(5, 5);
    for y in 0_i32..5 {
      for x in 0_i32..5 {
        if (x - 2).abs() + (y - 2).abs() <= 2 {
          source.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255]));
        }
      }
    }

    let distances = bevel_distance_field(&source, 4);
    let center = distances[2 * source.width() as usize + 2];

    assert!((center - 5.0_f32.sqrt()).abs() < 0.001);
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
      Static3dRenderOptions {
        extrusion_color: Some(Static3dColor {
          color: RgbColor { r: 255, g: 0, b: 0 },
          alpha: 255,
        }),
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );
    // The front face is exactly edge-on at +90 degrees, while the extrusion
    // remains a visible side plane.
    assert!(!image.pixels().any(|pixel| pixel[1] > 0));
    assert!(image.pixels().any(|pixel| pixel[0] > 0 && pixel[1] == 0));
  }

  #[test]
  fn extrusion_side_preserves_rasterized_edge_coverage() {
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
    image.put_pixel(4, 1, Rgba([0, 255, 0, 64]));
    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      Static3dRenderOptions {
        extrusion_color: Some(Static3dColor {
          color: RgbColor { r: 255, g: 0, b: 0 },
          alpha: 255,
        }),
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );

    let maximum_alpha = image.pixels().map(|pixel| pixel[3]).max().unwrap_or(0);
    assert!(maximum_alpha > 0);
    assert!(maximum_alpha < 255);
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
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
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
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );
    assert_eq!(image.get_pixel(2, 2)[3], 0);
    assert_eq!(image.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
  }

  #[test]
  fn top_bevel_survives_front_face_compositing() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
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
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );

    let planar = *image.get_pixel(3, 3);
    assert_ne!(planar, Rgba([200, 40, 40, 255]));
    assert!(image.pixels().any(|pixel| *pixel != planar));
  }

  #[test]
  fn top_bevel_adds_material_specular_light_to_black_edges() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let shape = a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(25_400)),
        height: Some(CoordinateValue::Emu(25_400)),
        preset: Some(a::BevelPresetValues::Circle),
      }),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::from_pixel(7, 7, Rgba([0, 0, 0, 255]));

    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );

    let planar = *image.get_pixel(3, 3);
    assert!(planar[0] > 0);
    assert!(
      image
        .pixels()
        .any(|pixel| pixel[3] > 0 && pixel[0] != planar[0])
    );
  }

  #[test]
  fn planar_front_face_receives_material_lighting_without_extrusion() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let shape = a::Shape3DType {
      preset_material: Some(a::PresetMaterialTypeValues::WarmMatte),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::from_pixel(5, 5, Rgba([0, 0, 0, 255]));

    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: None,
      },
    );

    let center = image.get_pixel(2, 2);
    assert_eq!(center[3], 255);
    assert!(center[0] > 0 && center[1] > 0 && center[2] > 0);
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
  fn three_point_front_plane_uses_the_transformed_office_direction() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let shade = light_rig_surface_shade(&scene, [0.0, 0.0, 1.0]);

    // Only the third ThreePoint light has positive z after MS-OI29500's
    // fixed basis conversion, so a front plane receives its 0.7769 term.
    for channel in shade {
      assert!((channel - 0.776_9).abs() < 0.000_1);
    }
  }

  #[test]
  fn material_specular_color_does_not_amplify_the_diffuse_term() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let normal = [0.0, 0.0, 1.0];

    // MS-OI29500 gives both presets Shape diffuse color, while only
    // warmMatte has a non-black specular color. Their diffuse terms therefore
    // remain identical even though the complete lit result can differ.
    assert_eq!(
      material_diffuse_shade(&scene, normal, Some(a::PresetMaterialTypeValues::WarmMatte),),
      material_diffuse_shade(&scene, normal, Some(a::PresetMaterialTypeValues::Matte)),
    );
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
