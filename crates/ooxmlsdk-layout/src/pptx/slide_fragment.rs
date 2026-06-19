use ooxmlsdk::parts::notes_master_part::NotesMasterPart;
use ooxmlsdk::parts::notes_slide_part::NotesSlidePart;
use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::text_list_style::TextListStyle;
use super::shape_group_context::PPTShapeGroupContext;
use super::slide::{
  BackgroundKind, BackgroundProperties, ColorMap, HeaderFooter, ShapeLocation, SlidePersist,
};

#[derive(Debug)]
pub(crate) struct SlideFragmentHandler {
  slide_persist: SlidePersist,
  shape_location: ShapeLocation,
  slide_name: Option<String>,
}

impl SlideFragmentHandler {
  pub(crate) fn new(slide_persist: SlidePersist, shape_location: ShapeLocation) -> Self {
    Self {
      slide_persist,
      shape_location,
      slide_name: None,
    }
  }

  pub(crate) fn import_slide_part(
    &mut self,
    package: &mut PresentationDocument,
    slide_part: &SlidePart,
  ) -> Result<()> {
    // The constructor imports related vmlDrawing/legacyDrawing before XML
    // contexts create shapes; destruction converts and inserts VML drawing.
    self.slide_persist.import_image_parts(package, slide_part);
    self
      .slide_persist
      .import_media_reference_parts(package, slide_part);
    self
      .slide_persist
      .import_hyperlink_reference_parts(package, slide_part);
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

  pub(crate) fn import_notes_slide_part(
    &mut self,
    package: &mut PresentationDocument,
    notes_part: &NotesSlidePart,
  ) -> Result<()> {
    // notes as a slide fragment with its own relationship scope and notes-size
    // page geometry. The notes master is attached by PresentationFragmentHandler
    // before this method, mirroring SlidePersist::setMasterPersist.
    self.slide_persist.import_image_parts(package, notes_part);
    self
      .slide_persist
      .import_media_reference_parts(package, notes_part);
    self
      .slide_persist
      .import_hyperlink_reference_parts(package, notes_part);
    self
      .slide_persist
      .import_graphic_frame_related_parts(package, notes_part)?;
    self.slide_persist.drawing.imported = notes_part.vml_drawing_parts(package).next().is_some();
    let notes = notes_part.root_element(package)?;
    self.slide_persist.show_master_shapes =
      notes.show_master_shapes.is_none_or(|value| value.as_bool());
    if !self.slide_persist.show_master_shapes {
      self.slide_persist.hide_master_location_shapes();
    }
    if let Some(color_map_override) = &notes.color_map_override {
      self
        .slide_persist
        .apply_color_map_override(color_map_override);
    }
    self.slide_name = notes.common_slide_data.name.clone();
    self.import_common_slide_data(&notes.common_slide_data);
    Ok(())
  }

  pub(crate) fn import_notes_master_part(
    &mut self,
    package: &mut PresentationDocument,
    notes_master_part: &NotesMasterPart,
  ) -> Result<()> {
    // notes slide content so placeholder/style lookup uses notes text style.
    self
      .slide_persist
      .import_image_parts(package, notes_master_part);
    self
      .slide_persist
      .import_media_reference_parts(package, notes_master_part);
    self
      .slide_persist
      .import_hyperlink_reference_parts(package, notes_master_part);
    self
      .slide_persist
      .import_graphic_frame_related_parts(package, notes_master_part)?;
    self.slide_persist.drawing.imported = notes_master_part
      .vml_drawing_parts(package)
      .next()
      .is_some();
    let notes_master = notes_master_part.root_element(package)?;
    self
      .slide_persist
      .set_color_map(ColorMap::from_pml(&notes_master.color_map));
    self.slide_persist.header_footer = notes_master
      .header_footer
      .as_deref()
      .map(HeaderFooter::from_pml)
      .unwrap_or_default();
    self.slide_persist.notes_text_style = notes_master
      .notes_style
      .as_ref()
      .map(|style| TextListStyle::from_pml_notes_style(style));
    self.slide_name = notes_master.common_slide_data.name.clone();
    self.import_common_slide_data(&notes_master.common_slide_data);
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

  fn import_background(&mut self, background: &p::Background) {
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
