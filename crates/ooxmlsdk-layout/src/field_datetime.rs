use crate::localization::canonical_locale;
use crate::options::FieldUpdateDateTime;
use icu_calendar::Gregorian;
use icu_calendar::cal::Japanese;
use icu_datetime::fieldsets::{T, YMD, YMDE};
use icu_datetime::input::{Date, DateTime, Time};
use icu_datetime::options::{TimePrecision, YearStyle};
use icu_datetime::pattern::{DateTimePattern, FixedCalendarDateTimeNames};
use icu_datetime::{FixedCalendarDateTimeFormatter, NoCalendarFormatter};
use writeable::TryWriteable;

const ABBREVIATED_DAY_PERIOD_MARKER: char = '\u{e000}';

/// Differences proved by Office output but not represented by CLDR's locale
/// defaults. Keep these as narrow data overrides above the generic ICU path.
struct OfficeDateTimeLocaleProfile {
  language: &'static str,
  region: &'static str,
  short_date_picture: &'static str,
  document_date_time_picture: &'static str,
}

const OFFICE_DATE_TIME_LOCALE_PROFILES: &[OfficeDateTimeLocaleProfile] =
  &[OfficeDateTimeLocaleProfile {
    language: "en",
    region: "IN",
    // PowerPoint numfmt.pptx and the Word document-property fixtures use the
    // Windows/Office `en-IN` dash form; CLDR's generic English fallback uses
    // slashes. All other locale data remains ICU-owned.
    short_date_picture: "dd-MM-yyyy",
    document_date_time_picture: "dd-MM-yyyy HH:mm:ss",
  }];

pub(crate) fn format_date_time_field(
  tokens: &[String],
  language: Option<&str>,
  mut value: FieldUpdateDateTime,
) -> Option<String> {
  let field_name = tokens.first()?;
  let default_format = if field_name.eq_ignore_ascii_case("DATE") {
    DefaultFieldFormat::Date
  } else if field_name.eq_ignore_ascii_case("TIME") {
    DefaultFieldFormat::Time
  } else if field_name.eq_ignore_ascii_case("PRINTDATE")
    || field_name.eq_ignore_ascii_case("SAVEDATE")
  {
    // PRINTDATE and SAVEDATE expose document-property times. The Office
    // fixed-output fixtures consistently persist those properties at minute
    // precision, unlike current DATE/TIME fields, so do not invent seconds
    // from the later conversion-manifest timestamp.
    value.second = 0;
    DefaultFieldFormat::DocumentDateTime
  } else if field_name.eq_ignore_ascii_case("CREATEDATE") {
    // CREATEDATE is sourced from the absolute dcterms:created core property.
    // Unlike PRINTDATE/SAVEDATE, preserve its recorded seconds after the
    // caller-selected time-zone conversion.
    DefaultFieldFormat::DocumentDateTime
  } else {
    return None;
  };
  if !valid_date_time(value) {
    return None;
  }

  let mut picture = None;
  for (index, token) in tokens.iter().enumerate().skip(1) {
    if token.eq_ignore_ascii_case(r"\@") {
      picture = Some(tokens.get(index + 1)?.trim());
      break;
    }
  }
  if let Some(picture) = picture {
    return format_picture(picture, language, value);
  }
  match default_format {
    DefaultFieldFormat::Date => format_office_short_date(language, value),
    DefaultFieldFormat::Time => format_office_default_time(language, value),
    DefaultFieldFormat::DocumentDateTime => format_office_document_date_time(language, value),
  }
}

#[derive(Clone, Copy)]
enum DefaultFieldFormat {
  Date,
  Time,
  DocumentDateTime,
}

pub(crate) fn format_office_short_date(
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  if let Some(profile) = office_date_time_locale_profile(language) {
    return format_picture(profile.short_date_picture, language, value);
  }
  let locale = field_locale(language)?;
  let date = field_date(value)?;
  let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    locale.into(),
    YMD::short().with_year_style(YearStyle::Full),
  )
  .ok()?;
  Some(normalize_office_field_output(
    formatter.format(&date).to_string(),
    language,
  ))
}

/// Returns the locale-resolved numeric short-date picture in SpreadsheetML's
/// number-format vocabulary.
///
/// Excel built-in format id 14 follows the application's format locale even
/// though no custom `numFmt` is stored in the package. ICU owns the regional
/// field order and separators; only the year width is promoted to four digits
/// to match Office's built-in fixed-output form.
pub(crate) fn spreadsheet_builtin_short_date_picture(language: Option<&str>) -> Option<String> {
  let locale = field_locale(language)?;
  let date = Date::try_new_gregorian(2006, 11, 23).ok()?;
  let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    locale.into(),
    YMD::short().with_year_style(YearStyle::Full),
  )
  .ok()?;
  let formatted = formatter.format(&date);
  spreadsheet_picture_from_icu_date_pattern(&formatted.pattern().to_string())
}

fn spreadsheet_picture_from_icu_date_pattern(pattern: &str) -> Option<String> {
  let mut output = String::new();
  let mut characters = pattern.chars().peekable();
  let mut quoted = false;
  while let Some(character) = characters.next() {
    if character == '\'' {
      if characters.peek() == Some(&'\'') {
        characters.next();
        if !quoted {
          output.push('"');
        }
        output.push('\'');
        if !quoted {
          output.push('"');
        }
      } else {
        quoted = !quoted;
        output.push('"');
      }
      continue;
    }
    if quoted {
      if character == '"' {
        output.push_str("\"\"");
      } else {
        output.push(character);
      }
      continue;
    }
    let replacement = match character {
      'y' | 'Y' | 'u' => Some('Y'),
      'M' | 'L' => Some('M'),
      'd' => Some('D'),
      _ if character.is_ascii_alphabetic() => return None,
      _ => None,
    };
    if let Some(replacement) = replacement {
      let mut width = 1usize;
      while characters.peek() == Some(&character) {
        characters.next();
        width += 1;
      }
      if replacement == 'Y' {
        output.push_str("YYYY");
      } else {
        output.extend(std::iter::repeat_n(replacement, width));
      }
    } else {
      output.push(character);
    }
  }
  if quoted {
    return None;
  }
  Some(output)
}

pub(crate) fn format_office_long_date(
  language: Option<&str>,
  value: FieldUpdateDateTime,
  include_weekday: bool,
) -> Option<String> {
  let locale = field_locale(language)?;
  let date = field_date(value)?;
  let formatted = if include_weekday {
    FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
      locale.into(),
      YMDE::long().with_year_style(YearStyle::Full),
    )
    .ok()?
    .format(&date)
    .to_string()
  } else {
    FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
      locale.into(),
      YMD::long().with_year_style(YearStyle::Full),
    )
    .ok()?
    .format(&date)
    .to_string()
  };
  Some(normalize_office_field_output(formatted, language))
}

pub(crate) fn format_spreadsheet_system_long_date(
  language: Option<&str>,
  value: FieldUpdateDateTime,
  compatibility_weekday: Option<u8>,
) -> Option<String> {
  let locale = field_locale(language)?;
  // Excel's F800 format omits the weekday in zh-CN fixed output, even
  // when its saved fallback picture contains dddd. This is independent
  // from PowerPoint fields that explicitly request a weekday.
  let chinese_mainland = locale.id.language.as_str() == "zh"
    && locale
      .id
      .region
      .is_some_and(|region| region.as_str() == "CN");
  if compatibility_weekday.is_none() {
    return format_office_long_date(language, value, !chinese_mainland);
  }
  let date = field_date(spreadsheet_calendar_anchor(value))?;
  let pattern = if chinese_mainland {
    FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
      locale.into(),
      YMD::long().with_year_style(YearStyle::Full),
    )
    .ok()?
    .format(&date)
    .pattern()
    .to_string()
  } else {
    FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
      locale.into(),
      YMDE::long().with_year_style(YearStyle::Full),
    )
    .ok()?
    .format(&date)
    .pattern()
    .to_string()
  };
  let pattern = spreadsheet_calendar_pattern(&pattern, language, value, compatibility_weekday)?;
  format_icu_picture(
    &pattern,
    language,
    spreadsheet_calendar_anchor(value),
    false,
  )
}

pub(crate) fn format_office_short_time(
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let locale = field_locale(language)?;
  let time = field_time(value)?;
  // ICU field-set length does not select time precision: its default still
  // includes seconds. Office short-time fields retain only hours and minutes.
  let formatter = NoCalendarFormatter::try_new(
    locale.into(),
    T::short().with_time_precision(TimePrecision::Minute),
  )
  .ok()?;
  Some(normalize_office_field_output(
    formatter.format(&time).to_string(),
    language,
  ))
}

pub(crate) fn format_office_default_time(
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let locale = field_locale(language)?;
  let time = field_time(value)?;
  let formatter = NoCalendarFormatter::try_new(locale.into(), T::medium()).ok()?;
  Some(normalize_office_field_output(
    formatter.format(&time).to_string(),
    language,
  ))
}

pub(crate) fn format_spreadsheet_system_time(
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let value = spreadsheet_calendar_anchor(value);
  let locale = field_locale(language)?;
  // Excel F400 uses Windows' unpadded zh-CN hour at 0, 6 and 9, even
  // when the saved fallback picture contains hh. CLDR pads this hour.
  if locale.id.language.as_str() == "zh"
    && locale
      .id
      .region
      .is_some_and(|region| region.as_str() == "CN")
  {
    return format_picture("H:mm:ss", language, value);
  }
  format_office_default_time(language, value)
}

fn format_office_document_date_time(
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  if let Some(profile) = office_date_time_locale_profile(language) {
    return format_picture(profile.document_date_time_picture, language, value);
  }
  Some(format!(
    "{} {}",
    format_office_short_date(language, value)?,
    format_office_default_time(language, value)?
  ))
}

fn format_picture(
  picture: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let (pattern, abbreviate_day_period) = office_picture_to_icu_pattern(picture)?;
  format_icu_picture(&pattern, language, value, abbreviate_day_period)
}

fn format_icu_picture(
  pattern: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
  abbreviate_day_period: bool,
) -> Option<String> {
  let pattern = DateTimePattern::try_from_pattern_str(pattern).ok()?;
  let locale = field_locale(language)?;
  let datetime = field_date_time(value)?;
  let mut names = FixedCalendarDateTimeNames::<Gregorian>::try_new(locale.into()).ok()?;
  let formatter = names.include_for_pattern(&pattern).ok()?;
  let mut formatted = formatter
    .format(&datetime)
    .try_write_to_string()
    .ok()?
    .into_owned();
  if abbreviate_day_period {
    let day_period = format_picture("am/pm", language, value)?;
    let abbreviated = day_period.chars().next()?;
    formatted = formatted.replace(ABBREVIATED_DAY_PERIOD_MARKER, &abbreviated.to_string());
  }
  Some(normalize_office_field_output(formatted, language))
}

fn office_picture_to_icu_pattern(picture: &str) -> Option<(String, bool)> {
  let chars = picture.chars().collect::<Vec<_>>();
  let mut output = String::new();
  let mut index = 0;
  let mut abbreviate_day_period = false;
  while index < chars.len() {
    if chars[index] == '\'' {
      output.push('\'');
      index += 1;
      let mut closed = false;
      while index < chars.len() {
        output.push(chars[index]);
        if chars[index] != '\'' {
          index += 1;
          continue;
        }
        if chars.get(index + 1) == Some(&'\'') {
          output.push('\'');
          index += 2;
          continue;
        }
        index += 1;
        closed = true;
        break;
      }
      if !closed {
        return None;
      }
      continue;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "am/pm") {
      output.push('a');
      index += "am/pm".len();
      continue;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "a/p") {
      output.push('\'');
      output.push(ABBREVIATED_DAY_PERIOD_MARKER);
      output.push('\'');
      abbreviate_day_period = true;
      index += "a/p".len();
      continue;
    }

    let ch = chars[index];
    let count = chars[index..]
      .iter()
      .take_while(|candidate| **candidate == ch)
      .count();
    match ch {
      'M' if (1..=5).contains(&count) => output.extend(chars[index..index + count].iter()),
      'd' | 'D' if count <= 2 => output.extend(std::iter::repeat_n('d', count)),
      'd' | 'D' if count == 3 => output.push_str("EEE"),
      'd' | 'D' if count == 4 => output.push_str("EEEE"),
      'y' | 'Y' if count == 1 || count == 2 => output.push_str("yy"),
      'y' | 'Y' if count == 4 => output.push_str("yyyy"),
      'h' | 'H' | 'm' | 's' if count <= 2 => output.extend(chars[index..index + count].iter()),
      _ if ch.is_ascii_alphabetic() => return None,
      _ => output.extend(chars[index..index + count].iter()),
    }
    index += count;
  }
  Some((output, abbreviate_day_period))
}

pub(crate) fn format_date_time_picture(
  picture: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  valid_date_time(value).then(|| format_picture(picture, language, value))?
}

/// Formats a SpreadsheetML number-format date picture through the shared
/// ICU4X date-name and locale pipeline used by Office fields.
///
/// Spreadsheet number formats use lower-case `m` for months, double quotes
/// for literals, backslash escapes, semicolon-separated value sections, and
/// optional legacy LCID markers. Word field pictures use upper-case `M` and
/// ICU-compatible single-quoted literals. Keeping this translation at the
/// formatting boundary avoids teaching chart layout about localized month
/// names or duplicating the field formatter's locale behavior.
pub(crate) fn format_spreadsheet_date_picture(
  picture: &str,
  fallback_language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  format_spreadsheet_date_picture_with_weekday(picture, fallback_language, value, None)
}

/// The first 61 days of Excel's 1900 calendar have compatibility weekdays,
/// including the fictitious January 0 and February 29. Weekdays are Monday=0.
pub(crate) fn format_spreadsheet_date_picture_with_weekday(
  picture: &str,
  fallback_language: Option<&str>,
  value: FieldUpdateDateTime,
  compatibility_weekday: Option<u8>,
) -> Option<String> {
  let embedded_language = spreadsheet_date_picture_language(picture);
  let language = embedded_language.as_deref().or(fallback_language);
  let anchor = spreadsheet_calendar_anchor(value);
  let picture = spreadsheet_date_picture_to_field_picture(picture, language, anchor)?;
  let (pattern, abbreviate_day_period) = office_picture_to_icu_pattern(&picture)?;
  let pattern = spreadsheet_calendar_pattern(&pattern, language, value, compatibility_weekday)?;
  format_icu_picture(&pattern, language, anchor, abbreviate_day_period)
}

fn spreadsheet_calendar_anchor(value: FieldUpdateDateTime) -> FieldUpdateDateTime {
  let day = match (value.year, value.month, value.day) {
    (1900, 1, 0) => 1,
    (1900, 2, 29) => 28,
    _ => value.day,
  };
  FieldUpdateDateTime { day, ..value }
}

fn spreadsheet_calendar_pattern(
  pattern: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
  compatibility_weekday: Option<u8>,
) -> Option<String> {
  let replace_day = spreadsheet_calendar_anchor(value).day != value.day;
  if !replace_day && compatibility_weekday.is_none() {
    return Some(pattern.to_owned());
  }
  // Let ICU supply localized month/era/time names from a valid civil date,
  // replacing only Excel's exceptional day and weekday fields with literals.
  // This leaves the Gregorian validation of Word/PowerPoint fields intact.
  let mut chars = pattern.chars().peekable();
  let mut output = String::new();
  while let Some(ch) = chars.next() {
    if ch == '\'' {
      let mut literal = String::new();
      if chars.next_if_eq(&'\'').is_some() {
        literal.push('\'');
      } else {
        loop {
          let ch = chars.next()?;
          if ch == '\'' {
            if chars.next_if_eq(&'\'').is_none() {
              break;
            }
          }
          literal.push(ch);
        }
      }
      push_icu_quoted_literal(&mut output, &literal);
      continue;
    }
    let mut count = 1;
    while chars.next_if_eq(&ch).is_some() {
      count += 1;
    }
    if ch == 'd' && replace_day {
      push_icu_quoted_literal(&mut output, &format!("{:0count$}", value.day));
    } else if ch == 'E'
      && let Some(weekday) = compatibility_weekday
    {
      let weekday_value = FieldUpdateDateTime {
        year: 2000,
        month: 1,
        day: 3 + weekday, // January 3, 2000 was Monday.
        ..value
      };
      let name = format_icu_picture(&"E".repeat(count), language, weekday_value, false)?;
      push_icu_quoted_literal(&mut output, &name);
    } else {
      output.extend(std::iter::repeat_n(ch, count));
    }
  }
  Some(output)
}

fn spreadsheet_date_picture_to_field_picture(
  picture: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let chars = picture.chars().collect::<Vec<_>>();
  let mut output = String::new();
  let mut index = 0usize;
  let mut saw_date_token = false;
  let mut previous_field = None;
  let mut month_width = None;
  let uses_day_period =
    picture.to_ascii_lowercase().contains("am/pm") || picture.to_ascii_lowercase().contains("a/p");
  while index < chars.len() {
    let ch = chars[index];
    if ch == ';' {
      break;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "am/pm") {
      // SpreadsheetML's explicit AM/PM token keeps these labels even
      // under a Chinese format locale (built-in 18/19 and custom formats).
      // System-time F400 is resolved separately by the worksheet formatter.
      push_icu_quoted_literal(&mut output, if value.hour < 12 { "AM" } else { "PM" });
      previous_field = Some('a');
      index += "am/pm".len();
      continue;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "a/p") {
      output.push_str("a/p");
      previous_field = Some('a');
      index += "a/p".len();
      continue;
    }
    match ch {
      '[' => {
        let end = chars[index + 1..]
          .iter()
          .position(|candidate| *candidate == ']')
          .map(|offset| index + 1 + offset)?;
        let marker = chars[index + 1..end].iter().collect::<String>();
        if matches!(
          marker.to_ascii_lowercase().as_str(),
          "h" | "hh" | "m" | "mm" | "s" | "ss"
        ) {
          // Elapsed-time brackets are not calendar fields. The worksheet
          // formatter handles them before entering this locale formatter.
          return None;
        }
        index = end + 1;
      }
      '"' => {
        index += 1;
        let mut literal = String::new();
        while index < chars.len() && chars[index] != '"' {
          literal.push(chars[index]);
          index += 1;
        }
        if index < chars.len() {
          index += 1;
        }
        push_icu_quoted_literal(&mut output, &literal);
      }
      '\\' => {
        index += 1;
        if let Some(literal) = chars.get(index) {
          push_icu_quoted_literal(&mut output, &literal.to_string());
          index += 1;
        }
      }
      '_' | '*' => {
        index = (index + 2).min(chars.len());
      }
      'y' | 'Y' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count();
        output.push_str(if count <= 2 { "yy" } else { "yyyy" });
        saw_date_token = true;
        previous_field = Some('y');
        index += count;
      }
      'g' | 'G' | 'e' | 'E' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count();
        let text = spreadsheet_era_field(ch.to_ascii_lowercase(), count, language, value)?;
        push_icu_quoted_literal(&mut output, &text);
        saw_date_token = true;
        previous_field = Some(ch.to_ascii_lowercase());
        index += count;
      }
      'm' | 'M' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count()
          .min(5);
        let minute = previous_field == Some('h')
          || next_spreadsheet_date_time_field(&chars, index + count) == Some('s');
        if !minute
          && count == 4
          && let Some(name) = office_spreadsheet_wide_month_name(language, value.month)
        {
          push_icu_quoted_literal(&mut output, name);
        } else if !minute && month_width.is_some_and(|width| width != count) {
          // ICU's pattern names store allows only one width per field.
          // Excel permits a month name and its numeric/abbreviated form in
          // the same picture. Resolve the additional width through ICU
          // separately, without touching authored literals or minute fields.
          let name = format_icu_picture(&"M".repeat(count), language, value, false)?;
          push_icu_quoted_literal(&mut output, &name);
        } else {
          output.extend(std::iter::repeat_n(if minute { 'm' } else { 'M' }, count));
          if !minute {
            month_width = Some(count);
          }
        }
        saw_date_token = true;
        previous_field = Some('m');
        index += count;
      }
      'd' | 'D' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count()
          .min(4);
        output.extend(std::iter::repeat_n('d', count));
        saw_date_token = true;
        previous_field = Some('d');
        index += count;
      }
      'h' | 'H' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count()
          .min(2);
        output.extend(std::iter::repeat_n(
          if uses_day_period { 'h' } else { 'H' },
          count,
        ));
        saw_date_token = true;
        previous_field = Some('h');
        index += count;
      }
      's' | 'S' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count()
          .min(2);
        output.extend(std::iter::repeat_n('s', count));
        saw_date_token = true;
        previous_field = Some('s');
        index += count;
      }
      '@' => index += 1,
      _ if ch.is_ascii_alphabetic() => return None,
      _ => {
        output.push(ch);
        index += 1;
      }
    }
  }
  saw_date_token.then_some(output)
}

fn spreadsheet_date_picture_language(picture: &str) -> Option<String> {
  let mut chars = picture.chars();
  let mut language = None;
  while let Some(ch) = chars.next() {
    match ch {
      ';' => break,
      '\\' | '_' | '*' => {
        chars.next();
      }
      '"' => {
        for ch in chars.by_ref() {
          if ch == '"' {
            break;
          }
        }
      }
      '[' => {
        let marker = chars
          .by_ref()
          .take_while(|ch| *ch != ']')
          .collect::<String>();
        let Some((_, culture)) = marker.strip_prefix('$').and_then(|s| s.split_once('-')) else {
          continue;
        };
        // Legacy LCIDs and formatCode16 culture tags select the format
        // language independently of the application's UI/format locale.
        let resolved = spreadsheet_lcid_language(culture)
          .map(ToOwned::to_owned)
          .or_else(|| canonical_locale(culture).map(|_| culture.to_owned()));
        if resolved.is_some() {
          language = resolved;
        }
      }
      _ => {}
    }
  }
  language
}

fn spreadsheet_era_field(
  field: char,
  width: usize,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let locale = field_locale(language)?;
  if locale.id.language.as_str() != "ja" {
    // MS-OI29500 2.1.713(f): Office maps both e and ee to yyyy outside
    // Japan and Taiwan, overriding ECMA-376's two-digit-year rule.
    // Taiwanese eras require a separate calendar implementation.
    let taiwanese = locale.id.language.as_str() == "zh"
      && locale
        .id
        .region
        .is_some_and(|region| region.as_str() == "TW");
    return (field == 'e' && !taiwanese).then(|| format!("{:04}", value.year));
  }
  let date = field_date(value)?.to_calendar(Japanese::new());
  if field == 'e' {
    let year = date.era_year().year;
    // Excel's explicit Gannen format displays the first year as 元; the
    // calendar data, rather than a fixture-specific date subtraction,
    // decides the era and the year at every transition.
    if year == 1
      && language.is_some_and(|language| language.to_ascii_lowercase().contains("-x-gannen"))
    {
      return Some("元".to_owned());
    }
    return Some(format!("{year:0width$}", width = width.min(2)));
  }
  let pattern =
    DateTimePattern::try_from_pattern_str(if width == 1 { "GGGGG" } else { "GGGG" }).ok()?;
  let mut names = FixedCalendarDateTimeNames::<Japanese>::try_new(locale.into()).ok()?;
  let formatter = names.include_for_pattern(&pattern).ok()?;
  let datetime = DateTime {
    date,
    time: field_time(value)?,
  };
  let name = formatter
    .format(&datetime)
    .try_write_to_string()
    .ok()?
    .into_owned();
  Some(if width == 2 {
    name.chars().take(1).collect()
  } else {
    name
  })
}

fn next_spreadsheet_date_time_field(chars: &[char], mut index: usize) -> Option<char> {
  while index < chars.len() {
    match chars[index] {
      ';' => return None,
      '[' => {
        index += 1;
        while index < chars.len() && chars[index] != ']' {
          index += 1;
        }
        index = (index + 1).min(chars.len());
      }
      '"' => {
        index += 1;
        while index < chars.len() && chars[index] != '"' {
          index += 1;
        }
        index = (index + 1).min(chars.len());
      }
      '\\' | '_' | '*' => index = (index + 2).min(chars.len()),
      field if matches!(field.to_ascii_lowercase(), 'y' | 'm' | 'd' | 'h' | 's') => {
        return Some(field.to_ascii_lowercase());
      }
      field if field.is_ascii_alphabetic() => return Some(field.to_ascii_lowercase()),
      _ => index += 1,
    }
  }
  None
}

fn push_icu_quoted_literal(output: &mut String, literal: &str) {
  // Keep adjacent SpreadsheetML escapes in one ICU literal. Emitting
  // `'.'' '` for `\.\ ` makes the middle `''` an authored apostrophe;
  // Excel instead renders the escaped dot and space as one literal run.
  if output.ends_with('\'') {
    output.pop();
  } else {
    output.push('\'');
  }
  for ch in literal.chars() {
    output.push(ch);
    if ch == '\'' {
      output.push('\'');
    }
  }
  output.push('\'');
}

fn office_spreadsheet_wide_month_name(language: Option<&str>, month: u8) -> Option<&'static str> {
  // Excel's sr-Cyrl-BA twelve-month controls and Windows GetLocaleInfoEx
  // agree on these two regional spellings; CLDR uses Serbia's јун/јул.
  // Other months, abbreviations, weekdays and scripts remain ICU-owned.
  let locale = canonical_locale(language?)?;
  if locale.id.language.as_str() != "sr"
    || locale
      .id
      .region
      .is_none_or(|region| region.as_str() != "BA")
    || locale
      .id
      .script
      .is_some_and(|script| script.as_str() != "Cyrl")
  {
    return None;
  }
  match month {
    6 => Some("јуни"),
    7 => Some("јули"),
    _ => None,
  }
}

fn spreadsheet_lcid_language(value: &str) -> Option<&'static str> {
  // OOXML keeps legacy hexadecimal Windows LCIDs in number-format markers.
  // ICU4X owns locale data once the identifier is BCP 47; this deliberately
  // small bridge covers the common Office authoring locales and remains
  // independent from UI-language selection. Unknown LCIDs fall back to the
  // caller's format locale instead of silently selecting English.
  match u32::from_str_radix(value.trim_start_matches('0'), 16).ok()? {
    0x0404 => Some("zh-TW"),
    0x0407 => Some("de-DE"),
    0x0409 => Some("en-US"),
    0x040c => Some("fr-FR"),
    0x040e => Some("hu-HU"),
    0x0410 => Some("it-IT"),
    0x0411 => Some("ja-JP"),
    0x0412 => Some("ko-KR"),
    0x0415 => Some("pl-PL"),
    0x0416 => Some("pt-BR"),
    0x0419 => Some("ru-RU"),
    0x0804 => Some("zh-CN"),
    0x0809 => Some("en-GB"),
    0x0816 => Some("pt-PT"),
    // MS-LCID 2.2: Australian English is distinct from the caller's locale.
    0x0c09 => Some("en-AU"),
    0x0c0a => Some("es-ES"),
    // MS-LCID 2.2: Serbian Cyrillic, Bosnia and Herzegovina.
    0x1c1a => Some("sr-Cyrl-BA"),
    _ => None,
  }
}

fn office_date_time_locale_profile(
  language: Option<&str>,
) -> Option<&'static OfficeDateTimeLocaleProfile> {
  let locale = canonical_locale(language?)?;
  OFFICE_DATE_TIME_LOCALE_PROFILES.iter().find(|profile| {
    locale.id.language.as_str() == profile.language
      && locale
        .id
        .region
        .is_some_and(|region| region.as_str() == profile.region)
  })
}

fn field_locale(language: Option<&str>) -> Option<icu_locale::Locale> {
  language
    .and_then(canonical_locale)
    .or_else(|| canonical_locale("en-US"))
}

fn field_date(value: FieldUpdateDateTime) -> Option<Date<Gregorian>> {
  valid_date_time(value).then_some(())?;
  Date::try_new_gregorian(i32::from(value.year), value.month, value.day).ok()
}

fn field_time(value: FieldUpdateDateTime) -> Option<Time> {
  valid_date_time(value).then_some(())?;
  Time::try_new(value.hour, value.minute, value.second, 0).ok()
}

fn field_date_time(value: FieldUpdateDateTime) -> Option<DateTime<Gregorian>> {
  Some(DateTime {
    date: field_date(value)?,
    time: field_time(value)?,
  })
}

fn normalize_office_field_output(value: String, language: Option<&str>) -> String {
  let value = value.replace(['\u{00a0}', '\u{202f}'], " ");
  if !english_language(language) {
    return value;
  }
  uppercase_ascii_day_periods(value)
}

fn english_language(language: Option<&str>) -> bool {
  language.is_none_or(|language| {
    canonical_locale(language).is_some_and(|locale| locale.id.language.as_str() == "en")
  })
}

fn uppercase_ascii_day_periods(value: String) -> String {
  let chars = value.chars().collect::<Vec<_>>();
  let mut output = String::with_capacity(value.len());
  let mut index = 0;
  while index < chars.len() {
    let previous_is_letter = index > 0 && chars[index - 1].is_ascii_alphabetic();
    let next_is_letter = chars.get(index + 2).is_some_and(char::is_ascii_alphabetic);
    if !previous_is_letter
      && !next_is_letter
      && chars
        .get(index..index + 2)
        .is_some_and(|token| ascii_prefix_eq_ignore_case(token, "am"))
    {
      output.push_str("AM");
      index += 2;
    } else if !previous_is_letter
      && !next_is_letter
      && chars
        .get(index..index + 2)
        .is_some_and(|token| ascii_prefix_eq_ignore_case(token, "pm"))
    {
      output.push_str("PM");
      index += 2;
    } else {
      output.push(chars[index]);
      index += 1;
    }
  }
  output
}

fn ascii_prefix_eq_ignore_case(chars: &[char], expected: &str) -> bool {
  let expected = expected.chars().collect::<Vec<_>>();
  chars.len() >= expected.len()
    && chars[..expected.len()]
      .iter()
      .zip(expected)
      .all(|(actual, expected)| actual.eq_ignore_ascii_case(&expected))
}

fn valid_date_time(value: FieldUpdateDateTime) -> bool {
  value.month >= 1
    && value.month <= 12
    && value.day >= 1
    && value.day <= days_in_month(value.year, value.month)
    && value.hour <= 23
    && value.minute <= 59
    && value.second <= 59
}

fn days_in_month(year: u16, month: u8) -> u8 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if leap_year(year) => 29,
    2 => 28,
    _ => 0,
  }
}

fn leap_year(year: u16) -> bool {
  year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[cfg(test)]
mod tests {
  use super::{format_date_time_field, format_office_default_time, format_office_short_time};
  use crate::options::FieldUpdateDateTime;

  const VALUE: FieldUpdateDateTime = FieldUpdateDateTime {
    year: 2026,
    month: 7,
    day: 12,
    hour: 20,
    minute: 19,
    second: 54,
  };

  fn tokens(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
  }

  #[test]
  fn office_short_time_keeps_minutes_and_omits_seconds() {
    for second in [0, 3, 54, 59] {
      let value = FieldUpdateDateTime { second, ..VALUE };
      for (locale, expected) in [("zh-CN", "20:19"), ("en-US", "8:19 PM"), ("de-DE", "20:19")] {
        assert_eq!(
          format_office_short_time(Some(locale), value).as_deref(),
          Some(expected)
        );
      }
    }
    let whole_hour = FieldUpdateDateTime {
      minute: 0,
      second: 59,
      ..VALUE
    };
    assert_eq!(
      format_office_short_time(Some("en-US"), whole_hour).as_deref(),
      Some("8:00 PM")
    );
    assert_eq!(
      format_office_short_time(Some("zh-CN"), whole_hour).as_deref(),
      Some("20:00")
    );
  }

  #[test]
  fn office_short_time_keeps_day_periods_at_hour_boundaries() {
    for (hour, minute, expected) in [
      (0, 0, "12:00 AM"),
      (11, 59, "11:59 AM"),
      (12, 0, "12:00 PM"),
      (23, 59, "11:59 PM"),
    ] {
      let value = FieldUpdateDateTime {
        hour,
        minute,
        second: 59,
        ..VALUE
      };
      assert_eq!(
        format_office_short_time(Some("en-US"), value).as_deref(),
        Some(expected)
      );
    }
    // The retained Invoice Tracking Office PDF prints 23:59 for a supplied
    // civil clock of 23:59:46, without carrying into the following day.
    let end_of_day = FieldUpdateDateTime {
      hour: 23,
      minute: 59,
      second: 59,
      ..VALUE
    };
    assert_eq!(
      format_office_short_time(Some("zh-CN"), end_of_day).as_deref(),
      Some("23:59")
    );
  }

  #[test]
  fn office_default_time_retains_its_seconds() {
    assert_eq!(
      format_office_default_time(Some("zh-CN"), VALUE).as_deref(),
      Some("20:19:54")
    );
    assert_eq!(
      format_office_default_time(Some("en-US"), VALUE).as_deref(),
      Some("8:19:54 PM")
    );
  }

  #[test]
  fn formats_office_golden_numeric_and_time_pictures() {
    assert_eq!(
      format_date_time_field(&tokens(&["DATE", r"\@", "MM/DD/YY"]), Some("en-US"), VALUE),
      Some("07/12/26".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["TIME", r"\@", "dd.MM.yyyy"]),
        Some("en-US"),
        VALUE
      ),
      Some("12.07.2026".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["TIME", r"\@", "M/d/yyyy h:mm:ss am/pm"]),
        Some("en-GB"),
        VALUE,
      ),
      Some("7/12/2026 8:19:54 PM".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["DATE", r"\@", "h时m分s秒"]),
        Some("en-US"),
        FieldUpdateDateTime {
          hour: 15,
          minute: 21,
          second: 43,
          ..VALUE
        },
      ),
      Some("3时21分43秒".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["PRINTDATE", r"\@", "h:mm:ss am/pm"]),
        Some("en-US"),
        VALUE,
      ),
      Some("8:19:00 PM".to_string())
    );
  }

  #[test]
  fn formats_english_names_weekdays_and_quoted_literals() {
    assert_eq!(
      format_date_time_field(&tokens(&["DATE", r"\@", "d-MMM-yy"]), Some("en-GB"), VALUE),
      Some("12-Jul-26".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["DATE", r"\@", "dddd, MMMM dd, yyyy"]),
        Some("en-US"),
        VALUE,
      ),
      Some("Sunday, July 12, 2026".to_string())
    );
    assert_eq!(
      format_date_time_field(
        &tokens(&["TIME", r"\@", "'Today is 'HH:mm:ss"]),
        Some("en-US"),
        VALUE,
      ),
      Some("Today is 20:19:54".to_string())
    );
  }

  #[test]
  fn default_format_uses_locale_data_beyond_us_english() {
    assert_eq!(
      format_date_time_field(&tokens(&["DATE"]), Some("en-US"), VALUE),
      Some("7/12/2026".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["DATE"]), Some("en-GB"), VALUE),
      Some("12/07/2026".to_string())
    );
    assert!(
      format_date_time_field(&tokens(&["DATE", r"\@", "MMMM d"]), Some("zh-CN"), VALUE).is_some()
    );
  }

  #[test]
  fn document_property_dates_use_source_backed_locale_defaults_and_minute_precision() {
    assert_eq!(
      format_date_time_field(&tokens(&["SAVEDATE"]), Some("en-US"), VALUE),
      Some("7/12/2026 8:19:00 PM".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["PRINTDATE"]), Some("en-GB"), VALUE),
      Some("12/07/2026 20:19:00".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["PRINTDATE"]), Some("en-IN"), VALUE),
      Some("12-07-2026 20:19:00".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["SAVEDATE"]), Some("fr-FR"), VALUE),
      Some("12/07/2026 20:19:00".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["CREATEDATE"]), Some("en-US"), VALUE),
      Some("7/12/2026 8:19:54 PM".to_string())
    );
  }

  #[test]
  fn spreadsheet_explicit_ampm_keeps_its_labels_across_format_locales() {
    for language in ["en-US", "zh-CN", "fr-CA", "ja-JP"] {
      for (hour, expected) in [
        (0, "12:19 AM"),
        (8, "8:19 AM"),
        (12, "12:19 PM"),
        (20, "8:19 PM"),
      ] {
        let value = FieldUpdateDateTime { hour, ..VALUE };
        for picture in ["h:mm AM/PM", "h:mm am/pm"] {
          assert_eq!(
            super::format_spreadsheet_date_picture(picture, Some(language), value).as_deref(),
            Some(expected),
            "{language}: {picture}"
          );
        }
      }
    }
    // Word field pictures continue to use localized day-period resources.
    assert_eq!(
      super::format_date_time_picture("h:mm am/pm", Some("zh-CN"), VALUE).as_deref(),
      Some("8:19 下午")
    );
  }

  #[test]
  fn date_pictures_preserve_repeated_literal_characters() {
    assert_eq!(
      super::format_date_time_picture("yyyy//MM//dd  HH::mm::ss.000", Some("en-US"), VALUE)
        .as_deref(),
      Some("2026//07//12  20::19::54.000")
    );
    for picture in [r"hh:mm:ss\.000", r#"hh:mm:ss".000""#] {
      assert_eq!(
        super::format_spreadsheet_date_picture(picture, Some("en-US"), VALUE).as_deref(),
        Some("20:19:54.000"),
        "{picture}"
      );
    }
    assert_eq!(
      super::format_spreadsheet_date_picture(r#"yyyy//mm//dd "hh" \m\m"#, Some("en-US"), VALUE)
        .as_deref(),
      Some("2026//07//12 hh mm")
    );
  }

  #[test]
  fn adjacent_spreadsheet_escapes_remain_one_literal() {
    assert_eq!(
      super::format_spreadsheet_date_picture(r"yyyy/\ m/\ d\.\ h:mm", Some("zh-CN"), VALUE),
      Some("2026/ 7/ 12. 20:19".to_string())
    );
  }

  #[test]
  fn japanese_spreadsheet_eras_follow_calendar_boundaries_and_format_tags() {
    for (year, month, day, expected) in [
      (2024, 5, 28, "令和6年5月28日"),
      (2019, 4, 30, "平成31年4月30日"),
      (2019, 5, 1, "令和元年5月1日"),
      (2020, 1, 1, "令和2年1月1日"),
      (1989, 1, 7, "昭和64年1月7日"),
      (1989, 1, 8, "平成元年1月8日"),
    ] {
      let value = FieldUpdateDateTime {
        year,
        month,
        day,
        ..VALUE
      };
      assert_eq!(
        super::format_spreadsheet_date_picture(
          r#"[$-ja-JP-x-gannen]ggge"年"m"月"d"日";@"#,
          Some("zh-CN"),
          value,
        )
        .as_deref(),
        Some(expected),
      );
    }
    let value = FieldUpdateDateTime {
      year: 2019,
      month: 5,
      day: 1,
      ..VALUE
    };
    for (picture, expected) in [
      (r#"[$-411]ggge"年"m"月"d"日""#, "令和1年5月1日"),
      ("[$-ja-JP]ge/mm/dd", "R1/05/01"),
      ("[$-ja-JP]ggee/mm/dd", "令01/05/01"),
      (r#"[$-ja-JP]"ggge" yyyy/mm/dd"#, "ggge 2019/05/01"),
      ("[$-zh-CN]e/mm/dd", "2019/05/01"),
      ("[$-en-US]ee/mm/dd", "2019/05/01"),
      (r#"[$-ja-JP]ggge"年"m"月"d"日""#, "令和1年5月1日"),
    ] {
      assert_eq!(
        super::format_spreadsheet_date_picture(picture, Some("en-US"), value).as_deref(),
        Some(expected),
      );
    }
  }

  #[test]
  fn non_japanese_era_years_follow_office_full_year_width() {
    for year in [1999, 2000, 2026] {
      let value = FieldUpdateDateTime { year, ..VALUE };
      for language in ["en-US", "zh-CN", "fr-FR"] {
        for field in ["e", "ee"] {
          assert_eq!(
            super::format_spreadsheet_date_picture(
              &format!("{field}/mm/dd"),
              Some(language),
              value
            ),
            Some(format!("{year}/07/12")),
          );
        }
      }
      assert_eq!(
        super::format_spreadsheet_date_picture("yy/mm/dd", Some("en-US"), value),
        Some(format!("{:02}/07/12", year % 100)),
      );
    }
  }

  #[test]
  fn fictitious_spreadsheet_dates_remain_invalid_document_field_dates() {
    for (month, day) in [(1, 0), (2, 29)] {
      let value = FieldUpdateDateTime {
        year: 1900,
        month,
        day,
        ..VALUE
      };
      assert_eq!(
        super::format_date_time_picture("yyyy-MM-dd", Some("en-US"), value),
        None
      );
      assert_eq!(
        super::format_office_long_date(Some("zh-CN"), value, true),
        None
      );
    }
  }

  #[test]
  fn spreadsheet_system_long_date_preserves_explicit_weekday_fields() {
    for (year, month, day, expected) in [
      (1904, 3, 1, "1904年3月1日"),
      (2005, 9, 12, "2005年9月12日"),
      (1976, 8, 26, "1976年8月26日"),
      (2019, 1, 21, "2019年1月21日"),
    ] {
      let value = FieldUpdateDateTime {
        year,
        month,
        day,
        ..VALUE
      };
      assert_eq!(
        super::format_spreadsheet_system_long_date(Some("zh-CN"), value, None).as_deref(),
        Some(expected)
      );
      let explicit_weekday = super::format_office_long_date(Some("zh-CN"), value, true).unwrap();
      assert!(explicit_weekday.contains(expected));
      assert_ne!(explicit_weekday, expected);
    }
  }

  #[test]
  fn australian_date_lcid_overrides_the_fallback_format_language() {
    let value = FieldUpdateDateTime {
      year: 2019,
      month: 1,
      day: 21,
      ..VALUE
    };
    let picture = r"dddd\,\ d\ mmmm\ yyyy";
    for fallback in ["zh-CN", "ja-JP", "fr-FR"] {
      for culture in ["C09", "0c09", "000C09", "en-AU"] {
        assert_eq!(
          super::format_spreadsheet_date_picture(
            &format!("[$-{culture}]{picture};@"),
            Some(fallback),
            value,
          )
          .as_deref(),
          Some("Monday, 21 January 2019"),
          "{culture}: {fallback}",
        );
      }
    }
    // Unrecognized LCIDs still follow the requested format language.
    let french = super::format_spreadsheet_date_picture(picture, Some("fr-FR"), value);
    assert!(french.is_some());
    assert_ne!(french.as_deref(), Some("Monday, 21 January 2019"));
    assert_eq!(
      super::format_spreadsheet_date_picture(&format!("[$-7FFF]{picture}"), Some("fr-FR"), value,),
      french,
    );
  }

  #[test]
  fn serbian_bosnia_date_lcid_keeps_office_month_names_and_literal_boundaries() {
    // Excel 20326 exports: full year, abbreviations, initials, quoted
    // month-looking literals and minutes, with Chinese and English UI.
    let months = [
      "јануар",
      "фебруар",
      "март",
      "април",
      "мај",
      "јуни",
      "јули",
      "август",
      "септембар",
      "октобар",
      "новембар",
      "децембар",
    ];
    let abbreviated = [
      "јан", "феб", "мар", "апр", "мај", "јун", "јул", "авг", "сеп", "окт", "нов", "дец",
    ];
    for fallback in ["zh-CN", "en-US"] {
      for culture in ["1C1A", "1c1a", "001C1A", "sr-Cyrl-BA"] {
        for (index, month_name) in months.into_iter().enumerate() {
          let value = FieldUpdateDateTime {
            year: 1972,
            month: index as u8 + 1,
            day: 17,
            hour: 14,
            minute: 23,
            ..VALUE
          };
          for (picture, expected) in [
            ("mmmm", month_name.to_owned()),
            ("mmm", abbreviated[index].to_owned()),
            ("mmmmm", month_name.chars().next().unwrap().to_string()),
            (
              r#""јун" mmmm "јул" mm "mmmm" hh:mm"#,
              format!("јун {month_name} јул {:02} mmmm 14:23", index + 1),
            ),
          ] {
            assert_eq!(
              super::format_spreadsheet_date_picture(
                &format!("[$-{culture}]{picture};@"),
                Some(fallback),
                value,
              ),
              Some(expected),
              "{culture}, {fallback}, month={}, {picture}",
              value.month,
            );
          }
        }
      }
    }
    for (index, weekday) in [
      "понедјељак",
      "уторак",
      "сриједа",
      "четвртак",
      "петак",
      "субота",
      "недјеља",
    ]
    .into_iter()
    .enumerate()
    {
      let value = FieldUpdateDateTime {
        year: 1972,
        month: 6,
        day: index as u8 + 12,
        ..VALUE
      };
      assert_eq!(
        super::format_spreadsheet_date_picture(
          r"[$-1C1A]dddd\,\ d\.\ mmmm\ yyyy",
          Some("zh-CN"),
          value,
        ),
        Some(format!("{weekday}, {}. јуни 1972", value.day)),
      );
    }
    // Neighboring locales keep their observed month spelling and script.
    for (culture, expected) in [
      ("sr-Cyrl-RS", "субота, 17. јун 1972"),
      ("sr-Latn-BA", "subota, 17. jun 1972"),
    ] {
      assert_eq!(
        super::format_spreadsheet_date_picture(
          &format!(r"[$-{culture}]dddd\,\ d\.\ mmmm\ yyyy"),
          Some("zh-CN"),
          FieldUpdateDateTime {
            year: 1972,
            month: 6,
            day: 17,
            ..VALUE
          },
        )
        .as_deref(),
        Some(expected),
        "{culture}",
      );
    }
  }

  #[test]
  fn spreadsheet_culture_markers_ignore_quoted_escaped_and_later_section_text() {
    let picture = r#""[$-ja-JP]"\[\$\-ja\-JP\] [$-fr-FR]d-mmm;[$-ja-JP]ggge"#;
    assert_eq!(
      super::spreadsheet_date_picture_language(picture).as_deref(),
      Some("fr-FR")
    );
    assert_eq!(
      super::spreadsheet_date_picture_language("[$-0409]d-mmm").as_deref(),
      Some("en-US")
    );
  }
}
