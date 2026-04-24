//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebImagesSupportingRichData Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:webImagesSrd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImagesSupportingRichData/xlrdwi:webImagesSrd")]
pub struct WebImagesSupportingRichData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xlrdwi:CT_WebImageSupportingRichData/xlrdwi:webImageSrd"))]
  pub xlrdwi_web_image_srd: Vec<WebImageSupportingRichData>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrdwi:extLst"))]
  pub xlrdwi_ext_lst: Option<ExtensionList>,
}
/// Defines the WebImageSupportingRichData Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:webImageSrd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImageSupportingRichData/xlrdwi:webImageSrd")]
pub struct WebImageSupportingRichData {
  /// _
  #[sdk(child(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:address"))]
  pub address_web_image_supporting_rich_data_relationship:
    std::boxed::Box<AddressWebImageSupportingRichDataRelationship>,
  /// _
  #[sdk(child(
    qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:moreImagesAddress"
  ))]
  pub more_images_address_web_image_supporting_rich_data_relationship:
    Option<MoreImagesAddressWebImageSupportingRichDataRelationship>,
  /// _
  #[sdk(child(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:blip"))]
  pub blip_web_image_supporting_rich_data_relationship:
    Option<BlipWebImageSupportingRichDataRelationship>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xlrdwi:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the AddressWebImageSupportingRichDataRelationship Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:address.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:address")]
pub struct AddressWebImageSupportingRichDataRelationship {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the MoreImagesAddressWebImageSupportingRichDataRelationship Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:moreImagesAddress.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:moreImagesAddress")]
pub struct MoreImagesAddressWebImageSupportingRichDataRelationship {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the BlipWebImageSupportingRichDataRelationship Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrdwi:blip.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/xlrdwi:blip")]
pub struct BlipWebImageSupportingRichDataRelationship {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the OpenXmlWebImageSupportingRichDataRelationshipElement Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrdwi:CT_WebImageSupportingRichDataRelationship/")]
pub struct OpenXmlWebImageSupportingRichDataRelationshipElement {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
