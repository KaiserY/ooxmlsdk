use super::shape::Shape;

#[derive(Debug)]
pub(crate) struct ShapeContext<'a> {
  pub(crate) shape: &'a mut Shape,
}

impl<'a> ShapeContext<'a> {
  pub(crate) fn new(shape: &'a mut Shape) -> Self {
    Self { shape }
  }
}
