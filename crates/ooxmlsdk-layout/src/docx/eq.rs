//! Styled operands of legacy EQ fields (ECMA-376 Part 4 §14.10.4.6).

mod expression;

pub(super) use expression::parse as expression;
pub(crate) use expression::{BorderSides, ExpressionInline, NaryOperator, Node};

use super::{
  EqArrayInline, OverstrikeAlignment, OverstrikeInline, RubyAlignment, RubyInline, TextRun,
  TextStyle,
};

pub(super) struct ArrayField {
  pub portion: EqArrayInline,
  pub trailing: Vec<TextRun>,
}

/// The array switches accept attached or separated integer arguments. Keep
/// instruction offsets so separately formatted field-code runs still own the
/// cell text. Unsupported nested equation grammar retains the cached path.
pub(super) fn array(
  instruction: &str,
  instruction_styles: &[(usize, TextStyle)],
  fallback_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<ArrayField> {
  let rest = super::strip_ascii_case_prefix(instruction.trim_start(), "eq")?.trim_start();
  let mut rest = super::strip_ascii_case_prefix(rest, r"\a")?;
  let mut columns = 1;
  let mut alignment = RubyAlignment::Center;
  let mut horizontal_gap_pt = 0.0;
  let mut vertical_gap_pt = 0.0;
  loop {
    rest = rest.trim_start();
    if rest.starts_with('(') {
      break;
    }
    let switch = rest.get(..3)?;
    rest = &rest[3..];
    if switch.eq_ignore_ascii_case(r"\al") {
      alignment = RubyAlignment::Left;
    } else if switch.eq_ignore_ascii_case(r"\ac") {
      alignment = RubyAlignment::Center;
    } else if switch.eq_ignore_ascii_case(r"\ar") {
      alignment = RubyAlignment::Right;
    } else {
      let is_columns = switch.eq_ignore_ascii_case(r"\co");
      let is_horizontal = switch.eq_ignore_ascii_case(r"\hs");
      if !is_columns && !is_horizontal && !switch.eq_ignore_ascii_case(r"\vs") {
        return None;
      }
      rest = rest.trim_start();
      let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
      let value = rest.get(..end)?.parse::<u16>().ok()?;
      rest = &rest[end..];
      if is_columns {
        columns = usize::from(value);
        if columns == 0 {
          return None;
        }
      } else {
        // [MS-OE376] Part4 §2.16.5.22: spacing switches use integral points
        // between 0 and 127. They add space between cells, not around them.
        if value > 127 {
          return None;
        }
        if is_horizontal {
          horizontal_gap_pt = f32::from(value);
        } else {
          vertical_gap_pt = f32::from(value);
        }
      }
    }
  }
  let arguments = rest.strip_prefix('(')?.trim_end().strip_suffix(')')?;
  if arguments.contains(['(', ')', '\\', ';', '\r', '\n']) {
    return None;
  }
  let arguments_start = instruction.len() - rest.len() + 1;
  let mut start = arguments_start;
  let mut cells = Vec::new();
  for argument in arguments.split(',') {
    let end = start + argument.len();
    cells.push(styled_runs(
      instruction,
      start..end,
      instruction_styles,
      fallback_style,
      hyperlink_url,
    )?);
    start = end + 1;
  }
  if !cells.iter().flatten().any(|run| !run.text.is_empty()) {
    return None;
  }
  let trailing = styled_runs(
    instruction,
    start..instruction.len(),
    instruction_styles,
    fallback_style,
    hyperlink_url,
  )?;
  Some(ArrayField {
    portion: EqArrayInline {
      columns: columns.min(cells.len()),
      cells,
      alignment,
      horizontal_gap_pt,
      vertical_gap_pt,
      style: fallback_style.clone(),
    },
    trailing,
  })
}

pub(super) struct RubyField<'a> {
  pub portion: RubyInline,
  pub font: Option<&'a str>,
  pub trailing: Vec<TextRun>,
}

/// Word's legacy phonetic-guide encoding. The jc switch distinguishes a ruby
/// from an ordinary overstrike; hps and Font override the guide's run format.
/// Native controls cover all six alignments, independently varied run/metadata
/// sizes and fonts, and omission of each switch. Keep the original instruction
/// offsets so field-code run boundaries still supply the operand formatting.
pub(super) fn ruby<'a>(
  instruction: &'a str,
  instruction_styles: &[(usize, TextStyle)],
  fallback_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<RubyField<'a>> {
  let mut rest = super::strip_ascii_case_prefix(instruction.trim_start(), "eq")?.trim_start();
  let mut alignment = None;
  let mut size = None;
  let mut font = None;
  while let Some(value) = rest.strip_prefix(r"\*") {
    rest = value.trim_start();
    let token;
    if let Some(quoted) = rest.strip_prefix('"') {
      let end = quoted.find('"')?;
      token = &quoted[..end];
      rest = &quoted[end + 1..];
    } else {
      let end = rest.find(char::is_whitespace)?;
      token = &rest[..end];
      rest = &rest[end..];
    }
    rest = rest.trim_start();
    if let Some(value) = super::strip_ascii_case_prefix(token, "jc") {
      alignment = Some(match value {
        "0" => RubyAlignment::Center,
        "1" => RubyAlignment::DistributeLetter,
        "2" => RubyAlignment::DistributeSpace,
        "3" => RubyAlignment::Left,
        "4" => RubyAlignment::Right,
        "5" => RubyAlignment::RightVertical,
        _ => return None,
      });
    } else if let Some(value) = super::strip_ascii_case_prefix(token, "hps") {
      let half_points = value.parse::<u16>().ok()?;
      if half_points == 0 {
        return None;
      }
      size = Some(f32::from(half_points) / 2.0);
    } else {
      let value = super::strip_ascii_case_prefix(token, "Font:")?;
      font = Some(value.trim()).filter(|value| !value.is_empty());
    }
  }
  let alignment = alignment?;
  let parsed = parse_overstrike(
    instruction,
    rest,
    instruction_styles,
    fallback_style,
    hyperlink_url,
  )?;
  let [mut guide, base]: [Operand; 2] = parsed.operands.try_into().ok()?;
  let raise_pt = guide.shift?;
  if raise_pt < 0.0 || base.shift.is_some() || parsed.alignment != OverstrikeAlignment::Center {
    return None;
  }
  if let Some(size) = size {
    for run in &mut guide.runs {
      run.style.font_size_pt = size;
      run.style.complex_font_size_pt = Some(size);
    }
  }
  Some(RubyField {
    portion: RubyInline {
      fit_text: fallback_style.wordprocessing_fit_text,
      container_font_size_pt: fallback_style.font_size_pt,
      base: base.runs,
      guide: guide.runs,
      alignment,
      raise_pt,
    },
    font,
    trailing: parsed.trailing,
  })
}

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
  let rest = super::strip_ascii_case_prefix(instruction.trim_start(), "eq")?.trim_start();
  let parsed = parse_overstrike(
    instruction,
    rest,
    instruction_styles,
    fallback_style,
    hyperlink_url,
  )?;
  let operands = parsed
    .operands
    .into_iter()
    .map(|mut operand| {
      for run in &mut operand.runs {
        run.style.baseline_shift_pt += operand.shift.unwrap_or(0.0);
      }
      operand.runs
    })
    .collect();
  Some(OverstrikeField {
    portion: OverstrikeInline {
      operands,
      alignment: parsed.alignment,
    },
    trailing: parsed.trailing,
  })
}

struct Operand {
  runs: Vec<TextRun>,
  shift: Option<f32>,
}

struct ParsedOverstrike {
  operands: Vec<Operand>,
  alignment: OverstrikeAlignment,
  trailing: Vec<TextRun>,
}

fn parse_overstrike(
  instruction: &str,
  mut rest: &str,
  instruction_styles: &[(usize, TextStyle)],
  fallback_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<ParsedOverstrike> {
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
    let mut shift = None;
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
      shift = Some(
        direction
          * if points.is_empty() {
            2.0
          } else {
            points.parse::<i32>().ok()? as f32
          },
      );
      let literal = rest[opening + 1..].trim_end().strip_suffix(')')?;
      operand_start += argument.len() - rest.len() + opening + 1;
      literal
    } else {
      argument
    };
    if literal.contains(['(', ')', '\\', ',']) {
      return None;
    }
    let runs = styled_runs(
      instruction,
      operand_start..operand_start + literal.len(),
      instruction_styles,
      fallback_style,
      hyperlink_url,
    )?;
    operands.push(Operand { runs, shift });
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
    .any(|operand| !operand.runs.is_empty())
    .then_some(ParsedOverstrike {
      operands,
      alignment,
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
  fn array_switches_keep_row_major_cells_styles_and_field_tail() {
    let base = TextStyle::default();
    let large = TextStyle {
      font_size_pt: 20.0,
      ..base.clone()
    };
    let code = r" EQ \a\ar\co 2\hs3\vs 6(Milk,Eggs,Detergent,Veggies) ";
    let start = code.find("Detergent").unwrap();
    let field = array(
      code,
      &[(0, base.clone()), (start, large), (start + 9, base.clone())],
      &base,
      Some("#array"),
    )
    .unwrap();
    assert_eq!(field.portion.columns, 2);
    assert_eq!(field.portion.alignment, RubyAlignment::Right);
    assert_eq!(field.portion.horizontal_gap_pt, 3.0);
    assert_eq!(field.portion.vertical_gap_pt, 6.0);
    assert_eq!(
      field
        .portion
        .cells
        .iter()
        .map(|cell| cell[0].text.as_str())
        .collect::<Vec<_>>(),
      ["Milk", "Eggs", "Detergent", "Veggies"]
    );
    assert_eq!(field.portion.cells[2][0].style.font_size_pt, 20.0);
    assert_eq!(
      field.portion.cells[2][0].hyperlink_url.as_deref(),
      Some("#array")
    );
    assert_eq!(field.trailing[0].text, " ");
  }

  #[test]
  fn array_defaults_and_explicit_alignment_are_independent_of_column_count() {
    let style = TextStyle::default();
    for (switch, expected) in [
      ("", RubyAlignment::Center),
      (r"\al", RubyAlignment::Left),
      (r"\ac", RubyAlignment::Center),
      (r"\ar", RubyAlignment::Right),
    ] {
      for columns in [1, 2, 3] {
        let code = format!(r"EQ \a {switch} \co{columns}(A,B,C,D)");
        let field = array(&code, &[], &style, None).unwrap();
        assert_eq!(field.portion.columns, columns);
        assert_eq!(field.portion.alignment, expected);
        assert!(field.trailing.is_empty());
      }
    }
  }

  #[test]
  fn unsupported_array_grammar_keeps_the_cached_field_path() {
    for code in [
      r"EQ \a\co0(A,B)",
      r"EQ \a\vs128(A,B)",
      r"EQ \a\hs-1(A,B)",
      r"EQ \a(\f(A,B),C)",
      r"EQ \a(A;B)",
      r"EQ \a(A,B) tail",
      r"EQ \a()",
    ] {
      assert!(
        array(code, &[], &TextStyle::default(), None).is_none(),
        "{code}"
      );
    }
  }

  #[test]
  fn ruby_metadata_overrides_guide_size_without_shifting_its_runs_twice() {
    let base = TextStyle {
      font_size_pt: 20.0,
      ..Default::default()
    };
    for hps in [12, 16, 24] {
      for run_size in [6.0, 8.0, 14.0] {
        let instruction =
          format!(r#" EQ \* jc0 \* "Font:MS Mincho" \* hps{hps} \o(\s\up 19(にがつ),二月) "#);
        let start = instruction.find('に').unwrap();
        let end = start + "にがつ".len();
        let guide = TextStyle {
          font_size_pt: run_size,
          ..base.clone()
        };
        let styles = instruction
          .char_indices()
          .map(|(position, _)| {
            (
              position,
              if (start..end).contains(&position) {
                guide.clone()
              } else {
                base.clone()
              },
            )
          })
          .collect::<Vec<_>>();
        let field = ruby(&instruction, &styles, &base, Some("#target")).unwrap();
        assert_eq!(field.font, Some("MS Mincho"));
        assert_eq!(field.portion.raise_pt, 19.0);
        assert_eq!(field.portion.base[0].text, "二月");
        assert_eq!(field.portion.base[0].style, base);
        assert_eq!(field.portion.guide.len(), 1);
        assert_eq!(field.portion.guide[0].text, "にがつ");
        assert_eq!(field.portion.guide[0].style.font_size_pt, hps as f32 / 2.0);
        assert_eq!(
          field.portion.guide[0].style.complex_font_size_pt,
          Some(hps as f32 / 2.0)
        );
        assert_eq!(field.portion.guide[0].style.baseline_shift_pt, 0.0);
        assert_eq!(
          field.portion.guide[0].hyperlink_url.as_deref(),
          Some("#target")
        );
        assert_eq!(field.trailing[0].text, " ");
      }
    }
  }

  #[test]
  fn ruby_jc_selects_alignment_and_missing_metadata_keeps_run_format() {
    let style = TextStyle {
      font_size_pt: 8.0,
      ..Default::default()
    };
    for (jc, alignment) in [
      RubyAlignment::Center,
      RubyAlignment::DistributeLetter,
      RubyAlignment::DistributeSpace,
      RubyAlignment::Left,
      RubyAlignment::Right,
      RubyAlignment::RightVertical,
    ]
    .into_iter()
    .enumerate()
    {
      let instruction = format!(r"EQ \* jc{jc} \o(\s\up19(にがつ),二月)");
      let field = ruby(&instruction, &[], &style, None).unwrap();
      assert_eq!(field.portion.alignment, alignment);
      assert_eq!(field.portion.guide[0].style, style);
      assert!(field.font.is_none());
    }
    for instruction in [
      r"EQ \o(\s\up19(にがつ),二月)",
      r"EQ \* hps16 \o(\s\up19(にがつ),二月)",
      r"EQ \* jc6 \o(\s\up19(にがつ),二月)",
      r"EQ \* jc0 \* unsupported \o(\s\up19(にがつ),二月)",
      r"EQ \* jc0 \o(\s\up19(にがつ),二月,三月)",
      r"EQ \* jc0 \o(にがつ,二月)",
    ] {
      assert!(
        ruby(instruction, &[], &style, None).is_none(),
        "{instruction}"
      );
    }
  }

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
