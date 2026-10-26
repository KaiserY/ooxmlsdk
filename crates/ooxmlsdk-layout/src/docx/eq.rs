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
  // Keep unsupported separators and escapes on the cached-result path.
  // Parenthesized \s operands are parsed below, independently of authored
  // instruction-run boundaries.
  if arguments.contains([';', '\r', '\n']) {
    return None;
  }
  let arguments_start = instruction.len() - rest.len() + 1;
  let mut depth = 0usize;
  let mut start = 0;
  let mut ranges = Vec::new();
  for (index, character) in arguments.char_indices() {
    match character {
      '(' => depth += 1,
      ')' => depth = depth.checked_sub(1)?,
      ',' if depth == 0 => {
        ranges.push(start..index);
        start = index + 1;
      }
      _ => {}
    }
  }
  if depth != 0 {
    return None;
  }
  ranges.push(start..arguments.len());
  let mut operands = Vec::new();
  for range in ranges {
    let argument = &arguments[range.clone()];
    let mut operand_start = arguments_start + range.start;
    let mut shift = 0.0;
    let literal = if argument.trim_start().starts_with('\\') {
      let rest = super::strip_ascii_case_prefix(argument.trim_start(), r"\s")?.trim_start();
      let (rest, direction) = if let Some(rest) = super::strip_ascii_case_prefix(rest, r"\up") {
        (rest, 1.0)
      } else {
        (super::strip_ascii_case_prefix(rest, r"\do")?, -1.0)
      };
      let opening = rest.find('(')?;
      let points = rest[..opening].trim();
      // ECMA-376 Part4 §14.10.4.6: \s\up/\do displaces its single
      // argument by an integral point distance (default 2). Native controls
      // retain the authored size, including explicit operand font sizes;
      // this is distinct from w:vertAlign automatic font reduction.
      shift = direction
        * if points.is_empty() {
          2.0
        } else {
          points.parse::<i32>().ok()? as f32
        };
      let literal = rest[opening + 1..].trim_end().strip_suffix(')')?;
      operand_start += argument.len() - rest.len() + opening + 1;
      literal
    } else {
      argument
    };
    if literal.contains(['(', ')', '\\', ',']) {
      return None;
    }
    let mut runs = styled_runs(
      instruction,
      operand_start..operand_start + literal.len(),
      instruction_styles,
      fallback_style,
      hyperlink_url,
    )?;
    for run in &mut runs {
      run.style.baseline_shift_pt += shift;
    }
    operands.push(runs);
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
  fn overstrike_script_operands_keep_native_sizes_and_point_offsets() {
    for size in [6.0, 10.5, 14.0, 18.0] {
      for points in [0, 2, 7, 14] {
        let instruction = format!(" EQ \\o(ABCD,\\s\\up{points}(催化剂),\\s\\do{points}(△)) ");
        let base = TextStyle {
          font_size_pt: size,
          ..Default::default()
        };
        let upper_start = instruction.find('催').unwrap();
        let upper_end = upper_start + "催化剂".len();
        let upper = TextStyle {
          font_size_pt: 18.0,
          baseline_shift_pt: 1.5,
          ..base.clone()
        };
        let styles = instruction
          .char_indices()
          .map(|(index, _)| {
            (
              index,
              if (upper_start..upper_end).contains(&index) {
                upper.clone()
              } else {
                base.clone()
              },
            )
          })
          .collect::<Vec<_>>();
        let field = overstrike(&instruction, &styles, &base, Some("#target")).unwrap();
        assert_eq!(field.portion.operands.len(), 3);
        for (index, text, font_size, shift) in [
          (0, "ABCD", size, 0.0),
          (1, "催化剂", 18.0, points as f32 + 1.5),
          (2, "△", size, -(points as f32)),
        ] {
          let runs = &field.portion.operands[index];
          assert_eq!(runs.len(), 1);
          assert_eq!(runs[0].text, text);
          assert_eq!(runs[0].style.font_size_pt, font_size);
          assert_eq!(runs[0].style.baseline_shift_pt, shift);
          assert_eq!(runs[0].hyperlink_url.as_deref(), Some("#target"));
        }
        assert_eq!(field.trailing[0].text, " ");
      }
    }
    for instruction in [r"EQ \o(AB,\s\up(C))", r"EQ \o(AB, \S \UP 2(C))"] {
      let field = overstrike(instruction, &[], &TextStyle::default(), None).unwrap();
      assert_eq!(field.portion.operands[1][0].text, "C");
      assert_eq!(field.portion.operands[1][0].style.baseline_shift_pt, 2.0);
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
      r"EQ \o(\s\ai3(A),B)",
      r"EQ \o(\s\up3(A,B),C)",
      r"EQ \o(\s\up3(A),\f(B,C))",
      r"EQ \o(\s\up1.5(A),B)",
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
