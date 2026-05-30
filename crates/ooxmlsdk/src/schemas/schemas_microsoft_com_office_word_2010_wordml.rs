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
#[sdk(qname = "w14:conflictIns")]
pub struct RunConflictInsertion {
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
  pub run_conflict_insertion_choice: Vec<RunConflictInsertionChoice>,
}
/// Defines the RunConflictDeletion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:conflictDel")]
pub struct RunConflictDeletion {
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
  pub run_conflict_deletion_choice: Vec<RunConflictDeletionChoice>,
}
/// Defines the ConflictInsertion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:conflictIns")]
pub struct ConflictInsertion {
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
/// Defines the ConflictDeletion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:conflictDel")]
pub struct ConflictDeletion {
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
/// Defines the CustomXmlConflictInsertionRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:customXmlConflictInsRangeStart")]
pub struct CustomXmlConflictInsertionRangeStart {
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
/// Defines the CustomXmlConflictDeletionRangeStart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:customXmlConflictDelRangeStart")]
pub struct CustomXmlConflictDeletionRangeStart {
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
/// Defines the Tint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:tint")]
pub struct Tint {
  /// val
  #[sdk(attr(qname = "w14:val"))]
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
#[sdk(qname = "w14:shade")]
pub struct Shade {
  /// val
  #[sdk(attr(qname = "w14:val"))]
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
#[sdk(qname = "w14:alpha")]
pub struct Alpha {
  /// val
  #[sdk(attr(qname = "w14:val"))]
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
#[sdk(qname = "w14:hueMod")]
pub struct HueModulation {
  /// val
  #[sdk(attr(qname = "w14:val"))]
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
#[sdk(qname = "w14:sat")]
pub struct Saturation {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SaturationOffset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:satOff")]
pub struct SaturationOffset {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SaturationModulation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:satMod")]
pub struct SaturationModulation {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the Luminance Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:lum")]
pub struct Luminance {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the LuminanceOffset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:lumOff")]
pub struct LuminanceOffset {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the LuminanceModulation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:lumMod")]
pub struct LuminanceModulation {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the RgbColorModelHex Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:srgbClr")]
pub struct RgbColorModelHex {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 3u32, max = 3u32))]
  pub val: crate::simple_type::HexBinaryValue,
  #[sdk(
        choice(
            child(variant = Tint, qname = "w14:tint"),
            child(variant = Shade, qname = "w14:shade"),
            child(variant = Alpha, qname = "w14:alpha"),
            child(variant = HueModulation, qname = "w14:hueMod"),
            child(variant = Saturation, qname = "w14:sat"),
            child(variant = SaturationOffset, qname = "w14:satOff"),
            child(variant = SaturationModulation, qname = "w14:satMod"),
            child(variant = Luminance, qname = "w14:lum"),
            child(variant = LuminanceOffset, qname = "w14:lumOff"),
            child(variant = LuminanceModulation, qname = "w14:lumMod")
        )
    )]
  pub rgb_color_model_hex_choice: Vec<RgbColorModelHexChoice>,
}
/// Defines the SchemeColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:schemeClr")]
pub struct SchemeColor {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: SchemeColorValues,
  #[sdk(
        choice(
            child(variant = Tint, qname = "w14:tint"),
            child(variant = Shade, qname = "w14:shade"),
            child(variant = Alpha, qname = "w14:alpha"),
            child(variant = HueModulation, qname = "w14:hueMod"),
            child(variant = Saturation, qname = "w14:sat"),
            child(variant = SaturationOffset, qname = "w14:satOff"),
            child(variant = SaturationModulation, qname = "w14:satMod"),
            child(variant = Luminance, qname = "w14:lum"),
            child(variant = LuminanceOffset, qname = "w14:lumOff"),
            child(variant = LuminanceModulation, qname = "w14:lumMod")
        )
    )]
  pub scheme_color_choice: Vec<SchemeColorChoice>,
}
/// Defines the LinearShadeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:lin")]
pub struct LinearShadeProperties {
  /// ang
  #[sdk(attr(qname = "w14:ang"))]
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
  #[sdk(attr(qname = "w14:scaled"))]
  pub scaled: Option<OnOffValues>,
}
/// Defines the PathShadeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:path")]
pub struct PathShadeProperties {
  /// path
  #[sdk(attr(qname = "w14:path"))]
  pub path: Option<PathShadeTypeValues>,
  /// Defines the FillToRectangle Class.
  #[sdk(child(qname = "w14:fillToRect"))]
  pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Defines the SolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:solidFill")]
pub struct SolidColorFillProperties {
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub solid_color_fill_properties_choice: Option<SolidColorFillPropertiesChoice>,
}
/// Defines the GradientFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:gradFill")]
pub struct GradientFillProperties {
  /// Defines the GradientStopList Class.
  #[sdk(child(qname = "w14:gsLst"))]
  pub gradient_stop_list: Option<GradientStopList>,
  #[sdk(
        choice(
            child(variant = LinearShadeProperties, qname = "w14:lin"),
            child(variant = PathShadeProperties, qname = "w14:path")
        )
    )]
  pub gradient_fill_properties_choice: Option<GradientFillPropertiesChoice>,
}
/// Defines the PresetLineDashProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:prstDash")]
pub struct PresetLineDashProperties {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<PresetLineDashValues>,
}
/// Defines the LineJoinMiterProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:miter")]
pub struct LineJoinMiterProperties {
  /// lim
  #[sdk(attr(qname = "w14:lim"))]
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
#[sdk(qname = "w14:glow")]
pub struct Glow {
  /// rad
  #[sdk(attr(qname = "w14:rad"))]
  #[sdk(number_range(
    source = 1u32,
    union = 0u64,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub glow_radius: Option<crate::simple_type::Int64Value>,
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub glow_choice: Option<GlowChoice>,
}
/// Defines the Shadow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:shadow")]
pub struct Shadow {
  /// blurRad
  #[sdk(attr(qname = "w14:blurRad"))]
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
  #[sdk(attr(qname = "w14:dist"))]
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
  #[sdk(attr(qname = "w14:dir"))]
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
  #[sdk(attr(qname = "w14:sx"))]
  pub horizontal_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// sy
  #[sdk(attr(qname = "w14:sy"))]
  pub vertical_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// kx
  #[sdk(attr(qname = "w14:kx"))]
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
  #[sdk(attr(qname = "w14:ky"))]
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
  #[sdk(attr(qname = "w14:algn"))]
  pub alignment: Option<RectangleAlignmentValues>,
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub shadow_choice: Option<ShadowChoice>,
}
/// Defines the Reflection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:reflection")]
pub struct Reflection {
  /// blurRad
  #[sdk(attr(qname = "w14:blurRad"))]
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
  #[sdk(attr(qname = "w14:stA"))]
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
  #[sdk(attr(qname = "w14:stPos"))]
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
  #[sdk(attr(qname = "w14:endA"))]
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
  #[sdk(attr(qname = "w14:endPos"))]
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
  #[sdk(attr(qname = "w14:dist"))]
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
  #[sdk(attr(qname = "w14:dir"))]
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
  #[sdk(attr(qname = "w14:fadeDir"))]
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
  #[sdk(attr(qname = "w14:sx"))]
  pub horizontal_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// sy
  #[sdk(attr(qname = "w14:sy"))]
  pub vertical_scaling_factor: Option<crate::simple_type::Int32Value>,
  /// kx
  #[sdk(attr(qname = "w14:kx"))]
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
  #[sdk(attr(qname = "w14:ky"))]
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
  #[sdk(attr(qname = "w14:algn"))]
  pub alignment: Option<RectangleAlignmentValues>,
}
/// Defines the TextOutlineEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:textOutline")]
pub struct TextOutlineEffect {
  /// w
  #[sdk(attr(qname = "w14:w"))]
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
  #[sdk(attr(qname = "w14:cap"))]
  pub cap_type: Option<LineCapValues>,
  /// cmpd
  #[sdk(attr(qname = "w14:cmpd"))]
  pub compound: Option<CompoundLineValues>,
  /// algn
  #[sdk(attr(qname = "w14:algn"))]
  pub alignment: Option<PenAlignmentValues>,
  #[sdk(
        choice(
            empty_child(variant = NoFillEmpty, qname = "w14:noFill"),
            child(variant = SolidColorFillProperties, qname = "w14:solidFill"),
            child(variant = GradientFillProperties, qname = "w14:gradFill")
        )
    )]
  pub text_outline_effect_choice1: Option<TextOutlineEffectChoice>,
  /// Defines the PresetLineDashProperties Class.
  #[sdk(child(qname = "w14:prstDash"))]
  pub preset_line_dash_properties: Option<PresetLineDashProperties>,
  #[sdk(
        choice(
            empty_child(variant = RoundEmpty, qname = "w14:round"),
            empty_child(variant = BevelEmpty, qname = "w14:bevel"),
            child(variant = LineJoinMiterProperties, qname = "w14:miter")
        )
    )]
  pub text_outline_effect_choice3: Option<TextOutlineEffectChoice2>,
}
/// Defines the FillTextEffect Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:textFill")]
pub struct FillTextEffect {
  #[sdk(
        choice(
            empty_child(variant = NoFillEmpty, qname = "w14:noFill"),
            child(variant = SolidColorFillProperties, qname = "w14:solidFill"),
            child(variant = GradientFillProperties, qname = "w14:gradFill")
        )
    )]
  pub fill_text_effect_choice: Option<FillTextEffectChoice>,
}
/// Defines the Scene3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:scene3d")]
pub struct Scene3D {
  /// Defines the Camera Class.
  #[sdk(child(qname = "w14:camera"))]
  pub camera: std::boxed::Box<Camera>,
  /// Defines the LightRig Class.
  #[sdk(child(qname = "w14:lightRig"))]
  pub light_rig: std::boxed::Box<LightRig>,
}
/// Defines the Properties3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:props3d")]
pub struct Properties3D {
  /// extrusionH
  #[sdk(attr(qname = "w14:extrusionH"))]
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
  #[sdk(attr(qname = "w14:contourW"))]
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
  #[sdk(attr(qname = "w14:prstMaterial"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_material_type: Option<PresetMaterialTypeValues>,
  /// Defines the BevelTop Class.
  #[sdk(child(qname = "w14:bevelT"))]
  pub bevel_top: Option<BevelTop>,
  /// Defines the BevelBottom Class.
  #[sdk(child(qname = "w14:bevelB"))]
  pub bevel_bottom: Option<BevelBottom>,
  /// Defines the ExtrusionColor Class.
  #[sdk(child(qname = "w14:extrusionClr"))]
  pub extrusion_color: Option<std::boxed::Box<ExtrusionColor>>,
  /// Defines the ContourColor Class.
  #[sdk(child(qname = "w14:contourClr"))]
  pub contour_color: Option<std::boxed::Box<ContourColor>>,
}
/// Defines the Ligatures Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:ligatures")]
pub struct Ligatures {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: LigaturesValues,
}
/// Defines the NumberingFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:numForm")]
pub struct NumberingFormat {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: NumberFormValues,
}
/// Defines the NumberSpacing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:numSpacing")]
pub struct NumberSpacing {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: NumberSpacingValues,
}
/// Defines the StylisticSets Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:stylisticSets")]
pub struct StylisticSets {
  /// Defines the StyleSet Class.
  #[sdk(child(qname = "w14:styleSet"))]
  pub style_set: Vec<StyleSet>,
}
/// Defines the ContextualAlternatives Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:cntxtAlts")]
pub struct ContextualAlternatives {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the ConflictMode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:conflictMode")]
pub struct ConflictMode {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the DiscardImageEditingData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:discardImageEditingData")]
pub struct DiscardImageEditingData {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the Checked Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:checked")]
pub struct Checked {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the ContentPart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:contentPart")]
pub struct ContentPart {
  /// bwMode
  #[sdk(attr(qname = "w14:bwMode"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub relationship_id: crate::simple_type::StringValue,
  /// Defines the WordNonVisualContentPartShapeProperties Class.
  #[sdk(child(qname = "w14:nvContentPartPr"))]
  pub word_non_visual_content_part_shape_properties:
    Option<std::boxed::Box<WordNonVisualContentPartShapeProperties>>,
  /// Defines the Transform2D Class.
  #[sdk(child(qname = "w14:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<Transform2D>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "w14:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DocumentId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:docId")]
pub struct DocumentId {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// val
  #[sdk(attr(qname = "w14:val"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub val: crate::simple_type::HexBinaryValue,
}
/// Defines the CustomXmlConflictInsertionRangeEnd Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:customXmlConflictInsRangeEnd")]
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
#[sdk(qname = "w14:customXmlConflictDelRangeEnd")]
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
#[sdk(qname = "w14:defaultImageDpi")]
pub struct DefaultImageDpi {
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SdtContentCheckBox Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:checkbox")]
pub struct SdtContentCheckBox {
  /// Defines the Checked Class.
  #[sdk(child(qname = "w14:checked"))]
  pub checked: Option<Checked>,
  /// Defines the CheckedState Class.
  #[sdk(child(qname = "w14:checkedState"))]
  pub checked_state: Option<CheckedState>,
  /// Defines the UncheckedState Class.
  #[sdk(child(qname = "w14:uncheckedState"))]
  pub unchecked_state: Option<UncheckedState>,
}
/// Defines the GradientStop Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:gs")]
pub struct GradientStop {
  /// pos
  #[sdk(attr(qname = "w14:pos"))]
  #[sdk(number_range(
    source = 2u32,
    union = 0u64,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true,
  ))]
  pub stop_position: crate::simple_type::Int32Value,
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub gradient_stop_choice: Option<GradientStopChoice>,
}
/// Defines the FillToRectangle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:fillToRect")]
pub struct FillToRectangle {
  /// l
  #[sdk(attr(qname = "w14:l"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// t
  #[sdk(attr(qname = "w14:t"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// r
  #[sdk(attr(qname = "w14:r"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = "w14:b"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
}
/// Defines the GradientStopList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:gsLst")]
pub struct GradientStopList {
  /// Defines the GradientStop Class.
  #[sdk(child(qname = "w14:gs"))]
  pub gradient_stop: Vec<GradientStop>,
}
/// Defines the SphereCoordinates Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:rot")]
pub struct SphereCoordinates {
  /// lat
  #[sdk(attr(qname = "w14:lat"))]
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
  #[sdk(attr(qname = "w14:lon"))]
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
  #[sdk(attr(qname = "w14:rev"))]
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
#[sdk(qname = "w14:camera")]
pub struct Camera {
  /// prst
  #[sdk(attr(qname = "w14:prst"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub preset_camera_type: PresetCameraTypeValues,
}
/// Defines the LightRig Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:lightRig")]
pub struct LightRig {
  /// rig
  #[sdk(attr(qname = "w14:rig"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub light_rig_type: LightRigTypeValues,
  /// dir
  #[sdk(attr(qname = "w14:dir"))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub light_direction_type: LightRigDirectionValues,
  /// Defines the SphereCoordinates Class.
  #[sdk(child(qname = "w14:rot"))]
  pub sphere_coordinates: Option<SphereCoordinates>,
}
/// Defines the BevelTop Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:bevelT")]
pub struct BevelTop {
  /// w
  #[sdk(attr(qname = "w14:w"))]
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
  #[sdk(attr(qname = "w14:h"))]
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
  #[sdk(attr(qname = "w14:prst"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the BevelBottom Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:bevelB")]
pub struct BevelBottom {
  /// w
  #[sdk(attr(qname = "w14:w"))]
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
  #[sdk(attr(qname = "w14:h"))]
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
  #[sdk(attr(qname = "w14:prst"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the ExtrusionColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:extrusionClr")]
pub struct ExtrusionColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub extrusion_color_choice: Option<ExtrusionColorChoice>,
}
/// Defines the ContourColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:contourClr")]
pub struct ContourColor {
  #[sdk(
        choice(
            child(variant = RgbColorModelHex, qname = "w14:srgbClr"),
            child(variant = SchemeColor, qname = "w14:schemeClr")
        )
    )]
  pub contour_color_choice: Option<ContourColorChoice>,
}
/// Defines the StyleSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:styleSet")]
pub struct StyleSet {
  /// id
  #[sdk(attr(qname = "w14:id"))]
  pub id: crate::simple_type::UInt32Value,
  /// val
  #[sdk(attr(qname = "w14:val"))]
  pub val: Option<OnOffValues>,
}
/// Defines the CheckedState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:checkedState")]
pub struct CheckedState {
  /// font
  #[sdk(attr(qname = "w14:font"))]
  pub font: Option<crate::simple_type::StringValue>,
  /// val
  #[sdk(attr(qname = "w14:val"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 2u32, max = 2u32))]
  pub val: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the UncheckedState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:uncheckedState")]
pub struct UncheckedState {
  /// font
  #[sdk(attr(qname = "w14:font"))]
  pub font: Option<crate::simple_type::StringValue>,
  /// val
  #[sdk(attr(qname = "w14:val"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 2u32, max = 2u32))]
  pub val: Option<crate::simple_type::HexBinaryValue>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:cNvPr")]
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
/// Defines the NonVisualInkContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// Defines the ContentPartLocks Class.
  #[sdk(child(qname = "a14:cpLocks"))]
  pub content_part_locks: Option<std::boxed::Box<crate::schemas::a14::ContentPartLocks>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "a14:extLst"))]
  pub office_art_extension_list: Option<crate::schemas::a14::OfficeArtExtensionList>,
}
/// Defines the WordNonVisualContentPartShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:nvContentPartPr")]
pub struct WordNonVisualContentPartShapeProperties {
  /// Defines the NonVisualDrawingProperties Class.
  #[sdk(child(qname = "w14:cNvPr"))]
  pub non_visual_drawing_properties: Option<std::boxed::Box<NonVisualDrawingProperties>>,
  /// Defines the NonVisualInkContentPartProperties Class.
  #[sdk(child(qname = "w14:cNvContentPartPr"))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:xfrm")]
pub struct Transform2D {
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
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunConflictInsertionChoice {
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
  CustomXmlConflictInsertionRangeStart(std::boxed::Box<CustomXmlConflictInsertionRangeStart>),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(std::boxed::Box<CustomXmlConflictInsertionRangeEnd>),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(std::boxed::Box<CustomXmlConflictDeletionRangeStart>),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(std::boxed::Box<CustomXmlConflictDeletionRangeEnd>),
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
  RunConflictInsertion(std::boxed::Box<RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<RunConflictDeletion>),
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
  #[sdk(child(qname = "m:r"))]
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  #[sdk(child(qname = "w:r"))]
  WRun(std::boxed::Box<crate::schemas::w::Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<crate::schemas::w::BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<crate::schemas::w::BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RunConflictDeletionChoice {
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
  CustomXmlConflictInsertionRangeStart(std::boxed::Box<CustomXmlConflictInsertionRangeStart>),
  /// Defines the CustomXmlConflictInsertionRangeEnd Class.
  CustomXmlConflictInsertionRangeEnd(std::boxed::Box<CustomXmlConflictInsertionRangeEnd>),
  /// Defines the CustomXmlConflictDeletionRangeStart Class.
  CustomXmlConflictDeletionRangeStart(std::boxed::Box<CustomXmlConflictDeletionRangeStart>),
  /// Defines the CustomXmlConflictDeletionRangeEnd Class.
  CustomXmlConflictDeletionRangeEnd(std::boxed::Box<CustomXmlConflictDeletionRangeEnd>),
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
  RunConflictInsertion(std::boxed::Box<RunConflictInsertion>),
  /// Defines the RunConflictDeletion Class.
  RunConflictDeletion(std::boxed::Box<RunConflictDeletion>),
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
  #[sdk(child(qname = "m:r"))]
  MRun(std::boxed::Box<crate::schemas::m::Run>),
  /// Phonetic Guide Text Run.
  #[sdk(child(qname = "w:r"))]
  WRun(std::boxed::Box<crate::schemas::w::Run>),
  /// Defines the BidirectionalOverride Class.
  BidirectionalOverride(std::boxed::Box<crate::schemas::w::BidirectionalOverride>),
  /// Defines the BidirectionalEmbedding Class.
  BidirectionalEmbedding(std::boxed::Box<crate::schemas::w::BidirectionalEmbedding>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RgbColorModelHexChoice {
  /// Defines the Tint Class.
  Tint(std::boxed::Box<Tint>),
  /// Defines the Shade Class.
  Shade(std::boxed::Box<Shade>),
  /// Defines the Alpha Class.
  Alpha(std::boxed::Box<Alpha>),
  /// Defines the HueModulation Class.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Defines the Saturation Class.
  Saturation(std::boxed::Box<Saturation>),
  /// Defines the SaturationOffset Class.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Defines the SaturationModulation Class.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Defines the Luminance Class.
  Luminance(std::boxed::Box<Luminance>),
  /// Defines the LuminanceOffset Class.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Defines the LuminanceModulation Class.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SchemeColorChoice {
  /// Defines the Tint Class.
  Tint(std::boxed::Box<Tint>),
  /// Defines the Shade Class.
  Shade(std::boxed::Box<Shade>),
  /// Defines the Alpha Class.
  Alpha(std::boxed::Box<Alpha>),
  /// Defines the HueModulation Class.
  HueModulation(std::boxed::Box<HueModulation>),
  /// Defines the Saturation Class.
  Saturation(std::boxed::Box<Saturation>),
  /// Defines the SaturationOffset Class.
  SaturationOffset(std::boxed::Box<SaturationOffset>),
  /// Defines the SaturationModulation Class.
  SaturationModulation(std::boxed::Box<SaturationModulation>),
  /// Defines the Luminance Class.
  Luminance(std::boxed::Box<Luminance>),
  /// Defines the LuminanceOffset Class.
  LuminanceOffset(std::boxed::Box<LuminanceOffset>),
  /// Defines the LuminanceModulation Class.
  LuminanceModulation(std::boxed::Box<LuminanceModulation>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SolidColorFillPropertiesChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientFillPropertiesChoice {
  /// Defines the LinearShadeProperties Class.
  LinearShadeProperties(std::boxed::Box<LinearShadeProperties>),
  /// Defines the PathShadeProperties Class.
  PathShadeProperties(std::boxed::Box<PathShadeProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GlowChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShadowChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextOutlineEffectChoice {
  /// Defines the NoFillEmpty Class.
  #[sdk(empty_child(qname = "w14:noFill"))]
  NoFillEmpty,
  /// Defines the SolidColorFillProperties Class.
  SolidColorFillProperties(std::boxed::Box<SolidColorFillProperties>),
  /// Defines the GradientFillProperties Class.
  GradientFillProperties(std::boxed::Box<GradientFillProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextOutlineEffectChoice2 {
  /// Defines the RoundEmpty Class.
  #[sdk(empty_child(qname = "w14:round"))]
  RoundEmpty,
  /// Defines the BevelEmpty Class.
  #[sdk(empty_child(qname = "w14:bevel"))]
  BevelEmpty,
  /// Defines the LineJoinMiterProperties Class.
  LineJoinMiterProperties(std::boxed::Box<LineJoinMiterProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillTextEffectChoice {
  /// Defines the NoFillEmpty Class.
  #[sdk(empty_child(qname = "w14:noFill"))]
  NoFillEmpty,
  /// Defines the SolidColorFillProperties Class.
  SolidColorFillProperties(std::boxed::Box<SolidColorFillProperties>),
  /// Defines the GradientFillProperties Class.
  GradientFillProperties(std::boxed::Box<GradientFillProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GradientStopChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ExtrusionColorChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ContourColorChoice {
  /// Defines the RgbColorModelHex Class.
  RgbColorModelHex(std::boxed::Box<RgbColorModelHex>),
  /// Defines the SchemeColor Class.
  SchemeColor(std::boxed::Box<SchemeColor>),
}
