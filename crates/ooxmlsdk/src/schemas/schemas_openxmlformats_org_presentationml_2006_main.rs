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
  #[cfg(feature = "microsoft365")]
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
/// All Slides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldAll.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:sldAll")]
pub struct SlideAll {}
/// Presenter Slide Show Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:present.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:present")]
pub struct PresenterSlideMode {}
/// Stop Sound Action.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:endSnd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:endSnd")]
pub struct EndSoundAction {}
/// Build As One.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldAsOne.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:bldAsOne")]
pub struct BuildAsOne {}
/// Slide Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldTgt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:sldTgt")]
pub struct SlideTarget {}
/// Background.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:bg")]
pub struct BackgroundAnimation {}
/// Defines the CircleTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:circle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:circle")]
pub struct CircleTransition {}
/// Defines the DissolveTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:dissolve.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:dissolve")]
pub struct DissolveTransition {}
/// Defines the DiamondTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:diamond.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:diamond")]
pub struct DiamondTransition {}
/// Defines the NewsflashTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:newsflash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:newsflash")]
pub struct NewsflashTransition {}
/// Defines the PlusTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:plus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:plus")]
pub struct PlusTransition {}
/// Defines the RandomTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:random.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:random")]
pub struct RandomTransition {}
/// Defines the WedgeTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:wedge.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p:wedge")]
pub struct WedgeTransition {}
/// Defines the EmptyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/")]
pub struct EmptyType {}
/// Slide Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldRg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:sldRg")]
pub struct SlideRange {
  /// Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Character Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:charRg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:charRg")]
pub struct CharRange {
  /// Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Paragraph Text Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:pRg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/p:pRg")]
pub struct ParagraphIndexRange {
  /// Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Defines the IndexRangeType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_IndexRange/")]
pub struct IndexRangeType {
  /// Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub start: crate::simple_type::UInt32Value,
  /// End
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :end
  #[sdk(attr(qname = ":end"))]
  pub end: crate::simple_type::UInt32Value,
}
/// Custom Show.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:custShow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShowId/p:custShow")]
pub struct CustomShowReference {
  /// Custom Show Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
}
/// Extension.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Extension/p:ext")]
pub struct Extension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "p14:CT_Media/p14:media"))]
  pub media: Option<
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Media>,
  >,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "p14:CT_RandomId/p14:creationId"))]
  pub creation_id:
    Option<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::CreationId>,
  /// _
  #[sdk(any)]
  pub unknown_xml: String,
}
/// Browse Slide Show Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:browse.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowInfoBrowse/p:browse")]
pub struct BrowseSlideMode {
  /// Show Scroll Bar in Window
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showScrollbar
  #[sdk(attr(qname = ":showScrollbar"))]
  pub show_scrollbar: Option<crate::simple_type::BooleanValue>,
}
/// Kiosk Slide Show Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:kiosk.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowInfoKiosk/p:kiosk")]
pub struct KioskSlideMode {
  /// Restart Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :restart
  #[sdk(attr(qname = ":restart"))]
  pub restart: Option<crate::simple_type::UInt32Value>,
}
/// Color Scheme Map.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:clrMap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/p:clrMap")]
pub struct ColorMap {
  /// Background 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bg1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tx1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Background 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bg2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tx2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 3
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent3:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 4
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent4:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 5
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent5:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 6
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent6:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Hyperlink
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Followed Hyperlink
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :folHlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub followed_hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Color Scheme Map Override.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:clrMapOvr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMappingOverride/p:clrMapOvr")]
pub struct ColorMapOverride {
  #[sdk(choice(
    qname = "a:CT_EmptyElement/a:masterClrMapping",
    qname = "a:CT_ColorMapping/a:overrideClrMapping"
  ))]
  pub xml_children: Option<ColorMapOverrideChoice>,
}
/// Background Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bgPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_BackgroundProperties/p:bgPr")]
pub struct BackgroundProperties {
  /// Shade to Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shadeToTitle
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
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ExtensionList>,
}
/// Background Style Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bgRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/p:bgRef")]
pub struct BackgroundStyleReference {
  /// Style Matrix Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
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
  pub xml_children: Option<BackgroundStyleReferenceChoice>,
}
#[cfg(feature = "microsoft365")]
/// Data for the Windows platform..
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p188:CT_CommentPropertiesExtension/p:ext")]
pub struct CommentPropertiesExtension {
  /// _
  #[sdk(child(qname = "p228:CT_TaskDetails/p228:taskDetails"))]
  pub task_details: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2022_08_main::TaskDetails,
    >,
  >,
  /// _
  #[sdk(child(qname = "p223:CT_Reactions/p223:reactions"))]
  pub reactions:
    Option<crate::schemas::schemas_microsoft_com_office_powerpoint_2022_03_main::Reactions>,
}
/// List of Comment Authors.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cmAuthorLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorList/p:cmAuthorLst")]
pub struct CommentAuthorList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p:CT_CommentAuthor/p:cmAuthor"))]
  pub p_cm_author: Vec<CommentAuthor>,
}
/// Comment List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cmLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentList/p:cmLst")]
pub struct CommentList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p:CT_Comment/p:cm"))]
  pub p_cm: Vec<Comment>,
}
/// Global Element for OLE Objects and Controls.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:oleObj.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObject/p:oleObj")]
pub struct OleObject {
  /// spid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// showAsIcon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAsIcon
  #[sdk(attr(qname = ":showAsIcon"))]
  pub show_as_icon: Option<crate::simple_type::BooleanValue>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// imgW
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imgW
  #[sdk(attr(qname = ":imgW"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub image_width: Option<crate::simple_type::Int32Value>,
  /// imgH
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imgH
  #[sdk(attr(qname = ":imgH"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub image_height: Option<crate::simple_type::Int32Value>,
  /// progId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :progId
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "p:CT_OleObjectEmbed/p:embed",
    qname = "p:CT_OleObjectLink/p:link"
  ))]
  pub ole_object_choice: Option<OleObjectChoice>,
  /// _
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  pub p_pic: Option<std::boxed::Box<Picture>>,
}
/// Presentation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:presentation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Presentation/p:presentation")]
pub struct Presentation {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// serverZoom
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverZoom
  #[sdk(attr(qname = ":serverZoom"))]
  pub server_zoom: Option<crate::simple_type::Int32Value>,
  /// firstSlideNum
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstSlideNum
  #[sdk(attr(qname = ":firstSlideNum"))]
  pub first_slide_num: Option<crate::simple_type::Int32Value>,
  /// showSpecialPlsOnTitleSld
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showSpecialPlsOnTitleSld
  #[sdk(attr(qname = ":showSpecialPlsOnTitleSld"))]
  pub show_special_placeholder_on_title_slide: Option<crate::simple_type::BooleanValue>,
  /// rtl
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rtl
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// removePersonalInfoOnSave
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :removePersonalInfoOnSave
  #[sdk(attr(qname = ":removePersonalInfoOnSave"))]
  pub remove_personal_info_on_save: Option<crate::simple_type::BooleanValue>,
  /// compatMode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compatMode
  #[sdk(attr(qname = ":compatMode"))]
  pub compatibility_mode: Option<crate::simple_type::BooleanValue>,
  /// strictFirstAndLastChars
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strictFirstAndLastChars
  #[sdk(attr(qname = ":strictFirstAndLastChars"))]
  pub strict_first_and_last_chars: Option<crate::simple_type::BooleanValue>,
  /// embedTrueTypeFonts
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :embedTrueTypeFonts
  #[sdk(attr(qname = ":embedTrueTypeFonts"))]
  pub embed_true_type_fonts: Option<crate::simple_type::BooleanValue>,
  /// saveSubsetFonts
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saveSubsetFonts
  #[sdk(attr(qname = ":saveSubsetFonts"))]
  pub save_subset_fonts: Option<crate::simple_type::BooleanValue>,
  /// autoCompressPictures
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoCompressPictures
  #[sdk(attr(qname = ":autoCompressPictures"))]
  pub auto_compress_pictures: Option<crate::simple_type::BooleanValue>,
  /// bookmarkIdSeed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bookmarkIdSeed
  #[sdk(attr(qname = ":bookmarkIdSeed"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub bookmark_id_seed: Option<crate::simple_type::UInt32Value>,
  /// conformance
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :conformance
  #[sdk(attr(qname = ":conformance"))]
  pub conformance: Option<ConformanceClassValues>,
  /// _
  #[sdk(child(qname = "p:CT_SlideMasterIdList/p:sldMasterIdLst"))]
  pub slide_master_id_list: Option<SlideMasterIdList>,
  /// _
  #[sdk(child(qname = "p:CT_NotesMasterIdList/p:notesMasterIdLst"))]
  pub notes_master_id_list: Option<std::boxed::Box<NotesMasterIdList>>,
  /// _
  #[sdk(child(qname = "p:CT_HandoutMasterIdList/p:handoutMasterIdLst"))]
  pub handout_master_id_list: Option<std::boxed::Box<HandoutMasterIdList>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideIdList/p:sldIdLst"))]
  pub slide_id_list: Option<SlideIdList>,
  /// _
  #[sdk(child(qname = "p:CT_SlideSize/p:sldSz"))]
  pub slide_size: Option<SlideSize>,
  /// _
  #[sdk(child(qname = "a:CT_PositiveSize2D/p:notesSz"))]
  pub notes_size: std::boxed::Box<NotesSize>,
  /// _
  #[sdk(child(qname = "p:CT_EmbeddedFontList/p:embeddedFontLst"))]
  pub embedded_font_list: Option<EmbeddedFontList>,
  /// _
  #[sdk(child(qname = "p:CT_CustomShowList/p:custShowLst"))]
  pub custom_show_list: Option<CustomShowList>,
  /// _
  #[sdk(child(qname = "p:CT_PhotoAlbum/p:photoAlbum"))]
  pub photo_album: Option<std::boxed::Box<PhotoAlbum>>,
  /// _
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  /// _
  #[sdk(child(qname = "p:CT_Kinsoku/p:kinsoku"))]
  pub kinsoku: Option<Kinsoku>,
  /// _
  #[sdk(child(qname = "a:CT_TextListStyle/p:defaultTextStyle"))]
  pub default_text_style: Option<std::boxed::Box<DefaultTextStyle>>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// _
  #[sdk(child(qname = "p:CT_ModifyVerifier/p:modifyVerifier"))]
  pub modification_verifier: Option<ModificationVerifier>,
  /// _
  #[sdk(child(qname = "p:CT_PresentationExtensionList/p:extLst"))]
  pub presentation_extension_list: Option<PresentationExtensionList>,
}
/// Presentation-wide Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:presentationPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationProperties/p:presentationPr")]
pub struct PresentationProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  ///HTML Publishing Properties
  #[sdk(child(qname = "p:CT_HtmlPublishProperties/p:htmlPubPr"))]
  pub html_publish_properties: Option<std::boxed::Box<HtmlPublishProperties>>,
  ///Web Properties
  #[sdk(child(qname = "p:CT_WebProperties/p:webPr"))]
  pub web_properties: Option<std::boxed::Box<WebProperties>>,
  /// _
  #[sdk(child(qname = "p:CT_PrintProperties/p:prnPr"))]
  pub printing_properties: Option<std::boxed::Box<PrintingProperties>>,
  /// _
  #[sdk(child(qname = "p:CT_ShowProperties/p:showPr"))]
  pub show_properties: Option<std::boxed::Box<ShowProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_ColorMRU/p:clrMru"))]
  pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
  /// _
  #[sdk(child(qname = "p:CT_PresentationPropertiesExtensionList/p:extLst"))]
  pub presentation_properties_extension_list: Option<PresentationPropertiesExtensionList>,
}
/// Presentation Slide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Slide/p:sld")]
pub struct Slide {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Show Master Shapes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterSp
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterPhAnim
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// Show Slide in Slide Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :show
  #[sdk(attr(qname = ":show"))]
  pub show: Option<crate::simple_type::BooleanValue>,
  ///Common slide data for slides
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map Override.
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  ///Slide Transition
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  ///Slide Timing Information for a Slide
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideExtensionList/p:extLst"))]
  pub slide_extension_list: Option<SlideExtensionList>,
}
/// Slide Layout.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldLayout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayout/p:sldLayout")]
pub struct SlideLayout {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Show Master Shapes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterSp
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterPhAnim
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  /// matchingName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :matchingName
  #[sdk(attr(qname = ":matchingName"))]
  pub matching_name: Option<crate::simple_type::StringValue>,
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<SlideLayoutValues>,
  /// preserve
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserve
  #[sdk(attr(qname = ":preserve"))]
  pub preserve: Option<crate::simple_type::BooleanValue>,
  /// userDrawn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :userDrawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map Override.
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// _
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// _
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideLayoutExtensionList/p:extLst"))]
  pub slide_layout_extension_list: Option<SlideLayoutExtensionList>,
}
/// Slide Master.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldMaster.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMaster/p:sldMaster")]
pub struct SlideMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// preserve
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserve
  #[sdk(attr(qname = ":preserve"))]
  pub preserve: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map.
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// _
  #[sdk(child(qname = "p:CT_SlideLayoutIdList/p:sldLayoutIdLst"))]
  pub slide_layout_id_list: Option<SlideLayoutIdList>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// _
  #[sdk(child(qname = "p:CT_SlideTransition/p:transition"))]
  pub transition: Option<std::boxed::Box<Transition>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideTiming/p:timing"))]
  pub timing: Option<std::boxed::Box<Timing>>,
  /// _
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideMasterTextStyles/p:txStyles"))]
  pub text_styles: Option<std::boxed::Box<TextStyles>>,
  /// _
  #[sdk(child(qname = "p:CT_SlideMasterExtensionList/p:extLst"))]
  pub slide_master_extension_list: Option<SlideMasterExtensionList>,
}
/// Handout Master.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:handoutMaster.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMaster/p:handoutMaster")]
pub struct HandoutMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map.
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// _
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// _
  #[sdk(child(qname = "p:CT_HandoutMasterExtensionList/p:extLst"))]
  pub handout_master_extension_list: Option<HandoutMasterExtensionList>,
}
/// Notes Master.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesMaster.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMaster/p:notesMaster")]
pub struct NotesMaster {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map.
  #[sdk(child(qname = "a:CT_ColorMapping/p:clrMap"))]
  pub color_map: std::boxed::Box<ColorMap>,
  /// _
  #[sdk(child(qname = "p:CT_HeaderFooter/p:hf"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// _
  #[sdk(child(qname = "a:CT_TextListStyle/p:notesStyle"))]
  pub notes_style: Option<std::boxed::Box<NotesStyle>>,
  /// _
  #[sdk(child(qname = "p:CT_NotesMasterExtensionList/p:extLst"))]
  pub notes_master_extension_list: Option<NotesMasterExtensionList>,
}
/// Notes Slide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesSlide/p:notes")]
pub struct NotesSlide {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Show Master Shapes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterSp
  #[sdk(attr(qname = ":showMasterSp"))]
  pub show_master_shapes: Option<crate::simple_type::BooleanValue>,
  /// Show Master Placeholder Animations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMasterPhAnim
  #[sdk(attr(qname = ":showMasterPhAnim"))]
  pub show_master_placeholder_animations: Option<crate::simple_type::BooleanValue>,
  ///Common slide data for notes slides
  #[sdk(child(qname = "p:CT_CommonSlideData/p:cSld"))]
  pub common_slide_data: std::boxed::Box<CommonSlideData>,
  ///Color Scheme Map Override.
  #[sdk(child(qname = "a:CT_ColorMappingOverride/p:clrMapOvr"))]
  pub color_map_override: Option<std::boxed::Box<ColorMapOverride>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Slide Synchronization Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldSyncPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSyncProperties/p:sldSyncPr")]
pub struct SlideSyncProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Server's Slide File ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverSldId
  #[sdk(attr(qname = ":serverSldId"))]
  pub server_slide_id: crate::simple_type::StringValue,
  /// Server's Slide File's modification date/time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverSldModifiedTime
  #[sdk(attr(qname = ":serverSldModifiedTime"))]
  pub server_slide_modified_time: crate::simple_type::DateTimeValue,
  /// Client Slide Insertion date/time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clientInsertedTime
  #[sdk(attr(qname = ":clientInsertedTime"))]
  pub client_inserted_time: crate::simple_type::DateTimeValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Programmable Tab List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tagLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TagList/p:tagLst")]
pub struct TagList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "p:CT_StringTag/p:tag"))]
  pub p_tag: Vec<Tag>,
}
/// Presentation-wide View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:viewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ViewProperties/p:viewPr")]
pub struct ViewProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Last View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lastView
  #[sdk(attr(qname = ":lastView"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub last_view: Option<ViewValues>,
  /// Show Comments
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showComments
  #[sdk(attr(qname = ":showComments"))]
  pub show_comments: Option<crate::simple_type::BooleanValue>,
  ///Normal View Properties
  #[sdk(child(qname = "p:CT_NormalViewProperties/p:normalViewPr"))]
  pub normal_view_properties: Option<std::boxed::Box<NormalViewProperties>>,
  ///Slide View Properties
  #[sdk(child(qname = "p:CT_SlideViewProperties/p:slideViewPr"))]
  pub slide_view_properties: Option<std::boxed::Box<SlideViewProperties>>,
  ///Outline View Properties
  #[sdk(child(qname = "p:CT_OutlineViewProperties/p:outlineViewPr"))]
  pub outline_view_properties: Option<std::boxed::Box<OutlineViewProperties>>,
  ///Notes Text View Properties
  #[sdk(child(qname = "p:CT_NotesTextViewProperties/p:notesTextViewPr"))]
  pub notes_text_view_properties: Option<std::boxed::Box<NotesTextViewProperties>>,
  ///Slide Sorter View Properties
  #[sdk(child(qname = "p:CT_SlideSorterViewProperties/p:sorterViewPr"))]
  pub sorter_view_properties: Option<std::boxed::Box<SorterViewProperties>>,
  ///Notes View Properties
  #[sdk(child(qname = "p:CT_NotesViewProperties/p:notesViewPr"))]
  pub notes_view_properties: Option<std::boxed::Box<NotesViewProperties>>,
  ///Grid Spacing
  #[sdk(child(qname = "a:CT_PositiveSize2D/p:gridSpacing"))]
  pub grid_spacing: Option<GridSpacing>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
#[cfg(feature = "microsoft365")]
/// Defines the ContentPart Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:contentPart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ContentPart/p:contentPart")]
pub struct ContentPart {
    /// bwMode
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: p14:bwMode
    #[sdk(attr(qname = "p14:bwMode"))]
    pub p14_bw_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// id
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: r:id
    #[sdk(attr(qname = "r:id"))]
    pub r_id: crate::simple_type::StringValue,
    /// _
    #[sdk(child(qname = "p14:CT_ContentPartNonVisual/p14:nvContentPartPr"))]
    pub non_visual_content_part_properties: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::NonVisualContentPartProperties,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_Transform2D/p14:xfrm"))]
    pub transform2_d: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Transform2D,
        >,
    >,
    /// _
    #[sdk(child(qname = "p:CT_ExtensionListModify/p14:extLst"))]
    pub extension_list_modify: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ExtensionListModify,
    >,
}
/// Sound.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:snd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/p:snd")]
pub struct Sound {
  /// Embedded Audio File Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: crate::simple_type::StringValue,
  /// Sound Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Recognized Built-In Sound
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :builtIn
  #[sdk(attr(qname = ":builtIn"))]
  pub built_in: Option<crate::simple_type::BooleanValue>,
}
/// Sound Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sndTgt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt")]
pub struct SoundTarget {
  /// Embedded Audio File Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: crate::simple_type::StringValue,
  /// Sound Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Recognized Built-In Sound
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :builtIn
  #[sdk(attr(qname = ":builtIn"))]
  pub built_in: Option<crate::simple_type::BooleanValue>,
}
/// Defines the EmbeddedWavAudioFileType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/")]
pub struct EmbeddedWavAudioFileType {
  /// Embedded Audio File Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: crate::simple_type::StringValue,
  /// Sound Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Recognized Built-In Sound
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :builtIn
  #[sdk(attr(qname = ":builtIn"))]
  pub built_in: Option<crate::simple_type::BooleanValue>,
}
/// Start Sound Action.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:stSnd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TransitionStartSoundAction/p:stSnd")]
pub struct StartSoundAction {
  /// Loop Sound
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :loop
  #[sdk(attr(qname = ":loop"))]
  pub r#loop: Option<crate::simple_type::BooleanValue>,
  ///Sound
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/p:snd"))]
  pub sound: std::boxed::Box<Sound>,
}
/// Time Absolute.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tmAbs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateIntervalTime/p:tmAbs")]
pub struct TimeAbsolute {
  /// Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 3u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 4u32, union = 1u64, values = &["indefinite"]))]
  pub val: crate::simple_type::StringValue,
}
/// Time Percentage.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tmPct.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateIntervalPercentage/p:tmPct")]
pub struct TimePercentage {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Target Element Trigger Choice.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tgtEl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeTargetElement/p:tgtEl")]
pub struct TargetElement {
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:sldTgt",
    qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt",
    qname = "p:CT_TLShapeTargetElement/p:spTgt",
    qname = "p:CT_TLSubShapeId/p:inkTgt"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt"))
  )]
  pub xml_children: Option<TargetElementChoice>,
}
/// Time Node.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTriggerTimeNodeID/p:tn")]
pub struct TimeNode {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Runtime Node Trigger Choice.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:rtn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTriggerRuntimeNode/p:rtn")]
pub struct RuntimeNodeTrigger {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub val: TriggerRuntimeNodeValues,
}
/// Condition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cond.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeCondition/p:cond")]
pub struct Condition {
  /// Trigger Event
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evt
  #[sdk(attr(qname = ":evt"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub event: Option<TriggerEventValues>,
  /// Trigger Delay
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delay
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
  pub xml_children: Option<ConditionChoice>,
}
/// Defines the EndSync Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:endSync.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeCondition/p:endSync")]
pub struct EndSync {
  /// Trigger Event
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evt
  #[sdk(attr(qname = ":evt"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub event: Option<TriggerEventValues>,
  /// Trigger Delay
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delay
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
  pub xml_children: Option<EndSyncChoice>,
}
/// Defines the TimeListConditionalType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeCondition/")]
pub struct TimeListConditionalType {
  /// Trigger Event
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evt
  #[sdk(attr(qname = ":evt"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub event: Option<TriggerEventValues>,
  /// Trigger Delay
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delay
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
  pub xml_children: Option<TimeListConditionalTypeChoice>,
}
/// Parallel Time Node.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:par.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeParallel/p:par")]
pub struct ParallelTimeNode {
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  ///Parallel TimeNode
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Sequence Time Node.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:seq.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeSequence/p:seq")]
pub struct SequenceTimeNode {
  /// Concurrent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :concurrent
  #[sdk(attr(qname = ":concurrent"))]
  pub concurrent: Option<crate::simple_type::BooleanValue>,
  /// Previous Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prevAc
  #[sdk(attr(qname = ":prevAc"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub previous_action: Option<PreviousActionValues>,
  /// Next Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nextAc
  #[sdk(attr(qname = ":nextAc"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub next_action: Option<NextActionValues>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  ///Common TimeNode Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  ///Previous Conditions List
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:prevCondLst"))]
  pub previous_condition_list: Option<PreviousConditionList>,
  ///Next Conditions List
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:nextCondLst"))]
  pub next_condition_list: Option<NextConditionList>,
}
/// Exclusive.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:excl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeNodeExclusive/p:excl")]
pub struct ExclusiveTimeNode {
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  ///Common TimeNode Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
}
/// Animate.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:anim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateBehavior/p:anim")]
pub struct Animate {
  /// by
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::StringValue>,
  /// from
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// to
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  /// calcmode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calcmode
  #[sdk(attr(qname = ":calcmode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub calculation_mode: Option<AnimateBehaviorCalculateModeValues>,
  /// valueType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :valueType
  #[sdk(attr(qname = ":valueType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub value_type: Option<AnimateBehaviorValues>,
  #[cfg(feature = "microsoft365")]
  /// bounceEnd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// _
  #[sdk(child(qname = "p:CT_TLTimeAnimateValueList/p:tavLst"))]
  pub time_animate_value_list: Option<TimeAnimateValueList>,
}
/// Animate Color Behavior.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:animClr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateColorBehavior/p:animClr")]
pub struct AnimateColor {
  /// Color Space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clrSpc
  #[sdk(attr(qname = ":clrSpc"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub color_space: Option<AnimateColorSpaceValues>,
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<AnimateColorDirectionValues>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  ///By
  #[sdk(child(qname = "p:CT_TLByAnimateColorTransform/p:by"))]
  pub by_color: Option<std::boxed::Box<ByColor>>,
  ///From
  #[sdk(child(qname = "a:CT_Color3/p:from"))]
  pub from_color: Option<std::boxed::Box<FromColor>>,
  ///To
  #[sdk(child(qname = "a:CT_Color3/p:to"))]
  pub to_color: Option<std::boxed::Box<ToColor>>,
}
/// Animate Effect.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:animEffect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect")]
pub struct AnimateEffect {
  /// Transition
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :transition
  #[sdk(attr(qname = ":transition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub transition: Option<AnimateEffectTransitionValues>,
  /// Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filter
  #[sdk(attr(qname = ":filter"))]
  pub filter: Option<crate::simple_type::StringValue>,
  /// Property List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prLst
  #[sdk(attr(qname = ":prLst"))]
  pub property_list: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  ///Progress
  #[sdk(child(qname = "p:CT_TLAnimFloat/p:progress"))]
  pub progress: Option<std::boxed::Box<Progress>>,
}
/// Animate Motion.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:animMotion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion")]
pub struct AnimateMotion {
  /// origin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :origin
  #[sdk(attr(qname = ":origin"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub origin: Option<AnimateMotionBehaviorOriginValues>,
  /// path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :path
  #[sdk(attr(qname = ":path"))]
  pub path: Option<crate::simple_type::StringValue>,
  /// pathEditMode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pathEditMode
  #[sdk(attr(qname = ":pathEditMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub path_edit_mode: Option<AnimateMotionPathEditModeValues>,
  /// rAng
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rAng
  #[sdk(attr(qname = ":rAng"))]
  pub relative_angle: Option<crate::simple_type::Int32Value>,
  /// ptsTypes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ptsTypes
  #[sdk(attr(qname = ":ptsTypes"))]
  pub point_types: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// bounceEnd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:by"))]
  pub by_position: Option<ByPosition>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:from"))]
  pub from_position: Option<FromPosition>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:to"))]
  pub to_position: Option<ToPosition>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:rCtr"))]
  pub rotation_center: Option<RotationCenter>,
}
/// Animate Rotation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:animRot.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateRotationBehavior/p:animRot")]
pub struct AnimateRotation {
  /// by
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::Int32Value>,
  /// from
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::Int32Value>,
  /// to
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::Int32Value>,
  #[cfg(feature = "microsoft365")]
  /// bounceEnd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Animate Scale.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:animScale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimateScaleBehavior/p:animScale")]
pub struct AnimateScale {
  /// zoomContents
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomContents
  #[sdk(attr(qname = ":zoomContents"))]
  pub zoom_contents: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// bounceEnd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:bounceEnd
  #[sdk(attr(qname = "p14:bounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub bounce_end: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:by"))]
  pub by_position: Option<ByPosition>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:from"))]
  pub from_position: Option<FromPosition>,
  /// _
  #[sdk(child(qname = "p:CT_TLPoint/p:to"))]
  pub to_position: Option<ToPosition>,
}
/// Command.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cmd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommandBehavior/p:cmd")]
pub struct Command {
  /// Command Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<CommandValues>,
  /// Command
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cmd
  #[sdk(attr(qname = ":cmd"))]
  pub command_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
}
/// Set Time Node Behavior.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSetBehavior/p:set")]
pub struct SetBehavior {
  ///Common Behavior
  #[sdk(child(qname = "p:CT_TLCommonBehaviorData/p:cBhvr"))]
  pub common_behavior: std::boxed::Box<CommonBehavior>,
  ///To
  #[sdk(child(qname = "p:CT_TLAnimVariant/p:to"))]
  pub to_variant_value: Option<std::boxed::Box<ToVariantValue>>,
}
/// Audio.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:audio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLMediaNodeAudio/p:audio")]
pub struct Audio {
  /// Is Narration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :isNarration
  #[sdk(attr(qname = ":isNarration"))]
  pub is_narration: Option<crate::simple_type::BooleanValue>,
  ///Common Media Node Properties
  #[sdk(child(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Video.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:video.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLMediaNodeVideo/p:video")]
pub struct Video {
  /// Full Screen
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fullScrn
  #[sdk(attr(qname = ":fullScrn"))]
  pub full_screen: Option<crate::simple_type::BooleanValue>,
  ///Common Media Node Properties
  #[sdk(child(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode"))]
  pub common_media_node: std::boxed::Box<CommonMediaNode>,
}
/// Parallel TimeNode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cTn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonTimeNodeData/p:cTn")]
pub struct CommonTimeNode {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// presetID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :presetID
  #[sdk(attr(qname = ":presetID"))]
  pub preset_id: Option<crate::simple_type::Int32Value>,
  /// presetClass
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :presetClass
  #[sdk(attr(qname = ":presetClass"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset_class: Option<TimeNodePresetClassValues>,
  /// presetSubtype
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :presetSubtype
  #[sdk(attr(qname = ":presetSubtype"))]
  pub preset_subtype: Option<crate::simple_type::Int32Value>,
  /// dur
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dur
  #[sdk(attr(qname = ":dur"))]
  pub duration: Option<crate::simple_type::StringValue>,
  /// repeatCount
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :repeatCount
  #[sdk(attr(qname = ":repeatCount"))]
  pub repeat_count: Option<crate::simple_type::StringValue>,
  /// repeatDur
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :repeatDur
  #[sdk(attr(qname = ":repeatDur"))]
  pub repeat_duration: Option<crate::simple_type::StringValue>,
  /// spd
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spd
  #[sdk(attr(qname = ":spd"))]
  pub speed: Option<crate::simple_type::Int32Value>,
  /// accel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accel
  #[sdk(attr(qname = ":accel"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub acceleration: Option<crate::simple_type::Int32Value>,
  /// decel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :decel
  #[sdk(attr(qname = ":decel"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub deceleration: Option<crate::simple_type::Int32Value>,
  /// autoRev
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoRev
  #[sdk(attr(qname = ":autoRev"))]
  pub auto_reverse: Option<crate::simple_type::BooleanValue>,
  /// restart
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :restart
  #[sdk(attr(qname = ":restart"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub restart: Option<TimeNodeRestartValues>,
  /// fill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fill
  #[sdk(attr(qname = ":fill"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub fill: Option<TimeNodeFillValues>,
  /// syncBehavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :syncBehavior
  #[sdk(attr(qname = ":syncBehavior"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub sync_behavior: Option<TimeNodeSyncValues>,
  /// tmFilter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tmFilter
  #[sdk(attr(qname = ":tmFilter"))]
  pub time_filter: Option<crate::simple_type::StringValue>,
  /// evtFilter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evtFilter
  #[sdk(attr(qname = ":evtFilter"))]
  pub event_filter: Option<crate::simple_type::StringValue>,
  /// display
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :display
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::BooleanValue>,
  /// masterRel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :masterRel
  #[sdk(attr(qname = ":masterRel"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub master_relation: Option<TimeNodeMasterRelationValues>,
  /// bldLvl
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bldLvl
  #[sdk(attr(qname = ":bldLvl"))]
  pub build_level: Option<crate::simple_type::Int32Value>,
  /// grpId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: Option<crate::simple_type::UInt32Value>,
  /// afterEffect
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :afterEffect
  #[sdk(attr(qname = ":afterEffect"))]
  pub after_effect: Option<crate::simple_type::BooleanValue>,
  /// nodeType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nodeType
  #[sdk(attr(qname = ":nodeType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub node_type: Option<TimeNodeValues>,
  /// nodePh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nodePh
  #[sdk(attr(qname = ":nodePh"))]
  pub node_placeholder: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// presetBounceEnd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:presetBounceEnd
  #[sdk(attr(qname = "p14:presetBounceEnd"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub preset_bounce_end: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:stCondLst"))]
  pub start_condition_list: Option<StartConditionList>,
  /// _
  #[sdk(child(qname = "p:CT_TLTimeConditionList/p:endCondLst"))]
  pub end_condition_list: Option<EndConditionList>,
  /// _
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:endSync"))]
  pub end_sync: Option<std::boxed::Box<EndSync>>,
  /// _
  #[sdk(child(qname = "p:CT_TLIterateData/p:iterate"))]
  pub iterate: Option<std::boxed::Box<Iterate>>,
  /// _
  #[sdk(child(qname = "p:CT_TimeNodeList/p:childTnLst"))]
  pub child_time_node_list: Option<ChildTimeNodeList>,
  /// _
  #[sdk(child(qname = "p:CT_TimeNodeList/p:subTnLst"))]
  pub sub_time_node_list: Option<SubTimeNodeList>,
}
/// Previous Conditions List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:prevCondLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:prevCondLst")]
pub struct PreviousConditionList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Next Conditions List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nextCondLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:nextCondLst")]
pub struct NextConditionList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Defines the StartConditionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:stCondLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:stCondLst")]
pub struct StartConditionList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Defines the EndConditionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:endCondLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/p:endCondLst")]
pub struct EndConditionList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub p_cond: Vec<Condition>,
}
/// Defines the TimeListTimeConditionalListType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeConditionList/")]
pub struct TimeListTimeConditionalListType {
  ///Condition.
  #[sdk(child(qname = "p:CT_TLTimeCondition/p:cond"))]
  pub condition: Vec<Condition>,
}
/// Attribute Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:attrName.
pub type AttributeName = crate::simple_type::StringValue;
/// Defines the Text Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:text.
pub type Text = crate::simple_type::StringValue;
/// Attribute Name List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:attrNameLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBehaviorAttributeNameList/p:attrNameLst")]
pub struct AttributeNameList {
  /// _
  #[sdk(text_child(qname = "xsd:string/p:attrName"))]
  pub p_attr_name: Vec<crate::simple_type::StringValue>,
}
/// Boolean Variant.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:boolVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal")]
pub struct BooleanVariantValue {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Integer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:intVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal")]
pub struct IntegerVariantValue {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Float Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:fltVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal")]
pub struct FloatVariantValue {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::SingleValue,
}
/// String Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:strVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariantStringVal/p:strVal")]
pub struct StringVariantValue {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Color Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:clrVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p:clrVal")]
pub struct ColorValue {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<ColorValueChoice>,
}
/// Pen Color for Slide Show.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:penClr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p:penClr")]
pub struct PenColor {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<PenColorChoice>,
}
/// Defines the ColorType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/")]
pub struct ColorType {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<ColorTypeChoice>,
}
/// Time Animate Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tav.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeAnimateValue/p:tav")]
pub struct TimeAnimateValue {
  /// Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tm
  #[sdk(attr(qname = ":tm"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  pub time: Option<crate::simple_type::StringValue>,
  /// Formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fmla
  #[sdk(attr(qname = ":fmla"))]
  pub fomula: Option<crate::simple_type::StringValue>,
  ///Value
  #[sdk(child(qname = "p:CT_TLAnimVariant/p:val"))]
  pub variant_value: Option<std::boxed::Box<VariantValue>>,
}
/// RGB.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:rgb.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByRgbColorTransform/p:rgb")]
pub struct RgbColor {
  /// Red
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub red: crate::simple_type::Int32Value,
  /// Green
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :g
  #[sdk(attr(qname = ":g"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub green: crate::simple_type::Int32Value,
  /// Blue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub blue: crate::simple_type::Int32Value,
}
/// HSL.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:hsl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByHslColorTransform/p:hsl")]
pub struct HslColor {
  /// Hue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :h
  #[sdk(attr(qname = ":h"))]
  pub hue: crate::simple_type::Int32Value,
  /// Saturation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub saturation: crate::simple_type::Int32Value,
  /// Lightness
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub lightness: crate::simple_type::Int32Value,
}
/// Defines the CommonBehavior Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cBhvr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonBehaviorData/p:cBhvr")]
pub struct CommonBehavior {
  /// Additive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :additive
  #[sdk(attr(qname = ":additive"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub additive: Option<BehaviorAdditiveValues>,
  /// Accumulate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accumulate
  #[sdk(attr(qname = ":accumulate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub accumulate: Option<BehaviorAccumulateValues>,
  /// Transform Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xfrmType
  #[sdk(attr(qname = ":xfrmType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub transform_type: Option<BehaviorTransformValues>,
  /// From
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :from
  #[sdk(attr(qname = ":from"))]
  pub from: Option<crate::simple_type::StringValue>,
  /// To
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :to
  #[sdk(attr(qname = ":to"))]
  pub to: Option<crate::simple_type::StringValue>,
  /// By
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :by
  #[sdk(attr(qname = ":by"))]
  pub by: Option<crate::simple_type::StringValue>,
  /// Runtime Context
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rctx
  #[sdk(attr(qname = ":rctx"))]
  pub runtime_context: Option<crate::simple_type::StringValue>,
  /// Override
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :override
  #[sdk(attr(qname = ":override"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#override: Option<BehaviorOverrideValues>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// _
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  ///Target Element
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
  ///Attribute Name List
  #[sdk(child(qname = "p:CT_TLBehaviorAttributeNameList/p:attrNameLst"))]
  pub attribute_name_list: Option<AttributeNameList>,
}
/// Progress.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:progress.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimFloat/p:progress")]
pub struct Progress {
  ///Float Value.
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  pub float_variant_value: std::boxed::Box<FloatVariantValue>,
}
/// To.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:to.
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
  pub xml_children: Option<ToVariantValueChoice>,
}
/// Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:val.
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
  pub xml_children: Option<VariantValueChoice>,
}
/// Defines the TimeListAnimationVariantType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLAnimVariant/")]
pub struct TimeListAnimationVariantType {
  #[sdk(choice(
    qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal",
    qname = "p:CT_TLAnimVariantIntegerVal/p:intVal",
    qname = "p:CT_TLAnimVariantFloatVal/p:fltVal",
    qname = "p:CT_TLAnimVariantStringVal/p:strVal",
    qname = "a:CT_Color/p:clrVal"
  ))]
  pub xml_children: Option<TimeListAnimationVariantTypeChoice>,
}
/// Common Media Node Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cMediaNode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLCommonMediaNodeData/p:cMediaNode")]
pub struct CommonMediaNode {
  /// Volume
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vol
  #[sdk(attr(qname = ":vol"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub volume: Option<crate::simple_type::Int32Value>,
  /// Mute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mute
  #[sdk(attr(qname = ":mute"))]
  pub mute: Option<crate::simple_type::BooleanValue>,
  /// Number of Slides
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numSld
  #[sdk(attr(qname = ":numSld"))]
  pub slide_count: Option<crate::simple_type::UInt32Value>,
  /// Show When Stopped
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showWhenStopped
  #[sdk(attr(qname = ":showWhenStopped"))]
  pub show_when_stopped: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  ///Common Time Node Properties
  #[sdk(child(qname = "p:CT_TLCommonTimeNodeData/p:cTn"))]
  pub common_time_node: std::boxed::Box<CommonTimeNode>,
  /// _
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  pub target_element: std::boxed::Box<TargetElement>,
}
/// Time Node List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tnLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_RootTimeNode/p:tnLst")]
pub struct TimeNodeList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  pub parallel_time_node: std::boxed::Box<ParallelTimeNode>,
}
/// Template Effects.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tmpl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTemplate/p:tmpl")]
pub struct Template {
  /// Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lvl
  #[sdk(attr(qname = ":lvl"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  ///Time Node List
  #[sdk(child(qname = "p:CT_RootTimeNode/p:tnLst"))]
  pub time_node_list: std::boxed::Box<TimeNodeList>,
}
/// Template effects.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tmplLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTemplateList/p:tmplLst")]
pub struct TemplateList {
  /// _
  #[sdk(child(qname = "p:CT_TLTemplate/p:tmpl"))]
  pub p_tmpl: Vec<Template>,
}
/// Build Sub Elements.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldSub.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub")]
pub struct BuildSubElement {
  #[sdk(choice(
    qname = "a:CT_AnimationDgmBuildProperties/a:bldDgm",
    qname = "a:CT_AnimationChartBuildProperties/a:bldChart"
  ))]
  pub xml_children: Option<BuildSubElementChoice>,
}
/// Build Paragraph.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldP.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBuildParagraph/p:bldP")]
pub struct BuildParagraph {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiExpand
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Build Types
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :build
  #[sdk(attr(qname = ":build"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub build: Option<ParagraphBuildValues>,
  /// Build Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bldLvl
  #[sdk(attr(qname = ":bldLvl"))]
  pub build_level: Option<crate::simple_type::UInt32Value>,
  /// Animate Background
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :animBg
  #[sdk(attr(qname = ":animBg"))]
  pub animate_background: Option<crate::simple_type::BooleanValue>,
  /// Auto Update Animation Background
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoUpdateAnimBg
  #[sdk(attr(qname = ":autoUpdateAnimBg"))]
  pub auto_animate_background: Option<crate::simple_type::BooleanValue>,
  /// Reverse
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// Auto Advance Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :advAuto
  #[sdk(attr(qname = ":advAuto"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["indefinite"]))]
  #[sdk(string_format(source = 2u32, union = 1u64, kind = "token"))]
  #[sdk(string_set(source = 3u32, union = 1u64, values = &["indefinite"]))]
  pub auto_advance: Option<crate::simple_type::StringValue>,
  ///Template effects
  #[sdk(child(qname = "p:CT_TLTemplateList/p:tmplLst"))]
  pub template_list: Option<TemplateList>,
}
/// Build Diagram.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldDgm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLBuildDiagram/p:bldDgm")]
pub struct BuildDiagram {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiExpand
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Diagram Build Types
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bld
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub build: Option<DiagramBuildValues>,
}
/// Build OLE Chart.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldOleChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLOleBuildChart/p:bldOleChart")]
pub struct BuildOleChart {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiExpand
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  /// Build
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bld
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub build: Option<OleChartBuildValues>,
  /// Animate Background
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :animBg
  #[sdk(attr(qname = ":animBg"))]
  pub animate_background: Option<crate::simple_type::BooleanValue>,
}
/// Build Graphics.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldGraphic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic")]
pub struct BuildGraphics {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  #[sdk(number_type(source = 3u32, union = 0u64, type_name = "a:ST_DrawingElementId"))]
  pub shape_id: crate::simple_type::StringValue,
  /// Group ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grpId
  #[sdk(attr(qname = ":grpId"))]
  pub group_id: crate::simple_type::UInt32Value,
  /// Expand UI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiExpand
  #[sdk(attr(qname = ":uiExpand"))]
  pub ui_expand: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "p:CT_Empty/p:bldAsOne",
    qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub"
  ))]
  pub xml_children: Option<BuildGraphicsChoice>,
}
/// Build List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bldLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_BuildList/p:bldLst")]
pub struct BuildList {
  #[sdk(choice(
    qname = "p:CT_TLBuildParagraph/p:bldP",
    qname = "p:CT_TLBuildDiagram/p:bldDgm",
    qname = "p:CT_TLOleBuildChart/p:bldOleChart",
    qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic"
  ))]
  pub xml_children: Vec<BuildListChoice>,
}
/// Defines the ExtensionListWithModification Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionListModify/p:extLst")]
pub struct ExtensionListWithModification {
  /// Modify
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mod
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension: Vec<Extension>,
}
/// By.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:by.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLByAnimateColorTransform/p:by")]
pub struct ByColor {
  #[sdk(choice(
    qname = "p:CT_TLByRgbColorTransform/p:rgb",
    qname = "p:CT_TLByHslColorTransform/p:hsl"
  ))]
  pub xml_children: Option<ByColorChoice>,
}
/// From.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:from.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color3/p:from")]
pub struct FromColor {
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<FromColorChoice>,
}
/// To.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:to.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color3/p:to")]
pub struct ToColor {
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<ToColorChoice>,
}
/// Defines the Color3Type Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color3/")]
pub struct Color3Type {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Color3TypeChoice>,
}
/// Presentation Slide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideRelationshipListEntry/p:sld")]
pub struct SlideListEntry {
  /// Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:custData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomerData/p:custData")]
pub struct CustomerData {
  /// Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Customer Data Tags.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tags.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TagsData/p:tags")]
pub struct CustomerDataTags {
  /// Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Comment Author.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cmAuthor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthor/p:cmAuthor")]
pub struct CommentAuthor {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// initials
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :initials
  #[sdk(attr(qname = ":initials"))]
  pub initials: crate::simple_type::StringValue,
  /// lastIdx
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lastIdx
  #[sdk(attr(qname = ":lastIdx"))]
  pub last_index: crate::simple_type::UInt32Value,
  /// clrIdx
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clrIdx
  #[sdk(attr(qname = ":clrIdx"))]
  pub color_index: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "p:CT_CommentAuthorExtensionList/p:extLst"))]
  pub comment_author_extension_list: Option<CommentAuthorExtensionList>,
}
/// Comment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Comment/p:cm")]
pub struct Comment {
  /// authorId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: crate::simple_type::UInt32Value,
  /// dt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dt
  #[sdk(attr(qname = ":dt"))]
  pub date_time: Option<crate::simple_type::DateTimeValue>,
  /// idx
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "a:CT_Point2D/p:pos"))]
  pub position: std::boxed::Box<Position>,
  /// _
  #[sdk(text_child(qname = "xsd:string/p:text"))]
  pub text: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_CommentExtensionList/p:extLst"))]
  pub comment_extension_list: Option<CommentExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension: Vec<Extension>,
}
/// Embedded Control.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Control/p:control")]
pub struct Control {
  /// spid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub shape_id: Option<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// showAsIcon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAsIcon
  #[sdk(attr(qname = ":showAsIcon"))]
  pub show_as_icon: Option<crate::simple_type::BooleanValue>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// imgW
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imgW
  #[sdk(attr(qname = ":imgW"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub image_width: Option<crate::simple_type::Int32Value>,
  /// imgH
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imgH
  #[sdk(attr(qname = ":imgH"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub image_height: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
  /// _
  #[sdk(child(qname = "p:CT_Picture/p:pic"))]
  pub picture: Option<std::boxed::Box<Picture>>,
}
/// Slide ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideIdListEntry/p:sldId")]
pub struct SlideId {
  /// Slide Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(
    source = 1u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub id: crate::simple_type::UInt32Value,
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Master ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldMasterId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterIdListEntry/p:sldMasterId")]
pub struct SlideMasterId {
  /// Slide Master Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Master ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesMasterId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterIdListEntry/p:notesMasterId")]
pub struct NotesMasterId {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Handout Master ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:handoutMasterId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterIdListEntry/p:handoutMasterId")]
pub struct HandoutMasterId {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Embedded Font Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextFont/p:font")]
pub struct Font {
  /// Text Typeface
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :panose
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
  pub panose: Option<crate::simple_type::HexBinaryValue>,
  /// Similar Font Family
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pitchFamily
  #[sdk(attr(qname = ":pitchFamily"))]
  pub pitch_family: Option<crate::simple_type::SByteValue>,
  /// Similar Character Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :charset
  #[sdk(attr(qname = ":charset"))]
  pub character_set: Option<crate::simple_type::SByteValue>,
}
/// Regular Embedded Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:regular.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:regular")]
pub struct RegularFont {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Embedded Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bold.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:bold")]
pub struct BoldFont {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Italic Embedded Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:italic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:italic")]
pub struct ItalicFont {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Bold Italic Embedded Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:boldItalic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/p:boldItalic")]
pub struct BoldItalicFont {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the EmbeddedFontDataIdType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontDataId/")]
pub struct EmbeddedFontDataIdType {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Embedded Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:embeddedFont.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontListEntry/p:embeddedFont")]
pub struct EmbeddedFont {
  ///Embedded Font Name
  #[sdk(child(qname = "a:CT_TextFont/p:font"))]
  pub font: std::boxed::Box<Font>,
  ///Regular Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:regular"))]
  pub regular_font: Option<RegularFont>,
  ///Bold Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:bold"))]
  pub bold_font: Option<BoldFont>,
  ///Italic Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:italic"))]
  pub italic_font: Option<ItalicFont>,
  ///Bold Italic Embedded Font
  #[sdk(child(qname = "p:CT_EmbeddedFontDataId/p:boldItalic"))]
  pub bold_italic_font: Option<BoldItalicFont>,
}
/// List of Presentation Slides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideRelationshipList/p:sldLst")]
pub struct SlideList {
  /// _
  #[sdk(child(qname = "p:CT_SlideRelationshipListEntry/p:sld"))]
  pub p_sld: Vec<SlideListEntry>,
}
/// Custom Show.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:custShow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShow/p:custShow")]
pub struct CustomShow {
  /// Custom Show Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Custom Show ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  ///List of Presentation Slides
  #[sdk(child(qname = "p:CT_SlideRelationshipList/p:sldLst"))]
  pub slide_list: std::boxed::Box<SlideList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/p:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :id
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// Name compatible with Object Model (non-unique).
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// Description of the drawing element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :descr
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// Flag determining to show or hide this element.
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :hidden
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// Title
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :title
    #[sdk(attr(qname = ":title"))]
    pub title: Option<crate::simple_type::StringValue>,
    ///Hyperlink associated with clicking or selecting the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    ///Hyperlink associated with hovering over the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    ///Future extension
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Non-Visual Drawing Properties for a Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/p:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  /// Text Box
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :txBox
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  ///Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Application Non-Visual Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
  /// Is a Photo Album
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :isPhoto
  #[sdk(attr(qname = ":isPhoto"))]
  pub is_photo: Option<crate::simple_type::BooleanValue>,
  /// Is User Drawn
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :userDrawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub user_drawn: Option<crate::simple_type::BooleanValue>,
  ///Placeholder Shape
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
  /// _
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub p_cust_data_lst: Option<std::boxed::Box<CustomerDataList>>,
  /// _
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ApplicationNonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Properties for a Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShapeNonVisual/p:nvSpPr")]
pub struct NonVisualShapeProperties {
  ///Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Drawing Properties for a Shape
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/p:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
  ///Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/p:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Shape Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/p:style")]
pub struct ShapeStyle {
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectReference,
  >,
  ///Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Shape Text Body.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:txBody.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/p:txBody")]
pub struct TextBody {
  ///Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ///Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Non-Visual Connector Shape Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvCxnSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/p:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  ///Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
  >,
  ///Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection>,
  ///Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvCxnSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ConnectorNonVisual/p:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  ///Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/p:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
  ///Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Non-Visual Picture Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvPicPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/p:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[sdk(attr(qname = ":preferRelativeResize"))]
    pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
    /// _
    #[sdk(child(qname = "a:CT_PictureLocking/a:picLocks"))]
    pub picture_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst"))]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Properties for a Picture.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvPicPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PictureNonVisual/p:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/p:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
  /// _
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Picture Fill.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:blipFill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties/p:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dpi
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rotWithShape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip:
    Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip>>,
  ///Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle>,
  #[sdk(choice(
    qname = "a:CT_TileInfoProperties/a:tile",
    qname = "a:CT_StretchInfoProperties/a:stretch"
  ))]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Non-Visual Graphic Frame Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/p:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  ///Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrameLocks,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvGraphicFramePr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GraphicalObjectFrameNonVisual/p:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  ///Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/p:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
  ///Application Non-Visual Drawing Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// 2D Transform for Graphic Frame.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:xfrm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/p:xfrm")]
pub struct Transform {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipH
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :flipV
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  ///Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  ///Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Non-Visual Group Shape Drawing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cNvGrpSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/p:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
    /// _
    #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
    pub group_shape_locks: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
        >,
    >,
    /// _
    #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Slide Master Title Text Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:titleStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:titleStyle")]
pub struct TitleStyle {
  ///Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  >,
  ///List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  >,
  ///List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  >,
  ///List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  >,
  ///List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  >,
  ///List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  >,
  ///List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  >,
  ///List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  >,
  ///List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  >,
  ///List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Slide Master Body Text Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bodyStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:bodyStyle")]
pub struct BodyStyle {
  ///Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  >,
  ///List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  >,
  ///List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  >,
  ///List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  >,
  ///List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  >,
  ///List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  >,
  ///List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  >,
  ///List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  >,
  ///List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  >,
  ///List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Slide Master Other Text Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:otherStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:otherStyle")]
pub struct OtherStyle {
  ///Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  >,
  ///List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  >,
  ///List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  >,
  ///List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  >,
  ///List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  >,
  ///List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  >,
  ///List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  >,
  ///List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  >,
  ///List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  >,
  ///List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the DefaultTextStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:defaultTextStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:defaultTextStyle")]
pub struct DefaultTextStyle {
  ///Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  >,
  ///List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  >,
  ///List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  >,
  ///List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  >,
  ///List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  >,
  ///List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  >,
  ///List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  >,
  ///List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  >,
  ///List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  >,
  ///List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the NotesStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/p:notesStyle")]
pub struct NotesStyle {
  ///Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  >,
  ///List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  >,
  ///List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  >,
  ///List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  >,
  ///List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  >,
  ///List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  >,
  ///List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  >,
  ///List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  >,
  ///List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  >,
  ///List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextListStyleType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/")]
pub struct TextListStyleType {
  #[sdk(choice(
    qname = "a:CT_TextParagraphProperties/a:defPPr",
    qname = "a:CT_TextParagraphProperties/a:lvl1pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl2pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl3pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl4pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl5pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl6pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl7pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl8pPr",
    qname = "a:CT_TextParagraphProperties/a:lvl9pPr",
    qname = "a:CT_OfficeArtExtensionList/a:extLst"
  ))]
  pub xml_children: Vec<TextListStyleTypeChoice>,
}
/// Slide Layout Id.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldLayoutId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutIdListEntry/p:sldLayoutId")]
pub struct SlideLayoutId {
  /// ID Tag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// ID Tag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Common slide data for notes slides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cSld.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideData/p:cSld")]
pub struct CommonSlideData {
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  ///Slide Background
  #[sdk(child(qname = "p:CT_Background/p:bg"))]
  pub background: Option<std::boxed::Box<Background>>,
  ///Shape Tree
  #[sdk(child(qname = "p:CT_GroupShape/p:spTree"))]
  pub shape_tree: std::boxed::Box<ShapeTree>,
  ///Customer Data List
  #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
  pub customer_data_list: Option<std::boxed::Box<CustomerDataList>>,
  ///List of controls
  #[sdk(child(qname = "p:CT_ControlList/p:controls"))]
  pub control_list: Option<ControlList>,
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideDataExtensionList/p:extLst"))]
  pub common_slide_data_extension_list: Option<CommonSlideDataExtensionList>,
}
/// Programmable Extensibility Tag.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tag.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_StringTag/p:tag")]
pub struct Tag {
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Normal View Restored Left Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:restoredLeft.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewPortion/p:restoredLeft")]
pub struct RestoredLeft {
  /// Normal View Dimension Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: crate::simple_type::Int32Value,
  /// Auto Adjust Normal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoAdjust
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// Normal View Restored Top Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:restoredTop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewPortion/p:restoredTop")]
pub struct RestoredTop {
  /// Normal View Dimension Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: crate::simple_type::Int32Value,
  /// Auto Adjust Normal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoAdjust
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// Defines the NormalViewPortionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewPortion/")]
pub struct NormalViewPortionType {
  /// Normal View Dimension Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: crate::simple_type::Int32Value,
  /// Auto Adjust Normal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoAdjust
  #[sdk(attr(qname = ":autoAdjust"))]
  pub auto_adjust: Option<crate::simple_type::BooleanValue>,
}
/// View Scale.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Scale2D/p:scale")]
pub struct ScaleFactor {
  ///Horizontal Ratio
  #[sdk(child(qname = "a:CT_Ratio/a:sx"))]
  pub scale_x:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ScaleX>,
  ///Vertical Ratio
  #[sdk(child(qname = "a:CT_Ratio/a:sy"))]
  pub scale_y:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ScaleY>,
}
/// View Origin.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:origin.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/p:origin")]
pub struct Origin {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the Position Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:pos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/p:pos")]
pub struct Position {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Defines the Point2DType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/")]
pub struct Point2DType {
  /// X-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Axis Coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
}
/// Base properties for Notes View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonViewProperties/p:cViewPr")]
pub struct CommonViewProperties {
  /// Variable Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :varScale
  #[sdk(attr(qname = ":varScale"))]
  pub variable_scale: Option<crate::simple_type::BooleanValue>,
  ///View Scale
  #[sdk(child(qname = "a:CT_Scale2D/p:scale"))]
  pub scale_factor: std::boxed::Box<ScaleFactor>,
  ///View Origin
  #[sdk(child(qname = "a:CT_Point2D/p:origin"))]
  pub origin: std::boxed::Box<Origin>,
}
/// Presentation Slide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewSlideEntry/p:sld")]
pub struct OutlineViewSlideListEntry {
  /// Relationship Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Collapsed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :collapse
  #[sdk(attr(qname = ":collapse"))]
  pub collapse: Option<crate::simple_type::BooleanValue>,
}
/// List of Presentation Slides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewSlideList/p:sldLst")]
pub struct OutlineViewSlideList {
  /// _
  #[sdk(child(qname = "p:CT_OutlineViewSlideEntry/p:sld"))]
  pub p_sld: Vec<OutlineViewSlideListEntry>,
}
/// A Guide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Guide/p:guide")]
pub struct Guide {
  /// Guide Orientation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// Guide Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
}
/// List of Guides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:guideLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GuideList/p:guideLst")]
pub struct GuideList {
  /// _
  #[sdk(child(qname = "p:CT_Guide/p:guide"))]
  pub p_guide: Vec<Guide>,
}
/// Defines the CommonSlideViewProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cSldViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr")]
pub struct CommonSlideViewProperties {
  /// Snap Objects to Grid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :snapToGrid
  #[sdk(attr(qname = ":snapToGrid"))]
  pub snap_to_grid: Option<crate::simple_type::BooleanValue>,
  /// Snap Objects to Objects
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :snapToObjects
  #[sdk(attr(qname = ":snapToObjects"))]
  pub snap_to_objects: Option<crate::simple_type::BooleanValue>,
  /// Show Guides in View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showGuides
  #[sdk(attr(qname = ":showGuides"))]
  pub show_guides: Option<crate::simple_type::BooleanValue>,
  ///Base properties for Slide View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  ///List of Guides
  #[sdk(child(qname = "p:CT_GuideList/p:guideLst"))]
  pub guide_list: Option<GuideList>,
}
/// Normal View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:normalViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NormalViewProperties/p:normalViewPr")]
pub struct NormalViewProperties {
  /// Show Outline Icons in Normal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showOutlineIcons
  #[sdk(attr(qname = ":showOutlineIcons"))]
  pub show_outline_icons: Option<crate::simple_type::BooleanValue>,
  /// Snap Vertical Splitter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :snapVertSplitter
  #[sdk(attr(qname = ":snapVertSplitter"))]
  pub snap_vertical_splitter: Option<crate::simple_type::BooleanValue>,
  /// State of the Vertical Splitter Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vertBarState
  #[sdk(attr(qname = ":vertBarState"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_bar_state: Option<SplitterBarStateValues>,
  /// State of the Horizontal Splitter Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horzBarState
  #[sdk(attr(qname = ":horzBarState"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_bar_state: Option<SplitterBarStateValues>,
  /// Prefer Single View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preferSingleView
  #[sdk(attr(qname = ":preferSingleView"))]
  pub prefer_single_view: Option<crate::simple_type::BooleanValue>,
  ///Normal View Restored Left Properties
  #[sdk(child(qname = "p:CT_NormalViewPortion/p:restoredLeft"))]
  pub restored_left: std::boxed::Box<RestoredLeft>,
  ///Normal View Restored Top Properties
  #[sdk(child(qname = "p:CT_NormalViewPortion/p:restoredTop"))]
  pub restored_top: std::boxed::Box<RestoredTop>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:slideViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideViewProperties/p:slideViewPr")]
pub struct SlideViewProperties {
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Outline View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:outlineViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OutlineViewProperties/p:outlineViewPr")]
pub struct OutlineViewProperties {
  ///Common View Properties
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  ///List of Presentation Slides
  #[sdk(child(qname = "p:CT_OutlineViewSlideList/p:sldLst"))]
  pub outline_view_slide_list: Option<OutlineViewSlideList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes Text View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesTextViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesTextViewProperties/p:notesTextViewPr")]
pub struct NotesTextViewProperties {
  ///Base properties for Notes View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Sorter View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sorterViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSorterViewProperties/p:sorterViewPr")]
pub struct SorterViewProperties {
  /// Show Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showFormatting
  #[sdk(attr(qname = ":showFormatting"))]
  pub show_formatting: Option<crate::simple_type::BooleanValue>,
  ///Base properties for Slide Sorter View
  #[sdk(child(qname = "p:CT_CommonViewProperties/p:cViewPr"))]
  pub common_view_properties: std::boxed::Box<CommonViewProperties>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Notes View Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesViewPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesViewProperties/p:notesViewPr")]
pub struct NotesViewProperties {
  ///Common Slide View Properties
  #[sdk(child(qname = "p:CT_CommonSlideViewProperties/p:cSldViewPr"))]
  pub common_slide_view_properties: std::boxed::Box<CommonSlideViewProperties>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Grid Spacing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:gridSpacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/p:gridSpacing")]
pub struct GridSpacing {
  /// Extent Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: crate::simple_type::Int64Value,
}
/// Defines the NotesSize Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesSz.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/p:notesSz")]
pub struct NotesSize {
  /// Extent Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: crate::simple_type::Int64Value,
}
/// Defines the PositiveSize2DType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/")]
pub struct PositiveSize2DType {
  /// Extent Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: crate::simple_type::Int64Value,
  /// Extent Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: crate::simple_type::Int64Value,
}
/// Defines the SlideExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideExtension/p:ext")]
pub struct SlideExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "p14:CT_LaserTraceList/p14:laserTraceLst",
      qname = "p14:CT_ShowEventRecordList/p14:showEvtLst",
      qname = "p188:CT_CommentRelationship/p188:commentRel"
    ))
  )]
  pub xml_children: Option<SlideExtensionChoice>,
}
/// Defines the CommonSlideDataExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideDataExtension/p:ext")]
pub struct CommonSlideDataExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p14:CT_RandomId/p14:creationId"))
  )]
  pub xml_children: Option<CommonSlideDataExtensionChoice>,
}
/// Defines the ShowPropertiesExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowPropertiesExtension/p:ext")]
pub struct ShowPropertiesExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "p14:CT_BrowseMode/p14:browseMode",
      qname = "a:CT_Color/p14:laserClr",
      qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls"
    ))
  )]
  pub xml_children: Option<ShowPropertiesExtensionChoice>,
}
/// Defines the Picture Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:pic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Picture/p:pic")]
pub struct Picture {
  ///Non-Visual Properties for a Picture
  #[sdk(child(qname = "p:CT_PictureNonVisual/p:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  ///Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties/p:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the OleObjectEmbed Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:embed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObjectEmbed/p:embed")]
pub struct OleObjectEmbed {
  /// Color Scheme Properties for OLE Object
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :followColorScheme
  #[sdk(attr(qname = ":followColorScheme"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub follow_color_scheme: Option<OleObjectFollowColorSchemeValues>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OleObjectLink Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:link.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OleObjectLink/p:link")]
pub struct OleObjectLink {
  /// Update Linked OLE Objects Automatically
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :updateAutomatic
  #[sdk(attr(qname = ":updateAutomatic"))]
  pub auto_update: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Slide Transition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:transition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideTransition/p:transition")]
pub struct Transition {
  /// spd
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spd
  #[sdk(attr(qname = ":spd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub speed: Option<TransitionSpeedValues>,
  #[cfg(feature = "microsoft365")]
  /// dur
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: p14:dur
  #[sdk(attr(qname = "p14:dur"))]
  pub duration: Option<crate::simple_type::StringValue>,
  /// Specifies whether a mouse click will advance the slide.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :advClick
  #[sdk(attr(qname = ":advClick"))]
  pub advance_on_click: Option<crate::simple_type::BooleanValue>,
  /// advTm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :advTm
  #[sdk(attr(qname = ":advTm"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub advance_after_time: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
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
    qname = "p:CT_InOutTransition/p:zoom"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
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
    ))
  )]
  pub transition_choice: Option<TransitionChoice>,
  /// _
  #[sdk(child(qname = "p:CT_TransitionSoundAction/p:sndAc"))]
  pub p_snd_ac: Option<std::boxed::Box<SoundAction>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Slide Timing Information for a Slide.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:timing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideTiming/p:timing")]
pub struct Timing {
  /// _
  #[sdk(child(qname = "p:CT_RootTimeNode/p:tnLst"))]
  pub time_node_list: Option<std::boxed::Box<TimeNodeList>>,
  ///Build List
  #[sdk(child(qname = "p:CT_BuildList/p:bldLst"))]
  pub build_list: Option<BuildList>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideExtensionList/p:extLst")]
pub struct SlideExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_SlideExtension/p:ext"))]
  pub p_ext: Vec<SlideExtension>,
}
/// Slide Background.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:bg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Background/p:bg")]
pub struct Background {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  #[sdk(choice(
    qname = "p:CT_BackgroundProperties/p:bgPr",
    qname = "a:CT_StyleMatrixReference/p:bgRef"
  ))]
  pub xml_children: Option<BackgroundChoice>,
}
/// Shape Tree.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:spTree.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShape/p:spTree")]
pub struct ShapeTree {
  ///Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  ///Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/p:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "p:CT_Shape/p:sp",
    qname = "p:CT_GroupShape/p:grpSp",
    qname = "p:CT_GraphicalObjectFrame/p:graphicFrame",
    qname = "p:CT_Connector/p:cxnSp",
    qname = "p:CT_Picture/p:pic"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p:CT_ContentPart/p:contentPart"))
  )]
  pub shape_tree_choice: Vec<ShapeTreeChoice>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Group Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:grpSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShape/p:grpSp")]
pub struct GroupShape {
  ///Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  ///Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/p:grpSpPr"))]
  pub group_shape_properties: std::boxed::Box<GroupShapeProperties>,
  #[sdk(choice(
    qname = "mc:CT_AlternateContent/mc:AlternateContent",
    qname = "p:CT_Shape/p:sp",
    qname = "p:CT_GroupShape/p:grpSp",
    qname = "p:CT_GraphicalObjectFrame/p:graphicFrame",
    qname = "p:CT_Connector/p:cxnSp",
    qname = "p:CT_Picture/p:pic"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p:CT_ContentPart/p:contentPart"))
  )]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub p_ext_lst: Option<ExtensionListWithModification>,
}
/// Defines the GroupShapeType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShape/")]
pub struct GroupShapeType {
  #[sdk(choice(
    qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr",
    qname = "a:CT_GroupShapeProperties/p:grpSpPr",
    qname = "p:CT_Shape/p:sp",
    qname = "p:CT_GroupShape/p:grpSp",
    qname = "p:CT_GraphicalObjectFrame/p:graphicFrame",
    qname = "p:CT_Connector/p:cxnSp",
    qname = "p:CT_Picture/p:pic",
    qname = "p:CT_ExtensionListModify/p:extLst"
  ))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p:CT_ContentPart/p:contentPart"))
  )]
  pub xml_children: Vec<GroupShapeTypeChoice>,
}
/// Customer Data List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:custDataLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomerDataList/p:custDataLst")]
pub struct CustomerDataList {
  /// _
  #[sdk(child(qname = "p:CT_CustomerData/p:custData"))]
  pub p_cust_data: Vec<CustomerData>,
  /// _
  #[sdk(child(qname = "p:CT_TagsData/p:tags"))]
  pub p_tags: Option<CustomerDataTags>,
}
/// List of controls.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:controls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ControlList/p:controls")]
pub struct ControlList {
  /// _
  #[sdk(child(qname = "p:CT_Control/p:control"))]
  pub p_control: Vec<Control>,
}
/// Defines the CommonSlideDataExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommonSlideDataExtensionList/p:extLst")]
pub struct CommonSlideDataExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_CommonSlideDataExtension/p:ext"))]
  pub p_ext: Vec<CommonSlideDataExtension>,
}
/// Non-Visual Properties for a Group Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:nvGrpSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  ///Non-visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/p:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  ///Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/p:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
  ///Non-Visual Properties
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingProps/p:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Group Shape Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:grpSpPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/p:grpSpPr")]
pub struct GroupShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  ///2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup>,
  >,
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
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Shape/p:sp")]
pub struct Shape {
  /// Use Background Fill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useBgFill
  #[sdk(attr(qname = ":useBgFill"))]
  pub use_background_fill: Option<crate::simple_type::BooleanValue>,
  ///Non-Visual Properties for a Shape
  #[sdk(child(qname = "p:CT_ShapeNonVisual/p:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  ///Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  ///Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/p:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Graphic Frame.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:graphicFrame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_GraphicalObjectFrame/p:graphicFrame")]
pub struct GraphicFrame {
  ///Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "p:CT_GraphicalObjectFrameNonVisual/p:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  ///2D Transform for Graphic Frame
  #[sdk(child(qname = "a:CT_Transform2D/p:xfrm"))]
  pub transform: std::boxed::Box<Transform>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic>,
  ///Extension List with Modification Flag
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Connection Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cxnSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Connector/p:cxnSp")]
pub struct ConnectionShape {
  ///Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "p:CT_ConnectorNonVisual/p:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  ///Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/p:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  ///Connector Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/p:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ShowPropertiesExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowPropertiesExtensionList/p:extLst")]
pub struct ShowPropertiesExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_ShowPropertiesExtension/p:ext"))]
  pub p_ext: Vec<ShowPropertiesExtension>,
}
/// Shape Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:spTgt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLShapeTargetElement/p:spTgt")]
pub struct ShapeTarget {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
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
  pub xml_children: Option<ShapeTargetChoice>,
}
/// Ink Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:inkTgt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSubShapeId/p:inkTgt")]
pub struct InkTarget {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Subshape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:subSp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSubShapeId/p:subSp")]
pub struct SubShape {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Defines the TimeListSubShapeIdType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLSubShapeId/")]
pub struct TimeListSubShapeIdType {
  /// Shape ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spid
  #[sdk(attr(qname = ":spid"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub shape_id: crate::simple_type::StringValue,
}
/// Defines the CommentAuthorExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorExtension/p:ext")]
pub struct CommentAuthorExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_PresenceInfo/p15:presenceInfo"))
  )]
  pub xml_children: Option<CommentAuthorExtensionChoice>,
}
/// Defines the CommentExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentExtension/p:ext")]
pub struct CommentExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_CommentThreading/p15:threadingInfo"))
  )]
  pub xml_children: Option<CommentExtensionChoice>,
}
/// Defines the SlideLayoutExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutExtension/p:ext")]
pub struct SlideLayoutExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))
  )]
  pub xml_children: Option<SlideLayoutExtensionChoice>,
}
/// Defines the SlideMasterExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterExtension/p:ext")]
pub struct SlideMasterExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))
  )]
  pub xml_children: Option<SlideMasterExtensionChoice>,
}
/// Defines the HandoutMasterExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterExtension/p:ext")]
pub struct HandoutMasterExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))
  )]
  pub xml_children: Option<HandoutMasterExtensionChoice>,
}
/// Defines the NotesMasterExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterExtension/p:ext")]
pub struct NotesMasterExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))
  )]
  pub xml_children: Option<NotesMasterExtensionChoice>,
}
/// Placeholder Shape.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ph.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Placeholder/p:ph")]
pub struct PlaceholderShape {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<PlaceholderValues>,
  /// orient
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// sz
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub size: Option<PlaceholderSizeValues>,
  /// idx
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// hasCustomPrompt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hasCustomPrompt
  #[sdk(attr(qname = ":hasCustomPrompt"))]
  pub has_custom_prompt: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst")]
pub struct ApplicationNonVisualDrawingPropertiesExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingPropsExtension/p:ext"))]
  pub p_ext: Vec<ApplicationNonVisualDrawingPropertiesExtension>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ApplicationNonVisualDrawingPropsExtension/p:ext")]
pub struct ApplicationNonVisualDrawingPropertiesExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "p14:CT_Media/p14:media", qname = "p14:CT_RandomId/p14:modId"))
  )]
  pub xml_children: Option<ApplicationNonVisualDrawingPropertiesExtensionChoice>,
}
/// Defines the Iterate Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:iterate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLIterateData/p:iterate")]
pub struct Iterate {
  /// Iterate Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<IterateValues>,
  /// Backwards
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :backwards
  #[sdk(attr(qname = ":backwards"))]
  pub backwards: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "p:CT_TLIterateIntervalTime/p:tmAbs",
    qname = "p:CT_TLIterateIntervalPercentage/p:tmPct"
  ))]
  pub xml_children: Option<IterateChoice>,
}
/// Defines the ChildTimeNodeList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:childTnLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TimeNodeList/p:childTnLst")]
pub struct ChildTimeNodeList {
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
    qname = "p:CT_TLMediaNodeVideo/p:video"
  ))]
  pub xml_children: Vec<ChildTimeNodeListChoice>,
}
/// Defines the SubTimeNodeList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:subTnLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TimeNodeList/p:subTnLst")]
pub struct SubTimeNodeList {
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
    qname = "p:CT_TLMediaNodeVideo/p:video"
  ))]
  pub xml_children: Vec<SubTimeNodeListChoice>,
}
/// Defines the TimeTypeListType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TimeNodeList/")]
pub struct TimeTypeListType {
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
    qname = "p:CT_TLMediaNodeVideo/p:video"
  ))]
  pub xml_children: Vec<TimeTypeListTypeChoice>,
}
/// Defines the TimeAnimateValueList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:tavLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTimeAnimateValueList/p:tavLst")]
pub struct TimeAnimateValueList {
  /// _
  #[sdk(child(qname = "p:CT_TLTimeAnimateValue/p:tav"))]
  pub p_tav: Vec<TimeAnimateValue>,
}
/// Defines the ByPosition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:by.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:by")]
pub struct ByPosition {
  /// X coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the FromPosition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:from.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:from")]
pub struct FromPosition {
  /// X coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the ToPosition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:to.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:to")]
pub struct ToPosition {
  /// X coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the RotationCenter Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:rCtr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/p:rCtr")]
pub struct RotationCenter {
  /// X coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the TimeListType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLPoint/")]
pub struct TimeListType {
  /// X coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: crate::simple_type::Int32Value,
  /// Y coordinate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  pub y: crate::simple_type::Int32Value,
}
/// Defines the CommentAuthorExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentAuthorExtensionList/p:extLst")]
pub struct CommentAuthorExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_CommentAuthorExtension/p:ext"))]
  pub p_ext: Vec<CommentAuthorExtension>,
}
/// Defines the CommentExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CommentExtensionList/p:extLst")]
pub struct CommentExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_CommentExtension/p:ext"))]
  pub p_ext: Vec<CommentExtension>,
}
/// Defines the SlideMasterIdList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldMasterIdLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterIdList/p:sldMasterIdLst")]
pub struct SlideMasterIdList {
  /// _
  #[sdk(child(qname = "p:CT_SlideMasterIdListEntry/p:sldMasterId"))]
  pub p_sld_master_id: Vec<SlideMasterId>,
}
/// Defines the NotesMasterIdList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:notesMasterIdLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterIdList/p:notesMasterIdLst")]
pub struct NotesMasterIdList {
  ///Notes Master ID
  #[sdk(child(qname = "p:CT_NotesMasterIdListEntry/p:notesMasterId"))]
  pub notes_master_id: Option<std::boxed::Box<NotesMasterId>>,
}
/// Defines the HandoutMasterIdList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:handoutMasterIdLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterIdList/p:handoutMasterIdLst")]
pub struct HandoutMasterIdList {
  ///Handout Master ID
  #[sdk(child(qname = "p:CT_HandoutMasterIdListEntry/p:handoutMasterId"))]
  pub handout_master_id: Option<std::boxed::Box<HandoutMasterId>>,
}
/// Defines the SlideIdList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldIdLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideIdList/p:sldIdLst")]
pub struct SlideIdList {
  /// _
  #[sdk(child(qname = "p:CT_SlideIdListEntry/p:sldId"))]
  pub p_sld_id: Vec<SlideId>,
}
/// Defines the SlideSize Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldSz.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideSize/p:sldSz")]
pub struct SlideSize {
  /// Extent Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "914400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: crate::simple_type::Int32Value,
  /// Extent Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "914400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: crate::simple_type::Int32Value,
  /// Type of Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<SlideSizeValues>,
}
/// Defines the EmbeddedFontList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:embeddedFontLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EmbeddedFontList/p:embeddedFontLst")]
pub struct EmbeddedFontList {
  /// _
  #[sdk(child(qname = "p:CT_EmbeddedFontListEntry/p:embeddedFont"))]
  pub p_embedded_font: Vec<EmbeddedFont>,
}
/// Defines the CustomShowList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:custShowLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CustomShowList/p:custShowLst")]
pub struct CustomShowList {
  /// _
  #[sdk(child(qname = "p:CT_CustomShow/p:custShow"))]
  pub p_cust_show: Vec<CustomShow>,
}
/// Defines the PhotoAlbum Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:photoAlbum.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PhotoAlbum/p:photoAlbum")]
pub struct PhotoAlbum {
  /// Black and White
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bw
  #[sdk(attr(qname = ":bw"))]
  pub black_white: Option<crate::simple_type::BooleanValue>,
  /// Show/Hide Captions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showCaptions
  #[sdk(attr(qname = ":showCaptions"))]
  pub show_captions: Option<crate::simple_type::BooleanValue>,
  /// Photo Album Layout
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :layout
  #[sdk(attr(qname = ":layout"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub layout: Option<PhotoAlbumLayoutValues>,
  /// Frame Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :frame
  #[sdk(attr(qname = ":frame"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub frame: Option<PhotoAlbumFrameShapeValues>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Kinsoku Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:kinsoku.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Kinsoku/p:kinsoku")]
pub struct Kinsoku {
  /// Language
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Invalid Kinsoku Start Characters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :invalStChars
  #[sdk(attr(qname = ":invalStChars"))]
  pub invalid_start_chars: crate::simple_type::StringValue,
  /// Invalid Kinsoku End Characters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :invalEndChars
  #[sdk(attr(qname = ":invalEndChars"))]
  pub invalid_end_chars: crate::simple_type::StringValue,
}
/// Defines the ModificationVerifier Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:modifyVerifier.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ModifyVerifier/p:modifyVerifier")]
pub struct ModificationVerifier {
  /// Cryptographic Provider Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptProviderType
  #[sdk(attr(qname = ":cryptProviderType"))]
  pub cryptographic_provider_type: CryptProviderValues,
  /// Cryptographic Algorithm Class
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptAlgorithmClass
  #[sdk(attr(qname = ":cryptAlgorithmClass"))]
  pub cryptographic_algorithm_class: CryptAlgorithmClassValues,
  /// Cryptographic Algorithm Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptAlgorithmType
  #[sdk(attr(qname = ":cryptAlgorithmType"))]
  pub cryptographic_algorithm_type: CryptAlgorithmValues,
  /// Cryptographic Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptAlgorithmSid
  #[sdk(attr(qname = ":cryptAlgorithmSid"))]
  pub cryptographic_algorithm_sid: crate::simple_type::UInt32Value,
  /// Iterations to Run Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: crate::simple_type::UInt32Value,
  /// Salt for Password Verifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saltData
  #[sdk(attr(qname = ":saltData"))]
  pub salt_data: crate::simple_type::Base64BinaryValue,
  /// Password Hash
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hashData
  #[sdk(attr(qname = ":hashData"))]
  pub hash_data: crate::simple_type::StringValue,
  /// Cryptographic Provider
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptProvider
  #[sdk(attr(qname = ":cryptProvider"))]
  pub cryptographic_provider: Option<crate::simple_type::StringValue>,
  /// Cryptographic Algorithm Extensibility
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algIdExt
  #[sdk(attr(qname = ":algIdExt"))]
  pub extended_cryptographic_algorithm: Option<crate::simple_type::UInt32Value>,
  /// Algorithm Extensibility Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algIdExtSource
  #[sdk(attr(qname = ":algIdExtSource"))]
  pub extended_cryptographic_algorithm_source: Option<crate::simple_type::StringValue>,
  /// Cryptographic Provider Type Extensibility
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptProviderTypeExt
  #[sdk(attr(qname = ":cryptProviderTypeExt"))]
  pub cryptographic_provider_type_extensibility: Option<crate::simple_type::UInt32Value>,
  /// Provider Type Extensibility Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cryptProviderTypeExtSource
  #[sdk(attr(qname = ":cryptProviderTypeExtSource"))]
  pub cryptographic_provider_type_extensibility_source: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// algorithmName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// hashValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  #[cfg(feature = "microsoft365")]
  /// saltValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  #[cfg(feature = "microsoft365")]
  /// spinValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spinValue
  #[sdk(attr(qname = ":spinValue"))]
  pub spin_value: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PresentationExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationExtensionList/p:extLst")]
pub struct PresentationExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_PresentationExtension/p:ext"))]
  pub p_ext: Vec<PresentationExtension>,
}
/// Defines the PresentationExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationExtension/p:ext")]
pub struct PresentationExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "p14:CT_SectionProperties/p14:sectionPr",
      qname = "p14:CT_SectionList/p14:sectionLst",
      qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst",
      qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst"
    ))
  )]
  pub xml_children: Option<PresentationExtensionChoice>,
}
/// HTML Publishing Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:htmlPubPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HtmlPublishProperties/p:htmlPubPr")]
pub struct HtmlPublishProperties {
  /// Show Speaker Notes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showSpeakerNotes
  #[sdk(attr(qname = ":showSpeakerNotes"))]
  pub show_speaker_notes: Option<crate::simple_type::BooleanValue>,
  /// Browser Support Target
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pubBrowser
  #[sdk(attr(qname = ":pubBrowser"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub target_browser: Option<HtmlPublishWebBrowserSupportValues>,
  /// Publish Path
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "p:CT_Empty/p:sldAll",
    qname = "p:CT_IndexRange/p:sldRg",
    qname = "p:CT_CustomShowId/p:custShow"
  ))]
  pub html_publish_properties_choice: Option<HtmlPublishPropertiesChoice>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ExtensionList>,
}
/// Web Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:webPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_WebProperties/p:webPr")]
pub struct WebProperties {
  /// Show animation in HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAnimation
  #[sdk(attr(qname = ":showAnimation"))]
  pub show_animation: Option<crate::simple_type::BooleanValue>,
  /// Resize graphics in HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :resizeGraphics
  #[sdk(attr(qname = ":resizeGraphics"))]
  pub resize_graphics: Option<crate::simple_type::BooleanValue>,
  /// Allow PNG in HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowPng
  #[sdk(attr(qname = ":allowPng"))]
  pub allow_png: Option<crate::simple_type::BooleanValue>,
  /// Rely on VML for HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relyOnVml
  #[sdk(attr(qname = ":relyOnVml"))]
  pub rely_on_vml: Option<crate::simple_type::BooleanValue>,
  /// Organize HTML output in folders
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :organizeInFolders
  #[sdk(attr(qname = ":organizeInFolders"))]
  pub organize_in_folders: Option<crate::simple_type::BooleanValue>,
  /// Use long file names in HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useLongFilenames
  #[sdk(attr(qname = ":useLongFilenames"))]
  pub use_long_filenames: Option<crate::simple_type::BooleanValue>,
  /// Image size for HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imgSz
  #[sdk(attr(qname = ":imgSz"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub image_size: Option<WebScreenSizeValues>,
  /// Encoding for HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :encoding
  #[sdk(attr(qname = ":encoding"))]
  pub encoding: Option<crate::simple_type::StringValue>,
  /// Slide Navigation Colors for HTML output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clr
  #[sdk(attr(qname = ":clr"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub color: Option<WebColorValues>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PrintingProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:prnPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PrintProperties/p:prnPr")]
pub struct PrintingProperties {
  /// Print Output
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prnWhat
  #[sdk(attr(qname = ":prnWhat"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub print_what: Option<PrintOutputValues>,
  /// Print Color Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clrMode
  #[sdk(attr(qname = ":clrMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub color_mode: Option<PrintColorModeValues>,
  /// Print Hidden Slides
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenSlides
  #[sdk(attr(qname = ":hiddenSlides"))]
  pub hidden_slides: Option<crate::simple_type::BooleanValue>,
  /// Scale to Fit Paper when printing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scaleToFitPaper
  #[sdk(attr(qname = ":scaleToFitPaper"))]
  pub scale_to_fit_paper: Option<crate::simple_type::BooleanValue>,
  /// Frame slides when printing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :frameSlides
  #[sdk(attr(qname = ":frameSlides"))]
  pub frame_slides: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ShowProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:showPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ShowProperties/p:showPr")]
pub struct ShowProperties {
  /// Loop Slide Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :loop
  #[sdk(attr(qname = ":loop"))]
  pub r#loop: Option<crate::simple_type::BooleanValue>,
  /// Show Narration in Slide Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showNarration
  #[sdk(attr(qname = ":showNarration"))]
  pub show_narration: Option<crate::simple_type::BooleanValue>,
  /// Show Animation in Slide Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAnimation
  #[sdk(attr(qname = ":showAnimation"))]
  pub show_animation: Option<crate::simple_type::BooleanValue>,
  /// Use Timings in Slide Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useTimings
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
  /// _
  #[sdk(child(qname = "a:CT_Color/p:penClr"))]
  pub p_pen_clr: Option<std::boxed::Box<PenColor>>,
  /// _
  #[sdk(child(qname = "p:CT_ShowPropertiesExtensionList/p:extLst"))]
  pub p_ext_lst: Option<ShowPropertiesExtensionList>,
}
/// Defines the ColorMostRecentlyUsed Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:clrMru.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMRU/p:clrMru")]
pub struct ColorMostRecentlyUsed {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub color_most_recently_used_choice: Vec<ColorMostRecentlyUsedChoice>,
}
/// Defines the PresentationPropertiesExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationPropertiesExtensionList/p:extLst")]
pub struct PresentationPropertiesExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_PresentationPropertiesExtension/p:ext"))]
  pub p_ext: Vec<PresentationPropertiesExtension>,
}
/// Defines the PresentationPropertiesExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PresentationPropertiesExtension/p:ext")]
pub struct PresentationPropertiesExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData",
      qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi",
      qname = "a14:CT_TextMath/a14:m",
      qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased"
    ))
  )]
  pub xml_children: Option<PresentationPropertiesExtensionChoice>,
}
/// Defines the HeaderFooter Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:hf.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HeaderFooter/p:hf")]
pub struct HeaderFooter {
  /// Slide Number Placeholder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sldNum
  #[sdk(attr(qname = ":sldNum"))]
  pub slide_number: Option<crate::simple_type::BooleanValue>,
  /// Header Placeholder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hdr
  #[sdk(attr(qname = ":hdr"))]
  pub header: Option<crate::simple_type::BooleanValue>,
  /// Footer Placeholder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ftr
  #[sdk(attr(qname = ":ftr"))]
  pub footer: Option<crate::simple_type::BooleanValue>,
  /// Date/Time Placeholder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dt
  #[sdk(attr(qname = ":dt"))]
  pub date_time: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideLayoutExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutExtensionList/p:extLst")]
pub struct SlideLayoutExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_SlideLayoutExtension/p:ext"))]
  pub p_ext: Vec<SlideLayoutExtension>,
}
/// Defines the SlideLayoutIdList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sldLayoutIdLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideLayoutIdList/p:sldLayoutIdLst")]
pub struct SlideLayoutIdList {
  /// _
  #[sdk(child(qname = "p:CT_SlideLayoutIdListEntry/p:sldLayoutId"))]
  pub p_sld_layout_id: Vec<SlideLayoutId>,
}
/// Defines the TextStyles Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:txStyles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterTextStyles/p:txStyles")]
pub struct TextStyles {
  ///Slide Master Title Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:titleStyle"))]
  pub title_style: Option<std::boxed::Box<TitleStyle>>,
  ///Slide Master Body Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:bodyStyle"))]
  pub body_style: Option<std::boxed::Box<BodyStyle>>,
  ///Slide Master Other Text Style
  #[sdk(child(qname = "a:CT_TextListStyle/p:otherStyle"))]
  pub other_style: Option<std::boxed::Box<OtherStyle>>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlideMasterExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SlideMasterExtensionList/p:extLst")]
pub struct SlideMasterExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_SlideMasterExtension/p:ext"))]
  pub p_ext: Vec<SlideMasterExtension>,
}
/// Defines the HandoutMasterExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_HandoutMasterExtensionList/p:extLst")]
pub struct HandoutMasterExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_HandoutMasterExtension/p:ext"))]
  pub p_ext: Vec<HandoutMasterExtension>,
}
/// Defines the NotesMasterExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_NotesMasterExtensionList/p:extLst")]
pub struct NotesMasterExtensionList {
  /// _
  #[sdk(child(qname = "p:CT_NotesMasterExtension/p:ext"))]
  pub p_ext: Vec<NotesMasterExtension>,
}
/// OLE Chart Element.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:oleChartEl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLOleChartTargetElement/p:oleChartEl")]
pub struct OleChartElement {
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: ChartSubElementValues,
  /// Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lvl
  #[sdk(attr(qname = ":lvl"))]
  pub level: Option<crate::simple_type::UInt32Value>,
}
/// Text Element.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:txEl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TLTextTargetElement/p:txEl")]
pub struct TextElement {
  #[sdk(choice(qname = "p:CT_IndexRange/p:charRg", qname = "p:CT_IndexRange/p:pRg"))]
  pub xml_children: Option<TextElementChoice>,
}
/// Graphic Element.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:graphicEl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationElementChoice/p:graphicEl")]
pub struct GraphicElement {
  #[sdk(choice(
    qname = "a:CT_AnimationDgmElement/a:dgm",
    qname = "a:CT_AnimationChartElement/a:chart"
  ))]
  pub xml_children: Option<GraphicElementChoice>,
}
/// Defines the BlindsTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:blinds.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:blinds")]
pub struct BlindsTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CheckerTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:checker.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:checker")]
pub struct CheckerTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CombTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:comb.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:comb")]
pub struct CombTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the RandomBarTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:randomBar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/p:randomBar")]
pub struct RandomBarTransition {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the OrientationTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OrientationTransition/")]
pub struct OrientationTransitionType {
  /// Transition Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<DirectionValues>,
}
/// Defines the CoverTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cover.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EightDirectionTransition/p:cover")]
pub struct CoverTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the PullTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:pull.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EightDirectionTransition/p:pull")]
pub struct PullTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the EightDirectionTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_EightDirectionTransition/")]
pub struct EightDirectionTransitionType {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "u", "r", "d"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the CutTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:cut.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OptionalBlackTransition/p:cut")]
pub struct CutTransition {
  /// Transition Through Black
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thruBlk
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the FadeTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:fade.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OptionalBlackTransition/p:fade")]
pub struct FadeTransition {
  /// Transition Through Black
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thruBlk
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the OptionalBlackTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_OptionalBlackTransition/")]
pub struct OptionalBlackTransitionType {
  /// Transition Through Black
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thruBlk
  #[sdk(attr(qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PushTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:push.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p:push")]
pub struct PushTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the WipeTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:wipe.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/p:wipe")]
pub struct WipeTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SideDirectionTransitionType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SideDirectionTransition/")]
pub struct SideDirectionTransitionType {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SplitTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:split.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_SplitTransition/p:split")]
pub struct SplitTransition {
  /// Orientation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub orientation: Option<DirectionValues>,
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the StripsTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:strips.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_CornerDirectionTransition/p:strips")]
pub struct StripsTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionCornerDirectionValues>,
}
/// Defines the WheelTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:wheel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_WheelTransition/p:wheel")]
pub struct WheelTransition {
  /// Spokes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ZoomTransition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:zoom.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_InOutTransition/p:zoom")]
pub struct ZoomTransition {
  /// Direction
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the SoundAction Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:sndAc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_TransitionSoundAction/p:sndAc")]
pub struct SoundAction {
  #[sdk(choice(
    qname = "p:CT_TransitionStartSoundAction/p:stSnd",
    qname = "p:CT_Empty/p:endSnd"
  ))]
  pub xml_children: Option<SoundActionChoice>,
}
#[cfg(feature = "microsoft365")]
/// Defines the PlaceholderExtension Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_PlaceholderExtension/p:ext")]
pub struct PlaceholderExtension {
    /// _
    #[sdk(child(qname = "p232:CT_PlaceholderTypeExtension/p232:phTypeExt"))]
    pub placeholder_type_extension: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2023_02_main::PlaceholderTypeExtension,
        >,
    >,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorMapOverrideChoice {
  #[sdk(child(qname = "a:CT_EmptyElement/a:masterClrMapping"))]
  AMasterClrMapping(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::MasterColorMapping,
    >,
  ),
  #[sdk(child(qname = "a:CT_ColorMapping/a:overrideClrMapping"))]
  AOverrideClrMapping(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::OverrideColorMapping,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundPropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundStyleReferenceChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
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
  #[sdk(child(qname = "p:CT_Empty/p:sldTgt"))]
  PSldTgt(std::boxed::Box<SlideTarget>),
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/p:sndTgt"))]
  PSndTgt(std::boxed::Box<SoundTarget>),
  #[sdk(child(qname = "p:CT_TLShapeTargetElement/p:spTgt"))]
  PSpTgt(std::boxed::Box<ShapeTarget>),
  #[sdk(child(qname = "p:CT_TLSubShapeId/p:inkTgt"))]
  PInkTgt(std::boxed::Box<InkTarget>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt"))]
  P14BmkTgt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BookmarkTarget,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ConditionChoice {
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  PTgtEl(std::boxed::Box<TargetElement>),
  #[sdk(child(qname = "p:CT_TLTriggerTimeNodeID/p:tn"))]
  PTn(std::boxed::Box<TimeNode>),
  #[sdk(child(qname = "p:CT_TLTriggerRuntimeNode/p:rtn"))]
  PRtn(std::boxed::Box<RuntimeNodeTrigger>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndSyncChoice {
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  PTgtEl(std::boxed::Box<TargetElement>),
  #[sdk(child(qname = "p:CT_TLTriggerTimeNodeID/p:tn"))]
  PTn(std::boxed::Box<TimeNode>),
  #[sdk(child(qname = "p:CT_TLTriggerRuntimeNode/p:rtn"))]
  PRtn(std::boxed::Box<RuntimeNodeTrigger>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TimeListConditionalTypeChoice {
  #[sdk(child(qname = "p:CT_TLTimeTargetElement/p:tgtEl"))]
  PTgtEl(std::boxed::Box<TargetElement>),
  #[sdk(child(qname = "p:CT_TLTriggerTimeNodeID/p:tn"))]
  PTn(std::boxed::Box<TimeNode>),
  #[sdk(child(qname = "p:CT_TLTriggerRuntimeNode/p:rtn"))]
  PRtn(std::boxed::Box<RuntimeNodeTrigger>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorValueChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PenColorChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorTypeChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ToVariantValueChoice {
  #[sdk(child(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal"))]
  PBoolVal(std::boxed::Box<BooleanVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal"))]
  PIntVal(std::boxed::Box<IntegerVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  PFltVal(std::boxed::Box<FloatVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantStringVal/p:strVal"))]
  PStrVal(std::boxed::Box<StringVariantValue>),
  #[sdk(child(qname = "a:CT_Color/p:clrVal"))]
  PClrVal(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VariantValueChoice {
  #[sdk(child(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal"))]
  PBoolVal(std::boxed::Box<BooleanVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal"))]
  PIntVal(std::boxed::Box<IntegerVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  PFltVal(std::boxed::Box<FloatVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantStringVal/p:strVal"))]
  PStrVal(std::boxed::Box<StringVariantValue>),
  #[sdk(child(qname = "a:CT_Color/p:clrVal"))]
  PClrVal(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TimeListAnimationVariantTypeChoice {
  #[sdk(child(qname = "p:CT_TLAnimVariantBooleanVal/p:boolVal"))]
  PBoolVal(std::boxed::Box<BooleanVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantIntegerVal/p:intVal"))]
  PIntVal(std::boxed::Box<IntegerVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantFloatVal/p:fltVal"))]
  PFltVal(std::boxed::Box<FloatVariantValue>),
  #[sdk(child(qname = "p:CT_TLAnimVariantStringVal/p:strVal"))]
  PStrVal(std::boxed::Box<StringVariantValue>),
  #[sdk(child(qname = "a:CT_Color/p:clrVal"))]
  PClrVal(std::boxed::Box<ColorValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildSubElementChoice {
  #[sdk(child(qname = "a:CT_AnimationDgmBuildProperties/a:bldDgm"))]
  ABldDgm(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BuildDiagram>,
  ),
  #[sdk(child(qname = "a:CT_AnimationChartBuildProperties/a:bldChart"))]
  ABldChart(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BuildChart>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildGraphicsChoice {
  #[sdk(child(qname = "p:CT_Empty/p:bldAsOne"))]
  PBldAsOne(std::boxed::Box<BuildAsOne>),
  #[sdk(child(qname = "a:CT_AnimationGraphicalObjectBuildProperties/p:bldSub"))]
  PBldSub(std::boxed::Box<BuildSubElement>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BuildListChoice {
  #[sdk(child(qname = "p:CT_TLBuildParagraph/p:bldP"))]
  PBldP(std::boxed::Box<BuildParagraph>),
  #[sdk(child(qname = "p:CT_TLBuildDiagram/p:bldDgm"))]
  PBldDgm(std::boxed::Box<BuildDiagram>),
  #[sdk(child(qname = "p:CT_TLOleBuildChart/p:bldOleChart"))]
  PBldOleChart(std::boxed::Box<BuildOleChart>),
  #[sdk(child(qname = "p:CT_TLGraphicalObjectBuild/p:bldGraphic"))]
  PBldGraphic(std::boxed::Box<BuildGraphics>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ByColorChoice {
  #[sdk(child(qname = "p:CT_TLByRgbColorTransform/p:rgb"))]
  PRgb(std::boxed::Box<RgbColor>),
  #[sdk(child(qname = "p:CT_TLByHslColorTransform/p:hsl"))]
  PHsl(std::boxed::Box<HslColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FromColorChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ToColorChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Color3TypeChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ApplicationNonVisualDrawingPropertiesChoice {
  #[sdk(child(qname = "a:CT_AudioCD/a:audioCd"))]
  AAudioCd(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromCd>,
  ),
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile"))]
  AWavAudioFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::WaveAudioFile>,
  ),
  #[sdk(child(qname = "a:CT_AudioFile/a:audioFile"))]
  AAudioFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromFile>,
  ),
  #[sdk(child(qname = "a:CT_VideoFile/a:videoFile"))]
  AVideoFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::VideoFromFile>,
  ),
  #[sdk(child(qname = "a:CT_QuickTimeFile/a:quickTimeFile"))]
  AQuickTimeFile(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::QuickTimeFromFile,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextListStyleTypeChoice {
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  ADefPPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  ALvl1pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  ALvl2pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  ALvl3pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  ALvl4pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  ALvl5pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  ALvl6pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  ALvl7pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  ALvl8pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  ALvl9pPr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
  ),
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  AExtLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LaserTraceList/p14:laserTraceLst"))]
  P14LaserTraceLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserTraceList,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_ShowEventRecordList/p14:showEvtLst"))]
  P14ShowEvtLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowEventRecordList,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p188:CT_CommentRelationship/p188:commentRel"))]
  P188CommentRel(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentRelationship,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommonSlideDataExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_RandomId/p14:creationId"))]
  P14CreationId(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::CreationId>,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_BrowseMode/p14:browseMode"))]
  P14BrowseMode(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BrowseMode>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "a:CT_Color/p14:laserClr"))]
  P14LaserClr(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserColor>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls"))]
  P14ShowMediaCtrls(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowMediaControls,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TransitionChoice {
  #[sdk(child(qname = "p:CT_OrientationTransition/p:blinds"))]
  PBlinds(std::boxed::Box<BlindsTransition>),
  #[sdk(child(qname = "p:CT_OrientationTransition/p:checker"))]
  PChecker(std::boxed::Box<CheckerTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:circle"))]
  PCircle(std::boxed::Box<CircleTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:dissolve"))]
  PDissolve(std::boxed::Box<DissolveTransition>),
  #[sdk(child(qname = "p:CT_OrientationTransition/p:comb"))]
  PComb(std::boxed::Box<CombTransition>),
  #[sdk(child(qname = "p:CT_EightDirectionTransition/p:cover"))]
  PCover(std::boxed::Box<CoverTransition>),
  #[sdk(child(qname = "p:CT_OptionalBlackTransition/p:cut"))]
  PCut(std::boxed::Box<CutTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:diamond"))]
  PDiamond(std::boxed::Box<DiamondTransition>),
  #[sdk(child(qname = "p:CT_OptionalBlackTransition/p:fade"))]
  PFade(std::boxed::Box<FadeTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:newsflash"))]
  PNewsflash(std::boxed::Box<NewsflashTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:plus"))]
  PPlus(std::boxed::Box<PlusTransition>),
  #[sdk(child(qname = "p:CT_EightDirectionTransition/p:pull"))]
  PPull(std::boxed::Box<PullTransition>),
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p:push"))]
  PPush(std::boxed::Box<PushTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:random"))]
  PRandom(std::boxed::Box<RandomTransition>),
  #[sdk(child(qname = "p:CT_OrientationTransition/p:randomBar"))]
  PRandomBar(std::boxed::Box<RandomBarTransition>),
  #[sdk(child(qname = "p:CT_SplitTransition/p:split"))]
  PSplit(std::boxed::Box<SplitTransition>),
  #[sdk(child(qname = "p:CT_CornerDirectionTransition/p:strips"))]
  PStrips(std::boxed::Box<StripsTransition>),
  #[sdk(child(qname = "p:CT_Empty/p:wedge"))]
  PWedge(std::boxed::Box<WedgeTransition>),
  #[sdk(child(qname = "p:CT_WheelTransition/p:wheel"))]
  PWheel(std::boxed::Box<WheelTransition>),
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p:wipe"))]
  PWipe(std::boxed::Box<WipeTransition>),
  #[sdk(child(qname = "p:CT_InOutTransition/p:zoom"))]
  PZoom(std::boxed::Box<ZoomTransition>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_Empty/p14:flash"))]
  P14Flash(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlashTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p14:vortex"))]
  P14Vortex(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::VortexTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LeftRightDirectionTransition/p14:switch"))]
  P14Switch(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SwitchTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LeftRightDirectionTransition/p14:flip"))]
  P14Flip(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlipTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_RippleTransition/p14:ripple"))]
  P14Ripple(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::RippleTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_GlitterTransition/p14:glitter"))]
  P14Glitter(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::GlitterTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_Empty/p14:honeycomb"))]
  P14Honeycomb(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::HoneycombTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_PrismTransition/p14:prism"))]
  P14Prism(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::PrismTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_OrientationTransition/p14:doors"))]
  P14Doors(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DoorsTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_OrientationTransition/p14:window"))]
  P14Window(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WindowTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_ShredTransition/p14:shred"))]
  P14Shred(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShredTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LeftRightDirectionTransition/p14:ferris"))]
  P14Ferris(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FerrisTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_FlyThroughTransition/p14:flythrough"))]
  P14Flythrough(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlythroughTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_InOutTransition/p14:warp"))]
  P14Warp(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WarpTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LeftRightDirectionTransition/p14:gallery"))]
  P14Gallery(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::GalleryTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_LeftRightDirectionTransition/p14:conveyor"))]
  P14Conveyor(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ConveyorTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_SideDirectionTransition/p14:pan"))]
  P14Pan(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::PanTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_RevealTransition/p14:reveal"))]
  P14Reveal(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::RevealTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_WheelTransition/p14:wheelReverse"))]
  P14WheelReverse(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WheelReverseTransition,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_PresetTransition/p15:prstTrans"))]
  P15PrstTrans(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresetTransition,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundChoice {
  #[sdk(child(qname = "p:CT_BackgroundProperties/p:bgPr"))]
  PBgPr(std::boxed::Box<BackgroundProperties>),
  #[sdk(child(qname = "a:CT_StyleMatrixReference/p:bgRef"))]
  PBgRef(std::boxed::Box<BackgroundStyleReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeTreeChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_ContentPart/p:contentPart"))]
  PContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  McAlternateContent(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
    >,
  ),
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_ContentPart/p:contentPart"))]
  PContentPart(std::boxed::Box<ContentPart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeTypeChoice {
  #[sdk(child(qname = "p:CT_GroupShapeNonVisual/p:nvGrpSpPr"))]
  PNvGrpSpPr(std::boxed::Box<NonVisualGroupShapeProperties>),
  #[sdk(child(qname = "a:CT_GroupShapeProperties/p:grpSpPr"))]
  PGrpSpPr(std::boxed::Box<GroupShapeProperties>),
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
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p:CT_ContentPart/p:contentPart"))]
  PContentPart(std::boxed::Box<ContentPart>),
  #[sdk(child(qname = "p:CT_ExtensionListModify/p:extLst"))]
  PExtLst(std::boxed::Box<ExtensionListWithModification>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapeTargetChoice {
  #[sdk(child(qname = "p:CT_Empty/p:bg"))]
  PBg(std::boxed::Box<BackgroundAnimation>),
  #[sdk(child(qname = "p:CT_TLSubShapeId/p:subSp"))]
  PSubSp(std::boxed::Box<SubShape>),
  #[sdk(child(qname = "p:CT_TLOleChartTargetElement/p:oleChartEl"))]
  POleChartEl(std::boxed::Box<OleChartElement>),
  #[sdk(child(qname = "p:CT_TLTextTargetElement/p:txEl"))]
  PTxEl(std::boxed::Box<TextElement>),
  #[sdk(child(qname = "a:CT_AnimationElementChoice/p:graphicEl"))]
  PGraphicEl(std::boxed::Box<GraphicElement>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentAuthorExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_PresenceInfo/p15:presenceInfo"))]
  P15PresenceInfo(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresenceInfo,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CommentExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_CommentThreading/p15:threadingInfo"))]
  P15ThreadingInfo(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ThreadingInfo,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideLayoutExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlideMasterExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HandoutMasterExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NotesMasterExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ApplicationNonVisualDrawingPropertiesExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_Media/p14:media"))]
  P14Media(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Media>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_RandomId/p14:modId"))]
  P14ModId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ModificationId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum IterateChoice {
  #[sdk(child(qname = "p:CT_TLIterateIntervalTime/p:tmAbs"))]
  PTmAbs(std::boxed::Box<TimeAbsolute>),
  #[sdk(child(qname = "p:CT_TLIterateIntervalPercentage/p:tmPct"))]
  PTmPct(std::boxed::Box<TimePercentage>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChildTimeNodeListChoice {
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  PPar(std::boxed::Box<ParallelTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeSequence/p:seq"))]
  PSeq(std::boxed::Box<SequenceTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeExclusive/p:excl"))]
  PExcl(std::boxed::Box<ExclusiveTimeNode>),
  #[sdk(child(qname = "p:CT_TLAnimateBehavior/p:anim"))]
  PAnim(std::boxed::Box<Animate>),
  #[sdk(child(qname = "p:CT_TLAnimateColorBehavior/p:animClr"))]
  PAnimClr(std::boxed::Box<AnimateColor>),
  #[sdk(child(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect"))]
  PAnimEffect(std::boxed::Box<AnimateEffect>),
  #[sdk(child(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion"))]
  PAnimMotion(std::boxed::Box<AnimateMotion>),
  #[sdk(child(qname = "p:CT_TLAnimateRotationBehavior/p:animRot"))]
  PAnimRot(std::boxed::Box<AnimateRotation>),
  #[sdk(child(qname = "p:CT_TLAnimateScaleBehavior/p:animScale"))]
  PAnimScale(std::boxed::Box<AnimateScale>),
  #[sdk(child(qname = "p:CT_TLCommandBehavior/p:cmd"))]
  PCmd(std::boxed::Box<Command>),
  #[sdk(child(qname = "p:CT_TLSetBehavior/p:set"))]
  PSet(std::boxed::Box<SetBehavior>),
  #[sdk(child(qname = "p:CT_TLMediaNodeAudio/p:audio"))]
  PAudio(std::boxed::Box<Audio>),
  #[sdk(child(qname = "p:CT_TLMediaNodeVideo/p:video"))]
  PVideo(std::boxed::Box<Video>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SubTimeNodeListChoice {
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  PPar(std::boxed::Box<ParallelTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeSequence/p:seq"))]
  PSeq(std::boxed::Box<SequenceTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeExclusive/p:excl"))]
  PExcl(std::boxed::Box<ExclusiveTimeNode>),
  #[sdk(child(qname = "p:CT_TLAnimateBehavior/p:anim"))]
  PAnim(std::boxed::Box<Animate>),
  #[sdk(child(qname = "p:CT_TLAnimateColorBehavior/p:animClr"))]
  PAnimClr(std::boxed::Box<AnimateColor>),
  #[sdk(child(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect"))]
  PAnimEffect(std::boxed::Box<AnimateEffect>),
  #[sdk(child(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion"))]
  PAnimMotion(std::boxed::Box<AnimateMotion>),
  #[sdk(child(qname = "p:CT_TLAnimateRotationBehavior/p:animRot"))]
  PAnimRot(std::boxed::Box<AnimateRotation>),
  #[sdk(child(qname = "p:CT_TLAnimateScaleBehavior/p:animScale"))]
  PAnimScale(std::boxed::Box<AnimateScale>),
  #[sdk(child(qname = "p:CT_TLCommandBehavior/p:cmd"))]
  PCmd(std::boxed::Box<Command>),
  #[sdk(child(qname = "p:CT_TLSetBehavior/p:set"))]
  PSet(std::boxed::Box<SetBehavior>),
  #[sdk(child(qname = "p:CT_TLMediaNodeAudio/p:audio"))]
  PAudio(std::boxed::Box<Audio>),
  #[sdk(child(qname = "p:CT_TLMediaNodeVideo/p:video"))]
  PVideo(std::boxed::Box<Video>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TimeTypeListTypeChoice {
  #[sdk(child(qname = "p:CT_TLTimeNodeParallel/p:par"))]
  PPar(std::boxed::Box<ParallelTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeSequence/p:seq"))]
  PSeq(std::boxed::Box<SequenceTimeNode>),
  #[sdk(child(qname = "p:CT_TLTimeNodeExclusive/p:excl"))]
  PExcl(std::boxed::Box<ExclusiveTimeNode>),
  #[sdk(child(qname = "p:CT_TLAnimateBehavior/p:anim"))]
  PAnim(std::boxed::Box<Animate>),
  #[sdk(child(qname = "p:CT_TLAnimateColorBehavior/p:animClr"))]
  PAnimClr(std::boxed::Box<AnimateColor>),
  #[sdk(child(qname = "p:CT_TLAnimateEffectBehavior/p:animEffect"))]
  PAnimEffect(std::boxed::Box<AnimateEffect>),
  #[sdk(child(qname = "p:CT_TLAnimateMotionBehavior/p:animMotion"))]
  PAnimMotion(std::boxed::Box<AnimateMotion>),
  #[sdk(child(qname = "p:CT_TLAnimateRotationBehavior/p:animRot"))]
  PAnimRot(std::boxed::Box<AnimateRotation>),
  #[sdk(child(qname = "p:CT_TLAnimateScaleBehavior/p:animScale"))]
  PAnimScale(std::boxed::Box<AnimateScale>),
  #[sdk(child(qname = "p:CT_TLCommandBehavior/p:cmd"))]
  PCmd(std::boxed::Box<Command>),
  #[sdk(child(qname = "p:CT_TLSetBehavior/p:set"))]
  PSet(std::boxed::Box<SetBehavior>),
  #[sdk(child(qname = "p:CT_TLMediaNodeAudio/p:audio"))]
  PAudio(std::boxed::Box<Audio>),
  #[sdk(child(qname = "p:CT_TLMediaNodeVideo/p:video"))]
  PVideo(std::boxed::Box<Video>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresentationExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_SectionProperties/p14:sectionPr"))]
  P14SectionPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionProperties,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p14:CT_SectionList/p14:sectionLst"))]
  P14SectionLst(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionList>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst"))]
  P15SldGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst"))]
  P15NotesGuideLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::NotesGuideList,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HtmlPublishPropertiesChoice {
  #[sdk(child(qname = "p:CT_Empty/p:sldAll"))]
  PSldAll(std::boxed::Box<SlideAll>),
  #[sdk(child(qname = "p:CT_IndexRange/p:sldRg"))]
  PSldRg(std::boxed::Box<SlideRange>),
  #[sdk(child(qname = "p:CT_CustomShowId/p:custShow"))]
  PCustShow(std::boxed::Box<CustomShowReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesChoice {
  #[sdk(child(qname = "p:CT_Empty/p:present"))]
  PPresent(std::boxed::Box<PresenterSlideMode>),
  #[sdk(child(qname = "p:CT_ShowInfoBrowse/p:browse"))]
  PBrowse(std::boxed::Box<BrowseSlideMode>),
  #[sdk(child(qname = "p:CT_ShowInfoKiosk/p:kiosk"))]
  PKiosk(std::boxed::Box<KioskSlideMode>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowPropertiesChoice2 {
  #[sdk(child(qname = "p:CT_Empty/p:sldAll"))]
  PSldAll(std::boxed::Box<SlideAll>),
  #[sdk(child(qname = "p:CT_IndexRange/p:sldRg"))]
  PSldRg(std::boxed::Box<SlideRange>),
  #[sdk(child(qname = "p:CT_CustomShowId/p:custShow"))]
  PCustShow(std::boxed::Box<CustomShowReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorMostRecentlyUsedChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresentationPropertiesExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData"))]
    P14DiscardImageEditData(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DiscardImageEditData,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi"))]
    P14DefaultImageDpi(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DefaultImageDpi,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "a14:CT_TextMath/a14:m"))]
    A14M(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2010_main::TextMath,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased"))]
    P15ChartTrackingRefBased(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ChartTrackingReferenceBased,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextElementChoice {
  #[sdk(child(qname = "p:CT_IndexRange/p:charRg"))]
  PCharRg(std::boxed::Box<CharRange>),
  #[sdk(child(qname = "p:CT_IndexRange/p:pRg"))]
  PPRg(std::boxed::Box<ParagraphIndexRange>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GraphicElementChoice {
  #[sdk(child(qname = "a:CT_AnimationDgmElement/a:dgm"))]
  ADgm(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Diagram>),
  #[sdk(child(qname = "a:CT_AnimationChartElement/a:chart"))]
  AChart(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Chart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SoundActionChoice {
  #[sdk(child(qname = "p:CT_TransitionStartSoundAction/p:stSnd"))]
  PStSnd(std::boxed::Box<StartSoundAction>),
  #[sdk(child(qname = "p:CT_Empty/p:endSnd"))]
  PEndSnd(std::boxed::Box<EndSoundAction>),
}
