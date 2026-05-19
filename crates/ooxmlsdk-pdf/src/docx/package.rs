use std::collections::HashMap;

use ooxmlsdk::common::RelationshipTargetKind;
use ooxmlsdk::parts::{
  chart_part::ChartPart, endnotes_part::EndnotesPart, footer_part::FooterPart,
  footnotes_part::FootnotesPart, header_part::HeaderPart, image_part::ImagePart,
  main_document_part::MainDocumentPart, numbering_definitions_part::NumberingDefinitionsPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::sdk::SdkPart;

#[derive(Clone, Debug, Default)]
pub(super) struct ImageCatalog {
  pub(super) by_relationship_id: HashMap<String, ImageResource>,
  pub(super) charts_by_relationship_id: HashMap<String, String>,
  pub(super) diagram_colors_by_relationship_id: HashMap<String, String>,
  pub(super) diagram_data_by_relationship_id: HashMap<String, String>,
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
  pub(super) fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let mut catalog = Self::from_image_parts(package, main.image_parts(package), |image_part| {
      main.get_id_of_part(package, image_part)
    });
    catalog.charts_by_relationship_id =
      Self::chart_parts(package, main.chart_parts(package), |chart_part| {
        main.get_id_of_part(package, chart_part)
      });
    catalog.diagram_colors_by_relationship_id =
      Self::xml_parts(package, main.diagram_colors_parts(package), |part| {
        main.get_id_of_part(package, part)
      });
    catalog.diagram_data_by_relationship_id =
      Self::xml_parts(package, main.diagram_data_parts(package), |part| {
        main.get_id_of_part(package, part)
      });
    catalog.diagram_drawings_by_relationship_id = Self::xml_parts(
      package,
      main.diagram_persist_layout_parts(package),
      |part| main.get_id_of_part(package, part),
    );
    catalog
  }

  pub(super) fn load_from_header(package: &WordprocessingDocument, header: &HeaderPart) -> Self {
    Self::from_image_parts(package, header.image_parts(package), |image_part| {
      header.get_id_of_part(package, image_part)
    })
  }

  pub(super) fn load_from_footer(package: &WordprocessingDocument, footer: &FooterPart) -> Self {
    Self::from_image_parts(package, footer.image_parts(package), |image_part| {
      footer.get_id_of_part(package, image_part)
    })
  }

  pub(super) fn load_from_footnotes(
    package: &WordprocessingDocument,
    footnotes: &FootnotesPart,
  ) -> Self {
    Self::from_image_parts(package, footnotes.image_parts(package), |image_part| {
      footnotes.get_id_of_part(package, image_part)
    })
  }

  pub(super) fn load_from_endnotes(
    package: &WordprocessingDocument,
    endnotes: &EndnotesPart,
  ) -> Self {
    Self::from_image_parts(package, endnotes.image_parts(package), |image_part| {
      endnotes.get_id_of_part(package, image_part)
    })
  }

  pub(super) fn load_from_comments(
    package: &WordprocessingDocument,
    comments: &WordprocessingCommentsPart,
  ) -> Self {
    Self::from_image_parts(package, comments.image_parts(package), |image_part| {
      comments.get_id_of_part(package, image_part)
    })
  }

  pub(super) fn load_from_numbering(
    package: &WordprocessingDocument,
    numbering: &NumberingDefinitionsPart,
  ) -> Self {
    Self::from_image_parts(package, numbering.image_parts(package), |image_part| {
      numbering.get_id_of_part(package, image_part)
    })
  }

  fn from_image_parts<'a>(
    package: &WordprocessingDocument,
    image_parts: impl Iterator<Item = ImagePart> + 'a,
    relationship_id: impl Fn(&ImagePart) -> Option<&'a str>,
  ) -> Self {
    let mut by_relationship_id = HashMap::new();
    for image_part in image_parts {
      let Some(relationship_id) = relationship_id(&image_part) else {
        continue;
      };
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
    package: &WordprocessingDocument,
    chart_parts: impl Iterator<Item = ChartPart> + 'a,
    relationship_id: impl Fn(&ChartPart) -> Option<&'a str>,
  ) -> HashMap<String, String> {
    Self::xml_parts(package, chart_parts, relationship_id)
  }

  fn xml_parts<'a, P>(
    package: &WordprocessingDocument,
    parts: impl Iterator<Item = P> + 'a,
    relationship_id: impl Fn(&P) -> Option<&'a str>,
  ) -> HashMap<String, String>
  where
    P: SdkPart,
  {
    let mut by_relationship_id = HashMap::new();
    for part in parts {
      let Some(relationship_id) = relationship_id(&part) else {
        continue;
      };
      let Some(data) = part.data_to_vec(package) else {
        continue;
      };
      let Ok(xml) = String::from_utf8(data) else {
        continue;
      };
      by_relationship_id.insert(relationship_id.to_string(), xml);
    }
    by_relationship_id
  }
}
