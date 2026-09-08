//! Realize the complete hosted Word effect graph before camera sampling.

use super::{FrameBounds, WordprocessingEffectRectangles, WordprocessingStatic3dGlowSource};
use crate::model::common_rect;
use crate::{common, units};
use common::drawingml_image_effects as effects;
use common::drawingml_shape_raster as raster;

pub(super) struct HostedBackdropInput<'a> {
  pub text: &'a common::TextRun<'static>,
  pub effects: &'a effects::ImageEffectContainer,
  pub style: &'a common::drawingml_3d::Static3dStyle,
  pub source: WordprocessingEffectRectangles,
  pub glow: Option<&'a WordprocessingStatic3dGlowSource>,
  pub ink_origin: (f32, f32),
  pub model: FrameBounds,
  pub target_bounds: common::Rect,
  pub target_dimensions: (u32, u32),
  pub target_pixels_per_point: f32,
  pub allocation_pixels_per_point: f32,
  pub rotation_deg: f32,
  pub antialiasing: raster::RasterPrimitiveAntialiasing,
  pub has_reflection: bool,
  pub final_grid: Option<common::drawingml_3d::Static3dTextFinalGrid>,
}

pub(super) struct HostedBackdropOutput {
  pub working: image::RgbaImage,
  pub final_grid: Option<image::RgbaImage>,
}

pub(super) fn render(input: HostedBackdropInput<'_>) -> Option<HostedBackdropOutput> {
  let HostedBackdropInput {
    text,
    effects: graph,
    style,
    source,
    glow,
    ink_origin,
    model,
    target_bounds,
    target_dimensions,
    target_pixels_per_point: ppp,
    allocation_pixels_per_point,
    rotation_deg,
    antialiasing,
    has_reflection,
    final_grid,
  } = input;
  // A scene texture is a source drawable, not the final output allocation.
  // Keep a separate graph so neither UV bounds nor nested finite masks
  // inherit fixed-output guards. The caller's final crop remains unchanged.
  let graph = graph.without_bitmap_allocation_guards();
  let (complete, source_bounds) = source_geometry(&graph, source, glow.map(|g| g.bounds))?;
  let bounds = common_rect(
    ink_origin.0 + complete.left_pt,
    ink_origin.1 + complete.top_pt,
    complete.right_pt - complete.left_pt,
    complete.bottom_pt - complete.top_pt,
  );
  let target_mapping = raster::PageToRasterMapping {
    width_px: target_dimensions.0,
    height_px: target_dimensions.1,
    scale_x: ppp,
    scale_y: ppp,
    translate_x: -target_bounds.origin.x.0 * ppp,
    translate_y: -target_bounds.origin.y.0 * ppp,
    text_hinting: None,
  };
  let plane_z_pt = style.wordprocessing_effect_plane_z_pt.unwrap_or_else(|| {
    style
      .shape
      .z
      .map_or(0.0, |value| units::emu_to_points(value.to_emu()))
  });
  let plan = common::drawingml_3d::BackdropTexturePlan::new(
    bounds,
    target_mapping,
    common::drawingml_3d::Static3dSurface {
      left_px: (model.x_pt - target_bounds.origin.x.0) * ppp,
      top_px: (model.y_pt - target_bounds.origin.y.0) * ppp,
      width_px: model.width_pt * ppp,
      height_px: model.height_pt * ppp,
    },
    common::drawingml_3d::camera_projection(&style.scene, rotation_deg),
    plane_z_pt,
    allocation_pixels_per_point,
  )?;
  let (mapping, crop_x, crop_y) = plan.working_mapping(target_bounds)?;
  let draw = |text: &common::TextRun<'static>, aa| {
    raster::rasterize_vector_items_at_mapping(
      &[common::DisplayItem::Text(text.clone())],
      mapping,
      aa,
    )
  };
  let base = draw(text, antialiasing)?;
  let mut flat = base.clone();
  let contour_width = common::Pt(
    style
      .shape
      .contour_width
      .map_or(0.0, |w| units::emu_to_points(w.to_emu())),
  );
  let contour_color = style.contour_color.map_or(
    common::Color {
      r: 0,
      g: 0,
      b: 0,
      a: 255,
    },
    |c| common::Color {
      r: c.color.r,
      g: c.color.g,
      b: c.color.b,
      a: c.alpha,
    },
  );
  let reflection = if has_reflection {
    if let Some(contour) =
      raster::static_3d_text_reflection_contour_item(text, contour_width, contour_color)
    {
      effects::composite_source_over(&mut flat, &draw(&contour, antialiasing)?);
    }
    Some(
      raster::rasterize_static_3d_text_reflection_source_with_mapping(
        text,
        mapping,
        contour_width,
        contour_color,
        antialiasing,
      )?,
    )
  } else {
    None
  };
  let glow_mask = if let Some(glow) = glow {
    Some(draw(
      &glow.material,
      raster::RasterPrimitiveAntialiasing::PerPrimitive,
    )?)
  } else {
    None
  };
  // Effects retain their CSS-pixel logical units. The common origin is the
  // padded texture's page origin; no geometry is inferred from alpha pixels.
  let css = units::CSS_PIXELS_PER_INCH / units::POINTS_PER_INCH;
  let origin = (
    -mapping.translate_x / mapping.scale_x,
    -mapping.translate_y / mapping.scale_y,
  );
  let to_logical = |b: effects::EffectOutputBounds| effects::ImageEffectContentBounds {
    left_px: (ink_origin.0 + b.left_pt - origin.0) * css,
    top_px: (ink_origin.1 + b.top_pt - origin.1) * css,
    width_px: (b.right_pt - b.left_pt) * css,
    height_px: (b.bottom_pt - b.top_pt) * css,
  };
  let paint = to_logical(source.paint);
  let anchor = to_logical(source.anchor);
  let shadow_anchor = to_logical(source.shadow_anchor);
  let ramp = to_logical(source.ramp);
  let logical_geometry = effects::ImageEffectSourceGeometry {
    paint_left_px: paint.left_px,
    paint_top_px: paint.top_px,
    paint_width_px: paint.width_px,
    paint_height_px: paint.height_px,
    shadow_anchor_left_px: shadow_anchor.left_px,
    shadow_anchor_top_px: shadow_anchor.top_px,
    shadow_anchor_width_px: shadow_anchor.width_px,
    shadow_anchor_height_px: shadow_anchor.height_px,
    anchor_left_px: anchor.left_px,
    anchor_top_px: anchor.top_px,
    anchor_width_px: anchor.width_px,
    anchor_height_px: anchor.height_px,
    ramp_left_px: ramp.left_px,
    ramp_top_px: ramp.top_px,
    ramp_width_px: ramp.width_px,
    ramp_height_px: ramp.height_px,
  };
  let scale = effects::EffectRasterScale {
    x: mapping.scale_x / css,
    y: mapping.scale_y / css,
  };
  let mut remaining = graph.clone();
  let shadow = if let Some(shadow_graph) =
    effects::extract_projected_wordprocessing_shadow_branch(&mut remaining)
  {
    let mut shadow =
      match raster::opaque_static_3d_text_effect_item(text, contour_width, contour_color.a) {
        Some(caster) => draw(&caster, raster::RasterPrimitiveAntialiasing::PerPrimitive)?,
        None => base,
      };
    effects::apply_container_to_padded_image_with_sources_on_raster(
      &mut shadow,
      &shadow_graph,
      logical_geometry,
      effects::ImageEffectSourceImages::default(),
      scale,
    )?;
    Some(shadow)
  } else {
    None
  };
  if reflection.is_some() {
    effects::bind_wordprocessing_reflection_paint(&mut remaining);
  }
  if glow_mask.is_some() {
    effects::bind_wordprocessing_glow_mask(&mut remaining);
  }
  remaining
    .effects
    .retain(|e| !matches!(e, effects::ImageEffect::Identity));
  effects::apply_container_to_padded_image_with_sources_on_raster(
    &mut flat,
    &remaining,
    logical_geometry,
    effects::ImageEffectSourceImages {
      children: shadow.as_ref(),
      effect_mask: glow_mask.as_ref(),
      reflection_paint: reflection.as_ref(),
      bounds: effects::ImageEffectSourcePixelBounds {
        children: source_bounds.children.map(to_logical),
        effect_mask: source_bounds.effect_mask.map(to_logical),
        reflection_paint: source_bounds.reflection_paint.map(to_logical),
        ..Default::default()
      },
      ..Default::default()
    },
    scale,
  )?;
  let texture = image::imageops::crop_imm(
    &flat,
    crop_x,
    crop_y,
    plan.mapping.width_px,
    plan.mapping.height_px,
  )
  .to_image();
  let final_grid = match final_grid {
    Some(grid) => Some(plan.project_on_grid(&texture, grid)?),
    None => None,
  };
  Some(HostedBackdropOutput {
    working: plan.project(&texture)?,
    final_grid,
  })
}

fn source_geometry(
  graph: &effects::ImageEffectContainer,
  source: WordprocessingEffectRectangles,
  glow: Option<effects::EffectOutputBounds>,
) -> Option<(
  effects::EffectOutputBounds,
  effects::ImageEffectSourceBounds,
)> {
  let mut remaining = graph.clone();
  let shadow =
    if let Some(shadow) = effects::extract_projected_wordprocessing_shadow_branch(&mut remaining) {
      Some(effects::container_output_bounds_with_anchors_and_ramp(
        &shadow,
        source.paint,
        source.anchor,
        source.shadow_anchor,
        source.ramp,
      )?)
    } else {
      None
    };
  if glow.is_some() {
    effects::bind_wordprocessing_glow_mask(&mut remaining);
  }
  effects::bind_wordprocessing_reflection_paint(&mut remaining);
  let sources = effects::ImageEffectSourceBounds {
    children: shadow,
    effect_mask: glow,
    reflection_paint: Some(source.paint),
    ..Default::default()
  };
  let complete = effects::container_output_bounds_with_sources(
    &remaining,
    source.paint,
    source.anchor,
    source.shadow_anchor,
    source.ramp,
    sources,
  )?;
  Some((complete, sources))
}
