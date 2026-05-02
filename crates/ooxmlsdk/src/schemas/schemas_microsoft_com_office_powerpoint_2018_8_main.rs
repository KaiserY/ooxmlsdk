//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CommentStatus {
  #[sdk(rename = "active")]
  #[default]
  Active,
  #[sdk(rename = "resolved")]
  Resolved,
  #[sdk(rename = "closed")]
  Closed,
}
/// Defines the TextBodyType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "a:CT_TextBody/p188:txBody")]
pub struct TextBodyType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Defines the CommentPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst"
)]
pub struct CommentPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Data for the Windows platform..
  #[sdk(child(office2021, qname = "p188:CT_CommentPropertiesExtension/p:ext"))]
  pub p_ext: Vec<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentPropertiesExtension,
  >,
}
/// Defines the AuthorList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_AuthorList/p188:authorLst")]
pub struct AuthorList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Author Class.
  #[sdk(child(office2021, qname = "p188:CT_Author/p188:author"))]
  pub p188_author: Vec<Author>,
}
/// Defines the CommentList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_CommentList/p188:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Comment Class.
  #[sdk(child(office2021, qname = "p188:CT_Comment/p188:cm"))]
  pub p188_cm: Vec<Comment>,
}
/// Defines the CommentRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_CommentRelationship/p188:commentRel")]
pub struct CommentRelationship {
  /// id
  #[sdk(attr(office2021, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p:CT_ExtensionList/p188:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the Author Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_Author/p188:author")]
pub struct Author {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// name
  #[sdk(attr(office2021, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// initials
  #[sdk(attr(office2021, qname = ":initials"))]
  pub initials: Option<crate::simple_type::StringValue>,
  /// userId
  #[sdk(attr(office2021, qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  #[sdk(attr(office2021, qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "p:CT_ExtensionList/p188:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentReply Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_CommentReply/p188:reply")]
pub struct CommentReply {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  #[sdk(attr(office2021, qname = ":status"))]
  #[sdk(string_format(kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  #[sdk(attr(office2021, qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  #[sdk(attr(office2021, qname = ":tags"))]
  pub tags: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// likes
  #[sdk(attr(office2021, qname = ":likes"))]
  pub likes: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// Defines the TextBodyType Class.
  #[sdk(child(office2021, qname = "a:CT_TextBody/p188:txBody"))]
  pub text_body_type: Option<std::boxed::Box<TextBodyType>>,
  /// Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(
    office2021,
    qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst"
  ))]
  pub comment_properties_extension_list: Option<CommentPropertiesExtensionList>,
}
/// Defines the Point2DType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "a:CT_Point2D/p188:pos")]
pub struct Point2DType {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the CommentReplyList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_CommentReplyList/p188:replyLst")]
pub struct CommentReplyList {
  /// Defines the CommentReply Class.
  #[sdk(child(office2021, qname = "p188:CT_CommentReply/p188:reply"))]
  pub p188_reply: Vec<CommentReply>,
}
/// Defines the Comment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_Comment/p188:cm")]
pub struct Comment {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  #[sdk(attr(office2021, qname = ":status"))]
  #[sdk(string_format(kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  #[sdk(attr(office2021, qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  #[sdk(attr(office2021, qname = ":tags"))]
  pub tags: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// likes
  #[sdk(attr(office2021, qname = ":likes"))]
  pub likes: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// startDate
  #[sdk(attr(office2021, qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// dueDate
  #[sdk(attr(office2021, qname = ":dueDate"))]
  pub due_date: Option<crate::simple_type::DateTimeValue>,
  /// assignedTo
  #[sdk(attr(office2021, qname = ":assignedTo"))]
  pub assigned_to: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// complete
  #[sdk(attr(office2021, qname = ":complete"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub complete: Option<crate::simple_type::Int32Value>,
  /// priority
  #[sdk(attr(office2021, qname = ":priority"))]
  #[sdk(number_range(range = 0..= 10))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// title
  #[sdk(attr(office2021, qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "pc:CT_SlideMonikerList/pc:sldMkLst",
    qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst",
    qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst",
    qname = "oac:CT_DrawingElementMonikerList/oac:deMkLst",
    qname = "oac:CT_TextBodyMonikerList/oac:txBodyMkLst",
    qname = "oac:CT_TextCharRangeMonikerList/oac:txMkLst",
    qname = "oac:CT_TableCellMonikerList/oac:tcMkLst",
    qname = "oac:CT_TableRowMonikerList/oac:trMkLst",
    qname = "oac:CT_TableColumnMonikerList/oac:gridColMkLst",
    qname = "p188:CT_CommentUnknownAnchor/p188:unknownAnchor"
  ))]
  pub comment_choice: Option<CommentChoice>,
  /// Defines the Point2DType Class.
  #[sdk(child(office2021, qname = "a:CT_Point2D/p188:pos"))]
  pub p188_pos: Option<Point2DType>,
  /// Defines the CommentReplyList Class.
  #[sdk(child(office2021, qname = "p188:CT_CommentReplyList/p188:replyLst"))]
  pub p188_reply_lst: Option<CommentReplyList>,
  /// Defines the TextBodyType Class.
  #[sdk(child(office2021, qname = "a:CT_TextBody/p188:txBody"))]
  pub p188_tx_body: Option<std::boxed::Box<TextBodyType>>,
  /// Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(
    office2021,
    qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst"
  ))]
  pub p188_ext_lst: Option<CommentPropertiesExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentChoice {
  #[sdk(child(office2016, qname = "pc:CT_SlideMonikerList/pc:sldMkLst"))]
    PcSldMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
        >,
    ),
    #[sdk(
        any_child(office2016, qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst")
    )]
    PcSldLayoutMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideLayoutMonikerList,
    ),
    #[sdk(
        any_child(office2016, qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst")
    )]
    PcSldMasterMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::MainMasterMonikerList,
    ),
    #[sdk(any_child(office2016, qname = "oac:CT_DrawingElementMonikerList/oac:deMkLst"))]
    OacDeMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DeMkLstDrawingElementMonikerList,
    ),
    #[sdk(any_child(office2016, qname = "oac:CT_TextBodyMonikerList/oac:txBodyMkLst"))]
    OacTxBodyMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextBodyMonikerList,
    ),
    #[sdk(any_child(office2016, qname = "oac:CT_TextCharRangeMonikerList/oac:txMkLst"))]
    OacTxMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextCharRangeMonikerList,
    ),
    #[sdk(any_child(office2016, qname = "oac:CT_TableCellMonikerList/oac:tcMkLst"))]
    OacTcMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableCellMonikerList,
    ),
    #[sdk(any_child(office2016, qname = "oac:CT_TableRowMonikerList/oac:trMkLst"))]
    OacTrMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableRowMonikerList,
    ),
    #[sdk(
        any_child(office2016, qname = "oac:CT_TableColumnMonikerList/oac:gridColMkLst")
    )]
    OacGridColMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableColumnMonikerList,
    ),
    /// Defines the CommentUnknownAnchor Class.
    #[sdk(
        empty_child(
            office2021,
            qname = "p188:CT_CommentUnknownAnchor/p188:unknownAnchor"
        )
    )]
    P188UnknownAnchor,
}
