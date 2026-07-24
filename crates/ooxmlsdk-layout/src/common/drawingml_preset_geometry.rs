//! DrawingML preset-shape geometry.
//!
//! The checked-in Rust tables cover all 187 schema values. LibreOffice's
//! `presetShapeDefinitions.xml` is the extraction reference; ECMA-376,
//! MS-OI29500, and Microsoft Office golden output govern reviewed differences.
//! Preset and authored custom geometry deliberately share one guide evaluator
//! and Kurbo lowering path.

use kurbo::{Rect, Shape};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::DrawingPath;
#[cfg(test)]
use crate::common::{DrawingPathFillMode, PathCommand};

use super::{
  drawingml_custom_geometry, drawingml_geometry::bez_path_commands, drawingml_preset_data,
};

pub(crate) fn paths(
  preset: Option<&a::PresetGeometry>,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Option<Vec<DrawingPath>> {
  let Some(preset) = preset else {
    return Some(vec![rectangle_path(left, top, width, height)]);
  };
  let geometry = drawingml_preset_data::geometry(preset);
  drawingml_custom_geometry::paths(&geometry, left, top, width, height)
}

/// Compatibility adapter for hosts that have not migrated to per-path paint.
///
/// A missing preset is the DrawingML rectangle default. Invalid preset data
/// returns no commands rather than silently substituting a rectangle.
#[cfg(test)]
fn path_commands(
  preset: Option<&a::PresetGeometry>,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Vec<PathCommand> {
  paths(preset, left, top, width, height)
    .unwrap_or_default()
    .into_iter()
    .filter(|path| path.fill_mode != DrawingPathFillMode::None)
    .flat_map(|path| path.commands)
    .collect()
}

fn rectangle_path(left: f32, top: f32, width: f32, height: f32) -> DrawingPath {
  DrawingPath {
    commands: bez_path_commands(
      Rect::new(
        f64::from(left),
        f64::from(top),
        f64::from(left + width),
        f64::from(top + height),
      )
      .to_path(0.1),
    ),
    ..DrawingPath::default()
  }
}

#[cfg(test)]
mod tests {
  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

  use super::{path_commands, paths};
  use crate::common::{DrawingPathFillMode, PathCommand};

  fn preset(value: &str) -> a::PresetGeometry {
    a::PresetGeometry {
      preset: value.parse().unwrap(),
      ..Default::default()
    }
  }

  #[test]
  fn lowers_source_backed_multi_path_preset_with_independent_paint() {
    let paths = paths(Some(&preset("cube")), 10.0, 20.0, 100.0, 200.0).unwrap();
    assert!(paths.len() > 1);
    assert!(paths.iter().any(|path| !path.stroke));
    assert!(
      paths
        .iter()
        .any(|path| path.fill_mode != DrawingPathFillMode::Normal)
    );
    assert!(
      paths
        .iter()
        .all(|path| !path.commands.is_empty() && path.commands[0].is_move_to())
    );
  }

  #[test]
  fn applies_authored_adjustment_over_definition_default() {
    let mut preset = preset("triangle");
    preset.adjust_value_list = Some(a::AdjustValueList {
      shape_guide: vec![a::ShapeGuide {
        name: "adj".into(),
        formula: "val 0".into(),
      }],
    });
    let commands = path_commands(Some(&preset), 0.0, 0.0, 100.0, 100.0);
    assert_eq!(
      commands.first(),
      Some(&PathCommand::MoveTo(crate::common::Point {
        x: crate::common::Pt(0.0),
        y: crate::common::Pt(0.0),
      }))
    );
  }

  trait PathCommandExt {
    fn is_move_to(&self) -> bool;
  }

  impl PathCommandExt for PathCommand {
    fn is_move_to(&self) -> bool {
      matches!(self, Self::MoveTo(_))
    }
  }
}
