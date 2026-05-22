use std::collections::HashMap;

use ooxmlsdk::parts::notes_master_part::NotesMasterPart;
use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::presentation_part::PresentationPart;
use ooxmlsdk::parts::slide_layout_part::SlideLayoutPart;
use ooxmlsdk::parts::slide_master_part::SlideMasterPart;
use ooxmlsdk::parts::slide_part::SlidePart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::error::Result;

use super::drawingml::text_list_style::TextListStyle;
use super::drawingml::theme::{ThemeColorScheme, ThemeFontScheme, ThemeFormatScheme};
use super::import::{PowerPointImport, part_path};
use super::slide::{ColorMap, ShapeLocation, SlideCommentAuthor, SlidePersist, SlideSize};
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
  default_text_list_style: Option<TextListStyle>,
  slide_size: SlideSize,
  notes_size: SlideSize,
  comment_authors: Vec<SlideCommentAuthor>,
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
    let notes_master_vector = presentation
      .notes_master_id_list
      .as_ref()
      .map(|list| {
        list
          .notes_master_id
          .iter()
          .map(|id| id.id.clone())
          .collect()
      })
      .unwrap_or_default();
    let default_text_list_style = presentation
      .default_text_style
      .as_ref()
      .map(|style| TextListStyle::from_pml_default_text_style(style));
    let comment_authors = presentation_comment_authors(package, &presentation_part)?;

    Ok(Self {
      presentation_part,
      slide_master_vector,
      slides_vector,
      notes_master_vector,
      slide_id_to_index_map,
      custom_show_list: Vec::new(),
      section_list: Vec::new(),
      default_text_list_style,
      slide_size,
      notes_size,
      comment_authors_read: !comment_authors.is_empty(),
      comment_authors,
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
    self.import_notes_master_slides(package, import)?;
    for (index, slide_ref) in self.slides_vector.clone().into_iter().enumerate() {
      let Some(slide_part) =
        self.slide_part_by_relationship_id(package, &slide_ref.relationship_id)
      else {
        continue;
      };
      self.import_slide(package, import, index, slide_ref, slide_part)?;
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
    for relationship_id in self.slide_master_vector.clone() {
      let Some(master_part) = self.slide_master_part_by_relationship_id(package, &relationship_id)
      else {
        continue;
      };
      let path = master_part
        .path(package)
        .map(str::to_string)
        .unwrap_or_else(|| "<slideMaster>".to_string());
      let mut persist = SlidePersist::new_master(path, self.slide_size);
      persist.theme_path = self.import_master_theme(package, import, &master_part)?;
      persist.import_image_parts(package, &master_part);
      persist.import_media_reference_parts(package, &master_part);
      persist.import_graphic_frame_related_parts(package, &master_part)?;
      let master = master_part.root_element(package)?;
      persist.shape_location = ShapeLocation::Master;
      persist.set_color_map(ColorMap::from_pml(&master.color_map));
      persist.set_default_text_style(self.default_text_list_style.clone());
      if let Some(text_styles) = &master.text_styles {
        persist.set_text_styles(text_styles);
      }
      let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Master);
      handler.import_common_slide_data(&master.common_slide_data);
      let mut persist = handler.finalize_import();
      import.set_actual_slide_persist_context(Some(&persist));
      persist.create_background(import);
      persist.create_x_shapes(import);
      import.set_actual_slide_persist_context(None);
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
    slide_ref: SlideRef,
    slide_part: SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice PresentationFragmentHandler::importSlide.
    let path = slide_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| format!("<slide:{index}>"));
    let mut persist =
      SlidePersist::new_slide(path, slide_ref.relationship_id.clone(), self.slide_size);
    if let Some(layout_part) = slide_part.slide_layout_part(package) {
      persist.layout_path = layout_part.path(package).map(str::to_string);
      if let Some(master_part) = layout_part.slide_master_part(package) {
        persist.master_path = master_part.path(package).map(str::to_string);
      }
      persist.master_page_index = self.import_layout_persist(package, import, layout_part)?;
      if let Some(layout_persist) = persist
        .master_page_index
        .and_then(|index| import.master_pages.get(index))
      {
        persist.shapes = layout_persist.shapes.clone();
        persist.background_properties = layout_persist.background_properties.clone();
        persist.background_color = layout_persist.background_color.clone();
        persist.master_color_map = layout_persist.color_map.clone();
        persist.color_map = layout_persist.color_map.clone();
        persist.default_text_style = layout_persist.default_text_style.clone();
        persist.title_text_style = layout_persist.title_text_style.clone();
        persist.body_text_style = layout_persist.body_text_style.clone();
        persist.notes_text_style = layout_persist.notes_text_style.clone();
        persist.other_text_style = layout_persist.other_text_style.clone();
        persist.theme_path = layout_persist.theme_path.clone();
        persist.inherit_related_part_resources_from(layout_persist);
      }
    }

    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Slide);
    handler.import_slide_part(package, &slide_part)?;
    let mut persist = handler.finalize_import();
    self.import_slide_comments(package, &mut persist, &slide_part)?;
    import.set_actual_slide_persist(Some(import.draw_pages.len()));
    import.set_actual_slide_persist_context(Some(&persist));
    persist.create_background(import);
    persist.create_x_shapes(import);
    import.set_actual_slide_persist_context(None);
    import.draw_pages.push(persist);
    import.set_actual_slide_persist(None);
    self.import_notes_slide(package, import, index, &slide_part)?;
    Ok(())
  }

  fn slide_part_by_relationship_id(
    &self,
    package: &PresentationDocument,
    relationship_id: &str,
  ) -> Option<SlidePart> {
    self
      .presentation_part
      .slide_parts(package)
      .find(|part| self.presentation_part.get_id_of_part(package, part) == Some(relationship_id))
  }

  fn slide_master_part_by_relationship_id(
    &self,
    package: &PresentationDocument,
    relationship_id: &str,
  ) -> Option<SlideMasterPart> {
    self
      .presentation_part
      .slide_master_parts(package)
      .find(|part| self.presentation_part.get_id_of_part(package, part) == Some(relationship_id))
  }

  fn notes_master_part_by_relationship_id(
    &self,
    package: &PresentationDocument,
    relationship_id: &str,
  ) -> Option<NotesMasterPart> {
    self
      .presentation_part
      .notes_master_part(package)
      .filter(|part| self.presentation_part.get_id_of_part(package, part) == Some(relationship_id))
  }

  fn import_notes_master_slides(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
  ) -> Result<()> {
    // Source: LibreOffice PresentationFragmentHandler records notes master ids
    // from presentation.xml and imports them as master persists using notes
    // page size/style state before notes slides are imported.
    for relationship_id in self.notes_master_vector.clone() {
      let Some(notes_master_part) =
        self.notes_master_part_by_relationship_id(package, &relationship_id)
      else {
        continue;
      };
      self.import_notes_master_persist(package, import, notes_master_part)?;
    }
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
    if let Some(master_path) = &master_path
      && let Some(master_persist) = import
        .master_pages
        .iter()
        .find(|persist| persist.is_master && persist.path == *master_path)
    {
      persist.shapes = master_persist.shapes.clone();
      persist.background_properties = master_persist.background_properties.clone();
      persist.background_color = master_persist.background_color.clone();
      persist.master_color_map = master_persist.color_map.clone();
      persist.color_map = master_persist.color_map.clone();
      persist.default_text_style = master_persist.default_text_style.clone();
      persist.title_text_style = master_persist.title_text_style.clone();
      persist.body_text_style = master_persist.body_text_style.clone();
      persist.notes_text_style = master_persist.notes_text_style.clone();
      persist.other_text_style = master_persist.other_text_style.clone();
      persist.theme_path = master_persist.theme_path.clone();
      persist.inherit_related_part_resources_from(master_persist);
    }
    persist.import_image_parts(package, &layout_part);
    persist.import_media_reference_parts(package, &layout_part);
    persist.import_graphic_frame_related_parts(package, &layout_part)?;
    persist.layout_path = layout_path;
    persist.master_path = master_path;
    persist.shape_location = ShapeLocation::Layout;
    let layout = layout_part.root_element(package)?;
    if let Some(color_map_override) = &layout.color_map_override {
      persist.apply_color_map_override(color_map_override);
    }
    let show_master_shapes = layout
      .show_master_shapes
      .as_ref()
      .is_none_or(|value| value.as_bool());
    persist.show_master_shapes = show_master_shapes;
    if !show_master_shapes {
      persist.hide_shapes_as_master_shapes();
    }
    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Layout);
    handler.import_common_slide_data(&layout.common_slide_data);
    let mut persist = handler.finalize_import();
    import.set_actual_slide_persist_context(Some(&persist));
    persist.create_background(import);
    persist.create_x_shapes(import);
    import.set_actual_slide_persist_context(None);
    import.master_pages.push(persist);
    Ok(Some(import.master_pages.len() - 1))
  }

  fn import_notes_slide(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    slide_index: usize,
    slide_part: &SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice PresentationFragmentHandler::importSlide imports the
    // corresponding notesSlide after the visible slide and stores it in the
    // notes-pages collection instead of the draw-page collection.
    let Some(notes_part) = slide_part.notes_slide_part(package) else {
      return Ok(());
    };
    let path = notes_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| format!("<notesSlide:{slide_index}>"));
    let mut persist = SlidePersist::new_notes(
      path,
      slide_part
        .get_id_of_part(package, &notes_part)
        .map(str::to_string),
      self.notes_size,
    );
    if let Some(notes_master_part) = notes_part.notes_master_part(package) {
      persist.master_path = notes_master_part.path(package).map(str::to_string);
      persist.master_page_index =
        self.import_notes_master_persist(package, import, notes_master_part)?;
      if let Some(notes_master_persist) = persist
        .master_page_index
        .and_then(|index| import.master_pages.get(index))
      {
        persist.shapes = notes_master_persist.shapes.clone();
        persist.background_properties = notes_master_persist.background_properties.clone();
        persist.background_color = notes_master_persist.background_color.clone();
        persist.master_color_map = notes_master_persist.color_map.clone();
        persist.color_map = notes_master_persist.color_map.clone();
        persist.default_text_style = notes_master_persist.default_text_style.clone();
        persist.title_text_style = notes_master_persist.title_text_style.clone();
        persist.body_text_style = notes_master_persist.body_text_style.clone();
        persist.notes_text_style = notes_master_persist.notes_text_style.clone();
        persist.other_text_style = notes_master_persist.other_text_style.clone();
        persist.theme_path = notes_master_persist.theme_path.clone();
        persist.header_footer = notes_master_persist.header_footer.clone();
        persist.inherit_related_part_resources_from(notes_master_persist);
      }
    }
    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Slide);
    handler.import_notes_slide_part(package, &notes_part)?;
    let mut persist = handler.finalize_import();
    import.set_actual_slide_persist_context(Some(&persist));
    persist.create_background(import);
    persist.create_x_shapes(import);
    import.set_actual_slide_persist_context(None);
    import.notes_pages.push(persist);
    Ok(())
  }

  fn import_notes_master_persist(
    &mut self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    notes_master_part: NotesMasterPart,
  ) -> Result<Option<usize>> {
    let path = notes_master_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| "<notesMaster>".to_string());
    if let Some(index) = import
      .master_pages
      .iter()
      .position(|persist| persist.is_master && persist.is_notes && persist.path == path)
    {
      return Ok(Some(index));
    }
    let mut persist = SlidePersist::new_notes_master(path, self.notes_size);
    persist.theme_path = self.import_notes_master_theme(package, import, &notes_master_part)?;
    persist.import_image_parts(package, &notes_master_part);
    persist.import_media_reference_parts(package, &notes_master_part);
    persist.import_graphic_frame_related_parts(package, &notes_master_part)?;
    persist.shape_location = ShapeLocation::Master;
    let mut handler = SlideFragmentHandler::new(persist, ShapeLocation::Master);
    handler.import_notes_master_part(package, &notes_master_part)?;
    let mut persist = handler.finalize_import();
    import.set_actual_slide_persist_context(Some(&persist));
    persist.create_background(import);
    persist.create_x_shapes(import);
    import.set_actual_slide_persist_context(None);
    import.master_pages.push(persist);
    Ok(Some(import.master_pages.len() - 1))
  }

  fn import_master_theme(
    &self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    master_part: &SlideMasterPart,
  ) -> Result<Option<String>> {
    // Source: LibreOffice PresentationFragmentHandler::importMasterSlides
    // resolves a master theme fragment and stores/reuses it by path before
    // importing master shapes.
    let Some(theme_part) = master_part.theme_part(package) else {
      return Ok(None);
    };
    let path = theme_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| "<theme>".to_string());
    let theme = theme_part.root_element(package)?;
    import.ensure_theme(
      path.clone(),
      theme.name.clone(),
      theme.theme_id.clone(),
      ThemeColorScheme::from_dml(&theme.theme_elements.color_scheme),
      ThemeFontScheme::from_dml(&theme.theme_elements.font_scheme),
      ThemeFormatScheme::from_dml(&theme.theme_elements.format_scheme),
    );
    Ok(Some(path))
  }

  fn import_notes_master_theme(
    &self,
    package: &mut PresentationDocument,
    import: &mut PowerPointImport,
    notes_master_part: &NotesMasterPart,
  ) -> Result<Option<String>> {
    // Source: notes masters carry their own theme relationship just like slide
    // masters; keep the theme path on the notes master persist for style/color
    // resolution during notes shape creation.
    let Some(theme_part) = notes_master_part.theme_part(package) else {
      return Ok(None);
    };
    let path = theme_part
      .path(package)
      .map(str::to_string)
      .unwrap_or_else(|| "<theme>".to_string());
    let theme = theme_part.root_element(package)?;
    import.ensure_theme(
      path.clone(),
      theme.name.clone(),
      theme.theme_id.clone(),
      ThemeColorScheme::from_dml(&theme.theme_elements.color_scheme),
      ThemeFontScheme::from_dml(&theme.theme_elements.font_scheme),
      ThemeFormatScheme::from_dml(&theme.theme_elements.format_scheme),
    );
    Ok(Some(path))
  }

  fn import_slide_comments(
    &mut self,
    package: &mut PresentationDocument,
    persist: &mut SlidePersist,
    slide_part: &SlidePart,
  ) -> Result<()> {
    // Source: LibreOffice oox/source/ppt/presentationfragmenthandler.cxx
    // imports the presentation-wide comment authors before resolving per-slide
    // comments and attaching annotations to the slide persist.
    persist.set_comment_authors(self.comment_authors.clone());
    if let Some(comments_part) = slide_part.slide_comments_part(package) {
      let comments = comments_part.root_element(package)?;
      persist.import_legacy_comments(comments);
    }
    let comment_parts = slide_part.comment_parts(package).collect::<Vec<_>>();
    for comments_part in comment_parts {
      let comments = comments_part.root_element(package)?;
      persist.import_modern_comments(comments);
    }
    Ok(())
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

fn presentation_comment_authors(
  package: &mut PresentationDocument,
  presentation_part: &PresentationPart,
) -> Result<Vec<SlideCommentAuthor>> {
  let mut authors = Vec::new();
  if let Some(comment_authors_part) = presentation_part.comment_authors_part(package) {
    let author_list = comment_authors_part.root_element(package)?;
    authors.extend(
      author_list
        .comment_author
        .iter()
        .map(SlideCommentAuthor::from_pml),
    );
  }
  if let Some(authors_part) = presentation_part.authors_part(package) {
    let author_list = authors_part.root_element(package)?;
    authors.extend(
      author_list
        .author
        .iter()
        .map(SlideCommentAuthor::from_modern),
    );
  }
  Ok(authors)
}
