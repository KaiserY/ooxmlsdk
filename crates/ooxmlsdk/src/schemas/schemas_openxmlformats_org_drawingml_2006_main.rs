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
#[sdk(qname = "a:CT_AudioCD/a:audioCd")]
pub struct AudioFromCd {
  /// Audio Start Time
  #[sdk(child(qname = "a:CT_AudioCDTime/a:st"))]
  pub start_time: std::boxed::Box<StartTime>,
  /// Audio End Time
  #[sdk(child(qname = "a:CT_AudioCDTime/a:end"))]
  pub end_time: std::boxed::Box<EndTime>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Audio from WAV File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile")]
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
#[sdk(qname = "a:CT_EmbeddedWAVAudioFile/a:snd")]
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
#[sdk(qname = "a:CT_AudioFile/a:audioFile")]
pub struct AudioFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Video from File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_VideoFile/a:videoFile")]
pub struct VideoFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// QuickTime from File.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_QuickTimeFile/a:quickTimeFile")]
pub struct QuickTimeFromFile {
  /// Linked Relationship ID
  #[sdk(attr(qname = "r:link"))]
  pub link: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Tint.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveFixedPercentage/a:tint")]
pub struct Tint {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Shade.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveFixedPercentage/a:shade")]
pub struct Shade {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Alpha.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveFixedPercentage/a:alpha")]
pub struct Alpha {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Alpha Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FixedPercentage/a:alphaOff")]
pub struct AlphaOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Alpha Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositivePercentage/a:alphaMod")]
pub struct AlphaModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Hue Modulate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositivePercentage/a:hueMod")]
pub struct HueModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Hue.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveFixedAngle/a:hue")]
pub struct Hue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Hue Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Angle/a:hueOff")]
pub struct HueOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Saturation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:sat")]
pub struct Saturation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Saturation Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:satOff")]
pub struct SaturationOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Saturation Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:satMod")]
pub struct SaturationModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Luminance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:lum")]
pub struct Luminance {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Luminance Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:lumOff")]
pub struct LuminanceOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Luminance Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:lumMod")]
pub struct LuminanceModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Red.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:red")]
pub struct Red {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Red Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:redOff")]
pub struct RedOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Red Modulation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:redMod")]
pub struct RedModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Green.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:green")]
pub struct Green {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Green Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:greenOff")]
pub struct GreenOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Green Modification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:greenMod")]
pub struct GreenModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Blue.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:blue")]
pub struct Blue {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Blue Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:blueOff")]
pub struct BlueOffset {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Blue Modification.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Percentage/a:blueMod")]
pub struct BlueModulation {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtension/a:ext")]
pub struct Extension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// RGB Color Model - Percentage Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ScRgbColor/a:scrgbClr")]
pub struct RgbColorModelPercentage {
  /// Red
  #[sdk(attr(qname = ":r"))]
  pub red_portion: crate::simple_type::Int32Value,
  /// Green
  #[sdk(attr(qname = ":g"))]
  pub green_portion: crate::simple_type::Int32Value,
  /// Blue
  #[sdk(attr(qname = ":b"))]
  pub blue_portion: crate::simple_type::Int32Value,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub rgb_color_model_percentage_choice: Vec<RgbColorModelPercentageChoice>,
}
/// RGB Color Model - Hex Variant.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SRgbColor/a:srgbClr")]
pub struct RgbColorModelHex {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_length(source = 1u32, min = 3u32, max = 3u32))]
  pub val: crate::simple_type::HexBinaryValue,
  /// legacySpreadsheetColorIndex
  #[sdk(attr(office2010, qname = "a14:legacySpreadsheetColorIndex"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "80",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub legacy_spreadsheet_color_index: Option<crate::simple_type::Int32Value>,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub rgb_color_model_hex_choice: Vec<RgbColorModelHexChoice>,
}
/// Hue, Saturation, Luminance Color Model.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_HslColor/a:hslClr")]
pub struct HslColor {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub hue_value: crate::simple_type::Int32Value,
  /// Saturation
  #[sdk(attr(qname = ":sat"))]
  pub sat_value: crate::simple_type::Int32Value,
  /// Luminance
  #[sdk(attr(qname = ":lum"))]
  pub lum_value: crate::simple_type::Int32Value,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub hsl_color_choice: Vec<HslColorChoice>,
}
/// System Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SystemColor/a:sysClr")]
pub struct SystemColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub val: SystemColorValues,
  /// Last Color
  #[sdk(attr(qname = ":lastClr"))]
  #[sdk(string_length(source = 0u32, min = 3u32, max = 3u32))]
  pub last_color: Option<crate::simple_type::HexBinaryValue>,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub system_color_choice: Vec<SystemColorChoice>,
}
/// Scheme Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SchemeColor/a:schemeClr")]
pub struct SchemeColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub val: SchemeColorValues,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub scheme_color_choice: Vec<SchemeColorChoice>,
}
/// Preset Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PresetColor/a:prstClr")]
pub struct PresetColor {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub val: PresetColorValues,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub preset_color_choice: Vec<PresetColorChoice>,
}
/// Apply 3D shape properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Shape3D/a:sp3d")]
pub struct Shape3DType {
  /// Shape Depth
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: Option<crate::simple_type::Int64Value>,
  /// Extrusion Height
  #[sdk(attr(qname = ":extrusionH"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub extrusion_height: Option<crate::simple_type::Int64Value>,
  /// Contour Width
  #[sdk(attr(qname = ":contourW"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub contour_width: Option<crate::simple_type::Int64Value>,
  /// Preset Material Type
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset_material: Option<PresetMaterialTypeValues>,
  /// Top Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelT"))]
  pub bevel_top: Option<BevelTop>,
  /// Bottom Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelB"))]
  pub bevel_bottom: Option<BevelBottom>,
  /// Extrusion Color
  #[sdk(child(qname = "a:CT_Color/a:extrusionClr"))]
  pub extrusion_color: Option<std::boxed::Box<ExtrusionColor>>,
  /// Contour Color
  #[sdk(child(qname = "a:CT_Color/a:contourClr"))]
  pub contour_color: Option<std::boxed::Box<ContourColor>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// No text in 3D scene.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FlatText/a:flatTx")]
pub struct FlatText {
  /// Z Coordinate
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: Option<crate::simple_type::Int64Value>,
}
/// Linear Gradient Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LinearShadeProperties/a:lin")]
pub struct LinearGradientFill {
  /// Angle
  #[sdk(attr(qname = ":ang"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub angle: Option<crate::simple_type::Int32Value>,
  /// Scaled
  #[sdk(attr(qname = ":scaled"))]
  pub scaled: Option<crate::simple_type::BooleanValue>,
}
/// Path Gradient.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PathShadeProperties/a:path")]
pub struct PathGradientFill {
  /// Gradient Fill Path
  #[sdk(attr(qname = ":path"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub path: Option<PathShadeValues>,
  /// Fill To Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:fillToRect"))]
  pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Tile.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TileInfoProperties/a:tile")]
pub struct Tile {
  /// Horizontal Offset
  #[sdk(attr(qname = ":tx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub horizontal_offset: Option<crate::simple_type::Int64Value>,
  /// Vertical Offset
  #[sdk(attr(qname = ":ty"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub vertical_offset: Option<crate::simple_type::Int64Value>,
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::Int32Value>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::Int32Value>,
  /// Tile Flipping
  #[sdk(attr(qname = ":flip"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub flip: Option<TileFlipValues>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
}
/// Stretch.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StretchInfoProperties/a:stretch")]
pub struct Stretch {
  /// Fill Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:fillRect"))]
  pub fill_rectangle: Option<FillRectangle>,
}
/// Defines the NoFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NoFillProperties/a:noFill")]
pub struct NoFill {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
}
/// Defines the SolidFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SolidColorFillProperties/a:solidFill")]
pub struct SolidFill {
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
  pub xml_children: Option<SolidFillChoice>,
}
/// Defines the GradientFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GradientFillProperties/a:gradFill")]
pub struct GradientFill {
  /// Tile Flip
  #[sdk(attr(qname = ":flip"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub flip: Option<TileFlipValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// Gradient Stop List
  #[sdk(child(qname = "a:CT_GradientStopList/a:gsLst"))]
  pub gradient_stop_list: Option<GradientStopList>,
  #[sdk(choice(
    qname = "a:CT_LinearShadeProperties/a:lin",
    qname = "a:CT_PathShadeProperties/a:path"
  ))]
  pub gradient_fill_choice: Option<GradientFillChoice>,
  /// _
  #[sdk(child(qname = "a:CT_RelativeRect/a:tileRect"))]
  pub a_tile_rect: Option<TileRectangle>,
}
/// Defines the BlipFill Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipFillProperties/a:blipFill")]
pub struct BlipFill {
  /// DPI Setting
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip: Option<std::boxed::Box<Blip>>,
  /// Source Rectangle
  #[sdk(child(qname = "a:CT_RelativeRect/a:srcRect"))]
  pub source_rectangle: Option<SourceRectangle>,
  #[sdk(choice(
    qname = "a:CT_TileInfoProperties/a:tile",
    qname = "a:CT_StretchInfoProperties/a:stretch"
  ))]
  pub blip_fill_choice: Option<BlipFillChoice>,
}
/// Pattern Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PatternFillProperties/a:pattFill")]
pub struct PatternFill {
  /// Preset Pattern
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset: Option<PresetPatternValues>,
  /// Foreground color
  #[sdk(child(qname = "a:CT_Color/a:fgClr"))]
  pub foreground_color: Option<std::boxed::Box<ForegroundColor>>,
  /// Background color
  #[sdk(child(qname = "a:CT_Color/a:bgClr"))]
  pub background_color: Option<std::boxed::Box<BackgroundColor>>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectContainer/a:cont")]
pub struct EffectContainer {
  /// Effect Container Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<EffectContainerValues>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "a:CT_EffectContainer/a:cont",
    qname = "a:CT_EffectReference/a:effect",
    qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel",
    qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling",
    qname = "a:CT_AlphaFloorEffect/a:alphaFloor",
    qname = "a:CT_AlphaInverseEffect/a:alphaInv",
    qname = "a:CT_AlphaModulateEffect/a:alphaMod",
    qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix",
    qname = "a:CT_AlphaOutsetEffect/a:alphaOutset",
    qname = "a:CT_AlphaReplaceEffect/a:alphaRepl",
    qname = "a:CT_BiLevelEffect/a:biLevel",
    qname = "a:CT_BlendEffect/a:blend",
    qname = "a:CT_BlurEffect/a:blur",
    qname = "a:CT_ColorChangeEffect/a:clrChange",
    qname = "a:CT_ColorReplaceEffect/a:clrRepl",
    qname = "a:CT_DuotoneEffect/a:duotone",
    qname = "a:CT_FillEffect/a:fill",
    qname = "a:CT_FillOverlayEffect/a:fillOverlay",
    qname = "a:CT_GlowEffect/a:glow",
    qname = "a:CT_GrayscaleEffect/a:grayscl",
    qname = "a:CT_HSLEffect/a:hsl",
    qname = "a:CT_InnerShadowEffect/a:innerShdw",
    qname = "a:CT_LuminanceEffect/a:lum",
    qname = "a:CT_OuterShadowEffect/a:outerShdw",
    qname = "a:CT_PresetShadowEffect/a:prstShdw",
    qname = "a:CT_ReflectionEffect/a:reflection",
    qname = "a:CT_RelativeOffsetEffect/a:relOff",
    qname = "a:CT_SoftEdgesEffect/a:softEdge",
    qname = "a:CT_TintEffect/a:tint",
    qname = "a:CT_TransformEffect/a:xfrm"
  ))]
  pub xml_children: Vec<EffectContainerChoice>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectContainer/a:effectDag")]
pub struct EffectDag {
  /// Effect Container Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<EffectContainerValues>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "a:CT_EffectContainer/a:cont",
    qname = "a:CT_EffectReference/a:effect",
    qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel",
    qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling",
    qname = "a:CT_AlphaFloorEffect/a:alphaFloor",
    qname = "a:CT_AlphaInverseEffect/a:alphaInv",
    qname = "a:CT_AlphaModulateEffect/a:alphaMod",
    qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix",
    qname = "a:CT_AlphaOutsetEffect/a:alphaOutset",
    qname = "a:CT_AlphaReplaceEffect/a:alphaRepl",
    qname = "a:CT_BiLevelEffect/a:biLevel",
    qname = "a:CT_BlendEffect/a:blend",
    qname = "a:CT_BlurEffect/a:blur",
    qname = "a:CT_ColorChangeEffect/a:clrChange",
    qname = "a:CT_ColorReplaceEffect/a:clrRepl",
    qname = "a:CT_DuotoneEffect/a:duotone",
    qname = "a:CT_FillEffect/a:fill",
    qname = "a:CT_FillOverlayEffect/a:fillOverlay",
    qname = "a:CT_GlowEffect/a:glow",
    qname = "a:CT_GrayscaleEffect/a:grayscl",
    qname = "a:CT_HSLEffect/a:hsl",
    qname = "a:CT_InnerShadowEffect/a:innerShdw",
    qname = "a:CT_LuminanceEffect/a:lum",
    qname = "a:CT_OuterShadowEffect/a:outerShdw",
    qname = "a:CT_PresetShadowEffect/a:prstShdw",
    qname = "a:CT_ReflectionEffect/a:reflection",
    qname = "a:CT_RelativeOffsetEffect/a:relOff",
    qname = "a:CT_SoftEdgesEffect/a:softEdge",
    qname = "a:CT_TintEffect/a:tint",
    qname = "a:CT_TransformEffect/a:xfrm"
  ))]
  pub xml_children: Vec<EffectDagChoice>,
}
/// Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectReference/a:effect")]
pub struct Effect {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub reference: Option<crate::simple_type::StringValue>,
}
/// Defines the AlphaBiLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel")]
pub struct AlphaBiLevel {
  /// Threshold
  #[sdk(attr(qname = ":thresh"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub threshold: crate::simple_type::Int32Value,
}
/// Alpha Inverse Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaInverseEffect/a:alphaInv")]
pub struct AlphaInverse {
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
  pub xml_children: Option<AlphaInverseChoice>,
}
/// Alpha Modulate Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaModulateEffect/a:alphaMod")]
pub struct AlphaModulationEffect {
  /// _
  #[sdk(child(qname = "a:CT_EffectContainer/a:cont"))]
  pub effect_container: std::boxed::Box<EffectContainer>,
}
/// Defines the AlphaModulationFixed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix")]
pub struct AlphaModulationFixed {
  /// Amount
  #[sdk(attr(qname = ":amt"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub amount: Option<crate::simple_type::Int32Value>,
}
/// Alpha Inset/Outset Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaOutsetEffect/a:alphaOutset")]
pub struct AlphaOutset {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub radius: Option<crate::simple_type::Int64Value>,
}
/// Alpha Replace Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl")]
pub struct AlphaReplace {
  /// Alpha
  #[sdk(attr(qname = ":a"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub alpha: crate::simple_type::Int32Value,
}
/// Defines the BiLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BiLevelEffect/a:biLevel")]
pub struct BiLevel {
  /// Threshold
  #[sdk(attr(qname = ":thresh"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub threshold: crate::simple_type::Int32Value,
}
/// Blend Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlendEffect/a:blend")]
pub struct Blend {
  /// Blend Mode
  #[sdk(attr(qname = ":blend"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub blend_mode: BlendModeValues,
  /// Effect to blend
  #[sdk(child(qname = "a:CT_EffectContainer/a:cont"))]
  pub effect_container: std::boxed::Box<EffectContainer>,
}
/// Defines the Blur Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlurEffect/a:blur")]
pub struct Blur {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub radius: Option<crate::simple_type::Int64Value>,
  /// Grow Bounds
  #[sdk(attr(qname = ":grow"))]
  pub grow: Option<crate::simple_type::BooleanValue>,
}
/// Color Change Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorChangeEffect/a:clrChange")]
pub struct ColorChange {
  /// Consider Alpha Values
  #[sdk(attr(qname = ":useA"))]
  pub use_alpha: Option<crate::simple_type::BooleanValue>,
  /// Change Color From
  #[sdk(child(qname = "a:CT_Color/a:clrFrom"))]
  pub color_from: std::boxed::Box<ColorFrom>,
  /// Change Color To
  #[sdk(child(qname = "a:CT_Color/a:clrTo"))]
  pub color_to: std::boxed::Box<ColorTo>,
}
/// Defines the ColorReplacement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorReplaceEffect/a:clrRepl")]
pub struct ColorReplacement {
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
  pub xml_children: Option<ColorReplacementChoice>,
}
/// Duotone Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DuotoneEffect/a:duotone")]
pub struct Duotone {
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
  pub duotone_choice: Vec<DuotoneChoice>,
}
/// Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FillEffect/a:fill")]
pub struct Fill {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub xml_children: Option<FillChoice>,
}
/// Fill Overlay Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FillOverlayEffect/a:fillOverlay")]
pub struct FillOverlay {
  /// Blend
  #[sdk(attr(qname = ":blend"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub blend: BlendModeValues,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub xml_children: Option<FillOverlayChoice>,
}
/// Glow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GlowEffect/a:glow")]
pub struct Glow {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub radius: Option<crate::simple_type::Int64Value>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<GlowChoice>,
}
/// Hue Saturation Luminance Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_HSLEffect/a:hsl")]
pub struct Hsl {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub hue: Option<crate::simple_type::Int32Value>,
  /// Saturation
  #[sdk(attr(qname = ":sat"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub saturation: Option<crate::simple_type::Int32Value>,
  /// Luminance
  #[sdk(attr(qname = ":lum"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub luminance: Option<crate::simple_type::Int32Value>,
}
/// Inner Shadow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_InnerShadowEffect/a:innerShdw")]
pub struct InnerShadow {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub blur_radius: Option<crate::simple_type::Int64Value>,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub distance: Option<crate::simple_type::Int64Value>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub direction: Option<crate::simple_type::Int32Value>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<InnerShadowChoice>,
}
/// Luminance.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LuminanceEffect/a:lum")]
pub struct LuminanceEffect {
  /// Brightness
  #[sdk(attr(qname = ":bright"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub brightness: Option<crate::simple_type::Int32Value>,
  /// Contrast
  #[sdk(attr(qname = ":contrast"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub contrast: Option<crate::simple_type::Int32Value>,
}
/// Outer Shadow Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OuterShadowEffect/a:outerShdw")]
pub struct OuterShadow {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub blur_radius: Option<crate::simple_type::Int64Value>,
  /// Shadow Offset Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub distance: Option<crate::simple_type::Int64Value>,
  /// Shadow Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub direction: Option<crate::simple_type::Int32Value>,
  /// Horizontal Scaling Factor
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::Int32Value>,
  /// Vertical Scaling Factor
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::Int32Value>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Shadow Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<OuterShadowChoice>,
}
/// Preset Shadow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PresetShadowEffect/a:prstShdw")]
pub struct PresetShadow {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Preset Shadow
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub preset: PresetShadowValues,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub distance: Option<crate::simple_type::Int64Value>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub direction: Option<crate::simple_type::Int32Value>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<PresetShadowChoice>,
}
/// Reflection Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ReflectionEffect/a:reflection")]
pub struct Reflection {
  /// Blur Radius
  #[sdk(attr(qname = ":blurRad"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub blur_radius: Option<crate::simple_type::Int64Value>,
  /// Start Opacity
  #[sdk(attr(qname = ":stA"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub start_opacity: Option<crate::simple_type::Int32Value>,
  /// Start Position
  #[sdk(attr(qname = ":stPos"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub start_position: Option<crate::simple_type::Int32Value>,
  /// End Alpha
  #[sdk(attr(qname = ":endA"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub end_alpha: Option<crate::simple_type::Int32Value>,
  /// End Position
  #[sdk(attr(qname = ":endPos"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub end_position: Option<crate::simple_type::Int32Value>,
  /// Distance
  #[sdk(attr(qname = ":dist"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub distance: Option<crate::simple_type::Int64Value>,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub direction: Option<crate::simple_type::Int32Value>,
  /// Fade Direction
  #[sdk(attr(qname = ":fadeDir"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub fade_direction: Option<crate::simple_type::Int32Value>,
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::Int32Value>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::Int32Value>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Shadow Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<RectangleAlignmentValues>,
  /// Rotate With Shape
  #[sdk(attr(qname = ":rotWithShape"))]
  pub rotate_with_shape: Option<crate::simple_type::BooleanValue>,
}
/// Relative Offset Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RelativeOffsetEffect/a:relOff")]
pub struct RelativeOffset {
  /// Offset X
  #[sdk(attr(qname = ":tx"))]
  pub offset_x: Option<crate::simple_type::Int32Value>,
  /// Offset Y
  #[sdk(attr(qname = ":ty"))]
  pub offset_y: Option<crate::simple_type::Int32Value>,
}
/// Soft Edge Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SoftEdgesEffect/a:softEdge")]
pub struct SoftEdge {
  /// Radius
  #[sdk(attr(qname = ":rad"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub radius: crate::simple_type::Int64Value,
}
/// Defines the TintEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TintEffect/a:tint")]
pub struct TintEffect {
  /// Hue
  #[sdk(attr(qname = ":hue"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub hue: Option<crate::simple_type::Int32Value>,
  /// Amount
  #[sdk(attr(qname = ":amt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub amount: Option<crate::simple_type::Int32Value>,
}
/// Transform Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TransformEffect/a:xfrm")]
pub struct TransformEffect {
  /// Horizontal Ratio
  #[sdk(attr(qname = ":sx"))]
  pub horizontal_ratio: Option<crate::simple_type::Int32Value>,
  /// Vertical Ratio
  #[sdk(attr(qname = ":sy"))]
  pub vertical_ratio: Option<crate::simple_type::Int32Value>,
  /// Horizontal Skew
  #[sdk(attr(qname = ":kx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub horizontal_skew: Option<crate::simple_type::Int32Value>,
  /// Vertical Skew
  #[sdk(attr(qname = ":ky"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false
  ))]
  pub vertical_skew: Option<crate::simple_type::Int32Value>,
  /// Horizontal Shift
  #[sdk(attr(qname = ":tx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub horizontal_shift: Option<crate::simple_type::Int64Value>,
  /// Vertical Shift
  #[sdk(attr(qname = ":ty"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub vertical_shift: Option<crate::simple_type::Int64Value>,
}
/// Effect Container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectList/a:effectLst")]
pub struct EffectList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Blur Effect
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  pub blur: Option<Blur>,
  /// _
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  pub fill_overlay: Option<std::boxed::Box<FillOverlay>>,
  /// _
  #[sdk(child(qname = "a:CT_GlowEffect/a:glow"))]
  pub glow: Option<std::boxed::Box<Glow>>,
  /// _
  #[sdk(child(qname = "a:CT_InnerShadowEffect/a:innerShdw"))]
  pub inner_shadow: Option<std::boxed::Box<InnerShadow>>,
  /// _
  #[sdk(child(qname = "a:CT_OuterShadowEffect/a:outerShdw"))]
  pub outer_shadow: Option<std::boxed::Box<OuterShadow>>,
  /// _
  #[sdk(child(qname = "a:CT_PresetShadowEffect/a:prstShdw"))]
  pub preset_shadow: Option<std::boxed::Box<PresetShadow>>,
  /// _
  #[sdk(child(qname = "a:CT_ReflectionEffect/a:reflection"))]
  pub reflection: Option<Reflection>,
  /// _
  #[sdk(child(qname = "a:CT_SoftEdgesEffect/a:softEdge"))]
  pub soft_edge: Option<SoftEdge>,
}
/// Custom geometry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_CustomGeometry2D/a:custGeom")]
pub struct CustomGeometry {
  /// Adjust Value List
  #[sdk(child(qname = "a:CT_GeomGuideList/a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
  /// List of Shape Guides
  #[sdk(child(qname = "a:CT_GeomGuideList/a:gdLst"))]
  pub shape_guide_list: Option<ShapeGuideList>,
  /// List of Shape Adjust Handles
  #[sdk(child(qname = "a:CT_AdjustHandleList/a:ahLst"))]
  pub adjust_handle_list: Option<AdjustHandleList>,
  /// List of Shape Connection Sites
  #[sdk(child(qname = "a:CT_ConnectionSiteList/a:cxnLst"))]
  pub connection_site_list: Option<ConnectionSiteList>,
  /// Shape Text Rectangle
  #[sdk(child(qname = "a:CT_GeomRect/a:rect"))]
  pub rectangle: Option<Rectangle>,
  /// List of Shape Paths
  #[sdk(child(qname = "a:CT_Path2DList/a:pathLst"))]
  pub path_list: std::boxed::Box<PathList>,
}
/// Preset geometry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PresetGeometry2D/a:prstGeom")]
pub struct PresetGeometry {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Preset Shape
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub preset: ShapeTypeValues,
  /// List of Shape Adjust Values
  #[sdk(child(qname = "a:CT_GeomGuideList/a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
}
/// Preset Text Warp.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PresetTextShape/a:prstTxWarp")]
pub struct PresetTextWarp {
  /// Preset Warp Shape
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub preset: TextShapeValues,
  /// Adjust Value List
  #[sdk(child(qname = "a:CT_GeomGuideList/a:avLst"))]
  pub adjust_value_list: Option<AdjustValueList>,
}
/// Miter Line Join.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineJoinMiterProperties/a:miter")]
pub struct Miter {
  /// Miter Join Limit
  #[sdk(attr(qname = ":lim"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub limit: Option<crate::simple_type::Int32Value>,
}
/// Preset Dash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PresetLineDashProperties/a:prstDash")]
pub struct PresetDash {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<PresetLineDashValues>,
}
/// Custom Dash.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DashStopList/a:custDash")]
pub struct CustomDash {
  /// _
  #[sdk(child(qname = "a:CT_DashStop/a:ds"))]
  pub a_ds: Vec<DashStop>,
}
/// Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FillProperties/a:fill")]
pub struct FillProperties {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub xml_children: Option<FillPropertiesChoice>,
}
/// Fill Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/a:fillRef")]
pub struct FillReference {
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
  pub xml_children: Option<FillReferenceChoice>,
}
/// Effect Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/a:effectRef")]
pub struct EffectReference {
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
  pub xml_children: Option<EffectReferenceChoice>,
}
/// Defines the LineReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrixReference/a:lnRef")]
pub struct LineReference {
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
  pub xml_children: Option<LineReferenceChoice>,
}
/// Effect.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectProperties/a:effect")]
pub struct EffectPropertiesType {
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub xml_children: Option<EffectPropertiesTypeChoice>,
}
/// Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontCollection/a:font")]
pub struct Fonts {
  /// Latin Font
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_SupplementalFont/a:font"))]
  pub a_font: Vec<SupplementalFont>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Major Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontCollection/a:majorFont")]
pub struct MajorFont {
  /// Latin Font
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_SupplementalFont/a:font"))]
  pub a_font: Vec<SupplementalFont>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Minor fonts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontCollection/a:minorFont")]
pub struct MinorFont {
  /// Latin Font
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub latin_font: std::boxed::Box<LatinFont>,
  /// East Asian Font
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub east_asian_font: std::boxed::Box<EastAsianFont>,
  /// Complex Script Font
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub complex_script_font: std::boxed::Box<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_SupplementalFont/a:font"))]
  pub a_font: Vec<SupplementalFont>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontReference/a:fontRef")]
pub struct FontReference {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Identifier
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub index: FontCollectionIndexValues,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<FontReferenceChoice>,
}
/// Normal AutoFit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextNormalAutofit/a:normAutofit")]
pub struct NormalAutoFit {
  /// Font Scale
  #[sdk(attr(qname = ":fontScale"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1000",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub font_scale: Option<crate::simple_type::Int32Value>,
  /// Line Space Reduction
  #[sdk(attr(qname = ":lnSpcReduction"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "13200000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub line_space_reduction: Option<crate::simple_type::Int32Value>,
}
/// Shape AutoFit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextShapeAutofit/a:spAutoFit")]
pub struct ShapeAutoFit {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
}
/// Color Specified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:buClr")]
pub struct BulletColor {
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
  pub xml_children: Option<BulletColorChoice>,
}
/// Extrusion Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:extrusionClr")]
pub struct ExtrusionColor {
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
  pub xml_children: Option<ExtrusionColorChoice>,
}
/// Contour Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:contourClr")]
pub struct ContourColor {
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
  pub xml_children: Option<ContourColorChoice>,
}
/// Change Color From.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:clrFrom")]
pub struct ColorFrom {
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
  pub xml_children: Option<ColorFromChoice>,
}
/// Change Color To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:clrTo")]
pub struct ColorTo {
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
  pub xml_children: Option<ColorToChoice>,
}
/// Foreground color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:fgClr")]
pub struct ForegroundColor {
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
  pub xml_children: Option<ForegroundColorChoice>,
}
/// Background color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:bgClr")]
pub struct BackgroundColor {
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
  pub xml_children: Option<BackgroundColorChoice>,
}
/// Defines the Highlight Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/a:highlight")]
pub struct Highlight {
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
  pub xml_children: Option<HighlightChoice>,
}
/// Bullet Size Percentage.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBulletSizePercent/a:buSzPct")]
pub struct BulletSizePercentage {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "25000",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Bullet Size Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBulletSizePoint/a:buSzPts")]
pub struct BulletSizePoints {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "100",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Specified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextFont/a:buFont")]
pub struct BulletFont {
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
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
#[sdk(qname = "a:CT_TextFont/a:latin")]
pub struct LatinFont {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
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
#[sdk(qname = "a:CT_TextFont/a:ea")]
pub struct EastAsianFont {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
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
#[sdk(qname = "a:CT_TextFont/a:cs")]
pub struct ComplexScriptFont {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
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
#[sdk(qname = "a:CT_TextFont/a:sym")]
pub struct SymbolFont {
  /// Text Typeface
  #[sdk(attr(qname = ":typeface"))]
  pub typeface: Option<crate::simple_type::StringValue>,
  /// Panose Setting
  #[sdk(attr(qname = ":panose"))]
  #[sdk(string_length(source = 0u32, min = 10u32, max = 10u32))]
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
#[sdk(qname = "a:CT_TextAutonumberBullet/a:buAutoNum")]
pub struct AutoNumberedBullet {
  /// Bullet Autonumbering Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: TextAutoNumberSchemeValues,
  /// Start Numbering At
  #[sdk(attr(qname = ":startAt"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "32767",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub start_at: Option<crate::simple_type::Int32Value>,
}
/// Character Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextCharBullet/a:buChar")]
pub struct CharacterBullet {
  /// Bullet Character
  #[sdk(attr(qname = ":char"))]
  pub char: crate::simple_type::StringValue,
}
/// Picture Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBlipBullet/a:buBlip")]
pub struct PictureBullet {
  /// Blip
  #[sdk(child(qname = "a:CT_Blip/a:blip"))]
  pub blip: std::boxed::Box<Blip>,
}
/// Underline Stroke.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:uLn")]
pub struct Underline {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub underline_choice1: Option<UnderlineChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub underline_choice2: Option<UnderlineChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub underline_choice3: Option<UnderlineChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:ln")]
pub struct Outline {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub outline_choice1: Option<OutlineChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub outline_choice2: Option<OutlineChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub outline_choice3: Option<OutlineChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Left Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnL")]
pub struct LeftBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub left_border_line_properties_choice1: Option<LeftBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub left_border_line_properties_choice2: Option<LeftBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub left_border_line_properties_choice3: Option<LeftBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnR")]
pub struct RightBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub right_border_line_properties_choice1: Option<RightBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub right_border_line_properties_choice2: Option<RightBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub right_border_line_properties_choice3: Option<RightBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Top Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnT")]
pub struct TopBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub top_border_line_properties_choice1: Option<TopBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub top_border_line_properties_choice2: Option<TopBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub top_border_line_properties_choice3: Option<TopBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Bottom Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnB")]
pub struct BottomBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub bottom_border_line_properties_choice1: Option<BottomBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub bottom_border_line_properties_choice2: Option<BottomBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub bottom_border_line_properties_choice3: Option<BottomBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Top-Left to Bottom-Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnTlToBr")]
pub struct TopLeftToBottomRightBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub top_left_to_bottom_right_border_line_properties_choice1:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub top_left_to_bottom_right_border_line_properties_choice2:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub top_left_to_bottom_right_border_line_properties_choice3:
    Option<TopLeftToBottomRightBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Bottom-Left to Top-Right Border Line Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineProperties/a:lnBlToTr")]
pub struct BottomLeftToTopRightBorderLineProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// line width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int32Value>,
  /// line cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub cap_type: Option<LineCapValues>,
  /// compound line type
  #[sdk(attr(qname = ":cmpd"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compound_line_type: Option<CompoundLineValues>,
  /// pen alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub bottom_left_to_top_right_border_line_properties_choice1:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_PresetLineDashProperties/a:prstDash",
    qname = "a:CT_DashStopList/a:custDash"
  ))]
  pub bottom_left_to_top_right_border_line_properties_choice2:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_LineJoinRound/a:round",
    qname = "a:CT_LineJoinBevel/a:bevel",
    qname = "a:CT_LineJoinMiterProperties/a:miter"
  ))]
  pub bottom_left_to_top_right_border_line_properties_choice3:
    Option<BottomLeftToTopRightBorderLinePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:headEnd"))]
  pub a_head_end: Option<HeadEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LineEndProperties/a:tailEnd"))]
  pub a_tail_end: Option<TailEnd>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<LinePropertiesExtensionList>,
}
/// Underline Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill")]
pub struct UnderlineFill {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub xml_children: Option<UnderlineFillChoice>,
}
/// Text Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RegularTextRun/a:r")]
pub struct Run {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Character Properties
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Text String
  #[sdk(text_child(qname = "xsd:string/a:t"))]
  pub text: crate::simple_type::StringValue,
}
/// Text Line Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextLineBreak/a:br")]
pub struct Break {
  /// Text Run Properties
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
}
/// Text Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextField/a:fld")]
pub struct Field {
  /// Field ID
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Field Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Text Character Properties
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:rPr"))]
  pub run_properties: Option<std::boxed::Box<RunProperties>>,
  /// Text Paragraph Properties
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  /// _
  #[sdk(text_child(qname = "xsd:string/a:t"))]
  pub text: Option<crate::simple_type::StringValue>,
}
/// Graphic Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GraphicalObject/a:graphic")]
pub struct Graphic {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Graphic Object Data
  #[sdk(child(qname = "a:CT_GraphicalObjectData/a:graphicData"))]
  pub graphic_data: std::boxed::Box<GraphicData>,
}
/// Defines the Blip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Blip/a:blip")]
pub struct Blip {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compression_state: Option<BlipCompressionValues>,
  #[sdk(choice(
    qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel",
    qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling",
    qname = "a:CT_AlphaFloorEffect/a:alphaFloor",
    qname = "a:CT_AlphaInverseEffect/a:alphaInv",
    qname = "a:CT_AlphaModulateEffect/a:alphaMod",
    qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix",
    qname = "a:CT_AlphaReplaceEffect/a:alphaRepl",
    qname = "a:CT_BiLevelEffect/a:biLevel",
    qname = "a:CT_BlurEffect/a:blur",
    qname = "a:CT_ColorChangeEffect/a:clrChange",
    qname = "a:CT_ColorReplaceEffect/a:clrRepl",
    qname = "a:CT_DuotoneEffect/a:duotone",
    qname = "a:CT_FillOverlayEffect/a:fillOverlay",
    qname = "a:CT_GrayscaleEffect/a:grayscl",
    qname = "a:CT_HSLEffect/a:hsl",
    qname = "a:CT_LuminanceEffect/a:lum",
    qname = "a:CT_TintEffect/a:tint"
  ))]
  pub blip_choice: Vec<BlipChoice>,
  /// _
  #[sdk(child(qname = "a:CT_BlipExtensionList/a:extLst"))]
  pub a_ext_lst: Option<BlipExtensionList>,
}
/// Theme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeStyleSheet/a:theme")]
pub struct Theme {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2013, qname = "thm15:id"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub theme_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_BaseStyles/a:themeElements"))]
  pub theme_elements: std::boxed::Box<ThemeElements>,
  /// _
  #[sdk(child(qname = "a:CT_ObjectStyleDefaults/a:objectDefaults"))]
  pub object_defaults: Option<std::boxed::Box<ObjectDefaults>>,
  /// _
  #[sdk(child(qname = "a:CT_ColorSchemeList/a:extraClrSchemeLst"))]
  pub extra_color_scheme_list: Option<ExtraColorSchemeList>,
  /// _
  #[sdk(child(qname = "a:CT_CustomColorList/a:custClrLst"))]
  pub custom_color_list: Option<CustomColorList>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeStyleSheetExtensionList/a:extLst"))]
  pub office_style_sheet_extension_list: Option<OfficeStyleSheetExtensionList>,
}
/// Theme Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BaseStylesOverride/a:themeOverride")]
pub struct ThemeOverride {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Scheme
  #[sdk(child(qname = "a:CT_ColorScheme/a:clrScheme"))]
  pub color_scheme: Option<std::boxed::Box<ColorScheme>>,
  /// _
  #[sdk(child(qname = "a:CT_FontScheme/a:fontScheme"))]
  pub font_scheme: Option<std::boxed::Box<FontScheme>>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrix/a:fmtScheme"))]
  pub format_scheme: Option<std::boxed::Box<FormatScheme>>,
}
/// Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Table/a:tbl")]
pub struct Table {
  /// Table Properties
  #[sdk(child(qname = "a:CT_TableProperties/a:tblPr"))]
  pub table_properties: Option<std::boxed::Box<TableProperties>>,
  /// Table Grid
  #[sdk(child(qname = "a:CT_TableGrid/a:tblGrid"))]
  pub table_grid: std::boxed::Box<TableGrid>,
  /// _
  #[sdk(child(qname = "a:CT_TableRow/a:tr"))]
  pub a_tr: Vec<TableRow>,
}
/// Table Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableStyleList/a:tblStyleLst")]
pub struct TableStyleList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Default
  #[sdk(attr(qname = ":def"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub default: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_TableStyle/a:tblStyle"))]
  pub a_tbl_style: Vec<TableStyleEntry>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/a:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<Extension>,
}
/// Audio Start Time.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AudioCDTime/a:st")]
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
#[sdk(qname = "a:CT_AudioCDTime/a:end")]
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
#[sdk(qname = "a:CT_CustomColor/a:custClr")]
pub struct CustomColor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<CustomColorChoice>,
}
/// Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SupplementalFont/a:font")]
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
#[sdk(qname = "a:CT_Scene3D/a:scene3d")]
pub struct Scene3DType {
  /// Camera
  #[sdk(child(qname = "a:CT_Camera/a:camera"))]
  pub camera: std::boxed::Box<Camera>,
  /// Light Rig
  #[sdk(child(qname = "a:CT_LightRig/a:lightRig"))]
  pub light_rig: std::boxed::Box<LightRig>,
  /// Backdrop Plane
  #[sdk(child(qname = "a:CT_Backdrop/a:backdrop"))]
  pub backdrop: Option<std::boxed::Box<Backdrop>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Effect Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectStyleItem/a:effectStyle")]
pub struct EffectStyle {
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub effect_style_choice: Option<EffectStyleChoice>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<Scene3DType>>,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<std::boxed::Box<Shape3DType>>,
}
/// Fill Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FillStyleList/a:fillStyleLst")]
pub struct FillStyleList {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub fill_style_list_choice: Vec<FillStyleListChoice>,
}
/// Line Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineStyleList/a:lnStyleLst")]
pub struct LineStyleList {
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Vec<Outline>,
}
/// Effect Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_EffectStyleList/a:effectStyleLst")]
pub struct EffectStyleList {
  /// _
  #[sdk(child(qname = "a:CT_EffectStyleItem/a:effectStyle"))]
  pub a_effect_style: Vec<EffectStyle>,
}
/// Background Fill Style List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BackgroundFillStyleList/a:bgFillStyleLst")]
pub struct BackgroundFillStyleList {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub background_fill_style_list_choice: Vec<BackgroundFillStyleListChoice>,
}
/// Defines the ColorScheme Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorScheme/a:clrScheme")]
pub struct ColorScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Dark 1
  #[sdk(child(qname = "a:CT_Color2/a:dk1"))]
  pub dark1_color: std::boxed::Box<Dark1Color>,
  /// Light 1
  #[sdk(child(qname = "a:CT_Color2/a:lt1"))]
  pub light1_color: std::boxed::Box<Light1Color>,
  /// Dark 2
  #[sdk(child(qname = "a:CT_Color2/a:dk2"))]
  pub dark2_color: std::boxed::Box<Dark2Color>,
  /// Light 2
  #[sdk(child(qname = "a:CT_Color2/a:lt2"))]
  pub light2_color: std::boxed::Box<Light2Color>,
  /// Accent 1
  #[sdk(child(qname = "a:CT_Color2/a:accent1"))]
  pub accent1_color: std::boxed::Box<Accent1Color>,
  /// Accent 2
  #[sdk(child(qname = "a:CT_Color2/a:accent2"))]
  pub accent2_color: std::boxed::Box<Accent2Color>,
  /// Accent 3
  #[sdk(child(qname = "a:CT_Color2/a:accent3"))]
  pub accent3_color: std::boxed::Box<Accent3Color>,
  /// Accent 4
  #[sdk(child(qname = "a:CT_Color2/a:accent4"))]
  pub accent4_color: std::boxed::Box<Accent4Color>,
  /// Accent 5
  #[sdk(child(qname = "a:CT_Color2/a:accent5"))]
  pub accent5_color: std::boxed::Box<Accent5Color>,
  /// Accent 6
  #[sdk(child(qname = "a:CT_Color2/a:accent6"))]
  pub accent6_color: std::boxed::Box<Accent6Color>,
  /// Hyperlink
  #[sdk(child(qname = "a:CT_Color2/a:hlink"))]
  pub hyperlink: std::boxed::Box<Hyperlink>,
  /// Followed Hyperlink
  #[sdk(child(qname = "a:CT_Color2/a:folHlink"))]
  pub followed_hyperlink_color: std::boxed::Box<FollowedHyperlinkColor>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Font Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_FontScheme/a:fontScheme")]
pub struct FontScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Major Font
  #[sdk(child(qname = "a:CT_FontCollection/a:majorFont"))]
  pub major_font: std::boxed::Box<MajorFont>,
  /// Minor fonts
  #[sdk(child(qname = "a:CT_FontCollection/a:minorFont"))]
  pub minor_font: std::boxed::Box<MinorFont>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Format Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_StyleMatrix/a:fmtScheme")]
pub struct FormatScheme {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Fill Style List
  #[sdk(child(qname = "a:CT_FillStyleList/a:fillStyleLst"))]
  pub fill_style_list: std::boxed::Box<FillStyleList>,
  /// Line Style List
  #[sdk(child(qname = "a:CT_LineStyleList/a:lnStyleLst"))]
  pub line_style_list: std::boxed::Box<LineStyleList>,
  /// Effect Style List
  #[sdk(child(qname = "a:CT_EffectStyleList/a:effectStyleLst"))]
  pub effect_style_list: std::boxed::Box<EffectStyleList>,
  /// Background Fill Style List
  #[sdk(child(qname = "a:CT_BackgroundFillStyleList/a:bgFillStyleLst"))]
  pub background_fill_style_list: std::boxed::Box<BackgroundFillStyleList>,
}
/// Dark 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:dk1")]
pub struct Dark1Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Dark1ColorChoice>,
}
/// Light 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:lt1")]
pub struct Light1Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Light1ColorChoice>,
}
/// Dark 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:dk2")]
pub struct Dark2Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Dark2ColorChoice>,
}
/// Light 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:lt2")]
pub struct Light2Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Light2ColorChoice>,
}
/// Accent 1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent1")]
pub struct Accent1Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent1ColorChoice>,
}
/// Accent 2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent2")]
pub struct Accent2Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent2ColorChoice>,
}
/// Accent 3.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent3")]
pub struct Accent3Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent3ColorChoice>,
}
/// Accent 4.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent4")]
pub struct Accent4Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent4ColorChoice>,
}
/// Accent 5.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent5")]
pub struct Accent5Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent5ColorChoice>,
}
/// Accent 6.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:accent6")]
pub struct Accent6Color {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<Accent6ColorChoice>,
}
/// Hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:hlink")]
pub struct Hyperlink {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<HyperlinkChoice>,
}
/// Followed Hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color2/a:folHlink")]
pub struct FollowedHyperlinkColor {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<FollowedHyperlinkColorChoice>,
}
/// Horizontal Ratio.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Ratio/a:sx")]
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
#[sdk(qname = "a:CT_Ratio/a:sy")]
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
#[sdk(qname = "a:CT_Point2D/a:off")]
pub struct Offset {
  /// X-Axis Coordinate
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
/// Child Offset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point2D/a:chOff")]
pub struct ChildOffset {
  /// X-Axis Coordinate
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
/// Extents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/a:ext")]
pub struct Extents {
  /// Extent Length
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
/// Child Extents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PositiveSize2D/a:chExt")]
pub struct ChildExtents {
  /// Extent Length
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
/// Shape Locks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeLocking/a:spLocks")]
pub struct ShapeLocks {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Connection Shape Locks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ConnectorLocking/a:cxnSpLocks")]
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
  /// _
  #[sdk(child(qname = "a:CT_ConnectorLockingExtensionList/a:extLst"))]
  pub connector_locking_extension_list: Option<ConnectorLockingExtensionList>,
}
/// Connection Start.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Connection/a:stCxn")]
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
#[sdk(qname = "a:CT_Connection/a:endCxn")]
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
#[sdk(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks")]
pub struct GraphicFrameLocks {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Graphic Object Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GraphicalObjectData/a:graphicData")]
pub struct GraphicData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Uniform Resource Identifier
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Diagram to Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationDgmElement/a:dgm")]
pub struct Diagram {
  /// Identifier
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Animation Build Step
  #[sdk(attr(qname = ":bldStep"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub build_step: Option<DiagramBuildStepValues>,
}
/// Chart to Animate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationChartElement/a:chart")]
pub struct Chart {
  /// Series Index
  #[sdk(attr(qname = ":seriesIdx"))]
  pub series_index: Option<crate::simple_type::Int32Value>,
  /// Category Index
  #[sdk(attr(qname = ":categoryIdx"))]
  pub category_index: Option<crate::simple_type::Int32Value>,
  /// Animation Build Step
  #[sdk(attr(qname = ":bldStep"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub build_step: ChartBuildStepValues,
}
/// Build Diagram.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AnimationDgmBuildProperties/a:bldDgm")]
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
#[sdk(qname = "a:CT_AnimationChartBuildProperties/a:bldChart")]
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
#[sdk(qname = "a:CT_TextBody/a:txBody")]
pub struct TextBody {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<ListStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<Paragraph>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Transform2D/a:xfrm")]
pub struct Transform2D {
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
  pub offset: Option<Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<Extents>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingProps/a:cNvPr")]
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
  pub hyperlink_on_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// Hyperlink associated with hovering over the element.
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
  pub hyperlink_on_hover: Option<std::boxed::Box<HyperlinkOnHover>>,
  /// Future extension
  #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
  pub non_visual_drawing_properties_extension_list: Option<NonVisualDrawingPropertiesExtensionList>,
}
/// Non-Visual Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingShapeProps/a:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Text Box
  #[sdk(attr(qname = ":txBox"))]
  pub text_box: Option<crate::simple_type::BooleanValue>,
  /// Shape Locks
  #[sdk(child(qname = "a:CT_ShapeLocking/a:spLocks"))]
  pub shape_locks: Option<std::boxed::Box<ShapeLocks>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlShapeNonVisual/a:nvSpPr")]
pub struct NonVisualShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingShapeProps/a:cNvSpPr"))]
  pub non_visual_shape_drawing_properties: std::boxed::Box<NonVisualShapeDrawingProperties>,
}
/// Visual Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/a:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode: Option<BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
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
  pub a_ln: Option<std::boxed::Box<Outline>>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<Scene3DType>>,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<std::boxed::Box<Shape3DType>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ShapePropertiesExtensionList>,
}
/// Text Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlTextShape/a:txSp")]
pub struct TextShape {
  /// Shape Text Body
  #[sdk(child(qname = "a:CT_TextBody/a:txBody"))]
  pub text_body: std::boxed::Box<TextBody>,
  #[sdk(choice(
    qname = "a:CT_GvmlUseShapeRectangle/a:useSpRect",
    qname = "a:CT_Transform2D/a:xfrm"
  ))]
  pub text_shape_choice: Option<TextShapeChoice>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/a:style")]
pub struct ShapeStyle {
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
}
/// Non-Visual Connector Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualConnectorProperties/a:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
  /// Connection Shape Locks
  #[sdk(child(qname = "a:CT_ConnectorLocking/a:cxnSpLocks"))]
  pub connection_shape_locks: Option<std::boxed::Box<ConnectionShapeLocks>>,
  /// Connection Start
  #[sdk(child(qname = "a:CT_Connection/a:stCxn"))]
  pub start_connection: Option<StartConnection>,
  /// Connection End
  #[sdk(child(qname = "a:CT_Connection/a:endCxn"))]
  pub end_connection: Option<EndConnection>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlConnectorNonVisual/a:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
  /// Non-Visual Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Connector Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualConnectorProperties/a:cNvCxnSpPr"))]
  pub non_visual_connector_shape_drawing_properties:
    std::boxed::Box<NonVisualConnectorShapeDrawingProperties>,
}
/// Non-Visual Picture Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPictureProperties/a:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
  /// preferRelativeResize
  #[sdk(attr(qname = ":preferRelativeResize"))]
  pub prefer_relative_resize: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_PictureLocking/a:picLocks"))]
  pub picture_locks: Option<std::boxed::Box<PictureLocks>>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst"))]
  pub non_visual_picture_properties_extension_list: Option<NonVisualPicturePropertiesExtensionList>,
}
/// Non-Visual Properties for a Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlPictureNonVisual/a:nvPicPr")]
pub struct NonVisualPictureProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Picture Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualPictureProperties/a:cNvPicPr"))]
  pub non_visual_picture_drawing_properties: std::boxed::Box<NonVisualPictureDrawingProperties>,
}
/// Non-Visual Graphic Frame Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGraphicFrameProperties/a:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
  /// Graphic Frame Locks
  #[sdk(child(qname = "a:CT_GraphicalObjectFrameLocking/a:graphicFrameLocks"))]
  pub graphic_frame_locks: Option<std::boxed::Box<GraphicFrameLocks>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGraphicFrameNonVisual/a:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Graphic Frame Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGraphicFrameProperties/a:cNvGraphicFramePr"))]
  pub non_visual_graphic_frame_drawing_properties:
    std::boxed::Box<NonVisualGraphicFrameDrawingProperties>,
}
/// Non-Visual Group Shape Drawing Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapeProps/a:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
  /// _
  #[sdk(child(qname = "a:CT_GroupLocking/a:grpSpLocks"))]
  pub group_shape_locks: Option<std::boxed::Box<GroupShapeLocks>>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst"))]
  pub non_visual_group_drawing_shape_props_extension_list:
    Option<NonVisualGroupDrawingShapePropsExtensionList>,
}
/// Rotation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SphereCoords/a:rot")]
pub struct Rotation {
  /// Latitude
  #[sdk(attr(qname = ":lat"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub latitude: crate::simple_type::Int32Value,
  /// Longitude
  #[sdk(attr(qname = ":lon"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub longitude: crate::simple_type::Int32Value,
  /// Revolution
  #[sdk(attr(qname = ":rev"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub revolution: crate::simple_type::Int32Value,
}
/// Camera.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Camera/a:camera")]
pub struct Camera {
  /// Preset Camera Type
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub preset: PresetCameraValues,
  /// Field of View
  #[sdk(attr(qname = ":fov"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "10800000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub field_of_view: Option<crate::simple_type::Int32Value>,
  /// Zoom
  #[sdk(attr(qname = ":zoom"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub zoom: Option<crate::simple_type::Int32Value>,
  /// Rotation
  #[sdk(child(qname = "a:CT_SphereCoords/a:rot"))]
  pub rotation: Option<Rotation>,
}
/// Light Rig.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LightRig/a:lightRig")]
pub struct LightRig {
  /// Rig Preset
  #[sdk(attr(qname = ":rig"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub rig: LightRigValues,
  /// Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub direction: LightRigDirectionValues,
  /// Rotation
  #[sdk(child(qname = "a:CT_SphereCoords/a:rot"))]
  pub rotation: Option<Rotation>,
}
/// Backdrop Plane.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Backdrop/a:backdrop")]
pub struct Backdrop {
  /// Anchor Point
  #[sdk(child(qname = "a:CT_Point3D/a:anchor"))]
  pub anchor: std::boxed::Box<Anchor>,
  /// Normal
  #[sdk(child(qname = "a:CT_Vector3D/a:norm"))]
  pub normal: std::boxed::Box<Normal>,
  /// Up Vector
  #[sdk(child(qname = "a:CT_Vector3D/a:up"))]
  pub up_vector: std::boxed::Box<UpVector>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Anchor Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Point3D/a:anchor")]
pub struct Anchor {
  /// X-Coordinate in 3D
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: crate::simple_type::Int64Value,
  /// Y-Coordinate in 3D
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: crate::simple_type::Int64Value,
  /// Z-Coordinate in 3D
  #[sdk(attr(qname = ":z"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub z: crate::simple_type::Int64Value,
}
/// Normal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/a:norm")]
pub struct Normal {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dz: crate::simple_type::Int64Value,
}
/// Up Vector.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Vector3D/a:up")]
pub struct UpVector {
  /// Distance along X-axis in 3D
  #[sdk(attr(qname = ":dx"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dx: crate::simple_type::Int64Value,
  /// Distance along Y-axis in 3D
  #[sdk(attr(qname = ":dy"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dy: crate::simple_type::Int64Value,
  /// Distance along Z-axis in 3D
  #[sdk(attr(qname = ":dz"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub dz: crate::simple_type::Int64Value,
}
/// Top Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Bevel/a:bevelT")]
pub struct BevelTop {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Bottom Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Bevel/a:bevelB")]
pub struct BevelBottom {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Bevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Bevel/a:bevel")]
pub struct Bevel {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// Preset Bevel
  #[sdk(attr(qname = ":prst"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset: Option<BevelPresetValues>,
}
/// Fill To Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RelativeRect/a:fillToRect")]
pub struct FillToRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Tile Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RelativeRect/a:tileRect")]
pub struct TileRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Fill Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RelativeRect/a:fillRect")]
pub struct FillRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Source Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_RelativeRect/a:srcRect")]
pub struct SourceRectangle {
  /// Left Offset
  #[sdk(attr(qname = ":l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// Top Offset
  #[sdk(attr(qname = ":t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// Right Offset
  #[sdk(attr(qname = ":r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// Bottom Offset
  #[sdk(attr(qname = ":b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Gradient stops.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GradientStop/a:gs")]
pub struct GradientStop {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Position
  #[sdk(attr(qname = ":pos"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub position: crate::simple_type::Int32Value,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<GradientStopChoice>,
}
/// Gradient Stop List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GradientStopList/a:gsLst")]
pub struct GradientStopList {
  /// _
  #[sdk(child(qname = "a:CT_GradientStop/a:gs"))]
  pub a_gs: Vec<GradientStop>,
}
/// Shape Guide.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GeomGuide/a:gd")]
pub struct ShapeGuide {
  /// Shape Guide Name
  #[sdk(attr(qname = ":name"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub name: crate::simple_type::StringValue,
  /// Shape Guide Formula
  #[sdk(attr(qname = ":fmla"))]
  pub formula: crate::simple_type::StringValue,
}
/// Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AdjPoint2D/a:pos")]
pub struct Position {
  /// X-Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub y: crate::simple_type::StringValue,
}
/// Move end point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AdjPoint2D/a:pt")]
pub struct Point {
  /// X-Coordinate
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub y: crate::simple_type::StringValue,
}
/// XY Adjust Handle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_XYAdjustHandle/a:ahXY")]
pub struct AdjustHandleXy {
  /// Horizontal Adjustment Guide
  #[sdk(attr(qname = ":gdRefX"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub x_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Horizontal Adjustment
  #[sdk(attr(qname = ":minX"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_x: Option<crate::simple_type::StringValue>,
  /// Vertical Adjustment Guide
  #[sdk(attr(qname = ":gdRefY"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub y_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Vertical Adjustment
  #[sdk(attr(qname = ":minY"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_y: Option<crate::simple_type::StringValue>,
  /// Position
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Polar Adjust Handle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PolarAdjustHandle/a:ahPolar")]
pub struct AdjustHandlePolar {
  /// Radial Adjustment Guide
  #[sdk(attr(qname = ":gdRefR"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub radial_adjustment_guide: Option<crate::simple_type::StringValue>,
  /// Minimum Radial Adjustment
  #[sdk(attr(qname = ":minR"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max_radial: Option<crate::simple_type::StringValue>,
  /// Angle Adjustment Guide
  #[sdk(attr(qname = ":gdRefAng"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
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
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Shape Connection Site.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ConnectionSite/a:cxn")]
pub struct ConnectionSite {
  /// Connection Site Angle
  #[sdk(attr(qname = ":ang"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "a:ST_Angle"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub angle: crate::simple_type::StringValue,
  /// Position
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pos"))]
  pub position: std::boxed::Box<Position>,
}
/// Move Path To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2DMoveTo/a:moveTo")]
pub struct MoveTo {
  /// Move end point
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pt"))]
  pub point: std::boxed::Box<Point>,
}
/// Draw Line To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2DLineTo/a:lnTo")]
pub struct LineTo {
  /// Line end point
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pt"))]
  pub point: std::boxed::Box<Point>,
}
/// Draw Arc To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2DArcTo/a:arcTo")]
pub struct ArcTo {
  /// Shape Arc Width Radius
  #[sdk(attr(qname = ":wR"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
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
#[sdk(qname = "a:CT_Path2DQuadBezierTo/a:quadBezTo")]
pub struct QuadraticBezierCurveTo {
  /// _
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pt"))]
  pub a_pt: Vec<Point>,
}
/// Draw Cubic Bezier Curve To.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2DCubicBezierTo/a:cubicBezTo")]
pub struct CubicBezierCurveTo {
  /// _
  #[sdk(child(qname = "a:CT_AdjPoint2D/a:pt"))]
  pub a_pt: Vec<Point>,
}
/// Shape Path.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2D/a:path")]
pub struct Path {
  /// Path Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// Path Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// Path Fill
  #[sdk(attr(qname = ":fill"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub fill: Option<PathFillModeValues>,
  /// Path Stroke
  #[sdk(attr(qname = ":stroke"))]
  pub stroke: Option<crate::simple_type::BooleanValue>,
  /// 3D Extrusion Allowed
  #[sdk(attr(qname = ":extrusionOk"))]
  pub extrusion_ok: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "a:CT_Path2DClose/a:close",
    qname = "a:CT_Path2DMoveTo/a:moveTo",
    qname = "a:CT_Path2DLineTo/a:lnTo",
    qname = "a:CT_Path2DArcTo/a:arcTo",
    qname = "a:CT_Path2DQuadBezierTo/a:quadBezTo",
    qname = "a:CT_Path2DCubicBezierTo/a:cubicBezTo"
  ))]
  pub xml_children: Vec<PathChoice>,
}
/// List of Shape Adjust Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GeomGuideList/a:avLst")]
pub struct AdjustValueList {
  /// _
  #[sdk(child(qname = "a:CT_GeomGuide/a:gd"))]
  pub a_gd: Vec<ShapeGuide>,
}
/// List of Shape Guides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GeomGuideList/a:gdLst")]
pub struct ShapeGuideList {
  /// _
  #[sdk(child(qname = "a:CT_GeomGuide/a:gd"))]
  pub a_gd: Vec<ShapeGuide>,
}
/// List of Shape Adjust Handles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_AdjustHandleList/a:ahLst")]
pub struct AdjustHandleList {
  #[sdk(choice(
    qname = "a:CT_XYAdjustHandle/a:ahXY",
    qname = "a:CT_PolarAdjustHandle/a:ahPolar"
  ))]
  pub xml_children: Vec<AdjustHandleListChoice>,
}
/// List of Shape Connection Sites.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ConnectionSiteList/a:cxnLst")]
pub struct ConnectionSiteList {
  /// _
  #[sdk(child(qname = "a:CT_ConnectionSite/a:cxn"))]
  pub a_cxn: Vec<ConnectionSite>,
}
/// Shape Text Rectangle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GeomRect/a:rect")]
pub struct Rectangle {
  /// Left
  #[sdk(attr(qname = ":l"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
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
    max_inclusive = true
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
    max_inclusive = true
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
    max_inclusive = true
  ))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub bottom: crate::simple_type::StringValue,
}
/// List of Shape Paths.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Path2DList/a:pathLst")]
pub struct PathList {
  /// _
  #[sdk(child(qname = "a:CT_Path2D/a:path"))]
  pub a_path: Vec<Path>,
}
/// Dash Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DashStop/a:ds")]
pub struct DashStop {
  /// Dash Length
  #[sdk(attr(qname = ":d"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub dash_length: crate::simple_type::Int32Value,
  /// Space Length
  #[sdk(attr(qname = ":sp"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub space_length: crate::simple_type::Int32Value,
}
/// 2D Transform for Grouped Objects.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupTransform2D/a:xfrm")]
pub struct TransformGroup {
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
  pub offset: Option<Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<Extents>,
  /// Child Offset
  #[sdk(child(qname = "a:CT_Point2D/a:chOff"))]
  pub child_offset: Option<ChildOffset>,
  /// Child Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:chExt"))]
  pub child_extents: Option<ChildExtents>,
}
/// Defines the BodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBodyProperties/a:bodyPr")]
pub struct BodyProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<TextVerticalOverflowValues>,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical: Option<TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap: Option<TextWrappingValues>,
  /// Left Inset
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "16",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
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
  #[sdk(child(qname = "a:CT_PresetTextShape/a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<PresetTextWarp>>,
  #[sdk(choice(
    qname = "a:CT_TextNoAutofit/a:noAutofit",
    qname = "a:CT_TextNormalAutofit/a:normAutofit",
    qname = "a:CT_TextShapeAutofit/a:spAutoFit"
  ))]
  pub body_properties_choice1: Option<BodyPropertiesChoice>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<Scene3DType>>,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub body_properties_choice2: Option<BodyPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Defines the ListStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextListStyle/a:lstStyle")]
pub struct ListStyle {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Default Paragraph Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:defPPr"))]
  pub default_paragraph_properties: Option<std::boxed::Box<DefaultParagraphProperties>>,
  /// List Level 1 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl1pPr"))]
  pub level1_paragraph_properties: Option<std::boxed::Box<Level1ParagraphProperties>>,
  /// List Level 2 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl2pPr"))]
  pub level2_paragraph_properties: Option<std::boxed::Box<Level2ParagraphProperties>>,
  /// List Level 3 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl3pPr"))]
  pub level3_paragraph_properties: Option<std::boxed::Box<Level3ParagraphProperties>>,
  /// List Level 4 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl4pPr"))]
  pub level4_paragraph_properties: Option<std::boxed::Box<Level4ParagraphProperties>>,
  /// List Level 5 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl5pPr"))]
  pub level5_paragraph_properties: Option<std::boxed::Box<Level5ParagraphProperties>>,
  /// List Level 6 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl6pPr"))]
  pub level6_paragraph_properties: Option<std::boxed::Box<Level6ParagraphProperties>>,
  /// List Level 7 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl7pPr"))]
  pub level7_paragraph_properties: Option<std::boxed::Box<Level7ParagraphProperties>>,
  /// List Level 8 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl8pPr"))]
  pub level8_paragraph_properties: Option<std::boxed::Box<Level8ParagraphProperties>>,
  /// List Level 9 Text Style
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:lvl9pPr"))]
  pub level9_paragraph_properties: Option<std::boxed::Box<Level9ParagraphProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Shape Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DefaultShapeDefinition/a:spDef")]
pub struct ShapeDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Line Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DefaultShapeDefinition/a:lnDef")]
pub struct LineDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Text Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DefaultShapeDefinition/a:txDef")]
pub struct TextDefault {
  /// Visual Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties: std::boxed::Box<BodyProperties>,
  /// _
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: std::boxed::Box<ListStyle>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Override Color Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/a:overrideClrMapping")]
pub struct OverrideColorMapping {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background1: ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text1: ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background2: ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text2: ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent1: ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent2: ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent3: ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent4: ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent5: ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent6: ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub hyperlink: ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub followed_hyperlink: ColorSchemeIndexValues,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/a:clrMap")]
pub struct ColorMap {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background1: ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text1: ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background2: ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text2: ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent1: ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent2: ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent3: ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent4: ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent5: ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent6: ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub hyperlink: ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub followed_hyperlink: ColorSchemeIndexValues,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Extra Color Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorSchemeAndMapping/a:extraClrScheme")]
pub struct ExtraColorScheme {
  /// _
  #[sdk(child(qname = "a:CT_ColorScheme/a:clrScheme"))]
  pub color_scheme: std::boxed::Box<ColorScheme>,
  /// _
  #[sdk(child(qname = "a:CT_ColorMapping/a:clrMap"))]
  pub color_map: Option<std::boxed::Box<ColorMap>>,
}
/// Defines the ThemeElements Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BaseStyles/a:themeElements")]
pub struct ThemeElements {
  /// _
  #[sdk(child(qname = "a:CT_ColorScheme/a:clrScheme"))]
  pub color_scheme: std::boxed::Box<ColorScheme>,
  /// Font Scheme
  #[sdk(child(qname = "a:CT_FontScheme/a:fontScheme"))]
  pub font_scheme: std::boxed::Box<FontScheme>,
  /// Format Scheme
  #[sdk(child(qname = "a:CT_StyleMatrix/a:fmtScheme"))]
  pub format_scheme: std::boxed::Box<FormatScheme>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell 3-D.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Cell3D/a:cell3D")]
pub struct Cell3DProperties {
  /// Preset Material
  #[sdk(attr(qname = ":prstMaterial"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub preset_material: Option<PresetMaterialTypeValues>,
  /// Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevel"))]
  pub bevel: std::boxed::Box<Bevel>,
  /// Light Rig
  #[sdk(child(qname = "a:CT_LightRig/a:lightRig"))]
  pub light_rig: Option<std::boxed::Box<LightRig>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableCellProperties/a:tcPr")]
pub struct TableCellProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Top Margin
  #[sdk(attr(qname = ":marT"))]
  pub top_margin: Option<crate::simple_type::Int32Value>,
  /// Bottom Margin
  #[sdk(attr(qname = ":marB"))]
  pub bottom_margin: Option<crate::simple_type::Int32Value>,
  /// Text Direction
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical: Option<TextVerticalValues>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor: Option<TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
  /// Left Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnL"))]
  pub left_border_line_properties: Option<std::boxed::Box<LeftBorderLineProperties>>,
  /// Right Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnR"))]
  pub right_border_line_properties: Option<std::boxed::Box<RightBorderLineProperties>>,
  /// Top Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnT"))]
  pub top_border_line_properties: Option<std::boxed::Box<TopBorderLineProperties>>,
  /// Bottom Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnB"))]
  pub bottom_border_line_properties: Option<std::boxed::Box<BottomBorderLineProperties>>,
  /// Top-Left to Bottom-Right Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnTlToBr"))]
  pub top_left_to_bottom_right_border_line_properties:
    Option<std::boxed::Box<TopLeftToBottomRightBorderLineProperties>>,
  /// Bottom-Left to Top-Right Border Line Properties
  #[sdk(child(qname = "a:CT_LineProperties/a:lnBlToTr"))]
  pub bottom_left_to_top_right_border_line_properties:
    Option<std::boxed::Box<BottomLeftToTopRightBorderLineProperties>>,
  /// Cell 3-D
  #[sdk(child(qname = "a:CT_Cell3D/a:cell3D"))]
  pub cell3_d_properties: Option<std::boxed::Box<Cell3DProperties>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub table_cell_properties_choice: Option<TableCellPropertiesChoice>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Table Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableCell/a:tc")]
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
  #[sdk(child(qname = "a:CT_TextBody/a:txBody"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// Table Cell Properties
  #[sdk(child(qname = "a:CT_TableCellProperties/a:tcPr"))]
  pub table_cell_properties: Option<std::boxed::Box<TableCellProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableStyle/a:tableStyle")]
pub struct TableStyle {
  /// Style ID
  #[sdk(attr(qname = ":styleId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub style_id: crate::simple_type::StringValue,
  /// Name
  #[sdk(attr(qname = ":styleName"))]
  pub style_name: crate::simple_type::StringValue,
  /// Table Background
  #[sdk(child(qname = "a:CT_TableBackgroundStyle/a:tblBg"))]
  pub table_background: Option<std::boxed::Box<TableBackground>>,
  /// Whole Table
  #[sdk(child(qname = "a:CT_TablePartStyle/a:wholeTbl"))]
  pub whole_table: Option<std::boxed::Box<WholeTable>>,
  /// Band 1 Horizontal
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band1H"))]
  pub band1_horizontal: Option<std::boxed::Box<Band1Horizontal>>,
  /// Band 2 Horizontal
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band2H"))]
  pub band2_horizontal: Option<std::boxed::Box<Band2Horizontal>>,
  /// Band 1 Vertical
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band1V"))]
  pub band1_vertical: Option<std::boxed::Box<Band1Vertical>>,
  /// Band 2 Vertical
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band2V"))]
  pub band2_vertical: Option<std::boxed::Box<Band2Vertical>>,
  /// Last Column
  #[sdk(child(qname = "a:CT_TablePartStyle/a:lastCol"))]
  pub last_column: Option<std::boxed::Box<LastColumn>>,
  /// First Column
  #[sdk(child(qname = "a:CT_TablePartStyle/a:firstCol"))]
  pub first_column: Option<std::boxed::Box<FirstColumn>>,
  /// Last Row
  #[sdk(child(qname = "a:CT_TablePartStyle/a:lastRow"))]
  pub last_row: Option<std::boxed::Box<LastRow>>,
  /// Southeast Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:seCell"))]
  pub southeast_cell: Option<std::boxed::Box<SoutheastCell>>,
  /// Southwest Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:swCell"))]
  pub southwest_cell: Option<std::boxed::Box<SouthwestCell>>,
  /// First Row
  #[sdk(child(qname = "a:CT_TablePartStyle/a:firstRow"))]
  pub first_row: Option<std::boxed::Box<FirstRow>>,
  /// Northeast Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:neCell"))]
  pub northeast_cell: Option<std::boxed::Box<NortheastCell>>,
  /// Northwest Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:nwCell"))]
  pub northwest_cell: Option<std::boxed::Box<NorthwestCell>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableStyle/a:tblStyle")]
pub struct TableStyleEntry {
  /// Style ID
  #[sdk(attr(qname = ":styleId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub style_id: crate::simple_type::StringValue,
  /// Name
  #[sdk(attr(qname = ":styleName"))]
  pub style_name: crate::simple_type::StringValue,
  /// Table Background
  #[sdk(child(qname = "a:CT_TableBackgroundStyle/a:tblBg"))]
  pub table_background: Option<std::boxed::Box<TableBackground>>,
  /// Whole Table
  #[sdk(child(qname = "a:CT_TablePartStyle/a:wholeTbl"))]
  pub whole_table: Option<std::boxed::Box<WholeTable>>,
  /// Band 1 Horizontal
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band1H"))]
  pub band1_horizontal: Option<std::boxed::Box<Band1Horizontal>>,
  /// Band 2 Horizontal
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band2H"))]
  pub band2_horizontal: Option<std::boxed::Box<Band2Horizontal>>,
  /// Band 1 Vertical
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band1V"))]
  pub band1_vertical: Option<std::boxed::Box<Band1Vertical>>,
  /// Band 2 Vertical
  #[sdk(child(qname = "a:CT_TablePartStyle/a:band2V"))]
  pub band2_vertical: Option<std::boxed::Box<Band2Vertical>>,
  /// Last Column
  #[sdk(child(qname = "a:CT_TablePartStyle/a:lastCol"))]
  pub last_column: Option<std::boxed::Box<LastColumn>>,
  /// First Column
  #[sdk(child(qname = "a:CT_TablePartStyle/a:firstCol"))]
  pub first_column: Option<std::boxed::Box<FirstColumn>>,
  /// Last Row
  #[sdk(child(qname = "a:CT_TablePartStyle/a:lastRow"))]
  pub last_row: Option<std::boxed::Box<LastRow>>,
  /// Southeast Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:seCell"))]
  pub southeast_cell: Option<std::boxed::Box<SoutheastCell>>,
  /// Southwest Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:swCell"))]
  pub southwest_cell: Option<std::boxed::Box<SouthwestCell>>,
  /// First Row
  #[sdk(child(qname = "a:CT_TablePartStyle/a:firstRow"))]
  pub first_row: Option<std::boxed::Box<FirstRow>>,
  /// Northeast Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:neCell"))]
  pub northeast_cell: Option<std::boxed::Box<NortheastCell>>,
  /// Northwest Cell
  #[sdk(child(qname = "a:CT_TablePartStyle/a:nwCell"))]
  pub northwest_cell: Option<std::boxed::Box<NorthwestCell>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Style ID.
pub type TableStyleId = crate::simple_type::StringValue;
/// Table Grid Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableCol/a:gridCol")]
pub struct GridColumn {
  /// Width
  #[sdk(attr(qname = ":w"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub width: crate::simple_type::Int64Value,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableProperties/a:tblPr")]
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
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub table_properties_choice1: Option<TablePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub table_properties_choice2: Option<TablePropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TableStyle/a:tableStyle",
    qname = "a:ST_Guid/a:tableStyleId"
  ))]
  pub table_properties_choice3: Option<TablePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Table Grid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableGrid/a:tblGrid")]
pub struct TableGrid {
  /// _
  #[sdk(child(qname = "a:CT_TableCol/a:gridCol"))]
  pub a_grid_col: Vec<GridColumn>,
}
/// Table Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableRow/a:tr")]
pub struct TableRow {
  /// Height
  #[sdk(attr(qname = ":h"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height: crate::simple_type::Int64Value,
  /// _
  #[sdk(child(qname = "a:CT_TableCell/a:tc"))]
  pub a_tc: Vec<TableCell>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Left Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:left")]
pub struct LeftBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<LeftBorderChoice>,
}
/// Right Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:right")]
pub struct RightBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<RightBorderChoice>,
}
/// Top Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:top")]
pub struct TopBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<TopBorderChoice>,
}
/// Bottom Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:bottom")]
pub struct BottomBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<BottomBorderChoice>,
}
/// Inside Horizontal Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:insideH")]
pub struct InsideHorizontalBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<InsideHorizontalBorderChoice>,
}
/// Inside Vertical Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:insideV")]
pub struct InsideVerticalBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<InsideVerticalBorderChoice>,
}
/// Top Left to Bottom Right Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:tl2br")]
pub struct TopLeftToBottomRightBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<TopLeftToBottomRightBorderChoice>,
}
/// Top Right to Bottom Left Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ThemeableLineStyle/a:tr2bl")]
pub struct TopRightToBottomLeftBorder {
  #[sdk(choice(
    qname = "a:CT_LineProperties/a:ln",
    qname = "a:CT_StyleMatrixReference/a:lnRef"
  ))]
  pub xml_children: Option<TopRightToBottomLeftBorderChoice>,
}
/// Table Cell Borders.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableCellBorderStyle/a:tcBdr")]
pub struct TableCellBorders {
  /// Left Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:left"))]
  pub left_border: Option<std::boxed::Box<LeftBorder>>,
  /// Right Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:right"))]
  pub right_border: Option<std::boxed::Box<RightBorder>>,
  /// Top Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:top"))]
  pub top_border: Option<std::boxed::Box<TopBorder>>,
  /// Bottom Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:bottom"))]
  pub bottom_border: Option<std::boxed::Box<BottomBorder>>,
  /// Inside Horizontal Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:insideH"))]
  pub inside_horizontal_border: Option<std::boxed::Box<InsideHorizontalBorder>>,
  /// Inside Vertical Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:insideV"))]
  pub inside_vertical_border: Option<std::boxed::Box<InsideVerticalBorder>>,
  /// Top Left to Bottom Right Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:tl2br"))]
  pub top_left_to_bottom_right_border: Option<std::boxed::Box<TopLeftToBottomRightBorder>>,
  /// Top Right to Bottom Left Border
  #[sdk(child(qname = "a:CT_ThemeableLineStyle/a:tr2bl"))]
  pub top_right_to_bottom_left_border: Option<std::boxed::Box<TopRightToBottomLeftBorder>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Table Cell Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle")]
pub struct TableCellTextStyle {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Bold
  #[sdk(attr(qname = ":b"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub bold: Option<BooleanStyleValues>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub italic: Option<BooleanStyleValues>,
  #[sdk(choice(
    qname = "a:CT_FontCollection/a:font",
    qname = "a:CT_FontReference/a:fontRef"
  ))]
  pub table_cell_text_style_choice1: Option<TableCellTextStyleChoice>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub table_cell_text_style_choice2: Option<TableCellTextStyleChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Table Cell Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableStyleCellStyle/a:tcStyle")]
pub struct TableCellStyle {
  /// Table Cell Borders
  #[sdk(child(qname = "a:CT_TableCellBorderStyle/a:tcBdr"))]
  pub table_cell_borders: Option<std::boxed::Box<TableCellBorders>>,
  #[sdk(choice(
    qname = "a:CT_FillProperties/a:fill",
    qname = "a:CT_StyleMatrixReference/a:fillRef"
  ))]
  pub table_cell_style_choice: Option<TableCellStyleChoice>,
  /// _
  #[sdk(child(qname = "a:CT_Cell3D/a:cell3D"))]
  pub a_cell3_d: Option<std::boxed::Box<Cell3DProperties>>,
}
/// Table Background.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TableBackgroundStyle/a:tblBg")]
pub struct TableBackground {
  #[sdk(choice(
    qname = "a:CT_FillProperties/a:fill",
    qname = "a:CT_StyleMatrixReference/a:fillRef"
  ))]
  pub table_background_choice1: Option<TableBackgroundChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectProperties/a:effect",
    qname = "a:CT_StyleMatrixReference/a:effectRef"
  ))]
  pub table_background_choice2: Option<TableBackgroundChoice2>,
}
/// Whole Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:wholeTbl")]
pub struct WholeTable {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 1 Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:band1H")]
pub struct Band1Horizontal {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 2 Horizontal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:band2H")]
pub struct Band2Horizontal {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 1 Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:band1V")]
pub struct Band1Vertical {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Band 2 Vertical.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:band2V")]
pub struct Band2Vertical {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Last Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:lastCol")]
pub struct LastColumn {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// First Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:firstCol")]
pub struct FirstColumn {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Last Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:lastRow")]
pub struct LastRow {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Southeast Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:seCell")]
pub struct SoutheastCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Southwest Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:swCell")]
pub struct SouthwestCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// First Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:firstRow")]
pub struct FirstRow {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Northeast Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:neCell")]
pub struct NortheastCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Northwest Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TablePartStyle/a:nwCell")]
pub struct NorthwestCell {
  /// Table Cell Text Style
  #[sdk(child(qname = "a:CT_TableStyleTextStyle/a:tcTxStyle"))]
  pub table_cell_text_style: Option<std::boxed::Box<TableCellTextStyle>>,
  /// Table Cell Style
  #[sdk(child(qname = "a:CT_TableStyleCellStyle/a:tcStyle"))]
  pub table_cell_style: Option<std::boxed::Box<TableCellStyle>>,
}
/// Text Paragraph Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:pPr")]
pub struct ParagraphProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub paragraph_properties_choice1: Option<ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub paragraph_properties_choice2: Option<ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub paragraph_properties_choice3: Option<ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub paragraph_properties_choice4: Option<ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Default Paragraph Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:defPPr")]
pub struct DefaultParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub default_paragraph_properties_choice1: Option<DefaultParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub default_paragraph_properties_choice2: Option<DefaultParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub default_paragraph_properties_choice3: Option<DefaultParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub default_paragraph_properties_choice4: Option<DefaultParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 1 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl1pPr")]
pub struct Level1ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level1_paragraph_properties_choice1: Option<Level1ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level1_paragraph_properties_choice2: Option<Level1ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level1_paragraph_properties_choice3: Option<Level1ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level1_paragraph_properties_choice4: Option<Level1ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 2 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl2pPr")]
pub struct Level2ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level2_paragraph_properties_choice1: Option<Level2ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level2_paragraph_properties_choice2: Option<Level2ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level2_paragraph_properties_choice3: Option<Level2ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level2_paragraph_properties_choice4: Option<Level2ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 3 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl3pPr")]
pub struct Level3ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level3_paragraph_properties_choice1: Option<Level3ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level3_paragraph_properties_choice2: Option<Level3ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level3_paragraph_properties_choice3: Option<Level3ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level3_paragraph_properties_choice4: Option<Level3ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 4 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl4pPr")]
pub struct Level4ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level4_paragraph_properties_choice1: Option<Level4ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level4_paragraph_properties_choice2: Option<Level4ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level4_paragraph_properties_choice3: Option<Level4ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level4_paragraph_properties_choice4: Option<Level4ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 5 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl5pPr")]
pub struct Level5ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level5_paragraph_properties_choice1: Option<Level5ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level5_paragraph_properties_choice2: Option<Level5ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level5_paragraph_properties_choice3: Option<Level5ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level5_paragraph_properties_choice4: Option<Level5ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 6 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl6pPr")]
pub struct Level6ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level6_paragraph_properties_choice1: Option<Level6ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level6_paragraph_properties_choice2: Option<Level6ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level6_paragraph_properties_choice3: Option<Level6ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level6_paragraph_properties_choice4: Option<Level6ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 7 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl7pPr")]
pub struct Level7ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level7_paragraph_properties_choice1: Option<Level7ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level7_paragraph_properties_choice2: Option<Level7ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level7_paragraph_properties_choice3: Option<Level7ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level7_paragraph_properties_choice4: Option<Level7ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 8 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl8pPr")]
pub struct Level8ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level8_paragraph_properties_choice1: Option<Level8ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level8_paragraph_properties_choice2: Option<Level8ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level8_paragraph_properties_choice3: Option<Level8ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level8_paragraph_properties_choice4: Option<Level8ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// List Level 9 Text Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraphProperties/a:lvl9pPr")]
pub struct Level9ParagraphProperties {
  /// Left Margin
  #[sdk(attr(qname = ":marL"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub left_margin: Option<crate::simple_type::Int32Value>,
  /// Right Margin
  #[sdk(attr(qname = ":marR"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub right_margin: Option<crate::simple_type::Int32Value>,
  /// Level
  #[sdk(attr(qname = ":lvl"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "8",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub level: Option<crate::simple_type::Int32Value>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-51206400",
    max = "51206400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub indent: Option<crate::simple_type::Int32Value>,
  /// Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextAlignmentTypeValues>,
  /// Default Tab Size
  #[sdk(attr(qname = ":defTabSz"))]
  pub default_tab_size: Option<crate::simple_type::Int32Value>,
  /// Right To Left
  #[sdk(attr(qname = ":rtl"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// East Asian Line Break
  #[sdk(attr(qname = ":eaLnBrk"))]
  pub east_asian_line_break: Option<crate::simple_type::BooleanValue>,
  /// Font Alignment
  #[sdk(attr(qname = ":fontAlgn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub font_alignment: Option<TextFontAlignmentValues>,
  /// Latin Line Break
  #[sdk(attr(qname = ":latinLnBrk"))]
  pub latin_line_break: Option<crate::simple_type::BooleanValue>,
  /// Hanging Punctuation
  #[sdk(attr(qname = ":hangingPunct"))]
  pub height: Option<crate::simple_type::BooleanValue>,
  /// Line Spacing
  #[sdk(child(qname = "a:CT_TextSpacing/a:lnSpc"))]
  pub line_spacing: Option<std::boxed::Box<LineSpacing>>,
  /// Space Before
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcBef"))]
  pub space_before: Option<std::boxed::Box<SpaceBefore>>,
  /// Space After
  #[sdk(child(qname = "a:CT_TextSpacing/a:spcAft"))]
  pub space_after: Option<std::boxed::Box<SpaceAfter>>,
  #[sdk(choice(
    qname = "a:CT_TextBulletColorFollowText/a:buClrTx",
    qname = "a:CT_Color/a:buClr"
  ))]
  pub level9_paragraph_properties_choice1: Option<Level9ParagraphPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_TextBulletSizeFollowText/a:buSzTx",
    qname = "a:CT_TextBulletSizePercent/a:buSzPct",
    qname = "a:CT_TextBulletSizePoint/a:buSzPts"
  ))]
  pub level9_paragraph_properties_choice2: Option<Level9ParagraphPropertiesChoice2>,
  #[sdk(choice(
    qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx",
    qname = "a:CT_TextFont/a:buFont"
  ))]
  pub level9_paragraph_properties_choice3: Option<Level9ParagraphPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextNoBullet/a:buNone",
    qname = "a:CT_TextAutonumberBullet/a:buAutoNum",
    qname = "a:CT_TextCharBullet/a:buChar",
    qname = "a:CT_TextBlipBullet/a:buBlip"
  ))]
  pub level9_paragraph_properties_choice4: Option<Level9ParagraphPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextTabStopList/a:tabLst"))]
  pub a_tab_lst: Option<TabStopList>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:defRPr"))]
  pub a_def_r_pr: Option<std::boxed::Box<DefaultRunProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// End Paragraph Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextCharacterProperties/a:endParaRPr")]
pub struct EndParagraphRunProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(number_range(
    source = 0u32,
    min = "100",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub font_size: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-400000",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spacing: Option<crate::simple_type::Int32Value>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::Int32Value>,
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub end_paragraph_run_properties_choice1: Option<EndParagraphRunPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub end_paragraph_run_properties_choice2: Option<EndParagraphRunPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Color/a:highlight"))]
  pub a_highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx",
    qname = "a:CT_LineProperties/a:uLn"
  ))]
  pub end_paragraph_run_properties_choice3: Option<EndParagraphRunPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx",
    qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"
  ))]
  pub end_paragraph_run_properties_choice4: Option<EndParagraphRunPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub a_latin: Option<LatinFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub a_ea: Option<EastAsianFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub a_cs: Option<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:sym"))]
  pub a_sym: Option<SymbolFont>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub a_hlink_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkMouseOver"))]
  pub a_hlink_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// _
  #[sdk(child(qname = "a:CT_Bool/a:rtl"))]
  pub a_rtl: Option<RightToLeft>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Text Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextCharacterProperties/a:rPr")]
pub struct RunProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(number_range(
    source = 0u32,
    min = "100",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub font_size: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-400000",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spacing: Option<crate::simple_type::Int32Value>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::Int32Value>,
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub run_properties_choice1: Option<RunPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub run_properties_choice2: Option<RunPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Color/a:highlight"))]
  pub a_highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx",
    qname = "a:CT_LineProperties/a:uLn"
  ))]
  pub run_properties_choice3: Option<RunPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx",
    qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"
  ))]
  pub run_properties_choice4: Option<RunPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub a_latin: Option<LatinFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub a_ea: Option<EastAsianFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub a_cs: Option<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:sym"))]
  pub a_sym: Option<SymbolFont>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub a_hlink_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkMouseOver"))]
  pub a_hlink_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// _
  #[sdk(child(qname = "a:CT_Bool/a:rtl"))]
  pub a_rtl: Option<RightToLeft>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Default Text Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextCharacterProperties/a:defRPr")]
pub struct DefaultRunProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(number_range(
    source = 0u32,
    min = "100",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub font_size: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub underline: Option<TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub strike: Option<TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub capital: Option<TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-400000",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spacing: Option<crate::simple_type::Int32Value>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::Int32Value>,
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
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<std::boxed::Box<Outline>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub default_run_properties_choice1: Option<DefaultRunPropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub default_run_properties_choice2: Option<DefaultRunPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Color/a:highlight"))]
  pub a_highlight: Option<std::boxed::Box<Highlight>>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx",
    qname = "a:CT_LineProperties/a:uLn"
  ))]
  pub default_run_properties_choice3: Option<DefaultRunPropertiesChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx",
    qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"
  ))]
  pub default_run_properties_choice4: Option<DefaultRunPropertiesChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub a_latin: Option<LatinFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub a_ea: Option<EastAsianFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub a_cs: Option<ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:sym"))]
  pub a_sym: Option<SymbolFont>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub a_hlink_click: Option<std::boxed::Box<HyperlinkOnClick>>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkMouseOver"))]
  pub a_hlink_mouse_over: Option<std::boxed::Box<HyperlinkOnMouseOver>>,
  /// _
  #[sdk(child(qname = "a:CT_Bool/a:rtl"))]
  pub a_rtl: Option<RightToLeft>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Text Paragraphs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextParagraph/a:p")]
pub struct Paragraph {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Text Paragraph Properties
  #[sdk(child(qname = "a:CT_TextParagraphProperties/a:pPr"))]
  pub paragraph_properties: Option<std::boxed::Box<ParagraphProperties>>,
  #[sdk(choice(
    qname = "a:CT_RegularTextRun/a:r",
    qname = "a:CT_TextLineBreak/a:br",
    qname = "a:CT_TextField/a:fld",
    qname = "a14:CT_TextMath/a14:m",
    text,
    any
  ))]
  pub paragraph_choice: Vec<ParagraphChoice>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/a:endParaRPr"))]
  pub a_end_para_r_pr: Option<std::boxed::Box<EndParagraphRunProperties>>,
}
/// Tab Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextTabStop/a:tab")]
pub struct TabStop {
  /// Tab Position
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
  /// Tab Alignment
  #[sdk(attr(qname = ":algn"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub alignment: Option<TextTabAlignmentValues>,
}
/// Spacing Percent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextSpacingPercent/a:spcPct")]
pub struct SpacingPercent {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "13200000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Spacing Points.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextSpacingPoint/a:spcPts")]
pub struct SpacingPoints {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "158400",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Line Spacing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextSpacing/a:lnSpc")]
pub struct LineSpacing {
  #[sdk(choice(
    qname = "a:CT_TextSpacingPercent/a:spcPct",
    qname = "a:CT_TextSpacingPoint/a:spcPts"
  ))]
  pub xml_children: Option<LineSpacingChoice>,
}
/// Space Before.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextSpacing/a:spcBef")]
pub struct SpaceBefore {
  #[sdk(choice(
    qname = "a:CT_TextSpacingPercent/a:spcPct",
    qname = "a:CT_TextSpacingPoint/a:spcPts"
  ))]
  pub xml_children: Option<SpaceBeforeChoice>,
}
/// Space After.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextSpacing/a:spcAft")]
pub struct SpaceAfter {
  #[sdk(choice(
    qname = "a:CT_TextSpacingPercent/a:spcPct",
    qname = "a:CT_TextSpacingPoint/a:spcPts"
  ))]
  pub xml_children: Option<SpaceAfterChoice>,
}
/// Tab List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextTabStopList/a:tabLst")]
pub struct TabStopList {
  /// _
  #[sdk(child(qname = "a:CT_TextTabStop/a:tab"))]
  pub a_tab: Vec<TabStop>,
}
/// Defines the Text Class.
pub type Text = crate::simple_type::StringValue;
/// Defines the ShapePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapePropertiesExtension/a:ext")]
pub struct ShapePropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a:CT_FillProperties/a14:hiddenFill",
    qname = "a:CT_LineProperties/a14:hiddenLine",
    qname = "a:CT_EffectProperties/a14:hiddenEffects",
    qname = "a:CT_Scene3D/a14:hiddenScene3d",
    qname = "a:CT_Shape3D/a14:hiddenSp3d",
    qname = "a14:CT_ShadowObscured/a14:shadowObscured",
    any
  ))]
  pub xml_children: Option<ShapePropertiesExtensionChoice>,
}
/// Defines the GvmlGroupShapeExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShapeExtension/a:ext")]
pub struct GvmlGroupShapeExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "a14:CT_IsGvmlCanvas/a14:isCanvas", any))]
  pub xml_children: Option<GvmlGroupShapeExtensionChoice>,
}
/// Defines the ShapePropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapePropertiesExtensionList/a:extLst")]
pub struct ShapePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtension/a:ext"))]
  pub a_ext: Vec<ShapePropertiesExtension>,
}
/// Non-Visual Properties for a Group Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingProps/a:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// Non-Visual Group Shape Drawing Properties
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapeProps/a:cNvGrpSpPr"))]
  pub non_visual_group_shape_drawing_properties:
    std::boxed::Box<NonVisualGroupShapeDrawingProperties>,
}
/// Visual Group Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupShapeProperties/a:grpSpPr")]
pub struct VisualGroupShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode: Option<BlackWhiteModeValues>,
  /// 2D Transform for Grouped Objects
  #[sdk(child(qname = "a:CT_GroupTransform2D/a:xfrm"))]
  pub transform_group: Option<std::boxed::Box<TransformGroup>>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub visual_group_shape_properties_choice1: Option<VisualGroupShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub visual_group_shape_properties_choice2: Option<VisualGroupShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<Scene3DType>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst: Option<ExtensionList>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlShape/a:sp")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Non-Visual Properties for a Shape
  #[sdk(child(qname = "a:CT_GvmlShapeNonVisual/a:nvSpPr"))]
  pub non_visual_shape_properties: std::boxed::Box<NonVisualShapeProperties>,
  /// Visual Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Text Shape
  #[sdk(child(qname = "a:CT_GvmlTextShape/a:txSp"))]
  pub text_shape: Option<std::boxed::Box<TextShape>>,
  /// Style
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Connection Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlConnector/a:cxnSp")]
pub struct ConnectionShape {
  /// Non-Visual Properties for a Connection Shape
  #[sdk(child(qname = "a:CT_GvmlConnectorNonVisual/a:nvCxnSpPr"))]
  pub non_visual_connection_shape_properties: std::boxed::Box<NonVisualConnectionShapeProperties>,
  /// Visual Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Picture.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlPicture/a:pic")]
pub struct Picture {
  /// Non-Visual Properties for a Picture
  #[sdk(child(qname = "a:CT_GvmlPictureNonVisual/a:nvPicPr"))]
  pub non_visual_picture_properties: std::boxed::Box<NonVisualPictureProperties>,
  /// Picture Fill
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  pub blip_fill: std::boxed::Box<BlipFill>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/a:spPr"))]
  pub shape_properties: std::boxed::Box<ShapeProperties>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeStyle/a:style"))]
  pub shape_style: Option<std::boxed::Box<ShapeStyle>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Graphic Frame.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame")]
pub struct GraphicFrame {
  /// Non-Visual Properties for a Graphic Frame
  #[sdk(child(qname = "a:CT_GvmlGraphicFrameNonVisual/a:nvGraphicFramePr"))]
  pub non_visual_graphic_frame_properties: std::boxed::Box<NonVisualGraphicFrameProperties>,
  /// _
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  pub graphic: std::boxed::Box<Graphic>,
  /// _
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: std::boxed::Box<Transform2D>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Group shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShape/a:grpSp")]
pub struct GroupShape {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Non-Visual Properties for a Group Shape
  #[sdk(child(qname = "a:CT_GvmlGroupShapeNonVisual/a:nvGrpSpPr"))]
  pub non_visual_group_shape_properties: std::boxed::Box<NonVisualGroupShapeProperties>,
  /// Visual Group Shape Properties
  #[sdk(child(qname = "a:CT_GroupShapeProperties/a:grpSpPr"))]
  pub visual_group_shape_properties: std::boxed::Box<VisualGroupShapeProperties>,
  #[sdk(choice(
    qname = "a:CT_GvmlTextShape/a:txSp",
    qname = "a:CT_GvmlShape/a:sp",
    qname = "a:CT_GvmlConnector/a:cxnSp",
    qname = "a:CT_GvmlPicture/a:pic",
    qname = "a14:CT_GvmlContentPart/a14:contentPart",
    qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame",
    qname = "a:CT_GvmlGroupShape/a:grpSp",
    text,
    any
  ))]
  pub group_shape_choice: Vec<GroupShapeChoice>,
  /// _
  #[sdk(child(qname = "a:CT_GvmlGroupShapeExtensionList/a:extLst"))]
  pub a_ext_lst: Option<GvmlGroupShapeExtensionList>,
}
/// Defines the GvmlGroupShapeExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GvmlGroupShapeExtensionList/a:extLst")]
pub struct GvmlGroupShapeExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_GvmlGroupShapeExtension/a:ext"))]
  pub a_ext: Vec<GvmlGroupShapeExtension>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapePropsExtension/a:ext")]
pub struct NonVisualGroupDrawingShapePropsExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "a15:CT_NonVisualGroupProps/a15:nonVisualGroupProps", any))]
  pub xml_children: Option<NonVisualGroupDrawingShapePropsExtensionChoice>,
}
/// Defines the OfficeStyleSheetExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeStyleSheetExtension/a:ext")]
pub struct OfficeStyleSheetExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "thm15:CT_ThemeFamily/thm15:themeFamily", any))]
  pub xml_children: Option<OfficeStyleSheetExtensionChoice>,
}
/// Defines the ConnectorLockingExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ConnectorLockingExtension/a:ext")]
pub struct ConnectorLockingExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "a:CT_GraphicalObject/a:graphic", any))]
  pub xml_children: Option<ConnectorLockingExtensionChoice>,
}
/// Defines the GroupShapeLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_GroupLocking/a:grpSpLocks")]
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
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualGroupDrawingShapePropsExtensionList/a:extLst")]
pub struct NonVisualGroupDrawingShapePropsExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualGroupDrawingShapePropsExtension/a:ext"))]
  pub a_ext: Vec<NonVisualGroupDrawingShapePropsExtension>,
}
/// Defines the ObjectDefaults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ObjectStyleDefaults/a:objectDefaults")]
pub struct ObjectDefaults {
  /// Shape Default
  #[sdk(child(qname = "a:CT_DefaultShapeDefinition/a:spDef"))]
  pub shape_default: Option<std::boxed::Box<ShapeDefault>>,
  /// Line Default
  #[sdk(child(qname = "a:CT_DefaultShapeDefinition/a:lnDef"))]
  pub line_default: Option<std::boxed::Box<LineDefault>>,
  /// Text Default
  #[sdk(child(qname = "a:CT_DefaultShapeDefinition/a:txDef"))]
  pub text_default: Option<std::boxed::Box<TextDefault>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtraColorSchemeList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorSchemeList/a:extraClrSchemeLst")]
pub struct ExtraColorSchemeList {
  /// _
  #[sdk(child(qname = "a:CT_ColorSchemeAndMapping/a:extraClrScheme"))]
  pub a_extra_clr_scheme: Vec<ExtraColorScheme>,
}
/// Defines the CustomColorList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_CustomColorList/a:custClrLst")]
pub struct CustomColorList {
  /// _
  #[sdk(child(qname = "a:CT_CustomColor/a:custClr"))]
  pub a_cust_clr: Vec<CustomColor>,
}
/// Defines the OfficeStyleSheetExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeStyleSheetExtensionList/a:extLst")]
pub struct OfficeStyleSheetExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeStyleSheetExtension/a:ext"))]
  pub a_ext: Vec<OfficeStyleSheetExtension>,
}
/// Defines the HyperlinkOnClick Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Hyperlink/a:hlinkClick")]
pub struct HyperlinkOnClick {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:CT_HyperlinkExtensionList/a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnMouseOver Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Hyperlink/a:hlinkMouseOver")]
pub struct HyperlinkOnMouseOver {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:CT_HyperlinkExtensionList/a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnHover Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Hyperlink/a:hlinkHover")]
pub struct HyperlinkOnHover {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
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
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:snd"))]
  pub hyperlink_sound: Option<HyperlinkSound>,
  /// Future extensions.
  #[sdk(child(qname = "a:CT_HyperlinkExtensionList/a:extLst"))]
  pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the RightToLeft Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Bool/a:rtl")]
pub struct RightToLeft {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the NonVisualDrawingPropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst")]
pub struct NonVisualDrawingPropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtension/a:ext"))]
  pub a_ext: Vec<NonVisualDrawingPropertiesExtension>,
}
/// Defines the ConnectorLockingExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ConnectorLockingExtensionList/a:extLst")]
pub struct ConnectorLockingExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_ConnectorLockingExtension/a:ext"))]
  pub a_ext: Vec<ConnectorLockingExtension>,
}
/// Defines the DataModelExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DataModelExtension/a:ext")]
pub struct DataModelExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "dsp:CT_DataModelExtBlock/dsp:dataModelExt",
    qname = "dgm14:CT_Boolean/dgm14:recolorImg",
    any
  ))]
  pub xml_children: Option<DataModelExtensionChoice>,
}
/// Defines the PtExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PtExtension/a:ext")]
pub struct PtExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "a:CT_NonVisualDrawingProps/dgm14:cNvPr", any))]
  pub xml_children: Option<PtExtensionChoice>,
}
/// Defines the HyperlinkExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_HyperlinkExtension/a:ext")]
pub struct HyperlinkExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "ahyp:CT_HyperlinkColor/ahyp:hlinkClr", any))]
  pub xml_children: Option<HyperlinkExtensionChoice>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_HyperlinkExtensionList/a:extLst")]
pub struct HyperlinkExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_HyperlinkExtension/a:ext"))]
  pub a_ext: Vec<HyperlinkExtension>,
}
/// Defines the LinePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LinePropertiesExtension/a:ext")]
pub struct LinePropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps",
    any
  ))]
  pub xml_children: Option<LinePropertiesExtensionChoice>,
}
/// default head line end style is none.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineEndProperties/a:headEnd")]
pub struct HeadEnd {
  /// Line Head/End Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<LineEndValues>,
  /// Width of Head/End
  #[sdk(attr(qname = ":w"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub width: Option<LineEndWidthValues>,
  /// Length of Head/End
  #[sdk(attr(qname = ":len"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub length: Option<LineEndLengthValues>,
}
/// default tail line end style is none.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LineEndProperties/a:tailEnd")]
pub struct TailEnd {
  /// Line Head/End Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<LineEndValues>,
  /// Width of Head/End
  #[sdk(attr(qname = ":w"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub width: Option<LineEndWidthValues>,
  /// Length of Head/End
  #[sdk(attr(qname = ":len"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub length: Option<LineEndLengthValues>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_LinePropertiesExtensionList/a:extLst")]
pub struct LinePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_LinePropertiesExtension/a:ext"))]
  pub a_ext: Vec<LinePropertiesExtension>,
}
/// Defines the NonVisualDrawingPropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualDrawingPropsExtension/a:ext")]
pub struct NonVisualDrawingPropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a14:CT_CompatExt/a14:compatExt",
    qname = "a15:CT_BackgroundPr/a15:backgroundPr",
    qname = "a16:CT_CreationId/a16:creationId",
    qname = "a16:CT_PredecessorDrawingElementReference/a16:predDERef",
    qname = "adec:CT_Decorative/adec:decorative",
    qname = "aclsh:CT_ClassificationOutcome/aclsh:classification",
    qname = "asl:CT_ScriptLink/asl:scriptLink",
    any
  ))]
  pub xml_children: Option<NonVisualDrawingPropertiesExtensionChoice>,
}
/// Defines the PictureLocks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PictureLocking/a:picLocks")]
pub struct PictureLocks {
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
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualPicturePropertiesExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPicturePropertiesExtensionList/a:extLst")]
pub struct NonVisualPicturePropertiesExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_NonVisualPicturePropertiesExtension/a:ext"))]
  pub a_ext: Vec<NonVisualPicturePropertiesExtension>,
}
/// Defines the NonVisualPicturePropertiesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_NonVisualPicturePropertiesExtension/a:ext")]
pub struct NonVisualPicturePropertiesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a14:CT_CameraTool/a14:cameraTool",
    qname = "a15:CT_SignatureLine/a15:signatureLine",
    qname = "a15:CT_ObjectPr/a15:objectPr",
    qname = "alf:CT_LiveFeedProperties/alf:liveFeedProps",
    qname = "aif:CT_ImageFormula/aif:imageFormula",
    any
  ))]
  pub xml_children: Option<NonVisualPicturePropertiesExtensionChoice>,
}
/// Future extensions..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipExtensionList/a:extLst")]
pub struct BlipExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_BlipExtension/a:ext"))]
  pub a_ext: Vec<BlipExtension>,
}
/// Defines the BlipExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BlipExtension/a:ext")]
pub struct BlipExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "a14:CT_Photo/a14:imgProps",
    qname = "a14:CT_UseLocalDpi/a14:useLocalDpi",
    qname = "wp15:CT_WebVideoPr/wp15:webVideoPr",
    qname = "asvg:CT_SVGBlip/asvg:svgBlip",
    qname = "a1611:CT_PictureAttributionSourceURL/a1611:picAttrSrcUrl",
    qname = "woe:CT_OEmbed/woe:oembed",
    qname = "aoe:CT_OEmbedShared/aoe:oembedShared",
    any
  ))]
  pub xml_children: Option<BlipExtensionChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RgbColorModelPercentageChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RgbColorModelHexChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HslColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SystemColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SchemeColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresetColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(std::boxed::Box<AlphaOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulation>),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(std::boxed::Box<HueOffset>),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(std::boxed::Box<HueModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(std::boxed::Box<SaturationOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(std::boxed::Box<SaturationModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(std::boxed::Box<LuminanceOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(std::boxed::Box<LuminanceModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(std::boxed::Box<RedOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(std::boxed::Box<RedModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(std::boxed::Box<GreenOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(std::boxed::Box<GreenModulation>),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(std::boxed::Box<BlueOffset>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(std::boxed::Box<BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SolidFillChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientFillChoice {
  #[sdk(child(qname = "a:CT_LinearShadeProperties/a:lin"))]
  ALin(std::boxed::Box<LinearGradientFill>),
  #[sdk(child(qname = "a:CT_PathShadeProperties/a:path"))]
  APath(std::boxed::Box<PathGradientFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipFillChoice {
  #[sdk(child(qname = "a:CT_TileInfoProperties/a:tile"))]
  ATile(std::boxed::Box<Tile>),
  #[sdk(child(qname = "a:CT_StretchInfoProperties/a:stretch"))]
  AStretch(std::boxed::Box<Stretch>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectContainerChoice {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:cont"))]
  ACont(std::boxed::Box<EffectContainer>),
  /// Effect.
  #[sdk(child(qname = "a:CT_EffectReference/a:effect"))]
  AEffect(std::boxed::Box<Effect>),
  /// Defines the AlphaBiLevel Class.
  #[sdk(child(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel"))]
  AAlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AAlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AAlphaFloor,
  /// Alpha Inverse Effect.
  #[sdk(child(qname = "a:CT_AlphaInverseEffect/a:alphaInv"))]
  AAlphaInv(std::boxed::Box<AlphaInverse>),
  /// Alpha Modulate Effect.
  #[sdk(child(qname = "a:CT_AlphaModulateEffect/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulationEffect>),
  /// Defines the AlphaModulationFixed Class.
  #[sdk(child(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix"))]
  AAlphaModFix(std::boxed::Box<AlphaModulationFixed>),
  /// Alpha Inset/Outset Effect.
  #[sdk(child(qname = "a:CT_AlphaOutsetEffect/a:alphaOutset"))]
  AAlphaOutset(std::boxed::Box<AlphaOutset>),
  /// Alpha Replace Effect.
  #[sdk(child(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl"))]
  AAlphaRepl(std::boxed::Box<AlphaReplace>),
  /// Defines the BiLevel Class.
  #[sdk(child(qname = "a:CT_BiLevelEffect/a:biLevel"))]
  ABiLevel(std::boxed::Box<BiLevel>),
  /// Blend Effect.
  #[sdk(child(qname = "a:CT_BlendEffect/a:blend"))]
  ABlend(std::boxed::Box<Blend>),
  /// Defines the Blur Class.
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  ABlur(std::boxed::Box<Blur>),
  /// Color Change Effect.
  #[sdk(child(qname = "a:CT_ColorChangeEffect/a:clrChange"))]
  AClrChange(std::boxed::Box<ColorChange>),
  /// Defines the ColorReplacement Class.
  #[sdk(child(qname = "a:CT_ColorReplaceEffect/a:clrRepl"))]
  AClrRepl(std::boxed::Box<ColorReplacement>),
  /// Duotone Effect.
  #[sdk(child(qname = "a:CT_DuotoneEffect/a:duotone"))]
  ADuotone(std::boxed::Box<Duotone>),
  /// Fill.
  #[sdk(child(qname = "a:CT_FillEffect/a:fill"))]
  AFill(std::boxed::Box<Fill>),
  /// Fill Overlay Effect.
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  AFillOverlay(std::boxed::Box<FillOverlay>),
  /// Glow Effect.
  #[sdk(child(qname = "a:CT_GlowEffect/a:glow"))]
  AGlow(std::boxed::Box<Glow>),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  AGrayscl,
  /// Hue Saturation Luminance Effect.
  #[sdk(child(qname = "a:CT_HSLEffect/a:hsl"))]
  AHsl(std::boxed::Box<Hsl>),
  /// Inner Shadow Effect.
  #[sdk(child(qname = "a:CT_InnerShadowEffect/a:innerShdw"))]
  AInnerShdw(std::boxed::Box<InnerShadow>),
  /// Luminance.
  #[sdk(child(qname = "a:CT_LuminanceEffect/a:lum"))]
  ALum(std::boxed::Box<LuminanceEffect>),
  /// Outer Shadow Effect.
  #[sdk(child(qname = "a:CT_OuterShadowEffect/a:outerShdw"))]
  AOuterShdw(std::boxed::Box<OuterShadow>),
  /// Preset Shadow.
  #[sdk(child(qname = "a:CT_PresetShadowEffect/a:prstShdw"))]
  APrstShdw(std::boxed::Box<PresetShadow>),
  /// Reflection Effect.
  #[sdk(child(qname = "a:CT_ReflectionEffect/a:reflection"))]
  AReflection(std::boxed::Box<Reflection>),
  /// Relative Offset Effect.
  #[sdk(child(qname = "a:CT_RelativeOffsetEffect/a:relOff"))]
  ARelOff(std::boxed::Box<RelativeOffset>),
  /// Soft Edge Effect.
  #[sdk(child(qname = "a:CT_SoftEdgesEffect/a:softEdge"))]
  ASoftEdge(std::boxed::Box<SoftEdge>),
  /// Defines the TintEffect Class.
  #[sdk(child(qname = "a:CT_TintEffect/a:tint"))]
  ATint(std::boxed::Box<TintEffect>),
  /// Transform Effect.
  #[sdk(child(qname = "a:CT_TransformEffect/a:xfrm"))]
  AXfrm(std::boxed::Box<TransformEffect>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectDagChoice {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:cont"))]
  ACont(std::boxed::Box<EffectContainer>),
  /// Effect.
  #[sdk(child(qname = "a:CT_EffectReference/a:effect"))]
  AEffect(std::boxed::Box<Effect>),
  /// Defines the AlphaBiLevel Class.
  #[sdk(child(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel"))]
  AAlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AAlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AAlphaFloor,
  /// Alpha Inverse Effect.
  #[sdk(child(qname = "a:CT_AlphaInverseEffect/a:alphaInv"))]
  AAlphaInv(std::boxed::Box<AlphaInverse>),
  /// Alpha Modulate Effect.
  #[sdk(child(qname = "a:CT_AlphaModulateEffect/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulationEffect>),
  /// Defines the AlphaModulationFixed Class.
  #[sdk(child(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix"))]
  AAlphaModFix(std::boxed::Box<AlphaModulationFixed>),
  /// Alpha Inset/Outset Effect.
  #[sdk(child(qname = "a:CT_AlphaOutsetEffect/a:alphaOutset"))]
  AAlphaOutset(std::boxed::Box<AlphaOutset>),
  /// Alpha Replace Effect.
  #[sdk(child(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl"))]
  AAlphaRepl(std::boxed::Box<AlphaReplace>),
  /// Defines the BiLevel Class.
  #[sdk(child(qname = "a:CT_BiLevelEffect/a:biLevel"))]
  ABiLevel(std::boxed::Box<BiLevel>),
  /// Blend Effect.
  #[sdk(child(qname = "a:CT_BlendEffect/a:blend"))]
  ABlend(std::boxed::Box<Blend>),
  /// Defines the Blur Class.
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  ABlur(std::boxed::Box<Blur>),
  /// Color Change Effect.
  #[sdk(child(qname = "a:CT_ColorChangeEffect/a:clrChange"))]
  AClrChange(std::boxed::Box<ColorChange>),
  /// Defines the ColorReplacement Class.
  #[sdk(child(qname = "a:CT_ColorReplaceEffect/a:clrRepl"))]
  AClrRepl(std::boxed::Box<ColorReplacement>),
  /// Duotone Effect.
  #[sdk(child(qname = "a:CT_DuotoneEffect/a:duotone"))]
  ADuotone(std::boxed::Box<Duotone>),
  /// Fill.
  #[sdk(child(qname = "a:CT_FillEffect/a:fill"))]
  AFill(std::boxed::Box<Fill>),
  /// Fill Overlay Effect.
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  AFillOverlay(std::boxed::Box<FillOverlay>),
  /// Glow Effect.
  #[sdk(child(qname = "a:CT_GlowEffect/a:glow"))]
  AGlow(std::boxed::Box<Glow>),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  AGrayscl,
  /// Hue Saturation Luminance Effect.
  #[sdk(child(qname = "a:CT_HSLEffect/a:hsl"))]
  AHsl(std::boxed::Box<Hsl>),
  /// Inner Shadow Effect.
  #[sdk(child(qname = "a:CT_InnerShadowEffect/a:innerShdw"))]
  AInnerShdw(std::boxed::Box<InnerShadow>),
  /// Luminance.
  #[sdk(child(qname = "a:CT_LuminanceEffect/a:lum"))]
  ALum(std::boxed::Box<LuminanceEffect>),
  /// Outer Shadow Effect.
  #[sdk(child(qname = "a:CT_OuterShadowEffect/a:outerShdw"))]
  AOuterShdw(std::boxed::Box<OuterShadow>),
  /// Preset Shadow.
  #[sdk(child(qname = "a:CT_PresetShadowEffect/a:prstShdw"))]
  APrstShdw(std::boxed::Box<PresetShadow>),
  /// Reflection Effect.
  #[sdk(child(qname = "a:CT_ReflectionEffect/a:reflection"))]
  AReflection(std::boxed::Box<Reflection>),
  /// Relative Offset Effect.
  #[sdk(child(qname = "a:CT_RelativeOffsetEffect/a:relOff"))]
  ARelOff(std::boxed::Box<RelativeOffset>),
  /// Soft Edge Effect.
  #[sdk(child(qname = "a:CT_SoftEdgesEffect/a:softEdge"))]
  ASoftEdge(std::boxed::Box<SoftEdge>),
  /// Defines the TintEffect Class.
  #[sdk(child(qname = "a:CT_TintEffect/a:tint"))]
  ATint(std::boxed::Box<TintEffect>),
  /// Transform Effect.
  #[sdk(child(qname = "a:CT_TransformEffect/a:xfrm"))]
  AXfrm(std::boxed::Box<TransformEffect>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AlphaInverseChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorReplacementChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DuotoneChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillOverlayChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GlowChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InnerShadowChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OuterShadowChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PresetShadowChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillPropertiesChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectPropertiesTypeChoice {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FontReferenceChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BulletColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ExtrusionColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ContourColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorFromChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorToChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ForegroundColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HighlightChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UnderlineChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UnderlineChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UnderlineChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OutlineChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OutlineChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OutlineChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LeftBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LeftBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LeftBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RightBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RightBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopLeftToBottomRightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_PresetLineDashProperties/a:prstDash"))]
  APrstDash(std::boxed::Box<PresetDash>),
  #[sdk(child(qname = "a:CT_DashStopList/a:custDash"))]
  ACustDash(std::boxed::Box<CustomDash>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomLeftToTopRightBorderLinePropertiesChoice3 {
  /// Round Line Join.
  #[sdk(empty_child(qname = "a:CT_LineJoinRound/a:round"))]
  ARound,
  /// Line Join Bevel.
  #[sdk(empty_child(qname = "a:CT_LineJoinBevel/a:bevel"))]
  ABevel,
  #[sdk(child(qname = "a:CT_LineJoinMiterProperties/a:miter"))]
  AMiter(std::boxed::Box<Miter>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UnderlineFillChoice {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipChoice {
  #[sdk(child(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel"))]
  AAlphaBiLevel(std::boxed::Box<AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AAlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AAlphaFloor,
  #[sdk(child(qname = "a:CT_AlphaInverseEffect/a:alphaInv"))]
  AAlphaInv(std::boxed::Box<AlphaInverse>),
  #[sdk(child(qname = "a:CT_AlphaModulateEffect/a:alphaMod"))]
  AAlphaMod(std::boxed::Box<AlphaModulationEffect>),
  #[sdk(child(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix"))]
  AAlphaModFix(std::boxed::Box<AlphaModulationFixed>),
  #[sdk(child(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl"))]
  AAlphaRepl(std::boxed::Box<AlphaReplace>),
  #[sdk(child(qname = "a:CT_BiLevelEffect/a:biLevel"))]
  ABiLevel(std::boxed::Box<BiLevel>),
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  ABlur(std::boxed::Box<Blur>),
  #[sdk(child(qname = "a:CT_ColorChangeEffect/a:clrChange"))]
  AClrChange(std::boxed::Box<ColorChange>),
  #[sdk(child(qname = "a:CT_ColorReplaceEffect/a:clrRepl"))]
  AClrRepl(std::boxed::Box<ColorReplacement>),
  #[sdk(child(qname = "a:CT_DuotoneEffect/a:duotone"))]
  ADuotone(std::boxed::Box<Duotone>),
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  AFillOverlay(std::boxed::Box<FillOverlay>),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  AGrayscl,
  #[sdk(child(qname = "a:CT_HSLEffect/a:hsl"))]
  AHsl(std::boxed::Box<Hsl>),
  #[sdk(child(qname = "a:CT_LuminanceEffect/a:lum"))]
  ALum(std::boxed::Box<LuminanceEffect>),
  #[sdk(child(qname = "a:CT_TintEffect/a:tint"))]
  ATint(std::boxed::Box<TintEffect>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectStyleChoice {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillStyleListChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundFillStyleListChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Dark1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Light1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Dark2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Light2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent1ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent2ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent3ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent4ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent5ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Accent6ColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HyperlinkChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FollowedHyperlinkColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(std::boxed::Box<CustomGeometry>),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(std::boxed::Box<PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextShapeChoice {
  /// Use Shape Text Rectangle.
  #[sdk(empty_child(qname = "a:CT_GvmlUseShapeRectangle/a:useSpRect"))]
  AUseSpRect,
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  AXfrm(std::boxed::Box<Transform2D>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientStopChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PathChoice {
  /// Close Shape Path.
  #[sdk(empty_child(qname = "a:CT_Path2DClose/a:close"))]
  AClose,
  /// Move Path To.
  #[sdk(child(qname = "a:CT_Path2DMoveTo/a:moveTo"))]
  AMoveTo(std::boxed::Box<MoveTo>),
  /// Draw Line To.
  #[sdk(child(qname = "a:CT_Path2DLineTo/a:lnTo"))]
  ALnTo(std::boxed::Box<LineTo>),
  /// Draw Arc To.
  #[sdk(child(qname = "a:CT_Path2DArcTo/a:arcTo"))]
  AArcTo(std::boxed::Box<ArcTo>),
  /// Draw Quadratic Bezier Curve To.
  #[sdk(child(qname = "a:CT_Path2DQuadBezierTo/a:quadBezTo"))]
  AQuadBezTo(std::boxed::Box<QuadraticBezierCurveTo>),
  /// Draw Cubic Bezier Curve To.
  #[sdk(child(qname = "a:CT_Path2DCubicBezierTo/a:cubicBezTo"))]
  ACubicBezTo(std::boxed::Box<CubicBezierCurveTo>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AdjustHandleListChoice {
  /// XY Adjust Handle.
  #[sdk(child(qname = "a:CT_XYAdjustHandle/a:ahXY"))]
  AAhXy(std::boxed::Box<AdjustHandleXy>),
  /// Polar Adjust Handle.
  #[sdk(child(qname = "a:CT_PolarAdjustHandle/a:ahPolar"))]
  AAhPolar(std::boxed::Box<AdjustHandlePolar>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit,
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(std::boxed::Box<NormalAutoFit>),
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(std::boxed::Box<ShapeAutoFit>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BodyPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(std::boxed::Box<Shape3DType>),
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(std::boxed::Box<FlatText>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellPropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TablePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TablePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TablePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_TableStyle/a:tableStyle"))]
  ATableStyle(std::boxed::Box<TableStyle>),
  #[sdk(text_child(qname = "a:ST_Guid/a:tableStyleId"))]
  ATableStyleId(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LeftBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RightBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BottomBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InsideHorizontalBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum InsideVerticalBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopLeftToBottomRightBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TopRightToBottomLeftBorderChoice {
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  ALn(std::boxed::Box<Outline>),
  /// Defines the LineReference Class.
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:lnRef"))]
  ALnRef(std::boxed::Box<LineReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellTextStyleChoice {
  #[sdk(child(qname = "a:CT_FontCollection/a:font"))]
  AFont(std::boxed::Box<Fonts>),
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  AFontRef(std::boxed::Box<FontReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellTextStyleChoice2 {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(std::boxed::Box<RgbColorModelPercentage>),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(std::boxed::Box<RgbColorModelHex>),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(std::boxed::Box<HslColor>),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(std::boxed::Box<SystemColor>),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(std::boxed::Box<SchemeColor>),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(std::boxed::Box<PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableCellStyleChoice {
  #[sdk(child(qname = "a:CT_FillProperties/a:fill"))]
  AFill(std::boxed::Box<FillProperties>),
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  AFillRef(std::boxed::Box<FillReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableBackgroundChoice {
  #[sdk(child(qname = "a:CT_FillProperties/a:fill"))]
  AFill(std::boxed::Box<FillProperties>),
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:fillRef"))]
  AFillRef(std::boxed::Box<FillReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableBackgroundChoice2 {
  #[sdk(child(qname = "a:CT_EffectProperties/a:effect"))]
  AEffect(std::boxed::Box<EffectPropertiesType>),
  #[sdk(child(qname = "a:CT_StyleMatrixReference/a:effectRef"))]
  AEffectRef(std::boxed::Box<EffectReference>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level1ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level1ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level1ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level1ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level2ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level2ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level2ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level2ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level3ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level3ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level3ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level3ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level4ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level4ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level4ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level4ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level5ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level5ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level5ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level5ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level6ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level6ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level6ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level6ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level7ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level7ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level7ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level7ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level8ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level8ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level8ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level8ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level9ParagraphPropertiesChoice {
  /// Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletColorFollowText/a:buClrTx"))]
  ABuClrTx,
  #[sdk(child(qname = "a:CT_Color/a:buClr"))]
  ABuClr(std::boxed::Box<BulletColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level9ParagraphPropertiesChoice2 {
  /// Bullet Size Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextBulletSizeFollowText/a:buSzTx"))]
  ABuSzTx,
  #[sdk(child(qname = "a:CT_TextBulletSizePercent/a:buSzPct"))]
  ABuSzPct(std::boxed::Box<BulletSizePercentage>),
  #[sdk(child(qname = "a:CT_TextBulletSizePoint/a:buSzPts"))]
  ABuSzPts(std::boxed::Box<BulletSizePoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level9ParagraphPropertiesChoice3 {
  /// Follow text.
  #[sdk(empty_child(qname = "a:CT_TextBulletTypefaceFollowText/a:buFontTx"))]
  ABuFontTx,
  #[sdk(child(qname = "a:CT_TextFont/a:buFont"))]
  ABuFont(std::boxed::Box<BulletFont>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Level9ParagraphPropertiesChoice4 {
  /// No Bullet.
  #[sdk(empty_child(qname = "a:CT_TextNoBullet/a:buNone"))]
  ABuNone,
  #[sdk(child(qname = "a:CT_TextAutonumberBullet/a:buAutoNum"))]
  ABuAutoNum(std::boxed::Box<AutoNumberedBullet>),
  #[sdk(child(qname = "a:CT_TextCharBullet/a:buChar"))]
  ABuChar(std::boxed::Box<CharacterBullet>),
  #[sdk(child(qname = "a:CT_TextBlipBullet/a:buBlip"))]
  ABuBlip(std::boxed::Box<PictureBullet>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndParagraphRunPropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndParagraphRunPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndParagraphRunPropertiesChoice3 {
  /// Underline Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx"))]
  AULnTx,
  #[sdk(child(qname = "a:CT_LineProperties/a:uLn"))]
  AULn(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EndParagraphRunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx"))]
  AUFillTx,
  #[sdk(child(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"))]
  AUFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice3 {
  /// Underline Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx"))]
  AULnTx,
  #[sdk(child(qname = "a:CT_LineProperties/a:uLn"))]
  AULn(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx"))]
  AUFillTx,
  #[sdk(child(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"))]
  AUFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultRunPropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultRunPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultRunPropertiesChoice3 {
  /// Underline Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx"))]
  AULnTx,
  #[sdk(child(qname = "a:CT_LineProperties/a:uLn"))]
  AULn(std::boxed::Box<Underline>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DefaultRunPropertiesChoice4 {
  /// Underline Fill Properties Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx"))]
  AUFillTx,
  #[sdk(child(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"))]
  AUFill(std::boxed::Box<UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ParagraphChoice {
  #[sdk(child(qname = "a:CT_RegularTextRun/a:r"))]
  AR(std::boxed::Box<Run>),
  #[sdk(child(qname = "a:CT_TextLineBreak/a:br"))]
  ABr(std::boxed::Box<Break>),
  #[sdk(child(qname = "a:CT_TextField/a:fld"))]
  AFld(std::boxed::Box<Field>),
  /// Defines the TextMath Class.
  #[sdk(empty_child(office2010, qname = "a14:CT_TextMath/a14:m"))]
  A14M,
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineSpacingChoice {
  /// Spacing Percent.
  #[sdk(child(qname = "a:CT_TextSpacingPercent/a:spcPct"))]
  ASpcPct(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  #[sdk(child(qname = "a:CT_TextSpacingPoint/a:spcPts"))]
  ASpcPts(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SpaceBeforeChoice {
  /// Spacing Percent.
  #[sdk(child(qname = "a:CT_TextSpacingPercent/a:spcPct"))]
  ASpcPct(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  #[sdk(child(qname = "a:CT_TextSpacingPoint/a:spcPts"))]
  ASpcPts(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SpaceAfterChoice {
  /// Spacing Percent.
  #[sdk(child(qname = "a:CT_TextSpacingPercent/a:spcPct"))]
  ASpcPct(std::boxed::Box<SpacingPercent>),
  /// Spacing Points.
  #[sdk(child(qname = "a:CT_TextSpacingPoint/a:spcPts"))]
  ASpcPts(std::boxed::Box<SpacingPoints>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesExtensionChoice {
  #[sdk(child(office2010, qname = "a:CT_FillProperties/a14:hiddenFill"))]
  A14HiddenFill(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenFillProperties,
    >,
  ),
  #[sdk(child(office2010, qname = "a:CT_LineProperties/a14:hiddenLine"))]
  A14HiddenLine(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenLineProperties,
    >,
  ),
  #[sdk(child(office2010, qname = "a:CT_EffectProperties/a14:hiddenEffects"))]
  A14HiddenEffects(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenEffectsProperties,
    >,
  ),
  #[sdk(child(office2010, qname = "a:CT_Scene3D/a14:hiddenScene3d"))]
  A14HiddenScene3d(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenScene3D>,
  ),
  #[sdk(child(office2010, qname = "a:CT_Shape3D/a14:hiddenSp3d"))]
  A14HiddenSp3d(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenShape3D>,
  ),
  #[sdk(child(office2010, qname = "a14:CT_ShadowObscured/a14:shadowObscured"))]
  A14ShadowObscured(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ShadowObscured>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GvmlGroupShapeExtensionChoice {
  #[sdk(child(office2010, qname = "a14:CT_IsGvmlCanvas/a14:isCanvas"))]
  A14IsCanvas(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::IsCanvas>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VisualGroupShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<SolidFill>),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<GradientFill>),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<BlipFill>),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum VisualGroupShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<EffectList>),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupShapeChoice {
  #[sdk(child(qname = "a:CT_GvmlTextShape/a:txSp"))]
  ATxSp(std::boxed::Box<TextShape>),
  #[sdk(child(qname = "a:CT_GvmlShape/a:sp"))]
  ASp(std::boxed::Box<Shape>),
  #[sdk(child(qname = "a:CT_GvmlConnector/a:cxnSp"))]
  ACxnSp(std::boxed::Box<ConnectionShape>),
  #[sdk(child(qname = "a:CT_GvmlPicture/a:pic"))]
  APic(std::boxed::Box<Picture>),
  #[sdk(child(office2010, qname = "a14:CT_GvmlContentPart/a14:contentPart"))]
  A14ContentPart(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::GvmlContentPart,
    >,
  ),
  #[sdk(child(qname = "a:CT_GvmlGraphicalObjectFrame/a:graphicFrame"))]
  AGraphicFrame(std::boxed::Box<GraphicFrame>),
  #[sdk(child(qname = "a:CT_GvmlGroupShape/a:grpSp"))]
  AGrpSp(std::boxed::Box<GroupShape>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NonVisualGroupDrawingShapePropsExtensionChoice {
  #[sdk(child(
    office2013,
    qname = "a15:CT_NonVisualGroupProps/a15:nonVisualGroupProps"
  ))]
  A15NonVisualGroupProps(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_main::NonVisualGroupProperties,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OfficeStyleSheetExtensionChoice {
  #[sdk(child(office2013, qname = "thm15:CT_ThemeFamily/thm15:themeFamily"))]
  Thm15ThemeFamily(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_thememl_2012_main::ThemeFamily>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ConnectorLockingExtensionChoice {
  #[sdk(child(qname = "a:CT_GraphicalObject/a:graphic"))]
  AGraphic(std::boxed::Box<Graphic>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DataModelExtensionChoice {
  #[sdk(child(office2010, qname = "dsp:CT_DataModelExtBlock/dsp:dataModelExt"))]
  DspDataModelExt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::DataModelExtensionBlock,
    >,
  ),
  #[sdk(child(office2010, qname = "dgm14:CT_Boolean/dgm14:recolorImg"))]
  Dgm14RecolorImg(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::RecolorImages,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PtExtensionChoice {
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/dgm14:cNvPr"))]
  Dgm14CNvPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::NonVisualDrawingProperties,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum HyperlinkExtensionChoice {
  #[sdk(child(office2019, qname = "ahyp:CT_HyperlinkColor/ahyp:hlinkClr"))]
  AhypHlinkClr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2018_hyperlinkcolor::HyperlinkColor,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LinePropertiesExtensionChoice {
  #[sdk(
        child(
            office2021,
            qname = "ask:CT_LineSketchStyleProperties/ask:lineSketchStyleProps"
        )
    )]
    AskLineSketchStyleProps(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2018_sketchyshapes::LineSketchStyleProperties,
        >,
    ),
    #[sdk(any)]
    XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NonVisualDrawingPropertiesExtensionChoice {
  #[sdk(child(office2010, qname = "a14:CT_CompatExt/a14:compatExt"))]
    A14CompatExt(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CompatExtension,
        >,
    ),
    #[sdk(child(office2013, qname = "a15:CT_BackgroundPr/a15:backgroundPr"))]
    A15BackgroundPr(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2012_main::BackgroundProperties,
        >,
    ),
    #[sdk(child(office2016, qname = "a16:CT_CreationId/a16:creationId"))]
    A16CreationId(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2014_main::CreationId,
        >,
    ),
    #[sdk(
        child(
            office2016,
            qname = "a16:CT_PredecessorDrawingElementReference/a16:predDERef"
        )
    )]
    A16PredDeRef(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2014_main::PredecessorDrawingElementReference,
        >,
    ),
    #[sdk(child(office2019, qname = "adec:CT_Decorative/adec:decorative"))]
    AdecDecorative(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2017_decorative::Decorative,
        >,
    ),
    #[sdk(
        child(office2021, qname = "aclsh:CT_ClassificationOutcome/aclsh:classification")
    )]
    AclshClassification(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2020_classification_shape::ClassificationOutcome,
        >,
    ),
    #[sdk(child(microsoft365, qname = "asl:CT_ScriptLink/asl:scriptLink"))]
    AslScriptLink(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2021_scriptlink::ScriptLink,
        >,
    ),
    #[sdk(any)]
    XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NonVisualPicturePropertiesExtensionChoice {
  #[sdk(child(office2010, qname = "a14:CT_CameraTool/a14:cameraTool"))]
  A14CameraTool(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CameraTool>,
  ),
  #[sdk(child(office2013, qname = "a15:CT_SignatureLine/a15:signatureLine"))]
  A15SignatureLine(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_main::SignatureLine>,
  ),
  #[sdk(child(office2013, qname = "a15:CT_ObjectPr/a15:objectPr"))]
  A15ObjectPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_main::ObjectProperties,
    >,
  ),
  #[sdk(child(office2021, qname = "alf:CT_LiveFeedProperties/alf:liveFeedProps"))]
  AlfLiveFeedProps(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2021_livefeed::LiveFeedProperties,
    >,
  ),
  #[sdk(child(microsoft365, qname = "aif:CT_ImageFormula/aif:imageFormula"))]
  AifImageFormula(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2022_imageformula::ImageFormula,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BlipExtensionChoice {
  #[sdk(child(office2010, qname = "a14:CT_Photo/a14:imgProps"))]
    A14ImgProps(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ImageProperties,
        >,
    ),
    #[sdk(child(office2010, qname = "a14:CT_UseLocalDpi/a14:useLocalDpi"))]
    A14UseLocalDpi(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2010_main::UseLocalDpi,
        >,
    ),
    #[sdk(child(office2013, qname = "wp15:CT_WebVideoPr/wp15:webVideoPr"))]
    Wp15WebVideoPr(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2012_wordprocessing_drawing::WebVideoProperty,
        >,
    ),
    #[sdk(child(office2019, qname = "asvg:CT_SVGBlip/asvg:svgBlip"))]
    AsvgSvgBlip(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2016_svg_main::SvgBlip,
        >,
    ),
    #[sdk(
        child(
            office2019,
            qname = "a1611:CT_PictureAttributionSourceURL/a1611:picAttrSrcUrl"
        )
    )]
    A1611PicAttrSrcUrl(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2016_11_main::PictureAttributionSourceUrl,
        >,
    ),
    #[sdk(child(microsoft365, qname = "woe:CT_OEmbed/woe:oembed"))]
    WoeOembed(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_word_2020_oembed::OEmbed,
        >,
    ),
    #[sdk(child(microsoft365, qname = "aoe:CT_OEmbedShared/aoe:oembedShared"))]
    AoeOembedShared(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2021_oembed::OEmbedShared,
        >,
    ),
    #[sdk(any)]
    XmlOther(String),
}
