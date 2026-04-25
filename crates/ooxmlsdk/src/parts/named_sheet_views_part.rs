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
    #[sdk(part_root(accessor = "as_named_sheet_views_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<Box<str>>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl NamedSheetViewsPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[];
}
