//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OnOffValues {
  #[sdk(rename = "true")]
  #[default]
  True,
  #[sdk(rename = "false")]
  False,
  #[sdk(rename = "0")]
  Zero,
  #[sdk(rename = "1")]
  One,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SchemeColorValues {
  #[sdk(rename = "bg1")]
  #[default]
  BackgroundColor,
  #[sdk(rename = "tx1")]
  TextColor,
  #[sdk(rename = "bg2")]
  AdditionalBackgroundColor,
  #[sdk(rename = "tx2")]
  AdditionalTextColor,
  #[sdk(rename = "accent1")]
  ExtraSchemeColor1,
  #[sdk(rename = "accent2")]
  ExtraSchemeColor2,
  #[sdk(rename = "accent3")]
  ExtraSchemeColor3,
  #[sdk(rename = "accent4")]
  ExtraSchemeColor4,
  #[sdk(rename = "accent5")]
  ExtraSchemeColor5,
  #[sdk(rename = "accent6")]
  ExtraSchemeColor6,
  #[sdk(rename = "hlink")]
  HyperlinkColor,
  #[sdk(rename = "folHlink")]
  FollowedHyperlinkColor,
  #[sdk(rename = "dk1")]
  MainDarkColor1,
  #[sdk(rename = "lt1")]
  MainLightColor1,
  #[sdk(rename = "dk2")]
  MainDarkColor2,
  #[sdk(rename = "lt2")]
  MainLightColor2,
  #[sdk(rename = "phClr")]
  AutoColor,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RectangleAlignmentValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "tl")]
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
pub enum PathShadeTypeValues {
  #[sdk(rename = "shape")]
  #[default]
  Shape,
  #[sdk(rename = "circle")]
  Circle,
  #[sdk(rename = "rect")]
  Rect,
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
pub enum PresetLineDashValues {
  #[sdk(rename = "solid")]
  #[default]
  Solid,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "sysDot")]
  SysDot,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "sysDash")]
  SysDash,
  #[sdk(rename = "lgDash")]
  LongDash,
  #[sdk(rename = "dashDot")]
  DashDot,
  #[sdk(rename = "sysDashDot")]
  SystemDashDot,
  #[sdk(rename = "lgDashDot")]
  LongDashDot,
  #[sdk(rename = "lgDashDotDot")]
  LongDashDotDot,
  #[sdk(rename = "sysDashDotDot")]
  SystemDashDotDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PenAlignmentValues {
  #[sdk(rename = "ctr")]
  #[default]
  Center,
  #[sdk(rename = "in")]
  Inset,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CompoundLineValues {
  #[sdk(rename = "sng")]
  #[default]
  Simple,
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
pub enum PresetCameraTypeValues {
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
pub enum LightRigTypeValues {
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
  ThreePoint,
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
  TwoPoint,
  #[sdk(rename = "glow")]
  Glow,
  #[sdk(rename = "brightRoom")]
  BrightRoom,
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
pub enum BevelPresetTypeValues {
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
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LigaturesValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "standard")]
  Standard,
  #[sdk(rename = "contextual")]
  Contextual,
  #[sdk(rename = "historical")]
  Historical,
  #[sdk(rename = "discretional")]
  Discretional,
  #[sdk(rename = "standardContextual")]
  StandardContextual,
  #[sdk(rename = "standardHistorical")]
  StandardHistorical,
  #[sdk(rename = "contextualHistorical")]
  ContextualHistorical,
  #[sdk(rename = "standardDiscretional")]
  StandardDiscretional,
  #[sdk(rename = "contextualDiscretional")]
  ContextualDiscretional,
  #[sdk(rename = "historicalDiscretional")]
  HistoricalDiscretional,
  #[sdk(rename = "standardContextualHistorical")]
  StandardContextualHistorical,
  #[sdk(rename = "standardContextualDiscretional")]
  StandardContextualDiscretional,
  #[sdk(rename = "standardHistoricalDiscretional")]
  StandardHistoricalDiscretional,
  #[sdk(rename = "contextualHistoricalDiscretional")]
  ContextualHistoricalDiscretional,
  #[sdk(rename = "all")]
  All,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NumberFormValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "lining")]
  Lining,
  #[sdk(rename = "oldStyle")]
  OldStyle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NumberSpacingValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "proportional")]
  Proportional,
  #[sdk(rename = "tabular")]
  Tabular,
}
/// Defines the RunConflictInsertion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns")]
pub struct RunConflictInsertion {
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
    qname = "w:CT_DirContentRun/w:dir"
  ))]
  pub run_conflict_insertion_choice: Vec<RunConflictInsertionChoice>,
}
/// Defines the RunConflictDeletion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel")]
pub struct RunConflictDeletion {
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
    qname = "w:CT_DirContentRun/w:dir"
  ))]
  pub run_conflict_deletion_choice: Vec<RunConflictDeletionChoice>,
}
/// Defines the ConflictInsertion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_TrackChange/w14:conflictIns")]
pub struct ConflictInsertion {
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
/// Defines the ConflictDeletion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_TrackChange/w14:conflictDel")]
pub struct ConflictDeletion {
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
/// Defines the CustomXmlConflictInsertionRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart"
)]
pub struct CustomXmlConflictInsertionRangeStart {
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
/// Defines the CustomXmlConflictDeletionRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart"
)]
pub struct CustomXmlConflictDeletionRangeStart {
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
/// Defines the Tint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:tint")]
pub struct Tint {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the Shade Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:shade")]
pub struct Shade {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the Alpha Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:alpha")]
pub struct Alpha {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the HueModulation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PositivePercentage/w14:hueMod")]
pub struct HueModulation {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the Saturation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:sat")]
pub struct Saturation {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SaturationOffset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:satOff")]
pub struct SaturationOffset {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SaturationModulation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:satMod")]
pub struct SaturationModulation {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the Luminance Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:lum")]
pub struct Luminance {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the LuminanceOffset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:lumOff")]
pub struct LuminanceOffset {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the LuminanceModulation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Percentage/w14:lumMod")]
pub struct LuminanceModulation {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the RgbColorModelHex Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr")]
pub struct RgbColorModelHex {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 3u32, max = 3u32))]
  pub val: crate::simple_type::HexBinaryValue,
  #[sdk(choice(
    qname = "w14:CT_PositiveFixedPercentage/w14:tint",
    qname = "w14:CT_PositiveFixedPercentage/w14:shade",
    qname = "w14:CT_PositiveFixedPercentage/w14:alpha",
    qname = "w14:CT_PositivePercentage/w14:hueMod",
    qname = "w14:CT_Percentage/w14:sat",
    qname = "w14:CT_Percentage/w14:satOff",
    qname = "w14:CT_Percentage/w14:satMod",
    qname = "w14:CT_Percentage/w14:lum",
    qname = "w14:CT_Percentage/w14:lumOff",
    qname = "w14:CT_Percentage/w14:lumMod"
  ))]
  pub rgb_color_model_hex_choice: Vec<RgbColorModelHexChoice>,
}
/// Defines the SchemeColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr")]
pub struct SchemeColor {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: SchemeColorValues,
  #[sdk(choice(
    qname = "w14:CT_PositiveFixedPercentage/w14:tint",
    qname = "w14:CT_PositiveFixedPercentage/w14:shade",
    qname = "w14:CT_PositiveFixedPercentage/w14:alpha",
    qname = "w14:CT_PositivePercentage/w14:hueMod",
    qname = "w14:CT_Percentage/w14:sat",
    qname = "w14:CT_Percentage/w14:satOff",
    qname = "w14:CT_Percentage/w14:satMod",
    qname = "w14:CT_Percentage/w14:lum",
    qname = "w14:CT_Percentage/w14:lumOff",
    qname = "w14:CT_Percentage/w14:lumMod"
  ))]
  pub scheme_color_choice: Vec<SchemeColorChoice>,
}
/// Defines the LinearShadeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_LinearShadeProperties/w14:lin")]
pub struct LinearShadeProperties {
  /// ang
  #[sdk(attr(office2010, qname = "w14:ang"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub angle: Option<crate::simple_type::Int32Value>,
  /// scaled
  #[sdk(attr(office2010, qname = "w14:scaled"))]
  pub scaled: Option<OnOffValues>,
}
/// Defines the PathShadeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PathShadeProperties/w14:path")]
pub struct PathShadeProperties {
  /// path
  #[sdk(attr(office2010, qname = "w14:path"))]
  pub path: Option<PathShadeTypeValues>,
  /// Defines the FillToRectangle Class.
  #[sdk(child(office2010, qname = "w14:CT_RelativeRect/w14:fillToRect"))]
  pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Defines the SolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SolidColorFillProperties/w14:solidFill")]
pub struct SolidColorFillProperties {
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub solid_color_fill_properties_choice: Option<SolidColorFillPropertiesChoice>,
}
/// Defines the GradientFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_GradientFillProperties/w14:gradFill")]
pub struct GradientFillProperties {
  /// Defines the GradientStopList Class.
  #[sdk(child(office2010, qname = "w14:CT_GradientStopList/w14:gsLst"))]
  pub gradient_stop_list: Option<GradientStopList>,
  #[sdk(choice(
    qname = "w14:CT_LinearShadeProperties/w14:lin",
    qname = "w14:CT_PathShadeProperties/w14:path"
  ))]
  pub gradient_fill_properties_choice: Option<GradientFillPropertiesChoice>,
}
/// Defines the PresetLineDashProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_PresetLineDashProperties/w14:prstDash")]
pub struct PresetLineDashProperties {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<PresetLineDashValues>,
}
/// Defines the LineJoinMiterProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_LineJoinMiterProperties/w14:miter")]
pub struct LineJoinMiterProperties {
  /// lim
  #[sdk(attr(office2010, qname = "w14:lim"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub limit: Option<crate::simple_type::Int32Value>,
}
/// Defines the Glow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Glow/w14:glow")]
pub struct Glow {
  /// rad
  #[sdk(attr(office2010, qname = "w14:rad"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub glow_radius: Option<crate::simple_type::Int64Value>,
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub glow_choice: Option<GlowChoice>,
}
/// Defines the Shadow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Shadow/w14:shadow")]
pub struct Shadow {
  /// blurRad
  #[sdk(attr(office2010, qname = "w14:blurRad"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub blur_radius: Option<crate::simple_type::Int64Value>,
  /// dist
  #[sdk(attr(office2010, qname = "w14:dist"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub distance_from_text: Option<crate::simple_type::Int64Value>,
  /// dir
  #[sdk(attr(office2010, qname = "w14:dir"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub direction_angle: Option<crate::simple_type::Int32Value>,
  /// sx
  #[sdk(attr(office2010, qname = "w14:sx"))]
  pub horizontal_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// sy
  #[sdk(attr(office2010, qname = "w14:sy"))]
  pub vertical_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// kx
  #[sdk(attr(office2010, qname = "w14:kx"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  pub horizontal_skew_angle: Option<crate::simple_type::Int32Value>,
  /// ky
  #[sdk(attr(office2010, qname = "w14:ky"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  pub vertical_skew_angle: Option<crate::simple_type::Int32Value>,
  /// algn
  #[sdk(attr(office2010, qname = "w14:algn"))]
  pub alignment: Option<RectangleAlignmentValues>,
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub shadow_choice: Option<ShadowChoice>,
}
/// Defines the Reflection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Reflection/w14:reflection")]
pub struct Reflection {
  /// blurRad
  #[sdk(attr(office2010, qname = "w14:blurRad"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub blur_radius: Option<crate::simple_type::Int64Value>,
  /// stA
  #[sdk(attr(office2010, qname = "w14:stA"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub starting_opacity: Option<crate::simple_type::Int32Value>,
  /// stPos
  #[sdk(attr(office2010, qname = "w14:stPos"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub start_position: Option<crate::simple_type::Int32Value>,
  /// endA
  #[sdk(attr(office2010, qname = "w14:endA"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub ending_opacity: Option<crate::simple_type::Int32Value>,
  /// endPos
  #[sdk(attr(office2010, qname = "w14:endPos"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub end_position: Option<crate::simple_type::Int32Value>,
  /// dist
  #[sdk(attr(office2010, qname = "w14:dist"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub distance_from_text: Option<crate::simple_type::Int64Value>,
  /// dir
  #[sdk(attr(office2010, qname = "w14:dir"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub direction_angle: Option<crate::simple_type::Int32Value>,
  /// fadeDir
  #[sdk(attr(office2010, qname = "w14:fadeDir"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub fade_direction: Option<crate::simple_type::Int32Value>,
  /// sx
  #[sdk(attr(office2010, qname = "w14:sx"))]
  pub horizontal_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// sy
  #[sdk(attr(office2010, qname = "w14:sy"))]
  pub vertical_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// kx
  #[sdk(attr(office2010, qname = "w14:kx"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  pub horizontal_skew_angle: Option<crate::simple_type::Int32Value>,
  /// ky
  #[sdk(attr(office2010, qname = "w14:ky"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "-5400000",
    max = "5400000",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  pub vertical_skew_angle: Option<crate::simple_type::Int32Value>,
  /// algn
  #[sdk(attr(office2010, qname = "w14:algn"))]
  pub alignment: Option<RectangleAlignmentValues>,
}
/// Defines the TextOutlineEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_TextOutlineEffect/w14:textOutline")]
pub struct TextOutlineEffect {
  /// w
  #[sdk(attr(office2010, qname = "w14:w"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "20116800",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub line_width: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(office2010, qname = "w14:cap"))]
  pub cap_type: Option<LineCapValues>,
  /// cmpd
  #[sdk(attr(office2010, qname = "w14:cmpd"))]
  pub compound: Option<CompoundLineValues>,
  /// algn
  #[sdk(attr(office2010, qname = "w14:algn"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(choice(
    qname = "w:CT_Empty/w14:noFill",
    qname = "w14:CT_SolidColorFillProperties/w14:solidFill",
    qname = "w14:CT_GradientFillProperties/w14:gradFill"
  ))]
  pub text_outline_effect_choice1: Option<TextOutlineEffectChoice>,
  /// Defines the PresetLineDashProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_PresetLineDashProperties/w14:prstDash"))]
  pub w14_prst_dash: Option<PresetLineDashProperties>,
  #[sdk(choice(
    qname = "w:CT_Empty/w14:round",
    qname = "w:CT_Empty/w14:bevel",
    qname = "w14:CT_LineJoinMiterProperties/w14:miter"
  ))]
  pub text_outline_effect_choice3: Option<TextOutlineEffectChoice2>,
}
/// Defines the FillTextEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_FillTextEffect/w14:textFill")]
pub struct FillTextEffect {
  #[sdk(choice(
    qname = "w:CT_Empty/w14:noFill",
    qname = "w14:CT_SolidColorFillProperties/w14:solidFill",
    qname = "w14:CT_GradientFillProperties/w14:gradFill"
  ))]
  pub fill_text_effect_choice: Option<FillTextEffectChoice>,
}
/// Defines the Scene3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Scene3D/w14:scene3d")]
pub struct Scene3D {
  /// Defines the Camera Class.
  #[sdk(child(office2010, qname = "w14:CT_Camera/w14:camera"))]
  pub camera: std::boxed::Box<Camera>,
  /// Defines the LightRig Class.
  #[sdk(child(office2010, qname = "w14:CT_LightRig/w14:lightRig"))]
  pub light_rig: std::boxed::Box<LightRig>,
}
/// Defines the Properties3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Props3D/w14:props3d")]
pub struct Properties3D {
  /// extrusionH
  #[sdk(attr(office2010, qname = "w14:extrusionH"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub extrusion_height: Option<crate::simple_type::Int64Value>,
  /// contourW
  #[sdk(attr(office2010, qname = "w14:contourW"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub contour_width: Option<crate::simple_type::Int64Value>,
  /// prstMaterial
  #[sdk(attr(office2010, qname = "w14:prstMaterial"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_material_type: Option<PresetMaterialTypeValues>,
  /// Defines the BevelTop Class.
  #[sdk(child(office2010, qname = "w14:CT_Bevel/w14:bevelT"))]
  pub bevel_top: Option<BevelTop>,
  /// Defines the BevelBottom Class.
  #[sdk(child(office2010, qname = "w14:CT_Bevel/w14:bevelB"))]
  pub bevel_bottom: Option<BevelBottom>,
  /// Defines the ExtrusionColor Class.
  #[sdk(child(office2010, qname = "w14:CT_Color/w14:extrusionClr"))]
  pub extrusion_color: Option<std::boxed::Box<ExtrusionColor>>,
  /// Defines the ContourColor Class.
  #[sdk(child(office2010, qname = "w14:CT_Color/w14:contourClr"))]
  pub contour_color: Option<std::boxed::Box<ContourColor>>,
}
/// Defines the Ligatures Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Ligatures/w14:ligatures")]
pub struct Ligatures {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: LigaturesValues,
}
/// Defines the NumberingFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_NumForm/w14:numForm")]
pub struct NumberingFormat {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: NumberFormValues,
}
/// Defines the NumberSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_NumSpacing/w14:numSpacing")]
pub struct NumberSpacing {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: NumberSpacingValues,
}
/// Defines the StylisticSets Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_StylisticSets/w14:stylisticSets")]
pub struct StylisticSets {
  /// Defines the StyleSet Class.
  #[sdk(child(office2010, qname = "w14:CT_StyleSet/w14:styleSet"))]
  pub w14_style_set: Vec<StyleSet>,
}
/// Defines the ContextualAlternatives Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_OnOff/w14:cntxtAlts")]
pub struct ContextualAlternatives {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the ConflictMode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_OnOff/w14:conflictMode")]
pub struct ConflictMode {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the DiscardImageEditingData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_OnOff/w14:discardImageEditingData")]
pub struct DiscardImageEditingData {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the Checked Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_OnOff/w14:checked")]
pub struct Checked {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_WordContentPart/w14:contentPart")]
pub struct ContentPart {
  /// bwMode
  #[sdk(attr(office2010, qname = "w14:bwMode"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(office2010, qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the WordNonVisualContentPartShapeProperties Class.
  #[sdk(child(
    office2010,
    qname = "w14:CT_WordContentPartNonVisual/w14:nvContentPartPr"
  ))]
  pub word_non_visual_content_part_shape_properties:
    Option<std::boxed::Box<WordNonVisualContentPartShapeProperties>>,
  /// Defines the Transform2D Class.
  #[sdk(child(office2010, qname = "a:CT_Transform2D/w14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/w14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DocumentId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_LongHexNumber/w14:docId")]
pub struct DocumentId {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the CustomXmlConflictInsertionRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd")]
pub struct CustomXmlConflictInsertionRangeEnd {
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
/// Defines the CustomXmlConflictDeletionRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd")]
pub struct CustomXmlConflictDeletionRangeEnd {
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
/// Defines the DefaultImageDpi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_DefaultImageDpi/w14:defaultImageDpi")]
pub struct DefaultImageDpi {
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SdtContentCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SdtCheckbox/w14:checkbox")]
pub struct SdtContentCheckBox {
  /// Defines the Checked Class.
  #[sdk(child(office2010, qname = "w14:CT_OnOff/w14:checked"))]
  pub checked: Option<Checked>,
  /// Defines the CheckedState Class.
  #[sdk(child(office2010, qname = "w14:CT_SdtCheckboxSymbol/w14:checkedState"))]
  pub checked_state: Option<CheckedState>,
  /// Defines the UncheckedState Class.
  #[sdk(child(office2010, qname = "w14:CT_SdtCheckboxSymbol/w14:uncheckedState"))]
  pub unchecked_state: Option<UncheckedState>,
}
/// Defines the GradientStop Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_GradientStop/w14:gs")]
pub struct GradientStop {
  /// pos
  #[sdk(attr(office2010, qname = "w14:pos"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub stop_position: crate::simple_type::Int32Value,
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub gradient_stop_choice: Option<GradientStopChoice>,
}
/// Defines the FillToRectangle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_RelativeRect/w14:fillToRect")]
pub struct FillToRectangle {
  /// l
  #[sdk(attr(office2010, qname = "w14:l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(office2010, qname = "w14:t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(office2010, qname = "w14:r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(office2010, qname = "w14:b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Defines the GradientStopList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_GradientStopList/w14:gsLst")]
pub struct GradientStopList {
  /// Defines the GradientStop Class.
  #[sdk(child(office2010, qname = "w14:CT_GradientStop/w14:gs"))]
  pub w14_gs: Vec<GradientStop>,
}
/// Defines the SphereCoordinates Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SphereCoords/w14:rot")]
pub struct SphereCoordinates {
  /// lat
  #[sdk(attr(office2010, qname = "w14:lat"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub lattitude: crate::simple_type::Int32Value,
  /// lon
  #[sdk(attr(office2010, qname = "w14:lon"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub longitude: crate::simple_type::Int32Value,
  /// rev
  #[sdk(attr(office2010, qname = "w14:rev"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "21600000",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  pub revolution: crate::simple_type::Int32Value,
}
/// Defines the Camera Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Camera/w14:camera")]
pub struct Camera {
  /// prst
  #[sdk(attr(office2010, qname = "w14:prst"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub preset_camera_type: PresetCameraTypeValues,
}
/// Defines the LightRig Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_LightRig/w14:lightRig")]
pub struct LightRig {
  /// rig
  #[sdk(attr(office2010, qname = "w14:rig"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub light_rig_type: LightRigTypeValues,
  /// dir
  #[sdk(attr(office2010, qname = "w14:dir"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub light_direction_type: LightRigDirectionValues,
  /// Defines the SphereCoordinates Class.
  #[sdk(child(office2010, qname = "w14:CT_SphereCoords/w14:rot"))]
  pub sphere_coordinates: Option<SphereCoordinates>,
}
/// Defines the BevelTop Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Bevel/w14:bevelT")]
pub struct BevelTop {
  /// w
  #[sdk(attr(office2010, qname = "w14:w"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// h
  #[sdk(attr(office2010, qname = "w14:h"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// prst
  #[sdk(attr(office2010, qname = "w14:prst"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the BevelBottom Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Bevel/w14:bevelB")]
pub struct BevelBottom {
  /// w
  #[sdk(attr(office2010, qname = "w14:w"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub width: Option<crate::simple_type::Int64Value>,
  /// h
  #[sdk(attr(office2010, qname = "w14:h"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub height: Option<crate::simple_type::Int64Value>,
  /// prst
  #[sdk(attr(office2010, qname = "w14:prst"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the ExtrusionColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Color/w14:extrusionClr")]
pub struct ExtrusionColor {
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub extrusion_color_choice: Option<ExtrusionColorChoice>,
}
/// Defines the ContourColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_Color/w14:contourClr")]
pub struct ContourColor {
  #[sdk(choice(
    qname = "w14:CT_SRgbColor/w14:srgbClr",
    qname = "w14:CT_SchemeColor/w14:schemeClr"
  ))]
  pub contour_color_choice: Option<ContourColorChoice>,
}
/// Defines the StyleSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_StyleSet/w14:styleSet")]
pub struct StyleSet {
  /// id
  #[sdk(attr(office2010, qname = "w14:id"))]
  pub id: crate::simple_type::UInt32Value,
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the CheckedState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SdtCheckboxSymbol/w14:checkedState")]
pub struct CheckedState {
  /// font
  #[sdk(attr(office2010, qname = "w14:font"))]
  pub font: Option<crate::simple_type::StringValue>,
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 2u32, max = 2u32))]
  pub val: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the UncheckedState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "w14:CT_SdtCheckboxSymbol/w14:uncheckedState")]
pub struct UncheckedState {
  /// font
  #[sdk(attr(office2010, qname = "w14:font"))]
  pub font: Option<crate::simple_type::StringValue>,
  /// val
  #[sdk(attr(office2010, qname = "w14:val"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 2u32, max = 2u32))]
  pub val: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/w14:cNvPr")]
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
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    /// Hyperlink associated with hovering over the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    /// Future extension
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_NonVisualInkContentPartProperties/w14:cNvContentPartPr"
)]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(office2010, qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// Defines the ContentPartLocks Class.
  #[sdk(child(office2010, qname = "a14:CT_ContentPartLocking/a14:cpLocks"))]
  pub content_part_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ContentPartLocks,
    >,
  >,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list:
    Option<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::OfficeArtExtensionList>,
}
/// Defines the WordNonVisualContentPartShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "w14:CT_WordContentPartNonVisual/w14:nvContentPartPr"
)]
pub struct WordNonVisualContentPartShapeProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/w14:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// Defines the NonVisualInkContentPartProperties Class.
  #[sdk(child(
    office2010,
    qname = "a14:CT_NonVisualInkContentPartProperties/w14:cNvContentPartPr"
  ))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/w14:xfrm")]
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
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_OfficeArtExtensionList/w14:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunConflictInsertionChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun,
        >,
    ),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<CustomXmlConflictInsertionRangeStart>,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(std::boxed::Box<CustomXmlConflictInsertionRangeEnd>),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<CustomXmlConflictDeletionRangeStart>,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(std::boxed::Box<CustomXmlConflictDeletionRangeEnd>),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<std::boxed::Box<RunConflictInsertion>>,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<std::boxed::Box<RunConflictDeletion>>,
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
    WR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run,
        >,
    ),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalOverride,
        >,
    ),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalEmbedding,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunConflictDeletionChoice {
  /// Defines the SdtRun Class.
    #[sdk(child(qname = "w:CT_SdtRun/w:sdt"))]
    WSdt(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun,
        >,
    ),
    /// Defines the ProofError Class.
    #[sdk(child(qname = "w:CT_ProofErr/w:proofErr"))]
    WProofErr(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
        >,
    ),
    /// Defines the PermStart Class.
    #[sdk(child(qname = "w:CT_PermStart/w:permStart"))]
    WPermStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
        >,
    ),
    /// Defines the PermEnd Class.
    #[sdk(child(qname = "w:CT_Perm/w:permEnd"))]
    WPermEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
        >,
    ),
    /// Defines the BookmarkStart Class.
    #[sdk(child(qname = "w:CT_Bookmark/w:bookmarkStart"))]
    WBookmarkStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
        >,
    ),
    /// Defines the BookmarkEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:bookmarkEnd"))]
    WBookmarkEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
        >,
    ),
    /// Defines the CommentRangeStart Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeStart"))]
    WCommentRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
        >,
    ),
    /// Defines the CommentRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:commentRangeEnd"))]
    WCommentRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
        >,
    ),
    /// Defines the MoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveFromRangeStart"))]
    WMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
        >,
    ),
    /// Defines the MoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveFromRangeEnd"))]
    WMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
        >,
    ),
    /// Defines the MoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_MoveBookmark/w:moveToRangeStart"))]
    WMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
        >,
    ),
    /// Defines the MoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_MarkupRange/w:moveToRangeEnd"))]
    WMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
        >,
    ),
    /// Defines the CustomXmlInsRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlInsRangeStart"))]
    WCustomXmlInsRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
        >,
    ),
    /// Defines the CustomXmlInsRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlInsRangeEnd"))]
    WCustomXmlInsRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
        >,
    ),
    /// Defines the CustomXmlDelRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlDelRangeStart"))]
    WCustomXmlDelRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
        >,
    ),
    /// Defines the CustomXmlDelRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlDelRangeEnd"))]
    WCustomXmlDelRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
        >,
    ),
    /// Defines the CustomXmlMoveFromRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveFromRangeStart"))]
    WCustomXmlMoveFromRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
        >,
    ),
    /// Defines the CustomXmlMoveFromRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveFromRangeEnd"))]
    WCustomXmlMoveFromRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
        >,
    ),
    /// Defines the CustomXmlMoveToRangeStart Class.
    #[sdk(child(qname = "w:CT_TrackChange/w:customXmlMoveToRangeStart"))]
    WCustomXmlMoveToRangeStart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
        >,
    ),
    /// Defines the CustomXmlMoveToRangeEnd Class.
    #[sdk(child(qname = "w:CT_Markup/w:customXmlMoveToRangeEnd"))]
    WCustomXmlMoveToRangeEnd(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
        >,
    ),
    /// Defines the CustomXmlConflictInsertionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictInsRangeStart")
    )]
    W14CustomXmlConflictInsRangeStart(
        std::boxed::Box<CustomXmlConflictInsertionRangeStart>,
    ),
    /// Defines the CustomXmlConflictInsertionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictInsRangeEnd"))]
    W14CustomXmlConflictInsRangeEnd(std::boxed::Box<CustomXmlConflictInsertionRangeEnd>),
    /// Defines the CustomXmlConflictDeletionRangeStart Class.
    #[sdk(
        child(office2010, qname = "w:CT_TrackChange/w14:customXmlConflictDelRangeStart")
    )]
    W14CustomXmlConflictDelRangeStart(
        std::boxed::Box<CustomXmlConflictDeletionRangeStart>,
    ),
    /// Defines the CustomXmlConflictDeletionRangeEnd Class.
    #[sdk(child(office2010, qname = "w:CT_Markup/w14:customXmlConflictDelRangeEnd"))]
    W14CustomXmlConflictDelRangeEnd(std::boxed::Box<CustomXmlConflictDeletionRangeEnd>),
    /// Inserted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:ins"))]
    WIns(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
        >,
    ),
    /// Deleted Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:del"))]
    WDel(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
        >,
    ),
    /// Move Source Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveFrom"))]
    WMoveFrom(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
        >,
    ),
    /// Move Destination Run Content.
    #[sdk(child(qname = "w:CT_RunTrackChange/w:moveTo"))]
    WMoveTo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
        >,
    ),
    /// Defines the ContentPart Class.
    #[sdk(child(office2010, qname = "w:CT_ContentPart/w:contentPart"))]
    WContentPart(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
        >,
    ),
    /// Sequence of w14:conflictIns, w14:conflictDel
    #[sdk(sequence)]
    Sequence {
        /// Defines the RunConflictInsertion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictIns"))]
        run_conflict_insertion: Option<std::boxed::Box<RunConflictInsertion>>,
        /// Defines the RunConflictDeletion Class.
        #[sdk(child(office2010, qname = "w:CT_RunTrackChange/w14:conflictDel"))]
        run_conflict_deletion: Option<std::boxed::Box<RunConflictDeletion>>,
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
    WR(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run,
        >,
    ),
    /// Defines the BidirectionalOverride Class.
    #[sdk(child(office2010, qname = "w:CT_BdoContentRun/w:bdo"))]
    WBdo(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalOverride,
        >,
    ),
    /// Defines the BidirectionalEmbedding Class.
    #[sdk(child(office2010, qname = "w:CT_DirContentRun/w:dir"))]
    WDir(
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalEmbedding,
        >,
    ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RgbColorModelHexChoice {
  /// Defines the Tint Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:tint"))]
  W14Tint(std::boxed::Box<Tint>),
  /// Defines the Shade Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:shade"))]
  W14Shade(std::boxed::Box<Shade>),
  /// Defines the Alpha Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:alpha"))]
  W14Alpha(std::boxed::Box<Alpha>),
  /// Defines the HueModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_PositivePercentage/w14:hueMod"))]
  W14HueMod(std::boxed::Box<HueModulation>),
  /// Defines the Saturation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:sat"))]
  W14Sat(std::boxed::Box<Saturation>),
  /// Defines the SaturationOffset Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:satOff"))]
  W14SatOff(std::boxed::Box<SaturationOffset>),
  /// Defines the SaturationModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:satMod"))]
  W14SatMod(std::boxed::Box<SaturationModulation>),
  /// Defines the Luminance Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lum"))]
  W14Lum(std::boxed::Box<Luminance>),
  /// Defines the LuminanceOffset Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lumOff"))]
  W14LumOff(std::boxed::Box<LuminanceOffset>),
  /// Defines the LuminanceModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lumMod"))]
  W14LumMod(std::boxed::Box<LuminanceModulation>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SchemeColorChoice {
  /// Defines the Tint Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:tint"))]
  W14Tint(std::boxed::Box<Tint>),
  /// Defines the Shade Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:shade"))]
  W14Shade(std::boxed::Box<Shade>),
  /// Defines the Alpha Class.
  #[sdk(child(office2010, qname = "w14:CT_PositiveFixedPercentage/w14:alpha"))]
  W14Alpha(std::boxed::Box<Alpha>),
  /// Defines the HueModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_PositivePercentage/w14:hueMod"))]
  W14HueMod(std::boxed::Box<HueModulation>),
  /// Defines the Saturation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:sat"))]
  W14Sat(std::boxed::Box<Saturation>),
  /// Defines the SaturationOffset Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:satOff"))]
  W14SatOff(std::boxed::Box<SaturationOffset>),
  /// Defines the SaturationModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:satMod"))]
  W14SatMod(std::boxed::Box<SaturationModulation>),
  /// Defines the Luminance Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lum"))]
  W14Lum(std::boxed::Box<Luminance>),
  /// Defines the LuminanceOffset Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lumOff"))]
  W14LumOff(std::boxed::Box<LuminanceOffset>),
  /// Defines the LuminanceModulation Class.
  #[sdk(child(office2010, qname = "w14:CT_Percentage/w14:lumMod"))]
  W14LumMod(std::boxed::Box<LuminanceModulation>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SolidColorFillPropertiesChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientFillPropertiesChoice {
  #[sdk(child(office2010, qname = "w14:CT_LinearShadeProperties/w14:lin"))]
  W14Lin(std::boxed::Box<LinearShadeProperties>),
  #[sdk(child(office2010, qname = "w14:CT_PathShadeProperties/w14:path"))]
  W14Path(std::boxed::Box<PathShadeProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GlowChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShadowChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextOutlineEffectChoice {
  /// Defines the NoFillEmpty Class.
  #[sdk(empty_child(office2010, qname = "w:CT_Empty/w14:noFill"))]
  W14NoFill,
  /// Defines the SolidColorFillProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_SolidColorFillProperties/w14:solidFill"))]
  W14SolidFill(std::boxed::Box<SolidColorFillProperties>),
  /// Defines the GradientFillProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_GradientFillProperties/w14:gradFill"))]
  W14GradFill(std::boxed::Box<GradientFillProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextOutlineEffectChoice2 {
  /// Defines the RoundEmpty Class.
  #[sdk(empty_child(office2010, qname = "w:CT_Empty/w14:round"))]
  W14Round,
  /// Defines the BevelEmpty Class.
  #[sdk(empty_child(office2010, qname = "w:CT_Empty/w14:bevel"))]
  W14Bevel,
  /// Defines the LineJoinMiterProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_LineJoinMiterProperties/w14:miter"))]
  W14Miter(std::boxed::Box<LineJoinMiterProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillTextEffectChoice {
  /// Defines the NoFillEmpty Class.
  #[sdk(empty_child(office2010, qname = "w:CT_Empty/w14:noFill"))]
  W14NoFill,
  /// Defines the SolidColorFillProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_SolidColorFillProperties/w14:solidFill"))]
  W14SolidFill(std::boxed::Box<SolidColorFillProperties>),
  /// Defines the GradientFillProperties Class.
  #[sdk(child(office2010, qname = "w14:CT_GradientFillProperties/w14:gradFill"))]
  W14GradFill(std::boxed::Box<GradientFillProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientStopChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ExtrusionColorChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ContourColorChoice {
  /// Defines the RgbColorModelHex Class.
  #[sdk(child(office2010, qname = "w14:CT_SRgbColor/w14:srgbClr"))]
  W14SrgbClr(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  #[sdk(child(office2010, qname = "w14:CT_SchemeColor/w14:schemeClr"))]
  W14SchemeClr(std::boxed::Box<SchemeColor>),
}
