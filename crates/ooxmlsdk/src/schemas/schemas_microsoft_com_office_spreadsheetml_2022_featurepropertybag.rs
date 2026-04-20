//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the FeaturePropertyBags Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:FeaturePropertyBags.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_FeaturePropertyBags/xfpb:FeaturePropertyBags")]
pub struct FeaturePropertyBags {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// count
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xfpb:CT_BagExtensions/xfpb:bagExt"))]
  pub xfpb_bag_ext: Vec<BagExtensions>,
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag"))]
  pub xfpb_bag: Vec<FeaturePropertyBag>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub xfpb_ext_lst: Option<ExtensionList>,
}
/// Defines the FpbsFeaturePropertyBags Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:fpbs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs")]
pub struct FpbsFeaturePropertyBags {
  /// count
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xfpb:CT_BagExtensions/xfpb:bagExt"))]
  pub xfpb_bag_ext: Vec<BagExtensions>,
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag"))]
  pub xfpb_bag: Vec<FeaturePropertyBag>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub xfpb_ext_lst: Option<ExtensionList>,
}
/// Defines the OpenXmlFeaturePropertyBagsElement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_FeaturePropertyBags/")]
pub struct OpenXmlFeaturePropertyBagsElement {
  /// count
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Vec<OpenXmlFeaturePropertyBagsElementChoice>,
}
/// Defines the XfComplement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:xfComplement.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_XfComplement/xfpb:xfComplement")]
pub struct XfComplement {
  /// i
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the DXFComplement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:DXFComplement.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_DXFComplement/xfpb:DXFComplement")]
pub struct DxfComplement {
  /// i
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the RevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:revdxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_RevDxf/xfpb:revdxf")]
pub struct RevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:headerRowRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:headerRowRevDxf")]
pub struct HeaderRowRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the DataRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:dataRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:dataRevDxf")]
pub struct DataRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:totalsRowRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:totalsRowRevDxf")]
pub struct TotalsRowRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowBorderRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:headerRowBorderRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:headerRowBorderRevDxf")]
pub struct HeaderRowBorderRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TableBorderRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:tableBorderRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:tableBorderRevDxf")]
pub struct TableBorderRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowBorderRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:totalsRowBorderRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:totalsRowBorderRevDxf")]
pub struct TotalsRowBorderRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnHeaderRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:columnHeaderRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:columnHeaderRevDxf")]
pub struct ColumnHeaderRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnBodyRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:columnBodyRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:columnBodyRevDxf")]
pub struct ColumnBodyRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnTotalsRevDxfTableRevDxf Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:columnTotalsRevDxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/xfpb:columnTotalsRevDxf")]
pub struct ColumnTotalsRevDxfTableRevDxf {
  /// _
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the OpenXmlTableRevDxfElement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_TableRevDxf/")]
pub struct OpenXmlTableRevDxfElement {
  #[sdk(choice)]
  pub xml_children: Vec<OpenXmlTableRevDxfElementChoice>,
}
/// Defines the BagExtensions Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:bagExt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_BagExtensions/xfpb:bagExt")]
pub struct BagExtensions {
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xfpb:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FeaturePropertyBag Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:bag.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag")]
pub struct FeaturePropertyBag {
  /// type
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// extRef
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :extRef
  #[sdk(attr(qname = ":extRef"))]
  pub ext_ref: Option<crate::simple_type::StringValue>,
  /// bagExtId
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :bagExtId
  #[sdk(attr(qname = ":bagExtId"))]
  pub bag_ext_id: Option<crate::simple_type::UInt32Value>,
  /// att
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :att
  #[sdk(attr(qname = ":att"))]
  pub att: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<FeaturePropertyBagChoice>,
}
/// Defines the ExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xfpb:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the ArrayFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:a.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_ArrayFeatureProperty/xfpb:a")]
pub struct ArrayFeatureProperty {
  /// Name of the key for the key value pair.
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Vec<ArrayFeaturePropertyChoice>,
}
/// Defines the BagFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:bagId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_BagFeatureProperty/xfpb:bagId")]
pub struct BagFeatureProperty {
  /// k
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IntFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:i.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_IntFeatureProperty/xfpb:i")]
pub struct IntFeatureProperty {
  /// k
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Int32Value>,
}
/// Defines the StringFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:s.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_StringFeatureProperty/xfpb:s")]
pub struct StringFeatureProperty {
  /// k
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the BoolFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:b.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_BoolFeatureProperty/xfpb:b")]
pub struct BoolFeatureProperty {
  /// k
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DecimalFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:d.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_DecimalFeatureProperty/xfpb:d")]
pub struct DecimalFeatureProperty {
  /// k
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the RelFeatureProperty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:rel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:CT_RelFeatureProperty/xfpb:rel")]
pub struct RelFeatureProperty {
  /// Name of the key for the key value pair.
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the DifferentialFormatType Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:dxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/xfpb:dxf")]
pub struct DifferentialFormatType {
  ///Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font>,
  >,
  ///Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat>,
  ///Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill>,
  >,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment>,
  ///Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border>,
  >,
  ///Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the XsdunsignedInt Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:bagId.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the Xsdinteger Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:i.
pub type Xsdinteger = crate::simple_type::IntegerValue;
/// Defines the SXsdstring Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:s.
pub type SXsdstring = crate::simple_type::StringValue;
/// Defines the RelXsdstring Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:rel.
pub type RelXsdstring = crate::simple_type::StringValue;
/// Defines the Xsdboolean Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:b.
pub type Xsdboolean = crate::simple_type::BooleanValue;
/// Defines the Xsddouble Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xfpb:d.
pub type Xsddouble = crate::simple_type::DoubleValue;
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OpenXmlFeaturePropertyBagsElementChoice {
  #[sdk(child(qname = "xfpb:CT_BagExtensions/xfpb:bagExt"))]
  XfpbBagExt(std::boxed::Box<BagExtensions>),
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBag/xfpb:bag"))]
  XfpbBag(std::boxed::Box<FeaturePropertyBag>),
  #[sdk(child(qname = "x:CT_ExtensionList/xfpb:extLst"))]
  XfpbExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OpenXmlTableRevDxfElementChoice {
  #[sdk(child(qname = "xfpb:CT_FeaturePropertyBags/xfpb:fpbs"))]
  XfpbFpbs(std::boxed::Box<FpbsFeaturePropertyBags>),
  #[sdk(child(qname = "x:CT_Dxf/xfpb:dxf"))]
  XfpbDxf(std::boxed::Box<DifferentialFormatType>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FeaturePropertyBagChoice {
  #[sdk(child(qname = "xfpb:CT_ArrayFeatureProperty/xfpb:a"))]
  XfpbA(std::boxed::Box<ArrayFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_BagFeatureProperty/xfpb:bagId"))]
  XfpbBagId(std::boxed::Box<BagFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_IntFeatureProperty/xfpb:i"))]
  XfpbI(std::boxed::Box<IntFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_StringFeatureProperty/xfpb:s"))]
  XfpbS(std::boxed::Box<StringFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_BoolFeatureProperty/xfpb:b"))]
  XfpbB(std::boxed::Box<BoolFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_DecimalFeatureProperty/xfpb:d"))]
  XfpbD(std::boxed::Box<DecimalFeatureProperty>),
  #[sdk(child(qname = "xfpb:CT_RelFeatureProperty/xfpb:rel"))]
  XfpbRel(std::boxed::Box<RelFeatureProperty>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ArrayFeaturePropertyChoice {
  #[sdk(text_child(qname = "xsd:unsignedInt/xfpb:bagId"))]
  XfpbBagId(crate::simple_type::UInt32Value),
  #[sdk(text_child(qname = "xsd:integer/xfpb:i"))]
  XfpbI(crate::simple_type::IntegerValue),
  #[sdk(text_child(qname = "xsd:string/xfpb:s"))]
  XfpbS(crate::simple_type::StringValue),
  #[sdk(text_child(qname = "xsd:boolean/xfpb:b"))]
  XfpbB(crate::simple_type::BooleanValue),
  #[sdk(text_child(qname = "xsd:double/xfpb:d"))]
  XfpbD(crate::simple_type::DoubleValue),
  #[sdk(text_child(qname = "xsd:string/xfpb:rel"))]
  XfpbRel(crate::simple_type::StringValue),
}
