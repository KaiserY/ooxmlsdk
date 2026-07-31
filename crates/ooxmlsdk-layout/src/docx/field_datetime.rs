use crate::options::FieldUpdateDateTime;

const ENGLISH_MONTHS_SHORT: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const ENGLISH_MONTHS_LONG: [&str; 12] = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];
const ENGLISH_WEEKDAYS_SHORT: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const ENGLISH_WEEKDAYS_LONG: [&str; 7] = [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];

pub(super) fn format_date_time_field(
  tokens: &[String],
  language: Option<&str>,
  mut value: FieldUpdateDateTime,
) -> Option<String> {
  let field_name = tokens.first()?;
  let default_picture = if field_name.eq_ignore_ascii_case("DATE") {
    default_date_picture
  } else if field_name.eq_ignore_ascii_case("TIME") {
    default_time_picture
  } else if field_name.eq_ignore_ascii_case("PRINTDATE")
    || field_name.eq_ignore_ascii_case("SAVEDATE")
  {
    // PRINTDATE and SAVEDATE expose document-property times. The Office
    // fixed-output fixtures consistently persist those properties at minute
    // precision, unlike current DATE/TIME fields, so do not invent seconds
    // from the later conversion-manifest timestamp.
    value.second = 0;
    default_document_date_time_picture
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
  let picture = picture.or_else(|| default_picture(language))?;
  format_picture(picture, language, value)
}

fn default_date_picture(language: Option<&str>) -> Option<&'static str> {
  language
    .map_or(true, |language| {
      language.eq_ignore_ascii_case("en-US") || language.eq_ignore_ascii_case("en")
    })
    .then_some("M/d/yyyy")
}

fn default_time_picture(language: Option<&str>) -> Option<&'static str> {
  language
    .map_or(true, |language| {
      language.eq_ignore_ascii_case("en-US") || language.eq_ignore_ascii_case("en")
    })
    .then_some("h:mm:ss am/pm")
}

fn default_document_date_time_picture(language: Option<&str>) -> Option<&'static str> {
  match language?.to_ascii_lowercase().as_str() {
    // Word's no-picture PRINTDATE/SAVEDATE result follows w:lang. Keep this
    // deliberately bounded to the locales demonstrated by the upstream
    // field fixtures and Office golden output.
    "en-us" => Some("M/d/yyyy h:mm:ss am/pm"),
    "en-gb" => Some("dd/MM/yyyy HH:mm:ss"),
    "en-in" => Some("dd-MM-yyyy HH:mm:ss"),
    _ => None,
  }
}

fn format_picture(
  picture: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  let chars = picture.chars().collect::<Vec<_>>();
  let mut output = String::new();
  let mut index = 0;
  while index < chars.len() {
    if chars[index] == '\'' {
      index += 1;
      let literal_start = index;
      while index < chars.len() && chars[index] != '\'' {
        index += 1;
      }
      if index == chars.len() {
        return None;
      }
      output.extend(chars[literal_start..index].iter());
      index += 1;
      continue;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "am/pm") {
      output.push_str(if value.hour < 12 { "AM" } else { "PM" });
      index += "am/pm".len();
      continue;
    }
    if ascii_prefix_eq_ignore_case(&chars[index..], "a/p") {
      output.push(if value.hour < 12 { 'A' } else { 'P' });
      index += "a/p".len();
      continue;
    }

    let ch = chars[index];
    let count = chars[index..]
      .iter()
      .take_while(|candidate| **candidate == ch)
      .count();
    let formatted = match ch {
      'M' => format_month(value.month, count, language),
      'd' | 'D' => format_day(value, count, language),
      'y' | 'Y' => format_year(value.year, count),
      'h' => format_hour_12(value.hour, count),
      'H' => format_number(u16::from(value.hour), count, 2),
      'm' => format_number(u16::from(value.minute), count, 2),
      's' => format_number(u16::from(value.second), count, 2),
      _ if ch.is_ascii_alphabetic() => return None,
      _ => {
        output.push(ch);
        index += 1;
        continue;
      }
    }?;
    output.push_str(&formatted);
    index += count;
  }
  Some(output)
}

fn format_month(month: u8, count: usize, language: Option<&str>) -> Option<String> {
  match count {
    1 => Some(month.to_string()),
    2 => Some(format!("{month:02}")),
    3 if english_language(language) => {
      Some(ENGLISH_MONTHS_SHORT[usize::from(month - 1)].to_string())
    }
    4 if english_language(language) => {
      Some(ENGLISH_MONTHS_LONG[usize::from(month - 1)].to_string())
    }
    _ => None,
  }
}

fn format_day(value: FieldUpdateDateTime, count: usize, language: Option<&str>) -> Option<String> {
  match count {
    1 => Some(value.day.to_string()),
    2 => Some(format!("{:02}", value.day)),
    3 if english_language(language) => Some(ENGLISH_WEEKDAYS_SHORT[weekday(value)].to_string()),
    4 if english_language(language) => Some(ENGLISH_WEEKDAYS_LONG[weekday(value)].to_string()),
    _ => None,
  }
}

fn format_year(year: u16, count: usize) -> Option<String> {
  match count {
    1 | 2 => Some(format!("{:02}", year % 100)),
    4 => Some(format!("{year:04}")),
    _ => None,
  }
}

fn format_hour_12(hour: u8, count: usize) -> Option<String> {
  let hour = match hour % 12 {
    0 => 12,
    hour => hour,
  };
  format_number(u16::from(hour), count, 2)
}

fn format_number(value: u16, count: usize, padded_count: usize) -> Option<String> {
  match count {
    1 => Some(value.to_string()),
    count if count == padded_count => Some(format!("{value:0count$}")),
    _ => None,
  }
}

fn english_language(language: Option<&str>) -> bool {
  language.map_or(true, |language| {
    language.eq_ignore_ascii_case("en") || language.to_ascii_lowercase().starts_with("en-")
  })
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

fn weekday(value: FieldUpdateDateTime) -> usize {
  let offsets = [0_i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
  let mut year = i32::from(value.year);
  if value.month < 3 {
    year -= 1;
  }
  usize::try_from(
    (year + year / 4 - year / 100
      + year / 400
      + offsets[usize::from(value.month - 1)]
      + i32::from(value.day))
    .rem_euclid(7),
  )
  .expect("weekday is between zero and six")
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
  fn default_format_is_bounded_to_the_source_backed_us_english_context() {
    assert_eq!(
      format_date_time_field(&tokens(&["DATE"]), Some("en-US"), VALUE),
      Some("7/12/2026".to_string())
    );
    assert_eq!(
      format_date_time_field(&tokens(&["DATE"]), Some("en-GB"), VALUE),
      None
    );
    assert_eq!(
      format_date_time_field(&tokens(&["DATE", r"\@", "MMMM d"]), Some("zh-CN"), VALUE),
      None
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
      None
    );
    assert_eq!(
      format_date_time_field(&tokens(&["CREATEDATE"]), Some("en-US"), VALUE),
      None
    );
  }
}
