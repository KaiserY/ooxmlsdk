use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::shape::PptShape;
use super::slide::{ShapeLocation, SlidePersist};

#[derive(Debug)]
pub(crate) struct PPTShapeContext<'a> {
  shape: &'a mut PptShape,
}

impl<'a> PPTShapeContext<'a> {
  pub(crate) fn new(shape: &'a mut PptShape) -> Self {
    Self { shape }
  }

  pub(crate) fn on_create_context(
    &mut self,
    slide_persist: &mut SlidePersist,
    placeholder: &p::PlaceholderShape,
  ) {
    // Placeholder lookup applies layout/master references before shape
    // properties and text body are finalized.
    // ECMA-376 Part 1, CT_Placeholder defines `obj` as the default for an
    // omitted type. A slide can still adopt the already-resolved layout type
    // through its placeholder index, but a layout must not let an unrelated
    // same-index master placeholder replace that schema default (for example,
    // an object placeholder whose index is reused by a master date field).
    let mut sub_type = placeholder.r#type.unwrap_or(p::PlaceholderValues::Object);
    if placeholder.index != Some(u32::MAX) {
      self.shape.shape.sub_type_index = placeholder.index;
      if self.shape.shape_location == ShapeLocation::Slide
        && placeholder.r#type.is_none()
        && let Some(index) = placeholder.index
        && let Some(inherited_type) = PptShape::placeholder_type_by_index(slide_persist, index)
      {
        sub_type = inherited_type;
      }
    }
    self.shape.shape.sub_type = Some(sub_type);
    self.shape.apply_placeholder_reference(slide_persist);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pptx::drawingml::shape::{Shape, ShapeService};
  use crate::pptx::slide::{ShapeLocation, SlideSize};

  #[test]
  fn omitted_placeholder_type_uses_object_default_before_index_matching() {
    let mut slide = SlidePersist::new_layout(
      "ppt/slideLayouts/slideLayout1.xml".to_string(),
      SlideSize {
        width_pt: 720.0,
        height_pt: 540.0,
      },
    );
    let mut body = Shape::new(ShapeService::Outliner);
    body.shape_location = Some(ShapeLocation::Master);
    body.sub_type = Some(p::PlaceholderValues::Body);
    body.sub_type_index = Some(1);
    let mut date = Shape::new(ShapeService::DateTime);
    date.shape_location = Some(ShapeLocation::Master);
    date.sub_type = Some(p::PlaceholderValues::DateAndTime);
    date.sub_type_index = Some(2);
    slide.shapes = vec![body, date];

    let mut shape = PptShape::new(ShapeService::Outliner, ShapeLocation::Layout);
    PPTShapeContext::new(&mut shape).on_create_context(
      &mut slide,
      &p::PlaceholderShape {
        r#type: None,
        index: Some(2),
        ..p::PlaceholderShape::default()
      },
    );

    assert_eq!(shape.shape.sub_type, Some(p::PlaceholderValues::Object));
    assert_eq!(
      shape
        .shape
        .placeholder
        .as_deref()
        .and_then(|placeholder| placeholder.sub_type),
      Some(p::PlaceholderValues::Body)
    );
  }
}
