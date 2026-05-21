use super::color::Color;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LineProperties {
  pub(crate) color: Option<Color>,
  pub(crate) width_emu: Option<i64>,
}
