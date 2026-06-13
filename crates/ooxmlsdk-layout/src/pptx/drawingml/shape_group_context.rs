use super::shape::Shape;

#[derive(Debug)]
pub(crate) struct ShapeGroupContext {
  pub(crate) children: Vec<Shape>,
}

impl ShapeGroupContext {
  pub(crate) fn new() -> Self {
    Self {
      children: Vec::new(),
    }
  }
}
