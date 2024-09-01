use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;

pub fn simple_type_mapping(name: &str) -> &str {
  match name {
    "a:ST_Guid" => "crate::schemas::simple_type::StringValue",
    "xsd:string" => "crate::schemas::simple_type::StringValue",
    "c:ST_Xstring" => "crate::schemas::simple_type::StringValue",
    "cdr:ST_MarkerCoordinate" => "crate::schemas::simple_type::DoubleValue",
    "wp:ST_PositionOffset" => "crate::schemas::simple_type::Int32Value",
    "xdr:ST_ColID" => "crate::schemas::simple_type::Int32Value",
    "a:ST_Coordinate" => "crate::schemas::simple_type::Int64Value",
    "a:ST_Percentage" => "crate::schemas::simple_type::Int32Value",
    "a:ST_PositivePercentage" => "crate::schemas::simple_type::Int32Value",
    "ask:ST_LineSketchSeed" => "crate::schemas::simple_type::UInt32Value",
    "b:ST_String255" => "crate::schemas::simple_type::StringValue",
    "cppr:ST_PublishDate" => "crate::schemas::simple_type::StringValue",
    "cx:CT_Formula" => "crate::schemas::simple_type::StringValue",
    "cx:CT_NumericValue" => "crate::schemas::simple_type::DoubleValue",
    "cx:CT_StringValue" => "crate::schemas::simple_type::StringValue",
    "cx:ST_AxisId" => "crate::schemas::simple_type::UInt32Value",
    "emma:CT_Literal" => "crate::schemas::simple_type::StringValue",
    "inkml:CT_Annotation" => "crate::schemas::simple_type::StringValue",
    "inkml:CT_Matrix" => "crate::schemas::simple_type::StringValue",
    "inkml:CT_Table" => "crate::schemas::simple_type::StringValue",
    "inkml:CT_Trace" => "crate::schemas::simple_type::StringValue",
    "lp:CT_LongProp" => "crate::schemas::simple_type::StringValue",
    "m:CT_Text" => "crate::schemas::simple_type::StringValue",
    "msink:CT_Property" => "crate::schemas::simple_type::HexBinaryValue",
    "o:ST_TrueFalseBlank" => "crate::schemas::simple_type::TrueFalseBlankValue",
    "oac:CT_ImgData" => "crate::schemas::simple_type::Base64BinaryValue",
    "vt:CT_Cf" => "crate::schemas::simple_type::Base64BinaryValue",
    "vt:CT_Vstream" => "crate::schemas::simple_type::Base64BinaryValue",
    "vt:ST_Clsid" => "crate::schemas::simple_type::StringValue",
    "vt:ST_Cy" => "crate::schemas::simple_type::StringValue",
    "vt:ST_Error" => "crate::schemas::simple_type::StringValue",
    "w:CT_Base64BinaryText" => "crate::schemas::simple_type::Base64BinaryValue",
    "w:CT_Text" => "crate::schemas::simple_type::StringValue",
    "x:CT_CellFormula" => "crate::schemas::simple_type::StringValue",
    "x:CT_DefinedName" => "crate::schemas::simple_type::StringValue",
    "x:CT_TableFormula" => "crate::schemas::simple_type::StringValue",
    "x:CT_Xstring" => "crate::schemas::simple_type::StringValue",
    "x:ST_Formula" => "crate::schemas::simple_type::StringValue",
    "x:ST_Guid" => "crate::schemas::simple_type::StringValue",
    "x:ST_Xstring" => "crate::schemas::simple_type::StringValue",
    "x14:CT_DefinedNameArgumentDescription" => "crate::schemas::simple_type::StringValue",
    "x14:CT_PivotEditValue" => "crate::schemas::simple_type::StringValue",
    "xdr:ST_RowID" => "crate::schemas::simple_type::Int32Value",
    "xlrd:CT_RichValueFallback" => "crate::schemas::simple_type::StringValue",
    "xlrd:CT_Value" => "crate::schemas::simple_type::StringValue",
    "xlrd2:CT_ArrayValue" => "crate::schemas::simple_type::StringValue",
    "xlrd2:CT_RichStylePropertyValue" => "crate::schemas::simple_type::StringValue",
    "xlrd2:CT_SupportingPropertyBagArrayValue" => "crate::schemas::simple_type::StringValue",
    "xlrd2:CT_SupportingPropertyBagValue" => "crate::schemas::simple_type::StringValue",
    "xne:ST_Sqref" => "Vec<crate::schemas::simple_type::StringValue>",
    "xsd:anyURI" => "crate::schemas::simple_type::StringValue",
    "xsd:base64Binary" => "crate::schemas::simple_type::Base64BinaryValue",
    "xsd:boolean" => "crate::schemas::simple_type::BooleanValue",
    "xsd:byte" => "crate::schemas::simple_type::SByteValue",
    "xsd:dateTime" => "crate::schemas::simple_type::DateTimeValue",
    "xsd:decimal" => "crate::schemas::simple_type::DecimalValue",
    "xsd:double" => "crate::schemas::simple_type::DoubleValue",
    "xsd:float" => "crate::schemas::simple_type::SingleValue",
    "xsd:int" => "crate::schemas::simple_type::Int32Value",
    "xsd:integer" => "crate::schemas::simple_type::IntegerValue",
    "xsd:long" => "crate::schemas::simple_type::Int64Value",
    "xsd:nonNegativeInteger" => "crate::schemas::simple_type::IntegerValue",
    "xsd:short" => "crate::schemas::simple_type::Int16Value",
    "xsd:unsignedByte" => "crate::schemas::simple_type::ByteValue",
    "xsd:unsignedInt" => "crate::schemas::simple_type::UInt32Value",
    "xsd:unsignedLong" => "crate::schemas::simple_type::UInt64Value",
    "xsd:unsignedShort" => "crate::schemas::simple_type::UInt16Value",
    "xvml:ST_Macro" => "crate::schemas::simple_type::StringValue",
    "a:ST_Angle" => "crate::schemas::simple_type::Int32Value",
    "a:ST_DrawingElementId" => "crate::schemas::simple_type::UInt32Value",
    "a:ST_PositiveFixedPercentage" => "crate::schemas::simple_type::Int32Value",
    "msink:ST_Point" => "crate::schemas::simple_type::StringValue",
    "w:ST_DecimalNumber" => "crate::schemas::simple_type::Int32Value",
    "w:ST_HexColorRGB" => "crate::schemas::simple_type::HexBinaryValue",
    "w:ST_HpsMeasure_O12" => "crate::schemas::simple_type::UInt32Value",
    "w:ST_NonNegativeDecimalNumber" => "crate::schemas::simple_type::Int32Value",
    "w:ST_SignedDecimalNumberMax-1" => "crate::schemas::simple_type::Int32Value",
    "w:ST_SignedDecimalNumberMax-2" => "crate::schemas::simple_type::Int32Value",
    "w:ST_SignedHpsMeasure_O12" => "crate::schemas::simple_type::Int32Value",
    "w:ST_SignedTwipsMeasure_O12" => "crate::schemas::simple_type::Int32Value",
    "w:ST_StylePaneSortMethods_O12" => "crate::schemas::simple_type::HexBinaryValue",
    "w:ST_TwipsMeasure_O12" => "crate::schemas::simple_type::UInt32Value",
    "w:ST_UnsignedDecimalNumber" => "crate::schemas::simple_type::UInt32Value",
    "w:ST_UnsignedDecimalNumberMin1" => "crate::schemas::simple_type::Int32Value",
    "xne:ST_Ref" => "crate::schemas::simple_type::StringValue",
    "xfpb:CT_BagFeatureProperty" => "crate::schemas::simple_type::UInt32Value",
    "xfpb:CT_IntFeatureProperty" => "crate::schemas::simple_type::Int32Value",
    "xfpb:CT_StringFeatureProperty" => "crate::schemas::simple_type::StringValue",
    "xfpb:CT_BoolFeatureProperty" => "crate::schemas::simple_type::BooleanValue",
    "xfpb:CT_DecimalFeatureProperty" => "crate::schemas::simple_type::DoubleValue",
    "xfpb:CT_RelFeatureProperty" => "crate::schemas::simple_type::StringValue",
    _ => "crate::schemas::simple_type::StringValue", // FIXME: e.g. MoveWithCells
  }
}

pub fn gen_simple_type() -> Result<TokenStream, Box<dyn Error>> {
  Ok(quote! {
    pub type Base64BinaryValue = String;
    pub type BooleanValue = bool;
    pub type ByteValue = u8;
    pub type DateTimeValue = String;
    pub type DecimalValue = String;
    pub type DoubleValue = f64;
    pub type HexBinaryValue = String;
    pub type Int16Value = i16;
    pub type Int32Value = i32;
    pub type Int64Value = i64;
    pub type IntegerValue = String;
    pub type OnOffValue = bool;
    pub type SByteValue = String;
    pub type SingleValue = f32;
    pub type StringValue = String;
    pub type TrueFalseBlankValue = bool;
    pub type TrueFalseValue = bool;
    pub type UInt16Value = u16;
    pub type UInt32Value = u32;
    pub type UInt64Value = u64;
  })
}
