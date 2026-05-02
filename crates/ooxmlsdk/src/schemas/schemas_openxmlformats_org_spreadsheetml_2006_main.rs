//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FilterOperatorValues {
  #[sdk(rename = "equal")]
  #[default]
  Equal,
  #[sdk(rename = "lessThan")]
  LessThan,
  #[sdk(rename = "lessThanOrEqual")]
  LessThanOrEqual,
  #[sdk(rename = "notEqual")]
  NotEqual,
  #[sdk(rename = "greaterThanOrEqual")]
  GreaterThanOrEqual,
  #[sdk(rename = "greaterThan")]
  GreaterThan,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DynamicFilterValues {
  #[sdk(rename = "null")]
  #[default]
  Null,
  #[sdk(rename = "aboveAverage")]
  AboveAverage,
  #[sdk(rename = "belowAverage")]
  BelowAverage,
  #[sdk(rename = "tomorrow")]
  Tomorrow,
  #[sdk(rename = "today")]
  Today,
  #[sdk(rename = "yesterday")]
  Yesterday,
  #[sdk(rename = "nextWeek")]
  NextWeek,
  #[sdk(rename = "thisWeek")]
  ThisWeek,
  #[sdk(rename = "lastWeek")]
  LastWeek,
  #[sdk(rename = "nextMonth")]
  NextMonth,
  #[sdk(rename = "thisMonth")]
  ThisMonth,
  #[sdk(rename = "lastMonth")]
  LastMonth,
  #[sdk(rename = "nextQuarter")]
  NextQuarter,
  #[sdk(rename = "thisQuarter")]
  ThisQuarter,
  #[sdk(rename = "lastQuarter")]
  LastQuarter,
  #[sdk(rename = "nextYear")]
  NextYear,
  #[sdk(rename = "thisYear")]
  ThisYear,
  #[sdk(rename = "lastYear")]
  LastYear,
  #[sdk(rename = "yearToDate")]
  YearToDate,
  #[sdk(rename = "Q1")]
  Quarter1,
  #[sdk(rename = "Q2")]
  Quarter2,
  #[sdk(rename = "Q3")]
  Quarter3,
  #[sdk(rename = "Q4")]
  Quarter4,
  #[sdk(rename = "M1")]
  January,
  #[sdk(rename = "M2")]
  February,
  #[sdk(rename = "M3")]
  March,
  #[sdk(rename = "M4")]
  April,
  #[sdk(rename = "M5")]
  May,
  #[sdk(rename = "M6")]
  June,
  #[sdk(rename = "M7")]
  July,
  #[sdk(rename = "M8")]
  August,
  #[sdk(rename = "M9")]
  September,
  #[sdk(rename = "M10")]
  October,
  #[sdk(rename = "M11")]
  November,
  #[sdk(rename = "M12")]
  December,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum IconSetValues {
  #[sdk(rename = "3Arrows")]
  #[default]
  ThreeArrows,
  #[sdk(rename = "3ArrowsGray")]
  ThreeArrowsGray,
  #[sdk(rename = "3Flags")]
  ThreeFlags,
  #[sdk(rename = "3TrafficLights1")]
  ThreeTrafficLights1,
  #[sdk(rename = "3TrafficLights2")]
  ThreeTrafficLights2,
  #[sdk(rename = "3Signs")]
  ThreeSigns,
  #[sdk(rename = "3Symbols")]
  ThreeSymbols,
  #[sdk(rename = "3Symbols2")]
  ThreeSymbols2,
  #[sdk(rename = "4Arrows")]
  FourArrows,
  #[sdk(rename = "4ArrowsGray")]
  FourArrowsGray,
  #[sdk(rename = "4RedToBlack")]
  FourRedToBlack,
  #[sdk(rename = "4Rating")]
  FourRating,
  #[sdk(rename = "4TrafficLights")]
  FourTrafficLights,
  #[sdk(rename = "5Arrows")]
  FiveArrows,
  #[sdk(rename = "5ArrowsGray")]
  FiveArrowsGray,
  #[sdk(rename = "5Rating")]
  FiveRating,
  #[sdk(rename = "5Quarters")]
  FiveQuarters,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SortByValues {
  #[sdk(rename = "value")]
  #[default]
  Value,
  #[sdk(rename = "cellColor")]
  CellColor,
  #[sdk(rename = "fontColor")]
  FontColor,
  #[sdk(rename = "icon")]
  Icon,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SortMethodValues {
  #[sdk(rename = "stroke")]
  #[default]
  Stroke,
  #[sdk(rename = "pinYin")]
  PinYin,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CalendarValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "gregorian")]
  Gregorian,
  #[sdk(rename = "gregorianUs")]
  GregorianUs,
  #[sdk(rename = "japan")]
  Japan,
  #[sdk(rename = "taiwan")]
  Taiwan,
  #[sdk(rename = "korea")]
  Korea,
  #[sdk(rename = "hijri")]
  Hijri,
  #[sdk(rename = "thai")]
  Thai,
  #[sdk(rename = "hebrew")]
  Hebrew,
  #[sdk(rename = "gregorianMeFrench")]
  GregorianMiddleEastFrench,
  #[sdk(rename = "gregorianArabic")]
  GregorianArabic,
  #[sdk(rename = "gregorianXlitEnglish")]
  GregorianTransliteratedEnglish,
  #[sdk(rename = "gregorianXlitFrench")]
  GregorianTransliteratedFrench,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DateTimeGroupingValues {
  #[sdk(rename = "year")]
  #[default]
  Year,
  #[sdk(rename = "month")]
  Month,
  #[sdk(rename = "day")]
  Day,
  #[sdk(rename = "hour")]
  Hour,
  #[sdk(rename = "minute")]
  Minute,
  #[sdk(rename = "second")]
  Second,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HtmlFormattingValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "rtf")]
  HonorRichText,
  #[sdk(rename = "all")]
  All,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ParameterValues {
  #[sdk(rename = "prompt")]
  #[default]
  Prompt,
  #[sdk(rename = "value")]
  Value,
  #[sdk(rename = "cell")]
  Cell,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FileTypeValues {
  #[sdk(rename = "mac")]
  #[default]
  Mac,
  #[sdk(rename = "win")]
  Win,
  #[sdk(rename = "dos")]
  Dos,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum QualifierValues {
  #[sdk(rename = "doubleQuote")]
  #[default]
  DoubleQuote,
  #[sdk(rename = "singleQuote")]
  SingleQuote,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExternalConnectionValues {
  #[sdk(rename = "general")]
  #[default]
  General,
  #[sdk(rename = "text")]
  Text,
  #[sdk(rename = "MDY")]
  MonthDayYear,
  #[sdk(rename = "DMY")]
  DayMonthYear,
  #[sdk(rename = "YMD")]
  YearMonthDay,
  #[sdk(rename = "MYD")]
  MonthYearDay,
  #[sdk(rename = "DYM")]
  DayYearMonth,
  #[sdk(rename = "YDM")]
  YearDayMonth,
  #[sdk(rename = "skip")]
  Skip,
  #[sdk(rename = "EMD")]
  Emd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CredentialsMethodValues {
  #[sdk(rename = "integrated")]
  #[default]
  Integrated,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "stored")]
  Stored,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SourceValues {
  #[sdk(rename = "worksheet")]
  #[default]
  Worksheet,
  #[sdk(rename = "external")]
  External,
  #[sdk(rename = "consolidation")]
  Consolidation,
  #[sdk(rename = "scenario")]
  Scenario,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GroupByValues {
  #[sdk(rename = "range")]
  #[default]
  Range,
  #[sdk(rename = "seconds")]
  Seconds,
  #[sdk(rename = "minutes")]
  Minutes,
  #[sdk(rename = "hours")]
  Hours,
  #[sdk(rename = "days")]
  Days,
  #[sdk(rename = "months")]
  Months,
  #[sdk(rename = "quarters")]
  Quarters,
  #[sdk(rename = "years")]
  Years,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SortValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "ascending")]
  Ascending,
  #[sdk(rename = "descending")]
  Descending,
  #[sdk(rename = "ascendingAlpha")]
  AscendingAlpha,
  #[sdk(rename = "descendingAlpha")]
  DescendingAlpha,
  #[sdk(rename = "ascendingNatural")]
  AscendingNatural,
  #[sdk(rename = "descendingNatural")]
  DescendingNatural,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ScopeValues {
  #[sdk(rename = "selection")]
  #[default]
  Selection,
  #[sdk(rename = "data")]
  Data,
  #[sdk(rename = "field")]
  Field,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RuleValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "all")]
  All,
  #[sdk(rename = "row")]
  Row,
  #[sdk(rename = "column")]
  Column,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShowDataAsValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "difference")]
  Difference,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "percentDiff")]
  PercentageDifference,
  #[sdk(rename = "runTotal")]
  RunTotal,
  #[sdk(rename = "percentOfRow")]
  PercentOfRaw,
  #[sdk(rename = "percentOfCol")]
  PercentOfColumn,
  #[sdk(rename = "percentOfTotal")]
  PercentOfTotal,
  #[sdk(rename = "index")]
  Index,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ItemValues {
  #[sdk(rename = "data")]
  #[default]
  Data,
  #[sdk(rename = "default")]
  Default,
  #[sdk(rename = "sum")]
  Sum,
  #[sdk(rename = "countA")]
  CountA,
  #[sdk(rename = "avg")]
  Average,
  #[sdk(rename = "max")]
  Maximum,
  #[sdk(rename = "min")]
  Minimum,
  #[sdk(rename = "product")]
  Product,
  #[sdk(rename = "count")]
  Count,
  #[sdk(rename = "stdDev")]
  StandardDeviation,
  #[sdk(rename = "stdDevP")]
  StandardDeviationP,
  #[sdk(rename = "var")]
  Variance,
  #[sdk(rename = "varP")]
  VarianceP,
  #[sdk(rename = "grand")]
  Grand,
  #[sdk(rename = "blank")]
  Blank,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FieldSortValues {
  #[sdk(rename = "manual")]
  #[default]
  Manual,
  #[sdk(rename = "ascending")]
  Ascending,
  #[sdk(rename = "descending")]
  Descending,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PivotFilterValues {
  #[sdk(rename = "unknown")]
  #[default]
  Unknown,
  #[sdk(rename = "count")]
  Count,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "sum")]
  Sum,
  #[sdk(rename = "captionEqual")]
  CaptionEqual,
  #[sdk(rename = "captionNotEqual")]
  CaptionNotEqual,
  #[sdk(rename = "captionBeginsWith")]
  CaptionBeginsWith,
  #[sdk(rename = "captionNotBeginsWith")]
  CaptionNotBeginsWith,
  #[sdk(rename = "captionEndsWith")]
  CaptionEndsWith,
  #[sdk(rename = "captionNotEndsWith")]
  CaptionNotEndsWith,
  #[sdk(rename = "captionContains")]
  CaptionContains,
  #[sdk(rename = "captionNotContains")]
  CaptionNotContains,
  #[sdk(rename = "captionGreaterThan")]
  CaptionGreaterThan,
  #[sdk(rename = "captionGreaterThanOrEqual")]
  CaptionGreaterThanOrEqual,
  #[sdk(rename = "captionLessThan")]
  CaptionLessThan,
  #[sdk(rename = "captionLessThanOrEqual")]
  CaptionLessThanOrEqual,
  #[sdk(rename = "captionBetween")]
  CaptionBetween,
  #[sdk(rename = "captionNotBetween")]
  CaptionNotBetween,
  #[sdk(rename = "valueEqual")]
  ValueEqual,
  #[sdk(rename = "valueNotEqual")]
  ValueNotEqual,
  #[sdk(rename = "valueGreaterThan")]
  ValueGreaterThan,
  #[sdk(rename = "valueGreaterThanOrEqual")]
  ValueGreaterThanOrEqual,
  #[sdk(rename = "valueLessThan")]
  ValueLessThan,
  #[sdk(rename = "valueLessThanOrEqual")]
  ValueLessThanOrEqual,
  #[sdk(rename = "valueBetween")]
  ValueBetween,
  #[sdk(rename = "valueNotBetween")]
  ValueNotBetween,
  #[sdk(rename = "dateEqual")]
  DateEqual,
  #[sdk(rename = "dateNotEqual")]
  DateNotEqual,
  #[sdk(rename = "dateOlderThan")]
  DateOlderThan,
  #[sdk(rename = "dateOlderThanOrEqual")]
  DateOlderThanOrEqual,
  #[sdk(rename = "dateNewerThan")]
  DateNewerThan,
  #[sdk(rename = "dateNewerThanOrEqual")]
  DateNewerThanOrEqual,
  #[sdk(rename = "dateBetween")]
  DateBetween,
  #[sdk(rename = "dateNotBetween")]
  DateNotBetween,
  #[sdk(rename = "tomorrow")]
  Tomorrow,
  #[sdk(rename = "today")]
  Today,
  #[sdk(rename = "yesterday")]
  Yesterday,
  #[sdk(rename = "nextWeek")]
  NextWeek,
  #[sdk(rename = "thisWeek")]
  ThisWeek,
  #[sdk(rename = "lastWeek")]
  LastWeek,
  #[sdk(rename = "nextMonth")]
  NextMonth,
  #[sdk(rename = "thisMonth")]
  ThisMonth,
  #[sdk(rename = "lastMonth")]
  LastMonth,
  #[sdk(rename = "nextQuarter")]
  NextQuarter,
  #[sdk(rename = "thisQuarter")]
  ThisQuarter,
  #[sdk(rename = "lastQuarter")]
  LastQuarter,
  #[sdk(rename = "nextYear")]
  NextYear,
  #[sdk(rename = "thisYear")]
  ThisYear,
  #[sdk(rename = "lastYear")]
  LastYear,
  #[sdk(rename = "yearToDate")]
  YearToDate,
  #[sdk(rename = "Q1")]
  Quarter1,
  #[sdk(rename = "Q2")]
  Quarter2,
  #[sdk(rename = "Q3")]
  Quarter3,
  #[sdk(rename = "Q4")]
  Quarter4,
  #[sdk(rename = "M1")]
  January,
  #[sdk(rename = "M2")]
  February,
  #[sdk(rename = "M3")]
  March,
  #[sdk(rename = "M4")]
  April,
  #[sdk(rename = "M5")]
  May,
  #[sdk(rename = "M6")]
  June,
  #[sdk(rename = "M7")]
  July,
  #[sdk(rename = "M8")]
  August,
  #[sdk(rename = "M9")]
  September,
  #[sdk(rename = "M10")]
  October,
  #[sdk(rename = "M11")]
  November,
  #[sdk(rename = "M12")]
  December,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FormatActionValues {
  #[sdk(rename = "blank")]
  #[default]
  Blank,
  #[sdk(rename = "formatting")]
  Formatting,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PivotTableAxisValues {
  #[sdk(rename = "axisRow")]
  #[default]
  AxisRow,
  #[sdk(rename = "axisCol")]
  AxisColumn,
  #[sdk(rename = "axisPage")]
  AxisPage,
  #[sdk(rename = "axisValues")]
  AxisValues,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GrowShrinkValues {
  #[sdk(rename = "insertDelete")]
  #[default]
  InsertDelete,
  #[sdk(rename = "insertClear")]
  InsertClear,
  #[sdk(rename = "overwriteClear")]
  OverwriteClear,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PhoneticValues {
  #[sdk(rename = "halfwidthKatakana")]
  #[default]
  HalfWidthKatakana,
  #[sdk(rename = "fullwidthKatakana")]
  FullWidthKatakana,
  #[sdk(rename = "Hiragana")]
  Hiragana,
  #[sdk(rename = "noConversion")]
  NoConversion,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PhoneticAlignmentValues {
  #[sdk(rename = "noControl")]
  #[default]
  NoControl,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RowColumnActionValues {
  #[sdk(rename = "insertRow")]
  #[default]
  InsertRow,
  #[sdk(rename = "deleteRow")]
  DeleteRow,
  #[sdk(rename = "insertCol")]
  ColumnInsert,
  #[sdk(rename = "deleteCol")]
  DeleteColumn,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RevisionActionValues {
  #[sdk(rename = "add")]
  #[default]
  Add,
  #[sdk(rename = "delete")]
  Delete,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FormulaExpressionValues {
  #[sdk(rename = "ref")]
  #[default]
  Reference,
  #[sdk(rename = "refError")]
  ReferenceError,
  #[sdk(rename = "area")]
  Area,
  #[sdk(rename = "areaError")]
  AreaError,
  #[sdk(rename = "computedArea")]
  ComputedArea,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CellFormulaValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "array")]
  Array,
  #[sdk(rename = "dataTable")]
  DataTable,
  #[sdk(rename = "shared")]
  Shared,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PaneValues {
  #[sdk(rename = "bottomRight")]
  #[default]
  BottomRight,
  #[sdk(rename = "topRight")]
  TopRight,
  #[sdk(rename = "bottomLeft")]
  BottomLeft,
  #[sdk(rename = "topLeft")]
  TopLeft,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SheetViewValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "pageBreakPreview")]
  PageBreakPreview,
  #[sdk(rename = "pageLayout")]
  PageLayout,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataConsolidateFunctionValues {
  #[sdk(rename = "average")]
  #[default]
  Average,
  #[sdk(rename = "count")]
  Count,
  #[sdk(rename = "countNums")]
  CountNumbers,
  #[sdk(rename = "max")]
  Maximum,
  #[sdk(rename = "min")]
  Minimum,
  #[sdk(rename = "product")]
  Product,
  #[sdk(rename = "stdDev")]
  StandardDeviation,
  #[sdk(rename = "stdDevp")]
  StandardDeviationP,
  #[sdk(rename = "sum")]
  Sum,
  #[sdk(rename = "var")]
  Variance,
  #[sdk(rename = "varp")]
  VarianceP,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataValidationValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "whole")]
  Whole,
  #[sdk(rename = "decimal")]
  Decimal,
  #[sdk(rename = "list")]
  List,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "time")]
  Time,
  #[sdk(rename = "textLength")]
  TextLength,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataValidationOperatorValues {
  #[sdk(rename = "between")]
  #[default]
  Between,
  #[sdk(rename = "notBetween")]
  NotBetween,
  #[sdk(rename = "equal")]
  Equal,
  #[sdk(rename = "notEqual")]
  NotEqual,
  #[sdk(rename = "lessThan")]
  LessThan,
  #[sdk(rename = "lessThanOrEqual")]
  LessThanOrEqual,
  #[sdk(rename = "greaterThan")]
  GreaterThan,
  #[sdk(rename = "greaterThanOrEqual")]
  GreaterThanOrEqual,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataValidationErrorStyleValues {
  #[sdk(rename = "stop")]
  #[default]
  Stop,
  #[sdk(rename = "warning")]
  Warning,
  #[sdk(rename = "information")]
  Information,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataValidationImeModeValues {
  #[sdk(rename = "noControl")]
  #[default]
  NoControl,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "on")]
  On,
  #[sdk(rename = "disabled")]
  Disabled,
  #[sdk(rename = "hiragana")]
  Hiragana,
  #[sdk(rename = "fullKatakana")]
  FullKatakana,
  #[sdk(rename = "halfKatakana")]
  HalfKatakana,
  #[sdk(rename = "fullAlpha")]
  FullAlpha,
  #[sdk(rename = "halfAlpha")]
  HalfAlpha,
  #[sdk(rename = "fullHangul")]
  FullHangul,
  #[sdk(rename = "halfHangul")]
  HalfHangul,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConditionalFormatValues {
  #[sdk(rename = "expression")]
  #[default]
  Expression,
  #[sdk(rename = "cellIs")]
  CellIs,
  #[sdk(rename = "colorScale")]
  ColorScale,
  #[sdk(rename = "dataBar")]
  DataBar,
  #[sdk(rename = "iconSet")]
  IconSet,
  #[sdk(rename = "top10")]
  Top10,
  #[sdk(rename = "uniqueValues")]
  UniqueValues,
  #[sdk(rename = "duplicateValues")]
  DuplicateValues,
  #[sdk(rename = "containsText")]
  ContainsText,
  #[sdk(rename = "notContainsText")]
  NotContainsText,
  #[sdk(rename = "beginsWith")]
  BeginsWith,
  #[sdk(rename = "endsWith")]
  EndsWith,
  #[sdk(rename = "containsBlanks")]
  ContainsBlanks,
  #[sdk(rename = "notContainsBlanks")]
  NotContainsBlanks,
  #[sdk(rename = "containsErrors")]
  ContainsErrors,
  #[sdk(rename = "notContainsErrors")]
  NotContainsErrors,
  #[sdk(rename = "timePeriod")]
  TimePeriod,
  #[sdk(rename = "aboveAverage")]
  AboveAverage,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimePeriodValues {
  #[sdk(rename = "today")]
  #[default]
  Today,
  #[sdk(rename = "yesterday")]
  Yesterday,
  #[sdk(rename = "tomorrow")]
  Tomorrow,
  #[sdk(rename = "last7Days")]
  Last7Days,
  #[sdk(rename = "thisMonth")]
  ThisMonth,
  #[sdk(rename = "lastMonth")]
  LastMonth,
  #[sdk(rename = "nextMonth")]
  NextMonth,
  #[sdk(rename = "thisWeek")]
  ThisWeek,
  #[sdk(rename = "lastWeek")]
  LastWeek,
  #[sdk(rename = "nextWeek")]
  NextWeek,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConditionalFormattingOperatorValues {
  #[sdk(rename = "lessThan")]
  #[default]
  LessThan,
  #[sdk(rename = "lessThanOrEqual")]
  LessThanOrEqual,
  #[sdk(rename = "equal")]
  Equal,
  #[sdk(rename = "notEqual")]
  NotEqual,
  #[sdk(rename = "greaterThanOrEqual")]
  GreaterThanOrEqual,
  #[sdk(rename = "greaterThan")]
  GreaterThan,
  #[sdk(rename = "between")]
  Between,
  #[sdk(rename = "notBetween")]
  NotBetween,
  #[sdk(rename = "containsText")]
  ContainsText,
  #[sdk(rename = "notContains")]
  NotContains,
  #[sdk(rename = "beginsWith")]
  BeginsWith,
  #[sdk(rename = "endsWith")]
  EndsWith,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConditionalFormatValueObjectValues {
  #[sdk(rename = "num")]
  #[default]
  Number,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "max")]
  Max,
  #[sdk(rename = "min")]
  Min,
  #[sdk(rename = "formula")]
  Formula,
  #[sdk(rename = "percentile")]
  Percentile,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageOrderValues {
  #[sdk(rename = "downThenOver")]
  #[default]
  DownThenOver,
  #[sdk(rename = "overThenDown")]
  OverThenDown,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OrientationValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "portrait")]
  Portrait,
  #[sdk(rename = "landscape")]
  Landscape,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CellCommentsValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "asDisplayed")]
  AsDisplayed,
  #[sdk(rename = "atEnd")]
  AtEnd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PrintErrorValues {
  #[sdk(rename = "displayed")]
  #[default]
  Displayed,
  #[sdk(rename = "blank")]
  Blank,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "NA")]
  Na,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataViewAspectValues {
  #[sdk(rename = "DVASPECT_CONTENT")]
  #[default]
  DataViewAspectContent,
  #[sdk(rename = "DVASPECT_ICON")]
  DataViewAspectIcon,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OleUpdateValues {
  #[sdk(rename = "OLEUPDATE_ALWAYS")]
  #[default]
  OleUpdateAlways,
  #[sdk(rename = "OLEUPDATE_ONCALL")]
  OleUpdateOnCall,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum WebSourceValues {
  #[sdk(rename = "sheet")]
  #[default]
  Sheet,
  #[sdk(rename = "printArea")]
  PrintArea,
  #[sdk(rename = "autoFilter")]
  AutoFilter,
  #[sdk(rename = "range")]
  Range,
  #[sdk(rename = "chart")]
  Chart,
  #[sdk(rename = "pivotTable")]
  PivotTable,
  #[sdk(rename = "query")]
  Query,
  #[sdk(rename = "label")]
  Label,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PaneStateValues {
  #[sdk(rename = "split")]
  #[default]
  Split,
  #[sdk(rename = "frozen")]
  Frozen,
  #[sdk(rename = "frozenSplit")]
  FrozenSplit,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MdxFunctionValues {
  #[sdk(rename = "m")]
  #[default]
  CubeMember,
  #[sdk(rename = "v")]
  CubeValue,
  #[sdk(rename = "s")]
  CubeSet,
  #[sdk(rename = "c")]
  CubeSetCount,
  #[sdk(rename = "r")]
  CubeRankedMember,
  #[sdk(rename = "p")]
  CubeMemberProperty,
  #[sdk(rename = "k")]
  CubeKpiMember,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MdxSetOrderValues {
  #[sdk(rename = "u")]
  #[default]
  Unsorted,
  #[sdk(rename = "a")]
  Ascending,
  #[sdk(rename = "d")]
  Descending,
  #[sdk(rename = "aa")]
  AlphaAscendingSortOrder,
  #[sdk(rename = "ad")]
  AlphaDescendingSortOrder,
  #[sdk(rename = "na")]
  NaturalAscending,
  #[sdk(rename = "nd")]
  NaturalDescending,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MdxKpiPropertyValues {
  #[sdk(rename = "v")]
  #[default]
  Value,
  #[sdk(rename = "g")]
  Goal,
  #[sdk(rename = "s")]
  Status,
  #[sdk(rename = "t")]
  Trend,
  #[sdk(rename = "w")]
  Weight,
  #[sdk(rename = "m")]
  Time,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BorderStyleValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "thin")]
  Thin,
  #[sdk(rename = "medium")]
  Medium,
  #[sdk(rename = "dashed")]
  Dashed,
  #[sdk(rename = "dotted")]
  Dotted,
  #[sdk(rename = "thick")]
  Thick,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "hair")]
  Hair,
  #[sdk(rename = "mediumDashed")]
  MediumDashed,
  #[sdk(rename = "dashDot")]
  DashDot,
  #[sdk(rename = "mediumDashDot")]
  MediumDashDot,
  #[sdk(rename = "dashDotDot")]
  DashDotDot,
  #[sdk(rename = "mediumDashDotDot")]
  MediumDashDotDot,
  #[sdk(rename = "slantDashDot")]
  SlantDashDot,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PatternValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "solid")]
  Solid,
  #[sdk(rename = "mediumGray")]
  MediumGray,
  #[sdk(rename = "darkGray")]
  DarkGray,
  #[sdk(rename = "lightGray")]
  LightGray,
  #[sdk(rename = "darkHorizontal")]
  DarkHorizontal,
  #[sdk(rename = "darkVertical")]
  DarkVertical,
  #[sdk(rename = "darkDown")]
  DarkDown,
  #[sdk(rename = "darkUp")]
  DarkUp,
  #[sdk(rename = "darkGrid")]
  DarkGrid,
  #[sdk(rename = "darkTrellis")]
  DarkTrellis,
  #[sdk(rename = "lightHorizontal")]
  LightHorizontal,
  #[sdk(rename = "lightVertical")]
  LightVertical,
  #[sdk(rename = "lightDown")]
  LightDown,
  #[sdk(rename = "lightUp")]
  LightUp,
  #[sdk(rename = "lightGrid")]
  LightGrid,
  #[sdk(rename = "lightTrellis")]
  LightTrellis,
  #[sdk(rename = "gray125")]
  Gray125,
  #[sdk(rename = "gray0625")]
  Gray0625,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GradientValues {
  #[sdk(rename = "linear")]
  #[default]
  Linear,
  #[sdk(rename = "path")]
  Path,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum HorizontalAlignmentValues {
  #[sdk(rename = "general")]
  #[default]
  General,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "fill")]
  Fill,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "centerContinuous")]
  CenterContinuous,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableStyleValues {
  #[sdk(rename = "wholeTable")]
  #[default]
  WholeTable,
  #[sdk(rename = "headerRow")]
  HeaderRow,
  #[sdk(rename = "totalRow")]
  TotalRow,
  #[sdk(rename = "firstColumn")]
  FirstColumn,
  #[sdk(rename = "lastColumn")]
  LastColumn,
  #[sdk(rename = "firstRowStripe")]
  FirstRowStripe,
  #[sdk(rename = "secondRowStripe")]
  SecondRowStripe,
  #[sdk(rename = "firstColumnStripe")]
  FirstColumnStripe,
  #[sdk(rename = "secondColumnStripe")]
  SecondColumnStripe,
  #[sdk(rename = "firstHeaderCell")]
  FirstHeaderCell,
  #[sdk(rename = "lastHeaderCell")]
  LastHeaderCell,
  #[sdk(rename = "firstTotalCell")]
  FirstTotalCell,
  #[sdk(rename = "lastTotalCell")]
  LastTotalCell,
  #[sdk(rename = "firstSubtotalColumn")]
  FirstSubtotalColumn,
  #[sdk(rename = "secondSubtotalColumn")]
  SecondSubtotalColumn,
  #[sdk(rename = "thirdSubtotalColumn")]
  ThirdSubtotalColumn,
  #[sdk(rename = "firstSubtotalRow")]
  FirstSubtotalRow,
  #[sdk(rename = "secondSubtotalRow")]
  SecondSubtotalRow,
  #[sdk(rename = "thirdSubtotalRow")]
  ThirdSubtotalRow,
  #[sdk(rename = "blankRow")]
  BlankRow,
  #[sdk(rename = "firstColumnSubheading")]
  FirstColumnSubheading,
  #[sdk(rename = "secondColumnSubheading")]
  SecondColumnSubheading,
  #[sdk(rename = "thirdColumnSubheading")]
  ThirdColumnSubheading,
  #[sdk(rename = "firstRowSubheading")]
  FirstRowSubheading,
  #[sdk(rename = "secondRowSubheading")]
  SecondRowSubheading,
  #[sdk(rename = "thirdRowSubheading")]
  ThirdRowSubheading,
  #[sdk(rename = "pageFieldLabels")]
  PageFieldLabels,
  #[sdk(rename = "pageFieldValues")]
  PageFieldValues,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentRunValues {
  #[sdk(rename = "baseline")]
  #[default]
  Baseline,
  #[sdk(rename = "superscript")]
  Superscript,
  #[sdk(rename = "subscript")]
  Subscript,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FontSchemeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "major")]
  Major,
  #[sdk(rename = "minor")]
  Minor,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum UnderlineValues {
  #[sdk(rename = "single")]
  #[default]
  Single,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "singleAccounting")]
  SingleAccounting,
  #[sdk(rename = "doubleAccounting")]
  DoubleAccounting,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DdeValues {
  #[sdk(rename = "nil")]
  #[default]
  Nil,
  #[sdk(rename = "b")]
  Boolean,
  #[sdk(rename = "n")]
  RealNumber,
  #[sdk(rename = "e")]
  Error,
  #[sdk(rename = "str")]
  String,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TableValues {
  #[sdk(rename = "worksheet")]
  #[default]
  Worksheet,
  #[sdk(rename = "xml")]
  Xml,
  #[sdk(rename = "queryTable")]
  QueryTable,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TotalsRowFunctionValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "sum")]
  Sum,
  #[sdk(rename = "min")]
  Minimum,
  #[sdk(rename = "max")]
  Maximum,
  #[sdk(rename = "average")]
  Average,
  #[sdk(rename = "count")]
  Count,
  #[sdk(rename = "countNums")]
  CountNumbers,
  #[sdk(rename = "stdDev")]
  StandardDeviation,
  #[sdk(rename = "var")]
  Variance,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum XmlDataValues {
  #[sdk(rename = "string")]
  #[default]
  String,
  #[sdk(rename = "normalizedString")]
  NormalizedString,
  #[sdk(rename = "token")]
  Token,
  #[sdk(rename = "byte")]
  Byte,
  #[sdk(rename = "unsignedByte")]
  UnsignedByte,
  #[sdk(rename = "base64Binary")]
  Base64Binary,
  #[sdk(rename = "hexBinary")]
  HexBinary,
  #[sdk(rename = "integer")]
  Integer,
  #[sdk(rename = "positiveInteger")]
  PositiveInteger,
  #[sdk(rename = "negativeInteger")]
  NegativeInteger,
  #[sdk(rename = "nonPositiveInteger")]
  NonPositiveInteger,
  #[sdk(rename = "nonNegativeInteger")]
  NonNegativeInteger,
  #[sdk(rename = "int")]
  Int,
  #[sdk(rename = "unsignedInt")]
  UnsignedInteger,
  #[sdk(rename = "long")]
  Long,
  #[sdk(rename = "unsignedLong")]
  UnsignedLong,
  #[sdk(rename = "short")]
  Short,
  #[sdk(rename = "unsignedShort")]
  UnsignedShort,
  #[sdk(rename = "decimal")]
  Decimal,
  #[sdk(rename = "float")]
  Float,
  #[sdk(rename = "double")]
  Double,
  #[sdk(rename = "boolean")]
  Boolean,
  #[sdk(rename = "time")]
  Time,
  #[sdk(rename = "dateTime")]
  DateTime,
  #[sdk(rename = "duration")]
  Duration,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "gMonth")]
  Gmonth,
  #[sdk(rename = "gYear")]
  Gyear,
  #[sdk(rename = "gYearMonth")]
  GYearMonth,
  #[sdk(rename = "gDay")]
  Gday,
  #[sdk(rename = "gMonthDay")]
  GMonthDay,
  #[sdk(rename = "Name")]
  Name,
  #[sdk(rename = "QName")]
  Qname,
  #[sdk(rename = "NCName")]
  NcName,
  #[sdk(rename = "anyURI")]
  AnyUri,
  #[sdk(rename = "language")]
  Language,
  #[sdk(rename = "ID")]
  Id,
  #[sdk(rename = "IDREF")]
  IdRef,
  #[sdk(rename = "IDREFS")]
  IdRefs,
  #[sdk(rename = "ENTITY")]
  Entity,
  #[sdk(rename = "ENTITIES")]
  Entities,
  #[sdk(rename = "NOTATION")]
  Notation,
  #[sdk(rename = "NMTOKEN")]
  NmToken,
  #[sdk(rename = "NMTOKENS")]
  NmTokens,
  #[sdk(rename = "anyType")]
  AnyType,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VolatileDependencyValues {
  #[sdk(rename = "realTimeData")]
  #[default]
  RealTimeData,
  #[sdk(rename = "olapFunctions")]
  OlapFunctions,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VolatileValues {
  #[sdk(rename = "b")]
  #[default]
  Boolean,
  #[sdk(rename = "n")]
  RealNumber,
  #[sdk(rename = "e")]
  Error,
  #[sdk(rename = "s")]
  String,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum VisibilityValues {
  #[sdk(rename = "visible")]
  #[default]
  Visible,
  #[sdk(rename = "hidden")]
  Hidden,
  #[sdk(rename = "veryHidden")]
  VeryHidden,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CommentsValues {
  #[sdk(rename = "commNone")]
  #[default]
  CommentNone,
  #[sdk(rename = "commIndicator")]
  CommentIndicator,
  #[sdk(rename = "commIndAndComment")]
  CommentIndicatorAndComment,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectDisplayValues {
  #[sdk(rename = "all")]
  #[default]
  All,
  #[sdk(rename = "placeholders")]
  Placeholders,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SheetStateValues {
  #[sdk(rename = "visible")]
  #[default]
  Visible,
  #[sdk(rename = "hidden")]
  Hidden,
  #[sdk(rename = "veryHidden")]
  VeryHidden,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum UpdateLinksBehaviorValues {
  #[sdk(rename = "userSet")]
  #[default]
  UserSet,
  #[sdk(rename = "never")]
  Never,
  #[sdk(rename = "always")]
  Always,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CalculateModeValues {
  #[sdk(rename = "manual")]
  #[default]
  Manual,
  #[sdk(rename = "auto")]
  Auto,
  #[sdk(rename = "autoNoTable")]
  AutoNoTable,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ReferenceModeValues {
  #[sdk(rename = "A1")]
  #[default]
  A1,
  #[sdk(rename = "R1C1")]
  R1c1,
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
pub enum TextHorizontalAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextVerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CellValues {
  #[sdk(rename = "b")]
  #[default]
  Boolean,
  #[sdk(rename = "n")]
  Number,
  #[sdk(rename = "e")]
  Error,
  #[sdk(rename = "s")]
  SharedString,
  #[sdk(rename = "str")]
  String,
  #[sdk(rename = "inlineStr")]
  InlineString,
  #[sdk(rename = "d")]
  Date,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PivotAreaValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "normal")]
  Normal,
  #[sdk(rename = "data")]
  Data,
  #[sdk(rename = "all")]
  All,
  #[sdk(rename = "origin")]
  Origin,
  #[sdk(rename = "button")]
  Button,
  #[sdk(rename = "topRight")]
  TopRight,
  #[sdk(rename = "topEnd")]
  TopEnd,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConformanceClass {
  #[sdk(rename = "strict")]
  #[default]
  Enumstrict,
  #[sdk(rename = "transitional")]
  Enumtransitional,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Extension/x:ext")]
pub struct Extension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Calculation Chain Info.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcChain/x:calcChain")]
pub struct CalculationChain {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Cell.
  #[sdk(child(qname = "x:CT_CalcCell/x:c"))]
  pub x_c: Vec<CalculationCell>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Comments.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Comments/x:comments")]
pub struct Comments {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Authors
  #[sdk(child(qname = "x:CT_Authors/x:authors"))]
  pub authors: std::boxed::Box<Authors>,
  /// List of Comments
  #[sdk(child(qname = "x:CT_CommentList/x:commentList"))]
  pub comment_list: std::boxed::Box<CommentList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// XML Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MapInfo/x:MapInfo")]
pub struct MapInfo {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Prefix Mappings for XPath Expressions
  #[sdk(attr(qname = ":SelectionNamespaces"))]
  pub selection_namespaces: crate::simple_type::StringValue,
  /// XML Schema.
  #[sdk(child(qname = "x:CT_Schema/x:Schema"))]
  pub x_schema: Vec<Schema>,
  /// XML Mapping Properties.
  #[sdk(child(qname = "x:CT_Map/x:Map"))]
  pub x_map: Vec<Map>,
}
/// Connections.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Connections/x:connections")]
pub struct Connections {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Connection.
  #[sdk(child(qname = "x:CT_Connection/x:connection"))]
  pub x_connection: Vec<Connection>,
}
/// PivotCache Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinition/x:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// invalid
  #[sdk(attr(qname = ":invalid"))]
  pub invalid: Option<crate::simple_type::BooleanValue>,
  /// saveData
  #[sdk(attr(qname = ":saveData"))]
  pub save_data: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// optimizeMemory
  #[sdk(attr(qname = ":optimizeMemory"))]
  pub optimize_memory: Option<crate::simple_type::BooleanValue>,
  /// enableRefresh
  #[sdk(attr(qname = ":enableRefresh"))]
  pub enable_refresh: Option<crate::simple_type::BooleanValue>,
  /// refreshedBy
  #[sdk(attr(qname = ":refreshedBy"))]
  pub refreshed_by: Option<crate::simple_type::StringValue>,
  /// refreshedDateIso
  #[sdk(attr(qname = ":refreshedDateIso"))]
  pub last_refreshed_date_iso: Option<crate::simple_type::DateTimeValue>,
  /// refreshedDate
  #[sdk(attr(qname = ":refreshedDate"))]
  pub refreshed_date: Option<crate::simple_type::DoubleValue>,
  /// backgroundQuery
  #[sdk(attr(qname = ":backgroundQuery"))]
  pub background_query: Option<crate::simple_type::BooleanValue>,
  /// missingItemsLimit
  #[sdk(attr(qname = ":missingItemsLimit"))]
  pub missing_items_limit: Option<crate::simple_type::UInt32Value>,
  /// createdVersion
  #[sdk(attr(qname = ":createdVersion"))]
  pub created_version: Option<crate::simple_type::ByteValue>,
  /// refreshedVersion
  #[sdk(attr(qname = ":refreshedVersion"))]
  pub refreshed_version: Option<crate::simple_type::ByteValue>,
  /// minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// recordCount
  #[sdk(attr(qname = ":recordCount"))]
  pub record_count: Option<crate::simple_type::UInt32Value>,
  /// upgradeOnRefresh
  #[sdk(attr(qname = ":upgradeOnRefresh"))]
  pub upgrade_on_refresh: Option<crate::simple_type::BooleanValue>,
  /// tupleCache
  #[sdk(attr(qname = ":tupleCache"))]
  pub is_tuple_cache: Option<crate::simple_type::BooleanValue>,
  /// supportSubquery
  #[sdk(attr(qname = ":supportSubquery"))]
  pub support_subquery: Option<crate::simple_type::BooleanValue>,
  /// supportAdvancedDrill
  #[sdk(attr(qname = ":supportAdvancedDrill"))]
  pub support_advanced_drill: Option<crate::simple_type::BooleanValue>,
  /// Defines the CacheSource Class.
  #[sdk(child(qname = "x:CT_CacheSource/x:cacheSource"))]
  pub cache_source: std::boxed::Box<CacheSource>,
  /// Defines the CacheFields Class.
  #[sdk(child(qname = "x:CT_CacheFields/x:cacheFields"))]
  pub cache_fields: std::boxed::Box<CacheFields>,
  /// Defines the CacheHierarchies Class.
  #[sdk(child(qname = "x:CT_CacheHierarchies/x:cacheHierarchies"))]
  pub cache_hierarchies: Option<CacheHierarchies>,
  /// Defines the Kpis Class.
  #[sdk(child(qname = "x:CT_PCDKPIs/x:kpis"))]
  pub kpis: Option<Kpis>,
  /// Defines the TupleCache Class.
  #[sdk(child(qname = "x:CT_TupleCache/x:tupleCache"))]
  pub tuple_cache: Option<std::boxed::Box<TupleCache>>,
  /// Defines the CalculatedItems Class.
  #[sdk(child(qname = "x:CT_CalculatedItems/x:calculatedItems"))]
  pub calculated_items: Option<CalculatedItems>,
  /// Defines the CalculatedMembers Class.
  #[sdk(child(qname = "x:CT_CalculatedMembers/x:calculatedMembers"))]
  pub calculated_members: Option<CalculatedMembers>,
  /// Defines the Dimensions Class.
  #[sdk(child(qname = "x:CT_Dimensions/x:dimensions"))]
  pub dimensions: Option<Dimensions>,
  /// Defines the MeasureGroups Class.
  #[sdk(child(qname = "x:CT_MeasureGroups/x:measureGroups"))]
  pub measure_groups: Option<MeasureGroups>,
  /// Defines the Maps Class.
  #[sdk(child(qname = "x:CT_MeasureDimensionMaps/x:maps"))]
  pub maps: Option<Maps>,
  /// Defines the PivotCacheDefinitionExtensionList Class.
  #[sdk(child(qname = "x:CT_PivotCacheDefinitionExtensionList/x:extLst"))]
  pub pivot_cache_definition_extension_list: Option<PivotCacheDefinitionExtensionList>,
}
/// PivotCache Records.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheRecords/x:pivotCacheRecords")]
pub struct PivotCacheRecords {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// PivotCache Records Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotCache Record.
  #[sdk(child(qname = "x:CT_Record/x:r"))]
  pub x_r: Vec<PivotCacheRecord>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// PivotTable Definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinition/x:pivotTableDefinition")]
pub struct PivotTableDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cacheId
  #[sdk(attr(qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// dataOnRows
  #[sdk(attr(qname = ":dataOnRows"))]
  pub data_on_rows: Option<crate::simple_type::BooleanValue>,
  /// dataPosition
  #[sdk(attr(qname = ":dataPosition"))]
  pub data_position: Option<crate::simple_type::UInt32Value>,
  /// Auto Format Id
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// dataCaption
  #[sdk(attr(qname = ":dataCaption"))]
  pub data_caption: crate::simple_type::StringValue,
  /// grandTotalCaption
  #[sdk(attr(qname = ":grandTotalCaption"))]
  pub grand_total_caption: Option<crate::simple_type::StringValue>,
  /// errorCaption
  #[sdk(attr(qname = ":errorCaption"))]
  pub error_caption: Option<crate::simple_type::StringValue>,
  /// showError
  #[sdk(attr(qname = ":showError"))]
  pub show_error: Option<crate::simple_type::BooleanValue>,
  /// missingCaption
  #[sdk(attr(qname = ":missingCaption"))]
  pub missing_caption: Option<crate::simple_type::StringValue>,
  /// showMissing
  #[sdk(attr(qname = ":showMissing"))]
  pub show_missing: Option<crate::simple_type::BooleanValue>,
  /// pageStyle
  #[sdk(attr(qname = ":pageStyle"))]
  pub page_style: Option<crate::simple_type::StringValue>,
  /// pivotTableStyle
  #[sdk(attr(qname = ":pivotTableStyle"))]
  pub pivot_table_style_name: Option<crate::simple_type::StringValue>,
  /// vacatedStyle
  #[sdk(attr(qname = ":vacatedStyle"))]
  pub vacated_style: Option<crate::simple_type::StringValue>,
  /// tag
  #[sdk(attr(qname = ":tag"))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// updatedVersion
  #[sdk(attr(qname = ":updatedVersion"))]
  pub updated_version: Option<crate::simple_type::ByteValue>,
  /// minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// asteriskTotals
  #[sdk(attr(qname = ":asteriskTotals"))]
  pub asterisk_totals: Option<crate::simple_type::BooleanValue>,
  /// showItems
  #[sdk(attr(qname = ":showItems"))]
  pub show_items: Option<crate::simple_type::BooleanValue>,
  /// editData
  #[sdk(attr(qname = ":editData"))]
  pub edit_data: Option<crate::simple_type::BooleanValue>,
  /// disableFieldList
  #[sdk(attr(qname = ":disableFieldList"))]
  pub disable_field_list: Option<crate::simple_type::BooleanValue>,
  /// showCalcMbrs
  #[sdk(attr(qname = ":showCalcMbrs"))]
  pub show_calculated_members: Option<crate::simple_type::BooleanValue>,
  /// visualTotals
  #[sdk(attr(qname = ":visualTotals"))]
  pub visual_totals: Option<crate::simple_type::BooleanValue>,
  /// showMultipleLabel
  #[sdk(attr(qname = ":showMultipleLabel"))]
  pub show_multiple_label: Option<crate::simple_type::BooleanValue>,
  /// showDataDropDown
  #[sdk(attr(qname = ":showDataDropDown"))]
  pub show_data_drop_down: Option<crate::simple_type::BooleanValue>,
  /// showDrill
  #[sdk(attr(qname = ":showDrill"))]
  pub show_drill: Option<crate::simple_type::BooleanValue>,
  /// printDrill
  #[sdk(attr(qname = ":printDrill"))]
  pub print_drill: Option<crate::simple_type::BooleanValue>,
  /// showMemberPropertyTips
  #[sdk(attr(qname = ":showMemberPropertyTips"))]
  pub show_member_property_tips: Option<crate::simple_type::BooleanValue>,
  /// showDataTips
  #[sdk(attr(qname = ":showDataTips"))]
  pub show_data_tips: Option<crate::simple_type::BooleanValue>,
  /// enableWizard
  #[sdk(attr(qname = ":enableWizard"))]
  pub enable_wizard: Option<crate::simple_type::BooleanValue>,
  /// enableDrill
  #[sdk(attr(qname = ":enableDrill"))]
  pub enable_drill: Option<crate::simple_type::BooleanValue>,
  /// enableFieldProperties
  #[sdk(attr(qname = ":enableFieldProperties"))]
  pub enable_field_properties: Option<crate::simple_type::BooleanValue>,
  /// preserveFormatting
  #[sdk(attr(qname = ":preserveFormatting"))]
  pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
  /// useAutoFormatting
  #[sdk(attr(qname = ":useAutoFormatting"))]
  pub use_auto_formatting: Option<crate::simple_type::BooleanValue>,
  /// pageWrap
  #[sdk(attr(qname = ":pageWrap"))]
  pub page_wrap: Option<crate::simple_type::UInt32Value>,
  /// pageOverThenDown
  #[sdk(attr(qname = ":pageOverThenDown"))]
  pub page_over_then_down: Option<crate::simple_type::BooleanValue>,
  /// subtotalHiddenItems
  #[sdk(attr(qname = ":subtotalHiddenItems"))]
  pub subtotal_hidden_items: Option<crate::simple_type::BooleanValue>,
  /// rowGrandTotals
  #[sdk(attr(qname = ":rowGrandTotals"))]
  pub row_grand_totals: Option<crate::simple_type::BooleanValue>,
  /// colGrandTotals
  #[sdk(attr(qname = ":colGrandTotals"))]
  pub column_grand_totals: Option<crate::simple_type::BooleanValue>,
  /// fieldPrintTitles
  #[sdk(attr(qname = ":fieldPrintTitles"))]
  pub field_print_titles: Option<crate::simple_type::BooleanValue>,
  /// itemPrintTitles
  #[sdk(attr(qname = ":itemPrintTitles"))]
  pub item_print_titles: Option<crate::simple_type::BooleanValue>,
  /// mergeItem
  #[sdk(attr(qname = ":mergeItem"))]
  pub merge_item: Option<crate::simple_type::BooleanValue>,
  /// showDropZones
  #[sdk(attr(qname = ":showDropZones"))]
  pub show_drop_zones: Option<crate::simple_type::BooleanValue>,
  /// createdVersion
  #[sdk(attr(qname = ":createdVersion"))]
  pub created_version: Option<crate::simple_type::ByteValue>,
  /// indent
  #[sdk(attr(qname = ":indent"))]
  pub indent: Option<crate::simple_type::UInt32Value>,
  /// showEmptyRow
  #[sdk(attr(qname = ":showEmptyRow"))]
  pub show_empty_row: Option<crate::simple_type::BooleanValue>,
  /// showEmptyCol
  #[sdk(attr(qname = ":showEmptyCol"))]
  pub show_empty_column: Option<crate::simple_type::BooleanValue>,
  /// showHeaders
  #[sdk(attr(qname = ":showHeaders"))]
  pub show_headers: Option<crate::simple_type::BooleanValue>,
  /// compact
  #[sdk(attr(qname = ":compact"))]
  pub compact: Option<crate::simple_type::BooleanValue>,
  /// outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// outlineData
  #[sdk(attr(qname = ":outlineData"))]
  pub outline_data: Option<crate::simple_type::BooleanValue>,
  /// compactData
  #[sdk(attr(qname = ":compactData"))]
  pub compact_data: Option<crate::simple_type::BooleanValue>,
  /// published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// gridDropZones
  #[sdk(attr(qname = ":gridDropZones"))]
  pub grid_drop_zones: Option<crate::simple_type::BooleanValue>,
  /// immersive
  #[sdk(attr(qname = ":immersive"))]
  pub stop_immersive_ui: Option<crate::simple_type::BooleanValue>,
  /// multipleFieldFilters
  #[sdk(attr(qname = ":multipleFieldFilters"))]
  pub multiple_field_filters: Option<crate::simple_type::BooleanValue>,
  /// chartFormat
  #[sdk(attr(qname = ":chartFormat"))]
  pub chart_format: Option<crate::simple_type::UInt32Value>,
  /// rowHeaderCaption
  #[sdk(attr(qname = ":rowHeaderCaption"))]
  pub row_header_caption: Option<crate::simple_type::StringValue>,
  /// colHeaderCaption
  #[sdk(attr(qname = ":colHeaderCaption"))]
  pub column_header_caption: Option<crate::simple_type::StringValue>,
  /// fieldListSortAscending
  #[sdk(attr(qname = ":fieldListSortAscending"))]
  pub field_list_sort_ascending: Option<crate::simple_type::BooleanValue>,
  /// mdxSubqueries
  #[sdk(attr(qname = ":mdxSubqueries"))]
  pub mdx_subqueries: Option<crate::simple_type::BooleanValue>,
  /// customListSort
  #[sdk(attr(qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// Defines the Location Class.
  #[sdk(child(qname = "x:CT_Location/x:location"))]
  pub location: std::boxed::Box<Location>,
  /// Defines the PivotFields Class.
  #[sdk(child(qname = "x:CT_PivotFields/x:pivotFields"))]
  pub pivot_fields: Option<PivotFields>,
  /// Defines the RowFields Class.
  #[sdk(child(qname = "x:CT_RowFields/x:rowFields"))]
  pub row_fields: Option<RowFields>,
  /// Defines the RowItems Class.
  #[sdk(child(qname = "x:CT_rowItems/x:rowItems"))]
  pub row_items: Option<RowItems>,
  /// Defines the ColumnFields Class.
  #[sdk(child(qname = "x:CT_ColFields/x:colFields"))]
  pub column_fields: Option<ColumnFields>,
  /// Defines the ColumnItems Class.
  #[sdk(child(qname = "x:CT_colItems/x:colItems"))]
  pub column_items: Option<ColumnItems>,
  /// Defines the PageFields Class.
  #[sdk(child(qname = "x:CT_PageFields/x:pageFields"))]
  pub page_fields: Option<PageFields>,
  /// Defines the DataFields Class.
  #[sdk(child(qname = "x:CT_DataFields/x:dataFields"))]
  pub data_fields: Option<DataFields>,
  /// Defines the Formats Class.
  #[sdk(child(qname = "x:CT_Formats/x:formats"))]
  pub formats: Option<Formats>,
  /// Defines the ConditionalFormats Class.
  #[sdk(child(qname = "x:CT_ConditionalFormats/x:conditionalFormats"))]
  pub conditional_formats: Option<ConditionalFormats>,
  /// Defines the ChartFormats Class.
  #[sdk(child(qname = "x:CT_ChartFormats/x:chartFormats"))]
  pub chart_formats: Option<ChartFormats>,
  /// Defines the PivotHierarchies Class.
  #[sdk(child(qname = "x:CT_PivotHierarchies/x:pivotHierarchies"))]
  pub pivot_hierarchies: Option<PivotHierarchies>,
  /// Defines the PivotTableStyle Class.
  #[sdk(child(qname = "x:CT_PivotTableStyle/x:pivotTableStyleInfo"))]
  pub pivot_table_style: Option<PivotTableStyle>,
  /// Defines the PivotFilters Class.
  #[sdk(child(qname = "x:CT_PivotFilters/x:filters"))]
  pub pivot_filters: Option<PivotFilters>,
  /// Defines the RowHierarchiesUsage Class.
  #[sdk(child(qname = "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage"))]
  pub row_hierarchies_usage: Option<RowHierarchiesUsage>,
  /// Defines the ColumnHierarchiesUsage Class.
  #[sdk(child(qname = "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage"))]
  pub column_hierarchies_usage: Option<ColumnHierarchiesUsage>,
  /// Defines the PivotTableDefinitionExtensionList Class.
  #[sdk(child(qname = "x:CT_pivotTableDefinitionExtensionList/x:extLst"))]
  pub pivot_table_definition_extension_list: Option<PivotTableDefinitionExtensionList>,
}
/// Query Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTable/x:queryTable")]
pub struct QueryTable {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// headers
  #[sdk(attr(qname = ":headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
  /// rowNumbers
  #[sdk(attr(qname = ":rowNumbers"))]
  pub row_numbers: Option<crate::simple_type::BooleanValue>,
  /// disableRefresh
  #[sdk(attr(qname = ":disableRefresh"))]
  pub disable_refresh: Option<crate::simple_type::BooleanValue>,
  /// backgroundRefresh
  #[sdk(attr(qname = ":backgroundRefresh"))]
  pub background_refresh: Option<crate::simple_type::BooleanValue>,
  /// firstBackgroundRefresh
  #[sdk(attr(qname = ":firstBackgroundRefresh"))]
  pub first_background_refresh: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// growShrinkType
  #[sdk(attr(qname = ":growShrinkType"))]
  pub grow_shrink_type: Option<GrowShrinkValues>,
  /// fillFormulas
  #[sdk(attr(qname = ":fillFormulas"))]
  pub fill_formulas: Option<crate::simple_type::BooleanValue>,
  /// removeDataOnSave
  #[sdk(attr(qname = ":removeDataOnSave"))]
  pub remove_data_on_save: Option<crate::simple_type::BooleanValue>,
  /// disableEdit
  #[sdk(attr(qname = ":disableEdit"))]
  pub disable_edit: Option<crate::simple_type::BooleanValue>,
  /// preserveFormatting
  #[sdk(attr(qname = ":preserveFormatting"))]
  pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
  /// adjustColumnWidth
  #[sdk(attr(qname = ":adjustColumnWidth"))]
  pub adjust_column_width: Option<crate::simple_type::BooleanValue>,
  /// intermediate
  #[sdk(attr(qname = ":intermediate"))]
  pub intermediate: Option<crate::simple_type::BooleanValue>,
  /// connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: crate::simple_type::UInt32Value,
  /// Auto Format Id
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// Defines the QueryTableRefresh Class.
  #[sdk(child(qname = "x:CT_QueryTableRefresh/x:queryTableRefresh"))]
  pub query_table_refresh: Option<std::boxed::Box<QueryTableRefresh>>,
  /// Defines the QueryTableExtensionList Class.
  #[sdk(child(qname = "x:CT_QueryTableExtensionList/x:extLst"))]
  pub query_table_extension_list: Option<QueryTableExtensionList>,
}
/// Shared String Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sst/x:sst")]
pub struct SharedStringTable {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// String Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Unique String Count
  #[sdk(attr(qname = ":uniqueCount"))]
  pub unique_count: Option<crate::simple_type::UInt32Value>,
  /// String Item.
  #[sdk(child(qname = "x:CT_Rst/x:si"))]
  pub x_si: Vec<SharedStringItem>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Revision Headers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionHeaders/x:headers")]
pub struct Headers {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Last Revision GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Last GUID
  #[sdk(attr(qname = ":lastGuid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub last_guid: Option<crate::simple_type::StringValue>,
  /// Shared Workbook
  #[sdk(attr(qname = ":shared"))]
  pub shared: Option<crate::simple_type::BooleanValue>,
  /// Disk Revisions
  #[sdk(attr(qname = ":diskRevisions"))]
  pub disk_revisions: Option<crate::simple_type::BooleanValue>,
  /// History
  #[sdk(attr(qname = ":history"))]
  pub history: Option<crate::simple_type::BooleanValue>,
  /// Track Revisions
  #[sdk(attr(qname = ":trackRevisions"))]
  pub track_revisions: Option<crate::simple_type::BooleanValue>,
  /// Exclusive Mode
  #[sdk(attr(qname = ":exclusive"))]
  pub exclusive: Option<crate::simple_type::BooleanValue>,
  /// Revision Id
  #[sdk(attr(qname = ":revisionId"))]
  pub revision_id: Option<crate::simple_type::UInt32Value>,
  /// Version
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::Int32Value>,
  /// Keep Change History
  #[sdk(attr(qname = ":keepChangeHistory"))]
  pub keep_change_history: Option<crate::simple_type::BooleanValue>,
  /// Protected
  #[sdk(attr(qname = ":protected"))]
  pub protected: Option<crate::simple_type::BooleanValue>,
  /// Preserve History
  #[sdk(attr(qname = ":preserveHistory"))]
  pub preserve_history: Option<crate::simple_type::UInt32Value>,
  /// Header.
  #[sdk(child(qname = "x:CT_RevisionHeader/x:header"))]
  pub x_header: Vec<Header>,
}
/// Revisions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Revisions/x:revisions")]
pub struct Revisions {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "x:CT_RevisionRowColumn/x:rrc",
    qname = "x:CT_RevisionMove/x:rm",
    qname = "x:CT_RevisionCustomView/x:rcv",
    qname = "x:CT_RevisionSheetRename/x:rsnm",
    qname = "x:CT_RevisionInsertSheet/x:ris",
    qname = "x:CT_RevisionCellChange/x:rcc",
    qname = "x:CT_RevisionFormatting/x:rfmt",
    qname = "x:CT_RevisionAutoFormatting/x:raf",
    qname = "x:CT_RevisionDefinedName/x:rdn",
    qname = "x:CT_RevisionComment/x:rcmt",
    qname = "x:CT_RevisionQueryTableField/x:rqt",
    qname = "x:CT_RevisionConflict/x:rcft",
    text,
    any
  ))]
  pub revisions_choice: Vec<RevisionsChoice>,
}
/// User List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Users/x:users")]
pub struct Users {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Active User Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// User Information.
  #[sdk(child(qname = "x:CT_SharedUser/x:userInfo"))]
  pub x_user_info: Vec<UserInfo>,
}
/// Worksheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Worksheet/x:worksheet")]
pub struct Worksheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Sheet Properties.
  #[sdk(child(qname = "x:CT_SheetPr/x:sheetPr"))]
  pub sheet_properties: Option<std::boxed::Box<SheetProperties>>,
  /// Macro Sheet Dimensions.
  #[sdk(child(qname = "x:CT_SheetDimension/x:dimension"))]
  pub sheet_dimension: Option<SheetDimension>,
  /// Dialog Sheet Views.
  #[sdk(child(qname = "x:CT_SheetViews/x:sheetViews"))]
  pub sheet_views: Option<std::boxed::Box<SheetViews>>,
  /// Dialog Sheet Format Properties.
  #[sdk(child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
  pub sheet_format_properties: Option<SheetFormatProperties>,
  /// Column Information.
  #[sdk(child(qname = "x:CT_Cols/x:cols"))]
  pub x_cols: Vec<Columns>,
  /// Sheet Data.
  #[sdk(child(qname = "x:CT_SheetData/x:sheetData"))]
  pub x_sheet_data: std::boxed::Box<SheetData>,
  /// Defines the SheetCalculationProperties Class.
  #[sdk(child(qname = "x:CT_SheetCalcPr/x:sheetCalcPr"))]
  pub x_sheet_calc_pr: Option<SheetCalculationProperties>,
  /// Sheet Protection.
  #[sdk(child(qname = "x:CT_SheetProtection/x:sheetProtection"))]
  pub x_sheet_protection: Option<SheetProtection>,
  /// Defines the ProtectedRanges Class.
  #[sdk(child(qname = "x:CT_ProtectedRanges/x:protectedRanges"))]
  pub x_protected_ranges: Option<ProtectedRanges>,
  /// Defines the Scenarios Class.
  #[sdk(child(qname = "x:CT_Scenarios/x:scenarios"))]
  pub x_scenarios: Option<Scenarios>,
  /// AutoFilter Settings.
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub x_auto_filter: Option<std::boxed::Box<AutoFilter>>,
  /// Sort State for Auto Filter.
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<std::boxed::Box<SortState>>,
  /// Data Consolidation.
  #[sdk(child(qname = "x:CT_DataConsolidate/x:dataConsolidate"))]
  pub x_data_consolidate: Option<std::boxed::Box<DataConsolidate>>,
  /// Custom Sheet Views.
  #[sdk(child(qname = "x:CT_CustomSheetViews/x:customSheetViews"))]
  pub x_custom_sheet_views: Option<CustomSheetViews>,
  /// Defines the MergeCells Class.
  #[sdk(child(qname = "x:CT_MergeCells/x:mergeCells"))]
  pub x_merge_cells: Option<MergeCells>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
  /// Conditional Formatting.
  #[sdk(child(qname = "x:CT_ConditionalFormatting/x:conditionalFormatting"))]
  pub x_conditional_formatting: Vec<ConditionalFormatting>,
  /// Defines the DataValidations Class.
  #[sdk(child(qname = "x:CT_DataValidations/x:dataValidations"))]
  pub x_data_validations: Option<DataValidations>,
  /// Defines the Hyperlinks Class.
  #[sdk(child(qname = "x:CT_Hyperlinks/x:hyperlinks"))]
  pub x_hyperlinks: Option<Hyperlinks>,
  /// Print Options.
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub x_print_options: Option<PrintOptions>,
  /// Page Margins.
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub x_page_margins: Option<PageMargins>,
  /// Page Setup Settings.
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub x_page_setup: Option<PageSetup>,
  /// Header Footer Settings.
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub x_header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Horizontal Page Breaks.
  #[sdk(child(qname = "x:CT_PageBreak/x:rowBreaks"))]
  pub x_row_breaks: Option<RowBreaks>,
  /// Vertical Page Breaks.
  #[sdk(child(qname = "x:CT_PageBreak/x:colBreaks"))]
  pub x_col_breaks: Option<ColumnBreaks>,
  /// Custom Properties.
  #[sdk(child(qname = "x:CT_CustomProperties/x:customProperties"))]
  pub x_custom_properties: Option<CustomProperties>,
  /// Defines the CellWatches Class.
  #[sdk(child(qname = "x:CT_CellWatches/x:cellWatches"))]
  pub x_cell_watches: Option<CellWatches>,
  /// Defines the IgnoredErrors Class.
  #[sdk(child(qname = "x:CT_IgnoredErrors/x:ignoredErrors"))]
  pub x_ignored_errors: Option<std::boxed::Box<IgnoredErrors>>,
  /// Drawing.
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub x_drawing: Option<Drawing>,
  /// Defines the LegacyDrawing Class.
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub x_legacy_drawing: Option<LegacyDrawing>,
  /// Legacy Drawing Reference in  Header Footer.
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub x_legacy_drawing_hf: Option<LegacyDrawingHeaderFooter>,
  /// Defines the DrawingHeaderFooter Class.
  #[sdk(child(qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub x_drawing_hf: Option<DrawingHeaderFooter>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "x:CT_SheetBackgroundPicture/x:picture"))]
  pub x_picture: Option<Picture>,
  /// Defines the OleObjects Class.
  #[sdk(child(qname = "x:CT_OleObjects/x:oleObjects"))]
  pub x_ole_objects: Option<OleObjects>,
  /// Defines the Controls Class.
  #[sdk(child(qname = "x:CT_Controls/x:controls"))]
  pub x_controls: Option<Controls>,
  /// Defines the WebPublishItems Class.
  #[sdk(child(qname = "x:CT_WebPublishItems/x:webPublishItems"))]
  pub x_web_publish_items: Option<WebPublishItems>,
  /// Defines the TableParts Class.
  #[sdk(child(qname = "x:CT_TableParts/x:tableParts"))]
  pub x_table_parts: Option<TableParts>,
  /// Defines the WorksheetExtensionList Class.
  #[sdk(child(qname = "x:CT_WorksheetExtensionList/x:extLst"))]
  pub x_ext_lst: Option<WorksheetExtensionList>,
}
/// Chart Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Chartsheet/x:chartsheet")]
pub struct Chartsheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Chart Sheet Properties
  #[sdk(child(qname = "x:CT_ChartsheetPr/x:sheetPr"))]
  pub chart_sheet_properties: Option<std::boxed::Box<ChartSheetProperties>>,
  /// Chart Sheet Views
  #[sdk(child(qname = "x:CT_ChartsheetViews/x:sheetViews"))]
  pub chart_sheet_views: std::boxed::Box<ChartSheetViews>,
  /// Chart Sheet Protection
  #[sdk(child(qname = "x:CT_ChartsheetProtection/x:sheetProtection"))]
  pub chart_sheet_protection: Option<ChartSheetProtection>,
  /// Custom Chart Sheet Views
  #[sdk(child(qname = "x:CT_CustomChartsheetViews/x:customSheetViews"))]
  pub custom_chartsheet_views: Option<CustomChartsheetViews>,
  /// Page Margins.
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Chart Sheet Page Setup.
  #[sdk(child(qname = "x:CT_CsPageSetup/x:pageSetup"))]
  pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
  /// Header Footer Settings.
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Drawing
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub drawing: std::boxed::Box<Drawing>,
  /// Defines the LegacyDrawing Class.
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub legacy_drawing: Option<LegacyDrawing>,
  /// Legacy Drawing Reference in  Header Footer
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
  /// Defines the DrawingHeaderFooter Class.
  #[sdk(child(office2010, qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub drawing_header_footer: Option<DrawingHeaderFooter>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "x:CT_SheetBackgroundPicture/x:picture"))]
  pub picture: Option<Picture>,
  /// Defines the WebPublishItems Class.
  #[sdk(child(qname = "x:CT_WebPublishItems/x:webPublishItems"))]
  pub web_publish_items: Option<WebPublishItems>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Dialog Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dialogsheet/x:dialogsheet")]
pub struct DialogSheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Sheet Properties
  #[sdk(child(qname = "x:CT_SheetPr/x:sheetPr"))]
  pub sheet_properties: Option<std::boxed::Box<SheetProperties>>,
  /// Dialog Sheet Views
  #[sdk(child(qname = "x:CT_SheetViews/x:sheetViews"))]
  pub sheet_views: Option<std::boxed::Box<SheetViews>>,
  /// Dialog Sheet Format Properties
  #[sdk(child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
  pub sheet_format_properties: Option<SheetFormatProperties>,
  /// Sheet Protection
  #[sdk(child(qname = "x:CT_SheetProtection/x:sheetProtection"))]
  pub sheet_protection: Option<SheetProtection>,
  /// Custom Sheet Views
  #[sdk(child(qname = "x:CT_CustomSheetViews/x:customSheetViews"))]
  pub custom_sheet_views: Option<CustomSheetViews>,
  /// Print Options
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub print_options: Option<PrintOptions>,
  /// Page Margins
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Page Setup Settings
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub page_setup: Option<PageSetup>,
  /// Header and Footer Settings
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// Drawing
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub drawing: Option<Drawing>,
  /// Legacy Drawing
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub legacy_drawing: std::boxed::Box<LegacyDrawing>,
  /// Legacy Drawing Header Footer
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
  /// Defines the DrawingHeaderFooter Class.
  #[sdk(child(office2010, qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub drawing_header_footer: Option<DrawingHeaderFooter>,
  /// Defines the OleObjects Class.
  #[sdk(child(qname = "x:CT_OleObjects/x:oleObjects"))]
  pub ole_objects: Option<OleObjects>,
  /// Defines the Controls Class.
  #[sdk(child(office2010, qname = "x:CT_Controls/x:controls"))]
  pub controls: Option<Controls>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Metadata/x:metadata")]
pub struct Metadata {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Metadata Types Collection
  #[sdk(child(qname = "x:CT_MetadataTypes/x:metadataTypes"))]
  pub metadata_types: Option<MetadataTypes>,
  /// Metadata String Store
  #[sdk(child(qname = "x:CT_MetadataStrings/x:metadataStrings"))]
  pub metadata_strings: Option<MetadataStrings>,
  /// MDX Metadata Information
  #[sdk(child(qname = "x:CT_MdxMetadata/x:mdxMetadata"))]
  pub mdx_metadata: Option<MdxMetadata>,
  /// Future Metadata.
  #[sdk(child(qname = "x:CT_FutureMetadata/x:futureMetadata"))]
  pub x_future_metadata: Vec<FutureMetadata>,
  /// Cell Metadata.
  #[sdk(child(qname = "x:CT_MetadataBlocks/x:cellMetadata"))]
  pub x_cell_metadata: Option<CellMetadata>,
  /// Value Metadata.
  #[sdk(child(qname = "x:CT_MetadataBlocks/x:valueMetadata"))]
  pub x_value_metadata: Option<ValueMetadata>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Single Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SingleXmlCells/x:singleXmlCells")]
pub struct SingleXmlCells {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Table Properties.
  #[sdk(child(qname = "x:CT_SingleXmlCell/x:singleXmlCell"))]
  pub x_single_xml_cell: Vec<SingleXmlCell>,
}
/// Style Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Stylesheet/x:styleSheet")]
pub struct Stylesheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the NumberingFormats Class.
  #[sdk(child(qname = "x:CT_NumFmts/x:numFmts"))]
  pub numbering_formats: Option<NumberingFormats>,
  /// Defines the Fonts Class.
  #[sdk(child(qname = "x:CT_Fonts/x:fonts"))]
  pub fonts: Option<Fonts>,
  /// Defines the Fills Class.
  #[sdk(child(qname = "x:CT_Fills/x:fills"))]
  pub fills: Option<Fills>,
  /// Defines the Borders Class.
  #[sdk(child(qname = "x:CT_Borders/x:borders"))]
  pub borders: Option<Borders>,
  /// Defines the CellStyleFormats Class.
  #[sdk(child(qname = "x:CT_CellStyleXfs/x:cellStyleXfs"))]
  pub cell_style_formats: Option<CellStyleFormats>,
  /// Defines the CellFormats Class.
  #[sdk(child(qname = "x:CT_CellXfs/x:cellXfs"))]
  pub cell_formats: Option<CellFormats>,
  /// Defines the CellStyles Class.
  #[sdk(child(qname = "x:CT_CellStyles/x:cellStyles"))]
  pub cell_styles: Option<CellStyles>,
  /// Defines the DifferentialFormats Class.
  #[sdk(child(qname = "x:CT_Dxfs/x:dxfs"))]
  pub differential_formats: Option<DifferentialFormats>,
  /// Defines the TableStyles Class.
  #[sdk(child(qname = "x:CT_TableStyles/x:tableStyles"))]
  pub table_styles: Option<TableStyles>,
  /// Defines the Colors Class.
  #[sdk(child(qname = "x:CT_Colors/x:colors"))]
  pub colors: Option<std::boxed::Box<Colors>>,
  /// Defines the StylesheetExtensionList Class.
  #[sdk(child(qname = "x:CT_StylesheetExtensionList/x:extLst"))]
  pub stylesheet_extension_list: Option<StylesheetExtensionList>,
}
/// External Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalLink/x:externalLink")]
pub struct ExternalLink {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "x:CT_ExternalBook/x:externalBook",
    qname = "x:CT_DdeLink/x:ddeLink",
    qname = "x:CT_OleLink/x:oleLink"
  ))]
  pub external_link_choice: Option<ExternalLinkChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Table/x:table")]
pub struct Table {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Table Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Table Name
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: crate::simple_type::StringValue,
  /// Table Comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Table Type
  #[sdk(attr(qname = ":tableType"))]
  pub table_type: Option<TableValues>,
  /// Header Row Count
  #[sdk(attr(qname = ":headerRowCount"))]
  pub header_row_count: Option<crate::simple_type::UInt32Value>,
  /// Insert Row Showing
  #[sdk(attr(qname = ":insertRow"))]
  pub insert_row: Option<crate::simple_type::BooleanValue>,
  /// Insert Row Shift
  #[sdk(attr(qname = ":insertRowShift"))]
  pub insert_row_shift: Option<crate::simple_type::BooleanValue>,
  /// Totals Row Count
  #[sdk(attr(qname = ":totalsRowCount"))]
  pub totals_row_count: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Shown
  #[sdk(attr(qname = ":totalsRowShown"))]
  pub totals_row_shown: Option<crate::simple_type::BooleanValue>,
  /// Published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Header Row Format Id
  #[sdk(attr(qname = ":headerRowDxfId"))]
  pub header_row_format_id: Option<crate::simple_type::UInt32Value>,
  /// Data Area Format Id
  #[sdk(attr(qname = ":dataDxfId"))]
  pub data_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Format Id
  #[sdk(attr(qname = ":totalsRowDxfId"))]
  pub totals_row_format_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Border Format Id
  #[sdk(attr(qname = ":headerRowBorderDxfId"))]
  pub header_row_border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Table Border Format Id
  #[sdk(attr(qname = ":tableBorderDxfId"))]
  pub border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Border Format Id
  #[sdk(attr(qname = ":totalsRowBorderDxfId"))]
  pub totals_row_border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Style
  #[sdk(attr(qname = ":headerRowCellStyle"))]
  pub header_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Data Style Name
  #[sdk(attr(qname = ":dataCellStyle"))]
  pub data_cell_style: Option<crate::simple_type::StringValue>,
  /// Totals Row Style
  #[sdk(attr(qname = ":totalsRowCellStyle"))]
  pub totals_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Connection ID
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  /// Table AutoFilter
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: Option<std::boxed::Box<AutoFilter>>,
  /// Sort State
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub sort_state: Option<std::boxed::Box<SortState>>,
  /// Table Columns
  #[sdk(child(qname = "x:CT_TableColumns/x:tableColumns"))]
  pub table_columns: std::boxed::Box<TableColumns>,
  /// Table Style
  #[sdk(child(qname = "x:CT_TableStyleInfo/x:tableStyleInfo"))]
  pub table_style_info: Option<TableStyleInfo>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_TableExtensionList/x:extLst"))]
  pub table_extension_list: Option<TableExtensionList>,
}
/// Volatile Dependency Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTypes/x:volTypes")]
pub struct VolatileTypes {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Volatile Dependency Type.
  #[sdk(child(qname = "x:CT_VolType/x:volType"))]
  pub x_vol_type: Vec<VolatileType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Workbook.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Workbook/x:workbook")]
pub struct Workbook {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// conformance
  #[sdk(attr(qname = ":conformance"))]
  pub conformance: Option<ConformanceClass>,
  /// Defines the FileVersion Class.
  #[sdk(child(qname = "x:CT_FileVersion/x:fileVersion"))]
  pub file_version: Option<FileVersion>,
  /// Defines the FileSharing Class.
  #[sdk(child(qname = "x:CT_FileSharing/x:fileSharing"))]
  pub file_sharing: Option<FileSharing>,
  /// Defines the WorkbookProperties Class.
  #[sdk(child(qname = "x:CT_WorkbookPr/x:workbookPr"))]
  pub workbook_properties: Option<WorkbookProperties>,
  /// Defines the AbsolutePath Class.
  #[sdk(child(office2013, qname = "x15ac:CT_AbsolutePath/x15ac:absPath"))]
  pub absolute_path: Option<crate::schemas::x15ac::AbsolutePath>,
  /// Defines the WorkbookProtection Class.
  #[sdk(child(qname = "x:CT_WorkbookProtection/x:workbookProtection"))]
  pub workbook_protection: Option<WorkbookProtection>,
  /// Defines the BookViews Class.
  #[sdk(child(qname = "x:CT_BookViews/x:bookViews"))]
  pub book_views: Option<BookViews>,
  /// Defines the Sheets Class.
  #[sdk(child(qname = "x:CT_Sheets/x:sheets"))]
  pub sheets: std::boxed::Box<Sheets>,
  /// Defines the FunctionGroups Class.
  #[sdk(child(qname = "x:CT_FunctionGroups/x:functionGroups"))]
  pub function_groups: Option<FunctionGroups>,
  /// Defines the ExternalReferences Class.
  #[sdk(child(qname = "x:CT_ExternalReferences/x:externalReferences"))]
  pub external_references: Option<ExternalReferences>,
  /// Defines the DefinedNames Class.
  #[sdk(child(qname = "x:CT_DefinedNames/x:definedNames"))]
  pub defined_names: Option<DefinedNames>,
  /// Defines the CalculationProperties Class.
  #[sdk(child(qname = "x:CT_CalcPr/x:calcPr"))]
  pub calculation_properties: Option<CalculationProperties>,
  /// Defines the OleSize Class.
  #[sdk(child(qname = "x:CT_OleSize/x:oleSize"))]
  pub ole_size: Option<OleSize>,
  /// Defines the CustomWorkbookViews Class.
  #[sdk(child(qname = "x:CT_CustomWorkbookViews/x:customWorkbookViews"))]
  pub custom_workbook_views: Option<CustomWorkbookViews>,
  /// Defines the PivotCaches Class.
  #[sdk(child(qname = "x:CT_PivotCaches/x:pivotCaches"))]
  pub pivot_caches: Option<PivotCaches>,
  /// Defines the WebPublishing Class.
  #[sdk(child(qname = "x:CT_WebPublishing/x:webPublishing"))]
  pub web_publishing: Option<WebPublishing>,
  /// Defines the FileRecoveryProperties Class.
  #[sdk(child(qname = "x:CT_FileRecoveryPr/x:fileRecoveryPr"))]
  pub x_file_recovery_pr: Vec<FileRecoveryProperties>,
  /// Defines the WebPublishObjects Class.
  #[sdk(child(qname = "x:CT_WebPublishObjects/x:webPublishObjects"))]
  pub x_web_publish_objects: Option<WebPublishObjects>,
  /// Defines the WorkbookExtensionList Class.
  #[sdk(child(qname = "x:CT_WorkbookExtensionList/x:extLst"))]
  pub x_ext_lst: Option<WorkbookExtensionList>,
}
/// AutoFilter Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FilterColumn/x:filterColumn")]
pub struct FilterColumn {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Filter Column Data
  #[sdk(attr(qname = ":colId"))]
  pub column_id: crate::simple_type::UInt32Value,
  /// Hidden AutoFilter Button
  #[sdk(attr(qname = ":hiddenButton"))]
  pub hidden_button: Option<crate::simple_type::BooleanValue>,
  /// Show Filter Button
  #[sdk(attr(qname = ":showButton"))]
  pub show_button: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "x:CT_Filters/x:filters",
    qname = "x:CT_Top10/x:top10",
    qname = "x14:CT_CustomFilters/x14:customFilters",
    qname = "x:CT_CustomFilters/x:customFilters",
    qname = "x:CT_DynamicFilter/x:dynamicFilter",
    qname = "x:CT_ColorFilter/x:colorFilter",
    qname = "x14:CT_IconFilter/x14:iconFilter",
    qname = "x:CT_IconFilter/x:iconFilter",
    qname = "x:CT_ExtensionList/x:extLst"
  ))]
  pub filter_column_choice: Option<FilterColumnChoice>,
}
/// Sort State for Auto Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SortState/x:sortState")]
pub struct SortState {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Sort by Columns
  #[sdk(attr(qname = ":columnSort"))]
  pub column_sort: Option<crate::simple_type::BooleanValue>,
  /// Case Sensitive
  #[sdk(attr(qname = ":caseSensitive"))]
  pub case_sensitive: Option<crate::simple_type::BooleanValue>,
  /// Sort Method
  #[sdk(attr(qname = ":sortMethod"))]
  pub sort_method: Option<SortMethodValues>,
  /// Sort Range
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_SortCondition/x14:sortCondition",
    qname = "x:CT_SortCondition/x:sortCondition"
  ))]
  pub sort_state_choice: Option<SortStateChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/x:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<Extension>,
}
/// Custom Filter Criteria.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomFilter/x:customFilter")]
pub struct CustomFilter {
  /// Filter Comparison Operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<FilterOperatorValues>,
  /// Top or Bottom Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcCell/x:c")]
pub struct CalculationCell {
  /// Cell Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Sheet Id
  #[sdk(attr(qname = ":i"))]
  pub sheet_id: Option<crate::simple_type::Int32Value>,
  /// Child Chain
  #[sdk(attr(qname = ":s"))]
  pub in_child_chain: Option<crate::simple_type::BooleanValue>,
  /// New Dependency Level
  #[sdk(attr(qname = ":l"))]
  pub new_level: Option<crate::simple_type::BooleanValue>,
  /// New Thread
  #[sdk(attr(qname = ":t"))]
  pub new_thread: Option<crate::simple_type::BooleanValue>,
  /// Array
  #[sdk(attr(qname = ":a"))]
  pub array: Option<crate::simple_type::BooleanValue>,
}
/// Authors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Authors/x:authors")]
pub struct Authors {
  /// Author.
  #[sdk(child(qname = "x:CT_Xstring/x:author"))]
  pub x_author: Vec<Author>,
}
/// List of Comments.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CommentList/x:commentList")]
pub struct CommentList {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Comment.
  #[sdk(child(qname = "x:CT_Comment/x:comment"))]
  pub x_comment: Vec<Comment>,
}
/// Comment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Comment/x:comment")]
pub struct Comment {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Cell Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Author Id
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: crate::simple_type::UInt32Value,
  /// Unique Identifier for Comment
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: Option<crate::simple_type::StringValue>,
  /// shapeId
  #[sdk(attr(office2010, qname = ":shapeId"))]
  pub shape_id: Option<crate::simple_type::UInt32Value>,
  /// Comment Text
  #[sdk(child(qname = "x:CT_Rst/x:text"))]
  pub comment_text: std::boxed::Box<CommentText>,
  /// Defines the CommentProperties Class.
  #[sdk(child(office2010, qname = "x:CT_CommentPr/x:commentPr"))]
  pub comment_properties: Option<std::boxed::Box<CommentProperties>>,
}
/// Author.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:author")]
pub struct Author {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:t")]
pub struct Text {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Cell Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:v")]
pub struct CellValue {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula")]
pub struct Formula {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Old Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oldFormula")]
pub struct OldFormula {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Odd Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oddHeader")]
pub struct OddHeader {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Odd Page Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oddFooter")]
pub struct OddFooter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Even Page Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:evenHeader")]
pub struct EvenHeader {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Even Page Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:evenFooter")]
pub struct EvenFooter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// First Page Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:firstHeader")]
pub struct FirstHeader {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// First Page Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:firstFooter")]
pub struct FirstFooter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// DDE Link Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:val")]
pub struct DdeLinkValue {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Strings in Subtopic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:stp")]
pub struct Subtopic {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula1 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula1")]
pub struct Formula1 {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula2")]
pub struct Formula2 {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// XML Schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Schema/x:Schema")]
pub struct Schema {
  /// Schema ID
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::StringValue,
  /// Schema Reference
  #[sdk(attr(qname = ":SchemaRef"))]
  pub schema_reference: Option<crate::simple_type::StringValue>,
  /// Schema Root Namespace
  #[sdk(attr(qname = ":Namespace"))]
  pub namespace: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// XML Mapping Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Map/x:Map")]
pub struct Map {
  /// XML Mapping ID
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  /// XML Mapping Name
  #[sdk(attr(qname = ":Name"))]
  pub name: crate::simple_type::StringValue,
  /// Root Element Name
  #[sdk(attr(qname = ":RootElement"))]
  pub root_element: crate::simple_type::StringValue,
  /// Schema Name
  #[sdk(attr(qname = ":SchemaID"))]
  pub schema_id: crate::simple_type::StringValue,
  /// Show Validation Errors
  #[sdk(attr(qname = ":ShowImportExportValidationErrors"))]
  pub show_import_export_errors: crate::simple_type::BooleanValue,
  /// AutoFit Table on Refresh
  #[sdk(attr(qname = ":AutoFit"))]
  pub auto_fit: crate::simple_type::BooleanValue,
  /// Append Data to Table
  #[sdk(attr(qname = ":Append"))]
  pub append_data: crate::simple_type::BooleanValue,
  /// Preserve AutoFilter State
  #[sdk(attr(qname = ":PreserveSortAFLayout"))]
  pub preserve_auto_filter_state: crate::simple_type::BooleanValue,
  /// Preserve Cell Formatting
  #[sdk(attr(qname = ":PreserveFormat"))]
  pub preserve_format: crate::simple_type::BooleanValue,
  /// XML Mapping
  #[sdk(child(qname = "x:CT_DataBinding/x:DataBinding"))]
  pub data_binding: Option<std::boxed::Box<DataBinding>>,
}
/// XML Mapping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataBinding/x:DataBinding")]
pub struct DataBinding {
  /// DataBindingName
  #[sdk(attr(qname = ":DataBindingName"))]
  pub data_binding_name: Option<crate::simple_type::StringValue>,
  /// FileBinding
  #[sdk(attr(qname = ":FileBinding"))]
  pub file_binding: Option<crate::simple_type::BooleanValue>,
  /// ConnectionID
  #[sdk(attr(qname = ":ConnectionID"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  /// FileBindingName
  #[sdk(attr(qname = ":FileBindingName"))]
  pub file_binding_name: Option<crate::simple_type::StringValue>,
  /// DataBindingLoadMode
  #[sdk(attr(qname = ":DataBindingLoadMode"))]
  pub data_binding_load_mode: crate::simple_type::UInt32Value,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Connection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Connection/x:connection")]
pub struct Connection {
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// odcFile
  #[sdk(attr(qname = ":odcFile"))]
  pub connection_file: Option<crate::simple_type::StringValue>,
  /// keepAlive
  #[sdk(attr(qname = ":keepAlive"))]
  pub keep_alive: Option<crate::simple_type::BooleanValue>,
  /// interval
  #[sdk(attr(qname = ":interval"))]
  pub interval: Option<crate::simple_type::UInt32Value>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::UInt32Value>,
  /// reconnectionMethod
  #[sdk(attr(qname = ":reconnectionMethod"))]
  pub reconnection_method: Option<crate::simple_type::UInt32Value>,
  /// refreshedVersion
  #[sdk(attr(qname = ":refreshedVersion"))]
  pub refreshed_version: crate::simple_type::ByteValue,
  /// minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// savePassword
  #[sdk(attr(qname = ":savePassword"))]
  pub save_password: Option<crate::simple_type::BooleanValue>,
  /// new
  #[sdk(attr(qname = ":new"))]
  pub new: Option<crate::simple_type::BooleanValue>,
  /// deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// onlyUseConnectionFile
  #[sdk(attr(qname = ":onlyUseConnectionFile"))]
  pub only_use_connection_file: Option<crate::simple_type::BooleanValue>,
  /// background
  #[sdk(attr(qname = ":background"))]
  pub background: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// saveData
  #[sdk(attr(qname = ":saveData"))]
  pub save_data: Option<crate::simple_type::BooleanValue>,
  /// credentials
  #[sdk(attr(qname = ":credentials"))]
  pub credentials: Option<CredentialsMethodValues>,
  /// singleSignOnId
  #[sdk(attr(qname = ":singleSignOnId"))]
  pub single_sign_on_id: Option<crate::simple_type::StringValue>,
  /// Defines the DatabaseProperties Class.
  #[sdk(child(qname = "x:CT_DbPr/x:dbPr"))]
  pub database_properties: Option<DatabaseProperties>,
  /// Defines the OlapProperties Class.
  #[sdk(child(qname = "x:CT_OlapPr/x:olapPr"))]
  pub olap_properties: Option<OlapProperties>,
  /// Defines the WebQueryProperties Class.
  #[sdk(child(qname = "x:CT_WebPr/x:webPr"))]
  pub web_query_properties: Option<std::boxed::Box<WebQueryProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "x:CT_TextPr/x:textPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Defines the Parameters Class.
  #[sdk(child(qname = "x:CT_Parameters/x:parameters"))]
  pub parameters: Option<Parameters>,
  /// Defines the ConnectionExtensionList Class.
  #[sdk(child(qname = "x:CT_ConnectionExtensionList/x:extLst"))]
  pub connection_extension_list: Option<ConnectionExtensionList>,
}
/// Tables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tables/x:tables")]
pub struct Tables {
  /// Count of Tables
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_TableMissing/x:m",
    qname = "x:CT_XStringElement/x:s",
    qname = "x:CT_Index/x:x"
  ))]
  pub tables_choice: Vec<TablesChoice>,
}
/// Parameter Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Parameter/x:parameter")]
pub struct Parameter {
  /// Parameter Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// SQL Data Type
  #[sdk(attr(qname = ":sqlType"))]
  pub sql_type: Option<crate::simple_type::Int32Value>,
  /// Parameter Type
  #[sdk(attr(qname = ":parameterType"))]
  pub parameter_type: Option<ParameterValues>,
  /// Refresh on Change
  #[sdk(attr(qname = ":refreshOnChange"))]
  pub refresh_on_change: Option<crate::simple_type::BooleanValue>,
  /// Parameter Prompt String
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  /// Boolean
  #[sdk(attr(qname = ":boolean"))]
  pub boolean: Option<crate::simple_type::BooleanValue>,
  /// Double
  #[sdk(attr(qname = ":double"))]
  pub double: Option<crate::simple_type::DoubleValue>,
  /// Integer
  #[sdk(attr(qname = ":integer"))]
  pub integer: Option<crate::simple_type::Int32Value>,
  /// String
  #[sdk(attr(qname = ":string"))]
  pub string: Option<crate::simple_type::StringValue>,
  /// Cell Reference
  #[sdk(attr(qname = ":cell"))]
  pub cell: Option<crate::simple_type::StringValue>,
}
/// Character Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XStringElement/x:s")]
pub struct CharacterValue {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
}
/// Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Index/x:x")]
pub struct FieldItem {
  /// Shared Items Index
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Text Import Field Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextField/x:textField")]
pub struct TextField {
  /// Field Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ExternalConnectionValues>,
  /// Position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::UInt32Value>,
}
/// PivotCache Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheField/x:cacheField")]
pub struct CacheField {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// propertyName
  #[sdk(attr(qname = ":propertyName"))]
  pub property_name: Option<crate::simple_type::StringValue>,
  /// serverField
  #[sdk(attr(qname = ":serverField"))]
  pub server_field: Option<crate::simple_type::BooleanValue>,
  /// uniqueList
  #[sdk(attr(qname = ":uniqueList"))]
  pub unique_list: Option<crate::simple_type::BooleanValue>,
  /// numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// sqlType
  #[sdk(attr(qname = ":sqlType"))]
  pub sql_type: Option<crate::simple_type::Int32Value>,
  /// hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: Option<crate::simple_type::Int32Value>,
  /// level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// databaseField
  #[sdk(attr(qname = ":databaseField"))]
  pub database_field: Option<crate::simple_type::BooleanValue>,
  /// mappingCount
  #[sdk(attr(qname = ":mappingCount"))]
  pub mapping_count: Option<crate::simple_type::UInt32Value>,
  /// memberPropertyField
  #[sdk(attr(qname = ":memberPropertyField"))]
  pub member_property_field: Option<crate::simple_type::BooleanValue>,
  /// Defines the SharedItems Class.
  #[sdk(child(qname = "x:CT_SharedItems/x:sharedItems"))]
  pub shared_items: Option<SharedItems>,
  /// Defines the FieldGroup Class.
  #[sdk(child(qname = "x:CT_FieldGroup/x:fieldGroup"))]
  pub field_group: Option<std::boxed::Box<FieldGroup>>,
  /// Defines the MemberPropertiesMap Class.
  #[sdk(child(qname = "x:CT_X/x:mpMap"))]
  pub x_mp_map: Vec<MemberPropertiesMap>,
  /// Defines the CacheFieldExtensionList Class.
  #[sdk(child(qname = "x:CT_CacheFieldExtensionList/x:extLst"))]
  pub x_ext_lst: Option<CacheFieldExtensionList>,
}
/// Page Item Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Pages/x:pages")]
pub struct Pages {
  /// Page Item String Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Page Items.
  #[sdk(child(qname = "x:CT_PCDSCPage/x:page"))]
  pub x_page: Vec<Page>,
}
/// Range Sets.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangeSets/x:rangeSets")]
pub struct RangeSets {
  /// Reference and Page Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Range Set.
  #[sdk(child(qname = "x:CT_RangeSet/x:rangeSet"))]
  pub x_range_set: Vec<RangeSet>,
}
/// Page Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDSCPage/x:page")]
pub struct Page {
  /// Page Item String Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Page Item.
  #[sdk(child(qname = "x:CT_PageItem/x:pageItem"))]
  pub x_page_item: Vec<PageItem>,
}
/// Page Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageItem/x:pageItem")]
pub struct PageItem {
  /// Page Item Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Range Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangeSet/x:rangeSet")]
pub struct RangeSet {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field Item Index Page 1
  #[sdk(attr(qname = ":i1"))]
  pub field_item_index_page1: Option<crate::simple_type::UInt32Value>,
  /// Field Item Index Page 2
  #[sdk(attr(qname = ":i2"))]
  pub field_item_index_page2: Option<crate::simple_type::UInt32Value>,
  /// Field Item index Page 3
  #[sdk(attr(qname = ":i3"))]
  pub field_item_index_page3: Option<crate::simple_type::UInt32Value>,
  /// Field Item Index Page 4
  #[sdk(attr(qname = ":i4"))]
  pub field_item_index_page4: Option<crate::simple_type::UInt32Value>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// No Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Missing/x:m")]
pub struct MissingItem {
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// background Color
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// Tuples.
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Numeric.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Number/x:n")]
pub struct NumberItem {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::DoubleValue,
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Background Color
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// Tuples.
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Boolean.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Boolean/x:b")]
pub struct BooleanItem {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::BooleanValue,
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Error Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Error/x:e")]
pub struct ErrorItem {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Item Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// background Color
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// Tuples
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub tuples: Option<Tuples>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Character Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_String/x:s")]
pub struct StringItem {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Item Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Background Color
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// Tuples.
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Date Time.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DateTime/x:d")]
pub struct DateTimeItem {
  /// Value
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::DateTimeValue,
  /// Unused Item
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item Value
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Tuples.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuples/x:tpls")]
pub struct Tuples {
  /// Member Name Count
  #[sdk(attr(qname = ":c"))]
  pub member_name_count: Option<crate::simple_type::UInt32Value>,
  /// Tuple.
  #[sdk(child(qname = "x:CT_Tuple/x:tpl"))]
  pub x_tpl: Vec<Tuple>,
}
/// Sort By Tuple.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuples/x:sortByTuple")]
pub struct SortByTuple {
  /// Member Name Count
  #[sdk(attr(qname = ":c"))]
  pub member_name_count: Option<crate::simple_type::UInt32Value>,
  /// Tuple.
  #[sdk(child(qname = "x:CT_Tuple/x:tpl"))]
  pub x_tpl: Vec<Tuple>,
}
/// Member Property Indexes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_X/x:x")]
pub struct MemberPropertyIndex {
  /// Shared Items Index
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// Defines the MemberPropertiesMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_X/x:mpMap")]
pub struct MemberPropertiesMap {
  /// Shared Items Index
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// PivotCache Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Record/x:r")]
pub struct PivotCacheRecord {
  #[sdk(choice(
    qname = "x:CT_Missing/x:m",
    qname = "x:CT_Number/x:n",
    qname = "x:CT_Boolean/x:b",
    qname = "x:CT_Error/x:e",
    qname = "x:CT_String/x:s",
    qname = "x:CT_DateTime/x:d",
    qname = "x:CT_Index/x:x"
  ))]
  pub pivot_cache_record_choice: Vec<PivotCacheRecordChoice>,
}
/// OLAP KPI.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDKPI/x:kpi")]
pub struct Kpi {
  /// KPI Unique Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// KPI Display Name
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// KPI Display Folder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// KPI Measure Group Name
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// Parent KPI
  #[sdk(attr(qname = ":parent"))]
  pub parent_kpi: Option<crate::simple_type::StringValue>,
  /// KPI Value Unique Name
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::StringValue,
  /// KPI Goal Unique Name
  #[sdk(attr(qname = ":goal"))]
  pub goal: Option<crate::simple_type::StringValue>,
  /// KPI Status Unique Name
  #[sdk(attr(qname = ":status"))]
  pub status: Option<crate::simple_type::StringValue>,
  /// KPI Trend Unique Name
  #[sdk(attr(qname = ":trend"))]
  pub trend: Option<crate::simple_type::StringValue>,
  /// KPI Weight Unique Name
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
}
/// PivotCache Field Id.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldUsage/x:fieldUsage")]
pub struct FieldUsage {
  /// Field Index
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::Int32Value,
}
/// OLAP Grouping Levels.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupLevel/x:groupLevel")]
pub struct GroupLevel {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Unique Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Grouping Level Display Name
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// User-Defined Group Level
  #[sdk(attr(qname = ":user"))]
  pub user: Option<crate::simple_type::BooleanValue>,
  /// Custom Roll Up
  #[sdk(attr(qname = ":customRollUp"))]
  pub custom_roll_up: Option<crate::simple_type::BooleanValue>,
  /// OLAP Level Groups
  #[sdk(child(qname = "x:CT_Groups/x:groups"))]
  pub groups: Option<Groups>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// OLAP Level Groups.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Groups/x:groups")]
pub struct Groups {
  /// Level Group Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Group.
  #[sdk(child(qname = "x:CT_LevelGroup/x:group"))]
  pub x_group: Vec<Group>,
}
/// OLAP Group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LevelGroup/x:group")]
pub struct Group {
  /// Group Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Unique Group Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Group Caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// Parent Unique Name
  #[sdk(attr(qname = ":uniqueParent"))]
  pub unique_parent: Option<crate::simple_type::StringValue>,
  /// Group Id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::Int32Value>,
  /// OLAP Group Members
  #[sdk(child(qname = "x:CT_GroupMembers/x:groupMembers"))]
  pub group_members: std::boxed::Box<GroupMembers>,
}
/// OLAP Group Members.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupMembers/x:groupMembers")]
pub struct GroupMembers {
  /// Group Member Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Group Member.
  #[sdk(child(qname = "x:CT_GroupMember/x:groupMember"))]
  pub x_group_member: Vec<GroupMember>,
}
/// OLAP Group Member.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupMember/x:groupMember")]
pub struct GroupMember {
  /// Group Member Unique Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Group
  #[sdk(attr(qname = ":group"))]
  pub group: Option<crate::simple_type::BooleanValue>,
}
/// Entries.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDSDTCEntries/x:entries")]
pub struct Entries {
  /// Tuple Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_Missing/x:m",
    qname = "x:CT_Number/x:n",
    qname = "x:CT_Error/x:e",
    qname = "x:CT_String/x:s"
  ))]
  pub entries_choice: Vec<EntriesChoice>,
}
/// Sets.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sets/x:sets")]
pub struct Sets {
  /// Tuple Set Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Set.
  #[sdk(child(qname = "x:CT_Set/x:set"))]
  pub x_set: Vec<TupleSet>,
}
/// OLAP Query Cache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryCache/x:queryCache")]
pub struct QueryCache {
  /// Cached Query Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Query.
  #[sdk(child(qname = "x:CT_Query/x:query"))]
  pub x_query: Vec<Query>,
}
/// Server Formats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ServerFormats/x:serverFormats")]
pub struct ServerFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Server Format.
  #[sdk(child(qname = "x:CT_ServerFormat/x:serverFormat"))]
  pub x_server_format: Vec<ServerFormat>,
}
/// Server Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ServerFormat/x:serverFormat")]
pub struct ServerFormat {
  /// Culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// Format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Tuple.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuple/x:tpl")]
pub struct Tuple {
  /// Field Index
  #[sdk(attr(qname = ":fld"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Hierarchy Index
  #[sdk(attr(qname = ":hier"))]
  pub hierarchy: Option<crate::simple_type::UInt32Value>,
  /// Item Index
  #[sdk(attr(qname = ":item"))]
  pub item: crate::simple_type::UInt32Value,
}
/// OLAP Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Set/x:set")]
pub struct TupleSet {
  /// Number of Tuples
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Maximum Rank Requested
  #[sdk(attr(qname = ":maxRank"))]
  pub max_rank: crate::simple_type::Int32Value,
  /// MDX Set Definition
  #[sdk(attr(qname = ":setDefinition"))]
  pub set_definition: crate::simple_type::StringValue,
  /// Set Sort Order
  #[sdk(attr(qname = ":sortType"))]
  pub sort_type: Option<SortValues>,
  /// Query Failed
  #[sdk(attr(qname = ":queryFailed"))]
  pub query_failed: Option<crate::simple_type::BooleanValue>,
  /// Tuples.
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// Sort By Tuple.
  #[sdk(child(qname = "x:CT_Tuples/x:sortByTuple"))]
  pub x_sort_by_tuple: Option<SortByTuple>,
}
/// Query.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Query/x:query")]
pub struct Query {
  /// MDX Query String
  #[sdk(attr(qname = ":mdx"))]
  pub mdx: crate::simple_type::StringValue,
  /// Tuples
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub tuples: Option<Tuples>,
}
/// Calculated Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedItem/x:calculatedItem")]
pub struct CalculatedItem {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Calculated Item Formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// Calculated Item Location
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Item Location.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotArea/x:pivotArea")]
pub struct PivotArea {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::Int32Value>,
  /// Rule Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<PivotAreaValues>,
  /// Data Only
  #[sdk(attr(qname = ":dataOnly"))]
  pub data_only: Option<crate::simple_type::BooleanValue>,
  /// Labels Only
  #[sdk(attr(qname = ":labelOnly"))]
  pub label_only: Option<crate::simple_type::BooleanValue>,
  /// Include Row Grand Total
  #[sdk(attr(qname = ":grandRow"))]
  pub grand_row: Option<crate::simple_type::BooleanValue>,
  /// Include Column Grand Total
  #[sdk(attr(qname = ":grandCol"))]
  pub grand_column: Option<crate::simple_type::BooleanValue>,
  /// Cache Index
  #[sdk(attr(qname = ":cacheIndex"))]
  pub cache_index: Option<crate::simple_type::BooleanValue>,
  /// Outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Offset Reference
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Collapsed Levels Are Subtotals
  #[sdk(attr(qname = ":collapsedLevelsAreSubtotals"))]
  pub collapsed_levels_are_subtotals: Option<crate::simple_type::BooleanValue>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Field Position
  #[sdk(attr(qname = ":fieldPosition"))]
  pub field_position: Option<crate::simple_type::UInt32Value>,
  /// References
  #[sdk(child(qname = "x:CT_PivotAreaReferences/x:references"))]
  pub pivot_area_references: Option<PivotAreaReferences>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Member.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMember/x:calculatedMember")]
pub struct CalculatedMember {
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// mdx
  #[sdk(attr(qname = ":mdx"))]
  pub mdx: crate::simple_type::StringValue,
  /// memberName
  #[sdk(attr(qname = ":memberName"))]
  pub member_name: Option<crate::simple_type::StringValue>,
  /// hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: Option<crate::simple_type::StringValue>,
  /// parent
  #[sdk(attr(qname = ":parent"))]
  pub parent_name: Option<crate::simple_type::StringValue>,
  /// solveOrder
  #[sdk(attr(qname = ":solveOrder"))]
  pub solve_order: Option<crate::simple_type::Int32Value>,
  /// set
  #[sdk(attr(qname = ":set"))]
  pub set: Option<crate::simple_type::BooleanValue>,
  /// Defines the CalculatedMemberExtensionList Class.
  #[sdk(child(qname = "x:CT_CalculatedMemberExtensionList/x:extLst"))]
  pub calculated_member_extension_list: Option<CalculatedMemberExtensionList>,
}
/// PivotTable Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotField/x:pivotField")]
pub struct PivotField {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Data Field
  #[sdk(attr(qname = ":dataField"))]
  pub data_field: Option<crate::simple_type::BooleanValue>,
  /// Custom Subtotal Caption
  #[sdk(attr(qname = ":subtotalCaption"))]
  pub subtotal_caption: Option<crate::simple_type::StringValue>,
  /// Show PivotField Header Drop Downs
  #[sdk(attr(qname = ":showDropDowns"))]
  pub show_drop_downs: Option<crate::simple_type::BooleanValue>,
  /// Hidden Level
  #[sdk(attr(qname = ":hiddenLevel"))]
  pub hidden_level: Option<crate::simple_type::BooleanValue>,
  /// Unique Member Property
  #[sdk(attr(qname = ":uniqueMemberProperty"))]
  pub unique_member_property: Option<crate::simple_type::StringValue>,
  /// Compact
  #[sdk(attr(qname = ":compact"))]
  pub compact: Option<crate::simple_type::BooleanValue>,
  /// All Items Expanded
  #[sdk(attr(qname = ":allDrilled"))]
  pub all_drilled: Option<crate::simple_type::BooleanValue>,
  /// Number Format Id
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Outline Items
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Subtotals At Top
  #[sdk(attr(qname = ":subtotalTop"))]
  pub subtotal_top: Option<crate::simple_type::BooleanValue>,
  /// Drag To Row
  #[sdk(attr(qname = ":dragToRow"))]
  pub drag_to_row: Option<crate::simple_type::BooleanValue>,
  /// Drag To Column
  #[sdk(attr(qname = ":dragToCol"))]
  pub drag_to_column: Option<crate::simple_type::BooleanValue>,
  /// Multiple Field Filters
  #[sdk(attr(qname = ":multipleItemSelectionAllowed"))]
  pub multiple_item_selection_allowed: Option<crate::simple_type::BooleanValue>,
  /// Drag Field to Page
  #[sdk(attr(qname = ":dragToPage"))]
  pub drag_to_page: Option<crate::simple_type::BooleanValue>,
  /// Field Can Drag to Data
  #[sdk(attr(qname = ":dragToData"))]
  pub drag_to_data: Option<crate::simple_type::BooleanValue>,
  /// Drag Off
  #[sdk(attr(qname = ":dragOff"))]
  pub drag_off: Option<crate::simple_type::BooleanValue>,
  /// Show All Items
  #[sdk(attr(qname = ":showAll"))]
  pub show_all: Option<crate::simple_type::BooleanValue>,
  /// Insert Blank Row
  #[sdk(attr(qname = ":insertBlankRow"))]
  pub insert_blank_row: Option<crate::simple_type::BooleanValue>,
  /// Server-based Page Field
  #[sdk(attr(qname = ":serverField"))]
  pub server_field: Option<crate::simple_type::BooleanValue>,
  /// Insert Item Page Break
  #[sdk(attr(qname = ":insertPageBreak"))]
  pub insert_page_break: Option<crate::simple_type::BooleanValue>,
  /// Auto Show
  #[sdk(attr(qname = ":autoShow"))]
  pub auto_show: Option<crate::simple_type::BooleanValue>,
  /// Top Auto Show
  #[sdk(attr(qname = ":topAutoShow"))]
  pub top_auto_show: Option<crate::simple_type::BooleanValue>,
  /// Hide New Items
  #[sdk(attr(qname = ":hideNewItems"))]
  pub hide_new_items: Option<crate::simple_type::BooleanValue>,
  /// Measure Filter
  #[sdk(attr(qname = ":measureFilter"))]
  pub measure_filter: Option<crate::simple_type::BooleanValue>,
  /// Inclusive Manual Filter
  #[sdk(attr(qname = ":includeNewItemsInFilter"))]
  pub include_new_items_in_filter: Option<crate::simple_type::BooleanValue>,
  /// Items Per Page Count
  #[sdk(attr(qname = ":itemPageCount"))]
  pub item_page_count: Option<crate::simple_type::UInt32Value>,
  /// Auto Sort Type
  #[sdk(attr(qname = ":sortType"))]
  pub sort_type: Option<FieldSortValues>,
  /// Data Source Sort
  #[sdk(attr(qname = ":dataSourceSort"))]
  pub data_source_sort: Option<crate::simple_type::BooleanValue>,
  /// Auto Sort
  #[sdk(attr(qname = ":nonAutoSortDefault"))]
  pub non_auto_sort_default: Option<crate::simple_type::BooleanValue>,
  /// Auto Show Rank By
  #[sdk(attr(qname = ":rankBy"))]
  pub rank_by: Option<crate::simple_type::UInt32Value>,
  /// Show Default Subtotal
  #[sdk(attr(qname = ":defaultSubtotal"))]
  pub default_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Sum Subtotal
  #[sdk(attr(qname = ":sumSubtotal"))]
  pub sum_subtotal: Option<crate::simple_type::BooleanValue>,
  /// CountA
  #[sdk(attr(qname = ":countASubtotal"))]
  pub count_a_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Average
  #[sdk(attr(qname = ":avgSubtotal"))]
  pub average_sub_total: Option<crate::simple_type::BooleanValue>,
  /// Max Subtotal
  #[sdk(attr(qname = ":maxSubtotal"))]
  pub max_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Min Subtotal
  #[sdk(attr(qname = ":minSubtotal"))]
  pub min_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Product Subtotal
  #[sdk(attr(qname = ":productSubtotal"))]
  pub apply_product_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Count
  #[sdk(attr(qname = ":countSubtotal"))]
  pub count_subtotal: Option<crate::simple_type::BooleanValue>,
  /// StdDev Subtotal
  #[sdk(attr(qname = ":stdDevSubtotal"))]
  pub apply_standard_deviation_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// StdDevP Subtotal
  #[sdk(attr(qname = ":stdDevPSubtotal"))]
  pub apply_standard_deviation_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Variance Subtotal
  #[sdk(attr(qname = ":varSubtotal"))]
  pub apply_variance_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// VarP Subtotal
  #[sdk(attr(qname = ":varPSubtotal"))]
  pub apply_variance_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Show Member Property in Cell
  #[sdk(attr(qname = ":showPropCell"))]
  pub show_prop_cell: Option<crate::simple_type::BooleanValue>,
  /// Show Member Property ToolTip
  #[sdk(attr(qname = ":showPropTip"))]
  pub show_property_tooltip: Option<crate::simple_type::BooleanValue>,
  /// Show As Caption
  #[sdk(attr(qname = ":showPropAsCaption"))]
  pub show_prop_as_caption: Option<crate::simple_type::BooleanValue>,
  /// Drill State
  #[sdk(attr(qname = ":defaultAttributeDrillState"))]
  pub default_attribute_drill_state: Option<crate::simple_type::BooleanValue>,
  /// Field Items
  #[sdk(child(qname = "x:CT_Items/x:items"))]
  pub items: Option<Items>,
  /// AutoSort Scope
  #[sdk(child(qname = "x:CT_AutoSortScope/x:autoSortScope"))]
  pub auto_sort_scope: Option<std::boxed::Box<AutoSortScope>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_PivotFieldExtensionList/x:extLst"))]
  pub pivot_field_extension_list: Option<PivotFieldExtensionList>,
}
/// PivotTable Field Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Item/x:item")]
pub struct Item {
  /// Item User Caption
  #[sdk(attr(qname = ":n"))]
  pub item_name: Option<crate::simple_type::StringValue>,
  /// Item Type
  #[sdk(attr(qname = ":t"))]
  pub item_type: Option<ItemValues>,
  /// Hidden
  #[sdk(attr(qname = ":h"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Character
  #[sdk(attr(qname = ":s"))]
  pub has_string_vlue: Option<crate::simple_type::BooleanValue>,
  /// Hide Details
  #[sdk(attr(qname = ":sd"))]
  pub hide_details: Option<crate::simple_type::BooleanValue>,
  /// Calculated Member
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Missing
  #[sdk(attr(qname = ":m"))]
  pub missing: Option<crate::simple_type::BooleanValue>,
  /// Child Items
  #[sdk(attr(qname = ":c"))]
  pub child_items: Option<crate::simple_type::BooleanValue>,
  /// Item Index
  #[sdk(attr(qname = ":x"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// Expanded
  #[sdk(attr(qname = ":d"))]
  pub expanded: Option<crate::simple_type::BooleanValue>,
  /// Drill Across Attributes
  #[sdk(attr(qname = ":e"))]
  pub drill_across_attributes: Option<crate::simple_type::BooleanValue>,
}
/// Data Field Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataField/x:dataField")]
pub struct DataField {
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// fld
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::UInt32Value,
  /// subtotal
  #[sdk(attr(qname = ":subtotal"))]
  pub subtotal: Option<DataConsolidateFunctionValues>,
  /// showDataAs
  #[sdk(attr(qname = ":showDataAs"))]
  pub show_data_as: Option<ShowDataAsValues>,
  /// baseField
  #[sdk(attr(qname = ":baseField"))]
  pub base_field: Option<crate::simple_type::Int32Value>,
  /// baseItem
  #[sdk(attr(qname = ":baseItem"))]
  pub base_item: Option<crate::simple_type::UInt32Value>,
  /// numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Defines the DataFieldExtensionList Class.
  #[sdk(child(qname = "x:CT_DataFieldExtensionList/x:extLst"))]
  pub data_field_extension_list: Option<DataFieldExtensionList>,
}
/// Row Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_I/x:i")]
pub struct RowItem {
  /// Item Type
  #[sdk(attr(qname = ":t"))]
  pub item_type: Option<ItemValues>,
  /// Repeated Items Count
  #[sdk(attr(qname = ":r"))]
  pub repeated_item_count: Option<crate::simple_type::UInt32Value>,
  /// Data Field Index
  #[sdk(attr(qname = ":i"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// Member Property Indexes.
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Row Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Field/x:field")]
pub struct Field {
  /// Field Index
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::Int32Value,
}
/// PivotTable Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Format/x:format")]
pub struct Format {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Format Action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<FormatActionValues>,
  /// Format Id
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Pivot Table Location
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Conditional Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormat/x:conditionalFormat")]
pub struct ConditionalFormat {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Conditional Formatting Scope
  #[sdk(attr(qname = ":scope"))]
  pub scope: Option<ScopeValues>,
  /// Conditional Formatting Rule Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<RuleValues>,
  /// Priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: crate::simple_type::UInt32Value,
  /// Pivot Areas
  #[sdk(child(qname = "x:CT_PivotAreas/x:pivotAreas"))]
  pub pivot_areas: std::boxed::Box<PivotAreas>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Pivot Areas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreas/x:pivotAreas")]
pub struct PivotAreas {
  /// Pivot Area Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Calculated Item Location.
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub x_pivot_area: Vec<PivotArea>,
}
/// PivotChart Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartFormat/x:chartFormat")]
pub struct ChartFormat {
  /// Chart Index
  #[sdk(attr(qname = ":chart"))]
  pub chart: crate::simple_type::UInt32Value,
  /// Pivot Format Id
  #[sdk(attr(qname = ":format"))]
  pub format: crate::simple_type::UInt32Value,
  /// Series Format
  #[sdk(attr(qname = ":series"))]
  pub series: Option<crate::simple_type::BooleanValue>,
  /// Pivot Table Location Rule
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// OLAP Hierarchy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchy/x:pivotHierarchy")]
pub struct PivotHierarchy {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Outline New Levels
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Multiple Field Filters
  #[sdk(attr(qname = ":multipleItemSelectionAllowed"))]
  pub multiple_item_selection_allowed: Option<crate::simple_type::BooleanValue>,
  /// New Levels Subtotals At Top
  #[sdk(attr(qname = ":subtotalTop"))]
  pub subtotal_top: Option<crate::simple_type::BooleanValue>,
  /// Show In Field List
  #[sdk(attr(qname = ":showInFieldList"))]
  pub show_in_field_list: Option<crate::simple_type::BooleanValue>,
  /// Drag To Row
  #[sdk(attr(qname = ":dragToRow"))]
  pub drag_to_row: Option<crate::simple_type::BooleanValue>,
  /// Drag To Column
  #[sdk(attr(qname = ":dragToCol"))]
  pub drag_to_column: Option<crate::simple_type::BooleanValue>,
  /// Drag to Page
  #[sdk(attr(qname = ":dragToPage"))]
  pub drag_to_page: Option<crate::simple_type::BooleanValue>,
  /// Drag To Data
  #[sdk(attr(qname = ":dragToData"))]
  pub drag_to_data: Option<crate::simple_type::BooleanValue>,
  /// Drag Off
  #[sdk(attr(qname = ":dragOff"))]
  pub drag_off: Option<crate::simple_type::BooleanValue>,
  /// Inclusive Manual Filter
  #[sdk(attr(qname = ":includeNewItemsInFilter"))]
  pub include_new_items_in_filter: Option<crate::simple_type::BooleanValue>,
  /// Hierarchy Caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// OLAP Member Properties
  #[sdk(child(qname = "x:CT_MemberProperties/x:mps"))]
  pub member_properties: Option<MemberProperties>,
  /// Members.
  #[sdk(child(qname = "x:CT_Members/x:members"))]
  pub x_members: Vec<Members>,
  /// Future Feature Data Storage Area.
  #[sdk(child(qname = "x:CT_PivotHierarchyExtensionList/x:extLst"))]
  pub x_ext_lst: Option<PivotHierarchyExtensionList>,
}
/// Row OLAP Hierarchies.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HierarchyUsage/x:rowHierarchyUsage")]
pub struct RowHierarchyUsage {
  /// Hierarchy Usage
  #[sdk(attr(qname = ":hierarchyUsage"))]
  pub value: crate::simple_type::Int32Value,
}
/// Column OLAP Hierarchies.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HierarchyUsage/x:colHierarchyUsage")]
pub struct ColumnHierarchyUsage {
  /// Hierarchy Usage
  #[sdk(attr(qname = ":hierarchyUsage"))]
  pub value: crate::simple_type::Int32Value,
}
/// OLAP Member Property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MemberProperty/x:mp")]
pub struct MemberProperty {
  /// OLAP Member Property Unique Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show Cell
  #[sdk(attr(qname = ":showCell"))]
  pub show_cell: Option<crate::simple_type::BooleanValue>,
  /// Show Tooltip
  #[sdk(attr(qname = ":showTip"))]
  pub show_tip: Option<crate::simple_type::BooleanValue>,
  /// Show As Caption
  #[sdk(attr(qname = ":showAsCaption"))]
  pub show_as_caption: Option<crate::simple_type::BooleanValue>,
  /// Name Length
  #[sdk(attr(qname = ":nameLen"))]
  pub name_length: Option<crate::simple_type::UInt32Value>,
  /// Property Name Character Index
  #[sdk(attr(qname = ":pPos"))]
  pub property_name_position: Option<crate::simple_type::UInt32Value>,
  /// Property Name Length
  #[sdk(attr(qname = ":pLen"))]
  pub property_name_length: Option<crate::simple_type::UInt32Value>,
  /// Level Index
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: crate::simple_type::UInt32Value,
}
/// Member.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Member/x:member")]
pub struct Member {
  /// Hidden Item Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// OLAP Dimension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotDimension/x:dimension")]
pub struct Dimension {
  /// Measure
  #[sdk(attr(qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
  /// Dimension Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Dimension Unique Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Dimension Display Name
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
}
/// OLAP Measure Group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureGroup/x:measureGroup")]
pub struct MeasureGroup {
  /// Measure Group Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Measure Group Display Name
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
}
/// OLAP Measure Group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureDimensionMap/x:map")]
pub struct MeasureDimensionMap {
  /// Measure Group Id
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: crate::simple_type::UInt32Value,
  /// Dimension Id
  #[sdk(attr(qname = ":dimension"))]
  pub dimension: crate::simple_type::UInt32Value,
}
/// PivotTable Advanced Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilter/x:filter")]
pub struct PivotFilter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// fld
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::UInt32Value,
  /// mpFld
  #[sdk(attr(qname = ":mpFld"))]
  pub member_property_field_id: Option<crate::simple_type::UInt32Value>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: PivotFilterValues,
  /// evalOrder
  #[sdk(attr(qname = ":evalOrder"))]
  pub evaluation_order: Option<crate::simple_type::Int32Value>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// iMeasureHier
  #[sdk(attr(qname = ":iMeasureHier"))]
  pub measure_hierarchy: Option<crate::simple_type::UInt32Value>,
  /// iMeasureFld
  #[sdk(attr(qname = ":iMeasureFld"))]
  pub measure_field: Option<crate::simple_type::UInt32Value>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// stringValue1
  #[sdk(attr(qname = ":stringValue1"))]
  pub string_value1: Option<crate::simple_type::StringValue>,
  /// stringValue2
  #[sdk(attr(qname = ":stringValue2"))]
  pub string_value2: Option<crate::simple_type::StringValue>,
  /// AutoFilter Settings.
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: std::boxed::Box<AutoFilter>,
  /// Defines the PivotFilterExtensionList Class.
  #[sdk(child(qname = "x:CT_PivotFilterExtensionList/x:extLst"))]
  pub pivot_filter_extension_list: Option<PivotFilterExtensionList>,
}
/// PivotCache Hierarchy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchy/x:cacheHierarchy")]
pub struct CacheHierarchy {
  pub xml_other_attrs: Vec<(String, String)>,
  /// uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// measure
  #[sdk(attr(qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
  /// set
  #[sdk(attr(qname = ":set"))]
  pub set: Option<crate::simple_type::BooleanValue>,
  /// parentSet
  #[sdk(attr(qname = ":parentSet"))]
  pub parent_set: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<crate::simple_type::Int32Value>,
  /// attribute
  #[sdk(attr(qname = ":attribute"))]
  pub attribute: Option<crate::simple_type::BooleanValue>,
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::BooleanValue>,
  /// keyAttribute
  #[sdk(attr(qname = ":keyAttribute"))]
  pub key_attribute: Option<crate::simple_type::BooleanValue>,
  /// defaultMemberUniqueName
  #[sdk(attr(qname = ":defaultMemberUniqueName"))]
  pub default_member_unique_name: Option<crate::simple_type::StringValue>,
  /// allUniqueName
  #[sdk(attr(qname = ":allUniqueName"))]
  pub all_unique_name: Option<crate::simple_type::StringValue>,
  /// allCaption
  #[sdk(attr(qname = ":allCaption"))]
  pub all_caption: Option<crate::simple_type::StringValue>,
  /// dimensionUniqueName
  #[sdk(attr(qname = ":dimensionUniqueName"))]
  pub dimension_unique_name: Option<crate::simple_type::StringValue>,
  /// displayFolder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// measureGroup
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// measures
  #[sdk(attr(qname = ":measures"))]
  pub measures: Option<crate::simple_type::BooleanValue>,
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// oneField
  #[sdk(attr(qname = ":oneField"))]
  pub one_field: Option<crate::simple_type::BooleanValue>,
  /// memberValueDatatype
  #[sdk(attr(qname = ":memberValueDatatype"))]
  pub member_value_datatype: Option<crate::simple_type::UInt16Value>,
  /// unbalanced
  #[sdk(attr(qname = ":unbalanced"))]
  pub unbalanced: Option<crate::simple_type::BooleanValue>,
  /// unbalancedGroup
  #[sdk(attr(qname = ":unbalancedGroup"))]
  pub unbalanced_group: Option<crate::simple_type::BooleanValue>,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Defines the FieldsUsage Class.
  #[sdk(child(qname = "x:CT_FieldsUsage/x:fieldsUsage"))]
  pub fields_usage: Option<FieldsUsage>,
  /// Defines the GroupLevels Class.
  #[sdk(child(qname = "x:CT_GroupLevels/x:groupLevels"))]
  pub group_levels: Option<GroupLevels>,
  /// Defines the CacheHierarchyExtensionList Class.
  #[sdk(child(qname = "x:CT_CacheHierarchyExtensionList/x:extLst"))]
  pub cache_hierarchy_extension_list: Option<CacheHierarchyExtensionList>,
}
/// Range Grouping Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangePr/x:rangePr")]
pub struct RangeProperties {
  /// Source Data Set Beginning Range
  #[sdk(attr(qname = ":autoStart"))]
  pub auto_start: Option<crate::simple_type::BooleanValue>,
  /// Source Data Ending Range
  #[sdk(attr(qname = ":autoEnd"))]
  pub auto_end: Option<crate::simple_type::BooleanValue>,
  /// Group By
  #[sdk(attr(qname = ":groupBy"))]
  pub group_by: Option<GroupByValues>,
  /// Numeric Grouping Start Value
  #[sdk(attr(qname = ":startNum"))]
  pub start_number: Option<crate::simple_type::DoubleValue>,
  /// Numeric Grouping End Value
  #[sdk(attr(qname = ":endNum"))]
  pub end_num: Option<crate::simple_type::DoubleValue>,
  /// Date Grouping Start Value
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// Date Grouping End Value
  #[sdk(attr(qname = ":endDate"))]
  pub end_date: Option<crate::simple_type::DateTimeValue>,
  /// Grouping Interval
  #[sdk(attr(qname = ":groupInterval"))]
  pub group_interval: Option<crate::simple_type::DoubleValue>,
}
/// Discrete Grouping Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DiscretePr/x:discretePr")]
pub struct DiscreteProperties {
  /// Mapping Index Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Index.
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  pub x_x: Vec<FieldItem>,
}
/// OLAP Group Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupItems/x:groupItems")]
pub struct GroupItems {
  /// Items Created Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_Missing/x:m",
    qname = "x:CT_Number/x:n",
    qname = "x:CT_Boolean/x:b",
    qname = "x:CT_Error/x:e",
    qname = "x:CT_String/x:s",
    qname = "x:CT_DateTime/x:d"
  ))]
  pub group_items_choice: Vec<GroupItemsChoice>,
}
/// Page Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageField/x:pageField")]
pub struct PageField {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::Int32Value,
  /// Item Index
  #[sdk(attr(qname = ":item"))]
  pub item: Option<crate::simple_type::UInt32Value>,
  /// OLAP Hierarchy Index
  #[sdk(attr(qname = ":hier"))]
  pub hierarchy: Option<crate::simple_type::Int32Value>,
  /// Hierarchy Unique Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Hierarchy Display Name
  #[sdk(attr(qname = ":cap"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// References.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreaReferences/x:references")]
pub struct PivotAreaReferences {
  /// Pivot Filter Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Reference.
  #[sdk(child(qname = "x:CT_PivotAreaReference/x:reference"))]
  pub x_reference: Vec<PivotAreaReference>,
}
/// Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreaReference/x:reference")]
pub struct PivotAreaReference {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Item Index Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Selected
  #[sdk(attr(qname = ":selected"))]
  pub selected: Option<crate::simple_type::BooleanValue>,
  /// Positional Reference
  #[sdk(attr(qname = ":byPosition"))]
  pub by_position: Option<crate::simple_type::BooleanValue>,
  /// Relative Reference
  #[sdk(attr(qname = ":relative"))]
  pub relative: Option<crate::simple_type::BooleanValue>,
  /// Include Default Filter
  #[sdk(attr(qname = ":defaultSubtotal"))]
  pub default_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Sum Filter
  #[sdk(attr(qname = ":sumSubtotal"))]
  pub sum_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include CountA Filter
  #[sdk(attr(qname = ":countASubtotal"))]
  pub count_a_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Average Filter
  #[sdk(attr(qname = ":avgSubtotal"))]
  pub average_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Maximum Filter
  #[sdk(attr(qname = ":maxSubtotal"))]
  pub max_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Minimum Filter
  #[sdk(attr(qname = ":minSubtotal"))]
  pub min_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Product Filter
  #[sdk(attr(qname = ":productSubtotal"))]
  pub apply_product_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Count Subtotal
  #[sdk(attr(qname = ":countSubtotal"))]
  pub count_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include StdDev Filter
  #[sdk(attr(qname = ":stdDevSubtotal"))]
  pub apply_standard_deviation_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include StdDevP Filter
  #[sdk(attr(qname = ":stdDevPSubtotal"))]
  pub apply_standard_deviation_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Var Filter
  #[sdk(attr(qname = ":varSubtotal"))]
  pub apply_variance_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include VarP Filter
  #[sdk(attr(qname = ":varPSubtotal"))]
  pub apply_variance_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Index.
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  pub x_x: Vec<FieldItem>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Query table fields.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableFields/x:queryTableFields")]
pub struct QueryTableFields {
  /// Column Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// QueryTable Field.
  #[sdk(child(qname = "x:CT_QueryTableField/x:queryTableField"))]
  pub x_query_table_field: Vec<QueryTableField>,
}
/// Deleted Fields.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableDeletedFields/x:queryTableDeletedFields")]
pub struct QueryTableDeletedFields {
  /// Deleted Fields Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Deleted Field.
  #[sdk(child(qname = "x:CT_DeletedField/x:deletedField"))]
  pub x_deleted_field: Vec<DeletedField>,
}
/// Deleted Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DeletedField/x:deletedField")]
pub struct DeletedField {
  /// Deleted Fields Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// QueryTable Field.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableField/x:queryTableField")]
pub struct QueryTableField {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Field Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Data Bound Column
  #[sdk(attr(qname = ":dataBound"))]
  pub data_bound: Option<crate::simple_type::BooleanValue>,
  /// Row Numbers
  #[sdk(attr(qname = ":rowNumbers"))]
  pub row_numbers: Option<crate::simple_type::BooleanValue>,
  /// Fill This Formula On Refresh
  #[sdk(attr(qname = ":fillFormulas"))]
  pub fill_formulas: Option<crate::simple_type::BooleanValue>,
  /// Clipped Column
  #[sdk(attr(qname = ":clipped"))]
  pub clipped: Option<crate::simple_type::BooleanValue>,
  /// Table Column Id
  #[sdk(attr(qname = ":tableColumnId"))]
  pub table_column_id: Option<crate::simple_type::UInt32Value>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// String Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:si")]
pub struct SharedStringItem {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// Rich Text Run.
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// Phonetic Run.
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Rich Text Inline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:is")]
pub struct InlineString {
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// Rich Text Run.
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// Phonetic Run.
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Comment Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:text")]
pub struct CommentText {
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// Rich Text Run.
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// Phonetic Run.
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Bold.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:b")]
pub struct Bold {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Italic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:i")]
pub struct Italic {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Strike Through.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:strike")]
pub struct Strike {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Condense.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:condense")]
pub struct Condense {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Extend.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:extend")]
pub struct Extend {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Outline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:outline")]
pub struct Outline {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Shadow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:shadow")]
pub struct Shadow {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Underline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_UnderlineProperty/x:u")]
pub struct Underline {
  /// Underline Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<UnderlineValues>,
}
/// Vertical Alignment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign")]
pub struct VerticalTextAlignment {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: VerticalAlignmentRunValues,
}
/// Font Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontSize/x:sz")]
pub struct FontSize {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Text Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:color")]
pub struct Color {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Sheet Tab Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:tabColor")]
pub struct TabColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Foreground Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:fgColor")]
pub struct ForegroundColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Background Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:bgColor")]
pub struct BackgroundColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Font.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontName/x:rFont")]
pub struct RunFont {
  /// String Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Font Family.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IntProperty/x:family")]
pub struct FontFamily {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Character Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IntProperty/x:charset")]
pub struct RunPropertyCharSet {
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Font Scheme.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontScheme/x:scheme")]
pub struct FontScheme {
  /// Font Scheme
  #[sdk(attr(qname = ":val"))]
  pub val: FontSchemeValues,
}
/// Run Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RPrElt/x:rPr")]
pub struct RunProperties {
  /// Bold.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:b"))]
  pub x_b: Vec<Bold>,
  /// Italic.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:i"))]
  pub x_i: Vec<Italic>,
  /// Strike Through.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:strike"))]
  pub x_strike: Vec<Strike>,
  /// Condense.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:condense"))]
  pub x_condense: Vec<Condense>,
  /// Extend.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:extend"))]
  pub x_extend: Vec<Extend>,
  /// Outline.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:outline"))]
  pub x_outline: Vec<Outline>,
  /// Shadow.
  #[sdk(child(qname = "x:CT_BooleanProperty/x:shadow"))]
  pub x_shadow: Vec<Shadow>,
  /// Underline.
  #[sdk(child(qname = "x:CT_UnderlineProperty/x:u"))]
  pub x_u: Vec<Underline>,
  /// Vertical Alignment.
  #[sdk(child(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign"))]
  pub x_vert_align: Vec<VerticalTextAlignment>,
  /// Font Size.
  #[sdk(child(qname = "x:CT_FontSize/x:sz"))]
  pub x_sz: Vec<FontSize>,
  /// Text Color.
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
  /// Font.
  #[sdk(child(qname = "x:CT_FontName/x:rFont"))]
  pub x_r_font: Vec<RunFont>,
  /// Font Family.
  #[sdk(child(qname = "x:CT_IntProperty/x:family"))]
  pub x_family: Vec<FontFamily>,
  /// Character Set.
  #[sdk(child(qname = "x:CT_IntProperty/x:charset"))]
  pub x_charset: Vec<RunPropertyCharSet>,
  /// Font Scheme.
  #[sdk(child(qname = "x:CT_FontScheme/x:scheme"))]
  pub x_scheme: Vec<FontScheme>,
}
/// Rich Text Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RElt/x:r")]
pub struct Run {
  /// Run Properties
  #[sdk(child(qname = "x:CT_RPrElt/x:rPr"))]
  pub run_properties: Option<RunProperties>,
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: std::boxed::Box<Text>,
}
/// Phonetic Run.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PhoneticRun/x:rPh")]
pub struct PhoneticRun {
  /// Base Text Start Index
  #[sdk(attr(qname = ":sb"))]
  pub base_text_start_index: crate::simple_type::UInt32Value,
  /// Base Text End Index
  #[sdk(attr(qname = ":eb"))]
  pub ending_base_index: crate::simple_type::UInt32Value,
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: std::boxed::Box<Text>,
}
/// Phonetic Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PhoneticPr/x:phoneticPr")]
pub struct PhoneticProperties {
  /// Font Id
  #[sdk(attr(qname = ":fontId"))]
  pub font_id: crate::simple_type::UInt32Value,
  /// Character Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<PhoneticValues>,
  /// Alignment
  #[sdk(attr(qname = ":alignment"))]
  pub alignment: Option<PhoneticAlignmentValues>,
}
/// Header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionHeader/x:header")]
pub struct Header {
  pub xml_other_attrs: Vec<(String, String)>,
  /// GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Date Time
  #[sdk(attr(qname = ":dateTime"))]
  pub date_time: crate::simple_type::DateTimeValue,
  /// Last Sheet Id
  #[sdk(attr(qname = ":maxSheetId"))]
  pub max_sheet_id: crate::simple_type::UInt32Value,
  /// User Name
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// Relationship ID
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Minimum Revision Id
  #[sdk(attr(qname = ":minRId"))]
  pub min_revision_id: Option<crate::simple_type::UInt32Value>,
  /// Max Revision Id
  #[sdk(attr(qname = ":maxRId"))]
  pub max_revision_id: Option<crate::simple_type::UInt32Value>,
  /// Sheet Id Map
  #[sdk(child(qname = "x:CT_SheetIdMap/x:sheetIdMap"))]
  pub sheet_id_map: std::boxed::Box<SheetIdMap>,
  /// Reviewed List
  #[sdk(child(qname = "x:CT_ReviewedRevisions/x:reviewedList"))]
  pub reviewed_list: Option<ReviewedList>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Row Column Insert Delete.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionRowColumn/x:rrc")]
pub struct RevisionRowColumn {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// End Of List
  #[sdk(attr(qname = ":eol"))]
  pub end_of_list: Option<crate::simple_type::BooleanValue>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// User Action
  #[sdk(attr(qname = ":action"))]
  pub action: RowColumnActionValues,
  /// Edge Deleted
  #[sdk(attr(qname = ":edge"))]
  pub edge: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "x:CT_UndoInfo/x:undo",
    qname = "x:CT_RevisionCellChange/x:rcc",
    qname = "x:CT_RevisionFormatting/x:rfmt",
    text,
    any
  ))]
  pub revision_row_column_choice: Vec<RevisionRowColumnChoice>,
}
/// Revision Cell Move.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionMove/x:rm")]
pub struct RevisionMove {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Source
  #[sdk(attr(qname = ":source"))]
  pub source: crate::simple_type::StringValue,
  /// Destination
  #[sdk(attr(qname = ":destination"))]
  pub destination: crate::simple_type::StringValue,
  /// Source Sheet Id
  #[sdk(attr(qname = ":sourceSheetId"))]
  pub source_sheet_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_UndoInfo/x:undo",
    qname = "x:CT_RevisionCellChange/x:rcc",
    qname = "x:CT_RevisionFormatting/x:rfmt",
    text,
    any
  ))]
  pub revision_move_choice: Vec<RevisionMoveChoice>,
}
/// Revision Custom View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionCustomView/x:rcv")]
pub struct RevisionCustomView {
  /// GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Action
  #[sdk(attr(qname = ":action"))]
  pub action: RevisionActionValues,
}
/// Revision Sheet Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionSheetRename/x:rsnm")]
pub struct RevisionSheetName {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Old Sheet Name
  #[sdk(attr(qname = ":oldName"))]
  pub old_name: crate::simple_type::StringValue,
  /// New Sheet Name
  #[sdk(attr(qname = ":newName"))]
  pub new_name: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Insert Sheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionInsertSheet/x:ris")]
pub struct RevisionInsertSheet {
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Sheet Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Sheet Position
  #[sdk(attr(qname = ":sheetPosition"))]
  pub sheet_position: crate::simple_type::UInt32Value,
}
/// Revision Cell Change.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionCellChange/x:rcc")]
pub struct RevisionCellChange {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Old Formatting
  #[sdk(attr(qname = ":odxf"))]
  pub old_formatting: Option<crate::simple_type::BooleanValue>,
  /// Row Column Formatting Change
  #[sdk(attr(qname = ":xfDxf"))]
  pub row_column_formatting_affected: Option<crate::simple_type::BooleanValue>,
  /// Style Revision
  #[sdk(attr(qname = ":s"))]
  pub style_revision: Option<crate::simple_type::BooleanValue>,
  /// Formatting
  #[sdk(attr(qname = ":dxf"))]
  pub format: Option<crate::simple_type::BooleanValue>,
  /// Number Format Id
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Quote Prefix
  #[sdk(attr(qname = ":quotePrefix"))]
  pub quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Old Quote Prefix
  #[sdk(attr(qname = ":oldQuotePrefix"))]
  pub old_quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Phonetic Text
  #[sdk(attr(qname = ":ph"))]
  pub has_phonetic_text: Option<crate::simple_type::BooleanValue>,
  /// Old Phonetic Text
  #[sdk(attr(qname = ":oldPh"))]
  pub old_phonetic_text: Option<crate::simple_type::BooleanValue>,
  /// End of List  Formula Update
  #[sdk(attr(qname = ":endOfListFormulaUpdate"))]
  pub end_of_list_formula_update: Option<crate::simple_type::BooleanValue>,
  /// Old Cell Data
  #[sdk(child(qname = "x:CT_Cell/x:oc"))]
  pub old_cell: Option<std::boxed::Box<OldCell>>,
  /// New Cell Data
  #[sdk(child(qname = "x:CT_NewCell/x:nc"))]
  pub new_cell: std::boxed::Box<NewCell>,
  /// Old Formatting Information
  #[sdk(child(qname = "x:CT_Dxf/x:odxf"))]
  pub old_differential_format: Option<std::boxed::Box<OldDifferentialFormat>>,
  /// New Formatting Information
  #[sdk(child(qname = "x:CT_Dxf/x:ndxf"))]
  pub new_differential_format: Option<std::boxed::Box<NewDifferentialFormat>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionFormatting/x:rfmt")]
pub struct RevisionFormat {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Row or Column Formatting Change
  #[sdk(attr(qname = ":xfDxf"))]
  pub row_or_column_affected: Option<crate::simple_type::BooleanValue>,
  /// Style
  #[sdk(attr(qname = ":s"))]
  pub style_affected: Option<crate::simple_type::BooleanValue>,
  /// Sequence Of References
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Start index
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// Length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::UInt32Value>,
  /// Formatting
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub differential_format: Option<std::boxed::Box<DifferentialFormat>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision AutoFormat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionAutoFormatting/x:raf")]
pub struct RevisionAutoFormat {
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Auto Format Id
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Revision Defined Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionDefinedName/x:rdn")]
pub struct RevisionDefinedName {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Local Name Sheet Id
  #[sdk(attr(qname = ":localSheetId"))]
  pub local_sheet_id: Option<crate::simple_type::UInt32Value>,
  /// Custom View
  #[sdk(attr(qname = ":customView"))]
  pub custom_view: Option<crate::simple_type::BooleanValue>,
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// Old Function
  #[sdk(attr(qname = ":oldFunction"))]
  pub old_function: Option<crate::simple_type::BooleanValue>,
  /// Function Group Id
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::ByteValue>,
  /// Old Function Group Id
  #[sdk(attr(qname = ":oldFunctionGroupId"))]
  pub old_function_group_id: Option<crate::simple_type::ByteValue>,
  /// Shortcut Key
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::ByteValue>,
  /// Old Short Cut Key
  #[sdk(attr(qname = ":oldShortcutKey"))]
  pub old_shortcut_key: Option<crate::simple_type::ByteValue>,
  /// Named Range Hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Old Hidden
  #[sdk(attr(qname = ":oldHidden"))]
  pub old_hidden: Option<crate::simple_type::BooleanValue>,
  /// New Custom Menu
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// Old Custom Menu Text
  #[sdk(attr(qname = ":oldCustomMenu"))]
  pub old_custom_menu: Option<crate::simple_type::StringValue>,
  /// Description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Old Description
  #[sdk(attr(qname = ":oldDescription"))]
  pub old_description: Option<crate::simple_type::StringValue>,
  /// New Help Topic
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// Old Help Topic
  #[sdk(attr(qname = ":oldHelp"))]
  pub old_help: Option<crate::simple_type::StringValue>,
  /// Status Bar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// Old Status Bar
  #[sdk(attr(qname = ":oldStatusBar"))]
  pub old_status_bar: Option<crate::simple_type::StringValue>,
  /// Name Comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Old Name Comment
  #[sdk(attr(qname = ":oldComment"))]
  pub old_comment: Option<crate::simple_type::StringValue>,
  /// Formula
  #[sdk(child(qname = "x:CT_Xstring/x:formula"))]
  pub formula: Option<Formula>,
  /// Old Formula
  #[sdk(child(qname = "x:CT_Xstring/x:oldFormula"))]
  pub old_formula: Option<OldFormula>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Cell Comment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionComment/x:rcmt")]
pub struct RevisionComment {
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Cell
  #[sdk(attr(qname = ":cell"))]
  pub cell: crate::simple_type::StringValue,
  /// GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<RevisionActionValues>,
  /// Always Show Comment
  #[sdk(attr(qname = ":alwaysShow"))]
  pub always_show: Option<crate::simple_type::BooleanValue>,
  /// Old Comment
  #[sdk(attr(qname = ":old"))]
  pub old: Option<crate::simple_type::BooleanValue>,
  /// Comment In Hidden Row
  #[sdk(attr(qname = ":hiddenRow"))]
  pub hidden_row: Option<crate::simple_type::BooleanValue>,
  /// Hidden Column
  #[sdk(attr(qname = ":hiddenColumn"))]
  pub hidden_column: Option<crate::simple_type::BooleanValue>,
  /// Author
  #[sdk(attr(qname = ":author"))]
  pub author: crate::simple_type::StringValue,
  /// Original Comment Length
  #[sdk(attr(qname = ":oldLength"))]
  pub old_length: Option<crate::simple_type::UInt32Value>,
  /// New Comment Length
  #[sdk(attr(qname = ":newLength"))]
  pub new_length: Option<crate::simple_type::UInt32Value>,
}
/// Revision Query Table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionQueryTableField/x:rqt")]
pub struct RevisionQueryTable {
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// QueryTable Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Field Id
  #[sdk(attr(qname = ":fieldId"))]
  pub field_id: crate::simple_type::UInt32Value,
}
/// Revision Merge Conflict.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionConflict/x:rcft")]
pub struct RevisionConflict {
  /// Revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// Sheet Id Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetIdMap/x:sheetIdMap")]
pub struct SheetIdMap {
  /// Sheet Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Sheet Id.
  #[sdk(child(qname = "x:CT_SheetId/x:sheetId"))]
  pub x_sheet_id: Vec<SheetId>,
}
/// Reviewed List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ReviewedRevisions/x:reviewedList")]
pub struct ReviewedList {
  /// Reviewed Revisions Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Reviewed.
  #[sdk(child(qname = "x:CT_Reviewed/x:reviewed"))]
  pub x_reviewed: Vec<Reviewed>,
}
/// Reviewed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Reviewed/x:reviewed")]
pub struct Reviewed {
  /// revision Id
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
}
/// Undo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_UndoInfo/x:undo")]
pub struct Undo {
  /// Index
  #[sdk(attr(qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  /// Expression
  #[sdk(attr(qname = ":exp"))]
  pub expression: FormulaExpressionValues,
  /// Reference 3D
  #[sdk(attr(qname = ":ref3D"))]
  pub reference3_d: Option<crate::simple_type::BooleanValue>,
  /// Array Entered
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// Value Needed
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::BooleanValue>,
  /// Defined Name Formula
  #[sdk(attr(qname = ":nf"))]
  pub defined_name_formula: Option<crate::simple_type::BooleanValue>,
  /// Cross Sheet Move
  #[sdk(attr(qname = ":cs"))]
  pub cross_sheet_move: Option<crate::simple_type::BooleanValue>,
  /// Range
  #[sdk(attr(qname = ":dr"))]
  pub deleted_range: crate::simple_type::StringValue,
  /// Defined Name
  #[sdk(attr(qname = ":dn"))]
  pub defined_name: Option<crate::simple_type::StringValue>,
  /// Cell Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// Old Cell Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cell/x:oc")]
pub struct OldCell {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Style Index
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  /// Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  /// Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  /// Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cell/x:c")]
pub struct Cell {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Style Index
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  /// Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  /// Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  /// Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// New Cell Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NewCell/x:nc")]
pub struct NewCell {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Style Index
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  /// Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  /// Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  /// Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Old Formatting Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:odxf")]
pub struct OldDifferentialFormat {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// New Formatting Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:ndxf")]
pub struct NewDifferentialFormat {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:dxf")]
pub struct DifferentialFormat {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Sheet Id.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetId/x:sheetId")]
pub struct SheetId {
  /// Sheet Id
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellFormula/x:f")]
pub struct CellFormula {
  /// Formula Type
  #[sdk(attr(qname = ":t"))]
  pub formula_type: Option<CellFormulaValues>,
  /// Always Calculate Array
  #[sdk(attr(qname = ":aca"))]
  pub always_calculate_array: Option<crate::simple_type::BooleanValue>,
  /// Range of Cells
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Data Table 2-D
  #[sdk(attr(qname = ":dt2D"))]
  pub data_table2_d: Option<crate::simple_type::BooleanValue>,
  /// Data Table Row
  #[sdk(attr(qname = ":dtr"))]
  pub data_table_row: Option<crate::simple_type::BooleanValue>,
  /// Input 1 Deleted
  #[sdk(attr(qname = ":del1"))]
  pub input1_deleted: Option<crate::simple_type::BooleanValue>,
  /// Input 2 Deleted
  #[sdk(attr(qname = ":del2"))]
  pub input2_deleted: Option<crate::simple_type::BooleanValue>,
  /// Data Table Cell 1
  #[sdk(attr(qname = ":r1"))]
  pub r1: Option<crate::simple_type::StringValue>,
  /// Input Cell 2
  #[sdk(attr(qname = ":r2"))]
  pub r2: Option<crate::simple_type::StringValue>,
  /// Calculate Cell
  #[sdk(attr(qname = ":ca"))]
  pub calculate_cell: Option<crate::simple_type::BooleanValue>,
  /// Shared Group Index
  #[sdk(attr(qname = ":si"))]
  pub shared_index: Option<crate::simple_type::UInt32Value>,
  /// Assigns Value to Name
  #[sdk(attr(qname = ":bx"))]
  pub bx: Option<crate::simple_type::BooleanValue>,
  /// Content Contains Significant Whitespace
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// User Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SharedUser/x:userInfo")]
pub struct UserInfo {
  pub xml_other_attrs: Vec<(String, String)>,
  /// User Revisions GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// User Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::Int32Value,
  /// Date Time
  #[sdk(attr(qname = ":dateTime"))]
  pub date_time: crate::simple_type::DateTimeValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Row/x:row")]
pub struct Row {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Row Index
  #[sdk(attr(qname = ":r"))]
  pub row_index: Option<crate::simple_type::UInt32Value>,
  /// Spans
  #[sdk(attr(qname = ":spans"))]
  pub spans: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// Style Index
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Custom Format
  #[sdk(attr(qname = ":customFormat"))]
  pub custom_format: Option<crate::simple_type::BooleanValue>,
  /// Row Height
  #[sdk(attr(qname = ":ht"))]
  pub height: Option<crate::simple_type::DoubleValue>,
  /// Hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Custom Height
  #[sdk(attr(qname = ":customHeight"))]
  pub custom_height: Option<crate::simple_type::BooleanValue>,
  /// Outline Level
  #[sdk(attr(qname = ":outlineLevel"))]
  pub outline_level: Option<crate::simple_type::ByteValue>,
  /// Collapsed
  #[sdk(attr(qname = ":collapsed"))]
  pub collapsed: Option<crate::simple_type::BooleanValue>,
  /// Thick Top Border
  #[sdk(attr(qname = ":thickTop"))]
  pub thick_top: Option<crate::simple_type::BooleanValue>,
  /// Thick Bottom
  #[sdk(attr(qname = ":thickBot"))]
  pub thick_bot: Option<crate::simple_type::BooleanValue>,
  /// Show Phonetic
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  /// dyDescent
  #[sdk(attr(office2010, qname = "x14ac:dyDescent"))]
  pub dy_descent: Option<crate::simple_type::DoubleValue>,
  /// Cell.
  #[sdk(child(qname = "x:CT_Cell/x:c"))]
  pub x_c: Vec<Cell>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Column Width and Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Col/x:col")]
pub struct Column {
  /// Minimum Column
  #[sdk(attr(qname = ":min"))]
  pub min: crate::simple_type::UInt32Value,
  /// Maximum Column
  #[sdk(attr(qname = ":max"))]
  pub max: crate::simple_type::UInt32Value,
  /// Column Width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::DoubleValue>,
  /// Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::UInt32Value>,
  /// Hidden Columns
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Best Fit Column Width
  #[sdk(attr(qname = ":bestFit"))]
  pub best_fit: Option<crate::simple_type::BooleanValue>,
  /// Custom Width
  #[sdk(attr(qname = ":customWidth"))]
  pub custom_width: Option<crate::simple_type::BooleanValue>,
  /// Show Phonetic Information
  #[sdk(attr(qname = ":phonetic"))]
  pub phonetic: Option<crate::simple_type::BooleanValue>,
  /// Outline Level
  #[sdk(attr(qname = ":outlineLevel"))]
  pub outline_level: Option<crate::simple_type::ByteValue>,
  /// Collapsed
  #[sdk(attr(qname = ":collapsed"))]
  pub collapsed: Option<crate::simple_type::BooleanValue>,
}
/// Outline Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OutlinePr/x:outlinePr")]
pub struct OutlineProperties {
  /// Apply Styles in Outline
  #[sdk(attr(qname = ":applyStyles"))]
  pub apply_styles: Option<crate::simple_type::BooleanValue>,
  /// Summary Below
  #[sdk(attr(qname = ":summaryBelow"))]
  pub summary_below: Option<crate::simple_type::BooleanValue>,
  /// Summary Right
  #[sdk(attr(qname = ":summaryRight"))]
  pub summary_right: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  #[sdk(attr(qname = ":showOutlineSymbols"))]
  pub show_outline_symbols: Option<crate::simple_type::BooleanValue>,
}
/// Page Setup Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageSetUpPr/x:pageSetUpPr")]
pub struct PageSetupProperties {
  /// Show Auto Page Breaks
  #[sdk(attr(qname = ":autoPageBreaks"))]
  pub auto_page_breaks: Option<crate::simple_type::BooleanValue>,
  /// Fit To Page
  #[sdk(attr(qname = ":fitToPage"))]
  pub fit_to_page: Option<crate::simple_type::BooleanValue>,
}
/// View Pane.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Pane/x:pane")]
pub struct Pane {
  /// Horizontal Split Position
  #[sdk(attr(qname = ":xSplit"))]
  pub horizontal_split: Option<crate::simple_type::DoubleValue>,
  /// Vertical Split Position
  #[sdk(attr(qname = ":ySplit"))]
  pub vertical_split: Option<crate::simple_type::DoubleValue>,
  /// Top Left Visible Cell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  /// Active Pane
  #[sdk(attr(qname = ":activePane"))]
  pub active_pane: Option<PaneValues>,
  /// Split State
  #[sdk(attr(qname = ":state"))]
  pub state: Option<PaneStateValues>,
}
/// Selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Selection/x:selection")]
pub struct Selection {
  /// Pane
  #[sdk(attr(qname = ":pane"))]
  pub pane: Option<PaneValues>,
  /// Active Cell Location
  #[sdk(attr(qname = ":activeCell"))]
  pub active_cell: Option<crate::simple_type::StringValue>,
  /// Active Cell Index
  #[sdk(attr(qname = ":activeCellId"))]
  pub active_cell_id: Option<crate::simple_type::UInt32Value>,
  /// Sequence of References
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
}
/// PivotTable Selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotSelection/x:pivotSelection")]
pub struct PivotSelection {
  /// Pane
  #[sdk(attr(qname = ":pane"))]
  pub pane: Option<PaneValues>,
  /// Show Header
  #[sdk(attr(qname = ":showHeader"))]
  pub show_header: Option<crate::simple_type::BooleanValue>,
  /// Label
  #[sdk(attr(qname = ":label"))]
  pub label: Option<crate::simple_type::BooleanValue>,
  /// Data Selection
  #[sdk(attr(qname = ":data"))]
  pub data: Option<crate::simple_type::BooleanValue>,
  /// Extendable
  #[sdk(attr(qname = ":extendable"))]
  pub extendable: Option<crate::simple_type::BooleanValue>,
  /// Selection Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Dimension
  #[sdk(attr(qname = ":dimension"))]
  pub dimension: Option<crate::simple_type::UInt32Value>,
  /// Start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// Minimum
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// Maximum
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// Active Row
  #[sdk(attr(qname = ":activeRow"))]
  pub active_row: Option<crate::simple_type::UInt32Value>,
  /// Active Column
  #[sdk(attr(qname = ":activeCol"))]
  pub active_column: Option<crate::simple_type::UInt32Value>,
  /// Previous Row
  #[sdk(attr(qname = ":previousRow"))]
  pub previous_row: Option<crate::simple_type::UInt32Value>,
  /// Previous Column Selection
  #[sdk(attr(qname = ":previousCol"))]
  pub previous_column: Option<crate::simple_type::UInt32Value>,
  /// Click Count
  #[sdk(attr(qname = ":click"))]
  pub click: Option<crate::simple_type::UInt32Value>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Pivot Area
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// Break.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Break/x:brk")]
pub struct Break {
  /// Id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Minimum
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// Maximum
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// Manual Page Break
  #[sdk(attr(qname = ":man"))]
  pub manual_page_break: Option<crate::simple_type::BooleanValue>,
  /// Pivot-Created Page Break
  #[sdk(attr(qname = ":pt"))]
  pub pivot_table_page_break: Option<crate::simple_type::BooleanValue>,
}
/// Data Consolidation Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataRef/x:dataRef")]
pub struct DataReference {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Horizontal Page Breaks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageBreak/x:rowBreaks")]
pub struct RowBreaks {
  /// Page Break Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Manual Break Count
  #[sdk(attr(qname = ":manualBreakCount"))]
  pub manual_break_count: Option<crate::simple_type::UInt32Value>,
  /// Break.
  #[sdk(child(qname = "x:CT_Break/x:brk"))]
  pub x_brk: Vec<Break>,
}
/// Vertical Page Breaks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageBreak/x:colBreaks")]
pub struct ColumnBreaks {
  /// Page Break Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Manual Break Count
  #[sdk(attr(qname = ":manualBreakCount"))]
  pub manual_break_count: Option<crate::simple_type::UInt32Value>,
  /// Break.
  #[sdk(child(qname = "x:CT_Break/x:brk"))]
  pub x_brk: Vec<Break>,
}
/// Page Margins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageMargins/x:pageMargins")]
pub struct PageMargins {
  /// Left Page Margin
  #[sdk(attr(qname = ":left"))]
  pub left: crate::simple_type::DoubleValue,
  /// Right Page Margin
  #[sdk(attr(qname = ":right"))]
  pub right: crate::simple_type::DoubleValue,
  /// Top Page Margin
  #[sdk(attr(qname = ":top"))]
  pub top: crate::simple_type::DoubleValue,
  /// Bottom Page Margin
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: crate::simple_type::DoubleValue,
  /// Header Page Margin
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// Footer Page Margin
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Print Options.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PrintOptions/x:printOptions")]
pub struct PrintOptions {
  /// Horizontal Centered
  #[sdk(attr(qname = ":horizontalCentered"))]
  pub horizontal_centered: Option<crate::simple_type::BooleanValue>,
  /// Vertical Centered
  #[sdk(attr(qname = ":verticalCentered"))]
  pub vertical_centered: Option<crate::simple_type::BooleanValue>,
  /// Print Headings
  #[sdk(attr(qname = ":headings"))]
  pub headings: Option<crate::simple_type::BooleanValue>,
  /// Print Grid Lines
  #[sdk(attr(qname = ":gridLines"))]
  pub grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Grid Lines Set
  #[sdk(attr(qname = ":gridLinesSet"))]
  pub grid_lines_set: Option<crate::simple_type::BooleanValue>,
}
/// Page Setup Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageSetup/x:pageSetup")]
pub struct PageSetup {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Paper Size
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// Print Scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// Fit To Width
  #[sdk(attr(qname = ":fitToWidth"))]
  pub fit_to_width: Option<crate::simple_type::UInt32Value>,
  /// Fit To Height
  #[sdk(attr(qname = ":fitToHeight"))]
  pub fit_to_height: Option<crate::simple_type::UInt32Value>,
  /// Page Order
  #[sdk(attr(qname = ":pageOrder"))]
  pub page_order: Option<PageOrderValues>,
  /// Orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<OrientationValues>,
  /// Use Printer Defaults
  #[sdk(attr(qname = ":usePrinterDefaults"))]
  pub use_printer_defaults: Option<crate::simple_type::BooleanValue>,
  /// Black And White
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Print Cell Comments
  #[sdk(attr(qname = ":cellComments"))]
  pub cell_comments: Option<CellCommentsValues>,
  /// Use First Page Number
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Print Error Handling
  #[sdk(attr(qname = ":errors"))]
  pub errors: Option<PrintErrorValues>,
  /// Horizontal DPI
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::UInt32Value>,
  /// Vertical DPI
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::UInt32Value>,
  /// Number Of Copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
  /// Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Header Footer Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HeaderFooter/x:headerFooter")]
pub struct HeaderFooter {
  /// Different Odd Even Header Footer
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// Different First Page
  #[sdk(attr(qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// Scale Header and Footer With Document
  #[sdk(attr(qname = ":scaleWithDoc"))]
  pub scale_with_doc: Option<crate::simple_type::BooleanValue>,
  /// Align Margins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// Odd Header
  #[sdk(child(qname = "x:CT_Xstring/x:oddHeader"))]
  pub odd_header: Option<OddHeader>,
  /// Odd Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:oddFooter"))]
  pub odd_footer: Option<OddFooter>,
  /// Even Page Header
  #[sdk(child(qname = "x:CT_Xstring/x:evenHeader"))]
  pub even_header: Option<EvenHeader>,
  /// Even Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:evenFooter"))]
  pub even_footer: Option<EvenFooter>,
  /// First Page Header
  #[sdk(child(qname = "x:CT_Xstring/x:firstHeader"))]
  pub first_header: Option<FirstHeader>,
  /// First Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:firstFooter"))]
  pub first_footer: Option<FirstFooter>,
}
/// AutoFilter Settings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoFilter/x:autoFilter")]
pub struct AutoFilter {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Cell or Range Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// AutoFilter Column.
  #[sdk(child(qname = "x:CT_FilterColumn/x:filterColumn"))]
  pub x_filter_column: Vec<FilterColumn>,
  /// Sort State for Auto Filter.
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<std::boxed::Box<SortState>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Conditional Formatting Rule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRule/x:cfRule")]
pub struct ConditionalFormattingRule {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormatValues,
  /// Differential Formatting Id
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: crate::simple_type::Int32Value,
  /// Stop If True
  #[sdk(attr(qname = ":stopIfTrue"))]
  pub stop_if_true: Option<crate::simple_type::BooleanValue>,
  /// Above Or Below Average
  #[sdk(attr(qname = ":aboveAverage"))]
  pub above_average: Option<crate::simple_type::BooleanValue>,
  /// Top 10 Percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Bottom N
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::BooleanValue>,
  /// Operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<ConditionalFormattingOperatorValues>,
  /// Text
  #[sdk(attr(qname = ":text"))]
  pub text: Option<crate::simple_type::StringValue>,
  /// Time Period
  #[sdk(attr(qname = ":timePeriod"))]
  pub time_period: Option<TimePeriodValues>,
  /// Rank
  #[sdk(attr(qname = ":rank"))]
  pub rank: Option<crate::simple_type::UInt32Value>,
  /// StdDev
  #[sdk(attr(qname = ":stdDev"))]
  pub std_dev: Option<crate::simple_type::Int32Value>,
  /// Equal Average
  #[sdk(attr(qname = ":equalAverage"))]
  pub equal_average: Option<crate::simple_type::BooleanValue>,
  /// Formula.
  #[sdk(child(qname = "x:CT_Xstring/x:formula"))]
  pub x_formula: Vec<Formula>,
  /// Color Scale.
  #[sdk(child(qname = "x:CT_ColorScale/x:colorScale"))]
  pub x_color_scale: Option<ColorScale>,
  /// Data Bar.
  #[sdk(child(qname = "x:CT_DataBar/x:dataBar"))]
  pub x_data_bar: Option<std::boxed::Box<DataBar>>,
  /// Icon Set.
  #[sdk(child(qname = "x:CT_IconSet/x:iconSet"))]
  pub x_icon_set: Option<IconSet>,
  /// Defines the ConditionalFormattingRuleExtensionList Class.
  #[sdk(child(qname = "x:CT_CfRuleExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ConditionalFormattingRuleExtensionList>,
}
/// Hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Hyperlink/x:hyperlink")]
pub struct Hyperlink {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Location
  #[sdk(attr(qname = ":location"))]
  pub location: Option<crate::simple_type::StringValue>,
  /// Tool Tip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// Display String
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::StringValue>,
}
/// Conditional Format Value Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cfvo/x:cfvo")]
pub struct ConditionalFormatValueObject {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormatValueObjectValues,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// Greater Than Or Equal
  #[sdk(attr(qname = ":gte"))]
  pub greater_than_or_equal: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Scenario.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Scenario/x:scenario")]
pub struct Scenario {
  /// Scenario Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Scenario Locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Hidden Scenario
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Changing Cell Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// User Name
  #[sdk(attr(qname = ":user"))]
  pub user: Option<crate::simple_type::StringValue>,
  /// Scenario Comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Input Cells.
  #[sdk(child(qname = "x:CT_InputCells/x:inputCells"))]
  pub x_input_cells: Vec<InputCells>,
}
/// Protected Range.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ProtectedRange/x:protectedRange")]
pub struct ProtectedRange {
  /// password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// securityDescriptor
  #[sdk(attr(qname = ":securityDescriptor"))]
  pub security_descriptor: Option<crate::simple_type::StringValue>,
}
/// Cell Watch Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellWatch/x:cellWatch")]
pub struct CellWatch {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
}
/// Chart Sheet Page Setup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CsPageSetup/x:pageSetup")]
pub struct ChartSheetPageSetup {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Paper Size
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// Orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<OrientationValues>,
  /// Use Printer Defaults
  #[sdk(attr(qname = ":usePrinterDefaults"))]
  pub use_printer_defaults: Option<crate::simple_type::BooleanValue>,
  /// Black And White
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Use First Page Number
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Horizontal DPI
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::UInt32Value>,
  /// Vertical DPI
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::UInt32Value>,
  /// Number Of Copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
  /// Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Custom Property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomProperty/x:customPr")]
pub struct CustomProperty {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom Property Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Web Publishing Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishItem/x:webPublishItem")]
pub struct WebPublishItem {
  /// Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Destination Bookmark
  #[sdk(attr(qname = ":divId"))]
  pub div_id: crate::simple_type::StringValue,
  /// Web Source Type
  #[sdk(attr(qname = ":sourceType"))]
  pub source_type: WebSourceValues,
  /// Source Id
  #[sdk(attr(qname = ":sourceRef"))]
  pub source_ref: Option<crate::simple_type::StringValue>,
  /// Source Object Name
  #[sdk(attr(qname = ":sourceObject"))]
  pub source_object: Option<crate::simple_type::StringValue>,
  /// Destination File Name
  #[sdk(attr(qname = ":destinationFile"))]
  pub destination_file: crate::simple_type::StringValue,
  /// Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Automatically Publish
  #[sdk(attr(qname = ":autoRepublish"))]
  pub auto_republish: Option<crate::simple_type::BooleanValue>,
}
/// Table Part.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TablePart/x:tablePart")]
pub struct TablePart {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Chart Sheet View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetView/x:sheetView")]
pub struct ChartSheetView {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Sheet Tab Selected
  #[sdk(attr(qname = ":tabSelected"))]
  pub tab_selected: Option<crate::simple_type::BooleanValue>,
  /// Window Zoom Scale
  #[sdk(attr(qname = ":zoomScale"))]
  pub zoom_scale: Option<crate::simple_type::UInt32Value>,
  /// Workbook View Id
  #[sdk(attr(qname = ":workbookViewId"))]
  pub workbook_view_id: crate::simple_type::UInt32Value,
  /// Zoom To Fit
  #[sdk(attr(qname = ":zoomToFit"))]
  pub zoom_to_fit: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Custom Chart Sheet View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomChartsheetView/x:customSheetView")]
pub struct CustomChartsheetView {
  /// GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Print Scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// Visible State
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Zoom To Fit
  #[sdk(attr(qname = ":zoomToFit"))]
  pub zoom_to_fit: Option<crate::simple_type::BooleanValue>,
  /// Page Margins.
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Chart Sheet Page Setup
  #[sdk(child(qname = "x:CT_CsPageSetup/x:pageSetup"))]
  pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
  /// Header Footer Settings.
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
}
/// Input Cells.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_InputCells/x:inputCells")]
pub struct InputCells {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// Undone
  #[sdk(attr(qname = ":undone"))]
  pub undone: Option<crate::simple_type::BooleanValue>,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
  /// Number Format Id
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
}
/// Embedded Control.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Control/x:control")]
pub struct Control {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Shape Id
  #[sdk(attr(qname = ":shapeId"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Control Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Defines the ControlProperties Class.
  #[sdk(child(office2010, qname = "x:CT_ControlPr/x:controlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Ignored Error.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IgnoredError/x:ignoredError")]
pub struct IgnoredError {
  /// Sequence of References
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Evaluation Error
  #[sdk(attr(qname = ":evalError"))]
  pub eval_error: Option<crate::simple_type::BooleanValue>,
  /// Two Digit Text Year
  #[sdk(attr(qname = ":twoDigitTextYear"))]
  pub two_digit_text_year: Option<crate::simple_type::BooleanValue>,
  /// Number Stored As Text
  #[sdk(attr(qname = ":numberStoredAsText"))]
  pub number_stored_as_text: Option<crate::simple_type::BooleanValue>,
  /// Formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::BooleanValue>,
  /// Formula Range
  #[sdk(attr(qname = ":formulaRange"))]
  pub formula_range: Option<crate::simple_type::BooleanValue>,
  /// Unlocked Formula
  #[sdk(attr(qname = ":unlockedFormula"))]
  pub unlocked_formula: Option<crate::simple_type::BooleanValue>,
  /// Empty Cell Reference
  #[sdk(attr(qname = ":emptyCellReference"))]
  pub empty_cell_reference: Option<crate::simple_type::BooleanValue>,
  /// List Data Validation
  #[sdk(attr(qname = ":listDataValidation"))]
  pub list_data_validation: Option<crate::simple_type::BooleanValue>,
  /// Calculated Column
  #[sdk(attr(qname = ":calculatedColumn"))]
  pub calculated_column: Option<crate::simple_type::BooleanValue>,
}
/// Merged Cell.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MergeCell/x:mergeCell")]
pub struct MergeCell {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Data Validation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataValidation/x:dataValidation")]
pub struct DataValidation {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<DataValidationValues>,
  /// errorStyle
  #[sdk(attr(qname = ":errorStyle"))]
  pub error_style: Option<DataValidationErrorStyleValues>,
  /// imeMode
  #[sdk(attr(qname = ":imeMode"))]
  pub ime_mode: Option<DataValidationImeModeValues>,
  /// operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<DataValidationOperatorValues>,
  /// allowBlank
  #[sdk(attr(qname = ":allowBlank"))]
  pub allow_blank: Option<crate::simple_type::BooleanValue>,
  /// showDropDown
  #[sdk(attr(qname = ":showDropDown"))]
  pub show_drop_down: Option<crate::simple_type::BooleanValue>,
  /// showInputMessage
  #[sdk(attr(qname = ":showInputMessage"))]
  pub show_input_message: Option<crate::simple_type::BooleanValue>,
  /// showErrorMessage
  #[sdk(attr(qname = ":showErrorMessage"))]
  pub show_error_message: Option<crate::simple_type::BooleanValue>,
  /// errorTitle
  #[sdk(attr(qname = ":errorTitle"))]
  pub error_title: Option<crate::simple_type::StringValue>,
  /// error
  #[sdk(attr(qname = ":error"))]
  pub error: Option<crate::simple_type::StringValue>,
  /// promptTitle
  #[sdk(attr(qname = ":promptTitle"))]
  pub prompt_title: Option<crate::simple_type::StringValue>,
  /// prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  /// sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Defines the List Class.
  #[sdk(text_child(office2013, qname = "x:ST_Xstring/x12ac:list"))]
  pub list: Option<crate::simple_type::StringValue>,
  /// Defines the Formula1 Class.
  #[sdk(child(qname = "x:CT_Xstring/x:formula1"))]
  pub formula1: Option<Formula1>,
  /// Defines the Formula2 Class.
  #[sdk(child(qname = "x:CT_Xstring/x:formula2"))]
  pub formula2: Option<Formula2>,
}
/// Worksheet View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetView/x:sheetView")]
pub struct SheetView {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Window Protection
  #[sdk(attr(qname = ":windowProtection"))]
  pub window_protection: Option<crate::simple_type::BooleanValue>,
  /// Show Formulas
  #[sdk(attr(qname = ":showFormulas"))]
  pub show_formulas: Option<crate::simple_type::BooleanValue>,
  /// Show Grid Lines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Show Headers
  #[sdk(attr(qname = ":showRowColHeaders"))]
  pub show_row_col_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Zero Values
  #[sdk(attr(qname = ":showZeros"))]
  pub show_zeros: Option<crate::simple_type::BooleanValue>,
  /// Right To Left
  #[sdk(attr(qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// Sheet Tab Selected
  #[sdk(attr(qname = ":tabSelected"))]
  pub tab_selected: Option<crate::simple_type::BooleanValue>,
  /// Show Ruler
  #[sdk(attr(qname = ":showRuler"))]
  pub show_ruler: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  #[sdk(attr(qname = ":showOutlineSymbols"))]
  pub show_outline_symbols: Option<crate::simple_type::BooleanValue>,
  /// Default Grid Color
  #[sdk(attr(qname = ":defaultGridColor"))]
  pub default_grid_color: Option<crate::simple_type::BooleanValue>,
  /// Show White Space
  #[sdk(attr(qname = ":showWhiteSpace"))]
  pub show_white_space: Option<crate::simple_type::BooleanValue>,
  /// View Type
  #[sdk(attr(qname = ":view"))]
  pub view: Option<SheetViewValues>,
  /// Top Left Visible Cell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  /// Color Id
  #[sdk(attr(qname = ":colorId"))]
  pub color_id: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale
  #[sdk(attr(qname = ":zoomScale"))]
  pub zoom_scale: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Normal View
  #[sdk(attr(qname = ":zoomScaleNormal"))]
  pub zoom_scale_normal: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Page Break Preview
  #[sdk(attr(qname = ":zoomScaleSheetLayoutView"))]
  pub zoom_scale_sheet_layout_view: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Page Layout View
  #[sdk(attr(qname = ":zoomScalePageLayoutView"))]
  pub zoom_scale_page_layout_view: Option<crate::simple_type::UInt32Value>,
  /// Workbook View Index
  #[sdk(attr(qname = ":workbookViewId"))]
  pub workbook_view_id: crate::simple_type::UInt32Value,
  /// View Pane
  #[sdk(child(qname = "x:CT_Pane/x:pane"))]
  pub pane: Option<Pane>,
  /// Selection.
  #[sdk(child(qname = "x:CT_Selection/x:selection"))]
  pub x_selection: Vec<Selection>,
  /// PivotTable Selection.
  #[sdk(child(qname = "x:CT_PivotSelection/x:pivotSelection"))]
  pub x_pivot_selection: Vec<PivotSelection>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Sheet View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomSheetView/x:customSheetView")]
pub struct CustomSheetView {
  pub xml_other_attrs: Vec<(String, String)>,
  /// GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Print Scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// Color Id
  #[sdk(attr(qname = ":colorId"))]
  pub color_id: Option<crate::simple_type::UInt32Value>,
  /// Show Page Breaks
  #[sdk(attr(qname = ":showPageBreaks"))]
  pub show_page_breaks: Option<crate::simple_type::BooleanValue>,
  /// Show Formulas
  #[sdk(attr(qname = ":showFormulas"))]
  pub show_formulas: Option<crate::simple_type::BooleanValue>,
  /// Show Grid Lines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Show Headers
  #[sdk(attr(qname = ":showRowCol"))]
  pub show_row_column: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  #[sdk(attr(qname = ":outlineSymbols"))]
  pub outline_symbols: Option<crate::simple_type::BooleanValue>,
  /// Show Zero Values
  #[sdk(attr(qname = ":zeroValues"))]
  pub zero_values: Option<crate::simple_type::BooleanValue>,
  /// Fit To Page
  #[sdk(attr(qname = ":fitToPage"))]
  pub fit_to_page: Option<crate::simple_type::BooleanValue>,
  /// Print Area Defined
  #[sdk(attr(qname = ":printArea"))]
  pub print_area: Option<crate::simple_type::BooleanValue>,
  /// Filtered List
  #[sdk(attr(qname = ":filter"))]
  pub filter: Option<crate::simple_type::BooleanValue>,
  /// Show AutoFitler Drop Down Controls
  #[sdk(attr(qname = ":showAutoFilter"))]
  pub show_auto_filter: Option<crate::simple_type::BooleanValue>,
  /// Hidden Rows
  #[sdk(attr(qname = ":hiddenRows"))]
  pub hidden_rows: Option<crate::simple_type::BooleanValue>,
  /// Hidden Columns
  #[sdk(attr(qname = ":hiddenColumns"))]
  pub hidden_columns: Option<crate::simple_type::BooleanValue>,
  /// Visible State
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Filter
  #[sdk(attr(qname = ":filterUnique"))]
  pub filter_unique: Option<crate::simple_type::BooleanValue>,
  /// View Type
  #[sdk(attr(qname = ":view"))]
  pub view: Option<SheetViewValues>,
  /// Show Ruler
  #[sdk(attr(qname = ":showRuler"))]
  pub show_ruler: Option<crate::simple_type::BooleanValue>,
  /// Top Left Visible Cell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  /// Pane Split Information
  #[sdk(child(qname = "x:CT_Pane/x:pane"))]
  pub pane: Option<Pane>,
  /// Selection
  #[sdk(child(qname = "x:CT_Selection/x:selection"))]
  pub selection: Option<Selection>,
  /// Horizontal Page Breaks
  #[sdk(child(qname = "x:CT_PageBreak/x:rowBreaks"))]
  pub row_breaks: Option<RowBreaks>,
  /// Vertical Page Breaks
  #[sdk(child(qname = "x:CT_PageBreak/x:colBreaks"))]
  pub column_breaks: Option<ColumnBreaks>,
  /// Page Margins
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Print Options
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub print_options: Option<PrintOptions>,
  /// Page Setup Settings
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub page_setup: Option<PageSetup>,
  /// Header Footer Settings
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// AutoFilter Settings
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: Option<std::boxed::Box<AutoFilter>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// OLE Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleObject/x:oleObject")]
pub struct OleObject {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// OLE ProgId
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// Data or View Aspect
  #[sdk(attr(qname = ":dvAspect"))]
  pub data_or_view_aspect: Option<DataViewAspectValues>,
  /// OLE Link Moniker
  #[sdk(attr(qname = ":link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// OLE Update
  #[sdk(attr(qname = ":oleUpdate"))]
  pub ole_update: Option<OleUpdateValues>,
  /// Auto Load
  #[sdk(attr(qname = ":autoLoad"))]
  pub auto_load: Option<crate::simple_type::BooleanValue>,
  /// Shape Id
  #[sdk(attr(qname = ":shapeId"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the EmbeddedObjectProperties Class.
  #[sdk(child(office2010, qname = "x:CT_ObjectPr/x:objectPr"))]
  pub embedded_object_properties: Option<std::boxed::Box<EmbeddedObjectProperties>>,
}
/// Metadata Types Collection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataTypes/x:metadataTypes")]
pub struct MetadataTypes {
  /// Metadata Type Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Metadata Type Information.
  #[sdk(child(qname = "x:CT_MetadataType/x:metadataType"))]
  pub x_metadata_type: Vec<MetadataType>,
}
/// Metadata String Store.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataStrings/x:metadataStrings")]
pub struct MetadataStrings {
  /// MDX Metadata String Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Character Value.
  #[sdk(child(qname = "x:CT_XStringElement/x:s"))]
  pub x_s: Vec<CharacterValue>,
}
/// MDX Metadata Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxMetadata/x:mdxMetadata")]
pub struct MdxMetadata {
  /// MDX Metadata Record Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// MDX Metadata Record.
  #[sdk(child(qname = "x:CT_Mdx/x:mdx"))]
  pub x_mdx: Vec<Mdx>,
}
/// Future Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FutureMetadata/x:futureMetadata")]
pub struct FutureMetadata {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Metadata Type Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Future Metadata Block Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Future Metadata Block.
  #[sdk(child(qname = "x:CT_FutureMetadataBlock/x:bk"))]
  pub x_bk: Vec<FutureMetadataBlock>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Cell Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlocks/x:cellMetadata")]
pub struct CellMetadata {
  /// Metadata Block Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Metadata Block.
  #[sdk(child(qname = "x:CT_MetadataBlock/x:bk"))]
  pub x_bk: Vec<MetadataBlock>,
}
/// Value Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlocks/x:valueMetadata")]
pub struct ValueMetadata {
  /// Metadata Block Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Metadata Block.
  #[sdk(child(qname = "x:CT_MetadataBlock/x:bk"))]
  pub x_bk: Vec<MetadataBlock>,
}
/// Metadata Type Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataType/x:metadataType")]
pub struct MetadataType {
  /// Metadata Type Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Minimum Supported Version
  #[sdk(attr(qname = ":minSupportedVersion"))]
  pub min_supported_version: crate::simple_type::UInt32Value,
  /// Metadata Ghost Row
  #[sdk(attr(qname = ":ghostRow"))]
  pub ghost_row: Option<crate::simple_type::BooleanValue>,
  /// Metadata Ghost Column
  #[sdk(attr(qname = ":ghostCol"))]
  pub ghost_column: Option<crate::simple_type::BooleanValue>,
  /// Metadata Edit
  #[sdk(attr(qname = ":edit"))]
  pub edit: Option<crate::simple_type::BooleanValue>,
  /// Metadata Cell Value Delete
  #[sdk(attr(qname = ":delete"))]
  pub delete: Option<crate::simple_type::BooleanValue>,
  /// Metadata Copy
  #[sdk(attr(qname = ":copy"))]
  pub copy: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste All
  #[sdk(attr(qname = ":pasteAll"))]
  pub paste_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Formulas
  #[sdk(attr(qname = ":pasteFormulas"))]
  pub paste_formulas: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Special Values
  #[sdk(attr(qname = ":pasteValues"))]
  pub paste_values: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Formats
  #[sdk(attr(qname = ":pasteFormats"))]
  pub paste_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Comments
  #[sdk(attr(qname = ":pasteComments"))]
  pub paste_comments: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Data Validation
  #[sdk(attr(qname = ":pasteDataValidation"))]
  pub paste_data_validation: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Borders
  #[sdk(attr(qname = ":pasteBorders"))]
  pub paste_borders: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Column Widths
  #[sdk(attr(qname = ":pasteColWidths"))]
  pub paste_col_widths: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Number Formats
  #[sdk(attr(qname = ":pasteNumberFormats"))]
  pub paste_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Merge
  #[sdk(attr(qname = ":merge"))]
  pub merge: Option<crate::simple_type::BooleanValue>,
  /// Meatadata Split First
  #[sdk(attr(qname = ":splitFirst"))]
  pub split_first: Option<crate::simple_type::BooleanValue>,
  /// Metadata Split All
  #[sdk(attr(qname = ":splitAll"))]
  pub split_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Insert Delete
  #[sdk(attr(qname = ":rowColShift"))]
  pub row_column_shift: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear All
  #[sdk(attr(qname = ":clearAll"))]
  pub clear_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Formats
  #[sdk(attr(qname = ":clearFormats"))]
  pub clear_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Contents
  #[sdk(attr(qname = ":clearContents"))]
  pub clear_contents: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Comments
  #[sdk(attr(qname = ":clearComments"))]
  pub clear_comments: Option<crate::simple_type::BooleanValue>,
  /// Metadata Formula Assignment
  #[sdk(attr(qname = ":assign"))]
  pub assign: Option<crate::simple_type::BooleanValue>,
  /// Metadata Coercion
  #[sdk(attr(qname = ":coerce"))]
  pub coerce: Option<crate::simple_type::BooleanValue>,
  /// Adjust Metadata
  #[sdk(attr(qname = ":adjust"))]
  pub adjust: Option<crate::simple_type::BooleanValue>,
  /// Cell Metadata
  #[sdk(attr(qname = ":cellMeta"))]
  pub cell_meta: Option<crate::simple_type::BooleanValue>,
}
/// Metadata Block.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlock/x:bk")]
pub struct MetadataBlock {
  /// Metadata Record.
  #[sdk(child(qname = "x:CT_MetadataRecord/x:rc"))]
  pub x_rc: Vec<MetadataRecord>,
}
/// Metadata Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataRecord/x:rc")]
pub struct MetadataRecord {
  /// Metadata Record Type Index
  #[sdk(attr(qname = ":t"))]
  pub type_index: crate::simple_type::UInt32Value,
  /// Metadata Record Value Index
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Future Metadata Block.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FutureMetadataBlock/x:bk")]
pub struct FutureMetadataBlock {
  /// Future Feature Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// MDX Metadata Record.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Mdx/x:mdx")]
pub struct Mdx {
  /// Connection Name Index
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// Cube Function Tag
  #[sdk(attr(qname = ":f"))]
  pub cube_function: MdxFunctionValues,
  #[sdk(choice(
    qname = "x:CT_MdxTuple/x:t",
    qname = "x:CT_MdxSet/x:ms",
    qname = "x:CT_MdxMemeberProp/x:p",
    qname = "x:CT_MdxKPI/x:k"
  ))]
  pub mdx_choice: Option<MdxChoice>,
}
/// Tuple MDX Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxTuple/x:t")]
pub struct MdxTuple {
  /// Member Index Count
  #[sdk(attr(qname = ":c"))]
  pub member_index_count: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Culture Currency
  #[sdk(attr(qname = ":ct"))]
  pub culture_currency: Option<crate::simple_type::StringValue>,
  /// Server Formatting String Index
  #[sdk(attr(qname = ":si"))]
  pub formatting_string_index: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Built-In Number Format Index
  #[sdk(attr(qname = ":fi"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Background Color
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Server Formatting Foreground Color
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Server Formatting Italic Font
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Underline Font
  #[sdk(attr(qname = ":u"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Strikethrough Font
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Bold Font
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// Member Unique Name Index.
  #[sdk(child(qname = "x:CT_MetadataStringIndex/x:n"))]
  pub x_n: Vec<NameIndex>,
}
/// Set MDX Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxSet/x:ms")]
pub struct MdxSet {
  /// Set Definition Index
  #[sdk(attr(qname = ":ns"))]
  pub set_definition_index: crate::simple_type::UInt32Value,
  /// Sort By Member Index Count
  #[sdk(attr(qname = ":c"))]
  pub member_index_count: Option<crate::simple_type::UInt32Value>,
  /// Set Sort Order
  #[sdk(attr(qname = ":o"))]
  pub sorting_order: Option<MdxSetOrderValues>,
  /// Member Unique Name Index.
  #[sdk(child(qname = "x:CT_MetadataStringIndex/x:n"))]
  pub x_n: Vec<NameIndex>,
}
/// Member Property MDX Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxMemeberProp/x:p")]
pub struct MdxMemberProp {
  /// Member Unique Name Index
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// Property Name Index
  #[sdk(attr(qname = ":np"))]
  pub property_name_index: crate::simple_type::UInt32Value,
}
/// KPI MDX Metadata.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxKPI/x:k")]
pub struct MdxKpi {
  /// Member Unique Name Index
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// KPI Index
  #[sdk(attr(qname = ":np"))]
  pub kpi_index: crate::simple_type::UInt32Value,
  /// KPI Property
  #[sdk(attr(qname = ":p"))]
  pub kpi_property: MdxKpiPropertyValues,
}
/// Member Unique Name Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataStringIndex/x:n")]
pub struct NameIndex {
  /// Index Value
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::UInt32Value,
  /// String is a Set
  #[sdk(attr(qname = ":s"))]
  pub is_a_set: Option<crate::simple_type::BooleanValue>,
}
/// Table Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SingleXmlCell/x:singleXmlCell")]
pub struct SingleXmlCell {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Table Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Connection ID
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: crate::simple_type::UInt32Value,
  /// Cell Properties
  #[sdk(child(qname = "x:CT_XmlCellPr/x:xmlCellPr"))]
  pub xml_cell_properties: std::boxed::Box<XmlCellProperties>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlCellPr/x:xmlCellPr")]
pub struct XmlCellProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Table Field Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Unique Table Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Column XML Properties
  #[sdk(child(qname = "x:CT_XmlPr/x:xmlPr"))]
  pub xml_properties: std::boxed::Box<XmlProperties>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Column XML Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlPr/x:xmlPr")]
pub struct XmlProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  /// XML Map Id
  #[sdk(attr(qname = ":mapId"))]
  pub map_id: crate::simple_type::UInt32Value,
  /// XPath
  #[sdk(attr(qname = ":xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// XML Data Type
  #[sdk(attr(qname = ":xmlDataType"))]
  pub xml_data_type: XmlDataValues,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Pattern.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PatternFill/x:patternFill")]
pub struct PatternFill {
  /// Pattern Type
  #[sdk(attr(qname = ":patternType"))]
  pub pattern_type: Option<PatternValues>,
  /// Foreground Color
  #[sdk(child(qname = "x:CT_Color/x:fgColor"))]
  pub foreground_color: Option<ForegroundColor>,
  /// Background Color
  #[sdk(child(qname = "x:CT_Color/x:bgColor"))]
  pub background_color: Option<BackgroundColor>,
}
/// Gradient.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GradientFill/x:gradientFill")]
pub struct GradientFill {
  /// Gradient Fill Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<GradientValues>,
  /// Linear Gradient Degree
  #[sdk(attr(qname = ":degree"))]
  pub degree: Option<crate::simple_type::DoubleValue>,
  /// Left Convergence
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::DoubleValue>,
  /// Right Convergence
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::DoubleValue>,
  /// Top Gradient Convergence
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::DoubleValue>,
  /// Bottom Convergence
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::DoubleValue>,
  /// Gradient Stop.
  #[sdk(child(qname = "x:CT_GradientStop/x:stop"))]
  pub x_stop: Vec<GradientStop>,
}
/// Gradient Stop.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GradientStop/x:stop")]
pub struct GradientStop {
  /// Gradient Stop Position
  #[sdk(attr(qname = ":position"))]
  pub position: crate::simple_type::DoubleValue,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: std::boxed::Box<Color>,
}
/// Number Formats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NumFmt/x:numFmt")]
pub struct NumberingFormat {
  /// Number Format Id
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: crate::simple_type::UInt32Value,
  /// Number Format Code
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
}
/// Alignment.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellAlignment/x:alignment")]
pub struct Alignment {
  /// Horizontal Alignment
  #[sdk(attr(qname = ":horizontal"))]
  pub horizontal: Option<HorizontalAlignmentValues>,
  /// Vertical Alignment
  #[sdk(attr(qname = ":vertical"))]
  pub vertical: Option<VerticalAlignmentValues>,
  /// Text Rotation
  #[sdk(attr(qname = ":textRotation"))]
  pub text_rotation: Option<crate::simple_type::UInt32Value>,
  /// Wrap Text
  #[sdk(attr(qname = ":wrapText"))]
  pub wrap_text: Option<crate::simple_type::BooleanValue>,
  /// Indent
  #[sdk(attr(qname = ":indent"))]
  pub indent: Option<crate::simple_type::UInt32Value>,
  /// Relative Indent
  #[sdk(attr(qname = ":relativeIndent"))]
  pub relative_indent: Option<crate::simple_type::Int32Value>,
  /// Justify Last Line
  #[sdk(attr(qname = ":justifyLastLine"))]
  pub justify_last_line: Option<crate::simple_type::BooleanValue>,
  /// Shrink To Fit
  #[sdk(attr(qname = ":shrinkToFit"))]
  pub shrink_to_fit: Option<crate::simple_type::BooleanValue>,
  /// Reading Order
  #[sdk(attr(qname = ":readingOrder"))]
  pub reading_order: Option<crate::simple_type::UInt32Value>,
  /// mergeCell
  #[sdk(attr(qname = ":mergeCell"))]
  pub merge_cell: Option<crate::simple_type::StringValue>,
}
/// Protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellProtection/x:protection")]
pub struct Protection {
  /// Cell Locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Hidden Cell
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
}
/// Font Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Font/x:font")]
pub struct Font {
  /// Bold
  #[sdk(child(qname = "x:CT_BooleanProperty/x:b"))]
  pub bold: Option<Bold>,
  /// Italic
  #[sdk(child(qname = "x:CT_BooleanProperty/x:i"))]
  pub italic: Option<Italic>,
  /// Strike Through
  #[sdk(child(qname = "x:CT_BooleanProperty/x:strike"))]
  pub strike: Option<Strike>,
  /// Condense
  #[sdk(child(qname = "x:CT_BooleanProperty/x:condense"))]
  pub condense: Option<Condense>,
  /// Extend
  #[sdk(child(qname = "x:CT_BooleanProperty/x:extend"))]
  pub extend: Option<Extend>,
  /// Outline
  #[sdk(child(qname = "x:CT_BooleanProperty/x:outline"))]
  pub outline: Option<Outline>,
  /// Shadow
  #[sdk(child(qname = "x:CT_BooleanProperty/x:shadow"))]
  pub shadow: Option<Shadow>,
  /// Underline
  #[sdk(child(qname = "x:CT_UnderlineProperty/x:u"))]
  pub underline: Option<Underline>,
  /// Text Vertical Alignment
  #[sdk(child(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  /// Font Size
  #[sdk(child(qname = "x:CT_FontSize/x:sz"))]
  pub font_size: Option<FontSize>,
  /// Text Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
  /// Font Name
  #[sdk(child(qname = "x:CT_FontNameNonEmpty/x:name"))]
  pub font_name: Option<FontName>,
  /// Font Family
  #[sdk(child(qname = "x:CT_FontFamilyNum/x:family"))]
  pub font_family_numbering: Option<FontFamilyNumbering>,
  /// Character Set
  #[sdk(child(qname = "x:CT_ByteProperty/x:charset"))]
  pub font_char_set: Option<FontCharSet>,
  /// Scheme
  #[sdk(child(qname = "x:CT_FontScheme/x:scheme"))]
  pub font_scheme: Option<FontScheme>,
}
/// Fill.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fill/x:fill")]
pub struct Fill {
  #[sdk(choice(
    qname = "x:CT_PatternFill/x:patternFill",
    qname = "x:CT_GradientFill/x:gradientFill"
  ))]
  pub fill_choice: Option<FillChoice>,
}
/// Border Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Border/x:border")]
pub struct Border {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Diagonal Up
  #[sdk(attr(qname = ":diagonalUp"))]
  pub diagonal_up: Option<crate::simple_type::BooleanValue>,
  /// Diagonal Down
  #[sdk(attr(qname = ":diagonalDown"))]
  pub diagonal_down: Option<crate::simple_type::BooleanValue>,
  /// Outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Defines the StartBorder Class.
  #[sdk(child(office2010, qname = "x:CT_BorderPr/x:start"))]
  pub start_border: Option<std::boxed::Box<StartBorder>>,
  /// Defines the EndBorder Class.
  #[sdk(child(office2010, qname = "x:CT_BorderPr/x:end"))]
  pub end_border: Option<std::boxed::Box<EndBorder>>,
  /// Left Border
  #[sdk(child(qname = "x:CT_BorderPr/x:left"))]
  pub left_border: Option<std::boxed::Box<LeftBorder>>,
  /// Right Border
  #[sdk(child(qname = "x:CT_BorderPr/x:right"))]
  pub right_border: Option<std::boxed::Box<RightBorder>>,
  /// Top Border
  #[sdk(child(qname = "x:CT_BorderPr/x:top"))]
  pub top_border: Option<std::boxed::Box<TopBorder>>,
  /// Bottom Border
  #[sdk(child(qname = "x:CT_BorderPr/x:bottom"))]
  pub bottom_border: Option<std::boxed::Box<BottomBorder>>,
  /// Diagonal
  #[sdk(child(qname = "x:CT_BorderPr/x:diagonal"))]
  pub diagonal_border: Option<std::boxed::Box<DiagonalBorder>>,
  /// Vertical Inner Border
  #[sdk(child(qname = "x:CT_BorderPr/x:vertical"))]
  pub vertical_border: Option<std::boxed::Box<VerticalBorder>>,
  /// Horizontal Inner Borders
  #[sdk(child(qname = "x:CT_BorderPr/x:horizontal"))]
  pub horizontal_border: Option<std::boxed::Box<HorizontalBorder>>,
}
/// Color Indexes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IndexedColors/x:indexedColors")]
pub struct IndexedColors {
  /// RGB Color.
  #[sdk(child(qname = "x:CT_RgbColor/x:rgbColor"))]
  pub x_rgb_color: Vec<RgbColor>,
}
/// MRU Colors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MRUColors/x:mruColors")]
pub struct MruColors {
  /// Text Color.
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyle/x:tableStyle")]
pub struct TableStyle {
  /// Table Style Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Pivot Style
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// Table
  #[sdk(attr(qname = ":table"))]
  pub table: Option<crate::simple_type::BooleanValue>,
  /// Table Style Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Table Style.
  #[sdk(child(qname = "x:CT_TableStyleElement/x:tableStyleElement"))]
  pub x_table_style_element: Vec<TableStyleElement>,
}
/// RGB Color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RgbColor/x:rgbColor")]
pub struct RgbColor {
  /// Alpha Red Green Blue
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
}
/// Cell Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyle/x:cellStyle")]
pub struct CellStyle {
  pub xml_other_attrs: Vec<(String, String)>,
  /// User Defined Cell Style
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Format Id
  #[sdk(attr(qname = ":xfId"))]
  pub format_id: crate::simple_type::UInt32Value,
  /// Built-In Style Id
  #[sdk(attr(qname = ":builtinId"))]
  pub builtin_id: Option<crate::simple_type::UInt32Value>,
  /// Outline Style
  #[sdk(attr(qname = ":iLevel"))]
  pub outline_level: Option<crate::simple_type::UInt32Value>,
  /// Hidden Style
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Custom Built In
  #[sdk(attr(qname = ":customBuiltin"))]
  pub custom_builtin: Option<crate::simple_type::BooleanValue>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Formatting Elements.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xf/x:xf")]
pub struct CellFormat {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Number Format Id
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Font Id
  #[sdk(attr(qname = ":fontId"))]
  pub font_id: Option<crate::simple_type::UInt32Value>,
  /// Fill Id
  #[sdk(attr(qname = ":fillId"))]
  pub fill_id: Option<crate::simple_type::UInt32Value>,
  /// Border Id
  #[sdk(attr(qname = ":borderId"))]
  pub border_id: Option<crate::simple_type::UInt32Value>,
  /// Format Id
  #[sdk(attr(qname = ":xfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Quote Prefix
  #[sdk(attr(qname = ":quotePrefix"))]
  pub quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Pivot Button
  #[sdk(attr(qname = ":pivotButton"))]
  pub pivot_button: Option<crate::simple_type::BooleanValue>,
  /// Apply Number Format
  #[sdk(attr(qname = ":applyNumberFormat"))]
  pub apply_number_format: Option<crate::simple_type::BooleanValue>,
  /// Apply Font
  #[sdk(attr(qname = ":applyFont"))]
  pub apply_font: Option<crate::simple_type::BooleanValue>,
  /// Apply Fill
  #[sdk(attr(qname = ":applyFill"))]
  pub apply_fill: Option<crate::simple_type::BooleanValue>,
  /// Apply Border
  #[sdk(attr(qname = ":applyBorder"))]
  pub apply_border: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment
  #[sdk(attr(qname = ":applyAlignment"))]
  pub apply_alignment: Option<crate::simple_type::BooleanValue>,
  /// Apply Protection
  #[sdk(attr(qname = ":applyProtection"))]
  pub apply_protection: Option<crate::simple_type::BooleanValue>,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  /// Protection
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Font Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontNameNonEmpty/x:name")]
pub struct FontName {
  /// String Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_length(min = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Font Family.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontFamilyNum/x:family")]
pub struct FontFamilyNumbering {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 5))]
  pub val: crate::simple_type::Int32Value,
}
/// Character Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ByteProperty/x:charset")]
pub struct FontCharSet {
  /// Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 255))]
  pub val: crate::simple_type::Int32Value,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyleElement/x:tableStyleElement")]
pub struct TableStyleElement {
  /// Table Style Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: TableStyleValues,
  /// Band Size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Formatting Id
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defined Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalDefinedName/x:definedName")]
pub struct ExternalDefinedName {
  /// Defined Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Refers To
  #[sdk(attr(qname = ":refersTo"))]
  pub refers_to: Option<crate::simple_type::StringValue>,
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// External Sheet Data Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetData/x:sheetData")]
pub struct ExternalSheetData {
  /// Sheet Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Last Refresh Resulted in Error
  #[sdk(attr(qname = ":refreshError"))]
  pub refresh_error: Option<crate::simple_type::BooleanValue>,
  /// Row.
  #[sdk(child(qname = "x:CT_ExternalRow/x:row"))]
  pub x_row: Vec<ExternalRow>,
}
/// Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalRow/x:row")]
pub struct ExternalRow {
  /// Row
  #[sdk(attr(qname = ":r"))]
  pub row_index: crate::simple_type::UInt32Value,
  /// External Cell Data.
  #[sdk(child(qname = "x:CT_ExternalCell/x:cell"))]
  pub x_cell: Vec<ExternalCell>,
}
/// External Cell Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalCell/x:cell")]
pub struct ExternalCell {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Type
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Value Metadata
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value
  #[sdk(text_child(qname = "x:ST_Xstring/x:v"))]
  pub xstring: Option<crate::simple_type::StringValue>,
}
/// DDE Items Collection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeItems/x:ddeItems")]
pub struct DdeItems {
  /// DDE Item definition.
  #[sdk(child(qname = "x:CT_DdeItem/x:ddeItem"))]
  pub x_dde_item: Vec<DdeItem>,
}
/// DDE Item definition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeItem/x:ddeItem")]
pub struct DdeItem {
  /// DDE Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// OLE
  #[sdk(attr(qname = ":ole"))]
  pub use_ole: Option<crate::simple_type::BooleanValue>,
  /// Advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// Data is an Image
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
  /// DDE Name Values
  #[sdk(child(qname = "x:CT_DdeValues/x:values"))]
  pub values: Option<Values>,
}
/// DDE Name Values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeValues/x:values")]
pub struct Values {
  /// Rows
  #[sdk(attr(qname = ":rows"))]
  pub rows: Option<crate::simple_type::UInt32Value>,
  /// Columns
  #[sdk(attr(qname = ":cols"))]
  pub columns: Option<crate::simple_type::UInt32Value>,
  /// Value.
  #[sdk(child(qname = "x:CT_DdeValue/x:value"))]
  pub x_value: Vec<Value>,
}
/// Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeValue/x:value")]
pub struct Value {
  /// DDE Value Type
  #[sdk(attr(qname = ":t"))]
  pub value_type: Option<DdeValues>,
  /// DDE Link Value
  #[sdk(child(qname = "x:CT_Xstring/x:val"))]
  pub dde_link_value: std::boxed::Box<DdeLinkValue>,
}
/// OLE Link Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleItems/x:oleItems")]
pub struct OleItems {
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "x:CT_OleItem/x:oleItem",
    qname = "x14:CT_OleItem/x14:oleItem",
    text,
    any
  ))]
  pub ole_items_choice: Vec<OleItemsChoice>,
}
/// External Workbook.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalBook/x:externalBook")]
pub struct ExternalBook {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Relationship to supporting book file path
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Alternate URLs and identifiers of the external book
  #[sdk(child(
    microsoft365,
    qname = "xxl21:CT_ExternalBookAlternateUrls/xxl21:alternateUrls"
  ))]
  pub external_book_alternate_urls:
    Option<std::boxed::Box<crate::schemas::xxl21::ExternalBookAlternateUrls>>,
  /// Sheet names of supporting book
  #[sdk(child(qname = "x:CT_ExternalSheetNames/x:sheetNames"))]
  pub sheet_names: Option<SheetNames>,
  /// Defined names associated with supporting book.
  #[sdk(child(qname = "x:CT_ExternalDefinedNames/x:definedNames"))]
  pub external_defined_names: Option<ExternalDefinedNames>,
  /// Cached worksheet data associated with supporting book
  #[sdk(child(qname = "x:CT_ExternalSheetDataSet/x:sheetDataSet"))]
  pub sheet_data_set: Option<SheetDataSet>,
}
/// DDE Connection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeLink/x:ddeLink")]
pub struct DdeLink {
  /// Service name
  #[sdk(attr(qname = ":ddeService"))]
  pub dde_service: crate::simple_type::StringValue,
  /// Topic for DDE server
  #[sdk(attr(qname = ":ddeTopic"))]
  pub dde_topic: crate::simple_type::StringValue,
  /// DDE Items Collection
  #[sdk(child(qname = "x:CT_DdeItems/x:ddeItems"))]
  pub dde_items: Option<DdeItems>,
}
/// OLE Link.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleLink/x:oleLink")]
pub struct OleLink {
  /// OLE Link Relationship
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// OLE Link ProgID
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: crate::simple_type::StringValue,
  /// OLE Link Items
  #[sdk(child(qname = "x:CT_OleItems/x:oleItems"))]
  pub ole_items: Option<OleItems>,
}
/// Sheet Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetName/x:sheetName")]
pub struct SheetName {
  /// Sheet Name Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Value.
pub type Xstring = crate::simple_type::StringValue;
/// Table Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableColumn/x:tableColumn")]
pub struct TableColumn {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Table Field Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Unique Name
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// Column name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Totals Row Function
  #[sdk(attr(qname = ":totalsRowFunction"))]
  pub totals_row_function: Option<TotalsRowFunctionValues>,
  /// Totals Row Label
  #[sdk(attr(qname = ":totalsRowLabel"))]
  pub totals_row_label: Option<crate::simple_type::StringValue>,
  /// Query Table Field Id
  #[sdk(attr(qname = ":queryTableFieldId"))]
  pub query_table_field_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Cell Format Id
  #[sdk(attr(qname = ":headerRowDxfId"))]
  pub header_row_differential_formatting_id: Option<crate::simple_type::UInt32Value>,
  /// Data and Insert Row Format Id
  #[sdk(attr(qname = ":dataDxfId"))]
  pub data_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Format Id
  #[sdk(attr(qname = ":totalsRowDxfId"))]
  pub totals_row_differential_formatting_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Cell Style
  #[sdk(attr(qname = ":headerRowCellStyle"))]
  pub header_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Data Area Style Name
  #[sdk(attr(qname = ":dataCellStyle"))]
  pub data_cell_style: Option<crate::simple_type::StringValue>,
  /// Totals Row Style Name
  #[sdk(attr(qname = ":totalsRowCellStyle"))]
  pub totals_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Calculated Column Formula
  #[sdk(child(qname = "x:CT_TableFormula/x:calculatedColumnFormula"))]
  pub calculated_column_formula: Option<CalculatedColumnFormula>,
  /// Totals Row Formula
  #[sdk(child(qname = "x:CT_TableFormula/x:totalsRowFormula"))]
  pub totals_row_formula: Option<TotalsRowFormula>,
  /// XML Column Properties
  #[sdk(child(qname = "x:CT_XmlColumnPr/x:xmlColumnPr"))]
  pub xml_column_properties: Option<std::boxed::Box<XmlColumnProperties>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Column Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableFormula/x:calculatedColumnFormula")]
pub struct CalculatedColumnFormula {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Totals Row Formula.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableFormula/x:totalsRowFormula")]
pub struct TotalsRowFormula {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::xml::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// XML Column Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlColumnPr/x:xmlColumnPr")]
pub struct XmlColumnProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  /// XML Map Id
  #[sdk(attr(qname = ":mapId"))]
  pub map_id: crate::simple_type::UInt32Value,
  /// XPath
  #[sdk(attr(qname = ":xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Denormalized
  #[sdk(attr(qname = ":denormalized"))]
  pub denormalized: Option<crate::simple_type::BooleanValue>,
  /// XML Data Type
  #[sdk(attr(qname = ":xmlDataType"))]
  pub xml_data_type: XmlDataValues,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Volatile Dependency Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolType/x:volType")]
pub struct VolatileType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: VolatileDependencyValues,
  /// Main.
  #[sdk(child(qname = "x:CT_VolMain/x:main"))]
  pub x_main: Vec<Main>,
}
/// Main.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolMain/x:main")]
pub struct Main {
  /// First String
  #[sdk(attr(qname = ":first"))]
  pub first: crate::simple_type::StringValue,
  /// Topic.
  #[sdk(child(qname = "x:CT_VolTopic/x:tp"))]
  pub x_tp: Vec<Topic>,
}
/// Topic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTopic/x:tp")]
pub struct Topic {
  /// Type
  #[sdk(attr(qname = ":t"))]
  pub value_type: Option<VolatileValues>,
  /// Topic Value
  #[sdk(text_child(qname = "x:ST_Xstring/x:v"))]
  pub xstring: crate::simple_type::StringValue,
  /// Strings in Subtopic.
  #[sdk(child(qname = "x:CT_Xstring/x:stp"))]
  pub x_stp: Vec<Subtopic>,
  /// References.
  #[sdk(child(qname = "x:CT_VolTopicRef/x:tr"))]
  pub x_tr: Vec<TopicReferences>,
}
/// References.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTopicRef/x:tr")]
pub struct TopicReferences {
  /// Reference
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Sheet Id
  #[sdk(attr(qname = ":s"))]
  pub sheet_id: crate::simple_type::UInt32Value,
}
/// PivotCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCache/x:pivotCache")]
pub struct PivotCache {
  pub xml_other_attrs: Vec<(String, String)>,
  /// PivotCache Id
  #[sdk(attr(qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Web Publishing Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishObject/x:webPublishObject")]
pub struct WebPublishObject {
  /// Id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Div Id
  #[sdk(attr(qname = ":divId"))]
  pub div_id: crate::simple_type::StringValue,
  /// Source Object
  #[sdk(attr(qname = ":sourceObject"))]
  pub source_object: Option<crate::simple_type::StringValue>,
  /// Destination File
  #[sdk(attr(qname = ":destinationFile"))]
  pub destination_file: crate::simple_type::StringValue,
  /// Title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Auto Republish
  #[sdk(attr(qname = ":autoRepublish"))]
  pub auto_republish: Option<crate::simple_type::BooleanValue>,
}
/// External Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalReference/x:externalReference")]
pub struct ExternalReference {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Custom Workbook View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomWorkbookView/x:customWorkbookView")]
pub struct CustomWorkbookView {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom View Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Custom View GUID
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Auto Update
  #[sdk(attr(qname = ":autoUpdate"))]
  pub auto_update: Option<crate::simple_type::BooleanValue>,
  /// Merge Interval
  #[sdk(attr(qname = ":mergeInterval"))]
  pub merge_interval: Option<crate::simple_type::UInt32Value>,
  /// Changes Saved Win
  #[sdk(attr(qname = ":changesSavedWin"))]
  pub changes_saved_win: Option<crate::simple_type::BooleanValue>,
  /// Only Synch
  #[sdk(attr(qname = ":onlySync"))]
  pub only_sync: Option<crate::simple_type::BooleanValue>,
  /// Personal View
  #[sdk(attr(qname = ":personalView"))]
  pub personal_view: Option<crate::simple_type::BooleanValue>,
  /// Include Print Settings
  #[sdk(attr(qname = ":includePrintSettings"))]
  pub include_print_settings: Option<crate::simple_type::BooleanValue>,
  /// Include Hidden Rows and Columns
  #[sdk(attr(qname = ":includeHiddenRowCol"))]
  pub include_hidden_row_column: Option<crate::simple_type::BooleanValue>,
  /// Maximized
  #[sdk(attr(qname = ":maximized"))]
  pub maximized: Option<crate::simple_type::BooleanValue>,
  /// Minimized
  #[sdk(attr(qname = ":minimized"))]
  pub minimized: Option<crate::simple_type::BooleanValue>,
  /// Show Horizontal Scroll
  #[sdk(attr(qname = ":showHorizontalScroll"))]
  pub show_horizontal_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Vertical Scroll
  #[sdk(attr(qname = ":showVerticalScroll"))]
  pub show_vertical_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Sheet Tabs
  #[sdk(attr(qname = ":showSheetTabs"))]
  pub show_sheet_tabs: Option<crate::simple_type::BooleanValue>,
  /// Top Left Corner (X Coordinate)
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::Int32Value>,
  /// Top Left Corner (Y Coordinate)
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::Int32Value>,
  /// Window Width
  #[sdk(attr(qname = ":windowWidth"))]
  pub window_width: Option<crate::simple_type::UInt32Value>,
  /// Window Height
  #[sdk(attr(qname = ":windowHeight"))]
  pub window_height: Option<crate::simple_type::UInt32Value>,
  /// Sheet Tab Ratio
  #[sdk(attr(qname = ":tabRatio"))]
  pub tab_ratio: Option<crate::simple_type::UInt32Value>,
  /// Active Sheet in Book View
  #[sdk(attr(qname = ":activeSheetId"))]
  pub active_sheet_id: crate::simple_type::UInt32Value,
  /// Show Formula Bar
  #[sdk(attr(qname = ":showFormulaBar"))]
  pub show_formula_bar: Option<crate::simple_type::BooleanValue>,
  /// Show Status Bar
  #[sdk(attr(qname = ":showStatusbar"))]
  pub show_statusbar: Option<crate::simple_type::BooleanValue>,
  /// Show Comments
  #[sdk(attr(qname = ":showComments"))]
  pub show_comments: Option<CommentsValues>,
  /// Show Objects
  #[sdk(attr(qname = ":showObjects"))]
  pub show_objects: Option<ObjectDisplayValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Sheet Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sheet/x:sheet")]
pub struct Sheet {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Sheet Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Sheet Tab Id
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Visible State
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Workbook View.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BookView/x:workbookView")]
pub struct WorkbookView {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Visibility
  #[sdk(attr(qname = ":visibility"))]
  pub visibility: Option<VisibilityValues>,
  /// Minimized
  #[sdk(attr(qname = ":minimized"))]
  pub minimized: Option<crate::simple_type::BooleanValue>,
  /// Show Horizontal Scroll
  #[sdk(attr(qname = ":showHorizontalScroll"))]
  pub show_horizontal_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Vertical Scroll
  #[sdk(attr(qname = ":showVerticalScroll"))]
  pub show_vertical_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Sheet Tabs
  #[sdk(attr(qname = ":showSheetTabs"))]
  pub show_sheet_tabs: Option<crate::simple_type::BooleanValue>,
  /// Upper Left Corner (X Coordinate)
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::Int32Value>,
  /// Upper Left Corner (Y Coordinate)
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::Int32Value>,
  /// Window Width
  #[sdk(attr(qname = ":windowWidth"))]
  pub window_width: Option<crate::simple_type::UInt32Value>,
  /// Window Height
  #[sdk(attr(qname = ":windowHeight"))]
  pub window_height: Option<crate::simple_type::UInt32Value>,
  /// Sheet Tab Ratio
  #[sdk(attr(qname = ":tabRatio"))]
  pub tab_ratio: Option<crate::simple_type::UInt32Value>,
  /// First Sheet
  #[sdk(attr(qname = ":firstSheet"))]
  pub first_sheet: Option<crate::simple_type::UInt32Value>,
  /// Active Sheet Index
  #[sdk(attr(qname = ":activeTab"))]
  pub active_tab: Option<crate::simple_type::UInt32Value>,
  /// AutoFilter Date Grouping
  #[sdk(attr(qname = ":autoFilterDateGrouping"))]
  pub auto_filter_date_grouping: Option<crate::simple_type::BooleanValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defined Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DefinedName/x:definedName")]
pub struct DefinedName {
  /// Defined Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Custom Menu Text
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// Description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Help
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// Status Bar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// Local Name Sheet Id
  #[sdk(attr(qname = ":localSheetId"))]
  pub local_sheet_id: Option<crate::simple_type::UInt32Value>,
  /// Hidden Name
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// Procedure
  #[sdk(attr(qname = ":vbProcedure"))]
  pub vb_procedure: Option<crate::simple_type::BooleanValue>,
  /// External Function
  #[sdk(attr(qname = ":xlm"))]
  pub xlm: Option<crate::simple_type::BooleanValue>,
  /// Function Group Id
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::UInt32Value>,
  /// Shortcut Key
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::StringValue>,
  /// Publish To Server
  #[sdk(attr(qname = ":publishToServer"))]
  pub publish_to_server: Option<crate::simple_type::BooleanValue>,
  /// Workbook Parameter (Server)
  #[sdk(attr(qname = ":workbookParameter"))]
  pub workbook_parameter: Option<crate::simple_type::BooleanValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Function Group.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FunctionGroup/x:functionGroup")]
pub struct FunctionGroup {
  /// Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the ObjectAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_ObjectAnchor/x:anchor")]
pub struct ObjectAnchor {
  /// moveWithCells
  #[sdk(attr(qname = ":moveWithCells"))]
  pub move_with_cells: Option<crate::simple_type::BooleanValue>,
  /// sizeWithCells
  #[sdk(attr(qname = ":sizeWithCells"))]
  pub size_with_cells: Option<crate::simple_type::BooleanValue>,
  /// z-order
  #[sdk(attr(qname = ":z-order"))]
  pub z_order: Option<crate::simple_type::UInt32Value>,
  /// Defines the FromMarker Class.
  #[sdk(child(office2010, qname = "xdr:CT_Marker/x:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  /// Defines the ToMarker Class.
  #[sdk(child(office2010, qname = "xdr:CT_Marker/x:to"))]
  pub to_marker: std::boxed::Box<ToMarker>,
}
/// Defines the FromMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "xdr:CT_Marker/x:from")]
pub struct FromMarker {
  /// Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  /// Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  /// Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  /// Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Defines the ToMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "xdr:CT_Marker/x:to")]
pub struct ToMarker {
  /// Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  /// Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  /// Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  /// Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Defines the ConditionalFormattingRuleExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRuleExtension/x:ext")]
pub struct ConditionalFormattingRuleExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "x:ST_Guid/x14:id", any))]
  pub conditional_formatting_rule_extension_choice:
    Option<ConditionalFormattingRuleExtensionChoice>,
}
/// Defines the PivotHierarchyExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchyExtension/x:ext")]
pub struct PivotHierarchyExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "x14:CT_PivotHierarchy/x14:pivotHierarchy", any))]
  pub pivot_hierarchy_extension_choice: Option<PivotHierarchyExtensionChoice>,
}
/// Defines the PivotFieldExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFieldExtension/x:ext")]
pub struct PivotFieldExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "x14:CT_PivotField/x14:pivotField", any))]
  pub pivot_field_extension_choice: Option<PivotFieldExtensionChoice>,
}
/// Defines the CacheSourceExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSourceExtension/x:ext")]
pub struct CacheSourceExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "x14:CT_SourceConnection/x14:sourceConnection", any))]
  pub cache_source_extension_choice: Option<CacheSourceExtensionChoice>,
}
/// OLE Link Item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleItem/x:oleItem")]
pub struct OleItem {
  /// OLE Name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Icon
  #[sdk(attr(qname = ":icon"))]
  pub icon: Option<crate::simple_type::BooleanValue>,
  /// Advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// Object is an Image
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
}
/// Defines the StartBorder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_BorderPr/x:start")]
pub struct StartBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Defines the EndBorder Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_BorderPr/x:end")]
pub struct EndBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Left Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:left")]
pub struct LeftBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Right Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:right")]
pub struct RightBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Top Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:top")]
pub struct TopBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Bottom Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:bottom")]
pub struct BottomBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Diagonal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:diagonal")]
pub struct DiagonalBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Vertical Inner Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:vertical")]
pub struct VerticalBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Horizontal Inner Borders.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:horizontal")]
pub struct HorizontalBorder {
  /// Line Style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  /// Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Defines the ControlProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_ControlPr/x:controlPr")]
pub struct ControlProperties {
  /// locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// recalcAlways
  #[sdk(attr(qname = ":recalcAlways"))]
  pub recalc_always: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// autoPict
  #[sdk(attr(qname = ":autoPict"))]
  pub auto_pict: Option<crate::simple_type::BooleanValue>,
  /// macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// linkedCell
  #[sdk(attr(qname = ":linkedCell"))]
  pub linked_cell: Option<crate::simple_type::StringValue>,
  /// listFillRange
  #[sdk(attr(qname = ":listFillRange"))]
  pub list_fill_range: Option<crate::simple_type::StringValue>,
  /// cf
  #[sdk(attr(qname = ":cf"))]
  pub cf: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
  /// Defines the ObjectAnchor Class.
  #[sdk(child(office2010, qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
/// Defines the EmbeddedObjectProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_ObjectPr/x:objectPr")]
pub struct EmbeddedObjectProperties {
  /// locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// autoPict
  #[sdk(attr(qname = ":autoPict"))]
  pub auto_pict: Option<crate::simple_type::BooleanValue>,
  /// macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// dde
  #[sdk(attr(qname = ":dde"))]
  pub dde: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
  /// Defines the ObjectAnchor Class.
  #[sdk(child(office2010, qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
/// Chart Sheet Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetPr/x:sheetPr")]
pub struct ChartSheetProperties {
  /// Published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Code Name
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// Sheet Tab Color.
  #[sdk(child(qname = "x:CT_Color/x:tabColor"))]
  pub tab_color: Option<TabColor>,
}
/// Chart Sheet Views.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetViews/x:sheetViews")]
pub struct ChartSheetViews {
  /// Chart Sheet View.
  #[sdk(child(qname = "x:CT_ChartsheetView/x:sheetView"))]
  pub x_sheet_view: Vec<ChartSheetView>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Chart Sheet Protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetProtection/x:sheetProtection")]
pub struct ChartSheetProtection {
  /// Password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// Cryptographic Algorithm Name
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// Contents
  #[sdk(attr(qname = ":content"))]
  pub content: Option<crate::simple_type::BooleanValue>,
  /// Objects Locked
  #[sdk(attr(qname = ":objects"))]
  pub objects: Option<crate::simple_type::BooleanValue>,
}
/// Custom Chart Sheet Views.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomChartsheetViews/x:customSheetViews")]
pub struct CustomChartsheetViews {
  /// Custom Chart Sheet View.
  #[sdk(child(qname = "x:CT_CustomChartsheetView/x:customSheetView"))]
  pub x_custom_sheet_view: Vec<CustomChartsheetView>,
}
/// Drawing.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Drawing/x:drawing")]
pub struct Drawing {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Relationship id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the LegacyDrawing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LegacyDrawing/x:legacyDrawing")]
pub struct LegacyDrawing {
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Legacy Drawing Reference in  Header Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the DrawingHeaderFooter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DrawingHF/x:drawingHF")]
pub struct DrawingHeaderFooter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// lho
  #[sdk(attr(qname = ":lho"))]
  pub lho: Option<crate::simple_type::UInt32Value>,
  /// lhe
  #[sdk(attr(qname = ":lhe"))]
  pub lhe: Option<crate::simple_type::UInt32Value>,
  /// lhf
  #[sdk(attr(qname = ":lhf"))]
  pub lhf: Option<crate::simple_type::UInt32Value>,
  /// cho
  #[sdk(attr(qname = ":cho"))]
  pub cho: Option<crate::simple_type::UInt32Value>,
  /// che
  #[sdk(attr(qname = ":che"))]
  pub che: Option<crate::simple_type::UInt32Value>,
  /// chf
  #[sdk(attr(qname = ":chf"))]
  pub chf: Option<crate::simple_type::UInt32Value>,
  /// rho
  #[sdk(attr(qname = ":rho"))]
  pub rho: Option<crate::simple_type::UInt32Value>,
  /// rhe
  #[sdk(attr(qname = ":rhe"))]
  pub rhe: Option<crate::simple_type::UInt32Value>,
  /// rhf
  #[sdk(attr(qname = ":rhf"))]
  pub rhf: Option<crate::simple_type::UInt32Value>,
  /// lfo
  #[sdk(attr(qname = ":lfo"))]
  pub lfo: Option<crate::simple_type::UInt32Value>,
  /// lfe
  #[sdk(attr(qname = ":lfe"))]
  pub lfe: Option<crate::simple_type::UInt32Value>,
  /// lff
  #[sdk(attr(qname = ":lff"))]
  pub lff: Option<crate::simple_type::UInt32Value>,
  /// cfo
  #[sdk(attr(qname = ":cfo"))]
  pub cfo: Option<crate::simple_type::UInt32Value>,
  /// cfe
  #[sdk(attr(qname = ":cfe"))]
  pub cfe: Option<crate::simple_type::UInt32Value>,
  /// cff
  #[sdk(attr(qname = ":cff"))]
  pub cff: Option<crate::simple_type::UInt32Value>,
  /// rfo
  #[sdk(attr(qname = ":rfo"))]
  pub rfo: Option<crate::simple_type::UInt32Value>,
  /// rfe
  #[sdk(attr(qname = ":rfe"))]
  pub rfe: Option<crate::simple_type::UInt32Value>,
  /// rff
  #[sdk(attr(qname = ":rff"))]
  pub rff: Option<crate::simple_type::UInt32Value>,
}
/// Defines the Picture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetBackgroundPicture/x:picture")]
pub struct Picture {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the WebPublishItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishItems/x:webPublishItems")]
pub struct WebPublishItems {
  /// Web Publishing Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Web Publishing Item.
  #[sdk(child(qname = "x:CT_WebPublishItem/x:webPublishItem"))]
  pub x_web_publish_item: Vec<WebPublishItem>,
}
/// Color Scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColorScale/x:colorScale")]
pub struct ColorScale {
  /// Conditional Format Value Object.
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
  /// Text Color.
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
}
/// Data Bar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataBar/x:dataBar")]
pub struct DataBar {
  /// Minimum Length
  #[sdk(attr(qname = ":minLength"))]
  pub min_length: Option<crate::simple_type::UInt32Value>,
  /// Maximum Length
  #[sdk(attr(qname = ":maxLength"))]
  pub max_length: Option<crate::simple_type::UInt32Value>,
  /// Show Values
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// Conditional Format Value Object.
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
  /// Text Color.
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: std::boxed::Box<Color>,
}
/// Icon Set.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IconSet/x:iconSet")]
pub struct IconSet {
  /// Icon Set
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set_value: Option<IconSetValues>,
  /// Show Value
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// Percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Reverse Icons
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// Conditional Format Value Object.
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
}
/// Defines the ConditionalFormattingRuleExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRuleExtensionList/x:extLst")]
pub struct ConditionalFormattingRuleExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the ConditionalFormattingRuleExtension Class.
  #[sdk(child(qname = "x:CT_CfRuleExtension/x:ext"))]
  pub x_ext: Vec<ConditionalFormattingRuleExtension>,
}
/// Data Consolidation References.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataRefs/x:dataRefs")]
pub struct DataReferences {
  /// Data Consolidation Reference Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Data Consolidation Reference.
  #[sdk(child(qname = "x:CT_DataRef/x:dataRef"))]
  pub x_data_ref: Vec<DataReference>,
}
/// Sheet Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetPr/x:sheetPr")]
pub struct SheetProperties {
  /// Synch Horizontal
  #[sdk(attr(qname = ":syncHorizontal"))]
  pub sync_horizontal: Option<crate::simple_type::BooleanValue>,
  /// Synch Vertical
  #[sdk(attr(qname = ":syncVertical"))]
  pub sync_vertical: Option<crate::simple_type::BooleanValue>,
  /// Synch Reference
  #[sdk(attr(qname = ":syncRef"))]
  pub sync_reference: Option<crate::simple_type::StringValue>,
  /// Transition Formula Evaluation
  #[sdk(attr(qname = ":transitionEvaluation"))]
  pub transition_evaluation: Option<crate::simple_type::BooleanValue>,
  /// Transition Formula Entry
  #[sdk(attr(qname = ":transitionEntry"))]
  pub transition_entry: Option<crate::simple_type::BooleanValue>,
  /// Published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Code Name
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// Filter Mode
  #[sdk(attr(qname = ":filterMode"))]
  pub filter_mode: Option<crate::simple_type::BooleanValue>,
  /// Enable Conditional Formatting Calculations
  #[sdk(attr(qname = ":enableFormatConditionsCalculation"))]
  pub enable_format_conditions_calculation: Option<crate::simple_type::BooleanValue>,
  /// Sheet Tab Color
  #[sdk(child(qname = "x:CT_Color/x:tabColor"))]
  pub tab_color: Option<TabColor>,
  /// Outline Properties
  #[sdk(child(qname = "x:CT_OutlinePr/x:outlinePr"))]
  pub outline_properties: Option<OutlineProperties>,
  /// Page Setup Properties
  #[sdk(child(qname = "x:CT_PageSetUpPr/x:pageSetUpPr"))]
  pub page_setup_properties: Option<PageSetupProperties>,
}
/// Dialog Sheet Views.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetViews/x:sheetViews")]
pub struct SheetViews {
  /// Worksheet View.
  #[sdk(child(qname = "x:CT_SheetView/x:sheetView"))]
  pub x_sheet_view: Vec<SheetView>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Dialog Sheet Format Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetFormatPr/x:sheetFormatPr")]
pub struct SheetFormatProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Base Column Width
  #[sdk(attr(qname = ":baseColWidth"))]
  pub base_column_width: Option<crate::simple_type::UInt32Value>,
  /// Default Column Width
  #[sdk(attr(qname = ":defaultColWidth"))]
  pub default_column_width: Option<crate::simple_type::DoubleValue>,
  /// Default Row Height
  #[sdk(attr(qname = ":defaultRowHeight"))]
  pub default_row_height: crate::simple_type::DoubleValue,
  /// Custom Height
  #[sdk(attr(qname = ":customHeight"))]
  pub custom_height: Option<crate::simple_type::BooleanValue>,
  /// Hidden By Default
  #[sdk(attr(qname = ":zeroHeight"))]
  pub zero_height: Option<crate::simple_type::BooleanValue>,
  /// Thick Top Border
  #[sdk(attr(qname = ":thickTop"))]
  pub thick_top: Option<crate::simple_type::BooleanValue>,
  /// Thick Bottom Border
  #[sdk(attr(qname = ":thickBottom"))]
  pub thick_bottom: Option<crate::simple_type::BooleanValue>,
  /// Maximum Outline Row
  #[sdk(attr(qname = ":outlineLevelRow"))]
  pub outline_level_row: Option<crate::simple_type::ByteValue>,
  /// Column Outline Level
  #[sdk(attr(qname = ":outlineLevelCol"))]
  pub outline_level_column: Option<crate::simple_type::ByteValue>,
  /// dyDescent
  #[sdk(attr(office2010, qname = "x14ac:dyDescent"))]
  pub dy_descent: Option<crate::simple_type::DoubleValue>,
}
/// Sheet Protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetProtection/x:sheetProtection")]
pub struct SheetProtection {
  /// Password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// Cryptographic Algorithm Name
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// Sheet Locked
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::BooleanValue>,
  /// Objects Locked
  #[sdk(attr(qname = ":objects"))]
  pub objects: Option<crate::simple_type::BooleanValue>,
  /// Scenarios Locked
  #[sdk(attr(qname = ":scenarios"))]
  pub scenarios: Option<crate::simple_type::BooleanValue>,
  /// Format Cells Locked
  #[sdk(attr(qname = ":formatCells"))]
  pub format_cells: Option<crate::simple_type::BooleanValue>,
  /// Format Columns Locked
  #[sdk(attr(qname = ":formatColumns"))]
  pub format_columns: Option<crate::simple_type::BooleanValue>,
  /// Format Rows Locked
  #[sdk(attr(qname = ":formatRows"))]
  pub format_rows: Option<crate::simple_type::BooleanValue>,
  /// Insert Columns Locked
  #[sdk(attr(qname = ":insertColumns"))]
  pub insert_columns: Option<crate::simple_type::BooleanValue>,
  /// Insert Rows Locked
  #[sdk(attr(qname = ":insertRows"))]
  pub insert_rows: Option<crate::simple_type::BooleanValue>,
  /// Insert Hyperlinks Locked
  #[sdk(attr(qname = ":insertHyperlinks"))]
  pub insert_hyperlinks: Option<crate::simple_type::BooleanValue>,
  /// Delete Columns Locked
  #[sdk(attr(qname = ":deleteColumns"))]
  pub delete_columns: Option<crate::simple_type::BooleanValue>,
  /// Delete Rows Locked
  #[sdk(attr(qname = ":deleteRows"))]
  pub delete_rows: Option<crate::simple_type::BooleanValue>,
  /// Select Locked Cells Locked
  #[sdk(attr(qname = ":selectLockedCells"))]
  pub select_locked_cells: Option<crate::simple_type::BooleanValue>,
  /// Sort Locked
  #[sdk(attr(qname = ":sort"))]
  pub sort: Option<crate::simple_type::BooleanValue>,
  /// AutoFilter Locked
  #[sdk(attr(qname = ":autoFilter"))]
  pub auto_filter: Option<crate::simple_type::BooleanValue>,
  /// Pivot Tables Locked
  #[sdk(attr(qname = ":pivotTables"))]
  pub pivot_tables: Option<crate::simple_type::BooleanValue>,
  /// Select Unlocked Cells Locked
  #[sdk(attr(qname = ":selectUnlockedCells"))]
  pub select_unlocked_cells: Option<crate::simple_type::BooleanValue>,
}
/// Custom Sheet Views.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomSheetViews/x:customSheetViews")]
pub struct CustomSheetViews {
  /// Custom Sheet View.
  #[sdk(child(qname = "x:CT_CustomSheetView/x:customSheetView"))]
  pub x_custom_sheet_view: Vec<CustomSheetView>,
}
/// Defines the OleObjects Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleObjects/x:oleObjects")]
pub struct OleObjects {
  /// OLE Object.
  #[sdk(child(qname = "x:CT_OleObject/x:oleObject"))]
  pub x_ole_object: Vec<OleObject>,
}
/// Defines the Controls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Controls/x:controls")]
pub struct Controls {
  /// Embedded Control.
  #[sdk(child(qname = "x:CT_Control/x:control"))]
  pub x_control: Vec<Control>,
}
/// Macro Sheet Dimensions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetDimension/x:dimension")]
pub struct SheetDimension {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Column Information.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cols/x:cols")]
pub struct Columns {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Column Width and Formatting.
  #[sdk(child(qname = "x:CT_Col/x:col"))]
  pub x_col: Vec<Column>,
}
/// Sheet Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetData/x:sheetData")]
pub struct SheetData {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Row.
  #[sdk(child(qname = "x:CT_Row/x:row"))]
  pub x_row: Vec<Row>,
}
/// Data Consolidation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataConsolidate/x:dataConsolidate")]
pub struct DataConsolidate {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Function Index
  #[sdk(attr(qname = ":function"))]
  pub function: Option<DataConsolidateFunctionValues>,
  /// Use Left Column Labels
  #[sdk(attr(qname = ":leftLabels"))]
  pub left_labels: Option<crate::simple_type::BooleanValue>,
  /// startLabels
  #[sdk(attr(office2010, qname = ":startLabels"))]
  pub start_labels: Option<crate::simple_type::BooleanValue>,
  /// Labels In Top Row
  #[sdk(attr(qname = ":topLabels"))]
  pub top_labels: Option<crate::simple_type::BooleanValue>,
  /// Link
  #[sdk(attr(qname = ":link"))]
  pub link: Option<crate::simple_type::BooleanValue>,
  /// Data Consolidation References
  #[sdk(child(qname = "x:CT_DataRefs/x:dataRefs"))]
  pub data_references: Option<DataReferences>,
}
/// Conditional Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormatting/x:conditionalFormatting")]
pub struct ConditionalFormatting {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// PivotTable Conditional Formatting
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// Sequence of References
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// Conditional Formatting Rule.
  #[sdk(child(qname = "x:CT_CfRule/x:cfRule"))]
  pub x_cf_rule: Vec<ConditionalFormattingRule>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomProperties/x:customProperties")]
pub struct CustomProperties {
  /// Custom Property.
  #[sdk(child(qname = "x:CT_CustomProperty/x:customPr"))]
  pub x_custom_pr: Vec<CustomProperty>,
}
/// OLAP Member Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MemberProperties/x:mps")]
pub struct MemberProperties {
  /// OLAP Member Properties Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Member Property.
  #[sdk(child(qname = "x:CT_MemberProperty/x:mp"))]
  pub x_mp: Vec<MemberProperty>,
}
/// Members.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Members/x:members")]
pub struct Members {
  /// Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Hierarchy Level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// Member.
  #[sdk(child(qname = "x:CT_Member/x:member"))]
  pub x_member: Vec<Member>,
}
/// Future Feature Data Storage Area.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchyExtensionList/x:extLst")]
pub struct PivotHierarchyExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PivotHierarchyExtension Class.
  #[sdk(child(qname = "x:CT_PivotHierarchyExtension/x:ext"))]
  pub x_ext: Vec<PivotHierarchyExtension>,
}
/// Field Items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Items/x:items")]
pub struct Items {
  /// Field Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotTable Field Item.
  #[sdk(child(qname = "x:CT_Item/x:item"))]
  pub x_item: Vec<Item>,
}
/// AutoSort Scope.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoSortScope/x:autoSortScope")]
pub struct AutoSortScope {
  /// Auto Sort Scope
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// Future Feature Data Storage Area.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFieldExtensionList/x:extLst")]
pub struct PivotFieldExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PivotFieldExtension Class.
  #[sdk(child(qname = "x:CT_PivotFieldExtension/x:ext"))]
  pub x_ext: Vec<PivotFieldExtension>,
}
/// Defines the WorksheetSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetSource/x:worksheetSource")]
pub struct WorksheetSource {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Defines the Consolidation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Consolidation/x:consolidation")]
pub struct Consolidation {
  /// Auto Page
  #[sdk(attr(qname = ":autoPage"))]
  pub auto_page: Option<crate::simple_type::BooleanValue>,
  /// Page Item Values
  #[sdk(child(qname = "x:CT_Pages/x:pages"))]
  pub pages: Option<Pages>,
  /// Range Sets
  #[sdk(child(qname = "x:CT_RangeSets/x:rangeSets"))]
  pub range_sets: std::boxed::Box<RangeSets>,
}
/// Defines the CacheSourceExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSourceExtensionList/x:extLst")]
pub struct CacheSourceExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CacheSourceExtension Class.
  #[sdk(child(qname = "x:CT_CacheSourceExtension/x:ext"))]
  pub x_ext: Vec<CacheSourceExtension>,
}
/// Defines the CommentProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_CommentPr/x:commentPr")]
pub struct CommentProperties {
  /// locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// textHAlign
  #[sdk(attr(qname = ":textHAlign"))]
  pub text_h_align: Option<TextHorizontalAlignmentValues>,
  /// textVAlign
  #[sdk(attr(qname = ":textVAlign"))]
  pub text_v_align: Option<TextVerticalAlignmentValues>,
  /// lockText
  #[sdk(attr(qname = ":lockText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// justLastX
  #[sdk(attr(qname = ":justLastX"))]
  pub just_last_x: Option<crate::simple_type::BooleanValue>,
  /// autoScale
  #[sdk(attr(qname = ":autoScale"))]
  pub auto_scale: Option<crate::simple_type::BooleanValue>,
  /// rowHidden
  #[sdk(attr(qname = ":rowHidden"))]
  pub row_hidden: Option<crate::simple_type::BooleanValue>,
  /// colHidden
  #[sdk(attr(qname = ":colHidden"))]
  pub col_hidden: Option<crate::simple_type::BooleanValue>,
  /// Defines the ObjectAnchor Class.
  #[sdk(child(office2010, qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
/// Defines the SortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SortCondition/x:sortCondition")]
pub struct SortCondition {
  /// Descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// Sort By
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by: Option<SortByValues>,
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Custom List
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// Format Id
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Icon Set
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<IconSetValues>,
  /// Icon Id
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Filter/x:filter")]
pub struct Filter {
  /// Filter Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Date Grouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DateGroupItem/x:dateGroupItem")]
pub struct DateGroupItem {
  /// Year
  #[sdk(attr(qname = ":year"))]
  pub year: crate::simple_type::UInt16Value,
  /// Month
  #[sdk(attr(qname = ":month"))]
  pub month: Option<crate::simple_type::UInt16Value>,
  /// Day
  #[sdk(attr(qname = ":day"))]
  pub day: Option<crate::simple_type::UInt16Value>,
  /// Hour
  #[sdk(attr(qname = ":hour"))]
  pub hour: Option<crate::simple_type::UInt16Value>,
  /// Minute
  #[sdk(attr(qname = ":minute"))]
  pub minute: Option<crate::simple_type::UInt16Value>,
  /// Second
  #[sdk(attr(qname = ":second"))]
  pub second: Option<crate::simple_type::UInt16Value>,
  /// Date Time Grouping
  #[sdk(attr(qname = ":dateTimeGrouping"))]
  pub date_time_grouping: DateTimeGroupingValues,
}
/// Filter Criteria.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Filters/x:filters")]
pub struct Filters {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Filter by Blank
  #[sdk(attr(qname = ":blank"))]
  pub blank: Option<crate::simple_type::BooleanValue>,
  /// Calendar Type
  #[sdk(attr(qname = ":calendarType"))]
  pub calendar_type: Option<CalendarValues>,
  #[sdk(choice(
    qname = "x14:CT_Filter/x14:filter",
    qname = "x:CT_Filter/x:filter",
    qname = "x:CT_DateGroupItem/x:dateGroupItem"
  ))]
  pub filters_choice: Option<FiltersChoice>,
}
/// Top 10.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Top10/x:top10")]
pub struct Top10 {
  /// Top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::BooleanValue>,
  /// Filter by Percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Top or Bottom Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
  /// Filter Value
  #[sdk(attr(qname = ":filterVal"))]
  pub filter_value: Option<crate::simple_type::DoubleValue>,
}
/// Custom Filters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomFilters/x:customFilters")]
pub struct CustomFilters {
  /// And
  #[sdk(attr(qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  /// Custom Filter Criteria.
  #[sdk(child(qname = "x:CT_CustomFilter/x:customFilter"))]
  pub x_custom_filter: Vec<CustomFilter>,
}
/// Dynamic Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DynamicFilter/x:dynamicFilter")]
pub struct DynamicFilter {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Dynamic filter type
  #[sdk(attr(qname = ":type"))]
  pub r#type: DynamicFilterValues,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Max Value
  #[sdk(attr(qname = ":maxVal"))]
  pub max_val: Option<crate::simple_type::DoubleValue>,
  /// valIso
  #[sdk(attr(office2010, qname = ":valIso"))]
  pub val_iso: Option<crate::simple_type::DateTimeValue>,
  /// maxValIso
  #[sdk(attr(office2010, qname = ":maxValIso"))]
  pub max_val_iso: Option<crate::simple_type::DateTimeValue>,
}
/// Color Filter Criteria.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColorFilter/x:colorFilter")]
pub struct ColorFilter {
  /// Differential Format Record Id
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: crate::simple_type::UInt32Value,
  /// Filter By Cell Color
  #[sdk(attr(qname = ":cellColor"))]
  pub cell_color: Option<crate::simple_type::BooleanValue>,
}
/// Icon Filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IconFilter/x:iconFilter")]
pub struct IconFilter {
  /// Icon Set
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetValues,
  /// Icon Id
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SlicerCacheDefinitionExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SlicerCacheDefinitionExtension/x:ext")]
pub struct SlicerCacheDefinitionExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_SlicerCachePivotTables/x15:slicerCachePivotTables",
    qname = "x15:CT_TableSlicerCache/x15:tableSlicerCache",
    qname = "x15:CT_SlicerCacheHideNoData/x15:slicerCacheHideItemsWithNoData",
    any
  ))]
  pub slicer_cache_definition_extension_choice: Option<SlicerCacheDefinitionExtensionChoice>,
}
/// Defines the PivotFilterExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilterExtension/x:ext")]
pub struct PivotFilterExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x15:CT_PivotFilter/x15:pivotFilter",
    qname = "x15:CT_MovingPeriodState/x15:movingPeriodState",
    any
  ))]
  pub pivot_filter_extension_choice: Option<PivotFilterExtensionChoice>,
}
/// Defines the QueryTableExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableExtension/x:ext")]
pub struct QueryTableExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "x15:CT_QueryTable/x15:queryTable", any))]
  pub query_table_extension_choice: Option<QueryTableExtensionChoice>,
}
/// Defines the DatabaseProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DbPr/x:dbPr")]
pub struct DatabaseProperties {
  /// Connection String
  #[sdk(attr(qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
  /// Command Text
  #[sdk(attr(qname = ":command"))]
  pub command: Option<crate::simple_type::StringValue>,
  /// Command Text
  #[sdk(attr(qname = ":serverCommand"))]
  pub server_command: Option<crate::simple_type::StringValue>,
  /// OLE DB Command Type
  #[sdk(attr(qname = ":commandType"))]
  pub command_type: Option<crate::simple_type::UInt32Value>,
}
/// Defines the OlapProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OlapPr/x:olapPr")]
pub struct OlapProperties {
  /// Local Cube
  #[sdk(attr(qname = ":local"))]
  pub local: Option<crate::simple_type::BooleanValue>,
  /// Local Cube Connection
  #[sdk(attr(qname = ":localConnection"))]
  pub local_connection: Option<crate::simple_type::StringValue>,
  /// Local Refresh
  #[sdk(attr(qname = ":localRefresh"))]
  pub local_refresh: Option<crate::simple_type::BooleanValue>,
  /// Send Locale to OLAP
  #[sdk(attr(qname = ":sendLocale"))]
  pub send_locale: Option<crate::simple_type::BooleanValue>,
  /// Drill Through Count
  #[sdk(attr(qname = ":rowDrillCount"))]
  pub row_drill_count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Fill Formatting
  #[sdk(attr(qname = ":serverFill"))]
  pub server_fill: Option<crate::simple_type::BooleanValue>,
  /// OLAP Number Format
  #[sdk(attr(qname = ":serverNumberFormat"))]
  pub server_number_format: Option<crate::simple_type::BooleanValue>,
  /// OLAP Server Font
  #[sdk(attr(qname = ":serverFont"))]
  pub server_font: Option<crate::simple_type::BooleanValue>,
  /// OLAP Font Formatting
  #[sdk(attr(qname = ":serverFontColor"))]
  pub server_font_color: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebQueryProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPr/x:webPr")]
pub struct WebQueryProperties {
  /// XML Source
  #[sdk(attr(qname = ":xml"))]
  pub xml_source: Option<crate::simple_type::BooleanValue>,
  /// Import XML Source Data
  #[sdk(attr(qname = ":sourceData"))]
  pub source_data: Option<crate::simple_type::BooleanValue>,
  /// Parse PRE
  #[sdk(attr(qname = ":parsePre"))]
  pub parse_pre_tag: Option<crate::simple_type::BooleanValue>,
  /// Consecutive Delimiters
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// Use First Row
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::BooleanValue>,
  /// Created in Excel 97
  #[sdk(attr(qname = ":xl97"))]
  pub created_in_excel97: Option<crate::simple_type::BooleanValue>,
  /// Dates as Text
  #[sdk(attr(qname = ":textDates"))]
  pub text_dates: Option<crate::simple_type::BooleanValue>,
  /// Refreshed in Excel 2000
  #[sdk(attr(qname = ":xl2000"))]
  pub refreshed_in_excel2000: Option<crate::simple_type::BooleanValue>,
  /// URL
  #[sdk(attr(qname = ":url"))]
  pub url: Option<crate::simple_type::StringValue>,
  /// Web Post
  #[sdk(attr(qname = ":post"))]
  pub post: Option<crate::simple_type::StringValue>,
  /// HTML Tables Only
  #[sdk(attr(qname = ":htmlTables"))]
  pub html_tables: Option<crate::simple_type::BooleanValue>,
  /// HTML Formatting Handling
  #[sdk(attr(qname = ":htmlFormat"))]
  pub html_format: Option<HtmlFormattingValues>,
  /// Edit Query URL
  #[sdk(attr(qname = ":editPage"))]
  pub edit_page: Option<crate::simple_type::StringValue>,
  /// Tables
  #[sdk(child(qname = "x:CT_Tables/x:tables"))]
  pub tables: Option<Tables>,
}
/// Defines the TextProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextPr/x:textPr")]
pub struct TextProperties {
  /// prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::BooleanValue>,
  /// fileType
  #[sdk(attr(qname = ":fileType"))]
  pub file_type: Option<FileTypeValues>,
  /// codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub text_character_set: Option<crate::simple_type::StringValue>,
  /// firstRow
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::UInt32Value>,
  /// sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// delimited
  #[sdk(attr(qname = ":delimited"))]
  pub delimited: Option<crate::simple_type::BooleanValue>,
  /// decimal
  #[sdk(attr(qname = ":decimal"))]
  pub decimal: Option<crate::simple_type::StringValue>,
  /// thousands
  #[sdk(attr(qname = ":thousands"))]
  pub thousands: Option<crate::simple_type::StringValue>,
  /// tab
  #[sdk(attr(qname = ":tab"))]
  pub tab_as_delimiter: Option<crate::simple_type::BooleanValue>,
  /// space
  #[sdk(attr(qname = ":space"))]
  pub space: Option<crate::simple_type::BooleanValue>,
  /// comma
  #[sdk(attr(qname = ":comma"))]
  pub comma: Option<crate::simple_type::BooleanValue>,
  /// semicolon
  #[sdk(attr(qname = ":semicolon"))]
  pub semicolon: Option<crate::simple_type::BooleanValue>,
  /// consecutive
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// qualifier
  #[sdk(attr(qname = ":qualifier"))]
  pub qualifier: Option<QualifierValues>,
  /// delimiter
  #[sdk(attr(qname = ":delimiter"))]
  pub delimiter: Option<crate::simple_type::StringValue>,
  /// Defines the TextFields Class.
  #[sdk(child(qname = "x:CT_TextFields/x:textFields"))]
  pub text_fields: Option<TextFields>,
}
/// Defines the Parameters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Parameters/x:parameters")]
pub struct Parameters {
  /// Parameter Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Parameter Properties.
  #[sdk(child(qname = "x:CT_Parameter/x:parameter"))]
  pub x_parameter: Vec<Parameter>,
}
/// Defines the ConnectionExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConnectionExtensionList/x:extLst")]
pub struct ConnectionExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ConnectionExtension Class.
  #[sdk(child(qname = "x:CT_ConnectionExtension/x:ext"))]
  pub x_ext: Vec<ConnectionExtension>,
}
/// Defines the ConnectionExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConnectionExtension/x:ext")]
pub struct ConnectionExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_Connection/x14:connection",
    qname = "x15:CT_Connection/x15:connection",
    any
  ))]
  pub connection_extension_choice: Option<ConnectionExtensionChoice>,
}
/// Defines the TextFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextFields/x:textFields")]
pub struct TextFields {
  /// Count of Fields
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Text Import Field Settings.
  #[sdk(child(qname = "x:CT_TextField/x:textField"))]
  pub x_text_field: Vec<TextField>,
}
/// Defines the SharedItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SharedItems/x:sharedItems")]
pub struct SharedItems {
  /// Contains Semi Mixed Data Types
  #[sdk(attr(qname = ":containsSemiMixedTypes"))]
  pub contains_semi_mixed_types: Option<crate::simple_type::BooleanValue>,
  /// Contains Non Date
  #[sdk(attr(qname = ":containsNonDate"))]
  pub contains_non_date: Option<crate::simple_type::BooleanValue>,
  /// Contains Date
  #[sdk(attr(qname = ":containsDate"))]
  pub contains_date: Option<crate::simple_type::BooleanValue>,
  /// Contains String
  #[sdk(attr(qname = ":containsString"))]
  pub contains_string: Option<crate::simple_type::BooleanValue>,
  /// Contains Blank
  #[sdk(attr(qname = ":containsBlank"))]
  pub contains_blank: Option<crate::simple_type::BooleanValue>,
  /// Contains Mixed Data Types
  #[sdk(attr(qname = ":containsMixedTypes"))]
  pub contains_mixed_types: Option<crate::simple_type::BooleanValue>,
  /// Contains Numbers
  #[sdk(attr(qname = ":containsNumber"))]
  pub contains_number: Option<crate::simple_type::BooleanValue>,
  /// Contains Integer
  #[sdk(attr(qname = ":containsInteger"))]
  pub contains_integer: Option<crate::simple_type::BooleanValue>,
  /// Minimum Numeric Value
  #[sdk(attr(qname = ":minValue"))]
  pub min_value: Option<crate::simple_type::DoubleValue>,
  /// Maximum Numeric Value
  #[sdk(attr(qname = ":maxValue"))]
  pub max_value: Option<crate::simple_type::DoubleValue>,
  /// Minimum Date Time
  #[sdk(attr(qname = ":minDate"))]
  pub min_date: Option<crate::simple_type::DateTimeValue>,
  /// Maximum Date Time Value
  #[sdk(attr(qname = ":maxDate"))]
  pub max_date: Option<crate::simple_type::DateTimeValue>,
  /// Shared Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Long Text
  #[sdk(attr(qname = ":longText"))]
  pub long_text: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "x:CT_Missing/x:m",
    qname = "x:CT_Number/x:n",
    qname = "x:CT_Boolean/x:b",
    qname = "x:CT_Error/x:e",
    qname = "x:CT_String/x:s",
    qname = "x:CT_DateTime/x:d"
  ))]
  pub shared_items_choice: Vec<SharedItemsChoice>,
}
/// Defines the FieldGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldGroup/x:fieldGroup")]
pub struct FieldGroup {
  /// Parent
  #[sdk(attr(qname = ":par"))]
  pub parent_id: Option<crate::simple_type::UInt32Value>,
  /// Field Base
  #[sdk(attr(qname = ":base"))]
  pub base: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_RangePr/x:rangePr",
    qname = "x:CT_DiscretePr/x:discretePr"
  ))]
  pub field_group_choice: Option<FieldGroupChoice>,
  /// OLAP Group Items.
  #[sdk(child(qname = "x:CT_GroupItems/x:groupItems"))]
  pub x_group_items: Option<GroupItems>,
}
/// Defines the CacheFieldExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFieldExtensionList/x:extLst")]
pub struct CacheFieldExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CacheFieldExtension Class.
  #[sdk(child(qname = "x:CT_CacheFieldExtension/x:ext"))]
  pub x_ext: Vec<CacheFieldExtension>,
}
/// Defines the CacheFieldExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFieldExtension/x:ext")]
pub struct CacheFieldExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_CacheField/x14:cacheField",
    qname = "x15:CT_CachedUniqueNames/x15:cachedUniqueNames",
    any
  ))]
  pub cache_field_extension_choice: Option<CacheFieldExtensionChoice>,
}
/// Defines the FieldsUsage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldsUsage/x:fieldsUsage")]
pub struct FieldsUsage {
  /// Field Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotCache Field Id.
  #[sdk(child(qname = "x:CT_FieldUsage/x:fieldUsage"))]
  pub x_field_usage: Vec<FieldUsage>,
}
/// Defines the GroupLevels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupLevels/x:groupLevels")]
pub struct GroupLevels {
  /// Grouping Level Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Grouping Levels.
  #[sdk(child(qname = "x:CT_GroupLevel/x:groupLevel"))]
  pub x_group_level: Vec<GroupLevel>,
}
/// Defines the CacheHierarchyExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchyExtensionList/x:extLst")]
pub struct CacheHierarchyExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CacheHierarchyExtension Class.
  #[sdk(child(qname = "x:CT_CacheHierarchyExtension/x:ext"))]
  pub x_ext: Vec<CacheHierarchyExtension>,
}
/// Defines the CacheHierarchyExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchyExtension/x:ext")]
pub struct CacheHierarchyExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_CacheHierarchy/x14:cacheHierarchy",
    qname = "x15:CT_CacheHierarchy/x15:cacheHierarchy",
    any
  ))]
  pub cache_hierarchy_extension_choice: Option<CacheHierarchyExtensionChoice>,
}
/// Defines the CalculatedMemberExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMemberExtensionList/x:extLst")]
pub struct CalculatedMemberExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CalculatedMemberExtension Class.
  #[sdk(child(qname = "x:CT_CalculatedMemberExtension/x:ext"))]
  pub x_ext: Vec<CalculatedMemberExtension>,
}
/// Defines the CalculatedMemberExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMemberExtension/x:ext")]
pub struct CalculatedMemberExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_CalculatedMember/x14:calculatedMember",
    qname = "x15:CT_CalculatedMember/x15:calculatedMember",
    any
  ))]
  pub calculated_member_extension_choice: Option<CalculatedMemberExtensionChoice>,
}
/// Defines the DataFieldExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFieldExtensionList/x:extLst")]
pub struct DataFieldExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the DataFieldExtension Class.
  #[sdk(child(qname = "x:CT_DataFieldExtension/x:ext"))]
  pub x_ext: Vec<DataFieldExtension>,
}
/// Defines the DataFieldExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFieldExtension/x:ext")]
pub struct DataFieldExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_DataField/x14:dataField",
    qname = "x15:CT_DataField/x15:dataField",
    any
  ))]
  pub data_field_extension_choice: Option<DataFieldExtensionChoice>,
}
/// Defines the PivotFilterExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilterExtensionList/x:extLst")]
pub struct PivotFilterExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PivotFilterExtension Class.
  #[sdk(child(qname = "x:CT_PivotFilterExtension/x:ext"))]
  pub x_ext: Vec<PivotFilterExtension>,
}
/// Defines the QueryTableRefresh Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableRefresh/x:queryTableRefresh")]
pub struct QueryTableRefresh {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Preserve Sort and Filter Layout
  #[sdk(attr(qname = ":preserveSortFilterLayout"))]
  pub preserve_sort_filter_layout: Option<crate::simple_type::BooleanValue>,
  /// Next Field Id Wrapped
  #[sdk(attr(qname = ":fieldIdWrapped"))]
  pub field_id_wrapped: Option<crate::simple_type::BooleanValue>,
  /// Headers In Last Refresh
  #[sdk(attr(qname = ":headersInLastRefresh"))]
  pub headers_in_last_refresh: Option<crate::simple_type::BooleanValue>,
  /// Minimum Refresh Version
  #[sdk(attr(qname = ":minimumVersion"))]
  pub minimum_version: Option<crate::simple_type::ByteValue>,
  /// Next field id
  #[sdk(attr(qname = ":nextId"))]
  pub next_id: Option<crate::simple_type::UInt32Value>,
  /// Columns Left
  #[sdk(attr(qname = ":unboundColumnsLeft"))]
  pub unbound_columns_left: Option<crate::simple_type::UInt32Value>,
  /// Columns Right
  #[sdk(attr(qname = ":unboundColumnsRight"))]
  pub unbound_columns_right: Option<crate::simple_type::UInt32Value>,
  /// Query table fields
  #[sdk(child(qname = "x:CT_QueryTableFields/x:queryTableFields"))]
  pub query_table_fields: std::boxed::Box<QueryTableFields>,
  /// Deleted Fields
  #[sdk(child(qname = "x:CT_QueryTableDeletedFields/x:queryTableDeletedFields"))]
  pub query_table_deleted_fields: Option<QueryTableDeletedFields>,
  /// Sort State
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub sort_state: Option<std::boxed::Box<SortState>>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QueryTableExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableExtensionList/x:extLst")]
pub struct QueryTableExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the QueryTableExtension Class.
  #[sdk(child(qname = "x:CT_QueryTableExtension/x:ext"))]
  pub x_ext: Vec<QueryTableExtension>,
}
/// Defines the SheetCalculationProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetCalcPr/x:sheetCalcPr")]
pub struct SheetCalculationProperties {
  /// Full Calculation On Load
  #[sdk(attr(qname = ":fullCalcOnLoad"))]
  pub full_calculation_on_load: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ProtectedRanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ProtectedRanges/x:protectedRanges")]
pub struct ProtectedRanges {
  /// Protected Range.
  #[sdk(child(qname = "x:CT_ProtectedRange/x:protectedRange"))]
  pub x_protected_range: Vec<ProtectedRange>,
}
/// Defines the Scenarios Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Scenarios/x:scenarios")]
pub struct Scenarios {
  /// Current Scenario
  #[sdk(attr(qname = ":current"))]
  pub current: Option<crate::simple_type::UInt32Value>,
  /// Last Shown Scenario
  #[sdk(attr(qname = ":show"))]
  pub show: Option<crate::simple_type::UInt32Value>,
  /// Sequence of References
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// Scenario.
  #[sdk(child(qname = "x:CT_Scenario/x:scenario"))]
  pub x_scenario: Vec<Scenario>,
}
/// Defines the MergeCells Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MergeCells/x:mergeCells")]
pub struct MergeCells {
  /// Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Merged Cell.
  #[sdk(child(qname = "x:CT_MergeCell/x:mergeCell"))]
  pub x_merge_cell: Vec<MergeCell>,
}
/// Defines the DataValidations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataValidations/x:dataValidations")]
pub struct DataValidations {
  /// Disable Prompts
  #[sdk(attr(qname = ":disablePrompts"))]
  pub disable_prompts: Option<crate::simple_type::BooleanValue>,
  /// Top Left Corner (X Coodrinate)
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::UInt32Value>,
  /// Top Left Corner (Y Coordinate)
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::UInt32Value>,
  /// Data Validation Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Data Validation.
  #[sdk(child(qname = "x:CT_DataValidation/x:dataValidation"))]
  pub x_data_validation: Vec<DataValidation>,
}
/// Defines the Hyperlinks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Hyperlinks/x:hyperlinks")]
pub struct Hyperlinks {
  /// Hyperlink.
  #[sdk(child(qname = "x:CT_Hyperlink/x:hyperlink"))]
  pub x_hyperlink: Vec<Hyperlink>,
}
/// Defines the CellWatches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellWatches/x:cellWatches")]
pub struct CellWatches {
  /// Cell Watch Item.
  #[sdk(child(qname = "x:CT_CellWatch/x:cellWatch"))]
  pub x_cell_watch: Vec<CellWatch>,
}
/// Defines the IgnoredErrors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IgnoredErrors/x:ignoredErrors")]
pub struct IgnoredErrors {
  /// Ignored Error.
  #[sdk(child(qname = "x:CT_IgnoredError/x:ignoredError"))]
  pub x_ignored_error: Vec<IgnoredError>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Defines the TableParts Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableParts/x:tableParts")]
pub struct TableParts {
  /// Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Table Part.
  #[sdk(child(qname = "x:CT_TablePart/x:tablePart"))]
  pub x_table_part: Vec<TablePart>,
}
/// Defines the WorksheetExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetExtensionList/x:extLst")]
pub struct WorksheetExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WorksheetExtension Class.
  #[sdk(child(qname = "x:CT_WorksheetExtension/x:ext"))]
  pub x_ext: Vec<WorksheetExtension>,
}
/// Defines the WorksheetExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetExtension/x:ext")]
pub struct WorksheetExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_ConditionalFormattings/x14:conditionalFormattings",
    qname = "x14:CT_DataValidations/x14:dataValidations",
    qname = "x14:CT_SparklineGroups/x14:sparklineGroups",
    qname = "x14:CT_SlicerRefs/x14:slicerList",
    qname = "x14:CT_ProtectedRanges/x14:protectedRanges",
    qname = "x14:CT_IgnoredErrors/x14:ignoredErrors",
    qname = "x15:CT_WebExtensions/x15:webExtensions",
    qname = "x15:CT_TimelineRefs/x15:timelineRefs",
    any
  ))]
  pub worksheet_extension_choice: Option<WorksheetExtensionChoice>,
}
/// Defines the NumberingFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NumFmts/x:numFmts")]
pub struct NumberingFormats {
  /// Number Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Number Formats.
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub x_num_fmt: Vec<NumberingFormat>,
}
/// Defines the Fonts Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fonts/x:fonts")]
pub struct Fonts {
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Font Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// knownFonts
  #[sdk(attr(office2010, qname = "x14ac:knownFonts"))]
  pub known_fonts: Option<crate::simple_type::BooleanValue>,
  /// Font Properties.
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub x_font: Vec<Font>,
}
/// Defines the Fills Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fills/x:fills")]
pub struct Fills {
  /// Fill Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Fill.
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub x_fill: Vec<Fill>,
}
/// Defines the Borders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Borders/x:borders")]
pub struct Borders {
  /// Border Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Border Properties.
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub x_border: Vec<Border>,
}
/// Defines the CellStyleFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyleXfs/x:cellStyleXfs")]
pub struct CellStyleFormats {
  /// Style Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting Elements.
  #[sdk(child(qname = "x:CT_Xf/x:xf"))]
  pub x_xf: Vec<CellFormat>,
}
/// Defines the CellFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellXfs/x:cellXfs")]
pub struct CellFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting Elements.
  #[sdk(child(qname = "x:CT_Xf/x:xf"))]
  pub x_xf: Vec<CellFormat>,
}
/// Defines the CellStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyles/x:cellStyles")]
pub struct CellStyles {
  /// Style Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Cell Style.
  #[sdk(child(qname = "x:CT_CellStyle/x:cellStyle"))]
  pub x_cell_style: Vec<CellStyle>,
}
/// Defines the DifferentialFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxfs/x:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting.
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf: Vec<DifferentialFormat>,
}
/// Defines the TableStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyles/x:tableStyles")]
pub struct TableStyles {
  /// Table Style Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Default Table Style
  #[sdk(attr(qname = ":defaultTableStyle"))]
  pub default_table_style: Option<crate::simple_type::StringValue>,
  /// Default Pivot Style
  #[sdk(attr(qname = ":defaultPivotStyle"))]
  pub default_pivot_style: Option<crate::simple_type::StringValue>,
  /// Table Style.
  #[sdk(child(qname = "x:CT_TableStyle/x:tableStyle"))]
  pub x_table_style: Vec<TableStyle>,
}
/// Defines the Colors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Colors/x:colors")]
pub struct Colors {
  /// Color Indexes
  #[sdk(child(qname = "x:CT_IndexedColors/x:indexedColors"))]
  pub indexed_colors: Option<IndexedColors>,
  /// MRU Colors
  #[sdk(child(qname = "x:CT_MRUColors/x:mruColors"))]
  pub mru_colors: Option<MruColors>,
}
/// Defines the StylesheetExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_StylesheetExtensionList/x:extLst")]
pub struct StylesheetExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the StylesheetExtension Class.
  #[sdk(child(qname = "x:CT_StylesheetExtension/x:ext"))]
  pub x_ext: Vec<StylesheetExtension>,
}
/// Defines the StylesheetExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_StylesheetExtension/x:ext")]
pub struct StylesheetExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x:CT_Dxfs/x14:dxfs",
    qname = "x14:CT_SlicerStyles/x14:slicerStyles",
    qname = "x:CT_Dxfs/x15:dxfs",
    qname = "x15:CT_TimelineStyles/x15:timelineStyles",
    any
  ))]
  pub stylesheet_extension_choice: Option<StylesheetExtensionChoice>,
}
/// Defines the Location Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Location/x:location")]
pub struct Location {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// First Header Row
  #[sdk(attr(qname = ":firstHeaderRow"))]
  pub first_header_row: crate::simple_type::UInt32Value,
  /// PivotTable Data First Row
  #[sdk(attr(qname = ":firstDataRow"))]
  pub first_data_row: crate::simple_type::UInt32Value,
  /// First Data Column
  #[sdk(attr(qname = ":firstDataCol"))]
  pub first_data_column: crate::simple_type::UInt32Value,
  /// Rows Per Page Count
  #[sdk(attr(qname = ":rowPageCount"))]
  pub row_page_count: Option<crate::simple_type::UInt32Value>,
  /// Columns Per Page
  #[sdk(attr(qname = ":colPageCount"))]
  pub columns_per_page: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PivotFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFields/x:pivotFields")]
pub struct PivotFields {
  /// Field Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotTable Field.
  #[sdk(child(qname = "x:CT_PivotField/x:pivotField"))]
  pub x_pivot_field: Vec<PivotField>,
}
/// Defines the RowFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RowFields/x:rowFields")]
pub struct RowFields {
  /// Repeated Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row Items.
  #[sdk(child(qname = "x:CT_Field/x:field"))]
  pub x_field: Vec<Field>,
}
/// Defines the RowItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_rowItems/x:rowItems")]
pub struct RowItems {
  /// Items in a Row Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row Items.
  #[sdk(child(qname = "x:CT_I/x:i"))]
  pub x_i: Vec<RowItem>,
}
/// Defines the ColumnFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColFields/x:colFields")]
pub struct ColumnFields {
  /// Repeated Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row Items.
  #[sdk(child(qname = "x:CT_Field/x:field"))]
  pub x_field: Vec<Field>,
}
/// Defines the ColumnItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_colItems/x:colItems")]
pub struct ColumnItems {
  /// Column Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row Items.
  #[sdk(child(qname = "x:CT_I/x:i"))]
  pub x_i: Vec<RowItem>,
}
/// Defines the PageFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageFields/x:pageFields")]
pub struct PageFields {
  /// Page Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Page Field.
  #[sdk(child(qname = "x:CT_PageField/x:pageField"))]
  pub x_page_field: Vec<PageField>,
}
/// Defines the DataFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFields/x:dataFields")]
pub struct DataFields {
  /// Data Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Data Field Item.
  #[sdk(child(qname = "x:CT_DataField/x:dataField"))]
  pub x_data_field: Vec<DataField>,
}
/// Defines the Formats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Formats/x:formats")]
pub struct Formats {
  /// Formats Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotTable Format.
  #[sdk(child(qname = "x:CT_Format/x:format"))]
  pub x_format: Vec<Format>,
}
/// Defines the ConditionalFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormats/x:conditionalFormats")]
pub struct ConditionalFormats {
  /// Conditional Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Conditional Formatting.
  #[sdk(child(qname = "x:CT_ConditionalFormat/x:conditionalFormat"))]
  pub x_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the ChartFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartFormats/x:chartFormats")]
pub struct ChartFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotChart Format.
  #[sdk(child(qname = "x:CT_ChartFormat/x:chartFormat"))]
  pub x_chart_format: Vec<ChartFormat>,
}
/// Defines the PivotHierarchies Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchies/x:pivotHierarchies")]
pub struct PivotHierarchies {
  /// OLAP Hierarchy Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Hierarchy.
  #[sdk(child(qname = "x:CT_PivotHierarchy/x:pivotHierarchy"))]
  pub x_pivot_hierarchy: Vec<PivotHierarchy>,
}
/// Defines the PivotTableStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotTableStyle/x:pivotTableStyleInfo")]
pub struct PivotTableStyle {
  /// Table Style Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show Row Header Formatting
  #[sdk(attr(qname = ":showRowHeaders"))]
  pub show_row_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Table Style Column Header Formatting
  #[sdk(attr(qname = ":showColHeaders"))]
  pub show_column_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Row Stripes
  #[sdk(attr(qname = ":showRowStripes"))]
  pub show_row_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Column Stripes
  #[sdk(attr(qname = ":showColStripes"))]
  pub show_column_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Last Column
  #[sdk(attr(qname = ":showLastColumn"))]
  pub show_last_column: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotFilters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilters/x:filters")]
pub struct PivotFilters {
  /// Pivot Filter Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotTable Advanced Filter.
  #[sdk(child(qname = "x:CT_PivotFilter/x:filter"))]
  pub x_filter: Vec<PivotFilter>,
}
/// Defines the RowHierarchiesUsage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage")]
pub struct RowHierarchiesUsage {
  /// Item Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row OLAP Hierarchies.
  #[sdk(child(qname = "x:CT_HierarchyUsage/x:rowHierarchyUsage"))]
  pub x_row_hierarchy_usage: Vec<RowHierarchyUsage>,
}
/// Defines the ColumnHierarchiesUsage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage")]
pub struct ColumnHierarchiesUsage {
  /// Items Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Column OLAP Hierarchies.
  #[sdk(child(qname = "x:CT_HierarchyUsage/x:colHierarchyUsage"))]
  pub x_col_hierarchy_usage: Vec<ColumnHierarchyUsage>,
}
/// Defines the PivotTableDefinitionExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinitionExtensionList/x:extLst")]
pub struct PivotTableDefinitionExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PivotTableDefinitionExtension Class.
  #[sdk(child(qname = "x:CT_pivotTableDefinitionExtension/x:ext"))]
  pub x_ext: Vec<PivotTableDefinitionExtension>,
}
/// Defines the PivotTableDefinitionExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinitionExtension/x:ext")]
pub struct PivotTableDefinitionExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_PivotTableDefinition/x14:pivotTableDefinition",
    qname = "x15:CT_PivotTableData/x15:pivotTableData",
    qname = "x15:CT_PivotTableUISettings/x15:pivotTableUISettings",
    qname = "xxpvi:CT_PivotVersionInfo/xxpvi:pivotVersionInfo",
    any
  ))]
  pub pivot_table_definition_extension_choice: Option<PivotTableDefinitionExtensionChoice>,
}
/// Defines the CacheSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSource/x:cacheSource")]
pub struct CacheSource {
  pub xml_other_attrs: Vec<(String, String)>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: SourceValues,
  /// connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "x:CT_WorksheetSource/x:worksheetSource",
    qname = "x:CT_Consolidation/x:consolidation",
    qname = "x:CT_CacheSourceExtensionList/x:extLst"
  ))]
  pub cache_source_choice: Option<CacheSourceChoice>,
}
/// Defines the CacheFields Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFields/x:cacheFields")]
pub struct CacheFields {
  /// Field Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotCache Field.
  #[sdk(child(qname = "x:CT_CacheField/x:cacheField"))]
  pub x_cache_field: Vec<CacheField>,
}
/// Defines the CacheHierarchies Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchies/x:cacheHierarchies")]
pub struct CacheHierarchies {
  /// Hierarchy Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// PivotCache Hierarchy.
  #[sdk(child(qname = "x:CT_CacheHierarchy/x:cacheHierarchy"))]
  pub x_cache_hierarchy: Vec<CacheHierarchy>,
}
/// Defines the Kpis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDKPIs/x:kpis")]
pub struct Kpis {
  /// KPI Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP KPI.
  #[sdk(child(qname = "x:CT_PCDKPI/x:kpi"))]
  pub x_kpi: Vec<Kpi>,
}
/// Defines the TupleCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TupleCache/x:tupleCache")]
pub struct TupleCache {
  /// Entries
  #[sdk(child(qname = "x:CT_PCDSDTCEntries/x:entries"))]
  pub entries: Option<Entries>,
  /// Sets
  #[sdk(child(qname = "x:CT_Sets/x:sets"))]
  pub sets: Option<Sets>,
  /// OLAP Query Cache
  #[sdk(child(qname = "x:CT_QueryCache/x:queryCache"))]
  pub query_cache: Option<QueryCache>,
  /// Server Formats
  #[sdk(child(qname = "x:CT_ServerFormats/x:serverFormats"))]
  pub server_formats: Option<ServerFormats>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CalculatedItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedItems/x:calculatedItems")]
pub struct CalculatedItems {
  /// Calculated Item Formula Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Calculated Item.
  #[sdk(child(qname = "x:CT_CalculatedItem/x:calculatedItem"))]
  pub x_calculated_item: Vec<CalculatedItem>,
}
/// Defines the CalculatedMembers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMembers/x:calculatedMembers")]
pub struct CalculatedMembers {
  /// Calculated Members Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Calculated Member.
  #[sdk(child(qname = "x:CT_CalculatedMember/x:calculatedMember"))]
  pub x_calculated_member: Vec<CalculatedMember>,
}
/// Defines the Dimensions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dimensions/x:dimensions")]
pub struct Dimensions {
  /// OLAP Dimensions Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Dimension.
  #[sdk(child(qname = "x:CT_PivotDimension/x:dimension"))]
  pub x_dimension: Vec<Dimension>,
}
/// Defines the MeasureGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureGroups/x:measureGroups")]
pub struct MeasureGroups {
  /// Measure Group Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Measure Group.
  #[sdk(child(qname = "x:CT_MeasureGroup/x:measureGroup"))]
  pub x_measure_group: Vec<MeasureGroup>,
}
/// Defines the Maps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureDimensionMaps/x:maps")]
pub struct Maps {
  /// Measure Group Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Measure Group.
  #[sdk(child(qname = "x:CT_MeasureDimensionMap/x:map"))]
  pub x_map: Vec<MeasureDimensionMap>,
}
/// Defines the PivotCacheDefinitionExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinitionExtensionList/x:extLst")]
pub struct PivotCacheDefinitionExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the PivotCacheDefinitionExtension Class.
  #[sdk(child(qname = "x:CT_PivotCacheDefinitionExtension/x:ext"))]
  pub x_ext: Vec<PivotCacheDefinitionExtension>,
}
/// Defines the PivotCacheDefinitionExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinitionExtension/x:ext")]
pub struct PivotCacheDefinitionExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_PivotCacheDefinition/x14:pivotCacheDefinition",
    qname = "x15:CT_PivotCacheDecoupled/x15:pivotCacheDecoupled",
    qname = "x15:CT_TimelinePivotCacheDefinition/x15:timelinePivotCacheDefinition",
    qname = "x15:CT_PivotCacheIdVersion/x15:pivotCacheIdVersion",
    qname = "xsd:boolean/xxpim:implicitMeasureSupport",
    qname = "xprd:CT_PivotCacheRichInfo/xprd:richInfo",
    qname = "xxpvi:CT_CacheVersionInfo/xxpvi:cacheVersionInfo",
    qname = "xsd:boolean/xlpar:autoRefresh",
    qname = "xlpda:CT_PivotCacheDynamicArray/xlpda:pivotCacheDynamicArray",
    any
  ))]
  pub pivot_cache_definition_extension_choice: Option<PivotCacheDefinitionExtensionChoice>,
}
/// Sheet names of supporting book.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetNames/x:sheetNames")]
pub struct SheetNames {
  /// Sheet Name.
  #[sdk(child(qname = "x:CT_ExternalSheetName/x:sheetName"))]
  pub x_sheet_name: Vec<SheetName>,
}
/// Defined names associated with supporting book..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalDefinedNames/x:definedNames")]
pub struct ExternalDefinedNames {
  /// Defined Name.
  #[sdk(child(qname = "x:CT_ExternalDefinedName/x:definedName"))]
  pub x_defined_name: Vec<ExternalDefinedName>,
}
/// Cached worksheet data associated with supporting book.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetDataSet/x:sheetDataSet")]
pub struct SheetDataSet {
  /// External Sheet Data Set.
  #[sdk(child(qname = "x:CT_ExternalSheetData/x:sheetData"))]
  pub x_sheet_data: Vec<ExternalSheetData>,
}
/// Table Columns.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableColumns/x:tableColumns")]
pub struct TableColumns {
  /// Column Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Table Column.
  #[sdk(child(qname = "x:CT_TableColumn/x:tableColumn"))]
  pub x_table_column: Vec<TableColumn>,
}
/// Table Style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyleInfo/x:tableStyleInfo")]
pub struct TableStyleInfo {
  /// Style Name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show First Column
  #[sdk(attr(qname = ":showFirstColumn"))]
  pub show_first_column: Option<crate::simple_type::BooleanValue>,
  /// Show Last Column
  #[sdk(attr(qname = ":showLastColumn"))]
  pub show_last_column: Option<crate::simple_type::BooleanValue>,
  /// Show Row Stripes
  #[sdk(attr(qname = ":showRowStripes"))]
  pub show_row_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Column Stripes
  #[sdk(attr(qname = ":showColumnStripes"))]
  pub show_column_stripes: Option<crate::simple_type::BooleanValue>,
}
/// Future Feature Data Storage Area.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableExtensionList/x:extLst")]
pub struct TableExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the TableExtension Class.
  #[sdk(child(qname = "x:CT_TableExtension/x:ext"))]
  pub x_ext: Vec<TableExtension>,
}
/// Defines the TableExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableExtension/x:ext")]
pub struct TableExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_Table/x14:table",
    qname = "xlmsforms:CT_MsForm/xlmsforms:msForm",
    any
  ))]
  pub table_extension_choice: Option<TableExtensionChoice>,
}
/// Defines the FileVersion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileVersion/x:fileVersion")]
pub struct FileVersion {
  /// Application Name
  #[sdk(attr(qname = ":appName"))]
  pub application_name: Option<crate::simple_type::StringValue>,
  /// Last Edited Version
  #[sdk(attr(qname = ":lastEdited"))]
  pub last_edited: Option<crate::simple_type::StringValue>,
  /// Lowest Edited Version
  #[sdk(attr(qname = ":lowestEdited"))]
  pub lowest_edited: Option<crate::simple_type::StringValue>,
  /// Build Version
  #[sdk(attr(qname = ":rupBuild"))]
  pub build_version: Option<crate::simple_type::StringValue>,
  /// Code Name
  #[sdk(attr(qname = ":codeName"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub code_name: Option<crate::simple_type::StringValue>,
}
/// Defines the FileSharing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileSharing/x:fileSharing")]
pub struct FileSharing {
  /// Read Only Recommended
  #[sdk(attr(qname = ":readOnlyRecommended"))]
  pub read_only_recommended: Option<crate::simple_type::BooleanValue>,
  /// User Name
  #[sdk(attr(qname = ":userName"))]
  pub user_name: Option<crate::simple_type::StringValue>,
  /// Write Reservation Password
  #[sdk(attr(qname = ":reservationPassword"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub reservation_password: Option<crate::simple_type::HexBinaryValue>,
  /// Password hash algorithm
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password hash
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt for password hash
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Spin count for password hash
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
}
/// Defines the WorkbookProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookPr/x:workbookPr")]
pub struct WorkbookProperties {
  pub xml_other_attrs: Vec<(String, String)>,
  /// Date 1904
  #[sdk(attr(qname = ":date1904"))]
  pub date1904: Option<crate::simple_type::BooleanValue>,
  /// dateCompatibility
  #[sdk(attr(office2010, qname = ":dateCompatibility"))]
  pub date_compatibility: Option<crate::simple_type::BooleanValue>,
  /// Show Objects
  #[sdk(attr(qname = ":showObjects"))]
  pub show_objects: Option<ObjectDisplayValues>,
  /// Show Border Unselected Table
  #[sdk(attr(qname = ":showBorderUnselectedTables"))]
  pub show_border_unselected_tables: Option<crate::simple_type::BooleanValue>,
  /// Filter Privacy
  #[sdk(attr(qname = ":filterPrivacy"))]
  pub filter_privacy: Option<crate::simple_type::BooleanValue>,
  /// Prompted Solutions
  #[sdk(attr(qname = ":promptedSolutions"))]
  pub prompted_solutions: Option<crate::simple_type::BooleanValue>,
  /// Show Ink Annotations
  #[sdk(attr(qname = ":showInkAnnotation"))]
  pub show_ink_annotation: Option<crate::simple_type::BooleanValue>,
  /// Create Backup File
  #[sdk(attr(qname = ":backupFile"))]
  pub backup_file: Option<crate::simple_type::BooleanValue>,
  /// Save External Link Values
  #[sdk(attr(qname = ":saveExternalLinkValues"))]
  pub save_external_link_values: Option<crate::simple_type::BooleanValue>,
  /// Update Links Behavior
  #[sdk(attr(qname = ":updateLinks"))]
  pub update_links: Option<UpdateLinksBehaviorValues>,
  /// Code Name
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// Hide Pivot Field List
  #[sdk(attr(qname = ":hidePivotFieldList"))]
  pub hide_pivot_field_list: Option<crate::simple_type::BooleanValue>,
  /// Show Pivot Chart Filter
  #[sdk(attr(qname = ":showPivotChartFilter"))]
  pub show_pivot_chart_filter: Option<crate::simple_type::BooleanValue>,
  /// Allow Refresh Query
  #[sdk(attr(qname = ":allowRefreshQuery"))]
  pub allow_refresh_query: Option<crate::simple_type::BooleanValue>,
  /// Publish Items
  #[sdk(attr(qname = ":publishItems"))]
  pub publish_items: Option<crate::simple_type::BooleanValue>,
  /// Check Compatibility On Save
  #[sdk(attr(qname = ":checkCompatibility"))]
  pub check_compatibility: Option<crate::simple_type::BooleanValue>,
  /// Auto Compress Pictures
  #[sdk(attr(qname = ":autoCompressPictures"))]
  pub auto_compress_pictures: Option<crate::simple_type::BooleanValue>,
  /// Refresh all Connections on Open
  #[sdk(attr(qname = ":refreshAllConnections"))]
  pub refresh_all_connections: Option<crate::simple_type::BooleanValue>,
  /// Default Theme Version
  #[sdk(attr(qname = ":defaultThemeVersion"))]
  pub default_theme_version: Option<crate::simple_type::UInt32Value>,
}
/// Defines the WorkbookProtection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookProtection/x:workbookProtection")]
pub struct WorkbookProtection {
  /// Workbook Password
  #[sdk(attr(qname = ":workbookPassword"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub workbook_password: Option<crate::simple_type::HexBinaryValue>,
  /// Revisions Password
  #[sdk(attr(qname = ":revisionsPassword"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub revisions_password: Option<crate::simple_type::HexBinaryValue>,
  /// Lock Structure
  #[sdk(attr(qname = ":lockStructure"))]
  pub lock_structure: Option<crate::simple_type::BooleanValue>,
  /// Lock Windows
  #[sdk(attr(qname = ":lockWindows"))]
  pub lock_windows: Option<crate::simple_type::BooleanValue>,
  /// Lock Revisions
  #[sdk(attr(qname = ":lockRevision"))]
  pub lock_revision: Option<crate::simple_type::BooleanValue>,
  /// Cryptographic Algorithm Name
  #[sdk(attr(qname = ":revisionsAlgorithmName"))]
  pub revisions_algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  #[sdk(attr(qname = ":revisionsHashValue"))]
  pub revisions_hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  #[sdk(attr(qname = ":revisionsSaltValue"))]
  pub revisions_salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = ":revisionsSpinCount"))]
  pub revisions_spin_count: Option<crate::simple_type::UInt32Value>,
  /// Cryptographic Algorithm Name
  #[sdk(attr(qname = ":workbookAlgorithmName"))]
  pub workbook_algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  #[sdk(attr(qname = ":workbookHashValue"))]
  pub workbook_hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  #[sdk(attr(qname = ":workbookSaltValue"))]
  pub workbook_salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  #[sdk(attr(qname = ":workbookSpinCount"))]
  pub workbook_spin_count: Option<crate::simple_type::UInt32Value>,
}
/// Defines the BookViews Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BookViews/x:bookViews")]
pub struct BookViews {
  /// Workbook View.
  #[sdk(child(qname = "x:CT_BookView/x:workbookView"))]
  pub x_workbook_view: Vec<WorkbookView>,
}
/// Defines the Sheets Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sheets/x:sheets")]
pub struct Sheets {
  /// Sheet Information.
  #[sdk(child(qname = "x:CT_Sheet/x:sheet"))]
  pub x_sheet: Vec<Sheet>,
}
/// Defines the FunctionGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FunctionGroups/x:functionGroups")]
pub struct FunctionGroups {
  /// Built-in Function Group Count
  #[sdk(attr(qname = ":builtInGroupCount"))]
  pub built_in_group_count: Option<crate::simple_type::UInt32Value>,
  /// Function Group.
  #[sdk(child(qname = "x:CT_FunctionGroup/x:functionGroup"))]
  pub x_function_group: Vec<FunctionGroup>,
}
/// Defines the ExternalReferences Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalReferences/x:externalReferences")]
pub struct ExternalReferences {
  /// External Reference.
  #[sdk(child(qname = "x:CT_ExternalReference/x:externalReference"))]
  pub x_external_reference: Vec<ExternalReference>,
}
/// Defines the DefinedNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DefinedNames/x:definedNames")]
pub struct DefinedNames {
  /// Defined Name.
  #[sdk(child(qname = "x:CT_DefinedName/x:definedName"))]
  pub x_defined_name: Vec<DefinedName>,
}
/// Defines the CalculationProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcPr/x:calcPr")]
pub struct CalculationProperties {
  /// Calculation Id
  #[sdk(attr(qname = ":calcId"))]
  pub calculation_id: Option<crate::simple_type::UInt32Value>,
  /// Calculation Mode
  #[sdk(attr(qname = ":calcMode"))]
  pub calculation_mode: Option<CalculateModeValues>,
  /// Full Calculation On Load
  #[sdk(attr(qname = ":fullCalcOnLoad"))]
  pub full_calculation_on_load: Option<crate::simple_type::BooleanValue>,
  /// Reference Mode
  #[sdk(attr(qname = ":refMode"))]
  pub reference_mode: Option<ReferenceModeValues>,
  /// Calculation Iteration
  #[sdk(attr(qname = ":iterate"))]
  pub iterate: Option<crate::simple_type::BooleanValue>,
  /// Iteration Count
  #[sdk(attr(qname = ":iterateCount"))]
  pub iterate_count: Option<crate::simple_type::UInt32Value>,
  /// Iterative Calculation Delta
  #[sdk(attr(qname = ":iterateDelta"))]
  pub iterate_delta: Option<crate::simple_type::DoubleValue>,
  /// Full Precision Calculation
  #[sdk(attr(qname = ":fullPrecision"))]
  pub full_precision: Option<crate::simple_type::BooleanValue>,
  /// Calc Completed
  #[sdk(attr(qname = ":calcCompleted"))]
  pub calculation_completed: Option<crate::simple_type::BooleanValue>,
  /// Calculate On Save
  #[sdk(attr(qname = ":calcOnSave"))]
  pub calculation_on_save: Option<crate::simple_type::BooleanValue>,
  /// Concurrent Calculations
  #[sdk(attr(qname = ":concurrentCalc"))]
  pub concurrent_calculation: Option<crate::simple_type::BooleanValue>,
  /// Concurrent Thread Manual Count
  #[sdk(attr(qname = ":concurrentManualCount"))]
  pub concurrent_manual_count: Option<crate::simple_type::UInt32Value>,
  /// Force Full Calculation
  #[sdk(attr(qname = ":forceFullCalc"))]
  pub force_full_calculation: Option<crate::simple_type::BooleanValue>,
}
/// Defines the OleSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleSize/x:oleSize")]
pub struct OleSize {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Defines the CustomWorkbookViews Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomWorkbookViews/x:customWorkbookViews")]
pub struct CustomWorkbookViews {
  /// Custom Workbook View.
  #[sdk(child(qname = "x:CT_CustomWorkbookView/x:customWorkbookView"))]
  pub x_custom_workbook_view: Vec<CustomWorkbookView>,
}
/// Defines the PivotCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/x:pivotCaches")]
pub struct PivotCaches {
  /// PivotCache.
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache: Vec<PivotCache>,
}
/// Defines the WebPublishing Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishing/x:webPublishing")]
pub struct WebPublishing {
  /// css
  #[sdk(attr(qname = ":css"))]
  pub use_css: Option<crate::simple_type::BooleanValue>,
  /// thicket
  #[sdk(attr(qname = ":thicket"))]
  pub thicket: Option<crate::simple_type::BooleanValue>,
  /// longFileNames
  #[sdk(attr(qname = ":longFileNames"))]
  pub long_file_names: Option<crate::simple_type::BooleanValue>,
  /// vml
  #[sdk(attr(qname = ":vml"))]
  pub use_vml: Option<crate::simple_type::BooleanValue>,
  /// allowPng
  #[sdk(attr(qname = ":allowPng"))]
  pub allow_png: Option<crate::simple_type::BooleanValue>,
  /// targetScreenSize
  #[sdk(attr(qname = ":targetScreenSize"))]
  pub target_screen_size: Option<TargetScreenSizeValues>,
  /// dpi
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub character_set: Option<crate::simple_type::StringValue>,
}
/// Defines the FileRecoveryProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileRecoveryPr/x:fileRecoveryPr")]
pub struct FileRecoveryProperties {
  /// Auto Recover
  #[sdk(attr(qname = ":autoRecover"))]
  pub auto_recover: Option<crate::simple_type::BooleanValue>,
  /// Crash Save
  #[sdk(attr(qname = ":crashSave"))]
  pub crash_save: Option<crate::simple_type::BooleanValue>,
  /// Data Extract Load
  #[sdk(attr(qname = ":dataExtractLoad"))]
  pub data_extract_load: Option<crate::simple_type::BooleanValue>,
  /// Repair Load
  #[sdk(attr(qname = ":repairLoad"))]
  pub repair_load: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebPublishObjects Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishObjects/x:webPublishObjects")]
pub struct WebPublishObjects {
  /// Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Web Publishing Object.
  #[sdk(child(qname = "x:CT_WebPublishObject/x:webPublishObject"))]
  pub x_web_publish_object: Vec<WebPublishObject>,
}
/// Defines the WorkbookExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookExtensionList/x:extLst")]
pub struct WorkbookExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WorkbookExtension Class.
  #[sdk(child(qname = "x:CT_WorkbookExtension/x:ext"))]
  pub x_ext: Vec<WorkbookExtension>,
}
/// Defines the WorkbookExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookExtension/x:ext")]
pub struct WorkbookExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "x14:CT_DefinedNames/x14:definedNames",
    qname = "x:CT_PivotCaches/x14:pivotCaches",
    qname = "x14:CT_SlicerCaches/x14:slicerCaches",
    qname = "x14:CT_SlicerCaches/x15:slicerCaches",
    qname = "x14:CT_WorkbookPr/x14:workbookPr",
    qname = "x:CT_PivotCaches/x15:pivotCaches",
    qname = "x15:CT_PivotTableReferences/x15:pivotTableReferences",
    qname = "x:CT_PivotCaches/x15:timelineCachePivotCaches",
    qname = "x15:CT_TimelineCacheRefs/x15:timelineCacheRefs",
    qname = "x15:CT_WorkbookPr/x15:workbookPr",
    qname = "x15:CT_DataModel/x15:dataModel",
    qname = "xlecs:CT_ExternalCodeService/xlecs:externalCodeService",
    qname = "xlwcv:CT_Version/xlwcv:version",
    qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput",
    any
  ))]
  pub workbook_extension_choice: Option<WorkbookExtensionChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionsChoice {
  /// Revision Row Column Insert Delete.
  #[sdk(child(qname = "x:CT_RevisionRowColumn/x:rrc"))]
  XRrc(std::boxed::Box<RevisionRowColumn>),
  /// Revision Cell Move.
  #[sdk(child(qname = "x:CT_RevisionMove/x:rm"))]
  XRm(std::boxed::Box<RevisionMove>),
  /// Revision Custom View.
  #[sdk(child(qname = "x:CT_RevisionCustomView/x:rcv"))]
  XRcv(std::boxed::Box<RevisionCustomView>),
  /// Revision Sheet Name.
  #[sdk(child(qname = "x:CT_RevisionSheetRename/x:rsnm"))]
  XRsnm(std::boxed::Box<RevisionSheetName>),
  /// Revision Insert Sheet.
  #[sdk(child(qname = "x:CT_RevisionInsertSheet/x:ris"))]
  XRis(std::boxed::Box<RevisionInsertSheet>),
  /// Revision Cell Change.
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  /// Revision Format.
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
  /// Revision AutoFormat.
  #[sdk(child(qname = "x:CT_RevisionAutoFormatting/x:raf"))]
  XRaf(std::boxed::Box<RevisionAutoFormat>),
  /// Revision Defined Name.
  #[sdk(child(qname = "x:CT_RevisionDefinedName/x:rdn"))]
  XRdn(std::boxed::Box<RevisionDefinedName>),
  /// Revision Cell Comment.
  #[sdk(child(qname = "x:CT_RevisionComment/x:rcmt"))]
  XRcmt(std::boxed::Box<RevisionComment>),
  /// Revision Query Table.
  #[sdk(child(qname = "x:CT_RevisionQueryTableField/x:rqt"))]
  XRqt(std::boxed::Box<RevisionQueryTable>),
  /// Revision Merge Conflict.
  #[sdk(child(qname = "x:CT_RevisionConflict/x:rcft"))]
  XRcft(std::boxed::Box<RevisionConflict>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ExternalLinkChoice {
  #[sdk(child(qname = "x:CT_ExternalBook/x:externalBook"))]
  XExternalBook(std::boxed::Box<ExternalBook>),
  #[sdk(child(qname = "x:CT_DdeLink/x:ddeLink"))]
  XDdeLink(std::boxed::Box<DdeLink>),
  #[sdk(child(qname = "x:CT_OleLink/x:oleLink"))]
  XOleLink(std::boxed::Box<OleLink>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FilterColumnChoice {
  /// Filter Criteria.
  #[sdk(child(qname = "x:CT_Filters/x:filters"))]
  XFilters(std::boxed::Box<Filters>),
  /// Top 10.
  #[sdk(child(qname = "x:CT_Top10/x:top10"))]
  XTop10(std::boxed::Box<Top10>),
  /// Defines the CustomFilters Class.
  #[sdk(child(office2010, qname = "x14:CT_CustomFilters/x14:customFilters"))]
  X14CustomFilters(std::boxed::Box<crate::schemas::x14::CustomFilters>),
  /// Custom Filters.
  #[sdk(child(qname = "x:CT_CustomFilters/x:customFilters"))]
  XCustomFilters(std::boxed::Box<CustomFilters>),
  /// Dynamic Filter.
  #[sdk(child(qname = "x:CT_DynamicFilter/x:dynamicFilter"))]
  XDynamicFilter(std::boxed::Box<DynamicFilter>),
  /// Color Filter Criteria.
  #[sdk(child(qname = "x:CT_ColorFilter/x:colorFilter"))]
  XColorFilter(std::boxed::Box<ColorFilter>),
  /// Defines the IconFilter Class.
  #[sdk(child(office2010, qname = "x14:CT_IconFilter/x14:iconFilter"))]
  X14IconFilter(std::boxed::Box<crate::schemas::x14::IconFilter>),
  /// Icon Filter.
  #[sdk(child(qname = "x:CT_IconFilter/x:iconFilter"))]
  XIconFilter(std::boxed::Box<IconFilter>),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SortStateChoice {
  #[sdk(child(office2010, qname = "x14:CT_SortCondition/x14:sortCondition"))]
  X14SortCondition(std::boxed::Box<crate::schemas::x14::SortCondition>),
  #[sdk(child(qname = "x:CT_SortCondition/x:sortCondition"))]
  XSortCondition(std::boxed::Box<SortCondition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TablesChoice {
  /// No Value.
  #[sdk(empty_child(qname = "x:CT_TableMissing/x:m"))]
  XM,
  /// Character Value.
  #[sdk(child(qname = "x:CT_XStringElement/x:s"))]
  XS(std::boxed::Box<CharacterValue>),
  /// Index.
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  XX(std::boxed::Box<FieldItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotCacheRecordChoice {
  /// No Value.
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  /// Numeric.
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  /// Boolean.
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  /// Error Value.
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  /// Character Value.
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  /// Date Time.
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
  /// Index.
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  XX(std::boxed::Box<FieldItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EntriesChoice {
  /// No Value.
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  /// Numeric.
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  /// Error Value.
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  /// Character Value.
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GroupItemsChoice {
  /// No Value.
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  /// Numeric.
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  /// Boolean.
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  /// Error Value.
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  /// Character Value.
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  /// Date Time.
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionRowColumnChoice {
  /// Undo.
  #[sdk(child(qname = "x:CT_UndoInfo/x:undo"))]
  XUndo(std::boxed::Box<Undo>),
  /// Revision Cell Change.
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  /// Revision Format.
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionMoveChoice {
  /// Undo.
  #[sdk(child(qname = "x:CT_UndoInfo/x:undo"))]
  XUndo(std::boxed::Box<Undo>),
  /// Revision Cell Change.
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  /// Revision Format.
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MdxChoice {
  /// Tuple MDX Metadata.
  #[sdk(child(qname = "x:CT_MdxTuple/x:t"))]
  XT(std::boxed::Box<MdxTuple>),
  /// Set MDX Metadata.
  #[sdk(child(qname = "x:CT_MdxSet/x:ms"))]
  XMs(std::boxed::Box<MdxSet>),
  /// Member Property MDX Metadata.
  #[sdk(child(qname = "x:CT_MdxMemeberProp/x:p"))]
  XP(std::boxed::Box<MdxMemberProp>),
  /// KPI MDX Metadata.
  #[sdk(child(qname = "x:CT_MdxKPI/x:k"))]
  XK(std::boxed::Box<MdxKpi>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillChoice {
  /// Pattern.
  #[sdk(child(qname = "x:CT_PatternFill/x:patternFill"))]
  XPatternFill(std::boxed::Box<PatternFill>),
  /// Gradient.
  #[sdk(child(qname = "x:CT_GradientFill/x:gradientFill"))]
  XGradientFill(std::boxed::Box<GradientFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OleItemsChoice {
  /// OLE Link Item.
  #[sdk(child(qname = "x:CT_OleItem/x:oleItem"))]
  XOleItem(std::boxed::Box<OleItem>),
  /// Defines the OleItem Class.
  #[sdk(child(office2010, qname = "x14:CT_OleItem/x14:oleItem"))]
  X14OleItem(std::boxed::Box<crate::schemas::x14::OleItem>),
  /// Unknown XML child.
  #[sdk(any)]
  XmlOther(String),
  /// Unknown XML text.
  #[sdk(text)]
  XmlText(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ConditionalFormattingRuleExtensionChoice {
  /// Defines the Id Class.
  #[sdk(text_child(office2010, qname = "x:ST_Guid/x14:id"))]
  X14Id(crate::simple_type::StringValue),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotHierarchyExtensionChoice {
  /// Defines the PivotHierarchy Class.
  #[sdk(child(office2010, qname = "x14:CT_PivotHierarchy/x14:pivotHierarchy"))]
  X14PivotHierarchy(std::boxed::Box<crate::schemas::x14::PivotHierarchy>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotFieldExtensionChoice {
  /// Defines the PivotField Class.
  #[sdk(child(office2010, qname = "x14:CT_PivotField/x14:pivotField"))]
  X14PivotField(std::boxed::Box<crate::schemas::x14::PivotField>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CacheSourceExtensionChoice {
  /// Defines the SourceConnection Class.
  #[sdk(child(office2010, qname = "x14:CT_SourceConnection/x14:sourceConnection"))]
  X14SourceConnection(std::boxed::Box<crate::schemas::x14::SourceConnection>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FiltersChoice {
  /// Defines the Filter Class.
  #[sdk(child(office2010, qname = "x14:CT_Filter/x14:filter"))]
  X14Filter(std::boxed::Box<crate::schemas::x14::Filter>),
  /// Filter.
  #[sdk(child(qname = "x:CT_Filter/x:filter"))]
  XFilter(std::boxed::Box<Filter>),
  /// Date Grouping.
  #[sdk(child(qname = "x:CT_DateGroupItem/x:dateGroupItem"))]
  XDateGroupItem(std::boxed::Box<DateGroupItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlicerCacheDefinitionExtensionChoice {
  /// Defines the SlicerCachePivotTables Class.
  #[sdk(child(
    office2013,
    qname = "x14:CT_SlicerCachePivotTables/x15:slicerCachePivotTables"
  ))]
  X15SlicerCachePivotTables(std::boxed::Box<crate::schemas::x15::SlicerCachePivotTables>),
  /// Defines the TableSlicerCache Class.
  #[sdk(child(office2013, qname = "x15:CT_TableSlicerCache/x15:tableSlicerCache"))]
  X15TableSlicerCache(std::boxed::Box<crate::schemas::x15::TableSlicerCache>),
  /// Defines the SlicerCacheHideItemsWithNoData Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_SlicerCacheHideNoData/x15:slicerCacheHideItemsWithNoData"
  ))]
  X15SlicerCacheHideItemsWithNoData(
    std::boxed::Box<crate::schemas::x15::SlicerCacheHideItemsWithNoData>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotFilterExtensionChoice {
  /// Defines the PivotFilter Class.
  #[sdk(child(office2013, qname = "x15:CT_PivotFilter/x15:pivotFilter"))]
  X15PivotFilter(std::boxed::Box<crate::schemas::x15::PivotFilter>),
  /// Defines the MovingPeriodState Class.
  #[sdk(child(office2013, qname = "x15:CT_MovingPeriodState/x15:movingPeriodState"))]
  X15MovingPeriodState(std::boxed::Box<crate::schemas::x15::MovingPeriodState>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum QueryTableExtensionChoice {
  /// Defines the QueryTable Class.
  #[sdk(child(office2013, qname = "x15:CT_QueryTable/x15:queryTable"))]
  X15QueryTable(std::boxed::Box<crate::schemas::x15::QueryTable>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ConnectionExtensionChoice {
  /// Defines the Connection Class.
  #[sdk(child(office2010, qname = "x14:CT_Connection/x14:connection"))]
  X14Connection(std::boxed::Box<crate::schemas::x14::Connection>),
  /// Defines the Connection Class.
  #[sdk(child(office2013, qname = "x15:CT_Connection/x15:connection"))]
  X15Connection(std::boxed::Box<crate::schemas::x15::Connection>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SharedItemsChoice {
  /// No Value.
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  /// Numeric.
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  /// Boolean.
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  /// Error Value.
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  /// Character Value.
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  /// Date Time.
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FieldGroupChoice {
  #[sdk(child(qname = "x:CT_RangePr/x:rangePr"))]
  XRangePr(std::boxed::Box<RangeProperties>),
  #[sdk(child(qname = "x:CT_DiscretePr/x:discretePr"))]
  XDiscretePr(std::boxed::Box<DiscreteProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CacheFieldExtensionChoice {
  /// Defines the CacheField Class.
  #[sdk(child(office2010, qname = "x14:CT_CacheField/x14:cacheField"))]
  X14CacheField(std::boxed::Box<crate::schemas::x14::CacheField>),
  /// Defines the CachedUniqueNames Class.
  #[sdk(child(office2013, qname = "x15:CT_CachedUniqueNames/x15:cachedUniqueNames"))]
  X15CachedUniqueNames(std::boxed::Box<crate::schemas::x15::CachedUniqueNames>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CacheHierarchyExtensionChoice {
  /// Defines the CacheHierarchy Class.
  #[sdk(child(office2010, qname = "x14:CT_CacheHierarchy/x14:cacheHierarchy"))]
  X14CacheHierarchy(std::boxed::Box<crate::schemas::x14::CacheHierarchy>),
  /// Defines the CacheHierarchy Class.
  #[sdk(child(office2013, qname = "x15:CT_CacheHierarchy/x15:cacheHierarchy"))]
  X15CacheHierarchy(std::boxed::Box<crate::schemas::x15::CacheHierarchy>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CalculatedMemberExtensionChoice {
  /// Defines the CalculatedMember Class.
  #[sdk(child(office2010, qname = "x14:CT_CalculatedMember/x14:calculatedMember"))]
  X14CalculatedMember(std::boxed::Box<crate::schemas::x14::CalculatedMember>),
  /// Defines the CalculatedMember Class.
  #[sdk(child(office2013, qname = "x15:CT_CalculatedMember/x15:calculatedMember"))]
  X15CalculatedMember(std::boxed::Box<crate::schemas::x15::CalculatedMember>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DataFieldExtensionChoice {
  /// Defines the DataField Class.
  #[sdk(child(office2010, qname = "x14:CT_DataField/x14:dataField"))]
  X14DataField(std::boxed::Box<crate::schemas::x14::DataField>),
  /// Defines the DataField Class.
  #[sdk(child(office2013, qname = "x15:CT_DataField/x15:dataField"))]
  X15DataField(std::boxed::Box<crate::schemas::x15::DataField>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WorksheetExtensionChoice {
  /// Defines the ConditionalFormattings Class.
  #[sdk(child(
    office2010,
    qname = "x14:CT_ConditionalFormattings/x14:conditionalFormattings"
  ))]
  X14ConditionalFormattings(std::boxed::Box<crate::schemas::x14::ConditionalFormattings>),
  /// Defines the DataValidations Class.
  #[sdk(child(office2010, qname = "x14:CT_DataValidations/x14:dataValidations"))]
  X14DataValidations(std::boxed::Box<crate::schemas::x14::DataValidations>),
  /// Defines the SparklineGroups Class.
  #[sdk(child(office2010, qname = "x14:CT_SparklineGroups/x14:sparklineGroups"))]
  X14SparklineGroups(std::boxed::Box<crate::schemas::x14::SparklineGroups>),
  /// Defines the SlicerList Class.
  #[sdk(child(office2010, qname = "x14:CT_SlicerRefs/x14:slicerList"))]
  X14SlicerList(std::boxed::Box<crate::schemas::x14::SlicerList>),
  /// Defines the ProtectedRanges Class.
  #[sdk(child(office2010, qname = "x14:CT_ProtectedRanges/x14:protectedRanges"))]
  X14ProtectedRanges(std::boxed::Box<crate::schemas::x14::ProtectedRanges>),
  /// Defines the IgnoredErrors Class.
  #[sdk(child(office2010, qname = "x14:CT_IgnoredErrors/x14:ignoredErrors"))]
  X14IgnoredErrors(std::boxed::Box<crate::schemas::x14::IgnoredErrors>),
  /// Defines the WebExtensions Class.
  #[sdk(child(office2013, qname = "x15:CT_WebExtensions/x15:webExtensions"))]
  X15WebExtensions(std::boxed::Box<crate::schemas::x15::WebExtensions>),
  /// Defines the TimelineReferences Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineRefs/x15:timelineRefs"))]
  X15TimelineRefs(std::boxed::Box<crate::schemas::x15::TimelineReferences>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StylesheetExtensionChoice {
  /// Defines the DifferentialFormats Class.
  #[sdk(child(office2010, qname = "x:CT_Dxfs/x14:dxfs"))]
  X14Dxfs(std::boxed::Box<crate::schemas::x14::DifferentialFormats>),
  /// Defines the SlicerStyles Class.
  #[sdk(child(office2010, qname = "x14:CT_SlicerStyles/x14:slicerStyles"))]
  X14SlicerStyles(std::boxed::Box<crate::schemas::x14::SlicerStyles>),
  /// Defines the DifferentialFormats Class.
  #[sdk(child(office2013, qname = "x:CT_Dxfs/x15:dxfs"))]
  X15Dxfs(std::boxed::Box<crate::schemas::x15::DifferentialFormats>),
  /// Defines the TimelineStyles Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineStyles/x15:timelineStyles"))]
  X15TimelineStyles(std::boxed::Box<crate::schemas::x15::TimelineStyles>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotTableDefinitionExtensionChoice {
  /// Defines the PivotTableDefinition Class.
  #[sdk(child(
    office2010,
    qname = "x14:CT_PivotTableDefinition/x14:pivotTableDefinition"
  ))]
  X14PivotTableDefinition(std::boxed::Box<crate::schemas::x14::PivotTableDefinition>),
  /// Defines the PivotTableData Class.
  #[sdk(child(office2013, qname = "x15:CT_PivotTableData/x15:pivotTableData"))]
  X15PivotTableData(std::boxed::Box<crate::schemas::x15::PivotTableData>),
  /// Defines the PivotTableUISettings Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_PivotTableUISettings/x15:pivotTableUISettings"
  ))]
  X15PivotTableUiSettings(std::boxed::Box<crate::schemas::x15::PivotTableUiSettings>),
  /// Defines the PivotVersionInfo Class.
  #[sdk(child(
    microsoft365,
    qname = "xxpvi:CT_PivotVersionInfo/xxpvi:pivotVersionInfo"
  ))]
  XxpviPivotVersionInfo(std::boxed::Box<crate::schemas::xxpvi::PivotVersionInfo>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CacheSourceChoice {
  /// Defines the WorksheetSource Class.
  #[sdk(child(qname = "x:CT_WorksheetSource/x:worksheetSource"))]
  XWorksheetSource(std::boxed::Box<WorksheetSource>),
  /// Defines the Consolidation Class.
  #[sdk(child(qname = "x:CT_Consolidation/x:consolidation"))]
  XConsolidation(std::boxed::Box<Consolidation>),
  /// Defines the CacheSourceExtensionList Class.
  #[sdk(child(qname = "x:CT_CacheSourceExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<CacheSourceExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotCacheDefinitionExtensionChoice {
  /// Defines the PivotCacheDefinition Class.
  #[sdk(child(
    office2010,
    qname = "x14:CT_PivotCacheDefinition/x14:pivotCacheDefinition"
  ))]
  X14PivotCacheDefinition(std::boxed::Box<crate::schemas::x14::PivotCacheDefinition>),
  /// Defines the PivotCacheDecoupled Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_PivotCacheDecoupled/x15:pivotCacheDecoupled"
  ))]
  X15PivotCacheDecoupled(std::boxed::Box<crate::schemas::x15::PivotCacheDecoupled>),
  /// Defines the TimelinePivotCacheDefinition Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_TimelinePivotCacheDefinition/x15:timelinePivotCacheDefinition"
  ))]
  X15TimelinePivotCacheDefinition(
    std::boxed::Box<crate::schemas::x15::TimelinePivotCacheDefinition>,
  ),
  /// Defines the PivotCacheIdVersion Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_PivotCacheIdVersion/x15:pivotCacheIdVersion"
  ))]
  X15PivotCacheIdVersion(std::boxed::Box<crate::schemas::x15::PivotCacheIdVersion>),
  /// Defines the Xsdboolean Class.
  #[sdk(text_child(office2021, qname = "xsd:boolean/xxpim:implicitMeasureSupport"))]
  XxpimImplicitMeasureSupport(crate::simple_type::BooleanValue),
  /// Defines the PivotCacheRichInfo Class.
  #[sdk(child(microsoft365, qname = "xprd:CT_PivotCacheRichInfo/xprd:richInfo"))]
  XprdRichInfo(std::boxed::Box<crate::schemas::xprd::PivotCacheRichInfo>),
  /// Defines the CacheVersionInfo Class.
  #[sdk(child(
    microsoft365,
    qname = "xxpvi:CT_CacheVersionInfo/xxpvi:cacheVersionInfo"
  ))]
  XxpviCacheVersionInfo(std::boxed::Box<crate::schemas::xxpvi::CacheVersionInfo>),
  /// Defines the Xsdboolean Class.
  #[sdk(text_child(microsoft365, qname = "xsd:boolean/xlpar:autoRefresh"))]
  XlparAutoRefresh(crate::simple_type::BooleanValue),
  /// Defines the PivotCacheDynamicArray Class.
  #[sdk(child(
    microsoft365,
    qname = "xlpda:CT_PivotCacheDynamicArray/xlpda:pivotCacheDynamicArray"
  ))]
  XlpdaPivotCacheDynamicArray(std::boxed::Box<crate::schemas::xlpda::PivotCacheDynamicArray>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TableExtensionChoice {
  /// Defines the Table Class.
  #[sdk(child(office2010, qname = "x14:CT_Table/x14:table"))]
  X14Table(std::boxed::Box<crate::schemas::x14::Table>),
  /// Defines the MsForm Class.
  #[sdk(child(microsoft365, qname = "xlmsforms:CT_MsForm/xlmsforms:msForm"))]
  XlmsformsMsForm(std::boxed::Box<crate::schemas::xlmsforms::MsForm>),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum WorkbookExtensionChoice {
  /// Defines the DefinedNames Class.
  #[sdk(child(office2010, qname = "x14:CT_DefinedNames/x14:definedNames"))]
  X14DefinedNames(std::boxed::Box<crate::schemas::x14::DefinedNames>),
  /// Defines the PivotCaches Class.
  #[sdk(child(office2010, qname = "x:CT_PivotCaches/x14:pivotCaches"))]
  X14PivotCaches(std::boxed::Box<crate::schemas::x14::PivotCaches>),
  /// Defines the SlicerCaches Class.
  #[sdk(child(office2010, qname = "x14:CT_SlicerCaches/x14:slicerCaches"))]
  X14SlicerCaches(std::boxed::Box<crate::schemas::x14::SlicerCaches>),
  /// Defines the SlicerCaches Class.
  #[sdk(child(office2013, qname = "x14:CT_SlicerCaches/x15:slicerCaches"))]
  X15SlicerCaches(std::boxed::Box<crate::schemas::x15::SlicerCaches>),
  /// Defines the WorkbookProperties Class.
  #[sdk(child(office2010, qname = "x14:CT_WorkbookPr/x14:workbookPr"))]
  X14WorkbookPr(std::boxed::Box<crate::schemas::x14::WorkbookProperties>),
  /// Defines the PivotCaches Class.
  #[sdk(child(office2013, qname = "x:CT_PivotCaches/x15:pivotCaches"))]
  X15PivotCaches(std::boxed::Box<crate::schemas::x15::PivotCaches>),
  /// Defines the PivotTableReferences Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_PivotTableReferences/x15:pivotTableReferences"
  ))]
  X15PivotTableReferences(std::boxed::Box<crate::schemas::x15::PivotTableReferences>),
  /// Defines the TimelineCachePivotCaches Class.
  #[sdk(child(office2013, qname = "x:CT_PivotCaches/x15:timelineCachePivotCaches"))]
  X15TimelineCachePivotCaches(std::boxed::Box<crate::schemas::x15::TimelineCachePivotCaches>),
  /// Defines the TimelineCacheReferences Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineCacheRefs/x15:timelineCacheRefs"))]
  X15TimelineCacheRefs(std::boxed::Box<crate::schemas::x15::TimelineCacheReferences>),
  /// Defines the WorkbookProperties Class.
  #[sdk(child(office2013, qname = "x15:CT_WorkbookPr/x15:workbookPr"))]
  X15WorkbookPr(std::boxed::Box<crate::schemas::x15::WorkbookProperties>),
  /// Defines the DataModel Class.
  #[sdk(child(office2013, qname = "x15:CT_DataModel/x15:dataModel"))]
  X15DataModel(std::boxed::Box<crate::schemas::x15::DataModel>),
  /// Defines the ExternalCodeService Class.
  #[sdk(child(
    microsoft365,
    qname = "xlecs:CT_ExternalCodeService/xlecs:externalCodeService"
  ))]
  XlecsExternalCodeService(std::boxed::Box<crate::schemas::xlecs::ExternalCodeService>),
  /// Defines the Version Class.
  #[sdk(child(microsoft365, qname = "xlwcv:CT_Version/xlwcv:version"))]
  XlwcvVersion(std::boxed::Box<crate::schemas::xlwcv::Version>),
  /// Defines the ExternalCodeServiceImageAsInput Class.
  #[sdk(child(
    microsoft365,
    qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput"
  ))]
  Xlecs2ExternalCodeServiceImageAsInput(
    std::boxed::Box<crate::schemas::xlecs2::ExternalCodeServiceImageAsInput>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
