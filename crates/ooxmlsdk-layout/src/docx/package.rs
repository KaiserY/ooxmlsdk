use base64::Engine;
use bytes::Bytes;
use olecfsdk::{
  cfb::CompoundFile,
  forms::{CommandButtonControl, LabelControl, MorphDataControl, TextProps},
  ograph::{OgraphChart, OgraphFile},
};
use ooxmlsdk::common::RelationshipTargetKind;
use ooxmlsdk::parts::{
  alternative_format_import_part::AlternativeFormatImportPart, chart_part::ChartPart,
  diagram_colors_part::DiagramColorsPart, diagram_data_part::DiagramDataPart,
  diagram_persist_layout_part::DiagramPersistLayoutPart,
  embedded_control_persistence_part::EmbeddedControlPersistencePart,
  embedded_object_part::EmbeddedObjectPart, endnotes_part::EndnotesPart,
  extended_chart_part::ExtendedChartPart, footer_part::FooterPart, footnotes_part::FootnotesPart,
  header_part::HeaderPart, image_part::ImagePart, main_document_part::MainDocumentPart,
  numbering_definitions_part::NumberingDefinitionsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_2006_active_x as ax,
  schemas_microsoft_com_office_drawing_2008_diagram as dsp,
  schemas_microsoft_com_office_drawing_2012_chart_style as cs,
  schemas_microsoft_com_office_drawing_2014_chartex as cx,
  schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
};
use ooxmlsdk::sdk::{RelatedPart, SdkPart, SdkType};
use quick_xml::events::{BytesStart, Event};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub(super) struct ImageCatalog {
  pub(super) by_relationship_id: HashMap<String, ImageResource>,
  pub(super) signed_signature_line_images_by_id: HashMap<String, SignatureLineImages>,
  pub(super) active_x_text_style_by_relationship_id: HashMap<String, ActiveXTextStyle>,
  pub(super) math_type_by_relationship_id: HashMap<String, super::math_type::MathTypeEquation>,
  pub(super) ograph_charts_by_relationship_id: HashMap<String, OgraphChartResource>,
  pub(super) charts_by_relationship_id: HashMap<String, c::ChartSpace>,
  pub(super) extended_charts_by_relationship_id: HashMap<String, ExtendedChartResource>,
  pub(super) diagram_colors_by_relationship_id: HashMap<String, dgm::ColorsDefinition>,
  pub(super) diagram_data_by_relationship_id: HashMap<String, dgm::DataModelRoot>,
  pub(super) diagram_drawings_by_relationship_id: HashMap<String, dsp::Drawing>,
}

#[derive(Clone, Debug)]
pub(super) struct OgraphChartResource {
  pub(super) chart: OgraphChart,
  pub(super) chart_space: c::ChartSpace,
}

#[derive(Clone, Debug)]
pub(super) struct ActiveXTextStyle {
  /// An explicitly persisted Forms font. `None` means that the Forms
  /// TextProps record exists but leaves FontName at the host default.
  pub(super) font_family: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct ExtendedChartResource {
  pub(super) chart_space: cx::ChartSpace,
  pub(super) chart_styles: Vec<cs::ChartStyle>,
  pub(super) color_styles: Vec<cs::ColorStyle>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct HyperlinkCatalog {
  by_relationship_id: HashMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct AltChunkCatalog {
  pub(super) by_relationship_id: HashMap<String, AltChunkResource>,
}

#[derive(Clone, Debug)]
pub(super) struct AltChunkResource {
  pub(super) data: Bytes,
  pub(super) content_type: Option<String>,
}

impl AltChunkCatalog {
  pub(super) fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let by_relationship_id = main
      .related_parts_of_type::<_, AlternativeFormatImportPart>(package)
      .filter_map(|related| {
        let relationship_id = related.relationship_id().to_string();
        let part = related.part();
        let data = part.try_data_bytes(package).ok()?;
        Some((
          relationship_id,
          AltChunkResource {
            data,
            content_type: part.content_type(package).map(str::to_string),
          },
        ))
      })
      .collect();
    Self { by_relationship_id }
  }
}

impl HyperlinkCatalog {
  pub(super) fn load<P>(package: &WordprocessingDocument, part: &P) -> Self
  where
    P: SdkPart,
  {
    let by_relationship_id = part
      .hyperlink_relationships(package)
      .filter(|relationship| relationship.target_kind() == RelationshipTargetKind::External)
      .map(|relationship| {
        (
          relationship.id().to_string(),
          relationship.target().to_string(),
        )
      })
      .collect();
    Self { by_relationship_id }
  }

  pub(super) fn target(&self, relationship_id: &str) -> Option<&str> {
    self
      .by_relationship_id
      .get(relationship_id)
      .map(String::as_str)
  }
}

#[derive(Clone, Debug)]
pub(super) struct ImageResource {
  pub(super) data: Bytes,
  pub(super) content_type: Option<String>,
}

#[derive(Clone, Debug)]
pub(super) struct SignatureLineImages {
  pub(super) valid: ImageResource,
  pub(super) invalid: Option<ImageResource>,
}

impl SignatureLineImages {
  pub(super) fn for_validity(&self, valid: bool) -> Option<&ImageResource> {
    if valid {
      Some(&self.valid)
    } else {
      self.invalid.as_ref()
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SignatureImageObject {
  Valid,
  Invalid,
}

#[derive(Default)]
struct SignatureLineImagesBuilder {
  setup_id: String,
  valid_base64: String,
  invalid_base64: String,
  references_office_object: bool,
  references_valid_image: bool,
  references_invalid_image: bool,
  signature_value: String,
}

fn package_signature_line_images(
  package: &WordprocessingDocument,
) -> HashMap<String, SignatureLineImages> {
  let Some(origin) = package.digital_signature_origin_part() else {
    return HashMap::new();
  };
  origin
    .xml_signature_parts(package)
    .filter_map(|part| signature_line_images_from_xml(part.data(package)?))
    .collect()
}

fn signature_line_images_from_xml(xml: &[u8]) -> Option<(String, SignatureLineImages)> {
  let mut reader = quick_xml::Reader::from_reader(xml);
  reader.config_mut().trim_text(false);
  let mut builder = SignatureLineImagesBuilder::default();
  let mut object = None;
  let mut in_setup_id = false;
  let mut in_signature_value = false;

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => match event.local_name().as_ref() {
        b"Reference" => collect_signature_reference(&event, &mut builder),
        b"Object" => object = signature_image_object(&event),
        b"SetupID" => in_setup_id = true,
        b"SignatureValue" => in_signature_value = true,
        _ => {}
      },
      Ok(Event::Empty(event)) if event.local_name().as_ref() == b"Reference" => {
        collect_signature_reference(&event, &mut builder)
      }
      Ok(Event::Text(text)) => {
        let Ok(text) = text.xml10_content() else {
          continue;
        };
        if in_setup_id {
          builder.setup_id.push_str(&text);
        } else if in_signature_value {
          builder.signature_value.push_str(&text);
        } else {
          match object {
            Some(SignatureImageObject::Valid) => builder.valid_base64.push_str(&text),
            Some(SignatureImageObject::Invalid) => builder.invalid_base64.push_str(&text),
            None => {}
          }
        }
      }
      Ok(Event::End(event)) => match event.local_name().as_ref() {
        b"Object" => object = None,
        b"SetupID" => in_setup_id = false,
        b"SignatureValue" => in_signature_value = false,
        _ => {}
      },
      Ok(Event::Eof) => break,
      Err(_) => return None,
      _ => {}
    }
  }

  let setup_id = builder.setup_id.trim();
  if setup_id.is_empty()
    || builder.signature_value.trim().is_empty()
    || !builder.references_office_object
    || !builder.references_valid_image
  {
    return None;
  }
  let valid = decode_signature_line_image(&builder.valid_base64)?;
  let invalid = builder
    .references_invalid_image
    .then(|| decode_signature_line_image(&builder.invalid_base64))
    .flatten();
  Some((
    setup_id.to_ascii_lowercase(),
    SignatureLineImages { valid, invalid },
  ))
}

fn collect_signature_reference(event: &BytesStart<'_>, builder: &mut SignatureLineImagesBuilder) {
  let Some(uri) = signature_xml_attribute(event, b"URI") else {
    return;
  };
  match uri.as_str() {
    "#idOfficeObject" => builder.references_office_object = true,
    "#idValidSigLnImg" => builder.references_valid_image = true,
    "#idInvalidSigLnImg" => builder.references_invalid_image = true,
    _ => {}
  }
}

fn signature_image_object(event: &BytesStart<'_>) -> Option<SignatureImageObject> {
  match signature_xml_attribute(event, b"Id")?.as_str() {
    "idValidSigLnImg" => Some(SignatureImageObject::Valid),
    "idInvalidSigLnImg" => Some(SignatureImageObject::Invalid),
    _ => None,
  }
}

fn signature_xml_attribute(event: &BytesStart<'_>, name: &[u8]) -> Option<String> {
  event.attributes().flatten().find_map(|attribute| {
    xml_local_name(attribute.key.as_ref())
      .eq(name)
      .then(|| String::from_utf8_lossy(attribute.value.as_ref()).into_owned())
  })
}

fn xml_local_name(name: &[u8]) -> &[u8] {
  name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn decode_signature_line_image(encoded: &str) -> Option<ImageResource> {
  let compact = encoded
    .bytes()
    .filter(|byte| !byte.is_ascii_whitespace())
    .collect::<Vec<_>>();
  let data = base64::engine::general_purpose::STANDARD
    .decode(compact)
    .ok()?;
  let content_type = if data.len() >= 44 && &data[40..44] == b" EMF" {
    "image/x-emf"
  } else if data.starts_with(b"\x89PNG\r\n\x1a\n") {
    "image/png"
  } else if data.starts_with(&[0xff, 0xd8, 0xff]) {
    "image/jpeg"
  } else {
    return None;
  };
  Some(ImageResource {
    data: data.into(),
    content_type: Some(content_type.to_string()),
  })
}

fn active_x_text_style(
  control: &ax::ActiveXControlData,
  binary: Option<&[u8]>,
) -> Option<ActiveXTextStyle> {
  if let Some(font_family) = active_x_property_bag_font(control) {
    return Some(ActiveXTextStyle {
      font_family: Some(font_family),
    });
  }

  let class_id = control.active_x_control_class_id.as_str();
  let persistence: Cow<'_, [u8]> = match control.persistence {
    ax::PersistenceValues::PersistPropertyBag => {
      // [MS-OI29500] stores explicitly authored font data in ax:ocxPr. A
      // Word-hosted TextBox without that property delegates the effective
      // preview font to its host. Other Forms controls retain the authored
      // preview font: [MS-OFORMS] gives absent TextProps::FontName its own
      // MS Sans Serif file-format default rather than a Word run default.
      return is_text_box_class(class_id).then_some(ActiveXTextStyle { font_family: None });
    }
    ax::PersistenceValues::PersistStorage => {
      let compound = CompoundFile::from_bytes(binary?).ok()?;
      Cow::Owned(
        compound
          .stream("contents")
          .or_else(|| compound.stream("Contents"))?
          .to_vec(),
      )
    }
    ax::PersistenceValues::PersistStream | ax::PersistenceValues::PersistStreamInit => {
      Cow::Borrowed(binary?)
    }
  };

  let font_family = if class_id.eq_ignore_ascii_case("{D7053240-CE69-11CD-A777-00DD01143C57}") {
    let control = CommandButtonControl::from_bytes(persistence.as_ref()).ok()?;
    text_props_font_family(&control.text_props)
  } else if class_id.eq_ignore_ascii_case("{978C9E23-D4B0-11CE-BF2D-00AA003F40D0}") {
    let control = LabelControl::from_bytes(persistence.as_ref()).ok()?;
    text_props_font_family(&control.text_props)
  } else if is_morph_data_class(class_id) {
    let control = MorphDataControl::from_bytes(persistence.as_ref()).ok()?;
    text_props_font_family(&control.text_props)
  } else {
    return None;
  };

  match font_family {
    Some(font_family) => Some(ActiveXTextStyle {
      font_family: Some(font_family),
    }),
    None if is_text_box_class(class_id) => Some(ActiveXTextStyle { font_family: None }),
    None => None,
  }
}

fn text_props_font_family(text_props: &TextProps) -> Option<String> {
  let descriptor = text_props.data_block.font_name.as_ref()?.value;
  let family = text_props
    .extra_data_block
    .font_name
    .as_ref()?
    .decode(descriptor)
    .ok()?;
  let family = family.trim_matches('\0').trim();
  (!family.is_empty()).then(|| family.to_string())
}

fn active_x_property_bag_font(control: &ax::ActiveXControlData) -> Option<String> {
  for property in &control.active_x_object_property {
    if property.name.as_str().eq_ignore_ascii_case("FontName")
      && let Some(value) = property.value.as_ref()
      && !value.as_str().trim().is_empty()
    {
      return Some(value.as_str().trim().to_string());
    }
    if let Some(ax::ActiveXObjectPropertyChoice::SharedComFont(font)) =
      property.active_x_object_property_choice.as_ref()
      && let Some(value) = font.active_x_object_property.iter().find_map(|property| {
        property
          .name
          .as_str()
          .eq_ignore_ascii_case("Name")
          .then_some(property.value.as_ref())
          .flatten()
      })
      && !value.as_str().trim().is_empty()
    {
      return Some(value.as_str().trim().to_string());
    }
  }
  None
}

fn is_morph_data_class(class_id: &str) -> bool {
  [
    "{8BD21D10-EC42-11CE-9E0D-00AA006002F3}",
    "{8BD21D20-EC42-11CE-9E0D-00AA006002F3}",
    "{8BD21D30-EC42-11CE-9E0D-00AA006002F3}",
    "{8BD21D40-EC42-11CE-9E0D-00AA006002F3}",
    "{8BD21D50-EC42-11CE-9E0D-00AA006002F3}",
    "{8BD21D60-EC42-11CE-9E0D-00AA006002F3}",
  ]
  .iter()
  .any(|known| class_id.eq_ignore_ascii_case(known))
}

fn is_text_box_class(class_id: &str) -> bool {
  class_id.eq_ignore_ascii_case("{8BD21D10-EC42-11CE-9E0D-00AA006002F3}")
}

impl ImageCatalog {
  pub(super) fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    Self::load_from_part(package, main)
  }

  pub(super) fn load_from_header(package: &WordprocessingDocument, header: &HeaderPart) -> Self {
    Self::load_from_part(package, header)
  }

  pub(super) fn load_from_footer(package: &WordprocessingDocument, footer: &FooterPart) -> Self {
    Self::load_from_part(package, footer)
  }

  pub(super) fn load_from_footnotes(
    package: &WordprocessingDocument,
    footnotes: &FootnotesPart,
  ) -> Self {
    Self::load_from_part(package, footnotes)
  }

  pub(super) fn load_from_endnotes(
    package: &WordprocessingDocument,
    endnotes: &EndnotesPart,
  ) -> Self {
    Self::load_from_part(package, endnotes)
  }

  pub(super) fn load_from_numbering(
    package: &WordprocessingDocument,
    numbering: &NumberingDefinitionsPart,
  ) -> Self {
    Self::load_from_part(package, numbering)
  }

  fn load_from_part<P>(package: &WordprocessingDocument, part: &P) -> Self
  where
    P: SdkPart,
  {
    let mut catalog = Self::from_image_parts(package, part.related_parts_of_type(package));
    catalog.active_x_text_style_by_relationship_id = part
      .related_parts_of_type::<_, EmbeddedControlPersistencePart>(package)
      .filter_map(|related| {
        let relationship_id = related.relationship_id().to_string();
        let control_part = related.part();
        let control_xml = control_part.try_data_bytes(package).ok()?;
        let control = ax::ActiveXControlData::from_bytes(&control_xml).ok()?;
        let binary = control_part
          .embedded_control_persistence_binary_data_parts(package)
          .next()
          .and_then(|part| part.try_data_bytes(package).ok());
        let style = active_x_text_style(&control, binary.as_deref())?;
        Some((relationship_id, style))
      })
      .collect();
    catalog.math_type_by_relationship_id = part
      .related_parts_of_type::<_, EmbeddedObjectPart>(package)
      .filter_map(|related| {
        let data = related.part().try_data_bytes(package).ok()?;
        let equation = super::math_type::equation_native(&data)?;
        Some((related.relationship_id().to_string(), equation))
      })
      .collect();
    catalog.ograph_charts_by_relationship_id = part
      .related_parts_of_type::<_, EmbeddedObjectPart>(package)
      .filter_map(|related| {
        let data = related.part().try_data_bytes(package).ok()?;
        let file = OgraphFile::from_bytes_compatible(&data).ok()?.value;
        let chart = file.workbook.chart().ok()?;
        Some((
          related.relationship_id().to_string(),
          OgraphChartResource {
            chart_space: super::ograph::chart_space(&chart),
            chart,
          },
        ))
      })
      .collect();

    let chart_parts = part
      .related_parts_of_type::<_, ChartPart>(package)
      .map(|related| (related.relationship_id().to_string(), related.into_part()))
      .collect::<Vec<_>>();
    let (charts, extended_charts) = Self::chart_parts(package, chart_parts);
    catalog.charts_by_relationship_id = charts;
    catalog.extended_charts_by_relationship_id = extended_charts;

    let extended_chart_parts = part
      .related_parts_of_type::<_, ExtendedChartPart>(package)
      .map(|related| (related.relationship_id().to_string(), related.into_part()))
      .collect::<Vec<_>>();
    catalog
      .extended_charts_by_relationship_id
      .extend(Self::extended_chart_parts(package, extended_chart_parts));

    let diagram_color_parts = part
      .related_parts_of_type::<_, DiagramColorsPart>(package)
      .map(|related| (related.relationship_id().to_string(), related.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_colors_by_relationship_id =
      Self::diagram_color_parts(package, diagram_color_parts);

    let diagram_data_parts = part
      .related_parts_of_type::<_, DiagramDataPart>(package)
      .map(|related| (related.relationship_id().to_string(), related.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_data_by_relationship_id = Self::diagram_data_parts(package, diagram_data_parts);

    let diagram_drawing_parts = part
      .related_parts_of_type::<_, DiagramPersistLayoutPart>(package)
      .map(|related| (related.relationship_id().to_string(), related.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_drawings_by_relationship_id =
      Self::diagram_drawing_parts(package, diagram_drawing_parts);
    catalog.signed_signature_line_images_by_id = package_signature_line_images(package);
    catalog
  }

  fn from_image_parts<'a>(
    package: &WordprocessingDocument,
    image_parts: impl Iterator<Item = RelatedPart<'a, ImagePart>> + 'a,
  ) -> Self {
    let mut by_relationship_id = HashMap::new();
    for related_part in image_parts {
      let relationship_id = related_part.relationship_id();
      let image_part = related_part.part();
      let Ok(data) = image_part.try_data_bytes(package) else {
        continue;
      };
      by_relationship_id.insert(
        relationship_id.to_string(),
        ImageResource {
          data,
          content_type: image_part.content_type(package).map(str::to_string),
        },
      );
    }

    Self {
      by_relationship_id,
      signed_signature_line_images_by_id: HashMap::new(),
      active_x_text_style_by_relationship_id: HashMap::new(),
      math_type_by_relationship_id: HashMap::new(),
      ograph_charts_by_relationship_id: HashMap::new(),
      charts_by_relationship_id: HashMap::new(),
      extended_charts_by_relationship_id: HashMap::new(),
      diagram_colors_by_relationship_id: HashMap::new(),
      diagram_data_by_relationship_id: HashMap::new(),
      diagram_drawings_by_relationship_id: HashMap::new(),
    }
  }

  fn chart_parts<'a>(
    package: &WordprocessingDocument,
    chart_parts: impl IntoIterator<Item = (String, ChartPart)> + 'a,
  ) -> (
    HashMap<String, c::ChartSpace>,
    HashMap<String, ExtendedChartResource>,
  ) {
    let mut classic_by_relationship_id = HashMap::new();
    let mut extended_by_relationship_id = HashMap::new();
    for (relationship_id, chart_part) in chart_parts {
      if let Ok(chart_space) = chart_part.root_element(package) {
        classic_by_relationship_id.insert(relationship_id, chart_space.clone());
      } else if let Ok(data) = chart_part.try_data_bytes(package)
        && let Ok(chart_space) = cx::ChartSpace::from_bytes(&data)
      {
        // Some Office producers keep the legacy chart relationship/content
        // type while storing a ChartEx root. Resolve by the typed root after
        // package/MCE selection instead of falling back to the sibling chart.
        let chart_style_parts: Vec<_> = chart_part.chart_style_parts(package).collect();
        let chart_color_style_parts: Vec<_> = chart_part.chart_color_style_parts(package).collect();
        let chart_styles = chart_style_parts
          .iter()
          .filter_map(|part| part.root_element(package).ok().cloned())
          .collect();
        let color_styles = chart_color_style_parts
          .iter()
          .filter_map(|part| part.root_element(package).ok().cloned())
          .collect();
        extended_by_relationship_id.insert(
          relationship_id,
          ExtendedChartResource {
            chart_space,
            chart_styles,
            color_styles,
          },
        );
      }
    }
    (classic_by_relationship_id, extended_by_relationship_id)
  }

  fn extended_chart_parts<'a>(
    package: &WordprocessingDocument,
    chart_parts: impl IntoIterator<Item = (String, ExtendedChartPart)> + 'a,
  ) -> HashMap<String, ExtendedChartResource> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, chart_part) in chart_parts {
      let Ok(chart_space) = chart_part.root_element(package) else {
        continue;
      };
      let chart_space = chart_space.clone();
      let chart_style_parts: Vec<_> = chart_part.chart_style_parts(package).collect();
      let chart_color_style_parts: Vec<_> = chart_part.chart_color_style_parts(package).collect();
      let chart_styles = chart_style_parts
        .iter()
        .filter_map(|part| part.root_element(package).ok().cloned())
        .collect();
      let color_styles = chart_color_style_parts
        .iter()
        .filter_map(|part| part.root_element(package).ok().cloned())
        .collect();
      by_relationship_id.insert(
        relationship_id,
        ExtendedChartResource {
          chart_space,
          chart_styles,
          color_styles,
        },
      );
    }
    by_relationship_id
  }

  fn diagram_color_parts<'a>(
    package: &WordprocessingDocument,
    parts: impl IntoIterator<Item = (String, DiagramColorsPart)> + 'a,
  ) -> HashMap<String, dgm::ColorsDefinition> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, part) in parts {
      let Ok(root) = part.root_element(package) else {
        continue;
      };
      by_relationship_id.insert(relationship_id, root.clone());
    }
    by_relationship_id
  }

  fn diagram_data_parts<'a>(
    package: &WordprocessingDocument,
    parts: impl IntoIterator<Item = (String, DiagramDataPart)> + 'a,
  ) -> HashMap<String, dgm::DataModelRoot> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, part) in parts {
      let Ok(root) = part.root_element(package) else {
        continue;
      };
      by_relationship_id.insert(relationship_id, root.clone());
    }
    by_relationship_id
  }

  fn diagram_drawing_parts<'a>(
    package: &WordprocessingDocument,
    parts: impl IntoIterator<Item = (String, DiagramPersistLayoutPart)> + 'a,
  ) -> HashMap<String, dsp::Drawing> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, part) in parts {
      let Ok(root) = part.root_element(package) else {
        continue;
      };
      by_relationship_id.insert(relationship_id, root.clone());
    }
    by_relationship_id
  }
}

#[cfg(test)]
mod tests {
  use base64::Engine;

  use super::signature_line_images_from_xml;

  #[test]
  fn signature_line_images_require_signed_references_and_match_the_setup_id() {
    let mut emf = vec![0u8; 44];
    emf[40..44].copy_from_slice(b" EMF");
    let encoded = base64::engine::general_purpose::STANDARD.encode(&emf);
    let xml = format!(
      r##"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">
        <SignedInfo>
          <Reference URI="#idOfficeObject"/>
          <Reference URI="#idValidSigLnImg"/>
          <Reference URI="#idInvalidSigLnImg"/>
        </SignedInfo>
        <SignatureValue>signed</SignatureValue>
        <Object Id="idOfficeObject"><SignatureInfoV1 xmlns="http://schemas.microsoft.com/office/2006/digsig"><SetupID>{{ABC}}</SetupID></SignatureInfoV1></Object>
        <Object Id="idValidSigLnImg">{encoded}</Object>
        <Object Id="idInvalidSigLnImg">{encoded}</Object>
      </Signature>"##
    );

    let (id, images) = signature_line_images_from_xml(xml.as_bytes()).expect("signed images");
    assert_eq!(id, "{abc}");
    assert_eq!(images.valid.data.as_ref(), emf);
    assert_eq!(
      images.invalid.as_ref().map(|image| image.data.as_ref()),
      Some(emf.as_slice())
    );

    let unsigned = xml.replace("<Reference URI=\"#idValidSigLnImg\"/>", "");
    assert!(signature_line_images_from_xml(unsigned.as_bytes()).is_none());
  }
}
