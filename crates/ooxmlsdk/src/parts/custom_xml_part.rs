//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
pub const PATH_PREFIX: &str = "../customXml";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "item";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] =
  &[crate::sdk::PartChildDescriptor::new(
    "custom_xml_properties_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
    "crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart",
    crate::sdk::PartChildCardinality::Optional,
  )];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomXmlPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CustomXmlPart {
  pub fn custom_xml_properties_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
    )
  }
  pub fn custom_xml_properties_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
    )
  }
}
