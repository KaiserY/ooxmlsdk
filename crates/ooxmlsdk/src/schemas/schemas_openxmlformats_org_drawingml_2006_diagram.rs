//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ColorApplicationMethodValues {
  #[sdk(rename = "span")]
  #[default]
  Span,
  #[sdk(rename = "cycle")]
  Cycle,
  #[sdk(rename = "repeat")]
  Repeat,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HueDirectionValues {
  #[sdk(rename = "cw")]
  #[default]
  Clockwise,
  #[sdk(rename = "ccw")]
  Counterclockwise,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PointValues {
  #[sdk(rename = "node")]
  #[default]
  Node,
  #[sdk(rename = "asst")]
  Assistant,
  #[sdk(rename = "doc")]
  Document,
  #[sdk(rename = "pres")]
  Presentation,
  #[sdk(rename = "parTrans")]
  ParentTransition,
  #[sdk(rename = "sibTrans")]
  SiblingTransition,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectionValues {
  #[sdk(rename = "parOf")]
  #[default]
  ParentOf,
  #[sdk(rename = "presOf")]
  PresentationOf,
  #[sdk(rename = "presParOf")]
  PresentationParentOf,
  #[sdk(rename = "unknownRelationship")]
  UnknownRelationship,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DirectionValues {
  #[sdk(rename = "norm")]
  #[default]
  Normal,
  #[sdk(rename = "rev")]
  Reversed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HierarchyBranchStyleValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "hang")]
  Hanging,
  #[sdk(rename = "std")]
  Standard,
  #[sdk(rename = "init")]
  Initial,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimateOneByOneValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "one")]
  One,
  #[sdk(rename = "branch")]
  Branch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AnimationLevelStringValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "lvl")]
  Level,
  #[sdk(rename = "ctr")]
  Center,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ResizeHandlesStringValues {
  #[sdk(rename = "exact")]
  #[default]
  Exact,
  #[sdk(rename = "rel")]
  Relative,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AlgorithmValues {
  #[sdk(rename = "composite")]
  #[default]
  Composite,
  #[sdk(rename = "conn")]
  Connector,
  #[sdk(rename = "cycle")]
  Cycle,
  #[sdk(rename = "hierChild")]
  HierarchyChild,
  #[sdk(rename = "hierRoot")]
  HierarchyRoot,
  #[sdk(rename = "pyra")]
  Pyramid,
  #[sdk(rename = "lin")]
  Linear,
  #[sdk(rename = "sp")]
  Space,
  #[sdk(rename = "tx")]
  Text,
  #[sdk(rename = "snake")]
  Snake,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AxisValues {
  #[sdk(rename = "self")]
  #[default]
  _Self,
  #[sdk(rename = "ch")]
  Child,
  #[sdk(rename = "des")]
  Descendant,
  #[sdk(rename = "desOrSelf")]
  DescendantOrSelf,
  #[sdk(rename = "par")]
  Parent,
  #[sdk(rename = "ancst")]
  Ancestor,
  #[sdk(rename = "ancstOrSelf")]
  AncestorOrSelf,
  #[sdk(rename = "followSib")]
  FollowSibling,
  #[sdk(rename = "precedSib")]
  PrecedingSibling,
  #[sdk(rename = "follow")]
  Follow,
  #[sdk(rename = "preced")]
  Preceding,
  #[sdk(rename = "root")]
  Root,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BoolOperatorValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "equ")]
  Equal,
  #[sdk(rename = "gte")]
  GreaterThanOrEqualTo,
  #[sdk(rename = "lte")]
  LessThanOrEqualTo,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChildOrderValues {
  #[sdk(rename = "b")]
  #[default]
  Bottom,
  #[sdk(rename = "t")]
  Top,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConstraintValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "alignOff")]
  AlignmentOffset,
  #[sdk(rename = "begMarg")]
  BeginningMargin,
  #[sdk(rename = "bendDist")]
  BendingDistance,
  #[sdk(rename = "begPad")]
  BeginningPadding,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "bMarg")]
  BottomMargin,
  #[sdk(rename = "bOff")]
  BottomOffset,
  #[sdk(rename = "ctrX")]
  CenterHeight,
  #[sdk(rename = "ctrXOff")]
  CenterXOffset,
  #[sdk(rename = "ctrY")]
  CenterWidth,
  #[sdk(rename = "ctrYOff")]
  CenterYOffset,
  #[sdk(rename = "connDist")]
  ConnectionDistance,
  #[sdk(rename = "diam")]
  Diameter,
  #[sdk(rename = "endMarg")]
  EndMargin,
  #[sdk(rename = "endPad")]
  EndPadding,
  #[sdk(rename = "h")]
  Height,
  #[sdk(rename = "hArH")]
  ArrowheadHeight,
  #[sdk(rename = "hOff")]
  HeightOffset,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "lMarg")]
  LeftMargin,
  #[sdk(rename = "lOff")]
  LeftOffset,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "rMarg")]
  RightMargin,
  #[sdk(rename = "rOff")]
  RightOffset,
  #[sdk(rename = "primFontSz")]
  PrimaryFontSize,
  #[sdk(rename = "pyraAcctRatio")]
  PyramidAccentRatio,
  #[sdk(rename = "secFontSz")]
  SecondaryFontSize,
  #[sdk(rename = "sibSp")]
  SiblingSpacing,
  #[sdk(rename = "secSibSp")]
  SecondarySiblingSpacing,
  #[sdk(rename = "sp")]
  Spacing,
  #[sdk(rename = "stemThick")]
  StemThickness,
  #[sdk(rename = "t")]
  Top,
  #[sdk(rename = "tMarg")]
  TopMargin,
  #[sdk(rename = "tOff")]
  TopOffset,
  #[sdk(rename = "userA")]
  UserDefinedA,
  #[sdk(rename = "userB")]
  UserDefinedB,
  #[sdk(rename = "userC")]
  UserDefinedC,
  #[sdk(rename = "userD")]
  UserDefinedD,
  #[sdk(rename = "userE")]
  UserDefinedE,
  #[sdk(rename = "userF")]
  UserDefinedF,
  #[sdk(rename = "userG")]
  UserDefinedG,
  #[sdk(rename = "userH")]
  UserDefinedH,
  #[sdk(rename = "userI")]
  UserDefinedI,
  #[sdk(rename = "userJ")]
  UserDefinedJ,
  #[sdk(rename = "userK")]
  UserDefinedK,
  #[sdk(rename = "userL")]
  UserDefinedL,
  #[sdk(rename = "userM")]
  UserDefinedM,
  #[sdk(rename = "userN")]
  UserDefinedN,
  #[sdk(rename = "userO")]
  UserDefinedO,
  #[sdk(rename = "userP")]
  UserDefinedP,
  #[sdk(rename = "userQ")]
  UserDefinedQ,
  #[sdk(rename = "userR")]
  UserDefinedR,
  #[sdk(rename = "userS")]
  UserDefinedS,
  #[sdk(rename = "userT")]
  UserDefinedT,
  #[sdk(rename = "userU")]
  UserDefinedU,
  #[sdk(rename = "userV")]
  UserDefinedV,
  #[sdk(rename = "userW")]
  UserDefinedW,
  #[sdk(rename = "userX")]
  UserDefinedX,
  #[sdk(rename = "userY")]
  UserDefinedY,
  #[sdk(rename = "userZ")]
  UserDefinedZ,
  #[sdk(rename = "w")]
  Width,
  #[sdk(rename = "wArH")]
  ArrowheadWidth,
  #[sdk(rename = "wOff")]
  WidthOffset,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConstraintRelationshipValues {
  #[sdk(rename = "self")]
  #[default]
  _Self,
  #[sdk(rename = "ch")]
  Child,
  #[sdk(rename = "des")]
  Descendant,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ElementValues {
  #[sdk(rename = "all")]
  #[default]
  All,
  #[sdk(rename = "doc")]
  Document,
  #[sdk(rename = "node")]
  Node,
  #[sdk(rename = "norm")]
  Normal,
  #[sdk(rename = "nonNorm")]
  NonNormal,
  #[sdk(rename = "asst")]
  Assistant,
  #[sdk(rename = "nonAsst")]
  NonAssistant,
  #[sdk(rename = "parTrans")]
  ParentTransition,
  #[sdk(rename = "pres")]
  Presentation,
  #[sdk(rename = "sibTrans")]
  SiblingTransition,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ParameterIdValues {
  #[sdk(rename = "horzAlign")]
  #[default]
  HorizontalAlignment,
  #[sdk(rename = "vertAlign")]
  VerticalAlignment,
  #[sdk(rename = "chDir")]
  ChildDirection,
  #[sdk(rename = "chAlign")]
  ChildAlignment,
  #[sdk(rename = "secChAlign")]
  SecondaryChildAlignment,
  #[sdk(rename = "linDir")]
  LinearDirection,
  #[sdk(rename = "secLinDir")]
  SecondaryLinearDirection,
  #[sdk(rename = "stElem")]
  StartElement,
  #[sdk(rename = "bendPt")]
  BendPoint,
  #[sdk(rename = "connRout")]
  ConnectionRoute,
  #[sdk(rename = "begSty")]
  BeginningArrowheadStyle,
  #[sdk(rename = "endSty")]
  EndStyle,
  #[sdk(rename = "dim")]
  ConnectorDimension,
  #[sdk(rename = "rotPath")]
  RotationPath,
  #[sdk(rename = "ctrShpMap")]
  CenterShapeMapping,
  #[sdk(rename = "nodeHorzAlign")]
  NodeHorizontalAlignment,
  #[sdk(rename = "nodeVertAlign")]
  NodeVerticalAlignment,
  #[sdk(rename = "fallback")]
  FallbackScale,
  #[sdk(rename = "txDir")]
  TextDirection,
  #[sdk(rename = "pyraAcctPos")]
  PyramidAccentPosition,
  #[sdk(rename = "pyraAcctTxMar")]
  PyramidAccentTextMargin,
  #[sdk(rename = "txBlDir")]
  TextBlockDirection,
  #[sdk(rename = "txAnchorHorz")]
  TextAnchorHorizontal,
  #[sdk(rename = "txAnchorVert")]
  TextAnchorVertical,
  #[sdk(rename = "txAnchorHorzCh")]
  TextAnchorHorizontalWithChildren,
  #[sdk(rename = "txAnchorVertCh")]
  TextAnchorVerticalWithChildren,
  #[sdk(rename = "parTxLTRAlign")]
  ParentTextLeftToRightAlignment,
  #[sdk(rename = "parTxRTLAlign")]
  ParentTextRightToLeftAlignment,
  #[sdk(rename = "shpTxLTRAlignCh")]
  ShapeTextLeftToRightAlignment,
  #[sdk(rename = "shpTxRTLAlignCh")]
  ShapeTextRightToLeftAlignment,
  #[sdk(rename = "autoTxRot")]
  AutoTextRotation,
  #[sdk(rename = "grDir")]
  GrowDirection,
  #[sdk(rename = "flowDir")]
  FlowDirection,
  #[sdk(rename = "contDir")]
  ContinueDirection,
  #[sdk(rename = "bkpt")]
  Breakpoint,
  #[sdk(rename = "off")]
  Offset,
  #[sdk(rename = "hierAlign")]
  HierarchyAlignment,
  #[sdk(rename = "bkPtFixedVal")]
  BreakpointFixedValue,
  #[sdk(rename = "stBulletLvl")]
  StartBulletsAtLevel,
  #[sdk(rename = "stAng")]
  StartAngle,
  #[sdk(rename = "spanAng")]
  SpanAngle,
  #[sdk(rename = "ar")]
  AspectRatio,
  #[sdk(rename = "lnSpPar")]
  LineSpacingParent,
  #[sdk(rename = "lnSpAfParP")]
  LineSpacingAfterParentParagraph,
  #[sdk(rename = "lnSpCh")]
  LineSpacingChildren,
  #[sdk(rename = "lnSpAfChP")]
  LineSpacingAfterChildrenParagraph,
  #[sdk(rename = "rtShortDist")]
  RouteShortestDistance,
  #[sdk(rename = "alignTx")]
  TextAlignment,
  #[sdk(rename = "pyraLvlNode")]
  PyramidLevelNode,
  #[sdk(rename = "pyraAcctBkgdNode")]
  PyramidAccentBackgroundNode,
  #[sdk(rename = "pyraAcctTxNode")]
  PyramidAccentTextNode,
  #[sdk(rename = "srcNode")]
  SourceNode,
  #[sdk(rename = "dstNode")]
  DestinationNode,
  #[sdk(rename = "begPts")]
  BeginningPoints,
  #[sdk(rename = "endPts")]
  EndPoints,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FunctionValues {
  #[sdk(rename = "cnt")]
  #[default]
  Count,
  #[sdk(rename = "pos")]
  Position,
  #[sdk(rename = "revPos")]
  ReversePosition,
  #[sdk(rename = "posEven")]
  PositionEven,
  #[sdk(rename = "posOdd")]
  PositionOdd,
  #[sdk(rename = "var")]
  Variable,
  #[sdk(rename = "depth")]
  Depth,
  #[sdk(rename = "maxDepth")]
  MaxDepth,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FunctionOperatorValues {
  #[sdk(rename = "equ")]
  #[default]
  Equal,
  #[sdk(rename = "neq")]
  NotEqualTo,
  #[sdk(rename = "gt")]
  GreaterThan,
  #[sdk(rename = "lt")]
  LessThan,
  #[sdk(rename = "gte")]
  GreaterThanOrEqualTo,
  #[sdk(rename = "lte")]
  LessThanOrEqualTo,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAlignmentValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChildDirectionValues {
  #[sdk(rename = "horz")]
  #[default]
  Horizontal,
  #[sdk(rename = "vert")]
  Vertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ChildAlignmentValues {
  #[sdk(rename = "t")]
  #[default]
  Top,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SecondaryChildAlignmentValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "t")]
  Top,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LinearDirectionValues {
  #[sdk(rename = "fromL")]
  #[default]
  FromLeft,
  #[sdk(rename = "fromR")]
  FromRight,
  #[sdk(rename = "fromT")]
  FromTop,
  #[sdk(rename = "fromB")]
  FromBottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SecondaryLinearDirectionValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "fromL")]
  FromLeft,
  #[sdk(rename = "fromR")]
  FromRight,
  #[sdk(rename = "fromT")]
  FromTop,
  #[sdk(rename = "fromB")]
  FromBottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StartingElementValues {
  #[sdk(rename = "node")]
  #[default]
  Node,
  #[sdk(rename = "trans")]
  Transition,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RotationPathValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "alongPath")]
  AlongPath,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CenterShapeMappingValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "fNode")]
  FirstNode,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BendPointValues {
  #[sdk(rename = "beg")]
  #[default]
  Beginning,
  #[sdk(rename = "def")]
  Default,
  #[sdk(rename = "end")]
  End,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectorRoutingValues {
  #[sdk(rename = "stra")]
  #[default]
  Straight,
  #[sdk(rename = "bend")]
  Bending,
  #[sdk(rename = "curve")]
  Curve,
  #[sdk(rename = "longCurve")]
  LongCurve,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ArrowheadStyleValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "arr")]
  Arrow,
  #[sdk(rename = "noArr")]
  NoArrow,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectorDimensionValues {
  #[sdk(rename = "1D")]
  #[default]
  OneDimension,
  #[sdk(rename = "2D")]
  TwoDimension,
  #[sdk(rename = "cust")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConnectorPointValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "bCtr")]
  BottomCenter,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "midL")]
  MiddleLeft,
  #[sdk(rename = "midR")]
  MiddleRight,
  #[sdk(rename = "tCtr")]
  TopCenter,
  #[sdk(rename = "bL")]
  BottomLeft,
  #[sdk(rename = "bR")]
  BottomRight,
  #[sdk(rename = "tL")]
  TopLeft,
  #[sdk(rename = "tR")]
  TopRight,
  #[sdk(rename = "radial")]
  Radial,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NodeHorizontalAlignmentValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NodeVerticalAlignmentValues {
  #[sdk(rename = "t")]
  #[default]
  Top,
  #[sdk(rename = "mid")]
  Middle,
  #[sdk(rename = "b")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FallbackDimensionValues {
  #[sdk(rename = "1D")]
  #[default]
  OneDimension,
  #[sdk(rename = "2D")]
  TwoDimension,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextDirectionValues {
  #[sdk(rename = "fromT")]
  #[default]
  FromTop,
  #[sdk(rename = "fromB")]
  FromBottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PyramidAccentPositionValues {
  #[sdk(rename = "bef")]
  #[default]
  Before,
  #[sdk(rename = "aft")]
  After,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PyramidAccentTextMarginValues {
  #[sdk(rename = "step")]
  #[default]
  Step,
  #[sdk(rename = "stack")]
  Stack,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextBlockDirectionValues {
  #[sdk(rename = "horz")]
  #[default]
  Horizontal,
  #[sdk(rename = "vert")]
  Vertical,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAnchorHorizontalValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "ctr")]
  Center,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAnchorVerticalValues {
  #[sdk(rename = "t")]
  #[default]
  Top,
  #[sdk(rename = "mid")]
  Middle,
  #[sdk(rename = "b")]
  Bottom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextAlignmentValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "r")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AutoTextRotationValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "upr")]
  Upright,
  #[sdk(rename = "grav")]
  Gravity,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GrowDirectionValues {
  #[sdk(rename = "tL")]
  #[default]
  TopLeft,
  #[sdk(rename = "tR")]
  TopRight,
  #[sdk(rename = "bL")]
  BottomLeft,
  #[sdk(rename = "bR")]
  BottomRight,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FlowDirectionValues {
  #[sdk(rename = "row")]
  #[default]
  Row,
  #[sdk(rename = "col")]
  Column,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ContinueDirectionValues {
  #[sdk(rename = "revDir")]
  #[default]
  ReverseDirection,
  #[sdk(rename = "sameDir")]
  SameDirection,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BreakpointValues {
  #[sdk(rename = "endCnv")]
  #[default]
  EndCanvas,
  #[sdk(rename = "bal")]
  Balanced,
  #[sdk(rename = "fixed")]
  Fixed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OffsetValues {
  #[sdk(rename = "ctr")]
  #[default]
  Center,
  #[sdk(rename = "off")]
  Offset,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HierarchyAlignmentValues {
  #[sdk(rename = "tL")]
  #[default]
  TopLeft,
  #[sdk(rename = "tR")]
  TopRight,
  #[sdk(rename = "tCtrCh")]
  TopCenterChildren,
  #[sdk(rename = "tCtrDes")]
  TopCenterDescendants,
  #[sdk(rename = "bL")]
  BottomLeft,
  #[sdk(rename = "bR")]
  BottomRight,
  #[sdk(rename = "bCtrCh")]
  BottomCenterChild,
  #[sdk(rename = "bCtrDes")]
  BottomCenterDescendant,
  #[sdk(rename = "lT")]
  LeftTop,
  #[sdk(rename = "lB")]
  LeftBottom,
  #[sdk(rename = "lCtrCh")]
  LeftCenterChild,
  #[sdk(rename = "lCtrDes")]
  LeftCenterDescendant,
  #[sdk(rename = "rT")]
  RightTop,
  #[sdk(rename = "rB")]
  RightBottom,
  #[sdk(rename = "rCtrCh")]
  RightCenterChildren,
  #[sdk(rename = "rCtrDes")]
  RightCenterDescendants,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VariableValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "orgChart")]
  OrganizationalChart,
  #[sdk(rename = "chMax")]
  ChildMax,
  #[sdk(rename = "chPref")]
  ChildPreference,
  #[sdk(rename = "bulEnabled")]
  BulletsEnabled,
  #[sdk(rename = "dir")]
  Direction,
  #[sdk(rename = "hierBranch")]
  HierarchyBranch,
  #[sdk(rename = "animOne")]
  AnimateOne,
  #[sdk(rename = "animLvl")]
  AnimationLevel,
  #[sdk(rename = "resizeHandles")]
  ResizeHandles,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OutputShapeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "conn")]
  Connection,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentValues {
  #[sdk(rename = "t")]
  #[default]
  Top,
  #[sdk(rename = "mid")]
  Middle,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "top")]
  Top2010,
  #[sdk(rename = "center")]
  Middle2010,
  #[sdk(rename = "bottom")]
  Bottom2010,
}
/// Color Transform Definitions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ColorTransform/dgm:colorsDef")]
pub struct ColorsDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Unique ID
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// Minimum Version
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTName/dgm:title"))]
  pub dgm_title: Vec<ColorDefinitionTitle>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTDescription/dgm:desc"))]
  pub dgm_desc: Vec<ColorTransformDescription>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTCategories/dgm:catLst"))]
  pub dgm_cat_lst: Option<ColorTransformCategories>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTStyleLabel/dgm:styleLbl"))]
  pub dgm_style_lbl: Vec<ColorTransformStyleLabel>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// Color Transform Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ColorTransformHeader/dgm:colorsDefHdr")]
pub struct ColorsDefinitionHeader {
  /// Unique ID
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: crate::simple_type::StringValue,
  /// Minimum Version
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// Resource ID
  #[sdk(attr(qname = ":resId"))]
  pub resource_id: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTName/dgm:title"))]
  pub dgm_title: Vec<ColorDefinitionTitle>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTDescription/dgm:desc"))]
  pub dgm_desc: Vec<ColorTransformDescription>,
  /// _
  #[sdk(child(qname = "dgm:CT_CTCategories/dgm:catLst"))]
  pub dgm_cat_lst: Option<ColorTransformCategories>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// Color Transform Header List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ColorTransformHeaderLst/dgm:colorsDefHdrLst")]
pub struct ColorsDefinitionHeaderList {
  /// _
  #[sdk(child(qname = "dgm:CT_ColorTransformHeader/dgm:colorsDefHdr"))]
  pub dgm_colors_def_hdr: Vec<ColorsDefinitionHeader>,
}
/// Data Model.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DataModelRoot/dgm:dataModel")]
pub struct DataModelRoot {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Point List
  #[sdk(child(qname = "dgm:CT_PtList/dgm:ptLst"))]
  pub point_list: std::boxed::Box<PointList>,
  /// Connection List
  #[sdk(child(qname = "dgm:CT_CxnList/dgm:cxnLst"))]
  pub connection_list: Option<ConnectionList>,
  /// Background Formatting
  #[sdk(child(qname = "a:CT_BackgroundFormatting/dgm:bg"))]
  pub background: Option<std::boxed::Box<Background>>,
  /// Whole E2O Formatting
  #[sdk(child(qname = "a:CT_WholeE2oFormatting/dgm:whole"))]
  pub whole: Option<std::boxed::Box<Whole>>,
  /// _
  #[sdk(child(qname = "a:CT_DataModelExtensionList/dgm:extLst"))]
  pub data_model_extension_list: Option<DataModelExtensionList>,
}
/// Layout Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DiagramDefinition/dgm:layoutDef")]
pub struct LayoutDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// uniqueId
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// minVer
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// defStyle
  #[sdk(attr(qname = ":defStyle"))]
  pub default_style: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "dgm:CT_Name/dgm:title"))]
  pub dgm_title: Vec<Title>,
  /// _
  #[sdk(child(qname = "dgm:CT_Description/dgm:desc"))]
  pub dgm_desc: Vec<Description>,
  /// _
  #[sdk(child(qname = "dgm:CT_Categories/dgm:catLst"))]
  pub dgm_cat_lst: Option<CategoryList>,
  /// _
  #[sdk(child(qname = "dgm:CT_SampleData/dgm:sampData"))]
  pub dgm_samp_data: Option<std::boxed::Box<SampleData>>,
  /// _
  #[sdk(child(qname = "dgm:CT_SampleData/dgm:styleData"))]
  pub dgm_style_data: Option<std::boxed::Box<StyleData>>,
  /// _
  #[sdk(child(qname = "dgm:CT_SampleData/dgm:clrData"))]
  pub dgm_clr_data: Option<std::boxed::Box<ColorData>>,
  /// _
  #[sdk(child(qname = "dgm:CT_LayoutNode/dgm:layoutNode"))]
  pub dgm_layout_node: std::boxed::Box<LayoutNode>,
  /// _
  #[sdk(child(qname = "dgm:CT_DiagramDefinitionExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<DiagramDefinitionExtensionList>,
}
/// Layout Definition Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DiagramDefinitionHeader/dgm:layoutDefHdr")]
pub struct LayoutDefinitionHeader {
  /// Unique Identifier
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: crate::simple_type::StringValue,
  /// Minimum Version
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// Default Style
  #[sdk(attr(qname = ":defStyle"))]
  pub default_style: Option<crate::simple_type::StringValue>,
  /// Resource Identifier
  #[sdk(attr(qname = ":resId"))]
  pub resource_id: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "dgm:CT_Name/dgm:title"))]
  pub dgm_title: Vec<Title>,
  /// _
  #[sdk(child(qname = "dgm:CT_Description/dgm:desc"))]
  pub dgm_desc: Vec<Description>,
  /// _
  #[sdk(child(qname = "dgm:CT_Categories/dgm:catLst"))]
  pub dgm_cat_lst: Option<CategoryList>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// Diagram Layout Header List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DiagramDefinitionHeaderLst/dgm:layoutDefHdrLst")]
pub struct LayoutDefinitionHeaderList {
  /// _
  #[sdk(child(qname = "dgm:CT_DiagramDefinitionHeader/dgm:layoutDefHdr"))]
  pub dgm_layout_def_hdr: Vec<LayoutDefinitionHeader>,
}
/// Explicit Relationships to Diagram Parts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_RelIds/dgm:relIds")]
pub struct RelationshipIds {
  /// Explicit Relationship to Diagram Data Part
  #[sdk(attr(qname = "r:dm"))]
  pub data_part: crate::simple_type::StringValue,
  /// Explicit Relationship to Diagram Layout Definition Part
  #[sdk(attr(qname = "r:lo"))]
  pub layout_part: crate::simple_type::StringValue,
  /// Explicit Relationship to Style Definition Part
  #[sdk(attr(qname = "r:qs"))]
  pub style_part: crate::simple_type::StringValue,
  /// Explicit Relationship to Diagram Colors Part
  #[sdk(attr(qname = "r:cs"))]
  pub color_part: crate::simple_type::StringValue,
}
/// Style Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_StyleDefinition/dgm:styleDef")]
pub struct StyleDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Unique Style ID
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// Minimum Version
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDName/dgm:title"))]
  pub dgm_title: Vec<StyleDefinitionTitle>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDDescription/dgm:desc"))]
  pub dgm_desc: Vec<StyleLabelDescription>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDCategories/dgm:catLst"))]
  pub dgm_cat_lst: Option<StyleDisplayCategories>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/dgm:scene3d"))]
  pub dgm_scene3d: Option<std::boxed::Box<Scene3D>>,
  /// _
  #[sdk(child(qname = "dgm:CT_StyleLabel/dgm:styleLbl"))]
  pub dgm_style_lbl: Vec<StyleLabel>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// Style Definition Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_StyleDefinitionHeader/dgm:styleDefHdr")]
pub struct StyleDefinitionHeader {
  /// Unique Style ID
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: crate::simple_type::StringValue,
  /// Minimum Version
  #[sdk(attr(qname = ":minVer"))]
  pub min_version: Option<crate::simple_type::StringValue>,
  /// Resource ID
  #[sdk(attr(qname = ":resId"))]
  pub resource_id: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDName/dgm:title"))]
  pub dgm_title: Vec<StyleDefinitionTitle>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDDescription/dgm:desc"))]
  pub dgm_desc: Vec<StyleLabelDescription>,
  /// _
  #[sdk(child(qname = "dgm:CT_SDCategories/dgm:catLst"))]
  pub dgm_cat_lst: Option<StyleDisplayCategories>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// List of Style Definition Headers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_StyleDefinitionHeaderLst/dgm:styleDefHdrLst")]
pub struct StyleDefinitionHeaderList {
  /// _
  #[sdk(child(qname = "dgm:CT_StyleDefinitionHeader/dgm:styleDefHdr"))]
  pub dgm_style_def_hdr: Vec<StyleDefinitionHeader>,
}
/// Color Transform Category.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CTCategory/dgm:cat")]
pub struct ColorTransformCategory {
  /// Category Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub r#type: crate::simple_type::StringValue,
  /// Priority
  #[sdk(attr(qname = ":pri"))]
  pub priority: crate::simple_type::UInt32Value,
}
/// Fill Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:fillClrLst")]
pub struct FillColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub fill_color_list_choice: Vec<FillColorListChoice>,
}
/// Line Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:linClrLst")]
pub struct LineColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub line_color_list_choice: Vec<LineColorListChoice>,
}
/// Effect Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:effectClrLst")]
pub struct EffectColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub effect_color_list_choice: Vec<EffectColorListChoice>,
}
/// Text Line Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:txLinClrLst")]
pub struct TextLineColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub text_line_color_list_choice: Vec<TextLineColorListChoice>,
}
/// Text Fill Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:txFillClrLst")]
pub struct TextFillColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub text_fill_color_list_choice: Vec<TextFillColorListChoice>,
}
/// Text Effect Color List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/dgm:txEffectClrLst")]
pub struct TextEffectColorList {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub text_effect_color_list_choice: Vec<TextEffectColorListChoice>,
}
/// Defines the ColorsType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Colors/")]
pub struct ColorsType {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Color Application Method Type
  #[sdk(attr(qname = ":meth"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub method: Option<ColorApplicationMethodValues>,
  /// Hue Direction
  #[sdk(attr(qname = ":hueDir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hue_direction: Option<HueDirectionValues>,
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
  pub xml_children: Vec<ColorsTypeChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/dgm:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Title.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CTName/dgm:title")]
pub struct ColorDefinitionTitle {
  /// Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Description Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Description.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CTDescription/dgm:desc")]
pub struct ColorTransformDescription {
  /// Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Description Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Color Transform Category List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CTCategories/dgm:catLst")]
pub struct ColorTransformCategories {
  /// _
  #[sdk(child(qname = "dgm:CT_CTCategory/dgm:cat"))]
  pub dgm_cat: Vec<ColorTransformCategory>,
}
/// Style Label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CTStyleLabel/dgm:styleLbl")]
pub struct ColorTransformStyleLabel {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Fill Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:fillClrLst"))]
  pub fill_color_list: Option<FillColorList>,
  /// Line Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:linClrLst"))]
  pub line_color_list: Option<LineColorList>,
  /// Effect Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:effectClrLst"))]
  pub effect_color_list: Option<EffectColorList>,
  /// Text Line Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:txLinClrLst"))]
  pub text_line_color_list: Option<TextLineColorList>,
  /// Text Fill Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:txFillClrLst"))]
  pub text_fill_color_list: Option<TextFillColorList>,
  /// Text Effect Color List
  #[sdk(child(qname = "dgm:CT_Colors/dgm:txEffectClrLst"))]
  pub text_effect_color_list: Option<TextEffectColorList>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Pt/dgm:pt")]
pub struct Point {
  /// Model Identifier
  #[sdk(attr(qname = ":modelId"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub model_id: crate::simple_type::StringValue,
  /// Point Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<PointValues>,
  /// Connection Identifier
  #[sdk(attr(qname = ":cxnId"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub connection_id: Option<crate::simple_type::StringValue>,
  /// Property Set
  #[sdk(child(qname = "dgm:CT_ElemPropSet/dgm:prSet"))]
  pub property_set: Option<std::boxed::Box<PropertySet>>,
  /// Shape Properties
  #[sdk(child(qname = "a:CT_ShapeProperties/dgm:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Text Body
  #[sdk(child(qname = "a:CT_TextBody/dgm:t"))]
  pub text_body: Option<std::boxed::Box<TextBody>>,
  /// _
  #[sdk(child(qname = "a:CT_PtExtensionList/dgm:extLst"))]
  pub pt_extension_list: Option<PtExtensionList>,
}
/// Connection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Cxn/dgm:cxn")]
pub struct Connection {
  /// Model Identifier
  #[sdk(attr(qname = ":modelId"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub model_id: crate::simple_type::StringValue,
  /// Point Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#type: Option<ConnectionValues>,
  /// Source Identifier
  #[sdk(attr(qname = ":srcId"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub source_id: crate::simple_type::StringValue,
  /// Destination Identifier
  #[sdk(attr(qname = ":destId"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 2u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub destination_id: crate::simple_type::StringValue,
  /// Source Position
  #[sdk(attr(qname = ":srcOrd"))]
  pub source_position: crate::simple_type::UInt32Value,
  /// Destination Position
  #[sdk(attr(qname = ":destOrd"))]
  pub destination_position: crate::simple_type::UInt32Value,
  /// Parent Transition Identifier
  #[sdk(attr(qname = ":parTransId"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub parent_transition_id: Option<crate::simple_type::StringValue>,
  /// Sibling Transition Identifier
  #[sdk(attr(qname = ":sibTransId"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub sibling_transition_id: Option<crate::simple_type::StringValue>,
  /// Presentation Identifier
  #[sdk(attr(qname = ":presId"))]
  pub presentation_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Constraint.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Constraint/dgm:constr")]
pub struct Constraint {
  /// Constraint Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: ConstraintValues,
  /// For
  #[sdk(attr(qname = ":for"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#for: Option<ConstraintRelationshipValues>,
  /// For Name
  #[sdk(attr(qname = ":forName"))]
  pub for_name: Option<crate::simple_type::StringValue>,
  /// Data Point Type
  #[sdk(attr(qname = ":ptType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub point_type: Option<ElementValues>,
  /// Reference Type
  #[sdk(attr(qname = ":refType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub reference_type: Option<ConstraintValues>,
  /// Reference For
  #[sdk(attr(qname = ":refFor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub reference_for: Option<ConstraintRelationshipValues>,
  /// Reference For Name
  #[sdk(attr(qname = ":refForName"))]
  pub reference_for_name: Option<crate::simple_type::StringValue>,
  /// Reference Point Type
  #[sdk(attr(qname = ":refPtType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub reference_point_type: Option<ElementValues>,
  /// Operator
  #[sdk(attr(qname = ":op"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub operator: Option<BoolOperatorValues>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Factor
  #[sdk(attr(qname = ":fact"))]
  pub fact: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_NumericRule/dgm:rule")]
pub struct Rule {
  /// Constraint Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: ConstraintValues,
  /// For
  #[sdk(attr(qname = ":for"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r#for: Option<ConstraintRelationshipValues>,
  /// For Name
  #[sdk(attr(qname = ":forName"))]
  pub for_name: Option<crate::simple_type::StringValue>,
  /// Data Point Type
  #[sdk(attr(qname = ":ptType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub point_type: Option<ElementValues>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Factor
  #[sdk(attr(qname = ":fact"))]
  pub fact: Option<crate::simple_type::DoubleValue>,
  /// Max Value
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Shape Adjust.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Adj/dgm:adj")]
pub struct Adjust {
  /// Adjust Handle Index
  #[sdk(attr(qname = ":idx"))]
  #[sdk(number_range(source = 1u32, min = "1", min_inclusive = true, max_inclusive = false))]
  pub index: crate::simple_type::UInt32Value,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Shape Adjust List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_AdjLst/dgm:adjLst")]
pub struct AdjustList {
  /// _
  #[sdk(child(qname = "dgm:CT_Adj/dgm:adj"))]
  pub dgm_adj: Vec<Adjust>,
}
/// Parameter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Parameter/dgm:param")]
pub struct Parameter {
  /// Parameter Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: ParameterIdValues,
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["l", "ctr", "r", "none"]))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["t",
            "mid",
            "b",
            "none",
            "top",
            "center",
            "bottom"]
        )
    )]
  #[sdk(string_set(source = 2u32, union = 0u64, values = &["horz", "vert"]))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["t", "b", "l", "r"]))]
  #[sdk(
        string_set(source = 4u32, union = 0u64, values = &["none", "t", "b", "l", "r"])
    )]
  #[sdk(
        string_set(
            source = 5u32,
            union = 0u64,
            values = &["fromL",
            "fromR",
            "fromT",
            "fromB"]
        )
    )]
  #[sdk(
        string_set(
            source = 6u32,
            union = 0u64,
            values = &["none",
            "fromL",
            "fromR",
            "fromT",
            "fromB"]
        )
    )]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["node", "trans"]))]
  #[sdk(string_set(source = 8u32, union = 0u64, values = &["beg", "def", "end"]))]
  #[sdk(
        string_set(
            source = 9u32,
            union = 0u64,
            values = &["stra",
            "bend",
            "curve",
            "longCurve"]
        )
    )]
  #[sdk(string_set(source = 10u32, union = 0u64, values = &["auto", "arr", "noArr"]))]
  #[sdk(string_set(source = 11u32, union = 0u64, values = &["1D", "2D", "cust"]))]
  #[sdk(string_set(source = 12u32, union = 0u64, values = &["none", "alongPath"]))]
  #[sdk(string_set(source = 13u32, union = 0u64, values = &["none", "fNode"]))]
  #[sdk(string_set(source = 14u32, union = 0u64, values = &["l", "ctr", "r"]))]
  #[sdk(string_set(source = 15u32, union = 0u64, values = &["t", "mid", "b"]))]
  #[sdk(string_set(source = 16u32, union = 0u64, values = &["1D", "2D"]))]
  #[sdk(string_set(source = 17u32, union = 0u64, values = &["fromT", "fromB"]))]
  #[sdk(string_set(source = 18u32, union = 0u64, values = &["bef", "aft"]))]
  #[sdk(string_set(source = 19u32, union = 0u64, values = &["step", "stack"]))]
  #[sdk(string_set(source = 20u32, union = 0u64, values = &["horz", "vert"]))]
  #[sdk(string_set(source = 21u32, union = 0u64, values = &["none", "ctr"]))]
  #[sdk(string_set(source = 22u32, union = 0u64, values = &["t", "mid", "b"]))]
  #[sdk(string_set(source = 23u32, union = 0u64, values = &["l", "ctr", "r"]))]
  #[sdk(string_set(source = 24u32, union = 0u64, values = &["none", "upr", "grav"]))]
  #[sdk(string_set(source = 25u32, union = 0u64, values = &["tL", "tR", "bL", "bR"]))]
  #[sdk(string_set(source = 26u32, union = 0u64, values = &["row", "col"]))]
  #[sdk(string_set(source = 27u32, union = 0u64, values = &["revDir", "sameDir"]))]
  #[sdk(
        string_set(source = 28u32, union = 0u64, values = &["endCnv", "bal", "fixed"])
    )]
  #[sdk(string_set(source = 29u32, union = 0u64, values = &["ctr", "off"]))]
  #[sdk(
        string_set(
            source = 30u32,
            union = 0u64,
            values = &["tL",
            "tR",
            "tCtrCh",
            "tCtrDes",
            "bL",
            "bR",
            "bCtrCh",
            "bCtrDes",
            "lT",
            "lB",
            "lCtrCh",
            "lCtrDes",
            "rT",
            "rB",
            "rCtrCh",
            "rCtrDes"]
        )
    )]
  #[sdk(number_type(source = 31u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(number_type(source = 32u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(number_type(source = 33u32, union = 0u64, type_name = "xsd:boolean"))]
  #[sdk(string_format(source = 34u32, union = 0u64, kind = "token"))]
  #[sdk(
        string_set(
            source = 35u32,
            union = 0u64,
            values = &["auto",
            "bCtr",
            "ctr",
            "midL",
            "midR",
            "tCtr",
            "bL",
            "bR",
            "tL",
            "tR",
            "radial"]
        )
    )]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Algorithm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Algorithm/dgm:alg")]
pub struct Algorithm {
  /// Algorithm Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type: AlgorithmValues,
  /// Revision Number
  #[sdk(attr(qname = ":rev"))]
  pub revision: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "dgm:CT_Parameter/dgm:param"))]
  pub dgm_param: Vec<Parameter>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub dgm_ext_lst: Option<ExtensionList>,
}
/// Shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Shape/dgm:shape")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::DoubleValue>,
  /// Shape Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(
        string_set(
            source = 0u32,
            union = 0u64,
            values = &["line",
            "lineInv",
            "triangle",
            "rtTriangle",
            "rect",
            "diamond",
            "parallelogram",
            "trapezoid",
            "nonIsoscelesTrapezoid",
            "pentagon",
            "hexagon",
            "heptagon",
            "octagon",
            "decagon",
            "dodecagon",
            "star4",
            "star5",
            "star6",
            "star7",
            "star8",
            "star10",
            "star12",
            "star16",
            "star24",
            "star32",
            "roundRect",
            "round1Rect",
            "round2SameRect",
            "round2DiagRect",
            "snipRoundRect",
            "snip1Rect",
            "snip2SameRect",
            "snip2DiagRect",
            "plaque",
            "ellipse",
            "teardrop",
            "homePlate",
            "chevron",
            "pieWedge",
            "pie",
            "blockArc",
            "donut",
            "noSmoking",
            "rightArrow",
            "leftArrow",
            "upArrow",
            "downArrow",
            "stripedRightArrow",
            "notchedRightArrow",
            "bentUpArrow",
            "leftRightArrow",
            "upDownArrow",
            "leftUpArrow",
            "leftRightUpArrow",
            "quadArrow",
            "leftArrowCallout",
            "rightArrowCallout",
            "upArrowCallout",
            "downArrowCallout",
            "leftRightArrowCallout",
            "upDownArrowCallout",
            "quadArrowCallout",
            "bentArrow",
            "uturnArrow",
            "circularArrow",
            "leftCircularArrow",
            "leftRightCircularArrow",
            "curvedRightArrow",
            "curvedLeftArrow",
            "curvedUpArrow",
            "curvedDownArrow",
            "swooshArrow",
            "cube",
            "can",
            "lightningBolt",
            "heart",
            "sun",
            "moon",
            "smileyFace",
            "irregularSeal1",
            "irregularSeal2",
            "foldedCorner",
            "bevel",
            "frame",
            "halfFrame",
            "corner",
            "diagStripe",
            "chord",
            "arc",
            "leftBracket",
            "rightBracket",
            "leftBrace",
            "rightBrace",
            "bracketPair",
            "bracePair",
            "straightConnector1",
            "bentConnector2",
            "bentConnector3",
            "bentConnector4",
            "bentConnector5",
            "curvedConnector2",
            "curvedConnector3",
            "curvedConnector4",
            "curvedConnector5",
            "callout1",
            "callout2",
            "callout3",
            "accentCallout1",
            "accentCallout2",
            "accentCallout3",
            "borderCallout1",
            "borderCallout2",
            "borderCallout3",
            "accentBorderCallout1",
            "accentBorderCallout2",
            "accentBorderCallout3",
            "wedgeRectCallout",
            "wedgeRoundRectCallout",
            "wedgeEllipseCallout",
            "cloudCallout",
            "cloud",
            "ribbon",
            "ribbon2",
            "ellipseRibbon",
            "ellipseRibbon2",
            "leftRightRibbon",
            "verticalScroll",
            "horizontalScroll",
            "wave",
            "doubleWave",
            "plus",
            "flowChartProcess",
            "flowChartDecision",
            "flowChartInputOutput",
            "flowChartPredefinedProcess",
            "flowChartInternalStorage",
            "flowChartDocument",
            "flowChartMultidocument",
            "flowChartTerminator",
            "flowChartPreparation",
            "flowChartManualInput",
            "flowChartManualOperation",
            "flowChartConnector",
            "flowChartPunchedCard",
            "flowChartPunchedTape",
            "flowChartSummingJunction",
            "flowChartOr",
            "flowChartCollate",
            "flowChartSort",
            "flowChartExtract",
            "flowChartMerge",
            "flowChartOfflineStorage",
            "flowChartOnlineStorage",
            "flowChartMagneticTape",
            "flowChartMagneticDisk",
            "flowChartMagneticDrum",
            "flowChartDisplay",
            "flowChartDelay",
            "flowChartAlternateProcess",
            "flowChartOffpageConnector",
            "actionButtonBlank",
            "actionButtonHome",
            "actionButtonHelp",
            "actionButtonInformation",
            "actionButtonForwardNext",
            "actionButtonBackPrevious",
            "actionButtonEnd",
            "actionButtonBeginning",
            "actionButtonReturn",
            "actionButtonDocument",
            "actionButtonSound",
            "actionButtonMovie",
            "gear6",
            "gear9",
            "funnel",
            "mathPlus",
            "mathMinus",
            "mathMultiply",
            "mathDivide",
            "mathEqual",
            "mathNotEqual",
            "cornerTabs",
            "squareTabs",
            "plaqueTabs",
            "chartX",
            "chartStar",
            "chartPlus"]
        )
    )]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["none", "conn"]))]
  pub r#type: Option<crate::simple_type::StringValue>,
  /// Relationship to Image Part
  #[sdk(attr(qname = "r:blip"))]
  pub blip: Option<crate::simple_type::StringValue>,
  /// Z-Order Offset
  #[sdk(attr(qname = ":zOrderOff"))]
  pub z_order_offset: Option<crate::simple_type::Int32Value>,
  /// Hide Geometry
  #[sdk(attr(qname = ":hideGeom"))]
  pub hide_geometry: Option<crate::simple_type::BooleanValue>,
  /// Prevent Text Editing
  #[sdk(attr(qname = ":lkTxEntry"))]
  pub locked_text: Option<crate::simple_type::BooleanValue>,
  /// Image Placeholder
  #[sdk(attr(qname = ":blipPhldr"))]
  pub blip_placeholder: Option<crate::simple_type::BooleanValue>,
  /// Shape Adjust List
  #[sdk(child(qname = "dgm:CT_AdjLst/dgm:adjLst"))]
  pub adjust_list: Option<AdjustList>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Presentation Of.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_PresentationOf/dgm:presOf")]
pub struct PresentationOf {
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<crate::simple_type::ListValue<AxisValues>>,
  /// Data Point Type
  #[sdk(attr(qname = ":ptType"))]
  pub point_type: Option<crate::simple_type::ListValue<ElementValues>>,
  /// Hide Last Transition
  #[sdk(attr(qname = ":hideLastTrans"))]
  pub hide_last_trans: Option<crate::simple_type::ListValue<crate::simple_type::BooleanValue>>,
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  /// Count
  #[sdk(attr(qname = ":cnt"))]
  pub count: Option<crate::simple_type::ListValue<crate::simple_type::UInt32Value>>,
  /// Step
  #[sdk(attr(qname = ":step"))]
  pub step: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Constraint List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Constraints/dgm:constrLst")]
pub struct Constraints {
  /// _
  #[sdk(child(qname = "dgm:CT_Constraint/dgm:constr"))]
  pub dgm_constr: Vec<Constraint>,
}
/// Rule List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Rules/dgm:ruleLst")]
pub struct RuleList {
  /// _
  #[sdk(child(qname = "dgm:CT_NumericRule/dgm:rule"))]
  pub dgm_rule: Vec<Rule>,
}
/// Variable List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_LayoutVariablePropertySet/dgm:varLst")]
pub struct VariableList {
  /// Show Organization Chart User Interface
  #[sdk(child(qname = "dgm:CT_OrgChart/dgm:orgChart"))]
  pub organization_chart: Option<OrganizationChart>,
  /// Maximum Children
  #[sdk(child(qname = "dgm:CT_ChildMax/dgm:chMax"))]
  pub max_number_of_children: Option<MaxNumberOfChildren>,
  /// Preferred Number of Children
  #[sdk(child(qname = "dgm:CT_ChildPref/dgm:chPref"))]
  pub preferred_number_of_children: Option<PreferredNumberOfChildren>,
  /// Show Insert Bullet
  #[sdk(child(qname = "dgm:CT_BulletEnabled/dgm:bulletEnabled"))]
  pub bullet_enabled: Option<BulletEnabled>,
  /// Diagram Direction
  #[sdk(child(qname = "dgm:CT_Direction/dgm:dir"))]
  pub direction: Option<Direction>,
  /// Organization Chart Branch Style
  #[sdk(child(qname = "dgm:CT_HierBranchStyle/dgm:hierBranch"))]
  pub hierarchy_branch: Option<HierarchyBranch>,
  /// One by One Animation String
  #[sdk(child(qname = "dgm:CT_AnimOne/dgm:animOne"))]
  pub animate_one_by_one: Option<AnimateOneByOne>,
  /// Level Animation
  #[sdk(child(qname = "dgm:CT_AnimLvl/dgm:animLvl"))]
  pub animation_level: Option<AnimationLevel>,
  /// Shape Resize Style
  #[sdk(child(qname = "dgm:CT_ResizeHandles/dgm:resizeHandles"))]
  pub resize_handles: Option<ResizeHandles>,
}
/// Presentation Layout Variables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_LayoutVariablePropertySet/dgm:presLayoutVars")]
pub struct PresentationLayoutVariables {
  /// Show Organization Chart User Interface
  #[sdk(child(qname = "dgm:CT_OrgChart/dgm:orgChart"))]
  pub organization_chart: Option<OrganizationChart>,
  /// Maximum Children
  #[sdk(child(qname = "dgm:CT_ChildMax/dgm:chMax"))]
  pub max_number_of_children: Option<MaxNumberOfChildren>,
  /// Preferred Number of Children
  #[sdk(child(qname = "dgm:CT_ChildPref/dgm:chPref"))]
  pub preferred_number_of_children: Option<PreferredNumberOfChildren>,
  /// Show Insert Bullet
  #[sdk(child(qname = "dgm:CT_BulletEnabled/dgm:bulletEnabled"))]
  pub bullet_enabled: Option<BulletEnabled>,
  /// Diagram Direction
  #[sdk(child(qname = "dgm:CT_Direction/dgm:dir"))]
  pub direction: Option<Direction>,
  /// Organization Chart Branch Style
  #[sdk(child(qname = "dgm:CT_HierBranchStyle/dgm:hierBranch"))]
  pub hierarchy_branch: Option<HierarchyBranch>,
  /// One by One Animation String
  #[sdk(child(qname = "dgm:CT_AnimOne/dgm:animOne"))]
  pub animate_one_by_one: Option<AnimateOneByOne>,
  /// Level Animation
  #[sdk(child(qname = "dgm:CT_AnimLvl/dgm:animLvl"))]
  pub animation_level: Option<AnimationLevel>,
  /// Shape Resize Style
  #[sdk(child(qname = "dgm:CT_ResizeHandles/dgm:resizeHandles"))]
  pub resize_handles: Option<ResizeHandles>,
}
/// Defines the LayoutVariablePropertySetType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_LayoutVariablePropertySet/")]
pub struct LayoutVariablePropertySetType {
  #[sdk(choice(
    qname = "dgm:CT_OrgChart/dgm:orgChart",
    qname = "dgm:CT_ChildMax/dgm:chMax",
    qname = "dgm:CT_ChildPref/dgm:chPref",
    qname = "dgm:CT_BulletEnabled/dgm:bulletEnabled",
    qname = "dgm:CT_Direction/dgm:dir",
    qname = "dgm:CT_HierBranchStyle/dgm:hierBranch",
    qname = "dgm:CT_AnimOne/dgm:animOne",
    qname = "dgm:CT_AnimLvl/dgm:animLvl",
    qname = "dgm:CT_ResizeHandles/dgm:resizeHandles"
  ))]
  pub xml_children: Vec<LayoutVariablePropertySetTypeChoice>,
}
/// For Each.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ForEach/dgm:forEach")]
pub struct ForEach {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<crate::simple_type::ListValue<AxisValues>>,
  /// Data Point Type
  #[sdk(attr(qname = ":ptType"))]
  pub point_type: Option<crate::simple_type::ListValue<ElementValues>>,
  /// Hide Last Transition
  #[sdk(attr(qname = ":hideLastTrans"))]
  pub hide_last_trans: Option<crate::simple_type::ListValue<crate::simple_type::BooleanValue>>,
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  /// Count
  #[sdk(attr(qname = ":cnt"))]
  pub count: Option<crate::simple_type::ListValue<crate::simple_type::UInt32Value>>,
  /// Step
  #[sdk(attr(qname = ":step"))]
  pub step: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  #[sdk(choice(
    qname = "dgm:CT_Algorithm/dgm:alg",
    qname = "dgm:CT_Shape/dgm:shape",
    qname = "dgm:CT_PresentationOf/dgm:presOf",
    qname = "dgm:CT_Constraints/dgm:constrLst",
    qname = "dgm:CT_Rules/dgm:ruleLst",
    qname = "dgm:CT_ForEach/dgm:forEach",
    qname = "dgm:CT_LayoutNode/dgm:layoutNode",
    qname = "dgm:CT_Choose/dgm:choose",
    qname = "a:CT_OfficeArtExtensionList/dgm:extLst"
  ))]
  pub xml_children: Vec<ForEachChoice>,
}
/// Layout Node.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_LayoutNode/dgm:layoutNode")]
pub struct LayoutNode {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Style Label
  #[sdk(attr(qname = ":styleLbl"))]
  pub style_label: Option<crate::simple_type::StringValue>,
  /// Child Order
  #[sdk(attr(qname = ":chOrder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub child_order: Option<ChildOrderValues>,
  /// Move With
  #[sdk(attr(qname = ":moveWith"))]
  pub move_with: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "dgm:CT_Algorithm/dgm:alg",
    qname = "dgm:CT_Shape/dgm:shape",
    qname = "dgm:CT_PresentationOf/dgm:presOf",
    qname = "dgm:CT_Constraints/dgm:constrLst",
    qname = "dgm:CT_Rules/dgm:ruleLst",
    qname = "dgm:CT_LayoutVariablePropertySet/dgm:varLst",
    qname = "dgm:CT_ForEach/dgm:forEach",
    qname = "dgm:CT_LayoutNode/dgm:layoutNode",
    qname = "dgm:CT_Choose/dgm:choose",
    qname = "a:CT_OfficeArtExtensionList/dgm:extLst"
  ))]
  pub xml_children: Vec<LayoutNodeChoice>,
}
/// Choose Element.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Choose/dgm:choose")]
pub struct Choose {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "dgm:CT_When/dgm:if"))]
  pub dgm_if: Vec<DiagramChooseIf>,
  /// _
  #[sdk(child(qname = "dgm:CT_Otherwise/dgm:else"))]
  pub dgm_else: Option<DiagramChooseElse>,
}
/// If.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_When/dgm:if")]
pub struct DiagramChooseIf {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<crate::simple_type::ListValue<AxisValues>>,
  /// Data Point Type
  #[sdk(attr(qname = ":ptType"))]
  pub point_type: Option<crate::simple_type::ListValue<ElementValues>>,
  /// Hide Last Transition
  #[sdk(attr(qname = ":hideLastTrans"))]
  pub hide_last_trans: Option<crate::simple_type::ListValue<crate::simple_type::BooleanValue>>,
  /// Start
  #[sdk(attr(qname = ":st"))]
  pub start: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  /// Count
  #[sdk(attr(qname = ":cnt"))]
  pub count: Option<crate::simple_type::ListValue<crate::simple_type::UInt32Value>>,
  /// Step
  #[sdk(attr(qname = ":step"))]
  pub step: Option<crate::simple_type::ListValue<crate::simple_type::Int32Value>>,
  /// Function
  #[sdk(attr(qname = ":func"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub function: FunctionValues,
  /// Argument
  #[sdk(attr(qname = ":arg"))]
  #[sdk(
        string_set(
            source = 0u32,
            values = &["none",
            "orgChart",
            "chMax",
            "chPref",
            "bulEnabled",
            "dir",
            "hierBranch",
            "animOne",
            "animLvl",
            "resizeHandles"]
        )
    )]
  pub argument: Option<crate::simple_type::StringValue>,
  /// Operator
  #[sdk(attr(qname = ":op"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub operator: FunctionOperatorValues,
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_type(source = 1u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(number_type(source = 2u32, union = 0u64, type_name = "xsd:boolean"))]
  #[sdk(string_set(source = 3u32, union = 0u64, values = &["norm", "rev"]))]
  #[sdk(
        string_set(
            source = 4u32,
            union = 0u64,
            values = &["l",
            "r",
            "hang",
            "std",
            "init"]
        )
    )]
  #[sdk(string_set(source = 5u32, union = 0u64, values = &["none", "one", "branch"]))]
  #[sdk(string_set(source = 6u32, union = 0u64, values = &["none", "lvl", "ctr"]))]
  #[sdk(string_set(source = 7u32, union = 0u64, values = &["exact", "rel"]))]
  pub val: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "dgm:CT_Algorithm/dgm:alg",
    qname = "dgm:CT_Shape/dgm:shape",
    qname = "dgm:CT_PresentationOf/dgm:presOf",
    qname = "dgm:CT_Constraints/dgm:constrLst",
    qname = "dgm:CT_Rules/dgm:ruleLst",
    qname = "dgm:CT_ForEach/dgm:forEach",
    qname = "dgm:CT_LayoutNode/dgm:layoutNode",
    qname = "dgm:CT_Choose/dgm:choose",
    qname = "a:CT_OfficeArtExtensionList/dgm:extLst"
  ))]
  pub xml_children: Vec<DiagramChooseIfChoice>,
}
/// Else.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Otherwise/dgm:else")]
pub struct DiagramChooseElse {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "dgm:CT_Algorithm/dgm:alg",
    qname = "dgm:CT_Shape/dgm:shape",
    qname = "dgm:CT_PresentationOf/dgm:presOf",
    qname = "dgm:CT_Constraints/dgm:constrLst",
    qname = "dgm:CT_Rules/dgm:ruleLst",
    qname = "dgm:CT_ForEach/dgm:forEach",
    qname = "dgm:CT_LayoutNode/dgm:layoutNode",
    qname = "dgm:CT_Choose/dgm:choose",
    qname = "a:CT_OfficeArtExtensionList/dgm:extLst"
  ))]
  pub xml_children: Vec<DiagramChooseElseChoice>,
}
/// Data Model.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DataModel/dgm:dataModel")]
pub struct DataModel {
  /// Point List
  #[sdk(child(qname = "dgm:CT_PtList/dgm:ptLst"))]
  pub point_list: std::boxed::Box<PointList>,
  /// Connection List
  #[sdk(child(qname = "dgm:CT_CxnList/dgm:cxnLst"))]
  pub connection_list: Option<ConnectionList>,
  /// Background Formatting
  #[sdk(child(qname = "a:CT_BackgroundFormatting/dgm:bg"))]
  pub background: Option<std::boxed::Box<Background>>,
  /// Whole E2O Formatting
  #[sdk(child(qname = "a:CT_WholeE2oFormatting/dgm:whole"))]
  pub whole: Option<std::boxed::Box<Whole>>,
  /// _
  #[sdk(child(qname = "a:CT_DataModelExtensionList/dgm:extLst"))]
  pub data_model_extension_list: Option<DataModelExtensionList>,
}
/// Category.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Category/dgm:cat")]
pub struct Category {
  /// Category Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub r#type: crate::simple_type::StringValue,
  /// Priority
  #[sdk(attr(qname = ":pri"))]
  pub priority: crate::simple_type::UInt32Value,
}
/// Title.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Name/dgm:title")]
pub struct Title {
  /// Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Description.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Description/dgm:desc")]
pub struct Description {
  /// Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Category List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Categories/dgm:catLst")]
pub struct CategoryList {
  /// _
  #[sdk(child(qname = "dgm:CT_Category/dgm:cat"))]
  pub dgm_cat: Vec<Category>,
}
/// Shape Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeStyle/dgm:style")]
pub struct Style {
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
  /// Font Reference
  #[sdk(child(qname = "a:CT_FontReference/a:fontRef"))]
  pub font_reference:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference>,
}
/// Show Organization Chart User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_OrgChart/dgm:orgChart")]
pub struct OrganizationChart {
  /// Show Organization Chart User Interface Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Maximum Children.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ChildMax/dgm:chMax")]
pub struct MaxNumberOfChildren {
  /// Maximum Children Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 0u32, min = "-1", min_inclusive = true, max_inclusive = false))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// Preferred Number of Children.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ChildPref/dgm:chPref")]
pub struct PreferredNumberOfChildren {
  /// Preferred Number of CHildren Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 0u32, min = "-1", min_inclusive = true, max_inclusive = false))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// Show Insert Bullet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_BulletEnabled/dgm:bulletEnabled")]
pub struct BulletEnabled {
  /// Show Insert Bullet Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Diagram Direction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_Direction/dgm:dir")]
pub struct Direction {
  /// Diagram Direction Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<DirectionValues>,
}
/// Organization Chart Branch Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_HierBranchStyle/dgm:hierBranch")]
pub struct HierarchyBranch {
  /// Organization Chart Branch Style Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<HierarchyBranchStyleValues>,
}
/// One by One Animation String.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_AnimOne/dgm:animOne")]
pub struct AnimateOneByOne {
  /// One By One Animation Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<AnimateOneByOneValues>,
}
/// Level Animation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_AnimLvl/dgm:animLvl")]
pub struct AnimationLevel {
  /// Level Animation Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<AnimationLevelStringValues>,
}
/// Shape Resize Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ResizeHandles/dgm:resizeHandles")]
pub struct ResizeHandles {
  /// Shape Resize Style Type
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub val: Option<ResizeHandlesStringValues>,
}
/// Category.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SDCategory/dgm:cat")]
pub struct StyleDisplayCategory {
  /// Category Type
  #[sdk(attr(qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "uri"))]
  pub r#type: crate::simple_type::StringValue,
  /// Priority
  #[sdk(attr(qname = ":pri"))]
  pub priority: crate::simple_type::UInt32Value,
}
/// 3-D Scene.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Scene3D/dgm:scene3d")]
pub struct Scene3D {
  /// Camera
  #[sdk(child(qname = "a:CT_Camera/a:camera"))]
  pub camera:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Camera>,
  /// Light Rig
  #[sdk(child(qname = "a:CT_LightRig/a:lightRig"))]
  pub light_rig:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LightRig>,
  /// Backdrop Plane
  #[sdk(child(qname = "a:CT_Backdrop/a:backdrop"))]
  pub backdrop: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Backdrop>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// 3-D Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Shape3D/dgm:sp3d")]
pub struct Shape3D {
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
  pub preset_material: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetMaterialTypeValues,
  >,
  /// Top Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelT"))]
  pub bevel_top: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelTop>,
  /// Bottom Bevel
  #[sdk(child(qname = "a:CT_Bevel/a:bevelB"))]
  pub bevel_bottom:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelBottom>,
  /// Extrusion Color
  #[sdk(child(qname = "a:CT_Color/a:extrusionClr"))]
  pub extrusion_color: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtrusionColor>,
  >,
  /// Contour Color
  #[sdk(child(qname = "a:CT_Color/a:contourClr"))]
  pub contour_color: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ContourColor>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Text Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_TextProps/dgm:txPr")]
pub struct TextProperties {
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub xml_children: Option<TextPropertiesChoice>,
}
/// Title.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SDName/dgm:title")]
pub struct StyleDefinitionTitle {
  /// Natural Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Description Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Style Label Description.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SDDescription/dgm:desc")]
pub struct StyleLabelDescription {
  /// Natural Language
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// Description Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Category List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SDCategories/dgm:catLst")]
pub struct StyleDisplayCategories {
  /// _
  #[sdk(child(qname = "dgm:CT_SDCategory/dgm:cat"))]
  pub dgm_cat: Vec<StyleDisplayCategory>,
}
/// Style Label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_StyleLabel/dgm:styleLbl")]
pub struct StyleLabel {
  /// Style Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// 3-D Scene
  #[sdk(child(qname = "a:CT_Scene3D/dgm:scene3d"))]
  pub scene3_d: Option<std::boxed::Box<Scene3D>>,
  /// 3-D Shape Properties
  #[sdk(child(qname = "a:CT_Shape3D/dgm:sp3d"))]
  pub shape3_d: Option<std::boxed::Box<Shape3D>>,
  /// Text Properties
  #[sdk(child(qname = "dgm:CT_TextProps/dgm:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/dgm:style"))]
  pub style: Option<std::boxed::Box<Style>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Point List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_PtList/dgm:ptLst")]
pub struct PointList {
  /// _
  #[sdk(child(qname = "dgm:CT_Pt/dgm:pt"))]
  pub dgm_pt: Vec<Point>,
}
/// Connection List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_CxnList/dgm:cxnLst")]
pub struct ConnectionList {
  /// _
  #[sdk(child(qname = "dgm:CT_Cxn/dgm:cxn"))]
  pub dgm_cxn: Vec<Connection>,
}
/// Background Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_BackgroundFormatting/dgm:bg")]
pub struct Background {
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub background_choice1: Option<BackgroundChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub background_choice2: Option<BackgroundChoice2>,
}
/// Whole E2O Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_WholeE2oFormatting/dgm:whole")]
pub struct Whole {
  /// Outline
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub whole_choice: Option<WholeChoice>,
}
/// Defines the DataModelExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_DataModelExtensionList/dgm:extLst")]
pub struct DataModelExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_DataModelExtension/a:ext"))]
  pub a_ext:
    Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DataModelExtension>,
}
/// Property Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_ElemPropSet/dgm:prSet")]
pub struct PropertySet {
  /// Presentation Element Identifier
  #[sdk(attr(qname = ":presAssocID"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:int"))]
  #[sdk(pattern(
    source = 1u32,
    union = 0u64,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  pub presentation_element_id: Option<crate::simple_type::StringValue>,
  /// Presentation Name
  #[sdk(attr(qname = ":presName"))]
  pub presentation_name: Option<crate::simple_type::StringValue>,
  /// Presentation Style Label
  #[sdk(attr(qname = ":presStyleLbl"))]
  pub presentation_style_label: Option<crate::simple_type::StringValue>,
  /// Presentation Style Index
  #[sdk(attr(qname = ":presStyleIdx"))]
  pub presentation_style_index: Option<crate::simple_type::Int32Value>,
  /// Presentation Style Count
  #[sdk(attr(qname = ":presStyleCnt"))]
  pub presentation_style_count: Option<crate::simple_type::Int32Value>,
  /// Current Diagram Type
  #[sdk(attr(qname = ":loTypeId"))]
  pub layout_type_id: Option<crate::simple_type::StringValue>,
  /// Current Diagram Category
  #[sdk(attr(qname = ":loCatId"))]
  pub layout_category_id: Option<crate::simple_type::StringValue>,
  /// Current Style Type
  #[sdk(attr(qname = ":qsTypeId"))]
  pub quick_style_type_id: Option<crate::simple_type::StringValue>,
  /// Current Style Category
  #[sdk(attr(qname = ":qsCatId"))]
  pub quick_style_category_id: Option<crate::simple_type::StringValue>,
  /// Color Transform Type Identifier
  #[sdk(attr(qname = ":csTypeId"))]
  pub color_type: Option<crate::simple_type::StringValue>,
  /// Color Transform Category
  #[sdk(attr(qname = ":csCatId"))]
  pub color_category_id: Option<crate::simple_type::StringValue>,
  /// Coherent 3D Behavior
  #[sdk(attr(qname = ":coherent3DOff"))]
  pub coherent3_d: Option<crate::simple_type::BooleanValue>,
  /// Placeholder Text
  #[sdk(attr(qname = ":phldrT"))]
  pub placeholder_text: Option<crate::simple_type::StringValue>,
  /// Placeholder
  #[sdk(attr(qname = ":phldr"))]
  pub placeholder: Option<crate::simple_type::BooleanValue>,
  /// Custom Rotation
  #[sdk(attr(qname = ":custAng"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Custom Vertical Flip
  #[sdk(attr(qname = ":custFlipVert"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  /// Custom Horizontal Flip
  #[sdk(attr(qname = ":custFlipHor"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Fixed Width Override
  #[sdk(attr(qname = ":custSzX"))]
  pub fixed_width_override: Option<crate::simple_type::Int32Value>,
  /// Fixed Height Override
  #[sdk(attr(qname = ":custSzY"))]
  pub fixed_height_override: Option<crate::simple_type::Int32Value>,
  /// Width Scale
  #[sdk(attr(qname = ":custScaleX"))]
  pub width_scale: Option<crate::simple_type::Int32Value>,
  /// Height Scale
  #[sdk(attr(qname = ":custScaleY"))]
  pub height_scale: Option<crate::simple_type::Int32Value>,
  /// Text Changed
  #[sdk(attr(qname = ":custT"))]
  pub text_changed: Option<crate::simple_type::BooleanValue>,
  /// Custom Factor Width
  #[sdk(attr(qname = ":custLinFactX"))]
  pub factor_width: Option<crate::simple_type::Int32Value>,
  /// Custom Factor Height
  #[sdk(attr(qname = ":custLinFactY"))]
  pub factor_height: Option<crate::simple_type::Int32Value>,
  /// Neighbor Offset Width
  #[sdk(attr(qname = ":custLinFactNeighborX"))]
  pub neighbor_offset_width: Option<crate::simple_type::Int32Value>,
  /// Neighbor Offset Height
  #[sdk(attr(qname = ":custLinFactNeighborY"))]
  pub neighbor_offset_height: Option<crate::simple_type::Int32Value>,
  /// Radius Scale
  #[sdk(attr(qname = ":custRadScaleRad"))]
  pub radius_scale: Option<crate::simple_type::Int32Value>,
  /// Include Angle Scale
  #[sdk(attr(qname = ":custRadScaleInc"))]
  pub include_angle_scale: Option<crate::simple_type::Int32Value>,
  /// Presentation Layout Variables
  #[sdk(child(qname = "dgm:CT_LayoutVariablePropertySet/dgm:presLayoutVars"))]
  pub presentation_layout_variables: Option<std::boxed::Box<PresentationLayoutVariables>>,
  /// Shape Style
  #[sdk(child(qname = "a:CT_ShapeStyle/dgm:style"))]
  pub style: Option<std::boxed::Box<Style>>,
}
/// Shape Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/dgm:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
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
/// Text Body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/dgm:t")]
pub struct TextBody {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Defines the PtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_PtExtensionList/dgm:extLst")]
pub struct PtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "a:CT_PtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PtExtension>,
}
/// Defines the DiagramDefinitionExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DiagramDefinitionExtension/dgm:ext")]
pub struct DiagramDefinitionExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst",
    qname = "a:CT_TextListStyle/dgm1612:lstStyle",
    any
  ))]
  pub xml_children: Option<DiagramDefinitionExtensionChoice>,
}
/// Defines the SampleData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SampleData/dgm:sampData")]
pub struct SampleData {
  /// Use Default
  #[sdk(attr(qname = ":useDef"))]
  pub use_default: Option<crate::simple_type::BooleanValue>,
  /// Data Model
  #[sdk(child(qname = "dgm:CT_DataModel/dgm:dataModel"))]
  pub data_model: Option<std::boxed::Box<DataModel>>,
}
/// Defines the StyleData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SampleData/dgm:styleData")]
pub struct StyleData {
  /// Use Default
  #[sdk(attr(qname = ":useDef"))]
  pub use_default: Option<crate::simple_type::BooleanValue>,
  /// Data Model
  #[sdk(child(qname = "dgm:CT_DataModel/dgm:dataModel"))]
  pub data_model: Option<std::boxed::Box<DataModel>>,
}
/// Defines the ColorData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SampleData/dgm:clrData")]
pub struct ColorData {
  /// Use Default
  #[sdk(attr(qname = ":useDef"))]
  pub use_default: Option<crate::simple_type::BooleanValue>,
  /// Data Model
  #[sdk(child(qname = "dgm:CT_DataModel/dgm:dataModel"))]
  pub data_model: Option<std::boxed::Box<DataModel>>,
}
/// Defines the SampleDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_SampleData/")]
pub struct SampleDataType {
  /// Use Default
  #[sdk(attr(qname = ":useDef"))]
  pub use_default: Option<crate::simple_type::BooleanValue>,
  /// Data Model
  #[sdk(child(qname = "dgm:CT_DataModel/dgm:dataModel"))]
  pub data_model: Vec<DataModel>,
}
/// List of extensions to the CT_DiagramDefintions type..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dgm:CT_DiagramDefinitionExtensionList/dgm:extLst")]
pub struct DiagramDefinitionExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(qname = "dgm:CT_DiagramDefinitionExtension/dgm:ext"))]
  pub dgm_ext: Vec<DiagramDefinitionExtension>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextLineColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextFillColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextEffectColorListChoice {
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
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorsTypeChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LayoutVariablePropertySetTypeChoice {
  /// Show Organization Chart User Interface
  #[sdk(child(qname = "dgm:CT_OrgChart/dgm:orgChart"))]
  DgmOrgChart(std::boxed::Box<OrganizationChart>),
  /// Maximum Children
  #[sdk(child(qname = "dgm:CT_ChildMax/dgm:chMax"))]
  DgmChMax(std::boxed::Box<MaxNumberOfChildren>),
  /// Preferred Number of Children
  #[sdk(child(qname = "dgm:CT_ChildPref/dgm:chPref"))]
  DgmChPref(std::boxed::Box<PreferredNumberOfChildren>),
  /// Show Insert Bullet
  #[sdk(child(qname = "dgm:CT_BulletEnabled/dgm:bulletEnabled"))]
  DgmBulletEnabled(std::boxed::Box<BulletEnabled>),
  /// Diagram Direction
  #[sdk(child(qname = "dgm:CT_Direction/dgm:dir"))]
  DgmDir(std::boxed::Box<Direction>),
  /// Organization Chart Branch Style
  #[sdk(child(qname = "dgm:CT_HierBranchStyle/dgm:hierBranch"))]
  DgmHierBranch(std::boxed::Box<HierarchyBranch>),
  /// One by One Animation String
  #[sdk(child(qname = "dgm:CT_AnimOne/dgm:animOne"))]
  DgmAnimOne(std::boxed::Box<AnimateOneByOne>),
  /// Level Animation
  #[sdk(child(qname = "dgm:CT_AnimLvl/dgm:animLvl"))]
  DgmAnimLvl(std::boxed::Box<AnimationLevel>),
  /// Shape Resize Style
  #[sdk(child(qname = "dgm:CT_ResizeHandles/dgm:resizeHandles"))]
  DgmResizeHandles(std::boxed::Box<ResizeHandles>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ForEachChoice {
  /// Algorithm.
  #[sdk(child(qname = "dgm:CT_Algorithm/dgm:alg"))]
  DgmAlg(std::boxed::Box<Algorithm>),
  /// Shape.
  #[sdk(child(qname = "dgm:CT_Shape/dgm:shape"))]
  DgmShape(std::boxed::Box<Shape>),
  /// Presentation Of.
  #[sdk(child(qname = "dgm:CT_PresentationOf/dgm:presOf"))]
  DgmPresOf(std::boxed::Box<PresentationOf>),
  /// Constraint List.
  #[sdk(child(qname = "dgm:CT_Constraints/dgm:constrLst"))]
  DgmConstrLst(std::boxed::Box<Constraints>),
  /// Rule List.
  #[sdk(child(qname = "dgm:CT_Rules/dgm:ruleLst"))]
  DgmRuleLst(std::boxed::Box<RuleList>),
  /// For Each.
  #[sdk(child(qname = "dgm:CT_ForEach/dgm:forEach"))]
  DgmForEach(std::boxed::Box<ForEach>),
  /// Layout Node.
  #[sdk(child(qname = "dgm:CT_LayoutNode/dgm:layoutNode"))]
  DgmLayoutNode(std::boxed::Box<LayoutNode>),
  /// Choose Element.
  #[sdk(child(qname = "dgm:CT_Choose/dgm:choose"))]
  DgmChoose(std::boxed::Box<Choose>),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  DgmExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LayoutNodeChoice {
  /// Algorithm.
  #[sdk(child(qname = "dgm:CT_Algorithm/dgm:alg"))]
  DgmAlg(std::boxed::Box<Algorithm>),
  /// Shape.
  #[sdk(child(qname = "dgm:CT_Shape/dgm:shape"))]
  DgmShape(std::boxed::Box<Shape>),
  /// Presentation Of.
  #[sdk(child(qname = "dgm:CT_PresentationOf/dgm:presOf"))]
  DgmPresOf(std::boxed::Box<PresentationOf>),
  /// Constraint List.
  #[sdk(child(qname = "dgm:CT_Constraints/dgm:constrLst"))]
  DgmConstrLst(std::boxed::Box<Constraints>),
  /// Rule List.
  #[sdk(child(qname = "dgm:CT_Rules/dgm:ruleLst"))]
  DgmRuleLst(std::boxed::Box<RuleList>),
  /// Variable List.
  #[sdk(child(qname = "dgm:CT_LayoutVariablePropertySet/dgm:varLst"))]
  DgmVarLst(std::boxed::Box<VariableList>),
  /// For Each.
  #[sdk(child(qname = "dgm:CT_ForEach/dgm:forEach"))]
  DgmForEach(std::boxed::Box<ForEach>),
  /// Layout Node.
  #[sdk(child(qname = "dgm:CT_LayoutNode/dgm:layoutNode"))]
  DgmLayoutNode(std::boxed::Box<LayoutNode>),
  /// Choose Element.
  #[sdk(child(qname = "dgm:CT_Choose/dgm:choose"))]
  DgmChoose(std::boxed::Box<Choose>),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  DgmExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DiagramChooseIfChoice {
  /// Algorithm.
  #[sdk(child(qname = "dgm:CT_Algorithm/dgm:alg"))]
  DgmAlg(std::boxed::Box<Algorithm>),
  /// Shape.
  #[sdk(child(qname = "dgm:CT_Shape/dgm:shape"))]
  DgmShape(std::boxed::Box<Shape>),
  /// Presentation Of.
  #[sdk(child(qname = "dgm:CT_PresentationOf/dgm:presOf"))]
  DgmPresOf(std::boxed::Box<PresentationOf>),
  /// Constraint List.
  #[sdk(child(qname = "dgm:CT_Constraints/dgm:constrLst"))]
  DgmConstrLst(std::boxed::Box<Constraints>),
  /// Rule List.
  #[sdk(child(qname = "dgm:CT_Rules/dgm:ruleLst"))]
  DgmRuleLst(std::boxed::Box<RuleList>),
  /// For Each.
  #[sdk(child(qname = "dgm:CT_ForEach/dgm:forEach"))]
  DgmForEach(std::boxed::Box<ForEach>),
  /// Layout Node.
  #[sdk(child(qname = "dgm:CT_LayoutNode/dgm:layoutNode"))]
  DgmLayoutNode(std::boxed::Box<LayoutNode>),
  /// Choose Element.
  #[sdk(child(qname = "dgm:CT_Choose/dgm:choose"))]
  DgmChoose(std::boxed::Box<Choose>),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  DgmExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DiagramChooseElseChoice {
  /// Algorithm.
  #[sdk(child(qname = "dgm:CT_Algorithm/dgm:alg"))]
  DgmAlg(std::boxed::Box<Algorithm>),
  /// Shape.
  #[sdk(child(qname = "dgm:CT_Shape/dgm:shape"))]
  DgmShape(std::boxed::Box<Shape>),
  /// Presentation Of.
  #[sdk(child(qname = "dgm:CT_PresentationOf/dgm:presOf"))]
  DgmPresOf(std::boxed::Box<PresentationOf>),
  /// Constraint List.
  #[sdk(child(qname = "dgm:CT_Constraints/dgm:constrLst"))]
  DgmConstrLst(std::boxed::Box<Constraints>),
  /// Rule List.
  #[sdk(child(qname = "dgm:CT_Rules/dgm:ruleLst"))]
  DgmRuleLst(std::boxed::Box<RuleList>),
  /// For Each.
  #[sdk(child(qname = "dgm:CT_ForEach/dgm:forEach"))]
  DgmForEach(std::boxed::Box<ForEach>),
  /// Layout Node.
  #[sdk(child(qname = "dgm:CT_LayoutNode/dgm:layoutNode"))]
  DgmLayoutNode(std::boxed::Box<LayoutNode>),
  /// Choose Element.
  #[sdk(child(qname = "dgm:CT_Choose/dgm:choose"))]
  DgmChoose(std::boxed::Box<Choose>),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/dgm:extLst"))]
  DgmExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextPropertiesChoice {
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  ),
  /// No text in 3D scene.
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundChoice {
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
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BackgroundChoice2 {
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
pub enum WholeChoice {
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
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
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
pub enum DiagramDefinitionExtensionChoice {
  #[sdk(child(
    office2019,
    qname = "dgm1611:CT_NumberDiagramInfoList/dgm1611:autoBuNodeInfoLst"
  ))]
  Dgm1611AutoBuNodeInfoLst(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2016_11_diagram::NumberDiagramInfoList,
    >,
  ),
  #[sdk(child(office2019, qname = "a:CT_TextListStyle/dgm1612:lstStyle"))]
  Dgm1612LstStyle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2016_12_diagram::TextListStyleType,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
