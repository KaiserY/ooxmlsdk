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
#[sdk(qname = "p:sldRg")]
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
#[sdk(qname = "p:charRg")]
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
#[sdk(qname = "p:pRg")]
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
#[sdk(qname = "p:custShow")]
pub struct CustomShowReference {
  /// Custom Show Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct Extension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
/// Browse Slide Show Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:browse")]
pub struct BrowseSlideMode {
  /// Show Scroll Bar in Window
  #[sdk(attr(qname = ":showScrollbar"))]
  pub show_scrollbar: Option<crate::simple_type::BooleanValue>,
}
/// Kiosk Slide Show Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:kiosk")]
pub struct KioskSlideMode {
  /// Restart Show
  #[sdk(attr(qname = ":restart"))]
  pub restart: Option<crate::simple_type::UInt32Value>,
}
/// Color Scheme Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:clrMap")]
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
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Color Scheme Map Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:clrMapOvr")]
pub struct ColorMapOverride {
  #[sdk(
        choice(
            empty_child(variant = MasterColorMapping, qname = "a:masterClrMapping"),
            child(variant = OverrideColorMapping, boxed, qname = "a:overrideClrMapping")
        )
    )]
  pub color_map_override_choice: Option<ColorMapOverrideChoice>,
}
/// Background Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bgPr")]
pub struct BackgroundProperties {
  /// Shade to Title
  #[sdk(attr(qname = ":shadeToTitle"))]
  pub shade_to_title: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = BlipFill, boxed, qname = "a:blipFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill")
        )
    )]
  pub background_properties_choice1: Option<BackgroundPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, boxed, qname = "a:effectLst"),
            child(variant = EffectDag, boxed, qname = "a:effectDag"),
            child(variant = PandocEffectsList, boxed, qname = "a:effectsLst")
        )
    )]
  pub background_properties_choice2: Option<BackgroundPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Background Style Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bgRef")]
pub struct BackgroundStyleReference {
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub background_style_reference_choice: Option<BackgroundStyleReferenceChoice>,
}
/// Data for the Windows platform..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(version = office2021, qname = "p:ext")]
pub struct CommentPropertiesExtension {
  /// Defines the TaskDetails Class.
  #[sdk(child(qname = "p228:taskDetails"))]
  pub task_details: Option<std::boxed::Box<crate::schemas::p228::TaskDetails>>,
  /// Defines the Reactions Class.
  #[sdk(child(qname = "p223:reactions"))]
  pub reactions: Option<crate::schemas::p223::Reactions>,
}
/// List of Comment Authors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:cmAuthorLst")]
pub struct CommentAuthorList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Comment Author.
  #[sdk(child(qname = "p:cmAuthor"))]
  pub comment_author: Vec<CommentAuthor>,
}
/// Comment List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Comment.
  #[sdk(child(qname = "p:cm"))]
  pub comment: Vec<Comment>,
}
/// Global Element for OLE Objects and Controls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:oleObj")]
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
  #[sdk(
        choice(
            child(variant = OleObjectEmbed, boxed, qname = "p:embed"),
            child(variant = OleObjectLink, boxed, qname = "p:link")
        )
    )]
  pub ole_object_choice: Option<OleObjectChoice>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "p:pic"))]
  pub picture: Option<std::boxed::Box<Picture>>,
}
/// Presentation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:presentation")]
pub struct Presentation {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// serverZoom
  #[sdk(attr(qname = ":serverZoom"))]
  pub server_zoom: Option<crate::simple_type::DrawingmlPercentageValue>,
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
  #[sdk(child(qname = "p:sldMasterIdLst"))]
  pub slide_master_id_list: Option<SlideMasterIdList>,
  /// Defines the NotesMasterIdList Class.
  #[sdk(child(qname = "p:notesMasterIdLst"))]
  pub notes_master_id_list: Option<std::boxed::Box<NotesMasterIdList>>,
  /// Defines the HandoutMasterIdList Class.
  #[sdk(child(qname = "p:handoutMasterIdLst"))]
  pub handout_master_id_list: Option<std::boxed::Box<HandoutMasterIdList>>,
  /// Defines the SlideIdList Class.
  #[sdk(child(qname = "p:sldIdLst"))]
  pub slide_id_list: Option<SlideIdList>,
  /// Defines the SlideSize Class.
  #[sdk(child(qname = "p:sldSz"))]
  pub slide_size: Option<SlideSize>,
  /// Defines the NotesSize Class.
  #[sdk(child(qname = "p:notesSz"))]
  pub notes_size: NotesSize,
  /// Defines the EmbeddedFontList Class.
  #[sdk(child(qname = "p:embeddedFontLst"))]
  pub embedded_font_list: Option<EmbeddedFontList>,
  /// Defines the CustomShowList Class.
  #[sdk(child(qname = "p:custShowLst"))]
  pub custom_show_list: Option<CustomShowList>,
  /// Defines the PhotoAlbum Class.
  #[sdk(child(qname = "p:photoAlbum"))]
  pub photo_album: Option<std::boxed::Box<PhotoAlbum>>,
  /// Customer Data List.
  #[sdk(child(qname = "p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// Defines the Kinsoku Class.
  #[sdk(child(qname = "p:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// Defines the DefaultTextStyle Class.
  #[sdk(child(qname = "p:defaultTextStyle"))]
  pub default_text_style: Option<std::boxed::Box<DefaultTextStyle>>,
  /// Defines the ModificationVerifier Class.
  #[sdk(child(qname = "p:modifyVerifier"))]
  pub modification_verifier: Option<ModificationVerifier>,
  /// Defines the PresentationExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub presentation_extension_list: Option<PresentationExtensionList>,
}
/// Presentation-wide Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:presentationPr")]
pub struct PresentationProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// HTML Publishing Properties
  #[sdk(child(qname = "p:htmlPubPr"))]
  pub html_publish_properties: Option<std::boxed::Box<HtmlPublishProperties>>,
  /// Web Properties
  #[sdk(child(qname = "p:webPr"))]
  pub web_properties: Option<std::boxed::Box<WebProperties>>,
  /// Defines the PrintingProperties Class.
  #[sdk(child(qname = "p:prnPr"))]
  pub printing_properties: Option<std::boxed::Box<PrintingProperties>>,
  /// Defines the ShowProperties Class.
  #[sdk(child(qname = "p:showPr"))]
  pub show_properties: Option<std::boxed::Box<ShowProperties>>,
  /// Defines the ColorMostRecentlyUsed Class.
  #[sdk(child(qname = "p:clrMru"))]
  pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
  /// Defines the PresentationPropertiesExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub presentation_properties_extension_list: Option<PresentationPropertiesExtensionList>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:sld")]
pub struct Slide {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
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
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Markup Compatibility alternate content at this schema position.
  #[sdk(mce(qname = "mc:AlternateContent", children = [transition]))]
  pub alternate_content: Vec<crate::schemas::mc::AlternateContent>,
  /// Slide Transition
  #[sdk(child(qname = "p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide
  #[sdk(child(qname = "p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the SlideExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub slide_extension_list: Option<SlideExtensionList>,
}
/// Slide Layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:sldLayout")]
pub struct SlideLayout {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
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
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Markup Compatibility alternate content at this schema position.
  #[sdk(mce(qname = "mc:AlternateContent", children = [transition]))]
  pub alternate_content: Vec<crate::schemas::mc::AlternateContent>,
  /// Slide Transition.
  #[sdk(child(qname = "p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide.
  #[sdk(child(qname = "p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the SlideLayoutExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub slide_layout_extension_list: Option<SlideLayoutExtensionList>,
}
/// Slide Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:sldMaster")]
pub struct SlideMaster {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// preserve
  #[sdk(attr(qname = ":preserve"))]
  pub preserve: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the SlideLayoutIdList Class.
  #[sdk(child(qname = "p:sldLayoutIdLst"))]
  pub slide_layout_id_list: Option<SlideLayoutIdList>,
  /// Markup Compatibility alternate content at this schema position.
  #[sdk(mce(qname = "mc:AlternateContent", children = [transition]))]
  pub alternate_content: Vec<crate::schemas::mc::AlternateContent>,
  /// Slide Transition.
  #[sdk(child(qname = "p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// Slide Timing Information for a Slide.
  #[sdk(child(qname = "p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the TextStyles Class.
  #[sdk(child(qname = "p:txStyles"))]
  pub text_styles: Option<std::boxed::Box<TextStyles>>,
  /// Defines the SlideMasterExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub slide_master_extension_list: Option<SlideMasterExtensionList>,
}
/// Handout Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:handoutMaster")]
pub struct HandoutMaster {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the HandoutMasterExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub handout_master_extension_list: Option<HandoutMasterExtensionList>,
}
/// Notes Master.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:notesMaster")]
pub struct NotesMaster {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// Common slide data for notes slides.
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map
  #[sdk(child(qname = "p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Defines the NotesStyle Class.
  #[sdk(child(qname = "p:notesStyle"))]
  pub notes_style: Option<std::boxed::Box<NotesStyle>>,
  /// Defines the NotesMasterExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub notes_master_extension_list: Option<NotesMasterExtensionList>,
}
/// Notes Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:notes")]
pub struct NotesSlide {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// Show Master Shapes
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// Common slide data for notes slides
  #[sdk(child(qname = "p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  /// Color Scheme Map Override
  #[sdk(child(qname = "p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Slide Synchronization Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:sldSyncPr")]
pub struct SlideSyncProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Programmable Tab List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:tagLst")]
pub struct TagList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Programmable Extensibility Tag.
  #[sdk(child(qname = "p:tag"))]
  pub tag: Vec<Tag>,
}
/// Presentation-wide View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "p:viewPr")]
pub struct ViewProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
  /// Last View
  #[sdk(attr(qname = ":lastView"))]
  #[sdk(string_format(kind = "token"))]
  pub last_view: Option<ViewValues>,
  /// Show Comments
  #[sdk(attr(qname = ":showComments"))]
  pub show_comments: Option<crate::simple_type::BooleanValue>,
  /// Normal View Properties
  #[sdk(child(qname = "p:normalViewPr"))]
  pub normal_view_properties: Option<std::boxed::Box<NormalViewProperties>>,
  /// Slide View Properties
  #[sdk(child(qname = "p:slideViewPr"))]
  pub slide_view_properties: Option<std::boxed::Box<SlideViewProperties>>,
  /// Outline View Properties
  #[sdk(child(qname = "p:outlineViewPr"))]
  pub outline_view_properties: Option<std::boxed::Box<OutlineViewProperties>>,
  /// Notes Text View Properties
  #[sdk(child(qname = "p:notesTextViewPr"))]
  pub notes_text_view_properties: Option<std::boxed::Box<NotesTextViewProperties>>,
  /// Slide Sorter View Properties
  #[sdk(child(qname = "p:sorterViewPr"))]
  pub sorter_view_properties: Option<std::boxed::Box<SorterViewProperties>>,
  /// Notes View Properties
  #[sdk(child(qname = "p:notesViewPr"))]
  pub notes_view_properties: Option<std::boxed::Box<NotesViewProperties>>,
  /// Grid Spacing
  #[sdk(child(qname = "p:gridSpacing"))]
  pub grid_spacing: Option<GridSpacing>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(version = office2010, qname = "p:contentPart")]
pub struct ContentPart {
  /// bwMode
  #[sdk(attr(qname = "p14:bwMode"))]
  pub p14_bw_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// Defines the NonVisualContentPartProperties Class.
  #[sdk(child(qname = "p14:nvContentPartPr"))]
  pub non_visual_content_part_properties:
    Option<std::boxed::Box<crate::schemas::p14::NonVisualContentPartProperties>>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "p14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::p14::Transform2D>>,
  /// Defines the ExtensionListModify Class.
  #[sdk(child(qname = "p14:extLst"))]
  pub extension_list_modify: Option<crate::schemas::p14::ExtensionListModify>,
}
/// Sound.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:snd")]
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
#[sdk(qname = "p:sndTgt")]
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
#[sdk(qname = "p:stSnd")]
pub struct StartSoundAction {
  /// Loop Sound
  #[sdk(attr(qname = ":loop"))]
  pub r#loop: Option<crate::simple_type::BooleanValue>,
  /// Sound
  #[sdk(child(qname = "p:snd"))]
  pub sound: std::boxed::Box<Sound>,
}
/// Time Absolute.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tmAbs")]
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
#[sdk(qname = "p:tmPct")]
pub struct TimePercentage {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::PositiveDrawingmlPercentageValue,
}
/// Target Element Trigger Choice.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tgtEl")]
pub struct TargetElement {
  #[sdk(
        choice(
            empty_child(variant = SlideTarget, qname = "p:sldTgt"),
            child(variant = SoundTarget, qname = "p:sndTgt"),
            child(variant = ShapeTarget, boxed, qname = "p:spTgt"),
            child(variant = InkTarget, qname = "p:inkTgt"),
            child(variant = BookmarkTarget, qname = "p14:bmkTgt")
        )
    )]
  pub target_element_choice: Option<TargetElementChoice>,
}
/// Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tn")]
pub struct TimeNode {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Runtime Node Trigger Choice.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:rtn")]
pub struct RuntimeNodeTrigger {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: TriggerRuntimeNodeValues,
}
/// Condition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cond")]
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
  #[sdk(
        choice(
            child(variant = TargetElement, boxed, qname = "p:tgtEl"),
            child(variant = TimeNode, qname = "p:tn"),
            child(variant = RuntimeNodeTrigger, qname = "p:rtn")
        )
    )]
  pub condition_choice: Option<ConditionChoice>,
}
/// Defines the EndSync Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:endSync")]
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
  #[sdk(
        choice(
            child(variant = TargetElement, boxed, qname = "p:tgtEl"),
            child(variant = TimeNode, qname = "p:tn"),
            child(variant = RuntimeNodeTrigger, qname = "p:rtn")
        )
    )]
  pub end_sync_choice: Option<EndSyncChoice>,
}
/// Parallel Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:par")]
pub struct ParallelTimeNode {
  /// Parallel TimeNode
  #[sdk(child(qname = "p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Sequence Time Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:seq")]
pub struct SequenceTimeNode {
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
  #[sdk(child(qname = "p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Previous Conditions List
  #[sdk(child(qname = "p:prevCondLst"))]
  pub previous_condition_list: Option<PreviousConditionList>,
  /// Next Conditions List
  #[sdk(child(qname = "p:nextCondLst"))]
  pub next_condition_list: Option<NextConditionList>,
}
/// Exclusive.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:excl")]
pub struct ExclusiveTimeNode {
  /// Common TimeNode Properties
  #[sdk(child(qname = "p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:anim")]
pub struct Animate {
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
  #[sdk(attr(qname = "p14:bounceEnd"))]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the TimeAnimateValueList Class.
  #[sdk(child(qname = "p:tavLst"))]
  pub time_animate_value_list: Option<TimeAnimateValueList>,
}
/// Animate Color Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:animClr")]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// By
  #[sdk(child(qname = "p:by"))]
  pub by_color: Option<std::boxed::Box<ByColor>>,
  /// From
  #[sdk(child(qname = "p:from"))]
  pub from_color: Option<std::boxed::Box<FromColor>>,
  /// To
  #[sdk(child(qname = "p:to"))]
  pub to_color: Option<std::boxed::Box<ToColor>>,
}
/// Animate Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:animEffect")]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Progress
  #[sdk(child(qname = "p:progress"))]
  pub progress: Option<std::boxed::Box<Progress>>,
}
/// Animate Motion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:animMotion")]
pub struct AnimateMotion {
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
  pub relative_angle: Option<crate::simple_type::DrawingmlAngleValue>,
  /// ptsTypes
  #[sdk(attr(qname = ":ptsTypes"))]
  pub point_types: Option<crate::simple_type::StringValue>,
  /// bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the ByPosition Class.
  #[sdk(child(qname = "p:by"))]
  pub by_position: Option<ByPosition>,
  /// Defines the FromPosition Class.
  #[sdk(child(qname = "p:from"))]
  pub from_position: Option<FromPosition>,
  /// Defines the ToPosition Class.
  #[sdk(child(qname = "p:to"))]
  pub to_position: Option<ToPosition>,
  /// Defines the RotationCenter Class.
  #[sdk(child(qname = "p:rCtr"))]
  pub rotation_center: Option<RotationCenter>,
}
/// Animate Rotation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:animRot")]
pub struct AnimateRotation {
  /// by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::DrawingmlAngleValue>,
  /// from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::DrawingmlAngleValue>,
  /// to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::DrawingmlAngleValue>,
  /// bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Animate Scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:animScale")]
pub struct AnimateScale {
  /// zoomContents
  #[sdk(attr(qname = ":zoomContents"))]
  pub zoom_contents: Option<crate::simple_type::BooleanValue>,
  /// bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
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
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// Defines the ByPosition Class.
  #[sdk(child(qname = "p:by"))]
  pub by_position: Option<ByPosition>,
  /// Defines the FromPosition Class.
  #[sdk(child(qname = "p:from"))]
  pub from_position: Option<FromPosition>,
  /// Defines the ToPosition Class.
  #[sdk(child(qname = "p:to"))]
  pub to_position: Option<ToPosition>,
}
/// Command.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cmd")]
pub struct Command {
  /// Command Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<CommandValues>,
  /// Command
  #[sdk(attr(qname = ":cmd"))]
  pub command_name: Option<crate::simple_type::StringValue>,
  /// Defines the CommonBehavior Class.
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Set Time Node Behavior.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:set")]
pub struct SetBehavior {
  /// Common Behavior
  #[sdk(child(qname = "p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// To
  #[sdk(child(qname = "p:to"))]
  pub to_variant_value: Option<std::boxed::Box<ToVariantValue>>,
}
/// Audio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:audio")]
pub struct Audio {
  /// Is Narration
  #[sdk(attr(qname = ":isNarration"))]
  pub is_narration: Option<crate::simple_type::BooleanValue>,
  /// Common Media Node Properties
  #[sdk(child(qname = "p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Video.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:video")]
pub struct Video {
  /// Full Screen
  #[sdk(attr(qname = ":fullScrn"))]
  pub full_screen: Option<crate::simple_type::BooleanValue>,
  /// Common Media Node Properties
  #[sdk(child(qname = "p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Parallel TimeNode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cTn")]
pub struct CommonTimeNode {
  pub xmlns: Vec<crate::common::XmlNamespace>,
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
  pub speed: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// accel
  #[sdk(attr(qname = ":accel"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub acceleration: Option<crate::simple_type::PositiveFixedPercentageValue>,
  /// decel
  #[sdk(attr(qname = ":decel"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub deceleration: Option<crate::simple_type::PositiveFixedPercentageValue>,
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
  #[sdk(attr(qname = "p14:presetBounceEnd"))]
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
  #[sdk(child(qname = "p:stCondLst"))]
  pub start_condition_list: Option<StartConditionList>,
  /// Defines the EndConditionList Class.
  #[sdk(child(qname = "p:endCondLst"))]
  pub end_condition_list: Option<EndConditionList>,
  /// Defines the EndSync Class.
  #[sdk(child(qname = "p:endSync"))]
  pub end_sync: Option<std::boxed::Box<EndSync>>,
  /// Defines the Iterate Class.
  #[sdk(child(qname = "p:iterate"))]
  pub iterate: Option<std::boxed::Box<Iterate>>,
  /// Defines the ChildTimeNodeList Class.
  #[sdk(child(qname = "p:childTnLst"))]
  pub child_time_node_list: Option<ChildTimeNodeList>,
  /// Defines the SubTimeNodeList Class.
  #[sdk(child(qname = "p:subTnLst"))]
  pub sub_time_node_list: Option<SubTimeNodeList>,
}
/// Previous Conditions List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:prevCondLst")]
pub struct PreviousConditionList {
  /// Condition.
  #[sdk(child(qname = "p:cond"))]
  pub condition: Vec<Condition>,
}
/// Next Conditions List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nextCondLst")]
pub struct NextConditionList {
  /// Condition.
  #[sdk(child(qname = "p:cond"))]
  pub condition: Vec<Condition>,
}
/// Defines the StartConditionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:stCondLst")]
pub struct StartConditionList {
  /// Condition.
  #[sdk(child(qname = "p:cond"))]
  pub condition: Vec<Condition>,
}
/// Defines the EndConditionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:endCondLst")]
pub struct EndConditionList {
  /// Condition.
  #[sdk(child(qname = "p:cond"))]
  pub condition: Vec<Condition>,
}
/// Attribute Name.
pub type AttributeName = crate::simple_type::StringValue;
/// Defines the Text Class.
pub type Text = crate::simple_type::StringValue;
/// Attribute Name List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:attrNameLst")]
pub struct AttributeNameList {
  /// Attribute Name.
  #[sdk(text_child(qname = "p:attrName"))]
  pub attribute_name: Vec<AttributeName>,
}
/// Boolean Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:boolVal")]
pub struct BooleanVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Integer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:intVal")]
pub struct IntegerVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Float Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:fltVal")]
pub struct FloatVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::SingleValue,
}
/// String Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:strVal")]
pub struct StringVariantValue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Color Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:clrVal")]
pub struct ColorValue {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_value_choice: Option<ColorValueChoice>,
}
/// Pen Color for Slide Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:penClr")]
pub struct PenColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub pen_color_choice: Option<PenColorChoice>,
}
/// Time Animate Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tav")]
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
  #[sdk(child(qname = "p:val"))]
  pub variant_value: Option<std::boxed::Box<VariantValue>>,
}
/// RGB.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:rgb")]
pub struct RgbColor {
  /// Red
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub red: crate::simple_type::FixedPercentageValue,
  /// Green
  #[sdk(attr(qname = ":g"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub green: crate::simple_type::FixedPercentageValue,
  /// Blue
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub blue: crate::simple_type::FixedPercentageValue,
}
/// HSL.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:hsl")]
pub struct HslColor {
  /// Hue
  #[sdk(attr(qname = ":h"))]
  pub hue: crate::simple_type::DrawingmlAngleValue,
  /// Saturation
  #[sdk(attr(qname = ":s"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub saturation: crate::simple_type::FixedPercentageValue,
  /// Lightness
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub lightness: crate::simple_type::FixedPercentageValue,
}
/// Defines the CommonBehavior Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cBhvr")]
pub struct CommonBehavior {
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
  #[sdk(child(qname = "p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Target Element
  #[sdk(child(qname = "p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
  /// Attribute Name List
  #[sdk(child(qname = "p:attrNameLst"))]
  pub attribute_name_list: Option<AttributeNameList>,
}
/// Progress.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:progress")]
pub struct Progress {
  /// Float Value.
  #[sdk(child(qname = "p:fltVal"))]
  pub float_variant_value: FloatVariantValue,
}
/// To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:to")]
pub struct ToVariantValue {
  #[sdk(
        choice(
            child(variant = BooleanVariantValue, qname = "p:boolVal"),
            child(variant = IntegerVariantValue, qname = "p:intVal"),
            child(variant = FloatVariantValue, qname = "p:fltVal"),
            child(variant = StringVariantValue, qname = "p:strVal"),
            child(variant = ColorValue, boxed, qname = "p:clrVal")
        )
    )]
  pub to_variant_value_choice: Option<ToVariantValueChoice>,
}
/// Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:val")]
pub struct VariantValue {
  #[sdk(
        choice(
            child(variant = BooleanVariantValue, qname = "p:boolVal"),
            child(variant = IntegerVariantValue, qname = "p:intVal"),
            child(variant = FloatVariantValue, qname = "p:fltVal"),
            child(variant = StringVariantValue, qname = "p:strVal"),
            child(variant = ColorValue, boxed, qname = "p:clrVal")
        )
    )]
  pub variant_value_choice: Option<VariantValueChoice>,
}
/// Common Media Node Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cMediaNode")]
pub struct CommonMediaNode {
  /// Volume
  #[sdk(attr(qname = ":vol"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub volume: Option<crate::simple_type::PositiveFixedPercentageValue>,
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
  #[sdk(child(qname = "p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// Target Element Trigger Choice.
  #[sdk(child(qname = "p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
}
/// Time Node List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tnLst")]
pub struct TimeNodeList {
  /// Parallel Time Node.
  #[sdk(child(qname = "p:par"))]
  pub parallel_time_node: std::boxed::Box<ParallelTimeNode>,
}
/// Template Effects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tmpl")]
pub struct Template {
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// Time Node List
  #[sdk(child(qname = "p:tnLst"))]
  pub time_node_list: std::boxed::Box<TimeNodeList>,
}
/// Template effects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tmplLst")]
pub struct TemplateList {
  /// Template Effects.
  #[sdk(child(qname = "p:tmpl"))]
  pub template: Vec<Template>,
}
/// Build Sub Elements.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bldSub")]
pub struct BuildSubElement {
  #[sdk(
        choice(
            child(variant = BuildDiagram, qname = "a:bldDgm"),
            child(variant = BuildChart, qname = "a:bldChart")
        )
    )]
  pub build_sub_element_choice: Option<BuildSubElementChoice>,
}
/// Build Paragraph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bldP")]
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
  #[sdk(child(qname = "p:tmplLst"))]
  pub template_list: Option<TemplateList>,
}
/// Build Diagram.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bldDgm")]
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
#[sdk(qname = "p:bldOleChart")]
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
#[sdk(qname = "p:bldGraphic")]
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
  #[sdk(
        choice(
            empty_child(variant = BuildAsOne, qname = "p:bldAsOne"),
            child(variant = BuildSubElement, boxed, qname = "p:bldSub")
        )
    )]
  pub build_graphics_choice: Option<BuildGraphicsChoice>,
}
/// Build List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bldLst")]
pub struct BuildList {
  #[sdk(
        choice(
            child(variant = BuildParagraph, boxed, qname = "p:bldP"),
            child(variant = BuildDiagram, qname = "p:bldDgm"),
            child(variant = BuildOleChart, qname = "p:bldOleChart"),
            child(variant = BuildGraphics, boxed, qname = "p:bldGraphic")
        )
    )]
  pub build_list_choice: Vec<BuildListChoice>,
}
/// Defines the ExtensionListWithModification Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct ExtensionListWithModification {
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<Extension>,
}
/// By.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:by")]
pub struct ByColor {
  #[sdk(
        choice(
            child(variant = RgbColor, qname = "p:rgb"),
            child(variant = HslColor, qname = "p:hsl")
        )
    )]
  pub by_color_choice: Option<ByColorChoice>,
}
/// From.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:from")]
pub struct FromColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub from_color_choice: Option<FromColorChoice>,
}
/// To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:to")]
pub struct ToColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub to_color_choice: Option<ToColorChoice>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sld")]
pub struct SlideListEntry {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:custData")]
pub struct CustomerData {
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data Tags.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tags")]
pub struct CustomerDataTags {
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Comment Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cmAuthor")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub comment_author_extension_list: Option<CommentAuthorExtensionList>,
}
/// Comment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cm")]
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
  #[sdk(child(qname = "p:pos"))]
  pub position: Position,
  /// Defines the Text Class.
  #[sdk(text_child(qname = "p:text"))]
  pub text: Text,
  /// Defines the CommentExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub comment_extension_list: Option<CommentExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<Extension>,
}
/// Embedded Control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:control")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "p:pic"))]
  pub picture: Option<std::boxed::Box<Picture>>,
}
/// Slide ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldId")]
pub struct SlideId {
  /// Slide Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 256..2147483648))]
  pub id: crate::simple_type::UInt32Value,
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldMasterId")]
pub struct SlideMasterId {
  /// Slide Master Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 2147483648..))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:notesMasterId")]
pub struct NotesMasterId {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Handout Master ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:handoutMasterId")]
pub struct HandoutMasterId {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Embedded Font Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:font")]
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
#[sdk(qname = "p:regular")]
pub struct RegularFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bold")]
pub struct BoldFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Italic Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:italic")]
pub struct ItalicFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Italic Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:boldItalic")]
pub struct BoldItalicFont {
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Embedded Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:embeddedFont")]
pub struct EmbeddedFont {
  /// Embedded Font Name
  #[sdk(child(qname = "p:font"))]
  pub font: std::boxed::Box<Font>,
  /// Regular Embedded Font
  #[sdk(child(qname = "p:regular"))]
  pub regular_font: Option<RegularFont>,
  /// Bold Embedded Font
  #[sdk(child(qname = "p:bold"))]
  pub bold_font: Option<BoldFont>,
  /// Italic Embedded Font
  #[sdk(child(qname = "p:italic"))]
  pub italic_font: Option<ItalicFont>,
  /// Bold Italic Embedded Font
  #[sdk(child(qname = "p:boldItalic"))]
  pub bold_italic_font: Option<BoldItalicFont>,
}
/// List of Presentation Slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldLst")]
pub struct SlideList {
  /// Presentation Slide.
  #[sdk(child(qname = "p:sld"))]
  pub slide_list_entry: Vec<SlideListEntry>,
}
/// Custom Show.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:custShow")]
pub struct CustomShow {
  /// Custom Show Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Custom Show ID
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// List of Presentation Slides
  #[sdk(child(qname = "p:sldLst"))]
  pub slide_list: SlideList,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvPr")]
pub struct NonVisualDrawingProperties {
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
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list:
    Option<crate::schemas::a::NonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Drawing Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<crate::schemas::a::ShapeLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Application Non-Visual Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// Is a Photo Album
  #[sdk(attr(qname = ":isPhoto"))]
  pub is_photo: Option<crate::simple_type::BooleanValue>,
  /// Is User Drawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  /// Placeholder Shape
  #[sdk(child(qname = "p:ph"))]
  pub placeholder_shape: Option<std::boxed::Box<PlaceholderShape>>,
  #[sdk(
        choice(
            child(variant = AudioFromCd, boxed, qname = "a:audioCd"),
            child(variant = WaveAudioFile, qname = "a:wavAudioFile"),
            child(variant = AudioFromFile, boxed, qname = "a:audioFile"),
            child(variant = VideoFromFile, boxed, qname = "a:videoFile"),
            child(variant = QuickTimeFromFile, boxed, qname = "a:quickTimeFile")
        )
    )]
  pub application_non_visual_drawing_properties_choice:
    Option<ApplicationNonVisualDrawingPropertiesChoice>,
  /// Customer Data List.
  #[sdk(child(qname = "p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub application_non_visual_drawing_properties_extension_list:
    Option<ApplicationNonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvSpPr")]
pub struct NonVisualShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Drawing Properties for a Shape
  #[sdk(child(qname = "p:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, boxed, qname = "a:custGeom"),
            child(variant = PresetGeometry, boxed, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = BlipFill, boxed, qname = "a:blipFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, boxed, qname = "a:effectLst"),
            child(variant = EffectDag, boxed, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Shape Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:lnRef"))]
  pub line_reference: std::boxed::Box<crate::schemas::a::LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:fillRef"))]
  pub fill_reference: std::boxed::Box<crate::schemas::a::FillReference>,
  /// Effect Reference.
  #[sdk(child(qname = "a:effectRef"))]
  pub effect_reference: std::boxed::Box<crate::schemas::a::EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:fontRef"))]
  pub font_reference: std::boxed::Box<crate::schemas::a::FontReference>,
}
/// Shape Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:txBody")]
pub struct TextBody {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<crate::schemas::a::Paragraph>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<crate::schemas::a::ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:stCxn"))]
  pub start_connection: Option<crate::schemas::a::StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:endCxn"))]
  pub end_connection: Option<crate::schemas::a::EndConnection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "p:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the PictureLocks Class.
  #[sdk(child(qname = "a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<crate::schemas::a::PictureLocks>>,
  /// Defines the NonVisualPicturePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_picture_properties_extension_list:
    Option<crate::schemas::a::NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Non-Visual Drawing Properties.
  #[sdk(child(qname = "p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "p:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
  /// Application Non-Visual Drawing Properties.
  #[sdk(child(qname = "p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Picture Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:blip"))]
  pub blip: Option<std::boxed::Box<crate::schemas::a::Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:srcRect"))]
  pub source_rectangle: Option<crate::schemas::a::SourceRectangle>,
  #[sdk(
        choice(
            child(variant = Tile, boxed, qname = "a:tile"),
            child(variant = Stretch, boxed, qname = "a:stretch")
        )
    )]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<crate::schemas::a::GraphicFrameLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "p:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
  /// Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// 2D Transform for Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:xfrm")]
pub struct Transform {
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
  #[sdk(child(qname = "a:off"))]
  pub offset: Option<crate::schemas::a::Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<crate::schemas::a::Extents>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<crate::schemas::a::GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<crate::schemas::a::NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Slide Master Title Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:titleStyle")]
pub struct TitleStyle {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Master Body Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bodyStyle")]
pub struct BodyStyle {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Master Other Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:otherStyle")]
pub struct OtherStyle {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the DefaultTextStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:defaultTextStyle")]
pub struct DefaultTextStyle {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the NotesStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:notesStyle")]
pub struct NotesStyle {
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties:
    Option<std::boxed::Box<crate::schemas::a::Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Slide Layout Id.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldLayoutId")]
pub struct SlideLayoutId {
  /// ID Tag
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(range = 2147483648..))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// ID Tag
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Common slide data for notes slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cSld")]
pub struct CommonSlideData {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Slide Background
  #[sdk(child(qname = "p:bg"))]
  pub background: Option<std::boxed::Box<Background>>,
  /// Shape Tree
  #[sdk(child(qname = "p:spTree"))]
  pub shape_tree: std::boxed::Box<ShapeTree>,
  /// Customer Data List
  #[sdk(child(qname = "p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// List of controls
  #[sdk(child(qname = "p:controls"))]
  pub control_list: Option<ControlList>,
  /// Defines the CommonSlideDataExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub common_slide_data_extension_list: Option<CommonSlideDataExtensionList>,
}
/// Programmable Extensibility Tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tag")]
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
#[sdk(qname = "p:restoredLeft")]
pub struct RestoredLeft {
  /// Normal View Dimension Size
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub size: crate::simple_type::PositiveFixedPercentageValue,
  /// Auto Adjust Normal View
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// Normal View Restored Top Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:restoredTop")]
pub struct RestoredTop {
  /// Normal View Dimension Size
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub size: crate::simple_type::PositiveFixedPercentageValue,
  /// Auto Adjust Normal View
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// View Scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:scale")]
pub struct ScaleFactor {
  /// Horizontal Ratio
  #[sdk(child(qname = "a:sx"))]
  pub scale_x: std::boxed::Box<crate::schemas::a::ScaleX>,
  /// Vertical Ratio
  #[sdk(child(qname = "a:sy"))]
  pub scale_y: std::boxed::Box<crate::schemas::a::ScaleY>,
}
/// View Origin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:origin")]
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
#[sdk(qname = "p:pos")]
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
#[sdk(qname = "p:cViewPr")]
pub struct CommonViewProperties {
  /// Variable Scale
  #[sdk(attr(qname = ":varScale"))]
  pub variable_scale: Option<crate::simple_type::BooleanValue>,
  /// View Scale
  #[sdk(child(qname = "p:scale"))]
  pub scale_factor: std::boxed::Box<ScaleFactor>,
  /// View Origin
  #[sdk(child(qname = "p:origin"))]
  pub origin: std::boxed::Box<Origin>,
}
/// Presentation Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sld")]
pub struct OutlineViewSlideListEntry {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Relationship Identifier
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Collapsed
  #[sdk(attr(qname = ":collapse"))]
  pub collapse: Option<crate::simple_type::BooleanValue>,
}
/// List of Presentation Slides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldLst")]
pub struct OutlineViewSlideList {
  /// Presentation Slide.
  #[sdk(child(qname = "p:sld"))]
  pub outline_view_slide_list_entry: Vec<OutlineViewSlideListEntry>,
}
/// A Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:guide")]
pub struct Guide {
  /// Guide Orientation
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// Guide Position
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Coordinate32Value>,
}
/// List of Guides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:guideLst")]
pub struct GuideList {
  /// A Guide.
  #[sdk(child(qname = "p:guide"))]
  pub guide: Vec<Guide>,
}
/// Defines the CommonSlideViewProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cSldViewPr")]
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
  #[sdk(child(qname = "p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// List of Guides
  #[sdk(child(qname = "p:guideLst"))]
  pub guide_list: Option<GuideList>,
}
/// Normal View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:normalViewPr")]
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
  #[sdk(child(qname = "p:restoredLeft"))]
  pub restored_left: RestoredLeft,
  /// Normal View Restored Top Properties
  #[sdk(child(qname = "p:restoredTop"))]
  pub restored_top: RestoredTop,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:slideViewPr")]
pub struct SlideViewProperties {
  /// Defines the CommonSlideViewProperties Class.
  #[sdk(child(qname = "p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Outline View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:outlineViewPr")]
pub struct OutlineViewProperties {
  /// Common View Properties
  #[sdk(child(qname = "p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// List of Presentation Slides
  #[sdk(child(qname = "p:sldLst"))]
  pub outline_view_slide_list: Option<OutlineViewSlideList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Text View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:notesTextViewPr")]
pub struct NotesTextViewProperties {
  /// Base properties for Notes View
  #[sdk(child(qname = "p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Sorter View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sorterViewPr")]
pub struct SorterViewProperties {
  /// Show Formatting
  #[sdk(attr(qname = ":showFormatting"))]
  pub show_formatting: Option<crate::simple_type::BooleanValue>,
  /// Base properties for Slide Sorter View
  #[sdk(child(qname = "p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes View Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:notesViewPr")]
pub struct NotesViewProperties {
  /// Common Slide View Properties
  #[sdk(child(qname = "p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Grid Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:gridSpacing")]
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
#[sdk(qname = "p:notesSz")]
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
#[sdk(qname = "p:ext")]
pub struct SlideExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = LaserTraceList, qname = "p14:laserTraceLst"),
            child(variant = ShowEventRecordList, qname = "p14:showEvtLst"),
            child(variant = CommentRelationship, qname = "p188:commentRel"),
            any
        )
    )]
  pub slide_extension_choice: Option<SlideExtensionChoice>,
}
/// Defines the CommonSlideDataExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct CommonSlideDataExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = CreationId, qname = "p14:creationId"), any))]
  pub common_slide_data_extension_choice: Option<CommonSlideDataExtensionChoice>,
}
/// Defines the ShowPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct ShowPropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = BrowseMode, qname = "p14:browseMode"),
            child(variant = LaserColor, boxed, qname = "p14:laserClr"),
            child(variant = ShowMediaControls, qname = "p14:showMediaCtrls"),
            any
        )
    )]
  pub show_properties_extension_choice: Option<ShowPropertiesExtensionChoice>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:pic")]
pub struct Picture {
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "p:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Markup Compatibility alternate content at this schema position.
  #[sdk(mce(qname = "mc:AlternateContent", children = [blip_fill]))]
  pub alternate_content: Vec<crate::schemas::mc::AlternateContent>,
  /// Picture Fill
  #[sdk(child(qname = "p:blipFill"))]
  pub blip_fill: Option<std::boxed::Box<BlipFill>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style.
  #[sdk(child(qname = "p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the OleObjectEmbed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:embed")]
pub struct OleObjectEmbed {
  /// Color Scheme Properties for OLE Object
  #[sdk(attr(qname = ":followColorScheme"))]
  #[sdk(string_format(kind = "token"))]
  pub follow_color_scheme: Option<OleObjectFollowColorSchemeValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OleObjectLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:link")]
pub struct OleObjectLink {
  /// Update Linked OLE Objects Automatically
  #[sdk(attr(qname = ":updateAutomatic"))]
  pub auto_update: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Transition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:transition")]
pub struct Transition {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// spd
  #[sdk(attr(qname = ":spd"))]
  #[sdk(string_format(kind = "token"))]
  pub speed: Option<TransitionSpeedValues>,
  /// dur
  #[sdk(attr(qname = "p14:dur"))]
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
  #[sdk(
        choice(
            child(variant = BlindsTransition, qname = "p:blinds"),
            child(variant = CheckerTransition, qname = "p:checker"),
            empty_child(variant = CircleTransition, qname = "p:circle"),
            empty_child(variant = DissolveTransition, qname = "p:dissolve"),
            child(variant = CombTransition, qname = "p:comb"),
            child(variant = CoverTransition, qname = "p:cover"),
            child(variant = CutTransition, qname = "p:cut"),
            empty_child(variant = DiamondTransition, qname = "p:diamond"),
            child(variant = FadeTransition, qname = "p:fade"),
            empty_child(variant = NewsflashTransition, qname = "p:newsflash"),
            empty_child(variant = PlusTransition, qname = "p:plus"),
            child(variant = PullTransition, qname = "p:pull"),
            child(variant = PushTransition, qname = "p:push"),
            empty_child(variant = RandomTransition, qname = "p:random"),
            child(variant = RandomBarTransition, qname = "p:randomBar"),
            child(variant = SplitTransition, qname = "p:split"),
            child(variant = StripsTransition, qname = "p:strips"),
            empty_child(variant = WedgeTransition, qname = "p:wedge"),
            child(variant = WheelTransition, qname = "p:wheel"),
            child(variant = WipeTransition, qname = "p:wipe"),
            child(variant = ZoomTransition, qname = "p:zoom"),
            empty_child(variant = FlashTransition, qname = "p14:flash"),
            child(variant = VortexTransition, qname = "p14:vortex"),
            child(variant = SwitchTransition, qname = "p14:switch"),
            child(variant = FlipTransition, qname = "p14:flip"),
            child(variant = RippleTransition, qname = "p14:ripple"),
            child(variant = GlitterTransition, qname = "p14:glitter"),
            empty_child(variant = HoneycombTransition, qname = "p14:honeycomb"),
            child(variant = PrismTransition, qname = "p14:prism"),
            child(variant = DoorsTransition, qname = "p14:doors"),
            child(variant = WindowTransition, qname = "p14:window"),
            child(variant = ShredTransition, qname = "p14:shred"),
            child(variant = FerrisTransition, qname = "p14:ferris"),
            child(variant = FlythroughTransition, qname = "p14:flythrough"),
            child(variant = WarpTransition, qname = "p14:warp"),
            child(variant = GalleryTransition, qname = "p14:gallery"),
            child(variant = ConveyorTransition, qname = "p14:conveyor"),
            child(variant = PanTransition, qname = "p14:pan"),
            child(variant = RevealTransition, qname = "p14:reveal"),
            child(variant = WheelReverseTransition, qname = "p14:wheelReverse"),
            child(variant = PresetTransition, qname = "p15:prstTrans")
        )
    )]
  pub transition_choice: Option<TransitionChoice>,
  /// Defines the SoundAction Class.
  #[sdk(child(qname = "p:sndAc"))]
  pub sound_action: Option<std::boxed::Box<SoundAction>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Slide Timing Information for a Slide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:timing")]
pub struct Timing {
  /// Time Node List.
  #[sdk(child(qname = "p:tnLst"))]
  pub time_node_list: Option<std::boxed::Box<TimeNodeList>>,
  /// Build List
  #[sdk(child(qname = "p:bldLst"))]
  pub build_list: Option<BuildList>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct SlideExtensionList {
  /// Defines the SlideExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub slide_extension: Vec<SlideExtension>,
}
/// Slide Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:bg")]
pub struct Background {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  #[sdk(
        choice(
            child(variant = BackgroundProperties, boxed, qname = "p:bgPr"),
            child(variant = BackgroundStyleReference, boxed, qname = "p:bgRef")
        )
    )]
  pub background_choice: Option<BackgroundChoice>,
}
/// Shape Tree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:spTree")]
pub struct ShapeTree {
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Group Shape Properties
  #[sdk(child(qname = "p:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = Shape, boxed, qname = "p:sp"),
            child(variant = GroupShape, boxed, qname = "p:grpSp"),
            child(variant = GraphicFrame, boxed, qname = "p:graphicFrame"),
            child(variant = ConnectionShape, boxed, qname = "p:cxnSp"),
            child(variant = Picture, boxed, qname = "p:pic"),
            child(variant = ContentPart, boxed, qname = "p:contentPart"),
            child(variant = AlternateContent, boxed, qname = "mc:AlternateContent")
        )
    )]
  pub shape_tree_choice: Vec<ShapeTreeChoice>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:grpSp")]
pub struct GroupShape {
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Group Shape Properties
  #[sdk(child(qname = "p:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = Shape, boxed, qname = "p:sp"),
            child(variant = GroupShape, boxed, qname = "p:grpSp"),
            child(variant = GraphicFrame, boxed, qname = "p:graphicFrame"),
            child(variant = ConnectionShape, boxed, qname = "p:cxnSp"),
            child(variant = Picture, boxed, qname = "p:pic"),
            child(variant = ContentPart, boxed, qname = "p:contentPart"),
            child(variant = AlternateContent, boxed, qname = "mc:AlternateContent")
        )
    )]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Customer Data List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:custDataLst")]
pub struct CustomerDataList {
  /// Customer Data.
  #[sdk(child(qname = "p:custData"))]
  pub customer_data: Vec<CustomerData>,
  /// Customer Data Tags.
  #[sdk(child(qname = "p:tags"))]
  pub customer_data_tags: Option<CustomerDataTags>,
}
/// List of controls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:controls")]
pub struct ControlList {
  #[sdk(
        choice(
            child(variant = Control, boxed, qname = "p:control"),
            child(variant = AlternateContent, boxed, qname = "mc:AlternateContent")
        )
    )]
  pub xml_children: Vec<ControlListChoice>,
}
/// Defines the CommonSlideDataExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct CommonSlideDataExtensionList {
  /// Defines the CommonSlideDataExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub common_slide_data_extension: Vec<CommonSlideDataExtension>,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Non-visual Drawing Properties
  #[sdk(child(qname = "p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "p:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
  /// Non-Visual Properties
  #[sdk(child(qname = "p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<crate::schemas::a::TransformGroup>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = BlipFill, boxed, qname = "a:blipFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub group_shape_properties_choice1: Option<GroupShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, boxed, qname = "a:effectLst"),
            child(variant = EffectDag, boxed, qname = "a:effectDag")
        )
    )]
  pub group_shape_properties_choice2: Option<GroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sp")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Use Background Fill
  #[sdk(attr(qname = ":useBgFill"))]
  pub use_background_fill: Option<crate::simple_type::BooleanValue>,
  /// Non-Visual Properties for a Shape
  #[sdk(child(qname = "p:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Shape Text Body
  #[sdk(child(qname = "p:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:graphicFrame")]
pub struct GraphicFrame {
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "p:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// 2D Transform for Graphic Frame
  #[sdk(child(qname = "p:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// Graphic Object.
  #[sdk(child(qname = "a:graphic"))]
  pub graphic: std::boxed::Box<crate::schemas::a::Graphic>,
  /// Extension List with Modification Flag
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cxnSp")]
pub struct ConnectionShape {
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "p:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Shape Properties
  #[sdk(child(qname = "p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Connector Shape Style
  #[sdk(child(qname = "p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionListWithModification Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ShowPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct ShowPropertiesExtensionList {
  /// Defines the ShowPropertiesExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub show_properties_extension: Vec<ShowPropertiesExtension>,
}
/// Shape Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:spTgt")]
pub struct ShapeTarget {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  #[sdk(
        choice(
            empty_child(variant = BackgroundAnimation, qname = "p:bg"),
            child(variant = SubShape, qname = "p:subSp"),
            child(variant = OleChartElement, qname = "p:oleChartEl"),
            child(variant = TextElement, boxed, qname = "p:txEl"),
            child(variant = GraphicElement, boxed, qname = "p:graphicEl")
        )
    )]
  pub shape_target_choice: Option<ShapeTargetChoice>,
}
/// Ink Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:inkTgt")]
pub struct InkTarget {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Subshape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:subSp")]
pub struct SubShape {
  /// Shape ID
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Defines the CommentAuthorExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct CommentAuthorExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = PresenceInfo, qname = "p15:presenceInfo"), any))]
  pub comment_author_extension_choice: Option<CommentAuthorExtensionChoice>,
}
/// Defines the CommentExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct CommentExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = ThreadingInfo, boxed, qname = "p15:threadingInfo"), any)
    )]
  pub comment_extension_choice: Option<CommentExtensionChoice>,
}
/// Defines the SlideLayoutExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct SlideLayoutExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = SlideGuideList, boxed, qname = "p15:sldGuideLst"), any)
    )]
  pub slide_layout_extension_choice: Option<SlideLayoutExtensionChoice>,
}
/// Defines the SlideMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct SlideMasterExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = SlideGuideList, boxed, qname = "p15:sldGuideLst"), any)
    )]
  pub slide_master_extension_choice: Option<SlideMasterExtensionChoice>,
}
/// Defines the HandoutMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct HandoutMasterExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = SlideGuideList, boxed, qname = "p15:sldGuideLst"), any)
    )]
  pub handout_master_extension_choice: Option<HandoutMasterExtensionChoice>,
}
/// Defines the NotesMasterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct NotesMasterExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = SlideGuideList, boxed, qname = "p15:sldGuideLst"), any)
    )]
  pub notes_master_extension_choice: Option<NotesMasterExtensionChoice>,
}
/// Placeholder Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ph")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct ApplicationNonVisualDrawingPropertiesExtensionList {
  /// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub application_non_visual_drawing_properties_extension:
    Vec<ApplicationNonVisualDrawingPropertiesExtension>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct ApplicationNonVisualDrawingPropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = Media, boxed, qname = "p14:media"),
            child(variant = ModificationId, qname = "p14:modId"),
            any
        )
    )]
  pub application_non_visual_drawing_properties_extension_choice:
    Option<ApplicationNonVisualDrawingPropertiesExtensionChoice>,
}
/// Defines the Iterate Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:iterate")]
pub struct Iterate {
  /// Iterate Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<IterateValues>,
  /// Backwards
  #[sdk(attr(qname = ":backwards"))]
  pub backwards: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = TimeAbsolute, qname = "p:tmAbs"),
            child(variant = TimePercentage, qname = "p:tmPct")
        )
    )]
  pub iterate_choice: Option<IterateChoice>,
}
/// Defines the ChildTimeNodeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:childTnLst")]
pub struct ChildTimeNodeList {
  #[sdk(
        choice(
            child(variant = ParallelTimeNode, boxed, qname = "p:par"),
            child(variant = SequenceTimeNode, boxed, qname = "p:seq"),
            child(variant = ExclusiveTimeNode, boxed, qname = "p:excl"),
            child(variant = Animate, boxed, qname = "p:anim"),
            child(variant = AnimateColor, boxed, qname = "p:animClr"),
            child(variant = AnimateEffect, boxed, qname = "p:animEffect"),
            child(variant = AnimateMotion, boxed, qname = "p:animMotion"),
            child(variant = AnimateRotation, boxed, qname = "p:animRot"),
            child(variant = AnimateScale, boxed, qname = "p:animScale"),
            child(variant = Command, boxed, qname = "p:cmd"),
            child(variant = SetBehavior, boxed, qname = "p:set"),
            child(variant = Audio, boxed, qname = "p:audio"),
            child(variant = Video, boxed, qname = "p:video")
        )
    )]
  pub child_time_node_list_choice: Vec<ChildTimeNodeListChoice>,
}
/// Defines the SubTimeNodeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:subTnLst")]
pub struct SubTimeNodeList {
  #[sdk(
        choice(
            child(variant = ParallelTimeNode, boxed, qname = "p:par"),
            child(variant = SequenceTimeNode, boxed, qname = "p:seq"),
            child(variant = ExclusiveTimeNode, boxed, qname = "p:excl"),
            child(variant = Animate, boxed, qname = "p:anim"),
            child(variant = AnimateColor, boxed, qname = "p:animClr"),
            child(variant = AnimateEffect, boxed, qname = "p:animEffect"),
            child(variant = AnimateMotion, boxed, qname = "p:animMotion"),
            child(variant = AnimateRotation, boxed, qname = "p:animRot"),
            child(variant = AnimateScale, boxed, qname = "p:animScale"),
            child(variant = Command, boxed, qname = "p:cmd"),
            child(variant = SetBehavior, boxed, qname = "p:set"),
            child(variant = Audio, boxed, qname = "p:audio"),
            child(variant = Video, boxed, qname = "p:video")
        )
    )]
  pub sub_time_node_list_choice: Vec<SubTimeNodeListChoice>,
}
/// Defines the TimeAnimateValueList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:tavLst")]
pub struct TimeAnimateValueList {
  /// Time Animate Value.
  #[sdk(child(qname = "p:tav"))]
  pub time_animate_value: Vec<TimeAnimateValue>,
}
/// Defines the ByPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:by")]
pub struct ByPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::DrawingmlPercentageValue,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::DrawingmlPercentageValue,
}
/// Defines the FromPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:from")]
pub struct FromPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::DrawingmlPercentageValue,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::DrawingmlPercentageValue,
}
/// Defines the ToPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:to")]
pub struct ToPosition {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::DrawingmlPercentageValue,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::DrawingmlPercentageValue,
}
/// Defines the RotationCenter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:rCtr")]
pub struct RotationCenter {
  /// X coordinate
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::DrawingmlPercentageValue,
  /// Y coordinate
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::DrawingmlPercentageValue,
}
/// Defines the CommentAuthorExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct CommentAuthorExtensionList {
  /// Defines the CommentAuthorExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub comment_author_extension: Vec<CommentAuthorExtension>,
}
/// Defines the CommentExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct CommentExtensionList {
  /// Defines the CommentExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub comment_extension: Vec<CommentExtension>,
}
/// Defines the SlideMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldMasterIdLst")]
pub struct SlideMasterIdList {
  /// Slide Master ID.
  #[sdk(child(qname = "p:sldMasterId"))]
  pub slide_master_id: Vec<SlideMasterId>,
}
/// Defines the NotesMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:notesMasterIdLst")]
pub struct NotesMasterIdList {
  /// Notes Master ID
  #[sdk(child(qname = "p:notesMasterId"))]
  pub notes_master_id: Option<std::boxed::Box<NotesMasterId>>,
}
/// Defines the HandoutMasterIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:handoutMasterIdLst")]
pub struct HandoutMasterIdList {
  /// Handout Master ID
  #[sdk(child(qname = "p:handoutMasterId"))]
  pub handout_master_id: Option<std::boxed::Box<HandoutMasterId>>,
}
/// Defines the SlideIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldIdLst")]
pub struct SlideIdList {
  /// Slide ID.
  #[sdk(child(qname = "p:sldId"))]
  pub slide_id: Vec<SlideId>,
}
/// Defines the SlideSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldSz")]
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
#[sdk(qname = "p:embeddedFontLst")]
pub struct EmbeddedFontList {
  /// Embedded Font.
  #[sdk(child(qname = "p:embeddedFont"))]
  pub embedded_font: Vec<EmbeddedFont>,
}
/// Defines the CustomShowList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:custShowLst")]
pub struct CustomShowList {
  /// Custom Show.
  #[sdk(child(qname = "p:custShow"))]
  pub custom_show: Vec<CustomShow>,
}
/// Defines the PhotoAlbum Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:photoAlbum")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Kinsoku Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:kinsoku")]
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
#[sdk(qname = "p:modifyVerifier")]
pub struct ModificationVerifier {
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
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinValue
  #[sdk(attr(qname = ":spinValue"))]
  pub spin_value: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PresentationExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct PresentationExtensionList {
  /// Defines the PresentationExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub presentation_extension: Vec<PresentationExtension>,
}
/// Defines the PresentationExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct PresentationExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = SectionProperties, qname = "p14:sectionPr"),
            child(variant = SectionList, qname = "p14:sectionLst"),
            child(variant = SlideGuideList, boxed, qname = "p15:sldGuideLst"),
            child(variant = NotesGuideList, boxed, qname = "p15:notesGuideLst"),
            any
        )
    )]
  pub presentation_extension_choice: Option<PresentationExtensionChoice>,
}
/// HTML Publishing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:htmlPubPr")]
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
  #[sdk(
        choice(
            empty_child(variant = SlideAll, qname = "p:sldAll"),
            child(variant = SlideRange, qname = "p:sldRg"),
            child(variant = CustomShowReference, qname = "p:custShow")
        )
    )]
  pub html_publish_properties_choice: Option<HtmlPublishPropertiesChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Web Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:webPr")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PrintingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:prnPr")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ShowProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:showPr")]
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
  #[sdk(
        choice(
            empty_child(variant = PresenterSlideMode, qname = "p:present"),
            child(variant = BrowseSlideMode, qname = "p:browse"),
            child(variant = KioskSlideMode, qname = "p:kiosk")
        )
    )]
  pub show_properties_choice1: Option<ShowPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = SlideAll, qname = "p:sldAll"),
            child(variant = SlideRange, qname = "p:sldRg"),
            child(variant = CustomShowReference, qname = "p:custShow")
        )
    )]
  pub show_properties_choice2: Option<ShowPropertiesChoice2>,
  /// Pen Color for Slide Show.
  #[sdk(child(qname = "p:penClr"))]
  pub pen_color: Option<std::boxed::Box<PenColor>>,
  /// Defines the ShowPropertiesExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub show_properties_extension_list: Option<ShowPropertiesExtensionList>,
}
/// Defines the ColorMostRecentlyUsed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:clrMru")]
pub struct ColorMostRecentlyUsed {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, boxed, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_most_recently_used_choice: Vec<ColorMostRecentlyUsedChoice>,
}
/// Defines the PresentationPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct PresentationPropertiesExtensionList {
  /// Defines the PresentationPropertiesExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub presentation_properties_extension: Vec<PresentationPropertiesExtension>,
}
/// Defines the PresentationPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:ext")]
pub struct PresentationPropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = DiscardImageEditData, qname = "p14:discardImageEditData"),
            child(variant = DefaultImageDpi, qname = "p14:defaultImageDpi"),
            child(variant = TextMath, qname = "a14:m"),
            child(
                variant = ChartTrackingReferenceBased,
                qname = "p15:chartTrackingRefBased"
            ),
            any
        )
    )]
  pub presentation_properties_extension_choice: Option<PresentationPropertiesExtensionChoice>,
}
/// Defines the HeaderFooter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:hf")]
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
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideLayoutExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct SlideLayoutExtensionList {
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Defines the SlideLayoutExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub slide_layout_extension: Vec<SlideLayoutExtension>,
}
/// Defines the SlideLayoutIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sldLayoutIdLst")]
pub struct SlideLayoutIdList {
  /// Slide Layout Id.
  #[sdk(child(qname = "p:sldLayoutId"))]
  pub slide_layout_id: Vec<SlideLayoutId>,
}
/// Defines the TextStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:txStyles")]
pub struct TextStyles {
  /// Slide Master Title Text Style
  #[sdk(child(qname = "p:titleStyle"))]
  pub title_style: Option<std::boxed::Box<TitleStyle>>,
  /// Slide Master Body Text Style
  #[sdk(child(qname = "p:bodyStyle"))]
  pub body_style: Option<std::boxed::Box<BodyStyle>>,
  /// Slide Master Other Text Style
  #[sdk(child(qname = "p:otherStyle"))]
  pub other_style: Option<std::boxed::Box<OtherStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlideMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct SlideMasterExtensionList {
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Defines the SlideMasterExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub slide_master_extension: Vec<SlideMasterExtension>,
}
/// Defines the HandoutMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct HandoutMasterExtensionList {
  /// Defines the HandoutMasterExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub handout_master_extension: Vec<HandoutMasterExtension>,
}
/// Defines the NotesMasterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:extLst")]
pub struct NotesMasterExtensionList {
  /// Defines the NotesMasterExtension Class.
  #[sdk(child(qname = "p:ext"))]
  pub notes_master_extension: Vec<NotesMasterExtension>,
}
/// OLE Chart Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:oleChartEl")]
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
#[sdk(qname = "p:txEl")]
pub struct TextElement {
  #[sdk(
        choice(
            child(variant = CharRange, qname = "p:charRg"),
            child(variant = ParagraphIndexRange, qname = "p:pRg")
        )
    )]
  pub text_element_choice: Option<TextElementChoice>,
}
/// Graphic Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:graphicEl")]
pub struct GraphicElement {
  #[sdk(
        choice(
            child(variant = Diagram, qname = "a:dgm"),
            child(variant = Chart, qname = "a:chart")
        )
    )]
  pub graphic_element_choice: Option<GraphicElementChoice>,
}
/// Defines the BlindsTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:blinds")]
pub struct BlindsTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CheckerTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:checker")]
pub struct CheckerTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CombTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:comb")]
pub struct CombTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the RandomBarTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:randomBar")]
pub struct RandomBarTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CoverTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cover")]
pub struct CoverTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the PullTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:pull")]
pub struct PullTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the CutTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:cut")]
pub struct CutTransition {
  /// Transition Through Black
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the FadeTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:fade")]
pub struct FadeTransition {
  /// Transition Through Black
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PushTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:push")]
pub struct PushTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the WipeTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:wipe")]
pub struct WipeTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SplitTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:split")]
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
#[sdk(qname = "p:strips")]
pub struct StripsTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionCornerDirectionValues>,
}
/// Defines the WheelTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:wheel")]
pub struct WheelTransition {
  /// Spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ZoomTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:zoom")]
pub struct ZoomTransition {
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the SoundAction Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:sndAc")]
pub struct SoundAction {
  #[sdk(
        choice(
            child(variant = StartSoundAction, boxed, qname = "p:stSnd"),
            empty_child(variant = EndSoundAction, qname = "p:endSnd")
        )
    )]
  pub sound_action_choice: Option<SoundActionChoice>,
}
/// Defines the PlaceholderExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(version = microsoft365, qname = "p:ext")]
pub struct PlaceholderExtension {
  /// Defines the PlaceholderTypeExtension Class.
  #[sdk(child(qname = "p232:phTypeExt"))]
  pub placeholder_type_extension:
    Option<std::boxed::Box<crate::schemas::p232::PlaceholderTypeExtension>>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorMapOverrideChoice {
  /// Master Color Mapping.
  MasterColorMapping,
  /// Override Color Mapping.
  OverrideColorMapping(std::boxed::Box<crate::schemas::a::OverrideColorMapping>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundPropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
  /// Pandoc compatibility form of effectLst.
  PandocEffectsList(std::boxed::Box<crate::schemas::a::EffectList>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundStyleReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OleObjectChoice {
  /// Defines the OleObjectEmbed Class.
  OleObjectEmbed(std::boxed::Box<OleObjectEmbed>),
  /// Defines the OleObjectLink Class.
  OleObjectLink(std::boxed::Box<OleObjectLink>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TargetElementChoice {
  /// Slide Target.
  SlideTarget,
  /// Sound Target.
  SoundTarget(SoundTarget),
  /// Shape Target.
  ShapeTarget(std::boxed::Box<ShapeTarget>),
  /// Ink Target.
  InkTarget(InkTarget),
  /// Defines the BookmarkTarget Class.
  BookmarkTarget(crate::schemas::p14::BookmarkTarget),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConditionChoice {
  /// Target Element Trigger Choice.
  TargetElement(std::boxed::Box<TargetElement>),
  /// Time Node.
  TimeNode(TimeNode),
  /// Runtime Node Trigger Choice.
  RuntimeNodeTrigger(RuntimeNodeTrigger),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndSyncChoice {
  /// Target Element Trigger Choice.
  TargetElement(std::boxed::Box<TargetElement>),
  /// Time Node.
  TimeNode(TimeNode),
  /// Runtime Node Trigger Choice.
  RuntimeNodeTrigger(RuntimeNodeTrigger),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorValueChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PenColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ToVariantValueChoice {
  /// Boolean Variant.
  BooleanVariantValue(BooleanVariantValue),
  /// Integer.
  IntegerVariantValue(IntegerVariantValue),
  /// Float Value.
  FloatVariantValue(FloatVariantValue),
  /// String Value.
  StringVariantValue(StringVariantValue),
  /// Color Value.
  ColorValue(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum VariantValueChoice {
  /// Boolean Variant.
  BooleanVariantValue(BooleanVariantValue),
  /// Integer.
  IntegerVariantValue(IntegerVariantValue),
  /// Float Value.
  FloatVariantValue(FloatVariantValue),
  /// String Value.
  StringVariantValue(StringVariantValue),
  /// Color Value.
  ColorValue(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BuildSubElementChoice {
  /// Build Diagram.
  BuildDiagram(crate::schemas::a::BuildDiagram),
  /// Build Chart.
  BuildChart(crate::schemas::a::BuildChart),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BuildGraphicsChoice {
  /// Build As One.
  BuildAsOne,
  /// Build Sub Elements.
  BuildSubElement(std::boxed::Box<BuildSubElement>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BuildListChoice {
  /// Build Paragraph.
  BuildParagraph(std::boxed::Box<BuildParagraph>),
  /// Build Diagram.
  BuildDiagram(BuildDiagram),
  /// Build OLE Chart.
  BuildOleChart(BuildOleChart),
  /// Build Graphics.
  BuildGraphics(std::boxed::Box<BuildGraphics>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ByColorChoice {
  /// RGB.
  RgbColor(RgbColor),
  /// HSL.
  HslColor(HslColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FromColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ToColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationNonVisualDrawingPropertiesChoice {
  AudioFromCd(std::boxed::Box<crate::schemas::a::AudioFromCd>),
  WaveAudioFile(crate::schemas::a::WaveAudioFile),
  AudioFromFile(std::boxed::Box<crate::schemas::a::AudioFromFile>),
  VideoFromFile(std::boxed::Box<crate::schemas::a::VideoFromFile>),
  QuickTimeFromFile(std::boxed::Box<crate::schemas::a::QuickTimeFromFile>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipFillChoice {
  Tile(std::boxed::Box<crate::schemas::a::Tile>),
  Stretch(std::boxed::Box<crate::schemas::a::Stretch>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SlideExtensionChoice {
  /// Defines the LaserTraceList Class.
  LaserTraceList(crate::schemas::p14::LaserTraceList),
  /// Defines the ShowEventRecordList Class.
  ShowEventRecordList(crate::schemas::p14::ShowEventRecordList),
  /// Defines the CommentRelationship Class.
  CommentRelationship(crate::schemas::p188::CommentRelationship),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CommonSlideDataExtensionChoice {
  /// Defines the CreationId Class.
  CreationId(crate::schemas::p14::CreationId),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShowPropertiesExtensionChoice {
  /// Defines the BrowseMode Class.
  BrowseMode(crate::schemas::p14::BrowseMode),
  /// Defines the LaserColor Class.
  LaserColor(std::boxed::Box<crate::schemas::p14::LaserColor>),
  /// Defines the ShowMediaControls Class.
  ShowMediaControls(crate::schemas::p14::ShowMediaControls),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TransitionChoice {
  /// Defines the BlindsTransition Class.
  BlindsTransition(BlindsTransition),
  /// Defines the CheckerTransition Class.
  CheckerTransition(CheckerTransition),
  /// Defines the CircleTransition Class.
  CircleTransition,
  /// Defines the DissolveTransition Class.
  DissolveTransition,
  /// Defines the CombTransition Class.
  CombTransition(CombTransition),
  /// Defines the CoverTransition Class.
  CoverTransition(CoverTransition),
  /// Defines the CutTransition Class.
  CutTransition(CutTransition),
  /// Defines the DiamondTransition Class.
  DiamondTransition,
  /// Defines the FadeTransition Class.
  FadeTransition(FadeTransition),
  /// Defines the NewsflashTransition Class.
  NewsflashTransition,
  /// Defines the PlusTransition Class.
  PlusTransition,
  /// Defines the PullTransition Class.
  PullTransition(PullTransition),
  /// Defines the PushTransition Class.
  PushTransition(PushTransition),
  /// Defines the RandomTransition Class.
  RandomTransition,
  /// Defines the RandomBarTransition Class.
  RandomBarTransition(RandomBarTransition),
  /// Defines the SplitTransition Class.
  SplitTransition(SplitTransition),
  /// Defines the StripsTransition Class.
  StripsTransition(StripsTransition),
  /// Defines the WedgeTransition Class.
  WedgeTransition,
  /// Defines the WheelTransition Class.
  WheelTransition(WheelTransition),
  /// Defines the WipeTransition Class.
  WipeTransition(WipeTransition),
  /// Defines the ZoomTransition Class.
  ZoomTransition(ZoomTransition),
  /// Defines the FlashTransition Class.
  FlashTransition,
  VortexTransition(crate::schemas::p14::VortexTransition),
  SwitchTransition(crate::schemas::p14::SwitchTransition),
  FlipTransition(crate::schemas::p14::FlipTransition),
  RippleTransition(crate::schemas::p14::RippleTransition),
  GlitterTransition(crate::schemas::p14::GlitterTransition),
  /// Defines the HoneycombTransition Class.
  HoneycombTransition,
  PrismTransition(crate::schemas::p14::PrismTransition),
  DoorsTransition(crate::schemas::p14::DoorsTransition),
  WindowTransition(crate::schemas::p14::WindowTransition),
  ShredTransition(crate::schemas::p14::ShredTransition),
  FerrisTransition(crate::schemas::p14::FerrisTransition),
  FlythroughTransition(crate::schemas::p14::FlythroughTransition),
  WarpTransition(crate::schemas::p14::WarpTransition),
  GalleryTransition(crate::schemas::p14::GalleryTransition),
  ConveyorTransition(crate::schemas::p14::ConveyorTransition),
  PanTransition(crate::schemas::p14::PanTransition),
  RevealTransition(crate::schemas::p14::RevealTransition),
  WheelReverseTransition(crate::schemas::p14::WheelReverseTransition),
  PresetTransition(crate::schemas::p15::PresetTransition),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundChoice {
  /// Background Properties.
  BackgroundProperties(std::boxed::Box<BackgroundProperties>),
  /// Background Style Reference.
  BackgroundStyleReference(std::boxed::Box<BackgroundStyleReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeTreeChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  AlternateContent(std::boxed::Box<crate::schemas::mc::AlternateContent>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapeChoice {
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Group Shape.
  GroupShape(std::boxed::Box<GroupShape>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Defines the Picture Class.
  Picture(std::boxed::Box<Picture>),
  /// Defines the ContentPart Class.
  ContentPart(std::boxed::Box<ContentPart>),
  AlternateContent(std::boxed::Box<crate::schemas::mc::AlternateContent>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(crate::schemas::a::NoFill),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapeTargetChoice {
  /// Background.
  BackgroundAnimation,
  /// Subshape.
  SubShape(SubShape),
  /// OLE Chart Element.
  OleChartElement(OleChartElement),
  /// Text Element.
  TextElement(std::boxed::Box<TextElement>),
  /// Graphic Element.
  GraphicElement(std::boxed::Box<GraphicElement>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CommentAuthorExtensionChoice {
  /// Defines the PresenceInfo Class.
  PresenceInfo(crate::schemas::p15::PresenceInfo),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CommentExtensionChoice {
  /// Defines the ThreadingInfo Class.
  ThreadingInfo(std::boxed::Box<crate::schemas::p15::ThreadingInfo>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SlideLayoutExtensionChoice {
  /// Defines the SlideGuideList Class.
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SlideMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HandoutMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum NotesMasterExtensionChoice {
  /// Defines the SlideGuideList Class.
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationNonVisualDrawingPropertiesExtensionChoice {
  /// Defines the Media Class.
  Media(std::boxed::Box<crate::schemas::p14::Media>),
  /// Defines the ModificationId Class.
  ModificationId(crate::schemas::p14::ModificationId),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum IterateChoice {
  /// Time Absolute.
  TimeAbsolute(TimeAbsolute),
  /// Time Percentage.
  TimePercentage(TimePercentage),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ChildTimeNodeListChoice {
  /// Parallel Time Node.
  ParallelTimeNode(std::boxed::Box<ParallelTimeNode>),
  /// Sequence Time Node.
  SequenceTimeNode(std::boxed::Box<SequenceTimeNode>),
  /// Exclusive.
  ExclusiveTimeNode(std::boxed::Box<ExclusiveTimeNode>),
  /// Animate.
  Animate(std::boxed::Box<Animate>),
  /// Animate Color Behavior.
  AnimateColor(std::boxed::Box<AnimateColor>),
  /// Animate Effect.
  AnimateEffect(std::boxed::Box<AnimateEffect>),
  /// Animate Motion.
  AnimateMotion(std::boxed::Box<AnimateMotion>),
  /// Animate Rotation.
  AnimateRotation(std::boxed::Box<AnimateRotation>),
  /// Animate Scale.
  AnimateScale(std::boxed::Box<AnimateScale>),
  /// Command.
  Command(std::boxed::Box<Command>),
  /// Set Time Node Behavior.
  SetBehavior(std::boxed::Box<SetBehavior>),
  /// Audio.
  Audio(std::boxed::Box<Audio>),
  /// Video.
  Video(std::boxed::Box<Video>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SubTimeNodeListChoice {
  /// Parallel Time Node.
  ParallelTimeNode(std::boxed::Box<ParallelTimeNode>),
  /// Sequence Time Node.
  SequenceTimeNode(std::boxed::Box<SequenceTimeNode>),
  /// Exclusive.
  ExclusiveTimeNode(std::boxed::Box<ExclusiveTimeNode>),
  /// Animate.
  Animate(std::boxed::Box<Animate>),
  /// Animate Color Behavior.
  AnimateColor(std::boxed::Box<AnimateColor>),
  /// Animate Effect.
  AnimateEffect(std::boxed::Box<AnimateEffect>),
  /// Animate Motion.
  AnimateMotion(std::boxed::Box<AnimateMotion>),
  /// Animate Rotation.
  AnimateRotation(std::boxed::Box<AnimateRotation>),
  /// Animate Scale.
  AnimateScale(std::boxed::Box<AnimateScale>),
  /// Command.
  Command(std::boxed::Box<Command>),
  /// Set Time Node Behavior.
  SetBehavior(std::boxed::Box<SetBehavior>),
  /// Audio.
  Audio(std::boxed::Box<Audio>),
  /// Video.
  Video(std::boxed::Box<Video>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PresentationExtensionChoice {
  /// Defines the SectionProperties Class.
  SectionProperties(crate::schemas::p14::SectionProperties),
  /// Defines the SectionList Class.
  SectionList(crate::schemas::p14::SectionList),
  /// Defines the SlideGuideList Class.
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  /// Defines the NotesGuideList Class.
  NotesGuideList(std::boxed::Box<crate::schemas::p15::NotesGuideList>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HtmlPublishPropertiesChoice {
  /// All Slides.
  SlideAll,
  /// Slide Range.
  SlideRange(SlideRange),
  /// Custom Show.
  CustomShowReference(CustomShowReference),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShowPropertiesChoice {
  /// Presenter Slide Show Mode.
  PresenterSlideMode,
  /// Browse Slide Show Mode.
  BrowseSlideMode(BrowseSlideMode),
  /// Kiosk Slide Show Mode.
  KioskSlideMode(KioskSlideMode),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShowPropertiesChoice2 {
  /// All Slides.
  SlideAll,
  /// Slide Range.
  SlideRange(SlideRange),
  /// Custom Show.
  CustomShowReference(CustomShowReference),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorMostRecentlyUsedChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(crate::schemas::a::RgbColorModelPercentage),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(crate::schemas::a::HslColor),
  /// System Color.
  SystemColor(crate::schemas::a::SystemColor),
  /// Scheme Color.
  SchemeColor(crate::schemas::a::SchemeColor),
  /// Preset Color.
  PresetColor(crate::schemas::a::PresetColor),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PresentationPropertiesExtensionChoice {
  /// Defines the DiscardImageEditData Class.
  DiscardImageEditData(crate::schemas::p14::DiscardImageEditData),
  /// Defines the DefaultImageDpi Class.
  DefaultImageDpi(crate::schemas::p14::DefaultImageDpi),
  /// Defines the TextMath Class.
  TextMath(crate::schemas::a14::TextMath),
  /// Defines the ChartTrackingReferenceBased Class.
  ChartTrackingReferenceBased(crate::schemas::p15::ChartTrackingReferenceBased),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextElementChoice {
  /// Character Range.
  CharRange(CharRange),
  /// Paragraph Text Range.
  ParagraphIndexRange(ParagraphIndexRange),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GraphicElementChoice {
  /// Diagram to Animate.
  Diagram(crate::schemas::a::Diagram),
  /// Chart to Animate.
  Chart(crate::schemas::a::Chart),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SoundActionChoice {
  /// Start Sound Action.
  StartSoundAction(std::boxed::Box<StartSoundAction>),
  /// Stop Sound Action.
  EndSoundAction,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ControlListChoice {
  /// Embedded Control.
  Control(std::boxed::Box<Control>),
  /// Markup Compatibility alternate content.
  AlternateContent(std::boxed::Box<crate::schemas::mc::AlternateContent>),
}
