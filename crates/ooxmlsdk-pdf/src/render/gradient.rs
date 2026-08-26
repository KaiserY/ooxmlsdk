use ooxmlsdk_layout::common;

pub(super) fn stops_for_pdf(
  gradient: &common::GradientFill<'static>,
) -> Vec<common::GradientStop<'static>> {
  if gradient.interpolation != common::GradientInterpolation::PowerPointGammaSigma
    || gradient.stops.len() < 2
  {
    return gradient.stops.clone();
  }

  // Samples of the position-independent blend factor produced by the Windows
  // GDI+ LinearGradientBrush SetSigmaBellShape(1, 1) path. PowerPoint's
  // fixed-format PDF writer combines this falloff with gamma-correct color
  // interpolation for transformed DrawingML gradients.
  const SIGMA_BLEND_U8: [u8; 33] = [
    0, 2, 5, 8, 12, 17, 22, 29, 36, 45, 54, 65, 76, 88, 101, 114, 128, 141, 154, 167, 179, 190,
    201, 210, 219, 226, 233, 238, 243, 247, 250, 253, 255,
  ];
  let mut stops = Vec::with_capacity((gradient.stops.len() - 1) * 32 + 1);
  for pair in gradient.stops.windows(2) {
    let start = &pair[0];
    let end = &pair[1];
    for (step, blend) in SIGMA_BLEND_U8[..32].iter().enumerate() {
      let position_ratio = step as f32 / 32.0;
      let blend = f32::from(*blend) / 255.0;
      stops.push(common::GradientStop {
        position: start.position + (end.position - start.position) * position_ratio,
        color: gamma_correct_color(start.color, end.color, blend),
        scheme: None,
      });
    }
  }
  stops.push(
    gradient
      .stops
      .last()
      .expect("gradient has two stops")
      .clone(),
  );
  stops
}

pub(super) fn gamma_correct_color(
  start: common::Color,
  end: common::Color,
  blend: f32,
) -> common::Color {
  let channel = |start: u8, end: u8| {
    let start = gdiplus_gamma_decode(f32::from(start) / 255.0);
    let end = gdiplus_gamma_decode(f32::from(end) / 255.0);
    (gdiplus_gamma_encode(start + (end - start) * blend) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  common::Color {
    r: channel(start.r, end.r),
    g: channel(start.g, end.g),
    b: channel(start.b, end.b),
    a: (f32::from(start.a) + (f32::from(end.a) - f32::from(start.a)) * blend)
      .round()
      .clamp(0.0, 255.0) as u8,
  }
}

fn gdiplus_gamma_decode(value: f32) -> f32 {
  value.powf(2.2)
}

fn gdiplus_gamma_encode(value: f32) -> f32 {
  value.powf(1.0 / 2.2)
}

pub(super) fn linear_line(
  bounds: common::Rect,
  angle_degrees: Option<f32>,
  scaled: bool,
) -> (common::Point, common::Point) {
  let angle = angle_degrees.unwrap_or(0.0).to_radians();
  let mut direction_x = angle.cos();
  let mut direction_y = angle.sin();
  if scaled {
    direction_x *= bounds.size.width.0;
    direction_y *= bounds.size.height.0;
  }
  let length = direction_x.hypot(direction_y).max(f32::EPSILON);
  direction_x /= length;
  direction_y /= length;
  let half_span =
    (direction_x.abs() * bounds.size.width.0 + direction_y.abs() * bounds.size.height.0) / 2.0;
  let center_x = bounds.origin.x.0 + bounds.size.width.0 / 2.0;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 / 2.0;
  (
    common::Point {
      x: common::Pt(center_x - direction_x * half_span),
      y: common::Pt(center_y - direction_y * half_span),
    },
    common::Point {
      x: common::Pt(center_x + direction_x * half_span),
      y: common::Pt(center_y + direction_y * half_span),
    },
  )
}
