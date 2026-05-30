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
  /// Run Properties
  #[sdk(child(qname = "m:CT_RPR/m:rPr"))]
  pub math_run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Run Properties
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Option<std::boxed::Box<crate::schemas::w::RunProperties>>,
  #[sdk(
        choice(
            child(variant = Break, qname = "w:br"),
            child(variant = WText, qname = "w:t"),
            child(variant = DeletedText, qname = "w:delText"),
            child(variant = FieldCode, qname = "w:instrText"),
            child(variant = DeletedFieldCode, qname = "w:delInstrText"),
            empty_child(variant = NoBreakHyphen, qname = "w:noBreakHyphen"),
            empty_child(variant = SoftHyphen, qname = "w:softHyphen"),
            empty_child(variant = DayShort, qname = "w:dayShort"),
            empty_child(variant = MonthShort, qname = "w:monthShort"),
            empty_child(variant = YearShort, qname = "w:yearShort"),
            empty_child(variant = DayLong, qname = "w:dayLong"),
            empty_child(variant = MonthLong, qname = "w:monthLong"),
            empty_child(variant = YearLong, qname = "w:yearLong"),
            empty_child(variant = AnnotationReferenceMark, qname = "w:annotationRef"),
            empty_child(variant = FootnoteReferenceMark, qname = "w:footnoteRef"),
            empty_child(variant = EndnoteReferenceMark, qname = "w:endnoteRef"),
            empty_child(variant = SeparatorMark, qname = "w:separator"),
            empty_child(
                variant = ContinuationSeparatorMark,
                qname = "w:continuationSeparator"
            ),
            child(variant = SymbolChar, qname = "w:sym"),
            empty_child(variant = PageNumber, qname = "w:pgNum"),
            empty_child(variant = CarriageReturn, qname = "w:cr"),
            empty_child(variant = TabChar, qname = "w:tab"),
            child(variant = EmbeddedObject, qname = "w:object"),
            child(variant = Picture, qname = "w:pict"),
            child(variant = FieldChar, qname = "w:fldChar"),
            child(variant = Ruby, qname = "w:ruby"),
            child(variant = FootnoteReference, qname = "w:footnoteReference"),
            child(variant = EndnoteReference, qname = "w:endnoteReference"),
            child(variant = CommentReference, qname = "w:commentReference"),
            child(variant = Drawing, qname = "w:drawing"),
            child(variant = PositionalTab, qname = "w:ptab"),
            empty_child(
                variant = LastRenderedPageBreak,
                qname = "w:lastRenderedPageBreak"
            ),
            child(variant = MText, qname = "m:t"),
            any
        )
    )]
  pub run_choice: Vec<RunChoice>,
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
  pub base: Vec<Base>,
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
  pub base: Vec<Base>,
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
  pub matrix_row: Vec<MatrixRow>,
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
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Office Math Paragraph Properties
  #[sdk(child(qname = "m:CT_OMathParaPr/m:oMathParaPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(
        choice(
            child(variant = OfficeMath, qname = "m:oMath"),
            child(variant = MRun, qname = "m:r"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = WRun, qname = "w:r"),
            any
        )
    )]
  pub paragraph_choice: Vec<ParagraphChoice>,
}
/// Defines the OfficeMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMath/m:oMath")]
pub struct OfficeMath {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
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
  #[sdk(
        choice(
            child(variant = WrapIndent, qname = "m:wrapIndent"),
            child(variant = WrapRight, qname = "m:wrapRight")
        )
    )]
  pub math_properties_choice: Option<MathPropertiesChoice>,
  /// Integral Limit Locations.
  #[sdk(child(qname = "m:CT_LimLoc/m:intLim"))]
  pub integral_limit_location: Option<IntegralLimitLocation>,
  /// n-ary Limit Location.
  #[sdk(child(qname = "m:CT_LimLoc/m:naryLim"))]
  pub nary_limit_location: Option<NaryLimitLocation>,
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
  #[sdk(
        choice(
            child(variant = NormalText, qname = "m:nor"),
            sequence(variant = Sequence, child(qname = "m:scr"), child(qname = "m:sty"))
        )
    )]
  pub run_properties_choice: Option<RunPropertiesChoice>,
  /// Break.
  #[sdk(child(qname = "m:CT_ManualBreak/m:brk"))]
  pub r#break: Option<Break>,
  /// Align.
  #[sdk(child(qname = "m:CT_OnOff/m:aln"))]
  pub alignment: Option<Alignment>,
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
  #[sdk(
        choice(
            child(variant = RunProperties, qname = "w:rPr"),
            child(variant = InsertedMathControl, qname = "w:ins"),
            child(variant = DeletedMathControl, qname = "w:del"),
            child(variant = MoveFromMathControl, qname = "w:moveFrom"),
            child(variant = MoveToMathControl, qname = "w:moveTo"),
            child(variant = DrawingRunProperties, qname = "a:rPr")
        )
    )]
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
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub base_choice: Vec<BaseChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Numerator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:num")]
pub struct Numerator {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub numerator_choice: Vec<NumeratorChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Denominator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:den")]
pub struct Denominator {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub denominator_choice: Vec<DenominatorChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:fName")]
pub struct FunctionName {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub function_name_choice: Vec<FunctionNameChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Limit (Lower).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:lim")]
pub struct Limit {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub limit_choice: Vec<LimitChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower limit (n-ary) .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sub")]
pub struct SubArgument {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub sub_argument_choice: Vec<SubArgumentChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper limit (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:sup")]
pub struct SuperArgument {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub super_argument_choice: Vec<SuperArgumentChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Degree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:CT_OMathArg/m:deg")]
pub struct Degree {
  /// Argument Properties
  #[sdk(child(qname = "m:CT_OMathArgPr/m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, qname = "m:acc"),
            child(variant = Bar, qname = "m:bar"),
            child(variant = Box, qname = "m:box"),
            child(variant = BorderBox, qname = "m:borderBox"),
            child(variant = Delimiter, qname = "m:d"),
            child(variant = EquationArray, qname = "m:eqArr"),
            child(variant = Fraction, qname = "m:f"),
            child(variant = MathFunction, qname = "m:func"),
            child(variant = GroupChar, qname = "m:groupChr"),
            child(variant = LimitLower, qname = "m:limLow"),
            child(variant = LimitUpper, qname = "m:limUpp"),
            child(variant = Matrix, qname = "m:m"),
            child(variant = Nary, qname = "m:nary"),
            child(variant = Phantom, qname = "m:phant"),
            child(variant = Radical, qname = "m:rad"),
            child(variant = PreSubSuper, qname = "m:sPre"),
            child(variant = Subscript, qname = "m:sSub"),
            child(variant = SubSuperscript, qname = "m:sSubSup"),
            child(variant = Superscript, qname = "m:sSup"),
            child(variant = Run, qname = "m:r"),
            child(variant = CustomXmlRun, qname = "w:customXml"),
            child(variant = SimpleField, qname = "w:fldSimple"),
            child(variant = Hyperlink, qname = "w:hyperlink"),
            child(variant = SdtRun, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, qname = "w:moveToRangeStart"),
            child(variant = MoveToRangeEnd, qname = "w:moveToRangeEnd"),
            child(variant = CustomXmlInsRangeStart, qname = "w:customXmlInsRangeStart"),
            child(variant = CustomXmlInsRangeEnd, qname = "w:customXmlInsRangeEnd"),
            child(variant = CustomXmlDelRangeStart, qname = "w:customXmlDelRangeStart"),
            child(variant = CustomXmlDelRangeEnd, qname = "w:customXmlDelRangeEnd"),
            child(
                variant = CustomXmlMoveFromRangeStart,
                qname = "w:customXmlMoveFromRangeStart"
            ),
            child(
                variant = CustomXmlMoveFromRangeEnd,
                qname = "w:customXmlMoveFromRangeEnd"
            ),
            child(
                variant = CustomXmlMoveToRangeStart,
                qname = "w:customXmlMoveToRangeStart"
            ),
            child(
                variant = CustomXmlMoveToRangeEnd,
                qname = "w:customXmlMoveToRangeEnd"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeStart,
                qname = "w14:customXmlConflictInsRangeStart"
            ),
            child(
                variant = CustomXmlConflictInsertionRangeEnd,
                qname = "w14:customXmlConflictInsRangeEnd"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeStart,
                qname = "w14:customXmlConflictDelRangeStart"
            ),
            child(
                variant = CustomXmlConflictDeletionRangeEnd,
                qname = "w14:customXmlConflictDelRangeEnd"
            ),
            child(variant = InsertedRun, qname = "w:ins"),
            child(variant = DeletedRun, qname = "w:del"),
            child(variant = MoveFromRun, qname = "w:moveFrom"),
            child(variant = MoveToRun, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, qname = "w14:conflictDel"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub degree_choice: Vec<DegreeChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:CT_CtrlPr/m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
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
  pub matrix_column: Vec<MatrixColumn>,
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
  pub base: Vec<Base>,
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunChoice {
  /// Break.
  Break(std::boxed::Box<crate::schemas::w::Break>),
  /// Text.
  #[sdk(child(qname = "w:CT_Text/w:t"))]
  WText(std::boxed::Box<crate::schemas::w::Text>),
  /// Deleted Text.
  DeletedText(std::boxed::Box<crate::schemas::w::DeletedText>),
  /// Field Code.
  FieldCode(std::boxed::Box<crate::schemas::w::FieldCode>),
  /// Deleted Field Code.
  DeletedFieldCode(std::boxed::Box<crate::schemas::w::DeletedFieldCode>),
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
  AnnotationReferenceMark,
  /// Footnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:footnoteRef"))]
  FootnoteReferenceMark,
  /// Endnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:endnoteRef"))]
  EndnoteReferenceMark,
  /// Footnote/Endnote Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:separator"))]
  SeparatorMark,
  /// Continuation Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:continuationSeparator"))]
  ContinuationSeparatorMark,
  /// Symbol Character.
  SymbolChar(std::boxed::Box<crate::schemas::w::SymbolChar>),
  /// Page Number Block.
  #[sdk(empty_child(qname = "w:CT_Empty/w:pgNum"))]
  PageNumber,
  /// Carriage Return.
  #[sdk(empty_child(qname = "w:CT_Empty/w:cr"))]
  CarriageReturn,
  /// Tab Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:tab"))]
  TabChar,
  /// Inline Embedded Object.
  EmbeddedObject(std::boxed::Box<crate::schemas::w::EmbeddedObject>),
  /// VML Object.
  Picture(std::boxed::Box<crate::schemas::w::Picture>),
  /// Complex Field Character.
  FieldChar(std::boxed::Box<crate::schemas::w::FieldChar>),
  /// Phonetic Guide.
  Ruby(std::boxed::Box<crate::schemas::w::Ruby>),
  /// Footnote Reference.
  FootnoteReference(std::boxed::Box<crate::schemas::w::FootnoteReference>),
  /// Endnote Reference.
  EndnoteReference(std::boxed::Box<crate::schemas::w::EndnoteReference>),
  /// Comment Content Reference Mark.
  CommentReference(std::boxed::Box<crate::schemas::w::CommentReference>),
  /// DrawingML Object.
  Drawing(std::boxed::Box<crate::schemas::w::Drawing>),
  /// Absolute Position Tab Character.
  PositionalTab(std::boxed::Box<crate::schemas::w::PositionalTab>),
  /// Position of Last Calculated Page Break.
  #[sdk(empty_child(qname = "w:CT_Empty/w:lastRenderedPageBreak"))]
  LastRenderedPageBreak,
  /// Text.
  #[sdk(child(qname = "m:CT_Text/m:t"))]
  MText(std::boxed::Box<Text>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice {
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
  /// Defines the Run Class.
  #[sdk(child(qname = "m:CT_R/m:r"))]
  MRun(std::boxed::Box<Run>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Phonetic Guide Text Run.
  #[sdk(child(qname = "w:CT_R/w:r"))]
  WRun(std::boxed::Box<crate::schemas::w::Run>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeMathChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MathPropertiesChoice {
  /// Wrap Indent.
  WrapIndent(std::boxed::Box<WrapIndent>),
  /// Wrap Right.
  WrapRight(std::boxed::Box<WrapRight>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunPropertiesChoiceSequence {
  /// Script.
  #[sdk(child(qname = "m:CT_Script/m:scr"))]
  pub script: Option<Script>,
  /// style.
  #[sdk(child(qname = "m:CT_Style/m:sty"))]
  pub style: Option<Style>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice {
  /// Normal Text.
  NormalText(std::boxed::Box<NormalText>),
  /// Sequence of m:scr, m:sty
  #[sdk(sequence)]
  Sequence(std::boxed::Box<RunPropertiesChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ControlPropertiesChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  RunProperties(std::boxed::Box<crate::schemas::w::RunProperties>),
  /// Defines the InsertedMathControl Class.
  InsertedMathControl(std::boxed::Box<crate::schemas::w::InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  DeletedMathControl(std::boxed::Box<crate::schemas::w::DeletedMathControl>),
  /// Defines the MoveFromMathControl Class.
  MoveFromMathControl(std::boxed::Box<crate::schemas::w::MoveFromMathControl>),
  /// Defines the MoveToMathControl Class.
  MoveToMathControl(std::boxed::Box<crate::schemas::w::MoveToMathControl>),
  DrawingRunProperties(std::boxed::Box<crate::schemas::a::RunProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BaseChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumeratorChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DenominatorChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FunctionNameChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LimitChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubArgumentChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SuperArgumentChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DegreeChoice {
  /// Accent.
  Accent(std::boxed::Box<Accent>),
  /// Bar.
  Bar(std::boxed::Box<Bar>),
  /// Box Function.
  Box(std::boxed::Box<Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<Superscript>),
  /// Defines the Run Class.
  Run(std::boxed::Box<Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<crate::schemas::w::CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<crate::schemas::w::SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<crate::schemas::w::Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<crate::schemas::w::SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<crate::schemas::w::ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<crate::schemas::w::PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<crate::schemas::w::BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<crate::schemas::w::BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<crate::schemas::w::CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<crate::schemas::w::CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<crate::schemas::w::MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<crate::schemas::w::MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<crate::schemas::w::CustomXmlMoveToRangeEnd>),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<crate::schemas::w::ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<OfficeMath>),
}
