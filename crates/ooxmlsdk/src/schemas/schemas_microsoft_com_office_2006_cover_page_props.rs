//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CoverPageProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cppr:CT_CoverPageProperties/cppr:CoverPageProperties")]
pub struct CoverPageProperties {
  /// Defines the PublishDate Class.
  #[sdk(text_child(qname = "cppr:ST_PublishDate/cppr:PublishDate"))]
  pub publish_date: crate::simple_type::StringValue,
  /// Defines the DocumentAbstract Class.
  #[sdk(text_child(qname = "xsd:string/cppr:Abstract"))]
  pub document_abstract: crate::simple_type::StringValue,
  /// Defines the CompanyAddress Class.
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyAddress"))]
  pub company_address: crate::simple_type::StringValue,
  /// Defines the CompanyPhoneNumber Class.
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyPhone"))]
  pub company_phone_number: crate::simple_type::StringValue,
  /// Defines the CompanyFaxNumber Class.
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyFax"))]
  pub company_fax_number: crate::simple_type::StringValue,
  /// Defines the CompanyEmailAddress Class.
  #[sdk(text_child(qname = "xsd:string/cppr:CompanyEmail"))]
  pub company_email_address: crate::simple_type::StringValue,
}
/// Defines the PublishDate Class.
pub type PublishDate = crate::simple_type::StringValue;
/// Defines the DocumentAbstract Class.
pub type DocumentAbstract = crate::simple_type::StringValue;
/// Defines the CompanyAddress Class.
pub type CompanyAddress = crate::simple_type::StringValue;
/// Defines the CompanyPhoneNumber Class.
pub type CompanyPhoneNumber = crate::simple_type::StringValue;
/// Defines the CompanyFaxNumber Class.
pub type CompanyFaxNumber = crate::simple_type::StringValue;
/// Defines the CompanyEmailAddress Class.
pub type CompanyEmailAddress = crate::simple_type::StringValue;
