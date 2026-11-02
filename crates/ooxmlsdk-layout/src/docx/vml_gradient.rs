//! Word legacy linear brushes: transformed color planes and native sigma
//! samples. DrawingML and radial-brush policies remain separate.
use super::common;
use crate::common::drawingml_gradient;

pub(super) fn transformed_line(
  angle: Option<f32>,
  transform: kurbo::Affine,
) -> Option<(common::Point, common::Point)> {
  if !transform.determinant().is_finite() || transform.determinant().abs() < f64::EPSILON {
    return None;
  }
  let (sin, cos) = f64::from(angle.unwrap_or_default()).to_radians().sin_cos();
  let [a, b, c, d, _, _] = transform.inverse().as_coeffs();
  // A scalar linear color field transforms by the inverse transpose.
  let normal = kurbo::Vec2::new(a * cos + b * sin, c * cos + d * sin);
  let magnitude = normal.hypot();
  if magnitude <= f64::EPSILON {
    return None;
  }
  let direction = normal / magnitude;
  let [a, b, c, d, _, _] = transform.as_coeffs();
  let span = (direction.dot(kurbo::Vec2::new(a, b)).abs()
    + direction.dot(kurbo::Vec2::new(c, d)).abs())
    * 0.5;
  let center = transform * kurbo::Point::new(0.5, 0.5);
  let point = |p: kurbo::Point| common::Point {
    x: common::Pt(p.x as f32),
    y: common::Pt(p.y as f32),
  };
  Some((
    point(center - direction * span),
    point(center + direction * span),
  ))
}

// GDI+ SetSigmaBellShape(1,1), 256 positions i/255. Retain the full
// floating-point blend before gamma correction; byte-quantizing the blend
// changes dark channels. Symmetric half-table of native API observations.
// The normalized Gaussian CDF over +/-2 sigma is an independent check
// (Wine gdiplus/brush.c); no source implementation is copied.
const SIGMA_HALF: [f32; 128] = [
  0.000000000,
  0.000900269,
  0.001831055,
  0.002_777_1,
  0.003768921,
  0.004_791_26,
  0.005844116,
  0.006_927_49,
  0.008041382,
  0.009185791,
  0.010375977,
  0.011581421,
  0.012832642,
  0.014129639,
  0.015457153,
  0.016815186,
  0.018218994,
  0.019_653_32,
  0.021133423,
  0.022659302,
  0.024215698,
  0.025817871,
  0.027_465_82,
  0.029159546,
  0.030883789,
  0.032669067,
  0.034484863,
  0.036361694,
  0.038269043,
  0.040237427,
  0.042251587,
  0.044311523,
  0.046417236,
  0.048568726,
  0.050_781_25,
  0.053_054_81,
  0.055358887,
  0.057_724,
  0.060150146,
  0.062_622_07,
  0.065_155_03,
  0.067733765,
  0.070373535,
  0.073_074_34,
  0.075_820_92,
  0.078_628_54,
  0.081_497_19,
  0.084_426_88,
  0.087402344,
  0.090_454_1,
  0.093551636,
  0.096710205,
  0.099_929_81,
  0.103_210_45,
  0.106552124,
  0.109954834,
  0.113_433_84,
  0.116_958_62,
  0.120_544_43,
  0.124191284,
  0.127_899_17,
  0.131_683_35,
  0.135_513_3,
  0.139_404_3,
  0.143_371_58,
  0.147_399_9,
  0.151_474,
  0.155_624_39,
  0.159_835_82,
  0.164_108_28,
  0.168_441_77,
  0.172_836_3,
  0.177_291_87,
  0.181_823_73,
  0.186_401_37,
  0.191_040_04,
  0.195_739_75,
  0.200_500_49,
  0.205_322_27,
  0.210_205_08,
  0.215_133_67,
  0.220_138_55,
  0.225_189_21,
  0.230_300_9,
  0.235_458_37,
  0.240_692_14,
  0.245_956_42,
  0.251_297,
  0.256_683_35,
  0.262_115_48,
  0.267_608_64,
  0.273_147_58,
  0.278_732_3,
  0.284_378_05,
  0.290_054_32,
  0.295_791_63,
  0.301_574_7,
  0.307_403_56,
  0.313_262_94,
  0.319_183_35,
  0.325_134_28,
  0.331_130_98,
  0.337_158_2,
  0.343_231_2,
  0.349_334_72,
  0.355_484,
  0.361_663_82,
  0.367_874_15,
  0.374_115,
  0.380_386_35,
  0.386_688_23,
  0.393_005_37,
  0.399_368_3,
  0.405_746_46,
  0.412_139_9,
  0.418_563_84,
  0.425_003_05,
  0.431_457_52,
  0.437_942_5,
  0.444_427_5,
  0.450_927_73,
  0.457_443_24,
  0.463_974,
  0.470_520_02,
  0.477_050_78,
  0.483_612_06,
  0.490_158_08,
  0.496_719_36,
];

pub(super) fn resolve_sigma_profile(gradient: &mut common::GradientFill<'static>) {
  let gamma = match gradient.interpolation {
    common::GradientInterpolation::PowerPointGammaSigma => true,
    common::GradientInterpolation::SigmaSrgb => false,
    _ => return,
  };
  if gradient.stops.len() < 2 {
    return;
  }
  let mut resolved = Vec::with_capacity((gradient.stops.len() - 1) * 255 + 1);
  for pair in gradient.stops.windows(2) {
    for step in 0..255 {
      let blend = if step < 128 {
        SIGMA_HALF[step]
      } else {
        1.0 - SIGMA_HALF[255 - step]
      };
      let color = if gamma {
        drawingml_gradient::gamma_correct_color(pair[0].color, pair[1].color, blend)
      } else {
        drawingml_gradient::sample(
          &[
            common::GradientStop {
              position: 0.0,
              color: pair[0].color,
              scheme: None,
            },
            common::GradientStop {
              position: 1.0,
              color: pair[1].color,
              scheme: None,
            },
          ],
          blend,
        )
      };
      resolved.push(common::GradientStop {
        position: pair[0].position + (pair[1].position - pair[0].position) * step as f32 / 255.0,
        color,
        scheme: None,
      });
    }
  }
  resolved.push(gradient.stops.last().unwrap().clone());
  gradient.stops = resolved;
  gradient.interpolation = common::GradientInterpolation::LinearSrgb;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn vml_sigma_keeps_precision_until_final_channels() {
    let red = common::Color {
      r: 255,
      g: 0,
      b: 0,
      a: 255,
    };
    let green = common::Color {
      r: 0,
      g: 128,
      b: 0,
      a: 255,
    };
    let mut gradient = common::GradientFill {
      stops: vec![
        common::GradientStop {
          position: 0.0,
          color: red,
          scheme: None,
        },
        common::GradientStop {
          position: 1.0,
          color: green,
          scheme: None,
        },
      ],
      interpolation: common::GradientInterpolation::PowerPointGammaSigma,
      ..common::GradientFill::default()
    };
    resolve_sigma_profile(&mut gradient);
    assert_eq!(gradient.stops.len(), 256);
    for (index, rgb) in [
      (0, [255, 0, 0]),
      (63, [238, 52, 0]),
      (127, [187, 93, 0]),
      (128, [186, 94, 0]),
      (255, [0, 128, 0]),
    ] {
      let color = gradient.stops[index].color;
      for (actual, expected) in [color.r, color.g, color.b].into_iter().zip(rgb) {
        assert!(actual.abs_diff(expected) <= 1);
      }
    }
    assert_eq!(
      gradient.interpolation,
      common::GradientInterpolation::LinearSrgb
    );
  }

  #[test]
  fn vml_linear_color_planes_follow_nonsquare_hosts() {
    for (width, height, angle, expected) in [
      (204.0, 155.8, 60.0, [148.139, 30.526, 238.861, 236.274]),
      (204.0, 155.8, 45.0, [118.347, 35.003, 268.653, 231.803]),
      (408.0, 155.8, 60.0, [269.671, 16.219, 321.329, 250.581]),
      (123.0, 92.28, 45.0, [108.702, 42.595, 197.298, 160.685]),
    ] {
      let transform =
        kurbo::Affine::scale_non_uniform(width, height).then_translate((91.5, 55.5).into());
      let (a, b) = transformed_line(Some(angle), transform).unwrap();
      for (actual, expected) in [a.x.0, a.y.0, b.x.0, b.y.0].into_iter().zip(expected) {
        assert!(
          (actual - expected).abs() < 0.02,
          "{width}x{height}: {actual} != {expected}"
        );
      }
    }
  }
}
