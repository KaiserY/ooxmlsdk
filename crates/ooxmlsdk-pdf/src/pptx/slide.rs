use std::collections::HashMap;

use ooxmlsdk::common::{MediaDataPart, RelationshipRef};
use ooxmlsdk::parts::{
  chart_color_style_part::ChartColorStylePart, chart_drawing_part::ChartDrawingPart,
  chart_part::ChartPart, chart_style_part::ChartStylePart, diagram_colors_part::DiagramColorsPart,
  diagram_data_part::DiagramDataPart, diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  diagram_style_part::DiagramStylePart, embedded_object_part::EmbeddedObjectPart,
  embedded_package_part::EmbeddedPackagePart, extended_chart_part::ExtendedChartPart,
  image_part::ImagePart, presentation_document::PresentationDocument,
  theme_override_part::ThemeOverridePart,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2012_chart_style as cs,
  schemas_microsoft_com_office_drawing_2014_chartex as cx,
  schemas_microsoft_com_office_powerpoint_2018_8_main as p188,
  schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};
use ooxmlsdk::sdk::SdkPart;

use crate::docx::PageSetup;
use crate::error::Result;
use crate::units;

use super::drawingml::color::Color;
use super::drawingml::fill::FillProperties;
use super::drawingml::shape::{Shape, ShapeMapEntry};
use super::drawingml::text_list_style::TextListStyle;
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
  pub(crate) time_node_list: Vec<TimeNode>,
  pub(crate) header_footer: HeaderFooter,
  pub(crate) layout_value_token: Option<String>,
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
  pub(crate) embedded_object_resources: HashMap<String, BinaryResource>,
  pub(crate) embedded_package_resources: HashMap<String, BinaryResource>,
  pub(crate) media_resources: HashMap<String, MediaResource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImageResource {
  pub(crate) data: Vec<u8>,
  pub(crate) content_type: Option<String>,
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
          data: related_part.part().data_to_vec(package)?,
          content_type: related_part
            .part()
            .content_type(package)
            .map(str::to_string),
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
pub(crate) struct TimeNode;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct HeaderFooter {
  pub(crate) slide_number: bool,
  pub(crate) header: bool,
  pub(crate) footer: bool,
  pub(crate) date_time: bool,
}

impl HeaderFooter {
  pub(crate) fn from_pml(header_footer: &p::HeaderFooter) -> Self {
    Self {
      slide_number: header_footer
        .slide_number
        .is_some_and(|value| value.as_bool()),
      header: header_footer.header.is_some_and(|value| value.as_bool()),
      footer: header_footer.footer.is_some_and(|value| value.as_bool()),
      date_time: header_footer.date_time.is_some_and(|value| value.as_bool()),
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
        a::ParagraphChoice::TextMath(_) => {}
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
    // Source: LibreOffice oox/source/ppt/slidefragmenthandler.cxx
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
      time_node_list: Vec::new(),
      header_footer: HeaderFooter::default(),
      layout_value_token: None,
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
      embedded_object_resources: HashMap::new(),
      embedded_package_resources: HashMap::new(),
      media_resources: HashMap::new(),
    }
  }

  pub(crate) fn get_layout_from_value_token(&self) -> Option<String> {
    self.layout_value_token.clone()
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
    // Source: LibreOffice oox/source/drawingml/blipcontext.cxx resolves blip
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
          data,
          content_type: image_part.content_type(package).map(str::to_string),
        },
      );
    }
  }

  pub(crate) fn import_media_reference_parts<P>(&mut self, package: &PresentationDocument, part: &P)
  where
    P: SdkPart,
  {
    // Source: LibreOffice oox/source/drawingml/graphicshapecontext.cxx
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

  pub(crate) fn import_graphic_frame_related_parts<P>(
    &mut self,
    package: &mut PresentationDocument,
    part: &P,
  ) -> Result<()>
  where
    P: SdkPart,
  {
    // Source: LibreOffice oox/source/drawingml/graphicshapecontext.cxx and
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
      self.diagram_data_resources.insert(
        relationship_id,
        DiagramDataResource {
          path: diagram_part.path(package).map(str::to_string),
          model: diagram_part.root_element(package)?.clone(),
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
    self.embedded_object_resources = reference.embedded_object_resources.clone();
    self.embedded_package_resources = reference.embedded_package_resources.clone();
    self.media_resources = reference.media_resources.clone();
    self.comment_authors = reference.comment_authors.clone();
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
      | None => self
        .other_text_style
        .as_ref()
        .or(self.default_text_style.as_ref()),
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
    self.rebuild_shape_maps();
    self.create_connector_shape_connection();
  }

  pub(crate) fn apply_text_styles(&mut self, _import: &PowerPointImport) {
    // Source: LibreOffice oox/source/ppt/slidepersist.cxx
    // applyTextStyles prepares paragraph-level master style state before
    // createAndInsert lowers DrawingML text into drawing objects.
    for shape in &mut self.shapes {
      shape.apply_text_styles();
    }
  }

  pub(crate) fn create_connector_shape_connection(&mut self) {
    // Source: LibreOffice oox/source/ppt/slidepersist.cxx
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
