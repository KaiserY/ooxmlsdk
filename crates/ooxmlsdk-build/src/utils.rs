use heck::ToUpperCamelCase;
use syn::{parse_str, Type};

pub fn parse_simple_type(simple_type: &str) -> Result<Type, syn::Error> {
  match simple_type {
    "a:ST_Guid"
    | "xsd:string"
    | "c:ST_Xstring"
    | "b:ST_String255"
    | "cppr:ST_PublishDate"
    | "cx:CT_Formula"
    | "cx:CT_StringValue"
    | "emma:CT_Literal"
    | "inkml:CT_Annotation"
    | "inkml:CT_Matrix"
    | "inkml:CT_Table"
    | "inkml:CT_Trace"
    | "lp:CT_LongProp"
    | "m:CT_Text"
    | "x:CT_CellFormula"
    | "x:CT_DefinedName"
    | "x:CT_TableFormula"
    | "x:CT_Xstring"
    | "x:ST_Formula"
    | "x:ST_Guid"
    | "x:ST_Xstring"
    | "x14:CT_DefinedNameArgumentDescription"
    | "x14:CT_PivotEditValue"
    | "xlrd:CT_RichValueFallback"
    | "xlrd:CT_Value"
    | "xlrd2:CT_ArrayValue"
    | "xlrd2:CT_RichStylePropertyValue"
    | "xlrd2:CT_SupportingPropertyBagArrayValue"
    | "xlrd2:CT_SupportingPropertyBagValue"
    | "xsd:anyURI"
    | "xvml:ST_Macro"
    | "msink:ST_Point"
    | "xne:ST_Ref"
    | "xfpb:CT_StringFeatureProperty"
    | "xfpb:CT_RelFeatureProperty" => parse_str("String"),
    "xne:ST_Sqref" => parse_str("Vec<String>"),
    "cdr:ST_MarkerCoordinate"
    | "cx:CT_NumericValue"
    | "xsd:double"
    | "xfpb:CT_DecimalFeatureProperty" => parse_str("f64"),
    "xsd:float" => parse_str("f32"),
    "a:ST_Coordinate" | "xsd:long" => parse_str("i64"),
    "msink:CT_Property" | "w:ST_HexColorRGB" | "w:ST_StylePaneSortMethods_O12" => {
      parse_str("Vec<u8>")
    }
    "oac:CT_ImgData"
    | "vt:CT_Cf"
    | "vt:CT_Vstream"
    | "w:CT_Base64BinaryText"
    | "xsd:base64Binary" => parse_str("Vec<u8>"),
    "xsd:byte" => parse_str("Vec<u8>"),
    "o:ST_TrueFalseBlank" => parse_str("bool"),
    "xsd:boolean" | "xfpb:CT_BoolFeatureProperty" => parse_str("bool"),
    "ask:ST_LineSketchSeed"
    | "cx:ST_AxisId"
    | "xsd:unsignedInt"
    | "a:ST_DrawingElementId"
    | "w:ST_HpsMeasure_O12"
    | "w:ST_TwipsMeasure_O12"
    | "w:ST_UnsignedDecimalNumber"
    | "xfpb:CT_BagFeatureProperty" => parse_str("u32"),
    "xsd:dateTime" => parse_str("String"),
    "xsd:decimal" => parse_str("String"),
    "xsd:integer" | "xsd:nonNegativeInteger" => parse_str("String"),
    "xsd:short" => parse_str("i16"),
    "xsd:unsignedByte" => parse_str("u8"),
    "xsd:unsignedLong" => parse_str("u64"),
    "xsd:unsignedShort" => parse_str("u16"),
    "wp:ST_PositionOffset"
    | "xdr:ST_ColID"
    | "a:ST_Percentage"
    | "a:ST_PositivePercentage"
    | "xdr:ST_RowID"
    | "xsd:int"
    | "a:ST_Angle"
    | "a:ST_PositiveFixedPercentage"
    | "w:ST_DecimalNumber"
    | "w:ST_NonNegativeDecimalNumber"
    | "w:ST_SignedDecimalNumberMax-1"
    | "w:ST_SignedDecimalNumberMax-2"
    | "w:ST_SignedHpsMeasure_O12"
    | "w:ST_SignedTwipsMeasure_O12"
    | "w:ST_UnsignedDecimalNumberMin1"
    | "xfpb:CT_IntFeatureProperty" => parse_str("i32"),
    _ => parse_str(&simple_type.to_upper_camel_case()),
  }
}

pub fn escape_snake_case(name: String) -> String {
  match name.as_str() {
    "if" | "else" | "ref" | "type" | "macro" | "loop" | "mod" | "override" | "for" | "in"
    | "final" => {
      format!("r#{}", name)
    }
    _ => name,
  }
}

pub fn escape_upper_camel_case(name: String) -> String {
  match name.as_str() {
    "self" | "Self" => {
      format!("_{}", name)
    }
    _ => name,
  }
}
