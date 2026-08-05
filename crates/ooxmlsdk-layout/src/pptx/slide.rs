use std::collections::HashMap;
use std::sync::Arc;

use ooxmlsdk::common::{MediaDataPart, RelationshipRef};
use ooxmlsdk::parts::{
  chart_color_style_part::ChartColorStylePart, chart_drawing_part::ChartDrawingPart,
  chart_part::ChartPart, chart_style_part::ChartStylePart, diagram_colors_part::DiagramColorsPart,
  diagram_data_part::DiagramDataPart, diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  diagram_persist_layout_part::DiagramPersistLayoutPart, diagram_style_part::DiagramStylePart,
  embedded_object_part::EmbeddedObjectPart, embedded_package_part::EmbeddedPackagePart,
  extended_chart_part::ExtendedChartPart, image_part::ImagePart,
  presentation_document::PresentationDocument, slide_part::SlidePart,
  theme_override_part::ThemeOverridePart, vml_drawing_part::VmlDrawingPart,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2008_diagram as dsp,
  schemas_microsoft_com_office_drawing_2012_chart_style as cs,
  schemas_microsoft_com_office_drawing_2014_chartex as cx,
  schemas_microsoft_com_office_powerpoint_2018_8_main as p188,
  schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};
use ooxmlsdk::sdk::SdkPart;
use quick_xml::events::{BytesStart, Event};

use crate::docx::{ImageCrop, PageSetup};
use crate::error::Result;
use crate::render::math::text_math_text;
use crate::units;

use super::activex::ActiveXControlState;
use super::drawingml::color::{Color, RgbHexColor};
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::line::{LineFill, LineProperties};
use super::drawingml::shape::{LegacyVmlFillImage, Shape, ShapeMapEntry};
use super::drawingml::text_body::{TextBody, TextParagraph, TextRun, TextRunKind};
use super::drawingml::text_list_style::TextListStyle;
use super::import::PowerPointImport;
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
      width_pt: units::emu_to_points(size.cx),
      height_pt: units::emu_to_points(size.cy),
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
  pub(crate) layout_path: Option<String>,
  pub(crate) master_path: Option<String>,
  pub(crate) size: SlideSize,
  pub(crate) theme_path: Option<String>,
  pub(crate) theme_text_body_defaults: Option<TextBody>,
  pub(crate) color_map: Option<ColorMap>,
  pub(crate) master_color_map: Option<ColorMap>,
  pub(crate) master_page_index: Option<usize>,
  pub(crate) name: Option<String>,
  pub(crate) visible: bool,
  pub(crate) show_master_shapes: bool,
  pub(crate) shapes: Vec<Shape>,
  pub(crate) background_color: Option<Color>,
  pub(crate) background_properties: Option<BackgroundProperties>,
  pub(crate) default_text_style: Option<TextListStyle>,
  pub(crate) title_text_style: Option<TextListStyle>,
  pub(crate) body_text_style: Option<TextListStyle>,
  pub(crate) notes_text_style: Option<TextListStyle>,
  pub(crate) other_text_style: Option<TextListStyle>,
  pub(crate) header_footer: HeaderFooter,
  pub(crate) is_master: bool,
  pub(crate) is_notes: bool,
  pub(crate) comments: Vec<SlideComment>,
  pub(crate) comment_authors: Vec<SlideCommentAuthor>,
  pub(crate) drawing: VmlDrawing,
  pub(crate) shape_map: Vec<ShapeMapEntry>,
  pub(crate) connector_shape_map: Vec<ShapeMapEntry>,
  pub(crate) connector_connections_applied: bool,
  pub(crate) shape_location: ShapeLocation,
  pub(crate) image_resources: HashMap<String, ImageResource>,
  pub(crate) chart_resources: HashMap<String, ChartResource>,
  pub(crate) extended_chart_resources: HashMap<String, ExtendedChartResource>,
  pub(crate) diagram_data_resources: HashMap<String, DiagramDataResource>,
  pub(crate) diagram_layout_resources: HashMap<String, DiagramLayoutResource>,
  pub(crate) diagram_style_resources: HashMap<String, DiagramStyleResource>,
  pub(crate) diagram_color_resources: HashMap<String, DiagramColorResource>,
  pub(crate) diagram_drawing_resources: HashMap<String, DiagramDrawingResource>,
  pub(crate) embedded_object_resources: HashMap<String, BinaryResource>,
  pub(crate) embedded_package_resources: HashMap<String, BinaryResource>,
  pub(crate) media_resources: HashMap<String, MediaResource>,
  pub(crate) hyperlink_targets: HashMap<String, String>,
  pub(crate) active_x_controls: HashMap<String, ActiveXControlState>,
  pub(crate) active_x_controls_by_shape: HashMap<String, ActiveXControlRecord>,
  pub(crate) active_x_preview_shapes_by_relationship: HashMap<String, usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImageResource {
  pub(crate) data: Arc<[u8]>,
  pub(crate) content_type: Option<String>,
  pub(crate) monochrome_dib_palette_override: Option<[[u8; 3]; 2]>,
  pub(crate) metafile_external_header: Option<crate::render::emf_wmf::WmfExternalHeader>,
  pub(crate) metafile_semantic_text_includes_raster_backdrop: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ActiveXFallbackPreview {
  control_relationship_id: String,
  image_relationship_id: String,
  shape_id: Option<String>,
  name: Option<String>,
  show_as_icon: Option<bool>,
  image_width: Option<i32>,
  image_height: Option<i32>,
  position: super::drawingml::shape::Point,
  size: super::drawingml::shape::Size,
}

/// The selected `p:control`, kept intact until its VML host shape is imported.
///
/// In particular, `spid` identifies the VML shape while `imgW`/`imgH` describe
/// the thumbnail. Keeping the typed node also preserves `showAsIcon`, `extLst`,
/// and an inline `p:pic` instead of silently collapsing those representations.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ActiveXControlRecord {
  pub(crate) control: p::Control,
  pub(crate) state: Option<ActiveXControlState>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChartResource {
  pub(crate) path: Option<String>,
  pub(crate) chart_space: c::ChartSpace,
  pub(crate) drawing: Option<ChartDrawingResource>,
  pub(crate) embedded_package: Option<BinaryResource>,
  pub(crate) image_resources: HashMap<String, ImageResource>,
  pub(crate) theme_override: Option<ThemeOverrideResource>,
  pub(crate) style_resources: Vec<ChartStyleResource>,
  pub(crate) color_style_resources: Vec<ChartColorStyleResource>,
}

impl ChartResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.chart_space)
      || self
        .drawing
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || self
        .embedded_package
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || !self.image_resources.is_empty()
      || self
        .theme_override
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || self
        .style_resources
        .iter()
        .any(ChartStyleResource::has_payload)
      || self
        .color_style_resources
        .iter()
        .any(ChartColorStyleResource::has_payload)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExtendedChartResource {
  pub(crate) path: Option<String>,
  pub(crate) chart_space: cx::ChartSpace,
  pub(crate) drawing: Option<ChartDrawingResource>,
  pub(crate) embedded_package: Option<BinaryResource>,
  pub(crate) image_resources: HashMap<String, ImageResource>,
  pub(crate) theme_override: Option<ThemeOverrideResource>,
  pub(crate) style_resources: Vec<ChartStyleResource>,
  pub(crate) color_style_resources: Vec<ChartColorStyleResource>,
}

impl ExtendedChartResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.chart_space)
      || self
        .drawing
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || self
        .embedded_package
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || !self.image_resources.is_empty()
      || self
        .theme_override
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
      || self
        .style_resources
        .iter()
        .any(ChartStyleResource::has_payload)
      || self
        .color_style_resources
        .iter()
        .any(ChartColorStyleResource::has_payload)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChartDrawingResource {
  pub(crate) path: Option<String>,
  pub(crate) user_shapes: c::UserShapes,
  pub(crate) image_resources: HashMap<String, ImageResource>,
}

impl ChartDrawingResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.user_shapes)
      || !self.image_resources.is_empty()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ThemeOverrideResource {
  pub(crate) path: Option<String>,
  pub(crate) theme_override: a::ThemeOverride,
  pub(crate) image_resources: HashMap<String, ImageResource>,
}

impl ThemeOverrideResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.theme_override)
      || !self.image_resources.is_empty()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChartStyleResource {
  pub(crate) path: Option<String>,
  pub(crate) style: cs::ChartStyle,
}

impl ChartStyleResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.style)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ChartColorStyleResource {
  pub(crate) path: Option<String>,
  pub(crate) colors: cs::ColorStyle,
}

impl ChartColorStyleResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.colors)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DiagramDataResource {
  pub(crate) path: Option<String>,
  pub(crate) model: dgm::DataModelRoot,
  pub(crate) image_resources: HashMap<String, ImageResource>,
}

impl DiagramDataResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.model)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DiagramLayoutResource {
  pub(crate) path: Option<String>,
  pub(crate) layout: dgm::LayoutDefinition,
}

impl DiagramLayoutResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.layout)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DiagramStyleResource {
  pub(crate) path: Option<String>,
  pub(crate) style: dgm::StyleDefinition,
}

impl DiagramStyleResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.style)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DiagramColorResource {
  pub(crate) path: Option<String>,
  pub(crate) colors: dgm::ColorsDefinition,
}

impl DiagramColorResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || structured_resource_present(&self.colors)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DiagramDrawingResource {
  pub(crate) path: Option<String>,
  pub(crate) drawing: dsp::Drawing,
  pub(crate) image_resources: HashMap<String, ImageResource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BinaryResource {
  pub(crate) path: Option<String>,
  pub(crate) data: Vec<u8>,
  pub(crate) content_type: Option<String>,
}

impl BinaryResource {
  pub(crate) fn has_payload(&self) -> bool {
    self.path.as_ref().is_some_and(|path| !path.is_empty())
      || !self.data.is_empty()
      || self
        .content_type
        .as_ref()
        .is_some_and(|kind| !kind.is_empty())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MediaResource {
  pub(crate) relationship_id: String,
  pub(crate) relationship_type: String,
  pub(crate) target: String,
  pub(crate) external: bool,
  pub(crate) data: Option<BinaryResource>,
}

impl MediaResource {
  pub(crate) fn has_payload(&self) -> bool {
    !self.relationship_id.is_empty()
      || !self.relationship_type.is_empty()
      || !self.target.is_empty()
      || self.external
      || self.data.as_ref().is_some_and(BinaryResource::has_payload)
  }
}

fn binary_resource<P>(package: &PresentationDocument, part: &P) -> Option<BinaryResource>
where
  P: SdkPart,
{
  Some(BinaryResource {
    path: part.path(package).map(str::to_string),
    data: part.data_to_vec(package)?,
    content_type: part.content_type(package).map(str::to_string),
  })
}

fn binary_resource_from_media_data_part(
  package: &PresentationDocument,
  part: &MediaDataPart,
) -> Option<BinaryResource> {
  Some(BinaryResource {
    path: part.path(package).map(str::to_string),
    data: part.data(package)?.to_vec(),
    content_type: part.content_type(package).map(str::to_string),
  })
}

fn media_data_part_by_id(
  package: &PresentationDocument,
  part_id: ooxmlsdk::common::PartId,
) -> Option<MediaDataPart> {
  package
    .media_data_parts()
    .find(|part| part.part_id() == Some(part_id))
}

fn media_resource_from_relationship(
  package: &PresentationDocument,
  relationship: RelationshipRef<'_>,
) -> MediaResource {
  let data = relationship
    .target_part_id()
    .and_then(|part_id| media_data_part_by_id(package, part_id))
    .and_then(|part| binary_resource_from_media_data_part(package, &part));
  MediaResource {
    relationship_id: relationship.id().to_string(),
    relationship_type: relationship.relationship_type().to_string(),
    target: relationship.target().to_string(),
    external: data.is_none(),
    data,
  }
}

fn collect_image_resources<P>(
  package: &PresentationDocument,
  part: &P,
) -> HashMap<String, ImageResource>
where
  P: SdkPart,
{
  part
    .related_parts_of_type::<_, ImagePart>(package)
    .filter_map(|related_part| {
      Some((
        related_part.relationship_id().to_string(),
        ImageResource {
          data: related_part.part().data_to_vec(package)?.into(),
          content_type: related_part
            .part()
            .content_type(package)
            .map(str::to_string),
          monochrome_dib_palette_override: None,
          metafile_external_header: None,
          metafile_semantic_text_includes_raster_backdrop: false,
        },
      ))
    })
    .collect()
}

fn chart_resource(
  package: &mut PresentationDocument,
  chart_part: &ChartPart,
) -> Result<ChartResource> {
  let drawing_part = chart_part.chart_drawing_part(package);
  let embedded_package_part = chart_part.embedded_package_part(package);
  let theme_override_part = chart_part.theme_override_part(package);
  let chart_style_parts: Vec<_> = chart_part.chart_style_parts(package).collect();
  let chart_color_style_parts: Vec<_> = chart_part.chart_color_style_parts(package).collect();
  let image_resources = collect_image_resources(package, chart_part);

  Ok(ChartResource {
    path: chart_part.path(package).map(str::to_string),
    chart_space: chart_part.root_element(package)?.clone(),
    drawing: drawing_part
      .as_ref()
      .map(|part| chart_drawing_resource(package, part))
      .transpose()?,
    embedded_package: embedded_package_part
      .as_ref()
      .and_then(|part| binary_resource(package, part)),
    image_resources,
    theme_override: theme_override_part
      .as_ref()
      .map(|part| theme_override_resource(package, part))
      .transpose()?,
    style_resources: chart_style_parts
      .iter()
      .map(|part| chart_style_resource(package, part))
      .collect::<Result<Vec<_>>>()?,
    color_style_resources: chart_color_style_parts
      .iter()
      .map(|part| chart_color_style_resource(package, part))
      .collect::<Result<Vec<_>>>()?,
  })
}

fn extended_chart_resource(
  package: &mut PresentationDocument,
  chart_part: &ExtendedChartPart,
) -> Result<ExtendedChartResource> {
  let drawing_part = chart_part.chart_drawing_part(package);
  let embedded_package_part = chart_part.embedded_package_part(package);
  let theme_override_part = chart_part.theme_override_part(package);
  let chart_style_parts: Vec<_> = chart_part.chart_style_parts(package).collect();
  let chart_color_style_parts: Vec<_> = chart_part.chart_color_style_parts(package).collect();
  let image_resources = collect_image_resources(package, chart_part);

  Ok(ExtendedChartResource {
    path: chart_part.path(package).map(str::to_string),
    chart_space: chart_part.root_element(package)?.clone(),
    drawing: drawing_part
      .as_ref()
      .map(|part| chart_drawing_resource(package, part))
      .transpose()?,
    embedded_package: embedded_package_part
      .as_ref()
      .and_then(|part| binary_resource(package, part)),
    image_resources,
    theme_override: theme_override_part
      .as_ref()
      .map(|part| theme_override_resource(package, part))
      .transpose()?,
    style_resources: chart_style_parts
      .iter()
      .map(|part| chart_style_resource(package, part))
      .collect::<Result<Vec<_>>>()?,
    color_style_resources: chart_color_style_parts
      .iter()
      .map(|part| chart_color_style_resource(package, part))
      .collect::<Result<Vec<_>>>()?,
  })
}

fn chart_drawing_resource(
  package: &mut PresentationDocument,
  part: &ChartDrawingPart,
) -> Result<ChartDrawingResource> {
  let image_resources = collect_image_resources(package, part);
  Ok(ChartDrawingResource {
    path: part.path(package).map(str::to_string),
    user_shapes: part.root_element(package)?.clone(),
    image_resources,
  })
}

fn theme_override_resource(
  package: &mut PresentationDocument,
  part: &ThemeOverridePart,
) -> Result<ThemeOverrideResource> {
  let image_resources = collect_image_resources(package, part);
  Ok(ThemeOverrideResource {
    path: part.path(package).map(str::to_string),
    theme_override: part.root_element(package)?.clone(),
    image_resources,
  })
}

fn chart_style_resource(
  package: &mut PresentationDocument,
  part: &ChartStylePart,
) -> Result<ChartStyleResource> {
  Ok(ChartStyleResource {
    path: part.path(package).map(str::to_string),
    style: part.root_element(package)?.clone(),
  })
}

fn chart_color_style_resource(
  package: &mut PresentationDocument,
  part: &ChartColorStylePart,
) -> Result<ChartColorStyleResource> {
  Ok(ChartColorStyleResource {
    path: part.path(package).map(str::to_string),
    colors: part.root_element(package)?.clone(),
  })
}

fn structured_resource_present<T>(_root: &T) -> bool {
  true
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BackgroundProperties {
  pub(crate) kind: BackgroundKind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum BackgroundKind {
  Properties(FillProperties),
  StyleReference {
    style_index: u32,
    placeholder_color: Option<Color>,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HeaderFooter {
  pub(crate) slide_number: bool,
  pub(crate) header: bool,
  pub(crate) footer: bool,
  pub(crate) date_time: bool,
}

impl Default for HeaderFooter {
  fn default() -> Self {
    // CT_HeaderFooter defaults every visibility flag to true. This is also
    // the effective default when the optional p:hf element is absent: an
    // existing slide-level placeholder remains enabled unless a declaration
    // explicitly disables its slot.
    Self {
      slide_number: true,
      header: true,
      footer: true,
      date_time: true,
    }
  }
}

impl HeaderFooter {
  pub(crate) fn from_pml(header_footer: &p::HeaderFooter) -> Self {
    // ECMA-376 Part 1 §19.3.1.25 assigns true to every omitted
    // CT_HeaderFooter visibility attribute.
    Self {
      slide_number: header_footer
        .slide_number
        .is_none_or(|value| value.as_bool()),
      header: header_footer.header.is_none_or(|value| value.as_bool()),
      footer: header_footer.footer.is_none_or(|value| value.as_bool()),
      date_time: header_footer.date_time.is_none_or(|value| value.as_bool()),
    }
  }

  pub(crate) fn has_visible_slot(&self) -> bool {
    self.slide_number || self.header || self.footer || self.date_time
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SlideCommentSource {
  Legacy,
  Modern,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SlideComment {
  pub(crate) source: SlideCommentSource,
  pub(crate) id: Option<String>,
  pub(crate) author_id: String,
  pub(crate) index: Option<u32>,
  pub(crate) x_emu: Option<i64>,
  pub(crate) y_emu: Option<i64>,
  pub(crate) text: Option<String>,
  pub(crate) date_time: Option<String>,
  pub(crate) status: Option<String>,
  pub(crate) tags: Vec<String>,
  pub(crate) likes: Vec<String>,
  pub(crate) assigned_to: Vec<String>,
  pub(crate) title: Option<String>,
  pub(crate) complete: Option<i32>,
  pub(crate) priority: Option<u32>,
  pub(crate) reply_count: usize,
  pub(crate) modern_text_body: Option<p188::TextBodyType>,
}

impl SlideComment {
  pub(crate) fn from_pml(comment: &p::Comment) -> Self {
    Self {
      source: SlideCommentSource::Legacy,
      id: None,
      author_id: comment.author_id.to_string(),
      index: Some(comment.index),
      x_emu: Some(comment.position.x),
      y_emu: Some(comment.position.y),
      text: Some(comment.text.clone()),
      date_time: comment.date_time.clone(),
      status: None,
      tags: Vec::new(),
      likes: Vec::new(),
      assigned_to: Vec::new(),
      title: None,
      complete: None,
      priority: None,
      reply_count: 0,
      modern_text_body: None,
    }
  }

  pub(crate) fn from_modern(comment: &p188::Comment) -> Self {
    let modern_text_body = comment
      .text_body_type
      .as_ref()
      .map(|text_body| (**text_body).clone());
    Self {
      source: SlideCommentSource::Modern,
      id: Some(comment.id.clone()),
      author_id: comment.author_id.clone(),
      index: None,
      x_emu: comment.point2_d_type.as_ref().map(|position| position.x),
      y_emu: comment.point2_d_type.as_ref().map(|position| position.y),
      text: modern_text_body
        .as_ref()
        .and_then(|text_body| text_from_dml_paragraphs(&text_body.paragraph)),
      date_time: Some(comment.created.clone()),
      status: comment.status.map(|status| format!("{status:?}")),
      tags: comment.tags.clone().unwrap_or_default(),
      likes: comment.likes.clone().unwrap_or_default(),
      assigned_to: comment.assigned_to.clone().unwrap_or_default(),
      title: comment.title.clone(),
      complete: comment.complete,
      priority: comment.priority,
      reply_count: comment
        .comment_reply_list
        .as_ref()
        .map(|reply_list| reply_list.comment_reply.len())
        .unwrap_or_default(),
      modern_text_body,
    }
  }

  pub(crate) fn has_payload(&self) -> bool {
    matches!(
      self.source,
      SlideCommentSource::Legacy | SlideCommentSource::Modern
    ) || self.id.as_ref().is_some_and(|id| !id.is_empty())
      || !self.author_id.is_empty()
      || self.index.is_some()
      || self.x_emu.is_some()
      || self.y_emu.is_some()
      || self.text.as_ref().is_some_and(|text| !text.is_empty())
      || self
        .date_time
        .as_ref()
        .is_some_and(|date_time| !date_time.is_empty())
      || self
        .status
        .as_ref()
        .is_some_and(|status| !status.is_empty())
      || !self.tags.is_empty()
      || !self.likes.is_empty()
      || !self.assigned_to.is_empty()
      || self.title.as_ref().is_some_and(|title| !title.is_empty())
      || self.complete.is_some()
      || self.priority.is_some()
      || self.reply_count > 0
      || self.modern_text_body.is_some()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SlideCommentAuthor {
  pub(crate) source: SlideCommentSource,
  pub(crate) id: String,
  pub(crate) name: String,
  pub(crate) initials: Option<String>,
  pub(crate) color_index: Option<u32>,
  pub(crate) last_index: Option<u32>,
  pub(crate) user_id: Option<String>,
  pub(crate) provider_id: Option<String>,
}

impl SlideCommentAuthor {
  pub(crate) fn from_pml(author: &p::CommentAuthor) -> Self {
    Self {
      source: SlideCommentSource::Legacy,
      id: author.id.to_string(),
      name: author.name.clone(),
      initials: Some(author.initials.clone()),
      color_index: Some(author.color_index),
      last_index: Some(author.last_index),
      user_id: None,
      provider_id: None,
    }
  }

  pub(crate) fn from_modern(author: &p188::Author) -> Self {
    Self {
      source: SlideCommentSource::Modern,
      id: author.id.clone(),
      name: author.name.clone(),
      initials: author.initials.clone(),
      color_index: None,
      last_index: None,
      user_id: Some(author.user_id.clone()),
      provider_id: Some(author.provider_id.clone()),
    }
  }

  pub(crate) fn has_payload(&self) -> bool {
    matches!(
      self.source,
      SlideCommentSource::Legacy | SlideCommentSource::Modern
    ) || !self.id.is_empty()
      || !self.name.is_empty()
      || self
        .initials
        .as_ref()
        .is_some_and(|initials| !initials.is_empty())
      || self.color_index.is_some()
      || self.last_index.is_some()
      || self
        .user_id
        .as_ref()
        .is_some_and(|user_id| !user_id.is_empty())
      || self
        .provider_id
        .as_ref()
        .is_some_and(|provider_id| !provider_id.is_empty())
  }
}

fn text_from_dml_paragraphs(paragraphs: &[a::Paragraph]) -> Option<String> {
  let mut text = String::new();
  for (index, paragraph) in paragraphs.iter().enumerate() {
    if index > 0 {
      text.push('\n');
    }
    for choice in &paragraph.paragraph_choice {
      match choice {
        a::ParagraphChoice::Run(run) => text.push_str(&run.text),
        a::ParagraphChoice::Break(_) => text.push('\n'),
        a::ParagraphChoice::Field(field) => {
          if let Some(field_text) = &field.text {
            text.push_str(field_text);
          }
        }
        a::ParagraphChoice::TextMath(math) => text.push_str(&text_math_text(math)),
        a::ParagraphChoice::AlternateContent(_) => {}
      }
    }
  }
  if text.is_empty() { None } else { Some(text) }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlDrawing {
  pub(crate) imported: bool,
  pub(crate) converted: bool,
}

impl VmlDrawing {
  pub(crate) fn convert_and_insert(&mut self) {
    // SlideFragmentHandler destruction converts and inserts VML controls.
    // Rust keeps the explicit lifecycle slot until VML drawing import is
    // ported beyond a structured fallback marker.
    if self.imported {
      self.converted = true;
    }
  }
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

  pub(crate) fn new_notes(path: String, relationship_id: Option<String>, size: SlideSize) -> Self {
    Self::new(
      path,
      relationship_id,
      size,
      false,
      true,
      ShapeLocation::Slide,
    )
  }

  pub(crate) fn new_notes_master(path: String, size: SlideSize) -> Self {
    Self::new(path, None, size, true, true, ShapeLocation::Master)
  }

  fn new(
    path: String,
    _relationship_id: Option<String>,
    size: SlideSize,
    is_master: bool,
    is_notes: bool,
    shape_location: ShapeLocation,
  ) -> Self {
    Self {
      path,
      layout_path: None,
      master_path: None,
      size,
      theme_path: None,
      theme_text_body_defaults: None,
      color_map: None,
      master_color_map: None,
      master_page_index: None,
      name: None,
      visible: true,
      show_master_shapes: true,
      shapes: Vec::new(),
      background_color: None,
      background_properties: None,
      default_text_style: None,
      title_text_style: None,
      body_text_style: None,
      notes_text_style: None,
      other_text_style: None,
      header_footer: HeaderFooter::default(),
      is_master,
      is_notes,
      comments: Vec::new(),
      comment_authors: Vec::new(),
      drawing: VmlDrawing::default(),
      shape_map: Vec::new(),
      connector_shape_map: Vec::new(),
      connector_connections_applied: false,
      shape_location,
      image_resources: HashMap::new(),
      chart_resources: HashMap::new(),
      extended_chart_resources: HashMap::new(),
      diagram_data_resources: HashMap::new(),
      diagram_layout_resources: HashMap::new(),
      diagram_style_resources: HashMap::new(),
      diagram_color_resources: HashMap::new(),
      diagram_drawing_resources: HashMap::new(),
      embedded_object_resources: HashMap::new(),
      embedded_package_resources: HashMap::new(),
      media_resources: HashMap::new(),
      hyperlink_targets: HashMap::new(),
      active_x_controls: HashMap::new(),
      active_x_controls_by_shape: HashMap::new(),
      active_x_preview_shapes_by_relationship: HashMap::new(),
    }
  }

  pub(crate) fn set_color_map(&mut self, color_map: ColorMap) {
    self.color_map = Some(color_map);
  }

  pub(crate) fn set_default_text_style(&mut self, style: Option<TextListStyle>) {
    self.default_text_style = style;
  }

  pub(crate) fn set_text_styles(&mut self, styles: &p::TextStyles) {
    self.title_text_style = styles
      .title_style
      .as_ref()
      .map(|style| TextListStyle::from_pml_title_style(style));
    self.body_text_style = styles
      .body_style
      .as_ref()
      .map(|style| TextListStyle::from_pml_body_style(style));
    self.other_text_style = styles
      .other_style
      .as_ref()
      .map(|style| TextListStyle::from_pml_other_style(style));
  }

  pub(crate) fn set_comment_authors(&mut self, authors: Vec<SlideCommentAuthor>) {
    self.comment_authors = authors;
  }

  pub(crate) fn import_legacy_comments(&mut self, comments: &p::CommentList) {
    self
      .comments
      .extend(comments.comment.iter().map(SlideComment::from_pml));
  }

  pub(crate) fn import_modern_comments(&mut self, comments: &p188::CommentList) {
    self
      .comments
      .extend(comments.comment.iter().map(SlideComment::from_modern));
  }

  pub(crate) fn import_image_parts<P>(&mut self, package: &PresentationDocument, part: &P)
  where
    P: SdkPart,
  {
    // embed IDs against the current fragment's relationships, so cache image
    // bytes on the owning slide/layout/master persist before display lowering.
    for related_part in part.related_parts_of_type::<_, ImagePart>(package) {
      let relationship_id = related_part.relationship_id().to_string();
      let image_part = related_part.part();
      let Some(data) = image_part.data_to_vec(package) else {
        continue;
      };
      self.image_resources.insert(
        relationship_id,
        ImageResource {
          data: data.into(),
          content_type: image_part.content_type(package).map(str::to_string),
          monochrome_dib_palette_override: None,
          metafile_external_header: None,
          metafile_semantic_text_includes_raster_backdrop: false,
        },
      );
    }
  }

  pub(crate) fn import_media_reference_parts<P>(&mut self, package: &PresentationDocument, part: &P)
  where
    P: SdkPart,
  {
    // resolves a:wavAudioFile, a:audioFile, a:videoFile, and p14:media
    // relationship IDs against the current fragment before shape finalization.
    for relationship in part.data_part_reference_relationships(package) {
      self.media_resources.insert(
        relationship.id().to_string(),
        media_resource_from_relationship(package, relationship),
      );
    }
    for relationship in part.external_relationships(package).filter(|relationship| {
      matches!(
        relationship.relationship_type(),
        RelationshipRef::AUDIO_REFERENCE_RELATIONSHIP_TYPE
          | RelationshipRef::MEDIA_REFERENCE_RELATIONSHIP_TYPE
          | RelationshipRef::VIDEO_REFERENCE_RELATIONSHIP_TYPE
      )
    }) {
      self.media_resources.insert(
        relationship.id().to_string(),
        media_resource_from_relationship(package, relationship),
      );
    }
  }

  pub(crate) fn import_hyperlink_reference_parts<P>(
    &mut self,
    package: &PresentationDocument,
    part: &P,
  ) where
    P: SdkPart,
  {
    // a:hlinkClick r:id through the current fragment relationships.
    for relationship in part.hyperlink_relationships(package) {
      self.hyperlink_targets.insert(
        relationship.id().to_string(),
        relationship.target().to_string(),
      );
    }
    for related_slide in part.related_parts_of_type::<_, SlidePart>(package) {
      if let Some(target) = related_slide.part().path(package).and_then(slide_anchor) {
        self
          .hyperlink_targets
          .insert(related_slide.relationship_id().to_string(), target);
      }
    }
  }

  pub(crate) fn import_graphic_frame_related_parts<P>(
    &mut self,
    package: &mut PresentationDocument,
    part: &P,
  ) -> Result<()>
  where
    P: SdkPart,
  {
    // oox/source/ppt/slidefragmenthandler.cxx resolve graphicFrame targets
    // against the owning fragment. Cache targets here before inherited
    // master/layout shapes are cloned into another relationship scope.
    let chart_parts: Vec<_> = part
      .related_parts_of_type::<_, ChartPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, chart_part) in chart_parts {
      self
        .chart_resources
        .insert(relationship_id, chart_resource(package, &chart_part)?);
    }
    let extended_chart_parts: Vec<_> = part
      .related_parts_of_type::<_, ExtendedChartPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, chart_part) in extended_chart_parts {
      self.extended_chart_resources.insert(
        relationship_id,
        extended_chart_resource(package, &chart_part)?,
      );
    }
    let diagram_data_parts: Vec<_> = part
      .related_parts_of_type::<_, DiagramDataPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, diagram_part) in diagram_data_parts {
      self.import_image_parts(package, &diagram_part);
      self.diagram_data_resources.insert(
        relationship_id,
        DiagramDataResource {
          path: diagram_part.path(package).map(str::to_string),
          model: diagram_part.root_element(package)?.clone(),
          image_resources: collect_image_resources(package, &diagram_part),
        },
      );
    }
    let diagram_layout_parts: Vec<_> = part
      .related_parts_of_type::<_, DiagramLayoutDefinitionPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, diagram_part) in diagram_layout_parts {
      self.diagram_layout_resources.insert(
        relationship_id,
        DiagramLayoutResource {
          path: diagram_part.path(package).map(str::to_string),
          layout: diagram_part.root_element(package)?.clone(),
        },
      );
    }
    let diagram_style_parts: Vec<_> = part
      .related_parts_of_type::<_, DiagramStylePart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, diagram_part) in diagram_style_parts {
      self.diagram_style_resources.insert(
        relationship_id,
        DiagramStyleResource {
          path: diagram_part.path(package).map(str::to_string),
          style: diagram_part.root_element(package)?.clone(),
        },
      );
    }
    let diagram_color_parts: Vec<_> = part
      .related_parts_of_type::<_, DiagramColorsPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, diagram_part) in diagram_color_parts {
      self.diagram_color_resources.insert(
        relationship_id,
        DiagramColorResource {
          path: diagram_part.path(package).map(str::to_string),
          colors: diagram_part.root_element(package)?.clone(),
        },
      );
    }
    let diagram_drawing_parts: Vec<_> = part
      .related_parts_of_type::<_, DiagramPersistLayoutPart>(package)
      .map(|related_part| {
        (
          related_part.relationship_id().to_string(),
          related_part.part().clone(),
        )
      })
      .collect();
    for (relationship_id, diagram_part) in diagram_drawing_parts {
      self.diagram_drawing_resources.insert(
        relationship_id,
        DiagramDrawingResource {
          path: diagram_part.path(package).map(str::to_string),
          drawing: diagram_part.root_element(package)?.clone(),
          image_resources: collect_image_resources(package, &diagram_part),
        },
      );
    }
    for related_part in part.related_parts_of_type::<_, EmbeddedObjectPart>(package) {
      let relationship_id = related_part.relationship_id().to_string();
      if let Some(resource) = binary_resource(package, related_part.part()) {
        self
          .embedded_object_resources
          .insert(relationship_id, resource);
      }
    }
    for related_part in part.related_parts_of_type::<_, EmbeddedPackagePart>(package) {
      let relationship_id = related_part.relationship_id().to_string();
      if let Some(resource) = binary_resource(package, related_part.part()) {
        self
          .embedded_package_resources
          .insert(relationship_id, resource);
      }
    }
    Ok(())
  }

  pub(crate) fn inherit_related_part_resources_from(&mut self, reference: &SlidePersist) {
    self.image_resources = reference.image_resources.clone();
    self.chart_resources = reference.chart_resources.clone();
    self.extended_chart_resources = reference.extended_chart_resources.clone();
    self.diagram_data_resources = reference.diagram_data_resources.clone();
    self.diagram_layout_resources = reference.diagram_layout_resources.clone();
    self.diagram_style_resources = reference.diagram_style_resources.clone();
    self.diagram_color_resources = reference.diagram_color_resources.clone();
    self.diagram_drawing_resources = reference.diagram_drawing_resources.clone();
    self.embedded_object_resources = reference.embedded_object_resources.clone();
    self.embedded_package_resources = reference.embedded_package_resources.clone();
    self.media_resources = reference.media_resources.clone();
    self.hyperlink_targets = reference.hyperlink_targets.clone();
    self.comment_authors = reference.comment_authors.clone();
  }

  pub(crate) fn import_vml_preview_drawings(
    &mut self,
    package: &PresentationDocument,
    drawing_parts: &[VmlDrawingPart],
  ) {
    for drawing_part in drawing_parts {
      let image_resources = collect_image_resources(package, drawing_part);
      let models = drawing_part
        .data_to_vec(package)
        .map(|data| crate::xlsx::object_resources::vml_shapes(&data))
        .unwrap_or_default();
      for model in models {
        if model.hidden || !model.print_object {
          continue;
        }
        if model.image_relationship_id.is_none() {
          let Some((left_pt, top_pt, width_pt, height_pt)) =
            vml_absolute_rectangle(model.style.as_deref())
          else {
            continue;
          };
          let Some(paths) = crate::xlsx::vml_shape_drawing_paths(&model, width_pt, height_pt)
          else {
            continue;
          };
          let mut shape = Shape::new(super::drawingml::shape::ShapeService::Custom);
          shape.shape_location = Some(self.shape_location);
          shape.position = super::drawingml::shape::Point {
            x: points_to_emu(left_pt),
            y: points_to_emu(top_pt),
          };
          shape.size = super::drawingml::shape::Size {
            cx: points_to_emu(width_pt),
            cy: points_to_emu(height_pt),
          };
          shape.rotation = model
            .style
            .as_deref()
            .and_then(|style| vml_style_value(style, "rotation"))
            .map(vml_rotation_degrees)
            .unwrap_or(0.0);
          let flip = model
            .style
            .as_deref()
            .and_then(|style| vml_style_value(style, "flip"))
            .unwrap_or_default();
          shape.flip_h = flip
            .split_whitespace()
            .any(|value| value.eq_ignore_ascii_case("x"));
          shape.flip_v = flip
            .split_whitespace()
            .any(|value| value.eq_ignore_ascii_case("y"));
          shape.legacy_vml_paths = Some(paths);
          shape.legacy_vml_fill = Some(crate::xlsx::vml_shape_common_fill(
            &model,
            kurbo::Affine::IDENTITY,
          ));
          shape.legacy_vml_stroke = crate::xlsx::vml_shape_common_stroke(&model);
          shape.legacy_vml_fill_image = model
            .fill_image_relationship_id
            .as_deref()
            .and_then(|relationship_id| image_resources.get(relationship_id))
            .cloned()
            .map(|mut resource| {
              if let Some(data) =
                crate::xlsx::recolor_vml_pattern_image(&model, resource.data.as_ref())
              {
                resource.data = Arc::from(data);
                resource.content_type = Some("image/png".to_string());
                resource.monochrome_dib_palette_override = None;
              }
              LegacyVmlFillImage {
                resource,
                fill_type: model.fill_type,
                aspect: model.fill_image_aspect,
                size: model.fill_image_size.clone(),
                origin: model.fill_image_origin.clone(),
                position: model.fill_image_position.clone(),
                rotate_with_shape: model.fill_rotate_with_shape == Some(true),
              }
            });
          if shape.legacy_vml_fill_image.is_some() {
            shape.legacy_vml_fill = Some(crate::common::Fill::None);
          }
          shape.fill_properties = Some(FillProperties {
            kind: if model.filled {
              FillKind::Solid(Some(vml_preview_color(
                model.fill_color.as_deref(),
                (255, 255, 255),
              )))
            } else {
              FillKind::None
            },
            placeholder_color: None,
          });
          shape.line_properties = Some(LineProperties {
            fill: if model.stroked {
              LineFill::Solid(Some(vml_preview_color(
                model.stroke_color.as_deref(),
                (0, 0, 0),
              )))
            } else {
              LineFill::None
            },
            width_emu: model
              .stroke_weight
              .as_deref()
              .and_then(vml_measure_to_points)
              .map(points_to_emu),
            placeholder_color: None,
            source_outline: None,
          });
          if !model.text.is_empty() {
            shape.text_body = Some(TextBody {
              paragraphs: vec![TextParagraph {
                runs: vec![TextRun {
                  text: model.text,
                  kind: TextRunKind::Run,
                  hyperlink_url: None,
                  field_type: None,
                  run_properties: None,
                  field_paragraph_properties: None,
                }],
                ..TextParagraph::default()
              }],
              ..TextBody::default()
            });
          }
          self.shapes.push(shape);
          continue;
        }
        let Some(relationship_id) = model.image_relationship_id.as_deref() else {
          continue;
        };
        let Some(mut resource) = image_resources.get(relationship_id).cloned() else {
          continue;
        };
        let active_x_control =
          active_x_control_for_vml_model(&self.active_x_controls_by_shape, &model).cloned();
        let active_x_state = active_x_control
          .as_ref()
          .and_then(|record| record.state.as_ref());
        if let Some(palette) =
          active_x_state.and_then(ActiveXControlState::preview_palette_override)
        {
          // ActiveX previews use the persisted live control BackColor when
          // realizing a one-bit DIB pattern. Keep ordinary WMF/DIB palette
          // semantics untouched for every other image.
          resource.monochrome_dib_palette_override = Some(palette);
        }
        resource.metafile_external_header = active_x_control
          .as_ref()
          .and_then(|record| active_x_wmf_external_header(&record.control));
        // [MS-OI29500] defines p:control's image as the ActiveX thumbnail.
        // PowerPoint keeps text records in that native WMF preview searchable
        // even when an earlier icon bitmap uses a destination-reading ROP.
        // Ordinary OLE previews retain the raster-backdrop filter.
        resource.metafile_semantic_text_includes_raster_backdrop = active_x_control.is_some();
        if attach_vml_ole_preview(&mut self.shapes, &model, relationship_id, resource.clone()) {
          // ECMA-376 p:oleObj@spid associates this VML replacement graphic
          // with the already ordered DrawingML graphicFrame. It is not a
          // second shape at the end of the slide's z-order.
          continue;
        }
        let Some((left_pt, top_pt, width_pt, height_pt)) =
          vml_absolute_rectangle(model.style.as_deref())
        else {
          continue;
        };
        if active_x_control.as_ref().is_some_and(|control| {
          attach_vml_active_x_preview(
            &mut self.shapes,
            &self.active_x_preview_shapes_by_relationship,
            control,
            relationship_id,
            resource.clone(),
          )
        }) {
          // `p:control@spid` binds the selected control to this VML preview.
          // The fallback/inline picture keeps its authored rectangle. The
          // control's imgW/imgH remain available as external WMF playback
          // dimensions; they are not a replacement host transform.
          continue;
        }
        if self.shapes.iter().any(|shape| {
          shape
            .picture
            .as_ref()
            .and_then(|picture| picture.image_resource.as_ref())
            .is_some_and(|existing| {
              existing == &resource
                && vml_preview_rectangle_matches_shape(shape, left_pt, top_pt, width_pt, height_pt)
            })
        }) {
          // PowerPoint commonly retains the rounded VML compatibility
          // preview beside a more precise DrawingML p:pic fallback. They are
          // alternate representations of one OLE object, not two pictures.
          continue;
        }
        let mut shape = Shape::new(super::drawingml::shape::ShapeService::GraphicObject);
        shape.shape_location = Some(self.shape_location);
        shape.name = active_x_control
          .as_ref()
          .and_then(|record| record.control.name.clone());
        shape.position = super::drawingml::shape::Point {
          x: points_to_emu(left_pt),
          y: points_to_emu(top_pt),
        };
        shape.size = super::drawingml::shape::Size {
          cx: points_to_emu(width_pt),
          cy: points_to_emu(height_pt),
        };
        shape.set_picture(
          Some(relationship_id.to_string()),
          None,
          ImageCrop::default(),
          Vec::new(),
          Some(resource),
        );
        self.shapes.push(shape);
      }
    }
  }

  pub(crate) fn import_active_x_fallback_previews(&mut self, previews: &[ActiveXFallbackPreview]) {
    for preview in previews {
      let active_x_state = self
        .active_x_controls
        .get(&preview.control_relationship_id)
        .cloned();
      let fallback_record = ActiveXControlRecord {
        control: p::Control {
          shape_id: preview.shape_id.clone(),
          name: preview.name.clone(),
          show_as_icon: preview
            .show_as_icon
            .map(ooxmlsdk::simple_type::BooleanValue::from_bool),
          id: Some(preview.control_relationship_id.clone()),
          image_width: preview.image_width,
          image_height: preview.image_height,
          extension_list: None,
          picture: None,
        },
        state: active_x_state.clone(),
      };
      if let Some(name) = &preview.name {
        self
          .active_x_controls_by_shape
          .entry(name.clone())
          .or_insert_with(|| fallback_record.clone());
      }
      if let Some(shape_id) = &preview.shape_id {
        self
          .active_x_controls_by_shape
          .entry(normalize_vml_shape_id(shape_id))
          .or_insert_with(|| fallback_record.clone());
      }
      let Some(mut resource) = self
        .image_resources
        .get(&preview.image_relationship_id)
        .cloned()
      else {
        continue;
      };
      if let Some(palette) = active_x_state
        .as_ref()
        .and_then(ActiveXControlState::preview_palette_override)
      {
        resource.monochrome_dib_palette_override = Some(palette);
      }
      resource.metafile_external_header = active_x_wmf_external_header(&fallback_record.control);
      resource.metafile_semantic_text_includes_raster_backdrop = true;
      if let Some(shape_index) = self.shapes.iter().position(|shape| {
        shape.position == preview.position
          && shape.size == preview.size
          && shape
            .picture
            .as_ref()
            .and_then(|picture| picture.image_resource.as_ref())
            == Some(&resource)
      }) {
        self
          .active_x_preview_shapes_by_relationship
          .insert(preview.control_relationship_id.clone(), shape_index);
        continue;
      }

      let mut shape = Shape::new(super::drawingml::shape::ShapeService::GraphicObject);
      shape.shape_location = Some(self.shape_location);
      shape.name = preview.name.clone();
      shape.position = preview.position;
      shape.size = preview.size;
      shape.set_picture(
        Some(preview.image_relationship_id.clone()),
        None,
        ImageCrop::default(),
        Vec::new(),
        Some(resource),
      );
      let shape_index = self.shapes.len();
      self.shapes.push(shape);
      self
        .active_x_preview_shapes_by_relationship
        .insert(preview.control_relationship_id.clone(), shape_index);
    }
  }

  pub(crate) fn get_sub_type_text_list_style(
    &self,
    sub_type: Option<p::PlaceholderValues>,
  ) -> Option<&TextListStyle> {
    match sub_type {
      Some(p::PlaceholderValues::Title | p::PlaceholderValues::CenteredTitle) => {
        self.title_text_style.as_ref()
      }
      Some(
        p::PlaceholderValues::SubTitle | p::PlaceholderValues::Object | p::PlaceholderValues::Body,
      ) => {
        if self.is_notes {
          self.notes_text_style.as_ref()
        } else {
          self.body_text_style.as_ref()
        }
      }
      Some(
        p::PlaceholderValues::DateAndTime
        | p::PlaceholderValues::SlideNumber
        | p::PlaceholderValues::Footer
        | p::PlaceholderValues::Header
        | p::PlaceholderValues::Chart
        | p::PlaceholderValues::Table
        | p::PlaceholderValues::ClipArt
        | p::PlaceholderValues::Diagram
        | p::PlaceholderValues::Media
        | p::PlaceholderValues::SlideImage
        | p::PlaceholderValues::Picture,
      )
      | None => {
        // LibreOffice PPTShape::addShape uses the presentation default text
        // style for an ordinary shape that owns a text body. `otherStyle` is
        // selected only by its no-text-body fallback, so applying it here
        // would incorrectly override the shape's a:fontRef color.
        self.default_text_style.as_ref()
      }
    }
    .or(self.default_text_style.as_ref())
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

  pub(crate) fn hide_master_location_shapes(&mut self) {
    for shape in &mut self.shapes {
      shape.hide_if_master_location();
    }
  }

  pub(crate) fn create_background(&mut self, _import: &PowerPointImport) {
    // createBackground pushes resolved bg/bgPr/bgRef state to the page.
  }

  pub(crate) fn create_x_shapes(&mut self, import: &PowerPointImport) {
    // createXShapes applies text styles, creates shapes, then resolves
    // connector maps. Rust keeps a drawing model instead of UNO XShapes.
    self.apply_text_styles(import);
    for shape in &mut self.shapes {
      shape.create_and_insert(import);
    }
    self.rebuild_shape_maps();
    self.create_connector_shape_connection();
  }

  pub(crate) fn apply_text_styles(&mut self, _import: &PowerPointImport) {
    // applyTextStyles prepares paragraph-level master style state before
    // createAndInsert lowers DrawingML text into drawing objects.
    for shape in &mut self.shapes {
      shape.apply_text_styles();
    }
  }

  pub(crate) fn create_connector_shape_connection(&mut self) {
    // createXShapes builds a connector shape map after shape creation, then
    // applies connector endpoint links against the page shape map.
    self.connector_connections_applied = !self.connector_shape_map.is_empty();
  }

  fn rebuild_shape_maps(&mut self) {
    self.shape_map.clear();
    self.connector_shape_map.clear();
    for shape in &self.shapes {
      shape.collect_shape_maps(&mut self.shape_map, &mut self.connector_shape_map);
    }
  }
}

pub(crate) fn normalize_vml_shape_id(shape_id: &str) -> String {
  let shape_id = shape_id.trim();
  if shape_id.parse::<u32>().is_ok() {
    format!("_x0000_s{shape_id}")
  } else {
    shape_id.to_string()
  }
}

fn active_x_control_for_vml_model<'a>(
  controls_by_shape: &'a HashMap<String, ActiveXControlRecord>,
  model: &crate::xlsx::object_resources::VmlShapeModel,
) -> Option<&'a ActiveXControlRecord> {
  if let Some(shape_id) = model.shape_id.as_ref() {
    return controls_by_shape.get(shape_id);
  }
  model.id.as_ref().and_then(|id| controls_by_shape.get(id))
}

pub(crate) fn active_x_wmf_external_header(
  control: &p::Control,
) -> Option<crate::render::emf_wmf::WmfExternalHeader> {
  let width_emu = i64::from(control.image_width?);
  let height_emu = i64::from(control.image_height?);
  if width_emu <= 0 || height_emu <= 0 {
    return None;
  }
  Some(crate::render::emf_wmf::WmfExternalHeader {
    width_hundredths_mm: u32::try_from(ooxmlsdk::units::emu_to_mm100(width_emu)).ok()?,
    height_hundredths_mm: u32::try_from(ooxmlsdk::units::emu_to_mm100(height_emu)).ok()?,
    // Office ActiveX preview WMFs are screen-compatible 96-DPI metafiles.
    // Keep the reference-device resolution explicit because Win32 combines
    // it with METAFILEPICT.xExt/yExt when realizing the playback viewport.
    reference_device_dpi_x: 96,
    reference_device_dpi_y: 96,
  })
}

fn attach_vml_active_x_preview(
  shapes: &mut [Shape],
  preview_shapes_by_relationship: &HashMap<String, usize>,
  control: &ActiveXControlRecord,
  relationship_id: &str,
  mut resource: ImageResource,
) -> bool {
  let Some(control_relationship_id) = control.control.id.as_ref() else {
    return false;
  };
  let Some(shape_index) = preview_shapes_by_relationship.get(control_relationship_id) else {
    return false;
  };
  let Some(shape) = shapes.get_mut(*shape_index) else {
    return false;
  };

  if shape.name.is_none() {
    shape.name = control.control.name.clone();
  }
  resource.metafile_external_header = active_x_wmf_external_header(&control.control);
  if let Some(picture) = shape.picture.as_mut() {
    // Keep the selected/fallback p:pic relationship and structured blip
    // metadata, but realize the image through the associated VML part.
    picture.image_resource = Some(resource);
  } else {
    shape.set_picture(
      Some(relationship_id.to_string()),
      None,
      ImageCrop::default(),
      Vec::new(),
      Some(resource),
    );
  }
  true
}

fn attach_vml_ole_preview(
  shapes: &mut [Shape],
  model: &crate::xlsx::object_resources::VmlShapeModel,
  relationship_id: &str,
  resource: ImageResource,
) -> bool {
  let Some(shape_id) = model.id.as_deref() else {
    return false;
  };
  let Some(shape) = find_vml_ole_shape(shapes, shape_id) else {
    return false;
  };

  // LibreOffice's DrawingML OLE importer resolves the VML shape through
  // p:oleObj@spid, uses its imagedata as the replacement graphic, and also
  // transfers the VML fill/stroke. Keep an inline p:pic when one was selected;
  // the matched VML node still belongs to this OLE shape and must not be
  // emitted independently.
  if shape.picture.is_none() {
    shape.set_picture(
      Some(relationship_id.to_string()),
      None,
      ImageCrop::default(),
      Vec::new(),
      Some(resource),
    );
  }
  shape.legacy_vml_fill = Some(crate::xlsx::vml_shape_common_fill(
    model,
    kurbo::Affine::IDENTITY,
  ));
  shape.legacy_vml_stroke = crate::xlsx::vml_shape_common_stroke(model);
  true
}

fn find_vml_ole_shape<'a>(shapes: &'a mut [Shape], shape_id: &str) -> Option<&'a mut Shape> {
  for shape in shapes {
    let matches = shape
      .graphic_data
      .as_ref()
      .and_then(|record| record.ole_object.as_ref())
      .and_then(|ole| ole.shape_id.as_deref())
      == Some(shape_id);
    if matches {
      return Some(shape);
    }
    if let Some(found) = find_vml_ole_shape(&mut shape.children, shape_id) {
      return Some(found);
    }
  }
  None
}

fn vml_preview_color(value: Option<&str>, fallback: (u8, u8, u8)) -> Color {
  let color = value
    .and_then(crate::docx::parse_vml_color)
    .map(|color| (color.r, color.g, color.b))
    .unwrap_or(fallback);
  Color::RgbHex(RgbHexColor {
    value: format!("{:02X}{:02X}{:02X}", color.0, color.1, color.2),
    transformations: Vec::new(),
  })
}

#[derive(Default)]
struct ActiveXFallbackPreviewBuilder {
  control_relationship_id: Option<String>,
  image_relationship_id: Option<String>,
  shape_id: Option<String>,
  name: Option<String>,
  show_as_icon: Option<bool>,
  image_width: Option<i32>,
  image_height: Option<i32>,
  x: Option<i64>,
  y: Option<i64>,
  cx: Option<i64>,
  cy: Option<i64>,
}

impl ActiveXFallbackPreviewBuilder {
  fn finish(self) -> Option<ActiveXFallbackPreview> {
    Some(ActiveXFallbackPreview {
      control_relationship_id: self.control_relationship_id?,
      image_relationship_id: self.image_relationship_id?,
      shape_id: self.shape_id,
      name: self.name,
      show_as_icon: self.show_as_icon,
      image_width: self.image_width,
      image_height: self.image_height,
      position: super::drawingml::shape::Point {
        x: self.x?,
        y: self.y?,
      },
      size: super::drawingml::shape::Size {
        cx: self.cx?,
        cy: self.cy?,
      },
    })
  }
}

pub(crate) fn active_x_fallback_previews(xml: &[u8]) -> Vec<ActiveXFallbackPreview> {
  let mut reader = quick_xml::Reader::from_reader(xml);
  reader.config_mut().trim_text(true);
  let mut previews = Vec::new();
  let mut fallback_depth = 0usize;
  let mut picture_depth = 0usize;
  let mut shape_properties_depth = 0usize;
  let mut transform_depth = 0usize;
  let mut current = None::<ActiveXFallbackPreviewBuilder>;

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => match event.local_name().as_ref() {
        b"Fallback" => fallback_depth = fallback_depth.saturating_add(1),
        b"control" if fallback_depth > 0 && current.is_none() => {
          current = Some(ActiveXFallbackPreviewBuilder {
            control_relationship_id: xml_attribute(&event, b"id"),
            shape_id: xml_attribute(&event, b"spid"),
            name: xml_attribute(&event, b"name"),
            show_as_icon: xml_attribute(&event, b"showAsIcon")
              .and_then(|value| parse_xml_bool(&value)),
            image_width: xml_attribute(&event, b"imgW").and_then(|value| value.parse().ok()),
            image_height: xml_attribute(&event, b"imgH").and_then(|value| value.parse().ok()),
            ..ActiveXFallbackPreviewBuilder::default()
          });
        }
        b"pic" if current.is_some() => picture_depth = picture_depth.saturating_add(1),
        b"spPr" if picture_depth > 0 => {
          shape_properties_depth = shape_properties_depth.saturating_add(1)
        }
        b"xfrm" if shape_properties_depth > 0 => {
          transform_depth = transform_depth.saturating_add(1)
        }
        b"blip" if picture_depth > 0 => {
          if let Some(current) = current.as_mut() {
            current.image_relationship_id = xml_attribute(&event, b"embed");
          }
        }
        b"off" if transform_depth > 0 => {
          update_fallback_offset(current.as_mut(), &event);
        }
        b"ext" if transform_depth > 0 => {
          update_fallback_extent(current.as_mut(), &event);
        }
        _ => {}
      },
      Ok(Event::Empty(event)) => match event.local_name().as_ref() {
        b"blip" if picture_depth > 0 => {
          if let Some(current) = current.as_mut() {
            current.image_relationship_id = xml_attribute(&event, b"embed");
          }
        }
        b"off" if transform_depth > 0 => update_fallback_offset(current.as_mut(), &event),
        b"ext" if transform_depth > 0 => update_fallback_extent(current.as_mut(), &event),
        _ => {}
      },
      Ok(Event::End(event)) => match event.local_name().as_ref() {
        b"xfrm" if transform_depth > 0 => transform_depth -= 1,
        b"spPr" if shape_properties_depth > 0 => shape_properties_depth -= 1,
        b"pic" if picture_depth > 0 => picture_depth -= 1,
        b"control" if current.is_some() => {
          if let Some(preview) = current
            .take()
            .and_then(ActiveXFallbackPreviewBuilder::finish)
          {
            previews.push(preview);
          }
        }
        b"Fallback" if fallback_depth > 0 => fallback_depth -= 1,
        _ => {}
      },
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }
  previews
}

fn update_fallback_offset(
  current: Option<&mut ActiveXFallbackPreviewBuilder>,
  event: &BytesStart<'_>,
) {
  let Some(current) = current else {
    return;
  };
  current.x = xml_attribute(event, b"x").and_then(|value| value.parse().ok());
  current.y = xml_attribute(event, b"y").and_then(|value| value.parse().ok());
}

fn update_fallback_extent(
  current: Option<&mut ActiveXFallbackPreviewBuilder>,
  event: &BytesStart<'_>,
) {
  let Some(current) = current else {
    return;
  };
  current.cx = xml_attribute(event, b"cx").and_then(|value| value.parse().ok());
  current.cy = xml_attribute(event, b"cy").and_then(|value| value.parse().ok());
}

fn xml_attribute(event: &BytesStart<'_>, local_name: &[u8]) -> Option<String> {
  event.attributes().flatten().find_map(|attribute| {
    (attribute.key.as_ref().rsplit(|byte| *byte == b':').next() == Some(local_name))
      .then(|| String::from_utf8_lossy(attribute.value.as_ref()).into_owned())
  })
}

fn parse_xml_bool(value: &str) -> Option<bool> {
  match value.trim() {
    "true" | "1" => Some(true),
    "false" | "0" => Some(false),
    _ => None,
  }
}

fn vml_style_value<'a>(style: &'a str, key: &str) -> Option<&'a str> {
  style.split(';').find_map(|declaration| {
    let (name, value) = declaration.split_once(':')?;
    name
      .trim()
      .eq_ignore_ascii_case(key)
      .then_some(value.trim())
  })
}

fn vml_rotation_degrees(value: &str) -> f32 {
  let value = value.trim();
  -value
    .strip_suffix("fd")
    .and_then(|value| value.trim().parse::<f32>().ok())
    .map(|value| value / 65_536.0)
    .or_else(|| value.parse::<f32>().ok())
    .unwrap_or(0.0)
}

fn points_to_emu(value: f32) -> i64 {
  (value * ooxmlsdk::units::EMUS_PER_POINT as f32).round() as i64
}

fn vml_preview_rectangle_matches_shape(
  shape: &Shape,
  left_pt: f32,
  top_pt: f32,
  width_pt: f32,
  height_pt: f32,
) -> bool {
  const VML_ROUNDING_TOLERANCE_PT: f32 = 1.0;
  [
    (units::emu_to_points(shape.position.x), left_pt),
    (units::emu_to_points(shape.position.y), top_pt),
    (units::emu_to_points(shape.size.cx), width_pt),
    (units::emu_to_points(shape.size.cy), height_pt),
  ]
  .into_iter()
  .all(|(actual, expected)| (actual - expected).abs() <= VML_ROUNDING_TOLERANCE_PT)
}

fn vml_absolute_rectangle(style: Option<&str>) -> Option<(f32, f32, f32, f32)> {
  let mut left = None;
  let mut top = None;
  let mut width = None;
  let mut height = None;
  for declaration in style?.split(';') {
    let Some((name, value)) = declaration.split_once(':') else {
      continue;
    };
    let slot = match name.trim().to_ascii_lowercase().as_str() {
      "left" | "margin-left" => &mut left,
      "top" | "margin-top" => &mut top,
      "width" => &mut width,
      "height" => &mut height,
      _ => continue,
    };
    *slot = vml_measure_to_points(value);
  }
  Some((left?, top?, width?, height?))
}

pub(crate) fn vml_measure_to_points(value: &str) -> Option<f32> {
  let value = value.trim();
  let (number, multiplier) = if let Some(number) = value.strip_suffix("pt") {
    (number, 1.0)
  } else if let Some(number) = value.strip_suffix("in") {
    (number, units::POINTS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("cm") {
    (number, units::POINTS_PER_INCH / units::CENTIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("mm") {
    (number, units::POINTS_PER_INCH / units::MILLIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("px") {
    (number, units::POINTS_PER_CSS_PIXEL)
  } else {
    (value, 1.0)
  };
  number
    .trim()
    .parse::<f32>()
    .ok()
    .map(|value| value * multiplier)
}

fn slide_anchor(path: &str) -> Option<String> {
  let file_name = path.rsplit('/').next()?;
  let slide_number = file_name
    .strip_prefix("slide")?
    .strip_suffix(".xml")?
    .parse::<u32>()
    .ok()?;
  Some(format!("#Slide {slide_number}"))
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

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::sdk::SdkType;

  #[test]
  fn header_footer_attributes_default_to_enabled_when_element_exists() {
    let header_footer = HeaderFooter::from_pml(&p::HeaderFooter::default());

    assert!(header_footer.slide_number);
    assert!(header_footer.header);
    assert!(header_footer.footer);
    assert!(header_footer.date_time);
    assert!(HeaderFooter::default().has_visible_slot());
  }

  #[test]
  fn ordinary_shape_text_uses_presentation_default_style_not_other_style() {
    let default_style = TextListStyle {
      default_paragraph_properties: Some(Box::new(a::DefaultParagraphProperties {
        left_margin: Some(1),
        ..a::DefaultParagraphProperties::default()
      })),
      ..TextListStyle::default()
    };
    let other_style = TextListStyle {
      default_paragraph_properties: Some(Box::new(a::DefaultParagraphProperties {
        left_margin: Some(2),
        ..a::DefaultParagraphProperties::default()
      })),
      ..TextListStyle::default()
    };
    let mut slide = SlidePersist::new_slide(
      "ppt/slides/slide1.xml".to_string(),
      "rId1".to_string(),
      SlideSize::libreoffice_default(),
    );
    slide.default_text_style = Some(default_style.clone());
    slide.other_text_style = Some(other_style);

    assert_eq!(
      slide.get_sub_type_text_list_style(None),
      Some(&default_style)
    );
  }

  #[test]
  fn active_x_fallback_preview_preserves_control_metadata_and_picture_geometry() {
    let xml = br#"<p:sld xmlns:p="p" xmlns:a="a" xmlns:r="r" xmlns:mc="mc">
      <p:controls><mc:AlternateContent>
        <mc:Choice Requires="v"><p:control r:id="rId2" name="coarse"/></mc:Choice>
        <mc:Fallback><p:control spid="1140" r:id="rId2" name="CommandButton1"
          showAsIcon="1" imgW="1828800" imgH="1085760"><p:pic>
          <p:nvPicPr><p:cNvPr id="8" name="CommandButton1"><a:extLst>
            <a:ext uri="ignored"/>
          </a:extLst></p:cNvPr></p:nvPicPr>
          <p:blipFill><a:blip r:embed="rId20"/></p:blipFill>
          <p:spPr><a:xfrm><a:off x="516766" y="463895"/>
            <a:ext cx="1828869" cy="1086609"/></a:xfrm></p:spPr>
        </p:pic></p:control></mc:Fallback>
      </mc:AlternateContent></p:controls>
    </p:sld>"#;

    assert_eq!(
      active_x_fallback_previews(xml),
      vec![ActiveXFallbackPreview {
        control_relationship_id: "rId2".to_string(),
        image_relationship_id: "rId20".to_string(),
        shape_id: Some("1140".to_string()),
        name: Some("CommandButton1".to_string()),
        show_as_icon: Some(true),
        image_width: Some(1_828_800),
        image_height: Some(1_085_760),
        position: super::super::drawingml::shape::Point {
          x: 516_766,
          y: 463_895,
        },
        size: super::super::drawingml::shape::Size {
          cx: 1_828_869,
          cy: 1_086_609,
        },
      }]
    );
  }

  #[test]
  fn active_x_vml_spid_associates_preview_without_replacing_picture_transform() {
    let control = p::Control::from_bytes(
      br#"<p:control
        xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
        spid="1140" name="CommandButton1" showAsIcon="1" r:id="rId2"
        imgW="500000" imgH="600000">
        <p:extLst><p:ext uri="urn:control-counterexample"><x:payload
          xmlns:x="urn:counterexample"/></p:ext></p:extLst>
      </p:control>"#,
    )
    .expect("valid selected control");
    let mut slide = SlidePersist::new_slide(
      "ppt/slides/slide1.xml".to_string(),
      "rId1".to_string(),
      SlideSize::libreoffice_default(),
    );
    let mut group_context =
      super::super::shape_group_context::PPTShapeGroupContext::new(ShapeLocation::Slide);
    group_context.import_control_list(
      &mut slide,
      &p::ControlList {
        xml_children: vec![p::ControlListChoice::Control(Box::new(control))],
      },
    );

    let fallback_resource = ImageResource {
      data: Arc::from(&b"fallback"[..]),
      content_type: Some("image/x-wmf".to_string()),
      monochrome_dib_palette_override: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: false,
    };
    slide
      .image_resources
      .insert("rId20".to_string(), fallback_resource);
    slide.import_active_x_fallback_previews(&[ActiveXFallbackPreview {
      control_relationship_id: "rId2".to_string(),
      image_relationship_id: "rId20".to_string(),
      shape_id: None,
      name: Some("CommandButton1".to_string()),
      show_as_icon: None,
      image_width: Some(1_828_800),
      image_height: Some(1_085_760),
      position: super::super::drawingml::shape::Point { x: 111, y: 222 },
      size: super::super::drawingml::shape::Size { cx: 333, cy: 444 },
    }]);
    assert_eq!(
      slide.shapes[0]
        .picture
        .as_ref()
        .and_then(|picture| picture.image_resource.as_ref())
        .and_then(|resource| resource.metafile_external_header),
      Some(crate::render::emf_wmf::WmfExternalHeader {
        width_hundredths_mm: 5_080,
        height_hundredths_mm: 3_016,
        reference_device_dpi_x: 96,
        reference_device_dpi_y: 96,
      })
    );

    // A matching object name cannot override a conflicting o:spid. This is
    // the counterexample to image/name/near-rectangle based de-duplication.
    let wrong_spid = crate::xlsx::object_resources::VmlShapeModel {
      id: Some("CommandButton1".to_string()),
      shape_id: Some("_x0000_s9999".to_string()),
      ..crate::xlsx::object_resources::VmlShapeModel::default()
    };
    assert!(
      active_x_control_for_vml_model(&slide.active_x_controls_by_shape, &wrong_spid).is_none()
    );
    assert_eq!(
      slide.shapes[0].position,
      super::super::drawingml::shape::Point { x: 111, y: 222 }
    );

    let model = crate::xlsx::object_resources::VmlShapeModel {
      id: Some("CommandButton1".to_string()),
      shape_id: Some("_x0000_s1140".to_string()),
      ..crate::xlsx::object_resources::VmlShapeModel::default()
    };
    let record = active_x_control_for_vml_model(&slide.active_x_controls_by_shape, &model)
      .expect("spid-associated selected control")
      .clone();
    assert_eq!(record.control.image_width, Some(500_000));
    assert_eq!(record.control.image_height, Some(600_000));
    assert!(
      record
        .control
        .show_as_icon
        .is_some_and(|value| value.as_bool())
    );
    assert_eq!(
      record
        .control
        .extension_list
        .as_ref()
        .map(|extensions| extensions.extension.len()),
      Some(1)
    );

    let vml_resource = ImageResource {
      data: Arc::from(&b"vml"[..]),
      content_type: Some("image/x-wmf".to_string()),
      monochrome_dib_palette_override: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: true,
    };
    assert!(attach_vml_active_x_preview(
      &mut slide.shapes,
      &slide.active_x_preview_shapes_by_relationship,
      &record,
      "rId1",
      vml_resource.clone(),
    ));
    assert_eq!(
      slide.shapes[0].position,
      super::super::drawingml::shape::Point { x: 111, y: 222 }
    );
    assert_eq!(
      slide.shapes[0].size,
      super::super::drawingml::shape::Size { cx: 333, cy: 444 }
    );
    let attached = slide.shapes[0]
      .picture
      .as_ref()
      .and_then(|picture| picture.image_resource.as_ref())
      .expect("VML preview resource");
    assert_eq!(attached.data, vml_resource.data);
    assert_eq!(
      attached.metafile_external_header,
      Some(crate::render::emf_wmf::WmfExternalHeader {
        width_hundredths_mm: 1_389,
        height_hundredths_mm: 1_667,
        reference_device_dpi_x: 96,
        reference_device_dpi_y: 96,
      })
    );
  }

  #[test]
  fn vml_ole_preview_uses_spid_without_changing_shape_tree_z_order() {
    let xml = br#"<a:graphicData
      xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
      xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
      xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
      uri="http://schemas.openxmlformats.org/presentationml/2006/ole">
      <p:oleObj spid="_x0000_s1027" name="Document" r:id="rId3"
        imgW="5946064" imgH="903258" progId="Word.Document.12">
        <p:link updateAutomatic="1"/>
      </p:oleObj>
    </a:graphicData>"#;
    let graphic_data = a::GraphicData::from_bytes(xml).expect("valid OLE graphicData");
    let slide = SlidePersist::new_slide(
      "ppt/slides/slide1.xml".to_string(),
      "rId1".to_string(),
      SlideSize::libreoffice_default(),
    );
    let mut ole_shape = Shape::new(super::super::drawingml::shape::ShapeService::GraphicObject);
    super::super::drawingml::graphical_object_frame_context::GraphicalObjectFrameContext
      .dispatch_graphic_data(&graphic_data, &slide, &mut ole_shape);
    ole_shape.name = Some("ole".to_string());
    ole_shape.position = super::super::drawingml::shape::Point { x: 10, y: 20 };
    ole_shape.size = super::super::drawingml::shape::Size { cx: 30, cy: 40 };

    let ole = ole_shape
      .graphic_data
      .as_ref()
      .and_then(|record| record.ole_object.as_ref())
      .expect("typed OLE record");
    assert_eq!(ole.shape_id.as_deref(), Some("_x0000_s1027"));
    assert_eq!(ole.image_width, Some(5_946_064));
    assert_eq!(ole.image_height, Some(903_258));
    assert!(matches!(
      ole.ole_object_choice.as_ref(),
      Some(p::OleObjectChoice::OleObjectLink(link))
        if link.auto_update.as_ref().is_some_and(|value| value.as_bool())
    ));

    let mut underlay = Shape::new(super::super::drawingml::shape::ShapeService::Custom);
    underlay.name = Some("underlay".to_string());
    let mut overlay = Shape::new(super::super::drawingml::shape::ShapeService::Custom);
    overlay.name = Some("overlay".to_string());
    let mut shapes = vec![underlay, ole_shape, overlay];
    let model = crate::xlsx::object_resources::VmlShapeModel {
      id: Some("_x0000_s1027".to_string()),
      image_relationship_id: Some("rId1".to_string()),
      filled: false,
      stroked: false,
      ..crate::xlsx::object_resources::VmlShapeModel::default()
    };
    let resource = ImageResource {
      data: Arc::from(&b"emf"[..]),
      content_type: Some("image/x-emf".to_string()),
      monochrome_dib_palette_override: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: false,
    };

    assert!(attach_vml_ole_preview(
      &mut shapes,
      &model,
      "rId1",
      resource.clone(),
    ));
    assert_eq!(shapes.len(), 3);
    assert_eq!(shapes[0].name.as_deref(), Some("underlay"));
    assert_eq!(shapes[1].name.as_deref(), Some("ole"));
    assert_eq!(shapes[2].name.as_deref(), Some("overlay"));
    assert_eq!(
      shapes[1]
        .picture
        .as_ref()
        .and_then(|picture| picture.image_resource.as_ref()),
      Some(&resource)
    );
    assert!(shapes[0].picture.is_none());
    assert!(shapes[2].picture.is_none());
    assert_eq!(
      shapes[1].position,
      super::super::drawingml::shape::Point { x: 10, y: 20 }
    );
    assert_eq!(
      shapes[1].size,
      super::super::drawingml::shape::Size { cx: 30, cy: 40 }
    );
  }
}
