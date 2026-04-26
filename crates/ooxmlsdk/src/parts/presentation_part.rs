//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "ppt";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "presentation";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_presentation_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font"
  ))]
  pub(crate) font_parts: Vec<crate::parts::font_part::FontPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps"
  ))]
  pub(crate) presentation_properties_part:
    Option<Box<crate::parts::presentation_properties_part::PresentationPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles"
  ))]
  pub(crate) table_styles_part: Option<Box<crate::parts::table_styles_part::TableStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
  ))]
  pub(crate) theme_part: Option<Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps"
  ))]
  pub(crate) view_properties_part:
    Option<Box<crate::parts::view_properties_part::ViewPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster"
  ))]
  pub(crate) notes_master_part: Option<Box<crate::parts::notes_master_part::NotesMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
  ))]
  pub(crate) slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
  ))]
  pub(crate) slide_master_parts: Vec<crate::parts::slide_master_part::SlideMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags"
  ))]
  pub(crate) user_defined_tags_part:
    Option<Box<crate::parts::user_defined_tags_part::UserDefinedTagsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors"
  ))]
  pub(crate) comment_authors_part:
    Option<Box<crate::parts::comment_authors_part::CommentAuthorsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster"
  ))]
  pub(crate) handout_master_part: Option<Box<crate::parts::handout_master_part::HandoutMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo"
  ))]
  pub(crate) legacy_diagram_text_info_part:
    Option<Box<crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part: Option<Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments"
  ))]
  pub(crate) comment_parts: Vec<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/authors"
  ))]
  pub(crate) authors_part:
    Option<Box<crate::parts::power_point_authors_part::PowerPointAuthorsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl PresentationPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "custom_xml_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
      "crate::parts::custom_xml_part::CustomXmlPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "font_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
      "crate::parts::font_part::FontPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "presentation_properties_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
      "crate::parts::presentation_properties_part::PresentationPropertiesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "table_styles_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
      "crate::parts::table_styles_part::TableStylesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "theme_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
      "crate::parts::theme_part::ThemePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "view_properties_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
      "crate::parts::view_properties_part::ViewPropertiesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "notes_master_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
      "crate::parts::notes_master_part::NotesMasterPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "slide_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
      "crate::parts::slide_part::SlidePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "slide_master_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
      "crate::parts::slide_master_part::SlideMasterPart",
      crate::sdk::PartChildCardinality::RequiredRepeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "user_defined_tags_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
      "crate::parts::user_defined_tags_part::UserDefinedTagsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "comment_authors_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
      "crate::parts::comment_authors_part::CommentAuthorsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "handout_master_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
      "crate::parts::handout_master_part::HandoutMasterPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "legacy_diagram_text_info_part",
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
      "crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "vba_project_part",
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
      "crate::parts::vba_project_part::VbaProjectPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "comment_parts",
      "http://schemas.microsoft.com/office/2018/10/relationships/comments",
      "crate::parts::power_point_comment_part::PowerPointCommentPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "authors_part",
      "http://schemas.microsoft.com/office/2018/10/relationships/authors",
      "crate::parts::power_point_authors_part::PowerPointAuthorsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
  ];
}
