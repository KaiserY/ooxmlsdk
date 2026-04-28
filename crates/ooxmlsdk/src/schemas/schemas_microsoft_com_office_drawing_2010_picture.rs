//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ShapeStyle Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is pic14:style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/pic14:style")]
pub struct ShapeStyle {
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
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectReference,
  >,
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is pic14:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/pic14:extLst")]
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
