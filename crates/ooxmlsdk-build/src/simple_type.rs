pub fn simple_type_mapping(name: &str) -> &str {
  match name {
    "a:ST_Guid" | "xsd:string" | "c:ST_Xstring" => "StringValue",
    "cdr:ST_MarkerCoordinate" => "DoubleValue",
    "wp:ST_PositionOffset" | "xdr:ST_ColID" => "Int32Value",
    "ask:ST_LineSketchSeed" => "UInt32Value",
    "b:ST_String255" | "cppr:ST_PublishDate" => "StringValue",
    "cx:CT_Formula" => "StringValue",
    "cx:CT_NumericValue" => "DoubleValue",
    "cx:CT_StringValue" => "StringValue",
    "cx:ST_AxisId" => "UInt32Value",
    "emma:CT_Literal"
    | "inkml:CT_Annotation"
    | "inkml:CT_Matrix"
    | "inkml:CT_Table"
    | "inkml:CT_Trace"
    | "lp:CT_LongProp"
    | "m:CT_Text" => "StringValue",
    "msink:CT_Property" => "HexBinaryValue",
    "o:ST_TrueFalseBlank" => "TrueFalseBlankValue",
    "oac:CT_ImgData" | "vt:CT_Cf" | "vt:CT_Vstream" => "Base64BinaryValue",
    "vt:ST_Clsid" | "vt:ST_Cy" | "vt:ST_Error" => "StringValue",
    "w:CT_Base64BinaryText" => "Base64BinaryValue",
    "w:CT_Text"
    | "x:CT_CellFormula"
    | "x:CT_DefinedName"
    | "x:CT_TableFormula"
    | "x:CT_Xstring"
    | "x:ST_Formula"
    | "x:ST_Guid"
    | "x:ST_Xstring"
    | "x14:CT_DefinedNameArgumentDescription"
    | "x14:CT_PivotEditValue" => "StringValue",
    "xdr:ST_RowID" => "Int32Value",
    "xlrd:CT_RichValueFallback"
    | "xlrd:CT_Value"
    | "xlrd2:CT_ArrayValue"
    | "xlrd2:CT_RichStylePropertyValue"
    | "xlrd2:CT_SupportingPropertyBagArrayValue"
    | "xlrd2:CT_SupportingPropertyBagValue" => "StringValue",
    "xne:ST_Sqref" => "ListValue<StringValue>",
    "xsd:anyURI" => "StringValue",
    "xsd:base64Binary" => "Base64BinaryValue",
    "xsd:boolean" => "BooleanValue",
    "xsd:byte" => "SByteValue",
    "xsd:dateTime" => "DateTimeValue",
    "xsd:decimal" => "DecimalValue",
    "xsd:double" => "DoubleValue",
    "xsd:float" => "SingleValue",
    "xsd:int" => "Int32Value",
    "xsd:integer" | "xsd:nonNegativeInteger" => "IntegerValue",
    "xsd:long" => "Int64Value",
    "xsd:short" => "Int16Value",
    "xsd:unsignedByte" => "ByteValue",
    "xsd:unsignedInt" => "UInt32Value",
    "xsd:unsignedLong" => "UInt64Value",
    "xsd:unsignedShort" => "UInt16Value",
    "xvml:ST_Macro" => "StringValue",
    "a:ST_DrawingElementId" => "UInt32Value",
    "msink:ST_Point" => "StringValue",
    "w:ST_DecimalNumber"
    | "w:ST_NonNegativeDecimalNumber"
    | "w:ST_SignedDecimalNumberMax-1"
    | "w:ST_SignedDecimalNumberMax-2"
    | "w:ST_UnsignedDecimalNumberMin1" => "Int32Value",
    "w:ST_HexColorRGB" | "w:ST_StylePaneSortMethods_O12" => "HexBinaryValue",
    "w:ST_UnsignedDecimalNumber" => "UInt32Value",
    "xne:ST_Ref" => "StringValue",
    "xfpb:CT_BagFeatureProperty" => "UInt32Value",
    "xfpb:CT_IntFeatureProperty" => "Int32Value",
    "xfpb:CT_StringFeatureProperty" | "xfpb:CT_RelFeatureProperty" => "StringValue",
    "xfpb:CT_BoolFeatureProperty" => "BooleanValue",
    "xfpb:CT_DecimalFeatureProperty" => "DoubleValue",
    _ => name,
  }
}
