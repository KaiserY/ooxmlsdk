use crate::localization::canonical_locale;
use crate::options::FieldUpdateDateTime;
use icu_calendar::Gregorian;
use icu_datetime::fieldsets::{T, YMD, YMDE};
use icu_datetime::input::{Date, DateTime, Time};
use icu_datetime::options::YearStyle;
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

fn format_office_default_time(
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
  let pattern = DateTimePattern::try_from_pattern_str(&pattern).ok()?;
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
      _ => output.push(ch),
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
  let (picture, embedded_language) = spreadsheet_date_picture_to_field_picture(picture)?;
  format_picture(
    &picture,
    embedded_language.as_deref().or(fallback_language),
    value,
  )
}

fn spreadsheet_date_picture_to_field_picture(picture: &str) -> Option<(String, Option<String>)> {
  let chars = picture.chars().collect::<Vec<_>>();
  let mut output = String::new();
  let mut embedded_language = None;
  let mut index = 0usize;
  let mut saw_date_token = false;
  while index < chars.len() {
    let ch = chars[index];
    if ch == ';' {
      break;
    }
    match ch {
      '[' => {
        let end = chars[index + 1..]
          .iter()
          .position(|candidate| *candidate == ']')
          .map(|offset| index + 1 + offset)?;
        let marker = chars[index + 1..end].iter().collect::<String>();
        if let Some(lcid) = marker.strip_prefix("$-") {
          embedded_language = spreadsheet_lcid_language(lcid).map(ToOwned::to_owned);
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
        index += count;
      }
      'm' | 'M' => {
        let count = chars[index..]
          .iter()
          .take_while(|candidate| candidate.eq_ignore_ascii_case(&ch))
          .count()
          .min(5);
        output.extend(std::iter::repeat_n('M', count));
        saw_date_token = true;
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
  saw_date_token.then_some((output, embedded_language))
}

fn push_icu_quoted_literal(output: &mut String, literal: &str) {
  output.push('\'');
  for ch in literal.chars() {
    output.push(ch);
    if ch == '\'' {
      output.push('\'');
    }
  }
  output.push('\'');
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
    0x0c0a => Some("es-ES"),
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
  Date::try_new_gregorian(
    i32::from(value.year),
    u8::from(value.month),
    u8::from(value.day),
  )
  .ok()
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
  use super::format_date_time_field;
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
      None
    );
  }
}
