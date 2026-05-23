use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::shape::PptShape;
use super::slide::SlidePersist;

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
    // Source: LibreOffice oox/source/ppt/pptshapecontext.cxx
    // Placeholder lookup applies layout/master references before shape
    // properties and text body are finalized.
    let mut sub_type = placeholder.r#type.unwrap_or(p::PlaceholderValues::Object);
    if placeholder.index != Some(u32::MAX) {
      self.shape.shape.sub_type_index = placeholder.index;
      if placeholder.r#type.is_none()
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
