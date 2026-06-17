use super::*;

pub(crate) fn display_text_from_value(value: &FormulaValue<'_>) -> String {
  match value {
    FormulaValue::Number(value) if value.is_finite() && value.fract() == 0.0 => value.to_string(),
    FormulaValue::Number(value) if value.is_finite() => value.to_string(),
    FormulaValue::Number(_) => error_text_value(FormulaErrorValue::Value).to_string(),
    FormulaValue::String(value) => value.to_string(),
    FormulaValue::Boolean(value) => {
      if *value {
        "TRUE".to_string()
      } else {
        "FALSE".to_string()
      }
    }
    FormulaValue::Error(value) => error_text_value(*value).to_string(),
    FormulaValue::Blank => String::new(),
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {
      String::new()
    }
  }
}

pub(crate) fn display_text_from_value_with_number_format(
  value: &FormulaValue<'_>,
  context: Option<&NumberFormatContext<'_>>,
) -> Option<String> {
  let FormulaValue::Number(number) = value else {
    return None;
  };
  let format = context?.format_code.as_deref()?;
  let format = select_number_format_section(format, *number);
  format_simple_number_pattern(*number, &format)
}

pub(crate) fn format_text(
  value: &FormulaValue<'_>,
  format: &FormulaValue<'_>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> String {
  let format = evaluator.text(format);
  let Some(number) = evaluator.number(value) else {
    if format.is_empty() {
      return evaluator.text(value);
    }
    return evaluator.text(value);
  };
  if format.is_empty() {
    return String::new();
  }
  let format = select_number_format_section(&format, number);
  if let Some(text) = format_date_pattern(number, &format, evaluator.book.date_system) {
    return text;
  }
  if let Some(text) = format_simple_number_pattern(number, &format) {
    return text;
  }
  evaluator.text(value)
}

fn format_date_pattern(number: f64, format: &str, date_system: DateSystem) -> Option<String> {
  if !format_contains_date_time_token(format) {
    return None;
  }
  let day = number.floor();
  let fraction = number - day;
  let seconds = (fraction * 86_400.0).round() as i64;
  let day_adjust = seconds.div_euclid(86_400);
  let second_of_day = seconds.rem_euclid(86_400);
  let serial = day as i32 + day_adjust as i32;
  let (year, month, day) = date_from_serial_with_system(serial, date_system)?;
  let hour = second_of_day / 3600;
  let minute = (second_of_day % 3600) / 60;
  let second = second_of_day % 60;
  let mut result = String::new();
  let mut chars = format.chars().peekable();
  let mut after_hour = false;
  while let Some(ch) = chars.next() {
    if ch == '"' {
      for literal in chars.by_ref() {
        if literal == '"' {
          break;
        }
        result.push(literal);
      }
      continue;
    }
    if !ch.is_ascii_alphabetic() {
      result.push(ch);
      continue;
    }
    let lower = ch.to_ascii_lowercase();
    let mut len = 1usize;
    while chars
      .peek()
      .is_some_and(|next| next.to_ascii_lowercase() == lower)
    {
      chars.next();
      len += 1;
    }
    match lower {
      'y' => {
        if len <= 2 {
          result.push_str(&format!("{:02}", year.rem_euclid(100)));
        } else {
          result.push_str(&format!("{year:04}"));
        }
      }
      'd' => {
        if len == 1 {
          result.push_str(&day.to_string());
        } else {
          result.push_str(&format!("{day:02}"));
        }
      }
      'h' => {
        after_hour = true;
        if len == 1 {
          result.push_str(&hour.to_string());
        } else {
          result.push_str(&format!("{hour:02}"));
        }
      }
      'm' => {
        let value = if after_hour { minute as u32 } else { month };
        if len == 1 {
          result.push_str(&value.to_string());
        } else {
          result.push_str(&format!("{value:02}"));
        }
      }
      's' => {
        if len == 1 {
          result.push_str(&second.to_string());
        } else {
          result.push_str(&format!("{second:02}"));
        }
      }
      _ => {
        for _ in 0..len {
          result.push(ch);
        }
      }
    }
  }
  Some(result)
}

fn format_simple_number_pattern(number: f64, format: &str) -> Option<String> {
  let numeric = strip_number_format_directives(format.trim());
  if numeric.eq_ignore_ascii_case("General") {
    return None;
  }
  if numeric.starts_with('"') && numeric.ends_with('"') {
    return Some(numeric.trim_matches('"').to_string());
  }
  if numeric.contains('?') && numeric.contains('/') {
    return format_fraction_pattern(number, &numeric);
  }
  if numeric.to_ascii_uppercase().contains('E') {
    return format_scientific_pattern(number, &numeric);
  }
  if numeric.contains('%') {
    let body = numeric.replace('%', "");
    return format_simple_number_pattern(number * 100.0, &body).map(|text| format!("{text}%"));
  }
  if can_format_as_digit_mask(&numeric) && !numeric.contains('.') {
    return format_digit_mask(number, &numeric);
  }
  let grouping = numeric.contains(',');
  let prefix: String = numeric
    .chars()
    .take_while(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .collect();
  let suffix: String = numeric
    .chars()
    .rev()
    .take_while(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .collect::<String>()
    .chars()
    .rev()
    .collect();
  let digit_part = numeric
    .trim_start_matches(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .trim_end_matches(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','));
  let mut seen_digit = false;
  let mut min_integer_digits = 0usize;
  let mut decimal_places = 0usize;
  let mut after_decimal = false;
  for ch in digit_part.chars() {
    match ch {
      '#' | '0' | '?' => {
        seen_digit = true;
        if after_decimal {
          decimal_places += 1;
        } else if ch == '0' {
          min_integer_digits += 1;
        }
      }
      '.' => {
        if after_decimal {
          return None;
        }
        after_decimal = true;
      }
      ',' | ' ' => {}
      _ => return None,
    }
  }
  if !seen_digit {
    return None;
  }
  if decimal_places == 0 {
    let rounded = round_half_away_from_zero(number, 0) as i64;
    let mut text = format_integer_with_min_digits(rounded, min_integer_digits);
    if grouping {
      text = add_grouping_separators(&text);
    }
    return Some(format!("{prefix}{text}{suffix}"));
  }
  let rounded = round_half_away_from_zero(number, decimal_places as i32);
  let mut text = format!("{rounded:.decimal_places$}");
  if min_integer_digits > 0 {
    text = pad_integer_part(text, min_integer_digits);
  }
  if digit_part
    .rsplit_once('.')
    .is_some_and(|(_, fraction)| fraction.chars().all(|ch| ch == '#'))
  {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  if grouping {
    text = add_grouping_to_decimal(&text);
  }
  Some(format!("{prefix}{text}{suffix}"))
}

fn select_number_format_section(format: &str, number: f64) -> String {
  let sections = split_number_format_sections(format);
  for section in &sections {
    if let Some((op, threshold, body)) = parse_format_condition(section)
      && compare_format_condition(number, op, threshold)
    {
      return body.trim().to_string();
    }
  }
  let non_conditional: Vec<&str> = sections
    .iter()
    .map(String::as_str)
    .filter(|section| parse_format_condition(section).is_none())
    .collect();
  if non_conditional.is_empty() {
    return format.to_string();
  }
  if number < 0.0 && non_conditional.len() >= 2 {
    return non_conditional[1].trim().to_string();
  }
  if number == 0.0 && non_conditional.len() >= 3 {
    return non_conditional[2].trim().to_string();
  }
  non_conditional[0].trim().to_string()
}

fn split_number_format_sections(format: &str) -> Vec<String> {
  let mut sections = Vec::new();
  let mut section = String::new();
  let mut in_quotes = false;
  let mut bracket_depth = 0usize;
  for ch in format.chars() {
    match ch {
      '"' => {
        in_quotes = !in_quotes;
        section.push(ch);
      }
      '[' if !in_quotes => {
        bracket_depth += 1;
        section.push(ch);
      }
      ']' if !in_quotes => {
        bracket_depth = bracket_depth.saturating_sub(1);
        section.push(ch);
      }
      ';' if !in_quotes && bracket_depth == 0 => {
        sections.push(section.trim().to_string());
        section.clear();
      }
      _ => section.push(ch),
    }
  }
  sections.push(section.trim().to_string());
  sections
}

fn parse_format_condition(section: &str) -> Option<(&str, f64, &str)> {
  let rest = section.trim().strip_prefix('[')?;
  let (condition, body) = rest.split_once(']')?;
  for op in ["<=", ">=", "<>", "<", ">", "="] {
    if let Some(value) = condition.strip_prefix(op)
      && let Ok(threshold) = value.trim().parse::<f64>()
    {
      return Some((op, threshold, body));
    }
  }
  None
}

fn compare_format_condition(value: f64, op: &str, threshold: f64) -> bool {
  match op {
    "<=" => value <= threshold,
    ">=" => value >= threshold,
    "<>" => (value - threshold).abs() > f64::EPSILON,
    "<" => value < threshold,
    ">" => value > threshold,
    "=" => (value - threshold).abs() <= f64::EPSILON,
    _ => false,
  }
}

fn strip_number_format_directives(format: &str) -> String {
  let mut result = String::new();
  let mut chars = format.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '[' {
      let directive: String = chars.by_ref().take_while(|next| *next != ']').collect();
      if directive
        .chars()
        .next()
        .is_some_and(|value| matches!(value, '<' | '>' | '='))
      {
        continue;
      }
      continue;
    }
    if ch == '"' {
      for literal in chars.by_ref() {
        if literal == '"' {
          break;
        }
        result.push(literal);
      }
      continue;
    }
    if ch == '\\' {
      if let Some(literal) = chars.next() {
        result.push(literal);
      }
      continue;
    }
    result.push(ch);
  }
  result.trim().to_string()
}

fn format_contains_date_time_token(format: &str) -> bool {
  let mut in_quotes = false;
  let mut in_brackets = false;
  for ch in format.chars() {
    match ch {
      '"' => in_quotes = !in_quotes,
      '[' if !in_quotes => in_brackets = true,
      ']' if !in_quotes => in_brackets = false,
      _ if !in_quotes
        && !in_brackets
        && matches!(ch.to_ascii_lowercase(), 'y' | 'm' | 'd' | 'h' | 's') =>
      {
        return true;
      }
      _ => {}
    }
  }
  false
}

fn format_scientific_pattern(number: f64, format: &str) -> Option<String> {
  let upper = format.to_ascii_uppercase();
  let (mantissa_pattern, exponent_pattern) = upper.split_once('E')?;
  let decimals = mantissa_pattern.split_once('.').map_or(0, |(_, fraction)| {
    fraction
      .chars()
      .filter(|ch| matches!(ch, '0' | '#'))
      .count()
  });
  let exponent_digits = exponent_pattern
    .chars()
    .filter(|ch| matches!(ch, '0' | '#'))
    .count()
    .max(1);
  if number == 0.0 {
    return Some(format!(
      "{:.*}E+{:0width$}",
      decimals,
      0.0,
      0,
      width = exponent_digits
    ));
  }
  let sign = if number.is_sign_negative() { "-" } else { "" };
  let absolute = number.abs();
  let exponent = absolute.log10().floor() as i32;
  let mantissa = absolute / 10_f64.powi(exponent);
  let mantissa = round_half_away_from_zero(mantissa, decimals as i32);
  let (mantissa, exponent) = if mantissa >= 10.0 {
    (mantissa / 10.0, exponent + 1)
  } else {
    (mantissa, exponent)
  };
  Some(format!(
    "{sign}{mantissa:.decimals$}E{}{exp:0width$}",
    if exponent < 0 { "-" } else { "+" },
    exp = exponent.abs(),
    width = exponent_digits
  ))
}

fn format_fraction_pattern(number: f64, format: &str) -> Option<String> {
  if !format.contains('/') {
    return None;
  }
  let sign = if number < 0.0 { "-" } else { "" };
  let absolute = number.abs();
  let whole = absolute.floor() as i64;
  let fraction = absolute - whole as f64;
  let (numerator, denominator) = best_fraction(fraction, 9)?;
  if numerator == 0 {
    return Some(format!("{sign}{whole}"));
  }
  if whole == 0 {
    Some(format!("{sign}{numerator}/{denominator}"))
  } else {
    Some(format!("{sign}{whole} {numerator}/{denominator}"))
  }
}

fn best_fraction(value: f64, max_denominator: i64) -> Option<(i64, i64)> {
  let mut best = None;
  let mut best_error = f64::INFINITY;
  for denominator in 1..=max_denominator {
    let numerator = round_half_away_from_zero(value * denominator as f64, 0) as i64;
    let error = (value - numerator as f64 / denominator as f64).abs();
    if error < best_error {
      best_error = error;
      best = Some((numerator, denominator));
    }
  }
  best
}

fn can_format_as_digit_mask(format: &str) -> bool {
  let mut seen_digit = false;
  for ch in format.chars() {
    match ch {
      '#' | '0' => seen_digit = true,
      '?' | '.' | ',' => return false,
      _ => {}
    }
  }
  seen_digit && format.chars().any(|ch| !matches!(ch, '#' | '0'))
}

fn format_digit_mask(number: f64, format: &str) -> Option<String> {
  let rounded = round_half_away_from_zero(number.abs(), 0) as i64;
  let mut digits: Vec<char> = rounded.to_string().chars().collect();
  let mut result = Vec::new();
  for ch in format.chars().rev() {
    match ch {
      '0' => result.push(digits.pop().unwrap_or('0')),
      '#' => {
        if let Some(digit) = digits.pop() {
          result.push(digit);
        }
      }
      _ => result.push(ch),
    }
  }
  while let Some(digit) = digits.pop() {
    result.push(digit);
  }
  let mut text: String = result.into_iter().rev().collect();
  if number < 0.0 {
    text.insert(0, '-');
  }
  Some(text)
}

fn add_grouping_to_decimal(text: &str) -> String {
  let Some((integer, fraction)) = text.split_once('.') else {
    return add_grouping_separators(text);
  };
  format!("{}.{}", add_grouping_separators(integer), fraction)
}

fn add_grouping_separators(text: &str) -> String {
  let (negative, body) = text
    .strip_prefix('-')
    .map(|body| (true, body))
    .unwrap_or((false, text));
  let mut result = String::new();
  for (index, ch) in body.chars().rev().enumerate() {
    if index > 0 && index % 3 == 0 {
      result.push(',');
    }
    result.push(ch);
  }
  let mut grouped: String = result.chars().rev().collect();
  if negative {
    grouped.insert(0, '-');
  }
  grouped
}

fn format_integer_with_min_digits(value: i64, min_digits: usize) -> String {
  if min_digits == 0 {
    return value.to_string();
  }
  if value < 0 {
    format!("-{:0width$}", value.unsigned_abs(), width = min_digits)
  } else {
    format!("{value:0width$}", width = min_digits)
  }
}

fn pad_integer_part(text: String, min_digits: usize) -> String {
  let (negative, body) = text
    .strip_prefix('-')
    .map(|body| (true, body))
    .unwrap_or((false, text.as_str()));
  let Some((integer, fraction)) = body.split_once('.') else {
    return format_integer_with_min_digits(text.parse::<i64>().unwrap_or_default(), min_digits);
  };
  if integer.len() >= min_digits {
    return text;
  }
  let mut result = String::new();
  if negative {
    result.push('-');
  }
  result.extend(std::iter::repeat_n('0', min_digits - integer.len()));
  result.push_str(integer);
  result.push('.');
  result.push_str(fraction);
  result
}

pub(crate) fn error_text(value: &FormulaValue<'_>) -> Option<&'static str> {
  match value {
    FormulaValue::Error(error) => Some(error_text_value(*error)),
    _ => None,
  }
}

pub(crate) fn error_value(value: &str) -> FormulaErrorValue {
  crate::parser::formula_error_value(value)
    .map(formula_error_from_lex)
    .unwrap_or(FormulaErrorValue::Unknown)
}

pub(crate) fn error_text_value(value: FormulaErrorValue) -> &'static str {
  match value {
    FormulaErrorValue::Null => "#NULL!",
    FormulaErrorValue::Div0 => "#DIV/0!",
    FormulaErrorValue::Value => "#VALUE!",
    FormulaErrorValue::Ref => "#REF!",
    FormulaErrorValue::Name => "#NAME?",
    FormulaErrorValue::Num => "#NUM!",
    FormulaErrorValue::NA => "#N/A",
    FormulaErrorValue::GettingData => "#GETTING_DATA",
    FormulaErrorValue::Spill => "#SPILL!",
    FormulaErrorValue::Calc => "#CALC!",
    FormulaErrorValue::IllegalArgument => "Err:502",
    FormulaErrorValue::Parameter => "Err:511",
    FormulaErrorValue::Unknown => "#UNKNOWN!",
  }
}
