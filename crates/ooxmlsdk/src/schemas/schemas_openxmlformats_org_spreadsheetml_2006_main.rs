//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum SortMethodValues {
  #[sdk(rename = "stroke")]
  #[default]
  Stroke,
  #[sdk(rename = "pinYin")]
  PinYin,
  #[sdk(rename = "none")]
  None,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum HtmlFormattingValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "rtf")]
  HonorRichText,
  #[sdk(rename = "all")]
  All,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ParameterValues {
  #[sdk(rename = "prompt")]
  #[default]
  Prompt,
  #[sdk(rename = "value")]
  Value,
  #[sdk(rename = "cell")]
  Cell,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum FileTypeValues {
  #[sdk(rename = "mac")]
  #[default]
  Mac,
  #[sdk(rename = "win")]
  Win,
  #[sdk(rename = "dos")]
  Dos,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum QualifierValues {
  #[sdk(rename = "doubleQuote")]
  #[default]
  DoubleQuote,
  #[sdk(rename = "singleQuote")]
  SingleQuote,
  #[sdk(rename = "none")]
  None,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum CredentialsMethodValues {
  #[sdk(rename = "integrated")]
  #[default]
  Integrated,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "stored")]
  Stored,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ScopeValues {
  #[sdk(rename = "selection")]
  #[default]
  Selection,
  #[sdk(rename = "data")]
  Data,
  #[sdk(rename = "field")]
  Field,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum FieldSortValues {
  #[sdk(rename = "manual")]
  #[default]
  Manual,
  #[sdk(rename = "ascending")]
  Ascending,
  #[sdk(rename = "descending")]
  Descending,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum FormatActionValues {
  #[sdk(rename = "blank")]
  #[default]
  Blank,
  #[sdk(rename = "formatting")]
  Formatting,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum GrowShrinkValues {
  #[sdk(rename = "insertDelete")]
  #[default]
  InsertDelete,
  #[sdk(rename = "insertClear")]
  InsertClear,
  #[sdk(rename = "overwriteClear")]
  OverwriteClear,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum RevisionActionValues {
  #[sdk(rename = "add")]
  #[default]
  Add,
  #[sdk(rename = "delete")]
  Delete,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum SheetViewValues {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "pageBreakPreview")]
  PageBreakPreview,
  #[sdk(rename = "pageLayout")]
  PageLayout,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum DataValidationErrorStyleValues {
  #[sdk(rename = "stop")]
  #[default]
  Stop,
  #[sdk(rename = "warning")]
  Warning,
  #[sdk(rename = "information")]
  Information,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum PageOrderValues {
  #[sdk(rename = "downThenOver")]
  #[default]
  DownThenOver,
  #[sdk(rename = "overThenDown")]
  OverThenDown,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OrientationValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "portrait")]
  Portrait,
  #[sdk(rename = "landscape")]
  Landscape,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum CellCommentsValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "asDisplayed")]
  AsDisplayed,
  #[sdk(rename = "atEnd")]
  AtEnd,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum DataViewAspectValues {
  #[sdk(rename = "DVASPECT_CONTENT")]
  #[default]
  DataViewAspectContent,
  #[sdk(rename = "DVASPECT_ICON")]
  DataViewAspectIcon,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum OleUpdateValues {
  #[sdk(rename = "OLEUPDATE_ALWAYS")]
  #[default]
  OleUpdateAlways,
  #[sdk(rename = "OLEUPDATE_ONCALL")]
  OleUpdateOnCall,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum PaneStateValues {
  #[sdk(rename = "split")]
  #[default]
  Split,
  #[sdk(rename = "frozen")]
  Frozen,
  #[sdk(rename = "frozenSplit")]
  FrozenSplit,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum GradientValues {
  #[sdk(rename = "linear")]
  #[default]
  Linear,
  #[sdk(rename = "path")]
  Path,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum VerticalAlignmentRunValues {
  #[sdk(rename = "baseline")]
  #[default]
  Baseline,
  #[sdk(rename = "superscript")]
  Superscript,
  #[sdk(rename = "subscript")]
  Subscript,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum FontSchemeValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "major")]
  Major,
  #[sdk(rename = "minor")]
  Minor,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TableValues {
  #[sdk(rename = "worksheet")]
  #[default]
  Worksheet,
  #[sdk(rename = "xml")]
  Xml,
  #[sdk(rename = "queryTable")]
  QueryTable,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum VolatileDependencyValues {
  #[sdk(rename = "realTimeData")]
  #[default]
  RealTimeData,
  #[sdk(rename = "olapFunctions")]
  OlapFunctions,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum VisibilityValues {
  #[sdk(rename = "visible")]
  #[default]
  Visible,
  #[sdk(rename = "hidden")]
  Hidden,
  #[sdk(rename = "veryHidden")]
  VeryHidden,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum CommentsValues {
  #[sdk(rename = "commNone")]
  #[default]
  CommentNone,
  #[sdk(rename = "commIndicator")]
  CommentIndicator,
  #[sdk(rename = "commIndAndComment")]
  CommentIndicatorAndComment,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectDisplayValues {
  #[sdk(rename = "all")]
  #[default]
  All,
  #[sdk(rename = "placeholders")]
  Placeholders,
  #[sdk(rename = "none")]
  None,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum SheetStateValues {
  #[sdk(rename = "visible")]
  #[default]
  Visible,
  #[sdk(rename = "hidden")]
  Hidden,
  #[sdk(rename = "veryHidden")]
  VeryHidden,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum UpdateLinksBehaviorValues {
  #[sdk(rename = "userSet")]
  #[default]
  UserSet,
  #[sdk(rename = "never")]
  Never,
  #[sdk(rename = "always")]
  Always,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum CalculateModeValues {
  #[sdk(rename = "manual")]
  #[default]
  Manual,
  #[sdk(rename = "auto")]
  Auto,
  #[sdk(rename = "autoNoTable")]
  AutoNoTable,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ReferenceModeValues {
  #[sdk(rename = "A1")]
  #[default]
  A1,
  #[sdk(rename = "R1C1")]
  R1c1,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(rename = "d")]
  Date,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
  #[cfg(feature = "microsoft365")]
  #[sdk(rename = "topEnd")]
  TopEnd,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ConformanceClass {
  #[sdk(rename = "strict")]
  #[default]
  Enumstrict,
  #[sdk(rename = "transitional")]
  Enumtransitional,
}
/// Extension.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Extension/x:ext")]
pub struct Extension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Vec<ExtensionChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ExtensionChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Calculation Chain Info.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calcChain.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcChain/x:calcChain")]
pub struct CalculationChain {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_CalcCell/x:c"))]
  pub x_c: Vec<CalculationCell>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Comments.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:comments.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Comments/x:comments")]
pub struct Comments {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  ///Authors
  #[sdk(child(qname = "x:CT_Authors/x:authors"))]
  pub authors: std::boxed::Box<Authors>,
  ///List of Comments
  #[sdk(child(qname = "x:CT_CommentList/x:commentList"))]
  pub comment_list: std::boxed::Box<CommentList>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// XML Mapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:MapInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MapInfo/x:MapInfo")]
pub struct MapInfo {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Prefix Mappings for XPath Expressions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :SelectionNamespaces
  #[sdk(attr(qname = ":SelectionNamespaces"))]
  pub selection_namespaces: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_Schema/x:Schema"))]
  pub x_schema: Vec<Schema>,
  /// _
  #[sdk(child(qname = "x:CT_Map/x:Map"))]
  pub x_map: Vec<Map>,
}
/// Connections.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:connections.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Connections/x:connections")]
pub struct Connections {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_Connection/x:connection"))]
  pub x_connection: Vec<Connection>,
}
/// PivotCache Definition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotCacheDefinition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinition/x:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// invalid
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :invalid
  #[sdk(attr(qname = ":invalid"))]
  pub invalid: Option<crate::simple_type::BooleanValue>,
  /// saveData
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saveData
  #[sdk(attr(qname = ":saveData"))]
  pub save_data: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// optimizeMemory
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :optimizeMemory
  #[sdk(attr(qname = ":optimizeMemory"))]
  pub optimize_memory: Option<crate::simple_type::BooleanValue>,
  /// enableRefresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :enableRefresh
  #[sdk(attr(qname = ":enableRefresh"))]
  pub enable_refresh: Option<crate::simple_type::BooleanValue>,
  /// refreshedBy
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshedBy
  #[sdk(attr(qname = ":refreshedBy"))]
  pub refreshed_by: Option<crate::simple_type::StringValue>,
  /// refreshedDateIso
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshedDateIso
  #[sdk(attr(qname = ":refreshedDateIso"))]
  pub last_refreshed_date_iso: Option<crate::simple_type::DateTimeValue>,
  /// refreshedDate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshedDate
  #[sdk(attr(qname = ":refreshedDate"))]
  pub refreshed_date: Option<crate::simple_type::DoubleValue>,
  /// backgroundQuery
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :backgroundQuery
  #[sdk(attr(qname = ":backgroundQuery"))]
  pub background_query: Option<crate::simple_type::BooleanValue>,
  /// missingItemsLimit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :missingItemsLimit
  #[sdk(attr(qname = ":missingItemsLimit"))]
  pub missing_items_limit: Option<crate::simple_type::UInt32Value>,
  /// createdVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :createdVersion
  #[sdk(attr(qname = ":createdVersion"))]
  pub created_version: Option<crate::simple_type::ByteValue>,
  /// refreshedVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshedVersion
  #[sdk(attr(qname = ":refreshedVersion"))]
  pub refreshed_version: Option<crate::simple_type::ByteValue>,
  /// minRefreshableVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// recordCount
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :recordCount
  #[sdk(attr(qname = ":recordCount"))]
  pub record_count: Option<crate::simple_type::UInt32Value>,
  /// upgradeOnRefresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :upgradeOnRefresh
  #[sdk(attr(qname = ":upgradeOnRefresh"))]
  pub upgrade_on_refresh: Option<crate::simple_type::BooleanValue>,
  /// tupleCache
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tupleCache
  #[sdk(attr(qname = ":tupleCache"))]
  pub is_tuple_cache: Option<crate::simple_type::BooleanValue>,
  /// supportSubquery
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :supportSubquery
  #[sdk(attr(qname = ":supportSubquery"))]
  pub support_subquery: Option<crate::simple_type::BooleanValue>,
  /// supportAdvancedDrill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :supportAdvancedDrill
  #[sdk(attr(qname = ":supportAdvancedDrill"))]
  pub support_advanced_drill: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_CacheSource/x:cacheSource"))]
  pub cache_source: std::boxed::Box<CacheSource>,
  /// _
  #[sdk(child(qname = "x:CT_CacheFields/x:cacheFields"))]
  pub cache_fields: std::boxed::Box<CacheFields>,
  /// _
  #[sdk(child(qname = "x:CT_CacheHierarchies/x:cacheHierarchies"))]
  pub cache_hierarchies: Option<CacheHierarchies>,
  /// _
  #[sdk(child(qname = "x:CT_PCDKPIs/x:kpis"))]
  pub kpis: Option<Kpis>,
  /// _
  #[sdk(child(qname = "x:CT_TupleCache/x:tupleCache"))]
  pub tuple_cache: Option<std::boxed::Box<TupleCache>>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedItems/x:calculatedItems"))]
  pub calculated_items: Option<CalculatedItems>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMembers/x:calculatedMembers"))]
  pub calculated_members: Option<CalculatedMembers>,
  /// _
  #[sdk(child(qname = "x:CT_Dimensions/x:dimensions"))]
  pub dimensions: Option<Dimensions>,
  /// _
  #[sdk(child(qname = "x:CT_MeasureGroups/x:measureGroups"))]
  pub measure_groups: Option<MeasureGroups>,
  /// _
  #[sdk(child(qname = "x:CT_MeasureDimensionMaps/x:maps"))]
  pub maps: Option<Maps>,
  /// _
  #[sdk(child(qname = "x:CT_PivotCacheDefinitionExtensionList/x:extLst"))]
  pub pivot_cache_definition_extension_list: Option<PivotCacheDefinitionExtensionList>,
}
/// PivotCache Records.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotCacheRecords.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheRecords/x:pivotCacheRecords")]
pub struct PivotCacheRecords {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// PivotCache Records Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Record/x:r"))]
  pub x_r: Vec<PivotCacheRecord>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// PivotTable Definition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotTableDefinition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinition/x:pivotTableDefinition")]
pub struct PivotTableDefinition {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cacheId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cacheId
  #[sdk(attr(qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// dataOnRows
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataOnRows
  #[sdk(attr(qname = ":dataOnRows"))]
  pub data_on_rows: Option<crate::simple_type::BooleanValue>,
  /// dataPosition
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataPosition
  #[sdk(attr(qname = ":dataPosition"))]
  pub data_position: Option<crate::simple_type::UInt32Value>,
  /// Auto Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFormatId
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyNumberFormats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyBorderFormats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyFontFormats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyPatternFormats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyAlignmentFormats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyWidthHeightFormats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// dataCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataCaption
  #[sdk(attr(qname = ":dataCaption"))]
  pub data_caption: crate::simple_type::StringValue,
  /// grandTotalCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grandTotalCaption
  #[sdk(attr(qname = ":grandTotalCaption"))]
  pub grand_total_caption: Option<crate::simple_type::StringValue>,
  /// errorCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :errorCaption
  #[sdk(attr(qname = ":errorCaption"))]
  pub error_caption: Option<crate::simple_type::StringValue>,
  /// showError
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showError
  #[sdk(attr(qname = ":showError"))]
  pub show_error: Option<crate::simple_type::BooleanValue>,
  /// missingCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :missingCaption
  #[sdk(attr(qname = ":missingCaption"))]
  pub missing_caption: Option<crate::simple_type::StringValue>,
  /// showMissing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMissing
  #[sdk(attr(qname = ":showMissing"))]
  pub show_missing: Option<crate::simple_type::BooleanValue>,
  /// pageStyle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pageStyle
  #[sdk(attr(qname = ":pageStyle"))]
  pub page_style: Option<crate::simple_type::StringValue>,
  /// pivotTableStyle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pivotTableStyle
  #[sdk(attr(qname = ":pivotTableStyle"))]
  pub pivot_table_style_name: Option<crate::simple_type::StringValue>,
  /// vacatedStyle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vacatedStyle
  #[sdk(attr(qname = ":vacatedStyle"))]
  pub vacated_style: Option<crate::simple_type::StringValue>,
  /// tag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tag
  #[sdk(attr(qname = ":tag"))]
  pub tag: Option<crate::simple_type::StringValue>,
  /// updatedVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :updatedVersion
  #[sdk(attr(qname = ":updatedVersion"))]
  pub updated_version: Option<crate::simple_type::ByteValue>,
  /// minRefreshableVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// asteriskTotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :asteriskTotals
  #[sdk(attr(qname = ":asteriskTotals"))]
  pub asterisk_totals: Option<crate::simple_type::BooleanValue>,
  /// showItems
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showItems
  #[sdk(attr(qname = ":showItems"))]
  pub show_items: Option<crate::simple_type::BooleanValue>,
  /// editData
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :editData
  #[sdk(attr(qname = ":editData"))]
  pub edit_data: Option<crate::simple_type::BooleanValue>,
  /// disableFieldList
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disableFieldList
  #[sdk(attr(qname = ":disableFieldList"))]
  pub disable_field_list: Option<crate::simple_type::BooleanValue>,
  /// showCalcMbrs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showCalcMbrs
  #[sdk(attr(qname = ":showCalcMbrs"))]
  pub show_calculated_members: Option<crate::simple_type::BooleanValue>,
  /// visualTotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :visualTotals
  #[sdk(attr(qname = ":visualTotals"))]
  pub visual_totals: Option<crate::simple_type::BooleanValue>,
  /// showMultipleLabel
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMultipleLabel
  #[sdk(attr(qname = ":showMultipleLabel"))]
  pub show_multiple_label: Option<crate::simple_type::BooleanValue>,
  /// showDataDropDown
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDataDropDown
  #[sdk(attr(qname = ":showDataDropDown"))]
  pub show_data_drop_down: Option<crate::simple_type::BooleanValue>,
  /// showDrill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDrill
  #[sdk(attr(qname = ":showDrill"))]
  pub show_drill: Option<crate::simple_type::BooleanValue>,
  /// printDrill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :printDrill
  #[sdk(attr(qname = ":printDrill"))]
  pub print_drill: Option<crate::simple_type::BooleanValue>,
  /// showMemberPropertyTips
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showMemberPropertyTips
  #[sdk(attr(qname = ":showMemberPropertyTips"))]
  pub show_member_property_tips: Option<crate::simple_type::BooleanValue>,
  /// showDataTips
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDataTips
  #[sdk(attr(qname = ":showDataTips"))]
  pub show_data_tips: Option<crate::simple_type::BooleanValue>,
  /// enableWizard
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :enableWizard
  #[sdk(attr(qname = ":enableWizard"))]
  pub enable_wizard: Option<crate::simple_type::BooleanValue>,
  /// enableDrill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :enableDrill
  #[sdk(attr(qname = ":enableDrill"))]
  pub enable_drill: Option<crate::simple_type::BooleanValue>,
  /// enableFieldProperties
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :enableFieldProperties
  #[sdk(attr(qname = ":enableFieldProperties"))]
  pub enable_field_properties: Option<crate::simple_type::BooleanValue>,
  /// preserveFormatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserveFormatting
  #[sdk(attr(qname = ":preserveFormatting"))]
  pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
  /// useAutoFormatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useAutoFormatting
  #[sdk(attr(qname = ":useAutoFormatting"))]
  pub use_auto_formatting: Option<crate::simple_type::BooleanValue>,
  /// pageWrap
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pageWrap
  #[sdk(attr(qname = ":pageWrap"))]
  pub page_wrap: Option<crate::simple_type::UInt32Value>,
  /// pageOverThenDown
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pageOverThenDown
  #[sdk(attr(qname = ":pageOverThenDown"))]
  pub page_over_then_down: Option<crate::simple_type::BooleanValue>,
  /// subtotalHiddenItems
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalHiddenItems
  #[sdk(attr(qname = ":subtotalHiddenItems"))]
  pub subtotal_hidden_items: Option<crate::simple_type::BooleanValue>,
  /// rowGrandTotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowGrandTotals
  #[sdk(attr(qname = ":rowGrandTotals"))]
  pub row_grand_totals: Option<crate::simple_type::BooleanValue>,
  /// colGrandTotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colGrandTotals
  #[sdk(attr(qname = ":colGrandTotals"))]
  pub column_grand_totals: Option<crate::simple_type::BooleanValue>,
  /// fieldPrintTitles
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldPrintTitles
  #[sdk(attr(qname = ":fieldPrintTitles"))]
  pub field_print_titles: Option<crate::simple_type::BooleanValue>,
  /// itemPrintTitles
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :itemPrintTitles
  #[sdk(attr(qname = ":itemPrintTitles"))]
  pub item_print_titles: Option<crate::simple_type::BooleanValue>,
  /// mergeItem
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mergeItem
  #[sdk(attr(qname = ":mergeItem"))]
  pub merge_item: Option<crate::simple_type::BooleanValue>,
  /// showDropZones
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDropZones
  #[sdk(attr(qname = ":showDropZones"))]
  pub show_drop_zones: Option<crate::simple_type::BooleanValue>,
  /// createdVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :createdVersion
  #[sdk(attr(qname = ":createdVersion"))]
  pub created_version: Option<crate::simple_type::ByteValue>,
  /// indent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indent
  #[sdk(attr(qname = ":indent"))]
  pub indent: Option<crate::simple_type::UInt32Value>,
  /// showEmptyRow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showEmptyRow
  #[sdk(attr(qname = ":showEmptyRow"))]
  pub show_empty_row: Option<crate::simple_type::BooleanValue>,
  /// showEmptyCol
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showEmptyCol
  #[sdk(attr(qname = ":showEmptyCol"))]
  pub show_empty_column: Option<crate::simple_type::BooleanValue>,
  /// showHeaders
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showHeaders
  #[sdk(attr(qname = ":showHeaders"))]
  pub show_headers: Option<crate::simple_type::BooleanValue>,
  /// compact
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compact
  #[sdk(attr(qname = ":compact"))]
  pub compact: Option<crate::simple_type::BooleanValue>,
  /// outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// outlineData
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineData
  #[sdk(attr(qname = ":outlineData"))]
  pub outline_data: Option<crate::simple_type::BooleanValue>,
  /// compactData
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compactData
  #[sdk(attr(qname = ":compactData"))]
  pub compact_data: Option<crate::simple_type::BooleanValue>,
  /// published
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// gridDropZones
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gridDropZones
  #[sdk(attr(qname = ":gridDropZones"))]
  pub grid_drop_zones: Option<crate::simple_type::BooleanValue>,
  /// immersive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :immersive
  #[sdk(attr(qname = ":immersive"))]
  pub stop_immersive_ui: Option<crate::simple_type::BooleanValue>,
  /// multipleFieldFilters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :multipleFieldFilters
  #[sdk(attr(qname = ":multipleFieldFilters"))]
  pub multiple_field_filters: Option<crate::simple_type::BooleanValue>,
  /// chartFormat
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :chartFormat
  #[sdk(attr(qname = ":chartFormat"))]
  pub chart_format: Option<crate::simple_type::UInt32Value>,
  /// rowHeaderCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowHeaderCaption
  #[sdk(attr(qname = ":rowHeaderCaption"))]
  pub row_header_caption: Option<crate::simple_type::StringValue>,
  /// colHeaderCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colHeaderCaption
  #[sdk(attr(qname = ":colHeaderCaption"))]
  pub column_header_caption: Option<crate::simple_type::StringValue>,
  /// fieldListSortAscending
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldListSortAscending
  #[sdk(attr(qname = ":fieldListSortAscending"))]
  pub field_list_sort_ascending: Option<crate::simple_type::BooleanValue>,
  /// mdxSubqueries
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mdxSubqueries
  #[sdk(attr(qname = ":mdxSubqueries"))]
  pub mdx_subqueries: Option<crate::simple_type::BooleanValue>,
  /// customListSort
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customListSort
  #[sdk(attr(qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Location/x:location"))]
  pub location: std::boxed::Box<Location>,
  /// _
  #[sdk(child(qname = "x:CT_PivotFields/x:pivotFields"))]
  pub pivot_fields: Option<PivotFields>,
  /// _
  #[sdk(child(qname = "x:CT_RowFields/x:rowFields"))]
  pub row_fields: Option<RowFields>,
  /// _
  #[sdk(child(qname = "x:CT_rowItems/x:rowItems"))]
  pub row_items: Option<RowItems>,
  /// _
  #[sdk(child(qname = "x:CT_ColFields/x:colFields"))]
  pub column_fields: Option<ColumnFields>,
  /// _
  #[sdk(child(qname = "x:CT_colItems/x:colItems"))]
  pub column_items: Option<ColumnItems>,
  /// _
  #[sdk(child(qname = "x:CT_PageFields/x:pageFields"))]
  pub page_fields: Option<PageFields>,
  /// _
  #[sdk(child(qname = "x:CT_DataFields/x:dataFields"))]
  pub data_fields: Option<DataFields>,
  /// _
  #[sdk(child(qname = "x:CT_Formats/x:formats"))]
  pub formats: Option<Formats>,
  /// _
  #[sdk(child(qname = "x:CT_ConditionalFormats/x:conditionalFormats"))]
  pub conditional_formats: Option<ConditionalFormats>,
  /// _
  #[sdk(child(qname = "x:CT_ChartFormats/x:chartFormats"))]
  pub chart_formats: Option<ChartFormats>,
  /// _
  #[sdk(child(qname = "x:CT_PivotHierarchies/x:pivotHierarchies"))]
  pub pivot_hierarchies: Option<PivotHierarchies>,
  /// _
  #[sdk(child(qname = "x:CT_PivotTableStyle/x:pivotTableStyleInfo"))]
  pub pivot_table_style: Option<PivotTableStyle>,
  /// _
  #[sdk(child(qname = "x:CT_PivotFilters/x:filters"))]
  pub pivot_filters: Option<PivotFilters>,
  /// _
  #[sdk(child(qname = "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage"))]
  pub row_hierarchies_usage: Option<RowHierarchiesUsage>,
  /// _
  #[sdk(child(qname = "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage"))]
  pub column_hierarchies_usage: Option<ColumnHierarchiesUsage>,
  /// _
  #[sdk(child(qname = "x:CT_pivotTableDefinitionExtensionList/x:extLst"))]
  pub pivot_table_definition_extension_list: Option<PivotTableDefinitionExtensionList>,
}
/// Query Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryTable.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTable/x:queryTable")]
pub struct QueryTable {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// headers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headers
  #[sdk(attr(qname = ":headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
  /// rowNumbers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowNumbers
  #[sdk(attr(qname = ":rowNumbers"))]
  pub row_numbers: Option<crate::simple_type::BooleanValue>,
  /// disableRefresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disableRefresh
  #[sdk(attr(qname = ":disableRefresh"))]
  pub disable_refresh: Option<crate::simple_type::BooleanValue>,
  /// backgroundRefresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :backgroundRefresh
  #[sdk(attr(qname = ":backgroundRefresh"))]
  pub background_refresh: Option<crate::simple_type::BooleanValue>,
  /// firstBackgroundRefresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstBackgroundRefresh
  #[sdk(attr(qname = ":firstBackgroundRefresh"))]
  pub first_background_refresh: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// growShrinkType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :growShrinkType
  #[sdk(attr(qname = ":growShrinkType"))]
  pub grow_shrink_type: Option<GrowShrinkValues>,
  /// fillFormulas
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillFormulas
  #[sdk(attr(qname = ":fillFormulas"))]
  pub fill_formulas: Option<crate::simple_type::BooleanValue>,
  /// removeDataOnSave
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :removeDataOnSave
  #[sdk(attr(qname = ":removeDataOnSave"))]
  pub remove_data_on_save: Option<crate::simple_type::BooleanValue>,
  /// disableEdit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disableEdit
  #[sdk(attr(qname = ":disableEdit"))]
  pub disable_edit: Option<crate::simple_type::BooleanValue>,
  /// preserveFormatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserveFormatting
  #[sdk(attr(qname = ":preserveFormatting"))]
  pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
  /// adjustColumnWidth
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :adjustColumnWidth
  #[sdk(attr(qname = ":adjustColumnWidth"))]
  pub adjust_column_width: Option<crate::simple_type::BooleanValue>,
  /// intermediate
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :intermediate
  #[sdk(attr(qname = ":intermediate"))]
  pub intermediate: Option<crate::simple_type::BooleanValue>,
  /// connectionId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: crate::simple_type::UInt32Value,
  /// Auto Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFormatId
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyNumberFormats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyBorderFormats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyFontFormats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyPatternFormats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyAlignmentFormats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyWidthHeightFormats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_QueryTableRefresh/x:queryTableRefresh"))]
  pub query_table_refresh: Option<std::boxed::Box<QueryTableRefresh>>,
  /// _
  #[sdk(child(qname = "x:CT_QueryTableExtensionList/x:extLst"))]
  pub query_table_extension_list: Option<QueryTableExtensionList>,
}
/// Shared String Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sst/x:sst")]
pub struct SharedStringTable {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// String Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Unique String Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueCount
  #[sdk(attr(qname = ":uniqueCount"))]
  pub unique_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Rst/x:si"))]
  pub x_si: Vec<SharedStringItem>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Revision Headers.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:headers.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionHeaders/x:headers")]
pub struct Headers {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Last Revision GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Last GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lastGuid
  #[sdk(attr(qname = ":lastGuid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub last_guid: Option<crate::simple_type::StringValue>,
  /// Shared Workbook
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shared
  #[sdk(attr(qname = ":shared"))]
  pub shared: Option<crate::simple_type::BooleanValue>,
  /// Disk Revisions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :diskRevisions
  #[sdk(attr(qname = ":diskRevisions"))]
  pub disk_revisions: Option<crate::simple_type::BooleanValue>,
  /// History
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :history
  #[sdk(attr(qname = ":history"))]
  pub history: Option<crate::simple_type::BooleanValue>,
  /// Track Revisions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :trackRevisions
  #[sdk(attr(qname = ":trackRevisions"))]
  pub track_revisions: Option<crate::simple_type::BooleanValue>,
  /// Exclusive Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :exclusive
  #[sdk(attr(qname = ":exclusive"))]
  pub exclusive: Option<crate::simple_type::BooleanValue>,
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionId
  #[sdk(attr(qname = ":revisionId"))]
  pub revision_id: Option<crate::simple_type::UInt32Value>,
  /// Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::Int32Value>,
  /// Keep Change History
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :keepChangeHistory
  #[sdk(attr(qname = ":keepChangeHistory"))]
  pub keep_change_history: Option<crate::simple_type::BooleanValue>,
  /// Protected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :protected
  #[sdk(attr(qname = ":protected"))]
  pub protected: Option<crate::simple_type::BooleanValue>,
  /// Preserve History
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserveHistory
  #[sdk(attr(qname = ":preserveHistory"))]
  pub preserve_history: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_RevisionHeader/x:header"))]
  pub x_header: Vec<Header>,
}
/// Revisions.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:revisions.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Revisions/x:revisions")]
pub struct Revisions {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  #[sdk(choice)]
  pub xml_children: Vec<RevisionsChoice>,
}
/// User List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:users.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Users/x:users")]
pub struct Users {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Active User Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_SharedUser/x:userInfo"))]
  pub x_user_info: Vec<UserInfo>,
}
/// Worksheet.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:worksheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Worksheet/x:worksheet")]
pub struct Worksheet {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr:uid
  #[sdk(attr(qname = "xr:uid"))]
  pub xr_uid: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_SheetPr/x:sheetPr"))]
  pub sheet_properties: Option<std::boxed::Box<SheetProperties>>,
  /// _
  #[sdk(child(qname = "x:CT_SheetDimension/x:dimension"))]
  pub sheet_dimension: Option<SheetDimension>,
  /// _
  #[sdk(child(qname = "x:CT_SheetViews/x:sheetViews"))]
  pub sheet_views: Option<std::boxed::Box<SheetViews>>,
  /// _
  #[sdk(child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
  pub sheet_format_properties: Option<SheetFormatProperties>,
  /// _
  #[sdk(child(qname = "x:CT_Cols/x:cols"))]
  pub x_cols: Vec<Columns>,
  /// _
  #[sdk(child(qname = "x:CT_SheetData/x:sheetData"))]
  pub x_sheet_data: std::boxed::Box<SheetData>,
  /// _
  #[sdk(child(qname = "x:CT_SheetCalcPr/x:sheetCalcPr"))]
  pub x_sheet_calc_pr: Option<SheetCalculationProperties>,
  /// _
  #[sdk(child(qname = "x:CT_SheetProtection/x:sheetProtection"))]
  pub x_sheet_protection: Option<SheetProtection>,
  /// _
  #[sdk(child(qname = "x:CT_ProtectedRanges/x:protectedRanges"))]
  pub x_protected_ranges: Option<ProtectedRanges>,
  /// _
  #[sdk(child(qname = "x:CT_Scenarios/x:scenarios"))]
  pub x_scenarios: Option<Scenarios>,
  /// _
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub x_auto_filter: Option<std::boxed::Box<AutoFilter>>,
  /// _
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<std::boxed::Box<SortState>>,
  /// _
  #[sdk(child(qname = "x:CT_DataConsolidate/x:dataConsolidate"))]
  pub x_data_consolidate: Option<std::boxed::Box<DataConsolidate>>,
  /// _
  #[sdk(child(qname = "x:CT_CustomSheetViews/x:customSheetViews"))]
  pub x_custom_sheet_views: Option<CustomSheetViews>,
  /// _
  #[sdk(child(qname = "x:CT_MergeCells/x:mergeCells"))]
  pub x_merge_cells: Option<MergeCells>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
  /// _
  #[sdk(child(qname = "x:CT_ConditionalFormatting/x:conditionalFormatting"))]
  pub x_conditional_formatting: Vec<ConditionalFormatting>,
  /// _
  #[sdk(child(qname = "x:CT_DataValidations/x:dataValidations"))]
  pub x_data_validations: Option<DataValidations>,
  /// _
  #[sdk(child(qname = "x:CT_Hyperlinks/x:hyperlinks"))]
  pub x_hyperlinks: Option<Hyperlinks>,
  /// _
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub x_print_options: Option<PrintOptions>,
  /// _
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub x_page_margins: Option<PageMargins>,
  /// _
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub x_page_setup: Option<PageSetup>,
  /// _
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub x_header_footer: Option<std::boxed::Box<HeaderFooter>>,
  /// _
  #[sdk(child(qname = "x:CT_PageBreak/x:rowBreaks"))]
  pub x_row_breaks: Option<RowBreaks>,
  /// _
  #[sdk(child(qname = "x:CT_PageBreak/x:colBreaks"))]
  pub x_col_breaks: Option<ColumnBreaks>,
  /// _
  #[sdk(child(qname = "x:CT_CustomProperties/x:customProperties"))]
  pub x_custom_properties: Option<CustomProperties>,
  /// _
  #[sdk(child(qname = "x:CT_CellWatches/x:cellWatches"))]
  pub x_cell_watches: Option<CellWatches>,
  /// _
  #[sdk(child(qname = "x:CT_IgnoredErrors/x:ignoredErrors"))]
  pub x_ignored_errors: Option<std::boxed::Box<IgnoredErrors>>,
  /// _
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub x_drawing: Option<Drawing>,
  /// _
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub x_legacy_drawing: Option<LegacyDrawing>,
  /// _
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub x_legacy_drawing_hf: Option<LegacyDrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub x_drawing_hf: Option<DrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_SheetBackgroundPicture/x:picture"))]
  pub x_picture: Option<Picture>,
  /// _
  #[sdk(child(qname = "x:CT_OleObjects/x:oleObjects"))]
  pub x_ole_objects: Option<OleObjects>,
  /// _
  #[sdk(child(qname = "x:CT_Controls/x:controls"))]
  pub x_controls: Option<Controls>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishItems/x:webPublishItems"))]
  pub x_web_publish_items: Option<WebPublishItems>,
  /// _
  #[sdk(child(qname = "x:CT_TableParts/x:tableParts"))]
  pub x_table_parts: Option<TableParts>,
  /// _
  #[sdk(child(qname = "x:CT_WorksheetExtensionList/x:extLst"))]
  pub x_ext_lst: Option<WorksheetExtensionList>,
}
/// Chart Sheet.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:chartsheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Chartsheet/x:chartsheet")]
pub struct Chartsheet {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  ///Chart Sheet Properties
  #[sdk(child(qname = "x:CT_ChartsheetPr/x:sheetPr"))]
  pub chart_sheet_properties: Option<std::boxed::Box<ChartSheetProperties>>,
  ///Chart Sheet Views
  #[sdk(child(qname = "x:CT_ChartsheetViews/x:sheetViews"))]
  pub chart_sheet_views: std::boxed::Box<ChartSheetViews>,
  ///Chart Sheet Protection
  #[sdk(child(qname = "x:CT_ChartsheetProtection/x:sheetProtection"))]
  pub chart_sheet_protection: Option<ChartSheetProtection>,
  ///Custom Chart Sheet Views
  #[sdk(child(qname = "x:CT_CustomChartsheetViews/x:customSheetViews"))]
  pub custom_chartsheet_views: Option<CustomChartsheetViews>,
  /// _
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// _
  #[sdk(child(qname = "x:CT_CsPageSetup/x:pageSetup"))]
  pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
  /// _
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  ///Drawing
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub drawing: std::boxed::Box<Drawing>,
  /// _
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub legacy_drawing: Option<LegacyDrawing>,
  ///Legacy Drawing Reference in  Header Footer
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub drawing_header_footer: Option<DrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_SheetBackgroundPicture/x:picture"))]
  pub picture: Option<Picture>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishItems/x:webPublishItems"))]
  pub web_publish_items: Option<WebPublishItems>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Dialog Sheet.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dialogsheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dialogsheet/x:dialogsheet")]
pub struct DialogSheet {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  ///Sheet Properties
  #[sdk(child(qname = "x:CT_SheetPr/x:sheetPr"))]
  pub sheet_properties: Option<std::boxed::Box<SheetProperties>>,
  ///Dialog Sheet Views
  #[sdk(child(qname = "x:CT_SheetViews/x:sheetViews"))]
  pub sheet_views: Option<std::boxed::Box<SheetViews>>,
  ///Dialog Sheet Format Properties
  #[sdk(child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
  pub sheet_format_properties: Option<SheetFormatProperties>,
  ///Sheet Protection
  #[sdk(child(qname = "x:CT_SheetProtection/x:sheetProtection"))]
  pub sheet_protection: Option<SheetProtection>,
  ///Custom Sheet Views
  #[sdk(child(qname = "x:CT_CustomSheetViews/x:customSheetViews"))]
  pub custom_sheet_views: Option<CustomSheetViews>,
  ///Print Options
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub print_options: Option<PrintOptions>,
  ///Page Margins
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  ///Page Setup Settings
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub page_setup: Option<PageSetup>,
  ///Header and Footer Settings
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  ///Drawing
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub drawing: Option<Drawing>,
  ///Legacy Drawing
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub legacy_drawing: std::boxed::Box<LegacyDrawing>,
  ///Legacy Drawing Header Footer
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub drawing_header_footer: Option<DrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_OleObjects/x:oleObjects"))]
  pub ole_objects: Option<OleObjects>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_Controls/x:controls"))]
  pub controls: Option<Controls>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:metadata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Metadata/x:metadata")]
pub struct Metadata {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  ///Metadata Types Collection
  #[sdk(child(qname = "x:CT_MetadataTypes/x:metadataTypes"))]
  pub metadata_types: Option<MetadataTypes>,
  ///Metadata String Store
  #[sdk(child(qname = "x:CT_MetadataStrings/x:metadataStrings"))]
  pub metadata_strings: Option<MetadataStrings>,
  ///MDX Metadata Information
  #[sdk(child(qname = "x:CT_MdxMetadata/x:mdxMetadata"))]
  pub mdx_metadata: Option<MdxMetadata>,
  /// _
  #[sdk(child(qname = "x:CT_FutureMetadata/x:futureMetadata"))]
  pub x_future_metadata: Vec<FutureMetadata>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataBlocks/x:cellMetadata"))]
  pub x_cell_metadata: Option<CellMetadata>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataBlocks/x:valueMetadata"))]
  pub x_value_metadata: Option<ValueMetadata>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Single Cells.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:singleXmlCells.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SingleXmlCells/x:singleXmlCells")]
pub struct SingleXmlCells {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_SingleXmlCell/x:singleXmlCell"))]
  pub x_single_xml_cell: Vec<SingleXmlCell>,
}
/// Style Sheet.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:styleSheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Stylesheet/x:styleSheet")]
pub struct Stylesheet {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_NumFmts/x:numFmts"))]
  pub numbering_formats: Option<NumberingFormats>,
  /// _
  #[sdk(child(qname = "x:CT_Fonts/x:fonts"))]
  pub fonts: Option<Fonts>,
  /// _
  #[sdk(child(qname = "x:CT_Fills/x:fills"))]
  pub fills: Option<Fills>,
  /// _
  #[sdk(child(qname = "x:CT_Borders/x:borders"))]
  pub borders: Option<Borders>,
  /// _
  #[sdk(child(qname = "x:CT_CellStyleXfs/x:cellStyleXfs"))]
  pub cell_style_formats: Option<CellStyleFormats>,
  /// _
  #[sdk(child(qname = "x:CT_CellXfs/x:cellXfs"))]
  pub cell_formats: Option<CellFormats>,
  /// _
  #[sdk(child(qname = "x:CT_CellStyles/x:cellStyles"))]
  pub cell_styles: Option<CellStyles>,
  /// _
  #[sdk(child(qname = "x:CT_Dxfs/x:dxfs"))]
  pub differential_formats: Option<DifferentialFormats>,
  /// _
  #[sdk(child(qname = "x:CT_TableStyles/x:tableStyles"))]
  pub table_styles: Option<TableStyles>,
  /// _
  #[sdk(child(qname = "x:CT_Colors/x:colors"))]
  pub colors: Option<std::boxed::Box<Colors>>,
  /// _
  #[sdk(child(qname = "x:CT_StylesheetExtensionList/x:extLst"))]
  pub stylesheet_extension_list: Option<StylesheetExtensionList>,
}
/// External Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:externalLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalLink/x:externalLink")]
pub struct ExternalLink {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  #[sdk(choice)]
  pub external_link_choice: Option<ExternalLinkChoice>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:table.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Table/x:table")]
pub struct Table {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Table Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Table Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :displayName
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: crate::simple_type::StringValue,
  /// Table Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Table Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tableType
  #[sdk(attr(qname = ":tableType"))]
  pub table_type: Option<TableValues>,
  /// Header Row Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowCount
  #[sdk(attr(qname = ":headerRowCount"))]
  pub header_row_count: Option<crate::simple_type::UInt32Value>,
  /// Insert Row Showing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertRow
  #[sdk(attr(qname = ":insertRow"))]
  pub insert_row: Option<crate::simple_type::BooleanValue>,
  /// Insert Row Shift
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertRowShift
  #[sdk(attr(qname = ":insertRowShift"))]
  pub insert_row_shift: Option<crate::simple_type::BooleanValue>,
  /// Totals Row Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowCount
  #[sdk(attr(qname = ":totalsRowCount"))]
  pub totals_row_count: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Shown
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowShown
  #[sdk(attr(qname = ":totalsRowShown"))]
  pub totals_row_shown: Option<crate::simple_type::BooleanValue>,
  /// Published
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Header Row Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowDxfId
  #[sdk(attr(qname = ":headerRowDxfId"))]
  pub header_row_format_id: Option<crate::simple_type::UInt32Value>,
  /// Data Area Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataDxfId
  #[sdk(attr(qname = ":dataDxfId"))]
  pub data_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowDxfId
  #[sdk(attr(qname = ":totalsRowDxfId"))]
  pub totals_row_format_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Border Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowBorderDxfId
  #[sdk(attr(qname = ":headerRowBorderDxfId"))]
  pub header_row_border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Table Border Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tableBorderDxfId
  #[sdk(attr(qname = ":tableBorderDxfId"))]
  pub border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Border Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowBorderDxfId
  #[sdk(attr(qname = ":totalsRowBorderDxfId"))]
  pub totals_row_border_format_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowCellStyle
  #[sdk(attr(qname = ":headerRowCellStyle"))]
  pub header_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Data Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataCellStyle
  #[sdk(attr(qname = ":dataCellStyle"))]
  pub data_cell_style: Option<crate::simple_type::StringValue>,
  /// Totals Row Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowCellStyle
  #[sdk(attr(qname = ":totalsRowCellStyle"))]
  pub totals_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Connection ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr:uid
  #[sdk(attr(qname = "xr:uid"))]
  pub xr_uid: Option<crate::simple_type::StringValue>,
  ///Table AutoFilter
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: Option<std::boxed::Box<AutoFilter>>,
  ///Sort State
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub sort_state: Option<std::boxed::Box<SortState>>,
  ///Table Columns
  #[sdk(child(qname = "x:CT_TableColumns/x:tableColumns"))]
  pub table_columns: std::boxed::Box<TableColumns>,
  ///Table Style
  #[sdk(child(qname = "x:CT_TableStyleInfo/x:tableStyleInfo"))]
  pub table_style_info: Option<TableStyleInfo>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_TableExtensionList/x:extLst"))]
  pub table_extension_list: Option<TableExtensionList>,
}
/// Volatile Dependency Types.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:volTypes.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTypes/x:volTypes")]
pub struct VolatileTypes {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_VolType/x:volType"))]
  pub x_vol_type: Vec<VolatileType>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Workbook.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:workbook.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Workbook/x:workbook")]
pub struct Workbook {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// conformance
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :conformance
  #[sdk(attr(qname = ":conformance"))]
  pub conformance: Option<ConformanceClass>,
  /// _
  #[sdk(child(qname = "x:CT_FileVersion/x:fileVersion"))]
  pub file_version: Option<FileVersion>,
  /// _
  #[sdk(child(qname = "x:CT_FileSharing/x:fileSharing"))]
  pub file_sharing: Option<FileSharing>,
  /// _
  #[sdk(child(qname = "x:CT_WorkbookPr/x:workbookPr"))]
  pub workbook_properties: Option<WorkbookProperties>,
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x15ac:CT_AbsolutePath/x15ac:absPath"))]
  pub absolute_path:
    Option<crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_ac::AbsolutePath>,
  /// _
  #[sdk(child(qname = "x:CT_WorkbookProtection/x:workbookProtection"))]
  pub workbook_protection: Option<WorkbookProtection>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "xr:CT_RevisionPtr/xr:revisionPtr"))]
  pub xr_revision_ptr:
    Option<crate::schemas::schemas_microsoft_com_office_spreadsheetml_2014_revision::RevisionPtr>,
  /// _
  #[sdk(child(qname = "x:CT_BookViews/x:bookViews"))]
  pub book_views: Option<BookViews>,
  /// _
  #[sdk(child(qname = "x:CT_Sheets/x:sheets"))]
  pub sheets: std::boxed::Box<Sheets>,
  /// _
  #[sdk(child(qname = "x:CT_FunctionGroups/x:functionGroups"))]
  pub function_groups: Option<FunctionGroups>,
  /// _
  #[sdk(child(qname = "x:CT_ExternalReferences/x:externalReferences"))]
  pub external_references: Option<ExternalReferences>,
  /// _
  #[sdk(child(qname = "x:CT_DefinedNames/x:definedNames"))]
  pub defined_names: Option<DefinedNames>,
  /// _
  #[sdk(child(qname = "x:CT_CalcPr/x:calcPr"))]
  pub calculation_properties: Option<CalculationProperties>,
  /// _
  #[sdk(child(qname = "x:CT_OleSize/x:oleSize"))]
  pub ole_size: Option<OleSize>,
  /// _
  #[sdk(child(qname = "x:CT_CustomWorkbookViews/x:customWorkbookViews"))]
  pub custom_workbook_views: Option<CustomWorkbookViews>,
  /// _
  #[sdk(child(qname = "x:CT_PivotCaches/x:pivotCaches"))]
  pub pivot_caches: Option<PivotCaches>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishing/x:webPublishing"))]
  pub web_publishing: Option<WebPublishing>,
  /// _
  #[sdk(child(qname = "x:CT_FileRecoveryPr/x:fileRecoveryPr"))]
  pub x_file_recovery_pr: Vec<FileRecoveryProperties>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishObjects/x:webPublishObjects"))]
  pub x_web_publish_objects: Option<WebPublishObjects>,
  /// _
  #[sdk(child(qname = "x:CT_WorkbookExtensionList/x:extLst"))]
  pub x_ext_lst: Option<WorkbookExtensionList>,
}
/// AutoFilter Column.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:filterColumn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FilterColumn/x:filterColumn")]
pub struct FilterColumn {
  /// Filter Column Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colId
  #[sdk(attr(qname = ":colId"))]
  pub column_id: crate::simple_type::UInt32Value,
  /// Hidden AutoFilter Button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenButton
  #[sdk(attr(qname = ":hiddenButton"))]
  pub hidden_button: Option<crate::simple_type::BooleanValue>,
  /// Show Filter Button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showButton
  #[sdk(attr(qname = ":showButton"))]
  pub show_button: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub xml_children: Option<FilterColumnChoice>,
}
/// Sort State for Auto Filter.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sortState.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SortState/x:sortState")]
pub struct SortState {
  /// Sort by Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :columnSort
  #[sdk(attr(qname = ":columnSort"))]
  pub column_sort: Option<crate::simple_type::BooleanValue>,
  /// Case Sensitive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caseSensitive
  #[sdk(attr(qname = ":caseSensitive"))]
  pub case_sensitive: Option<crate::simple_type::BooleanValue>,
  /// Sort Method
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sortMethod
  #[sdk(attr(qname = ":sortMethod"))]
  pub sort_method: Option<SortMethodValues>,
  /// Sort Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub sort_state_choice: Option<SortStateChoice>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/x:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<Extension>,
}
/// Custom Filter Criteria.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomFilter/x:customFilter")]
pub struct CustomFilter {
  /// Filter Comparison Operator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<FilterOperatorValues>,
  /// Top or Bottom Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Cell.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:c.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcCell/x:c")]
pub struct CalculationCell {
  /// Cell Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub sheet_id: Option<crate::simple_type::Int32Value>,
  /// Child Chain
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub in_child_chain: Option<crate::simple_type::BooleanValue>,
  /// New Dependency Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub new_level: Option<crate::simple_type::BooleanValue>,
  /// New Thread
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub new_thread: Option<crate::simple_type::BooleanValue>,
  /// Array
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :a
  #[sdk(attr(qname = ":a"))]
  pub array: Option<crate::simple_type::BooleanValue>,
}
/// Authors.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:authors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Authors/x:authors")]
pub struct Authors {
  /// _
  #[sdk(child(qname = "x:CT_Xstring/x:author"))]
  pub x_author: Vec<Author>,
}
/// List of Comments.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:commentList.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CommentList/x:commentList")]
pub struct CommentList {
  /// _
  #[sdk(child(qname = "x:CT_Comment/x:comment"))]
  pub x_comment: Vec<Comment>,
}
/// Comment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:comment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Comment/x:comment")]
pub struct Comment {
  /// Cell Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Author Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: crate::simple_type::UInt32Value,
  /// Unique Identifier for Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub guid: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// shapeId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :shapeId
  #[sdk(attr(qname = ":shapeId"))]
  pub shape_id: Option<crate::simple_type::UInt32Value>,
  ///Comment Text
  #[sdk(child(qname = "x:CT_Rst/x:text"))]
  pub comment_text: std::boxed::Box<CommentText>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_CommentPr/x:commentPr"))]
  pub comment_properties: Option<std::boxed::Box<CommentProperties>>,
}
/// Author.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:author.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:author")]
pub struct Author {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:t.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:t")]
pub struct Text {
  /// Preserved Word 2010 extension attribute.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w14:a
  #[sdk(attr(qname = "w14:a"))]
  pub w14_a: Option<crate::simple_type::StringValue>,
  /// Preserved Word 2010 extension attribute.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w14:b
  #[sdk(attr(qname = "w14:b"))]
  pub w14_b: Option<crate::simple_type::StringValue>,
  /// Preserved Word 2010 extension attribute.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w14:c
  #[sdk(attr(qname = "w14:c"))]
  pub w14_c: Option<crate::simple_type::StringValue>,
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Cell Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:v.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:v")]
pub struct CellValue {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:formula.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula")]
pub struct Formula {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Old Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oldFormula.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oldFormula")]
pub struct OldFormula {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Odd Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oddHeader.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oddHeader")]
pub struct OddHeader {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Odd Page Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oddFooter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:oddFooter")]
pub struct OddFooter {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Even Page Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:evenHeader.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:evenHeader")]
pub struct EvenHeader {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Even Page Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:evenFooter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:evenFooter")]
pub struct EvenFooter {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// First Page Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:firstHeader.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:firstHeader")]
pub struct FirstHeader {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// First Page Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:firstFooter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:firstFooter")]
pub struct FirstFooter {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// DDE Link Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:val.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:val")]
pub struct DdeLinkValue {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Strings in Subtopic.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:stp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:stp")]
pub struct Subtopic {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula1 Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:formula1.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula1")]
pub struct Formula1 {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula2 Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:formula2.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/x:formula2")]
pub struct Formula2 {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the XstringType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xstring/")]
pub struct XstringType {
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// XML Schema.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:Schema.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Schema/x:Schema")]
pub struct Schema {
  /// Schema ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ID
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::StringValue,
  /// Schema Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :SchemaRef
  #[sdk(attr(qname = ":SchemaRef"))]
  pub schema_reference: Option<crate::simple_type::StringValue>,
  /// Schema Root Namespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :Namespace
  #[sdk(attr(qname = ":Namespace"))]
  pub namespace: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<SchemaChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SchemaChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// XML Mapping Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:Map.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Map/x:Map")]
pub struct Map {
  /// XML Mapping ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ID
  #[sdk(attr(qname = ":ID"))]
  pub id: crate::simple_type::UInt32Value,
  /// XML Mapping Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :Name
  #[sdk(attr(qname = ":Name"))]
  pub name: crate::simple_type::StringValue,
  /// Root Element Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :RootElement
  #[sdk(attr(qname = ":RootElement"))]
  pub root_element: crate::simple_type::StringValue,
  /// Schema Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :SchemaID
  #[sdk(attr(qname = ":SchemaID"))]
  pub schema_id: crate::simple_type::StringValue,
  /// Show Validation Errors
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ShowImportExportValidationErrors
  #[sdk(attr(qname = ":ShowImportExportValidationErrors"))]
  pub show_import_export_errors: crate::simple_type::BooleanValue,
  /// AutoFit Table on Refresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :AutoFit
  #[sdk(attr(qname = ":AutoFit"))]
  pub auto_fit: crate::simple_type::BooleanValue,
  /// Append Data to Table
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :Append
  #[sdk(attr(qname = ":Append"))]
  pub append_data: crate::simple_type::BooleanValue,
  /// Preserve AutoFilter State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :PreserveSortAFLayout
  #[sdk(attr(qname = ":PreserveSortAFLayout"))]
  pub preserve_auto_filter_state: crate::simple_type::BooleanValue,
  /// Preserve Cell Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :PreserveFormat
  #[sdk(attr(qname = ":PreserveFormat"))]
  pub preserve_format: crate::simple_type::BooleanValue,
  ///XML Mapping
  #[sdk(child(qname = "x:CT_DataBinding/x:DataBinding"))]
  pub data_binding: Option<std::boxed::Box<DataBinding>>,
}
/// XML Mapping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:DataBinding.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataBinding/x:DataBinding")]
pub struct DataBinding {
  /// DataBindingName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :DataBindingName
  #[sdk(attr(qname = ":DataBindingName"))]
  pub data_binding_name: Option<crate::simple_type::StringValue>,
  /// FileBinding
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :FileBinding
  #[sdk(attr(qname = ":FileBinding"))]
  pub file_binding: Option<crate::simple_type::BooleanValue>,
  /// ConnectionID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ConnectionID
  #[sdk(attr(qname = ":ConnectionID"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  /// FileBindingName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :FileBindingName
  #[sdk(attr(qname = ":FileBindingName"))]
  pub file_binding_name: Option<crate::simple_type::StringValue>,
  /// DataBindingLoadMode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :DataBindingLoadMode
  #[sdk(attr(qname = ":DataBindingLoadMode"))]
  pub data_binding_load_mode: crate::simple_type::UInt32Value,
  #[sdk(choice)]
  pub xml_children: Vec<DataBindingChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DataBindingChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Connection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:connection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Connection/x:connection")]
pub struct Connection {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// sourceFile
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// odcFile
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :odcFile
  #[sdk(attr(qname = ":odcFile"))]
  pub connection_file: Option<crate::simple_type::StringValue>,
  /// keepAlive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :keepAlive
  #[sdk(attr(qname = ":keepAlive"))]
  pub keep_alive: Option<crate::simple_type::BooleanValue>,
  /// interval
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :interval
  #[sdk(attr(qname = ":interval"))]
  pub interval: Option<crate::simple_type::UInt32Value>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::UInt32Value>,
  /// reconnectionMethod
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :reconnectionMethod
  #[sdk(attr(qname = ":reconnectionMethod"))]
  pub reconnection_method: Option<crate::simple_type::UInt32Value>,
  /// refreshedVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshedVersion
  #[sdk(attr(qname = ":refreshedVersion"))]
  pub refreshed_version: crate::simple_type::ByteValue,
  /// minRefreshableVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minRefreshableVersion
  #[sdk(attr(qname = ":minRefreshableVersion"))]
  pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
  /// savePassword
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :savePassword
  #[sdk(attr(qname = ":savePassword"))]
  pub save_password: Option<crate::simple_type::BooleanValue>,
  /// new
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :new
  #[sdk(attr(qname = ":new"))]
  pub new: Option<crate::simple_type::BooleanValue>,
  /// deleted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// onlyUseConnectionFile
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :onlyUseConnectionFile
  #[sdk(attr(qname = ":onlyUseConnectionFile"))]
  pub only_use_connection_file: Option<crate::simple_type::BooleanValue>,
  /// background
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :background
  #[sdk(attr(qname = ":background"))]
  pub background: Option<crate::simple_type::BooleanValue>,
  /// refreshOnLoad
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshOnLoad
  #[sdk(attr(qname = ":refreshOnLoad"))]
  pub refresh_on_load: Option<crate::simple_type::BooleanValue>,
  /// saveData
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saveData
  #[sdk(attr(qname = ":saveData"))]
  pub save_data: Option<crate::simple_type::BooleanValue>,
  /// credentials
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :credentials
  #[sdk(attr(qname = ":credentials"))]
  pub credentials: Option<CredentialsMethodValues>,
  /// singleSignOnId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :singleSignOnId
  #[sdk(attr(qname = ":singleSignOnId"))]
  pub single_sign_on_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_DbPr/x:dbPr"))]
  pub database_properties: Option<DatabaseProperties>,
  /// _
  #[sdk(child(qname = "x:CT_OlapPr/x:olapPr"))]
  pub olap_properties: Option<OlapProperties>,
  /// _
  #[sdk(child(qname = "x:CT_WebPr/x:webPr"))]
  pub web_query_properties: Option<std::boxed::Box<WebQueryProperties>>,
  /// _
  #[sdk(child(qname = "x:CT_TextPr/x:textPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// _
  #[sdk(child(qname = "x:CT_Parameters/x:parameters"))]
  pub parameters: Option<Parameters>,
  /// _
  #[sdk(child(qname = "x:CT_ConnectionExtensionList/x:extLst"))]
  pub connection_extension_list: Option<ConnectionExtensionList>,
}
/// Tables.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tables.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tables/x:tables")]
pub struct Tables {
  /// Count of Tables
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Vec<TablesChoice>,
}
/// Parameter Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:parameter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Parameter/x:parameter")]
pub struct Parameter {
  /// Parameter Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// SQL Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqlType
  #[sdk(attr(qname = ":sqlType"))]
  pub sql_type: Option<crate::simple_type::Int32Value>,
  /// Parameter Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :parameterType
  #[sdk(attr(qname = ":parameterType"))]
  pub parameter_type: Option<ParameterValues>,
  /// Refresh on Change
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshOnChange
  #[sdk(attr(qname = ":refreshOnChange"))]
  pub refresh_on_change: Option<crate::simple_type::BooleanValue>,
  /// Parameter Prompt String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  /// Boolean
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :boolean
  #[sdk(attr(qname = ":boolean"))]
  pub boolean: Option<crate::simple_type::BooleanValue>,
  /// Double
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :double
  #[sdk(attr(qname = ":double"))]
  pub double: Option<crate::simple_type::DoubleValue>,
  /// Integer
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :integer
  #[sdk(attr(qname = ":integer"))]
  pub integer: Option<crate::simple_type::Int32Value>,
  /// String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :string
  #[sdk(attr(qname = ":string"))]
  pub string: Option<crate::simple_type::StringValue>,
  /// Cell Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cell
  #[sdk(attr(qname = ":cell"))]
  pub cell: Option<crate::simple_type::StringValue>,
}
/// No Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:m.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableMissing/x:m")]
pub struct MissingTable {}
/// No Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w14:no.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:CT_Empty/w14:no")]
pub struct No {}
/// Character Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:s.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XStringElement/x:s")]
pub struct CharacterValue {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
}
/// Index.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:x.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Index/x:x")]
pub struct FieldItem {
  /// Shared Items Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Text Import Field Settings.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:textField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextField/x:textField")]
pub struct TextField {
  /// Field Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<ExternalConnectionValues>,
  /// Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<crate::simple_type::UInt32Value>,
}
/// PivotCache Field.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cacheField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheField/x:cacheField")]
pub struct CacheField {
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// propertyName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :propertyName
  #[sdk(attr(qname = ":propertyName"))]
  pub property_name: Option<crate::simple_type::StringValue>,
  /// serverField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverField
  #[sdk(attr(qname = ":serverField"))]
  pub server_field: Option<crate::simple_type::BooleanValue>,
  /// uniqueList
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueList
  #[sdk(attr(qname = ":uniqueList"))]
  pub unique_list: Option<crate::simple_type::BooleanValue>,
  /// numFmtId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// sqlType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqlType
  #[sdk(attr(qname = ":sqlType"))]
  pub sql_type: Option<crate::simple_type::Int32Value>,
  /// hierarchy
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: Option<crate::simple_type::Int32Value>,
  /// level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// databaseField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :databaseField
  #[sdk(attr(qname = ":databaseField"))]
  pub database_field: Option<crate::simple_type::BooleanValue>,
  /// mappingCount
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mappingCount
  #[sdk(attr(qname = ":mappingCount"))]
  pub mapping_count: Option<crate::simple_type::UInt32Value>,
  /// memberPropertyField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :memberPropertyField
  #[sdk(attr(qname = ":memberPropertyField"))]
  pub member_property_field: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_SharedItems/x:sharedItems"))]
  pub shared_items: Option<SharedItems>,
  /// _
  #[sdk(child(qname = "x:CT_FieldGroup/x:fieldGroup"))]
  pub field_group: Option<std::boxed::Box<FieldGroup>>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:mpMap"))]
  pub x_mp_map: Vec<MemberPropertiesMap>,
  /// _
  #[sdk(child(qname = "x:CT_CacheFieldExtensionList/x:extLst"))]
  pub x_ext_lst: Option<CacheFieldExtensionList>,
}
/// Page Item Values.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pages.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Pages/x:pages")]
pub struct Pages {
  /// Page Item String Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PCDSCPage/x:page"))]
  pub x_page: Vec<Page>,
}
/// Range Sets.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rangeSets.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangeSets/x:rangeSets")]
pub struct RangeSets {
  /// Reference and Page Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_RangeSet/x:rangeSet"))]
  pub x_range_set: Vec<RangeSet>,
}
/// Page Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:page.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDSCPage/x:page")]
pub struct Page {
  /// Page Item String Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PageItem/x:pageItem"))]
  pub x_page_item: Vec<PageItem>,
}
/// Page Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageItem/x:pageItem")]
pub struct PageItem {
  /// Page Item Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Range Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rangeSet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangeSet/x:rangeSet")]
pub struct RangeSet {
  /// Field Item Index Page 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i1
  #[sdk(attr(qname = ":i1"))]
  pub field_item_index_page1: Option<crate::simple_type::UInt32Value>,
  /// Field Item Index Page 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i2
  #[sdk(attr(qname = ":i2"))]
  pub field_item_index_page2: Option<crate::simple_type::UInt32Value>,
  /// Field Item index Page 3
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i3
  #[sdk(attr(qname = ":i3"))]
  pub field_item_index_page3: Option<crate::simple_type::UInt32Value>,
  /// Field Item Index Page 4
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i4
  #[sdk(attr(qname = ":i4"))]
  pub field_item_index_page4: Option<crate::simple_type::UInt32Value>,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheet
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// No Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:m.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Missing/x:m")]
pub struct MissingItem {
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// background Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :un
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Numeric.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:n.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Number/x:n")]
pub struct NumberItem {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::DoubleValue,
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Background Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :un
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Boolean.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:b.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Boolean/x:b")]
pub struct BooleanItem {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::BooleanValue,
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Error Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:e.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Error/x:e")]
pub struct ErrorItem {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Item Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// background Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :un
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  ///Tuples
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub tuples: Option<Tuples>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Character Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:s.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_String/x:s")]
pub struct StringItem {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::StringValue,
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Item Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// Format Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Background Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Foreground Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Italic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Underline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :un
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Strikethrough
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Bold
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Date Time.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:d.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DateTime/x:d")]
pub struct DateTimeItem {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::DateTimeValue,
  /// Unused Item
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unused: Option<crate::simple_type::BooleanValue>,
  /// Calculated Item Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// Member Property Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cp
  #[sdk(attr(qname = ":cp"))]
  pub property_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Tuples.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tpls.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuples/x:tpls")]
pub struct Tuples {
  /// Member Name Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub member_name_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Tuple/x:tpl"))]
  pub x_tpl: Vec<Tuple>,
}
/// Sort By Tuple.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sortByTuple.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuples/x:sortByTuple")]
pub struct SortByTuple {
  /// Member Name Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub member_name_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Tuple/x:tpl"))]
  pub x_tpl: Vec<Tuple>,
}
/// Defines the TuplesType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuples/")]
pub struct TuplesType {
  /// Member Name Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub member_name_count: Option<crate::simple_type::UInt32Value>,
  ///Tuple.
  #[sdk(child(qname = "x:CT_Tuple/x:tpl"))]
  pub tuple: Vec<Tuple>,
}
/// Member Property Indexes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:x.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_X/x:x")]
pub struct MemberPropertyIndex {
  /// Shared Items Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// Defines the MemberPropertiesMap Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mpMap.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_X/x:mpMap")]
pub struct MemberPropertiesMap {
  /// Shared Items Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// Defines the XType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_X/")]
pub struct XType {
  /// Shared Items Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::Int32Value>,
}
/// PivotCache Record.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:r.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Record/x:r")]
pub struct PivotCacheRecord {
  #[sdk(choice)]
  pub xml_children: Vec<PivotCacheRecordChoice>,
}
/// OLAP KPI.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:kpi.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDKPI/x:kpi")]
pub struct Kpi {
  /// KPI Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// KPI Display Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// KPI Display Folder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :displayFolder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// KPI Measure Group Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measureGroup
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// Parent KPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :parent
  #[sdk(attr(qname = ":parent"))]
  pub parent_kpi: Option<crate::simple_type::StringValue>,
  /// KPI Value Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::StringValue,
  /// KPI Goal Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :goal
  #[sdk(attr(qname = ":goal"))]
  pub goal: Option<crate::simple_type::StringValue>,
  /// KPI Status Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :status
  #[sdk(attr(qname = ":status"))]
  pub status: Option<crate::simple_type::StringValue>,
  /// KPI Trend Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :trend
  #[sdk(attr(qname = ":trend"))]
  pub trend: Option<crate::simple_type::StringValue>,
  /// KPI Weight Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :weight
  #[sdk(attr(qname = ":weight"))]
  pub weight: Option<crate::simple_type::StringValue>,
}
/// PivotCache Field Id.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fieldUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldUsage/x:fieldUsage")]
pub struct FieldUsage {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::Int32Value,
}
/// OLAP Grouping Levels.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groupLevel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupLevel/x:groupLevel")]
pub struct GroupLevel {
  /// Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Grouping Level Display Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// User-Defined Group Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :user
  #[sdk(attr(qname = ":user"))]
  pub user: Option<crate::simple_type::BooleanValue>,
  /// Custom Roll Up
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customRollUp
  #[sdk(attr(qname = ":customRollUp"))]
  pub custom_roll_up: Option<crate::simple_type::BooleanValue>,
  ///OLAP Level Groups
  #[sdk(child(qname = "x:CT_Groups/x:groups"))]
  pub groups: Option<Groups>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// OLAP Level Groups.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groups.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Groups/x:groups")]
pub struct Groups {
  /// Level Group Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_LevelGroup/x:group"))]
  pub x_group: Vec<Group>,
}
/// OLAP Group.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:group.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LevelGroup/x:group")]
pub struct Group {
  /// Group Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Unique Group Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Group Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
  /// Parent Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueParent
  #[sdk(attr(qname = ":uniqueParent"))]
  pub unique_parent: Option<crate::simple_type::StringValue>,
  /// Group Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::Int32Value>,
  ///OLAP Group Members
  #[sdk(child(qname = "x:CT_GroupMembers/x:groupMembers"))]
  pub group_members: std::boxed::Box<GroupMembers>,
}
/// OLAP Group Members.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groupMembers.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupMembers/x:groupMembers")]
pub struct GroupMembers {
  /// Group Member Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_GroupMember/x:groupMember"))]
  pub x_group_member: Vec<GroupMember>,
}
/// OLAP Group Member.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groupMember.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupMember/x:groupMember")]
pub struct GroupMember {
  /// Group Member Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Group
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :group
  #[sdk(attr(qname = ":group"))]
  pub group: Option<crate::simple_type::BooleanValue>,
}
/// Entries.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:entries.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDSDTCEntries/x:entries")]
pub struct Entries {
  /// Tuple Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Vec<EntriesChoice>,
}
/// Sets.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sets.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sets/x:sets")]
pub struct Sets {
  /// Tuple Set Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Set/x:set"))]
  pub x_set: Vec<TupleSet>,
}
/// OLAP Query Cache.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryCache.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryCache/x:queryCache")]
pub struct QueryCache {
  /// Cached Query Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Query/x:query"))]
  pub x_query: Vec<Query>,
}
/// Server Formats.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:serverFormats.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ServerFormats/x:serverFormats")]
pub struct ServerFormats {
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_ServerFormat/x:serverFormat"))]
  pub x_server_format: Vec<ServerFormat>,
}
/// Server Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:serverFormat.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ServerFormat/x:serverFormat")]
pub struct ServerFormat {
  /// Culture
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Tuple.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tpl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Tuple/x:tpl")]
pub struct Tuple {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fld
  #[sdk(attr(qname = ":fld"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Hierarchy Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hier
  #[sdk(attr(qname = ":hier"))]
  pub hierarchy: Option<crate::simple_type::UInt32Value>,
  /// Item Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :item
  #[sdk(attr(qname = ":item"))]
  pub item: crate::simple_type::UInt32Value,
}
/// OLAP Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:set.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Set/x:set")]
pub struct TupleSet {
  /// Number of Tuples
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Maximum Rank Requested
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxRank
  #[sdk(attr(qname = ":maxRank"))]
  pub max_rank: crate::simple_type::Int32Value,
  /// MDX Set Definition
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :setDefinition
  #[sdk(attr(qname = ":setDefinition"))]
  pub set_definition: crate::simple_type::StringValue,
  /// Set Sort Order
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sortType
  #[sdk(attr(qname = ":sortType"))]
  pub sort_type: Option<SortValues>,
  /// Query Failed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :queryFailed
  #[sdk(attr(qname = ":queryFailed"))]
  pub query_failed: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub x_tpls: Vec<Tuples>,
  /// _
  #[sdk(child(qname = "x:CT_Tuples/x:sortByTuple"))]
  pub x_sort_by_tuple: Option<SortByTuple>,
}
/// Query.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:query.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Query/x:query")]
pub struct Query {
  /// MDX Query String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mdx
  #[sdk(attr(qname = ":mdx"))]
  pub mdx: crate::simple_type::StringValue,
  ///Tuples
  #[sdk(child(qname = "x:CT_Tuples/x:tpls"))]
  pub tuples: Option<Tuples>,
}
/// Calculated Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calculatedItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedItem/x:calculatedItem")]
pub struct CalculatedItem {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :field
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Calculated Item Formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::StringValue>,
  ///Calculated Item Location
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Item Location.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotArea.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotArea/x:pivotArea")]
pub struct PivotArea {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :field
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::Int32Value>,
  /// Rule Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<PivotAreaValues>,
  /// Data Only
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataOnly
  #[sdk(attr(qname = ":dataOnly"))]
  pub data_only: Option<crate::simple_type::BooleanValue>,
  /// Labels Only
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :labelOnly
  #[sdk(attr(qname = ":labelOnly"))]
  pub label_only: Option<crate::simple_type::BooleanValue>,
  /// Include Row Grand Total
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grandRow
  #[sdk(attr(qname = ":grandRow"))]
  pub grand_row: Option<crate::simple_type::BooleanValue>,
  /// Include Column Grand Total
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grandCol
  #[sdk(attr(qname = ":grandCol"))]
  pub grand_column: Option<crate::simple_type::BooleanValue>,
  /// Cache Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cacheIndex
  #[sdk(attr(qname = ":cacheIndex"))]
  pub cache_index: Option<crate::simple_type::BooleanValue>,
  /// Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Offset Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Collapsed Levels Are Subtotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :collapsedLevelsAreSubtotals
  #[sdk(attr(qname = ":collapsedLevelsAreSubtotals"))]
  pub collapsed_levels_are_subtotals: Option<crate::simple_type::BooleanValue>,
  /// Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Field Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldPosition
  #[sdk(attr(qname = ":fieldPosition"))]
  pub field_position: Option<crate::simple_type::UInt32Value>,
  ///References
  #[sdk(child(qname = "x:CT_PivotAreaReferences/x:references"))]
  pub pivot_area_references: Option<PivotAreaReferences>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Member.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calculatedMember.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMember/x:calculatedMember")]
pub struct CalculatedMember {
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// mdx
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mdx
  #[sdk(attr(qname = ":mdx"))]
  pub mdx: crate::simple_type::StringValue,
  /// memberName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :memberName
  #[sdk(attr(qname = ":memberName"))]
  pub member_name: Option<crate::simple_type::StringValue>,
  /// hierarchy
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: Option<crate::simple_type::StringValue>,
  /// parent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :parent
  #[sdk(attr(qname = ":parent"))]
  pub parent_name: Option<crate::simple_type::StringValue>,
  /// solveOrder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :solveOrder
  #[sdk(attr(qname = ":solveOrder"))]
  pub solve_order: Option<crate::simple_type::Int32Value>,
  /// set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :set
  #[sdk(attr(qname = ":set"))]
  pub set: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMemberExtensionList/x:extLst"))]
  pub calculated_member_extension_list: Option<CalculatedMemberExtensionList>,
}
/// PivotTable Field.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotField/x:pivotField")]
pub struct PivotField {
  /// Field Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Data Field
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataField
  #[sdk(attr(qname = ":dataField"))]
  pub data_field: Option<crate::simple_type::BooleanValue>,
  /// Custom Subtotal Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalCaption
  #[sdk(attr(qname = ":subtotalCaption"))]
  pub subtotal_caption: Option<crate::simple_type::StringValue>,
  /// Show PivotField Header Drop Downs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDropDowns
  #[sdk(attr(qname = ":showDropDowns"))]
  pub show_drop_downs: Option<crate::simple_type::BooleanValue>,
  /// Hidden Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenLevel
  #[sdk(attr(qname = ":hiddenLevel"))]
  pub hidden_level: Option<crate::simple_type::BooleanValue>,
  /// Unique Member Property
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueMemberProperty
  #[sdk(attr(qname = ":uniqueMemberProperty"))]
  pub unique_member_property: Option<crate::simple_type::StringValue>,
  /// Compact
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compact
  #[sdk(attr(qname = ":compact"))]
  pub compact: Option<crate::simple_type::BooleanValue>,
  /// All Items Expanded
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allDrilled
  #[sdk(attr(qname = ":allDrilled"))]
  pub all_drilled: Option<crate::simple_type::BooleanValue>,
  /// Number Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Outline Items
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Subtotals At Top
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalTop
  #[sdk(attr(qname = ":subtotalTop"))]
  pub subtotal_top: Option<crate::simple_type::BooleanValue>,
  /// Drag To Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToRow
  #[sdk(attr(qname = ":dragToRow"))]
  pub drag_to_row: Option<crate::simple_type::BooleanValue>,
  /// Drag To Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToCol
  #[sdk(attr(qname = ":dragToCol"))]
  pub drag_to_column: Option<crate::simple_type::BooleanValue>,
  /// Multiple Field Filters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :multipleItemSelectionAllowed
  #[sdk(attr(qname = ":multipleItemSelectionAllowed"))]
  pub multiple_item_selection_allowed: Option<crate::simple_type::BooleanValue>,
  /// Drag Field to Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToPage
  #[sdk(attr(qname = ":dragToPage"))]
  pub drag_to_page: Option<crate::simple_type::BooleanValue>,
  /// Field Can Drag to Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToData
  #[sdk(attr(qname = ":dragToData"))]
  pub drag_to_data: Option<crate::simple_type::BooleanValue>,
  /// Drag Off
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragOff
  #[sdk(attr(qname = ":dragOff"))]
  pub drag_off: Option<crate::simple_type::BooleanValue>,
  /// Show All Items
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAll
  #[sdk(attr(qname = ":showAll"))]
  pub show_all: Option<crate::simple_type::BooleanValue>,
  /// Insert Blank Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertBlankRow
  #[sdk(attr(qname = ":insertBlankRow"))]
  pub insert_blank_row: Option<crate::simple_type::BooleanValue>,
  /// Server-based Page Field
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverField
  #[sdk(attr(qname = ":serverField"))]
  pub server_field: Option<crate::simple_type::BooleanValue>,
  /// Insert Item Page Break
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertPageBreak
  #[sdk(attr(qname = ":insertPageBreak"))]
  pub insert_page_break: Option<crate::simple_type::BooleanValue>,
  /// Auto Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoShow
  #[sdk(attr(qname = ":autoShow"))]
  pub auto_show: Option<crate::simple_type::BooleanValue>,
  /// Top Auto Show
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :topAutoShow
  #[sdk(attr(qname = ":topAutoShow"))]
  pub top_auto_show: Option<crate::simple_type::BooleanValue>,
  /// Hide New Items
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hideNewItems
  #[sdk(attr(qname = ":hideNewItems"))]
  pub hide_new_items: Option<crate::simple_type::BooleanValue>,
  /// Measure Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measureFilter
  #[sdk(attr(qname = ":measureFilter"))]
  pub measure_filter: Option<crate::simple_type::BooleanValue>,
  /// Inclusive Manual Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :includeNewItemsInFilter
  #[sdk(attr(qname = ":includeNewItemsInFilter"))]
  pub include_new_items_in_filter: Option<crate::simple_type::BooleanValue>,
  /// Items Per Page Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :itemPageCount
  #[sdk(attr(qname = ":itemPageCount"))]
  pub item_page_count: Option<crate::simple_type::UInt32Value>,
  /// Auto Sort Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sortType
  #[sdk(attr(qname = ":sortType"))]
  pub sort_type: Option<FieldSortValues>,
  /// Data Source Sort
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataSourceSort
  #[sdk(attr(qname = ":dataSourceSort"))]
  pub data_source_sort: Option<crate::simple_type::BooleanValue>,
  /// Auto Sort
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nonAutoSortDefault
  #[sdk(attr(qname = ":nonAutoSortDefault"))]
  pub non_auto_sort_default: Option<crate::simple_type::BooleanValue>,
  /// Auto Show Rank By
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rankBy
  #[sdk(attr(qname = ":rankBy"))]
  pub rank_by: Option<crate::simple_type::UInt32Value>,
  /// Show Default Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSubtotal
  #[sdk(attr(qname = ":defaultSubtotal"))]
  pub default_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Sum Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sumSubtotal
  #[sdk(attr(qname = ":sumSubtotal"))]
  pub sum_subtotal: Option<crate::simple_type::BooleanValue>,
  /// CountA
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :countASubtotal
  #[sdk(attr(qname = ":countASubtotal"))]
  pub count_a_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Average
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :avgSubtotal
  #[sdk(attr(qname = ":avgSubtotal"))]
  pub average_sub_total: Option<crate::simple_type::BooleanValue>,
  /// Max Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxSubtotal
  #[sdk(attr(qname = ":maxSubtotal"))]
  pub max_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Min Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minSubtotal
  #[sdk(attr(qname = ":minSubtotal"))]
  pub min_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Product Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :productSubtotal
  #[sdk(attr(qname = ":productSubtotal"))]
  pub apply_product_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :countSubtotal
  #[sdk(attr(qname = ":countSubtotal"))]
  pub count_subtotal: Option<crate::simple_type::BooleanValue>,
  /// StdDev Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stdDevSubtotal
  #[sdk(attr(qname = ":stdDevSubtotal"))]
  pub apply_standard_deviation_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// StdDevP Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stdDevPSubtotal
  #[sdk(attr(qname = ":stdDevPSubtotal"))]
  pub apply_standard_deviation_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Variance Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :varSubtotal
  #[sdk(attr(qname = ":varSubtotal"))]
  pub apply_variance_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// VarP Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :varPSubtotal
  #[sdk(attr(qname = ":varPSubtotal"))]
  pub apply_variance_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Show Member Property in Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showPropCell
  #[sdk(attr(qname = ":showPropCell"))]
  pub show_prop_cell: Option<crate::simple_type::BooleanValue>,
  /// Show Member Property ToolTip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showPropTip
  #[sdk(attr(qname = ":showPropTip"))]
  pub show_property_tooltip: Option<crate::simple_type::BooleanValue>,
  /// Show As Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showPropAsCaption
  #[sdk(attr(qname = ":showPropAsCaption"))]
  pub show_prop_as_caption: Option<crate::simple_type::BooleanValue>,
  /// Drill State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultAttributeDrillState
  #[sdk(attr(qname = ":defaultAttributeDrillState"))]
  pub default_attribute_drill_state: Option<crate::simple_type::BooleanValue>,
  ///Field Items
  #[sdk(child(qname = "x:CT_Items/x:items"))]
  pub items: Option<Items>,
  ///AutoSort Scope
  #[sdk(child(qname = "x:CT_AutoSortScope/x:autoSortScope"))]
  pub auto_sort_scope: Option<std::boxed::Box<AutoSortScope>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_PivotFieldExtensionList/x:extLst"))]
  pub pivot_field_extension_list: Option<PivotFieldExtensionList>,
}
/// PivotTable Field Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:item.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Item/x:item")]
pub struct Item {
  /// Item User Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub item_name: Option<crate::simple_type::StringValue>,
  /// Item Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub item_type: Option<ItemValues>,
  /// Hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :h
  #[sdk(attr(qname = ":h"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Character
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub has_string_vlue: Option<crate::simple_type::BooleanValue>,
  /// Hide Details
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sd
  #[sdk(attr(qname = ":sd"))]
  pub hide_details: Option<crate::simple_type::BooleanValue>,
  /// Calculated Member
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub calculated: Option<crate::simple_type::BooleanValue>,
  /// Missing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :m
  #[sdk(attr(qname = ":m"))]
  pub missing: Option<crate::simple_type::BooleanValue>,
  /// Child Items
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub child_items: Option<crate::simple_type::BooleanValue>,
  /// Item Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// Expanded
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub expanded: Option<crate::simple_type::BooleanValue>,
  /// Drill Across Attributes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :e
  #[sdk(attr(qname = ":e"))]
  pub drill_across_attributes: Option<crate::simple_type::BooleanValue>,
}
/// Data Field Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataField/x:dataField")]
pub struct DataField {
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// fld
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fld
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::UInt32Value,
  /// subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :subtotal
  #[sdk(attr(qname = ":subtotal"))]
  pub subtotal: Option<DataConsolidateFunctionValues>,
  /// showDataAs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDataAs
  #[sdk(attr(qname = ":showDataAs"))]
  pub show_data_as: Option<ShowDataAsValues>,
  /// baseField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseField
  #[sdk(attr(qname = ":baseField"))]
  pub base_field: Option<crate::simple_type::Int32Value>,
  /// baseItem
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseItem
  #[sdk(attr(qname = ":baseItem"))]
  pub base_item: Option<crate::simple_type::UInt32Value>,
  /// numFmtId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DataFieldExtensionList/x:extLst"))]
  pub data_field_extension_list: Option<DataFieldExtensionList>,
}
/// Row Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:i.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_I/x:i")]
pub struct RowItem {
  /// Item Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub item_type: Option<ItemValues>,
  /// Repeated Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub repeated_item_count: Option<crate::simple_type::UInt32Value>,
  /// Data Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub index: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_X/x:x"))]
  pub x_x: Vec<MemberPropertyIndex>,
}
/// Row Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:field.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Field/x:field")]
pub struct Field {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::Int32Value,
}
/// PivotTable Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:format.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Format/x:format")]
pub struct Format {
  /// Format Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<FormatActionValues>,
  /// Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  ///Pivot Table Location
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Conditional Formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormat.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormat/x:conditionalFormat")]
pub struct ConditionalFormat {
  /// Conditional Formatting Scope
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scope
  #[sdk(attr(qname = ":scope"))]
  pub scope: Option<ScopeValues>,
  /// Conditional Formatting Rule Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<RuleValues>,
  /// Priority
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: crate::simple_type::UInt32Value,
  ///Pivot Areas
  #[sdk(child(qname = "x:CT_PivotAreas/x:pivotAreas"))]
  pub pivot_areas: std::boxed::Box<PivotAreas>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Pivot Areas.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotAreas.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreas/x:pivotAreas")]
pub struct PivotAreas {
  /// Pivot Area Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub x_pivot_area: Vec<PivotArea>,
}
/// PivotChart Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:chartFormat.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartFormat/x:chartFormat")]
pub struct ChartFormat {
  /// Chart Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :chart
  #[sdk(attr(qname = ":chart"))]
  pub chart: crate::simple_type::UInt32Value,
  /// Pivot Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  pub format: crate::simple_type::UInt32Value,
  /// Series Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :series
  #[sdk(attr(qname = ":series"))]
  pub series: Option<crate::simple_type::BooleanValue>,
  ///Pivot Table Location Rule
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// OLAP Hierarchy.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotHierarchy.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchy/x:pivotHierarchy")]
pub struct PivotHierarchy {
  /// Outline New Levels
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Multiple Field Filters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :multipleItemSelectionAllowed
  #[sdk(attr(qname = ":multipleItemSelectionAllowed"))]
  pub multiple_item_selection_allowed: Option<crate::simple_type::BooleanValue>,
  /// New Levels Subtotals At Top
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :subtotalTop
  #[sdk(attr(qname = ":subtotalTop"))]
  pub subtotal_top: Option<crate::simple_type::BooleanValue>,
  /// Show In Field List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showInFieldList
  #[sdk(attr(qname = ":showInFieldList"))]
  pub show_in_field_list: Option<crate::simple_type::BooleanValue>,
  /// Drag To Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToRow
  #[sdk(attr(qname = ":dragToRow"))]
  pub drag_to_row: Option<crate::simple_type::BooleanValue>,
  /// Drag To Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToCol
  #[sdk(attr(qname = ":dragToCol"))]
  pub drag_to_column: Option<crate::simple_type::BooleanValue>,
  /// Drag to Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToPage
  #[sdk(attr(qname = ":dragToPage"))]
  pub drag_to_page: Option<crate::simple_type::BooleanValue>,
  /// Drag To Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragToData
  #[sdk(attr(qname = ":dragToData"))]
  pub drag_to_data: Option<crate::simple_type::BooleanValue>,
  /// Drag Off
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dragOff
  #[sdk(attr(qname = ":dragOff"))]
  pub drag_off: Option<crate::simple_type::BooleanValue>,
  /// Inclusive Manual Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :includeNewItemsInFilter
  #[sdk(attr(qname = ":includeNewItemsInFilter"))]
  pub include_new_items_in_filter: Option<crate::simple_type::BooleanValue>,
  /// Hierarchy Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  ///OLAP Member Properties
  #[sdk(child(qname = "x:CT_MemberProperties/x:mps"))]
  pub member_properties: Option<MemberProperties>,
  /// _
  #[sdk(child(qname = "x:CT_Members/x:members"))]
  pub x_members: Vec<Members>,
  /// _
  #[sdk(child(qname = "x:CT_PivotHierarchyExtensionList/x:extLst"))]
  pub x_ext_lst: Option<PivotHierarchyExtensionList>,
}
/// Row OLAP Hierarchies.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rowHierarchyUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HierarchyUsage/x:rowHierarchyUsage")]
pub struct RowHierarchyUsage {
  /// Hierarchy Usage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchyUsage
  #[sdk(attr(qname = ":hierarchyUsage"))]
  pub value: crate::simple_type::Int32Value,
}
/// Column OLAP Hierarchies.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colHierarchyUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HierarchyUsage/x:colHierarchyUsage")]
pub struct ColumnHierarchyUsage {
  /// Hierarchy Usage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchyUsage
  #[sdk(attr(qname = ":hierarchyUsage"))]
  pub value: crate::simple_type::Int32Value,
}
/// Defines the HierarchyUsageType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HierarchyUsage/")]
pub struct HierarchyUsageType {
  /// Hierarchy Usage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchyUsage
  #[sdk(attr(qname = ":hierarchyUsage"))]
  pub value: crate::simple_type::Int32Value,
}
/// OLAP Member Property.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MemberProperty/x:mp")]
pub struct MemberProperty {
  /// OLAP Member Property Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showCell
  #[sdk(attr(qname = ":showCell"))]
  pub show_cell: Option<crate::simple_type::BooleanValue>,
  /// Show Tooltip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showTip
  #[sdk(attr(qname = ":showTip"))]
  pub show_tip: Option<crate::simple_type::BooleanValue>,
  /// Show As Caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAsCaption
  #[sdk(attr(qname = ":showAsCaption"))]
  pub show_as_caption: Option<crate::simple_type::BooleanValue>,
  /// Name Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nameLen
  #[sdk(attr(qname = ":nameLen"))]
  pub name_length: Option<crate::simple_type::UInt32Value>,
  /// Property Name Character Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pPos
  #[sdk(attr(qname = ":pPos"))]
  pub property_name_position: Option<crate::simple_type::UInt32Value>,
  /// Property Name Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pLen
  #[sdk(attr(qname = ":pLen"))]
  pub property_name_length: Option<crate::simple_type::UInt32Value>,
  /// Level Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :field
  #[sdk(attr(qname = ":field"))]
  pub field: crate::simple_type::UInt32Value,
}
/// Member.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:member.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Member/x:member")]
pub struct Member {
  /// Hidden Item Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// OLAP Dimension.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dimension.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotDimension/x:dimension")]
pub struct Dimension {
  /// Measure
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measure
  #[sdk(attr(qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
  /// Dimension Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Dimension Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// Dimension Display Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
}
/// OLAP Measure Group.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:measureGroup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureGroup/x:measureGroup")]
pub struct MeasureGroup {
  /// Measure Group Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Measure Group Display Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: crate::simple_type::StringValue,
}
/// OLAP Measure Group.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:map.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureDimensionMap/x:map")]
pub struct MeasureDimensionMap {
  /// Measure Group Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measureGroup
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: crate::simple_type::UInt32Value,
  /// Dimension Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dimension
  #[sdk(attr(qname = ":dimension"))]
  pub dimension: crate::simple_type::UInt32Value,
}
/// PivotTable Advanced Filter.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:filter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilter/x:filter")]
pub struct PivotFilter {
  /// fld
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fld
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::UInt32Value,
  /// mpFld
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mpFld
  #[sdk(attr(qname = ":mpFld"))]
  pub member_property_field_id: Option<crate::simple_type::UInt32Value>,
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: PivotFilterValues,
  /// evalOrder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evalOrder
  #[sdk(attr(qname = ":evalOrder"))]
  pub evaluation_order: Option<crate::simple_type::Int32Value>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// iMeasureHier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iMeasureHier
  #[sdk(attr(qname = ":iMeasureHier"))]
  pub measure_hierarchy: Option<crate::simple_type::UInt32Value>,
  /// iMeasureFld
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iMeasureFld
  #[sdk(attr(qname = ":iMeasureFld"))]
  pub measure_field: Option<crate::simple_type::UInt32Value>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// stringValue1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stringValue1
  #[sdk(attr(qname = ":stringValue1"))]
  pub string_value1: Option<crate::simple_type::StringValue>,
  /// stringValue2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stringValue2
  #[sdk(attr(qname = ":stringValue2"))]
  pub string_value2: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: std::boxed::Box<AutoFilter>,
  /// _
  #[sdk(child(qname = "x:CT_PivotFilterExtensionList/x:extLst"))]
  pub pivot_filter_extension_list: Option<PivotFilterExtensionList>,
}
/// PivotCache Hierarchy.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cacheHierarchy.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchy/x:cacheHierarchy")]
pub struct CacheHierarchy {
  /// uniqueName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// caption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// measure
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measure
  #[sdk(attr(qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
  /// set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :set
  #[sdk(attr(qname = ":set"))]
  pub set: Option<crate::simple_type::BooleanValue>,
  /// parentSet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :parentSet
  #[sdk(attr(qname = ":parentSet"))]
  pub parent_set: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<crate::simple_type::Int32Value>,
  /// attribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :attribute
  #[sdk(attr(qname = ":attribute"))]
  pub attribute: Option<crate::simple_type::BooleanValue>,
  /// time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: Option<crate::simple_type::BooleanValue>,
  /// keyAttribute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :keyAttribute
  #[sdk(attr(qname = ":keyAttribute"))]
  pub key_attribute: Option<crate::simple_type::BooleanValue>,
  /// defaultMemberUniqueName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultMemberUniqueName
  #[sdk(attr(qname = ":defaultMemberUniqueName"))]
  pub default_member_unique_name: Option<crate::simple_type::StringValue>,
  /// allUniqueName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allUniqueName
  #[sdk(attr(qname = ":allUniqueName"))]
  pub all_unique_name: Option<crate::simple_type::StringValue>,
  /// allCaption
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allCaption
  #[sdk(attr(qname = ":allCaption"))]
  pub all_caption: Option<crate::simple_type::StringValue>,
  /// dimensionUniqueName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dimensionUniqueName
  #[sdk(attr(qname = ":dimensionUniqueName"))]
  pub dimension_unique_name: Option<crate::simple_type::StringValue>,
  /// displayFolder
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :displayFolder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// measureGroup
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measureGroup
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// measures
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :measures
  #[sdk(attr(qname = ":measures"))]
  pub measures: Option<crate::simple_type::BooleanValue>,
  /// count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// oneField
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oneField
  #[sdk(attr(qname = ":oneField"))]
  pub one_field: Option<crate::simple_type::BooleanValue>,
  /// memberValueDatatype
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :memberValueDatatype
  #[sdk(attr(qname = ":memberValueDatatype"))]
  pub member_value_datatype: Option<crate::simple_type::UInt16Value>,
  /// unbalanced
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :unbalanced
  #[sdk(attr(qname = ":unbalanced"))]
  pub unbalanced: Option<crate::simple_type::BooleanValue>,
  /// unbalancedGroup
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :unbalancedGroup
  #[sdk(attr(qname = ":unbalancedGroup"))]
  pub unbalanced_group: Option<crate::simple_type::BooleanValue>,
  /// hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_FieldsUsage/x:fieldsUsage"))]
  pub fields_usage: Option<FieldsUsage>,
  /// _
  #[sdk(child(qname = "x:CT_GroupLevels/x:groupLevels"))]
  pub group_levels: Option<GroupLevels>,
  /// _
  #[sdk(child(qname = "x:CT_CacheHierarchyExtensionList/x:extLst"))]
  pub cache_hierarchy_extension_list: Option<CacheHierarchyExtensionList>,
}
/// Range Grouping Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rangePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RangePr/x:rangePr")]
pub struct RangeProperties {
  /// Source Data Set Beginning Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoStart
  #[sdk(attr(qname = ":autoStart"))]
  pub auto_start: Option<crate::simple_type::BooleanValue>,
  /// Source Data Ending Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoEnd
  #[sdk(attr(qname = ":autoEnd"))]
  pub auto_end: Option<crate::simple_type::BooleanValue>,
  /// Group By
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :groupBy
  #[sdk(attr(qname = ":groupBy"))]
  pub group_by: Option<GroupByValues>,
  /// Numeric Grouping Start Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startNum
  #[sdk(attr(qname = ":startNum"))]
  pub start_number: Option<crate::simple_type::DoubleValue>,
  /// Numeric Grouping End Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endNum
  #[sdk(attr(qname = ":endNum"))]
  pub end_num: Option<crate::simple_type::DoubleValue>,
  /// Date Grouping Start Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// Date Grouping End Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endDate
  #[sdk(attr(qname = ":endDate"))]
  pub end_date: Option<crate::simple_type::DateTimeValue>,
  /// Grouping Interval
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :groupInterval
  #[sdk(attr(qname = ":groupInterval"))]
  pub group_interval: Option<crate::simple_type::DoubleValue>,
}
/// Discrete Grouping Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:discretePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DiscretePr/x:discretePr")]
pub struct DiscreteProperties {
  /// Mapping Index Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  pub x_x: Vec<FieldItem>,
}
/// OLAP Group Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groupItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupItems/x:groupItems")]
pub struct GroupItems {
  /// Items Created Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Vec<GroupItemsChoice>,
}
/// Page Field.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageField/x:pageField")]
pub struct PageField {
  /// Field
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fld
  #[sdk(attr(qname = ":fld"))]
  pub field: crate::simple_type::Int32Value,
  /// Item Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :item
  #[sdk(attr(qname = ":item"))]
  pub item: Option<crate::simple_type::UInt32Value>,
  /// OLAP Hierarchy Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hier
  #[sdk(attr(qname = ":hier"))]
  pub hierarchy: Option<crate::simple_type::Int32Value>,
  /// Hierarchy Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Hierarchy Display Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cap
  #[sdk(attr(qname = ":cap"))]
  pub caption: Option<crate::simple_type::StringValue>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// References.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:references.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreaReferences/x:references")]
pub struct PivotAreaReferences {
  /// Pivot Filter Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotAreaReference/x:reference"))]
  pub x_reference: Vec<PivotAreaReference>,
}
/// Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:reference.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreaReference/x:reference")]
pub struct PivotAreaReference {
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :field
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::UInt32Value>,
  /// Item Index Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Selected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :selected
  #[sdk(attr(qname = ":selected"))]
  pub selected: Option<crate::simple_type::BooleanValue>,
  /// Positional Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :byPosition
  #[sdk(attr(qname = ":byPosition"))]
  pub by_position: Option<crate::simple_type::BooleanValue>,
  /// Relative Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relative
  #[sdk(attr(qname = ":relative"))]
  pub relative: Option<crate::simple_type::BooleanValue>,
  /// Include Default Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSubtotal
  #[sdk(attr(qname = ":defaultSubtotal"))]
  pub default_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Sum Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sumSubtotal
  #[sdk(attr(qname = ":sumSubtotal"))]
  pub sum_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include CountA Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :countASubtotal
  #[sdk(attr(qname = ":countASubtotal"))]
  pub count_a_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Average Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :avgSubtotal
  #[sdk(attr(qname = ":avgSubtotal"))]
  pub average_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Maximum Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxSubtotal
  #[sdk(attr(qname = ":maxSubtotal"))]
  pub max_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Minimum Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minSubtotal
  #[sdk(attr(qname = ":minSubtotal"))]
  pub min_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Product Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :productSubtotal
  #[sdk(attr(qname = ":productSubtotal"))]
  pub apply_product_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Count Subtotal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :countSubtotal
  #[sdk(attr(qname = ":countSubtotal"))]
  pub count_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include StdDev Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stdDevSubtotal
  #[sdk(attr(qname = ":stdDevSubtotal"))]
  pub apply_standard_deviation_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include StdDevP Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stdDevPSubtotal
  #[sdk(attr(qname = ":stdDevPSubtotal"))]
  pub apply_standard_deviation_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include Var Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :varSubtotal
  #[sdk(attr(qname = ":varSubtotal"))]
  pub apply_variance_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// Include VarP Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :varPSubtotal
  #[sdk(attr(qname = ":varPSubtotal"))]
  pub apply_variance_p_in_subtotal: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  pub x_x: Vec<FieldItem>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Query table fields.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryTableFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableFields/x:queryTableFields")]
pub struct QueryTableFields {
  /// Column Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_QueryTableField/x:queryTableField"))]
  pub x_query_table_field: Vec<QueryTableField>,
}
/// Deleted Fields.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryTableDeletedFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableDeletedFields/x:queryTableDeletedFields")]
pub struct QueryTableDeletedFields {
  /// Deleted Fields Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DeletedField/x:deletedField"))]
  pub x_deleted_field: Vec<DeletedField>,
}
/// Deleted Field.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:deletedField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DeletedField/x:deletedField")]
pub struct DeletedField {
  /// Deleted Fields Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// QueryTable Field.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryTableField.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableField/x:queryTableField")]
pub struct QueryTableField {
  /// Field Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Data Bound Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataBound
  #[sdk(attr(qname = ":dataBound"))]
  pub data_bound: Option<crate::simple_type::BooleanValue>,
  /// Row Numbers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowNumbers
  #[sdk(attr(qname = ":rowNumbers"))]
  pub row_numbers: Option<crate::simple_type::BooleanValue>,
  /// Fill This Formula On Refresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillFormulas
  #[sdk(attr(qname = ":fillFormulas"))]
  pub fill_formulas: Option<crate::simple_type::BooleanValue>,
  /// Clipped Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clipped
  #[sdk(attr(qname = ":clipped"))]
  pub clipped: Option<crate::simple_type::BooleanValue>,
  /// Table Column Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tableColumnId
  #[sdk(attr(qname = ":tableColumnId"))]
  pub table_column_id: Option<crate::simple_type::UInt32Value>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// String Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:si.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:si")]
pub struct SharedStringItem {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  /// Preserved Word 2010 extension attribute.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w14:attr
  #[sdk(attr(qname = "w14:attr"))]
  pub w14_attr: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "w14:CT_Rst/w14:placeholder"))]
  pub w14_placeholder: Option<std::boxed::Box<Placeholder>>,
  /// _
  #[sdk(child(qname = "w14:CT_Empty/w14:no"))]
  pub w14_no: Option<No>,
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// _
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// String Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is w14:placeholder.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w14:CT_Rst/w14:placeholder")]
pub struct Placeholder {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  /// Preserved Word 2010 extension attribute.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: w14:attr
  #[sdk(attr(qname = "w14:attr"))]
  pub w14_attr: Option<crate::simple_type::StringValue>,
  /// MCE process content directive.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// MCE preserve attributes directive.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "w14:CT_Rst/w14:placeholder"))]
  pub w14_placeholder: Option<std::boxed::Box<Placeholder>>,
  /// _
  #[sdk(child(qname = "w14:CT_Empty/w14:no"))]
  pub w14_no: Option<No>,
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// _
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Rich Text Inline.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:is.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:is")]
pub struct InlineString {
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// _
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Comment Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:text.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/x:text")]
pub struct CommentText {
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<Text>,
  /// _
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<Run>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<PhoneticRun>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr: Option<PhoneticProperties>,
}
/// Defines the RstType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/")]
pub struct RstType {
  #[sdk(choice)]
  pub xml_children: Vec<RstTypeChoice>,
}
/// Bold.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:b.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:b")]
pub struct Bold {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Italic.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:i.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:i")]
pub struct Italic {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Strike Through.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:strike.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:strike")]
pub struct Strike {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Condense.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:condense.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:condense")]
pub struct Condense {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Extend.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extend.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:extend")]
pub struct Extend {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Outline.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:outline.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:outline")]
pub struct Outline {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Shadow.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:shadow.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/x:shadow")]
pub struct Shadow {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the BooleanPropertyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BooleanProperty/")]
pub struct BooleanPropertyType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Underline.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:u.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_UnderlineProperty/x:u")]
pub struct Underline {
  /// Underline Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<UnderlineValues>,
}
/// Vertical Alignment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:vertAlign.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign")]
pub struct VerticalTextAlignment {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: VerticalAlignmentRunValues,
}
/// Font Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sz.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontSize/x:sz")]
pub struct FontSize {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Text Color.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:color.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:color")]
pub struct Color {
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Sheet Tab Color.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tabColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:tabColor")]
pub struct TabColor {
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Foreground Color.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fgColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:fgColor")]
pub struct ForegroundColor {
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Background Color.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:bgColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x:bgColor")]
pub struct BackgroundColor {
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the ColorType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/")]
pub struct ColorType {
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Font.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rFont.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontName/x:rFont")]
pub struct RunFont {
  /// String Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Font Family.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:family.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IntProperty/x:family")]
pub struct FontFamily {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Character Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:charset.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IntProperty/x:charset")]
pub struct RunPropertyCharSet {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the InternationalPropertyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IntProperty/")]
pub struct InternationalPropertyType {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::Int32Value,
}
/// Font Scheme.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:scheme.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontScheme/x:scheme")]
pub struct FontScheme {
  /// Font Scheme
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: FontSchemeValues,
}
/// Run Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RPrElt/x:rPr")]
pub struct RunProperties {
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:b"))]
  pub x_b: Vec<Bold>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:i"))]
  pub x_i: Vec<Italic>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:strike"))]
  pub x_strike: Vec<Strike>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:condense"))]
  pub x_condense: Vec<Condense>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:extend"))]
  pub x_extend: Vec<Extend>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:outline"))]
  pub x_outline: Vec<Outline>,
  /// _
  #[sdk(child(qname = "x:CT_BooleanProperty/x:shadow"))]
  pub x_shadow: Vec<Shadow>,
  /// _
  #[sdk(child(qname = "x:CT_UnderlineProperty/x:u"))]
  pub x_u: Vec<Underline>,
  /// _
  #[sdk(child(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign"))]
  pub x_vert_align: Vec<VerticalTextAlignment>,
  /// _
  #[sdk(child(qname = "x:CT_FontSize/x:sz"))]
  pub x_sz: Vec<FontSize>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
  /// _
  #[sdk(child(qname = "x:CT_FontName/x:rFont"))]
  pub x_r_font: Vec<RunFont>,
  /// _
  #[sdk(child(qname = "x:CT_IntProperty/x:family"))]
  pub x_family: Vec<FontFamily>,
  /// _
  #[sdk(child(qname = "x:CT_IntProperty/x:charset"))]
  pub x_charset: Vec<RunPropertyCharSet>,
  /// _
  #[sdk(child(qname = "x:CT_FontScheme/x:scheme"))]
  pub x_scheme: Vec<FontScheme>,
}
/// Rich Text Run.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:r.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RElt/x:r")]
pub struct Run {
  ///Run Properties
  #[sdk(child(qname = "x:CT_RPrElt/x:rPr"))]
  pub run_properties: Option<RunProperties>,
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: std::boxed::Box<Text>,
}
/// Phonetic Run.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rPh.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PhoneticRun/x:rPh")]
pub struct PhoneticRun {
  /// Base Text Start Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sb
  #[sdk(attr(qname = ":sb"))]
  pub base_text_start_index: crate::simple_type::UInt32Value,
  /// Base Text End Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :eb
  #[sdk(attr(qname = ":eb"))]
  pub ending_base_index: crate::simple_type::UInt32Value,
  ///Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: std::boxed::Box<Text>,
}
/// Phonetic Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:phoneticPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PhoneticPr/x:phoneticPr")]
pub struct PhoneticProperties {
  /// Font Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fontId
  #[sdk(attr(qname = ":fontId"))]
  pub font_id: crate::simple_type::UInt32Value,
  /// Character Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<PhoneticValues>,
  /// Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alignment
  #[sdk(attr(qname = ":alignment"))]
  pub alignment: Option<PhoneticAlignmentValues>,
}
/// Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:header.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionHeader/x:header")]
pub struct Header {
  /// GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Date Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dateTime
  #[sdk(attr(qname = ":dateTime"))]
  pub date_time: crate::simple_type::DateTimeValue,
  /// Last Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxSheetId
  #[sdk(attr(qname = ":maxSheetId"))]
  pub max_sheet_id: crate::simple_type::UInt32Value,
  /// User Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// Relationship ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Minimum Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minRId
  #[sdk(attr(qname = ":minRId"))]
  pub min_revision_id: Option<crate::simple_type::UInt32Value>,
  /// Max Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxRId
  #[sdk(attr(qname = ":maxRId"))]
  pub max_revision_id: Option<crate::simple_type::UInt32Value>,
  ///Sheet Id Map
  #[sdk(child(qname = "x:CT_SheetIdMap/x:sheetIdMap"))]
  pub sheet_id_map: std::boxed::Box<SheetIdMap>,
  ///Reviewed List
  #[sdk(child(qname = "x:CT_ReviewedRevisions/x:reviewedList"))]
  pub reviewed_list: Option<ReviewedList>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Row Column Insert Delete.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rrc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionRowColumn/x:rrc")]
pub struct RevisionRowColumn {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sId
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// End Of List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :eol
  #[sdk(attr(qname = ":eol"))]
  pub end_of_list: Option<crate::simple_type::BooleanValue>,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// User Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: RowColumnActionValues,
  /// Edge Deleted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :edge
  #[sdk(attr(qname = ":edge"))]
  pub edge: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub xml_children: Vec<RevisionRowColumnChoice>,
}
/// Revision Cell Move.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionMove/x:rm")]
pub struct RevisionMove {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :source
  #[sdk(attr(qname = ":source"))]
  pub source: crate::simple_type::StringValue,
  /// Destination
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :destination
  #[sdk(attr(qname = ":destination"))]
  pub destination: crate::simple_type::StringValue,
  /// Source Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceSheetId
  #[sdk(attr(qname = ":sourceSheetId"))]
  pub source_sheet_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Vec<RevisionMoveChoice>,
}
/// Revision Custom View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rcv.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionCustomView/x:rcv")]
pub struct RevisionCustomView {
  /// GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: RevisionActionValues,
}
/// Revision Sheet Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rsnm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionSheetRename/x:rsnm")]
pub struct RevisionSheetName {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Old Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldName
  #[sdk(attr(qname = ":oldName"))]
  pub old_name: crate::simple_type::StringValue,
  /// New Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :newName
  #[sdk(attr(qname = ":newName"))]
  pub new_name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Insert Sheet.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ris.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionInsertSheet/x:ris")]
pub struct RevisionInsertSheet {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Sheet Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetPosition
  #[sdk(attr(qname = ":sheetPosition"))]
  pub sheet_position: crate::simple_type::UInt32Value,
}
/// Revision Cell Change.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rcc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionCellChange/x:rcc")]
pub struct RevisionCellChange {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sId
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Old Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :odxf
  #[sdk(attr(qname = ":odxf"))]
  pub old_formatting: Option<crate::simple_type::BooleanValue>,
  /// Row Column Formatting Change
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xfDxf
  #[sdk(attr(qname = ":xfDxf"))]
  pub row_column_formatting_affected: Option<crate::simple_type::BooleanValue>,
  /// Style Revision
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_revision: Option<crate::simple_type::BooleanValue>,
  /// Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxf
  #[sdk(attr(qname = ":dxf"))]
  pub format: Option<crate::simple_type::BooleanValue>,
  /// Number Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Quote Prefix
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :quotePrefix
  #[sdk(attr(qname = ":quotePrefix"))]
  pub quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Old Quote Prefix
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldQuotePrefix
  #[sdk(attr(qname = ":oldQuotePrefix"))]
  pub old_quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Phonetic Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub has_phonetic_text: Option<crate::simple_type::BooleanValue>,
  /// Old Phonetic Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldPh
  #[sdk(attr(qname = ":oldPh"))]
  pub old_phonetic_text: Option<crate::simple_type::BooleanValue>,
  /// End of List  Formula Update
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :endOfListFormulaUpdate
  #[sdk(attr(qname = ":endOfListFormulaUpdate"))]
  pub end_of_list_formula_update: Option<crate::simple_type::BooleanValue>,
  ///Old Cell Data
  #[sdk(child(qname = "x:CT_Cell/x:oc"))]
  pub old_cell: Option<std::boxed::Box<OldCell>>,
  ///New Cell Data
  #[sdk(child(qname = "x:CT_NewCell/x:nc"))]
  pub new_cell: std::boxed::Box<NewCell>,
  ///Old Formatting Information
  #[sdk(child(qname = "x:CT_Dxf/x:odxf"))]
  pub old_differential_format: Option<std::boxed::Box<OldDifferentialFormat>>,
  ///New Formatting Information
  #[sdk(child(qname = "x:CT_Dxf/x:ndxf"))]
  pub new_differential_format: Option<std::boxed::Box<NewDifferentialFormat>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rfmt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionFormatting/x:rfmt")]
pub struct RevisionFormat {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Row or Column Formatting Change
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xfDxf
  #[sdk(attr(qname = ":xfDxf"))]
  pub row_or_column_affected: Option<crate::simple_type::BooleanValue>,
  /// Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_affected: Option<crate::simple_type::BooleanValue>,
  /// Sequence Of References
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Start index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::UInt32Value>,
  ///Formatting
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub differential_format: Option<std::boxed::Box<DifferentialFormat>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision AutoFormat.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:raf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionAutoFormatting/x:raf")]
pub struct RevisionAutoFormat {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Auto Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFormatId
  #[sdk(attr(qname = ":autoFormatId"))]
  pub auto_format_id: Option<crate::simple_type::UInt32Value>,
  /// Apply Number Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyNumberFormats
  #[sdk(attr(qname = ":applyNumberFormats"))]
  pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Border Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyBorderFormats
  #[sdk(attr(qname = ":applyBorderFormats"))]
  pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Font Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyFontFormats
  #[sdk(attr(qname = ":applyFontFormats"))]
  pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Pattern Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyPatternFormats
  #[sdk(attr(qname = ":applyPatternFormats"))]
  pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyAlignmentFormats
  #[sdk(attr(qname = ":applyAlignmentFormats"))]
  pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
  /// Apply Width / Height Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyWidthHeightFormats
  #[sdk(attr(qname = ":applyWidthHeightFormats"))]
  pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Revision Defined Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rdn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionDefinedName/x:rdn")]
pub struct RevisionDefinedName {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Local Name Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :localSheetId
  #[sdk(attr(qname = ":localSheetId"))]
  pub local_sheet_id: Option<crate::simple_type::UInt32Value>,
  /// Custom View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customView
  #[sdk(attr(qname = ":customView"))]
  pub custom_view: Option<crate::simple_type::BooleanValue>,
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// Old Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldFunction
  #[sdk(attr(qname = ":oldFunction"))]
  pub old_function: Option<crate::simple_type::BooleanValue>,
  /// Function Group Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :functionGroupId
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::ByteValue>,
  /// Old Function Group Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldFunctionGroupId
  #[sdk(attr(qname = ":oldFunctionGroupId"))]
  pub old_function_group_id: Option<crate::simple_type::ByteValue>,
  /// Shortcut Key
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shortcutKey
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::ByteValue>,
  /// Old Short Cut Key
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldShortcutKey
  #[sdk(attr(qname = ":oldShortcutKey"))]
  pub old_shortcut_key: Option<crate::simple_type::ByteValue>,
  /// Named Range Hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Old Hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldHidden
  #[sdk(attr(qname = ":oldHidden"))]
  pub old_hidden: Option<crate::simple_type::BooleanValue>,
  /// New Custom Menu
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customMenu
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// Old Custom Menu Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldCustomMenu
  #[sdk(attr(qname = ":oldCustomMenu"))]
  pub old_custom_menu: Option<crate::simple_type::StringValue>,
  /// Description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Old Description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldDescription
  #[sdk(attr(qname = ":oldDescription"))]
  pub old_description: Option<crate::simple_type::StringValue>,
  /// New Help Topic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :help
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// Old Help Topic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldHelp
  #[sdk(attr(qname = ":oldHelp"))]
  pub old_help: Option<crate::simple_type::StringValue>,
  /// Status Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :statusBar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// Old Status Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldStatusBar
  #[sdk(attr(qname = ":oldStatusBar"))]
  pub old_status_bar: Option<crate::simple_type::StringValue>,
  /// Name Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Old Name Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldComment
  #[sdk(attr(qname = ":oldComment"))]
  pub old_comment: Option<crate::simple_type::StringValue>,
  ///Formula
  #[sdk(child(qname = "x:CT_Xstring/x:formula"))]
  pub formula: Option<Formula>,
  ///Old Formula
  #[sdk(child(qname = "x:CT_Xstring/x:oldFormula"))]
  pub old_formula: Option<OldFormula>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Revision Cell Comment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rcmt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionComment/x:rcmt")]
pub struct RevisionComment {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cell
  #[sdk(attr(qname = ":cell"))]
  pub cell: crate::simple_type::StringValue,
  /// GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Action
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: Option<RevisionActionValues>,
  /// Always Show Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alwaysShow
  #[sdk(attr(qname = ":alwaysShow"))]
  pub always_show: Option<crate::simple_type::BooleanValue>,
  /// Old Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :old
  #[sdk(attr(qname = ":old"))]
  pub old: Option<crate::simple_type::BooleanValue>,
  /// Comment In Hidden Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenRow
  #[sdk(attr(qname = ":hiddenRow"))]
  pub hidden_row: Option<crate::simple_type::BooleanValue>,
  /// Hidden Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenColumn
  #[sdk(attr(qname = ":hiddenColumn"))]
  pub hidden_column: Option<crate::simple_type::BooleanValue>,
  /// Author
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :author
  #[sdk(attr(qname = ":author"))]
  pub author: crate::simple_type::StringValue,
  /// Original Comment Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldLength
  #[sdk(attr(qname = ":oldLength"))]
  pub old_length: Option<crate::simple_type::UInt32Value>,
  /// New Comment Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :newLength
  #[sdk(attr(qname = ":newLength"))]
  pub new_length: Option<crate::simple_type::UInt32Value>,
}
/// Revision Query Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rqt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionQueryTableField/x:rqt")]
pub struct RevisionQueryTable {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// QueryTable Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Field Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldId
  #[sdk(attr(qname = ":fieldId"))]
  pub field_id: crate::simple_type::UInt32Value,
}
/// Revision Merge Conflict.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rcft.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RevisionConflict/x:rcft")]
pub struct RevisionConflict {
  /// Revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
  /// Revision From Rejection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ua
  #[sdk(attr(qname = ":ua"))]
  pub ua: Option<crate::simple_type::BooleanValue>,
  /// Revision Undo Rejected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ra
  #[sdk(attr(qname = ":ra"))]
  pub ra: Option<crate::simple_type::BooleanValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// Sheet Id Map.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetIdMap.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetIdMap/x:sheetIdMap")]
pub struct SheetIdMap {
  /// Sheet Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_SheetId/x:sheetId"))]
  pub x_sheet_id: Vec<SheetId>,
}
/// Reviewed List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:reviewedList.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ReviewedRevisions/x:reviewedList")]
pub struct ReviewedList {
  /// Reviewed Revisions Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Reviewed/x:reviewed"))]
  pub x_reviewed: Vec<Reviewed>,
}
/// Reviewed.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:reviewed.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Reviewed/x:reviewed")]
pub struct Reviewed {
  /// revision Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub revision_id: crate::simple_type::UInt32Value,
}
/// Undo.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:undo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_UndoInfo/x:undo")]
pub struct Undo {
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :index
  #[sdk(attr(qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  /// Expression
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :exp
  #[sdk(attr(qname = ":exp"))]
  pub expression: FormulaExpressionValues,
  /// Reference 3D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref3D
  #[sdk(attr(qname = ":ref3D"))]
  pub reference3_d: Option<crate::simple_type::BooleanValue>,
  /// Array Entered
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// Value Needed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: Option<crate::simple_type::BooleanValue>,
  /// Defined Name Formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nf
  #[sdk(attr(qname = ":nf"))]
  pub defined_name_formula: Option<crate::simple_type::BooleanValue>,
  /// Cross Sheet Move
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cs
  #[sdk(attr(qname = ":cs"))]
  pub cross_sheet_move: Option<crate::simple_type::BooleanValue>,
  /// Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dr
  #[sdk(attr(qname = ":dr"))]
  pub deleted_range: crate::simple_type::StringValue,
  /// Defined Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dn
  #[sdk(attr(qname = ":dn"))]
  pub defined_name: Option<crate::simple_type::StringValue>,
  /// Cell Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sId
  #[sdk(attr(qname = ":sId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// Old Cell Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cell/x:oc")]
pub struct OldCell {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Style Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cm
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vm
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  ///Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  ///Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  ///Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:c.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cell/x:c")]
pub struct Cell {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Style Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cm
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vm
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  ///Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  ///Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  ///Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CellType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cell/")]
pub struct CellType {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: Option<crate::simple_type::StringValue>,
  /// Style Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cm
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vm
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub xml_children: Vec<CellTypeChoice>,
}
/// New Cell Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:nc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NewCell/x:nc")]
pub struct NewCell {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Style Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Cell Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Cell Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cm
  #[sdk(attr(qname = ":cm"))]
  pub cell_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Value Metadata Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vm
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  /// Show Phonetic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  ///Formula
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  pub cell_formula: Option<CellFormula>,
  ///Cell Value
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  pub cell_value: Option<CellValue>,
  ///Rich Text Inline
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  pub inline_string: Option<std::boxed::Box<InlineString>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Old Formatting Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:odxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:odxf")]
pub struct OldDifferentialFormat {
  ///Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  ///Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  ///Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  ///Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  ///Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// New Formatting Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ndxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:ndxf")]
pub struct NewDifferentialFormat {
  ///Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  ///Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  ///Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  ///Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  ///Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x:dxf")]
pub struct DifferentialFormat {
  ///Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<std::boxed::Box<Font>>,
  ///Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  ///Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<std::boxed::Box<Fill>>,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  ///Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<std::boxed::Box<Border>>,
  ///Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/")]
pub struct DifferentialFormatType {
  #[sdk(choice)]
  pub xml_children: Vec<DifferentialFormatTypeChoice>,
}
/// Sheet Id.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetId/x:sheetId")]
pub struct SheetId {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:f.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellFormula/x:f")]
pub struct CellFormula {
  /// Formula Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub formula_type: Option<CellFormulaValues>,
  /// Always Calculate Array
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :aca
  #[sdk(attr(qname = ":aca"))]
  pub always_calculate_array: Option<crate::simple_type::BooleanValue>,
  /// Range of Cells
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Data Table 2-D
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dt2D
  #[sdk(attr(qname = ":dt2D"))]
  pub data_table2_d: Option<crate::simple_type::BooleanValue>,
  /// Data Table Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dtr
  #[sdk(attr(qname = ":dtr"))]
  pub data_table_row: Option<crate::simple_type::BooleanValue>,
  /// Input 1 Deleted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :del1
  #[sdk(attr(qname = ":del1"))]
  pub input1_deleted: Option<crate::simple_type::BooleanValue>,
  /// Input 2 Deleted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :del2
  #[sdk(attr(qname = ":del2"))]
  pub input2_deleted: Option<crate::simple_type::BooleanValue>,
  /// Data Table Cell 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r1
  #[sdk(attr(qname = ":r1"))]
  pub r1: Option<crate::simple_type::StringValue>,
  /// Input Cell 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r2
  #[sdk(attr(qname = ":r2"))]
  pub r2: Option<crate::simple_type::StringValue>,
  /// Calculate Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ca
  #[sdk(attr(qname = ":ca"))]
  pub calculate_cell: Option<crate::simple_type::BooleanValue>,
  /// Shared Group Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :si
  #[sdk(attr(qname = ":si"))]
  pub shared_index: Option<crate::simple_type::UInt32Value>,
  /// Assigns Value to Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bx
  #[sdk(attr(qname = ":bx"))]
  pub bx: Option<crate::simple_type::BooleanValue>,
  /// Content Contains Significant Whitespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// User Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:userInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SharedUser/x:userInfo")]
pub struct UserInfo {
  /// User Revisions GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// User Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// User Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::Int32Value,
  /// Date Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dateTime
  #[sdk(attr(qname = ":dateTime"))]
  pub date_time: crate::simple_type::DateTimeValue,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Row.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:row.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Row/x:row")]
pub struct Row {
  /// Row Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub row_index: Option<crate::simple_type::UInt32Value>,
  /// Spans
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spans
  #[sdk(attr(qname = ":spans"))]
  pub spans: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// Style Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub style_index: Option<crate::simple_type::UInt32Value>,
  /// Custom Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customFormat
  #[sdk(attr(qname = ":customFormat"))]
  pub custom_format: Option<crate::simple_type::BooleanValue>,
  /// Row Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ht
  #[sdk(attr(qname = ":ht"))]
  pub height: Option<crate::simple_type::DoubleValue>,
  /// Hidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Custom Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customHeight
  #[sdk(attr(qname = ":customHeight"))]
  pub custom_height: Option<crate::simple_type::BooleanValue>,
  /// Outline Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineLevel
  #[sdk(attr(qname = ":outlineLevel"))]
  pub outline_level: Option<crate::simple_type::ByteValue>,
  /// Collapsed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :collapsed
  #[sdk(attr(qname = ":collapsed"))]
  pub collapsed: Option<crate::simple_type::BooleanValue>,
  /// Thick Top Border
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thickTop
  #[sdk(attr(qname = ":thickTop"))]
  pub thick_top: Option<crate::simple_type::BooleanValue>,
  /// Thick Bottom
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thickBot
  #[sdk(attr(qname = ":thickBot"))]
  pub thick_bot: Option<crate::simple_type::BooleanValue>,
  /// Show Phonetic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ph
  #[sdk(attr(qname = ":ph"))]
  pub show_phonetic: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// dyDescent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: x14ac:dyDescent
  #[sdk(attr(qname = "x14ac:dyDescent"))]
  pub dy_descent: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "x:CT_Cell/x:c"))]
  pub x_c: Vec<Cell>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Column Width and Formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:col.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Col/x:col")]
pub struct Column {
  /// Minimum Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :min
  #[sdk(attr(qname = ":min"))]
  pub min: crate::simple_type::UInt32Value,
  /// Maximum Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :max
  #[sdk(attr(qname = ":max"))]
  pub max: crate::simple_type::UInt32Value,
  /// Column Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::DoubleValue>,
  /// Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::UInt32Value>,
  /// Hidden Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Best Fit Column Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bestFit
  #[sdk(attr(qname = ":bestFit"))]
  pub best_fit: Option<crate::simple_type::BooleanValue>,
  /// Custom Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customWidth
  #[sdk(attr(qname = ":customWidth"))]
  pub custom_width: Option<crate::simple_type::BooleanValue>,
  /// Show Phonetic Information
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :phonetic
  #[sdk(attr(qname = ":phonetic"))]
  pub phonetic: Option<crate::simple_type::BooleanValue>,
  /// Outline Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineLevel
  #[sdk(attr(qname = ":outlineLevel"))]
  pub outline_level: Option<crate::simple_type::ByteValue>,
  /// Collapsed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :collapsed
  #[sdk(attr(qname = ":collapsed"))]
  pub collapsed: Option<crate::simple_type::BooleanValue>,
}
/// Outline Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:outlinePr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OutlinePr/x:outlinePr")]
pub struct OutlineProperties {
  /// Apply Styles in Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyStyles
  #[sdk(attr(qname = ":applyStyles"))]
  pub apply_styles: Option<crate::simple_type::BooleanValue>,
  /// Summary Below
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :summaryBelow
  #[sdk(attr(qname = ":summaryBelow"))]
  pub summary_below: Option<crate::simple_type::BooleanValue>,
  /// Summary Right
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :summaryRight
  #[sdk(attr(qname = ":summaryRight"))]
  pub summary_right: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showOutlineSymbols
  #[sdk(attr(qname = ":showOutlineSymbols"))]
  pub show_outline_symbols: Option<crate::simple_type::BooleanValue>,
}
/// Page Setup Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageSetUpPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageSetUpPr/x:pageSetUpPr")]
pub struct PageSetupProperties {
  /// Show Auto Page Breaks
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoPageBreaks
  #[sdk(attr(qname = ":autoPageBreaks"))]
  pub auto_page_breaks: Option<crate::simple_type::BooleanValue>,
  /// Fit To Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitToPage
  #[sdk(attr(qname = ":fitToPage"))]
  pub fit_to_page: Option<crate::simple_type::BooleanValue>,
}
/// View Pane.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pane.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Pane/x:pane")]
pub struct Pane {
  /// Horizontal Split Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xSplit
  #[sdk(attr(qname = ":xSplit"))]
  pub horizontal_split: Option<crate::simple_type::DoubleValue>,
  /// Vertical Split Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ySplit
  #[sdk(attr(qname = ":ySplit"))]
  pub vertical_split: Option<crate::simple_type::DoubleValue>,
  /// Top Left Visible Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :topLeftCell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  /// Active Pane
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activePane
  #[sdk(attr(qname = ":activePane"))]
  pub active_pane: Option<PaneValues>,
  /// Split State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :state
  #[sdk(attr(qname = ":state"))]
  pub state: Option<PaneStateValues>,
}
/// Selection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:selection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Selection/x:selection")]
pub struct Selection {
  /// Pane
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pane
  #[sdk(attr(qname = ":pane"))]
  pub pane: Option<PaneValues>,
  /// Active Cell Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeCell
  #[sdk(attr(qname = ":activeCell"))]
  pub active_cell: Option<crate::simple_type::StringValue>,
  /// Active Cell Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeCellId
  #[sdk(attr(qname = ":activeCellId"))]
  pub active_cell_id: Option<crate::simple_type::UInt32Value>,
  /// Sequence of References
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
}
/// PivotTable Selection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotSelection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotSelection/x:pivotSelection")]
pub struct PivotSelection {
  /// Pane
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pane
  #[sdk(attr(qname = ":pane"))]
  pub pane: Option<PaneValues>,
  /// Show Header
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showHeader
  #[sdk(attr(qname = ":showHeader"))]
  pub show_header: Option<crate::simple_type::BooleanValue>,
  /// Label
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :label
  #[sdk(attr(qname = ":label"))]
  pub label: Option<crate::simple_type::BooleanValue>,
  /// Data Selection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :data
  #[sdk(attr(qname = ":data"))]
  pub data: Option<crate::simple_type::BooleanValue>,
  /// Extendable
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :extendable
  #[sdk(attr(qname = ":extendable"))]
  pub extendable: Option<crate::simple_type::BooleanValue>,
  /// Selection Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<PivotTableAxisValues>,
  /// Dimension
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dimension
  #[sdk(attr(qname = ":dimension"))]
  pub dimension: Option<crate::simple_type::UInt32Value>,
  /// Start
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// Minimum
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :min
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// Maximum
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :max
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// Active Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeRow
  #[sdk(attr(qname = ":activeRow"))]
  pub active_row: Option<crate::simple_type::UInt32Value>,
  /// Active Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeCol
  #[sdk(attr(qname = ":activeCol"))]
  pub active_column: Option<crate::simple_type::UInt32Value>,
  /// Previous Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :previousRow
  #[sdk(attr(qname = ":previousRow"))]
  pub previous_row: Option<crate::simple_type::UInt32Value>,
  /// Previous Column Selection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :previousCol
  #[sdk(attr(qname = ":previousCol"))]
  pub previous_column: Option<crate::simple_type::UInt32Value>,
  /// Click Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :click
  #[sdk(attr(qname = ":click"))]
  pub click: Option<crate::simple_type::UInt32Value>,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  ///Pivot Area
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// Break.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:brk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Break/x:brk")]
pub struct Break {
  /// Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Minimum
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :min
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// Maximum
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :max
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// Manual Page Break
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :man
  #[sdk(attr(qname = ":man"))]
  pub manual_page_break: Option<crate::simple_type::BooleanValue>,
  /// Pivot-Created Page Break
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pt
  #[sdk(attr(qname = ":pt"))]
  pub pivot_table_page_break: Option<crate::simple_type::BooleanValue>,
}
/// Data Consolidation Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataRef/x:dataRef")]
pub struct DataReference {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheet
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Horizontal Page Breaks.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rowBreaks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageBreak/x:rowBreaks")]
pub struct RowBreaks {
  /// Page Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Manual Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :manualBreakCount
  #[sdk(attr(qname = ":manualBreakCount"))]
  pub manual_break_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Break/x:brk"))]
  pub x_brk: Vec<Break>,
}
/// Vertical Page Breaks.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colBreaks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageBreak/x:colBreaks")]
pub struct ColumnBreaks {
  /// Page Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Manual Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :manualBreakCount
  #[sdk(attr(qname = ":manualBreakCount"))]
  pub manual_break_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Break/x:brk"))]
  pub x_brk: Vec<Break>,
}
/// Defines the PageBreakType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageBreak/")]
pub struct PageBreakType {
  /// Page Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Manual Break Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :manualBreakCount
  #[sdk(attr(qname = ":manualBreakCount"))]
  pub manual_break_count: Option<crate::simple_type::UInt32Value>,
  ///Break.
  #[sdk(child(qname = "x:CT_Break/x:brk"))]
  pub r#break: Vec<Break>,
}
/// Page Margins.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageMargins.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageMargins/x:pageMargins")]
pub struct PageMargins {
  /// Left Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: crate::simple_type::DoubleValue,
  /// Right Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: crate::simple_type::DoubleValue,
  /// Top Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: crate::simple_type::DoubleValue,
  /// Bottom Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: crate::simple_type::DoubleValue,
  /// Header Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :header
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// Footer Page Margin
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :footer
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Print Options.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:printOptions.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PrintOptions/x:printOptions")]
pub struct PrintOptions {
  /// Horizontal Centered
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalCentered
  #[sdk(attr(qname = ":horizontalCentered"))]
  pub horizontal_centered: Option<crate::simple_type::BooleanValue>,
  /// Vertical Centered
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :verticalCentered
  #[sdk(attr(qname = ":verticalCentered"))]
  pub vertical_centered: Option<crate::simple_type::BooleanValue>,
  /// Print Headings
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headings
  #[sdk(attr(qname = ":headings"))]
  pub headings: Option<crate::simple_type::BooleanValue>,
  /// Print Grid Lines
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gridLines
  #[sdk(attr(qname = ":gridLines"))]
  pub grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Grid Lines Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gridLinesSet
  #[sdk(attr(qname = ":gridLinesSet"))]
  pub grid_lines_set: Option<crate::simple_type::BooleanValue>,
}
/// Page Setup Settings.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageSetup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageSetup/x:pageSetup")]
pub struct PageSetup {
  /// Paper Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :paperSize
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// Print Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstPageNumber
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// Fit To Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitToWidth
  #[sdk(attr(qname = ":fitToWidth"))]
  pub fit_to_width: Option<crate::simple_type::UInt32Value>,
  /// Fit To Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitToHeight
  #[sdk(attr(qname = ":fitToHeight"))]
  pub fit_to_height: Option<crate::simple_type::UInt32Value>,
  /// Page Order
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pageOrder
  #[sdk(attr(qname = ":pageOrder"))]
  pub page_order: Option<PageOrderValues>,
  /// Orientation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<OrientationValues>,
  /// Use Printer Defaults
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :usePrinterDefaults
  #[sdk(attr(qname = ":usePrinterDefaults"))]
  pub use_printer_defaults: Option<crate::simple_type::BooleanValue>,
  /// Black And White
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blackAndWhite
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Print Cell Comments
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cellComments
  #[sdk(attr(qname = ":cellComments"))]
  pub cell_comments: Option<CellCommentsValues>,
  /// Use First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useFirstPageNumber
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Print Error Handling
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :errors
  #[sdk(attr(qname = ":errors"))]
  pub errors: Option<PrintErrorValues>,
  /// Horizontal DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalDpi
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::UInt32Value>,
  /// Vertical DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :verticalDpi
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::UInt32Value>,
  /// Number Of Copies
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
  /// Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Header Footer Settings.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:headerFooter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_HeaderFooter/x:headerFooter")]
pub struct HeaderFooter {
  /// Different Odd Even Header Footer
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :differentOddEven
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// Different First Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :differentFirst
  #[sdk(attr(qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// Scale Header and Footer With Document
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scaleWithDoc
  #[sdk(attr(qname = ":scaleWithDoc"))]
  pub scale_with_doc: Option<crate::simple_type::BooleanValue>,
  /// Align Margins
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alignWithMargins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  ///Odd Header
  #[sdk(child(qname = "x:CT_Xstring/x:oddHeader"))]
  pub odd_header: Option<OddHeader>,
  ///Odd Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:oddFooter"))]
  pub odd_footer: Option<OddFooter>,
  ///Even Page Header
  #[sdk(child(qname = "x:CT_Xstring/x:evenHeader"))]
  pub even_header: Option<EvenHeader>,
  ///Even Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:evenFooter"))]
  pub even_footer: Option<EvenFooter>,
  ///First Page Header
  #[sdk(child(qname = "x:CT_Xstring/x:firstHeader"))]
  pub first_header: Option<FirstHeader>,
  ///First Page Footer
  #[sdk(child(qname = "x:CT_Xstring/x:firstFooter"))]
  pub first_footer: Option<FirstFooter>,
}
/// AutoFilter Settings.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:autoFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoFilter/x:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr:uid
  #[sdk(attr(qname = "xr:uid"))]
  pub xr_uid: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_FilterColumn/x:filterColumn"))]
  pub x_filter_column: Vec<FilterColumn>,
  /// _
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<std::boxed::Box<SortState>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Conditional Formatting Rule.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cfRule.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRule/x:cfRule")]
pub struct ConditionalFormattingRule {
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormatValues,
  /// Differential Formatting Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Priority
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: crate::simple_type::Int32Value,
  /// Stop If True
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stopIfTrue
  #[sdk(attr(qname = ":stopIfTrue"))]
  pub stop_if_true: Option<crate::simple_type::BooleanValue>,
  /// Above Or Below Average
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :aboveAverage
  #[sdk(attr(qname = ":aboveAverage"))]
  pub above_average: Option<crate::simple_type::BooleanValue>,
  /// Top 10 Percent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Bottom N
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::BooleanValue>,
  /// Operator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<ConditionalFormattingOperatorValues>,
  /// Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :text
  #[sdk(attr(qname = ":text"))]
  pub text: Option<crate::simple_type::StringValue>,
  /// Time Period
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :timePeriod
  #[sdk(attr(qname = ":timePeriod"))]
  pub time_period: Option<TimePeriodValues>,
  /// Rank
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rank
  #[sdk(attr(qname = ":rank"))]
  pub rank: Option<crate::simple_type::UInt32Value>,
  /// StdDev
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :stdDev
  #[sdk(attr(qname = ":stdDev"))]
  pub std_dev: Option<crate::simple_type::Int32Value>,
  /// Equal Average
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :equalAverage
  #[sdk(attr(qname = ":equalAverage"))]
  pub equal_average: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Xstring/x:formula"))]
  pub x_formula: Vec<Formula>,
  /// _
  #[sdk(child(qname = "x:CT_ColorScale/x:colorScale"))]
  pub x_color_scale: Option<ColorScale>,
  /// _
  #[sdk(child(qname = "x:CT_DataBar/x:dataBar"))]
  pub x_data_bar: Option<std::boxed::Box<DataBar>>,
  /// _
  #[sdk(child(qname = "x:CT_IconSet/x:iconSet"))]
  pub x_icon_set: Option<IconSet>,
  /// _
  #[sdk(child(qname = "x:CT_CfRuleExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ConditionalFormattingRuleExtensionList>,
}
/// Hyperlink.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:hyperlink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Hyperlink/x:hyperlink")]
pub struct Hyperlink {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :location
  #[sdk(attr(qname = ":location"))]
  pub location: Option<crate::simple_type::StringValue>,
  /// Tool Tip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// Display String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :display
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::StringValue>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr:uid
  #[sdk(attr(qname = "xr:uid"))]
  pub xr_uid: Option<crate::simple_type::StringValue>,
}
/// Conditional Format Value Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cfvo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cfvo/x:cfvo")]
pub struct ConditionalFormatValueObject {
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormatValueObjectValues,
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// Greater Than Or Equal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :gte
  #[sdk(attr(qname = ":gte"))]
  pub greater_than_or_equal: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Scenario.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:scenario.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Scenario/x:scenario")]
pub struct Scenario {
  /// Scenario Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Scenario Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Hidden Scenario
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Changing Cell Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// User Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :user
  #[sdk(attr(qname = ":user"))]
  pub user: Option<crate::simple_type::StringValue>,
  /// Scenario Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_InputCells/x:inputCells"))]
  pub x_input_cells: Vec<InputCells>,
}
/// Protected Range.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:protectedRange.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ProtectedRange/x:protectedRange")]
pub struct ProtectedRange {
  /// password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// algorithmName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// sqref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// securityDescriptor
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :securityDescriptor
  #[sdk(attr(qname = ":securityDescriptor"))]
  pub security_descriptor: Option<crate::simple_type::StringValue>,
}
/// Cell Watch Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellWatch.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellWatch/x:cellWatch")]
pub struct CellWatch {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
}
/// Chart Sheet Page Setup.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageSetup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CsPageSetup/x:pageSetup")]
pub struct ChartSheetPageSetup {
  /// Paper Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :paperSize
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstPageNumber
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// Orientation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<OrientationValues>,
  /// Use Printer Defaults
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :usePrinterDefaults
  #[sdk(attr(qname = ":usePrinterDefaults"))]
  pub use_printer_defaults: Option<crate::simple_type::BooleanValue>,
  /// Black And White
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blackAndWhite
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Use First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useFirstPageNumber
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Horizontal DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalDpi
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::UInt32Value>,
  /// Vertical DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :verticalDpi
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::UInt32Value>,
  /// Number Of Copies
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
  /// Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Custom Property.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomProperty/x:customPr")]
pub struct CustomProperty {
  /// Custom Property Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Web Publishing Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPublishItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishItem/x:webPublishItem")]
pub struct WebPublishItem {
  /// Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Destination Bookmark
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :divId
  #[sdk(attr(qname = ":divId"))]
  pub div_id: crate::simple_type::StringValue,
  /// Web Source Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceType
  #[sdk(attr(qname = ":sourceType"))]
  pub source_type: WebSourceValues,
  /// Source Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceRef
  #[sdk(attr(qname = ":sourceRef"))]
  pub source_ref: Option<crate::simple_type::StringValue>,
  /// Source Object Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceObject
  #[sdk(attr(qname = ":sourceObject"))]
  pub source_object: Option<crate::simple_type::StringValue>,
  /// Destination File Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :destinationFile
  #[sdk(attr(qname = ":destinationFile"))]
  pub destination_file: crate::simple_type::StringValue,
  /// Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Automatically Publish
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoRepublish
  #[sdk(attr(qname = ":autoRepublish"))]
  pub auto_republish: Option<crate::simple_type::BooleanValue>,
}
/// Table Part.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tablePart.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TablePart/x:tablePart")]
pub struct TablePart {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Chart Sheet View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetView/x:sheetView")]
pub struct ChartSheetView {
  /// Sheet Tab Selected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tabSelected
  #[sdk(attr(qname = ":tabSelected"))]
  pub tab_selected: Option<crate::simple_type::BooleanValue>,
  /// Window Zoom Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomScale
  #[sdk(attr(qname = ":zoomScale"))]
  pub zoom_scale: Option<crate::simple_type::UInt32Value>,
  /// Workbook View Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookViewId
  #[sdk(attr(qname = ":workbookViewId"))]
  pub workbook_view_id: crate::simple_type::UInt32Value,
  /// Zoom To Fit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomToFit
  #[sdk(attr(qname = ":zoomToFit"))]
  pub zoom_to_fit: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Custom Chart Sheet View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customSheetView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomChartsheetView/x:customSheetView")]
pub struct CustomChartsheetView {
  /// GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Print Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// Visible State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :state
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Zoom To Fit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomToFit
  #[sdk(attr(qname = ":zoomToFit"))]
  pub zoom_to_fit: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  ///Chart Sheet Page Setup
  #[sdk(child(qname = "x:CT_CsPageSetup/x:pageSetup"))]
  pub chart_sheet_page_setup: Option<ChartSheetPageSetup>,
  /// _
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
}
/// Input Cells.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:inputCells.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_InputCells/x:inputCells")]
pub struct InputCells {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Deleted
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// Undone
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :undone
  #[sdk(attr(qname = ":undone"))]
  pub undone: Option<crate::simple_type::BooleanValue>,
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
  /// Number Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
}
/// Embedded Control.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:control.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Control/x:control")]
pub struct Control {
  /// Shape Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shapeId
  #[sdk(attr(qname = ":shapeId"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Control Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_ControlPr/x:controlPr"))]
  pub control_properties: Option<std::boxed::Box<ControlProperties>>,
}
/// Ignored Error.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ignoredError.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IgnoredError/x:ignoredError")]
pub struct IgnoredError {
  /// Sequence of References
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Evaluation Error
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :evalError
  #[sdk(attr(qname = ":evalError"))]
  pub eval_error: Option<crate::simple_type::BooleanValue>,
  /// Two Digit Text Year
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :twoDigitTextYear
  #[sdk(attr(qname = ":twoDigitTextYear"))]
  pub two_digit_text_year: Option<crate::simple_type::BooleanValue>,
  /// Number Stored As Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numberStoredAsText
  #[sdk(attr(qname = ":numberStoredAsText"))]
  pub number_stored_as_text: Option<crate::simple_type::BooleanValue>,
  /// Formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::BooleanValue>,
  /// Formula Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formulaRange
  #[sdk(attr(qname = ":formulaRange"))]
  pub formula_range: Option<crate::simple_type::BooleanValue>,
  /// Unlocked Formula
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :unlockedFormula
  #[sdk(attr(qname = ":unlockedFormula"))]
  pub unlocked_formula: Option<crate::simple_type::BooleanValue>,
  /// Empty Cell Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :emptyCellReference
  #[sdk(attr(qname = ":emptyCellReference"))]
  pub empty_cell_reference: Option<crate::simple_type::BooleanValue>,
  /// List Data Validation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :listDataValidation
  #[sdk(attr(qname = ":listDataValidation"))]
  pub list_data_validation: Option<crate::simple_type::BooleanValue>,
  /// Calculated Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calculatedColumn
  #[sdk(attr(qname = ":calculatedColumn"))]
  pub calculated_column: Option<crate::simple_type::BooleanValue>,
}
/// Merged Cell.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mergeCell.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MergeCell/x:mergeCell")]
pub struct MergeCell {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Data Validation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataValidation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataValidation/x:dataValidation")]
pub struct DataValidation {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<DataValidationValues>,
  /// errorStyle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :errorStyle
  #[sdk(attr(qname = ":errorStyle"))]
  pub error_style: Option<DataValidationErrorStyleValues>,
  /// imeMode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :imeMode
  #[sdk(attr(qname = ":imeMode"))]
  pub ime_mode: Option<DataValidationImeModeValues>,
  /// operator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<DataValidationOperatorValues>,
  /// allowBlank
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowBlank
  #[sdk(attr(qname = ":allowBlank"))]
  pub allow_blank: Option<crate::simple_type::BooleanValue>,
  /// showDropDown
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showDropDown
  #[sdk(attr(qname = ":showDropDown"))]
  pub show_drop_down: Option<crate::simple_type::BooleanValue>,
  /// showInputMessage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showInputMessage
  #[sdk(attr(qname = ":showInputMessage"))]
  pub show_input_message: Option<crate::simple_type::BooleanValue>,
  /// showErrorMessage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showErrorMessage
  #[sdk(attr(qname = ":showErrorMessage"))]
  pub show_error_message: Option<crate::simple_type::BooleanValue>,
  /// errorTitle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :errorTitle
  #[sdk(attr(qname = ":errorTitle"))]
  pub error_title: Option<crate::simple_type::StringValue>,
  /// error
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :error
  #[sdk(attr(qname = ":error"))]
  pub error: Option<crate::simple_type::StringValue>,
  /// promptTitle
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :promptTitle
  #[sdk(attr(qname = ":promptTitle"))]
  pub prompt_title: Option<crate::simple_type::StringValue>,
  /// prompt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::StringValue>,
  /// sqref
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr:uid
  #[sdk(attr(qname = "xr:uid"))]
  pub xr_uid: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(text_child(qname = "x:ST_Xstring/x12ac:list"))]
  pub list: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_Xstring/x:formula1"))]
  pub formula1: Option<Formula1>,
  /// _
  #[sdk(child(qname = "x:CT_Xstring/x:formula2"))]
  pub formula2: Option<Formula2>,
}
/// Worksheet View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetView/x:sheetView")]
pub struct SheetView {
  /// Window Protection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :windowProtection
  #[sdk(attr(qname = ":windowProtection"))]
  pub window_protection: Option<crate::simple_type::BooleanValue>,
  /// Show Formulas
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showFormulas
  #[sdk(attr(qname = ":showFormulas"))]
  pub show_formulas: Option<crate::simple_type::BooleanValue>,
  /// Show Grid Lines
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showGridLines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Show Headers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRowColHeaders
  #[sdk(attr(qname = ":showRowColHeaders"))]
  pub show_row_col_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Zero Values
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showZeros
  #[sdk(attr(qname = ":showZeros"))]
  pub show_zeros: Option<crate::simple_type::BooleanValue>,
  /// Right To Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rightToLeft
  #[sdk(attr(qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// Sheet Tab Selected
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tabSelected
  #[sdk(attr(qname = ":tabSelected"))]
  pub tab_selected: Option<crate::simple_type::BooleanValue>,
  /// Show Ruler
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRuler
  #[sdk(attr(qname = ":showRuler"))]
  pub show_ruler: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showOutlineSymbols
  #[sdk(attr(qname = ":showOutlineSymbols"))]
  pub show_outline_symbols: Option<crate::simple_type::BooleanValue>,
  /// Default Grid Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultGridColor
  #[sdk(attr(qname = ":defaultGridColor"))]
  pub default_grid_color: Option<crate::simple_type::BooleanValue>,
  /// Show White Space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showWhiteSpace
  #[sdk(attr(qname = ":showWhiteSpace"))]
  pub show_white_space: Option<crate::simple_type::BooleanValue>,
  /// View Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :view
  #[sdk(attr(qname = ":view"))]
  pub view: Option<SheetViewValues>,
  /// Top Left Visible Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :topLeftCell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  /// Color Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colorId
  #[sdk(attr(qname = ":colorId"))]
  pub color_id: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomScale
  #[sdk(attr(qname = ":zoomScale"))]
  pub zoom_scale: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Normal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomScaleNormal
  #[sdk(attr(qname = ":zoomScaleNormal"))]
  pub zoom_scale_normal: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Page Break Preview
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomScaleSheetLayoutView
  #[sdk(attr(qname = ":zoomScaleSheetLayoutView"))]
  pub zoom_scale_sheet_layout_view: Option<crate::simple_type::UInt32Value>,
  /// Zoom Scale Page Layout View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zoomScalePageLayoutView
  #[sdk(attr(qname = ":zoomScalePageLayoutView"))]
  pub zoom_scale_page_layout_view: Option<crate::simple_type::UInt32Value>,
  /// Workbook View Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookViewId
  #[sdk(attr(qname = ":workbookViewId"))]
  pub workbook_view_id: crate::simple_type::UInt32Value,
  ///View Pane
  #[sdk(child(qname = "x:CT_Pane/x:pane"))]
  pub pane: Option<Pane>,
  /// _
  #[sdk(child(qname = "x:CT_Selection/x:selection"))]
  pub x_selection: Vec<Selection>,
  /// _
  #[sdk(child(qname = "x:CT_PivotSelection/x:pivotSelection"))]
  pub x_pivot_selection: Vec<PivotSelection>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Sheet View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customSheetView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomSheetView/x:customSheetView")]
pub struct CustomSheetView {
  /// GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Print Scale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scale
  #[sdk(attr(qname = ":scale"))]
  pub scale: Option<crate::simple_type::UInt32Value>,
  /// Color Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colorId
  #[sdk(attr(qname = ":colorId"))]
  pub color_id: Option<crate::simple_type::UInt32Value>,
  /// Show Page Breaks
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showPageBreaks
  #[sdk(attr(qname = ":showPageBreaks"))]
  pub show_page_breaks: Option<crate::simple_type::BooleanValue>,
  /// Show Formulas
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showFormulas
  #[sdk(attr(qname = ":showFormulas"))]
  pub show_formulas: Option<crate::simple_type::BooleanValue>,
  /// Show Grid Lines
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showGridLines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: Option<crate::simple_type::BooleanValue>,
  /// Show Headers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRowCol
  #[sdk(attr(qname = ":showRowCol"))]
  pub show_row_column: Option<crate::simple_type::BooleanValue>,
  /// Show Outline Symbols
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineSymbols
  #[sdk(attr(qname = ":outlineSymbols"))]
  pub outline_symbols: Option<crate::simple_type::BooleanValue>,
  /// Show Zero Values
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zeroValues
  #[sdk(attr(qname = ":zeroValues"))]
  pub zero_values: Option<crate::simple_type::BooleanValue>,
  /// Fit To Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fitToPage
  #[sdk(attr(qname = ":fitToPage"))]
  pub fit_to_page: Option<crate::simple_type::BooleanValue>,
  /// Print Area Defined
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :printArea
  #[sdk(attr(qname = ":printArea"))]
  pub print_area: Option<crate::simple_type::BooleanValue>,
  /// Filtered List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filter
  #[sdk(attr(qname = ":filter"))]
  pub filter: Option<crate::simple_type::BooleanValue>,
  /// Show AutoFitler Drop Down Controls
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showAutoFilter
  #[sdk(attr(qname = ":showAutoFilter"))]
  pub show_auto_filter: Option<crate::simple_type::BooleanValue>,
  /// Hidden Rows
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenRows
  #[sdk(attr(qname = ":hiddenRows"))]
  pub hidden_rows: Option<crate::simple_type::BooleanValue>,
  /// Hidden Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenColumns
  #[sdk(attr(qname = ":hiddenColumns"))]
  pub hidden_columns: Option<crate::simple_type::BooleanValue>,
  /// Visible State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :state
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filterUnique
  #[sdk(attr(qname = ":filterUnique"))]
  pub filter_unique: Option<crate::simple_type::BooleanValue>,
  /// View Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :view
  #[sdk(attr(qname = ":view"))]
  pub view: Option<SheetViewValues>,
  /// Show Ruler
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRuler
  #[sdk(attr(qname = ":showRuler"))]
  pub show_ruler: Option<crate::simple_type::BooleanValue>,
  /// Top Left Visible Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :topLeftCell
  #[sdk(attr(qname = ":topLeftCell"))]
  pub top_left_cell: Option<crate::simple_type::StringValue>,
  ///Pane Split Information
  #[sdk(child(qname = "x:CT_Pane/x:pane"))]
  pub pane: Option<Pane>,
  ///Selection
  #[sdk(child(qname = "x:CT_Selection/x:selection"))]
  pub selection: Option<Selection>,
  ///Horizontal Page Breaks
  #[sdk(child(qname = "x:CT_PageBreak/x:rowBreaks"))]
  pub row_breaks: Option<RowBreaks>,
  ///Vertical Page Breaks
  #[sdk(child(qname = "x:CT_PageBreak/x:colBreaks"))]
  pub column_breaks: Option<ColumnBreaks>,
  ///Page Margins
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  ///Print Options
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub print_options: Option<PrintOptions>,
  ///Page Setup Settings
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub page_setup: Option<PageSetup>,
  ///Header Footer Settings
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<HeaderFooter>>,
  ///AutoFilter Settings
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub auto_filter: Option<std::boxed::Box<AutoFilter>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// OLE Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleObject.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleObject/x:oleObject")]
pub struct OleObject {
  /// OLE ProgId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :progId
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: Option<crate::simple_type::StringValue>,
  /// Data or View Aspect
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dvAspect
  #[sdk(attr(qname = ":dvAspect"))]
  pub data_or_view_aspect: Option<DataViewAspectValues>,
  /// OLE Link Moniker
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :link
  #[sdk(attr(qname = ":link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// OLE Update
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oleUpdate
  #[sdk(attr(qname = ":oleUpdate"))]
  pub ole_update: Option<OleUpdateValues>,
  /// Auto Load
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoLoad
  #[sdk(attr(qname = ":autoLoad"))]
  pub auto_load: Option<crate::simple_type::BooleanValue>,
  /// Shape Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shapeId
  #[sdk(attr(qname = ":shapeId"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_ObjectPr/x:objectPr"))]
  pub embedded_object_properties: Option<std::boxed::Box<EmbeddedObjectProperties>>,
}
/// Metadata Types Collection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:metadataTypes.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataTypes/x:metadataTypes")]
pub struct MetadataTypes {
  /// Metadata Type Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataType/x:metadataType"))]
  pub x_metadata_type: Vec<MetadataType>,
}
/// Metadata String Store.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:metadataStrings.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataStrings/x:metadataStrings")]
pub struct MetadataStrings {
  /// MDX Metadata String Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_XStringElement/x:s"))]
  pub x_s: Vec<CharacterValue>,
}
/// MDX Metadata Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mdxMetadata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxMetadata/x:mdxMetadata")]
pub struct MdxMetadata {
  /// MDX Metadata Record Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Mdx/x:mdx"))]
  pub x_mdx: Vec<Mdx>,
}
/// Future Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:futureMetadata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FutureMetadata/x:futureMetadata")]
pub struct FutureMetadata {
  /// Metadata Type Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Future Metadata Block Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_FutureMetadataBlock/x:bk"))]
  pub x_bk: Vec<FutureMetadataBlock>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Cell Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellMetadata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlocks/x:cellMetadata")]
pub struct CellMetadata {
  /// Metadata Block Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataBlock/x:bk"))]
  pub x_bk: Vec<MetadataBlock>,
}
/// Value Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:valueMetadata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlocks/x:valueMetadata")]
pub struct ValueMetadata {
  /// Metadata Block Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataBlock/x:bk"))]
  pub x_bk: Vec<MetadataBlock>,
}
/// Defines the MetadataBlocksType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlocks/")]
pub struct MetadataBlocksType {
  /// Metadata Block Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  ///Metadata Block.
  #[sdk(child(qname = "x:CT_MetadataBlock/x:bk"))]
  pub metadata_block: Vec<MetadataBlock>,
}
/// Metadata Type Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:metadataType.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataType/x:metadataType")]
pub struct MetadataType {
  /// Metadata Type Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Minimum Supported Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minSupportedVersion
  #[sdk(attr(qname = ":minSupportedVersion"))]
  pub min_supported_version: crate::simple_type::UInt32Value,
  /// Metadata Ghost Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ghostRow
  #[sdk(attr(qname = ":ghostRow"))]
  pub ghost_row: Option<crate::simple_type::BooleanValue>,
  /// Metadata Ghost Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ghostCol
  #[sdk(attr(qname = ":ghostCol"))]
  pub ghost_column: Option<crate::simple_type::BooleanValue>,
  /// Metadata Edit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :edit
  #[sdk(attr(qname = ":edit"))]
  pub edit: Option<crate::simple_type::BooleanValue>,
  /// Metadata Cell Value Delete
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delete
  #[sdk(attr(qname = ":delete"))]
  pub delete: Option<crate::simple_type::BooleanValue>,
  /// Metadata Copy
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :copy
  #[sdk(attr(qname = ":copy"))]
  pub copy: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste All
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteAll
  #[sdk(attr(qname = ":pasteAll"))]
  pub paste_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Formulas
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteFormulas
  #[sdk(attr(qname = ":pasteFormulas"))]
  pub paste_formulas: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Special Values
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteValues
  #[sdk(attr(qname = ":pasteValues"))]
  pub paste_values: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteFormats
  #[sdk(attr(qname = ":pasteFormats"))]
  pub paste_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Comments
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteComments
  #[sdk(attr(qname = ":pasteComments"))]
  pub paste_comments: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Data Validation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteDataValidation
  #[sdk(attr(qname = ":pasteDataValidation"))]
  pub paste_data_validation: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Borders
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteBorders
  #[sdk(attr(qname = ":pasteBorders"))]
  pub paste_borders: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Column Widths
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteColWidths
  #[sdk(attr(qname = ":pasteColWidths"))]
  pub paste_col_widths: Option<crate::simple_type::BooleanValue>,
  /// Metadata Paste Number Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pasteNumberFormats
  #[sdk(attr(qname = ":pasteNumberFormats"))]
  pub paste_number_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Merge
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :merge
  #[sdk(attr(qname = ":merge"))]
  pub merge: Option<crate::simple_type::BooleanValue>,
  /// Meatadata Split First
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :splitFirst
  #[sdk(attr(qname = ":splitFirst"))]
  pub split_first: Option<crate::simple_type::BooleanValue>,
  /// Metadata Split All
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :splitAll
  #[sdk(attr(qname = ":splitAll"))]
  pub split_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Insert Delete
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowColShift
  #[sdk(attr(qname = ":rowColShift"))]
  pub row_column_shift: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear All
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clearAll
  #[sdk(attr(qname = ":clearAll"))]
  pub clear_all: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Formats
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clearFormats
  #[sdk(attr(qname = ":clearFormats"))]
  pub clear_formats: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Contents
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clearContents
  #[sdk(attr(qname = ":clearContents"))]
  pub clear_contents: Option<crate::simple_type::BooleanValue>,
  /// Metadata Clear Comments
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :clearComments
  #[sdk(attr(qname = ":clearComments"))]
  pub clear_comments: Option<crate::simple_type::BooleanValue>,
  /// Metadata Formula Assignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :assign
  #[sdk(attr(qname = ":assign"))]
  pub assign: Option<crate::simple_type::BooleanValue>,
  /// Metadata Coercion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :coerce
  #[sdk(attr(qname = ":coerce"))]
  pub coerce: Option<crate::simple_type::BooleanValue>,
  /// Adjust Metadata
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :adjust
  #[sdk(attr(qname = ":adjust"))]
  pub adjust: Option<crate::simple_type::BooleanValue>,
  /// Cell Metadata
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cellMeta
  #[sdk(attr(qname = ":cellMeta"))]
  pub cell_meta: Option<crate::simple_type::BooleanValue>,
}
/// Metadata Block.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:bk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataBlock/x:bk")]
pub struct MetadataBlock {
  /// _
  #[sdk(child(qname = "x:CT_MetadataRecord/x:rc"))]
  pub x_rc: Vec<MetadataRecord>,
}
/// Metadata Record.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rc.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataRecord/x:rc")]
pub struct MetadataRecord {
  /// Metadata Record Type Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub type_index: crate::simple_type::UInt32Value,
  /// Metadata Record Value Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :v
  #[sdk(attr(qname = ":v"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Future Metadata Block.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:bk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FutureMetadataBlock/x:bk")]
pub struct FutureMetadataBlock {
  ///Future Feature Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// MDX Metadata Record.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mdx.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Mdx/x:mdx")]
pub struct Mdx {
  /// Connection Name Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// Cube Function Tag
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :f
  #[sdk(attr(qname = ":f"))]
  pub cube_function: MdxFunctionValues,
  #[sdk(choice)]
  pub xml_children: Option<MdxChoice>,
}
/// Tuple MDX Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:t.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxTuple/x:t")]
pub struct MdxTuple {
  /// Member Index Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub member_index_count: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Culture Currency
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ct
  #[sdk(attr(qname = ":ct"))]
  pub culture_currency: Option<crate::simple_type::StringValue>,
  /// Server Formatting String Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :si
  #[sdk(attr(qname = ":si"))]
  pub formatting_string_index: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Built-In Number Format Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fi
  #[sdk(attr(qname = ":fi"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// Server Formatting Background Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// Server Formatting Foreground Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// Server Formatting Italic Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Underline Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Strikethrough Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// Server Formatting Bold Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataStringIndex/x:n"))]
  pub x_n: Vec<NameIndex>,
}
/// Set MDX Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ms.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxSet/x:ms")]
pub struct MdxSet {
  /// Set Definition Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ns
  #[sdk(attr(qname = ":ns"))]
  pub set_definition_index: crate::simple_type::UInt32Value,
  /// Sort By Member Index Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub member_index_count: Option<crate::simple_type::UInt32Value>,
  /// Set Sort Order
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :o
  #[sdk(attr(qname = ":o"))]
  pub sorting_order: Option<MdxSetOrderValues>,
  /// _
  #[sdk(child(qname = "x:CT_MetadataStringIndex/x:n"))]
  pub x_n: Vec<NameIndex>,
}
/// Member Property MDX Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:p.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxMemeberProp/x:p")]
pub struct MdxMemberProp {
  /// Member Unique Name Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// Property Name Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :np
  #[sdk(attr(qname = ":np"))]
  pub property_name_index: crate::simple_type::UInt32Value,
}
/// KPI MDX Metadata.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:k.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MdxKPI/x:k")]
pub struct MdxKpi {
  /// Member Unique Name Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name_index: crate::simple_type::UInt32Value,
  /// KPI Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :np
  #[sdk(attr(qname = ":np"))]
  pub kpi_index: crate::simple_type::UInt32Value,
  /// KPI Property
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :p
  #[sdk(attr(qname = ":p"))]
  pub kpi_property: MdxKpiPropertyValues,
}
/// Member Unique Name Index.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:n.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MetadataStringIndex/x:n")]
pub struct NameIndex {
  /// Index Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub index: crate::simple_type::UInt32Value,
  /// String is a Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub is_a_set: Option<crate::simple_type::BooleanValue>,
}
/// Table Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:singleXmlCell.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SingleXmlCell/x:singleXmlCell")]
pub struct SingleXmlCell {
  /// Table Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Connection ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: crate::simple_type::UInt32Value,
  ///Cell Properties
  #[sdk(child(qname = "x:CT_XmlCellPr/x:xmlCellPr"))]
  pub xml_cell_properties: std::boxed::Box<XmlCellProperties>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Cell Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:xmlCellPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlCellPr/x:xmlCellPr")]
pub struct XmlCellProperties {
  /// Table Field Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Unique Table Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  ///Column XML Properties
  #[sdk(child(qname = "x:CT_XmlPr/x:xmlPr"))]
  pub xml_properties: std::boxed::Box<XmlProperties>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Column XML Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:xmlPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlPr/x:xmlPr")]
pub struct XmlProperties {
  /// XML Map Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mapId
  #[sdk(attr(qname = ":mapId"))]
  pub map_id: crate::simple_type::UInt32Value,
  /// XPath
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xpath
  #[sdk(attr(qname = ":xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// XML Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xmlDataType
  #[sdk(attr(qname = ":xmlDataType"))]
  pub xml_data_type: XmlDataValues,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Pattern.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:patternFill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PatternFill/x:patternFill")]
pub struct PatternFill {
  /// Pattern Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :patternType
  #[sdk(attr(qname = ":patternType"))]
  pub pattern_type: Option<PatternValues>,
  ///Foreground Color
  #[sdk(child(qname = "x:CT_Color/x:fgColor"))]
  pub foreground_color: Option<ForegroundColor>,
  ///Background Color
  #[sdk(child(qname = "x:CT_Color/x:bgColor"))]
  pub background_color: Option<BackgroundColor>,
}
/// Gradient.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:gradientFill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GradientFill/x:gradientFill")]
pub struct GradientFill {
  /// Gradient Fill Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<GradientValues>,
  /// Linear Gradient Degree
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :degree
  #[sdk(attr(qname = ":degree"))]
  pub degree: Option<crate::simple_type::DoubleValue>,
  /// Left Convergence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::DoubleValue>,
  /// Right Convergence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::DoubleValue>,
  /// Top Gradient Convergence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::DoubleValue>,
  /// Bottom Convergence
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "x:CT_GradientStop/x:stop"))]
  pub x_stop: Vec<GradientStop>,
}
/// Gradient Stop.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:stop.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GradientStop/x:stop")]
pub struct GradientStop {
  /// Gradient Stop Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: crate::simple_type::DoubleValue,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: std::boxed::Box<Color>,
}
/// Number Formats.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:numFmt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NumFmt/x:numFmt")]
pub struct NumberingFormat {
  /// Number Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: crate::simple_type::UInt32Value,
  /// Number Format Code
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
}
/// Alignment.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:alignment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellAlignment/x:alignment")]
pub struct Alignment {
  /// Horizontal Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horizontal
  #[sdk(attr(qname = ":horizontal"))]
  pub horizontal: Option<HorizontalAlignmentValues>,
  /// Vertical Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vertical
  #[sdk(attr(qname = ":vertical"))]
  pub vertical: Option<VerticalAlignmentValues>,
  /// Text Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textRotation
  #[sdk(attr(qname = ":textRotation"))]
  pub text_rotation: Option<crate::simple_type::UInt32Value>,
  /// Wrap Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrapText
  #[sdk(attr(qname = ":wrapText"))]
  pub wrap_text: Option<crate::simple_type::BooleanValue>,
  /// Indent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indent
  #[sdk(attr(qname = ":indent"))]
  pub indent: Option<crate::simple_type::UInt32Value>,
  /// Relative Indent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relativeIndent
  #[sdk(attr(qname = ":relativeIndent"))]
  pub relative_indent: Option<crate::simple_type::Int32Value>,
  /// Justify Last Line
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :justifyLastLine
  #[sdk(attr(qname = ":justifyLastLine"))]
  pub justify_last_line: Option<crate::simple_type::BooleanValue>,
  /// Shrink To Fit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shrinkToFit
  #[sdk(attr(qname = ":shrinkToFit"))]
  pub shrink_to_fit: Option<crate::simple_type::BooleanValue>,
  /// Reading Order
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :readingOrder
  #[sdk(attr(qname = ":readingOrder"))]
  pub reading_order: Option<crate::simple_type::UInt32Value>,
  /// mergeCell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mergeCell
  #[sdk(attr(qname = ":mergeCell"))]
  pub merge_cell: Option<crate::simple_type::StringValue>,
}
/// Protection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:protection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellProtection/x:protection")]
pub struct Protection {
  /// Cell Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Hidden Cell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
}
/// Font Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:font.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Font/x:font")]
pub struct Font {
  ///Bold
  #[sdk(child(qname = "x:CT_BooleanProperty/x:b"))]
  pub bold: Option<Bold>,
  ///Italic
  #[sdk(child(qname = "x:CT_BooleanProperty/x:i"))]
  pub italic: Option<Italic>,
  ///Strike Through
  #[sdk(child(qname = "x:CT_BooleanProperty/x:strike"))]
  pub strike: Option<Strike>,
  ///Condense
  #[sdk(child(qname = "x:CT_BooleanProperty/x:condense"))]
  pub condense: Option<Condense>,
  ///Extend
  #[sdk(child(qname = "x:CT_BooleanProperty/x:extend"))]
  pub extend: Option<Extend>,
  ///Outline
  #[sdk(child(qname = "x:CT_BooleanProperty/x:outline"))]
  pub outline: Option<Outline>,
  ///Shadow
  #[sdk(child(qname = "x:CT_BooleanProperty/x:shadow"))]
  pub shadow: Option<Shadow>,
  ///Underline
  #[sdk(child(qname = "x:CT_UnderlineProperty/x:u"))]
  pub underline: Option<Underline>,
  ///Text Vertical Alignment
  #[sdk(child(qname = "x:CT_VerticalAlignFontProperty/x:vertAlign"))]
  pub vertical_text_alignment: Option<VerticalTextAlignment>,
  ///Font Size
  #[sdk(child(qname = "x:CT_FontSize/x:sz"))]
  pub font_size: Option<FontSize>,
  ///Text Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
  ///Font Name
  #[sdk(child(qname = "x:CT_FontNameNonEmpty/x:name"))]
  pub font_name: Option<FontName>,
  ///Font Family
  #[sdk(child(qname = "x:CT_FontFamilyNum/x:family"))]
  pub font_family_numbering: Option<FontFamilyNumbering>,
  ///Character Set
  #[sdk(child(qname = "x:CT_ByteProperty/x:charset"))]
  pub font_char_set: Option<FontCharSet>,
  ///Scheme
  #[sdk(child(qname = "x:CT_FontScheme/x:scheme"))]
  pub font_scheme: Option<FontScheme>,
}
/// Fill.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fill.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fill/x:fill")]
pub struct Fill {
  #[sdk(choice)]
  pub xml_children: Option<FillChoice>,
}
/// Border Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:border.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Border/x:border")]
pub struct Border {
  /// Diagonal Up
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :diagonalUp
  #[sdk(attr(qname = ":diagonalUp"))]
  pub diagonal_up: Option<crate::simple_type::BooleanValue>,
  /// Diagonal Down
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :diagonalDown
  #[sdk(attr(qname = ":diagonalDown"))]
  pub diagonal_down: Option<crate::simple_type::BooleanValue>,
  /// Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_BorderPr/x:start"))]
  pub start_border: Option<std::boxed::Box<StartBorder>>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(child(qname = "x:CT_BorderPr/x:end"))]
  pub end_border: Option<std::boxed::Box<EndBorder>>,
  ///Left Border
  #[sdk(child(qname = "x:CT_BorderPr/x:left"))]
  pub left_border: Option<std::boxed::Box<LeftBorder>>,
  ///Right Border
  #[sdk(child(qname = "x:CT_BorderPr/x:right"))]
  pub right_border: Option<std::boxed::Box<RightBorder>>,
  ///Top Border
  #[sdk(child(qname = "x:CT_BorderPr/x:top"))]
  pub top_border: Option<std::boxed::Box<TopBorder>>,
  ///Bottom Border
  #[sdk(child(qname = "x:CT_BorderPr/x:bottom"))]
  pub bottom_border: Option<std::boxed::Box<BottomBorder>>,
  ///Diagonal
  #[sdk(child(qname = "x:CT_BorderPr/x:diagonal"))]
  pub diagonal_border: Option<std::boxed::Box<DiagonalBorder>>,
  ///Vertical Inner Border
  #[sdk(child(qname = "x:CT_BorderPr/x:vertical"))]
  pub vertical_border: Option<std::boxed::Box<VerticalBorder>>,
  ///Horizontal Inner Borders
  #[sdk(child(qname = "x:CT_BorderPr/x:horizontal"))]
  pub horizontal_border: Option<std::boxed::Box<HorizontalBorder>>,
}
/// Color Indexes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:indexedColors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IndexedColors/x:indexedColors")]
pub struct IndexedColors {
  /// _
  #[sdk(child(qname = "x:CT_RgbColor/x:rgbColor"))]
  pub x_rgb_color: Vec<RgbColor>,
}
/// MRU Colors.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mruColors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MRUColors/x:mruColors")]
pub struct MruColors {
  /// _
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
}
/// Table Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableStyle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyle/x:tableStyle")]
pub struct TableStyle {
  /// Table Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Pivot Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pivot
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// Table
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :table
  #[sdk(attr(qname = ":table"))]
  pub table: Option<crate::simple_type::BooleanValue>,
  /// Table Style Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_TableStyleElement/x:tableStyleElement"))]
  pub x_table_style_element: Vec<TableStyleElement>,
}
/// RGB Color.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rgbColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RgbColor/x:rgbColor")]
pub struct RgbColor {
  /// Alpha Red Green Blue
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
}
/// Cell Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellStyle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyle/x:cellStyle")]
pub struct CellStyle {
  /// User Defined Cell Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xfId
  #[sdk(attr(qname = ":xfId"))]
  pub format_id: crate::simple_type::UInt32Value,
  /// Built-In Style Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :builtinId
  #[sdk(attr(qname = ":builtinId"))]
  pub builtin_id: Option<crate::simple_type::UInt32Value>,
  /// Outline Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iLevel
  #[sdk(attr(qname = ":iLevel"))]
  pub outline_level: Option<crate::simple_type::UInt32Value>,
  /// Hidden Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Custom Built In
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customBuiltin
  #[sdk(attr(qname = ":customBuiltin"))]
  pub custom_builtin: Option<crate::simple_type::BooleanValue>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Formatting Elements.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:xf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Xf/x:xf")]
pub struct CellFormat {
  /// Number Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub number_format_id: Option<crate::simple_type::UInt32Value>,
  /// Font Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fontId
  #[sdk(attr(qname = ":fontId"))]
  pub font_id: Option<crate::simple_type::UInt32Value>,
  /// Fill Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fillId
  #[sdk(attr(qname = ":fillId"))]
  pub fill_id: Option<crate::simple_type::UInt32Value>,
  /// Border Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :borderId
  #[sdk(attr(qname = ":borderId"))]
  pub border_id: Option<crate::simple_type::UInt32Value>,
  /// Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xfId
  #[sdk(attr(qname = ":xfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Quote Prefix
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :quotePrefix
  #[sdk(attr(qname = ":quotePrefix"))]
  pub quote_prefix: Option<crate::simple_type::BooleanValue>,
  /// Pivot Button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pivotButton
  #[sdk(attr(qname = ":pivotButton"))]
  pub pivot_button: Option<crate::simple_type::BooleanValue>,
  /// Apply Number Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyNumberFormat
  #[sdk(attr(qname = ":applyNumberFormat"))]
  pub apply_number_format: Option<crate::simple_type::BooleanValue>,
  /// Apply Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyFont
  #[sdk(attr(qname = ":applyFont"))]
  pub apply_font: Option<crate::simple_type::BooleanValue>,
  /// Apply Fill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyFill
  #[sdk(attr(qname = ":applyFill"))]
  pub apply_fill: Option<crate::simple_type::BooleanValue>,
  /// Apply Border
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyBorder
  #[sdk(attr(qname = ":applyBorder"))]
  pub apply_border: Option<crate::simple_type::BooleanValue>,
  /// Apply Alignment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyAlignment
  #[sdk(attr(qname = ":applyAlignment"))]
  pub apply_alignment: Option<crate::simple_type::BooleanValue>,
  /// Apply Protection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :applyProtection
  #[sdk(attr(qname = ":applyProtection"))]
  pub apply_protection: Option<crate::simple_type::BooleanValue>,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment: Option<Alignment>,
  ///Protection
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection: Option<Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Font Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:name.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontNameNonEmpty/x:name")]
pub struct FontName {
  /// String Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(string_length(source = 1u32, min = 1u32))]
  pub val: crate::simple_type::StringValue,
}
/// Font Family.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:family.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FontFamilyNum/x:family")]
pub struct FontFamilyNumbering {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "5",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Character Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:charset.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ByteProperty/x:charset")]
pub struct FontCharSet {
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "255",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Table Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableStyleElement.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyleElement/x:tableStyleElement")]
pub struct TableStyleElement {
  /// Table Style Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: TableStyleValues,
  /// Band Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// Formatting Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defined Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:definedName.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalDefinedName/x:definedName")]
pub struct ExternalDefinedName {
  /// Defined Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Refers To
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refersTo
  #[sdk(attr(qname = ":refersTo"))]
  pub refers_to: Option<crate::simple_type::StringValue>,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: Option<crate::simple_type::UInt32Value>,
}
/// External Sheet Data Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetData/x:sheetData")]
pub struct ExternalSheetData {
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Last Refresh Resulted in Error
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshError
  #[sdk(attr(qname = ":refreshError"))]
  pub refresh_error: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExternalRow/x:row"))]
  pub x_row: Vec<ExternalRow>,
}
/// Row.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:row.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalRow/x:row")]
pub struct ExternalRow {
  /// Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub row_index: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x:CT_ExternalCell/x:cell"))]
  pub x_cell: Vec<ExternalCell>,
}
/// External Cell Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cell.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalCell/x:cell")]
pub struct ExternalCell {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub data_type: Option<CellValues>,
  /// Value Metadata
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vm
  #[sdk(attr(qname = ":vm"))]
  pub value_meta_index: Option<crate::simple_type::UInt32Value>,
  ///Value
  #[sdk(text_child(qname = "x:ST_Xstring/x:v"))]
  pub xstring: Option<crate::simple_type::StringValue>,
}
/// DDE Items Collection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ddeItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeItems/x:ddeItems")]
pub struct DdeItems {
  /// _
  #[sdk(child(qname = "x:CT_DdeItem/x:ddeItem"))]
  pub x_dde_item: Vec<DdeItem>,
}
/// DDE Item definition.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ddeItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeItem/x:ddeItem")]
pub struct DdeItem {
  /// DDE Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// OLE
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ole
  #[sdk(attr(qname = ":ole"))]
  pub use_ole: Option<crate::simple_type::BooleanValue>,
  /// Advise
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// Data is an Image
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preferPic
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
  ///DDE Name Values
  #[sdk(child(qname = "x:CT_DdeValues/x:values"))]
  pub values: Option<Values>,
}
/// DDE Name Values.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:values.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeValues/x:values")]
pub struct Values {
  /// Rows
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rows
  #[sdk(attr(qname = ":rows"))]
  pub rows: Option<crate::simple_type::UInt32Value>,
  /// Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cols
  #[sdk(attr(qname = ":cols"))]
  pub columns: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DdeValue/x:value"))]
  pub x_value: Vec<Value>,
}
/// Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:value.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeValue/x:value")]
pub struct Value {
  /// DDE Value Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub value_type: Option<DdeValues>,
  ///DDE Link Value
  #[sdk(child(qname = "x:CT_Xstring/x:val"))]
  pub dde_link_value: std::boxed::Box<DdeLinkValue>,
}
/// OLE Link Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleItems/x:oleItems")]
pub struct OleItems {
  #[sdk(choice)]
  pub xml_children: Vec<OleItemsChoice>,
}
/// External Workbook.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:externalBook.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalBook/x:externalBook")]
pub struct ExternalBook {
    /// Relationship to supporting book file path
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: r:id
    #[sdk(attr(qname = "r:id"))]
    pub id: crate::simple_type::StringValue,
    #[cfg(feature = "microsoft365")]
    ///Alternate URLs and identifiers of the external book
    #[sdk(child(qname = "xxl21:CT_ExternalBookAlternateUrls/xxl21:alternateUrls"))]
    pub external_book_alternate_urls: Option<
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2021_extlinks2021::ExternalBookAlternateUrls,
        >,
    >,
    ///Sheet names of supporting book
    #[sdk(child(qname = "x:CT_ExternalSheetNames/x:sheetNames"))]
    pub sheet_names: Option<SheetNames>,
    ///Defined names associated with supporting book.
    #[sdk(child(qname = "x:CT_ExternalDefinedNames/x:definedNames"))]
    pub external_defined_names: Option<ExternalDefinedNames>,
    ///Cached worksheet data associated with supporting book
    #[sdk(child(qname = "x:CT_ExternalSheetDataSet/x:sheetDataSet"))]
    pub sheet_data_set: Option<SheetDataSet>,
}
/// DDE Connection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ddeLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeLink/x:ddeLink")]
pub struct DdeLink {
  /// Service name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ddeService
  #[sdk(attr(qname = ":ddeService"))]
  pub dde_service: crate::simple_type::StringValue,
  /// Topic for DDE server
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ddeTopic
  #[sdk(attr(qname = ":ddeTopic"))]
  pub dde_topic: crate::simple_type::StringValue,
  ///DDE Items Collection
  #[sdk(child(qname = "x:CT_DdeItems/x:ddeItems"))]
  pub dde_items: Option<DdeItems>,
}
/// OLE Link.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleLink.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleLink/x:oleLink")]
pub struct OleLink {
  /// OLE Link Relationship
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// OLE Link ProgID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :progId
  #[sdk(attr(qname = ":progId"))]
  pub prog_id: crate::simple_type::StringValue,
  ///OLE Link Items
  #[sdk(child(qname = "x:CT_OleItems/x:oleItems"))]
  pub ole_items: Option<OleItems>,
}
/// Sheet Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetName.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetName/x:sheetName")]
pub struct SheetName {
  /// Sheet Name Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:v.
pub type Xstring = crate::simple_type::StringValue;
/// Table Column.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableColumn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableColumn/x:tableColumn")]
pub struct TableColumn {
  /// Table Field Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Unique Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// Column name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Totals Row Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowFunction
  #[sdk(attr(qname = ":totalsRowFunction"))]
  pub totals_row_function: Option<TotalsRowFunctionValues>,
  /// Totals Row Label
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowLabel
  #[sdk(attr(qname = ":totalsRowLabel"))]
  pub totals_row_label: Option<crate::simple_type::StringValue>,
  /// Query Table Field Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :queryTableFieldId
  #[sdk(attr(qname = ":queryTableFieldId"))]
  pub query_table_field_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Cell Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowDxfId
  #[sdk(attr(qname = ":headerRowDxfId"))]
  pub header_row_differential_formatting_id: Option<crate::simple_type::UInt32Value>,
  /// Data and Insert Row Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataDxfId
  #[sdk(attr(qname = ":dataDxfId"))]
  pub data_format_id: Option<crate::simple_type::UInt32Value>,
  /// Totals Row Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowDxfId
  #[sdk(attr(qname = ":totalsRowDxfId"))]
  pub totals_row_differential_formatting_id: Option<crate::simple_type::UInt32Value>,
  /// Header Row Cell Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headerRowCellStyle
  #[sdk(attr(qname = ":headerRowCellStyle"))]
  pub header_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Data Area Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataCellStyle
  #[sdk(attr(qname = ":dataCellStyle"))]
  pub data_cell_style: Option<crate::simple_type::StringValue>,
  /// Totals Row Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :totalsRowCellStyle
  #[sdk(attr(qname = ":totalsRowCellStyle"))]
  pub totals_row_cell_style: Option<crate::simple_type::StringValue>,
  /// Revision ID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr3:uid
  #[sdk(attr(qname = "xr3:uid"))]
  pub xr3_uid: Option<crate::simple_type::StringValue>,
  ///Calculated Column Formula
  #[sdk(child(qname = "x:CT_TableFormula/x:calculatedColumnFormula"))]
  pub calculated_column_formula: Option<CalculatedColumnFormula>,
  ///Totals Row Formula
  #[sdk(child(qname = "x:CT_TableFormula/x:totalsRowFormula"))]
  pub totals_row_formula: Option<TotalsRowFormula>,
  ///XML Column Properties
  #[sdk(child(qname = "x:CT_XmlColumnPr/x:xmlColumnPr"))]
  pub xml_column_properties: Option<std::boxed::Box<XmlColumnProperties>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Calculated Column Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calculatedColumnFormula.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableFormula/x:calculatedColumnFormula")]
pub struct CalculatedColumnFormula {
  /// Array
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Totals Row Formula.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:totalsRowFormula.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableFormula/x:totalsRowFormula")]
pub struct TotalsRowFormula {
  /// Array
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TableFormulaType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableFormula/")]
pub struct TableFormulaType {
  /// Array
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :array
  #[sdk(attr(qname = ":array"))]
  pub array: Option<crate::simple_type::BooleanValue>,
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:space
  #[sdk(attr(qname = "xml:space"))]
  pub space: Option<crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// XML Column Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:xmlColumnPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_XmlColumnPr/x:xmlColumnPr")]
pub struct XmlColumnProperties {
  /// XML Map Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mapId
  #[sdk(attr(qname = ":mapId"))]
  pub map_id: crate::simple_type::UInt32Value,
  /// XPath
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xpath
  #[sdk(attr(qname = ":xpath"))]
  pub x_path: crate::simple_type::StringValue,
  /// Denormalized
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :denormalized
  #[sdk(attr(qname = ":denormalized"))]
  pub denormalized: Option<crate::simple_type::BooleanValue>,
  /// XML Data Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xmlDataType
  #[sdk(attr(qname = ":xmlDataType"))]
  pub xml_data_type: XmlDataValues,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Volatile Dependency Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:volType.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolType/x:volType")]
pub struct VolatileType {
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: VolatileDependencyValues,
  /// _
  #[sdk(child(qname = "x:CT_VolMain/x:main"))]
  pub x_main: Vec<Main>,
}
/// Main.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:main.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolMain/x:main")]
pub struct Main {
  /// First String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :first
  #[sdk(attr(qname = ":first"))]
  pub first: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_VolTopic/x:tp"))]
  pub x_tp: Vec<Topic>,
}
/// Topic.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTopic/x:tp")]
pub struct Topic {
  /// Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub value_type: Option<VolatileValues>,
  ///Topic Value
  #[sdk(text_child(qname = "x:ST_Xstring/x:v"))]
  pub xstring: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_Xstring/x:stp"))]
  pub x_stp: Vec<Subtopic>,
  /// _
  #[sdk(child(qname = "x:CT_VolTopicRef/x:tr"))]
  pub x_tr: Vec<TopicReferences>,
}
/// References.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_VolTopicRef/x:tr")]
pub struct TopicReferences {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub cell_reference: crate::simple_type::StringValue,
  /// Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub sheet_id: crate::simple_type::UInt32Value,
}
/// PivotCache.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotCache.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCache/x:pivotCache")]
pub struct PivotCache {
  /// PivotCache Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cacheId
  #[sdk(attr(qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Web Publishing Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPublishObject.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishObject/x:webPublishObject")]
pub struct WebPublishObject {
  /// Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// Div Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :divId
  #[sdk(attr(qname = ":divId"))]
  pub div_id: crate::simple_type::StringValue,
  /// Source Object
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceObject
  #[sdk(attr(qname = ":sourceObject"))]
  pub source_object: Option<crate::simple_type::StringValue>,
  /// Destination File
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :destinationFile
  #[sdk(attr(qname = ":destinationFile"))]
  pub destination_file: crate::simple_type::StringValue,
  /// Title
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// Auto Republish
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoRepublish
  #[sdk(attr(qname = ":autoRepublish"))]
  pub auto_republish: Option<crate::simple_type::BooleanValue>,
}
/// External Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:externalReference.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalReference/x:externalReference")]
pub struct ExternalReference {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Custom Workbook View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customWorkbookView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomWorkbookView/x:customWorkbookView")]
pub struct CustomWorkbookView {
  /// Custom View Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Custom View GUID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// Auto Update
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoUpdate
  #[sdk(attr(qname = ":autoUpdate"))]
  pub auto_update: Option<crate::simple_type::BooleanValue>,
  /// Merge Interval
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :mergeInterval
  #[sdk(attr(qname = ":mergeInterval"))]
  pub merge_interval: Option<crate::simple_type::UInt32Value>,
  /// Changes Saved Win
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :changesSavedWin
  #[sdk(attr(qname = ":changesSavedWin"))]
  pub changes_saved_win: Option<crate::simple_type::BooleanValue>,
  /// Only Synch
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :onlySync
  #[sdk(attr(qname = ":onlySync"))]
  pub only_sync: Option<crate::simple_type::BooleanValue>,
  /// Personal View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :personalView
  #[sdk(attr(qname = ":personalView"))]
  pub personal_view: Option<crate::simple_type::BooleanValue>,
  /// Include Print Settings
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :includePrintSettings
  #[sdk(attr(qname = ":includePrintSettings"))]
  pub include_print_settings: Option<crate::simple_type::BooleanValue>,
  /// Include Hidden Rows and Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :includeHiddenRowCol
  #[sdk(attr(qname = ":includeHiddenRowCol"))]
  pub include_hidden_row_column: Option<crate::simple_type::BooleanValue>,
  /// Maximized
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maximized
  #[sdk(attr(qname = ":maximized"))]
  pub maximized: Option<crate::simple_type::BooleanValue>,
  /// Minimized
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minimized
  #[sdk(attr(qname = ":minimized"))]
  pub minimized: Option<crate::simple_type::BooleanValue>,
  /// Show Horizontal Scroll
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showHorizontalScroll
  #[sdk(attr(qname = ":showHorizontalScroll"))]
  pub show_horizontal_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Vertical Scroll
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showVerticalScroll
  #[sdk(attr(qname = ":showVerticalScroll"))]
  pub show_vertical_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Sheet Tabs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showSheetTabs
  #[sdk(attr(qname = ":showSheetTabs"))]
  pub show_sheet_tabs: Option<crate::simple_type::BooleanValue>,
  /// Top Left Corner (X Coordinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xWindow
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::Int32Value>,
  /// Top Left Corner (Y Coordinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :yWindow
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::Int32Value>,
  /// Window Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :windowWidth
  #[sdk(attr(qname = ":windowWidth"))]
  pub window_width: Option<crate::simple_type::UInt32Value>,
  /// Window Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :windowHeight
  #[sdk(attr(qname = ":windowHeight"))]
  pub window_height: Option<crate::simple_type::UInt32Value>,
  /// Sheet Tab Ratio
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tabRatio
  #[sdk(attr(qname = ":tabRatio"))]
  pub tab_ratio: Option<crate::simple_type::UInt32Value>,
  /// Active Sheet in Book View
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeSheetId
  #[sdk(attr(qname = ":activeSheetId"))]
  pub active_sheet_id: crate::simple_type::UInt32Value,
  /// Show Formula Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showFormulaBar
  #[sdk(attr(qname = ":showFormulaBar"))]
  pub show_formula_bar: Option<crate::simple_type::BooleanValue>,
  /// Show Status Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showStatusbar
  #[sdk(attr(qname = ":showStatusbar"))]
  pub show_statusbar: Option<crate::simple_type::BooleanValue>,
  /// Show Comments
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showComments
  #[sdk(attr(qname = ":showComments"))]
  pub show_comments: Option<CommentsValues>,
  /// Show Objects
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showObjects
  #[sdk(attr(qname = ":showObjects"))]
  pub show_objects: Option<ObjectDisplayValues>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Sheet Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sheet/x:sheet")]
pub struct Sheet {
  /// Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Sheet Tab Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheetId
  #[sdk(attr(qname = ":sheetId"))]
  pub sheet_id: crate::simple_type::UInt32Value,
  /// Visible State
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :state
  #[sdk(attr(qname = ":state"))]
  pub state: Option<SheetStateValues>,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Workbook View.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:workbookView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BookView/x:workbookView")]
pub struct WorkbookView {
  /// Visibility
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :visibility
  #[sdk(attr(qname = ":visibility"))]
  pub visibility: Option<VisibilityValues>,
  /// Minimized
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minimized
  #[sdk(attr(qname = ":minimized"))]
  pub minimized: Option<crate::simple_type::BooleanValue>,
  /// Show Horizontal Scroll
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showHorizontalScroll
  #[sdk(attr(qname = ":showHorizontalScroll"))]
  pub show_horizontal_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Vertical Scroll
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showVerticalScroll
  #[sdk(attr(qname = ":showVerticalScroll"))]
  pub show_vertical_scroll: Option<crate::simple_type::BooleanValue>,
  /// Show Sheet Tabs
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showSheetTabs
  #[sdk(attr(qname = ":showSheetTabs"))]
  pub show_sheet_tabs: Option<crate::simple_type::BooleanValue>,
  /// Upper Left Corner (X Coordinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xWindow
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::Int32Value>,
  /// Upper Left Corner (Y Coordinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :yWindow
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::Int32Value>,
  /// Window Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :windowWidth
  #[sdk(attr(qname = ":windowWidth"))]
  pub window_width: Option<crate::simple_type::UInt32Value>,
  /// Window Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :windowHeight
  #[sdk(attr(qname = ":windowHeight"))]
  pub window_height: Option<crate::simple_type::UInt32Value>,
  /// Sheet Tab Ratio
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tabRatio
  #[sdk(attr(qname = ":tabRatio"))]
  pub tab_ratio: Option<crate::simple_type::UInt32Value>,
  /// First Sheet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstSheet
  #[sdk(attr(qname = ":firstSheet"))]
  pub first_sheet: Option<crate::simple_type::UInt32Value>,
  /// Active Sheet Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :activeTab
  #[sdk(attr(qname = ":activeTab"))]
  pub active_tab: Option<crate::simple_type::UInt32Value>,
  /// AutoFilter Date Grouping
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFilterDateGrouping
  #[sdk(attr(qname = ":autoFilterDateGrouping"))]
  pub auto_filter_date_grouping: Option<crate::simple_type::BooleanValue>,
  /// Revision UID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr2:uid
  #[sdk(attr(qname = "xr2:uid"))]
  pub xr2_uid: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defined Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:definedName.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DefinedName/x:definedName")]
pub struct DefinedName {
  /// Defined Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Comment
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Custom Menu Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customMenu
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// Description
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Help
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :help
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// Status Bar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :statusBar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// Local Name Sheet Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :localSheetId
  #[sdk(attr(qname = ":localSheetId"))]
  pub local_sheet_id: Option<crate::simple_type::UInt32Value>,
  /// Hidden Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// Procedure
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vbProcedure
  #[sdk(attr(qname = ":vbProcedure"))]
  pub vb_procedure: Option<crate::simple_type::BooleanValue>,
  /// External Function
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xlm
  #[sdk(attr(qname = ":xlm"))]
  pub xlm: Option<crate::simple_type::BooleanValue>,
  /// Function Group Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :functionGroupId
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::UInt32Value>,
  /// Shortcut Key
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :shortcutKey
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::StringValue>,
  /// Publish To Server
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :publishToServer
  #[sdk(attr(qname = ":publishToServer"))]
  pub publish_to_server: Option<crate::simple_type::BooleanValue>,
  /// Workbook Parameter (Server)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookParameter
  #[sdk(attr(qname = ":workbookParameter"))]
  pub workbook_parameter: Option<crate::simple_type::BooleanValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Function Group.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:functionGroup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FunctionGroup/x:functionGroup")]
pub struct FunctionGroup {
  /// Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
#[cfg(feature = "microsoft365")]
/// Defines the ObjectAnchor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:anchor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ObjectAnchor/x:anchor")]
pub struct ObjectAnchor {
  /// moveWithCells
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :moveWithCells
  #[sdk(attr(qname = ":moveWithCells"))]
  pub move_with_cells: Option<crate::simple_type::BooleanValue>,
  /// sizeWithCells
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sizeWithCells
  #[sdk(attr(qname = ":sizeWithCells"))]
  pub size_with_cells: Option<crate::simple_type::BooleanValue>,
  /// z-order
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :z-order
  #[sdk(attr(qname = ":z-order"))]
  pub z_order: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xdr:CT_Marker/x:from"))]
  pub from_marker: std::boxed::Box<FromMarker>,
  /// _
  #[sdk(child(qname = "xdr:CT_Marker/x:to"))]
  pub to_marker: std::boxed::Box<ToMarker>,
}
#[cfg(feature = "microsoft365")]
/// Defines the FromMarker Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:from.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/x:from")]
pub struct FromMarker {
  ///Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  ///Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  ///Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  ///Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
#[cfg(feature = "microsoft365")]
/// Defines the ToMarker Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:to.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/x:to")]
pub struct ToMarker {
  ///Column)
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  pub column_id: crate::simple_type::Int32Value,
  ///Column Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  pub column_offset: crate::simple_type::Int64Value,
  ///Row
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  pub row_id: crate::simple_type::Int32Value,
  ///Row Offset
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  pub row_offset: crate::simple_type::Int64Value,
}
/// Defines the MarkerType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xdr:CT_Marker/")]
pub struct MarkerType {
  #[sdk(choice)]
  pub xml_children: Vec<MarkerTypeChoice>,
}
/// Defines the ConditionalFormattingRuleExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRuleExtension/x:ext")]
pub struct ConditionalFormattingRuleExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<ConditionalFormattingRuleExtensionChoice>,
}
/// Defines the PivotHierarchyExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchyExtension/x:ext")]
pub struct PivotHierarchyExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<PivotHierarchyExtensionChoice>,
}
/// Defines the PivotFieldExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFieldExtension/x:ext")]
pub struct PivotFieldExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<PivotFieldExtensionChoice>,
}
/// Defines the CacheSourceExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSourceExtension/x:ext")]
pub struct CacheSourceExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<CacheSourceExtensionChoice>,
}
/// OLE Link Item.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleItem/x:oleItem")]
pub struct OleItem {
  /// OLE Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Icon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :icon
  #[sdk(attr(qname = ":icon"))]
  pub icon: Option<crate::simple_type::BooleanValue>,
  /// Advise
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// Object is an Image
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preferPic
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
}
#[cfg(feature = "microsoft365")]
/// Defines the StartBorder Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:start.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:start")]
pub struct StartBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
#[cfg(feature = "microsoft365")]
/// Defines the EndBorder Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:end.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:end")]
pub struct EndBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Left Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:left.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:left")]
pub struct LeftBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Right Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:right.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:right")]
pub struct RightBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Top Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:top.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:top")]
pub struct TopBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Bottom Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:bottom.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:bottom")]
pub struct BottomBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Diagonal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:diagonal.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:diagonal")]
pub struct DiagonalBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Vertical Inner Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:vertical.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:vertical")]
pub struct VerticalBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Horizontal Inner Borders.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:horizontal.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/x:horizontal")]
pub struct HorizontalBorder {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Option<Color>,
}
/// Defines the BorderPropertiesType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BorderPr/")]
pub struct BorderPropertiesType {
  /// Line Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<BorderStyleValues>,
  ///Color
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub color: Vec<Color>,
}
#[cfg(feature = "microsoft365")]
/// Defines the ControlProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:controlPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ControlPr/x:controlPr")]
pub struct ControlProperties {
  /// locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// recalcAlways
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :recalcAlways
  #[sdk(attr(qname = ":recalcAlways"))]
  pub recalc_always: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// autoPict
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoPict
  #[sdk(attr(qname = ":autoPict"))]
  pub auto_pict: Option<crate::simple_type::BooleanValue>,
  /// macro
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// altText
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// linkedCell
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :linkedCell
  #[sdk(attr(qname = ":linkedCell"))]
  pub linked_cell: Option<crate::simple_type::StringValue>,
  /// listFillRange
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :listFillRange
  #[sdk(attr(qname = ":listFillRange"))]
  pub list_fill_range: Option<crate::simple_type::StringValue>,
  /// cf
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cf
  #[sdk(attr(qname = ":cf"))]
  pub cf: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
#[cfg(feature = "microsoft365")]
/// Defines the EmbeddedObjectProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:objectPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ObjectPr/x:objectPr")]
pub struct EmbeddedObjectProperties {
  /// locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// autoPict
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoPict
  #[sdk(attr(qname = ":autoPict"))]
  pub auto_pict: Option<crate::simple_type::BooleanValue>,
  /// macro
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :macro
  #[sdk(attr(qname = ":macro"))]
  pub r#macro: Option<crate::simple_type::StringValue>,
  /// altText
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// dde
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dde
  #[sdk(attr(qname = ":dde"))]
  pub dde: Option<crate::simple_type::BooleanValue>,
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
/// Chart Sheet Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetPr/x:sheetPr")]
pub struct ChartSheetProperties {
  /// Published
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Code Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codeName
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x:tabColor"))]
  pub tab_color: Option<TabColor>,
}
/// Chart Sheet Views.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetViews/x:sheetViews")]
pub struct ChartSheetViews {
  /// _
  #[sdk(child(qname = "x:CT_ChartsheetView/x:sheetView"))]
  pub x_sheet_view: Vec<ChartSheetView>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Chart Sheet Protection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetProtection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartsheetProtection/x:sheetProtection")]
pub struct ChartSheetProtection {
  /// Password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// Cryptographic Algorithm Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// Contents
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :content
  #[sdk(attr(qname = ":content"))]
  pub content: Option<crate::simple_type::BooleanValue>,
  /// Objects Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :objects
  #[sdk(attr(qname = ":objects"))]
  pub objects: Option<crate::simple_type::BooleanValue>,
}
/// Custom Chart Sheet Views.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customSheetViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomChartsheetViews/x:customSheetViews")]
pub struct CustomChartsheetViews {
  /// _
  #[sdk(child(qname = "x:CT_CustomChartsheetView/x:customSheetView"))]
  pub x_custom_sheet_view: Vec<CustomChartsheetView>,
}
/// Drawing.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:drawing.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Drawing/x:drawing")]
pub struct Drawing {
  /// Relationship id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the LegacyDrawing Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:legacyDrawing.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LegacyDrawing/x:legacyDrawing")]
pub struct LegacyDrawing {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Legacy Drawing Reference in  Header Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:legacyDrawingHF.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the LegacyDrawingType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_LegacyDrawing/")]
pub struct LegacyDrawingType {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the DrawingHeaderFooter Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:drawingHF.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DrawingHF/x:drawingHF")]
pub struct DrawingHeaderFooter {
  /// id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// lho
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lho
  #[sdk(attr(qname = ":lho"))]
  pub lho: Option<crate::simple_type::UInt32Value>,
  /// lhe
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lhe
  #[sdk(attr(qname = ":lhe"))]
  pub lhe: Option<crate::simple_type::UInt32Value>,
  /// lhf
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lhf
  #[sdk(attr(qname = ":lhf"))]
  pub lhf: Option<crate::simple_type::UInt32Value>,
  /// cho
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cho
  #[sdk(attr(qname = ":cho"))]
  pub cho: Option<crate::simple_type::UInt32Value>,
  /// che
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :che
  #[sdk(attr(qname = ":che"))]
  pub che: Option<crate::simple_type::UInt32Value>,
  /// chf
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :chf
  #[sdk(attr(qname = ":chf"))]
  pub chf: Option<crate::simple_type::UInt32Value>,
  /// rho
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rho
  #[sdk(attr(qname = ":rho"))]
  pub rho: Option<crate::simple_type::UInt32Value>,
  /// rhe
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rhe
  #[sdk(attr(qname = ":rhe"))]
  pub rhe: Option<crate::simple_type::UInt32Value>,
  /// rhf
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rhf
  #[sdk(attr(qname = ":rhf"))]
  pub rhf: Option<crate::simple_type::UInt32Value>,
  /// lfo
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lfo
  #[sdk(attr(qname = ":lfo"))]
  pub lfo: Option<crate::simple_type::UInt32Value>,
  /// lfe
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lfe
  #[sdk(attr(qname = ":lfe"))]
  pub lfe: Option<crate::simple_type::UInt32Value>,
  /// lff
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lff
  #[sdk(attr(qname = ":lff"))]
  pub lff: Option<crate::simple_type::UInt32Value>,
  /// cfo
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cfo
  #[sdk(attr(qname = ":cfo"))]
  pub cfo: Option<crate::simple_type::UInt32Value>,
  /// cfe
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cfe
  #[sdk(attr(qname = ":cfe"))]
  pub cfe: Option<crate::simple_type::UInt32Value>,
  /// cff
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cff
  #[sdk(attr(qname = ":cff"))]
  pub cff: Option<crate::simple_type::UInt32Value>,
  /// rfo
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rfo
  #[sdk(attr(qname = ":rfo"))]
  pub rfo: Option<crate::simple_type::UInt32Value>,
  /// rfe
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rfe
  #[sdk(attr(qname = ":rfe"))]
  pub rfe: Option<crate::simple_type::UInt32Value>,
  /// rff
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rff
  #[sdk(attr(qname = ":rff"))]
  pub rff: Option<crate::simple_type::UInt32Value>,
}
/// Defines the Picture Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:picture.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetBackgroundPicture/x:picture")]
pub struct Picture {
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the WebPublishItems Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPublishItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishItems/x:webPublishItems")]
pub struct WebPublishItems {
  /// Web Publishing Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishItem/x:webPublishItem"))]
  pub x_web_publish_item: Vec<WebPublishItem>,
}
/// Color Scale.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colorScale.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColorScale/x:colorScale")]
pub struct ColorScale {
  /// _
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: Vec<Color>,
}
/// Data Bar.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataBar.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataBar/x:dataBar")]
pub struct DataBar {
  /// Minimum Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minLength
  #[sdk(attr(qname = ":minLength"))]
  pub min_length: Option<crate::simple_type::UInt32Value>,
  /// Maximum Length
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxLength
  #[sdk(attr(qname = ":maxLength"))]
  pub max_length: Option<crate::simple_type::UInt32Value>,
  /// Show Values
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x:color"))]
  pub x_color: std::boxed::Box<Color>,
}
/// Icon Set.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:iconSet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IconSet/x:iconSet")]
pub struct IconSet {
  /// Icon Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set_value: Option<IconSetValues>,
  /// Show Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// Percent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Reverse Icons
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :reverse
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Cfvo/x:cfvo"))]
  pub x_cfvo: Vec<ConditionalFormatValueObject>,
}
/// Defines the ConditionalFormattingRuleExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CfRuleExtensionList/x:extLst")]
pub struct ConditionalFormattingRuleExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_CfRuleExtension/x:ext"))]
  pub x_ext: Vec<ConditionalFormattingRuleExtension>,
}
/// Data Consolidation References.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataRefs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataRefs/x:dataRefs")]
pub struct DataReferences {
  /// Data Consolidation Reference Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DataRef/x:dataRef"))]
  pub x_data_ref: Vec<DataReference>,
}
/// Sheet Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetPr/x:sheetPr")]
pub struct SheetProperties {
  /// Synch Horizontal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :syncHorizontal
  #[sdk(attr(qname = ":syncHorizontal"))]
  pub sync_horizontal: Option<crate::simple_type::BooleanValue>,
  /// Synch Vertical
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :syncVertical
  #[sdk(attr(qname = ":syncVertical"))]
  pub sync_vertical: Option<crate::simple_type::BooleanValue>,
  /// Synch Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :syncRef
  #[sdk(attr(qname = ":syncRef"))]
  pub sync_reference: Option<crate::simple_type::StringValue>,
  /// Transition Formula Evaluation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :transitionEvaluation
  #[sdk(attr(qname = ":transitionEvaluation"))]
  pub transition_evaluation: Option<crate::simple_type::BooleanValue>,
  /// Transition Formula Entry
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :transitionEntry
  #[sdk(attr(qname = ":transitionEntry"))]
  pub transition_entry: Option<crate::simple_type::BooleanValue>,
  /// Published
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :published
  #[sdk(attr(qname = ":published"))]
  pub published: Option<crate::simple_type::BooleanValue>,
  /// Code Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codeName
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// Filter Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filterMode
  #[sdk(attr(qname = ":filterMode"))]
  pub filter_mode: Option<crate::simple_type::BooleanValue>,
  /// Enable Conditional Formatting Calculations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :enableFormatConditionsCalculation
  #[sdk(attr(qname = ":enableFormatConditionsCalculation"))]
  pub enable_format_conditions_calculation: Option<crate::simple_type::BooleanValue>,
  ///Sheet Tab Color
  #[sdk(child(qname = "x:CT_Color/x:tabColor"))]
  pub tab_color: Option<TabColor>,
  ///Outline Properties
  #[sdk(child(qname = "x:CT_OutlinePr/x:outlinePr"))]
  pub outline_properties: Option<OutlineProperties>,
  ///Page Setup Properties
  #[sdk(child(qname = "x:CT_PageSetUpPr/x:pageSetUpPr"))]
  pub page_setup_properties: Option<PageSetupProperties>,
}
/// Dialog Sheet Views.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetViews/x:sheetViews")]
pub struct SheetViews {
  /// _
  #[sdk(child(qname = "x:CT_SheetView/x:sheetView"))]
  pub x_sheet_view: Vec<SheetView>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Dialog Sheet Format Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetFormatPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetFormatPr/x:sheetFormatPr")]
pub struct SheetFormatProperties {
  /// Base Column Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseColWidth
  #[sdk(attr(qname = ":baseColWidth"))]
  pub base_column_width: Option<crate::simple_type::UInt32Value>,
  /// Default Column Width
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultColWidth
  #[sdk(attr(qname = ":defaultColWidth"))]
  pub default_column_width: Option<crate::simple_type::DoubleValue>,
  /// Default Row Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultRowHeight
  #[sdk(attr(qname = ":defaultRowHeight"))]
  pub default_row_height: crate::simple_type::DoubleValue,
  /// Custom Height
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customHeight
  #[sdk(attr(qname = ":customHeight"))]
  pub custom_height: Option<crate::simple_type::BooleanValue>,
  /// Hidden By Default
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :zeroHeight
  #[sdk(attr(qname = ":zeroHeight"))]
  pub zero_height: Option<crate::simple_type::BooleanValue>,
  /// Thick Top Border
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thickTop
  #[sdk(attr(qname = ":thickTop"))]
  pub thick_top: Option<crate::simple_type::BooleanValue>,
  /// Thick Bottom Border
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thickBottom
  #[sdk(attr(qname = ":thickBottom"))]
  pub thick_bottom: Option<crate::simple_type::BooleanValue>,
  /// Maximum Outline Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineLevelRow
  #[sdk(attr(qname = ":outlineLevelRow"))]
  pub outline_level_row: Option<crate::simple_type::ByteValue>,
  /// Column Outline Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outlineLevelCol
  #[sdk(attr(qname = ":outlineLevelCol"))]
  pub outline_level_column: Option<crate::simple_type::ByteValue>,
  #[cfg(feature = "microsoft365")]
  /// dyDescent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: x14ac:dyDescent
  #[sdk(attr(qname = "x14ac:dyDescent"))]
  pub dy_descent: Option<crate::simple_type::DoubleValue>,
}
/// Sheet Protection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetProtection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetProtection/x:sheetProtection")]
pub struct SheetProtection {
  /// Password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// Cryptographic Algorithm Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// Sheet Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheet
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::BooleanValue>,
  /// Objects Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :objects
  #[sdk(attr(qname = ":objects"))]
  pub objects: Option<crate::simple_type::BooleanValue>,
  /// Scenarios Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :scenarios
  #[sdk(attr(qname = ":scenarios"))]
  pub scenarios: Option<crate::simple_type::BooleanValue>,
  /// Format Cells Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatCells
  #[sdk(attr(qname = ":formatCells"))]
  pub format_cells: Option<crate::simple_type::BooleanValue>,
  /// Format Columns Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatColumns
  #[sdk(attr(qname = ":formatColumns"))]
  pub format_columns: Option<crate::simple_type::BooleanValue>,
  /// Format Rows Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatRows
  #[sdk(attr(qname = ":formatRows"))]
  pub format_rows: Option<crate::simple_type::BooleanValue>,
  /// Insert Columns Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertColumns
  #[sdk(attr(qname = ":insertColumns"))]
  pub insert_columns: Option<crate::simple_type::BooleanValue>,
  /// Insert Rows Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertRows
  #[sdk(attr(qname = ":insertRows"))]
  pub insert_rows: Option<crate::simple_type::BooleanValue>,
  /// Insert Hyperlinks Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :insertHyperlinks
  #[sdk(attr(qname = ":insertHyperlinks"))]
  pub insert_hyperlinks: Option<crate::simple_type::BooleanValue>,
  /// Delete Columns Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :deleteColumns
  #[sdk(attr(qname = ":deleteColumns"))]
  pub delete_columns: Option<crate::simple_type::BooleanValue>,
  /// Delete Rows Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :deleteRows
  #[sdk(attr(qname = ":deleteRows"))]
  pub delete_rows: Option<crate::simple_type::BooleanValue>,
  /// Select Locked Cells Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :selectLockedCells
  #[sdk(attr(qname = ":selectLockedCells"))]
  pub select_locked_cells: Option<crate::simple_type::BooleanValue>,
  /// Sort Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sort
  #[sdk(attr(qname = ":sort"))]
  pub sort: Option<crate::simple_type::BooleanValue>,
  /// AutoFilter Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFilter
  #[sdk(attr(qname = ":autoFilter"))]
  pub auto_filter: Option<crate::simple_type::BooleanValue>,
  /// Pivot Tables Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pivotTables
  #[sdk(attr(qname = ":pivotTables"))]
  pub pivot_tables: Option<crate::simple_type::BooleanValue>,
  /// Select Unlocked Cells Locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :selectUnlockedCells
  #[sdk(attr(qname = ":selectUnlockedCells"))]
  pub select_unlocked_cells: Option<crate::simple_type::BooleanValue>,
}
/// Custom Sheet Views.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customSheetViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomSheetViews/x:customSheetViews")]
pub struct CustomSheetViews {
  /// _
  #[sdk(child(qname = "x:CT_CustomSheetView/x:customSheetView"))]
  pub x_custom_sheet_view: Vec<CustomSheetView>,
}
/// Defines the OleObjects Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleObjects.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleObjects/x:oleObjects")]
pub struct OleObjects {
  /// _
  #[sdk(child(qname = "x:CT_OleObject/x:oleObject"))]
  pub x_ole_object: Vec<OleObject>,
}
/// Defines the Controls Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:controls.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Controls/x:controls")]
pub struct Controls {
  /// _
  #[sdk(child(qname = "x:CT_Control/x:control"))]
  pub x_control: Vec<Control>,
}
/// Macro Sheet Dimensions.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dimension.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetDimension/x:dimension")]
pub struct SheetDimension {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Column Information.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cols.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Cols/x:cols")]
pub struct Columns {
  /// _
  #[sdk(child(qname = "x:CT_Col/x:col"))]
  pub x_col: Vec<Column>,
}
/// Sheet Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetData/x:sheetData")]
pub struct SheetData {
  /// _
  #[sdk(child(qname = "x:CT_Row/x:row"))]
  pub x_row: Vec<Row>,
}
/// Data Consolidation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataConsolidate.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataConsolidate/x:dataConsolidate")]
pub struct DataConsolidate {
  /// Function Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<DataConsolidateFunctionValues>,
  /// Use Left Column Labels
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :leftLabels
  #[sdk(attr(qname = ":leftLabels"))]
  pub left_labels: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// startLabels
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :startLabels
  #[sdk(attr(qname = ":startLabels"))]
  pub start_labels: Option<crate::simple_type::BooleanValue>,
  /// Labels In Top Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :topLabels
  #[sdk(attr(qname = ":topLabels"))]
  pub top_labels: Option<crate::simple_type::BooleanValue>,
  /// Link
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :link
  #[sdk(attr(qname = ":link"))]
  pub link: Option<crate::simple_type::BooleanValue>,
  ///Data Consolidation References
  #[sdk(child(qname = "x:CT_DataRefs/x:dataRefs"))]
  pub data_references: Option<DataReferences>,
}
/// Conditional Formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormatting.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormatting/x:conditionalFormatting")]
pub struct ConditionalFormatting {
  /// PivotTable Conditional Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :pivot
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// Sequence of References
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "x:CT_CfRule/x:cfRule"))]
  pub x_cf_rule: Vec<ConditionalFormattingRule>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Custom Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customProperties.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomProperties/x:customProperties")]
pub struct CustomProperties {
  /// _
  #[sdk(child(qname = "x:CT_CustomProperty/x:customPr"))]
  pub x_custom_pr: Vec<CustomProperty>,
}
/// OLAP Member Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mps.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MemberProperties/x:mps")]
pub struct MemberProperties {
  /// OLAP Member Properties Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MemberProperty/x:mp"))]
  pub x_mp: Vec<MemberProperty>,
}
/// Members.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:members.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Members/x:members")]
pub struct Members {
  /// Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Hierarchy Level
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Member/x:member"))]
  pub x_member: Vec<Member>,
}
/// Future Feature Data Storage Area.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchyExtensionList/x:extLst")]
pub struct PivotHierarchyExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_PivotHierarchyExtension/x:ext"))]
  pub x_ext: Vec<PivotHierarchyExtension>,
}
/// Field Items.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:items.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Items/x:items")]
pub struct Items {
  /// Field Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Item/x:item"))]
  pub x_item: Vec<Item>,
}
/// AutoSort Scope.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:autoSortScope.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoSortScope/x:autoSortScope")]
pub struct AutoSortScope {
  ///Auto Sort Scope
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
}
/// Future Feature Data Storage Area.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFieldExtensionList/x:extLst")]
pub struct PivotFieldExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_PivotFieldExtension/x:ext"))]
  pub x_ext: Vec<PivotFieldExtension>,
}
/// Defines the WorksheetSource Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:worksheetSource.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetSource/x:worksheetSource")]
pub struct WorksheetSource {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// Named Range
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Sheet Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sheet
  #[sdk(attr(qname = ":sheet"))]
  pub sheet: Option<crate::simple_type::StringValue>,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
/// Defines the Consolidation Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:consolidation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Consolidation/x:consolidation")]
pub struct Consolidation {
  /// Auto Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoPage
  #[sdk(attr(qname = ":autoPage"))]
  pub auto_page: Option<crate::simple_type::BooleanValue>,
  ///Page Item Values
  #[sdk(child(qname = "x:CT_Pages/x:pages"))]
  pub pages: Option<Pages>,
  ///Range Sets
  #[sdk(child(qname = "x:CT_RangeSets/x:rangeSets"))]
  pub range_sets: std::boxed::Box<RangeSets>,
}
/// Defines the CacheSourceExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSourceExtensionList/x:extLst")]
pub struct CacheSourceExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_CacheSourceExtension/x:ext"))]
  pub x_ext: Vec<CacheSourceExtension>,
}
#[cfg(feature = "microsoft365")]
/// Defines the CommentProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:commentPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CommentPr/x:commentPr")]
pub struct CommentProperties {
  /// locked
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// defaultSize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSize
  #[sdk(attr(qname = ":defaultSize"))]
  pub default_size: Option<crate::simple_type::BooleanValue>,
  /// print
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :print
  #[sdk(attr(qname = ":print"))]
  pub print: Option<crate::simple_type::BooleanValue>,
  /// disabled
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disabled
  #[sdk(attr(qname = ":disabled"))]
  pub disabled: Option<crate::simple_type::BooleanValue>,
  /// uiObject
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uiObject
  #[sdk(attr(qname = ":uiObject"))]
  pub ui_object: Option<crate::simple_type::BooleanValue>,
  /// autoFill
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoFill
  #[sdk(attr(qname = ":autoFill"))]
  pub auto_fill: Option<crate::simple_type::BooleanValue>,
  /// autoLine
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoLine
  #[sdk(attr(qname = ":autoLine"))]
  pub auto_line: Option<crate::simple_type::BooleanValue>,
  /// altText
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// textHAlign
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textHAlign
  #[sdk(attr(qname = ":textHAlign"))]
  pub text_h_align: Option<TextHorizontalAlignmentValues>,
  /// textVAlign
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textVAlign
  #[sdk(attr(qname = ":textVAlign"))]
  pub text_v_align: Option<TextVerticalAlignmentValues>,
  /// lockText
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lockText
  #[sdk(attr(qname = ":lockText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// justLastX
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :justLastX
  #[sdk(attr(qname = ":justLastX"))]
  pub just_last_x: Option<crate::simple_type::BooleanValue>,
  /// autoScale
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoScale
  #[sdk(attr(qname = ":autoScale"))]
  pub auto_scale: Option<crate::simple_type::BooleanValue>,
  /// rowHidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowHidden
  #[sdk(attr(qname = ":rowHidden"))]
  pub row_hidden: Option<crate::simple_type::BooleanValue>,
  /// colHidden
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colHidden
  #[sdk(attr(qname = ":colHidden"))]
  pub col_hidden: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_ObjectAnchor/x:anchor"))]
  pub object_anchor: std::boxed::Box<ObjectAnchor>,
}
/// Defines the SortCondition Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sortCondition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SortCondition/x:sortCondition")]
pub struct SortCondition {
  /// Descending
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// Sort By
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by: Option<SortByValues>,
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Custom List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// Format Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// Icon Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<IconSetValues>,
  /// Icon Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Filter.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:filter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Filter/x:filter")]
pub struct Filter {
  /// Filter Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Date Grouping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dateGroupItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DateGroupItem/x:dateGroupItem")]
pub struct DateGroupItem {
  /// Year
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :year
  #[sdk(attr(qname = ":year"))]
  pub year: crate::simple_type::UInt16Value,
  /// Month
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :month
  #[sdk(attr(qname = ":month"))]
  pub month: Option<crate::simple_type::UInt16Value>,
  /// Day
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :day
  #[sdk(attr(qname = ":day"))]
  pub day: Option<crate::simple_type::UInt16Value>,
  /// Hour
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hour
  #[sdk(attr(qname = ":hour"))]
  pub hour: Option<crate::simple_type::UInt16Value>,
  /// Minute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minute
  #[sdk(attr(qname = ":minute"))]
  pub minute: Option<crate::simple_type::UInt16Value>,
  /// Second
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :second
  #[sdk(attr(qname = ":second"))]
  pub second: Option<crate::simple_type::UInt16Value>,
  /// Date Time Grouping
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dateTimeGrouping
  #[sdk(attr(qname = ":dateTimeGrouping"))]
  pub date_time_grouping: DateTimeGroupingValues,
}
/// Filter Criteria.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:filters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Filters/x:filters")]
pub struct Filters {
  /// Filter by Blank
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blank
  #[sdk(attr(qname = ":blank"))]
  pub blank: Option<crate::simple_type::BooleanValue>,
  /// Calendar Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calendarType
  #[sdk(attr(qname = ":calendarType"))]
  pub calendar_type: Option<CalendarValues>,
  #[sdk(choice)]
  pub xml_children: Option<FiltersChoice>,
}
/// Top 10.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:top10.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Top10/x:top10")]
pub struct Top10 {
  /// Top
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::BooleanValue>,
  /// Filter by Percent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Top or Bottom Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
  /// Filter Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filterVal
  #[sdk(attr(qname = ":filterVal"))]
  pub filter_value: Option<crate::simple_type::DoubleValue>,
}
/// Custom Filters.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customFilters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomFilters/x:customFilters")]
pub struct CustomFilters {
  /// And
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :and
  #[sdk(attr(qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_CustomFilter/x:customFilter"))]
  pub x_custom_filter: Vec<CustomFilter>,
}
/// Dynamic Filter.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dynamicFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DynamicFilter/x:dynamicFilter")]
pub struct DynamicFilter {
  /// Dynamic filter type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: DynamicFilterValues,
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Max Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxVal
  #[sdk(attr(qname = ":maxVal"))]
  pub max_val: Option<crate::simple_type::DoubleValue>,
  #[cfg(feature = "microsoft365")]
  /// valIso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :valIso
  #[sdk(attr(qname = ":valIso"))]
  pub val_iso: Option<crate::simple_type::DateTimeValue>,
  #[cfg(feature = "microsoft365")]
  /// maxValIso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxValIso
  #[sdk(attr(qname = ":maxValIso"))]
  pub max_val_iso: Option<crate::simple_type::DateTimeValue>,
}
/// Color Filter Criteria.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colorFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColorFilter/x:colorFilter")]
pub struct ColorFilter {
  /// Differential Format Record Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: crate::simple_type::UInt32Value,
  /// Filter By Cell Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cellColor
  #[sdk(attr(qname = ":cellColor"))]
  pub cell_color: Option<crate::simple_type::BooleanValue>,
}
/// Icon Filter.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:iconFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IconFilter/x:iconFilter")]
pub struct IconFilter {
  /// Icon Set
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetValues,
  /// Icon Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SlicerCacheDefinitionExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SlicerCacheDefinitionExtension/x:ext")]
pub struct SlicerCacheDefinitionExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<SlicerCacheDefinitionExtensionChoice>,
}
/// Defines the PivotFilterExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilterExtension/x:ext")]
pub struct PivotFilterExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<PivotFilterExtensionChoice>,
}
/// Defines the QueryTableExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableExtension/x:ext")]
pub struct QueryTableExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<QueryTableExtensionChoice>,
}
/// Defines the DatabaseProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dbPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DbPr/x:dbPr")]
pub struct DatabaseProperties {
  /// Connection String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connection
  #[sdk(attr(qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
  /// Command Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :command
  #[sdk(attr(qname = ":command"))]
  pub command: Option<crate::simple_type::StringValue>,
  /// Command Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverCommand
  #[sdk(attr(qname = ":serverCommand"))]
  pub server_command: Option<crate::simple_type::StringValue>,
  /// OLE DB Command Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :commandType
  #[sdk(attr(qname = ":commandType"))]
  pub command_type: Option<crate::simple_type::UInt32Value>,
}
/// Defines the OlapProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:olapPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OlapPr/x:olapPr")]
pub struct OlapProperties {
  /// Local Cube
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :local
  #[sdk(attr(qname = ":local"))]
  pub local: Option<crate::simple_type::BooleanValue>,
  /// Local Cube Connection
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :localConnection
  #[sdk(attr(qname = ":localConnection"))]
  pub local_connection: Option<crate::simple_type::StringValue>,
  /// Local Refresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :localRefresh
  #[sdk(attr(qname = ":localRefresh"))]
  pub local_refresh: Option<crate::simple_type::BooleanValue>,
  /// Send Locale to OLAP
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sendLocale
  #[sdk(attr(qname = ":sendLocale"))]
  pub send_locale: Option<crate::simple_type::BooleanValue>,
  /// Drill Through Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowDrillCount
  #[sdk(attr(qname = ":rowDrillCount"))]
  pub row_drill_count: Option<crate::simple_type::UInt32Value>,
  /// OLAP Fill Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverFill
  #[sdk(attr(qname = ":serverFill"))]
  pub server_fill: Option<crate::simple_type::BooleanValue>,
  /// OLAP Number Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverNumberFormat
  #[sdk(attr(qname = ":serverNumberFormat"))]
  pub server_number_format: Option<crate::simple_type::BooleanValue>,
  /// OLAP Server Font
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverFont
  #[sdk(attr(qname = ":serverFont"))]
  pub server_font: Option<crate::simple_type::BooleanValue>,
  /// OLAP Font Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :serverFontColor
  #[sdk(attr(qname = ":serverFontColor"))]
  pub server_font_color: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebQueryProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPr/x:webPr")]
pub struct WebQueryProperties {
  /// XML Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xml
  #[sdk(attr(qname = ":xml"))]
  pub xml_source: Option<crate::simple_type::BooleanValue>,
  /// Import XML Source Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceData
  #[sdk(attr(qname = ":sourceData"))]
  pub source_data: Option<crate::simple_type::BooleanValue>,
  /// Parse PRE
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :parsePre
  #[sdk(attr(qname = ":parsePre"))]
  pub parse_pre_tag: Option<crate::simple_type::BooleanValue>,
  /// Consecutive Delimiters
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :consecutive
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// Use First Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstRow
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::BooleanValue>,
  /// Created in Excel 97
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xl97
  #[sdk(attr(qname = ":xl97"))]
  pub created_in_excel97: Option<crate::simple_type::BooleanValue>,
  /// Dates as Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :textDates
  #[sdk(attr(qname = ":textDates"))]
  pub text_dates: Option<crate::simple_type::BooleanValue>,
  /// Refreshed in Excel 2000
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xl2000
  #[sdk(attr(qname = ":xl2000"))]
  pub refreshed_in_excel2000: Option<crate::simple_type::BooleanValue>,
  /// URL
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :url
  #[sdk(attr(qname = ":url"))]
  pub url: Option<crate::simple_type::StringValue>,
  /// Web Post
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :post
  #[sdk(attr(qname = ":post"))]
  pub post: Option<crate::simple_type::StringValue>,
  /// HTML Tables Only
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :htmlTables
  #[sdk(attr(qname = ":htmlTables"))]
  pub html_tables: Option<crate::simple_type::BooleanValue>,
  /// HTML Formatting Handling
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :htmlFormat
  #[sdk(attr(qname = ":htmlFormat"))]
  pub html_format: Option<HtmlFormattingValues>,
  /// Edit Query URL
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :editPage
  #[sdk(attr(qname = ":editPage"))]
  pub edit_page: Option<crate::simple_type::StringValue>,
  ///Tables
  #[sdk(child(qname = "x:CT_Tables/x:tables"))]
  pub tables: Option<Tables>,
}
/// Defines the TextProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:textPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextPr/x:textPr")]
pub struct TextProperties {
  /// prompt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::BooleanValue>,
  /// fileType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fileType
  #[sdk(attr(qname = ":fileType"))]
  pub file_type: Option<FileTypeValues>,
  /// codePage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub text_character_set: Option<crate::simple_type::StringValue>,
  /// firstRow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstRow
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::UInt32Value>,
  /// sourceFile
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// delimited
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delimited
  #[sdk(attr(qname = ":delimited"))]
  pub delimited: Option<crate::simple_type::BooleanValue>,
  /// decimal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :decimal
  #[sdk(attr(qname = ":decimal"))]
  pub decimal: Option<crate::simple_type::StringValue>,
  /// thousands
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thousands
  #[sdk(attr(qname = ":thousands"))]
  pub thousands: Option<crate::simple_type::StringValue>,
  /// tab
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tab
  #[sdk(attr(qname = ":tab"))]
  pub tab_as_delimiter: Option<crate::simple_type::BooleanValue>,
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :space
  #[sdk(attr(qname = ":space"))]
  pub space: Option<crate::simple_type::BooleanValue>,
  /// comma
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comma
  #[sdk(attr(qname = ":comma"))]
  pub comma: Option<crate::simple_type::BooleanValue>,
  /// semicolon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :semicolon
  #[sdk(attr(qname = ":semicolon"))]
  pub semicolon: Option<crate::simple_type::BooleanValue>,
  /// consecutive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :consecutive
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// qualifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :qualifier
  #[sdk(attr(qname = ":qualifier"))]
  pub qualifier: Option<QualifierValues>,
  /// delimiter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delimiter
  #[sdk(attr(qname = ":delimiter"))]
  pub delimiter: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_TextFields/x:textFields"))]
  pub text_fields: Option<TextFields>,
}
/// Defines the Parameters Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:parameters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Parameters/x:parameters")]
pub struct Parameters {
  /// Parameter Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Parameter/x:parameter"))]
  pub x_parameter: Vec<Parameter>,
}
/// Defines the ConnectionExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConnectionExtensionList/x:extLst")]
pub struct ConnectionExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_ConnectionExtension/x:ext"))]
  pub x_ext: Vec<ConnectionExtension>,
}
/// Defines the ConnectionExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConnectionExtension/x:ext")]
pub struct ConnectionExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<ConnectionExtensionChoice>,
}
/// Defines the TextFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:textFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextFields/x:textFields")]
pub struct TextFields {
  /// Count of Fields
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_TextField/x:textField"))]
  pub x_text_field: Vec<TextField>,
}
/// Defines the SharedItems Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sharedItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SharedItems/x:sharedItems")]
pub struct SharedItems {
  /// Contains Semi Mixed Data Types
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsSemiMixedTypes
  #[sdk(attr(qname = ":containsSemiMixedTypes"))]
  pub contains_semi_mixed_types: Option<crate::simple_type::BooleanValue>,
  /// Contains Non Date
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsNonDate
  #[sdk(attr(qname = ":containsNonDate"))]
  pub contains_non_date: Option<crate::simple_type::BooleanValue>,
  /// Contains Date
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsDate
  #[sdk(attr(qname = ":containsDate"))]
  pub contains_date: Option<crate::simple_type::BooleanValue>,
  /// Contains String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsString
  #[sdk(attr(qname = ":containsString"))]
  pub contains_string: Option<crate::simple_type::BooleanValue>,
  /// Contains Blank
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsBlank
  #[sdk(attr(qname = ":containsBlank"))]
  pub contains_blank: Option<crate::simple_type::BooleanValue>,
  /// Contains Mixed Data Types
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsMixedTypes
  #[sdk(attr(qname = ":containsMixedTypes"))]
  pub contains_mixed_types: Option<crate::simple_type::BooleanValue>,
  /// Contains Numbers
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsNumber
  #[sdk(attr(qname = ":containsNumber"))]
  pub contains_number: Option<crate::simple_type::BooleanValue>,
  /// Contains Integer
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :containsInteger
  #[sdk(attr(qname = ":containsInteger"))]
  pub contains_integer: Option<crate::simple_type::BooleanValue>,
  /// Minimum Numeric Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minValue
  #[sdk(attr(qname = ":minValue"))]
  pub min_value: Option<crate::simple_type::DoubleValue>,
  /// Maximum Numeric Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxValue
  #[sdk(attr(qname = ":maxValue"))]
  pub max_value: Option<crate::simple_type::DoubleValue>,
  /// Minimum Date Time
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minDate
  #[sdk(attr(qname = ":minDate"))]
  pub min_date: Option<crate::simple_type::DateTimeValue>,
  /// Maximum Date Time Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxDate
  #[sdk(attr(qname = ":maxDate"))]
  pub max_date: Option<crate::simple_type::DateTimeValue>,
  /// Shared Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Long Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :longText
  #[sdk(attr(qname = ":longText"))]
  pub long_text: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub xml_children: Vec<SharedItemsChoice>,
}
/// Defines the FieldGroup Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fieldGroup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldGroup/x:fieldGroup")]
pub struct FieldGroup {
  /// Parent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :par
  #[sdk(attr(qname = ":par"))]
  pub parent_id: Option<crate::simple_type::UInt32Value>,
  /// Field Base
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :base
  #[sdk(attr(qname = ":base"))]
  pub base: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub field_group_choice: Option<FieldGroupChoice>,
  /// _
  #[sdk(child(qname = "x:CT_GroupItems/x:groupItems"))]
  pub x_group_items: Option<GroupItems>,
}
/// Defines the CacheFieldExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFieldExtensionList/x:extLst")]
pub struct CacheFieldExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_CacheFieldExtension/x:ext"))]
  pub x_ext: Vec<CacheFieldExtension>,
}
/// Defines the CacheFieldExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFieldExtension/x:ext")]
pub struct CacheFieldExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<CacheFieldExtensionChoice>,
}
/// Defines the FieldsUsage Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fieldsUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FieldsUsage/x:fieldsUsage")]
pub struct FieldsUsage {
  /// Field Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_FieldUsage/x:fieldUsage"))]
  pub x_field_usage: Vec<FieldUsage>,
}
/// Defines the GroupLevels Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:groupLevels.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_GroupLevels/x:groupLevels")]
pub struct GroupLevels {
  /// Grouping Level Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_GroupLevel/x:groupLevel"))]
  pub x_group_level: Vec<GroupLevel>,
}
/// Defines the CacheHierarchyExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchyExtensionList/x:extLst")]
pub struct CacheHierarchyExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_CacheHierarchyExtension/x:ext"))]
  pub x_ext: Vec<CacheHierarchyExtension>,
}
/// Defines the CacheHierarchyExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchyExtension/x:ext")]
pub struct CacheHierarchyExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<CacheHierarchyExtensionChoice>,
}
/// Defines the CalculatedMemberExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMemberExtensionList/x:extLst")]
pub struct CalculatedMemberExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMemberExtension/x:ext"))]
  pub x_ext: Vec<CalculatedMemberExtension>,
}
/// Defines the CalculatedMemberExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMemberExtension/x:ext")]
pub struct CalculatedMemberExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<CalculatedMemberExtensionChoice>,
}
/// Defines the DataFieldExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFieldExtensionList/x:extLst")]
pub struct DataFieldExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_DataFieldExtension/x:ext"))]
  pub x_ext: Vec<DataFieldExtension>,
}
/// Defines the DataFieldExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFieldExtension/x:ext")]
pub struct DataFieldExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<DataFieldExtensionChoice>,
}
/// Defines the PivotFilterExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilterExtensionList/x:extLst")]
pub struct PivotFilterExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_PivotFilterExtension/x:ext"))]
  pub x_ext: Vec<PivotFilterExtension>,
}
/// Defines the QueryTableRefresh Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:queryTableRefresh.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableRefresh/x:queryTableRefresh")]
pub struct QueryTableRefresh {
  /// Preserve Sort and Filter Layout
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :preserveSortFilterLayout
  #[sdk(attr(qname = ":preserveSortFilterLayout"))]
  pub preserve_sort_filter_layout: Option<crate::simple_type::BooleanValue>,
  /// Next Field Id Wrapped
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldIdWrapped
  #[sdk(attr(qname = ":fieldIdWrapped"))]
  pub field_id_wrapped: Option<crate::simple_type::BooleanValue>,
  /// Headers In Last Refresh
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :headersInLastRefresh
  #[sdk(attr(qname = ":headersInLastRefresh"))]
  pub headers_in_last_refresh: Option<crate::simple_type::BooleanValue>,
  /// Minimum Refresh Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minimumVersion
  #[sdk(attr(qname = ":minimumVersion"))]
  pub minimum_version: Option<crate::simple_type::ByteValue>,
  /// Next field id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :nextId
  #[sdk(attr(qname = ":nextId"))]
  pub next_id: Option<crate::simple_type::UInt32Value>,
  /// Columns Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :unboundColumnsLeft
  #[sdk(attr(qname = ":unboundColumnsLeft"))]
  pub unbound_columns_left: Option<crate::simple_type::UInt32Value>,
  /// Columns Right
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :unboundColumnsRight
  #[sdk(attr(qname = ":unboundColumnsRight"))]
  pub unbound_columns_right: Option<crate::simple_type::UInt32Value>,
  ///Query table fields
  #[sdk(child(qname = "x:CT_QueryTableFields/x:queryTableFields"))]
  pub query_table_fields: std::boxed::Box<QueryTableFields>,
  ///Deleted Fields
  #[sdk(child(qname = "x:CT_QueryTableDeletedFields/x:queryTableDeletedFields"))]
  pub query_table_deleted_fields: Option<QueryTableDeletedFields>,
  ///Sort State
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub sort_state: Option<std::boxed::Box<SortState>>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QueryTableExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_QueryTableExtensionList/x:extLst")]
pub struct QueryTableExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_QueryTableExtension/x:ext"))]
  pub x_ext: Vec<QueryTableExtension>,
}
/// Defines the SheetCalculationProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetCalcPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SheetCalcPr/x:sheetCalcPr")]
pub struct SheetCalculationProperties {
  /// Full Calculation On Load
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fullCalcOnLoad
  #[sdk(attr(qname = ":fullCalcOnLoad"))]
  pub full_calculation_on_load: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ProtectedRanges Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:protectedRanges.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ProtectedRanges/x:protectedRanges")]
pub struct ProtectedRanges {
  /// _
  #[sdk(child(qname = "x:CT_ProtectedRange/x:protectedRange"))]
  pub x_protected_range: Vec<ProtectedRange>,
}
/// Defines the Scenarios Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:scenarios.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Scenarios/x:scenarios")]
pub struct Scenarios {
  /// Current Scenario
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :current
  #[sdk(attr(qname = ":current"))]
  pub current: Option<crate::simple_type::UInt32Value>,
  /// Last Shown Scenario
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :show
  #[sdk(attr(qname = ":show"))]
  pub show: Option<crate::simple_type::UInt32Value>,
  /// Sequence of References
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sequence_of_references:
    Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "x:CT_Scenario/x:scenario"))]
  pub x_scenario: Vec<Scenario>,
}
/// Defines the MergeCells Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:mergeCells.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MergeCells/x:mergeCells")]
pub struct MergeCells {
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MergeCell/x:mergeCell"))]
  pub x_merge_cell: Vec<MergeCell>,
}
/// Defines the DataValidations Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataValidations.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataValidations/x:dataValidations")]
pub struct DataValidations {
  /// Disable Prompts
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :disablePrompts
  #[sdk(attr(qname = ":disablePrompts"))]
  pub disable_prompts: Option<crate::simple_type::BooleanValue>,
  /// Top Left Corner (X Coodrinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :xWindow
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::UInt32Value>,
  /// Top Left Corner (Y Coordinate)
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :yWindow
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::UInt32Value>,
  /// Data Validation Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DataValidation/x:dataValidation"))]
  pub x_data_validation: Vec<DataValidation>,
}
/// Defines the Hyperlinks Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:hyperlinks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Hyperlinks/x:hyperlinks")]
pub struct Hyperlinks {
  /// _
  #[sdk(child(qname = "x:CT_Hyperlink/x:hyperlink"))]
  pub x_hyperlink: Vec<Hyperlink>,
}
/// Defines the CellWatches Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellWatches.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellWatches/x:cellWatches")]
pub struct CellWatches {
  /// _
  #[sdk(child(qname = "x:CT_CellWatch/x:cellWatch"))]
  pub x_cell_watch: Vec<CellWatch>,
}
/// Defines the IgnoredErrors Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ignoredErrors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_IgnoredErrors/x:ignoredErrors")]
pub struct IgnoredErrors {
  /// _
  #[sdk(child(qname = "x:CT_IgnoredError/x:ignoredError"))]
  pub x_ignored_error: Vec<IgnoredError>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<ExtensionList>,
}
/// Defines the TableParts Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableParts.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableParts/x:tableParts")]
pub struct TableParts {
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_TablePart/x:tablePart"))]
  pub x_table_part: Vec<TablePart>,
}
/// Defines the WorksheetExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetExtensionList/x:extLst")]
pub struct WorksheetExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_WorksheetExtension/x:ext"))]
  pub x_ext: Vec<WorksheetExtension>,
}
/// Defines the WorksheetExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorksheetExtension/x:ext")]
pub struct WorksheetExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<WorksheetExtensionChoice>,
}
/// Defines the NumberingFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:numFmts.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_NumFmts/x:numFmts")]
pub struct NumberingFormats {
  /// Number Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub x_num_fmt: Vec<NumberingFormat>,
}
/// Defines the Fonts Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fonts.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fonts/x:fonts")]
pub struct Fonts {
  /// Font Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  #[cfg(feature = "microsoft365")]
  /// knownFonts
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: x14ac:knownFonts
  #[sdk(attr(qname = "x14ac:knownFonts"))]
  pub known_fonts: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub x_font: Vec<Font>,
}
/// Defines the Fills Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fills.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Fills/x:fills")]
pub struct Fills {
  /// Fill Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub x_fill: Vec<Fill>,
}
/// Defines the Borders Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:borders.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Borders/x:borders")]
pub struct Borders {
  /// Border Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub x_border: Vec<Border>,
}
/// Defines the CellStyleFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellStyleXfs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyleXfs/x:cellStyleXfs")]
pub struct CellStyleFormats {
  /// Style Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Xf/x:xf"))]
  pub x_xf: Vec<CellFormat>,
}
/// Defines the CellFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellXfs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellXfs/x:cellXfs")]
pub struct CellFormats {
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Xf/x:xf"))]
  pub x_xf: Vec<CellFormat>,
}
/// Defines the CellStyles Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cellStyles.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CellStyles/x:cellStyles")]
pub struct CellStyles {
  /// Style Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CellStyle/x:cellStyle"))]
  pub x_cell_style: Vec<CellStyle>,
}
/// Defines the DifferentialFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dxfs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxfs/x:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf: Vec<DifferentialFormat>,
}
/// Defines the TableStyles Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableStyles.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyles/x:tableStyles")]
pub struct TableStyles {
  /// Table Style Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Default Table Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultTableStyle
  #[sdk(attr(qname = ":defaultTableStyle"))]
  pub default_table_style: Option<crate::simple_type::StringValue>,
  /// Default Pivot Style
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultPivotStyle
  #[sdk(attr(qname = ":defaultPivotStyle"))]
  pub default_pivot_style: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_TableStyle/x:tableStyle"))]
  pub x_table_style: Vec<TableStyle>,
}
/// Defines the Colors Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Colors/x:colors")]
pub struct Colors {
  ///Color Indexes
  #[sdk(child(qname = "x:CT_IndexedColors/x:indexedColors"))]
  pub indexed_colors: Option<IndexedColors>,
  ///MRU Colors
  #[sdk(child(qname = "x:CT_MRUColors/x:mruColors"))]
  pub mru_colors: Option<MruColors>,
}
/// Defines the StylesheetExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_StylesheetExtensionList/x:extLst")]
pub struct StylesheetExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_StylesheetExtension/x:ext"))]
  pub x_ext: Vec<StylesheetExtension>,
}
/// Defines the StylesheetExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_StylesheetExtension/x:ext")]
pub struct StylesheetExtension {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<StylesheetExtensionChoice>,
}
/// Defines the Location Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:location.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Location/x:location")]
pub struct Location {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// First Header Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstHeaderRow
  #[sdk(attr(qname = ":firstHeaderRow"))]
  pub first_header_row: crate::simple_type::UInt32Value,
  /// PivotTable Data First Row
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstDataRow
  #[sdk(attr(qname = ":firstDataRow"))]
  pub first_data_row: crate::simple_type::UInt32Value,
  /// First Data Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstDataCol
  #[sdk(attr(qname = ":firstDataCol"))]
  pub first_data_column: crate::simple_type::UInt32Value,
  /// Rows Per Page Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rowPageCount
  #[sdk(attr(qname = ":rowPageCount"))]
  pub row_page_count: Option<crate::simple_type::UInt32Value>,
  /// Columns Per Page
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colPageCount
  #[sdk(attr(qname = ":colPageCount"))]
  pub columns_per_page: Option<crate::simple_type::UInt32Value>,
}
/// Defines the PivotFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFields/x:pivotFields")]
pub struct PivotFields {
  /// Field Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotField/x:pivotField"))]
  pub x_pivot_field: Vec<PivotField>,
}
/// Defines the RowFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rowFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RowFields/x:rowFields")]
pub struct RowFields {
  /// Repeated Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Field/x:field"))]
  pub x_field: Vec<Field>,
}
/// Defines the RowItems Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rowItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_rowItems/x:rowItems")]
pub struct RowItems {
  /// Items in a Row Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_I/x:i"))]
  pub x_i: Vec<RowItem>,
}
/// Defines the ColumnFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColFields/x:colFields")]
pub struct ColumnFields {
  /// Repeated Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Field/x:field"))]
  pub x_field: Vec<Field>,
}
/// Defines the ColumnItems Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_colItems/x:colItems")]
pub struct ColumnItems {
  /// Column Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_I/x:i"))]
  pub x_i: Vec<RowItem>,
}
/// Defines the PageFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pageFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PageFields/x:pageFields")]
pub struct PageFields {
  /// Page Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PageField/x:pageField"))]
  pub x_page_field: Vec<PageField>,
}
/// Defines the DataFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dataFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataFields/x:dataFields")]
pub struct DataFields {
  /// Data Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DataField/x:dataField"))]
  pub x_data_field: Vec<DataField>,
}
/// Defines the Formats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:formats.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Formats/x:formats")]
pub struct Formats {
  /// Formats Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Format/x:format"))]
  pub x_format: Vec<Format>,
}
/// Defines the ConditionalFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:conditionalFormats.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ConditionalFormats/x:conditionalFormats")]
pub struct ConditionalFormats {
  /// Conditional Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_ConditionalFormat/x:conditionalFormat"))]
  pub x_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the ChartFormats Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:chartFormats.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ChartFormats/x:chartFormats")]
pub struct ChartFormats {
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_ChartFormat/x:chartFormat"))]
  pub x_chart_format: Vec<ChartFormat>,
}
/// Defines the PivotHierarchies Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotHierarchies.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotHierarchies/x:pivotHierarchies")]
pub struct PivotHierarchies {
  /// OLAP Hierarchy Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotHierarchy/x:pivotHierarchy"))]
  pub x_pivot_hierarchy: Vec<PivotHierarchy>,
}
/// Defines the PivotTableStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotTableStyleInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotTableStyle/x:pivotTableStyleInfo")]
pub struct PivotTableStyle {
  /// Table Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show Row Header Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRowHeaders
  #[sdk(attr(qname = ":showRowHeaders"))]
  pub show_row_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Table Style Column Header Formatting
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showColHeaders
  #[sdk(attr(qname = ":showColHeaders"))]
  pub show_column_headers: Option<crate::simple_type::BooleanValue>,
  /// Show Row Stripes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRowStripes
  #[sdk(attr(qname = ":showRowStripes"))]
  pub show_row_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Column Stripes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showColStripes
  #[sdk(attr(qname = ":showColStripes"))]
  pub show_column_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Last Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showLastColumn
  #[sdk(attr(qname = ":showLastColumn"))]
  pub show_last_column: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotFilters Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:filters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotFilters/x:filters")]
pub struct PivotFilters {
  /// Pivot Filter Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotFilter/x:filter"))]
  pub x_filter: Vec<PivotFilter>,
}
/// Defines the RowHierarchiesUsage Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:rowHierarchiesUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage")]
pub struct RowHierarchiesUsage {
  /// Item Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_HierarchyUsage/x:rowHierarchyUsage"))]
  pub x_row_hierarchy_usage: Vec<RowHierarchyUsage>,
}
/// Defines the ColumnHierarchiesUsage Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:colHierarchiesUsage.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage")]
pub struct ColumnHierarchiesUsage {
  /// Items Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_HierarchyUsage/x:colHierarchyUsage"))]
  pub x_col_hierarchy_usage: Vec<ColumnHierarchyUsage>,
}
/// Defines the PivotTableDefinitionExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinitionExtensionList/x:extLst")]
pub struct PivotTableDefinitionExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_pivotTableDefinitionExtension/x:ext"))]
  pub x_ext: Vec<PivotTableDefinitionExtension>,
}
/// Defines the PivotTableDefinitionExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinitionExtension/x:ext")]
pub struct PivotTableDefinitionExtension {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<PivotTableDefinitionExtensionChoice>,
}
/// Defines the CacheSource Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cacheSource.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheSource/x:cacheSource")]
pub struct CacheSource {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: SourceValues,
  /// connectionId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :connectionId
  #[sdk(attr(qname = ":connectionId"))]
  pub connection_id: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice)]
  pub xml_children: Option<CacheSourceChoice>,
}
/// Defines the CacheFields Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cacheFields.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheFields/x:cacheFields")]
pub struct CacheFields {
  /// Field Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CacheField/x:cacheField"))]
  pub x_cache_field: Vec<CacheField>,
}
/// Defines the CacheHierarchies Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:cacheHierarchies.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CacheHierarchies/x:cacheHierarchies")]
pub struct CacheHierarchies {
  /// Hierarchy Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CacheHierarchy/x:cacheHierarchy"))]
  pub x_cache_hierarchy: Vec<CacheHierarchy>,
}
/// Defines the Kpis Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:kpis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PCDKPIs/x:kpis")]
pub struct Kpis {
  /// KPI Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PCDKPI/x:kpi"))]
  pub x_kpi: Vec<Kpi>,
}
/// Defines the TupleCache Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tupleCache.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TupleCache/x:tupleCache")]
pub struct TupleCache {
  ///Entries
  #[sdk(child(qname = "x:CT_PCDSDTCEntries/x:entries"))]
  pub entries: Option<Entries>,
  ///Sets
  #[sdk(child(qname = "x:CT_Sets/x:sets"))]
  pub sets: Option<Sets>,
  ///OLAP Query Cache
  #[sdk(child(qname = "x:CT_QueryCache/x:queryCache"))]
  pub query_cache: Option<QueryCache>,
  ///Server Formats
  #[sdk(child(qname = "x:CT_ServerFormats/x:serverFormats"))]
  pub server_formats: Option<ServerFormats>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CalculatedItems Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calculatedItems.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedItems/x:calculatedItems")]
pub struct CalculatedItems {
  /// Calculated Item Formula Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedItem/x:calculatedItem"))]
  pub x_calculated_item: Vec<CalculatedItem>,
}
/// Defines the CalculatedMembers Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calculatedMembers.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMembers/x:calculatedMembers")]
pub struct CalculatedMembers {
  /// Calculated Members Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMember/x:calculatedMember"))]
  pub x_calculated_member: Vec<CalculatedMember>,
}
/// Defines the Dimensions Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:dimensions.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dimensions/x:dimensions")]
pub struct Dimensions {
  /// OLAP Dimensions Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotDimension/x:dimension"))]
  pub x_dimension: Vec<Dimension>,
}
/// Defines the MeasureGroups Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:measureGroups.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureGroups/x:measureGroups")]
pub struct MeasureGroups {
  /// Measure Group Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MeasureGroup/x:measureGroup"))]
  pub x_measure_group: Vec<MeasureGroup>,
}
/// Defines the Maps Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:maps.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_MeasureDimensionMaps/x:maps")]
pub struct Maps {
  /// Measure Group Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_MeasureDimensionMap/x:map"))]
  pub x_map: Vec<MeasureDimensionMap>,
}
/// Defines the PivotCacheDefinitionExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinitionExtensionList/x:extLst")]
pub struct PivotCacheDefinitionExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_PivotCacheDefinitionExtension/x:ext"))]
  pub x_ext: Vec<PivotCacheDefinitionExtension>,
}
/// Defines the PivotCacheDefinitionExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCacheDefinitionExtension/x:ext")]
pub struct PivotCacheDefinitionExtension {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<PivotCacheDefinitionExtensionChoice>,
}
/// Sheet names of supporting book.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetNames.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetNames/x:sheetNames")]
pub struct SheetNames {
  /// _
  #[sdk(child(qname = "x:CT_ExternalSheetName/x:sheetName"))]
  pub x_sheet_name: Vec<SheetName>,
}
/// Defined names associated with supporting book..
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:definedNames.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalDefinedNames/x:definedNames")]
pub struct ExternalDefinedNames {
  /// _
  #[sdk(child(qname = "x:CT_ExternalDefinedName/x:definedName"))]
  pub x_defined_name: Vec<ExternalDefinedName>,
}
/// Cached worksheet data associated with supporting book.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheetDataSet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalSheetDataSet/x:sheetDataSet")]
pub struct SheetDataSet {
  /// _
  #[sdk(child(qname = "x:CT_ExternalSheetData/x:sheetData"))]
  pub x_sheet_data: Vec<ExternalSheetData>,
}
/// Table Columns.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableColumns.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableColumns/x:tableColumns")]
pub struct TableColumns {
  /// Column Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_TableColumn/x:tableColumn"))]
  pub x_table_column: Vec<TableColumn>,
}
/// Table Style.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:tableStyleInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableStyleInfo/x:tableStyleInfo")]
pub struct TableStyleInfo {
  /// Style Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Show First Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showFirstColumn
  #[sdk(attr(qname = ":showFirstColumn"))]
  pub show_first_column: Option<crate::simple_type::BooleanValue>,
  /// Show Last Column
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showLastColumn
  #[sdk(attr(qname = ":showLastColumn"))]
  pub show_last_column: Option<crate::simple_type::BooleanValue>,
  /// Show Row Stripes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showRowStripes
  #[sdk(attr(qname = ":showRowStripes"))]
  pub show_row_stripes: Option<crate::simple_type::BooleanValue>,
  /// Show Column Stripes
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showColumnStripes
  #[sdk(attr(qname = ":showColumnStripes"))]
  pub show_column_stripes: Option<crate::simple_type::BooleanValue>,
}
/// Future Feature Data Storage Area.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableExtensionList/x:extLst")]
pub struct TableExtensionList {
  /// _
  #[sdk(child(qname = "x:CT_TableExtension/x:ext"))]
  pub x_ext: Vec<TableExtension>,
}
/// Defines the TableExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TableExtension/x:ext")]
pub struct TableExtension {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<TableExtensionChoice>,
}
/// Defines the FileVersion Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fileVersion.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileVersion/x:fileVersion")]
pub struct FileVersion {
  /// Application Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :appName
  #[sdk(attr(qname = ":appName"))]
  pub application_name: Option<crate::simple_type::StringValue>,
  /// Last Edited Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lastEdited
  #[sdk(attr(qname = ":lastEdited"))]
  pub last_edited: Option<crate::simple_type::StringValue>,
  /// Lowest Edited Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lowestEdited
  #[sdk(attr(qname = ":lowestEdited"))]
  pub lowest_edited: Option<crate::simple_type::StringValue>,
  /// Build Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rupBuild
  #[sdk(attr(qname = ":rupBuild"))]
  pub build_version: Option<crate::simple_type::StringValue>,
  /// Code Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codeName
  #[sdk(attr(qname = ":codeName"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub code_name: Option<crate::simple_type::StringValue>,
}
/// Defines the FileSharing Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fileSharing.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileSharing/x:fileSharing")]
pub struct FileSharing {
  /// Read Only Recommended
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :readOnlyRecommended
  #[sdk(attr(qname = ":readOnlyRecommended"))]
  pub read_only_recommended: Option<crate::simple_type::BooleanValue>,
  /// User Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: Option<crate::simple_type::StringValue>,
  /// Write Reservation Password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :reservationPassword
  #[sdk(attr(qname = ":reservationPassword"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub reservation_password: Option<crate::simple_type::HexBinaryValue>,
  /// Password hash algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password hash
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt for password hash
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Spin count for password hash
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
}
/// Defines the WorkbookProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:workbookPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookPr/x:workbookPr")]
pub struct WorkbookProperties {
  /// Date 1904
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :date1904
  #[sdk(attr(qname = ":date1904"))]
  pub date1904: Option<crate::simple_type::BooleanValue>,
  #[cfg(feature = "microsoft365")]
  /// dateCompatibility
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dateCompatibility
  #[sdk(attr(qname = ":dateCompatibility"))]
  pub date_compatibility: Option<crate::simple_type::BooleanValue>,
  /// Show Objects
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showObjects
  #[sdk(attr(qname = ":showObjects"))]
  pub show_objects: Option<ObjectDisplayValues>,
  /// Show Border Unselected Table
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showBorderUnselectedTables
  #[sdk(attr(qname = ":showBorderUnselectedTables"))]
  pub show_border_unselected_tables: Option<crate::simple_type::BooleanValue>,
  /// Filter Privacy
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filterPrivacy
  #[sdk(attr(qname = ":filterPrivacy"))]
  pub filter_privacy: Option<crate::simple_type::BooleanValue>,
  /// Prompted Solutions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :promptedSolutions
  #[sdk(attr(qname = ":promptedSolutions"))]
  pub prompted_solutions: Option<crate::simple_type::BooleanValue>,
  /// Show Ink Annotations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showInkAnnotation
  #[sdk(attr(qname = ":showInkAnnotation"))]
  pub show_ink_annotation: Option<crate::simple_type::BooleanValue>,
  /// Create Backup File
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :backupFile
  #[sdk(attr(qname = ":backupFile"))]
  pub backup_file: Option<crate::simple_type::BooleanValue>,
  /// Save External Link Values
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :saveExternalLinkValues
  #[sdk(attr(qname = ":saveExternalLinkValues"))]
  pub save_external_link_values: Option<crate::simple_type::BooleanValue>,
  /// Update Links Behavior
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :updateLinks
  #[sdk(attr(qname = ":updateLinks"))]
  pub update_links: Option<UpdateLinksBehaviorValues>,
  /// Code Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codeName
  #[sdk(attr(qname = ":codeName"))]
  pub code_name: Option<crate::simple_type::StringValue>,
  /// Hide Pivot Field List
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hidePivotFieldList
  #[sdk(attr(qname = ":hidePivotFieldList"))]
  pub hide_pivot_field_list: Option<crate::simple_type::BooleanValue>,
  /// Show Pivot Chart Filter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showPivotChartFilter
  #[sdk(attr(qname = ":showPivotChartFilter"))]
  pub show_pivot_chart_filter: Option<crate::simple_type::BooleanValue>,
  /// Allow Refresh Query
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowRefreshQuery
  #[sdk(attr(qname = ":allowRefreshQuery"))]
  pub allow_refresh_query: Option<crate::simple_type::BooleanValue>,
  /// Publish Items
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :publishItems
  #[sdk(attr(qname = ":publishItems"))]
  pub publish_items: Option<crate::simple_type::BooleanValue>,
  /// Check Compatibility On Save
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :checkCompatibility
  #[sdk(attr(qname = ":checkCompatibility"))]
  pub check_compatibility: Option<crate::simple_type::BooleanValue>,
  /// Auto Compress Pictures
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoCompressPictures
  #[sdk(attr(qname = ":autoCompressPictures"))]
  pub auto_compress_pictures: Option<crate::simple_type::BooleanValue>,
  /// Refresh all Connections on Open
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refreshAllConnections
  #[sdk(attr(qname = ":refreshAllConnections"))]
  pub refresh_all_connections: Option<crate::simple_type::BooleanValue>,
  /// Default Theme Version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :defaultThemeVersion
  #[sdk(attr(qname = ":defaultThemeVersion"))]
  pub default_theme_version: Option<crate::simple_type::UInt32Value>,
}
/// Defines the WorkbookProtection Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:workbookProtection.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookProtection/x:workbookProtection")]
pub struct WorkbookProtection {
  /// Workbook Password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookPassword
  #[sdk(attr(qname = ":workbookPassword"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub workbook_password: Option<crate::simple_type::HexBinaryValue>,
  /// Revisions Password
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionsPassword
  #[sdk(attr(qname = ":revisionsPassword"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub revisions_password: Option<crate::simple_type::HexBinaryValue>,
  /// Lock Structure
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lockStructure
  #[sdk(attr(qname = ":lockStructure"))]
  pub lock_structure: Option<crate::simple_type::BooleanValue>,
  /// Lock Windows
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lockWindows
  #[sdk(attr(qname = ":lockWindows"))]
  pub lock_windows: Option<crate::simple_type::BooleanValue>,
  /// Lock Revisions
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lockRevision
  #[sdk(attr(qname = ":lockRevision"))]
  pub lock_revision: Option<crate::simple_type::BooleanValue>,
  /// Cryptographic Algorithm Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionsAlgorithmName
  #[sdk(attr(qname = ":revisionsAlgorithmName"))]
  pub revisions_algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionsHashValue
  #[sdk(attr(qname = ":revisionsHashValue"))]
  pub revisions_hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionsSaltValue
  #[sdk(attr(qname = ":revisionsSaltValue"))]
  pub revisions_salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :revisionsSpinCount
  #[sdk(attr(qname = ":revisionsSpinCount"))]
  pub revisions_spin_count: Option<crate::simple_type::UInt32Value>,
  /// Cryptographic Algorithm Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookAlgorithmName
  #[sdk(attr(qname = ":workbookAlgorithmName"))]
  pub workbook_algorithm_name: Option<crate::simple_type::StringValue>,
  /// Password Hash Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookHashValue
  #[sdk(attr(qname = ":workbookHashValue"))]
  pub workbook_hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Salt Value for Password Verifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookSaltValue
  #[sdk(attr(qname = ":workbookSaltValue"))]
  pub workbook_salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// Iterations to Run Hashing Algorithm
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :workbookSpinCount
  #[sdk(attr(qname = ":workbookSpinCount"))]
  pub workbook_spin_count: Option<crate::simple_type::UInt32Value>,
}
/// Defines the BookViews Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:bookViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_BookViews/x:bookViews")]
pub struct BookViews {
  /// _
  #[sdk(child(qname = "x:CT_BookView/x:workbookView"))]
  pub x_workbook_view: Vec<WorkbookView>,
}
/// Defines the Sheets Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:sheets.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Sheets/x:sheets")]
pub struct Sheets {
  /// _
  #[sdk(child(qname = "x:CT_Sheet/x:sheet"))]
  pub x_sheet: Vec<Sheet>,
}
/// Defines the FunctionGroups Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:functionGroups.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FunctionGroups/x:functionGroups")]
pub struct FunctionGroups {
  /// Built-in Function Group Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :builtInGroupCount
  #[sdk(attr(qname = ":builtInGroupCount"))]
  pub built_in_group_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_FunctionGroup/x:functionGroup"))]
  pub x_function_group: Vec<FunctionGroup>,
}
/// Defines the ExternalReferences Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:externalReferences.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExternalReferences/x:externalReferences")]
pub struct ExternalReferences {
  /// _
  #[sdk(child(qname = "x:CT_ExternalReference/x:externalReference"))]
  pub x_external_reference: Vec<ExternalReference>,
}
/// Defines the DefinedNames Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:definedNames.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DefinedNames/x:definedNames")]
pub struct DefinedNames {
  /// _
  #[sdk(child(qname = "x:CT_DefinedName/x:definedName"))]
  pub x_defined_name: Vec<DefinedName>,
}
/// Defines the CalculationProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:calcPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalcPr/x:calcPr")]
pub struct CalculationProperties {
  /// Calculation Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calcId
  #[sdk(attr(qname = ":calcId"))]
  pub calculation_id: Option<crate::simple_type::UInt32Value>,
  /// Calculation Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calcMode
  #[sdk(attr(qname = ":calcMode"))]
  pub calculation_mode: Option<CalculateModeValues>,
  /// Full Calculation On Load
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fullCalcOnLoad
  #[sdk(attr(qname = ":fullCalcOnLoad"))]
  pub full_calculation_on_load: Option<crate::simple_type::BooleanValue>,
  /// Reference Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :refMode
  #[sdk(attr(qname = ":refMode"))]
  pub reference_mode: Option<ReferenceModeValues>,
  /// Calculation Iteration
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iterate
  #[sdk(attr(qname = ":iterate"))]
  pub iterate: Option<crate::simple_type::BooleanValue>,
  /// Iteration Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iterateCount
  #[sdk(attr(qname = ":iterateCount"))]
  pub iterate_count: Option<crate::simple_type::UInt32Value>,
  /// Iterative Calculation Delta
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :iterateDelta
  #[sdk(attr(qname = ":iterateDelta"))]
  pub iterate_delta: Option<crate::simple_type::DoubleValue>,
  /// Full Precision Calculation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fullPrecision
  #[sdk(attr(qname = ":fullPrecision"))]
  pub full_precision: Option<crate::simple_type::BooleanValue>,
  /// Calc Completed
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calcCompleted
  #[sdk(attr(qname = ":calcCompleted"))]
  pub calculation_completed: Option<crate::simple_type::BooleanValue>,
  /// Calculate On Save
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :calcOnSave
  #[sdk(attr(qname = ":calcOnSave"))]
  pub calculation_on_save: Option<crate::simple_type::BooleanValue>,
  /// Concurrent Calculations
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :concurrentCalc
  #[sdk(attr(qname = ":concurrentCalc"))]
  pub concurrent_calculation: Option<crate::simple_type::BooleanValue>,
  /// Concurrent Thread Manual Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :concurrentManualCount
  #[sdk(attr(qname = ":concurrentManualCount"))]
  pub concurrent_manual_count: Option<crate::simple_type::UInt32Value>,
  /// Force Full Calculation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :forceFullCalc
  #[sdk(attr(qname = ":forceFullCalc"))]
  pub force_full_calculation: Option<crate::simple_type::BooleanValue>,
}
/// Defines the OleSize Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:oleSize.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_OleSize/x:oleSize")]
pub struct OleSize {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
}
/// Defines the CustomWorkbookViews Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:customWorkbookViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CustomWorkbookViews/x:customWorkbookViews")]
pub struct CustomWorkbookViews {
  /// _
  #[sdk(child(qname = "x:CT_CustomWorkbookView/x:customWorkbookView"))]
  pub x_custom_workbook_view: Vec<CustomWorkbookView>,
}
/// Defines the PivotCaches Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:pivotCaches.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/x:pivotCaches")]
pub struct PivotCaches {
  /// _
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache: Vec<PivotCache>,
}
/// Defines the WebPublishing Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPublishing.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishing/x:webPublishing")]
pub struct WebPublishing {
  /// css
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :css
  #[sdk(attr(qname = ":css"))]
  pub use_css: Option<crate::simple_type::BooleanValue>,
  /// thicket
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thicket
  #[sdk(attr(qname = ":thicket"))]
  pub thicket: Option<crate::simple_type::BooleanValue>,
  /// longFileNames
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :longFileNames
  #[sdk(attr(qname = ":longFileNames"))]
  pub long_file_names: Option<crate::simple_type::BooleanValue>,
  /// vml
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vml
  #[sdk(attr(qname = ":vml"))]
  pub use_vml: Option<crate::simple_type::BooleanValue>,
  /// allowPng
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :allowPng
  #[sdk(attr(qname = ":allowPng"))]
  pub allow_png: Option<crate::simple_type::BooleanValue>,
  /// targetScreenSize
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :targetScreenSize
  #[sdk(attr(qname = ":targetScreenSize"))]
  pub target_screen_size: Option<TargetScreenSizeValues>,
  /// dpi
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dpi
  #[sdk(attr(qname = ":dpi"))]
  pub dpi: Option<crate::simple_type::UInt32Value>,
  /// codePage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub character_set: Option<crate::simple_type::StringValue>,
}
/// Defines the FileRecoveryProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:fileRecoveryPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FileRecoveryPr/x:fileRecoveryPr")]
pub struct FileRecoveryProperties {
  /// Auto Recover
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :autoRecover
  #[sdk(attr(qname = ":autoRecover"))]
  pub auto_recover: Option<crate::simple_type::BooleanValue>,
  /// Crash Save
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :crashSave
  #[sdk(attr(qname = ":crashSave"))]
  pub crash_save: Option<crate::simple_type::BooleanValue>,
  /// Data Extract Load
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataExtractLoad
  #[sdk(attr(qname = ":dataExtractLoad"))]
  pub data_extract_load: Option<crate::simple_type::BooleanValue>,
  /// Repair Load
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :repairLoad
  #[sdk(attr(qname = ":repairLoad"))]
  pub repair_load: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebPublishObjects Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:webPublishObjects.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WebPublishObjects/x:webPublishObjects")]
pub struct WebPublishObjects {
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_WebPublishObject/x:webPublishObject"))]
  pub x_web_publish_object: Vec<WebPublishObject>,
}
/// Defines the WorkbookExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookExtensionList/x:extLst")]
pub struct WorkbookExtensionList {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// _
  #[sdk(child(qname = "x:CT_WorkbookExtension/x:ext"))]
  pub x_ext: Vec<WorkbookExtension>,
}
/// Defines the WorkbookExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is x:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_WorkbookExtension/x:ext")]
pub struct WorkbookExtension {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Option<WorkbookExtensionChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionsChoice {
  #[sdk(child(qname = "x:CT_RevisionRowColumn/x:rrc"))]
  XRrc(std::boxed::Box<RevisionRowColumn>),
  #[sdk(child(qname = "x:CT_RevisionMove/x:rm"))]
  XRm(std::boxed::Box<RevisionMove>),
  #[sdk(child(qname = "x:CT_RevisionCustomView/x:rcv"))]
  XRcv(std::boxed::Box<RevisionCustomView>),
  #[sdk(child(qname = "x:CT_RevisionSheetRename/x:rsnm"))]
  XRsnm(std::boxed::Box<RevisionSheetName>),
  #[sdk(child(qname = "x:CT_RevisionInsertSheet/x:ris"))]
  XRis(std::boxed::Box<RevisionInsertSheet>),
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
  #[sdk(child(qname = "x:CT_RevisionAutoFormatting/x:raf"))]
  XRaf(std::boxed::Box<RevisionAutoFormat>),
  #[sdk(child(qname = "x:CT_RevisionDefinedName/x:rdn"))]
  XRdn(std::boxed::Box<RevisionDefinedName>),
  #[sdk(child(qname = "x:CT_RevisionComment/x:rcmt"))]
  XRcmt(std::boxed::Box<RevisionComment>),
  #[sdk(child(qname = "x:CT_RevisionQueryTableField/x:rqt"))]
  XRqt(std::boxed::Box<RevisionQueryTable>),
  #[sdk(child(qname = "x:CT_RevisionConflict/x:rcft"))]
  XRcft(std::boxed::Box<RevisionConflict>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ExternalLinkChoice {
  #[sdk(child(qname = "x:CT_ExternalBook/x:externalBook"))]
  XExternalBook(std::boxed::Box<ExternalBook>),
  #[sdk(child(qname = "x:CT_DdeLink/x:ddeLink"))]
  XDdeLink(std::boxed::Box<DdeLink>),
  #[sdk(child(qname = "x:CT_OleLink/x:oleLink"))]
  XOleLink(std::boxed::Box<OleLink>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FilterColumnChoice {
  #[sdk(child(qname = "x:CT_Filters/x:filters"))]
  XFilters(std::boxed::Box<Filters>),
  #[sdk(child(qname = "x:CT_Top10/x:top10"))]
  XTop10(std::boxed::Box<Top10>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_CustomFilters/x14:customFilters"))]
  X14CustomFilters(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CustomFilters,
    >,
  ),
  #[sdk(child(qname = "x:CT_CustomFilters/x:customFilters"))]
  XCustomFilters(std::boxed::Box<CustomFilters>),
  #[sdk(child(qname = "x:CT_DynamicFilter/x:dynamicFilter"))]
  XDynamicFilter(std::boxed::Box<DynamicFilter>),
  #[sdk(child(qname = "x:CT_ColorFilter/x:colorFilter"))]
  XColorFilter(std::boxed::Box<ColorFilter>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_IconFilter/x14:iconFilter"))]
  X14IconFilter(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconFilter,
    >,
  ),
  #[sdk(child(qname = "x:CT_IconFilter/x:iconFilter"))]
  XIconFilter(std::boxed::Box<IconFilter>),
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SortStateChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_SortCondition/x14:sortCondition"))]
  X14SortCondition(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SortCondition,
    >,
  ),
  #[sdk(child(qname = "x:CT_SortCondition/x:sortCondition"))]
  XSortCondition(std::boxed::Box<SortCondition>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TablesChoice {
  #[sdk(child(qname = "x:CT_TableMissing/x:m"))]
  XM(std::boxed::Box<MissingTable>),
  #[sdk(child(qname = "x:CT_XStringElement/x:s"))]
  XS(std::boxed::Box<CharacterValue>),
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  XX(std::boxed::Box<FieldItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotCacheRecordChoice {
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
  #[sdk(child(qname = "x:CT_Index/x:x"))]
  XX(std::boxed::Box<FieldItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum EntriesChoice {
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GroupItemsChoice {
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum RstTypeChoice {
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  XT(std::boxed::Box<Text>),
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  XR(std::boxed::Box<Run>),
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  XRPh(std::boxed::Box<PhoneticRun>),
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  XPhoneticPr(std::boxed::Box<PhoneticProperties>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionRowColumnChoice {
  #[sdk(child(qname = "x:CT_UndoInfo/x:undo"))]
  XUndo(std::boxed::Box<Undo>),
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionMoveChoice {
  #[sdk(child(qname = "x:CT_UndoInfo/x:undo"))]
  XUndo(std::boxed::Box<Undo>),
  #[sdk(child(qname = "x:CT_RevisionCellChange/x:rcc"))]
  XRcc(std::boxed::Box<RevisionCellChange>),
  #[sdk(child(qname = "x:CT_RevisionFormatting/x:rfmt"))]
  XRfmt(std::boxed::Box<RevisionFormat>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CellTypeChoice {
  #[sdk(child(qname = "x:CT_CellFormula/x:f"))]
  XF(std::boxed::Box<CellFormula>),
  #[sdk(child(qname = "x:CT_Xstring/x:v"))]
  XV(std::boxed::Box<CellValue>),
  #[sdk(child(qname = "x:CT_Rst/x:is"))]
  XIs(std::boxed::Box<InlineString>),
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DifferentialFormatTypeChoice {
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  XFont(std::boxed::Box<Font>),
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  XNumFmt(std::boxed::Box<NumberingFormat>),
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  XFill(std::boxed::Box<Fill>),
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  XAlignment(std::boxed::Box<Alignment>),
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  XBorder(std::boxed::Box<Border>),
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  XProtection(std::boxed::Box<Protection>),
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MdxChoice {
  #[sdk(child(qname = "x:CT_MdxTuple/x:t"))]
  XT(std::boxed::Box<MdxTuple>),
  #[sdk(child(qname = "x:CT_MdxSet/x:ms"))]
  XMs(std::boxed::Box<MdxSet>),
  #[sdk(child(qname = "x:CT_MdxMemeberProp/x:p"))]
  XP(std::boxed::Box<MdxMemberProp>),
  #[sdk(child(qname = "x:CT_MdxKPI/x:k"))]
  XK(std::boxed::Box<MdxKpi>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FillChoice {
  #[sdk(child(qname = "x:CT_PatternFill/x:patternFill"))]
  XPatternFill(std::boxed::Box<PatternFill>),
  #[sdk(child(qname = "x:CT_GradientFill/x:gradientFill"))]
  XGradientFill(std::boxed::Box<GradientFill>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OleItemsChoice {
  #[sdk(child(qname = "x:CT_OleItem/x:oleItem"))]
  XOleItem(std::boxed::Box<OleItem>),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_OleItem/x14:oleItem"))]
  X14OleItem(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::OleItem,
    >,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MarkerTypeChoice {
  #[sdk(text_child(qname = "xdr:ST_ColID/xdr:col"))]
  XdrCol(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:colOff"))]
  XdrColOff(crate::simple_type::Int64Value),
  #[sdk(text_child(qname = "xdr:ST_RowID/xdr:row"))]
  XdrRow(crate::simple_type::Int32Value),
  #[sdk(text_child(qname = "a:ST_Coordinate/xdr:rowOff"))]
  XdrRowOff(crate::simple_type::Int64Value),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ConditionalFormattingRuleExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(text_child(qname = "x:ST_Guid/x14:id"))]
  X14Id(crate::simple_type::StringValue),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotHierarchyExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_PivotHierarchy/x14:pivotHierarchy"))]
  X14PivotHierarchy(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotHierarchy,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotFieldExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_PivotField/x14:pivotField"))]
  X14PivotField(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotField,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CacheSourceExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_SourceConnection/x14:sourceConnection"))]
  X14SourceConnection(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SourceConnection,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FiltersChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_Filter/x14:filter"))]
  X14Filter(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Filter>,
  ),
  #[sdk(child(qname = "x:CT_Filter/x:filter"))]
  XFilter(std::boxed::Box<Filter>),
  #[sdk(child(qname = "x:CT_DateGroupItem/x:dateGroupItem"))]
  XDateGroupItem(std::boxed::Box<DateGroupItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SlicerCacheDefinitionExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_SlicerCachePivotTables/x15:slicerCachePivotTables"))]
    X15SlicerCachePivotTables(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCachePivotTables,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_TableSlicerCache/x15:tableSlicerCache"))]
    X15TableSlicerCache(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TableSlicerCache,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(
        child(qname = "x15:CT_SlicerCacheHideNoData/x15:slicerCacheHideItemsWithNoData")
    )]
    X15SlicerCacheHideItemsWithNoData(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCacheHideItemsWithNoData,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotFilterExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_PivotFilter/x15:pivotFilter"))]
  X15PivotFilter(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotFilter,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_MovingPeriodState/x15:movingPeriodState"))]
  X15MovingPeriodState(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::MovingPeriodState,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum QueryTableExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_QueryTable/x15:queryTable"))]
  X15QueryTable(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::QueryTable,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ConnectionExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_Connection/x14:connection"))]
  X14Connection(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Connection,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_Connection/x15:connection"))]
  X15Connection(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Connection,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SharedItemsChoice {
  #[sdk(child(qname = "x:CT_Missing/x:m"))]
  XM(std::boxed::Box<MissingItem>),
  #[sdk(child(qname = "x:CT_Number/x:n"))]
  XN(std::boxed::Box<NumberItem>),
  #[sdk(child(qname = "x:CT_Boolean/x:b"))]
  XB(std::boxed::Box<BooleanItem>),
  #[sdk(child(qname = "x:CT_Error/x:e"))]
  XE(std::boxed::Box<ErrorItem>),
  #[sdk(child(qname = "x:CT_String/x:s"))]
  XS(std::boxed::Box<StringItem>),
  #[sdk(child(qname = "x:CT_DateTime/x:d"))]
  XD(std::boxed::Box<DateTimeItem>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FieldGroupChoice {
  #[sdk(child(qname = "x:CT_RangePr/x:rangePr"))]
  XRangePr(std::boxed::Box<RangeProperties>),
  #[sdk(child(qname = "x:CT_DiscretePr/x:discretePr"))]
  XDiscretePr(std::boxed::Box<DiscreteProperties>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CacheFieldExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_CacheField/x14:cacheField"))]
  X14CacheField(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CacheField,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_CachedUniqueNames/x15:cachedUniqueNames"))]
  X15CachedUniqueNames(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CachedUniqueNames,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CacheHierarchyExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_CacheHierarchy/x14:cacheHierarchy"))]
  X14CacheHierarchy(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CacheHierarchy,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_CacheHierarchy/x15:cacheHierarchy"))]
  X15CacheHierarchy(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CacheHierarchy,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CalculatedMemberExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_CalculatedMember/x14:calculatedMember"))]
  X14CalculatedMember(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CalculatedMember,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_CalculatedMember/x15:calculatedMember"))]
  X15CalculatedMember(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::CalculatedMember,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DataFieldExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_DataField/x14:dataField"))]
  X14DataField(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DataField,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_DataField/x15:dataField"))]
  X15DataField(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DataField,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum WorksheetExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_ConditionalFormattings/x14:conditionalFormattings"))]
    X14ConditionalFormattings(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::ConditionalFormattings,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_DataValidations/x14:dataValidations"))]
    X14DataValidations(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DataValidations,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_SparklineGroups/x14:sparklineGroups"))]
    X14SparklineGroups(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineGroups,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_SlicerRefs/x14:slicerList"))]
    X14SlicerList(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerList,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_ProtectedRanges/x14:protectedRanges"))]
    X14ProtectedRanges(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::ProtectedRanges,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_IgnoredErrors/x14:ignoredErrors"))]
    X14IgnoredErrors(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IgnoredErrors,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_WebExtensions/x15:webExtensions"))]
    X15WebExtensions(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::WebExtensions,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_TimelineRefs/x15:timelineRefs"))]
    X15TimelineRefs(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineReferences,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StylesheetExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x:CT_Dxfs/x14:dxfs"))]
  X14Dxfs(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DifferentialFormats,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_SlicerStyles/x14:slicerStyles"))]
  X14SlicerStyles(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerStyles,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x:CT_Dxfs/x15:dxfs"))]
  X15Dxfs(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DifferentialFormats,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x15:CT_TimelineStyles/x15:timelineStyles"))]
  X15TimelineStyles(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineStyles,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotTableDefinitionExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_PivotTableDefinition/x14:pivotTableDefinition"))]
    X14PivotTableDefinition(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotTableDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_PivotTableData/x15:pivotTableData"))]
    X15PivotTableData(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableData,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_PivotTableUISettings/x15:pivotTableUISettings"))]
    X15PivotTableUiSettings(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableUiSettings,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xxpvi:CT_PivotVersionInfo/xxpvi:pivotVersionInfo"))]
    XxpviPivotVersionInfo(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_pivot_version_info::PivotVersionInfo,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CacheSourceChoice {
  #[sdk(child(qname = "x:CT_WorksheetSource/x:worksheetSource"))]
  XWorksheetSource(std::boxed::Box<WorksheetSource>),
  #[sdk(child(qname = "x:CT_Consolidation/x:consolidation"))]
  XConsolidation(std::boxed::Box<Consolidation>),
  #[sdk(child(qname = "x:CT_CacheSourceExtensionList/x:extLst"))]
  XExtLst(std::boxed::Box<CacheSourceExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum PivotCacheDefinitionExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_PivotCacheDefinition/x14:pivotCacheDefinition"))]
    X14PivotCacheDefinition(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotCacheDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_PivotCacheDecoupled/x15:pivotCacheDecoupled"))]
    X15PivotCacheDecoupled(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCacheDecoupled,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(
        child(
            qname = "x15:CT_TimelinePivotCacheDefinition/x15:timelinePivotCacheDefinition"
        )
    )]
    X15TimelinePivotCacheDefinition(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelinePivotCacheDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_PivotCacheIdVersion/x15:pivotCacheIdVersion"))]
    X15PivotCacheIdVersion(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCacheIdVersion,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(text_child(qname = "xsd:boolean/xxpim:implicitMeasureSupport"))]
    XxpimImplicitMeasureSupport(crate::simple_type::BooleanValue),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xprd:CT_PivotCacheRichInfo/xprd:richInfo"))]
    XprdRichInfo(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_pivot_rich_data::PivotCacheRichInfo,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xxpvi:CT_CacheVersionInfo/xxpvi:cacheVersionInfo"))]
    XxpviCacheVersionInfo(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_pivot_version_info::CacheVersionInfo,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(text_child(qname = "xsd:boolean/xlpar:autoRefresh"))]
    XlparAutoRefresh(crate::simple_type::BooleanValue),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xlpda:CT_PivotCacheDynamicArray/xlpda:pivotCacheDynamicArray"))]
    XlpdaPivotCacheDynamicArray(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2024_pivot_dynamic_arrays::PivotCacheDynamicArray,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TableExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "x14:CT_Table/x14:table"))]
  X14Table(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Table>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "xlmsforms:CT_MsForm/xlmsforms:msForm"))]
  XlmsformsMsForm(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2023_ms_forms::MsForm,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum WorkbookExtensionChoice {
  #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_DefinedNames/x14:definedNames"))]
    X14DefinedNames(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DefinedNames,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x:CT_PivotCaches/x14:pivotCaches"))]
    X14PivotCaches(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::PivotCaches,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_SlicerCaches/x14:slicerCaches"))]
    X14SlicerCaches(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCaches,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_SlicerCaches/x15:slicerCaches"))]
    X15SlicerCaches(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::SlicerCaches,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x14:CT_WorkbookPr/x14:workbookPr"))]
    X14WorkbookPr(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::WorkbookProperties,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x:CT_PivotCaches/x15:pivotCaches"))]
    X15PivotCaches(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotCaches,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_PivotTableReferences/x15:pivotTableReferences"))]
    X15PivotTableReferences(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::PivotTableReferences,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x:CT_PivotCaches/x15:timelineCachePivotCaches"))]
    X15TimelineCachePivotCaches(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCachePivotCaches,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_TimelineCacheRefs/x15:timelineCacheRefs"))]
    X15TimelineCacheRefs(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheReferences,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_WorkbookPr/x15:workbookPr"))]
    X15WorkbookPr(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::WorkbookProperties,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "x15:CT_DataModel/x15:dataModel"))]
    X15DataModel(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::DataModel,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xlecs:CT_ExternalCodeService/xlecs:externalCodeService"))]
    XlecsExternalCodeService(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2023_external_code_service::ExternalCodeService,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(child(qname = "xlwcv:CT_Version/xlwcv:version"))]
    XlwcvVersion(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2024_workbook_compatibility_version::Version,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    #[sdk(
        child(
            qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput"
        )
    )]
    Xlecs2ExternalCodeServiceImageAsInput(
        std::boxed::Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2025_external_code_service2::ExternalCodeServiceImageAsInput,
        >,
    ),
    #[sdk(any)]
    UnknownXml(String),
}
