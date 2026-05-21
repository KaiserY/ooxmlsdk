use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::drawingml::shape::{Shape, ShapeService};
use super::slide::{ShapeLocation, SlidePersist};

#[derive(Clone, Debug)]
pub(crate) struct PptShape {
  pub(crate) shape: Shape,
  pub(crate) model_id: Option<String>,
  pub(crate) shape_location: ShapeLocation,
  pub(crate) referenced: bool,
  pub(crate) placeholder: Option<Box<Shape>>,
  pub(crate) has_noninherited_shape_properties: bool,
}

impl PptShape {
  pub(crate) fn new(service_name: ShapeService, shape_location: ShapeLocation) -> Self {
    Self {
      shape: Shape::new(service_name),
      model_id: None,
      shape_location,
      referenced: false,
      placeholder: None,
      has_noninherited_shape_properties: false,
    }
  }

  pub(crate) fn add_shape(self, slide_persist: &mut SlidePersist) {
    // Source: LibreOffice oox/source/ppt/pptshape.cxx
    // PPTShape::addShape selects service names and applies placeholder text
    // styles before generic DrawingML createAndInsert.
    let shape = self.into_shape(slide_persist);
    slide_persist.shapes.push(shape);
  }

  pub(crate) fn into_shape(mut self, slide_persist: &SlidePersist) -> Shape {
    self.set_text_master_styles(slide_persist);
    self.shape
  }

  pub(crate) fn find_placeholder<'a>(
    &'a self,
    slide_persist: &'a SlidePersist,
  ) -> Option<&'a Shape> {
    slide_persist.shapes.iter().find(|shape| {
      shape.sub_type == self.shape.sub_type && shape.sub_type_index == self.shape.sub_type_index
    })
  }

  pub(crate) fn find_placeholder_by_index<'a>(
    &'a self,
    slide_persist: &'a SlidePersist,
  ) -> Option<&'a Shape> {
    slide_persist
      .shapes
      .iter()
      .find(|shape| shape.sub_type_index == self.shape.sub_type_index)
  }

  pub(crate) fn get_sub_type_text_list_style(&self) -> Option<p::PlaceholderValues> {
    self.shape.sub_type
  }

  pub(crate) fn is_placeholder_candidate(&self) -> bool {
    self.shape.sub_type.is_some() || self.shape.sub_type_index.is_some()
  }

  pub(crate) fn set_text_master_styles(&mut self, _slide_persist: &SlidePersist) {}

  pub(crate) fn set_has_noninherited_shape_properties(&mut self) {
    self.has_noninherited_shape_properties = true;
  }
}
