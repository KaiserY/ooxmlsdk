use std::collections::HashMap;

use image::{Rgba, RgbaImage};
use kurbo::{BezPath, PathEl, Shape as KurboShape, flatten};

mod backdrop;
#[cfg(test)]
mod contour;
mod contour_mesh;
mod material_texture;
mod physical_curve;
pub(crate) use backdrop::BackdropTexturePlan;
pub(crate) use material_texture::{
  TextMaterialTexture, TextMaterialTexturePlan, TextSurfaceRealizationPlan,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use smallvec::SmallVec;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use super::{
  DisplayItem, PathCommand, Point, Rect,
  drawingml_curve_stroke::widen_centered_miter_butt,
  drawingml_direct_inset::{DirectInsetCell, direct_inset_cells},
  drawingml_geometry,
  drawingml_mil_raster::rasterize_nonzero_wpf_8x8_path,
};
use crate::model::RgbColor;

const EMUS_PER_POINT: f32 = 12_700.0;
const TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX: f64 = 0.1;
// Physical text meshes retain floating-point source coordinates through projection.
// Office's indexed front/back contours at 0.5/1/2 pt reject a translated
// 1/16-source-pixel lattice; its position shaders apply the scene matrix directly.
// Direct2D's extrusion sample snaps around its Tessellate call to absorb jitter.
// That workaround is not a contract for this mesh builder. Raster-device
// quantization remains a separate, post-projection operation.
const WORD_TEXT_STATIC_3D_EFFECTIVE_COORDINATE_MIN_EMU: i64 = 7;
const WORD_TEXT_ANTIALIASED_CONTOUR_PHASE_PX: f32 = 0.5;
const WORD_CONTOUR_HEIGHT_OVER_RADIUS: f32 = 1.1;

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
  /// Original Word run text-plane Z before adapting `w14:props3d` to the
  /// shared mesh builder's front-face coordinate. Native DrawingML styles do
  /// not need that adaptation and leave this unset.
  pub(crate) wordprocessing_effect_plane_z_pt: Option<f32>,
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
    .or_else(|| body.map(|body| body.shape.clone()));
  let scene = run
    .and_then(|run| run.scene.clone())
    .or_else(|| body.map(|body| body.scene.clone()));

  // [MS-DOCX] §2.6.3.23 gives absent props values zero geometry and a
  // warm-matte material. Those defaults describe a shape after 3-D has been
  // activated; they do not themselves activate Word's fixed-output 3-D
  // path. Controlled Word print exports independently hold every other
  // property constant and establish the trigger boundary: a scene, extrusion
  // height >= 7 EMU, or top/bottom bevel height >= 7 EMU. An empty props3d,
  // material, contour, color, coordinates in 0..=6 EMU, and bevel width
  // without height all remain bit-for-bit equivalent to ordinary 2-D text.
  let shape_activates_3d = shape
    .as_deref()
    .is_some_and(wordprocessing_text_shape_activates_static_3d);
  if scene.is_none() && !shape_activates_3d {
    return None;
  }

  // Scene-only Word text uses the neutral shape surface. Conversely, active
  // props-only text uses Word's neutral orthographic/three-point scene.
  let mut shape = shape.unwrap_or_default();
  let wordprocessing_effect_plane_z_pt = run_supplies_shape.then(|| {
    shape
      .z
      .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
      .unwrap_or(0.0)
  });
  if run_supplies_shape {
    anchor_wordprocessing_text_shape_at_extrusion_base(&mut shape);
  }
  let scene = scene.unwrap_or_else(default_text_3d_scene);
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
    wordprocessing_effect_plane_z_pt,
  })
}

/// Converts Word 2010 `w14:props3d` depth coordinates to the front-face
/// coordinate consumed by the shared DrawingML mesh builder.
///
/// The shared builder starts at `shape.z` and walks backwards through the top
/// bevel and extrusion. Word instead anchors its source text plane at the
/// extrusion base. MS-OI29500 20.1.5.3/4 fixes the bottom bevel at that base
/// and the top bevel above the extrusion. An exact-config Office 2x2 control
/// under an isometric camera independently measures positive-Z contributions
/// from both `extrusionH` and `bevelT/@h`. Move only run-level W14 properties;
/// body-level DrawingML `a:sp3d` retains its native coordinate contract.
fn anchor_wordprocessing_text_shape_at_extrusion_base(shape: &mut a::Shape3DType) {
  let front_offset_emu = shape
    .extrusion_height
    .map_or(0, |height| height.to_emu())
    .saturating_add(
      shape
        .bevel_top
        .as_ref()
        .and_then(|bevel| bevel.height)
        .map_or(0, |height| height.to_emu()),
    );
  if front_offset_emu == 0 {
    return;
  }
  let base_z_emu = shape.z.map_or(0, |z| z.to_emu());
  shape.z = Some(ooxmlsdk::units::CoordinateValue::Emu(
    base_z_emu.saturating_add(front_offset_emu),
  ));
}

fn wordprocessing_text_shape_activates_static_3d(shape: &a::Shape3DType) -> bool {
  coordinate_reaches_word_text_static_3d_boundary(shape.extrusion_height)
    || shape
      .bevel_top
      .as_ref()
      .is_some_and(|bevel| coordinate_reaches_word_text_static_3d_boundary(bevel.height))
    || shape
      .bevel_bottom
      .as_ref()
      .is_some_and(|bevel| coordinate_reaches_word_text_static_3d_boundary(bevel.height))
}

/// Whether Word transfers the painted outline from the scene-only surface to
/// the raw glyph-shaped physical solid.
///
/// Exact-config Word print controls for contour width, extrusion height, and
/// both bevel heights establish the same boundary: 0..=6 EMU retains the
/// painted scene-only surface, while >=7 EMU uses physical glyph geometry.
/// Contour participates in ownership once a scene/other property has activated
/// static 3-D, but it never activates the route by itself.
pub(crate) fn wordprocessing_text_shape_has_effective_3d_geometry(shape: &a::Shape3DType) -> bool {
  coordinate_reaches_word_text_static_3d_boundary(shape.contour_width)
    || wordprocessing_text_shape_activates_static_3d(shape)
}

fn coordinate_reaches_word_text_static_3d_boundary(
  value: Option<ooxmlsdk::units::CoordinateValue>,
) -> bool {
  value.is_some_and(|value| value.to_emu() >= WORD_TEXT_STATIC_3D_EFFECTIVE_COORDINATE_MIN_EMU)
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
  /// The unflattened device-space glyph outline used for continuous A8
  /// source-plane coverage. The lighting mesh still owns its independently
  /// flattened contours and curve-edge metadata.
  source_coverage_path: BezPath,
  /// Perspective profile precision belongs to the projected solid, not the
  /// source bitmap density. Both geometry and material lookup consume it.
  bevel_profile_tolerance_px: Option<f32>,
  solid_on_right: bool,
  page_plane_scale_x: f32,
  page_plane_scale_y: f32,
  page_plane_translate_x: f32,
  page_plane_translate_y: f32,
}

/// The two coordinate realizations consumed by Word's static-3-D text path.
///
/// The physical mesh uses Direct3D 9 integer-pixel centers. Independently
/// rasterized planar fill, outline and outline-coverage images retain
/// page-space pixel centers. The combined GDI+ texture remains a separate
/// realization rather than sharing those planar texture coordinates.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Static3dTextGeometryPaths {
  physical: Static3dTextGeometry,
  page: Static3dTextGeometry,
  surface_material_texture: Option<TextMaterialTexture>,
  /// Constant authored paint opacity when the text source has one uniform
  /// solid fill and no independently painted outline. This is paint state,
  /// not glyph coverage: retaining it here prevents the 3-D stage from
  /// dividing alpha produced by one rasterizer by coverage from another.
  uniform_paint_opacity: Option<f32>,
  /// Position-independent opacity of the independently rasterized fill and
  /// outline materials. These remain separate when both paints are present;
  /// recovering either value from its antialiased bitmap confuses coverage
  /// from one rasterizer with paint state from another.
  front_fill_uniform_paint_opacity: Option<f32>,
  front_outline_uniform_paint_opacity: Option<f32>,
  /// Inward edge of a visible centered character outline, measured from the
  /// raw glyph boundary on the physical text surface. Word clips the outward
  /// half of the line to the glyph solid, but keeps the inward half as a
  /// distinct W14 bevel material region.
  front_outline_material_inset_px: Option<f32>,
  /// Whether the W14 text fill carries an explicitly authored nonzero
  /// transparency transform. Keep this independently from its quantized
  /// material bitmap so the 3-D texture route can distinguish zero from one.
  front_fill_has_authored_transparency: bool,
  /// Whether the W14 text outline carries an explicitly authored nonzero
  /// transparency transform. Word selects an alpha-aware bevel-material
  /// raster route from this source fact even when the resolved byte alpha is
  /// still 255, so it cannot be reconstructed from the material bitmap.
  front_outline_has_authored_transparency: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Static3dTextContour {
  points: Vec<(f32, f32)>,
  /// Authored path elements for this contour, before curve flattening.
  /// Material boundaries must be located on the source curve rather than
  /// inferred from the number of tessellation vertices. The range addresses
  /// `Static3dTextGeometry::source_coverage_path` without copying its segments.
  source_path_range: std::ops::Range<usize>,
  /// Whether the edge ending at each point came from a source Bézier.
  /// Keeping this through flattening lets the lighting mesh share normals
  /// inside curves without smoothing an authored line/curve corner.
  incoming_curve_edges: Vec<bool>,
  /// Whether this point is a genuine crease in the source outline.
  ///
  /// Curve flattening inserts tessellation vertices, and TrueType quadratic
  /// outlines may be raised into several tangent-continuous cubic segments.
  /// Neither representation boundary is an outline edge of the extrusion.
  /// Preserve only tangent-discontinuous source joins for longitudinal
  /// contour bands.
  longitudinal_contour_joins: Vec<bool>,
  bounds: (f32, f32, f32, f32),
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
    let translate_x = -raster_bounds.origin.x.0 * pixels_per_point;
    let translate_y = -raster_bounds.origin.y.0 * pixels_per_point;
    Self::from_page_path_with_transform(
      commands,
      pixels_per_point,
      pixels_per_point,
      translate_x,
      translate_y,
      |point| {
        kurbo::Point::new(
          f64::from(point.x.0 * pixels_per_point + translate_x),
          f64::from(point.y.0 * pixels_per_point + translate_y),
        )
      },
    )
  }

  /// Maps a page-space text outline onto the static-3-D working surface.
  ///
  /// The physical outline is expressed in Direct3D 9 integer-pixel-center
  /// coordinates, while the software destination addresses each image cell
  /// over `[n, n + 1]`. Convert only this physical path by half a pixel; the
  /// parallel page-space path remains unshifted for paint/effect owners.
  /// Exact Office `l`/`x` controls at both the 7-EMU activation boundary and a
  /// full-point extrusion reject removing this conversion on every positive
  /// control, including the bevel-, contour-, and effect-free counterexamples.
  pub(crate) fn from_page_path_for_direct3d9(
    commands: &[PathCommand],
    raster_bounds: Rect,
    pixels_per_point: f32,
  ) -> Option<Self> {
    if commands.is_empty() || !pixels_per_point.is_finite() || pixels_per_point <= f32::EPSILON {
      return None;
    }
    const HALF_PIXEL_TO_INTEGER_CENTER: f32 = -0.5;
    let translate_x = -raster_bounds.origin.x.0 * pixels_per_point + HALF_PIXEL_TO_INTEGER_CENTER;
    let translate_y = -raster_bounds.origin.y.0 * pixels_per_point + HALF_PIXEL_TO_INTEGER_CENTER;
    Self::from_page_path_with_transform(
      commands,
      pixels_per_point,
      pixels_per_point,
      translate_x,
      translate_y,
      |point| {
        kurbo::Point::new(
          f64::from(point.x.0 * pixels_per_point + translate_x),
          f64::from(point.y.0 * pixels_per_point + translate_y),
        )
      },
    )
  }

  pub(crate) fn from_page_path_with_mapping(
    commands: &[PathCommand],
    scale_x: f32,
    scale_y: f32,
    translate_x: f32,
    translate_y: f32,
  ) -> Option<Self> {
    if commands.is_empty()
      || !scale_x.is_finite()
      || !scale_y.is_finite()
      || !translate_x.is_finite()
      || !translate_y.is_finite()
      || scale_x <= f32::EPSILON
      || scale_y <= f32::EPSILON
    {
      return None;
    }
    Self::from_page_path_with_transform(
      commands,
      scale_x,
      scale_y,
      translate_x,
      translate_y,
      |point| {
        kurbo::Point::new(
          f64::from(point.x.0 * scale_x + translate_x),
          f64::from(point.y.0 * scale_y + translate_y),
        )
      },
    )
  }

  fn from_page_path_with_transform(
    commands: &[PathCommand],
    page_plane_scale_x: f32,
    page_plane_scale_y: f32,
    page_plane_translate_x: f32,
    page_plane_translate_y: f32,
    transform: impl Fn(Point) -> kurbo::Point,
  ) -> Option<Self> {
    let elements = drawingml_geometry::mapped_path_elements(commands, transform);
    Self::from_mapped_path_elements(
      elements,
      page_plane_scale_x,
      page_plane_scale_y,
      page_plane_translate_x,
      page_plane_translate_y,
      None,
    )
  }

  fn from_mapped_path_elements(
    elements: Vec<PathEl>,
    page_plane_scale_x: f32,
    page_plane_scale_y: f32,
    page_plane_translate_x: f32,
    page_plane_translate_y: f32,
    physical_flatness: Option<(f64, f64)>,
  ) -> Option<Self> {
    let flatten_curve = |path: &[PathEl], emit: &mut dyn FnMut(PathEl)| {
      if let Some((tolerance, pixels_per_inch)) = physical_flatness {
        physical_curve::flatten(path.iter().copied(), tolerance, pixels_per_inch, emit);
      } else {
        flatten(
          path.iter().copied(),
          TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX,
          emit,
        );
      }
    };
    let source_coverage_path = BezPath::from_vec(elements.clone());
    let mut contours = Vec::new();
    let mut points = Vec::new();
    let mut incoming_curve_edges = Vec::new();
    let mut longitudinal_contour_joins = Vec::new();
    let mut first_source_start_tangent = None;
    let mut previous_source_end_tangent = None;
    let mut source_path_start = 0;
    for (element_index, element) in elements.into_iter().enumerate() {
      match element {
        PathEl::MoveTo(point) => {
          finish_text_3d_contour(
            &mut contours,
            &mut points,
            &mut incoming_curve_edges,
            &mut longitudinal_contour_joins,
            &mut first_source_start_tangent,
            &mut previous_source_end_tangent,
            source_path_start..element_index,
          );
          source_path_start = element_index;
          points.push((point.x as f32, point.y as f32));
          incoming_curve_edges.push(false);
          longitudinal_contour_joins.push(false);
        }
        PathEl::LineTo(point) => {
          let Some(&(current_x, current_y)) = points.last() else {
            continue;
          };
          let current = kurbo::Point::new(f64::from(current_x), f64::from(current_y));
          let tangent = text_3d_source_tangent(&[point - current]);
          begin_text_3d_source_segment(
            &mut longitudinal_contour_joins,
            &mut first_source_start_tangent,
            previous_source_end_tangent,
            tangent,
          );
          push_text_3d_contour_point(
            &mut points,
            &mut incoming_curve_edges,
            &mut longitudinal_contour_joins,
            point,
            false,
          );
          if tangent.is_some() {
            previous_source_end_tangent = tangent;
          }
        }
        PathEl::QuadTo(control, end) => {
          let Some(&(current_x, current_y)) = points.last() else {
            continue;
          };
          let current = kurbo::Point::new(f64::from(current_x), f64::from(current_y));
          let start_tangent = text_3d_source_tangent(&[control - current, end - current]);
          let end_tangent = text_3d_source_tangent(&[end - control, end - current]);
          begin_text_3d_source_segment(
            &mut longitudinal_contour_joins,
            &mut first_source_start_tangent,
            previous_source_end_tangent,
            start_tangent,
          );
          let first_flattened_point = points.len();
          flatten_curve(
            &[PathEl::MoveTo(current), PathEl::QuadTo(control, end)],
            &mut |flattened| {
              if let PathEl::LineTo(point) = flattened {
                push_text_3d_contour_point(
                  &mut points,
                  &mut incoming_curve_edges,
                  &mut longitudinal_contour_joins,
                  point,
                  true,
                );
              }
            },
          );
          if points.len() > first_flattened_point && end_tangent.is_some() {
            previous_source_end_tangent = end_tangent;
          }
        }
        PathEl::CurveTo(control1, control2, end) => {
          let Some(&(current_x, current_y)) = points.last() else {
            continue;
          };
          let current = kurbo::Point::new(f64::from(current_x), f64::from(current_y));
          let start_tangent =
            text_3d_source_tangent(&[control1 - current, control2 - current, end - current]);
          let end_tangent =
            text_3d_source_tangent(&[end - control2, end - control1, end - current]);
          begin_text_3d_source_segment(
            &mut longitudinal_contour_joins,
            &mut first_source_start_tangent,
            previous_source_end_tangent,
            start_tangent,
          );
          let first_flattened_point = points.len();
          flatten_curve(
            &[
              PathEl::MoveTo(current),
              PathEl::CurveTo(control1, control2, end),
            ],
            &mut |flattened| {
              if let PathEl::LineTo(point) = flattened {
                push_text_3d_contour_point(
                  &mut points,
                  &mut incoming_curve_edges,
                  &mut longitudinal_contour_joins,
                  point,
                  true,
                );
              }
            },
          );
          if points.len() > first_flattened_point && end_tangent.is_some() {
            previous_source_end_tangent = end_tangent;
          }
        }
        PathEl::ClosePath => {
          finish_text_3d_contour(
            &mut contours,
            &mut points,
            &mut incoming_curve_edges,
            &mut longitudinal_contour_joins,
            &mut first_source_start_tangent,
            &mut previous_source_end_tangent,
            source_path_start..element_index + 1,
          );
          source_path_start = element_index + 1;
        }
      }
    }
    finish_text_3d_contour(
      &mut contours,
      &mut points,
      &mut incoming_curve_edges,
      &mut longitudinal_contour_joins,
      &mut first_source_start_tangent,
      &mut previous_source_end_tangent,
      source_path_start..source_coverage_path.elements().len(),
    );
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
      source_coverage_path,
      bevel_profile_tolerance_px: None,
      solid_on_right,
      page_plane_scale_x,
      page_plane_scale_y,
      page_plane_translate_x,
      page_plane_translate_y,
    })
  }

  fn map_point_to(&self, target: &Self, point: (f32, f32)) -> (f32, f32) {
    let page_x = (point.0 - self.page_plane_translate_x) / self.page_plane_scale_x;
    let page_y = (point.1 - self.page_plane_translate_y) / self.page_plane_scale_y;
    (
      page_x * target.page_plane_scale_x + target.page_plane_translate_x,
      page_y * target.page_plane_scale_y + target.page_plane_translate_y,
    )
  }
}

impl Static3dTextGeometryPaths {
  /// Rebuild only the physical mesh at the target's projected density. Keep
  /// unflattened coverage and independently painted page geometry unchanged.
  pub(crate) fn with_physical_realization(
    mut self,
    realization: Option<TextSurfaceRealizationPlan>,
    pixels_per_point: f32,
  ) -> Self {
    if let Some(plan) = realization {
      let physical = &self.physical;
      if let Some(mut realized) = Static3dTextGeometry::from_mapped_path_elements(
        physical.source_coverage_path.elements().to_vec(),
        physical.page_plane_scale_x,
        physical.page_plane_scale_y,
        physical.page_plane_translate_x,
        physical.page_plane_translate_y,
        Some((plan.curve_tolerance_px, f64::from(pixels_per_point) * 72.0)),
      ) {
        realized.bevel_profile_tolerance_px =
          Some(plan.bevel_profile_tolerance_px(pixels_per_point));
        self.physical = realized;
      }
    }
    self
  }

  pub(crate) fn from_page_path_for_direct3d9(
    commands: &[PathCommand],
    raster_bounds: Rect,
    pixels_per_point: f32,
  ) -> Option<Self> {
    Some(Self {
      physical: Static3dTextGeometry::from_page_path_for_direct3d9(
        commands,
        raster_bounds,
        pixels_per_point,
      )?,
      page: Static3dTextGeometry::from_page_path(commands, raster_bounds, pixels_per_point)?,
      surface_material_texture: None,
      uniform_paint_opacity: None,
      front_fill_uniform_paint_opacity: None,
      front_outline_uniform_paint_opacity: None,
      front_outline_material_inset_px: None,
      front_fill_has_authored_transparency: false,
      front_outline_has_authored_transparency: false,
    })
  }

  pub(crate) fn with_uniform_paint_opacity(mut self, opacity: Option<f32>) -> Self {
    self.uniform_paint_opacity = opacity.map(|value| value.clamp(0.0, 1.0));
    self
  }

  pub(crate) fn with_surface_material_texture(
    mut self,
    texture: Option<TextMaterialTexture>,
  ) -> Self {
    self.surface_material_texture = texture;
    self
  }

  pub(crate) fn with_front_material_opacities(
    mut self,
    fill: Option<f32>,
    outline: Option<f32>,
  ) -> Self {
    self.front_fill_uniform_paint_opacity = fill.map(|value| value.clamp(0.0, 1.0));
    self.front_outline_uniform_paint_opacity = outline.map(|value| value.clamp(0.0, 1.0));
    self
  }

  pub(crate) fn with_front_outline_material_inset_px(mut self, inset_px: Option<f32>) -> Self {
    self.front_outline_material_inset_px =
      inset_px.filter(|value| value.is_finite() && *value > 0.0);
    self
  }

  pub(crate) fn with_front_fill_authored_transparency(mut self, authored: bool) -> Self {
    self.front_fill_has_authored_transparency = authored;
    self
  }

  pub(crate) fn with_front_outline_authored_transparency(mut self, authored: bool) -> Self {
    self.front_outline_has_authored_transparency = authored;
    self
  }

  #[cfg(test)]
  fn aligned(geometry: Static3dTextGeometry) -> Self {
    Self {
      physical: geometry.clone(),
      page: geometry,
      surface_material_texture: None,
      uniform_paint_opacity: None,
      front_fill_uniform_paint_opacity: None,
      front_outline_uniform_paint_opacity: None,
      front_outline_material_inset_px: None,
      front_fill_has_authored_transparency: false,
      front_outline_has_authored_transparency: false,
    }
  }
}

fn push_text_3d_contour_point(
  points: &mut Vec<(f32, f32)>,
  incoming_curve_edges: &mut Vec<bool>,
  longitudinal_contour_joins: &mut Vec<bool>,
  point: kurbo::Point,
  incoming_curve: bool,
) {
  let point = (point.x as f32, point.y as f32);
  if points
    .last()
    .is_none_or(|previous| (previous.0 - point.0).hypot(previous.1 - point.1) > 1.0e-4)
  {
    points.push(point);
    incoming_curve_edges.push(incoming_curve);
    longitudinal_contour_joins.push(false);
  }
}

type Text3dSourceTangent = (f64, f64);

/// Source path builders may raise one continuous TrueType quadratic span into
/// multiple cubics. The mapped coordinates pass through `f32`, so use a small
/// normalized cross-product tolerance to recognize that numerical G1
/// continuity without erasing real, shallow corners.
const TEXT_3D_SOURCE_G1_SINE_EPSILON: f64 = 1.0e-3;

fn text_3d_source_tangent(vectors: &[kurbo::Vec2]) -> Option<Text3dSourceTangent> {
  vectors.iter().find_map(|vector| {
    let length = vector.hypot();
    (length > 1.0e-9 && length.is_finite()).then_some((vector.x, vector.y))
  })
}

fn text_3d_source_tangents_form_contour_join(
  incoming: Text3dSourceTangent,
  outgoing: Text3dSourceTangent,
) -> bool {
  let incoming_length = incoming.0.hypot(incoming.1);
  let outgoing_length = outgoing.0.hypot(outgoing.1);
  let length_product = incoming_length * outgoing_length;
  if !length_product.is_finite() || length_product <= f64::EPSILON {
    return false;
  }
  let dot = incoming.0 * outgoing.0 + incoming.1 * outgoing.1;
  let sine = (incoming.0 * outgoing.1 - incoming.1 * outgoing.0).abs() / length_product;
  dot <= 0.0 || sine > TEXT_3D_SOURCE_G1_SINE_EPSILON
}

fn begin_text_3d_source_segment(
  longitudinal_contour_joins: &mut [bool],
  first_source_start_tangent: &mut Option<Text3dSourceTangent>,
  previous_source_end_tangent: Option<Text3dSourceTangent>,
  source_start_tangent: Option<Text3dSourceTangent>,
) {
  let Some(source_start_tangent) = source_start_tangent else {
    return;
  };
  if first_source_start_tangent.is_none() {
    *first_source_start_tangent = Some(source_start_tangent);
  }
  if let (Some(previous_source_end_tangent), Some(join)) = (
    previous_source_end_tangent,
    longitudinal_contour_joins.last_mut(),
  ) {
    *join =
      text_3d_source_tangents_form_contour_join(previous_source_end_tangent, source_start_tangent);
  }
}

fn finish_text_3d_contour(
  contours: &mut Vec<Static3dTextContour>,
  points: &mut Vec<(f32, f32)>,
  incoming_curve_edges: &mut Vec<bool>,
  longitudinal_contour_joins: &mut Vec<bool>,
  first_source_start_tangent: &mut Option<Text3dSourceTangent>,
  previous_source_end_tangent: &mut Option<Text3dSourceTangent>,
  source_path_range: std::ops::Range<usize>,
) {
  debug_assert_eq!(points.len(), incoming_curve_edges.len());
  debug_assert_eq!(points.len(), longitudinal_contour_joins.len());
  if points.len() >= 2 {
    let last = points.len() - 1;
    let closure = (
      f64::from(points[0].0 - points[last].0),
      f64::from(points[0].1 - points[last].1),
    );
    let explicitly_closed = closure.0.hypot(closure.1) <= 1.0e-4;
    if explicitly_closed {
      if let (Some(incoming), Some(outgoing)) =
        (*previous_source_end_tangent, *first_source_start_tangent)
      {
        longitudinal_contour_joins[last] =
          text_3d_source_tangents_form_contour_join(incoming, outgoing);
      }
      points.pop();
      incoming_curve_edges[0] = incoming_curve_edges.pop().unwrap_or(false);
      longitudinal_contour_joins[0] = longitudinal_contour_joins.pop().unwrap_or(false);
    } else {
      let closing_tangent = (closure.0, closure.1);
      if let Some(incoming) = *previous_source_end_tangent {
        longitudinal_contour_joins[last] =
          text_3d_source_tangents_form_contour_join(incoming, closing_tangent);
      }
      if let Some(outgoing) = *first_source_start_tangent {
        longitudinal_contour_joins[0] =
          text_3d_source_tangents_form_contour_join(closing_tangent, outgoing);
      }
      // The mesh closes every accepted open contour with one straight edge.
      incoming_curve_edges[0] = false;
    }
  }
  if points.len() >= 3 && signed_contour_area(points).abs() > 1.0e-3 {
    let bounds = text_3d_contour_bounds(points);
    contours.push(Static3dTextContour {
      points: std::mem::take(points),
      source_path_range,
      incoming_curve_edges: std::mem::take(incoming_curve_edges),
      longitudinal_contour_joins: std::mem::take(longitudinal_contour_joins),
      bounds,
    });
  } else {
    points.clear();
    incoming_curve_edges.clear();
    longitudinal_contour_joins.clear();
  }
  *first_source_start_tangent = None;
  *previous_source_end_tangent = None;
}

fn text_3d_contour_bounds(points: &[(f32, f32)]) -> (f32, f32, f32, f32) {
  points.iter().copied().fold(
    (
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ),
    |(left, top, right, bottom), point| {
      (
        left.min(point.0),
        top.min(point.1),
        right.max(point.0),
        bottom.max(point.1),
      )
    },
  )
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
      // Direct inset sweeps both adjacent edge lines until they intersect.
      // Keep that exact intersection here: imposing a conventional stroke
      // miter limit truncates legitimate merge/split faces at concave glyph
      // joins. Crossed portions are resolved against the complete glyph in
      // the collision pass below rather than by changing the swept geometry.
      let projection = (bisector.0 * next_normal.0 + bisector.1 * next_normal.1).abs();
      let miter_scale = projection.recip();
      (
        current.0 + bisector.0 * inward_distance * miter_scale,
        current.1 + bisector.1 * inward_distance * miter_scale,
      )
    };
    output.push(offset);
  }
  output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Text3dNormalJoinPolicy {
  /// Straight extrusion sides retain their face normal. Curve-owned sides
  /// interpolate shallow joins, including their boundary with a straight side.
  ExtrusionSurfaces,
  /// A swept bevel retains authored segment ownership so a line/curve corner
  /// does not accidentally become part of the curve's smooth normal field.
  SourceCurves,
}

fn text_3d_contour_edge_normals(
  points: &[(f32, f32)],
  incoming_curve_edges: &[bool],
  solid_on_right: bool,
  join_policy: Text3dNormalJoinPolicy,
) -> Vec<([f32; 2], [f32; 2])> {
  debug_assert_eq!(points.len(), incoming_curve_edges.len());
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
    let next = (index + 1) % edge_normals.len();
    let incoming = edge_normals[previous];
    let outgoing = edge_normals[index];
    if (join_policy == Text3dNormalJoinPolicy::ExtrusionSurfaces
      || incoming_curve_edges[index] && incoming_curve_edges[next])
      && incoming[0] * outgoing[0] + incoming[1] * outgoing[1] > 0.5
    {
      let mut average = [incoming[0] + outgoing[0], incoming[1] + outgoing[1]];
      let length = average[0].hypot(average[1]).max(f32::EPSILON);
      average[0] /= length;
      average[1] /= length;
      if join_policy != Text3dNormalJoinPolicy::ExtrusionSurfaces || incoming_curve_edges[index] {
        ends[previous] = average;
      }
      if join_policy != Text3dNormalJoinPolicy::ExtrusionSurfaces || incoming_curve_edges[next] {
        starts[index] = average;
      }
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

/// A fixed-output multisample target expressed in the coordinate space of the
/// already-built high-density text mesh.
///
/// The source rectangle is deliberately continuous. Preserve the source glyph
/// curves and paint coordinates, but realize their physical subdivision using
/// the final target's projected density. Office PDF and native-picture captures
/// retain identical source curves yet produce different physical polylines.
/// Shrinking a completed RGBA image would lose this per-sample depth ownership.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Static3dTextFinalGrid {
  pub(crate) width_px: u32,
  pub(crate) height_px: u32,
  pub(crate) source_left_px: f32,
  pub(crate) source_top_px: f32,
  pub(crate) source_width_px: f32,
  pub(crate) source_height_px: f32,
}

impl Static3dTextFinalGrid {
  pub(crate) fn new(
    width_px: u32,
    height_px: u32,
    source_left_px: f32,
    source_top_px: f32,
    source_width_px: f32,
    source_height_px: f32,
  ) -> Option<Self> {
    (width_px > 0
      && height_px > 0
      && source_left_px.is_finite()
      && source_top_px.is_finite()
      && source_width_px.is_finite()
      && source_height_px.is_finite()
      && source_width_px > f32::EPSILON
      && source_height_px > f32::EPSILON)
      .then_some(Self {
        width_px,
        height_px,
        source_left_px,
        source_top_px,
        source_width_px,
        source_height_px,
      })
  }
}

struct Static3dTextFinalGridOutput<'a> {
  target: Static3dTextFinalGrid,
  image: &'a mut Option<RgbaImage>,
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
  viewport_translation_x_px: f32,
  viewport_translation_y_px: f32,
  silhouette_translation_x_px: f32,
  silhouette_translation_y_px: f32,
}

impl Static3dProjection {
  /// Whether horizontal projection can be resolved without the vertical
  /// scene extent. A tilted/revolved view must retain the complete plane.
  pub(crate) fn has_independent_horizontal_axis(self) -> bool {
    self.rotation[0][1] == 0.0 && self.rotation[2][1] == 0.0
  }

  /// Applies the device-space viewport translation that remains after a
  /// caller quantizes a continuous projected origin to an integer bitmap
  /// crop. This is deliberately part of the projection: it moves every
  /// projected surface while leaving model geometry, material coordinates,
  /// camera normals, and lighting unchanged.
  pub(crate) fn with_viewport_translation_px(mut self, x: f32, y: f32) -> Self {
    self.viewport_translation_x_px = x;
    self.viewport_translation_y_px = y;
    self
  }

  /// Records the independent device-space phase used to allocate the output
  /// silhouette. Word renders transformed Direct3D 9 surfaces at the
  /// half-pixel-corrected viewport origin, but quantizes the fixed-output
  /// bitmap extent before that correction. Keeping the two phases separate
  /// prevents an extent adjustment from moving shared bevel boundaries.
  pub(crate) fn with_silhouette_translation_px(mut self, x: f32, y: f32) -> Self {
    self.silhouette_translation_x_px = x;
    self.silhouette_translation_y_px = y;
    self
  }
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
    viewport_translation_x_px: 0.0,
    viewport_translation_y_px: 0.0,
    silhouette_translation_x_px: 0.0,
    silhouette_translation_y_px: 0.0,
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
    resolve_bevel(
      bevel.width.map(|value| value.to_emu()),
      bevel.height.map(|value| value.to_emu()),
      bevel.preset,
      1.0,
    )
    .height_px
  });
  let bottom_bevel_height_pt = shape.bevel_bottom.as_ref().map_or(0.0, |bevel| {
    resolve_bevel(
      bevel.width.map(|value| value.to_emu()),
      bevel.height.map(|value| value.to_emu()),
      bevel.preset,
      1.0,
    )
    .height_px
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
  // Office's top and bottom bevel heights form one ordered depth chain. The
  // top bevel starts at `z` and consumes its height before the authored
  // extrusion begins; the bottom bevel then extends from the extrusion base.
  // A 90-degree Office control keeps the `z` edge fixed while independently
  // growing the opposite edge by bevelT.h and bevelB.h.
  let extrusion_front_pt = z_pt - top_bevel_height_pt;
  let extrusion_back_pt = extrusion_front_pt - depth_pt;
  for depth in [
    z_pt,
    extrusion_front_pt,
    extrusion_back_pt,
    extrusion_back_pt - bottom_bevel_height_pt,
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
  // ECMA-376 §20.1.5.3-4 defines height as how far above the shape a bevel is
  // applied. MS-OI29500 §20.1.10.9 makes that geometry explicit: its height
  // axis moves away from the face, while its width axis moves inward. A bevel
  // therefore does not enlarge the 2-D silhouette for an orthographic-front
  // camera, but its outward terminal plane moves the edge under a rotated or
  // perspective camera. Only a contour grows every edge in screen space.
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
  projected_plane_region_output_bounds(projection, z_pt, model_width_pt, model_height_pt, region)
}

/// Projects one logical region on the Word run effect/backdrop plane.
///
/// A run-level `w14:props3d` solid is adapted to the shared mesh builder by
/// raising its front face through the extrusion and top bevel. Glow, shadow,
/// reflection, and their alignment rectangles remain on the authored source
/// plane at the extrusion base. Native DrawingML styles do not carry that
/// adaptation and naturally fall back to their ordinary front plane.
pub(crate) fn projected_wordprocessing_effect_region_output_bounds(
  projection: Static3dProjection,
  style: &Static3dStyle,
  model_width_pt: f32,
  model_height_pt: f32,
  region: Static3dOutputBounds,
) -> Static3dOutputBounds {
  let z_pt = style.wordprocessing_effect_plane_z_pt.unwrap_or_else(|| {
    style
      .shape
      .z
      .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
      .unwrap_or(0.0)
  });
  projected_plane_region_output_bounds(projection, z_pt, model_width_pt, model_height_pt, region)
}

fn projected_plane_region_output_bounds(
  projection: Static3dProjection,
  z_pt: f32,
  model_width_pt: f32,
  model_height_pt: f32,
  region: Static3dOutputBounds,
) -> Static3dOutputBounds {
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
/// A caller which needs Word's run-level effect/backdrop plane must use
/// [`project_wordprocessing_static_3d_effect_plane`]; its adapted solid front
/// can be at a different Z. Microsoft's Direct2D 3-D transform has the same
/// bitmap input boundary; solid construction remains a separate stage below.
#[cfg(test)]
fn project_static_3d_front_face(
  source: &RgbaImage,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  pixels_per_point: f32,
  model_surface: Option<Static3dSurface>,
) -> RgbaImage {
  let front_z_pt = shape
    .z
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
    .unwrap_or(0.0);
  project_static_3d_plane(
    source,
    projection,
    front_z_pt,
    pixels_per_point,
    model_surface,
  )
}

/// Projects a Word run effect from its authored text/backdrop plane.
///
/// ECMA-376 Part 1 section 20.1.5.2 defines the backdrop as the 3-D plane on
/// which glow and shadow are applied. A run-level `w14:props3d` source plane
/// remains at the extrusion base even though the shared solid builder moves
/// its front face above that base by the extrusion and top-bevel heights.
pub(crate) fn project_wordprocessing_static_3d_effect_plane(
  source: &RgbaImage,
  projection: Static3dProjection,
  style: &Static3dStyle,
  pixels_per_point: f32,
  model_surface: Option<Static3dSurface>,
) -> RgbaImage {
  let plane_z_pt = style.wordprocessing_effect_plane_z_pt.unwrap_or_else(|| {
    style
      .shape
      .z
      .map(|value| value.to_emu() as f32 / EMUS_PER_POINT)
      .unwrap_or(0.0)
  });
  project_static_3d_plane(
    source,
    projection,
    plane_z_pt,
    pixels_per_point,
    model_surface,
  )
}

fn project_static_3d_plane(
  source: &RgbaImage,
  projection: Static3dProjection,
  plane_z_pt: f32,
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
  let plane_z_px = plane_z_pt * pixels_per_point;
  let mut output = RgbaImage::new(source.width(), source.height());
  composite_projected_image(
    &mut output,
    source,
    ProjectedImageOptions {
      projection,
      z: plane_z_px,
      bounds,
      model_surface,
      pixels_per_point,
      tint: None,
    },
  );
  output
}

const DEFAULT_BEVEL_EXTENT_EMU: i64 = 76_200;

#[derive(Clone, Copy, Debug)]
struct ResolvedBevel {
  authored_width_px: f32,
  height_px: f32,
  terminal_inset_px: f32,
  preset: Option<a::BevelPresetValues>,
}

impl ResolvedBevel {
  fn has_authored_surface(self) -> bool {
    // Width describes the inward footprint, but a zero-height profile has no
    // surface. This matters for W14, whose CT_Bevel defaults omitted height
    // to zero instead of DrawingML CT_Bevel's 76200 EMU.
    self.height_px > f32::EPSILON
  }
}

fn resolve_bevel(
  width_emu: Option<i64>,
  height_emu: Option<i64>,
  preset: Option<a::BevelPresetValues>,
  pixels_per_point: f32,
) -> ResolvedBevel {
  // ECMA-376 Part 1 L.4.6.3.2.1 defines one CT_Bevel for both bevelT and
  // bevelB, with 76200 EMU / 76200 EMU / circle defaults. Resolve that shared
  // contract before choosing the front or back reference plane.
  let authored_width_px =
    width_emu.unwrap_or(DEFAULT_BEVEL_EXTENT_EMU) as f32 / EMUS_PER_POINT * pixels_per_point;
  let height_px =
    height_emu.unwrap_or(DEFAULT_BEVEL_EXTENT_EMU) as f32 / EMUS_PER_POINT * pixels_per_point;
  let preset = Some(preset.unwrap_or(a::BevelPresetValues::Circle));
  let terminal_inset_px = authored_width_px * bevel_terminal_inset(preset);

  ResolvedBevel {
    authored_width_px,
    height_px,
    terminal_inset_px,
    preset,
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Static3dGeometryLighting {
  Shape,
  Text,
}

#[derive(Clone, Copy)]
struct Static3dGeometryInput<'a> {
  geometry: Option<&'a Static3dTextGeometry>,
  surface_material_texture: Option<&'a TextMaterialTexture>,
  material_geometry: Option<&'a Static3dTextGeometry>,
  page_geometry: Option<&'a Static3dTextGeometry>,
  front_fill_material: Option<&'a RgbaImage>,
  front_outline_material: Option<&'a RgbaImage>,
  front_outline_coverage: Option<&'a RgbaImage>,
  uniform_paint_opacity: Option<f32>,
  front_fill_uniform_paint_opacity: Option<f32>,
  front_outline_uniform_paint_opacity: Option<f32>,
  front_outline_material_inset_px: Option<f32>,
  front_fill_has_authored_transparency: bool,
  front_outline_has_authored_transparency: bool,
  lighting: Static3dGeometryLighting,
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
  apply_static_3d_impl(
    image,
    scene,
    projection,
    shape,
    options,
    Static3dGeometryInput {
      geometry: None,
      surface_material_texture: None,
      material_geometry: None,
      page_geometry: None,
      front_fill_material: None,
      front_outline_material: None,
      front_outline_coverage: None,
      uniform_paint_opacity: None,
      front_fill_uniform_paint_opacity: None,
      front_outline_uniform_paint_opacity: None,
      front_outline_material_inset_px: None,
      front_fill_has_authored_transparency: false,
      front_outline_has_authored_transparency: false,
      lighting: Static3dGeometryLighting::Shape,
    },
    None,
  );
}

pub(crate) fn apply_static_3d_shape_geometry(
  image: &mut RgbaImage,
  geometry: &Static3dTextGeometry,
  material_geometry: &Static3dTextGeometry,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
) {
  apply_static_3d_impl(
    image,
    scene,
    projection,
    shape,
    options,
    Static3dGeometryInput {
      geometry: Some(geometry),
      surface_material_texture: None,
      material_geometry: Some(material_geometry),
      page_geometry: None,
      front_fill_material: None,
      front_outline_material: None,
      front_outline_coverage: None,
      uniform_paint_opacity: None,
      front_fill_uniform_paint_opacity: None,
      front_outline_uniform_paint_opacity: None,
      front_outline_material_inset_px: None,
      front_fill_has_authored_transparency: false,
      front_outline_has_authored_transparency: false,
      lighting: Static3dGeometryLighting::Shape,
    },
    None,
  );
}

/// Geometry and independently painted materials on the same text surface.
#[derive(Clone, Copy)]
pub(crate) struct Static3dTextSurface<'a> {
  pub(crate) geometry: &'a Static3dTextGeometryPaths,
  pub(crate) front_fill_material: Option<&'a RgbaImage>,
  pub(crate) front_outline_material: Option<&'a RgbaImage>,
  pub(crate) front_outline_coverage: Option<&'a RgbaImage>,
}

#[cfg(test)]
fn apply_static_3d_text(
  image: &mut RgbaImage,
  geometry: &Static3dTextGeometryPaths,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
) {
  apply_static_3d_text_with_outline_material(
    image,
    crate::common::drawingml_3d::Static3dTextSurface {
      geometry,
      front_fill_material: None,
      front_outline_material: None,
      front_outline_coverage: None,
    },
    scene,
    projection,
    shape,
    options,
  );
}

pub(crate) fn apply_static_3d_text_with_outline_material(
  image: &mut RgbaImage,
  surface: Static3dTextSurface<'_>,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
) {
  apply_static_3d_text_with_outline_material_impl(
    image, surface, scene, projection, shape, options, None,
  );
}

pub(crate) fn apply_static_3d_text_with_outline_material_and_final_grid(
  image: &mut RgbaImage,
  surface: Static3dTextSurface<'_>,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
  target: Static3dTextFinalGrid,
) -> Option<RgbaImage> {
  let mut final_grid = None;
  apply_static_3d_text_with_outline_material_impl(
    image,
    surface,
    scene,
    projection,
    shape,
    options,
    Some(Static3dTextFinalGridOutput {
      target,
      image: &mut final_grid,
    }),
  );
  final_grid
}

fn apply_static_3d_text_with_outline_material_impl(
  image: &mut RgbaImage,
  surface: Static3dTextSurface<'_>,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
  final_grid: Option<Static3dTextFinalGridOutput<'_>>,
) {
  let Static3dTextSurface {
    geometry,
    front_fill_material,
    front_outline_material,
    front_outline_coverage,
  } = surface;
  debug_assert!(
    front_fill_material.is_none_or(|material| material.dimensions() == image.dimensions())
  );
  debug_assert!(
    front_outline_material.is_none_or(|material| material.dimensions() == image.dimensions())
  );
  debug_assert!(
    front_outline_coverage.is_none_or(|coverage| coverage.dimensions() == image.dimensions())
  );
  apply_static_3d_impl(
    image,
    scene,
    projection,
    shape,
    options,
    Static3dGeometryInput {
      geometry: Some(&geometry.physical),
      surface_material_texture: geometry.surface_material_texture.as_ref(),
      // The caller rasterizes both the composed paint and independent
      // materials with PerPrimitive page-pixel centers. Only the physical
      // mesh uses Direct3D's half-pixel translation; applying that translation
      // to the texture again misregisters the authored outline against it.
      material_geometry: Some(&geometry.page),
      page_geometry: Some(&geometry.page),
      front_fill_material,
      front_outline_material,
      front_outline_coverage,
      uniform_paint_opacity: geometry.uniform_paint_opacity,
      front_fill_uniform_paint_opacity: geometry.front_fill_uniform_paint_opacity,
      front_outline_uniform_paint_opacity: geometry.front_outline_uniform_paint_opacity,
      front_outline_material_inset_px: geometry.front_outline_material_inset_px,
      front_fill_has_authored_transparency: geometry.front_fill_has_authored_transparency,
      front_outline_has_authored_transparency: geometry.front_outline_has_authored_transparency,
      lighting: Static3dGeometryLighting::Text,
    },
    final_grid,
  );
}

/// Applies Word's text material/light equation to the already painted text
/// surface without replacing that surface by raw glyph geometry.
///
/// Word uses this path for scene-only W14 text and for props coordinates below
/// the effective 7-EMU boundary. The bitmap's combined fill/outline alpha is
/// the geometry, so centered outlines and no-fill rings remain intact.
pub(crate) fn apply_static_3d_text_paint(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
) {
  apply_static_3d_impl(
    image,
    scene,
    projection,
    shape,
    options,
    Static3dGeometryInput {
      geometry: None,
      surface_material_texture: None,
      material_geometry: None,
      page_geometry: None,
      front_fill_material: None,
      front_outline_material: None,
      front_outline_coverage: None,
      uniform_paint_opacity: None,
      front_fill_uniform_paint_opacity: None,
      front_outline_uniform_paint_opacity: None,
      front_outline_material_inset_px: None,
      front_fill_has_authored_transparency: false,
      front_outline_has_authored_transparency: false,
      lighting: Static3dGeometryLighting::Text,
    },
    None,
  );
}

/// Applies the DrawingML material/light equation to an orthographically
/// projected sphere while preserving the source image as the material's
/// Shape color and alpha mask.
///
/// This is deliberately a surface primitive rather than a synthetic
/// `a:sp3d/a:bevelT`: a circle bevel is a boundary profile plus a planar cap,
/// whereas a sphere has a continuously varying normal across its full face.
/// The model rectangle supplies the projected diameter (or a non-uniformly
/// scaled ellipse), so callers can keep the existing vector/raster ownership
/// and effect pipeline.
pub(crate) fn apply_static_3d_sphere_surface(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  material: Option<a::PresetMaterialTypeValues>,
  options: Static3dRenderOptions,
) {
  let Some(bounds) = alpha_bounds(image) else {
    return;
  };
  let model_surface = options.model_surface.unwrap_or(Static3dSurface {
    left_px: bounds.0 as f32,
    top_px: bounds.1 as f32,
    width_px: (bounds.2 - bounds.0 + 1).max(1) as f32,
    height_px: (bounds.3 - bounds.1 + 1).max(1) as f32,
  });
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let radius_x = (model_surface.width_px * 0.5).max(0.5);
  let radius_y = (model_surface.height_px * 0.5).max(0.5);
  let radius_z = radius_x.min(radius_y);
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);

  for (x, y, pixel) in image.enumerate_pixels_mut() {
    if pixel[3] == 0 {
      continue;
    }
    let model_x = x as f32 + 0.5 - center_x;
    let model_y = y as f32 + 0.5 - center_y;
    let model_normal = sphere_surface_normal(model_x / radius_x, model_y / radius_y);
    let model_z = model_normal[2] * radius_z;
    let normal = lighting_surface_normal(scene, projection, model_normal);
    let view_direction = surface_view_direction(
      scene,
      projection,
      [model_x, model_y, model_z],
      width,
      height,
      options.pixels_per_point,
    );
    let shade = legacy_material_diffuse_shade(scene, normal, material);
    let specular = legacy_light_rig_surface_specular(scene, normal, view_direction, material);
    let alpha = pixel[3];
    for channel in 0..3 {
      pixel[channel] =
        shade_gouraud_channel_with_specular(pixel[channel], shade[channel], specular[channel]);
    }
    pixel[3] = alpha;
  }
}

fn sphere_surface_normal(normalized_x: f32, normalized_y: f32) -> [f32; 3] {
  let radial_squared = normalized_x * normalized_x + normalized_y * normalized_y;
  if radial_squared >= 1.0 {
    let inverse_length = radial_squared.sqrt().recip();
    return [
      normalized_x * inverse_length,
      normalized_y * inverse_length,
      0.0,
    ];
  }
  [normalized_x, normalized_y, (1.0 - radial_squared).sqrt()]
}

fn apply_static_3d_impl(
  image: &mut RgbaImage,
  scene: &a::Scene3DType,
  projection: Static3dProjection,
  shape: &a::Shape3DType,
  options: Static3dRenderOptions,
  geometry_input: Static3dGeometryInput<'_>,
  mut final_grid: Option<Static3dTextFinalGridOutput<'_>>,
) {
  let Static3dGeometryInput {
    geometry: text_geometry,
    surface_material_texture,
    material_geometry,
    page_geometry,
    front_fill_material,
    front_outline_material,
    front_outline_coverage,
    uniform_paint_opacity,
    front_fill_uniform_paint_opacity,
    front_outline_uniform_paint_opacity,
    front_outline_material_inset_px,
    front_fill_has_authored_transparency,
    front_outline_has_authored_transparency,
    lighting: geometry_lighting,
  } = geometry_input;
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
  let mut contour_width_px = shape
    .contour_width
    .map(|value| value.to_emu() as f32 / EMUS_PER_POINT * pixels_per_point)
    .unwrap_or(0.0)
    .clamp(0.0, 64.0);
  let top_bevel = shape.bevel_top.as_ref().map(|bevel| {
    resolve_bevel(
      bevel.width.map(|value| value.to_emu()),
      bevel.height.map(|value| value.to_emu()),
      bevel.preset,
      pixels_per_point,
    )
  });
  let bottom_bevel = shape.bevel_bottom.as_ref().map(|bevel| {
    resolve_bevel(
      bevel.width.map(|value| value.to_emu()),
      bevel.height.map(|value| value.to_emu()),
      bevel.preset,
      pixels_per_point,
    )
  });
  let visible_top_bevel = top_bevel.filter(|bevel| bevel.has_authored_surface());
  let visible_bottom_bevel = bottom_bevel.filter(|bevel| bevel.has_authored_surface());
  if geometry_lighting == Static3dGeometryLighting::Text
    && depth_pt <= f32::EPSILON
    && visible_top_bevel.is_none()
    && visible_bottom_bevel.is_none()
    && projection_preserves_source_plane_coverage(projection)
  {
    // An exact-config Word camera/contour control keeps a contour-only
    // neutral orthographic scene byte-identical to the ordinary front face.
    // A rotated scene makes the same authored contour visible, as does any
    // real bevel or extrusion depth; keep those independently observed paths.
    contour_width_px = 0.0;
  }
  let contour_radius_px = shape
    .contour_width
    .map(|_| contour_width_px * 0.5)
    .unwrap_or(0.0)
    .round()
    .clamp(0.0, 32.0) as i32;
  // `w14:textOutline` and the glyph fill remain independently painted front
  // materials when `a:sp3d/@contourW` is present. The contour is a separate
  // solid ridge around the 3-D geometry; it does not flatten those authored
  // paint layers back into the precomposed source bitmap. Exact-config Word
  // controls at contourW={0,12700} and outline alpha={0,40000,100000} retain
  // an independent outline-alpha response in both contour columns.
  let separated_front_text_materials = geometry_lighting == Static3dGeometryLighting::Text;
  // ECMA-376 uses the same CT_Bevel profile at the two ends of the solid.
  // MS-OI29500 places bevelT above the top of the extrusion and bevelB below
  // its base. Office's 90-degree height controls make the reference sequence
  // observable: the inset top cap remains at z, the top bevel descends to the
  // extrusion, the extrusion retains its authored length, and the bottom
  // bevel descends from its base to the inset back cap.
  let extrusion_front_z_px =
    visible_top_bevel.map_or(front_z_px, |bevel| front_z_px - bevel.height_px);
  let extrusion_back_z_px = extrusion_front_z_px - depth_pt * pixels_per_point;
  let planar_front_z_px = visible_top_bevel.map_or(front_z_px, |bevel| {
    if geometry_lighting == Static3dGeometryLighting::Text {
      extrusion_front_z_px + word_text_bevel_terminal_profile(bevel.preset).height * bevel.height_px
    } else {
      front_z_px
    }
  });
  let planar_back_z_px = visible_bottom_bevel.map_or(extrusion_back_z_px, |bevel| {
    let terminal_height = if geometry_lighting == Static3dGeometryLighting::Text {
      word_text_bevel_terminal_profile(bevel.preset).height
    } else {
      1.0
    };
    extrusion_back_z_px - terminal_height * bevel.height_px
  });
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
  let word_text_lighting = geometry_lighting == Static3dGeometryLighting::Text;
  let text_contour = (contour_width_px > f32::EPSILON).then(|| {
    contour_color.unwrap_or(Static3dColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 255,
    })
  });
  // Direct2D's text extruder submits front, back, and side triangles in one
  // back-face-culled mesh. A separately projected back bitmap bypasses both
  // culling and the shared multisample/depth grid, so even a camera-facing
  // front acquires a second antialiased alpha edge underneath it. Preserve the
  // raster fallback for non-text geometry, but reject a vector-text back cap
  // whenever its -Z face points away from the camera.
  let word_text_back_cap_culled = word_text_lighting
    && text_geometry.is_some()
    && !surface_faces_camera(
      projection,
      [0.0, 0.0, -1.0],
      [0.0, 0.0, planar_back_z_px],
      bounds_width,
      bounds_height,
      pixels_per_point,
    );
  let word_text_back_surfaces_fully_occluded = word_text_lighting
    && text_geometry.is_some()
    && projection_preserves_source_plane_coverage(projection);
  let material_geometry = material_geometry.or(text_geometry);
  let page_geometry = page_geometry.or(material_geometry);
  let mut text_surface_triangles = Vec::new();
  let text_back_planar_inset_px = visible_bottom_bevel.map_or(0.0, |bevel| {
    if geometry_lighting == Static3dGeometryLighting::Text {
      bevel.authored_width_px * word_text_bevel_terminal_profile(bevel.preset).inset
    } else {
      bevel.terminal_inset_px
    }
  });
  if depth_pt > f32::EPSILON {
    let steps = projected_depth_steps(
      projection,
      extrusion_front_z_px,
      extrusion_back_z_px,
      bounds_width,
      bounds_height,
      pixels_per_point,
    );
    let extrusion = extrusion_color.unwrap_or_else(|| average_extrusion_color(&front));
    let back_normal = lighting_surface_normal(scene, projection, [0.0, 0.0, -1.0]);
    let back_view_direction = surface_view_direction(
      scene,
      projection,
      [0.0, 0.0, planar_back_z_px],
      bounds_width,
      bounds_height,
      pixels_per_point,
    );
    let back_shade = if word_text_lighting {
      material_diffuse_shade(
        scene,
        back_normal,
        back_view_direction,
        shape.preset_material,
      )
    } else {
      legacy_material_diffuse_shade(scene, back_normal, shape.preset_material)
    };
    let back_material = Static3dColor {
      alpha: material_surface_alpha(
        f32::from(extrusion.alpha) / 255.0,
        back_normal,
        back_view_direction,
        shape.preset_material,
      ),
      ..extrusion
    };
    if !wireframe && !word_text_back_cap_culled {
      let mut back_face = RgbaImage::new(image.width(), image.height());
      let options = ProjectedImageOptions {
        projection,
        z: planar_back_z_px,
        bounds,
        model_surface,
        pixels_per_point,
        tint: Some((back_material, back_shade)),
      };
      if let Some(geometry) = text_geometry {
        composite_projected_text_geometry(
          &mut back_face,
          &front,
          geometry,
          text_back_planar_inset_px,
          options,
        );
      } else {
        composite_projected_image(&mut back_face, &front, options);
      }
      if text_geometry.is_none()
        && let Some(bevel) = visible_bottom_bevel
      {
        let mask = back_face.clone();
        let _ = composite_bevel(
          &mut back_face,
          &mask,
          BevelOptions {
            width: bevel.terminal_inset_px,
            height: bevel.height_px,
            preset: bevel.preset,
            scene,
            projection,
            model_surface,
            pixels_per_point,
            surface_z: extrusion_back_z_px,
            material: shape.preset_material,
            back_face: true,
            height_direction: -1.0,
          },
        );
      }
      composite_image(image, &back_face);
    }
    if !wireframe
      && !word_text_back_surfaces_fully_occluded
      && let (Some(geometry), Some(material_geometry), Some(bevel)) =
        (text_geometry, material_geometry, visible_bottom_bevel)
    {
      let bottom_bevel_mesh = text_bevel_triangles(
        &front,
        geometry,
        material_geometry,
        page_geometry.unwrap_or(material_geometry),
        uniform_paint_opacity,
        TextBevelOptions {
          geometry_width: bevel.authored_width_px,
          normal_width: bevel.authored_width_px,
          height: bevel.height_px,
          preset: bevel.preset,
          scene,
          projection,
          model_surface,
          pixels_per_point,
          surface_z: extrusion_back_z_px,
          material: shape.preset_material,
          geometry_lighting,
          back_face: true,
          height_direction: -1.0,
          fill_material: None,
          outline_material: None,
          outline_coverage: None,
          fill_uniform_paint_opacity: None,
          outline_uniform_paint_opacity: None,
          outline_material_inset_px: None,
          fill_has_authored_transparency: false,
          outline_has_authored_transparency: false,
        },
      );
      text_surface_triangles.extend(bottom_bevel_mesh.triangles);
    }
    let options = ExtrusionEdgeOptions {
      bounds,
      model_surface,
      projection,
      front_z: extrusion_front_z_px,
      back_z: extrusion_back_z_px,
      pixels_per_point,
      steps,
      tint: extrusion,
      scene,
      material: shape.preset_material,
      wireframe,
      geometry_lighting,
      outline_contour: text_contour.map(|color| TextExtrusionOutlineContour {
        width_px: contour_width_px,
        color,
      }),
    };
    if let Some(geometry) = text_geometry.filter(|_| !wireframe) {
      text_surface_triangles.extend(text_extrusion_edge_triangles(geometry, options));
    } else {
      composite_extrusion_edges(image, &front, options);
    }
  }
  // Raster-only shapes and wireframe text retain the projected-silhouette
  // fallback. Vector text submits its front contour to the shared surface
  // depth buffer below.
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
      composite_projected_text_geometry(&mut silhouette, &front, geometry, 0.0, options);
    } else {
      composite_projected_image(&mut silhouette, &front, options);
    }
    composite_outline(image, &silhouette, contour_radius_px, contour);
  }
  let mut front_face = front.clone();
  let mut top_bevel = None;
  let mut text_planar_inset_px = 0.0;
  let mut text_bevel = None;
  let mut text_planar_inset_cells = None;
  // MS-OI29500 defines every Office bevel, including `circle`, as a 2-D
  // profile swept inward from the vector face boundary. Keep text on that
  // parametric surface path; the raster fallback below is only for callers
  // that do not provide glyph geometry.
  if let (Some(_geometry), Some(bevel)) = (text_geometry, visible_top_bevel) {
    text_planar_inset_px = if geometry_lighting == Static3dGeometryLighting::Text {
      bevel.authored_width_px * word_text_bevel_terminal_profile(bevel.preset).inset
    } else {
      bevel.terminal_inset_px
    };
    text_bevel = Some(TextBevelOptions {
      geometry_width: bevel.authored_width_px,
      normal_width: bevel.authored_width_px,
      height: bevel.height_px,
      preset: bevel.preset,
      scene,
      projection,
      model_surface,
      pixels_per_point,
      surface_z: extrusion_front_z_px,
      material: shape.preset_material,
      geometry_lighting,
      back_face: false,
      height_direction: 1.0,
      fill_material: separated_front_text_materials
        .then_some(front_fill_material)
        .flatten(),
      outline_material: separated_front_text_materials
        .then_some(front_outline_material)
        .flatten(),
      outline_coverage: separated_front_text_materials
        .then_some(front_outline_coverage)
        .flatten(),
      fill_uniform_paint_opacity: front_fill_uniform_paint_opacity,
      outline_uniform_paint_opacity: front_outline_uniform_paint_opacity,
      outline_material_inset_px: front_outline_material_inset_px,
      fill_has_authored_transparency: front_fill_has_authored_transparency,
      outline_has_authored_transparency: front_outline_has_authored_transparency,
    });
  } else if let (None, Some(bevel)) = (text_geometry, visible_top_bevel) {
    let options = BevelOptions {
      width: bevel.terminal_inset_px,
      height: bevel.height_px,
      preset: bevel.preset,
      scene,
      projection,
      model_surface,
      pixels_per_point,
      surface_z: extrusion_front_z_px,
      material: shape.preset_material,
      back_face: false,
      height_direction: 1.0,
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
    if let (Some(geometry), Some(material_geometry), Some(options)) =
      (text_geometry, material_geometry, text_bevel)
    {
      let top_bevel_mesh = text_bevel_triangles(
        &front,
        geometry,
        material_geometry,
        page_geometry.unwrap_or(material_geometry),
        uniform_paint_opacity,
        options,
      );
      text_planar_inset_cells = top_bevel_mesh.direct_inset_cells;
      text_surface_triangles.extend(top_bevel_mesh.triangles);
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
    // Activation, not the presence of a top bevel, selects the lit solid.
    // resolve_static_3d_style already leaves inert props-only text on the
    // ordinary 2-D path. Exact-config solid-color controls retain front-face
    // diffuse/specular lighting with extrusion and no top bevel; removing
    // all activating geometry is the separate unlit negative control.
    shade_planar_surface(
      &mut front_face,
      scene,
      &options,
      [0.0, 0.0, 1.0],
      shape.preset_material,
      word_text_lighting,
    );
    let planar_fill_material = separated_front_text_materials
      .then_some(front_fill_material)
      .flatten();
    let planar_outline_material = separated_front_text_materials
      .then_some(front_outline_material)
      .flatten();
    if let (Some(geometry), Some(material_geometry)) = (text_geometry, material_geometry) {
      let page_geometry = page_geometry.unwrap_or(geometry);
      let contour = text_contour;
      let source_plane_contour = projection_preserves_source_plane_coverage(projection);
      let has_physical_contour_host =
        depth_pt > f32::EPSILON || visible_top_bevel.is_some() || visible_bottom_bevel.is_some();
      if let Some(color) = contour.filter(|_| !source_plane_contour && has_physical_contour_host) {
        text_surface_triangles.extend(text_projected_contour_triangles(
          geometry,
          TextProjectedContourOptions {
            width_px: contour_width_px,
            base_z: extrusion_front_z_px,
            color,
            projection,
            model_surface,
            pixels_per_point,
          },
        ));
        if depth_pt > f32::EPSILON {
          // [MS-OI29500] 20.1.5.6 requires the same solid-filled contour on
          // the outline edges of the extrusion, not only around its front
          // edge. The back edge is a second copy of the underlying 2-D
          // outline at the extrusion base; the shared depth buffer leaves
          // only the camera-visible portions of that ridge.
          text_surface_triangles.extend(text_projected_contour_triangles(
            geometry,
            TextProjectedContourOptions {
              width_px: contour_width_px,
              base_z: extrusion_back_z_px,
              color,
              projection,
              model_surface,
              pixels_per_point,
            },
          ));
        }
      }
      let contour_surface =
        contour
          .filter(|_| source_plane_contour)
          .map(|color| TextContourSurfaceInput {
            width_px: contour_width_px,
            base_z: extrusion_front_z_px,
            color,
          });
      let mut solid = RgbaImage::new(image.width(), image.height());
      let surface_input = TextSolidSurfaceInput {
        scene,
        material: shape.preset_material,
        bevel_material_source: &front,
        surface_material_texture,
        source_geometry: geometry,
        material_geometry,
        paint_opacity_geometry: page_geometry,
        planar_fill_material,
        planar_outline_material,
        planar_outline_coverage: front_outline_coverage,
        planar_fill_uniform_paint_opacity: front_fill_uniform_paint_opacity,
        planar_outline_uniform_paint_opacity: front_outline_uniform_paint_opacity,
        planar_outline_material_inset_px: front_outline_material_inset_px,
        fill_has_authored_transparency: front_fill_has_authored_transparency,
        outline_has_authored_transparency: front_outline_has_authored_transparency,
        uniform_paint_opacity,
        planar_geometry: geometry,
        planar_coverage_geometry: geometry,
        planar_inset_px: text_planar_inset_px,
        direct_inset_cells: text_planar_inset_cells.as_deref(),
        planar_coverage: TextPlanarCoverageResolve::BoxSamples8x4,
        triangles: &text_surface_triangles,
        direct_inset_bevel: text_bevel
          .filter(|_| geometry_lighting == Static3dGeometryLighting::Text),
        contour: contour_surface,
        options,
        geometry_lighting,
      };
      composite_text_solid_surfaces(
        &mut solid,
        &front_face,
        surface_input,
        TextSurfaceRasterization::SourceGrid,
      );
      if !projection.parallel
        && word_text_back_cap_culled
        && let Some(final_grid) = final_grid.as_mut()
      {
        let mut resolved = RgbaImage::new(final_grid.target.width_px, final_grid.target.height_px);
        composite_text_solid_surfaces(
          &mut resolved,
          &front_face,
          surface_input,
          TextSurfaceRasterization::FinalGrid(final_grid.target),
        );
        *final_grid.image = Some(resolved);
      }
      let source_plane_coverage = (geometry_lighting == Static3dGeometryLighting::Text
        && projection_preserves_source_plane_coverage(projection))
      .then(|| {
        // Word's fixed-output alpha remains the raw front-face geometry when
        // an orthographic camera maps every depth to the same x/y plane.
        // Extrusion and bevel meshes still own RGB and depth, but cannot
        // become additional alpha layers. Resolve the uninset front plane
        // independently so overlapping back/bevel surfaces cannot compound
        // its antialias coverage.
        let mut coverage = RgbaImage::new(image.width(), image.height());
        composite_text_solid_surfaces(
          &mut coverage,
          &front_face,
          surface_input.independent_front_coverage(),
          TextSurfaceRasterization::SourceGrid,
        );
        if contour_width_px > 0.0 {
          let contour = contour_color.unwrap_or(Static3dColor {
            color: RgbColor { r: 0, g: 0, b: 0 },
            alpha: 255,
          });
          let mut contoured = RgbaImage::new(image.width(), image.height());
          composite_text_contour_stroke(
            &mut contoured,
            geometry,
            contour_width_px,
            // The exact natural XPS/PDF controls and no-bevel contour matrix
            // freeze the source-plane mask on the physical glyph path with
            // no second phase adjustment. The raised RGB contour separately
            // reverses HPC-to-IPC while competing with the 3-D mesh above.
            0.0,
            contour,
          );
          composite_image(&mut contoured, &coverage);
          coverage = contoured;
        }
        coverage
      });
      // The contour competes with the lit front mesh in the same subpixel
      // depth buffer. Identity output retains its independently pinned AreaA8;
      // rotated/perspective output submits the pre-projection 3-D contour mesh
      // above. A painter outline of the completed silhouette cannot represent
      // the swept band or its depth ownership.
      composite_image(image, &solid);
      if let Some(coverage) = source_plane_coverage.as_ref() {
        replace_surface_alpha(image, coverage);
      }
    } else {
      composite_projected_image(image, &front_face, options);
    }
  }

  if (text_geometry.is_none() || geometry_lighting == Static3dGeometryLighting::Shape)
    && contour_radius_px == 0
    && !wireframe
    && projection_preserves_source_plane_coverage(projection)
  {
    // MS-OI29500 defines the top bevel as an inward sweep whose outer edge is
    // the authored face boundary. With an identity orthographic projection,
    // neither that sweep nor a coincident extrusion changes the 2-D
    // silhouette. Preserve the source coverage exactly while retaining the
    // material-lit RGB produced above. This also preserves arbitrary paths,
    // holes, and authored fractional alpha instead of assuming a rectangle.
    replace_surface_alpha(image, &front);
  }
}

fn replace_surface_alpha(surface: &mut RgbaImage, coverage: &RgbaImage) {
  debug_assert_eq!(surface.dimensions(), coverage.dimensions());
  for (surface, coverage) in surface.pixels_mut().zip(coverage.pixels()) {
    if coverage[3] == 0 {
      *surface = Rgba([0, 0, 0, 0]);
    } else {
      if surface[3] == 0 {
        surface[0] = coverage[0];
        surface[1] = coverage[1];
        surface[2] = coverage[2];
      }
      surface[3] = coverage[3];
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

const TEXT_BEVEL_PROFILE_FLATTENING_TOLERANCE_PX: f32 = 0.25;
const TEXT_BEVEL_PROFILE_MAX_INTERVALS: usize = 4_096;

#[derive(Clone, Copy, Debug)]
struct BezierHfd {
  basis: [[f64; 2]; 4],
  remaining_steps: usize,
  parameter: f64,
  step_size: f64,
}

impl BezierHfd {
  fn new(points: [[f64; 2]; 4]) -> Self {
    let [first, control_1, control_2, last] = points;
    Self {
      basis: [
        first,
        [last[0] - first[0], last[1] - first[1]],
        [
          6.0 * (control_1[0] - 2.0 * control_2[0] + last[0]),
          6.0 * (control_1[1] - 2.0 * control_2[1] + last[1]),
        ],
        [
          6.0 * (first[0] - 2.0 * control_1[0] + control_2[0]),
          6.0 * (first[1] - 2.0 * control_1[1] + control_2[1]),
        ],
      ],
      remaining_steps: 1,
      parameter: 0.0,
      step_size: 1.0,
    }
  }

  fn error(vector: [f64; 2]) -> f64 {
    vector[0].abs().max(vector[1].abs())
  }

  fn halve_step(&mut self) {
    self.basis[2] = [
      (self.basis[2][0] + self.basis[3][0]) * 0.125,
      (self.basis[2][1] + self.basis[3][1]) * 0.125,
    ];
    self.basis[1] = [
      (self.basis[1][0] - self.basis[2][0]) * 0.5,
      (self.basis[1][1] - self.basis[2][1]) * 0.5,
    ];
    self.basis[3] = [self.basis[3][0] * 0.25, self.basis[3][1] * 0.25];
    self.remaining_steps *= 2;
    self.step_size *= 0.5;
  }

  fn advance(&mut self) {
    self.basis[0] = [
      self.basis[0][0] + self.basis[1][0],
      self.basis[0][1] + self.basis[1][1],
    ];
    let previous_second = self.basis[2];
    self.basis[1] = [
      self.basis[1][0] + previous_second[0],
      self.basis[1][1] + previous_second[1],
    ];
    self.basis[2] = [
      2.0 * previous_second[0] - self.basis[3][0],
      2.0 * previous_second[1] - self.basis[3][1],
    ];
    self.basis[3] = previous_second;
    self.parameter += self.step_size;
    self.remaining_steps -= 1;
  }

  fn try_double_step(&mut self, quarter_tolerance: f64) -> bool {
    if !self.remaining_steps.is_multiple_of(2) {
      return false;
    }
    let next_second = [
      2.0 * self.basis[2][0] - self.basis[3][0],
      2.0 * self.basis[2][1] - self.basis[3][1],
    ];
    if Self::error(self.basis[3]) > quarter_tolerance
      || Self::error(next_second) > quarter_tolerance
    {
      return false;
    }
    self.basis[1] = [
      2.0 * self.basis[1][0] + self.basis[2][0],
      2.0 * self.basis[1][1] + self.basis[2][1],
    ];
    self.basis[3] = [self.basis[3][0] * 4.0, self.basis[3][1] * 4.0];
    self.basis[2] = [next_second[0] * 4.0, next_second[1] * 4.0];
    self.remaining_steps /= 2;
    self.step_size *= 2.0;
    true
  }
}

fn flatten_cubic_profile_parameters(points: [[f64; 2]; 4], tolerance: f32) -> Vec<f32> {
  // WPF's CBezierFlattener tests the two HFD second-derivative terms, which
  // are six times the geometric approximation error.
  let tolerance = f64::from(tolerance.max(0.0)) * 6.0;
  let quarter_tolerance = tolerance * 0.25;
  let mut hfd = BezierHfd::new(points);
  while (BezierHfd::error(hfd.basis[2]) > tolerance || BezierHfd::error(hfd.basis[3]) > tolerance)
    && hfd.remaining_steps <= TEXT_BEVEL_PROFILE_MAX_INTERVALS / 2
  {
    hfd.halve_step();
  }

  let mut parameters = Vec::with_capacity(hfd.remaining_steps + 1);
  parameters.push(0.0);
  while hfd.remaining_steps > 1 {
    hfd.advance();
    parameters.push(hfd.parameter.clamp(0.0, 1.0) as f32);
    if BezierHfd::error(hfd.basis[2]) > tolerance
      && hfd.remaining_steps <= TEXT_BEVEL_PROFILE_MAX_INTERVALS / 2
    {
      hfd.halve_step();
    } else {
      while hfd.try_double_step(quarter_tolerance) {}
    }
  }
  parameters.push(1.0);
  parameters
}

fn word_text_bevel_geometry_point(
  preset: Option<a::BevelPresetValues>,
  terminal_height: f32,
  point: [f32; 2],
) -> [f32; 2] {
  let normalized_height = point[0] / terminal_height.max(f32::EPSILON);
  if matches!(
    preset.unwrap_or(a::BevelPresetValues::Circle),
    a::BevelPresetValues::Circle | a::BevelPresetValues::RelaxedInset
  ) {
    [point[1], normalized_height]
  } else {
    [normalized_height, point[1]]
  }
}

fn word_text_bevel_material_boundary_parameters(
  preset: Option<a::BevelPresetValues>,
  segment_index: usize,
  boundary_inset: f32,
) -> Vec<f32> {
  if !boundary_inset.is_finite() {
    return Vec::new();
  }
  let profile = bevel_profile(preset);
  let terminal_height = bevel_segment_endpoint(*profile.last().expect("bevel profile"))[0];
  let start = if segment_index == 0 {
    [0.0, 0.0]
  } else {
    bevel_segment_endpoint(profile[segment_index - 1])
  };
  let coordinate =
    |point| f64::from(word_text_bevel_geometry_point(preset, terminal_height, point)[1]);
  let start = coordinate(start);
  let boundary = f64::from(boundary_inset);
  let roots = match profile[segment_index] {
    BevelProfileSegment::Line { to } => {
      let delta = coordinate(to) - start;
      if delta.abs() <= f64::EPSILON {
        Vec::new()
      } else {
        vec![(boundary - start) / delta]
      }
    }
    BevelProfileSegment::Quadratic { control, to } => {
      let control = coordinate(control);
      let end = coordinate(to);
      kurbo::common::solve_quadratic(
        start - boundary,
        2.0 * (control - start),
        start - 2.0 * control + end,
      )
      .into_iter()
      .collect()
    }
    BevelProfileSegment::Cubic {
      control_1,
      control_2,
      to,
    } => {
      let control_1 = coordinate(control_1);
      let control_2 = coordinate(control_2);
      let end = coordinate(to);
      kurbo::common::solve_cubic(
        start - boundary,
        3.0 * (control_1 - start),
        3.0 * (start - 2.0 * control_1 + control_2),
        -start + 3.0 * control_1 - 3.0 * control_2 + end,
      )
      .into_iter()
      .collect()
    }
  };
  let mut roots = roots
    .into_iter()
    .filter(|root| root.is_finite() && *root > 1.0e-7 && *root < 1.0 - 1.0e-7)
    .map(|root| root as f32)
    .collect::<Vec<_>>();
  roots.sort_by(f32::total_cmp);
  roots.dedup_by(|left, right| (*left - *right).abs() <= 1.0e-6);
  roots
}

fn word_text_bevel_segment_parameters(
  normal_width: f32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  segment_index: usize,
  tolerance_px: f32,
) -> Vec<f32> {
  let profile = bevel_profile(preset);
  let terminal_height = bevel_segment_endpoint(*profile.last().expect("bevel profile"))[0];
  let start = if segment_index == 0 {
    [0.0, 0.0]
  } else {
    bevel_segment_endpoint(profile[segment_index - 1])
  };
  let segment = profile[segment_index];
  let cubic = match segment {
    BevelProfileSegment::Line { .. } => return vec![0.0, 1.0],
    BevelProfileSegment::Quadratic { control, to } => {
      let control_1 = [
        start[0] + (control[0] - start[0]) * (2.0 / 3.0),
        start[1] + (control[1] - start[1]) * (2.0 / 3.0),
      ];
      let control_2 = [
        to[0] + (control[0] - to[0]) * (2.0 / 3.0),
        to[1] + (control[1] - to[1]) * (2.0 / 3.0),
      ];
      [start, control_1, control_2, to]
    }
    BevelProfileSegment::Cubic {
      control_1,
      control_2,
      to,
    } => [start, control_1, control_2, to],
  };
  let points = cubic.map(|point| {
    let [profile_height, profile_inset] =
      word_text_bevel_geometry_point(preset, terminal_height, point);
    [
      f64::from(profile_height * height.abs()),
      f64::from(profile_inset * normal_width.abs()),
    ]
  });
  flatten_cubic_profile_parameters(points, tolerance_px)
}

fn word_text_bevel_geometry_profile(
  preset: Option<a::BevelPresetValues>,
  profile: BevelProfileSample,
) -> BevelProfileSample {
  // MS-OI29500 section 20.1.10.9 defines the generic DrawingML curve as
  // x=height/y=inset. MS-DOCX's independently defined W14 text types only
  // refer to that section for the preset meanings; they do not specify how
  // Word's text mesh consumes the two axes. Exact-config Word print controls
  // isolate the missing contract: across 24 heights and two contour widths,
  // Circle owns at least as much physical surface as the equal-axis Angle in
  // every stratum. The published Circle curve can produce that direction only
  // when W14 sweeps normalized x inward and uses raw y as height.
  // RelaxedInset controls independently place its fold at the published x
  // boundary and use the same mapping. Keep untested nonlinear presets on the
  // generic contract until they have their own W14 discriminator.
  if matches!(
    preset.unwrap_or(a::BevelPresetValues::Circle),
    a::BevelPresetValues::Circle | a::BevelPresetValues::RelaxedInset
  ) {
    return BevelProfileSample {
      height: profile.inset,
      inset: profile.height,
      height_tangent: profile.inset_tangent,
      inset_tangent: profile.height_tangent,
    };
  }
  profile
}

fn word_text_bevel_lighting_profile(
  preset: Option<a::BevelPresetValues>,
  profile: BevelProfileSample,
) -> BevelProfileSample {
  let preset = preset.unwrap_or(a::BevelPresetValues::Circle);
  if matches!(
    preset,
    a::BevelPresetValues::Circle | a::BevelPresetValues::RelaxedInset
  ) {
    // These profiles already carry W14 geometry-axis tangents. For Circle,
    // the former reflected-parameter lighting workaround was an observational
    // alias of this swap because the cubic is symmetric; it hid the incorrect
    // depth geometry and must not remain a second source of axis ownership.
    return profile;
  }

  // MS-OI29500 describes the profile's x/y coordinates as height/inset
  // geometry, but Word's fixed-output text mesh presents their derivative
  // components to the material normal in the opposite order for the remaining
  // unresolved profiles. Circle and RelaxedInset have their source-backed or
  // independently pinned rules above. Keep the authored geometry untouched until each other
  // branch/preset is independently resolved.
  BevelProfileSample {
    height_tangent: profile.inset_tangent,
    inset_tangent: profile.height_tangent,
    ..profile
  }
}

fn word_text_bevel_terminal_profile(preset: Option<a::BevelPresetValues>) -> BevelProfileSample {
  let profile = bevel_profile(preset);
  word_text_bevel_geometry_profile(
    preset,
    bevel_profile_sample(preset, profile.len().saturating_sub(1), 1.0),
  )
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

fn reflected_circle_bevel_lighting_profile(profile: BevelProfileSample) -> BevelProfileSample {
  // MS-OI29500 section 20.1.10.9 defines the Circle curve in height/inset
  // space. Office shape controls and the complete Word width sweep both run
  // its lighting normal from the side-facing outer edge to the face-facing
  // inner edge by reflecting the geometric inset before evaluating that
  // curve. Do not reflect the geometry itself.
  let (_, height_tangent, inset_tangent) = circle_bevel_profile(1.0 - profile.inset);
  BevelProfileSample {
    height_tangent,
    inset_tangent,
    ..profile
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
    // MS-OI29500 publishes the vectors in light-rig coordinates. Office first
    // applies the fixed 90-degree basis conversion and then the rig direction
    // or explicit rotation. The result follows D3DLIGHT9::Direction: it points
    // in the direction the light travels, while the diffuse equation consumes
    // the opposite, vertex-to-light direction.
    let direction = resolved_light_direction(scene, *light, rotation_degrees);
    let level = light.scale * dot3([-direction[0], -direction[1], -direction[2]], normal).max(0.0)
      + light.offset;
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
  shape_color: [u8; 3],
) -> [f32; 3] {
  word_text_surface_specular(
    light_rig_surface_specular_incident(scene, normal, view_direction, material),
    material,
    shape_color,
  )
}

/// Resolves the light/view-dependent specular input before applying the
/// material color. WPF's mesh path carries this independently from texture
/// coordinates and diffuse vertex color, allowing a gradient or outlined
/// source to supply its local `Shape` color after raster interpolation.
fn light_rig_surface_specular_incident(
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
    if blinn_highlight && normal_light <= 0.0 {
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
      // MS-OI29500's material table selects the reflection-vector highlight
      // unless `Blinn Highlight` is Yes. Word evaluates this term independently
      // of the diffuse N dot L gate: a light with zero diffuse contribution can
      // still supply a positive reflected-view term at a bevel vertex. Keep
      // that contribution before interpolation, including at vertices whose
      // normal faces away from the viewer. Clipping diffuse must not erase
      // the separately submitted specular attribute.
      let reflected = [
        2.0 * normal_light * normal[0] - toward_light[0],
        2.0 * normal_light * normal[1] - toward_light[1],
        2.0 * normal_light * normal[2] - toward_light[2],
      ];
      dot3(reflected, view_direction).max(0.0)
    };
    let level = highlight.powf(power);
    for (channel, light_color) in specular.iter_mut().zip(light.color) {
      *channel += light_color * level;
    }
  }
  specular
}

fn word_text_surface_specular(
  incident: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
  shape_color: [u8; 3],
) -> [f32; 3] {
  let material_reflectance = word_text_material_specular_reflectance(material, shape_color);
  std::array::from_fn(|channel| (incident[channel] * material_reflectance[channel]).clamp(0.0, 1.0))
}

/// The established shape path retains its independently calibrated minimum
/// illumination, while both shape and W14 text use D3D's light-travel vector
/// convention.
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
  view_direction: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 3] {
  let shade = scale_shade(
    light_rig_surface_shade(scene, normal),
    material_diffusion(material),
  );
  apply_material_diffuse_fresnel(shade, normal, view_direction, material)
}

fn apply_material_diffuse_fresnel(
  shade: [f32; 3],
  normal: [f32; 3],
  view_direction: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 3] {
  let fresnel = material_diffuse_fresnel(material);
  if fresnel == 0 {
    return shade;
  }

  // ECMA-376 defines the sign and exponent of each preset's Diffuse Fresnel
  // term. A complete Word matrix over the +/- presets and five Angle-bevel
  // slopes pins the fixed-output transfer function: use the glancing-angle
  // weight `(1 - N dot V)^abs(fresnel)` to interpolate the lit Shape factor
  // toward one for positive values and toward zero for negative values.
  let facing = dot3(normal, view_direction).clamp(0.0, 1.0);
  let weight = (1.0 - facing).powi(fresnel.abs());
  if fresnel > 0 {
    shade.map(|channel| channel + (1.0 - channel) * weight)
  } else {
    shade.map(|channel| channel * (1.0 - weight))
  }
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

fn transformed_bevel_surface_normal(
  outward: [f32; 2],
  normal_xy: f32,
  normal_z: f32,
  page_plane_scale_x: f32,
  page_plane_scale_y: f32,
  depth_scale: f32,
) -> [f32; 3] {
  // Direct3D 9 transforms a vertex normal by the inverse transpose of the
  // world-view matrix before lighting. Word's Screen bitmap can normalize
  // the page plane into an allocation whose x/y scales differ from the
  // point-to-pixel depth scale. The contour normal already has the transformed
  // x/y direction; its directional scale restores the missing magnitude so
  // the bevel slope is transformed consistently with z.
  let page_plane_directional_scale = (outward[0] * page_plane_scale_x)
    .hypot(outward[1] * page_plane_scale_y)
    .max(f32::EPSILON);
  let mut normal = [
    outward[0] * normal_xy / page_plane_directional_scale,
    outward[1] * normal_xy / page_plane_directional_scale,
    normal_z / depth_scale.max(f32::EPSILON),
  ];
  normalize3(&mut normal);
  normal
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

fn material_diffuse_fresnel(material: Option<a::PresetMaterialTypeValues>) -> i32 {
  use a::PresetMaterialTypeValues as M;
  match material.unwrap_or(M::WarmMatte) {
    M::Clear => -8,
    M::DarkEdge => -2,
    M::Flat | M::LegacyMatte => -4,
    M::Metal | M::SoftEdge => 4,
    M::Powder | M::TranslucentPowder => 2,
    M::LegacyMetal
    | M::LegacyPlastic
    | M::LegacyWireframe
    | M::Matte
    | M::Plastic
    | M::SoftMetal
    | M::WarmMatte => 0,
  }
}

/// Returns the diffuse-material alpha multiplier used by Office's 3-D
/// material pipeline.
///
/// MS-OI29500 section 20.1.10.50 gives `translucentPowder` a 0.7 diffuse
/// alpha and `clear` a 0.1 diffuse alpha. Direct3D 9 sources material alpha
/// from the diffuse material component; ambient alpha does not replace it.
fn material_base_alpha(material: Option<a::PresetMaterialTypeValues>) -> f32 {
  use a::PresetMaterialTypeValues as M;
  match material.unwrap_or(M::WarmMatte) {
    M::Clear => 0.1,
    M::TranslucentPowder => 0.7,
    M::DarkEdge
    | M::Flat
    | M::LegacyMatte
    | M::LegacyMetal
    | M::LegacyPlastic
    | M::LegacyWireframe
    | M::Matte
    | M::Metal
    | M::Plastic
    | M::Powder
    | M::SoftEdge
    | M::SoftMetal
    | M::WarmMatte => 1.0,
  }
}

fn material_alpha_fresnel(material: Option<a::PresetMaterialTypeValues>) -> i32 {
  use a::PresetMaterialTypeValues as M;
  match material.unwrap_or(M::WarmMatte) {
    M::Clear => 1,
    M::SoftEdge => -10,
    M::TranslucentPowder => -1,
    M::DarkEdge
    | M::Flat
    | M::LegacyMatte
    | M::LegacyMetal
    | M::LegacyPlastic
    | M::LegacyWireframe
    | M::Matte
    | M::Metal
    | M::Plastic
    | M::Powder
    | M::SoftMetal
    | M::WarmMatte => 0,
  }
}

fn material_surface_alpha_factor(
  normal: [f32; 3],
  view_direction: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> f32 {
  let base = material_base_alpha(material);
  let fresnel = material_alpha_fresnel(material);
  if fresnel == 0 {
    return base;
  }

  // ECMA-376 fixes the sign semantics, MS-OI29500 fixes every preset value,
  // and the Microsoft DirectX Fresnel samples use this glancing-angle weight.
  // A 40-case exact Word matrix across positive, zero and negative presets
  // pins the same single-precision transfer over five constant-normal slopes.
  let facing = dot3(normal, view_direction).clamp(0.0, 1.0);
  let weight = (1.0 - facing).powi(fresnel.abs());
  if fresnel > 0 {
    base + (1.0 - base) * weight
  } else {
    base * (1.0 - weight)
  }
}

fn material_surface_alpha(
  authored_opacity: f32,
  normal: [f32; 3],
  view_direction: [f32; 3],
  material: Option<a::PresetMaterialTypeValues>,
) -> u8 {
  // Word quantizes the material/vertex alpha before resolving antialias
  // coverage. The h/w=1 softEdge control distinguishes this floor from
  // round-to-nearest: 255 * 0.99999535f is emitted as 254.
  (authored_opacity.clamp(0.0, 1.0)
    * material_surface_alpha_factor(normal, view_direction, material)
    * 255.0)
    .floor()
    .clamp(0.0, 255.0) as u8
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

/// Resolves Word's static-3-D text specular material color.
///
/// The non-metal constants follow MS-OI29500 20.1.10.50. Word's W14 fixed-
/// output path has one narrower compatibility rule for all three metallic
/// presets: it uses `Lerp(Shape, White, 0.5)` instead of the three distinct
/// colors in that table. An exact-config Office matrix pairs each preset with
/// a material having identical power and Blinn state under black fill; the
/// independently solved reflectances are 0.49945 (Metal), 0.49754
/// (LegacyMetal), and 0.50216 (SoftMetal). Eight-color interpolation and the
/// Shape=1 endpoints establish the shared linear transfer.
///
/// Keep this material color distinct from each light's color: Direct3D's
/// fixed-function equation multiplies those inputs and adds specular after the
/// texture cascade.
fn word_text_material_specular_reflectance(
  material: Option<a::PresetMaterialTypeValues>,
  shape_color: [u8; 3],
) -> [f32; 3] {
  use a::PresetMaterialTypeValues as M;
  let shape = shape_color.map(|channel| f32::from(channel) / 255.0);
  match material.unwrap_or(M::WarmMatte) {
    M::LegacyMatte | M::LegacyWireframe | M::Matte => [0.0; 3],
    M::LegacyPlastic | M::DarkEdge | M::SoftEdge => [1.0; 3],
    M::LegacyMetal | M::Metal | M::SoftMetal => shape.map(|channel| channel * 0.5 + 0.5),
    M::Plastic | M::Clear => [0.6; 3],
    M::WarmMatte | M::TranslucentPowder | M::Powder => [0.3; 3],
    M::Flat => [0.8; 3],
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

pub(crate) fn projection_preserves_source_plane_coverage(projection: Static3dProjection) -> bool {
  const EPSILON: f32 = 1.0e-6;
  let nearly = |left: f32, right: f32| (left - right).abs() <= EPSILON;

  // For a parallel camera the x/y rows completely describe the page-plane
  // mapping at every depth. Requiring the identity basis as well as zero
  // depth translation excludes rotated and oblique cameras, even when their
  // front plane happens to cross z=0 without an offset.
  projection.parallel
    && nearly(projection.rotation[0][0], 1.0)
    && nearly(projection.rotation[0][1], 0.0)
    && nearly(projection.rotation[1][0], 0.0)
    && nearly(projection.rotation[1][1], 1.0)
    && nearly(projection.rotation[0][2] + projection.skew_x_per_depth, 0.0)
    && nearly(projection.rotation[1][2] + projection.skew_y_per_depth, 0.0)
    && nearly(projection.viewport_translation_x_px, 0.0)
    && nearly(projection.viewport_translation_y_px, 0.0)
}

fn plane_homography(
  projection: Static3dProjection,
  z: f32,
  width: f32,
  height: f32,
  pixels_per_point: f32,
) -> [[f32; 3]; 3] {
  let rotation = projection.rotation;
  let mut matrix = if projection.parallel {
    [
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
    ]
  } else {
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
  };

  // A viewport translation is an affine transform after the homogeneous
  // projection. Adding dx/dy times the denominator row is equivalent for
  // both parallel and perspective cameras and cannot alter model-space
  // normals or material coordinates.
  let denominator_row = matrix[2];
  let [x_row, y_row, _] = &mut matrix;
  for ((x, y), denominator) in x_row.iter_mut().zip(y_row).zip(denominator_row) {
    *x += projection.viewport_translation_x_px * denominator;
    *y += projection.viewport_translation_y_px * denominator;
  }
  matrix
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

#[derive(Clone, Copy)]
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
  let legacy_shade =
    (!word_text_lighting).then(|| legacy_material_diffuse_shade(scene, normal, material));
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
    let shade = if word_text_lighting {
      material_diffuse_shade(scene, normal, view_direction, material)
    } else {
      legacy_shade.expect("legacy planar shade")
    };
    let specular = if word_text_lighting {
      light_rig_surface_specular(
        scene,
        normal,
        view_direction,
        material,
        [pixel[0], pixel[1], pixel[2]],
      )
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
  inset_px: f32,
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
  let planar_inset = TextPlanarInset::new(geometry, inset_px, None, false);
  let Some(source_path) = text_geometry_path(geometry, |point| point) else {
    return;
  };
  let Some(source_mask) = text_geometry_mask(source.width(), source.height(), &source_path) else {
    return;
  };
  let projected_mask = if inset_px <= f32::EPSILON {
    let Some(projected_path) = text_geometry_path(geometry, |point| {
      let projected = map_homogeneous(matrix, point.0 - center_x, point.1 - center_y);
      (center_x + projected.0, center_y + projected.1)
    }) else {
      return;
    };
    let Some(projected_mask) =
      text_geometry_mask(destination.width(), destination.height(), &projected_path)
    else {
      return;
    };
    Some(projected_mask)
  } else {
    None
  };

  for target_y in 0..destination.height() {
    for target_x in 0..destination.width() {
      let target_coverage = if let Some(projected_mask) = projected_mask.as_ref() {
        f32::from(
          projected_mask
            .pixel(target_x, target_y)
            .map_or(0, |pixel| pixel.alpha()),
        ) / 255.0
      } else {
        const SAMPLE_GRID_X: usize = 8;
        const SAMPLE_GRID_Y: usize = 4;
        let mut covered = 0_u32;
        for sample_y in 0..SAMPLE_GRID_Y {
          for sample_x in 0..SAMPLE_GRID_X {
            let target = (
              target_x as f32 + (sample_x as f32 + 0.5) / SAMPLE_GRID_X as f32,
              target_y as f32 + (sample_y as f32 + 0.5) / SAMPLE_GRID_Y as f32,
            );
            let source_local = map_homogeneous(inverse, target.0 - center_x, target.1 - center_y);
            let source_point = (center_x + source_local.0, center_y + source_local.1);
            covered += u32::from(planar_inset.contains(geometry, source_point, inset_px));
          }
        }
        covered as f32 / (SAMPLE_GRID_X * SAMPLE_GRID_Y) as f32
      };
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

struct TextGeometryAreaMask {
  left: i32,
  top: i32,
  width: u32,
  height: u32,
  alpha: Vec<u8>,
}

impl TextGeometryAreaMask {
  fn alpha_at(&self, x: i32, y: i32) -> u8 {
    let local_x = x - self.left;
    let local_y = y - self.top;
    if local_x < 0 || local_y < 0 || local_x >= self.width as i32 || local_y >= self.height as i32 {
      return 0;
    }
    self.alpha[local_y as usize * self.width as usize + local_x as usize]
  }

  fn bounds(&self) -> (f32, f32, f32, f32) {
    (
      self.left as f32,
      self.top as f32,
      self.left as f32 + self.width as f32,
      self.top as f32 + self.height as f32,
    )
  }
}

/// Bilinearly samples the curve-preserving glyph alpha texture in the same
/// absolute device coordinate system as the material images. Pixels outside
/// the tightly allocated mask are transparent rather than edge-clamped.
fn sample_text_geometry_area_mask(mask: &TextGeometryAreaMask, x: f32, y: f32) -> f32 {
  if !x.is_finite() || !y.is_finite() {
    return 0.0;
  }
  let x0 = x.floor() as i32;
  let y0 = y.floor() as i32;
  let fraction_x = x - x0 as f32;
  let fraction_y = y - y0 as f32;
  [
    (x0, y0, (1.0 - fraction_x) * (1.0 - fraction_y)),
    (x0 + 1, y0, fraction_x * (1.0 - fraction_y)),
    (x0, y0 + 1, (1.0 - fraction_x) * fraction_y),
    (x0 + 1, y0 + 1, fraction_x * fraction_y),
  ]
  .into_iter()
  .map(|(sample_x, sample_y, weight)| f32::from(mask.alpha_at(sample_x, sample_y)) / 255.0 * weight)
  .sum::<f32>()
  .clamp(0.0, 1.0)
}

/// Rasterizes the original curved glyph path into a tightly bounded 8-bit
/// area mask.
///
/// WPF retains DirectWrite's byte alpha realization when the same glyph run
/// is uploaded to its Direct3D glyph bank. Controlled Word print exports pin
/// the matching fixed-output rule: as soon as 7 EMU selects the physical
/// static-3-D surface, otherwise bare text exposes 241..=254 distinct SMask
/// values across independent three-letter controls. A finite 8x4 occupancy
/// grid cannot represent that source alpha. Preserve the original curves and
/// rasterize their non-zero winding area directly instead of routing coverage
/// through the independently flattened lighting mesh.
fn text_geometry_area_mask(geometry: &Static3dTextGeometry) -> Option<TextGeometryAreaMask> {
  rasterize_kurbo_area_mask(&geometry.source_coverage_path, 0.0, 0.0)
}

fn text_geometry_stroke_area_mask(
  geometry: &Static3dTextGeometry,
  width_px: f32,
  phase_px: f32,
) -> Option<TextGeometryAreaMask> {
  if geometry.source_coverage_path.is_empty()
    || !width_px.is_finite()
    || width_px <= 0.0
    || !phase_px.is_finite()
  {
    return None;
  }
  let outline = widen_centered_miter_butt(&geometry.source_coverage_path, width_px)?;
  rasterize_kurbo_wpf_8x8_area_mask(&outline, phase_px, phase_px)
}

/// Converts a curve-preserving kurbo path to a tightly allocated grayscale
/// area mask. The integer placement remains in the caller's coordinate space;
/// only the temporary raster path is translated into its local allocation.
fn rasterize_kurbo_area_mask(
  path: &BezPath,
  translate_x: f32,
  translate_y: f32,
) -> Option<TextGeometryAreaMask> {
  rasterize_kurbo_mask(path, translate_x, translate_y, rasterize_nonzero_area_path)
}

/// Resolves a widened text contour with the 8x8 coverage contract used by
/// WPF/MIL and Word fixed output. Geometry widening remains independent; only
/// its final nonzero fill changes from continuous analytic area to the 64-step
/// byte-alpha lattice.
fn rasterize_kurbo_wpf_8x8_area_mask(
  path: &BezPath,
  translate_x: f32,
  translate_y: f32,
) -> Option<TextGeometryAreaMask> {
  rasterize_kurbo_mask(
    path,
    translate_x,
    translate_y,
    rasterize_nonzero_wpf_8x8_path,
  )
}

fn rasterize_kurbo_mask(
  path: &BezPath,
  translate_x: f32,
  translate_y: f32,
  rasterize: impl FnOnce(&BezPath, u32, u32, f64, f64) -> Option<Vec<u8>>,
) -> Option<TextGeometryAreaMask> {
  if path.elements().is_empty() || !translate_x.is_finite() || !translate_y.is_finite() {
    return None;
  }

  let bounds = path.bounding_box();
  let left = checked_floor_i32(bounds.x0 + f64::from(translate_x))?;
  let top = checked_floor_i32(bounds.y0 + f64::from(translate_y))?;
  let right = checked_ceil_i32(bounds.x1 + f64::from(translate_x))?;
  let bottom = checked_ceil_i32(bounds.y1 + f64::from(translate_y))?;
  let width = u32::try_from(right.checked_sub(left)?).ok()?;
  let height = u32::try_from(bottom.checked_sub(top)?).ok()?;
  if width == 0 || height == 0 {
    return None;
  }
  let _ = usize::try_from(width)
    .ok()?
    .checked_mul(usize::try_from(height).ok()?)?;

  let local_x = f64::from(translate_x) - f64::from(left);
  let local_y = f64::from(translate_y) - f64::from(top);
  let alpha = rasterize(path, width, height, local_x, local_y)?;
  if alpha.iter().all(|value| *value == 0) {
    return None;
  }
  debug_assert_eq!(alpha.len(), width as usize * height as usize);
  Some(TextGeometryAreaMask {
    left,
    top,
    width,
    height,
    alpha,
  })
}

fn checked_floor_i32(value: f64) -> Option<i32> {
  let value = value.floor();
  (value.is_finite() && value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX))
    .then_some(value as i32)
}

fn checked_ceil_i32(value: f64) -> Option<i32> {
  let value = value.ceil();
  (value.is_finite() && value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX))
    .then_some(value as i32)
}

const AREA_PIXEL_BITS: u32 = 8;
const AREA_ONE_PIXEL: i64 = 1_i64 << AREA_PIXEL_BITS;

#[derive(Clone, Copy, Debug, Default)]
struct AnalyticAreaCell {
  x: i32,
  cover: i64,
  area: i64,
}

#[derive(Clone, Copy, Debug, Default)]
struct AnalyticFixedPoint {
  x: i64,
  y: i64,
}

/// Safe fixed-point cell/area sweep for non-zero path fills.
///
/// Each directed edge contributes its signed vertical cover and first moment
/// to the cells it crosses. A left-to-right prefix then reconstructs the
/// covered area of every pixel. Coordinates are quantized to 24.8 fixed point,
/// matching the byte-alpha precision of the output without supersampling a
/// large intermediate bitmap.
struct AnalyticAreaRasterizer {
  width: i32,
  height: i32,
  rows: Vec<Vec<AnalyticAreaCell>>,
  start: AnalyticFixedPoint,
  current: AnalyticFixedPoint,
  cell_x: i32,
  cell_y: i32,
  cell_cover: i64,
  cell_area: i64,
  cell_valid: bool,
  contour_open: bool,
}

impl AnalyticAreaRasterizer {
  fn new(width: u32, height: u32) -> Option<Self> {
    let width = i32::try_from(width).ok()?;
    let height = i32::try_from(height).ok()?;
    Some(Self {
      width,
      height,
      rows: vec![Vec::new(); usize::try_from(height).ok()?],
      start: AnalyticFixedPoint::default(),
      current: AnalyticFixedPoint::default(),
      cell_x: 0,
      cell_y: 0,
      cell_cover: 0,
      cell_area: 0,
      cell_valid: false,
      contour_open: false,
    })
  }

  fn move_to(&mut self, point: AnalyticFixedPoint) {
    if self.contour_open {
      self.line_to(self.start);
    }
    self.set_cell(area_trunc(point.x), area_trunc(point.y));
    self.start = point;
    self.current = point;
    self.contour_open = true;
  }

  fn close(&mut self) {
    if self.contour_open {
      self.line_to(self.start);
      self.contour_open = false;
    }
  }

  fn set_cell(&mut self, x: i32, y: i32) {
    self.flush_cell();
    self.cell_x = x.max(-1);
    self.cell_y = y;
    self.cell_valid = y >= 0 && y < self.height && x < self.width;
  }

  fn flush_cell(&mut self) {
    if self.cell_valid && (self.cell_area != 0 || self.cell_cover != 0) {
      self.rows[self.cell_y as usize].push(AnalyticAreaCell {
        x: self.cell_x,
        cover: self.cell_cover,
        area: self.cell_area,
      });
    }
    self.cell_cover = 0;
    self.cell_area = 0;
  }

  fn line_to(&mut self, point: AnalyticFixedPoint) {
    let to_x = point.x;
    let to_y = point.y;
    let from_x = self.current.x;
    let from_y = self.current.y;
    let mut row = area_trunc(from_y);
    let target_row = area_trunc(to_y);
    if (row >= self.height && target_row >= self.height) || (row < 0 && target_row < 0) {
      self.current = point;
      return;
    }

    let mut column = area_trunc(from_x);
    let target_column = area_trunc(to_x);
    let mut fraction_x = area_fraction(from_x);
    let mut fraction_y = area_fraction(from_y);
    let delta_x = to_x - from_x;
    let delta_y = to_y - from_y;

    if column == target_column && row == target_row {
      // The remainder below accounts for the complete segment.
    } else if delta_y == 0 {
      self.set_cell(target_column, target_row);
      self.current = point;
      return;
    } else if delta_x == 0 {
      if delta_y > 0 {
        loop {
          let next_fraction_y = AREA_ONE_PIXEL;
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * fraction_x * 2;
          fraction_y = 0;
          row += 1;
          self.set_cell(column, row);
          if row == target_row {
            break;
          }
        }
      } else {
        loop {
          let next_fraction_y = 0;
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * fraction_x * 2;
          fraction_y = AREA_ONE_PIXEL;
          row -= 1;
          self.set_cell(column, row);
          if row == target_row {
            break;
          }
        }
      }
    } else {
      let mut product = delta_x * fraction_y - delta_y * fraction_x;
      let reciprocal_x = if column != target_column {
        0x00ff_ffff / delta_x
      } else {
        0
      };
      let reciprocal_y = if row != target_row {
        0x00ff_ffff / delta_y
      } else {
        0
      };
      loop {
        if product <= 0 && product - delta_x * AREA_ONE_PIXEL > 0 {
          let next_fraction_x = 0;
          let next_fraction_y = area_reciprocal_multiply(-product, -reciprocal_x);
          product -= delta_y * AREA_ONE_PIXEL;
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * (fraction_x + next_fraction_x);
          fraction_x = AREA_ONE_PIXEL;
          fraction_y = next_fraction_y;
          column -= 1;
        } else if product - delta_x * AREA_ONE_PIXEL <= 0
          && product - delta_x * AREA_ONE_PIXEL + delta_y * AREA_ONE_PIXEL > 0
        {
          product -= delta_x * AREA_ONE_PIXEL;
          let next_fraction_x = area_reciprocal_multiply(-product, reciprocal_y);
          let next_fraction_y = AREA_ONE_PIXEL;
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * (fraction_x + next_fraction_x);
          fraction_x = next_fraction_x;
          fraction_y = 0;
          row += 1;
        } else if product - delta_x * AREA_ONE_PIXEL + delta_y * AREA_ONE_PIXEL <= 0
          && product + delta_y * AREA_ONE_PIXEL >= 0
        {
          product += delta_y * AREA_ONE_PIXEL;
          let next_fraction_x = AREA_ONE_PIXEL;
          let next_fraction_y = area_reciprocal_multiply(product, reciprocal_x);
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * (fraction_x + next_fraction_x);
          fraction_x = 0;
          fraction_y = next_fraction_y;
          column += 1;
        } else {
          let next_fraction_x = area_reciprocal_multiply(product, -reciprocal_y);
          let next_fraction_y = 0;
          product += delta_x * AREA_ONE_PIXEL;
          self.cell_cover += next_fraction_y - fraction_y;
          self.cell_area += (next_fraction_y - fraction_y) * (fraction_x + next_fraction_x);
          fraction_x = next_fraction_x;
          fraction_y = AREA_ONE_PIXEL;
          row -= 1;
        }
        self.set_cell(column, row);
        if column == target_column && row == target_row {
          break;
        }
      }
    }

    let final_fraction_x = area_fraction(to_x);
    let final_fraction_y = area_fraction(to_y);
    self.cell_cover += final_fraction_y - fraction_y;
    self.cell_area += (final_fraction_y - fraction_y) * (fraction_x + final_fraction_x);
    self.current = point;
  }

  fn quad_to(&mut self, control: AnalyticFixedPoint, point: AnalyticFixedPoint) {
    let mut arc = [AnalyticFixedPoint::default(); 33];
    arc[0] = point;
    arc[1] = control;
    arc[2] = self.current;
    if area_curve_outside_vertical_clip(&arc[..3], self.height) {
      self.current = point;
      return;
    }

    let mut curvature = (arc[2].x + arc[0].x - 2 * arc[1].x)
      .abs()
      .max((arc[2].y + arc[0].y - 2 * arc[1].y).abs());
    let mut remaining = 1_i32;
    while curvature > AREA_ONE_PIXEL / 4 {
      curvature >>= 2;
      remaining <<= 1;
    }
    let mut offset = 0_usize;
    loop {
      let mut split = remaining & remaining.wrapping_neg();
      loop {
        split >>= 1;
        if split == 0 {
          break;
        }
        split_analytic_quad(&mut arc[offset..]);
        offset += 2;
      }
      self.line_to(arc[offset]);
      remaining -= 1;
      if remaining == 0 {
        break;
      }
      offset -= 2;
    }
  }

  fn curve_to(
    &mut self,
    control1: AnalyticFixedPoint,
    control2: AnalyticFixedPoint,
    point: AnalyticFixedPoint,
  ) {
    let mut arc = [AnalyticFixedPoint::default(); 129];
    arc[0] = point;
    arc[1] = control2;
    arc[2] = control1;
    arc[3] = self.current;
    if area_curve_outside_vertical_clip(&arc[..4], self.height) {
      self.current = point;
      return;
    }

    let mut offset = 0_usize;
    loop {
      let requires_split = (2 * arc[offset].x - 3 * arc[offset + 1].x + arc[offset + 3].x).abs()
        > AREA_ONE_PIXEL / 2
        || (2 * arc[offset].y - 3 * arc[offset + 1].y + arc[offset + 3].y).abs()
          > AREA_ONE_PIXEL / 2
        || (arc[offset].x - 3 * arc[offset + 2].x + 2 * arc[offset + 3].x).abs()
          > AREA_ONE_PIXEL / 2
        || (arc[offset].y - 3 * arc[offset + 2].y + 2 * arc[offset + 3].y).abs()
          > AREA_ONE_PIXEL / 2;
      if requires_split {
        if arc.len() - offset >= 7 {
          split_analytic_cubic(&mut arc[offset..]);
          offset += 3;
          continue;
        }
        self.line_to(point);
        return;
      }
      self.line_to(arc[offset]);
      if offset == 0 {
        return;
      }
      offset -= 3;
    }
  }

  fn finish(mut self) -> Vec<u8> {
    self.close();
    self.flush_cell();
    let mut alpha = vec![0; self.width as usize * self.height as usize];
    for (y, row) in self.rows.iter_mut().enumerate() {
      row.sort_unstable_by_key(|cell| cell.x);
      let mut cover = 0_i64;
      let mut x = 0_i32;
      let mut index = 0;
      while index < row.len() {
        let cell_x = row[index].x;
        if cover != 0 && cell_x > x {
          let value = nonzero_area_alpha(cover);
          let end = cell_x.min(self.width);
          if end > x {
            alpha[y * self.width as usize + x as usize..y * self.width as usize + end as usize]
              .fill(value);
          }
        }

        let mut cell_cover = 0_i64;
        let mut cell_area = 0_i64;
        while index < row.len() && row[index].x == cell_x {
          cell_cover += row[index].cover;
          cell_area += row[index].area;
          index += 1;
        }
        cover += cell_cover * AREA_ONE_PIXEL * 2;
        let area = cover - cell_area;
        if area != 0 && cell_x >= 0 && cell_x < self.width {
          alpha[y * self.width as usize + cell_x as usize] = nonzero_area_alpha(area);
        }
        x = cell_x + 1;
      }
      if cover != 0 && x < self.width {
        let start = x.max(0) as usize;
        alpha[y * self.width as usize + start..(y + 1) * self.width as usize]
          .fill(nonzero_area_alpha(cover));
      }
    }
    alpha
  }
}

fn rasterize_nonzero_area_path(
  path: &BezPath,
  width: u32,
  height: u32,
  translate_x: f64,
  translate_y: f64,
) -> Option<Vec<u8>> {
  let translate_x = translate_x as f32;
  let translate_y = translate_y as f32;
  let to_fixed = |point: kurbo::Point| {
    let x = ((point.x as f32) + translate_x) * AREA_ONE_PIXEL as f32;
    let y = ((point.y as f32) + translate_y) * AREA_ONE_PIXEL as f32;
    if !x.is_finite()
      || !y.is_finite()
      || f64::from(x) < i64::MIN as f64
      || f64::from(x) > i64::MAX as f64
      || f64::from(y) < i64::MIN as f64
      || f64::from(y) > i64::MAX as f64
    {
      return None;
    }
    Some(AnalyticFixedPoint {
      x: x.trunc() as i64,
      y: y.trunc() as i64,
    })
  };

  let mut rasterizer = AnalyticAreaRasterizer::new(width, height)?;
  for element in path.iter() {
    match element {
      PathEl::MoveTo(point) => rasterizer.move_to(to_fixed(point)?),
      PathEl::LineTo(point) => rasterizer.line_to(to_fixed(point)?),
      PathEl::QuadTo(control, point) => {
        rasterizer.quad_to(to_fixed(control)?, to_fixed(point)?);
      }
      PathEl::CurveTo(control1, control2, point) => {
        rasterizer.curve_to(to_fixed(control1)?, to_fixed(control2)?, to_fixed(point)?);
      }
      PathEl::ClosePath => rasterizer.close(),
    }
  }
  Some(rasterizer.finish())
}

fn area_reciprocal_multiply(value: i64, reciprocal: i64) -> i64 {
  debug_assert!(value >= 0 && reciprocal >= 0);
  ((i128::from(value) * i128::from(reciprocal)) >> (32 - AREA_PIXEL_BITS)) as i64
}

fn area_curve_outside_vertical_clip(points: &[AnalyticFixedPoint], height: i32) -> bool {
  points.iter().all(|point| area_trunc(point.y) >= height)
    || points.iter().all(|point| area_trunc(point.y) < 0)
}

fn split_analytic_quad(points: &mut [AnalyticFixedPoint]) {
  points[4].x = points[2].x;
  let mut a = points[0].x + points[1].x;
  let mut b = points[1].x + points[2].x;
  points[3].x = b >> 1;
  points[2].x = (a + b) >> 2;
  points[1].x = a >> 1;

  points[4].y = points[2].y;
  a = points[0].y + points[1].y;
  b = points[1].y + points[2].y;
  points[3].y = b >> 1;
  points[2].y = (a + b) >> 2;
  points[1].y = a >> 1;
}

fn split_analytic_cubic(points: &mut [AnalyticFixedPoint]) {
  points[6].x = points[3].x;
  let mut a = points[0].x + points[1].x;
  let mut b = points[1].x + points[2].x;
  let mut c = points[2].x + points[3].x;
  points[5].x = c >> 1;
  c += b;
  points[4].x = c >> 2;
  points[1].x = a >> 1;
  a += b;
  points[2].x = a >> 2;
  points[3].x = (a + c) >> 3;

  points[6].y = points[3].y;
  a = points[0].y + points[1].y;
  b = points[1].y + points[2].y;
  c = points[2].y + points[3].y;
  points[5].y = c >> 1;
  c += b;
  points[4].y = c >> 2;
  points[1].y = a >> 1;
  a += b;
  points[2].y = a >> 2;
  points[3].y = (a + c) >> 3;
}

fn area_trunc(value: i64) -> i32 {
  (value >> AREA_PIXEL_BITS) as i32
}

fn area_fraction(value: i64) -> i64 {
  value & (AREA_ONE_PIXEL - 1)
}

fn nonzero_area_alpha(mut area: i64) -> u8 {
  area >>= AREA_PIXEL_BITS * 2 + 1 - 8;
  if area < 0 {
    area = !area;
  }
  area.min(255) as u8
}

fn composite_text_contour_stroke(
  destination: &mut RgbaImage,
  geometry: &Static3dTextGeometry,
  width_px: f32,
  phase_px: f32,
  color: Static3dColor,
) {
  if !width_px.is_finite() || width_px <= 0.0 || color.alpha == 0 {
    return;
  }
  let Some(mask) = text_geometry_stroke_area_mask(geometry, width_px, phase_px) else {
    return;
  };
  for local_y in 0..mask.height {
    let destination_y = mask.top + local_y as i32;
    if destination_y < 0 || destination_y >= destination.height() as i32 {
      continue;
    }
    for local_x in 0..mask.width {
      let destination_x = mask.left + local_x as i32;
      if destination_x < 0 || destination_x >= destination.width() as i32 {
        continue;
      }
      let coverage = mask.alpha[local_y as usize * mask.width as usize + local_x as usize];
      let alpha = ((u16::from(coverage) * u16::from(color.alpha) + 127) / 255) as u8;
      if alpha != 0 {
        blend_over(
          destination.get_pixel_mut(destination_x as u32, destination_y as u32),
          Rgba([color.color.r, color.color.g, color.color.b, alpha]),
        );
      }
    }
  }
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

fn sample_pixmap_alpha_clamped(pixmap: &Pixmap, x: f32, y: f32) -> Option<f32> {
  let max_x = pixmap.width().checked_sub(1)? as f32;
  let max_y = pixmap.height().checked_sub(1)? as f32;
  sample_pixmap_alpha(pixmap, x.clamp(0.0, max_x), y.clamp(0.0, max_y))
}

fn sample_text_paint_opacity(
  source: &RgbaImage,
  source_mask: Option<&Pixmap>,
  uniform_paint_opacity: Option<f32>,
  source_geometry: &Static3dTextGeometry,
  paint_opacity_geometry: &Static3dTextGeometry,
  source_point: (f32, f32),
) -> Option<f32> {
  if let Some(opacity) = uniform_paint_opacity {
    return Some(opacity.clamp(0.0, 1.0));
  }
  let source_mask = source_mask?;
  let opacity_point = source_geometry.map_point_to(paint_opacity_geometry, source_point);
  let x = opacity_point.0 - 0.5;
  let y = opacity_point.1 - 0.5;
  let sample_alpha = sample_bilinear(source, x, y)?[3];
  let source_coverage = sample_pixmap_alpha(source_mask, x, y)?;
  Some(recover_text_paint_opacity(sample_alpha, source_coverage))
}

fn sample_text_paint_opacity_clamped(
  source: &RgbaImage,
  source_mask: Option<&Pixmap>,
  uniform_paint_opacity: Option<f32>,
  source_geometry: &Static3dTextGeometry,
  paint_opacity_geometry: &Static3dTextGeometry,
  source_point: (f32, f32),
) -> Option<f32> {
  if let Some(opacity) = uniform_paint_opacity {
    return Some(opacity.clamp(0.0, 1.0));
  }
  let source_mask = source_mask?;
  let opacity_point = source_geometry.map_point_to(paint_opacity_geometry, source_point);
  let max_x = source.width().checked_sub(1)? as f32;
  let max_y = source.height().checked_sub(1)? as f32;
  let x = (opacity_point.0 - 0.5).clamp(0.0, max_x);
  let y = (opacity_point.1 - 0.5).clamp(0.0, max_y);
  let sample_alpha = sample_bilinear_clamped(source, x, y)?[3];
  let source_coverage = sample_pixmap_alpha_clamped(source_mask, x, y)?;
  Some(recover_text_paint_opacity(sample_alpha, source_coverage))
}

fn recover_text_paint_opacity(sample_alpha: u8, source_coverage: f32) -> f32 {
  if sample_alpha == 0 || source_coverage <= f32::EPSILON {
    return 0.0;
  }
  let sampled_alpha = f32::from(sample_alpha) / 255.0;
  // The painted RGBA source has already rounded coverage to one byte, while
  // the separately sampled glyph mask is still a float. Values within the
  // source byte's half-step are the same fully opaque paint observation, not
  // authored transparency. Dividing them directly turns 129/255 over
  // 0.5078125 into 0.996199 and later floors an opaque matte surface to 254.
  const HALF_ALPHA_STEP: f32 = 0.5 / 255.0;
  if (sampled_alpha - source_coverage).abs() <= HALF_ALPHA_STEP + f32::EPSILON {
    1.0
  } else {
    (sampled_alpha / source_coverage).clamp(0.0, 1.0)
  }
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

fn sample_bilinear_clamped(image: &RgbaImage, x: f32, y: f32) -> Option<Rgba<u8>> {
  let max_x = image.width().checked_sub(1)? as f32;
  let max_y = image.height().checked_sub(1)? as f32;
  sample_bilinear(image, x.clamp(0.0, max_x), y.clamp(0.0, max_y))
}

#[derive(Clone, Copy)]
struct TextProjectedContourOptions {
  width_px: f32,
  base_z: f32,
  color: Static3dColor,
  projection: Static3dProjection,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
}

fn text_projected_contour_triangles(
  geometry: &Static3dTextGeometry,
  options: TextProjectedContourOptions,
) -> Vec<TextSurfaceTriangle> {
  let TextProjectedContourOptions {
    width_px,
    base_z,
    color,
    projection,
    model_surface,
    pixels_per_point,
  } = options;
  let radius = width_px * 0.5;
  if radius <= f32::EPSILON || color.alpha == 0 {
    return Vec::new();
  }

  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let model_width = model_surface.width_px.max(1.0);
  let model_height = model_surface.height_px.max(1.0);
  let vertex_color = [
    f32::from(color.color.r),
    f32::from(color.color.g),
    f32::from(color.color.b),
    f32::from(color.alpha),
  ];
  let project = |point: (f32, f32), z: f32| {
    // Contour, bevel and extrusion share continuous source coordinates.
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
      color: vertex_color,
      textured_material: None,
      bevel_collision: None,
    }
  };

  geometry
    .contours
    .iter()
    .flat_map(|contour| contour_mesh::triangles(contour, radius, base_z))
    .map(|triangle| TextSurfaceTriangle {
      vertices: triangle.map(|point| project((point[0], point[1]), point[2])),
      bevel_source: None,
    })
    .collect()
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
  geometry_lighting: Static3dGeometryLighting,
  outline_contour: Option<TextExtrusionOutlineContour>,
}

#[derive(Clone, Copy)]
struct TextExtrusionOutlineContour {
  width_px: f32,
  color: Static3dColor,
}

#[cfg(test)]
fn text_extrusion_surface_contour_joins(
  points: &[(f32, f32)],
  incoming_curves: &[bool],
  source_joins: &[bool],
  solid_on_right: bool,
) -> Vec<bool> {
  debug_assert_eq!(points.len(), incoming_curves.len());
  debug_assert_eq!(points.len(), source_joins.len());
  let count = points.len();
  // A source-segment boundary is not by itself an extrusion ridge. In
  // particular, rounded font terminals include nearly tangent curve/line
  // boundaries, while their flattened convex arc has surface edges absent
  // from the source-join metadata. Selecting only one of these two sets
  // leaves either a detached terminal island or an exposed rounded shoulder.
  // Keep this surface policy separate from the original curve topology.
  const MINIMUM_TURN_COSINE: f64 = 0.984_807_753_012_208; // cos(10 degrees)
  (0..count)
    .map(|index| {
      let previous = (index + count - 1) % count;
      let next = (index + 1) % count;
      if !incoming_curves[index] && !incoming_curves[next] {
        return source_joins[index];
      }
      let incoming = (
        f64::from(points[index].0) - f64::from(points[previous].0),
        f64::from(points[index].1) - f64::from(points[previous].1),
      );
      let outgoing = (
        f64::from(points[next].0) - f64::from(points[index].0),
        f64::from(points[next].1) - f64::from(points[index].1),
      );
      let length_product = incoming.0.hypot(incoming.1) * outgoing.0.hypot(outgoing.1);
      if length_product <= f64::EPSILON || !length_product.is_finite() {
        return false;
      }
      let dot = incoming.0 * outgoing.0 + incoming.1 * outgoing.1;
      if dot > MINIMUM_TURN_COSINE * length_product {
        return false;
      }
      let turn = incoming.0 * outgoing.1 - incoming.1 * outgoing.0;
      let convex = if solid_on_right {
        turn > 0.0
      } else {
        turn < 0.0
      };
      source_joins[index] || (convex && incoming_curves[index] && incoming_curves[next])
    })
    .collect()
}

fn text_extrusion_contour_core_parameters(
  edges: &[(f32, bool)],
  joins: &[bool],
  width: f32,
) -> Vec<(f32, f32)> {
  debug_assert_eq!(edges.len(), joins.len());
  debug_assert!(width > f32::EPSILON);
  let count = edges.len();
  let mut cores = vec![(0.0, 1.0); count];
  if count == 0 {
    return cores;
  }
  let is_contour_edge =
    |index: usize| joins[index] || edges[index].1 != edges[(index + count - 1) % count].1;
  let Some(first) = (0..count).find(|&index| is_contour_edge(index)) else {
    return cores;
  };
  // Width is measured along the side surface, not independently on each
  // flattened chord. Carry the remaining half-width across subdivisions;
  // introducing a collinear vertex must not shorten an existing contour.
  // Two cyclic scans keep this linear even when the band spans many chords.
  let radius = width * 0.5;
  let mut distance = 0.0;
  for offset in 0..count {
    let index = (first + offset) % count;
    if is_contour_edge(index) {
      distance = 0.0;
    }
    let length = edges[index].0;
    if length > f32::EPSILON {
      cores[index].0 = ((radius - distance) / length).clamp(0.0, 1.0);
    }
    distance = (distance + length).min(radius);
  }
  distance = 0.0;
  for offset in 1..=count {
    let index = (first + count - offset) % count;
    if is_contour_edge((index + 1) % count) {
      distance = 0.0;
    }
    let length = edges[index].0;
    if length > f32::EPSILON {
      cores[index].1 = 1.0 - ((radius - distance) / length).clamp(0.0, 1.0);
    }
    distance = (distance + length).min(radius);
    let (start, end) = &mut cores[index];
    if *start > *end {
      let middle = (*start + *end) * 0.5;
      *start = middle;
      *end = middle;
    }
  }
  cores
}

fn text_extrusion_edge_point(first: (f32, f32), second: (f32, f32), parameter: f32) -> (f32, f32) {
  // Material boundaries subdivide the continuous submitted edge. Neither the
  // endpoints nor inserted color boundaries acquire a source-space grid.
  if parameter == 0.0 {
    return first;
  }
  if parameter == 1.0 {
    return second;
  }
  (
    first.0 + (second.0 - first.0) * parameter,
    first.1 + (second.1 - first.1) * parameter,
  )
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
    geometry_lighting,
    outline_contour,
  } = options;
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let width = model_surface.width_px.max(1.0);
  let height = model_surface.height_px.max(1.0);
  let project = |point: (f32, f32), z, color: [f32; 4]| {
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
      color,
      textured_material: None,
      bevel_collision: None,
    }
  };
  let mut triangles = Vec::new();
  let outline_contour =
    outline_contour.filter(|contour| contour.width_px > f32::EPSILON && contour.color.alpha != 0);

  for contour in &geometry.contours {
    debug_assert_eq!(
      contour.points.len(),
      contour.longitudinal_contour_joins.len()
    );
    // Office's native lossless text image keeps straight extrusion faces
    // uniformly shaded (the two diagonal sides of x are distinct examples).
    // Applying the Direct2D sample's join average to both ends of a straight
    // side instead spreads a corner's lighting across that entire face.
    // Retain the curve-side interpolation without changing planar normals.
    let edge_normals = text_3d_contour_edge_normals(
      &contour.points,
      &contour.incoming_curve_edges,
      geometry.solid_on_right,
      Text3dNormalJoinPolicy::ExtrusionSurfaces,
    );
    let visible_edges = contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
      .map(|(&first, &second)| {
        let edge = (second.0 - first.0, second.1 - first.1);
        let edge_length = edge.0.hypot(edge.1);
        if edge_length <= 1.0e-4 {
          return (edge_length, false);
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
        (
          edge_length,
          surface_faces_camera(
            projection,
            outward,
            model_point,
            width,
            height,
            pixels_per_point,
          ),
        )
      })
      .collect::<Vec<_>>();
    // Keep the non-text geometry policy unchanged. Text's longitudinal
    // contour is a separately submitted physical tube, so it must neither
    // replace the lit wall nor split its material into coplanar paint bands.
    let contour_cores = outline_contour
      .filter(|_| geometry_lighting != Static3dGeometryLighting::Text)
      .map(|outline| {
        text_extrusion_contour_core_parameters(
          &visible_edges,
          &contour.longitudinal_contour_joins,
          outline.width_px,
        )
      });
    for (index, (&first, &second)) in contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
      .enumerate()
    {
      let (edge_length, visible) = visible_edges[index];
      if edge_length <= 1.0e-4 || !visible {
        continue;
      }
      let endpoint_color = |point: (f32, f32), z: f32, normal: [f32; 2]| {
        let surface_normal =
          lighting_surface_normal(scene, projection, [normal[0], normal[1], 0.0]);
        let model_point = [point.0 - center_x, point.1 - center_y, z];
        let view_direction = surface_view_direction(
          scene,
          projection,
          model_point,
          width,
          height,
          pixels_per_point,
        );
        let shade = if geometry_lighting == Static3dGeometryLighting::Text {
          material_diffuse_shade(scene, surface_normal, view_direction, material)
        } else {
          legacy_material_diffuse_shade(scene, surface_normal, material)
        };
        let specular = if geometry_lighting == Static3dGeometryLighting::Text {
          light_rig_surface_specular(
            scene,
            surface_normal,
            view_direction,
            material,
            [tint.color.r, tint.color.g, tint.color.b],
          )
        } else {
          legacy_light_rig_surface_specular(scene, surface_normal, view_direction, material)
        };
        let alpha = material_surface_alpha(
          f32::from(tint.alpha) / 255.0,
          surface_normal,
          view_direction,
          material,
        );
        shaded_geometry_pixel_with_specular(tint, shade, specular, alpha, geometry_lighting)
      };
      let (start_normal, end_normal) = edge_normals[index];
      let front_start_color = endpoint_color(first, front_z, start_normal)
        .0
        .map(f32::from);
      let front_end_color = endpoint_color(second, front_z, end_normal).0.map(f32::from);
      let back_start_color = endpoint_color(first, back_z, start_normal).0.map(f32::from);
      let back_end_color = endpoint_color(second, back_z, end_normal).0.map(f32::from);
      let point_at = |parameter: f32| text_extrusion_edge_point(first, second, parameter);
      let color_at = |start: [f32; 4], end: [f32; 4], parameter: f32| {
        std::array::from_fn(|channel| start[channel] + (end[channel] - start[channel]) * parameter)
      };
      let contour_vertex_color = outline_contour.map(|contour| {
        [
          f32::from(contour.color.color.r),
          f32::from(contour.color.color.g),
          f32::from(contour.color.color.b),
          f32::from(contour.color.alpha),
        ]
      });
      let mut emit_segment = |start: f32, end: f32, color: Option<[f32; 4]>| {
        if end - start <= f32::EPSILON {
          return;
        }
        let start_point = point_at(start);
        let end_point = point_at(end);
        let front_start = project(
          start_point,
          front_z,
          color.unwrap_or_else(|| color_at(front_start_color, front_end_color, start)),
        );
        let front_end = project(
          end_point,
          front_z,
          color.unwrap_or_else(|| color_at(front_start_color, front_end_color, end)),
        );
        let back_end = project(
          end_point,
          back_z,
          color.unwrap_or_else(|| color_at(back_start_color, back_end_color, end)),
        );
        let back_start = project(
          start_point,
          back_z,
          color.unwrap_or_else(|| color_at(back_start_color, back_end_color, start)),
        );
        triangles.push(TextSurfaceTriangle {
          vertices: [front_start, front_end, back_end],
          bevel_source: None,
        });
        triangles.push(TextSurfaceTriangle {
          vertices: [back_end, back_start, front_start],
          bevel_source: None,
        });
      };

      let Some(contour_cores) = &contour_cores else {
        emit_segment(0.0, 1.0, None);
        continue;
      };
      // Retained non-text realization: partition the side quad without
      // coplanar overlap. Text uses physical contour tubes below instead.
      let (extrusion_start, extrusion_end) = contour_cores[index];
      emit_segment(0.0, extrusion_start, contour_vertex_color);
      emit_segment(extrusion_start, extrusion_end, None);
      emit_segment(extrusion_end, 1.0, contour_vertex_color);
    }
    if let Some(outline) =
      outline_contour.filter(|_| geometry_lighting == Static3dGeometryLighting::Text)
    {
      let color = [
        f32::from(outline.color.color.r),
        f32::from(outline.color.color.g),
        f32::from(outline.color.color.b),
        f32::from(outline.color.alpha),
      ];
      triangles.extend(
        contour_mesh::longitudinal_triangles(
          &contour.points,
          &visible_edges,
          outline.width_px * 0.5,
          front_z,
          back_z,
        )
        .into_iter()
        .map(|vertices| TextSurfaceTriangle {
          vertices: vertices.map(|[x, y, z]| project((x, y), z, color)),
          bevel_source: None,
        }),
      );
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
    geometry_lighting: _,
    outline_contour: _,
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

fn shaded_fixed_gouraud_pixel_with_specular(
  color: Static3dColor,
  shade: [f32; 3],
  specular: [f32; 3],
  alpha: u8,
) -> Rgba<u8> {
  Rgba([
    shade_fixed_gouraud_channel_with_specular(color.color.r, shade[0], specular[0]),
    shade_fixed_gouraud_channel_with_specular(color.color.g, shade[1], specular[1]),
    shade_fixed_gouraud_channel_with_specular(color.color.b, shade[2], specular[2]),
    alpha,
  ])
}

fn shaded_geometry_pixel_with_specular(
  color: Static3dColor,
  shade: [f32; 3],
  specular: [f32; 3],
  alpha: u8,
  geometry_lighting: Static3dGeometryLighting,
) -> Rgba<u8> {
  if geometry_lighting == Static3dGeometryLighting::Shape {
    shaded_fixed_gouraud_pixel_with_specular(color, shade, specular, alpha)
  } else {
    shaded_pixel_with_specular(color, shade, specular, alpha)
  }
}

fn shade_fixed_gouraud_channel_with_specular(channel: u8, shade: f32, specular: f32) -> u8 {
  // Word's shape extrusion and bevel triangles expose the early D3D9
  // fixed-point color path. Controlled Screen exports over adjacent material
  // values pin its exact device lattice: truncate the normalized lit value to
  // 1/128, then convert that fixed-point value to 8-bit with round-to-nearest.
  // Text surfaces retain the higher-precision path above.
  quantize_gouraud_channel(f32::from(channel) / 255.0 * shade + specular)
}

fn quantize_gouraud_channel(normalized: f32) -> u8 {
  const FIXED_SCALE: u16 = 128;
  let fixed = (normalized.clamp(0.0, 1.0) * f32::from(FIXED_SCALE)).floor() as u16;
  ((fixed * 255 + FIXED_SCALE / 2) / FIXED_SCALE) as u8
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
  width: f32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  scene: &'a a::Scene3DType,
  projection: Static3dProjection,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  surface_z: f32,
  material: Option<a::PresetMaterialTypeValues>,
  back_face: bool,
  height_direction: f32,
}

#[derive(Clone, Copy)]
struct TextBevelOptions<'a> {
  geometry_width: f32,
  normal_width: f32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  scene: &'a a::Scene3DType,
  projection: Static3dProjection,
  model_surface: Static3dSurface,
  pixels_per_point: f32,
  surface_z: f32,
  material: Option<a::PresetMaterialTypeValues>,
  geometry_lighting: Static3dGeometryLighting,
  back_face: bool,
  height_direction: f32,
  fill_material: Option<&'a RgbaImage>,
  outline_material: Option<&'a RgbaImage>,
  outline_coverage: Option<&'a RgbaImage>,
  fill_uniform_paint_opacity: Option<f32>,
  outline_uniform_paint_opacity: Option<f32>,
  outline_material_inset_px: Option<f32>,
  fill_has_authored_transparency: bool,
  outline_has_authored_transparency: bool,
}

#[derive(Clone, Copy, Debug)]
struct TextSurfaceVertex {
  point: (f32, f32),
  visibility_depth: f32,
  color: [f32; 4],
  textured_material: Option<TextSurfaceMaterialVertex>,
  bevel_collision: Option<TextBevelCollisionVertex>,
}

/// Source-independent attributes carried by Word's textured 3-D text mesh.
///
/// Direct3D interpolates the light colors and texture coordinates as separate
/// vertex data. The texture cascade then multiplies the sampled Shape color
/// by the diffuse term and adds the specular color afterwards. Keeping these
/// values separate avoids baking a texture lookup into each vertex before
/// Gouraud interpolation, which is observably different at glyph joins and
/// along curved bevel strips.
#[derive(Clone, Copy, Debug)]
struct TextSurfaceMaterialVertex {
  source: TextSurfaceMaterialSource,
  material_point: (f32, f32),
  diffuse: [f32; 3],
  specular_incident: [f32; 3],
  /// Diffuse material alpha, excluding paint opacity already in the RGBA
  /// texture. Interpolate this alongside the lighting, not glyph coverage.
  material_opacity: f32,
  /// Fraction of the already-composited material opacity contributed by a
  /// paint whose authored transparency selects Word's alpha-bearing glyph
  /// texture route.
  alpha_texture_fraction: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextBevelCollisionVertex {
  source_point: (f32, f32),
  inset_px: f32,
  surface_distance_px: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextBevelMaterialEndpoint {
  source: TextSurfaceMaterialSource,
  source_point: (f32, f32),
  material_point: (f32, f32),
  outward: [f32; 2],
  source_color: [u8; 3],
  paint_opacity: f32,
  /// Absolute premultiplied opacity contribution which must be modulated by
  /// the curve-preserving glyph alpha texture.
  alpha_texture_opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TextSurfaceMaterialSource {
  Combined,
  Independent,
}

#[derive(Clone, Copy)]
struct TextSurfaceMaterialImages<'a> {
  combined: &'a RgbaImage,
  surface_material_texture: Option<&'a TextMaterialTexture>,
  independent_to_combined: Transform,
  independent_alpha: Option<&'a TextGeometryAreaMask>,
}

#[derive(Clone, Copy, Debug)]
struct ShadedTextBevelMaterialEndpoint {
  color: [u8; 4],
  textured_material: Option<TextSurfaceMaterialVertex>,
}

#[derive(Clone, Copy)]
struct TextBevelMaterialContext<'a> {
  geometry: &'a Static3dTextGeometry,
  options: TextBevelOptions<'a>,
}

fn shade_text_bevel_material_endpoint(
  context: TextBevelMaterialContext<'_>,
  endpoint: TextBevelMaterialEndpoint,
  profile: BevelProfileSample,
) -> ShadedTextBevelMaterialEndpoint {
  let TextBevelOptions {
    normal_width,
    height,
    scene,
    projection,
    model_surface,
    pixels_per_point,
    surface_z,
    material,
    geometry_lighting,
    back_face,
    height_direction,
    ..
  } = context.options;
  let normal_xy = height * profile.height_tangent;
  let normal_z = normal_width * profile.inset_tangent * if back_face { -1.0 } else { 1.0 };
  let normal = transformed_bevel_surface_normal(
    endpoint.outward,
    normal_xy,
    normal_z,
    context.geometry.page_plane_scale_x,
    context.geometry.page_plane_scale_y,
    pixels_per_point,
  );
  let normal = lighting_surface_normal(scene, projection, normal);
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let model_width = model_surface.width_px.max(1.0);
  let model_height = model_surface.height_px.max(1.0);
  let model_point = [
    endpoint.source_point.0 - center_x,
    endpoint.source_point.1 - center_y,
    surface_z + profile.height * height * height_direction,
  ];
  let view_direction = surface_view_direction(
    scene,
    projection,
    model_point,
    model_width,
    model_height,
    pixels_per_point,
  );
  let (specular, shade, textured_material) = if geometry_lighting == Static3dGeometryLighting::Text
  {
    let specular_incident =
      light_rig_surface_specular_incident(scene, normal, view_direction, material);
    let shade = material_diffuse_shade(scene, normal, view_direction, material);
    let vertex_diffuse = text_surface_diffuse_attribute(shade);
    (
      word_text_surface_specular(specular_incident, material, endpoint.source_color),
      shade,
      Some(TextSurfaceMaterialVertex {
        source: endpoint.source,
        material_point: endpoint.material_point,
        diffuse: vertex_diffuse,
        specular_incident,
        material_opacity: f32::from(material_surface_alpha(
          1.0,
          normal,
          view_direction,
          material,
        )) / 255.0,
        alpha_texture_fraction: if endpoint.paint_opacity > f32::EPSILON {
          (endpoint.alpha_texture_opacity / endpoint.paint_opacity).clamp(0.0, 1.0)
        } else {
          0.0
        },
      }),
    )
  } else {
    (
      legacy_light_rig_surface_specular(scene, normal, view_direction, material),
      legacy_material_diffuse_shade(scene, normal, material),
      None,
    )
  };
  let alpha = material_surface_alpha(endpoint.paint_opacity, normal, view_direction, material);
  let color = shaded_geometry_pixel_with_specular(
    Static3dColor {
      color: RgbColor {
        r: endpoint.source_color[0],
        g: endpoint.source_color[1],
        b: endpoint.source_color[2],
      },
      alpha,
    },
    shade,
    specular,
    alpha,
    geometry_lighting,
  )
  .0;
  ShadedTextBevelMaterialEndpoint {
    color,
    textured_material,
  }
}

fn text_surface_diffuse_attribute(shade: [f32; 3]) -> [f32; 3] {
  // Word's textured-solid vertex stream stores diffuse light in a packed
  // byte with 128 representing unity. The bound pixel constant multiplies
  // RGB by 255/128 after UNORM decoding, preserving illumination above one.
  // Clipping at one before interpolation loses that light even when the
  // sampled paint is dark enough that the final output would not saturate.
  // Keep this range contract separate from vertex packing/quantization.
  const MAX_DIFFUSE: f32 = 255.0 / 128.0;
  shade.map(|component| component.clamp(0.0, MAX_DIFFUSE))
}

fn interpolate_text_surface_material(
  first: TextSurfaceMaterialVertex,
  second: TextSurfaceMaterialVertex,
  parameter: f32,
) -> TextSurfaceMaterialVertex {
  debug_assert_eq!(first.source, second.source);
  TextSurfaceMaterialVertex {
    source: first.source,
    material_point: (
      first.material_point.0 + (second.material_point.0 - first.material_point.0) * parameter,
      first.material_point.1 + (second.material_point.1 - first.material_point.1) * parameter,
    ),
    diffuse: std::array::from_fn(|channel| {
      first.diffuse[channel] + (second.diffuse[channel] - first.diffuse[channel]) * parameter
    }),
    specular_incident: std::array::from_fn(|channel| {
      first.specular_incident[channel]
        + (second.specular_incident[channel] - first.specular_incident[channel]) * parameter
    }),
    material_opacity: first.material_opacity
      + (second.material_opacity - first.material_opacity) * parameter,
    alpha_texture_fraction: first.alpha_texture_fraction
      + (second.alpha_texture_fraction - first.alpha_texture_fraction) * parameter,
  }
}

fn resolve_text_surface_material_color(
  images: TextSurfaceMaterialImages<'_>,
  alpha_texture: Option<&TextGeometryAreaMask>,
  material: Option<a::PresetMaterialTypeValues>,
  vertex: TextSurfaceMaterialVertex,
  alpha: f32,
) -> Option<[f32; 4]> {
  // The texture stage samples the composed paint, not a second composition
  // of interpolated endpoint paint weights. Endpoint colors/opacity and
  // coverage remain independent vertex attributes. In particular their
  // page-space alpha coordinates must not replace the combined texture's
  // GDI+ device coordinates.
  let color_point = match vertex.source {
    TextSurfaceMaterialSource::Combined => vertex.material_point,
    TextSurfaceMaterialSource::Independent => {
      let transform = images.independent_to_combined;
      (
        vertex.material_point.0 * transform.sx + transform.tx,
        vertex.material_point.1 * transform.sy + transform.ty,
      )
    }
  };
  if let Some(texture) = images.surface_material_texture {
    return Some(shade_associated_text_material(
      texture.sample_associated(color_point),
      vertex,
      material,
    ));
  }
  let pixel = sample_bilinear_clamped(images.combined, color_point.0 - 0.5, color_point.1 - 0.5)?;
  let source_color = [pixel[0], pixel[1], pixel[2]];
  let alpha_texture = match vertex.source {
    TextSurfaceMaterialSource::Combined => alpha_texture,
    TextSurfaceMaterialSource::Independent => images.independent_alpha,
  };
  let specular = word_text_surface_specular(vertex.specular_incident, material, source_color);
  let alpha = alpha_texture.map_or(alpha, |texture| {
    let texture_alpha = sample_text_geometry_area_mask(
      texture,
      vertex.material_point.0 - 0.5,
      vertex.material_point.1 - 0.5,
    );
    alpha * (1.0 - vertex.alpha_texture_fraction.clamp(0.0, 1.0) * (1.0 - texture_alpha))
  });
  Some([
    f32::from(shade_gouraud_channel_with_specular(
      source_color[0],
      vertex.diffuse[0],
      specular[0],
    )),
    f32::from(shade_gouraud_channel_with_specular(
      source_color[1],
      vertex.diffuse[1],
      specular[1],
    )),
    f32::from(shade_gouraud_channel_with_specular(
      source_color[2],
      vertex.diffuse[2],
      specular[2],
    )),
    alpha,
  ])
}

/// Execute the alpha-bearing textured material stage before SourceOver.
///
/// Word's captured textured-solid shader samples premultiplied RGBA, multiplies
/// diffuse color, adds specular, and clamps RGB to the resulting alpha. Paint
/// opacity must not be replaced by another glyph mask or applied a second time
/// through vertex alpha. Return straight floating-point RGBA only at the sample
/// storage boundary; antialias resolve owns the final byte quantization.
fn shade_associated_text_material(
  texture: [f32; 4],
  vertex: TextSurfaceMaterialVertex,
  material: Option<a::PresetMaterialTypeValues>,
) -> [f32; 4] {
  let alpha = texture[3] * vertex.material_opacity.clamp(0.0, 1.0);
  // The shader discards alpha below one byte. Callers must not write depth
  // for this result, nor turn it into a missing-texture fallback.
  if alpha < 1.0 {
    return [0.0; 4];
  }
  let shape_color = std::array::from_fn(|channel| {
    (texture[channel] * 255.0 / texture[3])
      .round()
      .clamp(0.0, 255.0) as u8
  });
  let specular = word_text_surface_specular(vertex.specular_incident, material, shape_color);
  let mut color = [0.0; 4];
  for channel in 0..3 {
    let associated = texture[channel] * vertex.diffuse[channel] + 255.0 * specular[channel];
    color[channel] = associated.clamp(0.0, alpha) * 255.0 / alpha;
  }
  color[3] = alpha;
  color
}

/// Composite one admitted material pass over the previously visible surface.
///
/// Surface samples store straight RGBA in byte units. Keep alpha unrounded
/// while evaluating the premultiplied SourceOver equation; resolving the
/// multisample image is a separate operation.
fn text_material_surface_over(foreground: [f32; 4], background: [f32; 4]) -> [f32; 4] {
  if foreground[3] >= 255.0 || background[3] <= 0.0 {
    return foreground;
  }
  if foreground[3] <= 0.0 {
    return background;
  }
  let background_weight = background[3] * (1.0 - foreground[3] / 255.0);
  let alpha = foreground[3] + background_weight;
  let mut color = [0.0; 4];
  for channel in 0..3 {
    color[channel] =
      (foreground[channel] * foreground[3] + background[channel] * background_weight) / alpha;
  }
  color[3] = alpha;
  color
}

fn composite_text_bevel_material_endpoint(
  base: Option<TextBevelMaterialEndpoint>,
  overlay: Option<(TextBevelMaterialEndpoint, f32)>,
) -> Option<TextBevelMaterialEndpoint> {
  let Some((overlay, coverage)) = overlay else {
    return base;
  };
  let coverage = coverage.clamp(0.0, 1.0);
  let overlay_alpha = (overlay.paint_opacity * coverage).clamp(0.0, 1.0);
  let overlay_texture_alpha = (overlay.alpha_texture_opacity * coverage).clamp(0.0, overlay_alpha);
  if overlay_alpha <= f32::EPSILON {
    return base;
  }
  let Some(base) = base else {
    return Some(TextBevelMaterialEndpoint {
      paint_opacity: overlay_alpha,
      alpha_texture_opacity: overlay_texture_alpha,
      ..overlay
    });
  };
  let base_alpha = base.paint_opacity.clamp(0.0, 1.0);
  debug_assert_eq!(base.source, overlay.source);
  let base_texture_alpha = base.alpha_texture_opacity.clamp(0.0, base_alpha);
  let base_weight = 1.0 - overlay_alpha;
  let output_alpha = overlay_alpha + base_alpha * base_weight;
  if output_alpha <= f32::EPSILON {
    return None;
  }
  let source_color = std::array::from_fn(|channel| {
    ((f32::from(overlay.source_color[channel]) * overlay_alpha
      + f32::from(base.source_color[channel]) * base_alpha * base_weight)
      / output_alpha)
      .round()
      .clamp(0.0, 255.0) as u8
  });
  Some(TextBevelMaterialEndpoint {
    source_color,
    paint_opacity: output_alpha,
    alpha_texture_opacity: (overlay_texture_alpha + base_texture_alpha * base_weight)
      .clamp(0.0, output_alpha),
    ..base
  })
}

#[derive(Clone, Copy, Debug)]
struct TextSurfaceTriangle {
  vertices: [TextSurfaceVertex; 3],
  bevel_source: Option<TextBevelTriangleSource>,
}

#[derive(Clone, Copy, Debug)]
struct TextBevelTriangleSource {
  back_face: bool,
  contour_index: u32,
  edge_index: u32,
  profile_strip_index: u16,
  second_half: bool,
  direct_inset_graph: bool,
}

#[derive(Clone, Copy)]
struct TextSurfaceRasterSample {
  visibility_depth: f32,
  color: [f32; 4],
  covered: bool,
  tie_priority: u8,
}

impl Default for TextSurfaceRasterSample {
  fn default() -> Self {
    Self {
      visibility_depth: f32::NEG_INFINITY,
      color: [0.0; 4],
      covered: false,
      tie_priority: 0,
    }
  }
}

const TEXT_SURFACE_PLANAR_TIE_PRIORITY: u8 = 0;
const TEXT_SURFACE_CONTOUR_TIE_PRIORITY: u8 = 1;
const TEXT_SURFACE_TRIANGLE_TIE_PRIORITY: u8 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextSurfaceSamplePattern {
  OfficeAntiAlias8x4,
  Direct3dStandard8,
}

impl TextSurfaceSamplePattern {
  /// Word's observed MSAA centroid selection: a fully covered pixel uses its
  /// center; partial coverage uses the first covered standard sample. Coverage
  /// and depth stay per sample, but all shader inputs use this one location.
  fn centroid(self, coverage: u32) -> Option<(f32, f32)> {
    let full = u32::MAX >> (32 - self.count());
    let coverage = coverage & full;
    if coverage == 0 {
      None
    } else if coverage == full {
      Some((0.5, 0.5))
    } else {
      Some(self.position(coverage.trailing_zeros() as usize))
    }
  }

  fn count(self) -> usize {
    match self {
      Self::OfficeAntiAlias8x4 => 32,
      Self::Direct3dStandard8 => 8,
    }
  }

  fn position(self, sample_index: usize) -> (f32, f32) {
    match self {
      Self::OfficeAntiAlias8x4 => {
        const GRID_X: usize = 8;
        const GRID_Y: usize = 4;
        let sample_x = sample_index % GRID_X;
        let sample_y = sample_index / GRID_X;
        (
          (sample_x as f32 + 0.5) / GRID_X as f32,
          (sample_y as f32 + 0.5) / GRID_Y as f32,
        )
      }
      Self::Direct3dStandard8 => {
        // Standard eight-sample D3D/Vulkan positions on the 1/16-pixel grid.
        // The exact 48-cell phase matrix selects both these coordinates and
        // their x/y pairing; the independent 256-cell physical-surface slice
        // keeps them uniquely best across glyph, camera, depth, bevel, and
        // contour controls.
        const POSITIONS: [(f32, f32); 8] = [
          (9.0 / 16.0, 5.0 / 16.0),
          (7.0 / 16.0, 11.0 / 16.0),
          (13.0 / 16.0, 9.0 / 16.0),
          (5.0 / 16.0, 3.0 / 16.0),
          (3.0 / 16.0, 13.0 / 16.0),
          (1.0 / 16.0, 7.0 / 16.0),
          (11.0 / 16.0, 15.0 / 16.0),
          (15.0 / 16.0, 1.0 / 16.0),
        ];
        POSITIONS[sample_index]
      }
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TextSurfaceRasterization {
  SourceGrid,
  FinalGrid(Static3dTextFinalGrid),
}

impl TextSurfaceRasterization {
  fn sample_pattern(self) -> TextSurfaceSamplePattern {
    match self {
      Self::SourceGrid => TextSurfaceSamplePattern::OfficeAntiAlias8x4,
      Self::FinalGrid(_) => TextSurfaceSamplePattern::Direct3dStandard8,
    }
  }

  fn surface_to_raster(self, point: (f32, f32), dimensions: (u32, u32)) -> (f32, f32) {
    match self {
      Self::SourceGrid => point,
      Self::FinalGrid(target) => (
        (point.0 - target.source_left_px) * dimensions.0 as f32 / target.source_width_px,
        (point.1 - target.source_top_px) * dimensions.1 as f32 / target.source_height_px,
      ),
    }
  }

  fn raster_to_surface(self, point: (f32, f32), dimensions: (u32, u32)) -> (f32, f32) {
    match self {
      Self::SourceGrid => point,
      Self::FinalGrid(target) => (
        target.source_left_px + point.0 * target.source_width_px / dimensions.0 as f32,
        target.source_top_px + point.1 * target.source_height_px / dimensions.1 as f32,
      ),
    }
  }
}

fn text_surface_quad_projection_key(quad: &[TextSurfaceTriangle]) -> [u64; 4] {
  let mut vertices = SmallVec::<[u64; 4]>::new();
  for vertex in quad.iter().flat_map(|triangle| triangle.vertices) {
    let x = if vertex.point.0 == 0.0 {
      0
    } else {
      vertex.point.0.to_bits()
    };
    let y = if vertex.point.1 == 0.0 {
      0
    } else {
      vertex.point.1.to_bits()
    };
    let point = (u64::from(x) << 32) | u64::from(y);
    if !vertices.contains(&point) {
      vertices.push(point);
    }
  }
  vertices.sort_unstable();
  let mut key = [u64::MAX; 4];
  for (target, point) in key.iter_mut().zip(vertices) {
    *target = point;
  }
  key
}

fn text_surface_quad_fully_occludes(
  front: &[TextSurfaceTriangle],
  back: &[TextSurfaceTriangle],
) -> bool {
  let mut strictly_nearer = false;
  for back_vertex in back.iter().flat_map(|triangle| triangle.vertices) {
    let matching_depth = front
      .iter()
      .flat_map(|triangle| triangle.vertices)
      .filter(|front_vertex| {
        front_vertex.point.0.to_bits() == back_vertex.point.0.to_bits()
          && front_vertex.point.1.to_bits() == back_vertex.point.1.to_bits()
      })
      .map(|front_vertex| front_vertex.visibility_depth)
      .max_by(f32::total_cmp);
    let Some(matching_depth) = matching_depth else {
      return false;
    };
    if matching_depth + 1.0e-6 < back_vertex.visibility_depth {
      return false;
    }
    strictly_nearer |= matching_depth > back_vertex.visibility_depth + 1.0e-6;
  }
  strictly_nearer
}

fn fully_occluded_text_surface_triangles(triangles: &[TextSurfaceTriangle]) -> Vec<bool> {
  let quads = triangles.chunks_exact(2).collect::<Vec<_>>();
  let mut projection_groups = HashMap::<[u64; 4], SmallVec<[usize; 2]>>::new();
  for (index, quad) in quads.iter().enumerate() {
    projection_groups
      .entry(text_surface_quad_projection_key(quad))
      .or_default()
      .push(index);
  }
  let mut occluded = vec![false; triangles.len()];
  for group in projection_groups.values() {
    for &back_quad in group {
      let quad_is_occluded = group.iter().copied().any(|front_quad| {
        front_quad != back_quad
          && text_surface_quad_fully_occludes(quads[front_quad], quads[back_quad])
      });
      if quad_is_occluded {
        occluded[back_quad * 2] = true;
        occluded[back_quad * 2 + 1] = true;
      }
    }
  }
  occluded
}

fn translate_text_surface_coverage_masks(
  masks: &[u32],
  width: usize,
  height: usize,
  translation: (f32, f32),
) -> Vec<u32> {
  const SAMPLE_GRID_X: i32 = 8;
  const SAMPLE_GRID_Y: i32 = 4;
  let translation_x = (translation.0 * SAMPLE_GRID_X as f32).round() as i32;
  let translation_y = (translation.1 * SAMPLE_GRID_Y as f32).round() as i32;
  debug_assert!((translation.0 * SAMPLE_GRID_X as f32 - translation_x as f32).abs() < 1.0e-4);
  debug_assert!((translation.1 * SAMPLE_GRID_Y as f32 - translation_y as f32).abs() < 1.0e-4);
  if translation_x == 0 && translation_y == 0 {
    return masks.to_vec();
  }

  let mut translated = vec![0_u32; masks.len()];
  for pixel_y in 0..height as i32 {
    for pixel_x in 0..width as i32 {
      let mask = masks[pixel_y as usize * width + pixel_x as usize];
      if mask == 0 {
        continue;
      }
      for sample_y in 0..SAMPLE_GRID_Y {
        let row = ((mask >> (sample_y * SAMPLE_GRID_X)) & 0xff) as u16;
        if row == 0 {
          continue;
        }
        let target_sample_y = pixel_y * SAMPLE_GRID_Y + sample_y + translation_y;
        let target_pixel_y = target_sample_y.div_euclid(SAMPLE_GRID_Y);
        if target_pixel_y < 0 || target_pixel_y >= height as i32 {
          continue;
        }
        let target_row = target_sample_y.rem_euclid(SAMPLE_GRID_Y) as u32;
        let target_sample_x = pixel_x * SAMPLE_GRID_X + translation_x;
        let target_pixel_x = target_sample_x.div_euclid(SAMPLE_GRID_X);
        let target_column = target_sample_x.rem_euclid(SAMPLE_GRID_X) as u32;
        let shifted = row << target_column;
        for (pixel_offset, byte) in [(0, shifted as u8), (1, (shifted >> 8) as u8)] {
          let target_x = target_pixel_x + pixel_offset;
          if byte == 0 || target_x < 0 || target_x >= width as i32 {
            continue;
          }
          translated[target_pixel_y as usize * width + target_x as usize] |=
            u32::from(byte) << (target_row * SAMPLE_GRID_X as u32);
        }
      }
    }
  }
  translated
}

fn text_surface_edge(a: (f32, f32), b: (f32, f32), point: (f32, f32)) -> f32 {
  (b.0 - a.0) * (point.1 - a.1) - (b.1 - a.1) * (point.0 - a.0)
}

/// Whether an on-edge sample belongs to this directed triangle edge under
/// Direct3D's top-left fill convention.
///
/// Screen-space y increases downwards. For a positive-area triangle, an edge
/// is inclusive when it travels upwards, or when it is horizontal and travels
/// rightwards; reversing the triangle reverses both tests. This makes a shared
/// edge belong to exactly one adjacent triangle while retaining the outer top
/// and left boundaries. Direct3D applies the same rule independently at every
/// MSAA sample location.
fn text_surface_edge_is_top_left(first: (f32, f32), second: (f32, f32), signed_area: f32) -> bool {
  let delta_x = second.0 - first.0;
  let delta_y = second.1 - first.1;
  if signed_area > 0.0 {
    delta_y < 0.0 || (delta_y == 0.0 && delta_x > 0.0)
  } else {
    delta_y > 0.0 || (delta_y == 0.0 && delta_x < 0.0)
  }
}

fn text_surface_triangle_covers_sample(
  first: (f32, f32),
  second: (f32, f32),
  third: (f32, f32),
  point: (f32, f32),
  signed_area: f32,
) -> bool {
  let edges = [
    (second, third, text_surface_edge(second, third, point)),
    (third, first, text_surface_edge(third, first, point)),
    (first, second, text_surface_edge(first, second, point)),
  ];
  edges.into_iter().all(|(edge_first, edge_second, value)| {
    let strictly_inside = if signed_area > 0.0 {
      value > 0.0
    } else {
      value < 0.0
    };
    strictly_inside
      || (value == 0.0 && text_surface_edge_is_top_left(edge_first, edge_second, signed_area))
  })
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

fn perspective_correct_text_surface_weights(
  parallel: bool,
  reciprocal_w: [f32; 3],
  screen_weights: [f32; 3],
) -> Option<[f32; 3]> {
  if parallel {
    return Some(screen_weights);
  }
  let denominator = reciprocal_w
    .into_iter()
    .zip(screen_weights)
    .map(|(depth, weight)| depth * weight)
    .sum::<f32>();
  (denominator > f32::EPSILON)
    .then(|| std::array::from_fn(|index| reciprocal_w[index] * screen_weights[index] / denominator))
}

fn text_geometry_contains(geometry: &Static3dTextGeometry, point: (f32, f32)) -> bool {
  let mut winding = 0_i32;
  for contour in &geometry.contours {
    let (left, top, right, bottom) = contour.bounds;
    // Every contour is closed. A point outside its axis-aligned bounds has a
    // zero winding contribution, so reject it before walking all glyph edges.
    // Long WordArt runs otherwise rescan every other glyph for each 8x4
    // antialias sample.
    if point.0 < left || point.0 > right || point.1 < top || point.1 > bottom {
      continue;
    }
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

fn point_segment_distance(point: (f32, f32), first: (f32, f32), second: (f32, f32)) -> f32 {
  let edge = (second.0 - first.0, second.1 - first.1);
  let length_squared = edge.0 * edge.0 + edge.1 * edge.1;
  if length_squared <= f32::EPSILON {
    return (point.0 - first.0).hypot(point.1 - first.1);
  }
  let projection = ((point.0 - first.0) * edge.0 + (point.1 - first.1) * edge.1) / length_squared;
  let projection = projection.clamp(0.0, 1.0);
  let nearest = (first.0 + edge.0 * projection, first.1 + edge.1 * projection);
  (point.0 - nearest.0).hypot(point.1 - nearest.1)
}

fn text_geometry_boundary_distance(geometry: &Static3dTextGeometry, point: (f32, f32)) -> f32 {
  let bounds_distance_squared = |contour: &Static3dTextContour| {
    let (left, top, right, bottom) = contour.bounds;
    let delta_x = if point.0 < left {
      left - point.0
    } else if point.0 > right {
      point.0 - right
    } else {
      0.0
    };
    let delta_y = if point.1 < top {
      top - point.1
    } else if point.1 > bottom {
      point.1 - bottom
    } else {
      0.0
    };
    delta_x * delta_x + delta_y * delta_y
  };
  let contour_distance = |contour: &Static3dTextContour| {
    contour
      .points
      .iter()
      .copied()
      .zip(contour.points.iter().copied().cycle().skip(1))
      .take(contour.points.len())
      .map(|(first, second)| point_segment_distance(point, first, second))
      .min_by(f32::total_cmp)
      .unwrap_or(f32::INFINITY)
  };

  // A long WordArt run can contain thousands of flattened glyph edges. This
  // query runs for every 8x4 surface sample, but an inset point can only be
  // owned by its glyph and immediately adjacent counters. Seed the exact
  // distance with the nearest contour AABB, then reject contours whose AABB
  // lies wholly beyond that upper bound. The segment-distance calculation and
  // final minimum remain unchanged; the boxes only remove candidates that
  // cannot geometrically win.
  let Some((seed_index, seed)) =
    geometry
      .contours
      .iter()
      .enumerate()
      .min_by(|(_, left), (_, right)| {
        bounds_distance_squared(left).total_cmp(&bounds_distance_squared(right))
      })
  else {
    return f32::INFINITY;
  };
  let mut minimum = contour_distance(seed);
  for (index, contour) in geometry.contours.iter().enumerate() {
    if index == seed_index {
      continue;
    }
    let (left, top, right, bottom) = contour.bounds;
    if point.0 + minimum < left
      || point.0 - minimum > right
      || point.1 + minimum < top
      || point.1 - minimum > bottom
    {
      continue;
    }
    minimum = minimum.min(contour_distance(contour));
  }
  minimum
}

#[derive(Clone, Copy, Debug)]
struct TextGeometryBoundaryOwner {
  distance: f32,
  contour_index: usize,
  edge_index: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct TextDirectInsetTopologySample {
  owner: Option<TextGeometryBoundaryOwner>,
  source_point: Option<(f32, f32)>,
}

impl TextDirectInsetTopologySample {
  fn consider(&mut self, owner: TextGeometryBoundaryOwner, source_point: (f32, f32)) {
    // The inset graph is resolved before its 3-D attributes. When two swept
    // contour cells cover the same source-plane sample, the first wavefront
    // intersection (the smaller inset coordinate) owns the graph vertex.
    // Comparing camera depth here instead lets an inner counter overwrite an
    // outer glyph connection merely because its raw, already-crossed ring is
    // higher. Profile branches at the selected coordinate are resolved by
    // `text_direct_inset_surface_sample` after ownership is fixed.
    if self
      .owner
      .is_none_or(|current| owner.distance + 1.0e-5 < current.distance)
    {
      self.owner = Some(owner);
      self.source_point = Some(source_point);
    }
  }
}

#[derive(Clone, Copy)]
struct TextDirectInsetGraphOwner {
  boundary: TextGeometryBoundaryOwner,
  source_point: (f32, f32),
  profile_strip_index: usize,
  surface_distance_px: f32,
}

#[derive(Clone, Copy, Default)]
struct TextDirectInsetGraphSample {
  surface: TextSurfaceRasterSample,
  owner: Option<TextDirectInsetGraphOwner>,
}

impl TextDirectInsetGraphSample {
  fn consider(&mut self, visibility_depth: f32, color: [f32; 4], owner: TextDirectInsetGraphOwner) {
    // The source-plane inset graph has already resolved merges and splits.
    // Its projected triangles may overlap at DIFFERENT source points, even
    // on a single smooth edge. Select their camera-visible surface, keeping
    // depth and all source attributes from the same hit. Selecting minimum
    // inset independently can replace a near Circle limb with its far limb
    // and incorrectly expose a contour that the actual graph occludes.
    if !self.surface.covered || visibility_depth > self.surface.visibility_depth {
      self.surface = TextSurfaceRasterSample {
        visibility_depth,
        color,
        covered: true,
        tie_priority: TEXT_SURFACE_TRIANGLE_TIE_PRIORITY,
      };
      self.owner = Some(owner);
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct TextGeometryBoundaryEdge {
  contour_index: usize,
  edge_index: usize,
}

struct TextGeometryBoundaryIndex {
  left: i32,
  top: i32,
  tile_columns: usize,
  tiles: Vec<SmallVec<[TextGeometryBoundaryEdge; 16]>>,
}

impl TextGeometryBoundaryIndex {
  const TILE_SIZE: i32 = 8;

  fn new(
    geometry: &Static3dTextGeometry,
    left: i32,
    top: i32,
    width: usize,
    height: usize,
    maximum_distance: f32,
  ) -> Option<Self> {
    if width == 0 || height == 0 || !maximum_distance.is_finite() || maximum_distance < 0.0 {
      return None;
    }
    let tile_columns = width.div_ceil(Self::TILE_SIZE as usize);
    let tile_rows = height.div_ceil(Self::TILE_SIZE as usize);
    let mut tiles = vec![SmallVec::new(); tile_columns.checked_mul(tile_rows)?];
    let right = left + width as i32;
    let bottom = top + height as i32;

    for (contour_index, contour) in geometry.contours.iter().enumerate() {
      for (edge_index, (first, second)) in contour
        .points
        .iter()
        .copied()
        .zip(contour.points.iter().copied().cycle().skip(1))
        .take(contour.points.len())
        .enumerate()
      {
        let edge_left = (first.0.min(second.0) - maximum_distance).floor() as i32;
        let edge_top = (first.1.min(second.1) - maximum_distance).floor() as i32;
        let edge_right = (first.0.max(second.0) + maximum_distance).ceil() as i32;
        let edge_bottom = (first.1.max(second.1) + maximum_distance).ceil() as i32;
        if edge_right < left || edge_bottom < top || edge_left >= right || edge_top >= bottom {
          continue;
        }
        let first_tile_x = ((edge_left - left).max(0) / Self::TILE_SIZE) as usize;
        let first_tile_y = ((edge_top - top).max(0) / Self::TILE_SIZE) as usize;
        let last_tile_x = ((edge_right.min(right - 1) - left) / Self::TILE_SIZE) as usize;
        let last_tile_y = ((edge_bottom.min(bottom - 1) - top) / Self::TILE_SIZE) as usize;
        let edge = TextGeometryBoundaryEdge {
          contour_index,
          edge_index,
        };
        for tile_y in first_tile_y..=last_tile_y {
          for tile_x in first_tile_x..=last_tile_x {
            tiles[tile_y * tile_columns + tile_x].push(edge);
          }
        }
      }
    }
    Some(Self {
      left,
      top,
      tile_columns,
      tiles,
    })
  }

  fn owner_at(
    &self,
    geometry: &Static3dTextGeometry,
    point: (f32, f32),
  ) -> Option<TextGeometryBoundaryOwner> {
    let local_x = point.0.floor() as i32 - self.left;
    let local_y = point.1.floor() as i32 - self.top;
    if local_x < 0 || local_y < 0 {
      return None;
    }
    let tile_x = local_x as usize / Self::TILE_SIZE as usize;
    let tile_y = local_y as usize / Self::TILE_SIZE as usize;
    let tile = self
      .tiles
      .get(tile_y.checked_mul(self.tile_columns)? + tile_x)?;
    let mut owner = None::<TextGeometryBoundaryOwner>;
    for edge in tile {
      let contour = geometry.contours.get(edge.contour_index)?;
      let first = *contour.points.get(edge.edge_index)?;
      let second = contour.points[(edge.edge_index + 1) % contour.points.len()];
      let distance = point_segment_distance(point, first, second);
      if owner.is_none_or(|owner| distance < owner.distance) {
        owner = Some(TextGeometryBoundaryOwner {
          distance,
          contour_index: edge.contour_index,
          edge_index: edge.edge_index,
        });
      }
    }
    owner
  }
}

fn text_geometry_inset_contains(
  geometry: &Static3dTextGeometry,
  point: (f32, f32),
  inset_px: f32,
) -> bool {
  if !text_geometry_contains(geometry, point) {
    return false;
  }
  if inset_px <= f32::EPSILON {
    return true;
  }
  // The glyph path and its swept bevel use the same 0.1-pixel flattened
  // outline. Permit exactly that approximation error when testing the
  // offset surface against the nearest original edge. Any larger shortfall
  // means another edge has been reached: the profile stops at that medial
  // boundary instead of crossing it and creating a reversed inner cap.
  let boundary_distance = text_geometry_boundary_distance(geometry, point);
  boundary_distance + TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX as f32 >= inset_px
}

#[derive(Clone, Copy, Debug)]
struct TextPlanarInsetReflexJoin {
  points: [(f32, f32); 4],
  bounds: (f32, f32, f32, f32),
}

impl TextPlanarInsetReflexJoin {
  fn contains(self, point: (f32, f32)) -> bool {
    let (left, top, right, bottom) = self.bounds;
    if point.0 < left || point.0 > right || point.1 < top || point.1 > bottom {
      return false;
    }
    let mut winding = 0_i32;
    for (first, second) in self
      .points
      .into_iter()
      .zip(self.points.into_iter().cycle().skip(1))
    {
      if first.1 <= point.1 {
        if second.1 > point.1 && text_surface_edge(first, second, point) > 0.0 {
          winding += 1;
        }
      } else if second.1 <= point.1 && text_surface_edge(first, second, point) < 0.0 {
        winding -= 1;
      }
    }
    winding != 0
  }
}

struct TextPlanarInset {
  reflex_joins: Vec<TextPlanarInsetReflexJoin>,
  terminal_edges: Option<Vec<[(f32, f32); 2]>>,
}

impl TextPlanarInset {
  fn new(
    geometry: &Static3dTextGeometry,
    inset_px: f32,
    direct_inset_cells: Option<&[DirectInsetCell]>,
    use_mesh_boundary: bool,
  ) -> Self {
    let terminal_edges = direct_inset_cells
      .map(|cells| direct_inset_boundary_edges(cells, inset_px))
      .or_else(|| {
        (use_mesh_boundary && inset_px <= f32::EPSILON)
          .then(|| text_geometry_boundary_edges(geometry))
      });
    let mut reflex_joins = Vec::new();
    if inset_px <= f32::EPSILON {
      return Self {
        reflex_joins,
        terminal_edges,
      };
    }
    let inward_normal = |edge: (f32, f32)| {
      if geometry.solid_on_right {
        (-edge.1, edge.0)
      } else {
        (edge.1, -edge.0)
      }
    };
    for contour in &geometry.contours {
      if contour.points.len() < 3 {
        continue;
      }
      let inset_ring = offset_text_3d_contour(&contour.points, inset_px, geometry.solid_on_right);
      for (index, &inset_point) in inset_ring.iter().enumerate() {
        let previous = (index + contour.points.len() - 1) % contour.points.len();
        let next = (index + 1) % contour.points.len();
        // Flattened chords inside one source curve are only an approximation
        // of that smooth curve. A miter wedge exists at an authored corner,
        // not at every chord boundary.
        if contour.incoming_curve_edges[index] && contour.incoming_curve_edges[next] {
          continue;
        }
        let current = contour.points[index];
        let previous_point = contour.points[previous];
        let next_point = contour.points[next];
        let previous_edge = (current.0 - previous_point.0, current.1 - previous_point.1);
        let next_edge = (next_point.0 - current.0, next_point.1 - current.1);
        let previous_length = previous_edge.0.hypot(previous_edge.1);
        let next_length = next_edge.0.hypot(next_edge.1);
        if previous_length <= f32::EPSILON || next_length <= f32::EPSILON {
          continue;
        }
        let previous_edge = (
          previous_edge.0 / previous_length,
          previous_edge.1 / previous_length,
        );
        let next_edge = (next_edge.0 / next_length, next_edge.1 / next_length);
        let turn = previous_edge.0 * next_edge.1 - previous_edge.1 * next_edge.0;
        let reflex = if geometry.solid_on_right {
          turn < -1.0e-5
        } else {
          turn > 1.0e-5
        };
        if !reflex {
          continue;
        }
        let previous_normal = inward_normal(previous_edge);
        let next_normal = inward_normal(next_edge);
        let previous_offset = (
          current.0 + previous_normal.0 * inset_px,
          current.1 + previous_normal.1 * inset_px,
        );
        let next_offset = (
          current.0 + next_normal.0 * inset_px,
          current.1 + next_normal.1 * inset_px,
        );
        let points = [current, previous_offset, inset_point, next_offset];
        if signed_contour_area(&points).abs() <= 1.0e-5 {
          continue;
        }
        reflex_joins.push(TextPlanarInsetReflexJoin {
          points,
          bounds: text_3d_contour_bounds(&points),
        });
      }
    }
    Self {
      reflex_joins,
      terminal_edges,
    }
  }

  fn contains(&self, geometry: &Static3dTextGeometry, point: (f32, f32), inset_px: f32) -> bool {
    if let Some(edges) = self.terminal_edges.as_deref() {
      return text_direct_inset_boundary_contains(edges, point);
    }
    text_geometry_inset_contains(geometry, point, inset_px)
      && !self.reflex_joins.iter().any(|join| join.contains(point))
  }
}

fn text_geometry_boundary_edges(geometry: &Static3dTextGeometry) -> Vec<[(f32, f32); 2]> {
  geometry
    .contours
    .iter()
    .flat_map(|contour| {
      contour
        .points
        .iter()
        .copied()
        .zip(contour.points.iter().copied().cycle().skip(1))
        .take(contour.points.len())
    })
    .map(|(first, second)| [first, second])
    .collect()
}

fn direct_inset_boundary_edges(cells: &[DirectInsetCell], inset_px: f32) -> Vec<[(f32, f32); 2]> {
  const INSET_EPSILON: f32 = 1.0e-5;
  cells
    .iter()
    .filter_map(|cell| {
      let span = cell.inner.inset - cell.outer.inset;
      if span <= INSET_EPSILON
        || inset_px < cell.outer.inset - INSET_EPSILON
        || inset_px > cell.inner.inset + INSET_EPSILON
      {
        return None;
      }
      let parameter = ((inset_px - cell.outer.inset) / span).clamp(0.0, 1.0);
      let edge = std::array::from_fn(|index| {
        let outer = cell.outer.endpoints[index].point;
        let inner = cell.inner.endpoints[index].point;
        (
          outer.0 + (inner.0 - outer.0) * parameter,
          outer.1 + (inner.1 - outer.1) * parameter,
        )
      });
      ((edge[0].0 - edge[1].0).hypot(edge[0].1 - edge[1].1) > INSET_EPSILON).then_some(edge)
    })
    .collect()
}

fn text_direct_inset_boundary_contains(edges: &[[(f32, f32); 2]], point: (f32, f32)) -> bool {
  let mut winding = 0_i32;
  for [first, second] in edges {
    if first.1 <= point.1 {
      if second.1 > point.1 && text_surface_edge(*first, *second, point) > 0.0 {
        winding += 1;
      }
    } else if second.1 <= point.1 && text_surface_edge(*first, *second, point) < 0.0 {
      winding -= 1;
    }
  }
  winding != 0
}

#[derive(Clone, Copy, Debug)]
struct TextDirectInsetSurfaceSample {
  visibility_depth: f32,
  surface_distance_px: f32,
  profile_strip_index: usize,
  profile_parameter: f32,
}

fn text_direct_inset_surface_sample(
  options: TextBevelOptions<'_>,
  profile_strips: &[TextBevelProfileStrip],
  point: (f32, f32),
  center: (f32, f32),
  boundary_distance: f32,
  selected_profile_strip: Option<usize>,
) -> Option<TextDirectInsetSurfaceSample> {
  if options.geometry_width <= f32::EPSILON || !boundary_distance.is_finite() {
    return None;
  }
  let mut nearest = None::<TextDirectInsetSurfaceSample>;
  for (profile_strip_index, strip) in profile_strips.iter().enumerate() {
    if selected_profile_strip.is_some_and(|selected| selected != profile_strip_index) {
      continue;
    }
    let outer_inset = options.geometry_width * strip.outer.inset;
    let inner_inset = options.geometry_width * strip.inner.inset;
    let minimum_inset = outer_inset.min(inner_inset);
    let maximum_inset = outer_inset.max(inner_inset);
    if boundary_distance + 1.0e-5 < minimum_inset || boundary_distance - 1.0e-5 > maximum_inset {
      continue;
    }
    let inset_delta = inner_inset - outer_inset;
    let parameter = if inset_delta.abs() <= f32::EPSILON {
      0.5
    } else {
      ((boundary_distance - outer_inset) / inset_delta).clamp(0.0, 1.0)
    };
    // Keep the existing finite profile chords in this topology-only repair.
    // Continuous Bezier profile evaluation is an independently measured
    // variable and must not be folded into the direct-inset change.
    let profile_height = strip.outer.height + (strip.inner.height - strip.outer.height) * parameter;
    let surface_z = options.surface_z + profile_height * options.height * options.height_direction;
    let model_point = [point.0 - center.0, point.1 - center.1, surface_z];
    let visibility_depth =
      text_surface_visibility_depth(options.projection, model_point, options.pixels_per_point);
    let surface_distance_px = strip.outer_surface_distance_px
      + (strip.inner_surface_distance_px - strip.outer_surface_distance_px) * parameter;
    let sample = TextDirectInsetSurfaceSample {
      visibility_depth,
      surface_distance_px,
      profile_strip_index,
      profile_parameter: parameter,
    };
    if nearest.is_none_or(|nearest| visibility_depth > nearest.visibility_depth) {
      nearest = Some(sample);
    }
  }
  nearest
}

struct TextDirectInsetMaterialPoint {
  boundary: TextGeometryBoundaryOwner,
  surface: TextDirectInsetSurfaceSample,
  point: (f32, f32),
}

struct TextDirectInsetMaterialCells {
  profile_strip_count: usize,
  contour_edge_offsets: Vec<usize>,
  triangle_pairs: Vec<[usize; 2]>,
}

impl TextDirectInsetMaterialCells {
  fn new(
    geometry: &Static3dTextGeometry,
    profile_strip_count: usize,
    triangles: &[TextSurfaceTriangle],
    back_face: bool,
  ) -> Option<Self> {
    if profile_strip_count == 0 {
      return None;
    }
    let mut contour_edge_offsets = Vec::with_capacity(geometry.contours.len() + 1);
    contour_edge_offsets.push(0);
    for contour in &geometry.contours {
      contour_edge_offsets.push(contour_edge_offsets.last().copied()? + contour.points.len());
    }
    let edge_count = contour_edge_offsets.last().copied()?;
    let mut triangle_pairs = vec![[usize::MAX; 2]; edge_count * profile_strip_count];
    for (triangle_index, triangle) in triangles.iter().enumerate() {
      let Some(source) = triangle
        .bevel_source
        .filter(|source| source.back_face == back_face && !source.direct_inset_graph)
      else {
        continue;
      };
      let contour_index = source.contour_index as usize;
      let edge_index = source.edge_index as usize;
      let profile_strip_index = source.profile_strip_index as usize;
      let Some((&edge_start, &edge_end)) = contour_edge_offsets
        .get(contour_index)
        .zip(contour_edge_offsets.get(contour_index + 1))
      else {
        continue;
      };
      if edge_index >= edge_end - edge_start || profile_strip_index >= profile_strip_count {
        continue;
      }
      let cell_index = (edge_start + edge_index) * profile_strip_count + profile_strip_index;
      triangle_pairs[cell_index][usize::from(source.second_half)] = triangle_index;
    }
    Some(Self {
      profile_strip_count,
      contour_edge_offsets,
      triangle_pairs,
    })
  }

  fn color_at(
    &self,
    triangles: &[TextSurfaceTriangle],
    material_source: TextSurfaceMaterialImages<'_>,
    alpha_texture: Option<&TextGeometryAreaMask>,
    material: Option<a::PresetMaterialTypeValues>,
    sample: TextDirectInsetMaterialPoint,
  ) -> Option<[f32; 4]> {
    let TextDirectInsetMaterialPoint {
      boundary,
      surface,
      point,
    } = sample;
    let edge_start = *self.contour_edge_offsets.get(boundary.contour_index)?;
    let cell_index =
      (edge_start + boundary.edge_index) * self.profile_strip_count + surface.profile_strip_index;
    let [first_index, second_index] = *self.triangle_pairs.get(cell_index)?;
    let first = triangles.get(first_index)?;
    let second = triangles.get(second_index)?;
    let outer_first = first.vertices[0];
    let outer_second = first.vertices[1];
    let inner_second = first.vertices[2];
    let inner_first = second.vertices[1];
    let profile_parameter = surface.profile_parameter;
    let interpolate_point = |outer: (f32, f32), inner: (f32, f32)| {
      (
        outer.0 + (inner.0 - outer.0) * profile_parameter,
        outer.1 + (inner.1 - outer.1) * profile_parameter,
      )
    };
    let interpolate_color = |outer: [f32; 4], inner: [f32; 4]| -> [f32; 4] {
      std::array::from_fn(|channel| {
        outer[channel] + (inner[channel] - outer[channel]) * profile_parameter
      })
    };
    let start = interpolate_point(outer_first.point, inner_first.point);
    let end = interpolate_point(outer_second.point, inner_second.point);
    let start_color = interpolate_color(outer_first.color, inner_first.color);
    let end_color = interpolate_color(outer_second.color, inner_second.color);
    let edge = (end.0 - start.0, end.1 - start.1);
    let length_squared = edge.0 * edge.0 + edge.1 * edge.1;
    let edge_parameter = if length_squared <= f32::EPSILON {
      0.5
    } else {
      ((point.0 - start.0) * edge.0 + (point.1 - start.1) * edge.1) / length_squared
    }
    .clamp(0.0, 1.0);

    let interpolated_color = std::array::from_fn(|channel| {
      start_color[channel] + (end_color[channel] - start_color[channel]) * edge_parameter
    });
    let start_material = outer_first
      .textured_material
      .zip(inner_first.textured_material)
      .map(|(outer, inner)| interpolate_text_surface_material(outer, inner, profile_parameter));
    let end_material = outer_second
      .textured_material
      .zip(inner_second.textured_material)
      .map(|(outer, inner)| interpolate_text_surface_material(outer, inner, profile_parameter));
    let textured_material = start_material
      .zip(end_material)
      .map(|(start, end)| interpolate_text_surface_material(start, end, edge_parameter));
    textured_material
      .and_then(|vertex| {
        resolve_text_surface_material_color(
          material_source,
          alpha_texture,
          material,
          vertex,
          interpolated_color[3],
        )
      })
      .or(Some(interpolated_color))
  }
}

#[derive(Clone, Copy)]
struct TextSolidSurfaceInput<'a> {
  scene: &'a a::Scene3DType,
  material: Option<a::PresetMaterialTypeValues>,
  bevel_material_source: &'a RgbaImage,
  surface_material_texture: Option<&'a TextMaterialTexture>,
  source_geometry: &'a Static3dTextGeometry,
  material_geometry: &'a Static3dTextGeometry,
  paint_opacity_geometry: &'a Static3dTextGeometry,
  planar_fill_material: Option<&'a RgbaImage>,
  planar_outline_material: Option<&'a RgbaImage>,
  planar_outline_coverage: Option<&'a RgbaImage>,
  planar_fill_uniform_paint_opacity: Option<f32>,
  planar_outline_uniform_paint_opacity: Option<f32>,
  planar_outline_material_inset_px: Option<f32>,
  fill_has_authored_transparency: bool,
  outline_has_authored_transparency: bool,
  uniform_paint_opacity: Option<f32>,
  planar_geometry: &'a Static3dTextGeometry,
  planar_coverage_geometry: &'a Static3dTextGeometry,
  planar_inset_px: f32,
  direct_inset_cells: Option<&'a [DirectInsetCell]>,
  planar_coverage: TextPlanarCoverageResolve,
  triangles: &'a [TextSurfaceTriangle],
  direct_inset_bevel: Option<TextBevelOptions<'a>>,
  contour: Option<TextContourSurfaceInput>,
  options: ProjectedImageOptions,
  geometry_lighting: Static3dGeometryLighting,
}

impl<'a> TextSolidSurfaceInput<'a> {
  /// The orthographic fixed-output mask belongs to the uninset painted face.
  /// MS-OI29500's inward bevel sweep and coincident extrusion do not alter
  /// that silhouette. Exact-config top/bottom/extrusion controls preserve
  /// Office's entire SMask, not just its bounding box. Strip every geometric
  /// solid owner together; clearing only `direct_inset_bevel` still leaves
  /// the inset cap and previously emitted bevel triangles in this pass.
  fn independent_front_coverage(self) -> Self {
    Self {
      planar_inset_px: 0.0,
      direct_inset_cells: None,
      planar_coverage: TextPlanarCoverageResolve::AreaA8,
      triangles: &[],
      direct_inset_bevel: None,
      // The source-plane contour is composed separately using its AreaA8.
      contour: None,
      ..self
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct TextContourSurfaceInput {
  width_px: f32,
  base_z: f32,
  color: Static3dColor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextPlanarCoverageResolve {
  BoxSamples8x4,
  AreaA8,
}

#[derive(Clone, Copy, Debug)]
struct TextAreaResolvedSample {
  rgb: [u8; 3],
  authored_opacity: f32,
}

fn nearest_text_area_resolved_sample(
  samples: &[Option<TextAreaResolvedSample>],
  width: usize,
  height: usize,
  x: usize,
  y: usize,
) -> Option<TextAreaResolvedSample> {
  const MAX_FRINGE_SEARCH_RADIUS: usize = 8;
  for radius in 1..=MAX_FRINGE_SEARCH_RADIUS {
    let left = x.saturating_sub(radius);
    let top = y.saturating_sub(radius);
    let right = x.saturating_add(radius).min(width.saturating_sub(1));
    let bottom = y.saturating_add(radius).min(height.saturating_sub(1));
    let mut nearest = None;
    let mut nearest_distance = usize::MAX;
    for sample_y in top..=bottom {
      for sample_x in left..=right {
        if sample_x != left && sample_x != right && sample_y != top && sample_y != bottom {
          continue;
        }
        let Some(sample) = samples[sample_y * width + sample_x] else {
          continue;
        };
        let distance = sample_x.abs_diff(x).pow(2) + sample_y.abs_diff(y).pow(2);
        if distance < nearest_distance {
          nearest = Some(sample);
          nearest_distance = distance;
        }
      }
    }
    if nearest.is_some() {
      return nearest;
    }
  }
  None
}

#[derive(Clone, Copy, Debug)]
struct ResolvedTextSurfacePixel {
  rgb: [u8; 3],
  authored_opacity: f32,
  alpha: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct TextSurfaceColorAccumulator {
  premultiplied: [f32; 3],
  alpha_sum: f32,
  covered_samples: usize,
}

impl TextSurfaceColorAccumulator {
  fn add(&mut self, color: [f32; 4]) {
    let alpha = (color[3] / 255.0).clamp(0.0, 1.0);
    self.alpha_sum += alpha;
    self.covered_samples += 1;
    for (accumulator, channel) in self.premultiplied.iter_mut().zip(color) {
      *accumulator += channel.clamp(0.0, 255.0) * alpha;
    }
  }

  fn resolved(self) -> Option<([f32; 3], f32)> {
    if self.alpha_sum <= f32::EPSILON || self.covered_samples == 0 {
      return None;
    }
    Some((
      self.premultiplied.map(|channel| channel / self.alpha_sum),
      self.alpha_sum / self.covered_samples as f32,
    ))
  }
}

fn contour_depth_wins(base: &TextSurfaceRasterSample, contour_depth: f32) -> bool {
  contour_depth.is_finite()
    && (!base.covered
      || contour_depth > base.visibility_depth
      || (contour_depth == base.visibility_depth
        && base.tie_priority < TEXT_SURFACE_CONTOUR_TIE_PRIORITY))
}

#[derive(Clone, Copy, Debug, Default)]
struct TextContourSurfaceSample {
  active: bool,
  visible: bool,
}

#[derive(Clone, Copy)]
enum TextContourSamples<'a> {
  Depth(&'a [f32]),
  Surface(&'a [TextContourSurfaceSample]),
}

impl TextContourSamples<'_> {
  fn len(self) -> usize {
    match self {
      Self::Depth(samples) => samples.len(),
      Self::Surface(samples) => samples.len(),
    }
  }

  fn ownership(self, index: usize, base: &TextSurfaceRasterSample) -> (bool, bool) {
    match self {
      Self::Depth(samples) => {
        let depth = samples[index];
        (depth.is_finite(), contour_depth_wins(base, depth))
      }
      Self::Surface(samples) => {
        let sample = samples[index];
        debug_assert!(!sample.visible || sample.active);
        (sample.active, sample.visible)
      }
    }
  }
}

/// Resolves one pixel while keeping continuous contour area independent from
/// the finite depth discriminator.
///
/// `contour_samples` records the semantic contour at the same sample positions
/// as `base_samples`, but `base_samples` retains the underlying bevel/cap color
/// even where that contour wins. The exact no-bevel Office interpolation pins
/// the contour's absolute area to the phase-0 A8 mask; finite samples only
/// decide what fraction of that area remains visible on the 3-D surface.
fn resolve_text_surface_pixel(
  base_samples: &[TextSurfaceRasterSample],
  contour_samples: Option<TextContourSamples<'_>>,
  contour_area_alpha: u8,
  source_area_alpha: Option<u8>,
  contour_color: Option<[f32; 4]>,
) -> Option<ResolvedTextSurfacePixel> {
  if base_samples.is_empty() {
    return None;
  }

  let mut all_base = TextSurfaceColorAccumulator::default();
  for sample in base_samples.iter().filter(|sample| sample.covered) {
    all_base.add(sample.color);
  }
  let (Some(contour_samples), Some(contour_color)) = (contour_samples, contour_color) else {
    let (rgb, authored_opacity) = all_base.resolved()?;
    return Some(ResolvedTextSurfacePixel {
      rgb: rgb.map(|channel| channel.round().clamp(0.0, 255.0) as u8),
      authored_opacity,
      alpha: all_base.alpha_sum / base_samples.len() as f32,
    });
  };
  debug_assert_eq!(base_samples.len(), contour_samples.len());

  let mut visible_base = TextSurfaceColorAccumulator::default();
  let mut active_contour_samples = 0_usize;
  let mut winning_contour_samples = 0_usize;
  for (index, base) in base_samples.iter().enumerate() {
    let (active, winning) = contour_samples.ownership(index, base);
    if active {
      active_contour_samples += 1;
    }
    if winning {
      winning_contour_samples += 1;
    } else if base.covered {
      visible_base.add(base.color);
    }
  }

  let contour_area = f32::from(contour_area_alpha) / 255.0;
  let conditional_depth_win = if active_contour_samples == 0 {
    0.0
  } else {
    winning_contour_samples as f32 / active_contour_samples as f32
  };
  let visible_contour_area = contour_area * conditional_depth_win;
  let source_area = source_area_alpha.map_or_else(
    || all_base.covered_samples as f32 / base_samples.len() as f32,
    |alpha| f32::from(alpha) / 255.0,
  );
  // This is the same straight-alpha union used by the independently rendered
  // source-plane coverage. Using it here keeps RGB and the later alpha
  // replacement on one geometric denominator.
  let union_area = contour_area + source_area * (1.0 - contour_area);
  if union_area <= f32::EPSILON {
    return None;
  }

  let visible_base_area = (union_area - visible_contour_area).max(0.0);
  let base = visible_base.resolved().or_else(|| all_base.resolved());
  let mut premultiplied = [0.0_f32; 3];
  let contour_opacity = (contour_color[3] / 255.0).clamp(0.0, 1.0);
  let mut alpha = visible_contour_area * contour_opacity;
  for (accumulator, channel) in premultiplied.iter_mut().zip(contour_color) {
    *accumulator += visible_contour_area * contour_opacity * channel.clamp(0.0, 255.0);
  }
  if let Some((base_rgb, base_opacity)) = base {
    alpha += visible_base_area * base_opacity;
    for (accumulator, channel) in premultiplied.iter_mut().zip(base_rgb) {
      *accumulator += visible_base_area * base_opacity * channel.clamp(0.0, 255.0);
    }
  }
  if alpha <= f32::EPSILON {
    return None;
  }

  Some(ResolvedTextSurfacePixel {
    rgb: premultiplied.map(|channel| (channel / alpha).round().clamp(0.0, 255.0) as u8),
    authored_opacity: (alpha / union_area).clamp(0.0, 1.0),
    alpha: alpha.clamp(0.0, 1.0),
  })
}

fn composite_text_solid_surfaces(
  destination: &mut RgbaImage,
  source: &RgbaImage,
  input: TextSolidSurfaceInput<'_>,
  rasterization: TextSurfaceRasterization,
) {
  let TextSolidSurfaceInput {
    scene,
    material,
    bevel_material_source,
    surface_material_texture,
    source_geometry,
    material_geometry,
    paint_opacity_geometry,
    planar_fill_material,
    planar_outline_material,
    planar_outline_coverage,
    planar_fill_uniform_paint_opacity,
    planar_outline_uniform_paint_opacity,
    planar_outline_material_inset_px,
    fill_has_authored_transparency,
    outline_has_authored_transparency,
    uniform_paint_opacity,
    planar_geometry,
    planar_coverage_geometry,
    planar_inset_px,
    direct_inset_cells,
    planar_coverage,
    triangles,
    direct_inset_bevel,
    contour,
    options,
    geometry_lighting,
  } = input;
  // Keep source glyph alpha on the independent AreaA8 path below: the mesh
  // resolves lighting and depth ownership, not the DirectWrite glyph
  // realization.
  let sample_pattern = rasterization.sample_pattern();
  let sample_count = sample_pattern.count();
  let destination_dimensions = destination.dimensions();

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
  let planar_inset = TextPlanarInset::new(
    planar_geometry,
    planar_inset_px,
    direct_inset_cells,
    geometry_lighting == Static3dGeometryLighting::Text,
  );
  let source_grid = matches!(rasterization, TextSurfaceRasterization::SourceGrid);
  // Both composed paint and separate fill/outline/coverage images use the
  // caller's page-space sampling contract. Only the physical mesh receives
  // its device-space conversion. Select the geometry belonging to the paint
  // being sampled; do not infer a different pixel phase from composition.
  let planar_material_geometry =
    if planar_fill_material.is_some() || planar_outline_material.is_some() {
      paint_opacity_geometry
    } else {
      material_geometry
    };
  let material_alpha_mask = (fill_has_authored_transparency || outline_has_authored_transparency)
    .then(|| text_geometry_area_mask(material_geometry))
    .flatten();
  let independent_alpha_mask = (fill_has_authored_transparency
    || outline_has_authored_transparency)
    .then(|| text_geometry_area_mask(paint_opacity_geometry))
    .flatten();
  let color_scale_x =
    material_geometry.page_plane_scale_x / paint_opacity_geometry.page_plane_scale_x;
  let color_scale_y =
    material_geometry.page_plane_scale_y / paint_opacity_geometry.page_plane_scale_y;
  let material_images = TextSurfaceMaterialImages {
    combined: bevel_material_source,
    surface_material_texture,
    independent_to_combined: Transform::from_row(
      color_scale_x,
      0.0,
      0.0,
      color_scale_y,
      material_geometry.page_plane_translate_x
        - paint_opacity_geometry.page_plane_translate_x * color_scale_x,
      material_geometry.page_plane_translate_y
        - paint_opacity_geometry.page_plane_translate_y * color_scale_y,
    ),
    independent_alpha: independent_alpha_mask.as_ref(),
  };
  let planar_area_mask = (source_grid && planar_coverage == TextPlanarCoverageResolve::AreaA8)
    .then(|| text_geometry_area_mask(planar_coverage_geometry))
    .flatten();
  let contour_depth_phase_px = if geometry_lighting == Static3dGeometryLighting::Text {
    WORD_TEXT_ANTIALIASED_CONTOUR_PHASE_PX
  } else {
    0.0
  };
  let contour_area_mask = source_grid
    .then_some(contour)
    .flatten()
    .and_then(|contour| {
      // The exact 33-gray no-bevel Office interpolation freezes boundary area
      // in physical phase 0. Depth registration remains an independent
      // semantic-space decision below.
      text_geometry_stroke_area_mask(source_geometry, contour.width_px, 0.0)
    });
  let contour_source_area_mask = contour_area_mask
    .as_ref()
    .and_then(|_| text_geometry_area_mask(source_geometry));
  let planar_surface_normal = lighting_surface_normal(scene, projection, [0.0, 0.0, 1.0]);

  let triangle_bounds = triangles
    .iter()
    .flat_map(|triangle| {
      triangle
        .vertices
        .iter()
        .map(|vertex| rasterization.surface_to_raster(vertex.point, destination_dimensions))
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
  let planar_bounds = planar_geometry
    .contours
    .iter()
    .flat_map(|contour| contour.points.iter().copied())
    .map(|point| {
      let projected = map_homogeneous(planar_matrix, point.0 - center_x, point.1 - center_y);
      rasterization.surface_to_raster(
        (center_x + projected.0, center_y + projected.1),
        destination_dimensions,
      )
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
  let bounds = [
    triangle_bounds,
    planar_bounds,
    source_grid
      .then(|| planar_area_mask.as_ref().map(TextGeometryAreaMask::bounds))
      .flatten(),
    source_grid
      .then(|| contour_area_mask.as_ref().map(TextGeometryAreaMask::bounds))
      .flatten(),
  ]
  .into_iter()
  .flatten()
  .reduce(|first, second| {
    (
      first.0.min(second.0),
      first.1.min(second.1),
      first.2.max(second.2),
      first.3.max(second.3),
    )
  });
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
    vec![TextSurfaceRasterSample::default(); raster_width * raster_height * sample_count];
  // The complete graph is authoritative for coverage and source-edge
  // ownership. Keep the unsplit source mesh alongside it solely as the
  // material-attribute witness: subdividing a face at an inset event must not
  // silently change its texture, diffuse, or specular realization.
  let has_direct_inset_graph = direct_inset_bevel.is_some_and(|bevel| {
    triangles.iter().any(|triangle| {
      triangle
        .bevel_source
        .is_some_and(|source| source.back_face == bevel.back_face && source.direct_inset_graph)
    })
  });
  let direct_inset_profile_strips = direct_inset_bevel.map(|bevel| {
    text_bevel_profile_strips(
      bevel.normal_width,
      bevel.height,
      bevel.preset,
      bevel.geometry_lighting,
      bevel.outline_material_inset_px,
      bevel.outline_coverage.map(|_| 1.0),
      source_geometry.bevel_profile_tolerance_px,
    )
    .1
  });
  let direct_inset_material_cells = direct_inset_bevel
    .zip(direct_inset_profile_strips.as_deref())
    .and_then(|(bevel, profile_strips)| {
      TextDirectInsetMaterialCells::new(
        source_geometry,
        profile_strips.len(),
        triangles,
        bevel.back_face,
      )
    });
  let direct_inset_boundary_index = direct_inset_bevel
    .zip(direct_inset_profile_strips.as_deref())
    .and_then(|(bevel, profile_strips)| {
      let maximum_inset = profile_strips
        .iter()
        .flat_map(|strip| [strip.outer.inset, strip.inner.inset])
        .map(|inset| bevel.geometry_width * inset)
        .fold(0.0_f32, f32::max);
      // This index is queried with source-plane points recovered from the
      // projected mesh. Its tile frame must therefore remain the source
      // raster, not the camera-projected destination bounds.
      TextGeometryBoundaryIndex::new(
        source_geometry,
        0,
        0,
        source.width() as usize,
        source.height() as usize,
        maximum_inset + TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX as f32,
      )
    });
  let mut direct_inset_samples =
    direct_inset_bevel.map(|_| vec![TextSurfaceRasterSample::default(); samples.len()]);
  let mut direct_inset_topology =
    direct_inset_bevel.map(|_| vec![TextDirectInsetTopologySample::default(); samples.len()]);
  let mut direct_inset_graph_samples =
    has_direct_inset_graph.then(|| vec![TextDirectInsetGraphSample::default(); samples.len()]);
  let text_surface_contour =
    contour_area_mask.is_some() && geometry_lighting == Static3dGeometryLighting::Text;
  let mut bevel_surface_distances = (text_surface_contour
    && triangles.iter().any(|triangle| {
      triangle
        .vertices
        .iter()
        .any(|vertex| vertex.bevel_collision.is_some())
    }))
  .then(|| vec![f32::INFINITY; samples.len()]);
  let mut contour_surface_samples =
    text_surface_contour.then(|| vec![TextContourSurfaceSample::default(); samples.len()]);
  let mut contour_depth_samples = (contour_area_mask.is_some()
    && geometry_lighting == Static3dGeometryLighting::Shape)
    .then(|| vec![f32::NEG_INFINITY; samples.len()]);
  let fully_occluded_triangles = (geometry_lighting == Static3dGeometryLighting::Shape)
    .then(|| fully_occluded_text_surface_triangles(triangles));
  // Direct3D 9 transformed screen coordinates address pixel centers at
  // integer coordinates. Text continues to use the GDI+/image-space
  // convention whose coverage cell [n, n + 1] is centered at n + 1/2.
  let pixel_center_offset = if geometry_lighting == Static3dGeometryLighting::Shape {
    0.0
  } else {
    0.5
  };

  // Microsoft's Direct2D 3-D text samples emit one shared triangle mesh and
  // explicitly require that adjacent faces neither overlap nor leave
  // T-junctions. Rasterizing every quad through source-over violates that
  // contract at each antialiased shared edge. Give each subpixel sample one
  // surface owner instead, and use camera-space depth for folded Office
  // profiles whose branches overlap after projection.
  for (triangle_index, triangle) in triangles.iter().enumerate() {
    if fully_occluded_triangles
      .as_ref()
      .is_some_and(|occluded| occluded[triangle_index])
    {
      continue;
    }
    let [first, second, third] = triangle.vertices;
    let first_point = rasterization.surface_to_raster(first.point, destination_dimensions);
    let second_point = rasterization.surface_to_raster(second.point, destination_dimensions);
    let third_point = rasterization.surface_to_raster(third.point, destination_dimensions);
    let direct_inset_graph_triangle =
      direct_inset_bevel
        .zip(triangle.bevel_source)
        .is_some_and(|(bevel, source)| {
          source.back_face == bevel.back_face && source.direct_inset_graph
        });
    let direct_inset_triangle =
      direct_inset_bevel
        .zip(triangle.bevel_source)
        .is_some_and(|(bevel, source)| {
          source.back_face == bevel.back_face && !source.direct_inset_graph
        });
    let signed_area = text_surface_edge(first_point, second_point, third_point);
    if signed_area.abs() <= 1.0e-6 {
      continue;
    }
    let has_bevel_collision = first.bevel_collision.is_some()
      && second.bevel_collision.is_some()
      && third.bevel_collision.is_some();
    let textured_material = match (
      first.textured_material,
      second.textured_material,
      third.textured_material,
    ) {
      (Some(first), Some(second), Some(third)) => Some((first, second, third)),
      _ => None,
    };
    let has_source_attributes = has_bevel_collision || textured_material.is_some();
    let final_grid_text_msaa = geometry_lighting == Static3dGeometryLighting::Text
      && matches!(rasterization, TextSurfaceRasterization::FinalGrid(_));
    let triangle_left = first_point.0.min(second_point.0).min(third_point.0).floor() as i32;
    let triangle_top = first_point.1.min(second_point.1).min(third_point.1).floor() as i32;
    let triangle_right = first_point.0.max(second_point.0).max(third_point.0).ceil() as i32;
    let triangle_bottom = first_point.1.max(second_point.1).max(third_point.1).ceil() as i32;
    for pixel_y in triangle_top.max(top)..triangle_bottom.min(bottom) {
      for pixel_x in triangle_left.max(left)..triangle_right.min(right) {
        let local_pixel = (pixel_y - top) as usize * raster_width + (pixel_x - left) as usize;
        let center_point = (
          pixel_x as f32 + pixel_center_offset,
          pixel_y as f32 + pixel_center_offset,
        );
        let center_first_weight =
          text_surface_edge(second_point, third_point, center_point) / signed_area;
        let center_second_weight =
          text_surface_edge(third_point, first_point, center_point) / signed_area;
        let center_third_weight =
          text_surface_edge(first_point, second_point, center_point) / signed_area;
        let mut coverage_mask = 0_u32;
        if final_grid_text_msaa {
          for sample_index in 0..sample_count {
            let (sample_x, sample_y) = sample_pattern.position(sample_index);
            let point = (pixel_x as f32 + sample_x, pixel_y as f32 + sample_y);
            if text_surface_triangle_covers_sample(
              first_point,
              second_point,
              third_point,
              point,
              signed_area,
            ) {
              coverage_mask |= 1_u32 << sample_index;
            }
          }
          if coverage_mask == 0 {
            continue;
          }
        }
        let pixel_color = if geometry_lighting == Static3dGeometryLighting::Shape {
          // Direct3D MSAA interpolates vertex attributes once at the integer
          // pixel center—even when that center requires extrapolation—and
          // replicates the shader result to every covered sub-sample.
          let color = std::array::from_fn(|channel| {
            first.color[channel] * center_first_weight
              + second.color[channel] * center_second_weight
              + third.color[channel] * center_third_weight
          });
          Some(color)
        } else if final_grid_text_msaa {
          // Ordinary Direct3D multisampling runs the pixel shader once per
          // covered primitive/pixel. Coverage and depth remain per sample,
          // but the resulting color is replicated to every passing sample.
          // Word's captured shader declares BOTH COLOR and TEXCOORD centroid.
          // Testing the triangle center alone is not equivalent to the actual
          // full-coverage centroid selection at a partially covered pixel.
          let (centroid_x, centroid_y) = sample_pattern
            .centroid(coverage_mask)
            .expect("covered primitive pixel");
          let centroid_point = (pixel_x as f32 + centroid_x, pixel_y as f32 + centroid_y);
          let screen_color_weights = [
            text_surface_edge(second_point, third_point, centroid_point) / signed_area,
            text_surface_edge(third_point, first_point, centroid_point) / signed_area,
            text_surface_edge(first_point, second_point, centroid_point) / signed_area,
          ];
          let Some(color_weights) = perspective_correct_text_surface_weights(
            projection.parallel,
            [
              first.visibility_depth,
              second.visibility_depth,
              third.visibility_depth,
            ],
            screen_color_weights,
          ) else {
            continue;
          };
          let interpolated_color = std::array::from_fn(|channel| {
            first.color[channel] * color_weights[0]
              + second.color[channel] * color_weights[1]
              + third.color[channel] * color_weights[2]
          });
          // The inset graph owns the primitive's coverage, while the unsplit
          // source cell owns its lighting attributes. Evaluate that same cell
          // at the graph primitive's centroid ONCE, rather than relighting
          // newly inserted graph vertices or shading each coverage sample.
          let reconstructed = if direct_inset_graph_triangle {
            (|| {
              let cells = direct_inset_material_cells.as_ref()?;
              let bevel = direct_inset_bevel?;
              let strips = direct_inset_profile_strips.as_deref()?;
              let source = triangle.bevel_source?;
              let collisions = [
                first.bevel_collision?,
                second.bevel_collision?,
                third.bevel_collision?,
              ];
              let source_point = (
                (0..3)
                  .map(|i| collisions[i].source_point.0 * color_weights[i])
                  .sum(),
                (0..3)
                  .map(|i| collisions[i].source_point.1 * color_weights[i])
                  .sum(),
              );
              let boundary = TextGeometryBoundaryOwner {
                contour_index: source.contour_index as usize,
                edge_index: source.edge_index as usize,
                distance: (0..3)
                  .map(|i| collisions[i].inset_px * color_weights[i])
                  .sum(),
              };
              let surface = text_direct_inset_surface_sample(
                bevel,
                strips,
                source_point,
                (center_x, center_y),
                boundary.distance,
                Some(usize::from(source.profile_strip_index)),
              )?;
              cells.color_at(
                triangles,
                material_images,
                material_alpha_mask.as_ref(),
                material,
                TextDirectInsetMaterialPoint {
                  boundary,
                  surface,
                  point: rasterization.raster_to_surface(centroid_point, destination_dimensions),
                },
              )
            })()
          } else {
            None
          };
          let resolved = reconstructed.or_else(|| {
            textured_material.and_then(|(first_material, second_material, third_material)| {
              let source_weights = color_weights;
              let vertex = TextSurfaceMaterialVertex {
                source: first_material.source,
                material_point: (
                  first_material.material_point.0 * source_weights[0]
                    + second_material.material_point.0 * source_weights[1]
                    + third_material.material_point.0 * source_weights[2],
                  first_material.material_point.1 * source_weights[0]
                    + second_material.material_point.1 * source_weights[1]
                    + third_material.material_point.1 * source_weights[2],
                ),
                diffuse: std::array::from_fn(|channel| {
                  first_material.diffuse[channel] * color_weights[0]
                    + second_material.diffuse[channel] * color_weights[1]
                    + third_material.diffuse[channel] * color_weights[2]
                }),
                specular_incident: std::array::from_fn(|channel| {
                  first_material.specular_incident[channel] * color_weights[0]
                    + second_material.specular_incident[channel] * color_weights[1]
                    + third_material.specular_incident[channel] * color_weights[2]
                }),
                material_opacity: first_material.material_opacity * color_weights[0]
                  + second_material.material_opacity * color_weights[1]
                  + third_material.material_opacity * color_weights[2],
                alpha_texture_fraction: first_material.alpha_texture_fraction * source_weights[0]
                  + second_material.alpha_texture_fraction * source_weights[1]
                  + third_material.alpha_texture_fraction * source_weights[2],
              };
              resolve_text_surface_material_color(
                material_images,
                material_alpha_mask.as_ref(),
                material,
                vertex,
                interpolated_color[3],
              )
            })
          });
          Some(resolved.unwrap_or(interpolated_color))
        } else {
          None
        };
        for sample_index_in_pixel in 0..sample_count {
          let (sample_x, sample_y) = sample_pattern.position(sample_index_in_pixel);
          let point = (pixel_x as f32 + sample_x, pixel_y as f32 + sample_y);
          let first_weight = text_surface_edge(second_point, third_point, point) / signed_area;
          let second_weight = text_surface_edge(third_point, first_point, point) / signed_area;
          let third_weight = text_surface_edge(first_point, second_point, point) / signed_area;
          if if final_grid_text_msaa {
            coverage_mask & (1_u32 << sample_index_in_pixel) == 0
          } else {
            !text_surface_triangle_covers_sample(
              first_point,
              second_point,
              third_point,
              point,
              signed_area,
            )
          } {
            continue;
          }
          let visibility_depth = first.visibility_depth * first_weight
            + second.visibility_depth * second_weight
            + third.visibility_depth * third_weight;
          let sample_index = local_pixel * sample_count + sample_index_in_pixel;
          let screen_weights = [first_weight, second_weight, third_weight];
          let text_color_weights = if geometry_lighting == Static3dGeometryLighting::Text {
            let Some(weights) = perspective_correct_text_surface_weights(
              projection.parallel,
              [
                first.visibility_depth,
                second.visibility_depth,
                third.visibility_depth,
              ],
              screen_weights,
            ) else {
              continue;
            };
            weights
          } else {
            screen_weights
          };
          let source_weights = if has_source_attributes {
            perspective_correct_text_surface_weights(
              projection.parallel,
              [
                first.visibility_depth,
                second.visibility_depth,
                third.visibility_depth,
              ],
              screen_weights,
            )
          } else {
            None
          };
          if has_source_attributes && source_weights.is_none() {
            continue;
          }
          let interpolated_color = pixel_color.unwrap_or_else(|| {
            std::array::from_fn(|channel| {
              first.color[channel] * text_color_weights[0]
                + second.color[channel] * text_color_weights[1]
                + third.color[channel] * text_color_weights[2]
            })
          });
          let sample_material = match textured_material {
            Some((first_material, second_material, third_material)) => {
              // D3D9 iterates both texture coordinates and COLOR inputs with
              // perspective correction. COLOR remains centroid-qualified at
              // the final multisample grid; this source-grid branch evaluates
              // both streams at the current sample.
              let [
                source_first_weight,
                source_second_weight,
                source_third_weight,
              ] = source_weights.unwrap_or(screen_weights);
              Some(TextSurfaceMaterialVertex {
                source: first_material.source,
                material_point: (
                  first_material.material_point.0 * source_first_weight
                    + second_material.material_point.0 * source_second_weight
                    + third_material.material_point.0 * source_third_weight,
                  first_material.material_point.1 * source_first_weight
                    + second_material.material_point.1 * source_second_weight
                    + third_material.material_point.1 * source_third_weight,
                ),
                diffuse: std::array::from_fn(|channel| {
                  first_material.diffuse[channel] * text_color_weights[0]
                    + second_material.diffuse[channel] * text_color_weights[1]
                    + third_material.diffuse[channel] * text_color_weights[2]
                }),
                specular_incident: std::array::from_fn(|channel| {
                  first_material.specular_incident[channel] * text_color_weights[0]
                    + second_material.specular_incident[channel] * text_color_weights[1]
                    + third_material.specular_incident[channel] * text_color_weights[2]
                }),
                material_opacity: first_material.material_opacity * text_color_weights[0]
                  + second_material.material_opacity * text_color_weights[1]
                  + third_material.material_opacity * text_color_weights[2],
                alpha_texture_fraction: first_material.alpha_texture_fraction * source_first_weight
                  + second_material.alpha_texture_fraction * source_second_weight
                  + third_material.alpha_texture_fraction * source_third_weight,
              })
            }
            _ => None,
          };
          let sample_color = pixel_color.unwrap_or_else(|| {
            sample_material
              .and_then(|vertex| {
                resolve_text_surface_material_color(
                  material_images,
                  material_alpha_mask.as_ref(),
                  material,
                  vertex,
                  interpolated_color[3],
                )
              })
              .unwrap_or(interpolated_color)
          });
          let mut bevel_surface_distance_px = f32::INFINITY;
          let mut graph_owner = None;
          if direct_inset_triangle {
            let donor =
              &mut direct_inset_samples.as_mut().expect("direct inset samples")[sample_index];
            if !donor.covered || visibility_depth > donor.visibility_depth {
              donor.covered = true;
              donor.visibility_depth = visibility_depth;
              donor.color = sample_color;
              donor.tie_priority = TEXT_SURFACE_TRIANGLE_TIE_PRIORITY;
            }
          }
          if let (Some(first_collision), Some(second_collision), Some(third_collision)) = (
            first.bevel_collision,
            second.bevel_collision,
            third.bevel_collision,
          ) {
            // Offset contours cease to be valid when opposite glyph edges
            // meet. Interpolate the originating source-space point and its
            // requested inset, then reject the part of a swept quad that
            // has crossed the nearest original edge. This is the portable
            // equivalent of outlining the offset geometry before
            // tessellation: narrow strokes end in a shared medial ridge,
            // while the independently authored width still controls the
            // bevel normal and therefore its material lighting.
            let [
              source_first_weight,
              source_second_weight,
              source_third_weight,
            ] = source_weights.expect("bevel collision requires source weights");
            let source_point = (
              first_collision.source_point.0 * source_first_weight
                + second_collision.source_point.0 * source_second_weight
                + third_collision.source_point.0 * source_third_weight,
              first_collision.source_point.1 * source_first_weight
                + second_collision.source_point.1 * source_second_weight
                + third_collision.source_point.1 * source_third_weight,
            );
            let inset_px = first_collision.inset_px * source_first_weight
              + second_collision.inset_px * source_second_weight
              + third_collision.inset_px * source_third_weight;
            bevel_surface_distance_px = first_collision.surface_distance_px * source_first_weight
              + second_collision.surface_distance_px * source_second_weight
              + third_collision.surface_distance_px * source_third_weight;
            let inside_inset = if direct_inset_graph_triangle {
              true
            } else if direct_inset_triangle {
              direct_inset_boundary_index
                .as_ref()
                .is_some_and(|boundary_index| {
                  text_geometry_contains(source_geometry, source_point)
                    && boundary_index
                      .owner_at(source_geometry, source_point)
                      .is_some_and(|boundary| {
                        boundary.distance + TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX as f32 >= inset_px
                      })
                })
            } else {
              text_geometry_inset_contains(source_geometry, source_point, inset_px)
            };
            if !inside_inset {
              continue;
            }
            if direct_inset_graph_triangle || direct_inset_triangle {
              let source = triangle
                .bevel_source
                .expect("direct inset triangle requires a bevel source");
              let owner = TextGeometryBoundaryOwner {
                distance: inset_px,
                contour_index: source.contour_index as usize,
                edge_index: source.edge_index as usize,
              };
              if direct_inset_graph_triangle {
                graph_owner = Some(TextDirectInsetGraphOwner {
                  boundary: owner,
                  source_point,
                  profile_strip_index: usize::from(source.profile_strip_index),
                  surface_distance_px: bevel_surface_distance_px,
                });
              } else {
                direct_inset_topology
                  .as_mut()
                  .expect("direct inset topology")[sample_index]
                  .consider(owner, source_point);
              }
            }
          }
          if direct_inset_graph_triangle {
            let graph = &mut direct_inset_graph_samples
              .as_mut()
              .expect("direct inset graph samples")[sample_index];
            graph.consider(
              visibility_depth,
              sample_color,
              graph_owner.expect("graph triangle requires source attributes"),
            );
            continue;
          }
          // Raw direct-inset triangles above are material/topology
          // witnesses, not independently depth-resolved painter surfaces.
          // Their rings can overlap after separate glyph contours merge.
          // Submit one graph-owned surface below after all witnesses have
          // been seen; other bevel/extrusion triangles keep the ordinary
          // depth path here.
          if direct_inset_triangle {
            continue;
          }
          if material_images.surface_material_texture.is_some()
            && sample_material.is_some()
            && sample_color[3] < 1.0
          {
            continue;
          }
          let sample = &mut samples[sample_index];
          if sample.covered && visibility_depth <= sample.visibility_depth {
            continue;
          }
          sample.covered = true;
          sample.visibility_depth = visibility_depth;
          sample.tie_priority = TEXT_SURFACE_TRIANGLE_TIE_PRIORITY;
          sample.color = sample_color;
          if let Some(surface_distances) = bevel_surface_distances.as_mut() {
            surface_distances[sample_index] = bevel_surface_distance_px;
          }
        }
      }
    }
  }

  if let (
    Some(bevel),
    Some(profile_strips),
    Some(direct_samples),
    Some(topology_samples),
    Some(material_cells),
    Some(boundary_index),
  ) = (
    direct_inset_bevel,
    direct_inset_profile_strips.as_deref(),
    direct_inset_samples.as_deref(),
    direct_inset_topology.as_deref(),
    direct_inset_material_cells.as_ref(),
    direct_inset_boundary_index.as_ref(),
  ) {
    // Microsoft's direct-inset bevel algorithm constructs the complete inset
    // graph before triangulation, including merges, splits, and interacting
    // outer/inner glyph contours. When available, that graph alone admits a
    // surface sample and supplies its camera-visible source owner. The unsplit raw
    // cells remain only attribute witnesses: their texture, diffuse, and
    // specular values are interpolated without being recomputed at every
    // graph event. If graph construction is unsupported, retain the bounded
    // complete-outline reconstruction as the portable fallback.
    let center = (center_x, center_y);
    for sample_index in 0..samples.len() {
      let donor = &direct_samples[sample_index];
      let graph_sample = direct_inset_graph_samples
        .as_deref()
        .map(|graph_samples| &graph_samples[sample_index]);
      let graph_witness = graph_sample.map(|graph| &graph.surface);
      if graph_witness.is_some_and(|graph| !graph.covered) {
        continue;
      }
      let pixel_index = sample_index / sample_count;
      let subpixel = sample_index % sample_count;
      let local_pixel_x = pixel_index % raster_width;
      let local_pixel_y = pixel_index / raster_width;
      let (sample_x, sample_y) = sample_pattern.position(subpixel);
      let raster_point = (
        left as f32 + local_pixel_x as f32 + sample_x,
        top as f32 + local_pixel_y as f32 + sample_y,
      );
      let point = rasterization.raster_to_surface(raster_point, destination_dimensions);
      let graph_owner = graph_sample.and_then(|graph| graph.owner);
      let raw_topology = topology_samples[sample_index];
      let fallback = projection_preserves_source_plane_coverage(projection)
        .then(|| {
          boundary_index
            .owner_at(source_geometry, point)
            .map(|owner| (owner, point))
        })
        .flatten();
      let Some((boundary, source_point)) = graph_owner
        .map(|owner| (owner.boundary, owner.source_point))
        .or_else(|| raw_topology.owner.zip(raw_topology.source_point))
        .or(fallback)
      else {
        continue;
      };
      let Some(mut surface_sample) = text_direct_inset_surface_sample(
        bevel,
        profile_strips,
        source_point,
        center,
        boundary.distance,
        graph_owner.map(|owner| owner.profile_strip_index),
      ) else {
        continue;
      };
      if let (Some(owner), Some(witness)) = (graph_owner, graph_witness) {
        // Depth belongs to the actual projected, snapped triangle. Profile
        // interpolation above is only for its material coordinates; it must
        // not select another branch or reproject an unsnapped source point.
        surface_sample.visibility_depth = witness.visibility_depth;
        surface_sample.surface_distance_px = owner.surface_distance_px;
      }
      if !text_geometry_contains(source_geometry, source_point) {
        continue;
      }
      // Final-grid graph samples already carry the source-cell shader result
      // evaluated at their winning primitive's centroid. Keep that result
      // paired with the graph depth; re-evaluating at this coverage sample
      // would silently turn MSAA into supersampled material shading.
      let fragment_color = (!source_grid)
        .then(|| {
          graph_witness
            .filter(|graph| graph.covered)
            .map(|graph| graph.color)
        })
        .flatten();
      let material_color = match fragment_color.or_else(|| {
        material_cells.color_at(
          triangles,
          material_images,
          material_alpha_mask.as_ref(),
          material,
          TextDirectInsetMaterialPoint {
            boundary,
            surface: surface_sample,
            point,
          },
        )
      }) {
        Some(color) => color,
        None if donor.covered => donor.color,
        None if graph_witness.is_some_and(|graph| graph.covered) => {
          graph_witness.expect("covered graph witness").color
        }
        None => continue,
      };
      let sample = &mut samples[sample_index];
      if material_images.surface_material_texture.is_some() && material_color[3] < 1.0 {
        continue;
      }
      if sample.covered && surface_sample.visibility_depth <= sample.visibility_depth {
        continue;
      }
      // Office submits solid contour geometry without blending, then enables
      // premultiplied SourceOver for the material pass. A nearer translucent
      // bevel must retain the already visible contour/side colour. Resolve the
      // bevel's topology and nearest owner first, so overlapping candidate
      // triangles cannot accumulate the same material more than once.
      // Shape lighting has a separate surface/alpha contract.
      let material_color = if geometry_lighting == Static3dGeometryLighting::Text && sample.covered
      {
        text_material_surface_over(material_color, sample.color)
      } else {
        material_color
      };
      sample.covered = true;
      sample.visibility_depth = surface_sample.visibility_depth;
      sample.color = material_color;
      sample.tie_priority = TEXT_SURFACE_TRIANGLE_TIE_PRIORITY;
      if let Some(surface_distances) = bevel_surface_distances.as_mut() {
        surface_distances[sample_index] = surface_sample.surface_distance_px;
      }
    }
  }

  if let (Some(contour), Some(contour_area_mask), Some(contour_depth_samples)) = (
    contour,
    contour_area_mask.as_ref(),
    contour_depth_samples.as_mut(),
  ) {
    // ECMA-376 makes the contour an unlit solid-filled line. Exact-config
    // Word controls additionally pin its front surface as a centered raised
    // band that participates in the same depth buffer as the bevel. The
    // quadratic cap below is the current bounded fallback: corrected analysis
    // of the 720-case Angle matrix and a separate 1,001-value sweep do not pin
    // its 1.1 peak coefficient, and varying that coefficient cannot explain
    // the remaining PASS residual. Keep source alpha on the independent
    // AreaA8 path: this surface owns RGB/depth only.
    let radius = contour.width_px * 0.5;
    if radius > f32::EPSILON && contour.color.alpha != 0 {
      let contour_left = contour_area_mask.left.max(left);
      let contour_top = contour_area_mask.top.max(top);
      let contour_right = (contour_area_mask.left + contour_area_mask.width as i32).min(right);
      let contour_bottom = (contour_area_mask.top + contour_area_mask.height as i32).min(bottom);
      for pixel_y in contour_top..contour_bottom {
        for pixel_x in contour_left..contour_right {
          if contour_area_mask.alpha_at(pixel_x, pixel_y) == 0 {
            continue;
          }
          let local_pixel = (pixel_y - top) as usize * raster_width + (pixel_x - left) as usize;
          for sample_index_in_pixel in 0..sample_count {
            let (sample_x, sample_y) = sample_pattern.position(sample_index_in_pixel);
            let source_point = (pixel_x as f32 + sample_x, pixel_y as f32 + sample_y);
            // WPF converts the physical mesh from half-pixel-center device
            // coordinates to Direct3D integer-pixel centers. Its 2-D
            // antialiased edge path explicitly reverses that conversion
            // before resolving a semantic line. Word's contour is that
            // line: retain the frozen physical mesh, but evaluate the
            // centered raised band in the original antialiased phase.
            let contour_point = (
              source_point.0 - contour_depth_phase_px,
              source_point.1 - contour_depth_phase_px,
            );
            let boundary_distance = text_geometry_boundary_distance(source_geometry, contour_point);
            if boundary_distance > radius {
              continue;
            }
            let normalized_distance = (boundary_distance / radius).clamp(0.0, 1.0);
            let surface_z = contour.base_z
              + radius
                * WORD_CONTOUR_HEIGHT_OVER_RADIUS
                * (1.0 - normalized_distance * normalized_distance);
            let model_point = [
              source_point.0 - center_x,
              source_point.1 - center_y,
              surface_z,
            ];
            let visibility_depth =
              text_surface_visibility_depth(projection, model_point, pixels_per_point);
            let sample_index = local_pixel * sample_count + sample_index_in_pixel;
            contour_depth_samples[sample_index] = visibility_depth;
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
  let source_mask = if uniform_paint_opacity.is_some() {
    None
  } else {
    text_geometry_path(paint_opacity_geometry, |point| point)
      .and_then(|path| text_geometry_mask(source.width(), source.height(), &path))
  };
  if let Some(planar_inverse) = planar_inverse
    && (uniform_paint_opacity.is_some() || source_mask.is_some())
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
        let center_source = if geometry_lighting == Static3dGeometryLighting::Shape {
          let raster_target = (
            pixel_x as f32 + pixel_center_offset,
            pixel_y as f32 + pixel_center_offset,
          );
          let target = rasterization.raster_to_surface(raster_target, destination_dimensions);
          let source_local =
            map_homogeneous(planar_inverse, target.0 - center_x, target.1 - center_y);
          Some((
            (center_x + source_local.0, center_y + source_local.1),
            source_local,
          ))
        } else {
          None
        };
        let pixel_color = if geometry_lighting == Static3dGeometryLighting::Shape {
          (|| {
            let (source_point, source_local) = center_source?;
            let material_point = source_geometry.map_point_to(material_geometry, source_point);
            let mut color =
              sample_bilinear(source, material_point.0 - 0.5, material_point.1 - 0.5)?;
            let paint_opacity = sample_text_paint_opacity(
              source,
              source_mask.as_ref(),
              uniform_paint_opacity,
              source_geometry,
              paint_opacity_geometry,
              source_point,
            )?;
            if paint_opacity <= f32::EPSILON {
              return None;
            }
            let view_direction = surface_view_direction(
              scene,
              projection,
              [source_local.0, source_local.1, planar_z],
              model_width,
              model_height,
              pixels_per_point,
            );
            color[3] = material_surface_alpha(
              paint_opacity,
              planar_surface_normal,
              view_direction,
              material,
            );
            let color = color.0.map(f32::from);
            Some(color)
          })()
        } else {
          None
        };
        for sample_index_in_pixel in 0..sample_count {
          let (sample_x, sample_y) = sample_pattern.position(sample_index_in_pixel);
          let raster_target = (pixel_x as f32 + sample_x, pixel_y as f32 + sample_y);
          let target = rasterization.raster_to_surface(raster_target, destination_dimensions);
          let source_local =
            map_homogeneous(planar_inverse, target.0 - center_x, target.1 - center_y);
          let source_point = (center_x + source_local.0, center_y + source_local.1);
          let model_point = [source_local.0, source_local.1, planar_z];
          if !planar_inset.contains(planar_geometry, source_point, planar_inset_px) {
            continue;
          }
          let sample_color = if let Some(color) = pixel_color {
            color
          } else {
            let material_point =
              source_geometry.map_point_to(planar_material_geometry, source_point);
            let material_endpoint = |material_source: &RgbaImage,
                                     uniform_material_opacity: Option<f32>,
                                     has_authored_transparency: bool,
                                     source| {
              let pixel = sample_bilinear(
                material_source,
                material_point.0 - 0.5,
                material_point.1 - 0.5,
              )?;
              let paint_opacity = sample_text_paint_opacity(
                material_source,
                source_mask.as_ref(),
                uniform_material_opacity.or(uniform_paint_opacity),
                source_geometry,
                paint_opacity_geometry,
                source_point,
              )?;
              (paint_opacity > f32::EPSILON).then_some(TextBevelMaterialEndpoint {
                source,
                source_point,
                material_point,
                outward: [0.0, 0.0],
                source_color: [pixel[0], pixel[1], pixel[2]],
                paint_opacity,
                alpha_texture_opacity: if has_authored_transparency {
                  paint_opacity
                } else {
                  0.0
                },
              })
            };
            let base = if let Some(material) = planar_fill_material {
              material_endpoint(
                material,
                planar_fill_uniform_paint_opacity,
                fill_has_authored_transparency,
                TextSurfaceMaterialSource::Independent,
              )
            } else if planar_outline_material.is_none() {
              material_endpoint(
                source,
                uniform_paint_opacity,
                fill_has_authored_transparency || outline_has_authored_transparency,
                TextSurfaceMaterialSource::Combined,
              )
            } else {
              None
            };
            let outline = planar_outline_material
              .zip(planar_outline_coverage)
              .filter(|_| {
                planar_outline_material_inset_px.is_some_and(|inset_px| {
                  text_geometry_boundary_distance(source_geometry, source_point)
                    <= inset_px + 1.0 + f32::EPSILON
                })
              })
              .and_then(|(material, coverage)| {
                let endpoint = material_endpoint(
                  material,
                  planar_outline_uniform_paint_opacity,
                  outline_has_authored_transparency,
                  TextSurfaceMaterialSource::Independent,
                )?;
                let coverage =
                  sample_bilinear(coverage, material_point.0 - 0.5, material_point.1 - 0.5)?;
                let coverage = f32::from(coverage[3]) / 255.0;
                (coverage > f32::EPSILON).then_some((endpoint, coverage))
              });
            let Some(composed) = composite_text_bevel_material_endpoint(base, outline) else {
              continue;
            };
            let alpha_mask = match composed.source {
              TextSurfaceMaterialSource::Combined => material_alpha_mask.as_ref(),
              TextSurfaceMaterialSource::Independent => independent_alpha_mask.as_ref(),
            };
            let texture_alpha = alpha_mask.map_or(1.0, |texture| {
              sample_text_geometry_area_mask(
                texture,
                material_point.0 - 0.5,
                material_point.1 - 0.5,
              )
            });
            let paint_opacity = (composed.paint_opacity
              - composed.alpha_texture_opacity * (1.0 - texture_alpha))
              .clamp(0.0, 1.0);
            let view_direction = surface_view_direction(
              scene,
              projection,
              model_point,
              model_width,
              model_height,
              pixels_per_point,
            );
            let alpha = material_surface_alpha(
              paint_opacity,
              planar_surface_normal,
              view_direction,
              material,
            );
            // Independent images are unlit paint inputs. Compose the paint
            // before lighting: clamping each lit input before source-over
            // loses the highlight contribution on translucent outlines.
            // Word's black/white fill x outline controls distinguish these
            // orders even with identical geometry and alpha planes.
            let color = if composed.source == TextSurfaceMaterialSource::Independent {
              let diffuse =
                material_diffuse_shade(scene, planar_surface_normal, view_direction, material);
              let specular = light_rig_surface_specular(
                scene,
                planar_surface_normal,
                view_direction,
                material,
                composed.source_color,
              );
              std::array::from_fn(|channel| {
                shade_gouraud_channel_with_specular(
                  composed.source_color[channel],
                  diffuse[channel],
                  specular[channel],
                )
              })
            } else {
              composed.source_color
            };
            [
              f32::from(color[0]),
              f32::from(color[1]),
              f32::from(color[2]),
              f32::from(alpha),
            ]
          };
          let visibility_depth =
            text_surface_visibility_depth(projection, model_point, pixels_per_point);
          let sample_index = local_pixel * sample_count + sample_index_in_pixel;
          let sample = &mut samples[sample_index];
          if sample.covered && visibility_depth <= sample.visibility_depth {
            continue;
          }
          sample.covered = true;
          sample.visibility_depth = visibility_depth;
          sample.tie_priority = TEXT_SURFACE_PLANAR_TIE_PRIORITY;
          sample.color = sample_color;
          if let Some(surface_distances) = bevel_surface_distances.as_mut() {
            surface_distances[sample_index] = f32::INFINITY;
          }
        }
      }
    }
  }

  if let (Some(contour), Some(contour_area_mask), Some(contour_surface_samples)) = (
    contour,
    contour_area_mask.as_ref(),
    contour_surface_samples.as_mut(),
  ) {
    // Microsoft's 2-D/3-D Office pipeline (US7999807B2) generates the
    // contour from the underlying outline and then applies that contour to
    // extrusion and beveling. It is therefore not an independent raised cap:
    // the outer half of the centered line supplies new front geometry, while
    // the inner half follows the authored bevel for the same physical
    // distance. The profile-distance attribute above keeps folded presets on
    // the actually visible branch instead of reducing them to one inset.
    //
    // Preserve the independently pinned phase-0 AreaA8 line as the absolute
    // RGB area. These 8x4 samples only provide its conditional surface
    // visibility, exactly as the no-bevel controls require.
    let radius = contour.width_px * 0.5;
    if radius > f32::EPSILON && contour.color.alpha != 0 {
      let contour_left = contour_area_mask.left.max(left);
      let contour_top = contour_area_mask.top.max(top);
      let contour_right = (contour_area_mask.left + contour_area_mask.width as i32).min(right);
      let contour_bottom = (contour_area_mask.top + contour_area_mask.height as i32).min(bottom);
      for pixel_y in contour_top..contour_bottom {
        for pixel_x in contour_left..contour_right {
          if contour_area_mask.alpha_at(pixel_x, pixel_y) == 0 {
            continue;
          }
          let local_pixel = (pixel_y - top) as usize * raster_width + (pixel_x - left) as usize;
          for sample_index_in_pixel in 0..sample_count {
            let (sample_x, sample_y) = sample_pattern.position(sample_index_in_pixel);
            let point = (pixel_x as f32 + sample_x, pixel_y as f32 + sample_y);
            if text_geometry_boundary_distance(source_geometry, point) > radius {
              continue;
            }
            let sample_index = local_pixel * sample_count + sample_index_in_pixel;
            let inside = text_geometry_contains(source_geometry, point);
            let visible = !inside
              || bevel_surface_distances
                .as_ref()
                .is_none_or(|distances| distances[sample_index] <= radius);
            contour_surface_samples[sample_index] = TextContourSurfaceSample {
              active: true,
              visible,
            };
          }
        }
      }
    }
  }

  let silhouette_coverage_masks =
    (geometry_lighting == Static3dGeometryLighting::Shape).then(|| {
      let surface_masks = samples
        .chunks_exact(sample_count)
        .enumerate()
        .map(|(pixel_index, pixel_samples)| {
          let contour_depths = contour_depth_samples.as_ref().map(|depths| {
            let start = pixel_index * sample_count;
            &depths[start..start + sample_count]
          });
          pixel_samples
            .iter()
            .enumerate()
            .fold(0_u32, |mask, (index, sample)| {
              let covered = sample.covered
                || contour_depths.is_some_and(|depths| contour_depth_wins(sample, depths[index]));
              mask | (u32::from(covered) << index)
            })
        })
        .collect::<Vec<_>>();
      translate_text_surface_coverage_masks(
        &surface_masks,
        raster_width,
        raster_height,
        (
          projection.silhouette_translation_x_px,
          projection.silhouette_translation_y_px,
        ),
      )
    });
  let mut area_resolved_samples = planar_area_mask
    .as_ref()
    .map(|_| vec![None; raster_width * raster_height]);
  let resolved_contour_color = contour.map(|contour| {
    [
      f32::from(contour.color.color.r),
      f32::from(contour.color.color.g),
      f32::from(contour.color.color.b),
      f32::from(contour.color.alpha),
    ]
  });

  for local_y in 0..raster_height {
    for local_x in 0..raster_width {
      let pixel_index = local_y * raster_width + local_x;
      let sample_start = pixel_index * sample_count;
      let sample_end = sample_start + sample_count;
      let contour_samples = contour_surface_samples
        .as_ref()
        .map(|samples| TextContourSamples::Surface(&samples[sample_start..sample_end]))
        .or_else(|| {
          contour_depth_samples
            .as_ref()
            .map(|depths| TextContourSamples::Depth(&depths[sample_start..sample_end]))
        });
      let pixel_x = left + local_x as i32;
      let pixel_y = top + local_y as i32;
      let contour_area_alpha = contour_area_mask
        .as_ref()
        .map_or(0, |mask| mask.alpha_at(pixel_x, pixel_y));
      let source_area_alpha = contour_source_area_mask
        .as_ref()
        .map(|mask| mask.alpha_at(pixel_x, pixel_y));
      let Some(resolved) = resolve_text_surface_pixel(
        &samples[sample_start..sample_end],
        contour_samples,
        contour_area_alpha,
        source_area_alpha,
        resolved_contour_color,
      ) else {
        continue;
      };
      // Resolve authored opacity and geometric coverage independently. Office
      // Screen controls with a nonzero top inset retain fractional SMask
      // values at the two terminal bevel corners; promoting every majority
      // pixel to opaque creates a disconnected outer extrusion island once
      // the inset crosses a pixel boundary.
      let alpha = if let Some(area_mask) = planar_area_mask.as_ref() {
        resolved.authored_opacity * f32::from(area_mask.alpha_at(pixel_x, pixel_y)) / 255.0
      } else if let Some(silhouette_masks) = silhouette_coverage_masks.as_ref() {
        resolved.authored_opacity * silhouette_masks[pixel_index].count_ones() as f32
          / sample_count as f32
      } else {
        resolved.alpha
      };
      let color = Rgba([
        resolved.rgb[0],
        resolved.rgb[1],
        resolved.rgb[2],
        (alpha * 255.0).round().clamp(0.0, 255.0) as u8,
      ]);
      if color[3] == 0 {
        continue;
      }
      if let Some(samples) = area_resolved_samples.as_mut() {
        samples[pixel_index] = Some(TextAreaResolvedSample {
          rgb: resolved.rgb,
          authored_opacity: resolved.authored_opacity,
        });
      }
      blend_over(
        destination.get_pixel_mut(
          (left as usize + local_x) as u32,
          (top as usize + local_y) as u32,
        ),
        color,
      );
    }
  }

  if let (Some(area_mask), Some(resolved_samples)) =
    (planar_area_mask.as_ref(), area_resolved_samples.as_ref())
  {
    let default_sample = resolved_samples.iter().flatten().next().copied();
    for local_y in 0..raster_height {
      for local_x in 0..raster_width {
        let pixel_index = local_y * raster_width + local_x;
        if resolved_samples[pixel_index].is_some() {
          continue;
        }
        let alpha = area_mask.alpha_at(left + local_x as i32, top + local_y as i32);
        if alpha == 0 {
          continue;
        }
        let Some(sample) = nearest_text_area_resolved_sample(
          resolved_samples,
          raster_width,
          raster_height,
          local_x,
          local_y,
        )
        .or(default_sample) else {
          continue;
        };
        let alpha = (f32::from(alpha) * sample.authored_opacity)
          .round()
          .clamp(0.0, 255.0) as u8;
        if alpha == 0 {
          continue;
        }
        blend_over(
          destination.get_pixel_mut(
            (left as usize + local_x) as u32,
            (top as usize + local_y) as u32,
          ),
          Rgba([sample.rgb[0], sample.rgb[1], sample.rgb[2], alpha]),
        );
      }
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct TextBevelProfileStrip {
  outer: BevelProfileSample,
  inner: BevelProfileSample,
  middle: BevelProfileSample,
  outer_surface_distance_px: f32,
  inner_surface_distance_px: f32,
}

fn interpolate_bevel_profile_sample(
  first: BevelProfileSample,
  second: BevelProfileSample,
  parameter: f32,
) -> BevelProfileSample {
  BevelProfileSample {
    height: first.height + (second.height - first.height) * parameter,
    inset: first.inset + (second.inset - first.inset) * parameter,
    height_tangent: first.height_tangent
      + (second.height_tangent - first.height_tangent) * parameter,
    inset_tangent: first.inset_tangent + (second.inset_tangent - first.inset_tangent) * parameter,
  }
}

fn text_bevel_profile_strips(
  normal_width: f32,
  height: f32,
  preset: Option<a::BevelPresetValues>,
  geometry_lighting: Static3dGeometryLighting,
  outline_material_inset_px: Option<f32>,
  outline_material_filter_px: Option<f32>,
  realized_profile_tolerance_px: Option<f32>,
) -> (bool, Vec<TextBevelProfileStrip>) {
  let profile = bevel_profile(preset);
  let shape_circle_lighting = geometry_lighting == Static3dGeometryLighting::Shape
    && preset.unwrap_or(a::BevelPresetValues::Circle) == a::BevelPresetValues::Circle;
  // US7639249B2 describes the Office direct-inset contract: the profile object
  // flattens the authored profile, computes a normal at every flattened point,
  // and passes consecutive point/normal pairs to the insetter. WPF's
  // CBezierFlattener supplies the hybrid-forward-differencing algorithm, not
  // Word's precision policy. Perspective Word text realizes profile precision
  // from the projected solid, independently of source bitmap density. Keep
  // the older non-realized routes separate. Each authored segment keeps
  // its two endpoint tangents instead of being smoothed across the join.
  //
  // Shape-level Circle is independently pinned to the tighter 8x material
  // sampling below: its controlled light extremum lies inside a 1/16 profile
  // chord and cannot be reconstructed by Gouraud interpolation alone.
  let uniform_subdivisions = if shape_circle_lighting {
    (normal_width * 8.0).ceil().clamp(16.0, 4_096.0) as usize
  } else {
    (normal_width * 1.5 / profile.len() as f32)
      .ceil()
      .clamp(4.0, 16.0) as usize
  };
  let mut profile_strips = Vec::new();
  let mut previous_profile = None::<BevelProfileSample>;
  let mut surface_distance_px = 0.0_f32;
  for segment_index in 0..profile.len() {
    let mut parameters = if geometry_lighting == Static3dGeometryLighting::Text {
      word_text_bevel_segment_parameters(
        normal_width,
        height,
        preset,
        segment_index,
        realized_profile_tolerance_px.unwrap_or(TEXT_BEVEL_PROFILE_FLATTENING_TOLERANCE_PX),
      )
    } else {
      (0..=uniform_subdivisions)
        .map(|subdivision| subdivision as f32 / uniform_subdivisions as f32)
        .collect()
    };
    if geometry_lighting == Static3dGeometryLighting::Text
      && normal_width > f32::EPSILON
      && let Some(inset_px) = outline_material_inset_px
    {
      let filter_px = outline_material_filter_px.unwrap_or(0.0).max(0.0);
      for boundary_px in [inset_px, inset_px + filter_px] {
        parameters.extend(word_text_bevel_material_boundary_parameters(
          preset,
          segment_index,
          boundary_px / normal_width,
        ));
      }
      parameters.sort_by(f32::total_cmp);
      parameters.dedup_by(|left, right| (*left - *right).abs() <= 1.0e-6);
    }
    for interval in parameters.windows(2) {
      let outer_t = interval[0];
      let inner_t = interval[1];
      let middle_t = (outer_t + inner_t) * 0.5;
      let authored_outer = bevel_profile_sample(preset, segment_index, outer_t);
      let authored_inner = bevel_profile_sample(preset, segment_index, inner_t);
      let authored_middle = bevel_profile_sample(preset, segment_index, middle_t);
      let (outer, inner, middle) = if geometry_lighting == Static3dGeometryLighting::Text {
        (
          word_text_bevel_geometry_profile(preset, authored_outer),
          word_text_bevel_geometry_profile(preset, authored_inner),
          word_text_bevel_geometry_profile(preset, authored_middle),
        )
      } else {
        (authored_outer, authored_inner, authored_middle)
      };
      if let Some(previous) = previous_profile {
        surface_distance_px += ((outer.inset - previous.inset) * normal_width)
          .hypot((outer.height - previous.height) * height);
      }
      let outer_surface_distance_px = surface_distance_px;
      surface_distance_px +=
        ((inner.inset - outer.inset) * normal_width).hypot((inner.height - outer.height) * height);
      profile_strips.push(TextBevelProfileStrip {
        outer,
        inner,
        middle,
        outer_surface_distance_px,
        inner_surface_distance_px: surface_distance_px,
      });
      previous_profile = Some(inner);
    }
  }
  (
    geometry_lighting == Static3dGeometryLighting::Text || shape_circle_lighting,
    profile_strips,
  )
}

struct TextBevelMesh {
  triangles: Vec<TextSurfaceTriangle>,
  direct_inset_cells: Option<Vec<DirectInsetCell>>,
}

impl TextBevelMesh {
  fn empty() -> Self {
    Self {
      triangles: Vec::new(),
      direct_inset_cells: None,
    }
  }
}

fn text_bevel_triangles(
  source: &RgbaImage,
  geometry: &Static3dTextGeometry,
  material_geometry: &Static3dTextGeometry,
  paint_opacity_geometry: &Static3dTextGeometry,
  uniform_paint_opacity: Option<f32>,
  options: TextBevelOptions<'_>,
) -> TextBevelMesh {
  let material_context = TextBevelMaterialContext { geometry, options };
  let TextBevelOptions {
    geometry_width,
    normal_width,
    height,
    preset,
    projection,
    model_surface,
    pixels_per_point,
    surface_z,
    geometry_lighting,
    back_face,
    height_direction,
    ..
  } = options;
  if geometry_width <= f32::EPSILON && height <= f32::EPSILON {
    return TextBevelMesh::empty();
  }
  let source_mask = if uniform_paint_opacity.is_some() {
    None
  } else {
    text_geometry_path(paint_opacity_geometry, |point| point)
      .and_then(|path| text_geometry_mask(source.width(), source.height(), &path))
  };
  if uniform_paint_opacity.is_none() && source_mask.is_none() {
    return TextBevelMesh::empty();
  }
  let center_x = model_surface.left_px + model_surface.width_px * 0.5;
  let center_y = model_surface.top_px + model_surface.height_px * 0.5;
  let model_width = model_surface.width_px.max(1.0);
  let model_height = model_surface.height_px.max(1.0);
  let project = |point: (f32, f32),
                 z,
                 shaded: ShadedTextBevelMaterialEndpoint,
                 use_textured_material: bool,
                 inset_px: f32,
                 surface_distance_px: f32| {
    // Physical position and material/collision attributes refer to the same
    // continuous surface. Screen-space quantization happens later.
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
      color: shaded.color.map(f32::from),
      textured_material: use_textured_material
        .then_some(shaded.textured_material)
        .flatten(),
      bevel_collision: Some(TextBevelCollisionVertex {
        source_point: point,
        inset_px,
        surface_distance_px,
      }),
    }
  };
  // MS-OI29500 publishes every Office preset as a sequence of bevel-space
  // curves. Keep those curves parametric: `relaxedInset`, `softRound`, and
  // several other presets fold back in y, so reducing them to one height per
  // alpha-mask distance discards a complete visible surface. The shared mesh
  // is depth-tested below, so folded branches retain their authored order in
  // profile space while camera-space visibility decides the output sample.
  let (interpolate_profile_lighting, profile_strips) = text_bevel_profile_strips(
    normal_width,
    height,
    preset,
    geometry_lighting,
    options.outline_material_inset_px,
    options.outline_coverage.map(|_| 1.0),
    geometry.bevel_profile_tolerance_px,
  );
  #[derive(Clone, Copy)]
  struct TextBevelMeshCell {
    contour_index: usize,
    edge_index: usize,
    profile_strip_index: usize,
    outer_profile: BevelProfileSample,
    inner_profile: BevelProfileSample,
    middle_profile: BevelProfileSample,
    outer_surface_distance_px: f32,
    inner_surface_distance_px: f32,
    outer_points: [(f32, f32); 2],
    inner_points: [(f32, f32); 2],
    normals: ([f32; 2], [f32; 2]),
    direct_inset_graph: bool,
  }

  let contour_edge_normals = geometry
    .contours
    .iter()
    .map(|contour| {
      text_3d_contour_edge_normals(
        &contour.points,
        &contour.incoming_curve_edges,
        geometry.solid_on_right,
        Text3dNormalJoinPolicy::SourceCurves,
      )
    })
    .collect::<Vec<_>>();

  // US7639249B2 requires the complete inset graph to precede profile
  // triangulation. The graph is constructed entirely in the source plane;
  // camera projection happens only after its event cells have been resolved.
  // Consequently the same graph owns parallel and perspective text. Shape
  // meshes retain their independently calibrated path.
  let profile_insets = profile_strips
    .iter()
    .flat_map(|strip| [strip.outer.inset, strip.inner.inset])
    .map(|inset| geometry_width * inset)
    .collect::<Vec<_>>();
  let maximum_inset = profile_insets.iter().copied().fold(0.0_f32, f32::max);
  let graph_is_supported = geometry_lighting == Static3dGeometryLighting::Text
    && maximum_inset > f32::EPSILON
    && profile_insets.iter().all(|inset| *inset >= -1.0e-5);
  let contour_points = geometry
    .contours
    .iter()
    .map(|contour| contour.points.as_slice())
    .collect::<Vec<_>>();
  let direct_graph_cells = graph_is_supported
    .then(|| direct_inset_cells(&contour_points, geometry.solid_on_right, maximum_inset))
    .flatten()
    .filter(|cells| !cells.is_empty());

  let mut mesh_cells = Vec::<TextBevelMeshCell>::new();
  if let Some(graph_cells) = direct_graph_cells.as_deref() {
    let boundary_points_at = |cell: &DirectInsetCell, inset: f32| {
      let span = cell.inner.inset - cell.outer.inset;
      let parameter = if span.abs() <= f32::EPSILON {
        0.0
      } else {
        ((inset - cell.outer.inset) / span).clamp(0.0, 1.0)
      };
      std::array::from_fn(|index| {
        let outer = cell.outer.endpoints[index].point;
        let inner = cell.inner.endpoints[index].point;
        (
          outer.0 + (inner.0 - outer.0) * parameter,
          outer.1 + (inner.1 - outer.1) * parameter,
        )
      })
    };
    for (profile_strip_index, strip) in profile_strips.iter().enumerate() {
      let strip_outer_inset = geometry_width * strip.outer.inset;
      let strip_inner_inset = geometry_width * strip.inner.inset;
      let strip_minimum = strip_outer_inset.min(strip_inner_inset);
      let strip_maximum = strip_outer_inset.max(strip_inner_inset);
      let strip_span = strip_inner_inset - strip_outer_inset;
      if strip_span.abs() <= f32::EPSILON {
        continue;
      }
      for graph_cell in graph_cells {
        let overlap_minimum = graph_cell.outer.inset.max(strip_minimum);
        let overlap_maximum = graph_cell.inner.inset.min(strip_maximum);
        if overlap_maximum <= overlap_minimum + 1.0e-5 {
          continue;
        }
        let (outer_inset, inner_inset) = if strip_span > 0.0 {
          (overlap_minimum, overlap_maximum)
        } else {
          (overlap_maximum, overlap_minimum)
        };
        let outer_parameter = ((outer_inset - strip_outer_inset) / strip_span).clamp(0.0, 1.0);
        let inner_parameter = ((inner_inset - strip_outer_inset) / strip_span).clamp(0.0, 1.0);
        let middle_parameter = (outer_parameter + inner_parameter) * 0.5;
        let outer_profile =
          interpolate_bevel_profile_sample(strip.outer, strip.inner, outer_parameter);
        let inner_profile =
          interpolate_bevel_profile_sample(strip.outer, strip.inner, inner_parameter);
        let middle_profile =
          interpolate_bevel_profile_sample(strip.outer, strip.inner, middle_parameter);
        let surface_distance = |parameter: f32| {
          strip.outer_surface_distance_px
            + (strip.inner_surface_distance_px - strip.outer_surface_distance_px) * parameter
        };
        let contour_index = graph_cell.source_edge.contour_index;
        let edge_index = graph_cell.source_edge.edge_index;
        let Some(&normals) = contour_edge_normals
          .get(contour_index)
          .and_then(|normals| normals.get(edge_index))
        else {
          continue;
        };
        debug_assert_eq!(
          graph_cell
            .outer
            .endpoints
            .map(|endpoint| endpoint.source_vertex),
          graph_cell
            .inner
            .endpoints
            .map(|endpoint| endpoint.source_vertex)
        );
        mesh_cells.push(TextBevelMeshCell {
          contour_index,
          edge_index,
          profile_strip_index,
          outer_profile,
          inner_profile,
          middle_profile,
          outer_surface_distance_px: surface_distance(outer_parameter),
          inner_surface_distance_px: surface_distance(inner_parameter),
          outer_points: boundary_points_at(graph_cell, outer_inset),
          inner_points: boundary_points_at(graph_cell, inner_inset),
          normals,
          direct_inset_graph: true,
        });
      }
    }
  }

  // Keep one unsplit cell per source edge/profile strip as the material
  // attribute mesh. The complete inset graph owns coverage and collision
  // topology, but an event subdivision must not resample or relight an
  // otherwise unchanged source face. This is the same position/attribute
  // separation used by the fixed-function pipeline: graph vertices can split
  // geometry while texture, diffuse, and specular values remain attached to
  // their originating source-edge cells.
  for (contour_index, contour) in geometry.contours.iter().enumerate() {
    for (profile_strip_index, strip) in profile_strips.iter().enumerate() {
      let outer_ring = offset_text_3d_contour(
        &contour.points,
        geometry_width * strip.outer.inset,
        geometry.solid_on_right,
      );
      let inner_ring = offset_text_3d_contour(
        &contour.points,
        geometry_width * strip.inner.inset,
        geometry.solid_on_right,
      );
      for edge_index in 0..contour.points.len() {
        let next = (edge_index + 1) % contour.points.len();
        mesh_cells.push(TextBevelMeshCell {
          contour_index,
          edge_index,
          profile_strip_index,
          outer_profile: strip.outer,
          inner_profile: strip.inner,
          middle_profile: strip.middle,
          outer_surface_distance_px: strip.outer_surface_distance_px,
          inner_surface_distance_px: strip.inner_surface_distance_px,
          outer_points: [outer_ring[edge_index], outer_ring[next]],
          inner_points: [inner_ring[edge_index], inner_ring[next]],
          normals: contour_edge_normals[contour_index][edge_index],
          direct_inset_graph: false,
        });
      }
    }
  }

  let mut triangles = Vec::new();
  for cell in mesh_cells {
    let contour_index = cell.contour_index;
    let index = cell.edge_index;
    let profile_strip_index = cell.profile_strip_index;
    let outer_profile = cell.outer_profile;
    let inner_profile = cell.inner_profile;
    let middle_profile = cell.middle_profile;
    let normals = cell.normals;
    // `w14:textOutline` is an independently painted centered line.  Its
    // outward half is clipped by the raw-glyph solid; its inward half keeps
    // a fixed material width while the bevel beneath it may be arbitrarily
    // wide. The profile splitter above places exact strip boundaries in the
    // published geometric-inset coordinate for every preset. Select the independently rasterized outline
    // paint for only the outer strip instead of sampling the combined
    // fill+outline bitmap at every bevel midpoint.
    let separate_outline_material = options.outline_material.zip(options.outline_coverage);
    let (material_source, material_source_has_authored_transparency) =
      if separate_outline_material.is_some() {
        (
          options.fill_material.unwrap_or(source),
          if options.fill_material.is_some() {
            options.fill_has_authored_transparency
          } else {
            options.fill_has_authored_transparency || options.outline_has_authored_transparency
          },
        )
      } else if let Some(outline) = options.outline_material.filter(|_| {
        options
          .outline_material_inset_px
          .is_some_and(|inset_px| geometry_width * middle_profile.inset <= inset_px + f32::EPSILON)
      }) {
        (outline, options.outline_has_authored_transparency)
      } else if let Some(fill) = options.fill_material {
        (fill, options.fill_has_authored_transparency)
      } else {
        (
          source,
          options.fill_has_authored_transparency || options.outline_has_authored_transparency,
        )
      };
    let [outer_first, outer_second] = cell.outer_points;
    let [inner_first, inner_second] = cell.inner_points;
    let source_point = (
      (outer_first.0 + outer_second.0 + inner_second.0 + inner_first.0) * 0.25,
      (outer_first.1 + outer_second.1 + inner_second.1 + inner_first.1) * 0.25,
    );
    if separate_outline_material.is_none() {
      let Some(paint_opacity) = sample_text_paint_opacity_clamped(
        material_source,
        source_mask.as_ref(),
        uniform_paint_opacity,
        geometry,
        paint_opacity_geometry,
        source_point,
      ) else {
        continue;
      };
      if paint_opacity <= f32::EPSILON {
        continue;
      }
    }
    let lighting_profile = |profile: BevelProfileSample| {
      if geometry_lighting == Static3dGeometryLighting::Text {
        return word_text_bevel_lighting_profile(preset, profile);
      }
      if !interpolate_profile_lighting {
        return middle_profile;
      }
      // Office's enlarged circle/angle control keeps an angle bevel
      // constant across its whole width, while the circle starts with
      // the side-facing shade at the authored outer edge and converges
      // continuously to the flat-cap shade at the inner edge. Preserve
      // the MS-OI29500 profile geometry, but sample circle's symmetric
      // tangent field in that observed outer-to-inner direction. The
      // bounded shape fallback uses the same rule above.
      reflected_circle_bevel_lighting_profile(profile)
    };
    let outer_lighting_profile = lighting_profile(outer_profile);
    let inner_lighting_profile = lighting_profile(inner_profile);
    let material_endpoint = |material_source: &RgbaImage,
                             opacity_source: &RgbaImage,
                             uniform_material_opacity: Option<f32>,
                             has_authored_transparency: bool,
                             source,
                             outward: [f32; 2],
                             point: (f32, f32)| {
      // Shape-level bevels are a Direct3D material mesh, not another
      // alpha-mask crop. D3DTADDRESS_CLAMP extends the edge texel when a
      // projected outer vertex falls beyond the bounded render target;
      // discarding that lookup removes the outer profile strips and
      // changes variable-normal presets such as circle into an inward
      // subset of the authored MS-OI29500 surface.
      let texture_geometry = match source {
        TextSurfaceMaterialSource::Combined => material_geometry,
        TextSurfaceMaterialSource::Independent => paint_opacity_geometry,
      };
      let material_point = geometry.map_point_to(texture_geometry, point);
      let source_pixel = sample_bilinear_clamped(
        material_source,
        material_point.0 - 0.5,
        material_point.1 - 0.5,
      )?;
      let paint_opacity = sample_text_paint_opacity_clamped(
        opacity_source,
        source_mask.as_ref(),
        uniform_material_opacity.or(uniform_paint_opacity),
        geometry,
        paint_opacity_geometry,
        point,
      )?;
      if paint_opacity <= f32::EPSILON {
        return None;
      }
      Some(TextBevelMaterialEndpoint {
        source,
        source_point: point,
        material_point,
        outward,
        source_color: [source_pixel[0], source_pixel[1], source_pixel[2]],
        paint_opacity,
        alpha_texture_opacity: if has_authored_transparency {
          paint_opacity
        } else {
          0.0
        },
      })
    };
    let material_endpoint_pair = |material_source,
                                  opacity_source,
                                  uniform_material_opacity,
                                  has_authored_transparency,
                                  source,
                                  first,
                                  second| match (
      material_endpoint(
        material_source,
        opacity_source,
        uniform_material_opacity,
        has_authored_transparency,
        source,
        normals.0,
        first,
      ),
      material_endpoint(
        material_source,
        opacity_source,
        uniform_material_opacity,
        has_authored_transparency,
        source,
        normals.1,
        second,
      ),
    ) {
      (Some(start), Some(end)) => Some([start, end]),
      (Some(endpoint), None) | (None, Some(endpoint)) => Some([endpoint, endpoint]),
      (None, None) => None,
    };
    let base_material_source = if separate_outline_material.is_some() {
      options.fill_material.map(|material| {
        (
          material,
          options.fill_uniform_paint_opacity,
          options.fill_has_authored_transparency,
          TextSurfaceMaterialSource::Independent,
        )
      })
    } else {
      Some((
        material_source,
        uniform_paint_opacity,
        material_source_has_authored_transparency,
        TextSurfaceMaterialSource::Combined,
      ))
    };
    let outline_material_endpoint = |material: &RgbaImage,
                                     coverage: &RgbaImage,
                                     outward: [f32; 2],
                                     point: (f32, f32),
                                     inset_px: f32| {
      // A raw offset ring may cross a narrow stem's medial axis and
      // land on the outline belonging to the opposite source edge.
      // That pixel is not a second material interval for this edge.
      // Bound its raster-filter support in the edge's authored inward
      // coordinate; the direct-inset repair can then interpolate the
      // captured endpoint colours up to the medial ridge without
      // importing paint from the crossed ring.
      if options
        .outline_material_inset_px
        .is_some_and(|outline_inset_px| inset_px > outline_inset_px + 1.0 + f32::EPSILON)
      {
        return None;
      }
      let endpoint = material_endpoint(
        material,
        material,
        options.outline_uniform_paint_opacity,
        options.outline_has_authored_transparency,
        TextSurfaceMaterialSource::Independent,
        outward,
        point,
      )?;
      let material_point = geometry.map_point_to(paint_opacity_geometry, point);
      let coverage =
        sample_bilinear_clamped(coverage, material_point.0 - 0.5, material_point.1 - 0.5)?;
      let coverage = f32::from(coverage[3]) / 255.0;
      (coverage > f32::EPSILON).then_some((endpoint, coverage))
    };
    let composite_material_endpoint =
      |base, outline| composite_text_bevel_material_endpoint(base, outline);
    let composed_material_endpoints = |first, second, inset_px| {
      // These attributes belong to the emitted endpoints, not the midpoint
      // of the current profile strip. Reusing midpoint UVs for both ends
      // makes a shared vertex sample two different texture locations when
      // reached from adjacent strips. Keep fill and outline in the same
      // endpoint coordinate frame before compositing their attributes.
      let base_material_endpoints =
        base_material_source.and_then(|(material, uniform_opacity, authored, source)| {
          material_endpoint_pair(
            material,
            material_source,
            uniform_opacity,
            authored,
            source,
            first,
            second,
          )
        });
      let [base_first, base_second] = base_material_endpoints
        .map(|endpoints| endpoints.map(Some))
        .unwrap_or([None, None]);
      let (outline_first, outline_second) =
        separate_outline_material.map_or((None, None), |(material, coverage)| {
          (
            outline_material_endpoint(material, coverage, normals.0, first, inset_px),
            outline_material_endpoint(material, coverage, normals.1, second, inset_px),
          )
        });
      match (
        composite_material_endpoint(base_first, outline_first),
        composite_material_endpoint(base_second, outline_second),
      ) {
        (Some(first), Some(second)) => Some([first, second]),
        (Some(endpoint), None) | (None, Some(endpoint)) => Some([endpoint, endpoint]),
        (None, None) => None,
      }
    };
    let Some(outer_material_endpoints) = composed_material_endpoints(
      outer_first,
      outer_second,
      geometry_width * outer_profile.inset,
    ) else {
      continue;
    };
    let Some(inner_material_endpoints) = composed_material_endpoints(
      inner_first,
      inner_second,
      geometry_width * inner_profile.inset,
    ) else {
      continue;
    };
    let edge_colors = |endpoints: [TextBevelMaterialEndpoint; 2], profile| {
      (
        shade_text_bevel_material_endpoint(material_context, endpoints[0], profile),
        shade_text_bevel_material_endpoint(material_context, endpoints[1], profile),
      )
    };
    let (outer_start_color, outer_end_color, inner_start_color, inner_end_color) =
      if interpolate_profile_lighting {
        let (outer_start, outer_end) =
          edge_colors(outer_material_endpoints, outer_lighting_profile);
        let (inner_start, inner_end) =
          edge_colors(inner_material_endpoints, inner_lighting_profile);
        (outer_start, outer_end, inner_start, inner_end)
      } else {
        let (outer_start, outer_end) =
          edge_colors(outer_material_endpoints, outer_lighting_profile);
        (outer_start, outer_end, outer_start, outer_end)
      };
    let outer_z = surface_z + outer_profile.height * height * height_direction;
    let inner_z = surface_z + inner_profile.height * height * height_direction;
    let outer_inset_px = geometry_width * outer_profile.inset;
    let inner_inset_px = geometry_width * inner_profile.inset;
    let use_textured_material = geometry_lighting == Static3dGeometryLighting::Text;
    let outer_first = project(
      outer_first,
      outer_z,
      outer_start_color,
      use_textured_material,
      outer_inset_px,
      cell.outer_surface_distance_px,
    );
    let outer_second = project(
      outer_second,
      outer_z,
      outer_end_color,
      use_textured_material,
      outer_inset_px,
      cell.outer_surface_distance_px,
    );
    let inner_second = project(
      inner_second,
      inner_z,
      inner_end_color,
      use_textured_material,
      inner_inset_px,
      cell.inner_surface_distance_px,
    );
    let inner_first = project(
      inner_first,
      inner_z,
      inner_start_color,
      use_textured_material,
      inner_inset_px,
      cell.inner_surface_distance_px,
    );
    triangles.push(TextSurfaceTriangle {
      vertices: [outer_first, outer_second, inner_second],
      bevel_source: Some(TextBevelTriangleSource {
        back_face,
        contour_index: contour_index as u32,
        edge_index: index as u32,
        profile_strip_index: profile_strip_index as u16,
        second_half: false,
        direct_inset_graph: cell.direct_inset_graph,
      }),
    });
    triangles.push(TextSurfaceTriangle {
      vertices: [inner_second, inner_first, outer_first],
      bevel_source: Some(TextBevelTriangleSource {
        back_face,
        contour_index: contour_index as u32,
        edge_index: index as u32,
        profile_strip_index: profile_strip_index as u16,
        second_half: true,
        direct_inset_graph: cell.direct_inset_graph,
      }),
    });
  }
  TextBevelMesh {
    triangles,
    direct_inset_cells: direct_graph_cells,
  }
}

fn bevel_distance_field(source: &RgbaImage, width: f32) -> Vec<f32> {
  let image_width = source.width() as usize;
  let image_height = source.height() as usize;
  let limit = width.max(1.0) + 1.0;
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
    height_direction,
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
      // Keep the distance field and authored width in the same fractional
      // device-pixel coordinate space. Controlled Word exports change the
      // bevel lighting for a quarter-pixel width increment even when the
      // resolved alpha mask is unchanged, so rounding either operand here
      // discards observable geometry.
      let distance = distance_at(x, y);
      if distance >= width {
        continue;
      }
      let inward_fraction = distance / width.max(f32::EPSILON);
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
      let (normal_profile_dx, normal_profile_dy) =
        if preset.unwrap_or(a::BevelPresetValues::Circle) == a::BevelPresetValues::Circle {
          // MS-OI29500 publishes the circle profile from the outer edge to the
          // inner cap, while its tangent field runs from the flat-face normal
          // to the side-face normal. Sample the symmetric tangent position so
          // the outer-to-inner raster coordinates retain that orientation.
          let (_, dx, dy) = circle_bevel_profile(1.0 - inward_fraction);
          (dx, dy)
        } else {
          (profile_dx, profile_dy)
        };
      let normal_xy = height * normal_profile_dx;
      let normal_z = width * normal_profile_dy * if back_face { -1.0 } else { 1.0 };
      if !profile_height.is_nan() {
        let index = y as usize * source.width() as usize + x as usize;
        height_offsets[index] = profile_height * height * height_direction;
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
    BevelOptions, ProjectedImageOptions, Static3dColor, Static3dGeometryLighting,
    Static3dOutputBounds, Static3dRenderOptions, Static3dStyleParts, Static3dSurface,
    Static3dTextFinalGrid, Static3dTextGeometry, Static3dTextGeometryPaths, Text3dNormalJoinPolicy,
    TextContourSamples, TextPlanarCoverageResolve, TextPlanarInset, TextSolidSurfaceInput,
    TextSurfaceRasterSample, TextSurfaceRasterization, TextSurfaceTriangle, TextSurfaceVertex,
    WORD_TEXT_ANTIALIASED_CONTOUR_PHASE_PX, apply_material_diffuse_fresnel, apply_static_3d,
    apply_static_3d_text, bevel_distance_field, bevel_profile, bevel_profile_sample,
    bevel_terminal_inset, camera_projection, circle_bevel_profile, composite_bevel,
    composite_text_contour_stroke, composite_text_solid_surfaces, direct_inset_cells,
    legacy_material_diffuse_shade, light_rig, light_rig_surface_shade, light_rig_surface_specular,
    lighting_surface_normal, material_alpha_fresnel, material_base_alpha, material_diffuse_fresnel,
    material_diffuse_shade, material_surface_alpha, offset_text_3d_contour, output_padding,
    perspective_correct_text_surface_weights, project_static_3d_front_face,
    project_wordprocessing_static_3d_effect_plane, projected_front_region_output_bounds,
    projected_output_bounds, projected_region_output_bounds,
    projected_wordprocessing_effect_region_output_bounds, recover_text_paint_opacity,
    replace_surface_alpha, resolve_bevel, resolve_static_3d_style, resolve_text_surface_pixel,
    sample_bilinear, sample_bilinear_clamped, shade_fixed_gouraud_channel_with_specular,
    shade_gouraud_channel, shaded_geometry_pixel_with_specular, text_3d_contour_edge_normals,
    text_3d_source_tangents_form_contour_join, text_bevel_profile_strips,
    text_extrusion_contour_core_parameters, text_extrusion_edge_point, text_geometry_area_mask,
    text_geometry_boundary_distance, text_geometry_contains, text_geometry_inset_contains,
    text_geometry_mask, text_geometry_path, text_geometry_stroke_area_mask,
    text_material_surface_over, text_surface_edge, text_surface_triangle_covers_sample,
    transformed_bevel_surface_normal, word_text_bevel_geometry_profile,
    word_text_bevel_lighting_profile, word_text_bevel_material_boundary_parameters,
    word_text_bevel_segment_parameters, word_text_bevel_terminal_profile,
    word_text_material_specular_reflectance, wordprocessing_text_shape_has_effective_3d_geometry,
  };
  use crate::common::{PathCommand, Point, Pt, Rect, Size};
  use crate::model::RgbColor;

  #[test]
  fn text_specular_reflection_is_independent_of_diffuse_light_culling() {
    let mut scene = scene(a::PresetCameraValues::PerspectiveLeft);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let material = Some(a::PresetMaterialTypeValues::WarmMatte);
    // Captured source vertices: the first receives its highlight from a light
    // whose diffuse dot product is negative. The second is an unchanged
    // positive-light counterexample. Neither normal or view vector is altered.
    for (normal, view, diffuse_byte, specular_byte) in [
      (
        [-0.28261203, -0.9592343, 0.0],
        [0.29325512, 0.010299132, 0.95597875],
        102.38259,
        34.37604,
      ),
      (
        [0.11079722, -0.6983724, 7_071_068.0 / 10_000_000.0],
        [0.29880625, 0.008709835, 0.95427406],
        135.33762,
        5.634258,
      ),
    ] {
      let diffuse = material_diffuse_shade(&scene, normal, view, material);
      let specular = light_rig_surface_specular(&scene, normal, view, material, [0; 3]);
      for channel in 0..3 {
        assert!((diffuse[channel] * 128.0 - diffuse_byte).abs() < 0.001);
        assert!((specular[channel] * 255.0 - specular_byte).abs() < 0.001);
      }
      assert_eq!(
        light_rig_surface_specular(
          &scene,
          normal,
          view,
          Some(a::PresetMaterialTypeValues::Matte),
          [255; 3],
        ),
        [0.0; 3],
      );
    }
  }

  #[test]
  fn text_material_surface_over_retains_covered_contour() {
    let foreground = [202.0, 143.0, 46.0, 240.320_22];
    let background = [191.0, 144.0, 0.0, 255.0];
    let actual = text_material_surface_over(foreground, background);
    assert_eq!(actual[3], 255.0);
    for channel in 0..3 {
      let opacity = foreground[3] / 255.0;
      let expected = foreground[channel] * opacity + background[channel] * (1.0 - opacity);
      assert!((actual[channel] - expected).abs() < 0.000_1);
    }
    assert!(actual[2] < foreground[2]);
    assert!(actual[1] > foreground[1]);
  }

  #[test]
  fn text_material_surface_over_matches_unrounded_alpha_equation() {
    for foreground_alpha in 0..=255 {
      for background_alpha in 0..=255 {
        let foreground = [255.0, 64.0, 0.0, foreground_alpha as f32];
        let background = [255.0, 192.0, 96.0, background_alpha as f32];
        let actual = text_material_surface_over(foreground, background);
        let weight = f64::from(background_alpha) * (1.0 - f64::from(foreground_alpha) / 255.0);
        let alpha = f64::from(foreground_alpha) + weight;
        assert!((f64::from(actual[3]) - alpha).abs() < 0.000_1);
        if alpha > 0.0 {
          for channel in 0..3 {
            let expected = (f64::from(foreground[channel]) * f64::from(foreground_alpha)
              + f64::from(background[channel]) * weight)
              / alpha;
            assert!((f64::from(actual[channel]) - expected).abs() < 0.000_1);
          }
        }
      }
    }
    let background = [191.0, 144.0, 0.0, 128.0];
    assert_eq!(
      text_material_surface_over([3.0, 4.0, 5.0, 0.0], background),
      background
    );
    let opaque = [3.0, 4.0, 5.0, 255.0];
    assert_eq!(text_material_surface_over(opaque, background), opaque);
  }

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

  fn text_surface_strip(
    left: f32,
    right: f32,
    visibility_depth: f32,
    color: [f32; 4],
  ) -> [TextSurfaceTriangle; 2] {
    let vertex = |point| TextSurfaceVertex {
      point,
      visibility_depth,
      color,
      textured_material: None,
      bevel_collision: None,
    };
    [
      TextSurfaceTriangle {
        vertices: [
          vertex((left, 0.0)),
          vertex((right, 0.0)),
          vertex((right, 1.0)),
        ],
        bevel_source: None,
      },
      TextSurfaceTriangle {
        vertices: [
          vertex((right, 1.0)),
          vertex((left, 1.0)),
          vertex((left, 0.0)),
        ],
        bevel_source: None,
      },
    ]
  }

  fn text_surface_quad(visibility_depth: f32, color: [f32; 4]) -> [TextSurfaceTriangle; 2] {
    text_surface_strip(0.0, 1.0, visibility_depth, color)
  }

  fn rasterize_test_surfaces(
    triangles: &[TextSurfaceTriangle],
    geometry_lighting: Static3dGeometryLighting,
  ) -> RgbaImage {
    rasterize_test_surfaces_on_grid(
      triangles,
      geometry_lighting,
      TextSurfaceRasterization::SourceGrid,
    )
  }

  fn rasterize_test_surfaces_on_grid(
    triangles: &[TextSurfaceTriangle],
    geometry_lighting: Static3dGeometryLighting,
    rasterization: TextSurfaceRasterization,
  ) -> RgbaImage {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let projection = camera_projection(&scene, 0.0);
    let geometry = Static3dTextGeometry {
      contours: Vec::new(),
      source_coverage_path: kurbo::BezPath::new(),
      bevel_profile_tolerance_px: None,
      solid_on_right: true,
      page_plane_scale_x: 1.0,
      page_plane_scale_y: 1.0,
      page_plane_translate_x: 0.0,
      page_plane_translate_y: 0.0,
    };
    let source = RgbaImage::new(1, 1);
    let mut destination = RgbaImage::new(1, 1);
    composite_text_solid_surfaces(
      &mut destination,
      &source,
      TextSolidSurfaceInput {
        scene: &scene,
        material: None,
        bevel_material_source: &source,
        surface_material_texture: None,
        source_geometry: &geometry,
        material_geometry: &geometry,
        paint_opacity_geometry: &geometry,
        planar_fill_material: None,
        planar_outline_material: None,
        planar_outline_coverage: None,
        planar_fill_uniform_paint_opacity: None,
        planar_outline_uniform_paint_opacity: None,
        planar_outline_material_inset_px: None,
        fill_has_authored_transparency: false,
        outline_has_authored_transparency: false,
        uniform_paint_opacity: None,
        planar_geometry: &geometry,
        planar_coverage_geometry: &geometry,
        planar_inset_px: 0.0,
        direct_inset_cells: None,
        planar_coverage: TextPlanarCoverageResolve::BoxSamples8x4,
        triangles,
        direct_inset_bevel: None,
        contour: None,
        options: ProjectedImageOptions {
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
        geometry_lighting,
      },
      rasterization,
    );
    destination
  }

  fn rasterize_test_text_surfaces(triangles: &[TextSurfaceTriangle]) -> RgbaImage {
    rasterize_test_surfaces(triangles, Static3dGeometryLighting::Text)
  }

  #[test]
  fn clamped_material_sampling_extends_the_edge_texel() {
    let mut image = RgbaImage::new(2, 1);
    image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    image.put_pixel(1, 0, Rgba([0, 0, 255, 255]));

    assert_eq!(sample_bilinear(&image, -1.0, 0.0), None);
    assert_eq!(sample_bilinear(&image, 2.0, 0.0), None);
    assert_eq!(
      sample_bilinear_clamped(&image, -1.0, 0.0),
      Some(Rgba([255, 0, 0, 255]))
    );
    assert_eq!(
      sample_bilinear_clamped(&image, 2.0, 0.0),
      Some(Rgba([0, 0, 255, 255]))
    );
  }

  #[test]
  fn shape_material_points_follow_the_source_raster_mapping_not_the_physical_mesh_mapping() {
    let physical = Static3dTextGeometry {
      contours: Vec::new(),
      source_coverage_path: kurbo::BezPath::new(),
      bevel_profile_tolerance_px: None,
      solid_on_right: true,
      page_plane_scale_x: 1.306_076,
      page_plane_scale_y: 1.307_189_5,
      page_plane_translate_x: -304.651_6,
      page_plane_translate_y: -187.264_7,
    };
    let material = Static3dTextGeometry {
      contours: Vec::new(),
      source_coverage_path: kurbo::BezPath::new(),
      bevel_profile_tolerance_px: None,
      solid_on_right: true,
      page_plane_scale_x: 1.304_347_9,
      page_plane_scale_y: 1.305_555_6,
      page_plane_translate_x: -304.217_4,
      page_plane_translate_y: -187.0,
    };

    let top_left = physical.map_point_to(&material, (0.970_184_3, 0.970_596_3));
    let bottom_right = physical.map_point_to(&material, (46.029_808, 48.029_404));

    // These are the two mappings GDB observes in the controlled Office
    // Screen case: physical geometry keeps the unquantized effect surface,
    // while its paint source is snapped to one clear top/left guard pixel.
    assert!((top_left.0 - 1.0).abs() < 0.000_1);
    assert!((top_left.1 - 1.0).abs() < 0.000_1);
    assert!((bottom_right.0 - 46.0).abs() < 0.000_1);
    assert!((bottom_right.1 - 48.0).abs() < 0.000_1);
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
  fn text_surface_triangle_uses_direct3d_top_left_sample_ownership() {
    let upper_right = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)];
    let lower_left = [(0.0, 1.0), (0.0, 0.0), (1.0, 1.0)];
    let upper_right_area = text_surface_edge(upper_right[0], upper_right[1], upper_right[2]);
    let lower_left_area = text_surface_edge(lower_left[0], lower_left[1], lower_left[2]);
    let covers = |triangle: [(f32, f32); 3], area: f32, point| {
      text_surface_triangle_covers_sample(triangle[0], triangle[1], triangle[2], point, area)
    };

    // The shared diagonal belongs to exactly one triangle.
    assert!(covers(upper_right, upper_right_area, (0.5, 0.5)));
    assert!(!covers(lower_left, lower_left_area, (0.5, 0.5)));
    // Outer top/left edges are inclusive; outer bottom/right edges are not.
    assert!(covers(upper_right, upper_right_area, (0.5, 0.0)));
    assert!(!covers(upper_right, upper_right_area, (1.0, 0.5)));
    assert!(covers(lower_left, lower_left_area, (0.0, 0.5)));
    assert!(!covers(lower_left, lower_left_area, (0.5, 1.0)));

    // Winding does not change physical edge ownership.
    let reversed = [upper_right[2], upper_right[1], upper_right[0]];
    let reversed_area = text_surface_edge(reversed[0], reversed[1], reversed[2]);
    assert!(covers(reversed, reversed_area, (0.5, 0.5)));
    assert!(covers(reversed, reversed_area, (0.5, 0.0)));
    assert!(!covers(reversed, reversed_area, (1.0, 0.5)));
  }

  #[test]
  fn perspective_text_surface_color_uses_reciprocal_w_weights() {
    let screen_weights = [0.2, 0.3, 0.5];

    assert_eq!(
      perspective_correct_text_surface_weights(true, [1.0, 2.0, 4.0], screen_weights),
      Some(screen_weights)
    );

    let corrected =
      perspective_correct_text_surface_weights(false, [1.0, 2.0, 4.0], screen_weights)
        .expect("positive reciprocal-w denominator");
    let expected = [1.0 / 14.0, 3.0 / 14.0, 5.0 / 7.0];
    for (actual, expected) in corrected.into_iter().zip(expected) {
      assert!((actual - expected).abs() < 1.0e-6);
    }
    assert!((corrected.into_iter().sum::<f32>() - 1.0).abs() < 1.0e-6);
  }

  #[test]
  fn extrusion_contour_width_survives_collinear_subdivision() {
    let coverage = |edges: &[(f32, bool)]| {
      let cores = text_extrusion_contour_core_parameters(edges, &vec![false; edges.len()], 2.0);
      edges
        .iter()
        .zip(cores)
        .filter(|((_, visible), _)| *visible)
        .map(|((length, _), (start, end))| length * (start + 1.0 - end))
        .sum::<f32>()
    };
    let expected = coverage(&[(10.0, true), (10.0, false)]);
    assert!((expected - 2.0).abs() < 1.0e-5);
    assert!(
      (coverage(&[(0.25, true), (0.25, true), (9.5, true), (10.0, false)]) - expected).abs()
        < 1.0e-5
    );
    for count in 1..=64 {
      let mut edges = vec![(10.0 / count as f32, true); count];
      edges.push((10.0, false));
      // Include every cyclic start position: the closing edge is not special.
      for _ in 0..edges.len() {
        let actual = coverage(&edges);
        assert!(
          (actual - expected).abs() < 1.0e-4,
          "{count}: {actual} != {expected}"
        );
        edges.rotate_left(1);
      }
    }
  }

  #[test]
  fn extrusion_surface_ridges_distinguish_curve_continuations_and_convex_facets() {
    let select = |points: &[(f32, f32)], curves: &[bool], joins: &[bool], right| {
      super::text_extrusion_surface_contour_joins(points, curves, joins, right)[1]
    };
    for scale in [0.1_f32, 1.0, 10.0] {
      let shallow = [(0.0, 0.0), (scale, 0.0), (2.0 * scale, 0.05 * scale)];
      // A small curve/line turn must not cut a complete transverse stripe
      // through the terminal. Genuine authored straight/straight joins keep
      // their existing contract, independently of this curve policy.
      assert!(!select(
        &shallow,
        &[false, true, false],
        &[false, true, false],
        true
      ));
      assert!(select(&shallow, &[false; 3], &[false, true, false], true));
      let convex = [(0.0, 0.0), (scale, 0.0), (scale, scale)];
      assert!(select(&convex, &[false, true, true], &[false; 3], true));
      assert!(!select(&convex, &[false, true, true], &[false; 3], false));
      // Do not infer a new corner between a curve and a line solely from a
      // flattening chord; source metadata owns genuine mixed-segment joins.
      assert!(!select(&convex, &[false, true, false], &[false; 3], true));
      assert!(select(
        &convex,
        &[false, true, false],
        &[false, true, false],
        true
      ));
    }
    assert!(!select(&[(0.0, 0.0); 3], &[true; 3], &[true; 3], true));
    assert!(super::text_extrusion_surface_contour_joins(&[], &[], &[], true).is_empty());
  }

  #[test]
  fn extrusion_color_splits_preserve_the_submitted_edge() {
    for (first, second) in [
      ((0.0, 0.0), (3.0625, 1.0)),
      ((3.0625, 1.0), (0.0, 0.0)),
      ((12.013, -4.027), (15.081, -3.018)),
    ] {
      let start = first;
      let end = second;
      for subdivisions in [3, 7, 16, 31] {
        for index in 0..=subdivisions {
          let parameter = index as f32 / subdivisions as f32;
          let point = text_extrusion_edge_point(first, second, parameter);
          let distance =
            text_surface_edge(start, end, point).abs() / (end.0 - start.0).hypot(end.1 - start.1);
          assert!(
            distance <= 2.0e-6,
            "split {parameter} moved off the submitted edge by {distance}"
          );
        }
      }
    }
  }

  #[test]
  fn extrusion_contour_is_centered_only_at_real_depth_running_edges() {
    assert_eq!(
      text_extrusion_contour_core_parameters(&[(10.0, true)], &[true], 2.0),
      vec![(0.1, 0.9)]
    );
    assert_eq!(
      text_extrusion_contour_core_parameters(&[(10.0, true), (10.0, true)], &[true, false], 2.0),
      vec![(0.1, 1.0), (0.0, 0.9)]
    );
    assert_eq!(
      text_extrusion_contour_core_parameters(&[(10.0, true), (10.0, true)], &[false, false], 2.0),
      vec![(0.0, 1.0), (0.0, 1.0)]
    );
    assert_eq!(
      text_extrusion_contour_core_parameters(&[(1.0, true)], &[true], 2.0),
      vec![(0.5, 0.5)]
    );
    assert!(text_extrusion_contour_core_parameters(&[], &[], 2.0).is_empty());
    let degenerate = text_extrusion_contour_core_parameters(
      &[(0.0, true), (0.25, true), (9.75, true), (10.0, false)],
      &[false; 4],
      2.0,
    );
    assert_eq!(degenerate[1], (1.0, 1.0));
    assert!(
      degenerate
        .iter()
        .all(|(start, end)| start.is_finite() && end.is_finite())
    );
  }

  #[test]
  fn continuous_contour_area_is_independent_of_finite_depth_samples() {
    let base = TextSurfaceRasterSample {
      visibility_depth: 0.0,
      color: [0.0, 0.0, 255.0, 255.0],
      covered: true,
      tie_priority: 0,
    };
    let base_samples = [base; 32];
    let mut contour_depths = [f32::NEG_INFINITY; 32];
    contour_depths[..16].fill(1.0);

    let full_depth_win = resolve_text_surface_pixel(
      &base_samples,
      Some(TextContourSamples::Depth(&contour_depths)),
      64,
      Some(255),
      Some([255.0, 0.0, 0.0, 255.0]),
    )
    .expect("resolved contour");
    // The finite grid marks 16/32 samples, but all 16 active contour samples
    // win depth. Its absolute contribution is therefore the continuous
    // 64/255 area, not the finite 1/2 occupancy.
    assert_eq!(full_depth_win.rgb, [64, 0, 191]);
    assert!((full_depth_win.alpha - 1.0).abs() < f32::EPSILON);

    contour_depths[8..16].fill(-1.0);
    let half_depth_win = resolve_text_surface_pixel(
      &base_samples,
      Some(TextContourSamples::Depth(&contour_depths)),
      64,
      Some(255),
      Some([255.0, 0.0, 0.0, 255.0]),
    )
    .expect("resolved contour");
    assert_eq!(half_depth_win.rgb, [32, 0, 223]);
    assert!((half_depth_win.alpha - 1.0).abs() < f32::EPSILON);
  }

  #[test]
  fn shape_surface_material_is_evaluated_once_per_primitive_at_the_pixel_center() {
    let triangles = text_surface_strip(0.0, 0.25, 1.0, [0.0, 0.0, 0.0, 255.0])
      .into_iter()
      .chain(text_surface_strip(0.25, 1.0, 1.0, [100.0, 0.0, 0.0, 255.0]))
      .collect::<Vec<_>>();

    let raster = rasterize_test_surfaces(&triangles, Static3dGeometryLighting::Shape);

    // Direct3D MSAA invokes the pixel shader once per covered primitive and
    // replicates that primitive's result only to its covered samples. The
    // left strip owns 8/32 samples and the right strip owns 24/32, so resolve
    // retains both pixel-center material evaluations.
    assert_eq!(raster.get_pixel(0, 0), &Rgba([75, 0, 0, 255]));
  }

  #[test]
  fn shape_surface_mask_retains_geometric_coverage_without_discarding_material_samples() {
    let half = text_surface_strip(0.5, 1.0, 1.0, [100.0, 0.0, 0.0, 255.0]);
    let majority = text_surface_strip(0.25, 1.0, 1.0, [100.0, 0.0, 0.0, 255.0]);

    let shape_half = rasterize_test_surfaces(&half, Static3dGeometryLighting::Shape);
    let text_half = rasterize_test_surfaces(&half, Static3dGeometryLighting::Text);
    let shape_majority = rasterize_test_surfaces(&majority, Static3dGeometryLighting::Shape);

    // Shape and text surfaces both retain geometric coverage in alpha. Shape
    // material attributes are still evaluated at the D3D9 pixel center; that
    // distinction must not turn a partially covered primitive into a binary
    // mask.
    assert_eq!(shape_half.get_pixel(0, 0), &Rgba([100, 0, 0, 128]));
    assert_eq!(text_half.get_pixel(0, 0), &Rgba([100, 0, 0, 128]));
    assert_eq!(shape_majority.get_pixel(0, 0), &Rgba([100, 0, 0, 191]));
  }

  #[test]
  fn shape_surface_attributes_use_the_d3d9_integer_pixel_center() {
    let vertex = |point: (f32, f32), red: f32| TextSurfaceVertex {
      point,
      visibility_depth: 1.0,
      color: [red, 0.0, 0.0, 255.0],
      textured_material: None,
      bevel_collision: None,
    };
    let triangles = [
      TextSurfaceTriangle {
        vertices: [
          vertex((0.0, 0.0), 0.0),
          vertex((1.0, 0.0), 100.0),
          vertex((1.0, 1.0), 100.0),
        ],
        bevel_source: None,
      },
      TextSurfaceTriangle {
        vertices: [
          vertex((1.0, 1.0), 100.0),
          vertex((0.0, 1.0), 0.0),
          vertex((0.0, 0.0), 0.0),
        ],
        bevel_source: None,
      },
    ];

    let raster = rasterize_test_surfaces(&triangles, Static3dGeometryLighting::Shape);

    // Direct3D 9 evaluates both primitives at pixel center (0, 0), not at
    // the image-space cell center (0.5, 0.5).
    assert_eq!(raster.get_pixel(0, 0), &Rgba([0, 0, 0, 255]));
  }

  #[test]
  fn final_grid_text_surface_shades_once_at_the_covered_centroid() {
    let vertex = |point: (f32, f32), red: f32| TextSurfaceVertex {
      point,
      visibility_depth: 1.0,
      color: [red, 0.0, 0.0, 255.0],
      textured_material: None,
      bevel_collision: None,
    };
    let triangles = [TextSurfaceTriangle {
      vertices: [
        vertex((0.5, 0.0), 0.0),
        vertex((1.0, 0.0), 100.0),
        vertex((0.5, 1.0), 0.0),
      ],
      bevel_source: None,
    }];
    let final_grid =
      Static3dTextFinalGrid::new(1, 1, 0.0, 0.0, 1.0, 1.0).expect("valid final grid");

    let raster = rasterize_test_surfaces_on_grid(
      &triangles,
      Static3dGeometryLighting::Text,
      TextSurfaceRasterization::FinalGrid(final_grid),
    );

    // Two standard samples are covered. Word's centroid shader uses the first
    // covered sample (9/16, 5/16), even though the pixel center is on the left
    // edge. Red is 12.5 at that point, replicated to both covered samples.
    // Center-only shading gives zero; per-sample shading gives 50 instead.
    assert_eq!(raster.get_pixel(0, 0), &Rgba([13, 0, 0, 64]));
  }

  #[test]
  fn final_grid_text_centroid_covers_every_eight_sample_mask() {
    use super::TextSurfaceSamplePattern;
    let pattern = TextSurfaceSamplePattern::Direct3dStandard8;
    assert_eq!(pattern.centroid(0), None);
    assert_eq!(pattern.centroid(255), Some((0.5, 0.5)));
    for mask in 1_u32..255 {
      let first = (0..8).find(|index| mask & (1 << index) != 0).unwrap();
      assert_eq!(pattern.centroid(mask), Some(pattern.position(first)));
    }
    // No overflowing full-mask construction for the independent 32-sample
    // source-grid pattern. This does not select MSAA for the native path.
    let source = TextSurfaceSamplePattern::OfficeAntiAlias8x4;
    assert_eq!(source.centroid(u32::MAX), Some((0.5, 0.5)));
    assert_eq!(source.centroid(1 << 31), Some(source.position(31)));
  }

  #[test]
  fn inert_props_only_text_3d_remains_flat() {
    let parts = Static3dStyleParts {
      shape: Some(Box::new(a::Shape3DType::default())),
      ..Static3dStyleParts::default()
    };

    assert_eq!(resolve_static_3d_style(None, Some(&parts)), None);
  }

  #[test]
  fn non_geometric_props_only_text_3d_remains_flat() {
    for shape in [
      a::Shape3DType {
        contour_width: Some(CoordinateValue::Emu(12_700)),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        preset_material: Some(a::PresetMaterialTypeValues::Metal),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        bevel_top: Some(a::BevelTop {
          width: Some(CoordinateValue::Emu(38_100)),
          ..a::BevelTop::default()
        }),
        ..a::Shape3DType::default()
      },
    ] {
      let parts = Static3dStyleParts {
        shape: Some(Box::new(shape)),
        ..Static3dStyleParts::default()
      };
      assert_eq!(resolve_static_3d_style(None, Some(&parts)), None);
    }
  }

  #[test]
  fn word_text_static_3d_activation_uses_the_office_six_seven_emu_boundary() {
    let resolve_shape = |shape| {
      resolve_static_3d_style(
        None,
        Some(&Static3dStyleParts {
          shape: Some(Box::new(shape)),
          ..Static3dStyleParts::default()
        }),
      )
    };

    for height in [0, 1, 6] {
      assert!(
        resolve_shape(a::Shape3DType {
          extrusion_height: Some(CoordinateValue::Emu(height)),
          ..a::Shape3DType::default()
        })
        .is_none()
      );
    }
    assert!(
      resolve_shape(a::Shape3DType {
        extrusion_height: Some(CoordinateValue::Emu(7)),
        ..a::Shape3DType::default()
      })
      .is_some()
    );
    assert!(
      resolve_shape(a::Shape3DType {
        bevel_top: Some(a::BevelTop {
          height: Some(CoordinateValue::Emu(6)),
          ..a::BevelTop::default()
        }),
        ..a::Shape3DType::default()
      })
      .is_none()
    );
    assert!(
      resolve_shape(a::Shape3DType {
        bevel_bottom: Some(a::BevelBottom {
          height: Some(CoordinateValue::Emu(7)),
          ..a::BevelBottom::default()
        }),
        ..a::Shape3DType::default()
      })
      .is_some()
    );
  }

  #[test]
  fn word_text_outline_ownership_uses_the_same_effective_geometry_boundary() {
    for coordinate in [0, 1, 6] {
      assert!(!wordprocessing_text_shape_has_effective_3d_geometry(
        &a::Shape3DType {
          contour_width: Some(CoordinateValue::Emu(coordinate)),
          ..a::Shape3DType::default()
        }
      ));
    }
    for shape in [
      a::Shape3DType {
        contour_width: Some(CoordinateValue::Emu(7)),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        extrusion_height: Some(CoordinateValue::Emu(7)),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        bevel_top: Some(a::BevelTop {
          height: Some(CoordinateValue::Emu(7)),
          ..a::BevelTop::default()
        }),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        bevel_bottom: Some(a::BevelBottom {
          height: Some(CoordinateValue::Emu(7)),
          ..a::BevelBottom::default()
        }),
        ..a::Shape3DType::default()
      },
    ] {
      assert!(wordprocessing_text_shape_has_effective_3d_geometry(&shape));
    }
  }

  #[test]
  fn props_only_text_3d_geometry_uses_the_neutral_word_scene() {
    for shape in [
      a::Shape3DType {
        extrusion_height: Some(CoordinateValue::Emu(57_150)),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        bevel_top: Some(a::BevelTop {
          height: Some(CoordinateValue::Emu(38_100)),
          ..a::BevelTop::default()
        }),
        ..a::Shape3DType::default()
      },
      a::Shape3DType {
        bevel_bottom: Some(a::BevelBottom {
          height: Some(CoordinateValue::Emu(69_850)),
          ..a::BevelBottom::default()
        }),
        ..a::Shape3DType::default()
      },
    ] {
      let parts = Static3dStyleParts {
        shape: Some(Box::new(shape)),
        ..Static3dStyleParts::default()
      };
      let style = resolve_static_3d_style(None, Some(&parts))
        .expect("positive 3-D geometry must remain visible");

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
  }

  #[test]
  fn scene_only_text_3d_uses_the_neutral_shape() {
    let scene = scene(a::PresetCameraValues::PerspectiveLeft);
    let parts = Static3dStyleParts {
      scene: Some(Box::new(scene.clone())),
      ..Static3dStyleParts::default()
    };
    let style = resolve_static_3d_style(None, Some(&parts)).expect("scene3d activates 3-D text");

    assert_eq!(*style.scene, scene);
    assert_eq!(*style.shape, a::Shape3DType::default());
  }

  #[test]
  fn body_scene_and_run_shape_anchor_word_depth_at_the_extrusion_base() {
    let body = super::Static3dStyle {
      scene: Box::new(scene(a::PresetCameraValues::PerspectiveLeft)),
      shape: Box::new(a::Shape3DType::default()),
      extrusion_color: None,
      contour_color: None,
      wordprocessing_effect_plane_z_pt: None,
    };
    let run_shape = Box::new(a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(63_500)),
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(38_100)),
        height: Some(CoordinateValue::Emu(38_100)),
        ..a::BevelTop::default()
      }),
      bevel_bottom: Some(a::BevelBottom {
        width: Some(CoordinateValue::Emu(50_800)),
        height: Some(CoordinateValue::Emu(50_800)),
        ..a::BevelBottom::default()
      }),
      ..a::Shape3DType::default()
    });
    let run = Static3dStyleParts {
      shape: Some(run_shape.clone()),
      ..Static3dStyleParts::default()
    };

    let combined = resolve_static_3d_style(Some(&body), Some(&run)).unwrap();

    let mut expected_shape = run_shape;
    // The top face is 5pt extrusion + 3pt top bevel above the source plane.
    // The bottom bevel starts at that source/base plane and is not included.
    expected_shape.z = Some(CoordinateValue::Emu(101_600));
    assert_eq!(combined.shape, expected_shape);
    assert_eq!(combined.wordprocessing_effect_plane_z_pt, Some(0.0));
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
  fn viewport_translation_is_applied_after_parallel_and_perspective_projection() {
    for preset in [
      a::PresetCameraValues::OrthographicFront,
      a::PresetCameraValues::PerspectiveFront,
    ] {
      let projection = camera_projection(&scene(preset), 0.0);
      let translated = projection.with_viewport_translation_px(-2.0 / 3.0, 0.25);
      let before = super::project_local_pixels(projection, 7.0, -3.0, -5.0, 64.0, 36.0, 4.0 / 3.0);
      let after = super::project_local_pixels(translated, 7.0, -3.0, -5.0, 64.0, 36.0, 4.0 / 3.0);

      assert!((after.0 - before.0 + 2.0 / 3.0).abs() < 0.000_1);
      assert!((after.1 - before.1 - 0.25).abs() < 0.000_1);
    }
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
  fn orthographic_front_bevel_preserves_arbitrary_source_coverage() {
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let shape = a::Shape3DType {
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(12_700)),
        height: Some(CoordinateValue::Emu(12_700)),
        preset: Some(a::BevelPresetValues::Circle),
      }),
      ..a::Shape3DType::default()
    };
    let mut image = RgbaImage::new(8, 6);
    for y in 1..5 {
      for x in 1..7 {
        let alpha = if y == 1 {
          112
        } else if y == 4 {
          105
        } else {
          255
        };
        image.put_pixel(x, y, Rgba([180, 80, 30, alpha]));
      }
    }
    image.put_pixel(3, 2, Rgba([0, 0, 0, 0]));
    image.put_pixel(4, 3, Rgba([180, 80, 30, 96]));
    let source_alpha = image.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>();

    apply_static_3d(
      &mut image,
      &scene,
      camera_projection(&scene, 0.0),
      &shape,
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: None,
        pixels_per_point: 1.0,
        model_surface: Some(Static3dSurface {
          left_px: 1.0,
          top_px: 1.0,
          width_px: 6.0,
          height_px: 4.0,
        }),
      },
    );

    assert_eq!(
      image.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>(),
      source_alpha
    );
  }

  #[test]
  fn bevel_material_sampling_separates_composed_rgb_and_independent_alpha() {
    use super::{
      TextGeometryAreaMask, TextSurfaceMaterialImages, TextSurfaceMaterialSource,
      TextSurfaceMaterialVertex, interpolate_text_surface_material,
      resolve_text_surface_material_color,
    };
    let combined = RgbaImage::from_fn(2, 2, |x, y| {
      Rgba([11 + 40 * x as u8, 22 + 40 * y as u8, 33, 255])
    });
    let independent_alpha = TextGeometryAreaMask {
      left: 0,
      top: 0,
      width: 2,
      height: 2,
      alpha: vec![0, 0, 0, 255],
    };
    let combined_alpha = TextGeometryAreaMask {
      left: 0,
      top: 0,
      width: 2,
      height: 2,
      alpha: vec![128; 4],
    };
    let images = TextSurfaceMaterialImages {
      combined: &combined,
      surface_material_texture: None,
      independent_to_combined: tiny_skia::Transform::from_translate(-0.5, -0.5),
      independent_alpha: Some(&independent_alpha),
    };
    let vertex = TextSurfaceMaterialVertex {
      source: TextSurfaceMaterialSource::Independent,
      material_point: (1.0, 1.0),
      diffuse: [1.0; 3],
      specular_incident: [0.0; 3],
      material_opacity: 1.0,
      alpha_texture_fraction: 1.0,
    };
    assert_eq!(
      resolve_text_surface_material_color(images, Some(&combined_alpha), None, vertex, 255.0),
      Some([11.0, 22.0, 33.0, 63.75])
    );
    assert_eq!(
      resolve_text_surface_material_color(
        images,
        Some(&combined_alpha),
        None,
        TextSurfaceMaterialVertex {
          source: TextSurfaceMaterialSource::Combined,
          material_point: (0.5, 0.5),
          ..vertex
        },
        255.0
      ),
      Some([11.0, 22.0, 33.0, 128.0]),
    );
    let interpolated = interpolate_text_surface_material(
      vertex,
      TextSurfaceMaterialVertex {
        material_point: (2.0, 2.0),
        ..vertex
      },
      0.25,
    );
    assert_eq!(interpolated.source, TextSurfaceMaterialSource::Independent);
    assert_eq!(interpolated.material_point, (1.25, 1.25));
  }

  #[test]
  fn active_text_front_is_lit_without_a_top_bevel() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometryPaths::from_page_path_for_direct3d9(
      &[
        PathCommand::MoveTo(point(1.0, 1.0)),
        PathCommand::LineTo(point(11.0, 1.0)),
        PathCommand::LineTo(point(11.0, 11.0)),
        PathCommand::LineTo(point(1.0, 11.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: Point::default(),
        size: Size {
          width: Pt(12.0),
          height: Pt(12.0),
        },
      },
      1.0,
    )
    .expect("front geometry")
    .with_uniform_paint_opacity(Some(1.0));
    let scene = super::default_text_3d_scene();
    let projection = camera_projection(&scene, 0.0);
    let shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(57_150)),
      preset_material: Some(a::PresetMaterialTypeValues::Metal),
      ..a::Shape3DType::default()
    };
    let options = Static3dRenderOptions {
      extrusion_color: None,
      contour_color: None,
      pixels_per_point: 1.0,
      model_surface: Some(Static3dSurface {
        left_px: 0.0,
        top_px: 0.0,
        width_px: 12.0,
        height_px: 12.0,
      }),
    };
    for color in [
      [0, 0, 0],
      [64, 64, 64],
      [128, 128, 128],
      [192, 192, 192],
      [64, 128, 192],
      [192, 128, 64],
    ] {
      let source = RgbaImage::from_pixel(12, 12, Rgba([color[0], color[1], color[2], 255]));
      let mut expected = source.clone();
      super::shade_planar_surface(
        &mut expected,
        &scene,
        &ProjectedImageOptions {
          projection,
          z: 0.0,
          bounds: (0, 0, 11, 11),
          model_surface: options.model_surface.unwrap(),
          pixels_per_point: 1.0,
          tint: None,
        },
        [0.0, 0.0, 1.0],
        shape.preset_material,
        true,
      );
      assert_ne!(expected.get_pixel(6, 6), source.get_pixel(6, 6));
      // Both the combined fallback and the independent fill-material owner
      // must apply the same existing lighting equation, exactly once.
      for separate_fill in [false, true] {
        let mut actual = source.clone();
        super::apply_static_3d_text_with_outline_material(
          &mut actual,
          crate::common::drawingml_3d::Static3dTextSurface {
            geometry: &geometry,
            front_fill_material: separate_fill.then_some(&source),
            front_outline_material: None,
            front_outline_coverage: None,
          },
          &scene,
          projection,
          &shape,
          options,
        );
        assert_eq!(
          actual.get_pixel(6, 6),
          expected.get_pixel(6, 6),
          "color={color:?}, separate_fill={separate_fill}"
        );
      }
    }
    // Independent fill and outline represent paint layers on ONE lit face,
    // not two separately saturated surfaces. Include black/white endpoints
    // and intermediate colours/opacities: the white endpoints alone hide
    // premature clipping. Coverage is full here so this tests only ordering.
    let coverage = RgbaImage::from_pixel(12, 12, Rgba([255; 4]));
    for fill in [0_u8, 96, 255] {
      for outline in [0_u8, 160, 255] {
        for opacity in [0.0_f32, 0.4, 0.6, 1.0] {
          let mixed =
            (f32::from(fill) * (1.0 - opacity) + f32::from(outline) * opacity).round() as u8;
          let combined = RgbaImage::from_pixel(12, 12, Rgba([mixed, mixed, mixed, 255]));
          let fill_image = RgbaImage::from_pixel(12, 12, Rgba([fill, fill, fill, 255]));
          let outline_image = RgbaImage::from_pixel(
            12,
            12,
            Rgba([outline, outline, outline, (opacity * 255.0).round() as u8]),
          );
          let layered_geometry = geometry
            .clone()
            .with_front_material_opacities(Some(1.0), Some(opacity))
            .with_front_outline_material_inset_px(Some(10.0));
          let mut actual = combined.clone();
          super::apply_static_3d_text_with_outline_material(
            &mut actual,
            crate::common::drawingml_3d::Static3dTextSurface {
              geometry: &layered_geometry,
              front_fill_material: Some(&fill_image),
              front_outline_material: Some(&outline_image),
              front_outline_coverage: Some(&coverage),
            },
            &scene,
            projection,
            &shape,
            options,
          );
          let mut expected = combined;
          super::apply_static_3d_text_with_outline_material(
            &mut expected,
            crate::common::drawingml_3d::Static3dTextSurface {
              geometry: &geometry,
              front_fill_material: None,
              front_outline_material: None,
              front_outline_coverage: None,
            },
            &scene,
            projection,
            &shape,
            options,
          );
          assert_eq!(
            actual.get_pixel(6, 6),
            expected.get_pixel(6, 6),
            "paint-before-light fill={fill}, outline={outline}, opacity={opacity}",
          );
        }
      }
    }
    // A constant texture cannot expose registration errors. Each texel here
    // identifies its page-space position; the physical mesh's half-pixel
    // conversion must not also move either page-rasterized material source.
    let source = RgbaImage::from_fn(12, 12, |x, y| Rgba([(x * 8) as u8, (y * 8) as u8, 96, 255]));
    let mut lit = source.clone();
    super::shade_planar_surface(
      &mut lit,
      &scene,
      &ProjectedImageOptions {
        projection,
        z: 0.0,
        bounds: (0, 0, 11, 11),
        model_surface: options.model_surface.unwrap(),
        pixels_per_point: 1.0,
        tint: None,
      },
      [0.0, 0.0, 1.0],
      shape.preset_material,
      true,
    );
    for separate_fill in [false, true] {
      let mut actual = source.clone();
      super::apply_static_3d_text_with_outline_material(
        &mut actual,
        crate::common::drawingml_3d::Static3dTextSurface {
          geometry: &geometry,
          front_fill_material: separate_fill.then_some(&source),
          front_outline_material: None,
          front_outline_coverage: None,
        },
        &scene,
        projection,
        &shape,
        options,
      );
      for channel in 0..3 {
        let expected = (u16::from(lit.get_pixel(6, 6)[channel])
          + u16::from(lit.get_pixel(7, 7)[channel]))
        .div_ceil(2);
        // Material interpolation and subpixel resolve independently round to
        // bytes. At most one byte differs from their combined midpoint;
        // the incorrect physical-space lookup misses by four bytes.
        assert!(
          i32::from(actual.get_pixel(6, 6)[channel]).abs_diff(i32::from(expected)) <= 1,
          "page texture channel={channel}, separate_fill={separate_fill}"
        );
      }
    }
  }

  #[test]
  fn word_text_orthographic_alpha_depends_only_on_front_geometry_and_contour() {
    // Integer-aligned opaque rectangles hid the accidental reuse of beveled
    // triangles in the coverage pass. Fractional edges and varying paint
    // opacity expose that coupling without depending on a system font.
    for (phase, variable_opacity) in [(0.0_f32, false), (0.23_f32, true)] {
      let point = |x: f32, y: f32| Point {
        x: Pt(x + phase),
        y: Pt(y + phase * 0.5),
      };
      let commands = [
        PathCommand::MoveTo(point(1.0, 1.0)),
        PathCommand::LineTo(point(11.0, 1.0)),
        PathCommand::LineTo(point(11.0, 9.0)),
        PathCommand::LineTo(point(1.0, 9.0)),
        PathCommand::Close,
        PathCommand::MoveTo(point(4.0, 3.0)),
        PathCommand::LineTo(point(4.0, 7.0)),
        PathCommand::LineTo(point(8.0, 7.0)),
        PathCommand::LineTo(point(8.0, 3.0)),
        PathCommand::Close,
      ];
      let geometry = Static3dTextGeometryPaths::from_page_path_for_direct3d9(
        &commands,
        Rect {
          origin: Point::default(),
          size: Size {
            width: Pt(12.0),
            height: Pt(10.0),
          },
        },
        1.0,
      )
      .expect("closed text outline");
      let source_path = text_geometry_path(&geometry.page, |point| point).expect("source path");
      let source_mask = text_geometry_mask(12, 10, &source_path).expect("source alpha");
      let mut source = RgbaImage::new(12, 10);
      for y in 0..source.height() {
        for x in 0..source.width() {
          let mut alpha = source_mask.pixel(x, y).map_or(0, |pixel| pixel.alpha());
          if variable_opacity {
            alpha = (u32::from(alpha) * (80 + x * 12) / 255) as u8;
          }
          if alpha != 0 {
            source.put_pixel(x, y, Rgba([80 + x as u8 * 8, 40 + y as u8 * 9, 120, alpha]));
          }
        }
      }
      let scene = scene(a::PresetCameraValues::OrthographicFront);
      let projection = camera_projection(&scene, 0.0);

      for contour_enabled in [false, true] {
        let mut reference_alpha = None;
        for (preset, mask) in [
          a::BevelPresetValues::Angle,
          a::BevelPresetValues::Circle,
          a::BevelPresetValues::RelaxedInset,
        ]
        .into_iter()
        .flat_map(|preset| (0_u8..8).map(move |mask| (preset, mask)))
        {
          let extrusion_enabled = mask & 4 != 0;
          let top_enabled = mask & 2 != 0;
          let bottom_enabled = mask & 1 != 0;
          let shape = a::Shape3DType {
            extrusion_height: Some(CoordinateValue::Emu(if extrusion_enabled {
              57_150
            } else {
              // Keep the independently tested 3-D activation boundary. With
              // zero depth and no bevel, Word's neutral contour-only route
              // suppresses the contour and is not this invariant's domain.
              7
            })),
            contour_width: Some(CoordinateValue::Emu(if contour_enabled {
              12_700
            } else {
              0
            })),
            preset_material: Some(a::PresetMaterialTypeValues::Metal),
            bevel_top: Some(a::BevelTop {
              width: Some(CoordinateValue::Emu(38_100)),
              height: Some(CoordinateValue::Emu(if top_enabled { 38_100 } else { 0 })),
              preset: Some(preset),
            }),
            bevel_bottom: Some(a::BevelBottom {
              width: Some(CoordinateValue::Emu(69_850)),
              height: Some(CoordinateValue::Emu(if bottom_enabled {
                69_850
              } else {
                0
              })),
              preset: Some(a::BevelPresetValues::Divot),
            }),
            ..a::Shape3DType::default()
          };
          let mut image = source.clone();
          apply_static_3d_text(
            &mut image,
            &geometry,
            &scene,
            projection,
            &shape,
            Static3dRenderOptions {
              extrusion_color: Some(Static3dColor {
                color: RgbColor {
                  r: 210,
                  g: 220,
                  b: 240,
                },
                alpha: 255,
              }),
              contour_color: Some(Static3dColor {
                color: RgbColor {
                  r: 20,
                  g: 80,
                  b: 140,
                },
                alpha: 255,
              }),
              pixels_per_point: 1.0,
              model_surface: Some(Static3dSurface {
                left_px: 0.0,
                top_px: 0.0,
                width_px: 12.0,
                height_px: 10.0,
              }),
            },
          );
          let alpha = image.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>();
          if let Some(reference) = reference_alpha.as_ref() {
            assert_eq!(
              &alpha, reference,
              "phase={phase}, contour={contour_enabled}, preset={preset:?}, mask={mask:03b}"
            );
          } else {
            reference_alpha = Some(alpha);
          }
        }
      }
    }
  }

  #[test]
  fn word_text_orthographic_front_fully_occludes_the_bottom_bevel() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let mut path = Vec::new();
    for (left, right) in [(1.5, 3.5), (6.0, 8.0), (10.5, 12.5)] {
      path.extend([
        PathCommand::MoveTo(point(left, 1.0)),
        PathCommand::LineTo(point(right, 1.0)),
        PathCommand::LineTo(point(right, 11.0)),
        PathCommand::LineTo(point(left, 11.0)),
        PathCommand::Close,
      ]);
    }
    let geometry = Static3dTextGeometry::from_page_path(
      &path,
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(15.0),
          height: Pt(12.0),
        },
      },
      2.0,
    )
    .expect("three narrow text stems");
    let mut source = RgbaImage::new(30, 24);
    for y in 0..source.height() {
      for x in 0..source.width() {
        if text_geometry_contains(&geometry, (x as f32 + 0.5, y as f32 + 0.5)) {
          source.put_pixel(x, y, Rgba([213, 220, 228, 255]));
        }
      }
    }
    let geometry = Static3dTextGeometryPaths::aligned(geometry);
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let projection = camera_projection(&scene, 0.0);
    let shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(63_500)),
      contour_width: Some(CoordinateValue::Emu(25_400)),
      preset_material: Some(a::PresetMaterialTypeValues::SoftEdge),
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(38_100)),
        height: Some(CoordinateValue::Emu(38_100)),
        preset: Some(a::BevelPresetValues::RelaxedInset),
      }),
      ..a::Shape3DType::default()
    };
    let options = Static3dRenderOptions {
      extrusion_color: Some(Static3dColor {
        color: RgbColor {
          r: 240,
          g: 240,
          b: 240,
        },
        alpha: 255,
      }),
      contour_color: Some(Static3dColor {
        color: RgbColor {
          r: 146,
          g: 208,
          b: 80,
        },
        alpha: 255,
      }),
      pixels_per_point: 2.0,
      model_surface: Some(Static3dSurface {
        left_px: 0.0,
        top_px: 0.0,
        width_px: 30.0,
        height_px: 24.0,
      }),
    };
    let mut without_bottom = source.clone();
    apply_static_3d_text(
      &mut without_bottom,
      &geometry,
      &scene,
      projection,
      &shape,
      options,
    );
    let mut with_bottom = source;
    apply_static_3d_text(
      &mut with_bottom,
      &geometry,
      &scene,
      projection,
      &a::Shape3DType {
        bevel_bottom: Some(a::BevelBottom {
          width: Some(CoordinateValue::Emu(69_850)),
          height: Some(CoordinateValue::Emu(38_100)),
          preset: Some(a::BevelPresetValues::Cross),
        }),
        ..shape
      },
      options,
    );

    assert_eq!(with_bottom, without_bottom);
  }

  #[test]
  fn word_text_orthographic_contour_overlays_the_lit_front_material() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(1.0, 1.0)),
        PathCommand::LineTo(point(11.0, 1.0)),
        PathCommand::LineTo(point(11.0, 9.0)),
        PathCommand::LineTo(point(1.0, 9.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(12.0),
          height: Pt(10.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");
    let geometry = Static3dTextGeometryPaths::aligned(geometry);
    let mut image = RgbaImage::new(12, 10);
    for y in 1..9 {
      for x in 1..11 {
        image.put_pixel(x, y, Rgba([90, 160, 220, 255]));
      }
    }
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let contour = Static3dColor {
      color: RgbColor {
        r: 20,
        g: 80,
        b: 140,
      },
      alpha: 255,
    };

    apply_static_3d_text(
      &mut image,
      &geometry,
      &scene,
      camera_projection(&scene, 0.0),
      &a::Shape3DType {
        // Keep the solid active: the independently verified neutral,
        // zero-geometry case suppresses contour even with an authored width.
        extrusion_height: Some(CoordinateValue::Emu(7)),
        contour_width: Some(CoordinateValue::Emu(25_400)),
        ..a::Shape3DType::default()
      },
      Static3dRenderOptions {
        extrusion_color: None,
        contour_color: Some(contour),
        pixels_per_point: 1.0,
        model_surface: Some(Static3dSurface {
          left_px: 0.0,
          top_px: 0.0,
          width_px: 12.0,
          height_px: 10.0,
        }),
      },
    );

    assert_eq!(image.get_pixel(1, 5), &Rgba([20, 80, 140, 255]));
    assert_ne!(image.get_pixel(5, 5).0[..3], [20, 80, 140]);
  }

  #[test]
  fn word_text_contour_depth_exposes_less_rgb_through_a_taller_angle_bevel() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(4.0, 2.0)),
        PathCommand::LineTo(point(20.0, 2.0)),
        PathCommand::LineTo(point(20.0, 14.0)),
        PathCommand::LineTo(point(4.0, 14.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(24.0),
          height: Pt(16.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");
    let geometry = Static3dTextGeometryPaths::aligned(geometry);
    let mut source = RgbaImage::new(24, 16);
    for y in 2..14 {
      for x in 4..20 {
        source.put_pixel(x, y, Rgba([190, 215, 240, 255]));
      }
    }
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let projection = camera_projection(&scene, 0.0);
    let render = |bevel_height: Option<i64>, contour_color: RgbColor| {
      let mut image = source.clone();
      apply_static_3d_text(
        &mut image,
        &geometry,
        &scene,
        projection,
        &a::Shape3DType {
          extrusion_height: Some(CoordinateValue::Emu(7)),
          contour_width: Some(CoordinateValue::Emu(50_800)),
          bevel_top: bevel_height.map(|height| a::BevelTop {
            width: Some(CoordinateValue::Emu(76_200)),
            height: Some(CoordinateValue::Emu(height)),
            preset: Some(a::BevelPresetValues::Angle),
          }),
          preset_material: Some(a::PresetMaterialTypeValues::Matte),
          ..a::Shape3DType::default()
        },
        Static3dRenderOptions {
          extrusion_color: None,
          contour_color: Some(Static3dColor {
            color: contour_color,
            alpha: 255,
          }),
          pixels_per_point: 1.0,
          model_surface: Some(Static3dSurface {
            left_px: 0.0,
            top_px: 0.0,
            width_px: 24.0,
            height_px: 16.0,
          }),
        },
      );
      image
    };
    let contribution = |height| {
      let black = render(height, RgbColor { r: 0, g: 0, b: 0 });
      let red = render(height, RgbColor { r: 255, g: 0, b: 0 });
      black
        .pixels()
        .zip(red.pixels())
        .map(|(black, red)| u64::from(red[0].saturating_sub(black[0])))
        .sum::<u64>()
    };

    let full_contour = contribution(None);
    let shallow_bevel = contribution(Some(19_050));
    let tall_bevel = contribution(Some(152_400));
    assert!(
      full_contour > shallow_bevel,
      "{full_contour} <= {shallow_bevel}"
    );
    assert!(
      shallow_bevel > tall_bevel,
      "{shallow_bevel} <= {tall_bevel}"
    );
  }

  #[test]
  fn replacing_static_3d_alpha_preserves_owned_surface_rgb() {
    let mut surface = RgbaImage::from_raw(3, 1, vec![10, 20, 30, 200, 0, 0, 0, 0, 70, 80, 90, 100])
      .expect("surface");
    let coverage =
      RgbaImage::from_raw(3, 1, vec![110, 120, 130, 64, 140, 150, 160, 32, 0, 0, 0, 0])
        .expect("coverage");

    replace_surface_alpha(&mut surface, &coverage);

    assert_eq!(surface.get_pixel(0, 0), &Rgba([10, 20, 30, 64]));
    assert_eq!(surface.get_pixel(1, 0), &Rgba([140, 150, 160, 32]));
    assert_eq!(surface.get_pixel(2, 0), &Rgba([0, 0, 0, 0]));
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
  fn text_contour_stroke_preserves_fractional_authored_width() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(4.0, 4.0)),
        PathCommand::LineTo(point(12.0, 4.0)),
        PathCommand::LineTo(point(12.0, 12.0)),
        PathCommand::LineTo(point(4.0, 12.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(16.0),
          height: Pt(16.0),
        },
      },
      1.0,
    )
    .expect("closed contour geometry");
    let color = Static3dColor {
      color: RgbColor {
        r: 20,
        g: 80,
        b: 140,
      },
      alpha: 255,
    };
    let mut two_pixels = RgbaImage::new(16, 16);
    let mut fractional = RgbaImage::new(16, 16);
    composite_text_contour_stroke(&mut two_pixels, &geometry, 2.0, 0.0, color);
    composite_text_contour_stroke(&mut fractional, &geometry, 2.75, 0.0, color);

    let alpha_sum =
      |image: &RgbaImage| image.pixels().map(|pixel| u32::from(pixel[3])).sum::<u32>();
    assert!(alpha_sum(&fractional) > alpha_sum(&two_pixels));
    assert_eq!(two_pixels.get_pixel(8, 8)[3], 0);
    assert_eq!(fractional.get_pixel(8, 8)[3], 0);
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
    let area_mask = text_geometry_area_mask(&geometry).expect("area mask");
    assert_eq!(area_mask.alpha_at(5, 5), 0);
    assert!(text_geometry_contains(&geometry, (2.0, 2.0)));
    assert!(!text_geometry_contains(&geometry, (5.0, 5.0)));
    assert!(!text_geometry_contains(&geometry, (11.0, 5.0)));
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
  fn text_geometry_area_mask_retains_curves_and_continuous_byte_coverage() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(1.123, 0.2)),
        PathCommand::LineTo(point(5.123, 0.2)),
        PathCommand::CubicTo {
          control1: point(14.0, 70.0),
          control2: point(31.0, 190.0),
          end: point(40.987, 257.8),
        },
        PathCommand::LineTo(point(36.987, 257.8)),
        PathCommand::CubicTo {
          control1: point(27.0, 190.0),
          control2: point(10.0, 70.0),
          end: point(1.123, 0.2),
        },
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(48.0),
          height: Pt(260.0),
        },
      },
      1.0,
    )
    .expect("curved text geometry");

    assert!(
      geometry
        .source_coverage_path
        .iter()
        .any(|command| matches!(command, kurbo::PathEl::CurveTo(..)))
    );
    let mask = text_geometry_area_mask(&geometry).expect("area mask");
    let mut values = [false; 256];
    for alpha in mask.alpha {
      values[usize::from(alpha)] = true;
    }
    assert!(values.into_iter().filter(|present| *present).count() > 128);

    let mut stroke = RgbaImage::new(48, 260);
    composite_text_contour_stroke(
      &mut stroke,
      &geometry,
      2.0,
      0.0,
      Static3dColor {
        color: RgbColor {
          r: 20,
          g: 80,
          b: 140,
        },
        alpha: 255,
      },
    );
    let mut stroke_values = [false; 256];
    for pixel in stroke.pixels() {
      stroke_values[usize::from(pixel[3])] = true;
    }
    assert!(stroke_values.into_iter().filter(|present| *present).count() > 128);
  }

  #[test]
  fn direct3d9_text_geometry_separates_physical_and_page_pixel_centers() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometryPaths::from_page_path_for_direct3d9(
      &[
        PathCommand::MoveTo(point(1.0, 1.0)),
        PathCommand::LineTo(point(3.0, 1.0)),
        PathCommand::LineTo(point(3.0, 3.0)),
        PathCommand::LineTo(point(1.0, 3.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(4.0),
          height: Pt(4.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");

    assert_eq!(geometry.physical.contours[0].bounds, (0.5, 0.5, 2.5, 2.5));
    assert_eq!(geometry.page.contours[0].bounds, (1.0, 1.0, 3.0, 3.0));
    assert_eq!(geometry.physical.page_plane_translate_x, -0.5);
    assert_eq!(geometry.physical.page_plane_translate_y, -0.5);
    assert_eq!(geometry.page.page_plane_translate_x, 0.0);
    assert_eq!(geometry.page.page_plane_translate_y, 0.0);
    assert_eq!(
      geometry.physical.map_point_to(&geometry.page, (0.5, 0.5)),
      (1.0, 1.0)
    );
  }

  #[test]
  fn word_text_contour_restores_antialiased_half_pixel_phase() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path_for_direct3d9(
      &[
        PathCommand::MoveTo(point(3.0, 1.0)),
        PathCommand::LineTo(point(5.0, 1.0)),
        PathCommand::LineTo(point(5.0, 7.0)),
        PathCommand::LineTo(point(3.0, 7.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(8.0),
          height: Pt(8.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");

    let physical = text_geometry_stroke_area_mask(&geometry, 2.0, 0.0).expect("physical mask");
    let antialiased =
      text_geometry_stroke_area_mask(&geometry, 2.0, WORD_TEXT_ANTIALIASED_CONTOUR_PHASE_PX)
        .expect("antialiased contour mask");

    assert_eq!(geometry.contours[0].bounds, (2.5, 0.5, 4.5, 6.5));
    assert_eq!(physical.alpha_at(1, 3), 127);
    assert_eq!(antialiased.alpha_at(1, 3), 0);
    assert_eq!(antialiased.alpha_at(2, 3), 255);
    assert_eq!(geometry.contours[0].bounds, (2.5, 0.5, 4.5, 6.5));
  }

  #[test]
  fn text_bevel_material_uv_tracks_each_emitted_endpoint() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(4.0, 4.0)),
        PathCommand::LineTo(point(28.0, 4.0)),
        PathCommand::LineTo(point(28.0, 28.0)),
        PathCommand::LineTo(point(4.0, 28.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(32.0),
          height: Pt(32.0),
        },
      },
      1.0,
    )
    .expect("closed material test outline");
    let scene = scene(a::PresetCameraValues::OrthographicFront);
    let source = RgbaImage::from_pixel(32, 32, Rgba([180, 120, 60, 255]));
    let outline = RgbaImage::from_pixel(32, 32, Rgba([80, 140, 200, 255]));
    for separate_outline in [false, true] {
      for preset in [a::BevelPresetValues::Circle, a::BevelPresetValues::Angle] {
        let mesh = super::text_bevel_triangles(
          &source,
          &geometry,
          &geometry,
          &geometry,
          Some(1.0),
          super::TextBevelOptions {
            geometry_width: 4.0,
            normal_width: 4.0,
            height: 3.0,
            preset: Some(preset),
            scene: &scene,
            projection: camera_projection(&scene, 0.0),
            model_surface: Static3dSurface {
              left_px: 0.0,
              top_px: 0.0,
              width_px: 32.0,
              height_px: 32.0,
            },
            pixels_per_point: 1.0,
            surface_z: 0.0,
            material: Some(a::PresetMaterialTypeValues::WarmMatte),
            geometry_lighting: super::Static3dGeometryLighting::Text,
            back_face: false,
            height_direction: -1.0,
            fill_material: Some(&source),
            outline_material: separate_outline.then_some(&outline),
            outline_coverage: separate_outline.then_some(&outline),
            fill_uniform_paint_opacity: Some(1.0),
            outline_uniform_paint_opacity: Some(1.0),
            outline_material_inset_px: separate_outline.then_some(1.0),
            fill_has_authored_transparency: false,
            outline_has_authored_transparency: false,
          },
        );
        assert!(!mesh.triangles.is_empty());
        let mut checked = 0;
        for vertex in mesh.triangles.iter().flat_map(|triangle| triangle.vertices) {
          let Some(material) = vertex.textured_material else {
            continue;
          };
          let endpoint = vertex
            .bevel_collision
            .expect("source endpoint")
            .source_point;
          let expected = geometry.map_point_to(&geometry, endpoint);
          assert!(
            (material.material_point.0 - expected.0).abs() < 1.0e-5
              && (material.material_point.1 - expected.1).abs() < 1.0e-5,
            "{preset:?}, outline={separate_outline}: endpoint {expected:?}, UV {:?}",
            material.material_point,
          );
          checked += 1;
        }
        assert!(checked >= 12, "must exercise a complete textured mesh");
      }
    }
  }

  #[test]
  fn text_bevel_inset_stops_when_opposite_edges_reach_the_medial_boundary() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(0.0, 0.0)),
        PathCommand::LineTo(point(10.0, 0.0)),
        PathCommand::LineTo(point(10.0, 20.0)),
        PathCommand::LineTo(point(0.0, 20.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(10.0),
          height: Pt(20.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");

    assert!((text_geometry_boundary_distance(&geometry, (5.0, 10.0)) - 5.0).abs() < 1.0e-6);
    assert!(text_geometry_inset_contains(&geometry, (5.0, 10.0), 5.0));
    assert!(!text_geometry_inset_contains(&geometry, (5.0, 10.0), 5.11));
    assert!(text_geometry_inset_contains(&geometry, (6.0, 10.0), 4.0));
    assert!(!text_geometry_inset_contains(&geometry, (6.0, 10.0), 6.0));
  }

  #[test]
  fn text_planar_inset_respects_the_miter_at_a_concave_join() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(0.0, 0.0)),
        PathCommand::LineTo(point(10.0, 0.0)),
        PathCommand::LineTo(point(10.0, 10.0)),
        PathCommand::LineTo(point(15.0, 5.0)),
        PathCommand::LineTo(point(25.0, 5.0)),
        PathCommand::LineTo(point(25.0, 25.0)),
        PathCommand::LineTo(point(0.0, 25.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(25.0),
          height: Pt(25.0),
        },
      },
      1.0,
    )
    .expect("closed concave text outline");
    let inset = TextPlanarInset::new(&geometry, 3.0, None, false);
    let concave_join = (7.25, 16.75);

    // Finite-segment distance alone mistakes the area behind the reflex
    // vertex for the cap. The intersecting offset edge lines retain the
    // mitered bevel wedge there, while an ordinary deep interior point stays
    // in the cap under both predicates.
    assert!(text_geometry_inset_contains(&geometry, concave_join, 3.0));
    assert!(!inset.contains(&geometry, concave_join, 3.0));
    assert!(text_geometry_inset_contains(&geometry, (12.0, 20.0), 3.0));
    assert!(inset.contains(&geometry, (12.0, 20.0), 3.0));
  }

  #[test]
  fn projected_contour_stops_inset_faces_at_their_collapse() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let bounds = Rect {
      origin: point(0.0, 0.0),
      size: Size {
        width: Pt(2.0),
        height: Pt(10.0),
      },
    };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(0.0, 0.0)),
        PathCommand::LineTo(point(2.0, 0.0)),
        PathCommand::LineTo(point(2.0, 10.0)),
        PathCommand::LineTo(point(0.0, 10.0)),
        PathCommand::Close,
      ],
      bounds,
      1.0,
    )
    .expect("rectangle");
    let points = &geometry.contours[0].points;
    let xmin = points.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
    let xmax = points.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
    let ymin = points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
    let ymax = points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
    let triangles = super::text_projected_contour_triangles(
      &geometry,
      super::TextProjectedContourOptions {
        width_px: 4.0,
        base_z: 0.0,
        color: Static3dColor {
          color: RgbColor {
            r: 255,
            g: 255,
            b: 255,
          },
          alpha: 255,
        },
        projection: camera_projection(&scene(a::PresetCameraValues::OrthographicFront), 0.0),
        model_surface: super::Static3dSurface {
          left_px: 0.0,
          top_px: 0.0,
          width_px: 2.0,
          height_px: 10.0,
        },
        pixels_per_point: 1.0,
      },
    );
    assert!(!triangles.is_empty());
    let mut checked = 0;
    for vertex in triangles.iter().flat_map(|triangle| &triangle.vertices) {
      let (x, y) = vertex.point;
      let distance = (x - xmin).min(xmax - x).min(y - ymin).min(ymax - y);
      if distance >= 0.0 {
        let expected =
          2.0 * super::WORD_CONTOUR_HEIGHT_OVER_RADIUS * (1.0 - (distance / 2.0).powi(2));
        assert!(
          (vertex.visibility_depth - expected).abs() < 0.0001,
          "inset face continued past collapse: {x},{y}, depth={} expected={expected}",
          vertex.visibility_depth
        );
        checked += 1;
      }
    }
    assert!(checked > 0);
  }

  #[test]
  fn text_planar_inset_uses_the_complete_direct_inset_terminal_wavefront() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(0.0, 0.0)),
        PathCommand::LineTo(point(10.0, 0.0)),
        PathCommand::LineTo(point(10.0, 6.0)),
        PathCommand::LineTo(point(0.0, 6.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(10.0),
          height: Pt(6.0),
        },
      },
      1.0,
    )
    .expect("closed rectangular text outline");
    let contours = geometry
      .contours
      .iter()
      .map(|contour| contour.points.as_slice())
      .collect::<Vec<_>>();
    let cells = direct_inset_cells(&contours, geometry.solid_on_right, 2.0)
      .expect("complete direct-inset graph");
    let inset = TextPlanarInset::new(&geometry, 2.0, Some(&cells), false);

    assert!(inset.contains(&geometry, (5.0, 3.0), 2.0));
    assert!(!inset.contains(&geometry, (1.5, 3.0), 2.0));
    assert!(!inset.contains(&geometry, (5.0, 1.5), 2.0));
  }

  #[test]
  fn text_planar_front_cap_and_emitted_mesh_share_continuous_boundaries() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(1.03, 1.03)),
        PathCommand::LineTo(point(5.03, 1.03)),
        PathCommand::LineTo(point(5.03, 5.03)),
        PathCommand::LineTo(point(1.03, 5.03)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(8.0),
          height: Pt(8.0),
        },
      },
      1.0,
    )
    .expect("closed text outline");
    let continuous = TextPlanarInset::new(&geometry, 0.0, None, false);
    let emitted = TextPlanarInset::new(&geometry, 0.0, None, true);

    // Both owners retain x=1.03. Source-grid snapping would incorrectly
    // cover the strip between x=1.0 and x=1.03 before camera projection.
    assert!(!continuous.contains(&geometry, (1.02, 3.0), 0.0));
    assert!(!emitted.contains(&geometry, (1.02, 3.0), 0.0));
    assert!(emitted.contains(&geometry, (1.04, 3.0), 0.0));
    assert_eq!(geometry.contours[0].points[0], (1.03, 1.03));
  }

  #[test]
  fn direct_inset_graph_retains_one_camera_visible_hit_with_all_attributes() {
    let far = super::TextDirectInsetGraphOwner {
      boundary: super::TextGeometryBoundaryOwner {
        distance: 0.18,
        contour_index: 0,
        edge_index: 16,
      },
      source_point: (834.44, 400.73),
      profile_strip_index: 0,
      surface_distance_px: 0.4,
    };
    let near = super::TextDirectInsetGraphOwner {
      boundary: super::TextGeometryBoundaryOwner {
        distance: 1.5,
        ..far.boundary
      },
      source_point: (835.76, 400.73),
      profile_strip_index: 3,
      surface_distance_px: 8.7,
    };
    // Source-plane topology is already resolved here. A smaller inset at
    // another source point must not replace the nearest projected surface.
    for reverse in [false, true] {
      let mut hits = [
        (0.000_110_54, [255.0; 4], far),
        (0.000_110_68, [80.0; 4], near),
      ];
      if reverse {
        hits.reverse();
      }
      let mut graph = super::TextDirectInsetGraphSample::default();
      for (depth, color, owner) in hits {
        graph.consider(depth, color, owner);
      }
      assert!(graph.surface.covered);
      assert_eq!(graph.surface.visibility_depth, 0.000_110_68);
      assert_eq!(graph.surface.color, [80.0; 4]);
      let selected = graph.owner.expect("visible graph hit");
      assert_eq!(selected.boundary.distance, near.boundary.distance);
      assert_eq!(selected.source_point, near.source_point);
      assert_eq!(selected.profile_strip_index, near.profile_strip_index);
      assert_eq!(selected.surface_distance_px, near.surface_distance_px);

      // A tied duplicate from another contour cannot replace only the
      // source attributes while keeping the old depth and color.
      let duplicate = super::TextDirectInsetGraphOwner {
        boundary: super::TextGeometryBoundaryOwner {
          contour_index: 1,
          edge_index: 2,
          ..far.boundary
        },
        ..far
      };
      graph.consider(graph.surface.visibility_depth, [120.0; 4], duplicate);
      let selected = graph.owner.expect("stable equal-depth owner");
      assert_eq!(selected.boundary.contour_index, near.boundary.contour_index);
      assert_eq!(selected.boundary.edge_index, near.boundary.edge_index);
      assert_eq!(selected.source_point, near.source_point);
      assert_eq!(graph.surface.color, [80.0; 4]);
    }
  }

  #[test]
  fn direct_inset_topology_selects_the_first_cross_contour_wavefront() {
    let mut topology = super::TextDirectInsetTopologySample::default();
    topology.consider(
      super::TextGeometryBoundaryOwner {
        distance: 5.795_424_5,
        contour_index: 0,
        edge_index: 13,
      },
      (17.25, 9.5),
    );
    topology.consider(
      super::TextGeometryBoundaryOwner {
        distance: 8.218_496,
        contour_index: 1,
        edge_index: 30,
      },
      (40.0, 30.0),
    );

    let owner = topology.owner.expect("first inset owner");
    assert!((owner.distance - 5.795_424_5).abs() < 1.0e-6);
    assert_eq!((owner.contour_index, owner.edge_index), (0, 13));
    assert_eq!(topology.source_point, Some((17.25, 9.5)));

    // A genuinely earlier merge event replaces the frozen owner, while an
    // equal-coordinate duplicate retains deterministic contour ordering.
    topology.consider(
      super::TextGeometryBoundaryOwner {
        distance: 4.0,
        contour_index: 2,
        edge_index: 7,
      },
      (8.0, 6.0),
    );
    topology.consider(
      super::TextGeometryBoundaryOwner {
        distance: 4.0,
        contour_index: 3,
        edge_index: 9,
      },
      (99.0, 99.0),
    );
    let owner = topology.owner.expect("updated inset owner");
    assert_eq!((owner.contour_index, owner.edge_index), (2, 7));
    assert_eq!(topology.source_point, Some((8.0, 6.0)));
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
  fn wordprocessing_effect_plane_remains_at_the_extrusion_base() {
    let run = Static3dStyleParts {
      scene: Some(Box::new(scene(a::PresetCameraValues::PerspectiveLeft))),
      shape: Some(Box::new(a::Shape3DType {
        extrusion_height: Some(CoordinateValue::Emu(63_500)),
        bevel_top: Some(a::BevelTop {
          width: Some(CoordinateValue::Emu(38_100)),
          height: Some(CoordinateValue::Emu(38_100)),
          ..a::BevelTop::default()
        }),
        ..a::Shape3DType::default()
      })),
      ..Static3dStyleParts::default()
    };
    let style = resolve_static_3d_style(None, Some(&run)).expect("run-level static 3-D");
    let projection = camera_projection(&style.scene, 0.0);
    let mut source = RgbaImage::new(120, 80);
    for y in 24..56 {
      for x in 30..90 {
        source.put_pixel(x, y, Rgba([160, 180, 200, 255]));
      }
    }
    let surface = Static3dSurface {
      left_px: 0.0,
      top_px: 0.0,
      width_px: 120.0,
      height_px: 80.0,
    };

    let front = project_static_3d_front_face(&source, projection, &style.shape, 1.0, Some(surface));
    let effect = project_wordprocessing_static_3d_effect_plane(
      &source,
      projection,
      &style,
      1.0,
      Some(surface),
    );
    let front_bounds = super::alpha_bounds(&front).expect("front alpha");
    let effect_bounds = super::alpha_bounds(&effect).expect("effect alpha");
    let region = Static3dOutputBounds {
      left_pt: 30.0,
      top_pt: 24.0,
      right_pt: 90.0,
      bottom_pt: 56.0,
    };
    let front_region =
      projected_front_region_output_bounds(projection, &style.shape, 120.0, 80.0, region);
    let effect_region =
      projected_wordprocessing_effect_region_output_bounds(projection, &style, 120.0, 80.0, region);

    // perspectiveLeft moves the +8pt solid front toward page left. The W14
    // effect and its alignment rectangle stay on the original z=0
    // text/backdrop plane instead.
    assert!(
      effect_bounds.0 > front_bounds.0,
      "{effect_bounds:?} <= {front_bounds:?}"
    );
    assert!(
      effect_region.left_pt > front_region.left_pt,
      "{effect_region:?} <= {front_region:?}"
    );
    assert_eq!(style.wordprocessing_effect_plane_z_pt, Some(0.0));
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
  fn direct_inset_join_intersects_both_swept_edge_lines() {
    let inset = 8.0;
    let contour = [(0.0, -10.0), (0.0, 0.0), (5.0, -5.0), (10.0, -10.0)];
    let offset = offset_text_3d_contour(&contour, inset, true);
    let join = offset[1];

    // The incoming vertical edge sweeps to x=-inset. The outgoing diagonal
    // sweeps along its inward unit normal, so its supporting line becomes
    // x+y=sqrt(2)*inset. Their exact intersection is longer than a
    // conventional miter-limit-2 join and is required by the inset graph.
    assert!((join.0 + inset).abs() < 0.000_1, "{join:?}");
    assert!(
      (join.0 + join.1 - std::f32::consts::SQRT_2 * inset).abs() < 0.000_1,
      "{join:?}"
    );
    assert!(join.0.hypot(join.1) > 2.0 * inset, "{join:?}");
  }

  #[test]
  fn top_and_bottom_bevels_share_ecma_defaults_and_authored_surface_resolution() {
    let screen_pixels_per_point = 4.0 / 3.0;
    let zero_width = resolve_bevel(
      Some(0),
      Some(25_400),
      Some(a::BevelPresetValues::Angle),
      screen_pixels_per_point,
    );
    let quarter_point = resolve_bevel(
      Some(3_175),
      Some(25_400),
      Some(a::BevelPresetValues::Angle),
      screen_pixels_per_point,
    );
    let half_point = resolve_bevel(
      Some(6_350),
      Some(25_400),
      Some(a::BevelPresetValues::Angle),
      screen_pixels_per_point,
    );
    let defaults = resolve_bevel(None, None, None, screen_pixels_per_point);

    assert!(zero_width.has_authored_surface());
    assert!(zero_width.authored_width_px.abs() < f32::EPSILON);
    assert!((zero_width.height_px - 8.0 / 3.0).abs() < 0.000_1);
    assert!(quarter_point.has_authored_surface());
    assert!(half_point.has_authored_surface());
    assert!((quarter_point.terminal_inset_px - 1.0 / 3.0).abs() < 0.000_1);
    assert!((half_point.terminal_inset_px - 2.0 / 3.0).abs() < 0.000_1);
    assert!((defaults.authored_width_px - 8.0).abs() < f32::EPSILON);
    assert!((defaults.height_px - 8.0).abs() < f32::EPSILON);
    assert_eq!(defaults.preset, Some(a::BevelPresetValues::Circle));
  }

  #[test]
  fn zero_width_bevel_heights_form_the_office_ordered_depth_chain() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    scene.camera.rotation = Some(a::Rotation {
      latitude: 0,
      longitude: 16_200_000,
      revolution: 0,
    });
    let projection = camera_projection(&scene, 0.0);
    let flat_shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(152_400)),
      ..a::Shape3DType::default()
    };
    let beveled_shape = a::Shape3DType {
      extrusion_height: Some(CoordinateValue::Emu(152_400)),
      bevel_top: Some(a::BevelTop {
        width: Some(CoordinateValue::Emu(0)),
        height: Some(CoordinateValue::Emu(25_400)),
        preset: Some(a::BevelPresetValues::Angle),
      }),
      bevel_bottom: Some(a::BevelBottom {
        width: Some(CoordinateValue::Emu(0)),
        height: Some(CoordinateValue::Emu(25_400)),
        preset: Some(a::BevelPresetValues::Angle),
      }),
      ..a::Shape3DType::default()
    };
    let flat = projected_output_bounds(projection, &flat_shape, 64.0, 36.0);
    let beveled = projected_output_bounds(projection, &beveled_shape, 64.0, 36.0);
    let flat_span = flat.right_pt - flat.left_pt;
    let beveled_span = beveled.right_pt - beveled.left_pt;

    // The 12 pt extrusion gains one independent 2 pt height at each end. The
    // Office side-view controls keep the z reference edge fixed and extend
    // both heights through the opposite edge of the ordered depth chain.
    assert!((beveled_span - flat_span - 4.0).abs() < 0.000_1);
    assert!((beveled.right_pt - flat.right_pt).abs() < 0.000_1);
    assert!((flat.left_pt - beveled.left_pt - 4.0).abs() < 0.000_1);
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
  fn word_text_bevel_profile_separates_w14_geometry_from_drawingml_geometry() {
    let circle_outer = bevel_profile_sample(Some(a::BevelPresetValues::Circle), 0, 0.0);
    let circle_outer_geometry =
      word_text_bevel_geometry_profile(Some(a::BevelPresetValues::Circle), circle_outer);
    let circle_outer_lighting =
      word_text_bevel_lighting_profile(Some(a::BevelPresetValues::Circle), circle_outer_geometry);
    assert_eq!(circle_outer_geometry.height, circle_outer.inset);
    assert_eq!(circle_outer_geometry.inset, circle_outer.height);
    assert!((circle_outer_lighting.height_tangent - 1.668_81).abs() < 0.001);
    assert!(circle_outer_lighting.inset_tangent.abs() < 0.001);

    let circle_quarter = bevel_profile_sample(Some(a::BevelPresetValues::Circle), 0, 0.25);
    let circle_quarter_geometry =
      word_text_bevel_geometry_profile(Some(a::BevelPresetValues::Circle), circle_quarter);
    let circle_quarter_lighting =
      word_text_bevel_lighting_profile(Some(a::BevelPresetValues::Circle), circle_quarter_geometry);
    assert_eq!(circle_quarter_geometry.height, circle_quarter.inset);
    assert_eq!(circle_quarter_geometry.inset, circle_quarter.height);
    assert_eq!(
      circle_quarter_lighting.height_tangent,
      circle_quarter.inset_tangent
    );
    assert_eq!(
      circle_quarter_lighting.inset_tangent,
      circle_quarter.height_tangent
    );

    let angle = bevel_profile_sample(Some(a::BevelPresetValues::Angle), 0, 0.5);
    let angle_geometry = word_text_bevel_geometry_profile(Some(a::BevelPresetValues::Angle), angle);
    assert_eq!(angle_geometry.height, angle.height);
    assert_eq!(angle_geometry.inset, angle.inset);
    assert_eq!(
      word_text_bevel_lighting_profile(Some(a::BevelPresetValues::Angle), angle_geometry)
        .height_tangent,
      angle.height_tangent
    );
    assert_eq!(
      word_text_bevel_lighting_profile(Some(a::BevelPresetValues::Angle), angle_geometry)
        .inset_tangent,
      angle.inset_tangent
    );

    let relaxed_quarter = bevel_profile_sample(Some(a::BevelPresetValues::RelaxedInset), 0, 0.25);
    let relaxed_quarter_geometry =
      word_text_bevel_geometry_profile(Some(a::BevelPresetValues::RelaxedInset), relaxed_quarter);
    let relaxed_quarter_lighting = word_text_bevel_lighting_profile(
      Some(a::BevelPresetValues::RelaxedInset),
      relaxed_quarter_geometry,
    );
    assert_eq!(relaxed_quarter_lighting.height, relaxed_quarter.inset);
    assert_eq!(relaxed_quarter_lighting.inset, relaxed_quarter.height);
    assert_eq!(
      relaxed_quarter_lighting.height_tangent,
      relaxed_quarter.inset_tangent
    );
    assert_eq!(
      relaxed_quarter_lighting.inset_tangent,
      relaxed_quarter.height_tangent
    );

    let relaxed_fold = bevel_profile_sample(Some(a::BevelPresetValues::RelaxedInset), 1, 0.0);
    assert!(relaxed_fold.height_tangent > 0.0);
    assert!(relaxed_fold.inset_tangent < 0.0);
    let relaxed_fold_geometry =
      word_text_bevel_geometry_profile(Some(a::BevelPresetValues::RelaxedInset), relaxed_fold);
    let relaxed_fold_lighting = word_text_bevel_lighting_profile(
      Some(a::BevelPresetValues::RelaxedInset),
      relaxed_fold_geometry,
    );
    assert!(relaxed_fold_lighting.height_tangent < 0.0);
    assert!(relaxed_fold_lighting.inset_tangent > 0.0);

    let relaxed_terminal =
      word_text_bevel_terminal_profile(Some(a::BevelPresetValues::RelaxedInset));
    assert!((relaxed_terminal.height - 0.64).abs() < f32::EPSILON);
    assert!((relaxed_terminal.inset - 1.0).abs() < f32::EPSILON);
  }

  #[test]
  fn word_text_profile_realization_matches_office_width_height_boundaries() {
    // Exact-option Office uncut profiles, before inset collisions or texture
    // splitting. Precision is the observed producer input, not an image fit.
    let cases: &[(f32, f32, f32, &[f32])] = &[
      (3.0, 3.0, 0.004_910_706_5, &[0.0, 0.5, 1.0]),
      (1.0, 1.0, 0.004_919_391, &[0.0, 0.5, 1.0]),
      (1.0, 3.0, 0.004_910_706_5, &[0.0, 0.5, 1.0]),
      (1.0, 9.0, 0.004_884_649_5, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (3.0, 1.0, 0.004_919_391, &[0.0, 0.5, 1.0]),
      (3.0, 9.0, 0.004_884_649_5, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (9.0, 1.0, 0.004_919_391, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (9.0, 3.0, 0.004_910_706_5, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (9.0, 9.0, 0.004_884_649_5, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (1.0, 0.1, 0.004_921_563, &[0.0, 0.5, 1.0]),
      (1.0, 0.5, 0.004_921_563, &[0.0, 0.5, 1.0]),
      (1.0, 3.2, 0.004_909_837, &[0.0, 0.5, 0.75, 1.0]),
      (1.0, 3.3, 0.004_909_403, &[0.0, 0.5, 0.75, 1.0]),
      (1.0, 3.4, 0.004_908_969, &[0.0, 0.5, 0.75, 1.0]),
      (1.0, 4.5, 0.004_904_192, &[0.0, 0.5, 0.75, 1.0]),
      (1.0, 4.6, 0.004_903_758, &[0.0, 0.5, 0.75, 1.0]),
      (1.0, 6.7, 0.004_894_638, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (1.0, 6.8, 0.004_894_204, &[0.0, 0.25, 0.5, 0.75, 1.0]),
      (
        1.0,
        18.0,
        0.004_845_565,
        &[0.0, 0.25, 0.5, 0.625, 0.75, 0.875, 1.0],
      ),
    ];
    for &(width_pt, height_pt, tolerance_inches, expected) in cases {
      for dpi in [72.0, 96.0, 200.0, 600.0, 1200.0] {
        let actual = word_text_bevel_segment_parameters(
          width_pt * dpi / 72.0,
          height_pt * dpi / 72.0,
          Some(a::BevelPresetValues::Circle),
          0,
          tolerance_inches * dpi,
        );
        assert_eq!(actual, expected, "{width_pt}x{height_pt}, {dpi} DPI");
        assert_eq!(
          word_text_bevel_segment_parameters(
            width_pt * dpi / 72.0,
            height_pt * dpi / 72.0,
            Some(a::BevelPresetValues::Angle),
            0,
            tolerance_inches * dpi,
          ),
          [0.0, 1.0],
        );
      }
    }
  }

  #[test]
  fn word_text_profile_hfd_preserves_the_non_realized_tolerance() {
    let preset = Some(a::BevelPresetValues::RelaxedInset);
    let interval_counts = |extent| {
      (0..bevel_profile(preset).len())
        .map(|segment| {
          word_text_bevel_segment_parameters(extent, extent, preset, segment, 0.25).len() - 1
        })
        .collect::<Vec<_>>()
    };

    // Non-realized/orthographic callers retain their existing precision.
    // These arithmetic checks do not establish perspective Word's policy.
    assert_eq!(interval_counts(8.333_333), [4, 4]);
    assert_eq!(interval_counts(24.0), [7, 4]);
    assert_eq!(interval_counts(48.0), [8, 8]);
    assert_eq!(
      word_text_bevel_segment_parameters(24.0, 24.0, Some(a::BevelPresetValues::Angle), 0, 0.25),
      [0.0, 1.0]
    );
  }

  #[test]
  fn word_text_angle_profile_splits_at_the_outline_material_boundary() {
    let normal_width = 3.0 * 200.0 / 72.0;
    let centered_outline_inset = 2.0 * 0.5 * 200.0 / 72.0;
    let (interpolated, strips) = text_bevel_profile_strips(
      normal_width,
      normal_width,
      Some(a::BevelPresetValues::Angle),
      Static3dGeometryLighting::Text,
      Some(centered_outline_inset),
      None,
      None,
    );

    assert!(interpolated);
    assert_eq!(strips.len(), 2);
    assert!(strips[0].outer.inset.abs() < f32::EPSILON);
    assert!((strips[0].inner.inset - 1.0 / 3.0).abs() < 0.000_1);
    assert!((strips[0].inner.height - strips[1].outer.height).abs() < f32::EPSILON);
    assert!((strips[0].inner.inset - strips[1].outer.inset).abs() < f32::EPSILON);
    assert!((strips[1].inner.inset - 1.0).abs() < f32::EPSILON);
    let (_, unsplit) = text_bevel_profile_strips(
      normal_width,
      normal_width,
      Some(a::BevelPresetValues::Angle),
      Static3dGeometryLighting::Text,
      None,
      None,
      None,
    );
    assert_eq!(unsplit.len(), 1);
  }

  #[test]
  fn word_text_circle_profile_material_boundary_has_a_nonlinear_parameter() {
    let normal_width = 3.0 * 200.0 / 72.0;
    let centered_outline_inset = 2.0 * 0.5 * 200.0 / 72.0;
    let normalized_boundary = centered_outline_inset / normal_width;
    let parameters = word_text_bevel_material_boundary_parameters(
      Some(a::BevelPresetValues::Circle),
      0,
      normalized_boundary,
    );

    assert_eq!(parameters.len(), 1);
    let boundary_parameter = parameters[0];
    assert!((boundary_parameter - normalized_boundary).abs() > 0.05);
    let boundary_profile = word_text_bevel_geometry_profile(
      Some(a::BevelPresetValues::Circle),
      bevel_profile_sample(Some(a::BevelPresetValues::Circle), 0, boundary_parameter),
    );
    assert!((boundary_profile.inset - normalized_boundary).abs() < 1.0e-5);
  }

  #[test]
  fn word_text_profile_preserves_the_relaxed_inset_cusp_normals() {
    let preset = Some(a::BevelPresetValues::RelaxedInset);
    let (interpolated, strips) = text_bevel_profile_strips(
      8.333_333,
      8.333_333,
      preset,
      Static3dGeometryLighting::Text,
      None,
      None,
      None,
    );

    assert!(interpolated);
    assert_eq!(strips.len(), 8);
    let incoming = strips[3].inner;
    let outgoing = strips[4].outer;
    assert_eq!(incoming.height, outgoing.height);
    assert_eq!(incoming.inset, outgoing.inset);
    assert!(incoming.height_tangent > 0.0);
    assert!(outgoing.height_tangent < 0.0);
    assert_ne!(incoming.height_tangent, outgoing.height_tangent);
    assert_ne!(incoming.inset_tangent, outgoing.inset_tangent);
  }

  #[test]
  fn circle_bevel_outer_band_uses_the_material_light_rig() {
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
        width: 4.0,
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
        height_direction: -1.0,
      },
    );

    // The profile coordinate is geometry, not opacity. Its boundary remains
    // fully covered and must be shaded by the material/light rig rather than
    // retaining the unlit source gray. Word height/light-rig sweeps expose
    // the same independent alpha and RGB behavior at this outer band.
    let outer_edge = bevel.get_pixel(0, 4);
    assert_eq!(outer_edge[3], 255);
    assert_ne!(outer_edge[0], 200, "outer edge was {outer_edge:?}");
  }

  #[test]
  fn raster_bevel_preserves_fractional_authored_width() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let source = RgbaImage::from_pixel(11, 11, Rgba([220, 220, 220, 255]));
    let projection = camera_projection(&scene, 0.0);
    let render = |width| {
      let mut bevel = RgbaImage::new(11, 11);
      composite_bevel(
        &mut bevel,
        &source,
        BevelOptions {
          width,
          height: 4.0,
          preset: Some(a::BevelPresetValues::Circle),
          scene: &scene,
          projection,
          model_surface: Static3dSurface {
            left_px: 0.0,
            top_px: 0.0,
            width_px: 11.0,
            height_px: 11.0,
          },
          pixels_per_point: 1.0,
          surface_z: 0.0,
          material: Some(a::PresetMaterialTypeValues::Matte),
          back_face: false,
          height_direction: -1.0,
        },
      );
      bevel
    };

    assert_ne!(render(4.0), render(4.25));
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
  fn text_curve_normals_smooth_only_curve_owned_vertices() {
    let points = [(0.0, 0.0), (10.0, 0.0), (20.0, 1.0), (20.0, 10.0)];
    let authored_lines = text_3d_contour_edge_normals(
      &points,
      &[false; 4],
      true,
      Text3dNormalJoinPolicy::SourceCurves,
    );
    let curve_join = text_3d_contour_edge_normals(
      &points,
      &[false, true, true, false],
      true,
      Text3dNormalJoinPolicy::SourceCurves,
    );

    assert_ne!(authored_lines[0].1, authored_lines[1].0);
    assert_eq!(curve_join[0].1, curve_join[1].0);
    assert_ne!(curve_join[1].1, curve_join[2].0);
  }

  #[test]
  fn text_extrusion_normals_preserve_planar_sides_at_shallow_joins() {
    let points = [(0.0, 0.0), (10.0, 0.0), (20.0, 1.0), (20.0, 10.0)];
    let normals = text_3d_contour_edge_normals(
      &points,
      &[false; 4],
      true,
      Text3dNormalJoinPolicy::ExtrusionSurfaces,
    );

    assert_eq!(normals[0].0, normals[0].1);
    assert_eq!(normals[1].0, normals[1].1);
    assert_ne!(normals[0].1, normals[1].0);
    assert_ne!(normals[1].1, normals[2].0);
  }

  #[test]
  fn text_extrusion_curve_boundary_does_not_bend_the_neighboring_plane() {
    let points = [(0.0, 0.0), (10.0, 0.0), (20.0, 1.0), (20.0, 10.0)];
    let normals = text_3d_contour_edge_normals(
      &points,
      &[false, false, true, false],
      true,
      Text3dNormalJoinPolicy::ExtrusionSurfaces,
    );
    assert_eq!(normals[0], ([0.0, -1.0], [0.0, -1.0]));
    assert_ne!(normals[1].0, normals[1].1);
    assert_ne!(normals[0].1, normals[1].0);
  }

  #[test]
  fn text_extrusion_curves_retain_shallow_join_interpolation() {
    let points = [(0.0, 0.0), (10.0, 0.0), (20.0, 1.0), (20.0, 10.0)];
    let normals = text_3d_contour_edge_normals(
      &points,
      &[true; 4],
      true,
      Text3dNormalJoinPolicy::ExtrusionSurfaces,
    );
    assert_eq!(normals[0].1, normals[1].0);
    assert_ne!(normals[1].1, normals[2].0);
    assert_eq!(
      normals,
      text_3d_contour_edge_normals(
        &points,
        &[true; 4],
        true,
        Text3dNormalJoinPolicy::SourceCurves,
      ),
    );
  }

  #[test]
  fn text_extrusion_vertices_preserve_sub_sixteenth_source_positions() {
    for x in [1.03, 1.03124, 1.03125, 1.03126, 1.04, -1.03125] {
      let first = (x, -x);
      let second = (x + 0.7, x + 1.3);
      assert_eq!(text_extrusion_edge_point(first, second, 0.0), first);
      assert_eq!(text_extrusion_edge_point(first, second, 1.0), second);
      let middle = text_extrusion_edge_point(first, second, 0.5);
      assert!((middle.0 - (first.0 + second.0) * 0.5).abs() < 2.0e-7);
      assert!((middle.1 - (first.1 + second.1) * 0.5).abs() < 2.0e-7);
    }
  }

  #[test]
  fn text_geometry_keeps_only_true_source_creases_through_flattening() {
    let point = |x, y| Point { x: Pt(x), y: Pt(y) };
    let geometry = Static3dTextGeometry::from_page_path(
      &[
        PathCommand::MoveTo(point(0.0, 0.0)),
        PathCommand::LineTo(point(10.0, 0.0)),
        PathCommand::CubicTo {
          control1: point(13.0, 0.0),
          control2: point(17.0, 10.0),
          end: point(20.0, 10.0),
        },
        PathCommand::LineTo(point(0.0, 10.0)),
        PathCommand::Close,
      ],
      Rect {
        origin: point(0.0, 0.0),
        size: Size {
          width: Pt(20.0),
          height: Pt(10.0),
        },
      },
      1.0,
    )
    .expect("closed geometry");
    let contour = &geometry.contours[0];
    let line_end = contour
      .points
      .iter()
      .position(|&(x, y)| (x - 10.0).abs() < 1.0e-4 && y.abs() < 1.0e-4)
      .expect("line/curve boundary");

    assert!(!contour.incoming_curve_edges[line_end]);
    assert!(contour.incoming_curve_edges[(line_end + 1) % contour.points.len()]);
    let curve_points = contour
      .incoming_curve_edges
      .iter()
      .filter(|is_curve| **is_curve)
      .collect::<Vec<_>>();
    assert!(curve_points.len() > 2, "the source cubic was not flattened");
    assert_eq!(
      contour
        .longitudinal_contour_joins
        .iter()
        .filter(|is_join| **is_join)
        .count(),
      3,
      "neither flattened vertices nor a G1 line/curve boundary is a crease"
    );
    let curve_end = contour
      .points
      .iter()
      .position(|&(x, y)| (x - 20.0).abs() < 1.0e-4 && (y - 10.0).abs() < 1.0e-4)
      .expect("curve/line boundary");
    assert!(contour.longitudinal_contour_joins[curve_end]);
    assert!(!contour.longitudinal_contour_joins[line_end]);
    assert!(
      contour
        .incoming_curve_edges
        .iter()
        .zip(&contour.longitudinal_contour_joins)
        .any(|(is_curve, is_join)| *is_curve && !*is_join)
    );
  }

  #[test]
  fn text_source_join_distinguishes_g1_noise_from_real_corners() {
    let angle = |degrees: f64| degrees.to_radians().sin_cos();
    let (g1_sine, g1_cosine) = angle(0.0125);
    let (shallow_sine, shallow_cosine) = angle(0.08);

    assert!(!text_3d_source_tangents_form_contour_join(
      (1.0, 0.0),
      (g1_cosine, g1_sine),
    ));
    assert!(text_3d_source_tangents_form_contour_join(
      (1.0, 0.0),
      (shallow_cosine, shallow_sine),
    ));
    assert!(text_3d_source_tangents_form_contour_join(
      (1.0, 0.0),
      (-1.0, 0.0),
    ));
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

    let distances = bevel_distance_field(&source, 4.0);
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
  fn word_screen_angle_bevel_normal_uses_the_non_uniform_page_plane_scale() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      rotation: Some(a::Rotation {
        latitude: 0,
        longitude: 0,
        revolution: 1_200_000,
      }),
    };
    let projection = camera_projection(&scene, 0.0);
    let depth_scale = 4.0 / 3.0;
    let page_plane_scale_x = 46.0 / 35.22;
    let page_plane_scale_y = 48.0 / 36.72;
    let device_channel = |inset_pt: f32| {
      let normal = transformed_bevel_surface_normal(
        [0.0, 1.0],
        2.0 * depth_scale,
        inset_pt * depth_scale,
        page_plane_scale_x,
        page_plane_scale_y,
        depth_scale,
      );
      let normal = lighting_surface_normal(&scene, projection, normal);
      let shade =
        legacy_material_diffuse_shade(&scene, normal, Some(a::PresetMaterialTypeValues::Matte));
      shade_gouraud_channel(255, shade[0])
    };

    // Reopened Word Screen controls vary only bevel inset. Their flat bottom
    // faces are 195 at 4 pt and 211 at 5 pt. The two values independently
    // cross-check the inverse-transpose transform rather than a color offset.
    assert_eq!(device_channel(4.0), 195);
    assert_eq!(device_channel(5.0), 211);
  }

  #[test]
  fn word_screen_shape_surfaces_use_the_legacy_fixed_point_lattice() {
    // A Flat-rig control isolates the final color conversion from directional
    // lighting. These transition witnesses span zero, the midpoint, the
    // upper endpoint, and both sides of the lattice's 8-bit expansion.
    for (input, expected) in [
      (0, 0),
      (1, 0),
      (2, 2),
      (128, 128),
      (129, 128),
      (130, 129),
      (254, 253),
      (255, 255),
    ] {
      assert_eq!(
        shade_fixed_gouraud_channel_with_specular(input, 1.0, 0.0),
        expected
      );
    }

    // The ThreePoint control changes only extrusion gray. GDB resolves the
    // wall shade to this value; the transition witnesses below independently
    // pin final fixed-point quantization instead of a per-case color offset.
    let three_point_wall_shade = 0.520_460_9;
    for (input, expected) in [
      (2, 0),
      (4, 2),
      (23, 12),
      (96, 50),
      (245, 128),
      (249, 129),
      (253, 131),
      (255, 131),
    ] {
      assert_eq!(
        shade_fixed_gouraud_channel_with_specular(input, three_point_wall_shade, 0.0),
        expected
      );
    }

    // Two reopened Screen controls retain the same shape, camera, ThreePoint
    // rig, matte material, 2-pt height, and 12-pt extrusion. The first varies
    // only a 0.25-pt Angle bevel's fill; the second varies only a 4-pt Angle
    // bevel's fill. Both expose the same early packed-color lattice as the
    // extrusion wall, while the text-material path remains high precision.
    for (shade, transfers, high_precision) in [
      (
        0.894,
        &[(64, 56), (128, 114), (192, 171), (255, 227)][..],
        &[57, 114, 172, 228][..],
      ),
      (
        1.094,
        &[(124, 135), (126, 137), (127, 137), (128, 139), (130, 141)][..],
        &[136, 138, 139, 140, 142][..],
      ),
    ] {
      for ((input, expected), text_expected) in transfers.iter().zip(high_precision) {
        let color = Static3dColor {
          color: RgbColor {
            r: *input,
            g: 0,
            b: 0,
          },
          alpha: 255,
        };
        assert_eq!(
          shaded_geometry_pixel_with_specular(
            color,
            [shade; 3],
            [0.0; 3],
            255,
            Static3dGeometryLighting::Shape,
          )[0],
          *expected
        );
        assert_eq!(
          shaded_geometry_pixel_with_specular(
            color,
            [shade; 3],
            [0.0; 3],
            255,
            Static3dGeometryLighting::Text,
          )[0],
          *text_expected
        );
      }
    }
  }

  #[test]
  fn word_text_three_point_uses_the_d3d_light_travel_direction() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let front_shade = light_rig_surface_shade(&scene, [0.0, 0.0, 1.0]);
    let bottom_angle_shade = light_rig_surface_shade(
      &scene,
      [
        0.0,
        std::f32::consts::FRAC_1_SQRT_2,
        std::f32::consts::FRAC_1_SQRT_2,
      ],
    );

    // The 20-case Word matrix independently varies material Fresnel and five
    // Angle-bevel slopes. Its matte controls have no specular or Fresnel term:
    // the front plane resolves at about 1.044 times Shape color, and the 45°
    // page-bottom surface at about 0.512. Both values are the sum of the
    // negative transformed ThreePoint directions prescribed by D3D.
    for channel in front_shade {
      assert!((channel - 1.043_6).abs() < 0.000_2);
    }
    for channel in bottom_angle_shade {
      assert!((channel - 0.512_2).abs() < 0.000_2);
    }
  }

  #[test]
  fn word_text_diffuse_fresnel_matches_the_office_angle_bevel_controls() {
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let view = [0.0, 0.0, 1.0];

    for (height_over_width, office_matte_red, office_metal_red) in
      [(2.0_f32, 19_u8, 23_u8), (4.0, 11, 28)]
    {
      let inverse_length = (height_over_width * height_over_width + 1.0).sqrt().recip();
      let normal = [height_over_width * inverse_length, 0.0, inverse_length];
      let matte = material_diffuse_shade(
        &scene,
        normal,
        view,
        Some(a::PresetMaterialTypeValues::Matte),
      );
      let metal = material_diffuse_shade(
        &scene,
        normal,
        view,
        Some(a::PresetMaterialTypeValues::Metal),
      );

      assert_eq!(shade_gouraud_channel(64, matte[0]), office_matte_red);
      assert!(
        shade_gouraud_channel(64, metal[0]).abs_diff(office_metal_red) <= 1,
        "height/width={height_over_width}"
      );
    }
  }

  #[test]
  fn diffuse_fresnel_preset_table_covers_every_ecma_material() {
    use a::PresetMaterialTypeValues as M;

    for (material, expected) in [
      (M::Clear, -8),
      (M::DarkEdge, -2),
      (M::Flat, -4),
      (M::LegacyMatte, -4),
      (M::LegacyMetal, 0),
      (M::LegacyPlastic, 0),
      (M::LegacyWireframe, 0),
      (M::Matte, 0),
      (M::Metal, 4),
      (M::Plastic, 0),
      (M::Powder, 2),
      (M::SoftEdge, 4),
      (M::SoftMetal, 0),
      (M::TranslucentPowder, 2),
      (M::WarmMatte, 0),
    ] {
      assert_eq!(material_diffuse_fresnel(Some(material)), expected);
    }
    assert_eq!(material_diffuse_fresnel(None), 0);

    let shade = [0.4, 0.8, 1.2];
    let normal = [0.0, 0.8, 0.6];
    let view = [0.0, 0.0, 1.0];
    let positive = apply_material_diffuse_fresnel(shade, normal, view, Some(M::Metal));
    let negative = apply_material_diffuse_fresnel(shade, normal, view, Some(M::DarkEdge));
    let positive_weight = 0.4_f32.powi(4);
    let negative_weight = 0.4_f32.powi(2);
    for channel in 0..3 {
      assert!(
        (positive[channel] - (shade[channel] + (1.0 - shade[channel]) * positive_weight)).abs()
          < 1.0e-6
      );
      assert!((negative[channel] - shade[channel] * (1.0 - negative_weight)).abs() < 1.0e-6);
    }
  }

  #[test]
  fn text_paint_opacity_treats_one_byte_rounding_as_opaque_not_translucent() {
    assert_eq!(recover_text_paint_opacity(129, 0.507_812_5), 1.0);
    assert_eq!(recover_text_paint_opacity(0, 0.001), 0.0);
    assert!((recover_text_paint_opacity(64, 0.5) - 128.0 / 255.0).abs() < f32::EPSILON);
  }

  #[test]
  fn alpha_fresnel_preset_table_and_word_angle_controls_are_complete() {
    use a::PresetMaterialTypeValues as M;

    for (material, expected_base, expected_fresnel) in [
      (M::Clear, 0.1, 1),
      (M::DarkEdge, 1.0, 0),
      (M::Flat, 1.0, 0),
      (M::LegacyMatte, 1.0, 0),
      (M::LegacyMetal, 1.0, 0),
      (M::LegacyPlastic, 1.0, 0),
      (M::LegacyWireframe, 1.0, 0),
      (M::Matte, 1.0, 0),
      (M::Metal, 1.0, 0),
      (M::Plastic, 1.0, 0),
      (M::Powder, 1.0, 0),
      (M::SoftEdge, 1.0, -10),
      (M::SoftMetal, 1.0, 0),
      (M::TranslucentPowder, 0.7, -1),
      (M::WarmMatte, 1.0, 0),
    ] {
      assert_eq!(material_base_alpha(Some(material)), expected_base);
      assert_eq!(material_alpha_fresnel(Some(material)), expected_fresnel);
    }
    assert_eq!(material_base_alpha(None), 1.0);
    assert_eq!(material_alpha_fresnel(None), 0);

    let view = [0.0, 0.0, 1.0];
    let controls = [
      (M::Matte, [255, 255, 255, 255, 255]),
      (M::SoftEdge, [255, 255, 254, 254, 239]),
      (M::TranslucentPowder, [173, 159, 126, 79, 43]),
      (M::Clear, [32, 49, 92, 152, 199]),
    ];
    for (material, expected) in controls {
      for (height_over_width, expected_alpha) in
        [0.25_f32, 0.5, 1.0, 2.0, 4.0].into_iter().zip(expected)
      {
        let inverse_length = (height_over_width * height_over_width + 1.0).sqrt().recip();
        let normal = [height_over_width * inverse_length, 0.0, inverse_length];
        assert_eq!(
          material_surface_alpha(1.0, normal, view, Some(material)),
          expected_alpha,
          "material={material:?} height/width={height_over_width}"
        );
      }
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
      material_diffuse_shade(
        &scene,
        normal,
        normal,
        Some(a::PresetMaterialTypeValues::WarmMatte),
      ),
      material_diffuse_shade(
        &scene,
        normal,
        normal,
        Some(a::PresetMaterialTypeValues::Matte),
      ),
    );
  }

  #[test]
  fn word_text_specular_reflectance_matches_the_fixed_output_controls() {
    use a::PresetMaterialTypeValues as M;

    let shape = [64, 128, 192];
    let normalized = shape.map(|channel| f32::from(channel) / 255.0);
    let nearly = |left: f32, right: f32| (left - right).abs() < 1.0e-6;
    assert_eq!(
      word_text_material_specular_reflectance(Some(M::Matte), shape),
      [0.0; 3]
    );
    let legacy_metal = word_text_material_specular_reflectance(Some(M::LegacyMetal), shape);
    let metal = word_text_material_specular_reflectance(Some(M::Metal), shape);
    let soft_metal = word_text_material_specular_reflectance(Some(M::SoftMetal), shape);
    for channel in 0..3 {
      let expected_metallic = 0.5 + 0.5 * normalized[channel];
      assert!(nearly(legacy_metal[channel], expected_metallic));
      assert!(nearly(metal[channel], expected_metallic));
      assert!(nearly(soft_metal[channel], expected_metallic));
    }
    assert_eq!(
      word_text_material_specular_reflectance(Some(M::Plastic), shape),
      [0.6; 3]
    );

    // The black Metal/Matte pair is the stopping counterexample for the old
    // `Shape`-only implementation: exact-config Word emits a visible Metal
    // highlight while Matte remains black.
    let mut scene = scene(a::PresetCameraValues::OrthographicFront);
    *scene.light_rig = a::LightRig {
      rig: a::LightRigValues::ThreePoints,
      direction: a::LightRigDirectionValues::Top,
      ..a::LightRig::default()
    };
    let normal = [0.421_694_43, 0.567_603_6, std::f32::consts::FRAC_1_SQRT_2];
    let view = [0.0, 0.0, 1.0];
    assert_eq!(
      light_rig_surface_specular(&scene, normal, view, Some(M::Matte), [0; 3]),
      [0.0; 3]
    );
    assert!(
      light_rig_surface_specular(&scene, normal, view, Some(M::Metal), [0; 3])
        .into_iter()
        .any(|channel| channel > 0.0)
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
