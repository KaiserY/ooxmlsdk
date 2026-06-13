use std::collections::HashMap;

use ooxmlsdk::common::RelationshipTargetKind;
use ooxmlsdk::parts::{
  chart_part::ChartPart, diagram_colors_part::DiagramColorsPart,
  diagram_data_part::DiagramDataPart, diagram_persist_layout_part::DiagramPersistLayoutPart,
  endnotes_part::EndnotesPart, footer_part::FooterPart, footnotes_part::FootnotesPart,
  header_part::HeaderPart, image_part::ImagePart, main_document_part::MainDocumentPart,
  numbering_definitions_part::NumberingDefinitionsPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_chart as c,
  schemas_openxmlformats_org_drawingml_2006_diagram as dgm,
};
use ooxmlsdk::sdk::{RelatedPart, SdkPart, SdkType};

#[derive(Clone, Debug, Default)]
pub(super) struct ImageCatalog {
  pub(super) by_relationship_id: HashMap<String, ImageResource>,
  pub(super) charts_by_relationship_id: HashMap<String, c::ChartSpace>,
  pub(super) diagram_colors_by_relationship_id: HashMap<String, dgm::ColorsDefinition>,
  pub(super) diagram_data_by_relationship_id: HashMap<String, dgm::DataModelRoot>,
  pub(super) diagram_drawings_by_relationship_id: HashMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct HyperlinkCatalog {
  by_relationship_id: HashMap<String, String>,
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
  pub(super) data: Vec<u8>,
  pub(super) content_type: Option<String>,
}

impl ImageCatalog {
  pub(super) fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let mut catalog = Self::from_image_parts(package, main.related_parts_of_type(package));
    let chart_parts = main
      .related_parts_of_type::<_, ChartPart>(package)
      .map(|part| (part.relationship_id().to_string(), part.into_part()))
      .collect::<Vec<_>>();
    catalog.charts_by_relationship_id = Self::chart_parts(package, chart_parts);
    let diagram_color_parts = main
      .related_parts_of_type::<_, DiagramColorsPart>(package)
      .map(|part| (part.relationship_id().to_string(), part.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_colors_by_relationship_id =
      Self::diagram_color_parts(package, diagram_color_parts);
    let diagram_data_parts = main
      .related_parts_of_type::<_, DiagramDataPart>(package)
      .map(|part| (part.relationship_id().to_string(), part.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_data_by_relationship_id = Self::diagram_data_parts(package, diagram_data_parts);
    let diagram_drawing_parts = main
      .related_parts_of_type::<_, DiagramPersistLayoutPart>(package)
      .map(|part| (part.relationship_id().to_string(), part.into_part()))
      .collect::<Vec<_>>();
    catalog.diagram_drawings_by_relationship_id =
      Self::diagram_drawing_parts(package, diagram_drawing_parts);
    catalog
  }

  pub(super) fn load_from_header(package: &WordprocessingDocument, header: &HeaderPart) -> Self {
    Self::from_image_parts(package, header.related_parts_of_type(package))
  }

  pub(super) fn load_from_footer(package: &WordprocessingDocument, footer: &FooterPart) -> Self {
    Self::from_image_parts(package, footer.related_parts_of_type(package))
  }

  pub(super) fn load_from_footnotes(
    package: &WordprocessingDocument,
    footnotes: &FootnotesPart,
  ) -> Self {
    Self::from_image_parts(package, footnotes.related_parts_of_type(package))
  }

  pub(super) fn load_from_endnotes(
    package: &WordprocessingDocument,
    endnotes: &EndnotesPart,
  ) -> Self {
    Self::from_image_parts(package, endnotes.related_parts_of_type(package))
  }

  pub(super) fn load_from_comments(
    package: &WordprocessingDocument,
    comments: &WordprocessingCommentsPart,
  ) -> Self {
    Self::from_image_parts(package, comments.related_parts_of_type(package))
  }

  pub(super) fn load_from_numbering(
    package: &WordprocessingDocument,
    numbering: &NumberingDefinitionsPart,
  ) -> Self {
    Self::from_image_parts(package, numbering.related_parts_of_type(package))
  }

  fn from_image_parts<'a>(
    package: &WordprocessingDocument,
    image_parts: impl Iterator<Item = RelatedPart<'a, ImagePart>> + 'a,
  ) -> Self {
    let mut by_relationship_id = HashMap::new();
    for related_part in image_parts {
      let relationship_id = related_part.relationship_id();
      let image_part = related_part.part();
      let Some(data) = image_part.data_to_vec(package) else {
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
      charts_by_relationship_id: HashMap::new(),
      diagram_colors_by_relationship_id: HashMap::new(),
      diagram_data_by_relationship_id: HashMap::new(),
      diagram_drawings_by_relationship_id: HashMap::new(),
    }
  }

  fn chart_parts<'a>(
    package: &mut WordprocessingDocument,
    chart_parts: impl IntoIterator<Item = (String, ChartPart)> + 'a,
  ) -> HashMap<String, c::ChartSpace> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, chart_part) in chart_parts {
      let Ok(chart_space) = chart_part.root_element(package) else {
        continue;
      };
      by_relationship_id.insert(relationship_id, chart_space.clone());
    }
    by_relationship_id
  }

  fn diagram_color_parts<'a>(
    package: &mut WordprocessingDocument,
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
    package: &mut WordprocessingDocument,
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
    package: &mut WordprocessingDocument,
    parts: impl IntoIterator<Item = (String, DiagramPersistLayoutPart)> + 'a,
  ) -> HashMap<String, String> {
    let mut by_relationship_id = HashMap::new();
    for (relationship_id, part) in parts {
      let Ok(root) = part.root_element(package) else {
        continue;
      };
      let Ok(bytes) = root.to_bytes() else {
        continue;
      };
      let Ok(xml) = String::from_utf8(bytes) else {
        continue;
      };
      by_relationship_id.insert(relationship_id, xml);
    }
    by_relationship_id
  }
}
