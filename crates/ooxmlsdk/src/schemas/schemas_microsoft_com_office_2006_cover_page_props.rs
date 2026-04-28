//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CoverPageProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:CoverPageProperties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cppr:CT_CoverPageProperties/cppr:CoverPageProperties")]
pub struct CoverPageProperties {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "cppr:ST_PublishDate/cppr:PublishDate"))]
  pub publish_date: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cppr:Abstract"))]
  pub document_abstract: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyAddress"))]
  pub company_address: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyPhone"))]
  pub company_phone_number: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyFax"))]
  pub company_fax_number: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyEmail"))]
  pub company_email_address: crate::simple_type::StringValue,
}
/// Defines the PublishDate Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:PublishDate.
pub type PublishDate = crate::simple_type::StringValue;
/// Defines the DocumentAbstract Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:Abstract.
pub type DocumentAbstract = crate::simple_type::StringValue;
/// Defines the CompanyAddress Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyAddress.
pub type CompanyAddress = crate::simple_type::StringValue;
/// Defines the CompanyPhoneNumber Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyPhone.
pub type CompanyPhoneNumber = crate::simple_type::StringValue;
/// Defines the CompanyFaxNumber Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyFax.
pub type CompanyFaxNumber = crate::simple_type::StringValue;
/// Defines the CompanyEmailAddress Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cppr:CompanyEmail.
pub type CompanyEmailAddress = crate::simple_type::StringValue;
