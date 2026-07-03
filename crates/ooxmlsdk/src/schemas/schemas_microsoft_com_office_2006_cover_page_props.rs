//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CoverPageProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cppr:CoverPageProperties")]
pub struct CoverPageProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the PublishDate Class.
  #[sdk(text_child(qname = "cppr:PublishDate"))]
  pub publish_date: PublishDate,
  /// Defines the DocumentAbstract Class.
  #[sdk(text_child(qname = "cppr:Abstract"))]
  pub document_abstract: DocumentAbstract,
  /// Defines the CompanyAddress Class.
  #[sdk(text_child(qname = "cppr:CompanyAddress"))]
  pub company_address: CompanyAddress,
  /// Defines the CompanyPhoneNumber Class.
  #[sdk(text_child(qname = "cppr:CompanyPhone"))]
  pub company_phone_number: CompanyPhoneNumber,
  /// Defines the CompanyFaxNumber Class.
  #[sdk(text_child(qname = "cppr:CompanyFax"))]
  pub company_fax_number: CompanyFaxNumber,
  /// Defines the CompanyEmailAddress Class.
  #[sdk(text_child(qname = "cppr:CompanyEmail"))]
  pub company_email_address: CompanyEmailAddress,
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
