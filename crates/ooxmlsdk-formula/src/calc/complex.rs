use num_complex::Complex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormulaComplex {
  value: Complex<f64>,
  suffix: char,
}

impl FormulaComplex {
  pub fn new(real: f64, imaginary: f64, suffix: char) -> Self {
    Self {
      value: Complex::new(real, imaginary),
      suffix: if suffix == 'j' { 'j' } else { 'i' },
    }
  }

  pub fn from_value(value: Complex<f64>, suffix: char) -> Self {
    Self {
      value,
      suffix: if suffix == 'j' { 'j' } else { 'i' },
    }
  }

  pub fn value(self) -> Complex<f64> {
    self.value
  }

  pub fn real(self) -> f64 {
    self.value.re
  }

  pub fn imaginary(self) -> f64 {
    self.value.im
  }

  pub fn suffix(self) -> char {
    self.suffix
  }
}

pub fn parse_complex_number(value: &str) -> Option<FormulaComplex> {
  let normalized;
  let value = if value.contains(',') {
    normalized = value.replace(',', ".");
    normalized.trim()
  } else {
    value.trim()
  };
  if value == "i" || value == "j" {
    return Some(FormulaComplex::new(0.0, 1.0, value.chars().next()?));
  }
  let suffix = value.chars().last()?;
  if suffix != 'i' && suffix != 'j' {
    return value
      .parse::<f64>()
      .ok()
      .map(|real| FormulaComplex::new(real, 0.0, 'i'));
  }
  let body = &value[..value.len() - suffix.len_utf8()];
  if body.is_empty() || body == "+" {
    return Some(FormulaComplex::new(0.0, 1.0, suffix));
  }
  if body == "-" {
    return Some(FormulaComplex::new(0.0, -1.0, suffix));
  }
  let mut split = None;
  for (index, ch) in body.char_indices().skip(1) {
    let previous = body[..index].chars().next_back();
    if (ch == '+' || ch == '-') && !matches!(previous, Some('e' | 'E')) {
      split = Some(index);
    }
  }
  if let Some(index) = split {
    let real = body[..index].parse::<f64>().ok()?;
    let imaginary = parse_complex_imaginary_part(&body[index..])?;
    Some(FormulaComplex::new(real, imaginary, suffix))
  } else {
    parse_complex_imaginary_part(body).map(|imaginary| FormulaComplex::new(0.0, imaginary, suffix))
  }
}

fn parse_complex_imaginary_part(value: &str) -> Option<f64> {
  match value {
    "" | "+" => Some(1.0),
    "-" => Some(-1.0),
    _ => value.parse::<f64>().ok(),
  }
}

pub fn binary_suffix(left: FormulaComplex, right: FormulaComplex) -> char {
  if left.suffix() == 'j' || right.suffix() == 'j' {
    'j'
  } else {
    'i'
  }
}

pub fn format_complex_number(
  complex: FormulaComplex,
  mut format_component: impl FnMut(f64, bool) -> String,
) -> String {
  let real = complex.real();
  let imaginary = complex.imaginary();
  let has_imaginary = imaginary != 0.0;
  let has_real = !has_imaginary || real != 0.0;
  let mut output = String::new();
  if has_real {
    output.push_str(&format_component(real, false));
  }
  if has_imaginary {
    if imaginary == 1.0 {
      if has_real {
        output.push('+');
      }
    } else if imaginary == -1.0 {
      output.push('-');
    } else {
      output.push_str(&format_component(imaginary, has_real));
    }
    output.push(complex.suffix());
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_spreadsheet_complex_numbers() {
    assert_eq!(
      parse_complex_number("-238+240i"),
      Some(FormulaComplex::new(-238.0, 240.0, 'i'))
    );
    assert_eq!(
      parse_complex_number("0-j"),
      Some(FormulaComplex::new(0.0, -1.0, 'j'))
    );
    assert_eq!(
      parse_complex_number("1,5+2,5i"),
      Some(FormulaComplex::new(1.5, 2.5, 'i'))
    );
    assert_eq!(
      parse_complex_number("4"),
      Some(FormulaComplex::new(4.0, 0.0, 'i'))
    );
  }

  #[test]
  fn formats_spreadsheet_complex_numbers() {
    let format = |value: f64, leading_sign: bool| {
      if leading_sign && value > 0.0 {
        format!("+{value}")
      } else {
        value.to_string()
      }
    };
    assert_eq!(
      format_complex_number(FormulaComplex::new(3.0, 4.0, 'j'), format),
      "3+4j"
    );
    assert_eq!(
      format_complex_number(FormulaComplex::new(0.0, -1.0, 'i'), format),
      "-i"
    );
  }
}
