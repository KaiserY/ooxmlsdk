use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::presentation_part::PresentationPart;

use crate::error::Result;

use super::presentation::PresentationFragmentHandler;
use super::slide::{SlidePersist, SlideSize};

#[derive(Debug)]
pub(crate) struct PowerPointImport {
  pub(crate) themes: Vec<ThemeRecord>,
  pub(crate) draw_pages: Vec<SlidePersist>,
  pub(crate) master_pages: Vec<SlidePersist>,
  pub(crate) notes_pages: Vec<SlidePersist>,
  pub(crate) actual_slide_persist: Option<usize>,
  pub(crate) table_style_list_path: Option<String>,
  pub(crate) table_style_list: Option<TableStyleList>,
  pub(crate) chart_converter: ChartConverter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeRecord {
  pub(crate) path: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct TableStyleList {
  pub(crate) path: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ChartConverter;

impl PowerPointImport {
  pub(crate) fn import_document(package: &mut PresentationDocument) -> Result<Self> {
    // Source: LibreOffice oox/source/ppt/pptimport.cxx
    // PowerPointImport::importDocument resolves the officeDocument part,
    // imports the presentation fragment, then imports presProps when present.
    let presentation_part = package.presentation_part()?;
    let mut import = Self {
      themes: Vec::new(),
      draw_pages: Vec::new(),
      master_pages: Vec::new(),
      notes_pages: Vec::new(),
      actual_slide_persist: None,
      table_style_list_path: None,
      table_style_list: None,
      chart_converter: ChartConverter,
    };

    let mut handler = PresentationFragmentHandler::new(package, presentation_part.clone())?;
    handler.finalize_import(package, &mut import)?;
    Ok(import)
  }

  pub(crate) fn get_current_theme(&self) -> Option<&ThemeRecord> {
    self.get_current_theme_ptr()
  }

  pub(crate) fn get_current_theme_ptr(&self) -> Option<&ThemeRecord> {
    self.themes.first()
  }

  pub(crate) fn get_scheme_color_token(&self, token: &str) -> Option<String> {
    Some(token.to_string())
  }

  pub(crate) fn get_scheme_color(&self, token: &str) -> Option<String> {
    // Source: LibreOffice oox/source/ppt/pptimport.cxx
    // Full implementation must consult the active SlidePersist clrMap,
    // the master clrMap, then the current theme color scheme.
    self.get_scheme_color_token(token)
  }

  pub(crate) fn get_table_styles(&mut self) -> Option<&TableStyleList> {
    if self.table_style_list.is_none() {
      self.table_style_list = Some(TableStyleList {
        path: self.table_style_list_path.clone(),
      });
    }
    self.table_style_list.as_ref()
  }

  pub(crate) fn get_vml_drawing(&self) -> Option<()> {
    None
  }

  pub(crate) fn get_chart_converter(&self) -> &ChartConverter {
    &self.chart_converter
  }

  pub(crate) fn set_actual_slide_persist(&mut self, index: Option<usize>) {
    self.actual_slide_persist = index;
  }

  pub(crate) fn default_slide_size() -> SlideSize {
    SlideSize::libreoffice_default()
  }
}

pub(crate) fn part_path(package: &PresentationDocument, part: &PresentationPart) -> String {
  part
    .path(package)
    .map(str::to_string)
    .unwrap_or_else(|| "<presentation>".to_string())
}
