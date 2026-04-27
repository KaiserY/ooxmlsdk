//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.documenttasks+xml";
pub const TARGET_NAME: &str = "tasks";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DocumentTasksPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DocumentTasksPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks,
    DocumentTasksPart,
    as_document_tasks_part,
    as_document_tasks_part_mut
  );
}
