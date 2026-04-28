//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the EmbeddedAnimation Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is a3danim:embedAnim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a3danim:CT_EmbeddedAnimation/a3danim:embedAnim")]
pub struct EmbeddedAnimation {
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
  /// animId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :animId
  #[sdk(attr(qname = ":animId"))]
  pub anim_id: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "aanim:CT_AnimationProperties/a3danim:animPr"))]
  pub animation_properties: std::boxed::Box<AnimationProperties>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a3danim:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PosterFrame Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is a3danim:posterFrame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a3danim:CT_PosterFrame/a3danim:posterFrame")]
pub struct PosterFrame {
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
  /// animId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :animId
  #[sdk(attr(qname = ":animId"))]
  pub anim_id: crate::simple_type::UInt32Value,
  /// frame
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :frame
  #[sdk(attr(qname = ":frame"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub frame: Option<crate::simple_type::Int32Value>,
}
/// Defines the AnimationProperties Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is a3danim:animPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aanim:CT_AnimationProperties/a3danim:animPr")]
pub struct AnimationProperties {
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
  /// name
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// length
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: crate::simple_type::StringValue,
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  pub count: Option<crate::simple_type::StringValue>,
  /// auto
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// offset
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// st
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub st: Option<crate::simple_type::StringValue>,
  /// end
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/aanim:extLst"))]
  pub office_art_extension_list: Option<
    crate::schemas::schemas_microsoft_com_office_drawing_2018_animation::OfficeArtExtensionList,
  >,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is a3danim:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/a3danim:extLst")]
pub struct OfficeArtExtensionList {
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
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
