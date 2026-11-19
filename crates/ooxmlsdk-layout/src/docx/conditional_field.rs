//! Structured IF arguments retain nested field values and their formatting.
//! Page-dependent persisted results remain available for
//! pre-pagination measurement; repeating stories select rich branches only
//! after their physical page and the final page count are known.
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::Range;
use std::rc::Rc;

use super::model::{Block, DynamicFieldKind, InlineItem, ParagraphFieldEvent, TextStyle};

#[derive(Clone, Debug)]
pub(crate) struct InstructionObject {
  pub range: Range<usize>,
  pub value: InstructionValue,
}

#[derive(Clone, Debug)]
pub(crate) enum InstructionValue {
  PageValue(DynamicFieldKind),
  Content(Vec<InlineItem>),
  FieldResult {
    inlines: Vec<InlineItem>,
    requires_print_update: bool,
  },
}

#[derive(Clone, Debug)]
enum Part {
  Text { value: String, offset: usize },
  Object(InstructionValue),
}

#[derive(Clone, Debug)]
enum Operand {
  Literal(String),
  PageValue(DynamicFieldKind),
}

/// Immutable source branch resource shared within one layout. Repeating
/// paragraphs clone this handle,
/// while selected paint inlines remain independently owned. Event equality
/// compares source identity, without comparing or copying the drawing tree.
#[derive(Clone, Debug)]
pub(crate) struct Field(Rc<FieldData>);

impl PartialEq for Field {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

#[derive(Debug)]
struct FieldData {
  left: Operand,
  right: Operand,
  comparison: Comparison,
  when_true: Vec<InlineItem>,
  when_false: Vec<InlineItem>,
}

#[derive(Clone, Copy, Debug)]
enum Comparison {
  Equal,
  NotEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
}

fn tokens(instruction: &str, objects: &[InstructionObject]) -> Option<Vec<Vec<Part>>> {
  let mut out = Vec::new();
  let mut current = Vec::new();
  let mut text = String::new();
  let mut quoted = false;
  let mut text_offset = 0;
  let mut characters = instruction.char_indices().peekable();
  let mut objects = objects.iter().peekable();
  while let Some((offset, ch)) = characters.next() {
    if let Some(object) = objects.peek()
      && object.range.start == offset
    {
      if !text.is_empty() {
        current.push(Part::Text {
          value: std::mem::take(&mut text),
          offset: text_offset,
        });
      }
      current.push(Part::Object(object.value.clone()));
      while characters
        .peek()
        .is_some_and(|(index, _)| *index < object.range.end)
      {
        characters.next();
      }
      objects.next();
      continue;
    }
    // Word recognizes typographic quote delimiters in the instruction,
    // including guillemets. Nested results bypass this lexical step.
    if matches!(ch, '"' | '«' | '»' | '“' | '”') {
      if !text.is_empty() {
        current.push(Part::Text {
          value: std::mem::take(&mut text),
          offset: text_offset,
        });
      }
      if quoted || !current.is_empty() {
        out.push(std::mem::take(&mut current));
      }
      quoted = !quoted;
    } else if ch.is_whitespace() && !quoted {
      if !text.is_empty() {
        current.push(Part::Text {
          value: std::mem::take(&mut text),
          offset: text_offset,
        });
      }
      if !current.is_empty() {
        out.push(std::mem::take(&mut current));
      }
    } else {
      if text.is_empty() {
        text_offset = offset;
      }
      text.push(ch);
    }
  }
  if quoted || objects.next().is_some() {
    return None;
  }
  if !text.is_empty() {
    current.push(Part::Text {
      value: text,
      offset: text_offset,
    });
  }
  if !current.is_empty() {
    out.push(current);
  }
  Some(out)
}

fn literal(token: &[Part]) -> Option<String> {
  token
    .iter()
    .map(|part| match part {
      Part::Text { value, .. } => Some(value.as_str()),
      Part::Object(_) => None,
    })
    .collect::<Option<String>>()
}

fn operand(token: &[Part]) -> Option<Operand> {
  if let [Part::Object(InstructionValue::PageValue(value))] = token {
    return Some(Operand::PageValue(value.clone()));
  }
  let mut value = String::new();
  for part in token {
    match part {
      Part::Text { value: text, .. } => value.push_str(text),
      Part::Object(InstructionValue::FieldResult { inlines, .. }) => {
        value.push_str(&super::field_result_text(inlines).unwrap_or_default());
      }
      Part::Object(_) => return None,
    }
  }
  // Wildcard matching and compound expressions retain their cached result.
  if value.contains(['*', '?']) {
    return None;
  }
  Some(Operand::Literal(value))
}

fn content(
  token: &[Part],
  style: &TextStyle,
  instruction_styles: &[(usize, TextStyle)],
  hyperlink: Option<&str>,
) -> Option<Vec<InlineItem>> {
  let mut inlines = Vec::new();
  for part in token {
    match part {
      Part::Text { value, offset } => {
        // A branch may cross several code runs; preserve each run's format,
        // independently of both the outer cache and nested field formatting.
        let end = offset + value.len();
        let mut start = *offset;
        for boundary in instruction_styles
          .iter()
          .map(|(start, _)| *start)
          .filter(|boundary| *boundary > *offset && *boundary < end)
          .chain(std::iter::once(end))
        {
          let style = instruction_styles
            .iter()
            .rev()
            .find(|(boundary, _)| *boundary <= start)
            .map(|(_, style)| style)
            .unwrap_or(style);
          super::push_resolved_field_text(
            &mut inlines,
            value[start - offset..boundary - offset].to_owned(),
            style.clone(),
            hyperlink,
          );
          start = boundary;
        }
      }
      Part::Object(InstructionValue::Content(items))
      | Part::Object(InstructionValue::FieldResult { inlines: items, .. }) => {
        inlines.extend(items.iter().cloned());
      }
      Part::Object(InstructionValue::PageValue(_)) => return None,
    }
  }
  Some(inlines)
}

pub(super) enum StaticRecovery {
  UnknownOperator,
  Content(Vec<InlineItem>),
}

/// Word's print-time recovery for a condition without a recognized binary
/// operator. Native quoted/unquoted and signed/numeric opposites establish
/// the lexical distinction: a digit-led unquoted operand requires an operator;
/// a text operand selects its following arguments by empty/nonempty content.
pub(super) fn recover_static_condition(
  instruction: &str,
  objects: &[InstructionObject],
  style: &TextStyle,
  instruction_styles: &[(usize, TextStyle)],
  hyperlink: Option<&str>,
) -> Option<StaticRecovery> {
  let tokens = tokens(instruction, objects)?;
  if !literal(tokens.first()?)?.eq_ignore_ascii_case("IF") {
    return None;
  }
  let condition = literal(tokens.get(1)?)?;
  if tokens
    .get(2)
    .and_then(|token| literal(token))
    .is_some_and(|operator| {
      matches!(
        operator.as_str(),
        "=" | "==" | "<>" | "!=" | "<" | "<=" | ">" | ">="
      )
    })
  {
    return None;
  }
  if instruction
    .trim_start()
    .get(2..)?
    .trim_start()
    .starts_with(|ch: char| ch.is_ascii_digit())
  {
    return Some(StaticRecovery::UnknownOperator);
  }
  // A nested field value is an atomic operand in the ordinary rich parser;
  // never flatten it into an authored string or reparse its returned quotes.
  let branch = if condition.is_empty() { 3 } else { 2 };
  let inlines = if let Some(token) = tokens.get(branch) {
    content(token, style, instruction_styles, hyperlink)?
  } else {
    Vec::new()
  };
  Some(StaticRecovery::Content(inlines))
}

pub(super) fn parse(
  instruction: &str,
  objects: &[InstructionObject],
  style: &TextStyle,
  instruction_styles: &[(usize, TextStyle)],
  hyperlink: Option<&str>,
) -> Option<Field> {
  if !objects.iter().any(|object| {
    matches!(
      object.value,
      InstructionValue::Content(_) | InstructionValue::FieldResult { .. }
    )
  }) {
    return None;
  }
  let tokens = tokens(instruction, objects)?;
  if tokens.len() < 6 || !literal(&tokens[0])?.eq_ignore_ascii_case("IF") {
    return None;
  }
  // IF consumes its five arguments. Native Word ignores additional text;
  // do not feed a nested result back into this argument scanner.
  let left = operand(&tokens[1])?;
  let right = operand(&tokens[3])?;
  let comparison = match literal(&tokens[2])?.as_str() {
    "=" | "==" => Comparison::Equal,
    "<>" | "!=" => Comparison::NotEqual,
    "<" => Comparison::Less,
    "<=" => Comparison::LessEqual,
    ">" => Comparison::Greater,
    ">=" => Comparison::GreaterEqual,
    _ => return None,
  };
  Some(Field(Rc::new(FieldData {
    left,
    right,
    comparison,
    when_true: content(&tokens[4], style, instruction_styles, hyperlink)?,
    when_false: content(&tokens[5], style, instruction_styles, hyperlink)?,
  })))
}

impl Field {
  pub(super) fn static_result(&self) -> Option<&[InlineItem]> {
    self.selected(&|_| None)
  }

  fn selected(
    &self,
    resolve: &impl Fn(&DynamicFieldKind) -> Option<String>,
  ) -> Option<&[InlineItem]> {
    let value = |operand: &Operand| match operand {
      Operand::Literal(value) => Some(value.clone()),
      Operand::PageValue(kind) => resolve(kind),
    };
    let left = value(&self.0.left)?;
    let right = value(&self.0.right)?;
    let order = match (left.parse::<f64>(), right.parse::<f64>()) {
      (Ok(left), Ok(right)) if left.is_finite() && right.is_finite() => left.partial_cmp(&right)?,
      _ => left.to_lowercase().cmp(&right.to_lowercase()),
    };
    let condition = match self.0.comparison {
      Comparison::Equal => order == Ordering::Equal,
      Comparison::NotEqual => order != Ordering::Equal,
      Comparison::Less => order == Ordering::Less,
      Comparison::LessEqual => order != Ordering::Greater,
      Comparison::Greater => order == Ordering::Greater,
      Comparison::GreaterEqual => order != Ordering::Less,
    };
    Some(if condition {
      &self.0.when_true
    } else {
      &self.0.when_false
    })
  }
}

pub(super) fn contains(blocks: &[Block]) -> bool {
  blocks.iter().any(|block| match block {
    Block::Paragraph(p) => p
      .field_events
      .iter()
      .any(|event| matches!(event, ParagraphFieldEvent::ConditionalResultSpan { .. })),
    Block::Table(table) => table
      .rows
      .iter()
      .flat_map(|row| &row.cells)
      .any(|cell| contains(&cell.blocks)),
    Block::Frame(frame) => contains(&frame.blocks),
  })
}

pub(super) fn resolve<'a>(
  blocks: &'a [Block],
  resolve: &impl Fn(&DynamicFieldKind) -> Option<String>,
) -> Cow<'a, [Block]> {
  if !contains(blocks) {
    return Cow::Borrowed(blocks);
  }
  let mut blocks = blocks.to_vec();
  materialize(&mut blocks, resolve);
  Cow::Owned(blocks)
}

fn materialize(blocks: &mut [Block], resolve: &impl Fn(&DynamicFieldKind) -> Option<String>) {
  for block in blocks {
    match block {
      Block::Paragraph(p) => {
        let mut replacements = p
          .field_events
          .iter()
          .filter_map(|event| {
            let ParagraphFieldEvent::ConditionalResultSpan {
              inline_start,
              inline_end,
              field,
            } = event
            else {
              return None;
            };
            Some((
              *inline_start,
              *inline_end,
              field.selected(resolve)?.to_vec(),
            ))
          })
          .collect::<Vec<_>>();
        replacements.sort_by_key(|(start, end, _)| (*start, *end));
        for (start, end, content) in replacements.into_iter().rev() {
          if start > end || end > p.inlines.len() {
            continue;
          }
          let replacement_len = content.len();
          p.inlines.splice(start..end, content);
          for event in &mut p.field_events {
            event.remap_inline_offsets(|offset| {
              if offset >= end {
                offset.saturating_sub(end - start) + replacement_len
              } else if offset > start {
                start
              } else {
                offset
              }
            });
          }
        }
        p.field_events
          .retain(|event| !matches!(event, ParagraphFieldEvent::ConditionalResultSpan { .. }));
      }
      Block::Table(table) => {
        for row in &mut table.rows {
          for cell in &mut row.cells {
            materialize(&mut cell.blocks, resolve);
          }
        }
      }
      Block::Frame(frame) => materialize(&mut frame.blocks, resolve),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::model::FieldNumberFormat;
  use super::*;

  #[test]
  fn native_static_condition_recovery_keeps_numeric_and_text_lexical_owners() {
    let style = TextStyle::default();
    for condition in ["0", "1", "2", "1=1", "1=2", "1= 1", "1 =1"] {
      let instruction = format!("IF {condition} \"YES\" \"NO\"");
      assert!(
        matches!(
          recover_static_condition(&instruction, &[], &style, &[], None),
          Some(StaticRecovery::UnknownOperator)
        ),
        "{instruction}"
      );
    }
    for (condition, expected) in [
      ("x", "YES"),
      ("\"x\"", "YES"),
      ("\"\"", "NO"),
      ("\"0\"", "YES"),
      ("\"1\"", "YES"),
      ("-1", "YES"),
      ("=", "YES"),
      ("foo", "YES"),
      ("TRUE", "YES"),
      ("FALSE", "YES"),
      ("x=x", "YES"),
      ("=\"VALUE\"", "VALUE"),
      ("\"1\"=1", "=1"),
    ] {
      let instruction = format!("IF {condition} \"YES\" \"NO\"");
      let Some(StaticRecovery::Content(inlines)) =
        recover_static_condition(&instruction, &[], &style, &[], None)
      else {
        panic!("{instruction}");
      };
      assert_eq!(
        super::super::field_result_text(&inlines).unwrap_or_default(),
        expected,
        "{instruction}"
      );
    }
    for operator in ["=", "==", "<>", "!=", "<", "<=", ">", ">="] {
      let instruction = format!("IF \"x\" {operator} \"y\" \"YES\" \"NO\"");
      assert!(recover_static_condition(&instruction, &[], &style, &[], None).is_none());
    }
  }

  #[test]
  fn native_text_condition_recovery_retains_selected_code_format_and_nested_values() {
    let instruction = "IF =\"VALUE\" \"unused\" \"\"";
    let style = TextStyle::default();
    let code_style = TextStyle {
      font_size_pt: 18.0,
      italic: true,
      ..style.clone()
    };
    let code_start = instruction.find("VALUE").unwrap();
    let Some(StaticRecovery::Content(inlines)) = recover_static_condition(
      instruction,
      &[],
      &style,
      &[(code_start, code_style)],
      Some("https://example.test"),
    ) else {
      panic!("text condition");
    };
    assert!(
      matches!(inlines.as_slice(), [InlineItem::Text(run)]if run.text=="VALUE" && run.style.font_size_pt==18.0 && run.style.italic && run.hyperlink_url.as_deref()==Some("https://example.test"))
    );
    let instruction = "IF \u{fffc} \"YES\" \"NO\"";
    let object = InstructionObject {
      range: 3..6,
      value: InstructionValue::FieldResult {
        inlines: Vec::new(),
        requires_print_update: true,
      },
    };
    assert!(recover_static_condition(instruction, &[object], &style, &[], None).is_none());
  }

  fn fixture(operator: &str, false_branch: bool) -> (String, Vec<InstructionObject>) {
    let branches = if false_branch {
      "\"\" \u{fffc}"
    } else {
      "\u{fffc} \"\""
    };
    let instruction = format!("IF \"3\" {operator} \"5\" {branches}");
    let page = instruction.find("\"3\"").unwrap();
    let total = instruction.find("\"5\"").unwrap();
    let content = instruction.find('\u{fffc}').unwrap();
    let objects = vec![
      InstructionObject {
        range: page..page + 3,
        value: InstructionValue::PageValue(DynamicFieldKind::Page {
          number_format: FieldNumberFormat::PageStyle,
        }),
      },
      InstructionObject {
        range: total..total + 3,
        value: InstructionValue::PageValue(DynamicFieldKind::NumPages {
          number_format: FieldNumberFormat::Decimal,
        }),
      },
      InstructionObject {
        range: content..content + 3,
        value: InstructionValue::Content(vec![InlineItem::BookmarkStart("rich-object".into())]),
      },
    ];
    (instruction, objects)
  }

  #[test]
  fn native_nested_results_are_atomic_and_keep_code_formatting() {
    // Word native controls: quotes/spaces in nested results stay literal;
    // guillemets in authored field code are quote delimiters instead.
    for value in ["«Phone_Numbers»", "two words", "a\"b", "", "«CACHED NAME»"] {
      for quoted in [false, true] {
        let operand = if quoted { "\"\u{fffc}\"" } else { "\u{fffc}" };
        let instruction =
          format!("IF {operand} = \"\" \"matched\" \"phone:\t\u{fffc}/\u{fffc}:suffix\"");
        let inner_style = TextStyle {
          font_size_pt: 9.0,
          ..TextStyle::default()
        };
        let mut inlines = Vec::new();
        super::super::push_resolved_field_text(&mut inlines, value.to_owned(), inner_style, None);
        let objects = instruction
          .match_indices('\u{fffc}')
          .map(|(offset, _)| InstructionObject {
            range: offset..offset + 3,
            value: InstructionValue::FieldResult {
              inlines: inlines.clone(),
              requires_print_update: true,
            },
          })
          .collect::<Vec<_>>();
        let style = TextStyle {
          font_size_pt: 11.0,
          ..TextStyle::default()
        };
        let field = parse(&instruction, &objects, &style, &[], None).unwrap();
        let result = field.static_result().unwrap();
        let expected = if value.is_empty() {
          "matched".into()
        } else {
          format!("phone:\t{value}/{value}:suffix")
        };
        assert_eq!(super::super::field_result_text(result).unwrap(), expected);
        for item in result {
          if let InlineItem::Text(run) = item {
            assert_eq!(
              run.style.font_size_pt,
              if run.text == value { 9.0 } else { 11.0 }
            );
          }
        }
      }
    }
  }

  #[test]
  fn native_instruction_quotes_do_not_reparse_nested_merge_placeholders() {
    for (operator, expected) in [("=", ""), ("<>", "Phone_Numbers")] {
      let instruction =
        format!("IF \u{fffc} {operator} \"«Phone_Numbers»\" \"MATCH\" \"DIFFERENT\"");
      let mut inlines = Vec::new();
      super::super::push_resolved_field_text(
        &mut inlines,
        "«Phone_Numbers»".into(),
        TextStyle::default(),
        None,
      );
      let objects = [InstructionObject {
        range: 3..6,
        value: InstructionValue::FieldResult {
          inlines,
          requires_print_update: true,
        },
      }];
      let field = parse(&instruction, &objects, &TextStyle::default(), &[], None).unwrap();
      assert_eq!(
        super::super::field_result_text(field.static_result().unwrap()).unwrap_or_default(),
        expected
      );
    }
  }

  #[test]
  fn page_values_select_rich_branches_instead_of_stale_nested_caches() {
    for (operator, false_branch, expected) in [
      ("=", false, [false, false, false, false, true]),
      ("<>", false, [true, true, true, true, false]),
      ("=", true, [true, true, true, true, false]),
      ("<", false, [true, true, true, true, false]),
      (">=", false, [false, false, false, false, true]),
    ] {
      let (instruction, objects) = fixture(operator, false_branch);
      let field = parse(&instruction, &objects, &TextStyle::default(), &[], None).unwrap();
      for (index, present) in expected.into_iter().enumerate() {
        let branch = field
          .selected(&|kind| {
            Some(match kind {
              DynamicFieldKind::Page { .. } => (index + 1).to_string(),
              DynamicFieldKind::NumPages { .. } => "5".to_owned(),
              _ => unreachable!(),
            })
          })
          .unwrap();
        assert_eq!(
          !branch.is_empty(),
          present,
          "operator={operator}, false={false_branch}, page={}",
          index + 1
        );
        if present {
          assert!(matches!(branch, [InlineItem::BookmarkStart(name)] if name=="rich-object"));
        }
      }
    }
  }

  #[test]
  fn quoted_rich_arguments_retain_their_text_and_object_order() {
    let instruction = "IF \"3\" = 3 \"before \u{fffc} after\" \"fallback\"";
    let object = instruction.find('\u{fffc}').unwrap();
    let objects = vec![
      InstructionObject {
        range: 3..6,
        value: InstructionValue::PageValue(DynamicFieldKind::Page {
          number_format: FieldNumberFormat::PageStyle,
        }),
      },
      InstructionObject {
        range: object..object + 3,
        value: InstructionValue::Content(vec![InlineItem::BookmarkStart("picture".into())]),
      },
    ];
    let field = parse(instruction, &objects, &TextStyle::default(), &[], None).unwrap();
    let branch = field.selected(&|_| Some("3".into())).unwrap();
    assert!(
      matches!(branch, [InlineItem::Text(before), InlineItem::BookmarkStart(name), InlineItem::Text(after)] if before.text=="before " && name=="picture" && after.text==" after")
    );
    let branch = field.selected(&|_| Some("2".into())).unwrap();
    assert!(matches!(branch, [InlineItem::Text(run)] if run.text=="fallback"));
    let mut malformed = instruction.to_owned();
    malformed.pop();
    assert!(parse(&malformed, &objects, &TextStyle::default(), &[], None).is_none());
  }
}
