use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::parts::presentation_part::PresentationPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::error::Result;

use super::drawingml::color::Color;
use super::drawingml::fill::FillProperties;
use super::drawingml::line::LineProperties;
use super::drawingml::shape_properties::EffectProperties;
use super::drawingml::table::{TableStyle, TableStyleList};
use super::drawingml::theme::{ThemeColorScheme, ThemeFontScheme, ThemeFormatScheme};
use super::presentation::PresentationFragmentHandler;
use super::slide::{SlidePersist, SlideSize};

#[derive(Debug)]
pub(crate) struct PowerPointImport {
  pub(crate) themes: Vec<ThemeFragmentRecord>,
  pub(crate) draw_pages: Vec<SlidePersist>,
  pub(crate) master_pages: Vec<SlidePersist>,
  pub(crate) notes_pages: Vec<SlidePersist>,
  pub(crate) is_endless: bool,
  pub(crate) is_automatic: bool,
  pub(crate) first_page_name: Option<String>,
  pub(crate) custom_show_name: Option<String>,
  pub(crate) embed_true_type_fonts: bool,
  pub(crate) save_subset_fonts: bool,
  pub(crate) embedded_font_typefaces: Vec<String>,
  pub(crate) actual_slide_persist: Option<usize>,
  pub(crate) actual_slide_persist_context: Option<Box<SlidePersist>>,
  pub(crate) table_style_list_path: Option<String>,
  pub(crate) table_style_list: Option<TableStyleList>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ThemeFragmentRecord {
  pub(crate) path: String,
  pub(crate) name: Option<String>,
  pub(crate) theme_id: Option<String>,
  pub(crate) color_scheme: ThemeColorScheme,
  pub(crate) font_scheme: ThemeFontScheme,
  pub(crate) format_scheme: ThemeFormatScheme,
}

impl PowerPointImport {
  pub(crate) fn import_document(package: &mut PresentationDocument) -> Result<Self> {
    // PowerPointImport::importDocument resolves the officeDocument part,
    // imports the presentation fragment, then imports presProps when present.
    let presentation_part = package.presentation_part()?;
    let mut import = Self {
      themes: Vec::new(),
      draw_pages: Vec::new(),
      master_pages: Vec::new(),
      notes_pages: Vec::new(),
      is_endless: false,
      is_automatic: false,
      first_page_name: None,
      custom_show_name: None,
      embed_true_type_fonts: false,
      save_subset_fonts: false,
      embedded_font_typefaces: Vec::new(),
      actual_slide_persist: None,
      actual_slide_persist_context: None,
      table_style_list_path: None,
      table_style_list: None,
    };

    let mut handler = PresentationFragmentHandler::new(package, presentation_part.clone())?;
    handler.finalize_import(package, &mut import)?;
    import.load_table_style_list(package, &presentation_part)?;
    Ok(import)
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
    color_scheme: ThemeColorScheme,
    font_scheme: ThemeFontScheme,
    format_scheme: ThemeFormatScheme,
  ) -> &ThemeFragmentRecord {
    if let Some(index) = self.themes.iter().position(|theme| theme.path == path) {
      return &self.themes[index];
    }
    self.themes.push(ThemeFragmentRecord {
      path,
      name,
      theme_id,
      color_scheme,
      font_scheme,
      format_scheme,
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

  pub(crate) fn get_scheme_color_token_for_slide(
    &self,
    slide: &SlidePersist,
    token: a::SchemeColorValues,
  ) -> Option<a::ColorSchemeIndexValues> {
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
    self.get_scheme_color_token(token)
  }

  pub(crate) fn get_scheme_color_record(&self, token: a::SchemeColorValues) -> Option<&Color> {
    let mapped_token = self.get_scheme_color_token(token)?;
    self
      .get_current_theme_ptr()
      .and_then(|theme| theme.color_scheme.get_color(mapped_token))
  }

  pub(crate) fn get_scheme_color_record_for_slide(
    &self,
    slide: &SlidePersist,
    token: a::SchemeColorValues,
  ) -> Option<&Color> {
    let mapped_token = self.get_scheme_color_token_for_slide(slide, token)?;
    slide
      .theme_path
      .as_deref()
      .and_then(|path| self.get_theme(path))
      .or_else(|| self.get_current_theme_ptr())
      .and_then(|theme| theme.color_scheme.get_color(mapped_token))
  }

  pub(crate) fn resolve_color(
    &self,
    color: &Color,
    placeholder_color: Option<&Color>,
  ) -> Option<super::drawingml::color::ResolvedColor> {
    let mut scheme_resolver = |token| self.get_scheme_color_record(token).cloned();
    color.resolve_rgb(&mut scheme_resolver, placeholder_color)
  }

  pub(crate) fn resolve_color_for_slide(
    &self,
    slide: &SlidePersist,
    color: &Color,
    placeholder_color: Option<&Color>,
  ) -> Option<super::drawingml::color::ResolvedColor> {
    let mut scheme_resolver = |token| {
      self
        .get_scheme_color_record_for_slide(slide, token)
        .cloned()
    };
    color.resolve_rgb(&mut scheme_resolver, placeholder_color)
  }

  pub(crate) fn get_theme_fill_style(&self, index: u32) -> Option<FillProperties> {
    self
      .get_current_theme_ptr()
      .and_then(|theme| theme.format_scheme.get_fill_style(index))
      .cloned()
  }

  pub(crate) fn get_theme_line_style(&self, index: u32) -> Option<LineProperties> {
    self
      .get_current_theme_ptr()
      .and_then(|theme| theme.format_scheme.get_line_style(index))
      .cloned()
  }

  pub(crate) fn get_theme_effect_style(&self, index: u32) -> Option<EffectProperties> {
    self
      .get_current_theme_ptr()
      .and_then(|theme| theme.format_scheme.get_effect_style(index))
      .cloned()
  }

  pub(crate) fn get_theme_latin_font(&self, index: a::FontCollectionIndexValues) -> Option<&str> {
    self
      .get_current_theme_ptr()
      .and_then(|theme| theme.font_scheme.latin_font(index))
  }

  pub(crate) fn get_table_styles(&self) -> Option<&TableStyleList> {
    self.table_style_list.as_ref()
  }

  pub(crate) fn get_table_style(&self, style_id: Option<&str>) -> Option<&TableStyle> {
    self.get_table_styles()?.style(style_id)
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

  fn load_table_style_list(
    &mut self,
    package: &mut PresentationDocument,
    presentation_part: &PresentationPart,
  ) -> Result<()> {
    // tableStyles relationship from the office document. The Rust importer
    // materializes it once after the presentation traversal so display/model
    // users can resolve table style IDs without reparsing package state.
    let Some(table_styles_part) = presentation_part.table_styles_part(package) else {
      return Ok(());
    };
    let path = table_styles_part.path(package).map(str::to_string);
    let table_style_list = table_styles_part.root_element(package)?;
    self.table_style_list_path = path.clone();
    self.table_style_list = Some(TableStyleList::from_dml(path, table_style_list));
    Ok(())
  }
}
