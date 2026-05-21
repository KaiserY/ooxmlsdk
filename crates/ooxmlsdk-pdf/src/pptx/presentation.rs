use std::collections::HashMap;

use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::presentation_part::PresentationPart;
use ooxmlsdk::parts::slide_layout_part::SlideLayoutPart;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;

use super::import::{PowerPointImport, part_path};
use super::slide::{ShapeLocation, SlidePersist, SlideSize};
use super::slide_fragment::SlideFragmentHandler;

#[derive(Debug)]
pub(crate) struct PresentationFragmentHandler {
  presentation_part: PresentationPart,
  slide_master_vector: Vec<String>,
  slides_vector: Vec<SlideRef>,
  notes_master_vector: Vec<String>,
  slide_id_to_index_map: HashMap<u32, usize>,
  custom_show_list: Vec<CustomShow>,
  section_list: Vec<SlideSection>,
  default_text_list_style: Option<DefaultTextListStyle>,
  slide_size: SlideSize,
  notes_size: SlideSize,
  author_list: Vec<CommentAuthor>,
  comment_authors_read: bool,
  embed_true_type_fonts: bool,
  in_section_extension: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SlideRef {
  pub(crate) slide_id: u32,
  pub(crate) relationship_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CustomShow {
  pub(crate) name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SlideSection {
  pub(crate) name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommentAuthor {
  pub(crate) id: u32,
  pub(crate) name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DefaultTextListStyle;

impl PresentationFragmentHandler {
  pub(crate) fn new(
    package: &mut PresentationDocument,
    presentation_part: PresentationPart,
  ) -> Result<Self> {
    // Source: LibreOffice oox/source/ppt/presentationfragmenthandler.cxx
    // PresentationFragmentHandler records ids and presentation-wide state
    // before finalizeImport imports master and slide fragments.
    let presentation = presentation_part.root_element(package)?;
    let slide_size = presentation
      .slide_size
      .as_ref()
      .map(SlideSize::from_pml)
      .unwrap_or_else(PowerPointImport::default_slide_size);
    let notes_size = SlideSize::from_notes(&presentation.notes_size);
    let slides_vector = slide_refs(&presentation);
    let slide_id_to_index_map = slides_vector
      .iter()
      .enumerate()
      .map(|(index, slide_ref)| (slide_ref.slide_id, index))
      .collect();
    let slide_master_vector = presentation
      .slide_master_id_list
      .as_ref()
      .map(|list| {
        list
          .slide_master_id
          .iter()
          .map(|id| id.relationship_id.clone())
          .collect()
      })
      .unwrap_or_default();

    Ok(Self {
      presentation_part,
      slide_master_vector,
      slides_vector,
      notes_master_vector: Vec::new(),
      slide_id_to_index_map,
      custom_show_list: Vec::new(),
      section_list: Vec::new(),
      default_text_list_style: None,
      slide_size,
      notes_size,
      author_list: Vec::new(),
      comment_authors_read: false,
      embed_true_type_fonts: false,
      in_section_extension: false,
    })
  }

  pub(crate) fn finalize_import(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
  ) -> Result<()> {
    self.import_master_slides(package, import)?;
    let slide_parts = self
      .presentation_part
      .slide_parts(package)
      .collect::<Vec<_>>();
    for (index, slide_part) in slide_parts.iter().enumerate() {
      self.import_slide(package, import, index, slide_part.clone())?;
    }
    self.import_slide_names();
    self.import_custom_slide_show();
    self.save_sections();
    Ok(())
  }

  pub(crate) fn import_master_slides(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
  ) -> Result<()> {
    // Source: LibreOffice PresentationFragmentHandler::importMasterSlides.
    let master_parts = self
      .presentation_part
      .slide_master_parts(package)
      .collect::<Vec<_>>();
    for master_part in master_parts {
      let path = master_part
        .path(package)
        .map(str::to_string)
        .unwrap_or_else(|| "<slideMaster>".to_string());
      let mut persist = SlidePersist::new_master(path, self.slide_size);
      let master = master_part.root_element(package)?;
      persist.shape_location = ShapeLocation::Master;
      let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Master);
      handler.import_common_slide_data(&master.common_slide_data);
      let persist = handler.finalize_import();
      import.master_pages.push(persist);
    }
    Ok(())
  }

  pub(crate) fn import_master_slide(&mut self) {}

  pub(crate) fn import_slide(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    index: usize,
    slide_part: SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice PresentationFragmentHandler::importSlide.
    let path = slide_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| format!("<slide:{index}>"));
    let relationship_id = self
      .presentation_part
      .get_id_of_part(package, &slide_part)
      .map(str::to_string)
      .unwrap_or_else(|| {
        self
          .slides_vector
          .get(index)
          .map(|slide_ref| slide_ref.relationship_id.clone())
          .unwrap_or_default()
      });
    let mut persist = SlidePersist::new_slide(path, relationship_id, self.slide_size);
    if let Some(layout_part) = slide_part.slide_layout_part(package) {
      persist.layout_path = layout_part.path(package).map(str::to_string);
      if let Some(master_part) = layout_part.slide_master_part(package) {
        persist.master_path = master_part.path(package).map(str::to_string);
      }
      persist.master_page_index = self.import_layout_persist(package, import, layout_part)?;
    }

    import.set_actual_slide_persist(Some(import.draw_pages.len()));
    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Slide);
    handler.import_slide_part(package, &slide_part)?;
    let mut persist = handler.finalize_import();
    persist.create_background(import);
    persist.create_x_shapes(import);
    import.draw_pages.push(persist);
    import.set_actual_slide_persist(None);
    Ok(())
  }

  fn import_layout_persist(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    layout_part: SlideLayoutPart,
  ) -> Result<Option<usize>> {
    // Source: LibreOffice PresentationFragmentHandler::importSlide checks the
    // corresponding masterpage+layout pair and reuses an existing persist.
    let layout_path = layout_part.path(package).map(str::to_string);
    let master_path = layout_part
      .slide_master_part(package)
      .and_then(|master| master.path(package).map(str::to_string));
    if let Some(index) = import
      .master_pages
      .iter()
      .position(|persist| persist.layout_path == layout_path && persist.master_path == master_path)
    {
      return Ok(Some(index));
    }

    let path = layout_path
      .clone()
      .unwrap_or_else(|| "<slideLayout>".to_string());
    let mut persist = SlidePersist::new_layout(path, self.slide_size);
    persist.layout_path = layout_path;
    persist.master_path = master_path;
    persist.shape_location = ShapeLocation::Layout;
    let layout = layout_part.root_element(package)?;
    let show_master_shapes = layout
      .show_master_shapes
      .as_ref()
      .is_none_or(|value| value.as_bool());
    if !show_master_shapes {
      persist.hide_shapes_as_master_shapes();
    }
    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Layout);
    handler.import_common_slide_data(&layout.common_slide_data);
    let persist = handler.finalize_import();
    import.master_pages.push(persist);
    Ok(Some(import.master_pages.len() - 1))
  }

  pub(crate) fn import_slide_names(&self) {}

  pub(crate) fn save_sections(&self) {}

  pub(crate) fn save_theme_to_grab_bag(&self) {}

  pub(crate) fn save_color_map_to_grab_bag(&self) {}

  pub(crate) fn import_custom_slide_show(&self) {}

  pub(crate) fn presentation_path(&self, package: &PresentationDocument) -> String {
    part_path(package, &self.presentation_part)
  }
}

fn slide_refs(presentation: &p::Presentation) -> Vec<SlideRef> {
  presentation
    .slide_id_list
    .as_ref()
    .map(|list| {
      list
        .slide_id
        .iter()
        .map(|slide_id| SlideRef {
          slide_id: slide_id.id,
          relationship_id: slide_id.relationship_id.clone(),
        })
        .collect()
    })
    .unwrap_or_default()
}
