use crate::common::GradientStop;

/// Normalizes DrawingML stops for PowerPoint's fixed-format gradient model.
///
/// Equal-position stops in the middle of a gradient remain a sharp color
/// change. PowerPoint handles the malformed-but-supported trailing form
/// differently for linear fills: when the final two different colors share an offset strictly
/// between 0% and 100%, the final color becomes the end stop. At 0%, Office
/// uses the last color immediately (linear and radial shapes in
/// minimal-gradient-fill-issue.pptx). LibreOffice's generic gradient
/// code detects this same trailing condition separately from ordinary sharp
/// transitions (`BColorStops::checkPenultimate`).
pub(super) fn normalize_powerpoint_gradient_stops(
  stops: &mut [GradientStop<'static>],
  linear: bool,
) {
  stops.sort_by(|left, right| left.position.total_cmp(&right.position));
  if !linear {
    return;
  }
  let [.., penultimate, last] = stops else {
    return;
  };
  if last.position > 0.0
    && last.position < 1.0
    && last.position == penultimate.position
    && last.color != penultimate.color
  {
    last.position = 1.0;
  }
}

#[cfg(test)]
mod tests {
  use crate::common::{Color, GradientStop};

  use super::normalize_powerpoint_gradient_stops;

  fn stop(position: f32, color: Color) -> GradientStop<'static> {
    GradientStop {
      position,
      color,
      scheme: None,
    }
  }

  #[test]
  fn extends_only_the_last_different_color_at_a_duplicate_trailing_offset() {
    let white = Color {
      r: 255,
      g: 255,
      b: 255,
      a: 255,
    };
    let blue = Color {
      r: 91,
      g: 155,
      b: 213,
      a: 255,
    };
    let mut stops = vec![stop(0.5, white), stop(0.5, blue)];

    normalize_powerpoint_gradient_stops(&mut stops, true);

    assert_eq!(stops[0].position, 0.5);
    assert_eq!(stops[1].position, 1.0);

    for position in [0.0, 0.5, 1.0] {
      let mut stops = vec![stop(position, white), stop(position, blue)];
      normalize_powerpoint_gradient_stops(&mut stops, false);
      assert_eq!(stops[0].position, position);
      assert_eq!(stops[1].position, position);
    }

    for position in [0.0, 1.0] {
      let mut stops = vec![stop(position, white), stop(position, blue)];
      normalize_powerpoint_gradient_stops(&mut stops, true);
      assert_eq!(stops[0].position, position);
      assert_eq!(stops[1].position, position);
    }
  }

  #[test]
  fn preserves_an_internal_duplicate_offset_as_a_sharp_transition() {
    let red = Color {
      r: 255,
      g: 0,
      b: 0,
      a: 255,
    };
    let white = Color {
      r: 255,
      g: 255,
      b: 255,
      a: 255,
    };
    let blue = Color {
      r: 91,
      g: 155,
      b: 213,
      a: 255,
    };
    let mut stops = vec![
      stop(1.0, blue),
      stop(0.5, white),
      stop(0.5, blue),
      stop(0.0, red),
    ];

    normalize_powerpoint_gradient_stops(&mut stops, true);

    assert_eq!(
      stops.iter().map(|stop| stop.position).collect::<Vec<_>>(),
      vec![0.0, 0.5, 0.5, 1.0]
    );
  }
}
