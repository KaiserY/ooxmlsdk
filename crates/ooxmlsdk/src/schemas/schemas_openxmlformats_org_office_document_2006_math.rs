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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Script/m:scr")]
pub struct Script {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: ScriptValues,
}
/// style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Style/m:sty")]
pub struct Style {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: StyleValues,
}
/// Defines the Run Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_R/m:r")]
pub struct Run {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Run Properties
  #[sdk(child(qname = "m:CT_RPR/m:rPr"))]
  pub math_run_properties: Option<std::boxed::Box<RunProperties>>,
  /// _
  #[sdk(sequence(qname = "w:CT_RPr/w:rPr"))]
  pub sequence: Option<std::boxed::Box<RunSequence>>,
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
    qname = "m:CT_Text/m:t",
    any
  ))]
  pub choice: Vec<RunChoice2>,
}
/// Accent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Acc/m:acc")]
pub struct Accent {
  /// Accent Properties
  #[sdk(child(qname = "m:CT_AccPr/m:accPr"))]
  pub accent_properties: Option<std::boxed::Box<AccentProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Bar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Bar/m:bar")]
pub struct Bar {
  /// Bar Properties
  #[sdk(child(qname = "m:CT_BarPr/m:barPr"))]
  pub bar_properties: Option<std::boxed::Box<BarProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Box Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Box/m:box")]
pub struct Box {
  /// Box Properties
  #[sdk(child(qname = "m:CT_BoxPr/m:boxPr"))]
  pub box_properties: Option<std::boxed::Box<BoxProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Border-Box Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BorderBox/m:borderBox")]
pub struct BorderBox {
  /// Border Box Properties
  #[sdk(child(qname = "m:CT_BorderBoxPr/m:borderBoxPr"))]
  pub border_box_properties: Option<std::boxed::Box<BorderBoxProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Delimiter Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_D/m:d")]
pub struct Delimiter {
  /// Delimiter Properties
  #[sdk(child(qname = "m:CT_DPr/m:dPr"))]
  pub delimiter_properties: Option<std::boxed::Box<DelimiterProperties>>,
  /// Base.
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// Equation-Array Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_EqArr/m:eqArr")]
pub struct EquationArray {
  /// Equation Array Properties
  #[sdk(child(qname = "m:CT_EqArrPr/m:eqArrPr"))]
  pub equation_array_properties: Option<std::boxed::Box<EquationArrayProperties>>,
  /// Base.
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// Fraction Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_F/m:f")]
pub struct Fraction {
  /// Fraction Properties
  #[sdk(child(qname = "m:CT_FPr/m:fPr"))]
  pub fraction_properties: Option<std::boxed::Box<FractionProperties>>,
  /// Numerator
  #[sdk(child(qname = "m:CT_OMathArg/m:num"))]
  pub numerator: std::boxed::Box<Numerator>,
  /// Denominator
  #[sdk(child(qname = "m:CT_OMathArg/m:den"))]
  pub denominator: std::boxed::Box<Denominator>,
}
/// Function Apply Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Func/m:func")]
pub struct MathFunction {
  /// Function Properties
  #[sdk(child(qname = "m:CT_FuncPr/m:funcPr"))]
  pub function_properties: Option<std::boxed::Box<FunctionProperties>>,
  /// Function Name
  #[sdk(child(qname = "m:CT_OMathArg/m:fName"))]
  pub function_name: std::boxed::Box<FunctionName>,
  /// Base (Argument)
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Group-Character Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_GroupChr/m:groupChr")]
pub struct GroupChar {
  /// Group-Character Properties
  #[sdk(child(qname = "m:CT_GroupChrPr/m:groupChrPr"))]
  pub group_char_properties: Option<std::boxed::Box<GroupCharProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Lower-Limit Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLow/m:limLow")]
pub struct LimitLower {
  /// Lower Limit Properties
  #[sdk(child(qname = "m:CT_LimLowPr/m:limLowPr"))]
  pub limit_lower_properties: Option<std::boxed::Box<LimitLowerProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Limit (Lower)
  #[sdk(child(qname = "m:CT_OMathArg/m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Upper-Limit Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimUpp/m:limUpp")]
pub struct LimitUpper {
  /// Upper Limit Properties
  #[sdk(child(qname = "m:CT_LimUppPr/m:limUppPr"))]
  pub limit_upper_properties: Option<std::boxed::Box<LimitUpperProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Limit (Upper)
  #[sdk(child(qname = "m:CT_OMathArg/m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Matrix Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_M/m:m")]
pub struct Matrix {
  /// Matrix Properties
  #[sdk(child(qname = "m:CT_MPr/m:mPr"))]
  pub matrix_properties: Option<std::boxed::Box<MatrixProperties>>,
  /// Matrix Row.
  #[sdk(child(qname = "m:CT_MR/m:mr"))]
  pub m_mr: Vec<MatrixRow>,
}
/// n-ary Operator Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Nary/m:nary")]
pub struct Nary {
  /// n-ary Properties
  #[sdk(child(qname = "m:CT_NaryPr/m:naryPr"))]
  pub nary_properties: Option<std::boxed::Box<NaryProperties>>,
  /// Lower limit (n-ary)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Upper limit (n-ary)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  /// Base (Argument)
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Phantom Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Phant/m:phant")]
pub struct Phantom {
  /// Phantom Properties
  #[sdk(child(qname = "m:CT_PhantPr/m:phantPr"))]
  pub phantom_properties: Option<std::boxed::Box<PhantomProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Radical Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Rad/m:rad")]
pub struct Radical {
  /// Radical Properties
  #[sdk(child(qname = "m:CT_RadPr/m:radPr"))]
  pub radical_properties: Option<std::boxed::Box<RadicalProperties>>,
  /// Degree
  #[sdk(child(qname = "m:CT_OMathArg/m:deg"))]
  pub degree: std::boxed::Box<Degree>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Pre-Sub-Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SPre/m:sPre")]
pub struct PreSubSuper {
  /// Pre-Sub-Superscript Properties
  #[sdk(child(qname = "m:CT_SPrePr/m:sPrePr"))]
  pub pre_sub_super_properties: Option<std::boxed::Box<PreSubSuperProperties>>,
  /// Subscript (Pre-Sub-Superscript)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Superscript(Pre-Sub-Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Subscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSub/m:sSub")]
pub struct Subscript {
  /// Subscript Properties
  #[sdk(child(qname = "m:CT_SSubPr/m:sSubPr"))]
  pub subscript_properties: Option<std::boxed::Box<SubscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Subscript (Subscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
}
/// Sub-Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubSup/m:sSubSup")]
pub struct SubSuperscript {
  /// Sub-Superscript Properties
  #[sdk(child(qname = "m:CT_SSubSupPr/m:sSubSupPr"))]
  pub sub_superscript_properties: Option<std::boxed::Box<SubSuperscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Subscript (Sub-Superscript)
  #[sdk(child(qname = "m:CT_OMathArg/m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Superscript (Sub-Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSup/m:sSup")]
pub struct Superscript {
  /// Superscript Properties
  #[sdk(child(qname = "m:CT_SSupPr/m:sSupPr"))]
  pub superscript_properties: Option<std::boxed::Box<SuperscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Superscript (Superscript function)
  #[sdk(child(qname = "m:CT_OMathArg/m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Defines the Paragraph Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathPara/m:oMathPara")]
pub struct Paragraph {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Office Math Paragraph Properties
  #[sdk(child(qname = "m:CT_OMathParaPr/m:oMathParaPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(choice(
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
    qname = "w:CT_R/w:r",
    any
  ))]
  pub choice: Vec<ParagraphChoice9>,
}
/// Defines the OfficeMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMath/m:oMath")]
pub struct OfficeMath {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  #[sdk(choice(
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MathPr/m:mathPr")]
pub struct MathProperties {
  /// Math Font
  #[sdk(child(qname = "m:CT_FontFace/m:mathFont"))]
  pub math_font: Option<MathFont>,
  /// Break on Binary Operators
  #[sdk(child(qname = "m:CT_BreakBin/m:brkBin"))]
  pub break_binary: Option<BreakBinary>,
  /// Break on Binary Subtraction
  #[sdk(child(qname = "m:CT_BreakBinSub/m:brkBinSub"))]
  pub break_binary_subtraction: Option<BreakBinarySubtraction>,
  /// Small Fraction
  #[sdk(child(qname = "m:CT_OnOff/m:smallFrac"))]
  pub small_fraction: Option<SmallFraction>,
  /// Use Display Math Defaults
  #[sdk(child(qname = "m:CT_OnOff/m:dispDef"))]
  pub display_defaults: Option<DisplayDefaults>,
  /// Left Margin
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:lMargin"))]
  pub left_margin: Option<LeftMargin>,
  /// Right Margin
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:rMargin"))]
  pub right_margin: Option<RightMargin>,
  /// Default Justification
  #[sdk(child(qname = "m:CT_OMathJc/m:defJc"))]
  pub default_justification: Option<DefaultJustification>,
  /// Pre-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:preSp"))]
  pub pre_spacing: Option<PreSpacing>,
  /// Post-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:postSp"))]
  pub post_spacing: Option<PostSpacing>,
  /// Inter-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:interSp"))]
  pub inter_spacing: Option<InterSpacing>,
  /// Intra-Equation Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:intraSp"))]
  pub intra_spacing: Option<IntraSpacing>,
  #[sdk(choice(
    qname = "m:CT_TwipsMeasure/m:wrapIndent",
    qname = "m:CT_OnOff/m:wrapRight"
  ))]
  pub choice: Option<MathPropertiesChoice>,
  /// Integral Limit Locations.
  #[sdk(child(qname = "m:CT_LimLoc/m:intLim"))]
  pub m_int_lim: Option<IntegralLimitLocation>,
  /// n-ary Limit Location.
  #[sdk(child(qname = "m:CT_LimLoc/m:naryLim"))]
  pub m_nary_lim: Option<NaryLimitLocation>,
}
/// Literal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:lit")]
pub struct Literal {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Normal Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:nor")]
pub struct NormalText {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:aln")]
pub struct Alignment {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Operator Emulator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:opEmu")]
pub struct OperatorEmulator {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// No Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:noBreak")]
pub struct NoBreak {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Differential.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:diff")]
pub struct Differential {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Top Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideTop")]
pub struct HideTop {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Bottom Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideBot")]
pub struct HideBottom {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Left Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideLeft")]
pub struct HideLeft {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Right Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:hideRight")]
pub struct HideRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeH")]
pub struct StrikeHorizontal {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeV")]
pub struct StrikeVertical {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Bottom-Left to Top-Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeBLTR")]
pub struct StrikeBottomLeftToTopRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Top-Left to Bottom-Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:strikeTLBR")]
pub struct StrikeTopLeftToBottomRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Delimiter Grow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:grow")]
pub struct GrowOperators {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Maximum Distribution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:maxDist")]
pub struct MaxDistribution {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Object Distribution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:objDist")]
pub struct ObjectDistribution {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Placeholders (Matrix).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:plcHide")]
pub struct HidePlaceholder {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Subscript (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:subHide")]
pub struct HideSubArgument {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Superscript (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:supHide")]
pub struct HideSuperArgument {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:show")]
pub struct ShowPhantom {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroWid")]
pub struct ZeroWidth {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Ascent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroAsc")]
pub struct ZeroAscent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Descent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:zeroDesc")]
pub struct ZeroDescent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Transparent (Phantom).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:transp")]
pub struct Transparent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Degree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:degHide")]
pub struct HideDegree {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align Scripts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:alnScr")]
pub struct AlignScripts {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Small Fraction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:smallFrac")]
pub struct SmallFraction {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Use Display Math Defaults.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:dispDef")]
pub struct DisplayDefaults {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Wrap Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OnOff/m:wrapRight")]
pub struct WrapRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_ManualBreak/m:brk")]
pub struct Break {
  /// Index of Operator to Align To
  #[sdk(attr(qname = "m:alnAt"))]
  #[sdk(number_range(range = 1..= 255))]
  pub align_at: Option<crate::simple_type::IntegerValue>,
  /// Index of Operator to Align To
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 1..= 255))]
  pub val: Option<crate::simple_type::IntegerValue>,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_RPR/m:rPr")]
pub struct RunProperties {
  /// Literal
  #[sdk(child(qname = "m:CT_OnOff/m:lit"))]
  pub literal: Option<Literal>,
  #[sdk(choice(
    qname = "m:CT_OnOff/m:nor",
    qname = "m:CT_Script/m:scr",
    qname = "m:CT_Style/m:sty"
  ))]
  pub choice: RunPropertiesChoice,
  /// Break.
  #[sdk(child(qname = "m:CT_ManualBreak/m:brk"))]
  pub m_brk: Option<Break>,
  /// Align.
  #[sdk(child(qname = "m:CT_OnOff/m:aln"))]
  pub m_aln: Option<Alignment>,
}
/// Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Text/m:t")]
pub struct Text {
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Accent Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:chr")]
pub struct AccentChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Beginning Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:begChr")]
pub struct BeginChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Separator Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:sepChr")]
pub struct SeparatorChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Ending Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Char/m:endChr")]
pub struct EndChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Control Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_CtrlPr/m:ctrlPr")]
pub struct ControlProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_AccPr/m:accPr")]
pub struct AccentProperties {
  /// Accent Character
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// Control Properties
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Base.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:e")]
pub struct Base {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<BaseChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Numerator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:num")]
pub struct Numerator {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<NumeratorChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Denominator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:den")]
pub struct Denominator {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<DenominatorChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:fName")]
pub struct FunctionName {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<FunctionNameChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Limit (Lower).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:lim")]
pub struct Limit {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<LimitChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower limit (n-ary) .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sub")]
pub struct SubArgument {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<SubArgumentChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper limit (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sup")]
pub struct SuperArgument {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<SuperArgumentChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Degree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:deg")]
pub struct Degree {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(choice(
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
  pub choice: Vec<DegreeChoice15>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub m_ctrl_pr: Option<std::boxed::Box<ControlProperties>>,
}
/// Position (Bar).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TopBot/m:pos")]
pub struct Position {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Vertical Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TopBot/m:vertJc")]
pub struct VerticalJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Bar Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BarPr/m:barPr")]
pub struct BarProperties {
  /// Position (Bar)
  #[sdk(child(qname = "m:CT_TopBot/m:pos"))]
  pub position: Option<Position>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Box Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BoxPr/m:boxPr")]
pub struct BoxProperties {
  /// Operator Emulator
  #[sdk(child(qname = "m:CT_OnOff/m:opEmu"))]
  pub operator_emulator: Option<OperatorEmulator>,
  /// No Break
  #[sdk(child(qname = "m:CT_OnOff/m:noBreak"))]
  pub no_break: Option<NoBreak>,
  /// Differential
  #[sdk(child(qname = "m:CT_OnOff/m:diff"))]
  pub differential: Option<Differential>,
  /// Break
  #[sdk(child(qname = "m:CT_ManualBreak/m:brk"))]
  pub r#break: Option<Break>,
  /// Alignment
  #[sdk(child(qname = "m:CT_OnOff/m:aln"))]
  pub alignment: Option<Alignment>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Border Box Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BorderBoxPr/m:borderBoxPr")]
pub struct BorderBoxProperties {
  /// Hide Top Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideTop"))]
  pub hide_top: Option<HideTop>,
  /// Hide Bottom Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideBot"))]
  pub hide_bottom: Option<HideBottom>,
  /// Hide Left Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideLeft"))]
  pub hide_left: Option<HideLeft>,
  /// Hide Right Edge
  #[sdk(child(qname = "m:CT_OnOff/m:hideRight"))]
  pub hide_right: Option<HideRight>,
  /// Border Box Strikethrough Horizontal
  #[sdk(child(qname = "m:CT_OnOff/m:strikeH"))]
  pub strike_horizontal: Option<StrikeHorizontal>,
  /// Border Box Strikethrough Vertical
  #[sdk(child(qname = "m:CT_OnOff/m:strikeV"))]
  pub strike_vertical: Option<StrikeVertical>,
  /// Border Box Strikethrough Bottom-Left to Top-Right
  #[sdk(child(qname = "m:CT_OnOff/m:strikeBLTR"))]
  pub strike_bottom_left_to_top_right: Option<StrikeBottomLeftToTopRight>,
  /// Border Box Strikethrough Top-Left to Bottom-Right
  #[sdk(child(qname = "m:CT_OnOff/m:strikeTLBR"))]
  pub strike_top_left_to_bottom_right: Option<StrikeTopLeftToBottomRight>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Shape (Delimiters).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Shp/m:shp")]
pub struct Shape {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: ShapeDelimiterValues,
}
/// Delimiter Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_DPr/m:dPr")]
pub struct DelimiterProperties {
  /// Delimiter Beginning Character
  #[sdk(child(qname = "m:CT_Char/m:begChr"))]
  pub begin_char: Option<BeginChar>,
  /// Delimiter Separator Character
  #[sdk(child(qname = "m:CT_Char/m:sepChr"))]
  pub separator_char: Option<SeparatorChar>,
  /// Delimiter Ending Character
  #[sdk(child(qname = "m:CT_Char/m:endChr"))]
  pub end_char: Option<EndChar>,
  /// Delimiter Grow
  #[sdk(child(qname = "m:CT_OnOff/m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  /// Shape (Delimiters)
  #[sdk(child(qname = "m:CT_Shp/m:shp"))]
  pub shape: Option<Shape>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Equation Array Base Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_YAlign/m:baseJc")]
pub struct BaseJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalAlignmentValues,
}
/// Row Spacing Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SpacingRule/m:rSpRule")]
pub struct RowSpacingRule {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 0..= 4))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Gap Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SpacingRule/m:cGpRule")]
pub struct ColumnGapRule {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 0..= 4))]
  pub val: crate::simple_type::IntegerValue,
}
/// Row Spacing (Equation Array).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_UnSignedShort/m:rSp")]
pub struct RowSpacing {
  /// val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Matrix Column Gap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_UnSignedShort/m:cGp")]
pub struct ColumnGap {
  /// val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Equation Array Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_EqArrPr/m:eqArrPr")]
pub struct EquationArrayProperties {
  /// Equation Array Base Justification
  #[sdk(child(qname = "m:CT_YAlign/m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  /// Maximum Distribution
  #[sdk(child(qname = "m:CT_OnOff/m:maxDist"))]
  pub max_distribution: Option<MaxDistribution>,
  /// Object Distribution
  #[sdk(child(qname = "m:CT_OnOff/m:objDist"))]
  pub object_distribution: Option<ObjectDistribution>,
  /// Row Spacing Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  /// Row Spacing (Equation Array)
  #[sdk(child(qname = "m:CT_UnSignedShort/m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Fraction type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FType/m:type")]
pub struct FractionType {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: FractionTypeValues,
}
/// Fraction Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FPr/m:fPr")]
pub struct FractionProperties {
  /// Fraction type
  #[sdk(child(qname = "m:CT_FType/m:type"))]
  pub fraction_type: Option<FractionType>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FuncPr/m:funcPr")]
pub struct FunctionProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Group-Character Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_GroupChrPr/m:groupChrPr")]
pub struct GroupCharProperties {
  /// Group Character (Grouping Character)
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// Position (Group Character)
  #[sdk(child(qname = "m:CT_TopBot/m:pos"))]
  pub position: Option<Position>,
  /// Vertical Justification
  #[sdk(child(qname = "m:CT_TopBot/m:vertJc"))]
  pub vertical_justification: Option<VerticalJustification>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower Limit Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLowPr/m:limLowPr")]
pub struct LimitLowerProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper Limit Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimUppPr/m:limUppPr")]
pub struct LimitUpperProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Column Count.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Integer64/m:count")]
pub struct MatrixColumnCount {
  /// val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 1..= 64))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_XAlign/m:mcJc")]
pub struct MatrixColumnJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: HorizontalAlignmentValues,
}
/// Matrix Column Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MCPr/m:mcPr")]
pub struct MatrixColumnProperties {
  /// Matrix Column Count
  #[sdk(child(qname = "m:CT_Integer64/m:count"))]
  pub matrix_column_count: Option<MatrixColumnCount>,
  /// Matrix Column Justification
  #[sdk(child(qname = "m:CT_XAlign/m:mcJc"))]
  pub matrix_column_justification: Option<MatrixColumnJustification>,
}
/// Matrix Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MC/m:mc")]
pub struct MatrixColumn {
  /// Matrix Column Properties
  #[sdk(child(qname = "m:CT_MCPr/m:mcPr"))]
  pub matrix_column_properties: Option<std::boxed::Box<MatrixColumnProperties>>,
}
/// Matrix Column Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:cSp")]
pub struct ColumnSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Left Margin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:lMargin")]
pub struct LeftMargin {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Right Margin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:rMargin")]
pub struct RightMargin {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Pre-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:preSp")]
pub struct PreSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Post-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:postSp")]
pub struct PostSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Inter-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:interSp")]
pub struct InterSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Intra-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:intraSp")]
pub struct IntraSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Wrap Indent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_TwipsMeasure/m:wrapIndent")]
pub struct WrapIndent {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Matrix Columns.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MCS/m:mcs")]
pub struct MatrixColumns {
  /// Matrix Column.
  #[sdk(child(qname = "m:CT_MC/m:mc"))]
  pub m_mc: Vec<MatrixColumn>,
}
/// Matrix Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MPr/m:mPr")]
pub struct MatrixProperties {
  /// Matrix Base Justification
  #[sdk(child(qname = "m:CT_YAlign/m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  /// Hide Placeholders (Matrix)
  #[sdk(child(qname = "m:CT_OnOff/m:plcHide"))]
  pub hide_placeholder: Option<HidePlaceholder>,
  /// Row Spacing Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  /// Matrix Column Gap Rule
  #[sdk(child(qname = "m:CT_SpacingRule/m:cGpRule"))]
  pub column_gap_rule: Option<ColumnGapRule>,
  /// Row Spacing (Matrix)
  #[sdk(child(qname = "m:CT_UnSignedShort/m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  /// Matrix Column Spacing
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:cSp"))]
  pub column_spacing: Option<ColumnSpacing>,
  /// Matrix Column Gap
  #[sdk(child(qname = "m:CT_UnSignedShort/m:cGp"))]
  pub column_gap: Option<ColumnGap>,
  /// Matrix Columns
  #[sdk(child(qname = "m:CT_MCS/m:mcs"))]
  pub matrix_columns: Option<MatrixColumns>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_MR/m:mr")]
pub struct MatrixRow {
  /// Base.
  #[sdk(child(qname = "m:CT_OMathArg/m:e"))]
  pub m_e: Vec<Base>,
}
/// n-ary Limit Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:limLoc")]
pub struct LimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// Integral Limit Locations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:intLim")]
pub struct IntegralLimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Limit Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_LimLoc/m:naryLim")]
pub struct NaryLimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_NaryPr/m:naryPr")]
pub struct NaryProperties {
  /// n-ary Operator Character
  #[sdk(child(qname = "m:CT_Char/m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// n-ary Limit Location
  #[sdk(child(qname = "m:CT_LimLoc/m:limLoc"))]
  pub limit_location: Option<LimitLocation>,
  /// n-ary Grow
  #[sdk(child(qname = "m:CT_OnOff/m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  /// Hide Subscript (n-ary)
  #[sdk(child(qname = "m:CT_OnOff/m:subHide"))]
  pub hide_sub_argument: Option<HideSubArgument>,
  /// Hide Superscript (n-ary)
  #[sdk(child(qname = "m:CT_OnOff/m:supHide"))]
  pub hide_super_argument: Option<HideSuperArgument>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Phantom Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_PhantPr/m:phantPr")]
pub struct PhantomProperties {
  /// Phantom Show
  #[sdk(child(qname = "m:CT_OnOff/m:show"))]
  pub show_phantom: Option<ShowPhantom>,
  /// Phantom Zero Width
  #[sdk(child(qname = "m:CT_OnOff/m:zeroWid"))]
  pub zero_width: Option<ZeroWidth>,
  /// Phantom Zero Ascent
  #[sdk(child(qname = "m:CT_OnOff/m:zeroAsc"))]
  pub zero_ascent: Option<ZeroAscent>,
  /// Phantom Zero Descent
  #[sdk(child(qname = "m:CT_OnOff/m:zeroDesc"))]
  pub zero_descent: Option<ZeroDescent>,
  /// Transparent (Phantom)
  #[sdk(child(qname = "m:CT_OnOff/m:transp"))]
  pub transparent: Option<Transparent>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Radical Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_RadPr/m:radPr")]
pub struct RadicalProperties {
  /// Hide Degree
  #[sdk(child(qname = "m:CT_OnOff/m:degHide"))]
  pub hide_degree: Option<HideDegree>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Pre-Sub-Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SPrePr/m:sPrePr")]
pub struct PreSubSuperProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Subscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubPr/m:sSubPr")]
pub struct SubscriptProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Sub-Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSubSupPr/m:sSubSupPr")]
pub struct SubSuperscriptProperties {
  /// Align Scripts
  #[sdk(child(qname = "m:CT_OnOff/m:alnScr"))]
  pub align_scripts: Option<AlignScripts>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_SSupPr/m:sSupPr")]
pub struct SuperscriptProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Argument Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_Integer2/m:argSz")]
pub struct ArgumentSize {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = -2..= 2))]
  pub val: crate::simple_type::IntegerValue,
}
/// Argument Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArgPr/m:argPr")]
pub struct ArgumentProperties {
  /// Argument Size
  #[sdk(child(qname = "m:CT_Integer2/m:argSz"))]
  pub argument_size: Option<ArgumentSize>,
}
/// Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathJc/m:jc")]
pub struct Justification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Default Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathJc/m:defJc")]
pub struct DefaultJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Math Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_FontFace/m:mathFont")]
pub struct MathFont {
  /// val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 31u32))]
  pub val: crate::simple_type::StringValue,
}
/// Break on Binary Operators.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BreakBin/m:brkBin")]
pub struct BreakBinary {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BreakBinaryOperatorValues>,
}
/// Break on Binary Subtraction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_BreakBinSub/m:brkBinSub")]
pub struct BreakBinarySubtraction {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: BreakBinarySubtractionValues,
}
/// Office Math Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathParaPr/m:oMathParaPr")]
pub struct ParagraphProperties {
  /// Justification
  #[sdk(child(qname = "m:CT_OMathJc/m:jc"))]
  pub justification: Option<Justification>,
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunSequence {
  /// Run Properties
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Option<std::boxed::Box<crate::schemas::w::RunProperties>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunChoice {
  /// Break.
  #[sdk(child(qname = "w:CT_Br/w:br"))]
  Br(std::boxed::Box<crate::schemas::w::Break>),
  /// Text.
  #[sdk(child(qname = "w:CT_Text/w:t"))]
  T(std::boxed::Box<crate::schemas::w::Text>),
  /// Deleted Text.
  #[sdk(child(qname = "w:CT_Text/w:delText"))]
  DelText(std::boxed::Box<crate::schemas::w::DeletedText>),
  /// Field Code.
  #[sdk(child(qname = "w:CT_Text/w:instrText"))]
  InstrText(std::boxed::Box<crate::schemas::w::FieldCode>),
  /// Deleted Field Code.
  #[sdk(child(qname = "w:CT_Text/w:delInstrText"))]
  DelInstrText(std::boxed::Box<crate::schemas::w::DeletedFieldCode>),
  /// Non Breaking Hyphen Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:noBreakHyphen"))]
  NoBreakHyphen,
  /// Optional Hyphen Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:softHyphen"))]
  SoftHyphen,
  /// Date Block - Short Day Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:dayShort"))]
  DayShort,
  /// Date Block - Short Month Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:monthShort"))]
  MonthShort,
  /// Date Block - Short Year Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:yearShort"))]
  YearShort,
  /// Date Block - Long Day Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:dayLong"))]
  DayLong,
  /// Date Block - Long Month Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:monthLong"))]
  MonthLong,
  /// Date Block - Long Year Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:yearLong"))]
  YearLong,
  /// Comment Information Block.
  #[sdk(empty_child(qname = "w:CT_Empty/w:annotationRef"))]
  AnnotationRef,
  /// Footnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:footnoteRef"))]
  FootnoteRef,
  /// Endnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:endnoteRef"))]
  EndnoteRef,
  /// Footnote/Endnote Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:separator"))]
  Separator,
  /// Continuation Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:continuationSeparator"))]
  ContinuationSeparator,
  /// Symbol Character.
  #[sdk(child(qname = "w:CT_Sym/w:sym"))]
  Sym(std::boxed::Box<crate::schemas::w::SymbolChar>),
  /// Page Number Block.
  #[sdk(empty_child(qname = "w:CT_Empty/w:pgNum"))]
  PgNum,
  /// Carriage Return.
  #[sdk(empty_child(qname = "w:CT_Empty/w:cr"))]
  Cr,
  /// Tab Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:tab"))]
  Tab,
  /// Inline Embedded Object.
  #[sdk(child(qname = "w:CT_Object/w:object"))]
  Object(std::boxed::Box<crate::schemas::w::EmbeddedObject>),
  /// VML Object.
  #[sdk(child(qname = "w:CT_Picture/w:pict"))]
  Pict(std::boxed::Box<crate::schemas::w::Picture>),
  /// Complex Field Character.
  #[sdk(child(qname = "w:CT_FldChar/w:fldChar"))]
  FldChar(std::boxed::Box<crate::schemas::w::FieldChar>),
  /// Phonetic Guide.
  #[sdk(child(qname = "w:CT_Ruby/w:ruby"))]
  Ruby(std::boxed::Box<crate::schemas::w::Ruby>),
  /// Footnote Reference.
  #[sdk(child(qname = "w:CT_FtnEdnRef/w:footnoteReference"))]
  FootnoteReference(std::boxed::Box<crate::schemas::w::FootnoteReference>),
  /// Endnote Reference.
  #[sdk(child(qname = "w:CT_FtnEdnRef/w:endnoteReference"))]
  EndnoteReference(std::boxed::Box<crate::schemas::w::EndnoteReference>),
  /// Comment Content Reference Mark.
  #[sdk(child(qname = "w:CT_Markup/w:commentReference"))]
  CommentReference(std::boxed::Box<crate::schemas::w::CommentReference>),
  /// DrawingML Object.
  #[sdk(child(qname = "w:CT_Drawing/w:drawing"))]
  Drawing(std::boxed::Box<crate::schemas::w::Drawing>),
  /// Absolute Position Tab Character.
  #[sdk(child(qname = "w:CT_PTab/w:ptab"))]
  Ptab(std::boxed::Box<crate::schemas::w::PositionalTab>),
  /// Position of Last Calculated Page Break.
  #[sdk(empty_child(qname = "w:CT_Empty/w:lastRenderedPageBreak"))]
  LastRenderedPageBreak,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunChoice2 {
  #[sdk(choice)]
  Choice(std::boxed::Box<RunChoice>),
  /// Text.
  #[sdk(child(qname = "m:CT_Text/m:t"))]
  T(std::boxed::Box<Text>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<str>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice2 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice3 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice4 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice5 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<ParagraphChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<ParagraphChoice4>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct ParagraphSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice6 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<ParagraphChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<ParagraphChoice5>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<ParagraphSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice7 {
  /// Phonetic Guide Text Run.
  #[sdk(child(qname = "w:CT_R/w:r"))]
  R(std::boxed::Box<crate::schemas::w::Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<ParagraphChoice>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<ParagraphChoice6>),
  #[sdk(choice)]
  Choice3(std::boxed::Box<ParagraphChoice7>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice9 {
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<ParagraphChoice8>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<str>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice2 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  WProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  WPermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  WPermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  WBookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  WBookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  WCommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  WCommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  WMoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  WMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  WMoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  WMoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  WCustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  WCustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  WCustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  WCustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  WCustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  WCustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  WCustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  WCustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  W14CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  W14CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  W14CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  W14CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  WIns(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  WDel(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  WMoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  WMoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  WContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Sequence of w14:conflictIns, w14:conflictDel
  #[sdk(sequence)]
  Sequence {
    /// Defines the RunConflictInsertion Class.
    #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
    run_conflict_insertion: Option<std::boxed::Box<crate::schemas::w14::RunConflictInsertion>>,
    /// Defines the RunConflictDeletion Class.
    #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
    run_conflict_deletion: Option<std::boxed::Box<crate::schemas::w14::RunConflictDeletion>>,
  },
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  WCustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  WFldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  WHyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  WSdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Choice of choice1, choice2, w:ins, w:del, w:moveFrom, w:moveTo, w:contentPart, sequence8
  #[sdk(choice)]
  Choice(std::boxed::Box<OfficeMathChoice2>),
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  MOMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  MOMath(std::boxed::Box<OfficeMath>),
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  MAcc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  MBar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  MBox(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  MBorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  MD(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  MEqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  MF(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  MFunc(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  MGroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  MLimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  MLimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  MM(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  MNary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  MPhant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  MRad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  MSPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  MSSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  MSSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  MSSup(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MR(std::boxed::Box<Run>),
  /// Choice of choice1, choice2
  #[sdk(choice)]
  Choice(std::boxed::Box<OfficeMathChoice3>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MathPropertiesChoice {
  /// Wrap Indent.
  #[sdk(child(qname = "m:CT_TwipsMeasure/m:wrapIndent"))]
  WrapIndent(std::boxed::Box<WrapIndent>),
  /// Wrap Right.
  #[sdk(child(qname = "m:CT_OnOff/m:wrapRight"))]
  WrapRight(std::boxed::Box<WrapRight>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunPropertiesSequence {
  /// Script.
  #[sdk(child(qname = "m:CT_Script/m:scr"))]
  pub m_scr: Option<Script>,
  /// style.
  #[sdk(child(qname = "m:CT_Style/m:sty"))]
  pub m_sty: Option<Style>,
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunPropertiesSequence2 {
  /// _
  #[sdk(sequence(qname = "m:CT_Script/m:scr", qname = "m:CT_Style/m:sty"))]
  pub sequence: Option<std::boxed::Box<RunPropertiesSequence>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice {
  /// Normal Text.
  #[sdk(child(qname = "m:CT_OnOff/m:nor"))]
  Nor(std::boxed::Box<NormalText>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<RunPropertiesSequence2>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ControlPropertiesChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  Sequence(std::boxed::Box<crate::schemas::w::RunProperties>),
  /// Defines the InsertedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlIns/w:ins"))]
  WIns(std::boxed::Box<crate::schemas::w::InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlDel/w:del"))]
  WDel(std::boxed::Box<crate::schemas::w::DeletedMathControl>),
  /// Defines the MoveFromMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlMove/w:moveFrom"))]
  WMoveFrom(std::boxed::Box<crate::schemas::w::MoveFromMathControl>),
  /// Defines the MoveToMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlMove/w:moveTo"))]
  WMoveTo(std::boxed::Box<crate::schemas::w::MoveToMathControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<BaseChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<BaseChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<BaseChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct BaseSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<BaseChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<BaseChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<BaseSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<BaseChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<BaseChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<BaseChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<BaseChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<BaseChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<BaseChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<BaseChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<BaseChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<BaseChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<NumeratorChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<NumeratorChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<NumeratorChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct NumeratorSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<NumeratorChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<NumeratorChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<NumeratorSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<NumeratorChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<NumeratorChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<NumeratorChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<NumeratorChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<NumeratorChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<NumeratorChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<NumeratorChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<NumeratorChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<NumeratorChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DenominatorChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DenominatorChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DenominatorChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DenominatorSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DenominatorChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DenominatorChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DenominatorSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DenominatorChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DenominatorChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DenominatorChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DenominatorChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DenominatorChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DenominatorChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DenominatorChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DenominatorChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DenominatorChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<FunctionNameChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<FunctionNameChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<FunctionNameChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct FunctionNameSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<FunctionNameChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<FunctionNameChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<FunctionNameSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<FunctionNameChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<FunctionNameChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<FunctionNameChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<FunctionNameChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<FunctionNameChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<FunctionNameChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<FunctionNameChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<FunctionNameChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<FunctionNameChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<LimitChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<LimitChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<LimitChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct LimitSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<LimitChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<LimitChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<LimitSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<LimitChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<LimitChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<LimitChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<LimitChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<LimitChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<LimitChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<LimitChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<LimitChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<LimitChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SubArgumentChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SubArgumentChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SubArgumentChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct SubArgumentSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SubArgumentChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SubArgumentChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<SubArgumentSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SubArgumentChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SubArgumentChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SubArgumentChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SubArgumentChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SubArgumentChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SubArgumentChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SubArgumentChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SubArgumentChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SubArgumentChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SuperArgumentChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SuperArgumentChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SuperArgumentChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct SuperArgumentSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SuperArgumentChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SuperArgumentChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<SuperArgumentSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SuperArgumentChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SuperArgumentChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SuperArgumentChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SuperArgumentChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<SuperArgumentChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SuperArgumentChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SuperArgumentChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<SuperArgumentChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<SuperArgumentChoice14>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice2 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DegreeChoice>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice3 {
  /// Defines the CustomXmlRun Class.
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
  CustomXml(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
  FldSimple(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice4 {
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  ProofErr(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice5 {
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice6 {
  /// Defines the MoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice7 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DegreeChoice5>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DegreeChoice6>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
  ))]
  CustomXmlConflictInsRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
  CustomXmlConflictInsRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  #[sdk(child(
    office2010,
    qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
  ))]
  CustomXmlConflictDelRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
  CustomXmlConflictDelRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DegreeSequence {
  /// Defines the RunConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
  pub w14_conflict_ins: Option<crate::schemas::w14::RunConflictInsertion>,
  /// Defines the RunConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
  pub w14_conflict_del: Option<crate::schemas::w14::RunConflictDeletion>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice8 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DegreeChoice4>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DegreeChoice7>),
  /// Inserted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
  Ins(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
  Del(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
  MoveFrom(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
  MoveTo(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DegreeSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice9 {
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  R(std::boxed::Box<Run>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice10 {
  /// Accent.
  #[sdk(child(qname = "m:CT_Acc/m:acc"))]
  Acc(std::boxed::Box<Accent>),
  /// Bar.
  #[sdk(child(qname = "m:CT_Bar/m:bar"))]
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  #[sdk(child(qname = "m:CT_Box/m:box"))]
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  #[sdk(child(qname = "m:CT_D/m:d"))]
  D(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
  EqArr(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  #[sdk(child(qname = "m:CT_F/m:f"))]
  F(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  #[sdk(child(qname = "m:CT_Func/m:func"))]
  Func(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
  GroupChr(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
  LimLow(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
  LimUpp(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  #[sdk(child(qname = "m:CT_M/m:m"))]
  M(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  #[sdk(child(qname = "m:CT_Nary/m:nary"))]
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  #[sdk(child(qname = "m:CT_Phant/m:phant"))]
  Phant(std::boxed::Box<Phantom>),
  /// Radical Function.
  #[sdk(child(qname = "m:CT_Rad/m:rad"))]
  Rad(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
  SPre(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
  SSub(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
  SSubSup(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
  SSup(std::boxed::Box<Superscript>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DegreeChoice9>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice11 {
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
  OMathPara(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
  OMath(std::boxed::Box<OfficeMath>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DegreeChoice10>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice12 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DegreeChoice8>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DegreeChoice11>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice13 {
  /// Defines the SdtRun Class.
  #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
  Sdt(std::boxed::Box<crate::schemas::w::SdtRun>),
  #[sdk(choice)]
  Choice(std::boxed::Box<DegreeChoice12>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice14 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DegreeChoice3>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DegreeChoice13>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice15 {
  #[sdk(choice)]
  Choice1(std::boxed::Box<DegreeChoice2>),
  #[sdk(choice)]
  Choice2(std::boxed::Box<DegreeChoice14>),
}
