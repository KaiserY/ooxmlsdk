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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
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
  crate::sdk::PartChildDescriptor::new(
    "comment_parts",
    "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    "crate::parts::power_point_comment_part::PowerPointCommentPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "authors_part",
    "http://schemas.microsoft.com/office/2018/10/relationships/authors",
    "crate::parts::power_point_authors_part::PowerPointAuthorsPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PresentationPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
    PresentationPart,
    as_presentation_part,
    as_presentation_part_mut
  );
  pub fn custom_xml_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::custom_xml_part::CustomXmlPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::custom_xml_part::CustomXmlPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    )
  }
  pub fn font_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::font_part::FontPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::font_part::FontPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
    )
  }
  pub fn presentation_properties_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::presentation_properties_part::PresentationPropertiesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::presentation_properties_part::PresentationPropertiesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
    )
  }
  pub fn table_styles_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::table_styles_part::TableStylesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::table_styles_part::TableStylesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
    )
  }
  pub fn theme_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::theme_part::ThemePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::theme_part::ThemePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    )
  }
  pub fn view_properties_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::view_properties_part::ViewPropertiesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::view_properties_part::ViewPropertiesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
    )
  }
  pub fn notes_master_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::notes_master_part::NotesMasterPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::notes_master_part::NotesMasterPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
    )
  }
  pub fn slide_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::slide_part::SlidePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::slide_part::SlidePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    )
  }
  pub fn slide_master_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::slide_master_part::SlideMasterPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::slide_master_part::SlideMasterPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
    )
  }
  pub fn user_defined_tags_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::user_defined_tags_part::UserDefinedTagsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
    )
  }
  pub fn comment_authors_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::comment_authors_part::CommentAuthorsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::comment_authors_part::CommentAuthorsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
    )
  }
  pub fn handout_master_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::handout_master_part::HandoutMasterPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::handout_master_part::HandoutMasterPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
    )
  }
  pub fn legacy_diagram_text_info_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
    )
  }
  pub fn vba_project_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::vba_project_part::VbaProjectPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::vba_project_part::VbaProjectPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn comment_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::power_point_comment_part::PowerPointCommentPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::power_point_comment_part::PowerPointCommentPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn authors_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::power_point_authors_part::PowerPointAuthorsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::power_point_authors_part::PowerPointAuthorsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2018/10/relationships/authors",
    )
  }
}
