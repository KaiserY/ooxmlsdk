use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LineProperties {
  pub(crate) fill: LineFill,
  pub(crate) width_emu: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LineFill {
  Unspecified,
  None,
  Solid(Option<Color>),
  Gradient(std::boxed::Box<a::GradientFill>),
  Pattern(std::boxed::Box<a::PatternFill>),
}
