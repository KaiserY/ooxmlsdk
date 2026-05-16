//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView";
pub const PATH_PREFIX: &str = "../namedSheetViews";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.namedsheetviews+xml";
pub const TARGET_NAME: &str = "namedSheetView";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct NamedSheetViewsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl NamedSheetViewsPart {
  crate::sdk_part_root_methods!(
        crate
        ::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
        NamedSheetViewsPart, as_named_sheet_views_part, as_named_sheet_views_part_mut
    );
}
