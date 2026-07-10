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
#[sdk(qname = "m:scr")]
pub struct Script {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: ScriptValues,
}
/// style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sty")]
pub struct Style {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: StyleValues,
}
/// Defines the Run Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:r")]
pub struct Run {
  /// Run Properties
  #[sdk(child(qname = "m:rPr"))]
  pub math_run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Run Properties
  #[sdk(child(qname = "w:rPr"))]
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
            child(variant = EmbeddedObject, boxed, qname = "w:object"),
            child(variant = Picture, boxed, qname = "w:pict"),
            child(variant = FieldChar, boxed, qname = "w:fldChar"),
            child(variant = Ruby, boxed, qname = "w:ruby"),
            child(variant = FootnoteReference, qname = "w:footnoteReference"),
            child(variant = EndnoteReference, qname = "w:endnoteReference"),
            child(variant = CommentReference, qname = "w:commentReference"),
            child(variant = Drawing, boxed, qname = "w:drawing"),
            child(variant = PositionalTab, qname = "w:ptab"),
            empty_child(
                variant = LastRenderedPageBreak,
                qname = "w:lastRenderedPageBreak"
            ),
            child(variant = MText, qname = "m:t"),
            child(variant = DrawingRunProperties, boxed, qname = "a:rPr"),
            any
        )
    )]
  pub run_choice: Vec<RunChoice>,
}
/// Accent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:acc")]
pub struct Accent {
  /// Accent Properties
  #[sdk(child(qname = "m:accPr"))]
  pub accent_properties: Option<std::boxed::Box<AccentProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Bar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:bar")]
pub struct Bar {
  /// Bar Properties
  #[sdk(child(qname = "m:barPr"))]
  pub bar_properties: Option<std::boxed::Box<BarProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Box Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:box")]
pub struct Box {
  /// Box Properties
  #[sdk(child(qname = "m:boxPr"))]
  pub box_properties: Option<std::boxed::Box<BoxProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Border-Box Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:borderBox")]
pub struct BorderBox {
  /// Border Box Properties
  #[sdk(child(qname = "m:borderBoxPr"))]
  pub border_box_properties: Option<std::boxed::Box<BorderBoxProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Delimiter Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:d")]
pub struct Delimiter {
  /// Delimiter Properties
  #[sdk(child(qname = "m:dPr"))]
  pub delimiter_properties: Option<std::boxed::Box<DelimiterProperties>>,
  /// Base.
  #[sdk(child(qname = "m:e"))]
  pub base: Vec<Base>,
}
/// Equation-Array Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:eqArr")]
pub struct EquationArray {
  /// Equation Array Properties
  #[sdk(child(qname = "m:eqArrPr"))]
  pub equation_array_properties: Option<std::boxed::Box<EquationArrayProperties>>,
  /// Base.
  #[sdk(child(qname = "m:e"))]
  pub base: Vec<Base>,
}
/// Fraction Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:f")]
pub struct Fraction {
  /// Fraction Properties
  #[sdk(child(qname = "m:fPr"))]
  pub fraction_properties: Option<std::boxed::Box<FractionProperties>>,
  /// Numerator
  #[sdk(child(qname = "m:num"))]
  pub numerator: std::boxed::Box<Numerator>,
  /// Denominator
  #[sdk(child(qname = "m:den"))]
  pub denominator: std::boxed::Box<Denominator>,
}
/// Function Apply Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:func")]
pub struct MathFunction {
  /// Function Properties
  #[sdk(child(qname = "m:funcPr"))]
  pub function_properties: Option<std::boxed::Box<FunctionProperties>>,
  /// Function Name
  #[sdk(child(qname = "m:fName"))]
  pub function_name: std::boxed::Box<FunctionName>,
  /// Base (Argument)
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Group-Character Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:groupChr")]
pub struct GroupChar {
  /// Group-Character Properties
  #[sdk(child(qname = "m:groupChrPr"))]
  pub group_char_properties: Option<std::boxed::Box<GroupCharProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Lower-Limit Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:limLow")]
pub struct LimitLower {
  /// Lower Limit Properties
  #[sdk(child(qname = "m:limLowPr"))]
  pub limit_lower_properties: Option<std::boxed::Box<LimitLowerProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Limit (Lower)
  #[sdk(child(qname = "m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Upper-Limit Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:limUpp")]
pub struct LimitUpper {
  /// Upper Limit Properties
  #[sdk(child(qname = "m:limUppPr"))]
  pub limit_upper_properties: Option<std::boxed::Box<LimitUpperProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Limit (Upper)
  #[sdk(child(qname = "m:lim"))]
  pub limit: std::boxed::Box<Limit>,
}
/// Matrix Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:m")]
pub struct Matrix {
  /// Matrix Properties
  #[sdk(child(qname = "m:mPr"))]
  pub matrix_properties: Option<std::boxed::Box<MatrixProperties>>,
  /// Matrix Row.
  #[sdk(child(qname = "m:mr"))]
  pub matrix_row: Vec<MatrixRow>,
}
/// n-ary Operator Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:nary")]
pub struct Nary {
  /// n-ary Properties
  #[sdk(child(qname = "m:naryPr"))]
  pub nary_properties: Option<std::boxed::Box<NaryProperties>>,
  /// Lower limit (n-ary)
  #[sdk(child(qname = "m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Upper limit (n-ary)
  #[sdk(child(qname = "m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  /// Base (Argument)
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Phantom Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:phant")]
pub struct Phantom {
  /// Phantom Properties
  #[sdk(child(qname = "m:phantPr"))]
  pub phantom_properties: Option<std::boxed::Box<PhantomProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Radical Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:rad")]
pub struct Radical {
  /// Radical Properties
  #[sdk(child(qname = "m:radPr"))]
  pub radical_properties: Option<std::boxed::Box<RadicalProperties>>,
  /// Degree
  #[sdk(child(qname = "m:deg"))]
  pub degree: std::boxed::Box<Degree>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Pre-Sub-Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sPre")]
pub struct PreSubSuper {
  /// Pre-Sub-Superscript Properties
  #[sdk(child(qname = "m:sPrePr"))]
  pub pre_sub_super_properties: Option<std::boxed::Box<PreSubSuperProperties>>,
  /// Subscript (Pre-Sub-Superscript)
  #[sdk(child(qname = "m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Superscript(Pre-Sub-Superscript function)
  #[sdk(child(qname = "m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
}
/// Subscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSub")]
pub struct Subscript {
  /// Subscript Properties
  #[sdk(child(qname = "m:sSubPr"))]
  pub subscript_properties: Option<std::boxed::Box<SubscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Subscript (Subscript function)
  #[sdk(child(qname = "m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
}
/// Sub-Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSubSup")]
pub struct SubSuperscript {
  /// Sub-Superscript Properties
  #[sdk(child(qname = "m:sSubSupPr"))]
  pub sub_superscript_properties: Option<std::boxed::Box<SubSuperscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Subscript (Sub-Superscript)
  #[sdk(child(qname = "m:sub"))]
  pub sub_argument: std::boxed::Box<SubArgument>,
  /// Superscript (Sub-Superscript function)
  #[sdk(child(qname = "m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Superscript Function.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSup")]
pub struct Superscript {
  /// Superscript Properties
  #[sdk(child(qname = "m:sSupPr"))]
  pub superscript_properties: Option<std::boxed::Box<SuperscriptProperties>>,
  /// Base
  #[sdk(child(qname = "m:e"))]
  pub base: std::boxed::Box<Base>,
  /// Superscript (Superscript function)
  #[sdk(child(qname = "m:sup"))]
  pub super_argument: std::boxed::Box<SuperArgument>,
}
/// Defines the Paragraph Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:oMathPara")]
pub struct Paragraph {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Office Math Paragraph Properties
  #[sdk(child(qname = "m:oMathParaPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(
        choice(
            child(variant = OfficeMath, qname = "m:oMath"),
            child(variant = MRun, boxed, qname = "m:r"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = WRun, boxed, qname = "w:r"),
            any
        )
    )]
  pub paragraph_choice: Vec<ParagraphChoice>,
}
/// Defines the OfficeMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:oMath")]
pub struct OfficeMath {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub office_math_choice: Vec<OfficeMathChoice>,
}
/// Math Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mathPr")]
pub struct MathProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Math Font
  #[sdk(child(qname = "m:mathFont"))]
  pub math_font: Option<MathFont>,
  /// Break on Binary Operators
  #[sdk(child(qname = "m:brkBin"))]
  pub break_binary: Option<BreakBinary>,
  /// Break on Binary Subtraction
  #[sdk(child(qname = "m:brkBinSub"))]
  pub break_binary_subtraction: Option<BreakBinarySubtraction>,
  /// Small Fraction
  #[sdk(child(qname = "m:smallFrac"))]
  pub small_fraction: Option<SmallFraction>,
  /// Use Display Math Defaults
  #[sdk(child(qname = "m:dispDef"))]
  pub display_defaults: Option<DisplayDefaults>,
  /// Left Margin
  #[sdk(child(qname = "m:lMargin"))]
  pub left_margin: Option<LeftMargin>,
  /// Right Margin
  #[sdk(child(qname = "m:rMargin"))]
  pub right_margin: Option<RightMargin>,
  /// Default Justification
  #[sdk(child(qname = "m:defJc"))]
  pub default_justification: Option<DefaultJustification>,
  /// Pre-Equation Spacing
  #[sdk(child(qname = "m:preSp"))]
  pub pre_spacing: Option<PreSpacing>,
  /// Post-Equation Spacing
  #[sdk(child(qname = "m:postSp"))]
  pub post_spacing: Option<PostSpacing>,
  /// Inter-Equation Spacing
  #[sdk(child(qname = "m:interSp"))]
  pub inter_spacing: Option<InterSpacing>,
  /// Intra-Equation Spacing
  #[sdk(child(qname = "m:intraSp"))]
  pub intra_spacing: Option<IntraSpacing>,
  #[sdk(
        choice(
            child(variant = WrapIndent, qname = "m:wrapIndent"),
            child(variant = WrapRight, qname = "m:wrapRight")
        )
    )]
  pub math_properties_choice: Option<MathPropertiesChoice>,
  /// Integral Limit Locations.
  #[sdk(child(qname = "m:intLim"))]
  pub integral_limit_location: Option<IntegralLimitLocation>,
  /// n-ary Limit Location.
  #[sdk(child(qname = "m:naryLim"))]
  pub nary_limit_location: Option<NaryLimitLocation>,
}
/// Literal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:lit")]
pub struct Literal {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Normal Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:nor")]
pub struct NormalText {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:aln")]
pub struct Alignment {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Operator Emulator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:opEmu")]
pub struct OperatorEmulator {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// No Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:noBreak")]
pub struct NoBreak {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Differential.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:diff")]
pub struct Differential {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Top Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:hideTop")]
pub struct HideTop {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Bottom Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:hideBot")]
pub struct HideBottom {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Left Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:hideLeft")]
pub struct HideLeft {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Right Edge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:hideRight")]
pub struct HideRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:strikeH")]
pub struct StrikeHorizontal {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:strikeV")]
pub struct StrikeVertical {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Bottom-Left to Top-Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:strikeBLTR")]
pub struct StrikeBottomLeftToTopRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Top-Left to Bottom-Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:strikeTLBR")]
pub struct StrikeTopLeftToBottomRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Delimiter Grow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:grow")]
pub struct GrowOperators {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Maximum Distribution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:maxDist")]
pub struct MaxDistribution {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Object Distribution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:objDist")]
pub struct ObjectDistribution {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Placeholders (Matrix).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:plcHide")]
pub struct HidePlaceholder {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Subscript (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:subHide")]
pub struct HideSubArgument {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Superscript (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:supHide")]
pub struct HideSuperArgument {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:show")]
pub struct ShowPhantom {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:zeroWid")]
pub struct ZeroWidth {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Ascent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:zeroAsc")]
pub struct ZeroAscent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Phantom Zero Descent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:zeroDesc")]
pub struct ZeroDescent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Transparent (Phantom).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:transp")]
pub struct Transparent {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Hide Degree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:degHide")]
pub struct HideDegree {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Align Scripts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:alnScr")]
pub struct AlignScripts {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Small Fraction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:smallFrac")]
pub struct SmallFraction {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Use Display Math Defaults.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:dispDef")]
pub struct DisplayDefaults {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Wrap Right.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:wrapRight")]
pub struct WrapRight {
  /// value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BooleanValues>,
}
/// Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:brk")]
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
#[sdk(qname = "m:rPr")]
pub struct RunProperties {
  /// Literal
  #[sdk(child(qname = "m:lit"))]
  pub literal: Option<Literal>,
  #[sdk(
        choice(
            child(variant = NormalText, qname = "m:nor"),
            sequence(
                variant = Sequence,
                child(option_field = script, qname = "m:scr"),
                child(option_field = style, qname = "m:sty")
            )
        )
    )]
  pub run_properties_choice: Option<RunPropertiesChoice>,
  /// Break.
  #[sdk(child(qname = "m:brk"))]
  pub r#break: Option<Break>,
  /// Align.
  #[sdk(child(qname = "m:aln"))]
  pub alignment: Option<Alignment>,
}
/// Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:t")]
pub struct Text {
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Accent Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:chr")]
pub struct AccentChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Beginning Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:begChr")]
pub struct BeginChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Separator Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sepChr")]
pub struct SeparatorChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Delimiter Ending Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:endChr")]
pub struct EndChar {
  /// value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Control Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:ctrlPr")]
pub struct ControlProperties {
  #[sdk(
        choice(
            child(variant = RunProperties, boxed, qname = "w:rPr"),
            child(variant = InsertedMathControl, boxed, qname = "w:ins"),
            child(variant = DeletedMathControl, boxed, qname = "w:del"),
            child(variant = MoveFromMathControl, boxed, qname = "w:moveFrom"),
            child(variant = MoveToMathControl, boxed, qname = "w:moveTo"),
            child(variant = DrawingRunProperties, boxed, qname = "a:rPr")
        )
    )]
  pub control_properties_choice: Option<ControlPropertiesChoice>,
}
/// Accent Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:accPr")]
pub struct AccentProperties {
  /// Accent Character
  #[sdk(child(qname = "m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// Control Properties
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Base.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:e")]
pub struct Base {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub base_choice: Vec<BaseChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Numerator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:num")]
pub struct Numerator {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub numerator_choice: Vec<NumeratorChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Denominator.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:den")]
pub struct Denominator {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub denominator_choice: Vec<DenominatorChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:fName")]
pub struct FunctionName {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub function_name_choice: Vec<FunctionNameChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Limit (Lower).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:lim")]
pub struct Limit {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub limit_choice: Vec<LimitChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower limit (n-ary) .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sub")]
pub struct SubArgument {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub sub_argument_choice: Vec<SubArgumentChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper limit (n-ary).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sup")]
pub struct SuperArgument {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub super_argument_choice: Vec<SuperArgumentChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Degree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:deg")]
pub struct Degree {
  /// Argument Properties
  #[sdk(child(qname = "m:argPr"))]
  pub argument_properties: Option<std::boxed::Box<ArgumentProperties>>,
  #[sdk(
        choice(
            child(variant = Accent, boxed, qname = "m:acc"),
            child(variant = Bar, boxed, qname = "m:bar"),
            child(variant = Box, boxed, qname = "m:box"),
            child(variant = BorderBox, boxed, qname = "m:borderBox"),
            child(variant = Delimiter, boxed, qname = "m:d"),
            child(variant = EquationArray, boxed, qname = "m:eqArr"),
            child(variant = Fraction, boxed, qname = "m:f"),
            child(variant = MathFunction, boxed, qname = "m:func"),
            child(variant = GroupChar, boxed, qname = "m:groupChr"),
            child(variant = LimitLower, boxed, qname = "m:limLow"),
            child(variant = LimitUpper, boxed, qname = "m:limUpp"),
            child(variant = Matrix, boxed, qname = "m:m"),
            child(variant = Nary, boxed, qname = "m:nary"),
            child(variant = Phantom, boxed, qname = "m:phant"),
            child(variant = Radical, boxed, qname = "m:rad"),
            child(variant = PreSubSuper, boxed, qname = "m:sPre"),
            child(variant = Subscript, boxed, qname = "m:sSub"),
            child(variant = SubSuperscript, boxed, qname = "m:sSubSup"),
            child(variant = Superscript, boxed, qname = "m:sSup"),
            child(variant = Run, boxed, qname = "m:r"),
            child(variant = CustomXmlRun, boxed, qname = "w:customXml"),
            child(variant = SimpleField, boxed, qname = "w:fldSimple"),
            child(variant = Hyperlink, boxed, qname = "w:hyperlink"),
            child(variant = SdtRun, boxed, qname = "w:sdt"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, boxed, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd"),
            child(variant = MoveFromRangeStart, boxed, qname = "w:moveFromRangeStart"),
            child(variant = MoveFromRangeEnd, qname = "w:moveFromRangeEnd"),
            child(variant = MoveToRangeStart, boxed, qname = "w:moveToRangeStart"),
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
            child(variant = InsertedRun, boxed, qname = "w:ins"),
            child(variant = DeletedRun, boxed, qname = "w:del"),
            child(variant = MoveFromRun, boxed, qname = "w:moveFrom"),
            child(variant = MoveToRun, boxed, qname = "w:moveTo"),
            child(variant = ContentPart, qname = "w:contentPart"),
            child(variant = RunConflictInsertion, boxed, qname = "w14:conflictIns"),
            child(variant = RunConflictDeletion, boxed, qname = "w14:conflictDel"),
            child(variant = Paragraph, boxed, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath")
        )
    )]
  pub degree_choice: Vec<DegreeChoice>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Position (Bar).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:pos")]
pub struct Position {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Vertical Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:vertJc")]
pub struct VerticalJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalJustificationValues,
}
/// Bar Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:barPr")]
pub struct BarProperties {
  /// Position (Bar)
  #[sdk(child(qname = "m:pos"))]
  pub position: Option<Position>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Box Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:boxPr")]
pub struct BoxProperties {
  /// Operator Emulator
  #[sdk(child(qname = "m:opEmu"))]
  pub operator_emulator: Option<OperatorEmulator>,
  /// No Break
  #[sdk(child(qname = "m:noBreak"))]
  pub no_break: Option<NoBreak>,
  /// Differential
  #[sdk(child(qname = "m:diff"))]
  pub differential: Option<Differential>,
  /// Break
  #[sdk(child(qname = "m:brk"))]
  pub r#break: Option<Break>,
  /// Alignment
  #[sdk(child(qname = "m:aln"))]
  pub alignment: Option<Alignment>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Border Box Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:borderBoxPr")]
pub struct BorderBoxProperties {
  /// Hide Top Edge
  #[sdk(child(qname = "m:hideTop"))]
  pub hide_top: Option<HideTop>,
  /// Hide Bottom Edge
  #[sdk(child(qname = "m:hideBot"))]
  pub hide_bottom: Option<HideBottom>,
  /// Hide Left Edge
  #[sdk(child(qname = "m:hideLeft"))]
  pub hide_left: Option<HideLeft>,
  /// Hide Right Edge
  #[sdk(child(qname = "m:hideRight"))]
  pub hide_right: Option<HideRight>,
  /// Border Box Strikethrough Horizontal
  #[sdk(child(qname = "m:strikeH"))]
  pub strike_horizontal: Option<StrikeHorizontal>,
  /// Border Box Strikethrough Vertical
  #[sdk(child(qname = "m:strikeV"))]
  pub strike_vertical: Option<StrikeVertical>,
  /// Border Box Strikethrough Bottom-Left to Top-Right
  #[sdk(child(qname = "m:strikeBLTR"))]
  pub strike_bottom_left_to_top_right: Option<StrikeBottomLeftToTopRight>,
  /// Border Box Strikethrough Top-Left to Bottom-Right
  #[sdk(child(qname = "m:strikeTLBR"))]
  pub strike_top_left_to_bottom_right: Option<StrikeTopLeftToBottomRight>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Shape (Delimiters).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:shp")]
pub struct Shape {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: ShapeDelimiterValues,
}
/// Delimiter Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:dPr")]
pub struct DelimiterProperties {
  /// Delimiter Beginning Character
  #[sdk(child(qname = "m:begChr"))]
  pub begin_char: Option<BeginChar>,
  /// Delimiter Separator Character
  #[sdk(child(qname = "m:sepChr"))]
  pub separator_char: Option<SeparatorChar>,
  /// Delimiter Ending Character
  #[sdk(child(qname = "m:endChr"))]
  pub end_char: Option<EndChar>,
  /// Delimiter Grow
  #[sdk(child(qname = "m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  /// Shape (Delimiters)
  #[sdk(child(qname = "m:shp"))]
  pub shape: Option<Shape>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Equation Array Base Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:baseJc")]
pub struct BaseJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: VerticalAlignmentValues,
}
/// Row Spacing Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:rSpRule")]
pub struct RowSpacingRule {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 0..= 4))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Gap Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:cGpRule")]
pub struct ColumnGapRule {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 0..= 4))]
  pub val: crate::simple_type::IntegerValue,
}
/// Row Spacing (Equation Array).
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:rSp")]
pub struct RowSpacing {
  /// val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Matrix Column Gap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:cGp")]
pub struct ColumnGap {
  /// val
  #[sdk(attr(qname = "m:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Equation Array Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:eqArrPr")]
pub struct EquationArrayProperties {
  /// Equation Array Base Justification
  #[sdk(child(qname = "m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  /// Maximum Distribution
  #[sdk(child(qname = "m:maxDist"))]
  pub max_distribution: Option<MaxDistribution>,
  /// Object Distribution
  #[sdk(child(qname = "m:objDist"))]
  pub object_distribution: Option<ObjectDistribution>,
  /// Row Spacing Rule
  #[sdk(child(qname = "m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  /// Row Spacing (Equation Array)
  #[sdk(child(qname = "m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Fraction type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:type")]
pub struct FractionType {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: FractionTypeValues,
}
/// Fraction Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:fPr")]
pub struct FractionProperties {
  /// Fraction type
  #[sdk(child(qname = "m:type"))]
  pub fraction_type: Option<FractionType>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Function Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:funcPr")]
pub struct FunctionProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Group-Character Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:groupChrPr")]
pub struct GroupCharProperties {
  /// Group Character (Grouping Character)
  #[sdk(child(qname = "m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// Position (Group Character)
  #[sdk(child(qname = "m:pos"))]
  pub position: Option<Position>,
  /// Vertical Justification
  #[sdk(child(qname = "m:vertJc"))]
  pub vertical_justification: Option<VerticalJustification>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Lower Limit Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:limLowPr")]
pub struct LimitLowerProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Upper Limit Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:limUppPr")]
pub struct LimitUpperProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Column Count.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:count")]
pub struct MatrixColumnCount {
  /// val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = 1..= 64))]
  pub val: crate::simple_type::IntegerValue,
}
/// Matrix Column Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mcJc")]
pub struct MatrixColumnJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: HorizontalAlignmentValues,
}
/// Matrix Column Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mcPr")]
pub struct MatrixColumnProperties {
  /// Matrix Column Count
  #[sdk(child(qname = "m:count"))]
  pub matrix_column_count: Option<MatrixColumnCount>,
  /// Matrix Column Justification
  #[sdk(child(qname = "m:mcJc"))]
  pub matrix_column_justification: Option<MatrixColumnJustification>,
}
/// Matrix Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mc")]
pub struct MatrixColumn {
  /// Matrix Column Properties
  #[sdk(child(qname = "m:mcPr"))]
  pub matrix_column_properties: Option<std::boxed::Box<MatrixColumnProperties>>,
}
/// Matrix Column Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:cSp")]
pub struct ColumnSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Left Margin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:lMargin")]
pub struct LeftMargin {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Right Margin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:rMargin")]
pub struct RightMargin {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Pre-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:preSp")]
pub struct PreSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Post-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:postSp")]
pub struct PostSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Inter-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:interSp")]
pub struct InterSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Intra-Equation Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:intraSp")]
pub struct IntraSpacing {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Wrap Indent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:wrapIndent")]
pub struct WrapIndent {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Matrix Columns.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mcs")]
pub struct MatrixColumns {
  /// Matrix Column.
  #[sdk(child(qname = "m:mc"))]
  pub matrix_column: Vec<MatrixColumn>,
}
/// Matrix Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mPr")]
pub struct MatrixProperties {
  /// Matrix Base Justification
  #[sdk(child(qname = "m:baseJc"))]
  pub base_justification: Option<BaseJustification>,
  /// Hide Placeholders (Matrix)
  #[sdk(child(qname = "m:plcHide"))]
  pub hide_placeholder: Option<HidePlaceholder>,
  /// Row Spacing Rule
  #[sdk(child(qname = "m:rSpRule"))]
  pub row_spacing_rule: Option<RowSpacingRule>,
  /// Matrix Column Gap Rule
  #[sdk(child(qname = "m:cGpRule"))]
  pub column_gap_rule: Option<ColumnGapRule>,
  /// Row Spacing (Matrix)
  #[sdk(child(qname = "m:rSp"))]
  pub row_spacing: Option<RowSpacing>,
  /// Matrix Column Spacing
  #[sdk(child(qname = "m:cSp"))]
  pub column_spacing: Option<ColumnSpacing>,
  /// Matrix Column Gap
  #[sdk(child(qname = "m:cGp"))]
  pub column_gap: Option<ColumnGap>,
  /// Matrix Columns
  #[sdk(child(qname = "m:mcs"))]
  pub matrix_columns: Option<MatrixColumns>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Matrix Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mr")]
pub struct MatrixRow {
  /// Base.
  #[sdk(child(qname = "m:e"))]
  pub base: Vec<Base>,
}
/// n-ary Limit Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:limLoc")]
pub struct LimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// Integral Limit Locations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:intLim")]
pub struct IntegralLimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Limit Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:naryLim")]
pub struct NaryLimitLocation {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: LimitLocationValues,
}
/// n-ary Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:naryPr")]
pub struct NaryProperties {
  /// n-ary Operator Character
  #[sdk(child(qname = "m:chr"))]
  pub accent_char: Option<AccentChar>,
  /// n-ary Limit Location
  #[sdk(child(qname = "m:limLoc"))]
  pub limit_location: Option<LimitLocation>,
  /// n-ary Grow
  #[sdk(child(qname = "m:grow"))]
  pub grow_operators: Option<GrowOperators>,
  /// Hide Subscript (n-ary)
  #[sdk(child(qname = "m:subHide"))]
  pub hide_sub_argument: Option<HideSubArgument>,
  /// Hide Superscript (n-ary)
  #[sdk(child(qname = "m:supHide"))]
  pub hide_super_argument: Option<HideSuperArgument>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Phantom Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:phantPr")]
pub struct PhantomProperties {
  /// Phantom Show
  #[sdk(child(qname = "m:show"))]
  pub show_phantom: Option<ShowPhantom>,
  /// Phantom Zero Width
  #[sdk(child(qname = "m:zeroWid"))]
  pub zero_width: Option<ZeroWidth>,
  /// Phantom Zero Ascent
  #[sdk(child(qname = "m:zeroAsc"))]
  pub zero_ascent: Option<ZeroAscent>,
  /// Phantom Zero Descent
  #[sdk(child(qname = "m:zeroDesc"))]
  pub zero_descent: Option<ZeroDescent>,
  /// Transparent (Phantom)
  #[sdk(child(qname = "m:transp"))]
  pub transparent: Option<Transparent>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Radical Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:radPr")]
pub struct RadicalProperties {
  /// Hide Degree
  #[sdk(child(qname = "m:degHide"))]
  pub hide_degree: Option<HideDegree>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Pre-Sub-Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sPrePr")]
pub struct PreSubSuperProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Subscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSubPr")]
pub struct SubscriptProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Sub-Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSubSupPr")]
pub struct SubSuperscriptProperties {
  /// Align Scripts
  #[sdk(child(qname = "m:alnScr"))]
  pub align_scripts: Option<AlignScripts>,
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Superscript Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:sSupPr")]
pub struct SuperscriptProperties {
  /// Control Properties.
  #[sdk(child(qname = "m:ctrlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Argument Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:argSz")]
pub struct ArgumentSize {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  #[sdk(number_range(range = -2..= 2))]
  pub val: crate::simple_type::IntegerValue,
}
/// Argument Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:argPr")]
pub struct ArgumentProperties {
  /// Argument Size
  #[sdk(child(qname = "m:argSz"))]
  pub argument_size: Option<ArgumentSize>,
}
/// Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:jc")]
pub struct Justification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Default Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:defJc")]
pub struct DefaultJustification {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: JustificationValues,
}
/// Math Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:mathFont")]
pub struct MathFont {
  /// val
  #[sdk(attr(qname = "m:val"))]
  #[sdk(string_length(max = 31u32))]
  pub val: crate::simple_type::StringValue,
}
/// Break on Binary Operators.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:brkBin")]
pub struct BreakBinary {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: Option<BreakBinaryOperatorValues>,
}
/// Break on Binary Subtraction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:brkBinSub")]
pub struct BreakBinarySubtraction {
  /// Value
  #[sdk(attr(qname = "m:val"))]
  pub val: BreakBinarySubtractionValues,
}
/// Office Math Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "m:oMathParaPr")]
pub struct ParagraphProperties {
  /// Justification
  #[sdk(child(qname = "m:jc"))]
  pub justification: Option<Justification>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunChoice {
  /// Break.
  Break(crate::schemas::w::Break),
  /// Text.
  WText(crate::schemas::w::Text),
  /// Deleted Text.
  DeletedText(crate::schemas::w::DeletedText),
  /// Field Code.
  FieldCode(crate::schemas::w::FieldCode),
  /// Deleted Field Code.
  DeletedFieldCode(crate::schemas::w::DeletedFieldCode),
  /// Non Breaking Hyphen Character.
  NoBreakHyphen,
  /// Optional Hyphen Character.
  SoftHyphen,
  /// Date Block - Short Day Format.
  DayShort,
  /// Date Block - Short Month Format.
  MonthShort,
  /// Date Block - Short Year Format.
  YearShort,
  /// Date Block - Long Day Format.
  DayLong,
  /// Date Block - Long Month Format.
  MonthLong,
  /// Date Block - Long Year Format.
  YearLong,
  /// Comment Information Block.
  AnnotationReferenceMark,
  /// Footnote Reference Mark.
  FootnoteReferenceMark,
  /// Endnote Reference Mark.
  EndnoteReferenceMark,
  /// Footnote/Endnote Separator Mark.
  SeparatorMark,
  /// Continuation Separator Mark.
  ContinuationSeparatorMark,
  /// Symbol Character.
  SymbolChar(crate::schemas::w::SymbolChar),
  /// Page Number Block.
  PageNumber,
  /// Carriage Return.
  CarriageReturn,
  /// Tab Character.
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
  FootnoteReference(crate::schemas::w::FootnoteReference),
  /// Endnote Reference.
  EndnoteReference(crate::schemas::w::EndnoteReference),
  /// Comment Content Reference Mark.
  CommentReference(crate::schemas::w::CommentReference),
  /// DrawingML Object.
  Drawing(std::boxed::Box<crate::schemas::w::Drawing>),
  /// Absolute Position Tab Character.
  PositionalTab(crate::schemas::w::PositionalTab),
  /// Position of Last Calculated Page Break.
  LastRenderedPageBreak,
  /// Text.
  MText(Text),
  DrawingRunProperties(std::boxed::Box<crate::schemas::a::RunProperties>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphChoice {
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
  /// Defines the Run Class.
  MRun(std::boxed::Box<Run>),
  /// Defines the ProofError Class.
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<crate::schemas::w::Run>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MathPropertiesChoice {
  /// Wrap Indent.
  WrapIndent(WrapIndent),
  /// Wrap Right.
  WrapRight(WrapRight),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct RunPropertiesChoiceSequence {
  /// Script.
  #[sdk(child(qname = "m:scr"))]
  pub script: Option<Script>,
  /// style.
  #[sdk(child(qname = "m:sty"))]
  pub style: Option<Style>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice {
  /// Normal Text.
  NormalText(NormalText),
  /// Sequence of m:scr, m:sty
  Sequence(std::boxed::Box<RunPropertiesChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ControlPropertiesChoice {
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
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
#[derive(Clone, Debug, PartialEq)]
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
  ProofError(crate::schemas::w::ProofError),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<crate::schemas::w::PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(crate::schemas::w::PermEnd),
  /// Defines the BookmarkStart Class.
  BookmarkStart(crate::schemas::w::BookmarkStart),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(crate::schemas::w::BookmarkEnd),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(crate::schemas::w::CommentRangeStart),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(crate::schemas::w::CommentRangeEnd),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<crate::schemas::w::MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(crate::schemas::w::MoveFromRangeEnd),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<crate::schemas::w::MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(crate::schemas::w::MoveToRangeEnd),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(crate::schemas::w::CustomXmlInsRangeStart),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(crate::schemas::w::CustomXmlInsRangeEnd),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(crate::schemas::w::CustomXmlDelRangeStart),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(crate::schemas::w::CustomXmlDelRangeEnd),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(crate::schemas::w::CustomXmlMoveFromRangeStart),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(crate::schemas::w::CustomXmlMoveFromRangeEnd),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(crate::schemas::w::CustomXmlMoveToRangeStart),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(crate::schemas::w::CustomXmlMoveToRangeEnd),
  /// Defines the CustomXmlConflictInsertionRangeStart Class.
  CustomXmlConflictInsertionRangeStart(crate::schemas::w14::CustomXmlConflictInsertionRangeStart),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(crate::schemas::w14::CustomXmlConflictInsertionRangeEnd),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(crate::schemas::w14::CustomXmlConflictDeletionRangeStart),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(crate::schemas::w14::CustomXmlConflictDeletionRangeEnd),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<crate::schemas::w::InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<crate::schemas::w::DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<crate::schemas::w::MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<crate::schemas::w::MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(crate::schemas::w::ContentPart),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(OfficeMath),
}
