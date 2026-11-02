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

pub(crate) fn text_rectangle(
  preset: &a::PresetGeometry,
  width_emu: f64,
  height_emu: f64,
) -> Option<Rect> {
  drawingml_custom_geometry::text_rectangle(
    &drawingml_preset_data::text_geometry(preset),
    width_emu,
    height_emu,
  )
}

pub(crate) fn paths_with_word_shape_arcs(
  preset: &a::PresetGeometry,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Option<Vec<DrawingPath>> {
  let geometry = drawingml_preset_data::geometry(preset);
  drawingml_custom_geometry::paths_with_word_shape_arcs(&geometry, left, top, width, height)
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

  use super::{path_commands, paths, text_rectangle};
  use crate::common::{DrawingPathFillMode, PathCommand};

  fn preset(value: &str) -> a::PresetGeometry {
    a::PresetGeometry {
      preset: value.parse().unwrap(),
      ..Default::default()
    }
  }

  #[test]
  fn word_shape_arcs_match_native_adjusted_ellipses() {
    // Word fixed-format PDF controls: three moon adjustments, both aspect
    // ratios, 45-degree rotation, no glow or wrapping. Coordinates retain
    // native two-decimal serialization; the tolerance only accounts for it.
    let cases = [
      (
        25000,
        90.0,
        45.0,
        &[
          [(236.51, 199.78), (215.14, 164.17), (223.93, 155.38)],
          [(232.72, 146.59), (268.33, 167.96), (303.48, 203.11)],
          [(265.77, 170.53), (236.39, 159.54), (237.85, 178.58)],
          [(238.76, 190.36), (251.51, 211.61), (271.66, 234.93)],
        ][..],
      ),
      (
        25000,
        45.0,
        90.0,
        &[
          [(199.77, 239.86), (199.77, 211.36), (217.34, 193.79)],
          [(234.91, 176.22), (263.41, 176.22), (280.98, 193.79)],
          [(258.28, 181.34), (229.79, 189.65), (217.34, 212.35)],
          [(209.64, 226.39), (209.64, 243.39), (217.34, 257.43)],
        ][..],
      ),
      (
        50000,
        90.0,
        45.0,
        &[
          [(236.51, 199.78), (215.14, 164.17), (223.93, 155.38)],
          [(232.72, 146.59), (268.33, 167.96), (303.48, 203.11)],
          [(261.74, 174.55), (242.15, 179.90), (259.73, 215.04)],
          [(262.74, 221.07), (266.76, 227.77), (271.66, 234.93)],
        ][..],
      ),
      (
        50000,
        45.0,
        90.0,
        &[
          [(199.77, 239.86), (199.77, 211.36), (217.34, 193.79)],
          [(234.91, 176.22), (263.41, 176.22), (280.98, 193.79)],
          [(250.23, 189.40), (221.73, 210.77), (217.34, 241.52)],
          [(216.59, 246.80), (216.59, 252.15), (217.34, 257.43)],
        ][..],
      ),
      (
        75000,
        90.0,
        45.0,
        &[
          [(236.51, 199.78), (215.14, 164.17), (223.93, 155.38)],
          [(232.72, 146.59), (268.33, 167.96), (303.48, 203.11)],
          [(272.32, 191.84), (260.39, 203.77), (271.66, 234.93)],
        ][..],
      ),
      (
        75000,
        45.0,
        90.0,
        &[
          [(199.77, 239.86), (199.77, 211.36), (217.34, 193.79)],
          [(234.91, 176.22), (263.41, 176.22), (280.98, 193.79)],
          [(250.49, 203.07), (226.62, 226.94), (217.34, 257.43)],
        ][..],
      ),
    ];
    for (adjustment, width, height, expected) in cases {
      let mut shape = preset("moon");
      shape.adjust_value_list = Some(a::AdjustValueList {
        shape_guide: vec![a::ShapeGuide {
          name: "adj".into(),
          formula: format!("val {adjustment}"),
        }],
      });
      let paths = super::paths_with_word_shape_arcs(&shape, 0.0, 0.0, width, height)
        .expect("valid moon geometry");
      let center = kurbo::Vec2::new(f64::from(width) / 2.0, f64::from(height) / 2.0);
      let transform = kurbo::Affine::translate((210.75, 164.7))
        * kurbo::Affine::translate(center)
        * kurbo::Affine::rotate(std::f64::consts::FRAC_PI_4)
        * kurbo::Affine::translate(-center);
      let curves: Vec<_> = paths
        .iter()
        .flat_map(|path| &path.commands)
        .filter_map(|command| {
          if let PathCommand::CubicTo {
            control1,
            control2,
            end,
          } = command
          {
            Some(
              [control1, control2, end]
                .map(|p| transform * kurbo::Point::new(f64::from(p.x.0), f64::from(p.y.0))),
            )
          } else {
            None
          }
        })
        .collect();
      assert_eq!(
        curves.len(),
        expected.len(),
        "{adjustment}/{width}/{height}"
      );
      for (actual, expected) in curves.iter().zip(expected) {
        for (actual, &(x, y)) in actual.iter().zip(expected) {
          assert!(
            (actual.x - x).abs() <= 0.006 && (actual.y - y).abs() <= 0.006,
            "{adjustment}/{width}/{height}: {actual:?} != ({x}, {y})"
          );
        }
      }
    }
  }

  #[test]
  fn preset_text_rectangle_preserves_adjustments_and_rectangle_default() {
    assert_eq!(
      text_rectangle(&preset("rect"), 200.0, 300.0),
      Some(kurbo::Rect::new(0.0, 0.0, 200.0, 300.0))
    );
    let mut arrow = preset("downArrow");
    assert_eq!(
      text_rectangle(&arrow, 200.0, 300.0),
      Some(kurbo::Rect::new(50.0, 0.0, 150.0, 250.0))
    );
    arrow.adjust_value_list = Some(a::AdjustValueList {
      shape_guide: vec![a::ShapeGuide {
        name: "adj1".into(),
        formula: "val 100000".into(),
      }],
    });
    assert_eq!(
      text_rectangle(&arrow, 200.0, 300.0),
      Some(kurbo::Rect::new(0.0, 0.0, 200.0, 200.0))
    );
    assert!(text_rectangle(&arrow, 0.0, 300.0).is_none());
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
      commands.get(1),
      Some(&PathCommand::LineTo(crate::common::Point {
        x: crate::common::Pt(0.0),
        y: crate::common::Pt(0.0),
      }))
    );
  }

  #[test]
  fn zero_axis_connectors_preserve_their_stroked_paths() {
    for name in [
      "line",
      "straightConnector1",
      "bentConnector2",
      "bentConnector3",
      "curvedConnector2",
      "curvedConnector3",
    ] {
      for (width, height) in [(510.0, 0.0), (0.0, 510.0), (510.0, 1.0), (1.0, 510.0)] {
        let paths = paths(Some(&preset(name)), 96.0, 252.0, width, height)
          .expect("a zero shape axis does not erase a connector");
        assert!(
          paths
            .iter()
            .any(|path| path.stroke && path.commands.len() >= 2),
          "{name}: {width}x{height}"
        );
        let last = paths.last().unwrap().commands.last().unwrap();
        let expected = crate::common::Point {
          x: crate::common::Pt(96.0 + width),
          y: crate::common::Pt(252.0 + height),
        };
        let endpoint = match last {
          PathCommand::LineTo(end) | PathCommand::CubicTo { end, .. } => *end,
          _ => panic!("unexpected connector endpoint {last:?}"),
        };
        assert_eq!(endpoint, expected, "{name}: {width}x{height}");
      }
    }
  }

  #[test]
  fn zero_radius_preset_arc_keeps_the_surrounding_shape_path() {
    let mut preset = preset("snipRoundRect");
    preset.adjust_value_list = Some(a::AdjustValueList {
      shape_guide: vec![
        a::ShapeGuide {
          name: "adj1".into(),
          formula: "val 0".into(),
        },
        a::ShapeGuide {
          name: "adj2".into(),
          formula: "val 50000".into(),
        },
      ],
    });

    let paths = paths(Some(&preset), 0.0, 0.0, 100.0, 20.0)
      .expect("a legal zero-radius corner must not discard the preset shape");
    assert_eq!(paths.len(), 1);
    assert!(
      paths[0]
        .commands
        .iter()
        .any(|command| matches!(command, PathCommand::Close))
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
