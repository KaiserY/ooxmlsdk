pub type EmuValue = i64;
pub type TwipsValue = i64;
pub type UnsignedDecimalValue = u64;
pub type DecimalValue = i64;
pub type DrawingmlPercentValue = i32;
pub type WordPercentValue = i64;
pub type DrawingmlAngleValue = i32;
pub type VmlFixedValue = i32;
pub type TextFontSizeValue = i32;
pub type TextSpacingPointValue = i32;

pub const EMUS_PER_INCH: EmuValue = 914_400;
pub const EMUS_PER_CENTIMETER: EmuValue = 360_000;
pub const EMUS_PER_MILLIMETER: EmuValue = 36_000;
pub const EMUS_PER_POINT: EmuValue = 12_700;
pub const EMUS_PER_PICA: EmuValue = 152_400;
pub const EMUS_PER_TWIP: EmuValue = 635;
pub const EMUS_PER_CSS_PIXEL: EmuValue = 9_525;

pub const TWIPS_PER_INCH: TwipsValue = 1_440;
pub const TWIPS_PER_POINT: TwipsValue = 20;

pub const MM100_PER_MILLIMETER: i64 = 100;
pub const MM100_PER_INCH: i64 = 2_540;

pub const DRAWINGML_PERCENT_SCALE: DrawingmlPercentValue = 100_000;
pub const DRAWINGML_PERCENT_UNITS_PER_PERCENT: DrawingmlPercentValue = 1_000;
pub const WORD_PERCENT_SCALE: WordPercentValue = 5_000;
pub const VML_PERCENT_SCALE: i32 = 100;
pub const VML_FIXED_SCALE: VmlFixedValue = 65_536;
pub const HALF_POINTS_PER_POINT: i64 = 2;
pub const DRAWINGML_TEXT_SIZE_UNITS_PER_POINT: i64 = 100;
pub const POINTS100_PER_POINT: i64 = DRAWINGML_TEXT_SIZE_UNITS_PER_POINT;

pub const DRAWINGML_ANGLE_UNITS_PER_DEGREE: DrawingmlAngleValue = 60_000;
pub const DEGREE100_UNITS_PER_DEGREE: i32 = 100;
pub const DEGREE100_FULL_CIRCLE: i32 = 36_000;

pub const DRAWINGML_COORDINATE_MIN: EmuValue = -27_273_042_329_600;
pub const DRAWINGML_COORDINATE_MAX: EmuValue = 27_273_042_316_900;
pub const DRAWINGML_POSITIVE_COORDINATE_MIN: EmuValue = 0;
pub const DRAWINGML_POSITIVE_COORDINATE_MAX: EmuValue = DRAWINGML_COORDINATE_MAX;
pub const DRAWINGML_FIXED_PERCENT_MIN: DrawingmlPercentValue = -100_000;
pub const DRAWINGML_FIXED_PERCENT_MAX: DrawingmlPercentValue = 100_000;
pub const DRAWINGML_POSITIVE_FIXED_PERCENT_MIN: DrawingmlPercentValue = 0;
pub const DRAWINGML_POSITIVE_FIXED_PERCENT_MAX: DrawingmlPercentValue = 100_000;
pub const DRAWINGML_FIXED_ANGLE_MIN_EXCLUSIVE: DrawingmlAngleValue = -5_400_000;
pub const DRAWINGML_FIXED_ANGLE_MAX_EXCLUSIVE: DrawingmlAngleValue = 5_400_000;
pub const DRAWINGML_POSITIVE_FIXED_ANGLE_MIN: DrawingmlAngleValue = 0;
pub const DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE: DrawingmlAngleValue = 21_600_000;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LengthUnit {
  Millimeter,
  Centimeter,
  Inch,
  Point,
  Pica,
  PicaPi,
}

impl LengthUnit {
  #[inline(always)]
  pub const fn suffix(self) -> &'static str {
    match self {
      Self::Millimeter => "mm",
      Self::Centimeter => "cm",
      Self::Inch => "in",
      Self::Point => "pt",
      Self::Pica => "pc",
      Self::PicaPi => "pi",
    }
  }

  #[inline(always)]
  pub const fn emus_per_unit(self) -> EmuValue {
    match self {
      Self::Millimeter => EMUS_PER_MILLIMETER,
      Self::Centimeter => EMUS_PER_CENTIMETER,
      Self::Inch => EMUS_PER_INCH,
      Self::Point => EMUS_PER_POINT,
      Self::Pica | Self::PicaPi => EMUS_PER_PICA,
    }
  }

  #[inline(always)]
  pub const fn from_suffix_bytes(value: &[u8]) -> Option<Self> {
    match value {
      b"mm" => Some(Self::Millimeter),
      b"cm" => Some(Self::Centimeter),
      b"in" => Some(Self::Inch),
      b"pt" => Some(Self::Point),
      b"pc" => Some(Self::Pica),
      b"pi" => Some(Self::PicaPi),
      _ => None,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnitParseError {
  Empty,
  InvalidNumber,
  InvalidUnit,
  Overflow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UniversalMeasureValue {
  Millimeters(EmuValue),
  Centimeters(EmuValue),
  Inches(EmuValue),
  Points(EmuValue),
  Picas(EmuValue),
  PicasPi(EmuValue),
}

pub type PositiveUniversalMeasureValue = UniversalMeasureValue;

impl Default for UniversalMeasureValue {
  fn default() -> Self {
    Self::Points(0)
  }
}

impl UniversalMeasureValue {
  #[inline]
  pub const fn from_emu(unit: LengthUnit, emu: EmuValue) -> Self {
    match unit {
      LengthUnit::Millimeter => Self::Millimeters(emu),
      LengthUnit::Centimeter => Self::Centimeters(emu),
      LengthUnit::Inch => Self::Inches(emu),
      LengthUnit::Point => Self::Points(emu),
      LengthUnit::Pica => Self::Picas(emu),
      LengthUnit::PicaPi => Self::PicasPi(emu),
    }
  }

  #[inline]
  pub const fn unit(self) -> LengthUnit {
    match self {
      Self::Millimeters(_) => LengthUnit::Millimeter,
      Self::Centimeters(_) => LengthUnit::Centimeter,
      Self::Inches(_) => LengthUnit::Inch,
      Self::Points(_) => LengthUnit::Point,
      Self::Picas(_) => LengthUnit::Pica,
      Self::PicasPi(_) => LengthUnit::PicaPi,
    }
  }

  #[inline]
  pub const fn emu(self) -> EmuValue {
    match self {
      Self::Millimeters(value)
      | Self::Centimeters(value)
      | Self::Inches(value)
      | Self::Points(value)
      | Self::Picas(value)
      | Self::PicasPi(value) => value,
    }
  }

  #[inline]
  pub fn to_twips(self) -> TwipsValue {
    emu_to_twips(self.emu())
  }

  #[inline]
  pub fn to_points(self) -> f64 {
    emu_to_points(self.emu())
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    let unit = self.unit();
    format_decimal_ratio(self.emu(), unit.emus_per_unit(), unit.suffix())
  }

  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    parse_universal_measure_bytes(value)
  }
}

impl std::fmt::Display for UniversalMeasureValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for UniversalMeasureValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    parse_universal_measure(value)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TwipsMeasureValue {
  Twips(UnsignedDecimalValue),
  UniversalMeasure(PositiveUniversalMeasureValue),
}

impl Default for TwipsMeasureValue {
  fn default() -> Self {
    Self::Twips(0)
  }
}

impl TwipsMeasureValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_u64_bytes(value) {
      return Ok(Self::Twips(value));
    }
    parse_positive_universal_measure_bytes(value)
      .map(Self::UniversalMeasure)
      .or_else(|_| {
        parse_bare_twips_decimal_bytes(value, false).map(|value| Self::Twips(value as u64))
      })
  }

  #[inline]
  pub fn to_twips(self) -> TwipsValue {
    match self {
      Self::Twips(value) => u64_to_i64_saturating(value),
      Self::UniversalMeasure(value) => value.to_twips(),
    }
  }

  #[inline]
  pub fn to_emu(self) -> EmuValue {
    twips_to_emu(self.to_twips())
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Twips(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for TwipsMeasureValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for TwipsMeasureValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SignedTwipsMeasureValue {
  Twips(DecimalValue),
  UniversalMeasure(UniversalMeasureValue),
}

impl Default for SignedTwipsMeasureValue {
  fn default() -> Self {
    Self::Twips(0)
  }
}

impl SignedTwipsMeasureValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i64_bytes(value) {
      return Ok(Self::Twips(value));
    }
    parse_universal_measure_bytes(value)
      .map(Self::UniversalMeasure)
      .or_else(|_| parse_bare_twips_decimal_bytes(value, true).map(Self::Twips))
  }

  #[inline]
  pub fn to_twips(self) -> TwipsValue {
    match self {
      Self::Twips(value) => value,
      Self::UniversalMeasure(value) => value.to_twips(),
    }
  }

  #[inline]
  pub fn to_emu(self) -> EmuValue {
    twips_to_emu(self.to_twips())
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Twips(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for SignedTwipsMeasureValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for SignedTwipsMeasureValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum HpsMeasureValue {
  HalfPoints(UnsignedDecimalValue),
  UniversalMeasure(PositiveUniversalMeasureValue),
}

impl Default for HpsMeasureValue {
  fn default() -> Self {
    Self::HalfPoints(0)
  }
}

impl HpsMeasureValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_u64_bytes(value) {
      return Ok(Self::HalfPoints(value));
    }
    parse_positive_universal_measure_bytes(value).map(Self::UniversalMeasure)
  }

  #[inline]
  pub fn to_half_points(self) -> DecimalValue {
    match self {
      Self::HalfPoints(value) => u64_to_i64_saturating(value),
      Self::UniversalMeasure(value) => emu_to_half_points(value.emu()),
    }
  }

  #[inline]
  pub fn to_points(self) -> f64 {
    self.to_half_points() as f64 / 2.0
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::HalfPoints(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for HpsMeasureValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for HpsMeasureValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SignedHpsMeasureValue {
  HalfPoints(DecimalValue),
  UniversalMeasure(UniversalMeasureValue),
}

impl Default for SignedHpsMeasureValue {
  fn default() -> Self {
    Self::HalfPoints(0)
  }
}

impl SignedHpsMeasureValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i64_bytes(value) {
      return Ok(Self::HalfPoints(value));
    }
    parse_universal_measure_bytes(value).map(Self::UniversalMeasure)
  }

  #[inline]
  pub fn to_half_points(self) -> DecimalValue {
    match self {
      Self::HalfPoints(value) => value,
      Self::UniversalMeasure(value) => emu_to_half_points(value.emu()),
    }
  }

  #[inline]
  pub fn to_points(self) -> f64 {
    self.to_half_points() as f64 / 2.0
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::HalfPoints(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for SignedHpsMeasureValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for SignedHpsMeasureValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoordinateValue {
  Emu(EmuValue),
  UniversalMeasure(UniversalMeasureValue),
}

impl Default for CoordinateValue {
  fn default() -> Self {
    Self::Emu(0)
  }
}

impl CoordinateValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i64_bytes(value) {
      return Ok(Self::Emu(value));
    }
    parse_universal_measure_bytes(value).map(Self::UniversalMeasure)
  }

  #[inline]
  pub const fn to_emu(self) -> EmuValue {
    match self {
      Self::Emu(value) => value,
      Self::UniversalMeasure(value) => value.emu(),
    }
  }

  #[inline]
  pub fn to_twips(self) -> TwipsValue {
    emu_to_twips(self.to_emu())
  }

  #[inline]
  pub const fn is_valid_coordinate(self) -> bool {
    let value = self.to_emu();
    value >= DRAWINGML_COORDINATE_MIN && value <= DRAWINGML_COORDINATE_MAX
  }

  #[inline]
  pub const fn is_valid_positive_coordinate(self) -> bool {
    let value = self.to_emu();
    value >= DRAWINGML_POSITIVE_COORDINATE_MIN && value <= DRAWINGML_POSITIVE_COORDINATE_MAX
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Emu(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for CoordinateValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for CoordinateValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Coordinate32Value {
  Emu(i32),
  UniversalMeasure(UniversalMeasureValue),
}

pub type PositiveCoordinateValue = CoordinateValue;
pub type PositiveCoordinate32Value = Coordinate32Value;

impl Default for Coordinate32Value {
  fn default() -> Self {
    Self::Emu(0)
  }
}

impl Coordinate32Value {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i32_bytes(value) {
      return Ok(Self::Emu(value));
    }
    parse_universal_measure_bytes(value).map(Self::UniversalMeasure)
  }

  #[inline]
  pub const fn to_emu(self) -> EmuValue {
    match self {
      Self::Emu(value) => value as EmuValue,
      Self::UniversalMeasure(value) => value.emu(),
    }
  }

  #[inline]
  pub const fn is_valid_positive_coordinate(self) -> bool {
    self.to_emu() >= 0
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Emu(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for Coordinate32Value {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for Coordinate32Value {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DecimalNumberOrPercentValue {
  DecimalNumber(DecimalValue),
  Percent(DrawingmlPercentValue),
}

impl Default for DecimalNumberOrPercentValue {
  fn default() -> Self {
    Self::DecimalNumber(0)
  }
}

impl DecimalNumberOrPercentValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i64_bytes(value) {
      Ok(Self::DecimalNumber(value))
    } else if value.last() == Some(&b'%') {
      parse_percent_string_bytes(value).map(Self::Percent)
    } else {
      Err(UnitParseError::InvalidUnit)
    }
  }

  #[inline]
  pub const fn as_drawingml_percent_saturating(self) -> DrawingmlPercentValue {
    match self {
      Self::DecimalNumber(value) => i64_to_i32_saturating(value),
      Self::Percent(value) => value,
    }
  }

  #[inline]
  pub fn as_drawingml_ratio(self) -> f64 {
    drawingml_percent_to_ratio(self.as_drawingml_percent_saturating())
  }

  #[inline]
  pub fn as_word_ratio(self) -> f64 {
    match self {
      Self::DecimalNumber(value) => word_percent_to_ratio(value),
      Self::Percent(value) => drawingml_percent_to_ratio(value),
    }
  }

  #[inline]
  pub fn percent_to_lexical(self) -> Option<String> {
    match self {
      Self::DecimalNumber(_) => None,
      Self::Percent(value) => Some(format_percent_lexical(value)),
    }
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::DecimalNumber(value) => value.to_string(),
      Self::Percent(value) => format_percent_lexical(value),
    }
  }
}

impl std::fmt::Display for DecimalNumberOrPercentValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for DecimalNumberOrPercentValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DrawingmlPercentageValue {
  Decimal(DrawingmlPercentValue),
  PercentString(DrawingmlPercentValue),
}

pub type PositiveDrawingmlPercentageValue = DrawingmlPercentageValue;
pub type FixedPercentageValue = DrawingmlPercentageValue;
pub type PositiveFixedPercentageValue = DrawingmlPercentageValue;
pub type TextFontScalePercentOrPercentStringValue = DrawingmlPercentageValue;
pub type TextSpacingPercentOrPercentStringValue = DrawingmlPercentageValue;

impl Default for DrawingmlPercentageValue {
  fn default() -> Self {
    Self::Decimal(0)
  }
}

impl DrawingmlPercentageValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i32_bytes(value) {
      Ok(Self::Decimal(value))
    } else if value.last() == Some(&b'%') {
      parse_percent_string_bytes(value).map(Self::PercentString)
    } else {
      Err(UnitParseError::InvalidUnit)
    }
  }

  #[inline]
  pub const fn as_drawingml_percent(self) -> DrawingmlPercentValue {
    match self {
      Self::Decimal(value) | Self::PercentString(value) => value,
    }
  }

  #[inline]
  pub fn as_ratio(self) -> f64 {
    drawingml_percent_to_ratio(self.as_drawingml_percent())
  }

  #[inline]
  pub fn percent_to_lexical(self) -> Option<String> {
    match self {
      Self::Decimal(_) => None,
      Self::PercentString(value) => Some(format_percent_lexical(value)),
    }
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Decimal(value) => value.to_string(),
      Self::PercentString(value) => format_percent_lexical(value),
    }
  }
}

impl std::fmt::Display for DrawingmlPercentageValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for DrawingmlPercentageValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextPointValue {
  Points100(i32),
  UniversalMeasure(UniversalMeasureValue),
}

impl Default for TextPointValue {
  fn default() -> Self {
    Self::Points100(0)
  }
}

impl TextPointValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i32_bytes(value) {
      Ok(Self::Points100(value))
    } else {
      parse_universal_measure_bytes(value).map(Self::UniversalMeasure)
    }
  }

  #[inline]
  pub fn to_points(self) -> f64 {
    match self {
      Self::Points100(value) => drawingml_text_size_to_points(value),
      Self::UniversalMeasure(value) => value.to_points(),
    }
  }

  #[inline]
  pub fn to_emu(self) -> EmuValue {
    match self {
      Self::Points100(value) => drawingml_text_size_to_emu(value),
      Self::UniversalMeasure(value) => value.emu(),
    }
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Points100(value) => value.to_string(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for TextPointValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for TextPointValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextBulletSizeValue {
  Decimal(DrawingmlPercentValue),
  PercentString(DrawingmlPercentValue),
}

pub type TextBulletSizePercentValue = DrawingmlPercentValue;
pub type TextBulletSizeDecimalValue = DrawingmlPercentValue;

impl Default for TextBulletSizeValue {
  fn default() -> Self {
    Self::Decimal(0)
  }
}

impl TextBulletSizeValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i32_bytes(value) {
      Ok(Self::Decimal(value))
    } else if value.last() == Some(&b'%') {
      parse_percent_string_bytes(value).map(Self::PercentString)
    } else {
      Err(UnitParseError::InvalidUnit)
    }
  }

  #[inline]
  pub const fn as_drawingml_percent(self) -> DrawingmlPercentValue {
    match self {
      Self::Decimal(value) | Self::PercentString(value) => value,
    }
  }

  #[inline]
  pub fn as_ratio(self) -> f64 {
    drawingml_percent_to_ratio(self.as_drawingml_percent())
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::Decimal(value) => value.to_string(),
      Self::PercentString(value) => format_percent_lexical(value),
    }
  }
}

impl std::fmt::Display for TextBulletSizeValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for TextBulletSizeValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MeasurementOrPercentValue {
  DecimalNumberOrPercent(DecimalNumberOrPercentValue),
  UniversalMeasure(UniversalMeasureValue),
}

impl Default for MeasurementOrPercentValue {
  fn default() -> Self {
    Self::DecimalNumberOrPercent(DecimalNumberOrPercentValue::default())
  }
}

impl MeasurementOrPercentValue {
  #[inline]
  pub fn from_bytes(value: &[u8]) -> Result<Self, UnitParseError> {
    if let Some(value) = try_parse_i64_bytes(value) {
      Ok(Self::DecimalNumberOrPercent(
        DecimalNumberOrPercentValue::DecimalNumber(value),
      ))
    } else if value.last() == Some(&b'%') {
      parse_percent_string_bytes(value)
        .map(|value| Self::DecimalNumberOrPercent(DecimalNumberOrPercentValue::Percent(value)))
    } else {
      parse_universal_measure_bytes(value)
        .map(Self::UniversalMeasure)
        .or_else(|_| {
          parse_bare_twips_decimal_bytes(value, true).map(|value| {
            Self::DecimalNumberOrPercent(DecimalNumberOrPercentValue::DecimalNumber(value))
          })
        })
    }
  }

  #[inline]
  pub fn to_twips(self) -> Option<TwipsValue> {
    match self {
      Self::DecimalNumberOrPercent(DecimalNumberOrPercentValue::DecimalNumber(value)) => {
        Some(value)
      }
      Self::DecimalNumberOrPercent(DecimalNumberOrPercentValue::Percent(_)) => None,
      Self::UniversalMeasure(value) => Some(value.to_twips()),
    }
  }

  #[inline]
  pub fn as_word_ratio(self) -> Option<f64> {
    match self {
      Self::DecimalNumberOrPercent(value) => Some(value.as_word_ratio()),
      Self::UniversalMeasure(_) => None,
    }
  }

  #[inline]
  pub fn to_lexical(self) -> String {
    match self {
      Self::DecimalNumberOrPercent(value) => value.to_lexical(),
      Self::UniversalMeasure(value) => value.to_lexical(),
    }
  }
}

impl std::fmt::Display for MeasurementOrPercentValue {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.to_lexical().as_str())
  }
}

impl std::str::FromStr for MeasurementOrPercentValue {
  type Err = UnitParseError;

  #[inline]
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Self::from_bytes(value.as_bytes())
  }
}

#[inline]
pub const fn twips_to_emu(value: TwipsValue) -> EmuValue {
  value.saturating_mul(EMUS_PER_TWIP)
}

#[inline]
pub fn emu_to_twips(value: EmuValue) -> TwipsValue {
  mul_div_round(value, 1, EMUS_PER_TWIP)
}

#[inline]
pub fn emu_to_half_points(value: EmuValue) -> DecimalValue {
  mul_div_round(value, HALF_POINTS_PER_POINT, EMUS_PER_POINT)
}

#[inline]
pub fn half_points_to_emu(value: DecimalValue) -> EmuValue {
  mul_div_round(value, EMUS_PER_POINT, HALF_POINTS_PER_POINT)
}

#[inline]
pub fn drawingml_text_size_to_points(value: i32) -> f64 {
  points100_to_points(value)
}

#[inline]
pub fn drawingml_text_size_to_half_points(value: i32) -> DecimalValue {
  points100_to_half_points(value)
}

#[inline]
pub fn half_points_to_drawingml_text_size(value: DecimalValue) -> i32 {
  half_points_to_points100(value)
}

#[inline]
pub fn drawingml_text_size_to_emu(value: i32) -> EmuValue {
  points100_to_emu(value)
}

#[inline]
pub fn emu_to_drawingml_text_size(value: EmuValue) -> i32 {
  emu_to_points100(value)
}

#[inline]
pub fn text_spacing_point_to_mm100(value: TextSpacingPointValue) -> i64 {
  points100_to_mm100(value)
}

#[inline]
pub fn mm100_to_text_spacing_point(value: i64) -> TextSpacingPointValue {
  mm100_to_points100(value)
}

#[inline]
pub fn points100_to_points(value: i32) -> f64 {
  value as f64 / POINTS100_PER_POINT as f64
}

#[inline]
pub fn points100_to_half_points(value: i32) -> DecimalValue {
  mul_div_round(i64::from(value), HALF_POINTS_PER_POINT, POINTS100_PER_POINT)
}

#[inline]
pub fn half_points_to_points100(value: DecimalValue) -> i32 {
  i64_to_i32_saturating(mul_div_round(
    value,
    POINTS100_PER_POINT,
    HALF_POINTS_PER_POINT,
  ))
}

#[inline]
pub fn points100_to_emu(value: i32) -> EmuValue {
  mul_div_round(i64::from(value), EMUS_PER_POINT, POINTS100_PER_POINT)
}

#[inline]
pub fn emu_to_points100(value: EmuValue) -> i32 {
  i64_to_i32_saturating(mul_div_round(value, POINTS100_PER_POINT, EMUS_PER_POINT))
}

#[inline]
pub fn points100_to_mm100(value: i32) -> i64 {
  mul_div_round(i64::from(value), MM100_PER_INCH, POINTS100_PER_POINT * 72)
}

#[inline]
pub fn mm100_to_points100(value: i64) -> i32 {
  i64_to_i32_saturating(mul_div_round(
    value,
    POINTS100_PER_POINT * 72,
    MM100_PER_INCH,
  ))
}

#[inline]
pub fn emu_to_mm100(value: EmuValue) -> i64 {
  mul_div_round(value, MM100_PER_MILLIMETER, EMUS_PER_MILLIMETER)
}

#[inline]
pub fn mm100_to_emu(value: i64) -> EmuValue {
  mul_div_round(value, EMUS_PER_MILLIMETER, MM100_PER_MILLIMETER)
}

#[inline]
pub const fn points_to_emu(value: i64) -> EmuValue {
  value.saturating_mul(EMUS_PER_POINT)
}

#[inline]
pub fn emu_to_points(value: EmuValue) -> f64 {
  value as f64 / EMUS_PER_POINT as f64
}

#[inline]
pub fn drawingml_percent_to_ratio(value: DrawingmlPercentValue) -> f64 {
  value as f64 / DRAWINGML_PERCENT_SCALE as f64
}

#[inline]
pub fn ratio_to_drawingml_percent(value: f64) -> DrawingmlPercentValue {
  round_f64_to_i32(value * DRAWINGML_PERCENT_SCALE as f64)
}

#[inline]
pub fn word_percent_to_ratio(value: WordPercentValue) -> f64 {
  value as f64 / WORD_PERCENT_SCALE as f64
}

#[inline]
pub fn vml_percent_to_ratio(value: i32) -> f64 {
  value as f64 / VML_PERCENT_SCALE as f64
}

#[inline]
pub fn vml_fixed_to_ratio(value: VmlFixedValue) -> f64 {
  value as f64 / VML_FIXED_SCALE as f64
}

#[inline]
pub fn ratio_to_vml_fixed(value: f64) -> VmlFixedValue {
  round_f64_to_i32(value * VML_FIXED_SCALE as f64)
}

#[inline]
pub fn drawingml_angle_to_degrees(value: DrawingmlAngleValue) -> f64 {
  value as f64 / DRAWINGML_ANGLE_UNITS_PER_DEGREE as f64
}

#[inline]
pub fn degrees_to_drawingml_angle(value: f64) -> DrawingmlAngleValue {
  round_f64_to_i32(value * DRAWINGML_ANGLE_UNITS_PER_DEGREE as f64)
}

#[inline]
pub fn drawingml_angle_to_degree100(value: DrawingmlAngleValue) -> i32 {
  mul_div_round_i32(
    value,
    DEGREE100_UNITS_PER_DEGREE,
    DRAWINGML_ANGLE_UNITS_PER_DEGREE,
  )
}

#[inline]
pub fn degree100_to_drawingml_angle(value: i32) -> DrawingmlAngleValue {
  mul_div_round_i32(
    value,
    DRAWINGML_ANGLE_UNITS_PER_DEGREE,
    DEGREE100_UNITS_PER_DEGREE,
  )
}

#[inline]
pub fn vml_rotation_degrees_to_degree100(value: f64) -> i32 {
  normalize_degree100(round_f64_to_i32(value * -DEGREE100_UNITS_PER_DEGREE as f64))
}

#[inline]
pub fn vml_fixed_angle_to_degree100(value: VmlFixedValue) -> i32 {
  vml_rotation_degrees_to_degree100(vml_fixed_to_ratio(value))
}

#[inline]
pub const fn normalize_degree100(value: i32) -> i32 {
  let value = value % DEGREE100_FULL_CIRCLE;
  if value < 0 {
    value + DEGREE100_FULL_CIRCLE
  } else {
    value
  }
}

#[inline]
pub const fn is_valid_drawingml_fixed_percent(value: DrawingmlPercentValue) -> bool {
  value >= DRAWINGML_FIXED_PERCENT_MIN && value <= DRAWINGML_FIXED_PERCENT_MAX
}

#[inline]
pub const fn is_valid_drawingml_positive_fixed_percent(value: DrawingmlPercentValue) -> bool {
  value >= DRAWINGML_POSITIVE_FIXED_PERCENT_MIN && value <= DRAWINGML_POSITIVE_FIXED_PERCENT_MAX
}

#[inline]
pub const fn is_valid_drawingml_fixed_angle(value: DrawingmlAngleValue) -> bool {
  value > DRAWINGML_FIXED_ANGLE_MIN_EXCLUSIVE && value < DRAWINGML_FIXED_ANGLE_MAX_EXCLUSIVE
}

#[inline]
pub const fn is_valid_drawingml_positive_fixed_angle(value: DrawingmlAngleValue) -> bool {
  value >= DRAWINGML_POSITIVE_FIXED_ANGLE_MIN
    && value < DRAWINGML_POSITIVE_FIXED_ANGLE_MAX_EXCLUSIVE
}

#[inline]
pub fn parse_universal_measure(value: &str) -> Result<UniversalMeasureValue, UnitParseError> {
  parse_universal_measure_bytes(value.as_bytes())
}

#[inline]
pub fn parse_universal_measure_bytes(
  value: &[u8],
) -> Result<UniversalMeasureValue, UnitParseError> {
  let (number, unit) = split_universal_measure_bytes(value)?;
  let emu = parse_decimal_scaled_to_i64_bytes(number, unit.emus_per_unit(), true, None)?;
  Ok(UniversalMeasureValue::from_emu(unit, emu))
}

#[inline]
pub fn parse_positive_universal_measure(
  value: &str,
) -> Result<PositiveUniversalMeasureValue, UnitParseError> {
  parse_positive_universal_measure_bytes(value.as_bytes())
}

#[inline]
pub fn parse_positive_universal_measure_bytes(
  value: &[u8],
) -> Result<PositiveUniversalMeasureValue, UnitParseError> {
  let (number, unit) = split_universal_measure_bytes(value)?;
  let emu = parse_decimal_scaled_to_i64_bytes(number, unit.emus_per_unit(), false, None)?;
  Ok(UniversalMeasureValue::from_emu(unit, emu))
}

#[inline]
pub fn parse_percent_string(value: &str) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_percent_string_bytes(value.as_bytes())
}

#[inline]
pub fn parse_percent_string_bytes(value: &[u8]) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_percent_string_bytes_with_options(value, true, None)
}

#[inline]
pub fn parse_positive_percent_string(value: &str) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_positive_percent_string_bytes(value.as_bytes())
}

#[inline]
pub fn parse_positive_percent_string_bytes(
  value: &[u8],
) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_percent_string_bytes_with_options(value, false, None)
}

#[inline]
pub fn parse_fixed_percent_string(value: &str) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_fixed_percent_string_bytes(value.as_bytes())
}

#[inline]
pub fn parse_fixed_percent_string_bytes(
  value: &[u8],
) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_percent_string_bytes_with_options(value, true, Some(2)).and_then(|value| {
    if is_valid_drawingml_fixed_percent(value) {
      Ok(value)
    } else {
      Err(UnitParseError::InvalidNumber)
    }
  })
}

#[inline]
pub fn parse_positive_fixed_percent_string(
  value: &str,
) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_positive_fixed_percent_string_bytes(value.as_bytes())
}

#[inline]
pub fn parse_positive_fixed_percent_string_bytes(
  value: &[u8],
) -> Result<DrawingmlPercentValue, UnitParseError> {
  parse_percent_string_bytes_with_options(value, false, Some(2)).and_then(|value| {
    if is_valid_drawingml_positive_fixed_percent(value) {
      Ok(value)
    } else {
      Err(UnitParseError::InvalidNumber)
    }
  })
}

#[inline]
pub fn format_percent_lexical(value: DrawingmlPercentValue) -> String {
  format_decimal_ratio(
    i64::from(value),
    i64::from(DRAWINGML_PERCENT_UNITS_PER_PERCENT),
    "%",
  )
}

#[inline]
fn parse_percent_string_bytes_with_options(
  value: &[u8],
  allow_negative: bool,
  max_fraction_digits: Option<usize>,
) -> Result<DrawingmlPercentValue, UnitParseError> {
  let number = value
    .strip_suffix(b"%")
    .ok_or(UnitParseError::InvalidUnit)?;
  let scaled = parse_decimal_scaled_to_i64_bytes(
    number,
    i64::from(DRAWINGML_PERCENT_UNITS_PER_PERCENT),
    allow_negative,
    max_fraction_digits,
  )?;
  DrawingmlPercentValue::try_from(scaled).map_err(|_| UnitParseError::Overflow)
}

#[inline]
fn split_universal_measure_bytes(value: &[u8]) -> Result<(&[u8], LengthUnit), UnitParseError> {
  if value.is_empty() {
    return Err(UnitParseError::Empty);
  }

  if value.len() < 3 {
    return Err(UnitParseError::InvalidUnit);
  }

  let (number, unit) = value.split_at(value.len() - 2);
  let unit = LengthUnit::from_suffix_bytes(unit).ok_or(UnitParseError::InvalidUnit)?;

  if number.is_empty() {
    Err(UnitParseError::InvalidNumber)
  } else {
    Ok((number, unit))
  }
}

#[inline]
fn parse_decimal_scaled_to_i64_bytes(
  value: &[u8],
  scale: i64,
  allow_negative: bool,
  max_fraction_digits: Option<usize>,
) -> Result<i64, UnitParseError> {
  if value.is_empty() {
    return Err(UnitParseError::Empty);
  }

  let (negative, digits) = match value[0] {
    b'-' if allow_negative => (true, &value[1..]),
    b'-' | b'+' => return Err(UnitParseError::InvalidNumber),
    _ => (false, value),
  };

  if digits.is_empty() {
    return Err(UnitParseError::InvalidNumber);
  }

  let mut integer: i128 = 0;
  let mut fraction: i128 = 0;
  let mut fraction_scale: i128 = 1;
  let mut seen_dot = false;
  let mut integer_digits = 0;
  let mut fraction_digits = 0;

  for &byte in digits {
    match byte {
      b'0'..=b'9' => {
        let digit = i128::from(byte - b'0');
        if seen_dot {
          fraction = fraction
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or(UnitParseError::Overflow)?;
          fraction_scale = fraction_scale
            .checked_mul(10)
            .ok_or(UnitParseError::Overflow)?;
          fraction_digits += 1;
          if max_fraction_digits.is_some_and(|max| fraction_digits > max) {
            return Err(UnitParseError::InvalidNumber);
          }
        } else {
          integer = integer
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or(UnitParseError::Overflow)?;
          integer_digits += 1;
        }
      }
      b'.' if !seen_dot => {
        if integer_digits == 0 {
          return Err(UnitParseError::InvalidNumber);
        }
        seen_dot = true;
      }
      _ => return Err(UnitParseError::InvalidNumber),
    }
  }

  if integer_digits == 0 || (seen_dot && fraction_digits == 0) {
    return Err(UnitParseError::InvalidNumber);
  }

  let scale = i128::from(scale);
  let whole = integer.checked_mul(scale).ok_or(UnitParseError::Overflow)?;
  let fractional = mul_div_round_i128(fraction, scale, fraction_scale)?;
  let unsigned = whole
    .checked_add(fractional)
    .ok_or(UnitParseError::Overflow)?;
  let signed = if negative { -unsigned } else { unsigned };

  i64::try_from(signed).map_err(|_| UnitParseError::Overflow)
}

#[inline]
fn parse_bare_twips_decimal_bytes(
  value: &[u8],
  allow_negative: bool,
) -> Result<i64, UnitParseError> {
  if !value.contains(&b'.') {
    return Err(UnitParseError::InvalidNumber);
  }
  parse_decimal_scaled_to_i64_bytes(value, 1, allow_negative, None)
}

#[inline(always)]
fn try_parse_u64_bytes(value: &[u8]) -> Option<u64> {
  let digits = match value {
    [b'+', rest @ ..] => rest,
    _ => value,
  };
  if digits.is_empty() {
    return None;
  }

  let mut parsed: u64 = 0;
  for &digit in digits {
    if !digit.is_ascii_digit() {
      return None;
    }
    parsed = parsed
      .checked_mul(10)?
      .checked_add(u64::from(digit - b'0'))?;
  }
  Some(parsed)
}

#[inline(always)]
fn try_parse_i32_bytes(value: &[u8]) -> Option<i32> {
  let (negative, digits) = match value {
    [b'-', rest @ ..] => (true, rest),
    [b'+', rest @ ..] => (false, rest),
    _ => (false, value),
  };
  if digits.is_empty() {
    return None;
  }

  let mut parsed: i32 = 0;
  for &digit in digits {
    if !digit.is_ascii_digit() {
      return None;
    }
    parsed = parsed.checked_mul(10).and_then(|current| {
      if negative {
        current.checked_sub(i32::from(digit - b'0'))
      } else {
        current.checked_add(i32::from(digit - b'0'))
      }
    })?;
  }
  Some(parsed)
}

#[inline(always)]
fn try_parse_i64_bytes(value: &[u8]) -> Option<i64> {
  let (negative, digits) = match value {
    [b'-', rest @ ..] => (true, rest),
    [b'+', rest @ ..] => (false, rest),
    _ => (false, value),
  };
  if digits.is_empty() {
    return None;
  }

  let mut parsed: i64 = 0;
  for &digit in digits {
    if !digit.is_ascii_digit() {
      return None;
    }
    parsed = parsed.checked_mul(10).and_then(|current| {
      if negative {
        current.checked_sub(i64::from(digit - b'0'))
      } else {
        current.checked_add(i64::from(digit - b'0'))
      }
    })?;
  }
  Some(parsed)
}

#[inline]
fn mul_div_round(value: i64, mul: i64, div: i64) -> i64 {
  let value = i128::from(value);
  let mul = i128::from(mul);
  let div = i128::from(div);
  let rounded = if value >= 0 {
    (value * mul + div / 2) / div
  } else {
    (value * mul - div / 2) / div
  };
  rounded.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

fn mul_div_round_i32(value: i32, mul: i32, div: i32) -> i32 {
  let rounded = mul_div_round(i64::from(value), i64::from(mul), i64::from(div));
  i64_to_i32_saturating(rounded)
}

fn mul_div_round_i128(value: i128, mul: i128, div: i128) -> Result<i128, UnitParseError> {
  if div <= 0 {
    return Err(UnitParseError::InvalidNumber);
  }
  let product = value.checked_mul(mul).ok_or(UnitParseError::Overflow)?;
  Ok(if product >= 0 {
    (product + div / 2) / div
  } else {
    (product - div / 2) / div
  })
}

fn format_decimal_ratio(value: i64, scale: i64, suffix: &str) -> String {
  const MAX_FRACTION_DIGITS: usize = 9;

  let negative = value.is_negative();
  let value = i128::from(value).abs();
  let scale = i128::from(scale);
  let rounding_scale = 10_i128.pow(MAX_FRACTION_DIGITS as u32);
  let rounded = ((value * rounding_scale) + scale / 2) / scale;
  let integer = rounded / rounding_scale;
  let fraction = rounded % rounding_scale;

  let mut output = String::new();
  if negative && rounded != 0 {
    output.push('-');
  }
  output.push_str(&integer.to_string());

  if fraction != 0 {
    let mut digits = format!("{fraction:0MAX_FRACTION_DIGITS$}");
    while digits.ends_with('0') {
      digits.pop();
    }
    output.push('.');
    output.push_str(&digits);
  }

  output.push_str(suffix);
  output
}

const fn u64_to_i64_saturating(value: u64) -> i64 {
  if value > i64::MAX as u64 {
    i64::MAX
  } else {
    value as i64
  }
}

const fn i64_to_i32_saturating(value: i64) -> i32 {
  if value > i32::MAX as i64 {
    i32::MAX
  } else if value < i32::MIN as i64 {
    i32::MIN
  } else {
    value as i32
  }
}

fn round_f64_to_i32(value: f64) -> i32 {
  if value.is_nan() {
    0
  } else if value >= i32::MAX as f64 {
    i32::MAX
  } else if value <= i32::MIN as f64 {
    i32::MIN
  } else {
    value.round() as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_universal_measure_to_emu_integer_unit() {
    assert_eq!(
      parse_universal_measure("1in"),
      Ok(UniversalMeasureValue::Inches(914_400))
    );
    assert_eq!(
      parse_universal_measure_bytes(b"1in"),
      Ok(UniversalMeasureValue::Inches(914_400))
    );
    assert_eq!(
      parse_universal_measure("12.5pt"),
      Ok(UniversalMeasureValue::Points(158_750))
    );
    assert_eq!(
      parse_universal_measure("-0.25in"),
      Ok(UniversalMeasureValue::Inches(-228_600))
    );
  }

  #[test]
  fn rejects_non_schema_universal_measure_lexical_forms() {
    assert_eq!(
      parse_universal_measure("+1in"),
      Err(UnitParseError::InvalidNumber)
    );
    assert_eq!(
      parse_universal_measure(".5in"),
      Err(UnitParseError::InvalidNumber)
    );
    assert_eq!(
      parse_universal_measure("1.in"),
      Err(UnitParseError::InvalidNumber)
    );
    assert_eq!(
      parse_positive_universal_measure("-1in"),
      Err(UnitParseError::InvalidNumber)
    );
  }

  #[test]
  fn parses_percent_string_to_drawingml_scale() {
    assert_eq!(parse_percent_string("100%"), Ok(100_000));
    assert_eq!(parse_percent_string_bytes(b"100%"), Ok(100_000));
    assert_eq!(parse_percent_string("12.5%"), Ok(12_500));
    assert_eq!(parse_percent_string("-1.25%"), Ok(-1_250));
    assert_eq!(
      parse_positive_percent_string("-1%"),
      Err(UnitParseError::InvalidNumber)
    );
  }

  #[test]
  fn parses_fixed_percent_with_schema_range_and_precision() {
    assert_eq!(parse_fixed_percent_string("100%"), Ok(100_000));
    assert_eq!(parse_fixed_percent_string("-12.5%"), Ok(-12_500));
    assert_eq!(parse_fixed_percent_string("12.55%"), Ok(12_550));
    assert_eq!(
      parse_fixed_percent_string("12.555%"),
      Err(UnitParseError::InvalidNumber)
    );
    assert_eq!(
      parse_fixed_percent_string("100.01%"),
      Err(UnitParseError::InvalidNumber)
    );
  }

  #[test]
  fn parses_union_types_from_bytes_without_allocating() {
    assert_eq!(
      TwipsMeasureValue::from_bytes(b"1440"),
      Ok(TwipsMeasureValue::Twips(1440))
    );
    assert_eq!(
      TwipsMeasureValue::from_bytes(b"1in"),
      Ok(TwipsMeasureValue::UniversalMeasure(
        UniversalMeasureValue::Inches(914_400)
      ))
    );
    assert_eq!(
      SignedTwipsMeasureValue::from_bytes(b"-720"),
      Ok(SignedTwipsMeasureValue::Twips(-720))
    );
    assert_eq!(
      DecimalNumberOrPercentValue::from_bytes(b"12.5%"),
      Ok(DecimalNumberOrPercentValue::Percent(12_500))
    );
    assert_eq!(
      MeasurementOrPercentValue::from_bytes(b"12.5pt"),
      Ok(MeasurementOrPercentValue::UniversalMeasure(
        UniversalMeasureValue::Points(158_750)
      ))
    );
    assert_eq!(
      Coordinate32Value::from_bytes(b"12.5pt"),
      Ok(Coordinate32Value::UniversalMeasure(
        UniversalMeasureValue::Points(158_750)
      ))
    );
    assert_eq!(
      DrawingmlPercentageValue::from_bytes(b"50000"),
      Ok(DrawingmlPercentageValue::Decimal(50_000))
    );
    assert_eq!(
      DrawingmlPercentageValue::from_bytes(b"50%"),
      Ok(DrawingmlPercentageValue::PercentString(50_000))
    );
    assert_eq!(
      TextPointValue::from_bytes(b"1200"),
      Ok(TextPointValue::Points100(1_200))
    );
    assert_eq!(
      TextPointValue::from_bytes(b"12pt"),
      Ok(TextPointValue::UniversalMeasure(
        UniversalMeasureValue::Points(152_400)
      ))
    );
    assert_eq!(
      TextBulletSizeValue::from_bytes(b"25000"),
      Ok(TextBulletSizeValue::Decimal(25_000))
    );
    assert_eq!(
      TextBulletSizeValue::from_bytes(b"25%"),
      Ok(TextBulletSizeValue::PercentString(25_000))
    );
  }

  #[test]
  fn parses_bare_decimal_twips_like_libreoffice() {
    assert_eq!(
      TwipsMeasureValue::from_bytes(b"1133.8582677165355"),
      Ok(TwipsMeasureValue::Twips(1134))
    );
    assert_eq!(
      SignedTwipsMeasureValue::from_bytes(b"-1133.8582677165355"),
      Ok(SignedTwipsMeasureValue::Twips(-1134))
    );
    assert_eq!(
      MeasurementOrPercentValue::from_bytes(b"108.0"),
      Ok(MeasurementOrPercentValue::DecimalNumberOrPercent(
        DecimalNumberOrPercentValue::DecimalNumber(108)
      ))
    );
    assert_eq!(
      MeasurementOrPercentValue::from_bytes(b"2256.5"),
      Ok(MeasurementOrPercentValue::DecimalNumberOrPercent(
        DecimalNumberOrPercentValue::DecimalNumber(2257)
      ))
    );
    assert_eq!(
      TwipsMeasureValue::from_bytes(b"-1.5"),
      Err(UnitParseError::InvalidNumber)
    );
  }

  #[test]
  fn converts_length_units_with_rounding() {
    assert_eq!(twips_to_emu(1), 635);
    assert_eq!(emu_to_twips(635), 1);
    assert_eq!(emu_to_mm100(360), 1);
    assert_eq!(mm100_to_emu(1), 360);
    assert_eq!(emu_to_half_points(12_700), 2);
    assert_eq!(half_points_to_emu(2), 12_700);
    assert_eq!(drawingml_text_size_to_half_points(1_200), 24);
    assert_eq!(half_points_to_drawingml_text_size(24), 1_200);
    assert_eq!(drawingml_text_size_to_emu(1_200), 152_400);
    assert_eq!(emu_to_drawingml_text_size(152_400), 1_200);
    assert_eq!(points100_to_mm100(7_200), 2_540);
    assert_eq!(mm100_to_points100(2_540), 7_200);
    assert_eq!(text_spacing_point_to_mm100(7_200), 2_540);
    assert_eq!(mm100_to_text_spacing_point(2_540), 7_200);
  }

  #[test]
  fn converts_vml_fixed_and_rotation_units() {
    assert_eq!(ratio_to_vml_fixed(1.0), 65_536);
    assert_eq!(vml_fixed_angle_to_degree100(1_966_080), 33_000);
    assert_eq!(vml_rotation_degrees_to_degree100(30.0), 33_000);
    assert_eq!(normalize_degree100(-1), 35_999);
  }

  #[test]
  fn exposes_schema_range_helpers() {
    assert!(CoordinateValue::Emu(DRAWINGML_COORDINATE_MAX).is_valid_coordinate());
    assert!(!CoordinateValue::Emu(DRAWINGML_COORDINATE_MAX + 1).is_valid_coordinate());
    assert!(is_valid_drawingml_fixed_percent(100_000));
    assert!(!is_valid_drawingml_fixed_percent(100_001));
    assert!(is_valid_drawingml_fixed_angle(5_399_999));
    assert!(!is_valid_drawingml_fixed_angle(5_400_000));
  }

  #[test]
  fn formats_source_unit_from_canonical_emu() {
    assert_eq!(
      UniversalMeasureValue::Points(158_750).to_lexical(),
      "12.5pt"
    );
    assert_eq!(
      UniversalMeasureValue::Inches(-228_600).to_lexical(),
      "-0.25in"
    );
    assert_eq!(
      DecimalNumberOrPercentValue::Percent(12_500).percent_to_lexical(),
      Some("12.5%".to_string())
    );
  }
}
