//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct VisioDocumentPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_visio_2012_main::VisioDocument,
  >,
  pub(crate) visio_connections_part:
    crate::sdk::OptionalPart<crate::parts::visio_connections_part::VisioConnectionsPart>,
  pub(crate) visio_masters_part:
    crate::sdk::OptionalPart<crate::parts::visio_masters_part::VisioMastersPart>,
  pub(crate) visio_pages_part:
    crate::sdk::OptionalPart<crate::parts::visio_pages_part::VisioPagesPart>,
  pub(crate) visio_recordsets_part:
    crate::sdk::OptionalPart<crate::parts::visio_recordsets_part::VisioRecordsetsPart>,
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  pub(crate) visio_comments_part:
    crate::sdk::OptionalPart<crate::parts::visio_comments_part::VisioCommentsPart>,
  pub(crate) visio_extensions_part:
    crate::sdk::OptionalPart<crate::parts::visio_extensions_part::VisioExtensionsPart>,
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
}
pub type VisioDocumentPart = crate::sdk::PartHandle<VisioDocumentPartSpec>;
