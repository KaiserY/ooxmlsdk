//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebImagesSupportingRichData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "xlrdwi:webImagesSrd")]
pub struct WebImagesSupportingRichData {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the WebImageSupportingRichData Class.
  #[sdk(child(qname = "xlrdwi:webImageSrd"))]
  pub web_image_supporting_rich_data: Vec<WebImageSupportingRichData>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xlrdwi:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the WebImageSupportingRichData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:webImageSrd")]
pub struct WebImageSupportingRichData {
  /// Defines the AddressWebImageSupportingRichDataRelationship Class.
  #[sdk(child(qname = "xlrdwi:address"))]
  pub address_web_image_supporting_rich_data_relationship:
    std::boxed::Box<AddressWebImageSupportingRichDataRelationship>,
  /// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
  #[sdk(child(qname = "xlrdwi:moreImagesAddress"))]
  pub more_images_address_web_image_supporting_rich_data_relationship:
    Option<MoreImagesAddressWebImageSupportingRichDataRelationship>,
  /// Defines the BlipWebImageSupportingRichDataRelationship Class.
  #[sdk(child(qname = "xlrdwi:blip"))]
  pub blip_web_image_supporting_rich_data_relationship:
    Option<BlipWebImageSupportingRichDataRelationship>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the AddressWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:address")]
pub struct AddressWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:moreImagesAddress")]
pub struct MoreImagesAddressWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the BlipWebImageSupportingRichDataRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:blip")]
pub struct BlipWebImageSupportingRichDataRelationship {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
