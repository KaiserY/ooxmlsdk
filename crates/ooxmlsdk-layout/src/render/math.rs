use ooxmlsdk::schemas::{
  m, schemas_microsoft_com_office_drawing_2010_main as a14,
  schemas_openxmlformats_org_wordprocessingml_2006_main as w,
};

pub fn text_math_text(math: &a14::TextMath) -> String {
  let mut text = String::new();
  for child in &math.text_math_choice {
    match child {
      a14::TextMathChoice::Paragraph(paragraph) => append_paragraph_text(paragraph, &mut text),
      a14::TextMathChoice::OfficeMath(math) => append_office_math_text(math, &mut text),
      a14::TextMathChoice::Run(run) => append_run_text(run, &mut text),
    }
  }
  text
}

pub(crate) fn wordprocessing_math_text(choice: &w::ParagraphChoice) -> Option<String> {
  let mut text = String::new();
  match choice {
    w::ParagraphChoice::Paragraph(value) => append_paragraph_text(value, &mut text),
    w::ParagraphChoice::OfficeMath(value) => append_office_math_text(value, &mut text),
    w::ParagraphChoice::Accent(value) => append_accent_text(value, &mut text),
    w::ParagraphChoice::Bar(value) => append_base_text(&value.base, &mut text),
    w::ParagraphChoice::Box(value) => append_base_text(&value.base, &mut text),
    w::ParagraphChoice::BorderBox(value) => append_base_text(&value.base, &mut text),
    w::ParagraphChoice::Delimiter(value) => append_delimiter_text(value, &mut text),
    w::ParagraphChoice::EquationArray(value) => append_equation_array_text(value, &mut text),
    w::ParagraphChoice::Fraction(value) => append_fraction_text(value, &mut text),
    w::ParagraphChoice::MathFunction(value) => append_math_function_text(value, &mut text),
    w::ParagraphChoice::GroupChar(value) => append_base_text(&value.base, &mut text),
    w::ParagraphChoice::LimitLower(value) => append_limit_lower_text(value, &mut text),
    w::ParagraphChoice::LimitUpper(value) => append_limit_upper_text(value, &mut text),
    w::ParagraphChoice::Matrix(value) => append_matrix_text(value, &mut text),
    w::ParagraphChoice::Nary(value) => append_nary_text(value, &mut text),
    w::ParagraphChoice::Phantom(value) => append_base_text(&value.base, &mut text),
    w::ParagraphChoice::Radical(value) => append_radical_text(value, &mut text),
    w::ParagraphChoice::PreSubSuper(value) => append_pre_sub_super_text(value, &mut text),
    w::ParagraphChoice::Subscript(value) => append_subscript_text(value, &mut text),
    w::ParagraphChoice::SubSuperscript(value) => append_sub_superscript_text(value, &mut text),
    w::ParagraphChoice::Superscript(value) => append_superscript_text(value, &mut text),
    w::ParagraphChoice::MRun(value) => append_run_text(value, &mut text),
    _ => return None,
  }
  Some(text)
}

fn append_paragraph_text(paragraph: &m::Paragraph, text: &mut String) {
  for choice in &paragraph.paragraph_choice {
    match choice {
      m::ParagraphChoice::OfficeMath(math) => append_office_math_text(math, text),
      m::ParagraphChoice::MRun(run) => append_run_text(run, text),
      _ => {}
    }
  }
}

macro_rules! append_math_choice {
  ($choice_type:ident, $choice:expr, $text:expr) => {
    match $choice {
      m::$choice_type::Accent(value) => append_accent_text(value, $text),
      m::$choice_type::Bar(value) => append_base_text(&value.base, $text),
      m::$choice_type::Box(value) => append_base_text(&value.base, $text),
      m::$choice_type::BorderBox(value) => append_base_text(&value.base, $text),
      m::$choice_type::Delimiter(value) => append_delimiter_text(value, $text),
      m::$choice_type::EquationArray(value) => append_equation_array_text(value, $text),
      m::$choice_type::Fraction(value) => append_fraction_text(value, $text),
      m::$choice_type::MathFunction(value) => append_math_function_text(value, $text),
      m::$choice_type::GroupChar(value) => append_base_text(&value.base, $text),
      m::$choice_type::LimitLower(value) => append_limit_lower_text(value, $text),
      m::$choice_type::LimitUpper(value) => append_limit_upper_text(value, $text),
      m::$choice_type::Matrix(value) => append_matrix_text(value, $text),
      m::$choice_type::Nary(value) => append_nary_text(value, $text),
      m::$choice_type::Phantom(value) => append_base_text(&value.base, $text),
      m::$choice_type::Radical(value) => append_radical_text(value, $text),
      m::$choice_type::PreSubSuper(value) => append_pre_sub_super_text(value, $text),
      m::$choice_type::Subscript(value) => append_subscript_text(value, $text),
      m::$choice_type::SubSuperscript(value) => append_sub_superscript_text(value, $text),
      m::$choice_type::Superscript(value) => append_superscript_text(value, $text),
      m::$choice_type::Run(run) => append_run_text(run, $text),
      m::$choice_type::Paragraph(paragraph) => append_paragraph_text(paragraph, $text),
      m::$choice_type::OfficeMath(math) => append_office_math_text(math, $text),
      _ => {}
    }
  };
}

fn append_office_math_text(math: &m::OfficeMath, text: &mut String) {
  for choice in &math.office_math_choice {
    append_math_choice!(OfficeMathChoice, choice, text);
  }
}

macro_rules! define_argument_appender {
  ($function:ident, $argument:ident, $field:ident, $choice_type:ident) => {
    fn $function(argument: &m::$argument, text: &mut String) {
      for choice in &argument.$field {
        append_math_choice!($choice_type, choice, text);
      }
    }
  };
}

define_argument_appender!(append_base_text, Base, base_choice, BaseChoice);
define_argument_appender!(
  append_numerator_text,
  Numerator,
  numerator_choice,
  NumeratorChoice
);
define_argument_appender!(
  append_denominator_text,
  Denominator,
  denominator_choice,
  DenominatorChoice
);
define_argument_appender!(
  append_function_name_text,
  FunctionName,
  function_name_choice,
  FunctionNameChoice
);
define_argument_appender!(append_limit_text, Limit, limit_choice, LimitChoice);
define_argument_appender!(
  append_sub_argument_text,
  SubArgument,
  sub_argument_choice,
  SubArgumentChoice
);
define_argument_appender!(
  append_super_argument_text,
  SuperArgument,
  super_argument_choice,
  SuperArgumentChoice
);
define_argument_appender!(append_degree_text, Degree, degree_choice, DegreeChoice);

fn append_accent_text(accent: &m::Accent, text: &mut String) {
  append_base_text(&accent.base, text);
  if let Some(character) = accent
    .accent_properties
    .as_deref()
    .and_then(|properties| properties.accent_char.as_ref())
  {
    text.push_str(character.val.as_str());
  }
}

fn append_delimiter_text(delimiter: &m::Delimiter, text: &mut String) {
  let properties = delimiter.delimiter_properties.as_deref();
  if let Some(character) = properties.and_then(|properties| properties.begin_char.as_ref()) {
    text.push_str(character.val.as_str());
  }
  for (index, base) in delimiter.base.iter().enumerate() {
    if index > 0
      && let Some(character) = properties.and_then(|properties| properties.separator_char.as_ref())
    {
      text.push_str(character.val.as_str());
    }
    append_base_text(base, text);
  }
  if let Some(character) = properties.and_then(|properties| properties.end_char.as_ref()) {
    text.push_str(character.val.as_str());
  }
}

fn append_equation_array_text(array: &m::EquationArray, text: &mut String) {
  for base in &array.base {
    append_base_text(base, text);
  }
}

fn append_fraction_text(fraction: &m::Fraction, text: &mut String) {
  append_numerator_text(&fraction.numerator, text);
  if fraction
    .fraction_properties
    .as_deref()
    .and_then(|properties| properties.fraction_type.as_ref())
    .is_some_and(|fraction_type| {
      matches!(
        fraction_type.val,
        m::FractionTypeValues::Skewed | m::FractionTypeValues::Linear
      )
    })
  {
    // ECMA-376 Part 1, 22.1.2.36 and 22.1.2.38 render both skewed and
    // linear fractions with the fraction slash character U+2044.
    text.push('\u{2044}');
  }
  append_denominator_text(&fraction.denominator, text);
}

fn append_math_function_text(function: &m::MathFunction, text: &mut String) {
  append_function_name_text(&function.function_name, text);
  append_base_text(&function.base, text);
}

fn append_limit_lower_text(limit: &m::LimitLower, text: &mut String) {
  append_base_text(&limit.base, text);
  append_limit_text(&limit.limit, text);
}

fn append_limit_upper_text(limit: &m::LimitUpper, text: &mut String) {
  append_base_text(&limit.base, text);
  append_limit_text(&limit.limit, text);
}

fn append_matrix_text(matrix: &m::Matrix, text: &mut String) {
  for row in &matrix.matrix_row {
    for base in &row.base {
      append_base_text(base, text);
    }
  }
}

fn append_nary_text(nary: &m::Nary, text: &mut String) {
  if let Some(character) = nary
    .nary_properties
    .as_deref()
    .and_then(|properties| properties.accent_char.as_ref())
  {
    text.push_str(character.val.as_str());
  }
  append_sub_argument_text(&nary.sub_argument, text);
  append_super_argument_text(&nary.super_argument, text);
  append_base_text(&nary.base, text);
}

fn append_radical_text(radical: &m::Radical, text: &mut String) {
  text.push('\u{221a}');
  append_base_text(&radical.base, text);
  append_degree_text(&radical.degree, text);
}

fn append_pre_sub_super_text(value: &m::PreSubSuper, text: &mut String) {
  append_sub_argument_text(&value.sub_argument, text);
  append_super_argument_text(&value.super_argument, text);
  append_base_text(&value.base, text);
}

fn append_subscript_text(value: &m::Subscript, text: &mut String) {
  append_base_text(&value.base, text);
  append_sub_argument_text(&value.sub_argument, text);
}

fn append_sub_superscript_text(value: &m::SubSuperscript, text: &mut String) {
  append_base_text(&value.base, text);
  append_sub_argument_text(&value.sub_argument, text);
  append_super_argument_text(&value.super_argument, text);
}

fn append_superscript_text(value: &m::Superscript, text: &mut String) {
  append_base_text(&value.base, text);
  append_super_argument_text(&value.super_argument, text);
}

fn append_run_text(run: &m::Run, text: &mut String) {
  let (script, style, normal_text) = math_run_variant(run.math_run_properties.as_deref());
  for choice in &run.run_choice {
    if let m::RunChoice::MText(value) = choice
      && let Some(content) = value.xml_content.as_deref()
    {
      if normal_text {
        text.push_str(content);
      } else {
        append_math_variant_text(content, script, style, text);
      }
    }
  }
}

fn math_run_variant(
  properties: Option<&m::RunProperties>,
) -> (m::ScriptValues, m::StyleValues, bool) {
  let Some(properties) = properties else {
    return (m::ScriptValues::Roman, m::StyleValues::Italic, false);
  };
  match properties.run_properties_choice.as_ref() {
    Some(m::RunPropertiesChoice::NormalText(normal)) => (
      m::ScriptValues::Roman,
      m::StyleValues::Plain,
      normal.val.is_none_or(math_on_off),
    ),
    Some(m::RunPropertiesChoice::Sequence(sequence)) => (
      sequence
        .script
        .as_ref()
        .map(|script| script.val)
        .unwrap_or(m::ScriptValues::Roman),
      sequence
        .style
        .as_ref()
        .map(|style| style.val)
        .unwrap_or(m::StyleValues::Italic),
      false,
    ),
    None => (m::ScriptValues::Roman, m::StyleValues::Italic, false),
  }
}

fn math_on_off(value: m::BooleanValues) -> bool {
  matches!(
    value,
    m::BooleanValues::True | m::BooleanValues::On | m::BooleanValues::One
  )
}

fn append_math_variant_text(
  value: &str,
  script: m::ScriptValues,
  style: m::StyleValues,
  text: &mut String,
) {
  for character in value.chars() {
    text.push(math_variant_character(character, script, style).unwrap_or(character));
  }
}

fn math_variant_character(
  character: char,
  script: m::ScriptValues,
  style: m::StyleValues,
) -> Option<char> {
  if script != m::ScriptValues::Roman || style == m::StyleValues::Plain {
    return None;
  }
  if let Some(character) = roman_greek_variant_character(character, style) {
    return Some(character);
  }
  let offset = match character {
    'A'..='Z' => character as u32 - 'A' as u32,
    'a'..='z' => character as u32 - 'a' as u32,
    '0'..='9' => character as u32 - '0' as u32,
    _ => return None,
  };
  let codepoint = match (style, character) {
    (m::StyleValues::Italic, 'A'..='Z') => 0x1d434 + offset,
    (m::StyleValues::Italic, 'h') => return Some('\u{210e}'),
    (m::StyleValues::Italic, 'a'..='z') => 0x1d44e + offset,
    (m::StyleValues::Bold, 'A'..='Z') => 0x1d400 + offset,
    (m::StyleValues::Bold, 'a'..='z') => 0x1d41a + offset,
    (m::StyleValues::Bold, '0'..='9') => 0x1d7ce + offset,
    (m::StyleValues::BoldItalic, 'A'..='Z') => 0x1d468 + offset,
    (m::StyleValues::BoldItalic, 'a'..='z') => 0x1d482 + offset,
    (m::StyleValues::BoldItalic, '0'..='9') => 0x1d7ce + offset,
    _ => return None,
  };
  char::from_u32(codepoint)
}

fn roman_greek_variant_character(character: char, style: m::StyleValues) -> Option<char> {
  let (capital_base, small_base, variant_base) = match style {
    m::StyleValues::Bold => (0x1d6a8, 0x1d6c2, 0x1d6dc),
    m::StyleValues::Italic => (0x1d6e2, 0x1d6fc, 0x1d716),
    m::StyleValues::BoldItalic => (0x1d71c, 0x1d736, 0x1d750),
    m::StyleValues::Plain => return None,
  };
  const CAPITALS: &str = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡϴΣΤΥΦΧΨΩ";
  const SMALLS: &str = "αβγδεζηθικλμνξοπρςστυφχψω";
  const VARIANTS: &str = "ϵϑϰϕϱϖ";
  let codepoint = if character == '∇' {
    capital_base + 25
  } else if character == '∂' {
    small_base + 25
  } else if let Some(index) = CAPITALS.chars().position(|value| value == character) {
    capital_base + index as u32
  } else if let Some(index) = SMALLS.chars().position(|value| value == character) {
    small_base + index as u32
  } else {
    let index = VARIANTS.chars().position(|value| value == character)?;
    variant_base + index as u32
  };
  char::from_u32(codepoint)
}

#[cfg(test)]
mod tests {
  use super::{text_math_text, wordprocessing_math_text};
  use ooxmlsdk::schemas::{
    schemas_microsoft_com_office_drawing_2010_main::TextMath,
    schemas_openxmlformats_org_wordprocessingml_2006_main as w,
  };
  use ooxmlsdk::sdk::SdkType;

  #[test]
  fn text_math_text_reads_generated_math_fragment() {
    // ppt/diagrams/data1.xml. a14:m preserves selected MCE math as raw XML bytes;
    // the selected MCE dgm:pt already contains the m:oMathPara child.
    let xml = r#"<a14:m xmlns:a14="http://schemas.microsoft.com/office/drawing/2010/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMathPara><m:oMathParaPr><m:jc m:val="centerGroup"/></m:oMathParaPr><m:oMath><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝐴</m:t></m:r><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>=</m:t></m:r><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="el-GR" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝜋</m:t></m:r><m:sSup><m:sSupPr><m:ctrlPr><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr></m:ctrlPr></m:sSupPr><m:e><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝑟</m:t></m:r></m:e><m:sup><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>2</m:t></m:r></m:sup></m:sSup></m:oMath></m:oMathPara></a14:m>"#;
    let math = TextMath::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(text_math_text(&math), "𝐴=𝜋𝑟2");
  }

  #[test]
  fn wordprocessing_math_uses_default_roman_italic_and_explicit_delimiters() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:d><m:dPr><m:begChr m:val="["/><m:endChr m:val="]"/></m:dPr><m:e><m:r><m:t>abcπ</m:t></m:r></m:e></m:d></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("[𝑎𝑏𝑐𝜋]")
    );
  }

  #[test]
  fn wordprocessing_normal_math_text_stays_unmapped() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:r><m:rPr><m:nor/></m:rPr><m:t>rate</m:t></m:r></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("rate")
    );
  }

  #[test]
  fn wordprocessing_linear_and_skewed_fractions_include_fraction_slash() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:f><m:fPr><m:type m:val="lin"/></m:fPr><m:num><m:r><m:t>a</m:t></m:r></m:num><m:den><m:r><m:t>b</m:t></m:r></m:den></m:f><m:f><m:fPr><m:type m:val="skw"/></m:fPr><m:num><m:r><m:t>c</m:t></m:r></m:num><m:den><m:r><m:t>d</m:t></m:r></m:den></m:f><m:f><m:fPr><m:type m:val="noBar"/></m:fPr><m:num><m:r><m:t>e</m:t></m:r></m:num><m:den><m:r><m:t>f</m:t></m:r></m:den></m:f></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("𝑎⁄𝑏𝑐⁄𝑑𝑒𝑓")
    );
  }
}
