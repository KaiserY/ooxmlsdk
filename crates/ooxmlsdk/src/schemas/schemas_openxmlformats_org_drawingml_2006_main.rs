//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontCollectionIndexValues {
  #[sdk(rename = "major")]
  #[default]
  Major,
  #[sdk(rename = "minor")]
  Minor,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ColorSchemeIndexValues {
  #[sdk(rename = "dk1")]
  #[default]
  Dark1,
  #[sdk(rename = "lt1")]
  Light1,
  #[sdk(rename = "dk2")]
  Dark2,
  #[sdk(rename = "lt2")]
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
  #[sdk(rename = "hlink")]
  Hyperlink,
  #[sdk(rename = "folHlink")]
  FollowedHyperlink,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SystemColorValues {
  #[sdk(rename = "scrollBar")]
  #[default]
  ScrollBar,
  #[sdk(rename = "background")]
  Background,
  #[sdk(rename = "activeCaption")]
  ActiveCaption,
  #[sdk(rename = "inactiveCaption")]
  InactiveCaption,
  #[sdk(rename = "menu")]
  Menu,
  #[sdk(rename = "window")]
  Window,
  #[sdk(rename = "windowFrame")]
  WindowFrame,
  #[sdk(rename = "menuText")]
  MenuText,
  #[sdk(rename = "windowText")]
  WindowText,
  #[sdk(rename = "captionText")]
  CaptionText,
  #[sdk(rename = "activeBorder")]
  ActiveBorder,
  #[sdk(rename = "inactiveBorder")]
  InactiveBorder,
  #[sdk(rename = "appWorkspace")]
  ApplicationWorkspace,
  #[sdk(rename = "highlight")]
  Highlight,
  #[sdk(rename = "highlightText")]
  HighlightText,
  #[sdk(rename = "btnFace")]
  ButtonFace,
  #[sdk(rename = "btnShadow")]
  ButtonShadow,
  #[sdk(rename = "grayText")]
  GrayText,
  #[sdk(rename = "btnText")]
  ButtonText,
  #[sdk(rename = "inactiveCaptionText")]
  InactiveCaptionText,
  #[sdk(rename = "btnHighlight")]
  ButtonHighlight,
  #[sdk(rename = "3dDkShadow")]
  ThreeDDarkShadow,
  #[sdk(rename = "3dLight")]
  ThreeDLight,
  #[sdk(rename = "infoText")]
  InfoText,
  #[sdk(rename = "infoBk")]
  InfoBack,
  #[sdk(rename = "hotLight")]
  HotLight,
  #[sdk(rename = "gradientActiveCaption")]
  GradientActiveCaption,
  #[sdk(rename = "gradientInactiveCaption")]
  GradientInactiveCaption,
  #[sdk(rename = "menuHighlight")]
  MenuHighlight,
  #[sdk(rename = "menuBar")]
  MenuBar,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SchemeColorValues {
  #[sdk(rename = "bg1")]
  #[default]
  Background1,
  #[sdk(rename = "tx1")]
  Text1,
  #[sdk(rename = "bg2")]
  Background2,
  #[sdk(rename = "tx2")]
  Text2,
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
  #[sdk(rename = "hlink")]
  Hyperlink,
  #[sdk(rename = "folHlink")]
  FollowedHyperlink,
  #[sdk(rename = "phClr")]
  PhColor,
  #[sdk(rename = "dk1")]
  Dark1,
  #[sdk(rename = "lt1")]
  Light1,
  #[sdk(rename = "dk2")]
  Dark2,
  #[sdk(rename = "lt2")]
  Light2,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RectangleAlignmentValues {
  #[sdk(rename = "tl")]
  #[default]
  TopLeft,
  #[sdk(rename = "t")]
  Top,
  #[sdk(rename = "tr")]
  TopRight,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "bl")]
  BottomLeft,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "br")]
  BottomRight,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BlackWhiteModeValues {
  #[sdk(rename = "clr")]
  #[default]
  Color,
  #[sdk(rename = "auto")]
  Auto,
  #[sdk(rename = "gray")]
  Gray,
  #[sdk(rename = "ltGray")]
  LightGray,
  #[sdk(rename = "invGray")]
  InvGray,
  #[sdk(rename = "grayWhite")]
  GrayWhite,
  #[sdk(rename = "blackGray")]
  BlackGray,
  #[sdk(rename = "blackWhite")]
  BlackWhite,
  #[sdk(rename = "black")]
  Black,
  #[sdk(rename = "white")]
  White,
  #[sdk(rename = "hidden")]
  Hidden,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChartBuildStepValues {
  #[sdk(rename = "category")]
  #[default]
  Category,
  #[sdk(rename = "ptInCategory")]
  CategoryPoints,
  #[sdk(rename = "series")]
  Series,
  #[sdk(rename = "ptInSeries")]
  SeriesPoints,
  #[sdk(rename = "allPts")]
  AllPoints,
  #[sdk(rename = "gridLegend")]
  GridLegend,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DiagramBuildStepValues {
  #[sdk(rename = "sp")]
  #[default]
  Shape,
  #[sdk(rename = "bg")]
  Background,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimationBuildValues {
  #[sdk(rename = "allAtOnce")]
  #[default]
  AllAtOnce,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimationDiagramOnlyBuildValues {
  #[sdk(rename = "one")]
  #[default]
  One,
  #[sdk(rename = "lvlOne")]
  LevelOne,
  #[sdk(rename = "lvlAtOnce")]
  LevelAtOnce,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimationChartOnlyBuildValues {
  #[sdk(rename = "series")]
  #[default]
  Series,
  #[sdk(rename = "category")]
  Category,
  #[sdk(rename = "seriesEl")]
  SeriesElement,
  #[sdk(rename = "categoryEl")]
  CategoryElement,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetCameraValues {
  #[sdk(rename = "legacyObliqueTopLeft")]
  #[default]
  LegacyObliqueTopLeft,
  #[sdk(rename = "legacyObliqueTop")]
  LegacyObliqueTop,
  #[sdk(rename = "legacyObliqueTopRight")]
  LegacyObliqueTopRight,
  #[sdk(rename = "legacyObliqueLeft")]
  LegacyObliqueLeft,
  #[sdk(rename = "legacyObliqueFront")]
  LegacyObliqueFront,
  #[sdk(rename = "legacyObliqueRight")]
  LegacyObliqueRight,
  #[sdk(rename = "legacyObliqueBottomLeft")]
  LegacyObliqueBottomLeft,
  #[sdk(rename = "legacyObliqueBottom")]
  LegacyObliqueBottom,
  #[sdk(rename = "legacyObliqueBottomRight")]
  LegacyObliqueBottomRight,
  #[sdk(rename = "legacyPerspectiveTopLeft")]
  LegacyPerspectiveTopLeft,
  #[sdk(rename = "legacyPerspectiveTop")]
  LegacyPerspectiveTop,
  #[sdk(rename = "legacyPerspectiveTopRight")]
  LegacyPerspectiveTopRight,
  #[sdk(rename = "legacyPerspectiveLeft")]
  LegacyPerspectiveLeft,
  #[sdk(rename = "legacyPerspectiveFront")]
  LegacyPerspectiveFront,
  #[sdk(rename = "legacyPerspectiveRight")]
  LegacyPerspectiveRight,
  #[sdk(rename = "legacyPerspectiveBottomLeft")]
  LegacyPerspectiveBottomLeft,
  #[sdk(rename = "legacyPerspectiveBottom")]
  LegacyPerspectiveBottom,
  #[sdk(rename = "legacyPerspectiveBottomRight")]
  LegacyPerspectiveBottomRight,
  #[sdk(rename = "orthographicFront")]
  OrthographicFront,
  #[sdk(rename = "isometricTopUp")]
  IsometricTopUp,
  #[sdk(rename = "isometricTopDown")]
  IsometricTopDown,
  #[sdk(rename = "isometricBottomUp")]
  IsometricBottomUp,
  #[sdk(rename = "isometricBottomDown")]
  IsometricBottomDown,
  #[sdk(rename = "isometricLeftUp")]
  IsometricLeftUp,
  #[sdk(rename = "isometricLeftDown")]
  IsometricLeftDown,
  #[sdk(rename = "isometricRightUp")]
  IsometricRightUp,
  #[sdk(rename = "isometricRightDown")]
  IsometricRightDown,
  #[sdk(rename = "isometricOffAxis1Left")]
  IsometricOffAxis1Left,
  #[sdk(rename = "isometricOffAxis1Right")]
  IsometricOffAxis1Right,
  #[sdk(rename = "isometricOffAxis1Top")]
  IsometricOffAxis1Top,
  #[sdk(rename = "isometricOffAxis2Left")]
  IsometricOffAxis2Left,
  #[sdk(rename = "isometricOffAxis2Right")]
  IsometricOffAxis2Right,
  #[sdk(rename = "isometricOffAxis2Top")]
  IsometricOffAxis2Top,
  #[sdk(rename = "isometricOffAxis3Left")]
  IsometricOffAxis3Left,
  #[sdk(rename = "isometricOffAxis3Right")]
  IsometricOffAxis3Right,
  #[sdk(rename = "isometricOffAxis3Bottom")]
  IsometricOffAxis3Bottom,
  #[sdk(rename = "isometricOffAxis4Left")]
  IsometricOffAxis4Left,
  #[sdk(rename = "isometricOffAxis4Right")]
  IsometricOffAxis4Right,
  #[sdk(rename = "isometricOffAxis4Bottom")]
  IsometricOffAxis4Bottom,
  #[sdk(rename = "obliqueTopLeft")]
  ObliqueTopLeft,
  #[sdk(rename = "obliqueTop")]
  ObliqueTop,
  #[sdk(rename = "obliqueTopRight")]
  ObliqueTopRight,
  #[sdk(rename = "obliqueLeft")]
  ObliqueLeft,
  #[sdk(rename = "obliqueRight")]
  ObliqueRight,
  #[sdk(rename = "obliqueBottomLeft")]
  ObliqueBottomLeft,
  #[sdk(rename = "obliqueBottom")]
  ObliqueBottom,
  #[sdk(rename = "obliqueBottomRight")]
  ObliqueBottomRight,
  #[sdk(rename = "perspectiveFront")]
  PerspectiveFront,
  #[sdk(rename = "perspectiveLeft")]
  PerspectiveLeft,
  #[sdk(rename = "perspectiveRight")]
  PerspectiveRight,
  #[sdk(rename = "perspectiveAbove")]
  PerspectiveAbove,
  #[sdk(rename = "perspectiveBelow")]
  PerspectiveBelow,
  #[sdk(rename = "perspectiveAboveLeftFacing")]
  PerspectiveAboveLeftFacing,
  #[sdk(rename = "perspectiveAboveRightFacing")]
  PerspectiveAboveRightFacing,
  #[sdk(rename = "perspectiveContrastingLeftFacing")]
  PerspectiveContrastingLeftFacing,
  #[sdk(rename = "perspectiveContrastingRightFacing")]
  PerspectiveContrastingRightFacing,
  #[sdk(rename = "perspectiveHeroicLeftFacing")]
  PerspectiveHeroicLeftFacing,
  #[sdk(rename = "perspectiveHeroicRightFacing")]
  PerspectiveHeroicRightFacing,
  #[sdk(rename = "perspectiveHeroicExtremeLeftFacing")]
  PerspectiveHeroicExtremeLeftFacing,
  #[sdk(rename = "perspectiveHeroicExtremeRightFacing")]
  PerspectiveHeroicExtremeRightFacing,
  #[sdk(rename = "perspectiveRelaxed")]
  PerspectiveRelaxed,
  #[sdk(rename = "perspectiveRelaxedModerately")]
  PerspectiveRelaxedModerately,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LightRigDirectionValues {
  #[sdk(rename = "tl")]
  #[default]
  TopLeft,
  #[sdk(rename = "t")]
  Top,
  #[sdk(rename = "tr")]
  TopRight,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "bl")]
  BottomLeft,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "br")]
  BottomRight,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LightRigValues {
  #[sdk(rename = "legacyFlat1")]
  #[default]
  LegacyFlat1,
  #[sdk(rename = "legacyFlat2")]
  LegacyFlat2,
  #[sdk(rename = "legacyFlat3")]
  LegacyFlat3,
  #[sdk(rename = "legacyFlat4")]
  LegacyFlat4,
  #[sdk(rename = "legacyNormal1")]
  LegacyNormal1,
  #[sdk(rename = "legacyNormal2")]
  LegacyNormal2,
  #[sdk(rename = "legacyNormal3")]
  LegacyNormal3,
  #[sdk(rename = "legacyNormal4")]
  LegacyNormal4,
  #[sdk(rename = "legacyHarsh1")]
  LegacyHarsh1,
  #[sdk(rename = "legacyHarsh2")]
  LegacyHarsh2,
  #[sdk(rename = "legacyHarsh3")]
  LegacyHarsh3,
  #[sdk(rename = "legacyHarsh4")]
  LegacyHarsh4,
  #[sdk(rename = "threePt")]
  ThreePoints,
  #[sdk(rename = "balanced")]
  Balanced,
  #[sdk(rename = "soft")]
  Soft,
  #[sdk(rename = "harsh")]
  Harsh,
  #[sdk(rename = "flood")]
  Flood,
  #[sdk(rename = "contrasting")]
  Contrasting,
  #[sdk(rename = "morning")]
  Morning,
  #[sdk(rename = "sunrise")]
  Sunrise,
  #[sdk(rename = "sunset")]
  Sunset,
  #[sdk(rename = "chilly")]
  Chilly,
  #[sdk(rename = "freezing")]
  Freezing,
  #[sdk(rename = "flat")]
  Flat,
  #[sdk(rename = "twoPt")]
  TwoPoints,
  #[sdk(rename = "glow")]
  Glow,
  #[sdk(rename = "brightRoom")]
  BrightRoom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BevelPresetValues {
  #[sdk(rename = "relaxedInset")]
  #[default]
  RelaxedInset,
  #[sdk(rename = "circle")]
  Circle,
  #[sdk(rename = "slope")]
  Slope,
  #[sdk(rename = "cross")]
  Cross,
  #[sdk(rename = "angle")]
  Angle,
  #[sdk(rename = "softRound")]
  SoftRound,
  #[sdk(rename = "convex")]
  Convex,
  #[sdk(rename = "coolSlant")]
  CoolSlant,
  #[sdk(rename = "divot")]
  Divot,
  #[sdk(rename = "riblet")]
  Riblet,
  #[sdk(rename = "hardEdge")]
  HardEdge,
  #[sdk(rename = "artDeco")]
  ArtDeco,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetMaterialTypeValues {
  #[sdk(rename = "legacyMatte")]
  #[default]
  LegacyMatte,
  #[sdk(rename = "legacyPlastic")]
  LegacyPlastic,
  #[sdk(rename = "legacyMetal")]
  LegacyMetal,
  #[sdk(rename = "legacyWireframe")]
  LegacyWireframe,
  #[sdk(rename = "matte")]
  Matte,
  #[sdk(rename = "plastic")]
  Plastic,
  #[sdk(rename = "metal")]
  Metal,
  #[sdk(rename = "warmMatte")]
  WarmMatte,
  #[sdk(rename = "translucentPowder")]
  TranslucentPowder,
  #[sdk(rename = "powder")]
  Powder,
  #[sdk(rename = "dkEdge")]
  DarkEdge,
  #[sdk(rename = "softEdge")]
  SoftEdge,
  #[sdk(rename = "clear")]
  Clear,
  #[sdk(rename = "flat")]
  Flat,
  #[sdk(rename = "softmetal")]
  SoftMetal,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetShadowValues {
  #[sdk(rename = "shdw1")]
  #[default]
  TopLeftDropShadow,
  #[sdk(rename = "shdw2")]
  TopRightDropShadow,
  #[sdk(rename = "shdw3")]
  BackLeftPerspectiveShadow,
  #[sdk(rename = "shdw4")]
  BackRightPerspectiveShadow,
  #[sdk(rename = "shdw5")]
  BottomLeftDropShadow,
  #[sdk(rename = "shdw6")]
  BottomRightDropShadow,
  #[sdk(rename = "shdw7")]
  FrontLeftPerspectiveShadow,
  #[sdk(rename = "shdw8")]
  FrontRightPerspectiveShadow,
  #[sdk(rename = "shdw9")]
  TopLeftSmallDropShadow,
  #[sdk(rename = "shdw10")]
  TopLeftLargeDropShadow,
  #[sdk(rename = "shdw11")]
  BackLeftLongPerspectiveShadow,
  #[sdk(rename = "shdw12")]
  BackRightLongPerspectiveShadow,
  #[sdk(rename = "shdw13")]
  TopLeftDoubleDropShadow,
  #[sdk(rename = "shdw14")]
  BottomRightSmallDropShadow,
  #[sdk(rename = "shdw15")]
  FrontLeftLongPerspectiveShadow,
  #[sdk(rename = "shdw16")]
  FrontRightLongPerspectiveShadow,
  #[sdk(rename = "shdw17")]
  ThreeDimensionalOuterBoxShadow,
  #[sdk(rename = "shdw18")]
  ThreeDimensionalInnerBoxShadow,
  #[sdk(rename = "shdw19")]
  BackCenterPerspectiveShadow,
  #[sdk(rename = "shdw20")]
  FrontBottomShadow,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PathShadeValues {
  #[sdk(rename = "shape")]
  #[default]
  Shape,
  #[sdk(rename = "circle")]
  Circle,
  #[sdk(rename = "rect")]
  Rectangle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TileFlipValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "x")]
  Horizontal,
  #[sdk(rename = "y")]
  Vertical,
  #[sdk(rename = "xy")]
  HorizontalAndVertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BlipCompressionValues {
  #[sdk(rename = "email")]
  #[default]
  Email,
  #[sdk(rename = "screen")]
  Screen,
  #[sdk(rename = "print")]
  Print,
  #[sdk(rename = "hqprint")]
  HighQualityPrint,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetPatternValues {
  #[sdk(rename = "pct5")]
  #[default]
  Percent5,
  #[sdk(rename = "pct10")]
  Percent10,
  #[sdk(rename = "pct20")]
  Percent20,
  #[sdk(rename = "pct25")]
  Percent25,
  #[sdk(rename = "pct30")]
  Percent30,
  #[sdk(rename = "pct40")]
  Percent40,
  #[sdk(rename = "pct50")]
  Percent50,
  #[sdk(rename = "pct60")]
  Percent60,
  #[sdk(rename = "pct70")]
  Percent70,
  #[sdk(rename = "pct75")]
  Percent75,
  #[sdk(rename = "pct80")]
  Percent80,
  #[sdk(rename = "pct90")]
  Percent90,
  #[sdk(rename = "horz")]
  Horizontal,
  #[sdk(rename = "vert")]
  Vertical,
  #[sdk(rename = "ltHorz")]
  LightHorizontal,
  #[sdk(rename = "ltVert")]
  LightVertical,
  #[sdk(rename = "dkHorz")]
  DarkHorizontal,
  #[sdk(rename = "dkVert")]
  DarkVertical,
  #[sdk(rename = "narHorz")]
  NarrowHorizontal,
  #[sdk(rename = "narVert")]
  NarrowVertical,
  #[sdk(rename = "dashHorz")]
  DashedHorizontal,
  #[sdk(rename = "dashVert")]
  DashedVertical,
  #[sdk(rename = "cross")]
  Cross,
  #[sdk(rename = "dnDiag")]
  DownwardDiagonal,
  #[sdk(rename = "upDiag")]
  UpwardDiagonal,
  #[sdk(rename = "ltDnDiag")]
  LightDownwardDiagonal,
  #[sdk(rename = "ltUpDiag")]
  LightUpwardDiagonal,
  #[sdk(rename = "dkDnDiag")]
  DarkDownwardDiagonal,
  #[sdk(rename = "dkUpDiag")]
  DarkUpwardDiagonal,
  #[sdk(rename = "wdDnDiag")]
  WideDownwardDiagonal,
  #[sdk(rename = "wdUpDiag")]
  WideUpwardDiagonal,
  #[sdk(rename = "dashDnDiag")]
  DashedDownwardDiagonal,
  #[sdk(rename = "dashUpDiag")]
  DashedUpwardDiagonal,
  #[sdk(rename = "diagCross")]
  DiagonalCross,
  #[sdk(rename = "smCheck")]
  SmallCheck,
  #[sdk(rename = "lgCheck")]
  LargeCheck,
  #[sdk(rename = "smGrid")]
  SmallGrid,
  #[sdk(rename = "lgGrid")]
  LargeGrid,
  #[sdk(rename = "dotGrid")]
  DotGrid,
  #[sdk(rename = "smConfetti")]
  SmallConfetti,
  #[sdk(rename = "lgConfetti")]
  LargeConfetti,
  #[sdk(rename = "horzBrick")]
  HorizontalBrick,
  #[sdk(rename = "diagBrick")]
  DiagonalBrick,
  #[sdk(rename = "solidDmnd")]
  SolidDiamond,
  #[sdk(rename = "openDmnd")]
  OpenDiamond,
  #[sdk(rename = "dotDmnd")]
  DottedDiamond,
  #[sdk(rename = "plaid")]
  Plaid,
  #[sdk(rename = "sphere")]
  Sphere,
  #[sdk(rename = "weave")]
  Weave,
  #[sdk(rename = "divot")]
  Divot,
  #[sdk(rename = "shingle")]
  Shingle,
  #[sdk(rename = "wave")]
  Wave,
  #[sdk(rename = "trellis")]
  Trellis,
  #[sdk(rename = "zigZag")]
  ZigZag,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BlendModeValues {
  #[sdk(rename = "over")]
  #[default]
  Overlay,
  #[sdk(rename = "mult")]
  Multiply,
  #[sdk(rename = "screen")]
  Screen,
  #[sdk(rename = "darken")]
  Darken,
  #[sdk(rename = "lighten")]
  Lighten,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EffectContainerValues {
  #[sdk(rename = "sib")]
  #[default]
  Sibling,
  #[sdk(rename = "tree")]
  Tree,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShapeTypeValues {
  #[sdk(rename = "line")]
  #[default]
  Line,
  #[sdk(rename = "lineInv")]
  LineInverse,
  #[sdk(rename = "triangle")]
  Triangle,
  #[sdk(rename = "rtTriangle")]
  RightTriangle,
  #[sdk(rename = "rect")]
  Rectangle,
  #[sdk(rename = "diamond")]
  Diamond,
  #[sdk(rename = "parallelogram")]
  Parallelogram,
  #[sdk(rename = "trapezoid")]
  Trapezoid,
  #[sdk(rename = "nonIsoscelesTrapezoid")]
  NonIsoscelesTrapezoid,
  #[sdk(rename = "pentagon")]
  Pentagon,
  #[sdk(rename = "hexagon")]
  Hexagon,
  #[sdk(rename = "heptagon")]
  Heptagon,
  #[sdk(rename = "octagon")]
  Octagon,
  #[sdk(rename = "decagon")]
  Decagon,
  #[sdk(rename = "dodecagon")]
  Dodecagon,
  #[sdk(rename = "star4")]
  Star4,
  #[sdk(rename = "star5")]
  Star5,
  #[sdk(rename = "star6")]
  Star6,
  #[sdk(rename = "star7")]
  Star7,
  #[sdk(rename = "star8")]
  Star8,
  #[sdk(rename = "star10")]
  Star10,
  #[sdk(rename = "star12")]
  Star12,
  #[sdk(rename = "star16")]
  Star16,
  #[sdk(rename = "star24")]
  Star24,
  #[sdk(rename = "star32")]
  Star32,
  #[sdk(rename = "roundRect")]
  RoundRectangle,
  #[sdk(rename = "round1Rect")]
  Round1Rectangle,
  #[sdk(rename = "round2SameRect")]
  Round2SameRectangle,
  #[sdk(rename = "round2DiagRect")]
  Round2DiagonalRectangle,
  #[sdk(rename = "snipRoundRect")]
  SnipRoundRectangle,
  #[sdk(rename = "snip1Rect")]
  Snip1Rectangle,
  #[sdk(rename = "snip2SameRect")]
  Snip2SameRectangle,
  #[sdk(rename = "snip2DiagRect")]
  Snip2DiagonalRectangle,
  #[sdk(rename = "plaque")]
  Plaque,
  #[sdk(rename = "ellipse")]
  Ellipse,
  #[sdk(rename = "teardrop")]
  Teardrop,
  #[sdk(rename = "homePlate")]
  HomePlate,
  #[sdk(rename = "chevron")]
  Chevron,
  #[sdk(rename = "pieWedge")]
  PieWedge,
  #[sdk(rename = "pie")]
  Pie,
  #[sdk(rename = "blockArc")]
  BlockArc,
  #[sdk(rename = "donut")]
  Donut,
  #[sdk(rename = "noSmoking")]
  NoSmoking,
  #[sdk(rename = "rightArrow")]
  RightArrow,
  #[sdk(rename = "leftArrow")]
  LeftArrow,
  #[sdk(rename = "upArrow")]
  UpArrow,
  #[sdk(rename = "downArrow")]
  DownArrow,
  #[sdk(rename = "stripedRightArrow")]
  StripedRightArrow,
  #[sdk(rename = "notchedRightArrow")]
  NotchedRightArrow,
  #[sdk(rename = "bentUpArrow")]
  BentUpArrow,
  #[sdk(rename = "leftRightArrow")]
  LeftRightArrow,
  #[sdk(rename = "upDownArrow")]
  UpDownArrow,
  #[sdk(rename = "leftUpArrow")]
  LeftUpArrow,
  #[sdk(rename = "leftRightUpArrow")]
  LeftRightUpArrow,
  #[sdk(rename = "quadArrow")]
  QuadArrow,
  #[sdk(rename = "leftArrowCallout")]
  LeftArrowCallout,
  #[sdk(rename = "rightArrowCallout")]
  RightArrowCallout,
  #[sdk(rename = "upArrowCallout")]
  UpArrowCallout,
  #[sdk(rename = "downArrowCallout")]
  DownArrowCallout,
  #[sdk(rename = "leftRightArrowCallout")]
  LeftRightArrowCallout,
  #[sdk(rename = "upDownArrowCallout")]
  UpDownArrowCallout,
  #[sdk(rename = "quadArrowCallout")]
  QuadArrowCallout,
  #[sdk(rename = "bentArrow")]
  BentArrow,
  #[sdk(rename = "uturnArrow")]
  UTurnArrow,
  #[sdk(rename = "circularArrow")]
  CircularArrow,
  #[sdk(rename = "leftCircularArrow")]
  LeftCircularArrow,
  #[sdk(rename = "leftRightCircularArrow")]
  LeftRightCircularArrow,
  #[sdk(rename = "curvedRightArrow")]
  CurvedRightArrow,
  #[sdk(rename = "curvedLeftArrow")]
  CurvedLeftArrow,
  #[sdk(rename = "curvedUpArrow")]
  CurvedUpArrow,
  #[sdk(rename = "curvedDownArrow")]
  CurvedDownArrow,
  #[sdk(rename = "swooshArrow")]
  SwooshArrow,
  #[sdk(rename = "cube")]
  Cube,
  #[sdk(rename = "can")]
  Can,
  #[sdk(rename = "lightningBolt")]
  LightningBolt,
  #[sdk(rename = "heart")]
  Heart,
  #[sdk(rename = "sun")]
  Sun,
  #[sdk(rename = "moon")]
  Moon,
  #[sdk(rename = "smileyFace")]
  SmileyFace,
  #[sdk(rename = "irregularSeal1")]
  IrregularSeal1,
  #[sdk(rename = "irregularSeal2")]
  IrregularSeal2,
  #[sdk(rename = "foldedCorner")]
  FoldedCorner,
  #[sdk(rename = "bevel")]
  Bevel,
  #[sdk(rename = "frame")]
  Frame,
  #[sdk(rename = "halfFrame")]
  HalfFrame,
  #[sdk(rename = "corner")]
  Corner,
  #[sdk(rename = "diagStripe")]
  DiagonalStripe,
  #[sdk(rename = "chord")]
  Chord,
  #[sdk(rename = "arc")]
  Arc,
  #[sdk(rename = "leftBracket")]
  LeftBracket,
  #[sdk(rename = "rightBracket")]
  RightBracket,
  #[sdk(rename = "leftBrace")]
  LeftBrace,
  #[sdk(rename = "rightBrace")]
  RightBrace,
  #[sdk(rename = "bracketPair")]
  BracketPair,
  #[sdk(rename = "bracePair")]
  BracePair,
  #[sdk(rename = "straightConnector1")]
  StraightConnector1,
  #[sdk(rename = "bentConnector2")]
  BentConnector2,
  #[sdk(rename = "bentConnector3")]
  BentConnector3,
  #[sdk(rename = "bentConnector4")]
  BentConnector4,
  #[sdk(rename = "bentConnector5")]
  BentConnector5,
  #[sdk(rename = "curvedConnector2")]
  CurvedConnector2,
  #[sdk(rename = "curvedConnector3")]
  CurvedConnector3,
  #[sdk(rename = "curvedConnector4")]
  CurvedConnector4,
  #[sdk(rename = "curvedConnector5")]
  CurvedConnector5,
  #[sdk(rename = "callout1")]
  Callout1,
  #[sdk(rename = "callout2")]
  Callout2,
  #[sdk(rename = "callout3")]
  Callout3,
  #[sdk(rename = "accentCallout1")]
  AccentCallout1,
  #[sdk(rename = "accentCallout2")]
  AccentCallout2,
  #[sdk(rename = "accentCallout3")]
  AccentCallout3,
  #[sdk(rename = "borderCallout1")]
  BorderCallout1,
  #[sdk(rename = "borderCallout2")]
  BorderCallout2,
  #[sdk(rename = "borderCallout3")]
  BorderCallout3,
  #[sdk(rename = "accentBorderCallout1")]
  AccentBorderCallout1,
  #[sdk(rename = "accentBorderCallout2")]
  AccentBorderCallout2,
  #[sdk(rename = "accentBorderCallout3")]
  AccentBorderCallout3,
  #[sdk(rename = "wedgeRectCallout")]
  WedgeRectangleCallout,
  #[sdk(rename = "wedgeRoundRectCallout")]
  WedgeRoundRectangleCallout,
  #[sdk(rename = "wedgeEllipseCallout")]
  WedgeEllipseCallout,
  #[sdk(rename = "cloudCallout")]
  CloudCallout,
  #[sdk(rename = "cloud")]
  Cloud,
  #[sdk(rename = "ribbon")]
  Ribbon,
  #[sdk(rename = "ribbon2")]
  Ribbon2,
  #[sdk(rename = "ellipseRibbon")]
  EllipseRibbon,
  #[sdk(rename = "ellipseRibbon2")]
  EllipseRibbon2,
  #[sdk(rename = "leftRightRibbon")]
  LeftRightRibbon,
  #[sdk(rename = "verticalScroll")]
  VerticalScroll,
  #[sdk(rename = "horizontalScroll")]
  HorizontalScroll,
  #[sdk(rename = "wave")]
  Wave,
  #[sdk(rename = "doubleWave")]
  DoubleWave,
  #[sdk(rename = "plus")]
  Plus,
  #[sdk(rename = "flowChartProcess")]
  FlowChartProcess,
  #[sdk(rename = "flowChartDecision")]
  FlowChartDecision,
  #[sdk(rename = "flowChartInputOutput")]
  FlowChartInputOutput,
  #[sdk(rename = "flowChartPredefinedProcess")]
  FlowChartPredefinedProcess,
  #[sdk(rename = "flowChartInternalStorage")]
  FlowChartInternalStorage,
  #[sdk(rename = "flowChartDocument")]
  FlowChartDocument,
  #[sdk(rename = "flowChartMultidocument")]
  FlowChartMultidocument,
  #[sdk(rename = "flowChartTerminator")]
  FlowChartTerminator,
  #[sdk(rename = "flowChartPreparation")]
  FlowChartPreparation,
  #[sdk(rename = "flowChartManualInput")]
  FlowChartManualInput,
  #[sdk(rename = "flowChartManualOperation")]
  FlowChartManualOperation,
  #[sdk(rename = "flowChartConnector")]
  FlowChartConnector,
  #[sdk(rename = "flowChartPunchedCard")]
  FlowChartPunchedCard,
  #[sdk(rename = "flowChartPunchedTape")]
  FlowChartPunchedTape,
  #[sdk(rename = "flowChartSummingJunction")]
  FlowChartSummingJunction,
  #[sdk(rename = "flowChartOr")]
  FlowChartOr,
  #[sdk(rename = "flowChartCollate")]
  FlowChartCollate,
  #[sdk(rename = "flowChartSort")]
  FlowChartSort,
  #[sdk(rename = "flowChartExtract")]
  FlowChartExtract,
  #[sdk(rename = "flowChartMerge")]
  FlowChartMerge,
  #[sdk(rename = "flowChartOfflineStorage")]
  FlowChartOfflineStorage,
  #[sdk(rename = "flowChartOnlineStorage")]
  FlowChartOnlineStorage,
  #[sdk(rename = "flowChartMagneticTape")]
  FlowChartMagneticTape,
  #[sdk(rename = "flowChartMagneticDisk")]
  FlowChartMagneticDisk,
  #[sdk(rename = "flowChartMagneticDrum")]
  FlowChartMagneticDrum,
  #[sdk(rename = "flowChartDisplay")]
  FlowChartDisplay,
  #[sdk(rename = "flowChartDelay")]
  FlowChartDelay,
  #[sdk(rename = "flowChartAlternateProcess")]
  FlowChartAlternateProcess,
  #[sdk(rename = "flowChartOffpageConnector")]
  FlowChartOffpageConnector,
  #[sdk(rename = "actionButtonBlank")]
  ActionButtonBlank,
  #[sdk(rename = "actionButtonHome")]
  ActionButtonHome,
  #[sdk(rename = "actionButtonHelp")]
  ActionButtonHelp,
  #[sdk(rename = "actionButtonInformation")]
  ActionButtonInformation,
  #[sdk(rename = "actionButtonForwardNext")]
  ActionButtonForwardNext,
  #[sdk(rename = "actionButtonBackPrevious")]
  ActionButtonBackPrevious,
  #[sdk(rename = "actionButtonEnd")]
  ActionButtonEnd,
  #[sdk(rename = "actionButtonBeginning")]
  ActionButtonBeginning,
  #[sdk(rename = "actionButtonReturn")]
  ActionButtonReturn,
  #[sdk(rename = "actionButtonDocument")]
  ActionButtonDocument,
  #[sdk(rename = "actionButtonSound")]
  ActionButtonSound,
  #[sdk(rename = "actionButtonMovie")]
  ActionButtonMovie,
  #[sdk(rename = "gear6")]
  Gear6,
  #[sdk(rename = "gear9")]
  Gear9,
  #[sdk(rename = "funnel")]
  Funnel,
  #[sdk(rename = "mathPlus")]
  MathPlus,
  #[sdk(rename = "mathMinus")]
  MathMinus,
  #[sdk(rename = "mathMultiply")]
  MathMultiply,
  #[sdk(rename = "mathDivide")]
  MathDivide,
  #[sdk(rename = "mathEqual")]
  MathEqual,
  #[sdk(rename = "mathNotEqual")]
  MathNotEqual,
  #[sdk(rename = "cornerTabs")]
  CornerTabs,
  #[sdk(rename = "squareTabs")]
  SquareTabs,
  #[sdk(rename = "plaqueTabs")]
  PlaqueTabs,
  #[sdk(rename = "chartX")]
  ChartX,
  #[sdk(rename = "chartStar")]
  ChartStar,
  #[sdk(rename = "chartPlus")]
  ChartPlus,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextShapeValues {
  #[sdk(rename = "textNoShape")]
  #[default]
  TextNoShape,
  #[sdk(rename = "textPlain")]
  TextPlain,
  #[sdk(rename = "textStop")]
  TextStop,
  #[sdk(rename = "textTriangle")]
  TextTriangle,
  #[sdk(rename = "textTriangleInverted")]
  TextTriangleInverted,
  #[sdk(rename = "textChevron")]
  TextChevron,
  #[sdk(rename = "textChevronInverted")]
  TextChevronInverted,
  #[sdk(rename = "textRingInside")]
  TextRingInside,
  #[sdk(rename = "textRingOutside")]
  TextRingOutside,
  #[sdk(rename = "textArchUp")]
  TextArchUp,
  #[sdk(rename = "textArchDown")]
  TextArchDown,
  #[sdk(rename = "textCircle")]
  TextCircle,
  #[sdk(rename = "textButton")]
  TextButton,
  #[sdk(rename = "textArchUpPour")]
  TextArchUpPour,
  #[sdk(rename = "textArchDownPour")]
  TextArchDownPour,
  #[sdk(rename = "textCirclePour")]
  TextCirclePour,
  #[sdk(rename = "textButtonPour")]
  TextButtonPour,
  #[sdk(rename = "textCurveUp")]
  TextCurveUp,
  #[sdk(rename = "textCurveDown")]
  TextCurveDown,
  #[sdk(rename = "textCanUp")]
  TextCanUp,
  #[sdk(rename = "textCanDown")]
  TextCanDown,
  #[sdk(rename = "textWave1")]
  TextWave1,
  #[sdk(rename = "textWave2")]
  TextWave2,
  #[sdk(rename = "textDoubleWave1")]
  TextDoubleWave1,
  #[sdk(rename = "textWave4")]
  TextWave4,
  #[sdk(rename = "textInflate")]
  TextInflate,
  #[sdk(rename = "textDeflate")]
  TextDeflate,
  #[sdk(rename = "textInflateBottom")]
  TextInflateBottom,
  #[sdk(rename = "textDeflateBottom")]
  TextDeflateBottom,
  #[sdk(rename = "textInflateTop")]
  TextInflateTop,
  #[sdk(rename = "textDeflateTop")]
  TextDeflateTop,
  #[sdk(rename = "textDeflateInflate")]
  TextDeflateInflate,
  #[sdk(rename = "textDeflateInflateDeflate")]
  TextDeflateInflateDeflate,
  #[sdk(rename = "textFadeRight")]
  TextFadeRight,
  #[sdk(rename = "textFadeLeft")]
  TextFadeLeft,
  #[sdk(rename = "textFadeUp")]
  TextFadeUp,
  #[sdk(rename = "textFadeDown")]
  TextFadeDown,
  #[sdk(rename = "textSlantUp")]
  TextSlantUp,
  #[sdk(rename = "textSlantDown")]
  TextSlantDown,
  #[sdk(rename = "textCascadeUp")]
  TextCascadeUp,
  #[sdk(rename = "textCascadeDown")]
  TextCascadeDown,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PathFillModeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "norm")]
  Norm,
  #[sdk(rename = "lighten")]
  Lighten,
  #[sdk(rename = "lightenLess")]
  LightenLess,
  #[sdk(rename = "darken")]
  Darken,
  #[sdk(rename = "darkenLess")]
  DarkenLess,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineEndValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "triangle")]
  Triangle,
  #[sdk(rename = "stealth")]
  Stealth,
  #[sdk(rename = "diamond")]
  Diamond,
  #[sdk(rename = "oval")]
  Oval,
  #[sdk(rename = "arrow")]
  Arrow,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineEndWidthValues {
  #[sdk(rename = "sm")]
  #[default]
  Small,
  #[sdk(rename = "med")]
  Medium,
  #[sdk(rename = "lg")]
  Large,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineEndLengthValues {
  #[sdk(rename = "sm")]
  #[default]
  Small,
  #[sdk(rename = "med")]
  Medium,
  #[sdk(rename = "lg")]
  Large,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetLineDashValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "lgDash")]
  LargeDash,
  #[sdk(rename = "dashDot")]
  DashDot,
  #[sdk(rename = "lgDashDot")]
  LargeDashDot,
  #[sdk(rename = "lgDashDotDot")]
  LargeDashDotDot,
  #[sdk(rename = "sysDash")]
  SystemDash,
  #[sdk(rename = "sysDot")]
  SystemDot,
  #[sdk(rename = "sysDashDot")]
  SystemDashDot,
  #[sdk(rename = "sysDashDotDot")]
  SystemDashDotDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LineCapValues {
  #[sdk(rename = "rnd")]
  #[default]
  Round,
  #[sdk(rename = "sq")]
  Square,
  #[sdk(rename = "flat")]
  Flat,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PenAlignmentValues {
  #[sdk(rename = "ctr")]
  #[default]
  Center,
  #[sdk(rename = "in")]
  Insert,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CompoundLineValues {
  #[sdk(rename = "sng")]
  #[default]
  Single,
  #[sdk(rename = "dbl")]
  Double,
  #[sdk(rename = "thickThin")]
  ThickThin,
  #[sdk(rename = "thinThick")]
  ThinThick,
  #[sdk(rename = "tri")]
  Triple,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BooleanStyleValues {
  #[sdk(rename = "on")]
  #[default]
  On,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "def")]
  Default,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextVerticalOverflowValues {
  #[sdk(rename = "overflow")]
  #[default]
  Overflow,
  #[sdk(rename = "ellipsis")]
  Ellipsis,
  #[sdk(rename = "clip")]
  Clip,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextHorizontalOverflowValues {
  #[sdk(rename = "overflow")]
  #[default]
  Overflow,
  #[sdk(rename = "clip")]
  Clip,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextVerticalValues {
  #[sdk(rename = "horz")]
  #[default]
  Horizontal,
  #[sdk(rename = "vert")]
  Vertical,
  #[sdk(rename = "vert270")]
  Vertical270,
  #[sdk(rename = "wordArtVert")]
  WordArtVertical,
  #[sdk(rename = "eaVert")]
  EastAsianVetical,
  #[sdk(rename = "mongolianVert")]
  MongolianVertical,
  #[sdk(rename = "wordArtVertRtl")]
  WordArtLeftToRight,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextWrappingValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "square")]
  Square,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAnchoringTypeValues {
  #[sdk(rename = "t")]
  #[default]
  Top,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "just")]
  Justified,
  #[sdk(rename = "dist")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAutoNumberSchemeValues {
  #[sdk(rename = "alphaLcParenBoth")]
  #[default]
  AlphaLowerCharacterParenBoth,
  #[sdk(rename = "alphaUcParenBoth")]
  AlphaUpperCharacterParenBoth,
  #[sdk(rename = "alphaLcParenR")]
  AlphaLowerCharacterParenR,
  #[sdk(rename = "alphaUcParenR")]
  AlphaUpperCharacterParenR,
  #[sdk(rename = "alphaLcPeriod")]
  AlphaLowerCharacterPeriod,
  #[sdk(rename = "alphaUcPeriod")]
  AlphaUpperCharacterPeriod,
  #[sdk(rename = "arabicParenBoth")]
  ArabicParenBoth,
  #[sdk(rename = "arabicParenR")]
  ArabicParenR,
  #[sdk(rename = "arabicPeriod")]
  ArabicPeriod,
  #[sdk(rename = "arabicPlain")]
  ArabicPlain,
  #[sdk(rename = "romanLcParenBoth")]
  RomanLowerCharacterParenBoth,
  #[sdk(rename = "romanUcParenBoth")]
  RomanUpperCharacterParenBoth,
  #[sdk(rename = "romanLcParenR")]
  RomanLowerCharacterParenR,
  #[sdk(rename = "romanUcParenR")]
  RomanUpperCharacterParenR,
  #[sdk(rename = "romanLcPeriod")]
  RomanLowerCharacterPeriod,
  #[sdk(rename = "romanUcPeriod")]
  RomanUpperCharacterPeriod,
  #[sdk(rename = "circleNumDbPlain")]
  CircleNumberDoubleBytePlain,
  #[sdk(rename = "circleNumWdBlackPlain")]
  CircleNumberWingdingsBlackPlain,
  #[sdk(rename = "circleNumWdWhitePlain")]
  CircleNumberWingdingsWhitePlain,
  #[sdk(rename = "arabicDbPeriod")]
  ArabicDoubleBytePeriod,
  #[sdk(rename = "arabicDbPlain")]
  ArabicDoubleBytePlain,
  #[sdk(rename = "ea1ChsPeriod")]
  EastAsianSimplifiedChinesePeriod,
  #[sdk(rename = "ea1ChsPlain")]
  EastAsianSimplifiedChinesePlain,
  #[sdk(rename = "ea1ChtPeriod")]
  EastAsianTraditionalChinesePeriod,
  #[sdk(rename = "ea1ChtPlain")]
  EastAsianTraditionalChinesePlain,
  #[sdk(rename = "ea1JpnChsDbPeriod")]
  EastAsianJapaneseDoubleBytePeriod,
  #[sdk(rename = "ea1JpnKorPlain")]
  EastAsianJapaneseKoreanPlain,
  #[sdk(rename = "ea1JpnKorPeriod")]
  EastAsianJapaneseKoreanPeriod,
  #[sdk(rename = "arabic1Minus")]
  Arabic1Minus,
  #[sdk(rename = "arabic2Minus")]
  Arabic2Minus,
  #[sdk(rename = "hebrew2Minus")]
  Hebrew2Minus,
  #[sdk(rename = "thaiAlphaPeriod")]
  ThaiAlphaPeriod,
  #[sdk(rename = "thaiAlphaParenR")]
  ThaiAlphaParenthesisRight,
  #[sdk(rename = "thaiAlphaParenBoth")]
  ThaiAlphaParenthesisBoth,
  #[sdk(rename = "thaiNumPeriod")]
  ThaiNumberPeriod,
  #[sdk(rename = "thaiNumParenR")]
  ThaiNumberParenthesisRight,
  #[sdk(rename = "thaiNumParenBoth")]
  ThaiNumberParenthesisBoth,
  #[sdk(rename = "hindiAlphaPeriod")]
  HindiAlphaPeriod,
  #[sdk(rename = "hindiNumPeriod")]
  HindiNumPeriod,
  #[sdk(rename = "hindiNumParenR")]
  HindiNumberParenthesisRight,
  #[sdk(rename = "hindiAlpha1Period")]
  HindiAlpha1Period,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextUnderlineValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "words")]
  Words,
  #[sdk(rename = "sng")]
  Single,
  #[sdk(rename = "dbl")]
  Double,
  #[sdk(rename = "heavy")]
  Heavy,
  #[sdk(rename = "dotted")]
  Dotted,
  #[sdk(rename = "dottedHeavy")]
  HeavyDotted,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "dashHeavy")]
  DashHeavy,
  #[sdk(rename = "dashLong")]
  DashLong,
  #[sdk(rename = "dashLongHeavy")]
  DashLongHeavy,
  #[sdk(rename = "dotDash")]
  DotDash,
  #[sdk(rename = "dotDashHeavy")]
  DotDashHeavy,
  #[sdk(rename = "dotDotDash")]
  DotDotDash,
  #[sdk(rename = "dotDotDashHeavy")]
  DotDotDashHeavy,
  #[sdk(rename = "wavy")]
  Wavy,
  #[sdk(rename = "wavyHeavy")]
  WavyHeavy,
  #[sdk(rename = "wavyDbl")]
  WavyDouble,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextStrikeValues {
  #[sdk(rename = "noStrike")]
  #[default]
  NoStrike,
  #[sdk(rename = "sngStrike")]
  SingleStrike,
  #[sdk(rename = "dblStrike")]
  DoubleStrike,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextCapsValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "small")]
  Small,
  #[sdk(rename = "all")]
  All,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextTabAlignmentValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "dec")]
  Decimal,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAlignmentTypeValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "just")]
  Justified,
  #[sdk(rename = "justLow")]
  JustifiedLow,
  #[sdk(rename = "dist")]
  Distributed,
  #[sdk(rename = "thaiDist")]
  ThaiDistributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextFontAlignmentValues {
  #[sdk(rename = "auto")]
  #[default]
  Automatic,
  #[sdk(rename = "t")]
  Top,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "base")]
  Baseline,
  #[sdk(rename = "b")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PresetColorValues {
  #[sdk(rename = "aliceBlue")]
  #[default]
  AliceBlue,
  #[sdk(rename = "antiqueWhite")]
  AntiqueWhite,
  #[sdk(rename = "aqua")]
  Aqua,
  #[sdk(rename = "aquamarine")]
  Aquamarine,
  #[sdk(rename = "azure")]
  Azure,
  #[sdk(rename = "beige")]
  Beige,
  #[sdk(rename = "bisque")]
  Bisque,
  #[sdk(rename = "black")]
  Black,
  #[sdk(rename = "blanchedAlmond")]
  BlanchedAlmond,
  #[sdk(rename = "blue")]
  Blue,
  #[sdk(rename = "blueViolet")]
  BlueViolet,
  #[sdk(rename = "brown")]
  Brown,
  #[sdk(rename = "burlyWood")]
  BurlyWood,
  #[sdk(rename = "cadetBlue")]
  CadetBlue,
  #[sdk(rename = "chartreuse")]
  Chartreuse,
  #[sdk(rename = "chocolate")]
  Chocolate,
  #[sdk(rename = "coral")]
  Coral,
  #[sdk(rename = "cornflowerBlue")]
  CornflowerBlue,
  #[sdk(rename = "cornsilk")]
  Cornsilk,
  #[sdk(rename = "crimson")]
  Crimson,
  #[sdk(rename = "cyan")]
  Cyan,
  #[sdk(rename = "dkBlue")]
  DarkBlue,
  #[sdk(rename = "dkCyan")]
  DarkCyan,
  #[sdk(rename = "dkGoldenrod")]
  DarkGoldenrod,
  #[sdk(rename = "dkGray")]
  DarkGray,
  #[sdk(rename = "dkGreen")]
  DarkGreen,
  #[sdk(rename = "dkKhaki")]
  DarkKhaki,
  #[sdk(rename = "dkMagenta")]
  DarkMagenta,
  #[sdk(rename = "dkOliveGreen")]
  DarkOliveGreen,
  #[sdk(rename = "dkOrange")]
  DarkOrange,
  #[sdk(rename = "dkOrchid")]
  DarkOrchid,
  #[sdk(rename = "dkRed")]
  DarkRed,
  #[sdk(rename = "dkSalmon")]
  DarkSalmon,
  #[sdk(rename = "dkSeaGreen")]
  DarkSeaGreen,
  #[sdk(rename = "dkSlateBlue")]
  DarkSlateBlue,
  #[sdk(rename = "dkSlateGray")]
  DarkSlateGray,
  #[sdk(rename = "dkTurquoise")]
  DarkTurquoise,
  #[sdk(rename = "dkViolet")]
  DarkViolet,
  #[sdk(rename = "deepPink")]
  DeepPink,
  #[sdk(rename = "deepSkyBlue")]
  DeepSkyBlue,
  #[sdk(rename = "dimGray")]
  DimGray,
  #[sdk(rename = "dodgerBlue")]
  DodgerBlue,
  #[sdk(rename = "firebrick")]
  Firebrick,
  #[sdk(rename = "floralWhite")]
  FloralWhite,
  #[sdk(rename = "forestGreen")]
  ForestGreen,
  #[sdk(rename = "fuchsia")]
  Fuchsia,
  #[sdk(rename = "gainsboro")]
  Gainsboro,
  #[sdk(rename = "ghostWhite")]
  GhostWhite,
  #[sdk(rename = "gold")]
  Gold,
  #[sdk(rename = "goldenrod")]
  Goldenrod,
  #[sdk(rename = "gray")]
  Gray,
  #[sdk(rename = "green")]
  Green,
  #[sdk(rename = "greenYellow")]
  GreenYellow,
  #[sdk(rename = "honeydew")]
  Honeydew,
  #[sdk(rename = "hotPink")]
  HotPink,
  #[sdk(rename = "indianRed")]
  IndianRed,
  #[sdk(rename = "indigo")]
  Indigo,
  #[sdk(rename = "ivory")]
  Ivory,
  #[sdk(rename = "khaki")]
  Khaki,
  #[sdk(rename = "lavender")]
  Lavender,
  #[sdk(rename = "lavenderBlush")]
  LavenderBlush,
  #[sdk(rename = "lawnGreen")]
  LawnGreen,
  #[sdk(rename = "lemonChiffon")]
  LemonChiffon,
  #[sdk(rename = "ltBlue")]
  LightBlue,
  #[sdk(rename = "ltCoral")]
  LightCoral,
  #[sdk(rename = "ltCyan")]
  LightCyan,
  #[sdk(rename = "ltGoldenrodYellow")]
  LightGoldenrodYellow,
  #[sdk(rename = "ltGray")]
  LightGray,
  #[sdk(rename = "ltGreen")]
  LightGreen,
  #[sdk(rename = "ltPink")]
  LightPink,
  #[sdk(rename = "ltSalmon")]
  LightSalmon,
  #[sdk(rename = "ltSeaGreen")]
  LightSeaGreen,
  #[sdk(rename = "ltSkyBlue")]
  LightSkyBlue,
  #[sdk(rename = "ltSlateGray")]
  LightSlateGray,
  #[sdk(rename = "ltSteelBlue")]
  LightSteelBlue,
  #[sdk(rename = "ltYellow")]
  LightYellow,
  #[sdk(rename = "lime")]
  Lime,
  #[sdk(rename = "limeGreen")]
  LimeGreen,
  #[sdk(rename = "linen")]
  Linen,
  #[sdk(rename = "magenta")]
  Magenta,
  #[sdk(rename = "maroon")]
  Maroon,
  #[sdk(rename = "medAquamarine")]
  MedAquamarine,
  #[sdk(rename = "medBlue")]
  MediumBlue,
  #[sdk(rename = "medOrchid")]
  MediumOrchid,
  #[sdk(rename = "medPurple")]
  MediumPurple,
  #[sdk(rename = "medSeaGreen")]
  MediumSeaGreen,
  #[sdk(rename = "medSlateBlue")]
  MediumSlateBlue,
  #[sdk(rename = "medSpringGreen")]
  MediumSpringGreen,
  #[sdk(rename = "medTurquoise")]
  MediumTurquoise,
  #[sdk(rename = "medVioletRed")]
  MediumVioletRed,
  #[sdk(rename = "midnightBlue")]
  MidnightBlue,
  #[sdk(rename = "mintCream")]
  MintCream,
  #[sdk(rename = "mistyRose")]
  MistyRose,
  #[sdk(rename = "moccasin")]
  Moccasin,
  #[sdk(rename = "navajoWhite")]
  NavajoWhite,
  #[sdk(rename = "navy")]
  Navy,
  #[sdk(rename = "oldLace")]
  OldLace,
  #[sdk(rename = "olive")]
  Olive,
  #[sdk(rename = "oliveDrab")]
  OliveDrab,
  #[sdk(rename = "orange")]
  Orange,
  #[sdk(rename = "orangeRed")]
  OrangeRed,
  #[sdk(rename = "orchid")]
  Orchid,
  #[sdk(rename = "paleGoldenrod")]
  PaleGoldenrod,
  #[sdk(rename = "paleGreen")]
  PaleGreen,
  #[sdk(rename = "paleTurquoise")]
  PaleTurquoise,
  #[sdk(rename = "paleVioletRed")]
  PaleVioletRed,
  #[sdk(rename = "papayaWhip")]
  PapayaWhip,
  #[sdk(rename = "peachPuff")]
  PeachPuff,
  #[sdk(rename = "peru")]
  Peru,
  #[sdk(rename = "pink")]
  Pink,
  #[sdk(rename = "plum")]
  Plum,
  #[sdk(rename = "powderBlue")]
  PowderBlue,
  #[sdk(rename = "purple")]
  Purple,
  #[sdk(rename = "red")]
  Red,
  #[sdk(rename = "rosyBrown")]
  RosyBrown,
  #[sdk(rename = "royalBlue")]
  RoyalBlue,
  #[sdk(rename = "saddleBrown")]
  SaddleBrown,
  #[sdk(rename = "salmon")]
  Salmon,
  #[sdk(rename = "sandyBrown")]
  SandyBrown,
  #[sdk(rename = "seaGreen")]
  SeaGreen,
  #[sdk(rename = "seaShell")]
  SeaShell,
  #[sdk(rename = "sienna")]
  Sienna,
  #[sdk(rename = "silver")]
  Silver,
  #[sdk(rename = "skyBlue")]
  SkyBlue,
  #[sdk(rename = "slateBlue")]
  SlateBlue,
  #[sdk(rename = "slateGray")]
  SlateGray,
  #[sdk(rename = "snow")]
  Snow,
  #[sdk(rename = "springGreen")]
  SpringGreen,
  #[sdk(rename = "steelBlue")]
  SteelBlue,
  #[sdk(rename = "tan")]
  Tan,
  #[sdk(rename = "teal")]
  Teal,
  #[sdk(rename = "thistle")]
  Thistle,
  #[sdk(rename = "tomato")]
  Tomato,
  #[sdk(rename = "turquoise")]
  Turquoise,
  #[sdk(rename = "violet")]
  Violet,
  #[sdk(rename = "wheat")]
  Wheat,
  #[sdk(rename = "white")]
  White,
  #[sdk(rename = "whiteSmoke")]
  WhiteSmoke,
  #[sdk(rename = "yellow")]
  Yellow,
  #[sdk(rename = "yellowGreen")]
  YellowGreen,
  #[sdk(rename = "darkBlue")]
  DarkBlue2010,
  #[sdk(rename = "darkCyan")]
  DarkCyan2010,
  #[sdk(rename = "darkGoldenrod")]
  DarkGoldenrod2010,
  #[sdk(rename = "darkGray")]
  DarkGray2010,
  #[sdk(rename = "darkGrey")]
  DarkGrey2010,
  #[sdk(rename = "darkGreen")]
  DarkGreen2010,
  #[sdk(rename = "darkKhaki")]
  DarkKhaki2010,
  #[sdk(rename = "darkMagenta")]
  DarkMagenta2010,
  #[sdk(rename = "darkOliveGreen")]
  DarkOliveGreen2010,
  #[sdk(rename = "darkOrange")]
  DarkOrange2010,
  #[sdk(rename = "darkOrchid")]
  DarkOrchid2010,
  #[sdk(rename = "darkRed")]
  DarkRed2010,
  #[sdk(rename = "darkSalmon")]
  DarkSalmon2010,
  #[sdk(rename = "darkSeaGreen")]
  DarkSeaGreen2010,
  #[sdk(rename = "darkSlateBlue")]
  DarkSlateBlue2010,
  #[sdk(rename = "darkSlateGray")]
  DarkSlateGray2010,
  #[sdk(rename = "darkSlateGrey")]
  DarkSlateGrey2010,
  #[sdk(rename = "darkTurquoise")]
  DarkTurquoise2010,
  #[sdk(rename = "darkViolet")]
  DarkViolet2010,
  #[sdk(rename = "lightBlue")]
  LightBlue2010,
  #[sdk(rename = "lightCoral")]
  LightCoral2010,
  #[sdk(rename = "lightCyan")]
  LightCyan2010,
  #[sdk(rename = "lightGoldenrodYellow")]
  LightGoldenrodYellow2010,
  #[sdk(rename = "lightGray")]
  LightGray2010,
  #[sdk(rename = "lightGrey")]
  LightGrey2010,
  #[sdk(rename = "lightGreen")]
  LightGreen2010,
  #[sdk(rename = "lightPink")]
  LightPink2010,
  #[sdk(rename = "lightSalmon")]
  LightSalmon2010,
  #[sdk(rename = "lightSeaGreen")]
  LightSeaGreen2010,
  #[sdk(rename = "lightSkyBlue")]
  LightSkyBlue2010,
  #[sdk(rename = "lightSlateGray")]
  LightSlateGray2010,
  #[sdk(rename = "lightSlateGrey")]
  LightSlateGrey2010,
  #[sdk(rename = "lightSteelBlue")]
  LightSteelBlue2010,
  #[sdk(rename = "lightYellow")]
  LightYellow2010,
  #[sdk(rename = "mediumAquamarine")]
  MediumAquamarine2010,
  #[sdk(rename = "mediumBlue")]
  MediumBlue2010,
  #[sdk(rename = "mediumOrchid")]
  MediumOrchid2010,
  #[sdk(rename = "mediumPurple")]
  MediumPurple2010,
  #[sdk(rename = "mediumSeaGreen")]
  MediumSeaGreen2010,
  #[sdk(rename = "mediumSlateBlue")]
  MediumSlateBlue2010,
  #[sdk(rename = "mediumSpringGreen")]
  MediumSpringGreen2010,
  #[sdk(rename = "mediumTurquoise")]
  MediumTurquoise2010,
  #[sdk(rename = "mediumVioletRed")]
  MediumVioletRed2010,
  #[sdk(rename = "dkGrey")]
  DarkGrey,
  #[sdk(rename = "dimGrey")]
  DimGrey,
  #[sdk(rename = "dkSlateGrey")]
  DarkSlateGrey,
  #[sdk(rename = "grey")]
  Grey,
  #[sdk(rename = "ltGrey")]
  LightGrey,
  #[sdk(rename = "ltSlateGrey")]
  LightSlateGrey,
  #[sdk(rename = "slateGrey")]
  SlateGrey,
}
/// Audio from CD.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:audioCd")]
pub struct AudioFromCd {
  /// Audio Start Time
  #[sdk(child(qname = "a:st"))]
  pub start_time: std::boxed::Box<StartTime>,
  /// Audio End Time
  #[sdk(child(qname = "a:end"))]
  pub end_time: std::boxed::Box<EndTime>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Audio from WAV File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:wavAudioFile")]
pub struct WaveAudioFile {
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
/// Sound to play..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:snd")]
pub struct HyperlinkSound {
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
/// Audio from File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:audioFile")]
pub struct AudioFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Video from File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:videoFile")]
pub struct VideoFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// QuickTime from File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:quickTimeFile")]
pub struct QuickTimeFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Tint.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tint")]
pub struct Tint {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::PositiveFixedPercentageValue,
}
/// Shade.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:shade")]
pub struct Shade {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::PositiveFixedPercentageValue,
}
/// Alpha.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alpha")]
pub struct Alpha {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::PositiveFixedPercentageValue,
}
/// Alpha Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaOff")]
pub struct AlphaOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub val: crate::simple_type::FixedPercentageValue,
}
/// Alpha Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaMod")]
pub struct AlphaModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::PositiveDrawingmlPercentageValue,
}
/// Hue Modulate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hueMod")]
pub struct HueModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::PositiveDrawingmlPercentageValue,
}
/// Hue.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hue")]
pub struct Hue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..21600000))]
  pub val: crate::simple_type::Int32Value,
}
/// Hue Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hueOff")]
pub struct HueOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Saturation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sat")]
pub struct Saturation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Saturation Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:satOff")]
pub struct SaturationOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Saturation Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:satMod")]
pub struct SaturationModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Luminance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lum")]
pub struct Luminance {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Luminance Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lumOff")]
pub struct LuminanceOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Luminance Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lumMod")]
pub struct LuminanceModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Red.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:red")]
pub struct Red {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Red Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:redOff")]
pub struct RedOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Red Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:redMod")]
pub struct RedModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Green.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:green")]
pub struct Green {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Green Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:greenOff")]
pub struct GreenOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Green Modification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:greenMod")]
pub struct GreenModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Blue.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blue")]
pub struct Blue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Blue Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blueOff")]
pub struct BlueOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Blue Modification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blueMod")]
pub struct BlueModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DrawingmlPercentageValue,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct Extension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
/// RGB Color Model - Percentage Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:scrgbClr")]
pub struct RgbColorModelPercentage {
  /// Red
  #[sdk(attr(qname = ":r"))]
  pub red_portion: crate::simple_type::DrawingmlPercentageValue,
  /// Green
  #[sdk(attr(qname = ":g"))]
  pub green_portion: crate::simple_type::DrawingmlPercentageValue,
  /// Blue
  #[sdk(attr(qname = ":b"))]
  pub blue_portion: crate::simple_type::DrawingmlPercentageValue,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub rgb_color_model_percentage_choice: Vec<RgbColorModelPercentageChoice>,
}
/// RGB Color Model - Hex Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:srgbClr")]
pub struct RgbColorModelHex {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_length(min = 3u32, max = 3u32))]
  pub val: crate::simple_type::HexBinaryValue,
  /// legacySpreadsheetColorIndex
  #[sdk(attr(qname = "a14:legacySpreadsheetColorIndex"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "80",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub legacy_spreadsheet_color_index: Option<crate::simple_type::Int32Value>,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub rgb_color_model_hex_choice: Vec<RgbColorModelHexChoice>,
}
/// Hue, Saturation, Luminance Color Model.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hslClr")]
pub struct HslColor {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(range = 0..21600000))]
  pub hue_value: crate::simple_type::Int32Value,
  /// Saturation
  #[sdk(attr(qname = ":sat"))]
  pub sat_value: crate::simple_type::DrawingmlPercentageValue,
  /// Luminance
  #[sdk(attr(qname = ":lum"))]
  pub lum_value: crate::simple_type::DrawingmlPercentageValue,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub hsl_color_choice: Vec<HslColorChoice>,
}
/// System Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sysClr")]
pub struct SystemColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: SystemColorValues,
  /// Last Color
  #[sdk(attr(qname = ":lastClr"))]
  #[sdk(string_length(min = 3u32, max = 3u32))]
  pub last_color: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub system_color_choice: Vec<SystemColorChoice>,
}
/// Scheme Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:schemeClr")]
pub struct SchemeColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: SchemeColorValues,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub scheme_color_choice: Vec<SchemeColorChoice>,
}
/// Preset Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:prstClr")]
pub struct PresetColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: PresetColorValues,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub preset_color_choice: Vec<PresetColorChoice>,
}
/// Apply 3D shape properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sp3d")]
pub struct Shape3DType {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Shape Depth
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: Option<crate::simple_type::CoordinateValue>,
  /// Extrusion Height
  #[sdk(attr(qname = ":extrusionH"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub extrusion_height: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Contour Width
  #[sdk(attr(qname = ":contourW"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub contour_width: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Preset Material Type
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(kind = "token"))]
  pub preset_material: Option<PresetMaterialTypeValues>,
  /// Top Bevel
  #[sdk(child(qname = "a:bevelT"))]
  pub bevel_top: Option<BevelTop>,
  /// Bottom Bevel
  #[sdk(child(qname = "a:bevelB"))]
  pub bevel_bottom: Option<BevelBottom>,
  /// Extrusion Color
  #[sdk(child(qname = "a:extrusionClr"))]
  pub extrusion_color: Option<std::boxed::Box<ExtrusionColor>>,
  /// Contour Color
  #[sdk(child(qname = "a:contourClr"))]
  pub contour_color: Option<std::boxed::Box<ContourColor>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// No text in 3D scene.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:flatTx")]
pub struct FlatText {
  /// Z Coordinate
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: Option<crate::simple_type::CoordinateValue>,
}
/// Linear Gradient Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lin")]
pub struct LinearGradientFill {
  /// Angle
  #[sdk(attr(qname = ":ang"))]
  #[sdk(number_range(range = 0..21600000))]
  pub angle: Option<crate::simple_type::Int32Value>,
  /// Scaled
  #[sdk(attr(qname = ":scaled"))]
  pub scaled: Option<crate::simple_type::BooleanValue>,
}
/// Path Gradient.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:path")]
pub struct PathGradientFill {
  /// Gradient Fill Path
  #[sdk(attr(qname = ":path"))]
  #[sdk(string_format(kind = "token"))]
  pub path: Option<PathShadeValues>,
  /// Fill To Rectangle
  #[sdk(child(qname = "a:fillToRect"))]
  pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Tile.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tile")]
pub struct Tile {
  /// Horizontal Offset
  #[sdk(attr(qname = ":tx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub horizontal_offset: Option<crate::simple_type::CoordinateValue>,
  /// Vertical Offset
  #[sdk(attr(qname = ":ty"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub vertical_offset: Option<crate::simple_type::CoordinateValue>,
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Tile Flipping
  #[sdk(attr(qname = ":flip"))]
  #[sdk(string_format(kind = "token"))]
  pub flip: Option<TileFlipValues>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
}
/// Stretch.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:stretch")]
pub struct Stretch {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Fill Rectangle
  #[sdk(child(qname = "a:fillRect"))]
  pub fill_rectangle: Option<FillRectangle>,
}
/// Defines the NoFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:noFill")]
pub struct NoFill {
  pub xmlns: Vec<crate::common::XmlNamespace>,
}
/// Defines the SolidFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:solidFill")]
pub struct SolidFill {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub solid_fill_choice: Option<SolidFillChoice>,
}
/// Defines the GradientFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gradFill")]
pub struct GradientFill {
  /// Tile Flip
  #[sdk(attr(qname = ":flip"))]
  #[sdk(string_format(kind = "token"))]
  pub flip: Option<TileFlipValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Gradient Stop List
  #[sdk(child(qname = "a:gsLst"))]
  pub gradient_stop_list: Option<GradientStopList>,
  #[sdk(
        choice(
            child(variant = LinearGradientFill, qname = "a:lin"),
            child(variant = PathGradientFill, qname = "a:path")
        )
    )]
  pub gradient_fill_choice: Option<GradientFillChoice>,
  /// Tile Rectangle.
  #[sdk(child(qname = "a:tileRect"))]
  pub tile_rectangle: Option<TileRectangle>,
}
/// Defines the BlipFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Defines the Blip Class.
  #[sdk(child(qname = "a:blip"))]
  pub blip: Option<std::boxed::Box<Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:srcRect"))]
  pub source_rectangle: Option<SourceRectangle>,
  #[sdk(
        choice(
            child(variant = Tile, qname = "a:tile"),
            child(variant = Stretch, qname = "a:stretch")
        )
    )]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Pattern Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pattFill")]
pub struct PatternFill {
  /// Preset Pattern
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: Option<PresetPatternValues>,
  /// Foreground color
  #[sdk(child(qname = "a:fgClr"))]
  pub foreground_color: Option<std::boxed::Box<ForegroundColor>>,
  /// Background color
  #[sdk(child(qname = "a:bgClr"))]
  pub background_color: Option<std::boxed::Box<BackgroundColor>>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cont")]
pub struct EffectContainer {
  /// Effect Container Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<EffectContainerValues>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(kind = "token"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = EffectContainer, qname = "a:cont"),
            child(variant = Effect, qname = "a:effect"),
            child(variant = AlphaBiLevel, qname = "a:alphaBiLevel"),
            empty_child(variant = AlphaCeiling, qname = "a:alphaCeiling"),
            empty_child(variant = AlphaFloor, qname = "a:alphaFloor"),
            child(variant = AlphaInverse, qname = "a:alphaInv"),
            child(variant = AlphaModulationEffect, qname = "a:alphaMod"),
            child(variant = AlphaModulationFixed, qname = "a:alphaModFix"),
            child(variant = AlphaOutset, qname = "a:alphaOutset"),
            child(variant = AlphaReplace, qname = "a:alphaRepl"),
            child(variant = BiLevel, qname = "a:biLevel"),
            child(variant = Blend, qname = "a:blend"),
            child(variant = Blur, qname = "a:blur"),
            child(variant = ColorChange, qname = "a:clrChange"),
            child(variant = ColorReplacement, qname = "a:clrRepl"),
            child(variant = Duotone, qname = "a:duotone"),
            child(variant = Fill, qname = "a:fill"),
            child(variant = FillOverlay, qname = "a:fillOverlay"),
            child(variant = Glow, qname = "a:glow"),
            empty_child(variant = Grayscale, qname = "a:grayscl"),
            child(variant = Hsl, qname = "a:hsl"),
            child(variant = InnerShadow, qname = "a:innerShdw"),
            child(variant = LuminanceEffect, qname = "a:lum"),
            child(variant = OuterShadow, qname = "a:outerShdw"),
            child(variant = PresetShadow, qname = "a:prstShdw"),
            child(variant = Reflection, qname = "a:reflection"),
            child(variant = RelativeOffset, qname = "a:relOff"),
            child(variant = SoftEdge, qname = "a:softEdge"),
            child(variant = TintEffect, qname = "a:tint"),
            child(variant = TransformEffect, qname = "a:xfrm")
        )
    )]
  pub effect_container_choice: Vec<EffectContainerChoice>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effectDag")]
pub struct EffectDag {
  /// Effect Container Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<EffectContainerValues>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(kind = "token"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = EffectContainer, qname = "a:cont"),
            child(variant = Effect, qname = "a:effect"),
            child(variant = AlphaBiLevel, qname = "a:alphaBiLevel"),
            empty_child(variant = AlphaCeiling, qname = "a:alphaCeiling"),
            empty_child(variant = AlphaFloor, qname = "a:alphaFloor"),
            child(variant = AlphaInverse, qname = "a:alphaInv"),
            child(variant = AlphaModulationEffect, qname = "a:alphaMod"),
            child(variant = AlphaModulationFixed, qname = "a:alphaModFix"),
            child(variant = AlphaOutset, qname = "a:alphaOutset"),
            child(variant = AlphaReplace, qname = "a:alphaRepl"),
            child(variant = BiLevel, qname = "a:biLevel"),
            child(variant = Blend, qname = "a:blend"),
            child(variant = Blur, qname = "a:blur"),
            child(variant = ColorChange, qname = "a:clrChange"),
            child(variant = ColorReplacement, qname = "a:clrRepl"),
            child(variant = Duotone, qname = "a:duotone"),
            child(variant = Fill, qname = "a:fill"),
            child(variant = FillOverlay, qname = "a:fillOverlay"),
            child(variant = Glow, qname = "a:glow"),
            empty_child(variant = Grayscale, qname = "a:grayscl"),
            child(variant = Hsl, qname = "a:hsl"),
            child(variant = InnerShadow, qname = "a:innerShdw"),
            child(variant = LuminanceEffect, qname = "a:lum"),
            child(variant = OuterShadow, qname = "a:outerShdw"),
            child(variant = PresetShadow, qname = "a:prstShdw"),
            child(variant = Reflection, qname = "a:reflection"),
            child(variant = RelativeOffset, qname = "a:relOff"),
            child(variant = SoftEdge, qname = "a:softEdge"),
            child(variant = TintEffect, qname = "a:tint"),
            child(variant = TransformEffect, qname = "a:xfrm")
        )
    )]
  pub effect_dag_choice: Vec<EffectDagChoice>,
}
/// Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effect")]
pub struct Effect {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(kind = "token"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
/// Defines the AlphaBiLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaBiLevel")]
pub struct AlphaBiLevel {
  /// Threshold
  #[sdk(attr(qname = ":thresh"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub threshold: crate::simple_type::PositiveFixedPercentageValue,
}
/// Alpha Inverse Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaInv")]
pub struct AlphaInverse {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub alpha_inverse_choice: Option<AlphaInverseChoice>,
}
/// Alpha Modulate Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaMod")]
pub struct AlphaModulationEffect {
  /// Effect Container.
  #[sdk(child(qname = "a:cont"))]
  pub effect_container: std::boxed::Box<EffectContainer>,
}
/// Defines the AlphaModulationFixed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaModFix")]
pub struct AlphaModulationFixed {
  /// Amount
  #[sdk(attr(qname = ":amt"))]
  #[sdk(number_range(range = 0..))]
  pub amount: Option<crate::simple_type::PositiveDrawingmlPercentageValue>,
}
/// Alpha Inset/Outset Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaOutset")]
pub struct AlphaOutset {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub radius: Option<crate::simple_type::CoordinateValue>,
}
/// Alpha Replace Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:alphaRepl")]
pub struct AlphaReplace {
  /// Alpha
  #[sdk(attr(qname = ":a"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub alpha: crate::simple_type::PositiveFixedPercentageValue,
}
/// Defines the BiLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:biLevel")]
pub struct BiLevel {
  /// Threshold
  #[sdk(attr(qname = ":thresh"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub threshold: crate::simple_type::PositiveFixedPercentageValue,
}
/// Blend Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blend")]
pub struct Blend {
  /// Blend Mode
  #[sdk(attr(qname = ":blend"))]
  #[sdk(string_format(kind = "token"))]
  pub blend_mode: BlendModeValues,
  /// Effect to blend
  #[sdk(child(qname = "a:cont"))]
  pub effect_container: std::boxed::Box<EffectContainer>,
}
/// Defines the Blur Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blur")]
pub struct Blur {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub radius: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Grow Bounds
  #[sdk(attr(qname = ":grow"))]
  pub grow: Option<crate::simple_type::BooleanValue>,
}
/// Color Change Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrChange")]
pub struct ColorChange {
  /// Consider Alpha Values
  #[sdk(attr(qname = ":useA"))]
  pub use_alpha: Option<crate::simple_type::BooleanValue>,
  /// Change Color From
  #[sdk(child(qname = "a:clrFrom"))]
  pub color_from: std::boxed::Box<ColorFrom>,
  /// Change Color To
  #[sdk(child(qname = "a:clrTo"))]
  pub color_to: std::boxed::Box<ColorTo>,
}
/// Defines the ColorReplacement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrRepl")]
pub struct ColorReplacement {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_replacement_choice: Option<ColorReplacementChoice>,
}
/// Duotone Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:duotone")]
pub struct Duotone {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub duotone_choice: Vec<DuotoneChoice>,
}
/// Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fill")]
pub struct Fill {
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub fill_choice: Option<FillChoice>,
}
/// Fill Overlay Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fillOverlay")]
pub struct FillOverlay {
  /// Blend
  #[sdk(attr(qname = ":blend"))]
  #[sdk(string_format(kind = "token"))]
  pub blend: BlendModeValues,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub fill_overlay_choice: Option<FillOverlayChoice>,
}
/// Glow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:glow")]
pub struct Glow {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub radius: Option<crate::simple_type::PositiveCoordinateValue>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub glow_choice: Option<GlowChoice>,
}
/// Hue Saturation Luminance Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hsl")]
pub struct Hsl {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(range = 0..21600000))]
  pub hue: Option<crate::simple_type::Int32Value>,
  /// Saturation
  #[sdk(attr(qname = ":sat"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub saturation: Option<crate::simple_type::FixedPercentageValue>,
  /// Luminance
  #[sdk(attr(qname = ":lum"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub luminance: Option<crate::simple_type::FixedPercentageValue>,
}
/// Inner Shadow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:innerShdw")]
pub struct InnerShadow {
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub blur_radius: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub distance: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(range = 0..21600000))]
  pub direction: Option<crate::simple_type::Int32Value>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub inner_shadow_choice: Option<InnerShadowChoice>,
}
/// Luminance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lum")]
pub struct LuminanceEffect {
  /// Brightness
  #[sdk(attr(qname = ":bright"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub brightness: Option<crate::simple_type::FixedPercentageValue>,
  /// Contrast
  #[sdk(attr(qname = ":contrast"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub contrast: Option<crate::simple_type::FixedPercentageValue>,
}
/// Outer Shadow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:outerShdw")]
pub struct OuterShadow {
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub blur_radius: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Shadow Offset Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub distance: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Shadow Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(range = 0..21600000))]
  pub direction: Option<crate::simple_type::Int32Value>,
  /// Horizontal Scaling Factor
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Vertical Scaling Factor
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Shadow Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub outer_shadow_choice: Option<OuterShadowChoice>,
}
/// Preset Shadow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:prstShdw")]
pub struct PresetShadow {
  /// Preset Shadow
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: PresetShadowValues,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub distance: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(range = 0..21600000))]
  pub direction: Option<crate::simple_type::Int32Value>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub preset_shadow_choice: Option<PresetShadowChoice>,
}
/// Reflection Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:reflection")]
pub struct Reflection {
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub blur_radius: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Start Opacity
  #[sdk(attr(qname = ":stA"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub start_opacity: Option<crate::simple_type::PositiveFixedPercentageValue>,
  /// Start Position
  #[sdk(attr(qname = ":stPos"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub start_position: Option<crate::simple_type::PositiveFixedPercentageValue>,
  /// End Alpha
  #[sdk(attr(qname = ":endA"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub end_alpha: Option<crate::simple_type::PositiveFixedPercentageValue>,
  /// End Position
  #[sdk(attr(qname = ":endPos"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub end_position: Option<crate::simple_type::PositiveFixedPercentageValue>,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub distance: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(range = 0..21600000))]
  pub direction: Option<crate::simple_type::Int32Value>,
  /// Fade Direction
  #[sdk(attr(qname = ":fadeDir"))]
  #[sdk(number_range(range = 0..21600000))]
  pub fade_direction: Option<crate::simple_type::Int32Value>,
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Shadow Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
}
/// Relative Offset Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:relOff")]
pub struct RelativeOffset {
  /// Offset X
  #[sdk(attr(qname = ":tx"))]
  pub offset_x: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Offset Y
  #[sdk(attr(qname = ":ty"))]
  pub offset_y: Option<crate::simple_type::DrawingmlPercentageValue>,
}
/// Soft Edge Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:softEdge")]
pub struct SoftEdge {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub radius: crate::simple_type::PositiveCoordinateValue,
}
/// Defines the TintEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tint")]
pub struct TintEffect {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(range = 0..21600000))]
  pub hue: Option<crate::simple_type::Int32Value>,
  /// Amount
  #[sdk(attr(qname = ":amt"))]
  #[sdk(number_range(range = -100000..= 100000))]
  pub amount: Option<crate::simple_type::FixedPercentageValue>,
}
/// Transform Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:xfrm")]
pub struct TransformEffect {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(
        number_range(
            min = -5400000,
            max = 5400000,
            min_inclusive = false,
            max_inclusive = false,
        )
    )]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Horizontal Shift
  #[sdk(attr(qname = ":tx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub horizontal_shift: Option<crate::simple_type::CoordinateValue>,
  /// Vertical Shift
  #[sdk(attr(qname = ":ty"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub vertical_shift: Option<crate::simple_type::CoordinateValue>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effectLst")]
pub struct EffectList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Blur Effect
  #[sdk(child(qname = "a:blur"))]
  pub blur: Option<Blur>,
  /// Fill Overlay Effect.
  #[sdk(child(qname = "a:fillOverlay"))]
  pub fill_overlay: Option<std::boxed::Box<FillOverlay>>,
  /// Glow Effect.
  #[sdk(child(qname = "a:glow"))]
  pub glow: Option<std::boxed::Box<Glow>>,
  /// Inner Shadow Effect.
  #[sdk(child(qname = "a:innerShdw"))]
  pub inner_shadow: Option<std::boxed::Box<InnerShadow>>,
  /// Outer Shadow Effect.
  #[sdk(child(qname = "a:outerShdw"))]
  pub outer_shadow: Option<std::boxed::Box<OuterShadow>>,
  /// Preset Shadow.
  #[sdk(child(qname = "a:prstShdw"))]
  pub preset_shadow: Option<std::boxed::Box<PresetShadow>>,
  /// Reflection Effect.
  #[sdk(child(qname = "a:reflection"))]
  pub reflection: Option<Reflection>,
  /// Soft Edge Effect.
  #[sdk(child(qname = "a:softEdge"))]
  pub soft_edge: Option<SoftEdge>,
}
/// Custom geometry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:custGeom")]
pub struct CustomGeometry {
  /// Adjust Value List
  #[sdk(child(qname = "a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
  /// List of Shape Guides
  #[sdk(child(qname = "a:gdLst"))]
  pub shape_guide_list: Option<ShapeGuideList>,
  /// List of Shape Adjust Handles
  #[sdk(child(qname = "a:ahLst"))]
  pub adjust_handle_list: Option<AdjustHandleList>,
  /// List of Shape Connection Sites
  #[sdk(child(qname = "a:cxnLst"))]
  pub connection_site_list: Option<ConnectionSiteList>,
  /// Shape Text Rectangle
  #[sdk(child(qname = "a:rect"))]
  pub rectangle: Option<Rectangle>,
  /// List of Shape Paths
  #[sdk(child(qname = "a:pathLst"))]
  pub path_list: std::boxed::Box<PathList>,
}
/// Preset geometry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:prstGeom")]
pub struct PresetGeometry {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Preset Shape
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: ShapeTypeValues,
  /// List of Shape Adjust Values
  #[sdk(child(qname = "a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
}
/// Preset Text Warp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:prstTxWarp")]
pub struct PresetTextWarp {
  /// Preset Warp Shape
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: TextShapeValues,
  /// Adjust Value List
  #[sdk(child(qname = "a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
}
/// Miter Line Join.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:miter")]
pub struct Miter {
  /// Miter Join Limit
  #[sdk(attr(qname = ":lim"))]
  #[sdk(number_range(range = 0..))]
  pub limit: Option<crate::simple_type::PositiveDrawingmlPercentageValue>,
}
/// Preset Dash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:prstDash")]
pub struct PresetDash {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(kind = "token"))]
  pub val: Option<PresetLineDashValues>,
}
/// Custom Dash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:custDash")]
pub struct CustomDash {
  /// Dash Stop.
  #[sdk(child(qname = "a:ds"))]
  pub dash_stop: Vec<DashStop>,
}
/// Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fill")]
pub struct FillProperties {
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub fill_properties_choice: Option<FillPropertiesChoice>,
}
/// Fill Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fillRef")]
pub struct FillReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub fill_reference_choice: Option<FillReferenceChoice>,
}
/// Effect Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effectRef")]
pub struct EffectReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub effect_reference_choice: Option<EffectReferenceChoice>,
}
/// Defines the LineReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnRef")]
pub struct LineReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Style Matrix Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub line_reference_choice: Option<LineReferenceChoice>,
}
/// Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effect")]
pub struct EffectPropertiesType {
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub effect_properties_type_choice: Option<EffectPropertiesTypeChoice>,
}
/// Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:font")]
pub struct Fonts {
  /// Latin Font
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// Font.
  #[sdk(child(qname = "a:font"))]
  pub supplemental_font: Vec<SupplementalFont>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Major Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:majorFont")]
pub struct MajorFont {
  /// Latin Font
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// Font.
  #[sdk(child(qname = "a:font"))]
  pub supplemental_font: Vec<SupplementalFont>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Minor fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:minorFont")]
pub struct MinorFont {
  /// Latin Font
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// Font.
  #[sdk(child(qname = "a:font"))]
  pub supplemental_font: Vec<SupplementalFont>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fontRef")]
pub struct FontReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Identifier
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(kind = "token"))]
  pub index: FontCollectionIndexValues,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub font_reference_choice: Option<FontReferenceChoice>,
}
/// Normal AutoFit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:normAutofit")]
pub struct NormalAutoFit {
  /// Font Scale
  #[sdk(attr(qname = ":fontScale"))]
  #[sdk(number_range(range = 1000..= 100000))]
  pub font_scale: Option<crate::simple_type::TextFontScalePercentOrPercentStringValue>,
  /// Line Space Reduction
  #[sdk(attr(qname = ":lnSpcReduction"))]
  #[sdk(number_range(range = 0..= 13200000))]
  pub line_space_reduction: Option<crate::simple_type::TextSpacingPercentOrPercentStringValue>,
}
/// Color Specified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buClr")]
pub struct BulletColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub bullet_color_choice: Option<BulletColorChoice>,
}
/// Extrusion Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extrusionClr")]
pub struct ExtrusionColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub extrusion_color_choice: Option<ExtrusionColorChoice>,
}
/// Contour Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:contourClr")]
pub struct ContourColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub contour_color_choice: Option<ContourColorChoice>,
}
/// Change Color From.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrFrom")]
pub struct ColorFrom {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_from_choice: Option<ColorFromChoice>,
}
/// Change Color To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrTo")]
pub struct ColorTo {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_to_choice: Option<ColorToChoice>,
}
/// Foreground color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fgClr")]
pub struct ForegroundColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub foreground_color_choice: Option<ForegroundColorChoice>,
}
/// Background color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bgClr")]
pub struct BackgroundColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub background_color_choice: Option<BackgroundColorChoice>,
}
/// Defines the Highlight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:highlight")]
pub struct Highlight {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub highlight_choice: Option<HighlightChoice>,
}
/// Bullet Size Percentage.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buSzPct")]
pub struct BulletSizePercentage {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 25000..= 400000))]
  pub val: crate::simple_type::TextBulletSizeValue,
}
/// Bullet Size Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buSzPts")]
pub struct BulletSizePoints {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 100..= 400000))]
  pub val: crate::simple_type::TextFontSizeValue,
}
/// Specified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buFont")]
pub struct BulletFont {
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
/// Latin Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:latin")]
pub struct LatinFont {
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
/// East Asian Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ea")]
pub struct EastAsianFont {
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
/// Complex Script Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cs")]
pub struct ComplexScriptFont {
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
/// Defines the SymbolFont Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sym")]
pub struct SymbolFont {
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
/// Auto-Numbered Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buAutoNum")]
pub struct AutoNumberedBullet {
  /// Bullet Autonumbering Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: TextAutoNumberSchemeValues,
  /// Start Numbering At
  #[sdk(attr(qname = ":startAt"))]
  #[sdk(number_range(range = 1..= 32767))]
  pub start_at: Option<crate::simple_type::Int32Value>,
}
/// Character Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buChar")]
pub struct CharacterBullet {
  /// Bullet Character
  #[sdk(attr(qname = ":char"))]
  pub char: crate::simple_type::StringValue,
}
/// Picture Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:buBlip")]
pub struct PictureBullet {
  /// Blip
  #[sdk(child(qname = "a:blip"))]
  pub blip: std::boxed::Box<Blip>,
}
/// Underline Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:uLn")]
pub struct Underline {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub underline_choice1: Option<UnderlineChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub underline_choice2: Option<UnderlineChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub underline_choice3: Option<UnderlineChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ln")]
pub struct Outline {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub outline_choice1: Option<OutlineChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub outline_choice2: Option<OutlineChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub outline_choice3: Option<OutlineChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Left Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnL")]
pub struct LeftBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub left_border_line_properties_choice1: Option<LeftBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub left_border_line_properties_choice2: Option<LeftBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub left_border_line_properties_choice3: Option<LeftBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnR")]
pub struct RightBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub right_border_line_properties_choice1: Option<RightBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub right_border_line_properties_choice2: Option<RightBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub right_border_line_properties_choice3: Option<RightBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Top Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnT")]
pub struct TopBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub top_border_line_properties_choice1: Option<TopBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub top_border_line_properties_choice2: Option<TopBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub top_border_line_properties_choice3: Option<TopBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Bottom Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnB")]
pub struct BottomBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub bottom_border_line_properties_choice1: Option<BottomBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub bottom_border_line_properties_choice2: Option<BottomBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub bottom_border_line_properties_choice3: Option<BottomBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Top-Left to Bottom-Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnTlToBr")]
pub struct TopLeftToBottomRightBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub top_left_to_bottom_right_border_line_properties_choice1:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub top_left_to_bottom_right_border_line_properties_choice2:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub top_left_to_bottom_right_border_line_properties_choice3:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Bottom-Left to Top-Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnBlToTr")]
pub struct BottomLeftToTopRightBorderLineProperties {
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 20116800))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = PatternFill, qname = "a:pattFill")
        )
    )]
  pub bottom_left_to_top_right_border_line_properties_choice1:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = PresetDash, qname = "a:prstDash"),
            child(variant = CustomDash, qname = "a:custDash")
        )
    )]
  pub bottom_left_to_top_right_border_line_properties_choice2:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = Round, qname = "a:round"),
            empty_child(variant = LineJoinBevel, qname = "a:bevel"),
            child(variant = Miter, qname = "a:miter")
        )
    )]
  pub bottom_left_to_top_right_border_line_properties_choice3:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice3>,
  /// default head line end style is none.
  #[sdk(child(qname = "a:headEnd"))]
  pub head_end: Option<HeadEnd>,
  /// default tail line end style is none.
  #[sdk(child(qname = "a:tailEnd"))]
  pub tail_end: Option<TailEnd>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub line_properties_extension_list: Option<LinePropertiesExtensionList>,
}
/// Underline Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:uFill")]
pub struct UnderlineFill {
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub underline_fill_choice: Option<UnderlineFillChoice>,
}
/// Text Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:r")]
pub struct Run {
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// Text Character Properties
  #[sdk(child(qname = "a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Text String
  #[sdk(text_child(simple_type = "StringValue", qname = "a:t"))]
  pub text: Text,
}
/// Text Line Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:br")]
pub struct Break {
  /// Text Run Properties
  #[sdk(child(qname = "a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
}
/// Text Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fld")]
pub struct Field {
  /// Field ID
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Field Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Text Character Properties
  #[sdk(child(qname = "a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Text Paragraph Properties
  #[sdk(child(qname = "a:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  /// Defines the Text Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "a:t"))]
  pub text: Option<Text>,
}
/// Graphic Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:graphic")]
pub struct Graphic {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Graphic Object Data
  #[sdk(child(qname = "a:graphicData"))]
  pub graphic_data: std::boxed::Box<GraphicData>,
}
/// Defines the Blip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:blip")]
pub struct Blip {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(kind = "token"))]
  pub compression_state: Option<BlipCompressionValues>,
  #[sdk(
        choice(
            child(variant = AlphaBiLevel, qname = "a:alphaBiLevel"),
            empty_child(variant = AlphaCeiling, qname = "a:alphaCeiling"),
            empty_child(variant = AlphaFloor, qname = "a:alphaFloor"),
            child(variant = AlphaInverse, qname = "a:alphaInv"),
            child(variant = AlphaModulationEffect, qname = "a:alphaMod"),
            child(variant = AlphaModulationFixed, qname = "a:alphaModFix"),
            child(variant = AlphaReplace, qname = "a:alphaRepl"),
            child(variant = BiLevel, qname = "a:biLevel"),
            child(variant = Blur, qname = "a:blur"),
            child(variant = ColorChange, qname = "a:clrChange"),
            child(variant = ColorReplacement, qname = "a:clrRepl"),
            child(variant = Duotone, qname = "a:duotone"),
            child(variant = FillOverlay, qname = "a:fillOverlay"),
            empty_child(variant = Grayscale, qname = "a:grayscl"),
            child(variant = Hsl, qname = "a:hsl"),
            child(variant = LuminanceEffect, qname = "a:lum"),
            child(variant = TintEffect, qname = "a:tint")
        )
    )]
  pub blip_choice: Vec<BlipChoice>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub blip_extension_list: Option<BlipExtensionList>,
}
/// Theme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:theme")]
pub struct Theme {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = "thm15:id"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub theme_id: Option<crate::simple_type::StringValue>,
  /// Defines the ThemeElements Class.
  #[sdk(child(qname = "a:themeElements"))]
  pub theme_elements: std::boxed::Box<ThemeElements>,
  /// Defines the ObjectDefaults Class.
  #[sdk(child(qname = "a:objectDefaults"))]
  pub object_defaults: Option<std::boxed::Box<ObjectDefaults>>,
  /// Defines the ExtraColorSchemeList Class.
  #[sdk(child(qname = "a:extraClrSchemeLst"))]
  pub extra_color_scheme_list: Option<ExtraColorSchemeList>,
  /// Defines the CustomColorList Class.
  #[sdk(child(qname = "a:custClrLst"))]
  pub custom_color_list: Option<CustomColorList>,
  /// Defines the OfficeStyleSheetExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub office_style_sheet_extension_list: Option<OfficeStyleSheetExtensionList>,
}
/// Theme Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:themeOverride")]
pub struct ThemeOverride {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Color Scheme
  #[sdk(child(qname = "a:clrScheme"))]
  pub color_scheme: Option<std::boxed::Box<ColorScheme>>,
  /// Font Scheme.
  #[sdk(child(qname = "a:fontScheme"))]
  pub font_scheme: Option<std::boxed::Box<FontScheme>>,
  /// Format Scheme.
  #[sdk(child(qname = "a:fmtScheme"))]
  pub format_scheme: Option<std::boxed::Box<FormatScheme>>,
}
/// Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tbl")]
pub struct Table {
  /// Table Properties
  #[sdk(child(qname = "a:tblPr"))]
  pub table_properties: Option<std::boxed::Box<TableProperties>>,
  /// Table Grid
  #[sdk(child(qname = "a:tblGrid"))]
  pub table_grid: std::boxed::Box<TableGrid>,
  /// Table Row.
  #[sdk(child(qname = "a:tr"))]
  pub table_row: Vec<TableRow>,
}
/// Table Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tblStyleLst")]
pub struct TableStyleList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Default
  #[sdk(attr(qname = ":def"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub default: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = TableStyleEntry, qname = "a:tblStyle"), any))]
  pub xml_children: Vec<TableStyleListChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<Extension>,
}
/// Audio Start Time.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:st")]
pub struct StartTime {
  /// Track
  #[sdk(attr(qname = ":track"))]
  pub track: crate::simple_type::ByteValue,
  /// Time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::UInt32Value>,
}
/// Audio End Time.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:end")]
pub struct EndTime {
  /// Track
  #[sdk(attr(qname = ":track"))]
  pub track: crate::simple_type::ByteValue,
  /// Time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::UInt32Value>,
}
/// Custom color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:custClr")]
pub struct CustomColor {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub custom_color_choice: Option<CustomColorChoice>,
}
/// Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:font")]
pub struct SupplementalFont {
  /// Script
  #[sdk(attr(qname = ":script"))]
  pub script: crate::simple_type::StringValue,
  /// Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: crate::simple_type::StringValue,
}
/// 3D Scene Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:scene3d")]
pub struct Scene3DType {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Camera
  #[sdk(child(qname = "a:camera"))]
  pub camera: std::boxed::Box<Camera>,
  /// Light Rig
  #[sdk(child(qname = "a:lightRig"))]
  pub light_rig: std::boxed::Box<LightRig>,
  /// Backdrop Plane
  #[sdk(child(qname = "a:backdrop"))]
  pub backdrop: Option<std::boxed::Box<Backdrop>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Effect Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effectStyle")]
pub struct EffectStyle {
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub effect_style_choice: Option<EffectStyleChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<Shape3DType>>,
}
/// Fill Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fillStyleLst")]
pub struct FillStyleList {
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub fill_style_list_choice: Vec<FillStyleListChoice>,
}
/// Line Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnStyleLst")]
pub struct LineStyleList {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Vec<Outline>,
}
/// Effect Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:effectStyleLst")]
pub struct EffectStyleList {
  /// Effect Style.
  #[sdk(child(qname = "a:effectStyle"))]
  pub effect_style: Vec<EffectStyle>,
}
/// Background Fill Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bgFillStyleLst")]
pub struct BackgroundFillStyleList {
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub background_fill_style_list_choice: Vec<BackgroundFillStyleListChoice>,
}
/// Defines the ColorScheme Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrScheme")]
pub struct ColorScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Dark 1
  #[sdk(child(qname = "a:dk1"))]
  pub dark1_color: std::boxed::Box<Dark1Color>,
  /// Light 1
  #[sdk(child(qname = "a:lt1"))]
  pub light1_color: std::boxed::Box<Light1Color>,
  /// Dark 2
  #[sdk(child(qname = "a:dk2"))]
  pub dark2_color: std::boxed::Box<Dark2Color>,
  /// Light 2
  #[sdk(child(qname = "a:lt2"))]
  pub light2_color: std::boxed::Box<Light2Color>,
  /// Accent 1
  #[sdk(child(qname = "a:accent1"))]
  pub accent1_color: std::boxed::Box<Accent1Color>,
  /// Accent 2
  #[sdk(child(qname = "a:accent2"))]
  pub accent2_color: std::boxed::Box<Accent2Color>,
  /// Accent 3
  #[sdk(child(qname = "a:accent3"))]
  pub accent3_color: std::boxed::Box<Accent3Color>,
  /// Accent 4
  #[sdk(child(qname = "a:accent4"))]
  pub accent4_color: std::boxed::Box<Accent4Color>,
  /// Accent 5
  #[sdk(child(qname = "a:accent5"))]
  pub accent5_color: std::boxed::Box<Accent5Color>,
  /// Accent 6
  #[sdk(child(qname = "a:accent6"))]
  pub accent6_color: std::boxed::Box<Accent6Color>,
  /// Hyperlink
  #[sdk(child(qname = "a:hlink"))]
  pub hyperlink: std::boxed::Box<Hyperlink>,
  /// Followed Hyperlink
  #[sdk(child(qname = "a:folHlink"))]
  pub followed_hyperlink_color: std::boxed::Box<FollowedHyperlinkColor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Font Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fontScheme")]
pub struct FontScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Major Font
  #[sdk(child(qname = "a:majorFont"))]
  pub major_font: std::boxed::Box<MajorFont>,
  /// Minor fonts
  #[sdk(child(qname = "a:minorFont"))]
  pub minor_font: std::boxed::Box<MinorFont>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Format Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fmtScheme")]
pub struct FormatScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Fill Style List
  #[sdk(child(qname = "a:fillStyleLst"))]
  pub fill_style_list: std::boxed::Box<FillStyleList>,
  /// Line Style List
  #[sdk(child(qname = "a:lnStyleLst"))]
  pub line_style_list: std::boxed::Box<LineStyleList>,
  /// Effect Style List
  #[sdk(child(qname = "a:effectStyleLst"))]
  pub effect_style_list: std::boxed::Box<EffectStyleList>,
  /// Background Fill Style List
  #[sdk(child(qname = "a:bgFillStyleLst"))]
  pub background_fill_style_list: std::boxed::Box<BackgroundFillStyleList>,
}
/// Dark 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:dk1")]
pub struct Dark1Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub dark1_color_choice: Option<Dark1ColorChoice>,
}
/// Light 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lt1")]
pub struct Light1Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub light1_color_choice: Option<Light1ColorChoice>,
}
/// Dark 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:dk2")]
pub struct Dark2Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub dark2_color_choice: Option<Dark2ColorChoice>,
}
/// Light 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lt2")]
pub struct Light2Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub light2_color_choice: Option<Light2ColorChoice>,
}
/// Accent 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent1")]
pub struct Accent1Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent1_color_choice: Option<Accent1ColorChoice>,
}
/// Accent 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent2")]
pub struct Accent2Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent2_color_choice: Option<Accent2ColorChoice>,
}
/// Accent 3.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent3")]
pub struct Accent3Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent3_color_choice: Option<Accent3ColorChoice>,
}
/// Accent 4.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent4")]
pub struct Accent4Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent4_color_choice: Option<Accent4ColorChoice>,
}
/// Accent 5.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent5")]
pub struct Accent5Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent5_color_choice: Option<Accent5ColorChoice>,
}
/// Accent 6.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:accent6")]
pub struct Accent6Color {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub accent6_color_choice: Option<Accent6ColorChoice>,
}
/// Hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hlink")]
pub struct Hyperlink {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub hyperlink_choice: Option<HyperlinkChoice>,
}
/// Followed Hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:folHlink")]
pub struct FollowedHyperlinkColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub followed_hyperlink_color_choice: Option<FollowedHyperlinkColorChoice>,
}
/// Horizontal Ratio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sx")]
pub struct ScaleX {
  /// Numerator
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Vertical Ratio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sy")]
pub struct ScaleY {
  /// Numerator
  #[sdk(attr(qname = ":n"))]
  pub numerator: crate::simple_type::Int32Value,
  /// Denominator
  #[sdk(attr(qname = ":d"))]
  pub denominator: crate::simple_type::Int32Value,
}
/// Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:off")]
pub struct Offset {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::CoordinateValue,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::CoordinateValue,
}
/// Child Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:chOff")]
pub struct ChildOffset {
  /// X-Axis Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::CoordinateValue,
  /// Y-Axis Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::CoordinateValue,
}
/// Extents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct Extents {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: crate::simple_type::PositiveCoordinateValue,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: crate::simple_type::PositiveCoordinateValue,
}
/// Child Extents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:chExt")]
pub struct ChildExtents {
  /// Extent Length
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: crate::simple_type::PositiveCoordinateValue,
  /// Extent Width
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: crate::simple_type::PositiveCoordinateValue,
}
/// Shape Locks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spLocks")]
pub struct ShapeLocks {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Text Editing
  #[sdk(attr(qname = ":noTextEdit"))]
  pub no_text_edit: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Connection Shape Locks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cxnSpLocks")]
pub struct ConnectionShapeLocks {
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Defines the ConnectorLockingExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub connector_locking_extension_list: Option<ConnectorLockingExtensionList>,
}
/// Connection Start.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:stCxn")]
pub struct StartConnection {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Connection End.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:endCxn")]
pub struct EndConnection {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
}
/// Graphic Frame Locks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:graphicFrameLocks")]
pub struct GraphicFrameLocks {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Selection of Child Shapes
  #[sdk(attr(qname = ":noDrilldown"))]
  pub no_drilldown: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Graphic Object Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:graphicData")]
pub struct GraphicData {
  /// Uniform Resource Identifier
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = Graphic, qname = "a:graphic"),
            child(variant = Blip, qname = "a:blip"),
            child(variant = Theme, qname = "a:theme"),
            child(variant = ThemeOverride, qname = "a:themeOverride"),
            empty_child(variant = ThemeManager, qname = "a:themeManager"),
            child(variant = Table, qname = "a:tbl"),
            child(variant = TableStyleList, qname = "a:tblStyleLst"),
            child(variant = OEmbedShared, qname = "aoe:oembedShared"),
            child(variant = OEmbed, qname = "woe:oembed"),
            child(variant = ImageFormula, qname = "aif:imageFormula"),
            child(variant = LiveFeedProperties, qname = "alf:liveFeedProps"),
            child(variant = ScriptLink, qname = "asl:scriptLink"),
            child(variant = ClassificationOutcome, qname = "aclsh:classification"),
            child(
                variant = LineSketchStyleProperties,
                qname = "ask:lineSketchStyleProps"
            ),
            child(variant = PictureAttributionSourceUrl, qname = "a1611:picAttrSrcUrl"),
            child(variant = SvgBlip, qname = "asvg:svgBlip"),
            child(variant = Decorative, qname = "adec:decorative"),
            child(variant = A16CreationId, qname = "a16:creationId"),
            child(variant = PredecessorDrawingElementReference, qname = "a16:predDERef"),
            child(variant = ConnectableReferences, qname = "a16:cxnDERefs"),
            child(variant = RowIdIdentifier, qname = "a16:rowId"),
            child(variant = ColIdIdentifier, qname = "a16:colId"),
            child(variant = HyperlinkColor, qname = "ahyp:hlinkClr"),
            child(variant = WebVideoProperty, qname = "wp15:webVideoPr"),
            child(variant = ThemeFamily, qname = "thm15:themeFamily"),
            child(variant = BackgroundProperties, qname = "a15:backgroundPr"),
            child(variant = NonVisualGroupProperties, qname = "a15:nonVisualGroupProps"),
            child(variant = ObjectProperties, qname = "a15:objectPr"),
            child(variant = A15SignatureLine, qname = "a15:signatureLine"),
            child(variant = CameraTool, qname = "a14:cameraTool"),
            child(variant = CompatExtension, qname = "a14:compatExt"),
            child(variant = IsCanvas, qname = "a14:isCanvas"),
            child(variant = GvmlContentPart, qname = "a14:contentPart"),
            child(variant = ShadowObscured, qname = "a14:shadowObscured"),
            child(variant = HiddenFillProperties, qname = "a14:hiddenFill"),
            child(variant = HiddenLineProperties, qname = "a14:hiddenLine"),
            child(variant = HiddenEffectsProperties, qname = "a14:hiddenEffects"),
            child(variant = HiddenScene3D, qname = "a14:hiddenScene3d"),
            child(variant = HiddenShape3D, qname = "a14:hiddenSp3d"),
            child(variant = ImageProperties, qname = "a14:imgProps"),
            child(variant = UseLocalDpi, qname = "a14:useLocalDpi"),
            child(variant = TextMath, qname = "a14:m"),
            child(variant = NonVisualDrawingProperties, qname = "dgm14:cNvPr"),
            child(variant = RecolorImages, qname = "dgm14:recolorImg"),
            child(variant = Drawing, qname = "dsp:drawing"),
            child(variant = DataModelExtensionBlock, qname = "dsp:dataModelExt"),
            child(variant = ColorsDefinition, qname = "dgm:colorsDef"),
            child(variant = ColorsDefinitionHeader, qname = "dgm:colorsDefHdr"),
            child(variant = ColorsDefinitionHeaderList, qname = "dgm:colorsDefHdrLst"),
            child(variant = DataModelRoot, qname = "dgm:dataModel"),
            child(variant = LayoutDefinition, qname = "dgm:layoutDef"),
            child(variant = LayoutDefinitionHeader, qname = "dgm:layoutDefHdr"),
            child(variant = LayoutDefinitionHeaderList, qname = "dgm:layoutDefHdrLst"),
            child(variant = RelationshipIds, qname = "dgm:relIds"),
            child(variant = StyleDefinition, qname = "dgm:styleDef"),
            child(variant = StyleDefinitionHeader, qname = "dgm:styleDefHdr"),
            child(variant = StyleDefinitionHeaderList, qname = "dgm:styleDefHdrLst"),
            child(variant = Dgm1612ShapeProperties, qname = "dgm1612:spPr"),
            child(variant = TextListStyleType, qname = "dgm1612:lstStyle"),
            child(variant = NumberDiagramInfoList, qname = "dgm1611:autoBuNodeInfoLst"),
            child(variant = ChartSpace, qname = "c:chartSpace"),
            child(variant = UserShapes, qname = "c:userShapes"),
            child(variant = ChartReference, qname = "c:chart"),
            child(variant = DataDisplayOptions16, qname = "c16r3:dataDisplayOptions16"),
            child(variant = C16ShapeProperties, qname = "c16:spPr"),
            child(variant = UnsignedIntegerType, qname = "c16:explosion"),
            child(variant = InvertIfNegativeBoolean, qname = "c16:invertIfNegative"),
            child(variant = Bubble3DBoolean, qname = "c16:bubble3D"),
            child(variant = Marker, qname = "c16:marker"),
            child(variant = DLbl, qname = "c16:dLbl"),
            child(
                variant = C16CategoryFilterExceptions,
                qname = "c16:categoryFilterExceptions"
            ),
            child(variant = PivotOptions16, qname = "c16:pivotOptions16"),
            child(
                variant = ChartDataPointUniqueIdMap,
                qname = "c16:datapointuniqueidmap"
            ),
            child(variant = UniqueIdChartUniqueId, qname = "c16:uniqueId"),
            child(variant = PivotSource, qname = "c15:pivotSource"),
            child(variant = NumberingFormat, qname = "c15:numFmt"),
            child(variant = C15ShapeProperties, qname = "c15:spPr"),
            child(variant = Layout, qname = "c15:layout"),
            child(variant = FullReference, qname = "c15:fullRef"),
            child(variant = LevelReference, qname = "c15:levelRef"),
            child(variant = FormulaReference, qname = "c15:formulaRef"),
            child(variant = FilteredSeriesTitle, qname = "c15:filteredSeriesTitle"),
            child(variant = FilteredCategoryTitle, qname = "c15:filteredCategoryTitle"),
            child(variant = FilteredAreaSeries, qname = "c15:filteredAreaSeries"),
            child(variant = FilteredBarSeries, qname = "c15:filteredBarSeries"),
            child(variant = FilteredBubbleSeries, qname = "c15:filteredBubbleSeries"),
            child(
                variant = FilteredLineSeriesExtension,
                qname = "c15:filteredLineSeries"
            ),
            child(variant = FilteredPieSeries, qname = "c15:filteredPieSeries"),
            child(variant = FilteredRadarSeries, qname = "c15:filteredRadarSeries"),
            child(variant = FilteredScatterSeries, qname = "c15:filteredScatterSeries"),
            child(variant = FilteredSurfaceSeries, qname = "c15:filteredSurfaceSeries"),
            child(variant = DataLabelsRange, qname = "c15:datalabelsRange"),
            child(
                variant = C15CategoryFilterExceptions,
                qname = "c15:categoryFilterExceptions"
            ),
            child(variant = DataLabelFieldTable, qname = "c15:dlblFieldTable"),
            child(variant = ExceptionForSave, qname = "c15:xForSave"),
            child(variant = ShowDataLabelsRange, qname = "c15:showDataLabelsRange"),
            child(variant = ChartText, qname = "c15:tx"),
            child(variant = ShowLeaderLines, qname = "c15:showLeaderLines"),
            child(variant = LeaderLines, qname = "c15:leaderLines"),
            child(variant = AutoGeneneratedCategories, qname = "c15:autoCat"),
            child(variant = PivotOptions, qname = "c14:pivotOptions"),
            child(variant = SketchOptions, qname = "c14:sketchOptions"),
            child(variant = InvertSolidFillFormat, qname = "c14:invertSolidFillFmt"),
            child(variant = Style, qname = "c14:style"),
            child(variant = Cdr14ContentPart, qname = "cdr14:contentPart"),
            child(variant = LegacyDrawing, qname = "comp:legacyDrawing"),
            child(variant = LockedCanvas, qname = "lc:lockedCanvas"),
            child(variant = Inline, qname = "wp:inline"),
            child(variant = Anchor, qname = "wp:anchor"),
            text_child(
                variant = PercentagePositionHeightOffset,
                qname = "wp14:pctPosHOffset"
            ),
            text_child(
                variant = PercentagePositionVerticalOffset,
                qname = "wp14:pctPosVOffset"
            ),
            child(variant = RelativeWidth, qname = "wp14:sizeRelH"),
            child(variant = RelativeHeight, qname = "wp14:sizeRelV"),
            child(variant = Picture, qname = "pic:pic"),
            child(variant = ShapeStyle, qname = "pic14:style"),
            child(variant = OfficeArtExtensionList, qname = "pic14:extLst"),
            child(variant = WorksheetDrawing, qname = "xdr:wsDr"),
            child(variant = XdrContentPart, qname = "xdr:contentPart"),
            child(variant = Xdr14ContentPart, qname = "xdr14:contentPart"),
            any_child(variant = CommentAuthorMonikerList, qname = "pc:cmAuthorMkLst"),
            any_child(variant = CommentMonikerList, qname = "pc:cmMkLst"),
            any_child(variant = StringTagMonikerList, qname = "pc:tagMkLst"),
            any_child(variant = CustomShowMonikerList, qname = "pc:custShowMkLst"),
            any_child(variant = DocumentMonikerList, qname = "pc:docMkLst"),
            any_child(variant = SectionMonikerList, qname = "pc:sectionMkLst"),
            any_child(variant = SlideBaseMonikerList, qname = "pc:sldBaseMkLst"),
            any_child(variant = SlideLayoutMonikerList, qname = "pc:sldLayoutMkLst"),
            any_child(variant = MainMasterMonikerList, qname = "pc:sldMasterMkLst"),
            child(variant = SlideMonikerList, qname = "pc:sldMkLst"),
            any_child(variant = SlidePosMonikerList, qname = "pc:sldPosMkLst"),
            any_child(variant = NotesMonikerList, qname = "pc:notesMkLst"),
            any_child(variant = NotesTextMonikerList, qname = "pc:notesTxtMkLst"),
            any_child(variant = NotesMasterMonikerList, qname = "pc:notesMasterMkLst"),
            any_child(variant = HandoutMonikerList, qname = "pc:handoutMkLst"),
            any_child(
                variant = AnimEffectMkLstAnimationEffectMonikerList,
                qname = "pc:animEffectMkLst"
            ),
            any_child(
                variant = AnimEffectParentMkLstAnimationEffectMonikerList,
                qname = "pc:animEffectParentMkLst"
            ),
            any_child(variant = OsfTaskPaneAppMonikerList, qname = "pc:tkAppMkLst"),
            any_child(variant = SummaryZoomMonikerList, qname = "pc:tocMkLst"),
            any_child(
                variant = SectionLinkObjMonikerList,
                qname = "pc:sectionLnkObjMkLst"
            ),
            any_child(variant = DesignerTagMonikerList, qname = "pc:designTagMkLst"),
            any_child(variant = CustomXmlPartMonikerList, qname = "pc:cXmlMkLst"),
            child(variant = CommentAuthorList, qname = "p:cmAuthorLst"),
            child(variant = PCommentList, qname = "p:cmLst"),
            child(variant = POleObject, qname = "p:oleObj"),
            child(variant = Presentation, qname = "p:presentation"),
            child(variant = PresentationProperties, qname = "p:presentationPr"),
            child(variant = Slide, qname = "p:sld"),
            child(variant = SlideLayout, qname = "p:sldLayout"),
            child(variant = SlideMaster, qname = "p:sldMaster"),
            child(variant = HandoutMaster, qname = "p:handoutMaster"),
            child(variant = NotesMaster, qname = "p:notesMaster"),
            child(variant = NotesSlide, qname = "p:notes"),
            child(variant = SlideSyncProperties, qname = "p:sldSyncPr"),
            child(variant = TagList, qname = "p:tagLst"),
            child(variant = ViewProperties, qname = "p:viewPr"),
            child(variant = PContentPart, qname = "p:contentPart"),
            child(variant = PlaceholderTypeExtension, qname = "p232:phTypeExt"),
            child(variant = AuthorList, qname = "p188:authorLst"),
            child(variant = P188CommentList, qname = "p188:cmLst"),
            child(variant = CommentRelationship, qname = "p188:commentRel"),
            child(variant = Reactions, qname = "p223:reactions"),
            child(variant = TaskDetails, qname = "p228:taskDetails"),
            child(variant = TaskHistoryDetails, qname = "p1912:taskHistoryDetails"),
            empty_child(variant = TextBodyPackage, qname = "oac:txBodyPkg"),
            child(variant = GroupCommand, qname = "oac:grpCmd"),
            text_child(variant = ImgDataImgData, qname = "oac:imgData"),
            text_child(variant = OrigImgDataImgData, qname = "oac:origImgData"),
            child(variant = ImgLink, qname = "oac:imgLink"),
            any_child(variant = DrawingMonikerList, qname = "oac:dgMkLst"),
            any_child(variant = DocumentContextMonikerList, qname = "oac:dcMkLst"),
            any_child(
                variant = GraphicParentMonikerList,
                qname = "oac:graphicParentMkLst"
            ),
            any_child(variant = DeMkLstDrawingElementMonikerList, qname = "oac:deMkLst"),
            any_child(
                variant = DeMasterMkLstDrawingElementMonikerList,
                qname = "oac:deMasterMkLst"
            ),
            any_child(variant = ShapeMonikerList, qname = "oac:spMkLst"),
            any_child(variant = GroupShapeMonikerList, qname = "oac:grpSpMkLst"),
            any_child(
                variant = GraphicFrameMonikerList,
                qname = "oac:graphicFrameMkLst"
            ),
            any_child(variant = ConnectorMonikerList, qname = "oac:cxnSpMkLst"),
            any_child(variant = PictureMonikerList, qname = "oac:picMkLst"),
            any_child(variant = InkMonikerList, qname = "oac:inkMkLst"),
            any_child(variant = TextBodyMonikerList, qname = "oac:txBodyMkLst"),
            any_child(variant = TextCharRangeMonikerList, qname = "oac:txMkLst"),
            any_child(variant = HyperlinkMonikerList, qname = "oac:hlinkMkLst"),
            any_child(variant = Model3DMonikerList, qname = "oac:model3DMkLst"),
            any_child(variant = ViewSelectionStgList, qname = "oac:viewSelLst"),
            any_child(variant = EditorSelectionStgList, qname = "oac:editorSelLst"),
            any_child(variant = DrawingSelectionStgList, qname = "oac:drSelLst"),
            any_child(variant = TableMonikerList, qname = "oac:tblMkLst"),
            any_child(variant = TableCellMonikerList, qname = "oac:tcMkLst"),
            any_child(variant = TableRowMonikerList, qname = "oac:trMkLst"),
            any_child(variant = TableColumnMonikerList, qname = "oac:gridColMkLst"),
            child(variant = InkmlInk, qname = "inkml:ink"),
            child(variant = OneOf, qname = "emma:one-of"),
            child(variant = EmmaGroup, qname = "emma:group"),
            child(variant = Sequence, qname = "emma:sequence"),
            child(variant = EndPoint, qname = "emma:endpoint"),
            child(variant = EndPointInfo, qname = "emma:endpoint-info"),
            child(variant = Info, qname = "emma:info"),
            child(variant = Grammar, qname = "emma:grammar"),
            child(variant = DerivedFrom, qname = "emma:derived-from"),
            child(variant = Node, qname = "emma:node"),
            child(variant = EmmaArc, qname = "emma:arc"),
            child(variant = Lattice, qname = "emma:lattice"),
            text_child(variant = Literal, qname = "emma:literal"),
            child(variant = Interpretation, qname = "emma:interpretation"),
            child(variant = GroupInfo, qname = "emma:group-info"),
            child(variant = Derivation, qname = "emma:derivation"),
            child(variant = Model, qname = "emma:model"),
            child(variant = Emma, qname = "emma:emma"),
            child(variant = ContextNode, qname = "msink:context"),
            child(variant = PresetTransition, qname = "p15:prstTrans"),
            child(variant = PresenceInfo, qname = "p15:presenceInfo"),
            child(variant = ThreadingInfo, qname = "p15:threadingInfo"),
            child(variant = SlideGuideList, qname = "p15:sldGuideLst"),
            child(variant = NotesGuideList, qname = "p15:notesGuideLst"),
            child(
                variant = ChartTrackingReferenceBased,
                qname = "p15:chartTrackingRefBased"
            ),
            child(
                variant = NonVisualContentPartProperties,
                qname = "p14:nvContentPartPr"
            ),
            child(variant = Transform2D, qname = "p14:xfrm"),
            child(variant = ExtensionListModify, qname = "p14:extLst"),
            child(variant = Media, qname = "p14:media"),
            child(variant = VortexTransition, qname = "p14:vortex"),
            child(variant = SwitchTransition, qname = "p14:switch"),
            child(variant = FlipTransition, qname = "p14:flip"),
            child(variant = RippleTransition, qname = "p14:ripple"),
            empty_child(variant = HoneycombTransition, qname = "p14:honeycomb"),
            child(variant = PrismTransition, qname = "p14:prism"),
            child(variant = DoorsTransition, qname = "p14:doors"),
            child(variant = WindowTransition, qname = "p14:window"),
            child(variant = FerrisTransition, qname = "p14:ferris"),
            child(variant = GalleryTransition, qname = "p14:gallery"),
            child(variant = ConveyorTransition, qname = "p14:conveyor"),
            child(variant = PanTransition, qname = "p14:pan"),
            child(variant = GlitterTransition, qname = "p14:glitter"),
            child(variant = WarpTransition, qname = "p14:warp"),
            child(variant = FlythroughTransition, qname = "p14:flythrough"),
            empty_child(variant = FlashTransition, qname = "p14:flash"),
            child(variant = ShredTransition, qname = "p14:shred"),
            child(variant = RevealTransition, qname = "p14:reveal"),
            child(variant = WheelReverseTransition, qname = "p14:wheelReverse"),
            child(variant = BookmarkTarget, qname = "p14:bmkTgt"),
            child(variant = SectionProperties, qname = "p14:sectionPr"),
            child(variant = SectionList, qname = "p14:sectionLst"),
            child(variant = BrowseMode, qname = "p14:browseMode"),
            child(variant = LaserColor, qname = "p14:laserClr"),
            child(variant = P14DefaultImageDpi, qname = "p14:defaultImageDpi"),
            child(variant = DiscardImageEditData, qname = "p14:discardImageEditData"),
            child(variant = ShowMediaControls, qname = "p14:showMediaCtrls"),
            child(variant = LaserTraceList, qname = "p14:laserTraceLst"),
            child(variant = P14CreationId, qname = "p14:creationId"),
            child(variant = ModificationId, qname = "p14:modId"),
            child(variant = ShowEventRecordList, qname = "p14:showEvtLst"),
            child(variant = Recipients, qname = "w:recipients"),
            child(variant = TextBoxContent, qname = "w:txbxContent"),
            child(variant = Comments, qname = "w:comments"),
            child(variant = Footnotes, qname = "w:footnotes"),
            child(variant = Endnotes, qname = "w:endnotes"),
            child(variant = Header, qname = "w:hdr"),
            child(variant = Footer, qname = "w:ftr"),
            child(variant = Settings, qname = "w:settings"),
            child(variant = WebSettings, qname = "w:webSettings"),
            child(variant = Fonts, qname = "w:fonts"),
            child(variant = Numbering, qname = "w:numbering"),
            child(variant = Styles, qname = "w:styles"),
            child(variant = Document, qname = "w:document"),
            child(variant = GlossaryDocument, qname = "w:glossaryDocument"),
            child(variant = Color, qname = "w15:color"),
            child(variant = DataBinding, qname = "w15:dataBinding"),
            child(variant = Appearance, qname = "w15:appearance"),
            child(variant = CommentsEx, qname = "w15:commentsEx"),
            child(variant = People, qname = "w15:people"),
            child(variant = SdtRepeatedSection, qname = "w15:repeatingSection"),
            empty_child(
                variant = SdtRepeatedSectionItem,
                qname = "w15:repeatingSectionItem"
            ),
            child(variant = ChartTrackingRefBased, qname = "w15:chartTrackingRefBased"),
            child(variant = DefaultCollapsed, qname = "w15:collapsed"),
            child(variant = PersistentDocumentId, qname = "w15:docId"),
            child(variant = FootnoteColumns, qname = "w15:footnoteColumns"),
            child(variant = WebExtensionLinked, qname = "w15:webExtensionLinked"),
            child(variant = WebExtensionCreated, qname = "w15:webExtensionCreated"),
            child(variant = W14ContentPart, qname = "w14:contentPart"),
            child(variant = DocumentId, qname = "w14:docId"),
            child(variant = ConflictMode, qname = "w14:conflictMode"),
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
            child(
                variant = DiscardImageEditingData,
                qname = "w14:discardImageEditingData"
            ),
            child(variant = W14DefaultImageDpi, qname = "w14:defaultImageDpi"),
            empty_child(variant = EntityPickerEmpty, qname = "w14:entityPicker"),
            child(variant = SdtContentCheckBox, qname = "w14:checkbox"),
            child(variant = SchemaLibrary, qname = "sl:schemaLibrary"),
            child(variant = MathProperties, qname = "m:mathPr"),
            child(variant = Paragraph, qname = "m:oMathPara"),
            child(variant = OfficeMath, qname = "m:oMath"),
            child(variant = Shape, qname = "v:shape"),
            child(variant = Shapetype, qname = "v:shapetype"),
            child(variant = VGroup, qname = "v:group"),
            child(variant = Background, qname = "v:background"),
            child(variant = Fill, qname = "v:fill"),
            child(variant = Formulas, qname = "v:formulas"),
            child(variant = ShapeHandles, qname = "v:handles"),
            child(variant = ImageData, qname = "v:imagedata"),
            child(variant = Path, qname = "v:path"),
            child(variant = TextBox, qname = "v:textbox"),
            child(variant = Shadow, qname = "v:shadow"),
            child(variant = Stroke, qname = "v:stroke"),
            child(variant = TextPath, qname = "v:textpath"),
            child(variant = VArc, qname = "v:arc"),
            child(variant = Curve, qname = "v:curve"),
            child(variant = ImageFile, qname = "v:image"),
            child(variant = Line, qname = "v:line"),
            child(variant = Oval, qname = "v:oval"),
            child(variant = PolyLine, qname = "v:polyline"),
            child(variant = Rectangle, qname = "v:rect"),
            child(variant = RoundRectangle, qname = "v:roundrect"),
            child(variant = ShapeDefaults, qname = "o:shapedefaults"),
            child(variant = ShapeLayout, qname = "o:shapelayout"),
            child(variant = OSignatureLine, qname = "o:signatureline"),
            child(variant = OInk, qname = "o:ink"),
            child(variant = Diagram, qname = "o:diagram"),
            child(variant = Skew, qname = "o:skew"),
            child(variant = Extrusion, qname = "o:extrusion"),
            child(variant = Callout, qname = "o:callout"),
            child(variant = Lock, qname = "o:lock"),
            child(variant = OOleObject, qname = "o:OLEObject"),
            child(variant = Complex, qname = "o:complex"),
            child(variant = LeftStroke, qname = "o:left"),
            child(variant = TopStroke, qname = "o:top"),
            child(variant = RightStroke, qname = "o:right"),
            child(variant = BottomStroke, qname = "o:bottom"),
            child(variant = ColumnStroke, qname = "o:column"),
            child(variant = ClipPath, qname = "o:clippath"),
            child(variant = FillExtendedProperties, qname = "o:fill"),
            child(variant = TopBorder, qname = "w10:bordertop"),
            child(variant = LeftBorder, qname = "w10:borderleft"),
            child(variant = RightBorder, qname = "w10:borderright"),
            child(variant = BottomBorder, qname = "w10:borderbottom"),
            child(variant = TextWrap, qname = "w10:wrap"),
            empty_child(variant = AnchorLock, qname = "w10:anchorlock"),
            child(variant = ClientData, qname = "xvml:ClientData"),
            empty_child(variant = InkAnnotationFlag, qname = "pvml:iscomment"),
            child(variant = TextData, qname = "pvml:textdata"),
            child(variant = WordprocessingCanvas, qname = "wpc:wpc"),
            child(variant = WordprocessingGroup, qname = "wpg:wgp"),
            child(variant = WordprocessingShape, qname = "wps:wsp"),
            child(variant = Slicer, qname = "sle:slicer"),
            child(variant = ColorStyle, qname = "cs:colorStyle"),
            child(variant = ChartStyle, qname = "cs:chartStyle"),
            child(variant = WebExtension, qname = "we:webextension"),
            child(variant = WebExtensionReference, qname = "we:webextensionref"),
            child(variant = TimeSlicer, qname = "tsle:timeslicer"),
            any
        )
    )]
  pub graphic_data_choice: Vec<GraphicDataChoice>,
}
/// Diagram to Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:dgm")]
pub struct Diagram {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Animation Build Step
  #[sdk(attr(qname = ":bldStep"))]
  #[sdk(string_format(kind = "token"))]
  pub build_step: Option<DiagramBuildStepValues>,
}
/// Chart to Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:chart")]
pub struct Chart {
  /// Series Index
  #[sdk(attr(qname = ":seriesIdx"))]
  pub series_index: Option<crate::simple_type::Int32Value>,
  /// Category Index
  #[sdk(attr(qname = ":categoryIdx"))]
  pub category_index: Option<crate::simple_type::Int32Value>,
  /// Animation Build Step
  #[sdk(attr(qname = ":bldStep"))]
  #[sdk(string_format(kind = "token"))]
  pub build_step: ChartBuildStepValues,
}
/// Build Diagram.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bldDgm")]
pub struct BuildDiagram {
  /// Build
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["allAtOnce"]))]
  #[sdk(
        string_set(source = 1u32, union = 0u64, values = &["one", "lvlOne", "lvlAtOnce"])
    )]
  pub build: Option<crate::simple_type::StringValue>,
  /// Reverse Animation
  #[sdk(attr(qname = ":rev"))]
  pub reverse_animation: Option<crate::simple_type::BooleanValue>,
}
/// Build Chart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bldChart")]
pub struct BuildChart {
  /// Build
  #[sdk(attr(qname = ":bld"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["allAtOnce"]))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["series",
            "category",
            "seriesEl",
            "categoryEl"]
        )
    )]
  pub build: Option<crate::simple_type::StringValue>,
  /// Animate Background
  #[sdk(attr(qname = ":animBg"))]
  pub animate_background: Option<crate::simple_type::BooleanValue>,
}
/// Shape Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:txBody")]
pub struct TextBody {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<Paragraph>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:xfrm")]
pub struct Transform2D {
  pub xmlns: Vec<crate::common::XmlNamespace>,
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
  pub offset: Option<Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<Extents>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvPr")]
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
  pub hyperlink_on_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_drawing_properties_extension_list: Option<NonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<ShapeLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nvSpPr")]
pub struct NonVisualShapeProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Shape Drawing Properties
  #[sdk(child(qname = "a:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Visual Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, qname = "a:custGeom"),
            child(variant = PresetGeometry, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<ShapePropertiesExtensionList>,
}
/// Text Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:txSp")]
pub struct TextShape {
  /// Shape Text Body
  #[sdk(child(qname = "a:txBody"))]
  pub text_body: std::boxed::Box<TextBody>,
  #[sdk(
        choice(
            empty_child(variant = UseShapeRectangle, qname = "a:useSpRect"),
            child(variant = Transform2D, qname = "a:xfrm")
        )
    )]
  pub text_shape_choice: Option<TextShapeChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:style")]
pub struct ShapeStyle {
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Fill Reference.
  #[sdk(child(qname = "a:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Effect Reference.
  #[sdk(child(qname = "a:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:stCxn"))]
  pub start_connection: Option<StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:endCxn"))]
  pub end_connection: Option<EndConnection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the PictureLocks Class.
  #[sdk(child(qname = "a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<PictureLocks>>,
  /// Defines the NonVisualPicturePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_picture_properties_extension_list: Option<NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<GraphicFrameLocks>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
  /// Defines the GroupShapeLocks Class.
  #[sdk(child(qname = "a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<GroupShapeLocks>>,
  /// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Rotation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:rot")]
pub struct Rotation {
  /// Latitude
  #[sdk(attr(qname = ":lat"))]
  #[sdk(number_range(range = 0..21600000))]
  pub latitude: crate::simple_type::Int32Value,
  /// Longitude
  #[sdk(attr(qname = ":lon"))]
  #[sdk(number_range(range = 0..21600000))]
  pub longitude: crate::simple_type::Int32Value,
  /// Revolution
  #[sdk(attr(qname = ":rev"))]
  #[sdk(number_range(range = 0..21600000))]
  pub revolution: crate::simple_type::Int32Value,
}
/// Camera.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:camera")]
pub struct Camera {
  /// Preset Camera Type
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: PresetCameraValues,
  /// Field of View
  #[sdk(attr(qname = ":fov"))]
  #[sdk(number_range(range = 0..= 10800000))]
  pub field_of_view: Option<crate::simple_type::Int32Value>,
  /// Zoom
  #[sdk(attr(qname = ":zoom"))]
  #[sdk(number_range(range = 0..))]
  pub zoom: Option<crate::simple_type::PositiveDrawingmlPercentageValue>,
  /// Rotation
  #[sdk(child(qname = "a:rot"))]
  pub rotation: Option<Rotation>,
}
/// Light Rig.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lightRig")]
pub struct LightRig {
  /// Rig Preset
  #[sdk(attr(qname = ":rig"))]
  #[sdk(string_format(kind = "token"))]
  pub rig: LightRigValues,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(kind = "token"))]
  pub direction: LightRigDirectionValues,
  /// Rotation
  #[sdk(child(qname = "a:rot"))]
  pub rotation: Option<Rotation>,
}
/// Backdrop Plane.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:backdrop")]
pub struct Backdrop {
  /// Anchor Point
  #[sdk(child(qname = "a:anchor"))]
  pub anchor: std::boxed::Box<Anchor>,
  /// Normal
  #[sdk(child(qname = "a:norm"))]
  pub normal: std::boxed::Box<Normal>,
  /// Up Vector
  #[sdk(child(qname = "a:up"))]
  pub up_vector: std::boxed::Box<UpVector>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:anchor")]
pub struct Anchor {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: crate::simple_type::CoordinateValue,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: crate::simple_type::CoordinateValue,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub z: crate::simple_type::CoordinateValue,
}
/// Normal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:norm")]
pub struct Normal {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dx: crate::simple_type::CoordinateValue,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dy: crate::simple_type::CoordinateValue,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dz: crate::simple_type::CoordinateValue,
}
/// Up Vector.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:up")]
pub struct UpVector {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dx: crate::simple_type::CoordinateValue,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dy: crate::simple_type::CoordinateValue,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub dz: crate::simple_type::CoordinateValue,
}
/// Top Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bevelT")]
pub struct BevelTop {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub width: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub height: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Bottom Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bevelB")]
pub struct BevelBottom {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub width: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub height: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bevel")]
pub struct Bevel {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub width: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub height: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Fill To Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fillToRect")]
pub struct FillToRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::DrawingmlPercentageValue>,
}
/// Tile Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tileRect")]
pub struct TileRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::DrawingmlPercentageValue>,
}
/// Fill Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:fillRect")]
pub struct FillRectangle {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::DrawingmlPercentageValue>,
}
/// Source Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:srcRect")]
pub struct SourceRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::DrawingmlPercentageValue>,
}
/// Gradient stops.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gs")]
pub struct GradientStop {
  /// Position
  #[sdk(attr(qname = ":pos"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub position: crate::simple_type::PositiveFixedPercentageValue,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub gradient_stop_choice: Option<GradientStopChoice>,
}
/// Gradient Stop List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gsLst")]
pub struct GradientStopList {
  /// Gradient stops.
  #[sdk(child(qname = "a:gs"))]
  pub gradient_stop: Vec<GradientStop>,
}
/// Shape Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gd")]
pub struct ShapeGuide {
  /// Shape Guide Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(kind = "token"))]
  pub name: crate::simple_type::StringValue,
  /// Shape Guide Formula
  #[sdk(attr(qname = ":fmla"))]
  pub formula: crate::simple_type::StringValue,
}
/// Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pos")]
pub struct Position {
  /// X-Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub x: crate::simple_type::StringValue,
  /// Y-Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub y: crate::simple_type::StringValue,
}
/// Move end point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pt")]
pub struct Point {
  /// X-Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub x: crate::simple_type::StringValue,
  /// Y-Coordinate
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub y: crate::simple_type::StringValue,
}
/// XY Adjust Handle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ahXY")]
pub struct AdjustHandleXy {
  /// Horizontal Adjustment Guide
  #[sdk(attr(qname = ":gdRefX"))]
  #[sdk(string_format(kind = "token"))]
  pub x_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Horizontal Adjustment
  #[sdk(attr(qname = ":minX"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min_x: Option<crate::simple_type::StringValue>,
  /// Maximum Horizontal Adjustment
  #[sdk(attr(qname = ":maxX"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_x: Option<crate::simple_type::StringValue>,
  /// Vertical Adjustment Guide
  #[sdk(attr(qname = ":gdRefY"))]
  #[sdk(string_format(kind = "token"))]
  pub y_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Vertical Adjustment
  #[sdk(attr(qname = ":minY"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min_y: Option<crate::simple_type::StringValue>,
  /// Maximum Vertical Adjustment
  #[sdk(attr(qname = ":maxY"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_y: Option<crate::simple_type::StringValue>,
  /// Position
  #[sdk(child(qname = "a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Polar Adjust Handle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ahPolar")]
pub struct AdjustHandlePolar {
  /// Radial Adjustment Guide
  #[sdk(attr(qname = ":gdRefR"))]
  #[sdk(string_format(kind = "token"))]
  pub radial_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Radial Adjustment
  #[sdk(attr(qname = ":minR"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min_radial: Option<crate::simple_type::StringValue>,
  /// Maximum Radial Adjustment
  #[sdk(attr(qname = ":maxR"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_radial: Option<crate::simple_type::StringValue>,
  /// Angle Adjustment Guide
  #[sdk(attr(qname = ":gdRefAng"))]
  #[sdk(string_format(kind = "token"))]
  pub angle_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Angle Adjustment
  #[sdk(attr(qname = ":minAng"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min_angle: Option<crate::simple_type::StringValue>,
  /// Maximum Angle Adjustment
  #[sdk(attr(qname = ":maxAng"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_angle: Option<crate::simple_type::StringValue>,
  /// Shape Position Coordinate
  #[sdk(child(qname = "a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Shape Connection Site.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cxn")]
pub struct ConnectionSite {
  /// Connection Site Angle
  #[sdk(attr(qname = ":ang"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub angle: crate::simple_type::StringValue,
  /// Position
  #[sdk(child(qname = "a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Move Path To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:moveTo")]
pub struct MoveTo {
  /// Move end point
  #[sdk(child(qname = "a:pt"))]
  pub point: std::boxed::Box<Point>,
}
/// Draw Line To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnTo")]
pub struct LineTo {
  /// Line end point
  #[sdk(child(qname = "a:pt"))]
  pub point: std::boxed::Box<Point>,
}
/// Draw Arc To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:arcTo")]
pub struct ArcTo {
  /// Shape Arc Width Radius
  #[sdk(attr(qname = ":wR"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub width_radius: crate::simple_type::StringValue,
  /// Shape Arc Height Radius
  #[sdk(attr(qname = ":hR"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub height_radius: crate::simple_type::StringValue,
  /// Shape Arc Start Angle
  #[sdk(attr(qname = ":stAng"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub start_angle: crate::simple_type::StringValue,
  /// Shape Arc Swing Angle
  #[sdk(attr(qname = ":swAng"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub swing_angle: crate::simple_type::StringValue,
}
/// Draw Quadratic Bezier Curve To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:quadBezTo")]
pub struct QuadraticBezierCurveTo {
  /// Move end point.
  #[sdk(child(qname = "a:pt"))]
  pub point: Vec<Point>,
}
/// Draw Cubic Bezier Curve To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cubicBezTo")]
pub struct CubicBezierCurveTo {
  /// Move end point.
  #[sdk(child(qname = "a:pt"))]
  pub point: Vec<Point>,
}
/// Shape Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:path")]
pub struct Path {
  /// Path Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub width: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Path Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub height: Option<crate::simple_type::PositiveCoordinateValue>,
  /// Path Fill
  #[sdk(attr(qname = ":fill"))]
  #[sdk(string_format(kind = "token"))]
  pub fill: Option<PathFillModeValues>,
  /// Path Stroke
  #[sdk(attr(qname = ":stroke"))]
  pub stroke: Option<crate::simple_type::BooleanValue>,
  /// 3D Extrusion Allowed
  #[sdk(attr(qname = ":extrusionOk"))]
  pub extrusion_ok: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            empty_child(variant = CloseShapePath, qname = "a:close"),
            child(variant = MoveTo, qname = "a:moveTo"),
            child(variant = LineTo, qname = "a:lnTo"),
            child(variant = ArcTo, qname = "a:arcTo"),
            child(variant = QuadraticBezierCurveTo, qname = "a:quadBezTo"),
            child(variant = CubicBezierCurveTo, qname = "a:cubicBezTo")
        )
    )]
  pub path_choice: Vec<PathChoice>,
}
/// List of Shape Adjust Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:avLst")]
pub struct AdjustValueList {
  /// Shape Guide.
  #[sdk(child(qname = "a:gd"))]
  pub shape_guide: Vec<ShapeGuide>,
}
/// List of Shape Guides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gdLst")]
pub struct ShapeGuideList {
  /// Shape Guide.
  #[sdk(child(qname = "a:gd"))]
  pub shape_guide: Vec<ShapeGuide>,
}
/// List of Shape Adjust Handles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ahLst")]
pub struct AdjustHandleList {
  #[sdk(
        choice(
            child(variant = AdjustHandleXy, qname = "a:ahXY"),
            child(variant = AdjustHandlePolar, qname = "a:ahPolar")
        )
    )]
  pub adjust_handle_list_choice: Vec<AdjustHandleListChoice>,
}
/// List of Shape Connection Sites.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cxnLst")]
pub struct ConnectionSiteList {
  /// Shape Connection Site.
  #[sdk(child(qname = "a:cxn"))]
  pub connection_site: Vec<ConnectionSite>,
}
/// Shape Text Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:rect")]
pub struct Rectangle {
  /// Left
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub left: crate::simple_type::StringValue,
  /// Top
  #[sdk(attr(qname = ":t"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub top: crate::simple_type::StringValue,
  /// Right
  #[sdk(attr(qname = ":r"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub right: crate::simple_type::StringValue,
  /// Bottom Position
  #[sdk(attr(qname = ":b"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub bottom: crate::simple_type::StringValue,
}
/// List of Shape Paths.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pathLst")]
pub struct PathList {
  /// Shape Path.
  #[sdk(child(qname = "a:path"))]
  pub path: Vec<Path>,
}
/// Dash Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ds")]
pub struct DashStop {
  /// Dash Length
  #[sdk(attr(qname = ":d"))]
  #[sdk(number_range(range = 0..))]
  pub dash_length: crate::simple_type::PositiveDrawingmlPercentageValue,
  /// Space Length
  #[sdk(attr(qname = ":sp"))]
  #[sdk(number_range(range = 0..))]
  pub space_length: crate::simple_type::PositiveDrawingmlPercentageValue,
}
/// 2D Transform for Grouped Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:xfrm")]
pub struct TransformGroup {
  pub xmlns: Vec<crate::common::XmlNamespace>,
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
  pub offset: Option<Offset>,
  /// Extents
  #[sdk(child(qname = "a:ext"))]
  pub extents: Option<Extents>,
  /// Child Offset
  #[sdk(child(qname = "a:chOff"))]
  pub child_offset: Option<ChildOffset>,
  /// Child Extents
  #[sdk(child(qname = "a:chExt"))]
  pub child_extents: Option<ChildExtents>,
}
/// Defines the BodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bodyPr")]
pub struct BodyProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_overflow: Option<TextVerticalOverflowValues>,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical: Option<TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(kind = "token"))]
  pub wrap: Option<TextWrappingValues>,
  /// Left Inset
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Coordinate32Value>,
  /// Top Inset
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Coordinate32Value>,
  /// Right Inset
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Coordinate32Value>,
  /// Bottom Inset
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Coordinate32Value>,
  /// Number of Columns
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(range = 1..= 16))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(range = 0..))]
  pub column_spacing: Option<crate::simple_type::PositiveCoordinate32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(kind = "token"))]
  pub anchor: Option<TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  /// Preset Text Shape
  #[sdk(child(qname = "a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<PresetTextWarp>>,
  #[sdk(
        choice(
            empty_child(variant = NoAutoFit, qname = "a:noAutofit"),
            child(variant = NormalAutoFit, qname = "a:normAutofit"),
            empty_child(variant = ShapeAutoFit, qname = "a:spAutoFit")
        )
    )]
  pub body_properties_choice1: Option<BodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<Scene3DType>>,
  #[sdk(
        choice(
            child(variant = Shape3DType, qname = "a:sp3d"),
            child(variant = FlatText, qname = "a:flatTx")
        )
    )]
  pub body_properties_choice2: Option<BodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ListStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lstStyle")]
pub struct ListStyle {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:defPPr"))]
  pub default_paragraph_properties: Option<std::boxed::Box<DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<std::boxed::Box<Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<std::boxed::Box<Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<std::boxed::Box<Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<std::boxed::Box<Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<std::boxed::Box<Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<std::boxed::Box<Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<std::boxed::Box<Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<std::boxed::Box<Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<std::boxed::Box<Level9ParagraphProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Shape Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spDef")]
pub struct ShapeDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the BodyProperties Class.
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// Defines the ListStyle Class.
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// Style.
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Line Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnDef")]
pub struct LineDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the BodyProperties Class.
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// Defines the ListStyle Class.
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// Style.
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Text Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:txDef")]
pub struct TextDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Defines the BodyProperties Class.
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// Defines the ListStyle Class.
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// Style.
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Override Color Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:overrideClrMapping")]
pub struct OverrideColorMapping {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(kind = "token"))]
  pub background1: ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(kind = "token"))]
  pub text1: ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(kind = "token"))]
  pub background2: ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(kind = "token"))]
  pub text2: ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(kind = "token"))]
  pub accent1: ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(kind = "token"))]
  pub accent2: ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(kind = "token"))]
  pub accent3: ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(kind = "token"))]
  pub accent4: ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(kind = "token"))]
  pub accent5: ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(kind = "token"))]
  pub accent6: ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(kind = "token"))]
  pub hyperlink: ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(kind = "token"))]
  pub followed_hyperlink: ColorSchemeIndexValues,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:clrMap")]
pub struct ColorMap {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(kind = "token"))]
  pub background1: ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(kind = "token"))]
  pub text1: ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(kind = "token"))]
  pub background2: ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(kind = "token"))]
  pub text2: ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(kind = "token"))]
  pub accent1: ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(kind = "token"))]
  pub accent2: ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(kind = "token"))]
  pub accent3: ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(kind = "token"))]
  pub accent4: ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(kind = "token"))]
  pub accent5: ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(kind = "token"))]
  pub accent6: ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(kind = "token"))]
  pub hyperlink: ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(kind = "token"))]
  pub followed_hyperlink: ColorSchemeIndexValues,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Extra Color Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extraClrScheme")]
pub struct ExtraColorScheme {
  /// Defines the ColorScheme Class.
  #[sdk(child(qname = "a:clrScheme"))]
  pub color_scheme: std::boxed::Box<ColorScheme>,
  /// Defines the ColorMap Class.
  #[sdk(child(qname = "a:clrMap"))]
  pub color_map: Option<std::boxed::Box<ColorMap>>,
}
/// Defines the ThemeElements Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:themeElements")]
pub struct ThemeElements {
  /// Defines the ColorScheme Class.
  #[sdk(child(qname = "a:clrScheme"))]
  pub color_scheme: std::boxed::Box<ColorScheme>,
  /// Font Scheme
  #[sdk(child(qname = "a:fontScheme"))]
  pub font_scheme: std::boxed::Box<FontScheme>,
  /// Format Scheme
  #[sdk(child(qname = "a:fmtScheme"))]
  pub format_scheme: std::boxed::Box<FormatScheme>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell 3-D.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cell3D")]
pub struct Cell3DProperties {
  /// Preset Material
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(kind = "token"))]
  pub preset_material: Option<PresetMaterialTypeValues>,
  /// Bevel
  #[sdk(child(qname = "a:bevel"))]
  pub bevel: std::boxed::Box<Bevel>,
  /// Light Rig
  #[sdk(child(qname = "a:lightRig"))]
  pub light_rig: Option<std::boxed::Box<LightRig>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tcPr")]
pub struct TableCellProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  pub left_margin: Option<crate::simple_type::Coordinate32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  pub right_margin: Option<crate::simple_type::Coordinate32Value>,
  /// Top Margin
  #[sdk(attr(qname = ":marT"))]
  pub top_margin: Option<crate::simple_type::Coordinate32Value>,
  /// Bottom Margin
  #[sdk(attr(qname = ":marB"))]
  pub bottom_margin: Option<crate::simple_type::Coordinate32Value>,
  /// Text Direction
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical: Option<TextVerticalValues>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(kind = "token"))]
  pub anchor: Option<TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
  /// Left Border Line Properties
  #[sdk(child(qname = "a:lnL"))]
  pub left_border_line_properties: Option<std::boxed::Box<LeftBorderLineProperties>>,
  /// Right Border Line Properties
  #[sdk(child(qname = "a:lnR"))]
  pub right_border_line_properties: Option<std::boxed::Box<RightBorderLineProperties>>,
  /// Top Border Line Properties
  #[sdk(child(qname = "a:lnT"))]
  pub top_border_line_properties: Option<std::boxed::Box<TopBorderLineProperties>>,
  /// Bottom Border Line Properties
  #[sdk(child(qname = "a:lnB"))]
  pub bottom_border_line_properties: Option<std::boxed::Box<BottomBorderLineProperties>>,
  /// Top-Left to Bottom-Right Border Line Properties
  #[sdk(child(qname = "a:lnTlToBr"))]
  pub top_left_to_bottom_right_border_line_properties:
    Option<std::boxed::Box<TopLeftToBottomRightBorderLineProperties>>,
  /// Bottom-Left to Top-Right Border Line Properties
  #[sdk(child(qname = "a:lnBlToTr"))]
  pub bottom_left_to_top_right_border_line_properties:
    Option<std::boxed::Box<BottomLeftToTopRightBorderLineProperties>>,
  /// Cell 3-D
  #[sdk(child(qname = "a:cell3D"))]
  pub cell3_d_properties: Option<std::boxed::Box<Cell3DProperties>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub table_cell_properties_choice: Option<TableCellPropertiesChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tc")]
pub struct TableCell {
  /// Row Span
  #[sdk(attr(qname = ":rowSpan"))]
  pub row_span: Option<crate::simple_type::Int32Value>,
  /// Grid Span
  #[sdk(attr(qname = ":gridSpan"))]
  pub grid_span: Option<crate::simple_type::Int32Value>,
  /// Horizontal Merge
  #[sdk(attr(qname = ":hMerge"))]
  pub horizontal_merge: Option<crate::simple_type::BooleanValue>,
  /// Vertical Merge
  #[sdk(attr(qname = ":vMerge"))]
  pub vertical_merge: Option<crate::simple_type::BooleanValue>,
  /// Text Body
  #[sdk(child(qname = "a:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// Table Cell Properties
  #[sdk(child(qname = "a:tcPr"))]
  pub table_cell_properties: Option<std::boxed::Box<TableCellProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tableStyle")]
pub struct TableStyle {
  /// Style ID
  #[sdk(attr(qname = ":styleId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub style_id: crate::simple_type::StringValue,
  /// Name
  #[sdk(attr(qname = ":styleName"))]
  pub style_name: crate::simple_type::StringValue,
  /// Table Background
  #[sdk(child(qname = "a:tblBg"))]
  pub table_background: Option<std::boxed::Box<TableBackground>>,
  /// Whole Table
  #[sdk(child(qname = "a:wholeTbl"))]
  pub whole_table: Option<std::boxed::Box<WholeTable>>,
  /// Band 1 Horizontal
  #[sdk(child(qname = "a:band1H"))]
  pub band1_horizontal: Option<std::boxed::Box<Band1Horizontal>>,
  /// Band 2 Horizontal
  #[sdk(child(qname = "a:band2H"))]
  pub band2_horizontal: Option<std::boxed::Box<Band2Horizontal>>,
  /// Band 1 Vertical
  #[sdk(child(qname = "a:band1V"))]
  pub band1_vertical: Option<std::boxed::Box<Band1Vertical>>,
  /// Band 2 Vertical
  #[sdk(child(qname = "a:band2V"))]
  pub band2_vertical: Option<std::boxed::Box<Band2Vertical>>,
  /// Last Column
  #[sdk(child(qname = "a:lastCol"))]
  pub last_column: Option<std::boxed::Box<LastColumn>>,
  /// First Column
  #[sdk(child(qname = "a:firstCol"))]
  pub first_column: Option<std::boxed::Box<FirstColumn>>,
  /// Last Row
  #[sdk(child(qname = "a:lastRow"))]
  pub last_row: Option<std::boxed::Box<LastRow>>,
  /// Southeast Cell
  #[sdk(child(qname = "a:seCell"))]
  pub southeast_cell: Option<std::boxed::Box<SoutheastCell>>,
  /// Southwest Cell
  #[sdk(child(qname = "a:swCell"))]
  pub southwest_cell: Option<std::boxed::Box<SouthwestCell>>,
  /// First Row
  #[sdk(child(qname = "a:firstRow"))]
  pub first_row: Option<std::boxed::Box<FirstRow>>,
  /// Northeast Cell
  #[sdk(child(qname = "a:neCell"))]
  pub northeast_cell: Option<std::boxed::Box<NortheastCell>>,
  /// Northwest Cell
  #[sdk(child(qname = "a:nwCell"))]
  pub northwest_cell: Option<std::boxed::Box<NorthwestCell>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tblStyle")]
pub struct TableStyleEntry {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Style ID
  #[sdk(attr(qname = ":styleId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub style_id: crate::simple_type::StringValue,
  /// Name
  #[sdk(attr(qname = ":styleName"))]
  pub style_name: crate::simple_type::StringValue,
  /// Table Background
  #[sdk(child(qname = "a:tblBg"))]
  pub table_background: Option<std::boxed::Box<TableBackground>>,
  /// Whole Table
  #[sdk(child(qname = "a:wholeTbl"))]
  pub whole_table: Option<std::boxed::Box<WholeTable>>,
  /// Band 1 Horizontal
  #[sdk(child(qname = "a:band1H"))]
  pub band1_horizontal: Option<std::boxed::Box<Band1Horizontal>>,
  /// Band 2 Horizontal
  #[sdk(child(qname = "a:band2H"))]
  pub band2_horizontal: Option<std::boxed::Box<Band2Horizontal>>,
  /// Band 1 Vertical
  #[sdk(child(qname = "a:band1V"))]
  pub band1_vertical: Option<std::boxed::Box<Band1Vertical>>,
  /// Band 2 Vertical
  #[sdk(child(qname = "a:band2V"))]
  pub band2_vertical: Option<std::boxed::Box<Band2Vertical>>,
  /// Last Column
  #[sdk(child(qname = "a:lastCol"))]
  pub last_column: Option<std::boxed::Box<LastColumn>>,
  /// First Column
  #[sdk(child(qname = "a:firstCol"))]
  pub first_column: Option<std::boxed::Box<FirstColumn>>,
  /// Last Row
  #[sdk(child(qname = "a:lastRow"))]
  pub last_row: Option<std::boxed::Box<LastRow>>,
  /// Southeast Cell
  #[sdk(child(qname = "a:seCell"))]
  pub southeast_cell: Option<std::boxed::Box<SoutheastCell>>,
  /// Southwest Cell
  #[sdk(child(qname = "a:swCell"))]
  pub southwest_cell: Option<std::boxed::Box<SouthwestCell>>,
  /// First Row
  #[sdk(child(qname = "a:firstRow"))]
  pub first_row: Option<std::boxed::Box<FirstRow>>,
  /// Northeast Cell
  #[sdk(child(qname = "a:neCell"))]
  pub northeast_cell: Option<std::boxed::Box<NortheastCell>>,
  /// Northwest Cell
  #[sdk(child(qname = "a:nwCell"))]
  pub northwest_cell: Option<std::boxed::Box<NorthwestCell>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style ID.
pub type TableStyleId = crate::simple_type::StringValue;
/// Table Grid Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:gridCol")]
pub struct GridColumn {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub width: crate::simple_type::CoordinateValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tblPr")]
pub struct TableProperties {
  /// Right-to-Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// First Row
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::BooleanValue>,
  /// First Column
  #[sdk(attr(qname = ":firstCol"))]
  pub first_column: Option<crate::simple_type::BooleanValue>,
  /// Last Row
  #[sdk(attr(qname = ":lastRow"))]
  pub last_row: Option<crate::simple_type::BooleanValue>,
  /// Last Column
  #[sdk(attr(qname = ":lastCol"))]
  pub last_column: Option<crate::simple_type::BooleanValue>,
  /// Banded Rows
  #[sdk(attr(qname = ":bandRow"))]
  pub band_row: Option<crate::simple_type::BooleanValue>,
  /// Banded Columns
  #[sdk(attr(qname = ":bandCol"))]
  pub band_column: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub table_properties_choice1: Option<TablePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub table_properties_choice2: Option<TablePropertiesChoice2>,
  #[sdk(
        choice(
            child(variant = TableStyle, qname = "a:tableStyle"),
            text_child(variant = TableStyleId, qname = "a:tableStyleId")
        )
    )]
  pub table_properties_choice3: Option<TablePropertiesChoice3>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tblGrid")]
pub struct TableGrid {
  /// Table Grid Column.
  #[sdk(child(qname = "a:gridCol"))]
  pub grid_column: Vec<GridColumn>,
}
/// Table Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tr")]
pub struct TableRow {
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub height: crate::simple_type::CoordinateValue,
  /// Table Cell.
  #[sdk(child(qname = "a:tc"))]
  pub table_cell: Vec<TableCell>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Left Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:left")]
pub struct LeftBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub left_border_choice: Option<LeftBorderChoice>,
}
/// Right Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:right")]
pub struct RightBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub right_border_choice: Option<RightBorderChoice>,
}
/// Top Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:top")]
pub struct TopBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub top_border_choice: Option<TopBorderChoice>,
}
/// Bottom Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:bottom")]
pub struct BottomBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub bottom_border_choice: Option<BottomBorderChoice>,
}
/// Inside Horizontal Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:insideH")]
pub struct InsideHorizontalBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub inside_horizontal_border_choice: Option<InsideHorizontalBorderChoice>,
}
/// Inside Vertical Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:insideV")]
pub struct InsideVerticalBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub inside_vertical_border_choice: Option<InsideVerticalBorderChoice>,
}
/// Top Left to Bottom Right Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tl2br")]
pub struct TopLeftToBottomRightBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub top_left_to_bottom_right_border_choice: Option<TopLeftToBottomRightBorderChoice>,
}
/// Top Right to Bottom Left Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tr2bl")]
pub struct TopRightToBottomLeftBorder {
  #[sdk(
        choice(
            child(variant = Outline, qname = "a:ln"),
            child(variant = LineReference, qname = "a:lnRef")
        )
    )]
  pub top_right_to_bottom_left_border_choice: Option<TopRightToBottomLeftBorderChoice>,
}
/// Table Cell Borders.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tcBdr")]
pub struct TableCellBorders {
  /// Left Border
  #[sdk(child(qname = "a:left"))]
  pub left_border: Option<std::boxed::Box<LeftBorder>>,
  /// Right Border
  #[sdk(child(qname = "a:right"))]
  pub right_border: Option<std::boxed::Box<RightBorder>>,
  /// Top Border
  #[sdk(child(qname = "a:top"))]
  pub top_border: Option<std::boxed::Box<TopBorder>>,
  /// Bottom Border
  #[sdk(child(qname = "a:bottom"))]
  pub bottom_border: Option<std::boxed::Box<BottomBorder>>,
  /// Inside Horizontal Border
  #[sdk(child(qname = "a:insideH"))]
  pub inside_horizontal_border: Option<std::boxed::Box<InsideHorizontalBorder>>,
  /// Inside Vertical Border
  #[sdk(child(qname = "a:insideV"))]
  pub inside_vertical_border: Option<std::boxed::Box<InsideVerticalBorder>>,
  /// Top Left to Bottom Right Border
  #[sdk(child(qname = "a:tl2br"))]
  pub top_left_to_bottom_right_border: Option<std::boxed::Box<TopLeftToBottomRightBorder>>,
  /// Top Right to Bottom Left Border
  #[sdk(child(qname = "a:tr2bl"))]
  pub top_right_to_bottom_left_border: Option<std::boxed::Box<TopRightToBottomLeftBorder>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tcTxStyle")]
pub struct TableCellTextStyle {
  /// Bold
  #[sdk(attr(qname = ":b"))]
  #[sdk(string_format(kind = "token"))]
  pub bold: Option<BooleanStyleValues>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  #[sdk(string_format(kind = "token"))]
  pub italic: Option<BooleanStyleValues>,
  #[sdk(
        choice(
            child(variant = Fonts, qname = "a:font"),
            child(variant = FontReference, qname = "a:fontRef")
        )
    )]
  pub table_cell_text_style_choice1: Option<TableCellTextStyleChoice>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub table_cell_text_style_choice2: Option<TableCellTextStyleChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tcStyle")]
pub struct TableCellStyle {
  /// Table Cell Borders
  #[sdk(child(qname = "a:tcBdr"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  #[sdk(
        choice(
            child(variant = FillProperties, qname = "a:fill"),
            child(variant = FillReference, qname = "a:fillRef")
        )
    )]
  pub table_cell_style_choice: Option<TableCellStyleChoice>,
  /// Cell 3-D.
  #[sdk(child(qname = "a:cell3D"))]
  pub cell3_d_properties: Option<std::boxed::Box<Cell3DProperties>>,
}
/// Table Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tblBg")]
pub struct TableBackground {
  #[sdk(
        choice(
            child(variant = FillProperties, qname = "a:fill"),
            child(variant = FillReference, qname = "a:fillRef")
        )
    )]
  pub table_background_choice1: Option<TableBackgroundChoice>,
  #[sdk(
        choice(
            child(variant = EffectPropertiesType, qname = "a:effect"),
            child(variant = EffectReference, qname = "a:effectRef")
        )
    )]
  pub table_background_choice2: Option<TableBackgroundChoice2>,
}
/// Whole Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:wholeTbl")]
pub struct WholeTable {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 1 Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:band1H")]
pub struct Band1Horizontal {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 2 Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:band2H")]
pub struct Band2Horizontal {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 1 Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:band1V")]
pub struct Band1Vertical {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 2 Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:band2V")]
pub struct Band2Vertical {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Last Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lastCol")]
pub struct LastColumn {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// First Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:firstCol")]
pub struct FirstColumn {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Last Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lastRow")]
pub struct LastRow {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Southeast Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:seCell")]
pub struct SoutheastCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Southwest Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:swCell")]
pub struct SouthwestCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// First Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:firstRow")]
pub struct FirstRow {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Northeast Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:neCell")]
pub struct NortheastCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Northwest Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nwCell")]
pub struct NorthwestCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Text Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pPr")]
pub struct ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub paragraph_properties_choice1: Option<ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub paragraph_properties_choice2: Option<ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub paragraph_properties_choice3: Option<ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub paragraph_properties_choice4: Option<ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Default Paragraph Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:defPPr")]
pub struct DefaultParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub default_paragraph_properties_choice1: Option<DefaultParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub default_paragraph_properties_choice2: Option<DefaultParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub default_paragraph_properties_choice3: Option<DefaultParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub default_paragraph_properties_choice4: Option<DefaultParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 1 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl1pPr")]
pub struct Level1ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level1_paragraph_properties_choice1: Option<Level1ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level1_paragraph_properties_choice2: Option<Level1ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level1_paragraph_properties_choice3: Option<Level1ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level1_paragraph_properties_choice4: Option<Level1ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 2 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl2pPr")]
pub struct Level2ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level2_paragraph_properties_choice1: Option<Level2ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level2_paragraph_properties_choice2: Option<Level2ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level2_paragraph_properties_choice3: Option<Level2ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level2_paragraph_properties_choice4: Option<Level2ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 3 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl3pPr")]
pub struct Level3ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level3_paragraph_properties_choice1: Option<Level3ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level3_paragraph_properties_choice2: Option<Level3ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level3_paragraph_properties_choice3: Option<Level3ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level3_paragraph_properties_choice4: Option<Level3ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 4 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl4pPr")]
pub struct Level4ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level4_paragraph_properties_choice1: Option<Level4ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level4_paragraph_properties_choice2: Option<Level4ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level4_paragraph_properties_choice3: Option<Level4ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level4_paragraph_properties_choice4: Option<Level4ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 5 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl5pPr")]
pub struct Level5ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level5_paragraph_properties_choice1: Option<Level5ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level5_paragraph_properties_choice2: Option<Level5ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level5_paragraph_properties_choice3: Option<Level5ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level5_paragraph_properties_choice4: Option<Level5ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 6 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl6pPr")]
pub struct Level6ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level6_paragraph_properties_choice1: Option<Level6ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level6_paragraph_properties_choice2: Option<Level6ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level6_paragraph_properties_choice3: Option<Level6ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level6_paragraph_properties_choice4: Option<Level6ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 7 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl7pPr")]
pub struct Level7ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level7_paragraph_properties_choice1: Option<Level7ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level7_paragraph_properties_choice2: Option<Level7ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level7_paragraph_properties_choice3: Option<Level7ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level7_paragraph_properties_choice4: Option<Level7ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 8 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl8pPr")]
pub struct Level8ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level8_paragraph_properties_choice1: Option<Level8ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level8_paragraph_properties_choice2: Option<Level8ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level8_paragraph_properties_choice3: Option<Level8ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level8_paragraph_properties_choice4: Option<Level8ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// List Level 9 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lvl9pPr")]
pub struct Level9ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(range = 0..= 51206400))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(range = 0..= 8))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(range = -51206400..= 51206400))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Coordinate32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(
        choice(
            empty_child(variant = BulletColorText, qname = "a:buClrTx"),
            child(variant = BulletColor, qname = "a:buClr")
        )
    )]
  pub level9_paragraph_properties_choice1: Option<Level9ParagraphPropertiesChoice>,
  #[sdk(
        choice(
            empty_child(variant = BulletSizeText, qname = "a:buSzTx"),
            child(variant = BulletSizePercentage, qname = "a:buSzPct"),
            child(variant = BulletSizePoints, qname = "a:buSzPts")
        )
    )]
  pub level9_paragraph_properties_choice2: Option<Level9ParagraphPropertiesChoice2>,
  #[sdk(
        choice(
            empty_child(variant = BulletFontText, qname = "a:buFontTx"),
            child(variant = BulletFont, qname = "a:buFont")
        )
    )]
  pub level9_paragraph_properties_choice3: Option<Level9ParagraphPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = NoBullet, qname = "a:buNone"),
            child(variant = AutoNumberedBullet, qname = "a:buAutoNum"),
            child(variant = CharacterBullet, qname = "a:buChar"),
            child(variant = PictureBullet, qname = "a:buBlip")
        )
    )]
  pub level9_paragraph_properties_choice4: Option<Level9ParagraphPropertiesChoice4>,
  /// Tab List.
  #[sdk(child(qname = "a:tabLst"))]
  pub tab_stop_list: Option<TabStopList>,
  /// Default Text Run Properties.
  #[sdk(child(qname = "a:defRPr"))]
  pub default_run_properties: Option<std::boxed::Box<DefaultRunProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// End Paragraph Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:endParaRPr")]
pub struct EndParagraphRunProperties {
  /// kumimoji
  #[sdk(attr(qname = ":kumimoji"))]
  pub kumimoji: Option<crate::simple_type::BooleanValue>,
  /// lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// altLang
  #[sdk(attr(qname = ":altLang"))]
  pub alternative_language: Option<crate::simple_type::StringValue>,
  /// sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 100..= 400000))]
  pub font_size: Option<crate::simple_type::TextFontSizeValue>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(range = 0..= 400000))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(range = -400000..= 400000))]
  pub spacing: Option<crate::simple_type::TextPointValue>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// noProof
  #[sdk(attr(qname = ":noProof"))]
  pub no_proof: Option<crate::simple_type::BooleanValue>,
  /// dirty
  #[sdk(attr(qname = ":dirty"))]
  pub dirty: Option<crate::simple_type::BooleanValue>,
  /// err
  #[sdk(attr(qname = ":err"))]
  pub spelling_error: Option<crate::simple_type::BooleanValue>,
  /// smtClean
  #[sdk(attr(qname = ":smtClean"))]
  pub smart_tag_clean: Option<crate::simple_type::BooleanValue>,
  /// smtId
  #[sdk(attr(qname = ":smtId"))]
  pub smart_tag_id: Option<crate::simple_type::UInt32Value>,
  /// bmk
  #[sdk(attr(qname = ":bmk"))]
  pub bookmark: Option<crate::simple_type::StringValue>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub end_paragraph_run_properties_choice1: Option<EndParagraphRunPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub end_paragraph_run_properties_choice2: Option<EndParagraphRunPropertiesChoice2>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "a:highlight"))]
  pub highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFollowsText, qname = "a:uLnTx"),
            child(variant = Underline, qname = "a:uLn")
        )
    )]
  pub end_paragraph_run_properties_choice3: Option<EndParagraphRunPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFillText, qname = "a:uFillTx"),
            child(variant = UnderlineFill, qname = "a:uFill")
        )
    )]
  pub end_paragraph_run_properties_choice4: Option<EndParagraphRunPropertiesChoice4>,
  /// Latin Font.
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: Option<LatinFont>,
  /// East Asian Font.
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: Option<EastAsianFont>,
  /// Complex Script Font.
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: Option<ComplexScriptFont>,
  /// Defines the SymbolFont Class.
  #[sdk(child(qname = "a:sym"))]
  pub symbol_font: Option<SymbolFont>,
  /// Defines the HyperlinkOnClick Class.
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// Defines the HyperlinkOnMouseOver Class.
  #[sdk(child(qname = "a:hlinkMouseOver"))]
  pub hyperlink_on_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// Defines the RightToLeft Class.
  #[sdk(child(qname = "a:rtl"))]
  pub right_to_left: Option<RightToLeft>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Text Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:rPr")]
pub struct RunProperties {
  /// kumimoji
  #[sdk(attr(qname = ":kumimoji"))]
  pub kumimoji: Option<crate::simple_type::BooleanValue>,
  /// lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// altLang
  #[sdk(attr(qname = ":altLang"))]
  pub alternative_language: Option<crate::simple_type::StringValue>,
  /// sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 100..= 400000))]
  pub font_size: Option<crate::simple_type::TextFontSizeValue>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(range = 0..= 400000))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(range = -400000..= 400000))]
  pub spacing: Option<crate::simple_type::TextPointValue>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// noProof
  #[sdk(attr(qname = ":noProof"))]
  pub no_proof: Option<crate::simple_type::BooleanValue>,
  /// dirty
  #[sdk(attr(qname = ":dirty"))]
  pub dirty: Option<crate::simple_type::BooleanValue>,
  /// err
  #[sdk(attr(qname = ":err"))]
  pub spelling_error: Option<crate::simple_type::BooleanValue>,
  /// smtClean
  #[sdk(attr(qname = ":smtClean"))]
  pub smart_tag_clean: Option<crate::simple_type::BooleanValue>,
  /// smtId
  #[sdk(attr(qname = ":smtId"))]
  pub smart_tag_id: Option<crate::simple_type::UInt32Value>,
  /// bmk
  #[sdk(attr(qname = ":bmk"))]
  pub bookmark: Option<crate::simple_type::StringValue>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub run_properties_choice1: Option<RunPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub run_properties_choice2: Option<RunPropertiesChoice2>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "a:highlight"))]
  pub highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFollowsText, qname = "a:uLnTx"),
            child(variant = Underline, qname = "a:uLn")
        )
    )]
  pub run_properties_choice3: Option<RunPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFillText, qname = "a:uFillTx"),
            child(variant = UnderlineFill, qname = "a:uFill")
        )
    )]
  pub run_properties_choice4: Option<RunPropertiesChoice4>,
  /// Latin Font.
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: Option<LatinFont>,
  /// East Asian Font.
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: Option<EastAsianFont>,
  /// Complex Script Font.
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: Option<ComplexScriptFont>,
  /// Defines the SymbolFont Class.
  #[sdk(child(qname = "a:sym"))]
  pub symbol_font: Option<SymbolFont>,
  /// Defines the HyperlinkOnClick Class.
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// Defines the HyperlinkOnMouseOver Class.
  #[sdk(child(qname = "a:hlinkMouseOver"))]
  pub hyperlink_on_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// Defines the RightToLeft Class.
  #[sdk(child(qname = "a:rtl"))]
  pub right_to_left: Option<RightToLeft>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Default Text Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:defRPr")]
pub struct DefaultRunProperties {
  /// kumimoji
  #[sdk(attr(qname = ":kumimoji"))]
  pub kumimoji: Option<crate::simple_type::BooleanValue>,
  /// lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// altLang
  #[sdk(attr(qname = ":altLang"))]
  pub alternative_language: Option<crate::simple_type::StringValue>,
  /// sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(range = 100..= 400000))]
  pub font_size: Option<crate::simple_type::TextFontSizeValue>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(range = 0..= 400000))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(range = -400000..= 400000))]
  pub spacing: Option<crate::simple_type::TextPointValue>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::DrawingmlPercentageValue>,
  /// noProof
  #[sdk(attr(qname = ":noProof"))]
  pub no_proof: Option<crate::simple_type::BooleanValue>,
  /// dirty
  #[sdk(attr(qname = ":dirty"))]
  pub dirty: Option<crate::simple_type::BooleanValue>,
  /// err
  #[sdk(attr(qname = ":err"))]
  pub spelling_error: Option<crate::simple_type::BooleanValue>,
  /// smtClean
  #[sdk(attr(qname = ":smtClean"))]
  pub smart_tag_clean: Option<crate::simple_type::BooleanValue>,
  /// smtId
  #[sdk(attr(qname = ":smtId"))]
  pub smart_tag_id: Option<crate::simple_type::UInt32Value>,
  /// bmk
  #[sdk(attr(qname = ":bmk"))]
  pub bookmark: Option<crate::simple_type::StringValue>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub default_run_properties_choice1: Option<DefaultRunPropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub default_run_properties_choice2: Option<DefaultRunPropertiesChoice2>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "a:highlight"))]
  pub highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFollowsText, qname = "a:uLnTx"),
            child(variant = Underline, qname = "a:uLn")
        )
    )]
  pub default_run_properties_choice3: Option<DefaultRunPropertiesChoice3>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFillText, qname = "a:uFillTx"),
            child(variant = UnderlineFill, qname = "a:uFill")
        )
    )]
  pub default_run_properties_choice4: Option<DefaultRunPropertiesChoice4>,
  /// Latin Font.
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: Option<LatinFont>,
  /// East Asian Font.
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: Option<EastAsianFont>,
  /// Complex Script Font.
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: Option<ComplexScriptFont>,
  /// Defines the SymbolFont Class.
  #[sdk(child(qname = "a:sym"))]
  pub symbol_font: Option<SymbolFont>,
  /// Defines the HyperlinkOnClick Class.
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// Defines the HyperlinkOnMouseOver Class.
  #[sdk(child(qname = "a:hlinkMouseOver"))]
  pub hyperlink_on_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// Defines the RightToLeft Class.
  #[sdk(child(qname = "a:rtl"))]
  pub right_to_left: Option<RightToLeft>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Text Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:p")]
pub struct Paragraph {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Text Paragraph Properties
  #[sdk(child(qname = "a:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(
        choice(
            child(variant = Run, qname = "a:r"),
            child(variant = Break, qname = "a:br"),
            child(variant = Field, qname = "a:fld"),
            child(variant = TextMath, qname = "a14:m")
        )
    )]
  pub paragraph_choice: Vec<ParagraphChoice>,
  /// End Paragraph Run Properties.
  #[sdk(child(qname = "a:endParaRPr"))]
  pub end_paragraph_run_properties: Option<std::boxed::Box<EndParagraphRunProperties>>,
}
/// Tab Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tab")]
pub struct TabStop {
  /// Tab Position
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Coordinate32Value>,
  /// Tab Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(kind = "token"))]
  pub alignment: Option<TextTabAlignmentValues>,
}
/// Spacing Percent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spcPct")]
pub struct SpacingPercent {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 13200000))]
  pub val: crate::simple_type::TextSpacingPercentOrPercentStringValue,
}
/// Spacing Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spcPts")]
pub struct SpacingPoints {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 158400))]
  pub val: crate::simple_type::TextSpacingPointValue,
}
/// Line Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:lnSpc")]
pub struct LineSpacing {
  #[sdk(
        choice(
            child(variant = SpacingPercent, qname = "a:spcPct"),
            child(variant = SpacingPoints, qname = "a:spcPts")
        )
    )]
  pub line_spacing_choice: Option<LineSpacingChoice>,
}
/// Space Before.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spcBef")]
pub struct SpaceBefore {
  #[sdk(
        choice(
            child(variant = SpacingPercent, qname = "a:spcPct"),
            child(variant = SpacingPoints, qname = "a:spcPts")
        )
    )]
  pub space_before_choice: Option<SpaceBeforeChoice>,
}
/// Space After.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:spcAft")]
pub struct SpaceAfter {
  #[sdk(
        choice(
            child(variant = SpacingPercent, qname = "a:spcPct"),
            child(variant = SpacingPoints, qname = "a:spcPts")
        )
    )]
  pub space_after_choice: Option<SpaceAfterChoice>,
}
/// Tab List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tabLst")]
pub struct TabStopList {
  /// Tab Stop.
  #[sdk(child(qname = "a:tab"))]
  pub tab_stop: Vec<TabStop>,
}
/// Defines the Text Class.
pub type Text = crate::simple_type::StringValue;
/// Defines the ShapePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct ShapePropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = HiddenFillProperties, qname = "a14:hiddenFill"),
            child(variant = HiddenLineProperties, qname = "a14:hiddenLine"),
            child(variant = HiddenEffectsProperties, qname = "a14:hiddenEffects"),
            child(variant = HiddenScene3D, qname = "a14:hiddenScene3d"),
            child(variant = HiddenShape3D, qname = "a14:hiddenSp3d"),
            child(variant = ShadowObscured, qname = "a14:shadowObscured"),
            any
        )
    )]
  pub shape_properties_extension_choice: Option<ShapePropertiesExtensionChoice>,
}
/// Defines the GvmlGroupShapeExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct GvmlGroupShapeExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = IsCanvas, qname = "a14:isCanvas"), any))]
  pub gvml_group_shape_extension_choice: Option<GvmlGroupShapeExtensionChoice>,
}
/// Defines the ShapePropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct ShapePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ShapePropertiesExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub shape_properties_extension: Vec<ShapePropertiesExtension>,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Visual Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:grpSpPr")]
pub struct VisualGroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<TransformGroup>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub visual_group_shape_properties_choice1: Option<VisualGroupShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub visual_group_shape_properties_choice2: Option<VisualGroupShapePropertiesChoice2>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<Scene3DType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:sp")]
pub struct Shape {
  /// Non-Visual Properties for a Shape
  #[sdk(child(qname = "a:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Visual Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Text Shape
  #[sdk(child(qname = "a:txSp"))]
  pub text_shape: Option<std::boxed::Box<TextShape>>,
  /// Style
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:cxnSp")]
pub struct ConnectionShape {
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "a:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Visual Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:pic")]
pub struct Picture {
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "a:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "a:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// Shape Properties
  #[sdk(child(qname = "a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Style.
  #[sdk(child(qname = "a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:graphicFrame")]
pub struct GraphicFrame {
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "a:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// Graphic Object.
  #[sdk(child(qname = "a:graphic"))]
  pub graphic: std::boxed::Box<Graphic>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: std::boxed::Box<Transform2D>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Group shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:grpSp")]
pub struct GroupShape {
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:grpSpPr"))]
  pub visual_group_shape_properties: std::boxed::Box<VisualGroupShapeProperties>,
  #[sdk(
        choice(
            child(variant = TextShape, qname = "a:txSp"),
            child(variant = Shape, qname = "a:sp"),
            child(variant = ConnectionShape, qname = "a:cxnSp"),
            child(variant = Picture, qname = "a:pic"),
            child(variant = GvmlContentPart, qname = "a14:contentPart"),
            child(variant = GraphicFrame, qname = "a:graphicFrame"),
            child(variant = GroupShape, qname = "a:grpSp")
        )
    )]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// Defines the GvmlGroupShapeExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub gvml_group_shape_extension_list: Option<GvmlGroupShapeExtensionList>,
}
/// Defines the GvmlGroupShapeExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct GvmlGroupShapeExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the GvmlGroupShapeExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub gvml_group_shape_extension: Vec<GvmlGroupShapeExtension>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct NonVisualGroupDrawingShapePropsExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = NonVisualGroupProperties, qname = "a15:nonVisualGroupProps"),
            any
        )
    )]
  pub non_visual_group_drawing_shape_props_extension_choice:
    Option<NonVisualGroupDrawingShapePropsExtensionChoice>,
}
/// Defines the OfficeStyleSheetExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct OfficeStyleSheetExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = ThemeFamily, qname = "thm15:themeFamily"), any))]
  pub office_style_sheet_extension_choice: Option<OfficeStyleSheetExtensionChoice>,
}
/// Defines the ConnectorLockingExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct ConnectorLockingExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = Graphic, qname = "a:graphic"), any))]
  pub connector_locking_extension_choice: Option<ConnectorLockingExtensionChoice>,
}
/// Defines the GroupShapeLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:grpSpLocks")]
pub struct GroupShapeLocks {
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Ungrouping
  #[sdk(attr(qname = ":noUngrp"))]
  pub no_ungrouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Moving Shape
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resizing
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct NonVisualGroupDrawingShapePropsExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the NonVisualGroupDrawingShapePropsExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub non_visual_group_drawing_shape_props_extension: Vec<NonVisualGroupDrawingShapePropsExtension>,
}
/// Defines the ObjectDefaults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:objectDefaults")]
pub struct ObjectDefaults {
  /// Shape Default
  #[sdk(child(qname = "a:spDef"))]
  pub shape_default: Option<std::boxed::Box<ShapeDefault>>,
  /// Line Default
  #[sdk(child(qname = "a:lnDef"))]
  pub line_default: Option<std::boxed::Box<LineDefault>>,
  /// Text Default
  #[sdk(child(qname = "a:txDef"))]
  pub text_default: Option<std::boxed::Box<TextDefault>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtraColorSchemeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extraClrSchemeLst")]
pub struct ExtraColorSchemeList {
  /// Extra Color Scheme.
  #[sdk(child(qname = "a:extraClrScheme"))]
  pub extra_color_scheme: Vec<ExtraColorScheme>,
}
/// Defines the CustomColorList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:custClrLst")]
pub struct CustomColorList {
  /// Custom color.
  #[sdk(child(qname = "a:custClr"))]
  pub custom_color: Vec<CustomColor>,
}
/// Defines the OfficeStyleSheetExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct OfficeStyleSheetExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the OfficeStyleSheetExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub office_style_sheet_extension: Vec<OfficeStyleSheetExtension>,
}
/// Defines the HyperlinkOnClick Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hlinkClick")]
pub struct HyperlinkOnClick {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// relationship identifier to find target URI
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
  #[sdk(attr(qname = ":invalidUrl"))]
  pub invalid_url: Option<crate::simple_type::StringValue>,
  /// Action to take, it may still need r:id to specify an action target
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// target frame for navigating to the URI
  #[sdk(attr(qname = ":tgtFrame"))]
  pub target_frame: Option<crate::simple_type::StringValue>,
  /// tooltip for display
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// whether to add this URI to the history when navigating to it
  #[sdk(attr(qname = ":history"))]
  pub history: Option<crate::simple_type::BooleanValue>,
  /// Whether to highlight it when click on a shape
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// Whether to stop previous sound when click on it
  #[sdk(attr(qname = ":endSnd"))]
  pub end_sound: Option<crate::simple_type::BooleanValue>,
  /// Sound to play.
  #[sdk(child(qname = "a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnMouseOver Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hlinkMouseOver")]
pub struct HyperlinkOnMouseOver {
  /// relationship identifier to find target URI
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
  #[sdk(attr(qname = ":invalidUrl"))]
  pub invalid_url: Option<crate::simple_type::StringValue>,
  /// Action to take, it may still need r:id to specify an action target
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// target frame for navigating to the URI
  #[sdk(attr(qname = ":tgtFrame"))]
  pub target_frame: Option<crate::simple_type::StringValue>,
  /// tooltip for display
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// whether to add this URI to the history when navigating to it
  #[sdk(attr(qname = ":history"))]
  pub history: Option<crate::simple_type::BooleanValue>,
  /// Whether to highlight it when click on a shape
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// Whether to stop previous sound when click on it
  #[sdk(attr(qname = ":endSnd"))]
  pub end_sound: Option<crate::simple_type::BooleanValue>,
  /// Sound to play.
  #[sdk(child(qname = "a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnHover Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:hlinkHover")]
pub struct HyperlinkOnHover {
  /// relationship identifier to find target URI
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
  #[sdk(attr(qname = ":invalidUrl"))]
  pub invalid_url: Option<crate::simple_type::StringValue>,
  /// Action to take, it may still need r:id to specify an action target
  #[sdk(attr(qname = ":action"))]
  pub action: Option<crate::simple_type::StringValue>,
  /// target frame for navigating to the URI
  #[sdk(attr(qname = ":tgtFrame"))]
  pub target_frame: Option<crate::simple_type::StringValue>,
  /// tooltip for display
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// whether to add this URI to the history when navigating to it
  #[sdk(attr(qname = ":history"))]
  pub history: Option<crate::simple_type::BooleanValue>,
  /// Whether to highlight it when click on a shape
  #[sdk(attr(qname = ":highlightClick"))]
  pub highlight_click: Option<crate::simple_type::BooleanValue>,
  /// Whether to stop previous sound when click on it
  #[sdk(attr(qname = ":endSnd"))]
  pub end_sound: Option<crate::simple_type::BooleanValue>,
  /// Sound to play.
  #[sdk(child(qname = "a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the RightToLeft Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:rtl")]
pub struct RightToLeft {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the NonVisualDrawingPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct NonVisualDrawingPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the NonVisualDrawingPropertiesExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub non_visual_drawing_properties_extension: Vec<NonVisualDrawingPropertiesExtension>,
}
/// Defines the ConnectorLockingExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct ConnectorLockingExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ConnectorLockingExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub connector_locking_extension: Vec<ConnectorLockingExtension>,
}
/// Defines the DataModelExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct DataModelExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = DataModelExtensionBlock, qname = "dsp:dataModelExt"),
            child(variant = RecolorImages, qname = "dgm14:recolorImg"),
            any
        )
    )]
  pub data_model_extension_choice: Option<DataModelExtensionChoice>,
}
/// Defines the PtExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct PtExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(child(variant = NonVisualDrawingProperties, qname = "dgm14:cNvPr"), any)
    )]
  pub pt_extension_choice: Option<PtExtensionChoice>,
}
/// Defines the HyperlinkExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct HyperlinkExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(child(variant = HyperlinkColor, qname = "ahyp:hlinkClr"), any))]
  pub hyperlink_extension_choice: Option<HyperlinkExtensionChoice>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct HyperlinkExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the HyperlinkExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub hyperlink_extension: Vec<HyperlinkExtension>,
}
/// Defines the LinePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct LinePropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(
                variant = LineSketchStyleProperties,
                qname = "ask:lineSketchStyleProps"
            ),
            any
        )
    )]
  pub line_properties_extension_choice: Option<LinePropertiesExtensionChoice>,
}
/// default head line end style is none.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:headEnd")]
pub struct HeadEnd {
  /// Line Head/End Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<LineEndValues>,
  /// Width of Head/End
  #[sdk(attr(qname = ":w"))]
  #[sdk(string_format(kind = "token"))]
  pub width: Option<LineEndWidthValues>,
  /// Length of Head/End
  #[sdk(attr(qname = ":len"))]
  #[sdk(string_format(kind = "token"))]
  pub length: Option<LineEndLengthValues>,
}
/// default tail line end style is none.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:tailEnd")]
pub struct TailEnd {
  /// Line Head/End Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(kind = "token"))]
  pub r#type: Option<LineEndValues>,
  /// Width of Head/End
  #[sdk(attr(qname = ":w"))]
  #[sdk(string_format(kind = "token"))]
  pub width: Option<LineEndWidthValues>,
  /// Length of Head/End
  #[sdk(attr(qname = ":len"))]
  #[sdk(string_format(kind = "token"))]
  pub length: Option<LineEndLengthValues>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct LinePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the LinePropertiesExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub line_properties_extension: Vec<LinePropertiesExtension>,
}
/// Defines the NonVisualDrawingPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct NonVisualDrawingPropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = CompatExtension, qname = "a14:compatExt"),
            child(variant = BackgroundProperties, qname = "a15:backgroundPr"),
            child(variant = CreationId, qname = "a16:creationId"),
            child(variant = PredecessorDrawingElementReference, qname = "a16:predDERef"),
            child(variant = Decorative, qname = "adec:decorative"),
            child(variant = ClassificationOutcome, qname = "aclsh:classification"),
            child(variant = ScriptLink, qname = "asl:scriptLink"),
            any
        )
    )]
  pub non_visual_drawing_properties_extension_choice:
    Option<NonVisualDrawingPropertiesExtensionChoice>,
}
/// Defines the PictureLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:picLocks")]
pub struct PictureLocks {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Disallow Shape Grouping
  #[sdk(attr(qname = ":noGrp"))]
  pub no_grouping: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Selection
  #[sdk(attr(qname = ":noSelect"))]
  pub no_selection: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Rotation
  #[sdk(attr(qname = ":noRot"))]
  pub no_rotation: Option<crate::simple_type::BooleanValue>,
  /// Disallow Aspect Ratio Change
  #[sdk(attr(qname = ":noChangeAspect"))]
  pub no_change_aspect: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Movement
  #[sdk(attr(qname = ":noMove"))]
  pub no_move: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Resize
  #[sdk(attr(qname = ":noResize"))]
  pub no_resize: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Point Editing
  #[sdk(attr(qname = ":noEditPoints"))]
  pub no_edit_points: Option<crate::simple_type::BooleanValue>,
  /// Disallow Showing Adjust Handles
  #[sdk(attr(qname = ":noAdjustHandles"))]
  pub no_adjust_handles: Option<crate::simple_type::BooleanValue>,
  /// Disallow Arrowhead Changes
  #[sdk(attr(qname = ":noChangeArrowheads"))]
  pub no_change_arrowheads: Option<crate::simple_type::BooleanValue>,
  /// Disallow Shape Type Change
  #[sdk(attr(qname = ":noChangeShapeType"))]
  pub no_change_shape_type: Option<crate::simple_type::BooleanValue>,
  /// Disallow Crop Changes
  #[sdk(attr(qname = ":noCrop"))]
  pub no_crop: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualPicturePropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct NonVisualPicturePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the NonVisualPicturePropertiesExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub non_visual_picture_properties_extension: Vec<NonVisualPicturePropertiesExtension>,
}
/// Defines the NonVisualPicturePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct NonVisualPicturePropertiesExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = CameraTool, qname = "a14:cameraTool"),
            child(variant = SignatureLine, qname = "a15:signatureLine"),
            child(variant = ObjectProperties, qname = "a15:objectPr"),
            child(variant = LiveFeedProperties, qname = "alf:liveFeedProps"),
            child(variant = ImageFormula, qname = "aif:imageFormula"),
            any
        )
    )]
  pub non_visual_picture_properties_extension_choice:
    Option<NonVisualPicturePropertiesExtensionChoice>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:extLst")]
pub struct BlipExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the BlipExtension Class.
  #[sdk(child(qname = "a:ext"))]
  pub blip_extension: Vec<BlipExtension>,
}
/// Defines the BlipExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:ext")]
pub struct BlipExtension {
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(
        choice(
            child(variant = ImageProperties, qname = "a14:imgProps"),
            child(variant = UseLocalDpi, qname = "a14:useLocalDpi"),
            child(variant = WebVideoProperty, qname = "wp15:webVideoPr"),
            child(variant = SvgBlip, qname = "asvg:svgBlip"),
            child(variant = PictureAttributionSourceUrl, qname = "a1611:picAttrSrcUrl"),
            child(variant = OEmbed, qname = "woe:oembed"),
            child(variant = OEmbedShared, qname = "aoe:oembedShared"),
            any
        )
    )]
  pub blip_extension_choice: Option<BlipExtensionChoice>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RgbColorModelPercentageChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RgbColorModelHexChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum HslColorChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SystemColorChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SchemeColorChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PresetColorChoice {
  /// Tint.
  Tint(std::boxed::Box<Tint>),
  /// Shade.
  Shade(std::boxed::Box<Shade>),
  /// Complement.
  Complement,
  /// Inverse.
  Inverse,
  /// Gray.
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<RedModulation>),
  /// Green.
  Green(std::boxed::Box<Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<BlueModulation>),
  /// Gamma.
  Gamma,
  /// Inverse Gamma.
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SolidFillChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GradientFillChoice {
  /// Linear Gradient Fill.
  LinearGradientFill(std::boxed::Box<LinearGradientFill>),
  /// Path Gradient.
  PathGradientFill(std::boxed::Box<PathGradientFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipFillChoice {
  /// Tile.
  Tile(std::boxed::Box<Tile>),
  /// Stretch.
  Stretch(std::boxed::Box<Stretch>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectContainerChoice {
  /// Effect Container.
  EffectContainer(std::boxed::Box<EffectContainer>),
  /// Effect.
  Effect(std::boxed::Box<Effect>),
  /// Defines the AlphaBiLevel Class.
  AlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  AlphaCeiling,
  /// Alpha Floor Effect.
  AlphaFloor,
  /// Alpha Inverse Effect.
  AlphaInverse(std::boxed::Box<AlphaInverse>),
  /// Alpha Modulate Effect.
  AlphaModulationEffect(std::boxed::Box<AlphaModulationEffect>),
  /// Defines the AlphaModulationFixed Class.
  AlphaModulationFixed(std::boxed::Box<AlphaModulationFixed>),
  /// Alpha Inset/Outset Effect.
  AlphaOutset(std::boxed::Box<AlphaOutset>),
  /// Alpha Replace Effect.
  AlphaReplace(std::boxed::Box<AlphaReplace>),
  /// Defines the BiLevel Class.
  BiLevel(std::boxed::Box<BiLevel>),
  /// Blend Effect.
  Blend(std::boxed::Box<Blend>),
  /// Defines the Blur Class.
  Blur(std::boxed::Box<Blur>),
  /// Color Change Effect.
  ColorChange(std::boxed::Box<ColorChange>),
  /// Defines the ColorReplacement Class.
  ColorReplacement(std::boxed::Box<ColorReplacement>),
  /// Duotone Effect.
  Duotone(std::boxed::Box<Duotone>),
  /// Fill.
  Fill(std::boxed::Box<Fill>),
  /// Fill Overlay Effect.
  FillOverlay(std::boxed::Box<FillOverlay>),
  /// Glow Effect.
  Glow(std::boxed::Box<Glow>),
  /// Gray Scale Effect.
  Grayscale,
  /// Hue Saturation Luminance Effect.
  Hsl(std::boxed::Box<Hsl>),
  /// Inner Shadow Effect.
  InnerShadow(std::boxed::Box<InnerShadow>),
  /// Luminance.
  LuminanceEffect(std::boxed::Box<LuminanceEffect>),
  /// Outer Shadow Effect.
  OuterShadow(std::boxed::Box<OuterShadow>),
  /// Preset Shadow.
  PresetShadow(std::boxed::Box<PresetShadow>),
  /// Reflection Effect.
  Reflection(std::boxed::Box<Reflection>),
  /// Relative Offset Effect.
  RelativeOffset(std::boxed::Box<RelativeOffset>),
  /// Soft Edge Effect.
  SoftEdge(std::boxed::Box<SoftEdge>),
  /// Defines the TintEffect Class.
  TintEffect(std::boxed::Box<TintEffect>),
  /// Transform Effect.
  TransformEffect(std::boxed::Box<TransformEffect>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectDagChoice {
  /// Effect Container.
  EffectContainer(std::boxed::Box<EffectContainer>),
  /// Effect.
  Effect(std::boxed::Box<Effect>),
  /// Defines the AlphaBiLevel Class.
  AlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  AlphaCeiling,
  /// Alpha Floor Effect.
  AlphaFloor,
  /// Alpha Inverse Effect.
  AlphaInverse(std::boxed::Box<AlphaInverse>),
  /// Alpha Modulate Effect.
  AlphaModulationEffect(std::boxed::Box<AlphaModulationEffect>),
  /// Defines the AlphaModulationFixed Class.
  AlphaModulationFixed(std::boxed::Box<AlphaModulationFixed>),
  /// Alpha Inset/Outset Effect.
  AlphaOutset(std::boxed::Box<AlphaOutset>),
  /// Alpha Replace Effect.
  AlphaReplace(std::boxed::Box<AlphaReplace>),
  /// Defines the BiLevel Class.
  BiLevel(std::boxed::Box<BiLevel>),
  /// Blend Effect.
  Blend(std::boxed::Box<Blend>),
  /// Defines the Blur Class.
  Blur(std::boxed::Box<Blur>),
  /// Color Change Effect.
  ColorChange(std::boxed::Box<ColorChange>),
  /// Defines the ColorReplacement Class.
  ColorReplacement(std::boxed::Box<ColorReplacement>),
  /// Duotone Effect.
  Duotone(std::boxed::Box<Duotone>),
  /// Fill.
  Fill(std::boxed::Box<Fill>),
  /// Fill Overlay Effect.
  FillOverlay(std::boxed::Box<FillOverlay>),
  /// Glow Effect.
  Glow(std::boxed::Box<Glow>),
  /// Gray Scale Effect.
  Grayscale,
  /// Hue Saturation Luminance Effect.
  Hsl(std::boxed::Box<Hsl>),
  /// Inner Shadow Effect.
  InnerShadow(std::boxed::Box<InnerShadow>),
  /// Luminance.
  LuminanceEffect(std::boxed::Box<LuminanceEffect>),
  /// Outer Shadow Effect.
  OuterShadow(std::boxed::Box<OuterShadow>),
  /// Preset Shadow.
  PresetShadow(std::boxed::Box<PresetShadow>),
  /// Reflection Effect.
  Reflection(std::boxed::Box<Reflection>),
  /// Relative Offset Effect.
  RelativeOffset(std::boxed::Box<RelativeOffset>),
  /// Soft Edge Effect.
  SoftEdge(std::boxed::Box<SoftEdge>),
  /// Defines the TintEffect Class.
  TintEffect(std::boxed::Box<TintEffect>),
  /// Transform Effect.
  TransformEffect(std::boxed::Box<TransformEffect>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum AlphaInverseChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorReplacementChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DuotoneChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillOverlayChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GlowChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum InnerShadowChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OuterShadowChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PresetShadowChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LineReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectPropertiesTypeChoice {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FontReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BulletColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ExtrusionColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ContourColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorFromChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ColorToChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ForegroundColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HighlightChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnderlineChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnderlineChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnderlineChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OutlineChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OutlineChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OutlineChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LeftBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LeftBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LeftBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RightBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RightBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice2 {
  /// Preset Dash.
  PresetDash(std::boxed::Box<PresetDash>),
  /// Custom Dash.
  CustomDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  Round,
  /// Line Join Bevel.
  LineJoinBevel,
  /// Miter Line Join.
  Miter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum UnderlineFillChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipChoice {
  /// Defines the AlphaBiLevel Class.
  AlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  AlphaCeiling,
  /// Alpha Floor Effect.
  AlphaFloor,
  /// Alpha Inverse Effect.
  AlphaInverse(std::boxed::Box<AlphaInverse>),
  /// Alpha Modulate Effect.
  AlphaModulationEffect(std::boxed::Box<AlphaModulationEffect>),
  /// Defines the AlphaModulationFixed Class.
  AlphaModulationFixed(std::boxed::Box<AlphaModulationFixed>),
  /// Alpha Replace Effect.
  AlphaReplace(std::boxed::Box<AlphaReplace>),
  /// Defines the BiLevel Class.
  BiLevel(std::boxed::Box<BiLevel>),
  /// Defines the Blur Class.
  Blur(std::boxed::Box<Blur>),
  /// Color Change Effect.
  ColorChange(std::boxed::Box<ColorChange>),
  /// Defines the ColorReplacement Class.
  ColorReplacement(std::boxed::Box<ColorReplacement>),
  /// Duotone Effect.
  Duotone(std::boxed::Box<Duotone>),
  /// Fill Overlay Effect.
  FillOverlay(std::boxed::Box<FillOverlay>),
  /// Gray Scale Effect.
  Grayscale,
  /// Hue Saturation Luminance Effect.
  Hsl(std::boxed::Box<Hsl>),
  /// Luminance.
  LuminanceEffect(std::boxed::Box<LuminanceEffect>),
  /// Defines the TintEffect Class.
  TintEffect(std::boxed::Box<TintEffect>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableStyleListChoice {
  /// Table Style.
  TableStyleEntry(std::boxed::Box<TableStyleEntry>),
  /// Unknown XML child.
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CustomColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EffectStyleChoice {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FillStyleListChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundFillStyleListChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Dark1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Light1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Dark2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Light2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent3ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent4ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent5ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Accent6ColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HyperlinkChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FollowedHyperlinkColorChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GraphicDataChoice {
  /// Graphic Object.
  Graphic(std::boxed::Box<Graphic>),
  /// Defines the Blip Class.
  Blip(std::boxed::Box<Blip>),
  /// Theme.
  Theme(std::boxed::Box<Theme>),
  /// Theme Override.
  ThemeOverride(std::boxed::Box<ThemeOverride>),
  /// Theme Manager.
  ThemeManager,
  /// Table.
  Table(std::boxed::Box<Table>),
  /// Table Style List.
  TableStyleList(std::boxed::Box<TableStyleList>),
  OEmbedShared(std::boxed::Box<crate::schemas::aoe::OEmbedShared>),
  OEmbed(std::boxed::Box<crate::schemas::woe::OEmbed>),
  ImageFormula(std::boxed::Box<crate::schemas::aif::ImageFormula>),
  LiveFeedProperties(std::boxed::Box<crate::schemas::alf::LiveFeedProperties>),
  ScriptLink(std::boxed::Box<crate::schemas::asl::ScriptLink>),
  ClassificationOutcome(std::boxed::Box<crate::schemas::aclsh::ClassificationOutcome>),
  LineSketchStyleProperties(std::boxed::Box<crate::schemas::ask::LineSketchStyleProperties>),
  PictureAttributionSourceUrl(std::boxed::Box<crate::schemas::a1611::PictureAttributionSourceUrl>),
  SvgBlip(std::boxed::Box<crate::schemas::asvg::SvgBlip>),
  Decorative(std::boxed::Box<crate::schemas::adec::Decorative>),
  A16CreationId(std::boxed::Box<crate::schemas::a16::CreationId>),
  PredecessorDrawingElementReference(
    std::boxed::Box<crate::schemas::a16::PredecessorDrawingElementReference>,
  ),
  ConnectableReferences(std::boxed::Box<crate::schemas::a16::ConnectableReferences>),
  RowIdIdentifier(std::boxed::Box<crate::schemas::a16::RowIdIdentifier>),
  ColIdIdentifier(std::boxed::Box<crate::schemas::a16::ColIdIdentifier>),
  HyperlinkColor(std::boxed::Box<crate::schemas::ahyp::HyperlinkColor>),
  WebVideoProperty(std::boxed::Box<crate::schemas::wp15::WebVideoProperty>),
  ThemeFamily(std::boxed::Box<crate::schemas::thm15::ThemeFamily>),
  BackgroundProperties(std::boxed::Box<crate::schemas::a15::BackgroundProperties>),
  NonVisualGroupProperties(std::boxed::Box<crate::schemas::a15::NonVisualGroupProperties>),
  ObjectProperties(std::boxed::Box<crate::schemas::a15::ObjectProperties>),
  A15SignatureLine(std::boxed::Box<crate::schemas::a15::SignatureLine>),
  CameraTool(std::boxed::Box<crate::schemas::a14::CameraTool>),
  CompatExtension(std::boxed::Box<crate::schemas::a14::CompatExtension>),
  IsCanvas(std::boxed::Box<crate::schemas::a14::IsCanvas>),
  GvmlContentPart(std::boxed::Box<crate::schemas::a14::GvmlContentPart>),
  ShadowObscured(std::boxed::Box<crate::schemas::a14::ShadowObscured>),
  HiddenFillProperties(std::boxed::Box<crate::schemas::a14::HiddenFillProperties>),
  HiddenLineProperties(std::boxed::Box<crate::schemas::a14::HiddenLineProperties>),
  HiddenEffectsProperties(std::boxed::Box<crate::schemas::a14::HiddenEffectsProperties>),
  HiddenScene3D(std::boxed::Box<crate::schemas::a14::HiddenScene3D>),
  HiddenShape3D(std::boxed::Box<crate::schemas::a14::HiddenShape3D>),
  ImageProperties(std::boxed::Box<crate::schemas::a14::ImageProperties>),
  UseLocalDpi(std::boxed::Box<crate::schemas::a14::UseLocalDpi>),
  TextMath(std::boxed::Box<crate::schemas::a14::TextMath>),
  NonVisualDrawingProperties(std::boxed::Box<crate::schemas::dgm14::NonVisualDrawingProperties>),
  RecolorImages(std::boxed::Box<crate::schemas::dgm14::RecolorImages>),
  Drawing(std::boxed::Box<crate::schemas::dsp::Drawing>),
  DataModelExtensionBlock(std::boxed::Box<crate::schemas::dsp::DataModelExtensionBlock>),
  ColorsDefinition(std::boxed::Box<crate::schemas::dgm::ColorsDefinition>),
  ColorsDefinitionHeader(std::boxed::Box<crate::schemas::dgm::ColorsDefinitionHeader>),
  ColorsDefinitionHeaderList(std::boxed::Box<crate::schemas::dgm::ColorsDefinitionHeaderList>),
  DataModelRoot(std::boxed::Box<crate::schemas::dgm::DataModelRoot>),
  LayoutDefinition(std::boxed::Box<crate::schemas::dgm::LayoutDefinition>),
  LayoutDefinitionHeader(std::boxed::Box<crate::schemas::dgm::LayoutDefinitionHeader>),
  LayoutDefinitionHeaderList(std::boxed::Box<crate::schemas::dgm::LayoutDefinitionHeaderList>),
  RelationshipIds(std::boxed::Box<crate::schemas::dgm::RelationshipIds>),
  StyleDefinition(std::boxed::Box<crate::schemas::dgm::StyleDefinition>),
  StyleDefinitionHeader(std::boxed::Box<crate::schemas::dgm::StyleDefinitionHeader>),
  StyleDefinitionHeaderList(std::boxed::Box<crate::schemas::dgm::StyleDefinitionHeaderList>),
  Dgm1612ShapeProperties(std::boxed::Box<crate::schemas::dgm1612::ShapeProperties>),
  TextListStyleType(std::boxed::Box<crate::schemas::dgm1612::TextListStyleType>),
  NumberDiagramInfoList(std::boxed::Box<crate::schemas::dgm1611::NumberDiagramInfoList>),
  ChartSpace(std::boxed::Box<crate::schemas::c::ChartSpace>),
  UserShapes(std::boxed::Box<crate::schemas::c::UserShapes>),
  ChartReference(std::boxed::Box<crate::schemas::c::ChartReference>),
  DataDisplayOptions16(std::boxed::Box<crate::schemas::c16r3::DataDisplayOptions16>),
  C16ShapeProperties(std::boxed::Box<crate::schemas::c16::ShapeProperties>),
  UnsignedIntegerType(std::boxed::Box<crate::schemas::c16::UnsignedIntegerType>),
  InvertIfNegativeBoolean(std::boxed::Box<crate::schemas::c16::InvertIfNegativeBoolean>),
  Bubble3DBoolean(std::boxed::Box<crate::schemas::c16::Bubble3DBoolean>),
  Marker(std::boxed::Box<crate::schemas::c16::Marker>),
  DLbl(std::boxed::Box<crate::schemas::c16::DLbl>),
  C16CategoryFilterExceptions(std::boxed::Box<crate::schemas::c16::CategoryFilterExceptions>),
  PivotOptions16(std::boxed::Box<crate::schemas::c16::PivotOptions16>),
  ChartDataPointUniqueIdMap(std::boxed::Box<crate::schemas::c16::ChartDataPointUniqueIdMap>),
  UniqueIdChartUniqueId(std::boxed::Box<crate::schemas::c16::UniqueIdChartUniqueId>),
  PivotSource(std::boxed::Box<crate::schemas::c15::PivotSource>),
  NumberingFormat(std::boxed::Box<crate::schemas::c15::NumberingFormat>),
  C15ShapeProperties(std::boxed::Box<crate::schemas::c15::ShapeProperties>),
  Layout(std::boxed::Box<crate::schemas::c15::Layout>),
  FullReference(std::boxed::Box<crate::schemas::c15::FullReference>),
  LevelReference(std::boxed::Box<crate::schemas::c15::LevelReference>),
  FormulaReference(std::boxed::Box<crate::schemas::c15::FormulaReference>),
  FilteredSeriesTitle(std::boxed::Box<crate::schemas::c15::FilteredSeriesTitle>),
  FilteredCategoryTitle(std::boxed::Box<crate::schemas::c15::FilteredCategoryTitle>),
  FilteredAreaSeries(std::boxed::Box<crate::schemas::c15::FilteredAreaSeries>),
  FilteredBarSeries(std::boxed::Box<crate::schemas::c15::FilteredBarSeries>),
  FilteredBubbleSeries(std::boxed::Box<crate::schemas::c15::FilteredBubbleSeries>),
  FilteredLineSeriesExtension(std::boxed::Box<crate::schemas::c15::FilteredLineSeriesExtension>),
  FilteredPieSeries(std::boxed::Box<crate::schemas::c15::FilteredPieSeries>),
  FilteredRadarSeries(std::boxed::Box<crate::schemas::c15::FilteredRadarSeries>),
  FilteredScatterSeries(std::boxed::Box<crate::schemas::c15::FilteredScatterSeries>),
  FilteredSurfaceSeries(std::boxed::Box<crate::schemas::c15::FilteredSurfaceSeries>),
  DataLabelsRange(std::boxed::Box<crate::schemas::c15::DataLabelsRange>),
  C15CategoryFilterExceptions(std::boxed::Box<crate::schemas::c15::CategoryFilterExceptions>),
  DataLabelFieldTable(std::boxed::Box<crate::schemas::c15::DataLabelFieldTable>),
  ExceptionForSave(std::boxed::Box<crate::schemas::c15::ExceptionForSave>),
  ShowDataLabelsRange(std::boxed::Box<crate::schemas::c15::ShowDataLabelsRange>),
  ChartText(std::boxed::Box<crate::schemas::c15::ChartText>),
  ShowLeaderLines(std::boxed::Box<crate::schemas::c15::ShowLeaderLines>),
  LeaderLines(std::boxed::Box<crate::schemas::c15::LeaderLines>),
  AutoGeneneratedCategories(std::boxed::Box<crate::schemas::c15::AutoGeneneratedCategories>),
  PivotOptions(std::boxed::Box<crate::schemas::c14::PivotOptions>),
  SketchOptions(std::boxed::Box<crate::schemas::c14::SketchOptions>),
  InvertSolidFillFormat(std::boxed::Box<crate::schemas::c14::InvertSolidFillFormat>),
  Style(std::boxed::Box<crate::schemas::c14::Style>),
  Cdr14ContentPart(std::boxed::Box<crate::schemas::cdr14::ContentPart>),
  LegacyDrawing(std::boxed::Box<crate::schemas::comp::LegacyDrawing>),
  LockedCanvas(std::boxed::Box<crate::schemas::lc::LockedCanvas>),
  Inline(std::boxed::Box<crate::schemas::wp::Inline>),
  Anchor(std::boxed::Box<crate::schemas::wp::Anchor>),
  PercentagePositionHeightOffset(crate::schemas::wp14::PercentagePositionHeightOffset),
  PercentagePositionVerticalOffset(crate::schemas::wp14::PercentagePositionVerticalOffset),
  RelativeWidth(std::boxed::Box<crate::schemas::wp14::RelativeWidth>),
  RelativeHeight(std::boxed::Box<crate::schemas::wp14::RelativeHeight>),
  Picture(std::boxed::Box<crate::schemas::pic::Picture>),
  ShapeStyle(std::boxed::Box<crate::schemas::pic14::ShapeStyle>),
  OfficeArtExtensionList(std::boxed::Box<crate::schemas::pic14::OfficeArtExtensionList>),
  WorksheetDrawing(std::boxed::Box<crate::schemas::xdr::WorksheetDrawing>),
  XdrContentPart(std::boxed::Box<crate::schemas::xdr::ContentPart>),
  Xdr14ContentPart(std::boxed::Box<crate::schemas::xdr14::ContentPart>),
  CommentAuthorMonikerList(crate::schemas::pc::CommentAuthorMonikerList),
  CommentMonikerList(crate::schemas::pc::CommentMonikerList),
  StringTagMonikerList(crate::schemas::pc::StringTagMonikerList),
  CustomShowMonikerList(crate::schemas::pc::CustomShowMonikerList),
  DocumentMonikerList(crate::schemas::pc::DocumentMonikerList),
  SectionMonikerList(crate::schemas::pc::SectionMonikerList),
  SlideBaseMonikerList(crate::schemas::pc::SlideBaseMonikerList),
  SlideLayoutMonikerList(crate::schemas::pc::SlideLayoutMonikerList),
  MainMasterMonikerList(crate::schemas::pc::MainMasterMonikerList),
  SlideMonikerList(std::boxed::Box<crate::schemas::pc::SlideMonikerList>),
  SlidePosMonikerList(crate::schemas::pc::SlidePosMonikerList),
  NotesMonikerList(crate::schemas::pc::NotesMonikerList),
  NotesTextMonikerList(crate::schemas::pc::NotesTextMonikerList),
  NotesMasterMonikerList(crate::schemas::pc::NotesMasterMonikerList),
  HandoutMonikerList(crate::schemas::pc::HandoutMonikerList),
  AnimEffectMkLstAnimationEffectMonikerList(
    crate::schemas::pc::AnimEffectMkLstAnimationEffectMonikerList,
  ),
  AnimEffectParentMkLstAnimationEffectMonikerList(
    crate::schemas::pc::AnimEffectParentMkLstAnimationEffectMonikerList,
  ),
  OsfTaskPaneAppMonikerList(crate::schemas::pc::OsfTaskPaneAppMonikerList),
  SummaryZoomMonikerList(crate::schemas::pc::SummaryZoomMonikerList),
  SectionLinkObjMonikerList(crate::schemas::pc::SectionLinkObjMonikerList),
  DesignerTagMonikerList(crate::schemas::pc::DesignerTagMonikerList),
  CustomXmlPartMonikerList(crate::schemas::pc::CustomXmlPartMonikerList),
  CommentAuthorList(std::boxed::Box<crate::schemas::p::CommentAuthorList>),
  PCommentList(std::boxed::Box<crate::schemas::p::CommentList>),
  POleObject(std::boxed::Box<crate::schemas::p::OleObject>),
  Presentation(std::boxed::Box<crate::schemas::p::Presentation>),
  PresentationProperties(std::boxed::Box<crate::schemas::p::PresentationProperties>),
  Slide(std::boxed::Box<crate::schemas::p::Slide>),
  SlideLayout(std::boxed::Box<crate::schemas::p::SlideLayout>),
  SlideMaster(std::boxed::Box<crate::schemas::p::SlideMaster>),
  HandoutMaster(std::boxed::Box<crate::schemas::p::HandoutMaster>),
  NotesMaster(std::boxed::Box<crate::schemas::p::NotesMaster>),
  NotesSlide(std::boxed::Box<crate::schemas::p::NotesSlide>),
  SlideSyncProperties(std::boxed::Box<crate::schemas::p::SlideSyncProperties>),
  TagList(std::boxed::Box<crate::schemas::p::TagList>),
  ViewProperties(std::boxed::Box<crate::schemas::p::ViewProperties>),
  PContentPart(std::boxed::Box<crate::schemas::p::ContentPart>),
  PlaceholderTypeExtension(std::boxed::Box<crate::schemas::p232::PlaceholderTypeExtension>),
  AuthorList(std::boxed::Box<crate::schemas::p188::AuthorList>),
  P188CommentList(std::boxed::Box<crate::schemas::p188::CommentList>),
  CommentRelationship(std::boxed::Box<crate::schemas::p188::CommentRelationship>),
  Reactions(std::boxed::Box<crate::schemas::p223::Reactions>),
  TaskDetails(std::boxed::Box<crate::schemas::p228::TaskDetails>),
  TaskHistoryDetails(std::boxed::Box<crate::schemas::p1912::TaskHistoryDetails>),
  /// Defines the TextBodyPackage Class.
  TextBodyPackage,
  GroupCommand(std::boxed::Box<crate::schemas::oac::GroupCommand>),
  ImgDataImgData(crate::schemas::oac::ImgDataImgData),
  OrigImgDataImgData(crate::schemas::oac::OrigImgDataImgData),
  ImgLink(std::boxed::Box<crate::schemas::oac::ImgLink>),
  DrawingMonikerList(crate::schemas::oac::DrawingMonikerList),
  DocumentContextMonikerList(crate::schemas::oac::DocumentContextMonikerList),
  GraphicParentMonikerList(crate::schemas::oac::GraphicParentMonikerList),
  DeMkLstDrawingElementMonikerList(crate::schemas::oac::DeMkLstDrawingElementMonikerList),
  DeMasterMkLstDrawingElementMonikerList(
    crate::schemas::oac::DeMasterMkLstDrawingElementMonikerList,
  ),
  ShapeMonikerList(crate::schemas::oac::ShapeMonikerList),
  GroupShapeMonikerList(crate::schemas::oac::GroupShapeMonikerList),
  GraphicFrameMonikerList(crate::schemas::oac::GraphicFrameMonikerList),
  ConnectorMonikerList(crate::schemas::oac::ConnectorMonikerList),
  PictureMonikerList(crate::schemas::oac::PictureMonikerList),
  InkMonikerList(crate::schemas::oac::InkMonikerList),
  TextBodyMonikerList(crate::schemas::oac::TextBodyMonikerList),
  TextCharRangeMonikerList(crate::schemas::oac::TextCharRangeMonikerList),
  HyperlinkMonikerList(crate::schemas::oac::HyperlinkMonikerList),
  Model3DMonikerList(crate::schemas::oac::Model3DMonikerList),
  ViewSelectionStgList(crate::schemas::oac::ViewSelectionStgList),
  EditorSelectionStgList(crate::schemas::oac::EditorSelectionStgList),
  DrawingSelectionStgList(crate::schemas::oac::DrawingSelectionStgList),
  TableMonikerList(crate::schemas::oac::TableMonikerList),
  TableCellMonikerList(crate::schemas::oac::TableCellMonikerList),
  TableRowMonikerList(crate::schemas::oac::TableRowMonikerList),
  TableColumnMonikerList(crate::schemas::oac::TableColumnMonikerList),
  InkmlInk(std::boxed::Box<crate::schemas::inkml::Ink>),
  OneOf(std::boxed::Box<crate::schemas::emma::OneOf>),
  EmmaGroup(std::boxed::Box<crate::schemas::emma::Group>),
  Sequence(std::boxed::Box<crate::schemas::emma::Sequence>),
  EndPoint(std::boxed::Box<crate::schemas::emma::EndPoint>),
  EndPointInfo(std::boxed::Box<crate::schemas::emma::EndPointInfo>),
  Info(std::boxed::Box<crate::schemas::emma::Info>),
  Grammar(std::boxed::Box<crate::schemas::emma::Grammar>),
  DerivedFrom(std::boxed::Box<crate::schemas::emma::DerivedFrom>),
  Node(std::boxed::Box<crate::schemas::emma::Node>),
  EmmaArc(std::boxed::Box<crate::schemas::emma::Arc>),
  Lattice(std::boxed::Box<crate::schemas::emma::Lattice>),
  Literal(crate::schemas::emma::Literal),
  Interpretation(std::boxed::Box<crate::schemas::emma::Interpretation>),
  GroupInfo(std::boxed::Box<crate::schemas::emma::GroupInfo>),
  Derivation(std::boxed::Box<crate::schemas::emma::Derivation>),
  Model(std::boxed::Box<crate::schemas::emma::Model>),
  Emma(std::boxed::Box<crate::schemas::emma::Emma>),
  ContextNode(std::boxed::Box<crate::schemas::msink::ContextNode>),
  PresetTransition(std::boxed::Box<crate::schemas::p15::PresetTransition>),
  PresenceInfo(std::boxed::Box<crate::schemas::p15::PresenceInfo>),
  ThreadingInfo(std::boxed::Box<crate::schemas::p15::ThreadingInfo>),
  SlideGuideList(std::boxed::Box<crate::schemas::p15::SlideGuideList>),
  NotesGuideList(std::boxed::Box<crate::schemas::p15::NotesGuideList>),
  ChartTrackingReferenceBased(std::boxed::Box<crate::schemas::p15::ChartTrackingReferenceBased>),
  NonVisualContentPartProperties(
    std::boxed::Box<crate::schemas::p14::NonVisualContentPartProperties>,
  ),
  Transform2D(std::boxed::Box<crate::schemas::p14::Transform2D>),
  ExtensionListModify(std::boxed::Box<crate::schemas::p14::ExtensionListModify>),
  Media(std::boxed::Box<crate::schemas::p14::Media>),
  VortexTransition(std::boxed::Box<crate::schemas::p14::VortexTransition>),
  SwitchTransition(std::boxed::Box<crate::schemas::p14::SwitchTransition>),
  FlipTransition(std::boxed::Box<crate::schemas::p14::FlipTransition>),
  RippleTransition(std::boxed::Box<crate::schemas::p14::RippleTransition>),
  /// Defines the HoneycombTransition Class.
  HoneycombTransition,
  PrismTransition(std::boxed::Box<crate::schemas::p14::PrismTransition>),
  DoorsTransition(std::boxed::Box<crate::schemas::p14::DoorsTransition>),
  WindowTransition(std::boxed::Box<crate::schemas::p14::WindowTransition>),
  FerrisTransition(std::boxed::Box<crate::schemas::p14::FerrisTransition>),
  GalleryTransition(std::boxed::Box<crate::schemas::p14::GalleryTransition>),
  ConveyorTransition(std::boxed::Box<crate::schemas::p14::ConveyorTransition>),
  PanTransition(std::boxed::Box<crate::schemas::p14::PanTransition>),
  GlitterTransition(std::boxed::Box<crate::schemas::p14::GlitterTransition>),
  WarpTransition(std::boxed::Box<crate::schemas::p14::WarpTransition>),
  FlythroughTransition(std::boxed::Box<crate::schemas::p14::FlythroughTransition>),
  /// Defines the FlashTransition Class.
  FlashTransition,
  ShredTransition(std::boxed::Box<crate::schemas::p14::ShredTransition>),
  RevealTransition(std::boxed::Box<crate::schemas::p14::RevealTransition>),
  WheelReverseTransition(std::boxed::Box<crate::schemas::p14::WheelReverseTransition>),
  BookmarkTarget(std::boxed::Box<crate::schemas::p14::BookmarkTarget>),
  SectionProperties(std::boxed::Box<crate::schemas::p14::SectionProperties>),
  SectionList(std::boxed::Box<crate::schemas::p14::SectionList>),
  BrowseMode(std::boxed::Box<crate::schemas::p14::BrowseMode>),
  LaserColor(std::boxed::Box<crate::schemas::p14::LaserColor>),
  P14DefaultImageDpi(std::boxed::Box<crate::schemas::p14::DefaultImageDpi>),
  DiscardImageEditData(std::boxed::Box<crate::schemas::p14::DiscardImageEditData>),
  ShowMediaControls(std::boxed::Box<crate::schemas::p14::ShowMediaControls>),
  LaserTraceList(std::boxed::Box<crate::schemas::p14::LaserTraceList>),
  P14CreationId(std::boxed::Box<crate::schemas::p14::CreationId>),
  ModificationId(std::boxed::Box<crate::schemas::p14::ModificationId>),
  ShowEventRecordList(std::boxed::Box<crate::schemas::p14::ShowEventRecordList>),
  Recipients(std::boxed::Box<crate::schemas::w::Recipients>),
  TextBoxContent(std::boxed::Box<crate::schemas::w::TextBoxContent>),
  Comments(std::boxed::Box<crate::schemas::w::Comments>),
  Footnotes(std::boxed::Box<crate::schemas::w::Footnotes>),
  Endnotes(std::boxed::Box<crate::schemas::w::Endnotes>),
  Header(std::boxed::Box<crate::schemas::w::Header>),
  Footer(std::boxed::Box<crate::schemas::w::Footer>),
  Settings(std::boxed::Box<crate::schemas::w::Settings>),
  WebSettings(std::boxed::Box<crate::schemas::w::WebSettings>),
  Fonts(std::boxed::Box<crate::schemas::w::Fonts>),
  Numbering(std::boxed::Box<crate::schemas::w::Numbering>),
  Styles(std::boxed::Box<crate::schemas::w::Styles>),
  Document(std::boxed::Box<crate::schemas::w::Document>),
  GlossaryDocument(std::boxed::Box<crate::schemas::w::GlossaryDocument>),
  Color(std::boxed::Box<crate::schemas::w15::Color>),
  DataBinding(std::boxed::Box<crate::schemas::w15::DataBinding>),
  Appearance(std::boxed::Box<crate::schemas::w15::Appearance>),
  CommentsEx(std::boxed::Box<crate::schemas::w15::CommentsEx>),
  People(std::boxed::Box<crate::schemas::w15::People>),
  SdtRepeatedSection(std::boxed::Box<crate::schemas::w15::SdtRepeatedSection>),
  /// Defines the SdtRepeatedSectionItem Class.
  SdtRepeatedSectionItem,
  ChartTrackingRefBased(std::boxed::Box<crate::schemas::w15::ChartTrackingRefBased>),
  DefaultCollapsed(std::boxed::Box<crate::schemas::w15::DefaultCollapsed>),
  PersistentDocumentId(std::boxed::Box<crate::schemas::w15::PersistentDocumentId>),
  FootnoteColumns(std::boxed::Box<crate::schemas::w15::FootnoteColumns>),
  WebExtensionLinked(std::boxed::Box<crate::schemas::w15::WebExtensionLinked>),
  WebExtensionCreated(std::boxed::Box<crate::schemas::w15::WebExtensionCreated>),
  W14ContentPart(std::boxed::Box<crate::schemas::w14::ContentPart>),
  DocumentId(std::boxed::Box<crate::schemas::w14::DocumentId>),
  ConflictMode(std::boxed::Box<crate::schemas::w14::ConflictMode>),
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
  DiscardImageEditingData(std::boxed::Box<crate::schemas::w14::DiscardImageEditingData>),
  W14DefaultImageDpi(std::boxed::Box<crate::schemas::w14::DefaultImageDpi>),
  /// Defines the EntityPickerEmpty Class.
  EntityPickerEmpty,
  SdtContentCheckBox(std::boxed::Box<crate::schemas::w14::SdtContentCheckBox>),
  SchemaLibrary(std::boxed::Box<crate::schemas::sl::SchemaLibrary>),
  MathProperties(std::boxed::Box<crate::schemas::m::MathProperties>),
  Paragraph(std::boxed::Box<crate::schemas::m::Paragraph>),
  OfficeMath(std::boxed::Box<crate::schemas::m::OfficeMath>),
  Shape(std::boxed::Box<crate::schemas::v::Shape>),
  Shapetype(std::boxed::Box<crate::schemas::v::Shapetype>),
  VGroup(std::boxed::Box<crate::schemas::v::Group>),
  Background(std::boxed::Box<crate::schemas::v::Background>),
  Fill(std::boxed::Box<crate::schemas::v::Fill>),
  Formulas(std::boxed::Box<crate::schemas::v::Formulas>),
  ShapeHandles(std::boxed::Box<crate::schemas::v::ShapeHandles>),
  ImageData(std::boxed::Box<crate::schemas::v::ImageData>),
  Path(std::boxed::Box<crate::schemas::v::Path>),
  TextBox(std::boxed::Box<crate::schemas::v::TextBox>),
  Shadow(std::boxed::Box<crate::schemas::v::Shadow>),
  Stroke(std::boxed::Box<crate::schemas::v::Stroke>),
  TextPath(std::boxed::Box<crate::schemas::v::TextPath>),
  VArc(std::boxed::Box<crate::schemas::v::Arc>),
  Curve(std::boxed::Box<crate::schemas::v::Curve>),
  ImageFile(std::boxed::Box<crate::schemas::v::ImageFile>),
  Line(std::boxed::Box<crate::schemas::v::Line>),
  Oval(std::boxed::Box<crate::schemas::v::Oval>),
  PolyLine(std::boxed::Box<crate::schemas::v::PolyLine>),
  Rectangle(std::boxed::Box<crate::schemas::v::Rectangle>),
  RoundRectangle(std::boxed::Box<crate::schemas::v::RoundRectangle>),
  ShapeDefaults(std::boxed::Box<crate::schemas::o::ShapeDefaults>),
  ShapeLayout(std::boxed::Box<crate::schemas::o::ShapeLayout>),
  OSignatureLine(std::boxed::Box<crate::schemas::o::SignatureLine>),
  OInk(std::boxed::Box<crate::schemas::o::Ink>),
  Diagram(std::boxed::Box<crate::schemas::o::Diagram>),
  Skew(std::boxed::Box<crate::schemas::o::Skew>),
  Extrusion(std::boxed::Box<crate::schemas::o::Extrusion>),
  Callout(std::boxed::Box<crate::schemas::o::Callout>),
  Lock(std::boxed::Box<crate::schemas::o::Lock>),
  OOleObject(std::boxed::Box<crate::schemas::o::OleObject>),
  Complex(std::boxed::Box<crate::schemas::o::Complex>),
  LeftStroke(std::boxed::Box<crate::schemas::o::LeftStroke>),
  TopStroke(std::boxed::Box<crate::schemas::o::TopStroke>),
  RightStroke(std::boxed::Box<crate::schemas::o::RightStroke>),
  BottomStroke(std::boxed::Box<crate::schemas::o::BottomStroke>),
  ColumnStroke(std::boxed::Box<crate::schemas::o::ColumnStroke>),
  ClipPath(std::boxed::Box<crate::schemas::o::ClipPath>),
  FillExtendedProperties(std::boxed::Box<crate::schemas::o::FillExtendedProperties>),
  TopBorder(std::boxed::Box<crate::schemas::w10::TopBorder>),
  LeftBorder(std::boxed::Box<crate::schemas::w10::LeftBorder>),
  RightBorder(std::boxed::Box<crate::schemas::w10::RightBorder>),
  BottomBorder(std::boxed::Box<crate::schemas::w10::BottomBorder>),
  TextWrap(std::boxed::Box<crate::schemas::w10::TextWrap>),
  /// Anchor Location Is Locked.
  AnchorLock,
  ClientData(std::boxed::Box<crate::schemas::xvml::ClientData>),
  /// Ink Annotation Flag.
  InkAnnotationFlag,
  TextData(std::boxed::Box<crate::schemas::pvml::TextData>),
  WordprocessingCanvas(std::boxed::Box<crate::schemas::wpc::WordprocessingCanvas>),
  WordprocessingGroup(std::boxed::Box<crate::schemas::wpg::WordprocessingGroup>),
  WordprocessingShape(std::boxed::Box<crate::schemas::wps::WordprocessingShape>),
  Slicer(std::boxed::Box<crate::schemas::sle::Slicer>),
  ColorStyle(std::boxed::Box<crate::schemas::cs::ColorStyle>),
  ChartStyle(std::boxed::Box<crate::schemas::cs::ChartStyle>),
  WebExtension(std::boxed::Box<crate::schemas::we::WebExtension>),
  WebExtensionReference(std::boxed::Box<crate::schemas::we::WebExtensionReference>),
  TimeSlicer(std::boxed::Box<crate::schemas::tsle::TimeSlicer>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextShapeChoice {
  /// Use Shape Text Rectangle.
  UseShapeRectangle,
  /// Defines the Transform2D Class.
  Transform2D(std::boxed::Box<Transform2D>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GradientStopChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PathChoice {
  /// Close Shape Path.
  CloseShapePath,
  /// Move Path To.
  MoveTo(std::boxed::Box<MoveTo>),
  /// Draw Line To.
  LineTo(std::boxed::Box<LineTo>),
  /// Draw Arc To.
  ArcTo(std::boxed::Box<ArcTo>),
  /// Draw Quadratic Bezier Curve To.
  QuadraticBezierCurveTo(std::boxed::Box<QuadraticBezierCurveTo>),
  /// Draw Cubic Bezier Curve To.
  CubicBezierCurveTo(std::boxed::Box<CubicBezierCurveTo>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum AdjustHandleListChoice {
  /// XY Adjust Handle.
  AdjustHandleXy(std::boxed::Box<AdjustHandleXy>),
  /// Polar Adjust Handle.
  AdjustHandlePolar(std::boxed::Box<AdjustHandlePolar>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BodyPropertiesChoice {
  /// No AutoFit.
  NoAutoFit,
  /// Normal AutoFit.
  NormalAutoFit(std::boxed::Box<NormalAutoFit>),
  /// Shape AutoFit.
  ShapeAutoFit,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  Shape3DType(std::boxed::Box<Shape3DType>),
  /// No text in 3D scene.
  FlatText(std::boxed::Box<FlatText>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TablePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TablePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TablePropertiesChoice3 {
  /// Table Style.
  TableStyle(std::boxed::Box<TableStyle>),
  /// Table Style ID.
  TableStyleId(TableStyleId),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LeftBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RightBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BottomBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum InsideHorizontalBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum InsideVerticalBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopLeftToBottomRightBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TopRightToBottomLeftBorderChoice {
  /// Defines the Outline Class.
  Outline(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  LineReference(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellTextStyleChoice {
  /// Font.
  Fonts(std::boxed::Box<Fonts>),
  /// Defines the FontReference Class.
  FontReference(std::boxed::Box<FontReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellTextStyleChoice2 {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableCellStyleChoice {
  /// Fill.
  FillProperties(std::boxed::Box<FillProperties>),
  /// Fill Reference.
  FillReference(std::boxed::Box<FillReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableBackgroundChoice {
  /// Fill.
  FillProperties(std::boxed::Box<FillProperties>),
  /// Fill Reference.
  FillReference(std::boxed::Box<FillReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TableBackgroundChoice2 {
  /// Effect.
  EffectPropertiesType(std::boxed::Box<EffectPropertiesType>),
  /// Effect Reference.
  EffectReference(std::boxed::Box<EffectReference>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level1ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level1ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level1ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level1ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level2ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level2ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level2ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level2ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level3ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level3ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level3ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level3ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level4ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level4ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level4ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level4ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level5ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level5ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level5ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level5ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level6ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level6ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level6ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level6ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level7ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level7ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level7ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level7ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level8ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level8ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level8ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level8ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level9ParagraphPropertiesChoice {
  /// Follow Text.
  BulletColorText,
  /// Color Specified.
  BulletColor(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level9ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  BulletSizeText,
  /// Bullet Size Percentage.
  BulletSizePercentage(std::boxed::Box<BulletSizePercentage>),
  /// Bullet Size Points.
  BulletSizePoints(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level9ParagraphPropertiesChoice3 {
  /// Follow text.
  BulletFontText,
  /// Specified.
  BulletFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum Level9ParagraphPropertiesChoice4 {
  /// No Bullet.
  NoBullet,
  /// Auto-Numbered Bullet.
  AutoNumberedBullet(std::boxed::Box<AutoNumberedBullet>),
  /// Character Bullet.
  CharacterBullet(std::boxed::Box<CharacterBullet>),
  /// Picture Bullet.
  PictureBullet(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndParagraphRunPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndParagraphRunPropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndParagraphRunPropertiesChoice3 {
  /// Underline Follows Text.
  UnderlineFollowsText,
  /// Underline Stroke.
  Underline(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum EndParagraphRunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  UnderlineFillText,
  /// Underline Fill.
  UnderlineFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice3 {
  /// Underline Follows Text.
  UnderlineFollowsText,
  /// Underline Stroke.
  Underline(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  UnderlineFillText,
  /// Underline Fill.
  UnderlineFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultRunPropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultRunPropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultRunPropertiesChoice3 {
  /// Underline Follows Text.
  UnderlineFollowsText,
  /// Underline Stroke.
  Underline(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DefaultRunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  UnderlineFillText,
  /// Underline Fill.
  UnderlineFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphChoice {
  /// Text Run.
  Run(std::boxed::Box<Run>),
  /// Text Line Break.
  Break(std::boxed::Box<Break>),
  /// Text Field.
  Field(std::boxed::Box<Field>),
  TextMath(std::boxed::Box<crate::schemas::a14::TextMath>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LineSpacingChoice {
  /// Spacing Percent.
  SpacingPercent(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  SpacingPoints(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SpaceBeforeChoice {
  /// Spacing Percent.
  SpacingPercent(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  SpacingPoints(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SpaceAfterChoice {
  /// Spacing Percent.
  SpacingPercent(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  SpacingPoints(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesExtensionChoice {
  /// Defines the HiddenFillProperties Class.
  HiddenFillProperties(std::boxed::Box<crate::schemas::a14::HiddenFillProperties>),
  /// Defines the HiddenLineProperties Class.
  HiddenLineProperties(std::boxed::Box<crate::schemas::a14::HiddenLineProperties>),
  /// Defines the HiddenEffectsProperties Class.
  HiddenEffectsProperties(std::boxed::Box<crate::schemas::a14::HiddenEffectsProperties>),
  /// Defines the HiddenScene3D Class.
  HiddenScene3D(std::boxed::Box<crate::schemas::a14::HiddenScene3D>),
  /// Defines the HiddenShape3D Class.
  HiddenShape3D(std::boxed::Box<crate::schemas::a14::HiddenShape3D>),
  /// Defines the ShadowObscured Class.
  ShadowObscured(std::boxed::Box<crate::schemas::a14::ShadowObscured>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GvmlGroupShapeExtensionChoice {
  /// Defines the IsCanvas Class.
  IsCanvas(std::boxed::Box<crate::schemas::a14::IsCanvas>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum VisualGroupShapePropertiesChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum VisualGroupShapePropertiesChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GroupShapeChoice {
  /// Text Shape.
  TextShape(std::boxed::Box<TextShape>),
  /// Shape.
  Shape(std::boxed::Box<Shape>),
  /// Connection Shape.
  ConnectionShape(std::boxed::Box<ConnectionShape>),
  /// Picture.
  Picture(std::boxed::Box<Picture>),
  GvmlContentPart(std::boxed::Box<crate::schemas::a14::GvmlContentPart>),
  /// Graphic Frame.
  GraphicFrame(std::boxed::Box<GraphicFrame>),
  /// Group shape.
  GroupShape(std::boxed::Box<GroupShape>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum NonVisualGroupDrawingShapePropsExtensionChoice {
  /// Defines the NonVisualGroupProperties Class.
  NonVisualGroupProperties(std::boxed::Box<crate::schemas::a15::NonVisualGroupProperties>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum OfficeStyleSheetExtensionChoice {
  /// Defines the ThemeFamily Class.
  ThemeFamily(std::boxed::Box<crate::schemas::thm15::ThemeFamily>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectorLockingExtensionChoice {
  /// Graphic Object.
  Graphic(std::boxed::Box<Graphic>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DataModelExtensionChoice {
  /// Defines the DataModelExtensionBlock Class.
  DataModelExtensionBlock(std::boxed::Box<crate::schemas::dsp::DataModelExtensionBlock>),
  /// Defines the RecolorImages Class.
  RecolorImages(std::boxed::Box<crate::schemas::dgm14::RecolorImages>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum PtExtensionChoice {
  /// Defines the NonVisualDrawingProperties Class.
  NonVisualDrawingProperties(std::boxed::Box<crate::schemas::dgm14::NonVisualDrawingProperties>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum HyperlinkExtensionChoice {
  /// Defines the HyperlinkColor Class.
  HyperlinkColor(std::boxed::Box<crate::schemas::ahyp::HyperlinkColor>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum LinePropertiesExtensionChoice {
  /// Defines the LineSketchStyleProperties Class.
  LineSketchStyleProperties(std::boxed::Box<crate::schemas::ask::LineSketchStyleProperties>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum NonVisualDrawingPropertiesExtensionChoice {
  /// Defines the CompatExtension Class.
  CompatExtension(std::boxed::Box<crate::schemas::a14::CompatExtension>),
  /// Defines the BackgroundProperties Class.
  BackgroundProperties(std::boxed::Box<crate::schemas::a15::BackgroundProperties>),
  /// Defines the CreationId Class.
  CreationId(std::boxed::Box<crate::schemas::a16::CreationId>),
  /// Defines the PredecessorDrawingElementReference Class.
  PredecessorDrawingElementReference(
    std::boxed::Box<crate::schemas::a16::PredecessorDrawingElementReference>,
  ),
  /// Defines the Decorative Class.
  Decorative(std::boxed::Box<crate::schemas::adec::Decorative>),
  /// Defines the ClassificationOutcome Class.
  ClassificationOutcome(std::boxed::Box<crate::schemas::aclsh::ClassificationOutcome>),
  /// Defines the ScriptLink Class.
  ScriptLink(std::boxed::Box<crate::schemas::asl::ScriptLink>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum NonVisualPicturePropertiesExtensionChoice {
  /// Defines the CameraTool Class.
  CameraTool(std::boxed::Box<crate::schemas::a14::CameraTool>),
  /// Defines the SignatureLine Class.
  SignatureLine(std::boxed::Box<crate::schemas::a15::SignatureLine>),
  /// Defines the ObjectProperties Class.
  ObjectProperties(std::boxed::Box<crate::schemas::a15::ObjectProperties>),
  /// Defines the LiveFeedProperties Class.
  LiveFeedProperties(std::boxed::Box<crate::schemas::alf::LiveFeedProperties>),
  /// Defines the ImageFormula Class.
  ImageFormula(std::boxed::Box<crate::schemas::aif::ImageFormula>),
  XmlAny(std::boxed::Box<[u8]>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BlipExtensionChoice {
  /// Defines the ImageProperties Class.
  ImageProperties(std::boxed::Box<crate::schemas::a14::ImageProperties>),
  /// Defines the UseLocalDpi Class.
  UseLocalDpi(std::boxed::Box<crate::schemas::a14::UseLocalDpi>),
  /// Defines the WebVideoProperty Class.
  WebVideoProperty(std::boxed::Box<crate::schemas::wp15::WebVideoProperty>),
  /// Defines the SVGBlip Class.
  SvgBlip(std::boxed::Box<crate::schemas::asvg::SvgBlip>),
  /// Defines the PictureAttributionSourceURL Class.
  PictureAttributionSourceUrl(std::boxed::Box<crate::schemas::a1611::PictureAttributionSourceUrl>),
  /// Defines the OEmbed Class.
  OEmbed(std::boxed::Box<crate::schemas::woe::OEmbed>),
  /// Defines the OEmbedShared Class.
  OEmbedShared(std::boxed::Box<crate::schemas::aoe::OEmbedShared>),
  XmlAny(std::boxed::Box<[u8]>),
}
