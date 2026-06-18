use crate::DateSystem;

pub fn weekday_index_from_serial(serial: i64) -> usize {
  let days_since_unix = if serial < 60 {
    serial - 25_568
  } else {
    serial - 25_569
  };
  (days_since_unix + 3).rem_euclid(7) as usize
}

pub fn date_diff_months(start: i32, end: i32, mode: i32) -> Option<i32> {
  let (start_year, start_month, start_day) = date_from_serial(start)?;
  let (end_year, end_month, end_day) = date_from_serial(end)?;
  let mut result = end_month as i32 - start_month as i32 + (end_year - start_year) * 12;
  if mode == 1 || start == end {
    return Some(result);
  }
  if start < end {
    if start_day > end_day {
      result -= 1;
    }
  } else if start_day < end_day {
    result += 1;
  }
  Some(result)
}

pub fn date_diff_years(start: i32, end: i32, mode: i32) -> Option<i32> {
  if mode != 1 {
    return date_diff_months(start, end, mode).map(|months| months / 12);
  }
  let (start_year, _, _) = date_from_serial(start)?;
  let (end_year, _, _) = date_from_serial(end)?;
  Some(end_year - start_year)
}

pub fn date_serial(year: i32, month: i32, day: i32) -> Option<f64> {
  date_serial_with_system(year, month, day, DateSystem::Date1900)
}

pub fn date_serial_with_system(
  year: i32,
  month: i32,
  day: i32,
  date_system: DateSystem,
) -> Option<f64> {
  let (normalized_year, normalized_month, normalized_day) =
    normalized_date_components(year, month, day)?;
  let days = days_from_civil(
    normalized_year,
    normalized_month as i32,
    normalized_day as i32,
  )?;
  date_serial_from_days(days, date_system)
}

pub fn normalized_date_components(year: i32, month: i32, day: i32) -> Option<(i32, u32, u32)> {
  let month_index = month - 1;
  let normalized_year = year + month_index.div_euclid(12);
  let normalized_month = month_index.rem_euclid(12) + 1;
  let days = days_from_civil(normalized_year, normalized_month, 1)? + i64::from(day - 1);
  civil_from_days(days)
}

fn date_serial_from_days(days: i64, date_system: DateSystem) -> Option<f64> {
  let serial = match date_system {
    DateSystem::Date1900 => {
      let base = days_from_civil(1899, 12, 31)?;
      let mut serial = days - base;
      let leap_bug_start = days_from_civil(1900, 3, 1)?;
      if days >= leap_bug_start {
        serial += 1;
      }
      serial
    }
    DateSystem::Date1904 => days - days_from_civil(1904, 1, 1)?,
    DateSystem::LibreOffice => days - days_from_civil(1899, 12, 30)?,
  };
  Some(serial as f64)
}

pub fn is_valid_libreoffice_gregorian_date(year: i32, month: u32, day: u32) -> bool {
  (year, month, day) >= (1582, 10, 15)
}

pub fn date_from_serial(serial: i32) -> Option<(i32, u32, u32)> {
  date_from_serial_with_system(serial, DateSystem::Date1900)
}

pub fn date_from_serial_with_system(
  serial: i32,
  date_system: DateSystem,
) -> Option<(i32, u32, u32)> {
  match date_system {
    DateSystem::Date1900 => {
      if serial == 60 {
        return Some((1900, 2, 29));
      }
      let base = days_from_civil(1899, 12, 31)?;
      let adjusted = if serial > 60 { serial - 1 } else { serial };
      civil_from_days(base + i64::from(adjusted))
    }
    DateSystem::Date1904 => civil_from_days(days_from_civil(1904, 1, 1)? + i64::from(serial)),
    DateSystem::LibreOffice => civil_from_days(days_from_civil(1899, 12, 30)? + i64::from(serial)),
  }
}

pub fn basis_o_datetime_text(serial: f64) -> Option<String> {
  if !serial.is_finite() {
    return None;
  }
  let day = serial.floor();
  let fraction = serial - day;
  let seconds = (fraction * 86_400.0).round() as i64;
  let day_adjust = seconds.div_euclid(86_400);
  let second_of_day = seconds.rem_euclid(86_400);
  let base = days_from_civil(1899, 12, 30)?;
  let (year, month, date) = civil_from_days(base + day as i64 + day_adjust)?;
  Some(format!(
    "{year:04}-{month:02}-{date:02} {:02}:{:02}:{:02}",
    second_of_day / 3600,
    (second_of_day % 3600) / 60,
    second_of_day % 60
  ))
}

pub fn weeks_mode_one_index(serial: i32, date_system: DateSystem) -> Option<i64> {
  let null_date_days = match date_system {
    DateSystem::Date1900 => date_to_days(1899, 12, 31)?,
    DateSystem::LibreOffice => date_to_days(1899, 12, 30)?,
    DateSystem::Date1904 => date_to_days(1904, 1, 1)?,
  };
  Some((i64::from(serial) + null_date_days - 1).div_euclid(7))
}

pub fn date_to_days(year: i32, month: i32, day: i32) -> Option<i64> {
  Some(days_from_civil(year, month, day)? - days_from_civil(1, 1, 1)? + 1)
}

pub fn weeks_in_year_from_serial_with_system(serial: i32, date_system: DateSystem) -> Option<i32> {
  let (year, _, _) = date_from_serial_with_system(serial, date_system)?;
  iso_weeks_in_year(year)
}

pub fn iso_weeknum_from_serial_with_system(serial: i32, date_system: DateSystem) -> Option<i32> {
  let (year, month, day) = date_from_serial_with_system(serial, date_system)?;
  let days = days_from_civil(year, month as i32, day as i32)?;
  let year_start = days_from_civil(year, 1, 1)?;
  let day_of_year = (days - year_start + 1) as i32;
  let weekday = ((days - days_from_civil(1, 1, 1)?).rem_euclid(7) + 1) as i32;
  let week = (day_of_year - weekday + 10).div_euclid(7);
  if week < 1 {
    iso_weeks_in_year(year - 1)
  } else {
    let weeks = iso_weeks_in_year(year)?;
    Some(if week > weeks { 1 } else { week })
  }
}

fn iso_weeks_in_year(year: i32) -> Option<i32> {
  let jan1_days = days_from_civil(year, 1, 1)? - days_from_civil(1, 1, 1)?;
  let jan1_weekday = jan1_days.rem_euclid(7);
  if jan1_weekday == 3 || (jan1_weekday == 2 && is_leap_year(year)) {
    Some(53)
  } else {
    Some(52)
  }
}

pub fn days360(start: i32, end: i32, european: bool) -> Option<i32> {
  days360_with_system(start, end, european, DateSystem::Date1900)
}

pub fn days360_with_system(
  mut start: i32,
  mut end: i32,
  european: bool,
  date_system: DateSystem,
) -> Option<i32> {
  let sign = if european && end < start {
    std::mem::swap(&mut start, &mut end);
    -1
  } else {
    1
  };
  let (year1, month1, day1) = date_from_serial_with_system(start, date_system)?;
  let (year2, month2, day2) = date_from_serial_with_system(end, date_system)?;
  let mut day1 = day1 as i32;
  let mut day2 = day2 as i32;
  if day1 == 31 {
    day1 = 30;
  } else if !european && month1 == 2 {
    let last = last_day_of_month(year1, month1) as i32;
    if day1 == last {
      day1 = 30;
    }
  }
  if day2 == 31 && (european || day1 == 30) {
    day2 = 30;
  }
  Some(sign * ((year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1)))
}

pub fn yearfrac(start: i32, end: i32, basis: i32) -> Option<f64> {
  let (start, end) = if start <= end {
    (start, end)
  } else {
    (end, start)
  };
  if start == end {
    return Some(0.0);
  }
  let (year1, month1, day1) = date_from_serial(start)?;
  let (year2, month2, day2) = date_from_serial(end)?;
  let day_diff = match basis {
    0 => {
      let mut day1 = day1 as i32;
      let mut day2 = day2 as i32;
      if day1 == 31 {
        day1 -= 1;
      }
      if day1 == 30 && day2 == 31 {
        day2 -= 1;
      } else if month1 == 2 && day1 == last_day_of_month(year1, month1) as i32 {
        day1 = 30;
        if month2 == 2 && day2 == last_day_of_month(year2, month2) as i32 {
          day2 = 30;
        }
      }
      (year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1)
    }
    1..=3 => end - start,
    4 => {
      let day1 = if day1 == 31 { 30 } else { day1 } as i32;
      let day2 = if day2 == 31 { 30 } else { day2 } as i32;
      (year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1)
    }
    _ => return None,
  };
  let days_in_year = match basis {
    0 | 2 | 4 => 360.0,
    3 => 365.0,
    1 => {
      if year1 != year2
        && (year2 != year1 + 1 || month1 < month2 || (month1 == month2 && day1 < day2))
      {
        let day_count = (year1..=year2)
          .map(|year| if is_leap_year(year) { 366 } else { 365 })
          .sum::<i32>();
        day_count as f64 / (year2 - year1 + 1) as f64
      } else if (year1 == year2 && is_leap_year(year1))
        || (year1 != year2
          && ((is_leap_year(year1) && (month1 < 2 || (month1 == 2 && day1 <= 29)))
            || (is_leap_year(year2) && (month2 > 2 || (month2 == 2 && day2 == 29)))))
      {
        366.0
      } else {
        365.0
      }
    }
    _ => return None,
  };
  Some(day_diff as f64 / days_in_year)
}

pub fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn last_day_of_month(year: i32, month: u32) -> u32 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if is_leap_year(year) => 29,
    2 => 28,
    _ => 31,
  }
}

pub fn days_from_civil(year: i32, month: i32, day: i32) -> Option<i64> {
  if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
    return None;
  }
  let year = year - i32::from(month <= 2);
  let era = if year >= 0 { year } else { year - 399 }.div_euclid(400);
  let yoe = year - era * 400;
  let month = month as i64;
  let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
  let doe = i64::from(yoe) * 365 + i64::from(yoe) / 4 - i64::from(yoe) / 100 + doy;
  Some(i64::from(era) * 146_097 + doe - 719_468)
}

pub fn civil_from_days(days: i64) -> Option<(i32, u32, u32)> {
  let days = days + 719_468;
  let era = if days >= 0 { days } else { days - 146_096 }.div_euclid(146_097);
  let doe = days - era * 146_097;
  let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096).div_euclid(365);
  let year = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2).div_euclid(153);
  let day = doy - (153 * mp + 2).div_euclid(5) + 1;
  let month = mp + if mp < 10 { 3 } else { -9 };
  let year = year + i64::from(month <= 2);
  Some((year.try_into().ok()?, month as u32, day as u32))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn date1900_preserves_excel_leap_year_serial_boundary() {
    assert_eq!(date_from_serial(59), Some((1900, 2, 28)));
    assert_eq!(date_from_serial(60), Some((1900, 2, 29)));
    assert_eq!(date_from_serial(61), Some((1900, 3, 1)));
    assert_eq!(date_serial(1900, 2, 28), Some(59.0));
    assert_eq!(date_serial(1900, 3, 1), Some(61.0));
  }

  #[test]
  fn date_systems_use_their_expected_null_dates() {
    assert_eq!(
      date_from_serial_with_system(0, DateSystem::Date1904),
      Some((1904, 1, 1))
    );
    assert_eq!(
      date_from_serial_with_system(0, DateSystem::LibreOffice),
      Some((1899, 12, 30))
    );
    assert_eq!(
      date_serial_with_system(1904, 1, 1, DateSystem::Date1904),
      Some(0.0)
    );
    assert_eq!(
      date_serial_with_system(1899, 12, 30, DateSystem::LibreOffice),
      Some(0.0)
    );
  }

  #[test]
  fn normalizes_month_and_day_overflow_like_spreadsheet_dates() {
    assert_eq!(normalized_date_components(2024, 13, 1), Some((2025, 1, 1)));
    assert_eq!(normalized_date_components(2024, 3, 0), Some((2024, 2, 29)));
    assert_eq!(normalized_date_components(2024, 0, 1), Some((2023, 12, 1)));
  }
}
