//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet";
pub const PATH_PREFIX: &str = "macrosheets";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.intlmacrosheet+xml";
pub const TARGET_NAME: &str = "intlsheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct InternationalMacroSheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl InternationalMacroSheetPart {
  pub fn spreadsheet_printer_settings_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<
    Item = crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  > + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
    )
  }
  pub fn drawings_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::drawings_part::DrawingsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::drawings_part::DrawingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
    )
  }
  pub fn vml_drawing_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::vml_drawing_part::VmlDrawingPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::vml_drawing_part::VmlDrawingPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    )
  }
  pub fn worksheet_comments_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::worksheet_comments_part::WorksheetCommentsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    )
  }
  pub fn custom_property_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::custom_property_part::CustomPropertyPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::custom_property_part::CustomPropertyPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
    )
  }
  pub fn embedded_object_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::embedded_object_part::EmbeddedObjectPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::embedded_object_part::EmbeddedObjectPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    )
  }
  pub fn embedded_package_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::embedded_package_part::EmbeddedPackagePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    )
  }
  pub fn image_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::image_part::ImagePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::image_part::ImagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    )
  }
}
