use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::shape_group_context::PPTShapeGroupContext;
use super::slide::{BackgroundKind, BackgroundProperties, ShapeLocation, SlidePersist};

#[derive(Debug)]
pub(crate) struct SlideFragmentHandler {
  slide_persist: SlidePersist,
  shape_location: ShapeLocation,
  slide_name: Option<String>,
  slide_properties: SlideProperties,
  char_vector: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct SlideProperties;

impl SlideFragmentHandler {
  pub(crate) fn new(slide_persist: SlidePersist, shape_location: ShapeLocation) -> Self {
    Self {
      slide_persist,
      shape_location,
      slide_name: None,
      slide_properties: SlideProperties,
      char_vector: String::new(),
    }
  }

  pub(crate) fn import_slide_part(
    &mut self,
    package: &mut PresentationDocument,
    slide_part: &SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice oox/source/ppt/slidefragmenthandler.cxx
    // The constructor imports related vmlDrawing/legacyDrawing before XML
    // contexts create shapes; destruction converts and inserts VML drawing.
    self.slide_persist.import_image_parts(package, slide_part);
    self
      .slide_persist
      .import_graphic_frame_related_parts(package, slide_part)?;
    self.slide_persist.drawing.imported = slide_part.vml_drawing_parts(package).next().is_some();
    let slide = slide_part.root_element(package)?;
    self.slide_persist.visible = slide.show.is_none_or(|value| value.as_bool());
    self.slide_persist.show_master_shapes =
      slide.show_master_shapes.is_none_or(|value| value.as_bool());
    if !self.slide_persist.show_master_shapes {
      self.slide_persist.hide_master_location_shapes();
    }
    if let Some(color_map_override) = &slide.color_map_override {
      self
        .slide_persist
        .apply_color_map_override(color_map_override);
    }
    self.slide_name = slide.common_slide_data.name.clone();
    self.import_common_slide_data(&slide.common_slide_data);
    Ok(())
  }

  pub(crate) fn import_common_slide_data(&mut self, common_slide_data: &p::CommonSlideData) {
    self.slide_name = common_slide_data.name.clone();
    if let Some(background) = &common_slide_data.background {
      self.import_background(background);
    }
    self.on_create_context(&common_slide_data.shape_tree);
  }

  pub(crate) fn finalize_import(mut self) -> SlidePersist {
    self.slide_persist.name = self.slide_name;
    self.slide_persist.drawing.convert_and_insert();
    self.slide_persist
  }

  pub(crate) fn on_create_context(&mut self, shape_tree: &p::ShapeTree) {
    let mut group_context = PPTShapeGroupContext::new(self.shape_location);
    group_context.on_create_context(&mut self.slide_persist, shape_tree);
  }

  pub(crate) fn on_characters(&mut self, text: &str) {
    self.char_vector.push_str(text);
  }

  pub(crate) fn get_char_vector(&self) -> &str {
    &self.char_vector
  }

  fn import_background(&mut self, background: &p::Background) {
    // Source: LibreOffice oox/source/ppt/slidefragmenthandler.cxx
    // bgPr/bgRef populate SlidePersist background state; createBackground
    // later applies that state to the page.
    match background.background_choice.as_ref() {
      Some(p::BackgroundChoice::BackgroundProperties(properties)) => {
        if let Some(fill_properties) = import_background_fill(properties) {
          self.slide_persist.background_properties = Some(BackgroundProperties {
            kind: BackgroundKind::Properties(fill_properties),
          });
        }
      }
      Some(p::BackgroundChoice::BackgroundStyleReference(reference)) => {
        self.slide_persist.background_properties = Some(BackgroundProperties {
          kind: BackgroundKind::StyleReference {
            style_index: reference.index,
            placeholder_color: reference
              .background_style_reference_choice
              .as_ref()
              .and_then(import_background_reference_color),
          },
        });
      }
      None => {}
    }
  }
}

fn import_background_fill(properties: &p::BackgroundProperties) -> Option<FillProperties> {
  match properties.background_properties_choice1.as_ref()? {
    p::BackgroundPropertiesChoice::NoFill(_) => Some(FillProperties {
      kind: FillKind::None,
      placeholder_color: None,
    }),
    p::BackgroundPropertiesChoice::SolidFill(fill) => Some(FillProperties {
      kind: FillKind::Solid(import_solid_fill_color(fill)),
      placeholder_color: None,
    }),
    p::BackgroundPropertiesChoice::GradientFill(fill) => Some(FillProperties {
      kind: FillKind::Gradient(fill.clone()),
      placeholder_color: None,
    }),
    p::BackgroundPropertiesChoice::BlipFill(fill) => Some(FillProperties {
      kind: FillKind::Blip(fill.clone()),
      placeholder_color: None,
    }),
    p::BackgroundPropertiesChoice::PatternFill(fill) => Some(FillProperties {
      kind: FillKind::Pattern(fill.clone()),
      placeholder_color: None,
    }),
  }
}

fn import_background_reference_color(choice: &p::BackgroundStyleReferenceChoice) -> Option<Color> {
  Color::from_background_style_reference_choice(choice)
}

fn import_solid_fill_color(fill: &a::SolidFill) -> Option<Color> {
  Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)
}
