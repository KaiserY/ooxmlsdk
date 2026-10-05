use ooxmlsdk_layout::common;

pub(super) fn stops_for_pdf(
  gradient: &common::GradientFill<'static>,
) -> Vec<common::GradientStop<'static>> {
  common::resolve_gradient_stops(gradient)
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
