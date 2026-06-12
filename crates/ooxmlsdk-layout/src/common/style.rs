use std::borrow::Cow;

use crate::common::Pt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stroke<'doc> {
  pub width: Pt,
  pub color: Color,
  pub dash: Option<Vec<Pt>>,
  pub source_style_id: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fill<'doc> {
  None,
  Solid(Color),
  Theme(Cow<'doc, str>),
  Pattern {
    foreground: Color,
    background: Color,
  },
}

impl Default for Fill<'_> {
  fn default() -> Self {
    Self::None
  }
}
