//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the FeaturePropertyBags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "xfpb:FeaturePropertyBags")]
pub struct FeaturePropertyBags {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the BagExtensions Class.
  #[sdk(child(qname = "xfpb:bagExt"))]
  pub bag_extensions: Vec<BagExtensions>,
  /// Defines the FeaturePropertyBag Class.
  #[sdk(child(qname = "xfpb:bag"))]
  pub feature_property_bag: Vec<FeaturePropertyBag>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xfpb:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FpbsFeaturePropertyBags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:fpbs")]
pub struct FpbsFeaturePropertyBags {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the BagExtensions Class.
  #[sdk(child(qname = "xfpb:bagExt"))]
  pub bag_extensions: Vec<BagExtensions>,
  /// Defines the FeaturePropertyBag Class.
  #[sdk(child(qname = "xfpb:bag"))]
  pub feature_property_bag: Vec<FeaturePropertyBag>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xfpb:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the XfComplement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:xfComplement")]
pub struct XfComplement {
  /// i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the DXFComplement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:DXFComplement")]
pub struct DxfComplement {
  /// i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the RevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:revdxf")]
pub struct RevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:headerRowRevDxf")]
pub struct HeaderRowRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the DataRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:dataRevDxf")]
pub struct DataRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:totalsRowRevDxf")]
pub struct TotalsRowRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the HeaderRowBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:headerRowBorderRevDxf")]
pub struct HeaderRowBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TableBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:tableBorderRevDxf")]
pub struct TableBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the TotalsRowBorderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:totalsRowBorderRevDxf")]
pub struct TotalsRowBorderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnHeaderRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:columnHeaderRevDxf")]
pub struct ColumnHeaderRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnBodyRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:columnBodyRevDxf")]
pub struct ColumnBodyRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the ColumnTotalsRevDxfTableRevDxf Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:columnTotalsRevDxf")]
pub struct ColumnTotalsRevDxfTableRevDxf {
  /// Defines the FpbsFeaturePropertyBags Class.
  #[sdk(child(qname = "xfpb:fpbs"))]
  pub fpbs_feature_property_bags: std::boxed::Box<FpbsFeaturePropertyBags>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xfpb:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
}
/// Defines the BagExtensions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:bagExt")]
pub struct BagExtensions {
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xfpb:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FeaturePropertyBag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:bag")]
pub struct FeaturePropertyBag {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// extRef
  #[sdk(attr(qname = ":extRef"))]
  pub ext_ref: Option<crate::simple_type::StringValue>,
  /// bagExtId
  #[sdk(attr(qname = ":bagExtId"))]
  pub bag_ext_id: Option<crate::simple_type::UInt32Value>,
  /// att
  #[sdk(attr(qname = ":att"))]
  pub att: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = ArrayFeatureProperty, qname = "xfpb:a"),
            child(variant = BagFeatureProperty, qname = "xfpb:bagId"),
            child(variant = IntFeatureProperty, qname = "xfpb:i"),
            child(variant = StringFeatureProperty, qname = "xfpb:s"),
            child(variant = BoolFeatureProperty, qname = "xfpb:b"),
            child(variant = DecimalFeatureProperty, qname = "xfpb:d"),
            child(variant = RelFeatureProperty, qname = "xfpb:rel")
        )
    )]
  pub feature_property_bag_choice: Vec<FeaturePropertyBagChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the ArrayFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:a")]
pub struct ArrayFeatureProperty {
  /// Name of the key for the key value pair.
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(
        choice(
            text_child(
                variant = XsdunsignedInt,
                simple_type = "UInt32Value",
                qname = "xfpb:bagId"
            ),
            text_child(
                variant = Xsdinteger,
                simple_type = "IntegerValue",
                qname = "xfpb:i"
            ),
            text_child(variant = SXsdstring, qname = "xfpb:s"),
            text_child(
                variant = Xsdboolean,
                simple_type = "BooleanValue",
                qname = "xfpb:b"
            ),
            text_child(
                variant = Xsddouble,
                simple_type = "DoubleValue",
                qname = "xfpb:d"
            ),
            text_child(variant = RelXsdstring, qname = "xfpb:rel")
        )
    )]
  pub array_feature_property_choice: Vec<ArrayFeaturePropertyChoice>,
}
/// Defines the BagFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:bagId")]
pub struct BagFeatureProperty {
  /// k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IntFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:i")]
pub struct IntFeatureProperty {
  /// k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Int32Value>,
}
/// Defines the StringFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:s")]
pub struct StringFeatureProperty {
  /// k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the BoolFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:b")]
pub struct BoolFeatureProperty {
  /// k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DecimalFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:d")]
pub struct DecimalFeatureProperty {
  /// k
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the RelFeatureProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:rel")]
pub struct RelFeatureProperty {
  /// Name of the key for the key value pair.
  #[sdk(attr(qname = ":k"))]
  pub k: crate::simple_type::StringValue,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the DifferentialFormatType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xfpb:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:font"))]
  pub font: Option<crate::schemas::x::Font>,
  /// Number Format
  #[sdk(child(qname = "x:numFmt"))]
  pub numbering_format: Option<crate::schemas::x::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::x::Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:alignment"))]
  pub alignment: Option<crate::schemas::x::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:border"))]
  pub border: Option<std::boxed::Box<crate::schemas::x::Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:protection"))]
  pub protection: Option<crate::schemas::x::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:extLst"))]
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
#[derive(Clone, Debug, PartialEq)]
pub enum FeaturePropertyBagChoice {
  /// Defines the ArrayFeatureProperty Class.
  ArrayFeatureProperty(std::boxed::Box<ArrayFeatureProperty>),
  /// Defines the BagFeatureProperty Class.
  BagFeatureProperty(std::boxed::Box<BagFeatureProperty>),
  /// Defines the IntFeatureProperty Class.
  IntFeatureProperty(std::boxed::Box<IntFeatureProperty>),
  /// Defines the StringFeatureProperty Class.
  StringFeatureProperty(std::boxed::Box<StringFeatureProperty>),
  /// Defines the BoolFeatureProperty Class.
  BoolFeatureProperty(std::boxed::Box<BoolFeatureProperty>),
  /// Defines the DecimalFeatureProperty Class.
  DecimalFeatureProperty(std::boxed::Box<DecimalFeatureProperty>),
  /// Defines the RelFeatureProperty Class.
  RelFeatureProperty(std::boxed::Box<RelFeatureProperty>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ArrayFeaturePropertyChoice {
  /// Defines the XsdunsignedInt Class.
  XsdunsignedInt(XsdunsignedInt),
  /// Defines the Xsdinteger Class.
  Xsdinteger(Xsdinteger),
  /// Defines the SXsdstring Class.
  SXsdstring(SXsdstring),
  /// Defines the Xsdboolean Class.
  Xsdboolean(Xsdboolean),
  /// Defines the Xsddouble Class.
  Xsddouble(Xsddouble),
  /// Defines the RelXsdstring Class.
  RelXsdstring(RelXsdstring),
}
