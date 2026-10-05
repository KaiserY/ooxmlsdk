//! Native Office AutoShape faces absent from the serialized VML outline.
//!
//! The legacy Ribbon2 (spt54) and HorizontalScroll (spt98) definitions serialize
//! their face boundaries as `nf` paths. Office supplies separate shaded faces
//! for the recognized AutoShape. Keep the authored outline and recognize the
//! definition before adding faces: a custom path can retain its original spt.
//! Legacy shape identities are documented by MS-ODRAW/POI ShapeType, and their
//! face profiles by DrawingML presets and EnhancedCustomShape2d color data.

use ooxmlsdk::schemas::{a, v};

use super::{
  InlineShape, InlineShapeGeometry, vml_adjustment_values, vml_coordinate_pair,
  vml_path_coordinate_pair, vml_path_tokens, vml_shape_adjustments, vml_shape_formulas,
  vml_shape_path, vml_shapetype_formulas, vml_shapetype_path,
};
use crate::common::{self, DrawingPathFillMode};

const SCROLL_PATH: &str = "m0@5qy@2@1l@0@1@0@2qy@7,,21600@2l21600@9qy@7@10l@1@10@1@11qy@2,21600,0@11xem0@5nfqy@2@6@1@5@3@4@2@5l@2@6em@1@5nfl@1@10em21600@2nfqy@7@1l@0@1em@0@2nfqy@8@3@7@2l@7@1e";
const SCROLL_GUIDES: &[&str] = &[
  "sum width 0 #0",
  "val #0",
  "prod @1 1 2",
  "prod @1 3 4",
  "prod @1 5 4",
  "prod @1 3 2",
  "prod @1 2 1",
  "sum width 0 @2",
  "sum width 0 @3",
  "sum height 0 @5",
  "sum height 0 @1",
  "sum height 0 @2",
  "val width",
  "prod width 1 2",
  "prod height 1 2",
];
const RIBBON_PATH: &str = "m0@29l@3@29qx@4@19l@4@10@5@10@5@19qy@6@29l@28@29@26@22@28@23@9@23@9@24qy@8,l@1,qx@0@24l@0@23,0@23,2700@22xem@4@19nfqy@3@20l@1@20qx@0@21@1@10l@4@10em@5@19nfqy@6@20l@8@20qx@9@21@8@10l@5@10em@0@21nfl@0@23em@9@21nfl@9@23e";
const RIBBON_GUIDES: &[&str] = &[
  "val #0",
  "sum @0 675 0",
  "sum @1 675 0",
  "sum @2 675 0",
  "sum @3 675 0",
  "sum width 0 @4",
  "sum width 0 @3",
  "sum width 0 @2",
  "sum width 0 @1",
  "sum width 0 @0",
  "val #1",
  "prod @10 1 4",
  "prod @10 1 2",
  "prod @10 3 4",
  "prod height 3 4",
  "prod height 1 2",
  "prod height 1 4",
  "prod height 3 2",
  "prod height 2 3",
  "sum @11 @14 0",
  "sum @12 @15 0",
  "sum @13 @16 0",
  "sum @17 0 @20",
  "sum height 0 @10",
  "sum height 0 @19",
  "prod width 1 2",
  "sum width 0 2700",
  "sum @25 0 2700",
  "val width",
  "val height",
];

fn preset(shape: &v::Shape, shape_type: &v::Shapetype, path: &str) -> Option<a::PresetGeometry> {
  let number = shape_type.optional_number.or_else(|| {
    shape_type
      .id
      .as_deref()?
      .strip_prefix("_x0000_t")?
      .parse()
      .ok()
  })?;
  let (kind, expected_path, expected_guides) = match number {
    54 => (a::ShapeTypeValues::Ribbon2, RIBBON_PATH, RIBBON_GUIDES),
    98 => (
      a::ShapeTypeValues::HorizontalScroll,
      SCROLL_PATH,
      SCROLL_GUIDES,
    ),
    _ => return None,
  };
  if vml_path_tokens(path)? != vml_path_tokens(expected_path)? {
    return None;
  }
  let size = vml_coordinate_pair(
    shape
      .coordinate_size
      .as_deref()
      .or(shape_type.coordinate_size.as_deref()),
  );
  let origin = vml_coordinate_pair(
    shape
      .coordinate_origin
      .as_deref()
      .or(shape_type.coordinate_origin.as_deref()),
  );
  if size.unwrap_or((21600.0, 21600.0)) != (21600.0, 21600.0)
    || origin.unwrap_or((0.0, 0.0)) != (0.0, 0.0)
  {
    return None;
  }
  let formulas = vml_shape_formulas(shape).or_else(|| vml_shapetype_formulas(shape_type))?;
  if formulas.formula.len() != expected_guides.len()
    || !formulas
      .formula
      .iter()
      .zip(expected_guides)
      .all(|(formula, expected)| {
        formula.equation.as_deref().is_some_and(|equation| {
          equation
            .split_ascii_whitespace()
            .eq(expected.split_ascii_whitespace())
        })
      })
  {
    return None;
  }
  let path_properties = vml_shape_path(shape).or_else(|| vml_shapetype_path(shape_type));
  if path_properties
    .and_then(|path| path.allow_fill)
    .is_some_and(|allow| !allow.as_bool())
  {
    return None;
  }
  let limo = path_properties
    .and_then(|path| path.limo.as_deref())
    .and_then(vml_path_coordinate_pair);
  // Removing the native scroll's limo changes it into a custom shape, even
  // though its spt and serialized path are retained. It then has no faces.
  if (number == 98 && limo != Some((10800.0, 10800.0))) || (number == 54 && limo.is_some()) {
    return None;
  }
  let adjustment = vml_shape_adjustments(shape, Some(shape_type));
  let values = vml_adjustment_values(adjustment.as_deref())?;
  let first = *values.first()?.as_ref()?;
  let guide = |name: &str, value: f64| a::ShapeGuide {
    name: name.into(),
    formula: format!("val {value}"),
  };
  let guides = if number == 98 {
    if !(0..=5400).contains(&first) {
      return None;
    }
    vec![guide("adj", f64::from(first) * 100000.0 / 21600.0)]
  } else {
    let second = *values.get(1)?.as_ref()?;
    if !(2700..=8100).contains(&first) || !(14400..=21600).contains(&second) {
      return None;
    }
    vec![
      guide("adj1", (21600.0 - f64::from(second)) * 100000.0 / 21600.0),
      guide(
        "adj2",
        (21600.0 - 2.0 * f64::from(first)) * 100000.0 / 21600.0,
      ),
    ]
  };
  Some(a::PresetGeometry {
    preset: kind,
    adjust_value_list: Some(a::AdjustValueList {
      shape_guide: guides,
    }),
    ..Default::default()
  })
}

pub(super) fn restore_fold_faces(
  inline: &mut InlineShape,
  shape: &v::Shape,
  shape_type: Option<&v::Shapetype>,
  path: &str,
  filled: bool,
) {
  if !filled
    || inline.fill_image.is_some()
    || inline.fill_pattern.is_some()
    || matches!(
      inline.fill_override.as_deref(),
      Some(common::Fill::Pattern(_))
    )
  {
    return;
  }
  let Some(preset) = shape_type.and_then(|shape_type| preset(shape, shape_type, path)) else {
    return;
  };
  let Some(faces) = common::drawingml_preset_geometry::paths(
    Some(&preset),
    0.0,
    0.0,
    inline.width_pt,
    inline.height_pt,
  ) else {
    return;
  };
  let InlineShapeGeometry::Path { paths, .. } = &mut inline.geometry else {
    return;
  };
  let mut strokes = Vec::new();
  for path in paths.iter_mut() {
    if path.stroke {
      let mut stroke = path.clone();
      stroke.fill_mode = DrawingPathFillMode::None;
      strokes.push(stroke);
      path.stroke = false;
    }
  }
  paths.retain(|path| path.fill_mode != DrawingPathFillMode::None);
  // Paint the recognized faces over the primary fill and beneath every
  // authored outline. Never replace the serialized outer path or its guides.
  paths.extend(
    faces
      .into_iter()
      .filter(|path| path.fill_mode == DrawingPathFillMode::DarkenLess),
  );
  paths.extend(strokes);
  inline.vml_autoshape_shaded_faces = true;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{ImageCatalog, vml_shape_shape};
  use ooxmlsdk::sdk::SdkType;

  fn shape_type(number: i32) -> v::Shapetype {
    let (path, guides, adjustment, limo) = if number == 98 {
      (SCROLL_PATH, SCROLL_GUIDES, "2700", " limo=\"10800,10800\"")
    } else {
      (RIBBON_PATH, RIBBON_GUIDES, "5400,18900", "")
    };
    let formulas = guides
      .iter()
      .map(|equation| format!("<v:f eqn=\"{equation}\"/>"))
      .collect::<String>();
    v::Shapetype::from_bytes(format!("<v:shapetype xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\" id=\"_x0000_t{number}\" coordsize=\"21600,21600\" o:spt=\"{number}\" adj=\"{adjustment}\" path=\"{path}\"><v:formulas>{formulas}</v:formulas><v:path{limo}/></v:shapetype>").as_bytes()).unwrap()
  }
  fn shape(number: i32, width: f32, height: f32) -> v::Shape {
    let adjustment = if number == 54 { " adj=\"3366\"" } else { "" };
    v::Shape::from_bytes(format!("<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\" id=\"native-shape\" type=\"#_x0000_t{number}\" style=\"width:{width}pt;height:{height}pt\" fillcolor=\"white\"{adjustment}/>").as_bytes()).unwrap()
  }

  #[test]
  fn recognized_legacy_folds_keep_the_native_outline_and_paint_order() {
    for (number, width, height) in [(98, 136.5, 115.4), (54, 502.0, 97.35)] {
      let source = shape(number, width, height);
      let definition = shape_type(number);
      let inline = vml_shape_shape(&source, &ImageCatalog::default(), &[&definition]).unwrap();
      assert!(inline.vml_autoshape_shaded_faces);
      let InlineShapeGeometry::Path { paths, .. } = inline.geometry else {
        panic!("VML paths")
      };
      assert_eq!(paths.len(), 7);
      assert_eq!(paths[0].fill_mode, DrawingPathFillMode::Normal);
      assert_eq!(paths[1].fill_mode, DrawingPathFillMode::DarkenLess);
      assert!(!paths[0].stroke && !paths[1].stroke);
      assert!(
        paths[2..]
          .iter()
          .all(|path| path.fill_mode == DrawingPathFillMode::None && path.stroke)
      );
      assert_eq!(paths[0].commands, paths[2].commands);
      // Native PDF fold bounds relative to the AutoShape's host origin.
      let mut bounds = Vec::new();
      let mut points = Vec::new();
      let finish = |points: &mut Vec<(f32, f32)>, bounds: &mut Vec<[f32; 4]>| {
        if points.is_empty() {
          return;
        }
        bounds.push([
          points.iter().map(|p| p.0).fold(f32::INFINITY, f32::min),
          points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min),
          points.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max),
          points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max),
        ]);
        points.clear();
      };
      for command in &paths[1].commands {
        match *command {
          common::PathCommand::MoveTo(p) => {
            finish(&mut points, &mut bounds);
            points.push((p.x.0, p.y.0));
          }
          common::PathCommand::LineTo(p) => points.push((p.x.0, p.y.0)),
          common::PathCommand::CubicTo {
            control1,
            control2,
            end,
          } => points.extend([control1, control2, end].map(|p| (p.x.0, p.y.0))),
          common::PathCommand::Close => {}
        }
      }
      finish(&mut points, &mut bounds);
      let expected = if number == 98 {
        [
          [7.2125, 18.03125, 14.425, 28.85],
          [122.075, 0.0, 136.5, 14.425],
        ]
      } else {
        [
          [78.22611, 85.18125, 140.9761, 94.307816],
          [361.0239, 85.18125, 423.7739, 94.307816],
        ]
      };
      assert_eq!(bounds.len(), 2);
      for (actual, expected) in bounds.into_iter().zip(expected) {
        for (actual, expected) in actual.into_iter().zip(expected) {
          assert!(
            (actual - expected).abs() < 0.02,
            "spt{number}: {actual} != {expected}"
          );
        }
      }
    }
  }

  #[test]
  fn legacy_fold_recognition_respects_native_custom_path_controls() {
    for number in [54, 98] {
      for variant in [
        "original",
        "no-spt",
        "unknown-id",
        "zero-spt",
        "inherited-rectangle",
        "direct-rectangle",
        "no-limo",
        "fill-disabled",
      ] {
        if number == 54 && variant == "no-limo" {
          continue;
        }
        let mut definition = shape_type(number);
        let mut source = shape(number, 120.0, 60.0);
        match variant {
          "no-spt" => definition.optional_number = None,
          "unknown-id" => {
            definition.id = Some("custom-type".into());
            source.r#type = Some("#custom-type".into());
          }
          "zero-spt" => {
            definition.optional_number = Some(0);
            definition.id = Some("custom-type".into());
            source.r#type = Some("#custom-type".into());
          }
          "inherited-rectangle" => {
            definition.edge_path = Some("m0,0l21600,0,21600,21600,0,21600xe".into())
          }
          "direct-rectangle" => {
            source.edge_path = Some("m0,0l21600,0,21600,21600,0,21600xe".into())
          }
          "no-limo" => {
            for choice in &mut definition.shapetype_choice {
              if let v::ShapetypeChoice::Path(path) = choice {
                path.limo = None;
              }
            }
          }
          "fill-disabled" => source.filled = Some(false.into()),
          _ => {}
        }
        let inline = vml_shape_shape(&source, &ImageCatalog::default(), &[&definition]).unwrap();
        assert_eq!(
          inline.vml_autoshape_shaded_faces,
          matches!(variant, "original" | "no-spt" | "unknown-id"),
          "spt{number}: {variant}"
        );
        if variant == "direct-rectangle" || variant == "inherited-rectangle" {
          let InlineShapeGeometry::Path { paths, .. } = inline.geometry else {
            panic!("authored rectangle")
          };
          assert_eq!(paths.len(), 1);
          assert_eq!(paths[0].commands.len(), 5);
        }
      }
    }
  }
}
