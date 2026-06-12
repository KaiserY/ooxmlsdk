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

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Fill<'doc> {
  #[default]
  None,
  Solid(Color),
  Theme(Cow<'doc, str>),
  Gradient(GradientFill<'doc>),
  Image {
    relationship_id: Option<Cow<'doc, str>>,
    tile: bool,
  },
  Pattern {
    foreground: Color,
    background: Color,
  },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientFill<'doc> {
  pub stops: Vec<GradientStop<'doc>>,
  pub angle_degrees: Option<f32>,
  pub path: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GradientStop<'doc> {
  pub position: f32,
  pub color: Color,
  pub scheme: Option<Cow<'doc, str>>,
}
