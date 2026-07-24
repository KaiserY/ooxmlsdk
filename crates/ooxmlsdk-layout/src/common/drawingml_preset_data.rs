//! Typed DrawingML preset geometry data.
//!
//! The checked-in tables are Rust implementation data. The generator uses
//! LibreOffice's preset XML as a candidate extraction source, while ECMA-376,
//! MS-OI29500, focused tests, and Microsoft Office golden output remain the
//! authority for behavior and any compatibility overrides.

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::drawingml_preset_data_generated as generated;

const _: () = {
  assert!(generated::DEFINITION_COUNT == 187);
  assert!(generated::PATH_COUNT == 320);
  assert!(generated::STROKE_ONLY_PATH_COUNT == 95);
  assert!(generated::FILL_ONLY_PATH_COUNT == 85);
};

pub(super) struct Definition {
  pub adjustments: &'static [GuideDef],
  pub guides: &'static [GuideDef],
  pub paths: &'static [PathDef],
}

pub(super) struct GuideDef {
  pub name: &'static str,
  pub formula: &'static str,
}

pub(super) struct PathDef {
  pub width: Option<&'static str>,
  pub height: Option<&'static str>,
  pub fill: Option<a::PathFillModeValues>,
  pub stroke: Option<bool>,
  pub extrusion_allowed: Option<bool>,
  pub commands: &'static [PathCommandDef],
}

pub(super) struct PointDef {
  pub x: &'static str,
  pub y: &'static str,
}

pub(super) enum PathCommandDef {
  Close,
  MoveTo(PointDef),
  LineTo(PointDef),
  ArcTo {
    width_radius: &'static str,
    height_radius: &'static str,
    start_angle: &'static str,
    sweep_angle: &'static str,
  },
  QuadraticTo(&'static [PointDef]),
  CubicTo(&'static [PointDef]),
}

pub(crate) fn geometry(preset: &a::PresetGeometry) -> a::CustomGeometry {
  let definition = generated::definition(preset.preset);
  let mut adjustments = definition
    .adjustments
    .iter()
    .map(shape_guide)
    .collect::<Vec<_>>();
  if let Some(overrides) = preset.adjust_value_list.as_ref() {
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
    ..Default::default()
  }
}

fn shape_guide(guide: &GuideDef) -> a::ShapeGuide {
  a::ShapeGuide {
    name: guide.name.into(),
    formula: guide.formula.into(),
  }
}

fn schema_path(path: &PathDef) -> a::Path {
  a::Path {
    width: path.width.map(parse_coordinate),
    height: path.height.map(parse_coordinate),
    fill: path.fill,
    stroke: path.stroke.map(Into::into),
    extrusion_ok: path.extrusion_allowed.map(Into::into),
    path_choice: path.commands.iter().map(schema_command).collect(),
  }
}

fn parse_coordinate(value: &str) -> ooxmlsdk::simple_type::PositiveCoordinateValue {
  value
    .parse()
    .unwrap_or_else(|error| panic!("invalid generated preset coordinate {value}: {error:?}"))
}

fn schema_command(command: &PathCommandDef) -> a::PathChoice {
  match command {
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
  }
}

fn schema_point(point: &PointDef) -> a::Point {
  a::Point {
    x: point.x.into(),
    y: point.y.into(),
  }
}

#[cfg(test)]
mod tests {
  use super::generated;

  #[test]
  fn generated_candidate_inventory_is_explicit() {
    assert_eq!(generated::DEFINITION_COUNT, 187);
    assert_eq!(generated::PATH_COUNT, 320);
    assert_eq!(generated::STROKE_ONLY_PATH_COUNT, 95);
    assert_eq!(generated::FILL_ONLY_PATH_COUNT, 85);
  }
}
