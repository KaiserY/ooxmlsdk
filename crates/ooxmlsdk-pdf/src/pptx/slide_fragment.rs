use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;

use super::drawingml::fill::{FillKind, FillProperties};
use super::shape_group_context::PPTShapeGroupContext;
use super::slide::{BackgroundProperties, ShapeLocation, SlidePersist};

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
    self.slide_persist.drawing.imported = slide_part.vml_drawing_parts(package).next().is_some();
    let slide = slide_part.root_element(package)?;
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

  pub(crate) fn finalize_import(self) -> SlidePersist {
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
            fill_properties,
            color: None,
            style_index: None,
          });
        }
      }
      Some(p::BackgroundChoice::BackgroundStyleReference(reference)) => {
        self.slide_persist.background_properties = Some(BackgroundProperties {
          fill_properties: FillProperties {
            kind: FillKind::Solid(
              reference
                .background_style_reference_choice
                .as_ref()
                .and_then(import_background_reference_color)
                .unwrap_or_default(),
            ),
          },
          color: reference
            .background_style_reference_choice
            .as_ref()
            .and_then(import_background_reference_color),
          style_index: Some(reference.index),
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
    }),
    p::BackgroundPropertiesChoice::SolidFill(fill) => Some(FillProperties {
      kind: FillKind::Solid(import_solid_fill_color(fill).unwrap_or_default()),
    }),
    p::BackgroundPropertiesChoice::GradientFill(_)
    | p::BackgroundPropertiesChoice::BlipFill(_)
    | p::BackgroundPropertiesChoice::PatternFill(_) => None,
  }
}

fn import_background_reference_color(choice: &p::BackgroundStyleReferenceChoice) -> Option<String> {
  match choice {
    p::BackgroundStyleReferenceChoice::RgbColorModelHex(color) => Some(format!("#{}", color.val)),
    p::BackgroundStyleReferenceChoice::SchemeColor(color) => {
      Some(format!("scheme:{:?}", color.val))
    }
    p::BackgroundStyleReferenceChoice::PresetColor(color) => {
      Some(format!("preset:{:?}", color.val))
    }
    p::BackgroundStyleReferenceChoice::RgbColorModelPercentage(_)
    | p::BackgroundStyleReferenceChoice::HslColor(_)
    | p::BackgroundStyleReferenceChoice::SystemColor(_) => None,
  }
}

fn import_solid_fill_color(fill: &a::SolidFill) -> Option<String> {
  match fill.solid_fill_choice.as_ref()? {
    a::SolidFillChoice::RgbColorModelHex(color) => Some(format!("#{}", color.val)),
    a::SolidFillChoice::SchemeColor(color) => Some(format!("scheme:{:?}", color.val)),
    a::SolidFillChoice::PresetColor(color) => Some(format!("preset:{:?}", color.val)),
    a::SolidFillChoice::RgbColorModelPercentage(_)
    | a::SolidFillChoice::HslColor(_)
    | a::SolidFillChoice::SystemColor(_) => None,
  }
}
