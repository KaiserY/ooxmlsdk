//! Preserve authored single-cubic textpaths before preset normalization.

use super::{
  VmlFormulaValue, VmlPathGeometryOptions, VmlPathToken, vml_formula_integer,
  vml_geometry_environment, vml_path_tokens,
};
use crate::common::text_warp_projection::{VmlPathSource, vml_arc_cubics};

pub(super) fn source(path: &str, options: &VmlPathGeometryOptions<'_>) -> Option<VmlPathSource> {
  let environment = vml_geometry_environment(options)?;
  // Limo stretches literals and guides by different rules. Its existing
  // geometry adapter remains the owner until integer textpath stretching is
  // established independently.
  if environment.limo.is_some() {
    return None;
  }
  let tokens = vml_path_tokens(path)?;
  let resolve = |value: VmlFormulaValue| match value {
    VmlFormulaValue::Number(value) => vml_formula_integer(value),
    VmlFormulaValue::Adjustment(index) => *environment.adjustments.get(index)?,
    VmlFormulaValue::Formula(index) => *environment.formula_values.get(index)?,
  };
  let mut index = 0;
  let mut points = Vec::new();
  while index < tokens.len() {
    let VmlPathToken::Command(command) = tokens[index] else {
      return None;
    };
    index += 1;
    let start = index;
    while index < tokens.len() && matches!(tokens[index], VmlPathToken::Value(_)) {
      index += 1;
    }
    let values = tokens[start..index]
      .iter()
      .map(|token| match token {
        VmlPathToken::Value(value) => resolve(*value),
        VmlPathToken::Command(_) => None,
      })
      .collect::<Option<Vec<_>>>()?;
    match command {
      "m" if points.is_empty() && values.len() == 2 => points.push([values[0], values[1]]),
      "c" if !points.is_empty() && !values.is_empty() && values.len().is_multiple_of(6) => {
        points.extend(values.as_chunks::<2>().0.iter().copied());
      }
      "al" | "ae" if !values.is_empty() && values.len().is_multiple_of(6) => {
        for &arc in values.as_chunks::<6>().0 {
          let curve = vml_arc_cubics(arc)?;
          if points.is_empty() {
            points.extend(curve);
          } else if command == "ae" && points.last() == curve.first() {
            points.extend(curve.into_iter().skip(1));
          } else {
            // A disconnected subpath or connecting line requires another
            // sampler segment type. Keep the existing envelope route.
            return None;
          }
        }
      }
      "e" if values.is_empty() && index == tokens.len() => break,
      "nf" | "ns" if values.is_empty() => {}
      _ => return None,
    }
  }
  if points.len() < 4 || !(points.len() - 1).is_multiple_of(3) {
    return None;
  }
  Some(VmlPathSource {
    points,
    origin: [
      environment.context.coordinate_origin_x,
      environment.context.coordinate_origin_y,
    ],
    size: [
      environment.context.coordinate_width,
      environment.context.coordinate_height,
    ],
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::{schemas::schemas_microsoft_com_vml as v, sdk::SdkType};

  #[test]
  fn native_vml_textpath_keeps_integer_formula_arc_and_rejects_disconnected_bands() {
    let formulas = v::Formulas::from_bytes(
      br#"<v:formulas xmlns:v="urn:schemas-microsoft-com:vml"><v:f eqn="val #1"/><v:f eqn="sum 0 0 #0"/><v:f eqn="val @0"/></v:formulas>"#,
    ).unwrap();
    let options = VmlPathGeometryOptions {
      coordinate_origin: None,
      coordinate_size: None,
      width_pt: 286.45,
      height_pt: 134.8,
      adjustment: Some("11796480"),
      formulas: Some(&formulas),
      limo: None,
      filled: true,
      stroked: true,
      stroke_width_pt: 1.0,
      allow_fill: true,
      allow_stroke: true,
      allow_extrusion: true,
    };
    let path = source("al10800,10800,10800,10800@1@1e", &options).unwrap();
    assert_eq!(path.origin, [0, 0]);
    assert_eq!(path.size, [21600, 21600]);
    assert_eq!(
      path.points,
      [
        [0, 10800],
        [0, 4835],
        [4835, 0],
        [10800, 0],
        [16764, 0],
        [21599, 4835],
        [21599, 10799],
      ]
    );
    assert!(source("m0,0c1,2,3,4,5,6m10,10c11,12,13,14,15,16e", &options).is_none());
    assert!(source("al10800,10800,10800,10800@0@1e", &options).is_none());
    assert!(source("al10800,10800,10800,10800@2@1e", &options).is_none());
  }
}
