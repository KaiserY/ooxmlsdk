use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::presentation_part::PresentationPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::error::Result;

use super::presentation::PresentationFragmentHandler;
use super::slide::{SlidePersist, SlideSize};

#[derive(Debug)]
pub(crate) struct PowerPointImport {
  pub(crate) themes: Vec<ThemeFragmentRecord>,
  pub(crate) draw_pages: Vec<SlidePersist>,
  pub(crate) master_pages: Vec<SlidePersist>,
  pub(crate) notes_pages: Vec<SlidePersist>,
  pub(crate) actual_slide_persist: Option<usize>,
  pub(crate) actual_slide_persist_context: Option<Box<SlidePersist>>,
  pub(crate) table_style_list_path: Option<String>,
  pub(crate) table_style_list: Option<TableStyleList>,
  pub(crate) chart_converter: ChartConverter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ThemeFragmentRecord {
  pub(crate) path: String,
  pub(crate) name: Option<String>,
  pub(crate) theme_id: Option<String>,
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
      actual_slide_persist_context: None,
      table_style_list_path: None,
      table_style_list: None,
      chart_converter: ChartConverter,
    };

    let mut handler = PresentationFragmentHandler::new(package, presentation_part.clone())?;
    handler.finalize_import(package, &mut import)?;
    Ok(import)
  }

  pub(crate) fn get_current_theme(&self) -> Option<&ThemeFragmentRecord> {
    self.get_current_theme_ptr()
  }

  pub(crate) fn get_current_theme_ptr(&self) -> Option<&ThemeFragmentRecord> {
    self
      .actual_slide_persist_ref()
      .and_then(|slide| slide.theme_path.as_deref())
      .and_then(|path| self.get_theme(path))
      .or_else(|| self.themes.first())
  }

  pub(crate) fn get_theme(&self, path: &str) -> Option<&ThemeFragmentRecord> {
    self.themes.iter().find(|theme| theme.path == path)
  }

  pub(crate) fn ensure_theme(
    &mut self,
    path: String,
    name: Option<String>,
    theme_id: Option<String>,
  ) -> &ThemeFragmentRecord {
    if let Some(index) = self.themes.iter().position(|theme| theme.path == path) {
      return &self.themes[index];
    }
    self.themes.push(ThemeFragmentRecord {
      path,
      name,
      theme_id,
    });
    self.themes.last().expect("theme inserted")
  }

  pub(crate) fn get_scheme_color_token(
    &self,
    token: a::SchemeColorValues,
  ) -> Option<a::ColorSchemeIndexValues> {
    if let Some(slide) = self.actual_slide_persist_ref() {
      if let Some(mapped) = slide
        .color_map
        .as_ref()
        .and_then(|color_map| color_map.map_token(token))
      {
        return Some(mapped);
      }
      if let Some(mapped) = slide
        .master_color_map
        .as_ref()
        .and_then(|color_map| color_map.map_token(token))
      {
        return Some(mapped);
      }
      if let Some(mapped) = slide
        .master_page_index
        .and_then(|index| self.master_pages.get(index))
        .and_then(|master| master.color_map.as_ref())
        .and_then(|color_map| color_map.map_token(token))
      {
        return Some(mapped);
      }
    }
    match token {
      a::SchemeColorValues::Background1 => Some(a::ColorSchemeIndexValues::Light1),
      a::SchemeColorValues::Text1 => Some(a::ColorSchemeIndexValues::Dark1),
      a::SchemeColorValues::Background2 => Some(a::ColorSchemeIndexValues::Light2),
      a::SchemeColorValues::Text2 => Some(a::ColorSchemeIndexValues::Dark2),
      a::SchemeColorValues::Dark1 => Some(a::ColorSchemeIndexValues::Dark1),
      a::SchemeColorValues::Light1 => Some(a::ColorSchemeIndexValues::Light1),
      a::SchemeColorValues::Dark2 => Some(a::ColorSchemeIndexValues::Dark2),
      a::SchemeColorValues::Light2 => Some(a::ColorSchemeIndexValues::Light2),
      a::SchemeColorValues::Accent1 => Some(a::ColorSchemeIndexValues::Accent1),
      a::SchemeColorValues::Accent2 => Some(a::ColorSchemeIndexValues::Accent2),
      a::SchemeColorValues::Accent3 => Some(a::ColorSchemeIndexValues::Accent3),
      a::SchemeColorValues::Accent4 => Some(a::ColorSchemeIndexValues::Accent4),
      a::SchemeColorValues::Accent5 => Some(a::ColorSchemeIndexValues::Accent5),
      a::SchemeColorValues::Accent6 => Some(a::ColorSchemeIndexValues::Accent6),
      a::SchemeColorValues::Hyperlink => Some(a::ColorSchemeIndexValues::Hyperlink),
      a::SchemeColorValues::FollowedHyperlink => Some(a::ColorSchemeIndexValues::FollowedHyperlink),
      a::SchemeColorValues::PhColor => None,
    }
  }

  pub(crate) fn get_scheme_color(&self, token: a::SchemeColorValues) -> Option<String> {
    // Source: LibreOffice oox/source/ppt/pptimport.cxx
    // getSchemeColor first maps the scheme token using the active slide/layout
    // and master color maps, then resolves the mapped token against the current
    // DrawingML theme. Do not return a token string as if it were a resolved
    // color; theme color-scheme import must be ported from upstream first.
    let _mapped_token = self.get_scheme_color_token(token)?;
    None
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
    if index.is_none() {
      self.actual_slide_persist_context = None;
    }
  }

  pub(crate) fn set_actual_slide_persist_context(&mut self, persist: Option<&SlidePersist>) {
    self.actual_slide_persist_context = persist.cloned().map(Box::new);
  }

  fn actual_slide_persist_ref(&self) -> Option<&SlidePersist> {
    self.actual_slide_persist_context.as_deref().or_else(|| {
      self
        .actual_slide_persist
        .and_then(|index| self.draw_pages.get(index))
    })
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
