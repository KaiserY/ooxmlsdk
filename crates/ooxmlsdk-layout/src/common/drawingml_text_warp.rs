//! Source-backed DrawingML preset text-warp envelopes.
//!
//! Each preset defines one or more paths. A single path is a text centerline;
//! multiple paths form an ordered vertical deformation grid and are
//! interpolated piecewise. The checked-in data is generated from LibreOffice's
//! reviewed preset definitions; ECMA-376, MS-OI29500, and Office fixed output
//! remain the compatibility authority.

use std::sync::Arc;

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::model::{PageItem, RgbColor};
use crate::text_metrics::TextMetrics;

use super::{
  DrawingPath, PdfGlyphOutlineOptions, Rect, TextWarp, drawingml_custom_geometry,
  drawingml_preset_data::Definition, drawingml_text_warp_data_generated as generated,
};

const _: () = {
  assert!(generated::DEFINITION_COUNT == 40);
  assert!(generated::PATH_COUNT == 88);
};

pub(crate) fn paths(
  preset: &a::PresetTextWarp,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Option<Vec<DrawingPath>> {
  let definition = generated::definition(preset.preset)?;
  let geometry = geometry(definition, preset.adjust_value_list.as_ref());
  drawingml_custom_geometry::paths(&geometry, left, top, width, height)
}

pub(crate) fn apply_to_text_items(
  items: &mut [PageItem],
  preset: &a::PresetTextWarp,
  target: Rect,
  text_metrics: &mut TextMetrics,
  color: Option<RgbColor>,
) -> bool {
  let Some((left, top, right, bottom)) = text_items_ink_bounds(items, text_metrics) else {
    return false;
  };
  let source_width = right - left;
  let source_height = bottom - top;
  if source_width <= f32::EPSILON || source_height <= f32::EPSILON {
    return false;
  }
  let source_bounds = Rect {
    origin: super::Point {
      x: super::Pt(left),
      y: super::Pt(top),
    },
    size: super::Size {
      width: super::Pt(source_width),
      height: super::Pt(source_height),
    },
  };
  let Some(text_warp) = text_warp(preset, source_bounds, target) else {
    return false;
  };
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    if let Some(color) = color {
      text.style.color = color;
    }
    text.style.pdf_glyph_outlines = true;
    let mut options = text
      .style
      .pdf_glyph_outline_options
      .as_deref()
      .cloned()
      .unwrap_or_else(PdfGlyphOutlineOptions::default);
    // Preserve the host decision: ordinary Office WordArt retains an
    // invisible text layer, while scene3d WordArt is outline-only.
    options.transform = None;
    options.text_warp = Some(text_warp.clone());
    text.style.pdf_glyph_outline_options = Some(Arc::new(options));
  }
  true
}

pub(crate) fn text_warp(
  preset: &a::PresetTextWarp,
  source_bounds: Rect,
  target: Rect,
) -> Option<Arc<TextWarp>> {
  if source_bounds.size.width.0 <= f32::EPSILON || source_bounds.size.height.0 <= f32::EPSILON {
    return None;
  }
  let paths = paths(
    preset,
    target.origin.x.0,
    target.origin.y.0,
    target.size.width.0,
    target.size.height.0,
  )?;
  let boundaries = paths
    .into_iter()
    .map(|path| path.commands)
    .filter(|commands| !commands.is_empty())
    .collect::<Vec<_>>();
  if boundaries.is_empty() {
    return None;
  }
  Some(Arc::new(TextWarp {
    source_bounds,
    boundaries,
  }))
}

fn text_items_ink_bounds(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    let baseline_offset = if text.style.use_windows_font_metrics {
      text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
        &text.text,
        &text.style,
        text.line_height_pt,
      )
    } else {
      text_metrics.baseline_offset_in_line_for_text(&text.text, &text.style, text.line_height_pt)
    };
    let baseline = text.y_pt + baseline_offset;
    let mut glyph_x = text.x_pt;
    let Some(shaped) = text_metrics.shape_text(&text.text, &text.style) else {
      continue;
    };
    for glyph in shaped.glyphs {
      let font_size = glyph.font_size_pt;
      if let Some(glyph_bounds) = glyph.bounds_em {
        let glyph_left = glyph_x + (glyph.x_offset_em + glyph_bounds.x_min_em) * font_size;
        let glyph_right = glyph_x + (glyph.x_offset_em + glyph_bounds.x_max_em) * font_size;
        let glyph_top = baseline - (glyph.y_offset_em + glyph_bounds.y_max_em) * font_size;
        let glyph_bottom = baseline - (glyph.y_offset_em + glyph_bounds.y_min_em) * font_size;
        bounds = Some(match bounds {
          Some((old_left, old_top, old_right, old_bottom)) => (
            old_left.min(glyph_left),
            old_top.min(glyph_top),
            old_right.max(glyph_right),
            old_bottom.max(glyph_bottom),
          ),
          None => (glyph_left, glyph_top, glyph_right, glyph_bottom),
        });
      }
      glyph_x += glyph.x_advance_em * font_size;
    }
  }
  bounds
}

fn geometry(definition: &Definition, overrides: Option<&a::AdjustValueList>) -> a::CustomGeometry {
  let mut adjustments = definition
    .adjustments
    .iter()
    .map(shape_guide)
    .collect::<Vec<_>>();
  if let Some(overrides) = overrides {
    for adjustment in &overrides.shape_guide {
      if let Some(default) = adjustments
        .iter_mut()
        .find(|default| default.name == adjustment.name)
      {
        default.formula.clone_from(&adjustment.formula);
      } else {
        adjustments.push(adjustment.clone());
      }
    }
  }
  a::CustomGeometry {
    adjust_value_list: Some(a::AdjustValueList {
      shape_guide: adjustments,
    }),
    shape_guide_list: Some(a::ShapeGuideList {
      shape_guide: definition.guides.iter().map(shape_guide).collect(),
    }),
    path_list: a::PathList {
      path: definition.paths.iter().map(schema_path).collect(),
    },
    ..a::CustomGeometry::default()
  }
}

fn shape_guide(guide: &super::drawingml_preset_data::GuideDef) -> a::ShapeGuide {
  a::ShapeGuide {
    name: guide.name.into(),
    formula: guide.formula.into(),
  }
}

fn schema_path(path: &super::drawingml_preset_data::PathDef) -> a::Path {
  use super::drawingml_preset_data::PathCommandDef;

  let command = |command: &PathCommandDef| match command {
    PathCommandDef::Close => a::PathChoice::CloseShapePath,
    PathCommandDef::MoveTo(point) => a::PathChoice::MoveTo(Box::new(a::MoveTo {
      point: schema_point(point),
    })),
    PathCommandDef::LineTo(point) => a::PathChoice::LineTo(Box::new(a::LineTo {
      point: schema_point(point),
    })),
    PathCommandDef::ArcTo {
      width_radius,
      height_radius,
      start_angle,
      sweep_angle,
    } => a::PathChoice::ArcTo(Box::new(a::ArcTo {
      width_radius: (*width_radius).into(),
      height_radius: (*height_radius).into(),
      start_angle: (*start_angle).into(),
      swing_angle: (*sweep_angle).into(),
    })),
    PathCommandDef::QuadraticTo(points) => {
      a::PathChoice::QuadraticBezierCurveTo(a::QuadraticBezierCurveTo {
        point: points.iter().map(schema_point).collect(),
      })
    }
    PathCommandDef::CubicTo(points) => a::PathChoice::CubicBezierCurveTo(a::CubicBezierCurveTo {
      point: points.iter().map(schema_point).collect(),
    }),
  };
  a::Path {
    width: path
      .width
      .map(|value| value.parse().expect("generated width")),
    height: path
      .height
      .map(|value| value.parse().expect("generated height")),
    fill: path.fill,
    stroke: path.stroke.map(Into::into),
    extrusion_ok: path.extrusion_allowed.map(Into::into),
    path_choice: path.commands.iter().map(command).collect(),
  }
}

fn schema_point(point: &super::drawingml_preset_data::PointDef) -> a::Point {
  a::Point {
    x: point.x.into(),
    y: point.y.into(),
  }
}

#[cfg(test)]
mod tests {
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

  use super::{generated, paths};

  #[test]
  fn generated_text_warp_inventory_is_complete() {
    assert_eq!(generated::DEFINITION_COUNT, 40);
    assert_eq!(generated::PATH_COUNT, 88);
  }

  #[test]
  fn every_warp_preset_has_source_backed_boundary_paths() {
    for preset in [
      "textPlain",
      "textArchUp",
      "textCirclePour",
      "textDoubleWave1",
      "textCascadeDown",
    ] {
      let preset = a::PresetTextWarp {
        preset: preset.parse().unwrap(),
        ..a::PresetTextWarp::default()
      };
      assert!(paths(&preset, 10.0, 20.0, 200.0, 100.0).is_some());
    }
  }
}
