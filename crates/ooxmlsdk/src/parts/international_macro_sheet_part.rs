//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct InternationalMacroSheetPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) spreadsheet_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
  >,
  pub(crate) drawings_part: crate::sdk::OptionalPart<crate::parts::drawings_part::DrawingsPart>,
  pub(crate) vml_drawing_parts:
    crate::sdk::RepeatedPart<crate::parts::vml_drawing_part::VmlDrawingPart>,
  pub(crate) worksheet_comments_part:
    crate::sdk::OptionalPart<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
  pub(crate) custom_property_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_property_part::CustomPropertyPart>,
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
