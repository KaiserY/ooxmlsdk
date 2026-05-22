use ooxmlsdk::schemas::schemas_microsoft_com_office_powerpoint_2010_main as p14;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::graphical_object_frame_context::GraphicalObjectFrameContext;
use super::drawingml::line::LineProperties;
use super::drawingml::shape::{CustomShapeGeometry, FrameType, Point, Shape, ShapeService, Size};
use super::drawingml::text_body::TextBody;
use super::shape::PptShape;
use super::shape_context::PPTShapeContext;
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
      if let Some(shape) = self.import_shape_tree_choice(slide_persist, choice) {
        slide_persist.shapes.push(shape);
      }
    }
  }

  fn import_shape_tree_choice(
    &mut self,
    slide_persist: &mut SlidePersist,
    choice: &p::ShapeTreeChoice,
  ) -> Option<Shape> {
    match choice {
      p::ShapeTreeChoice::Shape(shape) => {
        Some(self.import_shape(slide_persist, shape, ShapeService::CustomShape))
      }
      p::ShapeTreeChoice::GroupShape(group) => Some(self.import_group_shape(slide_persist, group)),
      p::ShapeTreeChoice::GraphicFrame(frame) => {
        Some(self.import_graphic_frame(slide_persist, frame))
      }
      p::ShapeTreeChoice::ConnectionShape(shape) => {
        Some(self.import_connection_shape(slide_persist, shape))
      }
      p::ShapeTreeChoice::Picture(picture) => Some(self.import_picture(slide_persist, picture)),
      p::ShapeTreeChoice::ContentPart(content_part) => {
        Some(self.import_content_part(slide_persist, content_part))
      }
      p::ShapeTreeChoice::XmlAny(_) => None,
    }
  }

  fn import_group_shape_choice(
    &mut self,
    slide_persist: &mut SlidePersist,
    choice: &p::GroupShapeChoice,
  ) -> Option<Shape> {
    match choice {
      p::GroupShapeChoice::Shape(shape) => {
        Some(self.import_shape(slide_persist, shape, ShapeService::CustomShape))
      }
      p::GroupShapeChoice::GroupShape(group) => Some(self.import_group_shape(slide_persist, group)),
      p::GroupShapeChoice::GraphicFrame(frame) => {
        Some(self.import_graphic_frame(slide_persist, frame))
      }
      p::GroupShapeChoice::ConnectionShape(shape) => {
        Some(self.import_connection_shape(slide_persist, shape))
      }
      p::GroupShapeChoice::Picture(picture) => Some(self.import_picture(slide_persist, picture)),
      p::GroupShapeChoice::ContentPart(content_part) => {
        Some(self.import_content_part(slide_persist, content_part))
      }
    }
  }

  fn import_shape(
    &mut self,
    slide_persist: &mut SlidePersist,
    source: &p::Shape,
    service_name: ShapeService,
  ) -> Shape {
    let mut shape = PptShape::new(service_name, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &source
        .non_visual_shape_properties
        .non_visual_drawing_properties,
    );
    if let Some(placeholder) = &source
      .non_visual_shape_properties
      .application_non_visual_drawing_properties
      .placeholder_shape
    {
      PPTShapeContext::new(&mut shape).on_create_context(slide_persist, placeholder);
    }
    apply_transform_2d(
      &mut shape.shape,
      source.shape_properties.transform2_d.as_deref(),
    );
    apply_shape_properties(&mut shape.shape, &source.shape_properties);
    if let Some(style) = &source.shape_style {
      shape.shape.set_shape_style_refs(style);
    }
    if let Some(text_body) = &source.text_body {
      shape.shape.set_text_body(TextBody::from_pml(text_body));
    }
    shape.into_shape(slide_persist)
  }

  fn import_group_shape(
    &mut self,
    slide_persist: &mut SlidePersist,
    group: &p::GroupShape,
  ) -> Shape {
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
    shape.shape.children = group
      .group_shape_choice
      .iter()
      .filter_map(|choice| self.import_group_shape_choice(slide_persist, choice))
      .collect();
    shape.into_shape(slide_persist)
  }

  fn import_graphic_frame(
    &mut self,
    slide_persist: &mut SlidePersist,
    frame: &p::GraphicFrame,
  ) -> Shape {
    let mut shape = PptShape::new(ShapeService::GraphicObjectShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &frame
        .non_visual_graphic_frame_properties
        .non_visual_drawing_properties,
    );
    apply_presentation_transform(&mut shape.shape, &frame.transform);
    GraphicalObjectFrameContext
      .dispatch_graphic_data(&frame.graphic.graphic_data, &mut shape.shape);
    self.graphic_shape = Some(shape);
    self.import_ext_drawings();
    let shape = self
      .graphic_shape
      .take()
      .unwrap_or_else(|| PptShape::new(ShapeService::GraphicObjectShape, self.shape_location));
    shape.into_shape(slide_persist)
  }

  fn import_connection_shape(
    &mut self,
    slide_persist: &mut SlidePersist,
    source: &p::ConnectionShape,
  ) -> Shape {
    let mut shape = PptShape::new(ShapeService::ConnectorShape, self.shape_location);
    apply_non_visual_drawing_properties(
      &mut shape.shape,
      &source
        .non_visual_connection_shape_properties
        .non_visual_drawing_properties,
    );
    apply_connection_shape_properties(
      &mut shape.shape,
      &source
        .non_visual_connection_shape_properties
        .non_visual_connector_shape_drawing_properties,
    );
    apply_transform_2d(
      &mut shape.shape,
      source.shape_properties.transform2_d.as_deref(),
    );
    apply_shape_properties(&mut shape.shape, &source.shape_properties);
    if let Some(style) = &source.shape_style {
      shape.shape.set_shape_style_refs(style);
    }
    shape.into_shape(slide_persist)
  }

  fn import_picture(&mut self, slide_persist: &mut SlidePersist, picture: &p::Picture) -> Shape {
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
    apply_shape_properties(&mut shape.shape, &picture.shape_properties);
    if let Some(style) = &picture.shape_style {
      shape.shape.set_shape_style_refs(style);
    }
    if let Some(blip) = picture.blip_fill.blip.as_ref() {
      shape
        .shape
        .set_picture(blip.embed.clone(), blip.link.clone());
    }
    shape.into_shape(slide_persist)
  }

  fn import_content_part(
    &mut self,
    slide_persist: &mut SlidePersist,
    content_part: &p::ContentPart,
  ) -> Shape {
    let mut shape = PptShape::new(ShapeService::MediaShape, self.shape_location);
    if let Some(properties) = &content_part.non_visual_content_part_properties {
      apply_p14_non_visual_drawing_properties(
        &mut shape.shape,
        &properties.non_visual_drawing_properties,
      );
    }
    apply_p14_transform_2d(&mut shape.shape, content_part.transform2_d.as_deref());
    shape.shape.set_content_part(content_part.r_id.clone());
    shape.into_shape(slide_persist)
  }

  pub(crate) fn import_ext_drawings(&mut self) {
    if let Some(shape) = &mut self.graphic_shape {
      if shape.shape.frame_type == FrameType::Diagram {
        shape.shape.keep_diagram_drawing();
      }
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

fn apply_connection_shape_properties(
  shape: &mut Shape,
  properties: &p::NonVisualConnectorShapeDrawingProperties,
) {
  if let Some(connection) = &properties.start_connection {
    shape.add_connector_shape_properties(true, connection.id, connection.index);
  }
  if let Some(connection) = &properties.end_connection {
    shape.add_connector_shape_properties(false, connection.id, connection.index);
  }
}

fn apply_p14_non_visual_drawing_properties(
  shape: &mut Shape,
  properties: &p14::NonVisualDrawingProperties,
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

fn apply_shape_properties(shape: &mut Shape, properties: &p::ShapeProperties) {
  // Source: LibreOffice oox/source/drawingml/shapepropertiescontext.cxx
  // ShapePropertiesContext owns fill/line/effect state before the PPT shape is
  // converted to drawing objects.
  if let Some(geometry) = properties
    .shape_properties_choice1
    .as_ref()
    .map(import_custom_shape_geometry)
  {
    shape.set_custom_shape_geometry(geometry);
  }
  if let Some(fill) = properties
    .shape_properties_choice2
    .as_ref()
    .and_then(import_fill_properties)
  {
    shape.fill_properties = Some(fill);
  }
  if let Some(line) = properties
    .outline
    .as_deref()
    .and_then(import_line_properties)
  {
    shape.line_properties = Some(line);
  }
}

fn import_custom_shape_geometry(choice: &p::ShapePropertiesChoice) -> CustomShapeGeometry {
  // Source: LibreOffice oox/source/drawingml/shapepropertiescontext.cxx
  // custGeom/prstGeom populate CustomShapeProperties before createAndInsert;
  // do not lower preset geometry to PDF path data during import.
  match choice {
    p::ShapePropertiesChoice::CustomGeometry(geometry) => {
      CustomShapeGeometry::Custom(geometry.clone())
    }
    p::ShapePropertiesChoice::PresetGeometry(geometry) => {
      CustomShapeGeometry::Preset(geometry.clone())
    }
  }
}

fn import_fill_properties(choice: &p::ShapePropertiesChoice2) -> Option<FillProperties> {
  match choice {
    p::ShapePropertiesChoice2::NoFill(_) => Some(FillProperties {
      kind: FillKind::None,
      placeholder_color: None,
    }),
    p::ShapePropertiesChoice2::SolidFill(fill) => Some(FillProperties {
      kind: FillKind::Solid(import_solid_fill_color(fill)),
      placeholder_color: None,
    }),
    p::ShapePropertiesChoice2::GroupFill => Some(FillProperties {
      kind: FillKind::Group,
      placeholder_color: None,
    }),
    p::ShapePropertiesChoice2::GradientFill(fill) => Some(FillProperties {
      kind: FillKind::Gradient(fill.clone()),
      placeholder_color: None,
    }),
    p::ShapePropertiesChoice2::BlipFill(fill) => Some(FillProperties {
      kind: FillKind::Blip(fill.clone()),
      placeholder_color: None,
    }),
    p::ShapePropertiesChoice2::PatternFill(fill) => Some(FillProperties {
      kind: FillKind::Pattern(fill.clone()),
      placeholder_color: None,
    }),
  }
}

fn import_line_properties(outline: &a::Outline) -> Option<LineProperties> {
  LineProperties::from_dml_outline(outline)
}

fn import_solid_fill_color(fill: &a::SolidFill) -> Option<Color> {
  Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)
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

fn apply_p14_transform_2d(shape: &mut Shape, transform: Option<&p14::Transform2D>) {
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
