//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionSlideDirectionValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "u")]
  Up,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "d")]
  Down,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionCornerDirectionValues {
  #[sdk(rename = "lu")]
  #[default]
  LeftUp,
  #[sdk(rename = "ru")]
  RightUp,
  #[sdk(rename = "ld")]
  LeftDown,
  #[sdk(rename = "rd")]
  RightDown,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionInOutDirectionValues {
  #[sdk(rename = "out")]
  #[default]
  Out,
  #[sdk(rename = "in")]
  In,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionSpeedValues {
  #[sdk(rename = "slow")]
  #[default]
  Slow,
  #[sdk(rename = "med")]
  Medium,
  #[sdk(rename = "fast")]
  Fast,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum IndefiniteTimeDeclarationValues {
  #[sdk(rename = "indefinite")]
  #[default]
  Indefinite,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum IterateValues {
  #[sdk(rename = "el")]
  #[default]
  Element,
  #[sdk(rename = "wd")]
  Word,
  #[sdk(rename = "lt")]
  Letter,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChartSubElementValues {
  #[sdk(rename = "gridLegend")]
  #[default]
  GridLegend,
  #[sdk(rename = "series")]
  Series,
  #[sdk(rename = "category")]
  Category,
  #[sdk(rename = "ptInSeries")]
  PointInSeries,
  #[sdk(rename = "ptInCategory")]
  PointInCategory,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TriggerRuntimeNodeValues {
  #[sdk(rename = "first")]
  #[default]
  First,
  #[sdk(rename = "last")]
  Last,
  #[sdk(rename = "all")]
  All,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodePresetClassValues {
  #[sdk(rename = "entr")]
  #[default]
  Entrance,
  #[sdk(rename = "exit")]
  Exit,
  #[sdk(rename = "emph")]
  Emphasis,
  #[sdk(rename = "path")]
  Path,
  #[sdk(rename = "verb")]
  Verb,
  #[sdk(rename = "mediacall")]
  MediaCall,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodeRestartValues {
  #[sdk(rename = "always")]
  #[default]
  Always,
  #[sdk(rename = "whenNotActive")]
  WhenNotActive,
  #[sdk(rename = "never")]
  Never,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodeFillValues {
  #[sdk(rename = "remove")]
  #[default]
  Remove,
  #[sdk(rename = "freeze")]
  Freeze,
  #[sdk(rename = "hold")]
  Hold,
  #[sdk(rename = "transition")]
  Transition,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodeValues {
  #[sdk(rename = "clickEffect")]
  #[default]
  ClickEffect,
  #[sdk(rename = "withEffect")]
  WithEffect,
  #[sdk(rename = "afterEffect")]
  AfterEffect,
  #[sdk(rename = "mainSeq")]
  MainSequence,
  #[sdk(rename = "interactiveSeq")]
  InteractiveSequence,
  #[sdk(rename = "clickPar")]
  ClickParagraph,
  #[sdk(rename = "withGroup")]
  WithGroup,
  #[sdk(rename = "afterGroup")]
  AfterGroup,
  #[sdk(rename = "tmRoot")]
  TmingRoot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NextActionValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "seek")]
  Seek,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PreviousActionValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "skipTimed")]
  SkipTimed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BehaviorAdditiveValues {
  #[sdk(rename = "base")]
  #[default]
  Base,
  #[sdk(rename = "sum")]
  Sum,
  #[sdk(rename = "repl")]
  Replace,
  #[sdk(rename = "mult")]
  Multiply,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BehaviorAccumulateValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "always")]
  Always,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BehaviorTransformValues {
  #[sdk(rename = "pt")]
  #[default]
  Point,
  #[sdk(rename = "img")]
  Image,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BehaviorOverrideValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "childStyle")]
  ChildStyle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateBehaviorCalculateModeValues {
  #[sdk(rename = "discrete")]
  #[default]
  Discrete,
  #[sdk(rename = "lin")]
  Linear,
  #[sdk(rename = "fmla")]
  Formula,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateBehaviorValues {
  #[sdk(rename = "str")]
  #[default]
  String,
  #[sdk(rename = "num")]
  Number,
  #[sdk(rename = "clr")]
  Color,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateColorSpaceValues {
  #[sdk(rename = "rgb")]
  #[default]
  Rgb,
  #[sdk(rename = "hsl")]
  Hsl,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateColorDirectionValues {
  #[sdk(rename = "cw")]
  #[default]
  Clockwise,
  #[sdk(rename = "ccw")]
  CounterClockwise,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateEffectTransitionValues {
  #[sdk(rename = "in")]
  #[default]
  In,
  #[sdk(rename = "out")]
  Out,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateMotionBehaviorOriginValues {
  #[sdk(rename = "parent")]
  #[default]
  Parent,
  #[sdk(rename = "layout")]
  Layout,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateMotionPathEditModeValues {
  #[sdk(rename = "relative")]
  #[default]
  Relative,
  #[sdk(rename = "fixed")]
  Fixed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CommandValues {
  #[sdk(rename = "evt")]
  #[default]
  Event,
  #[sdk(rename = "call")]
  Call,
  #[sdk(rename = "verb")]
  Verb,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ParagraphBuildValues {
  #[sdk(rename = "allAtOnce")]
  #[default]
  AllAtOnce,
  #[sdk(rename = "p")]
  Paragraph,
  #[sdk(rename = "cust")]
  Custom,
  #[sdk(rename = "whole")]
  Whole,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DiagramBuildValues {
  #[sdk(rename = "whole")]
  #[default]
  Whole,
  #[sdk(rename = "depthByNode")]
  DepthByNode,
  #[sdk(rename = "depthByBranch")]
  DepthByBranch,
  #[sdk(rename = "breadthByNode")]
  BreadthByNode,
  #[sdk(rename = "breadthByLvl")]
  BreadthByLevel,
  #[sdk(rename = "cw")]
  Clockwise,
  #[sdk(rename = "cwIn")]
  ClockwiseIn,
  #[sdk(rename = "cwOut")]
  ClockwiseOut,
  #[sdk(rename = "ccw")]
  CounterClockwise,
  #[sdk(rename = "ccwIn")]
  CounterClockwiseIn,
  #[sdk(rename = "ccwOut")]
  CounterClockwiseOut,
  #[sdk(rename = "inByRing")]
  InByRing,
  #[sdk(rename = "outByRing")]
  OutByRing,
  #[sdk(rename = "up")]
  Up,
  #[sdk(rename = "down")]
  Down,
  #[sdk(rename = "allAtOnce")]
  AllAtOnce,
  #[sdk(rename = "cust")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleChartBuildValues {
  #[sdk(rename = "allAtOnce")]
  #[default]
  AllAtOnce,
  #[sdk(rename = "series")]
  Series,
  #[sdk(rename = "category")]
  Category,
  #[sdk(rename = "seriesEl")]
  SeriesElement,
  #[sdk(rename = "categoryEl")]
  CategoryElement,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodeMasterRelationValues {
  #[sdk(rename = "sameClick")]
  #[default]
  SameClick,
  #[sdk(rename = "nextClick")]
  NextClick,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeNodeSyncValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "canSlip")]
  CanSlip,
  #[sdk(rename = "locked")]
  Locked,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DirectionValues {
  #[sdk(rename = "horz")]
  #[default]
  Horizontal,
  #[sdk(rename = "vert")]
  Vertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleObjectFollowColorSchemeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "full")]
  Full,
  #[sdk(rename = "textAndBackground")]
  TextAndBackground,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PhotoAlbumLayoutValues {
  #[sdk(rename = "fitToSlide")]
  #[default]
  FitToSlide,
  #[sdk(rename = "1pic")]
  OnePic,
  #[sdk(rename = "2pic")]
  TwoPic,
  #[sdk(rename = "4pic")]
  FourPic,
  #[sdk(rename = "1picTitle")]
  OnePicWithTitle,
  #[sdk(rename = "2picTitle")]
  TwoPicWithTitle,
  #[sdk(rename = "4picTitle")]
  FourPicWithTitle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PhotoAlbumFrameShapeValues {
  #[sdk(rename = "frameStyle1")]
  #[default]
  FrameStyle1,
  #[sdk(rename = "frameStyle2")]
  FrameStyle2,
  #[sdk(rename = "frameStyle3")]
  FrameStyle3,
  #[sdk(rename = "frameStyle4")]
  FrameStyle4,
  #[sdk(rename = "frameStyle5")]
  FrameStyle5,
  #[sdk(rename = "frameStyle6")]
  FrameStyle6,
  #[sdk(rename = "frameStyle7")]
  FrameStyle7,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SlideSizeValues {
  #[sdk(rename = "screen4x3")]
  #[default]
  Screen4x3,
  #[sdk(rename = "letter")]
  Letter,
  #[sdk(rename = "A4")]
  A4,
  #[sdk(rename = "35mm")]
  Film35mm,
  #[sdk(rename = "overhead")]
  Overhead,
  #[sdk(rename = "banner")]
  Banner,
  #[sdk(rename = "custom")]
  Custom,
  #[sdk(rename = "ledger")]
  Ledger,
  #[sdk(rename = "A3")]
  A3,
  #[sdk(rename = "B4ISO")]
  B4iso,
  #[sdk(rename = "B5ISO")]
  B5iso,
  #[sdk(rename = "B4JIS")]
  B4jis,
  #[sdk(rename = "B5JIS")]
  B5jis,
  #[sdk(rename = "hagakiCard")]
  HagakiCard,
  #[sdk(rename = "screen16x9")]
  Screen16x9,
  #[sdk(rename = "screen16x10")]
  Screen16x10,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptProviderValues {
  #[sdk(rename = "rsaAES")]
  #[default]
  RsaAes,
  #[sdk(rename = "rsaFull")]
  RsaFull,
  #[sdk(rename = "invalid")]
  Invalid,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptAlgorithmClassValues {
  #[sdk(rename = "hash")]
  #[default]
  Hash,
  #[sdk(rename = "invalid")]
  Invalid,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CryptAlgorithmValues {
  #[sdk(rename = "typeAny")]
  #[default]
  TypeAny,
  #[sdk(rename = "invalid")]
  Invalid,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HtmlPublishWebBrowserSupportValues {
  #[sdk(rename = "v4")]
  #[default]
  V4,
  #[sdk(rename = "v3")]
  V3,
  #[sdk(rename = "v3v4")]
  V3v4,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum WebColorValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "browser")]
  Browser,
  #[sdk(rename = "presentationText")]
  PresentationText,
  #[sdk(rename = "presentationAccent")]
  PresentationAccent,
  #[sdk(rename = "whiteTextOnBlack")]
  WhiteTextOnBlack,
  #[sdk(rename = "blackTextOnWhite")]
  BlackTextOnWhite,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum WebScreenSizeValues {
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
  #[sdk(rename = "1800x1400")]
  Sz1800x1400,
  #[sdk(rename = "1920x1200")]
  Sz1920x1200,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PrintOutputValues {
  #[sdk(rename = "slides")]
  #[default]
  Slides,
  #[sdk(rename = "handouts1")]
  Handouts1,
  #[sdk(rename = "handouts2")]
  Handouts2,
  #[sdk(rename = "handouts3")]
  Handouts3,
  #[sdk(rename = "handouts4")]
  Handouts4,
  #[sdk(rename = "handouts6")]
  Handouts6,
  #[sdk(rename = "handouts9")]
  Handouts9,
  #[sdk(rename = "notes")]
  Notes,
  #[sdk(rename = "outline")]
  Outline,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PrintColorModeValues {
  #[sdk(rename = "bw")]
  #[default]
  BlackWhite,
  #[sdk(rename = "gray")]
  Gray,
  #[sdk(rename = "clr")]
  Color,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PlaceholderValues {
  #[sdk(rename = "title")]
  #[default]
  Title,
  #[sdk(rename = "body")]
  Body,
  #[sdk(rename = "ctrTitle")]
  CenteredTitle,
  #[sdk(rename = "subTitle")]
  SubTitle,
  #[sdk(rename = "dt")]
  DateAndTime,
  #[sdk(rename = "sldNum")]
  SlideNumber,
  #[sdk(rename = "ftr")]
  Footer,
  #[sdk(rename = "hdr")]
  Header,
  #[sdk(rename = "obj")]
  Object,
  #[sdk(rename = "chart")]
  Chart,
  #[sdk(rename = "tbl")]
  Table,
  #[sdk(rename = "clipArt")]
  ClipArt,
  #[sdk(rename = "dgm")]
  Diagram,
  #[sdk(rename = "media")]
  Media,
  #[sdk(rename = "sldImg")]
  SlideImage,
  #[sdk(rename = "pic")]
  Picture,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PlaceholderSizeValues {
  #[sdk(rename = "full")]
  #[default]
  Full,
  #[sdk(rename = "half")]
  Half,
  #[sdk(rename = "quarter")]
  Quarter,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SlideLayoutValues {
  #[sdk(rename = "title")]
  #[default]
  Title,
  #[sdk(rename = "tx")]
  Text,
  #[sdk(rename = "twoColTx")]
  TwoColumnText,
  #[sdk(rename = "tbl")]
  Table,
  #[sdk(rename = "txAndChart")]
  TextAndChart,
  #[sdk(rename = "chartAndTx")]
  ChartAndText,
  #[sdk(rename = "dgm")]
  Diagram,
  #[sdk(rename = "chart")]
  Chart,
  #[sdk(rename = "txAndClipArt")]
  TextAndClipArt,
  #[sdk(rename = "clipArtAndTx")]
  ClipArtAndText,
  #[sdk(rename = "titleOnly")]
  TitleOnly,
  #[sdk(rename = "blank")]
  Blank,
  #[sdk(rename = "txAndObj")]
  TextAndObject,
  #[sdk(rename = "objAndTx")]
  ObjectAndText,
  #[sdk(rename = "objOnly")]
  ObjectOnly,
  #[sdk(rename = "obj")]
  Object,
  #[sdk(rename = "txAndMedia")]
  TextAndMedia,
  #[sdk(rename = "mediaAndTx")]
  MidiaAndText,
  #[sdk(rename = "objOverTx")]
  ObjectOverText,
  #[sdk(rename = "txOverObj")]
  TextOverObject,
  #[sdk(rename = "txAndTwoObj")]
  TextAndTwoObjects,
  #[sdk(rename = "twoObjAndTx")]
  TwoObjectsAndText,
  #[sdk(rename = "twoObjOverTx")]
  TwoObjectsOverText,
  #[sdk(rename = "fourObj")]
  FourObjects,
  #[sdk(rename = "vertTx")]
  VerticalText,
  #[sdk(rename = "clipArtAndVertTx")]
  ClipArtAndVerticalText,
  #[sdk(rename = "vertTitleAndTx")]
  VerticalTitleAndText,
  #[sdk(rename = "vertTitleAndTxOverChart")]
  VerticalTitleAndTextOverChart,
  #[sdk(rename = "twoObj")]
  TwoObjects,
  #[sdk(rename = "objAndTwoObj")]
  ObjectAndTwoObjects,
  #[sdk(rename = "twoObjAndObj")]
  TwoObjectsAndObject,
  #[sdk(rename = "cust")]
  Custom,
  #[sdk(rename = "secHead")]
  SectionHeader,
  #[sdk(rename = "twoTxTwoObj")]
  TwoTextAndTwoObjects,
  #[sdk(rename = "objTx")]
  ObjectText,
  #[sdk(rename = "picTx")]
  PictureText,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SplitterBarStateValues {
  #[sdk(rename = "minimized")]
  #[default]
  Minimized,
  #[sdk(rename = "restored")]
  Restored,
  #[sdk(rename = "maximized")]
  Maximized,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ViewValues {
  #[sdk(rename = "sldView")]
  #[default]
  SlideView,
  #[sdk(rename = "sldMasterView")]
  SlideMasterView,
  #[sdk(rename = "notesView")]
  NotesView,
  #[sdk(rename = "handoutView")]
  HandoutView,
  #[sdk(rename = "notesMasterView")]
  NotesMasterView,
  #[sdk(rename = "outlineView")]
  OutlineView,
  #[sdk(rename = "sldSorterView")]
  SlideSorterView,
  #[sdk(rename = "sldThumbnailView")]
  SlideThumbnailView,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TriggerEventValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "onBegin")]
  OnBegin,
  #[sdk(rename = "onEnd")]
  OnEnd,
  #[sdk(rename = "begin")]
  Begin,
  #[sdk(rename = "end")]
  End,
  #[sdk(rename = "onClick")]
  OnClick,
  #[sdk(rename = "onDblClick")]
  OnDoubleClick,
  #[sdk(rename = "onMouseOver")]
  OnMouseOver,
  #[sdk(rename = "onMouseOut")]
  OnMouseOut,
  #[sdk(rename = "onNext")]
  OnNext,
  #[sdk(rename = "onPrev")]
  OnPrevious,
  #[sdk(rename = "onStopAudio")]
  OnStopAudio,
  #[sdk(rename = "onMediaBookmark")]
  OnMediaBookmark,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConformanceClassValues {
  #[sdk(rename = "strict")]
  #[default]
  Strict,
  #[sdk(rename = "transitional")]
  Transitional,
}
/// Slide Range.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:sldRg")]
pub struct SlideRange {
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Character Range.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:charRg")]
pub struct CharRange {
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Paragraph Text Range.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:pRg")]
pub struct ParagraphIndexRange {
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Custom Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShowId/p:custShow")]
pub struct CustomShowReference {
  /// Custom Show Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Extension/p:ext")]
pub struct Extension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Browse Slide Show Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowInfoBrowse/p:browse")]
pub struct BrowseSlideMode {
  /// Show Scroll Bar in Window
  #[sdk(attr(qname = ":showScrollbar"))]
  pub show_scrollbar: Option<crate::simple_type::BooleanValue>,
}
/// Kiosk Slide Show Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowInfoKiosk/p:kiosk")]
pub struct KioskSlideMode {
  /// Restart Show
  #[sdk(attr(qname = ":restart"))]
  pub restart: Option<crate::simple_type::UInt32Value>,
}
/// Color Scheme Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/p:clrMap")]
pub struct ColorMap {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(kind = "token"))]
  pub background1: crate::schemas::a::ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(kind = "token"))]
  pub text1: crate::schemas::a::ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(kind = "token"))]
  pub background2: crate::schemas::a::ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(kind = "token"))]
  pub text2: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(kind = "token"))]
  pub accent1: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(kind = "token"))]
  pub accent2: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(kind = "token"))]
  pub accent3: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(kind = "token"))]
  pub accent4: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(kind = "token"))]
  pub accent5: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(kind = "token"))]
  pub accent6: crate::schemas::a::ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(kind = "token"))]
  pub hyperlink: crate::schemas::a::ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(kind = "token"))]
  pub followed_hyperlink: crate::schemas::a::ColorSchemeIndexValues,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Color Scheme Map Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMappingOverride/p:clrMapOvr")]
pub struct ColorMapOverride {
  #[sdk(choice(
    qname = "a:CT_EmptyElement/a:masterClrMapping",
    qname = "a:CT_ColorMapping/a:overrideClrMapping"
  ))]
  pub color_map_override_choice: Option<ColorMapOverrideChoice>,
}
/// Background Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_BackgroundProperties/p:bgPr")]
pub struct BackgroundProperties {
  /// Shade to Title
  #[sdk(attr(qname = ":shadeToTitle"))]
  pub shade_to_title: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub background_properties_choice1: Option<BackgroundPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub background_properties_choice2: Option<BackgroundPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ExtensionList>,
}
/// Background Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/p:bgRef")]
pub struct BackgroundStyleReference {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub background_style_reference_choice: Option<BackgroundStyleReferenceChoice>,
}
/// Data for the Windows platform..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p188:CT_CommentPropertiesExtension/p:ext")]
pub struct CommentPropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the TaskDetails Class.
  #[sdk(child(microsoft365, qname = "p228:CT_TaskDetails/p228:taskDetails"))]
  pub task_details: Option<std::boxed::Box<crate::schemas::p228::TaskDetails>>,
  /// Defines the Reactions Class.
  #[sdk(child(microsoft365, qname = "p223:CT_Reactions/p223:reactions"))]
  pub reactions: Option<crate::schemas::p223::Reactions>,
}
/// List of Comment Authors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorList/p:cmAuthorLst")]
pub struct CommentAuthorList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Comment Author.
  #[sdk(child(qname = "p:CT_CommentAuthor/p:cmAuthor"))]
  pub p_cm_author: Vec<CommentAuthor>,
}
/// Comment List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentList/p:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Comment.
  #[sdk(child(qname = "p:CT_Comment/p:cm"))]
  pub p_cm: Vec<Comment>,
}
/// Global Element for OLE Objects and Controls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObject/p:oleObj")]
pub struct OleObject {
  /// spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// showAsIcon
  #[sdk(attr(qname = ":showAsIcon"))]
  pub show_as_icon: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// imgW
  #[sdk(attr(qname = ":imgW"))]
  #[sdk(number_range(range = 0..))]
  pub image_width: Option<crate::simple_type::Int32Value>,
  /// imgH
  #[sdk(attr(qname = ":imgH"))]
  #[sdk(number_range(range = 0..))]
  pub image_height: Option<crate::simple_type::Int32Value>,
  /// progId
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "p:CT_OleObjectEmbed/p:embed",
    qname = "p:CT_OleObjectLink/p:link"
  ))]
  pub ole_object_choice: Option<OleObjectChoice>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  pub p_pic: Option<std::boxed::Box<Picture>>,
}
/// Presentation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Presentation/p:presentation")]
pub struct Presentation {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// serverZoom
  #[sdk(attr(qname = ":serverZoom"))]
  pub server_zoom: Option<crate::simple_type::Int32Value>,
  /// firstSlideNum
  #[sdk(attr(qname = ":firstSlideNum"))]
  pub first_slide_num: Option<crate::simple_type::Int32Value>,
  /// showSpecialPlsOnTitleSld
  #[sdk(attr(qname = ":showSpecialPlsOnTitleSld"))]
  pub show_special_placeholder_on_title_slide: Option<crate::simple_type::BooleanValue>,
  /// rtl
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// removePersonalInfoOnSave
  #[sdk(attr(qname = ":removePersonalInfoOnSave"))]
  pub remove_personal_info_on_save: Option<crate::simple_type::BooleanValue>,
  /// compatMode
  #[sdk(attr(qname = ":compatMode"))]
  pub compatibility_mode: Option<crate::simple_type::BooleanValue>,
  /// strictFirstAndLastChars
  #[sdk(attr(qname = ":strictFirstAndLastChars"))]
  pub strict_first_and_last_chars: Option<crate::simple_type::BooleanValue>,
  /// embedTrueTypeFonts
  #[sdk(attr(qname = ":embedTrueTypeFonts"))]
  pub embed_true_type_fonts: Option<crate::simple_type::BooleanValue>,
  /// saveSubsetFonts
  #[sdk(attr(qname = ":saveSubsetFonts"))]
  pub save_subset_fonts: Option<crate::simple_type::BooleanValue>,
  /// autoCompressPictures
  #[sdk(attr(qname = ":autoCompressPictures"))]
  pub auto_compress_pictures: Option<crate::simple_type::BooleanValue>,
  /// bookmarkIdSeed
  #[sdk(attr(qname = ":bookmarkIdSeed"))]
  #[sdk(number_range(range = 1..2147483648))]
  pub bookmark_id_seed: Option<crate::simple_type::UInt32Value>,
  /// conformance
  #[sdk(attr(qname = ":conformance"))]
  pub conformance: Option<ConformanceClassValues>,
  /// Defines the SlideMasterIdList Class.
  #[sdk(child(qname = "p:CT_SlideMasterIdList/p:sldMasterIdLst"))]
  pub slide_master_id_list: Option<SlideMasterIdList>,
  /// Defines the NotesMasterIdList Class.
  #[sdk(child(qname = "p:CT_NotesMasterIdList/p:notesMasterIdLst"))]
  pub notes_master_id_list: Option<std::boxed::Box<NotesMasterIdList>>,
  /// Defines the HandoutMasterIdList Class.
  #[sdk(child(qname = "p:CT_HandoutMasterIdList/p:handoutMasterIdLst"))]
  pub handout_master_id_list: Option<std::boxed::Box<HandoutMasterIdList>>,
  /// Defines the SlideIdList Class.
  #[sdk(child(qname = "p:CT_SlideIdList/p:sldIdLst"))]
  pub slide_id_list: Option<SlideIdList>,
  /// Defines the SlideSize Class.
  #[sdk(child(qname = "p:CT_SlideSize/p:sldSz"))]
  pub slide_size: Option<SlideSize>,
  /// Defines the NotesSize Class.
  #[sdk(child(qname = "a:CT_PositiveSize2D/p:notesSz"))]
  pub notes_size: std::boxed::Box<NotesSize>,
  /// Defines the EmbeddedFontList Class.
  #[sdk(child(qname = "p:CT_EmbeddedFontList/p:embeddedFontLst"))]
  pub embedded_font_list: Option<EmbeddedFontList>,
  /// Defines the CustomShowList Class.
  #[sdk(child(qname = "p:CT_CustomShowList/p:custShowLst"))]
  pub custom_show_list: Option<CustomShowList>,
  /// Defines the PhotoAlbum Class.
  #[sdk(child(qname = "p:CT_PhotoAlbum/p:photoAlbum"))]
  pub photo_album: Option<std::boxed::Box<PhotoAlbum>>,
  /// Customer Data List.
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "p:CT_Kinsoku/p:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the DefaultTextStyle Class.
  #[sdk(child(qname = "a:CT_TextListStyle/p:defaultTextStyle"))]
  pub default_text_style: Option<std::boxed::Box<DefaultTextStyle>>,
  /// Defines the ModificationVerifier Class.
  #[sdk(child(qname = "p:CT_ModifyVerifier/p:modifyVerifier"))]
  pub modification_verifier: Option<ModificationVerifier>,
  /// Defines the PresentationExtensionList Class.
  #[sdk(child(qname = "p:CT_PresentationExtensionList/p:extLst"))]
  pub presentation_extension_list: Option<PresentationExtensionList>,
}
/// Presentation-wide Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationProperties/p:presentationPr")]
pub struct PresentationProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// HTML Publishing Properties
  #[sdk(child(qname = "p:CT_HtmlPublishProperties/p:htmlPubPr"))]
  pub html_publish_properties: Option<std::boxed::Box<HtmlPublishProperties>>,
  /// Web Properties
  #[sdk(child(qname = "p:CT_WebProperties/p:webPr"))]
  pub web_properties: Option<std::boxed::Box<WebProperties>>,
  /// Defines the PrintingProperties Class.
  #[sdk(child(qname = "p:CT_PrintProperties/p:prnPr"))]
  pub printing_properties: Option<std::boxed::Box<PrintingProperties>>,
  /// Defines the ShowProperties Class.
  #[sdk(child(qname = "p:CT_ShowProperties/p:showPr"))]
  pub show_properties: Option<std::boxed::Box<ShowProperties>>,
  /// Defines the ColorMostRecentlyUsed Class.
  #[sdk(child(qname = "a:CT_ColorMRU/p:clrMru"))]
  pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
  /// Defines the PresentationPropertiesExtensionList Class.
  #[sdk(child(qname = "p:CT_PresentationPropertiesExtensionList/p:extLst"))]
  pub presentation_properties_extension_list: Option<PresentationPropertiesExtensionList>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Slide/p:sld")]
pub struct Slide {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Show Master Shapes
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// Show Slide in Slide Show
  #[sdk(attr(qname = ":show"))]
  pub show: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for slides
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Slide Transition
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the SlideExtensionList Class.
  #[sdk(child(qname = "p:CT_SlideExtensionList/p:extLst"))]
  pub slide_extension_list: Option<SlideExtensionList>,
}
/// Slide Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayout/p:sldLayout")]
pub struct SlideLayout {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Show Master Shapes
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// matchingName
  #[sdk(attr(qname = ":matchingName"))]
  pub matching_name: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<SlideLayoutValues>,
  /// preserve
  #[sdk(attr(qname = ":preserve"))]
  pub preserve: Option<crate::simple_type::BooleanValue>,
  /// userDrawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Slide Transition.
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide.
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the SlideLayoutExtensionList Class.
  #[sdk(child(qname = "p:CT_SlideLayoutExtensionList/p:extLst"))]
  pub slide_layout_extension_list: Option<SlideLayoutExtensionList>,
}
/// Slide Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMaster/p:sldMaster")]
pub struct SlideMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// preserve
  #[sdk(attr(qname = ":preserve"))]
  pub preserve: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the SlideLayoutIdList Class.
  #[sdk(child(qname = "p:CT_SlideLayoutIdList/p:sldLayoutIdLst"))]
  pub slide_layout_id_list: Option<SlideLayoutIdList>,
  /// Slide Transition.
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide.
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the TextStyles Class.
  #[sdk(child(qname = "p:CT_SlideMasterTextStyles/p:txStyles"))]
  pub text_styles: Option<std::boxed::Box<TextStyles>>,
  /// Defines the SlideMasterExtensionList Class.
  #[sdk(child(qname = "p:CT_SlideMasterExtensionList/p:extLst"))]
  pub slide_master_extension_list: Option<SlideMasterExtensionList>,
}
/// Handout Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMaster/p:handoutMaster")]
pub struct HandoutMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the HandoutMasterExtensionList Class.
  #[sdk(child(qname = "p:CT_HandoutMasterExtensionList/p:extLst"))]
  pub handout_master_extension_list: Option<HandoutMasterExtensionList>,
}
/// Notes Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMaster/p:notesMaster")]
pub struct NotesMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the NotesStyle Class.
  #[sdk(child(qname = "a:CT_TextListStyle/p:notesStyle"))]
  pub notes_style: Option<std::boxed::Box<NotesStyle>>,
  /// Defines the NotesMasterExtensionList Class.
  #[sdk(child(qname = "p:CT_NotesMasterExtensionList/p:extLst"))]
  pub notes_master_extension_list: Option<NotesMasterExtensionList>,
}
/// Notes Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesSlide/p:notes")]
pub struct NotesSlide {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Show Master Shapes
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for notes slides
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Slide Synchronization Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSyncProperties/p:sldSyncPr")]
pub struct SlideSyncProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Server's Slide File ID
  #[sdk(attr(qname = ":serverSldId"))]
  pub server_slide_id: crate::simple_type::StringValue,
  /// Server's Slide File's modification date/time
  #[sdk(attr(qname = ":serverSldModifiedTime"))]
  pub server_slide_modified_time: crate::simple_type::DateTimeValue,
  /// Client Slide Insertion date/time
  #[sdk(attr(qname = ":clientInsertedTime"))]
  pub client_inserted_time: crate::simple_type::DateTimeValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Programmable Tab List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TagList/p:tagLst")]
pub struct TagList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Programmable Extensibility Tag.
  #[sdk(child(qname = "p:CT_StringTag/p:tag"))]
  pub p_tag: Vec<Tag>,
}
/// Presentation-wide View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ViewProperties/p:viewPr")]
pub struct ViewProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Last View
  #[sdk(attr(qname = ":lastView"))]
  #[sdk(string_format(kind = "token"))]
  pub last_view: Option<ViewValues>,
  /// Show Comments
  #[sdk(attr(qname = ":showComments"))]
  pub show_comments: Option<crate::simple_type::BooleanValue>,
  /// Normal View Properties
  #[sdk(child(qname = "p:CT_NormalViewProperties/p:normalViewPr"))]
  pub normal_view_properties: Option<std::boxed::Box<NormalViewProperties>>,
  /// Slide View Properties
  #[sdk(child(qname = "p:CT_SlideViewProperties/p:slideViewPr"))]
  pub slide_view_properties: Option<std::boxed::Box<SlideViewProperties>>,
  /// Outline View Properties
  #[sdk(child(qname = "p:CT_OutlineViewProperties/p:outlineViewPr"))]
  pub outline_view_properties: Option<std::boxed::Box<OutlineViewProperties>>,
  /// Notes Text View Properties
  #[sdk(child(qname = "p:CT_NotesTextViewProperties/p:notesTextViewPr"))]
  pub notes_text_view_properties: Option<std::boxed::Box<NotesTextViewProperties>>,
  /// Slide Sorter View Properties
  #[sdk(child(qname = "p:CT_SlideSorterViewProperties/p:sorterViewPr"))]
  pub sorter_view_properties: Option<std::boxed::Box<SorterViewProperties>>,
  /// Notes View Properties
  #[sdk(child(qname = "p:CT_NotesViewProperties/p:notesViewPr"))]
  pub notes_view_properties: Option<std::boxed::Box<NotesViewProperties>>,
  /// Grid Spacing
  #[sdk(child(qname = "a:CT_PositiveSize2D/p:gridSpacing"))]
  pub grid_spacing: Option<GridSpacing>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_ContentPart/p:contentPart")]
pub struct ContentPart {
  /// bwMode
  #[sdk(attr(office2010, qname = "p14:bwMode"))]
  pub p14_bw_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// Defines the NonVisualContentPartProperties Class.
  #[sdk(child(office2010, qname = "p14:CT_ContentPartNonVisual/p14:nvContentPartPr"))]
  pub non_visual_content_part_properties:
    Option<std::boxed::Box<crate::schemas::p14::NonVisualContentPartProperties>>,
  /// Defines the Transform2D Class.
  #[sdk(child(office2010, qname = "a:CT_Transform2D/p14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::p14::Transform2D>>,
  /// Defines the ExtensionListModify Class.
  #[sdk(child(office2010, qname = "p:CT_ExtensionListModify/p14:extLst"))]
  pub extension_list_modify: Option<crate::schemas::p14::ExtensionListModify>,
}
/// Sound.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/p:snd")]
pub struct Sound {
  /// Embedded Audio File Relationship ID
  #[sdk(attr(qname = "r:embed"))]
  pub embed: crate::simple_type::StringValue,
  /// Sound Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Recognized Built-In Sound
  #[sdk(attr(qname = ":builtIn"))]
  pub built_in: Option<crate::simple_type::BooleanValue>,
}
/// Sound Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt")]
pub struct SoundTarget {
  /// Embedded Audio File Relationship ID
  #[sdk(attr(qname = "r:embed"))]
  pub embed: crate::simple_type::StringValue,
  /// Sound Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Recognized Built-In Sound
  #[sdk(attr(qname = ":builtIn"))]
  pub built_in: Option<crate::simple_type::BooleanValue>,
}
/// Start Sound Action.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TransitionStartSoundAction/p:stSnd")]
pub struct StartSoundAction {
  /// Loop Sound
  #[sdk(attr(qname = ":loop"))]
  pub r#loop: Option<crate::simple_type::BooleanValue>,
  /// Sound
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/p:snd"))]
  pub sound: std::boxed::Box<Sound>,
}
/// Time Absolute.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateIntervalTime/p:tmAbs")]
pub struct TimeAbsolute {
  /// Time
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 3u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 4u32, union = 1u64, values = &["indefinite"]))]
  pub val: crate::simple_type::StringValue,
}
/// Time Percentage.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateIntervalPercentage/p:tmPct")]
pub struct TimePercentage {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::Int32Value,
}
/// Target Element Trigger Choice.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeTargetElement/p:tgtEl")]
pub struct TargetElement {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:sldTgt",
    qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt",
    qname = "p:CT_TLShapeTargetElement/p:spTgt",
    qname = "p:CT_TLSubShapeId/p:inkTgt",
    qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt"
  ))]
  pub target_element_choice: Option<TargetElementChoice>,
}
/// Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTriggerTimeNodeID/p:tn")]
pub struct TimeNode {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Runtime Node Trigger Choice.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTriggerRuntimeNode/p:rtn")]
pub struct RuntimeNodeTrigger {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: TriggerRuntimeNodeValues,
}
/// Condition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeCondition/p:cond")]
pub struct Condition {
  /// Trigger Event
  #[sdk(attr(qname = ":evt"))]
  #[sdk(string_format(kind = "token"))]
  pub event: Option<TriggerEventValues>,
  /// Trigger Delay
  #[sdk(attr(qname = ":delay"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 2u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 3u32, union = 1u64, values = &["indefinite"]))]
  pub delay: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "p:CT_TLTimeTargetElement/p:tgtEl",
    qname = "p:CT_TLTriggerTimeNodeID/p:tn",
    qname = "p:CT_TLTriggerRuntimeNode/p:rtn"
  ))]
  pub condition_choice: Option<ConditionChoice>,
}
/// Defines the EndSync Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeCondition/p:endSync")]
pub struct EndSync {
  /// Trigger Event
  #[sdk(attr(qname = ":evt"))]
  #[sdk(string_format(kind = "token"))]
  pub event: Option<TriggerEventValues>,
  /// Trigger Delay
  #[sdk(attr(qname = ":delay"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 2u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 3u32, union = 1u64, values = &["indefinite"]))]
  pub delay: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "p:CT_TLTimeTargetElement/p:tgtEl",
    qname = "p:CT_TLTriggerTimeNodeID/p:tn",
    qname = "p:CT_TLTriggerRuntimeNode/p:rtn"
  ))]
  pub end_sync_choice: Option<EndSyncChoice>,
}
/// Parallel Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeParallel/p:par")]
pub struct ParallelTimeNode {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Parallel TimeNode
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Sequence Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeSequence/p:seq")]
pub struct SequenceTimeNode {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Concurrent
  #[sdk(attr(qname = ":concurrent"))]
  pub concurrent: Option<crate::simple_type::BooleanValue>,
  /// Previous Action
  #[sdk(attr(qname = ":prevAc"))]
  #[sdk(string_format(kind = "token"))]
  pub previous_action: Option<PreviousActionValues>,
  /// Next Action
  #[sdk(attr(qname = ":nextAc"))]
  #[sdk(string_format(kind = "token"))]
  pub next_action: Option<NextActionValues>,
  /// Common TimeNode Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Previous Conditions List
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:prevCondLst"))]
  pub previous_condition_list: Option<PreviousConditionList>,
  /// Next Conditions List
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:nextCondLst"))]
  pub next_condition_list: Option<NextConditionList>,
}
/// Exclusive.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeExclusive/p:excl")]
pub struct ExclusiveTimeNode {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Common TimeNode Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateBehavior/p:anim")]
pub struct Animate {
  pub xml_other_attrs: Vec<(String, String)>,
  /// by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::StringValue>,
  /// from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  /// calcmode
  #[sdk(attr(qname = ":calcmode"))]
  #[sdk(string_format(kind = "token"))]
  pub calculation_mode: Option<AnimateBehaviorCalculateModeValues>,
  /// valueType
  #[sdk(attr(qname = ":valueType"))]
  #[sdk(string_format(kind = "token"))]
  pub value_type: Option<AnimateBehaviorValues>,
  /// bounceEnd
  #[sdk(attr(office2010, qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the TimeAnimateValueList Class.
  #[sdk(child(qname = "p:CT_TLTimeAnimateValueList/p:tavLst"))]
  pub time_animate_value_list: Option<TimeAnimateValueList>,
}
/// Animate Color Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateColorBehavior/p:animClr")]
pub struct AnimateColor {
  /// Color Space
  #[sdk(attr(qname = ":clrSpc"))]
  #[sdk(string_format(kind = "token"))]
  pub color_space: Option<AnimateColorSpaceValues>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<AnimateColorDirectionValues>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// By
  #[sdk(child(qname = "p:CT_TLByAnimateColorTransform/p:by"))]
  pub by_color: Option<std::boxed::Box<ByColor>>,
  /// From
  #[sdk(child(qname = "a:CT_Color3/p:from"))]
  pub from_color: Option<std::boxed::Box<FromColor>>,
  /// To
  #[sdk(child(qname = "a:CT_Color3/p:to"))]
  pub to_color: Option<std::boxed::Box<ToColor>>,
}
/// Animate Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect")]
pub struct AnimateEffect {
  /// Transition
  #[sdk(attr(qname = ":transition"))]
  #[sdk(string_format(kind = "token"))]
  pub transition: Option<AnimateEffectTransitionValues>,
  /// Filter
  #[sdk(attr(qname = ":filter"))]
  pub filter: Option<crate::simple_type::StringValue>,
  /// Property List
  #[sdk(attr(qname = ":prLst"))]
  pub property_list: Option<crate::simple_type::StringValue>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Progress
  #[sdk(child(qname = "p:CT_TLAnimFloat/p:progress"))]
  pub progress: Option<std::boxed::Box<Progress>>,
}
/// Animate Motion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion")]
pub struct AnimateMotion {
  pub xml_other_attrs: Vec<(String, String)>,
  /// origin
  #[sdk(attr(qname = ":origin"))]
  #[sdk(string_format(kind = "token"))]
  pub origin: Option<AnimateMotionBehaviorOriginValues>,
  /// path
  #[sdk(attr(qname = ":path"))]
  pub path: Option<crate::simple_type::StringValue>,
  /// pathEditMode
  #[sdk(attr(qname = ":pathEditMode"))]
  #[sdk(string_format(kind = "token"))]
  pub path_edit_mode: Option<AnimateMotionPathEditModeValues>,
  /// rAng
  #[sdk(attr(qname = ":rAng"))]
  pub relative_angle: Option<crate::simple_type::Int32Value>,
  /// ptsTypes
  #[sdk(attr(qname = ":ptsTypes"))]
  pub point_types: Option<crate::simple_type::StringValue>,
  /// bounceEnd
  #[sdk(attr(office2010, qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the ByPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:by"))]
  pub by_position: Option<ByPosition>,
  /// Defines the FromPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:from"))]
  pub from_position: Option<FromPosition>,
  /// Defines the ToPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:to"))]
  pub to_position: Option<ToPosition>,
  /// Defines the RotationCenter Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:rCtr"))]
  pub rotation_center: Option<RotationCenter>,
}
/// Animate Rotation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateRotationBehavior/p:animRot")]
pub struct AnimateRotation {
  pub xml_other_attrs: Vec<(String, String)>,
  /// by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::Int32Value>,
  /// from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::Int32Value>,
  /// to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::Int32Value>,
  /// bounceEnd
  #[sdk(attr(office2010, qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Animate Scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateScaleBehavior/p:animScale")]
pub struct AnimateScale {
  pub xml_other_attrs: Vec<(String, String)>,
  /// zoomContents
  #[sdk(attr(qname = ":zoomContents"))]
  pub zoom_contents: Option<crate::simple_type::BooleanValue>,
  /// bounceEnd
  #[sdk(attr(office2010, qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the ByPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:by"))]
  pub by_position: Option<ByPosition>,
  /// Defines the FromPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:from"))]
  pub from_position: Option<FromPosition>,
  /// Defines the ToPosition Class.
  #[sdk(child(qname = "p:CT_TLPoint/p:to"))]
  pub to_position: Option<ToPosition>,
}
/// Command.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommandBehavior/p:cmd")]
pub struct Command {
  /// Command Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<CommandValues>,
  /// Command
  #[sdk(attr(qname = ":cmd"))]
  pub command_name: Option<crate::simple_type::StringValue>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Set Time Node Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSetBehavior/p:set")]
pub struct SetBehavior {
  /// Common Behavior
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// To
  #[sdk(child(qname = "p:CT_TLAnimVariant/p:to"))]
  pub to_variant_value: Option<std::boxed::Box<ToVariantValue>>,
}
/// Audio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLMediaNodeAudio/p:audio")]
pub struct Audio {
  /// Is Narration
  #[sdk(attr(qname = ":isNarration"))]
  pub is_narration: Option<crate::simple_type::BooleanValue>,
  /// Common Media Node Properties
  #[sdk(child(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Video.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLMediaNodeVideo/p:video")]
pub struct Video {
  /// Full Screen
  #[sdk(attr(qname = ":fullScrn"))]
  pub full_screen: Option<crate::simple_type::BooleanValue>,
  /// Common Media Node Properties
  #[sdk(child(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Parallel TimeNode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonTimeNodeData/p:cTn")]
pub struct CommonTimeNode {
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// presetID
  #[sdk(attr(qname = ":presetID"))]
  pub preset_id: Option<crate::simple_type::Int32Value>,
  /// presetClass
  #[sdk(attr(qname = ":presetClass"))]
  #[sdk(string_format(kind = "token"))]
  pub preset_class: Option<TimeNodePresetClassValues>,
  /// presetSubtype
  #[sdk(attr(qname = ":presetSubtype"))]
  pub preset_subtype: Option<crate::simple_type::Int32Value>,
  /// dur
  #[sdk(attr(qname = ":dur"))]
  pub duration: Option<crate::simple_type::StringValue>,
  /// repeatCount
  #[sdk(attr(qname = ":repeatCount"))]
  pub repeat_count: Option<crate::simple_type::StringValue>,
  /// repeatDur
  #[sdk(attr(qname = ":repeatDur"))]
  pub repeat_duration: Option<crate::simple_type::StringValue>,
  /// spd
  #[sdk(attr(qname = ":spd"))]
  pub speed: Option<crate::simple_type::Int32Value>,
  /// accel
  #[sdk(attr(qname = ":accel"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub acceleration: Option<crate::simple_type::Int32Value>,
  /// decel
  #[sdk(attr(qname = ":decel"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub deceleration: Option<crate::simple_type::Int32Value>,
  /// autoRev
  #[sdk(attr(qname = ":autoRev"))]
  pub auto_reverse: Option<crate::simple_type::BooleanValue>,
  /// restart
  #[sdk(attr(qname = ":restart"))]
  #[sdk(string_format(kind = "token"))]
  pub restart: Option<TimeNodeRestartValues>,
  /// fill
  #[sdk(attr(qname = ":fill"))]
  #[sdk(string_format(kind = "token"))]
  pub fill: Option<TimeNodeFillValues>,
  /// syncBehavior
  #[sdk(attr(qname = ":syncBehavior"))]
  #[sdk(string_format(kind = "token"))]
  pub sync_behavior: Option<TimeNodeSyncValues>,
  /// tmFilter
  #[sdk(attr(qname = ":tmFilter"))]
  pub time_filter: Option<crate::simple_type::StringValue>,
  /// evtFilter
  #[sdk(attr(qname = ":evtFilter"))]
  pub event_filter: Option<crate::simple_type::StringValue>,
  /// display
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::BooleanValue>,
  /// masterRel
  #[sdk(attr(qname = ":masterRel"))]
  #[sdk(string_format(kind = "token"))]
  pub master_relation: Option<TimeNodeMasterRelationValues>,
  /// bldLvl
  #[sdk(attr(qname = ":bldLvl"))]
  pub build_level: Option<crate::simple_type::Int32Value>,
  /// grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: Option<crate::simple_type::UInt32Value>,
  /// afterEffect
  #[sdk(attr(qname = ":afterEffect"))]
  pub after_effect: Option<crate::simple_type::BooleanValue>,
  /// nodeType
  #[sdk(attr(qname = ":nodeType"))]
  #[sdk(string_format(kind = "token"))]
  pub node_type: Option<TimeNodeValues>,
  /// nodePh
  #[sdk(attr(qname = ":nodePh"))]
  pub node_placeholder: Option<crate::simple_type::BooleanValue>,
  /// presetBounceEnd
  #[sdk(attr(office2010, qname = "p14:presetBounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub preset_bounce_end: Option<crate::simple_type::Int32Value>,
  /// Defines the StartConditionList Class.
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:stCondLst"))]
  pub start_condition_list: Option<StartConditionList>,
  /// Defines the EndConditionList Class.
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:endCondLst"))]
  pub end_condition_list: Option<EndConditionList>,
  /// Defines the EndSync Class.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:endSync"))]
  pub end_sync: Option<std::boxed::Box<EndSync>>,
  /// Defines the Iterate Class.
  #[sdk(child(qname = "p:CT_TLIterateData/p:iterate"))]
  pub iterate: Option<std::boxed::Box<Iterate>>,
  /// Defines the ChildTimeNodeList Class.
  #[sdk(child(qname = "p:CT_TimeNodeList/p:childTnLst"))]
  pub child_time_node_list: Option<ChildTimeNodeList>,
  /// Defines the SubTimeNodeList Class.
  #[sdk(child(qname = "p:CT_TimeNodeList/p:subTnLst"))]
  pub sub_time_node_list: Option<SubTimeNodeList>,
}
/// Previous Conditions List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:prevCondLst")]
pub struct PreviousConditionList {
  /// Condition.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Next Conditions List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:nextCondLst")]
pub struct NextConditionList {
  /// Condition.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Defines the StartConditionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:stCondLst")]
pub struct StartConditionList {
  /// Condition.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Defines the EndConditionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:endCondLst")]
pub struct EndConditionList {
  /// Condition.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Attribute Name.
pub type AttributeName = crate::simple_type::StringValue;
/// Defines the Text Class.
pub type Text = crate::simple_type::StringValue;
/// Attribute Name List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBehaviorAttributeNameList/p:attrNameLst")]
pub struct AttributeNameList {
  /// Attribute Name.
  #[sdk(text_child(qname = "xsd:string/p:attrName"))]
  pub p_attr_name: Vec<crate::simple_type::StringValue>,
}
/// Boolean Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal")]
pub struct BooleanVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Integer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal")]
pub struct IntegerVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Float Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal")]
pub struct FloatVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::SingleValue,
}
/// String Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantStringVal/p:strVal")]
pub struct StringVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Color Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p:clrVal")]
pub struct ColorValue {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub color_value_choice: Option<ColorValueChoice>,
}
/// Pen Color for Slide Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p:penClr")]
pub struct PenColor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub pen_color_choice: Option<PenColorChoice>,
}
/// Time Animate Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeAnimateValue/p:tav")]
pub struct TimeAnimateValue {
  /// Time
  #[sdk(attr(qname = ":tm"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  pub time: Option<crate::simple_type::StringValue>,
  /// Formula
  #[sdk(attr(qname = ":fmla"))]
  pub fomula: Option<crate::simple_type::StringValue>,
  /// Value
  #[sdk(child(qname = "p:CT_TLAnimVariant/p:val"))]
  pub variant_value: Option<std::boxed::Box<VariantValue>>,
}
/// RGB.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByRgbColorTransform/p:rgb")]
pub struct RgbColor {
  /// Red
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub red: crate::simple_type::Int32Value,
  /// Green
  #[sdk(attr(qname = ":g"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub green: crate::simple_type::Int32Value,
  /// Blue
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub blue: crate::simple_type::Int32Value,
}
/// HSL.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByHslColorTransform/p:hsl")]
pub struct HslColor {
  /// Hue
  #[sdk(attr(qname = ":h"))]
  pub hue: crate::simple_type::Int32Value,
  /// Saturation
  #[sdk(attr(qname = ":s"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub saturation: crate::simple_type::Int32Value,
  /// Lightness
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub lightness: crate::simple_type::Int32Value,
}
/// Defines the CommonBehavior Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonBehaviorData/p:cBhvr")]
pub struct CommonBehavior {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Additive
  #[sdk(attr(qname = ":additive"))]
  #[sdk(string_format(kind = "token"))]
  pub additive: Option<BehaviorAdditiveValues>,
  /// Accumulate
  #[sdk(attr(qname = ":accumulate"))]
  #[sdk(string_format(kind = "token"))]
  pub accumulate: Option<BehaviorAccumulateValues>,
  /// Transform Type
  #[sdk(attr(qname = ":xfrmType"))]
  #[sdk(string_format(kind = "token"))]
  pub transform_type: Option<BehaviorTransformValues>,
  /// From
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// To
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  /// By
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::StringValue>,
  /// Runtime Context
  #[sdk(attr(qname = ":rctx"))]
  pub runtime_context: Option<crate::simple_type::StringValue>,
  /// Override
  #[sdk(attr(qname = ":override"))]
  #[sdk(string_format(kind = "token"))]
  pub r#override: Option<BehaviorOverrideValues>,
  /// Parallel TimeNode.
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Target Element
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
  /// Attribute Name List
  #[sdk(child(qname = "p:CT_TLBehaviorAttributeNameList/p:attrNameLst"))]
  pub attribute_name_list: Option<AttributeNameList>,
}
/// Progress.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimFloat/p:progress")]
pub struct Progress {
  /// Float Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  pub float_variant_value: std::boxed::Box<FloatVariantValue>,
}
/// To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariant/p:to")]
pub struct ToVariantValue {
  #[sdk(choice(
    qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal",
    qname = "p:CT_TLAnimVariantIntegerVal/p:intVal",
    qname = "p:CT_TLAnimVariantFloatVal/p:fltVal",
    qname = "p:CT_TLAnimVariantStringVal/p:strVal",
    qname = "a:CT_Color/p:clrVal"
  ))]
  pub to_variant_value_choice: Option<ToVariantValueChoice>,
}
/// Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariant/p:val")]
pub struct VariantValue {
  #[sdk(choice(
    qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal",
    qname = "p:CT_TLAnimVariantIntegerVal/p:intVal",
    qname = "p:CT_TLAnimVariantFloatVal/p:fltVal",
    qname = "p:CT_TLAnimVariantStringVal/p:strVal",
    qname = "a:CT_Color/p:clrVal"
  ))]
  pub variant_value_choice: Option<VariantValueChoice>,
}
/// Common Media Node Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode")]
pub struct CommonMediaNode {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Volume
  #[sdk(attr(qname = ":vol"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub volume: Option<crate::simple_type::Int32Value>,
  /// Mute
  #[sdk(attr(qname = ":mute"))]
  pub mute: Option<crate::simple_type::BooleanValue>,
  /// Number of Slides
  #[sdk(attr(qname = ":numSld"))]
  pub slide_count: Option<crate::simple_type::UInt32Value>,
  /// Show When Stopped
  #[sdk(attr(qname = ":showWhenStopped"))]
  pub show_when_stopped: Option<crate::simple_type::BooleanValue>,
  /// Common Time Node Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Target Element Trigger Choice.
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
}
/// Time Node List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_RootTimeNode/p:tnLst")]
pub struct TimeNodeList {
  /// Parallel Time Node.
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  pub parallel_time_node: std::boxed::Box<ParallelTimeNode>,
}
/// Template Effects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTemplate/p:tmpl")]
pub struct Template {
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// Time Node List
  #[sdk(child(qname = "p:CT_RootTimeNode/p:tnLst"))]
  pub time_node_list: std::boxed::Box<TimeNodeList>,
}
/// Template effects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTemplateList/p:tmplLst")]
pub struct TemplateList {
  /// Template Effects.
  #[sdk(child(qname = "p:CT_TLTemplate/p:tmpl"))]
  pub p_tmpl: Vec<Template>,
}
/// Build Sub Elements.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub")]
pub struct BuildSubElement {
  #[sdk(choice(
    qname = "a:CT_AnimationDgmBuildProperties/a:bldDgm",
    qname = "a:CT_AnimationChartBuildProperties/a:bldChart"
  ))]
  pub build_sub_element_choice: Option<BuildSubElementChoice>,
}
/// Build Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBuildParagraph/p:bldP")]
pub struct BuildParagraph {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Build Types
  #[sdk(attr(qname = ":build"))]
  #[sdk(string_format(kind = "token"))]
  pub build: Option<ParagraphBuildValues>,
  /// Build Level
  #[sdk(attr(qname = ":bldLvl"))]
  pub build_level: Option<crate::simple_type::UInt32Value>,
  /// Animate Background
  #[sdk(attr(qname = ":animBg"))]
  pub animate_background: Option<crate::simple_type::BooleanValue>,
  /// Auto Update Animation Background
  #[sdk(attr(qname = ":autoUpdateAnimBg"))]
  pub auto_animate_background: Option<crate::simple_type::BooleanValue>,
  /// Reverse
  #[sdk(attr(qname = ":rev"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// Auto Advance Time
  #[sdk(attr(qname = ":advAuto"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 2u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 3u32, union = 1u64, values = &["indefinite"]))]
  pub auto_advance: Option<crate::simple_type::StringValue>,
  /// Template effects
  #[sdk(child(qname = "p:CT_TLTemplateList/p:tmplLst"))]
  pub template_list: Option<TemplateList>,
}
/// Build Diagram.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBuildDiagram/p:bldDgm")]
pub struct BuildDiagram {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Diagram Build Types
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_format(kind = "token"))]
  pub build: Option<DiagramBuildValues>,
}
/// Build OLE Chart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLOleBuildChart/p:bldOleChart")]
pub struct BuildOleChart {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Build
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_format(kind = "token"))]
  pub build: Option<OleChartBuildValues>,
  /// Animate Background
  #[sdk(attr(qname = ":animBg"))]
  pub animate_background: Option<crate::simple_type::BooleanValue>,
}
/// Build Graphics.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic")]
pub struct BuildGraphics {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:bldAsOne",
    qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub"
  ))]
  pub build_graphics_choice: Option<BuildGraphicsChoice>,
}
/// Build List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_BuildList/p:bldLst")]
pub struct BuildList {
  #[sdk(choice(
    qname = "p:CT_TLBuildParagraph/p:bldP",
    qname = "p:CT_TLBuildDiagram/p:bldDgm",
    qname = "p:CT_TLOleBuildChart/p:bldOleChart",
    qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic"
  ))]
  pub build_list_choice: Vec<BuildListChoice>,
}
/// Defines the ExtensionListWithModification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionListModify/p:extLst")]
pub struct ExtensionListWithModification {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<Extension>,
}
/// By.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByAnimateColorTransform/p:by")]
pub struct ByColor {
  #[sdk(choice(
    qname = "p:CT_TLByRgbColorTransform/p:rgb",
    qname = "p:CT_TLByHslColorTransform/p:hsl"
  ))]
  pub by_color_choice: Option<ByColorChoice>,
}
/// From.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color3/p:from")]
pub struct FromColor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub from_color_choice: Option<FromColorChoice>,
}
/// To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color3/p:to")]
pub struct ToColor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub to_color_choice: Option<ToColorChoice>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideRelationshipListEntry/p:sld")]
pub struct SlideListEntry {
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomerData/p:custData")]
pub struct CustomerData {
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data Tags.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TagsData/p:tags")]
pub struct CustomerDataTags {
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Comment Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthor/p:cmAuthor")]
pub struct CommentAuthor {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// initials
  #[sdk(attr(qname = ":initials"))]
  pub initials: crate::simple_type::StringValue,
  /// lastIdx
  #[sdk(attr(qname = ":lastIdx"))]
  pub last_index: crate::simple_type::UInt32Value,
  /// clrIdx
  #[sdk(attr(qname = ":clrIdx"))]
  pub color_index: crate::simple_type::UInt32Value,
  /// Defines the CommentAuthorExtensionList Class.
  #[sdk(child(qname = "p:CT_CommentAuthorExtensionList/p:extLst"))]
  pub comment_author_extension_list: Option<CommentAuthorExtensionList>,
}
/// Comment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Comment/p:cm")]
pub struct Comment {
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: crate::simple_type::UInt32Value,
  /// dt
  #[sdk(attr(qname = ":dt"))]
  pub date_time: Option<crate::simple_type::DateTimeValue>,
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// Defines the Position Class.
  #[sdk(child(qname = "a:CT_Point2D/p:pos"))]
  pub position: std::boxed::Box<Position>,
  /// Defines the Text Class.
  #[sdk(text_child(qname = "xsd:string/p:text"))]
  pub text: crate::simple_type::StringValue,
  /// Defines the CommentExtensionList Class.
  #[sdk(child(qname = "p:CT_CommentExtensionList/p:extLst"))]
  pub comment_extension_list: Option<CommentExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<Extension>,
}
/// Embedded Control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Control/p:control")]
pub struct Control {
  /// spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// showAsIcon
  #[sdk(attr(qname = ":showAsIcon"))]
  pub show_as_icon: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// imgW
  #[sdk(attr(qname = ":imgW"))]
  #[sdk(number_range(range = 0..))]
  pub image_width: Option<crate::simple_type::Int32Value>,
  /// imgH
  #[sdk(attr(qname = ":imgH"))]
  #[sdk(number_range(range = 0..))]
  pub image_height: Option<crate::simple_type::Int32Value>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  pub picture: Option<std::boxed::Box<Picture>>,
}
/// Slide ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideIdListEntry/p:sldId")]
pub struct SlideId {
  /// Slide Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 256..2147483648))]
  pub id: crate::simple_type::UInt32Value,
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterIdListEntry/p:sldMasterId")]
pub struct SlideMasterId {
  /// Slide Master Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 2147483648..))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterIdListEntry/p:notesMasterId")]
pub struct NotesMasterId {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Handout Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterIdListEntry/p:handoutMasterId")]
pub struct HandoutMasterId {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Embedded Font Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextFont/p:font")]
pub struct Font {
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(min = 10u32, max = 10u32))]
  pub panose: Option<crate::simple_type::HexBinaryValue>,
  /// Similar Font Family
  #[sdk(attr(qname = ":pitchFamily"))]
  pub pitch_family: Option<crate::simple_type::SByteValue>,
  /// Similar Character Set
  #[sdk(attr(qname = ":charset"))]
  pub character_set: Option<crate::simple_type::SByteValue>,
}
/// Regular Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:regular")]
pub struct RegularFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:bold")]
pub struct BoldFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Italic Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:italic")]
pub struct ItalicFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Italic Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:boldItalic")]
pub struct BoldItalicFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontListEntry/p:embeddedFont")]
pub struct EmbeddedFont {
  /// Embedded Font Name
  #[sdk(child(qname = "a:CT_TextFont/p:font"))]
  pub font: std::boxed::Box<Font>,
  /// Regular Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:regular"))]
  pub regular_font: Option<RegularFont>,
  /// Bold Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:bold"))]
  pub bold_font: Option<BoldFont>,
  /// Italic Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:italic"))]
  pub italic_font: Option<ItalicFont>,
  /// Bold Italic Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:boldItalic"))]
  pub bold_italic_font: Option<BoldItalicFont>,
}
/// List of Presentation Slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideRelationshipList/p:sldLst")]
pub struct SlideList {
  /// Presentation Slide.
  #[sdk(child(qname = "p:CT_SlideRelationshipListEntry/p:sld"))]
  pub p_sld: Vec<SlideListEntry>,
}
/// Custom Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShow/p:custShow")]
pub struct CustomShow {
  /// Custom Show Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Custom Show ID
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// List of Presentation Slides
  #[sdk(child(qname = "p:CT_SlideRelationshipList/p:sldLst"))]
  pub slide_list: std::boxed::Box<SlideList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/p:cNvPr")]
pub struct NonVisualDrawingProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Application defined unique identifier.
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name compatible with Object Model (non-unique).
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Description of the drawing element.
  #[sdk(attr(qname = ":descr"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Flag determining to show or hide this element.
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Hyperlink associated with clicking or selecting the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Drawing Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/p:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<crate::schemas::a::ShapeLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Application Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// Is a Photo Album
  #[sdk(attr(qname = ":isPhoto"))]
  pub is_photo: Option<crate::simple_type::BooleanValue>,
  /// Is User Drawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Placeholder Shape
  #[sdk(child(qname = "p:CT_Placeholder/p:ph"))]
  pub placeholder_shape: Option<std::boxed::Box<PlaceholderShape>>,
  #[sdk(choice(
    qname = "a:CT_AudioCD/a:audioCd",
    qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile",
    qname = "a:CT_AudioFile/a:audioFile",
    qname = "a:CT_VideoFile/a:videoFile",
    qname = "a:CT_QuickTimeFile/a:quickTimeFile"
  ))]
  pub application_non_visual_drawing_properties_choice:
    Option<ApplicationNonVisualDrawingPropertiesChoice>,
  /// Customer Data List.
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub p_cust_data_lst: Option<std::boxed::Box<CustomerDataList>>,
  /// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ApplicationNonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShapeNonVisual/p:nvSpPr")]
pub struct NonVisualShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Drawing Properties for a Shape
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/p:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/p:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Shape Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/p:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference: std::boxed::Box<crate::schemas::a::LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference: std::boxed::Box<crate::schemas::a::FillReference>,
  /// Effect Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<crate::schemas::a::EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference: std::boxed::Box<crate::schemas::a::FontReference>,
}
/// Shape Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/p:txBody")]
pub struct TextBody {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::a::Paragraph>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/p:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<crate::schemas::a::ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection: Option<crate::schemas::a::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection: Option<crate::schemas::a::EndConnection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ConnectorNonVisual/p:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/p:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/p:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the PictureLocks Class.
  #[sdk(child(qname = "a:CT_PictureLocking/a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<crate::schemas::a::PictureLocks>>,
  /// Defines the NonVisualPicturePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst"))]
  pub non_visual_picture_properties_extension_list:
    Option<crate::schemas::a::NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PictureNonVisual/p:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Non-Visual Drawing Properties.
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/p:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
  /// Application Non-Visual Drawing Properties.
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties/p:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip: Option<std::boxed::Box<crate::schemas::a::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle: Option<crate::schemas::a::SourceRectangle>,
  #[sdk(choice(
    qname = "a:CT_TileInfoProperties/a:tile",
    qname = "a:CT_StretchInfoProperties/a:stretch"
  ))]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/p:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<crate::schemas::a::GraphicFrameLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GraphicalObjectFrameNonVisual/p:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/p:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// 2D Transform for Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/p:xfrm")]
pub struct Transform {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  /// Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/p:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<crate::schemas::a::GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<crate::schemas::a::NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Slide Master Title Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:titleStyle")]
pub struct TitleStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Master Body Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:bodyStyle")]
pub struct BodyStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Master Other Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:otherStyle")]
pub struct OtherStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the DefaultTextStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:defaultTextStyle")]
pub struct DefaultTextStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the NotesStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:notesStyle")]
pub struct NotesStyle {
  pub xml_other_children: Vec<(usize, String)>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Layout Id.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutIdListEntry/p:sldLayoutId")]
pub struct SlideLayoutId {
  /// ID Tag
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 2147483648..))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// ID Tag
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Common slide data for notes slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideData/p:cSld")]
pub struct CommonSlideData {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Slide Background
  #[sdk(child(qname = "p:CT_Background/p:bg"))]
  pub background: Option<std::boxed::Box<Background>>,
  /// Shape Tree
  #[sdk(child(qname = "p:CT_GroupShape/p:spTree"))]
  pub shape_tree: std::boxed::Box<ShapeTree>,
  /// Customer Data List
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// List of controls
  #[sdk(child(qname = "p:CT_ControlList/p:controls"))]
  pub control_list: Option<ControlList>,
  /// Defines the CommonSlideDataExtensionList Class.
  #[sdk(child(qname = "p:CT_CommonSlideDataExtensionList/p:extLst"))]
  pub common_slide_data_extension_list: Option<CommonSlideDataExtensionList>,
}
/// Programmable Extensibility Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_StringTag/p:tag")]
pub struct Tag {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Normal View Restored Left Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewPortion/p:restoredLeft")]
pub struct RestoredLeft {
  /// Normal View Dimension Size
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub size: crate::simple_type::Int32Value,
  /// Auto Adjust Normal View
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// Normal View Restored Top Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewPortion/p:restoredTop")]
pub struct RestoredTop {
  /// Normal View Dimension Size
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub size: crate::simple_type::Int32Value,
  /// Auto Adjust Normal View
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// View Scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Scale2D/p:scale")]
pub struct ScaleFactor {
  /// Horizontal Ratio
  #[sdk(child(qname = "a:CT_Ratio/a:sx"))]
  pub scale_x: std::boxed::Box<crate::schemas::a::ScaleX>,
  /// Vertical Ratio
  #[sdk(child(qname = "a:CT_Ratio/a:sy"))]
  pub scale_y: std::boxed::Box<crate::schemas::a::ScaleY>,
}
/// View Origin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/p:origin")]
pub struct Origin {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the Position Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/p:pos")]
pub struct Position {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::Int64Value,
}
/// Base properties for Notes View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonViewProperties/p:cViewPr")]
pub struct CommonViewProperties {
  /// Variable Scale
  #[sdk(attr(qname = ":varScale"))]
  pub variable_scale: Option<crate::simple_type::BooleanValue>,
  /// View Scale
  #[sdk(child(qname = "a:CT_Scale2D/p:scale"))]
  pub scale_factor: std::boxed::Box<ScaleFactor>,
  /// View Origin
  #[sdk(child(qname = "a:CT_Point2D/p:origin"))]
  pub origin: std::boxed::Box<Origin>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewSlideEntry/p:sld")]
pub struct OutlineViewSlideListEntry {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Collapsed
  #[sdk(attr(qname = ":collapse"))]
  pub collapse: Option<crate::simple_type::BooleanValue>,
}
/// List of Presentation Slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewSlideList/p:sldLst")]
pub struct OutlineViewSlideList {
  /// Presentation Slide.
  #[sdk(child(qname = "p:CT_OutlineViewSlideEntry/p:sld"))]
  pub p_sld: Vec<OutlineViewSlideListEntry>,
}
/// A Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Guide/p:guide")]
pub struct Guide {
  /// Guide Orientation
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// Guide Position
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
}
/// List of Guides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GuideList/p:guideLst")]
pub struct GuideList {
  /// A Guide.
  #[sdk(child(qname = "p:CT_Guide/p:guide"))]
  pub p_guide: Vec<Guide>,
}
/// Defines the CommonSlideViewProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr")]
pub struct CommonSlideViewProperties {
  /// Snap Objects to Grid
  #[sdk(attr(qname = ":snapToGrid"))]
  pub snap_to_grid: Option<crate::simple_type::BooleanValue>,
  /// Snap Objects to Objects
  #[sdk(attr(qname = ":snapToObjects"))]
  pub snap_to_objects: Option<crate::simple_type::BooleanValue>,
  /// Show Guides in View
  #[sdk(attr(qname = ":showGuides"))]
  pub show_guides: Option<crate::simple_type::BooleanValue>,
  /// Base properties for Slide View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// List of Guides
  #[sdk(child(qname = "p:CT_GuideList/p:guideLst"))]
  pub guide_list: Option<GuideList>,
}
/// Normal View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewProperties/p:normalViewPr")]
pub struct NormalViewProperties {
  /// Show Outline Icons in Normal View
  #[sdk(attr(qname = ":showOutlineIcons"))]
  pub show_outline_icons: Option<crate::simple_type::BooleanValue>,
  /// Snap Vertical Splitter
  #[sdk(attr(qname = ":snapVertSplitter"))]
  pub snap_vertical_splitter: Option<crate::simple_type::BooleanValue>,
  /// State of the Vertical Splitter Bar
  #[sdk(attr(qname = ":vertBarState"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_bar_state: Option<SplitterBarStateValues>,
  /// State of the Horizontal Splitter Bar
  #[sdk(attr(qname = ":horzBarState"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_bar_state: Option<SplitterBarStateValues>,
  /// Prefer Single View
  #[sdk(attr(qname = ":preferSingleView"))]
  pub prefer_single_view: Option<crate::simple_type::BooleanValue>,
  /// Normal View Restored Left Properties
  #[sdk(child(qname = "p:CT_NormalViewPortion/p:restoredLeft"))]
  pub restored_left: std::boxed::Box<RestoredLeft>,
  /// Normal View Restored Top Properties
  #[sdk(child(qname = "p:CT_NormalViewPortion/p:restoredTop"))]
  pub restored_top: std::boxed::Box<RestoredTop>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideViewProperties/p:slideViewPr")]
pub struct SlideViewProperties {
  /// Defines the CommonSlideViewProperties Class.
  #[sdk(child(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Outline View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewProperties/p:outlineViewPr")]
pub struct OutlineViewProperties {
  /// Common View Properties
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// List of Presentation Slides
  #[sdk(child(qname = "p:CT_OutlineViewSlideList/p:sldLst"))]
  pub outline_view_slide_list: Option<OutlineViewSlideList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Text View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesTextViewProperties/p:notesTextViewPr")]
pub struct NotesTextViewProperties {
  /// Base properties for Notes View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Sorter View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSorterViewProperties/p:sorterViewPr")]
pub struct SorterViewProperties {
  /// Show Formatting
  #[sdk(attr(qname = ":showFormatting"))]
  pub show_formatting: Option<crate::simple_type::BooleanValue>,
  /// Base properties for Slide Sorter View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesViewProperties/p:notesViewPr")]
pub struct NotesViewProperties {
  /// Common Slide View Properties
  #[sdk(child(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Grid Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/p:gridSpacing")]
pub struct GridSpacing {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: crate::simple_type::Int64Value,
}
/// Defines the NotesSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/p:notesSz")]
pub struct NotesSize {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: crate::simple_type::Int64Value,
}
/// Defines the SlideExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideExtension/p:ext")]
pub struct SlideExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p14:CT_LaserTraceList/p14:laserTraceLst",
    qname = "p14:CT_ShowEventRecordList/p14:showEvtLst",
    qname = "p188:CT_CommentRelationship/p188:commentRel",
    any
  ))]
  pub slide_extension_choice: Option<SlideExtensionChoice>,
}
/// Defines the CommonSlideDataExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideDataExtension/p:ext")]
pub struct CommonSlideDataExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p14:CT_RandomId/p14:creationId", any))]
  pub common_slide_data_extension_choice: Option<CommonSlideDataExtensionChoice>,
}
/// Defines the ShowPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowPropertiesExtension/p:ext")]
pub struct ShowPropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p14:CT_BrowseMode/p14:browseMode",
    qname = "a:CT_Color/p14:laserClr",
    qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls",
    any
  ))]
  pub show_properties_extension_choice: Option<ShowPropertiesExtensionChoice>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Picture/p:pic")]
pub struct Picture {
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "p:CT_PictureNonVisual/p:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties/p:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style.
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the OleObjectEmbed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObjectEmbed/p:embed")]
pub struct OleObjectEmbed {
  /// Color Scheme Properties for OLE Object
  #[sdk(attr(qname = ":followColorScheme"))]
  #[sdk(string_format(kind = "token"))]
  pub follow_color_scheme: Option<OleObjectFollowColorSchemeValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OleObjectLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObjectLink/p:link")]
pub struct OleObjectLink {
  /// Update Linked OLE Objects Automatically
  #[sdk(attr(qname = ":updateAutomatic"))]
  pub auto_update: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Transition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideTransition/p:transition")]
pub struct Transition {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// spd
  #[sdk(attr(qname = ":spd"))]
  #[sdk(string_format(kind = "token"))]
  pub speed: Option<TransitionSpeedValues>,
  /// dur
  #[sdk(attr(office2010, qname = "p14:dur"))]
  pub duration: Option<crate::simple_type::StringValue>,
  /// Specifies whether a mouse click will advance the slide.
  #[sdk(attr(qname = ":advClick"))]
  pub advance_on_click: Option<crate::simple_type::BooleanValue>,
  /// advTm
  #[sdk(attr(qname = ":advTm"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub advance_after_time: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "p:CT_OrientationTransition/p:blinds",
    qname = "p:CT_OrientationTransition/p:checker",
    qname = "p:CT_Empty/p:circle",
    qname = "p:CT_Empty/p:dissolve",
    qname = "p:CT_OrientationTransition/p:comb",
    qname = "p:CT_EightDirectionTransition/p:cover",
    qname = "p:CT_OptionalBlackTransition/p:cut",
    qname = "p:CT_Empty/p:diamond",
    qname = "p:CT_OptionalBlackTransition/p:fade",
    qname = "p:CT_Empty/p:newsflash",
    qname = "p:CT_Empty/p:plus",
    qname = "p:CT_EightDirectionTransition/p:pull",
    qname = "p:CT_SideDirectionTransition/p:push",
    qname = "p:CT_Empty/p:random",
    qname = "p:CT_OrientationTransition/p:randomBar",
    qname = "p:CT_SplitTransition/p:split",
    qname = "p:CT_CornerDirectionTransition/p:strips",
    qname = "p:CT_Empty/p:wedge",
    qname = "p:CT_WheelTransition/p:wheel",
    qname = "p:CT_SideDirectionTransition/p:wipe",
    qname = "p:CT_InOutTransition/p:zoom",
    qname = "p:CT_Empty/p14:flash",
    qname = "p:CT_SideDirectionTransition/p14:vortex",
    qname = "p14:CT_LeftRightDirectionTransition/p14:switch",
    qname = "p14:CT_LeftRightDirectionTransition/p14:flip",
    qname = "p14:CT_RippleTransition/p14:ripple",
    qname = "p14:CT_GlitterTransition/p14:glitter",
    qname = "p:CT_Empty/p14:honeycomb",
    qname = "p14:CT_PrismTransition/p14:prism",
    qname = "p:CT_OrientationTransition/p14:doors",
    qname = "p:CT_OrientationTransition/p14:window",
    qname = "p14:CT_ShredTransition/p14:shred",
    qname = "p14:CT_LeftRightDirectionTransition/p14:ferris",
    qname = "p14:CT_FlyThroughTransition/p14:flythrough",
    qname = "p:CT_InOutTransition/p14:warp",
    qname = "p14:CT_LeftRightDirectionTransition/p14:gallery",
    qname = "p14:CT_LeftRightDirectionTransition/p14:conveyor",
    qname = "p:CT_SideDirectionTransition/p14:pan",
    qname = "p14:CT_RevealTransition/p14:reveal",
    qname = "p:CT_WheelTransition/p14:wheelReverse",
    qname = "p15:CT_PresetTransition/p15:prstTrans"
  ))]
  pub transition_choice: Option<TransitionChoice>,
  /// Defines the SoundAction Class.
  #[sdk(child(qname = "p:CT_TransitionSoundAction/p:sndAc"))]
  pub p_snd_ac: Option<std::boxed::Box<SoundAction>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Slide Timing Information for a Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideTiming/p:timing")]
pub struct Timing {
  /// Time Node List.
  #[sdk(child(qname = "p:CT_RootTimeNode/p:tnLst"))]
  pub time_node_list: Option<std::boxed::Box<TimeNodeList>>,
  /// Build List
  #[sdk(child(qname = "p:CT_BuildList/p:bldLst"))]
  pub build_list: Option<BuildList>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideExtensionList/p:extLst")]
pub struct SlideExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SlideExtension Class.
  #[sdk(child(qname = "p:CT_SlideExtension/p:ext"))]
  pub p_ext: Vec<SlideExtension>,
}
/// Slide Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Background/p:bg")]
pub struct Background {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  #[sdk(choice(
    qname = "p:CT_BackgroundProperties/p:bgPr",
    qname = "a:CT_StyleMatrixReference/p:bgRef"
  ))]
  pub background_choice: Option<BackgroundChoice>,
}
/// Shape Tree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShape/p:spTree")]
pub struct ShapeTree {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: Option<std::boxed::Box<NonVisualGroupShapeProperties>>,
  /// Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/p:grpSpPr"))]
  pub group_shape_properties: Option<std::boxed::Box<GroupShapeProperties>>,
  #[sdk(choice(
    qname = "p:CT_Shape/p:sp",
    qname = "p:CT_GroupShape/p:grpSp",
    qname = "p:CT_GraphicalObjectFrame/p:graphicFrame",
    qname = "p:CT_Connector/p:cxnSp",
    qname = "p:CT_Picture/p:pic",
    qname = "p:CT_ContentPart/p:contentPart",
    text,
    any
  ))]
  pub shape_tree_choice: Vec<ShapeTreeChoice>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShape/p:grpSp")]
pub struct GroupShape {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: Option<std::boxed::Box<NonVisualGroupShapeProperties>>,
  /// Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/p:grpSpPr"))]
  pub group_shape_properties: Option<std::boxed::Box<GroupShapeProperties>>,
  #[sdk(choice(
    qname = "p:CT_Shape/p:sp",
    qname = "p:CT_GroupShape/p:grpSp",
    qname = "p:CT_GraphicalObjectFrame/p:graphicFrame",
    qname = "p:CT_Connector/p:cxnSp",
    qname = "p:CT_Picture/p:pic",
    qname = "p:CT_ContentPart/p:contentPart",
    text,
    any
  ))]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Customer Data List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomerDataList/p:custDataLst")]
pub struct CustomerDataList {
  /// Customer Data.
  #[sdk(child(qname = "p:CT_CustomerData/p:custData"))]
  pub p_cust_data: Vec<CustomerData>,
  /// Customer Data Tags.
  #[sdk(child(qname = "p:CT_TagsData/p:tags"))]
  pub p_tags: Option<CustomerDataTags>,
}
/// List of controls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ControlList/p:controls")]
pub struct ControlList {
  /// Embedded Control.
  #[sdk(child(qname = "p:CT_Control/p:control"))]
  pub p_control: Vec<Control>,
}
/// Defines the CommonSlideDataExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideDataExtensionList/p:extLst")]
pub struct CommonSlideDataExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CommonSlideDataExtension Class.
  #[sdk(child(qname = "p:CT_CommonSlideDataExtension/p:ext"))]
  pub p_ext: Vec<CommonSlideDataExtension>,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Non-visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/p:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
  /// Non-Visual Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/p:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<crate::schemas::a::TransformGroup>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::ExtensionList>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Shape/p:sp")]
pub struct Shape {
  /// Use Background Fill
  #[sdk(attr(qname = ":useBgFill"))]
  pub use_background_fill: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Shape
  #[sdk(child(qname = "p:CT_ShapeNonVisual/p:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/p:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GraphicalObjectFrame/p:graphicFrame")]
pub struct GraphicFrame {
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "p:CT_GraphicalObjectFrameNonVisual/p:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// 2D Transform for Graphic Frame
  #[sdk(child(qname = "a:CT_Transform2D/p:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// Graphic Object.
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
  /// Extension List with Modification Flag
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Connector/p:cxnSp")]
pub struct ConnectionShape {
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "p:CT_ConnectorNonVisual/p:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Connector Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ShowPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowPropertiesExtensionList/p:extLst")]
pub struct ShowPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ShowPropertiesExtension Class.
  #[sdk(child(qname = "p:CT_ShowPropertiesExtension/p:ext"))]
  pub p_ext: Vec<ShowPropertiesExtension>,
}
/// Shape Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLShapeTargetElement/p:spTgt")]
pub struct ShapeTarget {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p:CT_Empty/p:bg",
    qname = "p:CT_TLSubShapeId/p:subSp",
    qname = "p:CT_TLOleChartTargetElement/p:oleChartEl",
    qname = "p:CT_TLTextTargetElement/p:txEl",
    qname = "a:CT_AnimationElementChoice/p:graphicEl"
  ))]
  pub shape_target_choice: Option<ShapeTargetChoice>,
}
/// Ink Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSubShapeId/p:inkTgt")]
pub struct InkTarget {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Subshape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSubShapeId/p:subSp")]
pub struct SubShape {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Defines the CommentAuthorExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorExtension/p:ext")]
pub struct CommentAuthorExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_PresenceInfo/p15:presenceInfo", any))]
  pub comment_author_extension_choice: Option<CommentAuthorExtensionChoice>,
}
/// Defines the CommentExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentExtension/p:ext")]
pub struct CommentExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_CommentThreading/p15:threadingInfo", any))]
  pub comment_extension_choice: Option<CommentExtensionChoice>,
}
/// Defines the SlideLayoutExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutExtension/p:ext")]
pub struct SlideLayoutExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst", any))]
  pub slide_layout_extension_choice: Option<SlideLayoutExtensionChoice>,
}
/// Defines the SlideMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterExtension/p:ext")]
pub struct SlideMasterExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst", any))]
  pub slide_master_extension_choice: Option<SlideMasterExtensionChoice>,
}
/// Defines the HandoutMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterExtension/p:ext")]
pub struct HandoutMasterExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst", any))]
  pub handout_master_extension_choice: Option<HandoutMasterExtensionChoice>,
}
/// Defines the NotesMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterExtension/p:ext")]
pub struct NotesMasterExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst", any))]
  pub notes_master_extension_choice: Option<NotesMasterExtensionChoice>,
}
/// Placeholder Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Placeholder/p:ph")]
pub struct PlaceholderShape {
  /// type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<PlaceholderValues>,
  /// orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(string_format(kind = "token"))]
  pub size: Option<PlaceholderSizeValues>,
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// hasCustomPrompt
  #[sdk(attr(qname = ":hasCustomPrompt"))]
  pub has_custom_prompt: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst")]
pub struct ApplicationNonVisualDrawingPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingPropsExtension/p:ext"))]
  pub p_ext: Vec<ApplicationNonVisualDrawingPropertiesExtension>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingPropsExtension/p:ext")]
pub struct ApplicationNonVisualDrawingPropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p14:CT_Media/p14:media",
    qname = "p14:CT_RandomId/p14:modId",
    any
  ))]
  pub application_non_visual_drawing_properties_extension_choice:
    Option<ApplicationNonVisualDrawingPropertiesExtensionChoice>,
}
/// Defines the Iterate Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateData/p:iterate")]
pub struct Iterate {
  /// Iterate Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<IterateValues>,
  /// Backwards
  #[sdk(attr(qname = ":backwards"))]
  pub backwards: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "p:CT_TLIterateIntervalTime/p:tmAbs",
    qname = "p:CT_TLIterateIntervalPercentage/p:tmPct"
  ))]
  pub iterate_choice: Option<IterateChoice>,
}
/// Defines the ChildTimeNodeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TimeNodeList/p:childTnLst")]
pub struct ChildTimeNodeList {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "p:CT_TLTimeNodeParallel/p:par",
    qname = "p:CT_TLTimeNodeSequence/p:seq",
    qname = "p:CT_TLTimeNodeExclusive/p:excl",
    qname = "p:CT_TLAnimateBehavior/p:anim",
    qname = "p:CT_TLAnimateColorBehavior/p:animClr",
    qname = "p:CT_TLAnimateEffectBehavior/p:animEffect",
    qname = "p:CT_TLAnimateMotionBehavior/p:animMotion",
    qname = "p:CT_TLAnimateRotationBehavior/p:animRot",
    qname = "p:CT_TLAnimateScaleBehavior/p:animScale",
    qname = "p:CT_TLCommandBehavior/p:cmd",
    qname = "p:CT_TLSetBehavior/p:set",
    qname = "p:CT_TLMediaNodeAudio/p:audio",
    qname = "p:CT_TLMediaNodeVideo/p:video",
    text,
    any
  ))]
  pub child_time_node_list_choice: Vec<ChildTimeNodeListChoice>,
}
/// Defines the SubTimeNodeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TimeNodeList/p:subTnLst")]
pub struct SubTimeNodeList {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "p:CT_TLTimeNodeParallel/p:par",
    qname = "p:CT_TLTimeNodeSequence/p:seq",
    qname = "p:CT_TLTimeNodeExclusive/p:excl",
    qname = "p:CT_TLAnimateBehavior/p:anim",
    qname = "p:CT_TLAnimateColorBehavior/p:animClr",
    qname = "p:CT_TLAnimateEffectBehavior/p:animEffect",
    qname = "p:CT_TLAnimateMotionBehavior/p:animMotion",
    qname = "p:CT_TLAnimateRotationBehavior/p:animRot",
    qname = "p:CT_TLAnimateScaleBehavior/p:animScale",
    qname = "p:CT_TLCommandBehavior/p:cmd",
    qname = "p:CT_TLSetBehavior/p:set",
    qname = "p:CT_TLMediaNodeAudio/p:audio",
    qname = "p:CT_TLMediaNodeVideo/p:video",
    text,
    any
  ))]
  pub sub_time_node_list_choice: Vec<SubTimeNodeListChoice>,
}
/// Defines the TimeAnimateValueList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeAnimateValueList/p:tavLst")]
pub struct TimeAnimateValueList {
  /// Time Animate Value.
  #[sdk(child(qname = "p:CT_TLTimeAnimateValue/p:tav"))]
  pub p_tav: Vec<TimeAnimateValue>,
}
/// Defines the ByPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:by")]
pub struct ByPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the FromPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:from")]
pub struct FromPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the ToPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:to")]
pub struct ToPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the RotationCenter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:rCtr")]
pub struct RotationCenter {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the CommentAuthorExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorExtensionList/p:extLst")]
pub struct CommentAuthorExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CommentAuthorExtension Class.
  #[sdk(child(qname = "p:CT_CommentAuthorExtension/p:ext"))]
  pub p_ext: Vec<CommentAuthorExtension>,
}
/// Defines the CommentExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentExtensionList/p:extLst")]
pub struct CommentExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CommentExtension Class.
  #[sdk(child(qname = "p:CT_CommentExtension/p:ext"))]
  pub p_ext: Vec<CommentExtension>,
}
/// Defines the SlideMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterIdList/p:sldMasterIdLst")]
pub struct SlideMasterIdList {
  /// Slide Master ID.
  #[sdk(child(qname = "p:CT_SlideMasterIdListEntry/p:sldMasterId"))]
  pub p_sld_master_id: Vec<SlideMasterId>,
}
/// Defines the NotesMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterIdList/p:notesMasterIdLst")]
pub struct NotesMasterIdList {
  /// Notes Master ID
  #[sdk(child(qname = "p:CT_NotesMasterIdListEntry/p:notesMasterId"))]
  pub notes_master_id: Option<std::boxed::Box<NotesMasterId>>,
}
/// Defines the HandoutMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterIdList/p:handoutMasterIdLst")]
pub struct HandoutMasterIdList {
  /// Handout Master ID
  #[sdk(child(qname = "p:CT_HandoutMasterIdListEntry/p:handoutMasterId"))]
  pub handout_master_id: Option<std::boxed::Box<HandoutMasterId>>,
}
/// Defines the SlideIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideIdList/p:sldIdLst")]
pub struct SlideIdList {
  /// Slide ID.
  #[sdk(child(qname = "p:CT_SlideIdListEntry/p:sldId"))]
  pub p_sld_id: Vec<SlideId>,
}
/// Defines the SlideSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSize/p:sldSz")]
pub struct SlideSize {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 914400..= 51206400))]
  pub cx: crate::simple_type::Int32Value,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 914400..= 51206400))]
  pub cy: crate::simple_type::Int32Value,
  /// Type of Size
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<SlideSizeValues>,
}
/// Defines the EmbeddedFontList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontList/p:embeddedFontLst")]
pub struct EmbeddedFontList {
  /// Embedded Font.
  #[sdk(child(qname = "p:CT_EmbeddedFontListEntry/p:embeddedFont"))]
  pub p_embedded_font: Vec<EmbeddedFont>,
}
/// Defines the CustomShowList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShowList/p:custShowLst")]
pub struct CustomShowList {
  /// Custom Show.
  #[sdk(child(qname = "p:CT_CustomShow/p:custShow"))]
  pub p_cust_show: Vec<CustomShow>,
}
/// Defines the PhotoAlbum Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PhotoAlbum/p:photoAlbum")]
pub struct PhotoAlbum {
  /// Black and White
  #[sdk(attr(qname = ":bw"))]
  pub black_white: Option<crate::simple_type::BooleanValue>,
  /// Show/Hide Captions
  #[sdk(attr(qname = ":showCaptions"))]
  pub show_captions: Option<crate::simple_type::BooleanValue>,
  /// Photo Album Layout
  #[sdk(attr(qname = ":layout"))]
  #[sdk(string_format(kind = "token"))]
  pub layout: Option<PhotoAlbumLayoutValues>,
  /// Frame Type
  #[sdk(attr(qname = ":frame"))]
  #[sdk(string_format(kind = "token"))]
  pub frame: Option<PhotoAlbumFrameShapeValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Kinsoku Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Kinsoku/p:kinsoku")]
pub struct Kinsoku {
  /// Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Invalid Kinsoku Start Characters
  #[sdk(attr(qname = ":invalStChars"))]
  pub invalid_start_chars: crate::simple_type::StringValue,
  /// Invalid Kinsoku End Characters
  #[sdk(attr(qname = ":invalEndChars"))]
  pub invalid_end_chars: crate::simple_type::StringValue,
}
/// Defines the ModificationVerifier Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ModifyVerifier/p:modifyVerifier")]
pub struct ModificationVerifier {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Cryptographic Provider Type
  #[sdk(attr(qname = ":cryptProviderType"))]
  pub cryptographic_provider_type: CryptProviderValues,
  /// Cryptographic Algorithm Class
  #[sdk(attr(qname = ":cryptAlgorithmClass"))]
  pub cryptographic_algorithm_class: CryptAlgorithmClassValues,
  /// Cryptographic Algorithm Type
  #[sdk(attr(qname = ":cryptAlgorithmType"))]
  pub cryptographic_algorithm_type: CryptAlgorithmValues,
  /// Cryptographic Hashing Algorithm
  #[sdk(attr(qname = ":cryptAlgorithmSid"))]
  pub cryptographic_algorithm_sid: crate::simple_type::UInt32Value,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: crate::simple_type::UInt32Value,
  /// Salt for Password Verifier
  #[sdk(attr(qname = ":saltData"))]
  pub salt_data: crate::simple_type::Base64BinaryValue,
  /// Password Hash
  #[sdk(attr(qname = ":hashData"))]
  pub hash_data: crate::simple_type::StringValue,
  /// Cryptographic Provider
  #[sdk(attr(qname = ":cryptProvider"))]
  pub cryptographic_provider: Option<crate::simple_type::StringValue>,
  /// Cryptographic Algorithm Extensibility
  #[sdk(attr(qname = ":algIdExt"))]
  pub extended_cryptographic_algorithm: Option<crate::simple_type::UInt32Value>,
  /// Algorithm Extensibility Source
  #[sdk(attr(qname = ":algIdExtSource"))]
  pub extended_cryptographic_algorithm_source: Option<crate::simple_type::StringValue>,
  /// Cryptographic Provider Type Extensibility
  #[sdk(attr(qname = ":cryptProviderTypeExt"))]
  pub cryptographic_provider_type_extensibility: Option<crate::simple_type::UInt32Value>,
  /// Provider Type Extensibility Source
  #[sdk(attr(qname = ":cryptProviderTypeExtSource"))]
  pub cryptographic_provider_type_extensibility_source: Option<crate::simple_type::StringValue>,
  /// algorithmName
  #[sdk(attr(office2010, qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(office2010, qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(office2010, qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinValue
  #[sdk(attr(office2010, qname = ":spinValue"))]
  pub spin_value: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PresentationExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationExtensionList/p:extLst")]
pub struct PresentationExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PresentationExtension Class.
  #[sdk(child(qname = "p:CT_PresentationExtension/p:ext"))]
  pub p_ext: Vec<PresentationExtension>,
}
/// Defines the PresentationExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationExtension/p:ext")]
pub struct PresentationExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p14:CT_SectionProperties/p14:sectionPr",
    qname = "p14:CT_SectionList/p14:sectionLst",
    qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst",
    qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst",
    any
  ))]
  pub presentation_extension_choice: Option<PresentationExtensionChoice>,
}
/// HTML Publishing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HtmlPublishProperties/p:htmlPubPr")]
pub struct HtmlPublishProperties {
  /// Show Speaker Notes
  #[sdk(attr(qname = ":showSpeakerNotes"))]
  pub show_speaker_notes: Option<crate::simple_type::BooleanValue>,
  /// Browser Support Target
  #[sdk(attr(qname = ":pubBrowser"))]
  #[sdk(string_format(kind = "token"))]
  pub target_browser: Option<HtmlPublishWebBrowserSupportValues>,
  /// Publish Path
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p:CT_Empty/p:sldAll",
    qname = "p:CT_IndexRange/p:sldRg",
    qname = "p:CT_CustomShowId/p:custShow"
  ))]
  pub html_publish_properties_choice: Option<HtmlPublishPropertiesChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ExtensionList>,
}
/// Web Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_WebProperties/p:webPr")]
pub struct WebProperties {
  /// Show animation in HTML output
  #[sdk(attr(qname = ":showAnimation"))]
  pub show_animation: Option<crate::simple_type::BooleanValue>,
  /// Resize graphics in HTML output
  #[sdk(attr(qname = ":resizeGraphics"))]
  pub resize_graphics: Option<crate::simple_type::BooleanValue>,
  /// Allow PNG in HTML output
  #[sdk(attr(qname = ":allowPng"))]
  pub allow_png: Option<crate::simple_type::BooleanValue>,
  /// Rely on VML for HTML output
  #[sdk(attr(qname = ":relyOnVml"))]
  pub rely_on_vml: Option<crate::simple_type::BooleanValue>,
  /// Organize HTML output in folders
  #[sdk(attr(qname = ":organizeInFolders"))]
  pub organize_in_folders: Option<crate::simple_type::BooleanValue>,
  /// Use long file names in HTML output
  #[sdk(attr(qname = ":useLongFilenames"))]
  pub use_long_filenames: Option<crate::simple_type::BooleanValue>,
  /// Image size for HTML output
  #[sdk(attr(qname = ":imgSz"))]
  #[sdk(string_format(kind = "token"))]
  pub image_size: Option<WebScreenSizeValues>,
  /// Encoding for HTML output
  #[sdk(attr(qname = ":encoding"))]
  pub encoding: Option<crate::simple_type::StringValue>,
  /// Slide Navigation Colors for HTML output
  #[sdk(attr(qname = ":clr"))]
  #[sdk(string_format(kind = "token"))]
  pub color: Option<WebColorValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PrintingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PrintProperties/p:prnPr")]
pub struct PrintingProperties {
  /// Print Output
  #[sdk(attr(qname = ":prnWhat"))]
  #[sdk(string_format(kind = "token"))]
  pub print_what: Option<PrintOutputValues>,
  /// Print Color Mode
  #[sdk(attr(qname = ":clrMode"))]
  #[sdk(string_format(kind = "token"))]
  pub color_mode: Option<PrintColorModeValues>,
  /// Print Hidden Slides
  #[sdk(attr(qname = ":hiddenSlides"))]
  pub hidden_slides: Option<crate::simple_type::BooleanValue>,
  /// Scale to Fit Paper when printing
  #[sdk(attr(qname = ":scaleToFitPaper"))]
  pub scale_to_fit_paper: Option<crate::simple_type::BooleanValue>,
  /// Frame slides when printing
  #[sdk(attr(qname = ":frameSlides"))]
  pub frame_slides: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ShowProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowProperties/p:showPr")]
pub struct ShowProperties {
  /// Loop Slide Show
  #[sdk(attr(qname = ":loop"))]
  pub r#loop: Option<crate::simple_type::BooleanValue>,
  /// Show Narration in Slide Show
  #[sdk(attr(qname = ":showNarration"))]
  pub show_narration: Option<crate::simple_type::BooleanValue>,
  /// Show Animation in Slide Show
  #[sdk(attr(qname = ":showAnimation"))]
  pub show_animation: Option<crate::simple_type::BooleanValue>,
  /// Use Timings in Slide Show
  #[sdk(attr(qname = ":useTimings"))]
  pub use_timings: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:present",
    qname = "p:CT_ShowInfoBrowse/p:browse",
    qname = "p:CT_ShowInfoKiosk/p:kiosk"
  ))]
  pub show_properties_choice1: Option<ShowPropertiesChoice>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:sldAll",
    qname = "p:CT_IndexRange/p:sldRg",
    qname = "p:CT_CustomShowId/p:custShow"
  ))]
  pub show_properties_choice2: Option<ShowPropertiesChoice2>,
  /// Pen Color for Slide Show.
  #[sdk(child(qname = "a:CT_Color/p:penClr"))]
  pub p_pen_clr: Option<std::boxed::Box<PenColor>>,
  /// Defines the ShowPropertiesExtensionList Class.
  #[sdk(child(qname = "p:CT_ShowPropertiesExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ShowPropertiesExtensionList>,
}
/// Defines the ColorMostRecentlyUsed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMRU/p:clrMru")]
pub struct ColorMostRecentlyUsed {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr",
    text,
    any
  ))]
  pub color_most_recently_used_choice: Vec<ColorMostRecentlyUsedChoice>,
}
/// Defines the PresentationPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationPropertiesExtensionList/p:extLst")]
pub struct PresentationPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PresentationPropertiesExtension Class.
  #[sdk(child(qname = "p:CT_PresentationPropertiesExtension/p:ext"))]
  pub p_ext: Vec<PresentationPropertiesExtension>,
}
/// Defines the PresentationPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationPropertiesExtension/p:ext")]
pub struct PresentationPropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData",
    qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi",
    qname = "a14:CT_TextMath/a14:m",
    qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased",
    any
  ))]
  pub presentation_properties_extension_choice: Option<PresentationPropertiesExtensionChoice>,
}
/// Defines the HeaderFooter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HeaderFooter/p:hf")]
pub struct HeaderFooter {
  /// Slide Number Placeholder
  #[sdk(attr(qname = ":sldNum"))]
  pub slide_number: Option<crate::simple_type::BooleanValue>,
  /// Header Placeholder
  #[sdk(attr(qname = ":hdr"))]
  pub header: Option<crate::simple_type::BooleanValue>,
  /// Footer Placeholder
  #[sdk(attr(qname = ":ftr"))]
  pub footer: Option<crate::simple_type::BooleanValue>,
  /// Date/Time Placeholder
  #[sdk(attr(qname = ":dt"))]
  pub date_time: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideLayoutExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutExtensionList/p:extLst")]
pub struct SlideLayoutExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SlideLayoutExtension Class.
  #[sdk(child(qname = "p:CT_SlideLayoutExtension/p:ext"))]
  pub p_ext: Vec<SlideLayoutExtension>,
}
/// Defines the SlideLayoutIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutIdList/p:sldLayoutIdLst")]
pub struct SlideLayoutIdList {
  /// Slide Layout Id.
  #[sdk(child(qname = "p:CT_SlideLayoutIdListEntry/p:sldLayoutId"))]
  pub p_sld_layout_id: Vec<SlideLayoutId>,
}
/// Defines the TextStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterTextStyles/p:txStyles")]
pub struct TextStyles {
  /// Slide Master Title Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:titleStyle"))]
  pub title_style: Option<std::boxed::Box<TitleStyle>>,
  /// Slide Master Body Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:bodyStyle"))]
  pub body_style: Option<std::boxed::Box<BodyStyle>>,
  /// Slide Master Other Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:otherStyle"))]
  pub other_style: Option<std::boxed::Box<OtherStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlideMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterExtensionList/p:extLst")]
pub struct SlideMasterExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SlideMasterExtension Class.
  #[sdk(child(qname = "p:CT_SlideMasterExtension/p:ext"))]
  pub p_ext: Vec<SlideMasterExtension>,
}
/// Defines the HandoutMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterExtensionList/p:extLst")]
pub struct HandoutMasterExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the HandoutMasterExtension Class.
  #[sdk(child(qname = "p:CT_HandoutMasterExtension/p:ext"))]
  pub p_ext: Vec<HandoutMasterExtension>,
}
/// Defines the NotesMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterExtensionList/p:extLst")]
pub struct NotesMasterExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the NotesMasterExtension Class.
  #[sdk(child(qname = "p:CT_NotesMasterExtension/p:ext"))]
  pub p_ext: Vec<NotesMasterExtension>,
}
/// OLE Chart Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLOleChartTargetElement/p:oleChartEl")]
pub struct OleChartElement {
  /// Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: ChartSubElementValues,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  pub level: Option<crate::simple_type::UInt32Value>,
}
/// Text Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTextTargetElement/p:txEl")]
pub struct TextElement {
  #[sdk(choice(qname = "p:CT_IndexRange/p:charRg", qname = "p:CT_IndexRange/p:pRg"))]
  pub text_element_choice: Option<TextElementChoice>,
}
/// Graphic Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationElementChoice/p:graphicEl")]
pub struct GraphicElement {
  #[sdk(choice(
    qname = "a:CT_AnimationDgmElement/a:dgm",
    qname = "a:CT_AnimationChartElement/a:chart"
  ))]
  pub graphic_element_choice: Option<GraphicElementChoice>,
}
/// Defines the BlindsTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:blinds")]
pub struct BlindsTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CheckerTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:checker")]
pub struct CheckerTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CombTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:comb")]
pub struct CombTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the RandomBarTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:randomBar")]
pub struct RandomBarTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CoverTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EightDirectionTransition/p:cover")]
pub struct CoverTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the PullTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EightDirectionTransition/p:pull")]
pub struct PullTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the CutTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OptionalBlackTransition/p:cut")]
pub struct CutTransition {
  /// Transition Through Black
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the FadeTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OptionalBlackTransition/p:fade")]
pub struct FadeTransition {
  /// Transition Through Black
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PushTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p:push")]
pub struct PushTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the WipeTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p:wipe")]
pub struct WipeTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SplitTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SplitTransition/p:split")]
pub struct SplitTransition {
  /// Orientation
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the StripsTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CornerDirectionTransition/p:strips")]
pub struct StripsTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionCornerDirectionValues>,
}
/// Defines the WheelTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_WheelTransition/p:wheel")]
pub struct WheelTransition {
  /// Spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ZoomTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_InOutTransition/p:zoom")]
pub struct ZoomTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the SoundAction Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TransitionSoundAction/p:sndAc")]
pub struct SoundAction {
  #[sdk(choice(
    qname = "p:CT_TransitionStartSoundAction/p:stSnd",
    qname = "p:CT_Empty/p:endSnd"
  ))]
  pub sound_action_choice: Option<SoundActionChoice>,
}
/// Defines the PlaceholderExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p:CT_PlaceholderExtension/p:ext")]
pub struct PlaceholderExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PlaceholderTypeExtension Class.
  #[sdk(child(
    microsoft365,
    qname = "p232:CT_PlaceholderTypeExtension/p232:phTypeExt"
  ))]
  pub placeholder_type_extension:
    Option<std::boxed::Box<crate::schemas::p232::PlaceholderTypeExtension>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorMapOverrideChoice {
  /// Master Color Mapping.
  #[sdk(empty_child(qname = "a:CT_EmptyElement/a:masterClrMapping"))]
  AMasterClrMapping,
  /// Override Color Mapping.
  #[sdk(child(qname = "a:CT_ColorMapping/a:overrideClrMapping"))]
  AOverrideClrMapping(std::boxed::Box<crate::schemas::a::OverrideColorMapping>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundPropertiesChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<crate::schemas::a::PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundPropertiesChoice2 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundStyleReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OleObjectChoice {
  #[sdk(child(qname = "p:CT_OleObjectEmbed/p:embed"))]
  PEmbed(std::boxed::Box<OleObjectEmbed>),
  #[sdk(child(qname = "p:CT_OleObjectLink/p:link"))]
  PLink(std::boxed::Box<OleObjectLink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TargetElementChoice {
  /// Slide Target.
  #[sdk(empty_child(qname = "p:CT_Empty/p:sldTgt"))]
  PSldTgt,
  /// Sound Target.
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt"))]
  PSndTgt(std::boxed::Box<SoundTarget>),
  /// Shape Target.
  #[sdk(child(qname = "p:CT_TLShapeTargetElement/p:spTgt"))]
  PSpTgt(std::boxed::Box<ShapeTarget>),
  /// Ink Target.
  #[sdk(child(qname = "p:CT_TLSubShapeId/p:inkTgt"))]
  PInkTgt(std::boxed::Box<InkTarget>),
  /// Defines the BookmarkTarget Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt"))]
  P14BmkTgt(std::boxed::Box<crate::schemas::p14::BookmarkTarget>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ConditionChoice {
  /// Target Element Trigger Choice.
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  PTgtEl(std::boxed::Box<TargetElement>),
  /// Time Node.
  #[sdk(child(qname = "p:CT_TLTriggerTimeNodeID/p:tn"))]
  PTn(std::boxed::Box<TimeNode>),
  /// Runtime Node Trigger Choice.
  #[sdk(child(qname = "p:CT_TLTriggerRuntimeNode/p:rtn"))]
  PRtn(std::boxed::Box<RuntimeNodeTrigger>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndSyncChoice {
  /// Target Element Trigger Choice.
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  PTgtEl(std::boxed::Box<TargetElement>),
  /// Time Node.
  #[sdk(child(qname = "p:CT_TLTriggerTimeNodeID/p:tn"))]
  PTn(std::boxed::Box<TimeNode>),
  /// Runtime Node Trigger Choice.
  #[sdk(child(qname = "p:CT_TLTriggerRuntimeNode/p:rtn"))]
  PRtn(std::boxed::Box<RuntimeNodeTrigger>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorValueChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PenColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ToVariantValueChoice {
  /// Boolean Variant.
  #[sdk(child(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal"))]
  PBoolVal(std::boxed::Box<BooleanVariantValue>),
  /// Integer.
  #[sdk(child(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal"))]
  PIntVal(std::boxed::Box<IntegerVariantValue>),
  /// Float Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  PFltVal(std::boxed::Box<FloatVariantValue>),
  /// String Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantStringVal/p:strVal"))]
  PStrVal(std::boxed::Box<StringVariantValue>),
  /// Color Value.
  #[sdk(child(qname = "a:CT_Color/p:clrVal"))]
  PClrVal(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VariantValueChoice {
  /// Boolean Variant.
  #[sdk(child(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal"))]
  PBoolVal(std::boxed::Box<BooleanVariantValue>),
  /// Integer.
  #[sdk(child(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal"))]
  PIntVal(std::boxed::Box<IntegerVariantValue>),
  /// Float Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  PFltVal(std::boxed::Box<FloatVariantValue>),
  /// String Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantStringVal/p:strVal"))]
  PStrVal(std::boxed::Box<StringVariantValue>),
  /// Color Value.
  #[sdk(child(qname = "a:CT_Color/p:clrVal"))]
  PClrVal(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildSubElementChoice {
  /// Build Diagram.
  #[sdk(child(qname = "a:CT_AnimationDgmBuildProperties/a:bldDgm"))]
  ABldDgm(std::boxed::Box<crate::schemas::a::BuildDiagram>),
  /// Build Chart.
  #[sdk(child(qname = "a:CT_AnimationChartBuildProperties/a:bldChart"))]
  ABldChart(std::boxed::Box<crate::schemas::a::BuildChart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildGraphicsChoice {
  /// Build As One.
  #[sdk(empty_child(qname = "p:CT_Empty/p:bldAsOne"))]
  PBldAsOne,
  /// Build Sub Elements.
  #[sdk(child(qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub"))]
  PBldSub(std::boxed::Box<BuildSubElement>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildListChoice {
  /// Build Paragraph.
  #[sdk(child(qname = "p:CT_TLBuildParagraph/p:bldP"))]
  PBldP(std::boxed::Box<BuildParagraph>),
  /// Build Diagram.
  #[sdk(child(qname = "p:CT_TLBuildDiagram/p:bldDgm"))]
  PBldDgm(std::boxed::Box<BuildDiagram>),
  /// Build OLE Chart.
  #[sdk(child(qname = "p:CT_TLOleBuildChart/p:bldOleChart"))]
  PBldOleChart(std::boxed::Box<BuildOleChart>),
  /// Build Graphics.
  #[sdk(child(qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic"))]
  PBldGraphic(std::boxed::Box<BuildGraphics>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ByColorChoice {
  /// RGB.
  #[sdk(child(qname = "p:CT_TLByRgbColorTransform/p:rgb"))]
  PRgb(std::boxed::Box<RgbColor>),
  /// HSL.
  #[sdk(child(qname = "p:CT_TLByHslColorTransform/p:hsl"))]
  PHsl(std::boxed::Box<HslColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FromColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ToColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ApplicationNonVisualDrawingPropertiesChoice {
  #[sdk(child(qname = "a:CT_AudioCD/a:audioCd"))]
  AAudioCd(std::boxed::Box<crate::schemas::a::AudioFromCd>),
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile"))]
  AWavAudioFile(std::boxed::Box<crate::schemas::a::WaveAudioFile>),
  #[sdk(child(qname = "a:CT_AudioFile/a:audioFile"))]
  AAudioFile(std::boxed::Box<crate::schemas::a::AudioFromFile>),
  #[sdk(child(qname = "a:CT_VideoFile/a:videoFile"))]
  AVideoFile(std::boxed::Box<crate::schemas::a::VideoFromFile>),
  #[sdk(child(qname = "a:CT_QuickTimeFile/a:quickTimeFile"))]
  AQuickTimeFile(std::boxed::Box<crate::schemas::a::QuickTimeFromFile>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::a::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(std::boxed::Box<crate::schemas::a::Stretch>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideExtensionChoice {
  /// Defines the LaserTraceList Class.
  #[sdk(child(office2010, qname = "p14:CT_LaserTraceList/p14:laserTraceLst"))]
  P14LaserTraceLst(std::boxed::Box<crate::schemas::p14::LaserTraceList>),
  /// Defines the ShowEventRecordList Class.
  #[sdk(child(office2010, qname = "p14:CT_ShowEventRecordList/p14:showEvtLst"))]
  P14ShowEvtLst(std::boxed::Box<crate::schemas::p14::ShowEventRecordList>),
  /// Defines the CommentRelationship Class.
  #[sdk(child(office2021, qname = "p188:CT_CommentRelationship/p188:commentRel"))]
  P188CommentRel(std::boxed::Box<crate::schemas::p188::CommentRelationship>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommonSlideDataExtensionChoice {
  /// Defines the CreationId Class.
  #[sdk(child(office2010, qname = "p14:CT_RandomId/p14:creationId"))]
  P14CreationId(std::boxed::Box<crate::schemas::p14::CreationId>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesExtensionChoice {
  /// Defines the BrowseMode Class.
  #[sdk(child(office2010, qname = "p14:CT_BrowseMode/p14:browseMode"))]
  P14BrowseMode(std::boxed::Box<crate::schemas::p14::BrowseMode>),
  /// Defines the LaserColor Class.
  #[sdk(child(office2010, qname = "a:CT_Color/p14:laserClr"))]
  P14LaserClr(std::boxed::Box<crate::schemas::p14::LaserColor>),
  /// Defines the ShowMediaControls Class.
  #[sdk(child(office2010, qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls"))]
  P14ShowMediaCtrls(std::boxed::Box<crate::schemas::p14::ShowMediaControls>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TransitionChoice {
  #[sdk(child(qname = "p:CT_OrientationTransition/p:blinds"))]
  PBlinds(std::boxed::Box<BlindsTransition>),
  #[sdk(child(qname = "p:CT_OrientationTransition/p:checker"))]
  PChecker(std::boxed::Box<CheckerTransition>),
  /// Defines the CircleTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:circle"))]
  PCircle,
  /// Defines the DissolveTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:dissolve"))]
  PDissolve,
  #[sdk(child(qname = "p:CT_OrientationTransition/p:comb"))]
  PComb(std::boxed::Box<CombTransition>),
  #[sdk(child(qname = "p:CT_EightDirectionTransition/p:cover"))]
  PCover(std::boxed::Box<CoverTransition>),
  #[sdk(child(qname = "p:CT_OptionalBlackTransition/p:cut"))]
  PCut(std::boxed::Box<CutTransition>),
  /// Defines the DiamondTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:diamond"))]
  PDiamond,
  #[sdk(child(qname = "p:CT_OptionalBlackTransition/p:fade"))]
  PFade(std::boxed::Box<FadeTransition>),
  /// Defines the NewsflashTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:newsflash"))]
  PNewsflash,
  /// Defines the PlusTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:plus"))]
  PPlus,
  #[sdk(child(qname = "p:CT_EightDirectionTransition/p:pull"))]
  PPull(std::boxed::Box<PullTransition>),
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p:push"))]
  PPush(std::boxed::Box<PushTransition>),
  /// Defines the RandomTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:random"))]
  PRandom,
  #[sdk(child(qname = "p:CT_OrientationTransition/p:randomBar"))]
  PRandomBar(std::boxed::Box<RandomBarTransition>),
  #[sdk(child(qname = "p:CT_SplitTransition/p:split"))]
  PSplit(std::boxed::Box<SplitTransition>),
  #[sdk(child(qname = "p:CT_CornerDirectionTransition/p:strips"))]
  PStrips(std::boxed::Box<StripsTransition>),
  /// Defines the WedgeTransition Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p:wedge"))]
  PWedge,
  #[sdk(child(qname = "p:CT_WheelTransition/p:wheel"))]
  PWheel(std::boxed::Box<WheelTransition>),
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p:wipe"))]
  PWipe(std::boxed::Box<WipeTransition>),
  #[sdk(child(qname = "p:CT_InOutTransition/p:zoom"))]
  PZoom(std::boxed::Box<ZoomTransition>),
  /// Defines the FlashTransition Class.
  #[sdk(empty_child(office2010, qname = "p:CT_Empty/p14:flash"))]
  P14Flash,
  #[sdk(child(office2010, qname = "p:CT_SideDirectionTransition/p14:vortex"))]
  P14Vortex(std::boxed::Box<crate::schemas::p14::VortexTransition>),
  #[sdk(child(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:switch"))]
  P14Switch(std::boxed::Box<crate::schemas::p14::SwitchTransition>),
  #[sdk(child(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:flip"))]
  P14Flip(std::boxed::Box<crate::schemas::p14::FlipTransition>),
  #[sdk(child(office2010, qname = "p14:CT_RippleTransition/p14:ripple"))]
  P14Ripple(std::boxed::Box<crate::schemas::p14::RippleTransition>),
  #[sdk(child(office2010, qname = "p14:CT_GlitterTransition/p14:glitter"))]
  P14Glitter(std::boxed::Box<crate::schemas::p14::GlitterTransition>),
  /// Defines the HoneycombTransition Class.
  #[sdk(empty_child(office2010, qname = "p:CT_Empty/p14:honeycomb"))]
  P14Honeycomb,
  #[sdk(child(office2010, qname = "p14:CT_PrismTransition/p14:prism"))]
  P14Prism(std::boxed::Box<crate::schemas::p14::PrismTransition>),
  #[sdk(child(office2010, qname = "p:CT_OrientationTransition/p14:doors"))]
  P14Doors(std::boxed::Box<crate::schemas::p14::DoorsTransition>),
  #[sdk(child(office2010, qname = "p:CT_OrientationTransition/p14:window"))]
  P14Window(std::boxed::Box<crate::schemas::p14::WindowTransition>),
  #[sdk(child(office2010, qname = "p14:CT_ShredTransition/p14:shred"))]
  P14Shred(std::boxed::Box<crate::schemas::p14::ShredTransition>),
  #[sdk(child(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:ferris"))]
  P14Ferris(std::boxed::Box<crate::schemas::p14::FerrisTransition>),
  #[sdk(child(office2010, qname = "p14:CT_FlyThroughTransition/p14:flythrough"))]
  P14Flythrough(std::boxed::Box<crate::schemas::p14::FlythroughTransition>),
  #[sdk(child(office2010, qname = "p:CT_InOutTransition/p14:warp"))]
  P14Warp(std::boxed::Box<crate::schemas::p14::WarpTransition>),
  #[sdk(child(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:gallery"))]
  P14Gallery(std::boxed::Box<crate::schemas::p14::GalleryTransition>),
  #[sdk(child(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:conveyor"))]
  P14Conveyor(std::boxed::Box<crate::schemas::p14::ConveyorTransition>),
  #[sdk(child(office2010, qname = "p:CT_SideDirectionTransition/p14:pan"))]
  P14Pan(std::boxed::Box<crate::schemas::p14::PanTransition>),
  #[sdk(child(office2010, qname = "p14:CT_RevealTransition/p14:reveal"))]
  P14Reveal(std::boxed::Box<crate::schemas::p14::RevealTransition>),
  #[sdk(child(office2010, qname = "p:CT_WheelTransition/p14:wheelReverse"))]
  P14WheelReverse(std::boxed::Box<crate::schemas::p14::WheelReverseTransition>),
  #[sdk(child(office2013, qname = "p15:CT_PresetTransition/p15:prstTrans"))]
  P15PrstTrans(std::boxed::Box<crate::schemas::p15::PresetTransition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundChoice {
  /// Background Properties.
  #[sdk(child(qname = "p:CT_BackgroundProperties/p:bgPr"))]
  PBgPr(std::boxed::Box<BackgroundProperties>),
  /// Background Style Reference.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/p:bgRef"))]
  PBgRef(std::boxed::Box<BackgroundStyleReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeTreeChoice {
  #[sdk(child(qname = "p:CT_Shape/p:sp"))]
  PSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "p:CT_GroupShape/p:grpSp"))]
  PGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "p:CT_GraphicalObjectFrame/p:graphicFrame"))]
  PGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "p:CT_Connector/p:cxnSp"))]
  PCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  PPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "p:CT_ContentPart/p:contentPart"))]
  PContentPart(std::boxed::Box<ContentPart>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(qname = "p:CT_Shape/p:sp"))]
  PSp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "p:CT_GroupShape/p:grpSp"))]
  PGrpSp(std::boxed::Box<GroupShape>),
  #[sdk(child(qname = "p:CT_GraphicalObjectFrame/p:graphicFrame"))]
  PGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "p:CT_Connector/p:cxnSp"))]
  PCxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  PPic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "p:CT_ContentPart/p:contentPart"))]
  PContentPart(std::boxed::Box<ContentPart>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeTargetChoice {
  /// Background.
  #[sdk(empty_child(qname = "p:CT_Empty/p:bg"))]
  PBg,
  /// Subshape.
  #[sdk(child(qname = "p:CT_TLSubShapeId/p:subSp"))]
  PSubSp(std::boxed::Box<SubShape>),
  /// OLE Chart Element.
  #[sdk(child(qname = "p:CT_TLOleChartTargetElement/p:oleChartEl"))]
  POleChartEl(std::boxed::Box<OleChartElement>),
  /// Text Element.
  #[sdk(child(qname = "p:CT_TLTextTargetElement/p:txEl"))]
  PTxEl(std::boxed::Box<TextElement>),
  /// Graphic Element.
  #[sdk(child(qname = "a:CT_AnimationElementChoice/p:graphicEl"))]
  PGraphicEl(std::boxed::Box<GraphicElement>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentAuthorExtensionChoice {
  /// Defines the PresenceInfo Class.
  #[sdk(child(office2013, qname = "p15:CT_PresenceInfo/p15:presenceInfo"))]
  P15PresenceInfo(std::boxed::Box<crate::schemas::p15::PresenceInfo>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentExtensionChoice {
  /// Defines the ThreadingInfo Class.
  #[sdk(child(office2013, qname = "p15:CT_CommentThreading/p15:threadingInfo"))]
  P15ThreadingInfo(std::boxed::Box<crate::schemas::p15::ThreadingInfo>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideLayoutExtensionChoice {
  /// Defines the SlideGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HandoutMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NotesMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ApplicationNonVisualDrawingPropertiesExtensionChoice {
  /// Defines the Media Class.
  #[sdk(child(office2010, qname = "p14:CT_Media/p14:media"))]
  P14Media(std::boxed::Box<crate::schemas::p14::Media>),
  /// Defines the ModificationId Class.
  #[sdk(child(office2010, qname = "p14:CT_RandomId/p14:modId"))]
  P14ModId(std::boxed::Box<crate::schemas::p14::ModificationId>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum IterateChoice {
  /// Time Absolute.
  #[sdk(child(qname = "p:CT_TLIterateIntervalTime/p:tmAbs"))]
  PTmAbs(std::boxed::Box<TimeAbsolute>),
  /// Time Percentage.
  #[sdk(child(qname = "p:CT_TLIterateIntervalPercentage/p:tmPct"))]
  PTmPct(std::boxed::Box<TimePercentage>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChildTimeNodeListChoice {
  /// Parallel Time Node.
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  PPar(std::boxed::Box<ParallelTimeNode>),
  /// Sequence Time Node.
  #[sdk(child(qname = "p:CT_TLTimeNodeSequence/p:seq"))]
  PSeq(std::boxed::Box<SequenceTimeNode>),
  /// Exclusive.
  #[sdk(child(qname = "p:CT_TLTimeNodeExclusive/p:excl"))]
  PExcl(std::boxed::Box<ExclusiveTimeNode>),
  /// Animate.
  #[sdk(child(qname = "p:CT_TLAnimateBehavior/p:anim"))]
  PAnim(std::boxed::Box<Animate>),
  /// Animate Color Behavior.
  #[sdk(child(qname = "p:CT_TLAnimateColorBehavior/p:animClr"))]
  PAnimClr(std::boxed::Box<AnimateColor>),
  /// Animate Effect.
  #[sdk(child(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect"))]
  PAnimEffect(std::boxed::Box<AnimateEffect>),
  /// Animate Motion.
  #[sdk(child(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion"))]
  PAnimMotion(std::boxed::Box<AnimateMotion>),
  /// Animate Rotation.
  #[sdk(child(qname = "p:CT_TLAnimateRotationBehavior/p:animRot"))]
  PAnimRot(std::boxed::Box<AnimateRotation>),
  /// Animate Scale.
  #[sdk(child(qname = "p:CT_TLAnimateScaleBehavior/p:animScale"))]
  PAnimScale(std::boxed::Box<AnimateScale>),
  /// Command.
  #[sdk(child(qname = "p:CT_TLCommandBehavior/p:cmd"))]
  PCmd(std::boxed::Box<Command>),
  /// Set Time Node Behavior.
  #[sdk(child(qname = "p:CT_TLSetBehavior/p:set"))]
  PSet(std::boxed::Box<SetBehavior>),
  /// Audio.
  #[sdk(child(qname = "p:CT_TLMediaNodeAudio/p:audio"))]
  PAudio(std::boxed::Box<Audio>),
  /// Video.
  #[sdk(child(qname = "p:CT_TLMediaNodeVideo/p:video"))]
  PVideo(std::boxed::Box<Video>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubTimeNodeListChoice {
  /// Parallel Time Node.
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  PPar(std::boxed::Box<ParallelTimeNode>),
  /// Sequence Time Node.
  #[sdk(child(qname = "p:CT_TLTimeNodeSequence/p:seq"))]
  PSeq(std::boxed::Box<SequenceTimeNode>),
  /// Exclusive.
  #[sdk(child(qname = "p:CT_TLTimeNodeExclusive/p:excl"))]
  PExcl(std::boxed::Box<ExclusiveTimeNode>),
  /// Animate.
  #[sdk(child(qname = "p:CT_TLAnimateBehavior/p:anim"))]
  PAnim(std::boxed::Box<Animate>),
  /// Animate Color Behavior.
  #[sdk(child(qname = "p:CT_TLAnimateColorBehavior/p:animClr"))]
  PAnimClr(std::boxed::Box<AnimateColor>),
  /// Animate Effect.
  #[sdk(child(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect"))]
  PAnimEffect(std::boxed::Box<AnimateEffect>),
  /// Animate Motion.
  #[sdk(child(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion"))]
  PAnimMotion(std::boxed::Box<AnimateMotion>),
  /// Animate Rotation.
  #[sdk(child(qname = "p:CT_TLAnimateRotationBehavior/p:animRot"))]
  PAnimRot(std::boxed::Box<AnimateRotation>),
  /// Animate Scale.
  #[sdk(child(qname = "p:CT_TLAnimateScaleBehavior/p:animScale"))]
  PAnimScale(std::boxed::Box<AnimateScale>),
  /// Command.
  #[sdk(child(qname = "p:CT_TLCommandBehavior/p:cmd"))]
  PCmd(std::boxed::Box<Command>),
  /// Set Time Node Behavior.
  #[sdk(child(qname = "p:CT_TLSetBehavior/p:set"))]
  PSet(std::boxed::Box<SetBehavior>),
  /// Audio.
  #[sdk(child(qname = "p:CT_TLMediaNodeAudio/p:audio"))]
  PAudio(std::boxed::Box<Audio>),
  /// Video.
  #[sdk(child(qname = "p:CT_TLMediaNodeVideo/p:video"))]
  PVideo(std::boxed::Box<Video>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresentationExtensionChoice {
  /// Defines the SectionProperties Class.
  #[sdk(child(office2010, qname = "p14:CT_SectionProperties/p14:sectionPr"))]
  P14SectionPr(std::boxed::Box<crate::schemas::p14::SectionProperties>),
  /// Defines the SectionList Class.
  #[sdk(child(office2010, qname = "p14:CT_SectionList/p14:sectionLst"))]
  P14SectionLst(std::boxed::Box<crate::schemas::p14::SectionList>),
  /// Defines the SlideGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  /// Defines the NotesGuideList Class.
  #[sdk(child(office2013, qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst"))]
  P15NotesGuideLst(std::boxed::Box<crate::schemas::p15::NotesGuideList>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HtmlPublishPropertiesChoice {
  /// All Slides.
  #[sdk(empty_child(qname = "p:CT_Empty/p:sldAll"))]
  PSldAll,
  #[sdk(child(qname = "p:CT_IndexRange/p:sldRg"))]
  PSldRg(std::boxed::Box<SlideRange>),
  #[sdk(child(qname = "p:CT_CustomShowId/p:custShow"))]
  PCustShow(std::boxed::Box<CustomShowReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesChoice {
  /// Presenter Slide Show Mode.
  #[sdk(empty_child(qname = "p:CT_Empty/p:present"))]
  PPresent,
  /// Browse Slide Show Mode.
  #[sdk(child(qname = "p:CT_ShowInfoBrowse/p:browse"))]
  PBrowse(std::boxed::Box<BrowseSlideMode>),
  /// Kiosk Slide Show Mode.
  #[sdk(child(qname = "p:CT_ShowInfoKiosk/p:kiosk"))]
  PKiosk(std::boxed::Box<KioskSlideMode>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesChoice2 {
  /// All Slides.
  #[sdk(empty_child(qname = "p:CT_Empty/p:sldAll"))]
  PSldAll,
  /// Slide Range.
  #[sdk(child(qname = "p:CT_IndexRange/p:sldRg"))]
  PSldRg(std::boxed::Box<SlideRange>),
  /// Custom Show.
  #[sdk(child(qname = "p:CT_CustomShowId/p:custShow"))]
  PCustShow(std::boxed::Box<CustomShowReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorMostRecentlyUsedChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<crate::schemas::a::PresetColor>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresentationPropertiesExtensionChoice {
  /// Defines the DiscardImageEditData Class.
  #[sdk(child(
    office2010,
    qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData"
  ))]
  P14DiscardImageEditData(std::boxed::Box<crate::schemas::p14::DiscardImageEditData>),
  /// Defines the DefaultImageDpi Class.
  #[sdk(child(office2010, qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi"))]
  P14DefaultImageDpi(std::boxed::Box<crate::schemas::p14::DefaultImageDpi>),
  /// Defines the TextMath Class.
  #[sdk(empty_child(office2010, qname = "a14:CT_TextMath/a14:m"))]
  A14M,
  /// Defines the ChartTrackingReferenceBased Class.
  #[sdk(child(
    office2013,
    qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased"
  ))]
  P15ChartTrackingRefBased(std::boxed::Box<crate::schemas::p15::ChartTrackingReferenceBased>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextElementChoice {
  /// Character Range.
  #[sdk(child(qname = "p:CT_IndexRange/p:charRg"))]
  PCharRg(std::boxed::Box<CharRange>),
  /// Paragraph Text Range.
  #[sdk(child(qname = "p:CT_IndexRange/p:pRg"))]
  PPRg(std::boxed::Box<ParagraphIndexRange>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GraphicElementChoice {
  /// Diagram to Animate.
  #[sdk(child(qname = "a:CT_AnimationDgmElement/a:dgm"))]
  ADgm(std::boxed::Box<crate::schemas::a::Diagram>),
  /// Chart to Animate.
  #[sdk(child(qname = "a:CT_AnimationChartElement/a:chart"))]
  AChart(std::boxed::Box<crate::schemas::a::Chart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SoundActionChoice {
  /// Start Sound Action.
  #[sdk(child(qname = "p:CT_TransitionStartSoundAction/p:stSnd"))]
  PStSnd(std::boxed::Box<StartSoundAction>),
  /// Stop Sound Action.
  #[sdk(empty_child(qname = "p:CT_Empty/p:endSnd"))]
  PEndSnd,
}
