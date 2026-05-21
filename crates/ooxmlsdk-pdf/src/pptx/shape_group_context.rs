use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::drawingml::graphical_object_frame_context::GraphicalObjectFrameContext;
use super::drawingml::shape::{Point, Shape, ShapeService, Size};
use super::shape::PptShape;
use super::slide::{ShapeLocation, SlidePersist};

#[derive(Debug)]
pub(crate) struct PPTShapeGroupContext {
  shape_location: ShapeLocation,
  graphic_shape: Option<PptShape>,
}

impl PPTShapeGroupContext {
  pub(crate) fn new(shape_location: ShapeLocation) -> Self {
    Self {
      shape_location,
      graphic_shape: None,
    }
  }

  pub(crate) fn on_create_context(
    &mut self,
    slide_persist: &mut SlidePersist,
    shape_tree: &p::ShapeTree,
  ) {
    // Source: LibreOffice oox/source/ppt/pptshapegroupcontext.cxx
    // onCreateContext dispatches sp/cxnSp/grpSp/pic/graphicFrame and keeps
    // PPT shape-location state available to child shape contexts.
    self.import_shape_tree_choices(slide_persist, &shape_tree.shape_tree_choice);
  }

  fn import_shape_tree_choices(
    &mut self,
    slide_persist: &mut SlidePersist,
    choices: &[p::ShapeTreeChoice],
  ) {
    for choice in choices {
      match choice {
        p::ShapeTreeChoice::Shape(shape) => {
          self.import_shape(slide_persist, shape, ShapeService::CustomShape);
        }
        p::ShapeTreeChoice::GroupShape(group) => {
          self.import_group_shape(slide_persist, group);
        }
        p::ShapeTreeChoice::GraphicFrame(frame) => {
          self.import_graphic_frame(slide_persist, frame);
        }
        p::ShapeTreeChoice::ConnectionShape(shape) => {
          self.import_connection_shape(slide_persist, shape);
        }
        p::ShapeTreeChoice::Picture(picture) => {
          self.import_picture(slide_persist, picture);
        }
        p::ShapeTreeChoice::ContentPart(_) | p::ShapeTreeChoice::XmlAny(_) => {}
      }
    }
  }

  fn import_group_shape_choices(
    &mut self,
    slide_persist: &mut SlidePersist,
    choices: &[p::GroupShapeChoice],
  ) {
    for choice in choices {
      match choice {
        p::GroupShapeChoice::Shape(shape) => {
          self.import_shape(slide_persist, shape, ShapeService::CustomShape);
        }
        p::GroupShapeChoice::GroupShape(group) => {
          self.import_group_shape(slide_persist, group);
        }
        p::GroupShapeChoice::GraphicFrame(frame) => {
          self.import_graphic_frame(slide_persist, frame);
        }
        p::GroupShapeChoice::ConnectionShape(shape) => {
          self.import_connection_shape(slide_persist, shape);
        }
        p::GroupShapeChoice::Picture(picture) => {
          self.import_picture(slide_persist, picture);
        }
        p::GroupShapeChoice::ContentPart(_) => {}
      }
    }
  }

  fn import_shape(
    &mut self,
    slide_persist: &mut SlidePersist,
    source: &p::Shape,
    service_name: ShapeService,
  ) {
    let mut shape = PptShape::new(service_name, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &source
        .non_visual_shape_properties
        .non_visual_drawing_properties,
    );
    apply_transform_2d(
      &mut shape.shape,
      source.shape_properties.transform2_d.as_deref(),
    );
    if let Some(placeholder) = &source
      .non_visual_shape_properties
      .application_non_visual_drawing_properties
      .placeholder_shape
    {
      shape.shape.sub_type = placeholder
        .r#type
        .as_ref()
        .map(|value| format!("{value:?}"));
      shape.shape.sub_type_index = placeholder.index;
    }
    shape.add_shape(slide_persist);
  }

  fn import_group_shape(&mut self, slide_persist: &mut SlidePersist, group: &p::GroupShape) {
    let mut shape = PptShape::new(ShapeService::GroupShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &group
        .non_visual_group_shape_properties
        .non_visual_drawing_properties,
    );
    apply_group_transform(
      &mut shape.shape,
      group.group_shape_properties.transform_group.as_deref(),
    );
    shape.add_shape(slide_persist);
    self.import_group_shape_choices(slide_persist, &group.group_shape_choice);
  }

  fn import_graphic_frame(&mut self, slide_persist: &mut SlidePersist, frame: &p::GraphicFrame) {
    let mut shape = PptShape::new(ShapeService::GraphicObjectShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &frame
        .non_visual_graphic_frame_properties
        .non_visual_drawing_properties,
    );
    apply_presentation_transform(&mut shape.shape, &frame.transform);
    GraphicalObjectFrameContext
      .dispatch_graphic_data(&frame.graphic.graphic_data.uri, &mut shape.shape);
    self.graphic_shape = Some(shape.clone());
    shape.add_shape(slide_persist);
    self.import_ext_drawings(slide_persist);
  }

  fn import_connection_shape(
    &mut self,
    slide_persist: &mut SlidePersist,
    source: &p::ConnectionShape,
  ) {
    let mut shape = PptShape::new(ShapeService::ConnectorShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &source
        .non_visual_connection_shape_properties
        .non_visual_drawing_properties,
    );
    apply_transform_2d(
      &mut shape.shape,
      source.shape_properties.transform2_d.as_deref(),
    );
    shape.add_shape(slide_persist);
  }

  fn import_picture(&mut self, slide_persist: &mut SlidePersist, picture: &p::Picture) {
    let mut shape = PptShape::new(ShapeService::GraphicObjectShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &picture
        .non_visual_picture_properties
        .non_visual_drawing_properties,
    );
    apply_transform_2d(
      &mut shape.shape,
      picture.shape_properties.transform2_d.as_deref(),
    );
    shape.add_shape(slide_persist);
  }

  pub(crate) fn import_ext_drawings(&mut self, _slide_persist: &mut SlidePersist) {
    if let Some(shape) = &mut self.graphic_shape {
      shape.shape.keep_diagram_drawing();
    }
  }

  pub(crate) fn apply_font_ref_color(&mut self) {}
}

fn apply_non_visual_drawing_properties(
  shape: &mut Shape,
  properties: &p::NonVisualDrawingProperties,
) {
  shape.id = Some(properties.id);
  shape.name = Some(properties.name.clone());
  shape.description = properties.description.clone();
  shape.title = properties.title.clone();
  shape.hidden = properties
    .hidden
    .as_ref()
    .is_some_and(|hidden| hidden.as_bool());
}

fn apply_transform_2d(shape: &mut Shape, transform: Option<&a::Transform2D>) {
  let Some(transform) = transform else {
    return;
  };
  apply_transform_fields(
    shape,
    transform.rotation,
    transform.horizontal_flip.as_ref(),
    transform.vertical_flip.as_ref(),
    transform.offset.as_ref(),
    transform.extents.as_ref(),
  );
}

fn apply_presentation_transform(shape: &mut Shape, transform: &p::Transform) {
  apply_transform_fields(
    shape,
    transform.rotation,
    transform.horizontal_flip.as_ref(),
    transform.vertical_flip.as_ref(),
    transform.offset.as_ref(),
    transform.extents.as_ref(),
  );
}

fn apply_group_transform(shape: &mut Shape, transform: Option<&a::TransformGroup>) {
  let Some(transform) = transform else {
    return;
  };
  apply_transform_fields(
    shape,
    transform.rotation,
    transform.horizontal_flip.as_ref(),
    transform.vertical_flip.as_ref(),
    transform.offset.as_ref(),
    transform.extents.as_ref(),
  );
  if let Some(offset) = &transform.child_offset {
    shape.child_position = Point {
      x: offset.x.to_emu(),
      y: offset.y.to_emu(),
    };
  }
  if let Some(extents) = &transform.child_extents {
    shape.child_size = Size {
      cx: extents.cx.to_emu(),
      cy: extents.cy.to_emu(),
    };
  }
}

fn apply_transform_fields(
  shape: &mut Shape,
  rotation: Option<i32>,
  horizontal_flip: Option<&ooxmlsdk::simple_type::BooleanValue>,
  vertical_flip: Option<&ooxmlsdk::simple_type::BooleanValue>,
  offset: Option<&a::Offset>,
  extents: Option<&a::Extents>,
) {
  if let Some(rotation) = rotation {
    // Source: LibreOffice oox/source/drawingml/shapepropertiescontext.cxx
    // keeps DrawingML rotation as shape transform state before rendering.
    shape.rotation = rotation as f32 / 60_000.0;
  }
  shape.flip_h = horizontal_flip.is_some_and(|value| value.as_bool());
  shape.flip_v = vertical_flip.is_some_and(|value| value.as_bool());
  if let Some(offset) = offset {
    shape.position = Point {
      x: offset.x.to_emu(),
      y: offset.y.to_emu(),
    };
  }
  if let Some(extents) = extents {
    shape.size = Size {
      cx: extents.cx.to_emu(),
      cy: extents.cy.to_emu(),
    };
  }
}
