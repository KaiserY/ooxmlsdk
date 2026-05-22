use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FillProperties {
  pub(crate) kind: FillKind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FillKind {
  None,
  Solid(Option<Color>),
  Group,
  Gradient(std::boxed::Box<a::GradientFill>),
  Blip(std::boxed::Box<a::BlipFill>),
  Pattern(std::boxed::Box<a::PatternFill>),
}
