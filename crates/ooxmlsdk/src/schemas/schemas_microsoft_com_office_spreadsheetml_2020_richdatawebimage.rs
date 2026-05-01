//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebImagesSupportingRichData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrdwi:CT_WebImagesSupportingRichData/xlrdwi:webImagesSrd"
)]
pub struct WebImagesSupportingRichData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the WebImageSupportingRichData Class.
  #[sdk(child(
    office2021,
    qname = "xlrdwi:CT_WebImageSupportingRichData/xlrdwi:webImageSrd"
  ))]
  pub xlrdwi_web_image_srd: Vec<WebImageSupportingRichData>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xlrdwi:extLst"))]
  pub xlrdwi_ext_lst: Option<ExtensionList>,
}
/// Defines the WebImageSupportingRichData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrdwi:CT_WebImageSupportingRichData/xlrdwi:webImageSrd"
)]
pub struct WebImageSupportingRichData {
  /// Defines the AddressWebImageSupportingRichDataRelationship Class.
  #[sdk(child(
    office2021,
    qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:address"
  ))]
  pub address_web_image_supporting_rich_data_relationship:
    std::boxed::Box<AddressWebImageSupportingRichDataRelationship>,
  /// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
  #[sdk(child(
    office2021,
    qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:moreImagesAddress"
  ))]
  pub more_images_address_web_image_supporting_rich_data_relationship:
    Option<MoreImagesAddressWebImageSupportingRichDataRelationship>,
  /// Defines the BlipWebImageSupportingRichDataRelationship Class.
  #[sdk(child(
    office2021,
    qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:blip"
  ))]
  pub blip_web_image_supporting_rich_data_relationship:
    Option<BlipWebImageSupportingRichDataRelationship>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x:CT_ExtensionList/xlrdwi:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the AddressWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:address"
)]
pub struct AddressWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(office2021, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:moreImagesAddress"
)]
pub struct MoreImagesAddressWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(office2021, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the BlipWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:blip"
)]
pub struct BlipWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(office2021, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
