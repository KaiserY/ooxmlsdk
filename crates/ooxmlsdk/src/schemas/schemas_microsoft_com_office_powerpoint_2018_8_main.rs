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
#[sdk(qname = "p188:txBody")]
pub struct TextBodyType {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<crate::schemas::a::Paragraph>,
}
/// Defines the CommentPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:extLst")]
pub struct CommentPropertiesExtensionList {
  /// Data for the Windows platform..
  #[sdk(child(qname = "p:ext"))]
  pub comment_properties_extension: Vec<crate::schemas::p::CommentPropertiesExtension>,
}
/// Defines the AuthorList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p188:authorLst")]
pub struct AuthorList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the Author Class.
  #[sdk(child(qname = "p188:author"))]
  pub author: Vec<Author>,
}
/// Defines the CommentList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p188:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the Comment Class.
  #[sdk(child(qname = "p188:cm"))]
  pub comment: Vec<Comment>,
}
/// Defines the CommentRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:commentRel")]
pub struct CommentRelationship {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the Author Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:author")]
pub struct Author {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// initials
  #[sdk(attr(qname = ":initials"))]
  pub initials: Option<crate::simple_type::StringValue>,
  /// userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p188:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentReply Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:reply")]
pub struct CommentReply {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  #[sdk(attr(qname = ":status"))]
  #[sdk(string_format(kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  #[sdk(attr(qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  #[sdk(attr(list, qname = ":tags"))]
  pub tags: Option<Vec<crate::simple_type::StringValue>>,
  /// likes
  #[sdk(attr(list, qname = ":likes"))]
  pub likes: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the TextBodyType Class.
  #[sdk(child(qname = "p188:txBody"))]
  pub text_body_type: Option<std::boxed::Box<TextBodyType>>,
  /// Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(qname = "p188:extLst"))]
  pub comment_properties_extension_list: Option<CommentPropertiesExtensionList>,
}
/// Defines the Point2DType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:pos")]
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
#[sdk(qname = "p188:replyLst")]
pub struct CommentReplyList {
  /// Defines the CommentReply Class.
  #[sdk(child(qname = "p188:reply"))]
  pub comment_reply: Vec<CommentReply>,
}
/// Defines the Comment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:cm")]
pub struct Comment {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
  /// status
  #[sdk(attr(qname = ":status"))]
  #[sdk(string_format(kind = "token"))]
  pub status: Option<CommentStatus>,
  /// created
  #[sdk(attr(qname = ":created"))]
  pub created: crate::simple_type::DateTimeValue,
  /// tags
  #[sdk(attr(list, qname = ":tags"))]
  pub tags: Option<Vec<crate::simple_type::StringValue>>,
  /// likes
  #[sdk(attr(list, qname = ":likes"))]
  pub likes: Option<Vec<crate::simple_type::StringValue>>,
  /// startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// dueDate
  #[sdk(attr(qname = ":dueDate"))]
  pub due_date: Option<crate::simple_type::DateTimeValue>,
  /// assignedTo
  #[sdk(attr(list, qname = ":assignedTo"))]
  pub assigned_to: Option<Vec<crate::simple_type::StringValue>>,
  /// complete
  #[sdk(attr(qname = ":complete"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub complete: Option<crate::simple_type::Int32Value>,
  /// priority
  #[sdk(attr(qname = ":priority"))]
  #[sdk(number_range(range = 0..= 10))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = SlideMonikerList, boxed, qname = "pc:sldMkLst"),
            any_child(variant = SlideLayoutMonikerList, qname = "pc:sldLayoutMkLst"),
            any_child(variant = MainMasterMonikerList, qname = "pc:sldMasterMkLst"),
            any_child(variant = DeMkLstDrawingElementMonikerList, qname = "oac:deMkLst"),
            any_child(variant = TextBodyMonikerList, qname = "oac:txBodyMkLst"),
            any_child(variant = TextCharRangeMonikerList, qname = "oac:txMkLst"),
            any_child(variant = TableCellMonikerList, qname = "oac:tcMkLst"),
            any_child(variant = TableRowMonikerList, qname = "oac:trMkLst"),
            any_child(variant = TableColumnMonikerList, qname = "oac:gridColMkLst"),
            empty_child(variant = CommentUnknownAnchor, qname = "p188:unknownAnchor")
        )
    )]
  pub comment_choice: Vec<CommentChoice>,
  /// Defines the Point2DType Class.
  #[sdk(child(qname = "p188:pos"))]
  pub point2_d_type: Option<Point2DType>,
  /// Defines the CommentReplyList Class.
  #[sdk(child(qname = "p188:replyLst"))]
  pub comment_reply_list: Option<CommentReplyList>,
  /// Defines the TextBodyType Class.
  #[sdk(child(qname = "p188:txBody"))]
  pub text_body_type: Option<std::boxed::Box<TextBodyType>>,
  /// Defines the CommentPropertiesExtensionList Class.
  #[sdk(child(qname = "p188:extLst"))]
  pub comment_properties_extension_list: Option<CommentPropertiesExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum CommentChoice {
  SlideMonikerList(std::boxed::Box<crate::schemas::pc::SlideMonikerList>),
  SlideLayoutMonikerList(crate::schemas::pc::SlideLayoutMonikerList),
  MainMasterMonikerList(crate::schemas::pc::MainMasterMonikerList),
  DeMkLstDrawingElementMonikerList(crate::schemas::oac::DeMkLstDrawingElementMonikerList),
  TextBodyMonikerList(crate::schemas::oac::TextBodyMonikerList),
  TextCharRangeMonikerList(crate::schemas::oac::TextCharRangeMonikerList),
  TableCellMonikerList(crate::schemas::oac::TableCellMonikerList),
  TableRowMonikerList(crate::schemas::oac::TableRowMonikerList),
  TableColumnMonikerList(crate::schemas::oac::TableColumnMonikerList),
  /// Defines the CommentUnknownAnchor Class.
  CommentUnknownAnchor,
}
