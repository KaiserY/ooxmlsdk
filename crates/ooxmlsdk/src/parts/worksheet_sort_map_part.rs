//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/wsSortMap";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.wsSortMap+xml";
pub const TARGET_NAME: &str = "wsSortMap";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetSortMapPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WorksheetSortMapPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap,
    WorksheetSortMapPart,
    as_worksheet_sort_map_part,
    as_worksheet_sort_map_part_mut
  );
}
