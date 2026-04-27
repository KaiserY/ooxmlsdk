//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
pub const PATH_PREFIX: &str = "dialogsheets";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DialogsheetPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DialogsheetPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
    DialogsheetPart,
    as_dialogsheet_part,
    as_dialogsheet_part_mut
  );
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
}
