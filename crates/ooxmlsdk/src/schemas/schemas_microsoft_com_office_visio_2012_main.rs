//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:DataConnections")]
pub struct DataConnections {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(attr(qname = ":NextID"))]
  pub next_id: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:DataConnection"))]
  pub data_connection: Vec<DataConnection>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:DataRecordSets")]
pub struct DataRecordSets {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(attr(qname = ":NextID"))]
  pub next_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ActiveRecordsetID"))]
  pub active_recordset_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DataWindowOrder"))]
  pub data_window_order: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:DataRecordSet"))]
  pub data_record_set: Vec<DataRecordSet>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:Extensions")]
pub struct Extensions {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:CellDef"))]
  pub cell_def: Vec<CellDef>,
  /// _
  #[sdk(child(qname = "visio:FunctionDef"))]
  pub function_def: Vec<FunctionDef>,
  /// _
  #[sdk(child(qname = "visio:SectionDef"))]
  pub section_def: Vec<SectionDef>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:MasterContents")]
pub struct MasterContents {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:Shapes"))]
  pub shapes: Option<Shapes>,
  /// _
  #[sdk(child(qname = "visio:Connects"))]
  pub connects: Option<Connects>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:Masters")]
pub struct Masters {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:Master"))]
  pub master: Vec<Master>,
  /// _
  #[sdk(child(qname = "visio:MasterShortcut"))]
  pub master_shortcut: Vec<MasterShortcut>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:PageContents")]
pub struct PageContents {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:Shapes"))]
  pub shapes: Option<Shapes>,
  /// _
  #[sdk(child(qname = "visio:Connects"))]
  pub connects: Option<Connects>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:Pages")]
pub struct Pages {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:Page"))]
  pub page: Vec<Page>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:SolutionXML")]
pub struct SolutionXml {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Solutions")]
pub struct Solutions {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:Solution"))]
  pub solution: Vec<Solution>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Validation")]
pub struct Validation {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// _
  #[sdk(child(qname = "visio:ValidationProperties"))]
  pub validation_properties: Option<ValidationProperties>,
  /// _
  #[sdk(child(qname = "visio:RuleSets"))]
  pub rule_sets: Option<RuleSets>,
  /// _
  #[sdk(child(qname = "visio:Issues"))]
  pub issues: Option<Issues>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "visio:VisioDocument")]
pub struct VisioDocument {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":metric"))]
  pub metric: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":buildnum"))]
  pub buildnum: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":DocLangID"))]
  pub doc_lang_id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:DocumentProperties"))]
  pub document_properties: Option<DocumentProperties>,
  /// _
  #[sdk(child(qname = "visio:DocumentSettings"))]
  pub document_settings: Option<DocumentSettings>,
  /// _
  #[sdk(child(qname = "visio:Colors"))]
  pub colors: Option<Colors>,
  /// _
  #[sdk(child(qname = "visio:FaceNames"))]
  pub face_names: Option<FaceNames>,
  /// _
  #[sdk(child(qname = "visio:StyleSheets"))]
  pub style_sheets: Option<StyleSheets>,
  /// _
  #[sdk(child(qname = "visio:DocumentSheet"))]
  pub document_sheet: Option<DocumentSheet>,
  /// _
  #[sdk(child(qname = "visio:EventList"))]
  pub event_list: Option<EventList>,
  /// _
  #[sdk(child(qname = "visio:HeaderFooter"))]
  pub header_footer: Option<HeaderFooter>,
  /// _
  #[sdk(child(qname = "visio:DataTransferInfo"))]
  pub data_transfer_info: Option<DataTransferInfo>,
  /// _
  #[sdk(child(qname = "visio:PublishSettings"))]
  pub publish_settings: Option<PublishSettings>,
  /// _
  #[sdk(child(qname = "visio:Comments"))]
  pub comments: Option<std::boxed::Box<Comments>>,
  /// _
  #[sdk(any)]
  pub unknown_xml: Vec<std::boxed::Box<[u8]>>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Windows")]
pub struct Windows {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(attr(qname = ":ClientWidth"))]
  pub client_width: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":ClientHeight"))]
  pub client_height: Option<crate::simple_type::UInt16Value>,
  /// _
  #[sdk(child(qname = "visio:Window"))]
  pub window: Vec<Window>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Cell")]
pub struct Cell {
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":U"))]
  pub u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":E"))]
  pub e: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":F"))]
  pub f: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":V"))]
  pub v: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:RefBy"))]
  pub ref_by: Vec<RefBy>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ActionTagRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ActionsRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct AnnotationRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:AuthorEntry")]
pub struct AuthorEntry {
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Initials"))]
  pub initials: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":SIP"))]
  pub sip: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":SMTP"))]
  pub smtp: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ResolutionID"))]
  pub resolution_id: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RefBy")]
pub struct RefBy {
  #[sdk(attr(qname = ":T"))]
  pub t: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct CharacterRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:ColorEntry")]
pub struct ColorEntry {
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":RGB"))]
  pub rgb: crate::simple_type::StringValue,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:CommentEntry")]
pub struct CommentEntry {
  #[sdk(attr(qname = ":AuthorID"))]
  pub author_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":PageID"))]
  pub page_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Date"))]
  pub date: crate::simple_type::DateTimeValue,
  #[sdk(attr(qname = ":EditDate"))]
  pub edit_date: Option<crate::simple_type::DateTimeValue>,
  #[sdk(attr(qname = ":Done"))]
  pub done: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":CommentID"))]
  pub comment_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":AutoCommentType"))]
  pub auto_comment_type: Option<crate::simple_type::UInt32Value>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:AuthorList")]
pub struct AuthorList {
  /// _
  #[sdk(child(qname = "visio:AuthorEntry"))]
  pub author_entry: Vec<AuthorEntry>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:CommentList")]
pub struct CommentList {
  /// _
  #[sdk(child(qname = "visio:CommentEntry"))]
  pub comment_entry: Vec<CommentEntry>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ConnectionAbcdRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ConnectionRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Connect")]
pub struct Connect {
  #[sdk(attr(qname = ":FromSheet"))]
  pub from_sheet: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":FromCell"))]
  pub from_cell: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":FromPart"))]
  pub from_part: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":ToSheet"))]
  pub to_sheet: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ToCell"))]
  pub to_cell: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":ToPart"))]
  pub to_part: Option<crate::simple_type::Int32Value>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ControlRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:CustomProp")]
pub struct CustomProp {
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":PropType"))]
  pub prop_type: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DataColumn")]
pub struct DataColumn {
  #[sdk(attr(qname = ":ColumnNameID"))]
  pub column_name_id: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":Name"))]
  pub name: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":Label"))]
  pub label: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":OrigLabel"))]
  pub orig_label: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LangID"))]
  pub lang_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Calendar"))]
  pub calendar: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":DataType"))]
  pub data_type: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":UnitType"))]
  pub unit_type: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Currency"))]
  pub currency: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":Degree"))]
  pub degree: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DisplayWidth"))]
  pub display_width: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DisplayOrder"))]
  pub display_order: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Mapped"))]
  pub mapped: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":Hyperlink"))]
  pub hyperlink: Option<crate::simple_type::BooleanValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DataConnection")]
pub struct DataConnection {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":FileName"))]
  pub file_name: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":ConnectionString"))]
  pub connection_string: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Command"))]
  pub command: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":FriendlyName"))]
  pub friendly_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Timeout"))]
  pub timeout: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":AlwaysUseConnectionFile"))]
  pub always_use_connection_file: Option<crate::simple_type::BooleanValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Rel")]
pub struct Rel {
  /// Relationship identifier.
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DataColumns")]
pub struct DataColumns {
  #[sdk(attr(qname = ":SortColumn"))]
  pub sort_column: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":SortAsc"))]
  pub sort_asc: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "visio:DataColumn"))]
  pub data_column: Vec<DataColumn>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:PrimaryKey")]
pub struct PrimaryKey {
  #[sdk(attr(qname = ":ColumnNameID"))]
  pub column_name_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "visio:RowKeyValue"))]
  pub row_key_value: Vec<RowKeyValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RowMap")]
pub struct RowMap {
  #[sdk(attr(qname = ":RowID"))]
  pub row_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":PageID"))]
  pub page_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RefreshConflict")]
pub struct RefreshConflict {
  #[sdk(attr(qname = ":RowID"))]
  pub row_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":PageID"))]
  pub page_id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:AutoLinkComparison")]
pub struct AutoLinkComparison {
  #[sdk(attr(qname = ":ColumnName"))]
  pub column_name: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":ContextType"))]
  pub context_type: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ContextTypeLabel"))]
  pub context_type_label: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DataRecordSet")]
pub struct DataRecordSet {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ConnectionID"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Command"))]
  pub command: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Options"))]
  pub options: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TimeRefreshed"))]
  pub time_refreshed: Option<crate::simple_type::DateTimeValue>,
  #[sdk(attr(qname = ":NextRowID"))]
  pub next_row_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":RowOrder"))]
  pub row_order: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":RefreshOverwriteAll"))]
  pub refresh_overwrite_all: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":RefreshNoReconciliationUI"))]
  pub refresh_no_reconciliation_ui: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":RefreshInterval"))]
  pub refresh_interval: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ReplaceLinks"))]
  pub replace_links: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Checksum"))]
  pub checksum: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(empty_child(qname = "visio:ADOData"))]
  pub ado_data: Option<()>,
  /// _
  #[sdk(child(qname = "visio:Rel"))]
  pub rel: Rel,
  /// _
  #[sdk(child(qname = "visio:DataColumns"))]
  pub data_columns: std::boxed::Box<DataColumns>,
  /// _
  #[sdk(child(qname = "visio:PrimaryKey"))]
  pub primary_key: Vec<PrimaryKey>,
  /// _
  #[sdk(child(qname = "visio:RowMap"))]
  pub row_map: Vec<RowMap>,
  /// _
  #[sdk(child(qname = "visio:RefreshConflict"))]
  pub refresh_conflict: Vec<RefreshConflict>,
  /// _
  #[sdk(child(qname = "visio:AutoLinkComparison"))]
  pub auto_link_comparison: Vec<AutoLinkComparison>,
}
///
pub type Title = crate::simple_type::StringValue;
///
pub type Subject = crate::simple_type::StringValue;
///
pub type Creator = crate::simple_type::StringValue;
///
pub type Manager = crate::simple_type::StringValue;
///
pub type Company = crate::simple_type::StringValue;
///
pub type Category = crate::simple_type::StringValue;
///
pub type Keywords = crate::simple_type::StringValue;
///
pub type Desc = crate::simple_type::StringValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:HyperlinkBase")]
pub struct HyperlinkBase {
  #[sdk(attr(qname = ":href"))]
  pub href: crate::simple_type::StringValue,
}
///
pub type AlternateNames = crate::simple_type::StringValue;
///
pub type Template = crate::simple_type::StringValue;
///
pub type BuildNumberCreated = crate::simple_type::Int32Value;
///
pub type BuildNumberEdited = crate::simple_type::Int32Value;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:PreviewPicture")]
pub struct PreviewPicture {
  #[sdk(attr(qname = ":Size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Base64BinaryValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:CustomProps")]
pub struct CustomProps {
  /// _
  #[sdk(child(qname = "visio:CustomProp"))]
  pub custom_prop: Vec<CustomProp>,
}
///
pub type TimeCreated = crate::simple_type::DateTimeValue;
///
pub type TimeSaved = crate::simple_type::DateTimeValue;
///
pub type TimeEdited = crate::simple_type::DateTimeValue;
///
pub type TimePrinted = crate::simple_type::DateTimeValue;
///
pub type GlueSettings = crate::simple_type::Int32Value;
///
pub type SnapSettings = crate::simple_type::Int32Value;
///
pub type SnapExtensions = crate::simple_type::Int32Value;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:SnapAngles")]
pub struct SnapAngles {
  /// _
  #[sdk(text_child(qname = "visio:SnapAngle"))]
  pub snap_angle: Vec<SnapAngle>,
}
///
pub type DynamicGridEnabled = crate::simple_type::BooleanValue;
///
pub type ProtectStyles = crate::simple_type::BooleanValue;
///
pub type ProtectShapes = crate::simple_type::BooleanValue;
///
pub type ProtectMasters = crate::simple_type::BooleanValue;
///
pub type ProtectBkgnds = crate::simple_type::BooleanValue;
///
pub type CustomMenusFile = crate::simple_type::StringValue;
///
pub type CustomToolbarsFile = crate::simple_type::StringValue;
///
pub type AttachedToolbars = crate::simple_type::Base64BinaryValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:EventItem")]
pub struct EventItem {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Action"))]
  pub action: crate::simple_type::UInt16Value,
  #[sdk(attr(qname = ":EventCode"))]
  pub event_code: crate::simple_type::UInt16Value,
  #[sdk(attr(qname = ":Enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":Target"))]
  pub target: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":TargetArgs"))]
  pub target_args: crate::simple_type::StringValue,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:CellDef")]
pub struct CellDef {
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":T"))]
  pub t: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":F"))]
  pub f: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":S"))]
  pub s: Option<crate::simple_type::ByteValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:FunctionDef")]
pub struct FunctionDef {
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:SectionDef")]
pub struct SectionDef {
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":T"))]
  pub t: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":S"))]
  pub s: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:CellDef"))]
  pub cell_def: Vec<CellDef>,
  /// _
  #[sdk(child(qname = "visio:RowDef"))]
  pub row_def: Option<RowDef>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:FaceName")]
pub struct FaceName {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Name"))]
  pub name: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":UnicodeRanges"))]
  pub unicode_ranges: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":CharSets"))]
  pub char_sets: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Panose"))]
  pub panose: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Flags"))]
  pub flags: Option<crate::simple_type::UInt32Value>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct FieldRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct FillGradientRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct GeometryRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":T"))]
  pub t: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:HeaderMargin")]
pub struct HeaderMargin {
  #[sdk(attr(qname = ":Unit"))]
  pub unit: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:FooterMargin")]
pub struct FooterMargin {
  #[sdk(attr(qname = ":Unit"))]
  pub unit: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
///
pub type HeaderLeft = crate::simple_type::StringValue;
///
pub type HeaderCenter = crate::simple_type::StringValue;
///
pub type HeaderRight = crate::simple_type::StringValue;
///
pub type FooterLeft = crate::simple_type::StringValue;
///
pub type FooterCenter = crate::simple_type::StringValue;
///
pub type FooterRight = crate::simple_type::StringValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:HeaderFooterFont")]
pub struct HeaderFooterFont {
  #[sdk(attr(qname = ":Height"))]
  pub height: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Width"))]
  pub width: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Escapement"))]
  pub escapement: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Orientation"))]
  pub orientation: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Weight"))]
  pub weight: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Italic"))]
  pub italic: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":Underline"))]
  pub underline: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":StrikeOut"))]
  pub strike_out: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":CharSet"))]
  pub char_set: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":OutPrecision"))]
  pub out_precision: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":ClipPrecision"))]
  pub clip_precision: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":Quality"))]
  pub quality: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":PitchAndFamily"))]
  pub pitch_and_family: Option<crate::simple_type::ByteValue>,
  #[sdk(attr(qname = ":FaceName"))]
  pub face_name: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct HyperlinkRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:IssueTarget")]
pub struct IssueTarget {
  #[sdk(attr(qname = ":PageID"))]
  pub page_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":ShapeID"))]
  pub shape_id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RuleInfo")]
pub struct RuleInfo {
  #[sdk(attr(qname = ":RuleSetID"))]
  pub rule_set_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":RuleID"))]
  pub rule_id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Issue")]
pub struct Issue {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Ignored"))]
  pub ignored: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "visio:IssueTarget"))]
  pub issue_target: Option<IssueTarget>,
  /// _
  #[sdk(child(qname = "visio:RuleInfo"))]
  pub rule_info: Option<RuleInfo>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct LayerRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct LineGradientRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Shapes")]
pub struct Shapes {
  /// _
  #[sdk(child(qname = "visio:Shape"))]
  pub shape: Vec<Shape>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Connects")]
pub struct Connects {
  /// _
  #[sdk(child(qname = "visio:Connect"))]
  pub connect: Vec<Connect>,
}
///
pub type Icon = crate::simple_type::Base64BinaryValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:PageSheet")]
pub struct PageSheet {
  #[sdk(attr(qname = ":LineStyle"))]
  pub line_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":FillStyle"))]
  pub fill_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TextStyle"))]
  pub text_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":UniqueID"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = Text, qname = "visio:Text"),
            text_child(variant = Data1, qname = "visio:Data1"),
            text_child(variant = Data2, qname = "visio:Data2"),
            text_child(variant = Data3, qname = "visio:Data3"),
            child(variant = ForeignData, boxed, qname = "visio:ForeignData"),
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = Section, qname = "visio:Section"),
            any
        )
    )]
  pub page_sheet_choice: Vec<PageSheetChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Master")]
pub struct Master {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":BaseID"))]
  pub base_id: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":UniqueID"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":MatchByName"))]
  pub match_by_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IconSize"))]
  pub icon_size: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":PatternFlags"))]
  pub pattern_flags: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":Prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IconUpdate"))]
  pub icon_update: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":AlignName"))]
  pub align_name: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":MasterType"))]
  pub master_type: Option<crate::simple_type::UInt16Value>,
  #[sdk(
        choice(
            child(variant = PageSheet, boxed, qname = "visio:PageSheet"),
            child(variant = Rel, qname = "visio:Rel"),
            text_child(variant = Icon, qname = "visio:Icon")
        )
    )]
  pub master_choice: Vec<MasterChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:MasterShortcut")]
pub struct MasterShortcut {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IconSize"))]
  pub icon_size: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":PatternFlags"))]
  pub pattern_flags: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":Prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":ShortcutURL"))]
  pub shortcut_url: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":ShortcutHelp"))]
  pub shortcut_help: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":AlignName"))]
  pub align_name: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":MasterType"))]
  pub master_type: Option<crate::simple_type::UInt16Value>,
  /// _
  #[sdk(text_child(qname = "visio:Icon"))]
  pub icon: Icon,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Page")]
pub struct Page {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":Background"))]
  pub background: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":BackPage"))]
  pub back_page: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ViewScale"))]
  pub view_scale: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ViewCenterX"))]
  pub view_center_x: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ViewCenterY"))]
  pub view_center_y: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ReviewerID"))]
  pub reviewer_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":AssociatedPage"))]
  pub associated_page: Option<crate::simple_type::UInt32Value>,
  #[sdk(
        choice(
            child(variant = PageSheet, boxed, qname = "visio:PageSheet"),
            child(variant = Rel, qname = "visio:Rel")
        )
    )]
  pub page_choice: Vec<PageChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ParagraphRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RowKeyValue")]
pub struct RowKeyValue {
  #[sdk(attr(qname = ":RowID"))]
  pub row_id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Value"))]
  pub value: crate::simple_type::StringValue,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct PropertyRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:PublishedPage")]
pub struct PublishedPage {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RefreshableData")]
pub struct RefreshableData {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ReviewerRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RuleSetFlags")]
pub struct RuleSetFlags {
  #[sdk(attr(qname = ":Hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Rule")]
pub struct Rule {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":Category"))]
  pub category: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Description"))]
  pub description: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":RuleTarget"))]
  pub rule_target: Option<crate::simple_type::Int32Value>,
  #[sdk(attr(qname = ":Ignored"))]
  pub ignored: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(qname = "visio:RuleFilter"))]
  pub rule_filter: Option<RuleFilter>,
  /// _
  #[sdk(text_child(qname = "visio:RuleTest"))]
  pub rule_test: Option<RuleTest>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RuleSet")]
pub struct RuleSet {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":Description"))]
  pub description: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Enabled"))]
  pub enabled: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "visio:RuleSetFlags"))]
  pub rule_set_flags: Option<RuleSetFlags>,
  /// _
  #[sdk(child(qname = "visio:Rule"))]
  pub rule: Vec<Rule>,
}
///
pub type RuleFilter = crate::simple_type::StringValue;
///
pub type RuleTest = crate::simple_type::StringValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct ScratchRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RowDef")]
pub struct RowDef {
  /// _
  #[sdk(child(qname = "visio:CellDef"))]
  pub cell_def: Vec<CellDef>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Text")]
pub struct Text {
  #[sdk(
        choice(
            child(variant = Cp, qname = "visio:cp"),
            child(variant = Pp, qname = "visio:pp"),
            child(variant = Tp, qname = "visio:tp"),
            child(variant = Fld, qname = "visio:fld"),
            text
        )
    )]
  pub text_choice: Vec<TextChoice>,
}
///
pub type Data1 = crate::simple_type::StringValue;
///
pub type Data2 = crate::simple_type::StringValue;
///
pub type Data3 = crate::simple_type::StringValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:ForeignData")]
pub struct ForeignData {
  #[sdk(attr(qname = ":ForeignType"))]
  pub foreign_type: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":ObjectType"))]
  pub object_type: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ShowAsIcon"))]
  pub show_as_icon: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":ObjectWidth"))]
  pub object_width: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ObjectHeight"))]
  pub object_height: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":MappingMode"))]
  pub mapping_mode: Option<crate::simple_type::UInt16Value>,
  #[sdk(attr(qname = ":ExtentX"))]
  pub extent_x: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ExtentY"))]
  pub extent_y: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":CompressionType"))]
  pub compression_type: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":CompressionLevel"))]
  pub compression_level: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "visio:Rel"))]
  pub rel: Rel,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct RowUnion {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":T"))]
  pub t: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = Tab, qname = "visio:Tab")
        )
    )]
  pub row_union_choice: Vec<RowUnionChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Section")]
pub struct Section {
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  #[sdk(
        choice(
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = RowUnion, boxed, qname = "visio:Row")
        )
    )]
  pub section_choice: Vec<SectionChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Shape")]
pub struct Shape {
  #[sdk(attr(qname = ":LineStyle"))]
  pub line_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":FillStyle"))]
  pub fill_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TextStyle"))]
  pub text_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":OriginalID"))]
  pub original_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":MasterShape"))]
  pub master_shape: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":UniqueID"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":Master"))]
  pub master: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = Text, qname = "visio:Text"),
            text_child(variant = Data1, qname = "visio:Data1"),
            text_child(variant = Data2, qname = "visio:Data2"),
            text_child(variant = Data3, qname = "visio:Data3"),
            child(variant = ForeignData, boxed, qname = "visio:ForeignData"),
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = Section, qname = "visio:Section"),
            any
        )
    )]
  pub shape_choice: Vec<ShapeChoice>,
  /// _
  #[sdk(child(qname = "visio:Shapes"))]
  pub shapes: Option<Shapes>,
}
///
pub type SnapAngle = crate::simple_type::DoubleValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Solution")]
pub struct Solution {
  #[sdk(attr(qname = ":Name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "visio:Rel"))]
  pub rel: Rel,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:StyleSheet")]
pub struct StyleSheet {
  #[sdk(attr(qname = ":LineStyle"))]
  pub line_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":FillStyle"))]
  pub fill_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TextStyle"))]
  pub text_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = Text, qname = "visio:Text"),
            text_child(variant = Data1, qname = "visio:Data1"),
            text_child(variant = Data2, qname = "visio:Data2"),
            text_child(variant = Data3, qname = "visio:Data3"),
            child(variant = ForeignData, boxed, qname = "visio:ForeignData"),
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = Section, qname = "visio:Section"),
            any
        )
    )]
  pub style_sheet_choice: Vec<StyleSheetChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Tab")]
pub struct Tab {
  #[sdk(attr(qname = ":IX"))]
  pub ix: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: Vec<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct TabsRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "visio:Tab"))]
  pub tab: Vec<Tab>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:cp")]
pub struct Cp {
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:pp")]
pub struct Pp {
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:tp")]
pub struct Tp {
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:fld")]
pub struct Fld {
  #[sdk(attr(qname = ":IX"))]
  pub ix: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Row")]
pub struct UserRow {
  #[sdk(attr(qname = ":Del"))]
  pub del: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":N"))]
  pub n: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":LocalName"))]
  pub local_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "visio:Cell"))]
  pub cell: std::boxed::Box<Cell>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:ValidationProperties")]
pub struct ValidationProperties {
  #[sdk(attr(qname = ":LastValidated"))]
  pub last_validated: crate::simple_type::DateTimeValue,
  #[sdk(attr(qname = ":ShowIgnored"))]
  pub show_ignored: crate::simple_type::BooleanValue,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:RuleSets")]
pub struct RuleSets {
  /// _
  #[sdk(child(qname = "visio:RuleSet"))]
  pub rule_set: Vec<RuleSet>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Issues")]
pub struct Issues {
  /// _
  #[sdk(child(qname = "visio:Issue"))]
  pub issue: Vec<Issue>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DocumentProperties")]
pub struct DocumentProperties {
  #[sdk(
        choice(
            text_child(variant = Title, qname = "visio:Title"),
            text_child(variant = Subject, qname = "visio:Subject"),
            text_child(variant = Creator, qname = "visio:Creator"),
            text_child(variant = Manager, qname = "visio:Manager"),
            text_child(variant = Company, qname = "visio:Company"),
            text_child(variant = Category, qname = "visio:Category"),
            text_child(variant = Keywords, qname = "visio:Keywords"),
            text_child(variant = Desc, qname = "visio:Desc"),
            child(variant = HyperlinkBase, qname = "visio:HyperlinkBase"),
            text_child(variant = AlternateNames, qname = "visio:AlternateNames"),
            text_child(variant = Template, qname = "visio:Template"),
            text_child(variant = BuildNumberCreated, qname = "visio:BuildNumberCreated"),
            text_child(variant = BuildNumberEdited, qname = "visio:BuildNumberEdited"),
            child(variant = PreviewPicture, qname = "visio:PreviewPicture"),
            child(variant = CustomProps, qname = "visio:CustomProps"),
            text_child(variant = TimeCreated, qname = "visio:TimeCreated"),
            text_child(variant = TimeSaved, qname = "visio:TimeSaved"),
            text_child(variant = TimeEdited, qname = "visio:TimeEdited"),
            text_child(variant = TimePrinted, qname = "visio:TimePrinted")
        )
    )]
  pub document_properties_choice: Vec<DocumentPropertiesChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DocumentSettings")]
pub struct DocumentSettings {
  #[sdk(attr(qname = ":TopPage"))]
  pub top_page: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DefaultTextStyle"))]
  pub default_text_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DefaultLineStyle"))]
  pub default_line_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DefaultFillStyle"))]
  pub default_fill_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":DefaultGuideStyle"))]
  pub default_guide_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(
        choice(
            text_child(variant = GlueSettings, qname = "visio:GlueSettings"),
            text_child(variant = SnapSettings, qname = "visio:SnapSettings"),
            text_child(variant = SnapExtensions, qname = "visio:SnapExtensions"),
            child(variant = SnapAngles, qname = "visio:SnapAngles"),
            text_child(variant = DynamicGridEnabled, qname = "visio:DynamicGridEnabled"),
            text_child(variant = ProtectStyles, qname = "visio:ProtectStyles"),
            text_child(variant = ProtectShapes, qname = "visio:ProtectShapes"),
            text_child(variant = ProtectMasters, qname = "visio:ProtectMasters"),
            text_child(variant = ProtectBkgnds, qname = "visio:ProtectBkgnds"),
            text_child(variant = CustomMenusFile, qname = "visio:CustomMenusFile"),
            text_child(variant = CustomToolbarsFile, qname = "visio:CustomToolbarsFile"),
            text_child(variant = AttachedToolbars, qname = "visio:AttachedToolbars")
        )
    )]
  pub document_settings_choice: Vec<DocumentSettingsChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Colors")]
pub struct Colors {
  /// _
  #[sdk(child(qname = "visio:ColorEntry"))]
  pub color_entry: Vec<ColorEntry>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:FaceNames")]
pub struct FaceNames {
  /// _
  #[sdk(child(qname = "visio:FaceName"))]
  pub face_name: Vec<FaceName>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:StyleSheets")]
pub struct StyleSheets {
  /// _
  #[sdk(child(qname = "visio:StyleSheet"))]
  pub style_sheet: Vec<StyleSheet>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DocumentSheet")]
pub struct DocumentSheet {
  #[sdk(attr(qname = ":LineStyle"))]
  pub line_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":FillStyle"))]
  pub fill_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TextStyle"))]
  pub text_style: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":NameU"))]
  pub name_u: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":IsCustomName"))]
  pub is_custom_name: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":IsCustomNameU"))]
  pub is_custom_name_u: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":UniqueID"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = Text, qname = "visio:Text"),
            text_child(variant = Data1, qname = "visio:Data1"),
            text_child(variant = Data2, qname = "visio:Data2"),
            text_child(variant = Data3, qname = "visio:Data3"),
            child(variant = ForeignData, boxed, qname = "visio:ForeignData"),
            child(variant = Cell, boxed, qname = "visio:Cell"),
            child(variant = Section, qname = "visio:Section"),
            any
        )
    )]
  pub document_sheet_choice: Vec<DocumentSheetChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:EventList")]
pub struct EventList {
  /// _
  #[sdk(child(qname = "visio:EventItem"))]
  pub event_item: Vec<EventItem>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:HeaderFooter")]
pub struct HeaderFooter {
  #[sdk(attr(qname = ":HeaderFooterColor"))]
  pub header_footer_color: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = HeaderMargin, qname = "visio:HeaderMargin"),
            child(variant = FooterMargin, qname = "visio:FooterMargin"),
            text_child(variant = HeaderLeft, qname = "visio:HeaderLeft"),
            text_child(variant = HeaderCenter, qname = "visio:HeaderCenter"),
            text_child(variant = HeaderRight, qname = "visio:HeaderRight"),
            text_child(variant = FooterLeft, qname = "visio:FooterLeft"),
            text_child(variant = FooterCenter, qname = "visio:FooterCenter"),
            text_child(variant = FooterRight, qname = "visio:FooterRight"),
            child(variant = HeaderFooterFont, boxed, qname = "visio:HeaderFooterFont")
        )
    )]
  pub header_footer_choice: Vec<HeaderFooterChoice>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:DataTransferInfo")]
pub struct DataTransferInfo {
  #[sdk(attr(qname = ":Context"))]
  pub context: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":ContainerType"))]
  pub container_type: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Container"))]
  pub container: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":View"))]
  pub view: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Sheet"))]
  pub sheet: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":TransferType"))]
  pub transfer_type: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":TransferTime"))]
  pub transfer_time: Option<crate::simple_type::UInt32Value>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:PublishSettings")]
pub struct PublishSettings {
  /// _
  #[sdk(child(qname = "visio:PublishedPage"))]
  pub published_page: Vec<PublishedPage>,
  /// _
  #[sdk(child(qname = "visio:RefreshableData"))]
  pub refreshable_data: Vec<RefreshableData>,
}
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Comments")]
pub struct Comments {
  #[sdk(attr(qname = ":ShowCommentTags"))]
  pub show_comment_tags: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "visio:AuthorList"))]
  pub author_list: Option<AuthorList>,
  /// _
  #[sdk(child(qname = "visio:CommentList"))]
  pub comment_list: Option<CommentList>,
}
///
pub type StencilGroup = crate::simple_type::Int32Value;
///
pub type StencilGroupPos = crate::simple_type::Int32Value;
///
pub type ShowRulers = crate::simple_type::BooleanValue;
///
pub type ShowGrid = crate::simple_type::BooleanValue;
///
pub type ShowPageBreaks = crate::simple_type::BooleanValue;
///
pub type ShowGuides = crate::simple_type::BooleanValue;
///
pub type ShowConnectionPoints = crate::simple_type::BooleanValue;
///
pub type TabSplitterPos = crate::simple_type::DoubleValue;
///
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "visio:Window")]
pub struct Window {
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(attr(qname = ":WindowType"))]
  pub window_type: crate::simple_type::StringValue,
  #[sdk(attr(qname = ":WindowState"))]
  pub window_state: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Document"))]
  pub document: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":WindowLeft"))]
  pub window_left: Option<crate::simple_type::Int16Value>,
  #[sdk(attr(qname = ":WindowTop"))]
  pub window_top: Option<crate::simple_type::Int16Value>,
  #[sdk(attr(qname = ":WindowWidth"))]
  pub window_width: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":WindowHeight"))]
  pub window_height: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Master"))]
  pub master: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ContainerType"))]
  pub container_type: Option<crate::simple_type::StringValue>,
  #[sdk(attr(qname = ":Container"))]
  pub container: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Sheet"))]
  pub sheet: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ReadOnly"))]
  pub read_only: Option<crate::simple_type::BooleanValue>,
  #[sdk(attr(qname = ":ParentWindow"))]
  pub parent_window: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":Page"))]
  pub page: Option<crate::simple_type::UInt32Value>,
  #[sdk(attr(qname = ":ViewScale"))]
  pub view_scale: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ViewCenterX"))]
  pub view_center_x: Option<crate::simple_type::DoubleValue>,
  #[sdk(attr(qname = ":ViewCenterY"))]
  pub view_center_y: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(text_child(qname = "visio:StencilGroup"))]
  pub stencil_group: Option<StencilGroup>,
  /// _
  #[sdk(text_child(qname = "visio:StencilGroupPos"))]
  pub stencil_group_pos: Option<StencilGroupPos>,
  /// _
  #[sdk(text_child(qname = "visio:ShowRulers"))]
  pub show_rulers: Option<ShowRulers>,
  /// _
  #[sdk(text_child(qname = "visio:ShowGrid"))]
  pub show_grid: Option<ShowGrid>,
  /// _
  #[sdk(text_child(qname = "visio:ShowPageBreaks"))]
  pub show_page_breaks: Option<ShowPageBreaks>,
  /// _
  #[sdk(text_child(qname = "visio:ShowGuides"))]
  pub show_guides: Option<ShowGuides>,
  /// _
  #[sdk(text_child(qname = "visio:ShowConnectionPoints"))]
  pub show_connection_points: Option<ShowConnectionPoints>,
  /// _
  #[sdk(text_child(qname = "visio:GlueSettings"))]
  pub glue_settings: Option<GlueSettings>,
  /// _
  #[sdk(text_child(qname = "visio:SnapSettings"))]
  pub snap_settings: Option<SnapSettings>,
  /// _
  #[sdk(text_child(qname = "visio:SnapExtensions"))]
  pub snap_extensions: Option<SnapExtensions>,
  /// _
  #[sdk(child(qname = "visio:SnapAngles"))]
  pub snap_angles: Option<SnapAngles>,
  /// _
  #[sdk(text_child(qname = "visio:DynamicGridEnabled"))]
  pub dynamic_grid_enabled: Option<DynamicGridEnabled>,
  /// _
  #[sdk(text_child(qname = "visio:TabSplitterPos"))]
  pub tab_splitter_pos: Option<TabSplitterPos>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PageSheetChoice {
  Text(Text),
  Data1(Data1),
  Data2(Data2),
  Data3(Data3),
  ForeignData(std::boxed::Box<ForeignData>),
  Cell(std::boxed::Box<Cell>),
  Section(Section),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MasterChoice {
  PageSheet(std::boxed::Box<PageSheet>),
  Rel(Rel),
  Icon(Icon),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PageChoice {
  PageSheet(std::boxed::Box<PageSheet>),
  Rel(Rel),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextChoice {
  Cp(Cp),
  Pp(Pp),
  Tp(Tp),
  Fld(Fld),
  /// Character data between text marks.
  Text(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RowUnionChoice {
  Cell(std::boxed::Box<Cell>),
  Tab(Tab),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SectionChoice {
  Cell(std::boxed::Box<Cell>),
  RowUnion(std::boxed::Box<RowUnion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeChoice {
  Text(Text),
  Data1(Data1),
  Data2(Data2),
  Data3(Data3),
  ForeignData(std::boxed::Box<ForeignData>),
  Cell(std::boxed::Box<Cell>),
  Section(Section),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum StyleSheetChoice {
  Text(Text),
  Data1(Data1),
  Data2(Data2),
  Data3(Data3),
  ForeignData(std::boxed::Box<ForeignData>),
  Cell(std::boxed::Box<Cell>),
  Section(Section),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentPropertiesChoice {
  Title(Title),
  Subject(Subject),
  Creator(Creator),
  Manager(Manager),
  Company(Company),
  Category(Category),
  Keywords(Keywords),
  Desc(Desc),
  HyperlinkBase(HyperlinkBase),
  AlternateNames(AlternateNames),
  Template(Template),
  BuildNumberCreated(BuildNumberCreated),
  BuildNumberEdited(BuildNumberEdited),
  PreviewPicture(PreviewPicture),
  CustomProps(CustomProps),
  TimeCreated(TimeCreated),
  TimeSaved(TimeSaved),
  TimeEdited(TimeEdited),
  TimePrinted(TimePrinted),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentSettingsChoice {
  GlueSettings(GlueSettings),
  SnapSettings(SnapSettings),
  SnapExtensions(SnapExtensions),
  SnapAngles(SnapAngles),
  DynamicGridEnabled(DynamicGridEnabled),
  ProtectStyles(ProtectStyles),
  ProtectShapes(ProtectShapes),
  ProtectMasters(ProtectMasters),
  ProtectBkgnds(ProtectBkgnds),
  CustomMenusFile(CustomMenusFile),
  CustomToolbarsFile(CustomToolbarsFile),
  AttachedToolbars(AttachedToolbars),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentSheetChoice {
  Text(Text),
  Data1(Data1),
  Data2(Data2),
  Data3(Data3),
  ForeignData(std::boxed::Box<ForeignData>),
  Cell(std::boxed::Box<Cell>),
  Section(Section),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HeaderFooterChoice {
  HeaderMargin(HeaderMargin),
  FooterMargin(FooterMargin),
  HeaderLeft(HeaderLeft),
  HeaderCenter(HeaderCenter),
  HeaderRight(HeaderRight),
  FooterLeft(FooterLeft),
  FooterCenter(FooterCenter),
  FooterRight(FooterRight),
  HeaderFooterFont(std::boxed::Box<HeaderFooterFont>),
}
