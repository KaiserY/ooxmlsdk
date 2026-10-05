//! Styled operands of legacy EQ overstrikes (ECMA-376 Part 4 §14.10.4.6).

use super::{OverstrikeAlignment, OverstrikeInline, TextRun, TextStyle};

pub(super) struct OverstrikeField {
  pub portion: OverstrikeInline,
  pub trailing: Vec<TextRun>,
}

pub(super) fn overstrike(
  instruction: &str,
  instruction_styles: &[(usize, TextStyle)],
  fallback_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<OverstrikeField> {
  let mut rest = instruction.trim_start();
  rest = super::strip_ascii_case_prefix(rest, "eq")?.trim_start();
  rest = super::strip_ascii_case_prefix(rest, r"\o")?.trim_start();
  let alignment = if let Some(value) = super::strip_ascii_case_prefix(rest, r"\al") {
    rest = value.trim_start();
    OverstrikeAlignment::Left
  } else if let Some(value) = super::strip_ascii_case_prefix(rest, r"\ar") {
    rest = value.trim_start();
    OverstrikeAlignment::Right
  } else {
    if let Some(value) = super::strip_ascii_case_prefix(rest, r"\ac") {
      rest = value.trim_start();
    }
    OverstrikeAlignment::Center
  };
  let arguments = rest.strip_prefix('(')?.trim_end().strip_suffix(')')?;
  // Compound EQ expressions have their own operators and recursive layout.
  // Preserve their cached result until that grammar is supported. Likewise,
  // do not reinterpret escaped punctuation or locale-specific separators as
  // literal text. The supported operands are complete, plain styled strings.
  if arguments.contains(['(', ')', '\\', ';', '\r', '\n']) {
    return None;
  }
  let arguments_start = instruction.len() - rest.len() + 1;
  let mut start = arguments_start;
  let mut operands = Vec::new();
  for argument in arguments.split(',') {
    let end = start + argument.len();
    let runs = styled_runs(
      instruction,
      start..end,
      instruction_styles,
      fallback_style,
      hyperlink_url,
    )?;
    operands.push(runs);
    start = end + 1;
  }
  let trailing_start = arguments_start + arguments.len() + 1;
  let trailing = styled_runs(
    instruction,
    trailing_start..instruction.len(),
    instruction_styles,
    fallback_style,
    hyperlink_url,
  )?;
  operands
    .iter()
    .any(|operand| !operand.is_empty())
    .then_some(OverstrikeField {
      portion: OverstrikeInline {
        operands,
        alignment,
      },
      trailing,
    })
}

fn styled_runs(
  instruction: &str,
  range: std::ops::Range<usize>,
  instruction_styles: &[(usize, TextStyle)],
  fallback_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<Vec<TextRun>> {
  let start = range.start;
  let end = range.end;
  let mut runs: Vec<TextRun> = Vec::new();
  let mut offset = start;
  while offset < end {
    let next = instruction_styles.partition_point(|(position, _)| *position <= offset);
    let style = next
      .checked_sub(1)
      .map_or(fallback_style, |index| &instruction_styles[index].1);
    let run_end = instruction_styles
      .get(next)
      .map_or(end, |(position, _)| end.min(*position));
    let text = instruction.get(offset..run_end)?;
    if let Some(previous) = runs.last_mut()
      && previous.style == *style
    {
      previous.text.push_str(text);
    } else {
      runs.push(TextRun {
        text: text.to_string(),
        style: style.clone(),
        hyperlink_url: hyperlink_url.map(ToString::to_string),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: true,
      });
    }
    offset = run_end;
  }
  Some(runs)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn overstrike_preserves_operand_formatting_and_ignores_instruction_run_boundaries() {
    let instruction = r" EQ \o\ac(○,印,ABC)";
    let base = TextStyle::default();
    let small = TextStyle {
      font_size_pt: 8.0,
      baseline_shift_pt: 1.5,
      ..base.clone()
    };
    let stamp_start = instruction.find('印').unwrap();
    let stamp_end = stamp_start + '印'.len_utf8();
    for split in [false, true] {
      let styles: Vec<_> = instruction
        .char_indices()
        .filter(|&(offset, _)| split || [0, stamp_start, stamp_end].contains(&offset))
        .map(|(offset, _)| {
          (
            offset,
            if (stamp_start..stamp_end).contains(&offset) {
              small.clone()
            } else {
              base.clone()
            },
          )
        })
        .collect();
      let field = overstrike(instruction, &styles, &base, Some("#target"))
        .unwrap()
        .portion;
      assert_eq!(field.alignment, OverstrikeAlignment::Center);
      assert_eq!(field.operands.len(), 3);
      assert_eq!(field.operands[0][0].text, "○");
      assert_eq!(field.operands[1][0].text, "印");
      assert_eq!(field.operands[1][0].style, small);
      assert_eq!(field.operands[2].len(), 1);
      assert_eq!(field.operands[2][0].text, "ABC");
      assert_eq!(
        field.operands[2][0].hyperlink_url.as_deref(),
        Some("#target")
      );
    }
  }

  #[test]
  fn overstrike_preserves_spaces_after_the_expression() {
    // Native 12/16pt controls distinguish spaces inside the instruction from
    // a separate following text run; each contributes its own full advance.
    for suffix in ["", " ", "  "] {
      let instruction = format!(r" EQ \o(A,B){suffix}");
      let field = overstrike(&instruction, &[], &TextStyle::default(), None).unwrap();
      assert_eq!(
        field
          .trailing
          .iter()
          .map(|run| run.text.as_str())
          .collect::<String>(),
        suffix
      );
    }
  }

  #[test]
  fn overstrike_alignment_and_unsupported_grammar_remain_distinct() {
    let style = TextStyle::default();
    for (instruction, alignment) in [
      (r" EQ \o(A,B)", OverstrikeAlignment::Center),
      (r"EQ \o\al(A,B)", OverstrikeAlignment::Left),
      (r"EQ \o\ar(A,B)", OverstrikeAlignment::Right),
    ] {
      assert_eq!(
        overstrike(instruction, &[], &style, None)
          .unwrap()
          .portion
          .alignment,
        alignment
      );
    }
    for instruction in [
      r"EQ \o\ad(A,B)",
      r"EQ \o(\s\up3(A),B)",
      r"EQ \o(A;B)",
      r"EQ \o(A,B",
      r"EQ \o()",
      r"EQ \o(A,B) extra",
    ] {
      assert!(
        overstrike(instruction, &[], &style, None).is_none(),
        "{instruction}"
      );
    }
  }
}
