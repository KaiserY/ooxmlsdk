use crate::common::Pt;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
  pub x: Pt,
  pub y: Pt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
  pub width: Pt,
  pub height: Pt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
  pub origin: Point,
  pub size: Size,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Insets {
  pub top: Pt,
  pub right: Pt,
  pub bottom: Pt,
  pub left: Pt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
  pub m11: f32,
  pub m12: f32,
  pub m21: f32,
  pub m22: f32,
  pub dx: Pt,
  pub dy: Pt,
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      m11: 1.0,
      m12: 0.0,
      m21: 0.0,
      m22: 1.0,
      dx: Pt(0.0),
      dy: Pt(0.0),
    }
  }
}
