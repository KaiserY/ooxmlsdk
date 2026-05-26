use std::str::FromStr;

use ooxmlsdk::schemas::{m, schemas_microsoft_com_office_drawing_2010_main as a14};

pub(crate) fn text_math_text(math: &a14::TextMath) -> String {
  text_math_fragments_text(
    math
      .xml_other_children
      .iter()
      .map(|fragment| fragment.as_ref()),
  )
}

fn text_math_fragments_text<'a>(math: impl IntoIterator<Item = &'a str>) -> String {
  let mut text = String::new();
  for fragment in math {
    append_math_fragment_text(fragment, &mut text);
  }
  text
}

fn append_math_fragment_text(fragment: &str, text: &mut String) {
  match root_local_name(fragment).as_deref() {
    Some("oMathPara") => {
      if let Ok(paragraph) = m::Paragraph::from_str(fragment) {
        append_paragraph_text(&paragraph, text);
      }
    }
    Some("oMath") => {
      if let Ok(math) = m::OfficeMath::from_str(fragment) {
        append_office_math_text(&math, text);
      }
    }
    Some("r") => {
      if let Ok(run) = m::Run::from_str(fragment) {
        append_run_text(&run, text);
      }
    }
    _ => {}
  }
}

fn root_local_name(xml: &str) -> Option<String> {
  let start = xml.trim_start();
  let start = start.strip_prefix('<')?;
  let name = start
    .split(|character: char| {
      character.is_ascii_whitespace() || character == '>' || character == '/'
    })
    .next()?;
  Some(name.rsplit(':').next().unwrap_or(name).to_string())
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

fn append_office_math_text(math: &m::OfficeMath, text: &mut String) {
  for choice in &math.office_math_choice {
    append_office_math_choice_text(choice, text);
  }
}

fn append_office_math_choice_text(choice: &m::OfficeMathChoice, text: &mut String) {
  match choice {
    m::OfficeMathChoice::Run(run) => append_run_text(run, text),
    m::OfficeMathChoice::Superscript(value) => {
      append_base_text(&value.base, text);
      text.push('^');
      append_super_argument_text(&value.super_argument, text);
    }
    m::OfficeMathChoice::Subscript(value) => {
      append_base_text(&value.base, text);
      append_sub_argument_text(&value.sub_argument, text);
    }
    m::OfficeMathChoice::SubSuperscript(value) => {
      append_base_text(&value.base, text);
      append_sub_argument_text(&value.sub_argument, text);
      text.push('^');
      append_super_argument_text(&value.super_argument, text);
    }
    m::OfficeMathChoice::PreSubSuper(value) => {
      append_sub_argument_text(&value.sub_argument, text);
      text.push('^');
      append_super_argument_text(&value.super_argument, text);
      append_base_text(&value.base, text);
    }
    m::OfficeMathChoice::Paragraph(paragraph) => append_paragraph_text(paragraph, text),
    m::OfficeMathChoice::OfficeMath(math) => append_office_math_text(math, text),
    _ => {}
  }
}

fn append_base_text(base: &m::Base, text: &mut String) {
  for choice in &base.base_choice {
    match choice {
      m::BaseChoice::Run(run) => append_run_text(run, text),
      m::BaseChoice::Superscript(value) => {
        append_base_text(&value.base, text);
        text.push('^');
        append_super_argument_text(&value.super_argument, text);
      }
      m::BaseChoice::Subscript(value) => {
        append_base_text(&value.base, text);
        append_sub_argument_text(&value.sub_argument, text);
      }
      m::BaseChoice::SubSuperscript(value) => {
        append_base_text(&value.base, text);
        append_sub_argument_text(&value.sub_argument, text);
        text.push('^');
        append_super_argument_text(&value.super_argument, text);
      }
      m::BaseChoice::OfficeMath(math) => append_office_math_text(math, text),
      m::BaseChoice::Paragraph(paragraph) => append_paragraph_text(paragraph, text),
      _ => {}
    }
  }
}

fn append_sub_argument_text(argument: &m::SubArgument, text: &mut String) {
  for choice in &argument.sub_argument_choice {
    match choice {
      m::SubArgumentChoice::Run(run) => append_run_text(run, text),
      m::SubArgumentChoice::Superscript(value) => {
        append_base_text(&value.base, text);
        text.push('^');
        append_super_argument_text(&value.super_argument, text);
      }
      m::SubArgumentChoice::OfficeMath(math) => append_office_math_text(math, text),
      m::SubArgumentChoice::Paragraph(paragraph) => append_paragraph_text(paragraph, text),
      _ => {}
    }
  }
}

fn append_super_argument_text(argument: &m::SuperArgument, text: &mut String) {
  for choice in &argument.super_argument_choice {
    match choice {
      m::SuperArgumentChoice::Run(run) => append_run_text(run, text),
      m::SuperArgumentChoice::Superscript(value) => {
        append_base_text(&value.base, text);
        text.push('^');
        append_super_argument_text(&value.super_argument, text);
      }
      m::SuperArgumentChoice::OfficeMath(math) => append_office_math_text(math, text),
      m::SuperArgumentChoice::Paragraph(paragraph) => append_paragraph_text(paragraph, text),
      _ => {}
    }
  }
}

fn append_run_text(run: &m::Run, text: &mut String) {
  for choice in &run.run_choice {
    if let m::RunChoice::MText(value) = choice
      && let Some(content) = value.xml_content.as_deref()
    {
      text.push_str(content);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::text_math_fragments_text;

  #[test]
  fn text_math_text_reads_generated_math_fragment() {
    // Source: test-data/ooxmlsdk-pdf-test/libreoffice/pptx/tdf131553.pptx
    // ppt/diagrams/data1.xml. a14:m is generated as TextMath = Vec<String>;
    // the selected MCE dgm:pt already contains the m:oMathPara child.
    let math = [
      r#"<m:oMathPara xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:oMathParaPr><m:jc m:val="centerGroup"/></m:oMathParaPr><m:oMath><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝐴</m:t></m:r><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>=</m:t></m:r><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="el-GR" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝜋</m:t></m:r><m:sSup><m:sSupPr><m:ctrlPr><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr></m:ctrlPr></m:sSupPr><m:e><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>𝑟</m:t></m:r></m:e><m:sup><m:r><a:rPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" lang="en-US" altLang="zh-CN" i="1" smtClean="0"><a:latin typeface="Cambria Math"/></a:rPr><m:t>2</m:t></m:r></m:sup></m:sSup></m:oMath></m:oMathPara>"#,
    ];

    assert_eq!(text_math_fragments_text(math), "𝐴=𝜋𝑟^2");
  }
}
