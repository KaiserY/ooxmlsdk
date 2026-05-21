use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use crate::docx::PageSetup;
use crate::units;

use super::drawingml::color::Color;
use super::drawingml::fill::FillProperties;
use super::drawingml::shape::Shape;
use super::import::PowerPointImport;

// Source: LibreOffice sd/source/filter/eppt/pptx-epptbase.cxx falls back to
// a 28000 x 21000 mm100 master page when exporting a presentation with no page
// property value. Kept here until the full sd import defaults are ported.
const LO_DEFAULT_SLIDE_WIDTH_MM100: f32 = 28_000.0;
const LO_DEFAULT_SLIDE_HEIGHT_MM100: f32 = 21_000.0;
const DEFAULT_PRESENTATION_MARGIN_PT: f32 = 0.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SlideSize {
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
}

impl SlideSize {
  pub(crate) fn from_pml(size: &p::SlideSize) -> Self {
    Self {
      width_pt: units::emu_to_points(i64::from(size.cx)),
      height_pt: units::emu_to_points(i64::from(size.cy)),
    }
  }

  pub(crate) fn from_notes(size: &p::NotesSize) -> Self {
    Self {
      width_pt: units::emu_to_points(i64::from(size.cx)),
      height_pt: units::emu_to_points(i64::from(size.cy)),
    }
  }

  pub(crate) fn libreoffice_default() -> Self {
    Self {
      width_pt: units::millimeters_to_points(
        LO_DEFAULT_SLIDE_WIDTH_MM100 / units::MM100_PER_MILLIMETER,
      ),
      height_pt: units::millimeters_to_points(
        LO_DEFAULT_SLIDE_HEIGHT_MM100 / units::MM100_PER_MILLIMETER,
      ),
    }
  }

  pub(crate) fn to_page_setup(self) -> PageSetup {
    PageSetup {
      width_pt: self.width_pt,
      height_pt: self.height_pt,
      margin_top_pt: DEFAULT_PRESENTATION_MARGIN_PT,
      margin_right_pt: DEFAULT_PRESENTATION_MARGIN_PT,
      margin_bottom_pt: DEFAULT_PRESENTATION_MARGIN_PT,
      margin_left_pt: DEFAULT_PRESENTATION_MARGIN_PT,
      ..PageSetup::default()
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ShapeLocation {
  Master,
  Layout,
  Slide,
}

#[derive(Clone, Debug)]
pub(crate) struct SlidePersist {
  pub(crate) path: String,
  pub(crate) relationship_id: Option<String>,
  pub(crate) layout_path: Option<String>,
  pub(crate) master_path: Option<String>,
  pub(crate) size: SlideSize,
  pub(crate) theme_path: Option<String>,
  pub(crate) color_map: Option<ColorMap>,
  pub(crate) master_color_map: Option<ColorMap>,
  pub(crate) master_page_index: Option<usize>,
  pub(crate) shapes: Vec<Shape>,
  pub(crate) background_color: Option<Color>,
  pub(crate) background_properties: Option<BackgroundProperties>,
  pub(crate) time_node_list: Vec<TimeNode>,
  pub(crate) header_footer: HeaderFooter,
  pub(crate) layout_value_token: Option<String>,
  pub(crate) is_master: bool,
  pub(crate) is_notes: bool,
  pub(crate) comments: Vec<SlideComment>,
  pub(crate) comment_authors: Vec<SlideCommentAuthor>,
  pub(crate) drawing: VmlDrawing,
  pub(crate) shape_location: ShapeLocation,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ColorMap {
  pub(crate) entries: Vec<ColorMapEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ColorMapEntry {
  pub(crate) source: a::SchemeColorValues,
  pub(crate) target: a::ColorSchemeIndexValues,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BackgroundProperties {
  pub(crate) kind: BackgroundKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BackgroundKind {
  Properties(FillProperties),
  StyleReference {
    style_index: u32,
    placeholder_color: Option<Color>,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TimeNode;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct HeaderFooter {
  pub(crate) visible: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SlideComment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SlideCommentAuthor;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlDrawing {
  pub(crate) imported: bool,
}

impl SlidePersist {
  pub(crate) fn new_slide(path: String, relationship_id: String, size: SlideSize) -> Self {
    Self::new(
      path,
      Some(relationship_id),
      size,
      false,
      false,
      ShapeLocation::Slide,
    )
  }

  pub(crate) fn new_master(path: String, size: SlideSize) -> Self {
    Self::new(path, None, size, true, false, ShapeLocation::Master)
  }

  pub(crate) fn new_layout(path: String, size: SlideSize) -> Self {
    Self::new(path, None, size, false, false, ShapeLocation::Layout)
  }

  fn new(
    path: String,
    relationship_id: Option<String>,
    size: SlideSize,
    is_master: bool,
    is_notes: bool,
    shape_location: ShapeLocation,
  ) -> Self {
    Self {
      path,
      relationship_id,
      layout_path: None,
      master_path: None,
      size,
      theme_path: None,
      color_map: None,
      master_color_map: None,
      master_page_index: None,
      shapes: Vec::new(),
      background_color: None,
      background_properties: None,
      time_node_list: Vec::new(),
      header_footer: HeaderFooter::default(),
      layout_value_token: None,
      is_master,
      is_notes,
      comments: Vec::new(),
      comment_authors: Vec::new(),
      drawing: VmlDrawing::default(),
      shape_location,
    }
  }

  pub(crate) fn get_layout_from_value_token(&self) -> Option<String> {
    self.layout_value_token.clone()
  }

  pub(crate) fn set_color_map(&mut self, color_map: ColorMap) {
    self.color_map = Some(color_map);
  }

  pub(crate) fn apply_color_map_override(&mut self, override_map: &p::ColorMapOverride) {
    match override_map.color_map_override_choice.as_ref() {
      Some(p::ColorMapOverrideChoice::MasterColorMapping) => {}
      Some(p::ColorMapOverrideChoice::OverrideColorMapping(mapping)) => {
        self.color_map = Some(ColorMap::from_dml_override(mapping));
      }
      None => {}
    }
  }

  pub(crate) fn hide_shapes_as_master_shapes(&mut self) {
    for shape in &mut self.shapes {
      shape.hide_as_master_shape();
    }
  }

  pub(crate) fn create_background(&mut self, _import: &PowerPointImport) {
    // Source: LibreOffice oox/source/ppt/slidepersist.cxx
    // createBackground pushes resolved bg/bgPr/bgRef state to the page.
  }

  pub(crate) fn create_x_shapes(&mut self, import: &PowerPointImport) {
    // Source: LibreOffice oox/source/ppt/slidepersist.cxx
    // createXShapes applies text styles, creates shapes, then resolves
    // connector maps. Rust keeps a drawing model instead of UNO XShapes.
    self.apply_text_styles(import);
    for shape in &mut self.shapes {
      shape.create_and_insert(import);
    }
    self.create_connector_shape_connection();
  }

  pub(crate) fn apply_text_styles(&mut self, _import: &PowerPointImport) {}

  pub(crate) fn create_connector_shape_connection(&mut self) {}
}

impl ColorMap {
  pub(crate) fn map_token(&self, token: a::SchemeColorValues) -> Option<a::ColorSchemeIndexValues> {
    self
      .entries
      .iter()
      .find(|entry| entry.source == token)
      .map(|entry| entry.target)
  }

  pub(crate) fn from_pml(color_map: &p::ColorMap) -> Self {
    Self {
      entries: vec![
        ColorMapEntry::new(a::SchemeColorValues::Background1, color_map.background1),
        ColorMapEntry::new(a::SchemeColorValues::Text1, color_map.text1),
        ColorMapEntry::new(a::SchemeColorValues::Background2, color_map.background2),
        ColorMapEntry::new(a::SchemeColorValues::Text2, color_map.text2),
        ColorMapEntry::new(a::SchemeColorValues::Accent1, color_map.accent1),
        ColorMapEntry::new(a::SchemeColorValues::Accent2, color_map.accent2),
        ColorMapEntry::new(a::SchemeColorValues::Accent3, color_map.accent3),
        ColorMapEntry::new(a::SchemeColorValues::Accent4, color_map.accent4),
        ColorMapEntry::new(a::SchemeColorValues::Accent5, color_map.accent5),
        ColorMapEntry::new(a::SchemeColorValues::Accent6, color_map.accent6),
        ColorMapEntry::new(a::SchemeColorValues::Hyperlink, color_map.hyperlink),
        ColorMapEntry::new(
          a::SchemeColorValues::FollowedHyperlink,
          color_map.followed_hyperlink,
        ),
      ],
    }
  }

  pub(crate) fn from_dml_override(color_map: &a::OverrideColorMapping) -> Self {
    Self {
      entries: vec![
        ColorMapEntry::new(a::SchemeColorValues::Background1, color_map.background1),
        ColorMapEntry::new(a::SchemeColorValues::Text1, color_map.text1),
        ColorMapEntry::new(a::SchemeColorValues::Background2, color_map.background2),
        ColorMapEntry::new(a::SchemeColorValues::Text2, color_map.text2),
        ColorMapEntry::new(a::SchemeColorValues::Accent1, color_map.accent1),
        ColorMapEntry::new(a::SchemeColorValues::Accent2, color_map.accent2),
        ColorMapEntry::new(a::SchemeColorValues::Accent3, color_map.accent3),
        ColorMapEntry::new(a::SchemeColorValues::Accent4, color_map.accent4),
        ColorMapEntry::new(a::SchemeColorValues::Accent5, color_map.accent5),
        ColorMapEntry::new(a::SchemeColorValues::Accent6, color_map.accent6),
        ColorMapEntry::new(a::SchemeColorValues::Hyperlink, color_map.hyperlink),
        ColorMapEntry::new(
          a::SchemeColorValues::FollowedHyperlink,
          color_map.followed_hyperlink,
        ),
      ],
    }
  }
}

impl ColorMapEntry {
  fn new(source: a::SchemeColorValues, target: a::ColorSchemeIndexValues) -> Self {
    Self { source, target }
  }
}
