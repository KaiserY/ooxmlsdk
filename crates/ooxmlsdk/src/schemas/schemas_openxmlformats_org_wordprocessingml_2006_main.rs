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
  #[sdk(rename = "default")]
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontTypeHintValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "eastAsia")]
  EastAsia,
  #[sdk(rename = "cs")]
  ComplexScript,
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
  #[sdk(rename = "tab")]
  #[default]
  Tab,
  #[sdk(rename = "space")]
  Space,
  #[sdk(rename = "nothing")]
  Nothing,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MultiLevelValues {
  #[sdk(rename = "singleLevel")]
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
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
}
/// Table Cell Insertion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrackChange/w:cellIns")]
pub struct CellInsertion {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:cellDel")]
pub struct CellDeletion {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:customXmlInsRangeStart")]
pub struct CustomXmlInsRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:customXmlDelRangeStart")]
pub struct CustomXmlDelRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart")]
pub struct CustomXmlMoveFromRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart")]
pub struct CustomXmlMoveToRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:ins")]
pub struct Inserted {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:del")]
pub struct Deleted {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:moveFrom")]
pub struct MoveFrom {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TrackChange/w:moveTo")]
pub struct MoveTo {
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_CellMergeTrackChange/w:cellMerge")]
pub struct CellMerge {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_Bookmark/w:bookmarkStart")]
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
#[sdk(qname = "w:CT_MarkupRange/w:bookmarkEnd")]
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
#[sdk(qname = "w:CT_MarkupRange/w:commentRangeStart")]
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
#[sdk(qname = "w:CT_MarkupRange/w:commentRangeEnd")]
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
#[sdk(qname = "w:CT_MarkupRange/w:moveFromRangeEnd")]
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
#[sdk(qname = "w:CT_MarkupRange/w:moveToRangeEnd")]
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
#[sdk(qname = "w:CT_MoveBookmark/w:moveFromRangeStart")]
pub struct MoveFromRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: crate::simple_type::DateTimeValue,
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
#[sdk(qname = "w:CT_MoveBookmark/w:moveToRangeStart")]
pub struct MoveToRangeStart {
  /// author
  #[sdk(attr(qname = "w:author"))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: crate::simple_type::DateTimeValue,
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
#[sdk(qname = "w:CT_Markup/w:customXmlInsRangeEnd")]
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
#[sdk(qname = "w:CT_Markup/w:customXmlDelRangeEnd")]
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
#[sdk(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd")]
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
#[sdk(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd")]
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
#[sdk(qname = "w:CT_Markup/w:commentReference")]
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
#[sdk(qname = "w:CT_String/w:pStyle")]
pub struct ParagraphStyleId {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Date Display Mask.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:dateFormat")]
pub struct DateFormat {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Gallery Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:docPartGallery")]
pub struct DocPartGallery {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Category Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:docPartCategory")]
pub struct DocPartCategory {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Document Part Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:docPart")]
pub struct DocPartReference {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Custom XML Element Placeholder Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:placeholder")]
pub struct CustomXmlPlaceholder {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableCaption Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_String/w:tblCaption")]
pub struct TableCaption {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableDescription Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_String/w:tblDescription")]
pub struct TableDescription {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Name for Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:name")]
pub struct Name {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Predefined Merge Field Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:mappedName")]
pub struct MappedName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// UDL Connection String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:udl")]
pub struct UdlConnectionString {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Table Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:table")]
pub struct DataSourceTableName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Data Source Connection String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:connectString")]
pub struct ConnectString {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Query For Data Source Records To Merge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:query")]
pub struct Query {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Column Containing E-mail Address.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:addressFieldName")]
pub struct AddressFieldName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Merged E-mail or Fax Subject Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:mailSubject")]
pub struct MailSubject {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Frame Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:sz")]
pub struct FrameSize {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Associated Paragraph Style Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:style")]
pub struct StyleId {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Description for Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:description")]
pub struct Description {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the SdtAlias Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:alias")]
pub struct SdtAlias {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the Tag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:tag")]
pub struct Tag {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Attached Custom XML Schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:attachedSchema")]
pub struct AttachedSchema {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Radix Point for Field Code Evaluation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:decimalSymbol")]
pub struct DecimalSymbol {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// List Separator for Field Code Evaluation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:listSeparator")]
pub struct ListSeparator {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the WebPageEncoding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:encoding")]
pub struct WebPageEncoding {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the AltName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String/w:altName")]
pub struct AltName {
  /// String Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the KeepNext Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:keepNext")]
pub struct KeepNext {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the KeepLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:keepLines")]
pub struct KeepLines {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the PageBreakBefore Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:pageBreakBefore")]
pub struct PageBreakBefore {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WidowControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:widowControl")]
pub struct WidowControl {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressLineNumbers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressLineNumbers")]
pub struct SuppressLineNumbers {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressAutoHyphens Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressAutoHyphens")]
pub struct SuppressAutoHyphens {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Kinsoku Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:kinsoku")]
pub struct Kinsoku {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WordWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:wordWrap")]
pub struct WordWrap {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OverflowPunctuation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:overflowPunct")]
pub struct OverflowPunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TopLinePunctuation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:topLinePunct")]
pub struct TopLinePunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AutoSpaceDE Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autoSpaceDE")]
pub struct AutoSpaceDe {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AutoSpaceDN Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autoSpaceDN")]
pub struct AutoSpaceDn {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the BiDi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bidi")]
pub struct BiDi {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AdjustRightIndent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:adjustRightInd")]
pub struct AdjustRightIndent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SnapToGrid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:snapToGrid")]
pub struct SnapToGrid {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ContextualSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:contextualSpacing")]
pub struct ContextualSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the MirrorIndents Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:mirrorIndents")]
pub struct MirrorIndents {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SuppressOverlap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressOverlap")]
pub struct SuppressOverlap {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Bold Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:b")]
pub struct Bold {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the BoldComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bCs")]
pub struct BoldComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Italic Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:i")]
pub struct Italic {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ItalicComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:iCs")]
pub struct ItalicComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Caps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:caps")]
pub struct Caps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SmallCaps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:smallCaps")]
pub struct SmallCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Strike Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:strike")]
pub struct Strike {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoubleStrike Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:dstrike")]
pub struct DoubleStrike {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:outline")]
pub struct Outline {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Shadow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:shadow")]
pub struct Shadow {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Emboss Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:emboss")]
pub struct Emboss {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Imprint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:imprint")]
pub struct Imprint {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NoProof Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noProof")]
pub struct NoProof {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Vanish Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:vanish")]
pub struct Vanish {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the WebHidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:webHidden")]
pub struct WebHidden {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RightToLeftText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:rtl")]
pub struct RightToLeftText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ComplexScript Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:cs")]
pub struct ComplexScript {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the SpecVanish Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:specVanish")]
pub struct SpecVanish {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OfficeMath Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:oMath")]
pub struct OfficeMath {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the Hidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:hidden")]
pub struct Hidden {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FormProtection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:formProt")]
pub struct FormProtection {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NoEndnote Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noEndnote")]
pub struct NoEndnote {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TitlePage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:titlePg")]
pub struct TitlePage {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the GutterOnRight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:rtlGutter")]
pub struct GutterOnRight {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Form Field Enabled.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:enabled")]
pub struct Enabled {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Recalculate Fields When Current Field Is Modified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:calcOnExit")]
pub struct CalculateOnExit {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Size Form Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:sizeAuto")]
pub struct AutomaticallySizeFormField {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Default Checkbox Form Field State.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:default")]
pub struct DefaultCheckBoxFormFieldState {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Checkbox Form Field State.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:checked")]
pub struct Checked {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Keep Source Formatting on Import.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:matchSrc")]
pub struct MatchSource {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Invalidated Field Cache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:dirty")]
pub struct Dirty {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Built-In Document Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:docPartUnique")]
pub struct DocPartUnique {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Record Is Included in Mail Merge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:active")]
pub struct Active {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Country/Region-Based Address Field Ordering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:dynamicAddress")]
pub struct DynamicAddress {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// First Row of Data Source Contains Column Names.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:fHdr")]
pub struct FirstRowHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Query Contains Link to External Query File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:linkToQuery")]
pub struct LinkToQuery {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Blank Lines from Merged Documents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotSuppressBlankLines")]
pub struct DoNotSuppressBlankLines {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Merged Document To E-Mail Attachment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:mailAsAttachment")]
pub struct MailAsAttachment {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// View Merged Data Within Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:viewMergedData")]
pub struct ViewMergedData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display All Levels Using Arabic Numerals.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:isLgl")]
pub struct IsLegalNumberingStyle {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Data for HTML blockquote Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:blockQuote")]
pub struct BlockQuote {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Data for HTML body Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bodyDiv")]
pub struct BodyDiv {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Simplified Rules For Table Border Conflicts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useSingleBorderforContiguousCells")]
pub struct UseSingleBorderForContiguousCells {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 6.x Paragraph Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:wpJustification")]
pub struct WordPerfectJustification {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Create Custom Tab Stop for Hanging Indent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noTabHangInd")]
pub struct NoTabHangIndent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Add Leading Between Lines of Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noLeading")]
pub struct NoLeading {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Add Additional Space Below Baseline For Underlined East Asian Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:spaceForUL")]
pub struct SpaceForUnderline {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Balance Text Columns within a Section.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noColumnBalance")]
pub struct NoColumnBalance {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Balance Single Byte and Double Byte Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:balanceSingleByteDoubleByteWidth")]
pub struct BalanceSingleByteDoubleByteWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Center Content on Lines With Exact Line Height.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noExtraLineSpacing")]
pub struct NoExtraLineSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Convert Backslash To Yen Sign When Entered.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotLeaveBackslashAlone")]
pub struct DoNotLeaveBackslashAlone {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Underline All Trailing Spaces.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:ulTrailSpace")]
pub struct UnderlineTrailingSpaces {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Justify Lines Ending in Soft Line Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotExpandShiftReturn")]
pub struct DoNotExpandShiftReturn {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Expand/Condense Text By Whole Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:spacingInWholePoints")]
pub struct SpacingInWholePoints {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 6.0 Line Wrapping for East Asian Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:lineWrapLikeWord6")]
pub struct LineWrapLikeWord6 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Body Text before Header/Footer Contents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printBodyTextBeforeHeader")]
pub struct PrintBodyTextBeforeHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Colors as Black And White without Dithering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printColBlack")]
pub struct PrintColorBlackWhite {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Space width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:wpSpaceWidth")]
pub struct WordPerfectSpaceWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display Page/Column Breaks Present in Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:showBreaksInFrames")]
pub struct ShowBreaksInFrames {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Increase Priority Of Font Size During Font Substitution.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:subFontBySize")]
pub struct SubFontBySize {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Exact Line Height for Last Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressBottomSpacing")]
pub struct SuppressBottomSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Minimum and Exact Line Height for First Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressTopSpacing")]
pub struct SuppressTopSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Minimum Line Height for First Line on Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressSpacingAtTopOfPage")]
pub struct SuppressSpacingAtTopOfPage {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 5.x Line Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressTopSpacingWP")]
pub struct SuppressTopSpacingWordPerfect {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Space Before On First Line After a Page Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:suppressSpBfAfterPgBrk")]
pub struct SuppressSpacingBeforeAfterPageBreak {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Swap Paragraph Borders on Odd Numbered Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:swapBordersFacingPages")]
pub struct SwapBordersFacingPages {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Treat Backslash Quotation Delimiter as Two Quotation Marks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:convMailMergeEsc")]
pub struct ConvertMailMergeEscape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate WordPerfect 6.x Font Height Calculation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:truncateFontHeightsLikeWP6")]
pub struct TruncateFontHeightsLikeWordPerfect {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 5.x for the Macintosh Small Caps Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:mwSmallCaps")]
pub struct MacWordSmallCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Printer Metrics To Display Documents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:usePrinterMetrics")]
pub struct UsePrinterMetrics {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Suppress Paragraph Borders Next To Frames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotSuppressParagraphBorders")]
pub struct DoNotSuppressParagraphBorders {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Line Wrap Trailing Spaces.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:wrapTrailSpaces")]
pub struct WrapTrailSpaces {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 6.x/95/97 Footnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:footnoteLayoutLikeWW8")]
pub struct FootnoteLayoutLikeWord8 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 97 Text Wrapping Around Floating Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:shapeLayoutLikeWW8")]
pub struct ShapeLayoutLikeWord8 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Align Table Rows Independently.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:alignTablesRowByRow")]
pub struct AlignTablesRowByRow {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:forgetLastTabAlignment")]
pub struct ForgetLastTabAlignment {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Add Document Grid Line Pitch To Lines in Table Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:adjustLineHeightInTable")]
pub struct AdjustLineHeightInTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 95 Full-Width Character Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autoSpaceLikeWord95")]
pub struct AutoSpaceLikeWord95 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Increase Line Height for Raised/Lowered Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noSpaceRaiseLower")]
pub struct NoSpaceRaiseLower {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Fixed Paragraph Spacing for HTML Auto Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotUseHTMLParagraphAutoSpacing")]
pub struct DoNotUseHtmlParagraphAutoSpacing {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Space Before Table When Deciding If Table Should Wrap Floating Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:layoutRawTableWidth")]
pub struct LayoutRawTableWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Table Rows to Wrap Inline Objects Independently.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:layoutTableRowsApart")]
pub struct LayoutTableRowsApart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 97 East Asian Line Breaking.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useWord97LineBreakRules")]
pub struct UseWord97LineBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Allow Floating Tables To Break Across Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotBreakWrappedTables")]
pub struct DoNotBreakWrappedTables {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Snap to Document Grid in Table Cells with Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotSnapToGridInCell")]
pub struct DoNotSnapToGridInCell {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Select Field When First or Last Character Is Selected.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:selectFldWithFirstOrLastChar")]
pub struct SelectFieldWithFirstOrLastChar {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Legacy Ethiopic and Amharic Line Breaking Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:applyBreakingRules")]
pub struct ApplyBreakingRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Allow Hanging Punctuation With Character Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotWrapTextWithPunct")]
pub struct DoNotWrapTextWithPunctuation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Compress Compressible Characters When Using Document Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotUseEastAsianBreakRules")]
pub struct DoNotUseEastAsianBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Emulate Word 2002 Table Style Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useWord2002TableStyleRules")]
pub struct UseWord2002TableStyleRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Tables to AutoFit Into Page Margins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:growAutofit")]
pub struct GrowAutofit {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Bypass East Asian/Complex Script Layout Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useFELayout")]
pub struct UseFarEastLayout {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useNormalStyleForList")]
pub struct UseNormalStyleForList {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Hanging Indent When Creating Tab Stop After Numbering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotUseIndentAsNumberingTabStop")]
pub struct DoNotUseIndentAsNumberingTabStop {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Alternate Set of East Asian Line Breaking Rules.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useAltKinsokuLineBreakRules")]
pub struct UseAltKinsokuLineBreakRules {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Contextual Spacing of Paragraphs in Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:allowSpaceOfSameStyleInTable")]
pub struct AllowSpaceOfSameStyleInTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Ignore Floating Objects When Calculating Paragraph Indentation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotSuppressIndentation")]
pub struct DoNotSuppressIndentation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not AutoFit Tables To Fit Next To Wrapped Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotAutofitConstrainedTables")]
pub struct DoNotAutofitConstrainedTables {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Table Columns To Exceed Preferred Widths of Constituent Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autofitToFirstFixedWidthCell")]
pub struct AutofitToFirstFixedWidthCell {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Underline Following Character Following Numbering.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:underlineTabInNumList")]
pub struct UnderlineTabInNumberingList {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Always Use Fixed Width for Hangul Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:displayHangulFixedWidth")]
pub struct DisplayHangulFixedWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Always Move Paragraph Mark to Page after a Page Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:splitPgBreakAndParaMark")]
pub struct SplitPageBreakAndParagraphMark {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Vertically Align Cells Containing Floating Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotVertAlignCellWithSp")]
pub struct DoNotVerticallyAlignCellWithShape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Don't Break Table Rows Around Floating Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotBreakConstrainedForcedTable")]
pub struct DoNotBreakConstrainedForcedTable {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Vertical Alignment in Textboxes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotVertAlignInTxbx")]
pub struct DoNotVerticallyAlignInTextBox {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use ANSI Kerning Pairs from Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useAnsiKerningPairs")]
pub struct UseAnsiKerningPairs {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Cached Paragraph Information for Column Balancing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:cachedColBalance")]
pub struct CachedColumnBalance {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the ShowingPlaceholder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:showingPlcHdr")]
pub struct ShowingPlaceholder {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TemporarySdt Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:temporary")]
pub struct TemporarySdt {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Personal Information from Document Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:removePersonalInformation")]
pub struct RemovePersonalInformation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Remove Date and Time from Annotations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:removeDateAndTime")]
pub struct RemoveDateAndTime {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Boundary For Header/Footer or Between Pages.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotDisplayPageBoundaries")]
pub struct DoNotDisplayPageBoundaries {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Display Background Objects When Displaying Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:displayBackgroundShape")]
pub struct DisplayBackgroundShape {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print PostScript Codes With Document Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printPostScriptOverText")]
pub struct PrintPostScriptOverText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Fractional Character Widths.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printFractionalCharacterWidth")]
pub struct PrintFractionalCharacterWidth {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Print Form Field Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printFormsData")]
pub struct PrintFormsData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Embed TrueType Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:embedTrueTypeFonts")]
pub struct EmbedTrueTypeFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Embed Common System Fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:embedSystemFonts")]
pub struct EmbedSystemFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Subset Fonts When Embedding.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:saveSubsetFonts")]
pub struct SaveSubsetFonts {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Save Form Field Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:saveFormsData")]
pub struct SaveFormsData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Mirror Page Margins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:mirrorMargins")]
pub struct MirrorMargins {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Align Paragraph and Table Borders with Page Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:alignBordersAndEdges")]
pub struct AlignBorderAndEdges {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Page Border Excludes Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bordersDoNotSurroundHeader")]
pub struct BordersDoNotSurroundHeader {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Page Border Excludes Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bordersDoNotSurroundFooter")]
pub struct BordersDoNotSurroundFooter {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Position Gutter At Top of Page.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:gutterAtTop")]
pub struct GutterAtTop {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Indication of Spelling Errors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:hideSpellingErrors")]
pub struct HideSpellingErrors {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Display Visual Indication of Grammatical Errors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:hideGrammaticalErrors")]
pub struct HideGrammaticalErrors {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Structured Document Tag Placeholder Text Should be Resaved.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:formsDesign")]
pub struct FormsDesign {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Update Styles From Document Template.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:linkStyles")]
pub struct LinkStyles {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Track Revisions to Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:trackRevisions")]
pub struct TrackRevisions {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Move Syntax When Tracking Revisions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotTrackMoves")]
pub struct DoNotTrackMoves {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Track Formatting Revisions When Tracking Revisions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotTrackFormatting")]
pub struct DoNotTrackFormatting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Automatic Formatting to Override Formatting Protection Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autoFormatOverride")]
pub struct AutoFormatOverride {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Prevent Modification of Themes Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:styleLockTheme")]
pub struct StyleLockThemesPart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Prevent Replacement of Styles Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:styleLockQFSet")]
pub struct StyleLockStylesPart {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Hyphenate Document Contents When Displayed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:autoHyphenation")]
pub struct AutoHyphenation {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Hyphenate Words in ALL CAPITAL LETTERS.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotHyphenateCaps")]
pub struct DoNotHyphenateCaps {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Show E-Mail Message Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:showEnvelope")]
pub struct ShowEnvelope {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Different Even/Odd Page Headers and Footers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:evenAndOddHeaders")]
pub struct EvenAndOddHeaders {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Reverse Book Fold Printing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bookFoldRevPrinting")]
pub struct BookFoldReversePrinting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Book Fold Printing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:bookFoldPrinting")]
pub struct BookFoldPrinting {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Use Margins for Drawing Grid Origin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotUseMarginsForDrawingGridOrigin")]
pub struct DoNotUseMarginsForDrawingGridOrigin {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Show Visual Indicator For Form Fields.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotShadeFormData")]
pub struct DoNotShadeFormData {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Never Kern Punctuation Characters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:noPunctuationKerning")]
pub struct NoPunctuationKerning {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Print Two Pages Per Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:printTwoOnOne")]
pub struct PrintTwoOnOne {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Strict Kinsoku Rules for Japanese Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:strictFirstAndLastChars")]
pub struct StrictFirstAndLastChars {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Generate Thumbnail For Document On Save.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:savePreviewPicture")]
pub struct SavePreviewPicture {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Validate Custom XML Markup Against Schemas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotValidateAgainstSchema")]
pub struct DoNotValidateAgainstSchema {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Allow Saving Document As XML File When Custom XML Markup Is Invalid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:saveInvalidXml")]
pub struct SaveInvalidXml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Ignore Mixed Content When Validating Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:ignoreMixedContent")]
pub struct IgnoreMixedContent {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Use Custom XML Element Names as Default Placeholder Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:alwaysShowPlaceholderText")]
pub struct AlwaysShowPlaceholderText {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Show Visual Indicator For Invalid Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotDemarcateInvalidXml")]
pub struct DoNotDemarcateInvalidXml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Only Save Custom XML Markup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:saveXmlDataOnly")]
pub struct SaveXmlDataOnly {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Save Document as XML File through Custom XSL Transform.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:useXSLTWhenSaving")]
pub struct UseXsltWhenSaving {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Show Visual Indicators for Custom XML Markup Start/End Locations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:showXMLTags")]
pub struct ShowXmlTags {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Mark Custom XML Elements With No Namespace As Invalid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:alwaysMergeEmptyNamespace")]
pub struct AlwaysMergeEmptyNamespace {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Automatically Recalculate Fields on Open.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:updateFields")]
pub struct UpdateFieldsOnOpen {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Disable Features Incompatible With Earlier Word Processing Formats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:uiCompat97To2003")]
pub struct UiCompatibleWith97To2003 {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Include Content in Text Boxes, Footnotes, and Endnotes in Document Statistics.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotIncludeSubdocsInStats")]
pub struct DoNotIncludeSubdocsInStats {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Do Not Automatically Compress Images.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotAutoCompressPictures")]
pub struct DoNotAutoCompressPictures {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the OptimizeForBrowser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:optimizeForBrowser")]
pub struct OptimizeForBrowser {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the RelyOnVML Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:relyOnVML")]
pub struct RelyOnVml {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the AllowPNG Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:allowPNG")]
pub struct AllowPng {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotRelyOnCSS Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotRelyOnCSS")]
pub struct DoNotRelyOnCss {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotSaveAsSingleFile Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotSaveAsSingleFile")]
pub struct DoNotSaveAsSingleFile {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotOrganizeInFolder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotOrganizeInFolder")]
pub struct DoNotOrganizeInFolder {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the DoNotUseLongFileNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:doNotUseLongFileNames")]
pub struct DoNotUseLongFileNames {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NotTrueType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOff/w:notTrueType")]
pub struct NotTrueType {
  /// On/Off Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FrameProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FramePr/w:framePr")]
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
  pub width: Option<crate::simple_type::StringValue>,
  /// Frame Height
  #[sdk(attr(qname = "w:h"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// Vertical Frame Padding
  #[sdk(attr(qname = "w:vSpace"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub vertical_space: Option<crate::simple_type::StringValue>,
  /// Horizontal Frame Padding
  #[sdk(attr(qname = "w:hSpace"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub horizontal_space: Option<crate::simple_type::StringValue>,
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
  pub x: Option<crate::simple_type::StringValue>,
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
  pub y: Option<crate::simple_type::StringValue>,
  /// Relative Vertical Position
  #[sdk(attr(qname = "w:yAlign"))]
  pub y_align: Option<VerticalAlignmentValues>,
  /// Frame Height Type
  #[sdk(attr(qname = "w:hRule"))]
  pub height_type: Option<HeightRuleValues>,
  /// Lock Frame Anchor to Paragraph
  #[sdk(attr(qname = "w:anchorLock"))]
  pub anchor_lock: Option<crate::simple_type::OnOffValue>,
}
/// Defines the NumberingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NumPr/w:numPr")]
pub struct NumberingProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Numbering Level Reference
  #[sdk(child(qname = "w:CT_NonNegativeDecimalNumber255/w:ilvl"))]
  pub numbering_level_reference: Option<NumberingLevelReference>,
  /// Numbering Definition Instance Reference
  #[sdk(child(qname = "w:CT_NonNegativeDecimalNumber/w:numId"))]
  pub numbering_id: Option<NumberingId>,
  /// Previous Paragraph Numbering Properties
  #[sdk(child(qname = "w:CT_TrackChangeNumbering/w:numberingChange"))]
  pub numbering_change: Option<NumberingChange>,
  /// Inserted Numbering Properties
  #[sdk(child(qname = "w:CT_TrackChange/w:ins"))]
  pub inserted: Option<Inserted>,
}
/// Defines the ParagraphBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PBdr/w:pBdr")]
pub struct ParagraphBorders {
  /// Paragraph Border Above Identical Paragraphs
  #[sdk(child(qname = "w:CT_Border/w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Paragraph Border
  #[sdk(child(qname = "w:CT_Border/w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Paragraph Border Between Identical Paragraphs
  #[sdk(child(qname = "w:CT_Border/w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Paragraph Border
  #[sdk(child(qname = "w:CT_Border/w:right"))]
  pub right_border: Option<RightBorder>,
  /// Paragraph Border Between Identical Paragraphs
  #[sdk(child(qname = "w:CT_Border/w:between"))]
  pub between_border: Option<BetweenBorder>,
  /// Paragraph Border Between Facing Pages
  #[sdk(child(qname = "w:CT_Border/w:bar"))]
  pub bar_border: Option<BarBorder>,
}
/// Defines the Shading Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Shd/w:shd")]
pub struct Shading {
  /// Shading Pattern
  #[sdk(attr(qname = "w:val"))]
  pub val: ShadingPatternValues,
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
#[sdk(qname = "w:CT_Tabs/w:tabs")]
pub struct Tabs {
  /// Custom Tab Stop.
  #[sdk(child(qname = "w:CT_TabStop/w:tab"))]
  pub w_tab: Vec<TabStop>,
}
/// Defines the SpacingBetweenLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Spacing/w:spacing")]
pub struct SpacingBetweenLines {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Spacing Above Paragraph
  #[sdk(attr(qname = "w:before"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub before: Option<crate::simple_type::StringValue>,
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
  pub after: Option<crate::simple_type::StringValue>,
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
  pub line: Option<crate::simple_type::StringValue>,
  /// Type of Spacing Between Lines
  #[sdk(attr(qname = "w:lineRule"))]
  pub line_rule: Option<LineSpacingRuleValues>,
}
/// Defines the Indentation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Ind/w:ind")]
pub struct Indentation {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Left Indentation
  #[sdk(attr(qname = "w:left"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub left: Option<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(office2010, qname = "w:start"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub start: Option<crate::simple_type::StringValue>,
  /// Left Indentation in Character Units
  #[sdk(attr(qname = "w:leftChars"))]
  pub left_chars: Option<crate::simple_type::Int32Value>,
  /// startChars
  #[sdk(attr(office2010, qname = "w:startChars"))]
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
  pub right: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(office2010, qname = "w:end"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 3u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub end: Option<crate::simple_type::StringValue>,
  /// Right Indentation in Character Units
  #[sdk(attr(qname = "w:rightChars"))]
  pub right_chars: Option<crate::simple_type::Int32Value>,
  /// endChars
  #[sdk(attr(office2010, qname = "w:endChars"))]
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
  pub hanging: Option<crate::simple_type::StringValue>,
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
  pub first_line: Option<crate::simple_type::StringValue>,
  /// Additional First Line Indentation in Character Units
  #[sdk(attr(qname = "w:firstLineChars"))]
  pub first_line_chars: Option<crate::simple_type::Int32Value>,
}
/// Defines the Justification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Jc/w:jc")]
pub struct Justification {
  /// Alignment Type
  #[sdk(attr(qname = "w:val"))]
  pub val: JustificationValues,
}
/// Defines the TextDirection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TextDirection/w:textDirection")]
pub struct TextDirection {
  /// Direction of Text Flow
  #[sdk(attr(qname = "w:val"))]
  pub val: TextDirectionValues,
}
/// Defines the TextAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TextAlignment/w:textAlignment")]
pub struct TextAlignment {
  /// Vertical Character Alignment Position
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalTextAlignmentValues,
}
/// Defines the TextBoxTightWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap")]
pub struct TextBoxTightWrap {
  /// Lines to Tight Wrap to Paragraph Extents
  #[sdk(attr(qname = "w:val"))]
  pub val: TextBoxTightWrapValues,
}
/// Defines the OutlineLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:outlineLvl")]
pub struct OutlineLevel {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridSpan Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:gridSpan")]
pub struct GridSpan {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridBefore Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:gridBefore")]
pub struct GridBefore {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the GridAfter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:gridAfter")]
pub struct GridAfter {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Drop-Down List Selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:result")]
pub struct DropDownListSelection {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Record Currently Displayed In Merged Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:activeRecord")]
pub struct ActiveRecord {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Mail Merge Error Reporting Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:checkErrors")]
pub struct CheckErrors {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Restart Numbering Level Symbol.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:lvlRestart")]
pub struct LevelRestart {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Picture Numbering Symbol Definition Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:lvlPicBulletId")]
pub struct LevelPictureBulletId {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Numbering Level Starting Value Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:startOverride")]
pub struct StartOverrideNumberingValue {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Last Reviewed Abstract Numbering Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:numIdMacAtCleanup")]
pub struct NumberingIdMacAtCleanup {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SdtId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:id")]
pub struct SdtId {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the PixelsPerInch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DecimalNumber/w:pixelsPerInch")]
pub struct PixelsPerInch {
  /// Decimal Number Value
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the ParagraphPropertiesChange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrChange/w:pPrChange")]
pub struct ParagraphPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_PPrExtended/w:pPr"))]
  pub paragraph_properties_extended: std::boxed::Box<ParagraphPropertiesExtended>,
}
/// Header Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HdrFtrRef/w:headerReference")]
pub struct HeaderReference {
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: HeaderFooterValues,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Footer Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HdrFtrRef/w:footerReference")]
pub struct FooterReference {
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: HeaderFooterValues,
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Br/w:br")]
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
#[sdk(qname = "w:CT_Text/w:t")]
pub struct Text {
  pub xml_other_attrs: Vec<(String, String)>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Deleted Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Text/w:delText")]
pub struct DeletedText {
  pub xml_other_attrs: Vec<(String, String)>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Field Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Text/w:instrText")]
pub struct FieldCode {
  pub xml_other_attrs: Vec<(String, String)>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Deleted Field Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Text/w:delInstrText")]
pub struct DeletedFieldCode {
  pub xml_other_attrs: Vec<(String, String)>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Symbol Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Sym/w:sym")]
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
#[sdk(qname = "w:CT_Object/w:object")]
pub struct EmbeddedObject {
  pub xml_other_attrs: Vec<(String, String)>,
  /// dxaOrig
  #[sdk(attr(qname = "w:dxaOrig"))]
  pub dxa_original: Option<crate::simple_type::StringValue>,
  /// dyaOrig
  #[sdk(attr(qname = "w:dyaOrig"))]
  pub dya_original: Option<crate::simple_type::StringValue>,
  /// anchorId
  #[sdk(attr(office2010, qname = "w14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(choice(
    qname = "v:CT_Group/v:group",
    qname = "v:CT_Image/v:image",
    qname = "v:CT_Line/v:line",
    qname = "v:CT_Oval/v:oval",
    qname = "v:CT_PolyLine/v:polyline",
    qname = "v:CT_Rect/v:rect",
    qname = "v:CT_RoundRect/v:roundrect",
    qname = "v:CT_Shape/v:shape",
    qname = "v:CT_Shapetype/v:shapetype",
    qname = "v:CT_Arc/v:arc",
    qname = "v:CT_Curve/v:curve",
    qname = "o:CT_OLEObject/o:OLEObject"
  ))]
  pub embedded_object_choice1: Vec<EmbeddedObjectChoice>,
  /// DrawingML Object.
  #[sdk(child(qname = "w:CT_Drawing/w:drawing"))]
  pub w_drawing: Option<std::boxed::Box<Drawing>>,
  #[sdk(choice(
    qname = "w:CT_Control/w:control",
    qname = "w:CT_ObjectEmbed/w:objectEmbed",
    qname = "w:CT_ObjectLink/w:objectLink"
  ))]
  pub embedded_object_choice2: Option<EmbeddedObjectChoice2>,
}
/// VML Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Picture/w:pict")]
pub struct Picture {
  pub xml_other_attrs: Vec<(String, String)>,
  /// anchorId
  #[sdk(attr(office2010, qname = "w14:anchorId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w14_anchor_id: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(choice(
    qname = "v:CT_Group/v:group",
    qname = "v:CT_Image/v:image",
    qname = "v:CT_Line/v:line",
    qname = "v:CT_Oval/v:oval",
    qname = "v:CT_PolyLine/v:polyline",
    qname = "v:CT_Rect/v:rect",
    qname = "v:CT_RoundRect/v:roundrect",
    qname = "v:CT_Shape/v:shape",
    qname = "v:CT_Shapetype/v:shapetype",
    qname = "v:CT_Arc/v:arc",
    qname = "v:CT_Curve/v:curve",
    qname = "o:CT_OLEObject/o:OLEObject"
  ))]
  pub picture_choice: Vec<PictureChoice>,
  /// Defines the MovieReference Class.
  #[sdk(child(qname = "w:CT_Rel/w:movie"))]
  pub w_movie: Option<MovieReference>,
  /// Defines the Control Class.
  #[sdk(child(qname = "w:CT_Control/w:control"))]
  pub w_control: Option<Control>,
}
/// Complex Field Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FldChar/w:fldChar")]
pub struct FieldChar {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Field Character Type
  #[sdk(attr(qname = "w:fldCharType"))]
  pub field_char_type: FieldCharValues,
  /// Field Should Not Be Recalculated
  #[sdk(attr(qname = "w:fldLock"))]
  pub field_lock: Option<crate::simple_type::OnOffValue>,
  /// Field Result Invalidated
  #[sdk(attr(qname = "w:dirty"))]
  pub dirty: Option<crate::simple_type::OnOffValue>,
  #[sdk(choice(
    qname = "w:CT_Base64BinaryText/w:fldData",
    qname = "w:CT_FFData/w:ffData",
    qname = "w:CT_TrackChangeNumbering/w:numberingChange"
  ))]
  pub field_char_choice: Option<FieldCharChoice>,
}
/// Phonetic Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Ruby/w:ruby")]
pub struct Ruby {
  /// Phonetic Guide Properties
  #[sdk(child(qname = "w:CT_RubyPr/w:rubyPr"))]
  pub ruby_properties: std::boxed::Box<RubyProperties>,
  /// Phonetic Guide Text
  #[sdk(child(qname = "w:CT_RubyContent/w:rt"))]
  pub ruby_content: std::boxed::Box<RubyContent>,
  /// Phonetic Guide Base Text
  #[sdk(child(qname = "w:CT_RubyContent/w:rubyBase"))]
  pub ruby_base: std::boxed::Box<RubyBase>,
}
/// Footnote Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdnRef/w:footnoteReference")]
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
#[sdk(qname = "w:CT_FtnEdnRef/w:endnoteReference")]
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
#[sdk(qname = "w:CT_Drawing/w:drawing")]
pub struct Drawing {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(qname = "wp:CT_Anchor/wp:anchor", qname = "wp:CT_Inline/wp:inline"))]
  pub drawing_choice: Option<DrawingChoice>,
}
/// Absolute Position Tab Character.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PTab/w:ptab")]
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
#[sdk(qname = "w:CT_String253/w:rStyle")]
pub struct RunStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TableStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:tblStyle")]
pub struct TableStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Paragraph Style's Associated Numbering Level.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:pStyle")]
pub struct ParagraphStyleIdInLevel {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Abstract Numbering Definition Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:name")]
pub struct AbstractNumDefinitionName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Numbering Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:styleLink")]
pub struct StyleLink {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Numbering Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:numStyleLink")]
pub struct NumberingStyleLink {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Alternate Style Names.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:aliases")]
pub struct Aliases {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Parent Style ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:basedOn")]
pub struct BasedOn {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Style For Next Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:next")]
pub struct NextParagraphStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Linked Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:link")]
pub struct LinkedStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Paragraph Style Applied to Automatically Generated Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:clickAndTypeStyle")]
pub struct ClickAndTypeStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Default Table Style for Newly Inserted Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String253/w:defaultTableStyle")]
pub struct DefaultTableStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 253u32))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the RunFonts Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Fonts/w:rFonts")]
pub struct RunFonts {
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
#[sdk(qname = "w:CT_Color/w:color")]
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
  pub val: crate::simple_type::StringValue,
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
#[sdk(qname = "w:CT_ShortTwipsMeasure/w:spacing")]
pub struct Spacing {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CharacterScale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TextScale/w:w")]
pub struct CharacterScale {
  /// Text Expansion/Compression Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 1..= 600))]
  pub val: Option<crate::simple_type::IntegerValue>,
}
/// Defines the Kern Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HpsKern/w:kern")]
pub struct Kern {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3277))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the Position Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SignedHpsMeasure/w:position")]
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
#[sdk(qname = "w:CT_HpsMeasure/w:sz")]
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
#[sdk(qname = "w:CT_HpsMeasure/w:szCs")]
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
#[sdk(qname = "w:CT_HpsMeasure/w:size")]
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
#[sdk(qname = "w:CT_HpsMeasure/w:hps")]
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
#[sdk(qname = "w:CT_HpsMeasure/w:hpsBaseText")]
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
#[sdk(qname = "w:CT_Highlight/w:highlight")]
pub struct Highlight {
  /// Highlighting Color
  #[sdk(attr(qname = "w:val"))]
  pub val: HighlightColorValues,
}
/// Defines the Underline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Underline/w:u")]
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
#[sdk(qname = "w:CT_TextEffect/w:effect")]
pub struct TextEffect {
  /// Animated Text Effect Type
  #[sdk(attr(qname = "w:val"))]
  pub val: TextEffectValues,
}
/// Defines the Border Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Border/w:bdr")]
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
#[sdk(qname = "w:CT_Border/w:top")]
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
#[sdk(qname = "w:CT_Border/w:left")]
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
#[sdk(qname = "w:CT_Border/w:bottom")]
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
#[sdk(qname = "w:CT_Border/w:right")]
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
#[sdk(qname = "w:CT_Border/w:between")]
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
#[sdk(qname = "w:CT_Border/w:bar")]
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
#[sdk(office2010, qname = "w:CT_Border/w:start")]
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
#[sdk(office2010, qname = "w:CT_Border/w:end")]
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
#[sdk(qname = "w:CT_Border/w:insideH")]
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
#[sdk(qname = "w:CT_Border/w:insideV")]
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
#[sdk(qname = "w:CT_Border/w:tl2br")]
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
#[sdk(qname = "w:CT_Border/w:tr2bl")]
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
#[sdk(qname = "w:CT_FitText/w:fitText")]
pub struct FitText {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: crate::simple_type::UInt32Value,
  /// Fit Text Run ID
  #[sdk(attr(qname = "w:id"))]
  pub id: Option<crate::simple_type::Int32Value>,
}
/// Defines the VerticalTextAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_VerticalAlignRun/w:vertAlign")]
pub struct VerticalTextAlignment {
  /// Subscript/Superscript Value
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalPositionValues,
}
/// Defines the Emphasis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Em/w:em")]
pub struct Emphasis {
  /// Emphasis Mark Type
  #[sdk(attr(qname = "w:val"))]
  pub val: EmphasisMarkValues,
}
/// Defines the Languages Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Language/w:lang")]
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
#[sdk(qname = "w:CT_Language/w:themeFontLang")]
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
#[sdk(qname = "w:CT_EastAsianLayout/w:eastAsianLayout")]
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
#[sdk(qname = "w:CT_RPrChange/w:rPrChange")]
pub struct RunPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_RPrOriginal/w:rPr"))]
  pub previous_run_properties: std::boxed::Box<PreviousRunProperties>,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPr/w:rPr")]
pub struct RunProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the RunStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:rStyle"))]
  pub run_style: Option<RunStyle>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "w:CT_Highlight/w:highlight"))]
  pub highlight: Option<Highlight>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtl"))]
  pub right_to_left_text: Option<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:cs"))]
  pub complex_script: Option<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(office2010, qname = "w14:CT_Glow/w14:glow"))]
  pub glow:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(office2010, qname = "w14:CT_Shadow/w14:shadow"))]
  pub shadow14:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(office2010, qname = "w14:CT_Reflection/w14:reflection"))]
  pub reflection: Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_TextOutlineEffect/w14:textOutline"))]
  pub text_outline_effect: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    >,
  >,
  /// Defines the FillTextEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_FillTextEffect/w14:textFill"))]
  pub fill_text_effect: Option<
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect>,
  >,
  /// Defines the Scene3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Scene3D/w14:scene3d"))]
  pub scene3_d:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Props3D/w14:props3d"))]
  pub properties3_d: Option<
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D>,
  >,
  /// Defines the Ligatures Class.
  #[sdk(child(office2010, qname = "w14:CT_Ligatures/w14:ligatures"))]
  pub ligatures: Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2010, qname = "w14:CT_NumForm/w14:numForm"))]
  pub numbering_format:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(office2010, qname = "w14:CT_NumSpacing/w14:numSpacing"))]
  pub number_spacing:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(office2010, qname = "w14:CT_StylisticSets/w14:stylisticSets"))]
  pub stylistic_sets:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:cntxtAlts"))]
  pub contextual_alternatives:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives>,
  /// Defines the RunPropertiesChange Class.
  #[sdk(child(qname = "w:CT_RPrChange/w:rPrChange"))]
  pub run_properties_change: Option<std::boxed::Box<RunPropertiesChange>>,
}
/// Defines the InsertedMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MathCtrlIns/w:ins")]
pub struct InsertedMathControl {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(qname = "w:CT_RPr/w:rPr", qname = "w:CT_MathCtrlDel/w:del"))]
  pub inserted_math_control_choice: Option<InsertedMathControlChoice>,
}
/// Defines the DeletedMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MathCtrlDel/w:del")]
pub struct DeletedMathControl {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
}
/// Defines the MoveFromMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MathCtrlMove/w:moveFrom")]
pub struct MoveFromMathControl {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
    qname = "w:CT_RPr/w:rPr",
    qname = "w:CT_MathCtrlIns/w:ins",
    qname = "w:CT_MathCtrlDel/w:del"
  ))]
  pub move_from_math_control_choice: Option<MoveFromMathControlChoice>,
}
/// Defines the MoveToMathControl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MathCtrlMove/w:moveTo")]
pub struct MoveToMathControl {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
    qname = "w:CT_RPr/w:rPr",
    qname = "w:CT_MathCtrlIns/w:ins",
    qname = "w:CT_MathCtrlDel/w:del"
  ))]
  pub move_to_math_control_choice: Option<MoveToMathControlChoice>,
}
/// Defines the CustomXmlRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlRuby/w:customXml")]
pub struct CustomXmlRuby {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:CT_CustomXmlPr/w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub custom_xml_ruby_choice: Vec<CustomXmlRubyChoice>,
}
/// Defines the SimpleFieldRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SimpleFieldRuby/w:fldSimple")]
pub struct SimpleFieldRuby {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(text_child(qname = "w:CT_Base64BinaryText/w:fldData"))]
  pub field_data: Option<crate::simple_type::Base64BinaryValue>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub simple_field_ruby_choice: Vec<SimpleFieldRubyChoice>,
}
/// Defines the HyperlinkRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HyperlinkRuby/w:hyperlink")]
pub struct HyperlinkRuby {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub hyperlink_ruby_choice: Vec<HyperlinkRubyChoice>,
}
/// Phonetic Guide Text Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_R/w:r")]
pub struct Run {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
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
    text,
    any
  ))]
  pub run_choice: Vec<RunChoice>,
}
/// Defines the SdtRunRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtRunRuby/w:sdt")]
pub struct SdtRunRuby {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:CT_SdtPr/w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:CT_SdtEndPr/w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Defines the SdtContentRunRuby Class.
  #[sdk(child(qname = "w:CT_SdtContentRunRuby/w:sdtContent"))]
  pub sdt_content_run_ruby: Option<SdtContentRunRuby>,
  #[sdk(choice(
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
    text,
    any
  ))]
  pub sdt_run_ruby_choice: Vec<SdtRunRubyChoice>,
}
/// Defines the ProofError Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ProofErr/w:proofErr")]
pub struct ProofError {
  /// Proofing Error Anchor Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: ProofingErrorValues,
}
/// Defines the PermStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PermStart/w:permStart")]
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
#[sdk(qname = "w:CT_Perm/w:permEnd")]
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
#[sdk(qname = "w:CT_RunTrackChange/w:ins")]
pub struct InsertedRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    text,
    any
  ))]
  pub inserted_run_choice: Vec<InsertedRunChoice>,
}
/// Deleted Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RunTrackChange/w:del")]
pub struct DeletedRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    text,
    any
  ))]
  pub deleted_run_choice: Vec<DeletedRunChoice>,
}
/// Move Source Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RunTrackChange/w:moveFrom")]
pub struct MoveFromRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    text,
    any
  ))]
  pub move_from_run_choice: Vec<MoveFromRunChoice>,
}
/// Move Destination Run Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RunTrackChange/w:moveTo")]
pub struct MoveToRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    text,
    any
  ))]
  pub move_to_run_choice: Vec<MoveToRunChoice>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_ContentPart/w:contentPart")]
pub struct ContentPart {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the SdtRun Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtRun/w:sdt")]
pub struct SdtRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:CT_SdtPr/w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:CT_SdtEndPr/w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Inline-Level Structured Document Tag Content
  #[sdk(child(qname = "w:CT_SdtContentRun/w:sdtContent"))]
  pub sdt_content_run: Option<SdtContentRun>,
  #[sdk(choice(
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
    text,
    any
  ))]
  pub sdt_run_choice: Vec<SdtRunChoice>,
}
/// Defines the CustomXmlBlock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlBlock/w:customXml")]
pub struct CustomXmlBlock {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:CT_CustomXmlPr/w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub custom_xml_block_choice: Vec<CustomXmlBlockChoice>,
}
/// Defines the SdtBlock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtBlock/w:sdt")]
pub struct SdtBlock {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:CT_SdtPr/w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:CT_SdtEndPr/w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Block-Level Structured Document Tag Content
  #[sdk(child(qname = "w:CT_SdtContentBlock/w:sdtContent"))]
  pub sdt_content_block: Option<SdtContentBlock>,
  #[sdk(choice(
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
    text,
    any
  ))]
  pub sdt_block_choice: Vec<SdtBlockChoice>,
}
/// Defines the Paragraph Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_P/w:p")]
pub struct Paragraph {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(office2010, qname = "w14:paraId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub paragraph_id: Option<crate::simple_type::HexBinaryValue>,
  /// textId
  #[sdk(attr(office2010, qname = "w14:textId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub text_id: Option<crate::simple_type::HexBinaryValue>,
  /// noSpellErr
  #[sdk(attr(office2010, qname = "w14:noSpellErr"))]
  pub no_spell_error: Option<crate::simple_type::OnOffValue>,
  /// Paragraph Properties
  #[sdk(child(qname = "w:CT_PPr/w:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc",
    text,
    any
  ))]
  pub paragraph_choice: Vec<ParagraphChoice>,
}
/// Defines the Table Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Tbl/w:tbl")]
pub struct Table {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
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
    qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"
  ))]
  pub table_choice1: Vec<TableChoice>,
  /// Table Properties.
  #[sdk(child(qname = "w:CT_TblPr/w:tblPr"))]
  pub w_tbl_pr: Option<std::boxed::Box<TableProperties>>,
  /// Table Grid.
  #[sdk(child(qname = "w:CT_TblGrid/w:tblGrid"))]
  pub w_tbl_grid: Option<std::boxed::Box<TableGrid>>,
  #[sdk(choice(
    qname = "w:CT_Row/w:tr",
    qname = "w:CT_CustomXmlRow/w:customXml",
    qname = "w:CT_SdtRow/w:sdt",
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
    qname = "w:CT_RunTrackChange/w14:conflictDel"
  ))]
  pub table_choice2: Vec<TableChoice2>,
}
/// Table Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Row/w:tr")]
pub struct TableRow {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(office2010, qname = "w14:paraId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub paragraph_id: Option<crate::simple_type::HexBinaryValue>,
  /// textId
  #[sdk(attr(office2010, qname = "w14:textId"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub text_id: Option<crate::simple_type::HexBinaryValue>,
  /// Table-Level Property Exceptions
  #[sdk(child(qname = "w:CT_TblPrEx/w:tblPrEx"))]
  pub table_property_exceptions: Option<std::boxed::Box<TablePropertyExceptions>>,
  /// Table Row Properties
  #[sdk(child(qname = "w:CT_TrPr/w:trPr"))]
  pub table_row_properties: Option<std::boxed::Box<TableRowProperties>>,
  #[sdk(choice(
    qname = "w:CT_Tc/w:tc",
    qname = "w:CT_CustomXmlCell/w:customXml",
    qname = "w:CT_SdtCell/w:sdt",
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
    text,
    any
  ))]
  pub table_row_choice: Vec<TableRowChoice>,
}
/// Row-Level Custom XML Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlRow/w:customXml")]
pub struct CustomXmlRow {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:CT_CustomXmlPr/w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(choice(
    qname = "w:CT_Row/w:tr",
    qname = "w:CT_CustomXmlRow/w:customXml",
    qname = "w:CT_SdtRow/w:sdt",
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
    text,
    any
  ))]
  pub custom_xml_row_choice: Vec<CustomXmlRowChoice>,
}
/// Row-Level Structured Document Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtRow/w:sdt")]
pub struct SdtRow {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:CT_SdtPr/w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:CT_SdtEndPr/w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Row-Level Structured Document Tag Content
  #[sdk(child(qname = "w:CT_SdtContentRow/w:sdtContent"))]
  pub sdt_content_row: Option<SdtContentRow>,
  #[sdk(choice(
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
    text,
    any
  ))]
  pub sdt_row_choice: Vec<SdtRowChoice>,
}
/// Table Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Tc/w:tc")]
pub struct TableCell {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Table Cell Properties
  #[sdk(child(qname = "w:CT_TcPr/w:tcPr"))]
  pub table_cell_properties: Option<std::boxed::Box<TableCellProperties>>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub table_cell_choice: Vec<TableCellChoice>,
}
/// Cell-Level Custom XML Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlCell/w:customXml")]
pub struct CustomXmlCell {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:CT_CustomXmlPr/w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(choice(
    qname = "w:CT_Tc/w:tc",
    qname = "w:CT_CustomXmlCell/w:customXml",
    qname = "w:CT_SdtCell/w:sdt",
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
    text,
    any
  ))]
  pub custom_xml_cell_choice: Vec<CustomXmlCellChoice>,
}
/// Cell-Level Structured Document Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtCell/w:sdt")]
pub struct SdtCell {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Structured Document Tag Properties.
  #[sdk(child(qname = "w:CT_SdtPr/w:sdtPr"))]
  pub sdt_properties: Option<SdtProperties>,
  /// Structured Document Tag End Character Properties.
  #[sdk(child(qname = "w:CT_SdtEndPr/w:sdtEndPr"))]
  pub sdt_end_char_properties: Option<SdtEndCharProperties>,
  /// Cell-Level Structured Document Tag Content
  #[sdk(child(qname = "w:CT_SdtContentCell/w:sdtContent"))]
  pub sdt_content_cell: Option<SdtContentCell>,
  #[sdk(choice(
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
    text,
    any
  ))]
  pub sdt_cell_choice: Vec<SdtCellChoice>,
}
/// Defines the CustomXmlRun Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlRun/w:customXml")]
pub struct CustomXmlRun {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Element Properties.
  #[sdk(child(qname = "w:CT_CustomXmlPr/w:customXmlPr"))]
  pub custom_xml_properties: Option<std::boxed::Box<CustomXmlProperties>>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc",
    text,
    any
  ))]
  pub custom_xml_run_choice: Vec<CustomXmlRunChoice>,
}
/// Defines the SimpleField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SimpleField/w:fldSimple")]
pub struct SimpleField {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(text_child(qname = "w:CT_Base64BinaryText/w:fldData"))]
  pub field_data: Option<crate::simple_type::Base64BinaryValue>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc",
    text,
    any
  ))]
  pub simple_field_choice: Vec<SimpleFieldChoice>,
}
/// Defines the Hyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Hyperlink/w:hyperlink")]
pub struct Hyperlink {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc",
    text,
    any
  ))]
  pub hyperlink_choice: Vec<HyperlinkChoice>,
}
/// Defines the BidirectionalOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_BdoContentRun/w:bdo")]
pub struct BidirectionalOverride {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub w_val: Option<DirectionValues>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc"
  ))]
  pub bidirectional_override_choice: Vec<BidirectionalOverrideChoice>,
}
/// Defines the BidirectionalEmbedding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_DirContentRun/w:dir")]
pub struct BidirectionalEmbedding {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub w_val: Option<DirectionValues>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc"
  ))]
  pub bidirectional_embedding_choice: Vec<BidirectionalEmbeddingChoice>,
}
/// Anchor for Subdocument Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:subDoc")]
pub struct SubDocumentReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the PrinterSettingsReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:printerSettings")]
pub struct PrinterSettingsReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// ODSO Data Source File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:src")]
pub struct SourceReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Reference to Inclusion/Exclusion Data for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:recipientData")]
pub struct RecipientDataReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Data Source File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:dataSource")]
pub struct DataSourceReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Header Definition File Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:headerSource")]
pub struct HeaderSource {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Source File for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:sourceFileName")]
pub struct SourceFileReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the MovieReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:movie")]
pub struct MovieReference {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Attached Document Template.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Rel/w:attachedTemplate")]
pub struct AttachedTemplate {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ConditionalFormatStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Cnf/w:cnfStyle")]
pub struct ConditionalFormatStyle {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Conditional Formatting Bit Mask
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "[01]*"))]
  #[sdk(string_length(min = 12u32, max = 12u32))]
  pub val: crate::simple_type::StringValue,
  /// firstRow
  #[sdk(attr(office2010, qname = "w:firstRow"))]
  pub first_row: Option<crate::simple_type::OnOffValue>,
  /// lastRow
  #[sdk(attr(office2010, qname = "w:lastRow"))]
  pub last_row: Option<crate::simple_type::OnOffValue>,
  /// firstColumn
  #[sdk(attr(office2010, qname = "w:firstColumn"))]
  pub first_column: Option<crate::simple_type::OnOffValue>,
  /// lastColumn
  #[sdk(attr(office2010, qname = "w:lastColumn"))]
  pub last_column: Option<crate::simple_type::OnOffValue>,
  /// oddVBand
  #[sdk(attr(office2010, qname = "w:oddVBand"))]
  pub odd_vertical_band: Option<crate::simple_type::OnOffValue>,
  /// evenVBand
  #[sdk(attr(office2010, qname = "w:evenVBand"))]
  pub even_vertical_band: Option<crate::simple_type::OnOffValue>,
  /// oddHBand
  #[sdk(attr(office2010, qname = "w:oddHBand"))]
  pub odd_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// evenHBand
  #[sdk(attr(office2010, qname = "w:evenHBand"))]
  pub even_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// firstRowFirstColumn
  #[sdk(attr(office2010, qname = "w:firstRowFirstColumn"))]
  pub first_row_first_column: Option<crate::simple_type::OnOffValue>,
  /// firstRowLastColumn
  #[sdk(attr(office2010, qname = "w:firstRowLastColumn"))]
  pub first_row_last_column: Option<crate::simple_type::OnOffValue>,
  /// lastRowFirstColumn
  #[sdk(attr(office2010, qname = "w:lastRowFirstColumn"))]
  pub last_row_first_column: Option<crate::simple_type::OnOffValue>,
  /// lastRowLastColumn
  #[sdk(attr(office2010, qname = "w:lastRowLastColumn"))]
  pub last_row_last_column: Option<crate::simple_type::OnOffValue>,
}
/// Defines the TableCellWidth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:tcW")]
pub struct TableCellWidth {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthBeforeTableRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:wBefore")]
pub struct WidthBeforeTableRow {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthAfterTableRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:wAfter")]
pub struct WidthAfterTableRow {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableCellSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:tblCellSpacing")]
pub struct TableCellSpacing {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableWidth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:tblW")]
pub struct TableWidth {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Top Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:top")]
pub struct TopMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the StartMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_TblWidth/w:start")]
pub struct StartMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Bottom Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:bottom")]
pub struct BottomMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the EndMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_TblWidth/w:end")]
pub struct EndMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Left Margin Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:left")]
pub struct LeftMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Right Margin Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidth/w:right")]
pub struct RightMargin {
  /// Table Width Value
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 1u32, union = 0u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "w:ST_DecimalNumber"))]
  #[sdk(pattern(source = 3u32, union = 1u64, regex = "-?[0-9]+(\\.[0-9]+)?%"))]
  #[sdk(number_type(source = 4u32, union = 1u64, type_name = "w:ST_DecimalNumber"))]
  pub width: Option<crate::simple_type::StringValue>,
  /// Table Width Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the HorizontalMerge Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HMerge/w:hMerge")]
pub struct HorizontalMerge {
  /// Horizontal Merge Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<MergedCellValues>,
}
/// Defines the VerticalMerge Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_VMerge/w:vMerge")]
pub struct VerticalMerge {
  /// Vertical Merge Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<MergedCellValues>,
}
/// Defines the TableCellBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcBorders/w:tcBorders")]
pub struct TableCellBorders {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Table Cell Top Border
  #[sdk(child(qname = "w:CT_Border/w:top"))]
  pub top_border: Option<TopBorder>,
  /// Table Cell Left Border
  #[sdk(child(qname = "w:CT_Border/w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Defines the StartBorder Class.
  #[sdk(child(office2010, qname = "w:CT_Border/w:start"))]
  pub start_border: Option<StartBorder>,
  /// Table Cell Bottom Border
  #[sdk(child(qname = "w:CT_Border/w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Table Cell Right Border
  #[sdk(child(qname = "w:CT_Border/w:right"))]
  pub right_border: Option<RightBorder>,
  /// Defines the EndBorder Class.
  #[sdk(child(office2010, qname = "w:CT_Border/w:end"))]
  pub end_border: Option<EndBorder>,
  /// Table Cell Inside Horizontal Edges Border
  #[sdk(child(qname = "w:CT_Border/w:insideH"))]
  pub inside_horizontal_border: Option<InsideHorizontalBorder>,
  /// Table Cell Inside Vertical Edges Border
  #[sdk(child(qname = "w:CT_Border/w:insideV"))]
  pub inside_vertical_border: Option<InsideVerticalBorder>,
  /// Table Cell Top Left to Bottom Right Diagonal Border
  #[sdk(child(qname = "w:CT_Border/w:tl2br"))]
  pub top_left_to_bottom_right_cell_border: Option<TopLeftToBottomRightCellBorder>,
  /// Table Cell Top Right to Bottom Left Diagonal Border
  #[sdk(child(qname = "w:CT_Border/w:tr2bl"))]
  pub top_right_to_bottom_left_cell_border: Option<TopRightToBottomLeftCellBorder>,
}
/// Defines the NoWrap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:noWrap")]
pub struct NoWrap {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableCellFitText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:tcFitText")]
pub struct TableCellFitText {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the HideMark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:hideMark")]
pub struct HideMark {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the CantSplit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:cantSplit")]
pub struct CantSplit {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:tblHeader")]
pub struct TableHeader {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the BiDiVisual Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:bidiVisual")]
pub struct BiDiVisual {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Frame Cannot Be Resized.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:noResizeAllowed")]
pub struct NoResizeAllowed {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Maintain Link to Existing File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:linkedToFile")]
pub struct LinkedToFile {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Do Not Display Frameset Splitters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:noBorder")]
pub struct NoBorder {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Frameset Splitter Border Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:flatBorders")]
pub struct FlatBorders {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Automatically Merge User Formatting Into Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:autoRedefine")]
pub struct AutoRedefine {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Hide Style From User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:hidden")]
pub struct StyleHidden {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Hide Style From Main User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:semiHidden")]
pub struct SemiHidden {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Remove Semi-Hidden Property When Style Is Used.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:unhideWhenUsed")]
pub struct UnhideWhenUsed {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Primary Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:qFormat")]
pub struct PrimaryStyle {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Style Cannot Be Applied.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:locked")]
pub struct Locked {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:personal")]
pub struct Personal {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Composition Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:personalCompose")]
pub struct PersonalCompose {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Reply Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OnOffOnly/w:personalReply")]
pub struct PersonalReply {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableCellMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcMar/w:tcMar")]
pub struct TableCellMargin {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Table Cell Top Margin Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:top"))]
  pub top_margin: Option<TopMargin>,
  /// Table Cell Left Margin Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:left"))]
  pub left_margin: Option<LeftMargin>,
  /// Defines the StartMargin Class.
  #[sdk(child(office2010, qname = "w:CT_TblWidth/w:start"))]
  pub start_margin: Option<StartMargin>,
  /// Table Cell Bottom Margin Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:bottom"))]
  pub bottom_margin: Option<BottomMargin>,
  /// Table Cell Right Margin Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:right"))]
  pub right_margin: Option<RightMargin>,
  /// Defines the EndMargin Class.
  #[sdk(child(office2010, qname = "w:CT_TblWidth/w:end"))]
  pub end_margin: Option<EndMargin>,
}
/// Defines the TableCellVerticalAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_VerticalTblJc/w:vAlign")]
pub struct TableCellVerticalAlignment {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: TableVerticalAlignmentValues,
}
/// Defines the DivId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonZeroDecimalNumber/w:divId")]
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
#[sdk(qname = "w:CT_Height/w:trHeight")]
pub struct TableRowHeight {
  /// Table Row Height
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub val: Option<crate::simple_type::UInt32Value>,
  /// Table Row Height Type
  #[sdk(attr(qname = "w:hRule"))]
  pub height_type: Option<HeightRuleValues>,
}
/// Defines the TableJustification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblJc/w:jc")]
pub struct TableJustification {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: TableRowAlignmentValues,
}
/// Defines the TablePositionProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPPr/w:tblpPr")]
pub struct TablePositionProperties {
  /// Distance From Left of Table to Text
  #[sdk(attr(qname = "w:leftFromText"))]
  #[sdk(number_range(range = 0..))]
  pub left_from_text: Option<crate::simple_type::Int16Value>,
  /// (Distance From Right of Table to Text
  #[sdk(attr(qname = "w:rightFromText"))]
  #[sdk(number_range(range = 0..))]
  pub right_from_text: Option<crate::simple_type::Int16Value>,
  /// Distance From Top of Table to Text
  #[sdk(attr(qname = "w:topFromText"))]
  #[sdk(number_range(range = 0..))]
  pub top_from_text: Option<crate::simple_type::Int16Value>,
  /// Distance From Bottom of Table to Text
  #[sdk(attr(qname = "w:bottomFromText"))]
  #[sdk(number_range(range = 0..))]
  pub bottom_from_text: Option<crate::simple_type::Int16Value>,
  /// Table Vertical Anchor
  #[sdk(attr(qname = "w:vertAnchor"))]
  pub vertical_anchor: Option<VerticalAnchorValues>,
  /// Table Horizontal Anchor
  #[sdk(attr(qname = "w:horzAnchor"))]
  pub horizontal_anchor: Option<HorizontalAnchorValues>,
  /// Relative Horizontal Alignment From Anchor
  #[sdk(attr(qname = "w:tblpXSpec"))]
  pub table_position_x_alignment: Option<HorizontalAlignmentValues>,
  /// Absolute Horizontal Distance From Anchor
  #[sdk(attr(qname = "w:tblpX"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub table_position_x: Option<crate::simple_type::Int32Value>,
  /// Relative Vertical Alignment from Anchor
  #[sdk(attr(qname = "w:tblpYSpec"))]
  pub table_position_y_alignment: Option<VerticalAlignmentValues>,
  /// Absolute Vertical Distance From Anchor
  #[sdk(attr(qname = "w:tblpY"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub table_position_y: Option<crate::simple_type::Int32Value>,
}
/// Defines the TableOverlap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblOverlap/w:tblOverlap")]
pub struct TableOverlap {
  /// Floating Table Overlap Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: TableOverlapValues,
}
/// Defines the TableStyleRowBandSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedDecimalNumberMax3/w:tblStyleRowBandSize")]
pub struct TableStyleRowBandSize {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TableStyleColumnBandSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedDecimalNumberMax3/w:tblStyleColBandSize")]
pub struct TableStyleColumnBandSize {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 3))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TableIndentation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidthShort/w:tblInd")]
pub struct TableIndentation {
  /// w
  #[sdk(attr(qname = "w:w"))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableBorders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblBorders/w:tblBorders")]
pub struct TableBorders {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Table Top Border
  #[sdk(child(qname = "w:CT_Border/w:top"))]
  pub top_border: Option<TopBorder>,
  /// Table Left Border
  #[sdk(child(qname = "w:CT_Border/w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Defines the StartBorder Class.
  #[sdk(child(office2010, qname = "w:CT_Border/w:start"))]
  pub start_border: Option<StartBorder>,
  /// Table Bottom Border
  #[sdk(child(qname = "w:CT_Border/w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Table Right Border
  #[sdk(child(qname = "w:CT_Border/w:right"))]
  pub right_border: Option<RightBorder>,
  /// Defines the EndBorder Class.
  #[sdk(child(office2010, qname = "w:CT_Border/w:end"))]
  pub end_border: Option<EndBorder>,
  /// Table Inside Horizontal Edges Border
  #[sdk(child(qname = "w:CT_Border/w:insideH"))]
  pub inside_horizontal_border: Option<InsideHorizontalBorder>,
  /// Table Inside Vertical Edges Border
  #[sdk(child(qname = "w:CT_Border/w:insideV"))]
  pub inside_vertical_border: Option<InsideVerticalBorder>,
}
/// Defines the TableLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblLayoutType/w:tblLayout")]
pub struct TableLayout {
  /// Table Layout Setting
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<TableLayoutValues>,
}
/// Defines the TableCellMarginDefault Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblCellMar/w:tblCellMar")]
pub struct TableCellMarginDefault {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Table Cell Top Margin Default
  #[sdk(child(qname = "w:CT_TblWidth/w:top"))]
  pub top_margin: Option<TopMargin>,
  /// Table Cell Left Margin Default
  #[sdk(child(qname = "w:CT_TblWidthDxaNil/w:left"))]
  pub table_cell_left_margin: Option<TableCellLeftMargin>,
  /// Defines the StartMargin Class.
  #[sdk(child(office2010, qname = "w:CT_TblWidth/w:start"))]
  pub start_margin: Option<StartMargin>,
  /// Table Cell Bottom Margin Default
  #[sdk(child(qname = "w:CT_TblWidth/w:bottom"))]
  pub bottom_margin: Option<BottomMargin>,
  /// Table Cell Right Margin Default
  #[sdk(child(qname = "w:CT_TblWidthDxaNil/w:right"))]
  pub table_cell_right_margin: Option<TableCellRightMargin>,
  /// Defines the EndMargin Class.
  #[sdk(child(office2010, qname = "w:CT_TblWidth/w:end"))]
  pub end_margin: Option<EndMargin>,
}
/// Footnote and Endnote Numbering Starting Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdnNumStart/w:numStart")]
pub struct NumberingStart {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Footnote and Endnote Numbering Restart Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NumRestart/w:numRestart")]
pub struct NumberingRestart {
  /// Automatic Numbering Restart Value
  #[sdk(attr(qname = "w:val"))]
  pub val: RestartNumberValues,
}
/// Defines the AltChunk Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_AltChunk/w:altChunk")]
pub struct AltChunk {
  /// Relationship to Part
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// External Content Import Properties
  #[sdk(child(qname = "w:CT_AltChunkPr/w:altChunkPr"))]
  pub alt_chunk_properties: Option<std::boxed::Box<AltChunkProperties>>,
}
/// Defines the TableLook Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblLook/w:tblLook")]
pub struct TableLook {
  pub xml_other_attrs: Vec<(String, String)>,
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub w_val: Option<crate::simple_type::HexBinaryValue>,
  /// firstRow
  #[sdk(attr(office2010, qname = "w:firstRow"))]
  pub first_row: Option<crate::simple_type::OnOffValue>,
  /// lastRow
  #[sdk(attr(office2010, qname = "w:lastRow"))]
  pub last_row: Option<crate::simple_type::OnOffValue>,
  /// firstColumn
  #[sdk(attr(office2010, qname = "w:firstColumn"))]
  pub first_column: Option<crate::simple_type::OnOffValue>,
  /// lastColumn
  #[sdk(attr(office2010, qname = "w:lastColumn"))]
  pub last_column: Option<crate::simple_type::OnOffValue>,
  /// noHBand
  #[sdk(attr(office2010, qname = "w:noHBand"))]
  pub no_horizontal_band: Option<crate::simple_type::OnOffValue>,
  /// noVBand
  #[sdk(attr(office2010, qname = "w:noVBand"))]
  pub no_vertical_band: Option<crate::simple_type::OnOffValue>,
}
/// Defines the FootnoteProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnProps/w:footnotePr")]
pub struct FootnoteProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Footnote Placement
  #[sdk(child(qname = "w:CT_FtnPos/w:pos"))]
  pub footnote_position: Option<FootnotePosition>,
  /// Footnote Numbering Format
  #[sdk(child(qname = "w:CT_NumFmt/w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:CT_FtnEdnNumStart/w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:CT_NumRestart/w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
}
/// Defines the EndnoteProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_EdnProps/w:endnotePr")]
pub struct EndnoteProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Endnote Placement
  #[sdk(child(qname = "w:CT_EdnPos/w:pos"))]
  pub endnote_position: Option<EndnotePosition>,
  /// Endnote Numbering Format
  #[sdk(child(qname = "w:CT_NumFmt/w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:CT_FtnEdnNumStart/w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:CT_NumRestart/w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
}
/// Defines the SectionType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SectType/w:type")]
pub struct SectionType {
  /// Section Type Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: SectionMarkValues,
}
/// Defines the PageSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PageSz/w:pgSz")]
pub struct PageSize {
  /// Page Width
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// Page Height
  #[sdk(attr(qname = "w:h"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// Page Orientation
  #[sdk(attr(qname = "w:orient"))]
  pub orient: Option<PageOrientationValues>,
  /// Printer Paper Code
  #[sdk(attr(qname = "w:code"))]
  pub code: Option<crate::simple_type::UInt16Value>,
}
/// Defines the PageMargin Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PageMar/w:pgMar")]
pub struct PageMargin {
  /// Top Margin Spacing
  #[sdk(attr(qname = "w:top"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// Right Margin Spacing
  #[sdk(attr(qname = "w:right"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub right: Option<crate::simple_type::UInt32Value>,
  /// Page Bottom Spacing
  #[sdk(attr(qname = "w:bottom"))]
  #[sdk(number_range(range = -31680..= 31680))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// Left Margin Spacing
  #[sdk(attr(qname = "w:left"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub left: Option<crate::simple_type::UInt32Value>,
  /// Spacing to Top of Header
  #[sdk(attr(qname = "w:header"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub header: Option<crate::simple_type::UInt32Value>,
  /// Spacing to Bottom of Footer
  #[sdk(attr(qname = "w:footer"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub footer: Option<crate::simple_type::UInt32Value>,
  /// Page Gutter Spacing
  #[sdk(attr(qname = "w:gutter"))]
  #[sdk(number_range(max = 31680, min_inclusive = false))]
  pub gutter: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PaperSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PaperSource/w:paperSrc")]
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
#[sdk(qname = "w:CT_PageBorders/w:pgBorders")]
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
  #[sdk(child(qname = "w:CT_Border/w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Border
  #[sdk(child(qname = "w:CT_Border/w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Bottom Border
  #[sdk(child(qname = "w:CT_Border/w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Border
  #[sdk(child(qname = "w:CT_Border/w:right"))]
  pub right_border: Option<RightBorder>,
}
/// Defines the LineNumberType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LineNumber/w:lnNumType")]
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
  pub distance: Option<crate::simple_type::StringValue>,
  /// Line Numbering Restart Setting
  #[sdk(attr(qname = "w:restart"))]
  pub restart: Option<LineNumberRestartValues>,
}
/// Defines the PageNumberType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PageNumber/w:pgNumType")]
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
#[sdk(qname = "w:CT_Columns/w:cols")]
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
  pub space: Option<crate::simple_type::StringValue>,
  /// Number of Equal Width Columns
  #[sdk(attr(qname = "w:num"))]
  #[sdk(number_range(range = 1..= 45))]
  pub column_count: Option<crate::simple_type::Int16Value>,
  /// Draw Line Between Columns
  #[sdk(attr(qname = "w:sep"))]
  pub separator: Option<crate::simple_type::OnOffValue>,
  /// Single Column Definition.
  #[sdk(child(qname = "w:CT_Column/w:col"))]
  pub w_col: Vec<Column>,
}
/// Defines the VerticalTextAlignmentOnPage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_VerticalJc/w:vAlign")]
pub struct VerticalTextAlignmentOnPage {
  /// Vertical Alignment Setting
  #[sdk(attr(qname = "w:val"))]
  pub val: VerticalJustificationValues,
}
/// Defines the DocGrid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocGrid/w:docGrid")]
pub struct DocGrid {
  /// Document Grid Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<DocGridValues>,
  /// Document Grid Line Pitch
  #[sdk(attr(qname = "w:linePitch"))]
  pub line_pitch: Option<crate::simple_type::Int32Value>,
  /// Document Grid Character Pitch
  #[sdk(attr(qname = "w:charSpace"))]
  pub character_space: Option<crate::simple_type::Int32Value>,
}
/// Inclusion/Exclusion Data for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Recipients/w:recipients")]
pub struct Recipients {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Data About Single Data Source Record.
  #[sdk(child(qname = "w:CT_RecipientData/w:recipientData"))]
  pub w_recipient_data: Vec<RecipientData>,
}
/// Rich Text Box Content Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TxbxContent/w:txbxContent")]
pub struct TextBoxContent {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub text_box_content_choice: Vec<TextBoxContentChoice>,
}
/// Comments Collection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Comments/w:comments")]
pub struct Comments {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Comment Content.
  #[sdk(child(qname = "w:CT_Comment/w:comment"))]
  pub w_comment: Vec<Comment>,
}
/// Document Footnotes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Footnotes/w:footnotes")]
pub struct Footnotes {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Footnote Content.
  #[sdk(child(qname = "w:CT_FtnEdn/w:footnote"))]
  pub w_footnote: Vec<Footnote>,
}
/// Document Endnotes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Endnotes/w:endnotes")]
pub struct Endnotes {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Endnote Content.
  #[sdk(child(qname = "w:CT_FtnEdn/w:endnote"))]
  pub w_endnote: Vec<Endnote>,
}
/// Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HdrFtr/w:hdr")]
pub struct Header {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub header_choice: Vec<HeaderChoice>,
}
/// Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HdrFtr/w:ftr")]
pub struct Footer {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub footer_choice: Vec<FooterChoice>,
}
/// Document Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Settings/w:settings")]
pub struct Settings {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Write Protection
  #[sdk(child(qname = "w:CT_WriteProtection/w:writeProtection"))]
  pub write_protection: Option<WriteProtection>,
  /// Document View Setting
  #[sdk(child(qname = "w:CT_View/w:view"))]
  pub view: Option<View>,
  /// Magnification Setting
  #[sdk(child(qname = "w:CT_Zoom/w:zoom"))]
  pub zoom: Option<Zoom>,
  /// Remove Personal Information from Document Properties
  #[sdk(child(qname = "w:CT_OnOff/w:removePersonalInformation"))]
  pub remove_personal_information: Option<RemovePersonalInformation>,
  /// Remove Date and Time from Annotations
  #[sdk(child(qname = "w:CT_OnOff/w:removeDateAndTime"))]
  pub remove_date_and_time: Option<RemoveDateAndTime>,
  /// Do Not Display Visual Boundary For Header/Footer or Between Pages
  #[sdk(child(qname = "w:CT_OnOff/w:doNotDisplayPageBoundaries"))]
  pub do_not_display_page_boundaries: Option<DoNotDisplayPageBoundaries>,
  /// Display Background Objects When Displaying Document
  #[sdk(child(qname = "w:CT_OnOff/w:displayBackgroundShape"))]
  pub display_background_shape: Option<DisplayBackgroundShape>,
  /// Print PostScript Codes With Document Text
  #[sdk(child(qname = "w:CT_OnOff/w:printPostScriptOverText"))]
  pub print_post_script_over_text: Option<PrintPostScriptOverText>,
  /// Print Fractional Character Widths
  #[sdk(child(qname = "w:CT_OnOff/w:printFractionalCharacterWidth"))]
  pub print_fractional_character_width: Option<PrintFractionalCharacterWidth>,
  /// Only Print Form Field Content
  #[sdk(child(qname = "w:CT_OnOff/w:printFormsData"))]
  pub print_forms_data: Option<PrintFormsData>,
  /// Embed TrueType Fonts
  #[sdk(child(qname = "w:CT_OnOff/w:embedTrueTypeFonts"))]
  pub embed_true_type_fonts: Option<EmbedTrueTypeFonts>,
  /// Embed Common System Fonts
  #[sdk(child(qname = "w:CT_OnOff/w:embedSystemFonts"))]
  pub embed_system_fonts: Option<EmbedSystemFonts>,
  /// Subset Fonts When Embedding
  #[sdk(child(qname = "w:CT_OnOff/w:saveSubsetFonts"))]
  pub save_subset_fonts: Option<SaveSubsetFonts>,
  /// Only Save Form Field Content
  #[sdk(child(qname = "w:CT_OnOff/w:saveFormsData"))]
  pub save_forms_data: Option<SaveFormsData>,
  /// Mirror Page Margins
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorMargins"))]
  pub mirror_margins: Option<MirrorMargins>,
  /// Align Paragraph and Table Borders with Page Border
  #[sdk(child(qname = "w:CT_OnOff/w:alignBordersAndEdges"))]
  pub align_border_and_edges: Option<AlignBorderAndEdges>,
  /// Page Border Excludes Header
  #[sdk(child(qname = "w:CT_OnOff/w:bordersDoNotSurroundHeader"))]
  pub borders_do_not_surround_header: Option<BordersDoNotSurroundHeader>,
  /// Page Border Excludes Footer
  #[sdk(child(qname = "w:CT_OnOff/w:bordersDoNotSurroundFooter"))]
  pub borders_do_not_surround_footer: Option<BordersDoNotSurroundFooter>,
  /// Position Gutter At Top of Page
  #[sdk(child(qname = "w:CT_OnOff/w:gutterAtTop"))]
  pub gutter_at_top: Option<GutterAtTop>,
  /// Do Not Display Visual Indication of Spelling Errors
  #[sdk(child(qname = "w:CT_OnOff/w:hideSpellingErrors"))]
  pub hide_spelling_errors: Option<HideSpellingErrors>,
  /// Do Not Display Visual Indication of Grammatical Errors
  #[sdk(child(qname = "w:CT_OnOff/w:hideGrammaticalErrors"))]
  pub hide_grammatical_errors: Option<HideGrammaticalErrors>,
  /// Grammar Checking Settings.
  #[sdk(child(qname = "w:CT_WritingStyle/w:activeWritingStyle"))]
  pub w_active_writing_style: Vec<ActiveWritingStyle>,
  /// Spelling and Grammatical Checking State.
  #[sdk(child(qname = "w:CT_Proof/w:proofState"))]
  pub w_proof_state: Option<ProofState>,
  /// Structured Document Tag Placeholder Text Should be Resaved.
  #[sdk(child(qname = "w:CT_OnOff/w:formsDesign"))]
  pub w_forms_design: Option<FormsDesign>,
  /// Attached Document Template.
  #[sdk(child(qname = "w:CT_Rel/w:attachedTemplate"))]
  pub w_attached_template: Option<AttachedTemplate>,
  /// Automatically Update Styles From Document Template.
  #[sdk(child(qname = "w:CT_OnOff/w:linkStyles"))]
  pub w_link_styles: Option<LinkStyles>,
  /// Suggested Filtering for List of Document Styles.
  #[sdk(child(qname = "w:CT_StylePaneFormatFilter/w:stylePaneFormatFilter"))]
  pub w_style_pane_format_filter: Option<StylePaneFormatFilter>,
  /// Suggested Sorting for List of Document Styles.
  #[sdk(child(qname = "w:CT_StylePaneSortMethods/w:stylePaneSortMethod"))]
  pub w_style_pane_sort_method: Option<StylePaneSortMethods>,
  /// Document Classification.
  #[sdk(child(qname = "w:CT_DocType/w:documentType"))]
  pub w_document_type: Option<DocumentType>,
  /// Mail Merge Settings.
  #[sdk(child(qname = "w:CT_MailMerge/w:mailMerge"))]
  pub w_mail_merge: Option<std::boxed::Box<MailMerge>>,
  /// Visibility of Annotation Types.
  #[sdk(child(qname = "w:CT_TrackChangesView/w:revisionView"))]
  pub w_revision_view: Option<RevisionView>,
  /// Track Revisions to Document.
  #[sdk(child(qname = "w:CT_OnOff/w:trackRevisions"))]
  pub w_track_revisions: Option<TrackRevisions>,
  /// Do Not Use Move Syntax When Tracking Revisions.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotTrackMoves"))]
  pub w_do_not_track_moves: Option<DoNotTrackMoves>,
  /// Do Not Track Formatting Revisions When Tracking Revisions.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotTrackFormatting"))]
  pub w_do_not_track_formatting: Option<DoNotTrackFormatting>,
  /// Document Editing Restrictions.
  #[sdk(child(qname = "w:CT_DocProtect/w:documentProtection"))]
  pub w_document_protection: Option<DocumentProtection>,
  /// Allow Automatic Formatting to Override Formatting Protection Settings.
  #[sdk(child(qname = "w:CT_OnOff/w:autoFormatOverride"))]
  pub w_auto_format_override: Option<AutoFormatOverride>,
  /// Prevent Modification of Themes Part.
  #[sdk(child(qname = "w:CT_OnOff/w:styleLockTheme"))]
  pub w_style_lock_theme: Option<StyleLockThemesPart>,
  /// Prevent Replacement of Styles Part.
  #[sdk(child(qname = "w:CT_OnOff/w:styleLockQFSet"))]
  pub w_style_lock_qf_set: Option<StyleLockStylesPart>,
  /// Distance Between Automatic Tab Stops.
  #[sdk(child(qname = "w:CT_NonNegativeShort/w:defaultTabStop"))]
  pub w_default_tab_stop: Option<DefaultTabStop>,
  /// Automatically Hyphenate Document Contents When Displayed.
  #[sdk(child(qname = "w:CT_OnOff/w:autoHyphenation"))]
  pub w_auto_hyphenation: Option<AutoHyphenation>,
  /// Maximum Number of Consecutively Hyphenated Lines.
  #[sdk(child(qname = "w:CT_UnsignedShortNumber/w:consecutiveHyphenLimit"))]
  pub w_consecutive_hyphen_limit: Option<ConsecutiveHyphenLimit>,
  /// Hyphenation Zone.
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:hyphenationZone"))]
  pub w_hyphenation_zone: Option<HyphenationZone>,
  /// Do Not Hyphenate Words in ALL CAPITAL LETTERS.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotHyphenateCaps"))]
  pub w_do_not_hyphenate_caps: Option<DoNotHyphenateCaps>,
  /// Show E-Mail Message Header.
  #[sdk(child(qname = "w:CT_OnOff/w:showEnvelope"))]
  pub w_show_envelope: Option<ShowEnvelope>,
  /// Percentage of Document to Use When Generating Summary.
  #[sdk(child(qname = "w:CT_UnsignedInt100/w:summaryLength"))]
  pub w_summary_length: Option<SummaryLength>,
  /// Paragraph Style Applied to Automatically Generated Paragraphs.
  #[sdk(child(qname = "w:CT_String253/w:clickAndTypeStyle"))]
  pub w_click_and_type_style: Option<ClickAndTypeStyle>,
  /// Default Table Style for Newly Inserted Tables.
  #[sdk(child(qname = "w:CT_String253/w:defaultTableStyle"))]
  pub w_default_table_style: Option<DefaultTableStyle>,
  /// Different Even/Odd Page Headers and Footers.
  #[sdk(child(qname = "w:CT_OnOff/w:evenAndOddHeaders"))]
  pub w_even_and_odd_headers: Option<EvenAndOddHeaders>,
  /// Reverse Book Fold Printing.
  #[sdk(child(qname = "w:CT_OnOff/w:bookFoldRevPrinting"))]
  pub w_book_fold_rev_printing: Option<BookFoldReversePrinting>,
  /// Book Fold Printing.
  #[sdk(child(qname = "w:CT_OnOff/w:bookFoldPrinting"))]
  pub w_book_fold_printing: Option<BookFoldPrinting>,
  /// Number of Pages Per Booklet.
  #[sdk(child(qname = "w:CT_NonNegativeShort/w:bookFoldPrintingSheets"))]
  pub w_book_fold_printing_sheets: Option<BookFoldPrintingSheets>,
  /// Drawing Grid Horizontal Grid Unit Size.
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:drawingGridHorizontalSpacing"))]
  pub w_drawing_grid_horizontal_spacing: Option<DrawingGridHorizontalSpacing>,
  /// Drawing Grid Vertical Grid Unit Size.
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:drawingGridVerticalSpacing"))]
  pub w_drawing_grid_vertical_spacing: Option<DrawingGridVerticalSpacing>,
  /// Distance between Horizontal Gridlines.
  #[sdk(child(qname = "w:CT_UnsignedInt7/w:displayHorizontalDrawingGridEvery"))]
  pub w_display_horizontal_drawing_grid_every: Option<DisplayHorizontalDrawingGrid>,
  /// Distance between Vertical Gridlines.
  #[sdk(child(qname = "w:CT_UnsignedInt7/w:displayVerticalDrawingGridEvery"))]
  pub w_display_vertical_drawing_grid_every: Option<DisplayVerticalDrawingGrid>,
  /// Do Not Use Margins for Drawing Grid Origin.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotUseMarginsForDrawingGridOrigin"))]
  pub w_do_not_use_margins_for_drawing_grid_origin: Option<DoNotUseMarginsForDrawingGridOrigin>,
  /// Drawing Grid Horizontal Origin Point.
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:drawingGridHorizontalOrigin"))]
  pub w_drawing_grid_horizontal_origin: Option<DrawingGridHorizontalOrigin>,
  /// Drawing Grid Vertical Origin Point.
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:drawingGridVerticalOrigin"))]
  pub w_drawing_grid_vertical_origin: Option<DrawingGridVerticalOrigin>,
  /// Do Not Show Visual Indicator For Form Fields.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotShadeFormData"))]
  pub w_do_not_shade_form_data: Option<DoNotShadeFormData>,
  /// Never Kern Punctuation Characters.
  #[sdk(child(qname = "w:CT_OnOff/w:noPunctuationKerning"))]
  pub w_no_punctuation_kerning: Option<NoPunctuationKerning>,
  /// Character-Level Whitespace Compression.
  #[sdk(child(qname = "w:CT_CharacterSpacing/w:characterSpacingControl"))]
  pub w_character_spacing_control: Option<CharacterSpacingControl>,
  /// Print Two Pages Per Sheet.
  #[sdk(child(qname = "w:CT_OnOff/w:printTwoOnOne"))]
  pub w_print_two_on_one: Option<PrintTwoOnOne>,
  /// Use Strict Kinsoku Rules for Japanese Text.
  #[sdk(child(qname = "w:CT_OnOff/w:strictFirstAndLastChars"))]
  pub w_strict_first_and_last_chars: Option<StrictFirstAndLastChars>,
  /// Custom Set of Characters Which Cannot End a Line.
  #[sdk(child(qname = "w:CT_KinsokuAfter/w:noLineBreaksAfter"))]
  pub w_no_line_breaks_after: Option<NoLineBreaksAfterKinsoku>,
  /// Custom Set Of Characters Which Cannot Begin A Line.
  #[sdk(child(qname = "w:CT_KinsokuBefore/w:noLineBreaksBefore"))]
  pub w_no_line_breaks_before: Option<NoLineBreaksBeforeKinsoku>,
  /// Generate Thumbnail For Document On Save.
  #[sdk(child(qname = "w:CT_OnOff/w:savePreviewPicture"))]
  pub w_save_preview_picture: Option<SavePreviewPicture>,
  /// Do Not Validate Custom XML Markup Against Schemas.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotValidateAgainstSchema"))]
  pub w_do_not_validate_against_schema: Option<DoNotValidateAgainstSchema>,
  /// Allow Saving Document As XML File When Custom XML Markup Is Invalid.
  #[sdk(child(qname = "w:CT_OnOff/w:saveInvalidXml"))]
  pub w_save_invalid_xml: Option<SaveInvalidXml>,
  /// Ignore Mixed Content When Validating Custom XML Markup.
  #[sdk(child(qname = "w:CT_OnOff/w:ignoreMixedContent"))]
  pub w_ignore_mixed_content: Option<IgnoreMixedContent>,
  /// Use Custom XML Element Names as Default Placeholder Text.
  #[sdk(child(qname = "w:CT_OnOff/w:alwaysShowPlaceholderText"))]
  pub w_always_show_placeholder_text: Option<AlwaysShowPlaceholderText>,
  /// Do Not Show Visual Indicator For Invalid Custom XML Markup.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotDemarcateInvalidXml"))]
  pub w_do_not_demarcate_invalid_xml: Option<DoNotDemarcateInvalidXml>,
  /// Only Save Custom XML Markup.
  #[sdk(child(qname = "w:CT_OnOff/w:saveXmlDataOnly"))]
  pub w_save_xml_data_only: Option<SaveXmlDataOnly>,
  /// Save Document as XML File through Custom XSL Transform.
  #[sdk(child(qname = "w:CT_OnOff/w:useXSLTWhenSaving"))]
  pub w_use_xslt_when_saving: Option<UseXsltWhenSaving>,
  /// Custom XSL Transform To Use When Saving As XML File.
  #[sdk(child(qname = "w:CT_SaveThroughXslt/w:saveThroughXslt"))]
  pub w_save_through_xslt: Option<SaveThroughXslt>,
  /// Show Visual Indicators for Custom XML Markup Start/End Locations.
  #[sdk(child(qname = "w:CT_OnOff/w:showXMLTags"))]
  pub w_show_xml_tags: Option<ShowXmlTags>,
  /// Do Not Mark Custom XML Elements With No Namespace As Invalid.
  #[sdk(child(qname = "w:CT_OnOff/w:alwaysMergeEmptyNamespace"))]
  pub w_always_merge_empty_namespace: Option<AlwaysMergeEmptyNamespace>,
  /// Automatically Recalculate Fields on Open.
  #[sdk(child(qname = "w:CT_OnOff/w:updateFields"))]
  pub w_update_fields: Option<UpdateFieldsOnOpen>,
  /// Default Properties for VML Objects in Header and Footer.
  #[sdk(child(qname = "w:CT_ShapeDefaults/w:hdrShapeDefaults"))]
  pub w_hdr_shape_defaults: Option<HeaderShapeDefaults>,
  /// Document-Wide Footnote Properties.
  #[sdk(child(qname = "w:CT_FtnDocProps/w:footnotePr"))]
  pub w_footnote_pr: Option<std::boxed::Box<FootnoteDocumentWideProperties>>,
  /// Document-Wide Endnote Properties.
  #[sdk(child(qname = "w:CT_EdnDocProps/w:endnotePr"))]
  pub w_endnote_pr: Option<std::boxed::Box<EndnoteDocumentWideProperties>>,
  /// Compatibility Settings.
  #[sdk(child(qname = "w:CT_Compat/w:compat"))]
  pub w_compat: Option<std::boxed::Box<Compatibility>>,
  /// Document Variables.
  #[sdk(child(qname = "w:CT_DocVars/w:docVars"))]
  pub w_doc_vars: Option<DocumentVariables>,
  /// Listing of All Revision Save ID Values.
  #[sdk(child(qname = "w:CT_DocRsids/w:rsids"))]
  pub w_rsids: Option<std::boxed::Box<Rsids>>,
  /// Math Properties.
  #[sdk(child(qname = "m:CT_MathPr/m:mathPr"))]
  pub m_math_pr: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathProperties,
    >,
  >,
  /// Disable Features Incompatible With Earlier Word Processing Formats.
  #[sdk(child(qname = "w:CT_OnOff/w:uiCompat97To2003"))]
  pub w_ui_compat97_to2003: Option<UiCompatibleWith97To2003>,
  /// Attached Custom XML Schema.
  #[sdk(child(qname = "w:CT_String/w:attachedSchema"))]
  pub w_attached_schema: Vec<AttachedSchema>,
  /// Theme Font Languages.
  #[sdk(child(qname = "w:CT_Language/w:themeFontLang"))]
  pub w_theme_font_lang: Option<ThemeFontLanguages>,
  /// Theme Color Mappings.
  #[sdk(child(qname = "w:CT_ColorSchemeMapping/w:clrSchemeMapping"))]
  pub w_clr_scheme_mapping: Option<ColorSchemeMapping>,
  /// Do Not Include Content in Text Boxes, Footnotes, and Endnotes in Document Statistics.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotIncludeSubdocsInStats"))]
  pub w_do_not_include_subdocs_in_stats: Option<DoNotIncludeSubdocsInStats>,
  /// Do Not Automatically Compress Images.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotAutoCompressPictures"))]
  pub w_do_not_auto_compress_pictures: Option<DoNotAutoCompressPictures>,
  /// Upgrade Document on Open.
  #[sdk(empty_child(qname = "w:CT_Empty/w:forceUpgrade"))]
  pub w_force_upgrade: Option<()>,
  /// Caption Settings.
  #[sdk(child(qname = "w:CT_Captions/w:captions"))]
  pub w_captions: Option<std::boxed::Box<Captions>>,
  /// Freeze Document Layout.
  #[sdk(child(qname = "w:CT_ReadingModeInkLockDown/w:readModeInkLockDown"))]
  pub w_read_mode_ink_lock_down: Option<ReadModeInkLockDown>,
  /// Embedded Custom XML Schema Supplementary Data.
  #[sdk(child(qname = "sl:CT_SchemaLibrary/sl:schemaLibrary"))]
  pub sl_schema_library:
    Option<crate::schemas::schemas_openxmlformats_org_schema_library_2006_main::SchemaLibrary>,
  /// Default Properties for VML Objects in Main Document.
  #[sdk(child(qname = "w:CT_ShapeDefaults/w:shapeDefaults"))]
  pub w_shape_defaults: Option<ShapeDefaults>,
  /// Radix Point for Field Code Evaluation.
  #[sdk(child(qname = "w:CT_String/w:decimalSymbol"))]
  pub w_decimal_symbol: Option<DecimalSymbol>,
  /// List Separator for Field Code Evaluation.
  #[sdk(child(qname = "w:CT_String/w:listSeparator"))]
  pub w_list_separator: Option<ListSeparator>,
  /// Defines the DocumentId Class.
  #[sdk(child(office2010, qname = "w14:CT_LongHexNumber/w14:docId"))]
  pub w14_doc_id: Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DocumentId>,
  /// Defines the DiscardImageEditingData Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:discardImageEditingData"))]
  pub w14_discard_image_editing_data:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DiscardImageEditingData>,
  /// Defines the DefaultImageDpi Class.
  #[sdk(child(office2010, qname = "w14:CT_DefaultImageDpi/w14:defaultImageDpi"))]
  pub w14_default_image_dpi:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DefaultImageDpi>,
  /// Defines the ConflictMode Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:conflictMode"))]
  pub w14_conflict_mode:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictMode>,
  /// Defines the ChartTrackingRefBased Class.
  #[sdk(child(office2013, qname = "w:CT_OnOff/w15:chartTrackingRefBased"))]
  pub w15_chart_tracking_ref_based:
    Option<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::ChartTrackingRefBased>,
  /// Defines the PersistentDocumentId Class.
  #[sdk(child(office2013, qname = "w15:CT_Guid/w15:docId"))]
  pub w15_doc_id:
    Option<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::PersistentDocumentId>,
}
/// Web Page Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_WebSettings/w:webSettings")]
pub struct WebSettings {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Nested Frameset Definition.
  #[sdk(child(qname = "w:CT_Frameset/w:frameset"))]
  pub frameset: Option<std::boxed::Box<Frameset>>,
  /// Defines the Divs Class.
  #[sdk(child(qname = "w:CT_Divs/w:divs"))]
  pub divs: Option<Divs>,
  /// Defines the WebPageEncoding Class.
  #[sdk(child(qname = "w:CT_String/w:encoding"))]
  pub web_page_encoding: Option<WebPageEncoding>,
  /// Defines the OptimizeForBrowser Class.
  #[sdk(child(qname = "w:CT_OnOff/w:optimizeForBrowser"))]
  pub optimize_for_browser: Option<OptimizeForBrowser>,
  /// Defines the RelyOnVML Class.
  #[sdk(child(qname = "w:CT_OnOff/w:relyOnVML"))]
  pub rely_on_vml: Option<RelyOnVml>,
  /// Defines the AllowPNG Class.
  #[sdk(child(qname = "w:CT_OnOff/w:allowPNG"))]
  pub allow_png: Option<AllowPng>,
  /// Defines the DoNotRelyOnCSS Class.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotRelyOnCSS"))]
  pub do_not_rely_on_css: Option<DoNotRelyOnCss>,
  /// Defines the DoNotSaveAsSingleFile Class.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotSaveAsSingleFile"))]
  pub do_not_save_as_single_file: Option<DoNotSaveAsSingleFile>,
  /// Defines the DoNotOrganizeInFolder Class.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotOrganizeInFolder"))]
  pub do_not_organize_in_folder: Option<DoNotOrganizeInFolder>,
  /// Defines the DoNotUseLongFileNames Class.
  #[sdk(child(qname = "w:CT_OnOff/w:doNotUseLongFileNames"))]
  pub do_not_use_long_file_names: Option<DoNotUseLongFileNames>,
  /// Defines the PixelsPerInch Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:pixelsPerInch"))]
  pub pixels_per_inch: Option<PixelsPerInch>,
  /// Defines the TargetScreenSize Class.
  #[sdk(child(qname = "w:CT_TargetScreenSz/w:targetScreenSz"))]
  pub target_screen_size: Option<TargetScreenSize>,
}
/// Font Table Root Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FontsList/w:fonts")]
pub struct Fonts {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(qname = "w:CT_Font/w:font", any))]
  pub xml_children: Vec<FontsChoice>,
}
/// Numbering Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Numbering/w:numbering")]
pub struct Numbering {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Picture Numbering Symbol Definition.
  #[sdk(child(qname = "w:CT_NumPicBullet/w:numPicBullet"))]
  pub w_num_pic_bullet: Vec<NumberingPictureBullet>,
  /// Abstract Numbering Definition.
  #[sdk(child(qname = "w:CT_AbstractNum/w:abstractNum"))]
  pub w_abstract_num: Vec<AbstractNum>,
  /// Numbering Definition Instance.
  #[sdk(child(qname = "w:CT_Num/w:num"))]
  pub w_num: Vec<NumberingInstance>,
  /// Last Reviewed Abstract Numbering Definition.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:numIdMacAtCleanup"))]
  pub w_num_id_mac_at_cleanup: Option<NumberingIdMacAtCleanup>,
}
/// Style Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Styles/w:styles")]
pub struct Styles {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Document Default Paragraph and Run Properties
  #[sdk(child(qname = "w:CT_DocDefaults/w:docDefaults"))]
  pub doc_defaults: Option<std::boxed::Box<DocDefaults>>,
  /// Latent Style Information
  #[sdk(child(qname = "w:CT_LatentStyles/w:latentStyles"))]
  pub latent_styles: Option<LatentStyles>,
  /// Style Definition.
  #[sdk(child(qname = "w:CT_Style/w:style"))]
  pub w_style: Vec<Style>,
}
/// Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Document/w:document")]
pub struct Document {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// conformance
  #[sdk(attr(qname = "w:conformance"))]
  pub w_conformance: Option<DocumentConformance>,
  /// Document Background
  #[sdk(child(qname = "w:CT_Background/w:background"))]
  pub document_background: Option<std::boxed::Box<DocumentBackground>>,
  /// Defines the Body Class.
  #[sdk(child(qname = "w:CT_Body/w:body"))]
  pub body: Option<std::boxed::Box<Body>>,
}
/// Glossary Document Root Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_GlossaryDocument/w:glossaryDocument")]
pub struct GlossaryDocument {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Document Background
  #[sdk(child(qname = "w:CT_Background/w:background"))]
  pub document_background: Option<std::boxed::Box<DocumentBackground>>,
  /// List of Glossary Document Entries
  #[sdk(child(qname = "w:CT_DocParts/w:docParts"))]
  pub doc_parts: Option<std::boxed::Box<DocParts>>,
}
/// Previous Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrExBase/w:tblPrEx")]
pub struct PreviousTablePropertyExceptions {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Preferred Table Width Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Table Alignment Exception
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Table Cell Spacing Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Table Indent from Leading Margin Exception
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Table Borders Exceptions
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Table Shading Exception
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Table Layout Exception
  #[sdk(child(qname = "w:CT_TblLayoutType/w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Table Cell Margin Exceptions
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Table Style Conditional Formatting Settings Exception
  #[sdk(child(qname = "w:CT_TblLook/w:tblLook"))]
  pub table_look: Option<TableLook>,
}
/// Previous Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcPrInner/w:tcPr")]
pub struct PreviousTableCellProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Defines the TableCellWidth Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tcW"))]
  pub table_cell_width: Option<TableCellWidth>,
  /// Defines the GridSpan Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridSpan"))]
  pub grid_span: Option<GridSpan>,
  /// Defines the HorizontalMerge Class.
  #[sdk(child(qname = "w:CT_HMerge/w:hMerge"))]
  pub horizontal_merge: Option<HorizontalMerge>,
  /// Defines the VerticalMerge Class.
  #[sdk(child(qname = "w:CT_VMerge/w:vMerge"))]
  pub vertical_merge: Option<VerticalMerge>,
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:CT_TcBorders/w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:CT_TcMar/w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TableCellFitText Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:tcFitText"))]
  pub table_cell_fit_text: Option<TableCellFitText>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalTblJc/w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
  /// Defines the HideMark Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:hideMark"))]
  pub hide_mark: Option<HideMark>,
  #[sdk(choice(
    qname = "w:CT_TrackChange/w:cellIns",
    qname = "w:CT_TrackChange/w:cellDel",
    qname = "w:CT_CellMergeTrackChange/w:cellMerge"
  ))]
  pub previous_table_cell_properties_choice: Option<PreviousTableCellPropertiesChoice>,
}
/// Previous Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrPrBase/w:trPr")]
pub struct PreviousTableRowProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_Cnf/w:cnfStyle",
    qname = "w:CT_NonZeroDecimalNumber/w:divId",
    qname = "w:CT_DecimalNumber/w:gridBefore",
    qname = "w:CT_DecimalNumber/w:gridAfter",
    qname = "w:CT_TblWidth/w:wBefore",
    qname = "w:CT_TblWidth/w:wAfter",
    qname = "w:CT_Height/w:trHeight",
    qname = "w:CT_OnOff/w:hidden",
    qname = "w:CT_OnOffOnly/w:cantSplit",
    qname = "w:CT_OnOffOnly/w:tblHeader",
    qname = "w:CT_TblWidth/w:tblCellSpacing",
    qname = "w:CT_TblJc/w:jc",
    text,
    any
  ))]
  pub previous_table_row_properties_choice: Vec<PreviousTableRowPropertiesChoice>,
}
/// Previous Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrBase/w:tblPr")]
pub struct PreviousTableProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the TableStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:tblStyle"))]
  pub table_style: Option<TableStyle>,
  /// Defines the TablePositionProperties Class.
  #[sdk(child(qname = "w:CT_TblPPr/w:tblpPr"))]
  pub table_position_properties: Option<TablePositionProperties>,
  /// Defines the TableOverlap Class.
  #[sdk(child(qname = "w:CT_TblOverlap/w:tblOverlap"))]
  pub table_overlap: Option<TableOverlap>,
  /// Defines the BiDiVisual Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:bidiVisual"))]
  pub bi_di_visual: Option<BiDiVisual>,
  /// Defines the TableWidth Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableLayout Class.
  #[sdk(child(qname = "w:CT_TblLayoutType/w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Defines the TableLook Class.
  #[sdk(child(qname = "w:CT_TblLook/w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Defines the TableCaption Class.
  #[sdk(child(office2010, qname = "w:CT_String/w:tblCaption"))]
  pub table_caption: Option<TableCaption>,
  /// Defines the TableDescription Class.
  #[sdk(child(office2010, qname = "w:CT_String/w:tblDescription"))]
  pub table_description: Option<TableDescription>,
}
/// Previous Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SectPrBase/w:sectPr")]
pub struct PreviousSectionProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
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
  #[sdk(child(qname = "w:CT_FtnProps/w:footnotePr"))]
  pub footnote_properties: Option<std::boxed::Box<FootnoteProperties>>,
  /// Defines the EndnoteProperties Class.
  #[sdk(child(qname = "w:CT_EdnProps/w:endnotePr"))]
  pub endnote_properties: Option<std::boxed::Box<EndnoteProperties>>,
  /// Defines the SectionType Class.
  #[sdk(child(qname = "w:CT_SectType/w:type"))]
  pub section_type: Option<SectionType>,
  /// Defines the PageSize Class.
  #[sdk(child(qname = "w:CT_PageSz/w:pgSz"))]
  pub page_size: Option<PageSize>,
  /// Defines the PageMargin Class.
  #[sdk(child(qname = "w:CT_PageMar/w:pgMar"))]
  pub page_margin: Option<PageMargin>,
  /// Defines the PaperSource Class.
  #[sdk(child(qname = "w:CT_PaperSource/w:paperSrc"))]
  pub paper_source: Option<PaperSource>,
  /// Defines the PageBorders Class.
  #[sdk(child(qname = "w:CT_PageBorders/w:pgBorders"))]
  pub page_borders: Option<std::boxed::Box<PageBorders>>,
  /// Defines the LineNumberType Class.
  #[sdk(child(qname = "w:CT_LineNumber/w:lnNumType"))]
  pub line_number_type: Option<LineNumberType>,
  /// Defines the PageNumberType Class.
  #[sdk(child(qname = "w:CT_PageNumber/w:pgNumType"))]
  pub page_number_type: Option<PageNumberType>,
  /// Defines the Columns Class.
  #[sdk(child(qname = "w:CT_Columns/w:cols"))]
  pub columns: Option<Columns>,
  /// Defines the FormProtection Class.
  #[sdk(child(qname = "w:CT_OnOff/w:formProt"))]
  pub form_protection: Option<FormProtection>,
  /// Defines the VerticalTextAlignmentOnPage Class.
  #[sdk(child(qname = "w:CT_VerticalJc/w:vAlign"))]
  pub vertical_text_alignment_on_page: Option<VerticalTextAlignmentOnPage>,
  /// Defines the NoEndnote Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noEndnote"))]
  pub no_endnote: Option<NoEndnote>,
  /// Defines the TitlePage Class.
  #[sdk(child(qname = "w:CT_OnOff/w:titlePg"))]
  pub title_page: Option<TitlePage>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the GutterOnRight Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtlGutter"))]
  pub gutter_on_right: Option<GutterOnRight>,
  /// Defines the DocGrid Class.
  #[sdk(child(qname = "w:CT_DocGrid/w:docGrid"))]
  pub doc_grid: Option<DocGrid>,
  /// Defines the PrinterSettingsReference Class.
  #[sdk(child(qname = "w:CT_Rel/w:printerSettings"))]
  pub printer_settings_reference: Option<PrinterSettingsReference>,
  /// Defines the FootnoteColumns Class.
  #[sdk(child(office2013, qname = "w:CT_DecimalNumber/w15:footnoteColumns"))]
  pub footnote_columns:
    Option<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::FootnoteColumns>,
}
/// Previous Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrExtended/w:pPr")]
pub struct ParagraphPropertiesExtended {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:CT_String/w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:CT_OnOff/w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:CT_FramePr/w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:CT_OnOff/w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:CT_NumPr/w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:CT_PBdr/w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:CT_Tabs/w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:CT_OnOff/w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:CT_OnOff/w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:CT_Spacing/w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:CT_Ind/w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:CT_OnOff/w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:CT_Jc/w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:CT_TextAlignment/w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Defines the DivId Class.
  #[sdk(child(qname = "w:CT_NonZeroDecimalNumber/w:divId"))]
  pub div_id: Option<DivId>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
}
/// Previous Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPrOriginal/w:rPr")]
pub struct PreviousRunProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the RunStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:rStyle"))]
  pub w_r_style: Vec<RunStyle>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub w_r_fonts: Vec<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub w_b: Vec<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub w_b_cs: Vec<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub w_i: Vec<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub w_i_cs: Vec<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub w_caps: Vec<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub w_small_caps: Vec<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub w_strike: Vec<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub w_dstrike: Vec<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub w_outline: Vec<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub w_shadow: Vec<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub w_emboss: Vec<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub w_imprint: Vec<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub w_no_proof: Vec<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub w_snap_to_grid: Vec<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub w_vanish: Vec<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub w_web_hidden: Vec<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub w_color: Vec<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub w_spacing: Vec<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub w_w: Vec<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub w_kern: Vec<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub w_position: Vec<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub w_sz: Vec<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub w_sz_cs: Vec<FontSizeComplexScript>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "w:CT_Highlight/w:highlight"))]
  pub w_highlight: Vec<Highlight>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub w_u: Vec<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub w_effect: Vec<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub w_bdr: Vec<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub w_shd: Vec<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub w_fit_text: Vec<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub w_vert_align: Vec<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtl"))]
  pub w_rtl: Vec<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:cs"))]
  pub w_cs: Vec<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub w_em: Vec<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub w_lang: Vec<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub w_east_asian_layout: Vec<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub w_spec_vanish: Vec<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(office2010, qname = "w14:CT_Glow/w14:glow"))]
  pub w14_glow: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow>,
  /// Defines the Shadow Class.
  #[sdk(child(office2010, qname = "w14:CT_Shadow/w14:shadow"))]
  pub w14_shadow: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow>,
  /// Defines the Reflection Class.
  #[sdk(child(office2010, qname = "w14:CT_Reflection/w14:reflection"))]
  pub w14_reflection:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_TextOutlineEffect/w14:textOutline"))]
  pub w14_text_outline:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_FillTextEffect/w14:textFill"))]
  pub w14_text_fill:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect>,
  /// Defines the Scene3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Scene3D/w14:scene3d"))]
  pub w14_scene3d: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D>,
  /// Defines the Properties3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Props3D/w14:props3d"))]
  pub w14_props3d: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D>,
  /// Defines the Ligatures Class.
  #[sdk(child(office2010, qname = "w14:CT_Ligatures/w14:ligatures"))]
  pub w14_ligatures: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2010, qname = "w14:CT_NumForm/w14:numForm"))]
  pub w14_num_form:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(office2010, qname = "w14:CT_NumSpacing/w14:numSpacing"))]
  pub w14_num_spacing:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(office2010, qname = "w14:CT_StylisticSets/w14:stylisticSets"))]
  pub w14_stylistic_sets:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:cntxtAlts"))]
  pub w14_cntxt_alts:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives>,
}
/// Previous Run Properties for the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ParaRPrOriginal/w:rPr")]
pub struct PreviousParagraphMarkRunProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Inserted Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:ins"))]
  pub inserted: Option<Inserted>,
  /// Deleted Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:del"))]
  pub deleted: Option<Deleted>,
  /// Move Source Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:moveFrom"))]
  pub move_from: Option<MoveFrom>,
  /// Move Destination Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:moveTo"))]
  pub move_to: Option<MoveTo>,
  #[sdk(choice(
    microsoft365,
    qname = "w:CT_TrackChange/w14:conflictIns",
    qname = "w:CT_TrackChange/w14:conflictDel"
  ))]
  pub previous_paragraph_mark_run_properties_choice:
    Option<PreviousParagraphMarkRunPropertiesChoice>,
  /// Defines the RunStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:rStyle"))]
  pub w_r_style: Vec<RunStyle>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub w_r_fonts: Vec<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub w_b: Vec<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub w_b_cs: Vec<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub w_i: Vec<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub w_i_cs: Vec<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub w_caps: Vec<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub w_small_caps: Vec<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub w_strike: Vec<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub w_dstrike: Vec<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub w_outline: Vec<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub w_shadow: Vec<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub w_emboss: Vec<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub w_imprint: Vec<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub w_no_proof: Vec<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub w_snap_to_grid: Vec<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub w_vanish: Vec<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub w_web_hidden: Vec<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub w_color: Vec<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub w_spacing: Vec<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub w_w: Vec<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub w_kern: Vec<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub w_position: Vec<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub w_sz: Vec<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub w_sz_cs: Vec<FontSizeComplexScript>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "w:CT_Highlight/w:highlight"))]
  pub w_highlight: Vec<Highlight>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub w_u: Vec<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub w_effect: Vec<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub w_bdr: Vec<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub w_shd: Vec<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub w_fit_text: Vec<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub w_vert_align: Vec<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtl"))]
  pub w_rtl: Vec<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:cs"))]
  pub w_cs: Vec<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub w_em: Vec<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub w_lang: Vec<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub w_east_asian_layout: Vec<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub w_spec_vanish: Vec<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(office2010, qname = "w14:CT_Glow/w14:glow"))]
  pub w14_glow: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow>,
  /// Defines the Shadow Class.
  #[sdk(child(office2010, qname = "w14:CT_Shadow/w14:shadow"))]
  pub w14_shadow: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow>,
  /// Defines the Reflection Class.
  #[sdk(child(office2010, qname = "w14:CT_Reflection/w14:reflection"))]
  pub w14_reflection:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_TextOutlineEffect/w14:textOutline"))]
  pub w14_text_outline:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect>,
  /// Defines the FillTextEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_FillTextEffect/w14:textFill"))]
  pub w14_text_fill:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect>,
  /// Defines the Scene3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Scene3D/w14:scene3d"))]
  pub w14_scene3d: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D>,
  /// Defines the Properties3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Props3D/w14:props3d"))]
  pub w14_props3d: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D>,
  /// Defines the Ligatures Class.
  #[sdk(child(office2010, qname = "w14:CT_Ligatures/w14:ligatures"))]
  pub w14_ligatures: Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2010, qname = "w14:CT_NumForm/w14:numForm"))]
  pub w14_num_form:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(office2010, qname = "w14:CT_NumSpacing/w14:numSpacing"))]
  pub w14_num_spacing:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(office2010, qname = "w14:CT_StylisticSets/w14:stylisticSets"))]
  pub w14_stylistic_sets:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:cntxtAlts"))]
  pub w14_cntxt_alts:
    Vec<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives>,
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "w:CT_OnOff/w:oMath"))]
  pub w_o_math: Vec<OfficeMath>,
}
/// Numbering Level Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeDecimalNumber255/w:ilvl")]
pub struct NumberingLevelReference {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 255))]
  pub val: crate::simple_type::Int32Value,
}
/// Numbering Definition Instance Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeDecimalNumber/w:numId")]
pub struct NumberingId {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Starting Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeDecimalNumber/w:start")]
pub struct StartNumberingValue {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the AbstractNumId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeDecimalNumber/w:abstractNumId")]
pub struct AbstractNumId {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Previous Paragraph Numbering Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrackChangeNumbering/w:numberingChange")]
pub struct NumberingChange {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
#[sdk(qname = "w:CT_TabStop/w:tab")]
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
  pub position: crate::simple_type::Int32Value,
}
/// Run Properties for the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ParaRPr/w:rPr")]
pub struct ParagraphMarkRunProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Inserted Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:ins"))]
  pub inserted: Option<Inserted>,
  /// Deleted Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:del"))]
  pub deleted: Option<Deleted>,
  /// Move Source Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:moveFrom"))]
  pub move_from: Option<MoveFrom>,
  /// Move Destination Paragraph
  #[sdk(child(qname = "w:CT_TrackChange/w:moveTo"))]
  pub move_to: Option<MoveTo>,
  #[sdk(choice(
    microsoft365,
    qname = "w:CT_TrackChange/w14:conflictIns",
    qname = "w:CT_TrackChange/w14:conflictDel"
  ))]
  pub paragraph_mark_run_properties_choice: Option<ParagraphMarkRunPropertiesChoice>,
  /// Defines the RunStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:rStyle"))]
  pub w_r_style: Option<RunStyle>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub w_r_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub w_b: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub w_b_cs: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub w_i: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub w_i_cs: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub w_caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub w_small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub w_strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub w_dstrike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub w_outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub w_shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub w_emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub w_imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub w_no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub w_snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub w_vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub w_web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub w_color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub w_spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub w_w: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub w_kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub w_position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub w_sz: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub w_sz_cs: Option<FontSizeComplexScript>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "w:CT_Highlight/w:highlight"))]
  pub w_highlight: Option<Highlight>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub w_u: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub w_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub w_bdr: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub w_shd: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub w_fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub w_vert_align: Option<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtl"))]
  pub w_rtl: Option<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:cs"))]
  pub w_cs: Option<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub w_em: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub w_lang: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub w_east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub w_spec_vanish: Option<SpecVanish>,
  /// Defines the Glow Class.
  #[sdk(child(office2010, qname = "w14:CT_Glow/w14:glow"))]
  pub w14_glow:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow>>,
  /// Defines the Shadow Class.
  #[sdk(child(office2010, qname = "w14:CT_Shadow/w14:shadow"))]
  pub w14_shadow:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow>>,
  /// Defines the Reflection Class.
  #[sdk(child(office2010, qname = "w14:CT_Reflection/w14:reflection"))]
  pub w14_reflection:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection>,
  /// Defines the TextOutlineEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_TextOutlineEffect/w14:textOutline"))]
  pub w14_text_outline: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    >,
  >,
  /// Defines the FillTextEffect Class.
  #[sdk(child(office2010, qname = "w14:CT_FillTextEffect/w14:textFill"))]
  pub w14_text_fill: Option<
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect>,
  >,
  /// Defines the Scene3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Scene3D/w14:scene3d"))]
  pub w14_scene3d:
    Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D>>,
  /// Defines the Properties3D Class.
  #[sdk(child(office2010, qname = "w14:CT_Props3D/w14:props3d"))]
  pub w14_props3d: Option<
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D>,
  >,
  /// Defines the Ligatures Class.
  #[sdk(child(office2010, qname = "w14:CT_Ligatures/w14:ligatures"))]
  pub w14_ligatures:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures>,
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2010, qname = "w14:CT_NumForm/w14:numForm"))]
  pub w14_num_form:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat>,
  /// Defines the NumberSpacing Class.
  #[sdk(child(office2010, qname = "w14:CT_NumSpacing/w14:numSpacing"))]
  pub w14_num_spacing:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing>,
  /// Defines the StylisticSets Class.
  #[sdk(child(office2010, qname = "w14:CT_StylisticSets/w14:stylisticSets"))]
  pub w14_stylistic_sets:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets>,
  /// Defines the ContextualAlternatives Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:cntxtAlts"))]
  pub w14_cntxt_alts:
    Option<crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives>,
  /// Defines the OfficeMath Class.
  #[sdk(child(qname = "w:CT_OnOff/w:oMath"))]
  pub w_o_math: Option<OfficeMath>,
  /// Revision Information for Run Properties on the Paragraph Mark.
  #[sdk(child(qname = "w:CT_ParaRPrChange/w:rPrChange"))]
  pub w_r_pr_change: Option<std::boxed::Box<ParagraphMarkRunPropertiesChange>>,
}
/// Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SectPr/w:sectPr")]
pub struct SectionProperties {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(choice(
    qname = "w:CT_HdrFtrRef/w:headerReference",
    qname = "w:CT_HdrFtrRef/w:footerReference",
    text,
    any
  ))]
  pub section_properties_choice: Vec<SectionPropertiesChoice>,
  /// Defines the FootnoteProperties Class.
  #[sdk(child(qname = "w:CT_FtnProps/w:footnotePr"))]
  pub w_footnote_pr: Option<std::boxed::Box<FootnoteProperties>>,
  /// Defines the EndnoteProperties Class.
  #[sdk(child(qname = "w:CT_EdnProps/w:endnotePr"))]
  pub w_endnote_pr: Option<std::boxed::Box<EndnoteProperties>>,
  /// Defines the SectionType Class.
  #[sdk(child(qname = "w:CT_SectType/w:type"))]
  pub w_type: Option<SectionType>,
  /// Defines the PageSize Class.
  #[sdk(child(qname = "w:CT_PageSz/w:pgSz"))]
  pub w_pg_sz: Option<PageSize>,
  /// Defines the PageMargin Class.
  #[sdk(child(qname = "w:CT_PageMar/w:pgMar"))]
  pub w_pg_mar: Option<PageMargin>,
  /// Defines the PaperSource Class.
  #[sdk(child(qname = "w:CT_PaperSource/w:paperSrc"))]
  pub w_paper_src: Option<PaperSource>,
  /// Defines the PageBorders Class.
  #[sdk(child(qname = "w:CT_PageBorders/w:pgBorders"))]
  pub w_pg_borders: Option<std::boxed::Box<PageBorders>>,
  /// Defines the LineNumberType Class.
  #[sdk(child(qname = "w:CT_LineNumber/w:lnNumType"))]
  pub w_ln_num_type: Option<LineNumberType>,
  /// Defines the PageNumberType Class.
  #[sdk(child(qname = "w:CT_PageNumber/w:pgNumType"))]
  pub w_pg_num_type: Option<PageNumberType>,
  /// Defines the Columns Class.
  #[sdk(child(qname = "w:CT_Columns/w:cols"))]
  pub w_cols: Option<Columns>,
  /// Defines the FormProtection Class.
  #[sdk(child(qname = "w:CT_OnOff/w:formProt"))]
  pub w_form_prot: Option<FormProtection>,
  /// Defines the VerticalTextAlignmentOnPage Class.
  #[sdk(child(qname = "w:CT_VerticalJc/w:vAlign"))]
  pub w_v_align: Option<VerticalTextAlignmentOnPage>,
  /// Defines the NoEndnote Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noEndnote"))]
  pub w_no_endnote: Option<NoEndnote>,
  /// Defines the TitlePage Class.
  #[sdk(child(qname = "w:CT_OnOff/w:titlePg"))]
  pub w_title_pg: Option<TitlePage>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub w_text_direction: Option<TextDirection>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub w_bidi: Option<BiDi>,
  /// Defines the GutterOnRight Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtlGutter"))]
  pub w_rtl_gutter: Option<GutterOnRight>,
  /// Defines the DocGrid Class.
  #[sdk(child(qname = "w:CT_DocGrid/w:docGrid"))]
  pub w_doc_grid: Option<DocGrid>,
  /// Defines the PrinterSettingsReference Class.
  #[sdk(child(qname = "w:CT_Rel/w:printerSettings"))]
  pub w_printer_settings: Option<PrinterSettingsReference>,
  /// Defines the FootnoteColumns Class.
  #[sdk(child(office2013, qname = "w:CT_DecimalNumber/w15:footnoteColumns"))]
  pub w15_footnote_columns:
    Option<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::FootnoteColumns>,
  /// Revision Information for Section Properties.
  #[sdk(child(qname = "w:CT_SectPrChange/w:sectPrChange"))]
  pub w_sect_pr_change: Option<std::boxed::Box<SectionPropertiesChange>>,
}
/// Custom Field Data.
pub type FieldData = crate::simple_type::Base64BinaryValue;
/// Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFData/w:ffData")]
pub struct FormFieldData {
  #[sdk(choice(
    qname = "w:CT_FFName/w:name",
    qname = "w:CT_OnOff/w:enabled",
    qname = "w:CT_OnOff/w:calcOnExit",
    qname = "w:CT_MacroName/w:entryMacro",
    qname = "w:CT_MacroName/w:exitMacro",
    qname = "w:CT_FFHelpText/w:helpText",
    qname = "w:CT_FFStatusText/w:statusText",
    qname = "w:CT_FFCheckBox/w:checkBox",
    qname = "w:CT_FFDDList/w:ddList",
    qname = "w:CT_FFTextInput/w:textInput"
  ))]
  pub form_field_data_choice: Vec<FormFieldDataChoice>,
}
/// Form Field Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFName/w:name")]
pub struct FormFieldName {
  /// Form Field Name Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 20u32))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Script Function to Execute on Form Field Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MacroName/w:entryMacro")]
pub struct EntryMacro {
  /// Name of Script Function
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 33u32))]
  pub val: crate::simple_type::StringValue,
}
/// Script Function to Execute on Form Field Exit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MacroName/w:exitMacro")]
pub struct ExitMacro {
  /// Name of Script Function
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 33u32))]
  pub val: crate::simple_type::StringValue,
}
/// Associated Help Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFHelpText/w:helpText")]
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
#[sdk(qname = "w:CT_FFStatusText/w:statusText")]
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
#[sdk(qname = "w:CT_FFCheckBox/w:checkBox")]
pub struct CheckBox {
  #[sdk(choice(qname = "w:CT_HpsMeasure/w:size", qname = "w:CT_OnOff/w:sizeAuto"))]
  pub check_box_choice: Option<CheckBoxChoice>,
  /// Default Checkbox Form Field State.
  #[sdk(child(qname = "w:CT_OnOff/w:default"))]
  pub w_default: Option<DefaultCheckBoxFormFieldState>,
  /// Checkbox Form Field State.
  #[sdk(child(qname = "w:CT_OnOff/w:checked"))]
  pub w_checked: Option<Checked>,
}
/// Drop-Down List Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFDDList/w:ddList")]
pub struct DropDownListFormField {
  /// Drop-Down List Selection
  #[sdk(child(qname = "w:CT_DecimalNumber/w:result"))]
  pub drop_down_list_selection: Option<DropDownListSelection>,
  /// Default Drop-Down List Item Index
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumberMax24/w:default"))]
  pub default_drop_down_list_item_index: Option<DefaultDropDownListItemIndex>,
  /// Drop-Down List Entry.
  #[sdk(child(qname = "w:CT_String255/w:listEntry"))]
  pub w_list_entry: Vec<ListEntryFormField>,
}
/// Text Box Form Field Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFTextInput/w:textInput")]
pub struct TextInput {
  /// Text Box Form Field Type
  #[sdk(child(qname = "w:CT_FFTextType/w:type"))]
  pub text_box_form_field_type: Option<TextBoxFormFieldType>,
  /// Default Text Box Form Field String
  #[sdk(child(qname = "w:CT_String255/w:default"))]
  pub default_text_box_form_field_string: Option<DefaultTextBoxFormFieldString>,
  /// Text Box Form Field Maximum Length
  #[sdk(child(qname = "w:CT_PositiveShort/w:maxLength"))]
  pub max_length: Option<MaxLength>,
  /// Text Box Form Field Formatting
  #[sdk(child(qname = "w:CT_String64/w:format"))]
  pub format: Option<Format>,
}
/// Default Drop-Down List Item Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedDecimalNumberMax24/w:default")]
pub struct DefaultDropDownListItemIndex {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 24))]
  pub val: crate::simple_type::Int32Value,
}
/// Drop-Down List Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String255/w:listEntry")]
pub struct ListEntryFormField {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Default Text Box Form Field String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String255/w:default")]
pub struct DefaultTextBoxFormFieldString {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Frame Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String255/w:name")]
pub struct FrameName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 255u32))]
  pub val: crate::simple_type::StringValue,
}
/// Text Box Form Field Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FFTextType/w:type")]
pub struct TextBoxFormFieldType {
  /// Text Box Form Field Type Values
  #[sdk(attr(qname = "w:val"))]
  pub val: TextBoxFormFieldValues,
}
/// Text Box Form Field Maximum Length.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PositiveShort/w:maxLength")]
pub struct MaxLength {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 1..))]
  pub val: crate::simple_type::Int16Value,
}
/// Text Box Form Field Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_String64/w:format")]
pub struct Format {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 64u32))]
  pub val: crate::simple_type::StringValue,
}
/// Single Column Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Column/w:col")]
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
  pub width: Option<crate::simple_type::StringValue>,
  /// Space Before Following Column
  #[sdk(attr(qname = "w:space"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_TwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "w:ST_UnsignedDecimalNumber"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub space: Option<crate::simple_type::StringValue>,
}
/// Revision Information for Section Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SectPrChange/w:sectPrChange")]
pub struct SectionPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_SectPrBase/w:sectPr"))]
  pub previous_section_properties: Option<std::boxed::Box<PreviousSectionProperties>>,
}
/// Revision Information for Run Properties on the Paragraph Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ParaRPrChange/w:rPrChange")]
pub struct ParagraphMarkRunPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_ParaRPrOriginal/w:rPr"))]
  pub previous_paragraph_mark_run_properties: std::boxed::Box<PreviousParagraphMarkRunProperties>,
}
/// External Content Import Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_AltChunkPr/w:altChunkPr")]
pub struct AltChunkProperties {
  /// Keep Source Formatting on Import
  #[sdk(child(qname = "w:CT_OnOff/w:matchSrc"))]
  pub match_source: Option<MatchSource>,
}
/// Phonetic Guide Text Alignment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RubyAlign/w:rubyAlign")]
pub struct RubyAlign {
  /// Phonetic Guide Text Alignment Value
  #[sdk(attr(qname = "w:val"))]
  pub val: RubyAlignValues,
}
/// Distance Between Phonetic Guide Text and Phonetic Guide Base Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_HpsRaise/w:hpsRaise")]
pub struct PhoneticGuideRaise {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Int16Value,
}
/// Language ID for Phonetic Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Lang/w:lid")]
pub struct LanguageId {
  /// Language Code
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(max = 84u32))]
  pub val: crate::simple_type::StringValue,
}
/// Phonetic Guide Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RubyPr/w:rubyPr")]
pub struct RubyProperties {
  /// Phonetic Guide Text Alignment
  #[sdk(child(qname = "w:CT_RubyAlign/w:rubyAlign"))]
  pub ruby_align: std::boxed::Box<RubyAlign>,
  /// Phonetic Guide Text Font Size
  #[sdk(child(qname = "w:CT_HpsMeasure/w:hps"))]
  pub phonetic_guide_text_font_size: std::boxed::Box<PhoneticGuideTextFontSize>,
  /// Distance Between Phonetic Guide Text and Phonetic Guide Base Text
  #[sdk(child(qname = "w:CT_HpsRaise/w:hpsRaise"))]
  pub phonetic_guide_raise: std::boxed::Box<PhoneticGuideRaise>,
  /// Phonetic Guide Base Text Font Size
  #[sdk(child(qname = "w:CT_HpsMeasure/w:hpsBaseText"))]
  pub phonetic_guide_base_text_size: std::boxed::Box<PhoneticGuideBaseTextSize>,
  /// Language ID for Phonetic Guide
  #[sdk(child(qname = "w:CT_Lang/w:lid"))]
  pub language_id: std::boxed::Box<LanguageId>,
  /// Invalidated Field Cache
  #[sdk(child(qname = "w:CT_OnOff/w:dirty"))]
  pub dirty: Option<Dirty>,
}
/// Phonetic Guide Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RubyContent/w:rt")]
pub struct RubyContent {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub ruby_content_choice: Vec<RubyContentChoice>,
}
/// Phonetic Guide Base Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RubyContent/w:rubyBase")]
pub struct RubyBase {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub ruby_base_choice: Vec<RubyBaseChoice>,
}
/// Custom XML Data Date Storage Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtDateMappingType/w:storeMappedDataAs")]
pub struct SdtDateMappingType {
  /// Date Storage Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<DateFormatValues>,
}
/// Date Picker Calendar Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CalendarType/w:calendar")]
pub struct Calendar {
  /// Calendar Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<CalendarValues>,
}
/// Combo Box List Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtListItem/w:listItem")]
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
#[sdk(qname = "w:CT_SdtPr/w:sdtPr")]
pub struct SdtProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_RPr/w:rPr",
    qname = "w:CT_String/w:alias",
    qname = "w:CT_Lock/w:lock",
    qname = "w:CT_Placeholder/w:placeholder",
    qname = "w:CT_OnOff/w:showingPlcHdr",
    qname = "w:CT_DataBinding/w:dataBinding",
    qname = "w:CT_DataBinding/w15:dataBinding",
    qname = "w:CT_OnOff/w:temporary",
    qname = "w:CT_DecimalNumber/w:id",
    qname = "w:CT_String/w:tag",
    qname = "w:CT_Color/w15:color",
    qname = "w15:CT_SdtAppearance/w15:appearance",
    qname = "w:CT_OnOff/w15:webExtensionLinked",
    qname = "w:CT_OnOff/w15:webExtensionCreated",
    qname = "w:CT_Empty/w:equation",
    qname = "w:CT_SdtComboBox/w:comboBox",
    qname = "w:CT_SdtDate/w:date",
    qname = "w:CT_SdtDocPart/w:docPartObj",
    qname = "w:CT_SdtDocPart/w:docPartList",
    qname = "w:CT_SdtDropDownList/w:dropDownList",
    qname = "w:CT_Empty/w:picture",
    qname = "w:CT_Empty/w:richText",
    qname = "w:CT_SdtText/w:text",
    qname = "w:CT_Empty/w:citation",
    qname = "w:CT_Empty/w:group",
    qname = "w:CT_Empty/w:bibliography",
    qname = "w:CT_Empty/w14:entityPicker",
    qname = "w14:CT_SdtCheckbox/w14:checkbox",
    qname = "w15:CT_SdtRepeatedSection/w15:repeatingSection",
    qname = "w:CT_Empty/w15:repeatingSectionItem",
    text,
    any
  ))]
  pub sdt_properties_choice: Vec<SdtPropertiesChoice>,
}
/// Structured Document Tag End Character Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtEndPr/w:sdtEndPr")]
pub struct SdtEndCharProperties {
  /// Run Properties.
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  pub run_properties: Vec<RunProperties>,
}
/// Block-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtContentBlock/w:sdtContent")]
pub struct SdtContentBlock {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub sdt_content_block_choice: Vec<SdtContentBlockChoice>,
}
/// Inline-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtContentRun/w:sdtContent")]
pub struct SdtContentRun {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
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
    qname = "m:CT_OMath/m:oMath",
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
    qname = "w:CT_R/w:r",
    qname = "w:CT_BdoContentRun/w:bdo",
    qname = "w:CT_DirContentRun/w:dir",
    qname = "w:CT_Rel/w:subDoc",
    text,
    any
  ))]
  pub sdt_content_run_choice: Vec<SdtContentRunChoice>,
}
/// Defines the SdtContentRunRuby Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtContentRunRuby/w:sdtContent")]
pub struct SdtContentRunRuby {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_CustomXmlRuby/w:customXml",
    qname = "w:CT_SimpleFieldRuby/w:fldSimple",
    qname = "w:CT_HyperlinkRuby/w:hyperlink",
    qname = "w:CT_R/w:r",
    qname = "w:CT_SdtRunRuby/w:sdt",
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
    qname = "m:CT_OMath/m:oMath",
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
    text,
    any
  ))]
  pub sdt_content_run_ruby_choice: Vec<SdtContentRunRubyChoice>,
}
/// Cell-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtContentCell/w:sdtContent")]
pub struct SdtContentCell {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_Tc/w:tc",
    qname = "w:CT_CustomXmlCell/w:customXml",
    qname = "w:CT_SdtCell/w:sdt",
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
    text,
    any
  ))]
  pub sdt_content_cell_choice: Vec<SdtContentCellChoice>,
}
/// Row-Level Structured Document Tag Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtContentRow/w:sdtContent")]
pub struct SdtContentRow {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_Row/w:tr",
    qname = "w:CT_CustomXmlRow/w:customXml",
    qname = "w:CT_SdtRow/w:sdt",
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
    text,
    any
  ))]
  pub sdt_content_row_choice: Vec<SdtContentRowChoice>,
}
/// Custom XML Element Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CustomXmlPr/w:customXmlPr")]
pub struct CustomXmlProperties {
  /// Custom XML Element Placeholder Text
  #[sdk(child(qname = "w:CT_String/w:placeholder"))]
  pub custom_xml_placeholder: Option<CustomXmlPlaceholder>,
  /// Custom XML Attribute.
  #[sdk(child(qname = "w:CT_XmlAttr/w:attr"))]
  pub w_attr: Vec<CustomXmlAttribute>,
}
/// Custom XML Attribute.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_XmlAttr/w:attr")]
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
#[sdk(qname = "w:CT_TblGridCol/w:gridCol")]
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
  pub width: Option<crate::simple_type::StringValue>,
}
/// Revision Information for Table Grid Column Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblGridChange/w:tblGridChange")]
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
  #[sdk(child(qname = "w:CT_TblGridBase/w:tblGrid"))]
  pub previous_table_grid: std::boxed::Box<PreviousTableGrid>,
}
/// Revision Information for Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcPrChange/w:tcPrChange")]
pub struct TableCellPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_TcPrInner/w:tcPr"))]
  pub previous_table_cell_properties: std::boxed::Box<PreviousTableCellProperties>,
}
/// Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcPr/w:tcPr")]
pub struct TableCellProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Defines the TableCellWidth Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tcW"))]
  pub table_cell_width: Option<TableCellWidth>,
  /// Defines the GridSpan Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridSpan"))]
  pub grid_span: Option<GridSpan>,
  /// Defines the HorizontalMerge Class.
  #[sdk(child(qname = "w:CT_HMerge/w:hMerge"))]
  pub horizontal_merge: Option<HorizontalMerge>,
  /// Defines the VerticalMerge Class.
  #[sdk(child(qname = "w:CT_VMerge/w:vMerge"))]
  pub vertical_merge: Option<VerticalMerge>,
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:CT_TcBorders/w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:CT_TcMar/w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TableCellFitText Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:tcFitText"))]
  pub table_cell_fit_text: Option<TableCellFitText>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalTblJc/w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
  /// Defines the HideMark Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:hideMark"))]
  pub hide_mark: Option<HideMark>,
  #[sdk(choice(
    qname = "w:CT_TrackChange/w:cellIns",
    qname = "w:CT_TrackChange/w:cellDel",
    qname = "w:CT_CellMergeTrackChange/w:cellMerge"
  ))]
  pub table_cell_properties_choice: Option<TableCellPropertiesChoice>,
  /// Revision Information for Table Cell Properties.
  #[sdk(child(qname = "w:CT_TcPrChange/w:tcPrChange"))]
  pub w_tc_pr_change: Option<std::boxed::Box<TableCellPropertiesChange>>,
}
/// Revision Information for Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrChange/w:tblPrChange")]
pub struct TablePropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_TblPrBase/w:tblPr"))]
  pub previous_table_properties: std::boxed::Box<PreviousTableProperties>,
}
/// Revision Information for Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrExChange/w:tblPrExChange")]
pub struct TablePropertyExceptionsChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_TblPrExBase/w:tblPrEx"))]
  pub previous_table_property_exceptions: std::boxed::Box<PreviousTablePropertyExceptions>,
}
/// Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPr/w:tblPr")]
pub struct TableProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the TableStyle Class.
  #[sdk(child(qname = "w:CT_String253/w:tblStyle"))]
  pub table_style: Option<TableStyle>,
  /// Defines the TablePositionProperties Class.
  #[sdk(child(qname = "w:CT_TblPPr/w:tblpPr"))]
  pub table_position_properties: Option<TablePositionProperties>,
  /// Defines the TableOverlap Class.
  #[sdk(child(qname = "w:CT_TblOverlap/w:tblOverlap"))]
  pub table_overlap: Option<TableOverlap>,
  /// Defines the BiDiVisual Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:bidiVisual"))]
  pub bi_di_visual: Option<BiDiVisual>,
  /// Defines the TableWidth Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableLayout Class.
  #[sdk(child(qname = "w:CT_TblLayoutType/w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Defines the TableLook Class.
  #[sdk(child(qname = "w:CT_TblLook/w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Defines the TableCaption Class.
  #[sdk(child(office2010, qname = "w:CT_String/w:tblCaption"))]
  pub table_caption: Option<TableCaption>,
  /// Defines the TableDescription Class.
  #[sdk(child(office2010, qname = "w:CT_String/w:tblDescription"))]
  pub table_description: Option<TableDescription>,
  /// Revision Information for Table Properties
  #[sdk(child(qname = "w:CT_TblPrChange/w:tblPrChange"))]
  pub table_properties_change: Option<std::boxed::Box<TablePropertiesChange>>,
}
/// Table Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblGrid/w:tblGrid")]
pub struct TableGrid {
  /// Grid Column Definition.
  #[sdk(child(qname = "w:CT_TblGridCol/w:gridCol"))]
  pub w_grid_col: Vec<GridColumn>,
  /// Revision Information for Table Grid Column Definitions.
  #[sdk(child(qname = "w:CT_TblGridChange/w:tblGridChange"))]
  pub w_tbl_grid_change: Option<std::boxed::Box<TableGridChange>>,
}
/// Footnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnPos/w:pos")]
pub struct FootnotePosition {
  /// Footnote Position Type
  #[sdk(attr(qname = "w:val"))]
  pub val: FootnotePositionValues,
}
/// Footnote Numbering Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NumFmt/w:numFmt")]
pub struct NumberingFormat {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Numbering Format Type
  #[sdk(attr(qname = "w:val"))]
  pub val: NumberFormatValues,
  /// format
  #[sdk(attr(office2010, qname = "w:format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Endnote Placement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_EdnPos/w:pos")]
pub struct EndnotePosition {
  /// Endnote Position Type
  #[sdk(attr(qname = "w:val"))]
  pub val: EndnotePositionValues,
}
/// Special Footnote List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdnSepRef/w:footnote")]
pub struct FootnoteSpecialReference {
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// Special Endnote List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdnSepRef/w:endnote")]
pub struct EndnoteSpecialReference {
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
}
/// Index of Column Containing Unique Values for Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedDecimalNumber/w:column")]
pub struct ColumnIndex {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Column Delimiter for Data Source.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedDecimalNumber/w:colDelim")]
pub struct ColumnDelimiter {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Unique Value for Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Base64Binary/w:uniqueTag")]
pub struct UniqueTag {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::Base64BinaryValue,
}
/// Data About Single Data Source Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RecipientData/w:recipientData")]
pub struct RecipientData {
  /// Record Is Included in Mail Merge
  #[sdk(child(qname = "w:CT_OnOff/w:active"))]
  pub active: Option<Active>,
  /// Index of Column Containing Unique Values for Record
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumber/w:column"))]
  pub column_index: std::boxed::Box<ColumnIndex>,
  /// Unique Value for Record
  #[sdk(child(qname = "w:CT_Base64Binary/w:uniqueTag"))]
  pub unique_tag: std::boxed::Box<UniqueTag>,
}
/// Merge Field Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMergeOdsoFMDFieldType/w:type")]
pub struct MailMergeFieldType {
  /// Merge Field Mapping Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeOdsoFieldValues,
}
/// ODSO Data Source Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMergeSourceType/w:type")]
pub struct MailMergeSource {
  /// Data Source Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeSourceValues,
}
/// External Data Source to Merge Field Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_OdsoFieldMapData/w:fieldMapData")]
pub struct FieldMapData {
  /// Merge Field Mapping
  #[sdk(child(qname = "w:CT_MailMergeOdsoFMDFieldType/w:type"))]
  pub mail_merge_field_type: Option<MailMergeFieldType>,
  /// Data Source Name for Column
  #[sdk(child(qname = "w:CT_String/w:name"))]
  pub name: Option<Name>,
  /// Predefined Merge Field Name
  #[sdk(child(qname = "w:CT_String/w:mappedName"))]
  pub mapped_name: Option<MappedName>,
  /// Index of Column Being Mapped
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumber/w:column"))]
  pub column_index: Option<ColumnIndex>,
  /// Merge Field Name Language ID
  #[sdk(child(qname = "w:CT_Lang/w:lid"))]
  pub language_id: Option<LanguageId>,
  /// Use Country/Region-Based Address Field Ordering
  #[sdk(child(qname = "w:CT_OnOff/w:dynamicAddress"))]
  pub dynamic_address: Option<DynamicAddress>,
}
/// Source Document Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMergeDocType/w:mainDocumentType")]
pub struct MainDocumentType {
  /// Mail Merge Source Document Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDocumentValues,
}
/// Data Source Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMergeDataType/w:dataType")]
pub struct DataType {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDataValues,
}
/// Merged Document Destination.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMergeDest/w:destination")]
pub struct Destination {
  /// Mail Merge Merged Document Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MailMergeDestinationValues,
}
/// Office Data Source Object Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Odso/w:odso")]
pub struct DataSourceObject {
  /// UDL Connection String
  #[sdk(child(qname = "w:CT_String/w:udl"))]
  pub udl_connection_string: Option<UdlConnectionString>,
  /// Data Source Table Name
  #[sdk(child(qname = "w:CT_String/w:table"))]
  pub data_source_table_name: Option<DataSourceTableName>,
  /// ODSO Data Source File Path
  #[sdk(child(qname = "w:CT_Rel/w:src"))]
  pub source_reference: Option<SourceReference>,
  /// Column Delimiter for Data Source
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumber/w:colDelim"))]
  pub column_delimiter: Option<ColumnDelimiter>,
  /// ODSO Data Source Type
  #[sdk(child(qname = "w:CT_MailMergeSourceType/w:type"))]
  pub mail_merge_source: Option<MailMergeSource>,
  /// First Row of Data Source Contains Column Names
  #[sdk(child(qname = "w:CT_OnOff/w:fHdr"))]
  pub first_row_header: Option<FirstRowHeader>,
  /// External Data Source to Merge Field Mapping.
  #[sdk(child(qname = "w:CT_OdsoFieldMapData/w:fieldMapData"))]
  pub w_field_map_data: Vec<FieldMapData>,
  /// Reference to Inclusion/Exclusion Data for Data Source.
  #[sdk(child(qname = "w:CT_Rel/w:recipientData"))]
  pub w_recipient_data: Option<RecipientDataReference>,
}
/// Single Document Variable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocVar/w:docVar")]
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
#[sdk(qname = "w:CT_LongHexNumber/w:rsidRoot")]
pub struct RsidRoot {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Single Session Revision Save ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LongHexNumber/w:rsid")]
pub struct Rsid {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Abstract Numbering Definition Identifier.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LongHexNumber/w:nsid")]
pub struct Nsid {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Numbering Template Code.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LongHexNumber/w:tmpl")]
pub struct TemplateCode {
  /// Long Hexadecimal Number Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPrBaseStyleable/w:rPr")]
pub struct RunPropertiesBaseStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
}
/// Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrBaseStyleable/w:pPr")]
pub struct ParagraphPropertiesBaseStyle {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:CT_OnOff/w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:CT_FramePr/w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:CT_OnOff/w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:CT_NumPr/w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:CT_PBdr/w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:CT_Tabs/w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:CT_OnOff/w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:CT_OnOff/w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:CT_Spacing/w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:CT_Ind/w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:CT_OnOff/w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:CT_Jc/w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:CT_TextAlignment/w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
}
/// Default Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPrDefault/w:rPrDefault")]
pub struct RunPropertiesDefault {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Run Properties
  #[sdk(child(qname = "w:CT_RPrBaseStyleable/w:rPr"))]
  pub run_properties_base_style: Option<std::boxed::Box<RunPropertiesBaseStyle>>,
}
/// Default Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrDefault/w:pPrDefault")]
pub struct ParagraphPropertiesDefault {
  /// Paragraph Properties
  #[sdk(child(qname = "w:CT_PPrBaseStyleable/w:pPr"))]
  pub paragraph_properties_base_style: Option<std::boxed::Box<ParagraphPropertiesBaseStyle>>,
}
/// Left and Right Margin for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PixelsMeasure/w:marW")]
pub struct MarginWidth {
  /// Measurement in Pixels
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Top and Bottom Margin for Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PixelsMeasure/w:marH")]
pub struct MarginHeight {
  /// Measurement in Pixels
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Scrollbar Display Option.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FrameScrollbar/w:scrollbar")]
pub struct ScrollbarVisibility {
  /// Scrollbar Display Option Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FrameScrollbarVisibilityValues,
}
/// Frameset Splitter Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:w")]
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
  pub val: crate::simple_type::StringValue,
}
/// Hyphenation Zone.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:hyphenationZone")]
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
  pub val: crate::simple_type::StringValue,
}
/// Drawing Grid Horizontal Grid Unit Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:drawingGridHorizontalSpacing")]
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
  pub val: crate::simple_type::StringValue,
}
/// Drawing Grid Vertical Grid Unit Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:drawingGridVerticalSpacing")]
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
  pub val: crate::simple_type::StringValue,
}
/// Drawing Grid Horizontal Origin Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:drawingGridHorizontalOrigin")]
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
  pub val: crate::simple_type::StringValue,
}
/// Drawing Grid Vertical Origin Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TwipsMeasure/w:drawingGridVerticalOrigin")]
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
  pub val: crate::simple_type::StringValue,
}
/// Frameset Splitter Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FramesetSplitbar/w:framesetSplitbar")]
pub struct FramesetSplitbar {
  /// Frameset Splitter Width
  #[sdk(child(qname = "w:CT_TwipsMeasure/w:w"))]
  pub width: Option<Width>,
  /// Frameset Splitter Color
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub color: Option<Color>,
  /// Do Not Display Frameset Splitters
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noBorder"))]
  pub no_border: Option<NoBorder>,
  /// Frameset Splitter Border Style
  #[sdk(child(qname = "w:CT_OnOffOnly/w:flatBorders"))]
  pub flat_borders: Option<FlatBorders>,
}
/// Frameset Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FrameLayout/w:frameLayout")]
pub struct FrameLayout {
  /// Frameset Layout Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FrameLayoutValues,
}
/// Nested Frameset Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Frameset/w:frameset")]
pub struct Frameset {
  /// Nested Frameset Size
  #[sdk(child(qname = "w:CT_String/w:sz"))]
  pub frame_size: Option<FrameSize>,
  /// Frameset Splitter Properties
  #[sdk(child(qname = "w:CT_FramesetSplitbar/w:framesetSplitbar"))]
  pub frameset_splitbar: Option<std::boxed::Box<FramesetSplitbar>>,
  /// Frameset Layout
  #[sdk(child(qname = "w:CT_FrameLayout/w:frameLayout"))]
  pub frame_layout: Option<FrameLayout>,
  #[sdk(choice(qname = "w:CT_Frameset/w:frameset", qname = "w:CT_Frame/w:frame"))]
  pub frameset_choice: Vec<FramesetChoice>,
}
/// Single Frame Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Frame/w:frame")]
pub struct Frame {
  /// Frame Size
  #[sdk(child(qname = "w:CT_String/w:sz"))]
  pub frame_size: Option<FrameSize>,
  /// Frame Name
  #[sdk(child(qname = "w:CT_String255/w:name"))]
  pub frame_name: Option<FrameName>,
  /// Source File for Frame
  #[sdk(child(qname = "w:CT_Rel/w:sourceFileName"))]
  pub source_file_reference: Option<SourceFileReference>,
  /// Left and Right Margin for Frame
  #[sdk(child(qname = "w:CT_PixelsMeasure/w:marW"))]
  pub margin_width: Option<MarginWidth>,
  /// Top and Bottom Margin for Frame
  #[sdk(child(qname = "w:CT_PixelsMeasure/w:marH"))]
  pub margin_height: Option<MarginHeight>,
  /// Scrollbar Display Option
  #[sdk(child(qname = "w:CT_FrameScrollbar/w:scrollbar"))]
  pub scrollbar_visibility: Option<ScrollbarVisibility>,
  /// Frame Cannot Be Resized
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noResizeAllowed"))]
  pub no_resize_allowed: Option<NoResizeAllowed>,
  /// Maintain Link to Existing File
  #[sdk(child(qname = "w:CT_OnOffOnly/w:linkedToFile"))]
  pub linked_to_file: Option<LinkedToFile>,
}
/// Content Between Numbering Symbol and Paragraph Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LevelSuffix/w:suff")]
pub struct LevelSuffix {
  /// Character Type Between Numbering and Text
  #[sdk(attr(qname = "w:val"))]
  pub val: LevelSuffixValues,
}
/// Numbering Level Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LevelText/w:lvlText")]
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
#[sdk(qname = "w:CT_LvlLegacy/w:legacy")]
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
  pub legacy_space: Option<crate::simple_type::StringValue>,
  /// Legacy Indent
  #[sdk(attr(qname = "w:legacyIndent"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "w:ST_SignedTwipsMeasure_O12"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:integer"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "-?[0-9]+(\\.[0-9]+)?(mm|cm|in|pt|pc|pi)"
  ))]
  pub legacy_indent: Option<crate::simple_type::StringValue>,
}
/// Justification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LevelJustification/w:lvlJc")]
pub struct LevelJustification {
  /// Alignment Type
  #[sdk(attr(qname = "w:val"))]
  pub w_val: LevelJustificationValues,
}
/// Numbering Level Associated Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrBase/w:pPr")]
pub struct PreviousParagraphProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:CT_String/w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:CT_OnOff/w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:CT_FramePr/w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:CT_OnOff/w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:CT_NumPr/w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:CT_PBdr/w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:CT_Tabs/w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:CT_OnOff/w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:CT_OnOff/w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:CT_Spacing/w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:CT_Ind/w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:CT_OnOff/w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:CT_Jc/w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:CT_TextAlignment/w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
}
/// Numbering Symbol Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPrList/w:rPr")]
pub struct NumberingSymbolRunProperties {
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the RightToLeftText Class.
  #[sdk(child(qname = "w:CT_OnOff/w:rtl"))]
  pub right_to_left_text: Option<RightToLeftText>,
  /// Defines the ComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:cs"))]
  pub complex_script: Option<ComplexScript>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
}
/// Abstract Numbering Definition Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MultiLevelType/w:multiLevelType")]
pub struct MultiLevelType {
  /// Abstract Numbering Definition Type
  #[sdk(attr(qname = "w:val"))]
  pub val: MultiLevelValues,
}
/// Numbering Level Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Lvl/w:lvl")]
pub struct Level {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
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
  #[sdk(child(qname = "w:CT_NonNegativeDecimalNumber/w:start"))]
  pub start_numbering_value: Option<StartNumberingValue>,
  /// Numbering Format
  #[sdk(child(qname = "w:CT_NumFmt/w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Restart Numbering Level Symbol
  #[sdk(child(qname = "w:CT_DecimalNumber/w:lvlRestart"))]
  pub level_restart: Option<LevelRestart>,
  /// Paragraph Style's Associated Numbering Level
  #[sdk(child(qname = "w:CT_String253/w:pStyle"))]
  pub paragraph_style_id_in_level: Option<ParagraphStyleIdInLevel>,
  /// Display All Levels Using Arabic Numerals
  #[sdk(child(qname = "w:CT_OnOff/w:isLgl"))]
  pub is_legal_numbering_style: Option<IsLegalNumberingStyle>,
  /// Content Between Numbering Symbol and Paragraph Text
  #[sdk(child(qname = "w:CT_LevelSuffix/w:suff"))]
  pub level_suffix: Option<LevelSuffix>,
  /// Numbering Level Text
  #[sdk(child(qname = "w:CT_LevelText/w:lvlText"))]
  pub level_text: Option<LevelText>,
  /// Picture Numbering Symbol Definition Reference
  #[sdk(child(qname = "w:CT_DecimalNumber/w:lvlPicBulletId"))]
  pub level_picture_bullet_id: Option<LevelPictureBulletId>,
  /// Legacy Numbering Level Properties
  #[sdk(child(qname = "w:CT_LvlLegacy/w:legacy"))]
  pub legacy_numbering: Option<LegacyNumbering>,
  /// Justification
  #[sdk(child(qname = "w:CT_LevelJustification/w:lvlJc"))]
  pub level_justification: Option<LevelJustification>,
  /// Numbering Level Associated Paragraph Properties
  #[sdk(child(qname = "w:CT_PPrBase/w:pPr"))]
  pub previous_paragraph_properties: Option<std::boxed::Box<PreviousParagraphProperties>>,
  /// Numbering Symbol Run Properties
  #[sdk(child(qname = "w:CT_RPrList/w:rPr"))]
  pub numbering_symbol_run_properties: Option<std::boxed::Box<NumberingSymbolRunProperties>>,
}
/// Picture Numbering Symbol Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NumPicBullet/w:numPicBullet")]
pub struct NumberingPictureBullet {
  pub xml_other_attrs: Vec<(String, String)>,
  /// numPicBulletId
  #[sdk(attr(qname = "w:numPicBulletId"))]
  pub numbering_picture_bullet_id: crate::simple_type::Int32Value,
  #[sdk(choice(
    qname = "w:CT_PictureBulletBase/w:pict",
    qname = "w:CT_Drawing/w:drawing"
  ))]
  pub numbering_picture_bullet_choice: Option<NumberingPictureBulletChoice>,
}
/// Abstract Numbering Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_AbstractNum/w:abstractNum")]
pub struct AbstractNum {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Abstract Numbering Definition ID
  #[sdk(attr(qname = "w:abstractNumId"))]
  #[sdk(number_range(range = 0..))]
  pub abstract_number_id: crate::simple_type::Int32Value,
  /// Abstract Numbering Definition Identifier
  #[sdk(child(qname = "w:CT_LongHexNumber/w:nsid"))]
  pub nsid: Option<Nsid>,
  /// Abstract Numbering Definition Type
  #[sdk(child(qname = "w:CT_MultiLevelType/w:multiLevelType"))]
  pub multi_level_type: Option<MultiLevelType>,
  /// Numbering Template Code
  #[sdk(child(qname = "w:CT_LongHexNumber/w:tmpl"))]
  pub template_code: Option<TemplateCode>,
  /// Abstract Numbering Definition Name
  #[sdk(child(qname = "w:CT_String253/w:name"))]
  pub abstract_num_definition_name: Option<AbstractNumDefinitionName>,
  /// Numbering Style Definition
  #[sdk(child(qname = "w:CT_String253/w:styleLink"))]
  pub style_link: Option<StyleLink>,
  /// Numbering Style Reference
  #[sdk(child(qname = "w:CT_String253/w:numStyleLink"))]
  pub numbering_style_link: Option<NumberingStyleLink>,
  /// Numbering Level Definition.
  #[sdk(child(qname = "w:CT_Lvl/w:lvl"))]
  pub w_lvl: Vec<Level>,
}
/// Numbering Definition Instance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Num/w:num")]
pub struct NumberingInstance {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// numId
  #[sdk(attr(qname = "w:numId"))]
  pub number_id: crate::simple_type::Int32Value,
  /// durableId
  #[sdk(attr(qname = "w:durableId"))]
  pub w_durable_id: Option<crate::simple_type::Int32Value>,
  /// Defines the AbstractNumId Class.
  #[sdk(child(qname = "w:CT_NonNegativeDecimalNumber/w:abstractNumId"))]
  pub abstract_num_id: std::boxed::Box<AbstractNumId>,
  /// Defines the LevelOverride Class.
  #[sdk(child(qname = "w:CT_NumLvl/w:lvlOverride"))]
  pub w_lvl_override: Vec<LevelOverride>,
}
/// Table Style Conditional Formatting Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPrStyle/w:pPr")]
pub struct StyleParagraphProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:CT_OnOff/w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:CT_FramePr/w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:CT_OnOff/w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:CT_NumPr/w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:CT_PBdr/w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:CT_Tabs/w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:CT_OnOff/w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:CT_OnOff/w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:CT_Spacing/w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:CT_Ind/w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:CT_OnOff/w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:CT_Jc/w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:CT_TextAlignment/w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Defines the ParagraphPropertiesChange Class.
  #[sdk(child(qname = "w:CT_PPrChange/w:pPrChange"))]
  pub paragraph_properties_change: Option<std::boxed::Box<ParagraphPropertiesChange>>,
}
/// Table Style Conditional Formatting Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrStyleOverride/w:tblPr")]
pub struct TableStyleConditionalFormattingTableProperties {
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
}
/// Table Style Conditional Formatting Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrPrBaseStyleable/w:trPr")]
pub struct TableStyleConditionalFormattingTableRowProperties {
  #[sdk(choice(
    qname = "w:CT_OnOff/w:hidden",
    qname = "w:CT_OnOffOnly/w:cantSplit",
    qname = "w:CT_OnOffOnly/w:tblHeader",
    qname = "w:CT_TblWidth/w:tblCellSpacing",
    qname = "w:CT_TblJc/w:jc"
  ))]
  pub table_style_conditional_formatting_table_row_properties_choice:
    Vec<TableStyleConditionalFormattingTableRowPropertiesChoice>,
}
/// Table Style Conditional Formatting Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcPrStyleOverride/w:tcPr")]
pub struct TableStyleConditionalFormattingTableCellProperties {
  /// Defines the TableCellBorders Class.
  #[sdk(child(qname = "w:CT_TcBorders/w:tcBorders"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:CT_TcMar/w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalTblJc/w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
}
/// Primary Style Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_StyleName/w:name")]
pub struct StyleName {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "[^,]*"))]
  pub val: crate::simple_type::StringValue,
}
/// Optional User Interface Sorting Order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UiPriority/w:uiPriority")]
pub struct UiPriority {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 99))]
  pub val: crate::simple_type::Int32Value,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_RPrStyle/w:rPr")]
pub struct StyleRunProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the RunFonts Class.
  #[sdk(child(qname = "w:CT_Fonts/w:rFonts"))]
  pub run_fonts: Option<RunFonts>,
  /// Defines the Bold Class.
  #[sdk(child(qname = "w:CT_OnOff/w:b"))]
  pub bold: Option<Bold>,
  /// Defines the BoldComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bCs"))]
  pub bold_complex_script: Option<BoldComplexScript>,
  /// Defines the Italic Class.
  #[sdk(child(qname = "w:CT_OnOff/w:i"))]
  pub italic: Option<Italic>,
  /// Defines the ItalicComplexScript Class.
  #[sdk(child(qname = "w:CT_OnOff/w:iCs"))]
  pub italic_complex_script: Option<ItalicComplexScript>,
  /// Defines the Caps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:caps"))]
  pub caps: Option<Caps>,
  /// Defines the SmallCaps Class.
  #[sdk(child(qname = "w:CT_OnOff/w:smallCaps"))]
  pub small_caps: Option<SmallCaps>,
  /// Defines the Strike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:strike"))]
  pub strike: Option<Strike>,
  /// Defines the DoubleStrike Class.
  #[sdk(child(qname = "w:CT_OnOff/w:dstrike"))]
  pub double_strike: Option<DoubleStrike>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "w:CT_OnOff/w:outline"))]
  pub outline: Option<Outline>,
  /// Defines the Shadow Class.
  #[sdk(child(qname = "w:CT_OnOff/w:shadow"))]
  pub shadow: Option<Shadow>,
  /// Defines the Emboss Class.
  #[sdk(child(qname = "w:CT_OnOff/w:emboss"))]
  pub emboss: Option<Emboss>,
  /// Defines the Imprint Class.
  #[sdk(child(qname = "w:CT_OnOff/w:imprint"))]
  pub imprint: Option<Imprint>,
  /// Defines the NoProof Class.
  #[sdk(child(qname = "w:CT_OnOff/w:noProof"))]
  pub no_proof: Option<NoProof>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the Vanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:vanish"))]
  pub vanish: Option<Vanish>,
  /// Defines the WebHidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:webHidden"))]
  pub web_hidden: Option<WebHidden>,
  /// Defines the Color Class.
  #[sdk(child(qname = "w:CT_Color/w:color"))]
  pub color: Option<Color>,
  /// Defines the Spacing Class.
  #[sdk(child(qname = "w:CT_ShortTwipsMeasure/w:spacing"))]
  pub spacing: Option<Spacing>,
  /// Defines the CharacterScale Class.
  #[sdk(child(qname = "w:CT_TextScale/w:w"))]
  pub character_scale: Option<CharacterScale>,
  /// Defines the Kern Class.
  #[sdk(child(qname = "w:CT_HpsKern/w:kern"))]
  pub kern: Option<Kern>,
  /// Defines the Position Class.
  #[sdk(child(qname = "w:CT_SignedHpsMeasure/w:position"))]
  pub position: Option<Position>,
  /// Defines the FontSize Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:sz"))]
  pub font_size: Option<FontSize>,
  /// Defines the FontSizeComplexScript Class.
  #[sdk(child(qname = "w:CT_HpsMeasure/w:szCs"))]
  pub font_size_complex_script: Option<FontSizeComplexScript>,
  /// Defines the Underline Class.
  #[sdk(child(qname = "w:CT_Underline/w:u"))]
  pub underline: Option<Underline>,
  /// Defines the TextEffect Class.
  #[sdk(child(qname = "w:CT_TextEffect/w:effect"))]
  pub text_effect: Option<TextEffect>,
  /// Defines the Border Class.
  #[sdk(child(qname = "w:CT_Border/w:bdr"))]
  pub border: Option<Border>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the FitText Class.
  #[sdk(child(qname = "w:CT_FitText/w:fitText"))]
  pub fit_text: Option<FitText>,
  /// Defines the VerticalTextAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalAlignRun/w:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Defines the Emphasis Class.
  #[sdk(child(qname = "w:CT_Em/w:em"))]
  pub emphasis: Option<Emphasis>,
  /// Defines the Languages Class.
  #[sdk(child(qname = "w:CT_Language/w:lang"))]
  pub languages: Option<Languages>,
  /// Defines the EastAsianLayout Class.
  #[sdk(child(qname = "w:CT_EastAsianLayout/w:eastAsianLayout"))]
  pub east_asian_layout: Option<EastAsianLayout>,
  /// Defines the SpecVanish Class.
  #[sdk(child(qname = "w:CT_OnOff/w:specVanish"))]
  pub spec_vanish: Option<SpecVanish>,
  /// Defines the RunPropertiesChange Class.
  #[sdk(child(qname = "w:CT_RPrChange/w:rPrChange"))]
  pub run_properties_change: Option<std::boxed::Box<RunPropertiesChange>>,
}
/// Style Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrStyle/w:tblPr")]
pub struct StyleTableProperties {
  /// Defines the TableStyleRowBandSize Class.
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumberMax3/w:tblStyleRowBandSize"))]
  pub table_style_row_band_size: Option<TableStyleRowBandSize>,
  /// Defines the TableStyleColumnBandSize Class.
  #[sdk(child(qname = "w:CT_UnsignedDecimalNumberMax3/w:tblStyleColBandSize"))]
  pub table_style_column_band_size: Option<TableStyleColumnBandSize>,
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Defines the TableIndentation Class.
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Defines the TableBorders Class.
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the TableCellMarginDefault Class.
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
}
/// Style Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TcPrStyle/w:tcPr")]
pub struct StyleTableCellProperties {
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the NoWrap Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:noWrap"))]
  pub no_wrap: Option<NoWrap>,
  /// Defines the TableCellMargin Class.
  #[sdk(child(qname = "w:CT_TcMar/w:tcMar"))]
  pub table_cell_margin: Option<std::boxed::Box<TableCellMargin>>,
  /// Defines the TableCellVerticalAlignment Class.
  #[sdk(child(qname = "w:CT_VerticalTblJc/w:vAlign"))]
  pub table_cell_vertical_alignment: Option<TableCellVerticalAlignment>,
}
/// Style Conditional Table Formatting Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblStylePr/w:tblStylePr")]
pub struct TableStyleProperties {
  /// Table Style Conditional Formatting Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: TableStyleOverrideValues,
  /// Table Style Conditional Formatting Paragraph Properties
  #[sdk(child(qname = "w:CT_PPrStyle/w:pPr"))]
  pub style_paragraph_properties: Option<std::boxed::Box<StyleParagraphProperties>>,
  /// Table Style Conditional Formatting Run Properties
  #[sdk(child(qname = "w:CT_RPrBaseStyleable/w:rPr"))]
  pub run_properties_base_style: Option<std::boxed::Box<RunPropertiesBaseStyle>>,
  /// Table Style Conditional Formatting Table Properties
  #[sdk(child(qname = "w:CT_TblPrStyleOverride/w:tblPr"))]
  pub table_style_conditional_formatting_table_properties:
    Option<std::boxed::Box<TableStyleConditionalFormattingTableProperties>>,
  /// Table Style Conditional Formatting Table Row Properties
  #[sdk(child(qname = "w:CT_TrPrBaseStyleable/w:trPr"))]
  pub table_style_conditional_formatting_table_row_properties:
    Option<TableStyleConditionalFormattingTableRowProperties>,
  /// Table Style Conditional Formatting Table Cell Properties
  #[sdk(child(qname = "w:CT_TcPrStyleOverride/w:tcPr"))]
  pub table_style_conditional_formatting_table_cell_properties:
    Option<std::boxed::Box<TableStyleConditionalFormattingTableCellProperties>>,
}
/// Latent Style Exception.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LsdException/w:lsdException")]
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
#[sdk(qname = "w:CT_DocDefaults/w:docDefaults")]
pub struct DocDefaults {
  /// Default Run Properties
  #[sdk(child(qname = "w:CT_RPrDefault/w:rPrDefault"))]
  pub run_properties_default: Option<std::boxed::Box<RunPropertiesDefault>>,
  /// Default Paragraph Properties
  #[sdk(child(qname = "w:CT_PPrDefault/w:pPrDefault"))]
  pub paragraph_properties_default: Option<std::boxed::Box<ParagraphPropertiesDefault>>,
}
/// Latent Style Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_LatentStyles/w:latentStyles")]
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
  #[sdk(child(qname = "w:CT_LsdException/w:lsdException"))]
  pub w_lsd_exception: Vec<LatentStyleExceptionInfo>,
}
/// Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Style/w:style")]
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
  #[sdk(child(qname = "w:CT_StyleName/w:name"))]
  pub style_name: Option<StyleName>,
  /// Alternate Style Names
  #[sdk(child(qname = "w:CT_String253/w:aliases"))]
  pub aliases: Option<Aliases>,
  /// Parent Style ID
  #[sdk(child(qname = "w:CT_String253/w:basedOn"))]
  pub based_on: Option<BasedOn>,
  /// Style For Next Paragraph
  #[sdk(child(qname = "w:CT_String253/w:next"))]
  pub next_paragraph_style: Option<NextParagraphStyle>,
  /// Linked Style Reference
  #[sdk(child(qname = "w:CT_String253/w:link"))]
  pub linked_style: Option<LinkedStyle>,
  /// Automatically Merge User Formatting Into Style Definition
  #[sdk(child(qname = "w:CT_OnOffOnly/w:autoRedefine"))]
  pub auto_redefine: Option<AutoRedefine>,
  /// Hide Style From User Interface
  #[sdk(child(qname = "w:CT_OnOffOnly/w:hidden"))]
  pub style_hidden: Option<StyleHidden>,
  /// Optional User Interface Sorting Order
  #[sdk(child(qname = "w:CT_UiPriority/w:uiPriority"))]
  pub ui_priority: Option<UiPriority>,
  /// Hide Style From Main User Interface
  #[sdk(child(qname = "w:CT_OnOffOnly/w:semiHidden"))]
  pub semi_hidden: Option<SemiHidden>,
  /// Remove Semi-Hidden Property When Style Is Used
  #[sdk(child(qname = "w:CT_OnOffOnly/w:unhideWhenUsed"))]
  pub unhide_when_used: Option<UnhideWhenUsed>,
  /// Primary Style
  #[sdk(child(qname = "w:CT_OnOffOnly/w:qFormat"))]
  pub primary_style: Option<PrimaryStyle>,
  /// Style Cannot Be Applied
  #[sdk(child(qname = "w:CT_OnOffOnly/w:locked"))]
  pub locked: Option<Locked>,
  /// E-Mail Message Text Style
  #[sdk(child(qname = "w:CT_OnOffOnly/w:personal"))]
  pub personal: Option<Personal>,
  /// E-Mail Message Composition Style
  #[sdk(child(qname = "w:CT_OnOffOnly/w:personalCompose"))]
  pub personal_compose: Option<PersonalCompose>,
  /// E-Mail Message Reply Style
  #[sdk(child(qname = "w:CT_OnOffOnly/w:personalReply"))]
  pub personal_reply: Option<PersonalReply>,
  /// Revision Identifier for Style Definition
  #[sdk(child(qname = "w:CT_LongHexNumber/w:rsid"))]
  pub rsid: Option<Rsid>,
  /// Style Paragraph Properties
  #[sdk(child(qname = "w:CT_PPrStyle/w:pPr"))]
  pub style_paragraph_properties: Option<std::boxed::Box<StyleParagraphProperties>>,
  /// Run Properties
  #[sdk(child(qname = "w:CT_RPrStyle/w:rPr"))]
  pub style_run_properties: Option<std::boxed::Box<StyleRunProperties>>,
  /// Style Table Properties
  #[sdk(child(qname = "w:CT_TblPrStyle/w:tblPr"))]
  pub style_table_properties: Option<std::boxed::Box<StyleTableProperties>>,
  /// Style Table Row Properties
  #[sdk(child(qname = "w:CT_TrPrBaseStyleable/w:trPr"))]
  pub table_style_conditional_formatting_table_row_properties:
    Option<TableStyleConditionalFormattingTableRowProperties>,
  /// Style Table Cell Properties
  #[sdk(child(qname = "w:CT_TcPrStyle/w:tcPr"))]
  pub style_table_cell_properties: Option<std::boxed::Box<StyleTableCellProperties>>,
  /// Style Conditional Table Formatting Properties.
  #[sdk(child(qname = "w:CT_TblStylePr/w:tblStylePr"))]
  pub w_tbl_style_pr: Vec<TableStyleProperties>,
}
/// Properties for a Single Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Font/w:font")]
pub struct Font {
  /// name
  #[sdk(attr(qname = "w:name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the AltName Class.
  #[sdk(child(qname = "w:CT_String/w:altName"))]
  pub alt_name: Option<AltName>,
  /// Defines the Panose1Number Class.
  #[sdk(child(qname = "w:CT_Panose/w:panose1"))]
  pub panose1_number: Option<Panose1Number>,
  /// Defines the FontCharSet Class.
  #[sdk(child(qname = "w:CT_CharacterSet/w:charset"))]
  pub font_char_set: Option<FontCharSet>,
  /// Defines the FontFamily Class.
  #[sdk(child(qname = "w:CT_FontFamily/w:family"))]
  pub font_family: Option<FontFamily>,
  /// Defines the NotTrueType Class.
  #[sdk(child(qname = "w:CT_OnOff/w:notTrueType"))]
  pub not_true_type: Option<NotTrueType>,
  /// Defines the Pitch Class.
  #[sdk(child(qname = "w:CT_Pitch/w:pitch"))]
  pub pitch: Option<Pitch>,
  /// Defines the FontSignature Class.
  #[sdk(child(qname = "w:CT_FontSig/w:sig"))]
  pub font_signature: Option<FontSignature>,
  /// Defines the EmbedRegularFont Class.
  #[sdk(child(qname = "w:CT_FontRel/w:embedRegular"))]
  pub embed_regular_font: Option<EmbedRegularFont>,
  /// Defines the EmbedBoldFont Class.
  #[sdk(child(qname = "w:CT_FontRel/w:embedBold"))]
  pub embed_bold_font: Option<EmbedBoldFont>,
  /// Defines the EmbedItalicFont Class.
  #[sdk(child(qname = "w:CT_FontRel/w:embedItalic"))]
  pub embed_italic_font: Option<EmbedItalicFont>,
  /// Defines the EmbedBoldItalicFont Class.
  #[sdk(child(qname = "w:CT_FontRel/w:embedBoldItalic"))]
  pub embed_bold_italic_font: Option<EmbedBoldItalicFont>,
}
/// Left Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SignedTwipsMeasure/w:marLeft")]
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
  pub val: crate::simple_type::StringValue,
}
/// Right Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SignedTwipsMeasure/w:marRight")]
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
  pub val: crate::simple_type::StringValue,
}
/// Top Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SignedTwipsMeasure/w:marTop")]
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
  pub val: crate::simple_type::StringValue,
}
/// Bottom Margin for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SignedTwipsMeasure/w:marBottom")]
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
  pub val: crate::simple_type::StringValue,
}
/// Set of Borders for HTML div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DivBdr/w:divBdr")]
pub struct DivBorder {
  /// Top Border for HTML div
  #[sdk(child(qname = "w:CT_Border/w:top"))]
  pub top_border: Option<TopBorder>,
  /// Left Border for HTML div
  #[sdk(child(qname = "w:CT_Border/w:left"))]
  pub left_border: Option<LeftBorder>,
  /// Bottom Border for HTML div
  #[sdk(child(qname = "w:CT_Border/w:bottom"))]
  pub bottom_border: Option<BottomBorder>,
  /// Right Border for HTML div
  #[sdk(child(qname = "w:CT_Border/w:right"))]
  pub right_border: Option<RightBorder>,
}
/// Child div Elements Contained within Current div.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Divs/w:divsChild")]
pub struct DivsChild {
  /// Information About Single HTML div Element.
  #[sdk(child(qname = "w:CT_Div/w:div"))]
  pub w_div: Vec<Div>,
}
/// Defines the Divs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Divs/w:divs")]
pub struct Divs {
  /// Information About Single HTML div Element.
  #[sdk(child(qname = "w:CT_Div/w:div"))]
  pub w_div: Vec<Div>,
}
/// Information About Single HTML div Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Div/w:div")]
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
  #[sdk(child(qname = "w:CT_OnOff/w:blockQuote"))]
  pub block_quote: Option<BlockQuote>,
  /// Data for HTML body Element
  #[sdk(child(qname = "w:CT_OnOff/w:bodyDiv"))]
  pub body_div: Option<BodyDiv>,
  /// Left Margin for HTML div
  #[sdk(child(qname = "w:CT_SignedTwipsMeasure/w:marLeft"))]
  pub left_margin_div: std::boxed::Box<LeftMarginDiv>,
  /// Right Margin for HTML div
  #[sdk(child(qname = "w:CT_SignedTwipsMeasure/w:marRight"))]
  pub right_margin_div: std::boxed::Box<RightMarginDiv>,
  /// Top Margin for HTML div
  #[sdk(child(qname = "w:CT_SignedTwipsMeasure/w:marTop"))]
  pub top_margin_div: std::boxed::Box<TopMarginDiv>,
  /// Bottom Margin for HTML div
  #[sdk(child(qname = "w:CT_SignedTwipsMeasure/w:marBottom"))]
  pub bottom_margin_div: std::boxed::Box<BottomMarginDiv>,
  /// Set of Borders for HTML div
  #[sdk(child(qname = "w:CT_DivBdr/w:divBdr"))]
  pub div_border: Option<std::boxed::Box<DivBorder>>,
  /// Child div Elements Contained within Current div.
  #[sdk(child(qname = "w:CT_Divs/w:divsChild"))]
  pub w_divs_child: Vec<DivsChild>,
}
/// Comment Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Comment/w:comment")]
pub struct Comment {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
    qname = "w:CT_ProofErr/w:proofErr",
    qname = "w:CT_PermStart/w:permStart",
    qname = "w:CT_Perm/w:permEnd",
    qname = "w:CT_Bookmark/w:bookmarkStart",
    qname = "w:CT_MarkupRange/w:bookmarkEnd",
    qname = "w:CT_MarkupRange/w:commentRangeStart",
    qname = "w:CT_MarkupRange/w:commentRangeEnd",
    text,
    any
  ))]
  pub comment_choice: Vec<CommentChoice>,
}
/// Footnote Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdn/w:footnote")]
pub struct Footnote {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Footnote/Endnote Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<FootnoteEndnoteValues>,
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub footnote_choice: Vec<FootnoteChoice>,
}
/// Endnote Content.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnEdn/w:endnote")]
pub struct Endnote {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Footnote/Endnote Type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: Option<FootnoteEndnoteValues>,
  /// Footnote/Endnote ID
  #[sdk(attr(qname = "w:id"))]
  #[sdk(number_range(range = -2147483648..= 32767))]
  pub id: crate::simple_type::IntegerValue,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub endnote_choice: Vec<EndnoteChoice>,
}
/// Entry Insertion Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartBehavior/w:behavior")]
pub struct Behavior {
  /// Insertion Behavior Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartBehaviorValues,
}
/// Entry Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartType/w:type")]
pub struct DocPartType {
  /// Type Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartValues,
}
/// Gallery Associated With Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartGallery/w:gallery")]
pub struct Gallery {
  /// Gallery Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocPartGalleryValues,
}
/// Single Automatic Captioning Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_AutoCaption/w:autoCaption")]
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
#[sdk(qname = "w:CT_Caption/w:caption")]
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
#[sdk(qname = "w:CT_AutoCaptions/w:autoCaptions")]
pub struct AutoCaptions {
  /// Single Automatic Captioning Setting.
  #[sdk(child(qname = "w:CT_AutoCaption/w:autoCaption"))]
  pub w_auto_caption: Vec<AutoCaption>,
}
/// Document Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Background/w:background")]
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
  #[sdk(child(qname = "v:CT_Background/v:background"))]
  pub background: Option<std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Background>>,
}
/// List of Glossary Document Entries.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocParts/w:docParts")]
pub struct DocParts {
  /// Glossary Document Entry.
  #[sdk(child(qname = "w:CT_DocPart/w:docPart"))]
  pub doc_part: std::boxed::Box<DocPart>,
}
/// Entry Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartName/w:name")]
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
#[sdk(qname = "w:CT_DocPartCategory/w:category")]
pub struct Category {
  /// Category Associated With Entry
  #[sdk(child(qname = "w:CT_String/w:name"))]
  pub name: std::boxed::Box<Name>,
  /// Gallery Associated With Entry
  #[sdk(child(qname = "w:CT_DocPartGallery/w:gallery"))]
  pub gallery: std::boxed::Box<Gallery>,
}
/// Entry Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartTypes/w:types")]
pub struct DocPartTypes {
  /// Entry Is Of All Types
  #[sdk(attr(qname = "w:all"))]
  pub all: Option<crate::simple_type::OnOffValue>,
  /// Entry Type.
  #[sdk(child(qname = "w:CT_DocPartType/w:type"))]
  pub doc_part_type: std::boxed::Box<DocPartType>,
}
/// Entry Insertion Behaviors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartBehaviors/w:behaviors")]
pub struct Behaviors {
  /// Entry Insertion Behavior.
  #[sdk(child(qname = "w:CT_DocPartBehavior/w:behavior"))]
  pub behavior: std::boxed::Box<Behavior>,
}
/// Entry ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Guid/w:guid")]
pub struct DocPartId {
  /// GUID Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Glossary Document Entry Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPartPr/w:docPartPr")]
pub struct DocPartProperties {
  /// Entry Name
  #[sdk(child(qname = "w:CT_DocPartName/w:name"))]
  pub doc_part_name: Option<DocPartName>,
  /// Associated Paragraph Style Name
  #[sdk(child(qname = "w:CT_String/w:style"))]
  pub style_id: Option<StyleId>,
  /// Entry Categorization
  #[sdk(child(qname = "w:CT_DocPartCategory/w:category"))]
  pub category: Option<std::boxed::Box<Category>>,
  /// Entry Types
  #[sdk(child(qname = "w:CT_DocPartTypes/w:types"))]
  pub doc_part_types: Option<std::boxed::Box<DocPartTypes>>,
  /// Entry Insertion Behaviors
  #[sdk(child(qname = "w:CT_DocPartBehaviors/w:behaviors"))]
  pub behaviors: Option<std::boxed::Box<Behaviors>>,
  /// Description for Entry
  #[sdk(child(qname = "w:CT_String/w:description"))]
  pub description: Option<Description>,
  /// Entry ID
  #[sdk(child(qname = "w:CT_Guid/w:guid"))]
  pub doc_part_id: Option<DocPartId>,
}
/// Contents of Glossary Document Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Body/w:docPartBody")]
pub struct DocPartBody {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub doc_part_body_choice: Vec<DocPartBodyChoice>,
  /// Section Properties.
  #[sdk(child(qname = "w:CT_SectPr/w:sectPr"))]
  pub w_sect_pr: Option<std::boxed::Box<SectionProperties>>,
}
/// Defines the Body Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Body/w:body")]
pub struct Body {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_AltChunk/w:altChunk",
    qname = "w:CT_CustomXmlBlock/w:customXml",
    qname = "w:CT_SdtBlock/w:sdt",
    qname = "w:CT_P/w:p",
    qname = "w:CT_Tbl/w:tbl",
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
    text,
    any
  ))]
  pub body_choice: Vec<BodyChoice>,
  /// Section Properties.
  #[sdk(child(qname = "w:CT_SectPr/w:sectPr"))]
  pub w_sect_pr: Option<std::boxed::Box<SectionProperties>>,
}
/// Glossary Document Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocPart/w:docPart")]
pub struct DocPart {
  /// Glossary Document Entry Properties
  #[sdk(child(qname = "w:CT_DocPartPr/w:docPartPr"))]
  pub doc_part_properties: Option<std::boxed::Box<DocPartProperties>>,
  /// Contents of Glossary Document Entry
  #[sdk(child(qname = "w:CT_Body/w:docPartBody"))]
  pub doc_part_body: Option<std::boxed::Box<DocPartBody>>,
}
/// Defines the CompatibilitySetting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CompatSetting/w:compatSetting")]
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
#[sdk(qname = "w:CT_TblWidthDxaNil/w:left")]
pub struct TableCellLeftMargin {
  /// w
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_range(range = 0..))]
  pub width: crate::simple_type::Int16Value,
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: TableWidthValues,
}
/// Table Cell Right Margin Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblWidthDxaNil/w:right")]
pub struct TableCellRightMargin {
  /// w
  #[sdk(attr(qname = "w:w"))]
  #[sdk(number_range(range = 0..))]
  pub width: crate::simple_type::Int16Value,
  /// type
  #[sdk(attr(qname = "w:type"))]
  pub r#type: TableWidthValues,
}
/// Table-Level Property Exceptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TblPrEx/w:tblPrEx")]
pub struct TablePropertyExceptions {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Preferred Table Width Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:tblW"))]
  pub table_width: Option<TableWidth>,
  /// Table Alignment Exception
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  pub table_justification: Option<TableJustification>,
  /// Table Cell Spacing Exception
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  pub table_cell_spacing: Option<TableCellSpacing>,
  /// Table Indent from Leading Margin Exception
  #[sdk(child(qname = "w:CT_TblWidthShort/w:tblInd"))]
  pub table_indentation: Option<TableIndentation>,
  /// Table Borders Exceptions
  #[sdk(child(qname = "w:CT_TblBorders/w:tblBorders"))]
  pub table_borders: Option<std::boxed::Box<TableBorders>>,
  /// Table Shading Exception
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Table Layout Exception
  #[sdk(child(qname = "w:CT_TblLayoutType/w:tblLayout"))]
  pub table_layout: Option<TableLayout>,
  /// Table Cell Margin Exceptions
  #[sdk(child(qname = "w:CT_TblCellMar/w:tblCellMar"))]
  pub table_cell_margin_default: Option<std::boxed::Box<TableCellMarginDefault>>,
  /// Table Style Conditional Formatting Settings Exception
  #[sdk(child(qname = "w:CT_TblLook/w:tblLook"))]
  pub table_look: Option<TableLook>,
  /// Revision Information for Table-Level Property Exceptions
  #[sdk(child(qname = "w:CT_TblPrExChange/w:tblPrExChange"))]
  pub table_property_exceptions_change: Option<std::boxed::Box<TablePropertyExceptionsChange>>,
}
/// Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrPr/w:trPr")]
pub struct TableRowProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "w:CT_Cnf/w:cnfStyle",
    qname = "w:CT_NonZeroDecimalNumber/w:divId",
    qname = "w:CT_DecimalNumber/w:gridBefore",
    qname = "w:CT_DecimalNumber/w:gridAfter",
    qname = "w:CT_TblWidth/w:wBefore",
    qname = "w:CT_TblWidth/w:wAfter",
    qname = "w:CT_Height/w:trHeight",
    qname = "w:CT_OnOff/w:hidden",
    qname = "w:CT_OnOffOnly/w:cantSplit",
    qname = "w:CT_OnOffOnly/w:tblHeader",
    qname = "w:CT_TblWidth/w:tblCellSpacing",
    qname = "w:CT_TblJc/w:jc",
    text,
    any
  ))]
  pub table_row_properties_choice1: Vec<TableRowPropertiesChoice>,
  /// Inserted Paragraph.
  #[sdk(child(qname = "w:CT_TrackChange/w:ins"))]
  pub w_ins: Option<Inserted>,
  /// Deleted Paragraph.
  #[sdk(child(qname = "w:CT_TrackChange/w:del"))]
  pub w_del: Option<Deleted>,
  /// Revision Information for Table Row Properties.
  #[sdk(child(qname = "w:CT_TrPrChange/w:trPrChange"))]
  pub w_tr_pr_change: Option<std::boxed::Box<TableRowPropertiesChange>>,
  #[sdk(choice(
    microsoft365,
    qname = "w:CT_TrackChange/w14:conflictIns",
    qname = "w:CT_TrackChange/w14:conflictDel"
  ))]
  pub table_row_properties_choice2: Option<TableRowPropertiesChoice2>,
}
/// Revision Information for Table Row Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrPrChange/w:trPrChange")]
pub struct TableRowPropertiesChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// author
  #[sdk(attr(qname = "w:author"))]
  #[sdk(string_length(max = 255u32))]
  pub author: crate::simple_type::StringValue,
  /// date
  #[sdk(attr(qname = "w:date"))]
  pub date: Option<crate::simple_type::DateTimeValue>,
  /// dateUtc
  #[sdk(attr(microsoft365, qname = "w16du:dateUtc"))]
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
  #[sdk(child(qname = "w:CT_TrPrBase/w:trPr"))]
  pub previous_table_row_properties: std::boxed::Box<PreviousTableRowProperties>,
}
/// Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PPr/w:pPr")]
pub struct ParagraphProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ParagraphStyleId Class.
  #[sdk(child(qname = "w:CT_String/w:pStyle"))]
  pub paragraph_style_id: Option<ParagraphStyleId>,
  /// Defines the KeepNext Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepNext"))]
  pub keep_next: Option<KeepNext>,
  /// Defines the KeepLines Class.
  #[sdk(child(qname = "w:CT_OnOff/w:keepLines"))]
  pub keep_lines: Option<KeepLines>,
  /// Defines the PageBreakBefore Class.
  #[sdk(child(qname = "w:CT_OnOff/w:pageBreakBefore"))]
  pub page_break_before: Option<PageBreakBefore>,
  /// Defines the FrameProperties Class.
  #[sdk(child(qname = "w:CT_FramePr/w:framePr"))]
  pub frame_properties: Option<FrameProperties>,
  /// Defines the WidowControl Class.
  #[sdk(child(qname = "w:CT_OnOff/w:widowControl"))]
  pub widow_control: Option<WidowControl>,
  /// Defines the NumberingProperties Class.
  #[sdk(child(qname = "w:CT_NumPr/w:numPr"))]
  pub numbering_properties: Option<std::boxed::Box<NumberingProperties>>,
  /// Defines the SuppressLineNumbers Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressLineNumbers"))]
  pub suppress_line_numbers: Option<SuppressLineNumbers>,
  /// Defines the ParagraphBorders Class.
  #[sdk(child(qname = "w:CT_PBdr/w:pBdr"))]
  pub paragraph_borders: Option<std::boxed::Box<ParagraphBorders>>,
  /// Defines the Shading Class.
  #[sdk(child(qname = "w:CT_Shd/w:shd"))]
  pub shading: Option<Shading>,
  /// Defines the Tabs Class.
  #[sdk(child(qname = "w:CT_Tabs/w:tabs"))]
  pub tabs: Option<Tabs>,
  /// Defines the SuppressAutoHyphens Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressAutoHyphens"))]
  pub suppress_auto_hyphens: Option<SuppressAutoHyphens>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "w:CT_OnOff/w:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the WordWrap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:wordWrap"))]
  pub word_wrap: Option<WordWrap>,
  /// Defines the OverflowPunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:overflowPunct"))]
  pub overflow_punctuation: Option<OverflowPunctuation>,
  /// Defines the TopLinePunctuation Class.
  #[sdk(child(qname = "w:CT_OnOff/w:topLinePunct"))]
  pub top_line_punctuation: Option<TopLinePunctuation>,
  /// Defines the AutoSpaceDE Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDE"))]
  pub auto_space_de: Option<AutoSpaceDe>,
  /// Defines the AutoSpaceDN Class.
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceDN"))]
  pub auto_space_dn: Option<AutoSpaceDn>,
  /// Defines the BiDi Class.
  #[sdk(child(qname = "w:CT_OnOff/w:bidi"))]
  pub bi_di: Option<BiDi>,
  /// Defines the AdjustRightIndent Class.
  #[sdk(child(qname = "w:CT_OnOff/w:adjustRightInd"))]
  pub adjust_right_indent: Option<AdjustRightIndent>,
  /// Defines the SnapToGrid Class.
  #[sdk(child(qname = "w:CT_OnOff/w:snapToGrid"))]
  pub snap_to_grid: Option<SnapToGrid>,
  /// Defines the SpacingBetweenLines Class.
  #[sdk(child(qname = "w:CT_Spacing/w:spacing"))]
  pub spacing_between_lines: Option<SpacingBetweenLines>,
  /// Defines the Indentation Class.
  #[sdk(child(qname = "w:CT_Ind/w:ind"))]
  pub indentation: Option<Indentation>,
  /// Defines the ContextualSpacing Class.
  #[sdk(child(qname = "w:CT_OnOff/w:contextualSpacing"))]
  pub contextual_spacing: Option<ContextualSpacing>,
  /// Defines the MirrorIndents Class.
  #[sdk(child(qname = "w:CT_OnOff/w:mirrorIndents"))]
  pub mirror_indents: Option<MirrorIndents>,
  /// Defines the SuppressOverlap Class.
  #[sdk(child(qname = "w:CT_OnOff/w:suppressOverlap"))]
  pub suppress_overlap: Option<SuppressOverlap>,
  /// Defines the Justification Class.
  #[sdk(child(qname = "w:CT_Jc/w:jc"))]
  pub justification: Option<Justification>,
  /// Defines the TextDirection Class.
  #[sdk(child(qname = "w:CT_TextDirection/w:textDirection"))]
  pub text_direction: Option<TextDirection>,
  /// Defines the TextAlignment Class.
  #[sdk(child(qname = "w:CT_TextAlignment/w:textAlignment"))]
  pub text_alignment: Option<TextAlignment>,
  /// Defines the TextBoxTightWrap Class.
  #[sdk(child(qname = "w:CT_TextboxTightWrap/w:textboxTightWrap"))]
  pub text_box_tight_wrap: Option<TextBoxTightWrap>,
  /// Defines the OutlineLevel Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:outlineLvl"))]
  pub outline_level: Option<OutlineLevel>,
  /// Defines the DivId Class.
  #[sdk(child(qname = "w:CT_NonZeroDecimalNumber/w:divId"))]
  pub div_id: Option<DivId>,
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  pub conditional_format_style: Option<ConditionalFormatStyle>,
  /// Run Properties for the Paragraph Mark
  #[sdk(child(qname = "w:CT_ParaRPr/w:rPr"))]
  pub paragraph_mark_run_properties: Option<std::boxed::Box<ParagraphMarkRunProperties>>,
  /// Section Properties
  #[sdk(child(qname = "w:CT_SectPr/w:sectPr"))]
  pub section_properties: Option<std::boxed::Box<SectionProperties>>,
  /// Defines the ParagraphPropertiesChange Class.
  #[sdk(child(qname = "w:CT_PPrChange/w:pPrChange"))]
  pub paragraph_properties_change: Option<std::boxed::Box<ParagraphPropertiesChange>>,
}
/// Defines the Control Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Control/w:control")]
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
#[sdk(qname = "w:CT_TblGridBase/w:tblGrid")]
pub struct PreviousTableGrid {
  /// Grid Column Definition.
  #[sdk(child(qname = "w:CT_TblGridCol/w:gridCol"))]
  pub w_grid_col: Vec<GridColumn>,
}
/// Defines the ObjectEmbed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ObjectEmbed/w:objectEmbed")]
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
#[sdk(qname = "w:CT_ObjectLink/w:objectLink")]
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
#[sdk(qname = "w:CT_Lock/w:lock")]
pub struct Lock {
  /// Locking Type
  #[sdk(attr(qname = "w:val"))]
  pub val: Option<LockingValues>,
}
/// Defines the SdtPlaceholder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Placeholder/w:placeholder")]
pub struct SdtPlaceholder {
  /// Document Part Reference
  #[sdk(child(qname = "w:CT_String/w:docPart"))]
  pub doc_part_reference: std::boxed::Box<DocPartReference>,
}
/// Defines the DataBinding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DataBinding/w:dataBinding")]
pub struct DataBinding {
  /// XML Namespace Prefix Mappings
  #[sdk(attr(qname = "w:prefixMappings"))]
  pub prefix_mappings: Option<crate::simple_type::StringValue>,
  /// XPath
  #[sdk(attr(qname = "w:xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Custom XML Data Storage ID
  #[sdk(attr(qname = "w:storeItemID"))]
  pub store_item_id: crate::simple_type::StringValue,
}
/// Defines the SdtContentComboBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtComboBox/w:comboBox")]
pub struct SdtContentComboBox {
  /// Combo Box Last Saved Value
  #[sdk(attr(qname = "w:lastValue"))]
  pub last_value: Option<crate::simple_type::StringValue>,
  /// Combo Box List Item.
  #[sdk(child(qname = "w:CT_SdtListItem/w:listItem"))]
  pub w_list_item: Vec<ListItem>,
}
/// Defines the SdtContentDate Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtDate/w:date")]
pub struct SdtContentDate {
  /// Last Known Date in XML Schema DateTime Format
  #[sdk(attr(qname = "w:fullDate"))]
  pub full_date: Option<crate::simple_type::DateTimeValue>,
  /// Date Display Mask
  #[sdk(child(qname = "w:CT_String/w:dateFormat"))]
  pub date_format: Option<DateFormat>,
  /// Date Picker Language ID
  #[sdk(child(qname = "w:CT_Lang/w:lid"))]
  pub language_id: Option<LanguageId>,
  /// Custom XML Data Date Storage Format
  #[sdk(child(qname = "w:CT_SdtDateMappingType/w:storeMappedDataAs"))]
  pub sdt_date_mapping_type: Option<SdtDateMappingType>,
  /// Date Picker Calendar Type
  #[sdk(child(qname = "w:CT_CalendarType/w:calendar"))]
  pub calendar: Option<Calendar>,
}
/// Defines the SdtContentDocPartObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtDocPart/w:docPartObj")]
pub struct SdtContentDocPartObject {
  /// Document Part Gallery Filter
  #[sdk(child(qname = "w:CT_String/w:docPartGallery"))]
  pub doc_part_gallery: Option<DocPartGallery>,
  /// Document Part Category Filter
  #[sdk(child(qname = "w:CT_String/w:docPartCategory"))]
  pub doc_part_category: Option<DocPartCategory>,
  /// Built-In Document Part
  #[sdk(child(qname = "w:CT_OnOff/w:docPartUnique"))]
  pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtContentDocPartList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtDocPart/w:docPartList")]
pub struct SdtContentDocPartList {
  /// Document Part Gallery Filter
  #[sdk(child(qname = "w:CT_String/w:docPartGallery"))]
  pub doc_part_gallery: Option<DocPartGallery>,
  /// Document Part Category Filter
  #[sdk(child(qname = "w:CT_String/w:docPartCategory"))]
  pub doc_part_category: Option<DocPartCategory>,
  /// Built-In Document Part
  #[sdk(child(qname = "w:CT_OnOff/w:docPartUnique"))]
  pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtContentDropDownList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtDropDownList/w:dropDownList")]
pub struct SdtContentDropDownList {
  /// Drop-down List Last Saved Value
  #[sdk(attr(qname = "w:lastValue"))]
  pub last_value: Option<crate::simple_type::StringValue>,
  /// Combo Box List Item.
  #[sdk(child(qname = "w:CT_SdtListItem/w:listItem"))]
  pub w_list_item: Vec<ListItem>,
}
/// Defines the SdtContentText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_SdtText/w:text")]
pub struct SdtContentText {
  /// Allow Soft Line Breaks
  #[sdk(attr(qname = "w:multiLine"))]
  pub multi_line: Option<crate::simple_type::OnOffValue>,
}
/// Write Protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_WriteProtection/w:writeProtection")]
pub struct WriteProtection {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(office2010, qname = "w:algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(office2010, qname = "w:hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(office2010, qname = "w:saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(office2010, qname = "w:spinCount"))]
  pub spin_count: Option<crate::simple_type::Int32Value>,
}
/// Document View Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_View/w:view")]
pub struct View {
  /// Document View Setting  Value
  #[sdk(attr(qname = "w:val"))]
  pub val: ViewValues,
}
/// Magnification Setting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Zoom/w:zoom")]
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
  pub percent: Option<crate::simple_type::StringValue>,
}
/// Grammar Checking Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_WritingStyle/w:activeWritingStyle")]
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
#[sdk(qname = "w:CT_Proof/w:proofState")]
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
#[sdk(qname = "w:CT_StylePaneFormatFilter/w:stylePaneFormatFilter")]
pub struct StylePaneFormatFilter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub w_val: Option<crate::simple_type::HexBinaryValue>,
  /// allStyles
  #[sdk(attr(office2010, qname = "w:allStyles"))]
  pub all_styles: Option<crate::simple_type::OnOffValue>,
  /// customStyles
  #[sdk(attr(office2010, qname = "w:customStyles"))]
  pub custom_styles: Option<crate::simple_type::OnOffValue>,
  /// latentStyles
  #[sdk(attr(office2010, qname = "w:latentStyles"))]
  pub latent_styles: Option<crate::simple_type::OnOffValue>,
  /// stylesInUse
  #[sdk(attr(office2010, qname = "w:stylesInUse"))]
  pub styles_in_use: Option<crate::simple_type::OnOffValue>,
  /// headingStyles
  #[sdk(attr(office2010, qname = "w:headingStyles"))]
  pub heading_styles: Option<crate::simple_type::OnOffValue>,
  /// numberingStyles
  #[sdk(attr(office2010, qname = "w:numberingStyles"))]
  pub numbering_styles: Option<crate::simple_type::OnOffValue>,
  /// tableStyles
  #[sdk(attr(office2010, qname = "w:tableStyles"))]
  pub table_styles: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnRuns
  #[sdk(attr(office2010, qname = "w:directFormattingOnRuns"))]
  pub direct_formatting_on_runs: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnParagraphs
  #[sdk(attr(office2010, qname = "w:directFormattingOnParagraphs"))]
  pub direct_formatting_on_paragraphs: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnNumbering
  #[sdk(attr(office2010, qname = "w:directFormattingOnNumbering"))]
  pub direct_formatting_on_numbering: Option<crate::simple_type::OnOffValue>,
  /// directFormattingOnTables
  #[sdk(attr(office2010, qname = "w:directFormattingOnTables"))]
  pub direct_formatting_on_tables: Option<crate::simple_type::OnOffValue>,
  /// clearFormatting
  #[sdk(attr(office2010, qname = "w:clearFormatting"))]
  pub clear_formatting: Option<crate::simple_type::OnOffValue>,
  /// top3HeadingStyles
  #[sdk(attr(office2010, qname = "w:top3HeadingStyles"))]
  pub top3_heading_styles: Option<crate::simple_type::OnOffValue>,
  /// visibleStyles
  #[sdk(attr(office2010, qname = "w:visibleStyles"))]
  pub visible_styles: Option<crate::simple_type::OnOffValue>,
  /// alternateStyleNames
  #[sdk(attr(office2010, qname = "w:alternateStyleNames"))]
  pub alternate_style_names: Option<crate::simple_type::OnOffValue>,
}
/// Suggested Sorting for List of Document Styles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_StylePaneSortMethods/w:stylePaneSortMethod")]
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
#[sdk(qname = "w:CT_DocType/w:documentType")]
pub struct DocumentType {
  /// Document Classification Value
  #[sdk(attr(qname = "w:val"))]
  pub val: DocumentTypeValues,
}
/// Mail Merge Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_MailMerge/w:mailMerge")]
pub struct MailMerge {
  /// Source Document Type
  #[sdk(child(qname = "w:CT_MailMergeDocType/w:mainDocumentType"))]
  pub main_document_type: Option<MainDocumentType>,
  /// Query Contains Link to External Query File
  #[sdk(child(qname = "w:CT_OnOff/w:linkToQuery"))]
  pub link_to_query: Option<LinkToQuery>,
  /// Data Source Type
  #[sdk(child(qname = "w:CT_MailMergeDataType/w:dataType"))]
  pub data_type: std::boxed::Box<DataType>,
  /// Data Source Connection String
  #[sdk(child(qname = "w:CT_String/w:connectString"))]
  pub connect_string: Option<ConnectString>,
  /// Query For Data Source Records To Merge
  #[sdk(child(qname = "w:CT_String/w:query"))]
  pub query: Option<Query>,
  /// Data Source File Path
  #[sdk(child(qname = "w:CT_Rel/w:dataSource"))]
  pub data_source_reference: Option<DataSourceReference>,
  /// Header Definition File Path
  #[sdk(child(qname = "w:CT_Rel/w:headerSource"))]
  pub header_source: Option<HeaderSource>,
  /// Remove Blank Lines from Merged Documents
  #[sdk(child(qname = "w:CT_OnOff/w:doNotSuppressBlankLines"))]
  pub do_not_suppress_blank_lines: Option<DoNotSuppressBlankLines>,
  /// Merged Document Destination
  #[sdk(child(qname = "w:CT_MailMergeDest/w:destination"))]
  pub destination: Option<Destination>,
  /// Column Containing E-mail Address
  #[sdk(child(qname = "w:CT_String/w:addressFieldName"))]
  pub address_field_name: Option<AddressFieldName>,
  /// Merged E-mail or Fax Subject Line
  #[sdk(child(qname = "w:CT_String/w:mailSubject"))]
  pub mail_subject: Option<MailSubject>,
  /// Merged Document To E-Mail Attachment
  #[sdk(child(qname = "w:CT_OnOff/w:mailAsAttachment"))]
  pub mail_as_attachment: Option<MailAsAttachment>,
  /// View Merged Data Within Document
  #[sdk(child(qname = "w:CT_OnOff/w:viewMergedData"))]
  pub view_merged_data: Option<ViewMergedData>,
  /// Record Currently Displayed In Merged Document
  #[sdk(child(qname = "w:CT_DecimalNumber/w:activeRecord"))]
  pub active_record: Option<ActiveRecord>,
  /// Mail Merge Error Reporting Setting
  #[sdk(child(qname = "w:CT_DecimalNumber/w:checkErrors"))]
  pub check_errors: Option<CheckErrors>,
  /// Office Data Source Object Settings
  #[sdk(child(qname = "w:CT_Odso/w:odso"))]
  pub data_source_object: Option<std::boxed::Box<DataSourceObject>>,
}
/// Visibility of Annotation Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TrackChangesView/w:revisionView")]
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
#[sdk(qname = "w:CT_DocProtect/w:documentProtection")]
pub struct DocumentProtection {
  pub xml_other_attrs: Vec<(String, String)>,
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
  #[sdk(attr(office2010, qname = "w:algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(office2010, qname = "w:hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(office2010, qname = "w:saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(office2010, qname = "w:spinCount"))]
  pub spin_count: Option<crate::simple_type::Int32Value>,
}
/// Distance Between Automatic Tab Stops.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeShort/w:defaultTabStop")]
pub struct DefaultTabStop {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int16Value,
}
/// Number of Pages Per Booklet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_NonNegativeShort/w:bookFoldPrintingSheets")]
pub struct BookFoldPrintingSheets {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int16Value,
}
/// Maximum Number of Consecutively Hyphenated Lines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedShortNumber/w:consecutiveHyphenLimit")]
pub struct ConsecutiveHyphenLimit {
  /// val
  #[sdk(attr(qname = "w:val"))]
  pub val: crate::simple_type::UInt16Value,
}
/// Percentage of Document to Use When Generating Summary.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedInt100/w:summaryLength")]
pub struct SummaryLength {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 100))]
  pub val: crate::simple_type::Int32Value,
}
/// Distance between Horizontal Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedInt7/w:displayHorizontalDrawingGridEvery")]
pub struct DisplayHorizontalDrawingGrid {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 127))]
  pub val: crate::simple_type::Int32Value,
}
/// Distance between Vertical Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_UnsignedInt7/w:displayVerticalDrawingGridEvery")]
pub struct DisplayVerticalDrawingGrid {
  /// val
  #[sdk(attr(qname = "w:val"))]
  #[sdk(number_range(range = 0..= 127))]
  pub val: crate::simple_type::Int32Value,
}
/// Character-Level Whitespace Compression.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CharacterSpacing/w:characterSpacingControl")]
pub struct CharacterSpacingControl {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: CharacterSpacingValues,
}
/// Custom Set of Characters Which Cannot End a Line.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_KinsokuAfter/w:noLineBreaksAfter")]
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
#[sdk(qname = "w:CT_KinsokuBefore/w:noLineBreaksBefore")]
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
#[sdk(qname = "w:CT_SaveThroughXslt/w:saveThroughXslt")]
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
#[sdk(qname = "w:CT_ShapeDefaults/w:hdrShapeDefaults")]
pub struct HeaderShapeDefaults {
  #[sdk(choice(
    qname = "o:CT_ShapeDefaults/o:shapedefaults",
    qname = "o:CT_ShapeLayout/o:shapelayout"
  ))]
  pub header_shape_defaults_choice: Vec<HeaderShapeDefaultsChoice>,
}
/// Default Properties for VML Objects in Main Document.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ShapeDefaults/w:shapeDefaults")]
pub struct ShapeDefaults {
  #[sdk(choice(
    qname = "o:CT_ShapeDefaults/o:shapedefaults",
    qname = "o:CT_ShapeLayout/o:shapelayout"
  ))]
  pub shape_defaults_choice: Vec<ShapeDefaultsChoice>,
}
/// Document-Wide Footnote Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FtnDocProps/w:footnotePr")]
pub struct FootnoteDocumentWideProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Footnote Placement
  #[sdk(child(qname = "w:CT_FtnPos/w:pos"))]
  pub footnote_position: Option<FootnotePosition>,
  /// Footnote Numbering Format
  #[sdk(child(qname = "w:CT_NumFmt/w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:CT_FtnEdnNumStart/w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:CT_NumRestart/w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
  /// Special Footnote List.
  #[sdk(child(qname = "w:CT_FtnEdnSepRef/w:footnote"))]
  pub w_footnote: Vec<FootnoteSpecialReference>,
}
/// Document-Wide Endnote Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_EdnDocProps/w:endnotePr")]
pub struct EndnoteDocumentWideProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Endnote Placement
  #[sdk(child(qname = "w:CT_EdnPos/w:pos"))]
  pub endnote_position: Option<EndnotePosition>,
  /// Endnote Numbering Format
  #[sdk(child(qname = "w:CT_NumFmt/w:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Footnote and Endnote Numbering Starting Value
  #[sdk(child(qname = "w:CT_FtnEdnNumStart/w:numStart"))]
  pub numbering_start: Option<NumberingStart>,
  /// Footnote and Endnote Numbering Restart Location
  #[sdk(child(qname = "w:CT_NumRestart/w:numRestart"))]
  pub numbering_restart: Option<NumberingRestart>,
  /// Special Endnote List.
  #[sdk(child(qname = "w:CT_FtnEdnSepRef/w:endnote"))]
  pub w_endnote: Vec<EndnoteSpecialReference>,
}
/// Compatibility Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Compat/w:compat")]
pub struct Compatibility {
  /// Use Simplified Rules For Table Border Conflicts
  #[sdk(child(qname = "w:CT_OnOff/w:useSingleBorderforContiguousCells"))]
  pub use_single_border_for_contiguous_cells: Option<UseSingleBorderForContiguousCells>,
  /// Emulate WordPerfect 6.x Paragraph Justification
  #[sdk(child(qname = "w:CT_OnOff/w:wpJustification"))]
  pub word_perfect_justification: Option<WordPerfectJustification>,
  /// Do Not Create Custom Tab Stop for Hanging Indent
  #[sdk(child(qname = "w:CT_OnOff/w:noTabHangInd"))]
  pub no_tab_hang_indent: Option<NoTabHangIndent>,
  /// Do Not Add Leading Between Lines of Text
  #[sdk(child(qname = "w:CT_OnOff/w:noLeading"))]
  pub no_leading: Option<NoLeading>,
  /// Add Additional Space Below Baseline For Underlined East Asian Text
  #[sdk(child(qname = "w:CT_OnOff/w:spaceForUL"))]
  pub space_for_underline: Option<SpaceForUnderline>,
  /// Do Not Balance Text Columns within a Section
  #[sdk(child(qname = "w:CT_OnOff/w:noColumnBalance"))]
  pub no_column_balance: Option<NoColumnBalance>,
  /// Balance Single Byte and Double Byte Characters
  #[sdk(child(qname = "w:CT_OnOff/w:balanceSingleByteDoubleByteWidth"))]
  pub balance_single_byte_double_byte_width: Option<BalanceSingleByteDoubleByteWidth>,
  /// Do Not Center Content on Lines With Exact Line Height
  #[sdk(child(qname = "w:CT_OnOff/w:noExtraLineSpacing"))]
  pub no_extra_line_spacing: Option<NoExtraLineSpacing>,
  /// Convert Backslash To Yen Sign When Entered
  #[sdk(child(qname = "w:CT_OnOff/w:doNotLeaveBackslashAlone"))]
  pub do_not_leave_backslash_alone: Option<DoNotLeaveBackslashAlone>,
  /// Underline All Trailing Spaces
  #[sdk(child(qname = "w:CT_OnOff/w:ulTrailSpace"))]
  pub underline_trailing_spaces: Option<UnderlineTrailingSpaces>,
  /// Don't Justify Lines Ending in Soft Line Break
  #[sdk(child(qname = "w:CT_OnOff/w:doNotExpandShiftReturn"))]
  pub do_not_expand_shift_return: Option<DoNotExpandShiftReturn>,
  /// Only Expand/Condense Text By Whole Points
  #[sdk(child(qname = "w:CT_OnOff/w:spacingInWholePoints"))]
  pub spacing_in_whole_points: Option<SpacingInWholePoints>,
  /// Emulate Word 6.0 Line Wrapping for East Asian Text
  #[sdk(child(qname = "w:CT_OnOff/w:lineWrapLikeWord6"))]
  pub line_wrap_like_word6: Option<LineWrapLikeWord6>,
  /// Print Body Text before Header/Footer Contents
  #[sdk(child(qname = "w:CT_OnOff/w:printBodyTextBeforeHeader"))]
  pub print_body_text_before_header: Option<PrintBodyTextBeforeHeader>,
  /// Print Colors as Black And White without Dithering
  #[sdk(child(qname = "w:CT_OnOff/w:printColBlack"))]
  pub print_color_black_white: Option<PrintColorBlackWhite>,
  /// Space width
  #[sdk(child(qname = "w:CT_OnOff/w:wpSpaceWidth"))]
  pub word_perfect_space_width: Option<WordPerfectSpaceWidth>,
  /// Display Page/Column Breaks Present in Frames
  #[sdk(child(qname = "w:CT_OnOff/w:showBreaksInFrames"))]
  pub show_breaks_in_frames: Option<ShowBreaksInFrames>,
  /// Increase Priority Of Font Size During Font Substitution
  #[sdk(child(qname = "w:CT_OnOff/w:subFontBySize"))]
  pub sub_font_by_size: Option<SubFontBySize>,
  /// Ignore Exact Line Height for Last Line on Page
  #[sdk(child(qname = "w:CT_OnOff/w:suppressBottomSpacing"))]
  pub suppress_bottom_spacing: Option<SuppressBottomSpacing>,
  /// Ignore Minimum and Exact Line Height for First Line on Page
  #[sdk(child(qname = "w:CT_OnOff/w:suppressTopSpacing"))]
  pub suppress_top_spacing: Option<SuppressTopSpacing>,
  /// Ignore Minimum Line Height for First Line on Page
  #[sdk(child(qname = "w:CT_OnOff/w:suppressSpacingAtTopOfPage"))]
  pub suppress_spacing_at_top_of_page: Option<SuppressSpacingAtTopOfPage>,
  /// Emulate WordPerfect 5.x Line Spacing
  #[sdk(child(qname = "w:CT_OnOff/w:suppressTopSpacingWP"))]
  pub suppress_top_spacing_word_perfect: Option<SuppressTopSpacingWordPerfect>,
  /// Do Not Use Space Before On First Line After a Page Break
  #[sdk(child(qname = "w:CT_OnOff/w:suppressSpBfAfterPgBrk"))]
  pub suppress_spacing_before_after_page_break: Option<SuppressSpacingBeforeAfterPageBreak>,
  /// Swap Paragraph Borders on Odd Numbered Pages
  #[sdk(child(qname = "w:CT_OnOff/w:swapBordersFacingPages"))]
  pub swap_borders_facing_pages: Option<SwapBordersFacingPages>,
  /// Treat Backslash Quotation Delimiter as Two Quotation Marks
  #[sdk(child(qname = "w:CT_OnOff/w:convMailMergeEsc"))]
  pub convert_mail_merge_escape: Option<ConvertMailMergeEscape>,
  /// Emulate WordPerfect 6.x Font Height Calculation
  #[sdk(child(qname = "w:CT_OnOff/w:truncateFontHeightsLikeWP6"))]
  pub truncate_font_heights_like_word_perfect: Option<TruncateFontHeightsLikeWordPerfect>,
  /// Emulate Word 5.x for the Macintosh Small Caps Formatting
  #[sdk(child(qname = "w:CT_OnOff/w:mwSmallCaps"))]
  pub mac_word_small_caps: Option<MacWordSmallCaps>,
  /// Use Printer Metrics To Display Documents
  #[sdk(child(qname = "w:CT_OnOff/w:usePrinterMetrics"))]
  pub use_printer_metrics: Option<UsePrinterMetrics>,
  /// Do Not Suppress Paragraph Borders Next To Frames
  #[sdk(child(qname = "w:CT_OnOff/w:doNotSuppressParagraphBorders"))]
  pub do_not_suppress_paragraph_borders: Option<DoNotSuppressParagraphBorders>,
  /// Line Wrap Trailing Spaces
  #[sdk(child(qname = "w:CT_OnOff/w:wrapTrailSpaces"))]
  pub wrap_trail_spaces: Option<WrapTrailSpaces>,
  /// Emulate Word 6.x/95/97 Footnote Placement
  #[sdk(child(qname = "w:CT_OnOff/w:footnoteLayoutLikeWW8"))]
  pub footnote_layout_like_word8: Option<FootnoteLayoutLikeWord8>,
  /// Emulate Word 97 Text Wrapping Around Floating Objects
  #[sdk(child(qname = "w:CT_OnOff/w:shapeLayoutLikeWW8"))]
  pub shape_layout_like_word8: Option<ShapeLayoutLikeWord8>,
  /// Align Table Rows Independently
  #[sdk(child(qname = "w:CT_OnOff/w:alignTablesRowByRow"))]
  pub align_tables_row_by_row: Option<AlignTablesRowByRow>,
  /// Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned
  #[sdk(child(qname = "w:CT_OnOff/w:forgetLastTabAlignment"))]
  pub forget_last_tab_alignment: Option<ForgetLastTabAlignment>,
  /// Add Document Grid Line Pitch To Lines in Table Cells
  #[sdk(child(qname = "w:CT_OnOff/w:adjustLineHeightInTable"))]
  pub adjust_line_height_in_table: Option<AdjustLineHeightInTable>,
  /// Emulate Word 95 Full-Width Character Spacing
  #[sdk(child(qname = "w:CT_OnOff/w:autoSpaceLikeWord95"))]
  pub auto_space_like_word95: Option<AutoSpaceLikeWord95>,
  /// Do Not Increase Line Height for Raised/Lowered Text
  #[sdk(child(qname = "w:CT_OnOff/w:noSpaceRaiseLower"))]
  pub no_space_raise_lower: Option<NoSpaceRaiseLower>,
  /// Use Fixed Paragraph Spacing for HTML Auto Setting
  #[sdk(child(qname = "w:CT_OnOff/w:doNotUseHTMLParagraphAutoSpacing"))]
  pub do_not_use_html_paragraph_auto_spacing: Option<DoNotUseHtmlParagraphAutoSpacing>,
  /// Ignore Space Before Table When Deciding If Table Should Wrap Floating Object
  #[sdk(child(qname = "w:CT_OnOff/w:layoutRawTableWidth"))]
  pub layout_raw_table_width: Option<LayoutRawTableWidth>,
  /// Allow Table Rows to Wrap Inline Objects Independently
  #[sdk(child(qname = "w:CT_OnOff/w:layoutTableRowsApart"))]
  pub layout_table_rows_apart: Option<LayoutTableRowsApart>,
  /// Emulate Word 97 East Asian Line Breaking
  #[sdk(child(qname = "w:CT_OnOff/w:useWord97LineBreakRules"))]
  pub use_word97_line_break_rules: Option<UseWord97LineBreakRules>,
  /// Do Not Allow Floating Tables To Break Across Pages
  #[sdk(child(qname = "w:CT_OnOff/w:doNotBreakWrappedTables"))]
  pub do_not_break_wrapped_tables: Option<DoNotBreakWrappedTables>,
  /// Do Not Snap to Document Grid in Table Cells with Objects
  #[sdk(child(qname = "w:CT_OnOff/w:doNotSnapToGridInCell"))]
  pub do_not_snap_to_grid_in_cell: Option<DoNotSnapToGridInCell>,
  /// Select Field When First or Last Character Is Selected
  #[sdk(child(qname = "w:CT_OnOff/w:selectFldWithFirstOrLastChar"))]
  pub select_field_with_first_or_last_char: Option<SelectFieldWithFirstOrLastChar>,
  /// Use Legacy Ethiopic and Amharic Line Breaking Rules
  #[sdk(child(qname = "w:CT_OnOff/w:applyBreakingRules"))]
  pub apply_breaking_rules: Option<ApplyBreakingRules>,
  /// Do Not Allow Hanging Punctuation With Character Grid
  #[sdk(child(qname = "w:CT_OnOff/w:doNotWrapTextWithPunct"))]
  pub do_not_wrap_text_with_punctuation: Option<DoNotWrapTextWithPunctuation>,
  /// Do Not Compress Compressible Characters When Using Document Grid
  #[sdk(child(qname = "w:CT_OnOff/w:doNotUseEastAsianBreakRules"))]
  pub do_not_use_east_asian_break_rules: Option<DoNotUseEastAsianBreakRules>,
  /// Emulate Word 2002 Table Style Rules
  #[sdk(child(qname = "w:CT_OnOff/w:useWord2002TableStyleRules"))]
  pub use_word2002_table_style_rules: Option<UseWord2002TableStyleRules>,
  /// Allow Tables to AutoFit Into Page Margins
  #[sdk(child(qname = "w:CT_OnOff/w:growAutofit"))]
  pub grow_autofit: Option<GrowAutofit>,
  /// Do Not Bypass East Asian/Complex Script Layout Code
  #[sdk(child(qname = "w:CT_OnOff/w:useFELayout"))]
  pub use_far_east_layout: Option<UseFarEastLayout>,
  /// Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text
  #[sdk(child(qname = "w:CT_OnOff/w:useNormalStyleForList"))]
  pub use_normal_style_for_list: Option<UseNormalStyleForList>,
  /// Ignore Hanging Indent When Creating Tab Stop After Numbering
  #[sdk(child(qname = "w:CT_OnOff/w:doNotUseIndentAsNumberingTabStop"))]
  pub do_not_use_indent_as_numbering_tab_stop: Option<DoNotUseIndentAsNumberingTabStop>,
  /// Use Alternate Set of East Asian Line Breaking Rules
  #[sdk(child(qname = "w:CT_OnOff/w:useAltKinsokuLineBreakRules"))]
  pub use_alt_kinsoku_line_break_rules: Option<UseAltKinsokuLineBreakRules>,
  /// Allow Contextual Spacing of Paragraphs in Tables
  #[sdk(child(qname = "w:CT_OnOff/w:allowSpaceOfSameStyleInTable"))]
  pub allow_space_of_same_style_in_table: Option<AllowSpaceOfSameStyleInTable>,
  /// Do Not Ignore Floating Objects When Calculating Paragraph Indentation
  #[sdk(child(qname = "w:CT_OnOff/w:doNotSuppressIndentation"))]
  pub do_not_suppress_indentation: Option<DoNotSuppressIndentation>,
  /// Do Not AutoFit Tables To Fit Next To Wrapped Objects
  #[sdk(child(qname = "w:CT_OnOff/w:doNotAutofitConstrainedTables"))]
  pub do_not_autofit_constrained_tables: Option<DoNotAutofitConstrainedTables>,
  /// Allow Table Columns To Exceed Preferred Widths of Constituent Cells
  #[sdk(child(qname = "w:CT_OnOff/w:autofitToFirstFixedWidthCell"))]
  pub autofit_to_first_fixed_width_cell: Option<AutofitToFirstFixedWidthCell>,
  /// Underline Following Character Following Numbering
  #[sdk(child(qname = "w:CT_OnOff/w:underlineTabInNumList"))]
  pub underline_tab_in_numbering_list: Option<UnderlineTabInNumberingList>,
  /// Always Use Fixed Width for Hangul Characters
  #[sdk(child(qname = "w:CT_OnOff/w:displayHangulFixedWidth"))]
  pub display_hangul_fixed_width: Option<DisplayHangulFixedWidth>,
  /// Always Move Paragraph Mark to Page after a Page Break
  #[sdk(child(qname = "w:CT_OnOff/w:splitPgBreakAndParaMark"))]
  pub split_page_break_and_paragraph_mark: Option<SplitPageBreakAndParagraphMark>,
  /// Don't Vertically Align Cells Containing Floating Objects
  #[sdk(child(qname = "w:CT_OnOff/w:doNotVertAlignCellWithSp"))]
  pub do_not_vertically_align_cell_with_shape: Option<DoNotVerticallyAlignCellWithShape>,
  /// Don't Break Table Rows Around Floating Tables
  #[sdk(child(qname = "w:CT_OnOff/w:doNotBreakConstrainedForcedTable"))]
  pub do_not_break_constrained_forced_table: Option<DoNotBreakConstrainedForcedTable>,
  /// Ignore Vertical Alignment in Textboxes
  #[sdk(child(qname = "w:CT_OnOff/w:doNotVertAlignInTxbx"))]
  pub do_not_vertically_align_in_text_box: Option<DoNotVerticallyAlignInTextBox>,
  /// Use ANSI Kerning Pairs from Fonts
  #[sdk(child(qname = "w:CT_OnOff/w:useAnsiKerningPairs"))]
  pub use_ansi_kerning_pairs: Option<UseAnsiKerningPairs>,
  /// Use Cached Paragraph Information for Column Balancing
  #[sdk(child(qname = "w:CT_OnOff/w:cachedColBalance"))]
  pub cached_column_balance: Option<CachedColumnBalance>,
  /// Defines the CompatibilitySetting Class.
  #[sdk(child(qname = "w:CT_CompatSetting/w:compatSetting"))]
  pub w_compat_setting: Vec<CompatibilitySetting>,
}
/// Document Variables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocVars/w:docVars")]
pub struct DocumentVariables {
  /// Single Document Variable.
  #[sdk(child(qname = "w:CT_DocVar/w:docVar"))]
  pub w_doc_var: Vec<DocumentVariable>,
}
/// Listing of All Revision Save ID Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_DocRsids/w:rsids")]
pub struct Rsids {
  /// Original Document Revision Save ID
  #[sdk(child(qname = "w:CT_LongHexNumber/w:rsidRoot"))]
  pub rsid_root: Option<RsidRoot>,
  /// Single Session Revision Save ID.
  #[sdk(child(qname = "w:CT_LongHexNumber/w:rsid"))]
  pub w_rsid: Vec<Rsid>,
}
/// Theme Color Mappings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ColorSchemeMapping/w:clrSchemeMapping")]
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
#[sdk(qname = "w:CT_Captions/w:captions")]
pub struct Captions {
  /// Single Caption Type Definition.
  #[sdk(child(qname = "w:CT_Caption/w:caption"))]
  pub w_caption: Vec<Caption>,
  /// Automatic Captioning Settings.
  #[sdk(child(qname = "w:CT_AutoCaptions/w:autoCaptions"))]
  pub w_auto_captions: Option<AutoCaptions>,
}
/// Freeze Document Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_ReadingModeInkLockDown/w:readModeInkLockDown")]
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
  pub font_size: crate::simple_type::StringValue,
}
/// Defines the TargetScreenSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_TargetScreenSz/w:targetScreenSz")]
pub struct TargetScreenSize {
  /// Target Screen Size Value
  #[sdk(attr(qname = "w:val"))]
  pub val: TargetScreenSizeValues,
}
/// Defines the PictureBulletBase Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_PictureBulletBase/w:pict")]
pub struct PictureBulletBase {
  #[sdk(choice(
    qname = "v:CT_Group/v:group",
    qname = "v:CT_Image/v:image",
    qname = "v:CT_Line/v:line",
    qname = "v:CT_Oval/v:oval",
    qname = "v:CT_PolyLine/v:polyline",
    qname = "v:CT_Rect/v:rect",
    qname = "v:CT_RoundRect/v:roundrect",
    qname = "v:CT_Shape/v:shape",
    qname = "v:CT_Shapetype/v:shapetype"
  ))]
  pub picture_bullet_base_choice: Vec<PictureBulletBaseChoice>,
}
/// Defines the Panose1Number Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Panose/w:panose1")]
pub struct Panose1Number {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  #[sdk(string_length(min = 10u32, max = 10u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the FontCharSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_CharacterSet/w:charset")]
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
#[sdk(qname = "w:CT_FontFamily/w:family")]
pub struct FontFamily {
  /// Font Family Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FontFamilyValues,
}
/// Defines the Pitch Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_Pitch/w:pitch")]
pub struct Pitch {
  /// Value
  #[sdk(attr(qname = "w:val"))]
  pub val: FontPitchValues,
}
/// Defines the FontSignature Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w:CT_FontSig/w:sig")]
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
#[sdk(qname = "w:CT_FontRel/w:embedRegular")]
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
#[sdk(qname = "w:CT_FontRel/w:embedBold")]
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
#[sdk(qname = "w:CT_FontRel/w:embedItalic")]
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
#[sdk(qname = "w:CT_FontRel/w:embedBoldItalic")]
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
#[sdk(qname = "w:CT_NumLvl/w:lvlOverride")]
pub struct LevelOverride {
  /// Numbering Level ID
  #[sdk(attr(qname = "w:ilvl"))]
  pub level_index: crate::simple_type::Int32Value,
  /// Numbering Level Starting Value Override
  #[sdk(child(qname = "w:CT_DecimalNumber/w:startOverride"))]
  pub start_override_numbering_value: Option<StartOverrideNumberingValue>,
  /// Numbering Level Override Definition
  #[sdk(child(qname = "w:CT_Lvl/w:lvl"))]
  pub level: Option<std::boxed::Box<Level>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EmbeddedObjectChoice {
  /// Shape Group.
  #[sdk(child(qname = "v:CT_Group/v:group"))]
  VGroup(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Group>),
  /// Image File.
  #[sdk(child(qname = "v:CT_Image/v:image"))]
  VImage(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::ImageFile>),
  /// Line.
  #[sdk(child(qname = "v:CT_Line/v:line"))]
  VLine(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Line>),
  /// Oval.
  #[sdk(child(qname = "v:CT_Oval/v:oval"))]
  VOval(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Oval>),
  /// Multiple Path Line.
  #[sdk(child(qname = "v:CT_PolyLine/v:polyline"))]
  VPolyline(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::PolyLine>),
  /// Rectangle.
  #[sdk(child(qname = "v:CT_Rect/v:rect"))]
  VRect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Rectangle>),
  /// Rounded Rectangle.
  #[sdk(child(qname = "v:CT_RoundRect/v:roundrect"))]
  VRoundrect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::RoundRectangle>),
  /// Shape Definition.
  #[sdk(child(qname = "v:CT_Shape/v:shape"))]
  VShape(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shape>),
  /// Shape Template.
  #[sdk(child(qname = "v:CT_Shapetype/v:shapetype"))]
  VShapetype(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shapetype>),
  /// Arc Segment.
  #[sdk(child(qname = "v:CT_Arc/v:arc"))]
  VArc(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Arc>),
  /// Bezier Curve.
  #[sdk(child(qname = "v:CT_Curve/v:curve"))]
  VCurve(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Curve>),
  /// Embedded OLE Object.
  #[sdk(child(qname = "o:CT_OLEObject/o:OLEObject"))]
  OOleObject(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::OleObject>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EmbeddedObjectChoice2 {
  /// Defines the Control Class.
  #[sdk(child(qname = "w:CT_Control/w:control"))]
  WControl(std::boxed::Box<Control>),
  /// Defines the ObjectEmbed Class.
  #[sdk(child(qname = "w:CT_ObjectEmbed/w:objectEmbed"))]
  WObjectEmbed(std::boxed::Box<ObjectEmbed>),
  /// Defines the ObjectLink Class.
  #[sdk(child(qname = "w:CT_ObjectLink/w:objectLink"))]
  WObjectLink(std::boxed::Box<ObjectLink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PictureChoice {
  /// Shape Group.
  #[sdk(child(qname = "v:CT_Group/v:group"))]
  VGroup(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Group>),
  /// Image File.
  #[sdk(child(qname = "v:CT_Image/v:image"))]
  VImage(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::ImageFile>),
  /// Line.
  #[sdk(child(qname = "v:CT_Line/v:line"))]
  VLine(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Line>),
  /// Oval.
  #[sdk(child(qname = "v:CT_Oval/v:oval"))]
  VOval(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Oval>),
  /// Multiple Path Line.
  #[sdk(child(qname = "v:CT_PolyLine/v:polyline"))]
  VPolyline(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::PolyLine>),
  /// Rectangle.
  #[sdk(child(qname = "v:CT_Rect/v:rect"))]
  VRect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Rectangle>),
  /// Rounded Rectangle.
  #[sdk(child(qname = "v:CT_RoundRect/v:roundrect"))]
  VRoundrect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::RoundRectangle>),
  /// Shape Definition.
  #[sdk(child(qname = "v:CT_Shape/v:shape"))]
  VShape(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shape>),
  /// Shape Template.
  #[sdk(child(qname = "v:CT_Shapetype/v:shapetype"))]
  VShapetype(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shapetype>),
  /// Arc Segment.
  #[sdk(child(qname = "v:CT_Arc/v:arc"))]
  VArc(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Arc>),
  /// Bezier Curve.
  #[sdk(child(qname = "v:CT_Curve/v:curve"))]
  VCurve(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Curve>),
  /// Embedded OLE Object.
  #[sdk(child(qname = "o:CT_OLEObject/o:OLEObject"))]
  OOleObject(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::OleObject>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FieldCharChoice {
  /// Custom Field Data.
  #[sdk(text_child(qname = "w:CT_Base64BinaryText/w:fldData"))]
  WFldData(crate::simple_type::Base64BinaryValue),
  /// Form Field Properties.
  #[sdk(child(qname = "w:CT_FFData/w:ffData"))]
  WFfData(std::boxed::Box<FormFieldData>),
  /// Previous Paragraph Numbering Properties.
  #[sdk(child(qname = "w:CT_TrackChangeNumbering/w:numberingChange"))]
  WNumberingChange(std::boxed::Box<NumberingChange>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DrawingChoice {
  /// Anchor for Floating DrawingML Object.
  #[sdk(child(qname = "wp:CT_Anchor/wp:anchor"))]
  WpAnchor(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Anchor,
    >,
  ),
  /// Inline DrawingML Object.
  #[sdk(child(qname = "wp:CT_Inline/wp:inline"))]
  WpInline(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Inline,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InsertedMathControlChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  Sequence(std::boxed::Box<RunProperties>),
  /// Defines the DeletedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlDel/w:del"))]
  WDel(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MoveFromMathControlChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  Sequence(std::boxed::Box<RunProperties>),
  /// Defines the InsertedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlIns/w:ins"))]
  WIns(std::boxed::Box<InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlDel/w:del"))]
  WDel(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MoveToMathControlChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  Sequence(std::boxed::Box<RunProperties>),
  /// Defines the InsertedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlIns/w:ins"))]
  WIns(std::boxed::Box<InsertedMathControl>),
  /// Defines the DeletedMathControl Class.
  #[sdk(child(qname = "w:CT_MathCtrlDel/w:del"))]
  WDel(std::boxed::Box<DeletedMathControl>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlRubyChoice {
  /// Defines the CustomXmlRuby Class.
    #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    /// Defines the SimpleFieldRuby Class.
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    /// Defines the HyperlinkRuby Class.
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the SdtRunRuby Class.
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SimpleFieldRubyChoice {
  /// Defines the CustomXmlRuby Class.
    #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    /// Defines the SimpleFieldRuby Class.
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    /// Defines the HyperlinkRuby Class.
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the SdtRunRuby Class.
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HyperlinkRubyChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunChoice {
  #[sdk(child(qname = "w:CT_Br/w:br"))]
  WBr(std::boxed::Box<Break>),
  #[sdk(child(qname = "w:CT_Text/w:t"))]
  WT(std::boxed::Box<Text>),
  #[sdk(child(qname = "w:CT_Text/w:delText"))]
  WDelText(std::boxed::Box<DeletedText>),
  #[sdk(child(qname = "w:CT_Text/w:instrText"))]
  WInstrText(std::boxed::Box<FieldCode>),
  #[sdk(child(qname = "w:CT_Text/w:delInstrText"))]
  WDelInstrText(std::boxed::Box<DeletedFieldCode>),
  /// Non Breaking Hyphen Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:noBreakHyphen"))]
  WNoBreakHyphen,
  /// Optional Hyphen Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:softHyphen"))]
  WSoftHyphen,
  /// Date Block - Short Day Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:dayShort"))]
  WDayShort,
  /// Date Block - Short Month Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:monthShort"))]
  WMonthShort,
  /// Date Block - Short Year Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:yearShort"))]
  WYearShort,
  /// Date Block - Long Day Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:dayLong"))]
  WDayLong,
  /// Date Block - Long Month Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:monthLong"))]
  WMonthLong,
  /// Date Block - Long Year Format.
  #[sdk(empty_child(qname = "w:CT_Empty/w:yearLong"))]
  WYearLong,
  /// Comment Information Block.
  #[sdk(empty_child(qname = "w:CT_Empty/w:annotationRef"))]
  WAnnotationRef,
  /// Footnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:footnoteRef"))]
  WFootnoteRef,
  /// Endnote Reference Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:endnoteRef"))]
  WEndnoteRef,
  /// Footnote/Endnote Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:separator"))]
  WSeparator,
  /// Continuation Separator Mark.
  #[sdk(empty_child(qname = "w:CT_Empty/w:continuationSeparator"))]
  WContinuationSeparator,
  #[sdk(child(qname = "w:CT_Sym/w:sym"))]
  WSym(std::boxed::Box<SymbolChar>),
  /// Page Number Block.
  #[sdk(empty_child(qname = "w:CT_Empty/w:pgNum"))]
  WPgNum,
  /// Carriage Return.
  #[sdk(empty_child(qname = "w:CT_Empty/w:cr"))]
  WCr,
  /// Tab Character.
  #[sdk(empty_child(qname = "w:CT_Empty/w:tab"))]
  WTab,
  #[sdk(child(qname = "w:CT_Object/w:object"))]
  WObject(std::boxed::Box<EmbeddedObject>),
  #[sdk(child(qname = "w:CT_Picture/w:pict"))]
  WPict(std::boxed::Box<Picture>),
  #[sdk(child(qname = "w:CT_FldChar/w:fldChar"))]
  WFldChar(std::boxed::Box<FieldChar>),
  #[sdk(child(qname = "w:CT_Ruby/w:ruby"))]
  WRuby(std::boxed::Box<Ruby>),
  #[sdk(child(qname = "w:CT_FtnEdnRef/w:footnoteReference"))]
  WFootnoteReference(std::boxed::Box<FootnoteReference>),
  #[sdk(child(qname = "w:CT_FtnEdnRef/w:endnoteReference"))]
  WEndnoteReference(std::boxed::Box<EndnoteReference>),
  #[sdk(child(qname = "w:CT_Markup/w:commentReference"))]
  WCommentReference(std::boxed::Box<CommentReference>),
  #[sdk(child(qname = "w:CT_Drawing/w:drawing"))]
  WDrawing(std::boxed::Box<Drawing>),
  #[sdk(child(qname = "w:CT_PTab/w:ptab"))]
  WPtab(std::boxed::Box<PositionalTab>),
  /// Position of Last Calculated Page Break.
  #[sdk(empty_child(qname = "w:CT_Empty/w:lastRenderedPageBreak"))]
  WLastRenderedPageBreak,
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtRunRubyChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InsertedRunChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DeletedRunChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MoveFromRunChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MoveToRunChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtRunChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlBlockChoice {
  /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtBlockChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice {
  /// Defines the CustomXmlRun Class.
    #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    /// Defines the SimpleField Class.
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    /// Defines the Hyperlink Class.
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Anchor for Subdocument Location.
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableChoice2 {
  /// Table Row.
    #[sdk(child(qname = "w:CT_Row/w:tr"))]
    WTr(std::boxed::Box<TableRow>),
    /// Row-Level Custom XML Element.
    #[sdk(child(qname = "w:CT_CustomXmlRow/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRow>),
    /// Row-Level Structured Document Tag.
    #[sdk(child(qname = "w:CT_SdtRow/w:sdt"))]
    WSdt(std::boxed::Box<SdtRow>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableRowChoice {
  /// Table Cell.
    #[sdk(child(qname = "w:CT_Tc/w:tc"))]
    WTc(std::boxed::Box<TableCell>),
    /// Cell-Level Custom XML Element.
    #[sdk(child(qname = "w:CT_CustomXmlCell/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlCell>),
    /// Cell-Level Structured Document Tag.
    #[sdk(child(qname = "w:CT_SdtCell/w:sdt"))]
    WSdt(std::boxed::Box<SdtCell>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlRowChoice {
  /// Table Row.
    #[sdk(child(qname = "w:CT_Row/w:tr"))]
    WTr(std::boxed::Box<TableRow>),
    /// Row-Level Custom XML Element.
    #[sdk(child(qname = "w:CT_CustomXmlRow/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRow>),
    /// Row-Level Structured Document Tag.
    #[sdk(child(qname = "w:CT_SdtRow/w:sdt"))]
    WSdt(std::boxed::Box<SdtRow>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtRowChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellChoice {
  /// Defines the AltChunk Class.
    #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlCellChoice {
  /// Table Cell.
    #[sdk(child(qname = "w:CT_Tc/w:tc"))]
    WTc(std::boxed::Box<TableCell>),
    /// Cell-Level Custom XML Element.
    #[sdk(child(qname = "w:CT_CustomXmlCell/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlCell>),
    /// Cell-Level Structured Document Tag.
    #[sdk(child(qname = "w:CT_SdtCell/w:sdt"))]
    WSdt(std::boxed::Box<SdtCell>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtCellChoice {
  /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlRunChoice {
  /// Defines the CustomXmlRun Class.
    #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    /// Defines the SimpleField Class.
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    /// Defines the Hyperlink Class.
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Anchor for Subdocument Location.
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SimpleFieldChoice {
  /// Defines the CustomXmlRun Class.
    #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    /// Defines the SimpleField Class.
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    /// Defines the Hyperlink Class.
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    /// Defines the OfficeMath Class.
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    /// Accent.
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    /// Bar.
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    /// Box Function.
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    /// Border-Box Function.
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    /// Delimiter Function.
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    /// Equation-Array Function.
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    /// Fraction Function.
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    /// Function Apply Function.
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    /// Group-Character Function.
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    /// Lower-Limit Function.
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    /// Upper-Limit Function.
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    /// Matrix Function.
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    /// n-ary Operator Function.
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    /// Phantom Function.
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    /// Radical Function.
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    /// Pre-Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    /// Subscript Function.
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    /// Sub-Superscript Function.
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    /// Superscript Function.
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    /// Defines the Run Class.
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Phonetic Guide Text Run.
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    /// Anchor for Subdocument Location.
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HyperlinkChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BidirectionalOverrideChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BidirectionalEmbeddingChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBoxContentChoice {
  #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HeaderChoice {
  #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FooterChoice {
  #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FontsChoice {
  /// Properties for a Single Font.
  #[sdk(child(qname = "w:CT_Font/w:font"))]
  WFont(std::boxed::Box<Font>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PreviousTableCellPropertiesChoice {
  #[sdk(child(qname = "w:CT_TrackChange/w:cellIns"))]
  WCellIns(std::boxed::Box<CellInsertion>),
  #[sdk(child(qname = "w:CT_TrackChange/w:cellDel"))]
  WCellDel(std::boxed::Box<CellDeletion>),
  #[sdk(child(qname = "w:CT_CellMergeTrackChange/w:cellMerge"))]
  WCellMerge(std::boxed::Box<CellMerge>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PreviousTableRowPropertiesChoice {
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  WCnfStyle(std::boxed::Box<ConditionalFormatStyle>),
  #[sdk(child(qname = "w:CT_NonZeroDecimalNumber/w:divId"))]
  WDivId(std::boxed::Box<DivId>),
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridBefore"))]
  WGridBefore(std::boxed::Box<GridBefore>),
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridAfter"))]
  WGridAfter(std::boxed::Box<GridAfter>),
  #[sdk(child(qname = "w:CT_TblWidth/w:wBefore"))]
  WWBefore(std::boxed::Box<WidthBeforeTableRow>),
  #[sdk(child(qname = "w:CT_TblWidth/w:wAfter"))]
  WWAfter(std::boxed::Box<WidthAfterTableRow>),
  #[sdk(child(qname = "w:CT_Height/w:trHeight"))]
  WTrHeight(std::boxed::Box<TableRowHeight>),
  #[sdk(child(qname = "w:CT_OnOff/w:hidden"))]
  WHidden(std::boxed::Box<Hidden>),
  #[sdk(child(qname = "w:CT_OnOffOnly/w:cantSplit"))]
  WCantSplit(std::boxed::Box<CantSplit>),
  #[sdk(child(qname = "w:CT_OnOffOnly/w:tblHeader"))]
  WTblHeader(std::boxed::Box<TableHeader>),
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  WTblCellSpacing(std::boxed::Box<TableCellSpacing>),
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  WJc(std::boxed::Box<TableJustification>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PreviousParagraphMarkRunPropertiesChoice {
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictIns"))]
  W14ConflictIns(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    >,
  ),
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictDel"))]
  W14ConflictDel(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphMarkRunPropertiesChoice {
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictIns"))]
  W14ConflictIns(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    >,
  ),
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictDel"))]
  W14ConflictDel(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SectionPropertiesChoice {
  #[sdk(child(qname = "w:CT_HdrFtrRef/w:headerReference"))]
  WHeaderReference(std::boxed::Box<HeaderReference>),
  #[sdk(child(qname = "w:CT_HdrFtrRef/w:footerReference"))]
  WFooterReference(std::boxed::Box<FooterReference>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FormFieldDataChoice {
  #[sdk(child(qname = "w:CT_FFName/w:name"))]
  WName(std::boxed::Box<FormFieldName>),
  #[sdk(child(qname = "w:CT_OnOff/w:enabled"))]
  WEnabled(std::boxed::Box<Enabled>),
  #[sdk(child(qname = "w:CT_OnOff/w:calcOnExit"))]
  WCalcOnExit(std::boxed::Box<CalculateOnExit>),
  #[sdk(child(qname = "w:CT_MacroName/w:entryMacro"))]
  WEntryMacro(std::boxed::Box<EntryMacro>),
  #[sdk(child(qname = "w:CT_MacroName/w:exitMacro"))]
  WExitMacro(std::boxed::Box<ExitMacro>),
  #[sdk(child(qname = "w:CT_FFHelpText/w:helpText"))]
  WHelpText(std::boxed::Box<HelpText>),
  #[sdk(child(qname = "w:CT_FFStatusText/w:statusText"))]
  WStatusText(std::boxed::Box<StatusText>),
  #[sdk(child(qname = "w:CT_FFCheckBox/w:checkBox"))]
  WCheckBox(std::boxed::Box<CheckBox>),
  #[sdk(child(qname = "w:CT_FFDDList/w:ddList"))]
  WDdList(std::boxed::Box<DropDownListFormField>),
  #[sdk(child(qname = "w:CT_FFTextInput/w:textInput"))]
  WTextInput(std::boxed::Box<TextInput>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CheckBoxChoice {
  #[sdk(child(qname = "w:CT_HpsMeasure/w:size"))]
  WSize(std::boxed::Box<FormFieldSize>),
  #[sdk(child(qname = "w:CT_OnOff/w:sizeAuto"))]
  WSizeAuto(std::boxed::Box<AutomaticallySizeFormField>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RubyContentChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RubyBaseChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtPropertiesChoice {
  #[sdk(child(qname = "w:CT_RPr/w:rPr"))]
  WRPr(std::boxed::Box<RunProperties>),
  #[sdk(child(qname = "w:CT_String/w:alias"))]
  WAlias(std::boxed::Box<SdtAlias>),
  #[sdk(child(qname = "w:CT_Lock/w:lock"))]
  WLock(std::boxed::Box<Lock>),
  #[sdk(child(qname = "w:CT_Placeholder/w:placeholder"))]
  WPlaceholder(std::boxed::Box<SdtPlaceholder>),
  #[sdk(child(qname = "w:CT_OnOff/w:showingPlcHdr"))]
  WShowingPlcHdr(std::boxed::Box<ShowingPlaceholder>),
  #[sdk(child(qname = "w:CT_DataBinding/w:dataBinding"))]
  WDataBinding(std::boxed::Box<DataBinding>),
  #[sdk(child(office2013, qname = "w:CT_DataBinding/w15:dataBinding"))]
  W15DataBinding(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::DataBinding>,
  ),
  #[sdk(child(qname = "w:CT_OnOff/w:temporary"))]
  WTemporary(std::boxed::Box<TemporarySdt>),
  #[sdk(child(qname = "w:CT_DecimalNumber/w:id"))]
  WId(std::boxed::Box<SdtId>),
  #[sdk(child(qname = "w:CT_String/w:tag"))]
  WTag(std::boxed::Box<Tag>),
  #[sdk(child(office2013, qname = "w:CT_Color/w15:color"))]
  W15Color(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Color>),
  #[sdk(child(office2013, qname = "w15:CT_SdtAppearance/w15:appearance"))]
  W15Appearance(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Appearance>,
  ),
  #[sdk(child(office2013, qname = "w:CT_OnOff/w15:webExtensionLinked"))]
  W15WebExtensionLinked(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionLinked,
    >,
  ),
  #[sdk(child(office2013, qname = "w:CT_OnOff/w15:webExtensionCreated"))]
  W15WebExtensionCreated(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionCreated,
    >,
  ),
  /// Defines the SdtContentEquation Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:equation"))]
  WEquation,
  #[sdk(child(qname = "w:CT_SdtComboBox/w:comboBox"))]
  WComboBox(std::boxed::Box<SdtContentComboBox>),
  #[sdk(child(qname = "w:CT_SdtDate/w:date"))]
  WDate(std::boxed::Box<SdtContentDate>),
  #[sdk(child(qname = "w:CT_SdtDocPart/w:docPartObj"))]
  WDocPartObj(std::boxed::Box<SdtContentDocPartObject>),
  #[sdk(child(qname = "w:CT_SdtDocPart/w:docPartList"))]
  WDocPartList(std::boxed::Box<SdtContentDocPartList>),
  #[sdk(child(qname = "w:CT_SdtDropDownList/w:dropDownList"))]
  WDropDownList(std::boxed::Box<SdtContentDropDownList>),
  /// Defines the SdtContentPicture Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:picture"))]
  WPicture,
  /// Defines the SdtContentRichText Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:richText"))]
  WRichText,
  #[sdk(child(qname = "w:CT_SdtText/w:text"))]
  WText(std::boxed::Box<SdtContentText>),
  /// Defines the SdtContentCitation Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:citation"))]
  WCitation,
  /// Defines the SdtContentGroup Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:group"))]
  WGroup,
  /// Defines the SdtContentBibliography Class.
  #[sdk(empty_child(qname = "w:CT_Empty/w:bibliography"))]
  WBibliography,
  /// Defines the EntityPickerEmpty Class.
  #[sdk(empty_child(office2010, qname = "w:CT_Empty/w14:entityPicker"))]
  W14EntityPicker,
  #[sdk(child(office2010, qname = "w14:CT_SdtCheckbox/w14:checkbox"))]
  W14Checkbox(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::SdtContentCheckBox,
    >,
  ),
  #[sdk(child(office2013, qname = "w15:CT_SdtRepeatedSection/w15:repeatingSection"))]
  W15RepeatingSection(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2012_wordml::SdtRepeatedSection,
    >,
  ),
  /// Defines the SdtRepeatedSectionItem Class.
  #[sdk(empty_child(office2013, qname = "w:CT_Empty/w15:repeatingSectionItem"))]
  W15RepeatingSectionItem,
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtContentBlockChoice {
  #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtContentRunChoice {
  #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    #[sdk(child(qname = "w:CT_CustomXmlRun/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRun>),
    #[sdk(child(qname = "w:CT_SimpleField/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleField>),
    #[sdk(child(qname = "w:CT_Hyperlink/w:hyperlink"))]
    WHyperlink(std::boxed::Box<Hyperlink>),
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(std::boxed::Box<SdtRun>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(std::boxed::Box<BidirectionalOverride>),
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(std::boxed::Box<BidirectionalEmbedding>),
    #[sdk(child(qname = "w:CT_Rel/w:subDoc"))]
    WSubDoc(std::boxed::Box<SubDocumentReference>),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtContentRunRubyChoice {
  #[sdk(child(qname = "w:CT_CustomXmlRuby/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRuby>),
    #[sdk(child(qname = "w:CT_SimpleFieldRuby/w:fldSimple"))]
    WFldSimple(std::boxed::Box<SimpleFieldRuby>),
    #[sdk(child(qname = "w:CT_HyperlinkRuby/w:hyperlink"))]
    WHyperlink(std::boxed::Box<HyperlinkRuby>),
    #[sdk(child(qname = "w:CT_R/w:r"))]
    WR(std::boxed::Box<Run>),
    #[sdk(child(qname = "w:CT_SdtRunRuby/w:sdt"))]
    WSdt(std::boxed::Box<SdtRunRuby>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    #[sdk(child(qname = "m:CT_OMathPara/m:oMathPara"))]
    MOMathPara(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
        >,
    ),
    #[sdk(child(qname = "m:CT_OMath/m:oMath"))]
    MOMath(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
        >,
    ),
    #[sdk(child(qname = "m:CT_Acc/m:acc"))]
    MAcc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent,
        >,
    ),
    #[sdk(child(qname = "m:CT_Bar/m:bar"))]
    MBar(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar,
        >,
    ),
    #[sdk(child(qname = "m:CT_Box/m:box"))]
    MBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box,
        >,
    ),
    #[sdk(child(qname = "m:CT_BorderBox/m:borderBox"))]
    MBorderBox(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
        >,
    ),
    #[sdk(child(qname = "m:CT_D/m:d"))]
    MD(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter,
        >,
    ),
    #[sdk(child(qname = "m:CT_EqArr/m:eqArr"))]
    MEqArr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
        >,
    ),
    #[sdk(child(qname = "m:CT_F/m:f"))]
    MF(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction,
        >,
    ),
    #[sdk(child(qname = "m:CT_Func/m:func"))]
    MFunc(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
        >,
    ),
    #[sdk(child(qname = "m:CT_GroupChr/m:groupChr"))]
    MGroupChr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimLow/m:limLow"))]
    MLimLow(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
        >,
    ),
    #[sdk(child(qname = "m:CT_LimUpp/m:limUpp"))]
    MLimUpp(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
        >,
    ),
    #[sdk(child(qname = "m:CT_M/m:m"))]
    MM(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix,
        >,
    ),
    #[sdk(child(qname = "m:CT_Nary/m:nary"))]
    MNary(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary,
        >,
    ),
    #[sdk(child(qname = "m:CT_Phant/m:phant"))]
    MPhant(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
        >,
    ),
    #[sdk(child(qname = "m:CT_Rad/m:rad"))]
    MRad(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical,
        >,
    ),
    #[sdk(child(qname = "m:CT_SPre/m:sPre"))]
    MSPre(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSub/m:sSub"))]
    MSSub(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSubSup/m:sSubSup"))]
    MSSubSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_SSup/m:sSup"))]
    MSSup(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
        >,
    ),
    #[sdk(child(qname = "m:CT_R/m:r"))]
    MR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run,
        >,
    ),
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtContentCellChoice {
  #[sdk(child(qname = "w:CT_Tc/w:tc"))]
    WTc(std::boxed::Box<TableCell>),
    #[sdk(child(qname = "w:CT_CustomXmlCell/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlCell>),
    #[sdk(child(qname = "w:CT_SdtCell/w:sdt"))]
    WSdt(std::boxed::Box<SdtCell>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SdtContentRowChoice {
  #[sdk(child(qname = "w:CT_Row/w:tr"))]
    WTr(std::boxed::Box<TableRow>),
    #[sdk(child(qname = "w:CT_CustomXmlRow/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlRow>),
    #[sdk(child(qname = "w:CT_SdtRow/w:sdt"))]
    WSdt(std::boxed::Box<SdtRow>),
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellPropertiesChoice {
  #[sdk(child(qname = "w:CT_TrackChange/w:cellIns"))]
  WCellIns(std::boxed::Box<CellInsertion>),
  #[sdk(child(qname = "w:CT_TrackChange/w:cellDel"))]
  WCellDel(std::boxed::Box<CellDeletion>),
  #[sdk(child(qname = "w:CT_CellMergeTrackChange/w:cellMerge"))]
  WCellMerge(std::boxed::Box<CellMerge>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FramesetChoice {
  #[sdk(child(qname = "w:CT_Frameset/w:frameset"))]
  WFrameset(std::boxed::Box<Frameset>),
  #[sdk(child(qname = "w:CT_Frame/w:frame"))]
  WFrame(std::boxed::Box<Frame>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumberingPictureBulletChoice {
  /// Defines the PictureBulletBase Class.
  #[sdk(child(qname = "w:CT_PictureBulletBase/w:pict"))]
  WPict(std::boxed::Box<PictureBulletBase>),
  /// DrawingML Object.
  #[sdk(child(qname = "w:CT_Drawing/w:drawing"))]
  WDrawing(std::boxed::Box<Drawing>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableStyleConditionalFormattingTableRowPropertiesChoice {
  #[sdk(child(qname = "w:CT_OnOff/w:hidden"))]
  WHidden(std::boxed::Box<Hidden>),
  #[sdk(child(qname = "w:CT_OnOffOnly/w:cantSplit"))]
  WCantSplit(std::boxed::Box<CantSplit>),
  #[sdk(child(qname = "w:CT_OnOffOnly/w:tblHeader"))]
  WTblHeader(std::boxed::Box<TableHeader>),
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  WTblCellSpacing(std::boxed::Box<TableCellSpacing>),
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  WJc(std::boxed::Box<TableJustification>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentChoice {
  /// Defines the AltChunk Class.
  #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
  WAltChunk(std::boxed::Box<AltChunk>),
  /// Defines the CustomXmlBlock Class.
  #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
  WCustomXml(std::boxed::Box<CustomXmlBlock>),
  /// Defines the SdtBlock Class.
  #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
  WSdt(std::boxed::Box<SdtBlock>),
  /// Defines the Paragraph Class.
  #[sdk(child(qname = "w:CT_P/w:p"))]
  WP(std::boxed::Box<Paragraph>),
  /// Defines the Table Class.
  #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
  WTbl(std::boxed::Box<Table>),
  /// Defines the ProofError Class.
  #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
  WProofErr(std::boxed::Box<ProofError>),
  /// Defines the PermStart Class.
  #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
  WPermStart(std::boxed::Box<PermStart>),
  /// Defines the PermEnd Class.
  #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
  WPermEnd(std::boxed::Box<PermEnd>),
  /// Defines the BookmarkStart Class.
  #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
  WBookmarkStart(std::boxed::Box<BookmarkStart>),
  /// Defines the BookmarkEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
  WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
  /// Defines the CommentRangeStart Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
  WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
  /// Defines the CommentRangeEnd Class.
  #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
  WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FootnoteChoice {
  /// Defines the AltChunk Class.
    #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndnoteChoice {
  /// Defines the AltChunk Class.
    #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DocPartBodyChoice {
  /// Defines the AltChunk Class.
    #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BodyChoice {
  /// Defines the AltChunk Class.
    #[sdk(child(qname = "w:CT_AltChunk/w:altChunk"))]
    WAltChunk(std::boxed::Box<AltChunk>),
    /// Defines the CustomXmlBlock Class.
    #[sdk(child(qname = "w:CT_CustomXmlBlock/w:customXml"))]
    WCustomXml(std::boxed::Box<CustomXmlBlock>),
    /// Defines the SdtBlock Class.
    #[sdk(child(qname = "w:CT_SdtBlock/w:sdt"))]
    WSdt(std::boxed::Box<SdtBlock>),
    /// Defines the Paragraph Class.
    #[sdk(child(qname = "w:CT_P/w:p"))]
    WP(std::boxed::Box<Paragraph>),
    /// Defines the Table Class.
    #[sdk(child(qname = "w:CT_Tbl/w:tbl"))]
    WTbl(std::boxed::Box<Table>),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(std::boxed::Box<ProofError>),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(std::boxed::Box<PermStart>),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(std::boxed::Box<PermEnd>),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(std::boxed::Box<BookmarkStart>),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(std::boxed::Box<BookmarkEnd>),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(std::boxed::Box<CommentRangeStart>),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(std::boxed::Box<CommentRangeEnd>),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(std::boxed::Box<MoveFromRangeStart>),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(std::boxed::Box<MoveFromRangeEnd>),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(std::boxed::Box<MoveToRangeStart>),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(std::boxed::Box<MoveToRangeEnd>),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(std::boxed::Box<CustomXmlInsRangeStart>),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(std::boxed::Box<CustomXmlInsRangeEnd>),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(std::boxed::Box<CustomXmlDelRangeStart>),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(std::boxed::Box<CustomXmlDelRangeEnd>),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(std::boxed::Box<CustomXmlMoveFromRangeStart>),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(std::boxed::Box<CustomXmlMoveFromRangeEnd>),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(std::boxed::Box<CustomXmlMoveToRangeStart>),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(std::boxed::Box<CustomXmlMoveToRangeEnd>),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
        >,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
        >,
    ),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(std::boxed::Box<InsertedRun>),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(std::boxed::Box<DeletedRun>),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(std::boxed::Box<MoveFromRun>),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(std::boxed::Box<MoveToRun>),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(std::boxed::Box<ContentPart>),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
            >,
        >,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<
            std::boxed::Box<
                crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
            >,
        >,
    },
    /// Unknown XML child.
    #[sdk(any)]
    XmlOther(String),
    /// Unknown XML text.
    #[sdk(text)]
    XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableRowPropertiesChoice {
  /// Defines the ConditionalFormatStyle Class.
  #[sdk(child(qname = "w:CT_Cnf/w:cnfStyle"))]
  WCnfStyle(std::boxed::Box<ConditionalFormatStyle>),
  /// Defines the DivId Class.
  #[sdk(child(qname = "w:CT_NonZeroDecimalNumber/w:divId"))]
  WDivId(std::boxed::Box<DivId>),
  /// Defines the GridBefore Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridBefore"))]
  WGridBefore(std::boxed::Box<GridBefore>),
  /// Defines the GridAfter Class.
  #[sdk(child(qname = "w:CT_DecimalNumber/w:gridAfter"))]
  WGridAfter(std::boxed::Box<GridAfter>),
  /// Defines the WidthBeforeTableRow Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:wBefore"))]
  WWBefore(std::boxed::Box<WidthBeforeTableRow>),
  /// Defines the WidthAfterTableRow Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:wAfter"))]
  WWAfter(std::boxed::Box<WidthAfterTableRow>),
  /// Defines the TableRowHeight Class.
  #[sdk(child(qname = "w:CT_Height/w:trHeight"))]
  WTrHeight(std::boxed::Box<TableRowHeight>),
  /// Defines the Hidden Class.
  #[sdk(child(qname = "w:CT_OnOff/w:hidden"))]
  WHidden(std::boxed::Box<Hidden>),
  /// Defines the CantSplit Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:cantSplit"))]
  WCantSplit(std::boxed::Box<CantSplit>),
  /// Defines the TableHeader Class.
  #[sdk(child(qname = "w:CT_OnOffOnly/w:tblHeader"))]
  WTblHeader(std::boxed::Box<TableHeader>),
  /// Defines the TableCellSpacing Class.
  #[sdk(child(qname = "w:CT_TblWidth/w:tblCellSpacing"))]
  WTblCellSpacing(std::boxed::Box<TableCellSpacing>),
  /// Defines the TableJustification Class.
  #[sdk(child(qname = "w:CT_TblJc/w:jc"))]
  WJc(std::boxed::Box<TableJustification>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableRowPropertiesChoice2 {
  /// Defines the ConflictInsertion Class.
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictIns"))]
  W14ConflictIns(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    >,
  ),
  /// Defines the ConflictDeletion Class.
  #[sdk(child(office2010, qname = "w:CT_TrackChange/w14:conflictDel"))]
  W14ConflictDel(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HeaderShapeDefaultsChoice {
  /// New Shape Defaults.
  #[sdk(child(qname = "o:CT_ShapeDefaults/o:shapedefaults"))]
  OShapedefaults(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults>,
  ),
  /// Shape Layout Properties.
  #[sdk(child(qname = "o:CT_ShapeLayout/o:shapelayout"))]
  OShapelayout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ShapeLayout>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeDefaultsChoice {
  /// New Shape Defaults.
  #[sdk(child(qname = "o:CT_ShapeDefaults/o:shapedefaults"))]
  OShapedefaults(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults>,
  ),
  /// Shape Layout Properties.
  #[sdk(child(qname = "o:CT_ShapeLayout/o:shapelayout"))]
  OShapelayout(std::boxed::Box<crate::schemas::schemas_microsoft_com_office_office::ShapeLayout>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PictureBulletBaseChoice {
  #[sdk(child(qname = "v:CT_Group/v:group"))]
  VGroup(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Group>),
  #[sdk(child(qname = "v:CT_Image/v:image"))]
  VImage(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::ImageFile>),
  #[sdk(child(qname = "v:CT_Line/v:line"))]
  VLine(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Line>),
  #[sdk(child(qname = "v:CT_Oval/v:oval"))]
  VOval(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Oval>),
  #[sdk(child(qname = "v:CT_PolyLine/v:polyline"))]
  VPolyline(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::PolyLine>),
  #[sdk(child(qname = "v:CT_Rect/v:rect"))]
  VRect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Rectangle>),
  #[sdk(child(qname = "v:CT_RoundRect/v:roundrect"))]
  VRoundrect(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::RoundRectangle>),
  #[sdk(child(qname = "v:CT_Shape/v:shape"))]
  VShape(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shape>),
  #[sdk(child(qname = "v:CT_Shapetype/v:shapetype"))]
  VShapetype(std::boxed::Box<crate::schemas::schemas_microsoft_com_vml::Shapetype>),
}
