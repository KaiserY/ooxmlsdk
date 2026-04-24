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
/// Defines the CommentUnknownAnchor Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:unknownAnchor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentUnknownAnchor/p188:unknownAnchor")]
pub struct CommentUnknownAnchor {}
/// Defines the TextBodyType Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:txBody.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/p188:txBody")]
pub struct TextBodyType {
  ///Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ///Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Defines the CommentPropertiesExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst")]
pub struct CommentPropertiesExtensionList {
  ///Data for the Windows platform..
  #[sdk(child(qname = "p188:CT_CommentPropertiesExtension/p:ext"))]
  pub comment_properties_extension: Vec<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentPropertiesExtension,
  >,
}
/// Defines the AuthorList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:authorLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_AuthorList/p188:authorLst")]
pub struct AuthorList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p188:CT_Author/p188:author"))]
  pub p188_author: Vec<Author>,
}
/// Defines the CommentList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:cmLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentList/p188:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p188:CT_Comment/p188:cm"))]
  pub p188_cm: Vec<Comment>,
}
/// Defines the CommentRelationship Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:commentRel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentRelationship/p188:commentRel")]
pub struct CommentRelationship {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p188:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the Author Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_Author/p188:author")]
pub struct Author {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// name
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// initials
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :initials
  #[sdk(attr(qname = ":initials"))]
  pub initials: Option<crate::simple_type::StringValue>,
  /// userId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p188:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentReply Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:reply.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentReply/p188:reply")]
pub struct CommentReply {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :status
  #[sdk(attr(qname = ":status"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :created
  #[sdk(attr(qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :tags
  #[sdk(attr(qname = ":tags"))]
  pub tags: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// likes
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :likes
  #[sdk(attr(qname = ":likes"))]
  pub likes: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  ///Defines the TextBodyType Class.
  #[sdk(child(qname = "a:CT_TextBody/p188:txBody"))]
  pub text_body_type: Option<std::boxed::Box<TextBodyType>>,
  ///Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst"))]
  pub comment_properties_extension_list: Option<CommentPropertiesExtensionList>,
}
/// Defines the Point2DType Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:pos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/p188:pos")]
pub struct Point2DType {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the CommentReplyList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:replyLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentReplyList/p188:replyLst")]
pub struct CommentReplyList {
  /// _
  #[sdk(child(qname = "p188:CT_CommentReply/p188:reply"))]
  pub p188_reply: Vec<CommentReply>,
}
/// Defines the Comment Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p188:cm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_Comment/p188:cm")]
pub struct Comment {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :status
  #[sdk(attr(qname = ":status"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :created
  #[sdk(attr(qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :tags
  #[sdk(attr(qname = ":tags"))]
  pub tags: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// likes
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :likes
  #[sdk(attr(qname = ":likes"))]
  pub likes: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// startDate
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// dueDate
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :dueDate
  #[sdk(attr(qname = ":dueDate"))]
  pub due_date: Option<crate::simple_type::DateTimeValue>,
  /// assignedTo
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :assignedTo
  #[sdk(attr(qname = ":assignedTo"))]
  pub assigned_to: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// complete
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :complete
  #[sdk(attr(qname = ":complete"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub complete: Option<crate::simple_type::Int32Value>,
  /// priority
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :priority
  #[sdk(attr(qname = ":priority"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// title
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
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
  /// _
  #[sdk(child(qname = "a:CT_Point2D/p188:pos"))]
  pub p188_pos: Option<Point2DType>,
  /// _
  #[sdk(child(qname = "p188:CT_CommentReplyList/p188:replyLst"))]
  pub p188_reply_lst: Option<CommentReplyList>,
  ///Defines the TextBodyType Class.
  #[sdk(child(qname = "a:CT_TextBody/p188:txBody"))]
  pub text_body_type: Option<std::boxed::Box<TextBodyType>>,
  ///Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(qname = "p188:CT_CommentPropertiesExtensionList/p188:extLst"))]
  pub comment_properties_extension_list: Option<CommentPropertiesExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentChoice {
  #[sdk(child(qname = "pc:CT_SlideMonikerList/pc:sldMkLst"))]
    PcSldMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
        >,
    ),
    #[sdk(child(qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst"))]
    PcSldLayoutMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideLayoutMonikerList,
        >,
    ),
    #[sdk(child(qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst"))]
    PcSldMasterMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::MainMasterMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_DrawingElementMonikerList/oac:deMkLst"))]
    OacDeMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DeMkLstDrawingElementMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_TextBodyMonikerList/oac:txBodyMkLst"))]
    OacTxBodyMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextBodyMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_TextCharRangeMonikerList/oac:txMkLst"))]
    OacTxMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextCharRangeMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_TableCellMonikerList/oac:tcMkLst"))]
    OacTcMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableCellMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_TableRowMonikerList/oac:trMkLst"))]
    OacTrMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableRowMonikerList,
        >,
    ),
    #[sdk(child(qname = "oac:CT_TableColumnMonikerList/oac:gridColMkLst"))]
    OacGridColMkLst(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableColumnMonikerList,
        >,
    ),
    #[sdk(child(qname = "p188:CT_CommentUnknownAnchor/p188:unknownAnchor"))]
    P188UnknownAnchor(std::boxed::Box<CommentUnknownAnchor>),
}
