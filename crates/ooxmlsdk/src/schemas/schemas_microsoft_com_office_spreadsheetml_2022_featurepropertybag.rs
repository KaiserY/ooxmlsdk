//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the FeaturePropertyBags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xfpb:CT_FeaturePropertyBags/xfpb:FeaturePropertyBags"
)]
pub struct FeaturePropertyBags {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// count
  #[sdk(attr(microsoft365, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the BagExtensions Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_BagExtensions/xfpb:bagExt"))]
  pub xfpb_bag_ext: Vec<BagExtensions>,
  /// Defines the FeaturePropertyBag Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag"))]
  pub xfpb_bag: Vec<FeaturePropertyBag>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub xfpb_ext_lst: Option<ExtensionList>,
}
/// Defines the FpbsFeaturePropertyBags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs")]
pub struct FpbsFeaturePropertyBags {
  /// count
  #[sdk(attr(microsoft365, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the BagExtensions Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_BagExtensions/xfpb:bagExt"))]
  pub xfpb_bag_ext: Vec<BagExtensions>,
  /// Defines the FeaturePropertyBag Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag"))]
  pub xfpb_bag: Vec<FeaturePropertyBag>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub xfpb_ext_lst: Option<ExtensionList>,
}
/// Defines the XfComplement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_XfComplement/xfpb:xfComplement")]
pub struct XfComplement {
  /// i
  #[sdk(attr(microsoft365, qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the DXFComplement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_DXFComplement/xfpb:DXFComplement")]
pub struct DxfComplement {
  /// i
  #[sdk(attr(microsoft365, qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the RevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_RevDxf/xfpb:revdxf")]
pub struct RevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:headerRowRevDxf")]
pub struct HeaderRowRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the DataRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:dataRevDxf")]
pub struct DataRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:totalsRowRevDxf")]
pub struct TotalsRowRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:headerRowBorderRevDxf")]
pub struct HeaderRowBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TableBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:tableBorderRevDxf")]
pub struct TableBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:totalsRowBorderRevDxf")]
pub struct TotalsRowBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnHeaderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:columnHeaderRevDxf")]
pub struct ColumnHeaderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnBodyRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:columnBodyRevDxf")]
pub struct ColumnBodyRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnTotalsRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_TableRevDxf/xfpb:columnTotalsRevDxf")]
pub struct ColumnTotalsRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(microsoft365, qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the BagExtensions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_BagExtensions/xfpb:bagExt")]
pub struct BagExtensions {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FeaturePropertyBag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag")]
pub struct FeaturePropertyBag {
  /// type
  #[sdk(attr(microsoft365, qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// extRef
  #[sdk(attr(microsoft365, qname = ":extRef"))]
  pub ext_ref: Option<crate::simple_type::StringValue>,
  /// bagExtId
  #[sdk(attr(microsoft365, qname = ":bagExtId"))]
  pub bag_ext_id: Option<crate::simple_type::UInt32Value>,
  /// att
  #[sdk(attr(microsoft365, qname = ":att"))]
  pub att: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "xfpb:CT_ArrayFeatureProperty/xfpb:a",
    qname = "xfpb:CT_BagFeatureProperty/xfpb:bagId",
    qname = "xfpb:CT_IntFeatureProperty/xfpb:i",
    qname = "xfpb:CT_StringFeatureProperty/xfpb:s",
    qname = "xfpb:CT_BoolFeatureProperty/xfpb:b",
    qname = "xfpb:CT_DecimalFeatureProperty/xfpb:d",
    qname = "xfpb:CT_RelFeatureProperty/xfpb:rel"
  ))]
  pub feature_property_bag_choice: Vec<FeaturePropertyBagChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "x:CT_ExtensionList/xfpb:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
/// Defines the ArrayFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_ArrayFeatureProperty/xfpb:a")]
pub struct ArrayFeatureProperty {
  /// Name of the key for the key value pair.
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "xsd:unsignedInt/xfpb:bagId",
    qname = "xsd:integer/xfpb:i",
    qname = "xsd:string/xfpb:s",
    qname = "xsd:boolean/xfpb:b",
    qname = "xsd:double/xfpb:d",
    qname = "xsd:string/xfpb:rel"
  ))]
  pub array_feature_property_choice: Vec<ArrayFeaturePropertyChoice>,
}
/// Defines the BagFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_BagFeatureProperty/xfpb:bagId")]
pub struct BagFeatureProperty {
  /// k
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IntFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_IntFeatureProperty/xfpb:i")]
pub struct IntFeatureProperty {
  /// k
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Int32Value>,
}
/// Defines the StringFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_StringFeatureProperty/xfpb:s")]
pub struct StringFeatureProperty {
  /// k
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the BoolFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_BoolFeatureProperty/xfpb:b")]
pub struct BoolFeatureProperty {
  /// k
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DecimalFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_DecimalFeatureProperty/xfpb:d")]
pub struct DecimalFeatureProperty {
  /// k
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the RelFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xfpb:CT_RelFeatureProperty/xfpb:rel")]
pub struct RelFeatureProperty {
  /// Name of the key for the key value pair.
  #[sdk(attr(microsoft365, qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the DifferentialFormatType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "x:CT_Dxf/xfpb:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<crate::schemas::x::Font>>,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<crate::schemas::x::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::x::Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<crate::schemas::x::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<crate::schemas::x::Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<crate::schemas::x::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Defines the XsdunsignedInt Class.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the Xsdinteger Class.
pub type Xsdinteger = crate::simple_type::IntegerValue;
/// Defines the SXsdstring Class.
pub type SXsdstring = crate::simple_type::StringValue;
/// Defines the RelXsdstring Class.
pub type RelXsdstring = crate::simple_type::StringValue;
/// Defines the Xsdboolean Class.
pub type Xsdboolean = crate::simple_type::BooleanValue;
/// Defines the Xsddouble Class.
pub type Xsddouble = crate::simple_type::DoubleValue;
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FeaturePropertyBagChoice {
  /// Defines the ArrayFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_ArrayFeatureProperty/xfpb:a"))]
  XfpbA(std::boxed::Box<ArrayFeatureProperty>),
  /// Defines the BagFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_BagFeatureProperty/xfpb:bagId"))]
  XfpbBagId(std::boxed::Box<BagFeatureProperty>),
  /// Defines the IntFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_IntFeatureProperty/xfpb:i"))]
  XfpbI(std::boxed::Box<IntFeatureProperty>),
  /// Defines the StringFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_StringFeatureProperty/xfpb:s"))]
  XfpbS(std::boxed::Box<StringFeatureProperty>),
  /// Defines the BoolFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_BoolFeatureProperty/xfpb:b"))]
  XfpbB(std::boxed::Box<BoolFeatureProperty>),
  /// Defines the DecimalFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_DecimalFeatureProperty/xfpb:d"))]
  XfpbD(std::boxed::Box<DecimalFeatureProperty>),
  /// Defines the RelFeatureProperty Class.
  #[sdk(child(microsoft365, qname = "xfpb:CT_RelFeatureProperty/xfpb:rel"))]
  XfpbRel(std::boxed::Box<RelFeatureProperty>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ArrayFeaturePropertyChoice {
  /// Defines the XsdunsignedInt Class.
  #[sdk(text_child(microsoft365, qname = "xsd:unsignedInt/xfpb:bagId"))]
  XfpbBagId(crate::simple_type::UInt32Value),
  /// Defines the Xsdinteger Class.
  #[sdk(text_child(microsoft365, qname = "xsd:integer/xfpb:i"))]
  XfpbI(crate::simple_type::IntegerValue),
  /// Defines the SXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xfpb:s"))]
  XfpbS(crate::simple_type::StringValue),
  /// Defines the Xsdboolean Class.
  #[sdk(text_child(microsoft365, qname = "xsd:boolean/xfpb:b"))]
  XfpbB(crate::simple_type::BooleanValue),
  /// Defines the Xsddouble Class.
  #[sdk(text_child(microsoft365, qname = "xsd:double/xfpb:d"))]
  XfpbD(crate::simple_type::DoubleValue),
  /// Defines the RelXsdstring Class.
  #[sdk(text_child(microsoft365, qname = "xsd:string/xfpb:rel"))]
  XfpbRel(crate::simple_type::StringValue),
}
