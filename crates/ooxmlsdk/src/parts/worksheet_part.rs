//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
pub const PATH_PREFIX: &str = "worksheets";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
pub const TARGET_NAME: &str = "sheet";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetPart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_worksheet_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
        )
    )]
    pub(crate) spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing"
        )
    )]
    pub(crate) drawings_part: Option<Box<crate::parts::drawings_part::DrawingsPart>>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
        )
    )]
    pub(crate) vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
        )
    )]
    pub(crate) worksheet_comments_part: Option<
        Box<crate::parts::worksheet_comments_part::WorksheetCommentsPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable"
        )
    )]
    pub(crate) pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells"
        )
    )]
    pub(crate) single_cell_table_part: Option<
        Box<crate::parts::single_cell_table_part::SingleCellTablePart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table"
        )
    )]
    pub(crate) table_definition_parts: Vec<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
        )
    )]
    pub(crate) embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp"
        )
    )]
    pub(crate) control_properties_parts: Vec<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
        )
    )]
    pub(crate) embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
        )
    )]
    pub(crate) embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
        )
    )]
    pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty"
        )
    )]
    pub(crate) custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wsSortMap"
        )
    )]
    pub(crate) worksheet_sort_map_part: Option<
        Box<crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable"
        )
    )]
    pub(crate) query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicer"
        )
    )]
    pub(crate) slicers_parts: Vec<crate::parts::slicers_part::SlicersPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timeline"
        )
    )]
    pub(crate) time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment"
        )
    )]
    pub(crate) worksheet_threaded_comments_parts: Vec<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
        )
    )]
    pub(crate) model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView"
        )
    )]
    pub(crate) named_sheet_views_parts: Vec<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl WorksheetPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "spreadsheet_printer_settings_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
      "crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "drawings_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
      "crate::parts::drawings_part::DrawingsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "vml_drawing_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
      "crate::parts::vml_drawing_part::VmlDrawingPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "worksheet_comments_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
      "crate::parts::worksheet_comments_part::WorksheetCommentsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "pivot_table_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
      "crate::parts::pivot_table_part::PivotTablePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "single_cell_table_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells",
      "crate::parts::single_cell_table_part::SingleCellTablePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "table_definition_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
      "crate::parts::table_definition_part::TableDefinitionPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_control_persistence_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
      "crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "control_properties_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
      "crate::parts::control_properties_part::ControlPropertiesPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_object_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
      "crate::parts::embedded_object_part::EmbeddedObjectPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_package_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
      "crate::parts::embedded_package_part::EmbeddedPackagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "custom_property_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
      "crate::parts::custom_property_part::CustomPropertyPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "worksheet_sort_map_part",
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap",
      "crate::parts::worksheet_sort_map_part::WorksheetSortMapPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "query_table_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable",
      "crate::parts::query_table_part::QueryTablePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_control_persistence_binary_data_parts",
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
      "crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "slicers_parts",
      "http://schemas.microsoft.com/office/2007/relationships/slicer",
      "crate::parts::slicers_part::SlicersPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "time_line_parts",
      "http://schemas.microsoft.com/office/2011/relationships/timeline",
      "crate::parts::time_line_part::TimeLinePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "worksheet_threaded_comments_parts",
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
      "crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "model3_d_reference_relationship_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
      "crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "named_sheet_views_parts",
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
      "crate::parts::named_sheet_views_part::NamedSheetViewsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
