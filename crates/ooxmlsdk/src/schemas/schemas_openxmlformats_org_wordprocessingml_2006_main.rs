//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OnOffOnlyValues {
  #[sdk(rename = "on")]
  #[default]
  On,
  #[sdk(rename = "off")]
  Off,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HighlightColorValues {
  #[sdk(rename = "black")]
  #[default]
  Black,
  #[sdk(rename = "blue")]
  Blue,
  #[sdk(rename = "cyan")]
  Cyan,
  #[sdk(rename = "green")]
  Green,
  #[sdk(rename = "magenta")]
  Magenta,
  #[sdk(rename = "red")]
  Red,
  #[sdk(rename = "yellow")]
  Yellow,
  #[sdk(rename = "white")]
  White,
  #[sdk(rename = "darkBlue")]
  DarkBlue,
  #[sdk(rename = "darkCyan")]
  DarkCyan,
  #[sdk(rename = "darkGreen")]
  DarkGreen,
  #[sdk(rename = "darkMagenta")]
  DarkMagenta,
  #[sdk(rename = "darkRed")]
  DarkRed,
  #[sdk(rename = "darkYellow")]
  DarkYellow,
  #[sdk(rename = "darkGray")]
  DarkGray,
  #[sdk(rename = "lightGray")]
  LightGray,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AutomaticColorValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum UnderlineValues {
  #[sdk(rename = "single")]
  #[default]
  Single,
  #[sdk(rename = "words")]
  Words,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "thick")]
  Thick,
  #[sdk(rename = "dotted")]
  Dotted,
  #[sdk(rename = "dottedHeavy")]
  DottedHeavy,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "dashedHeavy")]
  DashedHeavy,
  #[sdk(rename = "dashLong")]
  DashLong,
  #[sdk(rename = "dashLongHeavy")]
  DashLongHeavy,
  #[sdk(rename = "dotDash")]
  DotDash,
  #[sdk(rename = "dashDotHeavy")]
  DashDotHeavy,
  #[sdk(rename = "dotDotDash")]
  DotDotDash,
  #[sdk(rename = "dashDotDotHeavy")]
  DashDotDotHeavy,
  #[sdk(rename = "wave")]
  Wave,
  #[sdk(rename = "wavyHeavy")]
  WavyHeavy,
  #[sdk(rename = "wavyDouble")]
  WavyDouble,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextEffectValues {
  #[sdk(rename = "blinkBackground")]
  #[default]
  BlinkBackground,
  #[sdk(rename = "lights")]
  Lights,
  #[sdk(rename = "antsBlack")]
  AntsBlack,
  #[sdk(rename = "antsRed")]
  AntsRed,
  #[sdk(rename = "shimmer")]
  Shimmer,
  #[sdk(rename = "sparkle")]
  Sparkle,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalPositionValues {
  #[sdk(rename = "baseline")]
  #[default]
  Baseline,
  #[sdk(rename = "superscript")]
  Superscript,
  #[sdk(rename = "subscript")]
  Subscript,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EmphasisMarkValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "comma")]
  Comma,
  #[sdk(rename = "circle")]
  Circle,
  #[sdk(rename = "underDot")]
  UnderDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CombineBracketValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "round")]
  Round,
  #[sdk(rename = "square")]
  Square,
  #[sdk(rename = "angle")]
  Angle,
  #[sdk(rename = "curly")]
  Curly,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "inside")]
  Inside,
  #[sdk(rename = "outside")]
  Outside,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentValues {
  #[sdk(rename = "inline")]
  #[default]
  Inline,
  #[sdk(rename = "top")]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "inside")]
  Inside,
  #[sdk(rename = "outside")]
  Outside,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HeightRuleValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "exact")]
  Exact,
  #[sdk(rename = "atLeast")]
  AtLeast,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextWrappingValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "notBeside")]
  NotBeside,
  #[sdk(rename = "around")]
  Around,
  #[sdk(rename = "tight")]
  Tight,
  #[sdk(rename = "through")]
  Through,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAnchorValues {
  #[sdk(rename = "text")]
  #[default]
  Text,
  #[sdk(rename = "margin")]
  Margin,
  #[sdk(rename = "page")]
  Page,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAnchorValues {
  #[sdk(rename = "text")]
  #[default]
  Text,
  #[sdk(rename = "margin")]
  Margin,
  #[sdk(rename = "page")]
  Page,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DropCapLocationValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "drop")]
  Drop,
  #[sdk(rename = "margin")]
  Margin,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TabStopLeaderCharValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "hyphen")]
  Hyphen,
  #[sdk(rename = "underscore")]
  Underscore,
  #[sdk(rename = "heavy")]
  Heavy,
  #[sdk(rename = "middleDot")]
  MiddleDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineSpacingRuleValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "exact")]
  Exact,
  #[sdk(rename = "atLeast")]
  AtLeast,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableRowAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "start")]
  Start,
  #[sdk(rename = "end")]
  End,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ViewValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "print")]
  Print,
  #[sdk(rename = "outline")]
  Outline,
  #[sdk(rename = "masterPages")]
  MasterPages,
  #[sdk(rename = "normal")]
  Normal,
  #[sdk(rename = "web")]
  Web,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetZoomValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "fullPage")]
  FullPage,
  #[sdk(rename = "bestFit")]
  BestFit,
  #[sdk(rename = "textFit")]
  TextFit,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ProofingStateValues {
  #[sdk(rename = "clean")]
  #[default]
  Clean,
  #[sdk(rename = "dirty")]
  Dirty,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocumentTypeValues {
  #[sdk(rename = "notSpecified")]
  #[default]
  NotSpecified,
  #[sdk(rename = "letter")]
  Letter,
  #[sdk(rename = "eMail")]
  Email,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocumentProtectionValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "readOnly")]
  ReadOnly,
  #[sdk(rename = "comments")]
  Comments,
  #[sdk(rename = "trackedChanges")]
  TrackedChanges,
  #[sdk(rename = "forms")]
  Forms,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MailMergeDocumentValues {
  #[sdk(rename = "catalog")]
  #[default]
  Catalog,
  #[sdk(rename = "envelopes")]
  Envelope,
  #[sdk(rename = "mailingLabels")]
  MailingLabel,
  #[sdk(rename = "formLetters")]
  FormLetter,
  #[sdk(rename = "email")]
  Email,
  #[sdk(rename = "fax")]
  Fax,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MailMergeDataValues {
  #[sdk(rename = "textFile")]
  #[default]
  TextFile,
  #[sdk(rename = "database")]
  Database,
  #[sdk(rename = "spreadsheet")]
  Spreadsheet,
  #[sdk(rename = "query")]
  Query,
  #[sdk(rename = "odbc")]
  Odbc,
  #[sdk(rename = "native")]
  Native,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MailMergeDestinationValues {
  #[sdk(rename = "newDocument")]
  #[default]
  NewDocument,
  #[sdk(rename = "printer")]
  Printer,
  #[sdk(rename = "email")]
  Email,
  #[sdk(rename = "fax")]
  Fax,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MailMergeOdsoFieldValues {
  #[sdk(rename = "null")]
  #[default]
  Null,
  #[sdk(rename = "dbColumn")]
  DbColumn,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalTextAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "baseline")]
  Baseline,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "auto")]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DisplacedByCustomXmlValues {
  #[sdk(rename = "next")]
  #[default]
  Next,
  #[sdk(rename = "prev")]
  Previous,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalMergeRevisionValues {
  #[sdk(rename = "cont")]
  #[default]
  Continue,
  #[sdk(rename = "rest")]
  Restart,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextBoxTightWrapValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "allLines")]
  AllLines,
  #[sdk(rename = "firstAndLastLine")]
  FirstAndLastLine,
  #[sdk(rename = "firstLineOnly")]
  FirstLineOnly,
  #[sdk(rename = "lastLineOnly")]
  LastLineOnly,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FieldCharValues {
  #[sdk(rename = "begin")]
  #[default]
  Begin,
  #[sdk(rename = "separate")]
  Separate,
  #[sdk(rename = "end")]
  End,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum InfoTextValues {
  #[sdk(rename = "text")]
  #[default]
  Text,
  #[sdk(rename = "autoText")]
  AutoText,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextBoxFormFieldValues {
  #[sdk(rename = "regular")]
  #[default]
  Regular,
  #[sdk(rename = "number")]
  Number,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "currentTime")]
  CurrentTime,
  #[sdk(rename = "currentDate")]
  CurrentDate,
  #[sdk(rename = "calculated")]
  Calculated,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SectionMarkValues {
  #[sdk(rename = "nextPage")]
  #[default]
  NextPage,
  #[sdk(rename = "nextColumn")]
  NextColumn,
  #[sdk(rename = "continuous")]
  Continuous,
  #[sdk(rename = "evenPage")]
  EvenPage,
  #[sdk(rename = "oddPage")]
  OddPage,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageOrientationValues {
  #[sdk(rename = "portrait")]
  #[default]
  Portrait,
  #[sdk(rename = "landscape")]
  Landscape,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageBorderZOrderValues {
  #[sdk(rename = "front")]
  #[default]
  Front,
  #[sdk(rename = "back")]
  Back,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageBorderDisplayValues {
  #[sdk(rename = "allPages")]
  #[default]
  AllPages,
  #[sdk(rename = "firstPage")]
  FirstPage,
  #[sdk(rename = "notFirstPage")]
  NotFirstPage,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageBorderOffsetValues {
  #[sdk(rename = "page")]
  #[default]
  Page,
  #[sdk(rename = "text")]
  Text,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChapterSeparatorValues {
  #[sdk(rename = "hyphen")]
  #[default]
  Hyphen,
  #[sdk(rename = "period")]
  Period,
  #[sdk(rename = "colon")]
  Colon,
  #[sdk(rename = "emDash")]
  EmDash,
  #[sdk(rename = "enDash")]
  EnDash,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineNumberRestartValues {
  #[sdk(rename = "newPage")]
  #[default]
  NewPage,
  #[sdk(rename = "newSection")]
  NewSection,
  #[sdk(rename = "continuous")]
  Continuous,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalJustificationValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "both")]
  Both,
  #[sdk(rename = "bottom")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableVerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocGridValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "lines")]
  Lines,
  #[sdk(rename = "linesAndChars")]
  LinesAndChars,
  #[sdk(rename = "snapToChars")]
  SnapToChars,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HeaderFooterValues {
  #[sdk(rename = "even")]
  #[default]
  Even,
  #[sdk(rename = "default", alias = "odd")]
  Default,
  #[sdk(rename = "first")]
  First,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FootnoteEndnoteValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "separator")]
  Separator,
  #[sdk(rename = "continuationSeparator")]
  ContinuationSeparator,
  #[sdk(rename = "continuationNotice")]
  ContinuationNotice,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BreakValues {
  #[sdk(rename = "page")]
  #[default]
  Page,
  #[sdk(rename = "column")]
  Column,
  #[sdk(rename = "textWrapping")]
  TextWrapping,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BreakTextRestartLocationValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "all")]
  All,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AbsolutePositionTabAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AbsolutePositionTabPositioningBaseValues {
  #[sdk(rename = "margin")]
  #[default]
  Margin,
  #[sdk(rename = "indent")]
  Indent,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AbsolutePositionTabLeaderCharValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "hyphen")]
  Hyphen,
  #[sdk(rename = "underscore")]
  Underscore,
  #[sdk(rename = "middleDot")]
  MiddleDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ProofingErrorValues {
  #[sdk(rename = "spellStart")]
  #[default]
  SpellStart,
  #[sdk(rename = "spellEnd")]
  SpellEnd,
  #[sdk(rename = "gramStart")]
  GrammarStart,
  #[sdk(rename = "gramEnd")]
  GrammarEnd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RangePermissionEditingGroupValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "everyone")]
  Everyone,
  #[sdk(rename = "administrators")]
  Administrators,
  #[sdk(rename = "contributors")]
  Contributors,
  #[sdk(rename = "editors")]
  Editors,
  #[sdk(rename = "owners")]
  Owners,
  #[sdk(rename = "current")]
  Current,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontTypeHintValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "eastAsia")]
  EastAsia,
  #[sdk(rename = "cs")]
  ComplexScript,
  #[sdk(other)]
  OtherVariant(Box<[u8]>),
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ThemeFontValues {
  #[sdk(rename = "majorEastAsia")]
  #[default]
  MajorEastAsia,
  #[sdk(rename = "majorBidi")]
  MajorBidi,
  #[sdk(rename = "majorAscii")]
  MajorAscii,
  #[sdk(rename = "majorHAnsi")]
  MajorHighAnsi,
  #[sdk(rename = "minorEastAsia")]
  MinorEastAsia,
  #[sdk(rename = "minorBidi")]
  MinorBidi,
  #[sdk(rename = "minorAscii")]
  MinorAscii,
  #[sdk(rename = "minorHAnsi")]
  MinorHighAnsi,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RubyAlignValues {
  #[sdk(rename = "center")]
  #[default]
  Center,
  #[sdk(rename = "distributeLetter")]
  DistributeLetter,
  #[sdk(rename = "distributeSpace")]
  DistributeSpace,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "rightVertical")]
  RightVertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LockingValues {
  #[sdk(rename = "sdtLocked")]
  #[default]
  SdtLocked,
  #[sdk(rename = "contentLocked")]
  ContentLocked,
  #[sdk(rename = "unlocked")]
  Unlocked,
  #[sdk(rename = "sdtContentLocked")]
  SdtContentLocked,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DateFormatValues {
  #[sdk(rename = "text")]
  #[default]
  Text,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "dateTime")]
  DateTime,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableWidthUnitValues {
  #[sdk(rename = "nil")]
  #[default]
  Nil,
  #[sdk(rename = "pct")]
  Pct,
  #[sdk(rename = "dxa")]
  Dxa,
  #[sdk(rename = "auto")]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableWidthValues {
  #[sdk(rename = "nil")]
  #[default]
  Nil,
  #[sdk(rename = "dxa")]
  Dxa,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MergedCellValues {
  #[sdk(rename = "continue")]
  #[default]
  Continue,
  #[sdk(rename = "restart")]
  Restart,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableLayoutValues {
  #[sdk(rename = "fixed")]
  #[default]
  Fixed,
  #[sdk(rename = "autofit")]
  Autofit,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableOverlapValues {
  #[sdk(rename = "never")]
  #[default]
  Never,
  #[sdk(rename = "overlap")]
  Overlap,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FootnotePositionValues {
  #[sdk(rename = "pageBottom")]
  #[default]
  PageBottom,
  #[sdk(rename = "beneathText")]
  BeneathText,
  #[sdk(rename = "sectEnd")]
  SectionEnd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EndnotePositionValues {
  #[sdk(rename = "sectEnd")]
  #[default]
  SectionEnd,
  #[sdk(rename = "docEnd")]
  DocumentEnd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RestartNumberValues {
  #[sdk(rename = "continuous")]
  #[default]
  Continuous,
  #[sdk(rename = "eachSect")]
  EachSection,
  #[sdk(rename = "eachPage")]
  EachPage,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MailMergeSourceValues {
  #[sdk(rename = "database")]
  #[default]
  Database,
  #[sdk(rename = "addressBook")]
  AddressBook,
  #[sdk(rename = "document1")]
  Document1,
  #[sdk(rename = "document2")]
  Document2,
  #[sdk(rename = "text")]
  Text,
  #[sdk(rename = "email")]
  Email,
  #[sdk(rename = "native")]
  Native,
  #[sdk(rename = "legacy")]
  Legacy,
  #[sdk(rename = "master")]
  Master,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TargetScreenSizeValues {
  #[sdk(rename = "544x376")]
  #[default]
  Sz544x376,
  #[sdk(rename = "640x480")]
  Sz640x480,
  #[sdk(rename = "720x512")]
  Sz720x512,
  #[sdk(rename = "800x600")]
  Sz800x600,
  #[sdk(rename = "1024x768")]
  Sz1024x768,
  #[sdk(rename = "1152x882")]
  Sz1152x882,
  #[sdk(rename = "1152x900")]
  Sz1152x900,
  #[sdk(rename = "1280x1024")]
  Sz1280x1024,
  #[sdk(rename = "1600x1200")]
  Sz1600x1200,
  #[sdk(rename = "1800x1440")]
  Sz1800x1440,
  #[sdk(rename = "1920x1200")]
  Sz1920x1200,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CharacterSpacingValues {
  #[sdk(rename = "doNotCompress")]
  #[default]
  DoNotCompress,
  #[sdk(rename = "compressPunctuation")]
  CompressPunctuation,
  #[sdk(rename = "compressPunctuationAndJapaneseKana")]
  CompressPunctuationAndJapaneseKana,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ColorSchemeIndexValues {
  #[sdk(rename = "dark1")]
  #[default]
  Dark1,
  #[sdk(rename = "light1")]
  Light1,
  #[sdk(rename = "dark2")]
  Dark2,
  #[sdk(rename = "light2")]
  Light2,
  #[sdk(rename = "accent1")]
  Accent1,
  #[sdk(rename = "accent2")]
  Accent2,
  #[sdk(rename = "accent3")]
  Accent3,
  #[sdk(rename = "accent4")]
  Accent4,
  #[sdk(rename = "accent5")]
  Accent5,
  #[sdk(rename = "accent6")]
  Accent6,
  #[sdk(rename = "hyperlink")]
  Hyperlink,
  #[sdk(rename = "followedHyperlink")]
  FollowedHyperlink,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FrameScrollbarVisibilityValues {
  #[sdk(rename = "on")]
  #[default]
  On,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "auto")]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FrameLayoutValues {
  #[sdk(rename = "rows")]
  #[default]
  Rows,
  #[sdk(rename = "cols")]
  Columns,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LevelSuffixValues {
  #[sdk(rename = "tab", alias = "Tab")]
  #[default]
  Tab,
  #[sdk(rename = "space")]
  Space,
  #[sdk(rename = "nothing")]
  Nothing,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MultiLevelValues {
  #[sdk(rename = "singleLevel", alias = "SingleLevel")]
  #[default]
  SingleLevel,
  #[sdk(rename = "multilevel")]
  Multilevel,
  #[sdk(rename = "hybridMultilevel")]
  HybridMultilevel,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableStyleOverrideValues {
  #[sdk(rename = "wholeTable")]
  #[default]
  WholeTable,
  #[sdk(rename = "firstRow")]
  FirstRow,
  #[sdk(rename = "lastRow")]
  LastRow,
  #[sdk(rename = "firstCol")]
  FirstColumn,
  #[sdk(rename = "lastCol")]
  LastColumn,
  #[sdk(rename = "band1Vert")]
  Band1Vertical,
  #[sdk(rename = "band2Vert")]
  Band2Vertical,
  #[sdk(rename = "band1Horz")]
  Band1Horizontal,
  #[sdk(rename = "band2Horz")]
  Band2Horizontal,
  #[sdk(rename = "neCell")]
  NorthEastCell,
  #[sdk(rename = "nwCell")]
  NorthWestCell,
  #[sdk(rename = "seCell")]
  SouthEastCell,
  #[sdk(rename = "swCell")]
  SouthWestCell,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleValues {
  #[sdk(rename = "paragraph")]
  #[default]
  Paragraph,
  #[sdk(rename = "character")]
  Character,
  #[sdk(rename = "table")]
  Table,
  #[sdk(rename = "numbering")]
  Numbering,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontFamilyValues {
  #[sdk(rename = "decorative")]
  #[default]
  Decorative,
  #[sdk(rename = "modern")]
  Modern,
  #[sdk(rename = "roman")]
  Roman,
  #[sdk(rename = "script")]
  Script,
  #[sdk(rename = "swiss")]
  Swiss,
  #[sdk(rename = "auto")]
  Auto,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontPitchValues {
  #[sdk(rename = "fixed")]
  #[default]
  Fixed,
  #[sdk(rename = "variable")]
  Variable,
  #[sdk(rename = "default")]
  Default,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ThemeColorValues {
  #[sdk(rename = "dark1")]
  #[default]
  Dark1,
  #[sdk(rename = "light1")]
  Light1,
  #[sdk(rename = "dark2")]
  Dark2,
  #[sdk(rename = "light2")]
  Light2,
  #[sdk(rename = "accent1")]
  Accent1,
  #[sdk(rename = "accent2")]
  Accent2,
  #[sdk(rename = "accent3")]
  Accent3,
  #[sdk(rename = "accent4")]
  Accent4,
  #[sdk(rename = "accent5")]
  Accent5,
  #[sdk(rename = "accent6")]
  Accent6,
  #[sdk(rename = "hyperlink")]
  Hyperlink,
  #[sdk(rename = "followedHyperlink")]
  FollowedHyperlink,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "background1")]
  Background1,
  #[sdk(rename = "text1")]
  Text1,
  #[sdk(rename = "background2")]
  Background2,
  #[sdk(rename = "text2")]
  Text2,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocPartBehaviorValues {
  #[sdk(rename = "content")]
  #[default]
  Content,
  #[sdk(rename = "p")]
  Paragraph,
  #[sdk(rename = "pg")]
  Page,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocPartValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "normal")]
  Normal,
  #[sdk(rename = "autoExp")]
  AutoExp,
  #[sdk(rename = "toolbar")]
  Toolbar,
  #[sdk(rename = "speller")]
  Speller,
  #[sdk(rename = "formFld")]
  FormField,
  #[sdk(rename = "bbPlcHdr")]
  SdtPlaceholder,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocPartGalleryValues {
  #[sdk(rename = "placeholder")]
  #[default]
  Placeholder,
  #[sdk(rename = "any")]
  Any,
  #[sdk(rename = "default")]
  Default,
  #[sdk(rename = "docParts")]
  DocumentPart,
  #[sdk(rename = "coverPg")]
  CoverPage,
  #[sdk(rename = "eq")]
  Equation,
  #[sdk(rename = "ftrs")]
  Footer,
  #[sdk(rename = "hdrs")]
  Header,
  #[sdk(rename = "pgNum")]
  PageNumber,
  #[sdk(rename = "tbls")]
  Table,
  #[sdk(rename = "watermarks")]
  WaterMark,
  #[sdk(rename = "autoTxt")]
  AutoText,
  #[sdk(rename = "txtBox")]
  TextBox,
  #[sdk(rename = "pgNumT")]
  PageNumberTop,
  #[sdk(rename = "pgNumB")]
  PageNumberBottom,
  #[sdk(rename = "pgNumMargins")]
  PageNumberMargins,
  #[sdk(rename = "tblOfContents")]
  TableOfContents,
  #[sdk(rename = "bib")]
  Bibliography,
  #[sdk(rename = "custQuickParts")]
  CustomQuickParts,
  #[sdk(rename = "custCoverPg")]
  CustomCoverPage,
  #[sdk(rename = "custEq")]
  CustomEquation,
  #[sdk(rename = "custFtrs")]
  CustomFooter,
  #[sdk(rename = "custHdrs")]
  CustomHeaders,
  #[sdk(rename = "custPgNum")]
  CustomPageNumber,
  #[sdk(rename = "custTbls")]
  CustomTable,
  #[sdk(rename = "custWatermarks")]
  CustomWatermark,
  #[sdk(rename = "custAutoTxt")]
  CustomAutoText,
  #[sdk(rename = "custTxtBox")]
  CustomTextBox,
  #[sdk(rename = "custPgNumT")]
  CustomPageNumberTop,
  #[sdk(rename = "custPgNumB")]
  CustomPageNumberBottom,
  #[sdk(rename = "custPgNumMargins")]
  CustomPageNumberMargin,
  #[sdk(rename = "custTblOfContents")]
  CustomTableOfContents,
  #[sdk(rename = "custBib")]
  CustomBibliography,
  #[sdk(rename = "custom1")]
  Custom1,
  #[sdk(rename = "custom2")]
  Custom2,
  #[sdk(rename = "custom3")]
  Custom3,
  #[sdk(rename = "custom4")]
  Custom4,
  #[sdk(rename = "custom5")]
  Custom5,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CaptionPositionValues {
  #[sdk(rename = "above")]
  #[default]
  Above,
  #[sdk(rename = "below")]
  Below,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LevelJustificationValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "start")]
  Start,
  #[sdk(rename = "end")]
  End,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShadingPatternValues {
  #[sdk(rename = "nil")]
  #[default]
  Nil,
  #[sdk(rename = "clear")]
  Clear,
  #[sdk(rename = "solid")]
  Solid,
  #[sdk(rename = "horzStripe")]
  HorizontalStripe,
  #[sdk(rename = "vertStripe")]
  VerticalStripe,
  #[sdk(rename = "reverseDiagStripe")]
  ReverseDiagonalStripe,
  #[sdk(rename = "diagStripe")]
  DiagonalStripe,
  #[sdk(rename = "horzCross")]
  HorizontalCross,
  #[sdk(rename = "diagCross")]
  DiagonalCross,
  #[sdk(rename = "thinHorzStripe")]
  ThinHorizontalStripe,
  #[sdk(rename = "thinVertStripe")]
  ThinVerticalStripe,
  #[sdk(rename = "thinReverseDiagStripe")]
  ThinReverseDiagonalStripe,
  #[sdk(rename = "thinDiagStripe")]
  ThinDiagonalStripe,
  #[sdk(rename = "thinHorzCross")]
  ThinHorizontalCross,
  #[sdk(rename = "thinDiagCross")]
  ThinDiagonalCross,
  #[sdk(rename = "pct5")]
  Percent5,
  #[sdk(rename = "pct10")]
  Percent10,
  #[sdk(rename = "pct12")]
  Percent12,
  #[sdk(rename = "pct15")]
  Percent15,
  #[sdk(rename = "pct20")]
  Percent20,
  #[sdk(rename = "pct25")]
  Percent25,
  #[sdk(rename = "pct30")]
  Percent30,
  #[sdk(rename = "pct35")]
  Percent35,
  #[sdk(rename = "pct37")]
  Percent37,
  #[sdk(rename = "pct40")]
  Percent40,
  #[sdk(rename = "pct45")]
  Percent45,
  #[sdk(rename = "pct50")]
  Percent50,
  #[sdk(rename = "pct55")]
  Percent55,
  #[sdk(rename = "pct60")]
  Percent60,
  #[sdk(rename = "pct62")]
  Percent62,
  #[sdk(rename = "pct65")]
  Percent65,
  #[sdk(rename = "pct70")]
  Percent70,
  #[sdk(rename = "pct75")]
  Percent75,
  #[sdk(rename = "pct80")]
  Percent80,
  #[sdk(rename = "pct85")]
  Percent85,
  #[sdk(rename = "pct87")]
  Percent87,
  #[sdk(rename = "pct90")]
  Percent90,
  #[sdk(rename = "pct95")]
  Percent95,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StylePaneSortMethodsValues {
  #[sdk(rename = "0000")]
  #[default]
  Zero,
  #[sdk(rename = "name")]
  Name,
  #[sdk(rename = "0001")]
  One,
  #[sdk(rename = "priority")]
  Priority,
  #[sdk(rename = "0002")]
  Two,
  #[sdk(rename = "font")]
  Font,
  #[sdk(rename = "0003")]
  Three,
  #[sdk(rename = "basedOn")]
  BasedOn,
  #[sdk(rename = "0004")]
  Four,
  #[sdk(rename = "type")]
  Type,
  #[sdk(rename = "0005")]
  Five,
  #[sdk(rename = "default")]
  Default,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DirectionValues {
  #[sdk(rename = "ltr")]
  #[default]
  Ltr,
  #[sdk(rename = "rtl")]
  Rtl,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CalendarValues {
  #[sdk(rename = "gregorian")]
  #[default]
  Gregorian,
  #[sdk(rename = "hijri")]
  Hijri,
  #[sdk(rename = "umalqura")]
  Umalqura,
  #[sdk(rename = "hebrew")]
  Hebrew,
  #[sdk(rename = "taiwan")]
  Taiwan,
  #[sdk(rename = "japan")]
  Japan,
  #[sdk(rename = "thai")]
  Thai,
  #[sdk(rename = "korea")]
  Korea,
  #[sdk(rename = "saka")]
  Saka,
  #[sdk(rename = "gregorianXlitEnglish")]
  GregorianTransliteratedEnglish,
  #[sdk(rename = "gregorianXlitFrench")]
  GregorianTransliteratedFrench,
  #[sdk(rename = "gregorianUs")]
  GregorianUs,
  #[sdk(rename = "gregorianMeFrench")]
  GregorianMeFrench,
  #[sdk(rename = "gregorianArabic")]
  GregorianArabic,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NumberFormatValues {
  #[sdk(rename = "decimal")]
  #[default]
  Decimal,
  #[sdk(rename = "upperRoman")]
  UpperRoman,
  #[sdk(rename = "lowerRoman")]
  LowerRoman,
  #[sdk(rename = "upperLetter")]
  UpperLetter,
  #[sdk(rename = "lowerLetter")]
  LowerLetter,
  #[sdk(rename = "ordinal")]
  Ordinal,
  #[sdk(rename = "cardinalText")]
  CardinalText,
  #[sdk(rename = "ordinalText")]
  OrdinalText,
  #[sdk(rename = "hex")]
  Hex,
  #[sdk(rename = "chicago")]
  Chicago,
  #[sdk(rename = "ideographDigital")]
  IdeographDigital,
  #[sdk(rename = "japaneseCounting")]
  JapaneseCounting,
  #[sdk(rename = "aiueo")]
  Aiueo,
  #[sdk(rename = "iroha")]
  Iroha,
  #[sdk(rename = "decimalFullWidth")]
  DecimalFullWidth,
  #[sdk(rename = "decimalHalfWidth")]
  DecimalHalfWidth,
  #[sdk(rename = "japaneseLegal")]
  JapaneseLegal,
  #[sdk(rename = "japaneseDigitalTenThousand")]
  JapaneseDigitalTenThousand,
  #[sdk(rename = "decimalEnclosedCircle")]
  DecimalEnclosedCircle,
  #[sdk(rename = "decimalFullWidth2")]
  DecimalFullWidth2,
  #[sdk(rename = "aiueoFullWidth")]
  AiueoFullWidth,
  #[sdk(rename = "irohaFullWidth")]
  IrohaFullWidth,
  #[sdk(rename = "decimalZero")]
  DecimalZero,
  #[sdk(rename = "bullet")]
  Bullet,
  #[sdk(rename = "ganada")]
  Ganada,
  #[sdk(rename = "chosung")]
  Chosung,
  #[sdk(rename = "decimalEnclosedFullstop")]
  DecimalEnclosedFullstop,
  #[sdk(rename = "decimalEnclosedParen")]
  DecimalEnclosedParen,
  #[sdk(rename = "decimalEnclosedCircleChinese")]
  DecimalEnclosedCircleChinese,
  #[sdk(rename = "ideographEnclosedCircle")]
  IdeographEnclosedCircle,
  #[sdk(rename = "ideographTraditional")]
  IdeographTraditional,
  #[sdk(rename = "ideographZodiac")]
  IdeographZodiac,
  #[sdk(rename = "ideographZodiacTraditional")]
  IdeographZodiacTraditional,
  #[sdk(rename = "taiwaneseCounting")]
  TaiwaneseCounting,
  #[sdk(rename = "ideographLegalTraditional")]
  IdeographLegalTraditional,
  #[sdk(rename = "taiwaneseCountingThousand")]
  TaiwaneseCountingThousand,
  #[sdk(rename = "taiwaneseDigital")]
  TaiwaneseDigital,
  #[sdk(rename = "chineseCounting")]
  ChineseCounting,
  #[sdk(rename = "chineseLegalSimplified")]
  ChineseLegalSimplified,
  #[sdk(rename = "chineseCountingThousand")]
  ChineseCountingThousand,
  #[sdk(rename = "koreanDigital")]
  KoreanDigital,
  #[sdk(rename = "koreanCounting")]
  KoreanCounting,
  #[sdk(rename = "koreanLegal")]
  KoreanLegal,
  #[sdk(rename = "koreanDigital2")]
  KoreanDigital2,
  #[sdk(rename = "vietnameseCounting")]
  VietnameseCounting,
  #[sdk(rename = "russianLower")]
  RussianLower,
  #[sdk(rename = "russianUpper")]
  RussianUpper,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "numberInDash")]
  NumberInDash,
  #[sdk(rename = "hebrew1")]
  Hebrew1,
  #[sdk(rename = "hebrew2")]
  Hebrew2,
  #[sdk(rename = "arabicAlpha")]
  ArabicAlpha,
  #[sdk(rename = "arabicAbjad")]
  ArabicAbjad,
  #[sdk(rename = "hindiVowels")]
  HindiVowels,
  #[sdk(rename = "hindiConsonants")]
  HindiConsonants,
  #[sdk(rename = "hindiNumbers")]
  HindiNumbers,
  #[sdk(rename = "hindiCounting")]
  HindiCounting,
  #[sdk(rename = "thaiLetters")]
  ThaiLetters,
  #[sdk(rename = "thaiNumbers")]
  ThaiNumbers,
  #[sdk(rename = "thaiCounting")]
  ThaiCounting,
  #[sdk(rename = "bahtText")]
  BahtText,
  #[sdk(rename = "dollarText")]
  DollarText,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextDirectionValues {
  #[sdk(rename = "lrTb")]
  #[default]
  LefToRightTopToBottom,
  #[sdk(rename = "tb")]
  LeftToRightTopToBottom2010,
  #[sdk(rename = "tbRl")]
  TopToBottomRightToLeft,
  #[sdk(rename = "rl")]
  TopToBottomRightToLeft2010,
  #[sdk(rename = "btLr")]
  BottomToTopLeftToRight,
  #[sdk(rename = "lr")]
  BottomToTopLeftToRight2010,
  #[sdk(rename = "lrTbV")]
  LefttoRightTopToBottomRotated,
  #[sdk(rename = "tbV")]
  LeftToRightTopToBottomRotated2010,
  #[sdk(rename = "tbRlV")]
  TopToBottomRightToLeftRotated,
  #[sdk(rename = "rlV")]
  TopToBottomRightToLeftRotated2010,
  #[sdk(rename = "tbLrV")]
  TopToBottomLeftToRightRotated,
  #[sdk(rename = "lrV")]
  TopToBottomLeftToRightRotated2010,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptAlgorithmValues {
  #[sdk(rename = "typeAny")]
  #[default]
  TypeAny,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptAlgorithmClassValues {
  #[sdk(rename = "hash")]
  #[default]
  Hash,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptProviderValues {
  #[sdk(rename = "rsaAES")]
  #[default]
  RsaAdvancedEncryptionStandard,
  #[sdk(rename = "rsaFull")]
  RsaFull,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum JustificationValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "start")]
  Start,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "end")]
  End,
  #[sdk(rename = "both")]
  Both,
  #[sdk(rename = "mediumKashida")]
  MediumKashida,
  #[sdk(rename = "distribute")]
  Distribute,
  #[sdk(rename = "numTab")]
  NumTab,
  #[sdk(rename = "highKashida")]
  HighKashida,
  #[sdk(rename = "lowKashida")]
  LowKashida,
  #[sdk(rename = "thaiDistribute")]
  ThaiDistribute,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TabStopValues {
  #[sdk(rename = "clear")]
  #[default]
  Clear,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "start")]
  Start,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "end")]
  End,
  #[sdk(rename = "decimal")]
  Decimal,
  #[sdk(rename = "bar")]
  Bar,
  #[sdk(rename = "num")]
  Number,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BorderValues {
  #[sdk(rename = "nil")]
  #[default]
  Nil,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "single")]
  Single,
  #[sdk(rename = "thick")]
  Thick,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "dotted")]
  Dotted,
  #[sdk(rename = "dashed")]
  Dashed,
  #[sdk(rename = "dotDash")]
  DotDash,
  #[sdk(rename = "dotDotDash")]
  DotDotDash,
  #[sdk(rename = "triple")]
  Triple,
  #[sdk(rename = "thinThickSmallGap")]
  ThinThickSmallGap,
  #[sdk(rename = "thickThinSmallGap")]
  ThickThinSmallGap,
  #[sdk(rename = "thinThickThinSmallGap")]
  ThinThickThinSmallGap,
  #[sdk(rename = "thinThickMediumGap")]
  ThinThickMediumGap,
  #[sdk(rename = "thickThinMediumGap")]
  ThickThinMediumGap,
  #[sdk(rename = "thinThickThinMediumGap")]
  ThinThickThinMediumGap,
  #[sdk(rename = "thinThickLargeGap")]
  ThinThickLargeGap,
  #[sdk(rename = "thickThinLargeGap")]
  ThickThinLargeGap,
  #[sdk(rename = "thinThickThinLargeGap")]
  ThinThickThinLargeGap,
  #[sdk(rename = "wave")]
  Wave,
  #[sdk(rename = "doubleWave")]
  DoubleWave,
  #[sdk(rename = "dashSmallGap")]
  DashSmallGap,
  #[sdk(rename = "dashDotStroked")]
  DashDotStroked,
  #[sdk(rename = "threeDEmboss")]
  ThreeDEmboss,
  #[sdk(rename = "threeDEngrave")]
  ThreeDEngrave,
  #[sdk(rename = "outset")]
  Outset,
  #[sdk(rename = "inset")]
  Inset,
  #[sdk(rename = "apples")]
  Apples,
  #[sdk(rename = "archedScallops")]
  ArchedScallops,
  #[sdk(rename = "babyPacifier")]
  BabyPacifier,
  #[sdk(rename = "babyRattle")]
  BabyRattle,
  #[sdk(rename = "balloons3Colors")]
  Balloons3Colors,
  #[sdk(rename = "balloonsHotAir")]
  BalloonsHotAir,
  #[sdk(rename = "basicBlackDashes")]
  BasicBlackDashes,
  #[sdk(rename = "basicBlackDots")]
  BasicBlackDots,
  #[sdk(rename = "basicBlackSquares")]
  BasicBlackSquares,
  #[sdk(rename = "basicThinLines")]
  BasicThinLines,
  #[sdk(rename = "basicWhiteDashes")]
  BasicWhiteDashes,
  #[sdk(rename = "basicWhiteDots")]
  BasicWhiteDots,
  #[sdk(rename = "basicWhiteSquares")]
  BasicWhiteSquares,
  #[sdk(rename = "basicWideInline")]
  BasicWideInline,
  #[sdk(rename = "basicWideMidline")]
  BasicWideMidline,
  #[sdk(rename = "basicWideOutline")]
  BasicWideOutline,
  #[sdk(rename = "bats")]
  Bats,
  #[sdk(rename = "birds")]
  Birds,
  #[sdk(rename = "birdsFlight")]
  BirdsFlight,
  #[sdk(rename = "cabins")]
  Cabins,
  #[sdk(rename = "cakeSlice")]
  CakeSlice,
  #[sdk(rename = "candyCorn")]
  CandyCorn,
  #[sdk(rename = "celticKnotwork")]
  CelticKnotwork,
  #[sdk(rename = "certificateBanner")]
  CertificateBanner,
  #[sdk(rename = "chainLink")]
  ChainLink,
  #[sdk(rename = "champagneBottle")]
  ChampagneBottle,
  #[sdk(rename = "checkedBarBlack")]
  CheckedBarBlack,
  #[sdk(rename = "checkedBarColor")]
  CheckedBarColor,
  #[sdk(rename = "checkered")]
  Checkered,
  #[sdk(rename = "christmasTree")]
  ChristmasTree,
  #[sdk(rename = "circlesLines")]
  CirclesLines,
  #[sdk(rename = "circlesRectangles")]
  CirclesRectangles,
  #[sdk(rename = "classicalWave")]
  ClassicalWave,
  #[sdk(rename = "clocks")]
  Clocks,
  #[sdk(rename = "compass")]
  Compass,
  #[sdk(rename = "confetti")]
  Confetti,
  #[sdk(rename = "confettiGrays")]
  ConfettiGrays,
  #[sdk(rename = "confettiOutline")]
  ConfettiOutline,
  #[sdk(rename = "confettiStreamers")]
  ConfettiStreamers,
  #[sdk(rename = "confettiWhite")]
  ConfettiWhite,
  #[sdk(rename = "cornerTriangles")]
  CornerTriangles,
  #[sdk(rename = "couponCutoutDashes")]
  CouponCutoutDashes,
  #[sdk(rename = "couponCutoutDots")]
  CouponCutoutDots,
  #[sdk(rename = "crazyMaze")]
  CrazyMaze,
  #[sdk(rename = "creaturesButterfly")]
  CreaturesButterfly,
  #[sdk(rename = "creaturesFish")]
  CreaturesFish,
  #[sdk(rename = "creaturesInsects")]
  CreaturesInsects,
  #[sdk(rename = "creaturesLadyBug")]
  CreaturesLadyBug,
  #[sdk(rename = "crossStitch")]
  CrossStitch,
  #[sdk(rename = "cup")]
  Cup,
  #[sdk(rename = "decoArch")]
  DecoArch,
  #[sdk(rename = "decoArchColor")]
  DecoArchColor,
  #[sdk(rename = "decoBlocks")]
  DecoBlocks,
  #[sdk(rename = "diamondsGray")]
  DiamondsGray,
  #[sdk(rename = "doubleD")]
  DoubleD,
  #[sdk(rename = "doubleDiamonds")]
  DoubleDiamonds,
  #[sdk(rename = "earth1")]
  Earth1,
  #[sdk(rename = "earth2")]
  Earth2,
  #[sdk(rename = "eclipsingSquares1")]
  EclipsingSquares1,
  #[sdk(rename = "eclipsingSquares2")]
  EclipsingSquares2,
  #[sdk(rename = "eggsBlack")]
  EggsBlack,
  #[sdk(rename = "fans")]
  Fans,
  #[sdk(rename = "film")]
  Film,
  #[sdk(rename = "firecrackers")]
  Firecrackers,
  #[sdk(rename = "flowersBlockPrint")]
  FlowersBlockPrint,
  #[sdk(rename = "flowersDaisies")]
  FlowersDaisies,
  #[sdk(rename = "flowersModern1")]
  FlowersModern1,
  #[sdk(rename = "flowersModern2")]
  FlowersModern2,
  #[sdk(rename = "flowersPansy")]
  FlowersPansy,
  #[sdk(rename = "flowersRedRose")]
  FlowersRedRose,
  #[sdk(rename = "flowersRoses")]
  FlowersRoses,
  #[sdk(rename = "flowersTeacup")]
  FlowersTeacup,
  #[sdk(rename = "flowersTiny")]
  FlowersTiny,
  #[sdk(rename = "gems")]
  Gems,
  #[sdk(rename = "gingerbreadMan")]
  GingerbreadMan,
  #[sdk(rename = "gradient")]
  Gradient,
  #[sdk(rename = "handmade1")]
  Handmade1,
  #[sdk(rename = "handmade2")]
  Handmade2,
  #[sdk(rename = "heartBalloon")]
  HeartBalloon,
  #[sdk(rename = "heartGray")]
  HeartGray,
  #[sdk(rename = "hearts")]
  Hearts,
  #[sdk(rename = "heebieJeebies")]
  HeebieJeebies,
  #[sdk(rename = "holly")]
  Holly,
  #[sdk(rename = "houseFunky")]
  HouseFunky,
  #[sdk(rename = "hypnotic")]
  Hypnotic,
  #[sdk(rename = "iceCreamCones")]
  IceCreamCones,
  #[sdk(rename = "lightBulb")]
  LightBulb,
  #[sdk(rename = "lightning1")]
  Lightning1,
  #[sdk(rename = "lightning2")]
  Lightning2,
  #[sdk(rename = "mapPins")]
  MapPins,
  #[sdk(rename = "mapleLeaf")]
  MapleLeaf,
  #[sdk(rename = "mapleMuffins")]
  MapleMuffins,
  #[sdk(rename = "marquee")]
  Marquee,
  #[sdk(rename = "marqueeToothed")]
  MarqueeToothed,
  #[sdk(rename = "moons")]
  Moons,
  #[sdk(rename = "mosaic")]
  Mosaic,
  #[sdk(rename = "musicNotes")]
  MusicNotes,
  #[sdk(rename = "northwest")]
  Northwest,
  #[sdk(rename = "ovals")]
  Ovals,
  #[sdk(rename = "packages")]
  Packages,
  #[sdk(rename = "palmsBlack")]
  PalmsBlack,
  #[sdk(rename = "palmsColor")]
  PalmsColor,
  #[sdk(rename = "paperClips")]
  PaperClips,
  #[sdk(rename = "papyrus")]
  Papyrus,
  #[sdk(rename = "partyFavor")]
  PartyFavor,
  #[sdk(rename = "partyGlass")]
  PartyGlass,
  #[sdk(rename = "pencils")]
  Pencils,
  #[sdk(rename = "people")]
  People,
  #[sdk(rename = "peopleWaving")]
  PeopleWaving,
  #[sdk(rename = "peopleHats")]
  PeopleHats,
  #[sdk(rename = "poinsettias")]
  Poinsettias,
  #[sdk(rename = "postageStamp")]
  PostageStamp,
  #[sdk(rename = "pumpkin1")]
  Pumpkin1,
  #[sdk(rename = "pushPinNote2")]
  PushPinNote2,
  #[sdk(rename = "pushPinNote1")]
  PushPinNote1,
  #[sdk(rename = "pyramids")]
  Pyramids,
  #[sdk(rename = "pyramidsAbove")]
  PyramidsAbove,
  #[sdk(rename = "quadrants")]
  Quadrants,
  #[sdk(rename = "rings")]
  Rings,
  #[sdk(rename = "safari")]
  Safari,
  #[sdk(rename = "sawtooth")]
  Sawtooth,
  #[sdk(rename = "sawtoothGray")]
  SawtoothGray,
  #[sdk(rename = "scaredCat")]
  ScaredCat,
  #[sdk(rename = "seattle")]
  Seattle,
  #[sdk(rename = "shadowedSquares")]
  ShadowedSquares,
  #[sdk(rename = "sharksTeeth")]
  SharksTeeth,
  #[sdk(rename = "shorebirdTracks")]
  ShorebirdTracks,
  #[sdk(rename = "skyrocket")]
  Skyrocket,
  #[sdk(rename = "snowflakeFancy")]
  SnowflakeFancy,
  #[sdk(rename = "snowflakes")]
  Snowflakes,
  #[sdk(rename = "sombrero")]
  Sombrero,
  #[sdk(rename = "southwest")]
  Southwest,
  #[sdk(rename = "stars")]
  Stars,
  #[sdk(rename = "starsTop")]
  StarsTop,
  #[sdk(rename = "stars3d")]
  Stars3d,
  #[sdk(rename = "starsBlack")]
  StarsBlack,
  #[sdk(rename = "starsShadowed")]
  StarsShadowed,
  #[sdk(rename = "sun")]
  Sun,
  #[sdk(rename = "swirligig")]
  Swirligig,
  #[sdk(rename = "tornPaper")]
  TornPaper,
  #[sdk(rename = "tornPaperBlack")]
  TornPaperBlack,
  #[sdk(rename = "trees")]
  Trees,
  #[sdk(rename = "triangleParty")]
  TriangleParty,
  #[sdk(rename = "triangles")]
  Triangles,
  #[sdk(rename = "tribal1")]
  Tribal1,
  #[sdk(rename = "tribal2")]
  Tribal2,
  #[sdk(rename = "tribal3")]
  Tribal3,
  #[sdk(rename = "tribal4")]
  Tribal4,
  #[sdk(rename = "tribal5")]
  Tribal5,
  #[sdk(rename = "tribal6")]
  Tribal6,
  #[sdk(rename = "triangle1")]
  Triangle1,
  #[sdk(rename = "triangle2")]
  Triangle2,
  #[sdk(rename = "triangleCircle1")]
  TriangleCircle1,
  #[sdk(rename = "triangleCircle2")]
  TriangleCircle2,
  #[sdk(rename = "shapes1")]
  Shapes1,
  #[sdk(rename = "shapes2")]
  Shapes2,
  #[sdk(rename = "twistedLines1")]
  TwistedLines1,
  #[sdk(rename = "twistedLines2")]
  TwistedLines2,
  #[sdk(rename = "vine")]
  Vine,
  #[sdk(rename = "waveline")]
  Waveline,
  #[sdk(rename = "weavingAngles")]
  WeavingAngles,
  #[sdk(rename = "weavingBraid")]
  WeavingBraid,
  #[sdk(rename = "weavingRibbon")]
  WeavingRibbon,
  #[sdk(rename = "weavingStrips")]
  WeavingStrips,
  #[sdk(rename = "whiteFlowers")]
  WhiteFlowers,
  #[sdk(rename = "woodwork")]
  Woodwork,
  #[sdk(rename = "xIllusions")]
  XIllusions,
  #[sdk(rename = "zanyTriangles")]
  ZanyTriangles,
  #[sdk(rename = "zigZag")]
  ZigZag,
  #[sdk(rename = "zigZagStitch")]
  ZigZagStitch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DocumentConformance {
  #[sdk(rename = "transitional")]
  #[default]
  Transitional,
  #[sdk(rename = "strict")]
  Strict,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StrictCharacterSet {
  #[sdk(rename = "iso-8859-1")]
  #[default]
  ChsAnsi,
  #[sdk(rename = "macintosh")]
  ChsMacFfn,
  #[sdk(rename = "shift_jis")]
  ChsShiftJis,
  #[sdk(rename = "ks_c-5601-1987")]
  ChsHangeul,
  #[sdk(rename = "KS_C-5601-1992")]
  ChsJohab,
  #[sdk(rename = "GBK")]
  ChsGb2312,
  #[sdk(rename = "Big5")]
  ChsChinese5,
  #[sdk(rename = "windows-1253")]
  ChsGreek,
  #[sdk(rename = "iso-8859-9")]
  ChsTurkish,
  #[sdk(rename = "windows-1258")]
  ChsVietnamese,
  #[sdk(rename = "windows-1255")]
  ChsHebrew,
  #[sdk(rename = "windows-1256")]
  ChsArabic,
  #[sdk(rename = "windows-1257")]
  ChsBaltic,
  #[sdk(rename = "windows-1251")]
  ChsRussian,
  #[sdk(rename = "windows-874")]
  ChsThai,
  #[sdk(rename = "windows-1250")]
  ChsEastEurope,
  #[sdk(other)]
  OtherVariant(Box<[u8]>),
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectDrawAspect {
  #[sdk(rename = "content")]
  #[default]
  Content,
  #[sdk(rename = "icon")]
  Icon,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectUpdateMode {
  #[sdk(rename = "always")]
  #[default]
  Always,
  #[sdk(rename = "onCall")]
  OnCall,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CompatSettingNameValues {
  #[sdk(rename = "compatibilityMode")]
  #[default]
  CompatibilityMode,
  #[sdk(rename = "overrideTableStyleFontSizeAndJustification")]
  OverrideTableStyleFontSizeAndJustification,
  #[sdk(rename = "enableOpenTypeFeatures")]
  EnableOpenTypeFeatures,
  #[sdk(rename = "doNotFlipMirrorIndents")]
  DoNotFlipMirrorIndents,
  #[sdk(rename = "differentiateMultirowTableHeaders")]
  DifferentiateMultirowTableHeaders,
  #[sdk(rename = "useWord2013TrackBottomHyphenation")]
  UseWord2013TrackBottomHyphenation,
  #[sdk(rename = "allowHyphenationAtTrackBottom")]
  AllowHyphenationAtTrackBottom,
  #[sdk(rename = "allowTextAfterFloatingTableBreak")]
  AllowTextAfterFloatingTableBreak,
  #[sdk(other)]
  OtherVariant(Box<[u8]>),
}
/// Table Cell Insertion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cellIns")]
pub struct CellInsertion {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Table Cell Deletion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cellDel")]
pub struct CellDeletion {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlInsRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlInsRangeStart")]
pub struct CustomXmlInsRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlDelRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlDelRangeStart")]
pub struct CustomXmlDelRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlMoveFromRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlMoveFromRangeStart")]
pub struct CustomXmlMoveFromRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlMoveToRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlMoveToRangeStart")]
pub struct CustomXmlMoveToRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Inserted Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ins")]
pub struct Inserted {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Deleted Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:del")]
pub struct Deleted {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Move Source Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveFrom")]
pub struct MoveFrom {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Move Destination Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveTo")]
pub struct MoveTo {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Vertically Merged/Split Table Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cellMerge")]
pub struct CellMerge {
  /// vMerge
  #[sdk(attr(qname = "w:vMerge"))]
  pub vertical_merge: Option<VerticalMergeRevisionValues>,
  /// vMergeOrig
  #[sdk(attr(qname = "w:vMergeOrig"))]
  pub vertical_merge_original: Option<VerticalMergeRevisionValues>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the BookmarkStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bookmarkStart")]
pub struct BookmarkStart {
  /// name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 40u32))]
  pub name: crate::simple_type::StringValue,
  /// colFirst
  #[sdk(attr(qname = "w:colFirst"))]
  pub column_first: Option<crate::simple_type::Int32Value>,
  /// colLast
  #[sdk(attr(qname = "w:colLast"))]
  pub column_last: Option<crate::simple_type::Int32Value>,
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the BookmarkEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bookmarkEnd")]
pub struct BookmarkEnd {
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:commentRangeStart")]
pub struct CommentRangeStart {
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:commentRangeEnd")]
pub struct CommentRangeEnd {
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MoveFromRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveFromRangeEnd")]
pub struct MoveFromRangeEnd {
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MoveToRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveToRangeEnd")]
pub struct MoveToRangeEnd {
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MoveFromRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveFromRangeStart")]
pub struct MoveFromRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 40u32))]
  pub name: crate::simple_type::StringValue,
  /// colFirst
  #[sdk(attr(qname = "w:colFirst"))]
  pub column_first: Option<crate::simple_type::Int32Value>,
  /// colLast
  #[sdk(attr(qname = "w:colLast"))]
  pub column_last: Option<crate::simple_type::Int32Value>,
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MoveToRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveToRangeStart")]
pub struct MoveToRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 40u32))]
  pub name: crate::simple_type::StringValue,
  /// colFirst
  #[sdk(attr(qname = "w:colFirst"))]
  pub column_first: Option<crate::simple_type::Int32Value>,
  /// colLast
  #[sdk(attr(qname = "w:colLast"))]
  pub column_last: Option<crate::simple_type::Int32Value>,
  /// displacedByCustomXml
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlInsRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlInsRangeEnd")]
pub struct CustomXmlInsRangeEnd {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlDelRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlDelRangeEnd")]
pub struct CustomXmlDelRangeEnd {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlMoveFromRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlMoveFromRangeEnd")]
pub struct CustomXmlMoveFromRangeEnd {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CustomXmlMoveToRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlMoveToRangeEnd")]
pub struct CustomXmlMoveToRangeEnd {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Comment Content Reference Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:commentReference")]
pub struct CommentReference {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ParagraphStyleId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pStyle")]
pub struct ParagraphStyleId {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Date Display Mask.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dateFormat")]
pub struct DateFormat {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Gallery Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartGallery")]
pub struct DocPartGallery {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Category Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartCategory")]
pub struct DocPartCategory {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPart")]
pub struct DocPartReference {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Custom XML Element Placeholder Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:placeholder")]
pub struct CustomXmlPlaceholder {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableCaption Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblCaption")]
pub struct TableCaption {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableDescription Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblDescription")]
pub struct TableDescription {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Name for Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct Name {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Predefined Merge Field Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mappedName")]
pub struct MappedName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// UDL Connection String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:udl")]
pub struct UdlConnectionString {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Table Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:table")]
pub struct DataSourceTableName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Connection String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:connectString")]
pub struct ConnectString {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Query For Data Source Records To Merge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:query")]
pub struct Query {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Column Containing E-mail Address.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:addressFieldName")]
pub struct AddressFieldName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Merged E-mail or Fax Subject Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mailSubject")]
pub struct MailSubject {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Frame Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sz")]
pub struct FrameSize {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Associated Paragraph Style Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:style")]
pub struct StyleId {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Description for Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:description")]
pub struct Description {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the SdtAlias Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:alias")]
pub struct SdtAlias {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the Tag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tag")]
pub struct Tag {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Attached Custom XML Schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:attachedSchema")]
pub struct AttachedSchema {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Radix Point for Field Code Evaluation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:decimalSymbol")]
pub struct DecimalSymbol {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// List Separator for Field Code Evaluation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:listSeparator")]
pub struct ListSeparator {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the WebPageEncoding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:encoding")]
pub struct WebPageEncoding {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the AltName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:altName")]
pub struct AltName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the KeepNext Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:keepNext")]
pub struct KeepNext {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the KeepLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:keepLines")]
pub struct KeepLines {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the PageBreakBefore Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pageBreakBefore")]
pub struct PageBreakBefore {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WidowControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:widowControl")]
pub struct WidowControl {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressLineNumbers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressLineNumbers")]
pub struct SuppressLineNumbers {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressAutoHyphens Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressAutoHyphens")]
pub struct SuppressAutoHyphens {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Kinsoku Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:kinsoku")]
pub struct Kinsoku {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WordWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wordWrap")]
pub struct WordWrap {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OverflowPunctuation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:overflowPunct")]
pub struct OverflowPunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TopLinePunctuation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:topLinePunct")]
pub struct TopLinePunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AutoSpaceDE Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoSpaceDE")]
pub struct AutoSpaceDe {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AutoSpaceDN Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoSpaceDN")]
pub struct AutoSpaceDn {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the BiDi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bidi")]
pub struct BiDi {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AdjustRightIndent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:adjustRightInd")]
pub struct AdjustRightIndent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SnapToGrid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:snapToGrid")]
pub struct SnapToGrid {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ContextualSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:contextualSpacing")]
pub struct ContextualSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the MirrorIndents Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mirrorIndents")]
pub struct MirrorIndents {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressOverlap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressOverlap")]
pub struct SuppressOverlap {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Bold Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:b")]
pub struct Bold {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the BoldComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bCs")]
pub struct BoldComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Italic Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:i")]
pub struct Italic {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ItalicComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:iCs")]
pub struct ItalicComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Caps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:caps")]
pub struct Caps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SmallCaps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:smallCaps")]
pub struct SmallCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Strike Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:strike")]
pub struct Strike {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoubleStrike Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dstrike")]
pub struct DoubleStrike {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:outline")]
pub struct Outline {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Shadow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:shadow")]
pub struct Shadow {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Emboss Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:emboss")]
pub struct Emboss {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Imprint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:imprint")]
pub struct Imprint {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NoProof Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noProof")]
pub struct NoProof {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Vanish Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:vanish")]
pub struct Vanish {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebHidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:webHidden")]
pub struct WebHidden {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RightToLeftText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rtl")]
pub struct RightToLeftText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cs")]
pub struct ComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SpecVanish Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:specVanish")]
pub struct SpecVanish {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OfficeMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:oMath")]
pub struct OfficeMath {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Hidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hidden")]
pub struct Hidden {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FormProtection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:formProt")]
pub struct FormProtection {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NoEndnote Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noEndnote")]
pub struct NoEndnote {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TitlePage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:titlePg")]
pub struct TitlePage {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the GutterOnRight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rtlGutter")]
pub struct GutterOnRight {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Form Field Enabled.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:enabled")]
pub struct Enabled {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Recalculate Fields When Current Field Is Modified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:calcOnExit")]
pub struct CalculateOnExit {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Size Form Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sizeAuto")]
pub struct AutomaticallySizeFormField {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Default Checkbox Form Field State.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:default")]
pub struct DefaultCheckBoxFormFieldState {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Checkbox Form Field State.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:checked")]
pub struct Checked {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Keep Source Formatting on Import.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:matchSrc")]
pub struct MatchSource {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Invalidated Field Cache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dirty")]
pub struct Dirty {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Built-In Document Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartUnique")]
pub struct DocPartUnique {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Record Is Included in Mail Merge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:active")]
pub struct Active {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Country/Region-Based Address Field Ordering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dynamicAddress")]
pub struct DynamicAddress {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// First Row of Data Source Contains Column Names.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fHdr")]
pub struct FirstRowHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Query Contains Link to External Query File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:linkToQuery")]
pub struct LinkToQuery {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Blank Lines from Merged Documents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotSuppressBlankLines")]
pub struct DoNotSuppressBlankLines {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Merged Document To E-Mail Attachment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mailAsAttachment")]
pub struct MailAsAttachment {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// View Merged Data Within Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:viewMergedData")]
pub struct ViewMergedData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display All Levels Using Arabic Numerals.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:isLgl")]
pub struct IsLegalNumberingStyle {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Data for HTML blockquote Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:blockQuote")]
pub struct BlockQuote {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Data for HTML body Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bodyDiv")]
pub struct BodyDiv {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Simplified Rules For Table Border Conflicts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useSingleBorderforContiguousCells")]
pub struct UseSingleBorderForContiguousCells {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 6.x Paragraph Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wpJustification")]
pub struct WordPerfectJustification {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Create Custom Tab Stop for Hanging Indent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noTabHangInd")]
pub struct NoTabHangIndent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Add Leading Between Lines of Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noLeading")]
pub struct NoLeading {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Add Additional Space Below Baseline For Underlined East Asian Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:spaceForUL")]
pub struct SpaceForUnderline {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Balance Text Columns within a Section.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noColumnBalance")]
pub struct NoColumnBalance {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Balance Single Byte and Double Byte Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:balanceSingleByteDoubleByteWidth")]
pub struct BalanceSingleByteDoubleByteWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Center Content on Lines With Exact Line Height.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noExtraLineSpacing")]
pub struct NoExtraLineSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Convert Backslash To Yen Sign When Entered.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotLeaveBackslashAlone")]
pub struct DoNotLeaveBackslashAlone {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Underline All Trailing Spaces.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ulTrailSpace")]
pub struct UnderlineTrailingSpaces {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Justify Lines Ending in Soft Line Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotExpandShiftReturn")]
pub struct DoNotExpandShiftReturn {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Expand/Condense Text By Whole Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:spacingInWholePoints")]
pub struct SpacingInWholePoints {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 6.0 Line Wrapping for East Asian Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lineWrapLikeWord6")]
pub struct LineWrapLikeWord6 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Body Text before Header/Footer Contents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printBodyTextBeforeHeader")]
pub struct PrintBodyTextBeforeHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Colors as Black And White without Dithering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printColBlack")]
pub struct PrintColorBlackWhite {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Space width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wpSpaceWidth")]
pub struct WordPerfectSpaceWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display Page/Column Breaks Present in Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:showBreaksInFrames")]
pub struct ShowBreaksInFrames {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Increase Priority Of Font Size During Font Substitution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:subFontBySize")]
pub struct SubFontBySize {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Exact Line Height for Last Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressBottomSpacing")]
pub struct SuppressBottomSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Minimum and Exact Line Height for First Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressTopSpacing")]
pub struct SuppressTopSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Minimum Line Height for First Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressSpacingAtTopOfPage")]
pub struct SuppressSpacingAtTopOfPage {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 5.x Line Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressTopSpacingWP")]
pub struct SuppressTopSpacingWordPerfect {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Space Before On First Line After a Page Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suppressSpBfAfterPgBrk")]
pub struct SuppressSpacingBeforeAfterPageBreak {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Swap Paragraph Borders on Odd Numbered Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:swapBordersFacingPages")]
pub struct SwapBordersFacingPages {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Treat Backslash Quotation Delimiter as Two Quotation Marks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:convMailMergeEsc")]
pub struct ConvertMailMergeEscape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 6.x Font Height Calculation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:truncateFontHeightsLikeWP6")]
pub struct TruncateFontHeightsLikeWordPerfect {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 5.x for the Macintosh Small Caps Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mwSmallCaps")]
pub struct MacWordSmallCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Printer Metrics To Display Documents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:usePrinterMetrics")]
pub struct UsePrinterMetrics {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Suppress Paragraph Borders Next To Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotSuppressParagraphBorders")]
pub struct DoNotSuppressParagraphBorders {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Line Wrap Trailing Spaces.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wrapTrailSpaces")]
pub struct WrapTrailSpaces {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 6.x/95/97 Footnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnoteLayoutLikeWW8")]
pub struct FootnoteLayoutLikeWord8 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 97 Text Wrapping Around Floating Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:shapeLayoutLikeWW8")]
pub struct ShapeLayoutLikeWord8 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Align Table Rows Independently.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:alignTablesRowByRow")]
pub struct AlignTablesRowByRow {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:forgetLastTabAlignment")]
pub struct ForgetLastTabAlignment {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Add Document Grid Line Pitch To Lines in Table Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:adjustLineHeightInTable")]
pub struct AdjustLineHeightInTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 95 Full-Width Character Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoSpaceLikeWord95")]
pub struct AutoSpaceLikeWord95 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Increase Line Height for Raised/Lowered Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noSpaceRaiseLower")]
pub struct NoSpaceRaiseLower {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Fixed Paragraph Spacing for HTML Auto Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotUseHTMLParagraphAutoSpacing")]
pub struct DoNotUseHtmlParagraphAutoSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Space Before Table When Deciding If Table Should Wrap Floating Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:layoutRawTableWidth")]
pub struct LayoutRawTableWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Table Rows to Wrap Inline Objects Independently.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:layoutTableRowsApart")]
pub struct LayoutTableRowsApart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 97 East Asian Line Breaking.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useWord97LineBreakRules")]
pub struct UseWord97LineBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Allow Floating Tables To Break Across Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotBreakWrappedTables")]
pub struct DoNotBreakWrappedTables {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Snap to Document Grid in Table Cells with Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotSnapToGridInCell")]
pub struct DoNotSnapToGridInCell {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Select Field When First or Last Character Is Selected.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:selectFldWithFirstOrLastChar")]
pub struct SelectFieldWithFirstOrLastChar {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Legacy Ethiopic and Amharic Line Breaking Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:applyBreakingRules")]
pub struct ApplyBreakingRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Allow Hanging Punctuation With Character Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotWrapTextWithPunct")]
pub struct DoNotWrapTextWithPunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Compress Compressible Characters When Using Document Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotUseEastAsianBreakRules")]
pub struct DoNotUseEastAsianBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 2002 Table Style Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useWord2002TableStyleRules")]
pub struct UseWord2002TableStyleRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Tables to AutoFit Into Page Margins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:growAutofit")]
pub struct GrowAutofit {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Bypass East Asian/Complex Script Layout Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useFELayout")]
pub struct UseFarEastLayout {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useNormalStyleForList")]
pub struct UseNormalStyleForList {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Hanging Indent When Creating Tab Stop After Numbering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotUseIndentAsNumberingTabStop")]
pub struct DoNotUseIndentAsNumberingTabStop {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Alternate Set of East Asian Line Breaking Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useAltKinsokuLineBreakRules")]
pub struct UseAltKinsokuLineBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Contextual Spacing of Paragraphs in Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:allowSpaceOfSameStyleInTable")]
pub struct AllowSpaceOfSameStyleInTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Ignore Floating Objects When Calculating Paragraph Indentation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotSuppressIndentation")]
pub struct DoNotSuppressIndentation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not AutoFit Tables To Fit Next To Wrapped Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotAutofitConstrainedTables")]
pub struct DoNotAutofitConstrainedTables {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Table Columns To Exceed Preferred Widths of Constituent Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autofitToFirstFixedWidthCell")]
pub struct AutofitToFirstFixedWidthCell {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Underline Following Character Following Numbering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:underlineTabInNumList")]
pub struct UnderlineTabInNumberingList {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Always Use Fixed Width for Hangul Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:displayHangulFixedWidth")]
pub struct DisplayHangulFixedWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Always Move Paragraph Mark to Page after a Page Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:splitPgBreakAndParaMark")]
pub struct SplitPageBreakAndParagraphMark {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Vertically Align Cells Containing Floating Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotVertAlignCellWithSp")]
pub struct DoNotVerticallyAlignCellWithShape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Break Table Rows Around Floating Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotBreakConstrainedForcedTable")]
pub struct DoNotBreakConstrainedForcedTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Vertical Alignment in Textboxes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotVertAlignInTxbx")]
pub struct DoNotVerticallyAlignInTextBox {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use ANSI Kerning Pairs from Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useAnsiKerningPairs")]
pub struct UseAnsiKerningPairs {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Cached Paragraph Information for Column Balancing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cachedColBalance")]
pub struct CachedColumnBalance {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ShowingPlaceholder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:showingPlcHdr")]
pub struct ShowingPlaceholder {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TemporarySdt Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:temporary")]
pub struct TemporarySdt {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Personal Information from Document Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:removePersonalInformation")]
pub struct RemovePersonalInformation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Date and Time from Annotations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:removeDateAndTime")]
pub struct RemoveDateAndTime {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Boundary For Header/Footer or Between Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotDisplayPageBoundaries")]
pub struct DoNotDisplayPageBoundaries {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display Background Objects When Displaying Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:displayBackgroundShape")]
pub struct DisplayBackgroundShape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print PostScript Codes With Document Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printPostScriptOverText")]
pub struct PrintPostScriptOverText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Fractional Character Widths.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printFractionalCharacterWidth")]
pub struct PrintFractionalCharacterWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Print Form Field Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printFormsData")]
pub struct PrintFormsData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Embed TrueType Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedTrueTypeFonts")]
pub struct EmbedTrueTypeFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Embed Common System Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedSystemFonts")]
pub struct EmbedSystemFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Subset Fonts When Embedding.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:saveSubsetFonts")]
pub struct SaveSubsetFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Save Form Field Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:saveFormsData")]
pub struct SaveFormsData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Mirror Page Margins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mirrorMargins")]
pub struct MirrorMargins {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Align Paragraph and Table Borders with Page Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:alignBordersAndEdges")]
pub struct AlignBorderAndEdges {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Page Border Excludes Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bordersDoNotSurroundHeader")]
pub struct BordersDoNotSurroundHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Page Border Excludes Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bordersDoNotSurroundFooter")]
pub struct BordersDoNotSurroundFooter {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Position Gutter At Top of Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gutterAtTop")]
pub struct GutterAtTop {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Indication of Spelling Errors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hideSpellingErrors")]
pub struct HideSpellingErrors {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Indication of Grammatical Errors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hideGrammaticalErrors")]
pub struct HideGrammaticalErrors {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Structured Document Tag Placeholder Text Should be Resaved.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:formsDesign")]
pub struct FormsDesign {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Update Styles From Document Template.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:linkStyles")]
pub struct LinkStyles {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Track Revisions to Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trackRevisions")]
pub struct TrackRevisions {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Move Syntax When Tracking Revisions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotTrackMoves")]
pub struct DoNotTrackMoves {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Track Formatting Revisions When Tracking Revisions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotTrackFormatting")]
pub struct DoNotTrackFormatting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Automatic Formatting to Override Formatting Protection Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoFormatOverride")]
pub struct AutoFormatOverride {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Prevent Modification of Themes Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:styleLockTheme")]
pub struct StyleLockThemesPart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Prevent Replacement of Styles Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:styleLockQFSet")]
pub struct StyleLockStylesPart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Hyphenate Document Contents When Displayed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoHyphenation")]
pub struct AutoHyphenation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Hyphenate Words in ALL CAPITAL LETTERS.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotHyphenateCaps")]
pub struct DoNotHyphenateCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Show E-Mail Message Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:showEnvelope")]
pub struct ShowEnvelope {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Different Even/Odd Page Headers and Footers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:evenAndOddHeaders")]
pub struct EvenAndOddHeaders {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Reverse Book Fold Printing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bookFoldRevPrinting")]
pub struct BookFoldReversePrinting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Book Fold Printing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bookFoldPrinting")]
pub struct BookFoldPrinting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Margins for Drawing Grid Origin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotUseMarginsForDrawingGridOrigin")]
pub struct DoNotUseMarginsForDrawingGridOrigin {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Show Visual Indicator For Form Fields.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotShadeFormData")]
pub struct DoNotShadeFormData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Never Kern Punctuation Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noPunctuationKerning")]
pub struct NoPunctuationKerning {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Two Pages Per Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printTwoOnOne")]
pub struct PrintTwoOnOne {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Strict Kinsoku Rules for Japanese Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:strictFirstAndLastChars")]
pub struct StrictFirstAndLastChars {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Generate Thumbnail For Document On Save.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:savePreviewPicture")]
pub struct SavePreviewPicture {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Validate Custom XML Markup Against Schemas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotValidateAgainstSchema")]
pub struct DoNotValidateAgainstSchema {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Saving Document As XML File When Custom XML Markup Is Invalid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:saveInvalidXml")]
pub struct SaveInvalidXml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Mixed Content When Validating Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ignoreMixedContent")]
pub struct IgnoreMixedContent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Custom XML Element Names as Default Placeholder Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:alwaysShowPlaceholderText")]
pub struct AlwaysShowPlaceholderText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Show Visual Indicator For Invalid Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotDemarcateInvalidXml")]
pub struct DoNotDemarcateInvalidXml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Save Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:saveXmlDataOnly")]
pub struct SaveXmlDataOnly {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Save Document as XML File through Custom XSL Transform.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:useXSLTWhenSaving")]
pub struct UseXsltWhenSaving {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Show Visual Indicators for Custom XML Markup Start/End Locations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:showXMLTags")]
pub struct ShowXmlTags {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Mark Custom XML Elements With No Namespace As Invalid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:alwaysMergeEmptyNamespace")]
pub struct AlwaysMergeEmptyNamespace {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Recalculate Fields on Open.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:updateFields")]
pub struct UpdateFieldsOnOpen {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Disable Features Incompatible With Earlier Word Processing Formats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:uiCompat97To2003")]
pub struct UiCompatibleWith97To2003 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Include Content in Text Boxes, Footnotes, and Endnotes in Document Statistics.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotIncludeSubdocsInStats")]
pub struct DoNotIncludeSubdocsInStats {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Automatically Compress Images.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotAutoCompressPictures")]
pub struct DoNotAutoCompressPictures {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OptimizeForBrowser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:optimizeForBrowser")]
pub struct OptimizeForBrowser {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RelyOnVML Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:relyOnVML")]
pub struct RelyOnVml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AllowPNG Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:allowPNG")]
pub struct AllowPng {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotRelyOnCSS Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotRelyOnCSS")]
pub struct DoNotRelyOnCss {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotSaveAsSingleFile Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotSaveAsSingleFile")]
pub struct DoNotSaveAsSingleFile {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotOrganizeInFolder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotOrganizeInFolder")]
pub struct DoNotOrganizeInFolder {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotUseLongFileNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:doNotUseLongFileNames")]
pub struct DoNotUseLongFileNames {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NotTrueType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:notTrueType")]
pub struct NotTrueType {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FrameProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:framePr")]
pub struct FrameProperties {
  /// Drop Cap Frame
  #[sdk(attr(qname = "w:dropCap"))]
  pub drop_cap: Option<DropCapLocationValues>,
  /// Drop Cap Vertical Height in Lines
  #[sdk(attr(qname = "w:lines"))]
  #[sdk(number_range(range = 1..= 10))]
  pub lines: Option<crate::simple_type::Int32Value>,
  /// Frame Width
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub width: Option<crate::simple_type::TwipsMeasureValue>,
  /// Frame Height
  #[sdk(attr(qname = "w:h"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub height: Option<crate::simple_type::TwipsMeasureValue>,
  /// Vertical Frame Padding
  #[sdk(attr(qname = "w:vSpace"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub vertical_space: Option<crate::simple_type::TwipsMeasureValue>,
  /// Horizontal Frame Padding
  #[sdk(attr(qname = "w:hSpace"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub horizontal_space: Option<crate::simple_type::TwipsMeasureValue>,
  /// Text Wrapping Around Frame
  #[sdk(attr(qname = "w:wrap"))]
  pub wrap: Option<TextWrappingValues>,
  /// Frame Horizontal Positioning Base
  #[sdk(attr(qname = "w:hAnchor"))]
  pub horizontal_position: Option<HorizontalAnchorValues>,
  /// Frame Vertical Positioning Base
  #[sdk(attr(qname = "w:vAnchor"))]
  pub vertical_position: Option<VerticalAnchorValues>,
  /// Absolute Horizontal Position
  #[sdk(attr(qname = "w:x"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub x: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Relative Horizontal Position
  #[sdk(attr(qname = "w:xAlign"))]
  pub x_align: Option<HorizontalAlignmentValues>,
  /// Absolute Vertical Position
  #[sdk(attr(qname = "w:y"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub y: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Relative Vertical Position
  #[sdk(attr(qname = "w:yAlign"))]
  pub y_align: Option<VerticalAlignmentValues>,
  /// Frame Height Type
  #[sdk(attr(qname = "w:hRule"))]
  pub height_type: Option<HeightRuleValues>,
  /// Lock Frame Anchor to Paragraph
  #[sdk(attr(qname = "w:anchorLock"))]
  pub anchor_lock: Option<crate::simple_type::OnOffValue>,
  /// Frame Fill Color
  #[sdk(attr(qname = ":fillcolor"))]
  pub fill_color: Option<crate::simple_type::StringValue>,
}
/// Defines the NumberingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numPr")]
pub struct NumberingProperties {
  /// Numbering Level Reference
  #[sdk(child(qname = "w:ilvl"))]
  pub numbering_level_reference: Option<NumberingLevelReference>,
  /// Numbering Definition Instance Reference
  #[sdk(child(qname = "w:numId"))]
  pub numbering_id: Option<NumberingId>,
  /// Previous Paragraph Numbering Properties
  #[sdk(child(qname = "w:numberingChange"))]
  pub numbering_change: Option<NumberingChange>,
  /// Inserted Numbering Properties
  #[sdk(child(qname = "w:ins"))]
  pub inserted: Option<Inserted>,
}
/// Defines the ParagraphBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pBdr")]
pub struct ParagraphBorders {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Paragraph Border Above Identical Paragraphs
  #[sdk(child(qname = "w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Paragraph Border
  #[sdk(child(qname = "w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Paragraph Border Between Identical Paragraphs
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Paragraph Border
  #[sdk(child(qname = "w:right"))]
  pub right_border: Option<RightBorder>,
  /// Paragraph Border Between Identical Paragraphs
  #[sdk(child(qname = "w:between"))]
  pub between_border: Option<BetweenBorder>,
  /// Paragraph Border Between Facing Pages
  #[sdk(child(qname = "w:bar"))]
  pub bar_border: Option<BarBorder>,
}
/// Defines the Shading Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:shd")]
pub struct Shading {
  /// Shading Pattern
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<ShadingPatternValues>,
  /// Shading Pattern Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Shading Pattern Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Shading Pattern Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Shading Pattern Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Shading Background Color
  #[sdk(attr(qname = "w:fill"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub fill: Option<crate::simple_type::StringValue>,
  /// Shading Background Theme Color
  #[sdk(attr(qname = "w:themeFill"))]
  pub theme_fill: Option<ThemeColorValues>,
  /// Shading Background Theme Color Tint
  #[sdk(attr(qname = "w:themeFillTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_fill_tint: Option<crate::simple_type::StringValue>,
  /// Shading Background Theme Color Shade
  #[sdk(attr(qname = "w:themeFillShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_fill_shade: Option<crate::simple_type::StringValue>,
}
/// Defines the Tabs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tabs")]
pub struct Tabs {
  /// Custom Tab Stop.
  #[sdk(child(qname = "w:tab"))]
  pub tab_stop: Vec<TabStop>,
}
/// Defines the SpacingBetweenLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:spacing")]
pub struct SpacingBetweenLines {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Spacing Above Paragraph
  #[sdk(attr(qname = "w:before"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub before: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Spacing Above Paragraph IN Line Units
  #[sdk(attr(qname = "w:beforeLines"))]
  pub before_lines: Option<crate::simple_type::Int32Value>,
  /// Automatically Determine Spacing Above Paragraph
  #[sdk(attr(qname = "w:beforeAutospacing"))]
  pub before_auto_spacing: Option<crate::simple_type::OnOffValue>,
  /// Spacing Below Paragraph
  #[sdk(attr(qname = "w:after"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub after: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Spacing Below Paragraph in Line Units
  #[sdk(attr(qname = "w:afterLines"))]
  pub after_lines: Option<crate::simple_type::Int32Value>,
  /// Automatically Determine Spacing Below Paragraph
  #[sdk(attr(qname = "w:afterAutospacing"))]
  pub after_auto_spacing: Option<crate::simple_type::OnOffValue>,
  /// Spacing Between Lines in Paragraph
  #[sdk(attr(qname = "w:line"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub line: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Type of Spacing Between Lines
  #[sdk(attr(qname = "w:lineRule"))]
  pub line_rule: Option<LineSpacingRuleValues>,
}
/// Defines the Indentation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ind")]
pub struct Indentation {
  /// Left Indentation
  #[sdk(attr(match_local_name, qname = "w:left"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub left: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// start
  #[sdk(attr(qname = "w:start"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub start: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Left Indentation in Character Units
  #[sdk(attr(qname = "w:leftChars"))]
  pub left_chars: Option<crate::simple_type::Int32Value>,
  /// startChars
  #[sdk(attr(qname = "w:startChars"))]
  pub start_characters: Option<crate::simple_type::Int32Value>,
  /// Right Indentation
  #[sdk(attr(qname = "w:right"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub right: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// end
  #[sdk(attr(qname = "w:end"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub end: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Right Indentation in Character Units
  #[sdk(attr(qname = "w:rightChars"))]
  pub right_chars: Option<crate::simple_type::Int32Value>,
  /// endChars
  #[sdk(attr(qname = "w:endChars"))]
  pub end_characters: Option<crate::simple_type::Int32Value>,
  /// Indentation Removed from First Line
  #[sdk(attr(qname = "w:hanging"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub hanging: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Indentation Removed From First Line in Character Units
  #[sdk(attr(qname = "w:hangingChars"))]
  pub hanging_chars: Option<crate::simple_type::Int32Value>,
  /// Additional First Line Indentation
  #[sdk(attr(qname = "w:firstLine"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub first_line: Option<crate::simple_type::TwipsMeasureValue>,
  /// Additional First Line Indentation in Character Units
  #[sdk(attr(qname = "w:firstLineChars"))]
  pub first_line_chars: Option<crate::simple_type::Int32Value>,
}
/// Defines the Justification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:jc")]
pub struct Justification {
  /// Alignment Type
  #[sdk(attr(qname = "w:val"))]
  pub val: JustificationValues,
}
/// Defines the TextDirection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:textDirection")]
pub struct TextDirection {
  /// Direction of Text Flow
  #[sdk(attr(qname = "w:val"))]
  pub val: TextDirectionValues,
}
/// Defines the TextAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:textAlignment")]
pub struct TextAlignment {
  /// Vertical Character Alignment Position
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalTextAlignmentValues,
}
/// Defines the TextBoxTightWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:textboxTightWrap")]
pub struct TextBoxTightWrap {
  /// Lines to Tight Wrap to Paragraph Extents
  #[sdk(attr(qname = "w:val"))]
  pub val: TextBoxTightWrapValues,
}
/// Defines the OutlineLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:outlineLvl")]
pub struct OutlineLevel {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridSpan Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gridSpan")]
pub struct GridSpan {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridBefore Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gridBefore")]
pub struct GridBefore {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridAfter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gridAfter")]
pub struct GridAfter {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Drop-Down List Selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:result")]
pub struct DropDownListSelection {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Record Currently Displayed In Merged Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:activeRecord")]
pub struct ActiveRecord {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Mail Merge Error Reporting Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:checkErrors")]
pub struct CheckErrors {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Restart Numbering Level Symbol.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvlRestart")]
pub struct LevelRestart {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Picture Numbering Symbol Definition Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvlPicBulletId")]
pub struct LevelPictureBulletId {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Numbering Level Starting Value Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:startOverride")]
pub struct StartOverrideNumberingValue {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Last Reviewed Abstract Numbering Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numIdMacAtCleanup")]
pub struct NumberingIdMacAtCleanup {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SdtId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:id")]
pub struct SdtId {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the PixelsPerInch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pixelsPerInch")]
pub struct PixelsPerInch {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the ParagraphPropertiesChange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPrChange")]
pub struct ParagraphPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub paragraph_properties_extended: Option<std::boxed::Box<ParagraphPropertiesExtended>>,
}
/// Header Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:headerReference")]
pub struct HeaderReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: HeaderFooterValues,
  /// Relationship to Part
  #[sdk(attr(match_local_name, qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Footer Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footerReference")]
pub struct FooterReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: HeaderFooterValues,
  /// Relationship to Part
  #[sdk(attr(match_local_name, qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:br")]
pub struct Break {
  /// Break Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<BreakValues>,
  /// Restart Location For Text Wrapping Break
  #[sdk(attr(qname = "w:clear"))]
  pub clear: Option<BreakTextRestartLocationValues>,
}
/// Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:t")]
pub struct Text(pub TextType);
/// Deleted Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:delText")]
pub struct DeletedText {
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Field Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:instrText")]
pub struct FieldCode {
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Deleted Field Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:delInstrText")]
pub struct DeletedFieldCode {
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TextType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "")]
pub struct TextType {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Symbol Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sym")]
pub struct SymbolChar {
  /// Symbol Character Font
  #[sdk(attr(qname = "w:font"))]
  #[sdk(string_length(max = 31u32))]
  pub font: Option<crate::simple_type::StringValue>,
  /// Symbol Character Code
  #[sdk(attr(qname = "w:char"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub char: Option<crate::simple_type::HexBinaryValue>,
}
/// Inline Embedded Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:object")]
pub struct EmbeddedObject {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// dxaOrig
  #[sdk(attr(qname = "w:dxaOrig"))]
  pub dxa_original: Option<crate::simple_type::TwipsMeasureValue>,
  /// dyaOrig
  #[sdk(attr(qname = "w:dyaOrig"))]
  pub dya_original: Option<crate::simple_type::TwipsMeasureValue>,
  /// anchorId
  #[sdk(attr(qname = "w14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(
        choice(
            child(variant = Group, qname = "v:group"),
            child(variant = ImageFile, qname = "v:image"),
            child(variant = Line, qname = "v:line"),
            child(variant = Oval, qname = "v:oval"),
            child(variant = PolyLine, qname = "v:polyline"),
            child(variant = Rectangle, qname = "v:rect"),
            child(variant = RoundRectangle, qname = "v:roundrect"),
            child(variant = Shape, qname = "v:shape"),
            child(variant = Shapetype, qname = "v:shapetype"),
            child(variant = Arc, qname = "v:arc"),
            child(variant = Curve, qname = "v:curve"),
            child(variant = OleObject, qname = "o:OLEObject")
        )
    )]
  pub embedded_object_choice1: Vec<EmbeddedObjectChoice>,
  /// DrawingML Object.
  #[sdk(child(qname = "w:drawing"))]
  pub drawing: Option<std::boxed::Box<Drawing>>,
  #[sdk(
        choice(
            child(variant = Control, qname = "w:control"),
            child(variant = ObjectEmbed, qname = "w:objectEmbed"),
            child(variant = ObjectLink, qname = "w:objectLink")
        )
    )]
  pub embedded_object_choice2: Option<EmbeddedObjectChoice2>,
}
/// VML Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pict")]
pub struct Picture {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// anchorId
  #[sdk(attr(qname = "w14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(
        choice(
            child(variant = Group, qname = "v:group"),
            child(variant = ImageFile, qname = "v:image"),
            child(variant = Line, qname = "v:line"),
            child(variant = Oval, qname = "v:oval"),
            child(variant = PolyLine, qname = "v:polyline"),
            child(variant = Rectangle, qname = "v:rect"),
            child(variant = RoundRectangle, qname = "v:roundrect"),
            child(variant = Shape, qname = "v:shape"),
            child(variant = Shapetype, qname = "v:shapetype"),
            child(variant = Arc, qname = "v:arc"),
            child(variant = Curve, qname = "v:curve"),
            child(variant = OleObject, qname = "o:OLEObject")
        )
    )]
  pub picture_choice: Vec<PictureChoice>,
  /// Defines the MovieReference Class.
  #[sdk(child(qname = "w:movie"))]
  pub movie_reference: Option<MovieReference>,
  /// Defines the Control Class.
  #[sdk(child(qname = "w:control"))]
  pub control: Option<Control>,
}
/// Complex Field Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fldChar")]
pub struct FieldChar {
  /// Field Character Type
  #[sdk(attr(qname = "w:fldCharType"))]
  pub field_char_type: FieldCharValues,
  /// Field Should Not Be Recalculated
  #[sdk(attr(qname = "w:fldLock"))]
  pub field_lock: Option<crate::simple_type::OnOffValue>,
  /// Field Result Invalidated
  #[sdk(attr(qname = "w:dirty"))]
  pub dirty: Option<crate::simple_type::OnOffValue>,
  #[sdk(
        choice(
            child(variant = FieldData, qname = "w:fldData"),
            child(variant = FormFieldData, qname = "w:ffData"),
            child(variant = NumberingChange, qname = "w:numberingChange")
        )
    )]
  pub field_char_choice: Option<FieldCharChoice>,
}
/// Phonetic Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ruby")]
pub struct Ruby {
  /// Phonetic Guide Properties
  #[sdk(child(qname = "w:rubyPr"))]
  pub ruby_properties: std::boxed::Box<RubyProperties>,
  /// Phonetic Guide Text
  #[sdk(child(qname = "w:rt"))]
  pub ruby_content: std::boxed::Box<RubyContent>,
  /// Phonetic Guide Base Text
  #[sdk(child(qname = "w:rubyBase"))]
  pub ruby_base: std::boxed::Box<RubyBase>,
}
/// Footnote Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnoteReference")]
pub struct FootnoteReference {
  /// Suppress Footnote/Endnote Reference Mark
  #[sdk(attr(qname = "w:customMarkFollows"))]
  pub custom_mark_follows: Option<crate::simple_type::OnOffValue>,
  /// Footnote/Endnote ID Reference
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// Endnote Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnoteReference")]
pub struct EndnoteReference {
  /// Suppress Footnote/Endnote Reference Mark
  #[sdk(attr(qname = "w:customMarkFollows"))]
  pub custom_mark_follows: Option<crate::simple_type::OnOffValue>,
  /// Footnote/Endnote ID Reference
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// DrawingML Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:drawing")]
pub struct Drawing {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  #[sdk(
        choice(
            child(variant = Anchor, qname = "wp:anchor"),
            child(variant = Inline, qname = "wp:inline")
        )
    )]
  pub drawing_choice: Option<DrawingChoice>,
}
/// Absolute Position Tab Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ptab")]
pub struct PositionalTab {
  /// Positional Tab Stop Alignment
  #[sdk(attr(qname = "w:alignment"))]
  pub alignment: AbsolutePositionTabAlignmentValues,
  /// Positional Tab Base
  #[sdk(attr(qname = "w:relativeTo"))]
  pub relative_to: AbsolutePositionTabPositioningBaseValues,
  /// Tab Leader Character
  #[sdk(attr(qname = "w:leader"))]
  pub leader: AbsolutePositionTabLeaderCharValues,
}
/// Defines the RunStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rStyle")]
pub struct RunStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblStyle")]
pub struct TableStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Paragraph Style's Associated Numbering Level.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pStyle")]
pub struct ParagraphStyleIdInLevel {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Abstract Numbering Definition Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct AbstractNumDefinitionName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Numbering Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:styleLink")]
pub struct StyleLink {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Numbering Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numStyleLink")]
pub struct NumberingStyleLink {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Alternate Style Names.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:aliases")]
pub struct Aliases {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Parent Style ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:basedOn")]
pub struct BasedOn {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Style For Next Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:next")]
pub struct NextParagraphStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Linked Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:link")]
pub struct LinkedStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Paragraph Style Applied to Automatically Generated Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:clickAndTypeStyle")]
pub struct ClickAndTypeStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Default Table Style for Newly Inserted Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:defaultTableStyle")]
pub struct DefaultTableStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the RunFonts Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rFonts")]
pub struct RunFonts {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Font Content Type
  #[sdk(attr(qname = "w:hint"))]
  pub hint: Option<FontTypeHintValues>,
  /// ASCII Font
  #[sdk(attr(qname = "w:ascii"))]
  #[sdk(string_length(max = 31u32))]
  pub ascii: Option<crate::simple_type::StringValue>,
  /// High ANSI Font
  #[sdk(attr(qname = "w:hAnsi"))]
  #[sdk(string_length(max = 31u32))]
  pub high_ansi: Option<crate::simple_type::StringValue>,
  /// East Asian Font
  #[sdk(attr(qname = "w:eastAsia"))]
  #[sdk(string_length(max = 31u32))]
  pub east_asia: Option<crate::simple_type::StringValue>,
  /// Complex Script Font
  #[sdk(attr(qname = "w:cs"))]
  #[sdk(string_length(max = 31u32))]
  pub complex_script: Option<crate::simple_type::StringValue>,
  /// ASCII Theme Font
  #[sdk(attr(qname = "w:asciiTheme"))]
  pub ascii_theme: Option<ThemeFontValues>,
  /// High ANSI Theme Font
  #[sdk(attr(qname = "w:hAnsiTheme"))]
  pub high_ansi_theme: Option<ThemeFontValues>,
  /// East Asian Theme Font
  #[sdk(attr(qname = "w:eastAsiaTheme"))]
  pub east_asia_theme: Option<ThemeFontValues>,
  /// Complex Script Theme Font
  #[sdk(attr(qname = "w:cstheme"))]
  pub complex_script_theme: Option<ThemeFontValues>,
}
/// Defines the Color Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:color")]
pub struct Color {
  /// Run Content Color
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 2u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub val: Option<crate::simple_type::StringValue>,
  /// Run Content Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Run Content Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Run Content Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
}
/// Defines the Spacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:spacing")]
pub struct Spacing {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CharacterScale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:w")]
pub struct CharacterScale {
  /// Text Expansion/Compression Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 1..= 600))]
  pub val: Option<crate::simple_type::IntegerValue>,
}
/// Defines the Kern Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:kern")]
pub struct Kern {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3277))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the Position Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:position")]
pub struct Position {
  /// Signed Half-Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedHpsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the FontSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sz")]
pub struct FontSize {
  /// Half Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "2",
    max = "3277",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the FontSizeComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:szCs")]
pub struct FontSizeComplexScript {
  /// Half Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "2",
    max = "3277",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Checkbox Form Field Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:size")]
pub struct FormFieldSize {
  /// Half Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "2",
    max = "3277",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Phonetic Guide Text Font Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hps")]
pub struct PhoneticGuideTextFontSize {
  /// Half Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "2",
    max = "3277",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Phonetic Guide Base Text Font Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hpsBaseText")]
pub struct PhoneticGuideBaseTextSize {
  /// Half Point Measurement
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "2",
    max = "3277",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the Highlight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:highlight")]
pub struct Highlight {
  /// Highlighting Color
  #[sdk(attr(qname = "w:val"))]
  pub val: HighlightColorValues,
}
/// Defines the Underline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:u")]
pub struct Underline {
  /// Underline Style
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<UnderlineValues>,
  /// Underline Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Underline Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Underline Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Underline Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
}
/// Defines the TextEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:effect")]
pub struct TextEffect {
  /// Animated Text Effect Type
  #[sdk(attr(qname = "w:val"))]
  pub val: TextEffectValues,
}
/// Defines the Border Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bdr")]
pub struct Border {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Paragraph Border Above Identical Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:top")]
pub struct TopBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Left Paragraph Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:left")]
pub struct LeftBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Paragraph Border Between Identical Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bottom")]
pub struct BottomBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Right Paragraph Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:right")]
pub struct RightBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Paragraph Border Between Identical Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:between")]
pub struct BetweenBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Paragraph Border Between Facing Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bar")]
pub struct BarBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Defines the StartBorder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:start")]
pub struct StartBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Defines the EndBorder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:end")]
pub struct EndBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Table Inside Horizontal Edges Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:insideH")]
pub struct InsideHorizontalBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Table Inside Vertical Edges Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:insideV")]
pub struct InsideVerticalBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Table Cell Top Left to Bottom Right Diagonal Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tl2br")]
pub struct TopLeftToBottomRightCellBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Table Cell Top Right to Bottom Left Diagonal Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tr2bl")]
pub struct TopRightToBottomLeftCellBorder {
  /// Border Style
  #[sdk(attr(qname = "w:val"))]
  pub val: BorderValues,
  /// Border Color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// Border Theme Color
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// Border Theme Color Tint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// Border Theme Color Shade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Border Width
  #[sdk(attr(qname = "w:sz"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Border Spacing Measurement
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_range(range = 0..= 31))]
  pub space: Option<crate::simple_type::UInt32Value>,
  /// Border Shadow
  #[sdk(attr(qname = "w:shadow"))]
  pub shadow: Option<crate::simple_type::OnOffValue>,
  /// Create Frame Effect
  #[sdk(attr(qname = "w:frame"))]
  pub frame: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FitText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fitText")]
pub struct FitText {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::TwipsMeasureValue,
  /// Fit Text Run ID
  #[sdk(attr(qname = "w:id"))]
  pub id: Option<crate::simple_type::Int32Value>,
}
/// Defines the VerticalTextAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:vertAlign")]
pub struct VerticalTextAlignment {
  /// Subscript/Superscript Value
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalPositionValues,
}
/// Defines the Emphasis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:em")]
pub struct Emphasis {
  /// Emphasis Mark Type
  #[sdk(attr(qname = "w:val"))]
  pub val: EmphasisMarkValues,
}
/// Defines the Languages Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lang")]
pub struct Languages {
  /// Latin Language
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 84u32))]
  pub val: Option<crate::simple_type::StringValue>,
  /// East Asian Language
  #[sdk(attr(qname = "w:eastAsia"))]
  #[sdk(string_length(max = 84u32))]
  pub east_asia: Option<crate::simple_type::StringValue>,
  /// Complex Script Language
  #[sdk(attr(qname = "w:bidi"))]
  #[sdk(string_length(max = 84u32))]
  pub bidi: Option<crate::simple_type::StringValue>,
}
/// Theme Font Languages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:themeFontLang")]
pub struct ThemeFontLanguages {
  /// Latin Language
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 84u32))]
  pub val: Option<crate::simple_type::StringValue>,
  /// East Asian Language
  #[sdk(attr(qname = "w:eastAsia"))]
  #[sdk(string_length(max = 84u32))]
  pub east_asia: Option<crate::simple_type::StringValue>,
  /// Complex Script Language
  #[sdk(attr(qname = "w:bidi"))]
  #[sdk(string_length(max = 84u32))]
  pub bidi: Option<crate::simple_type::StringValue>,
}
/// Defines the EastAsianLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:eastAsianLayout")]
pub struct EastAsianLayout {
  /// East Asian Typography Run ID
  #[sdk(attr(qname = "w:id"))]
  pub id: Option<crate::simple_type::Int32Value>,
  /// Two Lines in One
  #[sdk(attr(qname = "w:combine"))]
  pub combine: Option<crate::simple_type::OnOffValue>,
  /// Display Brackets Around Two Lines in One
  #[sdk(attr(qname = "w:combineBrackets"))]
  pub combine_brackets: Option<CombineBracketValues>,
  /// Horizontal in Vertical (Rotate Text)
  #[sdk(attr(qname = "w:vert"))]
  pub vertical: Option<crate::simple_type::OnOffValue>,
  /// Compress Rotated Text to Line Height
  #[sdk(attr(qname = "w:vertCompress"))]
  pub vertical_compress: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RunPropertiesChange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPrChange")]
pub struct RunPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub previous_run_properties: std::boxed::Box<PreviousRunProperties>,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct RunProperties {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  #[sdk(
        choice(
            child(variant = RunStyle, qname = "w:rStyle"),
            child(variant = RunFonts, qname = "w:rFonts"),
            child(variant = Bold, qname = "w:b"),
            child(variant = BoldComplexScript, qname = "w:bCs"),
            child(variant = Italic, qname = "w:i"),
            child(variant = ItalicComplexScript, qname = "w:iCs"),
            child(variant = Caps, qname = "w:caps"),
            child(variant = SmallCaps, qname = "w:smallCaps"),
            child(variant = Strike, qname = "w:strike"),
            child(variant = DoubleStrike, qname = "w:dstrike"),
            child(variant = Outline, qname = "w:outline"),
            child(variant = Shadow, qname = "w:shadow"),
            child(variant = Emboss, qname = "w:emboss"),
            child(variant = Imprint, qname = "w:imprint"),
            child(variant = NoProof, qname = "w:noProof"),
            child(variant = SnapToGrid, qname = "w:snapToGrid"),
            child(variant = Vanish, qname = "w:vanish"),
            child(variant = WebHidden, qname = "w:webHidden"),
            child(variant = Color, qname = "w:color"),
            child(variant = Spacing, qname = "w:spacing"),
            child(variant = CharacterScale, qname = "w:w"),
            child(variant = Kern, qname = "w:kern"),
            child(variant = Position, qname = "w:position"),
            child(variant = FontSize, qname = "w:sz"),
            child(variant = FontSizeComplexScript, qname = "w:szCs"),
            child(variant = Highlight, qname = "w:highlight"),
            child(variant = Underline, qname = "w:u"),
            child(variant = TextEffect, qname = "w:effect"),
            child(variant = Border, qname = "w:bdr"),
            child(variant = Shading, qname = "w:shd"),
            child(variant = FitText, qname = "w:fitText"),
            child(variant = VerticalTextAlignment, qname = "w:vertAlign"),
            child(variant = RightToLeftText, qname = "w:rtl"),
            child(variant = ComplexScript, qname = "w:cs"),
            child(variant = Emphasis, qname = "w:em"),
            child(variant = Languages, qname = "w:lang"),
            child(variant = EastAsianLayout, qname = "w:eastAsianLayout"),
            child(variant = SpecVanish, qname = "w:specVanish")
        )
    )]
  pub run_properties_choice: Vec<RunPropertiesChoice>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub glow: Option<std::boxed::Box<crate::schemas::w14::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub shadow14: Option<std::boxed::Box<crate::schemas::w14::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub reflection: Option<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub text_outline_effect: Option<std::boxed::Box<crate::schemas::w14::TextOutlineEffect>>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub fill_text_effect: Option<std::boxed::Box<crate::schemas::w14::FillTextEffect>>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub scene3_d: Option<std::boxed::Box<crate::schemas::w14::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub properties3_d: Option<std::boxed::Box<crate::schemas::w14::Properties3D>>,
  /// Defines the Ligatures Class.
  #[sdk(child(qname = "w14:ligatures"))]
  pub ligatures: Option<crate::schemas::w14::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(qname = "w14:numForm"))]
  pub numbering_format: Option<crate::schemas::w14::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(qname = "w14:numSpacing"))]
  pub number_spacing: Option<crate::schemas::w14::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(qname = "w14:stylisticSets"))]
  pub stylistic_sets: Option<crate::schemas::w14::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(qname = "w14:cntxtAlts"))]
  pub contextual_alternatives: Option<crate::schemas::w14::ContextualAlternatives>,
  /// Defines the RunPropertiesChange Class.
  #[sdk(child(qname = "w:rPrChange"))]
  pub run_properties_change: Option<std::boxed::Box<RunPropertiesChange>>,
}
/// Defines the InsertedMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ins")]
pub struct InsertedMathControl {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = RunProperties, qname = "w:rPr"),
            child(variant = DeletedMathControl, qname = "w:del")
        )
    )]
  pub inserted_math_control_choice: Option<InsertedMathControlChoice>,
}
/// Defines the DeletedMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:del")]
pub struct DeletedMathControl {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Run Properties.
  #[sdk(child(qname = "w:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
}
/// Defines the MoveFromMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveFrom")]
pub struct MoveFromMathControl {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = RunProperties, qname = "w:rPr"),
            child(variant = InsertedMathControl, qname = "w:ins"),
            child(variant = DeletedMathControl, qname = "w:del")
        )
    )]
  pub move_from_math_control_choice: Option<MoveFromMathControlChoice>,
}
/// Defines the MoveToMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveTo")]
pub struct MoveToMathControl {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = RunProperties, qname = "w:rPr"),
            child(variant = InsertedMathControl, qname = "w:ins"),
            child(variant = DeletedMathControl, qname = "w:del")
        )
    )]
  pub move_to_math_control_choice: Option<MoveToMathControlChoice>,
}
/// Defines the CustomXmlRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXml")]
pub struct CustomXmlRuby {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub custom_xml_ruby_choice: Vec<CustomXmlRubyChoice>,
}
/// Defines the SimpleFieldRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fldSimple")]
pub struct SimpleFieldRuby {
  /// instr
  #[sdk(attr(qname = "w:instr"))]
  pub instruction: crate::simple_type::StringValue,
  /// fldLock
  #[sdk(attr(qname = "w:fldLock"))]
  pub field_lock: Option<crate::simple_type::OnOffValue>,
  /// dirty
  #[sdk(attr(qname = "w:dirty"))]
  pub dirty: Option<crate::simple_type::OnOffValue>,
  /// Custom Field Data.
  #[sdk(child(qname = "w:fldData"))]
  pub field_data: Option<FieldData>,
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub simple_field_ruby_choice: Vec<SimpleFieldRubyChoice>,
}
/// Defines the HyperlinkRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hyperlink")]
pub struct HyperlinkRuby {
  /// tgtFrame
  #[sdk(attr(qname = "w:tgtFrame"))]
  #[sdk(string_length(max = 255u32))]
  pub target_frame: Option<crate::simple_type::StringValue>,
  /// tooltip
  #[sdk(attr(qname = "w:tooltip"))]
  #[sdk(string_length(max = 260u32))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// docLocation
  #[sdk(attr(qname = "w:docLocation"))]
  #[sdk(string_length(max = 255u32))]
  pub doc_location: Option<crate::simple_type::StringValue>,
  /// history
  #[sdk(attr(qname = "w:history"))]
  pub history: Option<crate::simple_type::OnOffValue>,
  /// anchor
  #[sdk(attr(qname = "w:anchor"))]
  #[sdk(string_length(max = 255u32))]
  pub anchor: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub hyperlink_ruby_choice: Vec<HyperlinkRubyChoice>,
}
/// Phonetic Guide Text Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:r")]
pub struct Run {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Revision Identifier for Run Properties
  #[sdk(attr(qname = "w:rsidRPr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_run_properties: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Run Deletion
  #[sdk(attr(qname = "w:rsidDel"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_run_deletion: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Run
  #[sdk(attr(qname = "w:rsidR"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_run_addition: Option<crate::simple_type::HexBinaryValue>,
  /// Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
  #[sdk(
        choice(
            child(variant = Break, qname = "w:br"),
            child(variant = Text, qname = "w:t"),
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
            child(variant = SymbolCharExt, qname = "w16se:sym"),
            any
        )
    )]
  pub run_choice: Vec<RunChoice>,
}
/// Defines the SdtRunRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdt")]
pub struct SdtRunRuby {
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Defines the SdtContentRunRuby Class.
  #[sdk(child(qname = "w:sdtContent"))]
  pub sdt_content_run_ruby: Option<SdtContentRunRuby>,
  #[sdk(
        choice(
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
            )
        )
    )]
  pub sdt_run_ruby_choice: Vec<SdtRunRubyChoice>,
}
/// Defines the ProofError Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:proofErr")]
pub struct ProofError {
  /// Proofing Error Anchor Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: ProofingErrorValues,
}
/// Defines the PermStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:permStart")]
pub struct PermStart {
  /// edGrp
  #[sdk(attr(qname = "w:edGrp"))]
  pub editor_group: Option<RangePermissionEditingGroupValues>,
  /// ed
  #[sdk(attr(qname = "w:ed"))]
  pub ed: Option<crate::simple_type::StringValue>,
  /// colFirst
  #[sdk(attr(qname = "w:colFirst"))]
  #[sdk(number_range(range = 0..))]
  pub column_first: Option<crate::simple_type::Int32Value>,
  /// colLast
  #[sdk(attr(qname = "w:colLast"))]
  #[sdk(number_range(range = 0..))]
  pub column_last: Option<crate::simple_type::Int32Value>,
  /// Annotation ID
  #[sdk(attr(qname = "w:id"))]
  pub id: crate::simple_type::Int32Value,
  /// Annotation Displaced By Custom XML Markup
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
}
/// Defines the PermEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:permEnd")]
pub struct PermEnd {
  /// Annotation ID
  #[sdk(attr(qname = "w:id"))]
  pub id: crate::simple_type::Int32Value,
  /// Annotation Displaced By Custom XML Markup
  #[sdk(attr(qname = "w:displacedByCustomXml"))]
  pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
}
/// Inserted Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ins")]
pub struct InsertedRun {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir")
        )
    )]
  pub inserted_run_choice: Vec<InsertedRunChoice>,
}
/// Deleted Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:del")]
pub struct DeletedRun {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir")
        )
    )]
  pub deleted_run_choice: Vec<DeletedRunChoice>,
}
/// Move Source Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveFrom")]
pub struct MoveFromRun {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir")
        )
    )]
  pub move_from_run_choice: Vec<MoveFromRunChoice>,
}
/// Move Destination Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:moveTo")]
pub struct MoveToRun {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir")
        )
    )]
  pub move_to_run_choice: Vec<MoveToRunChoice>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:contentPart")]
pub struct ContentPart {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the SdtRun Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdt")]
pub struct SdtRun {
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Inline-Level Structured Document Tag Content
  #[sdk(child(qname = "w:sdtContent"))]
  pub sdt_content_run: Option<SdtContentRun>,
  #[sdk(
        choice(
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
            )
        )
    )]
  pub sdt_run_choice: Vec<SdtRunChoice>,
}
/// Defines the CustomXmlBlock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXml")]
pub struct CustomXmlBlock {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(
        choice(
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub custom_xml_block_choice: Vec<CustomXmlBlockChoice>,
}
/// Defines the SdtBlock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdt")]
pub struct SdtBlock {
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Block-Level Structured Document Tag Content
  #[sdk(child(qname = "w:sdtContent"))]
  pub sdt_content_block: Option<SdtContentBlock>,
  #[sdk(
        choice(
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
            )
        )
    )]
  pub sdt_block_choice: Vec<SdtBlockChoice>,
}
/// Defines the Paragraph Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:p")]
pub struct Paragraph {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Revision Identifier for Paragraph Glyph Formatting
  #[sdk(attr(qname = "w:rsidRPr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_paragraph_mark_revision: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Paragraph
  #[sdk(attr(qname = "w:rsidR"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_paragraph_addition: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Paragraph Deletion
  #[sdk(attr(qname = "w:rsidDel"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_paragraph_deletion: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Paragraph Properties
  #[sdk(attr(qname = "w:rsidP"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_paragraph_properties: Option<crate::simple_type::HexBinaryValue>,
  /// Default Revision Identifier for Runs
  #[sdk(attr(qname = "w:rsidRDefault"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_run_addition_default: Option<crate::simple_type::HexBinaryValue>,
  /// paraId
  #[sdk(attr(qname = "w14:paraId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub paragraph_id: Option<crate::simple_type::HexBinaryValue>,
  /// textId
  #[sdk(attr(qname = "w14:textId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub text_id: Option<crate::simple_type::HexBinaryValue>,
  /// noSpellErr
  #[sdk(attr(qname = "w14:noSpellErr"))]
  pub no_spell_error: Option<crate::simple_type::OnOffValue>,
  /// Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc"),
            any
        )
    )]
  pub paragraph_choice: Vec<ParagraphChoice>,
}
/// Defines the Table Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tbl")]
pub struct Table {
  #[sdk(
        choice(
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
            )
        )
    )]
  pub table_choice1: Vec<TableChoice>,
  /// Table Properties.
  #[sdk(child(qname = "w:tblPr"))]
  pub table_properties: Option<std::boxed::Box<TableProperties>>,
  /// Table Grid.
  #[sdk(child(qname = "w:tblGrid"))]
  pub table_grid: Option<std::boxed::Box<TableGrid>>,
  #[sdk(
        choice(
            child(variant = TableRow, qname = "w:tr"),
            child(variant = CustomXmlRow, qname = "w:customXml"),
            child(variant = SdtRow, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub table_choice2: Vec<TableChoice2>,
}
/// Table Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tr")]
pub struct TableRow {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Revision Identifier for Table Row Glyph Formatting
  #[sdk(attr(qname = "w:rsidRPr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_table_row_mark_revision: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Table Row
  #[sdk(attr(qname = "w:rsidR"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_table_row_addition: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Table Row Deletion
  #[sdk(attr(qname = "w:rsidDel"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_table_row_deletion: Option<crate::simple_type::HexBinaryValue>,
  /// Revision Identifier for Table Row Properties
  #[sdk(attr(qname = "w:rsidTr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_table_row_properties: Option<crate::simple_type::HexBinaryValue>,
  /// paraId
  #[sdk(attr(qname = "w14:paraId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub paragraph_id: Option<crate::simple_type::HexBinaryValue>,
  /// textId
  #[sdk(attr(qname = "w14:textId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub text_id: Option<crate::simple_type::HexBinaryValue>,
  /// Table-Level Property Exceptions
  #[sdk(child(qname = "w:tblPrEx"))]
  pub table_property_exceptions: Option<std::boxed::Box<TablePropertyExceptions>>,
  /// Table Row Properties
  #[sdk(child(qname = "w:trPr"))]
  pub table_row_properties: Option<std::boxed::Box<TableRowProperties>>,
  #[sdk(
        choice(
            child(variant = TableCell, qname = "w:tc"),
            child(variant = CustomXmlCell, qname = "w:customXml"),
            child(variant = SdtCell, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub table_row_choice: Vec<TableRowChoice>,
}
/// Row-Level Custom XML Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXml")]
pub struct CustomXmlRow {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(
        choice(
            child(variant = TableRow, qname = "w:tr"),
            child(variant = CustomXmlRow, qname = "w:customXml"),
            child(variant = SdtRow, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub custom_xml_row_choice: Vec<CustomXmlRowChoice>,
}
/// Row-Level Structured Document Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdt")]
pub struct SdtRow {
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Row-Level Structured Document Tag Content
  #[sdk(child(qname = "w:sdtContent"))]
  pub sdt_content_row: Option<SdtContentRow>,
  #[sdk(
        choice(
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
            )
        )
    )]
  pub sdt_row_choice: Vec<SdtRowChoice>,
}
/// Table Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tc")]
pub struct TableCell {
  /// Table Cell Properties
  #[sdk(child(qname = "w:tcPr"))]
  pub table_cell_properties: Option<std::boxed::Box<TableCellProperties>>,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = Break, qname = "w:br")
        )
    )]
  pub table_cell_choice: Vec<TableCellChoice>,
}
/// Cell-Level Custom XML Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXml")]
pub struct CustomXmlCell {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(
        choice(
            child(variant = TableCell, qname = "w:tc"),
            child(variant = CustomXmlCell, qname = "w:customXml"),
            child(variant = SdtCell, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub custom_xml_cell_choice: Vec<CustomXmlCellChoice>,
}
/// Cell-Level Structured Document Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdt")]
pub struct SdtCell {
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Cell-Level Structured Document Tag Content
  #[sdk(child(qname = "w:sdtContent"))]
  pub sdt_content_cell: Option<SdtContentCell>,
  #[sdk(
        choice(
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
            )
        )
    )]
  pub sdt_cell_choice: Vec<SdtCellChoice>,
}
/// Defines the CustomXmlRun Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXml")]
pub struct CustomXmlRun {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub custom_xml_run_choice: Vec<CustomXmlRunChoice>,
}
/// Defines the SimpleField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fldSimple")]
pub struct SimpleField {
  /// Field Codes
  #[sdk(attr(qname = "w:instr"))]
  pub instruction: crate::simple_type::StringValue,
  /// Field Should Not Be Recalculated
  #[sdk(attr(qname = "w:fldLock"))]
  pub field_lock: Option<crate::simple_type::OnOffValue>,
  /// Field Result Invalidated
  #[sdk(attr(qname = "w:dirty"))]
  pub dirty: Option<crate::simple_type::OnOffValue>,
  /// Custom Field Data
  #[sdk(child(qname = "w:fldData"))]
  pub field_data: Option<FieldData>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub simple_field_choice: Vec<SimpleFieldChoice>,
}
/// Defines the Hyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hyperlink")]
pub struct Hyperlink {
  /// Hyperlink Target Frame
  #[sdk(attr(qname = "w:tgtFrame"))]
  #[sdk(string_length(max = 255u32))]
  pub target_frame: Option<crate::simple_type::StringValue>,
  /// Associated String
  #[sdk(attr(qname = "w:tooltip"))]
  #[sdk(string_length(max = 260u32))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// Location in Target Document
  #[sdk(attr(qname = "w:docLocation"))]
  #[sdk(string_length(max = 255u32))]
  pub doc_location: Option<crate::simple_type::StringValue>,
  /// Add To Viewed Hyperlinks
  #[sdk(attr(qname = "w:history"))]
  pub history: Option<crate::simple_type::OnOffValue>,
  /// Hyperlink Anchor
  #[sdk(attr(qname = "w:anchor"))]
  #[sdk(string_length(max = 255u32))]
  pub anchor: Option<crate::simple_type::StringValue>,
  /// Hyperlink Target
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub hyperlink_choice: Vec<HyperlinkChoice>,
}
/// Defines the BidirectionalOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bdo")]
pub struct BidirectionalOverride {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub w_val: Option<DirectionValues>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub bidirectional_override_choice: Vec<BidirectionalOverrideChoice>,
}
/// Defines the BidirectionalEmbedding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dir")]
pub struct BidirectionalEmbedding {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub w_val: Option<DirectionValues>,
  #[sdk(
        choice(
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r"),
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub bidirectional_embedding_choice: Vec<BidirectionalEmbeddingChoice>,
}
/// Anchor for Subdocument Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:subDoc")]
pub struct SubDocumentReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the PrinterSettingsReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:printerSettings")]
pub struct PrinterSettingsReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// ODSO Data Source File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:src")]
pub struct SourceReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Reference to Inclusion/Exclusion Data for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:recipientData")]
pub struct RecipientDataReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Data Source File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dataSource")]
pub struct DataSourceReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Header Definition File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:headerSource")]
pub struct HeaderSource {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Source File for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sourceFileName")]
pub struct SourceFileReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MovieReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:movie")]
pub struct MovieReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Attached Document Template.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:attachedTemplate")]
pub struct AttachedTemplate {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ConditionalFormatStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cnfStyle")]
pub struct ConditionalFormatStyle {
  /// Conditional Formatting Bit Mask
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "[01]*"))]
  #[sdk(string_length(min = 12u32, max = 12u32))]
  pub val: Option<crate::simple_type::StringValue>,
  /// firstRow
  #[sdk(attr(qname = "w:firstRow"))]
  pub first_row: Option<crate::simple_type::OnOffValue>,
  /// lastRow
  #[sdk(attr(qname = "w:lastRow"))]
  pub last_row: Option<crate::simple_type::OnOffValue>,
  /// firstColumn
  #[sdk(attr(qname = "w:firstColumn"))]
  pub first_column: Option<crate::simple_type::OnOffValue>,
  /// lastColumn
  #[sdk(attr(qname = "w:lastColumn"))]
  pub last_column: Option<crate::simple_type::OnOffValue>,
  /// oddVBand
  #[sdk(attr(qname = "w:oddVBand"))]
  pub odd_vertical_band: Option<crate::simple_type::OnOffValue>,
  /// evenVBand
  #[sdk(attr(qname = "w:evenVBand"))]
  pub even_vertical_band: Option<crate::simple_type::OnOffValue>,
  /// oddHBand
  #[sdk(attr(qname = "w:oddHBand"))]
  pub odd_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// evenHBand
  #[sdk(attr(qname = "w:evenHBand"))]
  pub even_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// firstRowFirstColumn
  #[sdk(attr(qname = "w:firstRowFirstColumn"))]
  pub first_row_first_column: Option<crate::simple_type::OnOffValue>,
  /// firstRowLastColumn
  #[sdk(attr(qname = "w:firstRowLastColumn"))]
  pub first_row_last_column: Option<crate::simple_type::OnOffValue>,
  /// lastRowFirstColumn
  #[sdk(attr(qname = "w:lastRowFirstColumn"))]
  pub last_row_first_column: Option<crate::simple_type::OnOffValue>,
  /// lastRowLastColumn
  #[sdk(attr(qname = "w:lastRowLastColumn"))]
  pub last_row_last_column: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TableCellWidth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcW")]
pub struct TableCellWidth {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthBeforeTableRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wBefore")]
pub struct WidthBeforeTableRow {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthAfterTableRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:wAfter")]
pub struct WidthAfterTableRow {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableCellSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblCellSpacing")]
pub struct TableCellSpacing {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableWidth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblW")]
pub struct TableWidth {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Top Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:top")]
pub struct TopMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the StartMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:start")]
pub struct StartMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Bottom Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bottom")]
pub struct BottomMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the EndMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:end")]
pub struct EndMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Left Margin Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:left")]
pub struct LeftMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Right Margin Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:right")]
pub struct RightMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the HorizontalMerge Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hMerge")]
pub struct HorizontalMerge {
  /// Horizontal Merge Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<MergedCellValues>,
}
/// Defines the VerticalMerge Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:vMerge")]
pub struct VerticalMerge {
  /// Vertical Merge Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<MergedCellValues>,
}
/// Defines the TableCellBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcBorders")]
pub struct TableCellBorders {
  /// Table Cell Top Border
  #[sdk(child(qname = "w:top"))]
  pub top_border: Option<TopBorder>,
  /// Table Cell Left Border
  #[sdk(child(qname = "w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Defines the StartBorder Class.
  #[sdk(child(qname = "w:start"))]
  pub start_border: Option<StartBorder>,
  /// Table Cell Bottom Border
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Table Cell Right Border
  #[sdk(child(qname = "w:right"))]
  pub right_border: Option<RightBorder>,
  /// Defines the EndBorder Class.
  #[sdk(child(qname = "w:end"))]
  pub end_border: Option<EndBorder>,
  /// Table Cell Inside Horizontal Edges Border
  #[sdk(child(qname = "w:insideH"))]
  pub inside_horizontal_border: Option<InsideHorizontalBorder>,
  /// Table Cell Inside Vertical Edges Border
  #[sdk(child(qname = "w:insideV"))]
  pub inside_vertical_border: Option<InsideVerticalBorder>,
  /// Table Cell Top Left to Bottom Right Diagonal Border
  #[sdk(child(qname = "w:tl2br"))]
  pub top_left_to_bottom_right_cell_border: Option<TopLeftToBottomRightCellBorder>,
  /// Table Cell Top Right to Bottom Left Diagonal Border
  #[sdk(child(qname = "w:tr2bl"))]
  pub top_right_to_bottom_left_cell_border: Option<TopRightToBottomLeftCellBorder>,
}
/// Defines the NoWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noWrap")]
pub struct NoWrap {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TableCellFitText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcFitText")]
pub struct TableCellFitText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the HideMark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hideMark")]
pub struct HideMark {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the CantSplit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cantSplit")]
pub struct CantSplit {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TableHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblHeader")]
pub struct TableHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the BiDiVisual Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bidiVisual")]
pub struct BiDiVisual {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Frame Cannot Be Resized.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noResizeAllowed")]
pub struct NoResizeAllowed {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Maintain Link to Existing File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:linkedToFile")]
pub struct LinkedToFile {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Frameset Splitters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noBorder")]
pub struct NoBorder {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Frameset Splitter Border Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:flatBorders")]
pub struct FlatBorders {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Merge User Formatting Into Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoRedefine")]
pub struct AutoRedefine {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Hide Style From User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hidden")]
pub struct StyleHidden {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Hide Style From Main User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:semiHidden")]
pub struct SemiHidden {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Semi-Hidden Property When Style Is Used.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:unhideWhenUsed")]
pub struct UnhideWhenUsed {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Primary Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:qFormat")]
pub struct PrimaryStyle {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Style Cannot Be Applied.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:locked")]
pub struct Locked {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// E-Mail Message Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:personal")]
pub struct Personal {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// E-Mail Message Composition Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:personalCompose")]
pub struct PersonalCompose {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// E-Mail Message Reply Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:personalReply")]
pub struct PersonalReply {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TableCellMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcMar")]
pub struct TableCellMargin {
  /// Table Cell Top Margin Exception
  #[sdk(child(qname = "w:top"))]
  pub top_margin: Option<TopMargin>,
  /// Table Cell Left Margin Exception
  #[sdk(child(qname = "w:left"))]
  pub left_margin: Option<TableCellLeftMargin>,
  /// Defines the StartMargin Class.
  #[sdk(child(qname = "w:start"))]
  pub start_margin: Option<StartMargin>,
  /// Table Cell Bottom Margin Exception
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_margin: Option<BottomMargin>,
  /// Table Cell Right Margin Exception
  #[sdk(child(qname = "w:right"))]
  pub right_margin: Option<TableCellRightMargin>,
  /// Defines the EndMargin Class.
  #[sdk(child(qname = "w:end"))]
  pub end_margin: Option<EndMargin>,
}
/// Defines the TableCellVerticalAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:vAlign")]
pub struct TableCellVerticalAlignment {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: TableVerticalAlignmentValues,
}
/// Defines the DivId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:divId")]
pub struct DivId {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "1",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-1",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableRowHeight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trHeight")]
pub struct TableRowHeight {
  /// Table Row Height
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: Option<crate::simple_type::TwipsMeasureValue>,
  /// Table Row Height Type
  #[sdk(attr(qname = "w:hRule"))]
  pub height_type: Option<HeightRuleValues>,
}
/// Defines the TableJustification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:jc")]
pub struct TableJustification {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: TableRowAlignmentValues,
}
/// Defines the TablePositionProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblpPr")]
pub struct TablePositionProperties {
  /// Distance From Left of Table to Text
  #[sdk(attr(qname = "w:leftFromText"))]
  #[sdk(number_range(range = 0..))]
  pub left_from_text: Option<crate::simple_type::TwipsMeasureValue>,
  /// (Distance From Right of Table to Text
  #[sdk(attr(qname = "w:rightFromText"))]
  #[sdk(number_range(range = 0..))]
  pub right_from_text: Option<crate::simple_type::TwipsMeasureValue>,
  /// Distance From Top of Table to Text
  #[sdk(attr(qname = "w:topFromText"))]
  #[sdk(number_range(range = 0..))]
  pub top_from_text: Option<crate::simple_type::TwipsMeasureValue>,
  /// Distance From Bottom of Table to Text
  #[sdk(attr(qname = "w:bottomFromText"))]
  #[sdk(number_range(range = 0..))]
  pub bottom_from_text: Option<crate::simple_type::TwipsMeasureValue>,
  /// Table Vertical Anchor
  #[sdk(attr(qname = "w:vertAnchor"))]
  pub vertical_anchor: Option<VerticalAnchorValues>,
  /// Table Horizontal Anchor
  #[sdk(attr(qname = "w:horzAnchor"))]
  pub horizontal_anchor: Option<HorizontalAnchorValues>,
  /// Relative Horizontal Alignment From Anchor
  #[sdk(attr(empty_as_none, qname = "w:tblpXSpec"))]
  pub table_position_x_alignment: Option<HorizontalAlignmentValues>,
  /// Absolute Horizontal Distance From Anchor
  #[sdk(attr(qname = "w:tblpX"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub table_position_x: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Relative Vertical Alignment from Anchor
  #[sdk(attr(empty_as_none, qname = "w:tblpYSpec"))]
  pub table_position_y_alignment: Option<VerticalAlignmentValues>,
  /// Absolute Vertical Distance From Anchor
  #[sdk(attr(qname = "w:tblpY"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub table_position_y: Option<crate::simple_type::SignedTwipsMeasureValue>,
}
/// Defines the TableOverlap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblOverlap")]
pub struct TableOverlap {
  /// Floating Table Overlap Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: TableOverlapValues,
}
/// Defines the TableStyleRowBandSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblStyleRowBandSize")]
pub struct TableStyleRowBandSize {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TableStyleColumnBandSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblStyleColBandSize")]
pub struct TableStyleColumnBandSize {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TableIndentation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblInd")]
pub struct TableIndentation {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblBorders")]
pub struct TableBorders {
  /// Table Top Border
  #[sdk(child(qname = "w:top"))]
  pub top_border: Option<TopBorder>,
  /// Table Left Border
  #[sdk(child(qname = "w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Defines the StartBorder Class.
  #[sdk(child(qname = "w:start"))]
  pub start_border: Option<StartBorder>,
  /// Table Bottom Border
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Table Right Border
  #[sdk(child(qname = "w:right"))]
  pub right_border: Option<RightBorder>,
  /// Defines the EndBorder Class.
  #[sdk(child(qname = "w:end"))]
  pub end_border: Option<EndBorder>,
  /// Table Inside Horizontal Edges Border
  #[sdk(child(qname = "w:insideH"))]
  pub inside_horizontal_border: Option<InsideHorizontalBorder>,
  /// Table Inside Vertical Edges Border
  #[sdk(child(qname = "w:insideV"))]
  pub inside_vertical_border: Option<InsideVerticalBorder>,
}
/// Defines the TableLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblLayout")]
pub struct TableLayout {
  /// Table Layout Setting
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableLayoutValues>,
}
/// Defines the TableCellMarginDefault Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblCellMar")]
pub struct TableCellMarginDefault {
  /// Table Cell Top Margin Default
  #[sdk(child(qname = "w:top"))]
  pub top_margin: Option<TopMargin>,
  /// Table Cell Left Margin Default
  #[sdk(child(qname = "w:left"))]
  pub table_cell_left_margin: Option<TableCellLeftMargin>,
  /// Defines the StartMargin Class.
  #[sdk(child(qname = "w:start"))]
  pub start_margin: Option<StartMargin>,
  /// Table Cell Bottom Margin Default
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_margin: Option<BottomMargin>,
  /// Table Cell Right Margin Default
  #[sdk(child(qname = "w:right"))]
  pub table_cell_right_margin: Option<TableCellRightMargin>,
  /// Defines the EndMargin Class.
  #[sdk(child(qname = "w:end"))]
  pub end_margin: Option<EndMargin>,
}
/// Footnote and Endnote Numbering Starting Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numStart")]
pub struct NumberingStart {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Footnote and Endnote Numbering Restart Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numRestart")]
pub struct NumberingRestart {
  /// Automatic Numbering Restart Value
  #[sdk(attr(qname = "w:val"))]
  pub val: RestartNumberValues,
}
/// Defines the AltChunk Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:altChunk")]
pub struct AltChunk {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// External Content Import Properties
  #[sdk(child(qname = "w:altChunkPr"))]
  pub alt_chunk_properties: Option<std::boxed::Box<AltChunkProperties>>,
}
/// Defines the TableLook Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblLook")]
pub struct TableLook {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub w_val: Option<crate::simple_type::HexBinaryValue>,
  /// firstRow
  #[sdk(attr(qname = "w:firstRow"))]
  pub first_row: Option<crate::simple_type::OnOffValue>,
  /// lastRow
  #[sdk(attr(qname = "w:lastRow"))]
  pub last_row: Option<crate::simple_type::OnOffValue>,
  /// firstColumn
  #[sdk(attr(qname = "w:firstColumn"))]
  pub first_column: Option<crate::simple_type::OnOffValue>,
  /// lastColumn
  #[sdk(attr(qname = "w:lastColumn"))]
  pub last_column: Option<crate::simple_type::OnOffValue>,
  /// noHBand
  #[sdk(attr(qname = "w:noHBand"))]
  pub no_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// noVBand
  #[sdk(attr(qname = "w:noVBand"))]
  pub no_vertical_band: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FootnoteProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnotePr")]
pub struct FootnoteProperties {
  /// Footnote Placement
  #[sdk(child(qname = "w:pos"))]
  pub footnote_position: Option<FootnotePosition>,
  /// Footnote Numbering Format
  #[sdk(child(qname = "w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
}
/// Defines the EndnoteProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnotePr")]
pub struct EndnoteProperties {
  /// Endnote Placement
  #[sdk(child(qname = "w:pos"))]
  pub endnote_position: Option<EndnotePosition>,
  /// Endnote Numbering Format
  #[sdk(child(qname = "w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
}
/// Defines the SectionType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:type")]
pub struct SectionType {
  /// Section Type Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: SectionMarkValues,
}
/// Defines the PageSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pgSz")]
pub struct PageSize {
  /// Page Width
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub width: Option<crate::simple_type::TwipsMeasureValue>,
  /// Page Height
  #[sdk(attr(qname = "w:h"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub height: Option<crate::simple_type::TwipsMeasureValue>,
  /// Page Orientation
  #[sdk(attr(qname = "w:orient"))]
  pub orient: Option<PageOrientationValues>,
  /// Printer Paper Code
  #[sdk(attr(qname = "w:code"))]
  pub code: Option<crate::simple_type::UInt16Value>,
}
/// Defines the PageMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pgMar")]
pub struct PageMargin {
  /// Top Margin Spacing
  #[sdk(attr(qname = "w:top"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub top: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Right Margin Spacing
  #[sdk(attr(qname = "w:right"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub right: Option<crate::simple_type::TwipsMeasureValue>,
  /// Page Bottom Spacing
  #[sdk(attr(qname = "w:bottom"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub bottom: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Left Margin Spacing
  #[sdk(attr(qname = "w:left"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub left: Option<crate::simple_type::TwipsMeasureValue>,
  /// Spacing to Top of Header
  #[sdk(attr(qname = "w:header"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub header: Option<crate::simple_type::TwipsMeasureValue>,
  /// Spacing to Bottom of Footer
  #[sdk(attr(qname = "w:footer"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub footer: Option<crate::simple_type::TwipsMeasureValue>,
  /// Page Gutter Spacing
  #[sdk(attr(qname = "w:gutter"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub gutter: Option<crate::simple_type::TwipsMeasureValue>,
}
/// Defines the PaperSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:paperSrc")]
pub struct PaperSource {
  /// First Page Printer Tray Code
  #[sdk(attr(qname = "w:first"))]
  pub first: Option<crate::simple_type::UInt16Value>,
  /// Non-First Page Printer Tray Code
  #[sdk(attr(qname = "w:other"))]
  pub other: Option<crate::simple_type::UInt16Value>,
}
/// Defines the PageBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pgBorders")]
pub struct PageBorders {
  /// Z-Ordering of Page Border
  #[sdk(attr(qname = "w:zOrder"))]
  pub z_order: Option<PageBorderZOrderValues>,
  /// Pages to Display Page Borders
  #[sdk(attr(qname = "w:display"))]
  pub display: Option<PageBorderDisplayValues>,
  /// Page Border Positioning
  #[sdk(attr(qname = "w:offsetFrom"))]
  pub offset_from: Option<PageBorderOffsetValues>,
  /// Top Border
  #[sdk(child(qname = "w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Border
  #[sdk(child(qname = "w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Bottom Border
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Border
  #[sdk(child(qname = "w:right"))]
  pub right_border: Option<RightBorder>,
}
/// Defines the LineNumberType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lnNumType")]
pub struct LineNumberType {
  /// Line Number Increments to Display
  #[sdk(attr(qname = "w:countBy"))]
  #[sdk(number_range(range = 1..= 100))]
  pub count_by: Option<crate::simple_type::Int16Value>,
  /// Line Numbering Starting Value
  #[sdk(attr(qname = "w:start"))]
  #[sdk(number_range(range = 0..))]
  pub start: Option<crate::simple_type::Int16Value>,
  /// Distance Between Text and Line Numbering
  #[sdk(attr(qname = "w:distance"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub distance: Option<crate::simple_type::TwipsMeasureValue>,
  /// Line Numbering Restart Setting
  #[sdk(attr(qname = "w:restart"))]
  pub restart: Option<LineNumberRestartValues>,
}
/// Defines the PageNumberType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pgNumType")]
pub struct PageNumberType {
  /// Page Number Format
  #[sdk(attr(qname = "w:fmt"))]
  pub format: Option<NumberFormatValues>,
  /// Starting Page Number
  #[sdk(attr(qname = "w:start"))]
  #[sdk(number_range(range = 0..))]
  pub start: Option<crate::simple_type::Int32Value>,
  /// Chapter Heading Style
  #[sdk(attr(qname = "w:chapStyle"))]
  pub chapter_style: Option<crate::simple_type::ByteValue>,
  /// Chapter Separator Character
  #[sdk(attr(qname = "w:chapSep"))]
  pub chapter_separator: Option<ChapterSeparatorValues>,
}
/// Defines the Columns Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:cols")]
pub struct Columns {
  /// Equal Column Widths
  #[sdk(attr(qname = "w:equalWidth"))]
  pub equal_width: Option<crate::simple_type::OnOffValue>,
  /// Spacing Between Equal Width Columns
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub space: Option<crate::simple_type::TwipsMeasureValue>,
  /// Number of Equal Width Columns
  #[sdk(attr(qname = "w:num"))]
  #[sdk(number_range(range = 1..= 45))]
  pub column_count: Option<crate::simple_type::Int16Value>,
  /// Draw Line Between Columns
  #[sdk(attr(qname = "w:sep"))]
  pub separator: Option<crate::simple_type::OnOffValue>,
  /// Single Column Definition.
  #[sdk(child(qname = "w:col"))]
  pub column: Vec<Column>,
}
/// Defines the VerticalTextAlignmentOnPage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:vAlign")]
pub struct VerticalTextAlignmentOnPage {
  /// Vertical Alignment Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalJustificationValues,
}
/// Defines the DocGrid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docGrid")]
pub struct DocGrid {
  /// Document Grid Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<DocGridValues>,
  /// Document Grid Line Pitch
  #[sdk(attr(qname = "w:linePitch"))]
  pub line_pitch: Option<crate::simple_type::Int32Value>,
  /// Document Grid Character Pitch
  #[sdk(attr(qname = "w:charSpace"))]
  pub character_space: Option<crate::simple_type::Int32ZeroOnOverflowValue>,
}
/// Inclusion/Exclusion Data for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:recipients")]
pub struct Recipients {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Data About Single Data Source Record.
  #[sdk(child(qname = "w:recipientData"))]
  pub recipient_data: Vec<RecipientData>,
}
/// Rich Text Box Content Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:txbxContent")]
pub struct TextBoxContent {
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub text_box_content_choice: Vec<TextBoxContentChoice>,
}
/// Comments Collection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:comments")]
pub struct Comments {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Comment Content.
  #[sdk(child(qname = "w:comment"))]
  pub comment: Vec<Comment>,
}
/// Document Footnotes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnotes")]
pub struct Footnotes {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Footnote Content.
  #[sdk(child(qname = "w:footnote"))]
  pub footnote: Vec<Footnote>,
}
/// Document Endnotes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnotes")]
pub struct Endnotes {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Endnote Content.
  #[sdk(child(qname = "w:endnote"))]
  pub endnote: Vec<Endnote>,
}
/// Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hdr")]
pub struct Header {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub header_choice: Vec<HeaderChoice>,
}
/// Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ftr")]
pub struct Footer {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub footer_choice: Vec<FooterChoice>,
}
/// Document Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:settings")]
pub struct Settings {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Write Protection
  #[sdk(child(qname = "w:writeProtection"))]
  pub write_protection: Option<WriteProtection>,
  /// Document View Setting
  #[sdk(child(qname = "w:view"))]
  pub view: Option<View>,
  /// Magnification Setting
  #[sdk(child(qname = "w:zoom"))]
  pub zoom: Option<Zoom>,
  /// Remove Personal Information from Document Properties
  #[sdk(child(qname = "w:removePersonalInformation"))]
  pub remove_personal_information: Option<RemovePersonalInformation>,
  /// Remove Date and Time from Annotations
  #[sdk(child(qname = "w:removeDateAndTime"))]
  pub remove_date_and_time: Option<RemoveDateAndTime>,
  /// Do Not Display Visual Boundary For Header/Footer or Between Pages
  #[sdk(child(qname = "w:doNotDisplayPageBoundaries"))]
  pub do_not_display_page_boundaries: Option<DoNotDisplayPageBoundaries>,
  /// Display Background Objects When Displaying Document
  #[sdk(child(qname = "w:displayBackgroundShape"))]
  pub display_background_shape: Option<DisplayBackgroundShape>,
  /// Print PostScript Codes With Document Text
  #[sdk(child(qname = "w:printPostScriptOverText"))]
  pub print_post_script_over_text: Option<PrintPostScriptOverText>,
  /// Print Fractional Character Widths
  #[sdk(child(qname = "w:printFractionalCharacterWidth"))]
  pub print_fractional_character_width: Option<PrintFractionalCharacterWidth>,
  /// Only Print Form Field Content
  #[sdk(child(qname = "w:printFormsData"))]
  pub print_forms_data: Option<PrintFormsData>,
  /// Embed TrueType Fonts
  #[sdk(child(qname = "w:embedTrueTypeFonts"))]
  pub embed_true_type_fonts: Option<EmbedTrueTypeFonts>,
  /// Embed Common System Fonts
  #[sdk(child(qname = "w:embedSystemFonts"))]
  pub embed_system_fonts: Option<EmbedSystemFonts>,
  /// Subset Fonts When Embedding
  #[sdk(child(qname = "w:saveSubsetFonts"))]
  pub save_subset_fonts: Option<SaveSubsetFonts>,
  /// Only Save Form Field Content
  #[sdk(child(qname = "w:saveFormsData"))]
  pub save_forms_data: Option<SaveFormsData>,
  /// Mirror Page Margins
  #[sdk(child(qname = "w:mirrorMargins"))]
  pub mirror_margins: Option<MirrorMargins>,
  /// Align Paragraph and Table Borders with Page Border
  #[sdk(child(qname = "w:alignBordersAndEdges"))]
  pub align_border_and_edges: Option<AlignBorderAndEdges>,
  /// Page Border Excludes Header
  #[sdk(child(qname = "w:bordersDoNotSurroundHeader"))]
  pub borders_do_not_surround_header: Option<BordersDoNotSurroundHeader>,
  /// Page Border Excludes Footer
  #[sdk(child(qname = "w:bordersDoNotSurroundFooter"))]
  pub borders_do_not_surround_footer: Option<BordersDoNotSurroundFooter>,
  /// Position Gutter At Top of Page
  #[sdk(child(qname = "w:gutterAtTop"))]
  pub gutter_at_top: Option<GutterAtTop>,
  /// Do Not Display Visual Indication of Spelling Errors
  #[sdk(child(qname = "w:hideSpellingErrors"))]
  pub hide_spelling_errors: Option<HideSpellingErrors>,
  /// Do Not Display Visual Indication of Grammatical Errors
  #[sdk(child(qname = "w:hideGrammaticalErrors"))]
  pub hide_grammatical_errors: Option<HideGrammaticalErrors>,
  /// Grammar Checking Settings.
  #[sdk(child(qname = "w:activeWritingStyle"))]
  pub active_writing_style: Vec<ActiveWritingStyle>,
  /// Spelling and Grammatical Checking State.
  #[sdk(child(qname = "w:proofState"))]
  pub proof_state: Option<ProofState>,
  /// Structured Document Tag Placeholder Text Should be Resaved.
  #[sdk(child(qname = "w:formsDesign"))]
  pub forms_design: Option<FormsDesign>,
  /// Attached Document Template.
  #[sdk(child(qname = "w:attachedTemplate"))]
  pub attached_template: Option<AttachedTemplate>,
  /// Automatically Update Styles From Document Template.
  #[sdk(child(qname = "w:linkStyles"))]
  pub link_styles: Option<LinkStyles>,
  /// Suggested Filtering for List of Document Styles.
  #[sdk(child(qname = "w:stylePaneFormatFilter"))]
  pub style_pane_format_filter: Option<StylePaneFormatFilter>,
  /// Suggested Sorting for List of Document Styles.
  #[sdk(child(qname = "w:stylePaneSortMethod"))]
  pub style_pane_sort_methods: Option<StylePaneSortMethods>,
  /// Document Classification.
  #[sdk(child(qname = "w:documentType"))]
  pub document_type: Option<DocumentType>,
  /// Mail Merge Settings.
  #[sdk(child(qname = "w:mailMerge"))]
  pub mail_merge: Option<std::boxed::Box<MailMerge>>,
  /// Visibility of Annotation Types.
  #[sdk(child(qname = "w:revisionView"))]
  pub revision_view: Option<RevisionView>,
  /// Track Revisions to Document.
  #[sdk(child(qname = "w:trackRevisions"))]
  pub track_revisions: Option<TrackRevisions>,
  /// Do Not Use Move Syntax When Tracking Revisions.
  #[sdk(child(qname = "w:doNotTrackMoves"))]
  pub do_not_track_moves: Option<DoNotTrackMoves>,
  /// Do Not Track Formatting Revisions When Tracking Revisions.
  #[sdk(child(qname = "w:doNotTrackFormatting"))]
  pub do_not_track_formatting: Option<DoNotTrackFormatting>,
  /// Document Editing Restrictions.
  #[sdk(child(qname = "w:documentProtection"))]
  pub document_protection: Option<DocumentProtection>,
  /// Allow Automatic Formatting to Override Formatting Protection Settings.
  #[sdk(child(qname = "w:autoFormatOverride"))]
  pub auto_format_override: Option<AutoFormatOverride>,
  /// Prevent Modification of Themes Part.
  #[sdk(child(qname = "w:styleLockTheme"))]
  pub style_lock_themes_part: Option<StyleLockThemesPart>,
  /// Prevent Replacement of Styles Part.
  #[sdk(child(qname = "w:styleLockQFSet"))]
  pub style_lock_styles_part: Option<StyleLockStylesPart>,
  /// Distance Between Automatic Tab Stops.
  #[sdk(child(qname = "w:defaultTabStop"))]
  pub default_tab_stop: Option<DefaultTabStop>,
  /// Automatically Hyphenate Document Contents When Displayed.
  #[sdk(child(qname = "w:autoHyphenation"))]
  pub auto_hyphenation: Option<AutoHyphenation>,
  /// Maximum Number of Consecutively Hyphenated Lines.
  #[sdk(child(qname = "w:consecutiveHyphenLimit"))]
  pub consecutive_hyphen_limit: Option<ConsecutiveHyphenLimit>,
  /// Hyphenation Zone.
  #[sdk(child(qname = "w:hyphenationZone"))]
  pub hyphenation_zone: Option<HyphenationZone>,
  /// Do Not Hyphenate Words in ALL CAPITAL LETTERS.
  #[sdk(child(qname = "w:doNotHyphenateCaps"))]
  pub do_not_hyphenate_caps: Option<DoNotHyphenateCaps>,
  /// Show E-Mail Message Header.
  #[sdk(child(qname = "w:showEnvelope"))]
  pub show_envelope: Option<ShowEnvelope>,
  /// Percentage of Document to Use When Generating Summary.
  #[sdk(child(qname = "w:summaryLength"))]
  pub summary_length: Option<SummaryLength>,
  /// Paragraph Style Applied to Automatically Generated Paragraphs.
  #[sdk(child(qname = "w:clickAndTypeStyle"))]
  pub click_and_type_style: Option<ClickAndTypeStyle>,
  /// Default Table Style for Newly Inserted Tables.
  #[sdk(child(qname = "w:defaultTableStyle"))]
  pub default_table_style: Option<DefaultTableStyle>,
  /// Different Even/Odd Page Headers and Footers.
  #[sdk(child(qname = "w:evenAndOddHeaders"))]
  pub even_and_odd_headers: Option<EvenAndOddHeaders>,
  /// Reverse Book Fold Printing.
  #[sdk(child(qname = "w:bookFoldRevPrinting"))]
  pub book_fold_reverse_printing: Option<BookFoldReversePrinting>,
  /// Book Fold Printing.
  #[sdk(child(qname = "w:bookFoldPrinting"))]
  pub book_fold_printing: Option<BookFoldPrinting>,
  /// Number of Pages Per Booklet.
  #[sdk(child(qname = "w:bookFoldPrintingSheets"))]
  pub book_fold_printing_sheets: Option<BookFoldPrintingSheets>,
  /// Drawing Grid Horizontal Grid Unit Size.
  #[sdk(child(qname = "w:drawingGridHorizontalSpacing"))]
  pub drawing_grid_horizontal_spacing: Option<DrawingGridHorizontalSpacing>,
  /// Drawing Grid Vertical Grid Unit Size.
  #[sdk(child(qname = "w:drawingGridVerticalSpacing"))]
  pub drawing_grid_vertical_spacing: Option<DrawingGridVerticalSpacing>,
  /// Distance between Horizontal Gridlines.
  #[sdk(child(qname = "w:displayHorizontalDrawingGridEvery"))]
  pub display_horizontal_drawing_grid: Option<DisplayHorizontalDrawingGrid>,
  /// Distance between Vertical Gridlines.
  #[sdk(child(qname = "w:displayVerticalDrawingGridEvery"))]
  pub display_vertical_drawing_grid: Option<DisplayVerticalDrawingGrid>,
  /// Do Not Use Margins for Drawing Grid Origin.
  #[sdk(child(qname = "w:doNotUseMarginsForDrawingGridOrigin"))]
  pub do_not_use_margins_for_drawing_grid_origin: Option<DoNotUseMarginsForDrawingGridOrigin>,
  /// Drawing Grid Horizontal Origin Point.
  #[sdk(child(qname = "w:drawingGridHorizontalOrigin"))]
  pub drawing_grid_horizontal_origin: Option<DrawingGridHorizontalOrigin>,
  /// Drawing Grid Vertical Origin Point.
  #[sdk(child(qname = "w:drawingGridVerticalOrigin"))]
  pub drawing_grid_vertical_origin: Option<DrawingGridVerticalOrigin>,
  /// Do Not Show Visual Indicator For Form Fields.
  #[sdk(child(qname = "w:doNotShadeFormData"))]
  pub do_not_shade_form_data: Option<DoNotShadeFormData>,
  /// Never Kern Punctuation Characters.
  #[sdk(child(qname = "w:noPunctuationKerning"))]
  pub no_punctuation_kerning: Option<NoPunctuationKerning>,
  /// Character-Level Whitespace Compression.
  #[sdk(child(qname = "w:characterSpacingControl"))]
  pub character_spacing_control: Option<CharacterSpacingControl>,
  /// Print Two Pages Per Sheet.
  #[sdk(child(qname = "w:printTwoOnOne"))]
  pub print_two_on_one: Option<PrintTwoOnOne>,
  /// Use Strict Kinsoku Rules for Japanese Text.
  #[sdk(child(qname = "w:strictFirstAndLastChars"))]
  pub strict_first_and_last_chars: Option<StrictFirstAndLastChars>,
  /// Custom Set of Characters Which Cannot End a Line.
  #[sdk(child(qname = "w:noLineBreaksAfter"))]
  pub no_line_breaks_after_kinsoku: Option<NoLineBreaksAfterKinsoku>,
  /// Custom Set Of Characters Which Cannot Begin A Line.
  #[sdk(child(qname = "w:noLineBreaksBefore"))]
  pub no_line_breaks_before_kinsoku: Option<NoLineBreaksBeforeKinsoku>,
  /// Generate Thumbnail For Document On Save.
  #[sdk(child(qname = "w:savePreviewPicture"))]
  pub save_preview_picture: Option<SavePreviewPicture>,
  /// Do Not Validate Custom XML Markup Against Schemas.
  #[sdk(child(qname = "w:doNotValidateAgainstSchema"))]
  pub do_not_validate_against_schema: Option<DoNotValidateAgainstSchema>,
  /// Allow Saving Document As XML File When Custom XML Markup Is Invalid.
  #[sdk(child(qname = "w:saveInvalidXml"))]
  pub save_invalid_xml: Option<SaveInvalidXml>,
  /// Ignore Mixed Content When Validating Custom XML Markup.
  #[sdk(child(qname = "w:ignoreMixedContent"))]
  pub ignore_mixed_content: Option<IgnoreMixedContent>,
  /// Use Custom XML Element Names as Default Placeholder Text.
  #[sdk(child(qname = "w:alwaysShowPlaceholderText"))]
  pub always_show_placeholder_text: Option<AlwaysShowPlaceholderText>,
  /// Do Not Show Visual Indicator For Invalid Custom XML Markup.
  #[sdk(child(qname = "w:doNotDemarcateInvalidXml"))]
  pub do_not_demarcate_invalid_xml: Option<DoNotDemarcateInvalidXml>,
  /// Only Save Custom XML Markup.
  #[sdk(child(qname = "w:saveXmlDataOnly"))]
  pub save_xml_data_only: Option<SaveXmlDataOnly>,
  /// Save Document as XML File through Custom XSL Transform.
  #[sdk(child(qname = "w:useXSLTWhenSaving"))]
  pub use_xslt_when_saving: Option<UseXsltWhenSaving>,
  /// Custom XSL Transform To Use When Saving As XML File.
  #[sdk(child(qname = "w:saveThroughXslt"))]
  pub save_through_xslt: Option<SaveThroughXslt>,
  /// Show Visual Indicators for Custom XML Markup Start/End Locations.
  #[sdk(child(qname = "w:showXMLTags"))]
  pub show_xml_tags: Option<ShowXmlTags>,
  /// Do Not Mark Custom XML Elements With No Namespace As Invalid.
  #[sdk(child(qname = "w:alwaysMergeEmptyNamespace"))]
  pub always_merge_empty_namespace: Option<AlwaysMergeEmptyNamespace>,
  /// Automatically Recalculate Fields on Open.
  #[sdk(child(qname = "w:updateFields"))]
  pub update_fields_on_open: Option<UpdateFieldsOnOpen>,
  /// Default Properties for VML Objects in Header and Footer.
  #[sdk(child(qname = "w:hdrShapeDefaults"))]
  pub header_shape_defaults: Option<HeaderShapeDefaults>,
  /// Document-Wide Footnote Properties.
  #[sdk(child(qname = "w:footnotePr"))]
  pub footnote_document_wide_properties: Option<std::boxed::Box<FootnoteDocumentWideProperties>>,
  /// Document-Wide Endnote Properties.
  #[sdk(child(qname = "w:endnotePr"))]
  pub endnote_document_wide_properties: Option<std::boxed::Box<EndnoteDocumentWideProperties>>,
  /// Compatibility Settings.
  #[sdk(child(qname = "w:compat"))]
  pub compatibility: Vec<Compatibility>,
  /// Document Variables.
  #[sdk(child(qname = "w:docVars"))]
  pub document_variables: Option<DocumentVariables>,
  /// Listing of All Revision Save ID Values.
  #[sdk(child(qname = "w:rsids"))]
  pub rsids: Option<std::boxed::Box<Rsids>>,
  /// Math Properties.
  #[sdk(child(qname = "m:mathPr"))]
  pub math_properties: Option<std::boxed::Box<crate::schemas::m::MathProperties>>,
  /// Disable Features Incompatible With Earlier Word Processing Formats.
  #[sdk(child(qname = "w:uiCompat97To2003"))]
  pub ui_compatible_with97_to2003: Option<UiCompatibleWith97To2003>,
  /// Attached Custom XML Schema.
  #[sdk(child(qname = "w:attachedSchema"))]
  pub attached_schema: Vec<AttachedSchema>,
  /// Theme Font Languages.
  #[sdk(child(qname = "w:themeFontLang"))]
  pub theme_font_languages: Option<ThemeFontLanguages>,
  /// Theme Color Mappings.
  #[sdk(child(qname = "w:clrSchemeMapping"))]
  pub color_scheme_mapping: Option<ColorSchemeMapping>,
  /// Do Not Include Content in Text Boxes, Footnotes, and Endnotes in Document Statistics.
  #[sdk(child(qname = "w:doNotIncludeSubdocsInStats"))]
  pub do_not_include_subdocs_in_stats: Option<DoNotIncludeSubdocsInStats>,
  /// Do Not Automatically Compress Images.
  #[sdk(child(qname = "w:doNotAutoCompressPictures"))]
  pub do_not_auto_compress_pictures: Option<DoNotAutoCompressPictures>,
  /// Upgrade Document on Open.
  #[sdk(empty_child(qname = "w:forceUpgrade"))]
  pub force_upgrade: Option<()>,
  /// Caption Settings.
  #[sdk(child(qname = "w:captions"))]
  pub captions: Option<std::boxed::Box<Captions>>,
  /// Freeze Document Layout.
  #[sdk(child(qname = "w:readModeInkLockDown"))]
  pub read_mode_ink_lock_down: Option<ReadModeInkLockDown>,
  /// Embedded Custom XML Schema Supplementary Data.
  #[sdk(child(qname = "sl:schemaLibrary"))]
  pub schema_library: Option<crate::schemas::sl::SchemaLibrary>,
  /// Default Properties for VML Objects in Main Document.
  #[sdk(child(qname = "w:shapeDefaults"))]
  pub shape_defaults: Option<ShapeDefaults>,
  /// Radix Point for Field Code Evaluation.
  #[sdk(child(qname = "w:decimalSymbol"))]
  pub decimal_symbol: Option<DecimalSymbol>,
  /// List Separator for Field Code Evaluation.
  #[sdk(child(qname = "w:listSeparator"))]
  pub list_separator: Option<ListSeparator>,
  /// Defines the DocumentId Class.
  #[sdk(child(qname = "w14:docId"))]
  pub document_id: Option<crate::schemas::w14::DocumentId>,
  /// Defines the DiscardImageEditingData Class.
  #[sdk(child(qname = "w14:discardImageEditingData"))]
  pub discard_image_editing_data: Option<crate::schemas::w14::DiscardImageEditingData>,
  /// Defines the DefaultImageDpi Class.
  #[sdk(child(qname = "w14:defaultImageDpi"))]
  pub default_image_dpi: Option<crate::schemas::w14::DefaultImageDpi>,
  /// Defines the ConflictMode Class.
  #[sdk(child(qname = "w14:conflictMode"))]
  pub conflict_mode: Option<crate::schemas::w14::ConflictMode>,
  /// Defines the ChartTrackingRefBased Class.
  #[sdk(child(qname = "w15:chartTrackingRefBased"))]
  pub chart_tracking_ref_based: Option<crate::schemas::w15::ChartTrackingRefBased>,
  /// Defines the PersistentDocumentId Class.
  #[sdk(child(qname = "w15:docId"))]
  pub persistent_document_id: Option<crate::schemas::w15::PersistentDocumentId>,
}
/// Web Page Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:webSettings")]
pub struct WebSettings {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Nested Frameset Definition.
  #[sdk(child(qname = "w:frameset"))]
  pub frameset: Option<std::boxed::Box<Frameset>>,
  /// Defines the Divs Class.
  #[sdk(child(qname = "w:divs"))]
  pub divs: Option<Divs>,
  /// Defines the WebPageEncoding Class.
  #[sdk(child(qname = "w:encoding"))]
  pub web_page_encoding: Option<WebPageEncoding>,
  /// Defines the OptimizeForBrowser Class.
  #[sdk(child(qname = "w:optimizeForBrowser"))]
  pub optimize_for_browser: Option<OptimizeForBrowser>,
  /// Defines the RelyOnVML Class.
  #[sdk(child(qname = "w:relyOnVML"))]
  pub rely_on_vml: Option<RelyOnVml>,
  /// Defines the AllowPNG Class.
  #[sdk(child(qname = "w:allowPNG"))]
  pub allow_png: Option<AllowPng>,
  /// Defines the DoNotRelyOnCSS Class.
  #[sdk(child(qname = "w:doNotRelyOnCSS"))]
  pub do_not_rely_on_css: Option<DoNotRelyOnCss>,
  /// Defines the DoNotSaveAsSingleFile Class.
  #[sdk(child(qname = "w:doNotSaveAsSingleFile"))]
  pub do_not_save_as_single_file: Option<DoNotSaveAsSingleFile>,
  /// Defines the DoNotOrganizeInFolder Class.
  #[sdk(child(qname = "w:doNotOrganizeInFolder"))]
  pub do_not_organize_in_folder: Option<DoNotOrganizeInFolder>,
  /// Defines the DoNotUseLongFileNames Class.
  #[sdk(child(qname = "w:doNotUseLongFileNames"))]
  pub do_not_use_long_file_names: Option<DoNotUseLongFileNames>,
  /// Defines the PixelsPerInch Class.
  #[sdk(child(qname = "w:pixelsPerInch"))]
  pub pixels_per_inch: Option<PixelsPerInch>,
  /// Defines the TargetScreenSize Class.
  #[sdk(child(qname = "w:targetScreenSz"))]
  pub target_screen_size: Option<TargetScreenSize>,
}
/// Font Table Root Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fonts")]
pub struct Fonts {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  #[sdk(choice(child(variant = Font, qname = "w:font"), any))]
  pub xml_children: Vec<FontsChoice>,
}
/// Numbering Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numbering")]
pub struct Numbering {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Picture Numbering Symbol Definition.
  #[sdk(child(qname = "w:numPicBullet"))]
  pub numbering_picture_bullet: Vec<NumberingPictureBullet>,
  /// Abstract Numbering Definition.
  #[sdk(child(qname = "w:abstractNum"))]
  pub abstract_num: Vec<AbstractNum>,
  /// Numbering Definition Instance.
  #[sdk(child(qname = "w:num"))]
  pub numbering_instance: Vec<NumberingInstance>,
  /// Last Reviewed Abstract Numbering Definition.
  #[sdk(child(qname = "w:numIdMacAtCleanup"))]
  pub numbering_id_mac_at_cleanup: Option<NumberingIdMacAtCleanup>,
}
/// Style Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:styles")]
pub struct Styles {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Document Default Paragraph and Run Properties
  #[sdk(child(qname = "w:docDefaults"))]
  pub doc_defaults: Option<std::boxed::Box<DocDefaults>>,
  /// Latent Style Information
  #[sdk(child(qname = "w:latentStyles"))]
  pub latent_styles: Option<LatentStyles>,
  /// Style Definition.
  #[sdk(child(qname = "w:style"))]
  pub style: Vec<Style>,
}
/// Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(canonical_namespace_prefix("ns2:r"), qname = "w:document")]
pub struct Document {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// conformance
  #[sdk(attr(qname = "w:conformance"))]
  pub w_conformance: Option<DocumentConformance>,
  /// Document Background
  #[sdk(child(qname = "w:background"))]
  pub document_background: Option<std::boxed::Box<DocumentBackground>>,
  /// Defines the Body Class.
  #[sdk(child(qname = "w:body"))]
  pub body: Option<std::boxed::Box<Body>>,
}
/// Glossary Document Root Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:glossaryDocument")]
pub struct GlossaryDocument {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Document Background
  #[sdk(child(qname = "w:background"))]
  pub document_background: Option<std::boxed::Box<DocumentBackground>>,
  /// List of Glossary Document Entries
  #[sdk(child(qname = "w:docParts"))]
  pub doc_parts: Option<DocParts>,
}
/// Previous Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPrEx")]
pub struct PreviousTablePropertyExceptions {
  /// Preferred Table Width Exception
  #[sdk(child(qname = "w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Table Alignment Exception
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Table Cell Spacing Exception
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Table Indent from Leading Margin Exception
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Table Borders Exceptions
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Table Shading Exception
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Table Layout Exception
  #[sdk(child(qname = "w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Table Cell Margin Exceptions
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Table Style Conditional Formatting Settings Exception
  #[sdk(child(qname = "w:tblLook"))]
  pub table_look: Option<TableLook>,
}
/// Previous Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcPr")]
pub struct PreviousTableCellProperties {
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Defines the TableCellWidth Class.
  #[sdk(child(qname = "w:tcW"))]
  pub table_cell_width: Option<TableCellWidth>,
  /// Defines the GridSpan Class.
  #[sdk(child(qname = "w:gridSpan"))]
  pub grid_span: Option<GridSpan>,
  /// Defines the HorizontalMerge Class.
  #[sdk(child(qname = "w:hMerge"))]
  pub horizontal_merge: Option<HorizontalMerge>,
  /// Defines the VerticalMerge Class.
  #[sdk(child(qname = "w:vMerge"))]
  pub vertical_merge: Option<VerticalMerge>,
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TableCellFitText Class.
  #[sdk(child(qname = "w:tcFitText"))]
  pub table_cell_fit_text: Option<TableCellFitText>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
  /// Defines the HideMark Class.
  #[sdk(child(qname = "w:hideMark"))]
  pub hide_mark: Option<HideMark>,
  #[sdk(
        choice(
            child(variant = CellInsertion, qname = "w:cellIns"),
            child(variant = CellDeletion, qname = "w:cellDel"),
            child(variant = CellMerge, qname = "w:cellMerge")
        )
    )]
  pub previous_table_cell_properties_choice: Option<PreviousTableCellPropertiesChoice>,
}
/// Previous Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trPr")]
pub struct PreviousTableRowProperties {
  #[sdk(
        choice(
            child(variant = ConditionalFormatStyle, qname = "w:cnfStyle"),
            child(variant = DivId, qname = "w:divId"),
            child(variant = GridBefore, qname = "w:gridBefore"),
            child(variant = GridAfter, qname = "w:gridAfter"),
            child(variant = WidthBeforeTableRow, qname = "w:wBefore"),
            child(variant = WidthAfterTableRow, qname = "w:wAfter"),
            child(variant = TableRowHeight, qname = "w:trHeight"),
            child(variant = Hidden, qname = "w:hidden"),
            child(variant = CantSplit, qname = "w:cantSplit"),
            child(variant = TableHeader, qname = "w:tblHeader"),
            child(variant = TableCellSpacing, qname = "w:tblCellSpacing"),
            child(variant = TableJustification, qname = "w:jc")
        )
    )]
  pub previous_table_row_properties_choice: Vec<PreviousTableRowPropertiesChoice>,
}
/// Previous Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPr")]
pub struct PreviousTableProperties {
  /// Defines the TableStyle Class.
  #[sdk(child(qname = "w:tblStyle"))]
  pub table_style: Option<TableStyle>,
  /// Defines the TablePositionProperties Class.
  #[sdk(child(qname = "w:tblpPr"))]
  pub table_position_properties: Option<TablePositionProperties>,
  /// Defines the TableOverlap Class.
  #[sdk(child(qname = "w:tblOverlap"))]
  pub table_overlap: Option<TableOverlap>,
  /// Defines the BiDiVisual Class.
  #[sdk(child(qname = "w:bidiVisual"))]
  pub bi_di_visual: Option<BiDiVisual>,
  /// Defines the TableStyleRowBandSize Class.
  #[sdk(child(qname = "w:tblStyleRowBandSize"))]
  pub table_style_row_band_size: Option<TableStyleRowBandSize>,
  /// Defines the TableStyleColumnBandSize Class.
  #[sdk(child(qname = "w:tblStyleColBandSize"))]
  pub table_style_column_band_size: Option<TableStyleColumnBandSize>,
  /// Defines the TableWidth Class.
  #[sdk(child(qname = "w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableLayout Class.
  #[sdk(child(qname = "w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Defines the TableLook Class.
  #[sdk(child(qname = "w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Defines the TableCaption Class.
  #[sdk(child(qname = "w:tblCaption"))]
  pub table_caption: Option<TableCaption>,
  /// Defines the TableDescription Class.
  #[sdk(child(qname = "w:tblDescription"))]
  pub table_description: Option<TableDescription>,
}
/// Previous Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sectPr")]
pub struct PreviousSectionProperties {
  /// Physical Section Mark Character Revision ID
  #[sdk(attr(qname = "w:rsidRPr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_r_pr: Option<crate::simple_type::HexBinaryValue>,
  /// Section Deletion Revision ID
  #[sdk(attr(qname = "w:rsidDel"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_del: Option<crate::simple_type::HexBinaryValue>,
  /// Section Addition Revision ID
  #[sdk(attr(qname = "w:rsidR"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_r: Option<crate::simple_type::HexBinaryValue>,
  /// Section Properties Revision ID
  #[sdk(attr(qname = "w:rsidSect"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_sect: Option<crate::simple_type::HexBinaryValue>,
  /// Defines the FootnoteProperties Class.
  #[sdk(child(qname = "w:footnotePr"))]
  pub footnote_properties: Option<std::boxed::Box<FootnoteProperties>>,
  /// Defines the EndnoteProperties Class.
  #[sdk(child(qname = "w:endnotePr"))]
  pub endnote_properties: Option<std::boxed::Box<EndnoteProperties>>,
  /// Defines the SectionType Class.
  #[sdk(child(qname = "w:type"))]
  pub section_type: Option<SectionType>,
  /// Defines the PageSize Class.
  #[sdk(child(qname = "w:pgSz"))]
  pub page_size: Option<PageSize>,
  /// Defines the PageMargin Class.
  #[sdk(child(qname = "w:pgMar"))]
  pub page_margin: Option<PageMargin>,
  /// Defines the PaperSource Class.
  #[sdk(child(qname = "w:paperSrc"))]
  pub paper_source: Option<PaperSource>,
  /// Defines the PageBorders Class.
  #[sdk(child(qname = "w:pgBorders"))]
  pub page_borders: Option<std::boxed::Box<PageBorders>>,
  /// Defines the LineNumberType Class.
  #[sdk(child(qname = "w:lnNumType"))]
  pub line_number_type: Option<LineNumberType>,
  /// Defines the PageNumberType Class.
  #[sdk(child(qname = "w:pgNumType"))]
  pub page_number_type: Option<PageNumberType>,
  /// Defines the Columns Class.
  #[sdk(child(qname = "w:cols"))]
  pub columns: Option<Columns>,
  /// Defines the FormProtection Class.
  #[sdk(child(qname = "w:formProt"))]
  pub form_protection: Option<FormProtection>,
  /// Defines the VerticalTextAlignmentOnPage Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub vertical_text_alignment_on_page: Option<VerticalTextAlignmentOnPage>,
  /// Defines the NoEndnote Class.
  #[sdk(child(qname = "w:noEndnote"))]
  pub no_endnote: Option<NoEndnote>,
  /// Defines the TitlePage Class.
  #[sdk(child(qname = "w:titlePg"))]
  pub title_page: Option<TitlePage>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the GutterOnRight Class.
  #[sdk(child(qname = "w:rtlGutter"))]
  pub gutter_on_right: Option<GutterOnRight>,
  /// Defines the DocGrid Class.
  #[sdk(child(qname = "w:docGrid"))]
  pub doc_grid: Option<DocGrid>,
  /// Defines the PrinterSettingsReference Class.
  #[sdk(child(qname = "w:printerSettings"))]
  pub printer_settings_reference: Option<PrinterSettingsReference>,
  /// Defines the FootnoteColumns Class.
  #[sdk(child(qname = "w15:footnoteColumns"))]
  pub footnote_columns: Option<crate::schemas::w15::FootnoteColumns>,
}
/// Previous Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPr")]
pub struct ParagraphPropertiesExtended {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Defines the DivId Class.
  #[sdk(child(qname = "w:divId"))]
  pub div_id: Option<DivId>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
}
/// Previous Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct PreviousRunProperties {
  #[sdk(
        choice(
            child(variant = RunStyle, qname = "w:rStyle"),
            child(variant = RunFonts, qname = "w:rFonts"),
            child(variant = Bold, qname = "w:b"),
            child(variant = BoldComplexScript, qname = "w:bCs"),
            child(variant = Italic, qname = "w:i"),
            child(variant = ItalicComplexScript, qname = "w:iCs"),
            child(variant = Caps, qname = "w:caps"),
            child(variant = SmallCaps, qname = "w:smallCaps"),
            child(variant = Strike, qname = "w:strike"),
            child(variant = DoubleStrike, qname = "w:dstrike"),
            child(variant = Outline, qname = "w:outline"),
            child(variant = Shadow, qname = "w:shadow"),
            child(variant = Emboss, qname = "w:emboss"),
            child(variant = Imprint, qname = "w:imprint"),
            child(variant = NoProof, qname = "w:noProof"),
            child(variant = SnapToGrid, qname = "w:snapToGrid"),
            child(variant = Vanish, qname = "w:vanish"),
            child(variant = WebHidden, qname = "w:webHidden"),
            child(variant = Color, qname = "w:color"),
            child(variant = Spacing, qname = "w:spacing"),
            child(variant = CharacterScale, qname = "w:w"),
            child(variant = Kern, qname = "w:kern"),
            child(variant = Position, qname = "w:position"),
            child(variant = FontSize, qname = "w:sz"),
            child(variant = FontSizeComplexScript, qname = "w:szCs"),
            child(variant = Highlight, qname = "w:highlight"),
            child(variant = Underline, qname = "w:u"),
            child(variant = TextEffect, qname = "w:effect"),
            child(variant = Border, qname = "w:bdr"),
            child(variant = Shading, qname = "w:shd"),
            child(variant = FitText, qname = "w:fitText"),
            child(variant = VerticalTextAlignment, qname = "w:vertAlign"),
            child(variant = RightToLeftText, qname = "w:rtl"),
            child(variant = ComplexScript, qname = "w:cs"),
            child(variant = Emphasis, qname = "w:em"),
            child(variant = Languages, qname = "w:lang"),
            child(variant = EastAsianLayout, qname = "w:eastAsianLayout"),
            child(variant = SpecVanish, qname = "w:specVanish")
        )
    )]
  pub run_properties_choice: Vec<PreviousRunPropertiesChoice>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub glow: Vec<crate::schemas::w14::Glow>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub shadow: Vec<crate::schemas::w14::Shadow>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub reflection: Vec<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub text_outline_effect: Vec<crate::schemas::w14::TextOutlineEffect>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub fill_text_effect: Vec<crate::schemas::w14::FillTextEffect>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub scene3_d: Vec<crate::schemas::w14::Scene3D>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub properties3_d: Vec<crate::schemas::w14::Properties3D>,
  /// Defines the Ligatures Class.
  #[sdk(child(qname = "w14:ligatures"))]
  pub ligatures: Vec<crate::schemas::w14::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(qname = "w14:numForm"))]
  pub numbering_format: Vec<crate::schemas::w14::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(qname = "w14:numSpacing"))]
  pub number_spacing: Vec<crate::schemas::w14::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(qname = "w14:stylisticSets"))]
  pub stylistic_sets: Vec<crate::schemas::w14::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(qname = "w14:cntxtAlts"))]
  pub contextual_alternatives: Vec<crate::schemas::w14::ContextualAlternatives>,
}
/// Previous Run Properties for the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct PreviousParagraphMarkRunProperties {
  /// Inserted Paragraph
  #[sdk(child(qname = "w:ins"))]
  pub inserted: Option<Inserted>,
  /// Deleted Paragraph
  #[sdk(child(qname = "w:del"))]
  pub deleted: Option<Deleted>,
  /// Move Source Paragraph
  #[sdk(child(qname = "w:moveFrom"))]
  pub move_from: Option<MoveFrom>,
  /// Move Destination Paragraph
  #[sdk(child(qname = "w:moveTo"))]
  pub move_to: Option<MoveTo>,
  #[sdk(
        choice(
            child(variant = ConflictInsertion, qname = "w14:conflictIns"),
            child(variant = ConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub previous_paragraph_mark_run_properties_choice1:
    Option<PreviousParagraphMarkRunPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = RunStyle, qname = "w:rStyle"),
            child(variant = RunFonts, qname = "w:rFonts"),
            child(variant = Bold, qname = "w:b"),
            child(variant = BoldComplexScript, qname = "w:bCs"),
            child(variant = Italic, qname = "w:i"),
            child(variant = ItalicComplexScript, qname = "w:iCs"),
            child(variant = Caps, qname = "w:caps"),
            child(variant = SmallCaps, qname = "w:smallCaps"),
            child(variant = Strike, qname = "w:strike"),
            child(variant = DoubleStrike, qname = "w:dstrike"),
            child(variant = Outline, qname = "w:outline"),
            child(variant = Shadow, qname = "w:shadow"),
            child(variant = Emboss, qname = "w:emboss"),
            child(variant = Imprint, qname = "w:imprint"),
            child(variant = NoProof, qname = "w:noProof"),
            child(variant = SnapToGrid, qname = "w:snapToGrid"),
            child(variant = Vanish, qname = "w:vanish"),
            child(variant = WebHidden, qname = "w:webHidden"),
            child(variant = Color, qname = "w:color"),
            child(variant = Spacing, qname = "w:spacing"),
            child(variant = CharacterScale, qname = "w:w"),
            child(variant = Kern, qname = "w:kern"),
            child(variant = Position, qname = "w:position"),
            child(variant = FontSize, qname = "w:sz"),
            child(variant = FontSizeComplexScript, qname = "w:szCs"),
            child(variant = Highlight, qname = "w:highlight"),
            child(variant = Underline, qname = "w:u"),
            child(variant = TextEffect, qname = "w:effect"),
            child(variant = Border, qname = "w:bdr"),
            child(variant = Shading, qname = "w:shd"),
            child(variant = FitText, qname = "w:fitText"),
            child(variant = VerticalTextAlignment, qname = "w:vertAlign"),
            child(variant = RightToLeftText, qname = "w:rtl"),
            child(variant = ComplexScript, qname = "w:cs"),
            child(variant = Emphasis, qname = "w:em"),
            child(variant = Languages, qname = "w:lang"),
            child(variant = EastAsianLayout, qname = "w:eastAsianLayout"),
            child(variant = SpecVanish, qname = "w:specVanish")
        )
    )]
  pub previous_paragraph_mark_run_properties_choice2:
    Vec<PreviousParagraphMarkRunPropertiesChoice2>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub glow: Vec<crate::schemas::w14::Glow>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub shadow: Vec<crate::schemas::w14::Shadow>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub reflection: Vec<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub text_outline_effect: Vec<crate::schemas::w14::TextOutlineEffect>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub fill_text_effect: Vec<crate::schemas::w14::FillTextEffect>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub scene3_d: Vec<crate::schemas::w14::Scene3D>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub properties3_d: Vec<crate::schemas::w14::Properties3D>,
  /// Defines the Ligatures Class.
  #[sdk(child(qname = "w14:ligatures"))]
  pub ligatures: Vec<crate::schemas::w14::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(qname = "w14:numForm"))]
  pub numbering_format: Vec<crate::schemas::w14::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(qname = "w14:numSpacing"))]
  pub number_spacing: Vec<crate::schemas::w14::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(qname = "w14:stylisticSets"))]
  pub stylistic_sets: Vec<crate::schemas::w14::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(qname = "w14:cntxtAlts"))]
  pub contextual_alternatives: Vec<crate::schemas::w14::ContextualAlternatives>,
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "w:oMath"))]
  pub office_math: Vec<OfficeMath>,
}
/// Numbering Level Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ilvl")]
pub struct NumberingLevelReference {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 255))]
  pub val: crate::simple_type::Int32Value,
}
/// Numbering Definition Instance Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numId")]
pub struct NumberingId {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Starting Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:start")]
pub struct StartNumberingValue {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the AbstractNumId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:abstractNumId")]
pub struct AbstractNumId {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Previous Paragraph Numbering Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numberingChange")]
pub struct NumberingChange {
  /// original
  #[sdk(attr(qname = "w:original"))]
  pub original: Option<crate::simple_type::StringValue>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
}
/// Custom Tab Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tab")]
pub struct TabStop {
  /// Tab Stop Type
  #[sdk(attr(qname = "w:val"))]
  pub val: TabStopValues,
  /// Tab Leader Character
  #[sdk(attr(qname = "w:leader"))]
  pub leader: Option<TabStopLeaderCharValues>,
  /// Tab Stop Position
  #[sdk(attr(qname = "w:pos"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub position: crate::simple_type::SignedTwipsMeasureValue,
}
/// Run Properties for the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct ParagraphMarkRunProperties {
  /// Inserted Paragraph
  #[sdk(child(qname = "w:ins"))]
  pub inserted: Option<Inserted>,
  /// Deleted Paragraph
  #[sdk(child(qname = "w:del"))]
  pub deleted: Option<Deleted>,
  /// Move Source Paragraph
  #[sdk(child(qname = "w:moveFrom"))]
  pub move_from: Option<MoveFrom>,
  /// Move Destination Paragraph
  #[sdk(child(qname = "w:moveTo"))]
  pub move_to: Option<MoveTo>,
  #[sdk(
        choice(
            child(variant = ConflictInsertion, qname = "w14:conflictIns"),
            child(variant = ConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub paragraph_mark_run_properties_choice1: Option<ParagraphMarkRunPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = RunStyle, qname = "w:rStyle"),
            child(variant = RunFonts, qname = "w:rFonts"),
            child(variant = Bold, qname = "w:b"),
            child(variant = BoldComplexScript, qname = "w:bCs"),
            child(variant = Italic, qname = "w:i"),
            child(variant = ItalicComplexScript, qname = "w:iCs"),
            child(variant = Caps, qname = "w:caps"),
            child(variant = SmallCaps, qname = "w:smallCaps"),
            child(variant = Strike, qname = "w:strike"),
            child(variant = DoubleStrike, qname = "w:dstrike"),
            child(variant = Outline, qname = "w:outline"),
            child(variant = Shadow, qname = "w:shadow"),
            child(variant = Emboss, qname = "w:emboss"),
            child(variant = Imprint, qname = "w:imprint"),
            child(variant = NoProof, qname = "w:noProof"),
            child(variant = SnapToGrid, qname = "w:snapToGrid"),
            child(variant = Vanish, qname = "w:vanish"),
            child(variant = WebHidden, qname = "w:webHidden"),
            child(variant = Color, qname = "w:color"),
            child(variant = Spacing, qname = "w:spacing"),
            child(variant = CharacterScale, qname = "w:w"),
            child(variant = Kern, qname = "w:kern"),
            child(variant = Position, qname = "w:position"),
            child(variant = FontSize, qname = "w:sz"),
            child(variant = FontSizeComplexScript, qname = "w:szCs"),
            child(variant = Highlight, qname = "w:highlight"),
            child(variant = Underline, qname = "w:u"),
            child(variant = TextEffect, qname = "w:effect"),
            child(variant = Border, qname = "w:bdr"),
            child(variant = Shading, qname = "w:shd"),
            child(variant = FitText, qname = "w:fitText"),
            child(variant = VerticalTextAlignment, qname = "w:vertAlign"),
            child(variant = RightToLeftText, qname = "w:rtl"),
            child(variant = ComplexScript, qname = "w:cs"),
            child(variant = Emphasis, qname = "w:em"),
            child(variant = Languages, qname = "w:lang"),
            child(variant = EastAsianLayout, qname = "w:eastAsianLayout"),
            child(variant = SpecVanish, qname = "w:specVanish")
        )
    )]
  pub paragraph_mark_run_properties_choice2: Vec<ParagraphMarkRunPropertiesChoice2>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub glow: Option<std::boxed::Box<crate::schemas::w14::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub shadow: Option<std::boxed::Box<crate::schemas::w14::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub reflection: Option<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub text_outline_effect: Option<std::boxed::Box<crate::schemas::w14::TextOutlineEffect>>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub fill_text_effect: Option<std::boxed::Box<crate::schemas::w14::FillTextEffect>>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub scene3_d: Option<std::boxed::Box<crate::schemas::w14::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub properties3_d: Option<std::boxed::Box<crate::schemas::w14::Properties3D>>,
  /// Defines the Ligatures Class.
  #[sdk(child(qname = "w14:ligatures"))]
  pub ligatures: Option<crate::schemas::w14::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(qname = "w14:numForm"))]
  pub numbering_format: Option<crate::schemas::w14::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(qname = "w14:numSpacing"))]
  pub number_spacing: Option<crate::schemas::w14::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(qname = "w14:stylisticSets"))]
  pub stylistic_sets: Option<crate::schemas::w14::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(qname = "w14:cntxtAlts"))]
  pub contextual_alternatives: Option<crate::schemas::w14::ContextualAlternatives>,
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "w:oMath"))]
  pub office_math: Option<OfficeMath>,
  /// Revision Information for Run Properties on the Paragraph Mark.
  #[sdk(child(qname = "w:rPrChange"))]
  pub paragraph_mark_run_properties_change:
    Option<std::boxed::Box<ParagraphMarkRunPropertiesChange>>,
}
/// Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sectPr")]
pub struct SectionProperties {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Physical Section Mark Character Revision ID
  #[sdk(attr(qname = "w:rsidRPr"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_r_pr: Option<crate::simple_type::HexBinaryValue>,
  /// Section Deletion Revision ID
  #[sdk(attr(qname = "w:rsidDel"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_del: Option<crate::simple_type::HexBinaryValue>,
  /// Section Addition Revision ID
  #[sdk(attr(qname = "w:rsidR"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_r: Option<crate::simple_type::HexBinaryValue>,
  /// Section Properties Revision ID
  #[sdk(attr(qname = "w:rsidSect"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rsid_sect: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(
        choice(
            child(variant = HeaderReference, qname = "w:headerReference"),
            child(variant = FooterReference, qname = "w:footerReference")
        )
    )]
  pub section_properties_choice: Vec<SectionPropertiesChoice>,
  /// Defines the FootnoteProperties Class.
  #[sdk(child(qname = "w:footnotePr"))]
  pub footnote_properties: Option<std::boxed::Box<FootnoteProperties>>,
  /// Defines the EndnoteProperties Class.
  #[sdk(child(qname = "w:endnotePr"))]
  pub endnote_properties: Option<std::boxed::Box<EndnoteProperties>>,
  /// Defines the SectionType Class.
  #[sdk(child(qname = "w:type"))]
  pub section_type: Option<SectionType>,
  /// Defines the PageSize Class.
  #[sdk(child(qname = "w:pgSz"))]
  pub page_size: Option<PageSize>,
  /// Defines the PageMargin Class.
  #[sdk(child(qname = "w:pgMar"))]
  pub page_margin: Option<PageMargin>,
  /// Defines the PaperSource Class.
  #[sdk(child(qname = "w:paperSrc"))]
  pub paper_source: Option<PaperSource>,
  /// Defines the PageBorders Class.
  #[sdk(child(qname = "w:pgBorders"))]
  pub page_borders: Option<std::boxed::Box<PageBorders>>,
  /// Defines the LineNumberType Class.
  #[sdk(child(qname = "w:lnNumType"))]
  pub line_number_type: Option<LineNumberType>,
  /// Defines the PageNumberType Class.
  #[sdk(child(qname = "w:pgNumType"))]
  pub page_number_type: Option<PageNumberType>,
  /// Defines the Columns Class.
  #[sdk(child(qname = "w:cols"))]
  pub columns: Option<Columns>,
  /// Defines the FormProtection Class.
  #[sdk(child(qname = "w:formProt"))]
  pub form_protection: Option<FormProtection>,
  /// Defines the VerticalTextAlignmentOnPage Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub vertical_text_alignment_on_page: Option<VerticalTextAlignmentOnPage>,
  /// Defines the NoEndnote Class.
  #[sdk(child(qname = "w:noEndnote"))]
  pub no_endnote: Option<NoEndnote>,
  /// Defines the TitlePage Class.
  #[sdk(child(qname = "w:titlePg"))]
  pub title_page: Option<TitlePage>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the GutterOnRight Class.
  #[sdk(child(qname = "w:rtlGutter"))]
  pub gutter_on_right: Option<GutterOnRight>,
  /// Defines the DocGrid Class.
  #[sdk(child(qname = "w:docGrid"))]
  pub doc_grid: Option<DocGrid>,
  /// Defines the PrinterSettingsReference Class.
  #[sdk(child(qname = "w:printerSettings"))]
  pub printer_settings_reference: Option<PrinterSettingsReference>,
  /// Defines the FootnoteColumns Class.
  #[sdk(child(qname = "w15:footnoteColumns"))]
  pub footnote_columns: Option<crate::schemas::w15::FootnoteColumns>,
  /// Revision Information for Section Properties.
  #[sdk(child(qname = "w:sectPrChange"))]
  pub section_properties_change: Option<std::boxed::Box<SectionPropertiesChange>>,
}
/// Custom Field Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fldData")]
pub struct FieldData {
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::Base64BinaryValue>,
}
/// Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ffData")]
pub struct FormFieldData {
  #[sdk(
        choice(
            child(variant = FormFieldName, qname = "w:name"),
            child(variant = Enabled, qname = "w:enabled"),
            child(variant = CalculateOnExit, qname = "w:calcOnExit"),
            child(variant = EntryMacro, qname = "w:entryMacro"),
            child(variant = ExitMacro, qname = "w:exitMacro"),
            child(variant = HelpText, qname = "w:helpText"),
            child(variant = StatusText, qname = "w:statusText"),
            child(variant = CheckBox, qname = "w:checkBox"),
            child(variant = DropDownListFormField, qname = "w:ddList"),
            child(variant = TextInput, qname = "w:textInput")
        )
    )]
  pub form_field_data_choice: Vec<FormFieldDataChoice>,
}
/// Form Field Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct FormFieldName {
  /// Form Field Name Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 20u32))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Script Function to Execute on Form Field Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:entryMacro")]
pub struct EntryMacro {
  /// Name of Script Function
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 33u32))]
  pub val: crate::simple_type::StringValue,
}
/// Script Function to Execute on Form Field Exit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:exitMacro")]
pub struct ExitMacro {
  /// Name of Script Function
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 33u32))]
  pub val: crate::simple_type::StringValue,
}
/// Associated Help Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:helpText")]
pub struct HelpText {
  /// Help Text Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<InfoTextValues>,
  /// Help Text Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Associated Status Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:statusText")]
pub struct StatusText {
  /// Status Text Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<InfoTextValues>,
  /// Status Text Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(source = 0u32, union = 0u64, max = 138u32))]
  #[sdk(string_length(source = 1u32, union = 0u64, max = 140u32))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Checkbox Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:checkBox")]
pub struct CheckBox {
  #[sdk(
        choice(
            child(variant = FormFieldSize, qname = "w:size"),
            child(variant = AutomaticallySizeFormField, qname = "w:sizeAuto")
        )
    )]
  pub check_box_choice: Option<CheckBoxChoice>,
  /// Default Checkbox Form Field State.
  #[sdk(child(qname = "w:default"))]
  pub default_check_box_form_field_state: Option<DefaultCheckBoxFormFieldState>,
  /// Checkbox Form Field State.
  #[sdk(child(qname = "w:checked"))]
  pub checked: Option<Checked>,
}
/// Drop-Down List Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:ddList")]
pub struct DropDownListFormField {
  /// Drop-Down List Selection
  #[sdk(child(qname = "w:result"))]
  pub drop_down_list_selection: Option<DropDownListSelection>,
  /// Default Drop-Down List Item Index
  #[sdk(child(qname = "w:default"))]
  pub default_drop_down_list_item_index: Option<DefaultDropDownListItemIndex>,
  /// Drop-Down List Entry.
  #[sdk(child(qname = "w:listEntry"))]
  pub list_entry_form_field: Vec<ListEntryFormField>,
}
/// Text Box Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:textInput")]
pub struct TextInput {
  /// Text Box Form Field Type
  #[sdk(child(qname = "w:type"))]
  pub text_box_form_field_type: Option<TextBoxFormFieldType>,
  /// Default Text Box Form Field String
  #[sdk(child(qname = "w:default"))]
  pub default_text_box_form_field_string: Option<DefaultTextBoxFormFieldString>,
  /// Text Box Form Field Maximum Length
  #[sdk(child(qname = "w:maxLength"))]
  pub max_length: Option<MaxLength>,
  /// Text Box Form Field Formatting
  #[sdk(child(qname = "w:format"))]
  pub format: Option<Format>,
}
/// Default Drop-Down List Item Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:default")]
pub struct DefaultDropDownListItemIndex {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 24))]
  pub val: crate::simple_type::Int32Value,
}
/// Drop-Down List Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:listEntry")]
pub struct ListEntryFormField {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Default Text Box Form Field String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:default")]
pub struct DefaultTextBoxFormFieldString {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Frame Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct FrameName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Text Box Form Field Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:type")]
pub struct TextBoxFormFieldType {
  /// Text Box Form Field Type Values
  #[sdk(attr(qname = "w:val"))]
  pub val: TextBoxFormFieldValues,
}
/// Text Box Form Field Maximum Length.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:maxLength")]
pub struct MaxLength {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 1..))]
  pub val: crate::simple_type::Int16Value,
}
/// Text Box Form Field Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:format")]
pub struct Format {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 64u32))]
  pub val: crate::simple_type::StringValue,
}
/// Single Column Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:col")]
pub struct Column {
  /// Column Width
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub width: Option<crate::simple_type::SignedTwipsMeasureValue>,
  /// Space Before Following Column
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub space: Option<crate::simple_type::SignedTwipsMeasureValue>,
}
/// Revision Information for Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sectPrChange")]
pub struct SectionPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Section Properties
  #[sdk(child(qname = "w:sectPr"))]
  pub previous_section_properties: Option<std::boxed::Box<PreviousSectionProperties>>,
}
/// Revision Information for Run Properties on the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPrChange")]
pub struct ParagraphMarkRunPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Run Properties for the Paragraph Mark
  #[sdk(child(qname = "w:rPr"))]
  pub previous_paragraph_mark_run_properties: std::boxed::Box<PreviousParagraphMarkRunProperties>,
}
/// External Content Import Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:altChunkPr")]
pub struct AltChunkProperties {
  /// Keep Source Formatting on Import
  #[sdk(child(qname = "w:matchSrc"))]
  pub match_source: Option<MatchSource>,
}
/// Phonetic Guide Text Alignment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rubyAlign")]
pub struct RubyAlign {
  /// Phonetic Guide Text Alignment Value
  #[sdk(attr(qname = "w:val"))]
  pub val: RubyAlignValues,
}
/// Distance Between Phonetic Guide Text and Phonetic Guide Base Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hpsRaise")]
pub struct PhoneticGuideRaise {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int16Value,
}
/// Language ID for Phonetic Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lid")]
pub struct LanguageId {
  /// Language Code
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 84u32))]
  pub val: crate::simple_type::StringValue,
}
/// Phonetic Guide Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rubyPr")]
pub struct RubyProperties {
  /// Phonetic Guide Text Alignment
  #[sdk(child(qname = "w:rubyAlign"))]
  pub ruby_align: std::boxed::Box<RubyAlign>,
  /// Phonetic Guide Text Font Size
  #[sdk(child(qname = "w:hps"))]
  pub phonetic_guide_text_font_size: std::boxed::Box<PhoneticGuideTextFontSize>,
  /// Distance Between Phonetic Guide Text and Phonetic Guide Base Text
  #[sdk(child(qname = "w:hpsRaise"))]
  pub phonetic_guide_raise: std::boxed::Box<PhoneticGuideRaise>,
  /// Phonetic Guide Base Text Font Size
  #[sdk(child(qname = "w:hpsBaseText"))]
  pub phonetic_guide_base_text_size: std::boxed::Box<PhoneticGuideBaseTextSize>,
  /// Language ID for Phonetic Guide
  #[sdk(child(qname = "w:lid"))]
  pub language_id: std::boxed::Box<LanguageId>,
  /// Invalidated Field Cache
  #[sdk(child(qname = "w:dirty"))]
  pub dirty: Option<Dirty>,
}
/// Phonetic Guide Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rt")]
pub struct RubyContent {
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub ruby_content_choice: Vec<RubyContentChoice>,
}
/// Phonetic Guide Base Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rubyBase")]
pub struct RubyBase {
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub ruby_base_choice: Vec<RubyBaseChoice>,
}
/// Custom XML Data Date Storage Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:storeMappedDataAs")]
pub struct SdtDateMappingType {
  /// Date Storage Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<DateFormatValues>,
}
/// Date Picker Calendar Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:calendar")]
pub struct Calendar {
  /// Calendar Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<CalendarValues>,
}
/// Combo Box List Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:listItem")]
pub struct ListItem {
  /// List Entry Display Text
  #[sdk(attr(qname = "w:displayText"))]
  pub display_text: Option<crate::simple_type::StringValue>,
  /// List Entry Value
  #[sdk(attr(qname = "w:value"))]
  pub value: Option<crate::simple_type::StringValue>,
}
/// Structured Document Tag Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtPr")]
pub struct SdtProperties {
  #[sdk(
        choice(
            child(variant = RunProperties, qname = "w:rPr"),
            child(variant = SdtAlias, qname = "w:alias"),
            child(variant = Lock, qname = "w:lock"),
            child(variant = SdtPlaceholder, qname = "w:placeholder"),
            child(variant = ShowingPlaceholder, qname = "w:showingPlcHdr"),
            child(variant = WDataBinding, qname = "w:dataBinding"),
            child(variant = W15DataBinding, qname = "w15:dataBinding"),
            child(variant = TemporarySdt, qname = "w:temporary"),
            child(variant = SdtId, qname = "w:id"),
            child(variant = Tag, qname = "w:tag"),
            child(variant = Color, qname = "w15:color"),
            child(variant = Appearance, qname = "w15:appearance"),
            child(variant = WebExtensionLinked, qname = "w15:webExtensionLinked"),
            child(variant = WebExtensionCreated, qname = "w15:webExtensionCreated"),
            empty_child(variant = SdtContentEquation, qname = "w:equation"),
            child(variant = SdtContentComboBox, qname = "w:comboBox"),
            child(variant = SdtContentDate, qname = "w:date"),
            child(variant = SdtContentDocPartObject, qname = "w:docPartObj"),
            child(variant = SdtContentDocPartList, qname = "w:docPartList"),
            child(variant = SdtContentDropDownList, qname = "w:dropDownList"),
            empty_child(variant = SdtContentPicture, qname = "w:picture"),
            empty_child(variant = SdtContentRichText, qname = "w:richText"),
            child(variant = SdtContentText, qname = "w:text"),
            empty_child(variant = SdtContentCitation, qname = "w:citation"),
            empty_child(variant = SdtContentGroup, qname = "w:group"),
            empty_child(variant = SdtContentBibliography, qname = "w:bibliography"),
            empty_child(variant = EntityPickerEmpty, qname = "w14:entityPicker"),
            child(variant = SdtContentCheckBox, qname = "w14:checkbox"),
            child(variant = SdtRepeatedSection, qname = "w15:repeatingSection"),
            empty_child(
                variant = SdtRepeatedSectionItem,
                qname = "w15:repeatingSectionItem"
            ),
            child(variant = TabIndex, qname = "w:tabIndex")
        )
    )]
  pub sdt_properties_choice: Vec<SdtPropertiesChoice>,
}
/// Structured Document Tag End Character Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtEndPr")]
pub struct SdtEndCharProperties {
  /// Run Properties.
  #[sdk(child(qname = "w:rPr"))]
  pub run_properties: Vec<RunProperties>,
}
/// Block-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtContent")]
pub struct SdtContentBlock {
  #[sdk(
        choice(
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub sdt_content_block_choice: Vec<SdtContentBlockChoice>,
}
/// Inline-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtContent")]
pub struct SdtContentRun {
  #[sdk(
        choice(
            child(variant = MRun, qname = "m:r"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = WRun, qname = "w:r"),
            child(variant = BidirectionalOverride, qname = "w:bdo"),
            child(variant = BidirectionalEmbedding, qname = "w:dir"),
            child(variant = SubDocumentReference, qname = "w:subDoc")
        )
    )]
  pub sdt_content_run_choice: Vec<SdtContentRunChoice>,
}
/// Defines the SdtContentRunRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtContent")]
pub struct SdtContentRunRuby {
  #[sdk(
        choice(
            child(variant = CustomXmlRuby, qname = "w:customXml"),
            child(variant = SimpleFieldRuby, qname = "w:fldSimple"),
            child(variant = HyperlinkRuby, qname = "w:hyperlink"),
            child(variant = WRun, qname = "w:r"),
            child(variant = SdtRunRuby, qname = "w:sdt"),
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
            child(variant = OfficeMath, qname = "m:oMath"),
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
            child(variant = MRun, qname = "m:r")
        )
    )]
  pub sdt_content_run_ruby_choice: Vec<SdtContentRunRubyChoice>,
}
/// Cell-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtContent")]
pub struct SdtContentCell {
  #[sdk(
        choice(
            child(variant = TableCell, qname = "w:tc"),
            child(variant = CustomXmlCell, qname = "w:customXml"),
            child(variant = SdtCell, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub sdt_content_cell_choice: Vec<SdtContentCellChoice>,
}
/// Row-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sdtContent")]
pub struct SdtContentRow {
  #[sdk(
        choice(
            child(variant = TableRow, qname = "w:tr"),
            child(variant = CustomXmlRow, qname = "w:customXml"),
            child(variant = SdtRow, qname = "w:sdt"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub sdt_content_row_choice: Vec<SdtContentRowChoice>,
}
/// Custom XML Element Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:customXmlPr")]
pub struct CustomXmlProperties {
  /// Custom XML Element Placeholder Text
  #[sdk(child(qname = "w:placeholder"))]
  pub custom_xml_placeholder: Option<CustomXmlPlaceholder>,
  /// Custom XML Attribute.
  #[sdk(child(qname = "w:attr"))]
  pub custom_xml_attribute: Vec<CustomXmlAttribute>,
}
/// Custom XML Attribute.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:attr")]
pub struct CustomXmlAttribute {
  /// uri
  #[sdk(attr(qname = "w:uri"))]
  pub uri: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 255u32))]
  #[sdk(string_format(kind = "token"))]
  #[sdk(string_format(kind = "ncname"))]
  pub name: crate::simple_type::StringValue,
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Grid Column Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gridCol")]
pub struct GridColumn {
  /// Grid Column Width
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub width: Option<crate::simple_type::TwipsMeasureValue>,
}
/// Revision Information for Table Grid Column Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblGridChange")]
pub struct TableGridChange {
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Table Grid
  #[sdk(child(qname = "w:tblGrid"))]
  pub previous_table_grid: std::boxed::Box<PreviousTableGrid>,
}
/// Revision Information for Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcPrChange")]
pub struct TableCellPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Table Cell Properties
  #[sdk(child(qname = "w:tcPr"))]
  pub previous_table_cell_properties: std::boxed::Box<PreviousTableCellProperties>,
}
/// Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcPr")]
pub struct TableCellProperties {
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Defines the TableCellWidth Class.
  #[sdk(child(qname = "w:tcW"))]
  pub table_cell_width: Option<TableCellWidth>,
  /// Defines the GridSpan Class.
  #[sdk(child(qname = "w:gridSpan"))]
  pub grid_span: Option<GridSpan>,
  /// Defines the HorizontalMerge Class.
  #[sdk(child(qname = "w:hMerge"))]
  pub horizontal_merge: Option<HorizontalMerge>,
  /// Defines the VerticalMerge Class.
  #[sdk(child(qname = "w:vMerge"))]
  pub vertical_merge: Option<VerticalMerge>,
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TableCellFitText Class.
  #[sdk(child(qname = "w:tcFitText"))]
  pub table_cell_fit_text: Option<TableCellFitText>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
  /// Defines the HideMark Class.
  #[sdk(child(qname = "w:hideMark"))]
  pub hide_mark: Option<HideMark>,
  #[sdk(
        choice(
            child(variant = CellInsertion, qname = "w:cellIns"),
            child(variant = CellDeletion, qname = "w:cellDel"),
            child(variant = CellMerge, qname = "w:cellMerge")
        )
    )]
  pub table_cell_properties_choice: Option<TableCellPropertiesChoice>,
  /// Revision Information for Table Cell Properties.
  #[sdk(child(qname = "w:tcPrChange"))]
  pub table_cell_properties_change: Option<std::boxed::Box<TableCellPropertiesChange>>,
}
/// Revision Information for Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPrChange")]
pub struct TablePropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Table Properties
  #[sdk(child(qname = "w:tblPr"))]
  pub previous_table_properties: std::boxed::Box<PreviousTableProperties>,
}
/// Revision Information for Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPrExChange")]
pub struct TablePropertyExceptionsChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Table-Level Property Exceptions
  #[sdk(child(qname = "w:tblPrEx"))]
  pub previous_table_property_exceptions: std::boxed::Box<PreviousTablePropertyExceptions>,
}
/// Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPr")]
pub struct TableProperties {
  /// Defines the TableStyle Class.
  #[sdk(child(qname = "w:tblStyle"))]
  pub table_style: Option<TableStyle>,
  /// Defines the TablePositionProperties Class.
  #[sdk(child(qname = "w:tblpPr"))]
  pub table_position_properties: Option<TablePositionProperties>,
  /// Defines the TableOverlap Class.
  #[sdk(child(qname = "w:tblOverlap"))]
  pub table_overlap: Option<TableOverlap>,
  /// Defines the BiDiVisual Class.
  #[sdk(child(qname = "w:bidiVisual"))]
  pub bi_di_visual: Option<BiDiVisual>,
  /// Defines the TableStyleRowBandSize Class.
  #[sdk(child(qname = "w:tblStyleRowBandSize"))]
  pub table_style_row_band_size: Option<TableStyleRowBandSize>,
  /// Defines the TableStyleColumnBandSize Class.
  #[sdk(child(qname = "w:tblStyleColBandSize"))]
  pub table_style_column_band_size: Option<TableStyleColumnBandSize>,
  /// Defines the TableWidth Class.
  #[sdk(child(qname = "w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableLayout Class.
  #[sdk(child(qname = "w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Defines the TableLook Class.
  #[sdk(child(qname = "w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Defines the TableCaption Class.
  #[sdk(child(qname = "w:tblCaption"))]
  pub table_caption: Option<TableCaption>,
  /// Defines the TableDescription Class.
  #[sdk(child(qname = "w:tblDescription"))]
  pub table_description: Option<TableDescription>,
  /// Revision Information for Table Properties
  #[sdk(child(qname = "w:tblPrChange"))]
  pub table_properties_change: Option<std::boxed::Box<TablePropertiesChange>>,
}
/// Table Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblGrid")]
pub struct TableGrid {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Grid Column Definition.
  #[sdk(child(qname = "w:gridCol"))]
  pub grid_column: Vec<GridColumn>,
  /// Revision Information for Table Grid Column Definitions.
  #[sdk(child(qname = "w:tblGridChange"))]
  pub table_grid_change: Option<std::boxed::Box<TableGridChange>>,
}
/// Footnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pos")]
pub struct FootnotePosition {
  /// Footnote Position Type
  #[sdk(attr(qname = "w:val"))]
  pub val: FootnotePositionValues,
}
/// Footnote Numbering Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numFmt")]
pub struct NumberingFormat {
  /// Numbering Format Type
  #[sdk(attr(qname = "w:val"))]
  pub val: NumberFormatValues,
  /// format
  #[sdk(attr(qname = "w:format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Endnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pos")]
pub struct EndnotePosition {
  /// Endnote Position Type
  #[sdk(attr(qname = "w:val"))]
  pub val: EndnotePositionValues,
}
/// Special Footnote List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnote")]
pub struct FootnoteSpecialReference {
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// Special Endnote List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnote")]
pub struct EndnoteSpecialReference {
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// Index of Column Containing Unique Values for Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:column")]
pub struct ColumnIndex {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Column Delimiter for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:colDelim")]
pub struct ColumnDelimiter {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Unique Value for Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:uniqueTag")]
pub struct UniqueTag {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Base64BinaryValue,
}
/// Data About Single Data Source Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:recipientData")]
pub struct RecipientData {
  /// Record Is Included in Mail Merge
  #[sdk(child(qname = "w:active"))]
  pub active: Option<Active>,
  /// Index of Column Containing Unique Values for Record
  #[sdk(child(qname = "w:column"))]
  pub column_index: std::boxed::Box<ColumnIndex>,
  /// Unique Value for Record
  #[sdk(child(qname = "w:uniqueTag"))]
  pub unique_tag: std::boxed::Box<UniqueTag>,
}
/// Merge Field Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:type")]
pub struct MailMergeFieldType {
  /// Merge Field Mapping Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeOdsoFieldValues,
}
/// ODSO Data Source Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:type")]
pub struct MailMergeSource {
  /// Data Source Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeSourceValues,
}
/// External Data Source to Merge Field Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:fieldMapData")]
pub struct FieldMapData {
  /// Merge Field Mapping
  #[sdk(child(qname = "w:type"))]
  pub mail_merge_field_type: Option<MailMergeFieldType>,
  /// Data Source Name for Column
  #[sdk(child(qname = "w:name"))]
  pub name: Option<Name>,
  /// Predefined Merge Field Name
  #[sdk(child(qname = "w:mappedName"))]
  pub mapped_name: Option<MappedName>,
  /// Index of Column Being Mapped
  #[sdk(child(qname = "w:column"))]
  pub column_index: Option<ColumnIndex>,
  /// Merge Field Name Language ID
  #[sdk(child(qname = "w:lid"))]
  pub language_id: Option<LanguageId>,
  /// Use Country/Region-Based Address Field Ordering
  #[sdk(child(qname = "w:dynamicAddress"))]
  pub dynamic_address: Option<DynamicAddress>,
}
/// Source Document Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mainDocumentType")]
pub struct MainDocumentType {
  /// Mail Merge Source Document Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDocumentValues,
}
/// Data Source Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dataType")]
pub struct DataType {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDataValues,
}
/// Merged Document Destination.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:destination")]
pub struct Destination {
  /// Mail Merge Merged Document Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDestinationValues,
}
/// Office Data Source Object Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:odso")]
pub struct DataSourceObject {
  /// UDL Connection String
  #[sdk(child(qname = "w:udl"))]
  pub udl_connection_string: Option<UdlConnectionString>,
  /// Data Source Table Name
  #[sdk(child(qname = "w:table"))]
  pub data_source_table_name: Option<DataSourceTableName>,
  /// ODSO Data Source File Path
  #[sdk(child(qname = "w:src"))]
  pub source_reference: Option<SourceReference>,
  /// Column Delimiter for Data Source
  #[sdk(child(qname = "w:colDelim"))]
  pub column_delimiter: Option<ColumnDelimiter>,
  /// ODSO Data Source Type
  #[sdk(child(qname = "w:type"))]
  pub mail_merge_source: Option<MailMergeSource>,
  /// First Row of Data Source Contains Column Names
  #[sdk(child(qname = "w:fHdr"))]
  pub first_row_header: Option<FirstRowHeader>,
  /// External Data Source to Merge Field Mapping.
  #[sdk(child(qname = "w:fieldMapData"))]
  pub field_map_data: Vec<FieldMapData>,
  /// Reference to Inclusion/Exclusion Data for Data Source.
  #[sdk(child(qname = "w:recipientData"))]
  pub recipient_data_reference: Option<RecipientDataReference>,
}
/// Single Document Variable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docVar")]
pub struct DocumentVariable {
  /// Document Variable Name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(min = 1u32, max = 255u32))]
  pub name: crate::simple_type::StringValue,
  /// Document Variable Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 0u32, max = 65280u32))]
  pub val: crate::simple_type::StringValue,
}
/// Original Document Revision Save ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rsidRoot")]
pub struct RsidRoot {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Single Session Revision Save ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rsid")]
pub struct Rsid {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Abstract Numbering Definition Identifier.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:nsid")]
pub struct Nsid {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Numbering Template Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tmpl")]
pub struct TemplateCode {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct RunPropertiesBaseStyle {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
}
/// Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPr")]
pub struct ParagraphPropertiesBaseStyle {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
}
/// Default Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPrDefault")]
pub struct RunPropertiesDefault {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub run_properties_base_style: Option<std::boxed::Box<RunPropertiesBaseStyle>>,
}
/// Default Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPrDefault")]
pub struct ParagraphPropertiesDefault {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub paragraph_properties_base_style: Option<std::boxed::Box<ParagraphPropertiesBaseStyle>>,
}
/// Left and Right Margin for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marW")]
pub struct MarginWidth {
  /// Measurement in Pixels
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Top and Bottom Margin for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marH")]
pub struct MarginHeight {
  /// Measurement in Pixels
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Scrollbar Display Option.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:scrollbar")]
pub struct ScrollbarVisibility {
  /// Scrollbar Display Option Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FrameScrollbarVisibilityValues,
}
/// Frameset Splitter Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:w")]
pub struct Width {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Hyphenation Zone.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hyphenationZone")]
pub struct HyphenationZone {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Drawing Grid Horizontal Grid Unit Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:drawingGridHorizontalSpacing")]
pub struct DrawingGridHorizontalSpacing {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Drawing Grid Vertical Grid Unit Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:drawingGridVerticalSpacing")]
pub struct DrawingGridVerticalSpacing {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Drawing Grid Horizontal Origin Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:drawingGridHorizontalOrigin")]
pub struct DrawingGridHorizontalOrigin {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Drawing Grid Vertical Origin Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:drawingGridVerticalOrigin")]
pub struct DrawingGridVerticalOrigin {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Frameset Splitter Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:framesetSplitbar")]
pub struct FramesetSplitbar {
  /// Frameset Splitter Width
  #[sdk(child(qname = "w:w"))]
  pub width: Option<Width>,
  /// Frameset Splitter Color
  #[sdk(child(qname = "w:color"))]
  pub color: Option<Color>,
  /// Do Not Display Frameset Splitters
  #[sdk(child(qname = "w:noBorder"))]
  pub no_border: Option<NoBorder>,
  /// Frameset Splitter Border Style
  #[sdk(child(qname = "w:flatBorders"))]
  pub flat_borders: Option<FlatBorders>,
}
/// Frameset Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:frameLayout")]
pub struct FrameLayout {
  /// Frameset Layout Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FrameLayoutValues,
}
/// Nested Frameset Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:frameset")]
pub struct Frameset {
  /// Nested Frameset Size
  #[sdk(child(qname = "w:sz"))]
  pub frame_size: Option<FrameSize>,
  /// Frameset Splitter Properties
  #[sdk(child(qname = "w:framesetSplitbar"))]
  pub frameset_splitbar: Option<std::boxed::Box<FramesetSplitbar>>,
  /// Frameset Layout
  #[sdk(child(qname = "w:frameLayout"))]
  pub frame_layout: Option<FrameLayout>,
  #[sdk(
        choice(
            child(variant = Frameset, qname = "w:frameset"),
            child(variant = Frame, qname = "w:frame")
        )
    )]
  pub frameset_choice: Vec<FramesetChoice>,
}
/// Single Frame Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:frame")]
pub struct Frame {
  /// Frame Size
  #[sdk(child(qname = "w:sz"))]
  pub frame_size: Option<FrameSize>,
  /// Frame Name
  #[sdk(child(qname = "w:name"))]
  pub frame_name: Option<FrameName>,
  /// Source File for Frame
  #[sdk(child(qname = "w:sourceFileName"))]
  pub source_file_reference: Option<SourceFileReference>,
  /// Left and Right Margin for Frame
  #[sdk(child(qname = "w:marW"))]
  pub margin_width: Option<MarginWidth>,
  /// Top and Bottom Margin for Frame
  #[sdk(child(qname = "w:marH"))]
  pub margin_height: Option<MarginHeight>,
  /// Scrollbar Display Option
  #[sdk(child(qname = "w:scrollbar"))]
  pub scrollbar_visibility: Option<ScrollbarVisibility>,
  /// Frame Cannot Be Resized
  #[sdk(child(qname = "w:noResizeAllowed"))]
  pub no_resize_allowed: Option<NoResizeAllowed>,
  /// Maintain Link to Existing File
  #[sdk(child(qname = "w:linkedToFile"))]
  pub linked_to_file: Option<LinkedToFile>,
}
/// Content Between Numbering Symbol and Paragraph Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:suff")]
pub struct LevelSuffix {
  /// Character Type Between Numbering and Text
  #[sdk(attr(qname = "w:val"))]
  pub val: LevelSuffixValues,
}
/// Numbering Level Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvlText")]
pub struct LevelText {
  /// Level Text
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// Level Text Is Null Character
  #[sdk(attr(qname = "w:null"))]
  pub null: Option<crate::simple_type::OnOffValue>,
}
/// Legacy Numbering Level Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:legacy")]
pub struct LegacyNumbering {
  /// Use Legacy Numbering Properties
  #[sdk(attr(qname = "w:legacy"))]
  pub legacy: Option<crate::simple_type::OnOffValue>,
  /// Legacy Spacing
  #[sdk(attr(qname = "w:legacySpace"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub legacy_space: Option<crate::simple_type::TwipsMeasureValue>,
  /// Legacy Indent
  #[sdk(attr(qname = "w:legacyIndent"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub legacy_indent: Option<crate::simple_type::SignedTwipsMeasureValue>,
}
/// Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvlJc")]
pub struct LevelJustification {
  /// Alignment Type
  #[sdk(attr(qname = "w:val"))]
  pub w_val: LevelJustificationValues,
}
/// Numbering Level Associated Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPr")]
pub struct PreviousParagraphProperties {
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
}
/// Numbering Symbol Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct NumberingSymbolRunProperties {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:rFonts"))]
  pub run_fonts: Vec<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "w:highlight"))]
  pub highlight: Option<Highlight>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:rtl"))]
  pub right_to_left_text: Option<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:cs"))]
  pub complex_script: Option<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub w14_glow: Option<std::boxed::Box<crate::schemas::w14::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub w14_shadow: Option<std::boxed::Box<crate::schemas::w14::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub w14_reflection: Option<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub w14_text_outline_effect: Option<std::boxed::Box<crate::schemas::w14::TextOutlineEffect>>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub w14_fill_text_effect: Option<std::boxed::Box<crate::schemas::w14::FillTextEffect>>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub w14_scene3_d: Option<std::boxed::Box<crate::schemas::w14::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub w14_properties3_d: Option<std::boxed::Box<crate::schemas::w14::Properties3D>>,
  /// Defines the Ligatures Class.
  #[sdk(child(qname = "w14:ligatures"))]
  pub w14_ligatures: Option<crate::schemas::w14::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(qname = "w14:numForm"))]
  pub w14_numbering_format: Option<crate::schemas::w14::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(qname = "w14:numSpacing"))]
  pub w14_number_spacing: Option<crate::schemas::w14::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(qname = "w14:stylisticSets"))]
  pub w14_stylistic_sets: Option<crate::schemas::w14::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(qname = "w14:cntxtAlts"))]
  pub w14_contextual_alternatives: Option<crate::schemas::w14::ContextualAlternatives>,
}
/// Abstract Numbering Definition Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:multiLevelType")]
pub struct MultiLevelType {
  /// Abstract Numbering Definition Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MultiLevelValues,
}
/// Numbering Level Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvl")]
pub struct Level {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Numbering Level
  #[sdk(attr(qname = "w:ilvl"))]
  pub level_index: crate::simple_type::Int32Value,
  /// Template Code
  #[sdk(attr(qname = "w:tplc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub template_code: Option<crate::simple_type::HexBinaryValue>,
  /// Tentative Numbering
  #[sdk(attr(qname = "w:tentative"))]
  pub tentative: Option<crate::simple_type::OnOffValue>,
  /// Starting Value
  #[sdk(child(qname = "w:start"))]
  pub start_numbering_value: Option<StartNumberingValue>,
  /// Numbering Format
  #[sdk(child(qname = "w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Restart Numbering Level Symbol
  #[sdk(child(qname = "w:lvlRestart"))]
  pub level_restart: Option<LevelRestart>,
  /// Paragraph Style's Associated Numbering Level
  #[sdk(child(qname = "w:pStyle"))]
  pub paragraph_style_id_in_level: Option<ParagraphStyleIdInLevel>,
  /// Display All Levels Using Arabic Numerals
  #[sdk(child(qname = "w:isLgl"))]
  pub is_legal_numbering_style: Option<IsLegalNumberingStyle>,
  /// Content Between Numbering Symbol and Paragraph Text
  #[sdk(child(qname = "w:suff"))]
  pub level_suffix: Option<LevelSuffix>,
  /// Numbering Level Text
  #[sdk(child(qname = "w:lvlText"))]
  pub level_text: Option<LevelText>,
  /// Picture Numbering Symbol Definition Reference
  #[sdk(child(qname = "w:lvlPicBulletId"))]
  pub level_picture_bullet_id: Option<LevelPictureBulletId>,
  /// Legacy Numbering Level Properties
  #[sdk(child(qname = "w:legacy"))]
  pub legacy_numbering: Option<LegacyNumbering>,
  /// Justification
  #[sdk(child(qname = "w:lvlJc"))]
  pub level_justification: Option<LevelJustification>,
  /// Numbering Level Associated Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub previous_paragraph_properties: Option<std::boxed::Box<PreviousParagraphProperties>>,
  /// Numbering Symbol Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub numbering_symbol_run_properties: Option<std::boxed::Box<NumberingSymbolRunProperties>>,
}
/// Picture Numbering Symbol Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:numPicBullet")]
pub struct NumberingPictureBullet {
  /// numPicBulletId
  #[sdk(attr(qname = "w:numPicBulletId"))]
  pub numbering_picture_bullet_id: crate::simple_type::Int32Value,
  #[sdk(
        choice(
            child(variant = PictureBulletBase, qname = "w:pict"),
            child(variant = Drawing, qname = "w:drawing")
        )
    )]
  pub numbering_picture_bullet_choice: Option<NumberingPictureBulletChoice>,
}
/// Abstract Numbering Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:abstractNum")]
pub struct AbstractNum {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Abstract Numbering Definition ID
  #[sdk(attr(qname = "w:abstractNumId"))]
  #[sdk(number_range(range = 0..))]
  pub abstract_number_id: crate::simple_type::Int32Value,
  /// Abstract Numbering Definition Identifier
  #[sdk(child(qname = "w:nsid"))]
  pub nsid: Option<Nsid>,
  /// Abstract Numbering Definition Type
  #[sdk(child(qname = "w:multiLevelType"))]
  pub multi_level_type: Option<MultiLevelType>,
  /// Numbering Template Code
  #[sdk(child(qname = "w:tmpl"))]
  pub template_code: Option<TemplateCode>,
  /// Abstract Numbering Definition Name
  #[sdk(child(qname = "w:name"))]
  pub abstract_num_definition_name: Option<AbstractNumDefinitionName>,
  /// Numbering Style Definition
  #[sdk(child(qname = "w:styleLink"))]
  pub style_link: Option<StyleLink>,
  /// Numbering Style Reference
  #[sdk(child(qname = "w:numStyleLink"))]
  pub numbering_style_link: Option<NumberingStyleLink>,
  /// Numbering Level Definition.
  #[sdk(child(qname = "w:lvl"))]
  pub level: Vec<Level>,
}
/// Numbering Definition Instance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:num")]
pub struct NumberingInstance {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// numId
  #[sdk(attr(qname = "w:numId"))]
  pub number_id: crate::simple_type::Int32Value,
  /// durableId
  #[sdk(attr(qname = "w:durableId"))]
  pub w_durable_id: Option<crate::simple_type::Int32Value>,
  /// Defines the AbstractNumId Class.
  #[sdk(child(qname = "w:abstractNumId"))]
  pub abstract_num_id: std::boxed::Box<AbstractNumId>,
  /// Defines the LevelOverride Class.
  #[sdk(child(qname = "w:lvlOverride"))]
  pub level_override: Vec<LevelOverride>,
}
/// Table Style Conditional Formatting Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPr")]
pub struct StyleParagraphProperties {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Paragraph Mark Run Properties.
  #[sdk(child(qname = "w:rPr"))]
  pub paragraph_mark_run_properties: Option<std::boxed::Box<ParagraphMarkRunProperties>>,
  /// Defines the ParagraphPropertiesChange Class.
  #[sdk(child(qname = "w:pPrChange"))]
  pub paragraph_properties_change: Option<std::boxed::Box<ParagraphPropertiesChange>>,
}
/// Table Style Conditional Formatting Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPr")]
pub struct TableStyleConditionalFormattingTableProperties {
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
}
/// Table Style Conditional Formatting Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trPr")]
pub struct TableStyleConditionalFormattingTableRowProperties {
  #[sdk(
        choice(
            child(variant = Hidden, qname = "w:hidden"),
            child(variant = CantSplit, qname = "w:cantSplit"),
            child(variant = TableHeader, qname = "w:tblHeader"),
            child(variant = TableCellSpacing, qname = "w:tblCellSpacing"),
            child(variant = TableJustification, qname = "w:jc"),
            child(variant = ConditionalFormatStyle, qname = "w:cnfStyle"),
            child(variant = DivId, qname = "w:divId"),
            child(variant = GridBefore, qname = "w:gridBefore"),
            child(variant = GridAfter, qname = "w:gridAfter"),
            child(variant = WidthBeforeTableRow, qname = "w:wBefore"),
            child(variant = WidthAfterTableRow, qname = "w:wAfter"),
            child(variant = TableRowHeight, qname = "w:trHeight")
        )
    )]
  pub table_style_conditional_formatting_table_row_properties_choice:
    Vec<TableStyleConditionalFormattingTableRowPropertiesChoice>,
}
/// Table Style Conditional Formatting Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcPr")]
pub struct TableStyleConditionalFormattingTableCellProperties {
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
}
/// Primary Style Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct StyleName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "[^,]*"))]
  pub val: crate::simple_type::StringValue,
}
/// Optional User Interface Sorting Order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:uiPriority")]
pub struct UiPriority {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 99))]
  pub val: crate::simple_type::Int32Value,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rPr")]
pub struct StyleRunProperties {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(qname = "w14:glow"))]
  pub w14_glow: Option<std::boxed::Box<crate::schemas::w14::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w14:shadow"))]
  pub w14_shadow: Option<std::boxed::Box<crate::schemas::w14::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(qname = "w14:reflection"))]
  pub w14_reflection: Option<crate::schemas::w14::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(qname = "w14:textOutline"))]
  pub w14_text_outline_effect: Option<std::boxed::Box<crate::schemas::w14::TextOutlineEffect>>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(qname = "w14:textFill"))]
  pub w14_fill_text_effect: Option<std::boxed::Box<crate::schemas::w14::FillTextEffect>>,
  /// Defines the Scene3D Class.
  #[sdk(child(qname = "w14:scene3d"))]
  pub w14_scene3_d: Option<std::boxed::Box<crate::schemas::w14::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(qname = "w14:props3d"))]
  pub w14_properties3_d: Option<std::boxed::Box<crate::schemas::w14::Properties3D>>,
  /// Defines the RunPropertiesChange Class.
  #[sdk(child(qname = "w:rPrChange"))]
  pub run_properties_change: Option<std::boxed::Box<RunPropertiesChange>>,
}
/// Style Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPr")]
pub struct StyleTableProperties {
  /// Defines the TableStyleRowBandSize Class.
  #[sdk(child(qname = "w:tblStyleRowBandSize"))]
  pub table_style_row_band_size: Option<TableStyleRowBandSize>,
  /// Defines the TableStyleColumnBandSize Class.
  #[sdk(child(qname = "w:tblStyleColBandSize"))]
  pub table_style_column_band_size: Option<TableStyleColumnBandSize>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Table Layout.
  #[sdk(child(qname = "w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
}
/// Style Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tcPr")]
pub struct StyleTableCellProperties {
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
}
/// Style Conditional Table Formatting Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblStylePr")]
pub struct TableStyleProperties {
  /// Table Style Conditional Formatting Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: TableStyleOverrideValues,
  /// Table Style Conditional Formatting Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub style_paragraph_properties: Option<std::boxed::Box<StyleParagraphProperties>>,
  /// Table Style Conditional Formatting Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub run_properties_base_style: Option<std::boxed::Box<RunPropertiesBaseStyle>>,
  /// Table Style Conditional Formatting Table Properties
  #[sdk(child(qname = "w:tblPr"))]
  pub table_style_conditional_formatting_table_properties:
    Option<std::boxed::Box<TableStyleConditionalFormattingTableProperties>>,
  /// Table Style Conditional Formatting Table Row Properties
  #[sdk(child(qname = "w:trPr"))]
  pub table_style_conditional_formatting_table_row_properties:
    Option<TableStyleConditionalFormattingTableRowProperties>,
  /// Table Style Conditional Formatting Table Cell Properties
  #[sdk(child(qname = "w:tcPr"))]
  pub table_style_conditional_formatting_table_cell_properties:
    Option<std::boxed::Box<TableStyleConditionalFormattingTableCellProperties>>,
}
/// Latent Style Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lsdException")]
pub struct LatentStyleExceptionInfo {
  /// Primary Style Name
  #[sdk(attr(qname = "w:name"))]
  pub name: crate::simple_type::StringValue,
  /// Latent Style Locking Setting
  #[sdk(attr(qname = "w:locked"))]
  pub locked: Option<crate::simple_type::OnOffValue>,
  /// Override default sorting order
  #[sdk(attr(qname = "w:uiPriority"))]
  #[sdk(number_range(range = 0..= 99))]
  pub ui_priority: Option<crate::simple_type::Int32Value>,
  /// Semi hidden text override
  #[sdk(attr(qname = "w:semiHidden"))]
  pub semi_hidden: Option<crate::simple_type::OnOffValue>,
  /// Unhide when used
  #[sdk(attr(qname = "w:unhideWhenUsed"))]
  pub unhide_when_used: Option<crate::simple_type::OnOffValue>,
  /// Latent Style Primary Style Setting
  #[sdk(attr(qname = "w:qFormat"))]
  pub primary_style: Option<crate::simple_type::OnOffValue>,
}
/// Document Default Paragraph and Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docDefaults")]
pub struct DocDefaults {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Default Run Properties
  #[sdk(child(qname = "w:rPrDefault"))]
  pub run_properties_default: Option<std::boxed::Box<RunPropertiesDefault>>,
  /// Default Paragraph Properties
  #[sdk(child(qname = "w:pPrDefault"))]
  pub paragraph_properties_default: Option<std::boxed::Box<ParagraphPropertiesDefault>>,
}
/// Latent Style Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:latentStyles")]
pub struct LatentStyles {
  /// Default Style Locking Setting
  #[sdk(attr(qname = "w:defLockedState"))]
  pub default_locked_state: Option<crate::simple_type::OnOffValue>,
  /// Default User Interface Priority Setting
  #[sdk(attr(qname = "w:defUIPriority"))]
  #[sdk(number_range(range = 0..= 99))]
  pub default_ui_priority: Option<crate::simple_type::Int32Value>,
  /// Default Semi-Hidden Setting
  #[sdk(attr(qname = "w:defSemiHidden"))]
  pub default_semi_hidden: Option<crate::simple_type::OnOffValue>,
  /// Default Hidden Until Used Setting
  #[sdk(attr(qname = "w:defUnhideWhenUsed"))]
  pub default_unhide_when_used: Option<crate::simple_type::OnOffValue>,
  /// Default Primary Style Setting
  #[sdk(attr(qname = "w:defQFormat"))]
  pub default_primary_style: Option<crate::simple_type::OnOffValue>,
  /// Latent Style Count
  #[sdk(attr(qname = "w:count"))]
  pub count: Option<crate::simple_type::Int32Value>,
  /// Latent Style Exception.
  #[sdk(child(qname = "w:lsdException"))]
  pub latent_style_exception_info: Vec<LatentStyleExceptionInfo>,
}
/// Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:style")]
pub struct Style {
  /// Style Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<StyleValues>,
  /// Style ID
  #[sdk(attr(qname = "w:styleId"))]
  #[sdk(string_length(max = 253u32))]
  pub style_id: Option<crate::simple_type::StringValue>,
  /// Default Style
  #[sdk(attr(qname = "w:default"))]
  pub default: Option<crate::simple_type::OnOffValue>,
  /// User-Defined Style
  #[sdk(attr(qname = "w:customStyle"))]
  pub custom_style: Option<crate::simple_type::OnOffValue>,
  /// Primary Style Name
  #[sdk(child(qname = "w:name"))]
  pub style_name: Option<StyleName>,
  /// Alternate Style Names
  #[sdk(child(qname = "w:aliases"))]
  pub aliases: Option<Aliases>,
  /// Parent Style ID
  #[sdk(child(qname = "w:basedOn"))]
  pub based_on: Option<BasedOn>,
  /// Style For Next Paragraph
  #[sdk(child(qname = "w:next"))]
  pub next_paragraph_style: Option<NextParagraphStyle>,
  /// Linked Style Reference
  #[sdk(child(qname = "w:link"))]
  pub linked_style: Option<LinkedStyle>,
  /// Automatically Merge User Formatting Into Style Definition
  #[sdk(child(qname = "w:autoRedefine"))]
  pub auto_redefine: Option<AutoRedefine>,
  /// Hide Style From User Interface
  #[sdk(child(qname = "w:hidden"))]
  pub style_hidden: Option<StyleHidden>,
  /// Optional User Interface Sorting Order
  #[sdk(child(qname = "w:uiPriority"))]
  pub ui_priority: Option<UiPriority>,
  /// Hide Style From Main User Interface
  #[sdk(child(qname = "w:semiHidden"))]
  pub semi_hidden: Option<SemiHidden>,
  /// Remove Semi-Hidden Property When Style Is Used
  #[sdk(child(qname = "w:unhideWhenUsed"))]
  pub unhide_when_used: Option<UnhideWhenUsed>,
  /// Primary Style
  #[sdk(child(qname = "w:qFormat"))]
  pub primary_style: Option<PrimaryStyle>,
  /// Style Cannot Be Applied
  #[sdk(child(qname = "w:locked"))]
  pub locked: Option<Locked>,
  /// E-Mail Message Text Style
  #[sdk(child(qname = "w:personal"))]
  pub personal: Option<Personal>,
  /// E-Mail Message Composition Style
  #[sdk(child(qname = "w:personalCompose"))]
  pub personal_compose: Option<PersonalCompose>,
  /// E-Mail Message Reply Style
  #[sdk(child(qname = "w:personalReply"))]
  pub personal_reply: Option<PersonalReply>,
  /// Revision Identifier for Style Definition
  #[sdk(child(qname = "w:rsid"))]
  pub rsid: Option<Rsid>,
  /// Style Paragraph Properties
  #[sdk(child(qname = "w:pPr"))]
  pub style_paragraph_properties: Option<std::boxed::Box<StyleParagraphProperties>>,
  /// Run Properties
  #[sdk(child(qname = "w:rPr"))]
  pub style_run_properties: Option<std::boxed::Box<StyleRunProperties>>,
  /// Style Table Properties
  #[sdk(child(qname = "w:tblPr"))]
  pub style_table_properties: Option<std::boxed::Box<StyleTableProperties>>,
  /// Style Table Row Properties
  #[sdk(child(qname = "w:trPr"))]
  pub table_style_conditional_formatting_table_row_properties:
    Option<TableStyleConditionalFormattingTableRowProperties>,
  /// Style Table Cell Properties
  #[sdk(child(qname = "w:tcPr"))]
  pub style_table_cell_properties: Option<std::boxed::Box<StyleTableCellProperties>>,
  /// Style Conditional Table Formatting Properties.
  #[sdk(child(qname = "w:tblStylePr"))]
  pub table_style_properties: Vec<TableStyleProperties>,
}
/// Properties for a Single Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:font")]
pub struct Font {
  /// name
  #[sdk(attr(qname = "w:name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the AltName Class.
  #[sdk(child(qname = "w:altName"))]
  pub alt_name: Option<AltName>,
  /// Defines the Panose1Number Class.
  #[sdk(child(qname = "w:panose1"))]
  pub panose1_number: Option<Panose1Number>,
  /// Defines the FontCharSet Class.
  #[sdk(child(qname = "w:charset"))]
  pub font_char_set: Option<FontCharSet>,
  /// Defines the FontFamily Class.
  #[sdk(child(qname = "w:family"))]
  pub font_family: Option<FontFamily>,
  /// Defines the NotTrueType Class.
  #[sdk(child(qname = "w:notTrueType"))]
  pub not_true_type: Option<NotTrueType>,
  /// Defines the Pitch Class.
  #[sdk(child(qname = "w:pitch"))]
  pub pitch: Option<Pitch>,
  /// Defines the FontSignature Class.
  #[sdk(child(qname = "w:sig"))]
  pub font_signature: Option<FontSignature>,
  /// Defines the EmbedRegularFont Class.
  #[sdk(child(qname = "w:embedRegular"))]
  pub embed_regular_font: Option<EmbedRegularFont>,
  /// Defines the EmbedBoldFont Class.
  #[sdk(child(qname = "w:embedBold"))]
  pub embed_bold_font: Option<EmbedBoldFont>,
  /// Defines the EmbedItalicFont Class.
  #[sdk(child(qname = "w:embedItalic"))]
  pub embed_italic_font: Option<EmbedItalicFont>,
  /// Defines the EmbedBoldItalicFont Class.
  #[sdk(child(qname = "w:embedBoldItalic"))]
  pub embed_bold_italic_font: Option<EmbedBoldItalicFont>,
}
/// Left Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marLeft")]
pub struct LeftMarginDiv {
  /// Positive or Negative Value in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::SignedTwipsMeasureValue,
}
/// Right Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marRight")]
pub struct RightMarginDiv {
  /// Positive or Negative Value in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::SignedTwipsMeasureValue,
}
/// Top Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marTop")]
pub struct TopMarginDiv {
  /// Positive or Negative Value in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::SignedTwipsMeasureValue,
}
/// Bottom Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:marBottom")]
pub struct BottomMarginDiv {
  /// Positive or Negative Value in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::SignedTwipsMeasureValue,
}
/// Set of Borders for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:divBdr")]
pub struct DivBorder {
  /// Top Border for HTML div
  #[sdk(child(qname = "w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Border for HTML div
  #[sdk(child(qname = "w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Bottom Border for HTML div
  #[sdk(child(qname = "w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Border for HTML div
  #[sdk(child(qname = "w:right"))]
  pub right_border: Option<RightBorder>,
}
/// Child div Elements Contained within Current div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:divsChild")]
pub struct DivsChild {
  /// Information About Single HTML div Element.
  #[sdk(child(qname = "w:div"))]
  pub div: Vec<Div>,
}
/// Defines the Divs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:divs")]
pub struct Divs {
  /// Information About Single HTML div Element.
  #[sdk(child(qname = "w:div"))]
  pub div: Vec<Div>,
}
/// Information About Single HTML div Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:div")]
pub struct Div {
  /// div Data ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "1",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-1",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Data for HTML blockquote Element
  #[sdk(child(qname = "w:blockQuote"))]
  pub block_quote: Option<BlockQuote>,
  /// Data for HTML body Element
  #[sdk(child(qname = "w:bodyDiv"))]
  pub body_div: Option<BodyDiv>,
  /// Left Margin for HTML div
  #[sdk(child(qname = "w:marLeft"))]
  pub left_margin_div: std::boxed::Box<LeftMarginDiv>,
  /// Right Margin for HTML div
  #[sdk(child(qname = "w:marRight"))]
  pub right_margin_div: std::boxed::Box<RightMarginDiv>,
  /// Top Margin for HTML div
  #[sdk(child(qname = "w:marTop"))]
  pub top_margin_div: std::boxed::Box<TopMarginDiv>,
  /// Bottom Margin for HTML div
  #[sdk(child(qname = "w:marBottom"))]
  pub bottom_margin_div: std::boxed::Box<BottomMarginDiv>,
  /// Set of Borders for HTML div
  #[sdk(child(qname = "w:divBdr"))]
  pub div_border: Option<std::boxed::Box<DivBorder>>,
  /// Child div Elements Contained within Current div.
  #[sdk(child(qname = "w:divsChild"))]
  pub divs_child: Vec<DivsChild>,
}
/// Comment Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:comment")]
pub struct Comment {
  /// initials
  #[sdk(attr(qname = "w:initials"))]
  #[sdk(string_length(max = 9u32))]
  pub initials: Option<crate::simple_type::StringValue>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
            child(variant = ProofError, qname = "w:proofErr"),
            child(variant = PermStart, qname = "w:permStart"),
            child(variant = PermEnd, qname = "w:permEnd"),
            child(variant = BookmarkStart, qname = "w:bookmarkStart"),
            child(variant = BookmarkEnd, qname = "w:bookmarkEnd"),
            child(variant = CommentRangeStart, qname = "w:commentRangeStart"),
            child(variant = CommentRangeEnd, qname = "w:commentRangeEnd")
        )
    )]
  pub comment_choice: Vec<CommentChoice>,
}
/// Footnote Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnote")]
pub struct Footnote {
  /// Footnote/Endnote Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<FootnoteEndnoteValues>,
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub footnote_choice: Vec<FootnoteChoice>,
}
/// Endnote Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnote")]
pub struct Endnote {
  /// Footnote/Endnote Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<FootnoteEndnoteValues>,
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub endnote_choice: Vec<EndnoteChoice>,
}
/// Entry Insertion Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:behavior")]
pub struct Behavior {
  /// Insertion Behavior Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartBehaviorValues,
}
/// Entry Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:type")]
pub struct DocPartType {
  /// Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartValues,
}
/// Gallery Associated With Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:gallery")]
pub struct Gallery {
  /// Gallery Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartGalleryValues,
}
/// Single Automatic Captioning Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoCaption")]
pub struct AutoCaption {
  /// Identifier of Object to be Automatically Captioned
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 255u32))]
  pub name: crate::simple_type::StringValue,
  /// Caption Used for Automatic Captioning
  #[sdk(attr(qname = "w:caption"))]
  #[sdk(string_length(max = 255u32))]
  pub caption: crate::simple_type::StringValue,
}
/// Single Caption Type Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:caption")]
pub struct Caption {
  /// Caption Type Name
  #[sdk(attr(qname = "w:name"))]
  #[sdk(string_length(max = 255u32))]
  pub name: crate::simple_type::StringValue,
  /// Automatic Caption Placement
  #[sdk(attr(qname = "w:pos"))]
  pub position: Option<CaptionPositionValues>,
  /// Include Chapter Number in Field for Caption
  #[sdk(attr(qname = "w:chapNum"))]
  pub chapter_number: Option<crate::simple_type::OnOffValue>,
  /// Style for Chapter Headings
  #[sdk(attr(qname = "w:heading"))]
  pub heading: Option<crate::simple_type::Int32Value>,
  /// Do Not Include Name In Caption
  #[sdk(attr(qname = "w:noLabel"))]
  pub no_label: Option<crate::simple_type::OnOffValue>,
  /// Caption Numbering Format
  #[sdk(attr(qname = "w:numFmt"))]
  pub number_format: Option<NumberFormatValues>,
  /// Chapter Number/Item Index Separator
  #[sdk(attr(qname = "w:sep"))]
  pub separator: Option<ChapterSeparatorValues>,
}
/// Automatic Captioning Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:autoCaptions")]
pub struct AutoCaptions {
  /// Single Automatic Captioning Setting.
  #[sdk(child(qname = "w:autoCaption"))]
  pub auto_caption: Vec<AutoCaption>,
}
/// Document Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:background")]
pub struct DocumentBackground {
  /// color
  #[sdk(attr(qname = "w:color"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_HexColorRGB",
    min = 3u32,
    max = 3u32,
  ))]
  pub color: Option<crate::simple_type::StringValue>,
  /// themeColor
  #[sdk(attr(qname = "w:themeColor"))]
  pub theme_color: Option<ThemeColorValues>,
  /// themeTint
  #[sdk(attr(qname = "w:themeTint"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_tint: Option<crate::simple_type::StringValue>,
  /// themeShade
  #[sdk(attr(qname = "w:themeShade"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub theme_shade: Option<crate::simple_type::StringValue>,
  /// Document Background.
  #[sdk(child(qname = "v:background"))]
  pub background: Option<std::boxed::Box<crate::schemas::v::Background>>,
}
/// List of Glossary Document Entries.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docParts")]
pub struct DocParts {
  /// Glossary Document Entry.
  #[sdk(child(qname = "w:docPart"))]
  pub doc_part: Vec<DocPart>,
}
/// Entry Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:name")]
pub struct DocPartName {
  /// Name Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
  /// Built-In Entry
  #[sdk(attr(qname = "w:decorated"))]
  pub decorated: Option<crate::simple_type::OnOffValue>,
}
/// Entry Categorization.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:category")]
pub struct Category {
  /// Category Associated With Entry
  #[sdk(child(qname = "w:name"))]
  pub name: std::boxed::Box<Name>,
  /// Gallery Associated With Entry
  #[sdk(child(qname = "w:gallery"))]
  pub gallery: std::boxed::Box<Gallery>,
}
/// Entry Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:types")]
pub struct DocPartTypes {
  /// Entry Is Of All Types
  #[sdk(attr(qname = "w:all"))]
  pub all: Option<crate::simple_type::OnOffValue>,
  /// Entry Type.
  #[sdk(child(qname = "w:type"))]
  pub doc_part_type: Vec<DocPartType>,
}
/// Entry Insertion Behaviors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:behaviors")]
pub struct Behaviors {
  /// Entry Insertion Behavior.
  #[sdk(child(qname = "w:behavior"))]
  pub behavior: Vec<Behavior>,
}
/// Entry ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:guid")]
pub struct DocPartId {
  /// GUID Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Glossary Document Entry Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartPr")]
pub struct DocPartProperties {
  /// Entry Name
  #[sdk(child(qname = "w:name"))]
  pub doc_part_name: Option<DocPartName>,
  /// Associated Paragraph Style Name
  #[sdk(child(qname = "w:style"))]
  pub style_id: Option<StyleId>,
  /// Entry Categorization
  #[sdk(child(qname = "w:category"))]
  pub category: Option<std::boxed::Box<Category>>,
  /// Entry Types
  #[sdk(child(qname = "w:types"))]
  pub doc_part_types: Option<DocPartTypes>,
  /// Entry Insertion Behaviors
  #[sdk(child(qname = "w:behaviors"))]
  pub behaviors: Option<Behaviors>,
  /// Description for Entry
  #[sdk(child(qname = "w:description"))]
  pub description: Option<Description>,
  /// Entry ID
  #[sdk(child(qname = "w:guid"))]
  pub doc_part_id: Option<DocPartId>,
}
/// Contents of Glossary Document Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartBody")]
pub struct DocPartBody {
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            child(variant = RunConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub doc_part_body_choice: Vec<DocPartBodyChoice>,
  /// Section Properties.
  #[sdk(child(qname = "w:sectPr"))]
  pub section_properties: Option<std::boxed::Box<SectionProperties>>,
}
/// Defines the Body Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:body")]
pub struct Body {
  #[sdk(
        choice(
            child(variant = AltChunk, qname = "w:altChunk"),
            child(variant = CustomXmlBlock, qname = "w:customXml"),
            child(variant = SdtBlock, qname = "w:sdt"),
            child(variant = Paragraph, qname = "w:p"),
            child(variant = Table, qname = "w:tbl"),
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
            any
        )
    )]
  pub body_choice: Vec<BodyChoice>,
  /// Section Properties.
  #[sdk(child(qname = "w:sectPr"))]
  pub section_properties: Option<std::boxed::Box<SectionProperties>>,
}
/// Glossary Document Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPart")]
pub struct DocPart {
  /// Glossary Document Entry Properties
  #[sdk(child(qname = "w:docPartPr"))]
  pub doc_part_properties: Option<std::boxed::Box<DocPartProperties>>,
  /// Contents of Glossary Document Entry
  #[sdk(child(qname = "w:docPartBody"))]
  pub doc_part_body: Option<std::boxed::Box<DocPartBody>>,
}
/// Defines the CompatibilitySetting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:compatSetting")]
pub struct CompatibilitySetting {
  /// name
  #[sdk(attr(qname = "w:name"))]
  pub w_name: CompatSettingNameValues,
  /// uri
  #[sdk(attr(qname = "w:uri"))]
  pub w_uri: crate::simple_type::StringValue,
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub w_val: crate::simple_type::StringValue,
}
/// Table Cell Left Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:left")]
pub struct TableCellLeftMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Right Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:right")]
pub struct TableCellRightMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::MeasurementOrPercentValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblPrEx")]
pub struct TablePropertyExceptions {
  /// Preferred Table Width Exception
  #[sdk(child(qname = "w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Table Alignment Exception
  #[sdk(child(qname = "w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Table Cell Spacing Exception
  #[sdk(child(qname = "w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Table Indent from Leading Margin Exception
  #[sdk(child(qname = "w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Table Borders Exceptions
  #[sdk(child(qname = "w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Table Shading Exception
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Table Layout Exception
  #[sdk(child(qname = "w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Table Cell Margin Exceptions
  #[sdk(child(qname = "w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Table Style Conditional Formatting Settings Exception
  #[sdk(child(qname = "w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Revision Information for Table-Level Property Exceptions
  #[sdk(child(qname = "w:tblPrExChange"))]
  pub table_property_exceptions_change: Option<std::boxed::Box<TablePropertyExceptionsChange>>,
}
/// Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trPr")]
pub struct TableRowProperties {
  #[sdk(
        choice(
            child(variant = ConditionalFormatStyle, qname = "w:cnfStyle"),
            child(variant = DivId, qname = "w:divId"),
            child(variant = GridBefore, qname = "w:gridBefore"),
            child(variant = GridAfter, qname = "w:gridAfter"),
            child(variant = WidthBeforeTableRow, qname = "w:wBefore"),
            child(variant = WidthAfterTableRow, qname = "w:wAfter"),
            child(variant = TableRowHeight, qname = "w:trHeight"),
            child(variant = Hidden, qname = "w:hidden"),
            child(variant = CantSplit, qname = "w:cantSplit"),
            child(variant = TableHeader, qname = "w:tblHeader"),
            child(variant = TableCellSpacing, qname = "w:tblCellSpacing"),
            child(variant = TableJustification, qname = "w:jc")
        )
    )]
  pub table_row_properties_choice1: Vec<TableRowPropertiesChoice>,
  /// Inserted Paragraph.
  #[sdk(child(qname = "w:ins"))]
  pub inserted: Option<Inserted>,
  /// Deleted Paragraph.
  #[sdk(child(qname = "w:del"))]
  pub deleted: Option<Deleted>,
  /// Revision Information for Table Row Properties.
  #[sdk(child(qname = "w:trPrChange"))]
  pub table_row_properties_change: Option<std::boxed::Box<TableRowPropertiesChange>>,
  #[sdk(
        choice(
            child(variant = ConflictInsertion, qname = "w14:conflictIns"),
            child(variant = ConflictDeletion, qname = "w14:conflictDel")
        )
    )]
  pub table_row_properties_choice2: Option<TableRowPropertiesChoice2>,
}
/// Revision Information for Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:trPrChange")]
pub struct TableRowPropertiesChange {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(qname = "w16du:dateUtc"))]
  pub date_utc: Option<crate::simple_type::DateTimeValue>,
  /// Annotation Identifier
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    max = "-2",
    min_inclusive = false,
    max_inclusive = true,
  ))]
  pub id: crate::simple_type::StringValue,
  /// Previous Table Row Properties
  #[sdk(child(qname = "w:trPr"))]
  pub previous_table_row_properties: std::boxed::Box<PreviousTableRowProperties>,
}
/// Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pPr")]
pub struct ParagraphProperties {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Defines the DivId Class.
  #[sdk(child(qname = "w:divId"))]
  pub div_id: Option<DivId>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Run Properties for the Paragraph Mark
  #[sdk(child(qname = "w:rPr"))]
  pub paragraph_mark_run_properties: Option<std::boxed::Box<ParagraphMarkRunProperties>>,
  /// Section Properties
  #[sdk(child(qname = "w:sectPr"))]
  pub section_properties: Option<std::boxed::Box<SectionProperties>>,
  /// Defines the ParagraphPropertiesChange Class.
  #[sdk(child(qname = "w:pPrChange"))]
  pub paragraph_properties_change: Option<std::boxed::Box<ParagraphPropertiesChange>>,
}
/// Defines the Control Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:control")]
pub struct Control {
  /// Unique Name for Embedded Control
  #[sdk(attr(qname = "w:name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Associated VML Data Reference
  #[sdk(attr(qname = "w:shapeid"))]
  #[sdk(string_length(max = 254u32))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// Embedded Control Properties Relationship Reference
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Previous Table Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tblGrid")]
pub struct PreviousTableGrid {
  /// Grid Column Definition.
  #[sdk(child(qname = "w:gridCol"))]
  pub grid_column: Vec<GridColumn>,
}
/// Defines the ObjectEmbed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:objectEmbed")]
pub struct ObjectEmbed {
  /// drawAspect
  #[sdk(attr(qname = "w:drawAspect"))]
  pub draw_aspect: Option<ObjectDrawAspect>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// progId
  #[sdk(attr(qname = "w:progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// shapeId
  #[sdk(attr(qname = "w:shapeId"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// fieldCodes
  #[sdk(attr(qname = "w:fieldCodes"))]
  pub field_codes: Option<crate::simple_type::StringValue>,
}
/// Defines the ObjectLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:objectLink")]
pub struct ObjectLink {
  /// updateMode
  #[sdk(attr(qname = "w:updateMode"))]
  pub update_mode: ObjectUpdateMode,
  /// lockedField
  #[sdk(attr(qname = "w:lockedField"))]
  pub locked_field: Option<crate::simple_type::OnOffValue>,
  /// drawAspect
  #[sdk(attr(qname = "w:drawAspect"))]
  pub draw_aspect: Option<ObjectDrawAspect>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// progId
  #[sdk(attr(qname = "w:progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// shapeId
  #[sdk(attr(qname = "w:shapeId"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// fieldCodes
  #[sdk(attr(qname = "w:fieldCodes"))]
  pub field_codes: Option<crate::simple_type::StringValue>,
}
/// Defines the Lock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lock")]
pub struct Lock {
  /// Locking Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<LockingValues>,
}
/// Defines the SdtPlaceholder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:placeholder")]
pub struct SdtPlaceholder {
  /// Document Part Reference
  #[sdk(child(qname = "w:docPart"))]
  pub doc_part_reference: Option<DocPartReference>,
}
/// Defines the DataBinding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dataBinding")]
pub struct DataBinding {
  /// XML Namespace Prefix Mappings
  #[sdk(attr(qname = "w:prefixMappings"))]
  pub prefix_mappings: Option<crate::simple_type::StringValue>,
  /// XPath
  #[sdk(attr(qname = "w:xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Custom XML Data Storage ID
  #[sdk(attr(qname = "w:storeItemID"))]
  pub store_item_id: Option<crate::simple_type::StringValue>,
}
/// Defines the SdtContentComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:comboBox")]
pub struct SdtContentComboBox {
  /// Combo Box Last Saved Value
  #[sdk(attr(qname = "w:lastValue"))]
  pub last_value: Option<crate::simple_type::StringValue>,
  /// Combo Box List Item.
  #[sdk(child(qname = "w:listItem"))]
  pub list_item: Vec<ListItem>,
}
/// Defines the SdtContentDate Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:date")]
pub struct SdtContentDate {
  /// Last Known Date in XML Schema DateTime Format
  #[sdk(attr(qname = "w:fullDate"))]
  pub full_date: Option<crate::simple_type::DateTimeValue>,
  /// Date Display Mask
  #[sdk(child(qname = "w:dateFormat"))]
  pub date_format: Option<DateFormat>,
  /// Date Picker Language ID
  #[sdk(child(qname = "w:lid"))]
  pub language_id: Option<LanguageId>,
  /// Custom XML Data Date Storage Format
  #[sdk(child(qname = "w:storeMappedDataAs"))]
  pub sdt_date_mapping_type: Option<SdtDateMappingType>,
  /// Date Picker Calendar Type
  #[sdk(child(qname = "w:calendar"))]
  pub calendar: Option<Calendar>,
}
/// Defines the SdtContentDocPartObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartObj")]
pub struct SdtContentDocPartObject {
  /// Document Part Gallery Filter
  #[sdk(child(qname = "w:docPartGallery"))]
  pub doc_part_gallery: Option<DocPartGallery>,
  /// Document Part Category Filter
  #[sdk(child(qname = "w:docPartCategory"))]
  pub doc_part_category: Option<DocPartCategory>,
  /// Built-In Document Part
  #[sdk(child(qname = "w:docPartUnique"))]
  pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtContentDocPartList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docPartList")]
pub struct SdtContentDocPartList {
  /// Document Part Gallery Filter
  #[sdk(child(qname = "w:docPartGallery"))]
  pub doc_part_gallery: Option<DocPartGallery>,
  /// Document Part Category Filter
  #[sdk(child(qname = "w:docPartCategory"))]
  pub doc_part_category: Option<DocPartCategory>,
  /// Built-In Document Part
  #[sdk(child(qname = "w:docPartUnique"))]
  pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtContentDropDownList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:dropDownList")]
pub struct SdtContentDropDownList {
  /// Drop-down List Last Saved Value
  #[sdk(attr(qname = "w:lastValue"))]
  pub last_value: Option<crate::simple_type::StringValue>,
  /// Combo Box List Item.
  #[sdk(child(qname = "w:listItem"))]
  pub list_item: Vec<ListItem>,
}
/// Defines the SdtContentText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:text")]
pub struct SdtContentText {
  /// Allow Soft Line Breaks
  #[sdk(attr(qname = "w:multiLine"))]
  pub multi_line: Option<crate::simple_type::OnOffValue>,
}
/// Write Protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:writeProtection")]
pub struct WriteProtection {
  /// Recommend Write Protection in User Interface
  #[sdk(attr(qname = "w:recommended"))]
  pub recommended: Option<crate::simple_type::OnOffValue>,
  /// Cryptographic Provider Type
  #[sdk(attr(qname = "w:cryptProviderType"))]
  pub cryptographic_provider_type: Option<CryptProviderValues>,
  /// Cryptographic Algorithm Class
  #[sdk(attr(qname = "w:cryptAlgorithmClass"))]
  pub cryptographic_algorithm_class: Option<CryptAlgorithmClassValues>,
  /// Cryptographic Algorithm Type
  #[sdk(attr(qname = "w:cryptAlgorithmType"))]
  pub cryptographic_algorithm_type: Option<CryptAlgorithmValues>,
  /// Cryptographic Hashing Algorithm
  #[sdk(attr(qname = "w:cryptAlgorithmSid"))]
  pub cryptographic_algorithm_sid: Option<crate::simple_type::Int32Value>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = "w:cryptSpinCount"))]
  #[sdk(number_range(max = 5000000, min_inclusive = false))]
  pub cryptographic_spin_count: Option<crate::simple_type::UInt32Value>,
  /// Cryptographic Provider
  #[sdk(attr(qname = "w:cryptProvider"))]
  pub cryptographic_provider: Option<crate::simple_type::StringValue>,
  /// Cryptographic Algorithm Extensibility
  #[sdk(attr(qname = "w:algIdExt"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub algorithm_id_extensibility: Option<crate::simple_type::HexBinaryValue>,
  /// Algorithm Extensibility Source
  #[sdk(attr(qname = "w:algIdExtSource"))]
  pub algorithm_id_extensibility_source: Option<crate::simple_type::StringValue>,
  /// Cryptographic Provider Type Extensibility
  #[sdk(attr(qname = "w:cryptProviderTypeExt"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub cryptographic_provider_type_extensibility: Option<crate::simple_type::HexBinaryValue>,
  /// Provider Type Extensibility Source
  #[sdk(attr(qname = "w:cryptProviderTypeExtSource"))]
  pub cryptographic_provider_type_ext_source: Option<crate::simple_type::StringValue>,
  /// Password Hash
  #[sdk(attr(qname = "w:hash"))]
  pub hash: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt for Password Verifier
  #[sdk(attr(qname = "w:salt"))]
  pub salt: Option<crate::simple_type::Base64BinaryValue>,
  /// algorithmName
  #[sdk(attr(qname = "w:algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(qname = "w:hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(qname = "w:saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(qname = "w:spinCount"))]
  pub spin_count: Option<crate::simple_type::Int32Value>,
}
/// Document View Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:view")]
pub struct View {
  /// Document View Setting  Value
  #[sdk(attr(qname = "w:val"))]
  pub val: ViewValues,
}
/// Magnification Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:zoom")]
pub struct Zoom {
  /// Zoom Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<PresetZoomValues>,
  /// Zoom Percentage
  #[sdk(attr(qname = "w:percent"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub percent: Option<crate::simple_type::DecimalNumberOrPercentValue>,
}
/// Grammar Checking Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:activeWritingStyle")]
pub struct ActiveWritingStyle {
  /// Writing Style Language
  #[sdk(attr(qname = "w:lang"))]
  #[sdk(string_length(max = 84u32))]
  pub language: crate::simple_type::StringValue,
  /// Grammatical Engine ID
  #[sdk(attr(qname = "w:vendorID"))]
  pub vendor_id: crate::simple_type::UInt16Value,
  /// Grammatical Check Engine Version
  #[sdk(attr(qname = "w:dllVersion"))]
  #[sdk(number_range(range = 0..))]
  pub dll_version: crate::simple_type::Int32Value,
  /// Natural Language Grammar Check
  #[sdk(attr(qname = "w:nlCheck"))]
  pub natural_language_grammar_check: Option<crate::simple_type::OnOffValue>,
  /// Check Stylistic Rules With Grammar
  #[sdk(attr(qname = "w:checkStyle"))]
  pub check_style: crate::simple_type::OnOffValue,
  /// Application Name
  #[sdk(attr(qname = "w:appName"))]
  pub application_name: crate::simple_type::StringValue,
}
/// Spelling and Grammatical Checking State.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:proofState")]
pub struct ProofState {
  /// Spell Checking State
  #[sdk(attr(qname = "w:spelling"))]
  pub spelling: Option<ProofingStateValues>,
  /// Grammatical Checking State
  #[sdk(attr(qname = "w:grammar"))]
  pub grammar: Option<ProofingStateValues>,
}
/// Suggested Filtering for List of Document Styles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:stylePaneFormatFilter")]
pub struct StylePaneFormatFilter {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub w_val: Option<crate::simple_type::HexBinaryValue>,
  /// allStyles
  #[sdk(attr(qname = "w:allStyles"))]
  pub all_styles: Option<crate::simple_type::OnOffValue>,
  /// customStyles
  #[sdk(attr(qname = "w:customStyles"))]
  pub custom_styles: Option<crate::simple_type::OnOffValue>,
  /// latentStyles
  #[sdk(attr(qname = "w:latentStyles"))]
  pub latent_styles: Option<crate::simple_type::OnOffValue>,
  /// stylesInUse
  #[sdk(attr(qname = "w:stylesInUse"))]
  pub styles_in_use: Option<crate::simple_type::OnOffValue>,
  /// headingStyles
  #[sdk(attr(qname = "w:headingStyles"))]
  pub heading_styles: Option<crate::simple_type::OnOffValue>,
  /// numberingStyles
  #[sdk(attr(qname = "w:numberingStyles"))]
  pub numbering_styles: Option<crate::simple_type::OnOffValue>,
  /// tableStyles
  #[sdk(attr(qname = "w:tableStyles"))]
  pub table_styles: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnRuns
  #[sdk(attr(qname = "w:directFormattingOnRuns"))]
  pub direct_formatting_on_runs: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnParagraphs
  #[sdk(attr(qname = "w:directFormattingOnParagraphs"))]
  pub direct_formatting_on_paragraphs: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnNumbering
  #[sdk(attr(qname = "w:directFormattingOnNumbering"))]
  pub direct_formatting_on_numbering: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnTables
  #[sdk(attr(qname = "w:directFormattingOnTables"))]
  pub direct_formatting_on_tables: Option<crate::simple_type::OnOffValue>,
  /// clearFormatting
  #[sdk(attr(qname = "w:clearFormatting"))]
  pub clear_formatting: Option<crate::simple_type::OnOffValue>,
  /// top3HeadingStyles
  #[sdk(attr(qname = "w:top3HeadingStyles"))]
  pub top3_heading_styles: Option<crate::simple_type::OnOffValue>,
  /// visibleStyles
  #[sdk(attr(qname = "w:visibleStyles"))]
  pub visible_styles: Option<crate::simple_type::OnOffValue>,
  /// alternateStyleNames
  #[sdk(attr(qname = "w:alternateStyleNames"))]
  pub alternate_style_names: Option<crate::simple_type::OnOffValue>,
}
/// Suggested Sorting for List of Document Styles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:stylePaneSortMethod")]
pub struct StylePaneSortMethods {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(
    source = 1u32,
    union = 0u64,
    type_name = "w:ST_StylePaneSortMethods_O12",
    min = 2u32,
    max = 2u32,
  ))]
  #[sdk(
        string_set(
            source = 2u32,
            union = 0u64,
            values = &["0000",
            "name",
            "0001",
            "priority",
            "0002",
            "font",
            "0003",
            "basedOn",
            "0004",
            "type",
            "0005",
            "default"]
        )
    )]
  pub w_val: crate::simple_type::StringValue,
}
/// Document Classification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:documentType")]
pub struct DocumentType {
  /// Document Classification Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocumentTypeValues,
}
/// Mail Merge Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:mailMerge")]
pub struct MailMerge {
  /// Source Document Type
  #[sdk(child(qname = "w:mainDocumentType"))]
  pub main_document_type: Option<MainDocumentType>,
  /// Query Contains Link to External Query File
  #[sdk(child(qname = "w:linkToQuery"))]
  pub link_to_query: Option<LinkToQuery>,
  /// Data Source Type
  #[sdk(child(qname = "w:dataType"))]
  pub data_type: Option<DataType>,
  /// Data Source Connection String
  #[sdk(child(qname = "w:connectString"))]
  pub connect_string: Option<ConnectString>,
  /// Query For Data Source Records To Merge
  #[sdk(child(qname = "w:query"))]
  pub query: Option<Query>,
  /// Data Source File Path
  #[sdk(child(qname = "w:dataSource"))]
  pub data_source_reference: Option<DataSourceReference>,
  /// Header Definition File Path
  #[sdk(child(qname = "w:headerSource"))]
  pub header_source: Option<HeaderSource>,
  /// Remove Blank Lines from Merged Documents
  #[sdk(child(qname = "w:doNotSuppressBlankLines"))]
  pub do_not_suppress_blank_lines: Option<DoNotSuppressBlankLines>,
  /// Merged Document Destination
  #[sdk(child(qname = "w:destination"))]
  pub destination: Option<Destination>,
  /// Column Containing E-mail Address
  #[sdk(child(qname = "w:addressFieldName"))]
  pub address_field_name: Option<AddressFieldName>,
  /// Merged E-mail or Fax Subject Line
  #[sdk(child(qname = "w:mailSubject"))]
  pub mail_subject: Option<MailSubject>,
  /// Merged Document To E-Mail Attachment
  #[sdk(child(qname = "w:mailAsAttachment"))]
  pub mail_as_attachment: Option<MailAsAttachment>,
  /// View Merged Data Within Document
  #[sdk(child(qname = "w:viewMergedData"))]
  pub view_merged_data: Option<ViewMergedData>,
  /// Record Currently Displayed In Merged Document
  #[sdk(child(qname = "w:activeRecord"))]
  pub active_record: Option<ActiveRecord>,
  /// Mail Merge Error Reporting Setting
  #[sdk(child(qname = "w:checkErrors"))]
  pub check_errors: Option<CheckErrors>,
  /// Office Data Source Object Settings
  #[sdk(child(qname = "w:odso"))]
  pub data_source_object: Option<std::boxed::Box<DataSourceObject>>,
}
/// Visibility of Annotation Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:revisionView")]
pub struct RevisionView {
  /// Display Visual Indicator Of Markup Area
  #[sdk(attr(qname = "w:markup"))]
  pub markup: Option<crate::simple_type::OnOffValue>,
  /// Display Comments
  #[sdk(attr(qname = "w:comments"))]
  pub comments: Option<crate::simple_type::OnOffValue>,
  /// Display Content Revisions
  #[sdk(attr(qname = "w:insDel"))]
  pub display_revision: Option<crate::simple_type::OnOffValue>,
  /// Display Formatting Revisions
  #[sdk(attr(qname = "w:formatting"))]
  pub formatting: Option<crate::simple_type::OnOffValue>,
  /// Display Ink Annotations
  #[sdk(attr(qname = "w:inkAnnotations"))]
  pub ink_annotations: Option<crate::simple_type::OnOffValue>,
}
/// Document Editing Restrictions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:documentProtection")]
pub struct DocumentProtection {
  /// Document Editing Restrictions
  #[sdk(attr(qname = "w:edit"))]
  pub edit: Option<DocumentProtectionValues>,
  /// Only Allow Formatting With Unlocked Styles
  #[sdk(attr(qname = "w:formatting"))]
  pub formatting: Option<crate::simple_type::OnOffValue>,
  /// Enforce Document Protection Settings
  #[sdk(attr(qname = "w:enforcement"))]
  pub enforcement: Option<crate::simple_type::OnOffValue>,
  /// Cryptographic Provider Type
  #[sdk(attr(qname = "w:cryptProviderType"))]
  pub cryptographic_provider_type: Option<CryptProviderValues>,
  /// Cryptographic Algorithm Class
  #[sdk(attr(qname = "w:cryptAlgorithmClass"))]
  pub cryptographic_algorithm_class: Option<CryptAlgorithmClassValues>,
  /// Cryptographic Algorithm Type
  #[sdk(attr(qname = "w:cryptAlgorithmType"))]
  pub cryptographic_algorithm_type: Option<CryptAlgorithmValues>,
  /// Cryptographic Hashing Algorithm
  #[sdk(attr(empty_as_none, qname = "w:cryptAlgorithmSid"))]
  pub cryptographic_algorithm_sid: Option<crate::simple_type::Int32Value>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = "w:cryptSpinCount"))]
  #[sdk(number_range(max = 5000000, min_inclusive = false))]
  pub cryptographic_spin_count: Option<crate::simple_type::UInt32Value>,
  /// Cryptographic Provider
  #[sdk(attr(qname = "w:cryptProvider"))]
  pub cryptographic_provider: Option<crate::simple_type::StringValue>,
  /// Cryptographic Algorithm Extensibility
  #[sdk(attr(qname = "w:algIdExt"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub algorithm_id_extensibility: Option<crate::simple_type::HexBinaryValue>,
  /// Algorithm Extensibility Source
  #[sdk(attr(qname = "w:algIdExtSource"))]
  pub algorithm_id_extensibility_source: Option<crate::simple_type::StringValue>,
  /// Cryptographic Provider Type Extensibility
  #[sdk(attr(qname = "w:cryptProviderTypeExt"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub cryptographic_provider_type_extensibility: Option<crate::simple_type::HexBinaryValue>,
  /// Provider Type Extensibility Source
  #[sdk(attr(qname = "w:cryptProviderTypeExtSource"))]
  pub cryptographic_provider_type_ext_source: Option<crate::simple_type::StringValue>,
  /// Password Hash
  #[sdk(attr(qname = "w:hash"))]
  pub hash: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt for Password Verifier
  #[sdk(attr(qname = "w:salt"))]
  pub salt: Option<crate::simple_type::Base64BinaryValue>,
  /// algorithmName
  #[sdk(attr(qname = "w:algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(qname = "w:hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(qname = "w:saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(qname = "w:spinCount"))]
  pub spin_count: Option<crate::simple_type::Int32Value>,
}
/// Distance Between Automatic Tab Stops.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:defaultTabStop")]
pub struct DefaultTabStop {
  /// Measurement in Twentieths of a Point
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub val: crate::simple_type::TwipsMeasureValue,
}
/// Number of Pages Per Booklet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:bookFoldPrintingSheets")]
pub struct BookFoldPrintingSheets {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int16Value,
}
/// Maximum Number of Consecutively Hyphenated Lines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:consecutiveHyphenLimit")]
pub struct ConsecutiveHyphenLimit {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Percentage of Document to Use When Generating Summary.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:summaryLength")]
pub struct SummaryLength {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 100))]
  pub val: crate::simple_type::Int32Value,
}
/// Distance between Horizontal Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:displayHorizontalDrawingGridEvery")]
pub struct DisplayHorizontalDrawingGrid {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 127))]
  pub val: crate::simple_type::Int32Value,
}
/// Distance between Vertical Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:displayVerticalDrawingGridEvery")]
pub struct DisplayVerticalDrawingGrid {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 127))]
  pub val: crate::simple_type::Int32Value,
}
/// Character-Level Whitespace Compression.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:characterSpacingControl")]
pub struct CharacterSpacingControl {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: CharacterSpacingValues,
}
/// Custom Set of Characters Which Cannot End a Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noLineBreaksAfter")]
pub struct NoLineBreaksAfterKinsoku {
  /// lang
  #[sdk(attr(qname = "w:lang"))]
  #[sdk(string_length(max = 84u32))]
  pub language: crate::simple_type::StringValue,
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 50u32))]
  pub val: crate::simple_type::StringValue,
}
/// Custom Set Of Characters Which Cannot Begin A Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:noLineBreaksBefore")]
pub struct NoLineBreaksBeforeKinsoku {
  /// lang
  #[sdk(attr(qname = "w:lang"))]
  #[sdk(string_length(max = 84u32))]
  pub language: crate::simple_type::StringValue,
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 100u32))]
  pub val: crate::simple_type::StringValue,
}
/// Custom XSL Transform To Use When Saving As XML File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:saveThroughXslt")]
pub struct SaveThroughXslt {
  /// XSL Transformation Location
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Local Identifier for XSL Transform
  #[sdk(attr(qname = "w:solutionID"))]
  pub solution_id: Option<crate::simple_type::StringValue>,
}
/// Default Properties for VML Objects in Header and Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:hdrShapeDefaults")]
pub struct HeaderShapeDefaults {
  #[sdk(
        choice(
            child(variant = ShapeDefaults, qname = "o:shapedefaults"),
            child(variant = ShapeLayout, qname = "o:shapelayout")
        )
    )]
  pub header_shape_defaults_choice: Vec<HeaderShapeDefaultsChoice>,
}
/// Default Properties for VML Objects in Main Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:shapeDefaults")]
pub struct ShapeDefaults {
  #[sdk(
        choice(
            child(variant = ShapeDefaults, qname = "o:shapedefaults"),
            child(variant = ShapeLayout, qname = "o:shapelayout")
        )
    )]
  pub shape_defaults_choice: Vec<ShapeDefaultsChoice>,
}
/// Document-Wide Footnote Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:footnotePr")]
pub struct FootnoteDocumentWideProperties {
  /// Footnote Placement
  #[sdk(child(qname = "w:pos"))]
  pub footnote_position: Option<FootnotePosition>,
  /// Footnote Numbering Format
  #[sdk(child(qname = "w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
  /// Special Footnote List.
  #[sdk(child(qname = "w:footnote"))]
  pub footnote_special_reference: Vec<FootnoteSpecialReference>,
}
/// Document-Wide Endnote Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:endnotePr")]
pub struct EndnoteDocumentWideProperties {
  /// Endnote Placement
  #[sdk(child(qname = "w:pos"))]
  pub endnote_position: Option<EndnotePosition>,
  /// Endnote Numbering Format
  #[sdk(child(qname = "w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
  /// Special Endnote List.
  #[sdk(child(qname = "w:endnote"))]
  pub endnote_special_reference: Vec<EndnoteSpecialReference>,
}
/// Compatibility Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:compat")]
pub struct Compatibility {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Use Simplified Rules For Table Border Conflicts
  #[sdk(child(qname = "w:useSingleBorderforContiguousCells"))]
  pub use_single_border_for_contiguous_cells: Option<UseSingleBorderForContiguousCells>,
  /// Emulate WordPerfect 6.x Paragraph Justification
  #[sdk(child(qname = "w:wpJustification"))]
  pub word_perfect_justification: Option<WordPerfectJustification>,
  /// Do Not Create Custom Tab Stop for Hanging Indent
  #[sdk(child(qname = "w:noTabHangInd"))]
  pub no_tab_hang_indent: Option<NoTabHangIndent>,
  /// Do Not Add Leading Between Lines of Text
  #[sdk(child(qname = "w:noLeading"))]
  pub no_leading: Option<NoLeading>,
  /// Add Additional Space Below Baseline For Underlined East Asian Text
  #[sdk(child(qname = "w:spaceForUL"))]
  pub space_for_underline: Option<SpaceForUnderline>,
  /// Do Not Balance Text Columns within a Section
  #[sdk(child(qname = "w:noColumnBalance"))]
  pub no_column_balance: Option<NoColumnBalance>,
  /// Balance Single Byte and Double Byte Characters
  #[sdk(child(qname = "w:balanceSingleByteDoubleByteWidth"))]
  pub balance_single_byte_double_byte_width: Option<BalanceSingleByteDoubleByteWidth>,
  /// Do Not Center Content on Lines With Exact Line Height
  #[sdk(child(qname = "w:noExtraLineSpacing"))]
  pub no_extra_line_spacing: Option<NoExtraLineSpacing>,
  /// Convert Backslash To Yen Sign When Entered
  #[sdk(child(qname = "w:doNotLeaveBackslashAlone"))]
  pub do_not_leave_backslash_alone: Option<DoNotLeaveBackslashAlone>,
  /// Underline All Trailing Spaces
  #[sdk(child(qname = "w:ulTrailSpace"))]
  pub underline_trailing_spaces: Option<UnderlineTrailingSpaces>,
  /// Don't Justify Lines Ending in Soft Line Break
  #[sdk(child(qname = "w:doNotExpandShiftReturn"))]
  pub do_not_expand_shift_return: Option<DoNotExpandShiftReturn>,
  /// Only Expand/Condense Text By Whole Points
  #[sdk(child(qname = "w:spacingInWholePoints"))]
  pub spacing_in_whole_points: Option<SpacingInWholePoints>,
  /// Emulate Word 6.0 Line Wrapping for East Asian Text
  #[sdk(child(qname = "w:lineWrapLikeWord6"))]
  pub line_wrap_like_word6: Option<LineWrapLikeWord6>,
  /// Print Body Text before Header/Footer Contents
  #[sdk(child(qname = "w:printBodyTextBeforeHeader"))]
  pub print_body_text_before_header: Option<PrintBodyTextBeforeHeader>,
  /// Print Colors as Black And White without Dithering
  #[sdk(child(qname = "w:printColBlack"))]
  pub print_color_black_white: Option<PrintColorBlackWhite>,
  /// Space width
  #[sdk(child(qname = "w:wpSpaceWidth"))]
  pub word_perfect_space_width: Option<WordPerfectSpaceWidth>,
  /// Display Page/Column Breaks Present in Frames
  #[sdk(child(qname = "w:showBreaksInFrames"))]
  pub show_breaks_in_frames: Option<ShowBreaksInFrames>,
  /// Increase Priority Of Font Size During Font Substitution
  #[sdk(child(qname = "w:subFontBySize"))]
  pub sub_font_by_size: Option<SubFontBySize>,
  /// Ignore Exact Line Height for Last Line on Page
  #[sdk(child(qname = "w:suppressBottomSpacing"))]
  pub suppress_bottom_spacing: Option<SuppressBottomSpacing>,
  /// Ignore Minimum and Exact Line Height for First Line on Page
  #[sdk(child(qname = "w:suppressTopSpacing"))]
  pub suppress_top_spacing: Option<SuppressTopSpacing>,
  /// Ignore Minimum Line Height for First Line on Page
  #[sdk(child(qname = "w:suppressSpacingAtTopOfPage"))]
  pub suppress_spacing_at_top_of_page: Option<SuppressSpacingAtTopOfPage>,
  /// Emulate WordPerfect 5.x Line Spacing
  #[sdk(child(qname = "w:suppressTopSpacingWP"))]
  pub suppress_top_spacing_word_perfect: Option<SuppressTopSpacingWordPerfect>,
  /// Do Not Use Space Before On First Line After a Page Break
  #[sdk(child(qname = "w:suppressSpBfAfterPgBrk"))]
  pub suppress_spacing_before_after_page_break: Option<SuppressSpacingBeforeAfterPageBreak>,
  /// Swap Paragraph Borders on Odd Numbered Pages
  #[sdk(child(qname = "w:swapBordersFacingPages"))]
  pub swap_borders_facing_pages: Option<SwapBordersFacingPages>,
  /// Treat Backslash Quotation Delimiter as Two Quotation Marks
  #[sdk(child(qname = "w:convMailMergeEsc"))]
  pub convert_mail_merge_escape: Option<ConvertMailMergeEscape>,
  /// Emulate WordPerfect 6.x Font Height Calculation
  #[sdk(child(qname = "w:truncateFontHeightsLikeWP6"))]
  pub truncate_font_heights_like_word_perfect: Option<TruncateFontHeightsLikeWordPerfect>,
  /// Emulate Word 5.x for the Macintosh Small Caps Formatting
  #[sdk(child(qname = "w:mwSmallCaps"))]
  pub mac_word_small_caps: Option<MacWordSmallCaps>,
  /// Use Printer Metrics To Display Documents
  #[sdk(child(qname = "w:usePrinterMetrics"))]
  pub use_printer_metrics: Option<UsePrinterMetrics>,
  /// Do Not Suppress Paragraph Borders Next To Frames
  #[sdk(child(qname = "w:doNotSuppressParagraphBorders"))]
  pub do_not_suppress_paragraph_borders: Option<DoNotSuppressParagraphBorders>,
  /// Line Wrap Trailing Spaces
  #[sdk(child(qname = "w:wrapTrailSpaces"))]
  pub wrap_trail_spaces: Option<WrapTrailSpaces>,
  /// Emulate Word 6.x/95/97 Footnote Placement
  #[sdk(child(qname = "w:footnoteLayoutLikeWW8"))]
  pub footnote_layout_like_word8: Option<FootnoteLayoutLikeWord8>,
  /// Emulate Word 97 Text Wrapping Around Floating Objects
  #[sdk(child(qname = "w:shapeLayoutLikeWW8"))]
  pub shape_layout_like_word8: Option<ShapeLayoutLikeWord8>,
  /// Align Table Rows Independently
  #[sdk(child(qname = "w:alignTablesRowByRow"))]
  pub align_tables_row_by_row: Option<AlignTablesRowByRow>,
  /// Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned
  #[sdk(child(qname = "w:forgetLastTabAlignment"))]
  pub forget_last_tab_alignment: Option<ForgetLastTabAlignment>,
  /// Add Document Grid Line Pitch To Lines in Table Cells
  #[sdk(child(qname = "w:adjustLineHeightInTable"))]
  pub adjust_line_height_in_table: Option<AdjustLineHeightInTable>,
  /// Emulate Word 95 Full-Width Character Spacing
  #[sdk(child(qname = "w:autoSpaceLikeWord95"))]
  pub auto_space_like_word95: Option<AutoSpaceLikeWord95>,
  /// Do Not Increase Line Height for Raised/Lowered Text
  #[sdk(child(qname = "w:noSpaceRaiseLower"))]
  pub no_space_raise_lower: Option<NoSpaceRaiseLower>,
  /// Use Fixed Paragraph Spacing for HTML Auto Setting
  #[sdk(child(qname = "w:doNotUseHTMLParagraphAutoSpacing"))]
  pub do_not_use_html_paragraph_auto_spacing: Option<DoNotUseHtmlParagraphAutoSpacing>,
  /// Ignore Space Before Table When Deciding If Table Should Wrap Floating Object
  #[sdk(child(qname = "w:layoutRawTableWidth"))]
  pub layout_raw_table_width: Option<LayoutRawTableWidth>,
  /// Allow Table Rows to Wrap Inline Objects Independently
  #[sdk(child(qname = "w:layoutTableRowsApart"))]
  pub layout_table_rows_apart: Option<LayoutTableRowsApart>,
  /// Emulate Word 97 East Asian Line Breaking
  #[sdk(child(qname = "w:useWord97LineBreakRules"))]
  pub use_word97_line_break_rules: Option<UseWord97LineBreakRules>,
  /// Do Not Allow Floating Tables To Break Across Pages
  #[sdk(child(qname = "w:doNotBreakWrappedTables"))]
  pub do_not_break_wrapped_tables: Option<DoNotBreakWrappedTables>,
  /// Do Not Snap to Document Grid in Table Cells with Objects
  #[sdk(child(qname = "w:doNotSnapToGridInCell"))]
  pub do_not_snap_to_grid_in_cell: Option<DoNotSnapToGridInCell>,
  /// Select Field When First or Last Character Is Selected
  #[sdk(child(qname = "w:selectFldWithFirstOrLastChar"))]
  pub select_field_with_first_or_last_char: Option<SelectFieldWithFirstOrLastChar>,
  /// Use Legacy Ethiopic and Amharic Line Breaking Rules
  #[sdk(child(qname = "w:applyBreakingRules"))]
  pub apply_breaking_rules: Option<ApplyBreakingRules>,
  /// Do Not Allow Hanging Punctuation With Character Grid
  #[sdk(child(qname = "w:doNotWrapTextWithPunct"))]
  pub do_not_wrap_text_with_punctuation: Option<DoNotWrapTextWithPunctuation>,
  /// Do Not Compress Compressible Characters When Using Document Grid
  #[sdk(child(qname = "w:doNotUseEastAsianBreakRules"))]
  pub do_not_use_east_asian_break_rules: Option<DoNotUseEastAsianBreakRules>,
  /// Emulate Word 2002 Table Style Rules
  #[sdk(child(qname = "w:useWord2002TableStyleRules"))]
  pub use_word2002_table_style_rules: Option<UseWord2002TableStyleRules>,
  /// Allow Tables to AutoFit Into Page Margins
  #[sdk(child(qname = "w:growAutofit"))]
  pub grow_autofit: Option<GrowAutofit>,
  /// Do Not Bypass East Asian/Complex Script Layout Code
  #[sdk(child(qname = "w:useFELayout"))]
  pub use_far_east_layout: Option<UseFarEastLayout>,
  /// Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text
  #[sdk(child(qname = "w:useNormalStyleForList"))]
  pub use_normal_style_for_list: Option<UseNormalStyleForList>,
  /// Ignore Hanging Indent When Creating Tab Stop After Numbering
  #[sdk(child(qname = "w:doNotUseIndentAsNumberingTabStop"))]
  pub do_not_use_indent_as_numbering_tab_stop: Option<DoNotUseIndentAsNumberingTabStop>,
  /// Use Alternate Set of East Asian Line Breaking Rules
  #[sdk(child(qname = "w:useAltKinsokuLineBreakRules"))]
  pub use_alt_kinsoku_line_break_rules: Option<UseAltKinsokuLineBreakRules>,
  /// Allow Contextual Spacing of Paragraphs in Tables
  #[sdk(child(qname = "w:allowSpaceOfSameStyleInTable"))]
  pub allow_space_of_same_style_in_table: Option<AllowSpaceOfSameStyleInTable>,
  /// Do Not Ignore Floating Objects When Calculating Paragraph Indentation
  #[sdk(child(qname = "w:doNotSuppressIndentation"))]
  pub do_not_suppress_indentation: Option<DoNotSuppressIndentation>,
  /// Do Not AutoFit Tables To Fit Next To Wrapped Objects
  #[sdk(child(qname = "w:doNotAutofitConstrainedTables"))]
  pub do_not_autofit_constrained_tables: Option<DoNotAutofitConstrainedTables>,
  /// Allow Table Columns To Exceed Preferred Widths of Constituent Cells
  #[sdk(child(qname = "w:autofitToFirstFixedWidthCell"))]
  pub autofit_to_first_fixed_width_cell: Option<AutofitToFirstFixedWidthCell>,
  /// Underline Following Character Following Numbering
  #[sdk(child(qname = "w:underlineTabInNumList"))]
  pub underline_tab_in_numbering_list: Option<UnderlineTabInNumberingList>,
  /// Always Use Fixed Width for Hangul Characters
  #[sdk(child(qname = "w:displayHangulFixedWidth"))]
  pub display_hangul_fixed_width: Option<DisplayHangulFixedWidth>,
  /// Always Move Paragraph Mark to Page after a Page Break
  #[sdk(child(qname = "w:splitPgBreakAndParaMark"))]
  pub split_page_break_and_paragraph_mark: Option<SplitPageBreakAndParagraphMark>,
  /// Don't Vertically Align Cells Containing Floating Objects
  #[sdk(child(qname = "w:doNotVertAlignCellWithSp"))]
  pub do_not_vertically_align_cell_with_shape: Option<DoNotVerticallyAlignCellWithShape>,
  /// Don't Break Table Rows Around Floating Tables
  #[sdk(child(qname = "w:doNotBreakConstrainedForcedTable"))]
  pub do_not_break_constrained_forced_table: Option<DoNotBreakConstrainedForcedTable>,
  /// Ignore Vertical Alignment in Textboxes
  #[sdk(child(qname = "w:doNotVertAlignInTxbx"))]
  pub do_not_vertically_align_in_text_box: Option<DoNotVerticallyAlignInTextBox>,
  /// Use ANSI Kerning Pairs from Fonts
  #[sdk(child(qname = "w:useAnsiKerningPairs"))]
  pub use_ansi_kerning_pairs: Option<UseAnsiKerningPairs>,
  /// Use Cached Paragraph Information for Column Balancing
  #[sdk(child(qname = "w:cachedColBalance"))]
  pub cached_column_balance: Option<CachedColumnBalance>,
  /// Defines the CompatibilitySetting Class.
  #[sdk(child(qname = "w:compatSetting"))]
  pub compatibility_setting: Vec<CompatibilitySetting>,
}
/// Document Variables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:docVars")]
pub struct DocumentVariables {
  /// Single Document Variable.
  #[sdk(child(qname = "w:docVar"))]
  pub document_variable: Vec<DocumentVariable>,
}
/// Listing of All Revision Save ID Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:rsids")]
pub struct Rsids {
  /// Original Document Revision Save ID
  #[sdk(child(qname = "w:rsidRoot"))]
  pub rsid_root: Option<RsidRoot>,
  /// Single Session Revision Save ID.
  #[sdk(child(qname = "w:rsid"))]
  pub rsid: Vec<Rsid>,
}
/// Theme Color Mappings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:clrSchemeMapping")]
pub struct ColorSchemeMapping {
  /// Background 1 Theme Color Mapping
  #[sdk(attr(qname = "w:bg1"))]
  pub background1: Option<ColorSchemeIndexValues>,
  /// Text 1 Theme Color Mapping
  #[sdk(attr(qname = "w:t1"))]
  pub text1: Option<ColorSchemeIndexValues>,
  /// Background 2 Theme Color Mapping
  #[sdk(attr(qname = "w:bg2"))]
  pub background2: Option<ColorSchemeIndexValues>,
  /// Text 2 Theme Color Mapping
  #[sdk(attr(qname = "w:t2"))]
  pub text2: Option<ColorSchemeIndexValues>,
  /// Accent 1 Theme Color Mapping
  #[sdk(attr(qname = "w:accent1"))]
  pub accent1: Option<ColorSchemeIndexValues>,
  /// Accent 2 Theme Color Mapping
  #[sdk(attr(qname = "w:accent2"))]
  pub accent2: Option<ColorSchemeIndexValues>,
  /// Accent3 Theme Color Mapping
  #[sdk(attr(qname = "w:accent3"))]
  pub accent3: Option<ColorSchemeIndexValues>,
  /// Accent4 Theme Color Mapping
  #[sdk(attr(qname = "w:accent4"))]
  pub accent4: Option<ColorSchemeIndexValues>,
  /// Accent5 Theme Color Mapping
  #[sdk(attr(qname = "w:accent5"))]
  pub accent5: Option<ColorSchemeIndexValues>,
  /// Accent6 Theme Color Mapping
  #[sdk(attr(qname = "w:accent6"))]
  pub accent6: Option<ColorSchemeIndexValues>,
  /// Hyperlink Theme Color Mapping
  #[sdk(attr(qname = "w:hyperlink"))]
  pub hyperlink: Option<ColorSchemeIndexValues>,
  /// Followed Hyperlink Theme Color Mapping
  #[sdk(attr(qname = "w:followedHyperlink"))]
  pub followed_hyperlink: Option<ColorSchemeIndexValues>,
}
/// Caption Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:captions")]
pub struct Captions {
  /// Single Caption Type Definition.
  #[sdk(child(qname = "w:caption"))]
  pub caption: Vec<Caption>,
  /// Automatic Captioning Settings.
  #[sdk(child(qname = "w:autoCaptions"))]
  pub auto_captions: Option<AutoCaptions>,
}
/// Freeze Document Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:readModeInkLockDown")]
pub struct ReadModeInkLockDown {
  /// Use Actual Pages, Not Virtual Pages
  #[sdk(attr(qname = "w:actualPg"))]
  pub use_actual_pages: Option<crate::simple_type::OnOffValue>,
  /// Virtual Page Width
  #[sdk(attr(qname = "w:w"))]
  pub width: crate::simple_type::UInt32Value,
  /// Virtual Page Height
  #[sdk(attr(qname = "w:h"))]
  pub height: crate::simple_type::UInt32Value,
  /// Font Size Scaling
  #[sdk(attr(qname = "w:fontSz"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 2u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 4u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 5u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub font_size: crate::simple_type::DecimalNumberOrPercentValue,
}
/// Defines the TargetScreenSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:targetScreenSz")]
pub struct TargetScreenSize {
  /// Target Screen Size Value
  #[sdk(attr(qname = "w:val"))]
  pub val: TargetScreenSizeValues,
}
/// Defines the PictureBulletBase Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pict")]
pub struct PictureBulletBase {
  #[sdk(
        choice(
            child(variant = Group, qname = "v:group"),
            child(variant = ImageFile, qname = "v:image"),
            child(variant = Line, qname = "v:line"),
            child(variant = Oval, qname = "v:oval"),
            child(variant = PolyLine, qname = "v:polyline"),
            child(variant = Rectangle, qname = "v:rect"),
            child(variant = RoundRectangle, qname = "v:roundrect"),
            child(variant = Shape, qname = "v:shape"),
            child(variant = Shapetype, qname = "v:shapetype")
        )
    )]
  pub picture_bullet_base_choice: Vec<PictureBulletBaseChoice>,
}
/// Defines the Panose1Number Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:panose1")]
pub struct Panose1Number {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 10u32, max = 10u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the FontCharSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:charset")]
pub struct FontCharSet {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "[0-9a-fA-F]*"))]
  #[sdk(string_length(min = 1u32, max = 2u32))]
  pub val: Option<crate::simple_type::StringValue>,
  /// characterSet
  #[sdk(attr(qname = "w:characterSet"))]
  pub strict_character_set: Option<StrictCharacterSet>,
}
/// Defines the FontFamily Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:family")]
pub struct FontFamily {
  /// Font Family Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FontFamilyValues,
}
/// Defines the Pitch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:pitch")]
pub struct Pitch {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FontPitchValues,
}
/// Defines the FontSignature Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:sig")]
pub struct FontSignature {
  /// First 32 Bits of Unicode Subset Bitfield
  #[sdk(attr(qname = "w:usb0"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub unicode_signature0: crate::simple_type::HexBinaryValue,
  /// Second 32 Bits of Unicode Subset Bitfield
  #[sdk(attr(qname = "w:usb1"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub unicode_signature1: crate::simple_type::HexBinaryValue,
  /// Third 32 Bits of Unicode Subset Bitfield
  #[sdk(attr(qname = "w:usb2"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub unicode_signature2: crate::simple_type::HexBinaryValue,
  /// Fourth 32 Bits of Unicode Subset Bitfield
  #[sdk(attr(qname = "w:usb3"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub unicode_signature3: crate::simple_type::HexBinaryValue,
  /// Lower 32 Bits of Code Page Bit Field
  #[sdk(attr(qname = "w:csb0"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub code_page_signature0: crate::simple_type::HexBinaryValue,
  /// Upper 32 Bits of Code Page Bit Field
  #[sdk(attr(qname = "w:csb1"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub code_page_signature1: crate::simple_type::HexBinaryValue,
}
/// Defines the EmbedRegularFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedRegular")]
pub struct EmbedRegularFont {
  /// fontKey
  #[sdk(attr(qname = "w:fontKey"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub font_key: Option<crate::simple_type::StringValue>,
  /// subsetted
  #[sdk(attr(qname = "w:subsetted"))]
  pub subsetted: Option<crate::simple_type::OnOffValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the EmbedBoldFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedBold")]
pub struct EmbedBoldFont {
  /// fontKey
  #[sdk(attr(qname = "w:fontKey"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub font_key: Option<crate::simple_type::StringValue>,
  /// subsetted
  #[sdk(attr(qname = "w:subsetted"))]
  pub subsetted: Option<crate::simple_type::OnOffValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the EmbedItalicFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedItalic")]
pub struct EmbedItalicFont {
  /// fontKey
  #[sdk(attr(qname = "w:fontKey"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub font_key: Option<crate::simple_type::StringValue>,
  /// subsetted
  #[sdk(attr(qname = "w:subsetted"))]
  pub subsetted: Option<crate::simple_type::OnOffValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the EmbedBoldItalicFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:embedBoldItalic")]
pub struct EmbedBoldItalicFont {
  /// fontKey
  #[sdk(attr(qname = "w:fontKey"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub font_key: Option<crate::simple_type::StringValue>,
  /// subsetted
  #[sdk(attr(qname = "w:subsetted"))]
  pub subsetted: Option<crate::simple_type::OnOffValue>,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the LevelOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:lvlOverride")]
pub struct LevelOverride {
  /// Numbering Level ID
  #[sdk(attr(qname = "w:ilvl"))]
  pub level_index: crate::simple_type::Int32Value,
  /// Numbering Level Starting Value Override
  #[sdk(child(qname = "w:startOverride"))]
  pub start_override_numbering_value: Option<StartOverrideNumberingValue>,
  /// Numbering Level Override Definition
  #[sdk(child(qname = "w:lvl"))]
  pub level: Option<std::boxed::Box<Level>>,
}
/// Structured Document Tag Tab Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:tabIndex")]
pub struct TabIndex {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
#[derive(Clone, Debug, PartialEq)]
pub enum EmbeddedObjectChoice {
  /// Shape Group.
  Group(std::boxed::Box<crate::schemas::v::Group>),
  /// Image File.
  ImageFile(std::boxed::Box<crate::schemas::v::ImageFile>),
  /// Line.
  Line(std::boxed::Box<crate::schemas::v::Line>),
  /// Oval.
  Oval(std::boxed::Box<crate::schemas::v::Oval>),
  /// Multiple Path Line.
  PolyLine(std::boxed::Box<crate::schemas::v::PolyLine>),
  /// Rectangle.
  Rectangle(std::boxed::Box<crate::schemas::v::Rectangle>),
  /// Rounded Rectangle.
  RoundRectangle(std::boxed::Box<crate::schemas::v::RoundRectangle>),
  /// Shape Definition.
  Shape(std::boxed::Box<crate::schemas::v::Shape>),
  /// Shape Template.
  Shapetype(std::boxed::Box<crate::schemas::v::Shapetype>),
  /// Arc Segment.
  Arc(std::boxed::Box<crate::schemas::v::Arc>),
  /// Bezier Curve.
  Curve(std::boxed::Box<crate::schemas::v::Curve>),
  /// Embedded OLE Object.
  OleObject(std::boxed::Box<crate::schemas::o::OleObject>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EmbeddedObjectChoice2 {
  /// Defines the Control Class.
  Control(std::boxed::Box<Control>),
  /// Defines the ObjectEmbed Class.
  ObjectEmbed(std::boxed::Box<ObjectEmbed>),
  /// Defines the ObjectLink Class.
  ObjectLink(std::boxed::Box<ObjectLink>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PictureChoice {
  /// Shape Group.
  Group(std::boxed::Box<crate::schemas::v::Group>),
  /// Image File.
  ImageFile(std::boxed::Box<crate::schemas::v::ImageFile>),
  /// Line.
  Line(std::boxed::Box<crate::schemas::v::Line>),
  /// Oval.
  Oval(std::boxed::Box<crate::schemas::v::Oval>),
  /// Multiple Path Line.
  PolyLine(std::boxed::Box<crate::schemas::v::PolyLine>),
  /// Rectangle.
  Rectangle(std::boxed::Box<crate::schemas::v::Rectangle>),
  /// Rounded Rectangle.
  RoundRectangle(std::boxed::Box<crate::schemas::v::RoundRectangle>),
  /// Shape Definition.
  Shape(std::boxed::Box<crate::schemas::v::Shape>),
  /// Shape Template.
  Shapetype(std::boxed::Box<crate::schemas::v::Shapetype>),
  /// Arc Segment.
  Arc(std::boxed::Box<crate::schemas::v::Arc>),
  /// Bezier Curve.
  Curve(std::boxed::Box<crate::schemas::v::Curve>),
  /// Embedded OLE Object.
  OleObject(std::boxed::Box<crate::schemas::o::OleObject>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FieldCharChoice {
  /// Custom Field Data.
  FieldData(std::boxed::Box<FieldData>),
  /// Form Field Properties.
  FormFieldData(std::boxed::Box<FormFieldData>),
  /// Previous Paragraph Numbering Properties.
  NumberingChange(std::boxed::Box<NumberingChange>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DrawingChoice {
  /// Anchor for Floating DrawingML Object.
  Anchor(std::boxed::Box<crate::schemas::wp::Anchor>),
  /// Inline DrawingML Object.
  Inline(std::boxed::Box<crate::schemas::wp::Inline>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice {
  /// Defines the RunStyle Class.
  RunStyle(std::boxed::Box<RunStyle>),
  /// Defines the RunFonts Class.
  RunFonts(std::boxed::Box<RunFonts>),
  /// Defines the Bold Class.
  Bold(std::boxed::Box<Bold>),
  /// Defines the BoldComplexScript Class.
  BoldComplexScript(std::boxed::Box<BoldComplexScript>),
  /// Defines the Italic Class.
  Italic(std::boxed::Box<Italic>),
  /// Defines the ItalicComplexScript Class.
  ItalicComplexScript(std::boxed::Box<ItalicComplexScript>),
  /// Defines the Caps Class.
  Caps(std::boxed::Box<Caps>),
  /// Defines the SmallCaps Class.
  SmallCaps(std::boxed::Box<SmallCaps>),
  /// Defines the Strike Class.
  Strike(std::boxed::Box<Strike>),
  /// Defines the DoubleStrike Class.
  DoubleStrike(std::boxed::Box<DoubleStrike>),
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the Shadow Class.
  Shadow(std::boxed::Box<Shadow>),
  /// Defines the Emboss Class.
  Emboss(std::boxed::Box<Emboss>),
  /// Defines the Imprint Class.
  Imprint(std::boxed::Box<Imprint>),
  /// Defines the NoProof Class.
  NoProof(std::boxed::Box<NoProof>),
  /// Defines the SnapToGrid Class.
  SnapToGrid(std::boxed::Box<SnapToGrid>),
  /// Defines the Vanish Class.
  Vanish(std::boxed::Box<Vanish>),
  /// Defines the WebHidden Class.
  WebHidden(std::boxed::Box<WebHidden>),
  /// Defines the Color Class.
  Color(std::boxed::Box<Color>),
  /// Defines the Spacing Class.
  Spacing(std::boxed::Box<Spacing>),
  /// Defines the CharacterScale Class.
  CharacterScale(std::boxed::Box<CharacterScale>),
  /// Defines the Kern Class.
  Kern(std::boxed::Box<Kern>),
  /// Defines the Position Class.
  Position(std::boxed::Box<Position>),
  /// Defines the FontSize Class.
  FontSize(std::boxed::Box<FontSize>),
  /// Defines the FontSizeComplexScript Class.
  FontSizeComplexScript(std::boxed::Box<FontSizeComplexScript>),
  /// Defines the Highlight Class.
  Highlight(std::boxed::Box<Highlight>),
  /// Defines the Underline Class.
  Underline(std::boxed::Box<Underline>),
  /// Defines the TextEffect Class.
  TextEffect(std::boxed::Box<TextEffect>),
  /// Defines the Border Class.
  Border(std::boxed::Box<Border>),
  /// Defines the Shading Class.
  Shading(std::boxed::Box<Shading>),
  /// Defines the FitText Class.
  FitText(std::boxed::Box<FitText>),
  /// Defines the VerticalTextAlignment Class.
  VerticalTextAlignment(std::boxed::Box<VerticalTextAlignment>),
  /// Defines the RightToLeftText Class.
  RightToLeftText(std::boxed::Box<RightToLeftText>),
  /// Defines the ComplexScript Class.
  ComplexScript(std::boxed::Box<ComplexScript>),
  /// Defines the Emphasis Class.
  Emphasis(std::boxed::Box<Emphasis>),
  /// Defines the Languages Class.
  Languages(std::boxed::Box<Languages>),
  /// Defines the EastAsianLayout Class.
  EastAsianLayout(std::boxed::Box<EastAsianLayout>),
  /// Defines the SpecVanish Class.
  SpecVanish(std::boxed::Box<SpecVanish>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum InsertedMathControlChoice {
  RunProperties(std::boxed::Box<RunProperties>),
  /// Defines the DeletedMathControl Class.
  DeletedMathControl(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MoveFromMathControlChoice {
  RunProperties(std::boxed::Box<RunProperties>),
  /// Defines the InsertedMathControl Class.
  InsertedMathControl(std::boxed::Box<InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  DeletedMathControl(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MoveToMathControlChoice {
  RunProperties(std::boxed::Box<RunProperties>),
  /// Defines the InsertedMathControl Class.
  InsertedMathControl(std::boxed::Box<InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  DeletedMathControl(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomXmlRubyChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleFieldRubyChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HyperlinkRubyChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunChoice {
  /// Break.
  Break(std::boxed::Box<Break>),
  /// Text.
  Text(std::boxed::Box<Text>),
  /// Deleted Text.
  DeletedText(std::boxed::Box<DeletedText>),
  /// Field Code.
  FieldCode(std::boxed::Box<FieldCode>),
  /// Deleted Field Code.
  DeletedFieldCode(std::boxed::Box<DeletedFieldCode>),
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
  SymbolChar(std::boxed::Box<SymbolChar>),
  /// Page Number Block.
  PageNumber,
  /// Carriage Return.
  CarriageReturn,
  /// Tab Character.
  TabChar,
  /// Inline Embedded Object.
  EmbeddedObject(std::boxed::Box<EmbeddedObject>),
  /// VML Object.
  Picture(std::boxed::Box<Picture>),
  /// Complex Field Character.
  FieldChar(std::boxed::Box<FieldChar>),
  /// Phonetic Guide.
  Ruby(std::boxed::Box<Ruby>),
  /// Footnote Reference.
  FootnoteReference(std::boxed::Box<FootnoteReference>),
  /// Endnote Reference.
  EndnoteReference(std::boxed::Box<EndnoteReference>),
  /// Comment Content Reference Mark.
  CommentReference(std::boxed::Box<CommentReference>),
  /// DrawingML Object.
  Drawing(std::boxed::Box<Drawing>),
  /// Absolute Position Tab Character.
  PositionalTab(std::boxed::Box<PositionalTab>),
  /// Position of Last Calculated Page Break.
  LastRenderedPageBreak,
  /// Extended Symbol Character.
  SymbolCharExt(std::boxed::Box<SymbolChar>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtRunRubyChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum InsertedRunChoice {
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DeletedRunChoice {
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MoveFromRunChoice {
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MoveToRunChoice {
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtRunChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomXmlBlockChoice {
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtBlockChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableChoice2 {
  /// Table Row.
  TableRow(std::boxed::Box<TableRow>),
  /// Row-Level Custom XML Element.
  CustomXmlRow(std::boxed::Box<CustomXmlRow>),
  /// Row-Level Structured Document Tag.
  SdtRow(std::boxed::Box<SdtRow>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableRowChoice {
  /// Table Cell.
  TableCell(std::boxed::Box<TableCell>),
  /// Cell-Level Custom XML Element.
  CustomXmlCell(std::boxed::Box<CustomXmlCell>),
  /// Cell-Level Structured Document Tag.
  SdtCell(std::boxed::Box<SdtCell>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomXmlRowChoice {
  /// Table Row.
  TableRow(std::boxed::Box<TableRow>),
  /// Row-Level Custom XML Element.
  CustomXmlRow(std::boxed::Box<CustomXmlRow>),
  /// Row-Level Structured Document Tag.
  SdtRow(std::boxed::Box<SdtRow>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtRowChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Out-of-place break in table cell.
  Break(std::boxed::Box<Break>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomXmlCellChoice {
  /// Table Cell.
  TableCell(std::boxed::Box<TableCell>),
  /// Cell-Level Custom XML Element.
  CustomXmlCell(std::boxed::Box<CustomXmlCell>),
  /// Cell-Level Structured Document Tag.
  SdtCell(std::boxed::Box<SdtCell>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtCellChoice {
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomXmlRunChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleFieldChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  /// Defines the OfficeMath Class.
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  /// Accent.
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  /// Bar.
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  /// Box Function.
  Box(std::boxed::Box<crate::schemas::m::Box>),
  /// Border-Box Function.
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  /// Delimiter Function.
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  /// Equation-Array Function.
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  /// Fraction Function.
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  /// Function Apply Function.
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  /// Group-Character Function.
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  /// Lower-Limit Function.
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  /// Upper-Limit Function.
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  /// Matrix Function.
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  /// n-ary Operator Function.
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  /// Phantom Function.
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  /// Radical Function.
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  /// Pre-Sub-Superscript Function.
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  /// Subscript Function.
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  /// Sub-Superscript Function.
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  /// Superscript Function.
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Defines the Run Class.
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HyperlinkChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BidirectionalOverrideChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BidirectionalEmbeddingChoice {
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextBoxContentChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HeaderChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FooterChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FontsChoice {
  /// Properties for a Single Font.
  Font(std::boxed::Box<Font>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PreviousTableCellPropertiesChoice {
  /// Table Cell Insertion.
  CellInsertion(std::boxed::Box<CellInsertion>),
  /// Table Cell Deletion.
  CellDeletion(std::boxed::Box<CellDeletion>),
  /// Vertically Merged/Split Table Cells.
  CellMerge(std::boxed::Box<CellMerge>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PreviousTableRowPropertiesChoice {
  /// Defines the ConditionalFormatStyle Class.
  ConditionalFormatStyle(std::boxed::Box<ConditionalFormatStyle>),
  /// Defines the DivId Class.
  DivId(std::boxed::Box<DivId>),
  /// Defines the GridBefore Class.
  GridBefore(std::boxed::Box<GridBefore>),
  /// Defines the GridAfter Class.
  GridAfter(std::boxed::Box<GridAfter>),
  /// Defines the WidthBeforeTableRow Class.
  WidthBeforeTableRow(std::boxed::Box<WidthBeforeTableRow>),
  /// Defines the WidthAfterTableRow Class.
  WidthAfterTableRow(std::boxed::Box<WidthAfterTableRow>),
  /// Defines the TableRowHeight Class.
  TableRowHeight(std::boxed::Box<TableRowHeight>),
  /// Defines the Hidden Class.
  Hidden(std::boxed::Box<Hidden>),
  /// Defines the CantSplit Class.
  CantSplit(std::boxed::Box<CantSplit>),
  /// Defines the TableHeader Class.
  TableHeader(std::boxed::Box<TableHeader>),
  /// Defines the TableCellSpacing Class.
  TableCellSpacing(std::boxed::Box<TableCellSpacing>),
  /// Defines the TableJustification Class.
  TableJustification(std::boxed::Box<TableJustification>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PreviousRunPropertiesChoice {
  /// Defines the RunStyle Class.
  RunStyle(std::boxed::Box<RunStyle>),
  /// Defines the RunFonts Class.
  RunFonts(std::boxed::Box<RunFonts>),
  /// Defines the Bold Class.
  Bold(std::boxed::Box<Bold>),
  /// Defines the BoldComplexScript Class.
  BoldComplexScript(std::boxed::Box<BoldComplexScript>),
  /// Defines the Italic Class.
  Italic(std::boxed::Box<Italic>),
  /// Defines the ItalicComplexScript Class.
  ItalicComplexScript(std::boxed::Box<ItalicComplexScript>),
  /// Defines the Caps Class.
  Caps(std::boxed::Box<Caps>),
  /// Defines the SmallCaps Class.
  SmallCaps(std::boxed::Box<SmallCaps>),
  /// Defines the Strike Class.
  Strike(std::boxed::Box<Strike>),
  /// Defines the DoubleStrike Class.
  DoubleStrike(std::boxed::Box<DoubleStrike>),
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the Shadow Class.
  Shadow(std::boxed::Box<Shadow>),
  /// Defines the Emboss Class.
  Emboss(std::boxed::Box<Emboss>),
  /// Defines the Imprint Class.
  Imprint(std::boxed::Box<Imprint>),
  /// Defines the NoProof Class.
  NoProof(std::boxed::Box<NoProof>),
  /// Defines the SnapToGrid Class.
  SnapToGrid(std::boxed::Box<SnapToGrid>),
  /// Defines the Vanish Class.
  Vanish(std::boxed::Box<Vanish>),
  /// Defines the WebHidden Class.
  WebHidden(std::boxed::Box<WebHidden>),
  /// Defines the Color Class.
  Color(std::boxed::Box<Color>),
  /// Defines the Spacing Class.
  Spacing(std::boxed::Box<Spacing>),
  /// Defines the CharacterScale Class.
  CharacterScale(std::boxed::Box<CharacterScale>),
  /// Defines the Kern Class.
  Kern(std::boxed::Box<Kern>),
  /// Defines the Position Class.
  Position(std::boxed::Box<Position>),
  /// Defines the FontSize Class.
  FontSize(std::boxed::Box<FontSize>),
  /// Defines the FontSizeComplexScript Class.
  FontSizeComplexScript(std::boxed::Box<FontSizeComplexScript>),
  /// Defines the Highlight Class.
  Highlight(std::boxed::Box<Highlight>),
  /// Defines the Underline Class.
  Underline(std::boxed::Box<Underline>),
  /// Defines the TextEffect Class.
  TextEffect(std::boxed::Box<TextEffect>),
  /// Defines the Border Class.
  Border(std::boxed::Box<Border>),
  /// Defines the Shading Class.
  Shading(std::boxed::Box<Shading>),
  /// Defines the FitText Class.
  FitText(std::boxed::Box<FitText>),
  /// Defines the VerticalTextAlignment Class.
  VerticalTextAlignment(std::boxed::Box<VerticalTextAlignment>),
  /// Defines the RightToLeftText Class.
  RightToLeftText(std::boxed::Box<RightToLeftText>),
  /// Defines the ComplexScript Class.
  ComplexScript(std::boxed::Box<ComplexScript>),
  /// Defines the Emphasis Class.
  Emphasis(std::boxed::Box<Emphasis>),
  /// Defines the Languages Class.
  Languages(std::boxed::Box<Languages>),
  /// Defines the EastAsianLayout Class.
  EastAsianLayout(std::boxed::Box<EastAsianLayout>),
  /// Defines the SpecVanish Class.
  SpecVanish(std::boxed::Box<SpecVanish>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PreviousParagraphMarkRunPropertiesChoice {
  /// Defines the ConflictInsertion Class.
  ConflictInsertion(std::boxed::Box<crate::schemas::w14::ConflictInsertion>),
  /// Defines the ConflictDeletion Class.
  ConflictDeletion(std::boxed::Box<crate::schemas::w14::ConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PreviousParagraphMarkRunPropertiesChoice2 {
  /// Defines the RunStyle Class.
  RunStyle(std::boxed::Box<RunStyle>),
  /// Defines the RunFonts Class.
  RunFonts(std::boxed::Box<RunFonts>),
  /// Defines the Bold Class.
  Bold(std::boxed::Box<Bold>),
  /// Defines the BoldComplexScript Class.
  BoldComplexScript(std::boxed::Box<BoldComplexScript>),
  /// Defines the Italic Class.
  Italic(std::boxed::Box<Italic>),
  /// Defines the ItalicComplexScript Class.
  ItalicComplexScript(std::boxed::Box<ItalicComplexScript>),
  /// Defines the Caps Class.
  Caps(std::boxed::Box<Caps>),
  /// Defines the SmallCaps Class.
  SmallCaps(std::boxed::Box<SmallCaps>),
  /// Defines the Strike Class.
  Strike(std::boxed::Box<Strike>),
  /// Defines the DoubleStrike Class.
  DoubleStrike(std::boxed::Box<DoubleStrike>),
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the Shadow Class.
  Shadow(std::boxed::Box<Shadow>),
  /// Defines the Emboss Class.
  Emboss(std::boxed::Box<Emboss>),
  /// Defines the Imprint Class.
  Imprint(std::boxed::Box<Imprint>),
  /// Defines the NoProof Class.
  NoProof(std::boxed::Box<NoProof>),
  /// Defines the SnapToGrid Class.
  SnapToGrid(std::boxed::Box<SnapToGrid>),
  /// Defines the Vanish Class.
  Vanish(std::boxed::Box<Vanish>),
  /// Defines the WebHidden Class.
  WebHidden(std::boxed::Box<WebHidden>),
  /// Defines the Color Class.
  Color(std::boxed::Box<Color>),
  /// Defines the Spacing Class.
  Spacing(std::boxed::Box<Spacing>),
  /// Defines the CharacterScale Class.
  CharacterScale(std::boxed::Box<CharacterScale>),
  /// Defines the Kern Class.
  Kern(std::boxed::Box<Kern>),
  /// Defines the Position Class.
  Position(std::boxed::Box<Position>),
  /// Defines the FontSize Class.
  FontSize(std::boxed::Box<FontSize>),
  /// Defines the FontSizeComplexScript Class.
  FontSizeComplexScript(std::boxed::Box<FontSizeComplexScript>),
  /// Defines the Highlight Class.
  Highlight(std::boxed::Box<Highlight>),
  /// Defines the Underline Class.
  Underline(std::boxed::Box<Underline>),
  /// Defines the TextEffect Class.
  TextEffect(std::boxed::Box<TextEffect>),
  /// Defines the Border Class.
  Border(std::boxed::Box<Border>),
  /// Defines the Shading Class.
  Shading(std::boxed::Box<Shading>),
  /// Defines the FitText Class.
  FitText(std::boxed::Box<FitText>),
  /// Defines the VerticalTextAlignment Class.
  VerticalTextAlignment(std::boxed::Box<VerticalTextAlignment>),
  /// Defines the RightToLeftText Class.
  RightToLeftText(std::boxed::Box<RightToLeftText>),
  /// Defines the ComplexScript Class.
  ComplexScript(std::boxed::Box<ComplexScript>),
  /// Defines the Emphasis Class.
  Emphasis(std::boxed::Box<Emphasis>),
  /// Defines the Languages Class.
  Languages(std::boxed::Box<Languages>),
  /// Defines the EastAsianLayout Class.
  EastAsianLayout(std::boxed::Box<EastAsianLayout>),
  /// Defines the SpecVanish Class.
  SpecVanish(std::boxed::Box<SpecVanish>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphMarkRunPropertiesChoice {
  /// Defines the ConflictInsertion Class.
  ConflictInsertion(std::boxed::Box<crate::schemas::w14::ConflictInsertion>),
  /// Defines the ConflictDeletion Class.
  ConflictDeletion(std::boxed::Box<crate::schemas::w14::ConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphMarkRunPropertiesChoice2 {
  /// Defines the RunStyle Class.
  RunStyle(std::boxed::Box<RunStyle>),
  /// Defines the RunFonts Class.
  RunFonts(std::boxed::Box<RunFonts>),
  /// Defines the Bold Class.
  Bold(std::boxed::Box<Bold>),
  /// Defines the BoldComplexScript Class.
  BoldComplexScript(std::boxed::Box<BoldComplexScript>),
  /// Defines the Italic Class.
  Italic(std::boxed::Box<Italic>),
  /// Defines the ItalicComplexScript Class.
  ItalicComplexScript(std::boxed::Box<ItalicComplexScript>),
  /// Defines the Caps Class.
  Caps(std::boxed::Box<Caps>),
  /// Defines the SmallCaps Class.
  SmallCaps(std::boxed::Box<SmallCaps>),
  /// Defines the Strike Class.
  Strike(std::boxed::Box<Strike>),
  /// Defines the DoubleStrike Class.
  DoubleStrike(std::boxed::Box<DoubleStrike>),
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the Shadow Class.
  Shadow(std::boxed::Box<Shadow>),
  /// Defines the Emboss Class.
  Emboss(std::boxed::Box<Emboss>),
  /// Defines the Imprint Class.
  Imprint(std::boxed::Box<Imprint>),
  /// Defines the NoProof Class.
  NoProof(std::boxed::Box<NoProof>),
  /// Defines the SnapToGrid Class.
  SnapToGrid(std::boxed::Box<SnapToGrid>),
  /// Defines the Vanish Class.
  Vanish(std::boxed::Box<Vanish>),
  /// Defines the WebHidden Class.
  WebHidden(std::boxed::Box<WebHidden>),
  /// Defines the Color Class.
  Color(std::boxed::Box<Color>),
  /// Defines the Spacing Class.
  Spacing(std::boxed::Box<Spacing>),
  /// Defines the CharacterScale Class.
  CharacterScale(std::boxed::Box<CharacterScale>),
  /// Defines the Kern Class.
  Kern(std::boxed::Box<Kern>),
  /// Defines the Position Class.
  Position(std::boxed::Box<Position>),
  /// Defines the FontSize Class.
  FontSize(std::boxed::Box<FontSize>),
  /// Defines the FontSizeComplexScript Class.
  FontSizeComplexScript(std::boxed::Box<FontSizeComplexScript>),
  /// Defines the Highlight Class.
  Highlight(std::boxed::Box<Highlight>),
  /// Defines the Underline Class.
  Underline(std::boxed::Box<Underline>),
  /// Defines the TextEffect Class.
  TextEffect(std::boxed::Box<TextEffect>),
  /// Defines the Border Class.
  Border(std::boxed::Box<Border>),
  /// Defines the Shading Class.
  Shading(std::boxed::Box<Shading>),
  /// Defines the FitText Class.
  FitText(std::boxed::Box<FitText>),
  /// Defines the VerticalTextAlignment Class.
  VerticalTextAlignment(std::boxed::Box<VerticalTextAlignment>),
  /// Defines the RightToLeftText Class.
  RightToLeftText(std::boxed::Box<RightToLeftText>),
  /// Defines the ComplexScript Class.
  ComplexScript(std::boxed::Box<ComplexScript>),
  /// Defines the Emphasis Class.
  Emphasis(std::boxed::Box<Emphasis>),
  /// Defines the Languages Class.
  Languages(std::boxed::Box<Languages>),
  /// Defines the EastAsianLayout Class.
  EastAsianLayout(std::boxed::Box<EastAsianLayout>),
  /// Defines the SpecVanish Class.
  SpecVanish(std::boxed::Box<SpecVanish>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SectionPropertiesChoice {
  /// Header Reference.
  HeaderReference(std::boxed::Box<HeaderReference>),
  /// Footer Reference.
  FooterReference(std::boxed::Box<FooterReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FormFieldDataChoice {
  /// Form Field Name.
  FormFieldName(std::boxed::Box<FormFieldName>),
  /// Form Field Enabled.
  Enabled(std::boxed::Box<Enabled>),
  /// Recalculate Fields When Current Field Is Modified.
  CalculateOnExit(std::boxed::Box<CalculateOnExit>),
  /// Script Function to Execute on Form Field Entry.
  EntryMacro(std::boxed::Box<EntryMacro>),
  /// Script Function to Execute on Form Field Exit.
  ExitMacro(std::boxed::Box<ExitMacro>),
  /// Associated Help Text.
  HelpText(std::boxed::Box<HelpText>),
  /// Associated Status Text.
  StatusText(std::boxed::Box<StatusText>),
  /// Checkbox Form Field Properties.
  CheckBox(std::boxed::Box<CheckBox>),
  /// Drop-Down List Form Field Properties.
  DropDownListFormField(std::boxed::Box<DropDownListFormField>),
  /// Text Box Form Field Properties.
  TextInput(std::boxed::Box<TextInput>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CheckBoxChoice {
  /// Checkbox Form Field Size.
  FormFieldSize(std::boxed::Box<FormFieldSize>),
  /// Automatically Size Form Field.
  AutomaticallySizeFormField(std::boxed::Box<AutomaticallySizeFormField>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RubyContentChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RubyBaseChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtPropertiesChoice {
  /// Run Properties.
  RunProperties(std::boxed::Box<RunProperties>),
  /// Defines the SdtAlias Class.
  SdtAlias(std::boxed::Box<SdtAlias>),
  /// Defines the Lock Class.
  Lock(std::boxed::Box<Lock>),
  /// Defines the SdtPlaceholder Class.
  SdtPlaceholder(std::boxed::Box<SdtPlaceholder>),
  /// Defines the ShowingPlaceholder Class.
  ShowingPlaceholder(std::boxed::Box<ShowingPlaceholder>),
  /// Defines the DataBinding Class.
  WDataBinding(std::boxed::Box<DataBinding>),
  W15DataBinding(std::boxed::Box<crate::schemas::w15::DataBinding>),
  /// Defines the TemporarySdt Class.
  TemporarySdt(std::boxed::Box<TemporarySdt>),
  /// Defines the SdtId Class.
  SdtId(std::boxed::Box<SdtId>),
  /// Defines the Tag Class.
  Tag(std::boxed::Box<Tag>),
  Color(std::boxed::Box<crate::schemas::w15::Color>),
  Appearance(std::boxed::Box<crate::schemas::w15::Appearance>),
  WebExtensionLinked(std::boxed::Box<crate::schemas::w15::WebExtensionLinked>),
  WebExtensionCreated(std::boxed::Box<crate::schemas::w15::WebExtensionCreated>),
  /// Defines the SdtContentEquation Class.
  SdtContentEquation,
  /// Defines the SdtContentComboBox Class.
  SdtContentComboBox(std::boxed::Box<SdtContentComboBox>),
  /// Defines the SdtContentDate Class.
  SdtContentDate(std::boxed::Box<SdtContentDate>),
  /// Defines the SdtContentDocPartObject Class.
  SdtContentDocPartObject(std::boxed::Box<SdtContentDocPartObject>),
  /// Defines the SdtContentDocPartList Class.
  SdtContentDocPartList(std::boxed::Box<SdtContentDocPartList>),
  /// Defines the SdtContentDropDownList Class.
  SdtContentDropDownList(std::boxed::Box<SdtContentDropDownList>),
  /// Defines the SdtContentPicture Class.
  SdtContentPicture,
  /// Defines the SdtContentRichText Class.
  SdtContentRichText,
  /// Defines the SdtContentText Class.
  SdtContentText(std::boxed::Box<SdtContentText>),
  /// Defines the SdtContentCitation Class.
  SdtContentCitation,
  /// Defines the SdtContentGroup Class.
  SdtContentGroup,
  /// Defines the SdtContentBibliography Class.
  SdtContentBibliography,
  /// Defines the EntityPickerEmpty Class.
  EntityPickerEmpty,
  SdtContentCheckBox(std::boxed::Box<crate::schemas::w14::SdtContentCheckBox>),
  SdtRepeatedSection(std::boxed::Box<crate::schemas::w15::SdtRepeatedSection>),
  /// Defines the SdtRepeatedSectionItem Class.
  SdtRepeatedSectionItem,
  /// Structured Document Tag Tab Index.
  TabIndex(std::boxed::Box<TabIndex>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtContentBlockChoice {
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtContentRunChoice {
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Defines the CustomXmlRun Class.
  CustomXmlRun(std::boxed::Box<CustomXmlRun>),
  /// Defines the SimpleField Class.
  SimpleField(std::boxed::Box<SimpleField>),
  /// Defines the Hyperlink Class.
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Defines the SdtRun Class.
  SdtRun(std::boxed::Box<SdtRun>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<BidirectionalEmbedding>),
  /// Anchor for Subdocument Location.
  SubDocumentReference(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtContentRunRubyChoice {
  /// Defines the CustomXmlRuby Class.
  CustomXmlRuby(std::boxed::Box<CustomXmlRuby>),
  /// Defines the SimpleFieldRuby Class.
  SimpleFieldRuby(std::boxed::Box<SimpleFieldRuby>),
  /// Defines the HyperlinkRuby Class.
  HyperlinkRuby(std::boxed::Box<HyperlinkRuby>),
  /// Phonetic Guide Text Run.
  WRun(std::boxed::Box<Run>),
  /// Defines the SdtRunRuby Class.
  SdtRunRuby(std::boxed::Box<SdtRunRuby>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Accent(std::boxed::Box<crate::schemas::m::Accent>),
  Bar(std::boxed::Box<crate::schemas::m::Bar>),
  Box(std::boxed::Box<crate::schemas::m::Box>),
  BorderBox(std::boxed::Box<crate::schemas::m::BorderBox>),
  Delimiter(std::boxed::Box<crate::schemas::m::Delimiter>),
  EquationArray(std::boxed::Box<crate::schemas::m::EquationArray>),
  Fraction(std::boxed::Box<crate::schemas::m::Fraction>),
  MathFunction(std::boxed::Box<crate::schemas::m::MathFunction>),
  GroupChar(std::boxed::Box<crate::schemas::m::GroupChar>),
  LimitLower(std::boxed::Box<crate::schemas::m::LimitLower>),
  LimitUpper(std::boxed::Box<crate::schemas::m::LimitUpper>),
  Matrix(std::boxed::Box<crate::schemas::m::Matrix>),
  Nary(std::boxed::Box<crate::schemas::m::Nary>),
  Phantom(std::boxed::Box<crate::schemas::m::Phantom>),
  Radical(std::boxed::Box<crate::schemas::m::Radical>),
  PreSubSuper(std::boxed::Box<crate::schemas::m::PreSubSuper>),
  Subscript(std::boxed::Box<crate::schemas::m::Subscript>),
  SubSuperscript(std::boxed::Box<crate::schemas::m::SubSuperscript>),
  Superscript(std::boxed::Box<crate::schemas::m::Superscript>),
  MRun(std::boxed::Box<crate::schemas::m::Run>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtContentCellChoice {
  /// Table Cell.
  TableCell(std::boxed::Box<TableCell>),
  /// Cell-Level Custom XML Element.
  CustomXmlCell(std::boxed::Box<CustomXmlCell>),
  /// Cell-Level Structured Document Tag.
  SdtCell(std::boxed::Box<SdtCell>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SdtContentRowChoice {
  /// Table Row.
  TableRow(std::boxed::Box<TableRow>),
  /// Row-Level Custom XML Element.
  CustomXmlRow(std::boxed::Box<CustomXmlRow>),
  /// Row-Level Structured Document Tag.
  SdtRow(std::boxed::Box<SdtRow>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
  CustomXmlConflictInsertionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeStart>,
  ),
  CustomXmlConflictInsertionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictInsertionRangeEnd>,
  ),
  CustomXmlConflictDeletionRangeStart(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeStart>,
  ),
  CustomXmlConflictDeletionRangeEnd(
    std::boxed::Box<crate::schemas::w14::CustomXmlConflictDeletionRangeEnd>,
  ),
  /// Inserted Run Content.
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellPropertiesChoice {
  /// Table Cell Insertion.
  CellInsertion(std::boxed::Box<CellInsertion>),
  /// Table Cell Deletion.
  CellDeletion(std::boxed::Box<CellDeletion>),
  /// Vertically Merged/Split Table Cells.
  CellMerge(std::boxed::Box<CellMerge>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FramesetChoice {
  /// Nested Frameset Definition.
  Frameset(std::boxed::Box<Frameset>),
  /// Single Frame Properties.
  Frame(std::boxed::Box<Frame>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum NumberingPictureBulletChoice {
  /// Defines the PictureBulletBase Class.
  PictureBulletBase(std::boxed::Box<PictureBulletBase>),
  /// DrawingML Object.
  Drawing(std::boxed::Box<Drawing>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableStyleConditionalFormattingTableRowPropertiesChoice {
  /// Defines the Hidden Class.
  Hidden(std::boxed::Box<Hidden>),
  /// Defines the CantSplit Class.
  CantSplit(std::boxed::Box<CantSplit>),
  /// Defines the TableHeader Class.
  TableHeader(std::boxed::Box<TableHeader>),
  /// Defines the TableCellSpacing Class.
  TableCellSpacing(std::boxed::Box<TableCellSpacing>),
  /// Defines the TableJustification Class.
  TableJustification(std::boxed::Box<TableJustification>),
  /// Defines the ConditionalFormatStyle Class.
  ConditionalFormatStyle(std::boxed::Box<ConditionalFormatStyle>),
  /// Defines the DivId Class.
  DivId(std::boxed::Box<DivId>),
  /// Defines the GridBefore Class.
  GridBefore(std::boxed::Box<GridBefore>),
  /// Defines the GridAfter Class.
  GridAfter(std::boxed::Box<GridAfter>),
  /// Defines the WidthBeforeTableRow Class.
  WidthBeforeTableRow(std::boxed::Box<WidthBeforeTableRow>),
  /// Defines the WidthAfterTableRow Class.
  WidthAfterTableRow(std::boxed::Box<WidthAfterTableRow>),
  /// Defines the TableRowHeight Class.
  TableRowHeight(std::boxed::Box<TableRowHeight>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CommentChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FootnoteChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndnoteChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DocPartBodyChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BodyChoice {
  /// Defines the AltChunk Class.
  AltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  CustomXmlBlock(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  SdtBlock(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  Paragraph(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  Table(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  ProofError(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  PermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  PermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  BookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  BookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  CommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  CommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Defines the MoveFromRangeStart Class.
  MoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
  /// Defines the MoveFromRangeEnd Class.
  MoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
  /// Defines the MoveToRangeStart Class.
  MoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
  /// Defines the MoveToRangeEnd Class.
  MoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
  /// Defines the CustomXmlInsRangeStart Class.
  CustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
  /// Defines the CustomXmlInsRangeEnd Class.
  CustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
  /// Defines the CustomXmlDelRangeStart Class.
  CustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
  /// Defines the CustomXmlDelRangeEnd Class.
  CustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
  /// Defines the CustomXmlMoveFromRangeStart Class.
  CustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
  /// Defines the CustomXmlMoveFromRangeEnd Class.
  CustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
  /// Defines the CustomXmlMoveToRangeStart Class.
  CustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
  /// Defines the CustomXmlMoveToRangeEnd Class.
  CustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
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
  InsertedRun(std::boxed::Box<InsertedRun>),
  /// Deleted Run Content.
  DeletedRun(std::boxed::Box<DeletedRun>),
  /// Move Source Run Content.
  MoveFromRun(std::boxed::Box<MoveFromRun>),
  /// Move Destination Run Content.
  MoveToRun(std::boxed::Box<MoveToRun>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  /// Defines the RunConflictInsertion Class.
  RunConflictInsertion(std::boxed::Box<crate::schemas::w14::RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<crate::schemas::w14::RunConflictDeletion>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableRowPropertiesChoice {
  /// Defines the ConditionalFormatStyle Class.
  ConditionalFormatStyle(std::boxed::Box<ConditionalFormatStyle>),
  /// Defines the DivId Class.
  DivId(std::boxed::Box<DivId>),
  /// Defines the GridBefore Class.
  GridBefore(std::boxed::Box<GridBefore>),
  /// Defines the GridAfter Class.
  GridAfter(std::boxed::Box<GridAfter>),
  /// Defines the WidthBeforeTableRow Class.
  WidthBeforeTableRow(std::boxed::Box<WidthBeforeTableRow>),
  /// Defines the WidthAfterTableRow Class.
  WidthAfterTableRow(std::boxed::Box<WidthAfterTableRow>),
  /// Defines the TableRowHeight Class.
  TableRowHeight(std::boxed::Box<TableRowHeight>),
  /// Defines the Hidden Class.
  Hidden(std::boxed::Box<Hidden>),
  /// Defines the CantSplit Class.
  CantSplit(std::boxed::Box<CantSplit>),
  /// Defines the TableHeader Class.
  TableHeader(std::boxed::Box<TableHeader>),
  /// Defines the TableCellSpacing Class.
  TableCellSpacing(std::boxed::Box<TableCellSpacing>),
  /// Defines the TableJustification Class.
  TableJustification(std::boxed::Box<TableJustification>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableRowPropertiesChoice2 {
  /// Defines the ConflictInsertion Class.
  ConflictInsertion(std::boxed::Box<crate::schemas::w14::ConflictInsertion>),
  /// Defines the ConflictDeletion Class.
  ConflictDeletion(std::boxed::Box<crate::schemas::w14::ConflictDeletion>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HeaderShapeDefaultsChoice {
  /// New Shape Defaults.
  ShapeDefaults(std::boxed::Box<crate::schemas::o::ShapeDefaults>),
  /// Shape Layout Properties.
  ShapeLayout(std::boxed::Box<crate::schemas::o::ShapeLayout>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeDefaultsChoice {
  /// New Shape Defaults.
  ShapeDefaults(std::boxed::Box<crate::schemas::o::ShapeDefaults>),
  /// Shape Layout Properties.
  ShapeLayout(std::boxed::Box<crate::schemas::o::ShapeLayout>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PictureBulletBaseChoice {
  Group(std::boxed::Box<crate::schemas::v::Group>),
  ImageFile(std::boxed::Box<crate::schemas::v::ImageFile>),
  Line(std::boxed::Box<crate::schemas::v::Line>),
  Oval(std::boxed::Box<crate::schemas::v::Oval>),
  PolyLine(std::boxed::Box<crate::schemas::v::PolyLine>),
  Rectangle(std::boxed::Box<crate::schemas::v::Rectangle>),
  RoundRectangle(std::boxed::Box<crate::schemas::v::RoundRectangle>),
  Shape(std::boxed::Box<crate::schemas::v::Shape>),
  Shapetype(std::boxed::Box<crate::schemas::v::Shapetype>),
}
