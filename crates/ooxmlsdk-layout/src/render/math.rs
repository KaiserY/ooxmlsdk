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
    w::ParagraphChoice::GroupChar(value) => append_group_char_text(value, &mut text),
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

pub(crate) fn wordprocessing_math_run_properties(
  choice: &w::ParagraphChoice,
) -> Option<&w::RunProperties> {
  match choice {
    w::ParagraphChoice::MRun(run) => run.run_properties.as_deref(),
    w::ParagraphChoice::OfficeMath(math) => math
      .office_math_choice
      .iter()
      .find_map(office_math_choice_run_properties),
    w::ParagraphChoice::Paragraph(paragraph) => {
      paragraph
        .paragraph_choice
        .iter()
        .find_map(|choice| match choice {
          m::ParagraphChoice::MRun(run) => run.run_properties.as_deref(),
          m::ParagraphChoice::OfficeMath(math) => math
            .office_math_choice
            .iter()
            .find_map(office_math_choice_run_properties),
          _ => None,
        })
    }
    _ => None,
  }
}

fn office_math_choice_run_properties(choice: &m::OfficeMathChoice) -> Option<&w::RunProperties> {
  match choice {
    m::OfficeMathChoice::Run(run) => run.run_properties.as_deref(),
    _ => None,
  }
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
      m::$choice_type::GroupChar(value) => append_group_char_text(value, $text),
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
    // Office fixed output exposes the stretched/accent glyph as a separate
    // searchable-text portion. Keeping it out of the base glyph's shaping
    // cluster also prevents the ToUnicode mapping from losing the base.
    text.push(' ');
    text.push_str(character.val.as_str());
  }
}

fn append_delimiter_text(delimiter: &m::Delimiter, text: &mut String) {
  let properties = delimiter.delimiter_properties.as_deref();
  // ECMA-376 Part 1, 22.1.2.24 defaults omitted delimiter properties to
  // parentheses and the separator to a vertical bar.  An explicitly empty
  // value is different: it suppresses that delimiter.
  let begin = properties
    .and_then(|properties| properties.begin_char.as_ref())
    .map_or("(", |character| character.val.as_str());
  let separator = properties
    .and_then(|properties| properties.separator_char.as_ref())
    .map_or("|", |character| character.val.as_str());
  let end = properties
    .and_then(|properties| properties.end_char.as_ref())
    .map_or(")", |character| character.val.as_str());
  text.push_str(begin);
  for (index, base) in delimiter.base.iter().enumerate() {
    if index > 0 {
      text.push_str(separator);
    }
    let mut base_text = String::new();
    append_base_text(base, &mut base_text);
    for (line_index, line) in base_text.split('\n').enumerate() {
      if line_index > 0 {
        // A stretched delimiter surrounds every visual row. Repeating the
        // boundary characters in the flattened representation matches the
        // searchable text emitted by Word's fixed-output PDF.
        text.push_str(end);
        text.push('\n');
        text.push_str(begin);
      }
      text.push_str(line);
    }
  }
  text.push_str(end);
}

fn append_equation_array_text(array: &m::EquationArray, text: &mut String) {
  for (index, base) in array.base.iter().enumerate() {
    if index > 0 {
      text.push('\n');
    }
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

fn append_group_char_text(group: &m::GroupChar, text: &mut String) {
  append_base_text(&group.base, text);
  text.push_str(
    group
      .group_char_properties
      .as_deref()
      .and_then(|properties| properties.accent_char.as_ref())
      .map_or("\u{23df}", |character| character.val.as_str()),
  );
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
  for (row_index, row) in matrix.matrix_row.iter().enumerate() {
    if row_index > 0 {
      text.push('\n');
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MathRunCharacter {
  pub(crate) source: char,
  pub(crate) rendered: char,
}

pub(crate) fn math_run_characters(run: &m::Run) -> Vec<MathRunCharacter> {
  let mut characters = Vec::new();
  visit_run_characters(run, |character| characters.push(character));
  characters
}

fn append_run_text(run: &m::Run, text: &mut String) {
  visit_run_characters(run, |character| text.push(character.rendered));
}

fn visit_run_characters(run: &m::Run, mut visit: impl FnMut(MathRunCharacter)) {
  let (script, style, normal_text) = math_run_variant(run.math_run_properties.as_deref());
  for choice in &run.run_choice {
    let content = match choice {
      m::RunChoice::MText(value) => value.xml_content.as_deref(),
      m::RunChoice::WText(value) => value.0.xml_content.as_deref(),
      _ => None,
    };
    if let Some(content) = content {
      for source in content.chars() {
        let rendered = if normal_text {
          source
        } else {
          math_variant_character(source, script, style).unwrap_or(source)
        };
        visit(MathRunCharacter { source, rendered });
      }
      continue;
    }
    let character = match choice {
      m::RunChoice::TabChar => Some('\t'),
      m::RunChoice::CarriageReturn | m::RunChoice::Break(_) => Some('\n'),
      m::RunChoice::NoBreakHyphen => Some('\u{2011}'),
      m::RunChoice::SoftHyphen => Some('\u{00ad}'),
      _ => None,
    };
    if let Some(character) = character {
      visit(MathRunCharacter {
        source: character,
        rendered: character,
      });
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

fn math_variant_character(
  character: char,
  script: m::ScriptValues,
  style: m::StyleValues,
) -> Option<char> {
  // ECMA-376 Part 1 §§22.1.2.94 and 22.1.2.111 require m:scr and
  // m:sty to map the serialized base character into the corresponding
  // Unicode mathematical alphabet. Unicode 16 §22.2.3 Table 22-2 defines
  // the complete repertoire. Script, Fraktur, double-struck, and monospace
  // intentionally have fewer style axes than Roman or sans-serif, so the
  // unsupported italic axis preserves the same semantic alphabet rather
  // than falling back to an ordinary ASCII character.
  match script {
    m::ScriptValues::Roman => roman_variant_character(character, style),
    m::ScriptValues::Script => script_variant_character(character, style),
    m::ScriptValues::Fraktur => fraktur_variant_character(character, style),
    m::ScriptValues::DoubleStruck => double_struck_variant_character(character),
    m::ScriptValues::SansSerif => sans_serif_variant_character(character, style),
    m::ScriptValues::Monospace => monospace_variant_character(character),
  }
}

fn roman_variant_character(character: char, style: m::StyleValues) -> Option<char> {
  if style == m::StyleValues::Plain {
    return None;
  }
  if let Some(character) = roman_greek_variant_character(character, style) {
    return Some(character);
  }
  if style == m::StyleValues::Italic && character == 'h' {
    // U+1D455 is the historical hole occupied by PLANCK CONSTANT U+210E.
    return Some('\u{210e}');
  }
  match style {
    m::StyleValues::Plain => None,
    m::StyleValues::Bold => {
      ascii_math_variant_character(character, 0x1d400, 0x1d41a, Some(0x1d7ce))
    }
    m::StyleValues::Italic => ascii_math_variant_character(character, 0x1d434, 0x1d44e, None),
    m::StyleValues::BoldItalic => {
      // Unicode has no bold-italic digit alphabet; Office uses bold digits.
      ascii_math_variant_character(character, 0x1d468, 0x1d482, Some(0x1d7ce))
    }
  }
}

fn script_variant_character(character: char, style: m::StyleValues) -> Option<char> {
  if matches!(style, m::StyleValues::Bold | m::StyleValues::BoldItalic) {
    return ascii_math_variant_character(character, 0x1d4d0, 0x1d4ea, None);
  }
  // Unicode unified these pre-existing Letterlike Symbols with the regular
  // mathematical script alphabet and left holes in U+1D49C..U+1D4CF.
  // Word 12 fixed output additionally realizes script small l as U+2113;
  // equation.docx is the local serialized/fixed-output counterexample.
  let letterlike = match character {
    'B' => '\u{212c}',
    'E' => '\u{2130}',
    'F' => '\u{2131}',
    'H' => '\u{210b}',
    'I' => '\u{2110}',
    'L' => '\u{2112}',
    'M' => '\u{2133}',
    'R' => '\u{211b}',
    'e' => '\u{212f}',
    'g' => '\u{210a}',
    'l' => '\u{2113}',
    'o' => '\u{2134}',
    _ => return ascii_math_variant_character(character, 0x1d49c, 0x1d4b6, None),
  };
  Some(letterlike)
}

fn fraktur_variant_character(character: char, style: m::StyleValues) -> Option<char> {
  if matches!(style, m::StyleValues::Bold | m::StyleValues::BoldItalic) {
    return ascii_math_variant_character(character, 0x1d56c, 0x1d586, None);
  }
  let letterlike = match character {
    'C' => '\u{212d}',
    'H' => '\u{210c}',
    'I' => '\u{2111}',
    'R' => '\u{211c}',
    'Z' => '\u{2128}',
    _ => return ascii_math_variant_character(character, 0x1d504, 0x1d51e, None),
  };
  Some(letterlike)
}

fn double_struck_variant_character(character: char) -> Option<char> {
  let letterlike = match character {
    'C' => '\u{2102}',
    'H' => '\u{210d}',
    'N' => '\u{2115}',
    'P' => '\u{2119}',
    'Q' => '\u{211a}',
    'R' => '\u{211d}',
    'Z' => '\u{2124}',
    _ => return ascii_math_variant_character(character, 0x1d538, 0x1d552, Some(0x1d7d8)),
  };
  Some(letterlike)
}

fn sans_serif_variant_character(character: char, style: m::StyleValues) -> Option<char> {
  let (capital_base, small_base, digit_base) = match style {
    m::StyleValues::Plain => (0x1d5a0, 0x1d5ba, 0x1d7e2),
    m::StyleValues::Bold => {
      if let Some(character) = greek_variant_character(character, 0x1d756, 0x1d770, 0x1d78a) {
        return Some(character);
      }
      (0x1d5d4, 0x1d5ee, 0x1d7ec)
    }
    m::StyleValues::Italic => (0x1d608, 0x1d622, 0x1d7e2),
    m::StyleValues::BoldItalic => {
      if let Some(character) = greek_variant_character(character, 0x1d790, 0x1d7aa, 0x1d7c4) {
        return Some(character);
      }
      (0x1d63c, 0x1d656, 0x1d7ec)
    }
  };
  // Unicode has upright and bold sans-serif digit alphabets, but no italic
  // digit alphabets. Preserve the requested family and select its boldness.
  ascii_math_variant_character(character, capital_base, small_base, Some(digit_base))
}

fn monospace_variant_character(character: char) -> Option<char> {
  ascii_math_variant_character(character, 0x1d670, 0x1d68a, Some(0x1d7f6))
}

fn ascii_math_variant_character(
  character: char,
  capital_base: u32,
  small_base: u32,
  digit_base: Option<u32>,
) -> Option<char> {
  let codepoint = match character {
    'A'..='Z' => capital_base + (character as u32 - 'A' as u32),
    'a'..='z' => small_base + (character as u32 - 'a' as u32),
    '0'..='9' => digit_base? + (character as u32 - '0' as u32),
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
  greek_variant_character(character, capital_base, small_base, variant_base)
}

fn greek_variant_character(
  character: char,
  capital_base: u32,
  small_base: u32,
  variant_base: u32,
) -> Option<char> {
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
  use super::{
    math_run_characters, math_variant_character, text_math_text,
    wordprocessing_math_run_properties, wordprocessing_math_text,
  };
  use ooxmlsdk::schemas::{
    m, schemas_microsoft_com_office_drawing_2010_main::TextMath,
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
  fn wordprocessing_double_struck_math_maps_ascii_to_unicode() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:r><m:rPr><m:scr m:val="double-struck"/><m:sty m:val="bi"/></m:rPr><m:t>R C z 3〔</m:t></m:r></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("ℝ ℂ 𝕫 𝟛〔")
    );

    let w::ParagraphChoice::OfficeMath(math) = &paragraph.paragraph_choice[0] else {
      panic!("expected OfficeMath");
    };
    let m::OfficeMathChoice::Run(run) = &math.office_math_choice[0] else {
      panic!("expected direct math run");
    };
    assert_eq!(
      math_run_characters(run)
        .into_iter()
        .map(|character| (character.source, character.rendered))
        .collect::<Vec<_>>(),
      [
        ('R', 'ℝ'),
        (' ', ' '),
        ('C', 'ℂ'),
        (' ', ' '),
        ('z', '𝕫'),
        (' ', ' '),
        ('3', '𝟛'),
        ('〔', '〔'),
      ]
    );
  }

  #[test]
  fn omml_script_style_mapping_covers_unicode_families_and_letterlike_holes() {
    use ooxmlsdk::schemas::m::{ScriptValues as Script, StyleValues as Style};

    let roman_and_family_matrix = [
      ('A', Script::Roman, Style::Plain, None),
      ('A', Script::Roman, Style::Bold, Some('\u{1d400}')),
      ('A', Script::Roman, Style::Italic, Some('\u{1d434}')),
      ('A', Script::Roman, Style::BoldItalic, Some('\u{1d468}')),
      ('h', Script::Roman, Style::Italic, Some('\u{210e}')),
      ('0', Script::Roman, Style::Bold, Some('\u{1d7ce}')),
      ('0', Script::Roman, Style::Italic, None),
      ('0', Script::Roman, Style::BoldItalic, Some('\u{1d7ce}')),
      ('α', Script::Roman, Style::Bold, Some('\u{1d6c2}')),
      ('α', Script::Roman, Style::Italic, Some('\u{1d6fc}')),
      ('α', Script::Roman, Style::BoldItalic, Some('\u{1d736}')),
      ('A', Script::Script, Style::Plain, Some('\u{1d49c}')),
      ('A', Script::Script, Style::Italic, Some('\u{1d49c}')),
      ('A', Script::Script, Style::Bold, Some('\u{1d4d0}')),
      ('A', Script::Script, Style::BoldItalic, Some('\u{1d4d0}')),
      ('z', Script::Script, Style::Bold, Some('\u{1d503}')),
      ('A', Script::Fraktur, Style::Plain, Some('\u{1d504}')),
      ('A', Script::Fraktur, Style::Italic, Some('\u{1d504}')),
      ('A', Script::Fraktur, Style::Bold, Some('\u{1d56c}')),
      ('A', Script::Fraktur, Style::BoldItalic, Some('\u{1d56c}')),
      ('z', Script::Fraktur, Style::Bold, Some('\u{1d59f}')),
      ('e', Script::DoubleStruck, Style::Plain, Some('\u{1d556}')),
      (
        'e',
        Script::DoubleStruck,
        Style::BoldItalic,
        Some('\u{1d556}'),
      ),
      ('0', Script::DoubleStruck, Style::Italic, Some('\u{1d7d8}')),
      ('A', Script::SansSerif, Style::Plain, Some('\u{1d5a0}')),
      ('z', Script::SansSerif, Style::Plain, Some('\u{1d5d3}')),
      ('0', Script::SansSerif, Style::Plain, Some('\u{1d7e2}')),
      ('A', Script::SansSerif, Style::Italic, Some('\u{1d608}')),
      ('z', Script::SansSerif, Style::Italic, Some('\u{1d63b}')),
      ('0', Script::SansSerif, Style::Italic, Some('\u{1d7e2}')),
      ('A', Script::SansSerif, Style::Bold, Some('\u{1d5d4}')),
      ('z', Script::SansSerif, Style::Bold, Some('\u{1d607}')),
      ('0', Script::SansSerif, Style::Bold, Some('\u{1d7ec}')),
      ('α', Script::SansSerif, Style::Bold, Some('\u{1d770}')),
      ('A', Script::SansSerif, Style::BoldItalic, Some('\u{1d63c}')),
      ('z', Script::SansSerif, Style::BoldItalic, Some('\u{1d66f}')),
      ('0', Script::SansSerif, Style::BoldItalic, Some('\u{1d7ec}')),
      ('α', Script::SansSerif, Style::BoldItalic, Some('\u{1d7aa}')),
      ('A', Script::Monospace, Style::Plain, Some('\u{1d670}')),
      ('z', Script::Monospace, Style::BoldItalic, Some('\u{1d6a3}')),
      ('0', Script::Monospace, Style::Italic, Some('\u{1d7f6}')),
      ('α', Script::Monospace, Style::Plain, None),
      ('+', Script::Script, Style::Plain, None),
    ];
    for (source, script, style, expected) in roman_and_family_matrix {
      assert_eq!(
        math_variant_character(source, script, style),
        expected,
        "source={source:?} script={script:?} style={style:?}"
      );
    }

    let script_holes = [
      ('B', '\u{212c}'),
      ('E', '\u{2130}'),
      ('F', '\u{2131}'),
      ('H', '\u{210b}'),
      ('I', '\u{2110}'),
      ('L', '\u{2112}'),
      ('M', '\u{2133}'),
      ('R', '\u{211b}'),
      ('e', '\u{212f}'),
      ('g', '\u{210a}'),
      ('l', '\u{2113}'),
      ('o', '\u{2134}'),
    ];
    for (source, expected) in script_holes {
      assert_eq!(
        math_variant_character(source, Script::Script, Style::Plain),
        Some(expected)
      );
    }

    let fraktur_holes = [
      ('C', '\u{212d}'),
      ('H', '\u{210c}'),
      ('I', '\u{2111}'),
      ('R', '\u{211c}'),
      ('Z', '\u{2128}'),
    ];
    for (source, expected) in fraktur_holes {
      assert_eq!(
        math_variant_character(source, Script::Fraktur, Style::Plain),
        Some(expected)
      );
    }

    let double_struck_holes = [
      ('C', '\u{2102}'),
      ('H', '\u{210d}'),
      ('N', '\u{2115}'),
      ('P', '\u{2119}'),
      ('Q', '\u{211a}'),
      ('R', '\u{211d}'),
      ('Z', '\u{2124}'),
    ];
    for (source, expected) in double_struck_holes {
      assert_eq!(
        math_variant_character(source, Script::DoubleStruck, Style::Plain),
        Some(expected)
      );
    }
  }

  #[test]
  fn wordprocessing_math_run_applies_script_fraktur_and_double_struck_in_one_chain() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:r><m:rPr><m:scr m:val="script"/><m:sty m:val="p"/></m:rPr><m:t>l</m:t></m:r><m:r><m:rPr><m:scr m:val="fraktur"/><m:sty m:val="p"/></m:rPr><m:t>⇏↻⋩e</m:t></m:r><m:r><m:rPr><m:scr m:val="script"/><m:sty m:val="p"/></m:rPr><m:t>T</m:t></m:r><m:r><m:rPr><m:scr m:val="double-struck"/><m:sty m:val="p"/></m:rPr><m:t>e</m:t></m:r></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("ℓ⇏↻⋩𝔢𝒯𝕖")
    );
  }

  #[test]
  fn wordprocessing_math_exposes_direct_word_run_properties() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:r><w:rPr><w:rFonts w:ascii="Cambria Math"/><w:sz w:val="48"/></w:rPr><m:t>R</m:t></m:r></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();
    let properties = wordprocessing_math_run_properties(&paragraph.paragraph_choice[0]).unwrap();

    assert_eq!(
      properties
        .run_properties_choice
        .iter()
        .filter(|choice| matches!(choice, w::RunPropertiesChoice::FontSize(_)))
        .count(),
      1
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

  #[test]
  fn wordprocessing_math_preserves_word_text_and_structure_defaults() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:d><m:e><m:r><w:t>x</w:t></m:r></m:e><m:e><m:r><m:t>y</m:t></m:r></m:e></m:d><m:eqArr><m:e><m:r><m:t>a</m:t></m:r></m:e><m:e><m:r><m:t>b</m:t></m:r></m:e></m:eqArr><m:m><m:mr><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>2</m:t></m:r></m:e></m:mr><m:mr><m:e><m:r><m:t>3</m:t></m:r></m:e><m:e><m:r><m:t>4</m:t></m:r></m:e></m:mr></m:m></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("(𝑥|𝑦)𝑎\n𝑏12\n34")
    );
  }

  #[test]
  fn wordprocessing_accent_keeps_base_from_word_run_properties() {
    let xml = r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMath><m:acc><m:accPr><m:chr m:val="&#x301;"/></m:accPr><m:e><m:r><w:rPr><w:rFonts w:ascii="Cambria Math"/></w:rPr><m:t>a</m:t></m:r></m:e></m:acc></m:oMath></w:p>"#;
    let paragraph = w::Paragraph::from_bytes(xml.as_bytes()).unwrap();

    assert_eq!(
      wordprocessing_math_text(&paragraph.paragraph_choice[0]).as_deref(),
      Some("𝑎 \u{301}")
    );
  }
}
