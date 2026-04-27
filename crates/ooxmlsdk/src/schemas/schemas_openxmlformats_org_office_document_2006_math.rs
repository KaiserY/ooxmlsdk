//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShapeDelimiterValues {
  #[sdk(rename = "centered")]
  #[default]
  Centered,
  #[sdk(rename = "match")]
  Match,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FractionTypeValues {
  #[sdk(rename = "bar")]
  #[default]
  Bar,
  #[sdk(rename = "skw")]
  Skewed,
  #[sdk(rename = "lin")]
  Linear,
  #[sdk(rename = "noBar")]
  NoBar,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LimitLocationValues {
  #[sdk(rename = "undOvr")]
  #[default]
  UnderOver,
  #[sdk(rename = "subSup")]
  SubscriptSuperscript,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalJustificationValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "bot")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ScriptValues {
  #[sdk(rename = "roman")]
  #[default]
  Roman,
  #[sdk(rename = "script")]
  Script,
  #[sdk(rename = "fraktur")]
  Fraktur,
  #[sdk(rename = "double-struck")]
  DoubleStruck,
  #[sdk(rename = "sans-serif")]
  SansSerif,
  #[sdk(rename = "monospace")]
  Monospace,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleValues {
  #[sdk(rename = "p")]
  #[default]
  Plain,
  #[sdk(rename = "b")]
  Bold,
  #[sdk(rename = "i")]
  Italic,
  #[sdk(rename = "bi")]
  BoldItalic,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum JustificationValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "centerGroup")]
  CenterGroup,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BreakBinaryOperatorValues {
  #[sdk(rename = "before")]
  #[default]
  Before,
  #[sdk(rename = "after")]
  After,
  #[sdk(rename = "repeat")]
  Repeat,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BreakBinarySubtractionValues {
  #[sdk(rename = "--")]
  #[default]
  MinusMinus,
  #[sdk(rename = "-+")]
  MinusPlus,
  #[sdk(rename = "+-")]
  PlusMinus,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "bot")]
  Bot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanValues {
  #[sdk(rename = "true")]
  #[default]
  True,
  #[sdk(rename = "false")]
  False,
  #[sdk(rename = "on")]
  On,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "0")]
  Zero,
  #[sdk(rename = "1")]
  One,
}
/// Script.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:scr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Script/m:scr")]
pub struct Script {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: ScriptValues,
}
/// style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sty.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Style/m:sty")]
pub struct Style {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: StyleValues,
}
/// Defines the Run Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:r.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_R/m:r")]
pub struct Run {
  ///Run Properties
  #[sdk(child(qname = "m:CT_RPR/m:rPr"))]
  pub math_run_properties: Option<std::boxed::Box<RunProperties>>,
  ///Run Properties.
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::RunProperties,
    >,
  >,
  #[sdk(choice(
    qname = "w:CT_Br/w:br",
    qname = "w:CT_Text/w:t",
    qname = "w:CT_Text/w:delText",
    qname = "w:CT_Text/w:instrText",
    qname = "w:CT_Text/w:delInstrText",
    qname = "w:CT_Empty/w:noBreakHyphen",
    qname = "w:CT_Empty/w:softHyphen",
    qname = "w:CT_Empty/w:dayShort",
    qname = "w:CT_Empty/w:monthShort",
    qname = "w:CT_Empty/w:yearShort",
    qname = "w:CT_Empty/w:dayLong",
    qname = "w:CT_Empty/w:monthLong",
    qname = "w:CT_Empty/w:yearLong",
    qname = "w:CT_Empty/w:annotationRef",
    qname = "w:CT_Empty/w:footnoteRef",
    qname = "w:CT_Empty/w:endnoteRef",
    qname = "w:CT_Empty/w:separator",
    qname = "w:CT_Empty/w:continuationSeparator",
    qname = "w:CT_Sym/w:sym",
    qname = "w:CT_Empty/w:pgNum",
    qname = "w:CT_Empty/w:cr",
    qname = "w:CT_Empty/w:tab",
    qname = "w:CT_Object/w:object",
    qname = "w:CT_Picture/w:pict",
    qname = "w:CT_FldChar/w:fldChar",
    qname = "w:CT_Ruby/w:ruby",
    qname = "w:CT_FtnEdnRef/w:footnoteReference",
    qname = "w:CT_FtnEdnRef/w:endnoteReference",
    qname = "w:CT_Markup/w:commentReference",
    qname = "w:CT_Drawing/w:drawing",
    qname = "w:CT_PTab/w:ptab",
    qname = "w:CT_Empty/w:lastRenderedPageBreak",
    qname = "m:CT_Text/m:t"
  ))]
  pub run_choice: Vec<RunChoice>,
}
/// Accent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:acc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Acc/m:acc")]
pub struct Accent {
  ///Accent Properties
  #[sdk(child(qname = "m:CT_AccPr/m:accPr"))]
  pub accent_properties: Option<std::boxed::Box<AccentProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Bar.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:bar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Bar/m:bar")]
pub struct Bar {
  ///Bar Properties
  #[sdk(child(qname = "m:CT_BarPr/m:barPr"))]
  pub bar_properties: Option<std::boxed::Box<BarProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Box Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:box.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Box/m:box")]
pub struct Box {
  ///Box Properties
  #[sdk(child(qname = "m:CT_BoxPr/m:boxPr"))]
  pub box_properties: Option<std::boxed::Box<BoxProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Border-Box Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:borderBox.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BorderBox/m:borderBox")]
pub struct BorderBox {
  ///Border Box Properties
  #[sdk(child(qname = "m:CT_BorderBoxPr/m:borderBoxPr"))]
  pub border_box_properties: Option<std::boxed::Box<BorderBoxProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Delimiter Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:d.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_D/m:d")]
pub struct Delimiter {
  ///Delimiter Properties
  #[sdk(child(qname = "m:CT_DPr/m:dPr"))]
  pub delimiter_properties: Option<std::boxed::Box<DelimiterProperties>>,
  /// _
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// Equation-Array Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:eqArr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_EqArr/m:eqArr")]
pub struct EquationArray {
  ///Equation Array Properties
  #[sdk(child(qname = "m:CT_EqArrPr/m:eqArrPr"))]
  pub equation_array_properties: Option<std::boxed::Box<EquationArrayProperties>>,
  /// _
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// Fraction Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:f.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_F/m:f")]
pub struct Fraction {
  ///Fraction Properties
  #[sdk(child(qname = "m:CT_FPr/m:fPr"))]
  pub fraction_properties: Option<std::boxed::Box<FractionProperties>>,
  ///Numerator
  #[sdk(child(qname = "m:CT_OMathArg/m:num"))]
  pub numerator: std::boxed::Box<Numerator>,
  ///Denominator
  #[sdk(child(qname = "m:CT_OMathArg/m:den"))]
  pub denominator: std::boxed::Box<Denominator>,
}
/// Function Apply Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:func.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Func/m:func")]
pub struct MathFunction {
  ///Function Properties
  #[sdk(child(qname = "m:CT_FuncPr/m:funcPr"))]
  pub function_properties: Option<std::boxed::Box<FunctionProperties>>,
  ///Function Name
  #[sdk(child(qname = "m:CT_OMathArg/m:fName"))]
  pub function_name: std::boxed::Box<FunctionName>,
  ///Base (Argument)
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Group-Character Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:groupChr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_GroupChr/m:groupChr")]
pub struct GroupChar {
  ///Group-Character Properties
  #[sdk(child(qname = "m:CT_GroupChrPr/m:groupChrPr"))]
  pub group_char_properties: Option<std::boxed::Box<GroupCharProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Lower-Limit Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:limLow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLow/m:limLow")]
pub struct LimitLower {
  ///Lower Limit Properties
  #[sdk(child(qname = "m:CT_LimLowPr/m:limLowPr"))]
  pub limit_lower_properties: Option<std::boxed::Box<LimitLowerProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  ///Limit (Lower)
  #[sdk(child(qname = "m:CT_OMathArg/m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Upper-Limit Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:limUpp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimUpp/m:limUpp")]
pub struct LimitUpper {
  ///Upper Limit Properties
  #[sdk(child(qname = "m:CT_LimUppPr/m:limUppPr"))]
  pub limit_upper_properties: Option<std::boxed::Box<LimitUpperProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  ///Limit (Upper)
  #[sdk(child(qname = "m:CT_OMathArg/m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Matrix Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:m.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_M/m:m")]
pub struct Matrix {
  ///Matrix Properties
  #[sdk(child(qname = "m:CT_MPr/m:mPr"))]
  pub matrix_properties: Option<std::boxed::Box<MatrixProperties>>,
  /// _
  #[sdk(child(qname = "m:CT_MR/m:mr"))]
  pub m_mr: Vec<MatrixRow>,
}
/// n-ary Operator Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:nary.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Nary/m:nary")]
pub struct Nary {
  ///n-ary Properties
  #[sdk(child(qname = "m:CT_NaryPr/m:naryPr"))]
  pub nary_properties: Option<std::boxed::Box<NaryProperties>>,
  ///Lower limit (n-ary)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  ///Upper limit (n-ary)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  ///Base (Argument)
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Phantom Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:phant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Phant/m:phant")]
pub struct Phantom {
  ///Phantom Properties
  #[sdk(child(qname = "m:CT_PhantPr/m:phantPr"))]
  pub phantom_properties: Option<std::boxed::Box<PhantomProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Radical Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:rad.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Rad/m:rad")]
pub struct Radical {
  ///Radical Properties
  #[sdk(child(qname = "m:CT_RadPr/m:radPr"))]
  pub radical_properties: Option<std::boxed::Box<RadicalProperties>>,
  ///Degree
  #[sdk(child(qname = "m:CT_OMathArg/m:deg"))]
  pub degree: std::boxed::Box<Degree>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Pre-Sub-Superscript Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sPre.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SPre/m:sPre")]
pub struct PreSubSuper {
  ///Pre-Sub-Superscript Properties
  #[sdk(child(qname = "m:CT_SPrePr/m:sPrePr"))]
  pub pre_sub_super_properties: Option<std::boxed::Box<PreSubSuperProperties>>,
  ///Subscript (Pre-Sub-Superscript)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  ///Superscript(Pre-Sub-Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Subscript Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSub.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSub/m:sSub")]
pub struct Subscript {
  ///Subscript Properties
  #[sdk(child(qname = "m:CT_SSubPr/m:sSubPr"))]
  pub subscript_properties: Option<std::boxed::Box<SubscriptProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  ///Subscript (Subscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
}
/// Sub-Superscript Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSubSup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubSup/m:sSubSup")]
pub struct SubSuperscript {
  ///Sub-Superscript Properties
  #[sdk(child(qname = "m:CT_SSubSupPr/m:sSubSupPr"))]
  pub sub_superscript_properties: Option<std::boxed::Box<SubSuperscriptProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  ///Subscript (Sub-Superscript)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  ///Superscript (Sub-Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Superscript Function.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSup/m:sSup")]
pub struct Superscript {
  ///Superscript Properties
  #[sdk(child(qname = "m:CT_SSupPr/m:sSupPr"))]
  pub superscript_properties: Option<std::boxed::Box<SuperscriptProperties>>,
  ///Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  ///Superscript (Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Defines the Paragraph Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:oMathPara.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathPara/m:oMathPara")]
pub struct Paragraph {
  ///Office Math Paragraph Properties
  #[sdk(child(qname = "m:CT_OMathParaPr/m:oMathParaPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_OMath/m:oMath",
    qname = "m:CT_R/m:r",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "w:CT_R/w:r"
  ))]
  pub paragraph_choice: Vec<ParagraphChoice>,
}
/// Defines the OfficeMath Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:oMath.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMath/m:oMath")]
pub struct OfficeMath {
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub office_math_choice: Vec<OfficeMathChoice>,
}
/// Math Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mathPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MathPr/m:mathPr")]
pub struct MathProperties {
  ///Math Font
  #[sdk(child(qname = "m:CT_FontFace/m:mathFont"))]
  pub math_font: Option<MathFont>,
  ///Break on Binary Operators
  #[sdk(child(qname = "m:CT_BreakBin/m:brkBin"))]
  pub break_binary: Option<BreakBinary>,
  ///Break on Binary Subtraction
  #[sdk(child(qname = "m:CT_BreakBinSub/m:brkBinSub"))]
  pub break_binary_subtraction: Option<BreakBinarySubtraction>,
  ///Small Fraction
  #[sdk(child(qname = "m:CT_OnOff/m:smallFrac"))]
  pub small_fraction: Option<SmallFraction>,
  ///Use Display Math Defaults
  #[sdk(child(qname = "m:CT_OnOff/m:dispDef"))]
  pub display_defaults: Option<DisplayDefaults>,
  ///Left Margin
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:lMargin"))]
  pub left_margin: Option<LeftMargin>,
  ///Right Margin
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:rMargin"))]
  pub right_margin: Option<RightMargin>,
  ///Default Justification
  #[sdk(child(qname = "m:CT_OMathJc/m:defJc"))]
  pub default_justification: Option<DefaultJustification>,
  ///Pre-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:preSp"))]
  pub pre_spacing: Option<PreSpacing>,
  ///Post-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:postSp"))]
  pub post_spacing: Option<PostSpacing>,
  ///Inter-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:interSp"))]
  pub inter_spacing: Option<InterSpacing>,
  ///Intra-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:intraSp"))]
  pub intra_spacing: Option<IntraSpacing>,
  #[sdk(choice(
    qname = "m:CT_TwipsMeasure/m:wrapIndent",
    qname = "m:CT_OnOff/m:wrapRight"
  ))]
  pub math_properties_choice: Option<MathPropertiesChoice>,
  /// _
  #[sdk(child(qname = "m:CT_LimLoc/m:intLim"))]
  pub m_int_lim: Option<IntegralLimitLocation>,
  /// _
  #[sdk(child(qname = "m:CT_LimLoc/m:naryLim"))]
  pub m_nary_lim: Option<NaryLimitLocation>,
}
/// Literal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:lit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:lit")]
pub struct Literal {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Normal Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:nor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:nor")]
pub struct NormalText {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:aln.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:aln")]
pub struct Alignment {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Operator Emulator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:opEmu.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:opEmu")]
pub struct OperatorEmulator {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// No Break.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:noBreak.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:noBreak")]
pub struct NoBreak {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Differential.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:diff.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:diff")]
pub struct Differential {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Top Edge.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:hideTop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideTop")]
pub struct HideTop {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Bottom Edge.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:hideBot.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideBot")]
pub struct HideBottom {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Left Edge.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:hideLeft.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideLeft")]
pub struct HideLeft {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Right Edge.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:hideRight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideRight")]
pub struct HideRight {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Horizontal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:strikeH.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeH")]
pub struct StrikeHorizontal {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Vertical.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:strikeV.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeV")]
pub struct StrikeVertical {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Bottom-Left to Top-Right.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:strikeBLTR.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeBLTR")]
pub struct StrikeBottomLeftToTopRight {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Top-Left to Bottom-Right.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:strikeTLBR.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeTLBR")]
pub struct StrikeTopLeftToBottomRight {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Delimiter Grow.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:grow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:grow")]
pub struct GrowOperators {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Maximum Distribution.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:maxDist.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:maxDist")]
pub struct MaxDistribution {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Object Distribution.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:objDist.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:objDist")]
pub struct ObjectDistribution {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Placeholders (Matrix).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:plcHide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:plcHide")]
pub struct HidePlaceholder {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Subscript (n-ary).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:subHide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:subHide")]
pub struct HideSubArgument {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Superscript (n-ary).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:supHide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:supHide")]
pub struct HideSuperArgument {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Show.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:show")]
pub struct ShowPhantom {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Width.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:zeroWid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroWid")]
pub struct ZeroWidth {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Ascent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:zeroAsc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroAsc")]
pub struct ZeroAscent {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Descent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:zeroDesc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroDesc")]
pub struct ZeroDescent {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Transparent (Phantom).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:transp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:transp")]
pub struct Transparent {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Degree.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:degHide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:degHide")]
pub struct HideDegree {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align Scripts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:alnScr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:alnScr")]
pub struct AlignScripts {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Small Fraction.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:smallFrac.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:smallFrac")]
pub struct SmallFraction {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Use Display Math Defaults.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:dispDef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:dispDef")]
pub struct DisplayDefaults {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Wrap Right.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:wrapRight.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:wrapRight")]
pub struct WrapRight {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Defines the OnOffType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/")]
pub struct OnOffType {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Break.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:brk.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_ManualBreak/m:brk")]
pub struct Break {
  /// Index of Operator to Align To
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:alnAt
  #[sdk(attr(qname = "m:alnAt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "255",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub align_at: Option<crate::simple_type::IntegerValue>,
  /// Index of Operator to Align To
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "255",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::IntegerValue>,
}
/// Run Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:rPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_RPR/m:rPr")]
pub struct RunProperties {
  ///Literal
  #[sdk(child(qname = "m:CT_OnOff/m:lit"))]
  pub literal: Option<Literal>,
  #[sdk(choice(
    qname = "m:CT_OnOff/m:nor",
    qname = "m:CT_Script/m:scr",
    qname = "m:CT_Style/m:sty"
  ))]
  pub run_properties_choice: Option<RunPropertiesChoice>,
  /// _
  #[sdk(child(qname = "m:CT_ManualBreak/m:brk"))]
  pub m_brk: Option<Break>,
  /// _
  #[sdk(child(qname = "m:CT_OnOff/m:aln"))]
  pub m_aln: Option<Alignment>,
}
/// Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:t.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Text/m:t")]
pub struct Text {
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Accent Character.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:chr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:chr")]
pub struct AccentChar {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Beginning Character.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:begChr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:begChr")]
pub struct BeginChar {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Separator Character.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sepChr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:sepChr")]
pub struct SeparatorChar {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Ending Character.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:endChr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:endChr")]
pub struct EndChar {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the CharType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/")]
pub struct CharType {
  /// value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Control Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:ctrlPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_CtrlPr/m:ctrlPr")]
pub struct ControlProperties {
  #[sdk(choice(
    qname = "w:CT_RPr/w:rPr",
    qname = "w:CT_MathCtrlIns/w:ins",
    qname = "w:CT_MathCtrlDel/w:del",
    qname = "w:CT_MathCtrlMove/w:moveFrom",
    qname = "w:CT_MathCtrlMove/w:moveTo"
  ))]
  pub control_properties_choice: Option<ControlPropertiesChoice>,
}
/// Accent Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:accPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_AccPr/m:accPr")]
pub struct AccentProperties {
  ///Accent Character
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  ///Control Properties
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Base.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:e.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:e")]
pub struct Base {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub base_choice: Vec<BaseChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Numerator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:num.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:num")]
pub struct Numerator {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub numerator_choice: Vec<NumeratorChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Denominator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:den.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:den")]
pub struct Denominator {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub denominator_choice: Vec<DenominatorChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:fName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:fName")]
pub struct FunctionName {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub function_name_choice: Vec<FunctionNameChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Limit (Lower).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:lim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:lim")]
pub struct Limit {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub limit_choice: Vec<LimitChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower limit (n-ary) .
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sub.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sub")]
pub struct SubArgument {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub sub_argument_choice: Vec<SubArgumentChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper limit (n-ary).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sup")]
pub struct SuperArgument {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub super_argument_choice: Vec<SuperArgumentChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Degree.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:deg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:deg")]
pub struct Degree {
  ///Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "w:CT_ContentPart/w:contentPart",
    qname = "w:CT_RunTrackChange/w14:conflictIns",
    qname = "w:CT_RunTrackChange/w14:conflictDel",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath"
  ))]
  pub degree_choice: Vec<DegreeChoice>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Defines the OfficeMathArgumentType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/")]
pub struct OfficeMathArgumentType {
  #[sdk(choice(
    qname = "m:CT_OMathArgPr/m:argPr",
    qname = "m:CT_Acc/m:acc",
    qname = "m:CT_Bar/m:bar",
    qname = "m:CT_Box/m:box",
    qname = "m:CT_BorderBox/m:borderBox",
    qname = "m:CT_D/m:d",
    qname = "m:CT_EqArr/m:eqArr",
    qname = "m:CT_F/m:f",
    qname = "m:CT_Func/m:func",
    qname = "m:CT_GroupChr/m:groupChr",
    qname = "m:CT_LimLow/m:limLow",
    qname = "m:CT_LimUpp/m:limUpp",
    qname = "m:CT_M/m:m",
    qname = "m:CT_Nary/m:nary",
    qname = "m:CT_Phant/m:phant",
    qname = "m:CT_Rad/m:rad",
    qname = "m:CT_SPre/m:sPre",
    qname = "m:CT_SSub/m:sSub",
    qname = "m:CT_SSubSup/m:sSubSup",
    qname = "m:CT_SSup/m:sSup",
    qname = "m:CT_R/m:r",
    qname = "w:CT_CustomXmlRun/w:customXml",
    qname = "w:CT_SimpleField/w:fldSimple",
    qname = "w:CT_Hyperlink/w:hyperlink",
    qname = "w:CT_SdtRun/w:sdt",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveFromRangeStart",
    qname = "w:CT_MarkupRange/w:moveFromRangeEnd",
    qname = "w:CT_MoveBookmark/w:moveToRangeStart",
    qname = "w:CT_MarkupRange/w:moveToRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlInsRangeStart",
    qname = "w:CT_Markup/w:customXmlInsRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlDelRangeStart",
    qname = "w:CT_Markup/w:customXmlDelRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd",
    qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart",
    qname = "w:CT_Markup/w:customXmlMoveToRangeEnd",
    qname = "w:CT_RunTrackChange/w:ins",
    qname = "w:CT_RunTrackChange/w:del",
    qname = "w:CT_RunTrackChange/w:moveFrom",
    qname = "w:CT_RunTrackChange/w:moveTo",
    qname = "m:CT_OMathPara/m:oMathPara",
    qname = "m:CT_OMath/m:oMath",
    qname = "m:CT_CtrlPr/m:ctrlPr"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
      qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
      qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
      qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
      qname = "w:CT_ContentPart/w:contentPart",
      qname = "w:CT_RunTrackChange/w14:conflictIns",
      qname = "w:CT_RunTrackChange/w14:conflictDel"
    ))
  )]
  pub xml_children: Vec<OfficeMathArgumentTypeChoice>,
}
/// Position (Bar).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:pos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TopBot/m:pos")]
pub struct Position {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Vertical Justification.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:vertJc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TopBot/m:vertJc")]
pub struct VerticalJustification {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Defines the TopBottomType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TopBot/")]
pub struct TopBottomType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Bar Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:barPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BarPr/m:barPr")]
pub struct BarProperties {
  ///Position (Bar)
  #[sdk(child(qname = "m:CT_TopBot/m:pos"))]
  pub position: Option<Position>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Box Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:boxPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BoxPr/m:boxPr")]
pub struct BoxProperties {
  ///Operator Emulator
  #[sdk(child(qname = "m:CT_OnOff/m:opEmu"))]
  pub operator_emulator: Option<OperatorEmulator>,
  ///No Break
  #[sdk(child(qname = "m:CT_OnOff/m:noBreak"))]
  pub no_break: Option<NoBreak>,
  ///Differential
  #[sdk(child(qname = "m:CT_OnOff/m:diff"))]
  pub differential: Option<Differential>,
  ///Break
  #[sdk(child(qname = "m:CT_ManualBreak/m:brk"))]
  pub r#break: Option<Break>,
  ///Alignment
  #[sdk(child(qname = "m:CT_OnOff/m:aln"))]
  pub alignment: Option<Alignment>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Border Box Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:borderBoxPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BorderBoxPr/m:borderBoxPr")]
pub struct BorderBoxProperties {
  ///Hide Top Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideTop"))]
  pub hide_top: Option<HideTop>,
  ///Hide Bottom Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideBot"))]
  pub hide_bottom: Option<HideBottom>,
  ///Hide Left Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideLeft"))]
  pub hide_left: Option<HideLeft>,
  ///Hide Right Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideRight"))]
  pub hide_right: Option<HideRight>,
  ///Border Box Strikethrough Horizontal
  #[sdk(child(qname = "m:CT_OnOff/m:strikeH"))]
  pub strike_horizontal: Option<StrikeHorizontal>,
  ///Border Box Strikethrough Vertical
  #[sdk(child(qname = "m:CT_OnOff/m:strikeV"))]
  pub strike_vertical: Option<StrikeVertical>,
  ///Border Box Strikethrough Bottom-Left to Top-Right
  #[sdk(child(qname = "m:CT_OnOff/m:strikeBLTR"))]
  pub strike_bottom_left_to_top_right: Option<StrikeBottomLeftToTopRight>,
  ///Border Box Strikethrough Top-Left to Bottom-Right
  #[sdk(child(qname = "m:CT_OnOff/m:strikeTLBR"))]
  pub strike_top_left_to_bottom_right: Option<StrikeTopLeftToBottomRight>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Shape (Delimiters).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:shp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Shp/m:shp")]
pub struct Shape {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: ShapeDelimiterValues,
}
/// Delimiter Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:dPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_DPr/m:dPr")]
pub struct DelimiterProperties {
  ///Delimiter Beginning Character
  #[sdk(child(qname = "m:CT_Char/m:begChr"))]
  pub begin_char: Option<BeginChar>,
  ///Delimiter Separator Character
  #[sdk(child(qname = "m:CT_Char/m:sepChr"))]
  pub separator_char: Option<SeparatorChar>,
  ///Delimiter Ending Character
  #[sdk(child(qname = "m:CT_Char/m:endChr"))]
  pub end_char: Option<EndChar>,
  ///Delimiter Grow
  #[sdk(child(qname = "m:CT_OnOff/m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  ///Shape (Delimiters)
  #[sdk(child(qname = "m:CT_Shp/m:shp"))]
  pub shape: Option<Shape>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Equation Array Base Justification.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:baseJc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_YAlign/m:baseJc")]
pub struct BaseJustification {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalAlignmentValues,
}
/// Row Spacing Rule.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:rSpRule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SpacingRule/m:rSpRule")]
pub struct RowSpacingRule {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "4",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Gap Rule.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:cGpRule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SpacingRule/m:cGpRule")]
pub struct ColumnGapRule {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "4",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::IntegerValue,
}
/// Defines the SpacingRuleType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SpacingRule/")]
pub struct SpacingRuleType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "4",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::IntegerValue,
}
/// Row Spacing (Equation Array).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:rSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_UnSignedShort/m:rSp")]
pub struct RowSpacing {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Matrix Column Gap.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:cGp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_UnSignedShort/m:cGp")]
pub struct ColumnGap {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Defines the UnsignedShortType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_UnSignedShort/")]
pub struct UnsignedShortType {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Equation Array Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:eqArrPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_EqArrPr/m:eqArrPr")]
pub struct EquationArrayProperties {
  ///Equation Array Base Justification
  #[sdk(child(qname = "m:CT_YAlign/m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  ///Maximum Distribution
  #[sdk(child(qname = "m:CT_OnOff/m:maxDist"))]
  pub max_distribution: Option<MaxDistribution>,
  ///Object Distribution
  #[sdk(child(qname = "m:CT_OnOff/m:objDist"))]
  pub object_distribution: Option<ObjectDistribution>,
  ///Row Spacing Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  ///Row Spacing (Equation Array)
  #[sdk(child(qname = "m:CT_UnSignedShort/m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Fraction type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FType/m:type")]
pub struct FractionType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: FractionTypeValues,
}
/// Fraction Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:fPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FPr/m:fPr")]
pub struct FractionProperties {
  ///Fraction type
  #[sdk(child(qname = "m:CT_FType/m:type"))]
  pub fraction_type: Option<FractionType>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:funcPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FuncPr/m:funcPr")]
pub struct FunctionProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Group-Character Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:groupChrPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_GroupChrPr/m:groupChrPr")]
pub struct GroupCharProperties {
  ///Group Character (Grouping Character)
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  ///Position (Group Character)
  #[sdk(child(qname = "m:CT_TopBot/m:pos"))]
  pub position: Option<Position>,
  ///Vertical Justification
  #[sdk(child(qname = "m:CT_TopBot/m:vertJc"))]
  pub vertical_justification: Option<VerticalJustification>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower Limit Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:limLowPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLowPr/m:limLowPr")]
pub struct LimitLowerProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper Limit Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:limUppPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimUppPr/m:limUppPr")]
pub struct LimitUpperProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Column Count.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:count.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Integer64/m:count")]
pub struct MatrixColumnCount {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "1",
    max = "64",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Justification.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mcJc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_XAlign/m:mcJc")]
pub struct MatrixColumnJustification {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: HorizontalAlignmentValues,
}
/// Matrix Column Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mcPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MCPr/m:mcPr")]
pub struct MatrixColumnProperties {
  ///Matrix Column Count
  #[sdk(child(qname = "m:CT_Integer64/m:count"))]
  pub matrix_column_count: Option<MatrixColumnCount>,
  ///Matrix Column Justification
  #[sdk(child(qname = "m:CT_XAlign/m:mcJc"))]
  pub matrix_column_justification: Option<MatrixColumnJustification>,
}
/// Matrix Column.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MC/m:mc")]
pub struct MatrixColumn {
  ///Matrix Column Properties
  #[sdk(child(qname = "m:CT_MCPr/m:mcPr"))]
  pub matrix_column_properties: Option<std::boxed::Box<MatrixColumnProperties>>,
}
/// Matrix Column Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:cSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:cSp")]
pub struct ColumnSpacing {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Left Margin.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:lMargin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:lMargin")]
pub struct LeftMargin {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Right Margin.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:rMargin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:rMargin")]
pub struct RightMargin {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Pre-Equation Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:preSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:preSp")]
pub struct PreSpacing {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Post-Equation Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:postSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:postSp")]
pub struct PostSpacing {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Inter-Equation Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:interSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:interSp")]
pub struct InterSpacing {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Intra-Equation Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:intraSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:intraSp")]
pub struct IntraSpacing {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Wrap Indent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:wrapIndent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:wrapIndent")]
pub struct WrapIndent {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the TwipsMeasureType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/")]
pub struct TwipsMeasureType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    max = "31680",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::UInt32Value,
}
/// Matrix Columns.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mcs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MCS/m:mcs")]
pub struct MatrixColumns {
  /// _
  #[sdk(child(qname = "m:CT_MC/m:mc"))]
  pub m_mc: Vec<MatrixColumn>,
}
/// Matrix Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MPr/m:mPr")]
pub struct MatrixProperties {
  ///Matrix Base Justification
  #[sdk(child(qname = "m:CT_YAlign/m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  ///Hide Placeholders (Matrix)
  #[sdk(child(qname = "m:CT_OnOff/m:plcHide"))]
  pub hide_placeholder: Option<HidePlaceholder>,
  ///Row Spacing Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  ///Matrix Column Gap Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:cGpRule"))]
  pub column_gap_rule: Option<ColumnGapRule>,
  ///Row Spacing (Matrix)
  #[sdk(child(qname = "m:CT_UnSignedShort/m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  ///Matrix Column Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:cSp"))]
  pub column_spacing: Option<ColumnSpacing>,
  ///Matrix Column Gap
  #[sdk(child(qname = "m:CT_UnSignedShort/m:cGp"))]
  pub column_gap: Option<ColumnGap>,
  ///Matrix Columns
  #[sdk(child(qname = "m:CT_MCS/m:mcs"))]
  pub matrix_columns: Option<MatrixColumns>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Row.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MR/m:mr")]
pub struct MatrixRow {
  /// _
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// n-ary Limit Location.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:limLoc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:limLoc")]
pub struct LimitLocation {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// Integral Limit Locations.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:intLim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:intLim")]
pub struct IntegralLimitLocation {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Limit Location.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:naryLim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:naryLim")]
pub struct NaryLimitLocation {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// Defines the LimitLocationType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/")]
pub struct LimitLocationType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:naryPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_NaryPr/m:naryPr")]
pub struct NaryProperties {
  ///n-ary Operator Character
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  ///n-ary Limit Location
  #[sdk(child(qname = "m:CT_LimLoc/m:limLoc"))]
  pub limit_location: Option<LimitLocation>,
  ///n-ary Grow
  #[sdk(child(qname = "m:CT_OnOff/m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  ///Hide Subscript (n-ary)
  #[sdk(child(qname = "m:CT_OnOff/m:subHide"))]
  pub hide_sub_argument: Option<HideSubArgument>,
  ///Hide Superscript (n-ary)
  #[sdk(child(qname = "m:CT_OnOff/m:supHide"))]
  pub hide_super_argument: Option<HideSuperArgument>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Phantom Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:phantPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_PhantPr/m:phantPr")]
pub struct PhantomProperties {
  ///Phantom Show
  #[sdk(child(qname = "m:CT_OnOff/m:show"))]
  pub show_phantom: Option<ShowPhantom>,
  ///Phantom Zero Width
  #[sdk(child(qname = "m:CT_OnOff/m:zeroWid"))]
  pub zero_width: Option<ZeroWidth>,
  ///Phantom Zero Ascent
  #[sdk(child(qname = "m:CT_OnOff/m:zeroAsc"))]
  pub zero_ascent: Option<ZeroAscent>,
  ///Phantom Zero Descent
  #[sdk(child(qname = "m:CT_OnOff/m:zeroDesc"))]
  pub zero_descent: Option<ZeroDescent>,
  ///Transparent (Phantom)
  #[sdk(child(qname = "m:CT_OnOff/m:transp"))]
  pub transparent: Option<Transparent>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Radical Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:radPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_RadPr/m:radPr")]
pub struct RadicalProperties {
  ///Hide Degree
  #[sdk(child(qname = "m:CT_OnOff/m:degHide"))]
  pub hide_degree: Option<HideDegree>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Pre-Sub-Superscript Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sPrePr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SPrePr/m:sPrePr")]
pub struct PreSubSuperProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Subscript Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSubPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubPr/m:sSubPr")]
pub struct SubscriptProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Sub-Superscript Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSubSupPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubSupPr/m:sSubSupPr")]
pub struct SubSuperscriptProperties {
  ///Align Scripts
  #[sdk(child(qname = "m:CT_OnOff/m:alnScr"))]
  pub align_scripts: Option<AlignScripts>,
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Superscript Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:sSupPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSupPr/m:sSupPr")]
pub struct SuperscriptProperties {
  /// _
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Argument Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:argSz.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Integer2/m:argSz")]
pub struct ArgumentSize {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-2",
    max = "2",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::IntegerValue,
}
/// Argument Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:argPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArgPr/m:argPr")]
pub struct ArgumentProperties {
  ///Argument Size
  #[sdk(child(qname = "m:CT_Integer2/m:argSz"))]
  pub argument_size: Option<ArgumentSize>,
}
/// Justification.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:jc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathJc/m:jc")]
pub struct Justification {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Default Justification.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:defJc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathJc/m:defJc")]
pub struct DefaultJustification {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Defines the OfficeMathJustificationType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathJc/")]
pub struct OfficeMathJustificationType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Math Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:mathFont.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FontFace/m:mathFont")]
pub struct MathFont {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(source = 1u32, max = 31u32))]
  pub val: crate::simple_type::StringValue,
}
/// Break on Binary Operators.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:brkBin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BreakBin/m:brkBin")]
pub struct BreakBinary {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BreakBinaryOperatorValues>,
}
/// Break on Binary Subtraction.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:brkBinSub.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BreakBinSub/m:brkBinSub")]
pub struct BreakBinarySubtraction {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: m:val
  #[sdk(attr(qname = "m:val"))]
  pub val: BreakBinarySubtractionValues,
}
/// Office Math Paragraph Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is m:oMathParaPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathParaPr/m:oMathParaPr")]
pub struct ParagraphProperties {
  ///Justification
  #[sdk(child(qname = "m:CT_OMathJc/m:jc"))]
  pub justification: Option<Justification>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunChoice {
  #[sdk(child(qname = "w:CT_Br/w:br"))]
    WBr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Break,
        >,
    ),
    #[sdk(child(qname = "w:CT_Text/w:t"))]
    WT(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Text,
        >,
    ),
    #[sdk(child(qname = "w:CT_Text/w:delText"))]
    WDelText(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedText,
        >,
    ),
    #[sdk(child(qname = "w:CT_Text/w:instrText"))]
    WInstrText(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FieldCode,
        >,
    ),
    #[sdk(child(qname = "w:CT_Text/w:delInstrText"))]
    WDelInstrText(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedFieldCode,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:noBreakHyphen"))]
    WNoBreakHyphen(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::NoBreakHyphen,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:softHyphen"))]
    WSoftHyphen(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SoftHyphen,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:dayShort"))]
    WDayShort(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DayShort,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:monthShort"))]
    WMonthShort(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MonthShort,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:yearShort"))]
    WYearShort(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::YearShort,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:dayLong"))]
    WDayLong(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DayLong,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:monthLong"))]
    WMonthLong(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MonthLong,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:yearLong"))]
    WYearLong(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::YearLong,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:annotationRef"))]
    WAnnotationRef(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::AnnotationReferenceMark,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:footnoteRef"))]
    WFootnoteRef(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FootnoteReferenceMark,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:endnoteRef"))]
    WEndnoteRef(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EndnoteReferenceMark,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:separator"))]
    WSeparator(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SeparatorMark,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:continuationSeparator"))]
    WContinuationSeparator(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContinuationSeparatorMark,
        >,
    ),
    #[sdk(child(qname = "w:CT_Sym/w:sym"))]
    WSym(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SymbolChar,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:pgNum"))]
    WPgNum(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PageNumber,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:cr"))]
    WCr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CarriageReturn,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:tab"))]
    WTab(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TabChar,
        >,
    ),
    #[sdk(child(qname = "w:CT_Object/w:object"))]
    WObject(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EmbeddedObject,
        >,
    ),
    #[sdk(child(qname = "w:CT_Picture/w:pict"))]
    WPict(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Picture,
        >,
    ),
    #[sdk(child(qname = "w:CT_FldChar/w:fldChar"))]
    WFldChar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FieldChar,
        >,
    ),
    #[sdk(child(qname = "w:CT_Ruby/w:ruby"))]
    WRuby(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Ruby,
        >,
    ),
    #[sdk(child(qname = "w:CT_FtnEdnRef/w:footnoteReference"))]
    WFootnoteReference(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FootnoteReference,
        >,
    ),
    #[sdk(child(qname = "w:CT_FtnEdnRef/w:endnoteReference"))]
    WEndnoteReference(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EndnoteReference,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:commentReference"))]
    WCommentReference(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentReference,
        >,
    ),
    #[sdk(child(qname = "w:CT_Drawing/w:drawing"))]
    WDrawing(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Drawing,
        >,
    ),
    #[sdk(child(qname = "w:CT_PTab/w:ptab"))]
    WPtab(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PositionalTab,
        >,
    ),
    #[sdk(child(qname = "w:CT_Empty/w:lastRenderedPageBreak"))]
    WLastRenderedPageBreak(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::LastRenderedPageBreak,
        >,
    ),
    #[sdk(child(qname = "m:CT_Text/m:t"))]
    MT(std::boxed::Box<Text>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
    McAlternateContent(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<OfficeMathChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MathPropertiesChoice {
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:wrapIndent"))]
  MWrapIndent(std::boxed::Box<WrapIndent>),
  #[sdk(child(qname = "m:CT_OnOff/m:wrapRight"))]
  MWrapRight(std::boxed::Box<WrapRight>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunPropertiesChoiceSequence {
  ///Script.
  #[sdk(child(qname = "m:CT_Script/m:scr"))]
  pub script: Option<Script>,
  ///style.
  #[sdk(child(qname = "m:CT_Style/m:sty"))]
  pub style: Option<Style>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice {
  #[sdk(child(qname = "m:CT_OnOff/m:nor"))]
  MNor(std::boxed::Box<NormalText>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<RunPropertiesChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ControlPropertiesChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  EgRPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::RunProperties,
    >,
  ),
  #[sdk(child(qname = "w:CT_MathCtrlIns/w:ins"))]
  WIns(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedMathControl,
    >,
  ),
  #[sdk(child(qname = "w:CT_MathCtrlDel/w:del"))]
  WDel(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedMathControl,
    >,
  ),
  #[sdk(child(qname = "w:CT_MathCtrlMove/w:moveFrom"))]
  WMoveFrom(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromMathControl,
    >,
  ),
  #[sdk(child(qname = "w:CT_MathCtrlMove/w:moveTo"))]
  WMoveTo(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToMathControl,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<BaseChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<NumeratorChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<DenominatorChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<FunctionNameChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<LimitChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<SubArgumentChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<SuperArgumentChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice2 {
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(sequence)]
    Sequence {
        ///Defines the RunConflictInsertion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        ///Defines the RunConflictDeletion Class.
        #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    >,
  ),
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    >,
  ),
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    >,
  ),
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun>,
  ),
  #[sdk(choice)]
  EgRunLevelElts(std::boxed::Box<DegreeChoice2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathArgumentTypeChoice {
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
    MArgPr(std::boxed::Box<ArgumentProperties>),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(std::boxed::Box<Accent>),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(std::boxed::Box<Bar>),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(std::boxed::Box<Box>),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(std::boxed::Box<BorderBox>),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(std::boxed::Box<Delimiter>),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(std::boxed::Box<EquationArray>),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(std::boxed::Box<Fraction>),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(std::boxed::Box<MathFunction>),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(std::boxed::Box<GroupChar>),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(std::boxed::Box<LimitLower>),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(std::boxed::Box<LimitUpper>),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(std::boxed::Box<Matrix>),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(std::boxed::Box<Nary>),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(std::boxed::Box<Phantom>),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(std::boxed::Box<Radical>),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(std::boxed::Box<PreSubSuper>),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(std::boxed::Box<Subscript>),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(std::boxed::Box<SubSuperscript>),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(std::boxed::Box<Superscript>),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
        >,
    ),
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
        >,
    ),
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"))]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"))]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictIns"))]
    W14ConflictIns(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "w:CT_RunTrackChange/w14:conflictDel"))]
    W14ConflictDel(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(std::boxed::Box<OfficeMath>),
    #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
    MCtrlPr(std::boxed::Box<ControlProperties>),
}
